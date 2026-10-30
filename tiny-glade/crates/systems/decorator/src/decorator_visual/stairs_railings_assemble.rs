//! B `decorator_visual/stairs_railings_assemble.rs` — 楼梯扶手装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译（4 业务 VA）+ 骨架 callees（O）：
//! Affine3Packed::from_cols（0x14089d500）、BrickSourceId::from_wall_id
//! （0x140858750）、swizzlers x0y、memmove 族、rapier 碰撞、blueprint 链。
//!
//! 结构恢复（O 业务链，U 主体）：assemble_railings 0x1407f98e0（扶手段装配，
//! 67 callees）、railings_tips 0x1407fcc80（12+ 参面，扶手端头，Affine3Packed
//! + BrickSourceId 消费）、construct_wooden_railing 0x1407fdf90（8 参面，木扶手
//! 几何，sqrt 弦长计算）、add_corner_stone_railing_piece 0x1407febb0（13+ 参面，
//! 角石扶手件）。

use glam::{Vec2, Vec3};
use country_core::{geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo},
    resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo, walls::publication_fields::WallId},
    render::sparse_instance_buffer::SparseInstanceBufferWriter};


/// 原SSR只在本族消费相邻的两种writer；这里借用真实owner，不复制缓存或GPU分配。
pub struct RailWriters<'borrow, 'storage> {
    pub bricks: &'borrow mut SparseInstanceBufferWriter<'storage, BrickTransformSsbo>,
    pub wooden_planks: &'borrow mut SparseInstanceBufferWriter<'storage, WoodenPlankSsbo>,
}

/// 0x1407f98e0：两侧曲面栏杆、分层砖、木扶手和端部横向压顶。
#[allow(clippy::too_many_arguments)]
pub fn assemble_railings(
    writers: &mut RailWriters<'_, '_>, widths: &[f32], tangents: &[Vec2], positions: &[Vec3],
    rng: &mut fastrand::Rng, brick_source: u32, wooden_source: u32,
    _signifiers: &country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    endpoint_flags: [bool; 2], style: u8, sub_segment_kind: u8, _rail_metadata: u8,
    suppressed: [bool; 2], endpoint_heights: [Option<f32>; 2], auxiliary: &[f32],
) {
    emit_railings(widths, tangents, positions, rng, brick_source, wooden_source,
        endpoint_flags, style, sub_segment_kind, suppressed, endpoint_heights, auxiliary,
        &mut |piece| { let _ = writers.bricks.push(&piece); },
        &mut |piece| { let _ = writers.wooden_planks.push(&piece); });
}

fn railing_horizontal_unit(v: Vec2) -> Vec2 {
    let reciprocal = 1.0 / (v.y * v.y + v.x * v.x).sqrt();
    if reciprocal > 0.0 && reciprocal.is_finite() { v * reciprocal } else { Vec2::ZERO }
}

fn railing_brick(transform: Affine3Packed, source: u32, seed: u32,
    distances: [f32; 2], flags: u32) -> BrickTransformSsbo {
    BrickTransformSsbo { affine_columns: transform.cols.map(|v| v.to_array()),
        seed, material: BrickSourceId::from_wall_id(WallId(u64::from(source))).0,
        wall_distance: distances, flags, ..Default::default() }
}

#[allow(clippy::too_many_arguments)]
fn emit_railings(
    widths: &[f32], tangents: &[Vec2], positions: &[Vec3], rng: &mut fastrand::Rng,
    brick_source: u32, wooden_source: u32, endpoint_flags: [bool; 2], style: u8,
    sub_segment_kind: u8, suppressed: [bool; 2], endpoint_heights: [Option<f32>; 2], auxiliary: &[f32],
    emit_brick: &mut impl FnMut(BrickTransformSsbo), emit_wood: &mut impl FnMut(WoodenPlankSsbo),
) {
    use utils::curve::{Curve3, CurveInternalCoord};
    assert_eq!(positions.len(), auxiliary.len());
    let lifts = endpoint_heights.map(|height| height.map_or(0.0, |v| {
        let d = 0.15 - v; if d > 0.0 { d } else { 0.0 }
    }));
    let profile = if sub_segment_kind.wrapping_sub(2) < 3 { sub_segment_kind - 1 } else { 0 };
    let last = positions.len().wrapping_sub(1);
    let interpolate_lift = |index: usize| {
        let t = index as f32 / last as f32;
        t * lifts[1] + (1.0 - t) * lifts[0]
    };
    for (side, &disabled) in suppressed.iter().enumerate() {
        let style = if disabled || profile == 2 { 0 } else { style };
        let base_height = match style {
            1 if profile == 1 => f32::from_bits(0x3e8ac083),
            1 if profile == 2 => return,
            1 => f32::from_bits(0x3e72b021),
            2 if profile == 2 => return,
            2 => 0.58,
            3 => 0.06,
            _ => -0.04,
        };
        // 原先取first宽度；即使positions为空，也不能吞掉该边界失败。
        let first_width = widths[0];
        let negative_zero = f32::from_bits(base_height.to_bits() & 0x8000_0000);
        let mut side_points = Vec::with_capacity(positions.len());
        for (index, ((&width, tangent), &point)) in widths.iter().zip(tangents).zip(positions).enumerate() {
            let next_y = positions.get(index + 1).map_or(point.y, |next| point.y.max(next.y));
            let width = width + -0.268;
            let width = if width <= 0.1 { 0.1 } else { width };
            let width = width + auxiliary[index] * 0.35 * rng.f32();
            let normal = Vec3::new(-tangent.y, 0.0, tangent.x);
            let lateral = if side == 0 { point - (normal * width) * 0.5 }
                else { (normal * width) * 0.5 + point };
            let lift = interpolate_lift(index);
            let y = lateral.y + lift + base_height;
            let y = if style == 0 { y } else { y.max(point.y.max(next_y)) };
            let horizontal_lift = lift * 0.0;
            side_points.push(Vec3::new(lateral.x + horizontal_lift + negative_zero, y,
                horizontal_lift + lateral.z + negative_zero));
        }
        let Ok(curve) = Curve3::try_new_from_points(side_points, false) else { continue; };
        let Ok(mut curve) = curve.try_resample(0.5, true) else { continue; };
        let layers = if style == 2 { 2 } else { 1 };
        let layer_zero_y = if style == 2 { -0.05 } else { 0.0 };
        let layer_one_y = if style == 2 { -0.54 } else { -0.49 };
        let thickness = if style == 2 || style == 3 { -0.1 } else { 0.0 }
            + if style == 0 { f32::from_bits(0x3e970a3f) } else { f32::from_bits(0x3f0d4fe0) };
        let base_dimension = thickness + f32::from_bits(0x3e2c0831);
        let half_thickness = thickness * 0.5;
        let horizontal_half = thickness * 0.0 * 0.5;
        let narrow = if first_width > 0.5 { 0.0 } else { f32::from_bits(0xbdb43958) };
        for layer in 0..layers {
            let splits = utils::random_splits(curve.points.len(),
                (1.0 / curve.points.len() as f32) * 0.4, rng);
            let mut samples = Vec::with_capacity(splits.len());
            let mut aux = Vec::with_capacity(splits.len());
            for &u in &splits {
                let value = u.clamp(0.0, 1.0) * (auxiliary.len() - 1) as f32;
                let index = value.floor() as usize;
                let coord = CurveInternalCoord::new_for_point_count(index, value - value.trunc(), auxiliary.len());
                aux.push((auxiliary[coord.segment + 1] - auxiliary[coord.segment]) * coord.t + auxiliary[coord.segment]);
                samples.push(curve.get_pos_at_u(u));
            }
            let y_offset = if layer == 0 { layer_zero_y } else { layer_one_y };
            let extra_height = if layer == 0 { 0.0 } else { -0.15 };
            let lower_layer = style != 2 || layer != 0;
            for (index, pair) in samples.windows(2).enumerate() {
                let from = pair[0]; let to = pair[1];
                let from_y = from.y + y_offset; let to_y = to.y + y_offset;
                let previous = if index == 0 { tangents[0] }
                    else { Vec2::new(to.x - samples[index - 1].x, to.z - samples[index - 1].z) };
                let next = if index == samples.len() - 2 { *tangents.last().unwrap_or(&Vec2::ZERO) }
                    else { let next=samples[(index + 1).min(samples.len()-1)]; Vec2::new(next.x-from.x,next.z-from.z) };
                let previous = railing_horizontal_unit(previous); let next = railing_horizontal_unit(next);
                let dx = from.x - to.x; let dz = from.z - to.z;
                let horizontal_squared = dz * dz + dx * dx;
                let ratio = horizontal_squared.sqrt() / (from_y - to_y).abs();
                if !(ratio >= 0.1) { continue; }
                let first_gate = index == 0 && endpoint_flags[0];
                let last_gate = index == samples.len() - 2 && endpoint_flags[1];
                let diagonal = if ratio >= f32::from_bits(0x3f204189) {
                    ratio >= 0.25 && !first_gate && !last_gate
                } else { ratio >= 0.25 && previous.y * next.y + previous.x * next.x >= 0.95 && !first_gate && !last_gate };
                let (mut bottom, top, height_span) = if diagonal { (from_y,to_y,0.0) } else {
                    let minimum = from_y.min(to_y);
                    let top = from_y.max(to_y);
                    if (first_gate && from_y < to_y) || (last_gate && to_y < from_y) {
                        let middle = to_y * 0.5 + from_y * 0.5;
                        (middle,middle,top-minimum)
                    } else { (minimum,minimum,top-minimum) }
                };
                let direction = railing_unit(Vec3::new(to.x-from.x,top-bottom,to.z-from.z));
                let inverse = 1.0 / (direction.z * direction.z + direction.x * direction.x).sqrt();
                let across = Vec3::new(-inverse * direction.z,0.0,direction.x * inverse);
                let r = rng.f32();
                let vertical_jitter = (r-(1.0-r)) * 0.05;
                let distance = (dz * dz + ((bottom-top) * (bottom-top) + dx * dx)).sqrt();
                let distance = if distance <= 0.2 { 0.2 } else { distance };
                let transverse_width = rng.f32() * 0.05 + 0.27;
                let mut height = height_span + base_dimension + extra_height;
                bottom = (top * 0.5 + bottom * 0.5) - half_thickness;
                let a = aux[index]; let mut up = Vec3::Y; let mut shrink;
                if style == 0 {
                    shrink = 0.0;
                    if a > 0.1 {
                        let rise = rng.f32() * 0.1;
                        let jitter = Vec3::new(rng.f32() + -0.5,0.0,rng.f32() + -0.5);
                        let random_shrink = rng.f32();
                        let weight = if a+a <= 1.0 { a+a } else { 1.0 };
                        bottom += rise * weight;
                        up = jitter * 0.3 + Vec3::Y;
                        shrink = weight * 0.3 * random_shrink;
                    }
                } else if profile == 0 || profile == 3 {
                    if !lower_layer {
                        bottom += (rng.f32() * 0.1) * a;
                        let jitter = Vec3::new(rng.f32() + -0.5,0.0,rng.f32() + -0.5);
                        up = Vec3::new(jitter.x*0.3*a+0.0,jitter.y*0.3*a+1.0,jitter.z*0.3*a+0.0);
                    }
                    shrink = a * 0.2 * rng.f32();
                } else {
                    if !lower_layer { bottom += rng.f32() * 0.1 * a; }
                    let weight = a * 0.3;
                    let rise = rng.f32() * 0.1;
                    let random_height = rng.f32(); let random_shrink = rng.f32();
                    let perpendicular = railing_cross(across,direction);
                    up = railing_unit(Vec3::Y * (1.0-weight) + perpendicular * weight);
                    bottom += rise * a;
                    height += random_height * -0.12 * a;
                    shrink = a * 0.1 * random_shrink;
                }
                shrink = 1.0 - shrink;
                let transform = Affine3Packed::from_cols(direction * (distance*shrink),
                    up * (vertical_jitter + height*shrink), across * ((transverse_width+narrow)*shrink),
                    Vec3::new((to.x*0.5+from.x*0.5)-horizontal_half,bottom,
                        (to.z*0.5+from.z*0.5)-horizontal_half));
                emit_brick(railing_brick(transform,brick_source,rng.u32(0..1000),[0.0,1.0],
                    if profile==1 {0xc0} else {0x40}));
            }
        }
        if style == 3 {
            let first = positions[0]; let second = positions[1];
            if endpoint_flags[0] {
                curve.points[0] = curve.points[1] * 0.4 + curve.points[0] * 0.6;
            }
            if endpoint_flags[1] {
                let last = curve.points.len()-1;
                curve.points[last] = curve.points[last-1] * 0.4 + curve.points[last] * 0.6;
            }
            let spacing = if profile == 3 { 0.75 } else { f32::from_bits(0x3fa89375) };
            if let Ok(resampled) = curve.try_resample(spacing,true) {
                let selected = if resampled.points.len()>2 || profile!=3 { &resampled } else { &curve };
                let mut aux = vec![0.0;selected.points.len()];
                if let Ok(original) = Curve3::try_new_from_points(positions.to_vec(),true) {
                    for (value,point) in aux.iter_mut().zip(&selected.points) {
                        let u=original.get_approx_u_from_pos(Vec2::new(point.x,point.z));
                        let coord=original.get_coord_at_u(u);
                        *value=(auxiliary[coord.segment+1]-auxiliary[coord.segment])*coord.t+auxiliary[coord.segment];
                    }
                }
                let slope=(first.y-second.y).abs()/((first.z-second.z)*(first.z-second.z)
                    +(first.x-second.x)*(first.x-second.x)).sqrt();
                emit_wooden_railing(&selected.points,&aux,slope,wooden_source,rng,&mut *emit_wood);
            }
        }
    }
    if sub_segment_kind < 2 || sub_segment_kind.wrapping_sub(1) & !2 == 0 {
        for index in [0,last] {
            let half_width=widths[index]+-0.268+-0.27;
            let half_width=if half_width<=0.0 {0.0}else{half_width}*0.5;
            let tangent=tangents[index];
            let normal=Vec3::new(-tangent.y,0.0,tangent.x);
            let forward=Vec3::new(tangent.x,0.0,tangent.y);
            if half_width<0.05 {continue;}
            let point=positions[index];let sign=if index==0 {1.0}else{-1.0};
            let shift=forward* f32::from_bits(0x3d8f5c2a)*sign;
            let from=point-normal*half_width+shift; let to=point+normal*half_width+shift;
            let delta=to-from;
            let full_width=(delta.z*delta.z+(delta.y*delta.y+delta.x*delta.x)).sqrt();
            let length=full_width;
            let count=(length/0.46)as usize;
            let jitter=if length*0.3>=0.2 {0.2}else{length*0.3};
            let splits=utils::random_splits(count+1+usize::from(count==0),jitter/length,rng);
            let direction=railing_horizontal_unit(Vec2::new(-delta.z,delta.x));
            let basis=utils::xz_rotation_mat3_bad(direction);
            let gate=(index==0 && endpoint_flags[0])||(index==last && endpoint_flags[1]);
            let down=if gate {0.32}else{0.22};let thickness=if gate {0.26}else{0.18};
            let height=if gate {0.66}else{f32::from_bits(0x3ef5c290)};
            let lift=interpolate_lift(index);
            for pair in splits.windows(2) {
                // 原横向砖沿to→from插值，与侧面样本方向独立。
                let a=from*pair[0]+to*(1.0-pair[0])+Vec3::new(lift*0.0,lift,lift*0.0);
                let b=from*pair[1]+to*(1.0-pair[1])+Vec3::new(lift*0.0,lift,lift*0.0);
                let delta=a-b;
                let length=(delta.z*delta.z+(delta.y*delta.y+delta.x*delta.x)).sqrt();
                let transform=Affine3Packed::from_cols(basis.x_axis*length,basis.y_axis*height,
                    basis.z_axis*thickness,Vec3::new(b.x*0.5+a.x*0.5,
                        (b.y*0.5+a.y*0.5)-down,b.z*0.5+a.z*0.5));
                emit_brick(railing_brick(transform,brick_source,rng.u32(0..1000),
                    [pair[0]*full_width,pair[1]*full_width],0x40));
            }
        }
    }
}

/// 0x1407fcc80（真实18参）：先按图、墙归属和三条遮挡射线判端头，再写砖/木实例。
#[allow(clippy::too_many_arguments)]
pub fn railings_tips(
    state: &country_core::resources::stairs::stairs_state::StairsState,
    requests: &country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests,
    nodes: [country_core::resources::walls::decorator::DecoratorId; 2],
    rng: &mut fastrand::Rng, brick_source: u32, wooden_source: u32, style: u8,
    writers: &mut RailWriters<'_, '_>, widths: &[f32], tangents: &[Vec2], positions: &[Vec3],
    world: &country_core::systems::collision::world::RaycastWorld,
    storage: &country_core::resources::walls::decorator_storage::DecoratorStorage,
    _signifiers: &country_core::systems::ui::signifiers::signifier_create::UiSignifierStream,
    endpoint_flags: [bool; 2],
) {
    let enabled = railing_tip_endpoints(nodes.map(|id| id.0), widths,tangents,positions,endpoint_flags,
        |first,second| requests.alignments.edge_weight(
            country_core::resources::walls::decorator::DecoratorId(first),
            country_core::resources::walls::decorator::DecoratorId(second)).map(|kind| kind.tag),
        // canonical UnGraphMap邻接迭代器的精确size_hint直接读取长度，保持原O(1)度数门。
        |node| state.graph().neighbors(node).size_hint().0,
        |node| storage.try_lookup_storage(country_core::resources::walls::decorator::DecoratorId(node))
            .and_then(|address| address.try_as_wall()),
        |origin,direction,max_toi,wall| world.raycast_simple_with_max_toi_excluding_wall_id(
            origin,direction,max_toi,1,wall).is_miss());
    emit_railing_tips(widths,tangents,positions,enabled,style,brick_source,wooden_source,rng,
        &mut |piece| {let _=writers.bricks.push(&piece);},
        &mut |piece| {let _=writers.wooden_planks.push(&piece);});
}

#[allow(clippy::too_many_arguments)]
fn railing_tip_endpoints(
    nodes: [u32; 2], widths: &[f32], tangents: &[Vec2], positions: &[Vec3], flags: [bool; 2],
    mut alignment: impl FnMut(u32,u32)->Option<u8>, mut degree: impl FnMut(u32)->usize,
    mut wall: impl FnMut(u32)->Option<country_core::resources::walls::WallId>,
    mut miss: impl FnMut(Vec3,Vec3,f32,country_core::resources::walls::WallId)->bool,
) -> [bool; 2] {
    let mut enabled=[false;2];
    for end in 0..2 {
        if !flags[end] {continue;}
        // 原先查alignment后取graph；不能将None分支提到graph读取之前。
        let kind=alignment(nodes[end],nodes[1-end]);
        let degree=degree(nodes[end]);
        if degree!=1 || kind!=Some(1) {continue;}
        let Some(wall)=wall(nodes[end]) else {continue;};
        let index=if end==0 {0}else{positions.len().wrapping_sub(1)};
        let position=positions[index];let tangent=tangents[index];
        let width=widths[index]+-0.5+-0.5;
        let width=if width<=0.1 {0.1}else{width};
        let forward=Vec3::new(tangent.x,0.0,tangent.y);
        let normal=Vec3::new(-tangent.y,0.0,tangent.x);
        let first_origin=forward*0.5*0.5+position;
        let second_origin=Vec3::new(position.x-width*normal.x*0.5,
            position.y-width*normal.y*0.5,normal.z*width*-0.5+position.z);
        let third_origin=Vec3::new(position.x+0.0,position.y+0.25,position.z+0.0);
        enabled[end]=miss(first_origin,-forward,0.5,wall)
            &&miss(second_origin,normal,width,wall)
            &&miss(third_origin,Vec3::new(-0.0,-1.0,-0.0),0.5,wall);
    }
    enabled
}

#[allow(clippy::too_many_arguments)]
fn emit_railing_tips(
    widths: &[f32], tangents: &[Vec2], positions: &[Vec3], enabled: [bool;2], style: u8,
    brick_source: u32, wooden_source: u32, rng: &mut fastrand::Rng,
    emit_brick: &mut impl FnMut(BrickTransformSsbo), emit_wood: &mut impl FnMut(WoodenPlankSsbo),
) {
    use utils::curve::Curve3;
    for (end,enabled) in enabled.into_iter().enumerate() {
        if !enabled {continue;}
        let index=if end==0 {0}else{positions.len().wrapping_sub(1)};
        let point=positions[index];let tangent=tangents[index];
        let sign=if end==0 {1.0}else{-1.0};
        let forward=Vec3::new(tangent.x,0.0,tangent.y);
        let position=Vec3::new(point.x+forward.x*sign*0.0975,
            point.y+sign*forward.y*0.0975,point.z+forward.z*sign*0.0975);
        let inverse=1.0/(tangent.x*tangent.x+tangent.y*tangent.y).sqrt();
        let normal=Vec3::new(-tangent.y*inverse,0.0,tangent.x*inverse);
        let width=widths[index]+-0.5;
        let from=position+normal*width*0.5;
        let to=position-normal*width*0.5;
        let delta=from-to;
        let length=(delta.z*delta.z+(delta.y*delta.y+delta.x*delta.x)).sqrt();
        let count=((length/0.46)as usize).max(1);
        let basis=utils::xz_rotation_mat3_bad(tangent*inverse);
        if style>1 && style!=2 {
            let width=width+0.19;
            let points=vec![position+normal*width*0.5,position-normal*width*0.5];
            if let Ok(curve)=Curve3::try_new_from_points(points,false) {
                if let Ok(resampled)=curve.try_resample(f32::from_bits(0x3fa89375),true) {
                    let aux=vec![0.0;resampled.points.len()];
                    emit_wooden_railing(&resampled.points,&aux,0.0,wooden_source,rng,&mut *emit_wood);
                }
            }
            continue;
        }
        let layers=if style==2 {2}else{1};
        let jitter=if length*0.3>=0.2 {0.2}else{length*0.3};
        for layer in 0..layers {
            let splits=utils::random_splits(count+1,jitter/length,rng);
            let rise=(layer as f32+1.0)*0.325;
            let zero=(layer as f32+1.0)*0.0;
            for pair in splits.windows(2) {
                let a=Vec3::new(pair[0]*to.x+from.x*(1.0-pair[0])+zero,
                    pair[0]*to.y+from.y*(1.0-pair[0])+rise,
                    from.z*(1.0-pair[0])+pair[0]*to.z+zero);
                let b=Vec3::new(pair[1]*to.x+from.x*(1.0-pair[1])+zero,
                    pair[1]*to.y+from.y*(1.0-pair[1])+rise,
                    from.z*(1.0-pair[1])+pair[1]*to.z+zero);
                let delta=a-b;
                let segment_length=(delta.z*delta.z+(delta.y*delta.y+delta.x*delta.x)).sqrt();
                let translation=Vec3::new(a.x*0.5+b.x*0.5,b.y*0.5+a.y*0.5+-0.23,
                    b.z*0.5+a.z*0.5);
                let transform=Affine3Packed::from_cols(basis.x_axis*segment_length,
                    basis.y_axis*0.46,basis.z_axis*0.255,translation);
                emit_brick(railing_brick(transform,brick_source,rng.u32(0..1000),
                    [pair[0]*length,pair[1]*length],0x40));
            }
        }
    }
}

/// 0x1407fdf90：沿真实重采样曲线生成立柱与两层木横梁。
/// Sparse容量失败保持原逐件部分写入，后续随机抽取与几何仍继续。
pub fn construct_wooden_railing(
    writer: &mut SparseInstanceBufferWriter<'_, WoodenPlankSsbo>,
    positions: &[Vec3],
    auxiliary_samples: &[f32],
    slope: f32,
    source_id: u32,
    rng: &mut fastrand::Rng,
) {
    emit_wooden_railing(positions, auxiliary_samples, slope, source_id, rng, |piece| {
        let _ = writer.push(&piece);
    });
}

fn railing_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
}

fn railing_unit(a: Vec3) -> Vec3 {
    // 原SSE先加y²+x²，再与z²相加；不用反编译文本重排横向加法。
    a * (1.0 / (a.z * a.z + (a.y * a.y + a.x * a.x)).sqrt())
}

fn railing_beam(direction: Vec3, length: f32, position: Vec3) -> Affine3Packed {
    let unit = railing_unit(direction);
    let across = railing_cross(unit, Vec3::Y);
    let across = across * (1.0 / (across.z * across.z + (across.y * across.y + across.x * across.x)).sqrt());
    let normal = railing_cross(across, unit);
    let normal = normal * (1.0 / (normal.z * normal.z + (normal.y * normal.y + normal.x * normal.x)).sqrt());
    Affine3Packed::from_cols(across * 0.157, unit * length, normal * 0.157, position)
}

fn emit_wooden_railing(
    positions: &[Vec3], auxiliary_samples: &[f32], slope: f32, source_id: u32,
    rng: &mut fastrand::Rng, mut emit: impl FnMut(WoodenPlankSsbo),
) {
    for (index, &point) in positions.iter().enumerate() {
        let random = [rng.f32(), rng.f32(), rng.f32()];
        // 原先消耗三次随机，再检查aux边界，失败不能提前改变随机状态。
        let aux = auxiliary_samples[index];
        let jitter = Vec3::new(
            aux * (random[0] + -0.5) * 0.3 + 0.0,
            aux * (random[1] + -0.5) * 0.3 + 1.0,
            (random[2] + -0.5) * aux * 0.3 + 0.0,
        );
        let neighbor = positions.get(index + 1).copied()
            .unwrap_or(positions[index.saturating_sub(1)]);
        let dx = point.x + 0.0 - neighbor.x;
        let dz = point.z + 0.0 - neighbor.z;
        let inverse = 1.0 / (dz * dz + dx * dx).sqrt();
        let tangent = Vec3::new(dx * inverse, 0.0, dz * inverse);
        let second_inverse = 1.0 / (tangent.x * tangent.x + tangent.z * tangent.z).sqrt();
        let across = Vec3::new(-second_inverse * tangent.z, 0.0,
            second_inverse * tangent.x);
        let transform = Affine3Packed::from_cols(tangent * 0.157, jitter * 1.036,
            across * 0.157, Vec3::new(point.x + 0.0, (point.y + 0.07) + -0.18, point.z + 0.0));
        emit(WoodenPlankSsbo { transform, gate_seed: 0,
            piece_seed: rng.u32(0..1000) as i32, wall_id: source_id, flags: 24 });
    }
    for (index, pair) in positions.windows(2).enumerate() {
        let from = pair[0];
        let raw_to = pair[1];
        let r = [rng.f32(), rng.f32(), rng.f32()];
        let aux = auxiliary_samples[index];
        let start_y = from.y + -0.15;
        let to = Vec3::new(raw_to.x + (r[0] + -0.5) * aux * 0.2,
            (r[1] + -0.5) * aux * 0.2 + (raw_to.y + -0.15),
            (r[2] + -0.5) * aux * 0.2 + raw_to.z);
        let delta = Vec3::new(to.x - from.x, to.y - start_y, to.z - from.z);
        let reverse = Vec3::new(from.x - to.x, start_y - to.y, from.z - to.z);
        let length = (reverse.z * reverse.z + (reverse.y * reverse.y + reverse.x * reverse.x)).sqrt();
        // 原坡度门只抑制第一横梁的push和整数seed；上游浮点随机仍全部执行。
        let first = railing_beam(delta, length,
            Vec3::new(from.x + 0.0, start_y + 0.565, from.z + 0.0));
        if slope < 1.6 {
            emit(WoodenPlankSsbo { transform: first, gate_seed: 0,
                piece_seed: rng.u32(0..1000) as i32, wall_id: source_id, flags: 24 });
        }
        let r = [rng.f32(), rng.f32(), rng.f32()];
        // 第二层继续扰动第一次已扰动的端点；沿用第一层长度，不重新估算弦长。
        let second_delta = Vec3::new(
            (to.x + aux * (r[0] + -0.5) * 0.2) - from.x,
            ((to.y + -0.15) + aux * (r[1] + -0.5) * 0.2) - (start_y + -0.15),
            (to.z + aux * (r[2] + -0.5) * 0.2) - from.z,
        );
        let second = railing_beam(second_delta, length,
            Vec3::new(from.x + 0.0, start_y + f32::from_bits(0x3f74fdf4), from.z + 0.0));
        emit(WoodenPlankSsbo { transform: second, gate_seed: 0,
            piece_seed: rng.u32(0..1000) as i32, wall_id: source_id, flags: 24 });
    }
}

/// 0x1407febb0：按相邻楼梯的端点方向选择角石，并独立抑制两侧。
pub fn add_corner_stone_railing_piece(
    writer: &mut SparseInstanceBufferWriter<'_, BrickTransformSsbo>,
    source_id: u32, style: u8, widths: &[f32], tangents: &[Vec2], positions: &[Vec3],
    suppressed: [bool; 2], direction: Vec2,
) {
    emit_corner_stone(source_id, style, widths, tangents, positions, suppressed, direction,
        |piece| { let _ = writer.push(&piece); });
}

fn emit_corner_stone(
    source_id: u32, style: u8, widths: &[f32], tangents: &[Vec2], positions: &[Vec3],
    suppressed: [bool; 2], direction: Vec2, mut emit: impl FnMut(BrickTransformSsbo),
) {
    let first = tangents[0];
    let dot = first.y * direction.y + first.x * direction.x;
    let index = if dot > 0.0 { widths.len().wrapping_sub(1) } else { 0 };
    let width = widths[index];
    let tangent = tangents[index];
    let inverse = 1.0 / (tangent.x * tangent.x + tangent.y * tangent.y).sqrt();
    let normal = Vec3::new(-tangent.y * inverse, 0.0, tangent.x * inverse);
    let position = positions[index];
    let width = width + -0.268;
    let width = if width <= 0.1 { 0.1 } else { width };
    let forward = Vec3::new(tangent.x * inverse, 0.0, tangent.y * inverse);
    let sign = if dot > 0.0 { -1.0 } else { 1.0 };
    let base = Vec3::new(forward.x * sign * 0.27 * 0.5 + position.x,
        forward.y * sign * 0.27 * 0.5 + position.y,
        sign * forward.z * 0.27 * 0.5 + position.z);
    let height = [0.0, 0.769, 1.37, 0.769][usize::from(style)];
    let z_axis = railing_cross(normal, Vec3::Y) * 0.27;
    for (side, disabled) in suppressed.into_iter().enumerate() {
        if disabled { continue; }
        let translation = if side == 0 {
            Vec3::new(width * normal.x * -0.5 + base.x,
                width * normal.y * -0.5 + base.y, width * normal.z * -0.5 + base.z)
        } else {
            Vec3::new(base.x + width * normal.x * 0.5,
                base.y + width * normal.y * 0.5, base.z + width * normal.z * 0.5)
        };
        let transform = Affine3Packed::from_cols(normal * 0.27, Vec3::new(0.0, height, 0.0),
            z_axis, translation);
        emit(BrickTransformSsbo { affine_columns: transform.cols.map(|v| v.to_array()),
            seed: index as u32,
            material: BrickSourceId::from_wall_id(WallId(u64::from(source_id))).0,
            wall_distance: [0.0, 1.0], flags: 0x40, ..Default::default() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn floats(s: &str) -> Vec<f32> {
        if s.is_empty() { return Vec::new(); }
        s.split(',').map(|x| f32::from_bits(u32::from_str_radix(x,16).unwrap())).collect()
    }
    fn points(s: &str) -> Vec<Vec3> { floats(s).chunks_exact(3).map(|v| Vec3::new(v[0],v[1],v[2])).collect() }
    fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
    fn wood_bytes(record: WoodenPlankSsbo) -> Vec<u8> {
        let mut b = Vec::with_capacity(64);
        for col in record.transform.cols { for x in col.to_array() { b.extend(x.to_le_bytes()); } }
        b.extend(record.gate_seed.to_le_bytes()); b.extend(record.piece_seed.to_le_bytes());
        b.extend(record.wall_id.to_le_bytes()); b.extend(record.flags.to_le_bytes()); b
    }
    fn assert_native_record(a: &str, b: &str, context: &str) {
        assert_eq!(a.len(),b.len(),"{context}");
        for (lane,(a,b)) in a.as_bytes().chunks_exact(8).zip(b.as_bytes().chunks_exact(8)).enumerate() {
            let decode=|v:&[u8]|{let bytes:Vec<_>=v.chunks_exact(2).map(|s|u8::from_str_radix(core::str::from_utf8(s).unwrap(),16).unwrap()).collect();u32::from_le_bytes(bytes.try_into().unwrap())};
            let (a,b)=(decode(a),decode(b));
            // ARM与原x86对0*Inf产生的NaN符号不同；仅退化输入的NaN lane按NaN比较，有限值逐位。
            if f32::from_bits(a).is_nan() && f32::from_bits(b).is_nan() {continue;}
            assert_eq!(a,b,"{context} lane {lane}");
        }
    }
    #[derive(Default)]
    struct RailingDevice {
        created: std::sync::atomic::AtomicU64,
        writes: std::sync::atomic::AtomicUsize,
        released: std::sync::atomic::AtomicUsize,
    }
    impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for RailingDevice {
        fn create_buffer(&self, desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc,
            _: &str, _: Option<&[u8]>)
            -> Result<country_core::render::ssbo::rhapsody_boundary::Buffer,
                country_core::render::ssbo::rhapsody_boundary::BackendError> {
            Ok(country_core::render::ssbo::rhapsody_boundary::Buffer {
                id: self.created.fetch_add(1,std::sync::atomic::Ordering::Relaxed)+1,
                byte_size:desc.size as u64,..Default::default()})
        }
        fn update_buffer_bytes(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
            bytes:&[u8], ranges:&[std::ops::Range<usize>], offset:usize)
            -> Result<(),country_core::render::ssbo::rhapsody_boundary::BackendError> {
            for range in ranges { assert!(range.end<=bytes.len());assert!(offset+range.end<=buffer.byte_size as usize); }
            self.writes.fetch_add(1,std::sync::atomic::Ordering::Relaxed);Ok(())
        }
        fn release_buffer(&self,_:&country_core::render::ssbo::rhapsody_boundary::Buffer) {
            self.released.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
        }
    }
    #[test]
    fn railing_real_sparse_two_buffers_edit_stable_frames_and_retirement() {
        use country_core::render::sparse_instance_buffer::{SparseInstanceBuffer,MeshAtlasAllocation,SparseSsbo};
        use std::sync::{Arc,atomic::Ordering};
        let device=Arc::new(RailingDevice::default());
        let mut bricks=SparseInstanceBuffer::<BrickTransformSsbo>::try_new_owned(device.clone()).unwrap();
        let mut wooden=SparseInstanceBuffer::<WoodenPlankSsbo>::try_new_owned(device.clone()).unwrap();
        let brick_allocation=MeshAtlasAllocation::new(&bricks);
        let wooden_allocation=MeshAtlasAllocation::new(&wooden);
        let fixture=include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m01-railing-main-native.tsv"));
        let v:Vec<_>=fixture.lines().nth(225).unwrap().split('\t').collect();
        let positions=points(v[6]);let widths=floats(v[7]);let ts=floats(v[8]);
        let tangents:Vec<_>=ts.chunks_exact(2).map(|v|Vec2::new(v[0],v[1])).collect();
        let heights=floats(v[10]);let aux=floats(v[9]);
        let mut signifiers=country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::default();
        let mut rng=fastrand::Rng::with_seed(19);
        {
            let mut brick_writer=bricks.writer_for(brick_allocation.allocation());
            let mut wood_writer=wooden.writer_for(wooden_allocation.allocation());
            let mut writers=RailWriters{bricks:&mut brick_writer,wooden_planks:&mut wood_writer};
            assemble_railings(&mut writers,&widths,&tangents,&positions,&mut rng,100003,42,
                &mut signifiers,[false;2],3,1,0,[false;2],[Some(heights[0]),Some(heights[1])],&aux);
        }
        let expected:Vec<_>=v[12].split(';').collect();
        let expected_bricks:Vec<_>=expected.iter().filter_map(|v|v.strip_prefix("b:")).collect();
        let expected_wood:Vec<_>=expected.iter().filter_map(|v|v.strip_prefix("w:")).collect();
        assert!(!expected_bricks.is_empty()&&!expected_wood.is_empty());
        for (index,expected) in expected_bricks.iter().enumerate() {
            assert_native_record(&hex(&bricks.data[index].observed_bytes()),expected,"actual sparse brick");
        }
        for (index,expected) in expected_wood.iter().enumerate() {
            assert_native_record(&hex(&wood_bytes(wooden.data[index])),expected,"actual sparse wood");
        }
        assert_eq!(rng.get_seed(),v[11].parse::<u64>().unwrap());
        bricks.gpu_flush(device.as_ref()).unwrap();wooden.gpu_flush(device.as_ref()).unwrap();
        let writes=device.writes.load(Ordering::Relaxed);assert!(writes>=2);
        for _ in 0..4 {bricks.gpu_flush(device.as_ref()).unwrap();wooden.gpu_flush(device.as_ref()).unwrap();}
        assert_eq!(device.writes.load(Ordering::Relaxed),writes,"稳定帧不重传");
        {
            let mut brick_writer=bricks.writer_for(brick_allocation.allocation());brick_writer.clear();
            let mut wood_writer=wooden.writer_for(wooden_allocation.allocation());wood_writer.clear();
            let mut writers=RailWriters{bricks:&mut brick_writer,wooden_planks:&mut wood_writer};
            assemble_railings(&mut writers,&widths[..2],&tangents[..2],&positions[..2],
                &mut fastrand::Rng::with_seed(19),100003,42,&mut signifiers,[false;2],3,1,0,
                [false;2],[None;2],&aux[..2]);
        }
        assert!(!bricks.active_words().all(|v|v==0));assert!(!wooden.active_words().all(|v|v==0));
        drop(brick_allocation);drop(wooden_allocation);
        assert_eq!(bricks.waste.lock().len(),1);assert_eq!(wooden.waste.lock().len(),1);
        bricks.gpu_flush(device.as_ref()).unwrap();wooden.gpu_flush(device.as_ref()).unwrap();
        assert_eq!(bricks.free_chunks.read().len(),BrickTransformSsbo::CHUNK_COUNT);
        assert_eq!(wooden.free_chunks.read().len(),WoodenPlankSsbo::CHUNK_COUNT);
        assert!(bricks.active_words().all(|v|v==0));assert!(wooden.active_words().all(|v|v==0));
        drop(bricks);drop(wooden);
        assert_eq!(device.released.load(Ordering::Relaxed),2);
        assert_eq!(Arc::strong_count(&device),1);
    }

    #[test]
    fn railing_sparse_exhaustion_keeps_prefix_and_original_rng_progress() {
        use country_core::render::sparse_instance_buffer::{SparseInstanceBuffer,MeshAtlasAllocation};
        let fixture=include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m01-railing-leaves-native.tsv"));
        let v:Vec<_>=fixture.lines().nth(9).unwrap().split('\t').collect();
        let mut buffer=SparseInstanceBuffer::<WoodenPlankSsbo>::try_new(&RailingDevice::default()).unwrap();
        let allocation=MeshAtlasAllocation::new(&buffer);
        buffer.free_chunks.write().retain(|&chunk|chunk==0);
        let mut rng=fastrand::Rng::with_seed(v[1].parse().unwrap());
        {
            let mut writer=buffer.writer_for(allocation.allocation());
            for _ in 0..31 {writer.push(&WoodenPlankSsbo::default()).unwrap();}
            construct_wooden_railing(&mut writer,&points(v[3]),&floats(v[4]),floats(v[2])[0],42,&mut rng);
        }
        assert_eq!(buffer.byte_map[0],32);
        let first=v[6].split(';').next().unwrap();
        assert_native_record(&hex(&wood_bytes(buffer.data[31])),first,"容量不足保第一片");
        assert_eq!(rng.get_seed(),v[5].parse::<u64>().unwrap(),"失败push不能提前终止随机序列");
        assert!(buffer.free_chunks.read().is_empty());
    }
    // 本crate已直接依赖country-core/native-physics，不设不存在的同名本地feature。
    #[test]
    fn railing_tips_real_graph_bounded_rapier_wall_filter_and_unload() {
        use country_core::{render::sparse_instance_buffer::{SparseInstanceBuffer,MeshAtlasAllocation,SparseSsbo},
            resources::{stairs::{stairs_state::StairsState,stairs_assembly_params::{PlayermadeStairsAssemblyRequests,AlignKind}},
                walls::{decorator::{DecoratorId,DecoratorInfo},decorator_storage::{DecoratorStorage,DecoratorStorageAddr}}},
            systems::collision::{world::{RaycastWorld,RaycastMeshData,upsert_collider,remove_collider},
                query_policy::RaycastGeoSourceProjection}};
        use std::{sync::Arc,collections::HashMap};
        let device=RailingDevice::default();
        let mut bricks=SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&device).unwrap();
        let mut wooden=SparseInstanceBuffer::<WoodenPlankSsbo>::try_new(&device).unwrap();
        let ba=MeshAtlasAllocation::new(&bricks);let wa=MeshAtlasAllocation::new(&wooden);
        let nodes=[DecoratorId(31),DecoratorId(32)];let wall=country_core::resources::walls::WallId(55);
        let mut storage=DecoratorStorage::default();
        for id in nodes {
            storage.decorator_to_storage.insert(id,DecoratorStorageAddr::Wall(wall));
            storage.per_wall.entry(wall).or_default().insert(id,DecoratorInfo{decorator_id:0x100+id.0,..Default::default()});
        }
        let mut state=StairsState::new();state.add_edge(31,32);
        let mut requests=PlayermadeStairsAssemblyRequests::default();
        requests.alignments.add_edge(nodes[0],nodes[1],AlignKind{tag:1,..Default::default()});
        requests.alignments.add_edge(nodes[1],nodes[0],AlignKind{tag:1,..Default::default()});
        let positions=[Vec3::new(-1.25,1.2,0.35),Vec3::new(-0.55,1.51,-0.18)];
        let tangents=[Vec2::new(0.7,-0.3),Vec2::new(0.7,-0.13)];let widths=[1.2,1.4];
        let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        let mesh=Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(0.04),Vec3::ZERO));
        let source=|wall:u64|{let mut b=[0;24];b[8..16].copy_from_slice(&wall.to_le_bytes());RaycastGeoSourceProjection(b)};
        let mut signifiers=country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::default();
        let mut frame=|world:&RaycastWorld,state:&StairsState,requests:&PlayermadeStairsAssemblyRequests| {
            {
                let mut b=bricks.writer_for(ba.allocation());b.clear();
                let mut w=wooden.writer_for(wa.allocation());w.clear();
                railings_tips(state,requests,nodes,&mut fastrand::Rng::with_seed(0),100003,42,0,
                    &mut RailWriters{bricks:&mut b,wooden_planks:&mut w},&widths,&tangents,&positions,
                    world,&storage,&mut signifiers,[true;2]);
            }
            bricks.byte_map[..BrickTransformSsbo::CHUNK_COUNT].iter().map(|&v|usize::from(v)).sum::<usize>()
        };
        assert_eq!(frame(&world,&state,&requests),2,"真实空空间保留两个端头");
        let close=positions[0]+Vec3::new(tangents[0].x,0.0,tangents[0].y)*0.1;
        upsert_collider(&mut world,&mut instances,1u32,source(55),mesh.clone(),close,glam::Quat::IDENTITY);
        world.flush_colliders();
        assert_eq!(frame(&world,&state,&requests),2,"同墙源必须按完整WallId过滤");
        upsert_collider(&mut world,&mut instances,2u32,source(56),mesh.clone(),close,glam::Quat::IDENTITY);
        world.flush_colliders();
        assert_eq!(frame(&world,&state,&requests),1,"另一墙的近距实体挡住首端");
        let distant=positions[0]-Vec3::new(tangents[0].x,0.0,tangents[0].y)*2.0;
        upsert_collider(&mut world,&mut instances,2u32,source(56),mesh.clone(),distant,glam::Quat::IDENTITY);
        world.flush_colliders();
        assert_eq!(frame(&world,&state,&requests),2,"超过原0.5 TOI的实体不阻塞端头");
        state.add_edge(31,33);
        assert_eq!(frame(&world,&state,&requests),1,"度数改变后不沿用上一帧端头");
        state.remove_node(33);
        requests.alignments.edge_weight_mut(nodes[0],nodes[1]).unwrap().tag=0;
        assert_eq!(frame(&world,&state,&requests),1,"非tag1连接不生成端头");
        requests.alignments.edge_weight_mut(nodes[0],nodes[1]).unwrap().tag=1;
        assert_eq!(frame(&world,&state,&requests),2);
        drop(frame);
        for key in [1u32,2u32] {remove_collider(&mut world,&mut instances,&key).unwrap();}
        world.flush_colliders();drop(instances);drop(world);
        assert_eq!(Arc::strong_count(&mesh),1,"退役原碰撞网格没有遗留owner");
        drop(ba);drop(wa);bricks.gpu_flush(&device).unwrap();wooden.gpu_flush(&device).unwrap();
        assert!(bricks.active_words().all(|v|v==0));assert!(wooden.active_words().all(|v|v==0));
        assert_eq!(bricks.free_chunks.read().len(),BrickTransformSsbo::CHUNK_COUNT);
        assert_eq!(wooden.free_chunks.read().len(),WoodenPlankSsbo::CHUNK_COUNT);
    }

    #[test]
    fn railing_tips_match_original_instructions_and_ordered_ray_contract() {
        let fixture=include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m01-railing-tips-native.tsv"));
        for (case,line) in fixture.lines().enumerate() {
            let v:Vec<_>=line.split('\t').collect();let mut rng=fastrand::Rng::with_seed(v[0].parse().unwrap());
            let positions=points(v[7]);let widths=floats(v[8]);let ts=floats(v[9]);
            let tangents:Vec<_>=ts.chunks_exact(2).map(|v|Vec2::new(v[0],v[1])).collect();
            let flags=v[2].parse::<u16>().unwrap();let degree=v[3].parse::<isize>().unwrap();
            let align=v[4].parse::<i16>().unwrap();let address=v[5].parse::<u8>().unwrap();
            let hit=v[6].parse::<isize>().unwrap();let mut rays=Vec::new();
            let enabled=railing_tip_endpoints([31,32],&widths,&tangents,&positions,
                [flags&1!=0,flags&0x100!=0],|_,_|if align>=0 {Some(align as u8)}else{None},
                |_|degree.max(0)as usize,|_|if address==0 {Some(country_core::resources::walls::WallId(55))}else{None},
                |origin,direction,max,wall|{
                    let mut bytes=Vec::new();for v in [origin,direction] {for f in v.to_array(){bytes.extend(f.to_le_bytes());}}
                    rays.push(format!("{}:{:08x}:{}",hex(&bytes),max.to_bits(),wall.0));
                    (rays.len()as isize-1)%3!=hit
                });
            assert_eq!(rays.join(";"),v[11],"rays case {case}");
            let records=std::cell::RefCell::new(Vec::new());
            emit_railing_tips(&widths,&tangents,&positions,enabled,v[1].parse().unwrap(),100003,42,&mut rng,
                &mut|p|records.borrow_mut().push(format!("b:{}",hex(&p.observed_bytes()))),
                &mut|p|records.borrow_mut().push(format!("w:{}",hex(&wood_bytes(p)))));
            assert_eq!(rng.get_seed(),v[10].parse::<u64>().unwrap(),"tips rng case {case}");
            let expected:Vec<_>=v[12].split(';').filter(|v|!v.is_empty()).collect();let actual=records.into_inner();
            assert_eq!(actual.len(),expected.len(),"tips count case {case}");
            for (piece,(a,b)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(&a[..2],&b[..2],"tips kind case {case} piece {piece}");
                assert_native_record(&a[2..],&b[2..],&format!("tips case {case} piece {piece}"));
            }
        }
    }
    #[test]
    fn railing_main_matches_original_instructions() {
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-railing-main-native.tsv"));
        for (case,line) in fixture.lines().enumerate() {
            let v:Vec<_>=line.split('\t').collect();
            let mut rng=fastrand::Rng::with_seed(v[0].parse().unwrap());
            let t=floats(v[8]);let tangents:Vec<_>=t.chunks_exact(2).map(|v|Vec2::new(v[0],v[1])).collect();
            let heights=floats(v[10]); let records=std::cell::RefCell::new(Vec::new());
            let height_mask=v.get(13).map_or(3,|s|s.parse::<u8>().unwrap());
            let flags=v[4].parse::<u16>().unwrap();let suppress=v[5].parse::<u8>().unwrap();
            emit_railings(&floats(v[7]),&tangents,&points(v[6]),&mut rng,100003,42,
                [flags&1!=0,flags&0x100!=0],v[1].parse().unwrap(),v[2].parse().unwrap(),
                [suppress&1!=0,suppress&2!=0],[(height_mask&1!=0).then_some(heights[0]),(height_mask&2!=0).then_some(heights[1])],&floats(v[9]),
                &mut |p|records.borrow_mut().push(format!("b:{}",hex(&p.observed_bytes()))),
                &mut |p|records.borrow_mut().push(format!("w:{}",hex(&wood_bytes(p)))));
            assert_eq!(rng.get_seed(),v[11].parse::<u64>().unwrap(),"main rng case {case}");
            let expected:Vec<_>=v[12].split(';').filter(|v|!v.is_empty()).collect();
            let actual=records.into_inner();
            assert_eq!(actual.len(),expected.len(),"main count case {case}");
            for (piece,(a,b)) in actual.iter().zip(expected).enumerate() {
                assert_eq!(&a[..2],&b[..2],"kind case {case} piece {piece}");
                assert_native_record(&a[2..],&b[2..],&format!("main case {case} piece {piece}"));
            }
        }
    }
    #[test]
    fn railing_leaves_match_original_instructions() {
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-railing-leaves-native.tsv"));
        for (case, line) in fixture.lines().enumerate() {
            let v: Vec<_> = line.split('\t').collect(); let mut actual = Vec::new();
            if v[0] == "w" {
                let mut rng = fastrand::Rng::with_seed(v[1].parse().unwrap());
                emit_wooden_railing(&points(v[3]), &floats(v[4]), floats(v[2])[0],42,&mut rng,
                    |piece| actual.push(hex(&wood_bytes(piece))));
                assert_eq!(rng.get_seed(),v[5].parse::<u64>().unwrap(),"rng case {case}");
                let expected: Vec<_> = v[6].split(';').filter(|v| !v.is_empty()).collect();
                assert_eq!(actual.len(),expected.len(),"count case {case}");
                for (piece,(a,b)) in actual.iter().zip(expected).enumerate() {
                    assert_native_record(a,b,&format!("wood case {case} piece {piece}"));
                }
            } else {
                let direction=floats(v[3]); let tangents=floats(v[5]);
                let tangents:Vec<_>=tangents.chunks_exact(2).map(|p|Vec2::new(p[0],p[1])).collect();
                let flags=v[2].parse::<u8>().unwrap();
                emit_corner_stone(100003,v[1].parse().unwrap(),&floats(v[4]),&tangents,
                    &points(v[6]),[flags&1!=0,flags&2!=0],Vec2::new(direction[0],direction[1]),
                    |piece| actual.push(hex(&piece.observed_bytes())));
                let expected:Vec<_>=v[7].split(';').filter(|v|!v.is_empty()).collect();
                assert_eq!(actual.len(),expected.len(),"corner count {case}");
                for (piece,(a,b)) in actual.iter().zip(expected).enumerate() {
                    assert_native_record(a,b,&format!("corner case {case} piece {piece}"));
                }
            }
        }
    }
}
