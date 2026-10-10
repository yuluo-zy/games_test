//! Original rectangular roof's four hip/eave strip loops (0x1421af190..afbfb).
//! Prepared frames come from the original rectangle-bound and transform chain.
use crate::surface::{self, Curve2};
use crate::tiles::{self, TileRng, RoofTileRecord};

#[derive(Clone,Copy,Debug)]
pub struct EdgeFrame {pub origin_xz:[f32;2],pub delta_xz:[f32;2]}

pub fn frames_from_observed(rectangle:&[u8;24],roof:&[u8;88])->[EdgeFrame;4] {
    let ctx=tiles::rectangular_context_from_observed(rectangle,roof);
    let transform=|p:[f32;2]|[ctx.world_center_xz[0]+(p[0]*ctx.basis[0]+p[1]*ctx.basis[2]),
                             ctx.world_center_xz[1]+(p[0]*ctx.basis[1]+p[1]*ctx.basis[3])];
    std::array::from_fn(|i|{let bottom=transform(ctx.bottom_corners[i]);let top=transform(ctx.top_corners[i]);
        EdgeFrame{origin_xz:bottom,delta_xz:[top[0]-bottom[0],top[1]-bottom[1]]}})
}

pub fn assemble_edges_from_observed(rectangle:&[u8;24],roof:&[u8;88],
                                    rng:&mut TileRng,ordinal:&mut u32)->Vec<RoofTileRecord> {
    assemble_rectangular_edges(roof,&frames_from_observed(rectangle,roof),rng,ordinal)
}

pub fn assemble_rectangular_edges(roof:&[u8;88],frames:&[EdgeFrame;4],
                                  rng:&mut TileRng,ordinal:&mut u32)->Vec<RoofTileRecord> {
    let profile=f32::from_le_bytes(roof[0x2c..0x30].try_into().unwrap());
    let curve=surface::normalized_profile_curve(profile).expect("original normalized curve valid");
    let height=surface::height(roof);
    let roof_id=u32::from_le_bytes(roof[0..4].try_into().unwrap());
    let mut result=Vec::new();
    for frame in frames {
        result.extend(assemble_edge(&curve,*frame,height,roof_id,rng,ordinal));
    }
    result
}

pub fn assemble_edge(curve:&Curve2,frame:EdgeFrame,height:f32,roof_id:u32,
                     rng:&mut TileRng,ordinal:&mut u32)->Vec<RoofTileRecord> {
    let [dx,dz]=frame.delta_xz;
    let horizontal=(dz*dz+dx*dx).sqrt();
    let zero=height*0.0;
    let vertical=(height*height+zero*zero+zero*zero).sqrt();
    let world=Curve2::try_new(surface::profile_curve_ws_points(&curve.points,0.0,horizontal,vertical),false).expect("original curve valid");
    let n=(world.length/0.625).round().max(2.0) as usize;
    let splits=tiles::random_splits(n,0.125/world.length,rng);
    let a=[dx,0.0,dz];let b=[zero,height,zero];
    let raw_normal=[height*dz-zero*0.0,zero*dx-dz*zero,zero*0.0-height*dx];
    let inv=1.0/(raw_normal[2]*raw_normal[2]+raw_normal[1]*raw_normal[1]+raw_normal[0]*raw_normal[0]).sqrt();
    let normal=raw_normal.map(|v|v*inv);
    let origin=[frame.origin_xz[0],0.0,frame.origin_xz[1]];
    let epsilon=f32::from_bits(0x33bbbd2e);
    let mut result=Vec::new();
    for us in splits.windows(2) {
        let p=curve.pos_at_u(us[0]);let q=curve.pos_at_u(us[1]);
        let start=std::array::from_fn::<_,3,_>(|i|p[0]*a[i]+p[1]*b[i]+normal[i]*0.0+origin[i]);
        let end=std::array::from_fn::<_,3,_>(|i|q[0]*a[i]+q[1]*b[i]+normal[i]*0.0+origin[i]);
        let diff=[start[0]-end[0],start[1]-end[1],start[2]-end[2]];
        let length=(diff[2]*diff[2]+(diff[1]*diff[1]+diff[0]*diff[0])).sqrt()+0.05;
        let diff=[end[0]-start[0],end[1]-start[1],end[2]-start[2]];
        let inv=1.0/(diff[1]*diff[1]+diff[0]*diff[0]+diff[2]*diff[2]).sqrt();
        let dir=diff.map(|v|inv*v);
        let axis=[dir[1]*normal[2]-dir[2]*normal[1],dir[2]*normal[0]-dir[0]*normal[2],dir[0]*normal[1]-dir[1]*normal[0]];
        let c=[dir[1]*axis[2]-dir[2]*axis[1],dir[2]*axis[0]-dir[0]*axis[2],dir[0]*axis[1]-dir[1]*axis[0]];
        let ma=[axis[0]*0.0+(dir[0]*(-epsilon)-c[0]),axis[1]*0.0+(dir[1]*(-epsilon)-c[1]),(dir[2]*(-epsilon)-c[2])+axis[2]*0.0];
        let mb=[(c[0]*epsilon-dir[0])+axis[0]*0.0,(c[1]*epsilon-dir[1])+axis[1]*0.0,(c[2]*epsilon-dir[2])+axis[2]*0.0];
        let mc=[axis[0]+c[0]*0.0+dir[0]*0.0,dir[1]*0.0+c[1]*0.0+axis[1],dir[2]*0.0+c[2]*0.0+axis[2]];
        let quat=tiles::matrix_quat(ma,mb,mc);
        let pos=std::array::from_fn::<_,3,_>(|i|length*dir[i]*0.3+start[i]+axis[i]*(-0.05));
        let mut record=tiles::record(pos,[0.25,length,0.18],quat,us[0],roof_id,*ordinal,2);
        record.0[56..60].copy_from_slice(&0u32.to_le_bytes());
        result.push(record);*ordinal+=1;
    }
    result
}
