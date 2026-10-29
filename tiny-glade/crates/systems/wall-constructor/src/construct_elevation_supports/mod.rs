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

/// 原检测器分别发布矩形与圆形支撑重算事件；均携带墙ID和该墙的完整候选段。
#[derive(Event, Clone, Debug)]
pub struct RecalculateRectangleSupports {
    pub segments: Vec<find_overlap_segments::OverhangSegment>,
    pub wall: country_core::resources::walls::WallId,
}
#[derive(Event, Clone, Debug)]
pub struct RecalculateCircleSupports {
    pub segments: Vec<find_overlap_segments::OverhangSegment>,
    pub wall: country_core::resources::walls::WallId,
}

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
    output.clear_support_planks(planks, wall);
    let bricks = library
        .get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
        .expect("BrickTransformSsbo must be registered (FUN_140787d80 unwrap)");
    output.clear_support_bricks(bricks, wall);
}

/// 原版加载通知触发一次重新检测；读取后在本次检测中复位。
#[derive(Default)]
pub struct DelayOnLoad(pub bool);

/// B 0x1407867d0：会话事件先短路并推迟；随后帧再读取两组墙事件、
/// 合并脏瓦片并按 PublicWalls 原遍历顺序生成圆形或矩形重算事件。
/// 原 p8/p9 为 EventWriter，p10 为 ElevationSupportStats，p15 为木塔事件。
/// 当前仓库的 SSBO 使用显式可变资源与 ConstructorGpuOutput owner，故比原版
/// 多一个安全 owner 参数。三个消费者均已接入宿主，跨会话和失败路径仍需联验。
#[allow(clippy::too_many_arguments)]
pub fn detect_overhang_support_segments(
    mut wall_changed: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    mut bricks_changed: EventReader<country_core::resources::roofs::commands::RecalculateWallBricks>,
    mut subscriptions: ResMut<OverhangSupportDirtySubscriptions>,
    mut library: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    roofs: Query<&country_core::resources::roofs::Roof>,
    intersections: Res<country_core::resources::shape_intersections::ShapeIntersections>,
    mut rectangle_events: EventWriter<RecalculateRectangleSupports>,
    mut circle_events: EventWriter<RecalculateCircleSupports>,
    mut stats: ResMut<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>,
    mut columns: ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,
    mut new_session: EventReader<country_core::resources::session::NewSessionStartedCmd>,
    mut session_loaded: EventReader<country_core::resources::session::SessionLoadedCmd>,
    mut delay: Local<DelayOnLoad>,
    mut turrets: EventWriter<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    use country_core::utils::dirty_tiles::DirtyTiles;
    use std::collections::HashSet;
    // 原两个会话 reader 按顺序短路：命中者推进末尾并延迟一次，
    // 当前帧不读取另一 reader、墙事件或脏瓦片。
    if new_session.read().count() != 0 { delay.0 = true; return; }
    if session_loaded.read().count() != 0 { delay.0 = true; return; }
    let changed: HashSet<_> = wall_changed.read().map(|event| event.0).collect();
    let bricks: HashSet<_> = bricks_changed.read().map(|event| event.0).collect();
    let run = !changed.is_empty() || delay.0 || !bricks.is_empty();
    let mut dirty = DirtyTiles::new_clear();
    let mut got = false;
    for subscriber in &mut subscriptions.subscribers {
        if let Some(publication) = subscriber.take() {
            dirty.extend(&publication);
            got = true;
        }
    }
    delay.0 = false;
    if !run && !got { return; }
    let cache = find_overlap_segments::OverhangSupportFinderCache::new(&walls, &roofs);
    for (id, wall) in walls.all() {
        if wall.authority.elevation() == 0.0 {
            columns.clear_and_get_mut(id);
            stats.walls.insert(id, country_core::resources::walls::elevation_support_stats::ElevationSupports::default());
            clear_wall_instance_owners(&mut library, &mut output.gpu, id.0);
            turrets.send(country_core::resources::walls::plaster::RecalculateWoodenTurretCmd { instances: Vec::new(), wall: id });
            continue;
        }
        let Some(shape) = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape() else { continue };
        let selected = if changed.contains(&id) {
            true
        } else {
            let aabb = match shape { Shape::Circle(shape) => shape.aabb(), Shape::Rectangle(shape) => shape.aabb() };
            let intersects = dirty.intersects_world_rect([aabb.min.x, aabb.min.y, aabb.max.x, aabb.max.y]);
            match shape { Shape::Circle(_) => bricks.contains(&id) || intersects, Shape::Rectangle(_) => intersects }
        };
        if !selected { continue; }
        clear_wall_instance_owners(&mut library, &mut output.gpu, id.0);
        let (trim, minimum) = match shape { Shape::Circle(_) => (false, 0.0), Shape::Rectangle(_) => (true, 0.6) };
        let segments = find_overlap_segments::find_overhang_support_segments(id, wall, &walls, &intersections, &cache, &roofs, trim, minimum);
        match shape {
            Shape::Circle(_) => { circle_events.send(RecalculateCircleSupports { segments, wall: id }); }
            Shape::Rectangle(_) => { rectangle_events.send(RecalculateRectangleSupports { segments, wall: id }); }
        }
    }
}

/// B `construct_circle_supports`（0x140788160）：柱模式先底层、空木塔事件、
/// 柱实例；塔模式由塔构造器发布事件。非空邻墙和真实宿主生命周期仍待联验。
#[allow(clippy::too_many_arguments)]
pub fn construct_circle_supports(
    mut events: EventReader<RecalculateCircleSupports>,
    mut library: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    terrain: Res<country_core::resources::terrain_height::TerrainHeightsData>,
    mut stats: ResMut<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>,
    mut columns: ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,
    mut sounds: EventWriter<country_core::systems::audio::PlayElevationSupportSoundCmd>,
    mut turrets: EventWriter<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    for event in events.read() {
        let wall = walls.get(event.wall).expect("原圆形支撑事件引用的墙必须存在");
        let Some(Shape::Circle(circle)) = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape() else { continue };
        let collision_columns = columns.clear_and_get_mut(event.wall);
        let pillar_mode = circle_categorize::circle_categorize(&circle, &event.segments, &walls);
        let mut count = 0;
        if pillar_mode {
            circle_bottom_layer::construct_circle_bottom_layer_for_wall(event.wall, wall, &mut library, &mut output);
            turrets.write(country_core::resources::walls::plaster::RecalculateWoodenTurretCmd { instances: Vec::new(), wall: event.wall });
            circle_pillars::construct_circle_pillars(event.wall, wall, circle, &event.segments, &walls,
                &raycast, &terrain, &mut library, &mut output, collision_columns, &mut count);
        } else {
            circle_turret::construct_turret(event.wall, circle, wall, &walls, &event.segments, &mut library, &mut output, &mut turrets);
        }
        let current = stats.walls.entry(event.wall).or_default();
        if current.circle_output < count {
            sounds.write(country_core::systems::audio::PlayElevationSupportSoundCmd {
                kind: 2 | u8::from(wall.building_projection().is_halftimbered()),
                position: glam::Vec3::new(circle.center.x, wall.authority.elevation(), circle.center.y),
            });
        }
        current.circle_output = count;
        current.is_turret = !pillar_mode;
        current.columns = collision_columns.clone();
    }
}

/// B `construct_rectangle_columns`（0x140788a80）：按事件更新底层、柱碰撞、
/// 实例和统计。非空相交及完整卸载仍待原侧与真实宿主联验。
#[allow(clippy::too_many_arguments)]
pub fn construct_rectangle_columns(
    mut events: EventReader<RecalculateRectangleSupports>,
    mut library: ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    terrain: Res<country_core::resources::terrain_height::TerrainHeightsData>,
    mut stats: ResMut<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>,
    mut columns: ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,
    mut sounds: EventWriter<country_core::systems::audio::PlayElevationSupportSoundCmd>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    use country_core::resources::walls::wall_stone_columns::StoneColumn;
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    for event in events.read() {
        let wall = walls.get(event.wall).expect("原矩形支撑事件引用的墙必须存在");
        let Some(Shape::Rectangle(rectangle)) = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape() else { continue };
        let collision_columns = columns.clear_and_get_mut(event.wall);
        let wood = wall.building_projection().is_halftimbered();
        rectangle_bottom_layer::construct_rect_bottom_layer(event.wall, wall, rectangle, &mut library, &mut output);
        let proposals = rectangle_pillars::collect_rectangle_pillars(event.wall, wall, rectangle, &event.segments, &walls, &raycast, &terrain);
        collision_columns.extend(proposals.iter().map(|proposal| StoneColumn { bounds: proposal.stone_column_bounds() }));
        // 原 140c96240 对墙 ID 加高程百分数做 u32 饱和和回绕后混合。
        let input = (event.wall.0 as u32).wrapping_add((wall.authority.elevation() * 100.0) as u32) as u64;
        let seed = 0x13c7ca9293765u64.wrapping_add(0x759839da217fu64.wrapping_mul(input)) ^ 0x4d595df4d0f33173;
        let rng = fastrand::Rng::with_seed(seed);
        let mut count = 0;
        if wood {
            util_pillar_construction::construct_wooden_pillar_w_stone_base(event.wall, &proposals, &rng, &mut count, &mut library, &mut output);
        } else {
            util_pillar_construction::construct_brick_columns(event.wall, &proposals, &rng, &mut count, &mut library, &mut output);
        }
        let current = stats.walls.entry(event.wall).or_default();
        if current.circle_output < count {
            sounds.send(country_core::systems::audio::PlayElevationSupportSoundCmd {
                kind: 2 | u8::from(wood),
                position: glam::Vec3::new(rectangle.center.x, wall.authority.elevation(), rectangle.center.y),
            });
        }
        current.circle_output = count;
        current.columns = collision_columns.clone();
    }
}

/// B `construct_rectangle_brackets`（0x1407896b0）：小托架先行；+0x284
/// 达 1.6 才执行大托架，分别比较旧计数并按原顺序发送音效。
/// 已接入 set36；多墙、容量失败和真实宿主重入仍需继续联验。
#[allow(clippy::too_many_arguments)]
pub fn construct_rectangle_brackets(
    mut events:EventReader<RecalculateRectangleSupports>,
    mut library:ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    walls:Res<country_core::resources::walls::public_walls::PublicWalls>,
    small_raycast:Res<country_core::systems::collision::world::RaycastWorld>,
    _signifiers:Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    large_raycast:Res<country_core::systems::collision::world::RaycastWorld>,
    terrain:Res<country_core::resources::terrain_height::TerrainHeightsData>,
    mut stats:ResMut<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>,
    mut sounds:EventWriter<country_core::systems::audio::PlayElevationSupportSoundCmd>,
    mut output:ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    use country_core::systems::wall::valid_enclosure::{Shape,ShapeParameters};
    for event in events.read() {
        let wall=walls.get(event.wall).expect("原矩形托架事件引用的墙必须存在");
        let Some(Shape::Rectangle(rectangle))=ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape() else {continue};
        let mut small=0;let mut large=0;
        rectangle_small_brackets::construct_rectangle_small_brackets(event.wall,wall,rectangle,&walls,&event.segments,
            &small_raycast,&mut library,&mut output,&mut small);
        if 1.6<=wall.building_projection().float_0x284() {
            rectangle_large_brackets::construct_rectangle_large_brackets(event.wall,wall,rectangle,&walls,&event.segments,
                &large_raycast,&terrain,&mut library,&mut output,&mut large);
        }
        let entry=stats.walls.entry(event.wall).or_default();
        let position=glam::Vec3::new(rectangle.center.x,wall.authority.elevation(),rectangle.center.y);
        if entry.small_brackets<small {sounds.write(country_core::systems::audio::PlayElevationSupportSoundCmd {kind:0,position});}
        entry.small_brackets=small;
        if entry.large_brackets<large {sounds.write(country_core::systems::audio::PlayElevationSupportSoundCmd {kind:1,position});}
        entry.large_brackets=large;
    }
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

    /// 原 Reload 保留发布器；Startup 订阅跨世界重建仍接收后续碰撞。
    #[test]
    fn reload_preserves_collision_publisher_subscriptions() {
        use country_core::systems::collision::world::{RaycastWorld,RaycastGeoSource,RaycastMeshData,upsert_collider};
        use country_core::resources::walls::WallId;
        let mut world=World::new();
        world.init_resource::<OverhangSupportDirtySubscriptions>();
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(country_core::resources::terrain_height::TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
        world.run_system_once(overhang_subscribe_to_tabloids).unwrap();
        world.resource_mut::<RaycastWorld>().on_level_reload();
        {
            let mut physics=world.resource_mut::<RaycastWorld>();
            let mut owners=std::collections::HashMap::new();
            upsert_collider(&mut physics,&mut owners,WallId(42),RaycastGeoSource::roof(WallId(42),false,false),
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE,glam::Vec3::ZERO)),glam::Vec3::ZERO,glam::Quat::IDENTITY);
            physics.flush_colliders();
        }
        let mut subscribers=world.resource_mut::<OverhangSupportDirtySubscriptions>();
        assert_eq!(subscribers.subscribers.len(), 10);
        assert!(subscribers.subscribers[1].take().is_some(),"原订阅必须观察重建后的碰撞发布");
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

    impl country_core::render::load_texture::boundary::TextureDevice for TestDevice {
        fn create_texture(&self, _name: &str, _desc: &country_core::rhapsody_types::TextureDesc, _data: &[u8]) -> Result<country_core::rhapsody_types::Texture, BackendError> {
            Err(BackendError::invalid_usage("测试未使用纹理"))
        }
    }

    impl country_core::asset_libraries::texture_library::TextureLibraryDevice for TestDevice {
        fn create_persistent_descriptor_set(&self, _name: &str) -> Result<country_core::rhapsody_contract::DescriptorSet, BackendError> {
            Err(BackendError::invalid_usage("测试未使用纹理"))
        }
        fn update_persistent_descriptor_set(&self, _set: &country_core::rhapsody_contract::DescriptorSet, _updates: &[country_core::asset_libraries::texture_library::DescriptorImageUpdate]) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage("测试未使用纹理"))
        }
        fn get_vk_image_view(&self, _texture: &country_core::rhapsody_types::Texture, _mip: u32) -> Result<country_core::asset_libraries::texture_library::ImageView, BackendError> {
            Err(BackendError::invalid_usage("测试未使用纹理"))
        }
        fn update_texture(&self, _texture: &country_core::rhapsody_types::Texture, _update: &country_core::asset_libraries::texture_library::TextureUpdate) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage("测试未使用纹理"))
        }
    }

    fn detector_fixture(elevations: [f32; 2]) -> (World, country_core::resources::walls::WallId, country_core::resources::walls::WallId, tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>) {
        detector_fixture_with_style(elevations, 0)
    }

    fn detector_fixture_with_style(elevations: [f32; 2], rectangle_style: u8) -> (World, country_core::resources::walls::WallId, country_core::resources::walls::WallId, tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>) {
        detector_fixture_with_styles(elevations, 0, rectangle_style)
    }

    fn detector_fixture_with_styles(elevations: [f32; 2], circle_style: u8, rectangle_style: u8) -> (World, country_core::resources::walls::WallId, country_core::resources::walls::WallId, tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>) {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls, WallState, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512}};
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let circle = inner.new_wall(WallState::new(curve.clone(), 8.0, 0, 0, false));
        let rectangle = inner.new_wall(WallState::new(curve, 8.0, 0, 0, false));
        inner.update_elevation(circle, elevations[0]);
        inner.update_style(circle, circle_style);
        inner.update_elevation(rectangle, elevations[1]);
        inner.update_style(rectangle, rectangle_style);
        inner.update_wall_state_enclosure(circle, ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d { center: glam::Vec2::new(-10.0,0.0), radius: 2.0, rotation: 0.0 })).to_snapshot());
        inner.update_wall_state_enclosure(rectangle, ShapeParameters::from_shape(Shape::Rectangle(utils::geometry::Rectangle2d::new(glam::Vec2::new(10.0,0.0), glam::Vec2::splat(4.0), glam::Vec2::X))).to_snapshot());
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &terrain, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut library = AssetSsboLibrary::new();
        library.add_new::<WoodenPlankSsbo>(&TestDevice);
        library.add_new::<BrickTransformSsbo>(&TestDevice);
        let mut publisher = tabloid::Publisher::new();
        let subscribers = vec![publisher.subscribe()];
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(library);
        world.insert_resource(OverhangSupportDirtySubscriptions { subscribers });
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        world.init_resource::<country_core::resources::shape_intersections::ShapeIntersections>();
        world.init_resource::<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>();
        world.init_resource::<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>();
        world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallChanged>>();
        world.init_resource::<Events<country_core::resources::roofs::commands::RecalculateWallBricks>>();
        world.init_resource::<Events<country_core::resources::session::NewSessionStartedCmd>>();
        world.init_resource::<Events<country_core::resources::session::SessionLoadedCmd>>();
        world.init_resource::<Events<country_core::resources::walls::plaster::RecalculateWoodenTurretCmd>>();
        world.init_resource::<Events<RecalculateCircleSupports>>();
        world.init_resource::<Events<RecalculateRectangleSupports>>();
        (world, circle, rectangle, publisher)
    }

    #[test]
    fn detector_events_and_dirty_subscriptions_recalculate_selected_shapes_once() {
        use country_core::resources::{roofs::commands::RecalculateWallBricks, walls::public_walls::OnWallChanged};
        use country_core::utils::dirty_tiles::DirtyTiles;
        let (mut world, circle, rectangle, mut publisher) = detector_fixture([4.0,4.0]);
        let mut schedule = Schedule::default();
        schedule.add_systems(detect_overhang_support_segments);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(), 1);
        assert_eq!(world.resource::<Events<RecalculateCircleSupports>>().len(), 0);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(), 1);
        world.resource_mut::<Events<RecalculateWallBricks>>().send(RecalculateWallBricks(rectangle));
        world.resource_mut::<Events<RecalculateWallBricks>>().send(RecalculateWallBricks(circle));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(), 1);
        assert_eq!(world.resource::<Events<RecalculateCircleSupports>>().len(), 1);
        let mut dirty = DirtyTiles::new_clear();
        dirty.rasterize_world_rect([8.0,-2.0,12.0,2.0]);
        publisher.publish(dirty).unwrap();
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(), 2);
        assert_eq!(world.resource::<Events<RecalculateCircleSupports>>().len(), 1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(), 2);
    }

    #[test]
    fn detector_session_readers_short_circuit_and_preserve_pending_inputs() {
        use country_core::resources::{session::{NewSessionStartedCmd,SessionLoadedCmd},
            roofs::commands::RecalculateWallBricks,walls::public_walls::OnWallChanged};
        use country_core::utils::dirty_tiles::DirtyTiles;
        let (mut world,circle,rectangle,mut publisher)=detector_fixture([4.0,4.0]);
        let mut schedule=Schedule::default();schedule.add_systems(detect_overhang_support_segments);
        world.resource_mut::<Events<NewSessionStartedCmd>>().send(NewSessionStartedCmd);
        world.resource_mut::<Events<SessionLoadedCmd>>().send(SessionLoadedCmd);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        world.resource_mut::<Events<RecalculateWallBricks>>().send(RecalculateWallBricks(circle));
        let mut dirty=DirtyTiles::new_clear();dirty.rasterize_world_rect([8.0,-2.0,12.0,2.0]);publisher.publish(dirty).unwrap();
        for _ in 0..2 {
            schedule.run(&mut world);
            assert!(world.resource::<Events<RecalculateRectangleSupports>>().is_empty());
            assert!(world.resource::<Events<RecalculateCircleSupports>>().is_empty());
        }
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(),1);
        assert_eq!(world.resource::<Events<RecalculateCircleSupports>>().len(),1);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<RecalculateRectangleSupports>>().len(),1);
        assert_eq!(world.resource::<Events<RecalculateCircleSupports>>().len(),1);

        // 单独的 Local 延迟只允许整体扫描，不选择未变更的非零墙。
        let (mut quiet,_,_,_)=detector_fixture([4.0,4.0]);let mut schedule=Schedule::default();
        schedule.add_systems(detect_overhang_support_segments);
        quiet.resource_mut::<Events<NewSessionStartedCmd>>().send(NewSessionStartedCmd);
        schedule.run(&mut quiet);schedule.run(&mut quiet);
        assert!(quiet.resource::<Events<RecalculateRectangleSupports>>().is_empty());
        assert!(quiet.resource::<Events<RecalculateCircleSupports>>().is_empty());

        let (mut dirty_world,_,_,mut dirty_publisher)=detector_fixture([4.0,4.0]);let mut schedule=Schedule::default();
        schedule.add_systems(detect_overhang_support_segments);
        let mut pending=DirtyTiles::new_clear();pending.rasterize_world_rect([8.0,-2.0,12.0,2.0]);dirty_publisher.publish(pending).unwrap();
        dirty_world.resource_mut::<Events<NewSessionStartedCmd>>().send(NewSessionStartedCmd);
        schedule.run(&mut dirty_world);
        assert!(dirty_world.resource::<Events<RecalculateRectangleSupports>>().is_empty());
        schedule.run(&mut dirty_world);
        assert_eq!(dirty_world.resource::<Events<RecalculateRectangleSupports>>().len(),1);
    }

    /// 原砖柱写入器与正式墙 owner 联用；提案由夹具直接提供，尚不覆盖柱提案生产。
    #[test]
    fn brick_columns_write_formal_owner_and_advance_stats() {
        use country_core::resources::walls::public_walls::OnWallChanged;
        use super::util_pillar_construction::{construct_brick_columns, PillarProposal};
        let (mut world, circle, _, _) = detector_fixture([4.0, 4.0]);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle));
        let mut schedule = Schedule::default();
        schedule.add_systems(detect_overhang_support_segments);
        schedule.run(&mut world);
        let proposal = PillarProposal { center: glam::Vec2::ZERO, index: 0, axis: glam::Vec2::X,
            adjacent_directions: [glam::Vec2::X, glam::Vec2::Y], bottom: 1.0, top: 5.0,
            size: 1.0, adjacent_distances: [1.0, 1.0] };
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
            world.resource_scope(|world, mut stats: Mut<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>| {
                let mut library = world.resource_mut::<AssetSsboLibrary>();
                let count = &mut stats.walls.entry(circle).or_default().circle_output;
                construct_brick_columns(circle, &[proposal], &fastrand::Rng::with_seed(123), count, &mut library, &mut output);
            });
        });
        assert_eq!(world.resource::<country_core::resources::walls::elevation_support_stats::ElevationSupportStats>().walls[&circle].circle_output, 1);
        let count: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, count)| *count).sum();
        assert_eq!(count, 36);
    }

    #[test]
    fn detector_circle_event_builds_bottom_bricks_in_formal_owner() {
        use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
        fn produce(
            mut events: EventReader<RecalculateCircleSupports>, walls: Res<PublicWalls>,
            mut library: ResMut<AssetSsboLibrary>, mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
        ) {
            for event in events.read() {
                let wall = walls.get(event.wall).expect("检测器事件的墙必须存在");
                super::circle_bottom_layer::construct_circle_bottom_layer_for_wall(event.wall, wall, &mut library, &mut output);
            }
        }
        let (mut world, circle, _, _) = detector_fixture([4.0, 4.0]);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle));
        let mut schedule = Schedule::default();
        schedule.add_systems((detect_overhang_support_segments, produce).chain());
        schedule.run(&mut world);
        let count: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, count)| *count).sum();
        assert_eq!(count, 21);
        schedule.run(&mut world);
        let unchanged: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, count)| *count).sum();
        assert_eq!(unchanged, count);
    }

    #[test]
    fn detector_rectangle_columns_update_formal_owners_and_stats() {
        use country_core::resources::{terrain_height::TerrainHeightsData, walls::{public_walls::{OnWallChanged, OnWallDeleted},
            elevation_support_stats::ElevationSupportStats, wall_stone_columns::{StoneColumnCollisionStorage, StoneColumnsChanged}}};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd, collision::world::RaycastWorld,
            ui::UiSignifierStream};
        let (mut world, _, rectangle, _) = detector_fixture([4.0,4.0]);
        let mut terrain = TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new(); terrain.set_height_range(0.0,0.0);
        world.insert_resource(terrain); world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>(); world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        world.init_resource::<Events<OnWallDeleted>>(); world.init_resource::<Events<StoneColumnsChanged>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((detect_overhang_support_segments, construct_rectangle_columns,
            maintain_stone_column_colliders::maintain_stone_column_colliders).chain());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        schedule.run(&mut world);
        let stats = &world.resource::<ElevationSupportStats>().walls[&rectangle];
        assert_eq!(stats.circle_output, 4); assert_eq!(stats.columns.len(), 4);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().columns[&rectangle], stats.columns);
        assert!(world.resource::<StoneColumnCollisionStorage>().colliders.contains_key(&rectangle));
        assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(), 1);
        let before: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, count)| *count).sum();
        assert!(before > 28);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(), 1);
        let after: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, count)| *count).sum();
        assert_eq!(after, before);
        world.resource_mut::<Events<OnWallDeleted>>().send(OnWallDeleted(rectangle));
        schedule.run(&mut world);
        assert!(!world.resource::<StoneColumnCollisionStorage>().colliders.contains_key(&rectangle));
    }

    #[test]
    fn detector_rectangle_wood_columns_write_both_formal_owners() {
        use country_core::resources::{terrain_height::{TerrainHeightsData, TerrainHeightSsbo}, walls::{public_walls::OnWallChanged,
            elevation_support_stats::ElevationSupportStats, wall_stone_columns::StoneColumnCollisionStorage}};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd, collision::world::RaycastWorld, ui::UiSignifierStream};
        let (mut world, _, rectangle, _) = detector_fixture_with_style([4.0,4.0], 2);
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new(); terrain.set_height_range(0.0,0.0);
        world.insert_resource(terrain); world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>(); world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((detect_overhang_support_segments, construct_rectangle_columns).chain());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        schedule.run(&mut world);
        let stats = &world.resource::<ElevationSupportStats>().walls[&rectangle];
        assert_eq!(stats.circle_output,4); assert_eq!(stats.columns.len(),4);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().columns[&rectangle],stats.columns);
        let mut library = world.resource_mut::<AssetSsboLibrary>();
        let planks: u32 = library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        let bricks: u32 = library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum();
        assert_eq!(planks,16); assert_eq!(bricks,32);
        drop(library);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(rectangle));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(),1);
        let mut library = world.resource_mut::<AssetSsboLibrary>();
        assert_eq!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum::<u32>(),planks);
        assert_eq!(library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_, n)| *n).sum::<u32>(),bricks);
    }

    #[test]
    fn rectangle_brackets_event_writes_real_owner_and_updates_stats() {
        use country_core::resources::{roofs::geometry::surface::Curve2,terrain_height::{TerrainHeightsData,TerrainHeightSsbo},
            walls::{InnerWalls,WallState,elevation_support_stats::ElevationSupportStats,
                public_walls::{PublicWalls,WallPublicationDirty,WallSpatialHash},publication_source::TerrainHeightGrid512}};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd,collision::world::RaycastWorld,ui::UiSignifierStream,
            wall::valid_enclosure::{Shape,ShapeParameters}};
        let (mut world,_,_,_)=detector_fixture([4.0,4.0]);
        let base=Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]],false).unwrap();
        let mut inner=InnerWalls::new();let neighbor=inner.new_wall(WallState::new(base.clone(),8.0,0,0,false));
        let target=inner.new_wall(WallState::new(base,8.0,0,0,false));
        inner.update_elevation(neighbor,1.0);inner.update_elevation(target,4.0);
        inner.update_wall_state_enclosure(neighbor,ShapeParameters::from_shape(Shape::Circle(utils::geometry::Circle2d {center:glam::Vec2::ZERO,radius:2.0,rotation:0.0})).to_snapshot());
        inner.update_wall_state_enclosure(target,ShapeParameters::from_shape(Shape::Rectangle(utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X))).to_snapshot());
        let grid=TerrainHeightGrid512 {encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false};
        let mut walls=PublicWalls::default();walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());world.insert_resource(walls);
        let mut terrain=TerrainHeightsData::<TerrainHeightSsbo>::new();terrain.set_height_range(0.0,0.0);
        world.insert_resource(terrain);world.insert_resource(RaycastWorld::new());world.init_resource::<UiSignifierStream>();world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();
        let device=std::sync::Arc::new(TestDevice);
        world.insert_resource(country_core::asset_libraries::runtime::AssetLibraryRuntime::new(device.clone(),device));
        let data:Vec<u32>=include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-circle-fixture.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        let n=data[0] as usize;let points=(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
        let points_u=data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
        let segment=find_overlap_segments::OverhangSegment {curve:Curve2 {points,points_u,length:f32::from_bits(data[1])},wall:neighbor,
            interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(f32::from_bits(1064951713))],flag:false};
        let mut schedule=Schedule::default();schedule.add_systems((construct_rectangle_brackets,
            crate::gpu_output::delete_constructor_output,crate::gpu_output::flush_constructor_output).chain());
        for iteration in 0..2 {
            world.resource_scope(|world,mut output:Mut<crate::construct_arches::WallConstructorOutput>| {
                let mut library=world.resource_mut::<AssetSsboLibrary>();clear_wall_instance_owners(&mut library,&mut output.gpu,target.0);
            });
            world.resource_mut::<Events<RecalculateRectangleSupports>>().send(RecalculateRectangleSupports {wall:target,segments:vec![segment.clone()]});
            schedule.run(&mut world);
            let stats=&world.resource::<ElevationSupportStats>().walls[&target];assert_eq!(stats.small_brackets,27);assert_eq!(stats.large_brackets,0);
            let count:u32=world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
            assert_eq!(count,54);assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(),1,"iteration {iteration}");
        }
        world.resource_mut::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>().send(country_core::resources::walls::public_walls::OnWallDeleted(target));
        // 同一持久 schedule：析构只排入 waste，原 flush 才回收 active chunk。
        schedule.run(&mut world);
        let count:u32=world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
        assert_eq!(count,0);
    }

    #[test]
    fn large_brackets_real_rapier_hit_writes_and_retires_owner() {
        use country_core::resources::{roofs::geometry::surface::Curve2, terrain_height::{TerrainHeightsData,TerrainHeightSsbo},
            walls::{InnerWalls,WallState,public_walls::{PublicWalls,WallPublicationDirty,WallSpatialHash},publication_source::TerrainHeightGrid512}};
        use country_core::systems::{collision::world::{RaycastWorld,RaycastGeoSource,RaycastMeshData,upsert_collider},wall::valid_enclosure::{Shape,ShapeParameters}};
        let (mut world,_,_,_)=detector_fixture([4.0,4.0]);
        let base=Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]],false).unwrap();
        let mut inner=InnerWalls::new();
        let neighbor=inner.new_wall(WallState::new(base.clone(),8.0,0,0,false));
        let target=inner.new_wall(WallState::new(base,8.0,0,0,false));
        inner.update_elevation(neighbor,1.0);inner.update_elevation(target,4.0);
        let other=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(2.0),glam::Vec2::X);
        let rectangle=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        inner.update_wall_state_enclosure(neighbor,ShapeParameters::from_shape(Shape::Rectangle(other)).to_snapshot());
        inner.update_wall_state_enclosure(target,ShapeParameters::from_shape(Shape::Rectangle(rectangle)).to_snapshot());
        let grid=TerrainHeightGrid512 {encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false};
        let mut walls=PublicWalls::default();walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let data:Vec<u32>=include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-rectangle-fixture.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        let n=data[0] as usize;
        let segment=find_overlap_segments::OverhangSegment {curve:Curve2 {points:(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect(),
            points_u:data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect(),length:f32::from_bits(data[1])},wall:neighbor,
            interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(0.25)],flag:true};
        let mut physics=RaycastWorld::new();let mut colliders=std::collections::HashMap::new();
        upsert_collider(&mut physics,&mut colliders,neighbor,RaycastGeoSource::roof(neighbor,false,false),
            std::sync::Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::new(20.0,1.0,20.0),glam::Vec3::ZERO)),glam::Vec3::ZERO,glam::Quat::IDENTITY);
        physics.flush_colliders();
        let mut terrain=TerrainHeightsData::<TerrainHeightSsbo>::new();terrain.set_height_range(0.0,0.0);
        world.resource_scope(|world,mut output:Mut<crate::construct_arches::WallConstructorOutput>| {
            let mut library=world.resource_mut::<AssetSsboLibrary>();
            clear_wall_instance_owners(&mut library,&mut output.gpu,target.0);
            let mut count=0;
            rectangle_large_brackets::construct_rectangle_large_brackets(target,walls.get(target).unwrap(),rectangle,&walls,&[segment],&physics,&terrain,&mut library,&mut output,&mut count);
            assert_eq!(count,7,"真实 Rapier 命中必须生成全部七根托架");
            let active:u32=library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
            assert_eq!(active,7);
        });
        world.init_resource::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>();
        let device=std::sync::Arc::new(TestDevice);
        world.insert_resource(country_core::asset_libraries::runtime::AssetLibraryRuntime::new(device.clone(),device));
        world.resource_mut::<Events<country_core::resources::walls::public_walls::OnWallDeleted>>().send(country_core::resources::walls::public_walls::OnWallDeleted(target));
        let mut schedule=Schedule::default();schedule.add_systems((crate::gpu_output::delete_constructor_output,crate::gpu_output::flush_constructor_output).chain());schedule.run(&mut world);
        let active:u32=world.resource_mut::<AssetSsboLibrary>().get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
        assert_eq!(active,0);
    }

    #[test]
    fn detector_circle_supports_update_formal_owners_and_stats() {
        use country_core::resources::{terrain_height::{TerrainHeightsData, TerrainHeightSsbo}, walls::{public_walls::{OnWallChanged, OnWallDeleted},
            elevation_support_stats::ElevationSupportStats, wall_stone_columns::{StoneColumnCollisionStorage, StoneColumnsChanged}}};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd, collision::world::RaycastWorld, ui::UiSignifierStream};
        let (mut world, circle, _, _) = detector_fixture([4.0,4.0]);
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new(); terrain.set_height_range(0.0,0.0);
        world.insert_resource(terrain); world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>(); world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        world.init_resource::<Events<OnWallDeleted>>(); world.init_resource::<Events<StoneColumnsChanged>>();
        let mut schedule = Schedule::default();
        schedule.add_systems((detect_overhang_support_segments, construct_circle_supports,
            maintain_stone_column_colliders::maintain_stone_column_colliders).chain());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle));
        schedule.run(&mut world);
        let stats = &world.resource::<ElevationSupportStats>().walls[&circle];
        assert_eq!(stats.circle_output,4); assert!(!stats.is_turret); assert_eq!(stats.columns.len(),4);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().columns[&circle],stats.columns);
        assert!(world.resource::<StoneColumnCollisionStorage>().colliders.contains_key(&circle));
        assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(),1);
        let before: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)|*n).sum();
        assert!(before > 21);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle)); schedule.run(&mut world);
        let after: u32 = world.resource_mut::<AssetSsboLibrary>().get_mut::<BrickTransformSsbo>().unwrap()
            .to_draw_multi_instanced().iter().map(|(_, n)|*n).sum();
        assert_eq!(before,after); assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(),1);
        world.resource_mut::<Events<OnWallDeleted>>().send(OnWallDeleted(circle)); schedule.run(&mut world);
        assert!(!world.resource::<StoneColumnCollisionStorage>().colliders.contains_key(&circle));
    }

    #[test]
    fn detector_circle_wood_pillars_update_both_owners_without_duplicates() {
        use country_core::resources::{terrain_height::{TerrainHeightsData,TerrainHeightSsbo},walls::{public_walls::OnWallChanged,
            elevation_support_stats::ElevationSupportStats,plaster::RecalculateWoodenTurretCmd}};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd,collision::world::RaycastWorld,ui::UiSignifierStream};
        let (mut world,circle,_,_)=detector_fixture_with_styles([4.0,4.0],2,0);
        let mut terrain=TerrainHeightsData::<TerrainHeightSsbo>::new(); terrain.set_height_range(0.0,0.0);
        world.insert_resource(terrain);world.insert_resource(RaycastWorld::new());world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        let mut schedule=Schedule::default();schedule.add_systems((detect_overhang_support_segments,construct_circle_supports).chain());
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle));schedule.run(&mut world);
        let stats=&world.resource::<ElevationSupportStats>().walls[&circle];
        assert_eq!(stats.circle_output,4);assert_eq!(stats.columns.len(),4);assert!(!stats.is_turret);
        assert_eq!(world.resource::<Events<RecalculateWoodenTurretCmd>>().len(),1);
        let mut library=world.resource_mut::<AssetSsboLibrary>();
        let planks:u32=library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
        let bricks:u32=library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum();
        assert_eq!(planks,12);assert_eq!(bricks,32);drop(library);
        world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(circle));schedule.run(&mut world);
        assert_eq!(world.resource::<Events<PlayElevationSupportSoundCmd>>().len(),1);
        let mut library=world.resource_mut::<AssetSsboLibrary>();
        assert_eq!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum::<u32>(),planks);
        assert_eq!(library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().iter().map(|(_,n)|*n).sum::<u32>(),bricks);
    }

    #[test]
    fn circle_turret_sends_event_before_missing_brick_owner_panics() {
        use country_core::resources::{roofs::geometry::surface::Curve2, terrain_height::{TerrainHeightsData,TerrainHeightSsbo},
            walls::plaster::RecalculateWoodenTurretCmd};
        use country_core::systems::{audio::PlayElevationSupportSoundCmd, collision::world::RaycastWorld, ui::UiSignifierStream};
        let (mut world, circle, _, _) = detector_fixture([4.0,4.0]);
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new()); world.insert_resource(RaycastWorld::new());
        world.init_resource::<UiSignifierStream>(); world.init_resource::<Events<PlayElevationSupportSoundCmd>>();
        let segment = find_overlap_segments::OverhangSegment { wall: circle,
            curve: Curve2::try_new(vec![[0.0,0.0],[1.0,0.0]],false).unwrap(),
            interval: [utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)], flag: false };
        world.resource_mut::<Events<RecalculateCircleSupports>>().send(RecalculateCircleSupports { wall: circle, segments: vec![segment] });
        let mut schedule = Schedule::default(); schedule.add_systems(construct_circle_supports);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert_eq!(world.resource::<Events<RecalculateWoodenTurretCmd>>().len(),1);
    }

    #[test]
    fn detector_zero_elevation_clears_columns_stats_and_turret() {
        use country_core::resources::{session::NewSessionStartedCmd, walls::{elevation_support_stats::{ElevationSupportStats, ElevationSupports}, wall_stone_columns::StoneColumnCollisionStorage, plaster::RecalculateWoodenTurretCmd}};
        let (mut world, circle, rectangle, _) = detector_fixture([0.0,4.0]);
        world.resource_mut::<ElevationSupportStats>().walls.insert(circle, ElevationSupports { small_brackets: 3, large_brackets: 4, circle_output: 5, is_turret: true, ..Default::default() });
        world.resource_scope(|world, mut output: Mut<crate::construct_arches::WallConstructorOutput>| {
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            let key = ConstructorKey { wall: circle.0, part: ConstructorPart::ElevationSupports };
            output.gpu.append_bricks(library.get_mut::<BrickTransformSsbo>().unwrap(), key, &[BrickTransformSsbo::default()]);
            output.gpu.append_planks(library.get_mut::<WoodenPlankSsbo>().unwrap(), key, &[WoodenPlankSsbo::default()]);
        });
        world.resource_mut::<Events<NewSessionStartedCmd>>().send(NewSessionStartedCmd);
        let mut schedule = Schedule::default();
        schedule.add_systems(detect_overhang_support_segments);
        schedule.run(&mut world);
        assert!(world.resource::<StoneColumnCollisionStorage>().cleared.is_empty());
        schedule.run(&mut world);
        assert_eq!(world.resource::<StoneColumnCollisionStorage>().cleared, vec![circle]);
        {
            let mut library = world.resource_mut::<AssetSsboLibrary>();
            assert!(library.get_mut::<BrickTransformSsbo>().unwrap().to_draw_multi_instanced().is_empty());
            assert!(library.get_mut::<WoodenPlankSsbo>().unwrap().to_draw_multi_instanced().is_empty());
        }
        let state = &world.resource::<ElevationSupportStats>().walls[&circle];
        assert!(state.columns.is_empty());
        assert_eq!((state.small_brackets, state.large_brackets, state.circle_output, state.is_turret), (0,0,0,false));
        let mut events = world.resource_mut::<Events<RecalculateWoodenTurretCmd>>();
        let emitted: Vec<_> = events.drain().collect();
        assert_eq!(emitted.len(),1);
        assert_eq!(emitted[0].wall,circle);
        assert!(emitted[0].instances.is_empty());
        assert!(world.resource::<Events<RecalculateRectangleSupports>>().is_empty());
        assert_ne!(circle, rectangle);
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
            part: ConstructorPart::ElevationSupports,
        };
        let pkey = key;
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
