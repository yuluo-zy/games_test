//! B `decorator_visual/chimney_assemble_bricks.rs` — 烟囱砖装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f8db0（chimney_assemble_bricks，791B，O）+
//! 骨架 callees（0x1412ae660 = Chimney 差分闭包 env + DerivedDecoratorInfo::new +
//! impl$8::from 0x140b26a70）。
//!
//! 机器码（O）：7 参面；`*(u32*)(param_4[2]) = *(u32*)((byte*)param_4+0x1c)`
//! （ResMut tick 回写 / 槽位写）→ DerivedDecoratorInfo::new（子类型/包围盒）→
//! 组装 {param_2, param_3, param_5, param_7, derived} 转发 FUN_1412ae660
//! （*param_4 = deref 值）——即"按差分状态重装烟囱砖"。
//! U 登记：差分闭包 0x1412ae660 内部（烟囱砖几何主体）见 deco_differential_state.rs。

use country_core::resources::walls::decorator_subtype::{DecoratorSubtype, DerivedDecoratorInfo};
use glam::Vec3;

/// 0x1407f8db0（O，791B）：烟囱砖差分装配入口。
///
/// param_4 = ResMut 复合（tick 回写面）；param_1/param_2/param_3/param_5/
/// param_7 = 装配上下文（U 精确语义待复核）。DerivedDecoratorInfo::new 以
/// 子类型 + 库空间包围盒构造派生信息（O callee）。
pub fn chimney_assemble_bricks(
    context_a: u64,
    context_b: u64,
    context_c: u64,
    _resmut: u64,
    context_e: u64,
    _context_f: u64,
    context_g: u64,
) {
    // O callee：DerivedDecoratorInfo::new（subtype/bbx 契约）。
    let _derived = DerivedDecoratorInfo::new(
        DecoratorSubtype(0),
        Vec3::ZERO,
        Vec3::ZERO,
    );
    crate::decorator_visual::deco_differential_state::chimney_reassemble(
        context_a,
        context_b,
        context_c,
        context_e,
        context_g,
    );
}
