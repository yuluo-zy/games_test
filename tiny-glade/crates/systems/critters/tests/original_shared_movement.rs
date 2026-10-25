//! Native per-entity movement CFG outputs with explicit world boundary
//! inputs. Actual Bevy smooth query and actual core point-query/wall owner
//! contracts have additional nonempty tests below. No GPU/startup pass claimed.
use system_critters::{shared::*,components::*};
use glam::{Vec2,Vec3,Quat};
use std::cell::{Cell,RefCell};
#[derive(Debug,PartialEq)]enum Call {Sample(&'static str,[u32;2]),Sphere([u32;3],[u32;3],u32),Wall([u32;2])}
struct Env {water:Vec<f32>,garden:Vec<f32>,rock:u32,sphere:Option<Vec3>,wall_blocked:bool,wall_missing:bool,
 water_index:Cell<usize>,garden_index:Cell<usize>,calls:RefCell<Vec<Call>>}
impl Env {fn sample(&self,kind:&'static str,p:Vec2,values:&[f32],index:&Cell<usize>)->f32 {self.calls.borrow_mut().push(Call::Sample(kind,p.to_array().map(f32::to_bits)));let i=index.get();index.set(i+1);values[i.min(values.len()-1)]}}
impl CritterEnvironment for Env {
 fn water(&self,p:Vec2)->f32 {self.sample("water",p,&self.water,&self.water_index)}
 fn garden(&self,p:Vec2)->f32 {self.sample("garden",p,&self.garden,&self.garden_index)}
 fn rock(&self,p:Vec2)->u32 {self.calls.borrow_mut().push(Call::Sample("rock",p.to_array().map(f32::to_bits)));self.rock}
 fn paths(&self,_p:Vec2)->[f32;4] {panic!("movement must not read path rasters")}
 fn wall_intersection(&self,p:Vec3,_end:Vec2,loc:u32,require:bool)->bool {self.calls.borrow_mut().push(Call::Wall([p.x.to_bits(),p.z.to_bits()]));assert!(!(self.wall_missing&&(loc!=3||require)),"native PublicWalls::get failure");self.wall_blocked}
 fn spherecast(&self,p:Vec3,d:Vec3,r:f32)->Option<Vec3> {self.calls.borrow_mut().push(Call::Sphere(p.to_array().map(f32::to_bits),d.to_array().map(f32::to_bits),r.to_bits()));self.sphere}
}
// The original record owns Option<Entity>; the former `occupied: true` fixture is Some(entity).
fn occupied_spots()->system_critters::bird_spots::BirdSpots {
 use system_critters::bird_spots::{BirdSpot,BirdSpotId,BirdSpotManager};
 BirdSpotManager::default().create_spots(vec![BirdSpot {occupant:Some(bevy_ecs::entity::Entity::from_raw(1)),pos:Vec3::ZERO,id:BirdSpotId(0)};2])
}
fn transform_bits(t:&Transform)->[u32;10] {let q=t.rotation.to_array();let p=t.translation.to_array();let s=t.scale.to_array();[q[0],q[1],q[2],q[3],p[0],p[1],p[2],s[0],s[1],s[2]].map(f32::to_bits)}
#[test]fn native_per_entity_movement_policy_and_boundary_arguments() {
 let mut cases=0;
 { // loc0-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,0,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc0-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc0-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc0-other boundary calls");cases+=1;
 }
 { // loc0-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,0,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc0-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc0-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc0-duck boundary calls");cases+=1;
 }
 { // loc0-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,0,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc0-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc0-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc0-sheep boundary calls");cases+=1;
 }
 { // loc0-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,0,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc0-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc0-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc0-both boundary calls");cases+=1;
 }
 { // loc1-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,1,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc1-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc1-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc1-other boundary calls");cases+=1;
 }
 { // loc1-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,1,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc1-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc1-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc1-duck boundary calls");cases+=1;
 }
 { // loc1-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,1,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc1-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc1-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc1-sheep boundary calls");cases+=1;
 }
 { // loc1-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,1,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc1-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc1-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc1-both boundary calls");cases+=1;
 }
 { // loc2-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc2-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc2-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sample("rock",[1056964608,3208642560]),Call::Wall([0,0])],"case loc2-other boundary calls");cases+=1;
 }
 { // loc2-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case loc2-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case loc2-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sample("rock",[1056964608,3208642560])],"case loc2-duck boundary calls");cases+=1;
 }
 { // loc2-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc2-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc2-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sample("rock",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc2-sheep boundary calls");cases+=1;
 }
 { // loc2-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case loc2-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case loc2-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sample("rock",[1056964608,3208642560])],"case loc2-both boundary calls");cases+=1;
 }
 { // loc3-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc3-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc3-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc3-other boundary calls");cases+=1;
 }
 { // loc3-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc3-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc3-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1056964608)],"case loc3-duck boundary calls");cases+=1;
 }
 { // loc3-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc3-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc3-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc3-sheep boundary calls");cases+=1;
 }
 { // loc3-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc3-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc3-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc3-both boundary calls");cases+=1;
 }
 { // loc4-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,4,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc4-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc4-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc4-other boundary calls");cases+=1;
 }
 { // loc4-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,4,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc4-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc4-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1056964608)],"case loc4-duck boundary calls");cases+=1;
 }
 { // loc4-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,4,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc4-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc4-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case loc4-sheep boundary calls");cases+=1;
 }
 { // loc4-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,4,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc4-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc4-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case loc4-both boundary calls");cases+=1;
 }
 { // loc5-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,5,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc5-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc5-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc5-other boundary calls");cases+=1;
 }
 { // loc5-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,5,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc5-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc5-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc5-duck boundary calls");cases+=1;
 }
 { // loc5-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,5,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc5-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc5-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc5-sheep boundary calls");cases+=1;
 }
 { // loc5-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,5,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc5-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc5-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc5-both boundary calls");cases+=1;
 }
 { // loc6-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,6,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc6-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc6-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc6-other boundary calls");cases+=1;
 }
 { // loc6-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,6,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc6-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc6-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc6-duck boundary calls");cases+=1;
 }
 { // loc6-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,6,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc6-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc6-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc6-sheep boundary calls");cases+=1;
 }
 { // loc6-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,6,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc6-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc6-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560])],"case loc6-both boundary calls");cases+=1;
 }
 { // loc7-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,7,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc7-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc7-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc7-other boundary calls");cases+=1;
 }
 { // loc7-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,7,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc7-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc7-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1056964608)],"case loc7-duck boundary calls");cases+=1;
 }
 { // loc7-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,7,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc7-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc7-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc7-sheep boundary calls");cases+=1;
 }
 { // loc7-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,7,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc7-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc7-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc7-both boundary calls");cases+=1;
 }
 { // loc8-other
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,8,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc8-other transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc8-other spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case loc8-other boundary calls");cases+=1;
 }
 { // loc8-duck
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,8,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc8-duck transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc8-duck spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1056964608)],"case loc8-duck boundary calls");cases+=1;
 }
 { // loc8-sheep
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,8,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc8-sheep transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc8-sheep spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc8-sheep boundary calls");cases+=1;
 }
 { // loc8-both
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![1.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,8,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),true,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case loc8-both transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case loc8-both spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case loc8-both boundary calls");cases+=1;
 }
 { // mult-None
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),None,false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case mult-None transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case mult-None spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case mult-None boundary calls");cases+=1;
 }
 { // mult-0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(0.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case mult-0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case mult-0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![],"case mult-0 boundary calls");cases+=1;
 }
 { // mult--0.0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(f32::from_bits(2147483648)),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case mult--0.0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case mult--0.0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![],"case mult--0.0 boundary calls");cases+=1;
 }
 { // mult--1
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(-1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,1061766501,0,1058943529,3204448256,3196059648,1061158912,1073741824,1077936128,1082130432],"case mult--1 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[3223003613,1083076248,1067180351],"case mult--1 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[3204448256,1061158912]),Call::Wall([0,0])],"case mult--1 boundary calls");cases+=1;
 }
 { // mult-nan
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(f32::from_bits(2143289344)),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,2143289344,2143289344,2143289344,1073741824,1077936128,1082130432],"case mult-nan transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[2143289344,2143289344,2143289344],"case mult-nan spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[2143289344,2143289344]),Call::Wall([0,0])],"case mult-nan boundary calls");cases+=1;
 }
 { // dt-0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=0.0;
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case dt-0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,1083600536,3221057700],"case dt-0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,0]),Call::Wall([0,0])],"case dt-0 boundary calls");cases+=1;
 }
 { // dt--0.125
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(3187671040);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3202184224,0,1063702693,3196059648,3187671040,1052770304,1073741824,1077936128,1082130432],"case dt--0.125 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067821996,1083338392,3210557950],"case dt--0.125 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[3196059648,1052770304]),Call::Wall([0,0])],"case dt--0.125 boundary calls");cases+=1;
 }
 { // dt-1e-12
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(730643660);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,739032268,730643660,2891127602,1073741824,1077936128,1082130432],"case dt-1e-12 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[739032268,1083600536,3221057700],"case dt-1e-12 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[739032268,2891127602]),Call::Wall([0,0])],"case dt-1e-12 boundary calls");cases+=1;
 }
 { // velocity-[0, 0, 0]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([0.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case velocity-[0, 0, 0] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,1083600536,3221057700],"case velocity-[0, 0, 0] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,0]),Call::Wall([0,0])],"case velocity-[0, 0, 0] boundary calls");cases+=1;
 }
 { // velocity-[0, 2, 0]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([0.0,2.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,1056964608,0,1073741824,1077936128,1082130432],"case velocity-[0, 2, 0] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,1084649112,3221057700],"case velocity-[0, 2, 0] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,0]),Call::Wall([0,0])],"case velocity-[0, 2, 0] boundary calls");cases+=1;
 }
 { // velocity-[1e-15, 0, 0]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([f32::from_bits(646978941),0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3204739764,0,1062933248,630201725,0,0,1073741824,1077936128,1082130432],"case velocity-[1e-15, 0, 0] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1071672215,1083600536,3211494663],"case velocity-[1e-15, 0, 0] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[630201725,0]),Call::Wall([0,0])],"case velocity-[1e-15, 0, 0] boundary calls");cases+=1;
 }
 { // velocity-[inf, 0, 1]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([f32::from_bits(2139095040),0.0,1.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,2139095040,0,1048576000,1073741824,1077936128,1082130432],"case velocity-[inf, 0, 1] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[2139095040,1083600536,3218960548],"case velocity-[inf, 0, 1] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[2139095040,1048576000]),Call::Wall([0,0])],"case velocity-[inf, 0, 1] boundary calls");cases+=1;
 }
 { // velocity-[0, 0, 1]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([0.0,0.0,1.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3211287246,0,3201755818,0,0,1048576000,1073741824,1077936128,1082130432],"case velocity-[0, 0, 1] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[3217104264,1083600536,1069818114],"case velocity-[0, 0, 1] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,1048576000]),Call::Wall([0,0])],"case velocity-[0, 0, 1] boundary calls");cases+=1;
 }
 { // velocity-[0, 0, -1]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([0.0,0.0,-1.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,3196059648,1073741824,1077936128,1082130432],"case velocity-[0, 0, -1] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,1083600536,3222190162],"case velocity-[0, 0, -1] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[0,3196059648]),Call::Wall([0,0])],"case velocity-[0, 0, -1] boundary calls");cases+=1;
 }
 { // wall-intersect
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:true,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([4.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case wall-intersect transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case wall-intersect spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1065353216,0]),Call::Sample("rock",[1065353216,0]),Call::Wall([0,0])],"case wall-intersect boundary calls");cases+=1;
 }
 { // wall-parallel
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([4.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3204739764,0,1062933248,1065353216,0,0,1073741824,1077936128,1082130432],"case wall-parallel transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1076901324,1083600536,3211494663],"case wall-parallel spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1065353216,0]),Call::Sample("rock",[1065353216,0]),Call::Wall([0,0])],"case wall-parallel boundary calls");cases+=1;
 }
 { // wall-low
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([4.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3204739764,0,1062933248,1065353216,0,0,1073741824,1077936128,1082130432],"case wall-low transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1076901324,1083600536,3211494663],"case wall-low spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1065353216,0]),Call::Sample("rock",[1065353216,0]),Call::Wall([0,0])],"case wall-low boundary calls");cases+=1;
 }
 { // wall-missing
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:true,wall_missing:true,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,2,Vec3::from_array([4.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert!(actual.is_err());
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1065353216,0]),Call::Sample("rock",[1065353216,0]),Call::Wall([0,0])],"case wall-missing boundary calls");cases+=1;
 }
 { // wall-flying-missing
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:true,wall_missing:true,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([4.0,0.0,0.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case wall-flying-missing transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case wall-flying-missing spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1065353216,0]),Call::Wall([0,0])],"case wall-flying-missing boundary calls");cases+=1;
 }
 { // sphere-None
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case sphere-None transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case sphere-None spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case sphere-None boundary calls");cases+=1;
 }
 { // sphere-0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([0.0,0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case sphere-0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case sphere-0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case sphere-0 boundary calls");cases+=1;
 }
 { // sphere-0.39999998
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([f32::from_bits(1053609164),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case sphere-0.39999998 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case sphere-0.39999998 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case sphere-0.39999998 boundary calls");cases+=1;
 }
 { // sphere-0.4
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([f32::from_bits(1053609165),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case sphere-0.4 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case sphere-0.4 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case sphere-0.4 boundary calls");cases+=1;
 }
 { // sphere-0.99999994
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([f32::from_bits(1065353215),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case sphere-0.99999994 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case sphere-0.99999994 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773)],"case sphere-0.99999994 boundary calls");cases+=1;
 }
 { // sphere-1
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([1.0,0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case sphere-1 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case sphere-1 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case sphere-1 boundary calls");cases+=1;
 }
 { // sphere-1.0000001
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([f32::from_bits(1065353217),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case sphere-1.0000001 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case sphere-1.0000001 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case sphere-1.0000001 boundary calls");cases+=1;
 }
 { // sphere-nan
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,0.0,0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:Some(Vec3::from_array([f32::from_bits(2143289344),0.0,0.0])),wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case sphere-nan transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case sphere-nan spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case sphere-nan boundary calls");cases+=1;
 }
 { // garden-[0, 1]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0,1.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case garden-[0, 1] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case garden-[0, 1] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case garden-[0, 1] boundary calls");cases+=1;
 }
 { // garden-[1, 0]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![1.0,0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case garden-[1, 0] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case garden-[1, 0] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case garden-[1, 0] boundary calls");cases+=1;
 }
 { // garden-[nan, 1]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![f32::from_bits(2143289344),1.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),false);assert_eq!(transform_bits(&transform),[0,0,0,1065353216,0,0,0,1073741824,1077936128,1082130432],"case garden-[nan, 1] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[0,0,0],"case garden-[nan, 1] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case garden-[nan, 1] boundary calls");cases+=1;
 }
 { // garden-[0, nan]
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(3196059648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0,f32::from_bits(2143289344)],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,true,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case garden-[0, nan] transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case garden-[0, nan] spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Sphere([0,0,0],[1057543799,1049155191,3209511346],1061997773),Call::Sample("garden",[0,0]),Call::Sample("garden",[1056964608,3208642560])],"case garden-[0, nan] boundary calls");cases+=1;
 }
 { // y-nan
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(2143289344),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case y-nan transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case y-nan spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case y-nan boundary calls");cases+=1;
 }
 { // y--0.0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(2147483648),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case y--0.0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case y--0.0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case y--0.0 boundary calls");cases+=1;
 }
 { // y-0.0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(0),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1048576000,3208642560,1073741824,1077936128,1082130432],"case y-0.0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1084124824,3223622865],"case y-0.0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case y-0.0 boundary calls");cases+=1;
 }
 { // y-1.0
 let mut transform=Transform {rotation:Quat::from_array([0.0,0.0,0.0,1.0]),translation:Vec3::from_array([0.0,f32::from_bits(1065353216),0.0]),scale:Vec3::from_array([2.0,3.0,4.0])};
 let env=Env {water:vec![0.0],garden:vec![0.0],rock:0,sphere:None,wall_blocked:false,wall_missing:false,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};
 let mut spots=occupied_spots();let dt=f32::from_bits(1048576000);
 let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,3,Vec3::from_array([2.0,1.0,-3.0]),Some(1.0),false,false,Some(&mut spots),dt,0.01_f32.powf(dt))));
 assert_eq!(actual.unwrap(),true);assert_eq!(transform_bits(&transform),[0,3192704923,0,1065014210,1056964608,1067450368,3208642560,1073741824,1077936128,1082130432],"case y-1.0 transform");
 assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),[1067668792,1086221976,3223622865],"case y-1.0 spot");assert!(spots.spots.iter().all(|s|s.occupant.is_some()));
 assert_eq!(*env.calls.borrow(),vec![Call::Sample("water",[1056964608,3208642560]),Call::Wall([0,0])],"case y-1.0 boundary calls");cases+=1;
 }
 assert_eq!(cases,71);
}
#[test]fn actual_smoothing_query_does_not_require_optional_multiplier() {
 use bevy_ecs::{world::World,schedule::Schedule};
 let mut world=World::new();world.insert_resource(country_core::resources::time::GameTime {delta:0.25,..Default::default()});
 let target=CritterVelocityTarget {direction:Vec3::new(7.0,-3.0,2.0),retention_per_second:0.05};let initial=Vec3::new(-2.0,1.0,4.0);
 let entity=world.spawn((target,Velocity(initial))).id();let mut schedule=Schedule::default();schedule.add_systems(smooth_critter_velocities);schedule.run(&mut world);
 let mut expected=initial;smooth_velocity(&mut expected,target,0.25);assert_eq!(world.get::<Velocity>(entity).unwrap().0,expected);assert_ne!(expected,initial);assert!(world.get::<CritterVelocityMultiplier>(entity).is_none());
}
#[test]fn actual_point_cell_geometry_and_two_public_owner_failure_contracts() {
 use country_core::{resources::walls::{WallId,public_walls::PublicWalls},systems::wall::wall_state::wall_spatial_hash::{WallSpatialHash,WallSpatialSegment}};
 let mut hash=WallSpatialHash::default();let walls=PublicWalls::default();
 hash.add_segment(WallSpatialSegment {endpoints:[[0.5,-1.0],[0.5,1.0]],source_u:[0.0,1.0],relative_heights:[0.0;2],terrain_heights:[0.0;2],wall:WallId(91)});
 assert!(wall_intersection(&hash,&walls,Vec3::ZERO,Vec2::new(1.0,0.0),3,false));
 assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||wall_intersection(&hash,&walls,Vec3::ZERO,Vec2::new(1.0,0.0),3,true))).is_err());
 assert!(!wall_intersection(&hash,&walls,Vec3::new(65.0,0.0,0.0),Vec2::ZERO,2,true));
}
#[test]fn compatibility_raycast_name_is_the_canonical_world_identity() {
 assert_eq!(std::any::TypeId::of::<RaycastWorldHandle>(),std::any::TypeId::of::<country_core::systems::collision::world::RaycastWorld>());
}
