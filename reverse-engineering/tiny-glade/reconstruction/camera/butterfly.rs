use glam::{Quat,Vec3};
use dolly::{rig::CameraRig,drivers::{Position,Rotation,Smooth,Arm,LookAt}};
use super::{camera_body::CameraBody,schema::*};
#[derive(Debug)]
pub struct ButterflyCamera{pub rig:CameraRig,pub selected_index:u32,pub applied_index:u32}
impl ButterflyCamera{
 pub fn new()->Self{Self{rig:CameraRig::builder().with(Position::new(Vec3::ZERO)).with(Rotation::new(Quat::IDENTITY)).with(Smooth::new_position(2.).predictive(true)).with(Arm::new(Vec3::new(0.,1.5,3.5))).with(Smooth::new_position(4.)).with(LookAt::new(Vec3::ZERO).tracking_smoothness(4.)).build(),selected_index:0,applied_index:u32::MAX}}
 /// Original resolved Query<Transform, With<Butterfly>> updates these three drivers;
 /// target is transform position plus world +Y, not a fabricated animal spawn.
 pub fn follow(&mut self,t:CameraTransform){self.rig.driver_mut::<Position>().position=t.position.into();self.rig.driver_mut::<Rotation>().rotation=t.rotation.into();self.rig.driver_mut::<LookAt>().target=(t.position+Vec3::Y).into();}
 pub fn advance(&mut self,zoom:f32){if zoom!=0.{let sign=if zoom.is_sign_negative(){-1i32}else{1};self.selected_index=(self.selected_index as i32).wrapping_sub(sign).max(0)as u32;}}
}
impl Default for ButterflyCamera{fn default()->Self{Self::new()}}
impl CameraBody for ButterflyCamera{
 fn rotate_yaw_pitch(&mut self,_:f32,_:f32){}
 fn update(&mut self,dt:f32)->CameraTransform{let dt=if self.selected_index!=self.applied_index{self.applied_index=self.selected_index;1000.}else{dt};let t=self.rig.update(dt);CameraTransform{position:t.position.into(),rotation:t.rotation.into()}}
}
