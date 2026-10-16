//! B `systems/wall/shapes/mod.rs`: shape submodule root.
//!
//! Evidence: `chapters/systems/wall/shapes/mod.rs.md` (O, 0x140945f50) —
//! `add_wall_shape_ux` registers the eight shape systems and the four shape
//! resources (CalcShapeHeight / PreviewCircle / PreviewRectangle /
//! ShapePlacement). The per-system type hashes are U; registration is exposed
//! through the crate's schedule API.

pub mod calc_shape_height;
pub mod circle;
pub mod rectangle;
pub mod shape_placement;
pub mod shape_placement_signifiers;
pub mod utils;

use bevy_ecs::{event::Event, event::Events, resource::Resource, world::World};
use glam::{Vec2, Vec3};

use crate::resources::walls::WallId;

/// Toolbar shape-selection record `{tag u32@+0, ty u8@+4}` (shape_placement /
/// calc_shape_height / ui_create_* read `tag == 6` for wall shapes; ty 1 =
/// circle, 2 = rectangle).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SelectedWallShape {
    pub tag: u32,
    pub ty: u8,
}

impl SelectedWallShape {
    pub const WALL_SHAPES_TAG: u32 = 6;
    pub const TY_CIRCLE: u8 = 1;
    pub const TY_RECTANGLE: u8 = 2;
    pub fn is_circle(&self) -> bool {
        self.tag == Self::WALL_SHAPES_TAG && self.ty == Self::TY_CIRCLE
    }
    pub fn is_rectangle(&self) -> bool {
        self.tag == Self::WALL_SHAPES_TAG && self.ty == Self::TY_RECTANGLE
    }
}

/// PreviewCircle (B +0x10 center, +0x18 radius; larger record fields 待接线).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PreviewCircle {
    pub center: Vec2,
    pub radius: f32,
    pub rotation: f32,
    /// B +0x8 preview height type (2 = follow terrain).
    pub height_ty: i32,
    /// B +0x40 computed preview height (calc_shape_height).
    pub height: f32,
    /// B +0x3c previous preview height (interpolation base).
    pub old_height: f32,
}

/// PreviewRectangle (B +0x10 center, +0x18 dims, +0x20 orientation).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct PreviewRectangle {
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
    /// B +0x8 preview height type (2 = follow terrain).
    pub height_ty: i32,
    /// B +0x40 computed preview height (calc_shape_height).
    pub height: f32,
    /// B +0x3c previous preview height (interpolation base).
    pub old_height: f32,
}

/// ShapePlacement (B 0x30 B record: mode byte @+0, snapped @+1, target/normal).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShapePlacement {
    /// 0 = unsnapped, 2 = snapped.
    pub mode: u8,
    pub snapped: bool,
    pub position: Vec2,
    pub target: Vec3,
    pub normal: Vec3,
}

/// CalcShapeHeight (B 16 B record `{tag u32@+0, cur f32@+4, prev f32@+8,
/// delta f32@+0xc}`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct CalcShapeHeight {
    pub tag: u32,
    pub current: f32,
    pub previous: f32,
    pub delta: f32,
}

/// CreateCircleCmd (B payload: `{u64 seq, u32 tag=0, center qword, radius
/// qword}`; full 48 B Bevy slot 待取证).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct CreateCircleCmd {
    pub center: Vec2,
    pub radius: f32,
}

/// CreateRectangleCmd (B payload `{cmd u64×2, center@+0x20, dims@+0x30,
/// orientation@+0x40, ...}`).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct CreateRectangleCmd {
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
}

/// EditCircleCmd (radius / rotation edit transport).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct EditCircleCmd {
    pub wall: WallId,
    pub center: Vec2,
    pub radius: f32,
    pub rotation: f32,
}

/// EditRectangleCmd (B 0x40 B row, rectangle parameters @+0x10).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct EditRectangleCmd {
    pub wall: WallId,
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
}

/// FinishEditCircleCmd (ui_edit_circle release, history commit).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct FinishEditCircleCmd {
    pub wall: WallId,
}

/// FinishEditRectangleCmd (finish_edit_rectangle 0x141464a60 reader).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct FinishEditRectangleCmd {
    pub wall: WallId,
}

/// RotateCircleCmd (ui_rotate_circle 0x1409ac910): rotate the circle wall's
/// roof-ridge orientation.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RotateCircleCmd {
    pub wall: WallId,
    pub rotation: f32,
}

/// Convert the geometry-domain Curve2 into the wall-authority curve
/// (system_roof::surface::Curve2). The original had one Curve2; the
/// reconstruction keeps the two recovered representations bridged here.
/// (`::utils::` crate path: the local `shapes::utils` submodule shadows it.)
pub fn to_authority_curve(curve: &::utils::curve::Curve2) -> system_roof::surface::Curve2 {
    let points = curve
        .points
        .iter()
        .map(|p| [p.x, p.y])
        .collect::<Vec<_>>();
    system_roof::surface::Curve2::try_new(points, false)
        .expect("shape curve produced a valid authority Curve2")
}

/// B `add_wall_shape_ux` resource half: the four shape preview/placement
/// resources plus the shape command events.
pub fn register_shape_resources(world: &mut World) {
    world.init_resource::<SelectedWallShape>();
    world.init_resource::<PreviewCircle>();
    world.init_resource::<PreviewRectangle>();
    world.init_resource::<ShapePlacement>();
    world.init_resource::<CalcShapeHeight>();
    world.init_resource::<Events<CreateCircleCmd>>();
    world.init_resource::<Events<CreateRectangleCmd>>();
    world.init_resource::<Events<EditCircleCmd>>();
    world.init_resource::<Events<EditRectangleCmd>>();
    world.init_resource::<Events<FinishEditCircleCmd>>();
    world.init_resource::<Events<FinishEditRectangleCmd>>();
    world.init_resource::<Events<RotateCircleCmd>>();
    world.init_resource::<circle::ui_create_circle::CircleCanvasInput>();
    world.init_resource::<rectangle::ui_create_rectangle::RectangleCanvasInput>();
    world.init_resource::<rectangle::ui_edit_rectangle_dims::RectangleDimsEditInput>();
    world.init_resource::<rectangle::ui_edit_rectangle_orientation::RectangleOrientationEditInput>();
    world.init_resource::<circle::ui_edit_circle::CircleEditState>();
    world.init_resource::<rectangle::finish_edit_rectangle::RectangleEditUiState>();
}

/// Build the 64 B CreateRoofCmd request record consumed by
/// [`crate::recovered_building::roof_creation::initialize_resolved_roof`].
/// Layout (O, create_circle/create_rectangle chapters + roof_shape
/// `new_observed`): wall id @+8, roof shape params @+0x20..0x3c
/// (tip_offset_01[2], profile_01, height_01, ridge_length_01, eave_length_01,
/// ridge_dir_bit), style in the ridge_dir_bit second byte.
pub fn build_roof_create_request(
    wall: WallId,
    profile_01: f32,
    height_01: f32,
    ridge_length_01: f32,
    eave_length_01: f32,
    style: u8,
) -> crate::resources::roofs::commands::CreateRoofCmd {
    let mut record = [0u8; 64];
    record[8..16].copy_from_slice(&wall.0.to_le_bytes());
    record[0x28..0x2c].copy_from_slice(&profile_01.to_le_bytes());
    record[0x2c..0x30].copy_from_slice(&height_01.to_le_bytes());
    record[0x30..0x34].copy_from_slice(&ridge_length_01.to_le_bytes());
    record[0x34..0x38].copy_from_slice(&eave_length_01.to_le_bytes());
    let ridge_dir_bit = (style as u32) << 8;
    record[0x38..0x3c].copy_from_slice(&ridge_dir_bit.to_le_bytes());
    crate::resources::roofs::commands::CreateRoofCmd(
        crate::recovered_building::roof_creation::RoofCreateRequestObserved::from_observed(record),
    )
}
