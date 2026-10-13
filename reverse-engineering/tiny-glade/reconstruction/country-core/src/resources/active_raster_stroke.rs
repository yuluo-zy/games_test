//! active_raster_stroke.rs — 活动栅格化笔画（WP8 恢复）。
//!
//! 源：B skeleton `resources/active_raster_stroke.rs`。命名实体：`ActiveRasterStroke`、
//! `ActiveRasterStrokesStorage`（资源）、`InProgressRasterStroke`。
//! 方法行号：`impl$0::index`@23、`impl$1::index_mut`@29、`impl$6::default`@14、`impl$2::default`@47、
//! `impl$8::default`@89、`ActiveRasterStroke::resample_and_smooth`@74、`InProgressRasterStroke::last`@103、
//! `impl$4::solidify_fluid_into::closure$0`@143。
//! `rasterize_terrain_stroke`@0x1408c7120 读取 active 笔画（Vec@+0x10/+0x30 与 flag@+0x40，O）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::typed_gap::Gap;

/// 活动栅格化笔画（O 名；`ActiveRasterStroke`）。布局 U。
#[derive(Clone, Debug, Default)]
pub struct ActiveRasterStroke {
    /// typed gap：{点列, 宽度, 模式} 字段 U。
    pub _gap: Gap,
}

impl ActiveRasterStroke {
    /// 重采样并平滑（O 名；`resample_and_smooth`@74）。
    pub fn resample_and_smooth(&mut self, _spacing: f32) {
        unimplemented!("recover resample_and_smooth")
    }
}

/// 进行中笔画（O 名；`InProgressRasterStroke`）。布局 U。
#[derive(Clone, Debug, Default)]
pub struct InProgressRasterStroke {
    /// typed gap：{点列} 字段 U。
    pub _gap: Gap,
}

impl InProgressRasterStroke {
    /// 末点（O 名；`last`@103）。
    pub fn last(&self) -> Option<&Vec3> {
        None
    }
}

/// 活动笔画存储资源（O 名；`ActiveRasterStrokesStorage`）。
/// `rasterize_terrain_stroke` 观察：笔画数组 + 两 Vec + 栅格化 flag（I）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ActiveRasterStrokesStorage {
    /// typed gap：{笔画列表, flag} 字段 U。
    pub _gap: Gap,
}

impl std::ops::Index<usize> for ActiveRasterStrokesStorage {
    type Output = ActiveRasterStroke;
    fn index(&self, _index: usize) -> &ActiveRasterStroke {
        unimplemented!("recover index")
    }
}

impl std::ops::IndexMut<usize> for ActiveRasterStrokesStorage {
    fn index_mut(&mut self, _index: usize) -> &mut ActiveRasterStroke {
        unimplemented!("recover index_mut")
    }
}
