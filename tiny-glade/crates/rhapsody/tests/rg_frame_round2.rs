//! rg 第二轮：帧账本 / PassPgoNameId 驻留 / `Option` 形态的 PGO 查询（对应 `reports/rg-round2.md`）。
use rhapsody::rg::executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId, PassPgoStats};
use rhapsody::rg::frame::{self, FrameStage};

#[test]
fn pass_pgo_name_id_is_pointer_identity_of_an_interned_name() {
    let owned = String::from("Shadows");
    let a = PassPgoNameId::from_name(&owned);
    let b = PassPgoNameId::from_name(&format!("Shad{}", "ows"));
    assert_eq!(a, b, "同名跨来源必须同键");
    assert_eq!(a, PassPgoNameId::from_static("Shadows"));
    assert_ne!(a, PassPgoNameId::from_name("Shadow"));
    // 指针恒等：值就是驻留字符串的数据指针，反复获取不变
    for _ in 0..8 {
        assert_eq!(PassPgoNameId::from_name("Shadows").0, a.0);
    }
}

#[test]
fn pgo_lookup_has_the_original_option_shape_and_frames_swap() {
    let id = PassPgoNameId::from_name("main_pass");
    let mut frames = PassPgoFrames::default();
    assert_eq!(frames.previous.get(id), None);
    assert_eq!(frames.previous.duration_ns(id), 0, "未命中 unwrap_or(0)");
    frames.current.record(PassPgoDuration::new(id, 700));
    frames.current.record(PassPgoDuration::new(id, 300));
    frames.end_frame();
    assert_eq!(frames.previous.get(id), Some(300), "insert 覆盖：同名取最后一次");
    assert_eq!(frames.previous.total_ns(), 1000);
    assert_eq!(frames.previous.max_ns(), 700);
    assert!(frames.current.is_empty(), "新的本帧被清空");
    assert_eq!(frames.current.total_ns(), 0);
    let mut stats = PassPgoStats::default();
    stats.extend([PassPgoDuration::new(id, 1000)]);
    assert_eq!(stats.get(id), Some(1000));
    stats.reset();
    assert_eq!(stats.get(id), None);
}

#[test]
fn render_loop_order_matches_the_original_call_sequence() {
    use FrameStage::*;
    let order = [Compile, BeginFrame, ExecuteMain, Retire, Acquire, ExecutePresentation, EndFrame, Present, WaitGpuQueueTrim];
    assert!(order.windows(2).all(|w| w[0] < w[1]));
    let mut seq = frame::FrameSequence::new();
    for stage in order {
        if stage == ExecutePresentation {
            seq.set_acquired(true);
        }
        seq.advance(stage).unwrap();
    }
}

/// 空图在 lavapipe 上走完 begin → execute → end：账本记到提交，GPU 时间戳时长只在 < 1s 时给出。
#[test]
fn session_ledger_counts_the_submission_and_gpu_duration_is_option() {
    use rhapsody::raw::init::create_headless_device;
    use rhapsody::rg::vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession};
    let Some(device) = create_headless_device().ok() else { return };
    let backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    assert_eq!(session.last_known_gpu_frame_duration_ns(), None);
    let mut graph = session.begin_frame().unwrap();
    assert_eq!(session.frame_ledger().submissions(), 0);
    graph.add_pass("noop").render(|_| true);
    session.execute_frame(graph.build()).unwrap();
    assert!(session.frame_ledger().has_ended());
    assert!(session.frame_ledger().submissions() >= 1);
    session.wait_submits_done(10_000_000_000).unwrap();
}
