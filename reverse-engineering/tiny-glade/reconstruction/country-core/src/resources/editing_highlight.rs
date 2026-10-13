//! editing_highlight.rs — 编辑高亮资源（WP12 facade）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 编辑高亮（O：`EditingHighlight`；`wall_highlight` 写 rgba）。
#[derive(Resource, Clone, Debug, Default)]
pub struct EditingHighlight {
    pub raw: Gap,
}
