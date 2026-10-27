//! Image acquisition and presentation.
use ash::vk;
use crate::Error;
use crate::Result;
use std::sync::Arc;
use super::ACQUIRE_TIMEOUT_NS;
use super::Acquired;
use super::Swapchain;
use super::frame_sync::FrameSemaphores;
use super::is_zero_extent;
use super::lease::AcquiredImageLease;
use super::lease::SwapchainImageLease;
use super::policy::PresentPolicy;
use super::wsi::suboptimal_requires_recreate;

impl Swapchain {
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

    /// A SUBOPTIMAL acquire or present only retires this generation when the
    /// surface's current extent or transform differs from what it was created
    /// with; otherwise a persistent SUBOPTIMAL would rebuild every frame.
    pub(super) fn note_suboptimal(&self) {
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
        let lease = SwapchainImageLease {
            swapchain: Arc::clone(self), index, suboptimal, released: false,
            external_acquire_fence: fence,
        };
        if index as usize >= self.images.len() {
            drop(lease);
            return Err(Error::invalid_usage("driver returned an invalid swapchain image index"));
        }
        Ok(lease)
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
            on_failure: PresentPolicy::default(),
        })
    }

    /// Whether `vkReleaseSwapchainImagesEXT` is usable: the extension is
    /// enabled and the driver's headless WSI is not Mesa's software
    /// implementation, whose release entry point crashes.
    pub fn can_release_images(&self) -> bool {
        if !self
            .device
            .caps()
            .is_enabled(crate::device::Feature::SwapchainMaintenance1)
        {
            return false;
        }
        let mut driver = vk::PhysicalDeviceDriverProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut driver);
        unsafe {
            self.device
                .core
                .instance
                .instance
                .get_physical_device_properties2(self.device.core.physical, &mut properties);
        }
        driver.driver_id != vk::DriverId::MESA_LLVMPIPE
    }

    pub(super) fn release_image(&self, index: u32) -> Result<()> {
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

    pub(super) unsafe fn present_unnoted(&self, index: u32, wait: &[vk::Semaphore]) -> Result<bool> {
        self.policy.allow_present()?;
        if index as usize >= self.images.len() {
            return Err(Error::invalid_usage("交换链图片索引越界"));
        }
        let presentation = {
            let inner = self.inner.read();
            if inner.closing {
                return Err(Error::invalid_usage("交换链正在关闭"));
            }
            inner.presentations[index as usize].clone()
        };
        // Bounded fence wait for this image's previous present; no state lock.
        presentation.prepare(ACQUIRE_TIMEOUT_NS)?;
        let _wsi = self.wsi.lock();
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

    pub fn mark_stale(&self) {
        self.inner.write().stale = true;
    }

    pub fn is_stale(&self) -> bool {
        self.inner.read().stale
    }
}
