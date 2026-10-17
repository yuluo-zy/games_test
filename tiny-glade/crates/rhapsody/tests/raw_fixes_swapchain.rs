//! 交换链修复回归（VK_EXT_headless_surface）：零尺寸跳帧、合法首个零超时 acquire 与队列锁隔离。
//! 不通过超额获取来制造耗尽：规范要求当前已获取数超过 S-minImageCount 后不应继续 acquire。
//! MoltenVK 不能由本测试认证“图像耗尽后的实际 timeout”路径，该部分仍未验证。
mod gpu_test_support;
use ash::vk;
use rhapsody::raw::{
    device::Device,
    init::{DeviceSelectionPolicy, GraphicsInstance},
    instance::{GraphicsInstanceLoader, InstanceBuilder},
    physical_device::enumerate_physical_devices,
    swapchain::{Surface, Swapchain, SwapchainPolicy},
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
fn headless() -> Option<(Arc<Device>, Arc<Surface>)> {
    let loader =
        gpu_test_support::initialize("headless surface loader", GraphicsInstanceLoader::new())?;
    if !loader
        .extensions
        .iter()
        .any(|extension| extension == "VK_EXT_headless_surface")
    {
        eprintln!("UNSUPPORTED headless presentation: VK_EXT_headless_surface is unavailable");
        return None;
    }
    let builder = InstanceBuilder {
        extensions: vec![
            CString::new("VK_KHR_surface").unwrap(),
            CString::new("VK_EXT_headless_surface").unwrap(),
        ],
        ..Default::default()
    };
    let instance = gpu_test_support::initialize(
        "headless surface instance",
        builder.build_with_loader(loader),
    )?;
    let devices = gpu_test_support::initialize(
        "headless surface GPU enumeration",
        enumerate_physical_devices(instance.clone()),
    )?
    .into_iter()
    .map(Arc::new)
    .collect();
    let graphics = GraphicsInstance {
        instance: instance.clone(),
        devices,
    };
    let loader = ash::ext::headless_surface::Instance::new(&instance.entry, &instance.instance);
    let handle = gpu_test_support::initialize("headless surface creation", unsafe {
        loader.create_headless_surface(&vk::HeadlessSurfaceCreateInfoEXT::default(), None)
    })?;
    let surface = Arc::new(unsafe { Surface::from_raw(instance.clone(), handle, true) });
    let selected = gpu_test_support::initialize(
        "headless present device selection",
        graphics.available_for_surface(Some(handle), DeviceSelectionPolicy::default()),
    )?;
    let device = gpu_test_support::initialize(
        "headless present logical device",
        selected.create_device(&graphics),
    )?;
    Some((device, surface))
}

fn chain(device: &Arc<Device>, surface: &Arc<Surface>) -> Arc<Swapchain> {
    Arc::new(
        unsafe {
            Swapchain::new_with_policy(
                device.clone(),
                surface.clone(),
                EXTENT,
                true,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
                SwapchainPolicy::AcquireOnly,
            )
        }
        .expect("swapchain"),
    )
}

/// 最小化（零尺寸）时重建要跳帧（`Ok(None)`），不能每帧返回硬错误；恢复尺寸后照常取图。
#[test]
fn zero_extent_skips_frames_instead_of_erroring() {
    let Some((device, surface)) = headless() else {
        eprintln!("SKIP: Vulkan headless presentation environment unavailable");
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

/// A legal first acquire with timeout zero must return without taking the
/// graphics queue lock. This tests rhapsody's forwarding/locking contract,
/// without assuming that every WSI implementation can emulate exhaustion.
#[test]
fn first_zero_timeout_acquire_does_not_take_the_graphics_queue_lock() {
    let Some((device, surface)) = headless() else {
        eprintln!("SKIP: Vulkan headless presentation environment unavailable");
        return;
    };
    let chain = chain(&device, &surface);
    let fence_info = vk::FenceCreateInfo::default();
    // 持有 queue_lock 时，另一线程的 acquire（timeout = 0）应立即返回而不是等锁。
    let (sender, receiver) = mpsc::channel();
    let worker_chain = chain.clone();
    let worker_device = device.clone();

    let (worker, returned) = device
        .timelines
        .with_queue(device.queue_id(device.queue).unwrap(), || {
            let worker = std::thread::spawn(move || {
                let fence = match unsafe { worker_device.device.create_fence(&fence_info, None) } {
                    Ok(fence) => fence,
                    Err(error) => return Err(rhapsody::Error::from(error)),
                };
                let started = Instant::now();
                let result =
                    unsafe { worker_chain.acquire_next_image(0, vk::Semaphore::null(), fence) };
                let elapsed = started.elapsed();
                let _ = sender.send(());
                Ok((fence, result, elapsed))
            });
            let returned = receiver.recv_timeout(Duration::from_secs(2)).is_ok();
            Ok((worker, returned))
        })
        .unwrap();
    let (mut fence_wait, mut acquisition_error, mut duration) = (Ok(()), None, None);
    let worker_error = match worker.join() {
        Ok(Ok((fence, result, elapsed))) => {
            duration = Some(elapsed);
            if result.is_ok() {
                // Successful acquire can return before its fence signals.
                // A first acquisition is within the forward-progress bound.
                fence_wait = unsafe { device.device.wait_for_fences(&[fence], true, u64::MAX) };
            }
            acquisition_error = result.err(); // return a successful lease before teardown
            unsafe { device.device.destroy_fence(fence, None) };
            None
        }
        Ok(Err(error)) => Some(format!("fence creation failed: {error}")),
        Err(_) => Some("acquire worker panicked".into()),
    };
    let idle = device.wait_idle();
    // All successful acquisition signals and test-owned fences are retired
    // before any assertion, including when the queue-lock regression fails.
    assert!(worker_error.is_none(), "{worker_error:?}");
    fence_wait.expect("successful acquire fence");
    idle.unwrap();
    assert!(returned, "acquire 在等待 graphics queue lock");
    assert!(
        duration.unwrap() < Duration::from_millis(1500),
        "zero-timeout acquire blocked: {duration:?}"
    );
    if let Some(error) = acquisition_error {
        assert!(
            matches!(error, rhapsody::Error::Vulkan(vk::Result::NOT_READY)),
            "unexpected acquire error: {error}"
        );
    }
}

/// The canonical acquisition entry rejects absent completion synchronization.
#[test]
fn canonical_acquire_rejects_null_semaphore_and_fence() {
    let Some((device, surface)) = headless() else {
        eprintln!("SKIP: Vulkan headless presentation environment unavailable");
        return;
    };
    let inner = chain(&device, &surface);
    let result = unsafe { inner.acquire_next_image(0, vk::Semaphore::null(), vk::Fence::null()) };
    assert!(matches!(
        result,
        Err(rhapsody::raw::error::Error::InvalidUsage { .. })
    ));
    device.wait_idle().unwrap();
}
