//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_wall_id::WallId;
impl WallId {
    // ORIGINAL: country_core::resources::walls::WallId::get_inner
    // 三版对应：A=0x14010da30；B=0x1400885d0；C=0x1400cb570
    // 逻辑组：91fcd5704fcab6da5b
    /// 原64位ID原样返回；不做valid判断或实体表查询。
    pub fn get_inner(&self) -> u64 {
        self.0
    }
}
