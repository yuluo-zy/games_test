//! `indirect_params.rs` — indirect-args buffer type markers.
//!
//! Business logic (contracts/indirect-params.md §1): the module has **no
//! standalone functions** (PDB functions=0). The 6 types are zero-sized SSBO
//! carriers used as generic parameters by `AssetSsboLibrary::add_new<T>()` /
//! `get_rg_buffer<T>()` / `pass_resource_access::{read,write}_access_buffer<T>()` /
//! `ComputePass::{read,write}_storage_buffer<T>()` / `render::ssbo::TypedStorageBuffer<T>`
//! (all O via procedure_occurrences).

use crate::typed_gap::Gap;

/// 原子计数缓冲（GPU indirect-args 原子计数器）。O 类型名；元素/尺寸 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct AtomicCounterBuffer(pub Gap);

/// `VkDispatchIndirectCommand` 缓冲（12B/条）。O 类型名。
#[derive(Clone, Copy, Debug, Default)]
pub struct DispatchIndirectBuffer(pub Gap);

/// `VkDrawIndirectCommand` 缓冲（16B/条）。O 类型名。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndirectBuffer(pub Gap);

/// `VkDrawIndexedIndirectCommand` 缓冲（20B/条）。O 类型名。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndexedIndirectBuffer(pub Gap);

/// 分桶 DrawIndirect 子数组（nani GPU-driven 分桶）。O 类型名；布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndirectBucket(pub Gap);

/// 分桶 DrawIndexedIndirect 子数组（nani GPU-driven 分桶）。O 类型名；布局 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawIndexedIndirectBucket(pub Gap);
