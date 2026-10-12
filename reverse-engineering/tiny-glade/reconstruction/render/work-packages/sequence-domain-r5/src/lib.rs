use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct QueueIdentity { pub tag: u32, pub aux_index: u32 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pending { pub queue: QueueIdentity, pub cb_sequence: u64, pub timeline: u64 }

#[derive(Debug, Eq, PartialEq)]
pub enum Error { SequenceCollision(u64), QueueMismatch, TimelineRegression, MissingSequence(u64) }

#[derive(Default)]
pub struct SequenceDomain {
    counters: BTreeMap<QueueIdentity, u64>,
    buckets: BTreeSet<u64>,
    owners: BTreeMap<u64, QueueIdentity>,
    pending: BTreeMap<QueueIdentity, Vec<Pending>>,
    completed: BTreeMap<QueueIdentity, u64>,
}

impl SequenceDomain {
    pub fn begin_cb(&mut self, queue: QueueIdentity) -> Result<u64, Error> {
        let sequence = *self.counters.entry(queue).or_insert(0);
        if !self.buckets.insert(sequence) { return Err(Error::SequenceCollision(sequence)); }
        self.owners.insert(sequence, queue);
        *self.counters.get_mut(&queue).unwrap() += 1;
        Ok(sequence)
    }

    pub fn submit(&mut self, queue: QueueIdentity, cb_sequence: u64, timeline: u64) -> Result<(), Error> {
        if !self.buckets.contains(&cb_sequence) { return Err(Error::MissingSequence(cb_sequence)); }
        if self.owners.get(&cb_sequence).copied() != Some(queue) { return Err(Error::QueueMismatch); }
        let completed = self.completed.get(&queue).copied().unwrap_or(0);
        if timeline < completed { return Err(Error::TimelineRegression); }
        self.pending.entry(queue).or_default().push(Pending { queue, cb_sequence, timeline });
        Ok(())
    }

    pub fn complete(&mut self, queue: QueueIdentity, timeline: u64) -> Result<Vec<u64>, Error> {
        let prior = self.completed.get(&queue).copied().unwrap_or(0);
        if timeline < prior { return Err(Error::TimelineRegression); }
        self.completed.insert(queue, timeline);
        let list = self.pending.entry(queue).or_default();
        let split = list.partition_point(|p| p.timeline <= timeline);
        let ready = list.drain(..split).map(|p| p.cb_sequence).collect();
        Ok(ready)
    }

    pub fn dispose(&mut self, cb_sequence: u64) -> Result<(), Error> {
        if !self.buckets.remove(&cb_sequence) { return Err(Error::MissingSequence(cb_sequence)); }
        self.owners.remove(&cb_sequence);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn timeline_gates_cb() {
        let q = QueueIdentity { tag: 0, aux_index: 0 };
        let mut d = SequenceDomain::default();
        let cb = d.begin_cb(q).unwrap();
        d.submit(q, cb, 4).unwrap();
        assert!(d.complete(q, 3).unwrap().is_empty());
        assert_eq!(d.complete(q, 4).unwrap(), vec![cb]);
        d.dispose(cb).unwrap();
    }
    #[test] fn cross_queue_sequence_collision_is_explicit() {
        let mut d = SequenceDomain::default();
        let a = QueueIdentity { tag: 0, aux_index: 0 };
        let b = QueueIdentity { tag: 1, aux_index: 0 };
        assert_eq!(d.begin_cb(a).unwrap(), 0);
        assert_eq!(d.begin_cb(b), Err(Error::SequenceCollision(0)));
    }
}
