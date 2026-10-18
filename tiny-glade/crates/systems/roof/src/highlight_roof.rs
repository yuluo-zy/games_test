//! 屋顶高亮（highlight_roof.rs）。
//!
//! `highlight_roof`：签名（mangled 名 O，两 monomorphization wrapper
//! 0x141a284b0 run_without_applying_deferred / 0x141a31280 run_unsafe，主体一致）：
//! `void(ResMut<EditingHighlight>, EventReader<HighlightRoofCmd>,
//!  ResMut<RoofHighlight>)`。
//!
//! ## fresh 反编译（2026-10-17，`work/wave3-timber-roof/evidence/0x141a31280_*.c` 为存储迭代权威）
//!
//! 逐事件（0x18 步长：`wall u64@+8`、`color 判别 u8@+0x10`）：
//! 1. `RoofHighlight.tag = 0`（资源写前清 tag）；
//! 2. 每事件：`RoofHighlight.value ← event[+8]`（wall u64 低 u32）、`RoofHighlight.tag ← event[+0x10]`
//!    （color 判别字节）、`HighlightColor::to_rgb()` → `EditingHighlight.color`（Vec3）；
//! 3. changed tick 逐事件写（Rust `&mut` 查询自动标 Changed，等价 tick 语义）。
//!
//! 分歧（诚实）：`*puVar6 = uVar7` 写 8 字节覆盖 `{value u32, pad 4B}`——pad 字节不可表示
//! （I 级）；`0x141a284b0` 的 `next()` 迭代偏移与 0x141a31280 存储迭代对 RoofHighlight 源字段
//! 的读出不同（前者经迭代器返回指针、后者直接存迭代），以存储迭代 0x141a31280 为准。
use bevy_ecs::prelude::*;

use country_core::resources::editing_highlight::EditingHighlight;
use country_core::resources::roofs::commands::HighlightRoofCmd;
use country_core::systems::render_loop::draw_passes::draw_callbacks_a::RoofHighlight;

/// 屋顶高亮维护（B 0x141a284b0/0x141a31280；HighlightRoofCmd/HighlightColor 已落 core）。
pub fn highlight_roof(
    mut editing: ResMut<EditingHighlight>,
    mut cmds: EventReader<HighlightRoofCmd>,
    mut highlight: ResMut<RoofHighlight>,
) {
    // O：`*(undefined1 *)(puVar6 + 1) = 0` —— RoofHighlight.tag 清 0（无事件 → tag 保持 0）。
    highlight.tag = 0;
    for cmd in cmds.read() {
        // O：`*puVar6 = uVar7`（event+8 wall → value 低 u32）、`*(puVar6+1) = uVar3`
        //    （event+0x10 color 字节 → tag）。
        highlight.value = cmd.wall.get_inner() as u32;
        highlight.tag = cmd.color.0;
        // O：`HighlightColor::to_rgb()` → `EditingHighlight.color`（`*puVar5`/`*(puVar5+1)`）。
        editing.color = cmd.color.to_rgb().into();
    }
}
