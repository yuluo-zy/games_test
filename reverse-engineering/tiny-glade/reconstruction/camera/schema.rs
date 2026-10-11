//! B schema and typed outputs. Names not established by native consumers remain observed words.
use glam::{Mat4,Quat,Vec3};
use serde::{Serialize,Deserialize};
#[derive(Clone,Copy,Debug,PartialEq,Eq,Serialize,Deserialize)]
#[repr(u8)]
pub enum CameraMode { Normal=0,TiltShift=1,Free=2,Walk=3,TopDown=4,Butterfly=5 }
#[derive(Clone,Copy,Debug,PartialEq,Serialize,Deserialize)]
pub struct CameraRigParametersCache {
 pub yaw:f32,pub pitch:f32,pub position:[f32;3],pub camera_rig_arm_length:f32,
 #[serde(skip_serializing,default="default_vertical_fov_degrees")]
 pub vertical_fov_degrees:f32,
 pub pitch_offset:f32,pub yaw_offset:f32,
}
pub fn default_vertical_fov_degrees()->f32{45.0}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CameraTransform{pub position:Vec3,pub rotation:Quat}
impl CameraTransform{pub fn forward(self)->Vec3{self.rotation*Vec3::NEG_Z}}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CameraFocus{pub tag:u32,pub observed:[u32;5],pub smoothing:f32}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CameraLens{pub near:f32,pub vertical_fov_degrees:f32,pub target_vertical_fov_degrees:f32,pub fov_min:f32,pub fov_max:f32,pub fov_default:f32,pub observed_scalars:[f32;4],pub observed_flag:bool}
impl CameraLens{
 pub fn original_b(mode:CameraMode)->Self{
 let (fov,min,max,other,flag)=match mode{
 CameraMode::Normal|CameraMode::Free|CameraMode::Butterfly=>(45.,5.,90.,[40.,10.,80.,40.],false),
 CameraMode::TiltShift=>(15.,5.,20.,[960.,600.,2000.,960.],true),
 CameraMode::Walk=>(75.,30.,90.,[10.,2.,30.,10.],false),
 CameraMode::TopDown=>(8.,2.,10.,[40.,20.,60.,40.],false)};
 Self{near:0.1,vertical_fov_degrees:fov,target_vertical_fov_degrees:fov,fov_min:min,fov_max:max,fov_default:fov,observed_scalars:other,observed_flag:flag}
 }
 pub fn immediately_set_vertical_fov_degrees(&mut self,v:f32){self.vertical_fov_degrees=v;self.target_vertical_fov_degrees=v;}
 pub fn update(&mut self,dt:f32){let a=1.0-(dt / -0.2).exp();self.vertical_fov_degrees=(1.-a)*self.vertical_fov_degrees+a*self.target_vertical_fov_degrees;}
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct CameraMatrices{pub world_to_view:Mat4,pub view_to_world:Mat4,pub view_to_clip:Mat4,pub clip_to_view:Mat4}
impl CameraMatrices{
 pub fn new(t:CameraTransform,l:CameraLens,aspect:f32)->Self{
 let view_to_world=Mat4::from_rotation_translation(t.rotation,t.position);
 let cot=(l.vertical_fov_degrees*(core::f32::consts::PI/180.)*0.5).cos()/(l.vertical_fov_degrees*(core::f32::consts::PI/180.)*0.5).sin();
 // Original reverse-Z infinite perspective, RH negative-Z forward.
 let view_to_clip=Mat4::from_cols_array(&[cot/aspect,0.,0.,0.,0.,cot,0.,0.,0.,0.,0.,-1.,0.,0.,l.near,0.]);
 let clip_to_view=Mat4::from_cols_array(&[1./(cot/aspect),0.,0.,0.,0.,1./cot,0.,0.,0.,0.,0.,1./l.near,0.,0.,-1.,0.]);
 Self{world_to_view:view_to_world.inverse(),view_to_world,view_to_clip,clip_to_view}
 }
}
