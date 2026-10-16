//! `decorator_on_move_shape.rs` — 形状移动 → 装饰响应（0x140811fc0，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140811fc0-*.txt`）：
//! `(ResMut<DecoratorStorage>, EventReader<0x30B 事件>)`；事件 = `{? u64@0,
//! ids ptr @8, ids len @0x10, wall @0x18, …}`。逐事件：
//! 1. `wall_decorators_mut(wall)`（0x140a1e420）；
//! 2. 逐事件 id → 桶 `modified_ids.insert`（FUN_1402c9200 = HashSet insert，
//!    bucket+0x60）；
//! 3. 内嵌 map 非空 → 逐装饰（0x140a1b200 迭代 + 哈希 XOR 掩码 + d2s
//!    查/写）按形状新位更新（尾段 U：精确位置公式见
//!    `decomp-0x140811fc0` 尾；本实现按机器码结构 + 偏移量登记）。
//!
//! 事件类型（I）：[`MoveShapeEvent`] 为系统侧模型；原生事件族归属 core
//! wall 系统（U 登记）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::WallId;

/// 形状移动事件（I：0x30B 载荷模型；字段放置 O 读面）。
#[derive(Event, Clone, Debug)]
pub struct MoveShapeEvent {
    /// +0x00：判别/前域（U）。
    pub kind: u64,
    /// +0x08：受影响装饰 id 列表（机器码 ptr@+8 / len@+0x10）。
    pub affected: Vec<u32>,
    /// +0x18：墙 id。
    pub wall: WallId,
    /// +0x20..：附加（U）。
    pub extra: u64,
}

/// `decorator_on_move_shape` @0x140811fc0（O 结构）。
pub fn decorator_on_move_shape(
    mut storage: ResMut<DecoratorStorage>,
    mut events: EventReader<MoveShapeEvent>,
) {
    for event in events.read() {
        let Some(bucket) = storage.wall_decorators_mut(event.wall) else {
            continue;
        };
        // 机器码：事件 id → modified_ids（FUN_1402c9200 = HashSet insert）。
        for id in &event.affected {
            bucket.modified_ids.insert(*id);
        }
        // 机器码：内嵌 map 非空 → 逐装饰位置更新（尾段 U）。
        for (_id, info) in bucket.iter_mut() {
            let _ = info;
            // U：形状新位写入公式（事件 kind 判别分派）——证据见
            // decomp-0x140811fc0 尾段（哈希 XOR 掩码 d2s 面 + Vec2 add）。
        }
    }
}
