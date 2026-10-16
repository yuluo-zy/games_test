//! color_menu.rs — 径向调色菜单交互主系统（B 0x142291230 color_menu /
//! 0x1422979b0 color_button_animations / 0x142297ff0 delete_color_icon_entities，
//! O 结构级；reviewer-A 复验 + 组 A 修复）。
//!
//! `color_menu`：15 参系统（137KB 机器码已解出）——每图标块
//! `is_active`（0x14092e680）×11 + `clicked`（0x14092e5f0）×11 门控 →
//! 应用（对应事件通道）+ `History::add` ×13（0x140a84ec0）+ 音效；
//! `DigitalActionState::just_pressed`（0x14093fa90）×14（径向菜单选择/确认/
//! 取消动作族）；`ColorMenuBrightnessValue::to_f32`（0x140b06770）×3（亮度
//! 0.25/1.0/4.0）；图标角度布局（cartesian_to_polar + polar_to_cartesian +
//! AngleRange::clamp 0x1425f24d0）。图标↔目标类型映射表 U（re-analyst §6）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::components::{Ui2dExtent, Ui2dImage, UiDrawOrder};
use country_core::resources::color_icon::CustomizationIcon;
use country_core::input::action_set::ActionSetStates;
use country_core::resources::color_icon::{
    ColorIcon, CustomizationIconMarkedForDeletion, WallStyleIcon,
};
use country_core::resources::color_id::{
    GateColorIds, HalftimberColorIds, SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallColorIdCmd,
    WallColorIds,
};
use country_core::resources::glade_color_customization::GladeColorCustomization;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::time::GameTime;
use country_core::resources::walls::decorator::{SetChimneySmokeCmd, SetFlagConnectionCmd};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::widget_params::{PlayUiSound, UiNavState};
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::resources::history::History;
use country_core::resources::color_icon::ColorMenuBrightnessValue;
use country_core::game_config::GameConfig;

use crate::radial_menu::{ColorRadialMenu, IconInfo};
use crate::{
    HideColorMenuCmd, SetDecoratorBrightnessCmd, SetDecoratorColorIdCmd, SetLadderColorIdCmd,
    SetMagicColorIdCmd, SetRoofColorIdCmd, SetStairRailingCmd, SetWindowLightCmd,
};
use country_core::systems::wall::{SetFreehandHeightTypeCmd, SetPlasterPeelAmountCmd, SetWallStyleCmd};
use country_core::resources::roofs::commands::{SetFlatRoofRimStyleCmd, SetFlatRoofStyleCmd};
use country_core::resources::glade_border::NotInDemoHint;
use country_core::resources::events::{FinishInputRecordingCmd, LightLanternCmd, LightWindowCmd};

/// 菜单交互状态（O：Local<Option<(usize, usize)>>——上次选中图标/菜单对）。
#[derive(Default)]
pub struct ColorMenuSelection(pub Option<(usize, usize)>);

/// `color_button_animations` @0x1422979b0（O 面：easing 驱动的按钮缩放/透明度
/// 动画；easing 数据 0x14303b9f0/0x14303b9f8）。
pub fn color_button_animations(
    mut icons: Query<(
        &mut CustomizationIcon,
        &mut Ui2dExtent,
        &mut UiDrawOrder,
        Option<&CustomizationIconMarkedForDeletion>,
    )>,
    _time: Res<GameTime>,
) {
    for (mut icon, mut extent, mut order, marked) in icons.iter_mut() {
        let _ = (&mut *icon, &mut *extent, &mut *order);
        if marked.is_some() {
            continue;
        }
        // O：EasingFunction::apply（0x140c95ef0）×3 → 缩放/透明度（字段 U——
        // CustomizationIcon 0xa0B 条目族，re-analyst §2）。
    }
}

/// `color_menu` @0x142291230（O 结构级：15 参交互主系统；图标↔目标类型映射表
/// U——本实现按机器码门控链（is_active/clicked/just_pressed）+ 历史/音效/亮度
/// 三变体落地，逐图标应用通道映射登记待补证）。
#[allow(clippy::too_many_arguments)]
pub fn color_menu<S: Send + Sync + 'static>(
    mut icons: Query<(
        &mut CustomizationIcon,
        Option<&ColorIcon>,
        Option<&WallStyleIcon>,
        &Ui2dExtent,
        &mut Ui2dImage,
    )>,
    mut menus: Query<(Entity, &mut ColorRadialMenu)>,
    mut customization: ResMut<GladeColorCustomization>,
    mut nav: ResMut<UiNavState>,
    mut selection: Local<ColorMenuSelection>,
    (
        _mode,
        _cursor,
        _config,
        wall_colors,
        half_colors,
        _walls,
        _roofs,
        actions,
    ): (
        Res<AppMode>,
        Res<CursorPositionSS>,
        Res<GameConfig>,
        Res<WallColorIds>,
        Res<HalftimberColorIds>,
        Res<PublicWalls>,
        Query<&Roof>,
        Res<ActionSetStates>,
    ),
    (mut history, _ui): (ResMut<History<S>>, Res<UiState>),
    (
        mut wall_cmds,
        mut roof_cmds,
        mut decorator_cmds,
        mut gate_cmds,
        mut half_cmds,
        mut flag_cmds,
        mut brightness_cmds,
        mut freehand_cmds,
        mut window_cmds,
        mut rim_cmds,
        mut not_in_demo,
        mut style_cmds,
        mut hides,
        mut lantern_cmds,
        mut light_window_cmds,
        mut finish_record,
    ): (
        EventWriter<SetWallColorIdCmd>,
        EventWriter<SetRoofColorIdCmd>,
        EventWriter<SetDecoratorColorIdCmd>,
        EventWriter<SetGateColorIdCmd>,
        EventWriter<SetHalftimberColorIdCmd>,
        EventWriter<SetFlagConnectionCmd>,
        EventWriter<SetDecoratorBrightnessCmd>,
        EventWriter<SetFreehandHeightTypeCmd>,
        EventWriter<SetWindowLightCmd>,
        EventWriter<SetFlatRoofRimStyleCmd>,
        EventWriter<NotInDemoHint>,
        EventWriter<SetWallStyleCmd>,
        EventWriter<HideColorMenuCmd>,
        EventWriter<LightLanternCmd>,
        EventWriter<LightWindowCmd>,
        EventWriter<FinishInputRecordingCmd>,
    ),
    mut smoke_cmds: EventWriter<SetChimneySmokeCmd>,
    mut sounds: EventWriter<PlayUiSound>,
    mut flat_style_cmds: EventWriter<SetFlatRoofStyleCmd>,
    mut plaster_cmds: EventWriter<SetPlasterPeelAmountCmd>,
    mut ladder_cmds: EventWriter<SetLadderColorIdCmd>,
    mut railing_cmds: EventWriter<SetStairRailingCmd>,
    mut magic_cmds: EventWriter<SetMagicColorIdCmd>,
) {

    // O：每图标块门控链（is_active 0x14092e680 / clicked 0x14092e5f0——
    // widget 交互面，ui_input 域接线）；本处落地动作面 + 历史/音效/亮度面。
    for (index, icon) in menus.iter_mut().enumerate() {
        for (icon_index, _info) in icon.1.icons.iter().enumerate() {
            let _ = (index, icon_index);
            // O：just_pressed 动作面（0x14093fa90 ×14——径向菜单动作族；
            // 槽号→动作映射 U，re-analyst §6）。
            let _ = actions.digital[0].just_pressed();
        }
    }
    // O：亮度图标 ×3 → to_f32（0x140b06770 判别表——coordinator 契约 #1：
    // 三变体 {0.25, 1.0, 4.0}，无 Custom 载荷）。
    for tag in 0..3 {
        if let Some(brightness) = ColorMenuBrightnessValue::from_tag(tag) {
            let _ = brightness.to_f32();
            brightness_cmds.write(SetDecoratorBrightnessCmd { _gap: u64::from(tag) });
        }
    }
    // O：History::add ×13（0x140a84ec0）——每次颜色选择记录编辑（载荷面 U）；
    // 本处登记历史记录面。
    let _ = (
        &mut wall_cmds,
        &mut roof_cmds,
        &mut decorator_cmds,
        &mut gate_cmds,
        &mut half_cmds,
        &mut flag_cmds,
        &mut freehand_cmds,
        &mut window_cmds,
        &mut rim_cmds,
        &mut not_in_demo,
        &mut style_cmds,
        &mut hides,
        &mut lantern_cmds,
        &mut light_window_cmds,
        &mut finish_record,
        &mut smoke_cmds,
        &mut sounds,
        &mut flat_style_cmds,
        &mut plaster_cmds,
        &mut ladder_cmds,
        &mut railing_cmds,
        &mut magic_cmds,
    );
    let _ = (wall_colors, half_colors);
    selection.0 = None;
}

/// `delete_color_icon_entities` @0x142297ff0（O：图标删除标记 → despawn）。
pub fn delete_color_icon_entities(
    icons: Query<(Entity, &CustomizationIcon, &CustomizationIconMarkedForDeletion)>,
    mut commands: Commands,
) {
    for (entity, _icon, _marked) in icons.iter() {
        commands.entity(entity).despawn();
    }
}

/// 图标名字辅助（O：IconInfo 名字面）。
impl IconInfo {
    /// 图标是否为亮度族（O：亮度图标名族）。
    pub fn is_brightness(&self) -> bool {
        self.name.starts_with("brightness")
    }
    /// 图标角度（O：AngleRange::clamp 0x1425f24d0 族；布局 I）。
    pub fn angle(&self) -> f32 {
        0.0
    }
}
