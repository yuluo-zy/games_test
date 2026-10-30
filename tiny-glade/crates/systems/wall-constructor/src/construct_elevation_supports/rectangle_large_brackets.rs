//! 矩形大支架（construct_elevation_supports/rectangle_large_brackets.rs）。
//!
//! `construct_rectangle_large_brackets`（0x141253fc0）。
//! 2026-10-17 复核：旧实现（相邻点中点直推 64B）为**近似体**，已移除。
//! 已恢复采样、角点、碰撞判定及木板输出；多墙与容量失败仍待联验。
use glam::Vec3;

/// 大托架采样先要求目标矩形边界距点严格超过 1.3，随后从上端向地面射线。
fn segment_large_rays(
    segment:&super::find_overlap_segments::OverhangSegment,
    other:country_core::systems::wall::valid_enclosure::Shape,
    target:utils::geometry::Rectangle2d,elevation:f32,other_elevation:f32,first_for_wall:bool,
    point_allowed:impl FnMut(Vec3)->bool,
) -> Vec<(Vec3,Vec3,Vec3,usize)> {
    let mut rays=Vec::new();
    visit_large_rays(segment,other,target,elevation,other_elevation,first_for_wall,point_allowed,|a,b,c,i|rays.push((a,b,c,i)));
    rays
}

fn visit_large_rays(
    segment:&super::find_overlap_segments::OverhangSegment,
    other:country_core::systems::wall::valid_enclosure::Shape,
    target:utils::geometry::Rectangle2d,elevation:f32,other_elevation:f32,first_for_wall:bool,
    mut point_allowed:impl FnMut(Vec3)->bool,mut visit:impl FnMut(Vec3,Vec3,Vec3,usize),
) {
    use super::rectangle_small_brackets::{original_ray_box,visit_segment_support_samples};
    let inverse=target.orientation_mat2().inverse();
    let bounds=utils::geometry::aabb::Aabb2::from_center_dims(glam::Vec2::ZERO,target.size);
    let mut index=0usize;
    visit_segment_support_samples(segment,other,target,elevation,first_for_wall,0.90000004,0.51000005,|origin,normal,_,_| {
        let current=index;index=index.wrapping_add(1);
        if !point_allowed(origin) {return;}
        let p=glam::Vec2::new(origin.x,origin.z);let n=glam::Vec2::new(normal.x,normal.z);
        let (enter,exit)=original_ray_box(bounds,inverse*(p-target.center),inverse*n);
        if !(0.0<=exit && enter<=exit) {return;}
        let distance=if enter<0.0 {exit}else{enter};
        if !(1.3<distance) {return;}
        let lower_y=if elevation-1.2 <= other_elevation+0.28 {other_elevation+0.28}else{elevation-1.2};
        let lower=Vec3::new(p.x,lower_y,p.y);
        let upper=Vec3::new(p.x+n.x*1.3,elevation,p.y+n.y*1.3);
        let delta=Vec3::new(lower.x-upper.x,(lower.y-0.2)-upper.y,lower.z-upper.z);
        let reciprocal=1.0/(delta.y*delta.y+delta.x*delta.x+delta.z*delta.z).sqrt();
        visit(upper,Vec3::new(delta.x*reciprocal,delta.y*reciprocal,delta.z*reciprocal),lower,current);
    });
}

/// 原大托架由两端点构造 0.28 截面、两端合计延长 0.5 的木板。
pub(crate) fn large_bracket_piece(begin: Vec3, end: Vec3, wall: country_core::resources::walls::WallId, seed: u32)
    -> country_core::resources::roofs::roof_buffers::WoodenPlankSsbo {
    use country_core::resources::{render::Affine3Packed,roofs::roof_buffers::WoodenPlankSsbo};
    let d=end-begin;let length=(d.y*d.y+d.x*d.x+d.z*d.z).sqrt();let inverse=1.0/length;
    let q=utils::rotation_from_normal(Vec3::new(d.x*inverse,d.y*inverse,d.z*inverse));
    let xx=q.x+q.x;let yy=q.y+q.y;let zz=q.z+q.z;let long=length+0.5;
    let columns=[
        Vec3::new(0.28*(1.0-(q.y*yy+q.z*zz)),0.28*(yy*q.x+q.w*zz),0.28*(zz*q.x-yy*q.w)),
        Vec3::new(0.28*(yy*q.x-q.w*zz),0.28*(1.0-(q.z*zz+q.x*xx)),0.28*(xx*q.w+zz*q.y)),
        Vec3::new(long*(zz*q.x+yy*q.w),long*(zz*q.y-xx*q.w),long*(1.0-(q.x*xx+q.y*yy))),
    ];
    WoodenPlankSsbo {transform:Affine3Packed::from_cols(columns[0],columns[1],columns[2],
        Vec3::new((begin.x+end.x)*0.5,(begin.y+end.y)*0.5,(begin.z+end.z)*0.5)),
        gate_seed:0,piece_seed:seed as i32,wall_id:wall.0 as u32,flags:0}
}

#[cfg(test)]
mod original_piece_tests {
    #[test]
    fn rectangle_neighbor_large_rays_match_native() {
        use super::*; use country_core::systems::wall::valid_enclosure::Shape;
        let parse=|s:&str|->Vec<u32>{s.split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect()};
        let data=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-small-resample-rectangle-fixture.json"));let n=data[0] as usize;
        let points=(0..n).map(|i|[f32::from_bits(data[2+i*2]),f32::from_bits(data[3+i*2])]).collect();
        let points_u=data[2+2*n..2+3*n].iter().copied().map(f32::from_bits).collect();
        let segment=super::super::find_overlap_segments::OverhangSegment {curve:country_core::resources::roofs::geometry::surface::Curve2 {points,points_u,length:f32::from_bits(data[1])},wall:country_core::resources::walls::WallId(2),interval:[utils::curve::CurveU::from_raw(0.0),utils::curve::CurveU::from_raw(0.25)],flag:true};
        let other=Shape::Rectangle(utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(2.0),glam::Vec2::X));
        let target=utils::geometry::Rectangle2d::new(glam::Vec2::ZERO,glam::Vec2::splat(5.0),glam::Vec2::X);
        let rays=segment_large_rays(&segment,other,target,4.0,1.0,true,|_|true);
        let expected=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-large-bracket-hit-rays-native.json"));
        let endpoints=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-large-bracket-hit-endpoints-native.json"));
        assert_eq!(rays.len(),7);assert_eq!(expected.len(),42);assert_eq!(endpoints.len(),42);
        for (i,(origin,direction,lower,_)) in rays.iter().enumerate(){
            assert_eq!([origin.x.to_bits(),origin.y.to_bits(),origin.z.to_bits(),direction.x.to_bits(),direction.y.to_bits(),direction.z.to_bits()],expected[i*6..i*6+6],"ray {i}");
            assert_eq!([lower.x.to_bits(),lower.y.to_bits(),lower.z.to_bits(),origin.x.to_bits(),origin.y.to_bits(),origin.z.to_bits()],endpoints[i*6..i*6+6],"ends {i}");
        }
    }
    #[test]
    fn large_bracket_pieces_match_native_hit_sample() {
        use super::*;
        let parse=|s:&str|->Vec<u32>{s.split(|c:char|!c.is_ascii_digit()).filter(|v|!v.is_empty()).map(|v|v.parse().unwrap()).collect()};
        let endpoints=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-large-bracket-hit-endpoints-native.json"));
        let expected=parse(include_str!("../../../../../reports/module-recovery-2026-10-23/m06-large-bracket-hit-native-output.json"));
        assert_eq!(endpoints.len(),7*6);assert_eq!(expected.len(),7*16);
        for (i,row) in endpoints.chunks_exact(6).enumerate() {
            let f=|n|f32::from_bits(row[n]);let piece=large_bracket_piece(Vec3::new(f(0),f(1),f(2)),Vec3::new(f(3),f(4),f(5)),country_core::resources::walls::WallId(123),125+i as u32);
            let mut actual=Vec::new();for c in piece.transform.cols {actual.extend([c.x.to_bits(),c.y.to_bits(),c.z.to_bits()]);}
            actual.extend([piece.gate_seed as u32,piece.piece_seed as u32,piece.wall_id,piece.flags]);
            for (j,v) in actual.iter().enumerate(){assert_eq!(*v,expected[i*16+j],"piece {i} field {j}");}
        }
    }
}

/// 原大托架持墙木板锁，逐候选判定命中和地形后立即尝试写入并计数。
/// 已由 set36 入口调用；非空多墙和容量失败仍须完整联验。
#[allow(clippy::too_many_arguments)]
pub fn construct_rectangle_large_brackets(
    id:country_core::resources::walls::WallId,
    wall:&country_core::resources::walls::public_walls::PublicWallState,
    rectangle:utils::geometry::Rectangle2d,
    walls:&country_core::resources::walls::public_walls::PublicWalls,
    segments:&[super::find_overlap_segments::OverhangSegment],
    world:&country_core::systems::collision::world::RaycastWorld,
    terrain:&country_core::resources::terrain_height::TerrainHeightsData,
    library:&mut country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
    output:&mut crate::construct_arches::WallConstructorOutput,
    count:&mut u64,
) {
    use country_core::{resources::roofs::roof_buffers::WoodenPlankSsbo,systems::wall::valid_enclosure::ShapeParameters};
    let storage=library.get_mut::<WoodenPlankSsbo>().expect("原大托架要求 WoodenPlankSsbo");
    let mut writer=output.gpu.plank_writer(storage,id.0);
    let mut seen=std::collections::HashSet::new();
    for segment in segments {
        let other=walls.get(segment.wall).expect("原大托架邻墙必须存在");
        let shape=ShapeParameters::from_snapshot(other.get_wall_shape()).get_shape().expect("原大托架邻墙需要可用形状");
        let first=seen.insert(segment.wall);
        visit_large_rays(segment,shape,rectangle,wall.authority.elevation(),other.authority.elevation(),first,|point| {
            for other_segment in segments {
                if other_segment.wall==segment.wall {continue;}
                let other_wall=walls.get(other_segment.wall).expect("原大托架遮挡墙必须存在");
                if super::circle_categorize::shape_contains(other_wall.get_wall_shape(),glam::Vec2::new(point.x,point.z)){return false;}
            }
            true
        },|upper,direction,lower,index| {
            let hit=world.raycast_simple_excluding_wall_id(upper,direction,0x31cf,id);
            let tag=hit.handle[0] as u8;
            if tag==0x11 {return;}
            // 原版对所有非空命中先采样地形，再判别碰撞来源；保留失败顺序。
            let height=terrain.sample_at_world_xz(hit.position.x,hit.position.z);
            let allowed=match tag {
                0 | 6 => true,
                2 => hit.handle[0]&0x100==0,
                3|4|5|7|8|10|11|13|14 => false,
                1|9|12|15|16 => panic!("原大托架遇到不可达碰撞来源"),
                _ => return,
            };
            let dx=hit.position.x-upper.x;let dz=hit.position.z-upper.z;
            let horizontal=(dz*dz+dx*dx).sqrt();
            if 0.73999995<=horizontal && 0.6<=(wall.authority.elevation()-height)+ -0.3 && allowed {
                let seed=(id.0 as u32).wrapping_add(segment.wall.0 as u32).wrapping_add(index as u32);
                let _=writer.push(&large_bracket_piece(lower,upper,id,seed));
                *count=count.wrapping_add(1);
            }
        });
    }
}
