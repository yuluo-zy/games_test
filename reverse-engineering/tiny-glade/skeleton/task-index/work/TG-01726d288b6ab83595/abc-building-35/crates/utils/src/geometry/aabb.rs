//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_aabb::{Aabb2, Aabb3};
use glam::Vec2;
impl Aabb2 {
    // ORIGINAL: utils::geometry::aabb::Aabb2::width
    // 三版对应：A=0x140efbd80；B=0x1412402e0；C=0x140deac50
    // 逻辑组：ed860a691f6149bb90
    /// max.x-min.x，不取绝对值。
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    // ORIGINAL: utils::geometry::aabb::Aabb2::height
    // 三版对应：A=0x140efbd90；B=0x1412402f0；C=0x140deac90
    // 逻辑组：336c33ee2bee13c4e0
    /// max.y-min.y；不能复用其他模块中仅同名的height。
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
    // ORIGINAL: utils::geometry::aabb::Aabb2::contains
    // 三版对应：A=0x140efbbe0；B=0x141240140；C=0x140deaca0
    // 逻辑组：b3da256f0fca954273
    /// 按原x下界/x上界/y下界/y上界顺序作ordered比较；包含边界，NaN为false。
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && self.max.x >= point.x
            && point.y >= self.min.y
            && self.max.y >= point.y
    }
}
