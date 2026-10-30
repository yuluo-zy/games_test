//! Correctness-only, base-mip 2D image copies. No conversion, resolve,
//! shader, clear, new host, or CPU-performance claim is permitted here.
use super::{AccessRange, Descriptor, PassContext, ResourceHandle};
use crate::{Error, Result};
use crate::resource::texture::{TextureDesc, TextureKind};
use ash::vk::{self, Handle};
use std::sync::Arc;

fn invalid(message: &'static str) -> Error {
    Error::unsupported(message)
}

fn image_shape(desc: &TextureDesc) -> Result<()> {
    desc.validate()?;
    if desc.kind != TextureKind::D2
        || desc.mip_levels > 6
        || desc.array_elements != 1
        || desc.depth != 1
        || desc.sample_count != 1
    {
        return Err(invalid(
            "surface diagnostic supports only base mip of single-sample one-layer 2D images with at most six levels",
        ));
    }
    if !matches!(
        desc.format,
        vk::Format::B10G11R11_UFLOAT_PACK32 | vk::Format::D32_SFLOAT_S8_UINT
            | vk::Format::R16G16B16A16_SFLOAT | vk::Format::R8G8B8A8_UNORM
            | vk::Format::R32G32_UINT | vk::Format::R16G16_SFLOAT
    ) {
        return Err(invalid(
            "diagnostic requires original HDR, GI radiance/stats or D32S8 format; no conversion fallback",
        ));
    }
    Ok(())
}

fn aspects(desc: &TextureDesc) -> &'static [vk::ImageAspectFlags] {
    if desc.format == vk::Format::D32_SFLOAT_S8_UINT {
        &[vk::ImageAspectFlags::DEPTH, vk::ImageAspectFlags::STENCIL]
    } else {
        &[vk::ImageAspectFlags::COLOR]
    }
}

fn range(aspect: vk::ImageAspectFlags) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: aspect.as_raw(),
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}

fn declared(
    ctx: &PassContext<'_>,
    resource: ResourceHandle,
    access: u8,
    required: AccessRange,
) -> Result<Descriptor> {
    let pass = ctx
        .compiled
        .declarations
        .passes
        .get(ctx.pass_index)
        .ok_or_else(|| Error::invalid_usage("diagnostic callback has no actual pass"))?;
    if pass.graph_id != Some(resource.graph_id()) {
        return Err(Error::invalid_usage(
            "diagnostic copy resource belongs to another graph",
        ));
    }
    let desc = ctx
        .compiled
        .declarations
        .resources
        .get(resource.raw() as usize)
        .ok_or_else(|| Error::invalid_usage("diagnostic copy has no declared resource"))?
        .descriptor;
    required
        .validate(desc)
        .map_err(|_| Error::invalid_usage("diagnostic copy range is invalid"))?;
    let ranges: Vec<_> = pass
        .accesses
        .iter()
        .filter(|a| a.resource_index == resource.raw() && a.access_code == access)
        .map(|a| a.range)
        .collect();
    if !super::resource_contracts::covers(&ranges, required, desc)
        .map_err(|_| Error::invalid_usage("diagnostic copy declaration range is invalid"))?
    {
        return Err(Error::invalid_usage(
            "diagnostic copy lacks exact transfer access coverage",
        ));
    }
    Ok(desc)
}

fn capability(
    ctx: &PassContext<'_>,
    image: &crate::resource::texture::Texture,
    needed: vk::FormatFeatureFlags,
) -> Result<()> {
    let device = image.device();
    // Capabilities are checked on the real selected device, before native recording.
    let instance = &device.core.instance.instance;
    let queue =
        unsafe { instance.get_physical_device_queue_family_properties(device.core.physical) };
    if !queue
        .get(ctx.command.family as usize)
        .is_some_and(|q| q.queue_flags.contains(vk::QueueFlags::GRAPHICS))
    {
        return Err(invalid(
            "surface diagnostic copies require the actual graphics queue",
        ));
    }
    let props = unsafe {
        instance.get_physical_device_format_properties(device.core.physical, image.desc.format)
    };
    if !props.optimal_tiling_features.contains(needed) {
        return Err(invalid(
            "original image format lacks required transfer capability",
        ));
    }
    Ok(())
}

/// Same-format base-mip bit copy, including both real depth/stencil aspects.
/// Stencil is preserved but is not an initialized-content/pixel claim; the
/// surface fixture must prove stencil testing disabled and exclude its readback.
/// The caller must declare source=0x25 and destination=0x34 on this exact graph.
pub fn copy_image(
    ctx: &PassContext<'_>,
    source: ResourceHandle,
    destination: ResourceHandle,
) -> Result<()> {
    let managed = ctx
        .managed
        .ok_or_else(|| Error::invalid_usage("copy requires actual managed owners"))?;
    let src = managed.texture_for(source.raw())?;
    let dst = managed.texture_for(destination.raw())?;
    image_shape(&src.desc)?;
    image_shape(&dst.desc)?;
    if src.raw() == dst.raw()
        || !Arc::ptr_eq(src.device(), dst.device())
        || src.desc.format != dst.desc.format
        || src.desc.width != dst.desc.width
        || src.desc.height != dst.desc.height
        || src.desc.mip_levels != dst.desc.mip_levels
        || src.desc.flags != dst.desc.flags
    {
        return Err(Error::invalid_usage(
            "diagnostic copy requires distinct equivalent native images",
        ));
    }
    if !src.desc.usage.contains(vk::ImageUsageFlags::TRANSFER_SRC)
        || !dst.desc.usage.contains(vk::ImageUsageFlags::TRANSFER_DST)
    {
        return Err(Error::invalid_usage(
            "diagnostic image copy has no native transfer usage",
        ));
    }
    capability(ctx, &src, vk::FormatFeatureFlags::TRANSFER_SRC)?;
    capability(ctx, &dst, vk::FormatFeatureFlags::TRANSFER_DST)?;
    let mut copies = Vec::new();
    for &aspect in aspects(&src.desc) {
        declared(ctx, source, 0x25, range(aspect))?;
        declared(ctx, destination, 0x34, range(aspect))?;
        if ctx.image_layout_for_range(source.raw(), range(aspect))?
            != vk::ImageLayout::TRANSFER_SRC_OPTIMAL
            || ctx.image_layout_for_range(destination.raw(), range(aspect))?
                != vk::ImageLayout::TRANSFER_DST_OPTIMAL
        {
            return Err(Error::invalid_usage(
                "diagnostic copy layout differs from actual transfer plan",
            ));
        }
        let sub = vk::ImageSubresourceLayers::default()
            .aspect_mask(aspect)
            .layer_count(1);
        copies.push(
            vk::ImageCopy::default()
                .src_subresource(sub)
                .dst_subresource(sub)
                .extent(vk::Extent3D {
                    width: src.desc.width,
                    height: src.desc.height,
                    depth: 1,
                }),
        );
    }
    // Both references are covered by the checked access declarations. Register
    // the real owners with the executor's existing completion/retirement path.
    unsafe {
        ctx.retain_recorded(src.shared())?;
        ctx.retain_recorded(dst.shared())?;
        src.device().core.device.cmd_copy_image(
            vk::CommandBuffer::from_raw(ctx.command.handle()),
            src.raw(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            dst.raw(),
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &copies,
        );
    }
    Ok(())
}

/// Exact compact byte count. Padding of D32S8 is excluded by copying one
/// explicit aspect at a time; no undefined auxiliary attachment is accepted.
pub fn raw_byte_count(
    format: vk::Format,
    extent: [u32; 2],
    aspect: vk::ImageAspectFlags,
) -> Result<u64> {
    let bytes = match (format, aspect) {
        (vk::Format::B10G11R11_UFLOAT_PACK32, vk::ImageAspectFlags::COLOR) => 4,
        (vk::Format::R16G16B16A16_SFLOAT | vk::Format::R32G32_UINT, vk::ImageAspectFlags::COLOR) => 8,
        (vk::Format::R8G8B8A8_UNORM, vk::ImageAspectFlags::COLOR) => 4,
        (vk::Format::R16G16_SFLOAT, vk::ImageAspectFlags::COLOR) => 4,
        (vk::Format::D32_SFLOAT_S8_UINT, vk::ImageAspectFlags::DEPTH) => 4,
        _ => {
            return Err(invalid(
                "unsupported raw image/aspect; no conversion fallback",
            ));
        }
    };
    if extent.contains(&0) {
        return Err(Error::invalid_usage("zero raw image extent"));
    }
    u64::from(extent[0])
        .checked_mul(u64::from(extent[1]))
        .and_then(|n| n.checked_mul(bytes))
        .filter(|n| *n <= 64 * 1024 * 1024)
        .ok_or_else(|| Error::invalid_usage("raw image capture exceeds fixed 64MiB bound"))
}

pub fn copy_image_to_buffer(
    ctx: &PassContext<'_>,
    source: ResourceHandle,
    destination: ResourceHandle,
    aspect: vk::ImageAspectFlags,
) -> Result<()> {
    let managed = ctx
        .managed
        .ok_or_else(|| Error::invalid_usage("readback requires actual managed owners"))?;
    let src = managed.texture_for(source.raw())?;
    let dst = managed.buffer_for(destination.raw())?;
    image_shape(&src.desc)?;
    let bytes = raw_byte_count(src.desc.format, [src.desc.width, src.desc.height], aspect)?;
    if !Arc::ptr_eq(src.device(), dst.device())
        || dst.size != bytes
        || !src.desc.usage.contains(vk::ImageUsageFlags::TRANSFER_SRC)
        || !dst.usage.contains(vk::BufferUsageFlags::TRANSFER_DST)
    {
        return Err(Error::invalid_usage(
            "raw capture native owner/size/usage differs from declaration",
        ));
    }
    capability(ctx, &src, vk::FormatFeatureFlags::TRANSFER_SRC)?;
    declared(ctx, source, 0x25, range(aspect))?;
    declared(
        ctx,
        destination,
        0x34,
        AccessRange::Buffer {
            offset: 0,
            size: bytes,
        },
    )?;
    if ctx.image_layout_for_range(source.raw(), range(aspect))?
        != vk::ImageLayout::TRANSFER_SRC_OPTIMAL
    {
        return Err(Error::invalid_usage(
            "raw capture layout differs from transfer plan",
        ));
    }
    let copy = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(aspect)
                .layer_count(1),
        )
        .image_extent(vk::Extent3D {
            width: src.desc.width,
            height: src.desc.height,
            depth: 1,
        });
    unsafe {
        ctx.retain_recorded(src.shared())?;
        ctx.retain_recorded(dst.shared())?;
        src.device().core.device.cmd_copy_image_to_buffer(
            vk::CommandBuffer::from_raw(ctx.command.handle()),
            src.raw(),
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            dst.raw_handle(),
            &[copy],
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests;
