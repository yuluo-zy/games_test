#![doc = include_str!("../README.md")]
use parking_lot::{Mutex, MutexGuard};
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Weak},
};

/// An independently clonable batch of accumulated changes.
///
/// `empty`, `Clone` and delta application run under subscription locks. They must
/// be short and must not reenter this publisher or its subscribers. Clones must
/// not share mutable state with the original batch.
pub trait State: Clone {
    /// Construct a batch containing no changes.
    fn empty() -> Self;
}
/// A change merged once into each distinct pending batch during publication.
pub trait StateDelta<T: State> {
    /// Merge this change without modifying other batches or publication state.
    fn apply_to(&self, state: &mut T);
}

/// Owned batch contents and the latest publication sequence included in them.
pub struct Publication<T> {
    pub state: T,
    pub sequence: u64,
}
struct Subscription<T> {
    pending: Option<Arc<Publication<T>>>,
}
/// A consumption boundary; clones compete for the same pending batch.
pub struct Subscriber<T> {
    subscription: Arc<Mutex<Subscription<T>>>,
}
impl<T> Clone for Subscriber<T> {
    fn clone(&self) -> Self {
        Self {
            subscription: Arc::clone(&self.subscription),
        }
    }
}

/// An immutable snapshot. Retaining it never prevents future publication.
pub struct PublicationReader<T> {
    publication: Arc<Publication<T>>,
}
impl<T> Deref for PublicationReader<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.publication.state
    }
}
impl<T> PublicationReader<T> {
    /// The last successful publication included in this snapshot.
    pub fn sequence(&self) -> u64 {
        self.publication.sequence
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishError {
    SequenceExhausted,
}
impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("publication sequence exhausted")
    }
}
impl std::error::Error for PublishError {}

/// Publishes changes to independently consumed, immutable batch snapshots.
pub struct Publisher<T> {
    subscriptions: Vec<Weak<Mutex<Subscription<T>>>>,
    sequence: u64,
}
// Field drop order is intentional: unwinding must unlock every subscription
// before dropping candidate states, whose destructors may access subscribers.
struct PublicationPreparation<'a, T> {
    guards: Vec<MutexGuard<'a, Subscription<T>>>,
    candidate: Option<T>,
    batches: HashMap<Option<*const Publication<T>>, Arc<Publication<T>>>,
    replacements: Vec<Arc<Publication<T>>>,
}

impl<T> Default for Publisher<T> {
    fn default() -> Self {
        Self {
            subscriptions: Vec::new(),
            sequence: 0,
        }
    }
}
impl<T: State> Publisher<T> {
    /// Create a publisher with no subscribers and sequence zero.
    pub fn new() -> Self {
        Self::default()
    }
    /// Cloned subscribers share one consumption boundary. New subscribers receive
    /// future deltas only, not changes published before subscription.
    pub fn subscribe(&mut self) -> Subscriber<T> {
        self.subscriptions
            .retain(|subscription| subscription.strong_count() != 0);
        let subscription = Arc::new(Mutex::new(Subscription { pending: None }));
        self.subscriptions.push(Arc::downgrade(&subscription));
        Subscriber { subscription }
    }
    /// Apply a delta once per distinct pending batch, sharing its replacement.
    /// Consumption is briefly serialized while snapshots are prepared; returned
    /// readers carry no locks. Sequence exhaustion leaves all state unchanged.
    ///
    /// A panic from state construction, cloning or delta application propagates
    /// without committing any pending batch or advancing the sequence. Callback
    /// side effects outside the independently owned candidate cannot be undone.
    pub fn publish<D: StateDelta<T>>(&mut self, delta: D) -> Result<(), PublishError> {
        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or(PublishError::SequenceExhausted)?;
        let mut live = Vec::with_capacity(self.subscriptions.len());
        self.subscriptions
            .retain(|subscription| match subscription.upgrade() {
                Some(subscription) => {
                    live.push(subscription);
                    true
                }
                None => false,
            });
        // Lock in registration order, prepare every replacement, then commit.
        // No subscriber ever acquires a second subscription lock.
        let mut preparation = PublicationPreparation {
            guards: live
                .iter()
                .map(|subscription| subscription.lock())
                .collect(),
            candidate: None,
            batches: HashMap::new(),
            replacements: Vec::with_capacity(live.len()),
        };
        for subscription in &preparation.guards {
            let key = subscription.pending.as_ref().map(Arc::as_ptr);
            let replacement = if let Some(publication) = preparation.batches.get(&key) {
                Arc::clone(publication)
            } else {
                // Keep the candidate in the preparation owner before calling
                // user code, so a panicking delta cannot drop it under locks.
                let state = preparation.candidate.insert(
                    subscription
                        .pending
                        .as_ref()
                        .map_or_else(T::empty, |publication| publication.state.clone()),
                );
                delta.apply_to(state);
                let publication = Arc::new(Publication {
                    state: preparation
                        .candidate
                        .take()
                        .expect("candidate was just inserted"),
                    sequence,
                });
                preparation.batches.insert(key, Arc::clone(&publication));
                publication
            };
            preparation.replacements.push(replacement);
        }
        let mut retired = Vec::with_capacity(preparation.guards.len());
        for (subscription, publication) in
            preparation.guards.iter_mut().zip(preparation.replacements)
        {
            retired.push(subscription.pending.replace(publication));
        }
        self.sequence = sequence;
        drop(preparation.guards);
        drop(retired);
        Ok(())
    }
    /// Number of successfully committed publications, including those without subscribers.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}
impl<T> Subscriber<T> {
    /// Consume the pending batch, or return `None` if this boundary has no changes.
    pub fn take(&mut self) -> Option<PublicationReader<T>> {
        self.subscription
            .lock()
            .pending
            .take()
            .map(|publication| PublicationReader { publication })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq, Clone)]
    struct Changes(Vec<i32>);
    impl State for Changes {
        fn empty() -> Self {
            Self(Vec::new())
        }
    }
    impl StateDelta<Changes> for i32 {
        fn apply_to(&self, state: &mut Changes) {
            state.0.push(*self);
        }
    }
    #[test]
    fn independent_boundaries_and_stable_readers() {
        let mut p = Publisher::<Changes>::new();
        let mut fast = p.subscribe();
        let mut slow = p.subscribe();
        assert!(fast.take().is_none());
        p.publish(7).unwrap();
        let old = fast.take().unwrap();
        p.publish(9).unwrap();
        assert_eq!(old.0, [7]);
        assert_eq!(old.sequence(), 1);
        assert_eq!(fast.take().unwrap().0, [9]);
        assert_eq!(slow.take().unwrap().0, [7, 9]);
        let mut late = p.subscribe();
        assert!(late.take().is_none());
        p.publish(11).unwrap();
        assert_eq!(late.take().unwrap().0, [11]);
    }
    #[test]
    fn cloned_subscribers_share_take() {
        let mut p = Publisher::<Changes>::new();
        let mut a = p.subscribe();
        let mut b = a.clone();
        p.publish(1).unwrap();
        drop(a.take().unwrap());
        assert!(b.take().is_none());
        p.publish(2).unwrap();
        assert_eq!(b.take().unwrap().0, [2]);
    }
    #[test]
    fn abandoned_subscription_and_batch_are_reclaimed() {
        let mut p = Publisher::<Changes>::new();
        let s = p.subscribe();
        let weak = Arc::downgrade(&s.subscription);
        p.publish(1).unwrap();
        let batch = Arc::downgrade(s.subscription.lock().pending.as_ref().unwrap());
        drop(s);
        assert!(weak.upgrade().is_none());
        assert!(batch.upgrade().is_none());
        for _ in 0..1000 {
            drop(p.subscribe());
        }
        p.publish(2).unwrap();
        assert!(p.subscriptions.is_empty());
    }
    #[test]
    fn each_pending_batch_is_applied_once() {
        struct Counted<'a>(&'a std::sync::atomic::AtomicUsize);
        impl StateDelta<Changes> for Counted<'_> {
            fn apply_to(&self, state: &mut Changes) {
                self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                state.0.push(1);
            }
        }
        let calls = std::sync::atomic::AtomicUsize::new(0);
        let mut p = Publisher::<Changes>::new();
        let mut a = p.subscribe();
        let mut b = p.subscribe();
        p.publish(Counted(&calls)).unwrap();
        p.publish(Counted(&calls)).unwrap();
        assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 2);
        let x = a.take().unwrap();
        let y = b.take().unwrap();
        assert!(Arc::ptr_eq(&x.publication, &y.publication));
    }
    #[test]
    fn retired_states_are_dropped_after_unlocking() {
        use std::sync::atomic::{AtomicBool, Ordering};
        #[derive(Clone)]
        struct DropProbe(Arc<dyn Fn() + Send + Sync>);
        impl State for DropProbe {
            fn empty() -> Self {
                Self(Arc::new(|| {}))
            }
        }
        impl Drop for DropProbe {
            fn drop(&mut self) {
                (self.0)();
            }
        }
        impl StateDelta<DropProbe> for () {
            fn apply_to(&self, _: &mut DropProbe) {}
        }
        let mut publisher = Publisher::<DropProbe>::new();
        let subscriber = publisher.subscribe();
        let weak = Arc::downgrade(&subscriber.subscription);
        let unlocked = Arc::new(AtomicBool::new(false));
        let observed = unlocked.clone();
        subscriber.subscription.lock().pending = Some(Arc::new(Publication {
            state: DropProbe(Arc::new(move || {
                if let Some(subscription) = weak.upgrade() {
                    observed.store(subscription.try_lock().is_some(), Ordering::Relaxed);
                }
            })),
            sequence: 0,
        }));
        publisher.publish(()).unwrap();
        assert!(unlocked.load(Ordering::Relaxed));
    }
    #[test]
    fn failed_preparation_drops_candidates_after_unlocking() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        #[derive(Clone)]
        struct DropProbe(Arc<dyn Fn() + Send + Sync>);
        impl State for DropProbe {
            fn empty() -> Self {
                Self(Arc::new(|| {}))
            }
        }
        impl Drop for DropProbe {
            fn drop(&mut self) {
                (self.0)();
            }
        }
        impl StateDelta<DropProbe> for AtomicUsize {
            fn apply_to(&self, _: &mut DropProbe) {
                if self.fetch_add(1, Ordering::Relaxed) == 1 {
                    panic!("second batch failed");
                }
            }
        }
        let mut publisher = Publisher::<DropProbe>::new();
        let mut first = publisher.subscribe();
        let mut second = publisher.subscribe();
        let subscriptions = [
            Arc::downgrade(&first.subscription),
            Arc::downgrade(&second.subscription),
        ];
        let dropped_unlocked = Arc::new(Mutex::new(Vec::new()));
        for subscriber in [&first, &second] {
            let subscriptions = subscriptions.clone();
            let observed = Arc::clone(&dropped_unlocked);
            subscriber.subscription.lock().pending = Some(Arc::new(Publication {
                state: DropProbe(Arc::new(move || {
                    let all_unlocked = subscriptions.iter().all(|subscription| {
                        subscription
                            .upgrade()
                            .is_none_or(|subscription| subscription.try_lock().is_some())
                    });
                    observed.lock().push(all_unlocked);
                })),
                sequence: 0,
            }));
        }
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                publisher.publish(AtomicUsize::new(0)).unwrap();
            }))
            .is_err()
        );
        // Both the completed first replacement and failing second candidate
        // are retired only after every subscription lock has been released.
        assert_eq!(*dropped_unlocked.lock(), [true, true]);
        assert_eq!(publisher.sequence(), 0);
        assert_eq!(first.take().unwrap().sequence(), 0);
        assert_eq!(second.take().unwrap().sequence(), 0);
    }

    #[test]
    fn exhaustion_is_transactional() {
        let mut p = Publisher::<Changes>::new();
        let mut s = p.subscribe();
        p.publish(1).unwrap();
        p.sequence = u64::MAX;
        assert_eq!(p.publish(2), Err(PublishError::SequenceExhausted));
        assert_eq!(p.sequence(), u64::MAX);
        assert_eq!(s.take().unwrap().0, [1]);
    }
    #[test]
    fn concurrent_consumption_preserves_every_delta_once() {
        let mut p = Publisher::<Changes>::new();
        let mut s = p.subscribe();
        let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let finished = done.clone();
        let join = std::thread::spawn(move || {
            let mut values = Vec::new();
            while !finished.load(std::sync::atomic::Ordering::Acquire) {
                if let Some(r) = s.take() {
                    values.extend_from_slice(&r.0);
                } else {
                    std::thread::yield_now();
                }
            }
            if let Some(r) = s.take() {
                values.extend_from_slice(&r.0);
            }
            values
        });
        for i in 0..1000 {
            p.publish(i).unwrap();
        }
        done.store(true, std::sync::atomic::Ordering::Release);
        assert_eq!(join.join().unwrap(), (0..1000).collect::<Vec<_>>());
    }
}
