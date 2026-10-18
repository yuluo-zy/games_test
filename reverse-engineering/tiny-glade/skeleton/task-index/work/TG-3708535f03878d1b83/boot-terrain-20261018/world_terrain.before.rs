//! Typed original W3/W6/W8 consumers on the enclosing real world graph.
//!
//! Borrows canonical TerrainEditingData::rasterization_parts from the same App.
//! It does not install resources, replace W4/W5/W7, or report a complete scene.
//! The core W3 body is directly refactored, not replaced by ActiveStrokes copies.

use crate::{render_bridge::CoreGraphFrame, scene_compute::with_env};
use bevy_ecs::{
    change_detection::{DetectChangesMut, Mut},
    event::Events,
    world::World,
};
use country_core::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, gui::DebugGui,
        terrain_editing::TerrainEditingData,
    },
    systems::{
        mode_manager::AppMode,
        render_loop::compute_passes::{
            highlight_active_terrain_stroke::{self as highlight, AddTerrainHighlightCmd},
            mouse_read_terrain_stroke_index,
            rasterize_terrain_stroke::{self as terrain, RecalculateHeightMapCmd},
        },
    },
};

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original W3..W11: missing {name} producer"))
}

/// Native W3 @1408c7120. Changed is unconditional and precedes the event lookup
/// and early return (723a/723d/72bd). Events::is_empty observes both retained
/// buffers; this is not an EventReader that drains/advances across frames.
pub fn rasterize_terrain_stroke(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<usize> {
    let gpu = frame.raster_resources();
    if !world.contains_resource::<TerrainEditingData>() {
        return Err(missing("TerrainEditingData"));
    }
    world.resource_scope(|world, mut data: Mut<TerrainEditingData>| {
        data.set_changed();
        let requested = !world
            .get_resource::<Events<RecalculateHeightMapCmd>>()
            .ok_or_else(|| missing("Events<RecalculateHeightMapCmd>"))?
            .is_empty();
        let (strokes, rasterized) = data.rasterization_parts();
        if !requested && *rasterized {
            return Ok(0);
        }
        let mut constants = gpu.dynamic_constants().lock();
        let previous_rasterized = *rasterized;
        let result = with_env(frame, world, |env| {
            terrain::rasterize_terrain_stroke(env, &mut constants, strokes, rasterized, requested)
        });
        // Native builders fail before the final marker write. PassFrame's
        // current void record API defers admission errors to with_compute's
        // finish, so commit the same state only when that finish succeeds.
        // A failed frame must still be discarded by its enclosing caller.
        if result.is_err() {
            *rasterized = previous_rasterized;
        }
        result
    })
}

/// Native W6 @1409768b0. Uses the canonical hit position and mapped output
/// SSBO, including miss tag -> two zero push words. No parallel cursor view.
pub fn mouse_read_terrain_stroke_index(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<()> {
    let hit = world
        .get_resource::<CursorTerrainRaycast>()
        .ok_or_else(|| missing("CursorTerrainRaycast"))?;
    with_env(frame, world, |env| {
        let pass = mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index(env, hit);
        env.frame.record(pass);
        Ok(())
    })
}

/// Native W8 @140a58450. Native mode TypeId maps to AppMode and +f9 suppression
/// to DebugGui::disable_terrain_hover_highlight. The native code takes the last
/// element of the new retained buffer, or old when new is empty, only in tags5/10.
/// Events::get_cursor is the current safe adapter to that same retained latest
/// item; no assertion is made that the native code calls an EventCursor API.
pub fn highlight_active_terrain_stroke(
    frame: &mut CoreGraphFrame<'_>,
    world: &World,
) -> rhapsody::Result<usize> {
    let mode = world
        .get_resource::<AppMode>()
        .ok_or_else(|| missing("AppMode"))?
        .native_tag();
    let latest = if mode == 5 || mode == 10 {
        let events = world
            .get_resource::<Events<AddTerrainHighlightCmd>>()
            .ok_or_else(|| missing("Events<AddTerrainHighlightCmd>"))?;
        events.get_cursor().read(events).last().copied()
    } else {
        None
    };
    let suppress = world
        .get_resource::<DebugGui>()
        .ok_or_else(|| missing("DebugGui"))?
        .disable_terrain_hover_highlight;
    with_env(frame, world, |env| {
        // A single latest item has the same last() result as the original two
        // buffers. Do not label it the literal new Bevy event buffer.
        let passes = highlight::highlight_active_terrain_stroke(
            env,
            mode,
            &[],
            latest.as_slice(),
            u8::from(suppress),
        );
        let count = passes.len();
        for pass in passes {
            env.frame.record(pass);
        }
        Ok(count)
    })
}

/// Native W10 @1409014e0. The same Path/Cobblestone owners consumed by W1,
/// and the same Recalculate event read by W3, determine this thirteen-pass step.
/// All mandatory identities are checked even when its run gate will be false.
pub fn compute_path_masks(
    frame: &mut CoreGraphFrame<'_>,
    world: &mut World,
) -> rhapsody::Result<Option<u32>> {
    use country_core::resources::world_raster_defs::{
        Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, PathRaster,
    };
    macro_rules! require {
        ($r:ty) => {
            if !world.contains_resource::<$r>() {
                return Err(missing(stringify!($r)));
            }
        };
    }
    require!(PathRaster);
    require!(Cobblestone1Raster);
    require!(Cobblestone2Raster);
    require!(Cobblestone3Raster);
    let requested = !world
        .get_resource::<Events<RecalculateHeightMapCmd>>()
        .ok_or_else(|| missing("Events<RecalculateHeightMapCmd>"))?
        .is_empty();
    let tick = world.read_change_tick().get();
    world.resource_scope(|world, mut a: Mut<PathRaster>| {
        world.resource_scope(|world, mut b: Mut<Cobblestone1Raster>| {
            world.resource_scope(|world, mut c: Mut<Cobblestone2Raster>| {
                world.resource_scope(|world, mut d: Mut<Cobblestone3Raster>| {
                    with_env(frame, world, |env| Ok(
                        country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks(
                            env, &mut a, &mut b, &mut c, &mut d, requested, tick)))
                })
            })
        })
    })
}
