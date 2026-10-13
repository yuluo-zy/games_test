//! weather.rs — 天气资源（WP21）。
//!
//! O：PDB 实体 `Weather`；`impl$0::default` 内联。U。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// `Weather`（O 名；天气状态，U；audio weather 消费）。
#[derive(Resource, Clone, Debug, Default)]
pub struct Weather(pub Gap);
