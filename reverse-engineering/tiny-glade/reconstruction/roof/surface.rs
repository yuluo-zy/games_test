//! Recovered from Tiny Glade's matched RNE/PDB, module 498.
//! These functions preserve observed f32 evaluation order. Raw fields retain
//! offsets until the original type layout is independently recovered.
#[cfg(not(feature="library"))]
use std::io::{self, BufRead};

fn f(b: &[u8], o: usize) -> f32 { f32::from_le_bytes(b[o..o+4].try_into().unwrap()) }
fn put(b: &mut [u8], o: usize, v: f32) { b[o..o+4].copy_from_slice(&v.to_le_bytes()); }

/// Observed parameter block, copied by Roof::new to bytes 0x24..0x40.
/// Geometry consumers establish profile/height/eave/ridge roles; spelling is
/// aligned with shipped history, while Width/Length -> bit remains unverified.
#[derive(Debug,Clone,Copy)]
pub struct RoofShapeParams {
    pub tip_offset_01:[f32;2],pub profile_01:f32,pub height_01:f32,
    pub ridge_length_01:f32,pub eave_length_01:f32,pub ridge_dir_bit:u32,
}
impl RoofShapeParams {
    pub fn write_observed(&self,roof:&mut [u8;88]) {
        for (o,v) in [(0x24,self.tip_offset_01[0]),(0x28,self.tip_offset_01[1]),
            (0x2c,self.profile_01),(0x30,self.height_01),(0x34,self.ridge_length_01),(0x38,self.eave_length_01)] {put(roof,o,v);}
        roof[0x3c..0x40].copy_from_slice(&self.ridge_dir_bit.to_le_bytes());
    }
}

/// Exact observed section representation; frame words preserve source data
/// whose interpretation is not yet fully established.
#[derive(Debug,Clone,Copy)]
pub enum RoofSection {
    Circle {frame:[f32;2],radius:f32,word_0x18:u32},
    Rectangle {frame:[f32;4],dimensions:[f32;2]},
}
pub fn section(roof:&[u8;88],t:f32)->RoofSection {
    let mut out=[0;28];get_roof_shape(roof,t,&mut out);
    if out[0]==1 {RoofSection::Rectangle {frame:[f(&out,4),f(&out,8),f(&out,12),f(&out,16)],dimensions:[f(&out,20),f(&out,24)]}}
    else {RoofSection::Circle {frame:[f(&out,4),f(&out,8)],radius:f(&out,12),word_0x18:u32::from_le_bytes(out[16..20].try_into().unwrap())}}
}

/// Profile exponent used by both the profile sampler and its inverse.
pub fn profile_exponent(profile: f32) -> f32 {
    1.5 * profile + (1.0 - profile) * 0.5
}

/// Exact observed section producer. `t` is the normalized profile coordinate.
/// Rectangle outputs 28 bytes; circle outputs 20, preserving the last 8 bytes.
pub fn get_roof_shape(roof: &[u8; 88], t: f32, out: &mut [u8; 28]) {
    let profile = f(roof, 0x2c);
    let blend = t.powf(1.0 / profile_exponent(profile));
    let eave = f(roof, 0x38);
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) == 1 {
        out[..4].copy_from_slice(&1u32.to_le_bytes());
        out[4..20].copy_from_slice(&roof[0x0c..0x1c]);
        let expanded = (1.5 * eave + (1.0-eave)*0.0)
            * (profile.sqrt() + (1.0-profile.sqrt())*0.5);
        let ridge = f(roof, 0x34);
        let inset = (1.0-ridge)*0.1;
        let dx = f(roof,0x1c); let dz = f(roof,0x20);
        let bottom_x = expanded + (0.28 + dx);
        let bottom_z = expanded + (0.28 + dz);
        let top_x = dx*ridge + inset;
        let top_z = dz*ridge + inset;
        let switched = roof[0x3c]&1 != 0;
        put(out,20,blend*(if switched {0.1} else {top_x})+(1.0-blend)*bottom_x);
        put(out,24,blend*(if switched {top_z} else {0.1})+(1.0-blend)*bottom_z);
    } else {
        out[..4].copy_from_slice(&0u32.to_le_bytes());
        out[4..12].copy_from_slice(&roof[0x0c..0x14]);
        let expanded = ((profile.sqrt()+(1.0-profile.sqrt())*0.5)
            *(1.5*eave+(1.0-eave)*0.0)*1.2+0.28)*0.5;
        put(out,12,blend*0.2+(1.0-blend)*(expanded+f(roof,0x14)));
        out[16..20].copy_from_slice(&roof[0x18..0x1c]);
    }
}

/// RoofShapeParams::eave_length_ws returns a scalar in XMM0.
pub fn eave_length_ws(profile: f32, eave: f32) -> f32 {
    (profile.sqrt()+(1.0-profile.sqrt())*0.5)
        *(1.5*eave+(1.0-eave)*0.0)
}

fn sse_max(a:f32,b:f32)->f32 { if a > b {a} else {b} }
fn sse_min(a:f32,b:f32)->f32 { if a < b {a} else {b} }

/// Height rule: size-dependent smoothstep cap (8/9 -> 12) and 0.4 -> 12
/// interpolation by height_01. Panic cases match the observed assertion.
pub fn height(roof:&[u8;88])->f32 {
    let h=f(roof,0x30);
    if !h.is_nan() && h<=0.1 {return 0.0;}
    let (size,base) = if roof[8]&1 == 0 {
        ((f(roof,0x14)+(-0.90000004))/7.1,8.0)
    } else {
        let x=f(roof,0x1c); let z=f(roof,0x20);
        let greatest=if x.is_nan() {z} else {sse_max(z,x)};
        ((greatest+(-2.25))/12.75,9.0)
    };
    let u=sse_min(1.0,sse_max(0.0,size/0.4));
    let smooth=(3.0-(u+u))*(u*u);
    let cap=smooth*12.0+(1.0-smooth)*base;
    assert!(cap>=0.0,"original height cap assertion");
    let wanted=h*12.0+(1.0-h)*0.4;
    sse_min(cap,sse_max(0.0,wanted))
}

/// Twenty exact original normalized sample coordinates (f32 constants).
pub fn profile_curve_normalized(profile:f32)->[[f32;2];20] {
    const U:[u32;20]=[0,0x3d579436,0x3dd79436,0x3e21af28,0x3e579436,
        0x3e86bca2,0x3ea1af28,0x3ebca1af,0x3ed79436,0x3ef286bd,
        0x3f06bca2,0x3f1435e5,0x3f21af28,0x3f2f286c,0x3f3ca1af,
        0x3f4a1af3,0x3f579436,0x3f650d79,0x3f7286bd,0x3f800000];
    let exp=profile_exponent(profile);
    std::array::from_fn(|i| {let u=f32::from_bits(U[i]);[u,if i==19 {1.0} else {u.powf(exp)}]})
}

/// The observed point transform inside profile_curve_ws, before CurveU
/// packing, cumulative distance computation and its validity checks.
pub fn profile_curve_ws_points(points:&[[f32;2]],bottom:f32,top:f32,height:f32)->Vec<[f32;2]> {
    points.iter().map(|p|[p[0]*top+(1.0-p[0])*bottom,p[1]*height]).collect()
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurveError { TooFewPoints(u8), NonFiniteLength, DuplicateSegment(usize) }

/// Recovered Curve2 ownership-independent representation. Original backing
/// allocations and pointer identity are intentionally not part of this API.
#[derive(Debug, Clone)]
pub struct Curve2 {pub points:Vec<[f32;2]>,pub points_u:Vec<f32>,pub length:f32}
impl Curve2 {
    pub fn try_new(points:Vec<[f32;2]>,reject_duplicates:bool)->Result<Self,CurveError> {
        if points.len()<2 {return Err(CurveError::TooFewPoints(points.len() as u8));}
        let mut distance=0.0;let mut previous=points[0];let mut points_u=Vec::with_capacity(points.len());
        for point in &points {
            let dx=previous[0]-point[0];let dy=previous[1]-point[1];
            distance += (dy*dy+dx*dx).sqrt();points_u.push(distance);previous=*point;
        }
        if distance==0.0 {return Err(CurveError::NonFiniteLength);}
        for u in &mut points_u {
            *u /= distance;
            if u.to_bits()&0x7fffffff>0x7f7fffff {return Err(CurveError::NonFiniteLength);}
        }
        if reject_duplicates {
            for i in 0..points.len()-1 {
                if points[i][0]==points[i+1][0] && points[i][1]==points[i+1][1] {return Err(CurveError::DuplicateSegment(i));}
            }
        }
        Ok(Self{points,points_u,length:distance})
    }
    /// Original clamping, right-biased binary search, and inverse lerp.
    pub fn coord_at_u(&self,u:f32)->(usize,f32) {
        assert!(u.is_finite(),"original CurveU finite assertion");
        if u>=1.0 {return (self.points.len()-2,1.0);}
        if u<=0.0 {return (0,0.0);}
        let mut index=0;let mut remaining=self.points_u.len();
        while remaining>1 {let half=remaining>>1;let probe=index+half;if self.points_u[probe]<=u {index=probe;}remaining-=half;}
        let next=index+usize::from(u>=self.points_u[index]);assert!(next>0 && next<self.points_u.len());
        let segment=next-1;let a=self.points_u[segment];let b=self.points_u[next];assert!(a!=b);
        (segment,sse_min(1.0,sse_max(0.0,(u-a)/(b-a))))
    }
    pub fn pos_at_u(&self,u:f32)->[f32;2] {
        let (i,t)=self.coord_at_u(u);let a=self.points[i];let b=self.points[i+1];
        [b[0]*t+a[0]*(1.0-t),b[1]*t+a[1]*(1.0-t)]
    }
    pub fn tangent_at_u(&self,u:f32)->[f32;2] {
        let(i,_)=self.coord_at_u(u);let a=self.points[i];let b=self.points[i+1];
        let dx=b[0]-a[0];let dy=b[1]-a[1];let inv=1.0/(dy*dy+dx*dx).sqrt();[dx*inv,dy*inv]
    }
}

pub fn normalized_profile_curve(profile:f32)->Result<Curve2,CurveError> {
    Curve2::try_new(profile_curve_normalized(profile).to_vec(),false)
}
pub fn world_profile_curve(profile:f32,bottom:f32,top:f32,height:f32)->Result<Curve2,CurveError> {
    let normalized=normalized_profile_curve(profile)?;
    Curve2::try_new(profile_curve_ws_points(&normalized.points,bottom,top,height),false)
}

#[cfg(not(feature="library"))]
fn main(){
    for line in io::stdin().lock().lines(){
        let line=line.unwrap();let words:Vec<_>=line.split_whitespace().collect();
        let decode=|s:&str| -> Vec<u8> {let s=if s=="-" {""} else {s};(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()};
        match words[0] {
          "section"=>{let roof:[u8;88]=decode(words[1]).try_into().unwrap();let t=f32::from_bits(u32::from_str_radix(words[2],16).unwrap());let mut out=[0xa5;28];get_roof_shape(&roof,t,&mut out);for b in out {print!("{b:02x}");}println!();},
          "height"=>{let roof:[u8;88]=decode(words[1]).try_into().unwrap();println!("{:08x}",height(&roof).to_bits());},
          "eave"=>{let p=f32::from_bits(u32::from_str_radix(words[1],16).unwrap());let e=f32::from_bits(u32::from_str_radix(words[2],16).unwrap());println!("{:08x}",eave_length_ws(p,e).to_bits());},
          "profile"=>{let p=f32::from_bits(u32::from_str_radix(words[1],16).unwrap());let points=profile_curve_normalized(p);for point in points {for v in point {print!("{:08x}",v.to_bits());}}println!();},
          "transform"=>{let bottom=f32::from_bits(u32::from_str_radix(words[1],16).unwrap());let top=f32::from_bits(u32::from_str_radix(words[2],16).unwrap());let h=f32::from_bits(u32::from_str_radix(words[3],16).unwrap());let raw=decode(words[4]);let pts:Vec<_>=raw.chunks_exact(8).map(|r|[f(r,0),f(r,4)]).collect();for point in profile_curve_ws_points(&pts,bottom,top,h) {for v in point {for b in v.to_le_bytes() {print!("{b:02x}");}}}println!();},
          "curve"=>{let raw=decode(words[2]);let pts:Vec<_>=raw.chunks_exact(8).map(|r|[f(r,0),f(r,4)]).collect();match Curve2::try_new(pts,words[1]=="1") {Ok(c)=>{print!("ok {:08x} ",c.length.to_bits());for u in c.points_u {print!("{:08x}",u.to_bits());}println!();},Err(e)=>println!("error {e:?}")}},
          "sample"=>{let raw=decode(words[2]);let pts:Vec<_>=raw.chunks_exact(8).map(|r|[f(r,0),f(r,4)]).collect();let c=Curve2::try_new(pts,false).unwrap();let u=f32::from_bits(u32::from_str_radix(words[1],16).unwrap());let (i,t)=c.coord_at_u(u);let p=c.pos_at_u(u);let tangent=c.tangent_at_u(u);println!("{i} {:08x} {:08x} {:08x} {:08x} {:08x}",t.to_bits(),p[0].to_bits(),p[1].to_bits(),tangent[0].to_bits(),tangent[1].to_bits());},
          "chain"=>{let val=|i:usize|f32::from_bits(u32::from_str_radix(words[i],16).unwrap());let c=world_profile_curve(val(1),val(2),val(3),val(4)).unwrap();print!("{:08x} ",c.length.to_bits());for p in c.points {for v in p {print!("{:08x}",v.to_bits());}}print!(" ");for u in c.points_u {print!("{:08x}",u.to_bits());}println!();},
          _=>panic!("unknown operation")
        }
    }
}
