//! B `ui_systems/ui_focus_stairs.rs` — 楼梯焦点交互（0x14079c6d0，O 结构；195KB
//! fresh 反编译）。
//!
//! 主要观察（O）：widget 注册 `UiState::add("rotate_single_node_stairs_widget"，
//! 0x142a81fb0, 32 字符)`（0x1409236d0）+ `WidgetParams::new`（0x1409a8e10）+
//! `UiSounds::empty/boop/on_hover_start` + `occlude_by_world`（0x1409a8ee0）；
//! 指示符经 `SignifierFactory::new`（0x14089d8b0 + 0x142a81f80 族）；
//! `History::start_input_recording`/`add`（0x140a84ec0）；楼梯图
//! `get_all_connected_nodes` + `CameraCursorRay::intersect_with_xz_plane` +
//! `roof_uv_and_point_at_ws_pos` + `CachedDecoratorTransforms::position/transform`
//! + `DecoratorStorage::lookup_decorator_info` + `PublicWallState::
//! get_wall_attachment_height/flat_roof_y`；事件：`RequestRotateDecorator`
//! （ui_clutter 域，core 未落——本 crate 登记）、`InstaCreateStairPoint`、
//! `PlayStairsAudioCmd`、`FeedbackInputOutOfBorder`/`DisplayOutOfBorder`；
//! `UiWidgetContext::clicked/intersecting/is_active` 相位；box_trigger_volume
//! （ui_edit_rectangle_orientation 域，core 未落——登记）。
use bevy_ecs::prelude::*;
use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::systems::audio::PlayStairsAudioCmd;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{UiSounds, WidgetParams};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;

/// 楼梯焦点 widget 名（O：0x142a81fb0，32 字符）。
pub const ROTATE_SINGLE_NODE_STAIRS_WIDGET: &str = "rotate_single_node_stairs_widget";

/// 旋转请求（ui_clutter `ui_focus_decoclutter::RequestRotateDecorator` 域；
/// core 未落——本 crate 登记待 core 接线）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct RequestRotateDecorator;

/// B 0x14079c6d0（O 结构）：楼梯焦点主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_focus_stairs(
    ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    _transforms: Res<CachedDecoratorTransforms>,
    _storage: Res<DecoratorStorage>,
    stairs: Res<StairsState>,
    mut _rotates: EventWriter<RequestRotateDecorator>,
    mut _points: EventWriter<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint>,
    mut _stairs_audio: EventWriter<PlayStairsAudioCmd>,
    mut _border_feedback: EventWriter<FeedbackInputOutOfBorder>,
    mut _border_display: EventWriter<DisplayOutOfBorder>,
) {
    let endpoint = stairs.endpoint();
    if endpoint.is_unset() {
        return;
    }
    let node = endpoint.node();
    // O：每个连接节点注册旋转 widget + 指示符。
    let params = WidgetParams::new().sounds(UiSounds::empty());
    ui.add(ROTATE_SINGLE_NODE_STAIRS_WIDGET, params);
    for connected in stairs.get_all_connected_nodes(node) {
        let _ = connected;
        stream.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position: Default::default(),
            scale: 1.0,
            name: u64::from(connected),
        });
    }
    // I：相位处理（clicked → rotate/insert；esc/边界 → 反馈事件）细节 U。
    let _ = &mut stream;
}
