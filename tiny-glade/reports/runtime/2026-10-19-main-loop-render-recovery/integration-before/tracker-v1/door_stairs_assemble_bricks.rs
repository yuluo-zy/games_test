//! B `decorator_visual/door_stairs_assemble_bricks.rs` — 门梯砖装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407ff130（door_stairs_assemble_bricks，399B，
//! **O 全文**）+ 骨架 callees（0x1412af3a0 = DoorStairs 差分闭包 env）。
//!
//! 机器码（O）：
//! 1. `*(u32*)param_3[2] = *(u32*)((byte*)param_3 + 0x1c)`——ResMut 复合的
//!    槽位写回（把 +0x1c 写入第 3 个指针槽）；
//! 2. 组装 3 元组 `{param_2, param_1, param_4}` 转发 FUN_1412af3a0
//!    （*param_3 为差分代际值）。
//!
//! 本实现以类型化 seam 落地同一语义：槽位写回 + 差分重装。

use crate::decorator_visual::deco_differential_state::{
    door_stairs_reassemble, DoorStairsAssemblyParams, WallAttachedDecoTracker,
};

/// ResMut 复合（O：`+0x00` 值、`+0x1c` tick）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AssembleResMut {
    /// `+0x00`：差分代际/值。
    pub value: u32,
    /// `+0x1c`：本帧 tick。
    pub tick: u32,
}

/// 0x1407ff130（O，399B）：门梯砖差分装配入口。
///
/// 返回重装条目数。`rebuild` 对应 0x1412af3a0 的砖列装配主体（U，由宿主提供）。
pub fn door_stairs_assemble_bricks<F>(
    resmut: &mut AssembleResMut,
    tracker: &mut WallAttachedDecoTracker<DoorStairsAssemblyParams>,
    current: &std::collections::HashMap<u32, DoorStairsAssemblyParams>,
    mut rebuild: F,
) -> usize
where
    F: FnMut(u32, &DoorStairsAssemblyParams),
{
    // O：`*param_3[2] = *(u32*)(param_3 + 0x1c)`。
    resmut.value = resmut.tick;
    // O：转发 DoorStairs 差分重装闭包（仅变化项）。
    door_stairs_reassemble(tracker, current, |id, params| rebuild(id, params))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn slot_writeback_copies_tick() {
        let mut resmut = AssembleResMut { value: 3, tick: 11 };
        let mut tracker = WallAttachedDecoTracker::default();
        let current = HashMap::new();
        let n = door_stairs_assemble_bricks(&mut resmut, &mut tracker, &current, |_, _| {});
        assert_eq!(n, 0);
        assert_eq!(resmut.value, 11);
    }
}
