use super::*;
use std::sync::Arc;

#[test]
fn records_are_ordered_truncated_and_never_panic() {
    let ring = RingBuffer::new(3 * size_of::<Slot>());
    assert_eq!(ring.capacity(), 3);
    assert!(ring.dump().is_empty());
    ring.write(b"abc");
    ring.write(b"");
    ring.write(&[7; MAX_RECORD_BYTES + 37]);
    assert_eq!(ring.records()[0], b"abc");
    assert_eq!(ring.records()[2].len(), MAX_RECORD_BYTES);
    ring.write(b"d");
    assert_eq!(
        ring.records(),
        vec![Vec::new(), vec![7; MAX_RECORD_BYTES], b"d".to_vec()]
    );
    assert_eq!(ring.written(), 4);
    assert_eq!(RingBuffer::new(0).capacity(), 1);
}

#[test]
fn torn_slots_are_skipped() {
    let ring = RingBuffer::new(2 * size_of::<Slot>());
    ring.write(b"ok");
    ring.write(b"torn");
    // A writer that reserved slot 1 and has not committed yet.
    ring.slots[1].sequence.store(5, Ordering::Release);
    assert_eq!(ring.records(), vec![b"ok".to_vec()]);
}

#[test]
fn concurrent_writers_produce_only_whole_records() {
    let ring = Arc::new(RingBuffer::new(64 * size_of::<Slot>()));
    let workers: Vec<_> = (0..4u8)
        .map(|id| {
            let ring = ring.clone();
            std::thread::spawn(move || {
                for _ in 0..2000 {
                    ring.write(&[id; 32]);
                }
            })
        })
        .collect();
    for _ in 0..50 {
        for record in ring.records() {
            assert_eq!(record.len(), 32);
            assert!(record.iter().all(|&b| b == record[0]));
        }
    }
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(ring.records().len(), 64);
}

#[test]
fn overlapping_writer_is_dropped_before_touching_payload() {
    use std::sync::mpsc;
    let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
    let (entered_tx, entered_rx) = mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = mpsc::sync_channel(0);
    let writer = {
        let ring = ring.clone();
        std::thread::spawn(move || {
            ring.write_with(
                &[0xa5; MAX_RECORD_BYTES],
                |_| {},
                |i| {
                    if i == 0 {
                        entered_tx.send(()).unwrap();
                        resume_rx.recv().unwrap();
                    }
                },
            )
        })
    };
    entered_rx.recv().unwrap();
    // A has written only word0 and still owns this single slot. A second
    // writer must return without altering any payload word or sequence.
    ring.write(&[0x5a; MAX_RECORD_BYTES]);
    let during = ring.records();
    let counts = (ring.written(), ring.dropped());
    resume_tx.send(()).unwrap();
    assert!(writer.join().unwrap());
    assert!(during.is_empty());
    assert_eq!(counts, (2, 1));
    assert_eq!(ring.records(), vec![vec![0xa5; MAX_RECORD_BYTES]]);
}

#[test]
fn delayed_reservation_cannot_replace_a_newer_record() {
    use std::sync::mpsc;
    let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
    let (entered_tx, entered_rx) = mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = mpsc::sync_channel(0);
    let older = {
        let ring = ring.clone();
        std::thread::spawn(move || {
            ring.write_with(
                b"older",
                |_| {
                    entered_tx.send(()).unwrap();
                    resume_rx.recv().unwrap();
                },
                |_| {},
            )
        })
    };
    entered_rx.recv().unwrap();
    ring.write(b"newer");
    resume_tx.send(()).unwrap();
    assert!(!older.join().unwrap());
    assert_eq!(ring.written(), 2);
    assert_eq!(ring.dropped(), 1);
    assert_eq!(ring.records(), vec![b"newer".to_vec()]);
}

#[test]
fn a_slot_overwritten_mid_copy_is_skipped() {
    use std::sync::mpsc;
    let ring = Arc::new(RingBuffer::new(size_of::<Slot>()));
    ring.write(&[0xa5; MAX_RECORD_BYTES]);
    let (entered_tx, entered_rx) = mpsc::sync_channel(0);
    let (resume_tx, resume_rx) = mpsc::sync_channel(0);
    let reader = {
        let ring = ring.clone();
        std::thread::spawn(move || {
            ring.records_with(|_, word| {
                if word == 0 {
                    entered_tx.send(()).unwrap();
                    resume_rx.recv().unwrap();
                }
            })
        })
    };
    entered_rx.recv().unwrap();
    ring.write(&[0x5a; MAX_RECORD_BYTES]);
    resume_tx.send(()).unwrap();
    assert!(reader.join().unwrap().is_empty());
    assert_eq!(ring.records(), vec![vec![0x5a; MAX_RECORD_BYTES]]);
    assert_eq!((ring.written(), ring.dropped()), (2, 0));
}

#[test]
fn exhausted_tokens_never_reuse_a_publication_sequence() {
    let ring = RingBuffer::new(size_of::<Slot>());
    ring.next.store(u64::MAX / 2 - 1, Ordering::Relaxed);
    ring.write(b"last representable");
    let committed = ring.slots[0].sequence.load(Ordering::Relaxed);
    assert_eq!(committed, u64::MAX - 1);
    ring.write(b"not representable");
    assert_eq!(ring.dropped(), 1);
    assert_eq!(ring.slots[0].sequence.load(Ordering::Relaxed), committed);
    // Simulate eventual attempt-counter wrap: the terminal state must
    // still refuse publication, rather than reusing an early even stamp.
    ring.next.store(0, Ordering::Relaxed);
    ring.write(b"wrapped");
    assert_eq!(ring.dropped(), 2);
    assert_eq!(ring.records(), vec![b"last representable".to_vec()]);
}

#[test]
fn global_ring_is_idempotent_and_captures_actual_text() {
    init_logging();
    init_logging();
    record_log(format_args!("render error: {}", 42));
    assert!(dump_log_vec().ends_with(b"render error: 42\n"));
    record_log(format_args!("{}", "中".repeat(MAX_RECORD_BYTES)));
    let records = RING.get().unwrap().records();
    let last = records.last().unwrap();
    assert!(last.len() <= MAX_RECORD_BYTES);
    assert_eq!(last.last(), Some(&b'\n'));
    assert!(std::str::from_utf8(last).is_ok());
}
