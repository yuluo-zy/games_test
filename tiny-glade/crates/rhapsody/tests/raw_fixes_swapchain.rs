//! 交换链修复回归（lavapipe + `VK_EXT_headless_surface`）：零尺寸跳帧、acquire 超时与锁、格式回退用途校验。
use ash::vk;
use rhapsody::raw::{
    device::VulkanDevice,
    init::{DeviceSelectionPolicy, GraphicsInstance},
    instance::InstanceBuilder,
    physical_device::enumerate_physical_devices,
    swapchain::{Surface, Swapchain},
};
use std::{
    ffi::CString,
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};

const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 64,
    height: 32,
};

/// 不支持 headless surface 的环境返回 None（测试跳过）。
fn headless() -> Option<(Arc<VulkanDevice>, Arc<Surface>)> {
    let builder = InstanceBuilder {
        extensions: vec![
            CString::new("VK_KHR_surface").unwrap(),
            CString::new("VK_EXT_headless_surface").unwrap(),
        ],
        ..Default::default()
    };
    let instance = builder.build().ok()?;
    let devices = enumerate_physical_devices(instance.clone())
        .ok()?
        .into_iter()
        .map(Arc::new)
        .collect();
    let graphics = GraphicsInstance {
        instance: instance.clone(),
        devices,
    };
    let loader = ash::ext::headless_surface::Instance::new(&instance.entry, &instance.instance);
    let handle = unsafe {
        loader.create_headless_surface(&vk::HeadlessSurfaceCreateInfoEXT::default(), None)
    }
    .ok()?;
    let surface = Arc::new(unsafe { Surface::from_raw(instance.clone(), handle, true) });
    let device = graphics
        .available_for_surface(Some(handle), DeviceSelectionPolicy::default())
        .ok()?
        .create_device(&graphics)
        .ok()?;
    Some((device, surface))
}

fn chain(device: &Arc<VulkanDevice>, surface: &Arc<Surface>) -> Arc<Swapchain> {
    Arc::new(
        unsafe {
            Swapchain::new(
                device.clone(),
                surface.clone(),
                EXTENT,
                true,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
            )
        }
        .expect("swapchain"),
    )
}

/// 最小化（零尺寸）时重建要跳帧（`Ok(None)`），不能每帧返回硬错误；恢复尺寸后照常取图。
#[test]
fn zero_extent_skips_frames_instead_of_erroring() {
    let Some((device, surface)) = headless() else {
        return;
    };
    let mut chain = chain(&device, &surface);
    let original = Arc::as_ptr(&chain);
    chain.resize(vk::Extent2D::default()).unwrap();
    for _ in 0..2 {
        let frame = Swapchain::validate_swapchain_and_acquire_image(&mut chain)
            .expect("零尺寸应跳帧而非报错");
        assert!(frame.is_none());
        assert_eq!(Arc::as_ptr(&chain), original, "零尺寸不重建交换链");
    }
    chain.resize(EXTENT).unwrap();
    let frame = Swapchain::validate_swapchain_and_acquire_image(&mut chain)
        .expect("恢复尺寸后重建")
        .expect("取得图像");
    assert_ne!(Arc::as_ptr(&chain), original);
    drop(frame);
    device.wait_idle().unwrap();
}

/// `acquire_next_image` 的 timeout 参数必须生效；且阻塞的 acquire 不能持有全局 `queue_lock`。
#[test]
fn acquire_next_image_honors_timeout_and_does_not_take_queue_lock() {
    let Some((device, surface)) = headless() else {
        return;
    };
    let chain = chain(&device, &surface);
    let fence_info = vk::FenceCreateInfo::default();
    let mut fences = Vec::new();
    let mut leases = Vec::new();
    // 不呈现地取尽可取的图像；之后再取只能等待（有限超时合法）。
    let started = Instant::now();
    let exhausted = loop {
        let fence = unsafe { device.device.create_fence(&fence_info, None) }.unwrap();
        match chain.acquire_next_image(0, vk::Semaphore::null(), fence) {
            Ok(lease) => {
                fences.push(fence);
                leases.push(lease);
            }
            Err(error) => {
                unsafe { device.device.destroy_fence(fence, None) };
                break error;
            }
        }
        assert!(leases.len() <= chain.images.len());
    };
    assert!(
        started.elapsed() < Duration::from_millis(1500),
        "timeout = 0 却阻塞了 {:?}（{exhausted:?}）",
        started.elapsed()
    );
    // 持有 queue_lock 时，另一线程的 acquire（timeout = 0）应立即返回而不是等锁。
    let (sender, receiver) = mpsc::channel();
    let worker_chain = chain.clone();
    let worker_device = device.clone();

    let (worker, returned) = device
        .timelines
        .with_queue(device.queue_id(device.queue).unwrap(), || {
            let worker = std::thread::spawn(move || {
                let fence =
                    unsafe { worker_device.device.create_fence(&fence_info, None) }.unwrap();
                let result = worker_chain.acquire_next_image(0, vk::Semaphore::null(), fence);
                let _ = sender.send(());
                let lease = result.ok();
                if lease.is_none() {
                    unsafe { worker_device.device.destroy_fence(fence, None) };
                }
                lease.map(|lease| (fence, lease))
            });
            let returned = receiver.recv_timeout(Duration::from_secs(2)).is_ok();
            Ok((worker, returned))
        })
        .unwrap();
    if let Some((fence, lease)) = worker.join().unwrap() {
        fences.push(fence);
        leases.push(lease);
    }
    assert!(returned, "acquire 在等待 queue_lock");
    // 只剩成功 acquire 的 fence（失败的从未挂起，已销毁）；等其 signal 后再销毁。
    if !fences.is_empty() {
        unsafe { device.device.wait_for_fences(&fences, true, 1_000_000_000) }.unwrap();
    }
    drop(leases);
    device.wait_idle().unwrap();
    for fence in fences {
        unsafe { device.device.destroy_fence(fence, None) };
    }
}

/// 托管包装的 acquire 不能同时给空信号量与空栅栏（VUID-vkAcquireNextImageKHR-semaphore-01780）。
#[test]
fn managed_acquire_rejects_null_semaphore_and_fence() {
    let Some((device, surface)) = headless() else {
        return;
    };
    let inner = rhapsody::raw::managed::swapchain::SwapchainInner::new(chain(&device, &surface));
    let result = inner.acquire_next_image(0, vk::Semaphore::null(), vk::Fence::null());
    assert!(matches!(
        result,
        Err(rhapsody::raw::error::Error::InvalidUsage { .. })
    ));
    device.wait_idle().unwrap();
}
