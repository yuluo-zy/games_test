//! Native per-entity movement CFG outputs with explicit world boundary
//! inputs. Actual Bevy smooth query and actual core point-query/wall owner
//! contracts have additional nonempty tests below. No GPU/startup pass claimed.
use system_critters::{shared::*,components::*};
use glam::{Vec2,Vec3,Quat};
use std::cell::{Cell,RefCell};
#[derive(Debug,PartialEq)]enum Call {Sample(&'static str,[u32;2]),Sphere([u32;3],[u32;3],u32),Wall([u32;2])}
struct Env {water:Vec<f32>,garden:Vec<f32>,rock:u32,path_values:Vec<[f32;4]>,path_index:Cell<usize>,sphere:Option<Vec3>,wall_blocked:bool,wall_missing:bool,
 water_index:Cell<usize>,garden_index:Cell<usize>,calls:RefCell<Vec<Call>>}
impl Env {fn sample(&self,kind:&'static str,p:Vec2,values:&[f32],index:&Cell<usize>)->f32 {self.calls.borrow_mut().push(Call::Sample(kind,p.to_array().map(f32::to_bits)));let i=index.get();index.set(i+1);values[i.min(values.len()-1)]}}
impl CritterEnvironment for Env {
 fn water(&self,p:Vec2)->f32 {self.sample("water",p,&self.water,&self.water_index)}
 fn garden(&self,p:Vec2)->f32 {self.sample("garden",p,&self.garden,&self.garden_index)}
 fn rock(&self,p:Vec2)->u32 {self.calls.borrow_mut().push(Call::Sample("rock",p.to_array().map(f32::to_bits)));self.rock}
 fn paths(&self,p:Vec2)->[f32;4] {
  for kind in ["path0","path1","path2","path3"] {self.calls.borrow_mut().push(Call::Sample(kind,p.to_array().map(f32::to_bits)));}
  let i=self.path_index.get();self.path_index.set(i+1);self.path_values[i.min(self.path_values.len()-1)]
 }
 fn wall_intersection(&self,p:Vec3,_end:Vec2,loc:u32,require:bool)->bool {self.calls.borrow_mut().push(Call::Wall([p.x.to_bits(),p.z.to_bits()]));assert!(!(self.wall_missing&&(loc!=3||require)),"native PublicWalls::get failure");self.wall_blocked}
 fn spherecast(&self,p:Vec3,d:Vec3,r:f32)->Option<Vec3> {self.calls.borrow_mut().push(Call::Sphere(p.to_array().map(f32::to_bits),d.to_array().map(f32::to_bits),r.to_bits()));self.sphere}
}
fn transform_bits(t:&Transform)->[u32;10] {let q=t.rotation.to_array();let p=t.translation.to_array();let s=t.scale.to_array();[q[0],q[1],q[2],q[3],p[0],p[1],p[2],s[0],s[1],s[2]].map(f32::to_bits)}

#[test]fn native_per_entity_avoidance_policy_and_boundary_arguments() {let mut cases=0;
 { // avoid-loc0-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,0,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc0-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc0-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc0-other boundary calls");cases+=1;
 }
 { // avoid-loc0-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,0,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc0-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc0-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc0-duck boundary calls");cases+=1;
 }
 { // avoid-loc0-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,0,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc0-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc0-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415])],"case avoid-loc0-sheep boundary calls");cases+=1;
 }
 { // avoid-loc0-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,0,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc0-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc0-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc0-both boundary calls");cases+=1;
 }
 { // avoid-loc1-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,1,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc1-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc1-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc1-other boundary calls");cases+=1;
 }
 { // avoid-loc1-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,1,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc1-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc1-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc1-duck boundary calls");cases+=1;
 }
 { // avoid-loc1-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,1,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc1-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc1-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415])],"case avoid-loc1-sheep boundary calls");cases+=1;
 }
 { // avoid-loc1-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,1,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc1-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc1-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc1-both boundary calls");cases+=1;
 }
 { // avoid-loc2-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc2-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc2-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("rock",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc2-other boundary calls");cases+=1;
 }
 { // avoid-loc2-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc2-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc2-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("rock",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1056964608)],"case avoid-loc2-duck boundary calls");cases+=1;
 }
 { // avoid-loc2-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc2-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc2-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("rock",[1029911688,3182061415]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc2-sheep boundary calls");cases+=1;
 }
 { // avoid-loc2-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc2-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc2-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("rock",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc2-both boundary calls");cases+=1;
 }
 { // avoid-loc3-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc3-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc3-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc3-other boundary calls");cases+=1;
 }
 { // avoid-loc3-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc3-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc3-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1056964608)],"case avoid-loc3-duck boundary calls");cases+=1;
 }
 { // avoid-loc3-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc3-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc3-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc3-sheep boundary calls");cases+=1;
 }
 { // avoid-loc3-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc3-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc3-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc3-both boundary calls");cases+=1;
 }
 { // avoid-loc4-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,4,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc4-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc4-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc4-other boundary calls");cases+=1;
 }
 { // avoid-loc4-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,4,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc4-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc4-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1056964608)],"case avoid-loc4-duck boundary calls");cases+=1;
 }
 { // avoid-loc4-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,4,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc4-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc4-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc4-sheep boundary calls");cases+=1;
 }
 { // avoid-loc4-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,4,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc4-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc4-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc4-both boundary calls");cases+=1;
 }
 { // avoid-loc5-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,5,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc5-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc5-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc5-other boundary calls");cases+=1;
 }
 { // avoid-loc5-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,5,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc5-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc5-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc5-duck boundary calls");cases+=1;
 }
 { // avoid-loc5-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,5,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc5-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc5-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415])],"case avoid-loc5-sheep boundary calls");cases+=1;
 }
 { // avoid-loc5-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,5,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc5-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc5-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc5-both boundary calls");cases+=1;
 }
 { // avoid-loc6-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,6,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc6-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc6-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc6-other boundary calls");cases+=1;
 }
 { // avoid-loc6-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,6,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc6-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc6-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc6-duck boundary calls");cases+=1;
 }
 { // avoid-loc6-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,6,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc6-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc6-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415])],"case avoid-loc6-sheep boundary calls");cases+=1;
 }
 { // avoid-loc6-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,6,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc6-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc6-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818])],"case avoid-loc6-both boundary calls");cases+=1;
 }
 { // avoid-loc7-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,7,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc7-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc7-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc7-other boundary calls");cases+=1;
 }
 { // avoid-loc7-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,7,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc7-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc7-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1056964608)],"case avoid-loc7-duck boundary calls");cases+=1;
 }
 { // avoid-loc7-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,7,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc7-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc7-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc7-sheep boundary calls");cases+=1;
 }
 { // avoid-loc7-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,7,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc7-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc7-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc7-both boundary calls");cases+=1;
 }
 { // avoid-loc8-other
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,8,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc8-other");
 assert_eq!(goal.goal_tag,1,"case avoid-loc8-other goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Wall([0,0])],"case avoid-loc8-other boundary calls");cases+=1;
 }
 { // avoid-loc8-duck
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,8,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc8-duck");
 assert_eq!(goal.goal_tag,1,"case avoid-loc8-duck goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1056964608)],"case avoid-loc8-duck boundary calls");cases+=1;
 }
 { // avoid-loc8-sheep
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,8,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc8-sheep");
 assert_eq!(goal.goal_tag,1,"case avoid-loc8-sheep goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc8-sheep boundary calls");cases+=1;
 }
 { // avoid-loc8-both
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,8,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-loc8-both");
 assert_eq!(goal.goal_tag,1,"case avoid-loc8-both goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,3196059648,0],[1057882325,0,3210019136],1061997773)],"case avoid-loc8-both boundary calls");cases+=1;
 }
 { // goal-tag-0
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:0};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-tag-0");
 assert_eq!(goal.goal_tag,0,"case goal-tag-0 goal");
 assert_eq!(*env.calls.borrow(),vec![],"case goal-tag-0 boundary calls");cases+=1;
 }
 { // goal-tag-2
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:2};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-tag-2");
 assert_eq!(goal.goal_tag,2,"case goal-tag-2 goal");
 assert_eq!(*env.calls.borrow(),vec![],"case goal-tag-2 boundary calls");cases+=1;
 }
 { // goal-tag-4294967295
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:4294967295};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-tag-4294967295");
 assert_eq!(goal.goal_tag,4294967295,"case goal-tag-4294967295 goal");
 assert_eq!(*env.calls.borrow(),vec![],"case goal-tag-4294967295 boundary calls");cases+=1;
 }
 { // goal-position-[0, 0, 0]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[1.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([0.0,0.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-position-[0, 0, 0]");
 assert_eq!(goal.goal_tag,1,"case goal-position-[0, 0, 0] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,0]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sphere([0,0,0],[4290772992,4290772992,4290772992],1056964608)],"case goal-position-[0, 0, 0] boundary calls");cases+=1;
 }
 { // goal-position-[nan, 0, 2]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[1.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([f32::from_bits(2143289344),0.0,2.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-position-[nan, 0, 2]");
 assert_eq!(goal.goal_tag,1,"case goal-position-[nan, 0, 2] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,0]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[2143289344,2143289344]),Call::Sample("path1",[2143289344,2143289344]),Call::Sample("path2",[2143289344,2143289344]),Call::Sample("path3",[2143289344,2143289344]),Call::Sphere([0,0,0],[4290772992,4290772992,4290772992],1056964608)],"case goal-position-[nan, 0, 2] boundary calls");cases+=1;
 }
 { // goal-position-[9, 1, 2]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([4.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[1.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([9.0,1.0,2.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case goal-position-[9, 1, 2]");
 assert_eq!(goal.goal_tag,1,"case goal-position-[9, 1, 2] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1082325148,1024991011]),Call::Sample("path0",[1082130432,0]),Call::Sample("path1",[1082130432,0]),Call::Sample("path2",[1082130432,0]),Call::Sample("path3",[1082130432,0]),Call::Sample("path0",[1095353331,1079798967]),Call::Sample("path1",[1095353331,1079798967]),Call::Sample("path2",[1095353331,1079798967]),Call::Sample("path3",[1095353331,1079798967]),Call::Sphere([1082130432,0,0],[1064153254,0,1052649195],1056964608)],"case goal-position-[9, 1, 2] boundary calls");cases+=1;
 }
 { // paths-[[0, 0, 0, 0], [0, 0, 0.1, 0]]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0],[0.0,0.0,f32::from_bits(1036831949),0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([9.0,1.0,2.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case paths-[[0, 0, 0, 0], [0, 0, 0.1, 0]]");
 assert_eq!(goal.goal_tag,0,"case paths-[[0, 0, 0, 0], [0, 0, 0.1, 0]] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036512338,1018279326]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1050014014,1032144951]),Call::Sample("path1",[1050014014,1032144951]),Call::Sample("path2",[1050014014,1032144951]),Call::Sample("path3",[1050014014,1032144951])],"case paths-[[0, 0, 0, 0], [0, 0, 0.1, 0]] boundary calls");cases+=1;
 }
 { // paths-[[nan, 0, 0, 0], [0, 0, 0, 0]]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[f32::from_bits(2143289344),0.0,0.0,0.0],[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([9.0,1.0,2.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case paths-[[nan, 0, 0, 0], [0, 0, 0, 0]]");
 assert_eq!(goal.goal_tag,1,"case paths-[[nan, 0, 0, 0], [0, 0, 0, 0]] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036512338,1018279326]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1050014014,1032144951]),Call::Sample("path1",[1050014014,1032144951]),Call::Sample("path2",[1050014014,1032144951]),Call::Sample("path3",[1050014014,1032144951]),Call::Sphere([0,3196059648,0],[1064953702,0,1046356741],1056964608)],"case paths-[[nan, 0, 0, 0], [0, 0, 0, 0]] boundary calls");cases+=1;
 }
 { // paths-[[2, 3, 4, 5], [0, 0, 0, 0]]
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[2.0,3.0,4.0,5.0],[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([9.0,1.0,2.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case paths-[[2, 3, 4, 5], [0, 0, 0, 0]]");
 assert_eq!(goal.goal_tag,1,"case paths-[[2, 3, 4, 5], [0, 0, 0, 0]] goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036512338,1018279326]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1091622967,1073791025]),Call::Sample("path1",[1091622967,1073791025]),Call::Sample("path2",[1091622967,1073791025]),Call::Sample("path3",[1091622967,1073791025]),Call::Sphere([0,3196059648,0],[1064953702,0,1046356741],1056964608)],"case paths-[[2, 3, 4, 5], [0, 0, 0, 0]] boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-None
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-duck-None");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-duck-None goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-None boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-0.39999998
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609164),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),true,"despawn case avoid-sphere-duck-0.39999998");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-duck-0.39999998 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-0.39999998 boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-0.4
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609165),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-duck-0.4");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-duck-0.4 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-0.4 boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-0.99999994
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1065353215),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-duck-0.99999994");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-duck-0.99999994 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-0.99999994 boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-1
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([1.0,0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-duck-1");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-duck-1 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-1 boundary calls");cases+=1;
 }
 { // avoid-sphere-duck-nan
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(2143289344),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-duck-nan");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-duck-nan goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1056964608)],"case avoid-sphere-duck-nan boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-None
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-None");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-sheep-None goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-None boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-0.39999998
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609164),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-0.39999998");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-sheep-0.39999998 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-0.39999998 boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-0.4
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609165),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-0.4");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-sheep-0.4 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-0.4 boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-0.99999994
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1065353215),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-0.99999994");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-sheep-0.99999994 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-0.99999994 boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-1
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([1.0,0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-1");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-sheep-1 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-1 boundary calls");cases+=1;
 }
 { // avoid-sphere-sheep-nan
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(2143289344),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-sheep-nan");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-sheep-nan goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-sheep-nan boundary calls");cases+=1;
 }
 { // avoid-sphere-both-None
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-both-None");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-both-None goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-None boundary calls");cases+=1;
 }
 { // avoid-sphere-both-0.39999998
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609164),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),true,"despawn case avoid-sphere-both-0.39999998");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-both-0.39999998 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-0.39999998 boundary calls");cases+=1;
 }
 { // avoid-sphere-both-0.4
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1053609165),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-both-0.4");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-both-0.4 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-0.4 boundary calls");cases+=1;
 }
 { // avoid-sphere-both-0.99999994
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(1065353215),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-both-0.99999994");
 assert_eq!(goal.goal_tag,0,"case avoid-sphere-both-0.99999994 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-0.99999994 boundary calls");cases+=1;
 }
 { // avoid-sphere-both-1
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([1.0,0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-both-1");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-both-1 goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-1 boundary calls");cases+=1;
 }
 { // avoid-sphere-both-nan
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:Some(Vec3::from_array([f32::from_bits(2143289344),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,-3.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,true,true)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-sphere-both-nan");
 assert_eq!(goal.goal_tag,1,"case avoid-sphere-both-nan goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1029911688,3182061415]),Call::Sample("path0",[0,0]),Call::Sample("path1",[0,0]),Call::Sample("path2",[0,0]),Call::Sample("path3",[0,0]),Call::Sample("path0",[1042966374,3196033818]),Call::Sample("path1",[1042966374,3196033818]),Call::Sample("path2",[1042966374,3196033818]),Call::Sample("path3",[1042966374,3196033818]),Call::Sphere([0,0,0],[1057882325,0,3210019136],1061997773)],"case avoid-sphere-both-nan boundary calls");cases+=1;
 }
 { // avoid-wall-intersect
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:true,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-wall-intersect");
 assert_eq!(goal.goal_tag,0,"case avoid-wall-intersect goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036831949,0]),Call::Sample("rock",[1036831949,0]),Call::Wall([0,0])],"case avoid-wall-intersect boundary calls");cases+=1;
 }
 { // avoid-wall-parallel
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-wall-parallel");
 assert_eq!(goal.goal_tag,1,"case avoid-wall-parallel goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036831949,0]),Call::Sample("rock",[1036831949,0]),Call::Wall([0,0])],"case avoid-wall-parallel boundary calls");cases+=1;
 }
 { // avoid-wall-low
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,false)));
 assert_eq!(actual.unwrap(),false,"despawn case avoid-wall-low");
 assert_eq!(goal.goal_tag,1,"case avoid-wall-low goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036831949,0]),Call::Sample("rock",[1036831949,0]),Call::Wall([0,0])],"case avoid-wall-low boundary calls");cases+=1;
 }
 { // avoid-wall-missing
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:true,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,2,false,false)));
 assert!(actual.is_err());
 assert_eq!(goal.goal_tag,1,"case avoid-wall-missing goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036831949,0]),Call::Sample("rock",[1036831949,0]),Call::Wall([0,0])],"case avoid-wall-missing boundary calls");cases+=1;
 }
 { // avoid-wall-flying-missing
 let transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,path_values:vec![[0.0,0.0,0.0,0.0]],path_index:Cell::new(0),sphere:None,wall_blocked:false,wall_missing:true,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut goal=CritterGoal {position:Vec3::from_array([2.0,1.0,0.0]),goal_tag:1};
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,3,false,false)));
 assert!(actual.is_err());
 assert_eq!(goal.goal_tag,1,"case avoid-wall-flying-missing goal");
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1036831949,0]),Call::Wall([0,0])],"case avoid-wall-flying-missing boundary calls");cases+=1;
 }
 assert_eq!(cases,68);}
