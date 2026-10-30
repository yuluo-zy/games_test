//! 矩形小支架（rectangle_small_brackets.rs）。
//!
//! `construct_rectangle_small_brackets`（0x141250da0，7701B）。
//! 2026-10-17 复核：旧实现（逐点直推 96B）为**近似体**，已移除。
//! 真实逻辑（fresh 已取，1137 行 decomp）含托架几何参数/墙高门控/每角
//! 支架装配，未逐语句锚定。
use glam::Vec3;

/// 原 0x141251403：按邻墙形状选择全局或逐段重采样。
fn resample_support_curve(segment: &super::find_overlap_segments::OverhangSegment, other_is_rectangle: bool) -> utils::curve::Curve2 {
    resample_support_curve_with_spacing(segment,other_is_rectangle,0.6,0.48000002)
}
fn resample_support_curve_with_spacing(segment: &super::find_overlap_segments::OverhangSegment, other_is_rectangle: bool, rect_spacing:f32,circle_spacing:f32) -> utils::curve::Curve2 {
    let curve = utils::curve::Curve2::from_cached_parts(
        segment.curve.points.iter().copied().map(glam::Vec2::from_array).collect(),
        segment.curve.points_u.clone(), segment.curve.length,
    );
    if other_is_rectangle { curve.crashy_resample_per_segment(rect_spacing) }
    else { curve.crashy_resample(circle_spacing) }
}

/// 原先按邻墙去重，再按矩形邻墙四角顺序收集位于目标矩形内的角点。
fn contained_neighbor_corners(
    target: utils::geometry::Rectangle2d,
    segments: &[super::find_overlap_segments::OverhangSegment],
    walls: &country_core::resources::walls::public_walls::PublicWalls,
) -> Vec<(glam::Vec2, f32, glam::Vec2)> {
    use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
    let mut seen = std::collections::HashSet::new();
    let mut corners = Vec::new();
    for segment in segments {
        let wall = walls.get(segment.wall).expect("原小托架邻墙必须存在");
        if !seen.insert(segment.wall) { continue; }
        if let Some(Shape::Rectangle(rectangle)) = ShapeParameters::from_snapshot(wall.get_wall_shape()).get_shape() {
            for index in 0..4 {
                let point = rectangle.get_point(index);
                if target.contains_point(point) {
                    corners.push((point, rectangle.get_point_u(index), rectangle.get_corner_normal(index)));
                }
            }
        }
    }
    corners
}

/// 每个邻墙段先按重采样顺序射线，再处理该邻墙首次出现时的额外角点。
/// 返回原射线输入；碰撞过滤和支撑几何仍由后续阶段处理。
fn segment_support_rays(
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, first_for_wall: bool,
) -> Vec<(Vec3,Vec3)> {
    segment_support_samples(segment,other,target,elevation,first_for_wall).0.into_iter().map(|(origin,direction,_)|(origin,direction)).collect()
}

fn segment_support_samples(
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, first_for_wall: bool,
) -> (Vec<(Vec3,Vec3,f32)>,f32) {
    segment_support_samples_with_spacing(segment,other,target,elevation,first_for_wall,0.6,0.48000002)
}

pub(crate) fn segment_support_samples_with_spacing(
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, first_for_wall: bool,
    rect_spacing:f32,circle_spacing:f32,
) -> (Vec<(Vec3,Vec3,f32)>,f32) {
    let mut rays=Vec::new();
    let length=visit_segment_support_samples(segment,other,target,elevation,first_for_wall,rect_spacing,circle_spacing,
        |origin,direction,u,_|rays.push((origin,direction,u)));
    (rays,length)
}

pub(crate) fn visit_segment_support_samples(
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, first_for_wall: bool,
    rect_spacing:f32,circle_spacing:f32,mut visit:impl FnMut(Vec3,Vec3,f32,f32),
) -> f32 {
    use country_core::systems::wall::valid_enclosure::Shape;
    // 原先收集邻墙角点，随后才重采样和逐点写入。
    let mut corners=Vec::new();
    if first_for_wall {
        if let Shape::Rectangle(rectangle)=other {
            for index in 0..4 {
                let point=rectangle.get_point(index);
                if target.contains_point(point) {corners.push((point,rectangle.get_corner_normal(index),rectangle.get_point_u(index)));}
            }
        }
    }
    let sampled = resample_support_curve_with_spacing(segment, matches!(other, Shape::Rectangle(_)),rect_spacing,circle_spacing);
    let start = segment.interval[0].get(); let end = segment.interval[1].get();
    let forward_end = if start <= end { end } else { end + 1.0 };
    for (&point,&t) in sampled.points.iter().zip(&sampled.points_u) {
        let raw = forward_end * t + (1.0 - t) * start;
        assert!(!raw.is_nan(), "原循环参数不可为 NaN");
        let wrapped = raw % 1.0;
        let u = if wrapped < 0.0 { wrapped + 1.0 } else { wrapped };
        let normal = other.get_normal_at_u(u);
        visit(Vec3::new(point.x,elevation - 0.3,point.y),Vec3::new(normal.x,0.0,normal.y),u,sampled.length);
    }
    for (point,normal,u) in corners {
        visit(Vec3::new(point.x,elevation - 0.3,point.y),Vec3::new(normal.x,0.0,normal.y),u,sampled.length);
    }
    sampled.length
}

/// 原 0x141251baa 用 0.3 最大 TOI、0x0c 掩码；只有未命中才进入支撑生成。
fn support_ray_is_clear(world: &country_core::systems::collision::world::RaycastWorld, origin: Vec3, direction: Vec3) -> bool {
    world.raycast_simple_with_max_toi(origin,direction,0.3,0x0c).handle[0] as u8 == 0x11
}

/// 原 0x1412401c0 返回近、远两个参数；有序 SSE min/max 的 NaN 选参不可改为普通 clamp。
pub(crate) fn original_ray_box(bounds: utils::geometry::aabb::Aabb2, origin: glam::Vec2, direction: glam::Vec2) -> (f32,f32) {
    let far_x=(bounds.max.x-origin.x)/direction.x; let far_y=(bounds.max.y-origin.y)/direction.y;
    let near_x=(bounds.min.x-origin.x)/direction.x; let near_y=(bounds.min.y-origin.y)/direction.y;
    let min = |a:f32,b:f32| if a < b { a } else { b };
    let max = |a:f32,b:f32| if a > b { a } else { b };
    let nx=if far_x.is_nan(){near_x}else{min(near_x,far_x)};
    let ny=if far_y.is_nan(){near_y}else{min(near_y,far_y)};
    let fx=if far_x.is_nan(){near_x}else{max(near_x,far_x)};
    let fy=if far_y.is_nan(){near_y}else{max(near_y,far_y)};
    (if nx.is_nan(){ny}else{max(ny,nx)},if fx.is_nan(){fy}else{min(fy,fx)})
}

/// 原目标矩形局部空间求射线出界距离；保留原 0.2 最小支撑长度。
fn target_exit_distance(target: utils::geometry::Rectangle2d, origin: Vec3, direction: Vec3) -> Option<f32> {
    let inverse = target.orientation_mat2().inverse();
    let local_origin = inverse * (glam::Vec2::new(origin.x,origin.z) - target.center);
    let local_direction = inverse * glam::Vec2::new(direction.x,direction.z);
    let bounds = utils::geometry::aabb::Aabb2::from_center_dims(glam::Vec2::ZERO,target.size);
    let (near,far)=original_ray_box(bounds,local_origin,local_direction);
    if !(0.0 <= far && near <= far) { return None; }
    Some(if 0.2 <= far { far } else { 0.2 })
}

/// 对通过遮挡检测的邻墙采样生成原石材两件或半木一件几何。
/// 遮挡检测由调用者提供，以保持原邻墙集合和 raycast 顺序。
fn segment_support_pieces(
    wall: country_core::resources::walls::WallId,
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, wood: bool, first_for_wall: bool,
    mut clear: impl FnMut(Vec3,Vec3) -> bool,
) -> (Vec<SmallBracketPiece>,u64) {
    let mut pieces=Vec::new();let mut count=0;
    append_segment_support_pieces(wall,segment,other,target,elevation,wood,first_for_wall,clear,&mut pieces,&mut count);
    (pieces,count)
}

#[allow(clippy::too_many_arguments)]
fn append_segment_support_pieces(
    wall: country_core::resources::walls::WallId,
    segment: &super::find_overlap_segments::OverhangSegment,
    other: country_core::systems::wall::valid_enclosure::Shape,
    target: utils::geometry::Rectangle2d, elevation: f32, wood: bool, first_for_wall: bool,
    mut clear: impl FnMut(Vec3,Vec3) -> bool, pieces:&mut Vec<SmallBracketPiece>,count:&mut u64,
) {
    let rng=fastrand::Rng::with_seed(wall.0.wrapping_mul(segment.wall.0));
    let mut index=0usize;
    visit_segment_support_samples(segment,other,target,elevation,first_for_wall,0.6,0.48000002,|origin,direction,u,resampled_length| {
        let current=index;index=index.wrapping_add(1);
        if !clear(origin,direction) { return; }
        let Some(exit)=target_exit_distance(target,origin,direction) else { return };
        let sample=rng.f32();
        let thickness=(sample*0.5-(1.0-sample)*0.5)*0.06+0.6;
        let seed=(wall.0 as u32).wrapping_add(segment.wall.0 as u32).wrapping_add(current as u32);
        let distance=u*resampled_length;
        let rotation=utils::xz_rotation_bad(glam::Vec2::new(direction.x,direction.z));
        if !wood {
            pieces.push(SmallBracketPiece { rotation,
                position:Vec3::new(origin.x+direction.x*0.28,elevation-thickness*0.5,origin.z+direction.z*0.28),
                size:Vec3::new(0.3,thickness,0.3),wall_distance:[distance,distance],seed });
        }
        let length=if exit<=0.7 {exit}else{0.7} - 0.075 + if wood {0.5}else{0.0};
        let shift=length*0.5+0.28+0.15;
        let wood_shift=if wood {length * -0.5}else{0.0};
        pieces.push(SmallBracketPiece { rotation,
            position:Vec3::new(shift*direction.x+origin.x+wood_shift*direction.x,
                elevation-0.15+if wood {-0.15}else{0.0},shift*direction.z+origin.z+wood_shift*direction.z),
            size:Vec3::new(0.3,0.3,length),wall_distance:[distance,distance],seed });
        *count=count.wrapping_add(1);
    });
}

/// 原小托架在实例装配前的有类型几何；不是原 64B 内部结构的 ABI 副本。
#[derive(Clone, Copy, Debug)]
pub(crate) struct SmallBracketPiece {
    pub rotation: glam::Quat,
    pub position: Vec3,
    pub size: Vec3,
    pub wall_distance: [f32; 2],
    pub seed: u32,
}

/// 原 0x141252650..85e 将一条小托架几何转换为墙砖记录。
pub(crate) fn stone_piece(piece: SmallBracketPiece, wall: country_core::resources::walls::WallId)
    -> country_core::geometry::instanced_wall::BrickTransformSsbo {
    use country_core::geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo};
    let q = piece.rotation; let size = piece.size;
    let zz = q.z + q.z; let yy = q.y + q.y; let xx = q.x + q.x;
    BrickTransformSsbo {
        affine_columns: [
            [size.x * (1.0 - (q.y * yy + q.z * zz)), size.x * (yy * q.x + q.w * zz), size.x * (zz * q.x - yy * q.w)],
            [size.y * (yy * q.x - q.w * zz), size.y * (1.0 - (q.z * zz + q.x * xx)), size.y * (xx * q.w + zz * q.y)],
            [size.z * (zz * q.x + yy * q.w), size.z * (zz * q.y - xx * q.w), size.z * (1.0 - (q.x * xx + q.y * yy))],
            [piece.position.x,piece.position.y,piece.position.z],
        ],
        seed: piece.seed.wrapping_add(100),
        material: BrickSourceId::from_wall_id(country_core::resources::walls::publication_fields::WallId(wall.0)).0,
        wall_distance: piece.wall_distance,
        ..Default::default()
    }
}

/// 原半木小托架使用同一矩阵，输出墙木板并保留旗标 1。
pub(crate) fn wood_piece(piece: SmallBracketPiece, wall: country_core::resources::walls::WallId)
    -> country_core::resources::roofs::roof_buffers::WoodenPlankSsbo {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    let columns = stone_piece(piece, wall).affine_columns.map(Vec3::from_array);
    WoodenPlankSsbo { transform: Affine3Packed::from_cols(columns[0],columns[1],columns[2],columns[3]),
        gate_seed: 0, piece_seed: piece.seed.wrapping_add(wall.0 as u32) as i32, wall_id: wall.0 as u32, flags: 1 }
}

#[cfg(test)]
mod original_piece_tests {
    #[test]
    fn circle_segment_generates_original_small_bracket_pieces() {
        use super::*;
        use country_core::systems::wall::valid_enclosure::Shape;
        let parse=|s:&str|->Vec<u32>{s.split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect()};
        let data=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-circle-fixture.json"));let n=data[0] as usize;
        let points=(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
        let points_u=data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
        let segment=super::super::find_overlap_segments::OverhangSegment { curve:country_core::resources::roofs::geometry::surface::Curve2 {points,points_u,length:f32::from_bits(data[1])},
            wall:country_core::resources::walls::WallId(2),interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(f32::from_bits(1064951713))],flag:false };
        let other=Shape::Circle(utils::geometry::Circle2d {center:glam::Vec2::ZERO,radius:2.0,rotation:0.0});
        let target=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        for (wood,fixture) in [(false,include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-intermediate.json")),
            (true,include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-wood-intermediate.json"))] {
            let expected=parse(fixture);let (pieces,count)=segment_support_pieces(country_core::resources::walls::WallId(123),&segment,other,target,4.0,wood,true,|_,_|true);
            assert_eq!(count,27);assert_eq!(pieces.len()*16,expected.len());
            for (i,p) in pieces.iter().enumerate() {
                let actual=[p.rotation.x.to_bits(),p.rotation.y.to_bits(),p.rotation.z.to_bits(),p.rotation.w.to_bits(),p.position.x.to_bits(),p.position.y.to_bits(),p.position.z.to_bits(),p.size.x.to_bits(),p.size.y.to_bits(),p.size.z.to_bits(),0,0,p.wall_distance[0].to_bits(),p.wall_distance[1].to_bits(),p.seed,0];
                for j in [0,1,2,3,4,5,6,7,8,9,12,13,14] {assert_eq!(actual[j],expected[i*16+j],"piece {i} field {j}");}
            }
        }
        let rotated=utils::geometry::Rectangle2d::new(glam::Vec2::new(1.0,-2.0),glam::Vec2::new(5.0,6.0),glam::Vec2::new(0.6,0.8));
        let expected=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rotated-intermediate.json"));
        let (pieces,count)=segment_support_pieces(country_core::resources::walls::WallId(123),&segment,other,rotated,4.0,false,true,|_,_|true);
        assert_eq!(count,13);assert_eq!(pieces.len()*16,expected.len());
        for (i,p) in pieces.iter().enumerate() {
            let actual=[p.rotation.x.to_bits(),p.rotation.y.to_bits(),p.rotation.z.to_bits(),p.rotation.w.to_bits(),p.position.x.to_bits(),p.position.y.to_bits(),p.position.z.to_bits(),p.size.x.to_bits(),p.size.y.to_bits(),p.size.z.to_bits(),0,0,p.wall_distance[0].to_bits(),p.wall_distance[1].to_bits(),p.seed,0];
            for j in [0,1,2,3,4,5,6,7,8,9,12,13,14] {assert_eq!(actual[j],expected[i*16+j],"rotated piece {i} field {j}");}
        }
    }
    #[test]
    fn ray_box_intervals_match_original_sampled_rays() {
        use super::*;
        for fixture in [include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-slabs-native.json"),
            include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rect-slabs-native.json")] {
            let words:Vec<u32>=fixture.split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
            assert_eq!(words.len()%10,0);
            for (i,row) in words.chunks_exact(10).enumerate() {
                let f=|n|f32::from_bits(row[n]);
                let bounds=utils::geometry::aabb::Aabb2 { min:glam::Vec2::new(f(0),f(1)),max:glam::Vec2::new(f(2),f(3)) };
                let (near,far)=original_ray_box(bounds,glam::Vec2::new(f(4),f(5)),glam::Vec2::new(f(6),f(7)));
                assert_eq!([near.to_bits(),far.to_bits()],[row[8],row[9]],"slab {i}");
            }
        }
    }
    #[test]
    fn sampled_world_rays_project_to_original_exit_distances() {
        use super::*;
        let parse=|s:&str|->Vec<u32>{s.split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect()};
        for (rays,slabs) in [
            (include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rays-native.json"),include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-slabs-native.json")),
            (include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rect-rays-native.json"),include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rect-slabs-native.json")),
        ] {
            let rays=parse(rays);let slabs=parse(slabs); assert_eq!(rays.len()/6,slabs.len()/10);
            let target=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
            for (i,(r,s)) in rays.chunks_exact(6).zip(slabs.chunks_exact(10)).enumerate() {
                let f=|n|f32::from_bits(r[n]);let origin=Vec3::new(f(0),f(1),f(2));let direction=Vec3::new(f(3),f(4),f(5));
                let near=f32::from_bits(s[8]);let far=f32::from_bits(s[9]);
                let expected=if 0.0<=far && near<=far {Some(if 0.2<=far {far.to_bits()}else{0.2f32.to_bits()})}else{None};
                assert_eq!(target_exit_distance(target,origin,direction).map(f32::to_bits),expected,"ray {i}");
            }
        }
    }
    #[test]
    fn support_ray_filters_only_hits_within_original_bound() {
        use super::*;
        use country_core::systems::collision::world::{upsert_collider,RaycastGeoSource,RaycastMeshData,RaycastWorld};
        use std::{collections::HashMap,sync::Arc};
        let mut world=RaycastWorld::new();let mut colliders=HashMap::new();
        let source=RaycastGeoSource::Slot6;
        let mesh=Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.1),Vec3::ZERO));
        assert!(support_ray_is_clear(&world,Vec3::ZERO,Vec3::X));
        upsert_collider(&mut world,&mut colliders,1u64,source,mesh.clone(),Vec3::X*0.2,glam::Quat::IDENTITY);
        world.flush_colliders();assert!(!support_ray_is_clear(&world,Vec3::ZERO,Vec3::X));
        upsert_collider(&mut world,&mut colliders,1u64,source,mesh,Vec3::X,glam::Quat::IDENTITY);
        world.flush_colliders();assert!(support_ray_is_clear(&world,Vec3::ZERO,Vec3::X));
    }
    #[test]
    fn circle_samples_produce_original_rays() {
        use super::*;
        use country_core::systems::wall::valid_enclosure::Shape;
        let data: Vec<u32> = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-circle-fixture.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        let n=data[0] as usize;
        let points=(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
        let points_u=data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
        let segment=super::super::find_overlap_segments::OverhangSegment {
            curve:country_core::resources::roofs::geometry::surface::Curve2 { points,points_u,length:f32::from_bits(data[1]) },
            wall:country_core::resources::walls::WallId(2),interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(f32::from_bits(1064951713))],flag:false };
        let circle=utils::geometry::Circle2d { center:glam::Vec2::ZERO,radius:2.0,rotation:0.0 };
        let target=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        let actual=segment_support_rays(&segment,Shape::Circle(circle),target,4.0,true);
        let expected:Vec<u32>=include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rays-native.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        assert_eq!(actual.len(),27); assert_eq!(expected.len(),162);
        for (i,(origin,direction)) in actual.iter().enumerate() {
            assert_eq!([origin.x.to_bits(),origin.y.to_bits(),origin.z.to_bits(),direction.x.to_bits(),direction.y.to_bits(),direction.z.to_bits()],expected[i*6..i*6+6],"ray {i}");
        }
    }
    #[test]
    fn rectangle_samples_and_corners_produce_original_rays() {
        use super::*;
        use country_core::systems::wall::valid_enclosure::Shape;
        let data: Vec<u32> = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-rectangle-fixture.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        let n=data[0] as usize;
        let points=(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
        let points_u=data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
        let segment = super::super::find_overlap_segments::OverhangSegment {
            curve:country_core::resources::roofs::geometry::surface::Curve2 { points,points_u,length:f32::from_bits(data[1]) },
            wall:country_core::resources::walls::WallId(2),interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(0.25)],flag:true };
        let neighbor=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(2.0),glam::Vec2::X);
        let target=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        let actual=segment_support_rays(&segment,Shape::Rectangle(neighbor),target,4.0,true);
        let expected:Vec<u32>=include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rect-rays-native.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        assert_eq!(actual.len(),8); assert_eq!(expected.len(),48);
        for (i,(origin,direction)) in actual.iter().enumerate() {
            assert_eq!([origin.x.to_bits(),origin.y.to_bits(),origin.z.to_bits(),direction.x.to_bits(),direction.y.to_bits(),direction.z.to_bits()],expected[i*6..i*6+6],"ray {i}");
        }
    }
    #[test]
    fn contained_rectangle_corners_match_native_and_deduplicate_wall() {
        use super::*;
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls,WallState,
            public_walls::{PublicWalls,WallPublicationDirty,WallSpatialHash},publication_source::TerrainHeightGrid512}};
        use country_core::systems::wall::valid_enclosure::{Shape,ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0,0.0],[1.0,0.0]],false).unwrap();
        let mut inner = InnerWalls::new(); let id = inner.new_wall(WallState::new(curve.clone(),8.0,0,0,false));
        let neighbor = utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(2.0),glam::Vec2::X);
        inner.update_wall_state_enclosure(id,ShapeParameters::from_shape(Shape::Rectangle(neighbor)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false };
        let mut walls = PublicWalls::default(); walls.consume_with_paths(&mut inner,&grid,None,&WallPublicationDirty::default(),&WallSpatialHash::default());
        let segment = super::super::find_overlap_segments::OverhangSegment { curve,wall:id,
            interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)],flag:true };
        let target = utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        let actual = contained_neighbor_corners(target,&[segment.clone(),segment],&walls);
        let expected: Vec<u32> = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-rect-corners-native.json")
            .split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
        assert_eq!(actual.len(),4); assert_eq!(expected.len(),20);
        for (i,(point,u,normal)) in actual.iter().enumerate() {
            assert_eq!([point.x.to_bits(),point.y.to_bits(),u.to_bits(),normal.x.to_bits(),normal.y.to_bits()],expected[i*5..i*5+5]);
        }
    }
    #[test]
    fn support_curve_resampling_matches_original_circle_and_rectangle() {
        use super::*;
        for (fixture, rect) in [
            (include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-circle-fixture.json"),false),
            (include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-rectangle-fixture.json"),true),
        ] {
            let data: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect();
            let n = data[0] as usize; let length = f32::from_bits(data[1]);
            let points = (0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
            let points_u = data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
            let segment = super::super::find_overlap_segments::OverhangSegment { curve: country_core::resources::roofs::geometry::surface::Curve2 { points,points_u,length },
                wall: country_core::resources::walls::WallId(2), interval: [utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(1.0)], flag:false };
            let actual = resample_support_curve(&segment,rect);
            let offset = 2+3*n; let out_n = data[offset] as usize;
            assert_eq!(actual.points.len(),out_n); assert_eq!(actual.length.to_bits(),data[offset+1]);
            for (i,p) in actual.points.iter().enumerate() { assert_eq!([p.x.to_bits(),p.y.to_bits()],[data[offset+2+i*2],data[offset+3+i*2]],"point {i}"); }
            for (i,u) in actual.points_u.iter().enumerate() { assert_eq!(u.to_bits(),data[offset+2+out_n*2+i],"u {i}"); }
        }
    }
    #[test]
    fn wood_piece_matches_original_27_records() {
        use super::*;
        let parse = |s: &str| -> Vec<u32> { s.split(|c: char| !c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect() };
        let input = parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-wood-intermediate.json"));
        let expected = parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-wood-native-output.json"));
        assert_eq!(input.len(),27*16); assert_eq!(expected.len(),27*16);
        for (i, words) in input.chunks_exact(16).enumerate() {
            let f = |n| f32::from_bits(words[n]);
            let piece = SmallBracketPiece { rotation: glam::Quat::from_xyzw(f(0),f(1),f(2),f(3)),
                position: Vec3::new(f(4),f(5),f(6)),size: Vec3::new(f(7),f(8),f(9)),wall_distance:[f(12),f(13)],seed:words[14] };
            let plank = wood_piece(piece,country_core::resources::walls::WallId(123));
            let mut actual = Vec::new();
            for c in plank.transform.cols { actual.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]); }
            actual.extend([plank.gate_seed as u32,plank.piece_seed as u32,plank.wall_id,plank.flags]);
            for (j,w) in actual.iter().enumerate() { assert_eq!(*w,expected[i*16+j],"piece {i} field {j}"); }
        }
    }
    #[test]
    fn stone_piece_matches_original_54_records() {
        use super::*;
        let parse = |s: &str| -> Vec<u32> { s.split(|c: char| !c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect() };
        let input = parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-intermediate.json"));
        let expected = parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-bracket-native-output.json"));
        assert_eq!(input.len(),54*16); assert_eq!(expected.len(),54*24);
        for (i, words) in input.chunks_exact(16).enumerate() {
            let f = |n| f32::from_bits(words[n]);
            let piece = SmallBracketPiece { rotation: glam::Quat::from_xyzw(f(0),f(1),f(2),f(3)),
                position: Vec3::new(f(4),f(5),f(6)),size: Vec3::new(f(7),f(8),f(9)),wall_distance:[f(12),f(13)],seed:words[14] };
            let bytes = stone_piece(piece,country_core::resources::walls::WallId(123)).observed_bytes();
            for (j,w) in bytes.chunks_exact(4).enumerate() { assert_eq!(u32::from_le_bytes(w.try_into().unwrap()),expected[i*24+j],"piece {i} field {j}"); }
        }
    }
}

/// 原小托架先生成全部几何，随后按材质在同一墙 owner 持锁逐项写入。
/// 多墙遮挡与非有限输入尚待原侧联验，当前入口不得登记生产调度。
#[allow(clippy::too_many_arguments)]
pub fn construct_rectangle_small_brackets(
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: utils::geometry::Rectangle2d,
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    segments: &[super::find_overlap_segments::OverhangSegment],
    raycast: &country_core::systems::collision::world::RaycastWorld,
    library: &mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output: &mut crate::construct_arches::WallConstructorOutput,
    count: &mut u64,
) {
    use country_core::systems::wall::valid_enclosure::ShapeParameters;
    use country_core::{geometry::instanced_wall::BrickTransformSsbo,resources::roofs::roof_buffers::WoodenPlankSsbo};
    let mut seen=std::collections::HashSet::new();
    let mut pieces=Vec::new();
    for segment in segments {
        let other=walls.get(segment.wall).expect("原小托架邻墙必须存在");
        let shape=ShapeParameters::from_snapshot(other.get_wall_shape()).get_shape().expect("原小托架邻墙需要可用形状");
        let first=seen.insert(segment.wall);
        append_segment_support_pieces(id,segment,shape,rectangle,wall.authority.elevation(),
            wall.building_projection().is_halftimbered(),first,|origin,direction| {
                for other_segment in segments {
                    if other_segment.wall==segment.wall { continue; }
                    let other_wall=walls.get(other_segment.wall).expect("原小托架遮挡墙必须存在");
                    if super::circle_categorize::shape_contains(other_wall.get_wall_shape(),glam::Vec2::new(origin.x,origin.z)) { return false; }
                }
                support_ray_is_clear(raycast,origin,direction)
            },&mut pieces,count);
    }
    let key=crate::gpu_output::ConstructorKey {wall:id.0,part:crate::gpu_output::ConstructorPart::ElevationSupports};
    if wall.building_projection().is_halftimbered() {
        let failures={
            let storage=library.get_mut::<WoodenPlankSsbo>().expect("原小托架要求 WoodenPlankSsbo");
            let mut writer=output.gpu.plank_writer(storage,id.0);let mut failures=0;
            for piece in pieces {if writer.push(&wood_piece(piece,id)).is_err(){failures+=1;}} failures
        };
        output.gpu.record_plank_failures(key,failures);
    } else {
        let failures={
            let storage=library.get_mut::<BrickTransformSsbo>().expect("原小托架要求 BrickTransformSsbo");
            let mut writer=output.gpu.brick_writer(storage,id.0);let mut failures=0;
            for piece in pieces {if writer.push(&stone_piece(piece,id)).is_err(){failures+=1;}} failures
        };
        output.gpu.record_brick_failures(key,failures);
    }
}
