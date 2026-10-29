//! B decoclutter_show_pins（1412357c0）：真实装饰存储到世界空间锁定图钉。
//! 原事件取最后一项；变换、网格、墙状态、遮挡与实体动画各用既有权威 owner。
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3, Vec3Swizzles};
use utils::{easing_functions::EasingFunction, geometry::{Circle2d, Rectangle2d, Rectangle3d}};
use crate::{
    components::{Ui2dImage, Ui2dWsExtent},
    resources::{
        clutter_meshes::{ClutterMeshes, clutter_mesh_ty_data}, time::GameTime,
        walls::{cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
            decorator::DecoratorInfo, decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            public_walls::PublicWalls},
    },
    systems::{camera_rig::MainCamera, collision::world::RaycastWorld, mode_manager::ClutterMeshTy},
};

/// B 56 字节事件 union 的三个实际业务分支；保留语义，不复制私有 ABI。
/// 原消费者的区间运算是闭相交，圆/矩形的两个标量不归一化或改成中心/高度。
#[derive(Event, Clone, Copy, Debug)]
pub enum ShowPinnedDecoClutter {
    Circle {
        circle: Circle2d,
        vertical: [f32; 2],
        containment: bool,
        exclude_wall: Option<crate::resources::walls::WallId>,
    },
    Rectangle {
        rectangle: Rectangle2d,
        vertical: [f32; 2],
        exclude_wall: Option<crate::resources::walls::WallId>,
    },
    Wall(crate::resources::walls::WallId),
}
impl ShowPinnedDecoClutter {
    /// B tag2，活动墙 ID 在原载荷 +8；自由墙编辑/移动均使用此分支。
    pub fn for_wall(wall: crate::resources::walls::WallId) -> Self {
        Self::Wall(wall)
    }
    pub fn rectangle(
        rectangle: Rectangle2d,
        first: f32,
        second: f32,
        exclude_wall: Option<crate::resources::walls::WallId>,
    ) -> Self {
        Self::Rectangle { rectangle, vertical: [first, second], exclude_wall }
    }
    pub fn circle(
        circle: Circle2d,
        first: f32,
        second: f32,
        containment: bool,
        exclude_wall: Option<crate::resources::walls::WallId>,
    ) -> Self {
        Self::Circle { circle, vertical: [first, second], containment, exclude_wall }
    }
    pub fn for_circle(
        circle: Circle2d,
        elevation: f32,
        max_y: f32,
        exclude_wall: Option<crate::resources::walls::WallId>,
    ) -> Self {
        Self::circle(circle, elevation, max_y, false, exclude_wall)
    }
    /// 原地表画笔0x100分支：区间[0,height]，同时允许中心包含与边缘命中。
    pub fn brush_circle(circle: Circle2d, height: f32) -> Self {
        Self::circle(circle, 0.0, height, true, None)
    }
    /// B ui_move_shapes：两种形状各自写自己的载荷及同一排除墙，不能共用原始偏移。
    pub fn for_shape(
        shape: crate::systems::wall::valid_enclosure::ShapeParameters,
        elevation: f32,
        max_y: f32,
        wall: crate::resources::walls::WallId,
    ) -> Self {
        use crate::systems::wall::valid_enclosure::Shape;
        match shape.get_shape().expect("图钉形状分支需要实际预制形状") {
            Shape::Circle(circle) => Self::circle(circle, elevation, max_y, false, Some(wall)),
            Shape::Rectangle(rectangle) => Self::rectangle(rectangle, elevation, max_y, Some(wall)),
        }
    }
}

/// 原24字节组件：命中键、进度、基础尺寸、出生后时间、淡出判别。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct DecoClutterLock {
    pub id: u32,
    pub progress: f32,
    pub base_extent: Vec2,
    pub age: f32,
    pub hiding: bool,
}
#[derive(Clone, Copy)]
struct PinCandidate { id: u32, size: f32, position: Vec3 }
/// 只复用帧内 scratch 容量，不建立平行权威状态表。
#[derive(Default)]
pub struct PinScratch {
    candidates: Vec<PinCandidate>,
    hits: hashbrown::HashSet<u32>,
    existing: hashbrown::HashSet<u32>,
}
fn interval_intersects(a: [f32; 2], b: [f32; 2]) -> bool {
    // 141762b90两个CMPLEPS；不得归一化或解释成中心/高度。
    a[0] <= b[1] && b[0] <= a[1]
}
fn footprint(bounds: Rectangle3d, transform: CachedDecoratorTransform) -> Rectangle3d {
    let mut rectangle = bounds;
    rectangle.expand(Vec3::splat(0.5));
    let direction = transform.rotation * Vec3::X;
    rectangle.set_orientation_xz(direction.xz().normalize());
    rectangle.translate(transform.translation);
    rectangle
}
fn eligible_type(ty: u8) -> bool {
    let delta = ty.wrapping_sub(0x38);
    delta > 7 || delta == 6
}
fn intersects_request(
    request: ShowPinnedDecoClutter, rectangle: Rectangle3d, info: &DecoratorInfo,
    address: DecoratorStorageAddr, walls: &PublicWalls,
) -> bool {
    let vertical = [rectangle.bottom, rectangle.top];
    match request {
        ShowPinnedDecoClutter::Wall(id) => {
            let Some(wall) = walls.get(id) else { return false; };
            wall.height_outline().iter().any(|point| {
                let range = if wall.authority.freehand_ty() == 0 {
                    [point[1], wall.geometry.metrics.constrained_extra]
                } else { [0.0, wall.max_y()] };
                interval_intersects(vertical, range)
                    && rectangle.footprint.contains_point(Vec2::new(point[0], point[2]))
            })
        }
        ShowPinnedDecoClutter::Rectangle { rectangle: trigger, vertical: range, exclude_wall } => {
            if info.dst_tag() == 2 && exclude_wall == Some(address.as_wall()) { return false; }
            interval_intersects(vertical, range) && trigger.intersects(rectangle.footprint)
        }
        ShowPinnedDecoClutter::Circle { circle, vertical: range, containment, exclude_wall } => {
            if info.dst_tag() == 2 && exclude_wall == Some(address.as_wall()) { return false; }
            if !interval_intersects(vertical, range) { return false; }
            if containment {
                let center = rectangle.footprint.center;
                let delta = circle.center - center;
                if circle.contains_point(center)
                    || (delta.y * delta.y + delta.x * delta.x).sqrt() < circle.radius { return true; }
            }
            rectangle.footprint.intersects_circle(circle)
        }
    }
}
fn pin_size(size: Vec2) -> f32 {
    // minnum保留第一个相等值、NaN取另一项；最终MINSS上限0.5。
    let minimum = if size.x.is_nan() || size.y < size.x { size.y } else { size.x };
    let half = minimum * 0.5;
    if half < 0.5 { half } else { 0.5 }
}
#[allow(clippy::too_many_arguments)]
fn collect_candidates(
    request: ShowPinnedDecoClutter, storage: &DecoratorStorage,
    transforms: &CachedDecoratorTransforms, meshes: &ClutterMeshes, walls: &PublicWalls,
    camera: &MainCamera, collision: &RaycastWorld, scratch: &mut PinScratch,
) {
    let groups = storage.iter_walls().map(|(id, values)| (DecoratorStorageAddr::Wall(id), values))
        .chain(storage.iter_terrain_decorators().map(|values| (DecoratorStorageAddr::Terrain, values)))
        .chain(storage.iter_stair_decorators().map(|values| (DecoratorStorageAddr::Stair, values)));
    for (address, decorators) in groups {
        for (&id, info) in &decorators.decorators {
            if !eligible_type(info.ty) { continue; }
            let transform = transforms.transform(id);
            let ty = ClutterMeshTy::from_native_tag(info.ty).expect("图钉需要有效的原clutter mesh类型");
            let rectangle = footprint(clutter_mesh_ty_data(ty, meshes).mesh.bounds, transform);
            if !intersects_request(request, rectangle, info, address, walls) { continue; }
            let origin = camera.eye_position();
            let delta = transform.translation - origin;
            let reciprocal = 1.0 / (delta.y * delta.y + delta.x * delta.x + delta.z * delta.z).sqrt();
            let direction = delta * reciprocal;
            let maximum = (delta.z * delta.z + (delta.x * delta.x + delta.y * delta.y)).sqrt()
                + f32::from_bits(0xbe4c_cccd);
            // 原mask3、截止装饰前0.2m；只有实际Rapier miss才显示。
            #[cfg(feature = "native-physics")]
            let visible = collision.raycast_simple_with_max_toi(origin, direction, maximum, 3).is_miss();
            #[cfg(not(feature = "native-physics"))]
            let visible: bool = {
                let _ = (collision, direction, maximum);
                panic!("世界图钉遮挡需要native-physics，不能用默认miss代理")
            };
            if visible {
                scratch.hits.insert(id);
                scratch.candidates.push(PinCandidate {
                    id, size: pin_size(rectangle.footprint.size), position: transform.translation,
                });
            }
        }
    }
}
fn animate(pin: &mut DecoClutterLock, extent: &mut Ui2dWsExtent, delta: f32, hit: bool) -> bool {
    pin.age += delta;
    pin.hiding = !hit && pin.age > 0.75;
    let period = if pin.hiding { -0.2 } else { 0.3 };
    pin.progress = delta / period + pin.progress;
    if pin.progress > 0.0 || !pin.hiding {
        // MAXSS(0,value)/MINSS(1,lower)取第二个NaN/等值操作数。
        let lower = if pin.progress < 0.0 { 0.0 } else { pin.progress };
        pin.progress = if lower > 1.0 { 1.0 } else { lower };
        extent.extent = pin.base_extent * EasingFunction::BackOut.apply(pin.progress);
        true
    } else { false }
}
/// 无新事件仍按真实GameTime完成宽限、淡出与实体卸载。
#[allow(clippy::too_many_arguments)]
pub fn decoclutter_show_pins(
    mut requests: EventReader<ShowPinnedDecoClutter>, storage: Res<DecoratorStorage>,
    transforms: Option<Res<CachedDecoratorTransforms>>, meshes: Option<Res<ClutterMeshes>>,
    walls: Option<Res<PublicWalls>>, camera: Option<Res<MainCamera>>,
    collision: Option<Res<RaycastWorld>>, time: Res<GameTime>,
    mut pins: Query<(Entity, &mut DecoClutterLock, &mut Ui2dWsExtent)>,
    mut commands: Commands, mut scratch: Local<PinScratch>,
) {
    scratch.candidates.clear();
    scratch.hits.clear();
    scratch.existing.clear();
    if let Some(request) = requests.read().last().copied() {
        collect_candidates(request, &storage,
            transforms.as_deref().expect("图钉变换owner尚未初始化"),
            meshes.as_deref().expect("图钉网格资产尚未加载"),
            walls.as_deref().expect("图钉墙发布owner尚未初始化"),
            camera.as_deref().expect("图钉相机尚未初始化"),
            collision.as_deref().expect("图钉碰撞世界尚未初始化"), &mut scratch);
    }
    for (entity, mut pin, mut extent) in &mut pins {
        scratch.existing.insert(pin.id);
        let hit = scratch.hits.contains(&pin.id);
        if !animate(&mut pin, &mut extent, time.delta, hit) { commands.entity(entity).despawn(); }
    }
    for candidate in &scratch.candidates {
        if scratch.existing.contains(&candidate.id) { continue; }
        commands.spawn((
            DecoClutterLock { id: candidate.id, base_extent: Vec2::splat(candidate.size), ..Default::default() },
            Ui2dWsExtent { extent: Vec2::ZERO, position: candidate.position },
            Ui2dImage::new("menu/tod/locked"),
        ));
    }
}
/// 原derive入口，tuple另两个组件由真实Bevy bundle注册。
pub fn register_required_components(world: &mut World) { world.register_component::<DecoClutterLock>(); }

const _: () = assert!(std::mem::size_of::<DecoClutterLock>() == 24);
const _: () = assert!(std::mem::offset_of!(DecoClutterLock, hiding) == 20);

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;
    #[test]
    fn actual_event_variants_keep_independent_ranges_and_owner() {
        let wall = crate::resources::walls::WallId(0x123456789);
        assert!(matches!(ShowPinnedDecoClutter::for_wall(wall), ShowPinnedDecoClutter::Wall(id) if id == wall));
        let circle = Circle2d::new(Vec2::new(3.0, 7.0), 2.0);
        assert!(matches!(ShowPinnedDecoClutter::for_circle(circle, 8.0, 2.0, Some(wall)),
            ShowPinnedDecoClutter::Circle { vertical: [8.0, 2.0], containment: false, exclude_wall: Some(id), .. } if id == wall));
        assert!(matches!(ShowPinnedDecoClutter::brush_circle(circle, 4.0),
            ShowPinnedDecoClutter::Circle { vertical: [0.0, 4.0], containment: true, exclude_wall: None, .. }));
        assert!(interval_intersects([0.0, 1.0], [1.0, 2.0]));
        assert!(!interval_intersects([0.0, 1.0], [2.0, 1.0]));
        assert!(!interval_intersects([f32::NAN, 1.0], [0.0, 2.0]));
    }
    #[test]
    fn derived_footprint_uses_actual_rotation_and_position_without_scale() {
        let bounds = Rectangle3d::new(Vec3::ZERO, Vec3::splat(2.0), Vec2::X);
        let rectangle = footprint(bounds, CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            translation: Vec3::new(10.0, 3.0, 20.0), scale: Vec3::splat(50.0),
        });
        assert_eq!(rectangle.footprint.center, Vec2::new(10.0, 20.0));
        assert_eq!(rectangle.footprint.size, Vec2::splat(2.5));
        assert_eq!([rectangle.bottom, rectangle.top], [1.75, 4.25]);
        assert!((rectangle.footprint.axis - Vec2::NEG_Y).length() < 1e-6);
    }
    #[test]
    fn per_entity_grace_does_not_reset_age_and_fade_reaches_removal() {
        let mut pin = DecoClutterLock { base_extent: Vec2::splat(0.5), ..Default::default() };
        let mut extent = Ui2dWsExtent { position: Vec3::splat(11.0), ..Default::default() };
        assert!(animate(&mut pin, &mut extent, 0.3, true));
        assert_eq!(pin.progress, 1.0);
        assert_eq!(extent.extent, Vec2::splat(0.5));
        assert!(animate(&mut pin, &mut extent, 0.3, false));
        assert!(!pin.hiding);
        assert!(animate(&mut pin, &mut extent, 0.2, true));
        assert!(pin.age > 0.75);
        assert!(animate(&mut pin, &mut extent, 0.1, false));
        assert!(pin.hiding);
        assert_eq!(pin.progress, 0.5);
        assert!(!animate(&mut pin, &mut extent, 0.1, false));
        assert_eq!(extent.position, Vec3::splat(11.0));
    }

    #[cfg(feature = "native-physics")]
    fn fixture() -> (World, Schedule) {
        use crate::resources::clutter_meshes::ClutterMesh;
        let mut world = World::new();
        super::super::registration::register_ui_clutter_resources(&mut world);
        world.insert_resource(GameTime { delta: 0.3, ..Default::default() });
        let fixed = std::array::from_fn(|_| {
            let mut mesh = ClutterMesh::omitted_in_demo("pin_test", 0, None, false, false);
            mesh.bounds = Rectangle3d::new(Vec3::ZERO, Vec3::splat(2.0), Vec2::X);
            mesh
        });
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<PublicWalls>();
        let mut camera = MainCamera::default();
        camera.write_matrix(crate::systems::camera_rig::main_camera::INVERSE_VIEW_OFFSET,
            &glam::Mat4::from_translation(Vec3::new(0.0, 0.0, -5.0)));
        world.insert_resource(camera);
        world.insert_resource(RaycastWorld::new());
        let mut schedule = Schedule::default();
        schedule.add_systems(decoclutter_show_pins);
        (world, schedule)
    }
    #[cfg(feature = "native-physics")]
    fn put(world: &mut World, id: u32, address: DecoratorStorageAddr, position: Vec3, ty: u8) {
        use crate::resources::walls::decorator::DecoratorId;
        let mut storage = world.resource_mut::<DecoratorStorage>();
        storage.get_or_insert_default(address).insert(DecoratorId(id), DecoratorInfo { ty, ..Default::default() });
        storage.decorator_to_storage.insert(DecoratorId(id), address);
        world.resource_mut::<CachedDecoratorTransforms>().insert(id,
            CachedDecoratorTransform { rotation: Quat::IDENTITY, translation: position, scale: Vec3::ONE });
    }
    #[cfg(feature = "native-physics")]
    fn show(world: &mut World) {
        world.send_event(ShowPinnedDecoClutter::brush_circle(Circle2d::new(Vec2::ZERO, 10.0), 4.0));
    }
    #[cfg(feature = "native-physics")]
    #[test]
    fn real_storage_last_event_render_components_and_unload_share_one_entity() {
        let (mut world, mut schedule) = fixture();
        put(&mut world, 10, DecoratorStorageAddr::Wall(crate::resources::walls::WallId(4)), Vec3::ZERO, 0);
        put(&mut world, 20, DecoratorStorageAddr::Terrain, Vec3::new(2.0, 0.0, 0.0), 1);
        put(&mut world, 30, DecoratorStorageAddr::Stair, Vec3::new(-2.0, 0.0, 0.0), 2);
        put(&mut world, 40, DecoratorStorageAddr::Terrain, Vec3::ZERO, 0x38);
        world.send_event(ShowPinnedDecoClutter::brush_circle(Circle2d::new(Vec2::splat(100.0), 1.0), 4.0));
        show(&mut world);
        schedule.run(&mut world);
        let mut query = world.query::<(Entity, &DecoClutterLock, &Ui2dWsExtent, &Ui2dImage)>();
        let mut spawned = query.iter(&world).map(|(entity, pin, extent, image)| {
            assert_eq!(extent.extent, Vec2::ZERO);
            assert_eq!(image.name, "menu/tod/locked");
            assert_eq!(image.layer, 0);
            (entity, pin.id)
        }).collect::<Vec<_>>();
        spawned.sort_by_key(|(_, id)| *id);
        assert_eq!(spawned.iter().map(|(_, id)| *id).collect::<Vec<_>>(), [10, 20, 30]);
        for _ in 0..3 { show(&mut world); schedule.run(&mut world); }
        let mut actual = query.iter(&world).map(|(entity, pin, _, _)| (entity, pin.id)).collect::<Vec<_>>();
        actual.sort_by_key(|(_, id)| *id);
        assert_eq!(actual, spawned);
        // 删除权威装饰和资产后，无新事件仍必须自行淡出并卸载实体。
        *world.resource_mut::<DecoratorStorage>() = DecoratorStorage::default();
        world.remove_resource::<ClutterMeshes>();
        world.resource_mut::<GameTime>().delta = 0.1;
        schedule.run(&mut world);
        assert_eq!(query.iter(&world).count(), 3);
        schedule.run(&mut world);
        assert_eq!(query.iter(&world).count(), 0);
    }
    #[cfg(feature = "native-physics")]
    #[test]
    fn actual_rapier_occluder_blocks_and_near_target_cutoff_preserves_visibility() {
        use crate::systems::collision::world::{RaycastGeoSource, RaycastMeshData, upsert_collider, remove_collider};
        let (mut world, mut schedule) = fixture();
        put(&mut world, 10, DecoratorStorageAddr::Terrain, Vec3::ZERO, 0);
        let mut collision = world.remove_resource::<RaycastWorld>().unwrap();
        let mut map = std::collections::HashMap::new();
        upsert_collider(&mut collision, &mut map, 1_u32, RaycastGeoSource::Slot0,
            std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.5), Vec3::ZERO)),
            Vec3::new(0.0, 0.0, -2.0), Quat::IDENTITY);
        collision.flush_colliders();
        world.insert_resource(collision);
        show(&mut world); schedule.run(&mut world);
        assert_eq!(world.query::<&DecoClutterLock>().iter(&world).count(), 0);
        let mut collision = world.remove_resource::<RaycastWorld>().unwrap();
        remove_collider(&mut collision, &mut map, &1).unwrap();
        upsert_collider(&mut collision, &mut map, 2_u32, RaycastGeoSource::Slot0,
            std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.02), Vec3::ZERO)),
            Vec3::new(0.0, 0.0, -0.1), Quat::IDENTITY);
        collision.flush_colliders();
        world.insert_resource(collision);
        show(&mut world); schedule.run(&mut world);
        assert_eq!(world.query::<&DecoClutterLock>().iter(&world).count(), 1);
    }
    #[cfg(feature = "native-physics")]
    #[test]
    fn actual_published_wall_points_drive_global_request_without_a_shadow_geometry_map() {
        use crate::resources::walls::{InnerWalls, WallState,
            public_walls::WallPublicationDirty, publication_source::TerrainHeightGrid512};
        use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
        use crate::resources::roofs::geometry::surface::Curve2;
        let (mut world, mut schedule) = fixture();
        let source = Curve2::try_new(vec![[-1.0, 0.0], [1.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(source, 4.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 { encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        assert!(world.resource::<PublicWalls>().get(id).is_some());
        put(&mut world, 10, DecoratorStorageAddr::Terrain, Vec3::ZERO, 0);
        put(&mut world, 11, DecoratorStorageAddr::Terrain, Vec3::new(20.0, 0.0, 0.0), 0);
        put(&mut world, 12, DecoratorStorageAddr::Terrain, Vec3::new(0.0, 10.0, 0.0), 0);
        world.send_event(ShowPinnedDecoClutter::for_wall(id));
        schedule.run(&mut world);
        assert_eq!(world.query::<&DecoClutterLock>().iter(&world).map(|pin| pin.id).collect::<Vec<_>>(), [10]);
    }
    #[cfg(feature = "native-physics")]
    #[test]
    fn shape_requests_apply_native_owner_exclusion_and_vertical_ranges_before_occlusion() {
        use crate::resources::walls::{WallId, decorator::DecoratorId};
        let (mut world, mut schedule) = fixture();
        let wall = WallId(4);
        put(&mut world, 10, DecoratorStorageAddr::Wall(wall), Vec3::ZERO, 0);
        put(&mut world, 20, DecoratorStorageAddr::Terrain, Vec3::ZERO, 0);
        world.resource_mut::<DecoratorStorage>().decorator_info_mut(DecoratorId(10)).unwrap().u20 = 2 << 16;
        // 原边相交函数在完全重合的轴对齐边上会遇到0/0并返回false（原fixture0）。
        // 用真实非平行相交验证排除owner，不能以SAT补丁改变原几何行为。
        let rectangle = Rectangle2d::new(Vec2::ZERO, Vec2::splat(2.5), Vec2::new(0.6, 0.8));
        world.send_event(ShowPinnedDecoClutter::rectangle(rectangle, -1.0, 1.0, Some(wall)));
        schedule.run(&mut world);
        assert_eq!(world.query::<&DecoClutterLock>().iter(&world).map(|pin| pin.id).collect::<Vec<_>>(), [20]);
        for _ in 0..3 {
            world.send_event(ShowPinnedDecoClutter::for_circle(Circle2d::new(Vec2::ZERO, 1.5), -1.0, 1.0, Some(wall)));
            schedule.run(&mut world);
        }
        world.send_event(ShowPinnedDecoClutter::rectangle(rectangle, 2.0, 3.0, Some(wall)));
        world.resource_mut::<GameTime>().delta = 0.1;
        schedule.run(&mut world);
        let mut pins = world.query::<&DecoClutterLock>();
        let pin = pins.single(&world).unwrap();
        assert_eq!(pin.id, 20);
        assert!(pin.hiding);
        assert_eq!(pin.progress, 0.5);
    }
}
