//! `dormer_tiles.rs` — DormerTiles 资源（B `resources/roofs/dormer_tiles.rs`，1 胶水）。
//!
//! 文件仅 `impl$1::default` 一个派生胶水（`#[derive(Default)]`），无独立业务函数；
//! 天窗瓦生成逻辑在 systems/roof 消费方与共享 `system-roof` 矩形面瓦链中。
//! 章节：`chapters/resources/roofs/dormer_tiles.rs.md`。
use bevy_ecs::resource::Resource;

/// 天窗瓦片集合资源（天窗处生成的瓦片；本文件仅类型声明）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DormerTiles {
    /// 类型占位——原值语义（瓦片句柄集合）在系统消费方，待接线登记。
    pub(crate) _marker: (),
}
