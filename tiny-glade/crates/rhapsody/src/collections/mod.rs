//! Shared bounded containers and token hashing. These utilities have no render
//! graph or native-resource dependencies.
internal_mod!(cache);
pub(crate) mod atomic_snapshot;
pub(crate) mod cpu_pool;
pub mod cpu_scratch;
pub mod token_map;
