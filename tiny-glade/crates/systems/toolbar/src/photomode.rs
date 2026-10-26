//! toolbar 拍照模式（B `toolbar::photomode` 可取证部分）。
//!
//! 恢复范围：
//! - `photomode_buttons`（I 0x14234c230）：动作字节跳表 0..=0x11 的每个已取证
//!   case（4 截图目录、5 相机循环、0xb LUT捕获菜单、0xc Vermeer、0xd/0xe 配置位、
//!   0xf 隐藏 UI、0x11 退出摄影）；未知载荷标 U。
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
        gui::{DebugGui, EscMenuState, NextEscMenuState, PhotomodeScreen},
        image_formation::ImageFormationConfigs,
        photo_mode::PhotoModeCmd,
        platform::RuntimePlatform,
        window::OsWindowCmd,
    },
    startup::init_photomode_state::{PhotomodeState, SteamdeckScreenshotMsgScreen},
    systems::{
        camera_rig::main_camera::{FOV_OFFSETS, MainCamera},
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
/// observed one (ANALYSIS C12). B PDB binds PhotoModeCmd and OsWindowCmd;
/// the decompiler's WindowFocusEvent alias is not the actual event identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhotomodeAction {
    /// 0..=3 select the photo camera.
    Camera(u8),
    /// 4: open the screenshots folder.
    OpenScreenshotsFolder,
    /// 5: next photo camera (demo table excluded).
    NextCamera,
    /// Original camera-command byte; tag9 requests screenshot command4.
    Command(u8),
    /// 0x10: cycle the current camera focus target.
    CycleFocus,
    /// 0xb: request the LUT gallery capture before opening the browser.
    OpenLutGallery,
    /// 0xc: Vermeer activate/deactivate.
    ToggleVermeer,
    /// 0xd / 0xe: flip the per-config `+0x3c` / `+0x3d` row flags.
    ToggleRowFlag(u8),
    /// 0xf: toggle `PhotomodeState.hide_ui`.
    ToggleHideUi,
    /// 0x11: leave photomode for the original default app mode.
    LeavePhotomode,
}

/// O action-byte dispatch (jump table 0..=0x11). Actions7/8/0xa have no
/// business branch. Tag6 forwards its native byte but has no toolbar producer;
/// it does not establish a new camera variant.
pub fn photomode_action(tag: u8) -> Option<PhotomodeAction> {
    Some(match tag {
        0..=3 => PhotomodeAction::Camera(tag),
        4 => PhotomodeAction::OpenScreenshotsFolder,
        5 => PhotomodeAction::NextCamera,
        6 => PhotomodeAction::Command(6),
        9 => PhotomodeAction::Command(4),
        0x10 => PhotomodeAction::CycleFocus,
        0xb => PhotomodeAction::OpenLutGallery,
        0xc => PhotomodeAction::ToggleVermeer,
        0xd | 0xe => PhotomodeAction::ToggleRowFlag(tag),
        0xf => PhotomodeAction::ToggleHideUi,
        0x11 => PhotomodeAction::LeavePhotomode,
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
        ComponentWrite<UiInteractionState>,
        Option<&UiLocked>,
    )>,
    mut clicks: EventReader<UiClickEvent>,
    mut photo: ResMut<PhotomodeState>,
    config: Res<GameConfig>,
    mut output: EventWriter<PhotoModeCmd>,
    mut window_commands: EventWriter<OsWindowCmd>,
    mut next_menu: ResMut<NextEscMenuState>,
    mut vermeer: ResMut<VermeerState>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut app_mode: EventWriter<RequestAppModeChange>,
    screenshot_folder: Res<PhotomodeScreenshotFolder>,
    mut commands: Commands,
    (mut camera, mut screen, mut debug): (
        ResMut<MainCamera>,
        ResMut<PhotomodeScreen>,
        ResMut<DebugGui>,
    ),
    (platform, mut screenshot_message, mut mei): (
        Res<RuntimePlatform>,
        ResMut<SteamdeckScreenshotMsgScreen>,
        ResMut<crate::MeiContext>,
    ),
) {
    if *mode != AppMode::Photomode {
        return;
    }
    if screenshot_message.0 {
        screenshot_message_ui(&mut mei, *platform, &mut screenshot_message);
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
                // Original next() only computes a byte. The actual camera
                // transition consumer owns publishing PhotomodeState.
                output.write(PhotoModeCmd::from(next));
            }
            PhotomodeAction::OpenScreenshotsFolder => {
                if matches!(platform.native_tag(), 0 | 1) {
                    screenshot_message.0 = true;
                } else {
                    window_commands.write(OsWindowCmd);
                    let folder = &screenshot_folder.0;
                    // B creates only the final directory and never opens it
                    // following a failed creation. Config owns the parent.
                    if folder.is_dir() || std::fs::create_dir(folder).is_ok() {
                        if let Err(error) = open::that(folder) {
                            log::error!(
                                "failed to open screenshots folder {}: {error}",
                                folder.display()
                            );
                        }
                    }
                }
            }
            PhotomodeAction::Command(tag) => {
                output.write(PhotoModeCmd(tag));
            }
            PhotomodeAction::CycleFocus => cycle_focus_target(&mut camera),
            PhotomodeAction::OpenLutGallery => {
                // B14234caf2 writes flag0 and menu discriminant10. The
                // existing screenshot gather owns the pending capture and
                // transition to Photomode(true); never open the gallery early.
                next_menu.set(EscMenuState::Photomode(false));
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
            PhotomodeAction::LeavePhotomode => {
                app_mode.write(RequestAppModeChange(AppMode::default()));
                // B14234cc35..cc50 resets DebugGui+103 and screen+1.
                // PhotomodeState is unchanged until its actual entry owner.
                debug.camera_mode =
                    country_core::systems::camera_rig::main_camera::CameraMode::NORMAL;
                screen.enabled = false;
            }
        }
        commands.entity(entity).insert(UiPlaySelectAnimation {
            a: 0.1,
            b: 0.2,
            ..Default::default()
        });
    }
    // Original tail publishes selected bytes for all actual toggle producers.
    for (_, button, mut interaction, _) in &mut query {
        let selected = match button.tag {
            0..=3 => Some(button.tag == photo.camera_mode as u8),
            12 => Some(vermeer.is_active()),
            13 => Some(formation.current_mut().flag_3c != 0),
            14 => Some(formation.current_mut().flag_3d != 0),
            16 => {
                let focus = focus_offset(&camera);
                Some(u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap()) < 2)
            }
            _ => None,
        };
        if let Some(selected) = selected {
            interaction.selected = u8::from(selected);
        }
    }
}

/// Original14234c2a6..c5ed. The platform-specific installed template owns the
/// notice and image; only its answer slot is filled by the real menu widget.
fn screenshot_message_ui(
    mei: &mut crate::MeiContext,
    platform: RuntimePlatform,
    visible: &mut SteamdeckScreenshotMsgScreen,
) {
    let template = match platform.native_tag() {
        0 => "steamdeck/screenshot_msg",
        1 => "gfn/screenshot_msg",
        _ => panic!("screenshot message is only valid on Steam Deck or GeForce NOW"),
    };
    let root = mei.safe_frame("steamdeck_screenshot_msg", true);
    let answer = root.template(template).into_slot("answer");
    let button = answer.menu_button(&lang_lib::lookup("Ok")).menu_hotkey(19);
    if button.node().used() {
        visible.0 = false;
    }
}

/// Original do_photomode14234b86d..bfb2. The real camera-focus canvas owns
/// input; the existing autofocus compute pass consumes `photo.index` and
/// returns DofFocusReadbackSsbo. This is deliberately not MouseDepthValue.
#[allow(clippy::too_many_arguments)]
pub fn photomode_focus(
    screen: Res<PhotomodeScreen>,
    menu: Res<EscMenuState>,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    modifiers: Res<country_core::systems::backend::input::mod_key::Modifiers>,
    cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    window: Res<country_core::resources::window::WindowSize>,
    library: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    mut camera: ResMut<MainCamera>,
    mut photo: ResMut<PhotomodeState>,
    mut mei: ResMut<crate::MeiContext>,
    readbacks: Option<Res<country_core::render::gpu_readback::GpuReadbacks>>,
    mut rollercoaster: EventWriter<country_core::resources::events::DisplayRollercoasterPrompt>,
) {
    use country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo;
    if !screen.enabled {
        return;
    }
    update_photo_zoom(&mut camera, actions.analog[4].x);
    // Native14234b914: tag5 and its actual f32 distance, not a world ray or
    // a screen rectangle. Preserve the original default history flag.
    let canvas = ui.add("camera_focus_canvas", photo_focus_canvas_params());
    library
        .get::<DofFocusReadbackSsbo>()
        .expect("photomode requires the original DofFocusReadbackSsbo");
    // The render thread's latest completed autofocus response; before the
    // first one the buffer still holds its zero-initialized element.
    let response =
        country_core::render::gpu_readback::latest::<DofFocusReadbackSsbo>(readbacks.as_deref())
            .and_then(|samples| samples.first().copied())
            .unwrap_or_default();
    let marker = update_photo_focus(
        &mut camera,
        &mut photo,
        cursor.0,
        *window,
        canvas.dragged(),
        response,
    );
    if photo.show_focus_marker && menu.is_closed() && !matches!(camera.mode_byte(), 3 | 4) {
        draw_photo_focus_marker(&mut mei, marker);
    }
    draw_photo_walk_ghost(&mut mei, &camera, *modifiers);
    // 原 B14234bfb2..c0fe：步行穿透提示抢占本分支；没有菜单门控，且每帧
    // 发一个事件供后续引导系统消费。这里不能 take/swap 清除原写者的状态。
    if rollercoaster_prompt_visible(
        camera.mode_byte(),
        modifiers.disable_snapping(),
        country_core::systems::main_camera::PROMPT_ROLLERCOASTER
            .load(std::sync::atomic::Ordering::Relaxed),
    ) {
        rollercoaster.write(country_core::resources::events::DisplayRollercoasterPrompt);
    }
}

fn rollercoaster_prompt_visible(camera_mode: u8, ghost: bool, requested: bool) -> bool {
    camera_mode == 3 && !ghost && requested
}

#[cfg(test)]
mod rollercoaster_tail_tests {
    #[test]
    fn original_tail_preserves_all_cameras_modifier_priority_and_event_gate() {
        let cases: Vec<(u8, u8, bool, usize)> = ron::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../tests/fixtures/native/toolbar-rollercoaster-tail.ron"
        )))
        .unwrap();
        assert_eq!(cases.len(), 48);
        for (mode, modifier, requested, native_events) in cases {
            assert_eq!(
                usize::from(super::rollercoaster_prompt_visible(
                    mode,
                    country_core::systems::backend::input::mod_key::Modifiers {
                        flags: u32::from(modifier),
                    }
                    .disable_snapping(),
                    requested,
                )),
                native_events,
                "camera={mode} modifier={modifier} request={requested}",
            );
        }
    }
}

/// 原 B14234bfb2..c09c。0x1408c5680 在此读取的是 Modifiers 首字节，
/// 不是反编译器选择的同址 StructureType::is_fence 别名。
fn draw_photo_walk_ghost(
    mei: &mut crate::MeiContext,
    camera: &MainCamera,
    modifiers: country_core::systems::backend::input::mod_key::Modifiers,
) {
    if camera.mode_byte() == 3 && modifiers.disable_snapping() {
        mei.safe_frame("camera ghost", false)
            .template("other/camera_ghost");
    }
}

fn photo_focus_canvas_params() -> country_core::systems::ui::resources::widget_params::WidgetParams
{
    use country_core::systems::ui::resources::widget_params::{
        UiSounds, WidgetParams, WidgetVolumeShape,
    };
    WidgetParams::new(
        WidgetVolumeShape {
            tag: 5,
            layer: 0,
            size: glam::Vec2::new(f32::MAX, 0.),
            center: glam::Vec3::ZERO,
        },
        [0; 2],
    )
    .occlude_by_world(false)
    .sounds(UiSounds::empty())
}

fn draw_photo_focus_marker(mei: &mut crate::MeiContext, marker: PhotoFocusMarker) {
    let root = mei.new_at_pos(
        "camera focus",
        marker.position,
        Some(glam::Vec2::splat(marker.size)),
        glam::Vec2::ZERO,
        false,
    );
    let view = root.template(if marker.pinned {
        "other/camera_focus_pinned"
    } else {
        "other/camera_focus"
    });
    root.ignore_input();
    view.context.ignore_input();
}

fn update_photo_zoom(camera: &mut MainCamera, zoom: f32) {
    if zoom == 0. || matches!(camera.mode_byte(), 1 | 4) {
        return;
    }
    let lens = 0x100 + FOV_OFFSETS[camera.mode_byte() as usize];
    let lower = camera.read_f32(lens + 0xc);
    let upper = camera.read_f32(lens + 0x10);
    assert!(
        lower <= upper,
        "photomode lens clamp endpoints are unordered"
    );
    // Original exp2f then MAXSS/MINSS order; do not sanitize NaN inputs.
    let value = (zoom * f32::from_bits(0xbba3d70a)).exp2() * camera.read_f32(lens + 8);
    let value = if lower > value { lower } else { value };
    camera.write_f32(lens + 8, if upper < value { upper } else { value });
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PhotoFocusMarker {
    position: glam::Vec2,
    size: f32,
    pinned: bool,
}

/// Active union writes exactly follow14234ba27..bc8d, including preserving
/// inactive payload bytes. Raw drag phases3/4 mean start/continuation.
fn update_photo_focus(
    camera: &mut MainCamera,
    photo: &mut PhotomodeState,
    cursor: glam::Vec2,
    window: country_core::resources::window::WindowSize,
    drag: u8,
    response: country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo,
) -> PhotoFocusMarker {
    use glam::{Vec2, Vec4};
    let camera_mode = u32::from(camera.mode_byte());
    let focus = focus_offset(camera);
    let mut tag = camera.read_f32(focus).to_bits();
    photo.index = u32::MAX;
    if tag < 2 {
        if response.lock_reponse == 0 || response.lock_reponse != camera_mode {
            if tag == 0 {
                photo.index = camera_mode;
            }
        } else {
            tag = 1;
            camera.write_f32(focus, f32::from_bits(tag));
            for (index, value) in response.lock_pos_ws.into_iter().enumerate() {
                camera.write_f32(focus + 4 + index * 4, value);
            }
        }
    }
    let aspect = window.width as f32 / window.height as f32;
    let (x, y) = if tag == 2 {
        (0., 0.)
    } else {
        let x = cursor.x / window.width as f32;
        let y = cursor.y / window.height as f32;
        let x = aspect * (x + x - 1.);
        let y = -(y + y - 1.);
        if tag == 3 {
            if matches!(drag, 3 | 4) {
                camera.write_f32(focus + 4, x);
                camera.write_f32(focus + 8, y);
                (x, y)
            } else {
                (camera.read_f32(focus + 4), camera.read_f32(focus + 8))
            }
        } else if matches!(drag, 3 | 4) {
            photo.index = camera_mode;
            camera.write_f32(focus + 0x10, x);
            camera.write_f32(focus + 0x14, y);
            tag = 0;
            camera.write_f32(focus, f32::from_bits(tag));
            (x, y)
        } else if tag & 1 == 0 {
            (camera.read_f32(focus + 0x10), camera.read_f32(focus + 0x14))
        } else {
            // Match the two original SSE multiply/add sequences rather than
            // folding projection*view or using a transform that divides early.
            let mul = |matrix: glam::Mat4, point: Vec4| {
                let result = matrix.y_axis * point.y + matrix.x_axis * point.x;
                let result = matrix.z_axis * point.z + result;
                result + matrix.w_axis
            };
            let world = Vec4::new(
                camera.read_f32(focus + 4),
                camera.read_f32(focus + 8),
                camera.read_f32(focus + 0xc),
                1.,
            );
            let view = mul(camera.read_matrix(0x80), world);
            let clip = mul(camera.read_matrix(0), view);
            let projected = clip / clip.w;
            (aspect * projected.x, projected.y)
        }
    };
    PhotoFocusMarker {
        position: Vec2::new((x + aspect) * 0.5 * 1000., (1. - y) * 0.5 * 1000.),
        size: camera.read_f32(focus + 0x18) * 1000.,
        pinned: tag < 2,
    }
}

#[cfg(test)]
mod photo_focus_tests {
    use super::*;
    use country_core::{
        input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState},
        resources::{
            cursor_terrain_raycast::CursorTerrainRaycast,
            events::{EscapeMenuJustOpened, ShutdownGameCmd},
            mouse_capture::OsCursorVisibilityCmd,
            mouse_terrain_raycast::CursorPositionSS,
            read_mouse_depth_value::MouseDepthValue,
            window::{WindowFocusEvent, WindowSize},
        },
        systems::{
            brush_preview::OverwriteBrushPreview,
            camera_rig::main_camera::CameraMode,
            render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo,
            ui::{
                hover_intersection::{PickerInput, arbitrate, pick_chunk},
                resources::{
                    ui_state::{UiState, WidgetId},
                    widget_params::PlayUiSound,
                },
                ui_update::ui_update,
            },
        },
    };
    use glam::{Mat4, Vec2};

    /// Original release UI asset (`B版本内容/assets/ui/<relative>`), resolved from
    /// the manifest dir. Not part of the repository.
    fn original_ui_asset(relative: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../B版本内容/assets/ui")
            .join(relative)
    }

    fn camera(tag: u32) -> MainCamera {
        let mut camera = MainCamera::new(1.);
        camera.set_mode_byte(CameraMode::FREE);
        for offset in [0, 0x40, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        let focus = focus_offset(&camera);
        camera.write_f32(focus, f32::from_bits(tag));
        for (offset, value) in [
            (4, 7.),
            (8, 8.),
            (12, 9.),
            (16, 0.25),
            (20, -0.25),
            (24, 0.25),
        ] {
            camera.write_f32(focus + offset, value);
        }
        camera
    }

    fn setup_picker() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(camera(0));
        world.insert_resource(UiState::at_startup());
        world.insert_resource(PhotomodeState::default());
        world.insert_resource(ActionSetStates::default());
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(CursorPositionSS(Vec2::new(75., 25.)));
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(MouseDepthValue::default());
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        let mut schedule = Schedule::default();
        // Exercise the exact production picker/arbitration algorithms without
        // depending on an unrelated test having initialized its task pool.
        schedule.add_systems(
            (
                |mut ui: ResMut<UiState>,
                 camera: Res<MainCamera>,
                 cursor: Res<CursorPositionSS>,
                 window: Res<WindowSize>| {
                    let ray = utils::geometry::Ray::from_mouse_ss(
                        cursor.0,
                        window.as_vec2(),
                        camera.read_matrix(0x40),
                        camera.read_matrix(0xc0),
                    );
                    let (hits, canvases) = pick_chunk(
                        &ui.registry_snapshot(),
                        PickerInput {
                            ray,
                            screen: cursor.0,
                            terrain: None,
                        },
                    );
                    let hits = arbitrate(hits, canvases, ray, camera.read_matrix(0x80), 0.);
                    ui.set_hover_intersections(
                        hits.into_iter().map(|hit| hit.to_native()).collect(),
                    );
                },
                ui_update,
            )
                .chain_ignore_deferred(),
        );
        (world, schedule)
    }

    fn frame(
        world: &mut World,
        schedule: &mut Schedule,
        phase: DigitalActionPhase,
        response: DofFocusReadbackSsbo,
    ) -> (u8, PhotoFocusMarker) {
        world
            .resource::<UiState>()
            .add("camera_focus_canvas", photo_focus_canvas_params());
        world.resource_mut::<ActionSetStates>().digital[0] = DigitalActionState {
            phase,
            timer_seconds: 0.,
        };
        schedule.run(world);
        let drag = world
            .resource::<UiState>()
            .widget_context(WidgetId::from_name("camera_focus_canvas"))
            .dragged();
        let cursor = world.resource::<CursorPositionSS>().0;
        let window = *world.resource::<WindowSize>();
        let marker = world.resource_scope(|world, mut photo: Mut<PhotomodeState>| {
            update_photo_focus(
                &mut world.resource_mut::<MainCamera>(),
                &mut photo,
                cursor,
                window,
                drag,
                response,
            )
        });
        (drag, marker)
    }

    #[test]
    fn real_focus_canvas_picker_press_hold_release_and_gpu_lock_response() {
        let (mut world, mut schedule) = setup_picker();
        let none = DofFocusReadbackSsbo::default();
        frame(&mut world, &mut schedule, DigitalActionPhase::Idle, none);
        let before = world.resource::<MainCamera>().bytes().to_vec();
        let (drag, marker) = frame(
            &mut world,
            &mut schedule,
            DigitalActionPhase::JustPressed,
            none,
        );
        assert_eq!(drag, 3);
        assert_eq!(
            world.resource::<UiState>().active_widget(),
            WidgetId::from_name("camera_focus_canvas")
        );
        assert_eq!(
            marker,
            PhotoFocusMarker {
                position: Vec2::new(750., 250.),
                size: 250.,
                pinned: true
            }
        );
        assert_eq!(world.resource::<PhotomodeState>().index, 2);
        let camera = world.resource::<MainCamera>();
        let offset = focus_offset(camera);
        assert_eq!(camera.read_f32(offset).to_bits(), 0);
        assert_eq!(
            (camera.read_f32(offset + 16), camera.read_f32(offset + 20)),
            (0.5, 0.5)
        );
        for (index, (&old, &new)) in before.iter().zip(camera.bytes()).enumerate() {
            if !(offset..offset + 4).contains(&index)
                && !(offset + 16..offset + 24).contains(&index)
            {
                assert_eq!(old, new, "inactive camera payload changed at{index:x}");
            }
        }
        assert_eq!(
            frame(&mut world, &mut schedule, DigitalActionPhase::Held, none).0,
            4
        );
        assert_eq!(
            frame(
                &mut world,
                &mut schedule,
                DigitalActionPhase::JustReleased,
                none
            )
            .0,
            1
        );
        let wrong = DofFocusReadbackSsbo {
            lock_pos_ws: [10., 20., 30.],
            lock_reponse: 3,
        };
        frame(&mut world, &mut schedule, DigitalActionPhase::Idle, wrong);
        assert_eq!(world.resource::<MainCamera>().read_f32(offset).to_bits(), 0);
        assert_eq!(world.resource::<PhotomodeState>().index, 2);
        let response = DofFocusReadbackSsbo {
            lock_pos_ws: [0.25, -0.5, 2.],
            lock_reponse: 2,
        };
        let (_, marker) = frame(
            &mut world,
            &mut schedule,
            DigitalActionPhase::Idle,
            response,
        );
        assert_eq!(world.resource::<PhotomodeState>().index, u32::MAX);
        let camera = world.resource::<MainCamera>();
        assert_eq!(camera.read_f32(offset).to_bits(), 1);
        assert_eq!(
            [
                camera.read_f32(offset + 4),
                camera.read_f32(offset + 8),
                camera.read_f32(offset + 12)
            ],
            response.lock_pos_ws
        );
        assert_eq!(marker.position, Vec2::new(625., 750.));
    }

    #[test]
    fn all_camera_focus_unions_manual_cursor_and_marker_use_original_templates() {
        use crate::mei::{
            core::{FrameKind, GlobalId},
            widgets::FrameContent,
        };
        for mode in 0..6 {
            for tag in 0..4 {
                for drag in 0..6 {
                    let mut camera = camera(tag);
                    camera.set_mode_byte(CameraMode(mode));
                    let offset = focus_offset(&camera);
                    camera.write_f32(offset, f32::from_bits(tag));
                    camera.write_f32(offset + 4, 0.25);
                    camera.write_f32(offset + 8, -0.25);
                    camera.write_f32(offset + 12, 0.);
                    camera.write_f32(offset + 16, -0.5);
                    camera.write_f32(offset + 20, 0.5);
                    camera.write_f32(offset + 24, 0.25);
                    let mut photo = PhotomodeState::default();
                    let marker = update_photo_focus(
                        &mut camera,
                        &mut photo,
                        Vec2::new(150., 25.),
                        WindowSize::new(200, 100),
                        drag,
                        DofFocusReadbackSsbo::default(),
                    );
                    if tag == 2 {
                        assert_eq!(marker.position, Vec2::new(1000., 500.));
                        assert_eq!(camera.read_f32(offset).to_bits(), 2);
                    } else if matches!(drag, 3 | 4) {
                        assert_eq!(marker.position, Vec2::new(1500., 250.));
                        let target = if tag == 3 { offset + 4 } else { offset + 16 };
                        assert_eq!(
                            (camera.read_f32(target), camera.read_f32(target + 4)),
                            (1., 0.5)
                        );
                        assert_eq!(
                            camera.read_f32(offset).to_bits(),
                            if tag == 3 { 3 } else { 0 }
                        );
                    } else {
                        let expected = match tag {
                            0 => Vec2::new(750., 250.),
                            1 => Vec2::new(1250., 625.),
                            _ => Vec2::new(1125., 625.),
                        };
                        assert_eq!(marker.position, expected);
                    }
                }
            }
        }
        for pinned in [false, true] {
            let mut mei = crate::MeiContext::default();
            // Read at run time, like the other original-asset tests in this
            // module: the original release files are not in the repository, so a
            // compile-time include_str! would break `cargo check --all-targets`
            // on every machine without them. A missing file still fails this test
            // loudly instead of skipping it.
            let (template, config): (&str, crate::mei::config::NodeConfig) = if pinned {
                (
                    "other/camera_focus_pinned",
                    crate::mei::config::read_node_config_from_file(&original_ui_asset(
                        "other/camera_focus_pinned.ron",
                    ))
                    .unwrap(),
                )
            } else {
                (
                    "other/camera_focus",
                    crate::mei::config::read_node_config_from_file(&original_ui_asset(
                        "other/camera_focus.ron",
                    ))
                    .unwrap(),
                )
            };
            let shapes = config.own.draw.clone();
            mei.configs
                .insert(template.into(), std::sync::Arc::new(config));
            let marker = PhotoFocusMarker {
                position: Vec2::new(1250., 625.),
                size: 250.,
                pinned,
            };
            draw_photo_focus_marker(&mut mei, marker);
            let root = GlobalId::new("camera focus");
            assert_eq!(mei.frame_events[0].id, root);
            assert_eq!(
                mei.frame_events[0].kind,
                FrameKind::AtPos {
                    position: marker.position,
                    requested_size: Some(Vec2::splat(250.)),
                    anchor: Vec2::ZERO
                }
            );
            assert_eq!(mei.nodes[&root].flags & 2, 2);
            // 原版在 post_update 布局遍历时将输入屏蔽传播到模板后代；
            // 建树阶段的 TemplateView.context 仍指向调用者，不能提前检查子节点。
            mei.perform_layout(Vec2::new(2000., 1000.));
            let frames: Vec<_> = mei
                .nodes
                .values()
                .filter_map(|node| {
                    let content: &dyn std::any::Any = node.content.as_ref();
                    content
                        .downcast_ref::<FrameContent>()
                        .map(|frame| (node, frame))
                })
                .collect();
            assert!(
                frames
                    .iter()
                    .any(|(node, frame)| frame.shapes == shapes && node.flags & 2 != 0)
            );
            assert_eq!(mei.z_order.len(), 2);
            for id in &mei.z_order {
                let node = &mei.nodes[id];
                assert_eq!(node.flags & 2, 2);
                assert_eq!(node.size, Vec2::splat(marker.size));
                assert_eq!(node.pos, marker.position - Vec2::splat(marker.size * 0.5));
            }
        }
    }

    #[test]
    fn walk_ghost_uses_real_modifier_byte_and_original_installed_template() {
        use crate::mei::{
            core::{FrameKind, GlobalId},
            widgets::LabelContent,
        };
        use country_core::systems::backend::input::mod_key::Modifiers;
        for mode in 0..6 {
            for flags in [0, 1, 0x100, 0x101] {
                let mut camera = camera(0);
                camera.set_mode_byte(CameraMode(mode));
                let mut mei = crate::MeiContext::default();
                mei.font_table = std::sync::Arc::new(
                    tg_text::FontTable::from_ron(include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/fixtures/welcome/font-table.ron"
                    )))
                    .unwrap(),
                );
                let path = original_ui_asset("other/camera_ghost.ron");
                let config = crate::mei::config::read_node_config_from_file(&path).unwrap();
                mei.configs
                    .insert("other/camera_ghost".into(), std::sync::Arc::new(config));
                draw_photo_walk_ghost(&mut mei, &camera, Modifiers { flags });
                if mode == 3 && flags & 0xff != 0 {
                    assert_eq!(mei.frame_events.len(), 1);
                    assert_eq!(mei.frame_events[0].id, GlobalId::new("camera ghost"));
                    assert_eq!(mei.frame_events[0].kind, FrameKind::Safe);
                    assert_eq!(mei.nodes[&GlobalId::new("camera ghost")].flags & 1, 0);
                    assert!(mei.nodes.values().any(|node| {
                        let content: &dyn std::any::Any = node.content.as_ref();
                        content
                            .downcast_ref::<LabelContent>()
                            .is_some_and(|label| label.pending_text.as_deref() == Some("👻"))
                    }));
                } else {
                    assert!(mei.frame_events.is_empty());
                    assert!(mei.nodes.is_empty());
                }
            }
        }
    }

    #[test]
    fn photo_zoom_preserves_original_camera_gates_clamp_and_nan() {
        for mode in 0..6 {
            let mut camera = camera(0);
            camera.set_mode_byte(CameraMode(mode));
            let lens = 0x100 + FOV_OFFSETS[mode as usize];
            camera.write_f32(lens + 8, 20.);
            camera.write_f32(lens + 12, 1.);
            camera.write_f32(lens + 16, 40.);
            update_photo_zoom(&mut camera, 200.);
            if matches!(mode, 1 | 4) {
                assert_eq!(camera.read_f32(lens + 8), 20.);
            } else {
                assert_eq!(camera.read_f32(lens + 8), 10.);
                update_photo_zoom(&mut camera, f32::INFINITY);
                assert_eq!(camera.read_f32(lens + 8), 1.);
                update_photo_zoom(&mut camera, f32::NEG_INFINITY);
                assert_eq!(camera.read_f32(lens + 8), 40.);
                update_photo_zoom(&mut camera, f32::NAN);
                assert!(camera.read_f32(lens + 8).is_nan());
            }
        }
    }
}

/// Camera-only entry retained from the previous round (same real consumers).
pub fn photomode_camera_buttons(
    mode: Res<AppMode>,
    mut query: Query<(
        &PhotomodeButton,
        ComponentWrite<country_core::components::UiInteractionState>,
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
            ));
        }
        7 => slider_math::focus_parameter(camera.read_f32(focus_offset(camera) + 0x18)),
        8 => {
            return Some(slider_math::normalized_lens(
                camera.read_f32(lens + 0x1c),
                camera.read_f32(lens + 0x20),
                camera.read_f32(lens + 0x18),
            ));
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

/// B14234ca52..ce42: update only the active union payload. Native inactive
/// slots receive unspecified XMM lanes and carry no business state.
fn cycle_focus_target(camera: &mut MainCamera) {
    let focus = focus_offset(camera);
    let tag = u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap());
    let next = match tag {
        2 => 2,
        3 => {
            let position =
                u64::from_le_bytes(camera.bytes()[focus + 4..focus + 12].try_into().unwrap());
            camera.write_u64(focus + 0x10, position);
            0
        }
        _ => {
            let position = u64::from_le_bytes(
                camera.bytes()[focus + 0x10..focus + 0x18]
                    .try_into()
                    .unwrap(),
            );
            camera.write_u64(focus + 4, position);
            3
        }
    };
    camera.write_f32(focus, f32::from_bits(next));
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

    fn button_world(tag: u8, platform: u8) -> (World, Schedule, Entity) {
        let mut world = World::new();
        world.insert_resource(AppMode::Photomode);
        world.insert_resource(RuntimePlatform(platform));
        world.init_resource::<PhotomodeState>();
        world.init_resource::<PhotomodeScreen>();
        world.init_resource::<DebugGui>();
        world.init_resource::<SteamdeckScreenshotMsgScreen>();
        world.init_resource::<crate::MeiContext>();
        world.init_resource::<GameConfig>();
        world.init_resource::<NextEscMenuState>();
        world.init_resource::<VermeerState>();
        world.init_resource::<ImageFormationConfigs>();
        world.insert_resource(MainCamera::new(1.));
        // An invalid parent exercises the real creation failure branch without
        // starting an external file browser from a unit test.
        world.insert_resource(PhotomodeScreenshotFolder(
            "/dev/null/toolbar-screenshots".into(),
        ));
        world.init_resource::<Events<UiClickEvent>>();
        world.init_resource::<Events<PhotoModeCmd>>();
        world.init_resource::<Events<OsWindowCmd>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        let entity = world
            .spawn((PhotomodeButton { tag }, UiInteractionState::default()))
            .id();
        world.send_event(UiClickEvent { entity });
        let mut schedule = Schedule::default();
        schedule.add_systems(photomode_buttons);
        (world, schedule, entity)
    }

    #[test]
    fn real_photo_dispatch_uses_camera_commands_including_screenshot_four() {
        for (tag, expected) in [(0, 0), (1, 1), (2, 2), (3, 3), (5, 1), (9, 4), (6, 6)] {
            let (mut world, mut schedule, entity) = button_world(tag, 2);
            schedule.run(&mut world);
            world.flush();
            let events = world.resource::<Events<PhotoModeCmd>>();
            assert_eq!(
                events
                    .get_cursor()
                    .read(events)
                    .map(|cmd| cmd.0)
                    .collect::<Vec<_>>(),
                [expected]
            );
            assert!(world.resource::<Events<OsWindowCmd>>().is_empty());
            assert_eq!(
                world.resource::<PhotomodeState>().camera_mode as u8,
                0,
                "camera authority changes only in the production command consumer"
            );
            assert!(world.get::<UiPlaySelectAnimation>(entity).is_some());
        }
        // Tag6 has no producer in either native toolbar tree. This checks its
        // original forwarding only, not a nonexistent camera interpretation.
    }

    #[test]
    fn real_pin_dispatch_cycles_active_focus_payload_for_all_camera_bodies() {
        for mode in 0..6 {
            for tag in [0, 1, 2, 3] {
                let (mut world, mut schedule, _) = button_world(16, 2);
                let mut camera = world.resource_mut::<MainCamera>();
                camera.set_mode_byte(CameraMode(mode));
                let offset = focus_offset(&camera);
                let pinned = 0x40a0_0000_4020_0000;
                let tracked = 0xc0e0_0000_4040_0000;
                camera.write_f32(offset, f32::from_bits(tag));
                camera.write_u64(offset + 4, pinned);
                camera.write_u64(offset + 0x10, tracked);
                drop(camera);
                schedule.run(&mut world);
                let camera = world.resource::<MainCamera>();
                let actual =
                    u32::from_le_bytes(camera.bytes()[offset..offset + 4].try_into().unwrap());
                assert_eq!(
                    actual,
                    match tag {
                        2 => 2,
                        3 => 0,
                        _ => 3,
                    }
                );
                let (payload_offset, expected) = match tag {
                    2 => (offset + 4, pinned),
                    3 => (offset + 0x10, pinned),
                    _ => (offset + 4, tracked),
                };
                assert_eq!(
                    u64::from_le_bytes(
                        camera.bytes()[payload_offset..payload_offset + 8]
                            .try_into()
                            .unwrap()
                    ),
                    expected
                );
                assert!(world.resource::<Events<PhotoModeCmd>>().is_empty());
            }
        }
    }

    #[test]
    fn real_folder_dispatch_uses_platform_message_or_os_command() {
        for platform in [0, 1, 2] {
            let (mut world, mut schedule, entity) = button_world(4, platform);
            schedule.run(&mut world);
            world.flush();
            assert_eq!(
                world.resource::<SteamdeckScreenshotMsgScreen>().0,
                platform < 2
            );
            let events = world.resource::<Events<OsWindowCmd>>();
            assert_eq!(events.len(), usize::from(platform == 2));
            assert!(world.resource::<Events<PhotoModeCmd>>().is_empty());
            assert!(world.get::<UiPlaySelectAnimation>(entity).is_some());
        }
    }

    #[test]
    fn screenshot_notice_uses_installed_template_and_real_confirm_input() {
        use country_core::{
            input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState},
            resources::{mouse_terrain_raycast::CursorPositionSS, window::WindowSize},
            systems::{
                audio::PlayMenuUiSound,
                ui::resources::{ui_state::UiState, widget_params::UiNavState},
            },
        };
        for platform in [0, 1] {
            let (mut world, mut buttons, _) = button_world(4, platform);
            let mut mei = crate::MeiContext::default();
            mei.font_table = std::sync::Arc::new(
                tg_text::FontTable::from_ron(include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/fixtures/welcome/font-table.ron"
                )))
                .unwrap(),
            );
            let template = if platform == 0 {
                "steamdeck/screenshot_msg"
            } else {
                "gfn/screenshot_msg"
            };
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../../B版本内容/assets/ui")
                .join(format!("{template}.ron"));
            let config = crate::mei::config::read_node_config_from_file(&path).unwrap();
            mei.configs
                .insert(template.into(), std::sync::Arc::new(config));
            world.insert_resource(mei);
            world.insert_resource(WindowSize::new(960, 540));
            world.insert_resource(CursorPositionSS(glam::Vec2::splat(-100.)));
            world.insert_resource(UiState::at_startup());
            world.insert_resource(bevy_time::Time::<()>::default());
            world.init_resource::<ActionSetStates>();
            world.init_resource::<UiNavState>();
            world.init_resource::<Events<bevy_input::mouse::MouseWheel>>();
            world.init_resource::<Events<PlayMenuUiSound>>();
            let mut pre = Schedule::default();
            pre.add_systems(crate::mei::systems::mei_pre_update);
            // First real folder click sets the owner; the next frame builds
            // the native template. No fabricated node activation is injected.
            pre.run(&mut world);
            buttons.run(&mut world);
            world.resource_mut::<Events<UiClickEvent>>().clear();
            pre.run(&mut world);
            buttons.run(&mut world);
            assert!(world.resource::<SteamdeckScreenshotMsgScreen>().0);
            assert!(
                world
                    .resource::<crate::MeiContext>()
                    .nodes
                    .values()
                    .any(|node| (&*node.content as &dyn std::any::Any)
                        .is::<crate::mei::widgets::MenuButtonContent>())
            );
            world.resource_mut::<UiNavState>().hold = true;
            world.resource_mut::<ActionSetStates>().digital[19] = DigitalActionState {
                phase: DigitalActionPhase::JustPressed,
                timer_seconds: 0.,
            };
            pre.run(&mut world);
            buttons.run(&mut world);
            assert!(
                !world.resource::<SteamdeckScreenshotMsgScreen>().0,
                "real menu hotkey must close the platform notice"
            );
        }
    }

    #[test]
    fn real_lut_button_requests_capture_pending_menu_without_skipping_wait() {
        let (mut world, mut schedule, entity) = button_world(11, 2);
        schedule.run(&mut world);
        world.flush();
        assert!(
            matches!(
                world.resource::<NextEscMenuState>().0,
                Some(EscMenuState::Photomode(false))
            ),
            "native tag10/flag0 must reach screenshot gather before the gallery can open"
        );
        assert!(world.resource::<Events<PhotoModeCmd>>().is_empty());
        assert!(world.get::<UiPlaySelectAnimation>(entity).is_some());
    }

    #[test]
    fn real_back_button_resets_original_camera_owners_without_clearing_photo_state() {
        let (mut world, mut schedule, _) = button_world(17, 2);
        world.resource_mut::<DebugGui>().camera_mode = CameraMode::FREE;
        world.resource_mut::<PhotomodeScreen>().enabled = true;
        let mut photo = world.resource_mut::<PhotomodeState>();
        photo.hide_ui = true;
        photo.walk_camera_initialized = true;
        let before = *photo;
        drop(photo);
        schedule.run(&mut world);
        assert_eq!(world.resource::<DebugGui>().camera_mode, CameraMode::NORMAL);
        assert!(!world.resource::<PhotomodeScreen>().enabled);
        assert_eq!(*world.resource::<PhotomodeState>(), before);
        let events = world.resource::<Events<RequestAppModeChange>>();
        assert_eq!(
            events
                .get_cursor()
                .read(events)
                .map(|event| event.0)
                .collect::<Vec<_>>(),
            [AppMode::default()]
        );
    }

    #[test]
    fn real_toggle_buttons_update_all_original_selection_consumers() {
        for tag in [12, 13, 14, 16] {
            let (mut world, mut schedule, entity) = button_world(tag, 2);
            world.resource_mut::<VermeerState>().deactivate();
            let mut formation = world.resource_mut::<ImageFormationConfigs>();
            formation.switch_kind(2);
            for row in formation.iter_non_default_mut() {
                row.flag_3c = 0;
                row.flag_3d = 0;
            }
            drop(formation);
            let mut camera = world.resource_mut::<MainCamera>();
            let focus = focus_offset(&camera);
            camera.write_f32(focus, f32::from_bits(3));
            drop(camera);
            schedule.run(&mut world);
            assert_eq!(
                world.get::<UiInteractionState>(entity).unwrap().selected,
                1,
                "each real toggle's authoritative state must publish its selected byte"
            );
            world.resource_mut::<Events<UiClickEvent>>().clear();
            world.send_event(UiClickEvent { entity });
            schedule.run(&mut world);
            assert_eq!(world.get::<UiInteractionState>(entity).unwrap().selected, 0);
        }
    }

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

use country_core::render::extract::ComponentWrite;
