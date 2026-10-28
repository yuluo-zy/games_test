//! B `systems/wall/shapes/mod.rs`：形状创建、编辑与预览的原归属。
//!
//! 原140945f50包含嵌套A/C链，而非旧注释所称八个系统/四个资源。
//! 完整注册合同见toolbar的shape-registration-contract-review；实际游戏当前
//! 只启用已复原的toolbar必要子链，其余编辑叶子不能由资源存在推断已接通。

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

/// 原圆形画布预览：反馈位置、几何参数与交互起始平面。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PreviewCircle {
    /// 原 +0..0xf：上一次音频反馈位置，退出工具时清空。
    pub last_audio_position: Option<Vec3>,
    pub center: Vec2,
    pub radius: f32,
    pub rotation: f32,
    /// 原 +0x20：开始画布交互时保存的放置平面。
    pub placement_y: f32,
}

impl Default for PreviewCircle {
    fn default() -> Self {
        Self {
            last_audio_position: None,
            center: Vec2::ZERO,
            radius: 3.0,
            rotation: 0.0,
            placement_y: 0.0,
        }
    }
}

/// 原矩形画布预览：原几何块+0x10朝向/+0x18中心/+0x20尺寸。
/// 当前采用安全字段访问，声明顺序不承担原私有ABI。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PreviewRectangle {
    /// 原 +0..0xf：上一次音频反馈位置。
    pub last_audio_position: Option<Vec3>,
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
    /// 原 +0x28：拖拽起始放置平面。
    pub placement_y: f32,
}

impl Default for PreviewRectangle {
    fn default() -> Self {
        Self {
            last_audio_position: None,
            center: Vec2::ZERO,
            dimensions: Vec2::splat(6.0),
            orientation: Vec2::X,
            placement_y: 0.0,
        }
    }
}

pub use shape_placement::{DontRecalculateShapePlacement, ShapePlacement, SnapMode};

/// 兼容原恢复接口；与半木构造缓存共用同一个权威资源。
pub use calc_shape_height::TimberframeConstructionParamsCache as CalcShapeHeight;

/// 原圆形创建请求：继承墙身份、完整几何、高度与海拔。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct CreateCircleCmd {
    pub previous_wall: Option<WallId>,
    pub center: Vec2,
    pub radius: f32,
    pub rotation: f32,
    pub height: f32,
    pub elevation: f32,
}

/// 原矩形创建请求：继承墙身份、完整几何、高度与海拔。
///
/// `previous_wall` is the native event's `GlobalId` at record `+0x8`
/// (fresh 0x140a93790: `MOV RSI,[RDI+0x8]`), the wall whose roof supplies the
/// inherited style/pitch.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct CreateRectangleCmd {
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
    pub previous_wall: Option<WallId>,
    pub height: f32,
    pub elevation: f32,
}

/// EditRectangleCmd (B 0x40 B row, rectangle parameters @+0x10).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct EditRectangleCmd {
    pub wall: WallId,
    pub center: Vec2,
    pub dimensions: Vec2,
    pub orientation: Vec2,
    pub position: glam::Vec3,
    pub movement: f32,
    pub rotating: bool,
}

/// FinishEditRectangleCmd (finish_edit_rectangle 0x141464a60 reader).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct FinishEditRectangleCmd {
    pub wall: WallId,
}

/// Convert the geometry-domain Curve2 into the wall-authority curve
/// (crate::resources::roofs::geometry::surface::Curve2). The original had one Curve2; the
/// reconstruction keeps the two recovered representations bridged here.
/// (`::utils::` crate path: the local `shapes::utils` submodule shadows it.)
pub fn to_authority_curve(
    curve: &::utils::curve::Curve2,
) -> crate::resources::roofs::geometry::surface::Curve2 {
    let points = curve.points.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>();
    crate::resources::roofs::geometry::surface::Curve2::try_new(points, false)
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
    world.init_resource::<Events<crate::resources::glade_border::MaxDemoElevationReached>>();
    world.init_resource::<Events<CreateRectangleCmd>>();
    world.init_resource::<Events<EditRectangleCmd>>();
    world.init_resource::<Events<rectangle::finish_edit_rectangle::OnRectangleEdited>>();
    world.init_resource::<Events<FinishEditRectangleCmd>>();
    world.init_resource::<crate::resources::walls::shape_alignment::WallShapeAlignmentWrites>();
    world.init_resource::<rectangle::ui_create_rectangle::ShapesDragMouseMoved>();
    world.init_resource::<rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource>();
    world.init_resource::<Events<rectangle::ui_edit_rectangle_dims::DoUiEditRectangleDims>>();
    world.init_resource::<rectangle::ui_edit_rectangle_orientation::ClosestCornerCache>();
    world
        .init_resource::<Events<rectangle::ui_edit_rectangle_orientation::DoUiEditRectangleRotate>>(
        );
    world.init_resource::<rectangle::finish_edit_rectangle::RectangleEditUiState>();
}

/// Build the 64 B CreateRoofCmd request record consumed by
/// [`crate::resources::roofs::creation::initialize_resolved_roof`].
/// Layout (O, create_circle/create_rectangle fresh disasm 2026-10-17): wall id
/// @+8, selected roof colour @+0x10, roof appearance @+0x14..0x20 (flag2 +
/// 11-byte pair from `get_building_and_roof_colors`), tip_offset_01[2] @+0x20,
/// profile_01 @+0x28, height_01 @+0x2c, ridge_length_01 @+0x30,
/// eave_length_01 @+0x34, ridge_dir bool @+0x38, style @+0x39.
pub fn build_roof_create_request(
    wall: WallId,
    profile_01: f32,
    height_01: f32,
    ridge_length_01: f32,
    eave_length_01: f32,
    style: u8,
    ridge_dir: bool,
    roof_color_id: u32,
    roof_appearance: [u8; 12],
) -> crate::resources::roofs::commands::CreateRoofCmd {
    let mut record = [0u8; 64];
    record[8..16].copy_from_slice(&wall.0.to_le_bytes());
    record[0x10..0x14].copy_from_slice(&roof_color_id.to_le_bytes());
    record[0x14..0x20].copy_from_slice(&roof_appearance);
    record[0x28..0x2c].copy_from_slice(&profile_01.to_le_bytes());
    record[0x2c..0x30].copy_from_slice(&height_01.to_le_bytes());
    record[0x30..0x34].copy_from_slice(&ridge_length_01.to_le_bytes());
    record[0x34..0x38].copy_from_slice(&eave_length_01.to_le_bytes());
    record[0x38] = u8::from(ridge_dir);
    record[0x39] = style;
    crate::resources::roofs::commands::CreateRoofCmd(
        crate::resources::roofs::creation::RoofCreateRequestObserved::from_observed(record),
    )
}

pub mod authoring;
