//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_rectangle::Rectangle2d;
use glam::{Vec2, Vec3};
impl Rectangle2d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::area
    // 三版对应：A=0x1408d1650；B=0x140c99520；C=0x140def4f0
    // 逻辑组：99995990b110d2ed6b
    /// 原size.x*size.y，不取abs、不补正数约束。
    pub fn area(&self) -> f32 {
        self.size.x * self.size.y
    }
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::perimeter
    // 三版对应：A=0x1408d1660；B=0x140c99530；C=0x140deff20
    // 逻辑组：0f43b6c3298d9bc0de
    /// 原先加x/y，随后该和与自身相加，不换成乘法/FMA。
    pub fn perimeter(&self) -> f32 {
        let sum = self.size.x + self.size.y;
        sum + sum
    }
}
/// 三个短体交叉确认32B字段：orientation@0、center_xz@8、size_xz@16、min/max_y@24/28。
/// 作者字段名仍是语义命名；orientation存原两浮点，未增加单位向量约束。
pub struct Rectangle3d {
    pub orientation_xz: Vec2,
    pub center_xz: Vec2,
    pub size_xz: Vec2,
    pub min_y: f32,
    pub max_y: f32,
}
impl Rectangle3d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::extent
    // 三版对应：A=0x1408d0c50；B=0x140c98660；C=0x140df0190
    // 逻辑组：4a1fc2d3663bf71d4c
    /// 返回(width, max_y-min_y, depth)；xz size已是完整尺寸，不乘二。
    pub fn extent(&self) -> Vec3 {
        Vec3::new(self.size_xz.x, self.max_y - self.min_y, self.size_xz.y)
    }
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::translate
    // 三版对应：A=0x1408d0b40；B=0x140c984c0；C=0x140df0400
    // 逻辑组：65fae1cc037b211659
    /// 原地平移center及两条y边界，size/orientation不变。
    /// xz是old+delta，y两项是delta+old，保留原操作数顺序。
    pub fn translate(&mut self, delta: Vec3) {
        self.center_xz = self.center_xz + Vec2::new(delta.x, delta.z);
        self.min_y = delta.y + self.min_y;
        self.max_y = delta.y + self.max_y;
    }
    // ORIGINAL: utils::geometry::rectangle::Rectangle3d::set_orientation_xz
    // 三版对应：A=0x1408d0ba0；B=0x140c98520；C=0x140deffb0
    // 逻辑组：49f5246fbf28d39ea9
    /// 无条件写两个浮点；未调用normalize，不转角度，也不改变尺寸。
    pub fn set_orientation_xz(&mut self, orientation: Vec2) {
        self.orientation_xz = orientation;
    }
}
