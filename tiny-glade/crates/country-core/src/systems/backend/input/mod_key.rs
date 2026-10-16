//! `mod_key.rs` — 修饰键状态与吸附禁用开关（B `systems::mod_key`）。
//!
//! 证据：
//! - `Modifiers::disable_snapping` @0x1408c5680（ICF 折叠，与 `ActiveInputMode::is_controller`
//!   同码）：`movzx eax, byte ptr [rcx]; ret` —— 读位标志组第 0 字节（[O，mod-key 手解]）。
//! - `is_disable_snapping_just_pressed` @0x140af9bf0：`movzx eax, byte ptr [rcx + 1]; ret`
//!   —— 读位标志组第 1 字节（[O，0x140af9bf0.asm]；按 PDB 名差异分属 just-pressed 沿 vs
//!   稳态，机器码同形 U）。
//! - `update_mod_key_state` @0x140af9c00（① T3 契约 t3-input-update_mod_key_state）：
//!   `setb 字节@0` 门（状态 `[config+0x78] < 3`）+ 表1 跳表（0x142ae5f50，8 条目）分派写
//!   字节@1：组 0=强制 ON / 1,3,4=强制 OFF / 2=阈值 `ucomiss xmm0,[+0x7c]` setae /
//!   5,6,7=动作键组（键码映射待取证）。config 资源身份（`+0x78` 状态 / `+0x7c` 阈值）
//!   待①取证，以注册适配 [`ModKeyConfig`] 承载（不伪造）。
//!
//! 布局：`{u32 @+0, 字节@+1}`（位标志字节组，非 4 个独立 bool）。字节@0 = `disable_snapping`，
//! 字节@1 = just-pressed 沿；其余位待取证。

use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Res, ResMut};

/// `Modifiers`（原 `systems::mod_key::Modifiers`）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    /// Native u32 @+0：位标志字节组 { 0 = disable_snapping, 1 = just_pressed 沿, … 待取证 }。
    pub flags: u32,
}

impl Modifiers {
    /// `Modifiers::disable_snapping` @0x1408c5680：读位标志组第 0 字节非零。
    pub fn disable_snapping(self) -> bool {
        (self.flags & 0xff) != 0
    }

    /// `is_disable_snapping_just_pressed` @0x140af9bf0：读位标志组第 1 字节非零。
    pub fn is_disable_snapping_just_pressed(self) -> bool {
        ((self.flags >> 8) & 0xff) != 0
    }
}

/// 注册适配资源：native `update_mod_key_state` 读 `[config+0x78]` 状态 u32（键组选择器，
/// 0..7）+ `[config+0x7c]` 阈值 f32（组 2 ucomiss 比较）。config 资源身份待①取证；
/// 本适配承载状态/阈值两读面，宿主接线（① 定身份后替换为真资源）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ModKeyConfig {
    /// Native `[config+0x78]`：状态 u32（表1 键组选择器 0..7）。
    pub state: u32,
    /// Native `[config+0x7c]`：阈值 f32（组 2 `ucomiss xmm0,[+0x7c]` setae 比较）。
    pub threshold: f32,
}

/// `update_mod_key_state` 0x140af9c00 producer（① T3 契约 §1/§2/§5 可直接落地部分）：
/// - `setb 字节@0` 门（0x140af9c0b）：状态 `< 3` → disable_snapping 位置 1。
/// - 表1 跳表（0x142ae5f50，8 条目）分派写字节@1（just-pressed 沿）：
///   组 0（0x140af9c4b）= 强制 ON；组 1/3/4（0x140af9c38）= 强制 OFF；
///   组 2（0x140af9c3d）= `0.0 >= [+0x7c]` setae；组 5/6/7（0x140af9ca0/8c/d24）
///   = 动作/按键组 handler，**键码映射待取证**，缺键状态源时按 OFF 登记（不造）。
pub fn update_mod_key_state(
    config: Res<ModKeyConfig>,
    mut modifiers: ResMut<Modifiers>,
) {
    let mut flags = modifiers.flags;
    // O: setb 字节@0 —— 状态 < 3 → disable_snapping 位。
    flags = (flags & !0xff) | u32::from(config.state < 3);
    // O: 表1 分派 —— byte@1 = just-pressed 沿。
    let just_pressed = match config.state {
        0 => true,
        1 | 3 | 4 => false,
        2 => 0.0 >= config.threshold,
        _ => false,
    };
    flags = (flags & !0x100) | (u32::from(just_pressed) << 8);
    modifiers.flags = flags;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicates_read_the_flag_bytes() {
        // byte@0 = disable_snapping；byte@1 = just-pressed。
        let steady = Modifiers { flags: 0x01 };
        assert!(steady.disable_snapping());
        assert!(!steady.is_disable_snapping_just_pressed());
        let edge = Modifiers { flags: 0x1_00 };
        assert!(!edge.disable_snapping());
        assert!(edge.is_disable_snapping_just_pressed());
        let both = Modifiers { flags: 0x1_01 };
        assert!(both.disable_snapping());
        assert!(both.is_disable_snapping_just_pressed());
        // 高位字节不影响这两个谓词（待取证位不猜语义）。
        let high = Modifiers { flags: 0x00ff_0000 };
        assert!(!high.disable_snapping());
        assert!(!high.is_disable_snapping_just_pressed());
    }

    #[test]
    fn update_writes_flag_bytes_per_table_one() {
        // 组 0：强制 ON（byte@1 = 1）+ 状态 <3 → byte@0 = 1。
        let mut m = Modifiers::default();
        update_mod_key_state(ModKeyConfig { state: 0, threshold: 0.0 }, &mut m);
        assert!(m.disable_snapping());
        assert!(m.is_disable_snapping_just_pressed());
        // 组 1/3/4：强制 OFF（byte@1 = 0）。
        let mut m = Modifiers { flags: 0x1_00 };
        update_mod_key_state(ModKeyConfig { state: 4, threshold: 0.0 }, &mut m);
        assert!(!m.is_disable_snapping_just_pressed());
        // 组 2：阈值 setae = 0.0 >= threshold。
        let mut m = Modifiers::default();
        update_mod_key_state(ModKeyConfig { state: 2, threshold: -1.0 }, &mut m);
        assert!(m.is_disable_snapping_just_pressed());
        let mut m = Modifiers::default();
        update_mod_key_state(ModKeyConfig { state: 2, threshold: 0.5 }, &mut m);
        assert!(!m.is_disable_snapping_just_pressed());
        // 状态 >=3 → byte@0 = 0（disable_snapping 关）。
        let mut m = Modifiers { flags: 0x1 };
        update_mod_key_state(ModKeyConfig { state: 5, threshold: 0.0 }, &mut m);
        assert!(!m.disable_snapping());
        assert!(!m.is_disable_snapping_just_pressed());
    }
}
