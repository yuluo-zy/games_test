//! 隐藏 Win32 窗口仅作为 WSI 验证夹具，不代替原作 winit 分叉的窗口流程。
#[cfg(target_os = "windows")]
mod probe {
    use ash::vk::{self, Handle};
    use rhapsody::raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        init::GraphicsInstance,
        instance::InstanceBuilder,
        managed::{resources::ResourceManager, swapchain::ManagedSwapchain},
        physical_device::enumerate_physical_devices,
        swapchain::{ACQUIRE_TIMEOUT_NS, AcquiredSwapchainFrame, Surface, Swapchain},
    };
    use serde_json::{Value, json};
    use std::{
        ffi::{CStr, c_void},
        sync::Arc,
    };
    type WinHandle = *mut c_void;
    #[repr(C)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        fn CreateWindowExW(
            ex_style: u32,
            class: *const u16,
            name: *const u16,
            style: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: WinHandle,
            menu: WinHandle,
            instance: WinHandle,
            param: WinHandle,
        ) -> WinHandle;
        fn DestroyWindow(window: WinHandle) -> i32;
        fn GetClientRect(window: WinHandle, rect: *mut Rect) -> i32;
        fn SetWindowPos(
            window: WinHandle,
            after: WinHandle,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            flags: u32,
        ) -> i32;
        fn IsWindowVisible(window: WinHandle) -> i32;
        fn IsWindow(window: WinHandle) -> i32;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleW(name: *const u16) -> WinHandle;
    }
    struct HiddenWindow {
        handle: WinHandle,
        instance: WinHandle,
    }
    impl HiddenWindow {
        fn close(mut self) -> Result<(), Box<dyn std::error::Error>> {
            if unsafe { DestroyWindow(self.handle) } == 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            assert_eq!(unsafe { IsWindow(self.handle) }, 0);
            self.handle = std::ptr::null_mut();
            Ok(())
        }
        fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let class: Vec<_> = "STATIC\0".encode_utf16().collect();
            let name: Vec<_> = "Rhapsody WSI 验证\0".encode_utf16().collect();
            let instance = unsafe { GetModuleHandleW(std::ptr::null()) };
            // WS_POPUP 不含 WS_VISIBLE，窗口永不显示或抢焦点。
            let handle = unsafe {
                CreateWindowExW(
                    0,
                    class.as_ptr(),
                    name.as_ptr(),
                    0x80000000,
                    0,
                    0,
                    320,
                    240,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    instance,
                    std::ptr::null_mut(),
                )
            };
            if handle.is_null() {
                return Err(std::io::Error::last_os_error().into());
            }
            assert_eq!(unsafe { IsWindowVisible(handle) }, 0);
            Ok(Self { handle, instance })
        }
        fn extent(&self) -> Result<vk::Extent2D, Box<dyn std::error::Error>> {
            let mut rect = Rect {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            if unsafe { GetClientRect(self.handle, &mut rect) } == 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            Ok(vk::Extent2D {
                width: (rect.right - rect.left) as u32,
                height: (rect.bottom - rect.top) as u32,
            })
        }
        fn resize(&self, width: i32, height: i32) -> Result<(), Box<dyn std::error::Error>> {
            // NOSENDCHANGING 之外保持正常尺寸通知；NOACTIVATE/NOMOVE/NOZORDER，不设置 SHOWWINDOW。
            if unsafe { SetWindowPos(self.handle, std::ptr::null_mut(), 0, 0, width, height, 0x16) }
                == 0
            {
                return Err(std::io::Error::last_os_error().into());
            }
            assert_eq!(unsafe { IsWindowVisible(self.handle) }, 0);
            Ok(())
        }
    }
    impl Drop for HiddenWindow {
        fn drop(&mut self) {
            if !self.handle.is_null() {
                unsafe { DestroyWindow(self.handle) };
            }
        }
    }
    fn frame(
        chain: &mut Arc<Swapchain>,
        pool: &Arc<VulkanCommandPool>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let frame = Swapchain::validate_swapchain_and_acquire_image(chain)?;
        render_frame(frame, pool)
    }
    fn render_frame(
        frame: AcquiredSwapchainFrame,
        pool: &Arc<VulkanCommandPool>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let index = frame.index();
        let image = frame.image();
        let extent = frame.extent();
        let slot = frame.slot_index();
        let acquire = frame.acquire_semaphore().as_raw();
        let render = frame.render_semaphore().as_raw();
        let fence = frame.submission_fence().as_raw();
        let device = pool.device.clone();
        let mut command = pool.allocate()?;
        command.begin()?;
        let readback = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(4, vk::BufferUsageFlags::TRANSFER_DST),
        )?);
        command.keep_alive(readback.clone());
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let barrier = [vk::ImageMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(range)];
        unsafe {
            device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &barrier,
            );
            device.device.cmd_clear_color_image(
                command.handle,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue {
                    float32: [0.0, 1.0, 0.0, 1.0],
                },
                &[range],
            );
            let barrier = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(image)
                .subresource_range(range)];
            device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &barrier,
            );
            let regions = [vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: 1,
                    height: 1,
                    depth: 1,
                })];
            device.device.cmd_copy_image_to_buffer(
                command.handle,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback.handle,
                &regions,
            );
            let barrier = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_READ)
                .dst_access_mask(vk::AccessFlags::empty())
                .old_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(image)
                .subresource_range(range)];
            let host = [vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(readback.handle)
                .size(vk::WHOLE_SIZE)];
            device.device.cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST | vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &host,
                &barrier,
            );
        }
        command.end()?;
        // acquire 与呈现均为真实二进制信号量依赖；host 等待发生在已经发出 present 之后。
        let pending = frame.submit_and_present(command, vk::PipelineStageFlags::TRANSFER)?;
        pending.wait(u64::MAX)?;
        let mut pixel = [0u8; 4];
        readback.read_bytes(0, &mut pixel)?;
        assert_eq!(pixel, [0, 255, 0, 255]);
        Ok(
            json!({"extent":[extent.width,extent.height],"image_index":index,"sync_slot":slot,"acquire_semaphore":acquire,"render_semaphore":render,"submission_fence":fence,"acquire_timeout_ns":ACQUIRE_TIMEOUT_NS,"pixel":pixel,"suboptimal":pending.suboptimal,"binary_acquire_render_present":true}),
        )
    }
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let window = HiddenWindow::new()?;
        let instance = InstanceBuilder {
            extensions: vec![
                ash::khr::surface::NAME.to_owned(),
                ash::khr::win32_surface::NAME.to_owned(),
            ],
            ..Default::default()
        }
        .build()?;
        let surface = unsafe {
            Surface::from_win32(
                instance.clone(),
                window.instance as isize,
                window.handle as isize,
            )
        }?;
        let graphics = GraphicsInstance {
            devices: enumerate_physical_devices(instance.clone())?,
            instance,
        };
        let mut selected = None;
        for physical in &graphics.devices {
            for (family, properties) in physical.queues.iter().enumerate() {
                if properties.queue_count > 0
                    && properties.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && unsafe {
                        surface.loader.get_physical_device_surface_support(
                            physical.handle,
                            family as u32,
                            surface.handle,
                        )
                    }?
                {
                    if let Ok(device) = graphics.create_device(
                        physical,
                        family as u32,
                        &[ash::khr::swapchain::NAME.as_ptr()],
                    ) {
                        selected = Some(device);
                        break;
                    }
                }
            }
            if selected.is_some() {
                break;
            }
        }
        let device = selected.ok_or("没有支持此表面的图形呈现设备")?;
        let properties = unsafe {
            graphics
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }.to_string_lossy();
        let usage = vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::COLOR_ATTACHMENT;
        let mut chain = Arc::new(unsafe {
            Swapchain::new(
                device.clone(),
                surface.clone(),
                window.extent()?,
                true,
                usage,
            )
        }?);
        let pool = VulkanCommandPool::new(device.clone())?;
        let first = frame(&mut chain, &pool)?;
        let mut frames = vec![first.clone()];
        for _ in 0..chain.images.len() {
            frames.push(frame(&mut chain, &pool)?);
        }
        assert_eq!(frames[0]["sync_slot"], frames.last().unwrap()["sync_slot"]);
        assert_eq!(
            frames[0]["acquire_semaphore"],
            frames.last().unwrap()["acquire_semaphore"]
        );
        assert_eq!(
            frames[0]["render_semaphore"],
            frames.last().unwrap()["render_semaphore"]
        );
        assert_eq!(
            frames[0]["submission_fence"],
            frames.last().unwrap()["submission_fence"]
        );
        let before_resize = chain.handle;
        window.resize(384, 256)?;
        chain.resize(window.extent()?)?;
        // resize 只登记新尺寸；当前 Vulkan handle/extent 不变，validate 才创建替代链。
        assert_eq!(chain.handle, before_resize);
        assert_eq!(
            chain.extent.width,
            first["extent"][0].as_u64().unwrap() as u32
        );
        assert!(chain.is_stale());
        let second = frame(&mut chain, &pool)?;
        assert_ne!(chain.handle, before_resize);
        assert_ne!(first["extent"], second["extent"]);
        let before_vsync = chain.handle;
        chain.set_vsync(false)?;
        assert_eq!(chain.handle, before_vsync);
        let third = frame(&mut chain, &pool)?;
        assert_ne!(chain.handle, before_vsync);
        // 丢弃获取后尚未提交的帧会标记 stale；后续获取必须明确失败。
        let abandoned = chain.acquire_frame(ACQUIRE_TIMEOUT_NS)?;
        drop(abandoned);
        assert!(chain.is_stale());
        assert!(chain.acquire_frame(0).is_err());
        let recovered = frame(&mut chain, &pool)?;
        assert!(!chain.is_stale());
        // 同一 GPU 后端继续验证 managed 消费者，而不是只测 raw 的重建辅助函数。
        let store = ResourceManager::new();
        let mut managed = store.allocate_shared(chain.clone());
        drop(chain);
        let managed_frame = render_frame(managed.acquire_next()?, &pool)?;
        let old_key = managed.key();
        managed.resize(vk::Extent2D {
            width: 384,
            height: 256,
        })?;
        managed.set_vsync(true)?;
        let managed_rebuilt = render_frame(
            ManagedSwapchain::validate_swapchain_and_acquire_image(&store, &mut managed)?,
            &pool,
        )?;
        assert_ne!(managed.key(), old_key);
        drop(managed);
        // 完整关闭消费者：先确认设备完成，再排出 managed CPU 退役及销毁桶。
        store.dispose_after_wait(|| device.wait_idle())?;
        drop(store);
        drop(pool);
        device.wait_idle()?;
        drop(device);
        drop(surface);
        drop(graphics);
        let hidden = unsafe { IsWindowVisible(window.handle) } == 0;
        window.close()?;
        println!(
            "{}",
            json!({"status":"passed","device":name,"window_hidden":hidden,"slot_reuse_frames":frames,"frames":[first,second,third,recovered,managed_frame,managed_rebuilt],"resize_registration_without_recreate":true,"validate_out_of_date_rebuild":true,"vsync_registration":true,"semaphore_and_fence_per_slot":true,"managed_rebuild":true,"abandoned_lease_stale":true,"window_destroyed":true,"present_completion_boundary":"queue_wait_idle only on slot reuse/disposal; conservative candidate","original_winit_flow_equivalence":false,"original_slot_abi_equivalence":false})
        );
        Ok(())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        probe::run()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("此探针只验证 Win32 WSI".into())
    }
}
