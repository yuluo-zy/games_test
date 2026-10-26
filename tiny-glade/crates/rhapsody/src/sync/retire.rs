//! Pre-reserved retirement nodes in per-queue ordered trees.
//! Admission happens before native creation; enqueue and cross-queue movement
//! only transfer existing owners. Destruction happens outside all queue locks.
use super::{CompletedValues, MAX_QUEUES, QueueId, SyncSet};
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub(crate) struct Node<T> {
    value: u64,
    after: SyncSet,
    item: Option<T>,
    left: Option<RetirementTicket<T>>,
    right: Option<RetirementTicket<T>>,
    height: u8,
}

/// Exclusive construction credit. Not Clone: one native payload consumes one
/// node, or cancellation returns the node and its host-budget credit.
pub(crate) struct RetirementTicket<T> {
    // One fallibly allocated node; release physical storage before its credit.
    storage: Vec<Node<T>>,
    _memory: Option<crate::raw::allocator::CpuMemoryReservation>,
}
impl<T> std::ops::Deref for RetirementTicket<T> {
    type Target = Node<T>;
    fn deref(&self) -> &Self::Target {
        &self.storage[0]
    }
}
impl<T> std::ops::DerefMut for RetirementTicket<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage[0]
    }
}
impl<T> RetirementTicket<T> {
    pub(crate) fn reserve(
        stats: &Arc<crate::raw::allocator::AllocatorStats>,
    ) -> crate::Result<Self> {
        let memory = stats.reserve_cpu_bytes(
            crate::raw::allocator::cpu_vec_bytes::<Node<T>>(1)?,
            crate::device::BudgetCategory::Live,
        )?;
        Self::new(Some(memory))
    }
    fn new(mut memory: Option<crate::raw::allocator::CpuMemoryReservation>) -> crate::Result<Self> {
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        if let Some(memory) = &mut memory {
            memory.resize(crate::raw::allocator::cpu_vec_bytes::<Node<T>>(
                storage.capacity(),
            )?)?;
        }
        storage.push(Node {
            value: 0,
            after: SyncSet::EMPTY,
            item: None,
            left: None,
            right: None,
            height: 1,
        });
        Ok(Self {
            storage,
            _memory: memory,
        })
    }
}

struct Queue<T> {
    root: Option<RetirementTicket<T>>,
}
impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self { root: None }
    }
}
fn height<T>(node: &Option<RetirementTicket<T>>) -> u8 {
    node.as_ref().map_or(0, |node| node.height)
}
fn refresh<T>(node: &mut RetirementTicket<T>) {
    node.height = 1 + height(&node.left).max(height(&node.right));
}
fn rotate_left<T>(mut node: RetirementTicket<T>) -> RetirementTicket<T> {
    let mut root = node.right.take().expect("right-heavy node");
    node.right = root.left.take();
    refresh(&mut node);
    root.left = Some(node);
    refresh(&mut root);
    root
}
fn rotate_right<T>(mut node: RetirementTicket<T>) -> RetirementTicket<T> {
    let mut root = node.left.take().expect("left-heavy node");
    node.left = root.right.take();
    refresh(&mut node);
    root.right = Some(node);
    refresh(&mut root);
    root
}
fn balance<T>(mut node: RetirementTicket<T>) -> RetirementTicket<T> {
    refresh(&mut node);
    if height(&node.left) > height(&node.right) + 1 {
        let left = node.left.as_ref().unwrap();
        if height(&left.right) > height(&left.left) {
            node.left = node.left.take().map(rotate_left);
        }
        rotate_right(node)
    } else if height(&node.right) > height(&node.left) + 1 {
        let right = node.right.as_ref().unwrap();
        if height(&right.left) > height(&right.right) {
            node.right = node.right.take().map(rotate_right);
        }
        rotate_left(node)
    } else {
        node
    }
}
fn insert<T>(root: Option<RetirementTicket<T>>, node: RetirementTicket<T>) -> RetirementTicket<T> {
    let Some(mut root) = root else { return node };
    if node.value < root.value {
        root.left = Some(insert(root.left.take(), node));
    } else {
        root.right = Some(insert(root.right.take(), node));
    }
    balance(root)
}
fn pop_min<T>(mut node: RetirementTicket<T>) -> (Option<RetirementTicket<T>>, RetirementTicket<T>) {
    if let Some(left) = node.left.take() {
        let (left, minimum) = pop_min(left);
        node.left = left;
        (Some(balance(node)), minimum)
    } else {
        let right = node.right.take();
        node.height = 1;
        (right, node)
    }
}
impl<T> Queue<T> {
    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    fn insert(&mut self, node: RetirementTicket<T>) {
        self.root = Some(insert(self.root.take(), node));
    }
    fn front(&self) -> Option<&Node<T>> {
        let mut node = self.root.as_ref()?;
        while let Some(left) = &node.left {
            node = left;
        }
        Some(node)
    }
    fn pop(&mut self) -> Option<RetirementTicket<T>> {
        let (root, node) = pop_min(self.root.take()?);
        self.root = root;
        Some(node)
    }
}

pub struct Retirement<T> {
    queues: [Mutex<Queue<T>>; MAX_QUEUES],
    // Reached GPU dependencies with a still-busy native host owner. Equal
    // keys preserve retry FIFO, independent of the original timeline value.
    host_busy: [Mutex<Queue<T>>; MAX_QUEUES],
    prefer_busy: [AtomicBool; MAX_QUEUES],
    len: AtomicUsize,
    cursor: AtomicUsize,
    #[cfg(test)]
    fail_next_insert_queue: AtomicUsize,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CollectionProgress {
    pub scanned: usize,
    pub destroyed: usize,
}
impl<T> Default for Retirement<T> {
    fn default() -> Self {
        Self::new()
    }
}
/// Temporarily remove host-busy nodes so they cannot hide independent reached
/// entries. Unwinding also restores every owned node without allocating.
struct BusyNodes<'a, T> {
    owner: &'a Retirement<T>,
    queues: [Queue<T>; MAX_QUEUES],
}
impl<T> Drop for BusyNodes<'_, T> {
    fn drop(&mut self) {
        for (index, queue) in self.queues.iter_mut().enumerate() {
            while let Some(mut node) = queue.pop() {
                node.value = 0;
                self.owner.host_busy[index].lock().insert(node);
            }
        }
    }
}
fn pending_queue(after: &SyncSet, completed: &CompletedValues) -> Option<(usize, u64)> {
    after
        .points()
        .find(|point| point.value > completed.get(point.queue))
        .map(|point| (point.queue.index(), point.value))
}
impl<T> Retirement<T> {
    pub fn new() -> Self {
        Self {
            queues: std::array::from_fn(|_| Mutex::new(Queue::default())),
            host_busy: std::array::from_fn(|_| Mutex::new(Queue::default())),
            prefer_busy: std::array::from_fn(|_| AtomicBool::new(false)),
            len: AtomicUsize::new(0),
            cursor: AtomicUsize::new(0),
            #[cfg(test)]
            fail_next_insert_queue: AtomicUsize::new(usize::MAX),
        }
    }
    pub fn retire(
        &self,
        after: SyncSet,
        item: T,
        completed: &CompletedValues,
    ) -> Result<Option<T>, T> {
        if after.is_reached(completed) {
            return Ok(Some(item));
        }
        self.try_enqueue(after, item).map(|()| None)
    }
    /// Compatibility admission for callers without a native object. Native
    /// owners MUST reserve before creation and call enqueue_reserved at Drop.
    pub fn try_enqueue(&self, after: SyncSet, item: T) -> Result<(), T> {
        #[cfg(test)]
        {
            let queue = after.points().next().map_or(0, |point| point.queue.index());
            if self
                .fail_next_insert_queue
                .compare_exchange(queue, usize::MAX, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return Err(item);
            }
        }
        let ticket = match RetirementTicket::new(None) {
            Ok(ticket) => ticket,
            Err(_) => return Err(item),
        };
        self.enqueue_reserved(after, item, ticket);
        Ok(())
    }
    pub(crate) fn enqueue_reserved(&self, after: SyncSet, item: T, ticket: RetirementTicket<T>) {
        let (queue, value) = after
            .points()
            .next()
            .map_or((0, 0), |point| (point.queue.index(), point.value));
        let mut node = ticket;
        node.value = value;
        node.after = after;
        node.item = Some(item);
        self.len.fetch_add(1, Ordering::AcqRel);
        self.queues[queue].lock().insert(node);
    }
    fn pop_reached(
        &self,
        queue: usize,
        completed: &CompletedValues,
    ) -> Option<RetirementTicket<T>> {
        let done = QueueId::new(queue).map_or(0, |id| completed.get(id));
        let mut fifo = self.queues[queue].lock();
        if fifo.front().is_some_and(|entry| entry.value <= done) {
            fifo.pop()
        } else {
            None
        }
    }
    fn pop_next(&self, queue: usize, completed: &CompletedValues) -> Option<RetirementTicket<T>> {
        // Persist the alternation across rounds: a one-scan allowance must
        // neither starve newly reached objects nor permanently defer retries.
        if self.prefer_busy[queue].fetch_xor(true, Ordering::Relaxed) {
            let busy = { self.host_busy[queue].lock().pop() };
            busy.or_else(|| self.pop_reached(queue, completed))
        } else {
            self.pop_reached(queue, completed)
                .or_else(|| self.host_busy[queue].lock().pop())
        }
    }
    pub fn collect_limited(
        &self,
        completed: &CompletedValues,
        max_scans: usize,
        max_destroys: usize,
        mut ready: impl FnMut(T),
    ) -> CollectionProgress {
        self.collect_limited_retry(completed, max_scans, max_destroys, |item| {
            ready(item);
            Ok(())
        })
    }
    /// A completed GPU object may still have a busy native host owner (for
    /// example another command recording in its pool). Return its unchanged
    /// payload to the SAME reserved node and set it aside for this round.
    pub(crate) fn collect_limited_retry(
        &self,
        completed: &CompletedValues,
        max_scans: usize,
        max_destroys: usize,
        mut ready: impl FnMut(T) -> Result<(), T>,
    ) -> CollectionProgress {
        let mut progress = CollectionProgress::default();
        if self.is_empty() {
            return progress;
        }
        let mut busy = BusyNodes {
            owner: self,
            queues: std::array::from_fn(|_| Queue::default()),
        };
        let first = self.cursor.fetch_add(1, Ordering::Relaxed) % MAX_QUEUES;
        for offset in 0..MAX_QUEUES {
            let queue = (first + offset) % MAX_QUEUES;
            while progress.scanned < max_scans && progress.destroyed < max_destroys {
                let Some(mut node) = self.pop_next(queue, completed) else {
                    break;
                };
                progress.scanned += 1;
                if let Some((next, value)) = pending_queue(&node.after, completed) {
                    node.value = value;
                    self.queues[next].lock().insert(node);
                } else {
                    let item = node.item.take().expect("queued payload");
                    self.len.fetch_sub(1, Ordering::AcqRel);
                    match ready(item) {
                        Ok(()) => {
                            progress.destroyed += 1;
                            drop(node);
                        }
                        Err(item) => {
                            node.item = Some(item);
                            self.len.fetch_add(1, Ordering::AcqRel);
                            busy.queues[queue].insert(node);
                        }
                    }
                }
            }
        }
        progress
    }
    pub fn collect(&self, completed: &CompletedValues, ready: &mut Vec<T>) -> usize {
        self.collect_limited(completed, usize::MAX, usize::MAX, |item| ready.push(item))
            .destroyed
    }
    pub fn drain_all(&self, ready: &mut Vec<T>) -> usize {
        self.collect(&CompletedValues::all(), ready)
    }
    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{QueueId, SyncPoint};

    fn point(queue: usize, value: u64) -> SyncSet {
        SyncSet::from_point(SyncPoint {
            queue: QueueId::new(queue).unwrap(),
            value,
        })
    }

    #[test]
    fn unused_or_completed_items_come_back_immediately() {
        let retirement = Retirement::new();
        let done = CompletedValues::default();
        assert_eq!(
            retirement.retire(SyncSet::EMPTY, "never submitted", &done),
            Ok(Some("never submitted"))
        );
        let mut done = done;
        done.raise(QueueId::new(0).unwrap(), 3);
        assert_eq!(
            retirement.retire(point(0, 3), "finished", &done),
            Ok(Some("finished"))
        );
        assert!(retirement.is_empty());
    }

    #[test]
    fn items_wait_for_every_queue_they_used() {
        let retirement = Retirement::new();
        let mut done = CompletedValues::default();
        let mut both = point(0, 2);
        both.merge(&point(1, 5));
        assert!(
            retirement
                .retire(both, "cross-queue", &done)
                .unwrap()
                .is_none()
        );
        assert!(
            retirement
                .retire(point(0, 1), "graphics", &done)
                .unwrap()
                .is_none()
        );

        let mut ready = Vec::new();
        done.raise(QueueId::new(0).unwrap(), 2);
        assert_eq!(retirement.collect(&done, &mut ready), 1);
        assert_eq!(ready, ["graphics"], "队列 1 未完成时跨队列对象必须保留");

        done.raise(QueueId::new(1).unwrap(), 5);
        assert_eq!(retirement.collect(&done, &mut ready), 1);
        assert_eq!(ready, ["graphics", "cross-queue"]);
        assert!(retirement.is_empty());
    }

    #[test]
    fn drain_all_releases_everything() {
        let retirement = Retirement::new();
        let done = CompletedValues::default();
        for value in 1..=3 {
            assert!(
                retirement
                    .retire(point(0, value), value, &done)
                    .unwrap()
                    .is_none()
            );
        }
        let mut ready = Vec::new();
        assert_eq!(retirement.drain_all(&mut ready), 3);
        ready.sort_unstable();
        assert_eq!(ready, [1, 2, 3]);
        assert!(retirement.is_empty());
    }

    #[test]
    fn bounded_scans_progress_past_pending_entries_and_inspect_swapped_tail() {
        let retirement = Retirement::new();
        for value in 0..7 {
            retirement
                .try_enqueue(point(0, if value < 4 { 9 } else { 1 }), value)
                .ok()
                .unwrap();
        }
        let mut done = CompletedValues::default();
        done.raise(QueueId::new(0).unwrap(), 1);
        let mut ready = Vec::new();
        for _ in 0..7 {
            let progress = retirement.collect_limited(&done, 1, 1, |item| ready.push(item));
            assert!(progress.scanned <= 1 && progress.destroyed <= 1);
        }
        ready.sort_unstable();
        assert_eq!(ready, [4, 5, 6]);
        assert_eq!(retirement.len(), 4);
        done.raise(QueueId::new(0).unwrap(), 9);
        assert_eq!(retirement.collect(&done, &mut ready), 4);
        assert!(retirement.is_empty());
        retirement.try_enqueue(point(0, 1), 8).ok().unwrap();
        assert_eq!(
            retirement.collect_limited(&done, 0, 1, |_| panic!("zero scans")),
            CollectionProgress::default()
        );
        assert_eq!(
            retirement.collect_limited(&done, 1, 0, |_| panic!("zero destroys")),
            CollectionProgress::default()
        );
        let mut final_ready = Vec::new();
        retirement.collect_limited(&done, 1, 1, |item| final_ready.push(item));
        assert_eq!(final_ready, [8]);
    }

    #[test]
    fn every_three_queue_completion_permutation_obeys_small_budgets() {
        const ORDERS: [[usize; 3]; 6] = [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ];
        for order in ORDERS {
            for scans in 1..=7 {
                for destroys in 1..=scans.min(3) {
                    let retirement = Retirement::new();
                    let mut expected = Vec::new();
                    for item in 0..23 {
                        let mut after = SyncSet::EMPTY;
                        for queue in 0..3 {
                            after.merge(&point(queue, (item % 3 + 1) as u64));
                        }
                        expected.push(after);
                        retirement.try_enqueue(after, item).ok().unwrap();
                    }
                    let mut released = [false; 23];
                    let mut done = CompletedValues::default();
                    for value in 1..=3 {
                        for queue in order {
                            done.raise(QueueId::new(queue).unwrap(), value);
                            for _ in 0..23 {
                                let progress =
                                    retirement.collect_limited(&done, scans, destroys, |item| {
                                        assert!(expected[item].is_reached(&done));
                                        assert!(!released[item], "object was reclaimed twice");
                                        released[item] = true;
                                    });
                                assert!(progress.scanned <= scans);
                                assert!(progress.destroyed <= destroys);
                            }
                        }
                    }
                    assert!(retirement.is_empty(), "completion order {order:?}");
                    assert!(released.into_iter().all(|item| item));
                }
            }
        }
    }

    #[test]
    fn seeded_random_four_queue_completion_reclaims_exactly_once() {
        fn random(seed: &mut u64) -> u64 {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *seed
        }
        for mut seed in [1, 0x519a_423e, 0x7521_f1f4_3691_ab35] {
            let retirement = Retirement::new();
            let mut expected = Vec::new();
            for item in 0..97 {
                let mut after = SyncSet::EMPTY;
                for queue in 0..4 {
                    after.merge(&point(queue, random(&mut seed) % 8 + 1));
                }
                expected.push(after);
                retirement.try_enqueue(after, item).ok().unwrap();
            }
            let mut done = CompletedValues::default();
            let mut released = [false; 97];
            for tick in 0..1200 {
                let queue = QueueId::new((random(&mut seed) % 4) as usize).unwrap();
                done.raise(queue, (done.get(queue) + 1).min(8));
                let scans = (random(&mut seed) % 11 + 1) as usize;
                let destroys = (random(&mut seed) % scans as u64 + 1) as usize;
                let progress = retirement.collect_limited(&done, scans, destroys, |item| {
                    assert!(expected[item].is_reached(&done));
                    assert!(!released[item], "object was reclaimed twice at tick {tick}");
                    released[item] = true;
                });
                assert!(progress.scanned <= scans && progress.destroyed <= destroys);
                if retirement.is_empty() {
                    break;
                }
            }
            assert!(retirement.is_empty(), "completed entries must not starve");
            assert!(released.into_iter().all(|item| item));
        }
    }

    #[test]
    fn reserved_queue_move_never_requests_more_storage() {
        let stats = Arc::new(crate::raw::allocator::AllocatorStats::default());
        let retirement = Retirement::new();
        let mut after = point(0, 5);
        after.merge(&point(1, 100));
        let ticket = RetirementTicket::reserve(&stats).unwrap();
        let charged = stats.cpu_memory_usage().total();
        assert!(charged > 0);
        retirement.enqueue_reserved(after, "cross-queue", ticket);
        stats.set_host_budget(charged).unwrap(); // No further reservation fits.
        let mut done = CompletedValues::default();
        done.raise(QueueId::new(0).unwrap(), 5);
        let progress = retirement.collect_limited(&done, 8, 8, |_| panic!("early destruction"));
        assert_eq!(
            progress,
            CollectionProgress {
                scanned: 1,
                destroyed: 0
            }
        );
        assert_eq!(retirement.len(), 1);
        assert_eq!(stats.cpu_memory_usage().total(), charged);
        assert!(RetirementTicket::<&str>::reserve(&stats).is_err());
        done.raise(QueueId::new(1).unwrap(), 100);
        let mut ready = Vec::new();
        assert_eq!(retirement.collect(&done, &mut ready), 1);
        assert_eq!(ready, ["cross-queue"]);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        let ticket = RetirementTicket::<&str>::reserve(&stats).unwrap();
        drop(ticket); // Cancelled creation returns node storage and its credit.
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn injected_enqueue_storage_failure_returns_original_item_without_counting_it() {
        let retirement = Retirement::new();
        retirement
            .fail_next_insert_queue
            .store(1, Ordering::Relaxed);
        assert_eq!(retirement.try_enqueue(point(1, 12), "owned"), Err("owned"));
        assert!(retirement.is_empty());
        assert!(retirement.queues[1].lock().is_empty());
        retirement.try_enqueue(point(1, 12), "retry").ok().unwrap();
        assert_eq!(retirement.len(), 1);
        let mut ready = Vec::new();
        assert_eq!(retirement.drain_all(&mut ready), 1);
        assert_eq!(ready, ["retry"]);
        assert!(retirement.is_empty());
    }

    #[test]
    fn collection_touches_only_the_reached_prefix() {
        let retirement = Retirement::new();
        for value in 1..=1000u64 {
            retirement.try_enqueue(point(0, value), value).ok().unwrap();
        }
        let mut done = CompletedValues::default();
        done.raise(QueueId::new(0).unwrap(), 10);
        let mut ready = Vec::new();
        let progress = retirement.collect_limited(&done, 4096, 4096, |item| ready.push(item));
        assert_eq!(
            progress.scanned, 10,
            "unreached entries are never inspected"
        );
        assert_eq!(ready, (1..=10).collect::<Vec<_>>());
        // A multi-queue entry moves once to the queue it still waits for.
        let mut both = point(0, 5);
        both.merge(&point(1, 3));
        retirement.try_enqueue(both, 0).ok().unwrap();
        let mut ready = Vec::new();
        assert_eq!(retirement.collect(&done, &mut ready), 0);
        done.raise(QueueId::new(1).unwrap(), 3);
        assert_eq!(retirement.collect(&done, &mut ready), 1);
        assert_eq!(ready, [0]);
        assert_eq!(retirement.len(), 990);
    }
}

#[cfg(test)]
mod node_balance_tests {
    use super::*;
    fn check<T>(root: &Option<RetirementTicket<T>>) -> u8 {
        let Some(node) = root else { return 0 };
        let left = check(&node.left);
        let right = check(&node.right);
        assert!(left.abs_diff(right) <= 1);
        assert_eq!(node.height, 1 + left.max(right));
        node.height
    }
    #[test]
    fn ordered_and_duplicate_nodes_remain_balanced_and_fifo() {
        let mut queue = Queue::default();
        for index in 0..8192 {
            let mut ticket = RetirementTicket::new(None).unwrap();
            ticket.value = (index / 4) as u64;
            ticket.item = Some(index);
            queue.insert(ticket);
        }
        assert!(check(&queue.root) < 20);
        for index in 0..8192 {
            let mut node = queue.pop().unwrap();
            assert_eq!(node.item.take(), Some(index));
            check(&queue.root);
        }
        assert!(queue.is_empty());
    }
    #[test]
    fn ready_callback_can_reenter_the_same_retirement_queue() {
        let retirement = Retirement::new();
        retirement.try_enqueue(SyncSet::EMPTY, 1).unwrap();
        let mut observed = Vec::new();
        retirement.collect_limited(&CompletedValues::all(), 2, 2, |item| {
            observed.push(item);
            if item == 1 {
                retirement.try_enqueue(SyncSet::EMPTY, 2).unwrap();
            }
        });
        retirement.collect(&CompletedValues::all(), &mut observed);
        assert_eq!(observed, [1, 2]);
    }
}

#[cfg(test)]
mod retry_tests {
    use super::*;
    #[test]
    fn busy_owner_preserves_payload_then_next_tick_destroys_it_once() {
        let queue = Retirement::new();
        queue.try_enqueue(SyncSet::EMPTY, 7).unwrap();
        assert_eq!(
            queue.collect_limited_retry(&CompletedValues::all(), usize::MAX, usize::MAX, |item| {
                Err(item)
            }),
            CollectionProgress {
                scanned: 1,
                destroyed: 0
            }
        );
        assert_eq!(queue.len(), 1);
        let mut observed = Vec::new();
        assert_eq!(queue.collect(&CompletedValues::all(), &mut observed), 1);
        assert_eq!(observed, [7]);
    }
    #[test]
    fn panicking_destructor_does_not_strand_a_phantom_queue_entry() {
        let queue = Retirement::new();
        queue.try_enqueue(SyncSet::EMPTY, 7).unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            queue.collect_limited_retry(&CompletedValues::all(), 1, 1, |_| {
                panic!("injected destructor panic")
            });
        }));
        assert!(result.is_err());
        assert!(queue.is_empty());
    }
}

#[cfg(test)]
mod busy_fairness_tests {
    use super::*;
    #[test]
    fn busy_zero_does_not_hide_later_ready_owners_in_the_same_queue() {
        let queue = Retirement::new();
        queue.try_enqueue(SyncSet::EMPTY, 0).unwrap();
        let after = SyncSet::from_point(super::super::SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 1,
        });
        for item in 1..=3 {
            queue.try_enqueue(after, item).unwrap();
        }
        let mut observed = Vec::new();
        let progress = queue.collect_limited_retry(&CompletedValues::all(), 3, 3, |item| {
            if item == 0 {
                Err(item)
            } else {
                observed.push(item);
                Ok(())
            }
        });
        assert_eq!(
            progress,
            CollectionProgress {
                scanned: 3,
                destroyed: 2
            }
        );
        assert_eq!(observed, [1, 2]);
        assert_eq!(queue.len(), 2);
        queue.collect_limited_retry(&CompletedValues::all(), 3, 3, |item| {
            if item == 0 {
                Err(item)
            } else {
                observed.push(item);
                Ok(())
            }
        });
        assert_eq!(observed, [1, 2, 3]);
        assert_eq!(queue.len(), 1);
        queue.collect(&CompletedValues::all(), &mut observed);
        assert_eq!(observed, [1, 2, 3, 0]);
    }
    #[test]
    fn unwind_restores_previously_busy_nodes() {
        let queue = Retirement::new();
        queue.try_enqueue(SyncSet::EMPTY, 0).unwrap();
        queue.try_enqueue(SyncSet::EMPTY, 1).unwrap();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            queue.collect_limited_retry(&CompletedValues::all(), 8, 8, |item| {
                if item == 0 {
                    Err(item)
                } else {
                    panic!("later destructor")
                }
            });
        }));
        assert_eq!(queue.len(), 1);
        let mut observed = Vec::new();
        queue.collect(&CompletedValues::all(), &mut observed);
        assert_eq!(observed, [0]);
    }
}

#[cfg(test)]
mod cross_round_busy_fairness_tests {
    use super::*;
    #[test]
    fn one_scan_rounds_progress_past_busy_owners_and_retry_fifo() {
        let queue = Retirement::new();
        // More busy owners than the entire per-round allowance.
        for item in 0..9 {
            queue.try_enqueue(SyncSet::EMPTY, item).unwrap();
        }
        let after = SyncSet::from_point(super::super::SyncPoint {
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
}
