//! Exercise the actual typed weak stores under concurrent ownership changes.
use rhapsody::raw::managed::resources::ResourceManager;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
struct Allocation(Arc<AtomicUsize>);
impl Drop for Allocation {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
#[test]
fn concurrent_get_and_last_drop_never_resurrects_a_disposed_handle() {
    let store = Arc::new(ResourceManager::new(Default::default()).unwrap());
    for _ in 0..200 {
        let handle = store.allocate(1u32).unwrap();
        let key = handle.key();
        let barrier = Arc::new(std::sync::Barrier::new(2));
        let (reader, start) = (store.clone(), barrier.clone());
        let getter = std::thread::spawn(move || {
            start.wait();
            reader.get(key).map(|m| *m)
        });
        barrier.wait();
        drop(handle);
        if let Some(value) = getter.join().unwrap() {
            assert_eq!(value, 1);
        }
        assert!(store.get(key).is_none());
        assert!(store.is_empty());
    }
}
#[test]
fn concurrent_allocations_keep_each_live_typed_key_distinct() {
    let store = Arc::new(ResourceManager::new(Default::default()).unwrap());
    let workers: Vec<_> = (0..8u32)
        .map(|thread| {
            let store = store.clone();
            std::thread::spawn(move || {
                (0..2000u32)
                    .map(|i| {
                        let value = thread * 10_000 + i;
                        let handle = store.allocate(value).unwrap();
                        assert_eq!(store.get(handle.key()).map(|m| *m), Some(value));
                        (handle, value)
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let all: Vec<_> = workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect();
    let mut keys: Vec<_> = all.iter().map(|(handle, _)| handle.key()).collect();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), 16_000);
    assert_eq!(store.len(), 16_000);
    for (handle, value) in &all {
        assert_eq!(*store.get(handle.key()).unwrap(), *value);
    }
    drop(all);
    assert!(store.is_empty());
}
#[test]
fn concurrent_churn_rejects_every_stale_generation() {
    let store = Arc::new(ResourceManager::new(Default::default()).unwrap());
    let old: Vec<_> = (0..64u32).map(|i| store.allocate(i).unwrap()).collect();
    let stale: Vec<_> = old.iter().map(|h| h.key()).collect();
    drop(old);
    let workers: Vec<_> = (0..4u32)
        .map(|thread| {
            let (store, stale) = (store.clone(), stale.clone());
            std::thread::spawn(move || {
                for round in 0..300u32 {
                    let fresh = store.allocate(10_000 + thread * 1000 + round).unwrap();
                    for key in &stale {
                        assert!(store.get(*key).is_none());
                    }
                    assert_eq!(*store.get(fresh.key()).unwrap(), *fresh);
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert!(store.is_empty());
}
#[test]
fn recorded_raw_owner_survives_cpu_key_reuse() {
    let store = ResourceManager::new(Default::default()).unwrap();
    let freed = Arc::new(AtomicUsize::new(0));
    let handle = store.allocate(Allocation(freed.clone())).unwrap();
    let key = handle.key();
    let recorded_owner = handle.shared();
    drop(handle);
    assert!(store.get(key).is_none());
    assert_eq!(freed.load(Ordering::SeqCst), 0);
    let replacement = store.allocate(Allocation(freed.clone())).unwrap();
    assert_eq!(replacement.key().index(), key.index());
    assert_ne!(replacement.key(), key);
    drop(replacement);
    assert_eq!(freed.load(Ordering::SeqCst), 1);
    drop(recorded_owner);
    assert_eq!(freed.load(Ordering::SeqCst), 2);
}
#[test]
fn device_owned_store_has_no_strong_resource_back_edge() {
    struct Host {
        resources: ResourceManager<Owned>,
    }
    struct Owned {
        _host: Arc<Host>,
        freed: Arc<AtomicUsize>,
    }
    impl Drop for Owned {
        fn drop(&mut self) {
            self.freed.fetch_add(1, Ordering::SeqCst);
        }
    }
    let freed = Arc::new(AtomicUsize::new(0));
    let host = Arc::new(Host {
        resources: ResourceManager::new(Default::default()).unwrap(),
    });
    let weak = Arc::downgrade(&host);
    let resource = host
        .resources
        .allocate(Owned {
            _host: host.clone(),
            freed: freed.clone(),
        })
        .unwrap();
    let key = resource.key();
    drop(host);
    assert!(weak.upgrade().is_some());
    drop(resource);
    assert!(weak.upgrade().is_none());
    assert_eq!(freed.load(Ordering::SeqCst), 1);
    // key is a typed identity only; keeping it cannot retain the resource or host.
    assert!(!key.handle().is_null());
}
