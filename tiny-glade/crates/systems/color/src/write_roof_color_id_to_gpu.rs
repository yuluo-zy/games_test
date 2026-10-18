//! write_roof_color_id_to_gpu.rs — 屋顶颜色 ID 增量上传 GPU SSBO（B 0x142298520，
//! 999B；restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! **未实现**（分歧点）：真实控制流 = `FrozenMap::get` 取 `RoofColorIdSsbo`
//! （memcmp 0x37=55B 名校验）→ 双事件源遍历（stride 0x40 与 0x18）逐元素
//! `get_instruction()` 取索引 → `GraphicsDevice::update_buffer_bytes(4 字节,
//! offset=idx<<2)` 增量上传。core 的 `RoofColorIdSsbo` 为 typed-gap（无
//! `SsboType`），`GraphicsDeviceResource` 资源未接线——GPU 段无法编译；两事件源
//! 的确切类型（stride 0x40 候选 CreateRoofCmd 0x40B、stride 0x18 = SetRoofColorIdCmd）
//! 与 core `RoofCreateRequestObserved` 布局未对齐。本文件保留签名面与读取面，
//! GPU 上传登记 GAP。

use bevy_ecs::prelude::*;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::roofs::commands::CreateRoofCmd;

use crate::SetRoofColorIdCmd;

/// `write_roof_color_id_to_gpu` @0x142298520（999B，未实现——SSBO typed-gap 阻断）。
pub fn write_roof_color_id_to_gpu(
    _library: Res<AssetSsboLibrary>,
    mut roof_cmds: EventReader<SetRoofColorIdCmd>,
    mut creates: EventReader<CreateRoofCmd>,
) {
    // O：stride 0x18 源（SetRoofColorIdCmd）遍历——反编译第二源。
    for cmd in roof_cmds.read() {
        let _ = (cmd.id, cmd.color);
        // O：`update_buffer_bytes(4B, offset=idx<<2)` 上传（0x142298520 内联
        // gpu_write_with_offset；容量检查 `buffer_len < idx*4+4 → unwrap_failed`）。
        // —— GAP：RoofColorIdSsbo typed-gap，无法编译。
    }
    // O：stride 0x40 源（候选 CreateRoofCmd）遍历——反编译第一源。
    for _cmd in creates.read() {
        // —— GAP：同上，0x40B 元素索引面未对齐。
    }
}
