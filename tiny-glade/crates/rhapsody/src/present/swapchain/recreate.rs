//! Swapchain creation, resize and surface replacement.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::device::Device;
use crate::native::Surface;
use parking_lot::Mutex;
use parking_lot::RwLock;
use std::sync::Arc;
use super::NEXT_SWAPCHAIN;
use super::Swapchain;
use super::SwapchainDesc;
use super::SwapchainInner;
use super::SwapchainState;
use super::ZERO_SURFACE_EXTENT;
use super::ZERO_WINDOW_EXTENT;
use super::frame_sync::FrameSemaphores;
use super::fullscreen::FullscreenFallback;
use super::fullscreen::FullscreenState;
use super::policy::SwapchainPolicy;
use super::policy::PresentProof;
use super::policy::check_retired_generation_budget;
use super::ACQUIRE_TIMEOUT_NS;
use super::MAX_RETIRED_GENERATIONS;
use super::progress::ImagePresentation;
use super::wsi::wsi_call;

impl Swapchain {
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

    /// Acquisition-only test entry; production frame loops require `Strict`.
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
            color_space: None,
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
        self.color_space = Some(format.color_space);
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

    /// WSI call that also latches SURFACE_LOST on the surface.
    pub(super) fn surface_call<T, E: Into<Error>>(
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
    pub(super) fn replacement_for(&self, surface: &Arc<Surface>) -> Result<Self> {
        if surface.is_lost() {
            return Err(Error::SurfaceLost);
        }
        let same_surface = Arc::ptr_eq(surface, &self.surface);
        // Every retry polls existing acquire/present/cleanup proofs first. If
        // none has progressed, retain the current generation and apply bounded
        // backpressure. Without present fences (no maintenance1) queue-idle is
        // the proof, so exhausting the budget pays one idle instead of failing.
        self.collect_presentations()?;
        if self.retired_generation_count() >= MAX_RETIRED_GENERATIONS
            && self.present_proof() == PresentProof::QueueIdle
        {
            let _wsi = self.wsi.lock();
            self.prove_presents_by_queue_idle(ACQUIRE_TIMEOUT_NS)?;
            drop(_wsi);
            self.collect_presentations()?;
        }
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

    pub(super) fn publish_replacement(chain: &mut Arc<Self>, replacement: Self) {
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
}
