use super::*;
use super::compiler::evict_completed_requests;
use super::disk::{decode, encode};
use super::request::{Cached, Completion, Key, KeyPayload, State};
use super::compiler::worker;
use super::disk::atomic_write_temporary;
use crate::Error;
use parking_lot::{Condvar, Mutex};
use std::{sync::Arc, time::Duration};
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
};

#[test]
fn pressure_evicts_only_completed_unreferenced_requests_in_lru_order() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    let mut state = State {
        priority_burst: 0,
        cancelled: 0,
        queue: VecDeque::new(),
        cached: BTreeMap::new(),
        lru: VecDeque::new(),
        pending: 0,
        request_bytes: 0,
        cache_bytes: 0,
        stopping: false,
        peak_pending: 0,
        peak_request_bytes: 0,
        compiled: 0,
            failed: 0,
        merged: 0,
    };
    let mut tickets = Vec::new();
    // (status, ticket alive) in LRU order.
    let entries = [
        (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), false),
        (PipelinePoll::Pending, false),
        (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), true),
        (PipelinePoll::Failed(Arc::new(Error::OutOfHostMemory)), false),
    ];
    for (index, (status, referenced)) in entries.into_iter().enumerate() {
        let key: Key = Arc::new(KeyPayload::Raster(vec![index as u64]));
        let completion = Arc::new(Completion {
            status: Mutex::new(status),
            changed: Condvar::new(),
        });
        if referenced {
            tickets.push(completion.clone());
        }
        state.cached.insert(
            key.clone(),
            Cached {
                reservation: stats
                    .reserve_cpu_bytes(10, crate::accounting::BudgetCategory::Cache)
                    .unwrap(),
                completion,
                bytes: 10,
            },
        );
        state.lru.push_back(key);
        state.cache_bytes += 10;
    }
    assert_eq!(evict_completed_requests(&mut state, 5), 10);
    assert_eq!(state.cached.len(), 3);
    assert_eq!(evict_completed_requests(&mut state, 100), 10);
    let remaining: Vec<_> = state
        .lru
        .iter()
        .map(|key| match &**key {
            KeyPayload::Raster(values) => values[0],
            _ => unreachable!(),
        })
        .collect();
    assert_eq!(
        remaining,
        [1, 2],
        "pending and ticket-referenced requests stay"
    );
    assert_eq!(state.cache_bytes, 20);
    assert_eq!(stats.cpu_memory_usage().cache, 20);
    drop(tickets);
}
#[test]
fn atomic_cache_write_does_not_remove_an_unowned_temporary_collision() {
    let directory = std::env::temp_dir().join(format!(
        "rhapsody-cache-collision-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&directory).unwrap();
    let cache = directory.join("cache");
    let temporary = directory.join("occupied.tmp");
    fs::write(&cache, b"previous cache").unwrap();
    fs::write(&temporary, b"another writer owns this").unwrap();
    assert!(atomic_write_temporary(&cache, &temporary, &[b"new cache"]).is_err());
    assert_eq!(fs::read(&temporary).unwrap(), b"another writer owns this");
    assert_eq!(fs::read(&cache).unwrap(), b"previous cache");
    fs::remove_dir_all(directory).unwrap();
}
#[test]
fn disk_envelope_rejects_corruption_stale_identity_and_truncation() {
    let identity = vec![7; 32];
    let bytes = encode(&identity, b"driver payload");
    assert_eq!(decode(&bytes, &identity), Some(&b"driver payload"[..]));
    let mut corrupt = bytes.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    assert!(decode(&corrupt, &identity).is_none());
    assert!(decode(&bytes, &[8; 32]).is_none());
    for length in 0..bytes.len() {
        assert!(decode(&bytes[..length], &identity).is_none());
    }
}
#[test]
fn deferred_queue_owns_bounded_pending_requests_and_merges_before_refusing() {
    let device =
        match crate::device::Device::new_headless(&crate::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    std::env::var_os("RHAPSODY_REQUIRE_VULKAN").is_none(),
                    "{error}"
                );
                return;
            }
        };
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let mut compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device, &[]).unwrap()),
        CompilerConfig {
            max_pending: 1,
            max_deferred_pending: 3,
            max_cached_requests: 1,
            ..Default::default()
        },
    )
    .unwrap();
    // Stop idle workers before enqueueing to deterministically exercise a
    // saturated queue, independent of native driver compilation speed.
    compiler.shared.state.lock().stopping = true;
    compiler.shared.available.notify_all();
    for worker in compiler.workers.drain(..) {
        worker.join().unwrap();
    }
    compiler.shared.state.lock().stopping = false;
    let mut tickets = Vec::new();
    for value in 0..3 {
        tickets.push(
            compiler
                .defer_compute_spirv(vec![value; 4], c"main".to_owned(), layout.clone())
                .unwrap(),
        );
    }
    assert_eq!(compiler.stats().pending, 3);
    let merged = compiler
        .defer_compute_spirv(vec![0; 4], c"main".to_owned(), layout.clone())
        .unwrap();
    assert!(merged.same_request(&tickets[0]));
    let before = compiler.stats();
    assert!(matches!(
        compiler.defer_compute_spirv(vec![3; 4], c"main".to_owned(), layout.clone()),
        Err(Error::OutOfHostMemory)
    ));
    assert!(matches!(
        compiler.request_compute_spirv(vec![3; 4], c"main".to_owned(), layout),
        Err(Error::Timeout)
    ));
    let after = compiler.stats();
    assert_eq!(
        (
            before.pending,
            before.request_bytes,
            before.cached_key_bytes
        ),
        (after.pending, after.request_bytes, after.cached_key_bytes)
    );
    assert!(
        tickets
            .iter()
            .all(|ticket| matches!(ticket.poll(), PipelinePoll::Pending))
    );
    let shared = compiler.shared.clone();
    compiler
        .workers
        .push(std::thread::spawn(move || worker(shared)));
    for ticket in &tickets {
        assert!(
            ticket.wait(Duration::from_secs(10)).is_err(),
            "invalid SPIR-V must retain its terminal error"
        );
    }
    // Failed jobs leave the request table; no permanently full queue remains.
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while compiler.stats().pending != 0 {
        assert!(std::time::Instant::now() < deadline);
        std::thread::yield_now();
    }
    assert_eq!(compiler.stats().request_bytes, 0);
    assert!(compiler.stats().cached_requests <= 1);
}

#[test]
fn worker_and_memory_limits_must_be_bounded() {
    assert!(CompilerConfig::default().validate().is_ok());
    for config in [
        CompilerConfig {
            workers: 0,
            ..Default::default()
        },
        CompilerConfig {
            workers: 17,
            ..Default::default()
        },
        CompilerConfig {
            max_pending: 0,
            ..Default::default()
        },
        CompilerConfig {
            max_deferred_pending: 0,
            ..Default::default()
        },
        CompilerConfig {
            max_request_bytes: 0,
            ..Default::default()
        },
        CompilerConfig {
            max_cached_requests: 0,
            ..Default::default()
        },
    ] {
        assert!(config.validate().is_err());
    }
}
