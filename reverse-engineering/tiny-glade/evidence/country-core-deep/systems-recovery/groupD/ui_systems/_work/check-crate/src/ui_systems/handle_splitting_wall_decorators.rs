//! B `ui_systems/handle_splitting_wall_decorators.rs` — 墙分裂装饰处理（O）。
//!
//! - `display_decorator_wall_parts_and_get_hovered_part_id` 0x1412782b0（O
//!   结构）：按 rank（+0x44）遍历部件（`0..rank-1`），逐部件算位置
//!   （0x14125cf90 部件位置助手）并写指示符（记录 {1,1,pos,f32}，
//!   mesh 名经 SignifierFactory 0x14089d8b0 + UNK_142c23a18 族）；记录
//!   最近光标部件 id（输出）。type==':'（0x3a 窗型不可分裂）→ panic 行 40。
//! - `handle_moving_out_wall_decorator_part` 0x141279410（O 结构）：分裂移出
//!   部件——`TransientDecoratorIdGen::generate`（0x1408339e0）分配瞬时 id、
//!   `CachedDecoratorTransforms::insert`、0x40B 移动记录（内层 0x14127a480
//!   `{kind@0, addr@8, pos, height@0x14, u64@0x18, Δ@0x20, u8@0x30, sub@0x31,
//!   dst_tag@0x32, u8@0x38}`）+ `apply_decorator_op` + 历史写入。
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use glam::Vec3;

/// B 0x1412782b0（O 结构）：显示墙分裂部件并返回悬停部件 id。
/// 返回 `None` = 无悬停部件（I：光标距离阈值 U）。
pub fn display_decorator_wall_parts_and_get_hovered_part_id(
    info: &DecoratorInfo,
    _wall: &country_core::resources::walls::public_wall_state::PublicWallState,
    cursor_ws: Vec3,
    stream: &mut UiSignifierStream,
) -> Option<u8> {
    if info.decorator_type().0 == b':' {
        panic!(
            "display_decorator_wall_parts: unsupported decorator type ':' (handle_splitting_wall_decorators.rs:40)"
        );
    }
    let rank = info.rank;
    if rank <= 1 {
        return None;
    }
    let mut best: Option<(u8, f32)> = None;
    for part in 0..(rank - 1) {
        let pos = wall_part_position(info, part);
        let dist = (pos - cursor_ws).length();
        if best.is_none_or(|(_, d)| dist < d) {
            best = Some((part, dist));
        }
        let encoded = (u64::from(info.decorator_id)).rotate_right(32);
        stream.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position: pos,
            scale: 1.0,
            name: encoded,
        });
    }
    best.map(|(id, _)| id)
}

/// 部件位置（I：0x14125cf90 助手；沿墙空间按 rank 分段，几何 U）。
fn wall_part_position(info: &DecoratorInfo, part: u8) -> Vec3 {
    let _ = (info, part);
    Vec3::ZERO
}

/// B 0x141279410（O 结构）：分裂移出墙部件主入口（签名 I）。
#[allow(clippy::too_many_arguments)]
pub fn handle_moving_out_wall_decorator_part(
    _storage: &mut DecoratorStorage,
    _id: DecoratorId,
    _info: &DecoratorInfo,
    _addr: DecoratorStorageAddr,
    _part: u8,
    _stream: &mut UiSignifierStream,
) {
    // I：0x14127a480 内层记录构建 + TransientDecoratorIdGen + 应用（细节 U）。
}
