//! Table statistics and occupancy reports.
use super::BindlessHeap;
use super::kind::DescriptorKind;

/// 单类描述符的寿命账目；统计只读取计数器，不扫描整张描述符表。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BindlessTableStats {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    /// generation 已溢出的槽永久不可复用，不能计入可分配容量。
    pub exhausted: usize,
    pub peak_live: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessStats {
    pub peak_live: usize,
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    /// 顺序与 DescriptorKind 一致；优先通过 table() 按类别读取。
    pub tables: [BindlessTableStats; 4],
}

impl BindlessStats {
    pub fn table(&self, kind: DescriptorKind) -> BindlessTableStats {
        self.tables[kind as usize]
    }
}

/// Current occupancy sampled independently from the pool's physical heaps.
/// No global historical peak or simultaneous cross-heap snapshot is maintained.
#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessOccupancy {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    pub tables: [BindlessTableOccupancy; 4],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessTableOccupancy {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    pub exhausted: usize,
}

impl BindlessOccupancy {
    pub fn table(&self, kind: DescriptorKind) -> BindlessTableOccupancy { self.tables[kind as usize] }
    pub(super) fn include(&mut self, page: BindlessStats) {
        self.capacity += page.capacity;
        self.live += page.live;
        self.free += page.free;
        for (sum, value) in self.tables.iter_mut().zip(page.tables) {
            sum.capacity += value.capacity;
            sum.live += value.live;
            sum.free += value.free;
            sum.exhausted += value.exhausted;
        }
    }
}

impl BindlessHeap {
    pub fn stats(&self) -> BindlessStats {
        self.slots.lock().statistics()
    }
}
