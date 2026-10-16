//! B `ui_systems/ui_stairs_misc.rs` — 楼梯交互杂项（O 机器码手解）。
//!
//! - `stairs_reset_for_new_frame` 0x14079c350（O）：复位拷贝后校验门控状态
//!   `state ∈ {4, 5}` 或 `state==11 && payload 低字节=='?'`；不匹配 → 0x1408fe880
//!   （`[interaction 内层+0x10] = 3`，I：失效标记）。
//! - `continue_stairs` 0x14079c3a0（O）：门控 `state==4 || (state==11 && payload
//!   低 u32=='?')`；迭代备份 0x88B 事件记录（`disc@0==3 && [@+0x81]=='?'`）
//!   应用 StairsState：`add_node`（0x1408ffd30）+ `add_edge`（0x1408ffd70，
//!   经 0x1408fe890 状态对检查）+ `try_set_endpoint`
//!   （0x1408fe7b0，`[@+0x80]==0` 未应用时，DecoratorLocator::new 0x140833240）。
use bevy_ecs::prelude::*;
use country_core::resources::stairs::stairs_state::{ObservedStairEndpoint, StairsState};

use crate::backup::DecoratorBackup;
use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// B 0x14079c350（O）：每帧楼梯状态复位。
pub fn stairs_reset_for_new_frame(
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
) {
    interaction.reset_interaction();
    match dispatcher.state {
        4 | 5 => {}
        11 if (dispatcher.payload as u8) == b'?' => {}
        _ => {
            // O：0x1408fe880 `[内层+0x10] = 3`（I：失效标记）。
            interaction.mark_invalid();
        }
    }
}

/// B 0x14079c3a0（O）：继续楼梯（门控 + 备份 0x88B 记录应用）。
pub fn continue_stairs(
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    interaction: ResMut<MoveDecorationInteractionState>,
    backup: ResMut<DecoratorBackup>,
    mut stairs: ResMut<StairsState>,
) {
    let gated = dispatcher.state == 4
        || (dispatcher.state == 11 && (dispatcher.payload as u32) == 0x3f);
    if !gated {
        return;
    }
    interaction.reset_interaction();
    for event in &backup.events {
        // O：disc@0==3 && [@+0x81]=='?' 的记录才应用。
        if event[0..8] != 3u64.to_le_bytes() || event[0x81] != b'?' {
            continue;
        }
        let node = u32::from_le_bytes(event[0x40..0x44].try_into().unwrap());
        let a = u32::from_le_bytes(event[0x08..0x0c].try_into().unwrap());
        let b = u32::from_le_bytes(event[0x10..0x14].try_into().unwrap());
        stairs.add_node(node);
        // O：0x1408fe890 状态对检查 + `add_edge(a, node)`（仅不同 id）。
        if a != node {
            stairs.add_edge(a, node);
        }
        if event[0x80] == 0 {
            // O：未应用 → try_set_endpoint（DecoratorLocator::new 载荷，I；
            // 24B 端点观察 = {a, b, node} 三 u64）。
            let endpoint = ObservedStairEndpoint::from_observed([
                a, b, node, 0,
            ]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap());
            stairs.try_set_endpoint(endpoint);
        }
    }
}
