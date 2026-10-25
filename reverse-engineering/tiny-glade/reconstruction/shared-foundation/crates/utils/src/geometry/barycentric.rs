//! B 0x1416c2f20；退化分母不补fallback，保留IEEE结果。
use glam::{Vec2, Vec3};
/// B inlinee 0x1284c inside 0x1416f34a0. Signed determinant; no degenerate fallback.
pub fn barycentric2(point: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Vec3 {
    let ab = b - a;
    let ac = c - a;
    let d = point - a;
    let reciprocal = 1.0 / (ab.x * ac.y - ab.y * ac.x);
    let v = reciprocal * (ac.y * d.x - ac.x * d.y);
    let w = reciprocal * (d.y * ab.x - d.x * ab.y);
    Vec3::new((1.0 - v) - w, v, w)
}
pub fn barycentric(point: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = point - a;
    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);
    let denominator = d11 * d00 - d01 * d01;
    let w = (d00 * d21 - d01 * d20) / denominator;
    let v = (d11 * d20 - d01 * d21) / denominator;
    Vec3::new((1.0 - v) - w, v, w)
}
