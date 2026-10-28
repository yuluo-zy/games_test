//! 交换链修复回归（VK_EXT_headless_surface）：零尺寸跳帧、合法首个零超时 acquire 与队列锁隔离。
//! 不通过超额获取来制造耗尽：规范要求当前已获取数超过 S-minImageCount 后不应继续 acquire。
//! MoltenVK 不能由本测试认证“图像耗尽后的实际 timeout”路径，该部分仍未验证。
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::device::Device;
use rhapsody::present::swapchain::{Swapchain, SwapchainPolicy};
use rhapsody::native::surface::Surface;
use rhapsody::native::instance::{GraphicsInstanceLoader, InstanceBuilder};
use rhapsody::rg::{
    frame_loop::{FrameLoop, FrameStatus},
    graph::{BuiltGraph, RenderGraph},
    vulkan::{
        AccessTable, PresentTarget, SwapchainContents, VulkanGraphBackend, VulkanGraphSession,
    },
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
    headless_with(&[])
}

fn headless_with(optional: &[&str]) -> Option<(Arc<Device>, Arc<Surface>)> {
    let (device, surface, _) = headless_parts(optional)?;
    Some((device, surface))
}

fn create_headless_surface(
    instance: &Arc<rhapsody::native::instance::VulkanInstance>,
) -> Arc<Surface> {
    let loader =
        ash::ext::headless_surface::Instance::new(&instance.entry, instance.raw_instance());
    let handle = unsafe {
        loader.create_headless_surface(&vk::HeadlessSurfaceCreateInfoEXT::default(), None)
    }
    .expect("headless surface");
    Arc::new(unsafe { Surface::from_raw(instance.clone(), handle, true) })
}

fn headless_parts(
    optional: &[&str],
) -> Option<(
    Arc<Device>,
    Arc<Surface>,
    Arc<rhapsody::native::instance::VulkanInstance>,
)> {
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
    let mut extensions = vec![
        CString::new("VK_KHR_surface").unwrap(),
        CString::new("VK_EXT_headless_surface").unwrap(),
    ];
    for name in optional {
        if !loader.extensions.iter().any(|extension| extension == name) {
            eprintln!("UNSUPPORTED headless presentation: {name} is unavailable");
            return None;
        }
        extensions.push(CString::new(*name).unwrap());
    }
    let builder = InstanceBuilder {
        extensions,
        ..Default::default()
    };
    let instance = gpu_test_support::initialize(
        "headless surface instance",
        builder.build_with_loader(loader),
    )?;
    let loader =
        ash::ext::headless_surface::Instance::new(&instance.entry, instance.raw_instance());
    let handle = gpu_test_support::initialize("headless surface creation", unsafe {
        loader.create_headless_surface(&vk::HeadlessSurfaceCreateInfoEXT::default(), None)
    })?;
    let surface = Arc::new(unsafe { Surface::from_raw(instance.clone(), handle, true) });
    let device = gpu_test_support::initialize(
        "headless present logical device",
        Device::new(&instance, Some(&surface), &rhapsody::device::DeviceConfig::default()),
    )?;
    Some((device, surface, instance))
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
        .timelines()
        .with_queue(device.queue_id(device.primary_queue()).unwrap(), || {
            let worker = std::thread::spawn(move || {
                let fence =
                    match unsafe { worker_device.raw_device().create_fence(&fence_info, None) } {
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
                fence_wait = unsafe {
                    device
                        .raw_device()
                        .wait_for_fences(&[fence], true, u64::MAX)
                };
            }
            acquisition_error = result.err(); // return a successful lease before teardown
            unsafe { device.raw_device().destroy_fence(fence, None) };
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
    assert!(matches!(result, Err(rhapsody::Error::InvalidUsage { .. })));
    device.wait_idle().unwrap();
}

const TIMEOUT: u64 = 5_000_000_000;
const USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::from_raw(
    vk::ImageUsageFlags::COLOR_ATTACHMENT.as_raw() | vk::ImageUsageFlags::TRANSFER_DST.as_raw(),
);
/// Transfer-write access code (TRANSFER_DST_OPTIMAL).
const TRANSFER_WRITE: u8 = 0x34;

/// A strict FrameLoop on a headless surface; None when unsupported.
fn strict_frame_loop() -> Option<(FrameLoop, Arc<rhapsody::native::instance::VulkanInstance>)> {
    strict_frame_loop_with(USAGE)
}
fn strict_frame_loop_with(
    usage: vk::ImageUsageFlags,
) -> Option<(FrameLoop, Arc<rhapsody::native::instance::VulkanInstance>)> {
    let (device, surface, instance) = headless_parts(&[
        "VK_KHR_get_surface_capabilities2",
        "VK_EXT_surface_maintenance1",
    ])?;
    if !device
        .caps()
        .is_enabled(rhapsody::device::Feature::SwapchainMaintenance1)
    {
        eprintln!("UNSUPPORTED strict presentation: swapchainMaintenance1 is unavailable");
        return None;
    }
    let chain = Arc::new(
        unsafe { Swapchain::new(device.clone(), surface, EXTENT, true, usage) }.expect("swapchain"),
    );
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).expect("backend");
    let frame_loop = FrameLoop::new(VulkanGraphSession::new(backend), chain).expect("frame loop");
    Some((frame_loop, instance))
}

/// A graph whose only pass records a side effect, plus a retire callback.
fn side_effect_graph(
    frame_loop: &mut FrameLoop,
    recorded: &Arc<AtomicBool>,
    retired: &std::sync::Arc<RetireCount>,
) -> BuiltGraph {
    let mut graph = frame_loop.begin_frame().expect("begin frame");
    let recorded = recorded.clone();
    graph
        .add_pass("side effect")
        .with_side_effect()
        .render_with_context(move |_| {
            recorded.store(true, Ordering::Relaxed);
            true
        });
    let retired = retired.clone();
    graph.on_retire(move || retired.set(retired.get() + 1));
    graph.build()
}

/// One graph pass clearing the swapchain image; the library owns the
/// acquire wait, both layout transitions and the present signal.
fn clear_pass(
    graph: &mut RenderGraph,
    target: &PresentTarget,
    calls: &Arc<AtomicUsize>,
    color: f32,
) {
    let calls = calls.clone();
    let image = target.image();
    let device = target.texture().device().clone();
    // A later full overwrite would otherwise cull this pass as a dead write.
    let mut pass = graph.add_pass("clear swapchain").with_side_effect();
    pass.add_resource_access(target.resource(), TRANSFER_WRITE, 0)
        .unwrap();
    pass.render_with_context(move |ctx| {
        calls.fetch_add(1, Ordering::Relaxed);
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        unsafe {
            device.raw_device().cmd_clear_color_image(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue {
                    float32: [color, 0.4, 0.6, 1.0],
                },
                &[range],
            );
        }
        true
    });
}

fn clear_presentation(
    calls: &Arc<AtomicUsize>,
) -> impl FnOnce(&mut RenderGraph, &PresentTarget) -> rhapsody::Result<()> {
    let calls = calls.clone();
    move |graph, target| {
        clear_pass(graph, target, &calls, 0.2);
        Ok(())
    }
}

/// Where the presentation semaphores were injected into the submitted batches.
struct InjectedSync {
    submits: usize,
    acquire_waits: Vec<(usize, vk::PipelineStageFlags)>,
    rendered_signals: Vec<usize>,
}
fn injected_sync(
    session: &VulkanGraphSession,
    acquire: vk::Semaphore,
    rendered: vk::Semaphore,
) -> InjectedSync {
    let captures = &session.backend.submissions;
    let mut sync = InjectedSync {
        submits: captures.len(),
        acquire_waits: Vec::new(),
        rendered_signals: Vec::new(),
    };
    for (index, capture) in captures.iter().enumerate() {
        assert_eq!(capture.result, 0, "submit {index} succeeded");
        for batch in &capture.batches {
            for (position, &semaphore) in batch.wait_semaphores.iter().enumerate() {
                if semaphore == acquire {
                    sync.acquire_waits
                        .push((index, batch.wait_stages[position]));
                }
            }
            if batch.signal_semaphores.contains(&rendered) {
                sync.rendered_signals.push(index);
            }
            assert!(!batch.signal_semaphores.contains(&acquire));
            assert!(!batch.wait_semaphores.contains(&rendered));
        }
    }
    sync
}
fn present_transition_count(session: &VulkanGraphSession) -> usize {
    session
        .last_synchronization()
        .transitions()
        .iter()
        .filter(|transition| transition.new_layout == vk::ImageLayout::PRESENT_SRC_KHR)
        .count()
}

/// Step-by-step frame: main graph, acquire, presentation graph, present.
/// Returns the injected synchronization of the presentation graph.
fn present_step_by_step(
    frame_loop: &mut FrameLoop,
    build: impl FnOnce(&mut RenderGraph, &PresentTarget),
) -> InjectedSync {
    let recorded = Arc::new(AtomicBool::new(false));
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let main = side_effect_graph(frame_loop, &recorded, &retired);
    frame_loop.execute_graph(main).expect("main graph");
    let lease = frame_loop.acquire().expect("acquire").expect("image");
    let (acquire, rendered) = (lease.acquire_semaphore(), lease.render_semaphore());
    let mut graph = frame_loop.graph.begin_frame().expect("presentation graph");
    let target = frame_loop
        .graph
        .import_swapchain_image(&mut graph, lease, SwapchainContents::Discard)
        .expect("import swapchain image");
    build(&mut graph, &target);
    frame_loop
        .graph
        .execute_frame(graph.build())
        .expect("presentation graph executes");
    let sync = injected_sync(&frame_loop.graph, acquire, rendered);
    assert_eq!(
        present_transition_count(&frame_loop.graph),
        1,
        "one transition to PRESENT_SRC_KHR"
    );
    let receipt = frame_loop.present(&target).expect("present");
    frame_loop.wait_render(receipt, TIMEOUT).unwrap();
    sync
}

/// (1) A single-batch presentation graph waits on acquire and signals the
/// present wait in that one submit, with the PRESENT_SRC_KHR transition.
#[test]
fn single_batch_presentation_graph_waits_acquire_and_signals_rendered_once() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let calls = Arc::new(AtomicUsize::new(0));
    for _ in 0..4 {
        let sync = present_step_by_step(&mut frame_loop, |graph, target| {
            clear_pass(graph, target, &calls, 0.3)
        });
        assert_eq!(sync.submits, 1);
        assert_eq!(
            sync.acquire_waits,
            vec![(0, vk::PipelineStageFlags::TRANSFER)]
        );
        assert_eq!(sync.rendered_signals, vec![0]);
    }
    assert_eq!(calls.load(Ordering::Relaxed), 4);
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// (2) A presentation graph split into several batches: the acquire wait sits
/// on the first batch that touches the image (not the first batch), at its
/// first-use stage, and the present wait is signalled by the last one.
#[test]
fn multi_batch_presentation_graph_waits_on_first_use_and_signals_from_last_use() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let prologue = Arc::new(AtomicUsize::new(0));
    for frame in 0..5 {
        let sync = present_step_by_step(&mut frame_loop, |graph, target| {
            graph.batch_per_pass(true);
            let prologue = prologue.clone();
            graph
                .add_pass("prologue without the swapchain image")
                .with_side_effect()
                .render_with_context(move |_| {
                    prologue.fetch_add(1, Ordering::Relaxed);
                    true
                });
            clear_pass(graph, target, &calls, 0.1);
            clear_pass(graph, target, &calls, 0.9);
        });
        assert_eq!(sync.submits, 3, "frame {frame}: one submit per batch");
        assert_eq!(
            sync.acquire_waits,
            vec![(1, vk::PipelineStageFlags::TRANSFER)],
            "frame {frame}: acquire waits on the first batch using the image"
        );
        assert_eq!(
            sync.rendered_signals,
            vec![2],
            "frame {frame}: rendered from the last use"
        );
    }
    assert_eq!(prologue.load(Ordering::Relaxed), 5);
    assert_eq!(calls.load(Ordering::Relaxed), 10);
    // The convenience path presents multi-batch graphs too.
    for _ in 0..3 {
        let recorded = Arc::new(AtomicBool::new(false));
        let retired = std::sync::Arc::new(RetireCount::new(0));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let calls = calls.clone();
        let outcome = frame_loop
            .execute(built, move |graph, target| {
                graph.batch_per_pass(true);
                clear_pass(graph, target, &calls, 0.5);
                clear_pass(graph, target, &calls, 0.7);
                Ok(())
            })
            .expect("multi-batch frame");
        assert_eq!(outcome.status, FrameStatus::Presented);
        assert_eq!(outcome.presentation_graph.unwrap().receipts.len(), 2);
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// (2b) Passes ordered before the first use of the image share no batch with
/// it, so the acquire semaphore only blocks the batch that needs the image.
/// The boundary is derived from the plan (no `batch_per_pass`), so repeated
/// frames hit the plan cache; a graph without such passes stays one batch.
#[test]
fn independent_passes_before_the_image_are_not_blocked_by_the_acquire_wait() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let prologue = Arc::new(AtomicUsize::new(0));
    for frame in 0..4 {
        let recorded = Arc::new(AtomicBool::new(false));
        let retired = std::sync::Arc::new(RetireCount::new(0));
        let main = side_effect_graph(&mut frame_loop, &recorded, &retired);
        frame_loop.execute_graph(main).expect("main graph");
        let lease = frame_loop.acquire().expect("acquire").expect("image");
        let (acquire, rendered) = (lease.acquire_semaphore(), lease.render_semaphore());
        let mut graph = frame_loop.graph.begin_frame().expect("presentation graph");
        let target = frame_loop
            .graph
            .import_swapchain_image(&mut graph, lease, SwapchainContents::Discard)
            .expect("import");
        for _ in 0..2 {
            let prologue = prologue.clone();
            graph
                .add_pass("compute without the swapchain image")
                .with_side_effect()
                .render_with_context(move |_| {
                    prologue.fetch_add(1, Ordering::Relaxed);
                    true
                });
        }
        clear_pass(&mut graph, &target, &calls, 0.1);
        clear_pass(&mut graph, &target, &calls, 0.8);
        let receipt = frame_loop
            .graph
            .execute_frame(graph.build())
            .expect("presentation graph executes");
        let sync = injected_sync(&frame_loop.graph, acquire, rendered);
        assert_eq!(
            sync.submits, 2,
            "frame {frame}: prologue batch + image batch"
        );
        assert_eq!(receipt.stats.batches, 2, "frame {frame}");
        assert_eq!(
            sync.acquire_waits,
            vec![(1, vk::PipelineStageFlags::TRANSFER)],
            "frame {frame}: the prologue batch carries no acquire wait"
        );
        assert_eq!(sync.rendered_signals, vec![1], "frame {frame}");
        assert_eq!(receipt.stats.plan_cache_hit, frame > 0, "frame {frame}");
        let presented = frame_loop.present(&target).expect("present");
        frame_loop.wait_render(presented, TIMEOUT).unwrap();
    }
    assert_eq!(prologue.load(Ordering::Relaxed), 8);
    assert_eq!(calls.load(Ordering::Relaxed), 8);
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

#[test]
fn strict_frame_loop_presents_through_the_presentation_graph() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let recorded = Arc::new(AtomicBool::new(false));
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    let generation = frame_loop.generation();
    for frame in 0..6 {
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .expect("frame");
        assert_eq!(outcome.status, FrameStatus::Presented, "frame {frame}");
        assert!(outcome.presentation.is_some());
        assert_eq!(outcome.generation, generation);
    }
    assert!(recorded.load(Ordering::Relaxed));
    assert_eq!(displays.load(Ordering::Relaxed), 6);
    assert!(!frame_loop.graph.has_presentation());
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// (4) A skipped acquisition (minimised window) still runs the main graph.
#[test]
fn skipped_presentation_still_executes_and_retires_the_graph() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    frame_loop.resize(vk::Extent2D::default()).unwrap();
    for _ in 0..3 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .expect("minimised frame");
        assert_eq!(outcome.status, FrameStatus::Skipped);
        assert!(outcome.presentation.is_none());
        assert!(outcome.presentation_graph.is_none());
        assert!(
            recorded.load(Ordering::Relaxed),
            "a skipped presentation must not drop the graph"
        );
        assert!(
            !outcome.graph.receipts.is_empty(),
            "the graph reached the GPU"
        );
    }
    assert_eq!(displays.load(Ordering::Relaxed), 0);
    frame_loop.graph.wait_submits_done(TIMEOUT).unwrap();
    // Restoring the size rebuilds and presents again.
    frame_loop.resize(EXTENT).unwrap();
    let recorded = Arc::new(AtomicBool::new(false));
    let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
    let outcome = frame_loop
        .execute(built, clear_presentation(&displays))
        .expect("restored frame");
    assert!(outcome.presentation.is_some());
    frame_loop.wait_idle(TIMEOUT).unwrap();
    // Retirement runs when the session collects completed frames.
    for _ in 0..4 {
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        frame_loop
            .execute(built, clear_presentation(&displays))
            .unwrap();
        frame_loop.wait_idle(TIMEOUT).unwrap();
    }
    assert!(
        retired.get() >= 4,
        "skipped frames retire too: {}",
        retired.get()
    );
    frame_loop.shutdown(TIMEOUT).unwrap();
}

#[test]
fn windowed_focus_loss_keeps_presenting() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    frame_loop.set_focused(false).unwrap();
    let recorded = Arc::new(AtomicBool::new(false));
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    for _ in 0..3 {
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .unwrap();
        assert_eq!(outcome.status, FrameStatus::Presented);
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// (3) A presentation graph that fails after its first batch consumed the
/// acquire wait returns the image (discard + present, no second acquire
/// wait), keeps the main graph, and the next frames present normally. Failing
/// more often than there are images proves every image comes back.
#[test]
fn presentation_graph_failure_after_acquire_returns_the_image_and_recovers() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    for _ in 0..8 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let calls = displays.clone();
        let error = frame_loop
            .execute(built, move |graph, target| {
                graph.batch_per_pass(true);
                clear_pass(graph, target, &calls, 0.4);
                let mut failing = graph.add_pass("failing overlay");
                failing.add_resource_access(target.resource(), TRANSFER_WRITE, 0)?;
                failing.render_with_context(|_| false);
                Ok(())
            })
            .unwrap_err();
        assert_ne!(error.kind(), rhapsody::ErrorKind::DeviceLost, "{error}");
        assert!(
            recorded.load(Ordering::Relaxed),
            "the main graph was submitted first"
        );
        assert!(
            !frame_loop.graph.has_presentation(),
            "the image was returned"
        );
    }
    assert_eq!(
        displays.load(Ordering::Relaxed),
        8,
        "the first batch ran every time"
    );
    for _ in 0..3 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .unwrap();
        assert_eq!(outcome.status, FrameStatus::Presented);
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// A presentation build failure before anything was submitted returns the
/// image under the default `SkipPresent` policy: a real release where the
/// driver supports it, otherwise (Mesa's headless WSI crashes in
/// vkReleaseSwapchainImagesEXT, see `Swapchain::can_release_images`) a submit
/// that consumes the acquire and presents a cleared black frame.
#[test]
fn presentation_build_failure_returns_the_image_and_recovers() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    // Fail more often than there are images: each failure must return its
    // acquisition, otherwise acquire would eventually run out of images.
    for _ in 0..8 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let error = frame_loop
            .execute(built, |_, _| {
                Err(rhapsody::Error::invalid_usage(
                    "injected presentation failure",
                ))
            })
            .unwrap_err();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        assert!(
            recorded.load(Ordering::Relaxed),
            "the graph was submitted first"
        );
    }
    let recorded = Arc::new(AtomicBool::new(false));
    let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
    let outcome = frame_loop
        .execute(built, clear_presentation(&displays))
        .unwrap();
    assert!(outcome.presentation.is_some());
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

const READBACK_USAGE: vk::ImageUsageFlags =
    vk::ImageUsageFlags::from_raw(USAGE.as_raw() | vk::ImageUsageFlags::TRANSFER_SRC.as_raw());
/// Transfer-read access code (TRANSFER_SRC_OPTIMAL).
const TRANSFER_READ: u8 = 0x25;

/// Run frames until image `wanted` comes around again, then read its first
/// pixel back through a `Preserve` presentation graph. Every other image is
/// presented with an ordinary clear. Returns the pixel as RGBA bytes.
fn read_back_image(frame_loop: &mut FrameLoop, wanted: u32) -> [u8; 4] {
    frame_loop.set_swapchain_contents(SwapchainContents::Preserve);
    let device = frame_loop.device().clone();
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let calls = Arc::new(AtomicUsize::new(0));
    for _ in 0..16 {
        let recorded = Arc::new(AtomicBool::new(false));
        let main = side_effect_graph(frame_loop, &recorded, &retired);
        frame_loop.execute_graph(main).expect("main graph");
        let lease = frame_loop.acquire().expect("acquire").expect("image");
        let format = lease.format();
        let mut graph = frame_loop.graph.begin_frame().expect("graph");
        let target = frame_loop
            .graph
            .import_swapchain_image(&mut graph, lease, SwapchainContents::Preserve)
            .expect("import");
        if target.index() != wanted {
            clear_pass(&mut graph, &target, &calls, 0.0);
            frame_loop.graph.execute_frame(graph.build()).unwrap();
            let receipt = frame_loop.present(&target).unwrap();
            frame_loop.wait_render(receipt, TIMEOUT).unwrap();
            continue;
        }
        let readback = Arc::new(
            rhapsody::resource::buffer::Buffer::new(
                device.clone(),
                &rhapsody::resource::buffer::BufferDesc::new_gpu_to_cpu(
                    256,
                    vk::BufferUsageFlags::TRANSFER_DST,
                ),
            )
            .unwrap(),
        );
        let destination = frame_loop
            .graph
            .import_frame_buffer(&mut graph, readback.clone(), 0)
            .unwrap();
        let (image, raw_buffer, raw_device) = (
            target.image(),
            readback.raw_handle(),
            device.raw_device().clone(),
        );
        let mut pass = graph.add_pass("read back").with_side_effect();
        pass.add_resource_access(target.resource(), TRANSFER_READ, 0)
            .unwrap();
        pass.add_resource_access(destination, TRANSFER_WRITE, 0)
            .unwrap();
        pass.render_with_context(move |ctx| {
            let region = vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width: 1,
                    height: 1,
                    depth: 1,
                });
            unsafe {
                raw_device.cmd_copy_image_to_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    image,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    raw_buffer,
                    &[region],
                );
            }
            true
        });
        frame_loop.graph.execute_frame(graph.build()).unwrap();
        let receipt = frame_loop.present(&target).unwrap();
        frame_loop.wait_render(receipt, TIMEOUT).unwrap();
        let mut pixel = [0u8; 4];
        readback.read_bytes(0, &mut pixel).unwrap();
        let mut pixel = match format {
            vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB => {
                [pixel[2], pixel[1], pixel[0], pixel[3]]
            }
            _ => pixel,
        };
        // vkCmdClearColorImage encodes linear RGB when the native image is sRGB.
        if matches!(
            format,
            vk::Format::B8G8R8A8_SRGB | vk::Format::R8G8B8A8_SRGB
        ) {
            for channel in &mut pixel[..3] {
                let encoded = f32::from(*channel) / 255.0;
                let linear = if encoded <= 0.04045 {
                    encoded / 12.92
                } else {
                    ((encoded + 0.055) / 1.055).powf(2.4)
                };
                *channel = (linear * 255.0).round() as u8;
            }
        }
        return pixel;
    }
    panic!("image {wanted} never came around again");
}

fn near(actual: [u8; 4], expected: [f32; 4]) {
    for (channel, (&got, want)) in actual.iter().zip(expected).enumerate() {
        let want = (want * 255.0).round() as i32;
        assert!(
            (i32::from(got) - want).abs() <= 2,
            "channel {channel}: got {got}, expected about {want} ({actual:?})"
        );
    }
}

/// Fail a presentation graph after its first batch consumed the acquire and
/// return the index of the abandoned image.
fn fail_after_first_batch(frame_loop: &mut FrameLoop, abandoned: &mut Option<u32>) {
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let calls = Arc::new(AtomicUsize::new(0));
    let recorded = Arc::new(AtomicBool::new(false));
    let built = side_effect_graph(frame_loop, &recorded, &retired);
    let index = std::rc::Rc::new(std::cell::Cell::new(None));
    let seen = index.clone();
    let error = frame_loop
        .execute(built, move |graph, target| {
            seen.set(Some(target.index()));
            graph.batch_per_pass(true);
            clear_pass(graph, target, &calls, 0.9);
            let mut failing = graph.add_pass("failing overlay");
            failing.add_resource_access(target.resource(), TRANSFER_WRITE, 0)?;
            failing.render_with_context(|_| false);
            Ok(())
        })
        .unwrap_err();
    assert_ne!(error.kind(), rhapsody::ErrorKind::DeviceLost, "{error}");
    assert!(
        !frame_loop.graph.has_presentation(),
        "the image was returned"
    );
    *abandoned = index.get();
}

/// (4) `ClearTo`: after a partial failure the image that reaches the screen
/// holds the configured colour, never the undefined memory the discarded
/// first batch left behind.
#[test]
fn partial_failure_presents_the_configured_clear_colour() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop_with(READBACK_USAGE) else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let color = [0.25, 0.5, 0.75, 1.0];
    frame_loop
        .set_on_failure(rhapsody::present::PresentPolicy::ClearTo(color))
        .unwrap();
    assert_eq!(
        frame_loop.on_failure(),
        rhapsody::present::PresentPolicy::ClearTo(color)
    );
    let mut abandoned = None;
    fail_after_first_batch(&mut frame_loop, &mut abandoned);
    near(
        read_back_image(&mut frame_loop, abandoned.expect("image index")),
        color,
    );
    // Fail more often than there are images: every image comes back.
    for _ in 0..8 {
        fail_after_first_batch(&mut frame_loop, &mut abandoned);
    }
    near(
        read_back_image(&mut frame_loop, abandoned.expect("image index")),
        color,
    );
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// (5) `SkipPresent` (the default): the failed frame never shows the graph's
/// partial output. Where the release call is unusable (Mesa headless) the
/// acquire is still consumed by a real submit and an opaque black frame
/// replaces it; the next frames present normally either way.
#[test]
fn skip_present_never_presents_undefined_content_and_next_frames_present() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop_with(READBACK_USAGE) else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    assert_eq!(
        frame_loop.on_failure(),
        rhapsody::present::PresentPolicy::SkipPresent
    );
    let releasable = frame_loop.swapchain().can_release_images();
    let mut abandoned = None;
    for _ in 0..8 {
        fail_after_first_batch(&mut frame_loop, &mut abandoned);
    }
    let displays = Arc::new(AtomicUsize::new(0));
    let retired = std::sync::Arc::new(RetireCount::new(0));
    for _ in 0..3 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .unwrap();
        assert_eq!(outcome.status, FrameStatus::Presented);
    }
    if !releasable {
        fail_after_first_batch(&mut frame_loop, &mut abandoned);
        near(
            read_back_image(&mut frame_loop, abandoned.expect("image index")),
            rhapsody::present::PresentPolicy::FALLBACK_COLOR,
        );
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// `PreviousFrame` needs a retained copy of presented frames, which the
/// swapchain does not keep: it is rejected and changes nothing.
#[test]
fn previous_frame_policy_is_unsupported() {
    let Some((mut frame_loop, _instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let error = frame_loop
        .set_on_failure(rhapsody::present::PresentPolicy::PreviousFrame)
        .unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::Unsupported, "{error}");
    assert_eq!(
        frame_loop.on_failure(),
        rhapsody::present::PresentPolicy::SkipPresent
    );
    let error = frame_loop
        .set_on_failure(rhapsody::present::PresentPolicy::ClearTo([
            f32::NAN,
            0.0,
            0.0,
            1.0,
        ]))
        .unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
}

/// `ClearTo` on a swapchain whose usage lacks TRANSFER_DST cannot clear; it
/// degrades to `SkipPresent` and the image still comes back.
#[test]
fn clear_policy_without_transfer_dst_degrades_to_skip_present() {
    let Some((mut frame_loop, _instance)) =
        strict_frame_loop_with(vk::ImageUsageFlags::COLOR_ATTACHMENT)
    else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    if !frame_loop.swapchain().can_release_images() {
        eprintln!("SKIP: no release path and no TRANSFER_DST: only undefined content possible");
        return;
    }
    frame_loop
        .set_on_failure(rhapsody::present::PresentPolicy::ClearTo([1.0; 4]))
        .unwrap();
    let retired = std::sync::Arc::new(RetireCount::new(0));
    for _ in 0..6 {
        let recorded = Arc::new(AtomicBool::new(false));
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let error = frame_loop
            .execute(built, |_, _| {
                Err(rhapsody::Error::invalid_usage("injected"))
            })
            .unwrap_err();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

#[test]
fn replacing_the_surface_starts_a_new_generation() {
    let Some((mut frame_loop, instance)) = strict_frame_loop() else {
        eprintln!("SKIP: strict headless presentation unavailable");
        return;
    };
    let recorded = Arc::new(AtomicBool::new(false));
    let retired = std::sync::Arc::new(RetireCount::new(0));
    let displays = Arc::new(AtomicUsize::new(0));
    let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
    frame_loop
        .execute(built, clear_presentation(&displays))
        .unwrap();
    let before = frame_loop.generation();
    frame_loop
        .replace_surface(create_headless_surface(&instance))
        .unwrap();
    assert_ne!(frame_loop.generation(), before);
    assert!(!frame_loop.swapchain().surface.is_lost());
    for _ in 0..3 {
        let built = side_effect_graph(&mut frame_loop, &recorded, &retired);
        let outcome = frame_loop
            .execute(built, clear_presentation(&displays))
            .unwrap();
        assert!(outcome.presentation.is_some());
    }
    frame_loop.wait_idle(TIMEOUT).unwrap();
    frame_loop.shutdown(TIMEOUT).unwrap();
}

/// `Cell<usize>`-like counter that retire callbacks (now `Send`) may capture.
struct RetireCount(std::sync::atomic::AtomicUsize);
impl RetireCount {
    fn new(value: usize) -> Self {
        Self(std::sync::atomic::AtomicUsize::new(value))
    }
    fn get(&self) -> usize {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
    fn set(&self, value: usize) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed)
    }
}
