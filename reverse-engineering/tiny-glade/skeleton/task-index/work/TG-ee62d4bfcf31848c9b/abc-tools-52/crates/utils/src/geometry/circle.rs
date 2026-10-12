//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
use glam::Vec2;
pub struct Circle2d {
    pub center: Vec2,
    pub radius: f32,
}
/// 只恢复构造器，不增加未取证的三角采样 Iterator::next。
pub struct CirclePoints2<'a> {
    pub circle: &'a Circle2d,
    pub point_count: usize,
    pub front: usize,
    pub back: usize,
}
impl Circle2d {
    // ORIGINAL: utils::geometry::circle::Circle2d::signed_distance
    // 三版对应：A=0x1408d4fb0；B=0x140c98120；C=0x140deb6e0
    // 逻辑组：dc22ed40aa5fe71bc0
    /// 原指令按 y²+x² 求平方根后减半径，不改用 hypot 或平方距离近似。
    pub fn signed_distance(&self, point: Vec2) -> f32 {
        let d = point - self.center;
        let squares = d * d;
        (squares.y + squares.x).sqrt() - self.radius
    }
    // ORIGINAL: utils::geometry::circle::Circle2d::is_point_inside
    // 三版对应：A=0x1408d4af0；B=0x140c97b10；C=0x140deb6b0
    // 逻辑组：027df79607c0821d4c
    /// 严格小于半径；圆周上返回 false，任一比较操作数为 NaN 也返回 false。
    pub fn is_point_inside(&self, point: Vec2) -> bool {
        let d = point - self.center;
        let squares = d * d;
        self.radius > (squares.y + squares.x).sqrt()
    }
    // ORIGINAL: utils::geometry::circle::Circle2d::as_points2_ptcount
    // 三版对应：A=0x1408d4b80；B=0x140c97ba0；C=0x140deb870
    // 逻辑组：3a66e6b7a602f40098
    /// 原构造器仅写 circle 指针、count、front=0、back=count；不分配点数组。
    pub fn as_points2_ptcount(&self, point_count: usize) -> CirclePoints2<'_> {
        CirclePoints2 {
            circle: self,
            point_count,
            front: 0,
            back: point_count,
        }
    }
}
