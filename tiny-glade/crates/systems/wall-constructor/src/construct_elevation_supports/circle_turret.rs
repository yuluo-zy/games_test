//! B 0x14124cc30：圆塔五层，石砖与半木曲面共用候选中心。
use bevy_ecs::event::EventWriter;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo};
use country_core::render::nani::instance_data::CurvedHalfTimberInstanceData;
use country_core::resources::walls::{WallId, plaster::RecalculateWoodenTurretCmd, public_wall_state::PublicWallState, public_walls::PublicWalls};
use glam::{Vec2, Vec3};
use utils::geometry::Circle2d;
use super::find_overlap_segments::OverhangSegment;

/// 原循环挑选离圆心最近的候选；没有候选时仍发送空事件。
fn turret_target(circle: Circle2d, segments: &[OverhangSegment], walls: &PublicWalls) -> Option<Vec2> {
    let outer = circle.radius + 0.056;
    let inner = if outer * 0.4 <= 0.5 { 0.5 } else { outer * 0.4 };
    let mut best = 1000.0f32;
    let mut target = None;
    for segment in segments {
        let other = walls.get(segment.wall).expect("原圆塔引用墙必须存在");
        let point = if segment.flag || super::circle_categorize::shape_contains(other.get_wall_shape(), circle.center) {
            circle.center
        } else {
            let point = Vec2::from_array(segment.curve.pos_at_u(0.5));
            let d = circle.center - point;
            let raw = ((circle.radius - inner) + -0.46) / (d.y * d.y + d.x * d.x).sqrt();
            let t = if 0.0 <= raw { raw } else { 0.0 };
            let t = if t <= 1.0 { t } else { 1.0 };
            point * t + circle.center * (1.0 - t)
        };
        let d = point - circle.center;
        let distance = (d.y * d.y + d.x * d.x).sqrt();
        if distance < best { best = distance; target = Some(point); }
    }
    target
}

/// 原入口在发事件前暂存 0x50B 仿射项；获取锁后才转换为 96B 实例。
#[repr(C, align(16))]
struct TurretBrickGeometry {
    columns: [[f32; 4]; 4],
    seed: u32,
    distance: [f32; 2],
    padding: u32,
}
impl TurretBrickGeometry {
    fn to_brick(&self, id: WallId) -> BrickTransformSsbo {
        BrickTransformSsbo {
            affine_columns: self.columns.map(|v| [v[0],v[1],v[2]]),
            seed:self.seed,
            material:BrickSourceId::from_wall_id(country_core::resources::walls::publication_fields::WallId(id.0)).0,
            wall_distance:self.distance,
            ..Default::default()
        }
    }
}

/// 只负责原几何与随机流，输出仍由正式入口按原顺序提交。
fn turret_geometry(id: WallId, circle: Circle2d, elevation: f32, half: bool, target: Option<Vec2>)
    -> (Vec<TurretBrickGeometry>, Vec<CurvedHalfTimberInstanceData>) {
    let mut bricks = Vec::new();
    let mut curved = Vec::new();
    let Some(target) = target else { return (bricks, curved) };
    let outer = circle.radius + 0.056;
    let inner = if outer * 0.4 <= 0.5 { 0.5 } else { outer * 0.4 };
    let bottom = elevation + -0.27600002;
    let top = bottom + -1.3800001;
    let rng = fastrand::Rng::with_seed(id.0);
    for layer in 0..5usize {
        let t = layer as f32 * 0.25;
        let power = t.powf(1.5);
        let start = if half { outer + -0.1 } else { outer };
        let radius = (1.0 - power) * start + power * inner;
        let center = circle.center * (1.0 - t) + target * t;
        let height = t * top + (1.0 - t) * bottom;
        let ring = Circle2d { center, radius, rotation: circle.rotation };
        if half {
            curved.push(CurvedHalfTimberInstanceData::new_curved_xy(
                Vec3::new(center.x, height, center.y), radius, 0.56, f32::from_bits(0x3ed3f7d0), 0.0, std::f32::consts::TAU));
            continue;
        }
        let curve = ring.as_curve2(0.2);
        let perimeter = ring.perimeter();
        let count = ((perimeter / 0.46).ceil() as usize).wrapping_add(1);
        let mut splits = Vec::new();
        utils::random_splits_into(count, 0.32 / perimeter, &rng, &mut splits);
        let upper = height + f32::from_bits(0x3ed3f7d0);
        for (index, pair) in splits.windows(2).enumerate() {
            let u0 = utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(pair[0]).get();
            let u1 = utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(pair[1]).get();
            let a = curve.get_pos_at_u(u0);
            let b = curve.get_pos_at_u(u1);
            let d = b - a;
            let inv = 1.0 / (d.y * d.y + d.x * d.x).sqrt();
            #[cfg(test)]
            if std::env::var_os("M6_FLOAT_DIAGNOSTICS").is_some()
                && ((layer == 0 && index == 14) || (layer == 1 && index == 29)) {
                let c0 = curve.get_coord_at_u(u0);
                let c1 = curve.get_coord_at_u(u1);
                eprintln!("M6_TURRET layer={} index={} center={:08x},{:08x} radius={:08x} rotation={:08x} perimeter={:08x} count={} u={:08x},{:08x} coord={}, {:08x}; {}, {:08x} a={:08x},{:08x} b={:08x},{:08x} d={:08x},{:08x} inv={:08x}", layer, index, center.x.to_bits(), center.y.to_bits(), radius.to_bits(), circle.rotation.to_bits(), perimeter.to_bits(), curve.points.len(), u0.to_bits(), u1.to_bits(), c0.segment, c0.t.to_bits(), c1.segment, c1.t.to_bits(), a.x.to_bits(), a.y.to_bits(), b.x.to_bits(), b.y.to_bits(), d.x.to_bits(), d.y.to_bits(), inv.to_bits());
                for j in [c0.segment, c0.segment + 1, c1.segment, c1.segment + 1] {
                    let sample_u = j as f32 / (curve.points.len() - 1) as f32;
                    let rotation = ring.rotation % std::f32::consts::TAU;
                    let rotation = if rotation < 0.0 { std::f32::consts::TAU + rotation } else { rotation };
                    let angle = rotation + (1.0 - sample_u) * std::f32::consts::TAU;
                    let p = curve.points[j];
                    eprintln!("M6_TURRET_SAMPLE j={} u={:08x} angle={:08x} cos={:08x} sin={:08x} point={:08x},{:08x} knot={:08x}", j, sample_u.to_bits(), angle.to_bits(), angle.cos().to_bits(), angle.sin().to_bits(), p.x.to_bits(), p.y.to_bits(), curve.points_u[j].to_bits());
                }
            }
            let x = inv * d.x; let z = inv * d.y;
            let width = (u1 - u0) * perimeter + 0.1;
            let h = upper - height;
            let cx = (a.x+b.x)*0.5; let cy = (height+upper)*0.5; let cz = (a.y+b.y)*0.5;
            bricks.push(TurretBrickGeometry {
                columns: [[width*x, width*0.0, width*z, width*z], [h*0.0, h*1.0, h*0.0, h*0.0],
                    [-z*0.56, 0.0*0.56, x*0.56, x*0.56], [cx,cy,cz,cz]],
                seed: ((layer as u32 + 1).wrapping_mul(index as u32 + 1)).wrapping_add(1000),
                distance: [u0*perimeter, u1*perimeter], padding:0,
            });
        }
    }
    (bricks, curved)
}

/// 原事件先写入，石砖非空才查资源、取得同墙 owner 并追加。
pub fn construct_turret(
    id: WallId, circle: Circle2d, wall: &PublicWallState, walls: &PublicWalls,
    segments: &[OverhangSegment], library: &mut AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
    events: &mut EventWriter<RecalculateWoodenTurretCmd>,
) {
    let target = turret_target(circle, segments, walls);
    let (bricks, instances) = turret_geometry(id, circle, wall.authority.elevation(), wall.building_projection().is_halftimbered(), target);
    events.write(RecalculateWoodenTurretCmd { instances, wall: id });
    if !bricks.is_empty() {
        let storage = library.get_mut::<BrickTransformSsbo>().expect("原圆塔要求 BrickTransformSsbo");
        let mut writer = output.gpu.brick_writer(storage, id.0);
        for geometry in &bricks { let _ = writer.push(&geometry.to_brick(id)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(s: &str) -> Vec<u32> { s.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect() }
    #[test]
    fn turret_stone_matches_original_132_records() {
        assert_eq!(std::mem::size_of::<TurretBrickGeometry>(), 80);
        assert_eq!(std::mem::align_of::<TurretBrickGeometry>(), 16);
        let circle = Circle2d { center: Vec2::ZERO, radius: 2.5, rotation: 0.0 };
        let (bricks, curved) = turret_geometry(WallId(123), circle, 4.0, false, Some(Vec2::ZERO));
        assert!(curved.is_empty()); assert_eq!(bricks.len(), 132);
        let actual: Vec<u32> = bricks.iter().flat_map(|b| b.to_brick(WallId(123)).observed_bytes().chunks_exact(4).map(|w| u32::from_le_bytes(w.try_into().unwrap())).collect::<Vec<_>>()).collect();
        let expected = fixture(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-turret-native-words-0.json"));
        assert_eq!(actual, expected);
    }
    #[test]
    fn turret_half_matches_original_five_records() {
        let circle = Circle2d { center: Vec2::ZERO, radius: 2.5, rotation: 0.0 };
        let (bricks, curved) = turret_geometry(WallId(123), circle, 4.0, true, Some(Vec2::ZERO));
        assert!(bricks.is_empty()); assert_eq!(curved.len(), 5);
        let actual: Vec<u32> = curved.iter().flat_map(|p| [p.position.x.to_bits(),p.position.y.to_bits(),p.position.z.to_bits(),p.pad_0c,p.x0.to_bits(),p.x1.to_bits(),p.y0.to_bits(),p.y1.to_bits(),p.w20,p.w24,p.w28.to_bits(),p.w2c,p.w30,p.w34,p.w38,p.w3c]).collect();
        let expected = fixture(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-turret-native-words-1.json"));
        assert_eq!(actual, expected);
    }
    #[test]
    fn turret_offset_neighbor_original_records() {
        let circle = Circle2d { center: Vec2::new(1.0, -2.0), radius: 3.2, rotation: 0.37 };
        let inner = (circle.radius + 0.056) * 0.4;
        let t = ((circle.radius - inner) + -0.46) / 3.0;
        let target = Vec2::new(4.0, -2.0) * t + circle.center * (1.0 - t);
        let (bricks, _) = turret_geometry(WallId(123), circle, 4.0, false, Some(target));
        assert_eq!(bricks.len(), 169);
        let actual: Vec<u32> = bricks.iter().flat_map(|b| b.to_brick(WallId(123)).observed_bytes().chunks_exact(4).map(|w| u32::from_le_bytes(w.try_into().unwrap())).collect::<Vec<_>>()).collect();
        let expected = fixture(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-turret-native-offset-words-0.json"));
        assert_eq!(actual, expected);
        let (_, curved) = turret_geometry(WallId(123), circle, 4.0, true, Some(target));
        let actual: Vec<u32> = curved.iter().flat_map(|p| [p.position.x.to_bits(),p.position.y.to_bits(),p.position.z.to_bits(),p.pad_0c,p.x0.to_bits(),p.x1.to_bits(),p.y0.to_bits(),p.y1.to_bits(),p.w20,p.w24,p.w28.to_bits(),p.w2c,p.w30,p.w34,p.w38,p.w3c]).collect();
        let expected = fixture(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-turret-native-offset-words-1.json"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn target_uses_published_neighbor_and_segment() {
        use country_core::resources::walls::{InnerWalls, WallState, public_walls::{WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512};
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[3.0,-2.0],[5.0,-2.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve.clone(), 8.0, 0, 0, false));
        let shape = utils::geometry::Rectangle2d { axis: Vec2::X, center: Vec2::new(12.0,0.0), size: Vec2::new(4.0,2.0) };
        inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Rectangle(shape)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut segment = OverhangSegment { curve, wall:id, interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)], flag:false };
        let circle = Circle2d { center:Vec2::new(1.0,-2.0), radius:3.2, rotation:0.37 };
        assert_eq!(turret_target(circle, &[], &walls), None);
        let target = turret_target(circle, std::slice::from_ref(&segment), &walls).unwrap();
        assert_eq!((target.x.to_bits(),target.y.to_bits()), (2.4376001358032227f32.to_bits(),(-2.0f32).to_bits()));
        segment.flag = true;
        assert_eq!(turret_target(circle, &[segment], &walls), Some(circle.center));
    }

    #[derive(bevy_ecs::resource::Resource)]
    struct TurretFixture { id: WallId, circle: Circle2d, segments: Vec<OverhangSegment> }
    fn invoke_in_world(
        f: bevy_ecs::system::Res<TurretFixture>, walls: bevy_ecs::system::Res<PublicWalls>,
        mut library: bevy_ecs::system::ResMut<AssetSsboLibrary>,
        mut output: bevy_ecs::system::ResMut<crate::construct_arches::WallConstructorOutput>,
        mut events: EventWriter<RecalculateWoodenTurretCmd>,
    ) {
        construct_turret(f.id, f.circle, walls.get(f.id).unwrap(), &walls, &f.segments, &mut library, &mut output, &mut events);
    }
    fn world_fixture(with_neighbor: bool, style: u8) -> bevy_ecs::world::World {
        use bevy_ecs::event::Events;
        use country_core::resources::walls::{InnerWalls, WallState, public_walls::{WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512};
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve.clone(),8.0,0,0,false));
        inner.update_elevation(id,4.0); inner.update_style(id,style);
        let circle = Circle2d {center:Vec2::ZERO,radius:2.5,rotation:0.0};
        inner.update_wall_state_enclosure(id,ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot());
        let other = inner.new_wall(WallState::new(curve.clone(),8.0,0,0,false));
        let rect = utils::geometry::Rectangle2d { axis:Vec2::X, center:Vec2::ZERO, size:Vec2::new(4.0,2.0) };
        inner.update_wall_state_enclosure(other,ShapeParameters::from_shape(Shape::Rectangle(rect)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let segments = if with_neighbor { vec![OverhangSegment { curve, wall:other, interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)], flag:true }] } else { Vec::new() };
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(walls); world.insert_resource(AssetSsboLibrary::new());
        world.init_resource::<crate::construct_arches::WallConstructorOutput>();
        world.init_resource::<Events<RecalculateWoodenTurretCmd>>();
        world.insert_resource(TurretFixture {id,circle,segments}); world
    }
    #[test]
    fn world_empty_and_half_turret_do_not_require_brick_library() {
        use bevy_ecs::{event::Events,schedule::Schedule};
        for (neighbor,style,expected) in [(false,0,0),(true,2,5)] {
            let mut world = world_fixture(neighbor,style);
            let mut schedule = Schedule::default(); schedule.add_systems(invoke_in_world); schedule.run(&mut world);
            let events = world.resource::<Events<RecalculateWoodenTurretCmd>>();
            let mut reader = events.get_cursor();
            let items: Vec<_> = reader.read(events).collect();
            assert_eq!(items.len(),1); assert_eq!(items[0].instances.len(),expected);
            assert_eq!(items[0].wall,world.resource::<TurretFixture>().id);
        }
    }
    #[test]
    fn world_missing_brick_library_panics_after_event() {
        use bevy_ecs::{event::Events,schedule::Schedule};
        let mut world = world_fixture(true,0);
        let mut schedule = Schedule::default(); schedule.add_systems(invoke_in_world);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world))).is_err());
        let events = world.resource::<Events<RecalculateWoodenTurretCmd>>();
        let mut reader = events.get_cursor(); let items: Vec<_> = reader.read(events).collect();
        assert_eq!(items.len(),1); assert!(items[0].instances.is_empty());
    }

}
