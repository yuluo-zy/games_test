//! 校验层与 debug utils 收集器（wave3）。
//!
//! 本机若没有 `VK_LAYER_KHRONOS_validation`（见 `wave3-coverage.md`），涉及校验层的断言明确跳过并打印原因；
//! 收集器本身（经 `vkSubmitDebugUtilsMessageEXT`，不需要任何层）始终验证。
//! 在有校验层的机器上：`VK_LAYER_PATH=<层所在目录> cargo test -p rhapsody --test wave3_validation -- --nocapture`。
mod draw_support;
mod wave3_support;
use ash::vk;
use draw_support::*;
use rhapsody::rg::parallel::ParallelRecording;
use wave3_support::*;

#[test]
fn the_collector_records_messages_by_severity_without_any_layer() {
    let Some(env) = validated_env() else { return };
    let m = &env.messenger;
    assert!(
        m.collector.all().is_empty(),
        "创建实例/设备后收集器应为空: {:?}",
        m.collector.all()
    );
    m.submit(
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        c"wave3-id",
        c"synthetic error",
    );
    m.submit(
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        c"wave3-id",
        c"synthetic warning",
    );
    m.submit(
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        c"wave3-id",
        c"filtered out (INFO 不在订阅里)",
    );
    let errors = m.collector.errors();
    let warnings = m.collector.warnings();
    assert_eq!(
        (errors.len(), warnings.len()),
        (1, 1),
        "{:?}",
        m.collector.all()
    );
    assert_eq!(errors[0].message, "synthetic error");
    assert_eq!(errors[0].id_name, "wave3-id");
    assert_eq!(warnings[0].message, "synthetic warning");
    m.collector.clear();
    assert!(m.collector.all().is_empty());
}

#[test]
fn the_builder_enables_the_validation_layer_only_when_it_exists() {
    let mut builder = rhapsody::raw::instance::InstanceBuilder::default();
    let Ok(enabled) = builder.enable_validation() else {
        return;
    };
    let present = rhapsody::raw::instance::GraphicsInstanceLoader::new()
        .map(|l| {
            l.layers
                .iter()
                .any(|l| l.name == rhapsody::raw::instance::KHRONOS_VALIDATION_LAYER)
        })
        .unwrap_or(false);
    assert_eq!(enabled, present);
    assert_eq!(
        builder
            .layers
            .iter()
            .any(|l| l.to_str() == Ok(rhapsody::raw::instance::KHRONOS_VALIDATION_LAYER)),
        present
    );
    assert!(
        builder
            .extensions
            .iter()
            .any(|e| e.to_str() == Ok("VK_EXT_debug_utils"))
    );
    if !present {
        println!("跳过校验层断言：无 VK_LAYER_KHRONOS_validation");
    }
}

#[test]
fn deliberate_api_misuse_produces_a_validation_error_when_the_layer_exists() {
    let Some(env) = validated_env() else { return };
    if !env.layer_enabled {
        println!("跳过：无校验层（VK_LAYER_KHRONOS_validation 不可用），无法验证“误用必产生消息”");
        return;
    }
    // vkCreateBuffer size = 0 违反 VUID-VkBufferCreateInfo-size-00912。
    let info = vk::BufferCreateInfo::default()
        .size(0)
        .usage(vk::BufferUsageFlags::TRANSFER_DST);
    let result = unsafe { env.device.raw_device().create_buffer(&info, None) };
    if let Ok(buffer) = result {
        unsafe { env.device.raw_device().destroy_buffer(buffer, None) };
    }
    assert!(
        !env.messenger.collector.errors().is_empty(),
        "校验层必须报告 size=0 的误用"
    );
}

#[test]
fn the_animation_and_the_big_graph_run_without_validation_errors() {
    let Some(env) = validated_env() else { return };
    if !env.layer_enabled {
        println!("跳过校验断言：无校验层；仍执行流程以确认其在该设备上正常");
    }
    // 直接录制的三角形动画（12 帧）。
    let rig = rig_on(env.device.clone());
    let target = rig.new_target("validated target");
    for frame in 0..12 {
        rig.draw(
            &target,
            &rotated_triangle(frame as f32 * 15f32.to_radians()),
            frame_clear_color(frame),
        )
        .wait(u64::MAX)
        .unwrap();
        check_animation_frame(frame, 15.0, &read_pixels(&target));
    }
    // 渲染图大图（32 个填充 + 8 个拷贝 pass），串行与并行各一次，第二帧并行带历史。
    for parallel in [ParallelRecording::PARALLEL, ParallelRecording::EVERY_PASS] {
        let mut session = new_session(&env.device, parallel);
        let bufs = BigBuffers::new(&mut session, 32, 8);
        for salt in 1..=2u32 {
            let run = run_big_graph(&mut session, &bufs, salt, false);
            assert_eq!(run.buffers, expected_buffers(32, 8, salt));
        }
    }
    rig.device.wait_idle().unwrap();
    let errors = env.messenger.collector.errors();
    if env.layer_enabled {
        assert!(
            errors.is_empty(),
            "校验层报告了 {} 条 error: {:#?}",
            errors.len(),
            errors
        );
        println!(
            "校验层启用：0 条 error，warning {} 条",
            env.messenger.collector.warnings().len()
        );
    } else {
        println!(
            "无校验层：收集到 {} 条 error（仅 loader 自身可能产生）",
            errors.len()
        );
    }
}
