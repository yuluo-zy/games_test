use super::*;
#[test]
fn one_scan_rounds_progress_past_busy_owners_and_retry_fifo() {
    let queue = Retirement::new();
    // More busy owners than the entire per-round allowance.
    for item in 0..9 {
        queue.try_enqueue(SyncSet::EMPTY, item).unwrap();
    }
    let after = SyncSet::from_point(crate::sync::SyncPoint {
        queue: QueueId::new(0).unwrap(),
        value: 1,
    });
    for item in 9..12 {
        queue.try_enqueue(after, item).unwrap();
    }
    let mut release_second_busy = false;
    let mut seen = Vec::new();
    for round in 0..80 {
        if round == 30 {
            release_second_busy = true;
        }
        let progress = queue.collect_limited_retry(&CompletedValues::all(), 1, 1, |item| {
            if item >= 9 || (release_second_busy && item == 1) {
                seen.push(item);
                Ok(())
            } else {
                Err(item)
            }
        });
        assert!(progress.scanned <= 1 && progress.destroyed <= 1);
    }
    assert_eq!(
        seen,
        [9, 10, 11, 1],
        "a permanently busy first owner cannot hide another retry"
    );
    assert_eq!(queue.len(), 8);
    queue.collect(&CompletedValues::all(), &mut seen);
    seen.sort_unstable();
    assert_eq!(seen, (0..12).collect::<Vec<_>>());
    assert!(queue.is_empty());
}
