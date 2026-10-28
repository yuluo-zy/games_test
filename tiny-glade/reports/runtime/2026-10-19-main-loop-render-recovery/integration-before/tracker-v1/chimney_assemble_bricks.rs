//! B `decorator_visual/chimney_assemble_bricks.rs` — 烟囱砖装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x1407f8db0（chimney_assemble_bricks，791B，O 全文）
//! + 骨架 callees（0x1412ae660 = Chimney 差分闭包 env + DerivedDecoratorInfo::new
//! + impl$8::from 0x140b26a70）。
//!
//! 机器码（O）：
//! 1. `*(u32*)(param_4[2]) = *(u32*)((byte*)param_4 + 0x1c)`——ResMut 复合的
//!    **槽位写回**（把 +0x1c 的值写入第 3 个指针槽指向的 u32）；
//! 2. `DerivedDecoratorInfo::new`（0x140b26a70）由子类型/包围盒构造；
//! 3. 组装 5 元组 `{param_3, param_2, derived, param_5, param_7}` 转发
//!    FUN_1412ae660（*param_4 为差分代际值）。
//!
//! 本实现以类型化 seam 落地同一语义：槽位写回 + 派生信息 + 差分重装。

use country_core::resources::walls::decorator_subtype::{DecoratorSubtype, DerivedDecoratorInfo};
use glam::Vec3;

use crate::decorator_visual::deco_differential_state::{
    chimney_reassemble, ChimneyAssemblyParams, WallAttachedDecoTracker,
};

/// ResMut 复合（O：`+0x00` 值、`+0x1c` tick、`+0x10` 槽指针）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AssembleResMut {
    /// `+0x00`：差分代际/值（传给重装闭包）。
    pub value: u32,
    /// `+0x1c`：本帧 tick。
    pub tick: u32,
}

/// 0x1407f8db0（O，791B）：烟囱砖差分装配入口。
///
/// 返回重装条目数。`rebuild` 对应 0x1412ae660 的砖列装配主体（U，由宿主提供）。
pub fn chimney_assemble_bricks<F>(
    subtype: u8,
    bbx_min: Vec3,
    bbx_max: Vec3,
    resmut: &mut AssembleResMut,
    tracker: &mut WallAttachedDecoTracker<ChimneyAssemblyParams>,
    current: &std::collections::HashMap<u32, ChimneyAssemblyParams>,
    mut rebuild: F,
) -> usize
where
    F: FnMut(u32, &ChimneyAssemblyParams),
{
    // O：`*param_4[2] = *(u32*)(param_4 + 0x1c)`。
    resmut.value = resmut.tick;
    // O callee：DerivedDecoratorInfo::new（subtype/bbx 契约）。
    let _derived = DerivedDecoratorInfo::new(DecoratorSubtype(subtype), bbx_min, bbx_max);
    // O：转发差分重装闭包（仅变化项）。
    chimney_reassemble(tracker, current, |id, params| rebuild(id, params))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn slot_writeback_copies_tick() {
        let mut resmut = AssembleResMut { value: 1, tick: 9 };
        let mut tracker = WallAttachedDecoTracker::default();
        let current = HashMap::new();
        let n = chimney_assemble_bricks(
            0x15,
            Vec3::ZERO,
            Vec3::ONE,
            &mut resmut,
            &mut tracker,
            &current,
            |_, _| {},
        );
        assert_eq!(n, 0);
        assert_eq!(resmut.value, 9);
    }
}
