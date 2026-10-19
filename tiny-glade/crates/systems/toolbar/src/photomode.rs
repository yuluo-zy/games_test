//! toolbar 拍照模式（B `toolbar::photomode` 可取证部分）。
//!
//! 恢复范围：
//! - `photomode_buttons`（I 0x14234c230）：动作字节跳表 0..=0x11 的每个已取证
//!   case（4 截图目录、5 相机循环、0xb 菜单、0xc Vermeer、0xd/0xe 配置位、
//!   0xf 隐藏 UI、0x11 回设置）；未知载荷标 U。
//! - `ui_slider`（I 0x1423486d0）/`ui_slider_reset`（I 0x142347f90）：
//!   `slider_val_01`、参数写目标 0..0xc、`slider_range`；被截断分支标 U。
//! - `photomode_camera_buttons`：保留上一轮已判等的相机子集入口。
//!
//! 证据：`ANALYSIS-C-菜单拍照文本.md` §12-§14 与原函数符号/依赖注解；
//! 未逐语句闭合的字段/常量不填默认值。

pub mod lut_gallery;
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
        LogSlider, PhotomodeButton, UiInteractionState, UiLocked, UiPlaySelectAnimation, UiSlider,
        UiSliderReset,
    },
    ui_interaction::{UiClickEvent, UiDragEvent},
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
    mut query: Query<
        (
            Entity,
            &PhotomodeButton,
            &mut UiInteractionState,
            Option<&UiLocked>,
        ),
    >,
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
    mut query: Query<
        (
            &PhotomodeButton,
            &mut country_core::components::UiInteractionState,
            Option<&UiLocked>,
        ),
    >,
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

/// O `slider_val_01` 0x142349de0 for the recovered cases.
///
/// 0..=5 read the active `ImageFormationConfig` row fields (`+0x18..`, `+0x30..`,
/// `+0x38`); 6 is the row word; 7/8 select MainCamera focus/lens values that the
/// current camera owner does not expose per-slider (U). 9..=0xc are empty in the
/// native switch and return `None`.
pub fn slider_val_01(
    kind: u8,
    formation: &ImageFormationConfigs,
    _camera: &MainCamera,
) -> Option<f32> {
    let row = formation.current();
    Some(match kind {
        0 => row.params_18[0],
        1 => row.params_18[1],
        2 => row.params_18[2],
        3 => row.params_18[3],
        4 => row.params_30[0],
        5 => row.params_30[1],
        6 => f32::from_bits(row.word_38),
        7 | 8 => return None,
        _ => return None,
    })
}

/// O `slider_range` 0x1423486d0 range table for the recovered kinds.
///
/// 9 = brush size clamp 0.3..2.0 (ANALYSIS C14); 0xa/0xb/0xc use the real
/// `BrushSizes` ranges. U: 0..8 ranges live in the unfetched table
/// `0x14304ed8c/0x14304edc0`, so those kinds return `None`.
pub fn slider_range(kind: u8) -> Option<(f32, f32)> {
    match kind {
        9 => Some((0.3, 2.0)),
        0xa => Some(country_core::resources::brush_sizes::PATH_RANGE),
        0xb => Some(country_core::resources::brush_sizes::WATER_RANGE),
        0xc => Some(country_core::resources::brush_sizes::GARDEN_RANGE),
        _ => None,
    }
}

/// Native slider → parameter switch. Returns true when the kind had a real
/// write target. Cases 0..=5 write the two editable `ImageFormationConfig`
/// rows (`iter_non_default_mut`), 9/0xa/0xb/0xc write the real brush sizes.
/// U: 6/7/8 (FOV/focus/lens) need the per-lens slider mapping and are left
/// untouched, as are the truncated 0xd+ branches.
pub fn apply_slider_value(
    kind: u8,
    value: f32,
    formation: &mut ImageFormationConfigs,
    brushes: &mut BrushSizes,
    photomode_open: bool,
    mode: AppMode,
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
        9 => {
            // Native clamps the non-photomode brush size to 0.3..2.0 and writes
            // the active tool's brush. U: the exact tool selector byte is not
            // decoded; the canonical AppMode owner selects the same three
            // brush fields.
            let size = value.clamp(0.3, 2.0);
            match mode {
                AppMode::Path => brushes.set_path(size),
                AppMode::Water => brushes.set_water(size),
                AppMode::Garden => brushes.set_garden(size),
                _ => return false,
            }
            true
        }
        0xa => {
            brushes.set_path(value);
            true
        }
        0xb => {
            brushes.set_water(value);
            true
        }
        0xc => {
            brushes.set_garden(value);
            true
        }
        _ => false,
    }
}

/// `ui_slider_reset` 0x142347f90 recovered reset cases.
///
/// 0..=2 restore the three editable row fields, 3..=5 restore the row
/// `+0x30/+0x38/+0x34` values, 6/8 restore the current lens FOV/distance, and
/// 0xa/0xb/0xc reset the real brush owners. U: case 7 (log focus slider) and
/// the truncated 0x11/0x15/0x16/0x2a branches are not applied.
pub fn reset_slider(
    kind: u8,
    formation: &mut ImageFormationConfigs,
    camera: &mut MainCamera,
    brushes: &mut BrushSizes,
) -> bool {
    let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
    match kind {
        0..=2 => {
            for row in formation.iter_non_default_mut() {
                match kind {
                    0 => row.params_18[1] = 0.,
                    1 => row.params_18[2] = 0.,
                    2 => row.params_18[3] = 0.,
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
                    5 => row.params_30[1] = 0.,
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

/// O/I 0x1423486d0 `ui_slider`: apply real slider drags to the parameter owners.
///
/// Native tracks the two mouse buttons directly; this source adapter consumes
/// the bridge's `UiDragEvent` (`state == 3` = dragging) and maps the cursor
/// inside the entity extent through `slider_range`. Kinds whose range/table are
/// U are ignored (no fabricated default value).
pub fn ui_slider(
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    mut drags: EventReader<UiDragEvent>,
    sliders: Query<(&UiSlider, &country_core::components::Ui2dExtent)>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut brushes: ResMut<BrushSizes>,
    menu: Res<EscMenuState>,
    mode: Res<AppMode>,
) {
    // Native case 0..5 requires EscMenuState 0x13; brush cases run outside
    // photomode too, so the gate is applied per kind in `apply_slider_value`.
    let photomode_open = matches!(*menu, EscMenuState::Photomode(_));
    for drag in drags.read() {
        if drag.state != 3 {
            continue;
        }
        let Ok((slider, extent)) = sliders.get(drag.entity) else {
            continue;
        };
        let kind = slider.kind();
        let Some((min, max)) = slider_range(kind) else {
            continue;
        };
        let left = extent.pos.x - extent.size.x * 0.5;
        let fraction = if extent.size.x > 0. {
            ((cursor.0.x - left) / extent.size.x).clamp(0., 1.)
        } else {
            continue;
        };
        let value = min + (max - min) * fraction;
        apply_slider_value(
            kind,
            value,
            &mut formation,
            &mut brushes,
            photomode_open,
            *mode,
        );
    }
}

/// O/I 0x142347f90 `ui_slider_reset`: clicking a reset widget restores the
/// paired slider row's default. The native reset entity carries the slider
/// kind; this adapter reads `UiSlider` on the same entity when present.
pub fn ui_slider_reset(
    mut clicks: EventReader<UiClickEvent>,
    resets: Query<(&UiSliderReset, Option<&UiSlider>)>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut camera: ResMut<MainCamera>,
    mut brushes: ResMut<BrushSizes>,
) {
    for click in clicks.read() {
        let Ok((_reset, slider)) = resets.get(click.entity) else {
            continue;
        };
        let Some(slider) = slider else {
            continue;
        };
        reset_slider(
            slider.kind(),
            &mut formation,
            &mut camera,
            &mut brushes,
        );
    }
}

/// Native LogSlider current value helper used by the focus slider (case 7 is
/// U, but the inverse mapping is O and kept callable for the recovered range).
pub fn log_slider_value(slider: &LogSlider, value: f32) -> f32 {
    slider.value_to_param(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_table_matches_observed_bytes_and_leaves_unknowns_none() {
        for tag in 0..=3 {
            assert_eq!(photomode_action(tag), Some(PhotomodeAction::Camera(tag)));
        }
        assert_eq!(photomode_action(4), Some(PhotomodeAction::OpenScreenshotsFolder));
        assert_eq!(photomode_action(5), Some(PhotomodeAction::NextCamera));
        assert_eq!(photomode_action(6), Some(PhotomodeAction::FocusEvent(6)));
        assert_eq!(photomode_action(9), Some(PhotomodeAction::FocusEvent(9)));
        assert_eq!(photomode_action(0xb), Some(PhotomodeAction::BackToMenu));
        assert_eq!(photomode_action(0xc), Some(PhotomodeAction::ToggleVermeer));
        assert_eq!(photomode_action(0xd), Some(PhotomodeAction::ToggleRowFlag(0xd)));
        assert_eq!(photomode_action(0xf), Some(PhotomodeAction::ToggleHideUi));
        assert_eq!(photomode_action(0x11), Some(PhotomodeAction::BackToSettings));
        for unknown in [7, 8, 0xa, 0x10, 0x12, 0xff] {
            assert_eq!(photomode_action(unknown), None);
        }
    }

    #[test]
    fn slider_value_reads_the_active_formation_row() {
        let mut formation = ImageFormationConfigs::default();
        formation.current_mut().params_18 = [10., 11., 12., 13.];
        formation.current_mut().params_30 = [20., 21.];
        formation.current_mut().word_38 = 7.5f32.to_bits();
        let camera = MainCamera::default();
        assert_eq!(slider_val_01(0, &formation, &camera), Some(10.));
        assert_eq!(slider_val_01(3, &formation, &camera), Some(13.));
        assert_eq!(slider_val_01(4, &formation, &camera), Some(20.));
        assert_eq!(slider_val_01(6, &formation, &camera), Some(7.5));
        assert_eq!(slider_val_01(7, &formation, &camera), None);
    }

    #[test]
    fn slider_ranges_use_real_brush_bounds_only() {
        assert_eq!(slider_range(9), Some((0.3, 2.0)));
        assert_eq!(
            slider_range(0xa),
            Some(country_core::resources::brush_sizes::PATH_RANGE)
        );
        assert_eq!(slider_range(0), None);
        assert_eq!(slider_range(0x10), None);
    }

    #[test]
    fn applying_values_writes_every_editable_row_and_brush() {
        let mut formation = ImageFormationConfigs::default();
        let mut brushes = BrushSizes {
            path: 0.8,
            water: 2.,
            garden: 3.,
        };
        for (kind, value) in [(0u8, 1.5f32), (1, 2.5), (2, 3.5), (3, 4.5), (4, 5.5), (5, 6.5)] {
            assert!(apply_slider_value(
                kind,
                value,
                &mut formation,
                &mut brushes,
                true,
                AppMode::Path
            ));
        }
        for row in formation.iter_non_default_mut() {
            assert_eq!(row.params_18[1..], [1.5, 2.5, 3.5]);
            assert_eq!(row.params_30, [4.5, 6.5]);
            assert_eq!(row.word_38, 5.5f32.to_bits());
        }
        assert!(apply_slider_value(
            9,
            9.,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path
        ));
        // Native case 9 clamps to 0.3..2.0, then the real path brush clamp
        // enforces its own 1.2 upper bound.
        assert_eq!(brushes.path, 1.2);
        assert!(apply_slider_value(
            0xa,
            1.0,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path
        ));
        assert_eq!(brushes.path, 1.0);
        assert!(apply_slider_value(
            0xb,
            4.0,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path
        ));
        assert_eq!(brushes.water, 4.0);
        assert!(apply_slider_value(
            0xc,
            2.5,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path
        ));
        assert_eq!(brushes.garden, 2.5);
        assert!(!apply_slider_value(
            6,
            1.,
            &mut formation,
            &mut brushes,
            true,
            AppMode::Path
        ));
        // Formation writes outside photomode are rejected.
        assert!(!apply_slider_value(
            0,
            1.,
            &mut formation,
            &mut brushes,
            false,
            AppMode::Path
        ));
    }
}
