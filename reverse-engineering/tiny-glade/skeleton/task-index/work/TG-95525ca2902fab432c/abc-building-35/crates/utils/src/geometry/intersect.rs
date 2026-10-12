//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_intersect::{SphereIntersection, Triangle};
use glam::Vec3;
impl Triangle<[Vec3; 3]> {
    // ORIGINAL: utils::geometry::intersect::Triangle::new
    // 三版对应：A=0x140cee030；B=0x14123ad70；C=0x14147fe40
    // 逻辑组：5282ac00ffee6759f4
    /// 三个输入各12B，按顺序复制到0/12/24，共36B。
    /// 该构造证据补齐上批Triangle<T>点存储，不改points_mut的借用实现。
    /// 使用Vec3，不使用16B的Vec3A；不计算法线或重排顶点。
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self { points: [a, b, c] }
    }
}
