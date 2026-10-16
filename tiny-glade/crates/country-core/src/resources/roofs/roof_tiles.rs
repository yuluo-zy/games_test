//! `roof_tiles.rs` — RoofTile 组件（B `resources/roofs/roof_tiles.rs`，2 业务全胶水）。
//!
//! 本文件只含组件声明/注册面（HIGHLIGHTS #9：Bevy `register_required_components`
//! 复用标准模式，不计独立业务）；瓦片实例的实际 CPU 生成（assemble 圆形/矩形、
//! RoofShingle 64 字节 GPU 记录）已由共享 `system-roof` 的 tiles 模块恢复
//! （11,467 片圆形瓦 0 差异，`evidence/roof-tiles/`）。
//! `clone_behavior`（0x140002370）= 跨 6+ 域共享的 `xor eax,eax; ret` 空桩（返回 None）。
use bevy_ecs::component::Component;

/// 屋顶瓦片实例组件（B `RoofTile`）。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTile {
    /// 共享空桩不携带业务状态；瓦片记录本体在 GPU/装配链中。
    pub(crate) _marker: (),
}
