//! Presentation surfaces: the native `VkSurfaceKHR` with its sticky loss state.
use super::instance::VulkanInstance;
use crate::{Error, Result};
use ash::vk;
use std::sync::Arc;

pub struct Surface {
    pub(crate) instance: Arc<VulkanInstance>,
    pub(crate) handle: vk::SurfaceKHR,
    pub(crate) loader: ash::khr::surface::Instance,
    pub(crate) owned: bool,
    pub(crate) win32: bool,
    /// Sticky VK_ERROR_SURFACE_LOST_KHR. A lost surface never recovers; the
    /// host must create a new one and call [`Swapchain::replace_surface`].
    pub(crate) lost: std::sync::atomic::AtomicBool,
    // Dropped after Surface::drop destroys VkSurfaceKHR. Swapchain teardown
    // may retain this Surface on an unproven WSI timeout, so keep its actual
    // native window alive with it rather than relying on a host's field order.
    pub(crate) native_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
}

impl Surface {
    /// The instance that owns this object.
    pub fn instance(&self) -> &Arc<VulkanInstance> {
        &self.instance
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::SurfaceKHR {
        self.handle
    }
    /// Interop escape hatch: the extension function table.
    pub fn loader(&self) -> &ash::khr::surface::Instance {
        &self.loader
    }
}
impl Surface {
    /// True once any WSI call on this surface reported SURFACE_LOST.
    pub fn is_lost(&self) -> bool {
        self.lost.load(std::sync::atomic::Ordering::Acquire)
    }
    pub(crate) fn observe<T>(&self, result: &Result<T>) {
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
            native_owner: None,
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
        unsafe { Self::create_win32(instance, hinstance, hwnd, None) }
    }

    /// Create a Win32 surface retaining the actual HWND owner until Vulkan
    /// surface destruction, including deliberately retained WSI generations.
    /// This is one bounded owner slot; cloning/replacing a swapchain does not
    /// append additional native-window leases.
    ///
    /// # Safety
    /// In addition to from_win32's requirements, native_owner must retain the
    /// window that owns hwnd. The owner must not strongly refer to this Surface.
    pub unsafe fn from_win32_retained(
        instance: Arc<VulkanInstance>,
        hinstance: vk::HINSTANCE,
        hwnd: vk::HWND,
        native_owner: Arc<dyn std::any::Any + Send + Sync>,
    ) -> Result<Arc<Self>> {
        unsafe { Self::create_win32(instance, hinstance, hwnd, Some(native_owner)) }
    }

    unsafe fn create_win32(
        instance: Arc<VulkanInstance>,
        hinstance: vk::HINSTANCE,
        hwnd: vk::HWND,
        native_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    ) -> Result<Arc<Self>> {
        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(hinstance)
            .hwnd(hwnd);
        let handle = unsafe { loader.create_win32_surface(&info, None) }?;
        let mut surface = unsafe { Self::from_win32_raw(instance, handle, true) };
        surface.native_owner = native_owner;
        Ok(Arc::new(surface))
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
