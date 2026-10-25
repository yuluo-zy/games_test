//! system-wall-constructor：主墙构造系统（拱/檐梁/围栏/门/阶梯/支柱/石膏/半木构）。
//!
//! 原始布局（PDB 源路径 `crates/systems/wall-constructor/src/lib.rs`）：
//! 本 crate 消费 country-core 的墙权威（PublicWalls/PublicWallState 0x2a0
//! 投影、共享建筑策略链）并生产 96B BrickTransformSsbo /
//! 64B WoodenPlankSsbo 等实例记录。SSBO/渲染上传（FUN_141209c40 注册表 +
//! 虚调取缓冲 + Arc/condvar 握手）为渲染边界后置登记。
//!
//! 注册入口 `add_wall_constructor_systems` 按原调度阶段把系统接入 Bevy
//! 调度（`WallCpuStage::Bricks`/`Holes` 等 country-core 阶段集）。
pub mod arch_walker;
pub mod construct_arches;
pub mod construct_elevation_supports;
pub mod construct_fences;
pub mod construct_floors;
pub mod construct_gates;
pub mod construct_lintels;
pub mod construct_platform_stairs;
pub mod construct_playermade_stairs;
pub mod construct_walls;
pub mod curved_halftimber;
pub mod gpu_output;
pub mod init_wall_height_and_arch_segment_height;
pub mod place_pillars;
pub mod plaster_systems;
pub mod segmentation;
pub mod ui_arch_controls;
pub mod ui_lintel_controls;
pub mod utils;
pub mod wall_constructor;

use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;

/// 原 `system_wall_constructor::impl$N` 注册（按原调度阶段）。
/// GPU 直写系统（gpu_clear_wall_gate_pieces 0x140773c00 /
/// gpu_flush_plank_buffers 0x140774010）在 render 边界接线后加入
/// （当前登记为结构桩，见 lib.rs 各函数注释）。
pub fn add_wall_constructor_systems(schedule: &mut Schedule) {
    use country_core::systems::wall::registration::WallCpuStage;
    schedule.add_systems(
        (
            init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,
            segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
            segmentation::sample_wall_path_intersections::sample_wall_path_intersections,
            segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs,
        )
            .chain()
            .in_set(WallCpuStage::Edit),
    );
    schedule.add_systems(
        (
            construct_walls::construct_walls,
            construct_arches::construct_arches,
            construct_lintels::construct_lintels,
            construct_fences::construct_fences,
            construct_gates::systems::update_gates_hashmap,
            construct_gates::systems::construct_gates,
            place_pillars::place_pillars,
            curved_halftimber::curved_halftimber_maintain_entities,
            construct_platform_stairs::construct_platform_stairs,
            construct_playermade_stairs::construct_playermade_stairs_arches_n_walls::<true>,
            construct_playermade_stairs::pillars_visual::construct_playermade_stair_pillars_visuals,
            construct_elevation_supports::construct_circle_supports,
            construct_elevation_supports::construct_rectangle_columns,
            construct_elevation_supports::construct_rectangle_brackets,
        )
            .chain()
            .in_set(WallCpuStage::Bricks),
    );
    schedule.add_systems(
        (
            plaster_systems::maintain_plaster,
            plaster_systems::create_plaster,
        )
            .chain()
            .in_set(WallCpuStage::Holes),
    );
}

/// 原 `system_wall_constructor::gpu_flush_plank_buffers`（0x140774010）。
///
/// SystemParam 面：`(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>)`。
/// 机器码：`library.get_mut::<WoodenPlankSsbo>().unwrap()` → `SparseInstanceBuffer::gpu_flush`
/// （非内联调用 0x14120b030）；随后 `get_mut::<SparkleSsbo>().unwrap()` → 同一 `gpu_flush<T>` 的
/// 内联实例（puffin 作用域 `SparseInstanceBuffer<_>::gpu_flush`、"uploading chunk batches"）。
/// 两个缓冲各自先 `take_out_waste` 再合并脏 chunk 区间上传。两次 `get_mut` 各写一次 ResMut 变更 tick。
pub fn gpu_flush_plank_buffers(
    device: bevy_ecs::prelude::Res<country_core::render::device::GraphicsDeviceResource>,
    mut library: bevy_ecs::prelude::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use country_core::resources::sparkle::SparkleSsbo;
    library.get_mut::<WoodenPlankSsbo>().unwrap().gpu_flush(device.device());
    library.get_mut::<SparkleSsbo>().unwrap().gpu_flush(device.device());
}

/// 原 `system_wall_constructor::gpu_clear_wall_gate_pieces`（0x140773c00，956B）。
///
/// 原版参数面（`System::run_without_applying_deferred` 0x140451010 的取参顺序）：
/// `(Res<AssetSsboLibrary>, (Local<EventCursor>, Res<Events<OnWallChanged>>), Res<PublicWalls>)`。
/// 机器码：
/// 1. `AssetSsboLibrary::get::<WoodenPlankSsbo>()`（FrozenMap 按 NameHash 查 + 名字比较）；缺失 → `Option::unwrap`
///    panic（**即使本帧没有事件也 panic**）。
/// 2. 逐个事件（8B = WallId）：`PublicWalls` 记录表（桶 0x2a8，键 WallId）索引，缺失 →
///    `"no entry found for key"` panic；记录 +0x130 的 `Option<Box<Allocation>>`（木板 chunk 表，
///    `Mutex<Vec<ChunkId>>`）为 `None` → `Option::unwrap` panic；加锁后构造
///    `SparseInstanceBufferWriter<WoodenPlankSsbo>` 并 `clear()`（byte_map 清零、active 位清除、chunk 逆序归还空闲栈）。
///    不清脏位，随后的 `gpu_flush_plank_buffers` 仍会上传这些 chunk。
///
/// 本仓库的差异（设计更新，业务语义不变）：墙记录 +0x130 的 per-wall 木板分配不在 `PublicWallState`
/// 里，而由 `gpu_output::ConstructorGpuOutput` 拥有（lintel/fence 追加方共享同一 per-wall 分配）；故取参
/// 面多一个 `ResMut<WallConstructorOutput>`，库为 `ResMut`（`writer` 需要 `&mut` 存储）。
/// 分配不存在时 `clear_planks` 会创建空分配（原版此处 `unwrap` panic，仅在首次变更前无分配时可区分）。
pub fn gpu_clear_wall_gate_pieces(
    mut library: bevy_ecs::prelude::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut changed: bevy_ecs::prelude::EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    walls: bevy_ecs::prelude::Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut output: bevy_ecs::prelude::ResMut<construct_arches::WallConstructorOutput>,
) {
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    let storage = library.get_mut::<WoodenPlankSsbo>().unwrap();
    for event in changed.read() {
        walls
            .get(event.0)
            .expect("no entry found for key");
        output.gpu.clear_planks(storage, event.0.0);
    }
}

/// 原 `system_wall_constructor::debug_force_update_arches`（0x14003bb80）：
/// B 中该符号为 **1B `ret` 存根**（C3+0xCC 填充，无真实业务）；空函数体与之
/// **判等**（非"未恢复"）。若 A/C 版有真实语义则需另行恢复。
pub fn debug_force_update_arches() {}

#[cfg(test)]
mod gpu_system_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;
    use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};

    /// 参数面（Res/ResMut/Query 访问冲突）必须能在调度里初始化。
    #[test]
    fn gpu_flush_plank_buffers_param_surface_initializes() {
        let mut world = World::new();
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(gpu_flush_plank_buffers);
        schedule.initialize(&mut world).unwrap();
    }

    /// CPU 影子夹具：只负责建缓冲，任何 GPU 上传都显式失败。
    struct TestDevice(u64);
    impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for TestDevice {
        fn create_buffer(
            &self,
            desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<
            country_core::render::ssbo::rhapsody_boundary::Buffer,
            country_core::render::ssbo::rhapsody_boundary::BackendError,
        > {
            Ok(country_core::render::ssbo::rhapsody_boundary::Buffer {
                id: self.0,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
            _data: &[u8],
            _ranges: &[std::ops::Range<usize>],
            _offset: usize,
        ) -> Result<(), country_core::render::ssbo::rhapsody_boundary::BackendError> {
            Err(country_core::render::ssbo::rhapsody_boundary::BackendError::invalid_usage(
                "CPU-shadow fixture has no GPU upload device",
            ))
        }
    }

    fn clear_world() -> World {
        let mut world = World::new();
        world.init_resource::<Events<OnWallChanged>>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<construct_arches::WallConstructorOutput>();
        world
    }

    /// 原版 0x140773c00 先 `get::<WoodenPlankSsbo>()`，缺失即 `Option::unwrap` panic（与有无事件无关）。
    #[test]
    fn gpu_clear_wall_gate_pieces_panics_without_plank_storage() {
        let mut world = clear_world();
        world.insert_resource(AssetSsboLibrary::new());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = world.run_system_once(gpu_clear_wall_gate_pieces);
        }));
        assert!(result.is_err());
    }

    /// 已注册木板缓冲且无事件：不 panic、不改动缓冲（不创建分配、不清脏位）。
    #[test]
    fn gpu_clear_wall_gate_pieces_without_events_is_noop() {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        let mut world = clear_world();
        let library = AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(
            &TestDevice(1),
            None,
        ));
        world.insert_resource(library);
        world.run_system_once(gpu_clear_wall_gate_pieces).unwrap();
        let library = world.resource::<AssetSsboLibrary>();
        assert!(library.get::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().is_empty());
    }

    /// 变更事件指向未发布的墙：原版 `PublicWalls` 索引缺失 panic（"no entry found for key"）。
    #[test]
    fn gpu_clear_wall_gate_pieces_unknown_wall_panics() {
        use country_core::render::sparse_instance_buffer::SparseInstanceBuffer;
        use country_core::resources::walls::WallId;
        let mut world = clear_world();
        let library = AssetSsboLibrary::new();
        library.register::<WoodenPlankSsbo>(SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(
            &TestDevice(1),
            None,
        ));
        world.insert_resource(library);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(WallId(9)));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = world.run_system_once(gpu_clear_wall_gate_pieces);
        }));
        assert!(result.is_err());
    }

    /// `debug_force_update_arches` 在 B 中是 1 字节 `ret`（0x14003bb80）：运行为无操作。
    #[test]
    fn debug_force_update_arches_is_noop() {
        let mut world = World::new();
        world.run_system_once(debug_force_update_arches).unwrap();
    }
}
