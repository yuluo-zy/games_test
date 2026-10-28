//! B设置菜单：原总路由、页面、输入采集与配置请求，沿用当前MEI/渲染架构。
//! 原证及复验边界见toolbar-settings-router-review和video/keybindings专项报告。
//! needs_apply保留三位掩码和完整VideoMode，真实应用成功后才更新LastApplied。

use bevy_app::{App, Plugin};
use bevy_ecs::change_detection::Res;
use bevy_ecs::system::Local;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;

/// 原14024b37a..b3ca连续11个12字节KeyCode记录；首两个都是ArrowUp。
/// 旧反编译漏掉首条记录，曾被误读成十键和一个零值跳过项。
pub const KONAMI_SEQUENCE: [KeyCode; 11] = [
    KeyCode::ArrowUp,
    KeyCode::ArrowUp,
    KeyCode::ArrowDown,
    KeyCode::ArrowDown,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
    KeyCode::KeyB,
    KeyCode::KeyA,
    KeyCode::Enter,
];
pub const KONAMI_LENGTH: usize = KONAMI_SEQUENCE.len();
pub static SHOW_DEVELOPER_SETTINGS: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// `tiny_glade::gui::settings::SettingsMenuPlugin`。
#[derive(Debug, Default, Clone, Copy)]
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    /// `SettingsMenuPlugin::build` `0x140245570`（胶水-注册，判等）。
    fn build(&self, app: &mut App) {
        app.init_resource::<LastAppliedVideoSettings>()
            .init_resource::<country_core::resources::window::AvailableMonitors>()
            .add_event::<crate::app_config::ApplyVideoSettingsCmd>();
    }
}

/// 原Option<VideoSettings> owner：首次菜单更新从权威配置克隆，后续保留已应用版本。
#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct LastAppliedVideoSettings(pub Option<country_core::render::render_config::VideoSettings>);
impl LastAppliedVideoSettings {
    /// 原140245630返回位掩码：1全屏，2vsync，4监视器；不是bool。
    /// 窗口分辨率／图像质量不在该原函数的重建窗口判定内。
    pub fn needs_apply(&self, other: &country_core::render::render_config::VideoSettings) -> u8 {
        let previous = self
            .0
            .as_ref()
            .expect("原LastAppliedVideoSettings必须先populate");
        let old = &previous.display;
        let new = &other.display;
        let mut flags = u8::from(previous.vsync_enable != other.vsync_enable) << 1;
        if old.monitor != new.monitor {
            flags |= 4;
        }
        if old.fullscreen != new.fullscreen
            || (old.fullscreen
                && (old.allow_exclusive_fullscreen != new.allow_exclusive_fullscreen
                    || old.fullscreen_config != new.fullscreen_config))
        {
            flags |= 1;
        }
        flags
    }
    pub fn populate_empty_from(
        &mut self,
        other: &country_core::render::render_config::VideoSettings,
    ) {
        self.0.get_or_insert_with(|| other.clone());
    }
}

/// 原设置根、背景与页签。键位页面复用同一模板，保留各自采集状态机。
pub struct SettingsFrame {
    pub view: toolbar::mei::config::TemplateView,
    pub tab_delta: i32,
    pub image: bool,
}

/// 原245c99..245dea：根据当前场景和一天相位选择背景，比较次序不能合并成近似时段。
pub fn settings_background(glade: country_core::resources::session::GladeType, t: f32) -> &'static str {
    use country_core::resources::session::GladeType;
    if glade == GladeType::Olden {
        if (0.35 <= t && t <= 0.5) || (0.94 <= t && t <= 0.98) { "olden_sunset" }
        else if 0.5 <= t && t <= 0.84 { "olden_night" }
        else if 0.83 <= t && t <= 0.94 { "olden_sunrise" }
        else { "olden_day" }
    } else if (0.345 <= t && t <= 0.46) || (0.913 <= t && t <= 0.926) { "orange" }
    else if (0.46 <= t && t <= 0.48) || (0.84 <= t && t <= 0.913) { "pink" }
    else if 0.48 <= t && t <= 0.84 { "blue" }
    else { match glade { GladeType::Autumn => "orange", GladeType::Winter => "winter_day", _ => "green" } }
}

pub fn begin_settings_frame(
    mei: &mut toolbar::MeiContext,
    menu: &country_core::resources::gui::EscMenuState,
    glade: country_core::resources::session::GladeType,
    time_of_day: f32,
) -> Option<SettingsFrame> {
    use country_core::resources::gui::{EscMenuState, SettingsMenuState, VideoSettingsSubMenu};
    let EscMenuState::Settings(state) = menu else { return None };
    let image = *state == SettingsMenuState::Video(VideoSettingsSubMenu::Image);
    let background = mei.fullscreen("settings background");
    if menu.can_limit_frame_rate() {
        let name = format!("menu/settings/{}", settings_background(glade, time_of_day));
        let picture = background.image(&name, &name, 1000.).blur_extend();
        picture.content_mut::<toolbar::mei::widgets::ImageContent>().color = 0xffffffc8;
        picture.node_mut().local_opacity = 1.;
    } else { background.template("menu/settings_see_thru"); }
    let root = mei.safe_frame("settings", true).tab_switcher("tab_switcher");
    let delta = root.tab_delta();
    let view = root.template(if image { "menu/image-settings" } else { "menu/settings" });
    Some(SettingsFrame { view, tab_delta: delta, image })
}

fn category(state: country_core::resources::gui::SettingsMenuState) -> country_core::resources::gui::SettingsMenuState {
    use country_core::resources::gui::{SettingsMenuState as S, VideoSettingsSubMenu as V};
    match state { S::Video(_) => S::Video(V::Menu), other => other }
}

/// 原类别有条件插入Dev；导航按当前类别和完整delta循环，鼠标写Next不改变本帧权威页。
pub fn settings_categories(
    node: toolbar::mei::core::NodeContext,
    current: country_core::resources::gui::SettingsMenuState,
    delta: i32,
    next: &mut country_core::resources::gui::NextEscMenuState,
    developer: bool,
) {
    use country_core::resources::gui::{EscMenuState, SettingsMenuState as S, VideoSettingsSubMenu as V};
    use toolbar::mei::widgets::MenuButtonContent;
    let mut items = vec![
        (lang_lib::lookup("Menu.Settings.Audio").to_string(), S::Audio),
        (lang_lib::lookup("Menu.Settings.Video").to_string(), S::Video(V::Menu)),
        (lang_lib::lookup("Menu.Settings.Game").to_string(), S::Game),
        (lang_lib::lookup("Menu.Settings.Ui").to_string(), S::Ui),
        (lang_lib::lookup("Menu.Settings.Controls").to_string(), S::Controls),
    ];
    if developer { items.push(("Dev".into(), S::Developer)); }
    items.push(("文/A".into(), S::Language));
    for (name, target) in &items {
        let button = node.menu_button(&name.to_uppercase()).force_highlight(category(current) == *target);
        let content = button.content_mut::<MenuButtonContent>();
        content.text.set_max_width(280.).expect("原类别最大宽度");
        content.text.set_align(tg_text::Align::Center);
        button.node_mut().sound_click = 4;
        if button.node().used() { next.set(EscMenuState::Settings(*target)); }
    }
    if delta != 0 {
        let index = items.iter().position(|(_, target)| *target == category(current)).unwrap_or(0);
        let selected = (index as i64 + i64::from(delta)).rem_euclid(items.len() as i64) as usize;
        next.set(EscMenuState::Settings(items[selected].1));
    }
}

/// 原通用页面外壳：页面回调返回本帧应用掩码，只有实际使用Apply按钮才发送。
pub fn settings_page(
    frame: &SettingsFrame,
    state: country_core::resources::gui::SettingsMenuState,
    next: &mut country_core::resources::gui::NextEscMenuState,
    page: impl FnOnce(toolbar::mei::core::NodeContext, &mut toolbar::mei::widgets::tweaker::HintTracker, &mut country_core::resources::gui::NextEscMenuState) -> u8,
) -> Option<u8> {
    use country_core::resources::gui::{EscMenuState, SettingsMenuState as S, VideoSettingsSubMenu as V};
    use toolbar::mei::widgets::{LabelContent, FrameContent};
    if !frame.image {
        let title = frame.view.slot("header").label(lang_lib::lookup("Menu.Settings").to_uppercase());
        let text = &mut title.content_mut::<LabelContent>().text;
        text.set_metrics(90.).expect("原设置标题字号");
        text.set_weight(tg_text::Weight(800));
        settings_categories(frame.view.slot("categories"), state, frame.tab_delta, next,
            SHOW_DEVELOPER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed));
        frame.view.context.content_mut::<toolbar::mei::widgets::TabSwitcherContent>().delta = 0;
    }
    let mut hints = toolbar::mei::widgets::tweaker::HintTracker::default();
    let body = frame.view.slot("settings");
    body.node_mut().default_focus();
    let flags = page(body, &mut hints, next);
    let applied = flags != 0 && body.menu_button(&lang_lib::lookup("Menu.Settings.Video.Apply")).node().used();
    let back = frame.view.slot("back").menu_button(&lang_lib::lookup("Menu.Back")).menu_hotkey(18);
    back.node_mut().sound_click = 9;
    if back.node().used() {
        next.set(if matches!(state, S::Video(V::Image | V::Display)) {
            EscMenuState::Settings(S::Video(V::Menu))
        } else { EscMenuState::Main });
    }
    if state != S::Language {
        use toolbar::mei::core::calculate_hash;
        let info = frame.view.slot("settings-and-info")
            .template_with_id("menu/settings-info", calculate_hash(&("hint_nodes", ())));
        if !hints.current.is_empty() {
            let area = info.slot("hint").child("frame").template("menu/settings-info-frame").slot("hint");
            if frame.image {
                area.get_content_mut_impl::<FrameContent>(false).shapes =
                    tg_text::ShapeStream::new().fill_color(0x808080e0).corner_radius(10.);
            }
            let label = area.label(hints.current);
            let text = &mut label.content_mut::<LabelContent>().text;
            text.set_metrics(32.).expect("原设置说明字号");
            text.set_max_width(640.).expect("原设置说明最大宽度");
        }
    }
    applied.then_some(flags)
}

/// `do_gui_escape_menu_settings` `0x140245790` —— 正在接通权威资源与各页。
#[derive(bevy_ecs::system::SystemParam)]
pub struct SettingsGraphics<'w> {
    device: Option<Res<'w, country_core::render::device::GraphicsDeviceResource>>,
    monitors: Res<'w, country_core::resources::window::AvailableMonitors>,
    devices: Option<Res<'w, country_core::render::AvailableGraphicsDevices>>,
    steam: Option<Res<'w, country_core::systems::steam::SteamClientResource>>,
    input: Option<Res<'w, country_core::resources::steam::SteamInput>>,
}

#[allow(clippy::too_many_arguments)]
pub fn do_gui_escape_menu_settings(
    graphics: SettingsGraphics,
    menu: Res<country_core::resources::gui::EscMenuState>,
    (mut config, mut profile, mut frame_config, mut adaptive): (
        bevy_ecs::prelude::ResMut<country_core::game_config::GameConfig>,
        bevy_ecs::prelude::ResMut<country_core::game_profile::GameProfile>,
        bevy_ecs::prelude::ResMut<country_core::render::render_config::FrameRenderConfig>,
        bevy_ecs::prelude::ResMut<country_core::render::adaptive_render_state::AdaptiveRenderState>,
    ),
    mut last: bevy_ecs::prelude::ResMut<LastAppliedVideoSettings>,
    (mut apply, mut save, mut tutorial, mut cursor): (
        bevy_ecs::event::EventWriter<crate::app_config::ApplyVideoSettingsCmd>,
        bevy_ecs::event::EventWriter<crate::app_config::WriteSettingsFileCmd>,
        bevy_ecs::event::EventWriter<country_core::resources::onboarding::TutorialOp>,
        bevy_ecs::event::EventWriter<country_core::resources::mouse_capture::OsCursorVisibilityCmd>,
    ),
    mut mei: bevy_ecs::prelude::ResMut<toolbar::MeiContext>,
    mut next: bevy_ecs::prelude::ResMut<country_core::resources::gui::NextEscMenuState>,
    mut game_state: Local<u8>,
    tod: Res<country_core::resources::time_of_day::TimeOfDay>,
    session: Option<Res<country_core::resources::session::ActiveSession>>,
    mut key_state: Local<crate::gui::gui_controls::KeyBindingsMenuState>,
    mut keys: bevy_ecs::event::EventReader<bevy_input::keyboard::KeyboardInput>,
    mut mouse: bevy_ecs::event::EventReader<bevy_input::mouse::MouseButtonInput>,
    mut save_profile: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>,
) {
    use country_core::resources::gui::{EscMenuState, SettingsMenuState as S};
    if menu.is_closed() {
        key_state.reset();
        return;
    }
    last.populate_empty_from(&config.video);
    let EscMenuState::Settings(state) = *menu else { return };
    let session = session.as_deref().expect("原设置页面依赖真实ActiveSession");
    let frame = begin_settings_frame(&mut mei, &menu, session.meta.glade, tod.time_of_day)
        .expect("已确认设置状态");
    if state == S::KeyBindings {
        // 原只在键位页面消费事件，顺序和最后事件的语义交给同一采集状态机。
        let keys: Vec<_> = if key_state.wants_keyboard_events() { keys.read().cloned().collect() } else { Vec::new() };
        let mouse: Vec<_> = if key_state.wants_mouse_events() { mouse.read().copied().collect() } else { Vec::new() };
        let outcome = crate::gui::gui_controls::ui_settings_keybindings(
            &frame.view, &mut key_state, &mut profile, &keys, &mouse,
            frame.tab_delta, config.dev.demo);
        if outcome.profile_changed { save_profile.write(country_core::game_profile::SaveGameProfileCmd); }
        if outcome.back_to_controls { next.set(EscMenuState::Settings(S::Controls)); }
        return;
    }
    if state != S::Game { *game_state = 2; }
    let mut changed = false;
    let applied = settings_page(&frame, state, &mut next, |body, hints, next| {
        match state {
            S::Audio => ui_settings_audio(&mut changed, body, hints, &mut config),
            S::Game => ui_settings_game(&mut game_state, &mut changed, body, hints, &mut config, &mut tutorial),
            S::Ui => ui_settings_ui(&mut changed, body, hints, &mut config, &mut cursor),
            S::Developer => ui_settings_developer(&mut changed, body, hints, &mut config),
            S::Controls => ui_settings_controls(&mut changed, body, hints, &mut config, next,
                graphics.steam.as_deref(), graphics.input.as_deref()),
            S::Language => {
                let rows = body.child("language options").template("menu/language").slot("rows");
                crate::gui::language_menu::do_gui_language_menu(rows, &mut changed, &mut config);
            }
            S::Video(submenu) => {
                let devices = graphics.devices.as_deref().expect("原视频页依赖启动设备快照");
                let device = graphics.device.as_deref().expect("原视频页依赖真实图形设备");
                ui_settings_video(&mut changed, body, hints, &mut config, submenu, next,
                    &graphics.monitors, VideoSettingsRuntime {
                        devices: &devices.devices, device_init_states: &devices.init_states,
                        device: device.device().inner(), frame: &mut frame_config, adaptive: &mut adaptive,
                    });
            }
            S::KeyBindings => unreachable!("键位页面已由独立采集状态机路由"),
        }
        last.needs_apply(&config.video)
    });
    if let Some(flags) = applied { apply.write(crate::app_config::ApplyVideoSettingsCmd(flags)); }
    if changed { save.write(crate::app_config::WriteSettingsFileCmd); }
}

/// 原游戏设置页的Local字节：0/1确认，2正常，4执行完成。
pub fn ui_settings_game(
    state: &mut u8,
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
    tutorial: &mut bevy_ecs::event::EventWriter<country_core::resources::onboarding::TutorialOp>,
) {
    use country_core::{
        game_config::ScreenshotFormat as Format, resources::onboarding::TutorialOp,
    };
    use toolbar::mei::widgets::{
        LabelContent, MenuButtonContent,
        tweaker::{TweakableBool, TweakableEnum, TweakableFloatWithStep},
    };
    if *state == 2 {
        *changed |= node.tweaker(
            &lang_lib::lookup("Menu.Settings.Game.GladeBorderOutline"),
            hints.text(""),
            260.,
            TweakableBool(&mut config.game.hide_glade_border_outline),
        );
        *changed |= node.tweaker(
            &lang_lib::lookup("TimeOfDay.Speed"),
            hints.text(""),
            260.,
            TweakableFloatWithStep {
                value: &mut config.game.tod_speed,
                step: 0.1,
                min: 0.1,
                max: 5.,
            },
        );
        *changed |= node.tweaker(
            &lang_lib::lookup("Menu.Settings.Game.ScreenshotFormat"),
            hints.text(""),
            260.,
            TweakableEnum {
                value: &mut config.game.screenshot_format,
                variants: &[Format::Jpeg, Format::Png],
                formatter: |f| f.to_string(),
            },
        );
        let label = node.label(lang_lib::lookup("Menu.Settings.Game.ControlHints").to_string());
        label
            .content_mut::<LabelContent>()
            .text
            .set_metrics(40.)
            .expect("原教学标签字号");
        label
            .content_mut::<LabelContent>()
            .text
            .set_weight(tg_text::Weight(700));
        if label.node().hover() {
            hints
                .text(&lang_lib::lookup("Menu.Settings.Game.ControlHintsHint"))
                .confirm();
        }
        for (key, hint, target) in [
            (
                "Menu.Settings.Game.CompleteHints",
                "Menu.Settings.Game.CompleteHintsHint",
                0,
            ),
            (
                "Menu.Settings.Game.ResetTutorial",
                "Menu.Settings.Game.ResetHintsHint",
                1,
            ),
        ] {
            let button = node.menu_button(&lang_lib::lookup(key));
            button
                .content_mut::<MenuButtonContent>()
                .text
                .set_metrics(32.)
                .expect("原教学按钮字号");
            if button.node().hover() {
                hints.text(&lang_lib::lookup(hint)).confirm();
            }
            if button.node().used() {
                *state = target;
            }
        }
    } else if *state == 4 {
        node.label(lang_lib::lookup("Menu.Settings.Game.HintsResetDone").to_string());
    } else {
        node.label(lang_lib::lookup("AreYouSure").to_string());
        let button = node.menu_button(&lang_lib::lookup("AreYouSure.ImSure"));
        button
            .content_mut::<MenuButtonContent>()
            .text
            .set_metrics(40.)
            .expect("原确认字号");
        if button.node().used() {
            tutorial.write(if *state & 1 == 0 {
                TutorialOp::DisableHints
            } else {
                TutorialOp::ResetHints
            });
            *state = 4;
            *changed = true;
        }
        if node
            .menu_button(&lang_lib::lookup("AreYouSure.No"))
            .node()
            .used()
        {
            *state = 2;
        }
    }
}

/// 原1402475d0四行：step0.1、min0、max2，hint空、默认值列宽260。
pub fn ui_settings_audio(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
) {
    use toolbar::mei::widgets::tweaker::TweakableFloatWithStep;
    for (key, value) in [
        (
            "Menu.Settings.Audio.MasterVolume",
            &mut config.audio.master_volume,
        ),
        (
            "Menu.Settings.Audio.MusicVolume",
            &mut config.audio.music_volume,
        ),
        (
            "Menu.Settings.Audio.EffectsVolume",
            &mut config.audio.sfx_volume,
        ),
        (
            "Menu.Settings.Audio.AmbienceVolume",
            &mut config.audio.ambience_volume,
        ),
    ] {
        *changed |= node.tweaker(
            &lang_lib::lookup(key),
            hints.text(""),
            260.,
            TweakableFloatWithStep {
                value,
                step: 0.1,
                min: 0.,
                max: 2.,
            },
        );
    }
}

/// 原视频页使用启动时枚举的设备以及实际设备、帧配置和显存恢复状态。
/// 设备初始化状态必须由创建结果提供；不能把未知兼容性当作创建失败。
pub struct VideoSettingsRuntime<'a> {
    pub devices: &'a [rhapsody::device::AvailableGraphicsDevice],
    pub device_init_states: &'a [(
        uuid::Uuid,
        rhapsody::device::AvailableGraphicsDeviceInitState,
    )],
    pub device: &'a rhapsody::device::Device,
    pub frame: &'a mut country_core::render::render_config::FrameRenderConfig,
    pub adaptive: &'a mut country_core::render::adaptive_render_state::AdaptiveRenderState,
}

/// 原140247870，保留菜单、画质与显示三条分支，更新现有权威配置和帧状态。
pub fn ui_settings_video(
    changed: &mut bool,
    mut node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
    state: country_core::resources::gui::VideoSettingsSubMenu,
    next: &mut country_core::resources::gui::NextEscMenuState,
    monitors: &country_core::resources::window::AvailableMonitors,
    runtime: VideoSettingsRuntime<'_>,
) {
    use country_core::resources::gui::{EscMenuState, SettingsMenuState, VideoSettingsSubMenu};
    match state {
        VideoSettingsSubMenu::Menu => {
            for (key, target) in [
                ("Menu.Settings.Video.Image", VideoSettingsSubMenu::Image),
                ("Menu.Settings.Video.Display", VideoSettingsSubMenu::Display),
            ] {
                if node
                    .settings_button(&format!("> {}", lang_lib::lookup(key)))
                    .node()
                    .used()
                {
                    next.set(EscMenuState::Settings(SettingsMenuState::Video(target)));
                }
            }
        }
        VideoSettingsSubMenu::Image => {
            let requested = config.video.world_resolution_scale.unwrap_or(1.);
            let estimate = crate::vram::calculate_resolution_scale_mem_estimate_text(
                runtime.device,
                runtime.frame.extent_0,
                requested,
            );
            let maximum = if runtime.adaptive.enabled {
                crate::vram::calculate_max_resolution_scale(runtime.device, runtime.frame.extent_0)
            } else {
                256.
            };
            ui_settings_video_image(
                changed,
                &mut node,
                hints,
                &mut config.video,
                runtime.frame,
                runtime.adaptive,
                maximum,
                &estimate,
            );
        }
        VideoSettingsSubMenu::Display => {
            ui_settings_video_display(
                changed,
                node,
                hints,
                &mut config.video,
                monitors,
                runtime.devices,
                runtime.device_init_states,
                runtime.device.device_uuid(),
            );
        }
    }
}

#[derive(Clone, Copy)]
struct QualitySettings {
    preset: country_core::render::render_config::ImageQualityPreset,
    scale: f32,
    upscaling_preset: country_core::render::render_config::UpscalingPreset,
    upscaling_mode: country_core::render::render_config::UpscalingMode,
    dof: bool,
}

/// 原140247d3b..d8a构造的四条记录；Custom只用于未匹配时的选择器。
const QUALITY_SETTINGS: [QualitySettings; 4] = {
    use country_core::render::render_config::{
        ImageQualityPreset as Q, UpscalingMode as M, UpscalingPreset as U,
    };
    [
        QualitySettings {
            preset: Q::Performance,
            scale: 1.,
            upscaling_preset: U::Performance,
            upscaling_mode: M::Tgsr,
            dof: false,
        },
        QualitySettings {
            preset: Q::Balanced,
            scale: 1.,
            upscaling_preset: U::Quality,
            upscaling_mode: M::Tgsr,
            dof: true,
        },
        QualitySettings {
            preset: Q::Quality,
            scale: 1.,
            upscaling_preset: U::Quality,
            upscaling_mode: M::Off,
            dof: true,
        },
        QualitySettings {
            preset: Q::UltraQuality,
            scale: 1.5,
            upscaling_preset: U::UltraQuality,
            upscaling_mode: M::Tgsr,
            dof: true,
        },
    ]
};

fn matching_quality(video: &country_core::render::render_config::VideoSettings) -> Option<usize> {
    QUALITY_SETTINGS.iter().position(|q| {
        q.scale == video.world_resolution_scale.unwrap_or(1.)
            && q.upscaling_mode == video.upscaling_mode
            && q.upscaling_preset == video.upscaling_preset
            && q.dof == video.dof_enable
    })
}

fn apply_quality(
    video: &mut country_core::render::render_config::VideoSettings,
    frame: &mut country_core::render::render_config::FrameRenderConfig,
    adaptive: &mut country_core::render::adaptive_render_state::AdaptiveRenderState,
    quality: QualitySettings,
    maximum: f32,
) -> bool {
    // 原247ff2..2483c4：预算不足时拒绝整条预设，不部分应用DoF/TGSR。
    if !(quality.scale <= maximum) {
        return false;
    }
    video.upscaling_mode = quality.upscaling_mode;
    video.upscaling_preset = quality.upscaling_preset;
    video.dof_enable = quality.dof;
    frame.world_resolution_scale = quality.scale;
    let requested = if quality.scale == 1. {
        None
    } else {
        Some(quality.scale)
    };
    if video.world_resolution_scale != requested {
        video.world_resolution_scale = requested;
        adaptive.reset_after_manual_scale_change();
    }
    true
}

fn video_heading(node: toolbar::mei::core::NodeContext, key: &str) {
    let heading = node.label(lang_lib::lookup(key).to_string());
    let text = &mut heading
        .content_mut::<toolbar::mei::widgets::LabelContent>()
        .text;
    text.set_metrics(40.).expect("原视频设置标题字号");
    text.set_weight(tg_text::Weight(700));
}

fn ui_settings_video_image(
    changed: &mut bool,
    node: &mut toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    video: &mut country_core::render::render_config::VideoSettings,
    frame: &mut country_core::render::render_config::FrameRenderConfig,
    adaptive: &mut country_core::render::adaptive_render_state::AdaptiveRenderState,
    maximum: f32,
    estimate: &str,
) {
    use country_core::render::render_config::{
        ImageQualityPreset as Q, UpscalingMode as M, UpscalingPreset as U,
    };
    use toolbar::mei::widgets::tweaker::{
        TweakableBool, TweakableEnum, TweakableFloatWithStep, TweakableList,
    };
    video_heading(*node, "Menu.Settings.Video.ImageSettings");
    let presets = [Q::Performance, Q::Balanced, Q::Quality, Q::UltraQuality];
    // 原静态表1429a0fc0是[Quality,Custom,Quality]，允许自定义值向两侧返回Quality。
    let custom = [Q::Quality, Q::Custom, Q::Quality];
    let (values, mut index) = match matching_quality(video) {
        Some(index) => (&presets[..], index),
        None => (&custom[..], 1),
    };
    if node.tweaker(
        &lang_lib::lookup("Menu.Settings.Video.QualityPreset"),
        hints.text(""),
        260.,
        TweakableList {
            index: &mut index,
            values,
            formatter: |q| lang_lib::lookup(q.lang_key()).to_string(),
        },
    ) && values[index] != Q::Custom
    {
        let quality = *QUALITY_SETTINGS
            .iter()
            .find(|q| q.preset == values[index])
            .expect("原画质预设表");
        *changed |= apply_quality(video, frame, adaptive, quality, maximum);
    }
    let upscaling_hint = format!(
        "{}{}",
        if video.upscaling_mode == M::Tgsr {
            "Tiny Glade Super Resolution\n\n"
        } else {
            ""
        },
        lang_lib::lookup("Menu.Settings.Video.UpscalingHint")
    );
    *changed |= node.tweaker(
        &format!("    {}", lang_lib::lookup("Menu.Settings.Video.Upscaling")),
        hints.text(&upscaling_hint),
        260.,
        TweakableEnum {
            value: &mut video.upscaling_mode,
            variants: &[M::Off, M::Tgsr],
            formatter: |mode| match mode.display() {
                Ok(literal) => literal.to_string(),
                Err(key) => lang_lib::lookup(key).to_string(),
            },
        },
    );
    if video.upscaling_mode == M::Tgsr {
        *changed |= node.tweaker(
            &format!(
                "    {}",
                lang_lib::lookup("Menu.Settings.Video.UpscalingPreset")
            ),
            hints.text(""),
            260.,
            TweakableEnum {
                value: &mut video.upscaling_preset,
                variants: &U::ALL,
                formatter: |p| lang_lib::lookup(p.lang_key()).to_string(),
            },
        );
    }
    *changed |= node.tweaker(
        &format!("    {}", lang_lib::lookup("Menu.Settings.Video.DoF")),
        hints.text(""),
        260.,
        TweakableBool(&mut video.dof_enable),
    );
    let mut requested = video.world_resolution_scale.unwrap_or(1.);
    let mut scale_hint = format!(
        "{}\n\n{estimate}",
        lang_lib::lookup("Menu.Settings.Video.RenderScaleHint")
    );
    if requested > frame.world_resolution_scale {
        let reason = match adaptive.native_tag() {
            1 => Some("(graphics driver hint)"),
            2 => Some("(out-of-memory error)"),
            _ => None,
        };
        if let Some(reason) = reason {
            scale_hint = format!(
                "{scale_hint}\n\n⚠️ Reduced to {:.2} {reason}",
                frame.world_resolution_scale
            );
        }
    }
    scale_hint
        .push_str("\n\n(Run the game with --ignore-vram-budget to bypass limits at your own risk)");
    let maximum_widget = if adaptive.enabled {
        maximum.min(1.5)
    } else {
        maximum
    };
    if node.tweaker(
        &format!(
            "    {}",
            lang_lib::lookup("Menu.Settings.Video.RenderScale")
        ),
        hints.text(&scale_hint),
        260.,
        TweakableFloatWithStep {
            value: &mut requested,
            step: 0.1,
            min: maximum.min(0.1),
            max: maximum_widget,
        },
    ) {
        video.world_resolution_scale = Some(requested);
        frame.world_resolution_scale = requested.min(maximum);
        adaptive.reset_after_manual_scale_change();
        *changed = true;
    }
    node.space(0., 40.);
    *changed |= node.tweaker(
        &format!(
            "{} ({})",
            lang_lib::lookup("Menu.Settings.Video.DoF"),
            lang_lib::lookup("PhotoMode")
        ),
        hints.text(""),
        260.,
        TweakableBool(&mut video.photomode_dof_enable),
    );
    for (key, value, step, min, max, hint) in [
        (
            "Menu.Settings.Video.Brightness",
            &mut video.brightness_shift,
            0.2,
            -2.,
            2.,
            "",
        ),
        (
            "Menu.Settings.Video.Gamma",
            &mut video.gamma_shift,
            0.05,
            -0.5,
            0.5,
            "",
        ),
        (
            "Menu.Settings.Video.Sharpen",
            &mut video.sharpen,
            0.1,
            0.,
            1.,
            "Menu.Settings.Video.SharpenHint",
        ),
    ] {
        *changed |= node.tweaker(
            &lang_lib::lookup(key),
            hints.text(&lang_lib::lookup(hint)),
            260.,
            TweakableFloatWithStep {
                value,
                step,
                min,
                max,
            },
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum DisplayMode {
    Windowed = 0,
    Borderless = 1,
    Fullscreen = 2,
}

fn display_mode_name(mode: &DisplayMode) -> String {
    lang_lib::lookup(match mode {
        DisplayMode::Windowed => "Menu.Settings.Video.Windowed",
        DisplayMode::Borderless => "Menu.Settings.Video.Borderless",
        DisplayMode::Fullscreen => "Menu.Settings.Video.Fullscreen",
    })
    .to_string()
}

fn device_menu_name(device: &rhapsody::device::AvailableGraphicsDevice) -> String {
    if device.name.len() < 26 {
        device.name.clone()
    } else {
        format!("{}...", device.name[..25].trim())
    }
}

const FPS_CAPS: [Option<u32>; 9] = [
    Some(15),
    Some(20),
    Some(30),
    Some(60),
    Some(72),
    Some(80),
    Some(90),
    Some(120),
    None,
];
fn fps_cap_index(value: f32) -> usize {
    FPS_CAPS
        .iter()
        .position(|&cap| cap == Some(value as u32))
        .unwrap_or(8)
}

fn ui_settings_video_display(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    video: &mut country_core::render::render_config::VideoSettings,
    monitors: &country_core::resources::window::AvailableMonitors,
    devices: &[rhapsody::device::AvailableGraphicsDevice],
    device_init_states: &[(
        uuid::Uuid,
        rhapsody::device::AvailableGraphicsDeviceInitState,
    )],
    active_uuid: [u8; 16],
) {
    use country_core::render::render_config::FullscreenConfig;
    use toolbar::mei::widgets::{
        LabelContent,
        tweaker::{TweakableBool, TweakableEnum, TweakableList},
    };
    video_heading(node, "Menu.Settings.Video.Display");
    if let Some(mut index) = video
        .graphics_device_uuid
        .and_then(|id| devices.iter().position(|d| id.as_bytes() == &d.device_uuid))
    {
        let names: Vec<_> = devices.iter().map(device_menu_name).collect();
        if node.tweaker(
            &lang_lib::lookup("Menu.Settings.Video.Device"),
            hints.text(&lang_lib::lookup("Menu.Settings.Video.DeviceHint")),
            260.,
            TweakableList {
                index: &mut index,
                values: &names,
                formatter: Clone::clone,
            },
        ) {
            video.graphics_device_uuid = Some(uuid::Uuid::from_bytes(devices[index].device_uuid));
            *changed = true;
        }
        if video
            .graphics_device_uuid
            .as_ref()
            .map(uuid::Uuid::as_bytes)
            != Some(&active_uuid)
        {
            let failed = device_init_states
                .iter()
                .find(|(id, _)| id.as_bytes() == &devices[index].device_uuid)
                .is_some_and(|(_, state)| state.is_error());
            let label = node.label(
                lang_lib::lookup(if failed {
                    "Menu.Settings.Video.DeviceError"
                } else {
                    "Menu.Settings.Video.RestartToChangeDevice"
                })
                .to_string(),
            );
            let content = label.content_mut::<LabelContent>();
            content.text.set_metrics(32.).expect("原设备重启提示字号");
            content.color = 0xffa080ff;
        }
    }
    let mut mode = if !video.display.fullscreen {
        DisplayMode::Windowed
    } else if video.display.allow_exclusive_fullscreen {
        DisplayMode::Fullscreen
    } else {
        DisplayMode::Borderless
    };
    if node.tweaker(
        &lang_lib::lookup("Menu.Settings.Video.DisplayMode"),
        hints.text(&lang_lib::lookup("Menu.Settings.Video.DisplayModeHint")),
        260.,
        TweakableEnum {
            value: &mut mode,
            variants: &[
                DisplayMode::Windowed,
                DisplayMode::Borderless,
                DisplayMode::Fullscreen,
            ],
            formatter: display_mode_name,
        },
    ) {
        *changed = true;
        video.display.fullscreen = mode != DisplayMode::Windowed;
        if mode != DisplayMode::Windowed {
            video.display.allow_exclusive_fullscreen = mode == DisplayMode::Fullscreen;
        }
    }
    if mode != DisplayMode::Windowed {
        let mut names: Vec<_> = monitors
            .monitors
            .iter()
            .filter_map(|m| m.monitor.name.as_ref())
            .collect();
        names.sort();
        if let Some(mut index) = video
            .display
            .monitor
            .as_ref()
            .and_then(|m| m.name.as_ref())
            .and_then(|name| names.iter().position(|n| *n == name))
        {
            if node.tweaker(
                &lang_lib::lookup("Menu.Settings.Video.Monitor"),
                hints.text(""),
                260.,
                TweakableList {
                    index: &mut index,
                    values: &names,
                    formatter: |name| (*name).clone(),
                },
            ) {
                let selected = monitors
                    .monitors
                    .iter()
                    .find(|m| m.monitor.name.as_ref() == Some(names[index]))
                    .expect("原监视器名称来自可用列表");
                video.display.monitor = Some(selected.monitor.clone());
                *changed = true;
            }
        }
        if mode == DisplayMode::Fullscreen {
            let selected_name = video.display.monitor.as_ref().and_then(|m| m.name.as_ref());
            if let Some(monitor) = monitors
                .monitors
                .iter()
                .find(|m| m.monitor.name.as_ref() == selected_name)
            {
                let modes = &monitor.video_modes;
                let mut resolutions: Vec<_> = modes.iter().map(|m| m.resolution).collect();
                // 原版仅相邻去重后反转，顺序由窗口层的实际枚举负责。
                resolutions.dedup();
                resolutions.reverse();
                if !resolutions.is_empty() {
                    let existing = video
                        .display
                        .fullscreen_config
                        .resolution()
                        .map(|(w, h)| [w, h]);
                    let found = existing.and_then(|r| resolutions.iter().position(|x| *x == r));
                    let mut index = found.unwrap_or(resolutions.len() - 1);
                    let update = if found.is_some() {
                        node.tweaker(
                            &lang_lib::lookup("Menu.Settings.Video.Resolution"),
                            hints.text(""),
                            260.,
                            TweakableList {
                                index: &mut index,
                                values: &resolutions,
                                formatter: |r| format!("{} x {}", r[0], r[1]),
                            },
                        )
                    } else {
                        true
                    };
                    if update {
                        if let Some(mode) =
                            modes.iter().find(|m| m.resolution == resolutions[index])
                        {
                            video.display.fullscreen_config = FullscreenConfig::ClosestMatch(*mode);
                            *changed = true;
                        }
                    }
                    if let FullscreenConfig::ClosestMatch(current) = video.display.fullscreen_config
                    {
                        let mut rates: Vec<_> = modes
                            .iter()
                            .filter(|m| m.resolution == current.resolution)
                            .map(|m| m.refresh_rate_millihertz)
                            .collect();
                        rates.sort();
                        rates.dedup();
                        let found = rates
                            .iter()
                            .position(|r| *r == current.refresh_rate_millihertz);
                        let mut index = found.unwrap_or_else(|| {
                            rates
                                .len()
                                .checked_sub(1)
                                .expect("原当前分辨率必须有刷新率")
                        });
                        let update = if found.is_some() {
                            node.tweaker(
                                &lang_lib::lookup("Menu.Settings.Video.RefreshRate"),
                                hints.text(""),
                                260.,
                                TweakableList {
                                    index: &mut index,
                                    values: &rates,
                                    formatter: |rate| format!("{:.2} Hz", *rate as f32 / 1000.),
                                },
                            )
                        } else {
                            true
                        };
                        if update {
                            if let Some(mode) = modes.iter().find(|m| {
                                m.resolution == current.resolution
                                    && m.refresh_rate_millihertz == rates[index]
                            }) {
                                video.display.fullscreen_config =
                                    FullscreenConfig::ClosestMatch(*mode);
                                *changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
    *changed |= node.tweaker(
        &lang_lib::lookup("Menu.Settings.Video.VSync"),
        hints.text(&lang_lib::lookup("Menu.Settings.Video.VSyncHint")),
        260.,
        TweakableBool(&mut video.vsync_enable),
    );
    let mut index = fps_cap_index(video.fps_limit);
    if node.tweaker(
        &lang_lib::lookup("Menu.Settings.Video.FrameRateLimit"),
        hints.text(&lang_lib::lookup("Menu.Settings.Video.FrameRateLimitHint")),
        260.,
        TweakableList {
            index: &mut index,
            values: &FPS_CAPS,
            formatter: |cap| {
                cap.map_or_else(
                    || lang_lib::lookup("Menu.Settings.Video.FpsUnlimited").to_string(),
                    |n| n.to_string(),
                )
            },
        },
    ) {
        video.fps_limit = FPS_CAPS[index].map_or(0., |n| n as f32);
        *changed = true;
    }
}

#[cfg(test)]
mod video_settings_tests {
    use super::*;
    use country_core::render::{adaptive_render_state::AdaptiveRenderState, render_config::*};
    use toolbar::mei::{
        core::MeiContext,
        widgets::{
            LabelContent,
            tweaker::{HintTracker, TweakerContent},
        },
    };

    fn adaptive_at(phase: u8) -> AdaptiveRenderState {
        let mut state = AdaptiveRenderState::at_startup(false);
        let now = std::time::Instant::now();
        if phase != 0 {
            state.update(now, 1., 1., 0.5);
        }
        match phase {
            0 | 1 => {}
            2 => state.mark_out_of_memory(),
            3 => {
                state.update(now + std::time::Duration::from_millis(250), 0.5, 1., 2.);
            }
            _ => unreachable!(),
        }
        assert_eq!(state.native_tag(), phase as u64);
        state
    }

    #[test]
    fn video_quality_and_fps_match_original_bounded_instructions() {
        let rows: Vec<(u8, u32, u8, u8, u8, i8)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-video-quality-match.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 320);
        for (i, (tag, scale, mode, preset, dof, expected)) in rows.into_iter().enumerate() {
            let mut video = VideoSettings::default();
            video.world_resolution_scale = (tag != 0).then(|| f32::from_bits(scale));
            video.upscaling_mode = if mode == 0 {
                UpscalingMode::Off
            } else {
                UpscalingMode::Tgsr
            };
            video.upscaling_preset = UpscalingPreset::ALL[preset as usize];
            video.dof_enable = dof != 0;
            assert_eq!(
                matching_quality(&video).map(|x| x as i8).unwrap_or(-1),
                expected,
                "原预设识别{i}"
            );
        }
        type ApplyRow = (usize, u32, u8, u32, u8, u8, u8, u8, u8, u32, u32, u8, u8);
        let rows: Vec<ApplyRow> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-video-quality-apply.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 1024);
        for (
            i,
            (
                quality,
                maximum,
                tag,
                scale,
                phase,
                mode,
                preset,
                dof,
                newtag,
                newscale,
                newframe,
                newphase,
                changed,
            ),
        ) in rows.into_iter().enumerate()
        {
            let mut video = VideoSettings::default();
            video.upscaling_mode = UpscalingMode::Off;
            video.upscaling_preset = UpscalingPreset::UltraPerformance;
            video.dof_enable = false;
            video.world_resolution_scale = (tag != 0).then(|| f32::from_bits(scale));
            let mut frame = FrameRenderConfig::at_startup(
                country_core::resources::window::WindowSize::new(1920, 1080),
                &video,
            );
            frame.world_resolution_scale = 0.7;
            let mut adaptive = adaptive_at(phase);
            assert_eq!(
                apply_quality(
                    &mut video,
                    &mut frame,
                    &mut adaptive,
                    QUALITY_SETTINGS[quality],
                    f32::from_bits(maximum)
                ),
                changed != 0,
                "原预设应用{i}"
            );
            assert_eq!(
                (
                    video.upscaling_mode as u8,
                    video.upscaling_preset as u8,
                    u8::from(video.dof_enable)
                ),
                (mode, preset, dof),
                "原预设字段{i}"
            );
            assert_eq!(
                video.world_resolution_scale.is_some(),
                newtag != 0,
                "原Option判别{i}"
            );
            if newtag != 0 {
                assert_eq!(
                    video.world_resolution_scale.unwrap().to_bits(),
                    newscale,
                    "原比例{i}"
                );
            }
            assert_eq!(
                frame.world_resolution_scale.to_bits(),
                newframe,
                "原帧比例{i}"
            );
            assert_eq!(adaptive.native_tag(), newphase as u64, "原恢复状态{i}");
        }
        let rows: Vec<(u32, usize)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-video-fps.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 27);
        for (bits, expected) in rows {
            assert_eq!(
                fps_cap_index(f32::from_bits(bits)),
                expected,
                "原FPS {bits:08x}"
            );
        }
    }

    fn fonts() -> MeiContext {
        let root =
            std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").expect("原资产路径"));
        let mut mei = MeiContext::default();
        mei.load_font_table(&root.join("assets/fonts/table.ron"))
            .unwrap();
        mei
    }

    fn delta(mei: &mut MeiContext, label: &str, delta: i32) {
        for node in mei.nodes.values_mut() {
            let any: &mut dyn std::any::Any = node.content.as_mut();
            if let Some(content) = any.downcast_mut::<TweakerContent>() {
                content.delta = 0;
            }
            node.state_bits &= !0x30;
        }
        let row = *mei
            .nodes
            .iter()
            .find(|(_, node)| {
                node.content.type_name().ends_with("TweakerContent")
                    && node.children.iter().any(|id| {
                        let any: &dyn std::any::Any = mei.nodes[id].content.as_ref();
                        any.downcast_ref::<LabelContent>()
                            .is_some_and(|label_content| {
                                label_content.pending_text.as_deref() == Some(label)
                            })
                    })
            })
            .expect("实际Tweaker行")
            .0;
        toolbar::mei::core::NodeContext::new(mei, row)
            .content_mut::<TweakerContent>()
            .delta = delta;
    }

    #[test]
    #[ignore = "需TG_INSTALL_ROOT原字体表；CPU消费者，不发送OS输入"]
    fn video_image_real_widgets_apply_presets_budget_rejection_and_manual_recovery() {
        let mut mei = fonts();
        let mut hints = HintTracker::default();
        let mut video = VideoSettings::default();
        let mut adaptive = adaptive_at(2);
        let mut frame = FrameRenderConfig::at_startup(
            country_core::resources::window::WindowSize::new(1920, 1080),
            &video,
        );
        let draw = |mei: &mut MeiContext,
                    video: &mut VideoSettings,
                    frame: &mut FrameRenderConfig,
                    adaptive: &mut AdaptiveRenderState,
                    hints: &mut HintTracker,
                    max| {
            mei.recycle_frame_nodes();
            let mut root = mei.safe_frame("video image test", true);
            let mut changed = false;
            ui_settings_video_image(
                &mut changed,
                &mut root,
                hints,
                video,
                frame,
                adaptive,
                max,
                "GPU: real-query boundary supplied by test",
            );
            changed
        };
        assert!(!draw(
            &mut mei,
            &mut video,
            &mut frame,
            &mut adaptive,
            &mut hints,
            2.
        ));
        delta(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Video.QualityPreset"),
            -1,
        );
        assert!(draw(
            &mut mei,
            &mut video,
            &mut frame,
            &mut adaptive,
            &mut hints,
            2.
        ));
        assert_eq!(matching_quality(&video), Some(1));
        assert_eq!(adaptive.native_tag(), 2); // 原同None目标不重置旧OOM。
        delta(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Video.QualityPreset"),
            2,
        );
        assert!(!draw(
            &mut mei,
            &mut video,
            &mut frame,
            &mut adaptive,
            &mut hints,
            1.1
        ));
        assert_eq!(matching_quality(&video), Some(1));
        video.world_resolution_scale = Some(2.);
        frame.world_resolution_scale = 0.7;
        delta(
            &mut mei,
            &format!(
                "    {}",
                lang_lib::lookup("Menu.Settings.Video.RenderScale")
            ),
            -1,
        );
        assert!(draw(
            &mut mei,
            &mut video,
            &mut frame,
            &mut adaptive,
            &mut hints,
            1.
        ));
        assert_eq!(video.world_resolution_scale, Some(1.));
        assert_eq!(frame.world_resolution_scale, 1.);
        assert_eq!(adaptive.native_tag(), 0);
    }

    #[test]
    #[ignore = "需TG_INSTALL_ROOT原字体表；CPU消费者，不发送OS输入"]
    fn video_display_real_widgets_preserve_full_modes_and_apply_fps_and_window_state() {
        use country_core::resources::window::{AvailableMonitor, AvailableMonitors};
        let mut mei = fonts();
        let mut hints = HintTracker::default();
        let mut video = VideoSettings::default();
        let monitor = MonitorMatch {
            name: Some("Display B".into()),
            physical_position: [1920, 0],
            physical_size: [2560, 1440],
        };
        let monitors = AvailableMonitors {
            monitors: vec![AvailableMonitor {
                monitor: monitor.clone(),
                video_modes: vec![
                    VideoMode {
                        resolution: [2560, 1440],
                        refresh_rate_millihertz: 144000,
                        bit_depth: 10,
                    },
                    VideoMode {
                        resolution: [2560, 1440],
                        refresh_rate_millihertz: 60000,
                        bit_depth: 8,
                    },
                    VideoMode {
                        resolution: [1920, 1080],
                        refresh_rate_millihertz: 60000,
                        bit_depth: 8,
                    },
                ],
            }],
        };
        video.display.monitor = Some(monitor);
        let draw = |mei: &mut MeiContext, video: &mut VideoSettings, hints: &mut HintTracker| {
            mei.recycle_frame_nodes();
            let root = mei.safe_frame("video display test", true);
            let mut changed = false;
            ui_settings_video_display(
                &mut changed,
                root,
                hints,
                video,
                &monitors,
                &[],
                &[],
                [0; 16],
            );
            changed
        };
        assert!(draw(&mut mei, &mut video, &mut hints));
        assert_eq!(
            video.display.fullscreen_config,
            FullscreenConfig::ClosestMatch(monitors.monitors[0].video_modes[0])
        );
        delta(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Video.RefreshRate"),
            -1,
        );
        assert!(draw(&mut mei, &mut video, &mut hints));
        assert_eq!(
            video.display.fullscreen_config,
            FullscreenConfig::ClosestMatch(monitors.monitors[0].video_modes[1])
        );
        delta(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Video.FrameRateLimit"),
            1,
        );
        assert!(draw(&mut mei, &mut video, &mut hints));
        assert_eq!(video.fps_limit, 72.);
        delta(
            &mut mei,
            &lang_lib::lookup("Menu.Settings.Video.DisplayMode"),
            -2,
        );
        assert!(draw(&mut mei, &mut video, &mut hints));
        assert!(!video.display.fullscreen);
        assert!(video.display.allow_exclusive_fullscreen); // 原窗口模式不改exclusive位。
        assert_eq!(video.display.windowed_resolution, [1920, 1080]);
    }
}

/// 原14024aa80：权威GameConfig.ui；自定义光标开启或尺寸改变必须发真实OS命令。
pub fn ui_settings_ui(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
    cursor: &mut bevy_ecs::event::EventWriter<
        country_core::resources::mouse_capture::OsCursorVisibilityCmd,
    >,
) {
    use country_core::resources::{
        mouse_capture::OsCursorVisibilityCmd as Cursor, ui_settings::ToolbarAlignment as A,
    };
    use toolbar::mei::widgets::tweaker::{TweakableBool, TweakableEnum, TweakableFloatWithStep};
    let demo = config.dev.demo;
    let ui = &mut config.ui;
    *changed |= node.tweaker(
        &lang_lib::lookup("Menu.Settings.Ui.IconSize"),
        hints.text(""),
        260.,
        TweakableFloatWithStep {
            value: &mut ui.icon_size_mult,
            step: 0.1,
            min: 0.5,
            max: 2.,
        },
    );
    *changed |= node.tweaker(
        &lang_lib::lookup("Menu.Settings.Ui.ToolbarAlignment"),
        hints.text(""),
        260.,
        TweakableEnum {
            value: &mut ui.toolbar_alignment,
            variants: &[A::Bottom, A::Left, A::Right, A::Top],
            formatter: |value| {
                lang_lib::lookup(match value {
                    A::Bottom => "Menu.Settings.Ui.ToolbarAlignment.Bottom",
                    A::Left => "Menu.Settings.Ui.ToolbarAlignment.Left",
                    A::Right => "Menu.Settings.Ui.ToolbarAlignment.Right",
                    A::Top => "Menu.Settings.Ui.ToolbarAlignment.Top",
                })
                .to_string()
            },
        },
    );
    *changed |= node.tweaker(
        &lang_lib::lookup("Menu.Settings.Ui.AutoHideToolbar"),
        hints.text(&lang_lib::lookup("Menu.Settings.Ui.AutoHideToolbarHint")),
        260.,
        TweakableBool(&mut ui.auto_hide_toolbar),
    );
    let mut cursor_changed = node.tweaker(
        &lang_lib::lookup("Menu.Settings.Ui.CustomCursor"),
        hints.text(&lang_lib::lookup("Menu.Settings.Ui.CustomCursorHint")),
        260.,
        TweakableBool(&mut ui.custom_cursor),
    );
    if ui.custom_cursor {
        cursor_changed |= node.tweaker(
            "Custom cursor size",
            hints.text(""),
            260.,
            TweakableFloatWithStep {
                value: &mut ui.custom_cursor_size,
                step: 1.,
                min: 16.,
                max: 128.,
            },
        );
    }
    if cursor_changed {
        *changed = true;
        cursor.write(if ui.custom_cursor {
            Cursor::Custom(ui.custom_cursor_size)
        } else {
            Cursor::Standard
        });
    }
    if !demo {
        *changed |= node.tweaker(
            &lang_lib::lookup("Menu.Settings.Ui.Logo"),
            hints.text(""),
            260.,
            TweakableBool(&mut ui.game_logo),
        );
    }
}

/// 原14024ae60的六个真实字段；名称与说明保留原英语开发者文本。
pub fn ui_settings_developer(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
) {
    use toolbar::mei::widgets::{LabelContent, tweaker::TweakableBool};
    for (name, hint, value) in [
        (
            "Day-night cycle",
            "Spins the Earth and moves the Moon.",
            &mut config.dev.day_night_cycle,
        ),
        (
            "Capture crashes",
            "If enabled, crash and panic handlers will be installed.",
            &mut config.dev.capture_crashes,
        ),
        (
            "Report crashes",
            "If enabled, captured crashes and panics will be reported to Sentry.",
            &mut config.dev.report_crashes,
        ),
        (
            "Enable internal hotkeys",
            "Unlocks the super-secret Tab mode, among others.",
            &mut config.dev.internal_hotkeys,
        ),
        (
            "Validate graphics",
            "Enables Vulkan validation layers, checking for API errors.",
            &mut config.video.graphics_validation,
        ),
        ("Demo mode", "Toggles demo mode.", &mut config.dev.demo),
    ] {
        *changed |= node.tweaker(name, hints.text(hint), 260., TweakableBool(value));
    }
    let label = node.label("Restart Tiny Glade to apply these changes.");
    let content = label.content_mut::<LabelContent>();
    content.text.set_metrics(32.).expect("原开发者提示字号");
    content.color = 0xff4020ff;
}

/// 原14024b120：相机控制行、键位子页及已有平台接口的绑定面板入口。
pub fn ui_settings_controls(
    changed: &mut bool,
    node: toolbar::mei::core::NodeContext,
    hints: &mut toolbar::mei::widgets::tweaker::HintTracker,
    config: &mut country_core::game_config::GameConfig,
    next: &mut country_core::resources::gui::NextEscMenuState,
    steam: Option<&country_core::systems::steam::SteamClientResource>,
    input: Option<&country_core::resources::steam::SteamInput>,
) {
    use country_core::resources::gui::{EscMenuState,SettingsMenuState};
    use toolbar::mei::core::{GlobalId,calculate_hash};
    let name=calculate_hash(&("key_remapping",()));
    let local=calculate_hash(&(name,0xa79b7268a2a968d9u64));
    let child=node.child_with_global_id(GlobalId(calculate_hash(&(node.id.0,local))));
    let mut rows=child.template("menu/key_remapping").into_slot("rows");
    crate::gui::gui_controls::show_control_tweakers(changed,rows,hints,config);
    rows.space(0.,40.);
    if rows.menu_button(&lang_lib::lookup("Menu.Settings.Controls.KeyBindings")).node().used() {
        next.set(EscMenuState::Settings(SettingsMenuState::KeyBindings));
    }
    if let (Some(steam),Some(input))=(steam,input) {
        if steam.client.is_some() && input.active_controller!=0 && rows.menu_button(
            &lang_lib::lookup("Menu.Settings.Controls.ControllerBindings")).node().used()
        {
            steam.input_show_binding_panel(input.active_controller);
        }
    }
}

/// 原系统按ButtonInput本帧just_pressed集合的迭代顺序消费；不排序或每帧只取一个。
/// 11个目标判别均非零，编译器生成的KeyCode::Unidentified匹配分支不可达。
pub fn detect_konami_code(keys: Res<ButtonInput<KeyCode>>, mut progress: Local<usize>) {
    for key in keys.get_just_pressed() {
        if advance_konami(*key, &mut progress) {
            SHOW_DEVELOPER_SETTINGS.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
fn advance_konami(key: KeyCode, progress: &mut usize) -> bool {
    if *progress >= KONAMI_LENGTH {
        return false;
    }
    if key == KONAMI_SEQUENCE[*progress] {
        *progress += 1;
        *progress == KONAMI_LENGTH
    } else {
        *progress = 0;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_background_matches_original_instruction_boundaries() {
        use country_core::resources::session::GladeType;
        let cases: Vec<(usize, u32, String)> = ron::from_str(include_str!(
            "../../../../tests/fixtures/native/toolbar-settings-background.ron"
        )).expect("原机器码背景夹具");
        let glades = GladeType::ALL;
        for (glade, bits, expected) in cases {
            assert_eq!(settings_background(glades[glade], f32::from_bits(bits)), expected,
                "glade={glade}, time_bits={bits:#010x}");
        }
    }
    #[derive(bevy_ecs::prelude::Resource)]
    struct PageProbe {
        page: u8,
        state: u8,
        changed: bool,
        hints: toolbar::mei::widgets::tweaker::HintTracker,
    }
    fn page_probe(
        mut mei: bevy_ecs::prelude::ResMut<toolbar::MeiContext>,
        mut config: bevy_ecs::prelude::ResMut<country_core::game_config::GameConfig>,
        mut probe: bevy_ecs::prelude::ResMut<PageProbe>,
        mut tutorial: bevy_ecs::event::EventWriter<country_core::resources::onboarding::TutorialOp>,
        mut cursor: bevy_ecs::event::EventWriter<
            country_core::resources::mouse_capture::OsCursorVisibilityCmd,
        >,
    ) {
        mei.recycle_frame_nodes();
        let root = mei.safe_frame("settings consumer probe", true);
        let PageProbe {
            page,
            state,
            changed,
            hints,
        } = &mut *probe;
        *changed = false;
        match *page {
            0 => ui_settings_game(state, changed, root, hints, &mut config, &mut tutorial),
            1 => ui_settings_audio(changed, root, hints, &mut config),
            2 => ui_settings_ui(changed, root, hints, &mut config, &mut cursor),
            3 => ui_settings_developer(changed, root, hints, &mut config),
            4 => crate::gui::gui_controls::show_control_tweakers(changed, root, hints, &mut config),
            _ => unreachable!(),
        }
    }
    #[test]
    #[ignore = "需TG_INSTALL_ROOT原字体表；CPU消费者，不发送OS输入"]
    fn settings_consumers_change_authority_and_reach_real_tutorial_and_cursor_channels() {
        use bevy_app::{Startup, Update};
        use bevy_ecs::schedule::IntoScheduleConfigs;
        use country_core::{
            game_config::GameConfig,
            game_profile::{GameProfile, SaveGameProfileCmd},
            resources::{mouse_capture::OsCursorVisibilityCmd as Cursor, onboarding::TutorialOp},
        };
        use toolbar::mei::{
            core::{MeiContext, NodeContext},
            widgets::{MenuButtonContent, tweaker::TweakerContent},
        };
        let root = std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
        let mut mei = MeiContext::default();
        mei.load_font_table(&root.join("assets/fonts/table.ron"))
            .unwrap();
        let mut app = App::new();
        app.insert_resource(mei)
            .init_resource::<GameConfig>()
            .init_resource::<GameProfile>()
            .insert_resource(PageProbe {
                page: 0,
                state: 2,
                changed: false,
                hints: Default::default(),
            })
            .init_resource::<system_onboarding::reset_hints::HintResetState>()
            .add_event::<TutorialOp>()
            .add_event::<Cursor>()
            .add_event::<SaveGameProfileCmd>()
            .add_systems(Startup, system_onboarding::hints_startup::hints_startup)
            .add_systems(
                Update,
                (page_probe, system_onboarding::reset_hints::reset_hints).chain(),
            );
        let activate = |app: &mut App, text: &str| {
            let mut mei = app.world_mut().resource_mut::<MeiContext>();
            for n in mei.nodes.values_mut() {
                n.state_bits &= !0x30;
            }
            let id = *mei
                .nodes
                .iter()
                .find(|(_, n)| {
                    let any: &dyn std::any::Any = n.content.as_ref();
                    any.downcast_ref::<MenuButtonContent>()
                        .is_some_and(|c| c.pending_text.as_deref() == Some(text))
                })
                .expect("真实页面按钮")
                .0;
            mei.nodes.get_mut(&id).unwrap().state_bits |= 0x10;
            drop(mei);
            app.update();
        };
        app.update();
        activate(
            &mut app,
            &lang_lib::lookup("Menu.Settings.Game.CompleteHints"),
        );
        assert_eq!(app.world().resource::<PageProbe>().state, 0);
        app.update(); // 原确认页在下一次建树才出现。
        activate(&mut app, &lang_lib::lookup("AreYouSure.ImSure"));
        assert!(app.world().resource::<GameProfile>().tutorial);
        assert!(
            app.world()
                .resource::<system_onboarding::reset_hints::HintResetState>()
                .pending
        );
        app.world_mut().resource_mut::<PageProbe>().state = 2;
        app.update();
        activate(
            &mut app,
            &lang_lib::lookup("Menu.Settings.Game.ResetTutorial"),
        );
        assert_eq!(app.world().resource::<PageProbe>().state, 1);
        app.update();
        activate(&mut app, &lang_lib::lookup("AreYouSure.ImSure"));
        assert!(!app.world().resource::<GameProfile>().tutorial);
        assert!(
            !app.world()
                .resource::<system_onboarding::reset_hints::HintResetState>()
                .pending
        );
        assert!(
            !app.world()
                .resource::<bevy_ecs::event::Events<SaveGameProfileCmd>>()
                .is_empty()
        );
        let increment_row = |app: &mut App, index: usize| {
            let mut mei = app.world_mut().resource_mut::<MeiContext>();
            let root = mei
                .nodes
                .values()
                .find(|n| {
                    n.content.is_empty()
                        && n.children.iter().any(|id| {
                            mei.nodes[id]
                                .content
                                .type_name()
                                .ends_with("TweakerContent")
                        })
                })
                .unwrap();
            let id = root.children[index];
            let context = NodeContext {
                mei: &mut *mei,
                id,
                _pad: 0,
            };
            context.content_mut::<TweakerContent>().delta = 1;
        };
        app.world_mut().resource_mut::<PageProbe>().page = 1;
        app.update();
        increment_row(&mut app, 0);
        app.update();
        assert_eq!(
            app.world()
                .resource::<GameConfig>()
                .audio
                .master_volume
                .to_bits(),
            1.1f32.to_bits()
        );
        assert!(app.world().resource::<PageProbe>().changed);
        app.world_mut().resource_mut::<PageProbe>().page = 2;
        app.update();
        increment_row(&mut app, 3);
        app.update();
        assert!(app.world().resource::<GameConfig>().ui.custom_cursor);
        assert!(
            app.world()
                .resource::<bevy_ecs::event::Events<Cursor>>()
                .iter_current_update_events()
                .any(|e| *e == Cursor::Custom(24.))
        );
        app.world_mut().resource_mut::<PageProbe>().page = 4;
        app.update();
        assert_eq!(
            app.world()
                .resource::<GameConfig>()
                .game
                .camera_rotation_pivot,
            Some(country_core::systems::main_camera::CameraRotationPivot::CursorDepth)
        );
        increment_row(&mut app, 4);
        app.update();
        assert_eq!(
            app.world()
                .resource::<GameConfig>()
                .game
                .camera_rotation_pivot,
            Some(country_core::systems::main_camera::CameraRotationPivot::FilteredCursorDepth)
        );
    }
    #[test]
    fn apply_mask_matches_original_instructions_and_keeps_full_video_mode() {
        use country_core::render::render_config::{
            FullscreenConfig, MonitorMatch, VideoMode, VideoSettings,
        };
        fn config(
            v: (
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ),
        ) -> VideoSettings {
            let mut c = VideoSettings::default();
            c.vsync_enable = v.0 != 0;
            c.display.monitor = match v.1 {
                0 => None,
                n => Some(MonitorMatch {
                    name: match n {
                        1 => None,
                        2 => Some("A".into()),
                        3 => Some("B".into()),
                        _ => unreachable!(),
                    },
                    physical_position: [v.2 as i32, v.3 as i32],
                    physical_size: [v.4, v.5],
                }),
            };
            c.display.fullscreen = v.6 != 0;
            c.display.allow_exclusive_fullscreen = v.7 != 0;
            c.display.fullscreen_config = if v.8 == 0 {
                FullscreenConfig::Best
            } else {
                FullscreenConfig::ClosestMatch(VideoMode {
                    resolution: [v.9, v.10],
                    refresh_rate_millihertz: v.11,
                    bit_depth: v.12 as u16,
                })
            };
            c
        }
        type Input = (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        );
        let rows: Vec<(Input, Input, u8)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-video-apply.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 420);
        for (index, (old, new, expected)) in rows.into_iter().enumerate() {
            let old = config(old);
            let new = config(new);
            let mut applied = LastAppliedVideoSettings::default();
            applied.populate_empty_from(&old);
            applied.populate_empty_from(&new);
            assert_eq!(applied.needs_apply(&new), expected, "原样本{index}");
        }
        let mode = FullscreenConfig::ClosestMatch(VideoMode {
            resolution: [2560, 1440],
            refresh_rate_millihertz: 144000,
            bit_depth: 10,
        });
        let serialized = ron::to_string(&mode).unwrap();
        assert!(serialized.starts_with("ClosestMatch("));
        assert_eq!(
            ron::from_str::<FullscreenConfig>(&serialized).unwrap(),
            mode
        );
        assert!(
            std::panic::catch_unwind(
                || LastAppliedVideoSettings::default().needs_apply(&VideoSettings::default())
            )
            .is_err()
        );
    }
    #[test]
    fn original_instruction_key_cases_match() {
        let rows: Vec<(usize, u32, usize, bool)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/native/toolbar-konami.ron"
        )))
        .unwrap();
        assert_eq!(rows.len(), 126);
        for (before, key, expected, unlocked) in rows {
            let key = match key {
                0 => KeyCode::Unidentified(bevy_input::keyboard::NativeKeyCode::Unidentified),
                0x14 => KeyCode::KeyA,
                0x15 => KeyCode::KeyB,
                0x3a => KeyCode::Enter,
                0x50 => KeyCode::ArrowDown,
                0x51 => KeyCode::ArrowLeft,
                0x52 => KeyCode::ArrowRight,
                0x53 => KeyCode::ArrowUp,
                0x28 => KeyCode::KeyU,
                _ => unreachable!(),
            };
            let mut progress = before;
            assert_eq!(advance_konami(key, &mut progress), unlocked);
            assert_eq!(progress, expected);
        }
    }
    #[test]
    fn original_eleven_keys_unlock_once_and_wrong_key_resets_progress() {
        use bevy_app::Update;
        SHOW_DEVELOPER_SETTINGS.store(false, std::sync::atomic::Ordering::Relaxed);
        let mut app = App::new();
        app.init_resource::<ButtonInput<KeyCode>>()
            .add_systems(Update, detect_konami_code);
        let press = |app: &mut App, key| {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.reset_all();
            keys.press(key);
            app.update();
        };
        press(&mut app, KeyCode::ArrowUp);
        press(&mut app, KeyCode::KeyX);
        for key in KONAMI_SEQUENCE.into_iter().skip(1) {
            press(&mut app, key);
        }
        assert!(!SHOW_DEVELOPER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed));
        press(&mut app, KeyCode::KeyX);
        for key in KONAMI_SEQUENCE {
            press(&mut app, key);
        }
        assert!(SHOW_DEVELOPER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed));
        press(&mut app, KeyCode::KeyX);
        assert!(SHOW_DEVELOPER_SETTINGS.load(std::sync::atomic::Ordering::Relaxed));
    }
}
