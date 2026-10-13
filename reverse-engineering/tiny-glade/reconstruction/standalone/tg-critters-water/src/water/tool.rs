//! `draw_water` tool branch (0x141ba9130) and `draw_raster<Water>` state machine (0x142260ae0).
//! Evidence: water-extra-analysis 1, water-cpu-analysis "draw_water / draw_raster<Water>".

use super::raster::{BufferSel, WorldRaster};
use super::stroke::{select_write_mode, StrokeParams};
use crate::env::{CursorTerrainRaycast, DemoMode, GladeShape};
use bevy_ecs::prelude::*;
use glam::Vec3;
use smart_default::SmartDefault;

/// `BrushSizes::set_water` (0x1409f18a0) clamp range and `reset_water` default (0x1409f1890).
pub const WATER_BRUSH_MIN: f32 = 1.0;
pub const WATER_BRUSH_MAX: f32 = 5.0;
pub const WATER_BRUSH_DEFAULT: f32 = 2.0;
/// Min cursor travel before a point is appended (`InProgressRasterStroke::last` > ~0.0005).
pub const ADD_POINT_MIN_DIST: f32 = 0.0005;

/// Tool ids that reach the water code: id in [6,21]; `id-5 == 4` (id 9) eraser, `id-5 == 0xa` (id 15) water.
pub const TOOL_ERASER: u32 = 9;
pub const TOOL_WATER: u32 = 15;

/// `BrushSizes` resource (`water` @+4).
#[derive(Resource, SmartDefault, Clone, Copy, Debug)]
pub struct BrushSizes {
    #[default(WATER_BRUSH_DEFAULT)]
    pub water: f32,
}

impl BrushSizes {
    pub fn set_water(&mut self, v: f32) {
        self.water = v.clamp(WATER_BRUSH_MIN, WATER_BRUSH_MAX);
    }
    pub fn reset_water(&mut self) {
        self.water = WATER_BRUSH_DEFAULT;
    }
}

/// `ActiveRasterStroke` subset. UNKNOWN(evidence: water-extra-analysis 关键 unknown): the full layout
/// (+0x60/+0x64/+0x68/+0x6c names) is not recovered; only the two flags that select the write mode.
#[derive(Resource, Clone, Debug, Default)]
pub struct ActiveRasterStroke {
    pub points: Vec<Vec3>,
    /// byte @+0x64: non-zero = draw, zero = erase (selects closure env0/env2 vs env1).
    pub draw_flag: bool,
    /// bit0 of u32 @+0x68 (weighted-add path).
    pub bit0: bool,
}

/// Stroke state read via `ActiveRasterStroke::index_mut` (0 = start, 1 = in progress, else release).
/// UNKNOWN(evidence: water-cpu-analysis draw_raster): which input system drives the transitions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StrokePhase {
    #[default]
    Start,
    Update,
    Release,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterEvent {
    WaterDraw,
    WaterErase,
}

/// `WaterDrawEvent` (EventWriter of draw_raster / replay_raster).
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaterDrawEvent;

/// Erase counterpart (`+0x64 == 0` at the tail of draw_raster).
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaterEraseEvent;

/// Tool / stroke input read by draw_water: the selected tool id (`*in_RCX`) and the
/// `ActiveRasterStroke::index_mut` state. UNKNOWN(evidence: water-cpu-analysis draw_raster): which
/// input system drives the phase transitions.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaterToolInput {
    pub tool_id: u32,
    pub phase: StrokePhase,
}

/// UNKNOWN(evidence: water-extra-analysis 1/5): the constant at 0x142a16cb8 (circle radius per brush
/// unit) and the zeno stroke width.
#[derive(Resource, SmartDefault, Clone, Copy, Debug, PartialEq)]
pub struct WaterToolConfig {
    #[default(1.0)]
    pub radius_scale: f32,
    #[default(6.0)]
    pub half_width_cells: f32,
}

/// Result of the last `draw_water` run (the original reports through events / UI state).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct LastDrawWater(pub DrawWaterOutcome);

/// One `draw_raster<Water>` invocation. Returns events emitted this call.
/// `brush` is BrushSizes.water; `half_width_cells`: UNKNOWN(evidence: water-extra-analysis 5).
pub fn draw_raster(
    raster: &mut WorldRaster,
    stroke: &mut ActiveRasterStroke,
    phase: StrokePhase,
    cursor: Vec3,
    half_width_cells: f32,
    brush: f32,
) -> Vec<RasterEvent> {
    let mut events = Vec::new();
    match phase {
        StrokePhase::Start => {
            stroke.points.clear();
            stroke.points.push(cursor);
        }
        StrokePhase::Update => {
            let moved = stroke.points.last().is_none_or(|l| l.distance(cursor) > ADD_POINT_MIN_DIST);
            if moved {
                stroke.points.push(cursor);
                // resample_and_smooth: UNKNOWN(evidence: water-cpu-analysis draw_raster) algorithm; identity.
                raster.dirty = [0; super::raster::DIRTY_WORDS];
                raster.copy_composite_to_scratch();
                if let Some(mode) = select_write_mode(!stroke.draw_flag, stroke.bit0) {
                    raster.rasterize_stroke(
                        &stroke.points,
                        BufferSel::Scratch,
                        StrokeParams { half_width_cells, mode, brush_weight: brush },
                    );
                }
                // UNKNOWN(evidence: water-cpu-analysis copy_to_staging): selector value passed by callers.
                raster.copy_to_staging(BufferSel::Scratch);
            }
        }
        StrokePhase::Release => {
            raster.commit_scratch_to_composite();
            raster.copy_to_staging(BufferSel::Composite);
            events.push(if stroke.draw_flag { RasterEvent::WaterDraw } else { RasterEvent::WaterErase });
            stroke.points.clear();
        }
    }
    events
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DrawWaterOutcome {
    /// Not a water tool id / demo mode / no cursor.
    #[default]
    Ignored,
    /// Eraser tool: only a signifier hint (`f = 4*lerp + (1-lerp)` computed by the UI; not modelled).
    EraserHint,
    /// Cursor outside the glade shape: nothing drawn.
    OutsideGlade,
    Drawn { radius: f32, events: Vec<RasterEvent> },
}

pub struct DrawWaterInput {
    pub tool_id: u32,
    pub demo_mode: bool,
    pub cursor: Option<Vec3>,
    pub phase: StrokePhase,
    /// UNKNOWN(evidence: water-extra-analysis 1): the constant at 0x142a16cb8 (circle radius per brush unit).
    pub radius_scale: f32,
    pub half_width_cells: f32,
}

/// `draw_water` water branch. NaN brush is rejected (the 0x7fffffff / 0x7f7fffff check);
/// `GladeBorder::is_shape_inside(Circle2d::new(cursor, radius_scale * brush))` gates drawing.
pub fn draw_water_step(
    raster: &mut WorldRaster,
    stroke: &mut ActiveRasterStroke,
    brushes: &BrushSizes,
    glade: &GladeShape,
    input: &DrawWaterInput,
) -> DrawWaterOutcome {
    if input.demo_mode || !(6..=21).contains(&input.tool_id) {
        return DrawWaterOutcome::Ignored;
    }
    match input.tool_id {
        TOOL_ERASER => return DrawWaterOutcome::EraserHint,
        TOOL_WATER => {}
        _ => return DrawWaterOutcome::Ignored,
    }
    let brush = brushes.water;
    let Some(cursor) = input.cursor else { return DrawWaterOutcome::Ignored };
    if brush.is_nan() {
        return DrawWaterOutcome::Ignored;
    }
    let radius = input.radius_scale * brush;
    if !glade.is_shape_inside(cursor, radius) {
        return DrawWaterOutcome::OutsideGlade;
    }
    stroke.draw_flag = true;
    let events = draw_raster(raster, stroke, input.phase, cursor, input.half_width_cells, brush);
    DrawWaterOutcome::Drawn { radius, events }
}

/// `draw_water` (0x141ba9130) as a system: tool input, BrushSizes, cursor raycast, glade border,
/// the WaterRaster and the active stroke; emits `WaterDrawEvent` / `WaterEraseEvent`.
#[allow(clippy::too_many_arguments)]
pub fn draw_water(
    input: Res<WaterToolInput>,
    demo: Res<DemoMode>,
    brushes: Res<BrushSizes>,
    cursor: Res<CursorTerrainRaycast>,
    glade: Res<GladeShape>,
    config: Res<WaterToolConfig>,
    mut raster: ResMut<WorldRaster>,
    mut stroke: ResMut<ActiveRasterStroke>,
    mut last: ResMut<LastDrawWater>,
    mut draws: EventWriter<WaterDrawEvent>,
    mut erases: EventWriter<WaterEraseEvent>,
) {
    let step = DrawWaterInput {
        tool_id: input.tool_id,
        demo_mode: demo.0,
        cursor: cursor.0,
        phase: input.phase,
        radius_scale: config.radius_scale,
        half_width_cells: config.half_width_cells,
    };
    let outcome = draw_water_step(&mut raster, &mut stroke, &brushes, &glade, &step);
    if let DrawWaterOutcome::Drawn { events, .. } = &outcome {
        for e in events {
            match e {
                RasterEvent::WaterDraw => {
                    draws.write(WaterDrawEvent);
                }
                RasterEvent::WaterErase => {
                    erases.write(WaterEraseEvent);
                }
            }
        }
    }
    last.0 = outcome;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::base_app;
    use bevy_app::{App, Update};

    fn setup() -> App {
        let mut app = base_app(0.016);
        app.init_resource::<BrushSizes>()
            .init_resource::<ActiveRasterStroke>()
            .init_resource::<WaterToolInput>()
            .init_resource::<WaterToolConfig>()
            .init_resource::<LastDrawWater>()
            .insert_resource(WaterToolConfig { radius_scale: 1.0, half_width_cells: 3.0 })
            .add_systems(Update, draw_water);
        app
    }

    fn step(app: &mut App, tool: u32, phase: StrokePhase, cursor: Option<Vec3>) -> DrawWaterOutcome {
        app.insert_resource(WaterToolInput { tool_id: tool, phase });
        app.insert_resource(CursorTerrainRaycast(cursor));
        app.update();
        app.world().resource::<LastDrawWater>().0.clone()
    }

    fn event_count<E: Event>(app: &mut App) -> usize {
        app.world_mut().resource_mut::<Events<E>>().drain().count()
    }

    #[test]
    fn brush_clamp_and_reset() {
        let mut b = BrushSizes::default();
        assert_eq!(b.water, 2.0);
        b.set_water(99.0);
        assert_eq!(b.water, 5.0);
        b.set_water(0.1);
        assert_eq!(b.water, 1.0);
        b.reset_water();
        assert_eq!(b.water, 2.0);
    }

    #[test]
    fn tool_dispatch() {
        let mut app = setup();
        let o = Vec3::ZERO;
        assert_eq!(step(&mut app, 5, StrokePhase::Start, Some(o)), DrawWaterOutcome::Ignored);
        assert_eq!(step(&mut app, 22, StrokePhase::Start, Some(o)), DrawWaterOutcome::Ignored);
        assert_eq!(step(&mut app, 7, StrokePhase::Start, Some(o)), DrawWaterOutcome::Ignored);
        assert_eq!(step(&mut app, TOOL_ERASER, StrokePhase::Start, Some(o)), DrawWaterOutcome::EraserHint);
        assert_eq!(step(&mut app, TOOL_WATER, StrokePhase::Start, None), DrawWaterOutcome::Ignored);
        app.world_mut().resource_mut::<BrushSizes>().water = f32::NAN;
        assert_eq!(step(&mut app, TOOL_WATER, StrokePhase::Start, Some(o)), DrawWaterOutcome::Ignored);
        let mut demo = setup();
        demo.insert_resource(DemoMode(true));
        assert_eq!(step(&mut demo, TOOL_WATER, StrokePhase::Start, Some(o)), DrawWaterOutcome::Ignored);
    }

    #[test]
    fn outside_glade_draws_nothing() {
        let mut app = setup();
        app.insert_resource(GladeShape { radius: 1.0 });
        assert_eq!(step(&mut app, TOOL_WATER, StrokePhase::Start, Some(Vec3::new(10.0, 0.0, 0.0))), DrawWaterOutcome::OutsideGlade);
        assert!(app.world().resource::<ActiveRasterStroke>().points.is_empty());
    }

    #[test]
    fn full_stroke_lifecycle_start_update_release() {
        let mut app = setup();
        let p = |x: f32| Some(Vec3::new(x, 0.0, 0.0));
        step(&mut app, TOOL_WATER, StrokePhase::Start, p(-5.0));
        let out = step(&mut app, TOOL_WATER, StrokePhase::Update, p(5.0));
        assert_eq!(out, DrawWaterOutcome::Drawn { radius: 2.0, events: vec![] });
        // live: scratch holds the stroke, composite does not
        {
            let r = app.world().resource::<WorldRaster>();
            assert_eq!(r.scratch[256 * 512 + 256], 255);
            assert_eq!(r.composite[256 * 512 + 256], 0);
            assert_eq!(r.read_select, BufferSel::Scratch);
        }
        assert_eq!(event_count::<WaterDrawEvent>(&mut app), 0);
        let out = step(&mut app, TOOL_WATER, StrokePhase::Release, p(5.0));
        assert_eq!(out, DrawWaterOutcome::Drawn { radius: 2.0, events: vec![RasterEvent::WaterDraw] });
        let r = app.world().resource::<WorldRaster>();
        assert_eq!(r.composite[256 * 512 + 256], 255);
        assert_eq!(r.read_select, BufferSel::Composite);
        assert!(r.published);
        assert_eq!(event_count::<WaterDrawEvent>(&mut app), 1);
        assert_eq!(event_count::<WaterEraseEvent>(&mut app), 0);
    }

    #[test]
    fn tiny_move_is_ignored() {
        let (mut r, mut s) = (WorldRaster::new(), ActiveRasterStroke { draw_flag: true, ..Default::default() });
        draw_raster(&mut r, &mut s, StrokePhase::Start, Vec3::ZERO, 3.0, 2.0);
        draw_raster(&mut r, &mut s, StrokePhase::Update, Vec3::new(0.0001, 0.0, 0.0), 3.0, 2.0);
        assert_eq!(s.points.len(), 1);
        assert!(!r.published);
    }

    #[test]
    fn erase_flag_release_event_and_erase_mode() {
        let (mut r, mut s) = (WorldRaster::new(), ActiveRasterStroke { draw_flag: false, ..Default::default() });
        r.composite.fill(255);
        draw_raster(&mut r, &mut s, StrokePhase::Start, Vec3::new(-5.0, 0.0, 0.0), 3.0, 2.0);
        draw_raster(&mut r, &mut s, StrokePhase::Update, Vec3::new(5.0, 0.0, 0.0), 3.0, 2.0);
        let ev = draw_raster(&mut r, &mut s, StrokePhase::Release, Vec3::ZERO, 3.0, 2.0);
        assert_eq!(ev, vec![RasterEvent::WaterErase]);
        assert_eq!(r.composite[256 * 512 + 256], 0);
    }
}
