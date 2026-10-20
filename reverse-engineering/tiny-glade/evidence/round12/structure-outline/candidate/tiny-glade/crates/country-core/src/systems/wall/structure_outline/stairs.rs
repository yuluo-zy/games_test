//! Original structure_outline closure1 (14095fca0/2474B).
//! The full assembly owner supplies these already-owned arrays; this leaf does
//! not manufacture an empty startup resource or claim the native336B ABI.
use glam::{Vec2, Vec3};
use crate::utils::debug_viz::{DebugColor, DebugLine};

pub struct StairOutlineInput<'a> {
    pub path: &'a [Vec2],
    pub heights: &'a [f32],
    pub widths: &'a [f32],
    pub directions: &'a [Vec2],
}
fn line(points: Vec<Vec3>, strength: f32, red: bool) -> DebugLine {
    DebugLine { points, colors: None,
        color: if red { DebugColor::Red } else { DebugColor::White },
        line_width: strength, ui_pass: true, dashed: true }
}
pub fn stair_outline_lines(input: StairOutlineInput<'_>, kind: u8, strength: f32) -> Vec<DebugLine> {
    let count = input.path.len().min(input.heights.len()).min(input.widths.len()).min(input.directions.len());
    let mut points = Vec::with_capacity(input.path.len() * 2);
    if kind == 2 {
        for i in 0..count {
            points.push(Vec3::new(input.path[i].x, input.heights[i], input.path[i].y));
        }
    } else {
        for i in 0..count {
            let width = input.widths[i] + 0.3;
            let direction = input.directions[i];
            let reciprocal = 1.0 / (direction.x * direction.x + direction.y * direction.y).sqrt();
            let normal = if reciprocal.is_finite() && reciprocal > 0.0 {
                Vec2::new(-direction.y * reciprocal, direction.x * reciprocal)
            } else { Vec2::ZERO };
            points.push(Vec3::new(
                normal.x * width * 0.5 + input.path[i].x,
                width * 0.0 * 0.5 + input.heights[i],
                normal.y * width * 0.5 + input.path[i].y));
        }
        for i in (0..count).rev() {
            let width = input.widths[i] + 0.3;
            let direction = input.directions[i];
            let reciprocal = 1.0 / (direction.x * direction.x + direction.y * direction.y).sqrt();
            let normal = if reciprocal.is_finite() && reciprocal > 0.0 {
                Vec2::new(-direction.y * reciprocal, direction.x * reciprocal)
            } else { Vec2::ZERO };
            points.push(Vec3::new(
                input.path[i].x + normal.x * width * (-0.5),
                width * 0.0 * (-0.5) + input.heights[i],
                input.path[i].y + normal.y * width * (-0.5)));
        }
        // Original bounds failure on an empty ordinary perimeter is retained.
        points.push(points[0]);
    }
    let mut output = vec![line(points, strength, kind == 1)];
    if kind == 1 {
        // This independent second zip intentionally ignores width/dir counts.
        let center = input.path.iter().zip(input.heights).map(|(p, &h)| Vec3::new(p.x, h, p.y)).collect();
        output.push(line(center, strength, true));
    }
    output
}
