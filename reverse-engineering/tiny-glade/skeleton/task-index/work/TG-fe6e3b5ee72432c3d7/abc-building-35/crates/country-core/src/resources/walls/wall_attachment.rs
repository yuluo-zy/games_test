//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_attachment::WallCornerAttachment;
impl Default for WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::impl$1::default
    // 三版对应：A=0x140003390；B=0x140002370；C=0x140104100
    // 逻辑组：42eee1f8665166a000
    /// 原返回0，默认无附着；其他tag的完整枚举未扩充。
    fn default() -> Self {
        Self { observed_tag: 0 }
    }
}
impl WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
    // 三版对应：A=0x14066a760；B=0x140b256f0；C=0x140af95d0
    // 逻辑组：cc01a94107a5a74754
    /// tag=0返回None；非零返回零扩展tag-1的u64索引。
    /// 原None仍算出rdx=u64::MAX，但该payload无语义，安全Option不制造它。
    pub fn to_rectangle_corner_id(self) -> Option<u64> {
        if self.observed_tag == 0 {
            None
        } else {
            Some(u64::from(self.observed_tag) - 1)
        }
    }
}
