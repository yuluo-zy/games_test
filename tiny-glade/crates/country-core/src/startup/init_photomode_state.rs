//! `init_photomode_state.rs` — `PhotomodeState::on_enable_photomode`（B 0x140870570，汇编逐条核对）。
//!
//! 写入：`scalar f32 @+0 = if demo { 1.0 } else { 3.0 }`（0x142925904 / 0x1429cd078，demo 开关 0x143634d90）、
//! `u32 @+4 = 0xffffffff`、字节 `+8`、`+0xa`、`+0xc` 清零。此前文档写成字节 `+10`、`+11`，与汇编不符。
//! 各字段名未恢复，按偏移命名。

/// 拍照模式状态（只含 `on_enable_photomode` 触及的偏移）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhotomodeState {
    /// `+0x00`。
    pub scalar: f32,
    /// `+0x04`，启用时写 `u32::MAX`。
    pub index: u32,
    /// `+0x08`。
    pub flag_08: bool,
    /// `+0x0a`。
    pub flag_0a: bool,
    /// `+0x0c`。
    pub flag_0c: bool,
}

impl PhotomodeState {
    /// 0x140870570。
    pub fn on_enable_photomode(&mut self, demo: bool) {
        self.scalar = if demo { 1.0 } else { 3.0 };
        self.index = u32::MAX;
        self.flag_08 = false;
        self.flag_0a = false;
        self.flag_0c = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabling_resets_every_touched_field() {
        let mut s = PhotomodeState {
            scalar: 9.0,
            index: 3,
            flag_08: true,
            flag_0a: true,
            flag_0c: true,
        };
        s.on_enable_photomode(false);
        assert_eq!(
            s,
            PhotomodeState {
                scalar: 3.0,
                index: u32::MAX,
                flag_08: false,
                flag_0a: false,
                flag_0c: false
            }
        );
    }

    #[test]
    fn demo_uses_one_instead_of_three() {
        let mut s = PhotomodeState::default();
        s.on_enable_photomode(true);
        assert_eq!(s.scalar, 1.0);
    }
}
