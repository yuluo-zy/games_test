//! B NormalCamera also backs TiltShift and TopDown. Game-owned orbit backlog.
use glam::{EulerRot,Quat,Vec2,Vec3};
use super::{camera_body::*,schema::*};
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct ActiveRotation{pub arm_length:f32,pub target_arm_length:f32,pub pitch_delta:f32,pub yaw_delta:f32,pub anchor_direction:Vec3}
#[derive(Clone,Debug,PartialEq)]
pub struct NormalCamera{pub rotations:Vec<ActiveRotation>,pub active_rotation:Option<ActiveRotation>,pub position:Vec3,pub pending_translation:Vec3,pub pitch:f32,pub yaw:f32,pub pitch_min:f32,pub pitch_max:f32,pub radial_min:f32,pub radial_max:f32,pub terrain_clearance:f32,pub loose_ground_projection:bool,pub demo_mode:bool}
impl NormalCamera{
 pub fn original_b(mode:CameraMode)->Self{
 let (pitch,yaw,dist,pmin,pmax,rmin,rmax,loose)=match mode{CameraMode::Normal=>(-30.,-31.,50.,5.,90.,90.,150.,false),CameraMode::TiltShift=>(-30.,-31.,125.,12.,80.,300.,450.,true),CameraMode::TopDown=>(-90.,0.,500.,75.,90.,1000.,1200.,true),_=>panic!("not normal body")};
 let rotation=rotation(yaw,pitch);Self{rotations:Vec::new(),active_rotation:None,position:rotation*Vec3::new(0.,0.,dist),pending_translation:Vec3::ZERO,pitch,yaw,pitch_min:pmin,pitch_max:pmax,radial_min:rmin,radial_max:rmax,terrain_clearance:0.75,loose_ground_projection:loose,demo_mode:false}
 }
 pub fn transform(&self)->CameraTransform{CameraTransform{position:self.position,rotation:rotation(self.yaw,self.pitch)}}
 pub fn constrain_to_be_above_terrain(&mut self,terrain:&mut dyn TerrainQueries){let p=self.position+self.pending_translation;let y=(terrain.sample_height(Vec2::new(p.x,p.z))+self.terrain_clearance).max(p.y);self.pending_translation=Vec3::new(p.x-self.position.x,y-self.position.y,p.z-self.position.z);}
 pub fn translate_screen_space(&mut self,movement:Vec3,ctx:MotionContext){let t=ctx.matrices_transform;let ratio=(t.position.y / -t.forward().y).max(1.)/50.;self.pending_translation += (Quat::from_rotation_y(self.yaw.to_radians())*movement)*ratio;}
 pub fn begin_rotation(&mut self,anchor:Vec3,direction:Vec3,eye:Vec3){let l=(eye-anchor).length();self.active_rotation=Some(ActiveRotation{arm_length:l,target_arm_length:l,pitch_delta:0.,yaw_delta:0.,anchor_direction:direction});}
 pub fn finish_rotation(&mut self){if let Some(r)=self.active_rotation.take(){self.rotations.push(r)}}
 pub fn load(&mut self,p:&CameraRigParametersCache){self.pending_translation=Vec3::from(p.position)+rotation(p.yaw,p.pitch)*Vec3::new(0.,0.,p.camera_rig_arm_length)-self.position;self.active_rotation=None;self.rotations.clear();self.rotations.push(ActiveRotation{arm_length:0.,target_arm_length:0.,pitch_delta:p.pitch-self.pitch,yaw_delta:p.yaw-self.yaw,anchor_direction:Vec3::NEG_Z});}
 pub fn zoom(&mut self,amount:f32,anchor:Vec3,lower:f32){let factor=(amount * -0.08).max(-1.).min(1.).exp2();if let Some(r)=self.active_rotation.as_mut(){assert!(lower<=self.radial_max);r.target_arm_length=(factor*r.target_arm_length).max(lower).min(self.radial_max)}else{let offset=(self.position+self.pending_translation)-anchor;let mut next=offset*factor;if next.length_squared()<lower*lower{next=(next/next.length())*lower;}self.pending_translation += next-offset;}}
}
pub fn rotation(yaw:f32,pitch:f32)->Quat{Quat::from_euler(EulerRot::YXZ,yaw*(core::f32::consts::PI/180.),pitch*(core::f32::consts::PI/180.),0.)}
impl CameraBody for NormalCamera{
 fn rotate_yaw_pitch(&mut self,yaw:f32,pitch:f32){let mut pending=0.;for r in &self.rotations{pending+=r.pitch_delta;}if let Some(r)=&self.active_rotation{pending+=r.pitch_delta;}let base=self.pitch+pending;if let Some(r)=self.active_rotation.as_mut(){let bounded=(-(pitch+base)).min(self.pitch_max).max(self.pitch_min);r.pitch_delta += -bounded-base;r.yaw_delta+=yaw;}}
 fn update(&mut self,dt:f32)->CameraTransform{
 if dt<0.{return self.transform()}
 let a=smooth(dt);let target=self.position+self.pending_translation;
 if self.radial_max*self.radial_max<target.length_squared(){let constrained=(target/target.length())*self.radial_max;self.pending_translation += constrained-target;}
 let forward=self.transform().forward();let goal=if self.loose_ground_projection{
 let distance=((self.position.y-10.)/(-forward.y).max(0.15)).max(f32::from_bits(0x41406667));let back=-forward*distance;let anchor=Vec2::new(self.position.x-back.x,self.position.z-back.z);let bounded=bounds_xz(anchor,f32::from_bits(0x4205e666),self.demo_mode);let p=Vec3::new(bounded.x,self.position.y,bounded.y)+Vec3::new(back.x,0.,back.z);if self.radial_min*self.radial_min<p.length_squared(){(p/p.length())*self.radial_min}else{p}
 }else{let lookahead=forward*f32::from_bits(0x42b36666)*0.2;let anchor=Vec2::new(self.position.x+lookahead.x,self.position.z+lookahead.z);let bounded=bounds_xz(anchor,f32::from_bits(0x42336666),self.demo_mode);Vec3::new(bounded.x-lookahead.x,self.position.y,bounded.y-lookahead.z)};
 self.position=Vec3::new(lerp(self.position.x,goal.x,a),lerp(self.position.y,goal.y,a),lerp(self.position.z,goal.z,a));
 for r in self.rotations.iter_mut().chain(self.active_rotation.iter_mut()){
 let pd=a*r.pitch_delta;let yd=a*r.yaw_delta;r.pitch_delta-=pd;r.yaw_delta-=yd;
 let old=rotation(self.yaw,self.pitch)*(-r.anchor_direction*r.arm_length);r.arm_length=r.target_arm_length*a+r.arm_length*(1.-a);self.pitch+=pd;self.yaw+=yd;let new=rotation(self.yaw,self.pitch)*(-r.anchor_direction*r.arm_length);self.position+=new-old;
 }
 let moved=self.pending_translation*a;self.pending_translation-=moved;self.position+=moved;self.position.y=self.position.y.max(1.);
 self.rotations.retain(|r| !(r.pitch_delta.abs()<0.001 && r.yaw_delta.abs()<0.001 && (r.arm_length-r.target_arm_length).abs()/r.target_arm_length.abs().max(1e-20)<0.001));
 self.transform()
 }
}
