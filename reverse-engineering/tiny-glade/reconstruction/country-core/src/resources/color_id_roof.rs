//! `resources::color_id_roof` — 屋顶颜色 ID SSBO（O 名，字段 typed gap）。
//!
//! 原符号：`country_core::resources::color_id_roof::RoofColorIdSsbo`
//! （draw_bricks / instanced_roof 反编译 `read_storage_buffer<RoofColorIdSsbo>`）。
//! wp9 的 `color_id_roof` 已恢复值侧（颜色查询），共享 crate 尚未承载该 SSBO；
//! 本束以 typed-gap 载体补齐。

use crate::typed_gap::Gap;

/// 屋顶颜色 ID SSBO（O：`read_storage_buffer<RoofColorIdSsbo>`）。
#[derive(Clone, Debug, Default)]
pub struct RoofColorIdSsbo(pub Gap);
