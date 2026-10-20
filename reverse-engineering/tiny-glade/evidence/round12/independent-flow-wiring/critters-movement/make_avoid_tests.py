from pathlib import Path
import json,hashlib,difflib
here=Path(__file__).parent;fixture=json.loads((here/'native-avoid-policy.json').read_text());root=Path('D:/game/tiny-glade')
def f(v):
 if isinstance(v,dict):return 'f32::from_bits(%d)'%v['f32']
 return str(v)+'.0' if isinstance(v,int) else repr(v)+'_f32'
def array(v,fn=str):return '['+','.join(map(fn,v))+']'
source=(here/'movement.tests.rs').read_text().split('#[test]fn native_per_entity')[0]
source=source.replace('rock:u32,','rock:u32,path_values:Vec<[f32;4]>,path_index:Cell<usize>,',1)
source=source.replace('fn paths(&self,_p:Vec2)->[f32;4] {panic!("movement must not read path rasters")}','''fn paths(&self,p:Vec2)->[f32;4] {
  for kind in ["path0","path1","path2","path3"] {self.calls.borrow_mut().push(Call::Sample(kind,p.to_array().map(f32::to_bits)));}
  let i=self.path_index.get();self.path_index.set(i+1);self.path_values[i.min(self.path_values.len()-1)]
 }''')
source+='\n#[test]fn native_per_entity_avoidance_policy_and_boundary_arguments() {let mut cases=0;\n'
for row in fixture['results']:
 c=row['input'];o=row['output'];sphere='None' if c['sphere'] is None else 'Some(Vec3::from_array(%s))'%array(c['sphere'],f)
 source+=' { // '+c['name']+'\n'
 source+=' let transform=Transform {rotation:Quat::from_array(%s),translation:Vec3::from_array(%s),scale:Vec3::from_array(%s)};\n'%(array(c['rotation'],f),array(c['position'],f),array(c['scale'],f))
 source+=' let env=Env {water:vec!%s,garden:vec![0.0],rock:%d,path_values:vec!%s,path_index:Cell::new(0),sphere:%s,wall_blocked:%s,wall_missing:%s,water_index:Cell::new(0),garden_index:Cell::new(0),calls:RefCell::new(vec![])};\n'%(array(c['water'],f),c['rock'],array(c['paths'],lambda v:array(v,f)),sphere,str(bool(c['walls']) and o['goal_tag']==0).lower(),str(any(w.get('missing',False) for w in c['walls'])).lower())
 source+=' let mut goal=CritterGoal {position:Vec3::from_array(%s),goal_tag:%d};\n'%(array(c['goal'],f),c['goal_tag'])
 source+=' let actual=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||avoid_critter_collision(&env,&transform,&mut goal,%d,%s,%s)));\n'%(c['loc'],str(c['duck']).lower(),str(c['sheep']).lower())
 if o['status']=='panic':source+=' assert!(actual.is_err());\n'
 else:source+=' assert_eq!(actual.unwrap(),%s,"despawn case %s");\n'%(str(o['despawn']!=0).lower(),c['name'])
 source+=' assert_eq!(goal.goal_tag,%d,"case %s goal");\n'%(o['goal_tag'],c['name'])
 expected=[]
 for call in o['calls']:
  if call['kind'] in ['water','rock'] or call['kind'].startswith('path'):expected.append('Call::Sample("%s",%s)'%(call['kind'],array(call['position'])))
  elif call['kind']=='sphere':expected.append('Call::Sphere(%s,%s,%d)'%(array(call['origin']),array(call['direction']),call['radius']))
  elif call['kind']=='walls':expected.append('Call::Wall(%s)'%array(call['position']))
 source+=' assert_eq!(*env.calls.borrow(),vec!['+','.join(expected)+'],"case '+c['name']+' boundary calls");cases+=1;\n }\n'
source+=' assert_eq!(cases,68);}\n'
rel='crates/systems/critters/tests/original_shared_avoidance.rs';data=source.encode();dst=here/'avoid-candidate'/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(data)
assert not (root/rel).exists(),'do not replace central source/test'
patch=''.join(difflib.unified_diff([],source.splitlines(True),'/dev/null','b/'+rel));(here/'avoid-candidate.patch').write_bytes(patch.encode())
(here/'avoid-manifest.json').write_text(json.dumps({'scope':'evidence-only additional68-case native differential test; full business source already in first shared candidate','files':{rel:{'base_sha256':None,'candidate_sha256':hashlib.sha256(data).hexdigest()}},'execution':'original68 executed, candidate Rust pending central execution'},indent=2),encoding='utf-8');print('avoid Rust cases',len(fixture['results']))
