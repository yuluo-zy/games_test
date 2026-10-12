//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
/// 现有roof_components已将动画保存为8B，两f32解释由其advance/删除消费者支持。
pub struct RoofAnimation {
    pub first: f32,
    pub second: f32,
}
impl RoofAnimation {
    // ORIGINAL: country_core::resources::roofs::roof_animation::RoofAnimation::new
    // 三版对应：A=0x140ec3870；B=0x1408dbc40；C=0x1409e1790
    // 逻辑组：420b73e1528c2f128d
    /// 两返回通道均为正零；不设为1、不附加动画事件或history。
    pub fn new() -> Self {
        Self {
            first: 0.0,
            second: 0.0,
        }
    }
}
