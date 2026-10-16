//! B `ui_systems/ui_inactive.rs` — 装饰空闲系统（0x140791400，O 机器码手解）。
//!
//! 机器码（capstone，2026-10-16）：`[gate+0]==6 && [gate+8]==0` 时执行
//! 复位拷贝（`*[interaction+0x10] ← [interaction+0x1c]`）后尾调
//! `DecoratorBackup::clear`（jmp 0x140823660）；否则直接返回。门控 =
//! `(field0-2)==4 && 128 位余项==0`（`cmp rcx,4/setb`+`cmp rcx,4/sete` 组合）。
use bevy_ecs::prelude::*;

use crate::backup::DecoratorBackup;
use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// B 0x140791400（O）：空闲门控 → 复位交互 → 清备份。
pub fn ui_decorator_inactive(
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
    mut backup: ResMut<DecoratorBackup>,
) {
    if dispatcher.state == 6 && dispatcher.payload == 0 {
        interaction.reset_interaction();
        backup.clear();
    }
}
