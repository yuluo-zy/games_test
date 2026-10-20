from pathlib import Path
import json
here=Path(__file__).parent;fixture=json.loads((here/'native-move-policy.json').read_text())
def f(v):
 if isinstance(v,dict):return 'f32::from_bits(%d)'%v['f32']
 return str(v)+'.0' if isinstance(v,int) else repr(v)+'_f32'
def array(v,fn=str):return '['+','.join(map(fn,v))+']'
source='''//! Native per-entity movement CFG outputs with explicit world boundary
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
fn transform_bits(t:&Transform)->[u32;10] {let q=t.rotation.to_array();let p=t.translation.to_array();let s=t.scale.to_array();[q[0],q[1],q[2],q[3],p[0],p[1],p[2],s[0],s[1],s[2]].map(f32::to_bits)}
#[test]fn native_per_entity_movement_policy_and_boundary_arguments() {
 let mut cases=0;
'''
for row in fixture['results']:
 c=row['input'];o=row['output'];mult='None' if c['multiplier'] is None else 'Some(%s)'%f(c['multiplier']);sphere='None' if c['sphere'] is None else 'Some(Vec3::from_array(%s))'%array(c['sphere'],f)
 source+=' { // '+c['name']+'\n'
 source+=' let mut transform=Transform {rotation:Quat::from_array(%s),translation:Vec3::from_array(%s),scale:Vec3::from_array(%s)};\n'%(array(c['rotation'],f),array(c['position'],f),array(c['scale'],f))
 source+=' let env=Env {water:vec!%s,garden:vec!%s,rock:%d,sphere:%s,wall_blocked:%s,wall_missing:%s,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};\n'%(array(c['water'],f),array(c['garden'],f),c['rock'],sphere,str(bool(c['walls']) and not o['advanced']).lower(),str(any(w.get('missing',False) for w in c['walls'])).lower())
 source+=' let mut spots=BirdSpots {spots:vec![BirdSpot {pos:Vec3::ZERO,occupied:true};2]};let dt=%s;\n'%f(c['dt'])
 source+=' let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||advance_critter(&env,&mut transform,%d,Vec3::from_array(%s),%s,%s,%s,%s,dt,0.01_f32.powf(dt))));\n'%(c['loc'],array(c['velocity'],f),mult,str(c['duck']).lower(),str(c['sheep']).lower(),'Some(&mut spots)' if c['spots'] else 'None')
 if o['status']=='panic':source+=' assert!(actual.is_err());\n'
 else:
  source+=' assert_eq!(actual.unwrap(),%s);assert_eq!(transform_bits(&transform),%s,"case %s transform");\n'%(str(o['advanced']).lower(),array(o['transform_bits']),c['name'])
  if c['spots']:source+=' assert_eq!(spots.spots[0].pos.to_array().map(f32::to_bits),%s,"case %s spot");assert!(spots.spots.iter().all(|s|s.occupied));\n'%(array(o['spot_bits']),c['name'])
 expected=[]
 for call in o['calls']:
  if call['kind'] in ['water','garden','rock']:expected.append('Call::Sample("%s",%s)'%(call['kind'],array(call['position'])))
  elif call['kind']=='sphere':expected.append('Call::Sphere(%s,%s,%d)'%(array(call['origin']),array(call['direction']),call['radius']))
  elif call['kind']=='walls':expected.append('Call::Wall(%s)'%array(call['position']))
 source+=' assert_eq!(*env.calls.borrow(),vec!['+','.join(expected)+'],"case '+c['name']+' boundary calls");cases+=1;\n }\n'
source+=''' assert_eq!(cases,71);
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
'''
(here/'movement.tests.rs').write_text(source,encoding='utf-8');print('generated native movement test cases',len(fixture['results']))
