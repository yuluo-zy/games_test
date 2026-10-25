//! 原字体与真实MEI消费者验证；不发送OS输入，不代替完整游戏验收。
use bevy_app::{App, Update};
use bevy_ecs::event::Events;
use bevy_input::mouse::MouseWheel;
use bevy_time::Time;
use country_core::{
    input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet},
    resources::{mouse_terrain_raycast::CursorPositionSS, window::WindowSize},
    systems::{
        audio::PlayMenuUiSound,
        ui::resources::{
            ui_state::{UiInteraction, UiState, WidgetId},
            widget_params::{UiFocus, UiNavState},
        },
    },
};
use glam::Vec2;
use std::{path::PathBuf, sync::Arc};
use toolbar::mei::{
    core::{MeiContext, NodeContent, NodeDrawContext},
    systems::mei_pre_update,
    widgets::{MenuButtonContent, PillButtonContent},
};

/// 原 6816e2 写入与 681938 读取同一字节；覆盖全部动作和原 0x28 哨兵。
/// 这里执行真实控件 setter 和 pre_update，不代表 OS 输入验收。
#[test]
fn pill_hotkey_binding_rebinding_and_action_phases() {
    let mut mei = MeiContext::default();
    let root = mei.safe_frame("pill-hotkey", false);
    let button = root.pill_button("back", "menu/back");
    for action in 0..=40u8 {
        assert_eq!(button.pill_hotkey(action).id, button.id);
        let content = button.content_mut::<PillButtonContent>();
        assert_eq!((content.hotkey, content.action), (action, action));
        for (phase, timer, expected) in [
            (DigitalActionPhase::Idle, 0., false),
            (DigitalActionPhase::JustPressed, 0., true),
            (DigitalActionPhase::Held, 1., false),
            (DigitalActionPhase::Repeating, 0.1, false),
            (DigitalActionPhase::Repeating, 0., true),
            (DigitalActionPhase::Repeating, -0.1, true),
            (DigitalActionPhase::JustReleased, 0., false),
        ] {
            let mut actions = MainActionSet::default();
            if action != 40 {
                actions.digital[usize::from(action)] = DigitalActionState {
                    phase,
                    timer_seconds: timer,
                };
            } else {
                // 未绑定时所有输入同时触发也不应激活，更不能越界读取槽40。
                actions.digital.fill(DigitalActionState {
                    phase: DigitalActionPhase::JustPressed,
                    timer_seconds: 0.,
                });
            }
            let original_actions = actions.digital;
            let mut state = 0x05;
            content.pre_update(&actions, Vec2::ZERO, &mut state);
            assert_eq!(
                state,
                0x05 | if expected && action != 40 { 0x20 } else { 0 },
                "action={action}, phase={phase:?}, timer={timer}"
            );
            assert_eq!(actions.digital, original_actions);
        }
        if action != 40 {
            let mut actions = MainActionSet::default();
            actions.digital[(usize::from(action) + 1) % 40] = DigitalActionState {
                phase: DigitalActionPhase::JustPressed,
                timer_seconds: 0.,
            };
            let mut state = 0;
            content.pre_update(&actions, Vec2::ZERO, &mut state);
            assert_eq!(state, 0, "不能读取上次绑定或其它动作：{action}");
        }
    }
}

/// 用实际 mei_pre_update 验证 setter → 动作集 → 节点激活。
#[test]
fn pill_hotkey_flows_through_actual_mei_pre_update() {
    let mut mei = MeiContext::default();
    let button = mei
        .safe_frame("pill-input", false)
        .pill_button("back", "menu/back")
        .pill_hotkey(18);
    let id = button.id;
    button.node_mut().pos = Vec2::new(10., 10.);
    button.node_mut().size = Vec2::new(100., 60.);
    let mut actions = MainActionSet::default();
    actions.digital[18] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    let mut app = App::new();
    app.add_event::<MouseWheel>()
        .add_event::<PlayMenuUiSound>()
        .insert_resource(mei)
        .insert_resource(actions)
        .insert_resource(UiNavState::default())
        .insert_resource(UiState::at_startup())
        .insert_resource(WindowSize::new(1600, 1000))
        .insert_resource(CursorPositionSS(Vec2::splat(500.)))
        .insert_resource(Time::<()>::default())
        .add_systems(Update, mei_pre_update);
    app.update();
    assert_ne!(
        app.world().resource::<MeiContext>().previous_nodes[&id].state_bits & 0x20,
        0
    );
}

fn fonts() -> (Arc<tg_text::FontTable>, tg_text::TextRenderer) {
    let root = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("set TG_INSTALL_ROOT"));
    let table = tg_text::FontTable::from_ron(
        &std::fs::read_to_string(root.join("assets/fonts/table.ron")).unwrap(),
    )
    .unwrap();
    let mut renderer = tg_text::TextRenderer::new();
    fn load_dir(path: &std::path::Path, renderer: &mut tg_text::TextRenderer) {
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                load_dir(&path, renderer);
            } else if matches!(
                path.extension().and_then(|x| x.to_str()),
                Some("ttf" | "otf")
            ) {
                renderer.load_font_from_file(path).unwrap();
            }
        }
    }
    load_dir(&root.join("assets/fonts"), &mut renderer);
    (Arc::new(table), renderer)
}

/// Bounded actual MEI pre-update input/registration/sound/owner lifecycle,
/// followed by actual owning TextFragment glyph/primitive generation. This
/// doesn't claim the entire Load/Escape caller or GPU pipeline is validated.
#[test]
#[ignore = "requires TG_INSTALL_ROOT pointing at the original installed font assets"]
fn original_menu_button_keyboard_mouse_and_real_text_output() {
    let (fonts, mut renderer) = fonts();
    for case in ["idle", "hover", "cancel", "mouse", "confirm"] {
        let mut mei = MeiContext::default();
        mei.font_table = fonts.clone();
        let root = mei.safe_frame("menu-test", false);
        let button = root.menu_button("Cancel");
        button.content_mut::<MenuButtonContent>().action = 18;
        button.node_mut().pos = Vec2::new(10., 10.);
        button.node_mut().size = Vec2::new(200., 60.);
        let id = button.id;
        let pointer = button.content_mut::<MenuButtonContent>() as *mut MenuButtonContent;
        let mut actions = MainActionSet::default();
        let pressed = DigitalActionState {
            phase: DigitalActionPhase::JustPressed,
            timer_seconds: 0.,
        };
        if case == "cancel" {
            actions.digital[18] = pressed;
        }
        if case == "confirm" {
            actions.digital[19] = pressed;
        }
        let mut nav = UiNavState::default();
        if case == "confirm" {
            nav.focus = UiFocus {
                tag: 2,
                _pad: 0,
                payload: id.0,
            };
            nav.hold = true;
        }
        let cursor = if matches!(case, "hover" | "mouse") {
            Vec2::new(20., 20.)
        } else {
            Vec2::splat(500.)
        };
        let ui = if case == "mouse" {
            UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: 1,
                widget: WidgetId::from_name(&id.0.to_string()),
            })
        } else {
            UiState::at_startup()
        };
        let mut app = App::new();
        app.add_event::<MouseWheel>()
            .add_event::<PlayMenuUiSound>()
            .insert_resource(mei)
            .insert_resource(actions)
            .insert_resource(nav)
            .insert_resource(ui)
            .insert_resource(WindowSize::new(1600, 1000))
            .insert_resource(CursorPositionSS(cursor))
            .insert_resource(Time::<()>::default())
            .add_systems(Update, mei_pre_update);
        app.update();
        let states = app.world().resource::<MeiContext>().previous_nodes[&id].state_bits;
        assert_eq!(states & 0x20 != 0, case == "cancel", "{case}");
        assert_eq!(
            states & 0x10 != 0,
            matches!(case, "mouse" | "confirm"),
            "{case}"
        );
        assert_eq!(
            states & 1 != 0,
            matches!(case, "hover" | "mouse" | "confirm"),
            "{case}"
        );
        let sounds: Vec<_> = app
            .world_mut()
            .resource_mut::<Events<PlayMenuUiSound>>()
            .drain()
            .map(|e| e.kind)
            .collect();
        assert_eq!(
            sounds.contains(&0),
            matches!(case, "hover" | "mouse" | "confirm"),
            "{case}"
        );
        assert_eq!(
            sounds.contains(&2),
            // 原6842e0先调用所有content.pre_update，再保存声音比较的旧状态。
            // cancel的bit20已在前一阶段置位，不产生这一阶段的点击上升沿。
            // 见toolbar-input-sound-order-native.json原体第536/601/872行。
            matches!(case, "mouse" | "confirm"),
            "{case}"
        );
        let mei = app.world_mut().resource_mut::<MeiContext>();
        let root = mei.into_inner().safe_frame("menu-test", false);
        let same = root.menu_button("Cancel");
        assert_eq!(same.id, id);
        assert_eq!(
            same.content_mut::<MenuButtonContent>() as *mut MenuButtonContent,
            pointer
        );
        let content = same.content_mut::<MenuButtonContent>();
        let text = content.pending_text.take().unwrap();
        content.text.set_text(&mut renderer, text).unwrap();
        content.text.prepare(&mut renderer).unwrap();
        assert!(content.text.bounds()[0] > 0.);
        content.draw(&mut NodeDrawContext {
            text: &mut renderer,
            center: Vec2::new(110., 40.),
            extent: Vec2::new(200., 60.),
            opacity: 0.5,
            pixels_per_point: 1.,
            cursor,
            elapsed: 1. / 60.,
            state: states,
        });
        // 当前文字库在render阶段生成图元，prepare只排版；核验真实绘制结果。
        assert!(!content.text.primitives().is_empty(), "{case}");
        let batch = renderer.consume().unwrap();
        assert!(!batch.primitives.is_empty());
        let expected = tg_text::scale_u32_packed_srgba_color(
            if states & 1 != 0 {
                u32::MAX
            } else {
                0xffffffa0
            },
            0.5,
        );
        assert!(batch.primitives.iter().all(|p| p.color == expected));
    }
}

mod steam_hint_tests {
    use country_core::resources::steam::*;
    use std::sync::Mutex;
    use std::{collections::HashMap, sync::Arc};
    #[derive(Default)]
    struct Sdk {
        calls: Mutex<Vec<(bool, u64, u64)>>,
        connected: Mutex<Vec<u64>>,
        states: Mutex<HashMap<(u64, u64), bool>>,
    }
    impl SteamSdk for Sdk {
        fn user_steam_id(&self) -> u64 {
            unreachable!()
        }
        fn run_frame(&self) {}
        fn connected_controllers(&self) -> Vec<u64> {
            self.connected.lock().unwrap().clone()
        }
        fn action_set_handle(&self, _: &str) -> Option<u64> {
            unreachable!()
        }
        fn analog_action_handle(&self, _: &str) -> Option<u64> {
            unreachable!()
        }
        fn digital_action_handle(&self, _: &str) -> Option<u64> {
            unreachable!()
        }
        fn analog_action_data(&self, _: u64, _: u64) -> AnalogActionData {
            AnalogActionData::default()
        }
        fn digital_action_data(&self, controller: u64, handle: u64) -> DigitalActionData {
            DigitalActionData {
                state: self
                    .states
                    .lock()
                    .unwrap()
                    .get(&(controller, handle))
                    .copied()
                    .unwrap_or(false),
                active: true,
            }
        }
        fn activate_action_set(&self, _: u64, _: u64) {}
        fn init_input(&self) -> bool {
            unreachable!()
        }
        fn is_steam_running_on_steam_deck(&self) -> bool {
            unreachable!()
        }
        fn show_binding_panel(&self, _: u64) {
            unreachable!()
        }
        fn activate_game_overlay_to_web_page(&self, _: &str) {
            unreachable!()
        }
        fn hook_screenshots(&self, _: bool) {
            unreachable!()
        }
        fn show_floating_gamepad_text_input(&self, _: u32, _: i32, _: i32, _: i32, _: i32) {
            unreachable!()
        }
        fn analog_action_origins(&self, controller: u64, set: u64, handle: u64) -> Vec<u64> {
            assert_eq!(controller, 7);
            self.calls.lock().unwrap().push((true, set, handle));
            vec![0, 11, 22]
        }
        fn digital_action_origins(&self, controller: u64, set: u64, handle: u64) -> Vec<u64> {
            assert_eq!(controller, 7);
            self.calls.lock().unwrap().push((false, set, handle));
            vec![0, 11, 22]
        }
        fn glyph_for_action_origin(&self, origin: u64) -> String {
            format!("glyph/{origin}.png")
        }
    }
    fn input(sdk: Arc<Sdk>) -> SteamInput {
        SteamInput {
            client: sdk.clone(),
            active_controller: 7,
            last_controller: 0,
            main_action_set: SteamMainActionSet {
                loaded: 1,
                action_set_handle: 100,
                analog: [10, 11, 12],
                digital: std::array::from_fn(|i| 200 + i as u64),
            },
            menu_action_set: SteamMenuActionSet {
                loaded: 1,
                action_set_handle: 300,
                analog: [400],
                digital: std::array::from_fn(|i| 500 + i as u64),
            },
            actions_by_type: HashMap::new(),
        }
    }
    #[test]
    fn original_glyph_query_selects_all_main_and_menu_handles() {
        let sdk = Arc::new(Sdk::default());
        let mut input = input(sdk.clone());
        for (kind, count, set, base, analog) in [
            (0u16, 3, 100, 10, true),
            (1, 25, 100, 200, false),
            (2, 9, 300, 500, false),
        ] {
            for index in 0..count {
                assert_eq!(
                    input.get_action_origin_glyphs(ActionOriginGlyphQuery((index << 8) | kind)),
                    Some(vec!["glyph/11.png".into(), "glyph/22.png".into()])
                );
                assert_eq!(
                    sdk.calls.lock().unwrap().pop(),
                    Some((analog, set, base + u64::from(index)))
                );
            }
        }
        input
            .get_action_origin_glyphs(ActionOriginGlyphQuery(0x902))
            .unwrap();
        assert_eq!(sdk.calls.lock().unwrap().pop(), Some((true, 300, 400)));
        for code in [0x300, 0x1901, 0xa02, 3] {
            assert_eq!(
                input.get_action_origin_glyphs(ActionOriginGlyphQuery(code)),
                None
            );
        }
        input.active_controller = 0;
        assert!(
            input
                .get_action_origin_glyphs(ActionOriginGlyphQuery(1))
                .is_none()
        );
        input.active_controller = 7;
        input.main_action_set.loaded = 0;
        input.menu_action_set.loaded = 0;
        for code in [0, 1, 2] {
            assert!(
                input
                    .get_action_origin_glyphs(ActionOriginGlyphQuery(code))
                    .is_none()
            );
        }
        assert!(sdk.calls.lock().unwrap().is_empty());
    }

    #[test]
    fn real_steam_system_projects_all_native_sources_held_input_and_controller_lifecycle() {
        use bevy_app::{App, Update};
        use country_core::{
            input::ActiveInputModeTracker,
            input::action_set::ActionSetValues,
            resources::{gui::EscMenuState, time::GameTime},
            systems::{
                backend::input::translate_input_to_cursor_moved::InputCursorDelta,
                steam::{SteamClientResource, update_steam_inputs},
            },
        };
        let sdk = Arc::new(Sdk::default());
        *sdk.connected.lock().unwrap() = vec![7, 8];
        let mut app = App::new();
        let mut elapsed = bevy_time::Time::<()>::default();
        elapsed.advance_by(std::time::Duration::from_secs(100));
        app.add_event::<InputCursorDelta>()
            .insert_resource(SteamClientResource {
                client: Some(sdk.clone()),
            })
            .insert_resource(input(sdk.clone()))
            .insert_resource(ActionSetValues::default())
            .insert_resource(ActiveInputModeTracker::new_default_mouse_keyboard(
                std::time::Duration::ZERO,
            ))
            .insert_resource(GameTime {
                delta: 0.1,
                ..Default::default()
            })
            .insert_resource(elapsed)
            .insert_resource(EscMenuState::Closed)
            .add_systems(Update, update_steam_inputs);
        let oracle: Vec<(u16, u16)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/native/steam-action-origin-map.ron"
        )))
        .unwrap();
        assert_eq!(oracle.len(), 31);
        for menu in [false, true] {
            *app.world_mut().resource_mut::<EscMenuState>() = if menu {
                EscMenuState::Main
            } else {
                EscMenuState::Closed
            };
            let mut expected = oracle
                .iter()
                .copied()
                .filter(|(_, source)| source & 1 == u16::from(menu))
                .collect::<Vec<_>>();
            expected.sort();
            for &(target, code) in &expected {
                sdk.states.lock().unwrap().clear();
                sdk.states.lock().unwrap().insert(
                    (7, if menu { 500 } else { 200 } + u64::from(code >> 8)),
                    true,
                );
                for _ in 0..2 {
                    app.world_mut().resource_mut::<ActionSetValues>().current = Default::default();
                    app.update();
                    let values = &app.world().resource::<ActionSetValues>().current;
                    assert_eq!(
                        values
                            .digital
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &v)| v.then_some(i as u16))
                            .collect::<Vec<_>>(),
                        [target],
                        "source={code:x}"
                    );
                    let mapping = &app.world().resource::<SteamInput>().actions_by_type;
                    let mut actual = mapping
                        .iter()
                        .flat_map(|(&target, sources)| {
                            sources.iter().map(move |source| (target, source.code()))
                        })
                        .collect::<Vec<_>>();
                    actual.sort();
                    assert_eq!(actual, expected);
                }
            }
        }
        assert_eq!(
            app.world()
                .resource::<ActiveInputModeTracker>()
                .controller_deadline,
            std::time::Duration::from_secs(100) + country_core::input::INPUT_MODE_COOLDOWN
        );
        sdk.connected.lock().unwrap().clear();
        app.update();
        let input = app.world().resource::<SteamInput>();
        assert_eq!(input.active_controller, 0);
        assert!(input.actions_by_type.is_empty());
    }

    #[test]
    fn real_steam_right_button_distinguishes_click_drag_and_exact_threshold() {
        use bevy_app::{App, Update};
        use country_core::{
            input::{ActiveInputModeTracker, action_set::ActionSetValues},
            resources::{gui::EscMenuState, time::GameTime},
            systems::{
                backend::input::translate_input_to_cursor_moved::InputCursorDelta,
                steam::{SteamClientResource, update_steam_inputs},
            },
        };
        for (distance, clicked) in [(0., true), (2.9999, true), (3., false), (3.0001, false)] {
            let sdk = Arc::new(Sdk::default());
            *sdk.connected.lock().unwrap() = vec![7];
            let mut app = App::new();
            app.add_event::<InputCursorDelta>()
                .insert_resource(SteamClientResource {
                    client: Some(sdk.clone()),
                })
                .insert_resource(input(sdk.clone()))
                .insert_resource(ActionSetValues::default())
                .insert_resource(ActiveInputModeTracker::new_default_mouse_keyboard(
                    std::time::Duration::ZERO,
                ))
                .insert_resource(GameTime::default())
                .insert_resource(bevy_time::Time::<()>::default())
                .insert_resource(EscMenuState::Closed)
                .add_systems(Update, update_steam_inputs);
            sdk.states.lock().unwrap().insert((7, 201), true);
            app.world_mut()
                .send_event(InputCursorDelta(glam::Vec2::splat(100.)));
            app.update();
            assert!(app.world().resource::<ActionSetValues>().current.digital[2]);
            app.world_mut().resource_mut::<ActionSetValues>().current = Default::default();
            app.world_mut()
                .send_event(InputCursorDelta(glam::Vec2::new(distance, 0.)));
            app.update();
            sdk.states.lock().unwrap().clear();
            app.world_mut().resource_mut::<ActionSetValues>().current = Default::default();
            app.update();
            let values = &app.world().resource::<ActionSetValues>().current;
            assert_eq!(values.digital[1], clicked, "distance={distance}");
            assert!(!values.digital[2]);
        }
    }
}
