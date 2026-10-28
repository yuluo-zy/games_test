//! Library-owned fullscreen policy. Window styling and monitor selection belong
//! to the host; every native operation is serialized by the swapchain's WSI lock.
use super::*;
use super::wsi::wsi_call;

type SurfaceConfiguration = (
    vk::SurfaceCapabilitiesKHR,
    Vec<vk::SurfaceFormatKHR>,
    Vec<vk::PresentModeKHR>,
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowMode {
    #[default]
    Windowed,
    Borderless,
    /// A live Win32 HMONITOR supplied by the host (not a Vulkan handle).
    Exclusive {
        monitor: vk::HMONITOR,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullscreenFallback {
    Unsupported,
    AcquisitionFailed,
    CreationFailed,
    ExclusiveLost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullscreenStatus {
    pub requested: WindowMode,
    pub focused: bool,
    pub acquired: bool,
    /// A fallback stays latched until a new mode request or focus restoration.
    pub fallback: Option<FullscreenFallback>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FullscreenState {
    pub status: FullscreenStatus,
    /// Creation-time native policy of this particular generation.
    pub monitor: Option<vk::HMONITOR>,
    pub retired: bool,
}
impl Default for FullscreenState {
    fn default() -> Self {
        Self {
            status: FullscreenStatus {
                requested: WindowMode::Windowed,
                focused: true,
                acquired: false,
                fallback: None,
            },
            monitor: None,
            retired: false,
        }
    }
}
impl FullscreenState {
    fn set_focused(&mut self, focused: bool) -> bool {
        if self.status.focused == focused {
            return false;
        }
        self.status.focused = focused;
        if focused && matches!(self.status.requested, WindowMode::Exclusive { .. }) {
            self.status.fallback = None;
            return true;
        }
        false
    }
    pub fn replacement(mut self, supported: bool) -> Self {
        self.retired = false;
        self.status.acquired = false;
        self.monitor = match self.status.requested {
            WindowMode::Exclusive { monitor } if supported && self.status.fallback.is_none() => {
                Some(monitor)
            }
            WindowMode::Exclusive { .. } => {
                self.status
                    .fallback
                    .get_or_insert(FullscreenFallback::Unsupported);
                None
            }
            _ => None,
        };
        self
    }
    /// Acquisition pauses while unfocused only for a generation created in
    /// exclusive fullscreen; windowed and borderless keep presenting.
    pub fn paused(&self) -> bool {
        !self.status.focused && self.monitor.is_some()
    }
    pub fn lost(&mut self) {
        self.status.acquired = false;
        self.status.fallback = Some(FullscreenFallback::ExclusiveLost);
    }
    /// Returns true when a normal-present generation must replace this one.
    pub fn acquire(&mut self, call: impl FnOnce() -> Result<()>) -> Result<bool> {
        if self.retired || !self.status.focused || self.monitor.is_none() || self.status.acquired {
            return Ok(false);
        }
        match call() {
            Ok(()) => self.status.acquired = true,
            Err(Error::Vulkan(vk::Result::ERROR_INITIALIZATION_FAILED)) => {
                self.status.fallback = Some(FullscreenFallback::AcquisitionFailed);
                return Ok(true);
            }
            Err(Error::SurfaceOutOfDate) => {
                self.lost();
                return Ok(true);
            }
            Err(error) => return Err(error),
        }
        Ok(false)
    }
    pub fn release(&mut self, call: impl FnOnce() -> Result<()>) -> Result<()> {
        if !self.retired && self.status.acquired {
            match call() {
                Ok(()) => self.status.acquired = false,
                Err(Error::SurfaceOutOfDate) => self.lost(),
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}

impl Swapchain {
    /// Query exactly the same creation policy used in vkCreateSwapchainKHR.
    /// Unsupported monitor/policy combinations are downgraded before creation.
    pub(super) fn fullscreen_surface_configuration(&mut self) -> Result<SurfaceConfiguration> {
        if !self.fullscreen_extension_enabled() {
            return Ok((
                wsi_call(&self.device.queues.timelines, || unsafe {
                    self.surface
                        .loader
                        .get_physical_device_surface_capabilities(
                            self.device.core.physical,
                            self.surface.handle,
                        )
                })?,
                self.enumerate_surface_formats()?,
                wsi_call(&self.device.queues.timelines, || unsafe {
                    self.surface
                        .loader
                        .get_physical_device_surface_present_modes(
                            self.device.core.physical,
                            self.surface.handle,
                        )
                })?,
            ));
        }
        loop {
            let monitor = self.inner.get_mut().fullscreen.monitor;
            let mut exclusive = vk::SurfaceFullScreenExclusiveInfoEXT::default()
                .full_screen_exclusive(if monitor.is_some() {
                    vk::FullScreenExclusiveEXT::APPLICATION_CONTROLLED
                } else {
                    vk::FullScreenExclusiveEXT::DISALLOWED
                });
            let mut win32 = vk::SurfaceFullScreenExclusiveWin32InfoEXT::default()
                .hmonitor(monitor.unwrap_or(0));
            let mut surface = vk::PhysicalDeviceSurfaceInfo2KHR::default()
                .surface(self.surface.handle)
                .push_next(&mut exclusive);
            if monitor.is_some() {
                surface = surface.push_next(&mut win32);
            }
            let loader = ash::khr::get_surface_capabilities2::Instance::new(
                &self.device.core.instance.entry,
                &self.device.core.instance.instance,
            );
            let mut support = vk::SurfaceCapabilitiesFullScreenExclusiveEXT::default();
            let mut caps = vk::SurfaceCapabilities2KHR::default().push_next(&mut support);
            wsi_call(&self.device.queues.timelines, || unsafe {
                loader.get_physical_device_surface_capabilities2(
                    self.device.core.physical,
                    &surface,
                    &mut caps,
                )
            })?;
            let capabilities = caps.surface_capabilities;
            if monitor.is_some() && support.full_screen_exclusive_supported == vk::FALSE {
                let state = &mut self.inner.get_mut().fullscreen;
                state.monitor = None;
                state.status.fallback = Some(FullscreenFallback::Unsupported);
                continue;
            }
            let count = wsi_call(&self.device.queues.timelines, || unsafe {
                loader.get_physical_device_surface_formats2_len(self.device.core.physical, &surface)
            })?;
            let mut formats = vec![vk::SurfaceFormat2KHR::default(); count];
            wsi_call(&self.device.queues.timelines, || unsafe {
                loader.get_physical_device_surface_formats2(
                    self.device.core.physical,
                    &surface,
                    &mut formats,
                )
            })?;
            let modes = wsi_call(&self.device.queues.timelines, || unsafe {
                ash::ext::full_screen_exclusive::Instance::new(
                    &self.device.core.instance.entry,
                    &self.device.core.instance.instance,
                )
                .get_physical_device_surface_present_modes2(self.device.core.physical, &surface)
            })?;
            return Ok((
                capabilities,
                formats.into_iter().map(|f| f.surface_format).collect(),
                modes,
            ));
        }
    }
    pub(super) fn fullscreen_extension_enabled(&self) -> bool {
        cfg!(target_os = "windows")
            && self.surface.win32
            && self
                .device
                .core
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_EXT_full_screen_exclusive")
            && self
                .surface
                .instance
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_KHR_win32_surface")
            && self
                .surface
                .instance
                .enabled_extensions
                .iter()
                .any(|e| e == "VK_KHR_get_surface_capabilities2")
    }
    /// The caller holds the WSI lock.
    pub(super) fn release_fullscreen(&self, inner: &mut SwapchainInner) -> Result<()> {
        inner.fullscreen.release(|| {
            wsi_call(&self.device.queues.timelines, || unsafe {
                ash::ext::full_screen_exclusive::Device::new(
                    &self.device.core.instance.instance,
                    &self.device.core.device,
                )
                .release_full_screen_exclusive_mode(self.handle)
            })
        })
    }
    /// Deferred until outstanding image leases return; existing frames may
    /// still present before release, while no new images are acquired.
    pub(super) fn maintain_fullscreen(&self) -> Result<()> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let _wsi = self.wsi.lock();
        let mut inner = self.inner.write();
        if !inner.can_replace() || inner.fullscreen.retired {
            return Ok(());
        }
        if !inner.fullscreen.status.focused
            || inner.desc.extent.width == 0
            || inner.desc.extent.height == 0
            || inner.closing
        {
            return self.release_fullscreen(&mut inner);
        }
        if !inner.stale {
            inner.stale |= inner.fullscreen.acquire(|| {
                wsi_call(&self.device.queues.timelines, || unsafe {
                    ash::ext::full_screen_exclusive::Device::new(
                        &self.device.core.instance.instance,
                        &self.device.core.device,
                    )
                    .acquire_full_screen_exclusive_mode(self.handle)
                })
            })?;
        }
        Ok(())
    }
    pub fn fullscreen_status(&self) -> FullscreenStatus {
        self.inner.read().fullscreen.status
    }
    /// Host focus event. Loss pauses acquisition immediately for a generation
    /// in exclusive fullscreen (windowed and borderless keep presenting).
    /// Release is deferred while a frame owns an image; the next acquisition
    /// attempt finishes it after those leases return.
    pub fn set_focused(&self, focused: bool) -> Result<FullscreenStatus> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        {
            let mut inner = self.inner.write();
            if inner.closing {
                return Err(Error::invalid_usage("交换链正在关闭"));
            }
            inner.stale |= inner.fullscreen.set_focused(focused);
        }
        self.maintain_fullscreen()?;
        Ok(self.fullscreen_status())
    }
    /// Request native presentation policy after updating the OS window style.
    /// Unsupported/denied exclusive access falls back to ordinary presentation;
    /// status reports that fallback. This method never changes window styles.
    /// Repeating an exclusive request explicitly retries a latched fallback.
    ///
    /// # Safety
    /// For Exclusive, monitor must be a valid Win32 HMONITOR corresponding to
    /// this surface's fullscreen window. Keep it valid until generation teardown;
    /// request the new monitor when the window moves between displays.
    pub unsafe fn set_window_mode(&self, mode: WindowMode) -> Result<FullscreenStatus> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if matches!(mode, WindowMode::Exclusive { monitor: 0 }) {
            return Err(Error::invalid_usage("独占全屏需要有效的 HMONITOR"));
        }
        let mut inner = self.inner.write();
        if inner.closing {
            return Err(Error::invalid_usage("交换链正在关闭"));
        }
        if inner.fullscreen.status.requested != mode || inner.fullscreen.status.fallback.is_some() {
            inner.fullscreen.status.requested = mode;
            inner.fullscreen.status.fallback = None;
            inner.stale = true;
        }
        Ok(inner.fullscreen.status)
    }
}

#[cfg(test)]
mod tests;
