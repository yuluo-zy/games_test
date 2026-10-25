//! `WaterPlugin`: registers the water events, resources and systems.
//!
//! Order: `replay_raster` -> `draw_water` -> `update_water_audio_grid` in `Update` (the document gives
//! no ordering between them: UNKNOWN(evidence: water-cpu-analysis)); `water_rasterize_from_cpu` runs
//! in `Last` (render world in the original, `render_world` @0x1404ec2f6). The audio grid system has
//! `run_if Option<Res<WaterAudioGrid>>` like `system_water::impl$0::build` (0x1406501d0).

use super::audio_grid::{update_water_audio_grid, WaterAudioGrid};
use super::raster::WorldRaster;
use super::replay::{replay_raster, RecalculateHeightMapCmd, ReplayQueue, ScheduledTerrainReadback};
use super::tool::{
    draw_water, ActiveRasterStroke, BrushSizes, LastDrawWater, WaterDrawEvent, WaterEraseEvent, WaterToolConfig,
    WaterToolInput,
};
use crate::env::EnvPlugin;
use bevy_app::{App, Last, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::common_conditions::resource_exists;

/// `water_rasterize_from_cpu` (0x140a12680) as a system: staging -> GPU mask when `+0x29b` is set.
pub fn water_rasterize_from_cpu(mut raster: ResMut<WorldRaster>) {
    raster.water_rasterize_from_cpu();
}

#[derive(Default)]
pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvPlugin)
            .add_event::<WaterDrawEvent>()
            .add_event::<WaterEraseEvent>()
            .add_event::<RecalculateHeightMapCmd>()
            .init_resource::<BrushSizes>()
            .init_resource::<ActiveRasterStroke>()
            .init_resource::<WaterToolInput>()
            .init_resource::<WaterToolConfig>()
            .init_resource::<LastDrawWater>()
            .init_resource::<ReplayQueue>()
            .init_resource::<ScheduledTerrainReadback>()
            .add_systems(
                Update,
                (replay_raster, draw_water, update_water_audio_grid.run_if(resource_exists::<WaterAudioGrid>)).chain(),
            )
            .add_systems(Last, water_rasterize_from_cpu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::audio_grid::WaterAudioGridCell;
    use crate::water::replay::ReplayCmd;
    use crate::water::tool::{StrokePhase, TOOL_WATER};
    use crate::env::{CursorTerrainRaycast, Time};
    use glam::Vec3;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::from_delta(0.016)).add_plugins(WaterPlugin);
        app
    }

    #[test]
    fn plugin_draws_a_stroke_then_uploads_and_audio_sees_it() {
        let mut app = app();
        app.insert_resource(WaterToolConfig { radius_scale: 1.0, half_width_cells: 6.0 });
        app.insert_resource(WaterAudioGrid { cells: vec![WaterAudioGridCell::new(0.0, 0.0), WaterAudioGridCell::new(0.0, 50.0)] });
        let path = [Vec3::new(-20.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(20.0, 0.0, 0.0)];
        for (p, phase) in path.iter().zip([StrokePhase::Start, StrokePhase::Update, StrokePhase::Release]) {
            app.insert_resource(WaterToolInput { tool_id: TOOL_WATER, phase });
            app.insert_resource(CursorTerrainRaycast(Some(*p)));
            app.update();
        }
        assert!(app.world().resource::<WorldRaster>().sample_at_world_xz(0.0, 0.0) > 0.9);
        assert_eq!(app.world().resource::<WorldRaster>().sample_at_world_xz(0.0, 50.0), 0.0);
        app.world_mut().resource_mut::<WorldRaster>().mark_reupload_dirty();
        app.update(); // `Last`: water_rasterize_from_cpu
        let r = app.world().resource::<WorldRaster>();
        assert_eq!(r.gpu_upload_count, 1);
        assert!(!r.reupload_dirty);
        let grid = app.world().resource::<WaterAudioGrid>();
        assert!(grid.cells[0].window_hits() >= 1, "wet cell near the camera counted");
        assert_eq!(grid.cells[1].window_hits(), 0, "dry cell");
    }

    #[test]
    fn plugin_replays_history_and_is_idempotent_with_env() {
        let mut app = app();
        app.world_mut().resource_mut::<ReplayQueue>().cmds = vec![ReplayCmd {
            points: vec![Vec3::new(-8.0, 0.0, 0.0), Vec3::new(8.0, 0.0, 0.0)],
            weight: 1.0,
            emit_event: true,
            ..Default::default()
        }];
        app.update();
        assert_eq!(app.world().resource::<WorldRaster>().composite[256 * 512 + 256], 255);
        assert!(app.world().resource::<ReplayQueue>().cmds.is_empty());
    }

    #[test]
    fn both_plugins_coexist() {
        let mut app = App::new();
        app.add_plugins((WaterPlugin, crate::critters::CrittersPlugin::new(1, 1)));
        app.update();
        assert!(app.world().contains_resource::<WorldRaster>());
    }
}
