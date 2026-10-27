//! WSI creation, per-slot acquisition, display submission, presentation and
//! deferred recreation.
//!
//! Locking: `wsi` serialises every native call that externally synchronises
//! the swapchain handle (acquire, present, release, fullscreen acquire/release
//! and creation with `oldSwapchain`). `inner` guards bookkeeping only and is
//! never held across a blocking fence wait or a native WSI call, so state
//! queries (`desc`, `resize`, focus) never wait behind an acquire. Lock order:
//! `wsi` -> `inner` -> per-slot/per-image mutexes -> device queue lock.
use crate::{Error, Result};
use crate::device::Device;
use crate::native::Surface;
mod fullscreen;
mod graph_presentation;
use fullscreen::FullscreenState;
pub use fullscreen::{FullscreenFallback, FullscreenStatus, WindowMode};
pub(crate) use graph_presentation::{GraphPresentation, PresentSync};

use ash::vk;
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
mod wsi;
mod policy;
mod progress;
mod frame_sync;
mod lease;
mod recreate;
mod acquire;
mod retire;
use wsi::present_result;
pub use policy::{SwapchainPolicy, PresentPolicy};
use progress::{ImagePresentation, PresentProgress};
use frame_sync::FrameSemaphores;
pub use lease::{SwapchainImageLease, AcquiredImageLease, PresentTicket};

static NEXT_SWAPCHAIN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static RETAINED_GENERATIONS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Upper bound in nanoseconds for internal acquire and WSI fence waits.
pub const ACQUIRE_TIMEOUT_NS: u64 = 3_000_000_000;
/// Bound pending generations when an unextended presentation engine has not
/// produced a retirement proof yet. The current generation is additional.
pub const MAX_RETIRED_GENERATIONS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapchainDesc {
    pub extent: vk::Extent2D,
    pub vsync: bool,
    pub usage: vk::ImageUsageFlags,
}

/// Classification of one acquisition attempt by
/// [`Swapchain::validate_swapchain_and_acquire_image`].
enum Acquired {
    Frame(AcquiredImageLease),
    NoImage,
    Stale,
    Retired,
    Failed(Error),
}

/// Requested extent is zero (minimised window): skip the frame, do not rebuild.
const ZERO_WINDOW_EXTENT: &str = "窗口尺寸为零，延迟交换链重建";
/// Surface reports a zero current extent (Windows while minimised): skip too.
const ZERO_SURFACE_EXTENT: &str = "表面尺寸为零，等待窗口恢复";
fn is_zero_extent(error: &Error) -> bool {
    matches!(error, Error::InvalidUsage { what, .. } if what == ZERO_WINDOW_EXTENT || what == ZERO_SURFACE_EXTENT)
}
/// Mutable bookkeeping of one swapchain generation. Handles, format, images
/// and views never change after creation; recreation produces a new
/// [`Swapchain`].
pub(crate) struct SwapchainInner {
    desc: SwapchainDesc,
    fullscreen: FullscreenState,
    state: SwapchainState,
    leases: Vec<Arc<FrameSemaphores>>,
    presentations: Vec<Arc<ImagePresentation>>,
    next_slot: usize,
    stale: bool,
    any_presented: bool,
    native_present_successes: u64,
    active_acquisitions: usize,
    closing: bool,
    unproven_external_acquire: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SwapchainState {
    NotCreated,
    Created,
}
impl SwapchainInner {
    /// Validates that a new acquisition may start. Focus only gates
    /// acquisition for a generation that is actually in exclusive fullscreen;
    /// windowed and borderless windows keep rendering while unfocused.
    fn check_acquire(&self) -> Result<()> {
        if self.closing {
            return Err(Error::invalid_usage("交换链正在关闭"));
        }
        if self.fullscreen.paused() || self.desc.extent.width == 0 || self.desc.extent.height == 0 {
            return Err(vk::Result::NOT_READY.into());
        }
        if self.fullscreen.monitor.is_some() && !self.fullscreen.status.acquired {
            return Err(Error::SurfaceOutOfDate);
        }
        if self.stale || self.state != SwapchainState::Created {
            return Err(Error::SurfaceOutOfDate);
        }
        Ok(())
    }
    /// Publishes the outcome of a native acquire started after `check_acquire`.
    fn commit_acquire(&mut self, result: Result<(u32, bool)>) -> Result<(u32, bool)> {
        match result {
            Ok(image) => {
                self.acquired();
                Ok(image)
            }
            Err(error) => {
                if matches!(error, Error::SurfaceOutOfDate) {
                    self.stale = true;
                }
                Err(error)
            }
        }
    }
    #[cfg(test)]
    fn acquire_with(
        &mut self,
        acquire: impl FnOnce() -> Result<(u32, bool)>,
    ) -> Result<(u32, bool)> {
        self.check_acquire()?;
        let result = acquire();
        self.commit_acquire(result)
    }
    /// Publishes the outcome of a native present; returns SUBOPTIMAL.
    fn finish_present(
        &mut self,
        progress: &mut PresentProgress,
        has_fence: bool,
        result: std::result::Result<bool, vk::Result>,
    ) -> Result<bool> {
        if result == Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT) {
            self.fullscreen.lost();
        }
        progress.record_result(result, has_fence);
        // 可恢复的过期结果仍保留原入队/重建语义，不能算作实际呈现成功。
        if result.is_ok() {
            self.native_present_successes = self.native_present_successes.saturating_add(1);
        }
        match present_result(result) {
            Ok((suboptimal, out_of_date)) => {
                self.any_presented = true;
                self.stale |= out_of_date;
                Ok(suboptimal)
            }
            Err(error) => {
                self.stale = true;
                Err(error)
            }
        }
    }
    fn can_replace(&self) -> bool {
        self.active_acquisitions == 0
    }
    fn acquired(&mut self) {
        self.active_acquisitions += 1;
    }
    fn return_acquisition(&mut self, released: bool) {
        self.active_acquisitions -= 1;
        if !released {
            self.stale = true;
        }
    }
    fn new(desc: SwapchainDesc) -> Self {
        Self {
            desc,
            fullscreen: FullscreenState::default(),
            state: SwapchainState::NotCreated,
            leases: Vec::new(),
            presentations: Vec::new(),
            next_slot: 0,
            stale: true,
            any_presented: false,
            native_present_successes: 0,
            active_acquisitions: 0,
            closing: false,
            unproven_external_acquire: false,
        }
    }
}

pub struct Swapchain {
    pub(crate) device: Arc<Device>,
    pub surface: Arc<Surface>,
    pub(crate) handle: vk::SwapchainKHR,
    pub format: vk::Format,
    // 真实创建所用的颜色空间；创建成功前不能以默认枚举值冒充已选择值。
    color_space: Option<vk::ColorSpaceKHR>,
    pub extent: vk::Extent2D,
    /// Actual mode selected after querying the surface; recorded for performance runs.
    pub present_mode: vk::PresentModeKHR,
    pub(crate) images: Vec<vk::Image>,
    pub(crate) views: Vec<vk::ImageView>,
    pub(crate) loader: ash::khr::swapchain::Device,
    vsync: bool,
    usage: vk::ImageUsageFlags,
    pre_transform: vk::SurfaceTransformFlagsKHR,
    /// Serialises native calls that externally synchronise `handle`.
    wsi: Mutex<()>,
    inner: RwLock<SwapchainInner>,
    generation: u64,
    policy: SwapchainPolicy,
    retired_generations: Mutex<Vec<Arc<Swapchain>>>,
}

impl Swapchain {
    /// The device that owns this swapchain.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::SwapchainKHR {
        self.handle
    }
    /// Interop escape hatch: the swapchain images (owned by the swapchain).
    pub fn images(&self) -> &[vk::Image] {
        &self.images
    }
    /// Interop escape hatch: one view per swapchain image (owned by the swapchain).
    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.views
    }
    /// Interop escape hatch: the extension function table.
    pub fn loader(&self) -> &ash::khr::swapchain::Device {
        &self.loader
    }
}
impl Swapchain {
    pub fn policy(&self) -> SwapchainPolicy {
        self.policy
    }
    pub fn present_mode(&self) -> vk::PresentModeKHR {
        self.present_mode
    }
    /// The requested (not necessarily created) description.
    pub fn desc(&self) -> SwapchainDesc {
        self.inner.read().desc
    }
    pub fn format(&self) -> vk::Format {
        self.format
    }
    /// 当前原生交换链实际创建使用的格式与颜色空间，不是表面支持列表。
    pub fn surface_format(&self) -> Option<vk::SurfaceFormatKHR> {
        self.color_space.map(|color_space| vk::SurfaceFormatKHR {
            format: self.format,
            color_space,
        })
    }
}
impl Drop for Swapchain {
    fn drop(&mut self) {
        if let Err(error) = self.shutdown(ACQUIRE_TIMEOUT_NS)
            && !error.is_device_lost()
        {
            // A failed shutdown cannot manufacture final presentation proof.
            // Keep all owners alive, including the device
            // and surface, so neither a child nor its parent is destroyed early.
            // This is an observable safety fallback, never successful cleanup.
            log::error!(target: "rhapsody::raw::swapchain", "WSI generation {} retained after failed shutdown ({:?}): {error}", self.generation, self.policy);
            RETAINED_GENERATIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            std::mem::forget(self.device.clone());
            std::mem::forget(self.surface.clone());
            std::mem::forget(std::mem::take(&mut self.inner.get_mut().leases));
            std::mem::forget(std::mem::take(&mut self.inner.get_mut().presentations));
            std::mem::forget(std::mem::take(self.retired_generations.get_mut()));
            return;
        }
        unsafe {
            for view in self.views.drain(..) {
                self.device.core.device.destroy_image_view(view, None);
            }
            if self.handle != vk::SwapchainKHR::null() {
                self.loader.destroy_swapchain(self.handle, None);
            }
        }
    }
}

impl crate::tracking::GpuUse for Swapchain {
    fn visit_uses(&self, _visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests;

// WSI owners retain the existing presentation completion protocol. GPU use cells
// must not be interpreted as proof that presentation resources can be released.
