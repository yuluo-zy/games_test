//! Ridge-cap TileInstance producer, recovered from original rectangular roof
//! generator 0x1421afc02..0x1421b062d. The observed entry derives the complete
//! ridge endpoint preamble from the original rectangle and Roof bytes.
use crate::tiles::{TileRng,RoofTileRecord,random_splits,record};

/// Original cap endpoint preamble. Dimensions are the initially expanded top
/// dimensions, BEFORE the gable preamble modifies selected top/bottom sizes.
pub fn cap_endpoints_from_observed(rect:&[u8;24],roof:&[u8;88])->[[f32;3];2] {
    let context=crate::tiles::rectangular_context_from_observed(rect,roof);
    let dims=match crate::tiles::expanded_bounds_observed(roof) {
        crate::tiles::ExpandedRoofBounds::Rectangle{top,..}=>top,_=>panic!("rectangle roof required"),
    };
    let top=context.top_rect.0;
    let half=if dims[1]>dims[0] {[(f32::from_bits(top[1].to_bits()^0x80000000)*dims[1])*0.5,(top[0]*dims[1])*0.5]}
        else {[(top[0]*dims[0])*0.5,(top[1]*dims[0])*0.5]};
    let height=crate::surface::height(roof);let zero=0.0*height;
    std::array::from_fn(|i| {
        let local=if i==0 {[top[2]-half[0],top[3]-half[1]]} else {[top[2]+half[0],top[3]+half[1]]};
        let x=context.world_center_xz[0]+(local[1]*context.basis[2]+local[0]*context.basis[0]);
        let z=context.world_center_xz[1]+(local[1]*context.basis[3]+local[0]*context.basis[1]);
        [zero+x,height+0.0,z+zero]
    })
}

pub fn assemble_caps_from_observed(rect:&[u8;24],roof:&[u8;88],special_mode:bool,rng:&mut TileRng,ordinal:&mut u32)->Vec<RoofTileRecord> {
    let dims=crate::ridge::ridge_dims_from_observed(roof);
    if !(dims.0[0]>0.1 || dims.0[1]>0.1) {return Vec::new();}
    let f=|o|f32::from_le_bytes(roof[o..o+4].try_into().unwrap());
    let input=CapInput {endpoints:cap_endpoints_from_observed(rect,roof),height_01:f(0x30),profile_01:f(0x2c),
        is_gable:(f(0x30)>0.1||f(0x30).is_nan())&&f(0x34)==1.0,
        gable_style_byte:roof[0x54],roof_id:u32::from_le_bytes(roof[..4].try_into().unwrap()),special_mode};
    assemble_ridge_caps(&input,rng,ordinal)
}

#[derive(Clone,Copy,Debug)]
pub struct CapInput {
    /// Endpoints before the original is_gable extension step.
    pub endpoints:[[f32;3];2],pub height_01:f32,pub profile_01:f32,
    pub is_gable:bool,pub gable_style_byte:u8,pub roof_id:u32,pub special_mode:bool,
}

pub fn assemble_ridge_caps(input:&CapInput,rng:&mut TileRng,ordinal:&mut u32)->Vec<RoofTileRecord> {
    let [mut a,mut b]=input.endpoints;
    if input.is_gable {
        let d=[b[0]-a[0],b[1]-a[1],b[2]-a[2]];
        let inv=1.0/(d[2]*d[2]+(d[1]*d[1]+d[0]*d[0])).sqrt();
        let ext=std::array::from_fn::<_,3,_>(|i| {
            let value=if input.gable_style_byte==1 {d[i]*inv*0.56} else {inv*d[i]*0.56*(-0.5)};
            value
        });
        for i in 0..3 {a[i]-=ext[i];b[i]+=ext[i];}
    }
    let raw=[b[0]-a[0],b[1]-a[1],b[2]-a[2]];
    let length=(raw[2]*raw[2]+(raw[1]*raw[1]+raw[0]*raw[0])).sqrt();
    let boundaries_count=(length/0.625).round().max(2.0) as usize;
    let splits=random_splits(boundaries_count,0.125/length,rng);
    let inv=1.0/length;let d=[raw[0]*inv,raw[1]*inv,raw[2]*inv];
    let cross=[d[1]*0.0-d[2],d[2]*0.0-d[0]*0.0,d[0]-d[1]*0.0];
    let ci=1.0/((cross[2]*cross[2]+cross[0]*cross[0])+cross[1]*cross[1]).sqrt();
    let nx=cross[2]*ci;let c=d[1]-cross[0]*ci;
    let minus=0.5/(1.0-c).sqrt();let plus=0.5/(c+1.0).sqrt();
    let sum=cross[1]*ci+d[0];
    let q=if c>0.0 {[sum*plus,(c+1.0)*plus,plus*(1.0+d[2]),plus*(0.0-nx)]} else {[(1.0-c)*minus,sum*minus,minus*(nx+0.0),minus*(d[2]+(-1.0))]};
    let blend=input.profile_01*input.height_01;let complement=1.0-blend;
    let t=(((-0.8)+complement)/f32::from_bits(0x3e4ccccc)).max(0.0).min(1.0);
    let smooth=(t*t)*(3.0-(t+t));
    let width=smooth*0.15+(1.0-smooth)*0.0+0.4;
    let down=input.height_01*0.08+(1.0-input.height_01)*0.02
        +(blend*0.15+complement*0.1);
    let zero=0.0*down;
    let mut out=Vec::with_capacity(splits.len()-1);
    for (i,pair) in splits.windows(2).enumerate() {
        let u=pair[0];let v=pair[1];let seg=(v-u)*length;let half=seg*0.5;let other=1.0-u;
        let pos=[half*d[0]+(u*b[0]+other*a[0])-zero,
            half*d[1]+(u*b[1]+other*a[1])-down,
            d[2]*half+(b[2]*u+a[2]*other)-zero];
        let mode=if i==0 || i==splits.len()-2 || input.special_mode {2} else {1};
        let mut tile=record(pos,[width,seg*1.4,0.15],q,other*0.9+u,input.roof_id,*ordinal,mode);
        // Cap records use a distinct packed tag from ordinary surface tiles.
        tile.0[56..60].copy_from_slice(&0u32.to_le_bytes());
        out.push(tile);*ordinal=ordinal.wrapping_add(1);
    }
    out
}
