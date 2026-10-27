//! Descriptor slot table with free-list allocation.
use crate::Error;
use crate::Result;
use std::collections::VecDeque;
use super::BindlessHeap;
use super::kind::BindlessIndex;
use super::kind::DescriptorKind;
use super::lease::BindlessLease;
use super::stats::BindlessStats;
use super::stats::BindlessTableStats;

pub(super) enum SlotState {
    Free,
    Live,
    Exhausted,
}

pub(super) struct Slot {
    pub(super) generation: u64,
    state: SlotState,
}

/// A slot stays live while any CPU or submitted-command Arc holds its lease.
/// The last lease drop releases it; completion belongs to the command owner.
pub(super) struct Slots {
    live: usize,
    peak_live: usize,
    table_live: [usize; 4],
    table_exhausted: [usize; 4],
    table_peak_live: [usize; 4],
    pub(super) entries: [Vec<Slot>; 4],
    pub(super) free: [VecDeque<u32>; 4],
}

impl Slots {
    pub(super) fn new(counts: [u32; 4]) -> Result<Self> {
        let mut entries = std::array::from_fn(|_| Vec::new());
        let mut free = std::array::from_fn(|_| VecDeque::new());
        for table in 0..4 {
            let count = counts[table] as usize;
            entries[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            free[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            entries[table].resize_with(count, || Slot {
                generation: 0,
                state: SlotState::Free,
            });
            free[table].extend(0..counts[table]);
        }
        Ok(Self {
            live: 0,
            peak_live: 0,
            table_live: [0; 4],
            table_exhausted: [0; 4],
            table_peak_live: [0; 4],
            entries,
            free,
        })
    }
    pub(super) fn statistics(&self) -> BindlessStats {
        BindlessStats {
            peak_live: self.peak_live,
            capacity: self.entries.iter().map(Vec::len).sum(),
            live: self.live,
            free: self.free.iter().map(VecDeque::len).sum(),
            tables: DescriptorKind::ALL.map(|kind| self.table_stats(kind)),
        }
    }
    pub(super) fn host_bytes(&self) -> Result<u64> {
        Ok(crate::accounting::shared_cpu_bytes::<BindlessHeap>()?
            + self.entries.iter().map(Vec::len).sum::<usize>() as u64
                * crate::accounting::shared_cpu_bytes::<BindlessLease>()?
            + (self
                .entries
                .iter()
                .map(|v| v.capacity() * std::mem::size_of::<Slot>())
                .sum::<usize>()
                + self
                    .free
                    .iter()
                    .map(|v| v.capacity() * std::mem::size_of::<u32>())
                    .sum::<usize>()
) as u64)
    }
    pub(super) fn allocate(&mut self, kind: DescriptorKind) -> Result<BindlessIndex> {
        let table = kind as usize;
        while let Some(index) = self.free[table].pop_front() {
            let slot = &mut self.entries[table][index as usize];
            let Some(generation) = slot.generation.checked_add(1) else {
                slot.state = SlotState::Exhausted;
                self.table_exhausted[table] += 1;
                continue;
            };
            slot.generation = generation;
            slot.state = SlotState::Live;
            self.live += 1;
            self.peak_live = self.peak_live.max(self.live);
            self.table_live[table] += 1;
            self.table_peak_live[table] = self.table_peak_live[table].max(self.table_live[table]);
            return Ok(BindlessIndex {
                kind,
                index,
                generation,
            });
        }
        Err(Error::exhausted(format!(
            "bindless {kind:?} descriptor table is full (live={} free={} exhausted={} capacity={}; heap_live={})",
            self.table_live[table], self.free[table].len(), self.table_exhausted[table],
            self.entries[table].len(), self.live,
        )))
    }
    pub(super) fn valid(&self, index: BindlessIndex) -> bool {
        self.entries[index.kind as usize]
            .get(index.index as usize)
            .is_some_and(|slot| {
                slot.generation == index.generation && matches!(slot.state, SlotState::Live)
            })
    }
    pub(super) fn release(&mut self, index: BindlessIndex) {
        debug_assert!(self.valid(index));
        let table = index.kind as usize;
        self.live -= 1;
        self.table_live[table] -= 1;
        self.entries[table][index.index as usize].state = SlotState::Free;
        self.free[table].push_back(index.index);
    }
    pub(super) fn table_stats(&self, kind: DescriptorKind) -> BindlessTableStats {
        let table = kind as usize;
        BindlessTableStats {
            capacity: self.entries[table].len(),
            live: self.table_live[table],
            free: self.free[table].len(),
            exhausted: self.table_exhausted[table],
            peak_live: self.table_peak_live[table],
        }
    }

}
