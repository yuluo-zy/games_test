//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
/// 原返回EAX/EDX两槽投影；不能仅凭movl判断作者字段/enum的完整布局。
pub struct DecoratorLibKey {
    pub return_eax: u32,
    pub return_edx: u32,
}
impl DecoratorLibKey {
    // ORIGINAL: country_core::resources::walls::decorator_subtype::DecoratorLibKey::new
    // 三版对应：A=0x140633130；B=0x140b25c90；C=0x140c579b0
    // 逻辑组：fc4a4f93ae20a5ea5c
    /// EAX=第二输入，EDX=第一输入。C真实caller将AL/DL写到连续两byte，
    /// 可对应现有游戏key的subtype/rank低byte，但完整输入类型/ABI仍待更多caller核对。
    /// 保留32位原观察，不直接改为已有u8结构，也不将字段顺序按名字猜反。
    pub fn new(arg_ecx: u32, arg_edx: u32) -> Self {
        Self {
            return_eax: arg_edx,
            return_edx: arg_ecx,
        }
    }
}
