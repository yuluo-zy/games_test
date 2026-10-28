//! system-clutter（B 主基线）：自动杂物（autoclutter）散布链 + 屋顶城垛 + 形状缝合。
//!
//! 证据链（B VA；clutter-placement-analysis.md / clutter-extra-analysis.md）：
//! build 0x140652050 → populate_autoclutter_regions 0x1422d6ed0
//!   → process_autoclutter_candidates 0x1422ce260 → add_autoclutter_around_gates
//!   0x1422cc390 → add_autoclutter_around_windows 0x1422cefe0 → add_autoclutter_on_windows
//!   0x1422cd7d0 → add_birdnests 0x1422c9c70 → add_lantern_water_setdresssing 0x1422d2fa0
//!   → add_tree_setdressing 0x1422d3800 → manage_autoclutter_entities 0x1422d04e0
//! host_geometry：load_clutter_meshes 0x1422c0760 / inter_roof_merlons 0x1422d4870+
//!   0x141ce3a70 / inter_shape_stitches 0x141cd1a90+0x141ce65b0+0x141ce7190。
//!
//! 本 crate 按 B 机器码真实实现（不使用 reconstruction/clutter 的 TODO 占位）。

pub mod autoclutter;
pub mod inter_roof_merlons;
pub mod inter_shape_stitches;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;

use autoclutter::manage_autoclutter_entities::manage_autoclutter_entities;
use autoclutter::populate_autoclutter_regions::{autoclutter_subscribe_to_tabloids, populate_autoclutter_regions};
use autoclutter::process_autoclutter_candidates::process_autoclutter_candidates;
use inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons;
use inter_roof_merlons::detect_inter_roof_merlons::{detect_inter_roof_merlons, dirty_walls};
use inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners;
use inter_shape_stitches::stitch_bricks::{spawn_stitch_bricks, spawn_stitches};

/// 注册全部 clutter 系统与资源（B 0x140652050 `impl$0::build` 语义）。
/// 证据：`add_systems(...).chain()/.chain_ignore_deferred()/.in_set()/.run_if()`
/// 组合；`insert_resource`（AutoClutterState 等）；`EventRegistry::register_event`。
/// 链序（O）：populate → process → manage；add_* 规则在 process 前；stitch_bricks 与
/// preplaced 链按 `.chain()` 组合。
pub fn configure(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<autoclutter::AutoClutterState>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::MeshGroupWeights>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::DirtyTilesPublisher>();
    world.init_resource::<autoclutter::manage_autoclutter_entities::ManageRng>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::LanternRng>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::TreeRng>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonRng>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchRng>();
    world.init_resource::<autoclutter::LazyGridClutterCell>();
    // 2026-10-17 restorer-clutter-remaining：补齐系统参数所需资源（此前缺失会在运行期
    // panic；cargo check 不捕获）。宿主装配层仍可覆盖这些默认值。
    // The SSBO library is the actual readback owner; init preserves host data.
    world.init_resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
    world.init_resource::<autoclutter::read_bush_positions_from_gpu::BushGrid>();
    world.init_resource::<autoclutter::add_preplaced_autoclutter::PreplacedAutoClutter>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::TerrainDecoratorPositions>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::IsStarted>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::TreePositions>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsDragging>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsCorrupted>();
    world.init_resource::<autoclutter::manage_autoclutter_entities::ExistingAutoClutterKeys>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::DirtyTilesResource>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::RockMaskData>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::SessionMeta>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::PopulateLocal>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::RaycastWorldHandleClutter>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonBricks>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonHostGeometryList>();
    world.init_resource::<inter_roof_merlons::detect_inter_roof_merlons::WallAndRoofChanges>();
    world.init_resource::<inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsDescriptions>();
    world.init_resource::<inter_shape_stitches::detect_intra_shape_corners::Corners>();
    world.init_resource::<inter_shape_stitches::detect_intra_shape_corners::DirtyWallPairs>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchDescriptions>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::FloatingBrickParams>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchBrickInstances>();
    world.init_resource::<country_core::systems::mode_manager::AppMode>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::InterShapeBrickStitches>();
    world.init_resource::<inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHoles>();
    world.init_resource::<Events<inter_roof_merlons::inter_roof_merlons_audio::InterRoofMerlonAudioRequest>>();
    world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallChanged>>();
    world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();

    // Original build65226c/ee2723 registers this canonical reader. Keep the
    // current chain within Bevy's tuple limit; only add its data dependency.
    schedule.add_systems(
        autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu
            .after(autoclutter_subscribe_to_tabloids)
            .before(populate_autoclutter_regions),
    );
    schedule.add_systems(
        (
            autoclutter_subscribe_to_tabloids,
            autoclutter::autoclutter_animation::autoclutter_animation,
            autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates,
            autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows,
            autoclutter::add_autoclutter_on_windows::add_autoclutter_on_windows,
            autoclutter::add_birdnests::add_birdnests,
            autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing,
            autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing,
            autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter,
            populate_autoclutter_regions,
            process_autoclutter_candidates,
            manage_autoclutter_entities,
            // integration-registration 2026-10-17：dirty_walls（0x141cd16b0 判等）
            // 补独立调度注册点，链前序刷新脏墙集（detect_inter_roof_merlons 改读之）。
            dirty_walls,
            detect_inter_roof_merlons,
            construct_inter_roof_merlons,
            inter_roof_merlons::inter_roof_merlons_audio::request_inter_roof_merlon_audio,
            detect_intra_shape_corners,
            spawn_stitches,
            spawn_stitch_bricks,
        )
            .chain(),
    );
}

#[cfg(test)]
mod bush_registration_tests {
    use super::*;
    use country_core::{
        asset_libraries::ssbo_library::AssetSsboLibrary,
        render::ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer},
        systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo,
    };

    #[test]
    fn configure_registers_actual_reader_and_keeps_host_mapped_storage() {
        let mut world = World::new();
        let library = AssetSsboLibrary::new();
        library.register::<BushCpuSsbo>(MappedStorageBuffer {
            mapped: Box::leak(vec![BushCpuSsbo([0, 0x12345678])].into_boxed_slice()),
            buffer: Buffer { id: 71, generation: 1, _gap10: 0, byte_size: 8, _gap20: 0 },
        });
        world.insert_resource(library);
        let mut schedule = Schedule::default();
        configure(&mut world, &mut schedule);
        assert_eq!(world.resource::<AssetSsboLibrary>().get::<BushCpuSsbo>().unwrap().mapped[0].0, [0, 0x12345678]);
        schedule.initialize(&mut world).unwrap();
        assert_eq!(schedule.systems().unwrap().filter(|(_, system)| system.name().ends_with("::read_bush_positions_from_gpu")).count(), 1);
        assert_eq!(schedule.systems().unwrap().filter(|(_, system)| system.name().ends_with("::load_clutter_meshes")).count(), 0);
        // Running the whole clutter chain requires its separate host owners;
        // actual reader execution is covered in its World/library regressions.
    }
}
