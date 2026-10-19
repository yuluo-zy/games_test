//! Texture uploads: validation, staging layout and command recording shared by
//! [`super::TransferManager::upload_textures`] and the `raw::setup` helpers.
use super::record::{WrittenPiece, image_release_program};
use crate::{
    Error, Result,
    raw::{
        barrier::SynchronizationScope,
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandBuffer,
        format::vk_format_info,
        resource_access::image_access_borrowed,
        texture::Texture,
        texture_upload::{TextureUpdateRegion, copy_format_params, plan_texture_copies},
    },
    resource::range_state::AccessScope,
    sync::QueueId,
};
use ash::vk;
use std::sync::Arc;

/// Shader stages that sample an uploaded texture; the post-copy layout
/// transition makes the data visible to exactly these reads.
pub(crate) const IMAGE_CONSUMERS: AccessScope = AccessScope {
    stages: vk::PipelineStageFlags::from_raw(
        vk::PipelineStageFlags::VERTEX_SHADER.as_raw()
            | vk::PipelineStageFlags::FRAGMENT_SHADER.as_raw()
            | vk::PipelineStageFlags::COMPUTE_SHADER.as_raw(),
    ),
    access: vk::AccessFlags::SHADER_READ,
    write: false,
};

/// One texture's region list for [`super::TransferManager::upload_textures`].
pub struct TextureUpload<'a> {
    pub texture: &'a Arc<Texture>,
    pub regions: &'a [TextureUpdateRegion<'a>],
    /// `UNDEFINED` (initial data, discards contents) or `SHADER_READ_ONLY_OPTIMAL`.
    pub old_layout: vk::ImageLayout,
}

/// Validated copy program of one texture; buffer offsets are relative to a
/// base offset that callers place at a multiple of `alignment`.
pub(crate) struct TexturePlan {
    pub copies: Vec<vk::BufferImageCopy>,
    pub total: u64,
    pub alignment: u64,
}

/// Validate the destination, every region's subresource, extent and byte count.
pub(crate) fn plan_texture_upload(
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
    old_layout: vk::ImageLayout,
) -> Result<TexturePlan> {
    if regions.is_empty()
        || !texture
            .desc
            .usage
            .contains(vk::ImageUsageFlags::TRANSFER_DST)
        || texture.desc.samples() != vk::SampleCountFlags::TYPE_1
    {
        return Err(Error::invalid_usage(
            "texture upload requires regions and a single-sampled transfer destination",
        ));
    }
    if !matches!(
        old_layout,
        vk::ImageLayout::UNDEFINED | vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    ) {
        return Err(Error::invalid_usage(
            "texture upload old layout is unsupported",
        ));
    }
    let (block_bytes, aspect) = copy_format_params(texture.desc.format)?;
    let info = vk_format_info(texture.desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    let layers = texture.desc.array_layers()?;
    for region in regions {
        if region.mip_level >= u32::from(texture.desc.mip_levels) || region.array_layer >= layers {
            return Err(Error::invalid_usage(
                "texture upload subresource is out of range",
            ));
        }
        let full = [texture.desc.width, texture.desc.height, texture.desc.depth]
            .map(|size| (size >> region.mip_level).max(1));
        if region.extent.contains(&0)
            || region.extent.iter().zip(full).zip(info.block_extent).any(
                |((&size, full), block)| size > full || size != full && !size.is_multiple_of(block),
            )
        {
            return Err(Error::invalid_usage(
                "texture upload extent violates texel-block boundaries",
            ));
        }
        let required = region
            .extent
            .iter()
            .zip(info.block_extent)
            .try_fold(u64::from(info.block_bytes), |bytes, (&size, block)| {
                bytes.checked_mul(u64::from(size.div_ceil(block)))
            })
            .ok_or_else(|| Error::invalid_usage("texture upload size overflows"))?;
        if region.data.len() as u64 != required {
            return Err(Error::invalid_usage(
                "texture upload byte count differs from its region",
            ));
        }
    }
    let plan = plan_texture_copies(regions, block_bytes, aspect)?;
    let block = u64::from(block_bytes);
    Ok(TexturePlan {
        copies: plan.copies,
        total: plan.total,
        alignment: block / gcd(block, 4) * 4,
    })
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub(crate) fn align_up(value: u64, alignment: u64) -> Result<u64> {
    value
        .checked_add(alignment - 1)
        .map(|end| end / alignment * alignment)
        .ok_or_else(|| Error::invalid_usage("texture staging offset overflows"))
}

/// Base offsets for consecutive plans and the total staging size.
pub(crate) fn layout<'a>(plans: impl Iterator<Item = &'a TexturePlan>) -> Result<(Vec<u64>, u64)> {
    let mut bases = Vec::new();
    let mut cursor = 0u64;
    for plan in plans {
        let base = align_up(cursor, plan.alignment)?;
        bases.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
        bases.push(base);
        cursor = base
            .checked_add(plan.total)
            .ok_or_else(|| Error::invalid_usage("texture staging size overflows"))?;
    }
    Ok((bases, cursor))
}

/// Write each region at `base + copy.buffer_offset` (padding stays zero).
/// `staging` must be at least `base + plan.total` long.
pub(crate) fn pack_regions(
    staging: &mut [u8],
    base: u64,
    plan: &TexturePlan,
    regions: &[TextureUpdateRegion<'_>],
) {
    for (copy, region) in plan.copies.iter().zip(regions) {
        let start = (base + copy.buffer_offset) as usize;
        staging[start..start + region.data.len()].copy_from_slice(region.data);
    }
}

/// Pack every plan into one host vector laid out by [`layout`].
pub(crate) fn pack_all(
    plans: &[(&TexturePlan, &[TextureUpdateRegion<'_>])],
    bases: &[u64],
    total: u64,
    bytes: &mut Vec<u8>,
) -> Result<()> {
    let total = usize::try_from(total).map_err(|_| Error::OutOfHostMemory)?;
    bytes.clear();
    bytes
        .try_reserve_exact(total)
        .map_err(|_| Error::OutOfHostMemory)?;
    bytes.resize(total, 0);
    for ((plan, regions), base) in plans.iter().zip(bases) {
        pack_regions(bytes, *base, plan, regions);
    }
    Ok(())
}

/// Stage packed bytes in a fresh host-visible buffer with one host write.
pub(crate) fn new_staging(
    device: &Arc<crate::device::Device>,
    bytes: &[u8],
) -> Result<Arc<Buffer>> {
    let staging = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(bytes.len() as u64, vk::BufferUsageFlags::TRANSFER_SRC),
    )?;
    staging.write_bytes_unsynchronized(0, bytes)?;
    Ok(Arc::new(staging))
}

/// Record the state-derived transition of the whole texture to
/// TRANSFER_DST_OPTIMAL followed by the copies of `plan`.
fn record_transfer_dst_copies(
    command: &mut VulkanCommandBuffer,
    texture: &Texture,
    queue: QueueId,
    staging: vk::Buffer,
    base: u64,
    plan: &TexturePlan,
    old_layout: vk::ImageLayout,
) -> Result<crate::resource::OwnershipRange> {
    let range = Texture::ownership_range(&texture.desc)?;
    image_access_borrowed(
        command,
        texture,
        queue,
        range,
        AccessScope {
            stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE,
            write: true,
        },
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        Some(old_layout),
    )?;
    let mut copies = Vec::new();
    copies
        .try_reserve_exact(plan.copies.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    copies.extend(plan.copies.iter().map(|copy| vk::BufferImageCopy {
        buffer_offset: base + copy.buffer_offset,
        ..*copy
    }));
    // SAFETY: recording; the texture is retained by the access above and the
    // staging span by the caller.
    unsafe {
        command.pool.device.device.cmd_copy_buffer_to_image(
            command.handle,
            staging,
            texture.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &copies,
        );
    }
    Ok(range)
}

/// Record one texture's upload: a state-derived transition to
/// TRANSFER_DST_OPTIMAL, the copies, and a transition to
/// SHADER_READ_ONLY_OPTIMAL visible to [`IMAGE_CONSUMERS`]. The caller retains
/// the staging span `[base, base + plan.total)` and submits on `queue`.
pub(crate) fn record_texture_copies(
    command: &mut VulkanCommandBuffer,
    texture: &Texture,
    queue: QueueId,
    staging: vk::Buffer,
    base: u64,
    plan: &TexturePlan,
    old_layout: vk::ImageLayout,
) -> Result<()> {
    let range =
        record_transfer_dst_copies(command, texture, queue, staging, base, plan, old_layout)?;
    image_access_borrowed(
        command,
        texture,
        queue,
        range,
        IMAGE_CONSUMERS,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
    )
}

/// How a texture uploaded on a dedicated transfer queue is published.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TransferPublish {
    /// Same family or concurrent sharing: the final layout transition happens
    /// on the transfer queue (transfer-stage destination, legal on any
    /// transfer-capable family); consumers order through the timeline wait.
    Local,
    /// Exclusive image of another family: release to this family; the
    /// matching acquire runs on the consuming queue.
    Release { destination_family: u32 },
}

/// Whether `texture` may be written on `family` right now without an
/// ownership acquire. Exclusive images already owned by another family stay
/// on that family's queue.
pub(super) fn transfer_accessible(texture: &Texture, family: u32) -> bool {
    texture.sharing().allows_family(family)
        && Texture::ownership_range(&texture.desc)
            .is_ok_and(|range| texture.last_use.validate_queue_range(family, range).is_ok())
}

/// Record one texture's upload on a dedicated transfer queue. With
/// [`TransferPublish::Release`] the written pieces are released (their
/// transfers are appended to `released`); the image ends up
/// SHADER_READ_ONLY_OPTIMAL once acquired.
#[allow(clippy::too_many_arguments)]
pub(super) fn record_texture_copies_on_transfer(
    command: &mut VulkanCommandBuffer,
    texture: &Arc<Texture>,
    queue: QueueId,
    staging: vk::Buffer,
    base: u64,
    plan: &TexturePlan,
    old_layout: vk::ImageLayout,
    publish: TransferPublish,
    released: &mut Vec<crate::resource::range_state::RangeTransfer>,
) -> Result<()> {
    // SAFETY: every memory access below has a native state patch.
    unsafe {
        command.retain_recorded(texture.clone())?;
    }
    let range =
        record_transfer_dst_copies(command, texture, queue, staging, base, plan, old_layout)?;
    let destination_family = match publish {
        TransferPublish::Local => {
            return image_access_borrowed(
                command,
                texture,
                queue,
                range,
                AccessScope {
                    stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::empty(),
                    write: false,
                },
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
            );
        }
        TransferPublish::Release { destination_family } => destination_family,
    };
    let mut pieces: Vec<WrittenPiece> = Vec::new();
    command.with_recorded_state(
        &texture.last_use,
        crate::resource::range_state::NativeState::default(),
        queue,
        |table| {
            for cell in table.prepared_cells() {
                if crate::resource::range_state::intersection(cell.range, range).is_some() {
                    pieces.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                    pieces.push(WrittenPiece {
                        range: cell.range,
                        revision: cell.state.revision,
                    });
                }
            }
            Ok(())
        },
    )?;
    let mut patches = Vec::new();
    let mut scope = SynchronizationScope::new(
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::empty(),
    );
    let mut local = Vec::new();
    image_release_program(
        texture.handle,
        &pieces,
        command.pool.family,
        destination_family,
        &mut patches,
        &mut scope,
        &mut local,
    )?;
    for patch in patches {
        command.record_state_patch(texture.last_use.clone(), patch)?;
    }
    released
        .try_reserve(local.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    released.extend(local);
    scope.issue_into(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_aligns_each_texture_to_its_copy_alignment() {
        let plan = |total, alignment| TexturePlan {
            copies: Vec::new(),
            total,
            alignment,
        };
        // RGB8 (3-byte texels → 12-byte alignment), BC1 (8), BC7 (16).
        let plans = [plan(30, 12), plan(8, 8), plan(16, 16), plan(5, 12)];
        let (bases, total) = layout(plans.iter()).unwrap();
        assert_eq!(bases, [0, 32, 48, 72]);
        assert_eq!(total, 77);
        assert_eq!(align_up(13, 12).unwrap(), 24);
        assert!(align_up(u64::MAX, 16).is_err());
    }
}
