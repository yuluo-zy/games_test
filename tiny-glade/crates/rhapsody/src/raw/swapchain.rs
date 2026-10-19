//! WSI creation, per-slot acquisition, display submission, presentation and
//! deferred recreation.
//!
//! Locking: `wsi` serialises every native call that externally synchronises
//! the swapchain handle (acquire, present, release, fullscreen acquire/release
//! and creation with `oldSwapchain`). `inner` guards bookkeeping only and is
//! never held across a blocking fence wait or a native WSI call, so state
//! queries (`desc`, `resize`, focus) never wait behind an acquire. Lock order:
//! `wsi` -> `inner` -> per-slot/per-image mutexes -> device queue lock.
use super::{
    command_buffer::{VulkanCommandBuffer, VulkanCommandPool},
    device::Device,
    error::{Error, Result},
    instance::VulkanInstance,
};
mod fullscreen;
mod graph_presentation;
use fullscreen::FullscreenState;
pub use fullscreen::{FullscreenFallback, FullscreenStatus, WindowMode};
pub(crate) use graph_presentation::{GraphPresentation, PresentSync};

use ash::vk;
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;

// All fallible WSI device calls share the submission runtime's sticky loss
// state. Inspecting a VkResult only at the FrameLoop boundary is too late:
// acquired-image Drop may otherwise attempt a rescue submit while unwinding.
fn wsi_call<A: crate::sync::timeline::TimelineApi, T, E: Into<Error>>(
    timelines: &crate::sync::timeline::Timelines<A>,
    call: impl FnOnce() -> std::result::Result<T, E>,
) -> Result<T> {
    if timelines.is_device_lost() {
        return Err(Error::DeviceLost);
    }
    let result = call().map_err(Into::into);
    if let Err(error) = &result
        && error.is_device_lost()
    {
        timelines.mark_device_lost();
    }
    result
}

/// The fence boundary is shared by the live WSI state machine and CPU fault
/// injection. Failed waits must leave ownership intact and prevent any reset.
trait WsiFenceOperations {
    fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()>;
    fn reset(&self, fence: vk::Fence) -> Result<()>;
    fn signaled(&self, fence: vk::Fence) -> Result<bool>;
}
struct DeviceFenceOperations<'a>(&'a Device);
impl WsiFenceOperations for DeviceFenceOperations<'_> {
    fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.wait_for_fences(&[fence], true, timeout)
        })
    }
    fn reset(&self, fence: vk::Fence) -> Result<()> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.reset_fences(&[fence])
        })
    }
    fn signaled(&self, fence: vk::Fence) -> Result<bool> {
        wsi_call(&self.0.queues.timelines, || unsafe {
            self.0.core.device.get_fence_status(fence)
        })
    }
}

static NEXT_SWAPCHAIN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static RETAINED_GENERATIONS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Upper bound in nanoseconds for internal acquire and WSI fence waits.
pub const ACQUIRE_TIMEOUT_NS: u64 = 3_000_000_000;
/// Bound pending generations when an unextended presentation engine has not
/// produced a retirement proof yet. The current generation is additional.
pub const MAX_RETIRED_GENERATIONS: usize = 4;

/// Completion guarantees required by a swapchain owner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapchainPolicy {
    /// Production presentation and provable final teardown require maintenance1.
    #[default]
    Strict,
    /// Acquisition tests and offscreen consumers; presentation is rejected.
    AcquireOnly,
    /// Explicit compatibility mode. A final unproven generation is retained,
    /// including its device, rather than destroyed after an unrelated GPU wait.
    RetainUnproven,
}
impl SwapchainPolicy {
    fn validate(self, maintenance1: bool) -> Result<()> {
        if self == Self::Strict && !maintenance1 {
            Err(Error::unsupported(
                "严格呈现关闭需要 swapchainMaintenance1；只获取图像可选择 AcquireOnly",
            ))
        } else {
            Ok(())
        }
    }
    fn allow_present(self) -> Result<()> {
        if self == Self::AcquireOnly {
            Err(Error::unsupported("AcquireOnly 交换链不允许呈现"))
        } else {
            Ok(())
        }
    }
}

fn check_retired_generation_budget(pending: usize) -> Result<()> {
    if pending >= MAX_RETIRED_GENERATIONS {
        Err(Error::exhausted(
            "retired swapchain generations await completion proofs",
        ))
    } else {
        Ok(())
    }
}

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

/// Whether the present's queue operation (and therefore its semaphore wait)
/// was enqueued. Per vkQueuePresentKHR, OUT_OF_DATE, SURFACE_LOST and
/// FULL_SCREEN_EXCLUSIVE_MODE_LOST still enqueue the wait.
fn present_was_enqueued(result: &std::result::Result<bool, vk::Result>) -> bool {
    matches!(
        result,
        Ok(_)
            | Err(vk::Result::ERROR_OUT_OF_DATE_KHR
                | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
                | vk::Result::ERROR_SURFACE_LOST_KHR)
    )
}

/// Whether a present fence is guaranteed to signal for this result. The
/// maintenance1 specification does not guarantee a signal for presents that
/// the presentation engine rejected, and drivers have been observed to leave
/// those fences unsignaled; such fences are only polled, never waited on.
fn present_fence_is_certain(result: &std::result::Result<bool, vk::Result>) -> bool {
    result.is_ok()
}

/// Maps a present result to `(suboptimal, out_of_date)`. SUBOPTIMAL does not
/// by itself retire the generation (see [`Swapchain::note_suboptimal`]);
/// SURFACE_LOST is a failure.
fn present_result(result: std::result::Result<bool, vk::Result>) -> Result<(bool, bool)> {
    match result {
        Ok(suboptimal) => Ok((suboptimal, false)),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok((false, true)),
        Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT) => Ok((false, true)),
        Err(error) => Err(error.into()),
    }
}
/// Requested extent is zero (minimised window): skip the frame, do not rebuild.
const ZERO_WINDOW_EXTENT: &str = "窗口尺寸为零，延迟交换链重建";
/// Surface reports a zero current extent (Windows while minimised): skip too.
const ZERO_SURFACE_EXTENT: &str = "表面尺寸为零，等待窗口恢复";
fn is_zero_extent(error: &Error) -> bool {
    matches!(error, Error::InvalidUsage { what, .. } if what == ZERO_WINDOW_EXTENT || what == ZERO_SURFACE_EXTENT)
}
pub struct Surface {
    pub instance: Arc<VulkanInstance>,
    pub handle: vk::SurfaceKHR,
    pub loader: ash::khr::surface::Instance,
    owned: bool,
    win32: bool,
    /// Sticky VK_ERROR_SURFACE_LOST_KHR. A lost surface never recovers; the
    /// host must create a new one and call [`Swapchain::replace_surface`].
    lost: std::sync::atomic::AtomicBool,
}
impl Surface {
    /// True once any WSI call on this surface reported SURFACE_LOST.
    pub fn is_lost(&self) -> bool {
        self.lost.load(std::sync::atomic::Ordering::Acquire)
    }
    fn observe<T>(&self, result: &Result<T>) {
        if matches!(result, Err(Error::SurfaceLost)) {
            self.lost.store(true, std::sync::atomic::Ordering::Release);
        }
    }
    /// # Safety
    /// handle 必须属于给定实例且在此对象存活期间有效；owned=true 时转移销毁责任。
    pub unsafe fn from_raw(
        instance: Arc<VulkanInstance>,
        handle: vk::SurfaceKHR,
        owned: bool,
    ) -> Self {
        let loader = ash::khr::surface::Instance::new(&instance.entry, &instance.instance);
        Self {
            instance,
            handle,
            loader,
            owned,
            win32: false,
            lost: std::sync::atomic::AtomicBool::new(false),
        }
    }
    /// Wrap an existing Win32 Vulkan surface, allowing explicit fullscreen.
    ///
    /// # Safety
    /// In addition to from_raw's requirements, handle must have been created
    /// with vkCreateWin32SurfaceKHR and its HWND must outlive this surface.
    pub unsafe fn from_win32_raw(
        instance: Arc<VulkanInstance>,
        handle: vk::SurfaceKHR,
        owned: bool,
    ) -> Self {
        let mut surface = unsafe { Self::from_raw(instance, handle, owned) };
        surface.win32 = true;
        surface
    }
    /// # Safety
    /// Windows 句柄必须有效，且实例已经启用 surface 与 win32_surface 扩展。
    pub unsafe fn from_win32(
        instance: Arc<VulkanInstance>,
        hinstance: vk::HINSTANCE,
        hwnd: vk::HWND,
    ) -> Result<Arc<Self>> {
        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(hinstance)
            .hwnd(hwnd);
        let handle = unsafe { loader.create_win32_surface(&info, None) }?;
        Ok(Arc::new(unsafe {
            Self::from_win32_raw(instance, handle, true)
        }))
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                self.loader.destroy_surface(self.handle, None);
            }
        }
    }
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
        progress.record_result(&result, has_fence);
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
            active_acquisitions: 0,
            closing: false,
            unproven_external_acquire: false,
        }
    }
}

pub struct Swapchain {
    pub device: Arc<Device>,
    pub surface: Arc<Surface>,
    pub handle: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    /// Actual mode selected after querying the surface; recorded for performance runs.
    pub present_mode: vk::PresentModeKHR,
    pub images: Vec<vk::Image>,
    pub views: Vec<vk::ImageView>,
    pub loader: ash::khr::swapchain::Device,
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
    /// Generations permanently retained because teardown could not prove that
    /// all WSI operations completed. A successful strict shutdown leaves this
    /// process-wide diagnostic unchanged.
    pub fn retained_generation_count() -> u64 {
        RETAINED_GENERATIONS.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn policy(&self) -> SwapchainPolicy {
        self.policy
    }
    pub fn present_mode(&self) -> vk::PresentModeKHR {
        self.present_mode
    }
    pub fn is_closing(&self) -> bool {
        self.inner.read().closing
    }
    /// Process-unique identity of this generation; recreation and surface
    /// replacement produce a new value.
    pub fn generation(&self) -> u64 {
        self.generation
    }
    /// Poll acquisition and presentation completion proofs of this and all
    /// retired generations, releasing completed ancestors. Non-blocking.
    pub fn collect_presentations(&self) -> Result<()> {
        let inner = self.inner.read();
        for slot in &inner.leases {
            let mut state = slot.state.lock();
            if state.acquired
                && wsi_call(&self.device.queues.timelines, || unsafe {
                    self.device.core.device.get_fence_status(slot.acquire_ready)
                })?
            {
                slot.finish_acquire(&mut state, 0)?;
            }
        }
        let mut new_generation_present_completed = false;
        for image in &inner.presentations {
            image.poll()?;
            new_generation_present_completed |= image.progress.lock().completed > 0;
        }
        drop(inner);
        let mut retired = self.retired_generations.lock();
        for previous in retired.iter() {
            previous.collect_presentations()?;
        }
        if new_generation_present_completed {
            // Completion of this generation's first present also proves that
            // the presentation engine has stopped using all older generations.
            for previous in retired.iter() {
                previous.confirm_generation_complete();
            }
            retired.clear();
        } else {
            // Each generation may also finish through its own present fences
            // or reacquire proofs. Acquire-only generations have no presents.
            let mut index = 0;
            while index < retired.len() {
                if retired[index].retirement_ready()? {
                    retired.swap_remove(index);
                } else {
                    index += 1;
                }
            }
        }
        Ok(())
    }
    fn confirm_generation_complete(&self) {
        let inner = self.inner.read();
        for image in &inner.presentations {
            let mut progress = image.progress.lock();
            progress.generation_completed();
        }
        for previous in self.retired_generations.lock().iter() {
            previous.confirm_generation_complete();
        }
    }
    /// Stop new acquisitions and wait for actual acquire, GPU and presentation
    /// completion proofs. On failure the caller still owns this generation and
    /// may retry. No queue/device-idle result is used as a present proof.
    pub fn shutdown(&self, timeout: u64) -> Result<()> {
        let wsi = self.wsi.lock();
        let (leases, presentations) = {
            let mut inner = self.inner.write();
            if !inner.can_replace() {
                return Err(vk::Result::NOT_READY.into());
            }
            inner.closing = true;
            self.release_fullscreen(&mut inner)?;
            if inner.unproven_external_acquire {
                return Err(Error::Timeout);
            }
            (inner.leases.clone(), inner.presentations.clone())
        };
        // `closing` and the WSI lock exclude new acquisitions and presents;
        // the bounded waits below run without the state lock.
        for slot in &leases {
            slot.finish_acquire(&mut slot.state.lock(), timeout)?;
        }
        for image in &presentations {
            image.finish_gpu_use(timeout)?;
            image.finish_shutdown(timeout)?;
        }
        drop(wsi);
        // A completed present in this generation can finish unextended older
        // generations; acquire-only generations cannot supply that proof.
        self.collect_presentations()?;
        for previous in self.retired_generations.lock().iter() {
            previous.shutdown(timeout)?;
        }
        Ok(())
    }
    fn retirement_ready(&self) -> Result<bool> {
        let inner = self.inner.read();
        if !inner.can_replace() {
            return Ok(false);
        }
        for slot in &inner.leases {
            let state = slot.state.lock();
            if state.acquired
                || (state.submitted
                    && !wsi_call(&self.device.queues.timelines, || unsafe {
                        self.device.core.device.get_fence_status(slot.fence)
                    })?)
            {
                return Ok(false);
            }
        }
        let completed = self.device.queues.timelines.poll()?;
        Ok(inner
            .presentations
            .iter()
            .all(|image| image.progress.lock().retirement_ready(&completed)))
    }
    /// Pending generations, including all ancestors retained by older chains.
    /// The replacement budget bounds this traversal to MAX_RETIRED_GENERATIONS.
    pub fn retired_generation_count(&self) -> usize {
        self.retired_generations
            .lock()
            .iter()
            .map(|previous| 1 + previous.retired_generation_count())
            .sum()
    }
    pub fn any_present_successful(&self) -> bool {
        self.inner.read().any_presented
    }
    /// # Safety
    /// 设备必须启用 VK_KHR_swapchain，surface 必须属于设备实例。
    pub unsafe fn new(
        device: Arc<Device>,
        surface: Arc<Surface>,
        extent: vk::Extent2D,
        vsync: bool,
        usage: vk::ImageUsageFlags,
    ) -> Result<Self> {
        unsafe {
            Self::new_with_policy(
                device,
                surface,
                extent,
                vsync,
                usage,
                SwapchainPolicy::Strict,
            )
        }
    }
    /// Choose an explicit capability degradation for acquisition-only tests or
    /// compatibility consumers. Production frame loops require `Strict`.
    ///
    /// # Safety
    /// The device must enable VK_KHR_swapchain and own the surface's instance.
    pub unsafe fn new_with_policy(
        device: Arc<Device>,
        surface: Arc<Surface>,
        extent: vk::Extent2D,
        vsync: bool,
        usage: vk::ImageUsageFlags,
        policy: SwapchainPolicy,
    ) -> Result<Self> {
        policy.validate(
            device
                .caps()
                .is_enabled(crate::device::Feature::SwapchainMaintenance1),
        )?;
        unsafe {
            Self::new_with_old(
                device,
                surface,
                SwapchainDesc {
                    extent,
                    vsync,
                    usage,
                },
                vk::SwapchainKHR::null(),
                policy,
                FullscreenState::default(),
                &mut false,
            )
        }
    }
    unsafe fn new_with_old(
        device: Arc<Device>,
        surface: Arc<Surface>,
        desc: SwapchainDesc,
        old: vk::SwapchainKHR,
        policy: SwapchainPolicy,
        fullscreen: FullscreenState,
        old_retired: &mut bool,
    ) -> Result<Self> {
        let SwapchainDesc {
            extent,
            vsync,
            usage,
        } = desc;
        if !Arc::ptr_eq(&device.core.instance, &surface.instance) {
            return Err(Error::invalid_usage("交换链表面属于其他实例"));
        }
        if !wsi_call(&device.queues.timelines, || unsafe {
            surface.loader.get_physical_device_surface_support(
                device.core.physical,
                device.queues.primary_family,
                surface.handle,
            )
        })? {
            return Err(Error::unsupported("队列不支持呈现"));
        }
        // Reserve the ancestry holder before vkCreateSwapchain can retire old.
        // Publishing the successful replacement must not allocate afterward.
        let mut retired_generations = Vec::new();
        retired_generations
            .try_reserve(1)
            .map_err(|_| Error::OutOfHostMemory)?;
        let loader =
            ash::khr::swapchain::Device::new(&device.core.instance.instance, &device.core.device);
        let mut result = Self {
            retired_generations: Mutex::new(retired_generations),
            generation: NEXT_SWAPCHAIN.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            policy,
            device,
            surface,
            handle: vk::SwapchainKHR::null(),
            format: vk::Format::UNDEFINED,
            extent,
            present_mode: vk::PresentModeKHR::FIFO,
            images: Vec::new(),
            views: Vec::new(),
            loader,
            vsync,
            usage,
            pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
            wsi: Mutex::new(()),
            inner: RwLock::new(SwapchainInner::new(SwapchainDesc {
                extent,
                vsync,
                usage,
            })),
        };
        result.inner.get_mut().fullscreen =
            fullscreen.replacement(result.fullscreen_extension_enabled());
        let created = result.recreate(extent, old, old_retired);
        result.surface.observe(&created);
        created?;
        Ok(result)
    }
    pub fn enumerate_surface_formats(&self) -> Result<Vec<vk::SurfaceFormatKHR>> {
        wsi_call(&self.device.queues.timelines, || unsafe {
            self.surface
                .loader
                .get_physical_device_surface_formats(self.device.core.physical, self.surface.handle)
        })
    }
    /// Whether a 2D OPTIMAL image of this format supports the requested usage.
    fn format_supports_usage(&self, format: vk::Format) -> Result<bool> {
        match wsi_call(&self.device.queues.timelines, || unsafe {
            self.device
                .core
                .instance
                .instance
                .get_physical_device_image_format_properties(
                    self.device.core.physical,
                    format,
                    vk::ImageType::TYPE_2D,
                    vk::ImageTiling::OPTIMAL,
                    self.usage,
                    vk::ImageCreateFlags::empty(),
                )
        }) {
            Ok(_) => Ok(true),
            Err(Error::Vulkan(vk::Result::ERROR_FORMAT_NOT_SUPPORTED)) => Ok(false),
            Err(error) => Err(error),
        }
    }
    fn recreate(
        &mut self,
        requested: vk::Extent2D,
        old: vk::SwapchainKHR,
        old_retired: &mut bool,
    ) -> Result<()> {
        if requested.width == 0 || requested.height == 0 {
            self.mark_stale();
            return Err(Error::invalid_usage(ZERO_WINDOW_EXTENT));
        }
        let (caps, formats, modes) = self.fullscreen_surface_configuration()?;
        if !caps.supported_usage_flags.contains(self.usage) {
            return Err(Error::unsupported("表面不支持请求的图片用途"));
        }
        // 首选 B8G8R8A8_SRGB；回退时只取支持请求用途（OPTIMAL 2D 图像）的格式，不盲取第一个。
        let preferred = formats.iter().find(|format| {
            format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        });
        let mut selected = None;
        for candidate in preferred.into_iter().chain(formats.iter()) {
            if candidate.format == vk::Format::UNDEFINED
                || self.format_supports_usage(candidate.format)?
            {
                selected = Some(*candidate);
                break;
            }
        }
        let mut format = selected.ok_or(Error::unsupported("表面没有支持请求用途的格式"))?;
        if format.format == vk::Format::UNDEFINED {
            format.format = vk::Format::B8G8R8A8_SRGB;
        }
        let present_mode = if !self.vsync && modes.contains(&vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else if !self.vsync && modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
            vk::PresentModeKHR::IMMEDIATE
        } else {
            vk::PresentModeKHR::FIFO
        };
        let extent = if caps.current_extent.width != u32::MAX {
            caps.current_extent
        } else {
            vk::Extent2D {
                width: requested
                    .width
                    .clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                height: requested
                    .height
                    .clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            }
        };
        if extent.width == 0 || extent.height == 0 {
            return Err(Error::invalid_usage(ZERO_SURFACE_EXTENT));
        }
        let count = caps.min_image_count.saturating_add(1);
        let count = if caps.max_image_count == 0 {
            count
        } else {
            count.min(caps.max_image_count)
        };
        let composite = [
            vk::CompositeAlphaFlagsKHR::OPAQUE,
            vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
            vk::CompositeAlphaFlagsKHR::INHERIT,
        ]
        .into_iter()
        .find(|flag| caps.supported_composite_alpha.contains(*flag))
        .ok_or(Error::unsupported("没有合成透明度模式"))?;
        let mut info = vk::SwapchainCreateInfoKHR::default()
            .surface(self.surface.handle)
            .min_image_count(count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(self.usage)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(caps.current_transform)
            .composite_alpha(composite)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(old);
        let mut exclusive_info = vk::SurfaceFullScreenExclusiveInfoEXT::default()
            .full_screen_exclusive(if self.inner.get_mut().fullscreen.monitor.is_some() {
                vk::FullScreenExclusiveEXT::APPLICATION_CONTROLLED
            } else {
                vk::FullScreenExclusiveEXT::DISALLOWED
            });
        let mut monitor_info = vk::SurfaceFullScreenExclusiveWin32InfoEXT::default()
            .hmonitor(self.inner.get_mut().fullscreen.monitor.unwrap_or(0));
        if self.fullscreen_extension_enabled() {
            info = info.push_next(&mut exclusive_info);
            if self.inner.get_mut().fullscreen.monitor.is_some() {
                info = info.push_next(&mut monitor_info);
            }
        }
        // 成功建立新交换链后旧对象可能已被驱动退役；任何后续失败都不得继续获取旧图像。
        self.inner.get_mut().stale = true;
        let handle = match wsi_call(&self.device.queues.timelines, || unsafe {
            *old_retired |= old != vk::SwapchainKHR::null();
            self.loader.create_swapchain(&info, None)
        }) {
            Ok(handle) => handle,
            Err(Error::Vulkan(vk::Result::ERROR_INITIALIZATION_FAILED))
                if self.inner.get_mut().fullscreen.monitor.is_some() =>
            {
                self.inner.get_mut().fullscreen.monitor = None;
                self.inner.get_mut().fullscreen.status.fallback =
                    Some(FullscreenFallback::CreationFailed);
                // The first attempt retired oldSwapchain even though it failed.
                return self.recreate(requested, vk::SwapchainKHR::null(), old_retired);
            }
            Err(error) => return Err(error),
        };
        let images = match wsi_call(&self.device.queues.timelines, || unsafe {
            self.loader.get_swapchain_images(handle)
        }) {
            Ok(images) => images,
            Err(error) => {
                unsafe {
                    self.loader.destroy_swapchain(handle, None);
                }
                return Err(error);
            }
        };
        let mut views = Vec::new();
        for image in &images {
            let range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            let info = vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format.format)
                .subresource_range(range);
            match wsi_call(&self.device.queues.timelines, || unsafe {
                self.device.core.device.create_image_view(&info, None)
            }) {
                Ok(view) => views.push(view),
                Err(error) => {
                    unsafe {
                        for view in views {
                            self.device.core.device.destroy_image_view(view, None);
                        }
                        self.loader.destroy_swapchain(handle, None);
                    }
                    self.inner.get_mut().stale = true;
                    return Err(error);
                }
            }
        }
        unsafe {
            for view in self.views.drain(..) {
                self.device.core.device.destroy_image_view(view, None);
            }
            if self.handle != vk::SwapchainKHR::null() {
                self.loader.destroy_swapchain(self.handle, None);
            }
        }
        self.handle = handle;
        self.images = images;
        self.views = views;
        self.format = format.format;
        self.extent = extent;
        self.present_mode = present_mode;
        self.pre_transform = caps.current_transform;
        let mut slots = Vec::with_capacity(self.images.len());
        for _ in &self.images {
            slots.push(Arc::new(FrameSemaphores::new(self.device.clone())?));
        }
        let mut presentations = Vec::with_capacity(self.images.len());
        for _ in &self.images {
            presentations.push(Arc::new(ImagePresentation::new(self.device.clone())?));
        }
        let inner = self.inner.get_mut();
        inner.presentations = presentations;
        inner.leases = slots;
        inner.next_slot = 0;
        inner.state = SwapchainState::Created;
        inner.stale = false;
        Ok(())
    }
    /// Request a new extent; the next acquisition rebuilds the generation.
    pub fn resize(&self, extent: vk::Extent2D) -> Result<()> {
        let mut inner = self.inner.write();
        if inner.closing {
            return Err(Error::invalid_usage("交换链正在关闭"));
        }
        if inner.desc.extent != extent {
            inner.desc.extent = extent;
            inner.stale = true;
        }
        Ok(())
    }
    /// Request a new vsync mode; the next acquisition rebuilds the generation.
    pub fn set_vsync(&self, vsync: bool) -> Result<()> {
        let mut inner = self.inner.write();
        if inner.desc.vsync != vsync {
            inner.desc.vsync = vsync;
            inner.stale = true;
        }
        Ok(())
    }
    /// The requested (not necessarily created) description.
    pub fn desc(&self) -> SwapchainDesc {
        self.inner.read().desc
    }
    /// WSI call that also latches SURFACE_LOST on the surface.
    fn surface_call<T, E: Into<Error>>(
        &self,
        call: impl FnOnce() -> std::result::Result<T, E>,
    ) -> Result<T> {
        let result = wsi_call(&self.device.queues.timelines, call);
        self.surface.observe(&result);
        result
    }
    /// Prepare a replacement generation for the same or a new surface.
    ///
    /// Outstanding image leases keep the old generation alive; a replacement
    /// waits until every old acquisition returned through present, release or
    /// drop, so no old present can run after the newer generation's
    /// retirement proof. The WSI lock is held for creation because
    /// `oldSwapchain` is externally synchronised; the state lock is not.
    fn replacement_for(&self, surface: &Arc<Surface>) -> Result<Self> {
        if surface.is_lost() {
            return Err(Error::SurfaceLost);
        }
        let same_surface = Arc::ptr_eq(surface, &self.surface);
        // Every retry polls existing acquire/present/cleanup proofs first. If
        // none has progressed, retain the current generation and apply bounded
        // backpressure; queue-idle is not a substitute for a present proof.
        self.collect_presentations()?;
        check_retired_generation_budget(self.retired_generation_count())?;
        let _wsi = self.wsi.lock();
        let (desc, fullscreen, old) = {
            let mut inner = self.inner.write();
            if inner.closing {
                return Err(Error::invalid_usage("交换链正在关闭"));
            }
            if !inner.can_replace() {
                return Err(vk::Result::NOT_READY.into());
            }
            inner.stale = true;
            self.release_fullscreen(&mut inner)?;
            let old = if inner.fullscreen.retired || !same_surface {
                vk::SwapchainKHR::null()
            } else {
                self.handle
            };
            (inner.desc, inner.fullscreen, old)
        };
        let mut retired = fullscreen.retired;
        let result = unsafe {
            Self::new_with_old(
                self.device.clone(),
                surface.clone(),
                desc,
                old,
                self.policy,
                fullscreen,
                &mut retired,
            )
        };
        if same_surface {
            self.inner.write().fullscreen.retired = retired;
        }
        result
    }
    fn publish_replacement(chain: &mut Arc<Self>, replacement: Self) {
        // Capacity for this push was reserved before creation.
        replacement.retired_generations.lock().push(chain.clone());
        *chain = Arc::new(replacement);
    }
    /// Replace a lost (or any) surface with a new one supplied by the host.
    /// The previous generation is retired with its completion proofs.
    pub fn replace_surface(chain: &mut Arc<Self>, surface: Arc<Surface>) -> Result<()> {
        if !Arc::ptr_eq(&surface.instance, &chain.device.core.instance) {
            return Err(Error::invalid_usage("交换链表面属于其他实例"));
        }
        let replacement = chain.replacement_for(&surface)?;
        Self::publish_replacement(chain, replacement);
        Ok(())
    }
    /// Acquire the next image, rebuilding a stale or out-of-date generation.
    ///
    /// `Ok(None)` means no image this frame (minimised window, unfocused
    /// exclusive fullscreen, acquire timeout/NOT_READY, outstanding old
    /// acquisitions or retired-generation backpressure). A lost surface
    /// returns [`Error::SurfaceLost`] without any further native call until
    /// [`Self::replace_surface`] supplies a new surface.
    pub fn validate_swapchain_and_acquire_image(
        chain: &mut Arc<Self>,
    ) -> Result<Option<AcquiredImageLease>> {
        let reason = match Self::acquire_classified(chain) {
            Acquired::Frame(frame) => return Ok(Some(frame)),
            Acquired::NoImage => return Ok(None),
            Acquired::Failed(error) => return Err(error),
            Acquired::Stale => "stale",
            Acquired::Retired => "out of date",
        };
        log::debug!(target: "rhapsody::raw::swapchain", "Re-creating a {reason} swapchain.");
        let surface = chain.surface.clone();
        let replacement = match chain.replacement_for(&surface) {
            Ok(replacement) => replacement,
            Err(Error::Vulkan(vk::Result::NOT_READY) | Error::Exhausted(_)) => return Ok(None),
            Err(error) if is_zero_extent(&error) => {
                log::debug!(target: "rhapsody::raw::swapchain", "Skipping frame: {error} (after {reason})");
                return Ok(None);
            }
            Err(error) => {
                log::error!(target: "rhapsody::raw::swapchain", "Vulkan swapchain re-creation error: {error:?} (after {reason})");
                return Err(error);
            }
        };
        Self::publish_replacement(chain, replacement);
        match Self::acquire_classified(chain) {
            Acquired::Frame(frame) => Ok(Some(frame)),
            Acquired::NoImage | Acquired::Retired => Ok(None),
            Acquired::Stale => {
                log::warn!(target: "rhapsody::raw::swapchain", "The newly-created swapchain was reported stale ({reason}).");
                Ok(None)
            }
            Acquired::Failed(error) => Err(error),
        }
    }
    /// One acquisition attempt and its classification.
    fn acquire_classified(chain: &Arc<Self>) -> Acquired {
        if chain.surface.is_lost() {
            return Acquired::Failed(Error::SurfaceLost);
        }
        if let Err(error) = chain.maintain_fullscreen() {
            return Acquired::Failed(error);
        }
        {
            let inner = chain.inner.read();
            if inner.fullscreen.paused()
                || inner.desc.extent.width == 0
                || inner.desc.extent.height == 0
            {
                return Acquired::NoImage;
            }
            if inner.stale {
                return Acquired::Stale;
            }
        }
        match chain.acquire_frame_unmaintained(ACQUIRE_TIMEOUT_NS) {
            Ok(frame) => Acquired::Frame(frame),
            Err(Error::Timeout | Error::Vulkan(vk::Result::NOT_READY)) => Acquired::NoImage,
            Err(Error::SurfaceOutOfDate) => Acquired::Retired,
            Err(error) => Acquired::Failed(error),
        }
    }
    pub fn format(&self) -> vk::Format {
        self.format
    }
    pub fn mark_stale(&self) {
        self.inner.write().stale = true;
    }
    pub fn is_stale(&self) -> bool {
        self.inner.read().stale
    }
    /// A SUBOPTIMAL acquire or present only retires this generation when the
    /// surface's current extent or transform differs from what it was created
    /// with; otherwise a persistent SUBOPTIMAL would rebuild every frame.
    fn note_suboptimal(&self) {
        let caps = self.surface_call(|| unsafe {
            self.surface
                .loader
                .get_physical_device_surface_capabilities(
                    self.device.core.physical,
                    self.surface.handle,
                )
        });
        let stale = match caps {
            Ok(caps) => suboptimal_requires_recreate(&caps, self.extent, self.pre_transform),
            Err(error) => {
                log::debug!(target: "rhapsody::raw::swapchain", "surface query after SUBOPTIMAL failed: {error}");
                true
            }
        };
        if stale {
            self.mark_stale();
        }
    }
    /// Acquire using caller-owned synchronization; `timeout` (ns) is passed
    /// to vkAcquireNextImageKHR unchanged. Prefer [`Self::acquire_frame`] for
    /// owned synchronization and automatic cancellation.
    ///
    /// # Safety
    /// Semaphore/fence must belong to this device, be externally synchronized,
    /// and satisfy vkAcquireNextImageKHR's unsignaled/not-in-use requirements.
    /// Keep the nonnull fence and optional semaphore alive until acquisition is
    /// complete, including if the lease's bounded Drop wait times out. A caller
    /// that submits image work must also retain this generation until it ends.
    pub unsafe fn acquire_next_image(
        self: &Arc<Self>,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> Result<SwapchainImageLease> {
        // The raw lease must own a host-observable acquisition proof even when
        // its caller never submits a semaphore wait. Keep this fence valid until
        // the lease is consumed or dropped; Drop waits before returning it.
        if fence == vk::Fence::null() {
            return Err(Error::invalid_usage("底层获取租约需要有效的获取完成栅栏"));
        }
        self.maintain_fullscreen()?;
        let _wsi = self.wsi.lock();
        let (index, suboptimal) = self.acquire_native(timeout, semaphore, fence)?;
        Ok(SwapchainImageLease {
            swapchain: Arc::clone(self),
            index,
            suboptimal,
            released: false,
            external_acquire_fence: fence,
        })
    }
    /// Native acquire with the WSI lock held by the caller. The state lock is
    /// taken only to validate and to publish, never across the native call.
    /// The device queue lock is not taken: acquire is not a queue operation.
    fn acquire_native(
        &self,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> Result<(u32, bool)> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        self.inner.read().check_acquire()?;
        let mut exclusive_lost = false;
        let native = self.surface_call(|| {
            let native = unsafe {
                self.loader
                    .acquire_next_image(self.handle, timeout, semaphore, fence)
            };
            exclusive_lost = native == Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT);
            native
        });
        let mut inner = self.inner.write();
        if exclusive_lost {
            inner.fullscreen.lost();
        }
        inner.commit_acquire(native)
    }
    /// Acquire into a library-owned frame slot. The slot cursor advances only
    /// after a successful acquire; per-slot semaphores and fences are created
    /// with the generation.
    pub fn acquire_frame(self: &Arc<Self>, timeout: u64) -> Result<AcquiredImageLease> {
        self.maintain_fullscreen()?;
        self.acquire_frame_unmaintained(timeout)
    }
    fn acquire_frame_unmaintained(self: &Arc<Self>, timeout: u64) -> Result<AcquiredImageLease> {
        let _wsi = self.wsi.lock();
        let mut replaced = None;
        let (slot_index, sync) = {
            let mut inner = self.inner.write();
            inner.check_acquire()?;
            let slot_index = inner.next_slot;
            let abandoned = inner
                .leases
                .get(slot_index)
                .ok_or(Error::invalid_usage("交换链没有图像槽"))?
                .state
                .lock()
                .abandoned;
            if abandoned {
                let fresh = Arc::new(FrameSemaphores::new(self.device.clone())?);
                replaced = Some(std::mem::replace(&mut inner.leases[slot_index], fresh));
            }
            (slot_index, inner.leases[slot_index].clone())
        };
        // An abandoned slot's teardown may wait for its acquisition fence.
        drop(replaced);
        sync.reserve(timeout)?;
        let (index, suboptimal) =
            match self.acquire_native(timeout, sync.acquired, sync.acquire_ready) {
                Ok(acquired) => acquired,
                Err(error) => {
                    sync.release();
                    return Err(error);
                }
            };
        // Internal leases retain acquisition completion through the frame slot.
        let lease = SwapchainImageLease {
            swapchain: Arc::clone(self),
            index,
            suboptimal,
            released: false,
            external_acquire_fence: vk::Fence::null(),
        };
        let mut inner = self.inner.write();
        let Some(presentation) = inner.presentations.get(index as usize).cloned() else {
            drop(inner);
            {
                let mut state = sync.state.lock();
                state.acquired = true;
                state.abandoned = true;
            }
            sync.release();
            drop(lease);
            return Err(Error::invalid_usage("驱动返回的交换链图片索引越界"));
        };
        {
            let mut state = sync.state.lock();
            state.acquired = true;
            state.previous_present =
                Some((presentation.clone(), presentation.progress.lock().issued));
        }
        inner.next_slot = (slot_index + 1) % inner.leases.len();
        drop(inner);
        Ok(AcquiredImageLease {
            swapchain: Arc::clone(self),
            index,
            lease: Some(lease),
            presentation,
            sync,
            slot_index,
            recover_on_drop: true,
        })
    }
    fn release_image(&self, index: u32) -> Result<()> {
        let indices = [index];
        let info = vk::ReleaseSwapchainImagesInfoEXT::default()
            .swapchain(self.handle)
            .image_indices(&indices);
        let loader = ash::ext::swapchain_maintenance1::Device::new(
            &self.device.core.instance.instance,
            &self.device.core.device,
        );
        let _wsi = self.wsi.lock();
        self.surface_call(|| unsafe { loader.release_swapchain_images(&info) })
    }
    /// Present a caller-owned acquisition and wait semaphores.
    ///
    /// # Safety
    /// The index must be acquired from this generation and transitioned to
    /// PRESENT_SRC_KHR. Waits must be same-device binary semaphores with a single
    /// submitted signal, externally synchronized and kept alive until a real
    /// presentation completion proof (not merely rendering or queue-idle).
    pub unsafe fn present(&self, index: u32, wait: &[vk::Semaphore]) -> Result<bool> {
        let suboptimal = unsafe { self.present_unnoted(index, wait) }?;
        if suboptimal {
            self.note_suboptimal();
        }
        Ok(suboptimal)
    }
    unsafe fn present_unnoted(&self, index: u32, wait: &[vk::Semaphore]) -> Result<bool> {
        self.policy.allow_present()?;
        if index as usize >= self.images.len() {
            return Err(Error::invalid_usage("交换链图片索引越界"));
        }
        let _wsi = self.wsi.lock();
        let presentation = {
            let inner = self.inner.read();
            if inner.closing {
                return Err(Error::invalid_usage("交换链正在关闭"));
            }
            inner.presentations[index as usize].clone()
        };
        // Bounded fence wait for this image's previous present; no state lock.
        presentation.prepare(ACQUIRE_TIMEOUT_NS)?;
        let has_fence = presentation.fence != vk::Fence::null();
        let chains = [self.handle];
        let indices = [index];
        let fences = [presentation.fence];
        let mut fence_info = vk::SwapchainPresentFenceInfoEXT::default().fences(&fences);
        let mut info = vk::PresentInfoKHR::default()
            .wait_semaphores(wait)
            .swapchains(&chains)
            .image_indices(&indices);
        if has_fence {
            info = info.push_next(&mut fence_info);
        }
        let native = self
            .device
            .queues
            .timelines
            .with_queue(self.device.queue_id(self.device.queues.primary)?, || {
                Ok(unsafe { self.loader.queue_present(self.device.queues.primary, &info) })
            })?;
        let result =
            self.inner
                .write()
                .finish_present(&mut presentation.progress.lock(), has_fence, native);
        if result.as_ref().is_err_and(Error::is_device_lost) {
            self.device.queues.timelines.mark_device_lost();
        }
        self.surface.observe(&result);
        result
    }
}
/// Whether a SUBOPTIMAL generation must be rebuilt: only when the surface's
/// fixed current extent or its transform differs from the created chain.
fn suboptimal_requires_recreate(
    caps: &vk::SurfaceCapabilitiesKHR,
    extent: vk::Extent2D,
    transform: vk::SurfaceTransformFlagsKHR,
) -> bool {
    let fixed_extent = caps.current_extent.width != u32::MAX;
    (fixed_extent && caps.current_extent != extent) || caps.current_transform != transform
}
impl Drop for Swapchain {
    fn drop(&mut self) {
        if let Err(error) = self.shutdown(ACQUIRE_TIMEOUT_NS)
            && !error.is_device_lost()
        {
            // Explicit compatibility mode cannot manufacture the missing final
            // presentation proof. Keep *all* owners alive, including the device
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
pub struct SwapchainImageLease {
    pub swapchain: Arc<Swapchain>,
    pub index: u32,
    pub suboptimal: bool,
    released: bool,
    external_acquire_fence: vk::Fence,
}
impl SwapchainImageLease {
    pub fn image(&self) -> vk::Image {
        self.swapchain.images[self.index as usize]
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.swapchain.views[self.index as usize]
    }
    /// # Safety
    /// The image layout, rendering completion and semaphore ownership must
    /// satisfy [`Swapchain::present`]'s contract.
    pub unsafe fn present(mut self, wait: &[vk::Semaphore]) -> Result<bool> {
        let result = unsafe { self.swapchain.present_unnoted(self.index, wait) };
        // Swapchain::present retires the generation on every failed operation;
        // its caller handles a rejected wait independently from this image lease.
        self.released = true;
        if result
            .as_ref()
            .is_ok_and(|&present| present || self.suboptimal)
        {
            self.swapchain.note_suboptimal();
        }
        result.map(|present| present || self.suboptimal)
    }
}
impl Drop for SwapchainImageLease {
    fn drop(&mut self) {
        let unproven = self.external_acquire_fence != vk::Fence::null()
            && DeviceFenceOperations(&self.swapchain.device)
                .wait(self.external_acquire_fence, ACQUIRE_TIMEOUT_NS)
                .is_err_and(|error| !error.is_device_lost());
        let mut inner = self.swapchain.inner.write();
        inner.unproven_external_acquire |= unproven;
        inner.return_acquisition(self.released);
    }
}

/// One semaphore per *image*, independent of the rotating acquire slots.
/// Reacquisition proves the previous present wait was consumed only after its
/// acquire fence signals. The generation itself owns ambiguous present failures.
struct ImagePresentation {
    device: Arc<Device>,
    rendered: vk::Semaphore,
    fence: vk::Fence,
    progress: Mutex<PresentProgress>,
}
/// A present fence that the image's next present or teardown must consume.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingFence {
    serial: u64,
    /// False for presents the presentation engine rejected: their fence may
    /// never signal, so it is polled but never waited on.
    certain: bool,
}
#[derive(Default)]
struct PresentProgress {
    issued: u64,
    completed: u64,
    pending_fence: Option<PendingFence>,
    // Render and rejected-present cleanup submits can both access rendered.
    // Fixed-capacity registration is infallible immediately after submit.
    gpu_use: crate::sync::SyncSet,
}
impl PresentProgress {
    fn generation_completed(&mut self) {
        self.completed = self.issued;
    }
    /// Final teardown. A rejected present only enqueued its semaphore wait
    /// (no image reached the presentation engine); once the device timeline
    /// passed its render submission (`finish_gpu_use`, done by the caller)
    /// that wait has been satisfied, so an unsignaled uncertain fence does
    /// not stall teardown.
    fn finish_shutdown_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if let Some(pending) = self.pending_fence
            && !pending.certain
        {
            if !operations.signaled(fence)? {
                log::debug!(target: "rhapsody::raw::swapchain", "rejected present fence unsignaled at teardown; using the render timeline proof");
            }
            self.acquired(pending.serial);
            self.pending_fence = None;
        }
        self.finish_with(operations, fence, timeout)?;
        if self.all_presentations_complete() {
            Ok(())
        } else {
            Err(Error::unsupported("最终呈现尚无完成证明；保留交换链所有权"))
        }
    }
    /// Consume a pending certain fence with a bounded wait. An uncertain one
    /// completes only if already signaled; otherwise this returns
    /// SurfaceOutOfDate instead of waiting for a signal that may never come.
    fn finish_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if let Some(pending) = self.pending_fence {
            if pending.certain {
                operations.wait(fence, timeout)?;
            } else if !operations.signaled(fence)? {
                return Err(Error::SurfaceOutOfDate);
            }
            self.acquired(pending.serial);
            self.pending_fence = None;
        }
        Ok(())
    }
    fn prepare_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        fence: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        self.finish_with(operations, fence, timeout)?;
        if fence != vk::Fence::null() {
            operations.reset(fence)?;
        }
        Ok(())
    }
    fn poll_with(&mut self, operations: &impl WsiFenceOperations, fence: vk::Fence) -> Result<()> {
        if self.pending_fence.is_some() && operations.signaled(fence)? {
            self.finish_with(operations, fence, 0)?;
        }
        Ok(())
    }
    fn all_presentations_complete(&self) -> bool {
        self.completed == self.issued
    }
    fn record_gpu_submission(&mut self, point: crate::sync::SyncPoint) {
        self.gpu_use.insert(point);
    }
    fn retirement_ready(&self, completed: &crate::sync::CompletedValues) -> bool {
        self.all_presentations_complete() && self.gpu_use.is_reached(completed)
    }
    fn record_result(&mut self, result: &std::result::Result<bool, vk::Result>, has_fence: bool) {
        if present_was_enqueued(result) {
            let serial = self.issue();
            if has_fence {
                self.pending_fence = Some(PendingFence {
                    serial,
                    certain: present_fence_is_certain(result),
                });
            }
        }
    }
    fn issue(&mut self) -> u64 {
        self.issued += 1;
        self.issued
    }
    fn acquired(&mut self, serial: u64) {
        self.completed = self.completed.max(serial);
    }
    fn completed(&self, serial: u64) -> bool {
        self.completed >= serial
    }
}
impl ImagePresentation {
    fn finish_shutdown(&self, timeout: u64) -> Result<()> {
        self.progress.lock().finish_shutdown_with(
            &DeviceFenceOperations(&self.device),
            self.fence,
            timeout,
        )
    }
    fn finish_gpu_use(&self, timeout: u64) -> Result<()> {
        let after = self.progress.lock().gpu_use;
        self.device.queues.timelines.wait(&after, timeout)
    }
    fn poll(&self) -> Result<()> {
        self.progress
            .lock()
            .poll_with(&DeviceFenceOperations(&self.device), self.fence)
    }
    fn prepare(&self, timeout: u64) -> Result<()> {
        self.progress
            .lock()
            .prepare_with(&DeviceFenceOperations(&self.device), self.fence, timeout)
    }
    fn new(device: Arc<Device>) -> Result<Self> {
        let rendered = wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        })?;
        let fence = if device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            match wsi_call(&device.queues.timelines, || unsafe {
                device
                    .core
                    .device
                    .create_fence(&vk::FenceCreateInfo::default(), None)
            }) {
                Ok(fence) => fence,
                Err(error) => {
                    unsafe {
                        device.core.device.destroy_semaphore(rendered, None);
                    }
                    return Err(error);
                }
            }
        } else {
            vk::Fence::null()
        };
        Ok(Self {
            device,
            rendered,
            fence,
            progress: Mutex::new(PresentProgress::default()),
        })
    }
}
impl Drop for ImagePresentation {
    fn drop(&mut self) {
        // A timed-out cleanup wait still owns a real successful submit point.
        // Protect rendered independently from presentation completion proofs.
        let completed = self
            .finish_gpu_use(ACQUIRE_TIMEOUT_NS)
            .and_then(|()| self.finish_shutdown(ACQUIRE_TIMEOUT_NS));
        if let Err(error) = completed
            && !error.is_device_lost()
        {
            log::error!(target: "rhapsody::raw::swapchain", "present resources retained: {error}");
            std::mem::forget(self.device.clone());
            return;
        }
        unsafe {
            self.device
                .core
                .device
                .destroy_semaphore(self.rendered, None);
            if self.fence != vk::Fence::null() {
                self.device.core.device.destroy_fence(self.fence, None);
            }
        }
    }
}
struct FrameSemaphores {
    device: Arc<Device>,
    acquired: vk::Semaphore,
    acquire_ready: vk::Fence,
    fence: vk::Fence,
    state: Mutex<FrameSlotState>,
}
#[derive(Default)]
struct FrameSlotState {
    reserved: bool,
    submitted: bool,
    acquired: bool,
    abandoned: bool,
    previous_present: Option<(Arc<ImagePresentation>, u64)>,
}
impl FrameSlotState {
    fn finish_acquire_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        acquire_ready: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if self.acquired {
            operations.wait(acquire_ready, timeout)?;
            if let Some((image, serial)) = self.previous_present.take() {
                image.progress.lock().acquired(serial);
            }
            self.acquired = false;
        }
        Ok(())
    }
    fn reserve_with(
        &mut self,
        operations: &impl WsiFenceOperations,
        acquire_ready: vk::Fence,
        submitted: vk::Fence,
        timeout: u64,
    ) -> Result<()> {
        if self.reserved {
            return Err(vk::Result::NOT_READY.into());
        }
        if self.submitted {
            operations.wait(submitted, timeout)?;
            self.submitted = false;
        }
        self.finish_acquire_with(operations, acquire_ready, timeout)?;
        operations.reset(acquire_ready)?;
        self.reserved = true;
        Ok(())
    }
}
impl FrameSemaphores {
    fn new(device: Arc<Device>) -> Result<Self> {
        let acquired = wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        })?;
        let acquire_ready = match wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_semaphore(acquired, None);
                }
                return Err(error);
            }
        };
        let fence = match wsi_call(&device.queues.timelines, || unsafe {
            device
                .core
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        }) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_fence(acquire_ready, None);
                    device.core.device.destroy_semaphore(acquired, None);
                }
                return Err(error);
            }
        };
        Ok(Self {
            device,
            acquired,
            acquire_ready,
            fence,
            state: Mutex::new(FrameSlotState::default()),
        })
    }
    fn finish_acquire(&self, state: &mut FrameSlotState, timeout: u64) -> Result<()> {
        state.finish_acquire_with(
            &DeviceFenceOperations(&self.device),
            self.acquire_ready,
            timeout,
        )
    }
    fn reserve(&self, timeout: u64) -> Result<()> {
        self.state.lock().reserve_with(
            &DeviceFenceOperations(&self.device),
            self.acquire_ready,
            self.fence,
            timeout,
        )
    }
    fn release(&self) {
        self.state.lock().reserved = false;
    }
}
impl Drop for FrameSemaphores {
    fn drop(&mut self) {
        // Abandoned acquire slots can also be replaced within a live generation.
        // Their acquisition fence proves that the unused binary signal operation
        // has finished; it is separate from queue timeline completion.
        let mut state = self.state.lock();
        let result = self
            .finish_acquire(&mut state, ACQUIRE_TIMEOUT_NS)
            .and_then(|()| {
                if !state.submitted {
                    return Ok(());
                }
                wsi_call(&self.device.queues.timelines, || unsafe {
                    self.device
                        .core
                        .device
                        .wait_for_fences(&[self.fence], true, ACQUIRE_TIMEOUT_NS)
                })
            });
        if let Err(error) = result
            && !error.is_device_lost()
        {
            log::error!(target: "rhapsody::raw::swapchain", "acquire resources retained: {error}");
            std::mem::forget(self.device.clone());
            std::mem::forget(state.previous_present.take());
            return;
        }
        unsafe {
            self.device.core.device.destroy_fence(self.fence, None);
            self.device
                .core
                .device
                .destroy_fence(self.acquire_ready, None);
            self.device
                .core
                .device
                .destroy_semaphore(self.acquired, None);
        }
    }
}

fn color_barrier(image: vk::Image) -> vk::ImageMemoryBarrier<'static> {
    vk::ImageMemoryBarrier::default()
        .image(image)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1),
        )
}

/// An acquired image accepts exactly one display submission. Dropping it
/// unconsumed cancels the acquisition and retires the generation.
pub struct AcquiredImageLease {
    swapchain: Arc<Swapchain>,
    index: u32,
    lease: Option<SwapchainImageLease>,
    sync: Arc<FrameSemaphores>,
    slot_index: usize,
    presentation: Arc<ImagePresentation>,
    recover_on_drop: bool,
}
fn consumed() -> Error {
    Error::invalid_usage("交换链图像租约已被消费")
}
impl AcquiredImageLease {
    pub(crate) fn abandon_device_lost(mut self) {
        // Drop only releases CPU bookkeeping. No fence waits or rescue submits
        // are valid once the caller has observed device loss.
        self.recover_on_drop = false;
    }
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
    }
    pub(crate) fn device(&self) -> &Arc<Device> {
        &self.swapchain.device
    }
    /// Whether this image was handed to the presentation engine before in
    /// this generation, i.e. it is in PRESENT_SRC_KHR with defined contents.
    pub(crate) fn was_presented(&self) -> bool {
        self.presentation.progress.lock().issued > 0
    }
    pub fn image(&self) -> vk::Image {
        self.swapchain.images[self.index as usize]
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.swapchain.views[self.index as usize]
    }
    pub fn format(&self) -> vk::Format {
        self.swapchain.format
    }
    pub fn index(&self) -> u32 {
        self.index
    }
    pub fn slot_index(&self) -> usize {
        self.slot_index
    }
    pub fn acquire_semaphore(&self) -> vk::Semaphore {
        self.sync.acquired
    }
    pub fn render_semaphore(&self) -> vk::Semaphore {
        self.presentation.rendered
    }
    pub fn submission_fence(&self) -> vk::Fence {
        self.sync.fence
    }
    pub fn extent(&self) -> vk::Extent2D {
        self.swapchain.extent
    }
    /// A graph can borrow this actual acquired image while its generation is
    /// retained. Acquisition and present remain owned by the linear WSI lease.
    pub(crate) fn texture(&self) -> Result<Arc<super::texture::Texture>> {
        let chain = &self.swapchain;
        let desc = super::texture::TextureDesc::new_2d(
            chain.extent.width,
            chain.extent.height,
            chain.format,
        )
        .usage(chain.usage);
        Ok(Arc::new(super::texture::Texture::from_swapchain_image(
            chain.clone(),
            self.image(),
            desc,
        )?))
    }
    /// Cancel before any submit has consumed the acquire semaphore. With
    /// maintenance1, wait for acquisition and release the image; the signaled
    /// acquire semaphore is discarded independently. Otherwise submit a real
    /// UNDEFINED -> PRESENT transition, consuming acquire exactly once, and
    /// return the image through presentation.
    pub fn cancel(mut self, timeout: u64) -> Result<()> {
        self.recover_on_drop = false;
        self.cancel_unsubmitted(timeout)
    }
    fn cancel_unsubmitted(&mut self, timeout: u64) -> Result<()> {
        let chain = self.swapchain.clone();
        let index = self.lease.as_ref().ok_or_else(consumed)?.index;
        if chain.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if chain.policy == SwapchainPolicy::AcquireOnly {
            self.sync
                .finish_acquire(&mut self.sync.state.lock(), timeout)?;
            // An acquired image may be destroyed with its swapchain after its
            // acquisition completes. It cannot be acquired again without a
            // presentation or release, so retire this acquire-only generation.
            chain.mark_stale();
            self.sync.state.lock().abandoned = true;
            self.sync.release();
            self.lease.take();
            return Ok(());
        }
        if chain
            .device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            self.sync
                .finish_acquire(&mut self.sync.state.lock(), timeout)?;
            chain.release_image(index)?;
            {
                let mut state = self.sync.state.lock();
                // release-images does not consume the acquisition semaphore.
                state.abandoned = true;
                state.reserved = false;
            }
            if let Some(mut lease) = self.lease.take() {
                lease.released = true;
            }
            return Ok(());
        }
        let command = self.discard_to_present_command(vk::PipelineStageFlags::TOP_OF_PIPE)?;
        let mut recovery = Self {
            swapchain: chain.clone(),
            index,
            lease: self.lease.take(),
            sync: self.sync.clone(),
            slot_index: self.slot_index,
            presentation: self.presentation.clone(),
            // If recovery itself fails, retire this generation instead of
            // recursively attempting another recovery from Drop.
            recover_on_drop: false,
        };
        // The barrier's source scope is TOP_OF_PIPE, the acquire wait stage.
        drop(recovery.submit_with_stages(
            command,
            Some(vk::PipelineStageFlags::TOP_OF_PIPE),
            vk::PipelineStageFlags::TOP_OF_PIPE,
            crate::sync::SyncSet::EMPTY,
        )?);
        Ok(())
    }
    /// One command holding only `UNDEFINED -> PRESENT_SRC_KHR` on the
    /// presentation queue family. `source` must cover the stage at which the
    /// submit waits, so the layout transition is chained after that wait.
    fn discard_to_present_command(
        &self,
        source: vk::PipelineStageFlags,
    ) -> Result<VulkanCommandBuffer> {
        let chain = &self.swapchain;
        let pool = wsi_call(&chain.device.queues.timelines, || {
            VulkanCommandPool::new(chain.device.clone())
        })?;
        let mut command = wsi_call(&chain.device.queues.timelines, || pool.allocate())?;
        wsi_call(&chain.device.queues.timelines, || command.begin())?;
        let mut scope = super::barrier::SynchronizationScope::new(
            source,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
        );
        scope.images.push(
            color_barrier(self.image())
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR),
        );
        scope.issue_into(&command)?;
        wsi_call(&chain.device.queues.timelines, || command.end())?;
        Ok(command)
    }
    /// Return an image whose acquire semaphore was already consumed by a
    /// partially submitted presentation graph that never signalled
    /// `rendered`. After every submitted graph use (`after`), discard the
    /// contents, transition to PRESENT_SRC_KHR, signal `rendered` exactly once
    /// and present. The acquire semaphore is never waited a second time.
    fn present_after_partial_graph(
        &mut self,
        after: crate::sync::SyncSet,
    ) -> Result<PresentTicket> {
        if self.swapchain.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let command = self.discard_to_present_command(vk::PipelineStageFlags::ALL_COMMANDS)?;
        self.submit_with_stages(command, None, vk::PipelineStageFlags::ALL_COMMANDS, after)
    }
    /// Submit `command` on the presentation queue and present it. With
    /// `acquire_stage`, the submit consumes the acquire semaphore at that stage
    /// and arms the slot fence that guards the semaphore's reuse; without it,
    /// the acquire wait was already consumed by an earlier (graph) submit.
    fn submit_with_stages(
        &mut self,
        mut command: VulkanCommandBuffer,
        acquire_stage: Option<vk::PipelineStageFlags>,
        dependency_stage: vk::PipelineStageFlags,
        dependencies: crate::sync::SyncSet,
    ) -> Result<PresentTicket> {
        if !command.is_executable()
            || acquire_stage.is_some_and(|stage| stage.is_empty())
            || dependency_stage.is_empty()
        {
            return Err(Error::invalid_usage("呈现命令未结束或等待阶段为空"));
        }
        if self.lease.is_none() {
            return Err(consumed());
        }
        let chain = self.swapchain.clone();
        chain.policy.allow_present()?;
        if !Arc::ptr_eq(&command.pool.device, &chain.device) {
            return Err(Error::invalid_usage("呈现命令来自其他设备"));
        }
        if command.pool.family != chain.device.queues.primary_family {
            return Err(Error::invalid_usage("呈现命令的命令池不属于呈现队列族"));
        }
        command.retain_object(chain.clone())?;
        command.retain_object(self.sync.clone())?;
        command.validate_submission()?;
        let device = chain.device.clone();
        let mut waits = super::submit::SubmitArray::default();
        let mut wait_values = super::submit::SubmitArray::default();
        let mut stages = super::submit::SubmitArray::default();
        let fence = if let Some(acquire_stage) = acquire_stage {
            if let Err(error) = self.presentation.prepare(ACQUIRE_TIMEOUT_NS) {
                self.recover_on_drop &= !error.is_device_lost();
                return Err(error);
            }
            let fence = self.sync.fence;
            if let Err(error) = wsi_call(&device.queues.timelines, || unsafe {
                device.core.device.reset_fences(&[fence])
            }) {
                self.recover_on_drop &= !error.is_device_lost();
                return Err(error);
            }
            waits.push(self.sync.acquired);
            wait_values.push(0);
            stages.push(acquire_stage);
            fence
        } else {
            vk::Fence::null()
        };
        for point in dependencies.points() {
            waits.push(device.queues.timelines.semaphore(point.queue)?);
            wait_values.push(point.value);
            stages.push(dependency_stage);
        }
        let signals = [self.presentation.rendered];
        let commands = [command.handle];
        let mut uses = command.use_cells().to_vec();
        crate::resource::tracked::normalize_submission_uses(&mut uses);
        let _reservations = crate::resource::tracked::lock_submission_uses(&uses)?;
        command.validate_submission()?;
        let native_states = unsafe {
            super::command_buffer::prepare_submission_states(
                &[&command],
                &uses,
                device.queue_id(device.queues.primary)?,
                device.queues.primary_family,
                &device.memory.stats,
            )?
        };
        let mut prepared = [super::submit::PreparedSubmit {
            commands: commands.into_iter().collect(),
            wait_semaphores: waits,
            wait_values,
            wait_stages: stages,
            signal_semaphores: signals.into_iter().collect(),
            signal_values: [0].into_iter().collect(),
        }];
        device.add_timeline_waits(
            device.queues.primary,
            &mut prepared,
            &native_states.dependencies(),
        )?;
        let point = match device.submit_prepared(device.queues.primary, &prepared, fence) {
            Ok(point) => point,
            Err(error) => {
                if error.is_device_lost() {
                    self.recover_on_drop = false;
                }
                return Err(error);
            }
        };
        native_states.publish(point);
        self.presentation
            .progress
            .lock()
            .record_gpu_submission(point);
        command.record_submission(point);
        if acquire_stage.is_some() {
            self.sync.state.lock().submitted = true;
        }
        self.present_submitted(point, Some(command))
    }
    fn present_submitted(
        &mut self,
        point: crate::sync::SyncPoint,
        command: Option<VulkanCommandBuffer>,
    ) -> Result<PresentTicket> {
        // acquire has already been consumed. Drop must never rescue it by
        // waiting on the binary semaphore a second time.
        self.recover_on_drop = false;
        let lease = self.lease.take().ok_or_else(consumed)?;
        // After a successful submit, establish the owner before presenting so
        // the in-flight command stays protected whatever present returns.
        let serial = self.presentation.progress.lock().issued + 1;
        let mut pending = PresentTicket {
            command,
            sync: self.sync.clone(),
            point,
            swapchain: self.swapchain.clone(),
            presentation: self.presentation.clone(),
            serial,
            image_index: lease.index,
            suboptimal: lease.suboptimal,
        };
        let chain = self.swapchain.clone();
        // The submitted command owns the layout transition; its binary signal
        // and generation are retained by `pending` and the image protocol.
        match unsafe { lease.present(&[self.presentation.rendered]) } {
            Ok(suboptimal) => {
                pending.suboptimal |= suboptimal;
                Ok(pending)
            }
            Err(error) => {
                chain.mark_stale();
                // A present that never enqueued its wait leaves rendered
                // signaled and the image acquired. acquire was already
                // consumed by the render submit; never wait on it again.
                let enqueued = self.presentation.progress.lock().issued >= serial;
                if !enqueued
                    && !error.is_device_lost()
                    && let Err(cleanup) = pending.finish_rejected_present(ACQUIRE_TIMEOUT_NS)
                {
                    log::warn!(target: "rhapsody::raw::swapchain", "present cleanup failed after {error}: {cleanup}");
                }
                drop(pending);
                Err(error)
            }
        }
    }
}
impl Drop for AcquiredImageLease {
    fn drop(&mut self) {
        if self.lease.is_some()
            && std::mem::take(&mut self.recover_on_drop)
            && let Err(error) = self.cancel_unsubmitted(ACQUIRE_TIMEOUT_NS)
        {
            log::warn!(target: "rhapsody::raw::swapchain", "acquired-image cleanup failed: {error}");
        }
        if self.lease.is_some() {
            self.swapchain.mark_stale();
            let mut state = self.sync.state.lock();
            state.abandoned = true;
            state.reserved = false;
        }
    }
}

/// Render and presentation have different completion domains. Dropping a ticket
/// releases CPU ownership without an explicit GPU wait. Final destruction of
/// the last swapchain owner still runs the generation's teardown protocol.
pub struct PresentTicket {
    command: Option<VulkanCommandBuffer>,
    sync: Arc<FrameSemaphores>,
    swapchain: Arc<Swapchain>,
    presentation: Arc<ImagePresentation>,
    serial: u64,
    image_index: u32,
    pub point: crate::sync::SyncPoint,
    pub suboptimal: bool,
}
impl PresentTicket {
    fn finish_rejected_present(&self, timeout: u64) -> Result<()> {
        self.wait(timeout)?;
        let device = &self.swapchain.device;
        let point = device.submit_prepared(
            device.queues.primary,
            &[super::submit::PreparedSubmit {
                commands: Default::default(),
                wait_semaphores: [self.presentation.rendered].into_iter().collect(),
                wait_values: [0].into_iter().collect(),
                wait_stages: [vk::PipelineStageFlags::TOP_OF_PIPE].into_iter().collect(),
                signal_semaphores: Default::default(),
                signal_values: Default::default(),
            }],
            vk::Fence::null(),
        )?;
        self.presentation
            .progress
            .lock()
            .record_gpu_submission(point);
        device.queues.timelines.wait(&point.into(), timeout)?;
        if device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            self.swapchain.release_image(self.image_index)?;
        }
        Ok(())
    }
    pub fn generation(&self) -> u64 {
        self.swapchain.generation()
    }
    /// Wait only for rendering. This does not authorize present semaphore reuse.
    pub fn wait(&self, timeout: u64) -> Result<()> {
        self.sync
            .device
            .queues
            .timelines
            .wait(&self.point.into(), timeout)
    }
    pub fn is_complete(&self) -> Result<bool> {
        Ok(crate::sync::SyncSet::from(self.point)
            .is_reached(&self.sync.device.queues.timelines.poll()?))
    }
    /// Without maintenance1, a newer acquisition fence proves presentation
    /// completion. Shutdown never treats queue-idle as presentation completion.
    pub fn is_present_complete(&self) -> Result<bool> {
        self.swapchain.collect_presentations()?;
        Ok(self.presentation.progress.lock().completed(self.serial))
    }
}
impl Drop for PresentTicket {
    fn drop(&mut self) {
        self.command.take();
        self.sync.release();
    }
}

impl crate::resource::GpuUse for Swapchain {
    fn visit_uses(&self, _visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::resource::GpuUse for FrameSemaphores {
    fn visit_uses(&self, _visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;
    use ash::vk::Handle;
    use std::cell::{Cell, RefCell};

    fn certain(serial: u64) -> PendingFence {
        PendingFence {
            serial,
            certain: true,
        }
    }
    fn uncertain(serial: u64) -> PendingFence {
        PendingFence {
            serial,
            certain: false,
        }
    }

    fn created_state() -> SwapchainInner {
        let mut state = SwapchainInner::new(SwapchainDesc {
            extent: vk::Extent2D {
                width: 4,
                height: 2,
            },
            vsync: true,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        });
        state.state = SwapchainState::Created;
        state.stale = false;
        state
    }

    #[test]
    fn fullscreen_focus_loss_blocks_new_acquire_but_allows_owned_frame_to_present() {
        let mut state = created_state();
        state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
        state.fullscreen.monitor = Some(42);
        state.fullscreen.status.acquired = true;
        state.acquire_with(|| Ok((0, false))).unwrap();
        state.fullscreen.status.focused = false;
        assert!(!state.can_replace());
        assert!(matches!(
            state.acquire_with(|| panic!("unfocused acquire")),
            Err(Error::Vulkan(vk::Result::NOT_READY))
        ));
        let mut progress = PresentProgress::default();
        assert!(
            !state
                .finish_present(&mut progress, true, Ok(false))
                .unwrap()
        );
        state.return_acquisition(true);
        assert!(state.can_replace());
        state.fullscreen.status.focused = true;
        assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
    }

    #[test]
    fn windowed_and_borderless_focus_loss_keeps_acquiring() {
        for mode in [WindowMode::Windowed, WindowMode::Borderless] {
            let mut state = created_state();
            state.fullscreen.status.requested = mode;
            state.fullscreen.status.focused = false;
            assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
            state.return_acquisition(true);
        }
        // Exclusive requested but fallen back to ordinary presentation.
        let mut state = created_state();
        state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
        state.fullscreen.status.fallback = Some(FullscreenFallback::Unsupported);
        state.fullscreen.status.focused = false;
        assert!(state.acquire_with(|| Ok((0, false))).is_ok());
    }

    #[test]
    fn fullscreen_present_loss_latches_fallback_and_keeps_wait_consumption_proof() {
        let mut state = created_state();
        state.fullscreen.status.requested = WindowMode::Exclusive { monitor: 42 };
        state.fullscreen.monitor = Some(42);
        state.fullscreen.status.acquired = true;
        let mut progress = PresentProgress::default();
        assert!(
            !state
                .finish_present(
                    &mut progress,
                    true,
                    Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)
                )
                .unwrap()
        );
        assert!(state.stale);
        assert!(!state.fullscreen.status.acquired);
        assert_eq!(
            state.fullscreen.status.fallback,
            Some(FullscreenFallback::ExclusiveLost)
        );
        assert_eq!(progress.issued, 1);
        assert_eq!(progress.completed, 0);
    }

    #[test]
    fn fullscreen_minimized_and_released_generations_cannot_acquire() {
        let mut state = created_state();
        state.desc.extent.width = 0;
        assert!(state.acquire_with(|| panic!("minimized acquire")).is_err());
        state.desc.extent.width = 4;
        state.fullscreen.monitor = Some(42);
        assert!(matches!(
            state.acquire_with(|| panic!("released exclusive acquire")),
            Err(Error::SurfaceOutOfDate)
        ));
    }

    #[derive(Debug, PartialEq, Eq)]
    enum FenceCall {
        Wait(vk::Fence, u64),
        Reset(vk::Fence),
        Status(vk::Fence),
    }
    #[derive(Default)]
    struct FaultFences {
        calls: RefCell<Vec<FenceCall>>,
        fail_at: Cell<Option<(usize, vk::Result)>>,
        ready: Cell<bool>,
    }
    impl FaultFences {
        fn call(&self, operation: FenceCall) -> Result<()> {
            self.calls.borrow_mut().push(operation);
            if let Some((index, error)) = self.fail_at.get()
                && self.calls.borrow().len() == index
            {
                self.fail_at.set(None);
                return Err(error.into());
            }
            Ok(())
        }
    }
    impl WsiFenceOperations for FaultFences {
        fn wait(&self, fence: vk::Fence, timeout: u64) -> Result<()> {
            self.call(FenceCall::Wait(fence, timeout))
        }
        fn reset(&self, fence: vk::Fence) -> Result<()> {
            self.call(FenceCall::Reset(fence))
        }
        fn signaled(&self, fence: vk::Fence) -> Result<bool> {
            self.call(FenceCall::Status(fence))?;
            Ok(self.ready.get())
        }
    }

    #[test]
    fn production_policy_requires_provable_shutdown_but_acquire_only_does_not() {
        assert_eq!(SwapchainPolicy::default(), SwapchainPolicy::Strict);
        assert_eq!(
            SwapchainPolicy::Strict.validate(false).unwrap_err().kind(),
            ErrorKind::Unsupported
        );
        assert!(SwapchainPolicy::Strict.validate(true).is_ok());
        assert!(SwapchainPolicy::AcquireOnly.validate(false).is_ok());
        assert_eq!(
            SwapchainPolicy::AcquireOnly
                .allow_present()
                .unwrap_err()
                .kind(),
            ErrorKind::Unsupported
        );
        assert!(SwapchainPolicy::RetainUnproven.validate(false).is_ok());
    }

    #[test]
    fn live_acquire_state_publishes_nothing_on_failure_and_can_retry() {
        for error in [
            vk::Result::NOT_READY,
            vk::Result::TIMEOUT,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            vk::Result::ERROR_DEVICE_LOST,
        ] {
            let mut state = created_state();
            let result = state.acquire_with(|| Err(error.into()));
            assert_eq!(result.unwrap_err().kind(), Error::from(error).kind());
            assert_eq!(state.active_acquisitions, 0);
            assert_eq!(state.next_slot, 0);
            assert!(!state.stale);
            if error == vk::Result::ERROR_DEVICE_LOST {
                // The runtime's wsi_call observes terminal loss before lease
                // cleanup; this state object alone does not own that runtime.
                continue;
            }
            assert_eq!(state.acquire_with(|| Ok((1, false))).unwrap(), (1, false));
            assert_eq!(state.active_acquisitions, 1);
            state.return_acquisition(true);
        }
        let mut state = created_state();
        assert!(state.acquire_with(|| Err(Error::SurfaceOutOfDate)).is_err());
        assert!(state.stale);
        assert_eq!(state.active_acquisitions, 0);
        assert!(
            state
                .acquire_with(|| panic!("stale generations must not call Vulkan"))
                .is_err()
        );
        state.stale = false;
        state.closing = true;
        assert!(
            state
                .acquire_with(|| panic!("closing generations must not call Vulkan"))
                .is_err()
        );
    }

    #[test]
    fn live_slot_failure_matrix_never_resets_before_the_actual_waits_complete() {
        let acquired = vk::Fence::from_raw(11);
        let submitted = vk::Fence::from_raw(12);
        for failure in 1..=3 {
            let operations = FaultFences::default();
            operations.fail_at.set(Some((failure, vk::Result::TIMEOUT)));
            let mut state = FrameSlotState {
                submitted: true,
                acquired: true,
                ..Default::default()
            };
            assert_eq!(
                state
                    .reserve_with(&operations, acquired, submitted, 7)
                    .unwrap_err()
                    .kind(),
                ErrorKind::Timeout
            );
            assert!(!state.reserved);
            assert_eq!(state.submitted, failure == 1);
            assert_eq!(state.acquired, failure <= 2);
            assert_eq!(operations.calls.borrow().len(), failure);
            if failure <= 2 {
                assert!(
                    !operations
                        .calls
                        .borrow()
                        .iter()
                        .any(|call| matches!(call, FenceCall::Reset(_)))
                );
            }
            state
                .reserve_with(&operations, acquired, submitted, 9)
                .unwrap();
            assert!(state.reserved);
            assert!(!state.submitted);
            assert!(!state.acquired);
            let calls = operations.calls.borrow().len();
            assert!(
                state
                    .reserve_with(&operations, acquired, submitted, 9)
                    .is_err()
            );
            assert_eq!(
                operations.calls.borrow().len(),
                calls,
                "a reserved slot cannot mutate Vulkan state"
            );
        }
    }

    #[test]
    fn live_present_fence_timeout_preserves_serial_and_prevents_reset() {
        let fence = vk::Fence::from_raw(17);
        let operations = FaultFences::default();
        operations.fail_at.set(Some((1, vk::Result::TIMEOUT)));
        let mut progress = PresentProgress::default();
        progress.record_result(&Ok(false), true);
        assert!(progress.prepare_with(&operations, fence, 7).is_err());
        assert_eq!(progress.pending_fence, Some(certain(1)));
        assert!(!progress.all_presentations_complete());
        assert_eq!(*operations.calls.borrow(), [FenceCall::Wait(fence, 7)]);
        progress.prepare_with(&operations, fence, 9).unwrap();
        assert_eq!(progress.pending_fence, None);
        assert!(progress.all_presentations_complete());
        assert_eq!(
            *operations.calls.borrow(),
            [
                FenceCall::Wait(fence, 7),
                FenceCall::Wait(fence, 9),
                FenceCall::Reset(fence)
            ]
        );
    }

    #[test]
    fn live_reclaim_poll_does_not_publish_completion_on_status_or_wait_failure() {
        let fence = vk::Fence::from_raw(19);
        for failure in 1..=2 {
            let operations = FaultFences::default();
            operations.ready.set(true);
            operations
                .fail_at
                .set(Some((failure, vk::Result::ERROR_OUT_OF_HOST_MEMORY)));
            let mut progress = PresentProgress::default();
            progress.record_result(&Ok(false), true);
            assert_eq!(
                progress.poll_with(&operations, fence).unwrap_err().kind(),
                ErrorKind::OutOfMemory
            );
            assert_eq!(progress.pending_fence, Some(certain(1)));
            assert!(!progress.all_presentations_complete());
            progress.poll_with(&operations, fence).unwrap();
            assert!(progress.all_presentations_complete());
            assert_eq!(progress.pending_fence, None);
            assert!(
                !operations
                    .calls
                    .borrow()
                    .iter()
                    .any(|call| matches!(call, FenceCall::Reset(_)))
            );
        }
    }

    #[test]
    fn live_shutdown_rejects_unproven_unextended_present_without_gpu_wait_shortcut() {
        let operations = FaultFences::default();
        let mut progress = PresentProgress::default();
        progress.record_result(&Ok(false), false);
        assert_eq!(
            progress
                .finish_shutdown_with(&operations, vk::Fence::null(), 7)
                .unwrap_err()
                .kind(),
            ErrorKind::Unsupported
        );
        assert!(!progress.all_presentations_complete());
        assert!(
            operations.calls.borrow().is_empty(),
            "there is no invented queue-idle proof"
        );
        // The production newer-generation protocol calls this only after a
        // real completed present in the replacement generation.
        progress.generation_completed();
        progress
            .finish_shutdown_with(&operations, vk::Fence::null(), 7)
            .unwrap();
        assert!(progress.all_presentations_complete());
    }

    #[test]
    fn live_shutdown_timeout_retains_pending_fence_for_a_successful_retry() {
        let fence = vk::Fence::from_raw(23);
        let operations = FaultFences::default();
        operations.fail_at.set(Some((1, vk::Result::TIMEOUT)));
        let mut progress = PresentProgress::default();
        progress.record_result(&Ok(false), true);
        assert_eq!(
            progress
                .finish_shutdown_with(&operations, fence, 7)
                .unwrap_err()
                .kind(),
            ErrorKind::Timeout
        );
        assert_eq!(progress.pending_fence, Some(certain(1)));
        assert!(!progress.all_presentations_complete());
        progress
            .finish_shutdown_with(&operations, fence, 9)
            .unwrap();
        assert!(progress.all_presentations_complete());
        assert_eq!(progress.pending_fence, None);
        assert_eq!(
            *operations.calls.borrow(),
            [FenceCall::Wait(fence, 7), FenceCall::Wait(fence, 9)]
        );
    }

    #[test]
    fn live_present_error_matrix_retains_enqueued_operations_only() {
        for (error, enqueued) in [
            (vk::Result::ERROR_OUT_OF_DATE_KHR, true),
            (vk::Result::ERROR_SURFACE_LOST_KHR, true),
            (vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT, true),
            (vk::Result::ERROR_OUT_OF_HOST_MEMORY, false),
            (vk::Result::ERROR_OUT_OF_DEVICE_MEMORY, false),
            (vk::Result::ERROR_DEVICE_LOST, false),
        ] {
            let mut inner = created_state();
            let mut progress = PresentProgress::default();
            let result = inner.finish_present(&mut progress, true, Err(error));
            assert!(inner.stale);
            assert_eq!(progress.issued, u64::from(enqueued));
            // Rejected presents enqueue their wait, but their fence is not
            // guaranteed to signal: it is polled, never waited on.
            assert_eq!(progress.pending_fence, enqueued.then_some(uncertain(1)));
            assert_eq!(progress.all_presentations_complete(), !enqueued);
            if matches!(
                error,
                vk::Result::ERROR_OUT_OF_DATE_KHR
                    | vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
            ) {
                assert!(!result.unwrap());
            } else {
                assert_eq!(result.unwrap_err().kind(), Error::from(error).kind());
            }
        }
    }

    struct TestTimelineApi;
    impl crate::sync::timeline::TimelineApi for TestTimelineApi {
        fn counter_value(&self, _: vk::Semaphore) -> Result<u64> {
            Ok(0)
        }
        fn wait_all(&self, _: &[vk::Semaphore], _: &[u64], _: u64) -> Result<bool> {
            Ok(true)
        }
    }

    #[test]
    fn rapid_resize_generation_policy_applies_bounded_backpressure_until_collection() {
        let mut ancestors = 0;
        let mut accepted = 0;
        for _ in 0..100 {
            if check_retired_generation_budget(ancestors).is_ok() {
                ancestors += 1;
                accepted += 1;
            }
            assert!(ancestors <= MAX_RETIRED_GENERATIONS);
        }
        assert_eq!(accepted, MAX_RETIRED_GENERATIONS);
        assert_eq!(
            check_retired_generation_budget(ancestors)
                .unwrap_err()
                .kind(),
            ErrorKind::Exhausted
        );
        // The current chain remains owned on rejection; completion collection
        // can release ancestors and admit a later replacement without a reset.
        ancestors -= 1;
        assert!(check_retired_generation_budget(ancestors).is_ok());
        ancestors += 1;
        assert!(check_retired_generation_budget(ancestors).is_err());
        ancestors = 0;
        assert!(check_retired_generation_budget(ancestors).is_ok());
    }

    #[test]
    fn wsi_device_loss_is_sticky_before_cleanup_can_submit_again() {
        use crate::sync::{QueueId, timeline::Timelines};
        use std::cell::Cell;
        for operation in [
            "acquire",
            "fence wait",
            "fence status",
            "fence reset",
            "release",
            "creation",
        ] {
            let timelines = Timelines::new(TestTimelineApi, &[vk::Semaphore::null()]).unwrap();
            let calls = Cell::new(0);
            let error = wsi_call(&timelines, || {
                calls.set(calls.get() + 1);
                Err::<(), _>(vk::Result::ERROR_DEVICE_LOST)
            })
            .unwrap_err();
            assert!(error.is_device_lost(), "{operation}");
            assert!(timelines.is_device_lost(), "{operation}");
            assert!(
                wsi_call(&timelines, || {
                    calls.set(calls.get() + 1);
                    Ok::<_, vk::Result>(())
                })
                .unwrap_err()
                .is_device_lost()
            );
            assert_eq!(calls.get(), 1, "known-lost cleanup must not call Vulkan");
            let result: Result<(crate::sync::SyncPoint, ())> = timelines
                .submit(QueueId::new(0).unwrap(), |_| {
                    panic!("a WSI loss must block subsequent queue submission")
                });
            assert!(result.unwrap_err().is_device_lost());
        }
    }

    #[test]
    fn recoverable_wsi_errors_keep_their_category_and_allow_retry() {
        use crate::sync::timeline::Timelines;
        let timelines = Timelines::new(TestTimelineApi, &[vk::Semaphore::null()]).unwrap();
        for error in [
            vk::Result::TIMEOUT,
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            vk::Result::ERROR_OUT_OF_DATE_KHR,
            vk::Result::ERROR_SURFACE_LOST_KHR,
        ] {
            let observed = wsi_call(&timelines, || Err::<(), _>(error)).unwrap_err();
            assert_eq!(observed.kind(), Error::from(error).kind());
            assert!(!timelines.is_device_lost());
            assert!(wsi_call(&timelines, || Ok::<_, vk::Result>(())).is_ok());
        }
        assert!(
            wsi_call(&timelines, || Err::<(), _>(Error::DeviceLost))
                .unwrap_err()
                .is_device_lost()
        );
        assert!(
            timelines.is_device_lost(),
            "internal rescue command errors are observed too"
        );
    }

    #[test]
    fn acquire_suboptimal_keeps_image_usable_and_only_out_of_date_rebuilds() {
        let mut state = created_state();
        assert_eq!(state.acquire_with(|| Ok((2, true))).unwrap(), (2, true));
        state.return_acquisition(true);
        assert!(!state.stale);
        for (error, kind) in [
            (vk::Result::NOT_READY, ErrorKind::Vulkan),
            (vk::Result::TIMEOUT, ErrorKind::Timeout),
        ] {
            assert_eq!(
                state.acquire_with(|| Err(error.into())).unwrap_err().kind(),
                kind
            );
            assert!(!state.stale);
        }
        assert_eq!(
            state
                .acquire_with(|| Err(vk::Result::ERROR_OUT_OF_DATE_KHR.into()))
                .unwrap_err()
                .kind(),
            ErrorKind::SurfaceOutOfDate
        );
        assert!(state.stale);
    }

    #[test]
    fn swapchain_state_lives_in_one_rwlock() {
        let desc = SwapchainDesc {
            extent: vk::Extent2D {
                width: 4,
                height: 2,
            },
            vsync: true,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        };
        let lock: RwLock<SwapchainInner> = RwLock::new(SwapchainInner::new(desc));
        {
            let (r1, r2) = (lock.read(), lock.read());
            assert_eq!(r1.desc, r2.desc);
            assert!(lock.try_write().is_none(), "有读者时写锁不可得");
        }
        let mut w = lock.write();
        assert_eq!(w.state, SwapchainState::NotCreated);
        assert!(w.stale, "未创建视为陈旧");
        assert!(lock.try_read().is_none(), "写者独占");
        w.state = SwapchainState::Created;
    }

    #[test]
    fn present_progress_requires_acquisition_completion_for_the_same_image() {
        let mut a = PresentProgress::default();
        let mut b = PresentProgress::default();
        let first = a.issue();
        let other = b.issue();
        b.acquired(other);
        assert!(!a.completed(first));
        a.acquired(first);
        assert!(a.completed(first));
        let second = a.issue();
        assert!(!a.completed(second));
        a.acquired(first);
        assert!(!a.completed(second));
    }

    #[test]
    fn present_failure_matrix_distinguishes_enqueued_waits_from_rejected_submits() {
        for error in [
            vk::Result::ERROR_OUT_OF_DATE_KHR,
            vk::Result::ERROR_SURFACE_LOST_KHR,
            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
        ] {
            assert!(present_was_enqueued(&Err(error)));
        }
        for error in [
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            vk::Result::ERROR_DEVICE_LOST,
        ] {
            assert!(!present_was_enqueued(&Err(error)));
        }
        assert!(present_was_enqueued(&Ok(false)));
        assert!(present_was_enqueued(&Ok(true)));
    }

    #[test]
    fn generation_replacement_waits_for_every_acquisition_to_return() {
        let mut state = SwapchainInner::new(SwapchainDesc {
            extent: vk::Extent2D {
                width: 4,
                height: 2,
            },
            vsync: true,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        });
        state.stale = false;
        state.acquired();
        state.acquired();
        assert!(!state.can_replace());
        state.return_acquisition(true);
        assert!(
            !state.can_replace(),
            "the second lease may still submit an old present"
        );
        assert!(!state.stale);
        state.return_acquisition(false);
        assert!(state.can_replace());
        assert!(
            state.stale,
            "an abandoned raw acquisition requires retirement"
        );
    }

    #[test]
    fn rejected_present_does_not_publish_a_fence_or_completion_serial() {
        let mut state = PresentProgress::default();
        for error in [
            vk::Result::ERROR_OUT_OF_HOST_MEMORY,
            vk::Result::ERROR_OUT_OF_DEVICE_MEMORY,
            vk::Result::ERROR_DEVICE_LOST,
        ] {
            state.record_result(&Err(error), true);
            assert_eq!(state.issued, 0);
            assert_eq!(state.completed, 0);
            assert_eq!(state.pending_fence, None);
        }
        state.record_result(&Ok(false), true);
        assert_eq!(state.issued, 1);
        assert_eq!(state.pending_fence, Some(certain(1)));
        assert!(!state.completed(1));
        state.acquired(1);
        state.pending_fence = None;
        state.record_result(&Err(vk::Result::ERROR_OUT_OF_DATE_KHR), true);
        assert_eq!(state.issued, 2);
        assert_eq!(state.pending_fence, Some(uncertain(2)));
        assert!(!state.completed(2));
    }

    #[test]
    fn rejected_present_cleanup_keeps_its_submit_point_after_wait_timeout() {
        use crate::sync::{CompletedValues, QueueId, SyncPoint};
        let queue = QueueId::new(0).unwrap();
        let other_queue = QueueId::new(1).unwrap();
        let mut state = PresentProgress::default();
        let render = SyncPoint { queue, value: 4 };
        state.record_gpu_submission(render);
        state.record_result(&Err(vk::Result::ERROR_OUT_OF_HOST_MEMORY), true);
        let mut completed = CompletedValues::default();
        completed.raise(queue, render.value);
        assert!(state.retirement_ready(&completed));
        // The cleanup queue submit succeeds, but its bounded wait times out.
        // Completion of render/present alone must no longer permit retirement.
        let cleanup = SyncPoint { queue, value: 5 };
        state.record_gpu_submission(cleanup);
        assert!(!state.retirement_ready(&completed));
        completed.raise(other_queue, u64::MAX);
        assert!(!state.retirement_ready(&completed));
        assert_eq!(state.gpu_use.get(queue), cleanup.value);
        completed.raise(queue, cleanup.value);
        assert!(state.retirement_ready(&completed));
    }

    #[test]
    fn recovery_render_submission_also_blocks_retirement_until_gpu_completion() {
        use crate::sync::{CompletedValues, QueueId, SyncPoint};
        let queue = QueueId::new(0).unwrap();
        let point = SyncPoint { queue, value: 7 };
        let mut state = PresentProgress::default();
        state.record_gpu_submission(point);
        state.record_result(&Ok(false), false);
        // A present/acquire completion proof cannot erase a queued GPU use.
        state.acquired(state.issued);
        let mut completed = CompletedValues::default();
        assert!(!state.retirement_ready(&completed));
        completed.raise(queue, point.value);
        assert!(state.retirement_ready(&completed));
    }

    #[test]
    fn unextended_present_completion_requires_reacquisition_proof() {
        let mut state = PresentProgress::default();
        state.record_result(&Ok(false), false);
        state.record_result(&Err(vk::Result::ERROR_SURFACE_LOST_KHR), false);
        assert_eq!(state.issued, 2);
        assert_eq!(state.pending_fence, None);
        assert!(!state.completed(1));
        state.acquired(1);
        assert!(state.completed(1));
        assert!(!state.completed(2));
    }

    #[test]
    fn wsi_result_policy_classifies_nonfatal_codes() {
        assert_eq!(present_result(Ok(false)).unwrap(), (false, false));
        // SUBOPTIMAL alone does not retire; note_suboptimal decides.
        assert_eq!(present_result(Ok(true)).unwrap(), (true, false));
        assert_eq!(
            present_result(Err(vk::Result::ERROR_OUT_OF_DATE_KHR)).unwrap(),
            (false, true)
        );
        assert_eq!(
            present_result(Err(vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT)).unwrap(),
            (false, true)
        );
        assert_eq!(
            present_result(Err(vk::Result::ERROR_SURFACE_LOST_KHR))
                .unwrap_err()
                .kind(),
            ErrorKind::SurfaceLost
        );
    }

    #[test]
    fn rejected_present_fence_is_polled_never_waited() {
        let fence = vk::Fence::from_raw(29);
        for error in [
            vk::Result::ERROR_OUT_OF_DATE_KHR,
            vk::Result::ERROR_SURFACE_LOST_KHR,
            vk::Result::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
        ] {
            // Unsignaled: reuse is refused without blocking.
            let operations = FaultFences::default();
            let mut progress = PresentProgress::default();
            progress.record_result(&Err(error), true);
            assert_eq!(
                progress
                    .prepare_with(&operations, fence, 7)
                    .unwrap_err()
                    .kind(),
                ErrorKind::SurfaceOutOfDate
            );
            assert_eq!(*operations.calls.borrow(), [FenceCall::Status(fence)]);
            assert_eq!(progress.pending_fence, Some(uncertain(1)));
            // Teardown relies on the render timeline proof, never a wait.
            progress
                .finish_shutdown_with(&operations, fence, 7)
                .unwrap();
            assert!(progress.all_presentations_complete());
            assert!(
                !operations
                    .calls
                    .borrow()
                    .iter()
                    .any(|call| matches!(call, FenceCall::Wait(..))),
                "{error:?}"
            );
            // Signaled: consumed and reusable.
            let operations = FaultFences::default();
            operations.ready.set(true);
            let mut progress = PresentProgress::default();
            progress.record_result(&Err(error), true);
            progress.prepare_with(&operations, fence, 7).unwrap();
            assert!(progress.all_presentations_complete());
            assert_eq!(
                *operations.calls.borrow(),
                [FenceCall::Status(fence), FenceCall::Reset(fence)]
            );
        }
    }

    #[test]
    fn persistent_suboptimal_recreates_only_on_surface_change() {
        let extent = vk::Extent2D {
            width: 64,
            height: 32,
        };
        let identity = vk::SurfaceTransformFlagsKHR::IDENTITY;
        let caps = |width, transform| vk::SurfaceCapabilitiesKHR {
            current_extent: vk::Extent2D { width, height: 32 },
            current_transform: transform,
            ..Default::default()
        };
        assert!(!suboptimal_requires_recreate(
            &caps(64, identity),
            extent,
            identity
        ));
        assert!(suboptimal_requires_recreate(
            &caps(65, identity),
            extent,
            identity
        ));
        assert!(suboptimal_requires_recreate(
            &caps(64, vk::SurfaceTransformFlagsKHR::ROTATE_90),
            extent,
            identity
        ));
        // Window-defined extent: only an explicit resize retires the chain.
        assert!(!suboptimal_requires_recreate(
            &caps(u32::MAX, identity),
            extent,
            identity
        ));
    }
}

// WSI owners retain the existing presentation completion protocol. GPU use cells
// must not be interpreted as proof that presentation resources can be released.
