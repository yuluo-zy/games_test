//! One-shot setup recording and texture initialization/update.
//!
//! Texture uploads are asynchronous: they record state-derived barriers, copy
//! from a staging buffer retained by the command, and submit without a CPU
//! wait. The returned point is also the texture's tracked last use, so graph
//! and raw accesses (and destruction) order after it. `with_setup_command`
//! records an arbitrary opaque body and therefore stays synchronous, with a
//! conservative trailing barrier and a bounded wait.
use super::{
    command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
    device::Device,
    error::{Error, Result},
    format::vk_format_info,
    texture::Texture,
    texture_upload::{TextureUpdateRegion, copy_format_params, linear_regions, per_mip_regions},
};
use crate::sync::SyncPoint;
use crate::transfer::texture as upload;
use ash::vk;
use std::sync::Arc;

/// Upper bound for the synchronous setup helpers' completion wait.
pub const SETUP_WAIT_TIMEOUT_NS: u64 = 30_000_000_000;

/// Record `body` into a one-shot command over borrowed resources, add a full
/// memory barrier, submit, and wait (bounded) for its completion.
pub fn with_setup_resources<R>(
    device: &Arc<Device>,
    resources: &[&dyn crate::resource::GpuUse],
    body: impl FnOnce(vk::CommandBuffer) -> R,
) -> Result<R> {
    with_setup_command(device, |command| {
        for resource in resources {
            // The synchronous setup call retains these borrows until its wait;
            // a timed-out wait still leaves their tracked last use pending.
            unsafe {
                command.retain_borrowed_opaque(*resource)?;
            }
        }
        Ok(body(command.handle))
    })
}

/// Record an opaque body, then submit and wait for this command's real
/// completion point for at most [`SETUP_WAIT_TIMEOUT_NS`] (`Error::Timeout`).
/// The body's accesses are unknown, so a conservative memory barrier orders
/// them before every later command on the queue.
pub fn with_setup_command<R>(
    device: &Arc<Device>,
    body: impl FnOnce(&mut VulkanCommandBuffer) -> Result<R>,
) -> Result<R> {
    let pool = VulkanCommandPool::new(Arc::clone(device))?;
    let mut command = pool.allocate()?;
    command.begin()?;
    let result = body(&mut command)?;
    let barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
        .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE);
    let mut scope = super::barrier::SynchronizationScope::new(
        vk::PipelineStageFlags::ALL_COMMANDS,
        vk::PipelineStageFlags::ALL_COMMANDS,
    );
    scope.memory.push(barrier);
    unsafe {
        super::barrier::BarrierDispatch::new(device).issue(device, command.handle, &scope)?;
    }
    command.end()?;
    let pending = command.submit()?;
    pending.wait(SETUP_WAIT_TIMEOUT_NS)?;
    Ok(result)
}

/// Initial texture contents.
#[derive(Clone, Copy, Debug)]
pub enum TextureInitialData<'a> {
    /// Clear every subresource to zero (uncompressed formats).
    Zeroed,
    /// One tightly packed slice per mip level (uncompressed single-layer).
    PerMip(&'a [&'a [u8]]),
    /// Layer-major, mip-minor tightly packed data; supports block-compressed,
    /// array and cube textures (every face is a layer).
    Linear(&'a [u8]),
}

/// Upload a region table to `texture` without waiting. `old_layout` is
/// `UNDEFINED` for initial data or `SHADER_READ_ONLY_OPTIMAL` for updates; the
/// texture ends in SHADER_READ_ONLY_OPTIMAL. Returns the completion point.
pub fn upload_regions(
    device: &Arc<Device>,
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
    old_layout: vk::ImageLayout,
) -> Result<SyncPoint> {
    if !Arc::ptr_eq(device, &texture.device) {
        return Err(Error::invalid_usage("texture belongs to another device"));
    }
    let plan = upload::plan_texture_upload(texture, regions, old_layout)?;
    let mut bytes = Vec::new();
    upload::pack_all(&[(&plan, regions)], &[0], plan.total, &mut bytes)?;
    let staging = upload::new_staging(device, &bytes)?;
    let queue = device.queue(crate::device::QueueClass::Graphics)?;
    let pool = VulkanCommandPool::new(Arc::clone(device))?;
    let mut command = pool.allocate()?;
    // SAFETY: the copies read only this staging span.
    unsafe {
        command.retain_opaque_buffer_range(staging.clone(), 0, plan.total)?;
    }
    command.begin()?;
    upload::record_texture_copies(
        &mut command,
        texture,
        queue,
        staging.handle,
        0,
        &plan,
        old_layout,
    )?;
    command.end()?;
    // Dropping the receipt retires the command, staging and pool after the
    // real completion point; the texture's last use carries the same point.
    Ok(command.submit()?.point)
}

/// Regions of layer-major, mip-minor linear data covering the whole texture.
fn linear_regions_for<'a>(
    texture: &Texture,
    bytes: &'a [u8],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let info = vk_format_info(desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    copy_format_params(desc.format)?;
    let block = [
        u8::try_from(info.block_extent[0])
            .map_err(|_| Error::unsupported("copy block width is unsupported"))?,
        u8::try_from(info.block_extent[1])
            .map_err(|_| Error::unsupported("copy block height is unsupported"))?,
    ];
    linear_regions(
        bytes,
        [desc.width, desc.height, desc.depth],
        desc.mip_levels,
        desc.array_layers()?,
        info.block_bytes,
        block,
    )
}

/// Regions of per-mip slices; uncompressed single-layer textures only.
fn per_mip_regions_for<'a>(
    texture: &Texture,
    slices: &[&'a [u8]],
) -> Result<Vec<TextureUpdateRegion<'a>>> {
    let desc = &texture.desc;
    let info = vk_format_info(desc.format)
        .ok_or_else(|| Error::unsupported("texture format has no copy layout"))?;
    if info.block_extent != [1, 1, 1] || desc.array_layers()? != 1 {
        return Err(Error::unsupported(
            "per-mip slices require an uncompressed single layer; use linear data for compressed/cube/array textures",
        ));
    }
    let texel = vk_format_info(desc.format)
        .filter(|info| info.aspect == vk::ImageAspectFlags::COLOR)
        .map(|info| u64::from(info.block_bytes));
    per_mip_regions(
        slices,
        [desc.width, desc.height, desc.depth],
        desc.mip_levels,
        |item| {
            texel.map(|bytes| {
                u64::from(item.extent[0])
                    * u64::from(item.extent[1])
                    * u64::from(item.extent[2])
                    * bytes
            })
        },
    )
}

fn require_sampled(texture: &Texture) -> Result<()> {
    if texture.desc.usage.contains(vk::ImageUsageFlags::SAMPLED) {
        Ok(())
    } else {
        Err(Error::invalid_usage(
            "initialized textures require SAMPLED usage",
        ))
    }
}

fn require_transfer_dst(texture: &Texture) -> Result<()> {
    if texture
        .desc
        .usage
        .contains(vk::ImageUsageFlags::TRANSFER_DST)
    {
        Ok(())
    } else {
        Err(Error::invalid_usage("texture lacks TRANSFER_DST usage"))
    }
}

/// Initial upload of linear data (layer-major, mip-minor, every cube face a
/// layer). The image must be in UNDEFINED and ends in SHADER_READ_ONLY_OPTIMAL.
/// Does not wait; see [`upload_regions`].
pub fn init_texture_data(device: &Arc<Device>, texture: &Texture, bytes: &[u8]) -> Result<()> {
    init_texture_data_with(device, texture, TextureInitialData::Linear(bytes))
}

/// Initial contents in any [`TextureInitialData`] form. Does not wait.
pub fn init_texture_data_with(
    device: &Arc<Device>,
    texture: &Texture,
    data: TextureInitialData<'_>,
) -> Result<()> {
    init_texture(device, texture, data).map(drop)
}

/// [`init_texture_data_with`] returning the upload's completion point.
pub fn init_texture(
    device: &Arc<Device>,
    texture: &Texture,
    data: TextureInitialData<'_>,
) -> Result<SyncPoint> {
    require_sampled(texture)?;
    match data {
        TextureInitialData::Zeroed => zero_fill(device, texture),
        TextureInitialData::PerMip(slices) => {
            let regions = per_mip_regions_for(texture, slices)?;
            upload_regions(device, texture, &regions, vk::ImageLayout::UNDEFINED)
        }
        TextureInitialData::Linear(bytes) => {
            let regions = linear_regions_for(texture, bytes)?;
            upload_regions(device, texture, &regions, vk::ImageLayout::UNDEFINED)
        }
    }
}

/// Clear every subresource (all mips, all layers and cube faces) to zero and
/// leave the image in SHADER_READ_ONLY_OPTIMAL. Does not wait.
pub fn init_texture_zero_fill(device: &Arc<Device>, texture: &Texture) -> Result<()> {
    require_sampled(texture)?;
    zero_fill(device, texture).map(drop)
}

fn zero_fill(device: &Arc<Device>, texture: &Texture) -> Result<SyncPoint> {
    if !Arc::ptr_eq(device, &texture.device) {
        return Err(Error::invalid_usage("texture belongs to another device"));
    }
    let queue = device.queue(crate::device::QueueClass::Graphics)?;
    let pool = VulkanCommandPool::new(Arc::clone(device))?;
    let mut command = pool.allocate()?;
    command.begin()?;
    super::texture_upload::record_zero_fill_borrowed(
        &mut command,
        texture,
        queue,
        Some(vk::ImageLayout::UNDEFINED),
    )?;
    command.end()?;
    Ok(command.submit()?.point)
}

/// Replace the whole contents of an initialized texture (image in
/// SHADER_READ_ONLY_OPTIMAL) with linear data shaped like its initial data.
/// Does not wait; later accesses order after the upload through its state.
pub fn update_texture(device: &Arc<Device>, texture: &Texture, bytes: &[u8]) -> Result<()> {
    require_transfer_dst(texture)?;
    let regions = linear_regions_for(texture, bytes)?;
    upload_regions(
        device,
        texture,
        &regions,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )
    .map(drop)
}

/// Update explicit regions of an initialized texture. Does not wait.
pub fn update_texture_regions(
    device: &Arc<Device>,
    texture: &Texture,
    regions: &[TextureUpdateRegion<'_>],
) -> Result<()> {
    require_transfer_dst(texture)?;
    upload_regions(
        device,
        texture,
        regions,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )
    .map(drop)
}
