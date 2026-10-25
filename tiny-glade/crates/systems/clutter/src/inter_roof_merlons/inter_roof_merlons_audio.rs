//! inter_roof_merlons_audio（B 主基线；0x1422d4230，size 524）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x1422d4230_*.c`）。
//!
//! 控制流（O）：
//! - 首参模式字节 `*(int*)mode == 0xd`（native tag 13 = `AppMode::Replay`）→ 直接返回。
//! - 两次写 0x28B `indexmap::Bucket<...>` 事件（`+8` 判别 **8** 与 **9** 的
//!   `AlignKind`），载荷含 `CursorTerrainRaycast::position()` 结果与变化索引。
//! - 末尾把描述 Vec（stride 0x28）搬到另一缓冲，`spec_extend` 后把游标修到新长度。
//!
//! U 登记：两个写事件的精确条件（`uVar13 < len` / `cursor+0x18 == *ptr`）与
//! 0x28B bucket 载荷（位置 4×f32 + u32/u8 槽）为宿主 indexmap 合同；本 crate 以
//! [`InterRoofMerlonAudioRequest`] 承载 {position, kind}，两个 kind 对应 8/9。
//! 描述 Vec 搬迁为宿主缓冲管理（U）。

use bevy_ecs::prelude::*;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::systems::mode_manager::AppMode;

/// 城垛音频请求事件（宿主音频系统消费）。`kind` 为原版 bucket 判别 8/9。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct InterRoofMerlonAudioRequest {
    pub position: glam::Vec3,
    pub kind: u8,
}

/// request_inter_roof_merlon_audio（0x1422d4230）：Replay 模式跳过；光标位置发
/// kind 8/9 两个音频请求。
pub fn request_inter_roof_merlon_audio(
    mode: Res<AppMode>,
    cursor: Res<CursorTerrainRaycast>,
    mut ev: EventWriter<InterRoofMerlonAudioRequest>,
) {
    // O：native tag 13 = Replay → 不请求。
    if mode.native_tag() == 13 {
        return;
    }
    // O：cursor.position() 在两次写块中各自调用（两次读取同源）。
    let _ = ev.write(InterRoofMerlonAudioRequest {
        position: cursor.position(),
        kind: 8,
    });
    let _ = ev.write(InterRoofMerlonAudioRequest {
        position: cursor.position(),
        kind: 9,
    });
}
