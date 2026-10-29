//! B 圆形支撑柱：14124ba30，先收集提案与碰撞，再按墙材质写入。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::{WallId, public_wall_state::PublicWallState, public_walls::PublicWalls, wall_stone_columns::StoneColumn};
use country_core::systems::collision::world::RaycastWorld;
use glam::{Vec2, Vec3};
use utils::geometry::Circle2d;
use super::find_overlap_segments::OverhangSegment;
use super::util_pillar_construction::PillarProposal;

/// 原 14124bb04..bb99：周长 20..50 映射到 4..7 根；MAXSS/MINSS 源操作数决定 NaN。
fn circle_pillar_count(circle: Circle2d) -> usize {
    let t = utils::scalar::inverse_lerp(20.0, 50.0, circle.perimeter());
    let t = if 0.0 <= t { t } else { 0.0 };
    let t = if t <= 1.0 { t } else { 1.0 };
    (t * 7.0 + (1.0 - t) * 4.0).round() as usize
}

/// 依原顺序地形、邻墙、排除自身的下射线及最近邻朝向。
pub(crate) fn collect_circle_pillars(
    id: WallId, wall: &PublicWallState, circle: Circle2d, segments: &[OverhangSegment],
    walls: &PublicWalls, world: &RaycastWorld, terrain: &TerrainHeightsData,
) -> Vec<PillarProposal> {
    if wall.authority.removed_supports() != 0 { return Vec::new(); }
    let elevation = wall.authority.elevation();
    let count = circle_pillar_count(circle);
    let mut proposals = Vec::new();
    for index in 0..count {
        let u = index as f32 / count as f32;
        let point = circle.get_pos_at_u(u);
        if !(terrain.sample_at_world_xz(point.x, point.y) + 0.05 <= elevation) { continue; }
        if segments.iter().any(|segment| {
            let other = walls.get(segment.wall).expect("原圆柱引用墙必须存在");
            super::circle_categorize::shape_contains(other.get_wall_shape(), point)
        }) { continue; }
        let normal = circle.get_normal_at_u(u);
        let origin = Vec3::new(point.x + normal.x * -0.1, elevation + 0.0 * -0.1, point.y + normal.y * -0.1);
        let Some(bottom) = super::rectangle_pillars::find_stone_column_start_y(world, origin, terrain, None, id) else { continue };
        let t = ((elevation - bottom) + -5.0) / 5.0;
        let t = if 0.0 <= t { t } else { 0.0 };
        let t = if t <= 1.0 { t } else { 1.0 };
        let smooth = (t * t) * (3.0 - (t + t));
        let size = smooth * 1.5 + (1.0 - smooth);
        let normal = circle.get_normal_at_u(u);
        let center = Vec2::new(point.x + (size * normal.x) * -0.4, point.y + (size * normal.y) * -0.4);
        let mut nearest = 2.5f32;
        let mut axis = None;
        for segment in segments {
            let other = walls.get(segment.wall).expect("原圆柱引用墙必须存在");
            let shape = other.get_wall_shape();
            if let Some(other_circle) = other.is_circle() {
                let d = other_circle.signed_distance(center);
                let d = if d <= 0.0 { 0.0 } else { d };
                if d < nearest {
                    let v = other_circle.center - center;
                    let inverse = 1.0 / (v.y * v.y + v.x * v.x).sqrt();
                    axis = Some(Vec2::new(inverse * v.x, inverse * v.y)); nearest = d;
                }
            } else {
                let rectangle = other.is_rectangle().expect("原相邻墙必须是圆或矩形");
                let d = rectangle.distance_to_closest_edge(center);
                if d < nearest { axis = Some(rectangle.axis); nearest = d; }
            }
            let _ = shape;
        }
        let axis = axis.unwrap_or_else(|| {
            let v = circle.center - center;
            let inverse = 1.0 / (v.y * v.y + v.x * v.x).sqrt();
            Vec2::new(inverse * v.x, inverse * v.y)
        });
        let next = circle.get_pos_at_u(((index + 1) as f32 / count as f32) % 1.0);
        let previous_index = if index == 0 { count } else { index } - 1;
        let previous = circle.get_pos_at_u((previous_index as f32 / count as f32) % 1.0);
        let delta0 = next - center; let delta1 = previous - center;
        let length0 = (delta0.x * delta0.x + delta0.y * delta0.y).sqrt();
        let length1 = (delta1.x * delta1.x + delta1.y * delta1.y).sqrt();
        proposals.push(PillarProposal { center, index, axis,
            adjacent_directions: [delta0 * (1.0 / length0), delta1 * (1.0 / length1)],
            bottom, top: elevation, size, adjacent_distances: [length0, length1] });
    }
    proposals
}

/// 生成当前墙的碰撞记录与原材质对应的砖柱或木柱，调用方负责事件和旧输出清理。
pub fn construct_circle_pillars(
    id: WallId, wall: &PublicWallState, circle: Circle2d, segments: &[OverhangSegment],
    walls: &PublicWalls, world: &RaycastWorld, terrain: &TerrainHeightsData,
    library: &mut AssetSsboLibrary, output: &mut crate::construct_arches::WallConstructorOutput,
    collisions: &mut Vec<StoneColumn>, count: &mut u64,
) {
    if wall.authority.removed_supports() != 0 { return; }
    let proposals = collect_circle_pillars(id, wall, circle, segments, walls, world, terrain);
    collisions.extend(proposals.iter().map(|p| StoneColumn { bounds: p.stone_column_bounds() }));
    let value = (id.0 as u32).wrapping_add((wall.authority.elevation() * 100.0) as u32) as u64;
    let seed = 0x13c7ca9293765u64.wrapping_add(0x759839da217fu64.wrapping_mul(value)) ^ 0x4d595df4d0f33173;
    let rng = fastrand::Rng::with_seed(seed);
    if wall.building_projection().is_halftimbered() {
        super::util_pillar_construction::construct_wooden_pillar_w_stone_base(id, &proposals, &rng, count, library, output);
    } else {
        super::util_pillar_construction::construct_brick_columns(id, &proposals, &rng, count, library, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls, WallState,
        public_walls::{WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512}};
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

    #[test]
    fn circle_proposals_match_original_empty_neighbours() {
        let circle = Circle2d { center: Vec2::ZERO, radius: 2.5, rotation: 0.0 };
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve, 8.0, 0, 0, false));
        inner.update_elevation(id, 4.0);
        inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0; 512*512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut terrain = TerrainHeightsData::new(); terrain.set_height_range(0.0, 0.0);
        let actual = collect_circle_pillars(id, walls.get(id).unwrap(), circle, &[], &walls, &RaycastWorld::new(), &terrain);
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-output.json");
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        assert_eq!(actual.len() * 16, expected.len());
        for (i, p) in actual.iter().enumerate() {
            let values = [p.center.x.to_bits(), p.center.y.to_bits(), p.index as u32, 0,
                p.axis.x.to_bits(), p.axis.y.to_bits(), p.adjacent_directions[0].x.to_bits(), p.adjacent_directions[0].y.to_bits(),
                p.adjacent_directions[1].x.to_bits(), p.adjacent_directions[1].y.to_bits(), p.bottom.to_bits(), p.top.to_bits(), p.size.to_bits(),
                p.adjacent_distances[0].to_bits(), p.adjacent_distances[1].to_bits()];
            for (j, value) in values.into_iter().enumerate() { assert_eq!(value, expected[i*16+j], "柱 {i} 字段 {j}"); }
        }
    }
    #[test]
    fn circle_proposals_match_original_published_neighbors() {
        for case in 0..3 {
            let (circle, elevation, shape, fixture) = if case == 0 {
                (Circle2d { center:Vec2::ZERO, radius:2.5, rotation:0.0 }, 4.0,
                    Shape::Rectangle(utils::geometry::Rectangle2d { axis:Vec2::X, center:Vec2::new(3.5,0.0), size:Vec2::ONE }),
                    include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-neighbor-words-0.json"))
            } else if case == 2 {
                (Circle2d { center:Vec2::ZERO, radius:2.5, rotation:0.0 }, 4.0,
                    Shape::Rectangle(utils::geometry::Rectangle2d { axis:Vec2::X, center:Vec2::new(2.5,0.0), size:Vec2::ONE }),
                    include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-contained-words.json"))
            } else {
                (Circle2d { center:Vec2::new(3.0,-2.0), radius:4.2, rotation:0.37 }, 8.0,
                    Shape::Circle(Circle2d { center:Vec2::new(4.0,0.0), radius:0.6, rotation:0.0 }),
                    include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-neighbor-words-1.json"))
            };
            let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
            let mut inner = InnerWalls::new();
            let id = inner.new_wall(WallState::new(curve.clone(), 8.0, 0, 0, false));
            inner.update_elevation(id, elevation);
            inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot());
            let other = inner.new_wall(WallState::new(curve.clone(), 8.0, 0, 0, false));
            inner.update_wall_state_enclosure(other, ShapeParameters::from_shape(shape).to_snapshot());
            let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
            let mut walls = PublicWalls::default();
            walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
            let segment = OverhangSegment { curve, wall:other, interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)], flag:false };
            let mut terrain = TerrainHeightsData::new(); terrain.set_height_range(0.0,0.0);
            let actual = collect_circle_pillars(id, walls.get(id).unwrap(), circle, &[segment], &walls, &RaycastWorld::new(), &terrain);
            let expected: Vec<u32> = fixture.split(|c:char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
            assert_eq!(actual.len()*16,expected.len());
            let bounds_fixture = if case == 0 { include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-neighbor-bounds-0.json") } else if case == 1 { include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-neighbor-bounds-1.json") } else { include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-native-contained-bounds.json") };
            let expected_bounds: Vec<u32> = bounds_fixture.split(|c:char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
            assert_eq!(actual.len()*8, expected_bounds.len());
            for (i,p) in actual.iter().enumerate() {
                let b = p.stone_column_bounds();
                let bound_words = [b.footprint.axis.x.to_bits(),b.footprint.axis.y.to_bits(),b.footprint.center.x.to_bits(),b.footprint.center.y.to_bits(),b.footprint.size.x.to_bits(),b.footprint.size.y.to_bits(),b.bottom.to_bits(),b.top.to_bits()];
                assert_eq!(&bound_words, &expected_bounds[i*8..i*8+8], "样本 {case} 碰撞柱 {i}");
                let words = [p.center.x.to_bits(),p.center.y.to_bits(),p.index as u32,0,p.axis.x.to_bits(),p.axis.y.to_bits(),p.adjacent_directions[0].x.to_bits(),p.adjacent_directions[0].y.to_bits(),p.adjacent_directions[1].x.to_bits(),p.adjacent_directions[1].y.to_bits(),p.bottom.to_bits(),p.top.to_bits(),p.size.to_bits(),p.adjacent_distances[0].to_bits(),p.adjacent_distances[1].to_bits()];
                for (j,word) in words.into_iter().enumerate() { assert_eq!(word,expected[i*16+j],"样本 {case} 柱 {i} 字段 {j}"); }
            }
        }
    }

    #[test]
    fn circle_complete_material_chains_match_original_records() {
        let circle = Circle2d { center:Vec2::ZERO, radius:2.5, rotation:0.0 };
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let published_id = inner.new_wall(WallState::new(curve,8.0,0,0,false));
        inner.update_elevation(published_id,4.0);
        inner.update_wall_state_enclosure(published_id,ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let mut terrain = TerrainHeightsData::new(); terrain.set_height_range(0.0,0.0);
        let wall = WallId(123);
        let proposals = collect_circle_pillars(wall,walls.get(published_id).unwrap(),circle,&[],&walls,&RaycastWorld::new(),&terrain);
        let seed = 0x13c7ca9293765u64.wrapping_add(0x759839da217fu64.wrapping_mul(523)) ^ 0x4d595df4d0f33173;
        let mut bricks = Vec::new();
        let rng = fastrand::Rng::with_seed(seed);
        for p in &proposals {
            super::super::util_pillar_construction::construct_crossbrick_pillar(p.size,p.center,p.top,p.bottom,&rng,p.axis,wall,false,|b| -> Result<(),()> {
                bricks.extend(b.observed_bytes().chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap()))); Ok(())
            });
        }
        let parse = |s:&str| s.split(|c:char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let expected = parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-bricks-0.json"));
        assert_eq!(bricks.len(),144*24); assert_eq!(bricks.len(),expected.len());
        for (i,(a,b)) in bricks.iter().zip(&expected).enumerate() { assert_eq!(a,b,"石材砖 {} 字段 {}",i/24,i%24); }
        let mut planks = Vec::new(); let mut wood_bricks = Vec::new(); let mut count = 0;
        let rng = fastrand::Rng::with_seed(seed);
        super::super::util_pillar_construction::construct_wooden_planks(wall,&proposals,&mut count,|p| -> Result<(),()> {
            for c in p.transform.cols { planks.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]); }
            planks.extend([p.gate_seed as u32,p.piece_seed as u32,p.wall_id,p.flags]); Ok(())
        });
        super::super::util_pillar_construction::construct_wooden_stone_bases(wall,&proposals,&rng,|b| -> Result<(),()> {
            wood_bricks.extend(b.observed_bytes().chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap()))); Ok(())
        });
        assert_eq!(count,4);
        for (actual, expected, stride) in [
            (&planks,parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-planks-4.json")),16),
            (&wood_bricks,parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-bricks-4.json")),24),
        ] {
            assert_eq!(actual.len(),expected.len());
            for (i,(a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"半木记录 {} 字段 {} stride {}",i/stride,i%stride,stride); }
        }
    }

    #[test]
    fn circle_offset_rotated_brick_chain_matches_original() {
        let circle = Circle2d { center:Vec2::new(3.0,-2.0), radius:4.2, rotation:0.37 };
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let published_id = inner.new_wall(WallState::new(curve,8.0,0,0,false)); inner.update_elevation(published_id,8.0);
        inner.update_wall_state_enclosure(published_id,ShapeParameters::from_shape(Shape::Circle(circle)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let mut terrain = TerrainHeightsData::new(); terrain.set_height_range(0.0,0.0);
        let wall = WallId(123);
        let proposals = collect_circle_pillars(wall,walls.get(published_id).unwrap(),circle,&[],&walls,&RaycastWorld::new(),&terrain);
        let seed = 0x13c7ca9293765u64.wrapping_add(0x759839da217fu64.wrapping_mul(923)) ^ 0x4d595df4d0f33173;
        let rng = fastrand::Rng::with_seed(seed); let mut bricks = Vec::new();
        for p in &proposals {
            super::super::util_pillar_construction::construct_crossbrick_pillar(p.size,p.center,p.top,p.bottom,&rng,p.axis,wall,false,|b| -> Result<(),()> {
                bricks.extend(b.observed_bytes().chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap()))); Ok(())
            });
        }
        let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-bricks-1.json");
        let expected: Vec<u32> = fixture.split(|c:char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
        assert_eq!(bricks.len(),300*24); assert_eq!(bricks.len(),expected.len());
        for (i,(a,b)) in bricks.iter().zip(&expected).enumerate() { assert_eq!(a,b,"偏移圆柱砖 {} 字段 {}",i/24,i%24); }
        let mut planks = Vec::new(); let mut wood_bricks = Vec::new(); let mut count = 0;
        let rng = fastrand::Rng::with_seed(seed);
        super::super::util_pillar_construction::construct_wooden_planks(wall,&proposals,&mut count,|p| -> Result<(),()> {
            for c in p.transform.cols { planks.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]); }
            planks.extend([p.gate_seed as u32,p.piece_seed as u32,p.wall_id,p.flags]); Ok(())
        });
        super::super::util_pillar_construction::construct_wooden_stone_bases(wall,&proposals,&rng,|b| -> Result<(),()> {
            wood_bricks.extend(b.observed_bytes().chunks_exact(4).map(|w|u32::from_le_bytes(w.try_into().unwrap()))); Ok(())
        });
        assert_eq!(count,5); assert_eq!(planks.len(),15*16); assert_eq!(wood_bricks.len(),60*24);
        for (actual, fixture, stride) in [
            (&planks,include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-rotwood-planks.json"),16),
            (&wood_bricks,include_str!("../../../../../reports/module-recovery-2026-10-23/m06-circle-pillar-full-rotwood-bricks.json"),24),
        ] {
            let expected:Vec<u32> = fixture.split(|c:char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
            assert_eq!(actual.len(),expected.len());
            for (i,(a,b)) in actual.iter().zip(&expected).enumerate() { assert_eq!(a,b,"偏移半木记录 {} 字段 {} stride {}",i/stride,i%stride,stride); }
        }
    }

}
