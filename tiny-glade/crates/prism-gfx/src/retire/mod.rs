//! Pre-reserved retirement nodes in per-queue ordered trees.
//! Admission happens before native creation; enqueue and cross-queue movement
//! only transfer existing owners. Destruction happens outside all queue locks.
use crate::sync::{CompletedValues, MAX_QUEUES, QueueId, SyncSet};
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[doc(hidden)]
pub struct Node<T> {
    value: u64,
    after: SyncSet,
    item: Option<T>,
    left: Option<RetirementTicket<T>>,
    right: Option<RetirementTicket<T>>,
    height: u8,
}

/// Exclusive construction credit. Not Clone: one native payload consumes one
/// node, or cancellation returns the node and its host-budget credit.
#[doc(hidden)]
pub struct RetirementTicket<T> {
    // One fallibly allocated node; release physical storage before its credit.
    storage: Vec<Node<T>>,
    _memory: Option<crate::accounting::CpuMemoryReservation>,
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
    #[doc(hidden)]
    pub fn reserve(
        stats: &Arc<crate::accounting::AllocatorStats>,
    ) -> crate::Result<Self> {
        let memory = stats.reserve_cpu_bytes(
            crate::accounting::cpu_vec_bytes::<Node<T>>(1)?,
            crate::accounting::BudgetCategory::Live,
        )?;
        Self::new(Some(memory))
    }
    fn new(mut memory: Option<crate::accounting::CpuMemoryReservation>) -> crate::Result<Self> {
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        if let Some(memory) = &mut memory {
            memory.resize(crate::accounting::cpu_vec_bytes::<Node<T>>(
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
    #[doc(hidden)]
    pub fn enqueue_reserved(&self, after: SyncSet, item: T, ticket: RetirementTicket<T>) {
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
    #[doc(hidden)]
    pub fn collect_limited_retry(
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
mod tests;

#[cfg(test)]
mod node_balance_tests;

#[cfg(test)]
mod retry_tests;

#[cfg(test)]
mod busy_fairness_tests;

#[cfg(test)]
mod cross_round_busy_fairness_tests;
