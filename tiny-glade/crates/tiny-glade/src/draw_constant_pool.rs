//! CPU storage returned when Commit replaces a batch's owned constant lease,
//! including cancellation/fallback. GPU constants have their own completion pool.
use parking_lot::Mutex;
use std::sync::Arc;

const IDLE_BUDGET: usize = 16 << 20;
const MAX_IDLE_BLOCKS: usize = 1024;

#[derive(Default)]
struct Idle {
    blocks: Vec<Vec<u8>>,
    bytes: usize,
}

#[derive(Default)]
pub(crate) struct ConstantPool(Mutex<Idle>);

impl ConstantPool {
    pub(crate) fn take(&self, size: usize) -> Vec<u8> {
        let mut idle = self.0.lock();
        let reused = idle.blocks.iter().position(|block| block.capacity() == size)
            .map(|index| {
                let block = idle.blocks.swap_remove(index);
                idle.bytes -= block.capacity();
                block
            });
        drop(idle);
        let Some(mut bytes) = reused else { return vec![0; size]; };
        bytes.resize(size, 0);
        bytes.fill(0); // Unset optional uniforms must never retain a prior draw.
        bytes
    }

    pub(crate) fn recycle(&self, bytes: Vec<u8>) {
        if bytes.capacity() == 0 || bytes.capacity() > IDLE_BUDGET { return; }
        let mut idle = self.0.lock();
        if idle.blocks.len() >= MAX_IDLE_BLOCKS { return; }
        if idle.blocks.try_reserve_exact(1).is_err() { return; }
        let index_bytes = idle.blocks.capacity() * std::mem::size_of::<Vec<u8>>();
        // Capacity of the index and of each retained block counts, even if
        // a caller cleared its length. Release excess storage before returning.
        while idle.bytes.saturating_add(index_bytes).saturating_add(bytes.capacity()) > IDLE_BUDGET {
            let Some(old) = idle.blocks.pop() else { return; };
            idle.bytes -= old.capacity();
        }
        idle.bytes += bytes.capacity();
        idle.blocks.push(bytes);
    }

    pub(crate) fn own(self: &Arc<Self>, bytes: Vec<u8>) -> CollectedConstants {
        CollectedConstants { bytes, pool: self.clone() }
    }
}

pub(crate) struct CollectedConstants {
    pub(crate) bytes: Vec<u8>,
    pool: Arc<ConstantPool>,
}
impl Drop for CollectedConstants {
    fn drop(&mut self) { self.pool.recycle(std::mem::take(&mut self.bytes)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cancelled_owned_constants_return_storage_and_clear_unset_uniforms() {
        let pool = Arc::new(ConstantPool::default());
        let mut bytes = pool.take(128);
        bytes.fill(0xab);
        let address = bytes.as_ptr();
        drop(pool.own(bytes));
        let bytes = pool.take(128);
        assert_eq!(bytes.as_ptr(), address);
        assert!(bytes.iter().all(|byte| *byte == 0));
    }
    #[test]
    fn capacity_and_index_stay_bounded_when_lengths_are_cleared() {
        let pool = ConstantPool::default();
        for _ in 0..100 {
            pool.recycle(Vec::with_capacity(1 << 20));
        }
        pool.recycle(Vec::with_capacity(IDLE_BUDGET + 1));
        let idle = pool.0.lock();
        assert!(idle.bytes + idle.blocks.capacity() * std::mem::size_of::<Vec<u8>>() <= IDLE_BUDGET);
        assert_eq!(idle.bytes, idle.blocks.iter().map(Vec::capacity).sum::<usize>());
        assert!(idle.blocks.iter().all(Vec::is_empty));
    }
}
