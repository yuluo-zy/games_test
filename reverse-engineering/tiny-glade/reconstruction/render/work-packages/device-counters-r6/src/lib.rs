use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueIdentity {
    Primary,
    Auxiliary(u32),
}

impl QueueIdentity {
    pub fn counter_slot(self) -> usize {
        match self {
            Self::Primary => 0,
            Self::Auxiliary(index) => index.wrapping_add(1) as usize,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueFamily {
    pub flags: u32,
    pub queue_count: u32,
    pub family_index: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueuePlan {
    pub primary_family: u32,
    pub auxiliary_family: Option<u32>,
    pub auxiliary_count: u32,
}

impl QueuePlan {
    pub fn select(families: &[QueueFamily]) -> Option<Self> {
        let primary = families.iter().find(|family| family.flags & 1 != 0)?;
        let auxiliary = families.iter().find(|family| family.flags & 3 == 2);
        Some(Self {
            primary_family: primary.family_index,
            auxiliary_family: auxiliary.map(|family| family.family_index),
            auxiliary_count: auxiliary.map_or(0, |family| family.queue_count),
        })
    }

    pub fn counters(&self) -> DeviceCounters {
        DeviceCounters::new(self.auxiliary_count)
    }
}

pub struct DeviceCounters {
    slots: Box<[AtomicU64]>,
}

impl DeviceCounters {
    pub fn new(auxiliary_count: u32) -> Self {
        let slots = (0..u64::from(auxiliary_count) + 1)
            .map(|_| AtomicU64::new(1))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn begin_sequence(&self, queue: QueueIdentity) -> Option<u64> {
        self.slots
            .get(queue.counter_slot())
            .map(|counter| counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn next_value(&self, queue: QueueIdentity) -> Option<u64> {
        self.slots
            .get(queue.counter_slot())
            .map(|counter| counter.load(Ordering::Relaxed))
    }
}

pub fn raw_begin_projection(
    counters: &mut [u64],
    queue_tag: u32,
    auxiliary_index: u32,
) -> Option<u64> {
    let slot = if queue_tag & 1 == 0 {
        0
    } else {
        auxiliary_index.wrapping_add(1) as usize
    };
    counters.get_mut(slot).map(|counter| {
        let sequence = *counter;
        *counter = counter.wrapping_add(1);
        sequence
    })
}

#[derive(Default)]
pub struct BucketKeyWitness {
    live: BTreeMap<u64, QueueIdentity>,
}

impl BucketKeyWitness {
    pub fn insert(&mut self, sequence: u64, queue: QueueIdentity) -> Option<QueueIdentity> {
        self.live.insert(sequence, queue)
    }

    pub fn remove(&mut self, sequence: u64) -> Option<QueueIdentity> {
        self.live.remove(&sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_initializes_all_queue_slots_to_one() {
        for auxiliary_count in [0, 1, 2, 8] {
            let counters = DeviceCounters::new(auxiliary_count);
            assert_eq!(counters.len(), auxiliary_count as usize + 1);
            assert_eq!(counters.next_value(QueueIdentity::Primary), Some(1));
            for index in 0..auxiliary_count {
                assert_eq!(
                    counters.next_value(QueueIdentity::Auxiliary(index)),
                    Some(1)
                );
            }
        }
    }

    #[test]
    fn primary_and_auxiliary_have_independent_sequences() {
        let counters = DeviceCounters::new(2);
        assert_eq!(counters.begin_sequence(QueueIdentity::Primary), Some(1));
        assert_eq!(counters.begin_sequence(QueueIdentity::Primary), Some(2));
        assert_eq!(
            counters.begin_sequence(QueueIdentity::Auxiliary(0)),
            Some(1)
        );
        assert_eq!(
            counters.begin_sequence(QueueIdentity::Auxiliary(1)),
            Some(1)
        );
        assert_eq!(counters.next_value(QueueIdentity::Primary), Some(3));
    }

    #[test]
    fn global_bucket_collision_replaces_before_original_assertion() {
        let counters = DeviceCounters::new(1);
        let mut witness = BucketKeyWitness::default();
        let primary = counters.begin_sequence(QueueIdentity::Primary).unwrap();
        let auxiliary = counters
            .begin_sequence(QueueIdentity::Auxiliary(0))
            .unwrap();
        assert_eq!(primary, auxiliary);
        assert_eq!(witness.insert(primary, QueueIdentity::Primary), None);
        assert_eq!(
            witness.insert(auxiliary, QueueIdentity::Auxiliary(0)),
            Some(QueueIdentity::Primary)
        );
        assert_eq!(witness.remove(1), Some(QueueIdentity::Auxiliary(0)));
    }

    #[test]
    fn family_selection_matches_graphics_then_compute_only_scan() {
        let families = [
            QueueFamily {
                flags: 3,
                queue_count: 4,
                family_index: 7,
            },
            QueueFamily {
                flags: 6,
                queue_count: 3,
                family_index: 9,
            },
            QueueFamily {
                flags: 2,
                queue_count: 1,
                family_index: 11,
            },
        ];
        assert_eq!(
            QueuePlan::select(&families),
            Some(QueuePlan {
                primary_family: 7,
                auxiliary_family: Some(9),
                auxiliary_count: 3,
            })
        );
    }

    #[test]
    fn auxiliary_index_is_not_queue_family_index() {
        let counters = DeviceCounters::new(1);
        assert_eq!(counters.begin_sequence(QueueIdentity::Auxiliary(9)), None);
        assert_eq!(
            counters.begin_sequence(QueueIdentity::Auxiliary(0)),
            Some(1)
        );
    }

    #[test]
    fn raw_counter_addition_wraps() {
        let mut counters = [u64::MAX, 1];
        assert_eq!(raw_begin_projection(&mut counters, 0, 0), Some(u64::MAX));
        assert_eq!(counters, [0, 1]);
        assert_eq!(raw_begin_projection(&mut counters, 1, u32::MAX), Some(0));
        assert_eq!(counters, [1, 1]);
    }
}
