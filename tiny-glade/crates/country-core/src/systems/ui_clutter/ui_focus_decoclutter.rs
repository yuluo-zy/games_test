//! Canonical decorator affordance requests and the original request-building leaves.
//!
//! Native producer slices 1409aaa78..1409aab69 and 1409aafff..1409ab0d6
//! were executed against the original F507 binary with poisoned stack padding.
//! Full ui_focus_decoclutter widget geometry (native volume tag8) remains open;
//! it is deliberately not scheduled through the former boolean FocusUi adapter.
use bevy_ecs::prelude::*;
use crate::resources::walls::{
    decorator::{DecoratorId, DecoratorType},
    decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
};
use crate::systems::collision::world::RaycastWorldResult;
use crate::systems::ui::resources::ui_state::{UiState, WidgetId};

/// `LastClonedClutter`（原版 `ui_focus_decoclutter::LastClonedClutter`；`ClutterPlugin::build`
/// 在 0x140652754 `insert_resource`）。
///
/// 机器码证据（0x140652731…0x140652754）：初值 = 16 B 常量 `0x14293e0c0`
/// （`00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 3f`，即 (0,0,0,1) = `Quat::IDENTITY`）
/// + 1 B 取自被 ICF 折叠成 `return 0` 的 `UiNav::clone_behavior`（0x140002370）——一个默认值 0 的
/// 单字节枚举/布尔。故它**不是**仅 1 字节：Rust 侧为 `{ Quat, u8 }`（16 B 对齐，共 32 B）。
/// 该单字节的具体枚举类型与名字**未闭合**，以 `u8` 承载；读写者（clone-deco 流程）在其它 crate。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LastClonedClutter {
    /// 上次克隆杂物的旋转（初值 `Quat::IDENTITY`）。
    pub rotation: glam::Quat,
    /// 初值 0 的单字节（枚举类型未定型）。
    pub tag: u8,
}

impl Default for LastClonedClutter {
    fn default() -> Self {
        Self { rotation: glam::Quat::IDENTITY, tag: 0 }
    }
}

/// EventInstance stride96 including its8-byte EventId. The native second
/// enum word is a complete collision payload, not a default/miss hit.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RequestRotateDecorator {
    pub widget: WidgetId,
    pub hit: RaycastWorldResult,
    pub decorator_id: DecoratorId,
    pub amount: f32,
    pub ty: DecoratorType,
}

/// EventInstance stride88 including its8-byte EventId.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RequestMoveClutter {
    pub widget: WidgetId,
    pub hit: RaycastWorldResult,
    pub decorator_id: DecoratorId,
    pub ty: DecoratorType,
}

/// The original request variant replaces the collider identity with a
/// decorator+storage identity and takes the position from UiState+4c.
/// Discriminant/extra are inactive native padding, normalized in this owned API.
fn focused_hit(ui: &UiState, id: DecoratorId, addr: DecoratorStorageAddr) -> RaycastWorldResult {
    let (tag, payload) = match addr {
        DecoratorStorageAddr::Wall(wall) => (0, wall.0),
        DecoratorStorageAddr::Terrain => (1, 0),
        DecoratorStorageAddr::Stair => (2, 0),
    };
    RaycastWorldResult {
        handle: [4 | (u64::from(id.0) << 32), tag, payload],
        position: ui.intersection_point(),
        normal: glam::Vec3::Y,
        toi: 0.0,
        discriminant: 0,
        extra: [0.0; 2],
    }
}

/// Exact producer tail after the real rotate widget was registered. Caller
/// supplies the actual widget and original metadata bounds, never an ID guess.
pub fn request_focused_decorator_rotation(
    ui: &UiState, widget: WidgetId, id: DecoratorId, storage: &DecoratorStorage,
    footprint_bottom: f32, footprint_top: f32,
) -> Option<RequestRotateDecorator> {
    if !ui.widget_context(widget).is_active() { return None; }
    let addr = storage.lookup_storage(id)?;
    let info = storage.get(addr)?.get(id)?;
    Some(RequestRotateDecorator {
        widget, hit: focused_hit(ui, id, addr), decorator_id: id,
        amount: (footprint_top - footprint_bottom) + 0.5,
        ty: DecoratorType(info.ty),
    })
}

/// Exact move request tail, sharing the same real widget context gate.
pub fn request_focused_decorator_move(
    ui: &UiState, widget: WidgetId, id: DecoratorId, storage: &DecoratorStorage,
) -> Option<RequestMoveClutter> {
    if !ui.widget_context(widget).is_active() { return None; }
    let addr = storage.lookup_storage(id)?;
    let info = storage.get(addr)?.get(id)?;
    Some(RequestMoveClutter {
        widget, hit: focused_hit(ui, id, addr), decorator_id: id,
        ty: DecoratorType(info.ty),
    })
}
