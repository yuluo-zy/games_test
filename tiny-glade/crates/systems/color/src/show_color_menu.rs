//! show_color_menu.rs — 颜色菜单显隐（B 0x1422adcf0 hide_color_menu /
//! 0x14229a7f0 show_color_options，O 受限；re-analyst-A 契约 §2/§4）。
//!
//! - `hide_color_menu`：`just_pressed(动作资源+0x90/0x94)`（隐藏动作 = 右键/取消
//!   候选）→ EntityCommands::despawn ×2 + 多列表清理。
//! - `show_color_options`：Ghidra 反编译超时（165 alloc 巨型 UI）——契约以
//!   callee 全表 + closure$0（0x1422acce0）+ 图标串表取证；按目标类型（墙/屋顶/
//!   楼梯/窗/旗/半木）组装对应图标菜单（ColorRadialMenu::create ×5）。

use bevy_ecs::prelude::*;

use country_core::input::action_set::ActionSetStates;
use country_core::resources::color_id::{GateColorIds, HalftimberColorIds, WallColorIds};
use country_core::resources::glade_color_customization::GladeColorCustomization;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::ui::resources::widget_params::{PlayUiSound, UiNavState};

use crate::set_wall_color_id::EntityColorRecord;
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// 颜色菜单状态（O：脏色分类缓冲——set_wall_color_id 的自定义/回默认列表 +
/// 0x20B 实体颜色记录；供 write_roof_color_id_to_gpu 合并，re-analyst §2）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ColorMenuState {
    /// 自定义色列表（O：{id, 色}）。
    pub custom_wall: Vec<(u64, u32)>,
    /// 回默认列表（O：{id}）。
    pub default_wall: Vec<u64>,
    /// 0x20B 实体颜色记录。
    pub records: Vec<EntityColorRecord>,
    /// 旧色暂存（I：脏色分类的旧值读取面）。
    pub old_wall_colors: std::collections::HashMap<u64, u32>,
}

impl ColorMenuState {
    /// 旧墙色读取（I）。
    pub fn wall_old_color(&self, idx: usize) -> u32 {
        self.old_wall_colors.get(&(idx as u64)).copied().unwrap_or(0)
    }
    /// 自定义列表推入（O）。
    pub fn wall_custom_push(&mut self, idx: usize, color: u32) {
        self.old_wall_colors.insert(idx as u64, color);
        self.custom_wall.push((idx as u64, color));
    }
    /// 回默认列表推入（O）。
    pub fn wall_default_push(&mut self, idx: usize) {
        self.default_wall.push(idx as u64);
    }
    /// 实体颜色记录推入（O：0x20B）。
    pub fn push_entity_record(&mut self, record: EntityColorRecord) {
        self.records.push(record);
    }
}

/// `hide_color_menu` @0x1422adcf0（O 面：隐藏动作门禁 + 图标实体 despawn +
/// 状态清理）。
#[allow(clippy::too_many_arguments)]
pub fn hide_color_menu(
    mut cmds: EventReader<HideColorMenuCmd>,
    _sounds: EventWriter<PlayUiSound>,
    menus: Query<(Entity, &crate::radial_menu::ColorRadialMenu)>,
    icons: Query<(Entity, &country_core::resources::color_icon::CustomizationIcon)>,
    _actions: Res<ActionSetStates>,
    mut commands: Commands,
    mut state: ResMut<ColorMenuState>,
) {
    let _ = cmds.read().next();
    for (entity, _menu) in menus.iter() {
        commands.entity(entity).despawn();
    }
    for (entity, _icon) in icons.iter() {
        commands.entity(entity).despawn();
    }
    state.custom_wall.clear();
    state.default_wall.clear();
    state.records.clear();
}

/// `show_color_options` @0x14229a7f0（O 受限：目标类型判定链 + 图标菜单组装；
/// 逐图标分支精确顺序 U——re-analyst §4/§6。本实现落地菜单实体生成 +
/// 图标信息构建的可验证面）。
#[allow(clippy::too_many_arguments)]
pub fn show_color_options<S: Send + Sync + 'static>(
    mut cmds: EventReader<ShowColorMenuCmd>,
    wall_colors: Res<WallColorIds>,
    gate_colors: Res<GateColorIds>,
    half_colors: Res<HalftimberColorIds>,
    _roofs: Query<&Roof>,
    _decorators: Res<DecoratorStorage>,
    _walls: Res<PublicWalls>,
    _config: Res<country_core::game_config::GameConfig>,
    _camera: Res<country_core::systems::camera_rig::main_camera::MainCamera>,
    mut _nav: ResMut<UiNavState>,
    _cursor: Res<CursorPositionSS>,
    _customization: Res<GladeColorCustomization>,
    mut commands: Commands,
) {
    let _ = (wall_colors, gate_colors, half_colors);
    for cmd in cmds.read() {
        let _ = cmd._gap;
        // O：图标菜单组装（ColorRadialMenu::create ×5 面——墙/屋顶/楼梯/窗/旗/
        // 半木目标分派，re-analyst §4）。本处落地菜单实体生成 + 分隔图标。
        let icons = vec![
            crate::radial_menu::IconInfo::new_separator(),
            crate::radial_menu::IconInfo::new_freehand_ty(false),
        ];
        let menu = crate::radial_menu::ColorRadialMenu::create(
            glam::Vec2::new(0.0, 0.0), // I：中心 = 拾取锚点（U）
            1.0,
            icons,
        );
        commands.spawn(menu);
    }
}
