//! 上下文权威意图及最小决策；派生网格、关系索引和渲染资源不进入此数据。
use crate::{
    BlockId, BuildingId, DomainError,
    strokes::{Point, Stroke, StrokeId, StrokeKind},
    terrain::TerrainDocument,
};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureMode {
    Auto,
    Locked(StrokeKind),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoundaryStyle {
    #[default]
    Auto,
    Timber,
    Stone,
}
/// 兼容旧 Stroke 的语义部分；控制点编号在移动及等点数编辑时保持不变。
#[derive(Debug, Clone, PartialEq)]
pub struct LinearIntent {
    pub mode: StructureMode,
    pub style: BoundaryStyle,
    pub elevation_offset: f64,
    pub control_ids: Vec<u64>,
}
impl LinearIntent {
    pub fn legacy(stroke: &Stroke) -> Self {
        Self {
            mode: StructureMode::Locked(stroke.kind),
            style: BoundaryStyle::Auto,
            elevation_offset: 0.,
            control_ids: (1..=stroke.points.len() as u64).collect(),
        }
    }
    pub fn validate(&self, stroke: &Stroke) -> Result<(), DomainError> {
        let mut ids = self.control_ids.clone();
        ids.sort();
        ids.dedup();
        if !self.elevation_offset.is_finite()
            || self.elevation_offset.abs() > 3.
            || self.control_ids.len() != stroke.points.len()
            || ids.len() != self.control_ids.len()
            || ids.contains(&0)
        {
            return Err(DomainError("invalid linear intent"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpeningId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WallFace {
    Front,
    Back,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallAnchor {
    pub building: BuildingId,
    pub block: BlockId,
    pub face: WallFace,
}
/// 沿墙位置为中心坐标，高度相对体块底部；缩小或宿主消失时休眠而不删除。
#[derive(Debug, Clone, PartialEq)]
pub struct OpeningIntent {
    pub id: OpeningId,
    pub host: WallAnchor,
    pub along: f64,
    pub elevation: f64,
    pub width: f64,
    pub height: f64,
    pub style: u16,
}
impl OpeningIntent {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.id.0 == 0
            || !self.along.is_finite()
            || !self.elevation.is_finite()
            || !(0.3..=2.).contains(&self.width)
            || !(0.4..=2.5).contains(&self.height)
        {
            return Err(DomainError("invalid opening"));
        }
        Ok(())
    }
}
/// 端点连接的迟滞以稳定控制点和对方区段定位，不能按整对笔画扩大其他连接的容差。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionHint {
    pub endpoint: (StrokeId, u64),
    pub segment: (StrokeId, u64, u64),
    pub kind: StrokeKind,
}
/// 仅保存迟滞需要的选择，不保存完整关系或几何。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecisionState {
    pub kinds: BTreeMap<StrokeId, StrokeKind>,
    pub window_groups: Vec<Vec<OpeningId>>,
    pub connections: Vec<ConnectionHint>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextRules {
    pub version: u64,
    pub path_enter: f64,
    pub path_exit: f64,
    pub wall_enter: f64,
    pub wall_exit: f64,
    pub connect_enter: f64,
    pub connect_exit: f64,
    pub height_tolerance: f64,
    pub window_enter: f64,
    pub window_exit: f64,
    pub max_slope_degrees: f64,
}
impl ContextRules {
    /// 阈值必须具有正确的进入/退出顺序；拒绝 NaN 以免产生遍历顺序相关的结构。
    pub fn validate(self) -> Result<(), DomainError> {
        let values = [
            self.path_enter,
            self.path_exit,
            self.wall_enter,
            self.wall_exit,
            self.connect_enter,
            self.connect_exit,
            self.height_tolerance,
            self.window_enter,
            self.window_exit,
            self.max_slope_degrees,
        ];
        if self.version == 0
            || values.iter().any(|v| !v.is_finite() || *v < 0.)
            || self.path_enter > self.path_exit
            || self.path_exit >= self.wall_exit
            || self.wall_exit > self.wall_enter
            || self.connect_enter > self.connect_exit
            || self.connect_exit > 1.
            || self.window_enter > self.window_exit
            || self.window_exit > 1.
            || !(1.0..=60.0).contains(&self.max_slope_degrees)
        {
            return Err(DomainError("invalid context rules"));
        }
        Ok(())
    }
}
impl Default for ContextRules {
    fn default() -> Self {
        Self {
            version: 1,
            path_enter: 0.08,
            path_exit: 0.18,
            wall_enter: 1.5,
            wall_exit: 1.3,
            connect_enter: 0.15,
            connect_exit: 0.25,
            height_tolerance: 0.1,
            window_enter: 0.18,
            window_exit: 0.28,
            max_slope_degrees: 35.,
        }
    }
}
/// 自动地基的最低绝对标高与附加净空；地形变化不改写 Placement.elevation。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoundationIntent {
    pub reference_y: f64,
    pub clearance: f64,
}
impl FoundationIntent {
    pub fn validate(self) -> Result<(), DomainError> {
        if !self.reference_y.is_finite()
            || !self.clearance.is_finite()
            || !(0.0..=3.0).contains(&self.clearance)
        {
            return Err(DomainError("invalid foundation intent"));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneContext {
    pub terrain: TerrainDocument,
    pub foundations: BTreeMap<BuildingId, FoundationIntent>,
    pub linear: BTreeMap<StrokeId, LinearIntent>,
    pub openings: BTreeMap<OpeningId, OpeningIntent>,
    pub decisions: DecisionState,
    pub rules: ContextRules,
}
impl SceneContext {
    pub fn validate(&self) -> Result<(), DomainError> {
        self.terrain.validate()?;
        self.rules.validate()?;
        for foundation in self.foundations.values() {
            foundation.validate()?;
        }
        if self.openings.iter().any(|(id, w)| *id != w.id) {
            return Err(DomainError("invalid opening"));
        }
        if self.openings.len() > 256 {
            return Err(DomainError("too many openings"));
        }
        for window in self.openings.values() {
            window.validate()?;
        }
        Ok(())
    }
}
/// 世界 X/Z 区域用于查询和失效，不作为对象身份。
pub fn union_bounds(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0].min(b[0]),
        a[1].min(b[1]),
        a[2].max(b[2]),
        a[3].max(b[3]),
    ]
}
pub fn bounds_overlap(a: [f64; 4], b: [f64; 4]) -> bool {
    a[0] <= b[2] && b[0] <= a[2] && a[1] <= b[3] && b[1] <= a[3]
}
pub fn world_point(p: Point, x: f64, z: f64, yaw: f64) -> Point {
    Point {
        x: x + p.x * yaw.cos() + p.z * yaw.sin(),
        z: z - p.x * yaw.sin() + p.z * yaw.cos(),
    }
}
