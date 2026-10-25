//! UNVERIFIED B fence CPU owner. Original private game policy is recovered here;
//! Perchance, vector math, allocation and Bevy transport remain dependencies.
use crate::{building_gates::GateCurve3, building_wall_state::WallId,
    building_wall_structures::WallWoodenPlankInstance64B};
use glam::Vec3;
use perchance::PerchanceContext;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FenceError { InvalidInput, DuplicatePoint, InvalidCurve, InvalidDirection, UnrecoveredThresholdOwner }
#[derive(Clone, Debug, PartialEq)]
pub struct FencePost {
    pub position: [f32;3], pub top: [f32;3], pub forward: [f32;3],
    pub up: [f32;3], pub side: [f32;3], pub dimensions: [f32;3],
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FenceCpuOutput {
    pub posts: Vec<FencePost>, pub instances: Vec<WallWoodenPlankInstance64B>,
    /// True threshold callback ranges in the original prepared-point sequence.
    pub source_ranges: Vec<[usize;2]>, pub segment_seeds: Vec<u64>,
    pub fence_segments_count: usize, pub fence_segments_seed: u64,
}
#[derive(Clone, Copy)]
pub struct FenceSegmentInput<'a> {
    pub wall: WallId, pub height: f32, pub positions: &'a [[f32;3]],
    pub spacing_extent: f32, pub seed: u64, pub skip_first: bool,
    pub segment_ordinal: usize, pub segment_count: usize,
}
/// Original angle splitter compares horizontal directions but rejects exact
/// adjacent XYZ duplicates. Vertical-only edges preserve the NaN comparison.
pub fn split_fence_angles(points: &[[f32;3]]) -> Result<Vec<usize>, FenceError> {
    if points.len()<2 { return Err(FenceError::InvalidInput); }
    if points.windows(2).any(|p|p[0]==p[1]) { return Err(FenceError::DuplicatePoint); }
    let mut indices=vec![0]; let threshold=f32::from_bits(0x3f32b8c2).cos();
    for (i,p) in points.windows(3).enumerate() {
        let x1=p[1][0]-p[0][0]; let z1=p[1][2]-p[0][2];
        let x2=p[2][0]-p[1][0]; let z2=p[2][2]-p[1][2];
        let dot=(z2*z1+x1*x2)/((z2*z2+x2*x2)*(z1*z1+x1*x1)).sqrt();
        if dot<threshold { indices.push(i+1); }
    }
    indices.push(points.len()-1); Ok(indices)
}
fn post(position:Vec3,forward:Vec3,tilt:[f32;2],height:f32)->FencePost {
    let inverse=1.0/(tilt[1]*tilt[1]+tilt[0]*tilt[0]+1.0).sqrt();
    let up=Vec3::new(tilt[0]*inverse,inverse,tilt[1]*inverse);
    FencePost {position:position.to_array(),top:(up*height+position).to_array(),
        forward:forward.to_array(),up:up.to_array(),side:up.cross(forward).to_array(),dimensions:[0.3,height,0.3]}
}
fn packed(columns:[Vec3;4])->[f32;12] {
    // Affine3Packed::from_cols is the existing meaningful row-major projection.
    [columns[0].x,columns[1].x,columns[2].x,columns[3].x,
     columns[0].y,columns[1].y,columns[2].y,columns[3].y,
     columns[0].z,columns[1].z,columns[2].z,columns[3].z]
}
fn instance(input:FenceSegmentInput<'_>,index:u32,columns:[Vec3;4])->WallWoodenPlankInstance64B {
    WallWoodenPlankInstance64B {affine:packed(columns),segment_ordinal:input.seed as u32,
        source_index:index,wall_id:input.wall.0 as u32}
}
/// Whole process_segment geometry/RNG/board owner, including the endpoint
/// suppression and probabilistic slanted rails. Resampling follows the original
/// private Curve3 corner-candidate policy. This implementation is unverified.
pub fn process_fence_segment(input:FenceSegmentInput<'_>)->Result<FenceCpuOutput,FenceError> {
    if input.positions.len()<2 { return Ok(FenceCpuOutput::default()); }
    if input.segment_count==0 || input.segment_ordinal>input.segment_count
        || !input.height.is_finite() || !input.spacing_extent.is_finite() {return Err(FenceError::InvalidInput);}
    let splits=split_fence_angles(input.positions)?;
    let mut rng=PerchanceContext::new(input.seed as u128);
    let height=input.height+0.3; let mut posts=Vec::new();
    let factor=((input.spacing_extent-1.0)/5.0).max(0.0).min(1.0);
    let smooth=(3.0-(factor+factor))*factor*factor;
    let spacing=smooth*f32::from_bits(0x3fce147a)+(1.0-smooth)*0.7;
    for (span,pair) in splits.windows(2).enumerate() {
        let curve=fence_curve(&input.positions[pair[0]..=pair[1]])?;
        // Native passes length / ceil(length / spacing) to crashy_resample.
        let interval=curve.length/(curve.length/spacing).ceil();
        let sampled=resample_fence_curve(&curve,interval)?;
        for (edge,p) in sampled.windows(2).enumerate() {
            let a=Vec3::from_array(p[0]); let b=Vec3::from_array(p[1]);
            let dx=b.x-a.x; let dz=b.z-a.z; let inv=1.0/(dz*dz+dx*dx).sqrt();
            let forward=Vec3::new(dx*inv,0.0,dz*inv);
            if edge==0 && span==0 {
                let disk=rng.uniform_disc_vec2(); let r=rng.uniform_f32()*0.6+0.4;
                posts.push(post(a,forward,[disk.x*r*r*0.3,0.0],height));
            }
            let disk=rng.uniform_disc_vec2(); let r=rng.uniform_f32()*0.6+0.4; let square=r*r;
            let z=if edge+2==sampled.len(){0.0}else{disk.y*square*0.3};
            posts.push(post(b,forward,[disk.x*square*0.3,z],height));
        }
    }
    let mut instances=Vec::new();
    for (i,p) in posts.iter().enumerate() {
        if (input.skip_first && i==0) || (input.segment_count-1>input.segment_ordinal && i+1==posts.len()) {continue;}
        instances.push(instance(input,i as u32,[Vec3::from_array(p.forward)*(p.dimensions[0]+0.0),
            Vec3::from_array(p.up)*(p.dimensions[1]+0.1),Vec3::from_array(p.side)*(p.dimensions[2]+0.0),
            Vec3::from_array(p.position)+Vec3::new(0.0,-0.1,0.0)]));
    }
    for (pair_index,p) in posts.windows(2).enumerate() {
        let a=Vec3::from_array(p[0].position); let at=Vec3::from_array(p[0].top);
        let b=Vec3::from_array(p[1].position); let bt=Vec3::from_array(p[1].top);
        let draw=rng.uniform_f32(); let lower=draw*0.3+(1.0-draw)*0.1;
        let draw=rng.uniform_f32(); let upper=draw*0.8+(1.0-draw)*0.6;
        let span=(upper-lower)*height; let count=((span*4.0).floor() as usize).min(4).max(1);
        for board in 0..count {
            let t=if count==1 {0.5}else{board as f32/(count-1) as f32};
            let t=t*upper+(1.0-t)*lower; let start=at*t+a*(1.0-t); let end=bt*t+b*(1.0-t);
            let original=end-start; let length=((original.x*original.x+original.y*original.y)+original.z*original.z).sqrt();
            let mut direction=original; let mut anchor=start;
            if rng.uniform_f32()<0.15 {
                if rng.uniform_f32()<0.5 {direction=-direction;anchor=end;}
                let draw=rng.uniform_f32(); direction.y-=(draw*0.5+(1.0-draw)*0.1)*span;
                let sign=[-1.0,1.0][rng.usize_less_than(2)]*0.05;
                direction+=Vec3::new(-original.z,0.0,original.x)*sign;
            }
            let inv=1.0/((direction.x*direction.x+direction.y*direction.y)+direction.z*direction.z).sqrt();
            direction*=inv;
            if direction==Vec3::Y {return Err(FenceError::InvalidDirection);}
            let first=Vec3::Y.cross(direction); let first=first*(1.0/first.length());
            let third=first.cross(direction);
            instances.push(instance(input,board as u32+((pair_index+1)*4) as u32,
                [first*0.1,direction*length,third*0.2,anchor]));
        }
    }
    let fence_segments_count=posts.len().saturating_sub(1);
    Ok(FenceCpuOutput {posts,instances,fence_segments_count,..Default::default()})
}
fn distance3(a:[f32;3],b:[f32;3])->f32 {
    let x=a[0]-b[0];let y=a[1]-b[1];let z=a[2]-b[2];(z*z+y*y+x*x).sqrt()
}
fn fence_curve(points:&[[f32;3]])->Result<GateCurve3,FenceError> {
    if points.len()<2 {return Err(FenceError::InvalidCurve);}
    let mut length=0.0;let mut points_u=Vec::with_capacity(points.len());let mut previous=points[0];
    for &point in points {length+=distance3(previous,point);points_u.push(length);previous=point;}
    if length==0.0 {return Err(FenceError::InvalidCurve);}
    for u in &mut points_u {*u/=length;if !u.is_finite(){return Err(FenceError::InvalidCurve);}}
    Ok(GateCurve3 {positions:points.to_vec(),points_u,length})
}
/// Private crashy_resample preserves source corners through a candidate point,
/// rather than replacing the whole path by naive equidistant interpolation.
fn resample_fence_curve(curve:&GateCurve3,interval:f32)->Result<Vec<[f32;3]>,FenceError> {
    if interval==0.0 || !interval.is_finite(){return Err(FenceError::InvalidCurve);}
    let ratio=(interval/curve.length.max(f32::from_bits(0x322bcc77))).min(1.0);
    let subdivisions=(1.0/ratio).round() as usize;
    let count=subdivisions.checked_add(1+usize::from(subdivisions==0)).ok_or(FenceError::InvalidCurve)?;
    if count<2 || count>1_000_000 {return Err(FenceError::InvalidCurve);}
    let step=1.0/(count as f32-1.0);let limit=-0.5*step+1.0;
    let mut output=vec![curve.positions[0]];let mut cumulative=0.0;let mut emitted=0.0;
    let mut previous_u=0.0;let mut previous=curve.positions[0];
    let mut candidate=previous;let mut candidate_distance=0.0;
    for (&point,&u) in curve.positions.iter().zip(&curve.points_u).skip(1) {
        cumulative=(cumulative+(u-previous_u)).min(limit);
        while step<=cumulative-emitted {
            emitted+=step;let t=(emitted-previous_u)/(u-previous_u);
            let distance=distance3(point,*output.last().unwrap());
            if distance<=candidate_distance {
                if candidate_distance>0.0 {output.push(candidate);candidate_distance=0.0;}
            }else{
                let t=t.max(0.0).min(1.0);
                candidate=std::array::from_fn(|i|previous[i]*(1.0-t)+point[i]*t);
                output.push(candidate);candidate_distance=0.0;
            }
        }
        let distance=distance3(point,*output.last().unwrap());
        if candidate_distance<distance {candidate=point;}
        candidate_distance=candidate_distance.max(distance);previous_u=u;previous=point;
    }
    let end=*curve.positions.last().unwrap();let distance=distance3(end,*output.last().unwrap());
    if distance>0.0 {output.push(end);}else if candidate_distance>distance {
        output.push(candidate);if distance3(candidate,end)<=0.0{return Err(FenceError::InvalidCurve);}output.push(end);
    }
    // The original finishes through try_new_from_points; preserve its errors.
    fence_curve(&output)?;Ok(output)
}
#[derive(Clone, Copy)]
pub struct FenceConstructionInput<'a> {
    pub wall:WallId,pub raw_positions:&'a [[f32;2]],pub prepared_positions:&'a [[f32;3]],
    pub segment_values:&'a [f32],pub observed_public_284:f32,pub observed_public_296:u8,
}
/// Raw/public/prepared sources -> every actual threshold segment -> 64B boards.
/// Original stats count adjacent post pairs and sum (WallId + begin + end).
pub fn construct_fences_cpu(input:FenceConstructionInput<'_>)->Result<FenceCpuOutput,FenceError> {
    if input.observed_public_296!=1 {return Ok(FenceCpuOutput::default());}
    if input.prepared_positions.len()!=input.segment_values.len(){return Err(FenceError::InvalidInput);}
    if input.prepared_positions.is_empty(){return Ok(FenceCpuOutput::default());}
    let mut low=[f32::MAX;2];let mut high=[-f32::MAX;2];
    for point in input.raw_positions {for i in 0..2 {low[i]=low[i].min(point[i]);high[i]=high[i].max(point[i]);}}
    let dx=high[0]-low[0];let dy=high[1]-low[1];let extent=if dx.is_nan(){dy}else{dy.max(dx)};
    let mut output=FenceCpuOutput::default();let mut begin=0;
    while begin<input.segment_values.len() {
        while begin<input.segment_values.len() && input.segment_values[begin]>0.0 {begin+=1;}
        if begin==input.segment_values.len(){break;}
        let mut end=begin+1;
        while end<input.segment_values.len() && input.segment_values[end]<=0.0 {end+=1;}
        let seed=input.wall.0.wrapping_add(begin as u64).wrapping_add(end as u64);
        let mut segment=process_fence_segment(FenceSegmentInput {
            wall:input.wall,height:input.observed_public_284,positions:&input.prepared_positions[begin..end],
            spacing_extent:extent,seed,skip_first:begin!=0,segment_ordinal:end,segment_count:input.prepared_positions.len(),
        })?;
        output.fence_segments_count+=segment.fence_segments_count;
        output.fence_segments_seed=output.fence_segments_seed.wrapping_add(seed);
        output.source_ranges.push([begin,end]);output.segment_seeds.push(seed);
        output.posts.append(&mut segment.posts);output.instances.append(&mut segment.instances);begin=end;
    }
    Ok(output)
}
