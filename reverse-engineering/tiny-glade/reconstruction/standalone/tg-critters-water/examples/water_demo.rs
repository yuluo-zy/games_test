//! Rasterize a water stroke through the `draw_water` system of a real `bevy_app::App` and print an ASCII map.
//! Run: `cargo run -p tg-critters-water --example water_demo`

use bevy_app::App;
use glam::Vec3;
use tg_critters_water::env::{CursorTerrainRaycast, GladeShape, MainCamera, Time};
use tg_critters_water::water::audio_grid::{WaterAudioGrid, WaterAudioGridCell};
use tg_critters_water::water::tool::{BrushSizes, StrokePhase, WaterDrawEvent, WaterToolConfig, WaterToolInput, TOOL_WATER};
use tg_critters_water::water::{BufferSel, WaterPlugin, WorldRaster, GRID};
use bevy_ecs::prelude::Events;

fn ascii(raster: &WorldRaster, layer: BufferSel, cols: usize, rows: usize) -> String {
    let ramp = [' ', '.', ':', 'o', 'O', '#'];
    let buf = raster.layer(layer);
    let (bx, bz) = (GRID / cols, GRID / rows);
    let mut out = String::new();
    for r in 0..rows {
        for c in 0..cols {
            let mut m = 0u8;
            for z in 0..bz {
                for x in 0..bx {
                    m = m.max(buf[(r * bz + z) * GRID + c * bx + x]);
                }
            }
            out.push(ramp[(m as usize * (ramp.len() - 1)) / 255]);
        }
        out.push('\n');
    }
    out
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Time::from_delta(1.0 / 60.0))
        .insert_resource(GladeShape { radius: 60.0 })
        .add_plugins(WaterPlugin);
    let water_brush = app.world().resource::<BrushSizes>().water;
    app.insert_resource(WaterToolConfig { radius_scale: 1.0, half_width_cells: 6.0 * water_brush });
    // an S-curve across the 130-unit window
    let path: Vec<Vec3> = (0..=60)
        .map(|i| {
            let t = i as f32 / 60.0;
            Vec3::new(-50.0 + 100.0 * t, 0.0, 20.0 * (t * std::f32::consts::TAU).sin())
        })
        .collect();
    let mut draw_events = 0;
    for (i, p) in path.iter().enumerate() {
        let phase = if i == 0 {
            StrokePhase::Start
        } else if i + 1 == path.len() {
            StrokePhase::Release
        } else {
            StrokePhase::Update
        };
        app.insert_resource(WaterToolInput { tool_id: TOOL_WATER, phase });
        app.insert_resource(CursorTerrainRaycast(Some(*p)));
        app.update();
        draw_events += app.world_mut().resource_mut::<Events<WaterDrawEvent>>().drain().count();
    }
    app.world_mut().resource_mut::<WorldRaster>().mark_reupload_dirty();

    // water audio grid: 9 cells on a line, camera at the origin; 64 frames fill the rolling window
    app.insert_resource(WaterAudioGrid { cells: (-4..=4).map(|i| WaterAudioGridCell::new(i as f32 * 12.0, 0.0)).collect() });
    app.insert_resource(MainCamera::default());
    app.insert_resource(WaterToolInput::default());
    app.insert_resource(CursorTerrainRaycast(None));
    for _ in 0..64 {
        app.update();
    }

    let raster = app.world().resource::<WorldRaster>();
    println!("WaterDrawEvents: {draw_events}  gpu uploads: {}  published: {}", raster.gpu_upload_count, raster.published);
    println!("cells > 0.5: {} / {}", raster.cells_above(BufferSel::Composite, 0.5), GRID * GRID);
    println!("sample at (0,0): {:.3}   at (0,50): {:.3}", raster.sample_at_world_xz(0.0, 0.0), raster.sample_at_world_xz(0.0, 50.0));
    println!("{}", ascii(raster, BufferSel::Composite, 64, 32));
    let hits: Vec<u32> = app.world().resource::<WaterAudioGrid>().cells.iter().map(|c| c.window_hits()).collect();
    println!("audio grid window hits per cell (x = -48..48 step 12): {hits:?}");
}
