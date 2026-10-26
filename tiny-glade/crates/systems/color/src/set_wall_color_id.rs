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
use country_core::resources::color_id::{MAX_NUMBER_OF_WALLS, SetWallColorIdCmd, WallColorIds};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::audio::PlayWallUiSound;
use utils::geometry::aabb::Aabb2;
use utils::swizzlers::SwizzlerXVY;

/// 自定义色集合（O：新色 ∈ {4..10, 12} → 石膏重建；`6 < new-4U && new != 0xc`
/// 的机器码判别）。
pub const CUSTOM_COLOR_SET: [u32; 8] = [4, 5, 6, 7, 8, 9, 10, 12];

/// `init_color_ids_buffer` @0x142290750（198B）。
///
/// 原版顺序（fresh 反编译 + 注册面 `add_color` @0x140651870 的 `Schedules::add_systems`）：
/// `add_new::<WallColorIdSsbo>` → `add_new::<RoofColorIdSsbo>` →
/// `insert_resource(WallColorIds::new())` → `add_new::<GateColorIdSsbo>` →
/// `insert_resource(GateColorIds::new())` → `add_new::<HalftimberColorIdSsbo>` →
/// `insert_resource(HalftimberColorIds::new())`。签名（PDB）
/// `void(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, Commands)`。
///
/// 四个 SSBO 的 `SsboType` 实现已在 core 落地（旧版“typed-gap”注释失效），所以这里真实创建
/// GPU 缓冲；缺这一步时 `write_*_color_ids_to_gpu` 会在 `library.get::<WallColorIdSsbo>()` 失败。
/// CPU 表保持“已存在则不覆盖”：重复运行时不得抹掉已载入/已编辑的颜色。
pub fn init_color_ids_buffer(
    mut commands: Commands,
    device: Res<country_core::render::device::GraphicsDeviceResource>,
    library: Res<AssetSsboLibrary>,
    wall_colors: Option<Res<WallColorIds>>,
    gate_colors: Option<Res<country_core::resources::color_id::GateColorIds>>,
    half_colors: Option<Res<country_core::resources::color_id::HalftimberColorIds>>,
) {
    use country_core::resources::color_id::{
        GateColorIdSsbo, HalftimberColorIdSsbo, WallColorIdSsbo,
    };
    use country_core::resources::color_id_roof::RoofColorIdSsbo;
    // 原版 SSBO 创建与 CPU 表插入交错执行；插入是 Commands 延迟写，二者互不可见，故先集中创建
    // 四个 SSBO（顺序保持 Wall→Roof→Gate→Halftimber）再统一补表。
    library.add_new::<WallColorIdSsbo>(device.device());
    library.add_new::<RoofColorIdSsbo>(device.device());
    library.add_new::<GateColorIdSsbo>(device.device());
    library.add_new::<HalftimberColorIdSsbo>(device.device());
    insert_missing_color_tables(
        &mut commands,
        wall_colors.is_none(),
        gate_colors.is_none(),
        half_colors.is_none(),
    );
}

/// `init_color_ids_buffer` 的 CPU 表段：O `WallColorIds::new()`（0x140b19890）及 Gate/Halftimber
/// 同类构造的 `insert_resource`。已存在则不覆盖——重复运行不得抹掉已载入/已编辑的颜色。
fn insert_missing_color_tables(commands: &mut Commands, wall: bool, gate: bool, half: bool) {
    use country_core::resources::color_id::{GateColorIds, HalftimberColorIds};
    if wall {
        commands.insert_resource(WallColorIds::new());
    }
    if gate {
        commands.insert_resource(GateColorIds::new());
    }
    if half {
        commands.insert_resource(HalftimberColorIds::new());
    }
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
        let wall_id = cmd.wall_id();
        // O：`idx = id→索引`（0x1400885d0）；`0x4fff < idx` → panic（= idx <
        // MAX_NUMBER_OF_WALLS=0x5000）。
        assert!(
            wall_id.0 < MAX_NUMBER_OF_WALLS as u64,
            "wall_index < MAX_NUMBER_OF_WALLS"
        );
        let idx = wall_id.0 as usize;
        // O：读旧色、写新色。
        let old = wall_colors.0[idx];
        wall_colors.0[idx] = cmd.color;
        // O：PublicWalls::get → 命中才做 halftimber/石膏分类（反编译 do-while
        // 在 wall==null 时跳过处理直接进入下一事件）。
        if let Some(wall) = walls.get(wall_id) {
            if cmd.custom != 0 {
                // O：`WallStyle::is_halftimbered()`（内联于 0x142290820；返回未参与
                // 反编译分支，U——可能为 Ghidra 丢失的数据流；core `PublicWallState`
                // 未暴露该访问器，未调用，登记 GAP）。
                // O：折线 AABB（墙+0x40 点 ptr / +0x48 计数，每点 3×u32 (x,z,0)；
                // SIMD minps/maxps = glam Vec2::min/max，Aabb2::from_points 折叠）
                // → center → max_y → xvy。空折线时 Aabb2 保持 {f32::MAX, f32::MIN}，
                // center=0（与反编译一致）。
                let aabb =
                    Aabb2::from_points(wall.height_outline().iter().map(|p| Vec2::new(p[0], p[2])));
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
                    wall_id,
                ));
            } else if CUSTOM_COLOR_SET.contains(&old) {
                plaster_delete.write(country_core::resources::walls::plaster::DeletePlasterCmd(
                    wall_id,
                ));
            }
        }
    }
}

#[cfg(test)]
mod full_wall_id_tests {
    use super::*;
    use country_core::resources::{
        color_id::{GateColorIds, HalftimberColorIds, SetGateColorIdCmd, SetHalftimberColorIdCmd},
        roofs::geometry::surface::Curve2,
        walls::{
            InnerWalls, WallId, WallState,
            plaster::{CreatePlasterCmd, DeletePlasterCmd},
            publication_source::TerrainHeightGrid512,
        },
    };
    /// Same CPU-table segment as `init_color_ids_buffer` without the GPU device parameter.
    fn test_init_tables(
        mut commands: Commands,
        wall: Option<Res<WallColorIds>>,
        gate: Option<Res<GateColorIds>>,
        half: Option<Res<HalftimberColorIds>>,
    ) {
        insert_missing_color_tables(&mut commands, wall.is_none(), gate.is_none(), half.is_none());
    }
    fn world(public_id: Option<WallId>) -> World {
        let mut world = World::new();
        world.insert_resource(AssetSsboLibrary::new());
        world.init_resource::<WallColorIds>();
        world.init_resource::<GateColorIds>();
        world.init_resource::<HalftimberColorIds>();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetGateColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<PlayWallUiSound>>();
        world.init_resource::<Events<CreatePlasterCmd>>();
        world.init_resource::<Events<DeletePlasterCmd>>();
        let mut public = PublicWalls::default();
        if let Some(id) = public_id {
            let mut inner = InnerWalls::default();
            let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
            inner.new_wall_with_id(id, WallState::new(curve, 4.0, 0, 0, false));
            public.consume_with_paths(
                &mut inner,
                &TerrainHeightGrid512 {
                    encoded: vec![0u16; 512 * 512].into(),
                    minimum_y: 0.0,
                    maximum_y: 0.0,
                    sampling_fix: false,
                }, None, &country_core::resources::walls::public_walls::WallPublicationDirty::default(), &country_core::resources::walls::public_walls::WallSpatialHash::default());
            assert!(public.get(id).is_some());
        }
        world.insert_resource(public);
        world
    }
    #[test]
    fn wall_and_half_reject_high_identity_bits_before_touching_the_low_slot() {
        for half in [false, true] {
            let mut world = world(None);
            world.resource_mut::<WallColorIds>().0[7] = 91;
            world.resource_mut::<HalftimberColorIds>().0[7] = 91;
            let id = WallId(0x1_0000_0007);
            let mut schedule = Schedule::default();
            if half {
                world.send_event(SetHalftimberColorIdCmd::new(id, 12, 0));
                schedule.add_systems(crate::set_halftimber_color_id::set_halftimber_color_id);
            } else {
                world.send_event(SetWallColorIdCmd::new(id, 12, 0));
                schedule.add_systems(set_wall_color_id);
            }
            let result =
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
            assert!(result.is_err());
            assert_eq!(world.resource::<WallColorIds>().0[7], 91);
            assert_eq!(world.resource::<HalftimberColorIds>().0[7], 91);
            assert!(world.resource::<Events<CreatePlasterCmd>>().is_empty());
            assert!(world.resource::<Events<PlayWallUiSound>>().is_empty());
        }
    }
    #[test]
    fn gate_missing_full_identity_skips_without_mutating_a_low_identity_wall() {
        let mut world = world(Some(WallId(7)));
        world.resource_mut::<GateColorIds>().0[7] = 91;
        world.send_event(SetGateColorIdCmd::new(WallId(0x1_0000_0007), 4, 0));
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::set_gate_color_id::set_gate_color_id);
        schedule.run(&mut world);
        assert_eq!(world.resource::<GateColorIds>().0[7], 91);
        assert!(world.resource::<Events<PlayWallUiSound>>().is_empty());
    }
    #[test]
    fn gate_present_full_identity_reaches_the_original_full_width_bound() {
        let id = WallId(0x1_0000_0007);
        let mut world = world(Some(id));
        world.resource_mut::<GateColorIds>().0[7] = 91;
        world.send_event(SetGateColorIdCmd::new(id, 4, 0));
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::set_gate_color_id::set_gate_color_id);
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert_eq!(world.resource::<GateColorIds>().0[7], 91);
    }
    #[test]
    fn low_identity_keeps_current_color_and_plaster_behavior() {
        let id = WallId(7);
        let mut world = world(Some(id));
        world.send_event(SetWallColorIdCmd::new(id, 12, 0));
        world.send_event(SetGateColorIdCmd::new(id, 4, 0));
        world.send_event(SetHalftimberColorIdCmd::new(id, 5, 0));
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                set_wall_color_id,
                crate::set_gate_color_id::set_gate_color_id,
                crate::set_halftimber_color_id::set_halftimber_color_id,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 12);
        assert_eq!(world.resource::<GateColorIds>().0[7], 4);
        assert_eq!(world.resource::<HalftimberColorIds>().0[7], 5);
        let events = world.resource::<Events<CreatePlasterCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(cursor.read(events).next().unwrap().0, id);
        assert!(world.resource::<Events<DeletePlasterCmd>>().is_empty());
    }
    #[test]
    fn actual_init_then_setter_retains_color_on_second_frame_without_new_events() {
        let mut world = world(Some(WallId(7)));
        world.remove_resource::<WallColorIds>();
        world.remove_resource::<GateColorIds>();
        world.remove_resource::<HalftimberColorIds>();
        world.send_event(SetWallColorIdCmd::new(WallId(7), 12, 0));
        let mut schedule = Schedule::default();
        schedule.add_systems((test_init_tables, set_wall_color_id).chain());
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 12);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 12);
    }
    #[test]
    fn actual_init_preserves_all_loaded_color_tables() {
        let mut world = world(None);
        world.resource_mut::<WallColorIds>().0[7] = 91;
        world.resource_mut::<GateColorIds>().0[7] = 87;
        world.resource_mut::<HalftimberColorIds>().0[7] = 67;
        let mut schedule = Schedule::default();
        schedule.add_systems(test_init_tables);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 91);
        assert_eq!(world.resource::<GateColorIds>().0[7], 87);
        assert_eq!(world.resource::<HalftimberColorIds>().0[7], 67);
    }
    #[test]
    fn actual_init_only_creates_the_missing_table() {
        let mut world = world(None);
        world.resource_mut::<WallColorIds>().0[7] = 91;
        world.resource_mut::<HalftimberColorIds>().0[7] = 67;
        world.remove_resource::<GateColorIds>();
        let mut schedule = Schedule::default();
        schedule.add_systems(test_init_tables);
        schedule.run(&mut world);
        assert_eq!(world.resource::<WallColorIds>().0[7], 91);
        assert_eq!(world.resource::<HalftimberColorIds>().0[7], 67);
        assert_eq!(
            world.resource::<GateColorIds>().0.len(),
            MAX_NUMBER_OF_WALLS
        );
        assert_eq!(world.resource::<GateColorIds>().0[7], 0);
    }
}
