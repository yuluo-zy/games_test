//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct SphereIntersection {
    pub first: f32,
    pub second: f32,
}
impl SphereIntersection {
    // ORIGINAL: utils::geometry::intersect::SphereIntersection::closest_intersection_distance
    // 三版对应：A=0x140cee4d0；B=0x14123b200；C=0x14147fa50
    // 逻辑组：b30b1eb213a935e4a2
    /// 原 cmpunord/minss 组合忽略单边 NaN；相等时选 first，保留其零符号。
    /// 两边都是 NaN 时返回 second 的位模式，不增加正距离筛选。
    pub fn closest_intersection_distance(&self) -> f32 {
        if self.first.is_nan() {
            self.second
        } else if self.second < self.first {
            self.second
        } else {
            self.first
        }
    }
}
/// points_mut 的安全投影；原四字节指令只证指针恒等。
/// 实际点存储类型由调用方提供，不根据 Triangle 名字补造 Vec3 数组或完整布局。
pub struct Triangle<T> {
    pub points: T,
}
impl<T> Triangle<T> {
    // ORIGINAL: utils::geometry::intersect::Triangle::points_mut
    // 三版对应：A=0x14043c290；B=0x14067dd80；C=0x1406ca440
    // 逻辑组：3d44453e62b6e606e0
    /// 借用原有顶点存储，不分配或复制。
    pub fn points_mut(&mut self) -> &mut T {
        &mut self.points
    }
}
