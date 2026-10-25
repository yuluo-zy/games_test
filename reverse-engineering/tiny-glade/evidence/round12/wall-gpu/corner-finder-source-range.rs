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

