use super::*;
#[test]
fn coherent_memory_is_never_flushed_and_noncoherent_constants_defer() {
    let noncoherent = vk::MemoryPropertyFlags::HOST_VISIBLE;
    let coherent = noncoherent | vk::MemoryPropertyFlags::HOST_COHERENT;
    assert_eq!(host_write_flush(coherent, true), HostFlush::None);
    assert_eq!(host_write_flush(coherent, false), HostFlush::None);
    assert_eq!(host_write_flush(noncoherent, true), HostFlush::Deferred);
    assert_eq!(host_write_flush(noncoherent, false), HostFlush::Now);
}
#[test]
fn many_noncoherent_writes_flush_once_per_page_with_merged_atoms() {
    use ash::vk::Handle;
    let queue = HostFlushQueue::default();
    let atom = 64u64;
    let (page_a, page_b) = (
        vk::DeviceMemory::from_raw(0x10),
        vk::DeviceMemory::from_raw(0x20),
    );
    // 1000 consecutive 48-byte constants written into page A (atom
    // aligned by the writer), and a few scattered writes into page B.
    for index in 0..1000u64 {
        let start = index * 48 / atom * atom;
        let end = ((index + 1) * 48).div_ceil(atom) * atom;
        queue.record(1, page_a, start, end - start).unwrap();
    }
    queue.record(2, page_b, 0, atom).unwrap();
    queue.record(2, page_b, 4 * atom, atom).unwrap();
    queue.record(2, page_b, atom, atom).unwrap();
    let mut calls = 0;
    let mut seen = Vec::new();
    queue
        .flush_with(|ranges| {
            calls += 1;
            seen.extend(ranges.iter().map(|r| (r.memory, r.offset, r.size)));
            Ok(())
        })
        .unwrap();
    assert_eq!(calls, 1, "one flush call covers every dirty page");
    assert_eq!(
        seen,
        [
            (page_a, 0, 48_000u64.div_ceil(atom) * atom),
            (page_b, 0, 2 * atom),
            (page_b, 4 * atom, atom),
        ]
    );
    // Nothing dirty: the next submit performs no flush call.
    queue
        .flush_with(|_| panic!("clean queue must not flush"))
        .unwrap();
    assert_eq!(queue.flush_calls(), 1);
}
#[test]
fn failed_flush_keeps_ranges_and_freed_pages_are_forgotten() {
    use ash::vk::Handle;
    let queue = HostFlushQueue::default();
    let memory = vk::DeviceMemory::from_raw(0x30);
    queue.record(7, memory, 0, 256).unwrap();
    queue.record(8, memory, 512, vk::WHOLE_SIZE).unwrap();
    assert!(queue.flush_with(|_| Err(Error::OutOfHostMemory)).is_err());
    queue.forget(8);
    let mut seen = Vec::new();
    queue
        .flush_with(|ranges| {
            seen.extend(ranges.iter().map(|r| (r.offset, r.size)));
            Ok(())
        })
        .unwrap();
    assert_eq!(seen, [(0, 256)]);
}
#[test]
fn in_progress_flush_blocks_forget_and_other_submissions_until_native_completion() {
    use ash::vk::Handle;
    use std::{sync::mpsc, time::Duration};
    for forget in [true, false] {
        let queue = HostFlushQueue::default();
        queue.record(7, vk::DeviceMemory::from_raw(0x30), 0, 256).unwrap();
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (attempt_tx, attempt_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let returned_early = std::thread::scope(|scope| {
            let queued = &queue;
            let flushing = scope.spawn(move || queued.flush_with(|ranges| {
                assert_eq!(ranges.len(), 1);
                entered_tx.send(()).unwrap();
                release_rx.recv().unwrap();
                Ok(())
            }));
            entered_rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let queued = &queue;
            let waiting = scope.spawn(move || {
                attempt_tx.send(()).unwrap();
                if forget { queued.forget(7); }
                else { queued.flush_with(|_| panic!("completed ranges must not flush twice")).unwrap(); }
                done_tx.send(()).unwrap();
            });
            attempt_rx.recv_timeout(Duration::from_secs(2)).unwrap();
            let early = done_rx.recv_timeout(Duration::from_millis(50)).is_ok();
            // Release the controlled native call before asserting, so a
            // failing regression still joins both threads and terminates.
            release_tx.send(()).unwrap();
            flushing.join().unwrap().unwrap();
            waiting.join().unwrap();
            early
        });
        assert!(!returned_early, "forget={forget}: native flush is still using the memory");
        assert_eq!(queue.flush_calls(), 1);
    }
}
#[test]
fn scattered_writes_collapse_to_a_bounded_hull() {
    let mut page = DirtyPage {
        id: 1,
        memory: vk::DeviceMemory::null(),
        ranges: smallvec::SmallVec::new(),
    };
    for index in 0..(MAX_DIRTY_RANGES_PER_PAGE as u64 + 1) {
        page.insert(index * 128, index * 128 + 64);
    }
    assert_eq!(
        page.ranges.as_slice(),
        [(0, MAX_DIRTY_RANGES_PER_PAGE as u64 * 128 + 64)]
    );
}
