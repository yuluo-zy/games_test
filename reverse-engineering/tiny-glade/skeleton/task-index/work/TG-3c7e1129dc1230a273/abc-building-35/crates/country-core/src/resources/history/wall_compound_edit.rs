//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub struct CompoundEditBuilder;
impl CompoundEditBuilder {
    // ORIGINAL: country_core::resources::history::wall_compound_edit::CompoundEditBuilder::new_edit_sequence
    // 三版对应：A=0x14056c370；B=0x140a82b20；C=0x140b8f4d0
    // 逻辑组：b9adf60cea74abe3bc
    /// 原24B返回是empty Vec：capacity=0、dangling align=8、len=0。
    /// 直接复用std Vec，不实现原分配/指针/容量布局；T由真实command owner确定。
    /// 创建空序列不等于start/commit/cursor推进。
    pub fn new_edit_sequence<T>() -> Vec<T> {
        Vec::new()
    }
}
