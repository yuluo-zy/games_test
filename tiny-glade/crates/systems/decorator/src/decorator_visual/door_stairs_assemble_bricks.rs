//! B `decorator_visual/door_stairs_assemble_bricks.rs` — 门梯砖装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ff130（door_stairs_assemble_bricks，399B，
//! O）+ 骨架 callees（0x1412af3a0 = DoorStairs 差分闭包 env）。
//!
//! 机器码（O）：4 参面；`*(u32*)(param_3[2]) = *(u32*)((byte*)param_3+0x1c)`
//! （ResMut tick 回写 / 槽位写）→ 组装 3 参 {param_2, param_1, param_4} 转发
//! FUN_1412af3a0（*param_3 = deref 值）——即"按差分状态重装门梯砖"。
//! U 登记：差分闭包 0x1412af3a0 内部（门梯砖几何主体）见 deco_differential_state.rs。

/// 0x1407ff130（O，399B）：门梯砖差分装配入口。
///
/// param_3 = ResMut 复合（tick 回写面）；param_1/param_2/param_4 = 装配上下文
/// （U 精确语义待复核）。等价语义：调用 `deco_differential_state` 的
/// DoorStairs 差分重装闭包。
pub fn door_stairs_assemble_bricks(
    context_a: u64,
    context_b: u64,
    _resmut: u64,
    context_d: u64,
) {
    crate::decorator_visual::deco_differential_state::door_stairs_reassemble(
        context_a,
        context_b,
        context_d,
    );
}
