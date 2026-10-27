use super::*;
#[test]
fn device_upload_probe_categories_delta_and_generation_are_distinct() {
    let counters = TextureUploadProbeCounters::default();
    let before = counters.snapshot(7);
    for (kind, bytes) in [(TextureUploadProbeKind::SetupRegions, 3),
        (TextureUploadProbeKind::HostDirectRegions, 5), (TextureUploadProbeKind::TransferTextures, 11),
        (TextureUploadProbeKind::BufferSetup, 17)] { counters.record(kind, Some(bytes)); }
    let after = counters.snapshot(7);
    assert_eq!(after.checked_delta(before), Some(after));
    assert_eq!(after.setup_submitted_payload_bytes, 3);
    assert_eq!(after.direct_region_submitted_payload_bytes, 5);
    assert_eq!(after.transfer_submitted_payload_bytes, 11);
    assert_eq!(after.buffer_setup_submitted_payload_bytes, 17);
    assert!(after.checked_delta(counters.snapshot(8)).is_none());
    assert!(before.checked_delta(after).is_none());
    assert!(after.checked_delta(TextureUploadProbeSnapshot::default()).is_none());
}
#[test]
fn device_upload_probe_max_and_overflow_are_sticky_invalid() {
    for payload in [None, Some(u64::MAX)] {
        let counters = TextureUploadProbeCounters::default();
        let before = counters.snapshot(1);
        counters.record(TextureUploadProbeKind::SetupRegions, payload);
        counters.record(TextureUploadProbeKind::SetupRegions, Some(0));
        counters.record(TextureUploadProbeKind::SetupRegions, Some(7));
        let after = counters.snapshot(1);
        assert!(!after.valid);
        assert_eq!(after.setup_submitted_payload_bytes, u64::MAX);
        assert!(after.checked_delta(before).is_none());
    }
    let counters = TextureUploadProbeCounters::default();
    counters.record(TextureUploadProbeKind::BufferSetup, Some(u64::MAX - 1));
    assert!(counters.snapshot(1).valid);
    counters.record(TextureUploadProbeKind::BufferSetup, Some(1));
    assert!(!counters.snapshot(1).valid, "MAX is reserved even without arithmetic wrap");
}
#[test]
fn concurrent_upload_probe_updates_do_not_lose_payload() {
    let counters = TextureUploadProbeCounters::default();
    std::thread::scope(|scope| { for _ in 0..4 { let counters = &counters;
        scope.spawn(move || for _ in 0..100 { counters.record(TextureUploadProbeKind::SetupRegions, Some(7)); });
    }});
    let snapshot = counters.snapshot(2);
    assert!(snapshot.valid);
    assert_eq!(snapshot.setup_submitted_payload_bytes, 2800);
    assert_eq!(snapshot.direct_region_submitted_payload_bytes, 0);
}
