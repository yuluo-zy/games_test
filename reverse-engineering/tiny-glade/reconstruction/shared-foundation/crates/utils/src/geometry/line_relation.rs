//! 原B 0x14261f450/470：LineInterval为端点与闭区间，不仅是两端点。
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
        let r = self.end - self.start;
        let s = other.end - other.start;
        let q = other.start - self.start;
        let denominator = r.x * s.y - r.y * s.x;
        if denominator != 0.0 {
            // 按原先除方向分量再乘相对位置，而非先叉乘再除。
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
}
