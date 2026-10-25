//! B `garden_audio.rs`: garden draw/erase sounds — 4 loops driven by the native
//! 4-loop gate + volume modulation (decomp-0x140a89480, O).
//!
//! O-level facts (chapter garden_audio.rs.md, decomp-0x140a88fa0 /
//! 0x140a89480, rne-read): init creates 4 loops — `event:/SFX/Garden/GardenDraw`,
//! `event:/SFX/Garden/GardenDrawWater`, `event:/SFX/Garden/GardenErase`,
//! `event:/SFX/Garden/GardenEraseWater`. `garden_audio_system` (0x140a89480)
//! folds `GardenDrawEvent`/`GardenEraseEvent` into the 4 loop should-play flags
//! via a water-raster bilinear sample gate, then drives each loop with the
//! native `FUN_14166d9f0` template (position + `editing_speed` volume +
//! `PathSize` from the brush radius).
//!
//! Native 6-param contract (PDB FunctionSystem procs.tsv 行 49396, ① contract
//! round4/contracts/t1-garden_audio-130-gate-CONTRACT.md):
//! - `param_1` = `Res<AppMode>` (tag read at `*(int*)*param_1`).
//! - `param_2` = `GardenAudioState` (4 loops).
//! - `param_3` / `param_4` = draw / erase event buffers (bevy `EventReader`).
//! - `param_5` = `Res<CursorTerrainRaycast>`, native 32B `{tag:u32@0, pos:Vec3@0x04,
//!   delta:Vec3@0x10, speed:f32@0x1c}`. The sample position is `pos.x@+4 / pos.z@+0xc`;
//!   the editing volume is `[+0x1c] = speed` (written by
//!   `mouse_terrain_raycast` 0x1408ef320 each frame as `speed = len(delta)/[param_7+0x58]`;
//!   the +0x58 divisor resource identity 待取证). The 32B extension was approved
//!   by central arbitration (PROGRESS.md 仲裁表); the system reads `cursor.speed`
//!   directly. **Runtime dependency (③ REVIEW.md §4b)**: `mouse_terrain_raycast`
//!   0x1408ef320 is not yet an in-crate producer, so `speed`/`delta` stay 0 until
//!   the T2 run_game_thread host wiring registers the writer — the standalone
//!   non-Replay volume is therefore 0 until then.
//! - `param_6` = `Res<world_raster_defs::WaterRaster>` (a `WorldRaster<512>`; **not**
//!   GardenRaster — ① correction). The buffer selection byte `@+0x298` picks
//!   composite (0, ptr@+8/len@+0x10) or scratch (else, ptr@+0x20/len@+0x28);
//!   `current_buffer()` / `RasterBufferRef::new`.
//!
//! Gate (4 loop calls, O):
//! - `GardenDrawWater` (index 1, native loop@+0x38): draw count `!= 0` && sample
//!   `>= 0.01` → play.
//! - `GardenDraw` (index 0, native loop@+0): draw count `!= 0` && sample `< 0.01`.
//! - `GardenEraseWater` (index 3, native loop@+0xa8): erase count `!= 0` &&
//!   sample `>= 0.01`.
//! - `GardenErase` (index 2, native loop@+0x70): erase count `!= 0` && sample
//!   `< 0.01`.
//! The count passed to the template is the event count only when the gate
//! passes, else 0 (`FUN_14166d9f0` treats `0` as stop). The sample coordinates
//! come from `CursorTerrainRaycast.pos` — **not** the event positions.
//!
//! Volume (O): `fVar34 = 1.0` when `AppMode::native_tag() == 0xd` (Replay),
//! else `[cursor+0x1c] / 60.0` (`_UNK_1429d18cc` = `00 00 70 42` = f32 60.0).
//! The template sets the loop instance `editing_speed` parameter to it.
//!
//! Template secondary param (O, ① contract §5): `PathSize = 2 * inverse_lerp(
//! 0.32@0x142ac56c0, 1.2@0x1429cd128, event.radius@+0xc)` — brush radius in
//! `[0.32, 1.2]` maps to FMOD `PathSize` in `[0, 2]` (`set_parameter("PathSize")`).

use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::{
    resources::{
        audio_instance::EDITING_SPEED_PARAMETER,
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        cursor_terrain_raycast::CursorTerrainRaycast,
        world_raster::RasterBufferRef,
        world_raster_defs::WaterRaster,
    },
    systems::{
        garden::{GardenDrawEvent, GardenEraseEvent},
        mode_manager::AppMode,
    },
    utils::audio_utils::AudioLoop,
};

/// The four garden loops (O: rne-read).
pub const GARDEN_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Garden/GardenDraw",
    "event:/SFX/Garden/GardenDrawWater",
    "event:/SFX/Garden/GardenErase",
    "event:/SFX/Garden/GardenEraseWater",
];

/// Loop index of `GardenDraw` in `GARDEN_LOOP_EVENTS` / `GardenAudioState::loops`
/// (O: native loop@+0).
pub const GARDEN_DRAW_INDEX: usize = 0;
/// Loop index of `GardenDrawWater` (O: native loop@+0x38).
pub const GARDEN_DRAW_WATER_INDEX: usize = 1;
/// Loop index of `GardenErase` (O: native loop@+0x70).
pub const GARDEN_ERASE_INDEX: usize = 2;
/// Loop index of `GardenEraseWater` (O: native loop@+0xa8).
pub const GARDEN_ERASE_WATER_INDEX: usize = 3;

/// O: `_UNK_142960674` = bytes `0a d7 23 3c` (little-endian f32 `0x3c23_d70a`
/// ≈ 0.01). Sample `>=` this gate → the water loop pairs.
pub const GARDEN_WATER_GATE: f32 = f32::from_bits(0x3c23_d70a);
/// O: `_UNK_1429d18cc` = bytes `00 00 70 42` (little-endian f32 `0x4270_0000`
/// = 60.0). The `[+0x1c]` editing volume is normalized by it when not Replay.
pub const VOLUME_NORMALIZER: f32 = f32::from_bits(0x4270_0000);
/// O: `_UNK_142925904` = f32 1.0 — the volume when `AppMode == Replay` (0xd).
pub const REPLAY_VOLUME: f32 = 1.0;
/// O: `0x43020000` = f32 130.0 — the water-raster world extent (x/z) for the
/// bilinear sample window (`bilinear_sample_at_world_xz` extent, 130.0×2;
/// `_UNK_142a7b658` rne-read `00000243` = 130.0).
pub const GARDEN_RASTER_EXTENT: f32 = f32::from_bits(0x4302_0000);

/// O: `_UNK_142ac56c0` = bytes `0b d7 a3 3e` (little-endian f32 `0x3ea3_d70b`
/// = 0.32) — the `PathSize` inverse_lerp lower bound (brush radius minimum).
pub const PATH_SIZE_MIN: f32 = f32::from_bits(0x3ea3_d70b);
/// O: `_UNK_1429cd128` = bytes `9a 99 99 3f` (little-endian f32 `0x3f99_999a`
/// = 1.2) — the `PathSize` inverse_lerp upper bound (brush radius maximum).
pub const PATH_SIZE_MAX: f32 = f32::from_bits(0x3f99_999a);
/// O: the FMOD `PathSize` parameter is `2 * inverse_lerp(min, max, radius)` —
/// brush radius `[0.32, 1.2]` → `PathSize` `[0, 2]`.
pub const PATH_SIZE_SCALE: f32 = 2.0;
/// O: `0x142c84553` (len 8) — the FMOD parameter name for the brush size
/// (shared by the garden/path draw template).
pub const PATH_SIZE_PARAMETER: &str = "PathSize";

/// GardenAudioState: the four loops.
#[derive(Resource)]
pub struct GardenAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
}

impl<E: EventInstance> GardenAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        GardenAudioState {
            loops: GARDEN_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `garden_audio_init_system` (0x140a88fa0).
pub fn garden_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(GardenAudioState::<S::Event>::new(&core));
}

/// The 4-loop gate (O: decomp-0x140a89480 loop calls). Returns the should-play
/// flags for `[GardenDraw, GardenDrawWater, GardenErase, GardenEraseWater]`.
/// An event count is folded to "play" only when non-zero and the gate passes.
pub fn garden_loop_flags(sample: f32, draw_count: usize, erase_count: usize) -> [bool; 4] {
    let water = sample >= GARDEN_WATER_GATE;
    [
        draw_count != 0 && !water,
        draw_count != 0 && water,
        erase_count != 0 && !water,
        erase_count != 0 && water,
    ]
}

/// O: `PathSize = 2 * inverse_lerp(PATH_SIZE_MIN, PATH_SIZE_MAX, radius)` —
/// native `inverse_lerp` 0x140c904a0 (no clamp); brush radius `[0.32, 1.2]` →
/// FMOD `PathSize` `[0, 2]`.
pub fn garden_path_size(radius: f32) -> f32 {
    PATH_SIZE_SCALE * (radius - PATH_SIZE_MIN) / (PATH_SIZE_MAX - PATH_SIZE_MIN)
}

/// O: `FUN_14166d9f0` — the garden (volume-aware) draw-loop template.
/// `should_play` is the loop gate; `position`/`radius` come from the last
/// matching event (native passes the event-tail pointer); `volume` is the
/// normalized editing volume, stored on the loop instance as the `editing_speed`
/// parameter. The brush radius is mapped to the FMOD `PathSize` parameter
/// (`2 * inverse_lerp_unclamped(PATH_SIZE_MIN, PATH_SIZE_MAX, radius)`, ① contract
/// §5). The native starts the loop (idempotent; decomp shows a conditional +
/// unconditional `start_if_not_playing` pair that is equivalent to one call).
/// When `should_play == false` the loop is stopped (`FUN_14166d9f0` treats a
/// zero count / null event pointer as stop).
pub fn update_draw_audio_volume<E: EventInstance>(
    loop_player: &mut AudioLoop<AudioInstance<E>>,
    should_play: bool,
    position: Vec3,
    radius: f32,
    volume: f32,
) {
    if should_play {
        let current = &mut loop_player.entries[loop_player.cursor];
        let _ = current.set_position([position.x, position.y, position.z]);
        let _ = current.set_parameter(EDITING_SPEED_PARAMETER, volume);
        // O: PathSize = 2 * inverse_lerp(0.32, 1.2, radius) — native 0x140c904a0
        // (no clamp); radius [0.32, 1.2] → PathSize [0, 2].
        let _ = current.set_parameter(PATH_SIZE_PARAMETER, garden_path_size(radius));
        let _ = loop_player.start_if_not_playing();
    } else {
        let _ = loop_player.stop_now();
    }
}

/// `garden_audio_system` (0x140a89480): fold garden draw/erase events into the
/// loop should-play flags via the water-raster sample gate, then drive each
/// loop with the volume-aware template.
///
/// Param order mirrors the native 6-param contract: `AppMode`, `GardenAudioState`,
/// draws, erases, `CursorTerrainRaycast`, `WaterRaster`. The native `param_5`
/// carries both the sample position (`pos.x@+4 / pos.z@+0xc`) and the `+0x1c`
/// volume (`speed`) on the same resource; `param_6` is the water raster.
#[allow(clippy::too_many_arguments)]
pub fn garden_audio_system<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    mut state: ResMut<GardenAudioState<S::Event>>,
    mut draws: EventReader<GardenDrawEvent>,
    mut erases: EventReader<GardenEraseEvent>,
    cursor: Res<CursorTerrainRaycast>,
    raster: Res<WaterRaster>,
) {
    // O: volume — AppMode == 0xd (Replay) → 1.0, else [cursor+0x1c] / 60.0
    // (the +0x1c face is `CursorTerrainRaycast.speed`).
    let volume = if mode.native_tag() == AppMode::Replay.native_tag() {
        REPLAY_VOLUME
    } else {
        cursor.speed / VOLUME_NORMALIZER
    };

    let draw_events: Vec<_> = draws.read().copied().collect();
    let erase_events: Vec<_> = erases.read().copied().collect();

    // O: the native samples the water raster only where that channel's count is
    // non-zero (each loop call gates `count == 0 → 0`). Sampling unconditionally
    // is output-equivalent — when both counts are 0 the flags are all false
    // regardless of the sample — but we gate it anyway to mirror the native
    // control flow and avoid an out-of-bounds sample on an empty raster buffer.
    let flags = if draw_events.is_empty() && erase_events.is_empty() {
        [false; 4]
    } else {
        // O: sample coordinates come from `CursorTerrainRaycast.pos` (x@+4,
        // z@+0xc) — **not** the event positions; extent 130.0×2.
        let sample = RasterBufferRef::<512>::new(raster.current_buffer())
            .bilinear_sample_at_world_xz(
                cursor.position().x,
                cursor.position().z,
                GARDEN_RASTER_EXTENT,
                GARDEN_RASTER_EXTENT,
            );
        garden_loop_flags(sample, draw_events.len(), erase_events.len())
    };
    // O: each loop is driven at the last matching event's position/radius.
    let draw_last = draw_events.last().copied();
    let erase_last = erase_events.last().copied();
    let (draw_pos, draw_radius) = (
        draw_last.map_or(Vec3::ZERO, |e| e.position),
        draw_last.map_or(0.0, |e| e.radius),
    );
    let (erase_pos, erase_radius) = (
        erase_last.map_or(Vec3::ZERO, |e| e.position),
        erase_last.map_or(0.0, |e| e.radius),
    );
    for (index, loop_player) in state.loops.iter_mut().enumerate() {
        let (position, radius) = if index < 2 {
            (draw_pos, draw_radius)
        } else {
            (erase_pos, erase_radius)
        };
        update_draw_audio_volume(loop_player, flags[index], position, radius, volume);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garden_loop_names_are_original() {
        assert_eq!(GARDEN_LOOP_EVENTS[0], "event:/SFX/Garden/GardenDraw");
        assert_eq!(GARDEN_LOOP_EVENTS[1], "event:/SFX/Garden/GardenDrawWater");
        assert_eq!(GARDEN_LOOP_EVENTS[2], "event:/SFX/Garden/GardenErase");
        assert_eq!(GARDEN_LOOP_EVENTS[3], "event:/SFX/Garden/GardenEraseWater");
    }

    #[test]
    fn loop_indexes_match_the_native_offsets() {
        // O: loop@+0 (GardenDraw), +0x38 (GardenDrawWater), +0x70 (GardenErase),
        // +0xa8 (GardenEraseWater) → the 0x38 stride AudioLoop.
        assert_eq!(GARDEN_DRAW_INDEX, 0);
        assert_eq!(GARDEN_DRAW_WATER_INDEX, 1);
        assert_eq!(GARDEN_ERASE_INDEX, 2);
        assert_eq!(GARDEN_ERASE_WATER_INDEX, 3);
    }

    #[test]
    fn gate_constants_are_the_original_immediates() {
        // 0x3c23_d70a = f32 0.01; 0x4270_0000 = f32 60.0; 0x4302_0000 = f32 130.0.
        assert_eq!(GARDEN_WATER_GATE.to_bits(), 0x3c23_d70a);
        assert_eq!(VOLUME_NORMALIZER.to_bits(), 0x4270_0000);
        assert_eq!(REPLAY_VOLUME.to_bits(), 0x3f80_0000);
        assert_eq!(GARDEN_RASTER_EXTENT.to_bits(), 0x4302_0000);
        assert!((GARDEN_WATER_GATE - 0.01).abs() < 1e-6);
        // ① contract §5 rne-read: 0x142ac56c0 = 0x3ea3_d70b = 0.32;
        // 0x1429cd128 = 0x3f99_999a = 1.2.
        assert_eq!(PATH_SIZE_MIN.to_bits(), 0x3ea3_d70b);
        assert_eq!(PATH_SIZE_MAX.to_bits(), 0x3f99_999a);
        assert_eq!(PATH_SIZE_SCALE, 2.0);
        assert_eq!(PATH_SIZE_PARAMETER, "PathSize");
    }

    #[test]
    fn path_size_maps_brush_radius_to_the_fmod_parameter() {
        // radius 0.32 → PathSize 0; radius 1.2 → PathSize 2; radius 0.76 → 1.
        assert!((garden_path_size(0.32) - 0.0).abs() < 1e-6);
        assert!((garden_path_size(1.2) - 2.0).abs() < 1e-6);
        assert!((garden_path_size(0.76) - 1.0).abs() < 1e-6);
        // Unclamped (native 0x140c904a0): radius past the bounds extrapolates.
        assert!(garden_path_size(2.0) > 2.0);
        assert!(garden_path_size(0.0) < 0.0);
    }

    #[test]
    fn water_gate_selects_draw_water_and_erase_water_loops() {
        // sample >= 0.01 → draw-water / erase-water; count == 0 → stop.
        let flags = garden_loop_flags(0.05, 3, 1);
        assert_eq!(
            flags,
            [false, true, false, true],
            "water sample selects the water loop pairs"
        );
        let flags = garden_loop_flags(0.005, 3, 1);
        assert_eq!(
            flags,
            [true, false, true, false],
            "dry sample selects the normal loop pairs"
        );
    }

    #[test]
    fn zero_event_count_stops_the_loop_even_when_the_gate_passes() {
        let flags = garden_loop_flags(0.05, 0, 0);
        assert_eq!(flags, [false, false, false, false]);
        let flags = garden_loop_flags(0.005, 0, 0);
        assert_eq!(flags, [false, false, false, false]);
    }
}
