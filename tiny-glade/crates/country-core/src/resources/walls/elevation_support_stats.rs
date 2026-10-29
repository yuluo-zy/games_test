//! 原高程支撑的每墙统计，用于比较重算前后的柱与托架。
use bevy_ecs::resource::Resource;
use std::collections::HashMap;
use super::{WallId, wall_stone_columns::StoneColumn};

/// 原版按墙维护支撑统计；跨会话清理时点尚无充分原证。
#[derive(Resource, Clone, Debug, Default)]
pub struct ElevationSupportStats {
    pub walls: HashMap<WallId, ElevationSupports>,
}

/// 原 0x1407867d0 零高程插入：空柱、三个零值、分类标志 false。
/// 小/大托架值由对应原构造器逐项递增；圆形值由构造器输出，具体统计单位尚未确认。
#[derive(Clone, Debug, Default)]
pub struct ElevationSupports {
    pub columns: Vec<StoneColumn>,
    pub small_brackets: u64,
    pub large_brackets: u64,
    pub circle_output: u64,
    pub is_turret: bool,
}
