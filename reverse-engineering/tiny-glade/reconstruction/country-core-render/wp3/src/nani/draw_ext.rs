//! draw_ext.rs — DrawBuilder 的 nani 扩展（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `impl$0::nani_draw` 0x140904ae0（行 19，1185B）。
//! 完整契约见 contracts/draw_ext.md。DrawBuilder 本体属 WP1（pass_builder），此处只引用。

#![allow(dead_code, unused_imports)]

use crate::render::nani::buffers::{NaniDrawListLengthSsbo, NaniDrawListSsbo, NaniInstanceIndirectionSsbo};
use crate::render::nani::gpu_world::{GpuWorld, GpuWorldSubsetKind};
use crate::render::pass_builder::draw_builder::DrawBuilder;

/// nani 绘制扩展（`impl$0::nani_draw`，O）。
pub trait NaniDrawExt {
    /// 把 kind 对应 subset 的间接绘制接到 DrawBuilder（0x140904ae0）：
    /// 1. 取 GpuWorld 资源（`get_resource_id` hash 0x662978458699a5ef）。
    /// 2. 锁 subset 槽（+0xc0*kind，RwLock CAS）。
    /// 3. 绑定 NaniDrawListLengthSsbo / NaniDrawListSsbo / NaniInstanceIndirectionSsbo。
    /// 4. 细粒度分派：`NANI_FORCE_FINE_GRAINED_SORTING==0 && (kind>=2 || NO_FINE_GRAINED)` →
    ///    `draw_indexed_indirect_raw`，否则 `draw_indexed_indirect_count`。
    fn nani_draw(self, kind: GpuWorldSubsetKind) -> DrawBuilder;
}

impl NaniDrawExt for DrawBuilder {
    fn nani_draw(self, _kind: GpuWorldSubsetKind) -> DrawBuilder {
        todo!("recover nani_draw; see contracts/draw_ext.md §1")
    }
}

/// 编译期开关（public-symbols，O）：强制细粒度排序。
pub const NANI_FORCE_FINE_GRAINED_SORTING: bool = false;
/// 编译期开关（O）：禁止细粒度排序。
pub const NANI_FORCE_NO_FINE_GRAINED_SORTING: bool = false;

/// 引用类型：仅用于名称对齐（GpuWorld 资源类型存在于 ECS）。
pub type GpuWorldResource = GpuWorld;
