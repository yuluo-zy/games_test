//! Relations between parameterized 2D lines and closed intervals.
use glam::Vec2;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineInterval {
    pub start: Vec2,
    pub end: Vec2,
    pub min_t: f32,
    pub max_t: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineRelation {
    Intersection(Vec2),
    Disjoint,
    Collinear,
    Parallel,
}
impl LineInterval {
    pub fn line_segment(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            min_t: 0.0,
            max_t: 1.0,
        }
    }
    pub fn relate(self, other: Self) -> LineRelation {
        if !(self.min_t <= self.max_t && other.min_t <= other.max_t)
            || !(self.start.is_finite()
                && self.end.is_finite()
                && other.start.is_finite()
                && other.end.is_finite())
        {
            return LineRelation::Disjoint;
        }
        let r = self.end - self.start;
        let s = other.end - other.start;
        // A zero-length interval denotes a point, not an arbitrary collinear line.
        if r == Vec2::ZERO {
            return if other.contains_point(self.start) {
                LineRelation::Intersection(self.start)
            } else {
                LineRelation::Disjoint
            };
        }
        if s == Vec2::ZERO {
            return if self.contains_point(other.start) {
                LineRelation::Intersection(other.start)
            } else {
                LineRelation::Disjoint
            };
        }
        let q = other.start - self.start;
        let denominator = r.x * s.y - r.y * s.x;
        if !denominator.is_finite() {
            return LineRelation::Disjoint;
        }
        if denominator != 0.0 {
            // Divide the directions first to preserve established rounding.
            let sd = s / denominator;
            let t = q.x * sd.y - q.y * sd.x;
            let rd = Vec2::new(r.y, r.x) / denominator;
            let u = q.x * rd.x - q.y * rd.y;
            if self.min_t <= t && other.min_t <= u && t <= self.max_t && u <= other.max_t {
                return LineRelation::Intersection(self.start + r * t);
            }
            return LineRelation::Disjoint;
        }
        if q.x * r.y - q.y * r.x != 0.0 {
            LineRelation::Parallel
        } else {
            LineRelation::Collinear
        }
    }
    fn contains_point(self, point: Vec2) -> bool {
        let direction = self.end - self.start;
        let relative = point - self.start;
        if direction == Vec2::ZERO {
            return point == self.start;
        }
        if direction.perp_dot(relative) != 0.0 {
            return false;
        }
        let t = if direction.x.abs() >= direction.y.abs() {
            relative.x / direction.x
        } else {
            relative.y / direction.y
        };
        self.min_t <= t && t <= self.max_t
    }
}
