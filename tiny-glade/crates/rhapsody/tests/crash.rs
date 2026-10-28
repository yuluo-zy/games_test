//! GPU 故障数据解析使用原字数/标签合同，不触发真实设备故障。
use ash::vk;
use rhapsody::command::crash::{GpuCrashDiagnostics, has_any_gpu_assertion_failed, parse_gpu_debug_info};
use std::collections::BTreeMap;
#[test]
fn count_is_second_word_and_payload_is_bounded() {
    assert!(!has_any_gpu_assertion_failed(None).unwrap());
    assert!(!has_any_gpu_assertion_failed(Some(&[1, 0])).unwrap());
    assert!(has_any_gpu_assertion_failed(Some(&[0, 1])).unwrap());
    assert!(has_any_gpu_assertion_failed(Some(&[0])).is_err());
    let words = [99, 50, 99, 99, (3 << 24) | 77, 0x123, 4, 5];
    let log =
        parse_gpu_debug_info(&words, &BTreeMap::from([(0x123, "fixture.hlsl".into())])).unwrap();
    assert!(log.truncated);
    assert_eq!(log.records.len(), 1);
    assert_eq!(log.records[0].source_line, 77);
    assert_eq!(log.records[0].source_name.as_deref(), Some("fixture.hlsl"));
    assert_eq!(log.records[0].message, "assertion failed (4 > 5)");
}
#[test]
fn packet_tags_match_original_jump_table_and_unknown_is_retained() {
    let mut words = vec![0, 7, 0, 0];
    for tag in 1..=7 {
        words.extend([(tag << 24) | 12, 42, 3, 4]);
    }
    let log = parse_gpu_debug_info(&words, &BTreeMap::new()).unwrap();
    assert_eq!(
        log.records
            .iter()
            .map(|record| record.message.as_str())
            .collect::<Vec<_>>(),
        vec![
            "abort (3, 4)",
            "assertion failed",
            "assertion failed (3 > 4)",
            "assertion failed (3 < 4)",
            "assertion failed (3 >= 4)",
            "assertion failed (3 <= 4)",
            "???"
        ]
    );
    assert_eq!(log.records[0].source_id, 42);
    assert_eq!(log.records[6].kind, 7);
}
#[test]
fn only_timeout_and_device_lost_request_device_diagnostics() {
    assert!(GpuCrashDiagnostics::needs_vulkan_error_info(
        vk::Result::TIMEOUT
    ));
    assert!(GpuCrashDiagnostics::needs_vulkan_error_info(
        vk::Result::ERROR_DEVICE_LOST
    ));
    assert!(!GpuCrashDiagnostics::needs_vulkan_error_info(
        vk::Result::ERROR_OUT_OF_DEVICE_MEMORY
    ));
    assert!(!GpuCrashDiagnostics::needs_vulkan_error_info(
        vk::Result::SUCCESS
    ));
}
