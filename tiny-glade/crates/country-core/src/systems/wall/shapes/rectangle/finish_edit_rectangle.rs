//! B 141464a60：矩形拖拽快照、提交及未变更时的装饰物恢复。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use super::super::FinishEditRectangleCmd;
use super::edit_decision::{RectangleEditState, finish_rectangle_edit_decision};
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::history::{EditType, History};
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::walls::InnerWalls;
use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
use crate::resources::walls::decorator_storage::DecoratorStorage;

/// 原WipRectangle的80字节enum记录：0为空、1为旋转、2为尺寸。
/// 此记录保留完整before/after及拖拽锚点，不依赖全局PreviewRectangle。
#[derive(Resource, Clone, Debug)]
pub struct WipRectangle(pub [u8; 80]);

impl Default for WipRectangle {
    fn default() -> Self {
        Self([0; 80])
    }
}

impl WipRectangle {
    pub fn tag(&self) -> u32 {
        u32::from_le_bytes(self.0[..4].try_into().unwrap())
    }
    fn put_f32(&mut self, at: usize, value: f32) {
        self.0[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    pub fn scalar(&self, at: usize) -> f32 {
        f32::from_le_bytes(self.0[at..at + 4].try_into().unwrap())
    }
    fn put_rectangle(&mut self, at: usize, rectangle: utils::geometry::Rectangle2d) {
        for (i, value) in [
            rectangle.axis.x,
            rectangle.axis.y,
            rectangle.center.x,
            rectangle.center.y,
            rectangle.size.x,
            rectangle.size.y,
        ]
        .into_iter()
        .enumerate()
        {
            self.put_f32(at + 4 * i, value);
        }
    }
    fn rectangle_at(&self, at: usize) -> utils::geometry::Rectangle2d {
        utils::geometry::Rectangle2d {
            axis: glam::Vec2::new(self.scalar(at), self.scalar(at + 4)),
            center: glam::Vec2::new(self.scalar(at + 8), self.scalar(at + 12)),
            size: glam::Vec2::new(self.scalar(at + 16), self.scalar(at + 20)),
        }
    }
    pub fn before(&self) -> utils::geometry::Rectangle2d {
        self.rectangle_at(match self.tag() {
            1 => 32,
            2 => 24,
            _ => panic!("矩形编辑尚未开始"),
        })
    }
    pub fn current(&self) -> utils::geometry::Rectangle2d {
        self.rectangle_at(match self.tag() {
            1 => 56,
            2 => 48,
            _ => panic!("矩形编辑尚未开始"),
        })
    }
    pub fn set_current(&mut self, rectangle: utils::geometry::Rectangle2d) {
        let at = match self.tag() {
            1 => 56,
            2 => 48,
            _ => panic!("矩形编辑尚未开始"),
        };
        self.put_rectangle(at, rectangle);
    }
    /// B140ad6e00 Started只写该variant所拥有的字节，保留其他padding。
    pub fn begin_dimensions(
        &mut self,
        wall: crate::resources::walls::WallId,
        edge: usize,
        rectangle: utils::geometry::Rectangle2d,
        y: f32,
        offset: glam::Vec2,
    ) {
        self.0[..4].copy_from_slice(&2u32.to_le_bytes());
        self.put_f32(4, y);
        self.0[8..16].copy_from_slice(&wall.0.to_le_bytes());
        self.0[16..24].copy_from_slice(&(edge as u64).to_le_bytes());
        self.put_rectangle(24, rectangle);
        self.put_rectangle(48, rectangle);
        self.put_f32(72, offset.x);
        self.put_f32(76, offset.y);
    }
    /// B140ad8be0 Started保存夹角偏移，之后每帧都相对before计算。
    pub fn begin_rotation(
        &mut self,
        wall: crate::resources::walls::WallId,
        corner: usize,
        rectangle: utils::geometry::Rectangle2d,
        y: f32,
        angle: f32,
    ) {
        self.0[..4].copy_from_slice(&1u32.to_le_bytes());
        self.put_f32(4, y);
        self.put_f32(8, angle);
        self.0[16..24].copy_from_slice(&wall.0.to_le_bytes());
        self.0[24..32].copy_from_slice(&(corner as u64).to_le_bytes());
        self.put_rectangle(32, rectangle);
        self.put_rectangle(56, rectangle);
    }
}

/// 兼容旧导入路径；资源TypeId仍然只有WipRectangle一个。
pub use WipRectangle as RectangleEditUiState;

/// B事件有效载荷80字节，原slot+80的EventId由Bevy独立管理。
#[derive(Event, Clone, Debug)]
pub struct OnRectangleEdited(pub [u8; 80]);

/// 与B九个SystemParam对应。Finish为unit，墙及before必须来自Wip。
pub fn finish_edit_rectangle(
    inner: Res<InnerWalls>,
    mut wip: ResMut<WipRectangle>,
    mut history: ResMut<History>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    mut finish: EventReader<FinishEditRectangleCmd>,
    mut edited: EventWriter<OnRectangleEdited>,
    mut hints: EventWriter<NotifyHintSystemShapeEditWasPerformed>,
    mut sheep: EventWriter<CmdCheckForSheepInsideBuilding>,
) {
    // 原函数先发布活动快照，再消费finish；无finish的帧也必须发布。
    let mut state = RectangleEditState(wip.0);
    let snapshot = finish_rectangle_edit_decision(&mut state, false, None)
        .expect("矩形Wip包含原版未定义的标签");
    if let Some(snapshot) = snapshot.active_state_snapshot {
        edited.write(OnRectangleEdited(snapshot));
    }
    if finish.read().last().is_none() {
        return;
    }
    let wall = state.wall().expect("矩形完成事件缺少活动编辑");
    let source = inner
        .get(crate::resources::walls::WallId(wall.0))
        .map(|wall| wall.source());
    let decision = finish_rectangle_edit_decision(&mut state, true, source)
        .expect("矩形完成时权威墙/矩形源不存在");
    if let Some(edit) = decision.history {
        history
            .add(EditType::Wall(Arc::new(edit)))
            .expect("矩形完成时历史录制尚未开始或事务不可写");
    }
    if decision.restore_archived_decorators {
        archivist.extract_and_restore_to_storage(&mut storage);
    }
    if decision.empty_notification {
        hints.write(NotifyHintSystemShapeEditWasPerformed);
    }
    if let Some(wall) = decision.wall_notification {
        sheep.write(CmdCheckForSheepInsideBuilding {
            wall: crate::resources::walls::WallId(wall.0),
        });
    }
    // 原版只清tag；保留inactive字节，不能整块清零。
    wip.0 = state.0;
}
