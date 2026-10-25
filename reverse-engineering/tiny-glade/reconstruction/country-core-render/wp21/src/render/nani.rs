//! render/nani.rs — nani mod 胶水（WP21）。
//!
//! 骨架 `render/nani.rs`；PDB 实体 `GpuDrivenDrawable` / `InstanceHandle`；函数
//! `InstanceHandle::subset`、`impl$3::component_ids`、`impl$5::get_components`、
//! `impl$2::clone`。共享 crate `render/nani/`（wp1-6 gpu_world/mesh_atlas）已恢复；
//! 本文件为同面骨架（handle/组件 id 表面）。

use crate::typed_gap::Gap;

/// `InstanceHandle`（O：nani GPU 实例句柄；`subset` 0x?? 选择子集）。
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstanceHandle(pub Gap);

impl InstanceHandle {
    /// `subset`（O 名：按子集筛选句柄）。
    pub fn subset(&self, _kind: u8) -> Self {
        self.clone()
    }
    /// `impl$3::component_ids`（O 名：ECS 组件 id）。
    pub fn component_ids() -> Gap {
        Gap::default()
    }
    /// `impl$5::get_components`（O 名）。
    pub fn get_components(_handle: &InstanceHandle) -> Gap {
        Gap::default()
    }
}

/// `GpuDrivenDrawable`（O：GPU 驱动可绘制组件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuDrivenDrawable(pub Gap);
