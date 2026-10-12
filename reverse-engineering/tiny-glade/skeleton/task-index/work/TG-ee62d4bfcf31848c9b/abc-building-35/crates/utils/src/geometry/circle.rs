//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_circle::{Circle2d, CirclePoints2};
use glam::Vec2;
impl Circle2d {
    // ORIGINAL: utils::geometry::circle::Circle2d::contains_point
    // 三版对应：A=0x1408d4f80；B=0x140c980f0；C=0x140deb490
    // 逻辑组：d656094b63401ac3f5
    /// 严格比较radius² > y²+x²，不开平方；不能委托is_point_inside。
    /// 负半径被平方，超大半径可溢出；保留与距离版本不同的边界行为。
    pub fn contains_point(&self, point: Vec2) -> bool {
        let d = point - self.center;
        let squares = d * d;
        let distance_squared = squares.y + squares.x;
        let radius_squared = self.radius * self.radius;
        radius_squared > distance_squared
    }
}
