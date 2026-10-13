//! roof_trigger_volume.rs — RoofTriggerVolume（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_trigger_volume.rs`。命名实体 `RoofTriggerVolume`（**组件**，
//! create_roof spawn `RoofTriggerVolume::default`）。72B；3 集合（triangles 36B/三角、
//! normals、uvs）。函数：`default`（0x1408e4520）、`triangles`（0x1408e4560）、
//! `set_triangles`（0x1408e45b0）、`set_normals`（0x1408e45f0）、`set_uvs`（0x1408e4660）、
//! `get_position_from_mesh_uv`（0x1408e46d0）。生产者：`calculate_roof_trigger_volumes`
//! （0x1421a3220，systems/roof）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// 屋顶触发体（碰撞/交互用）。集合容器细节 U。
#[derive(Component, Clone, Debug, Default)]
pub struct RoofTriggerVolume {
    pub triangles: Gap,
    pub normals: Gap,
    pub uvs: Gap,
}

impl RoofTriggerVolume {
    /// RoofTriggerVolume::default（O：0x1408e4520）：三集合初值 `{0,4,0}`。
    pub fn new() -> Self {
        Self::default()
    }
    /// triangles（O：0x1408e4560）：返回 `(ptr, ptr+len·36)` 范围；空 panic。
    pub fn triangles(&self) -> (u64, u64) {
        // 空集合 → 原函数 panic；此处 typed gap 不臆造长度。
        let _ = &self.triangles;
        (0, 0)
    }
    /// set_triangles（O：0x1408e45b0）。
    pub fn set_triangles(&mut self, _data: Gap) {
        self.triangles = Gap;
    }
    /// set_normals（O：0x1408e45f0；断言长度==三角数）。
    pub fn set_normals(&mut self, _data: Gap) {
        self.normals = Gap;
    }
    /// set_uvs（O：0x1408e4660；断言长度==三角数）。
    pub fn set_uvs(&mut self, _data: Gap) {
        self.uvs = Gap;
    }
    /// get_position_from_mesh_uv（O：0x1408e46d0）→ Option<[f32;3]>（碰撞网格工具链 U）。
    pub fn get_position_from_mesh_uv(&self, _uv: [f32; 2]) -> Option<[f32; 3]> {
        None
    }
}
