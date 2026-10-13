//! B's change-publication policy, recovered from publish/subscribe/take and their
//! private retain predicates. Synchronization itself is the original dependency.
use parking_lot::lock_api::ArcRwLockReadGuard;
use parking_lot::{Mutex, RawRwLock, RwLock};
use std::{ops::Deref, sync::Arc};

pub trait State {
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
    last_sequence: u64,
    pending: Option<Arc<RwLock<Publication<T>>>>,
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

pub struct PublicationReader<T> {
    guard: ArcRwLockReadGuard<RawRwLock, Publication<T>>,
}
impl<T> Deref for PublicationReader<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.guard.state
    }
}
impl<T> PublicationReader<T> {
    pub fn sequence(&self) -> u64 {
        self.guard.sequence
    }
}

pub struct Publisher<T> {
    publications: Vec<Arc<RwLock<Publication<T>>>>,
    subscriptions: Vec<Arc<Mutex<Subscription<T>>>>,
    sequence: u64,
}
impl<T> Default for Publisher<T> {
    fn default() -> Self {
        Self {
            publications: Vec::new(),
            subscriptions: Vec::new(),
            sequence: 0,
        }
    }
}
impl<T: State> Publisher<T> {
    pub fn new() -> Self {
        Self::default()
    }
    /// A fresh subscription has no historical publication (subscribe initializes
    /// sequence=0,pending=None). It receives the next delta rather than a snapshot.
    pub fn subscribe(&mut self) -> Subscriber<T> {
        let subscription = Arc::new(Mutex::new(Subscription {
            last_sequence: 0,
            pending: None,
        }));
        self.subscriptions.push(Arc::clone(&subscription));
        Subscriber { subscription }
    }
    /// Accumulate into every still-shared publication. Subscribers that have not
    /// taken their pending publication keep that same batch. A subscriber that has
    /// consumed it starts a new batch on the next publish. Publication readers
    /// hold a read lock, so concurrent publication waits until they are released.
    pub fn publish<D: StateDelta<T>>(&mut self, delta: D) {
        self.sequence = self.sequence.wrapping_add(1);
        let sequence = self.sequence;
        self.publications.retain(|publication| {
            if Arc::strong_count(publication) == 1 {
                return false;
            }
            let mut publication = publication.write();
            delta.apply_to(&mut publication.state);
            publication.sequence = sequence;
            true
        });
        let mut new_publication = None;
        for subscription in &self.subscriptions {
            let mut subscription = subscription.lock();
            if subscription.pending.is_none() && subscription.last_sequence < sequence {
                let publication = new_publication.get_or_insert_with(|| {
                    let mut state = T::empty();
                    delta.apply_to(&mut state);
                    Arc::new(RwLock::new(Publication { state, sequence }))
                });
                subscription.pending = Some(Arc::clone(publication));
            }
        }
        if let Some(publication) = new_publication {
            self.publications.push(publication);
        }
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
}
impl<T> Subscriber<T> {
    pub fn take(&mut self) -> Option<PublicationReader<T>> {
        let mut subscription = self.subscription.lock();
        let publication = subscription.pending.take()?;
        subscription.last_sequence = publication.read().sequence;
        let guard = publication.read_arc();
        Some(PublicationReader { guard })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq)]
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
    fn subscribers_have_independent_consumption_boundaries() {
        let mut publisher = Publisher::<Changes>::new();
        let mut fast = publisher.subscribe();
        let mut slow = publisher.subscribe();
        assert!(fast.take().is_none());
        publisher.publish(7);
        {
            let r = fast.take().unwrap();
            assert_eq!(r.0, [7]);
        }
        publisher.publish(9);
        {
            let r = fast.take().unwrap();
            assert_eq!(r.0, [9]);
        }
        {
            let r = slow.take().unwrap();
            assert_eq!(r.0, [7, 9]);
            assert_eq!(r.sequence(), 2);
        }
        assert!(slow.take().is_none());
        let mut late = publisher.subscribe();
        assert!(late.take().is_none());
        publisher.publish(11);
        assert_eq!(late.take().unwrap().0, [11]);
    }
    #[test]
    fn cloning_subscription_shares_take_state() {
        let mut publisher = Publisher::<Changes>::new();
        let mut a = publisher.subscribe();
        let mut b = a.clone();
        publisher.publish(1);
        drop(a.take().unwrap());
        assert!(b.take().is_none());
        publisher.publish(2);
        assert_eq!(b.take().unwrap().0, [2]);
    }
    #[test]
    fn holding_reader_blocks_writer_until_drop() {
        let mut publisher = Publisher::<Changes>::new();
        let mut subscriber = publisher.subscribe();
        publisher.publish(1);
        let reader = subscriber.take().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let join = std::thread::spawn(move || {
            publisher.publish(2);
            tx.send(()).unwrap();
            publisher
        });
        assert!(
            rx.recv_timeout(std::time::Duration::from_millis(30))
                .is_err()
        );
        assert_eq!(reader.0, [1]);
        drop(reader);
        rx.recv_timeout(std::time::Duration::from_secs(3)).unwrap();
        let _publisher = join.join().unwrap();
        assert_eq!(subscriber.take().unwrap().0, [2]);
    }
}
