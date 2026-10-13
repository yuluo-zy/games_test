//! WSI 创建、逐槽获取、提交、呈现及延迟重建；原作槽结构与窗口 trait ABI 不作布局推测。
use super::{
    command_buffer::VulkanCommandBuffer,
    device::VulkanDevice,
    error::{Error, Result},
    instance::VulkanInstance,
};
use ash::vk;
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// 原 managed::acquire_next 的固定等待上限，单位为纳秒。
pub const ACQUIRE_TIMEOUT_NS: u64 = 3_000_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwapchainDesc {
    pub extent: vk::Extent2D,
    pub vsync: bool,
    pub usage: vk::ImageUsageFlags,
}

fn acquire_result(
    stale: &AtomicBool,
    result: std::result::Result<(u32, bool), vk::Result>,
) -> Result<(u32, bool)> {
    match result {
        Ok(image) => Ok(image),
        Err(error) => {
            if error == vk::Result::ERROR_OUT_OF_DATE_KHR {
                stale.store(true, Ordering::Release);
            }
            Err(error.into())
        }
    }
}

/// 只匹配原 present 的两个非致命错误；SURFACE_LOST 属于失败路径。
fn present_result(result: std::result::Result<bool, vk::Result>) -> Result<(bool, bool)> {
    match result {
        Ok(suboptimal) => Ok((suboptimal, suboptimal)),
        Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => Ok((false, true)),
        Err(error) if error.as_raw() == -1_000_255_000 => Ok((false, true)),
        Err(error) => Err(error.into()),
    }
}
pub struct Surface {
    pub instance: Arc<VulkanInstance>,
    pub handle: vk::SurfaceKHR,
    pub loader: ash::khr::surface::Instance,
    owned: bool,
}
impl Surface {
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
        }
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
        Ok(Arc::new(unsafe { Self::from_raw(instance, handle, true) }))
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
pub struct Swapchain {
    pub device: Arc<VulkanDevice>,
    pub surface: Arc<Surface>,
    pub handle: vk::SwapchainKHR,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub views: Vec<vk::ImageView>,
    pub loader: ash::khr::swapchain::Device,
    vsync: bool,
    usage: vk::ImageUsageFlags,
    requested: Mutex<SwapchainDesc>,
    slots: Mutex<FrameSlots>,
    stale: AtomicBool,
    any_presented: AtomicBool,
}
impl Swapchain {
    /// 至少成功 present 过一次（含 SUBOPTIMAL）。
    pub fn any_present_successful(&self) -> bool {
        self.any_presented.load(Ordering::Acquire)
    }
    /// # Safety
    /// 设备必须启用 VK_KHR_swapchain，surface 必须属于设备实例。
    pub unsafe fn new(
        device: Arc<VulkanDevice>,
        surface: Arc<Surface>,
        extent: vk::Extent2D,
        vsync: bool,
        usage: vk::ImageUsageFlags,
    ) -> Result<Self> {
        unsafe {
            Self::new_with_old(
                device,
                surface,
                extent,
                vsync,
                usage,
                vk::SwapchainKHR::null(),
            )
        }
    }
    unsafe fn new_with_old(
        device: Arc<VulkanDevice>,
        surface: Arc<Surface>,
        extent: vk::Extent2D,
        vsync: bool,
        usage: vk::ImageUsageFlags,
        old: vk::SwapchainKHR,
    ) -> Result<Self> {
        if !Arc::ptr_eq(&device.instance, &surface.instance) {
            return Err(Error::Invalid("交换链表面属于其他实例"));
        }
        if !unsafe {
            surface.loader.get_physical_device_surface_support(
                device.physical,
                device.queue_family,
                surface.handle,
            )
        }? {
            return Err(Error::Unsupported("队列不支持呈现"));
        }
        let loader = ash::khr::swapchain::Device::new(&device.instance.instance, &device.device);
        let mut result = Self {
            device,
            surface,
            handle: vk::SwapchainKHR::null(),
            format: vk::Format::UNDEFINED,
            extent,
            images: Vec::new(),
            views: Vec::new(),
            loader,
            vsync,
            usage,
            requested: Mutex::new(SwapchainDesc {
                extent,
                vsync,
                usage,
            }),
            slots: Mutex::new(FrameSlots {
                cursor: 0,
                slots: Vec::new(),
            }),
            stale: AtomicBool::new(true),
            any_presented: AtomicBool::new(false),
        };
        result.recreate(extent, old)?;
        Ok(result)
    }
    pub fn enumerate_surface_formats(&self) -> Result<Vec<vk::SurfaceFormatKHR>> {
        Ok(unsafe {
            self.surface
                .loader
                .get_physical_device_surface_formats(self.device.physical, self.surface.handle)
        }?)
    }
    fn recreate(&mut self, requested: vk::Extent2D, old: vk::SwapchainKHR) -> Result<()> {
        if requested.width == 0 || requested.height == 0 {
            self.mark_stale();
            return Err(Error::Invalid("窗口尺寸为零，延迟交换链重建"));
        }
        let caps = unsafe {
            self.surface
                .loader
                .get_physical_device_surface_capabilities(self.device.physical, self.surface.handle)
        }?;
        if !caps.supported_usage_flags.contains(self.usage) {
            return Err(Error::Unsupported("表面不支持请求的图片用途"));
        }
        let formats = self.enumerate_surface_formats()?;
        let mut format = *formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .or_else(|| formats.first())
            .ok_or(Error::Unsupported("表面没有可用格式"))?;
        if format.format == vk::Format::UNDEFINED {
            format.format = vk::Format::B8G8R8A8_SRGB;
        }
        let modes = unsafe {
            self.surface
                .loader
                .get_physical_device_surface_present_modes(
                    self.device.physical,
                    self.surface.handle,
                )
        }?;
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
            return Err(Error::Invalid("表面尺寸为零，等待窗口恢复"));
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
        .ok_or(Error::Unsupported("没有合成透明度模式"))?;
        let info = vk::SwapchainCreateInfoKHR::default()
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
        let _guard = self
            .device
            .queue_lock
            .lock();
        // 成功建立新交换链后旧对象可能已被驱动退役；任何后续失败都不得继续获取旧图像。
        self.stale.store(true, Ordering::Release);
        let handle = unsafe { self.loader.create_swapchain(&info, None) }?;
        let images = match unsafe { self.loader.get_swapchain_images(handle) } {
            Ok(images) => images,
            Err(error) => {
                unsafe {
                    self.loader.destroy_swapchain(handle, None);
                }
                return Err(error.into());
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
            match unsafe { self.device.device.create_image_view(&info, None) } {
                Ok(view) => views.push(view),
                Err(error) => {
                    unsafe {
                        for view in views {
                            self.device.device.destroy_image_view(view, None);
                        }
                        self.loader.destroy_swapchain(handle, None);
                    }
                    self.stale.store(true, Ordering::Release);
                    return Err(error.into());
                }
            }
        }
        unsafe {
            for view in self.views.drain(..) {
                self.device.device.destroy_image_view(view, None);
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
        let mut slots = Vec::with_capacity(self.images.len());
        for _ in &self.images {
            slots.push(Arc::new(FrameSemaphores::new(self.device.clone())?));
        }
        *self.slots.lock() = FrameSlots { cursor: 0, slots };
        self.stale.store(false, Ordering::Release);
        Ok(())
    }
    pub fn resize(&self, extent: vk::Extent2D) -> Result<()> {
        let mut desc = self
            .requested
            .lock();
        if desc.extent != extent {
            desc.extent = extent;
            self.mark_stale();
        }
        Ok(())
    }
    pub fn set_vsync(&self, vsync: bool) -> Result<()> {
        let mut desc = self
            .requested
            .lock();
        if desc.vsync != vsync {
            desc.vsync = vsync;
            self.mark_stale();
        }
        Ok(())
    }
    pub fn desc(&self) -> SwapchainDesc {
        *self.requested.lock()
    }
    /// 先将旧链标为退役；已有图像租约仍持有旧对象，不能被新链替换后提前销毁。
    pub(crate) fn replacement(&self) -> Result<Self> {
        self.mark_stale();
        let desc = self.desc();
        unsafe {
            Self::new_with_old(
                self.device.clone(),
                self.surface.clone(),
                desc.extent,
                desc.vsync,
                desc.usage,
                self.handle,
            )
        }
    }
    /// 只在 OUT_OF_DATE 上重建并重取一次；超时、NOT_READY 及其他错误原样向上返回。
    pub fn validate_swapchain_and_acquire_image(
        chain: &mut Arc<Self>,
    ) -> Result<AcquiredSwapchainFrame> {
        match chain.acquire_frame(ACQUIRE_TIMEOUT_NS) {
            Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DATE_KHR)) => {
                let replacement = chain.replacement()?;
                *chain = Arc::new(replacement);
                chain.acquire_frame(ACQUIRE_TIMEOUT_NS)
            }
            result => result,
        }
    }
    pub fn mark_stale(&self) {
        self.stale.store(true, Ordering::Release);
    }
    pub fn is_stale(&self) -> bool {
        self.stale.load(Ordering::Acquire)
    }
    pub fn on_focus(&self) {
        self.mark_stale();
    }
    pub fn acquire_next_image(
        self: &Arc<Self>,
        _timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> Result<SwapchainImageLease> {
        let _guard = self
            .device
            .queue_lock
            .lock();
        if self.is_stale() {
            return Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DATE_KHR));
        }
        let (index, suboptimal) = acquire_result(&self.stale, unsafe {
            self.loader
                .acquire_next_image(self.handle, ACQUIRE_TIMEOUT_NS, semaphore, fence)
        })?;
        Ok(SwapchainImageLease {
            swapchain: Arc::clone(self),
            index,
            suboptimal,
        })
    }
    /// 槽游标只在 acquire 成功后递增；信号量和 fence 在交换链创建时按槽分配。
    pub fn acquire_frame(self: &Arc<Self>, timeout: u64) -> Result<AcquiredSwapchainFrame> {
        if self.is_stale() {
            return Err(vk::Result::ERROR_OUT_OF_DATE_KHR.into());
        }
        let mut slots = self
            .slots
            .lock();
        let slot_index = slots.cursor;
        let sync = slots
            .slots
            .get(slot_index)
            .ok_or(Error::Invalid("交换链没有图像槽"))?
            .clone();
        sync.reserve()?;
        let lease = match self.acquire_next_image(timeout, sync.acquired, vk::Fence::null()) {
            Ok(lease) => lease,
            Err(error) => {
                sync.release();
                return Err(error);
            }
        };
        slots.cursor = (slot_index + 1) % slots.slots.len();
        Ok(AcquiredSwapchainFrame {
            lease: Some(lease),
            sync,
            slot_index,
        })
    }
    pub fn present(&self, index: u32, wait: &[vk::Semaphore]) -> Result<bool> {
        if index as usize >= self.images.len() {
            return Err(Error::Invalid("交换链图片索引越界"));
        }
        let chains = [self.handle];
        let indices = [index];
        let info = vk::PresentInfoKHR::default()
            .wait_semaphores(wait)
            .swapchains(&chains)
            .image_indices(&indices);
        let _guard = self
            .device
            .queue_lock
            .lock();
        let (suboptimal, stale) =
            present_result(unsafe { self.loader.queue_present(self.device.queue, &info) })?;
        self.any_presented.store(true, Ordering::Release);
        if stale {
            self.mark_stale();
        }
        Ok(suboptimal)
    }
}
impl Drop for Swapchain {
    fn drop(&mut self) {
        let _guard = self
            .device
            .queue_lock
            .lock();
        unsafe {
            // 这里只保护此链的呈现队列；尺寸登记与新链创建都不等待整台设备。
            // 原 retire/dispose 桶接线尚未恢复，此处仍是保守的 Vulkan 销毁边界。
            let _ = self.device.device.queue_wait_idle(self.device.queue);
            for view in self.views.drain(..) {
                self.device.device.destroy_image_view(view, None);
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
}
impl SwapchainImageLease {
    pub fn image(&self) -> vk::Image {
        self.swapchain.images[self.index as usize]
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.swapchain.views[self.index as usize]
    }
    pub fn present(self, wait: &[vk::Semaphore]) -> Result<bool> {
        self.swapchain.present(self.index, wait)
    }
}

struct FrameSemaphores {
    device: Arc<VulkanDevice>,
    acquired: vk::Semaphore,
    rendered: vk::Semaphore,
    fence: vk::Fence,
    state: Mutex<FrameSlotState>,
}
struct FrameSlots {
    cursor: usize,
    slots: Vec<Arc<FrameSemaphores>>,
}
#[derive(Default)]
struct FrameSlotState {
    reserved: bool,
    submitted: bool,
}
impl FrameSemaphores {
    fn new(device: Arc<VulkanDevice>) -> Result<Self> {
        let info = vk::SemaphoreCreateInfo::default();
        let acquired = unsafe { device.device.create_semaphore(&info, None) }?;
        let rendered = match unsafe { device.device.create_semaphore(&info, None) } {
            Ok(value) => value,
            Err(error) => {
                unsafe { device.device.destroy_semaphore(acquired, None) };
                return Err(error.into());
            }
        };
        let fence = match unsafe {
            device
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
        } {
            Ok(fence) => fence,
            Err(error) => {
                unsafe {
                    device.device.destroy_semaphore(rendered, None);
                    device.device.destroy_semaphore(acquired, None);
                }
                return Err(error.into());
            }
        };
        Ok(Self {
            device,
            acquired,
            rendered,
            fence,
            state: Mutex::new(FrameSlotState::default()),
        })
    }
    fn reserve(&self) -> Result<()> {
        let mut state = self
            .state
            .lock();
        if state.reserved {
            return Err(vk::Result::NOT_READY.into());
        }
        if state.submitted {
            unsafe {
                self.device
                    .device
                    .wait_for_fences(&[self.fence], true, ACQUIRE_TIMEOUT_NS)
            }?;
            // 现后端未启用 present-wait/maintenance 扩展。渲染 fence 不证明呈现已消费
            // rendered 信号量；仅在同槽再次使用时等待呈现队列，这是明确的保守候选边界。
            let _guard = self
                .device
                .queue_lock
                .lock();
            unsafe { self.device.device.queue_wait_idle(self.device.queue) }?;
            state.submitted = false;
        }
        state.reserved = true;
        Ok(())
    }
    fn release(&self) {
        self.state
            .lock()
            .reserved = false;
    }
}
impl Drop for FrameSemaphores {
    fn drop(&mut self) {
        if self.state.get_mut().submitted {
            let _guard = self
                .device
                .queue_lock
                .lock();
            unsafe {
                let _ = self
                    .device
                    .device
                    .wait_for_fences(&[self.fence], true, u64::MAX);
                let _ = self.device.device.queue_wait_idle(self.device.queue);
            }
        }
        unsafe {
            self.device.device.destroy_fence(self.fence, None);
            self.device.device.destroy_semaphore(self.rendered, None);
            self.device.device.destroy_semaphore(self.acquired, None);
        }
    }
}

/// 获取的图像只允许一次提交；未消费的租约会使交换链失效，强制后续重建。
pub struct AcquiredSwapchainFrame {
    lease: Option<SwapchainImageLease>,
    sync: Arc<FrameSemaphores>,
    slot_index: usize,
}
impl AcquiredSwapchainFrame {
    pub fn image(&self) -> vk::Image {
        self.lease.as_ref().unwrap().image()
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.lease.as_ref().unwrap().image_view()
    }
    pub fn index(&self) -> u32 {
        self.lease.as_ref().unwrap().index
    }
    pub fn slot_index(&self) -> usize {
        self.slot_index
    }
    pub fn acquire_semaphore(&self) -> vk::Semaphore {
        self.sync.acquired
    }
    pub fn render_semaphore(&self) -> vk::Semaphore {
        self.sync.rendered
    }
    pub fn submission_fence(&self) -> vk::Fence {
        self.sync.fence
    }
    pub fn extent(&self) -> vk::Extent2D {
        self.lease.as_ref().unwrap().swapchain.extent
    }
    /// 真实 GPU 依赖：acquire 信号量 → submit 等待 → rendered 信号量 → present 等待。
    /// 命令须已 end；wait_stage 是第一次访问交换链图像的阶段，不能用 CPU wait 替代。
    pub fn submit_and_present(
        mut self,
        mut command: VulkanCommandBuffer,
        wait_stage: vk::PipelineStageFlags,
    ) -> Result<PendingSwapchainFrame> {
        if !command.is_executable() || wait_stage.is_empty() {
            return Err(Error::Invalid("呈现命令未结束或等待阶段为空"));
        }
        let lease = self.lease.as_ref().unwrap();
        if !Arc::ptr_eq(&command.pool.device, &lease.swapchain.device) {
            return Err(Error::Invalid("呈现命令来自其他设备"));
        }
        if command.pool.family != lease.swapchain.device.queue_family {
            return Err(Error::Invalid("呈现命令的命令池不属于呈现队列族"));
        }
        command.keep_alive(lease.swapchain.clone());
        command.keep_alive(self.sync.clone());
        let device = lease.swapchain.device.clone();
        let fence = self.sync.fence;
        unsafe { device.device.reset_fences(&[fence]) }?;
        let waits = [self.sync.acquired];
        let stages = [wait_stage];
        let signals = [self.sync.rendered];
        let commands = [command.handle];
        let submits = [vk::SubmitInfo::default()
            .wait_semaphores(&waits)
            .wait_dst_stage_mask(&stages)
            .command_buffers(&commands)
            .signal_semaphores(&signals)];
        let result = {
            let _guard = device
                .queue_lock
                .lock();
            device.note_submit();
            unsafe { device.device.queue_submit(device.queue, &submits, fence) }
        };
        if let Err(error) = result {
            return Err(error.into());
        }
        self.sync
            .state
            .lock()
            .submitted = true;
        // 提交成功后，无论 present 成功与否，都先建立持有者以保护未完成的命令。
        let mut pending = PendingSwapchainFrame {
            command: Some(command),
            sync: self.sync.clone(),
            fence,
            suboptimal: lease.suboptimal,
        };
        let lease = self.lease.take().unwrap();
        match lease.present(&signals) {
            Ok(suboptimal) => {
                pending.suboptimal |= suboptimal;
                Ok(pending)
            }
            Err(error) => {
                drop(pending);
                Err(error)
            }
        }
    }
}
impl Drop for AcquiredSwapchainFrame {
    fn drop(&mut self) {
        if let Some(lease) = &self.lease {
            lease.swapchain.mark_stale();
            self.sync.release();
        }
    }
}

/// 获取帧与逐槽同步的租约；`into_sync_data` 暴露等待/信号/栅栏三元组。
pub struct SwapchainLeaseSync {
    frame: AcquiredSwapchainFrame,
}
impl SwapchainLeaseSync {
    pub fn new(frame: AcquiredSwapchainFrame) -> Self {
        Self { frame }
    }
    pub fn into_sync_data(&self) -> (vk::Semaphore, vk::Semaphore, vk::Fence) {
        (
            self.frame.acquire_semaphore(),
            self.frame.render_semaphore(),
            self.frame.submission_fence(),
        )
    }
    pub fn frame(&self) -> &AcquiredSwapchainFrame {
        &self.frame
    }
    pub fn into_frame(self) -> AcquiredSwapchainFrame {
        self.frame
    }
}

/// 已跟踪的交换链纹理租约；`into_present_source` 分离呈现租约与等待信号量。
pub struct TrackedSwapchainTextureLease {
    lease: SwapchainImageLease,
    waits: Vec<vk::Semaphore>,
}
impl TrackedSwapchainTextureLease {
    pub fn new(lease: SwapchainImageLease, waits: Vec<vk::Semaphore>) -> Self {
        Self { lease, waits }
    }
    pub fn image(&self) -> vk::Image {
        self.lease.image()
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.lease.image_view()
    }
    pub fn index(&self) -> u32 {
        self.lease.index
    }
    pub fn into_present_source(self) -> (SwapchainImageLease, Vec<vk::Semaphore>) {
        (self.lease, self.waits)
    }
}

/// 持有命令、交换链及信号量至实际提交与呈现完成。此安全候选采用保守清理边界。
pub struct PendingSwapchainFrame {
    command: Option<VulkanCommandBuffer>,
    sync: Arc<FrameSemaphores>,
    fence: vk::Fence,
    pub suboptimal: bool,
}
impl PendingSwapchainFrame {
    pub fn wait(&self, timeout: u64) -> Result<()> {
        unsafe {
            self.sync
                .device
                .device
                .wait_for_fences(&[self.fence], true, timeout)
        }?;
        Ok(())
    }
    /// 仅查询渲染提交的 fence；呈现等待仍由 wait/drop 的队列完成边界保护。
    pub fn is_complete(&self) -> Result<bool> {
        Ok(unsafe { self.sync.device.device.get_fence_status(self.fence) }?)
    }
}
impl Drop for PendingSwapchainFrame {
    fn drop(&mut self) {
        let _ = self.wait(u64::MAX);
        self.command.take();
        self.sync.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_suboptimal_keeps_image_usable_and_only_out_of_date_rebuilds() {
        let stale = AtomicBool::new(false);
        assert_eq!(acquire_result(&stale, Ok((2, true))).unwrap(), (2, true));
        assert!(!stale.load(Ordering::Acquire));
        for error in [vk::Result::NOT_READY, vk::Result::TIMEOUT] {
            assert!(
                matches!(acquire_result(&stale, Err(error)), Err(Error::Vulkan(code)) if code == error)
            );
            assert!(!stale.load(Ordering::Acquire));
        }
        assert!(matches!(
            acquire_result(&stale, Err(vk::Result::ERROR_OUT_OF_DATE_KHR)),
            Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DATE_KHR))
        ));
        assert!(stale.load(Ordering::Acquire));
    }

    #[test]
    fn wsi_result_policy_preserves_original_nonfatal_codes() {
        assert_eq!(ACQUIRE_TIMEOUT_NS, 3_000_000_000);
        assert_eq!(present_result(Ok(false)).unwrap(), (false, false));
        assert_eq!(present_result(Ok(true)).unwrap(), (true, true));
        assert_eq!(
            present_result(Err(vk::Result::ERROR_OUT_OF_DATE_KHR)).unwrap(),
            (false, true)
        );
        assert_eq!(
            present_result(Err(vk::Result::from_raw(-1_000_255_000))).unwrap(),
            (false, true)
        );
        assert!(matches!(
            present_result(Err(vk::Result::ERROR_SURFACE_LOST_KHR)),
            Err(Error::Vulkan(vk::Result::ERROR_SURFACE_LOST_KHR))
        ));
    }
}
