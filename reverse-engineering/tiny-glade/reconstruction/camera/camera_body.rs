use glam::{Vec2,Vec3};
use super::schema::CameraTransform;
/// Resolved contracts, not substitute terrain or collision implementations.
pub trait TerrainQueries {
 fn sample_height(&mut self,xz:Vec2)->f32;
 /// Original walk blend samples the selected RasterBufferRef<512> at world XZ,
 /// world size 130x130. The actual producer owns encoded values and grid choice.
 fn walk_height_blend(&mut self,xz:Vec2)->f32;
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CastRequest{pub origin:Vec3,pub direction:Vec3,pub radius:f32,pub max_toi:f32,pub filter:u32}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CastHit{pub toi:f32,pub normal:Vec3,pub observed_collision_kind:u8}
pub trait CollisionQueries{fn spherecast(&mut self,request:CastRequest)->Option<CastHit>;fn raycast(&mut self,origin:Vec3,direction:Vec3,max_toi:f32,filter:u32)->Option<Vec3>;}
#[derive(Clone,Copy,Debug)]
pub struct MotionContext{pub matrices_transform:CameraTransform,pub demo_mode:bool}
pub trait CameraBody{
 fn rotate_yaw_pitch(&mut self,yaw:f32,pitch:f32);
 fn update(&mut self,dt:f32)->CameraTransform;
}
pub(crate) fn lerp(a:f32,b:f32,t:f32)->f32{(b-a)*t+a}
pub(crate) fn smooth(dt:f32)->f32{1.0-(dt * -8.0).exp()}
/// B f32 min/max sanitizes only at the evidenced branch, no global sanitization.
pub(crate) fn bounds_xz(p:Vec2,extent:f32,demo:bool)->Vec2{
 if demo {let d=p.length_squared();if extent*extent*4.<d{p/p.length()*(extent*2.)}else{p}}
 else{let c=Vec2::new(p.x.max(-extent).min(extent),p.y.max(-extent).min(extent));let d=p-c;if extent*extent<d.length_squared(){c+d*(extent/d.length())}else{p}}
}
