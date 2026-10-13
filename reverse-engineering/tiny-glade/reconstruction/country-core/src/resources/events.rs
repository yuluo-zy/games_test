//! events.rs — 墙系统命令/通知事件（WP12 facade）。
//!
//! O 名；行布局按 PDB 骨架（typed gap 字段）。`systems::wall` 以 EventReader /
//! EventWriter 消费。

#![allow(dead_code, unused_imports)]

use bevy_ecs::event::Event;
use glam::Vec2;
use glam::Vec3;

use crate::resources::walls::{WallId, WallStyle};
use crate::typed_gap::Gap;

/// Wall-height adjust command (O: 0x30-byte row {ty u32@0x10, height f32@0x14,
/// WallId@0x18, flags}).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct AdjustWallHeightCmd {
    pub ty: u32,
    pub height: f32,
    pub wall: WallId,
    pub follow_terrain: bool,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct FinishAdjWallHeightMode;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct FinishWall;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct RecordMouseInputIntoWall;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct SetWallStyleCmd {
    pub wall: WallId,
    pub style: WallStyle,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct SetFreehandHeightTypeCmd {
    pub wall: WallId,
    pub ty: u8,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct SetPlasterPeelAmountCmd {
    pub wall: WallId,
    pub amount: f32,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct CreateCircleCmd {
    pub center: Vec3,
    pub radius: f32,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct CreateRectangleCmd {
    pub center: Vec3,
    pub dims: Vec2,
    pub angle: f32,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct EditRectangleCmd {
    pub wall: WallId,
    pub dims: Vec2,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct FinishEditRectangleCmd;
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct HighlightWallCmd {
    pub wall: WallId,
    pub ty: u8,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct OutlineWallCmd {
    pub wall: WallId,
}
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct OutlineStairsCmd;
