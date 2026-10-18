//! B `ui_systems/move_ops.rs` — 移动操作辅助（O）。
//!
//! - `show_decorator_remove_signifier` 0x14127fe10（fresh 反汇编 110 条，
//!   2026-10-17 逐语句）：`delta = pos - cursor`；`dist = |delta|`；
//!   输出位置 = `pos + dist * dir`；旋转 = `xz_projective_rotation_bad(dir.xz)`
//!   （原版为 16B 输出）；名字 = `format!("despawn_decorator_{:?}_{:?}", id,
//!   pos)`（格式片 "despawn_decorator_" + "_"，两实参均为 Debug）；
//!   48B Arc 载荷 = `{Quat 16B, Vec3}`；`SignifierFactory::new` 写 40B 记录
//!   `{name_len,name_ptr,name_len_dup,inner,marker}` → `UiSignifierStream::write`。
//!
//! U 登记：`SignifierFactory::new` 的 marker 实参在 B 为静态地址
//! 0x142c24800（类型标记），本实现以 0 承载；`xz_projective_rotation_bad`
//! 在 crate utils 中当前为 Mat3 近似（B 返回 16B，见该函数注释），本处按其
//! 输出的前 4 个 f32 存储（字节位型一致）。
//!
//! - `calculate_move_dst` 0x1412801f0（O）：按 `DecoratorInfo` 字 `@+0x20`
//!   查跳表 0x142c248b0 派发（跳表目标在 B 未导出，处理器语义 U——
//!   本实现保留 tag→种类映射并登记）。
use std::sync::Arc;

use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::SignifierFactory;
use glam::{Vec2, Vec3};

use crate::utils::xz_projective_rotation_bad;

/// `show_decorator_remove_signifier` 的装饰侧输入（B RDX 结构）：
/// `{position: Vec3 @+0, direction: Vec3 @+0xc}`。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RemoveSignifierSource {
    pub position: Vec3,
    pub direction: Vec3,
}

/// 48B Arc 载荷 `{rotation 16B, position 12B+pad}`（0x14127ff43..ff61）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RemoveSignifierPayload {
    pub rotation: [f32; 4],
    pub position: Vec3,
}

/// 移除指示符（0x14127fe10，fresh 逐语句）。
///
/// 参数面（实测寄存器）：`(id: DecoratorId, source: &RemoveSignifierSource,
/// cursor: &Vec3, stream: &mut UiSignifierStream)`。
pub fn show_decorator_remove_signifier(
    id: DecoratorId,
    source: &RemoveSignifierSource,
    cursor: &Vec3,
    stream: &mut UiSignifierStream,
) {
    // 0x14127fe32..0x14127fe7a：3D 距离（三段平方和 + sqrt）。
    let dx = source.position.x - cursor.x;
    let dy = source.position.y - cursor.y;
    let dz = source.position.z - cursor.z;
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();

    // 0x14127fe7a..0x14127fe96：pos + dist * dir。
    let position = source.position + dist * source.direction;

    // 0x14127fe9b：rotation = xz_projective_rotation_bad(dir)（B 输出 16B）。
    let rotation = xz_projective_rotation_bad(Vec2::new(
        source.direction.x,
        source.direction.z,
    ));
    let mut rotation_bytes = [0.0f32; 4];
    rotation_bytes.copy_from_slice(&rotation.to_cols_array()[..4]);

    // 0x14127fed2..0x14127ff05：format!("despawn_decorator_{:?}_{:?}", id, pos)。
    let name = format!("despawn_decorator_{:?}_{:?}", id, position);

    // 0x14127ff43..0x14127ff61：48B ArcInner {strong=1,weak=1,payload}；
    // payload = {rotation 16B, position 12B}。
    let payload = Arc::new(RemoveSignifierPayload {
        rotation: rotation_bytes,
        position,
    });

    // 0x14127ff7d：SignifierFactory::new(name, inner=Arc payload, marker)。
    // U：B marker = 静态 0x142c24800（类型标记），本实现填 0。
    let factory = SignifierFactory::new(
        name.as_bytes(),
        Arc::as_ptr(&payload) as *const u8,
        0,
    );
    // 0x14127ffa1：UiSignifierStream::write(factory 40B 记录)。
    stream.write_factory(&factory);
    // 0x14127ffa6..0x14127ffb9：名字 String 释放（Rust String drop 等价）。
}

/// B 0x1412801f0（O）：按 `DecoratorInfo` 字 `@+0x20` 查跳表派发移动目的地
/// 计算（跳表 0x142c248b0 的目标函数未被 Ghidra 恢复；处理器语义 U）。
pub fn calculate_move_dst(info: &DecoratorInfo) -> MoveDstKind {
    // 机器码索引 = `*(u32*)(info + 0x20)`（完整 32 位 `u20`，非 dst_tag 字节）。
    match info.u20 {
        0 | 1 => MoveDstKind::Wall,
        2 => MoveDstKind::Roof,
        3 => MoveDstKind::Terrain,
        4 => MoveDstKind::Stair,
        _ => MoveDstKind::Invalid,
    }
}

/// 移动目的地种类（I：跳表 4 路 + 无效）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoveDstKind {
    Wall,
    Roof,
    Terrain,
    Stair,
    Invalid,
}
