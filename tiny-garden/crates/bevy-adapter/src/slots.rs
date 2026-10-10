//! 所有生成类型共享的槽位租约。结果尚未被主世界收集时不会提前释放调度槽位。
use bevy_ecs::prelude::Resource;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
#[derive(Resource)]
pub struct GenerationSlots {
    active: Arc<AtomicUsize>,
    max: usize,
}
pub struct GenerationSlot(Arc<AtomicUsize>);
impl Drop for GenerationSlot {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}
impl GenerationSlots {
    pub fn new(max: usize) -> Self {
        Self {
            active: Arc::new(AtomicUsize::new(0)),
            max,
        }
    }
    pub fn try_acquire(&self) -> Option<GenerationSlot> {
        let mut n = self.active.load(Ordering::Acquire);
        while n < self.max {
            match self
                .active
                .compare_exchange_weak(n, n + 1, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return Some(GenerationSlot(self.active.clone())),
                Err(actual) => n = actual,
            }
        }
        None
    }
    pub fn active(&self) -> usize {
        self.active.load(Ordering::Acquire)
    }
}
