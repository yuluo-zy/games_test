use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Counter(AtomicUsize);
impl DeviceExtension for Counter {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn release_for_device_drop(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn a_slot_is_created_once_and_released_on_device_drop() {
    let extensions = Extensions::default();
    assert!(extensions.get::<Counter>(ExtensionSlot::Setup).is_none());
    let created = AtomicUsize::new(0);
    for _ in 0..3 {
        extensions.get_or_init(ExtensionSlot::Setup, || {
            created.fetch_add(1, Ordering::Relaxed);
            Counter(AtomicUsize::new(0))
        });
    }
    assert_eq!(created.load(Ordering::Relaxed), 1);
    extensions.release_for_device_drop();
    let counter = extensions.get::<Counter>(ExtensionSlot::Setup).unwrap();
    assert_eq!(counter.0.load(Ordering::Relaxed), 1);
}

#[test]
fn a_failed_initialiser_leaves_the_slot_empty_for_a_retry() {
    let extensions = Extensions::default();
    let failed = extensions.get_or_try_init::<Counter>(ExtensionSlot::Resources, || {
        Err(crate::Error::OutOfHostMemory)
    });
    assert!(matches!(failed, Err(crate::Error::OutOfHostMemory)));
    assert!(extensions.get::<Counter>(ExtensionSlot::Resources).is_none());
    let retried = extensions
        .get_or_try_init(ExtensionSlot::Resources, || Ok(Counter(AtomicUsize::new(0))));
    assert!(retried.is_ok());
}
