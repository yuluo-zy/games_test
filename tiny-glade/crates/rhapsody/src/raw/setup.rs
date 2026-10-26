//! Reusable setup recording and texture initialization/update.
//!
//! Setup work is recorded through a [`SetupContext`]: a small stack of
//! command pools whose command buffers are recycled once their own timeline
//! point completes, so no call creates a Vulkan pool or blocks on the GPU.
//! A body declares its memory accesses ([`SetupAccess`]); they are recorded
//! as precise state-derived barriers and tracked state before the body runs,
//! and later graph/raw users order after them through that state. Every call
//! returns the submission's [`SyncPoint`]; waiting is an explicit opt-in.
use crate::{Error, Result};
use super::{
    buffer::Buffer,
    command_buffer::{CommandPoolAllocation, VulkanCommandBuffer, VulkanCommandPool},
    graveyard::Doomed,
    device::Device,
    format::vk_format_info,
    texture::Texture,
    texture_upload::{TextureUpdateRegion, copy_format_params, linear_regions, per_mip_regions},
};
use crate::resource::{OwnershipRange, range_state::AccessScope};
use crate::sync::{SyncPoint, SyncSet};
use crate::transfer::texture as upload;
use ash::vk;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

/// Suggested bound for callers that wait on a setup submission.
pub const SETUP_WAIT_TIMEOUT_NS: u64 = 30_000_000_000;

/// Idle pools kept per context; concurrent or nested setup calls beyond it
/// create temporary pools.
const MAX_IDLE_POOLS: usize = 4;
/// Submitted command buffers kept for recycling per pool.
const MAX_RECYCLED_COMMANDS: usize = 16;

/// One declared memory access of a setup body. The access is recorded before
/// the body with barriers derived from the resource's tracked state; the body
/// must not touch memory it did not declare (except through helpers that
/// record their own tracked accesses, such as texture region uploads).
#[derive(Clone, Copy)]
pub enum SetupAccess<'a> {
    Buffer {
        buffer: &'a Buffer,
        offset: u64,
        size: u64,
        scope: AccessScope,
    },
    Image {
        texture: &'a Texture,
        /// `None` covers every aspect, mip and layer.
        range: Option<OwnershipRange>,
        scope: AccessScope,
        layout: vk::ImageLayout,
    },
}
impl<'a> SetupAccess<'a> {
    pub const TRANSFER_READ: AccessScope = AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_READ,
        write: false,
    };
    pub const TRANSFER_WRITE: AccessScope = AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_WRITE,
        write: true,
    };
    /// `buffer[offset..offset + size]` is read by transfer commands.
    pub fn transfer_read(buffer: &'a Buffer, offset: u64, size: u64) -> Self {
        Self::Buffer {
            buffer,
            offset,
            size,
            scope: Self::TRANSFER_READ,
        }
    }
    /// `buffer[offset..offset + size]` is written by transfer commands.
    pub fn transfer_write(buffer: &'a Buffer, offset: u64, size: u64) -> Self {
        Self::Buffer {
            buffer,
            offset,
            size,
            scope: Self::TRANSFER_WRITE,
        }
    }
    fn record(&self, command: &mut VulkanCommandBuffer, queue: crate::sync::QueueId) -> Result<()> {
        match *self {
            Self::Buffer {
                buffer,
                offset,
                size,
                scope,
            } => super::resource_access::buffer_access_borrowed(
                command, buffer, queue, offset, size, scope,
            ),
            Self::Image {
                texture,
                range,
                scope,
                layout,
            } => {
                let range = match range {
                    Some(range) => range,
                    None => Texture::ownership_range(&texture.desc)?,
                };
                super::resource_access::image_access_borrowed(
                    command, texture, queue, range, scope, layout, None,
                )
            }
        }
    }
}

/// A submitted setup command. Dropping it never blocks; the command and its
/// resources are recycled or retired after `point`.
#[must_use = "a setup submission carries the body's value and completion point"]
pub struct SetupSubmission<R> {
    pub value: R,
    pub point: SyncPoint,
    device: Arc<Device>,
}
impl<R> SetupSubmission<R> {
    /// Wait (bounded) for this submission only, then return the body's value.
    pub fn wait(self, timeout_ns: u64) -> Result<R> {
        self.device
            .wait(&SyncSet::from_point(self.point), timeout_ns)?;
        Ok(self.value)
    }
    pub fn is_complete(&self) -> Result<bool> {
        Ok(SyncSet::from_point(self.point).is_reached(&self.device.poll()?))
    }
    pub fn into_value(self) -> R {
        self.value
    }
}

/// One native command pool parked in the device with its recycled command
/// buffers. It holds **no** reference to the device (only the pool's own
/// allocation and raw handles), so keeping it in the device's caches forms no
/// cycle: a recording call re-wraps it around a temporary `Arc<Device>`.
/// Used by one recording thread at a time (Vulkan pools are externally
/// synchronized).
pub(crate) struct SetupPool {
    allocation: Arc<CommandPoolAllocation>,
    family: u32,
    /// Submitted command buffers and the completion of each, oldest first.
    in_flight: VecDeque<(SyncSet, vk::CommandBuffer)>,
}
/// Counters of the device's setup recording (statistics only).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SetupStats {
    pub pools_created: u64,
    pub commands_allocated: u64,
    pub commands_recycled: u64,
    pub submissions: u64,
}

/// Parked setup pools and counters, owned by the device's caches.
#[derive(Default)]
pub(crate) struct SetupCache {
    idle: Mutex<Vec<SetupPool>>,
    stats: Mutex<SetupStats>,
}
impl SetupCache {
    /// Destroy every parked pool. Only valid once the GPU is idle (device
    /// drop): native command buffers die with their pool.
    pub(crate) fn drain_idle(&self) {
        self.idle.lock().clear();
    }
}

impl SetupPool {
    fn new(device: &Arc<Device>) -> Result<Self> {
        let pool = VulkanCommandPool::new(device.clone())?;
        Ok(Self {
            allocation: pool.allocation(),
            family: pool.family,
            in_flight: VecDeque::new(),
        })
    }
    /// A reset command buffer wrapped for recording: the oldest recycled one
    /// whose submissions completed, or a new allocation.
    fn command(
        &mut self,
        device: &Arc<Device>,
        pool: &Arc<VulkanCommandPool>,
        stats: &Mutex<SetupStats>,
    ) -> Result<VulkanCommandBuffer> {
        // Cached completion alone never advances when setup is the only GPU
        // producer. Poll outstanding work once before allocating another command.
        let mut completed = device.completed();
        if self.in_flight.front().is_some_and(|(done, _)| !done.is_reached(&completed)) {
            completed = device.poll()?;
        }
        if self
            .in_flight
            .front()
            .is_some_and(|(done, _)| done.is_reached(&completed))
        {
            let (_, handle) = self.in_flight.pop_front().expect("checked front");
            match pool.adopt(handle) {
                Ok(mut command) => {
                    // SAFETY: every submission of this buffer reached its
                    // timeline point (checked against `completed` above).
                    if unsafe { command.reset_after_completion() }.is_ok() {
                        stats.lock().commands_recycled += 1;
                        return Ok(command);
                    }
                    // Dropping retires the buffer after its own points.
                }
                Err(_) => self.allocation.free(handle),
            }
        }
        let command = pool.allocate()?;
        stats.lock().commands_allocated += 1;
        Ok(command)
    }
    /// Park a submitted command for recycling after `done`.
    fn park(&mut self, device: &Device, done: SyncSet, handle: vk::CommandBuffer) {
        if self.in_flight.len() >= MAX_RECYCLED_COMMANDS
            && let Some((oldest, old)) = self.in_flight.pop_front()
        {
            device.retire(oldest, Doomed::CommandBuffer(old, self.allocation.clone()));
        }
        if self.in_flight.try_reserve(1).is_ok() {
            self.in_flight.push_back((done, handle));
        } else {
            device.retire(done, Doomed::CommandBuffer(handle, self.allocation.clone()));
        }
    }
    /// Give up this pool while the GPU may still use its buffers: each is
    /// freed after its own completion; the native pool is destroyed after the
    /// last of them.
    fn retire(self, device: &Device) {
        for (done, handle) in self.in_flight {
            device.retire(done, Doomed::CommandBuffer(handle, self.allocation.clone()));
        }
    }
}

/// Reusable setup recording for one device: a cheap handle over the pools the
/// device itself keeps ([`Device::setup_context`]). Pools are reused across
/// calls whether or not any caller keeps a handle: the device owns them
/// without owning itself (see [`SetupPool`]), and tears them down in its
/// structural drop order after the final GPU wait.
#[derive(Clone)]
pub struct SetupContext {
    device: Arc<Device>,
}
impl SetupContext {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        Ok(Self { device })
    }
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Counters shared by every handle of this device.
    pub fn stats(&self) -> SetupStats {
        *self.device.caches.setup.stats.lock()
    }
    fn take_pool(&self) -> Result<SetupPool> {
        if let Some(pool) = self.device.caches.setup.idle.lock().pop() {
            return Ok(pool);
        }
        let pool = SetupPool::new(&self.device)?;
        self.device.caches.setup.stats.lock().pools_created += 1;
        Ok(pool)
    }
    fn return_pool(&self, pool: SetupPool) {
        let cache = &self.device.caches.setup;
        let mut idle = cache.idle.lock();
        if idle.len() < MAX_IDLE_POOLS && idle.try_reserve(1).is_ok() {
            idle.push(pool);
            return;
        }
        drop(idle);
        pool.retire(&self.device);
    }
    /// Record the declared accesses, then `body`, and submit on the graphics
    /// queue without waiting. A failed body or submit publishes no state.
    pub fn record<R>(
        &self,
        accesses: &[SetupAccess<'_>],
        body: impl FnOnce(&mut VulkanCommandBuffer) -> Result<R>,
    ) -> Result<SetupSubmission<R>> {
        let queue = self.device.queue(crate::device::QueueClass::Graphics)?;
        let mut pool = self.take_pool()?;
        let result = (|| {
            // Temporary wrapper; the device is only referenced for this call.
            let live = VulkanCommandPool::attach(
                self.device.clone(),
                pool.family,
                pool.allocation.clone(),
            )?;
            let mut command = pool.command(&self.device, &live, &self.device.caches.setup.stats)?;
            command.begin()?;
            for access in accesses {
                access.record(&mut command, queue)?;
            }
            let value = body(&mut command)?;
            command.end()?;
            let pending = command.submit()?;
            let point = pending.point;
            let (handle, done) = pending.into_command().detach();
            pool.park(&self.device, done, handle);
            self.device.caches.setup.stats.lock().submissions += 1;
            Ok(SetupSubmission {
                value,
                point,
                device: self.device.clone(),
            })
        })();
        self.return_pool(pool);
        result
    }
}

/// [`SetupContext::record`] on the device's shared setup context.
pub fn with_setup_command<R>(
    device: &Arc<Device>,
    accesses: &[SetupAccess<'_>],
    body: impl FnOnce(&mut VulkanCommandBuffer) -> Result<R>,
) -> Result<SetupSubmission<R>> {
    device.setup_context()?.record(accesses, body)
}

/// [`with_setup_command`] for a body that records raw commands only.
pub fn with_setup_resources<R>(
    device: &Arc<Device>,
    accesses: &[SetupAccess<'_>],
    body: impl FnOnce(vk::CommandBuffer) -> R,
) -> Result<SetupSubmission<R>> {
    with_setup_command(device, accesses, |command| Ok(body(command.handle)))
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
    // The staging buffer's own state records this read; its storage retires
    // after the returned point. The texture's last use carries the same point.
    let submission = with_setup_command(
        device,
        &[SetupAccess::transfer_read(&staging, 0, plan.total)],
        |command| {
            upload::record_texture_copies(
                command,
                texture,
                queue,
                staging.handle,
                0,
                &plan,
                old_layout,
            )
        },
    )?;
    Ok(submission.point)
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
/// Does not wait; see `upload_regions`.
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
    let submission = with_setup_command(device, &[], |command| {
        super::texture_upload::record_zero_fill_borrowed(
            command,
            texture,
            queue,
            Some(vk::ImageLayout::UNDEFINED),
        )
    })?;
    Ok(submission.point)
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
