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
//! 原证恢复范围与未闭合的 producer/宿主依赖见恢复报告；本注册入口尚非完整原版调度。

pub mod autoclutter;
pub mod inter_roof_merlons;
pub mod inter_shape_stitches;

use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::{Schedule, common_conditions::resource_exists};
use bevy_state::state::OnExit;

use country_core::app_loading_state::AppLoadingState;
use country_core::systems::GameSystemSet;

use autoclutter::load_clutter_meshes::load_clutter_meshes;
use autoclutter::manage_autoclutter_entities::manage_autoclutter_entities;
use autoclutter::add_birdnests::birdnests_subscribe_to_tabloids;
use autoclutter::populate_autoclutter_regions::{autoclutter_subscribe_to_tabloids, populate_autoclutter_regions};
use autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu;
use autoclutter::process_autoclutter_candidates::process_autoclutter_candidates;
use inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons;
use inter_roof_merlons::detect_inter_roof_merlons::{detect_inter_roof_merlons, dirty_walls};
use inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners;
use inter_shape_stitches::stitch_bricks::{on_exit_game_state, spawn_stitch_bricks, spawn_stitches};

/// `<system_clutter::ClutterPlugin as bevy_app::Plugin>::build`（B 0x140652050，3380 B）。
///
/// 机器码逐调用点（PyGhidra + ReVa；`Box<dyn System>` 邻接类型名串 + `Schedules` TypeId
/// `0x6a782374c0e6529b8e1bd251962ba784` 门）：
///
/// | 调用点 | 动作 | 内容 |
/// |---|---|---|
/// | 0x1406520f9 | `Schedules::add_systems(Startup)`（辅助 0x140ed9480，tuple3） | `(load_clutter_meshes, autoclutter_subscribe_to_tabloids, birdnests_subscribe_to_tabloids)`，三者无序 |
/// | 0x140652111/0x14065211c | `in_set(GameSystemSet(0x17))` + `SubApp::add_systems(Update)` | `detect_intra_shape_corners` |
/// | 0x14065212d/0x140652138 | 同上 | `detect_inter_roof_merlons`（原版自带 `ev_wall_delete` 等 EventReader） |
/// | 0x140652141…0x14065229f/0x140652310 | `chain_ignore_deferred`（顶层 tuple12）+ `in_set_inner(0x24)` + `SubApp::add_systems(Update)` | 见下 |
/// | 0x14065238b | `Schedules::add_systems(OnExit(AppLoadingState::Game))`，DL=1 | `on_exit_game_state` |
/// | 0x1406524a0…0x1406527c2 | 14× `insert_resource` | 见 [`ClutterPlugin::build`] 内逐项注释 |
/// | 0x140652828…0x140652b08 + 尾跳 0x140eee380 | 10× `register_event`（各带 `get_resource_id` 幂等门 = `add_event`） | 见内逐项注释 |
///
/// 0x24 系的顶层 12 成员（机器码 0x140ee2560 内 `into_configs` 调用序 + 2 个预构配置实参）：
/// 0 `read_bush_positions_from_gpu`；1 `reset_autoclutter_for_new_frame`；
/// 2 普通 tuple `(add_autoclutter_around_gates, add_autoclutter_around_windows, add_birdnests,
/// add_autoclutter_on_windows)`（成员间无序）；3 `add_preplaced_autoclutter.run_if(resource_exists<
/// PreplacedAutoClutter>)`（0x140652141，条件串在 0x140ee1693）；4 `populate_autoclutter_regions`；
/// 5 `add_tree_setdressing`；6 `add_lantern_water_setdresssing`；7 `process_autoclutter_candidates`；
/// 8 `manage_autoclutter_entities`；9 `autoclutter_animation`；10 `decoclutter_show_pins`；
/// 11 `((spawn_stitches, spawn_stitch_bricks).chain(), construct_inter_roof_merlons).chain()`
/// （0x14065214d 先造 `(spawn_stitches, spawn_stitch_bricks).chain()`，0x1406521bf 的 tuple2 以该配置
/// 为 S0、`construct_inter_roof_merlons` 为 S1——函数 0x140ee1a80 向量写入序证实）。
/// 成员 3/11 的位置取自 `registration-order-v1/CONTRACT.md`（PDB 泛型实参）；顶层 **必须**
/// `chain_ignore_deferred`（与 `chain()` 的区别是契约：manager→animation 之间不插 `ApplyDeferred`）。
pub struct ClutterPlugin;

impl Plugin for ClutterPlugin {
    fn build(&self, app: &mut App) {
        use country_core::resources::clutter_meshes::DecoratorAutoClutterChildren;
        use country_core::resources::preplaced_autoclutter::PreplacedAutoClutter;
        use country_core::resources::shape_intersections::ShapeIntersections;
        use country_core::systems::collision::{BirdNestCandidateLocations, WindowAutoClutterCandidates};
        use country_core::systems::ui_clutter::DeletedAutoClutter;
        use country_core::systems::ui_clutter::debug_spawn_clutter::{DebugSpawnAllClutterCmd, DebugSpawnClutterCmd};
        use country_core::systems::ui_clutter::decoclutter_show_pins::{ShowPinnedDecoClutter, decoclutter_show_pins};
        use country_core::systems::ui_clutter::defered_autoclutter_history::RecordAutoClutterHistory;
        use country_core::systems::ui_clutter::rmb_autoclutter::EvRightClickedAutoClutter;
        use country_core::systems::ui_clutter::ui_autoclutter::{InstaCreateDeco, InstaCreateStairPoint, StartPlaceDecoClutterInteraction};
        use country_core::systems::ui_clutter::ui_focus_decoclutter::{LastClonedClutter, RequestMoveClutter, RequestRotateDecorator};

        // 0x1406520f9：Startup，tuple3 `(load_clutter_meshes, autoclutter_subscribe_to_tabloids,
        // birdnests_subscribe_to_tabloids)`，三者无序。`load_clutter_meshes` 本 crate 版额外读
        // `Res<GameConfig>`（宿主适配，原版仅 `Res<AssetMeshLibrary>, Res<MeshWatch>, Commands`）。
        app.add_systems(
            Startup,
            (load_clutter_meshes, autoclutter_subscribe_to_tabloids, birdnests_subscribe_to_tabloids),
        );

        // 0x140652111 / 0x14065211c 与 0x14065212d / 0x140652138：两个独立 Update 注册，set 0x17。
        app.add_systems(Update, detect_intra_shape_corners.in_set(GameSystemSet(0x17)));
        app.add_systems(Update, detect_inter_roof_merlons.in_set(GameSystemSet(0x17)));

        // 0x140652141…0x140652310：顶层 tuple12 的 chain_ignore_deferred，整体 in_set_inner(0x24)。
        app.add_systems(
            Update,
            (
                read_bush_positions_from_gpu,
                autoclutter::reset_autoclutter_for_new_frame,
                (
                    autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates,
                    autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows,
                    autoclutter::add_birdnests::add_birdnests,
                    autoclutter::add_autoclutter_on_windows::add_autoclutter_on_windows,
                ),
                autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter
                    .run_if(resource_exists::<PreplacedAutoClutter>),
                populate_autoclutter_regions,
                autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing,
                autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing,
                process_autoclutter_candidates,
                manage_autoclutter_entities,
                autoclutter::autoclutter_animation::autoclutter_animation,
                decoclutter_show_pins,
                ((spawn_stitches, spawn_stitch_bricks).chain(), construct_inter_roof_merlons).chain(),
            )
                .chain_ignore_deferred()
                .in_set(GameSystemSet(0x24)),
        );

        // 0x14065238b：OnExit(AppLoadingState::Game)（DL=1；与 CritterPlugin 同形 928 B 单态）。
        app.add_systems(OnExit(AppLoadingState::Game), on_exit_game_state);

        // 0x1406524a0…0x1406527c2：14 个 insert_resource，按机器码顺序。
        // ① AutoClutterState：`RandomState::new()` + `vec![Vec::new(); 0x1000]`（0x140652402，64×64 懒网格）+ epoch=0。
        app.insert_resource(autoclutter::AutoClutterState {
            lazy_grid: vec![Vec::new(); 0x1000],
            ..Default::default()
        });
        // ② InterShapeBrickStitches（空 Vec）。
        app.insert_resource(inter_shape_stitches::stitch_bricks::InterShapeBrickStitches::default());
        // ③ IntraShapeCorners（0x140652500，空 Vec，align 8）。
        app.insert_resource(inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners::default());
        // ④ ShapeIntersections（0x14065257c；country_core::resources::shape_intersections，空表 +
        // `RandomState::new()` 共 0x30 B）。
        app.insert_resource(ShapeIntersections::default());
        // ⑤ StitchBrickAlloc(0usize)（0x140652586，`XOR EDX,EDX`）。
        app.insert_resource(inter_shape_stitches::stitch_bricks::StitchBrickAlloc::default());
        // ⑥ InterRoofMerlons（0x1406525b6，空 Vec，align 8）。
        app.insert_resource(inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons::default());
        // ⑦ InterRoofMerlonBrickCount(0usize)（0x1406525c0）。
        app.insert_resource(inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount::default());
        // ⑧ BushGrid：`vec![cell; 0x1000]`（0x140652611）+ 内联 `Publisher<DirtyTiles>`（+0x18）。
        app.insert_resource(autoclutter::read_bush_positions_from_gpu::BushGrid::default());
        // ⑨ BirdNestCandidateLocations（空 Vec）。
        app.insert_resource(BirdNestCandidateLocations::default());
        // ⑩ WindowAutoClutterCandidates（空 Vec，align 16）。
        app.insert_resource(WindowAutoClutterCandidates::default());
        // ⑪ AutoClutterDirtySubscriptions（空 Vec）。
        app.insert_resource(autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions::default());
        // ⑫ DeletedAutoClutter：`HashSet::default()`（`RandomState::new()` 取线程局部 KEYS，0x140652729）。
        app.insert_resource(DeletedAutoClutter::default());
        // ⑬ LastClonedClutter（0x140652754；`{ Quat::IDENTITY, 0u8 }`，见类型文档）。
        app.insert_resource(LastClonedClutter::default());
        // ⑭ DecoratorAutoClutterChildren（0x1406527c2；空 HashMap + RandomState）。
        app.insert_resource(DecoratorAutoClutterChildren::default());

        // 0x140652828…0x140652b08 + 尾跳 0x140eee380：10 个 register_event（`add_event` 等价：已存在则跳过）。
        app.add_event::<StartPlaceDecoClutterInteraction>(); // ①（0x140652828）
        app.add_event::<InstaCreateDeco>(); // ②
        app.add_event::<InstaCreateStairPoint>(); // ③
        app.add_event::<EvRightClickedAutoClutter>(); // ④
        app.add_event::<RequestRotateDecorator>(); // ⑤
        app.add_event::<RecordAutoClutterHistory>(); // ⑥
        app.add_event::<RequestMoveClutter>(); // ⑦
        // ⑧ ShowPinnedDecoClutter：原版是 Event（56 B）；core 已改为 Event，`decoclutter_show_pins` 经 EventReader 读取。
        app.add_event::<ShowPinnedDecoClutter>();
        app.add_event::<DebugSpawnAllClutterCmd>(); // ⑨
        app.add_event::<DebugSpawnClutterCmd>(); // ⑩（尾跳 0x140eee380）

        // ---- 非原版 build 面：本 crate 近似系统所需的 crate 局部资源/事件 ----
        init_adapter_resources(app.world_mut());
        app.add_event::<country_core::systems::audio::PlayClutterSoundCmd>();
        app.add_event::<inter_roof_merlons::inter_roof_merlons_audio::InterRoofMerlonAudioRequest>();
        app.add_event::<country_core::resources::walls::public_walls::OnWallChanged>();
        app.add_event::<country_core::resources::walls::public_walls::OnWallDeleted>();
        // `dirty_walls`（0x141cd16b0 的判等辅助被本 crate 拆成独立系统）：原版 detect_* 各自带 EventReader，
        // 本 crate 的 detect_* 读 `WallAndRoofChanges.dirty_walls`，须先刷新（近似，登记）。
        app.add_systems(
            Update,
            dirty_walls
                .in_set(GameSystemSet(0x17))
                .before(detect_intra_shape_corners)
                .before(detect_inter_roof_merlons),
        );
    }
}

/// 本 crate 近似系统需要、但原版 `ClutterPlugin::build` 不插入的 crate 局部资源（`init_resource`，
/// 不覆盖宿主已插入者）。`AssetSsboLibrary`/`AppMode`/`WorldRaster<512>`/`WallHoles` 等跨 crate 资源
/// 由 core/宿主提供，此处不创建。
pub fn init_adapter_resources(world: &mut World) {
    world.init_resource::<autoclutter::populate_autoclutter_regions::MeshGroupWeights>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::LanternRng>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::TreeRng>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonRng>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchRng>();
    world.init_resource::<autoclutter::LazyGridClutterCell>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::TerrainDecoratorPositions>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::IsStarted>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::TreePositions>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsDragging>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsCorrupted>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::DirtyTilesResource>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::RockMaskData>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::SessionMeta>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::PopulateLocal>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonBricks>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonHostGeometryList>();
    world.init_resource::<inter_roof_merlons::detect_inter_roof_merlons::WallAndRoofChanges>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchDescriptions>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::FloatingBrickParams>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchBrickInstances>();
    world.init_resource::<inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHoles>();
}

/// Existing clutter schedule entry. This still contains legacy producer adapters;
/// it is not a complete reconstruction of B 0x140652050 or installed by run_game.
/// 证据：`add_systems(...).chain()/.chain_ignore_deferred()/.in_set()/.run_if()`
/// 组合；`insert_resource`（AutoClutterState 等）；`EventRegistry::register_event`。
/// 链序（O）：populate → process → manage；add_* 规则在 process 前；stitch_bricks 与
/// preplaced 链按 `.chain()` 组合。
pub fn configure(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<autoclutter::AutoClutterState>();
    world.init_resource::<country_core::systems::collision::WindowAutoClutterCandidates>();
    world.init_resource::<Events<country_core::systems::audio::PlayClutterSoundCmd>>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::MeshGroupWeights>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>();
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
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::TerrainDecoratorPositions>();
    world.init_resource::<autoclutter::add_lantern_water_setdresssing::IsStarted>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::TreePositions>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsDragging>();
    world.init_resource::<autoclutter::add_tree_vegetation_setdressing::IsCorrupted>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::DirtyTilesResource>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::RockMaskData>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::SessionMeta>();
    world.init_resource::<autoclutter::populate_autoclutter_regions::PopulateLocal>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonBricks>();
    world.init_resource::<inter_roof_merlons::construct_inter_roof_merlons::MerlonHostGeometryList>();
    world.init_resource::<inter_roof_merlons::detect_inter_roof_merlons::WallAndRoofChanges>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchDescriptions>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::FloatingBrickParams>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::StitchBrickInstances>();
    world.init_resource::<country_core::systems::mode_manager::AppMode>();
    world.init_resource::<inter_shape_stitches::stitch_bricks::InterShapeBrickStitches>();
    world.init_resource::<inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHoles>();
    world.init_resource::<Events<inter_roof_merlons::inter_roof_merlons_audio::InterRoofMerlonAudioRequest>>();
    world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallChanged>>();
    world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();

    // The original per-frame reset clears output/stitches and copies the real
    // GameTime epoch. Keep it before all output producers in this existing
    // schedule. Actual decorator collector ordering belongs to App integration.
    schedule.add_systems(
        autoclutter::reset_autoclutter_for_new_frame
            .before(autoclutter_subscribe_to_tabloids),
    );

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
            autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates,
            autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows,
            autoclutter::add_autoclutter_on_windows::add_autoclutter_on_windows,
            autoclutter::add_birdnests::add_birdnests,
            autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing,
            autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing,
            autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter,
            populate_autoclutter_regions,
            process_autoclutter_candidates,
            // Original outer members8/9: manager before animation, with no
            // automatic Commands flush between them. Other legacy members
            // in this entry remain subject to full plugin recovery.
            (manage_autoclutter_entities,
             autoclutter::autoclutter_animation::autoclutter_animation)
                .chain_ignore_deferred(),
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
        library.register::<BushCpuSsbo>(MappedStorageBuffer::from_mapping(
            Box::leak(vec![BushCpuSsbo([0, 0x12345678])].into_boxed_slice()),
            Buffer { id: 71, generation: 1, _gap10: 0, byte_size: 8, _gap20: 0 },
        ));
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

#[cfg(test)]
mod plugin_registration_tests {
    use super::*;
    use bevy_app::{App, Startup, Update};
    use bevy_ecs::schedule::{ScheduleLabel, Schedules};

    /// ClutterPlugin 的 Update/Startup/OnExit 调度图必须可构建（无环、无层级冲突），
    /// 且原版 build 插入的资源/事件均已落地。
    #[test]
    fn plugin_builds_valid_schedules_and_inserts_original_resources() {
        let mut app = App::new();
        app.add_plugins(ClutterPlugin);
        let world = app.world_mut();
        assert!(world.contains_resource::<autoclutter::AutoClutterState>());
        assert_eq!(world.resource::<autoclutter::AutoClutterState>().lazy_grid.len(), 0x1000);
        assert!(world.contains_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>());
        assert!(world.contains_resource::<country_core::systems::collision::BirdNestCandidateLocations>());
        assert!(world.contains_resource::<Events<country_core::systems::ui_clutter::ui_autoclutter::InstaCreateDeco>>());
        for label in [Startup.intern(), Update.intern()] {
            world.schedule_scope(label, |world, schedule| {
                schedule.initialize(world).expect("clutter schedule graph must build");
            });
        }
        let mut schedules = app.world_mut().resource_mut::<Schedules>();
        let update = schedules.get_mut(Update).unwrap();
        let names: Vec<String> = update.systems().unwrap().map(|(_, s)| s.name().to_string()).collect();
        for expected in ["detect_intra_shape_corners", "detect_inter_roof_merlons", "manage_autoclutter_entities", "autoclutter_animation", "decoclutter_show_pins", "construct_inter_roof_merlons", "spawn_stitch_bricks"] {
            assert_eq!(names.iter().filter(|n| n.ends_with(&format!("::{expected}"))).count(), 1, "{expected}");
        }
    }
}

#[cfg(test)]
mod plugin_closure_tests {
    use super::*;
    use bevy_ecs::schedule::Schedules;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
    use country_core::resources::world_raster::DirtyTilePublisher;
    use country_core::resources::world_raster_defs::{GardenRaster, PathRaster, WaterRaster};
    use country_core::systems::collision::world::RaycastWorld;

    /// 机器码顺序的 14 个 insert_resource 与 10 个 register_event 全部落地（含本轮补齐的类型）。
    #[test]
    fn plugin_inserts_all_original_resources_and_events() {
        let mut app = App::new();
        app.add_plugins(ClutterPlugin);
        let world = app.world();
        assert!(world.contains_resource::<autoclutter::AutoClutterState>());
        assert!(world.contains_resource::<inter_shape_stitches::stitch_bricks::InterShapeBrickStitches>());
        assert!(world.contains_resource::<inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>());
        assert!(world.contains_resource::<country_core::resources::shape_intersections::ShapeIntersections>());
        assert_eq!(world.resource::<inter_shape_stitches::stitch_bricks::StitchBrickAlloc>().0, 0);
        assert!(world.contains_resource::<inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>());
        assert_eq!(world.resource::<inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>().0, 0);
        assert!(world.contains_resource::<autoclutter::read_bush_positions_from_gpu::BushGrid>());
        assert!(world.contains_resource::<country_core::systems::collision::BirdNestCandidateLocations>());
        assert!(world.contains_resource::<country_core::systems::collision::WindowAutoClutterCandidates>());
        assert!(world.contains_resource::<autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>());
        assert!(world.contains_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>());
        let last = world.resource::<country_core::systems::ui_clutter::ui_focus_decoclutter::LastClonedClutter>();
        assert_eq!((last.rotation, last.tag), (glam::Quat::IDENTITY, 0));
        assert!(world.contains_resource::<country_core::resources::clutter_meshes::DecoratorAutoClutterChildren>());
        assert!(world.contains_resource::<Events<country_core::systems::ui_clutter::ui_autoclutter::StartPlaceDecoClutterInteraction>>());
        assert!(world.contains_resource::<Events<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>>());
    }

    /// Startup 三成员（load_clutter_meshes、autoclutter_subscribe_to_tabloids、birdnests_subscribe_to_tabloids）。
    #[test]
    fn startup_registers_the_three_original_members() {
        let mut app = App::new();
        app.add_plugins(ClutterPlugin);
        let world = app.world_mut();
        world.schedule_scope(Startup, |world, schedule| {
            schedule.initialize(world).unwrap();
            let names: Vec<String> = schedule.systems().unwrap().map(|(_, s)| s.name().to_string()).collect();
            for expected in ["load_clutter_meshes", "autoclutter_subscribe_to_tabloids", "birdnests_subscribe_to_tabloids"] {
                assert_eq!(names.iter().filter(|n| n.ends_with(&format!("::{expected}"))).count(), 1, "{expected}");
            }
        });
        let _ = world.resource::<Schedules>();
    }

    /// 真实 App 的世界：宿主资源齐全时，两个订阅系统都通过参数校验并完成订阅；
    /// clutter 不再持有私有 RaycastWorld 副本（订阅的是宿主 `RaycastWorld`）。
    #[test]
    fn subscription_systems_validate_and_subscribe_on_a_real_app_world() {
        let mut app = App::new();
        app.add_plugins(ClutterPlugin);
        let world = app.world_mut();
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.run_system_once(autoclutter_subscribe_to_tabloids).expect("params valid on real world");
        world.run_system_once(birdnests_subscribe_to_tabloids).expect("params valid on real world");
        let handles = world
            .resource::<autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>()
            .subscriber
            .lock()
            .unwrap()
            .len();
        assert_eq!(handles, 13);
        assert!(world.contains_resource::<autoclutter::add_birdnests::BirdNestLazySubscriber>());
    }
}

