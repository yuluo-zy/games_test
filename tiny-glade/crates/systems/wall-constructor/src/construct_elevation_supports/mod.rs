//! 高程支撑构造（construct_elevation_supports/mod.rs）。
//!
//! 5 系统（fresh decomp 全落盘）：
//! - `overhang_subscribe_to_tabloids`（0x140786440）
//! - `detect_overhang_support_segments`（0x1407867d0）
//! - `construct_circle_supports`（0x140788160）
//! - `construct_rectangle_columns`（0x140788a80）
//! - `construct_rectangle_brackets`（0x1407896b0）
use bevy_ecs::prelude::*;

pub mod circle_bottom_layer;
pub mod circle_categorize;
pub mod circle_pillars;
pub mod circle_turret;
pub mod find_overlap_segments;
pub mod maintain_stone_column_colliders;
pub mod rectangle_bottom_layer;
pub mod rectangle_large_brackets;
pub mod rectangle_pillars;
pub mod rectangle_small_brackets;
pub mod util_cross_brick_pillar;
pub mod util_pillar_construction;

/// B `OverhangSupportDirtySubscriptions`（`add_wall_systems` 0x140493120 的 `insert_resource` 值为
/// 三个 qword `(0, 8, 0)` = 空 `Vec`，cap=0 / dangling ptr=8 / len=0）。`overhang_subscribe_to_tabloids`
/// 0x140786440 每个元素为 8 字节：`Subscriber<DirtyTiles>`（tabloid `Arc<Mutex<Subscription>>`）。
/// 旧实现的 `hashbrown::HashMap<u64,u32>` 无机器码依据，已替换。
#[derive(bevy_ecs::resource::Resource, Clone, Default)]
pub struct OverhangSupportDirtySubscriptions {
    pub subscribers: Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>>,
}

/// B `overhang_subscribe_to_tabloids`（0x140786440，Startup）。
///
/// SystemParam 面（机器码 param_1..3，三者都写变更 tick = 均为 `ResMut`）：
/// `(ResMut<OverhangSupportDirtySubscriptions>, ResMut<RaycastWorld>, ResMut<TerrainHeightsData>)`；
/// 第三个资源在 `+0x18` 处持有 `Publisher<DirtyTiles>`（布局与 `TerrainHeightsData` 一致，按布局识别，
/// 未能由符号名交叉确认）。
/// 顺序：`RaycastWorld::subscribe_to_changes(world, slot)` 依次取槽 0,1,4,5,6,7,2,3,0xc
/// （共 9 个，每次 push 进 Vec），最后 `Publisher<DirtyTiles>::subscribe`（地形脏瓦片）push。
pub fn overhang_subscribe_to_tabloids(
    mut subscriptions: ResMut<OverhangSupportDirtySubscriptions>,
    mut raycast: ResMut<country_core::systems::collision::world::RaycastWorld>,
    mut terrain: ResMut<country_core::resources::terrain_height::TerrainHeightsData>,
) {
    for slot in [0u8, 1, 4, 5, 6, 7, 2, 3, 0xc] {
        let subscriber = raycast.subscribe_to_changes(slot);
        subscriptions.subscribers.push(subscriber);
    }
    let subscriber = terrain.subscribe();
    subscriptions.subscribers.push(subscriber);
}

/// 原版 `FUN_140787d80`（0x140787d80，884B，`detect_overhang_support_segments` 与
/// `construct_circle_supports` 同源文件内私有 helper，无符号名）。
///
/// 机器码顺序：`AssetSsboLibrary` 的 `FrozenMap::get` 取
/// `country_core::resources::roofs::roof_buffers::WoodenPlankSsbo`（类型名 0x3d 字节 memcmp，
/// rodata 0x142c1a968 逐字节核对）→ 对墙状态 `+0x140` 的 per-wall 分配加锁（`+0x18` 自旋位）→
/// `SparseInstanceBufferWriter::clear`；再同样取 `BrickTransformSsbo`（0x3a 字节，0x142c1a9a5）→
/// 墙状态 `+0x150` 分配 → `clear`。库缺失 / 类型名不符 / 分配缺失均为 `Option::unwrap` panic。
/// 本仓库的 per-wall 分配在 `ConstructorGpuOutput`（gpu_output.rs，`planks`/`bricks`），故落成其
/// `clear_planks` → `clear_bricks`（先 plank 后 brick，与机器码顺序一致）。
pub(crate) fn clear_wall_instance_owners(
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::gpu_output::ConstructorGpuOutput,
    wall: u64,
) {
    let planks = library
        .get_mut::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
        .expect("WoodenPlankSsbo must be registered (FUN_140787d80 unwrap)");
    output.clear_planks(planks, wall);
    let bricks = library
        .get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
        .expect("BrickTransformSsbo must be registered (FUN_140787d80 unwrap)");
    output.clear_bricks(bricks, wall);
}

/// 2026-10-20 取证（0x1407867d0，4258B，fresh 反编译 + 逐段核对）。
///
/// 15 个 raw 参数面（与 `add_wall_systems` 注册顺序一致；`+0x1c` 为 change tick 写回）：
/// - p1/p2：`EventReader<E>`（E 为 8 字节载荷，`Events` 布局：events_a `+8/+0x10` 起始计数 `+0x18`、
///   events_b `+0x28/+0x30`、`+0x38`、总数 `+0x40`，元素 stride 0x10 = `{id, payload}`）。各自把未读事件的
///   载荷收集到一个 `HashMap<WallId,()>`（`RandomState::new` ×2，`HashMap::extend`）：集合 A=p1，集合 B=p2；
/// - p3：`ResMut<OverhangSupportDirtySubscriptions>`（已实现）；p4：`Res<AssetSsboLibrary>` 引用（传给 `FUN_140787d80`）；
/// - p5：`Res<PublicWalls>`（`+0x18` 为墙数，stride 0x2a8 的 hashbrown 表）；p6：屋顶 `Query`（传给
///   `OverhangSupportFinderCache::new` 的第三参）；p7：重叠表 `Res`（`find_overhang_support_segments` 的 param_6）；
/// - p8：矩形墙悬挑表 `ResMut`；p9：圆形墙悬挑表 `ResMut`（二者同布局：`+0x20` Vec 容量/`+0x28` 指针/`+0x30` 长度，
///   条目 stride 0x28 = `{Vec<段> 0x18B, wall:u64 @0x18, version:u64 @0x20}`，`+0x40` 为单调 version 计数；
///   前 0x20B 为 hashbrown 索引表）；
/// - p10：`ResMut<` 0x20B 值的 `HashMap<WallId,_>` `>`（`HashMap::insert` 默认值 `{Vec::new(), 0..}`）；
/// - p11：`ResMut<StoneColumnCollisionStorage>`；p12/p13：仅排空的 `EventReader`（`last_event_count := event_count`，
///   任一有未读 → 置位 p14）；p14：`Local<bool>` 强制重算标志；p15：`ResMut<ElevationSupportStats>`
///   （`+0x20/+0x28/+0x30` Vec，条目 `{Vec 0x18B(0,4,0), wall @0x18, version @0x20}`，`+0x40` 计数）。
///
/// 体内逐步：
/// 1. puffin scope；
/// 2. 排空 p12、p13：`flag = 任一曾有未读`；二者各自 `last = event_count`；任一未读 → `*p14 = 1`；
/// 3. 收集集合 A、B；`run = !A.is_empty() || *p14 || !B.is_empty()`（`bVar42`）；
/// 4. `DirtyTiles::new_clear()`；对 p3 的每个订阅者 `Subscriber::take()`：有则合并进 `DirtyTiles`，
///    `got = true`；`*p14 = 0`；
/// 5. 仅当 `run || got`：`OverhangSupportFinderCache::new(PublicWalls, p6)`，遍历 `PublicWalls` 每个墙；
///    - 墙状态高度标量（`+0x280`-0x20 处的 f32）`== 0.0`：`StoneColumnCollisionStorage::clear_and_get_mut(wall)`、
///      p10 `insert(wall, 空)`、`FUN_140787d80`、p15 追加 `{空,wall,version}`、`version += 1`，下一个墙；
///    - 否则 `in_a = A∋wall`，`in_b = B∋wall`；墙形状 tag（`+0x258`）`< 2` 才处理（否则跳过）：
///      圆形（tag bit0==0）：`in_a` 为假时取 `Circle2d::aabb`，`DirtyTiles::intersects_world_space_rect`；
///      `!in_b && !intersects` 则跳过；矩形（tag bit0==1）：`in_a` 为假时取矩形外接 AABB，`!intersects` 则跳过
///      （矩形不看 `in_b`，机器码如此）；
///    - 通过则 `FUN_140787d80`；圆形 `find_overhang_support_segments(.., false, 0.0)` → 写 p9；矩形
///      `(.., true, 0.6f=0x3f19999a)` → 写 p8；各追加 `{段 Vec, wall, version}`、`version += 1`。
///
/// 阻塞项（未闭合）：`find_overhang_support_segments`（0x141248620，6819B，涉及 `Curve2::circular_slice` 的
/// 0x50B 输出记录 `{Curve 0x30B, wall @0x30, (u0,u1) @0x38, flag @0x40}`，需逐条核对 ~700 行反编译与
/// 十余个 rodata 常量）、`OverhangSupportFinderCache::new`（0x141248300，依赖原版 ECS 内部表布局，
/// 仓库 `PublicWallState` 与原 `+0x280` 偏移不对应）、p8/p9/p10/p15 的类型 Rust 化。
///
/// B `detect_overhang_support_segments`（0x1407867d0，4258B）。**未实现**（见上）。
pub fn detect_overhang_support_segments(
    _subscriptions: ResMut<OverhangSupportDirtySubscriptions>,
    _cache: ResMut<
        crate::construct_elevation_supports::find_overlap_segments::OverhangSupportFinderCache,
    >,
) {
    todo!(
        "未实现: detect_overhang_support_segments 0x1407867d0（tabloid 脏订阅→43KB 悬挑段→ElevationSupports 写面）"
    )
}

/// B `construct_circle_supports`（0x140788160，2305B / 544 行 fresh）。
///
/// 已解结构（`work/restorer-remaining-wallconstructor/evidence/
/// elevsys-0x140788160.c`）：
/// - 参数：`EventReader<RecalculateCircleSupports>`（事件 stride 0x28，
///   wall id @+0x18）、`Res<PublicWalls>`、以及 stack 上的
///   `ElevationSupports` 类 map / 输出记录 Vec / 一个 f32 写面；
/// - 逐事件 `PublicWalls` 探测（墙记录 stride 0x2a8）→
///   `PublicWallState::is_circle()` 且字段 == 1 才处理；
/// - `StoneColumnCollisionStorage::clear_and_get_mut()`；
/// - `circle_categorize()`：**false → `construct_turret()`**；
///   true → `construct_circle_bottom_layer()` + 0x28B indexmap bucket
///   `{0,4,0, wall_id, uVar13}`（ElevationSupportStats 记录）+
///   `construct_circle_pillars()`；
/// - 再把本次支柱记录复制/去重进 `ElevationSupports`（0x20B 记录 Vec：
///   复制循环先整块 alloc，再逐 2 条 0x20B 拷贝，旧 Vec dealloc）。
///
/// **未实现（core-type-gap）**：`country_core::resources::walls::
/// elevation_support_stats::ElevationSupports/ElevationSupportStats` 现为
/// `Gap`（布局 U），0x28B stats 记录与 0x20B 支柱记录字段无法逐语句落库；
/// 且被调 `circle_categorize`/`construct_turret`/`construct_circle_bottom_layer`/
/// `construct_circle_pillars` 均未实现。解锁条件：core 落 ElevationSupports
/// 布局 + 四个被调函数重建后按上述全序实现。
pub fn construct_circle_supports() {
    todo!(
        "未实现: construct_circle_supports 0x140788160（ElevationSupports 布局 U + circle_categorize/turret/bottom_layer/pillars 未实现）"
    )
}

/// B `construct_rectangle_columns`（0x140788a80，2674B / 651 行 fresh）。
/// **未实现**：矩形柱构造（4 角 `get_point`/地形门控/逐墙
/// `GladeBorder::is_pos_inside`/3 向 `legacy_raycast_w_params` 净空 ≥2.3/
/// `find_stone_column_start_y`/smoothstep/Rectangle3d 输出/
/// halftimber 二路——即 `construct_rectangle_pillars` 0x14124fcb0 的调用宿主，
/// 该被调函数仍未实现）。
pub fn construct_rectangle_columns() {
    todo!(
        "未实现: construct_rectangle_columns 0x140788a80（宿主链依赖 construct_rectangle_pillars 未实现）"
    )
}

/// B `construct_rectangle_brackets`（0x1407896b0，2626B / 584 行 fresh）。
/// **未实现**：矩形托架构造（`construct_rectangle_small_brackets`
/// 0x141250da0 / `construct_rectangle_large_brackets` 0x141253fc0 调用宿主；
/// 被调两者仍未实现）。
pub fn construct_rectangle_brackets() {
    todo!(
        "未实现: construct_rectangle_brackets 0x1407896b0（宿主链依赖 small/large brackets 未实现）"
    )
}

#[cfg(test)]
mod overhang_subscription_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;

    /// `add_wall_systems` 的 insert_resource 值是 `(0,8,0)` = `Vec::new()`：默认值为空且零容量。
    #[test]
    fn default_subscriptions_is_empty_vec_without_capacity() {
        let subs = OverhangSupportDirtySubscriptions::default();
        assert!(subs.subscribers.is_empty());
        assert_eq!(subs.subscribers.capacity(), 0);
    }

    /// 0x140786440：9 个 RaycastWorld 槽订阅（0,1,4,5,6,7,2,3,0xc）+ 1 个地形发布器订阅 = 10 项，
    /// 每个槽的订阅者只接收该槽发布器的变更。
    #[test]
    fn subscribe_to_tabloids_pushes_ten_subscribers_in_order() {
        let mut world = World::new();
        world.init_resource::<OverhangSupportDirtySubscriptions>();
        world.insert_resource(country_core::systems::collision::world::RaycastWorld::new());
        world.insert_resource(country_core::resources::terrain_height::TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
        world.run_system_once(overhang_subscribe_to_tabloids).unwrap();
        let subs = world.resource::<OverhangSupportDirtySubscriptions>();
        assert_eq!(subs.subscribers.len(), 10);
    }
}

#[cfg(test)]
mod clear_wall_instance_owners_tests {
    use super::*;
    use crate::gpu_output::{ConstructorGpuOutput, ConstructorKey, ConstructorPart};
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    use country_core::geometry::instanced_wall::BrickTransformSsbo;
    use country_core::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;

    struct TestDevice;
    impl GraphicsDevice for TestDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                id: 1,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[std::ops::Range<usize>],
            _offset: usize,
        ) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage("no GPU in fixture"))
        }
    }

    /// 0x140787d80：两个 per-wall 分配（plank、brick）都被清空；缺库时 unwrap panic。
    #[test]
    fn clears_both_plank_and_brick_owner_and_panics_when_unregistered() {
        let mut library = AssetSsboLibrary::new();
        library.add_new::<WoodenPlankSsbo>(&TestDevice);
        library.add_new::<BrickTransformSsbo>(&TestDevice);
        let mut output = ConstructorGpuOutput::default();
        let key = ConstructorKey {
            wall: 7,
            part: ConstructorPart::Arches,
        };
        let pkey = ConstructorKey {
            part: ConstructorPart::Lintels,
            ..key
        };
        {
            let bricks = library.get_mut::<BrickTransformSsbo>().unwrap();
            assert_eq!(output.append_bricks(bricks, key, &[BrickTransformSsbo::default(); 2]), 0);
            assert_eq!(bricks.to_draw_multi_instanced(), vec![(0, 2)]);
        }
        {
            let planks = library.get_mut::<WoodenPlankSsbo>().unwrap();
            assert_eq!(output.append_planks(planks, pkey, &[WoodenPlankSsbo::default(); 3]), 0);
            assert_eq!(planks.to_draw_multi_instanced(), vec![(0, 3)]);
        }
        clear_wall_instance_owners(&mut library, &mut output, 7);
        assert!(library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().is_empty());
        assert!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().is_empty());

        let mut empty = AssetSsboLibrary::new();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            clear_wall_instance_owners(&mut empty, &mut ConstructorGpuOutput::default(), 1)
        }));
        assert!(r.is_err());
    }
}
