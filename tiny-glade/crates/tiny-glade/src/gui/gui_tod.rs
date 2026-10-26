//! B time-of-day panel, owned by the original application GUI module.
//!
//! Sources: do_time_of_day_screen1405cc7f0, circle closure140505780,
//! passage-of-time closure1405068c0 and TodCircleContent1405ccb50/1405ccb80.
//! The existing MEI host owns drawing/input and the existing TOD system owns
//! authority, lighting and audio. This producer writes their canonical owners.
use bevy_ecs::prelude::*;
use country_core::{
    game_config::GameConfig,
    input::action_set::{ActionSetAction, ActionSetStates},
    resources::{
        gui::{EscMenuState, TimeOfDayScreen},
        mouse_terrain_raycast::CursorPositionSS,
        time::GameTime,
        time_of_day::TimeOfDay,
        window::WindowSize,
    },
    systems::{
        mode_manager::{AppMode, HistoryCommand},
        save::save_game::SaveGameCmd,
        ui::resources::widget_params::PlayUiSound,
    },
};
use glam::Vec2;
use std::{f32::consts::{PI, TAU}, sync::Arc};
use toolbar::mei::{
    core::{calculate_hash, GlobalId, NodeContent, NodeContext, NodePrepareContext, GLOBAL_ID_MASK},
    layout::{LayoutStyle, LengthPercentage},
    widgets::{DefaultNodeContent, ImageContent},
};
use utils::easing_functions::EasingFunction;
use utils::scalar::clamp01;

/// Native Local<TodButtonState>, with explicit session ownership so unload can
/// discard hover/pulse and previous sound buckets without changing normal frames.
#[derive(Resource, Default, Debug, PartialEq)]
pub struct TodButtonState {
    hover: [f32; 2],
    previous_target_angle: f32,
    previous_target_time: f32,
    previous_actual_angle: f32,
    previous_actual_time: f32,
    button_hover: f32,
    pulse: f32,
    pulsing: bool,
}
impl TodButtonState {
    pub fn clear_session(&mut self) { *self = Self::default(); }
}

#[derive(Default)]
pub struct TodCircleContent {
    delta: f32,
    frame_delta: f32,
    confirmed: bool,
}
impl DefaultNodeContent for TodCircleContent {
    fn from_font_table(_: Arc<tg_text::FontTable>) -> Self { Self::default() }
}
impl NodeContent for TodCircleContent {
    fn type_name(&self) -> &'static str { "tiny_glade::gui::gui_tod::TodCircleContent" }
    // B's interactive vtable slot aliases the empty false implementation. The
    // surrounding real nodes deliberately supply block_input/with_nav.
    fn prepare_layout(&mut self, ctx: &mut NodePrepareContext<'_>) -> Option<[LengthPercentage; 2]> {
        self.delta = 0.;
        self.confirmed = false;
        // MEI's historical `elapsed` spelling carries Time::delta_secs(), as
        // verified at mei_pre_update14068459f and its current real producer.
        self.frame_delta = ctx.elapsed;
        None
    }
    fn on_input_phase(&mut self, action: &ActionSetAction, phase: [bool; 2]) -> bool {
        if !phase[0] { return false; }
        match *action {
            ActionSetAction::Digital { slot: 19, state } if state.just_pressed() => {
                self.confirmed = true;
            }
            ActionSetAction::Digital { slot: 20, state } if state.pressed() => {
                self.delta -= self.frame_delta;
            }
            ActionSetAction::Digital { slot: 21, state } if state.pressed() => {
                self.delta += self.frame_delta;
            }
            _ => return false,
        }
        true
    }
}

fn fit(value: f32, start: f32, end: f32, lower: f32, upper: f32) -> f32 {
    (value - start) / (end - start) * (upper - lower) + lower
}

/// The original three source intervals are inclusive and tested in order.
/// Authored B constants are retained as bits rather than rounded decimal fits.
pub fn remap_tod_to_angle(value: f32) -> Option<f32> {
    let value = value.rem_euclid(1.);
    let dawn = f32::from_bits(0x3f0147ae);
    let dusk = f32::from_bits(0x3f547ae1);
    let initial = f32::from_bits(0x3f4a8d23);
    let final_angle = f32::from_bits(0x40e2617f);
    let angle = if (0. ..=dawn).contains(&value) {
        fit(value, 0., dawn, initial, PI)
    } else if (dawn..=dusk).contains(&value) {
        fit(value, dawn, dusk, PI, TAU)
    } else if (dusk..=1.).contains(&value) {
        fit(value, dusk, 1., TAU, final_angle)
    } else { return None; };
    Some(angle.rem_euclid(TAU))
}
pub fn remap_angle_to_tod(angle: f32) -> Option<f32> {
    let middle = f32::from_bits(0x3f2ccccd);
    let value = if (0. ..=PI).contains(&angle) {
        fit(angle, 0., PI, 0., middle)
    } else if (PI..=TAU).contains(&angle) {
        fit(angle, PI, TAU, middle, 1.)
    } else { return None; };
    Some((value + f32::from_bits(0xbe2e147c)).rem_euclid(1.))
}
fn parabola(value: f32) -> f32 {
    let pulse = value * 4. * (1. - value);
    pulse * pulse
}

/// Native typed-child hashing retains the source's u64 content identities.
fn typed_child(parent: NodeContext, value: u64, type_high: u64) -> NodeContext {
    let local = calculate_hash(&(value, type_high));
    parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, local))))
}
pub fn tod_circle(parent: NodeContext, id: u64) -> NodeContext {
    let node = typed_child(parent, id, 0x96b64fdf1b70bc63);
    node.get_content_mut_impl::<TodCircleContent>(false);
    node.layout(LayoutStyle::default().expand());
    node
}
fn image(parent: NodeContext, id: u64, source: &str, height: f32) -> NodeContext {
    let node = typed_child(parent, id, 0x01f6d60c8858b8ff);
    let content = node.get_content_mut_impl::<ImageContent>(false);
    content.image = source.into();
    content.height = height;
    content.fit_parent_height = false;
    node
}
fn sound(events: &mut EventWriter<PlayUiSound>, kind: u32) {
    events.write(PlayUiSound { kind, _payload: [0; 20] });
}
fn direction(angle: f32) -> Vec2 { Vec2::new(angle.cos(), angle.sin()) }
fn target_angle(screen: &TimeOfDayScreen, circle: usize) -> f32 {
    if circle == 0 {
        remap_tod_to_angle(screen.time_of_day).expect("native TOD target angle unwrap") - PI
    } else { screen.sun_xz_rotation }
}
fn actual_angle(tod: &TimeOfDay, circle: usize) -> f32 {
    if circle == 0 {
        remap_tod_to_angle(tod.time_of_day).expect("native TOD actual angle unwrap") - PI
    } else { TAU - tod.sun_xz_rotation }
}

#[allow(clippy::too_many_arguments)]
fn draw_circle(
    mei: &mut toolbar::MeiContext, parent: NodeContext, circle: usize,
    screen: &mut TimeOfDayScreen, state: &mut TodButtonState, tod: &TimeOfDay,
    cursor: Vec2, scale: f32, delta: f32, sounds: &mut EventWriter<PlayUiSound>,
    confirm: &mut bool,
) {
    parent.node_mut().block_input();
    let control = tod_circle(parent, 0);
    control.with_nav(true);
    let focused = control.node().has_focus();
    let content = control.get_content_impl::<TodCircleContent>();
    if circle == 0 && content.confirmed { *confirm = true; }
    let increment = content.delta * 0.25 * TAU;
    if increment != 0. {
        if circle == 0 {
            // The native fallback for a nonfinite remap is zero in this input
            // prefix; the later visual path deliberately unwraps the target.
            let angle = remap_tod_to_angle(screen.time_of_day).unwrap_or(0.);
            screen.time_of_day = remap_angle_to_tod((increment + angle).rem_euclid(TAU)).unwrap_or(0.);
            screen.time_input_active = true;
        } else {
            screen.sun_xz_rotation = (increment + screen.sun_xz_rotation).rem_euclid(TAU);
            screen.angle_input_active = true;
        }
    }
    let background = image(control, calculate_hash(&(circle as u64, ())),
        if circle == 0 { "menu/tod/circle_tod" } else { "menu/tod/circle_angle" }, 170.);
    if circle == 1 { background.node_mut().default_focus(); }
    let center = control.node().prev_computed_layout().center();
    let offset = direction(target_angle(screen, circle)) * 85. * 0.93;
    let root = mei.new_relative_with_id(
        GlobalId(calculate_hash(&(1i32, circle as u64)) ^ GLOBAL_ID_MASK),
        control.id, None, offset, Vec2::ZERO, Vec2::ZERO, false,
    );
    let knob = typed_child(root, 0, 0xa79b7268a2a968d9);
    knob.node_mut().local_z = 200.;
    knob.node_mut().force_interactive();
    knob.node_mut().set_sounds(0, 13);
    let (target, previous) = if circle == 0 {
        (screen.time_of_day, &mut state.previous_target_time)
    } else {
        (screen.sun_xz_rotation / TAU, &mut state.previous_target_angle)
    };
    let previous_normalized = if circle == 0 { *previous } else { *previous / TAU };
    if (target * 30.) as u64 != (previous_normalized * 30.) as u64 { sound(sounds, 30); }
    *previous = if circle == 0 { screen.time_of_day } else { screen.sun_xz_rotation };
    let interaction = knob.interaction();
    if interaction & 2 != 0 { sound(sounds, 26); }
    if interaction & 8 != 0 { sound(sounds, 27); }
    if screen.time_input_active || screen.angle_input_active {
        let (old, current) = if circle == 0 {
            (state.previous_actual_time, tod.time_of_day)
        } else { (state.previous_actual_angle / TAU, tod.sun_xz_rotation / TAU) };
        if (old * 30.) as u64 != (current * 30.) as u64 { sound(sounds, 31); }
    }
    if circle == 0 { state.previous_actual_time = tod.time_of_day; }
    else { state.previous_actual_angle = tod.sun_xz_rotation; }
    if interaction & 4 != 0 {
        // Preserve the original normalization singularity at the exact center.
        let relative = cursor - center * scale;
        let inverse_length = 1. / (relative.y * relative.y + relative.x * relative.x).sqrt();
        let angle = (inverse_length * relative.y).atan2(relative.x * inverse_length) + PI;
        if circle == 0 {
            screen.time_of_day = remap_angle_to_tod(angle).unwrap_or(0.);
            screen.time_input_active = true;
        } else {
            screen.sun_xz_rotation = (angle + PI) % TAU;
            screen.angle_input_active = true;
        }
    }
    let step = delta / 0.15;
    state.hover[circle] = clamp01(state.hover[circle] + if knob.hover() || focused { step } else { -step });
    let eased = EasingFunction::CubicOut.apply(state.hover[circle]);
    let size = (eased * 50. + (1. - eased) * 40.) * 0.6;
    knob.image("sun", "menu/tod/outer_ring", size).ignore_input();
    let actual = direction(actual_angle(tod, circle)) * 85. * 0.93;
    let real_root = mei.new_relative_with_id(
        GlobalId(calculate_hash(&(2i32, circle as u64)) ^ GLOBAL_ID_MASK),
        control.id, None, actual, Vec2::ZERO, Vec2::ZERO, false,
    );
    let marker = typed_child(real_root, 0, 0xa79b7268a2a968d9);
    marker.node_mut().local_z = 250.;
    marker.image("sun", "menu/tod/inner_circle", size).ignore_input();
}

fn time_button(
    mei: &mut toolbar::MeiContext, parent: NodeContext, state: &mut TodButtonState,
    config: &mut GameConfig, delta: f32, confirm: bool, sounds: &mut EventWriter<PlayUiSound>,
) {
    parent.node_mut().set_sounds(0, 13);
    parent.node_mut().block_input();
    let hovered = parent.hover();
    if hovered {
        let root = mei.new_relative_with_id(
            GlobalId(0x9e0c82f5u64 ^ GLOBAL_ID_MASK),
            parent.id, None, Vec2::ZERO, Vec2::new(0., 1.), Vec2::ZERO, false,
        );
        let hint = typed_child(root, 0, 0xa79b7268a2a968d9);
        hint.node_mut().local_z = 500.;
        hint.template("menu/tod_time_passage_hint");
    }
    state.button_hover = clamp01(state.button_hover + if hovered { delta / 0.15 } else { delta / -0.15 });
    if confirm || parent.node().used() {
        state.pulsing = true;
        config.dev.day_night_cycle ^= true;
        sound(sounds, if config.dev.day_night_cycle { 33 } else { 32 });
    }
    if state.pulsing { state.pulse = clamp01(state.pulse + delta / 0.2); }
    let ease = EasingFunction::CubicOut.apply(state.button_hover);
    let shifted = (1. - state.pulse) * 0.1 + state.pulse;
    let pulse = parabola(shifted);
    let height = (pulse * 10. + ease * 70. + (1. - ease) * 60.) * 0.75;
    parent.image("tod_button_img", if config.dev.day_night_cycle {
        "menu/tod/unlocked"
    } else { "menu/tod/locked" }, height).ignore_input();
    if state.pulse >= 1. { state.pulse = 0.; state.pulsing = false; }
}

/// Original fourteen-parameter producer. Run at the original GUI stage after
/// TOD authority and before the shared set44 MEI finalization; no fake picker,
/// screen resource or lighting writer is introduced here.
#[allow(clippy::too_many_arguments)]
pub fn do_time_of_day_screen(
    mut mei: ResMut<toolbar::MeiContext>, mut screen: ResMut<TimeOfDayScreen>,
    mut saves: EventWriter<SaveGameCmd>, mut history: EventWriter<HistoryCommand>,
    mut config: ResMut<GameConfig>, mut state: ResMut<TodButtonState>,
    mode: Res<AppMode>, menu: Res<EscMenuState>, window: Res<WindowSize>,
    cursor: Res<CursorPositionSS>, actions: Res<ActionSetStates>, time: Res<GameTime>,
    tod: Res<TimeOfDay>, mut sounds: EventWriter<PlayUiSound>,
) {
    if menu.is_open() || actions.digital[18].just_pressed() { screen.visible = false; }
    screen.just_opened = screen.visible && !screen.was_visible;
    let scale = window.as_vec2().y / 1000.;
    if screen.visible {
        if screen.just_opened {
            if *mode == AppMode::Replay {
                // Canonical quiet make-animations-instant command, not an OS
                // WindowFocusEvent despite the original folded allocation name.
                history.write(HistoryCommand { bytes: [0, 3, 0, 3] });
            }
            saves.write(SaveGameCmd);
        }
        let root = mei.safe_frame("tod", false);
        let template = root.template("menu/time_of_day_2");
        let mut confirm = false;
        for (circle, slot) in [(0, "tod_circle"), (1, "angle_circle")] {
            template.with_slot(slot, |parent| draw_circle(
                &mut mei, parent, circle, &mut screen, &mut state, &tod,
                cursor.0, scale, time.delta, &mut sounds, &mut confirm,
            ));
        }
        template.with_slot("tod_button", |parent| time_button(
            &mut mei, parent, &mut state, &mut config, time.delta, confirm, &mut sounds,
        ));
    }
    screen.was_visible = screen.visible;
}


#[cfg(test)]
mod tests {
    use super::*;
    use country_core::input::action_set::{DigitalActionPhase, DigitalActionState};

    // Executed B patched EXE SHA256:
    // cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323.
    // These are original instructions, with only the CRT fmodf boundary hooked.
    const TO_ANGLE: &[(u32, u32)] = &[
        (0xc1200000, 0x3f4a8d23),
        (0xbf800000, 0x3f4a8d23),
        (0x80000000, 0x3f4a8d23),
        (0x00000000, 0x3f4a8d23),
        (0x000116c2, 0x3f4a8d23),
        (0x3dcccccd, 0x3fa0d97d),
        (0x3e99999a, 0x400bffaa),
        (0x3f0147ae, 0x40490fdb),
        (0x3f547ae1, 0x00000000),
        (0x3f7fffff, 0x3f4a8d18),
        (0x3f800000, 0x3f4a8d23),
        (0x40000000, 0x3f4a8d23),
        (0x42c80000, 0x3f4a8d23),
        (0x7fc00000, 0x00000000),
        (0x7f800000, 0x00000000),
        (0xff800000, 0x00000000),
        (0xbf13b798, 0x4030a171),
        (0x407d3f38, 0x3f174c50),
        (0x3f1b1ea8, 0x4083c0d6),
        (0xbefad711, 0x404c3414),
        (0xc064fcf7, 0x40305b51),
        (0x3f0e96fc, 0x406939fb),
        (0x3fd52d3e, 0x40962905),
        (0xbffa24ff, 0x3f808740),
        (0x3f7ada9d, 0x3f3299b8),
        (0x3f131d6d, 0x4074297c),
        (0x405fef58, 0x40474510),
        (0x3f0f5c46, 0x406b16c0),
        (0xbfbfc5da, 0x404819e7),
        (0x3f95a081, 0x3fc9eea0),
        (0x403fa74f, 0x3f4419f8),
        (0x3febc372, 0x3d62e580),
        (0x3edecfe9, 0x403443c2),
        (0x3ee4f416, 0x4037d66a),
        (0xc034b4de, 0x3fce6674),
        (0x3fbd2761, 0x4040f301),
        (0x4076e26a, 0x3e036480),
        (0xbfab8c91, 0x409467fe),
        (0x3ebb1fe0, 0x401f80a0),
        (0x401a6c34, 0x402d9d50),
        (0xbfd35c77, 0x401a845e),
        (0xc03ab420, 0x3f9693a0),
        (0xc06c5d93, 0x400e0555),
        (0xbe54c81a, 0x40bd5f00),
        (0x405cd592, 0x4038d6ac),
        (0x40658bdb, 0x407b94ee),
        (0x40515639, 0x400353b6),
        (0x40028f21, 0x3f7a3192),
        (0xc076fe0a, 0x3fb91f6d),
        (0x4027d4c5, 0x4088d57c),
        (0x3f9aee5f, 0x3fe29e6d),
        (0xbf4a1106, 0x3fe2c8dc),
        (0x405d7efc, 0x403beb29),
        (0xc0642b66, 0x40342aaf),
        (0xc05dbc33, 0x405bdc97),
        (0xc00a1ae5, 0x3d66d500),
        (0x3ecf23f1, 0x402b25b0),
        (0x3f4c281f, 0x40bf0140),
        (0x3f04893d, 0x4050ee3e),
        (0xc004da41, 0x3ee06c90),
        (0xc034e885, 0x3fcc85a7),
        (0x4041a036, 0x3f68d1ac),
        (0x3f9239e1, 0x3fba1a8e),
        (0x3fe6bc1c, 0x40c0975c),
        (0x3dee1b5d, 0x3faa89c4),
        (0x3ea14794, 0x4010776c),
        (0xc05a003f, 0x407ff541),
        (0xbfe52400, 0x3fe248ec),
        (0xc04e99fe, 0x40b71330),
        (0x3cb0efc8, 0x3f644924),
        (0xbf249319, 0x401d041f),
        (0xbfc58499, 0x403abb52),
        (0xc0335460, 0x3fdb379a),
        (0xbebb98d6, 0x408c4f67),
        (0x402498b2, 0x40726670),
        (0xc05e058c, 0x40591794),
        (0xbe52ef2a, 0x40bdedde),
        (0xbf252b30, 0x401c5328),
        (0x407bb844, 0x3ef5bc00),
        (0x3ffcbeac, 0x3f2c4028),
    ];
    const TO_TOD: &[(u32, u32)] = &[
        (0x00000000, 0x3f547ae1),
        (0x80000000, 0x3f547ae1),
        (0x000116c2, 0x3f547ae1),
        (0x40490fdb, 0x3f0147ae),
        (0x40c90fdb, 0x3f547ae1),
        (0xbf800000, 0x00000000),
        (0x40e00000, 0x00000000),
        (0x7fc00000, 0x00000000),
        (0x7f800000, 0x00000000),
        (0x40039f48, 0x3e8b33ae),
        (0x40bed4a2, 0x3f4c0332),
        (0x3eb3ce7c, 0x3f67cbea),
        (0x3f4e4634, 0x3b4cc6c0),
        (0x40b6728d, 0x3f451316),
        (0x3f92a362, 0x3d9bf1ec),
        (0x405bf588, 0x3f091986),
        (0x3fae996a, 0x3dfc116c),
        (0x401cf882, 0x3eb6c5d6),
        (0x40520690, 0x3f04fd3c),
        (0x3f9c9b87, 0x3dbe378c),
        (0x3f45810f, 0x3f7eea63),
        (0x3f1fb4e1, 0x3f76cb5f),
        (0x3fca68a2, 0x3e2dd5c0),
        (0x40a22ae6, 0x3f344a72),
        (0x3cdb4578, 0x3f55f3c7),
        (0x40a32c4b, 0x3f351f77),
        (0x3f91dc79, 0x3d99461e),
        (0x3de73b22, 0x3f5ab0b5),
        (0x40b5fcb3, 0x3f44b18d),
        (0x3e07c913, 0x3f5bc610),
        (0x40820237, 0x3f19ad07),
        (0x403f653f, 0x3ef1f1d2),
        (0x4086e447, 0x3f1db797),
        (0x3fb4da46, 0x3e08c862),
        (0x3e05bfbf, 0x3f5baa0f),
        (0x3e83e0ce, 0x3f62a5ca),
        (0x3f45e0ae, 0x3f7efeee),
        (0x403e8a22, 0x3ef07932),
        (0x3ebef106, 0x3f68fe26),
        (0x40500135, 0x3f042727),
        (0x3fe075c0, 0x3e53bcfc),
        (0x408403f8, 0x3f1b5637),
        (0x40c2ddab, 0x3f4f5a25),
        (0x409ac043, 0x3f2e271f),
        (0x3fc9c112, 0x3e2cb5bc),
        (0x4036fdda, 0x3ee37fd8),
        (0x40c1363b, 0x3f4dfbb3),
        (0x40becdd5, 0x3f4bfd91),
        (0x40c373ca, 0x3f4fd662),
        (0x405c707a, 0x3f094c66),
        (0x40a325dd, 0x3f351a25),
        (0x40b7b420, 0x3f461d39),
        (0x3ea6b03a, 0x3f666324),
        (0x40b2378a, 0x3f4192c7),
        (0x40790401, 0x3f151f96),
        (0x4032a5b7, 0x3edc0838),
        (0x3fffae06, 0x3e84b359),
        (0x3f3eac1f, 0x3f7d729c),
        (0x3fa6a60e, 0x3de0bc5c),
        (0x409d7c56, 0x3f306a82),
        (0x3fecd515, 0x3e69013c),
        (0x3e986b11, 0x3f64daae),
        (0x4000956f, 0x3e85faa9),
        (0x40968795, 0x3f2aa8bf),
        (0x40c404fe, 0x3f504e8d),
        (0x40683aa8, 0x3f0e2d55),
        (0x40111038, 0x3ea24e37),
        (0x40a71c92, 0x3f3861ec),
        (0x4009457f, 0x3e94e98b),
        (0x3fab732d, 0x3df13dc8),
        (0x404edc6c, 0x3f03adff),
        (0x401a4150, 0x3eb21ae2),
        (0x4035b10e, 0x3ee143ce),
    ];
    const INPUT: &[(u8, u8, u8, u32, u8, u8)] = &[
        (0, 18, 0, 0x3f000000, 0, 0),
        (0, 18, 1, 0x3f000000, 0, 0),
        (0, 18, 2, 0x3f000000, 0, 0),
        (0, 18, 3, 0x3f000000, 0, 0),
        (0, 18, 4, 0x3f000000, 0, 0),
        (0, 19, 0, 0x3f000000, 0, 0),
        (0, 19, 1, 0x3f000000, 0, 0),
        (0, 19, 2, 0x3f000000, 0, 0),
        (0, 19, 3, 0x3f000000, 0, 0),
        (0, 19, 4, 0x3f000000, 0, 0),
        (0, 20, 0, 0x3f000000, 0, 0),
        (0, 20, 1, 0x3f000000, 0, 0),
        (0, 20, 2, 0x3f000000, 0, 0),
        (0, 20, 3, 0x3f000000, 0, 0),
        (0, 20, 4, 0x3f000000, 0, 0),
        (0, 21, 0, 0x3f000000, 0, 0),
        (0, 21, 1, 0x3f000000, 0, 0),
        (0, 21, 2, 0x3f000000, 0, 0),
        (0, 21, 3, 0x3f000000, 0, 0),
        (0, 21, 4, 0x3f000000, 0, 0),
        (0, 22, 0, 0x3f000000, 0, 0),
        (0, 22, 1, 0x3f000000, 0, 0),
        (0, 22, 2, 0x3f000000, 0, 0),
        (0, 22, 3, 0x3f000000, 0, 0),
        (0, 22, 4, 0x3f000000, 0, 0),
        (1, 18, 0, 0x3f000000, 0, 0),
        (1, 18, 1, 0x3f000000, 0, 0),
        (1, 18, 2, 0x3f000000, 0, 0),
        (1, 18, 3, 0x3f000000, 0, 0),
        (1, 18, 4, 0x3f000000, 0, 0),
        (1, 19, 0, 0x3f000000, 1, 1),
        (1, 19, 1, 0x3f000000, 0, 0),
        (1, 19, 2, 0x3f000000, 1, 1),
        (1, 19, 3, 0x3f000000, 0, 0),
        (1, 19, 4, 0x3f000000, 0, 0),
        (1, 20, 0, 0x3ec00000, 0, 1),
        (1, 20, 1, 0x3ec00000, 0, 1),
        (1, 20, 2, 0x3ec00000, 0, 1),
        (1, 20, 3, 0x3f000000, 0, 0),
        (1, 20, 4, 0x3f000000, 0, 0),
        (1, 21, 0, 0x3f200000, 0, 1),
        (1, 21, 1, 0x3f200000, 0, 1),
        (1, 21, 2, 0x3f200000, 0, 1),
        (1, 21, 3, 0x3f000000, 0, 0),
        (1, 21, 4, 0x3f000000, 0, 0),
        (1, 22, 0, 0x3f000000, 0, 0),
        (1, 22, 1, 0x3f000000, 0, 0),
        (1, 22, 2, 0x3f000000, 0, 0),
        (1, 22, 3, 0x3f000000, 0, 0),
        (1, 22, 4, 0x3f000000, 0, 0),
    ];

    #[test]
    fn both_piecewise_maps_match_153_original_instruction_samples() {
        for &(input, expected) in TO_ANGLE {
            assert_eq!(remap_tod_to_angle(f32::from_bits(input)).unwrap_or(0.).to_bits(), expected,
                "native TOD-to-angle input {input:08x}");
        }
        for &(input, expected) in TO_TOD {
            assert_eq!(remap_angle_to_tod(f32::from_bits(input)).unwrap_or(0.).to_bits(), expected,
                "native angle-to-TOD input {input:08x}");
        }
        assert!(remap_tod_to_angle(f32::NAN).is_none());
        assert!(remap_angle_to_tod(f32::NAN).is_none());
    }

    #[test]
    fn content_capture_bubble_hold_repeat_and_confirm_match_original_50_cases() {
        for &(phase, slot, action_phase, expected_delta, confirmed, handled) in INPUT {
            let phase_value = match action_phase {
                0 => DigitalActionPhase::JustPressed,
                1 => DigitalActionPhase::Held,
                2 => DigitalActionPhase::Repeating,
                3 => DigitalActionPhase::JustReleased,
                4 => DigitalActionPhase::Idle,
                _ => unreachable!(),
            };
            let action = ActionSetAction::Digital {
                slot, state: DigitalActionState { phase: phase_value, timer_seconds: 0. },
            };
            let mut content = TodCircleContent { delta: 0.5, frame_delta: 0.125, confirmed: false };
            let actual = content.on_input_phase(&action, [phase != 0, false]);
            assert_eq!((content.delta.to_bits(), u8::from(content.confirmed), u8::from(actual)),
                (expected_delta, confirmed, handled), "native phase {phase}, slot {slot}, state {action_phase}");
        }
    }

    fn panel() -> (World, Schedule) {
        let mut mei = toolbar::MeiContext::default();
        let asset_root = std::env::var_os("TG_INSTALL_ROOT").map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade"))
            .join("assets/ui");
        for name in ["menu/time_of_day_2", "menu/tod_time_passage_hint"] {
            let config = toolbar::mei::config::read_node_config_from_file(
                &asset_root.join(name).with_extension("ron")).unwrap();
            mei.configs.insert(name.into(), Arc::new(config));
        }
        let mut world = World::new();
        world.insert_resource(mei);
        world.insert_resource(TimeOfDayScreen { visible: true, time_of_day: 0.3,
            ..Default::default() });
        world.init_resource::<TodButtonState>();
        world.init_resource::<GameConfig>();
        world.init_resource::<TimeOfDay>();
        world.init_resource::<ActionSetStates>();
        world.init_resource::<GameTime>();
        world.insert_resource(AppMode::Replay);
        world.insert_resource(EscMenuState::Closed);
        world.insert_resource(WindowSize::new(960, 540));
        world.insert_resource(CursorPositionSS(Vec2::splat(-1000.)));
        world.init_resource::<Events<SaveGameCmd>>();
        world.init_resource::<Events<HistoryCommand>>();
        world.init_resource::<Events<PlayUiSound>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(do_time_of_day_screen);
        (world, schedule)
    }

    #[test]
    fn shipped_panel_open_edges_emit_canonical_save_and_quiet_replay_commands() {
        let (mut world, mut schedule) = panel();
        schedule.run(&mut world);
        assert!(world.resource::<TimeOfDayScreen>().just_opened);
        assert_eq!(world.resource_mut::<Events<SaveGameCmd>>().drain().count(), 1);
        let history: Vec<_> = world.resource_mut::<Events<HistoryCommand>>().drain().collect();
        assert_eq!(history, [HistoryCommand { bytes: [0, 3, 0, 3] }]);
        let mei = world.resource::<toolbar::MeiContext>();
        assert_eq!(mei.nodes.values().filter(|node|
            node.content.type_name() == "tiny_glade::gui::gui_tod::TodCircleContent").count(), 2);
        assert!(mei.nodes.values().any(|node| {
            let content: &dyn std::any::Any = node.content.as_ref();
            content.downcast_ref::<ImageContent>().is_some_and(|image|
                image.image == "menu/tod/circle_tod")
        }));
        schedule.run(&mut world);
        assert!(!world.resource::<TimeOfDayScreen>().just_opened);
        assert!(world.resource::<Events<SaveGameCmd>>().is_empty());
        assert!(world.resource::<Events<HistoryCommand>>().is_empty());
        world.resource_mut::<ActionSetStates>().digital[18] = DigitalActionState {
            phase: DigitalActionPhase::JustPressed, timer_seconds: 0.,
        };
        schedule.run(&mut world);
        assert!(!world.resource::<TimeOfDayScreen>().visible);
        assert!(!world.resource::<TimeOfDayScreen>().was_visible);
    }

    #[test]
    fn session_exit_clears_tod_drag_and_open_edges_before_reentry() {
        let (mut world, mut schedule) = panel();
        schedule.run(&mut world);
        // 模拟已经进入拖动的旧会话；退出必须清理本地动画及真实输入标志。
        *world.resource_mut::<TodButtonState>() = TodButtonState {
            hover: [0.7, 0.8], previous_target_angle: 2., previous_target_time: 0.3,
            previous_actual_angle: 1., previous_actual_time: 0.2,
            button_hover: 0.9, pulse: 0.5, pulsing: true,
        };
        {
            let mut screen = world.resource_mut::<TimeOfDayScreen>();
            screen.time_input_active = true;
            screen.angle_input_active = true;
        }
        let config_before = world.resource::<GameConfig>().dev.clone();
        crate::toolbar_plugin::clear_toolbar_session(&mut world);
        assert_eq!(*world.resource::<TodButtonState>(), TodButtonState::default());
        assert_eq!(*world.resource::<TimeOfDayScreen>(), TimeOfDayScreen::default());
        assert_eq!(world.resource::<GameConfig>().dev, config_before);
        assert!(world.resource::<toolbar::MeiContext>().nodes.is_empty());
        world.resource_mut::<Events<SaveGameCmd>>().clear();
        world.resource_mut::<Events<HistoryCommand>>().clear();
        // 重入后的隐藏帧不应保存；再次打开才重新产生一个真实打开边沿。
        schedule.run(&mut world);
        assert!(world.resource::<Events<SaveGameCmd>>().is_empty());
        world.resource_mut::<TimeOfDayScreen>().visible = true;
        schedule.run(&mut world);
        assert!(world.resource::<TimeOfDayScreen>().just_opened);
        assert_eq!(world.resource::<Events<SaveGameCmd>>().len(), 1);
        assert_eq!(world.resource::<Events<HistoryCommand>>().len(), 1);
    }
}
