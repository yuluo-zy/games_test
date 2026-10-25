//! B设置控制页与键位页面：原7分组、采集／确认／取消／重置的完整状态流。
//! 原owner、函数与静态表证据见toolbar-keybindings-native.json和slots报告。
//! 键位仍写GameProfile.key_bindings；上层仅在确认时发布SaveGameProfileCmd。
//! show_gui_controls的历史地址不是原函数入口，不纳入本设置页的恢复声明。

use bevy_input::{
    ButtonState,
    keyboard::{KeyCode, KeyboardInput},
    mouse::{MouseButton, MouseButtonInput},
};
use country_core::{
    game_key_bindings::{
        ActionDescription, KeyBinding, MouseBinding, keycode_to_str, mouse_button_to_string,
    },
    game_profile::{GameKeyBindings, GameProfile},
};
use toolbar::mei::{
    config::TemplateView,
    core::{GlobalId, NodeContext, calculate_hash},
    widgets::{
        LabelContent, MenuButtonContent,
        tweaker::{HintTracker, Tweakable},
    },
};

/// B GuiControlsState的业务投影。原tag：键盘0、鼠标1、重置确认2、浏览3。
/// 采集暂存和档案分离；确认按钮前不改写真实键位，也不提前持久化。
#[derive(Debug)]
pub struct KeyBindingsMenuState {
    pub group: usize,
    page: KeyBindingsPage,
    capture_frames: u32,
    captured_key: Option<KeyCode>,
    captured_mouse: Option<MouseButton>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyBindingsPage {
    Keyboard(KeyBinding),
    Mouse(MouseBinding),
    Reset,
    Browse,
}
impl Default for KeyBindingsMenuState {
    fn default() -> Self {
        Self {
            group: 0,
            page: KeyBindingsPage::Browse,
            capture_frames: 0,
            captured_key: None,
            captured_mouse: None,
        }
    }
}
impl KeyBindingsMenuState {
    /// 原外层仅在EscMenuState::Closed时重置；设置子页往返保留当前分组。
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// 原EventReader只在对应采集态推进cursor，浏览／重置页不消费。
    pub fn wants_keyboard_events(&self) -> bool {
        matches!(self.page, KeyBindingsPage::Keyboard(_))
    }
    pub fn wants_mouse_events(&self) -> bool {
        matches!(self.page, KeyBindingsPage::Mouse(_))
    }
    fn begin_keyboard(&mut self, key: KeyBinding) {
        self.page = KeyBindingsPage::Keyboard(key);
        self.capture_frames = 0;
        self.captured_key = None;
    }
    fn begin_mouse(&mut self, button: MouseBinding) {
        self.page = KeyBindingsPage::Mouse(button);
        self.capture_frames = 0;
        self.captured_mouse = None;
    }
    fn capture(&mut self, keyboard: &[KeyboardInput], mouse: &[MouseButtonInput]) {
        match self.page {
            KeyBindingsPage::Keyboard(_) => {
                self.capture_frames = self.capture_frames.wrapping_add(1);
                // 原read().last()先取最后事件再过滤，不取“最后一个合格事件”。
                // +54为ButtonState，未读取repeat；按原保留重复事件的可达路径。
                if let Some(event) = keyboard.last()
                    && event.key_code != KeyCode::Escape
                    && event.state == ButtonState::Pressed
                    && self.captured_key.is_none()
                    && self.capture_frames >= 2
                {
                    self.captured_key = Some(event.key_code);
                }
            }
            KeyBindingsPage::Mouse(_) => {
                // 原+10读取MouseButton而非ButtonState。只排除Left，含释放事件。
                if let Some(event) = mouse.last()
                    && event.button != MouseButton::Left
                {
                    self.captured_mouse = Some(event.button);
                }
            }
            _ => {}
        }
    }
}
#[derive(Default, Debug)]
pub struct KeyBindingsMenuOutcome {
    pub profile_changed: bool,
    pub back_to_controls: bool,
}

/// 原settings模板内的专用键位入口。调用方负责相同帧的事件顺序和SaveGameProfileCmd。
/// 使用现有真实模板与MEI输入消费，不创建另一套界面或平行键位存储。
pub fn ui_settings_keybindings(
    template: &TemplateView,
    local: &mut KeyBindingsMenuState,
    profile: &mut GameProfile,
    keyboard: &[KeyboardInput],
    mouse: &[MouseButtonInput],
    tab_delta: i32,
    is_demo: bool,
) -> KeyBindingsMenuOutcome {
    let mut outcome = KeyBindingsMenuOutcome::default();
    let header = template.slot("header");
    let title = header.label(lang_lib::lookup("Menu.Settings.Controls.KeyBindings").to_string());
    let title_content = title.content_mut::<LabelContent>();
    title_content.text.set_metrics(90.).expect("原键位标题字号");
    title_content.text.set_weight(tg_text::Weight(800));
    if local.page == KeyBindingsPage::Browse {
        local.group = cycle_group(local.group, tab_delta);
        // 原读取后立即归零；modal不读取这条路径，仍由layout阶段清理。
        template
            .context
            .content_mut::<toolbar::mei::widgets::TabSwitcherContent>()
            .delta = 0;
        let mut hints = HintTracker::default();
        template.slot("categories").tweaker_w_font_size(
            "",
            hints.text(""),
            350.,
            60.,
            CategoryTweak(&mut local.group),
        );
        let settings = template.slot("settings");
        settings.node_mut().default_focus();
        key_bindings_sub_menu(settings, local, &profile.key_bindings, is_demo);
        let back = template
            .slot("back")
            .menu_button(&lang_lib::lookup("Menu.Back"))
            .menu_hotkey(18);
        back.node_mut().sound_click = 9;
        outcome.back_to_controls = back.node().used();
        return outcome;
    }
    let settings = template.slot("settings");
    // 原三种modal使用相同匿名子节点身份和remap_keys模板。
    let child = ordinary_index(settings, 0);
    let modal = child.template("menu/remap_keys");
    local.capture(keyboard, mouse);
    let notice = modal.slot("notice");
    match local.page {
        KeyBindingsPage::Keyboard(action) => {
            action_label(notice, action.action_desc(), 32., 64.);
            notice.label("");
            let old = key_text(action.get_keycode(&profile.key_bindings));
            let selected = local.captured_key.map_or_else(
                || format!("{} ⌨", lang_lib::lookup("Menu.Settings.Controls.PressAKey")),
                |key| key_text(Some(key)).to_owned(),
            );
            label_size(notice, &format!("{old}  -> {selected}"), 32.);
            let answer = modal.slot("answer");
            if local.captured_key.is_some() {
                let confirm =
                    answer.menu_button(&lang_lib::lookup("Menu.Settings.Controls.Confirm"));
                confirm.node_mut().default_focus();
                if confirm.node().used() {
                    profile.key_bindings.assign_key(action, local.captured_key);
                    outcome.profile_changed = true;
                    local.page = KeyBindingsPage::Browse;
                }
            }
            // 原未采集时也只给Cancel；Esc取消，不将Escape绑定到其他动作。
            if (if local.captured_key.is_none() {
                ordinary_index(answer, 0)
            } else {
                answer
            })
            .menu_button(&lang_lib::lookup("Cancel"))
            .menu_hotkey(18)
            .node()
            .used()
            {
                local.page = KeyBindingsPage::Browse;
            }
        }
        KeyBindingsPage::Mouse(action) => {
            action_label(notice, action.action_desc(), 32., 64.);
            notice.label("");
            // B默认鼠标动作始终有键，旧档允许None时用同样的未绑定占位。
            let old = action
                .get_mouse_button(&profile.key_bindings)
                .map(mouse_button_to_string)
                .unwrap_or_else(|| "?".into());
            let selected = local
                .captured_mouse
                .map(mouse_button_to_string)
                .unwrap_or_else(|| {
                    format!("{} 🖱", lang_lib::lookup("Menu.Settings.Controls.PressAKey"))
                });
            label_size(notice, &format!("{old}  -> {selected}"), 32.);
            let answer = modal.slot("answer");
            if local.captured_mouse.is_some() {
                let confirm =
                    answer.menu_button(&lang_lib::lookup("Menu.Settings.Controls.Confirm"));
                confirm.node_mut().default_focus();
                if confirm.node().used() {
                    profile
                        .key_bindings
                        .assign_mouse_button(action, local.captured_mouse);
                    outcome.profile_changed = true;
                    local.page = KeyBindingsPage::Browse;
                }
            }
            if answer
                .menu_button(&lang_lib::lookup("Cancel"))
                .menu_hotkey(18)
                .node()
                .used()
            {
                local.page = KeyBindingsPage::Browse;
            }
        }
        KeyBindingsPage::Reset => {
            label_size(notice, &lang_lib::lookup("AreYouSure"), 32.);
            let answer = modal.slot("answer");
            if answer
                .menu_button(&lang_lib::lookup("AreYouSure.ImSure"))
                .node()
                .used()
            {
                profile.key_bindings = GameKeyBindings::default();
                local.group = 0;
                local.page = KeyBindingsPage::Browse;
                outcome.profile_changed = true;
            }
            if answer
                .menu_button(&lang_lib::lookup("AreYouSure.No"))
                .node()
                .used()
            {
                local.page = KeyBindingsPage::Browse;
            }
        }
        KeyBindingsPage::Browse => unreachable!("浏览页已在上方返回"),
    }
    outcome
}
fn cycle_group(group: usize, delta: i32) -> usize {
    assert!(group < 7, "原键位类别只有7个");
    ((group as i64 + i64::from(delta)).rem_euclid(7)) as usize
}
struct CategoryTweak<'a>(&'a mut usize);
impl Tweakable for CategoryTweak<'_> {
    fn decrease(&mut self) {
        *self.0 = cycle_group(*self.0, -1);
    }
    fn increase(&mut self) {
        *self.0 = cycle_group(*self.0, 1);
    }
    fn apply_delta(&mut self, delta: i32) {
        *self.0 = cycle_group(*self.0, delta);
    }
    fn format(&self) -> String {
        let label = match *self.0 {
            0 => lang_lib::lookup("Menu.Settings.Controls.Camera").to_string(),
            1 | 2 => lang_lib::lookup("Menu.Settings.Controls.Toolbar").to_string(),
            3 => lang_lib::lookup("Menu.Settings.Ui").to_string(),
            4 => lang_lib::lookup("Menu.Settings.Controls.Other").to_string(),
            5 => lang_lib::lookup("Menu.Settings.Controls.Reset").to_string(),
            6 => "Experimental".into(),
            _ => unreachable!("原键位类别只有7个"),
        };
        format!("{label} ({}/{})", *self.0 + 1, 7)
    }
}
struct BindingRow {
    keys: &'static [KeyBinding],
    label: String,
    mouse: Option<(Option<MouseBinding>, &'static str)>,
}
fn rows(group: usize, is_demo: bool) -> Vec<BindingRow> {
    use KeyBinding::*;
    let row = |keys, label, mouse| BindingRow { keys, label, mouse };
    let l = |key| lang_lib::lookup(key).to_string();
    match group {
        0 => vec![
            row(
                &[MoveForward, MoveLeft, MoveBack, MoveRight],
                format!("{} ⬆️ / ⬅️ / ⬇️ / ➡️", l("KeyBindings.Move")),
                Some((Some(MouseBinding::Pan), "🖱️")),
            ),
            row(&[MoveDown, MoveUp], l("KeyBindings.MoveDownUp"), None),
            row(
                &[RotateLeft, RotateRight],
                format!("{} ⬅️ / ➡️", l("KeyBindings.Rotate")),
                Some((Some(MouseBinding::Rotate), "🖱️")),
            ),
            row(
                &[ZoomIn, ZoomOut],
                l("KeyBindings.ZoomInOut"),
                Some((None, "🖱️ WHEEL")),
            ),
        ],
        1 => vec![
            row(&[Building], "building".into(), None),
            row(&[Path], "path".into(), None),
            row(&[Decorators], "decorators".into(), None),
            row(&[Customize], "customize".into(), None),
            row(&[Eraser], "eraser".into(), None),
        ],
        2 => {
            let mut result = vec![
                row(&[Terrain], "terrain".into(), None),
                row(&[Garden], "garden".into(), None),
            ];
            if !is_demo {
                result.extend([
                    row(&[Tree], "tree".into(), None),
                    row(&[Water], "water".into(), None),
                ]);
            }
            result.extend([
                row(&[Photomode], "photomode".into(), None),
                row(&[TimeOfDay], "time of day".into(), None),
            ]);
            result
        }
        3 => [
            ([UiCancel], "Cancel"),
            ([UiConfirm], "Menu.Settings.Controls.Confirm"),
            ([UiLeft], "Menu.Settings.Controls.Left"),
            ([UiRight], "Menu.Settings.Controls.Right"),
            ([UiDown], "Menu.Settings.Controls.Down"),
            ([UiUp], "Menu.Settings.Controls.Up"),
            ([UiTabPrev], "Menu.Settings.Controls.TabPrevious"),
            ([UiTabNext], "Menu.Settings.Controls.TabNext"),
        ]
        .into_iter()
        .map(|(keys, key)| {
            row(
                match keys[0] {
                    UiCancel => &[UiCancel],
                    UiConfirm => &[UiConfirm],
                    UiLeft => &[UiLeft],
                    UiRight => &[UiRight],
                    UiDown => &[UiDown],
                    UiUp => &[UiUp],
                    UiTabPrev => &[UiTabPrev],
                    UiTabNext => &[UiTabNext],
                    _ => unreachable!(),
                },
                l(key),
                None,
            )
        })
        .collect(),
        // 用户已移除反馈模块；保留旧档schema槽，删除不可用的SendFeedback编辑入口。
        4 => vec![
            row(&[Undo, Redo], l("KeyBindings.UndoRedo"), None),
            row(
                &[FocusEdit],
                l("KeyBindings.FocusOrDeselect"),
                Some((Some(MouseBinding::FocusEdit), "🖱️ ")),
            ),
            row(&[DisableSnapping], l("KeyBindings.DisableSnapping"), None),
            row(&[HideUiPhotomode], l("KeyBindings.HideUiInPhotoMode"), None),
        ],
        5 => vec![],
        6 => vec![row(
            &[ExperimentalCamera],
            "Camera keyframe UI".into(),
            None,
        )],
        _ => unreachable!("原键位类别只有7个"),
    }
}
/// 原1400854a0：7类别、分组行、键盘/鼠标选择和Reset确认入口。
pub fn key_bindings_sub_menu(
    node: NodeContext,
    local: &mut KeyBindingsMenuState,
    bindings: &GameKeyBindings,
    is_demo: bool,
) {
    for row in rows(local.group, is_demo) {
        let container = node.child(&row.label);
        let template = container.template("menu/key_remapping_row");
        let description = row.keys.first().map(|key| key.action_desc());
        // 多键行的描述有运行时本地化文字，不能借static枚举储存，独立分流。
        if row.keys.len() == 1 {
            action_label(
                template.slot("action"),
                description.expect("单键行描述"),
                24.,
                48.,
            );
        } else {
            label_size(template.slot("action"), &row.label, 24.);
        }
        let keynode = template.slot("key");
        for (index, action) in row.keys.iter().copied().enumerate() {
            let button = binding_button(
                keynode,
                index as u64,
                &format!("<{}>", key_text(action.get_keycode(bindings))),
            );
            if button.node().used() {
                local.begin_keyboard(action);
            }
        }
        if let Some((mouse, label)) = row.mouse {
            keynode.label("/");
            label_size(keynode, label, 24.);
            if let Some(mouse) = mouse {
                let text = mouse
                    .get_mouse_button(bindings)
                    .map(mouse_row_name)
                    .unwrap_or_else(|| "?".into());
                let button = binding_button(keynode, row.keys.len() as u64, &format!("<{text}>"));
                if button.node().used() {
                    local.begin_mouse(mouse);
                }
            }
        }
    }
    if local.group == 5 {
        let button = node.menu_button(&lang_lib::lookup("KeyBindings.ResetToDefaults"));
        button
            .content_mut::<MenuButtonContent>()
            .text
            .set_metrics(24.)
            .expect("原重置按钮字号");
        if button.node().used() {
            local.page = KeyBindingsPage::Reset;
        }
    }
}
fn mouse_row_name(button: MouseButton) -> String {
    match button {
        MouseButton::Back => "Back".into(),
        MouseButton::Forward => "Forward".into(),
        _ => mouse_button_to_string(button),
    }
}
fn key_text(key: Option<KeyCode>) -> &'static str {
    key.map_or("?", |key| keycode_to_str(key).unwrap_or("???"))
}
fn ordinary_index(parent: NodeContext, index: u64) -> NodeContext {
    let id = calculate_hash(&(index, 0xa79b7268a2a968d9u64));
    parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, id))))
}
fn binding_button(parent: NodeContext, index: u64, text: &str) -> NodeContext {
    let id = calculate_hash(&(index, 0x615bfcfe4597ab8fu64));
    let node = parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, id))));
    let content = node.get_content_mut_impl::<MenuButtonContent>(false);
    content.set_text(text.into());
    content.text.set_metrics(24.).expect("原键位按钮字号");
    node.node_mut().with_nav(true);
    node.node_mut().set_sounds(0, 2);
    node
}
fn label_size(node: NodeContext, text: &str, size: f32) {
    node.label(text)
        .content_mut::<LabelContent>()
        .text
        .set_metrics(size)
        .expect("原键位文字字号");
}
fn action_label(node: NodeContext, desc: ActionDescription, font_size: f32, image_height: f32) {
    match desc {
        ActionDescription::Image(image) => {
            node.image(image, image, image_height);
        }
        ActionDescription::Text(text) => label_size(node, text, font_size),
        ActionDescription::Localized { key, suffix } => label_size(
            node,
            &format!("{}{suffix}", lang_lib::lookup(key)),
            font_size,
        ),
    }
}

/// 原140088240七行，直接修改权威GameConfig.game；None pivot原本就会写成0。
pub fn show_control_tweakers(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
) {
    use country_core::systems::main_camera::CameraRotationPivot as P;
    use toolbar::mei::widgets::tweaker::{TweakableBool, TweakableEnum, TweakableFloatWithStep};
    let game = &mut config.game;
    for (key, value) in [
        (
            "Menu.Settings.Controls.RotationSpeed",
            &mut game.camera_rotation_sensitivity,
        ),
        (
            "Menu.Settings.Controls.MovementSpeed",
            &mut game.camera_movement_speed,
        ),
    ] {
        *changed |= node.tweaker(
            &lang_lib::lookup(key),
            hints.text(""),
            260.,
            TweakableFloatWithStep {
                value,
                step: 0.1,
                min: 0.1,
                max: 2.,
            },
        );
    }
    for (key, value) in [
        ("Menu.Settings.Controls.InvertY", &mut game.camera_invert_y),
        ("Menu.Settings.Controls.InvertX", &mut game.camera_invert_x),
    ] {
        *changed |= node.tweaker(
            &lang_lib::lookup(key),
            hints.text(""),
            260.,
            TweakableBool(value),
        );
    }
    const PIVOTS: [P; 6] = [
        P::CursorDepth,
        P::FilteredCursorDepth,
        P::CenterDepth,
        P::FilteredCenterDepth,
        P::CursorTerrain,
        P::CenterTerrain,
    ];
    for (key, value) in [
        (
            "Menu.Settings.Controls.CameraRotationPivot",
            &mut game.camera_rotation_pivot,
        ),
        (
            "Menu.Settings.Controls.CameraZoomPivot",
            &mut game.camera_zoom_pivot,
        ),
    ] {
        *changed |= node.tweaker(
            &lang_lib::lookup(key),
            hints.text(""),
            260.,
            TweakableEnum {
                value: value.get_or_insert(P::CursorDepth),
                variants: &PIVOTS,
                formatter: pivot_text,
            },
        );
    }
    if matches!(
        game.camera_rotation_pivot,
        Some(P::FilteredCursorDepth | P::FilteredCenterDepth)
    ) {
        *changed |= node.tweaker(
            &lang_lib::lookup("Menu.Settings.Controls.CameraPivotRegionSize"),
            hints.text(""),
            260.,
            TweakableFloatWithStep {
                value: &mut game.camera_pivot_filter_size,
                step: 0.05,
                min: 0.,
                max: 0.5,
            },
        );
    }
}
/// 原140912500跳表逐变体对应本地化键。
fn pivot_text(value: &country_core::systems::main_camera::CameraRotationPivot) -> String {
    use country_core::systems::main_camera::CameraRotationPivot as P;
    lang_lib::lookup(match value {
        P::CursorDepth => "Menu.Settings.CameraRotationPivot.CursorPoint",
        P::FilteredCursorDepth => "Menu.Settings.CameraRotationPivot.CursorRegion",
        P::CenterDepth => "Menu.Settings.CameraRotationPivot.CenterPoint",
        P::FilteredCenterDepth => "Menu.Settings.CameraRotationPivot.CenterRegion",
        P::CursorTerrain => "Menu.Settings.CameraRotationPivot.CursorTerrain",
        P::CenterTerrain => "Menu.Settings.CameraRotationPivot.CenterTerrain",
    })
    .to_string()
}

#[cfg(test)]
mod keybindings_tests {
    use super::*;
    use bevy_ecs::entity::Entity;
    use bevy_input::keyboard::Key;
    use toolbar::mei::{core::MeiContext, widgets::ImageContent};
    fn keyboard(code: KeyCode, state: ButtonState, repeat: bool) -> KeyboardInput {
        KeyboardInput {
            key_code: code,
            logical_key: Key::Character("x".into()),
            state,
            text: None,
            repeat,
            window: Entity::PLACEHOLDER,
        }
    }
    #[test]
    fn capture_matches_last_event_gate_escape_repeat_and_mouse_asymmetry() {
        let mut local = KeyBindingsMenuState::default();
        local.begin_keyboard(KeyBinding::MoveForward);
        local.capture(&[keyboard(KeyCode::KeyJ, ButtonState::Pressed, false)], &[]);
        assert_eq!(local.captured_key, None);
        // 最后释放事件会屏蔽前面的按下，而不是倒着找任意合格事件。
        local.capture(
            &[
                keyboard(KeyCode::KeyJ, ButtonState::Pressed, false),
                keyboard(KeyCode::KeyJ, ButtonState::Released, false),
            ],
            &[],
        );
        assert_eq!(local.captured_key, None);
        local.capture(
            &[keyboard(KeyCode::Escape, ButtonState::Pressed, false)],
            &[],
        );
        assert_eq!(local.captured_key, None);
        local.capture(&[keyboard(KeyCode::KeyK, ButtonState::Pressed, true)], &[]);
        assert_eq!(local.captured_key, Some(KeyCode::KeyK));
        local.capture(&[keyboard(KeyCode::KeyL, ButtonState::Pressed, false)], &[]);
        assert_eq!(local.captured_key, Some(KeyCode::KeyK));
        local.begin_mouse(MouseBinding::Rotate);
        let mouse = |button, state| MouseButtonInput {
            button,
            state,
            window: Entity::PLACEHOLDER,
        };
        local.capture(
            &[],
            &[
                mouse(MouseButton::Right, ButtonState::Pressed),
                mouse(MouseButton::Left, ButtonState::Released),
            ],
        );
        assert_eq!(local.captured_mouse, None);
        local.capture(&[], &[mouse(MouseButton::Right, ButtonState::Released)]);
        assert_eq!(local.captured_mouse, Some(MouseButton::Right));
        local.capture(&[], &[mouse(MouseButton::Other(19), ButtonState::Pressed)]);
        assert_eq!(local.captured_mouse, Some(MouseButton::Other(19)));
        local.group = 6;
        local.reset();
        assert_eq!(local.group, 0);
        assert_eq!(local.page, KeyBindingsPage::Browse);
        assert_eq!(local.captured_key, None);
        assert_eq!(local.captured_mouse, None);
    }
    #[test]
    fn original_group_rows_include_demo_mouse_and_user_exclusions() {
        assert_eq!(
            rows(0, false)
                .iter()
                .map(|r| r.keys.len())
                .collect::<Vec<_>>(),
            [4, 2, 2, 2]
        );
        assert_eq!(
            rows(0, false)
                .iter()
                .filter_map(|r| r.mouse.and_then(|m| m.0))
                .collect::<Vec<_>>(),
            [MouseBinding::Pan, MouseBinding::Rotate]
        );
        assert_eq!(rows(2, true).len(), 4);
        assert_eq!(rows(2, false).len(), 6);
        assert_eq!(rows(3, false).len(), 8);
        assert!(
            (0..7)
                .flat_map(|g| rows(g, false))
                .flat_map(|r| r.keys.iter())
                .all(|k| *k != KeyBinding::Feedback)
        );
        for group in 0..7 {
            for delta in [i32::MIN, -15, -7, -1, 0, 1, 7, 15, i32::MAX] {
                assert_eq!(
                    cycle_group(group, delta),
                    ((group as i64 + delta as i64).rem_euclid(7)) as usize
                );
            }
        }
    }
    fn real_mei() -> MeiContext {
        let root =
            std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("原资产路径"));
        let mut mei = MeiContext::default();
        mei.load_font_table(&root.join("assets/fonts/table.ron"))
            .unwrap();
        for name in ["menu/settings", "menu/key_remapping_row", "menu/remap_keys"] {
            mei.configs.insert(
                name.into(),
                std::sync::Arc::new(
                    toolbar::mei::config::read_node_config_from_file(
                        &root.join(format!("assets/ui/{name}.ron")),
                    )
                    .unwrap(),
                ),
            );
        }
        mei
    }
    fn frame(
        mei: &mut MeiContext,
        local: &mut KeyBindingsMenuState,
        profile: &mut GameProfile,
        keyboard: &[KeyboardInput],
        mouse: &[MouseButtonInput],
        tab: i32,
    ) -> KeyBindingsMenuOutcome {
        mei.recycle_frame_nodes();
        let template = mei
            .safe_frame("settings-menu", true)
            .tab_switcher("tab_switcher")
            .template("menu/settings");
        let outcome =
            ui_settings_keybindings(&template, local, profile, keyboard, mouse, tab, false);
        for node in mei.nodes.values_mut() {
            node.state_bits &= !0x30;
        }
        outcome
    }
    fn activate(mei: &mut MeiContext, text: &str) {
        let id = *mei
            .nodes
            .iter()
            .find(|(_, n)| {
                let any: &dyn std::any::Any = n.content.as_ref();
                any.downcast_ref::<MenuButtonContent>()
                    .is_some_and(|b| b.pending_text.as_deref() == Some(text))
            })
            .unwrap_or_else(|| panic!("实际按钮缺失 {text}"))
            .0;
        mei.nodes.get_mut(&id).unwrap().state_bits |= 0x10;
    }
    fn has_button(mei: &MeiContext, text: &str) -> bool {
        mei.nodes.values().any(|node| {
            let any: &dyn std::any::Any = node.content.as_ref();
            any.downcast_ref::<MenuButtonContent>()
                .is_some_and(|b| b.pending_text.as_deref() == Some(text))
        })
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；CPU真实MEI消费者，不发送OS输入"]
    fn actual_templates_capture_confirm_cancel_reset_roundtrip_and_back() {
        let mut mei = real_mei();
        let mut local = KeyBindingsMenuState::default();
        let mut profile = GameProfile::default();
        let original = profile.key_bindings.clone();
        assert!(!frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert!(has_button(&mei, "<W>"));
        assert!(has_button(&mei, "<S>"));
        assert!(has_button(&mei, "<A>"));
        assert!(has_button(&mei, "<D>"));
        assert!(has_button(&mei, "<MMB>"));
        assert!(has_button(&mei, "<RMB>"));
        activate(&mut mei, "<W>");
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        assert_eq!(
            local.page,
            KeyBindingsPage::Keyboard(KeyBinding::MoveForward)
        );
        let key = keyboard(KeyCode::KeyJ, ButtonState::Pressed, false);
        assert!(
            !frame(
                &mut mei,
                &mut local,
                &mut profile,
                std::slice::from_ref(&key),
                &[],
                0
            )
            .profile_changed
        );
        assert!(!has_button(
            &mei,
            &lang_lib::lookup("Menu.Settings.Controls.Confirm")
        ));
        assert_eq!(profile.key_bindings, original);
        frame(&mut mei, &mut local, &mut profile, &[key], &[], 0);
        assert!(has_button(
            &mei,
            &lang_lib::lookup("Menu.Settings.Controls.Confirm")
        ));
        assert_eq!(profile.key_bindings, original);
        activate(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Controls.Confirm"),
        );
        assert!(frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(profile.key_bindings.bindings[0], Some(KeyCode::KeyJ));
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        assert!(has_button(&mei, "<J>"));
        activate(&mut mei, "<J>");
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("Cancel"));
        assert!(!frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(profile.key_bindings.bindings[0], Some(KeyCode::KeyJ));
        // 原图标动作确实生成ImageContent，而不是把图标路径打印为文字。
        frame(&mut mei, &mut local, &mut profile, &[], &[], 1);
        assert!(mei.nodes.values().any(|node| {
            let any: &dyn std::any::Any = node.content.as_ref();
            any.downcast_ref::<ImageContent>()
                .is_some_and(|i| i.image == "toolbar/building" && i.height == 48.)
        }));
        frame(&mut mei, &mut local, &mut profile, &[], &[], 4);
        assert_eq!(local.group, 5);
        activate(&mut mei, &lang_lib::lookup("KeyBindings.ResetToDefaults"));
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        assert_eq!(local.page, KeyBindingsPage::Reset);
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("AreYouSure.No"));
        assert!(!frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(profile.key_bindings.bindings[0], Some(KeyCode::KeyJ));
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("KeyBindings.ResetToDefaults"));
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("AreYouSure.ImSure"));
        assert!(frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(local.group, 0);
        assert_eq!(profile.key_bindings, GameKeyBindings::default());
        let text = ron::to_string(&profile).unwrap();
        let restored: GameProfile = ron::from_str(&text).unwrap();
        assert_eq!(restored.key_bindings, profile.key_bindings);
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("Menu.Back"));
        assert!(frame(&mut mei, &mut local, &mut profile, &[], &[], 0).back_to_controls);
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原模板和字体；CPU消费者，不发送OS输入"]
    fn actual_mouse_modal_retains_other_identity_and_requires_confirmation() {
        let mut mei = real_mei();
        let mut local = KeyBindingsMenuState::default();
        let mut profile = GameProfile::default();
        let original = profile.key_bindings.clone();
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, "<MMB>");
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        assert_eq!(local.page, KeyBindingsPage::Mouse(MouseBinding::Pan));
        let event = MouseButtonInput {
            button: MouseButton::Other(1),
            state: ButtonState::Released,
            window: Entity::PLACEHOLDER,
        };
        assert!(!frame(&mut mei, &mut local, &mut profile, &[], &[event], 0).profile_changed);
        assert_eq!(local.captured_mouse, Some(MouseButton::Other(1)));
        assert_eq!(profile.key_bindings, original);
        activate(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Controls.Confirm"),
        );
        assert!(frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(
            profile.key_bindings.mouse,
            [
                Some(MouseButton::Right),
                Some(MouseButton::Other(1)),
                Some(MouseButton::Right)
            ]
        );
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        assert!(has_button(&mei, "<Mouse button 1>"));
        activate(&mut mei, "<Mouse button 1>");
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        frame(&mut mei, &mut local, &mut profile, &[], &[], 0);
        activate(&mut mei, &lang_lib::lookup("Cancel"));
        assert!(!frame(&mut mei, &mut local, &mut profile, &[], &[], 0).profile_changed);
        assert_eq!(profile.key_bindings.mouse[1], Some(MouseButton::Other(1)));
    }
}
