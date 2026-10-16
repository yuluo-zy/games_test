//! B `ui_systems/handle_splitting_roof_decorators.rs` — 屋顶分裂装饰处理（O）。
//!
//! - `try_display_roof_decorator_parts_and_get_hovered_part_id` 0x14127a790
//!   （O 结构）：同墙版（0x1412782b0）——按 rank 遍历屋顶部件、逐部件
//!   `RoofTriggerVolume::get_position_from_mesh_uv`（0x140c9acb0 族）+ 屋顶
//!   uv（0x1408e4750/0x1411ff640/0x1411ff740）算位置写指示符；type==':' 同
//!   行 40 panic。
//! - `handle_moving_out_roof_decorator_part` 0x14127bab0（O 结构）：屋顶移出
//!   部件（0x14127c7f0 内层：u 值回绕 [0,1)（O：`<0 → +1`、`>1 → -1`）、
//!   `{kind@0, u64@8, u@0x10, f32@0x14, u32@0x18=0, u8@0x32=2(roof), u8@0x38}`）
//!   + `apply_decorator_op` + 历史。
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use glam::Vec3;

/// B 0x14127a790（O 结构）：显示屋顶分裂部件并返回悬停部件 id。
pub fn try_display_roof_decorator_parts_and_get_hovered_part_id(
    info: &DecoratorInfo,
    roof_uv: impl Fn(u8) -> Option<(f32, f32)>,
    floor_y: f32,
    cursor_ws: Vec3,
    stream: &mut UiSignifierStream,
) -> Option<u8> {
    if info.decorator_type().0 == b':' {
        panic!(
            "try_display_roof_decorator_parts: unsupported decorator type ':' (handle_splitting_roof_decorators.rs:40)"
        );
    }
    let rank = info.rank;
    if rank <= 1 {
        return None;
    }
    let mut best: Option<(u8, f32)> = None;
    for part in 0..(rank - 1) {
        let pos = roof_part_position(part, &roof_uv, floor_y);
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

/// 屋顶部件位置（I：RoofTriggerVolume::get_position_from_mesh_uv(uv, floor_y)；
/// 几何 U）。
fn roof_part_position(
    part: u8,
    roof_uv: &impl Fn(u8) -> Option<(f32, f32)>,
    floor_y: f32,
) -> Vec3 {
    let _ = (part, floor_y);
    let _uv = roof_uv(part);
    Vec3::ZERO
}

/// B 0x14127bab0（O 结构）：屋顶分裂移出部件主入口（签名 I）。
#[allow(clippy::too_many_arguments)]
pub fn handle_moving_out_roof_decorator_part(
    _storage: &mut DecoratorStorage,
    _id: DecoratorId,
    _info: &DecoratorInfo,
    _addr: DecoratorStorageAddr,
    _part: u8,
    _stream: &mut UiSignifierStream,
) {
    // I：0x14127c7f0 内层（u 回绕 [0,1)）+ TransientDecoratorIdGen + 应用（细节 U）。
}
