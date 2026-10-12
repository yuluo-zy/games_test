use glam::{Vec2,Vec3};
use dolly::{rig::CameraRig,drivers::{Position,YawPitch,Smooth}};
use super::{camera_body::*,schema::*};
#[derive(Debug)]
pub struct FreeCamera{pub rig:CameraRig}
impl FreeCamera{
 pub fn new()->Self{Self::from_position_angles(Vec3::Y,0.,0.)}
 pub fn from_position_angles(position:Vec3,yaw:f32,pitch:f32)->Self{Self{rig:CameraRig::builder().with(Position::new(position)).with(YawPitch::new().yaw_degrees(yaw).pitch_degrees(pitch)).with(Smooth::new_position_rotation(1.,1.)).build()}}
 pub fn set(&mut self,position:Vec3,yaw:f32,pitch:f32){*self=Self::from_position_angles(position,yaw,pitch)}
 pub fn get_position(&self)->Vec3{self.rig.driver::<Position>().position.into()}
 pub fn get_yaw_pitch(&self)->Vec2{let p=self.rig.driver::<YawPitch>();Vec2::new(p.yaw_degrees,p.pitch_degrees)}
 pub fn transform(&self)->CameraTransform{CameraTransform{position:self.rig.final_transform.position.into(),rotation:self.rig.final_transform.rotation.into()}}
 pub fn translate_screen_space(&mut self,movement:Vec3,dt:f32,terrain:&mut dyn TerrainQueries){
 let p=self.get_position();let target=p+self.transform().rotation*movement;let a=smooth(dt);let x=target.x.max(-91.).min(91.);let z=target.z.max(-91.).min(91.);let height=terrain.sample_height(Vec2::new(x,z)).max(0.)+0.75;let y=lerp(target.y,target.y.max(0.).min(91.),a).max(height);
 let delta=Vec3::new((lerp(target.x,x,a)-p.x)*20./60.,(y-p.y)*20./60.,(lerp(target.z,z,a)-p.z)*20./60.);
 self.rig.driver_mut::<Position>().translate(delta);
 }
}
impl Default for FreeCamera{fn default()->Self{Self::new()}}
impl CameraBody for FreeCamera{
 fn rotate_yaw_pitch(&mut self,yaw:f32,pitch:f32){self.rig.driver_mut::<YawPitch>().rotate_yaw_pitch(yaw*0.5,pitch*0.5)}
 fn update(&mut self,dt:f32)->CameraTransform{let t=self.rig.update(dt);CameraTransform{position:t.position.into(),rotation:t.rotation.into()}}
}
