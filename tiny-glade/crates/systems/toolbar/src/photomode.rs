//! toolbar 拍照模式（B `toolbar::photomode` 可取证部分）。
//!
//! 恢复范围：
//! - `photomode_buttons`（I 0x14234c230）：动作字节跳表 0..=0x11 的每个已取证
//!   case（4 截图目录、5 相机循环、0xb 菜单、0xc Vermeer、0xd/0xe 配置位、
//!   0xf 隐藏 UI、0x11 回设置）；未知载荷标 U。
//! - 滑块完整13项数值表、归一化getter、三类镜头/焦点目标和重置；
//!   `ui_slider`保留原始绘制/回收、完整拖拽阶段、导航焦点与声音生命周期。
//! - `setup_photomode_toolbar`：原版完整/demo 两套 11 类别树、真实回调与菜单导航。
//! - `photomode_camera_buttons`：保留上一轮已判等的相机子集入口。
//!
//! 证据：`ANALYSIS-C-菜单拍照文本.md` §12-§14 与原函数符号/依赖注解；
//! 未逐语句闭合的字段/常量不填默认值。

pub mod lut_gallery;
pub mod setup_photomode_toolbar;
pub use setup_photomode_toolbar::setup_photomode_toolbar;
pub mod slider_math;
pub mod timelapse;

use bevy_ecs::prelude::*;

use country_core::{
    game_config::GameConfig,
    resources::{
        brush_sizes::BrushSizes,
        gui::{EscMenuState, NextEscMenuState},
        image_formation::ImageFormationConfigs,
        photo_mode::PhotoModeCmd,
        window::WindowFocusEvent,
    },
    startup::init_photomode_state::PhotomodeState,
    systems::{
        camera_rig::main_camera::{MainCamera, FOV_OFFSETS},
        mode_manager::{AppMode, RequestAppModeChange},
        render_loop::vermeer::{VermeerPhase, VermeerState},
    },
};

use crate::{
    resouces_and_components::{
        LogSlider, PhotomodeButton, UiInteractionState, UiLocked, UiPlaySelectAnimation,
        UiSliderReset,
    },
    ui_interaction::UiClickEvent,
};

/// Native photomode action byte (the `byte-4 → case 0..0xd` jump table).
///
/// The native table needs `PhotomodeState+0xd` (hide UI), the Vermeer phase,
/// and `ImageFormationConfigs` row flags; the action-byte mapping is the
/// observed one (ANALYSIS C12). U: the exact payload constants for actions 6/9
/// (`WindowFocusEvent`) and the screenshot directory source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhotomodeAction {
    /// 0..=3 select the photo camera.
    Camera(u8),
    /// 4: open the screenshots folder.
    OpenScreenshotsFolder,
    /// 5: next photo camera (demo table excluded).
    NextCamera,
    /// 6 / 9: `WindowFocusEvent` (payload U).
    FocusEvent(u8),
    /// 0xb: back to the main menu.
    BackToMenu,
    /// 0xc: Vermeer activate/deactivate.
    ToggleVermeer,
    /// 0xd / 0xe: flip the per-config `+0x3c` / `+0x3d` row flags.
    ToggleRowFlag(u8),
    /// 0xf: toggle `PhotomodeState.hide_ui`.
    ToggleHideUi,
    /// 0x11: back to settings / default app mode.
    BackToSettings,
}

/// O action-byte dispatch (jump table 0..=0x11). Actions 7, 8, 0xa and 0x10
/// are not recovered and return `None` instead of a guessed payload.
pub fn photomode_action(tag: u8) -> Option<PhotomodeAction> {
    Some(match tag {
        0..=3 => PhotomodeAction::Camera(tag),
        4 => PhotomodeAction::OpenScreenshotsFolder,
        5 => PhotomodeAction::NextCamera,
        6 | 9 => PhotomodeAction::FocusEvent(tag),
        0xb => PhotomodeAction::BackToMenu,
        0xc => PhotomodeAction::ToggleVermeer,
        0xd | 0xe => PhotomodeAction::ToggleRowFlag(tag),
        0xf => PhotomodeAction::ToggleHideUi,
        0x11 => PhotomodeAction::BackToSettings,
        _ => return None,
    })
}

/// Integration-provided screenshots directory (native `paths::config()` +
/// join; the `paths` crate is not a toolbar dependency).
#[derive(Resource, Clone, Debug)]
pub struct PhotomodeScreenshotFolder(pub std::path::PathBuf);

/// O 0x14234c230 `photomode_buttons` (camera/action producer).
///
/// Every handled click enqueues `UiPlaySelectAnimation` (native per-action
/// command tail). `PhotoModeCmd` stays the real camera consumer; the other
/// cases write their original resources/events.
#[allow(clippy::too_many_arguments)]
pub fn photomode_buttons(
    mode: Res<AppMode>,
    mut query: Query<(
        Entity,
        &PhotomodeButton,
        &mut UiInteractionState,
        Option<&UiLocked>,
    )>,
    mut clicks: EventReader<UiClickEvent>,
    mut photo: ResMut<PhotomodeState>,
    config: Res<GameConfig>,
    mut output: EventWriter<PhotoModeCmd>,
    mut focus_events: EventWriter<WindowFocusEvent>,
    mut next_menu: ResMut<NextEscMenuState>,
    mut vermeer: ResMut<VermeerState>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut app_mode: EventWriter<RequestAppModeChange>,
    screenshot_folder: Option<Res<PhotomodeScreenshotFolder>>,
    mut commands: Commands,
) {
    if *mode != AppMode::Photomode {
        return;
    }
    for click in clicks.read() {
        let Ok((entity, button, _, locked)) = query.get_mut(click.entity) else {
            continue;
        };
        if locked.is_some() {
            continue;
        }
        let Some(action) = photomode_action(button.tag) else {
            continue;
        };
        match action {
            PhotomodeAction::Camera(tag) => {
                output.write(PhotoModeCmd(tag));
            }
            PhotomodeAction::NextCamera => {
                let next = if config.dev.demo {
                    photo.camera_mode.next_demo()
                } else {
                    photo.camera_mode.next()
                };
                photo.camera_mode = next;
                output.write(PhotoModeCmd::from(next));
                focus_events.write(WindowFocusEvent(next as u8));
            }
            PhotomodeAction::OpenScreenshotsFolder => {
                // Native: OsWindowCmd + paths::config.join(dir) +
                // create_dir + open::that. The path owner is integration-side.
                if let Some(folder) = screenshot_folder.as_deref() {
                    let _ = std::fs::create_dir_all(&folder.0);
                    if let Err(error) = open::that(&folder.0) {
                        log::error!(
                            "failed to open photomode screenshots folder {}: {error}",
                            folder.0.display()
                        );
                    }
                }
            }
            PhotomodeAction::FocusEvent(tag) => {
                // U: native payload for actions 6/9 is not decoded; the action
                // byte is forwarded unchanged rather than inventing a target.
                focus_events.write(WindowFocusEvent(tag));
            }
            PhotomodeAction::BackToMenu => {
                next_menu.set(EscMenuState::Main);
            }
            PhotomodeAction::ToggleVermeer => {
                if vermeer.phase == VermeerPhase::Disabled {
                    vermeer.activate(true, false);
                } else {
                    vermeer.deactivate();
                }
            }
            PhotomodeAction::ToggleRowFlag(tag) => {
                let offset = if tag == 0xd { 0x3c } else { 0x3d };
                for row in formation.iter_non_default_mut() {
                    if offset == 0x3c {
                        row.flag_3c ^= 1;
                    } else {
                        row.flag_3d ^= 1;
                    }
                }
            }
            PhotomodeAction::ToggleHideUi => {
                photo.hide_ui = !photo.hide_ui;
            }
            PhotomodeAction::BackToSettings => {
                app_mode.write(RequestAppModeChange(AppMode::default()));
                photo.hide_ui = false;
                photo.walk_camera_initialized = false;
            }
        }
        commands.entity(entity).insert(UiPlaySelectAnimation {
            a: 0.1,
            b: 0.2,
            ..Default::default()
        });
    }
    // Native per-frame selected byte for the camera row (tags 0..=3).
    for (_, button, mut interaction, _) in &mut query {
        if button.tag <= 3 {
            interaction.selected = u8::from(button.tag == photo.camera_mode as u8);
        }
    }
}

/// Camera-only entry retained from the previous round (same real consumers).
pub fn photomode_camera_buttons(
    mode: Res<AppMode>,
    mut query: Query<(
        &PhotomodeButton,
        &mut country_core::components::UiInteractionState,
        Option<&UiLocked>,
    )>,
    mut clicks: EventReader<UiClickEvent>,
    photo: Res<PhotomodeState>,
    config: Res<GameConfig>,
    mut output: EventWriter<PhotoModeCmd>,
    mut commands: Commands,
) {
    if *mode != AppMode::Photomode {
        return;
    }
    for click in clicks.read() {
        let Ok((button, _, locked)) = query.get_mut(click.entity) else {
            continue;
        };
        if locked.is_some() {
            continue;
        }
        let command = match button.tag {
            0..=3 => PhotoModeCmd(button.tag),
            5 => PhotoModeCmd::from(if config.dev.demo {
                photo.camera_mode.next_demo()
            } else {
                photo.camera_mode.next()
            }),
            _ => continue,
        };
        output.write(command);
        commands.entity(click.entity).insert(UiPlaySelectAnimation {
            a: 0.1,
            b: 0.2,
            ..Default::default()
        });
    }
    for (button, mut interaction, _) in &mut query {
        if button.tag <= 3 {
            interaction.selected = u8::from(button.tag == photo.camera_mode as u8);
        }
    }
}

/// B getter 142349de0..142349fd4 returns a normalized slider position.
/// Tool brushes have a separate owner; see `brush_slider_val_01`.
pub fn slider_val_01(
    kind: u8,
    formation: &ImageFormationConfigs,
    camera: &MainCamera,
) -> Option<f32> {
    let row = formation.current();
    let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
    let value = match kind {
        0 => row.params_18[1],
        1 => row.params_18[2],
        2 => row.params_18[3],
        3 => row.params_30[0],
        4 => f32::from_bits(row.word_38),
        5 => row.params_30[1],
        6 => {
            return Some(slider_math::normalized_lens(
                camera.read_f32(lens + 0xc),
                camera.read_f32(lens + 0x10),
                camera.read_f32(lens + 8),
            ))
        }
        7 => slider_math::focus_parameter(camera.read_f32(focus_offset(camera) + 0x18)),
        8 => {
            return Some(slider_math::normalized_lens(
                camera.read_f32(lens + 0x1c),
                camera.read_f32(lens + 0x20),
                camera.read_f32(lens + 0x18),
            ))
        }
        _ => return None,
    };
    slider_math::normalized(kind, value)
}

/// Native fifth argument is BrushSizes. Kind 9 reads a distinct scalar owner.
pub fn brush_slider_val_01(kind: u8, brushes: &BrushSizes) -> Option<f32> {
    slider_math::normalized(
        kind,
        match kind {
            10 => brushes.path,
            11 => brushes.water,
            12 => brushes.garden,
            _ => return None,
        },
    )
}

/// The native fourth scalar argument is EraserSizeMultiplier, as confirmed
/// by ui_slider's PDB FunctionSystem parameter list and its independent store.
pub fn slider_current_position(
    kind: u8,
    formation: &ImageFormationConfigs,
    camera: &MainCamera,
    eraser: &country_core::resources::walls::EraserSizeMultiplier,
    brushes: &BrushSizes,
) -> Option<f32> {
    match kind {
        0..=8 => slider_val_01(kind, formation, camera),
        9 => slider_math::normalized(kind, eraser.0),
        10..=12 => brush_slider_val_01(kind, brushes),
        _ => None,
    }
}

/// Original focus-selector table 142aabb68 (six camera bodies).
fn focus_offset(camera: &MainCamera) -> usize {
    0x100 + [0x70, 0x128, 0x1a8, 0x248, 0x300, 0x388][camera.mode_byte() as usize]
}

/// Changed distance switches focus modes 0/1 into mode 3 and preserves the
/// original start-pose payload. Reset performs the transition unconditionally.
fn set_focus_distance(camera: &mut MainCamera, value: f32, force: bool) {
    let focus = focus_offset(camera);
    if force || camera.read_f32(focus + 0x18) != value {
        camera.write_f32(focus + 0x18, value);
        let mode = u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap());
        if mode < 2 {
            camera.write_f32(focus, f32::from_bits(3));
            let position = [camera.read_f32(focus + 0x10), camera.read_f32(focus + 0x14)];
            camera.write_f32(focus + 4, position[0]);
            camera.write_f32(focus + 8, position[1]);
        }
    }
}

/// B lower/upper endpoint tables 14304edc0/14304ed8c.
pub fn slider_range(kind: u8) -> Option<(f32, f32)> {
    slider_math::RANGES.get(kind as usize).copied()
}

/// Native parameter dispatch: 0..5 writes both editable formation rows;
/// 6..8 writes the selected original camera's lens/focus. Tool brush writes
/// 10..12 run outside photomode. `apply_slider_with_eraser` includes kind 9.
pub fn apply_slider_value(
    kind: u8,
    value: f32,
    formation: &mut ImageFormationConfigs,
    brushes: &mut BrushSizes,
    photomode_open: bool,
    _mode: AppMode,
    camera: &mut MainCamera,
) -> bool {
    match kind {
        0..=5 if photomode_open => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    0 => row.params_18[1] = value,
                    1 => row.params_18[2] = value,
                    2 => row.params_18[3] = value,
                    3 => row.params_30[0] = value,
                    4 => row.word_38 = value.to_bits(),
                    5 => row.params_30[1] = value,
                    _ => unreachable!(),
                }
            }
            true
        }
        6 | 8 if photomode_open => {
            let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
            let (target, lower, upper) = if kind == 6 {
                (8, 0xc, 0x10)
            } else {
                (0x18, 0x1c, 0x20)
            };
            let value = slider_math::lens_parameter(
                camera.read_f32(lens + lower),
                camera.read_f32(lens + upper),
                value,
            );
            camera.write_f32(lens + target, value);
            true
        }
        7 if photomode_open => {
            let value = LogSlider {
                min: -2.0,
                max: 2.0,
                _pad: 0.5,
                log2_base: -3.0,
            }
            .param_to_value(value);
            set_focus_distance(camera, value, false);
            true
        }
        // Kind 9 is handled by apply_slider_with_eraser; it must not
        // overwrite one of the unrelated three canonical BrushSizes fields.
        0xa if !photomode_open => {
            brushes.set_path(value);
            true
        }
        0xb if !photomode_open => {
            brushes.set_water(value);
            true
        }
        0xc if !photomode_open => {
            brushes.set_garden(value);
            true
        }
        _ => false,
    }
}

/// All original 13 parameter owners. The eraser's independent clamp must not
/// pass through Path/Water/Garden's additional per-tool bounds.
#[allow(clippy::too_many_arguments)]
pub fn apply_slider_with_eraser(
    kind: u8,
    value: f32,
    formation: &mut ImageFormationConfigs,
    brushes: &mut BrushSizes,
    photomode_open: bool,
    mode: AppMode,
    camera: &mut MainCamera,
    eraser: &mut country_core::resources::walls::EraserSizeMultiplier,
) -> bool {
    if kind == 9 {
        if photomode_open {
            return false;
        }
        let lower = if 0.3 > value { 0.3 } else { value };
        eraser.0 = if 2.0 < lower { 2.0 } else { lower };
        true
    } else {
        apply_slider_value(
            kind,
            value,
            formation,
            brushes,
            photomode_open,
            mode,
            camera,
        )
    }
}

/// `ui_slider_reset` 0x142347f90 recovered reset cases.
///
/// 0..2 copies the immutable default row's grading values; 3..5 uses the
/// original defaults (0,0,.015). 6/8 restores lens defaults and 7 applies
/// LogSlider::param_to_value(.5), including the original focus transition.
pub fn reset_slider(
    kind: u8,
    formation: &mut ImageFormationConfigs,
    camera: &mut MainCamera,
    brushes: &mut BrushSizes,
) -> bool {
    let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
    let defaults = formation.get(0).unwrap().clone();
    match kind {
        0..=2 => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    0 => row.params_18[1] = defaults.params_18[1],
                    1 => row.params_18[2] = defaults.params_18[2],
                    2 => row.params_18[3] = defaults.params_18[3],
                    _ => unreachable!(),
                }
            }
            true
        }
        3..=5 => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    3 => row.params_30[0] = 0.,
                    4 => row.word_38 = 0,
                    5 => row.params_30[1] = 0.015,
                    _ => unreachable!(),
                }
            }
            true
        }
        6 => {
            let default = camera.read_f32(lens + 0x14);
            camera.write_f32(lens + 8, default);
            true
        }
        7 => {
            let default = LogSlider {
                min: -2.0,
                max: 2.0,
                _pad: 0.5,
                log2_base: -3.0,
            }
            .param_to_value(0.5);
            set_focus_distance(camera, default, true);
            true
        }
        8 => {
            let default = camera.read_f32(lens + 0x24);
            camera.write_f32(lens + 0x18, default);
            true
        }
        0xa => {
            brushes.reset_path();
            true
        }
        0xb => {
            brushes.reset_water();
            true
        }
        0xc => {
            brushes.reset_garden();
            true
        }
        _ => false,
    }
}

/// Original full draw/input consumer; parameter mapping stays in this module.
pub mod ui_slider;
pub use ui_slider::ui_slider;

/// O/I 0x142347f90 `ui_slider_reset`: clicking a reset widget restores the
/// paired slider row's default. The original 1-byte reset component carries
/// the kind itself; the reset entity does not require a UiSlider component.
pub fn ui_slider_reset(
    mut clicks: EventReader<UiClickEvent>,
    resets: Query<&UiSliderReset>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut camera: ResMut<MainCamera>,
    mut brushes: ResMut<BrushSizes>,
) {
    for click in clicks.read() {
        let Ok(reset) = resets.get(click.entity) else {
            continue;
        };
        reset_slider(reset.flag, &mut formation, &mut camera, &mut brushes);
    }
}

/// Native LogSlider current value helper used by the focus slider (case 7 is
/// U, but the inverse mapping is O and kept callable for the recovered range).
pub fn log_slider_value(slider: &LogSlider, value: f32) -> f32 {
    slider.value_to_param(value)
}

#[cfg(test)]
mod slider_restoration_tests {
    use super::*;
    use country_core::systems::camera_rig::main_camera::CameraMode;

    #[test]
    fn reset_uses_immutable_configuration_instead_of_zeroing_contrast() {
        let mut formation = ImageFormationConfigs::default();
        formation.configs[0].params_18 = [0.7, -0.75, 1.25, 0.85];
        formation.configs[1].params_18 = [0.4, 9., 8., 7.];
        formation.configs[2].params_18 = [0.2, 6., 5., 4.];
        let original = formation.configs[0].clone();
        let mut camera = MainCamera::new(1.0);
        let mut brushes = BrushSizes {
            path: 0.8,
            water: 2.,
            garden: 3.,
        };
        for kind in 0..=2 {
            assert!(reset_slider(
                kind,
                &mut formation,
                &mut camera,
                &mut brushes
            ));
        }
        assert_eq!(formation.configs[0], original);
        assert_eq!(formation.configs[1].params_18, [0.4, -0.75, 1.25, 0.85]);
        assert_eq!(formation.configs[2].params_18, [0.2, -0.75, 1.25, 0.85]);
    }

    #[test]
    fn camera_slider_keeps_each_original_body_and_focus_transition() {
        let mut camera = MainCamera::new(1.0);
        let mut formation = ImageFormationConfigs::default();
        let mut brushes = BrushSizes {
            path: 0.8,
            water: 2.,
            garden: 3.,
        };
        for mode in 0..6 {
            camera.set_mode_byte(CameraMode(mode));
            let lens = 0x100 + FOV_OFFSETS[mode as usize];
            camera.write_f32(lens + 0xc, 20.);
            camera.write_f32(lens + 0x10, 100.);
            camera.write_f32(lens + 0x1c, 0.5);
            camera.write_f32(lens + 0x20, 8.5);
            let focus = focus_offset(&camera);
            camera.write_f32(focus, f32::from_bits(1));
            camera.write_f32(focus + 0x10, 17.);
            camera.write_f32(focus + 0x14, -23.);
            for (kind, value) in [(6, 0.25), (7, 0.0), (8, 0.75)] {
                assert!(apply_slider_value(
                    kind,
                    value,
                    &mut formation,
                    &mut brushes,
                    true,
                    AppMode::Path,
                    &mut camera
                ));
            }
            assert_eq!(camera.read_f32(lens + 8), 40.);
            assert_eq!(camera.read_f32(lens + 0x18), 6.5);
            assert_eq!(camera.read_f32(focus + 0x18), 0.125);
            assert_eq!(camera.read_f32(focus).to_bits(), 3);
            assert_eq!(
                [camera.read_f32(focus + 4), camera.read_f32(focus + 8)],
                [17., -23.]
            );
        }
        assert!(!apply_slider_value(
            10,
            1.1,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path,
            &mut camera
        ));
        assert_eq!(brushes.path, 0.8);
    }

    #[test]
    fn click_on_reset_component_needs_no_slider_component() {
        let mut world = World::new();
        world.init_resource::<Events<UiClickEvent>>();
        world.insert_resource(ImageFormationConfigs::default());
        world.insert_resource(BrushSizes {
            path: 0.8,
            water: 2.,
            garden: 3.,
        });
        let mut camera = MainCamera::new(1.0);
        let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
        camera.write_f32(lens + 8, 2.);
        camera.write_f32(lens + 0x14, 73.);
        world.insert_resource(camera);
        let entity = world.spawn(UiSliderReset { flag: 6 }).id();
        world
            .resource_mut::<Events<UiClickEvent>>()
            .send(UiClickEvent { entity });
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_slider_reset);
        schedule.run(&mut world);
        assert_eq!(world.resource::<MainCamera>().read_f32(lens + 8), 73.);
    }
}
