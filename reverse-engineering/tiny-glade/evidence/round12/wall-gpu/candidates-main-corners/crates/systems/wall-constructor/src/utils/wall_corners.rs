//! Original wall-corner index policy and the still-unrecovered geometry owner.
//! Finder proof: original141215683..141215919 with its closed-boundary branch
//! at1412170bc. Complete add_wall_corners is9427B, not just this finder.
use glam::{Vec2, Vec3};

pub const CORNER_ANGLE_MIN: f32 = f32::from_bits(0xbe31d0d5);
pub const CORNER_ANGLE_MAX: f32 = f32::from_bits(0x3e31d0d7);

/// Native compares the incoming and outgoing directions. Preserve the product
/// of squared lengths before sqrt; two independent vector normalizations use a
/// different floating-point algorithm at the threshold.
pub fn check_angle(previous: Vec2, current: Vec2, next: Vec2) -> bool {
    let a = next - current;
    let b = current - previous;
    let numerator = a.y * b.y + a.x * b.x;
    let denominator = ((a.x * a.x + a.y * a.y) * (b.x * b.x + b.y * b.y)).sqrt();
    let dot = numerator / denominator;
    dot.to_bits() & 0x7fff_ffff <= 0x7f7f_ffff
        && CORNER_ANGLE_MIN <= dot
        && dot < CORNER_ANGLE_MAX
}

/// Original paired scan, including the closed wall's point0. Closed outlines
/// contain a repeated final point; the seam's previous point is len-2.
pub fn find_corner_indices(points: &[Vec2], closed: bool) -> Vec<usize> {
    let mut out = Vec::new();
    if closed {
        assert!(points.len() >= 2, "closed corner outline needs its repeated endpoint");
        if check_angle(points[points.len()-2], points[0], points[1]) { out.push(0); }
    }
    let mut cursor = 0usize;
    let mut previous = 0usize;
    let mut current = 0usize;
    let mut first = true;
    let mut ordinal = 0usize;
    while cursor != points.len() {
        let mut next = cursor + 1;
        if first {
            if next == points.len() || cursor + 2 == points.len() { break; }
            previous = cursor;
            current = next;
            cursor += 2;
            next = cursor + 1;
        }
        if check_angle(points[previous], points[current], points[cursor]) {
            ordinal += 1;
            previous = current;
            current = cursor;
            cursor = next;
        } else {
            ordinal += 2;
            loop {
                if next == points.len() { return out; }
                if check_angle(points[current], points[cursor], points[next]) { break; }
                ordinal += 1;
                current = cursor;
                cursor = next;
                next += 1;
            }
            previous = cursor;
            current = next;
            cursor = next + 1;
        }
        out.push(ordinal);
        first = false;
    }
    out
}

/// Open-polyline API retained; actual full constructor must pass its real shape
/// to find_corner_indices rather than guessing closed from endpoint equality.
pub fn find_corners(points: &[Vec2]) -> Vec<usize> {
    find_corner_indices(points, false)
}

/// Full B141215540 geometry/overlap/96B writer still needs its complete typed
/// PublicWallState, both height slices, WIP holes and constructor options.
/// The old points-only signature cannot represent that original contract.
pub fn add_wall_corners_new(points: &[Vec3], _out: &mut Vec<[u8; 96]>) {
    let _ = points;
    todo!("complete add_wall_corners geometry, holes and96B emission remain unrecovered")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_missed_window_does_not_skip_the_following_corner() {
        let p=[Vec2::new(0.,0.),Vec2::new(1.,0.),Vec2::new(2.,0.),Vec2::new(2.,1.),Vec2::new(3.,1.),Vec2::new(3.,2.)];
        assert_eq!(find_corners(&p),[2,3,4]);
    }
    #[test]
    fn closed_seam_uses_the_penultimate_repeated_outline_point() {
        let p=[Vec2::ZERO,Vec2::X,Vec2::ONE,Vec2::Y,Vec2::ZERO];
        assert_eq!(find_corner_indices(&p,true),[0,1,2,3]);
        assert_eq!(find_corner_indices(&p,false),[1,2,3]);
    }
}
