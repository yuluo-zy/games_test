//! preplaced_autoclutter.rs — 预放置自动杂饰（WP21）。
//!
//! O：PDB 实体 `PreplacedAutoClutter`；`impl$1::default` 内联。U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `PreplacedAutoClutter`（O 名；布局 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PreplacedAutoClutter(pub Gap);
