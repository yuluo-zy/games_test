use super::*;

#[test]
fn begin_frame_drains_to_the_latest_one_in_submission_order() {
    let mut t = FrameThrottle::new();
    for i in 0..4 {
        t.end_frame(i);
    }
    let mut waited = Vec::new();
    t.begin_frame::<()>(|r| {
        waited.push(*r);
        Ok(())
    })
    .unwrap();
    assert_eq!(waited, [0, 1, 2]);
    assert_eq!(t.in_flight(), 1);
}

#[test]
fn groups_count_consecutive_equal_keys() {
    let mut t = FrameThrottle::new();
    assert_eq!(t.in_flight_groups(|r: &u32| *r), 0);
    for r in [1u32, 1, 2, 2, 3] {
        t.end_frame(r);
    }
    assert_eq!(t.in_flight_groups(|r| *r), 3);
    assert_eq!(t.in_flight(), 5);
    // 每个回执自成一组时与旧的 in_flight 一致。
    let mut single = FrameThrottle::new();
    for r in 0u32..4 {
        single.end_frame(r);
    }
    assert_eq!(single.in_flight_groups(|r| *r), single.in_flight());
}

#[test]
fn a_failed_wait_keeps_the_unwaited_receipts() {
    let mut t = FrameThrottle::new();
    for i in 0..3 {
        t.end_frame(i);
    }
    let r = t.begin_frame(|r| if *r == 1 { Err("lost") } else { Ok(()) });
    assert_eq!(r, Err("lost"));
    assert_eq!(t.in_flight(), 2);
}

#[test]
fn trim_waits_on_the_front_only_when_more_than_one_is_in_flight() {
    let mut t = FrameThrottle::new();
    t.end_frame(10);
    assert!(!t.wait_gpu_queue_trim(5, |_, _| unreachable!()));
    t.end_frame(11);
    assert!(!t.wait_gpu_queue_trim(0, |_, _| unreachable!()));
    let mut seen = None;
    assert!(t.wait_gpu_queue_trim(5, |r, d| seen = Some((*r, d))));
    assert_eq!(seen, Some((10, 5)));
    assert_eq!(t.in_flight(), 2);
}
