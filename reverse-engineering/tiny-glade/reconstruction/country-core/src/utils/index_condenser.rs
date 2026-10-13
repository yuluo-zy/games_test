//! utils/index_condenser.rs — `utils::index_condenser::IndexCondenser<T>`
//! (O field of `OwnedGpuWorldSubset`; wp3 gpu_world).

use core::marker::PhantomData;

/// Typed-gap condenser: chunk-id vector + index compaction (wp3 gpu_world).
pub struct IndexCondenser<T> {
    pub _gap: crate::Gap,
    pub _marker: PhantomData<fn() -> T>,
}
