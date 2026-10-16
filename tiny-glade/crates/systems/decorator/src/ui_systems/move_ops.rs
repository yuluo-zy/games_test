//! B `ui_systems/move_ops.rs` — 移动操作辅助（O）。
//!
//! - `show_decorator_remove_signifier` 0x14127fe10（O 结构）：移除指示符——
//!   光标与装饰位置差 → 光标 + 差值方向位置；signifier 记录 48B
//!   {1, 1, enc_id, id, pos, scale} → `UiSignifierStream::write`；名称哈希经
//!   0x140c911d0 + SignifierFactory::new（&UNK_142c24800，宿主接线）。
//! - `calculate_move_dst` 0x1412801f0（O）：按 DecoratorInfo 目的 tag
//!   （u32@+0x20 高字节）查跳表 UNK_142c248b0 派发（wall/roof/terrain/stair
//!   四路；处理器 I）。
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use glam::Vec3;

/// 移除指示符（O 结构；精确位置公式 O：`cursor + Δ·dir`）。
pub fn show_decorator_remove_signifier(
    id: DecoratorId,
    visual_pos: Vec3,
    cursor: Vec3,
    stream: &mut UiSignifierStream,
) {
    let delta = (visual_pos - cursor).length();
    let dir = if delta > 1e-6 {
        (visual_pos - cursor) / delta
    } else {
        Vec3::ZERO
    };
    let pos = cursor + dir * delta;
    let encoded = (u64::from(id.0)).rotate_right(32);
    stream.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: pos,
        scale: delta,
        name: encoded,
    });
}

/// B 0x1412801f0（O 结构）：按目的 tag 派发移动目的地计算（处理器 I）。
pub fn calculate_move_dst(_info: &DecoratorInfo) -> MoveDstKind {
    match _info.dst_tag() {
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
