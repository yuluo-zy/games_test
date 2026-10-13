//! vao_library.rs — 顶点缓冲对象注册表（O）。
//!
//! 源 `asset_libraries/vao_library.rs`；实体 `AssetVAOLibrary`。
//! `AssetVAOLibrary::new`（0x140924030）/ `add`（0x140924180，含 `build_vao`）/
//! `rebuild_vao`（0x1409243f0）/ `remove`（0x1409247a0）/ `has_vao`（内联于
//! `vao_update::rebuild_vaos`）。
//! 容器 O：`HashMap<Handle<Mesh>, Vao>`（hashbrown RawTable；add 先查重，缺则
//! `HANDLE_COUNT++` 分配并 `build_vao`）。Vao 本体属 wp2 `render::vao`。

#![allow(dead_code)]

use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::boundary::Vao;
use crate::typed_gap::Gap;

/// VAO 注册表（O：`Res<AssetVAOLibrary>`）。
#[derive(Resource, Default)]
pub struct AssetVAOLibrary {
    /// 网格句柄 → Vao（O）。
    pub vaos: HashMap<u64, Vao>,
    pub _gap: Gap,
}

impl AssetVAOLibrary {
    /// `new`（O 0x140924030）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `has_vao`（O：`rebuild_vaos` 内联）。
    pub fn has_vao(&self, mesh_handle: u64) -> bool {
        self.vaos.contains_key(&mesh_handle)
    }

    /// `add(mesh_handle)`（O 0x140924180）：已存在 → 早退；否则建 `Vao` 入表。
    pub fn add(&mut self, mesh_handle: u64) -> Result<(), ()> {
        if self.vaos.contains_key(&mesh_handle) {
            return Ok(());
        }
        // build_vao（wp2 恢复；顶点属性来源 U）。
        self.vaos.insert(mesh_handle, Vao {
            bindings: Vec::new(),
            _gap: Gap,
        });
        Ok(())
    }

    /// `rebuild_vao(mesh_handle)`（O 0x1409243f0）：按网格重建绑定。
    pub fn rebuild_vao(&mut self, _mesh_handle: u64) {
        // U：绑定差异重建（typed gap）。
    }

    /// `remove(mesh_handle)`（O 0x1409247a0）。
    pub fn remove(&mut self, mesh_handle: &u64) -> Option<Vao> {
        self.vaos.remove(mesh_handle)
    }
}
