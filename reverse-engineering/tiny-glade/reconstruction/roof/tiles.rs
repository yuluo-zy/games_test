//! Original Tiny Glade roof tile layout helpers recovered from RNE/PDB.
//! No original Rust ABI is asserted; these are explicit numerical APIs.

#[derive(Clone, Copy, Debug)]
pub struct TileRng(pub u64);

impl TileRng {
    /// Reuse the exact dependency version recorded by the original Cargo.lock.
    /// The explicit state wrapper keeps the observed game interface stable.
    pub fn next_f32(&mut self) -> f32 {
        let rng = fastrand::Rng::with_seed(self.0);
        let value = rng.f32();
        self.0 = rng.get_seed();
        value
    }
}

/// Actual jittered partition used by both rectangular and circular roof tiles.
/// `count` is the number of boundaries, not tiles; output includes 0 and 1.
/// Original `utils::random_splits_into` at 0x140c90050. Appending/reservation ABI
/// is intentionally replaced by an owned Vec; numerical results/state retained.
pub fn random_splits(count: usize, amplitude: f32, rng: &mut TileRng) -> Vec<f32> {
    assert!(count >= 2);
    if count == 2 { return vec![0.0, 1.0]; }
    let cap = (1.0 / ((count - 1) as f32)) * f32::from_bits(0x3efd70a4);
    let amp = if amplitude.is_nan() || amplitude > cap { cap } else { amplitude };
    let step = 1.0 / ((count as f32) - 1.0);
    let mut boundaries = Vec::with_capacity(count);
    boundaries.push(0.0);
    for index in 1..count - 1 {
        let jitter = (rng.next_f32() - 0.5) * amp;
        boundaries.push(jitter + (index as f32) * step);
    }
    boundaries.push(1.0);
    boundaries
}

#[derive(Clone, Copy, Debug)]
pub struct CircularRowPlan {
    pub row_count: i32,
    pub tile_height: f32,
    pub denominator: f32,
}

/// The original row-count/tile-height block at 0x1421b3201..0x1421b3240.
/// Input is the world-space profile curve's cumulative length.
pub fn circular_row_plan(profile_length: f32) -> CircularRowPlan {
    let rounded = (profile_length / 0.4375).ceil();
    // MAXSS with 2.0 as the source selects 2.0 on NaN.
    let count = if rounded.is_nan() || rounded < 2.0 { 2.0 } else { rounded };
    CircularRowPlan {
        row_count: count as i32,
        tile_height: (profile_length / count) * f32::from_bits(0x3fb6db6e),
        denominator: count - 1.0,
    }
}

/// Rows emitted by the original decreasing negative counter; final u=1 is
/// deliberately omitted. This is not an off-by-one fix: original control flow
/// at 0x1421b3297..0x1421b32d9 produces 0..N-2 inclusive.
pub fn circular_row_coordinates(plan: CircularRowPlan) -> Vec<f32> {
    (0..plan.row_count.saturating_sub(1)).map(|i| (i as f32) / plan.denominator).collect()
}

/// Original circumferential tile-count block. `perimeter` is Circle2d::perimeter
/// for the interpolated radius. Small radii reduce the nominal tile width.
pub fn circular_column_count(radius: f32, perimeter: f32) -> i32 {
    let t = (radius - f32::from_bits(0x3f666666)) /
        (f32::from_bits(0x3e99999a) - f32::from_bits(0x3f666666));
    let clipped = if t.is_nan() || t < 0.0 { 0.0 } else { t };
    let clipped = if clipped > 1.0 { 1.0 } else { clipped };
    let scale = clipped * 0.5 + (1.0 - clipped);
    let columns = (perimeter / (scale * 0.5)).ceil();
    let columns = if columns.is_nan() || columns < 5.0 { 5.0 } else { columns };
    columns as i32
}

/// The original multiplies a zero by scale then divides by circumference.
/// Preserve that expression for the original zero-radius NaN -> amplitude-cap
/// behavior of random_splits, rather than replacing it blindly with zero.
pub fn circular_boundaries(column_count: i32, radius: f32, perimeter: f32, rng: &mut TileRng) -> Vec<f32> {
    let t = (radius - f32::from_bits(0x3f666666)) /
        (f32::from_bits(0x3e99999a) - f32::from_bits(0x3f666666));
    let clipped = if t.is_nan() || t < 0.0 { 0.0 } else { t };
    let clipped = if clipped > 1.0 { 1.0 } else { clipped };
    let scale = clipped * 0.5 + (1.0 - clipped);
    random_splits((column_count as usize) + 1, (scale * 0.0) / perimeter, rng)
}

#[derive(Clone, Debug)]
pub struct CircularTileInput<'a> {
    pub profile_points: &'a [[f32; 2]],
    pub profile_u: &'a [f32],
    pub profile_length: f32,
    pub center_xz: [f32; 2],
    pub roof_radius: f32,
    pub tip_offset_normalized: [f32; 2],
    pub circle_rotation: f32,
    pub roof_id: u32,
    pub special_mode: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct RoofTileRecord(pub [u8; 64]);

pub fn half(v: f32) -> u16 {
    ::half::f16::from_f32(v).to_bits()
}

pub fn record(pos: [f32; 3], dims: [f32; 3], quat: [f32; 4], u: f32, roof: u32, ordinal: u32, mode: u32) -> RoofTileRecord {
    let mut b = [0u8;64];
    for (i,v) in pos.iter().enumerate() { b[i*4..i*4+4].copy_from_slice(&v.to_bits().to_le_bytes()); }
    b[16..24].copy_from_slice(&0x7bff00003c000000u64.to_le_bytes());
    for (i,v) in dims.iter().enumerate() { b[24+i*2..26+i*2].copy_from_slice(&half(*v).to_le_bytes()); }
    b[30..32].copy_from_slice(&half(u).to_le_bytes());
    for (i,v) in quat.iter().enumerate() { b[32+i*2..34+i*2].copy_from_slice(&half(*v).to_le_bytes()); }
    b[40..44].copy_from_slice(&roof.to_le_bytes()); b[52..56].copy_from_slice(&ordinal.to_le_bytes());
    b[56..60].copy_from_slice(&1u32.to_le_bytes()); b[60..64].copy_from_slice(&mode.to_le_bytes());
    RoofTileRecord(b)
}

fn curve_coord(us: &[f32], u: f32) -> (usize, f32) {
    if u <= 0.0 { return (0, 0.0); }
    if u >= 1.0 { return (us.len()-2, 1.0); }
    let mut index = 0;
    while index+1 < us.len() && us[index+1] <= u { index += 1; }
    (index, ((u-us[index])/(us[index+1]-us[index])).clamp(0.0,1.0))
}

fn circle_unit(rotation: f32, u: f32) -> [f32;2] {
    let tau=f32::from_bits(0x40c90fdb); let rem=rotation%tau;
    let rem=if rem < 0.0 { tau+rem } else { rem };
    let theta=rem+(1.0-u)*tau;
    [theta.cos(),theta.sin()]
}
fn rotate(v:[f32;2], cs:f32, sn:f32)->[f32;2] { [v[0]*cs+v[1]*(-sn),v[0]*sn+v[1]*cs] }

pub fn matrix_quat(a:[f32;3],b:[f32;3],c:[f32;3])->[f32;4] {
    let matrix = glam::Mat3::from_cols(glam::Vec3::from_array(a), glam::Vec3::from_array(b), glam::Vec3::from_array(c));
    glam::Quat::from_mat3(&matrix).to_array()
}

/// Circular roof tile-instance generation from a prepared world-space profile.
/// Matches the original body after `profile_curve_ws` returns. Profile curve
/// construction and ECS allocation are separate APIs, not invented substitutes.
pub fn assemble_circular_tiles(input: &CircularTileInput<'_>) -> Vec<RoofTileRecord> {
    assert!(input.profile_points.len()>=2 && input.profile_points.len()==input.profile_u.len());
    let plan=circular_row_plan(input.profile_length); let mut rng=TileRng(0);let mut output=Vec::new();let mut ordinal=0u32;
    let tip=[(input.roof_radius+input.roof_radius)*input.tip_offset_normalized[0]*0.5,(input.roof_radius+input.roof_radius)*input.tip_offset_normalized[1]*0.5];
    for u in circular_row_coordinates(plan) {
        let (i,t)=curve_coord(input.profile_u,u); let p=input.profile_points[i];let q=input.profile_points[i+1];
        let radius=(1.0-t)*p[0]+q[0]*t; let height=(1.0-t)*p[1]+q[1]*t;
        let origin=[tip[0]*u+input.center_xz[0],height,tip[1]*u+input.center_xz[1]];
        let perimeter=radius*f32::from_bits(0x40c90fdb);let n=circular_column_count(radius,perimeter);
        let phase=rng.next_f32()*f32::from_bits(0x40c90fdb);let sn=phase.sin();let cs=phase.cos();
        let splits=circular_boundaries(n,radius,perimeter,&mut rng);
        let dr=q[0]-p[0];let dh=q[1]-p[1];let inv=1.0/(dh*dh+dr*dr).sqrt();let slope=((inv*dh)/(dr*inv)).atan();
        for pair in splits.windows(2) {
            let p0=circle_unit(input.circle_rotation,pair[0]);let p1=circle_unit(input.circle_rotation,pair[1]);
            let p0=rotate([p0[0]*radius+0.0,p0[1]*radius+0.0],cs,sn);let p1=rotate([p1[0]*radius+0.0,p1[1]*radius+0.0],cs,sn);
            let mid=[(p0[0]+p1[0])*0.5,(p0[1]+p1[1])*0.5];let pos=[origin[0]+mid[0],origin[1]+0.0,origin[2]+mid[1]];
            let random=rng.next_f32();let tile_h=((1.0-random)*0.8+random*1.2)*plan.tile_height;
            let mid_u=(pair[0]+pair[1])*0.5;let normal=rotate(circle_unit(input.circle_rotation,mid_u),cs,sn);
            let (nx,ny,nz)=(normal[0],0.0,normal[1]);
            let axis=[ny*0.0-nz,nz*0.0-nx*0.0,nx-ny*0.0];
            let angle=(f32::from_bits(0xbe32b8c2)-slope)*(-1.0);
            let rotation=glam::Quat::from_axis_angle(glam::Vec3::from_array(axis),angle);
            let dir=(rotation*glam::Vec3::new(nx,ny,nz)).to_array();
            let c=[axis[1]*dir[2]-axis[2]*dir[1],axis[2]*dir[0]-dir[2]*axis[0],dir[1]*axis[0]-axis[1]*dir[0]];
            let quat=matrix_quat(axis,dir,c);let dx=p0[0]-p1[0];let dz=p0[1]-p1[1];let width=(dz*dz+dx*dx).sqrt();
            let jitter=(rng.next_f32()*0.2+u).clamp(0.0,1.0);
            output.push(record(pos,[width,tile_h,0.1],quat,jitter,input.roof_id,ordinal,if u==0.0||input.special_mode {2}else{1}));
            if u==0.0 {
                let a=[pos[0]+tile_h*dir[0]*0.5,pos[1]+tile_h*dir[1]*0.5,pos[2]+tile_h*dir[2]*0.5];
                let b=[pos[0]-tile_h*dir[0]*0.5,a[1],pos[2]-tile_h*dir[2]*0.5];
                let dx=b[0]-a[0];let dy=b[1]-a[1];let dz=b[2]-a[2];let len=(dz*dz+(dy*dy+dx*dx)).sqrt();
                if len>0.1 {
                    // Original horizontal eave filler frame, distinct from the pitched tile.
                    let filler_dir=[nx,0.0,nz];
                    let cross=[axis[1]*nz-axis[2]*0.0,axis[2]*nx-nz*axis[0],0.0*axis[0]-axis[1]*nx];
                    let q=matrix_quat(axis,filler_dir,cross);
                    output.push(record([(a[0]*0.5)+(b[0]*0.5),(a[1]*0.5)+(b[1]*0.5),(a[2]*0.5)+(b[2]*0.5)],[width,len,0.1],q,0.0,input.roof_id,ordinal,2));
                }
            }
            ordinal+=1;
        }
    }
    output
}

/// Composed original circular pipeline: expanded bounds, normalized profile,
/// world-space Curve2 packing, row/column layout, packed instance generation.
/// Callers provide the observed 88-byte Roof representation (Roof::new output).
pub fn assemble_circular_from_observed(roof: &[u8; 88], special_mode: bool)
    -> Result<Vec<RoofTileRecord>, super::surface::CurveError>
{
    assert!(roof[8] & 1 == 0, "circular roof required");
    let read = |o: usize| f32::from_le_bytes(roof[o..o+4].try_into().unwrap());
    let (bottom,top) = match expanded_bounds_observed(roof) {
        ExpandedRoofBounds::Circle { bottom, top } => (bottom,top),
        _ => unreachable!(),
    };
    let curve = super::surface::world_profile_curve(read(0x2c), bottom, top, super::surface::height(roof))?;
    Ok(assemble_circular_tiles(&CircularTileInput {
        profile_points: &curve.points, profile_u: &curve.points_u, profile_length: curve.length,
        center_xz: [read(0x0c), read(0x10)], roof_radius: read(0x14),
        tip_offset_normalized: [read(0x24), read(0x28)], circle_rotation: read(0x18),
        roof_id: u32::from_le_bytes(roof[..4].try_into().unwrap()), special_mode,
    }))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExpandedRoofBounds {
    Circle { bottom: f32, top: f32 },
    Rectangle { bottom: [f32; 2], top: [f32; 2] },
}

/// Original get_roof_expanded_top_and_bottom 0x140a880c0, including the
/// distinction between full dimensions of rectangular roofs and circle radii.
pub fn expanded_bounds_observed(roof: &[u8;88]) -> ExpandedRoofBounds {
    let f=|o|f32::from_le_bytes(roof[o..o+4].try_into().unwrap());
    let p=f(0x2c);let e=f(0x38);let root=p.sqrt();
    let expanded=(root+(1.0-root)*0.5)*(1.5*e+(1.0-e)*0.0);
    if roof[8]&1==0 {
        ExpandedRoofBounds::Circle {bottom:(expanded*1.2+0.28)*0.5+f(0x14),top:0.2}
    } else {
        let ridge=f(0x34);let inset=(1.0-ridge)*0.1;
        let top=if roof[0x3c]==0 {[f(0x1c)*ridge+inset,0.1]} else {[0.1,f(0x20)*ridge+inset]};
        ExpandedRoofBounds::Rectangle{bottom:[(0.28+f(0x1c))+expanded,(0.28+f(0x20))+expanded],top}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObservedRectangle(pub [f32;6]);
impl ObservedRectangle {
    pub fn bytes(&self)->[u8;24] {let mut b=[0;24];for (i,x) in self.0.iter().enumerate(){b[4*i..4*i+4].copy_from_slice(&x.to_le_bytes());}b}
    pub fn from_bytes(b:&[u8;24])->Self {Self(std::array::from_fn(|i|f32::from_le_bytes(b[i*4..i*4+4].try_into().unwrap())))}
    pub fn axis_aligned(dims:[f32;2],center:[f32;2])->Self {assert!(dims.iter().all(|x|x.is_finite()));Self([1.0,0.0,center[0],center[1],dims[0],dims[1]])}
    pub fn basis(&self)->[f32;4] {[self.0[0],self.0[1],f32::from_bits(self.0[1].to_bits()^0x80000000),self.0[0]]}
    /// Original Rectangle2d::as_points2 order and f32 operation order.
    pub fn corners(&self)->[[f32;2];4] {
        let [c,s,cx,cy,dx,dy]=self.0;assert!(dx*dy>0.0);
        let ns=f32::from_bits(s.to_bits()^0x80000000);let nx=dx*(-0.5);let ny=dy*(-0.5);let px=dx*0.5;let py=0.5*dy;
        [[nx*c+ny*ns+cx,nx*s+ny*c+cy],[nx*c+py*ns+cx,nx*s+py*c+cy],[px*c+py*ns+cx,px*s+py*c+cy],[px*c+ny*ns+cx,px*s+ny*c+cy]]
    }
}

#[derive(Clone, Debug)]
pub struct RectangularTileContext {
    pub bottom_rect: ObservedRectangle,
    pub top_rect: ObservedRectangle,
    pub bottom_corners: [[f32;2];4],
    pub top_corners: [[f32;2];4],
    pub basis: [f32;4],
    pub local_tip_center: [f32;2],
    pub world_center_xz: [f32;2],
}

/// Original rectangular assembly setup through 0x1421af1ac. Gable roofs modify
/// the selected rectangle dimension on both top and bottom, using +0x54 mode.
pub fn rectangular_context_from_observed(rect:&[u8;24],roof:&[u8;88])->RectangularTileContext {
    let f=|o|f32::from_le_bytes(roof[o..o+4].try_into().unwrap());
    let bounds=expanded_bounds_observed(roof);let (bottom,top)=match bounds {ExpandedRoofBounds::Rectangle{bottom,top}=>(bottom,top),_=>panic!("rectangle roof required")};
    let local_tip_center=[f(0x1c)*f(0x24)*0.0,f(0x20)*f(0x28)*0.0];
    let mut b=ObservedRectangle::axis_aligned(bottom,[0.0,0.0]);let mut t=ObservedRectangle::axis_aligned(top,local_tip_center);
    // CMPNLESS height > 0.1, equality ridge_length==1.0; NaN-height passes.
    let gable=(f(0x30)>0.1||f(0x30).is_nan()) && f(0x34)==1.0;
    if gable {
        let active=roof[0x54]!=0;let base=if active {0.25} else {f32::from_bits(0x3f4f5c29)};
        let mut delta=base+super::surface::eave_length_ws(f(0x2c),f(0x38));
        if !active {delta=f32::from_bits(delta.to_bits()^0x80000000);}
        let axis=if roof[0x3c]==0 {4}else{5};let value=delta+b.0[axis];b.0[axis]=value;t.0[axis]=value;
    }
    let frame=ObservedRectangle::from_bytes(rect);RectangularTileContext{bottom_rect:b,top_rect:t,bottom_corners:b.corners(),top_corners:t.corners(),basis:frame.basis(),local_tip_center,world_center_xz:[frame.0[2],frame.0[3]]}
}
