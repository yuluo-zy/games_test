//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_history::{is_decorator_history_diff_empty, DecoratorHistoryDiffView};
pub struct HistoryChainIndex(pub u32);
impl HistoryChainIndex {
    // ORIGINAL: country_core::resources::history::history_chain::HistoryChainIndex::new
    // 三版对应：A=0x140593250；B=0x140833250；C=0x1408fb460
    // 逻辑组：eff1dcc2d7913089e7
    /// 原32位索引恒等封装，不猜当前cursor、不检查历史链长度。
    pub fn new(index: u32) -> Self {
        Self(index)
    }
}
