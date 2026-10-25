//! 交换链托管。创建、获取与呈现复用 raw 层，CPU 退役与 GPU 完成分离。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    error::{Error, Result},
    swapchain::{
        ACQUIRE_TIMEOUT_NS, AcquiredSwapchainFrame, Swapchain, SwapchainDesc, SwapchainImageLease,
    },
};
use ash::vk;
use std::sync::Arc;
pub type ManagedSwapchain = Managed<Swapchain>;
impl Managed<Swapchain> {
    pub fn manage_swapchain(store: &ResourceManager<Swapchain>, swapchain: Swapchain) -> Self {
        store.allocate(swapchain)
    }
    pub fn raw(&self) -> vk::SwapchainKHR {
        self.handle
    }
    pub fn desc(&self) -> SwapchainDesc {
        self.resource().desc()
    }
    pub fn resize(&self, extent: vk::Extent2D) -> Result<()> {
        self.resource().resize(extent)
    }
    pub fn set_vsync(&self, vsync: bool) -> Result<()> {
        self.resource().set_vsync(vsync)
    }
    pub fn acquire_next(&self) -> Result<AcquiredSwapchainFrame> {
        self.0.resource.clone().acquire_frame(ACQUIRE_TIMEOUT_NS)
    }
    /// 窗口焦点变化：raw 层标记陈旧，强制重建。
    pub fn on_focus(&self) {
        self.resource().on_focus();
    }
    /// 呈现一帧；waits 必须来自该帧的提交信号量。
    pub fn present(&self, index: u32, waits: &[vk::Semaphore]) -> Result<bool> {
        self.resource().present(index, waits)
    }
    /// 退役仅禁止后续获取；已有租约仍保护资源。释放沿通用托管 store 路径，非原 ABI 镜像。
    pub fn retire(&self) {
        self.resource().mark_stale();
    }
    pub fn mark_stale(&self) {
        self.resource().mark_stale();
    }
    pub fn is_stale(&self) -> bool {
        self.resource().is_stale()
    }
    pub fn validate_swapchain_and_acquire_image(
        store: &ResourceManager<Swapchain>,
        current: &mut Self,
    ) -> Result<AcquiredSwapchainFrame> {
        match current.acquire_next() {
            Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DATE_KHR)) => {
                current.retire();
                let replacement = current.resource().replacement()?;
                *current = store.allocate(replacement);
                current.acquire_next()
            }
            result => result,
        }
    }
}

/// 交换链内层包装：暴露 `new`/`acquire_next_image`/`present_image`/`dispose` 原名。
/// 大部分委托 raw `Swapchain`；`set_fullscreen_exclusive` 为候选 gap（Vulkan 需要
/// `VK_EXT_full_screen_exclusive` 与窗口平台集成，本工程未接入）。
pub struct SwapchainInner {
    pub inner: Arc<Swapchain>,
}
impl SwapchainInner {
    pub fn new(swapchain: Arc<Swapchain>) -> Self {
        Self { inner: swapchain }
    }
    pub fn new_impl(swapchain: Arc<Swapchain>) -> Self {
        Self::new(swapchain)
    }
    pub fn raw(&self) -> vk::SwapchainKHR {
        self.inner.handle
    }
    pub fn acquire_next_image(&self, timeout: u64) -> Result<SwapchainImageLease> {
        self.inner.acquire_next_image(timeout, vk::Semaphore::null(), vk::Fence::null())
    }
    pub fn present_image(&self, index: u32, waits: &[vk::Semaphore]) -> Result<bool> {
        self.inner.present(index, waits)
    }
    pub fn on_focus(&self) {
        self.inner.on_focus();
    }
    pub fn resize(&self, extent: vk::Extent2D) -> Result<()> {
        self.inner.resize(extent)
    }
    pub fn set_vsync(&self, vsync: bool) -> Result<()> {
        self.inner.set_vsync(vsync)
    }
    pub fn mark_stale(&self) {
        self.inner.mark_stale();
    }
    pub fn retire(&self) {
        self.inner.mark_stale();
    }
    pub fn set_fullscreen_exclusive(&self, _exclusive: bool) -> Result<()> {
        Err(Error::Unsupported("full_screen_exclusive 扩展未接入"))
    }
    pub fn dispose(&self) {
        // 交换链销毁由 raw Drop/托管 store 延迟执行；这里仅标记陈旧。
        self.inner.mark_stale();
    }
}
