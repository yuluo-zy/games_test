//! set_wall_color_id.rs — 墙颜色写入（B 0x142290750 init_color_ids_buffer /
//! 0x142290820 set_wall_color_id；restorer-state-color 2026-10-17 fresh 反编译重建）。
//!
//! `set_wall_color_id` 真实控制流（0x142290820，1817B）：
//! 1. `FrozenMap::get<SsboId,BufferState>` 取 `WallColorIdSsbo`，memcmp 50B 名校验
//!    （不符 `assert_failed`），vtable+0x18 取缓冲；**SSBO 上传段未恢复**（country_core
//!    `WallColorIdSsbo` 为 typed-gap，无 `SsboType`，分歧点）。
//! 2. 事件循环（stride 0x18）：`idx = id→索引`（0x1400885d0）；断言
//!    `idx <= 0x4fff`（panic 0x1401412e0 区）；读旧色、写 `WallColorIds[idx]=新色`、
//!    写 change tick。
//! 3. `PublicWalls::get(id)` 命中时：`cmd.custom != 0` → `WallStyle::is_halftimbered()`
//!    （返回未在反编译中参与分支，U）+ 折线 AABB（`Aabb2::from_points` 折叠、
//!    `Aabb2::center`、`PublicWallState::max_y`、`xvy`）→ 推 0x20B 记录 tag=6。
//! 4. 脏色分类：新色 ∈ {4..10,12} → `CreatePlasterCmd`；否则旧色 ∈ 该集 →
//!    `DeletePlasterCmd`。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::color_id::{SetWallColorIdCmd, WallColorIds, MAX_NUMBER_OF_WALLS};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::audio::PlayWallUiSound;
use utils::geometry::aabb::Aabb2;
use utils::swizzlers::SwizzlerXVY;

/// 自定义色集合（O：新色 ∈ {4..10, 12} → 石膏重建；`6 < new-4U && new != 0xc`
/// 的机器码判别）。
pub const CUSTOM_COLOR_SET: [u32; 8] = [4, 5, 6, 7, 8, 9, 10, 12];

/// `init_color_ids_buffer` @0x142290750（198B，重建）。
///
/// fresh 反编译顺序：`add_new::<WallColorIdSsbo>`→`add_new::<RoofColorIdSsbo>`→
/// `insert_resource(WallColorIds::new())`→`add_new::<GateColorIdSsbo>`→
/// `insert_resource(GateColorIds::new())`→`add_new::<HalftimberColorIdSsbo>`→
/// `insert_resource(HalftimberColorIds::new())`。候选签名（O，PDB）
/// `void(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, Commands)`。
/// **分歧点**：`AssetSsboLibrary::add_new::<T>` 需 `T: SsboType`，而 core 的
/// `WallColorIdSsbo/RoofColorIdSsbo/GateColorIdSsbo/HalftimberColorIdSsbo` 为
/// typed-gap（无 `StorageElement`/`SsboType` 实现）——SSBO 注册无法编译，登记。
pub fn init_color_ids_buffer(
    mut commands: Commands,
    _library: Res<AssetSsboLibrary>,
) {
    use country_core::resources::color_id::{GateColorIds, HalftimberColorIds};
    // O：`WallColorIds::new()`（0x140b19890）insert_resource。
    commands.insert_resource(WallColorIds::new());
    commands.insert_resource(GateColorIds::new());
    commands.insert_resource(HalftimberColorIds::new());
}

/// `set_wall_color_id` @0x142290820（1817B，重建；SSBO 上传段未恢复）。
///
/// 分歧点：真实 8 参（含 `Res<GraphicsDeviceResource>`）中 rhapsody 设备接线未
/// 接入（core 无 `GraphicsDeviceResource` 资源）；`WallColorIdSsbo` typed-gap 使
/// `get::<WallColorIdSsbo>()`+`update_buffer_bytes` 段无法编译。已恢复：事件循环、
/// 索引断言、颜色写、halftimber AABB、脏色石膏分类、0x20B 记录 tag=6。
#[allow(clippy::too_many_arguments)]
pub fn set_wall_color_id(
    _library: Res<AssetSsboLibrary>,
    mut cmds: EventReader<SetWallColorIdCmd>,
    mut wall_colors: ResMut<WallColorIds>,
    mut sounds: EventWriter<PlayWallUiSound>,
    walls: Res<PublicWalls>,
    mut plaster_create: EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,
    mut plaster_delete: EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,
) {
    for cmd in cmds.read() {
        // O：`idx = id→索引`（0x1400885d0）；`0x4fff < idx` → panic（= idx <
        // MAX_NUMBER_OF_WALLS=0x5000）。
        let idx = cmd.id as usize;
        assert!(idx < MAX_NUMBER_OF_WALLS, "wall_index < MAX_NUMBER_OF_WALLS");
        // O：读旧色、写新色。
        let old = wall_colors.0[idx];
        wall_colors.0[idx] = cmd.color;
        // O：PublicWalls::get → 命中才做 halftimber/石膏分类（反编译 do-while
        // 在 wall==null 时跳过处理直接进入下一事件）。
        if let Some(wall) = walls.get(WallId(u64::from(cmd.id))) {
            if cmd.custom != 0 {
                // O：`WallStyle::is_halftimbered()`（内联于 0x142290820；返回未参与
                // 反编译分支，U——可能为 Ghidra 丢失的数据流；core `PublicWallState`
                // 未暴露该访问器，未调用，登记 GAP）。
                // O：折线 AABB（墙+0x40 点 ptr / +0x48 计数，每点 3×u32 (x,z,0)；
                // SIMD minps/maxps = glam Vec2::min/max，Aabb2::from_points 折叠）
                // → center → max_y → xvy。空折线时 Aabb2 保持 {f32::MAX, f32::MIN}，
                // center=0（与反编译一致）。
                let aabb = Aabb2::from_points(
                    wall.height_outline()
                        .iter()
                        .map(|p| Vec2::new(p[0], p[2])),
                );
                let center = aabb.center();
                let position = center.xvy(wall.max_y());
                // O：推 0x20B 记录（tag 6；PlayWallUiSound kind=6）。selector/
                // value 载荷在反编译为未解出的字节（+0x09/+0x0c），登记 U。
                sounds.write(PlayWallUiSound {
                    kind: 6,
                    selector: 0,
                    value: 0.0,
                    position,
                });
            }
            // O：脏色分类（新色∈{4..10,12} → CreatePlaster；否则旧色∈该集 →
            // DeletePlaster）。
            let new_custom = CUSTOM_COLOR_SET.contains(&cmd.color);
            if new_custom {
                plaster_create.write(country_core::resources::walls::plaster::CreatePlasterCmd(
                    WallId(u64::from(cmd.id)),
                ));
            } else if CUSTOM_COLOR_SET.contains(&old) {
                plaster_delete.write(country_core::resources::walls::plaster::DeletePlasterCmd(
                    WallId(u64::from(cmd.id)),
                ));
            }
        }
    }
}
