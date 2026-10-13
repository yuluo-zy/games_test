//! `rasterize_stroke<512>` (0x142261d10): world-space polyline -> zeno commands -> u8 writes.
//!
//! Evidence: water-cpu-analysis "rasterize_stroke<512>", water-extra-analysis 5.
//! zeno internals are a dependency and are NOT recovered: coverage here is a simple
//! capsule-distance anti-aliasing model. UNKNOWN(evidence: water-extra-analysis 5): the stroke
//! width and the exact fixed-point scale of zeno coverage (only `>>9` and `x65536` are known).

use super::raster::{world_to_grid, BufferSel, WorldRaster, GRID};
use glam::{Vec2, Vec3};

/// Fixed-point coverage that maps to alpha 255 (`alpha = coverage >> 9`, clamp 0xff).
/// UNKNOWN(evidence: water-extra-analysis 5): scale assumed so full coverage == 255 << 9.
pub const COVERAGE_ONE: i32 = 255 << 9;

/// Closure bodies inlined in rasterize_write instances (water-extra-analysis 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    /// closure_env$1: `dst = min(dst, 255 - alpha)` (erase)
    EraseMin,
    /// closure_env$0: `dst = max(dst, alpha)` (hard draw)
    Max,
    /// closure_env$2: `dst = saturating_add(dst, alpha * brush >> 16)`
    AddWeighted,
}

/// Dispatch at 0x142262221-0x1422622c8. `erase_flag_zero` = ActiveRasterStroke+0x64 byte == 0,
/// `bit0` = arg7 bit0. (0,0)->env1, (nz,0)->env0, (nz,1)->env2, (0,1)->panic (None).
pub fn select_write_mode(erase_flag_zero: bool, bit0: bool) -> Option<WriteMode> {
    match (erase_flag_zero, bit0) {
        (true, false) => Some(WriteMode::EraseMin),
        (false, false) => Some(WriteMode::Max),
        (false, true) => Some(WriteMode::AddWeighted),
        (true, true) => None,
    }
}

/// `alpha = coverage >> 9` clamped to 0xff (also clamps negatives to 0).
pub fn coverage_to_alpha(coverage: i32) -> u8 {
    (coverage >> 9).clamp(0, 0xff) as u8
}

/// `brush` as 16.16 fixed point: `clamp(brush, 0, 1) * 65536` (0x142262232-0x14226226c).
pub fn brush_fixed(brush: f32) -> u32 {
    (brush.clamp(0.0, 1.0) * 65536.0) as u32
}

/// Apply one closure body to a destination cell.
pub fn apply_write(mode: WriteMode, dst: u8, alpha: u8, brush_fixed: u32) -> u8 {
    match mode {
        WriteMode::Max => dst.max(alpha),
        WriteMode::EraseMin => dst.min(255 - alpha),
        WriteMode::AddWeighted => {
            let add = ((alpha as u32 * brush_fixed) >> 16).min(255) as u8;
            dst.saturating_add(add)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StrokeParams {
    /// UNKNOWN(evidence: water-extra-analysis 5): original stroke width; supplied by the caller.
    pub half_width_cells: f32,
    pub mode: WriteMode,
    /// Brush weight (only AddWeighted uses it).
    pub brush_weight: f32,
}

/// zeno command (move-to / line-to) in grid coordinates (28 B stride in the original).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    MoveTo([f32; 2]),
    LineTo([f32; 2]),
}

/// First point -> MoveTo, the rest LineTo; `grid = (world/130 + 0.5) * 512`.
pub fn build_commands(points: &[Vec3]) -> Vec<Command> {
    points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let g = world_to_grid(p.x, p.z);
            if i == 0 { Command::MoveTo(g) } else { Command::LineTo(g) }
        })
        .collect()
}

fn seg_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let l2 = ab.length_squared();
    let t = if l2 > 0.0 { ((p - a).dot(ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    p.distance(a + ab * t)
}

fn coverage_at(cmds: &[Command], centre: Vec2, half_width: f32) -> i32 {
    let pts: Vec<Vec2> = cmds
        .iter()
        .map(|c| match c {
            Command::MoveTo(g) | Command::LineTo(g) => Vec2::new(g[0], g[1]),
        })
        .collect();
    let d = match pts.len() {
        0 => return 0,
        1 => centre.distance(pts[0]),
        _ => pts.windows(2).map(|w| seg_distance(centre, w[0], w[1])).fold(f32::MAX, f32::min),
    };
    let cov = (half_width + 0.5 - d).clamp(0.0, 1.0);
    (cov * COVERAGE_ONE as f32) as i32
}

impl WorldRaster {
    /// Rasterize a stroke into `layer` (live drawing targets scratch, arg2 = 1).
    pub fn rasterize_stroke(&mut self, points: &[Vec3], layer: BufferSel, p: StrokeParams) {
        let cmds = build_commands(points);
        if cmds.is_empty() {
            return;
        }
        let (mut lo, mut hi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
        for c in &cmds {
            let (Command::MoveTo(g) | Command::LineTo(g)) = c;
            lo = lo.min(Vec2::new(g[0], g[1]));
            hi = hi.max(Vec2::new(g[0], g[1]));
        }
        let m = p.half_width_cells + 1.0;
        let max = GRID as f32 - 1.0;
        let (c0, c1) = ((lo.x - m).floor().clamp(0.0, max) as usize, (hi.x + m).ceil().clamp(0.0, max) as usize);
        let (r0, r1) = ((lo.y - m).floor().clamp(0.0, max) as usize, (hi.y + m).ceil().clamp(0.0, max) as usize);
        let bf = brush_fixed(p.brush_weight);
        let buf = self.layer_mut(layer);
        for r in r0..=r1 {
            for c in c0..=c1 {
                let cov = coverage_at(&cmds, Vec2::new(c as f32 + 0.5, r as f32 + 0.5), p.half_width_cells);
                let alpha = coverage_to_alpha(cov);
                // EraseMin with alpha 0 would be min(dst,255)=dst (no-op); Max/Add 0 are no-ops too.
                if alpha > 0 {
                    let cell = &mut buf[r * GRID + c];
                    *cell = apply_write(p.mode, *cell, alpha, bf);
                }
            }
        }
        self.mark_dirty_world_stroke(points, p.half_width_cells);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(mode: WriteMode) -> StrokeParams {
        StrokeParams { half_width_cells: 3.0, mode, brush_weight: 1.0 }
    }

    #[test]
    fn dispatch_matches_binary() {
        assert_eq!(select_write_mode(true, false), Some(WriteMode::EraseMin));
        assert_eq!(select_write_mode(false, false), Some(WriteMode::Max));
        assert_eq!(select_write_mode(false, true), Some(WriteMode::AddWeighted));
        assert_eq!(select_write_mode(true, true), None);
    }

    #[test]
    fn write_formulas() {
        assert_eq!(apply_write(WriteMode::Max, 10, 200, 0), 200);
        assert_eq!(apply_write(WriteMode::Max, 250, 200, 0), 250);
        assert_eq!(apply_write(WriteMode::EraseMin, 255, 200, 0), 55);
        assert_eq!(apply_write(WriteMode::EraseMin, 20, 200, 0), 20.min(55));
        assert_eq!(apply_write(WriteMode::AddWeighted, 100, 100, 65536), 200);
        assert_eq!(apply_write(WriteMode::AddWeighted, 100, 100, 32768), 150);
        assert_eq!(apply_write(WriteMode::AddWeighted, 200, 200, 65536), 255);
    }

    #[test]
    fn coverage_alpha_and_brush_fixed() {
        assert_eq!(coverage_to_alpha(0x7f << 9), 0x7f);
        assert_eq!(coverage_to_alpha(0x100 << 9), 0xff);
        assert_eq!(coverage_to_alpha(-5), 0);
        assert_eq!(brush_fixed(2.0), 65536);
        assert_eq!(brush_fixed(-1.0), 0);
        assert_eq!(brush_fixed(0.5), 32768);
    }

    #[test]
    fn commands_first_move_rest_line() {
        let c = build_commands(&[Vec3::ZERO, Vec3::new(13.0, 0.0, 0.0)]);
        assert_eq!(c[0], Command::MoveTo([256.0, 256.0]));
        assert_eq!(c[1], Command::LineTo([256.0 + 0.1 * 512.0, 256.0]));
        assert!(build_commands(&[]).is_empty());
    }

    #[test]
    fn stroke_paints_line_and_dirty() {
        let mut r = WorldRaster::new();
        r.rasterize_stroke(&[Vec3::new(-10.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)], BufferSel::Scratch, params(WriteMode::Max));
        assert_eq!(r.scratch[256 * GRID + 256], 255);
        assert_eq!(r.composite[256 * GRID + 256], 0, "composite untouched");
        assert_eq!(r.scratch[0], 0);
        assert!(r.dirty_tile_count() > 0);
    }

    #[test]
    fn erase_removes_water() {
        let mut r = WorldRaster::new();
        r.composite.fill(255);
        r.rasterize_stroke(&[Vec3::ZERO], BufferSel::Composite, params(WriteMode::EraseMin));
        assert_eq!(r.composite[256 * GRID + 256], 0);
        assert_eq!(r.composite[0], 255);
    }

    #[test]
    fn empty_stroke_noop() {
        let mut r = WorldRaster::new();
        r.rasterize_stroke(&[], BufferSel::Scratch, params(WriteMode::Max));
        assert_eq!(r.dirty_tile_count(), 0);
    }

    #[test]
    fn stroke_near_border_does_not_panic() {
        let mut r = WorldRaster::new();
        r.rasterize_stroke(&[Vec3::new(-200.0, 0.0, 200.0), Vec3::new(200.0, 0.0, 200.0)], BufferSel::Scratch, params(WriteMode::Max));
    }
}
