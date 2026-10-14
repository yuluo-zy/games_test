//! Accumulated changes with independent consumption boundaries and immutable snapshots.
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Weak},
};

/// States must clone independently. `Clone` and delta application must not panic
/// or reenter this publisher/subscriber while a publication is being prepared.
pub trait State: Clone {
    fn empty() -> Self;
}
pub trait StateDelta<T: State> {
    fn apply_to(&self, state: &mut T);
}

pub struct Publication<T> {
    pub state: T,
    pub sequence: u64,
}
struct Subscription<T> {
    pending: Option<Arc<Publication<T>>>,
}
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

pub struct Publisher<T> {
    subscriptions: Vec<Weak<Mutex<Subscription<T>>>>,
    sequence: u64,
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
    pub fn new() -> Self {
        Self::default()
    }
    /// Cloned subscribers share one consumption boundary. New subscribers receive
    /// future deltas only, not changes published before subscription.
    pub fn subscribe(&mut self) -> Subscriber<T> {
        self.subscriptions.retain(|s| s.strong_count() != 0);
        let subscription = Arc::new(Mutex::new(Subscription { pending: None }));
        self.subscriptions.push(Arc::downgrade(&subscription));
        Subscriber { subscription }
    }
    /// Apply a delta once per distinct pending batch, sharing its replacement.
    /// Consumption is briefly serialized while snapshots are prepared; returned
    /// readers carry no locks. Sequence exhaustion leaves all state unchanged.
    pub fn publish<D: StateDelta<T>>(&mut self, delta: D) -> Result<(), PublishError> {
        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or(PublishError::SequenceExhausted)?;
        let mut live = Vec::with_capacity(self.subscriptions.len());
        self.subscriptions.retain(|s| match s.upgrade() {
            Some(s) => {
                live.push(s);
                true
            }
            None => false,
        });
        // Lock in registration order, prepare every replacement, then commit.
        // No subscriber ever acquires a second subscription lock.
        let mut guards: Vec<_> = live.iter().map(|s| s.lock()).collect();
        let mut batches: HashMap<Option<usize>, Arc<Publication<T>>> = HashMap::new();
        let replacements: Vec<_> = guards
            .iter()
            .map(|subscription| {
                let key = subscription
                    .pending
                    .as_ref()
                    .map(|p| Arc::as_ptr(p) as usize);
                Arc::clone(batches.entry(key).or_insert_with(|| {
                    let mut state = subscription
                        .pending
                        .as_ref()
                        .map_or_else(T::empty, |p| p.state.clone());
                    delta.apply_to(&mut state);
                    Arc::new(Publication { state, sequence })
                }))
            })
            .collect();
        for (subscription, publication) in guards.iter_mut().zip(replacements) {
            subscription.pending = Some(publication);
        }
        self.sequence = sequence;
        Ok(())
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}
impl<T> Subscriber<T> {
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
