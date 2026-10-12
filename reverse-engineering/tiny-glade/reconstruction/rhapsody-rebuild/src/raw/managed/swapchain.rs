//! 交换链托管。创建、获取与呈现复用 raw 层，CPU 退役与 GPU 完成分离。
use super::{handle::Managed, resources::ResourceManager};
use crate::raw::{
    error::{Error, Result},
    swapchain::{ACQUIRE_TIMEOUT_NS, AcquiredSwapchainFrame, Swapchain, SwapchainDesc},
};
use ash::vk;
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
    /// 退役仅禁止后续获取；已有租约仍保护资源。释放沿通用托管 store 路径，非原 ABI 镜像。
    pub fn retire(&self) {
        self.resource().mark_stale();
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
