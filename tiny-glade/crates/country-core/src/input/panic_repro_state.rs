//! `panic_repro_state.rs` — `PanicReproState::is_inactive`（B 0x140833230：`cmp byte [rcx], 0; sete al`）。
//!
//! 状态首字节为 0 即 inactive（17 个调用点用它门控调试用的 panic 复现输入路径，I）。其余变体的含义未恢复，
//! 所以这里只保留判别字节。

/// 判别字节。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanicReproState(pub u8);

impl PanicReproState {
    /// 0x140833230。
    pub fn is_inactive(self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_zero_tag_is_inactive() {
        assert!(PanicReproState(0).is_inactive());
        assert!(PanicReproState::default().is_inactive());
        assert!(!PanicReproState(1).is_inactive());
        assert!(!PanicReproState(255).is_inactive());
    }
}
