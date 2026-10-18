//! `handle_external_decorator_cmds.rs` — 外部装饰命令处理（0x14081a380，O）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x14081a380-*.txt`）：
//! 11 参 ECS 面；两条外部命令事件流（游标推进）：
//! - 流 A（0x28B 记录，墙命令）：逐条构造 op（kind 强制 **4=remove**）→
//!   `apply_decorator_op`（0x141276e30）→ `History::add`（tag 10 =
//!   DecoratorV5，载荷 Arc{1,1,**5**}，file handle_external_decorator_cmds.rs
//!   0x142a90338）；
//! - 流 B（0x30B 记录，屋顶命令）：op kind = 记录自带 → `apply_decorator_op`
//!   → `History::add`（tag 10，载荷 Arc{1,1,**3**}）；
//! 命令载荷：`{wall u64@0(+8), info 12B@+8..0x14, dst 16B@+4..+0x14,
//! …}`（0x28B）/`{kind@0, …@0x28, dst@+0x10.., roof tag@+0x22, …}`（0x30B）。
//! 事件中还含 `{u64} * 2` 流（0x10B 元素：cancel 类，U）。
//!
//! 事件类型（I）：系统侧模型 [`ExternalDecoratorCmd`]；原生事件族归属
//! core/UI（U 登记）。History 链经 core `EditType::DecoratorV5`
//! （`record_to_history_edit` 模型适配，见 deferred_decorator_history.rs）。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::resources::history::{EditType, History};
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator::{
    DecoratorInfo,
};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;

use crate::apply_decorator_op::{apply_decorator_op, DecoratorOp};
use crate::deferred_decorator_history::{
    record_to_history_edit, DeferredDecoratorEditHistory,
};

/// Compatibility name for the authoritative core queue; no second resource.
pub use country_core::resources::walls::decorator::DeferredDecoratorOpEvents as DecoratorOpEvents;

/// 外部装饰命令（I：两条流模型；0x28B 墙 remove / 0x30B 屋顶命令）。
#[derive(Event, Clone, Debug)]
pub struct ExternalDecoratorCmd {
    /// 目标墙（流 A 载荷 @+8；流 B 由屋顶命令承载）。
    pub wall: u64,
    /// 受影响装饰 id。
    pub decorator_id: u32,
    /// 目的存储地址。
    pub dst: DecoratorStorageAddr,
    /// 命令体（12B 信息载荷；U 字段语义）。
    pub payload: [u8; 12],
    /// 流判别（I：0=流 A 墙 remove、1=流 B 屋顶）。
    pub stream: u8,
}

/// `handle_external_decorator_cmds` @0x14081a380（O 结构）。
pub fn handle_external_decorator_cmds<C: Send + Sync + 'static>(
    mut history: ResMut<History>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
    events: Res<DecoratorOpEvents>,
    mut cmds: EventReader<ExternalDecoratorCmd>,
) {
    for cmd in cmds.read() {
        let op = DecoratorOp {
            kind: if cmd.stream == 0 {
                DecoratorOp::KIND_REMOVE
            } else {
                cmd.wall
            },
            info: DecoratorInfo {
                dst_wall: cmd.wall,
                dst_secondary: u64::from(cmd.decorator_id),
                ..DecoratorInfo::default()
            },
            ..DecoratorOp::default()
        };
        let Some(record) = build_external_record(cmd) else {
            continue;
        };
        // 机器码：逐命令 apply + History::add（tag 10，Arc{1,1,kind}）。
        apply_decorator_op(
            &op,
            0,
            cmd.dst,
            &mut storage,
            Some(&mut archivist),
            Some(&mut deferred),
            &events,
        );
        let edit = record_to_history_edit(&record);
        let _ = history.add(EditType::DecoratorV5(Arc::new(edit)));
    }
}

/// 外部命令 → 延迟历史记录（O 载荷 kind：流 A=5、流 B=3）。
fn build_external_record(cmd: &ExternalDecoratorCmd) -> Option<crate::deferred_decorator_history::DeferredDecoratorRecord> {
    let kind = if cmd.stream == 0 {
        crate::deferred_decorator_history::DeferredDecoratorRecord::KIND_REMOVE
    } else {
        crate::deferred_decorator_history::DeferredDecoratorRecord::KIND_CHANGE
    };
    Some(crate::deferred_decorator_history::DeferredDecoratorRecord {
        kind,
        f1: u64::from(cmd.decorator_id),
        ..crate::deferred_decorator_history::DeferredDecoratorRecord::default()
    })
}

/// 外部命令流事件（流 A 0x28B / 流 B 0x30B / cancel 0x10B 的登记面）。
#[allow(dead_code)]
fn _stream_surface() {}
