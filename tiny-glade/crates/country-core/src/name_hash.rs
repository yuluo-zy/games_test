//! 进程内资源名哈希（O 0x140c9cd80，46 B）：`xxhash_rust::xxh3::xxh3_64`
//! （默认 secret、seed 0），结果即 u64。
//!
//! 唯一实现位于共享 `utils` crate；这里 re-export 以保留 `crate::name_hash` 路径。
pub use ::utils::name_hash::NameHash;
