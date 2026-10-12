//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
/// 两字段的借用视图。真实容器沿用现有archivist owner，不重写HashMap或collision句柄。
/// 原offset为+96/+144，具体作者容器类型仍由真实producer/consumer提供。
pub struct DecoratorArchivist<'a, C, I> {
    pub colliders: &'a C,
    pub interactions: &'a I,
}
impl<'a, C, I> DecoratorArchivist<'a, C, I> {
    // ORIGINAL: country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders
    // 三版对应：A=0x1400507f0；B=0x140005c60；C=0x140a4d2d0
    // 逻辑组：13210ea699cee8fc85
    /// getter借用+96对象；不清空、克隆或移动，原只有LEA。
    pub fn decorator_colliders(&self) -> &'a C {
        self.colliders
    }
    // ORIGINAL: country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions
    // 三版对应：A=0x140012e10；B=0x1403c03c0；C=0x140a4d2e0
    // 逻辑组：fa04f2f586a801f217
    /// getter借用+144对象；两种容器独立，不猜作同一个碰撞表。
    pub fn decorator_interactions(&self) -> &'a I {
        self.interactions
    }
}
