//! B `systems/wall/eraser/mod.rs`: wall eraser submodule root.

pub mod animate_wall_bulldoze;
pub mod archivist;
pub mod ui_bulldozer;
pub mod wall_eraser;

use bevy_ecs::event::Event;
use glam::Vec3;

use crate::resources::walls::WallId;

/// WallBulldoze command transport (equivalent of the B `AnimateWallBulldozeCmd`
/// 152 B record; the animation buffer element).
#[derive(Event, Clone, Debug)]
pub struct AnimateWallBulldozeCmd {
    pub wall: WallId,
    pub low: f32,
    pub high: f32,
    pub anchor: [f32; 3],
    /// Original command +0x68 flag (== 1 → second tag lane).
    pub flag: u8,
}

/// Bulldoze audio event (PlayWallAudio kind dispatch; host 待接线).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct WallBulldozeAudio {
    pub wall: WallId,
    pub position: Vec3,
    pub kind: u8,
    pub flag: u8,
}

/// Stroke input for the freehand eraser (UI producer 待接线).
#[derive(Event, Clone, Debug)]
pub struct WallEraserStroke {
    pub wall: WallId,
    /// Erased u-range on the wall curve.
    pub low: f32,
    pub high: f32,
}

pub mod bulldoze;
