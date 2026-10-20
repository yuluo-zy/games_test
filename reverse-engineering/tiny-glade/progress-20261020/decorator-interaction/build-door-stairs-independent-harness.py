import json,hashlib
from pathlib import Path
root=Path('D:/game');here=Path(__file__).parent
source=root/'tiny-glade/crates/systems/decorator/src/decorator_visual/door_stairs_geometry.rs'
cases=json.loads((here/'door-stairs-native.json').read_text())
out=['''#[path = "D:/game/tiny-glade/crates/systems/decorator/src/decorator_visual/door_stairs_geometry.rs"] mod actual;
use glam::{Quat,Vec3};
struct Case{q:[u32;4],p:[u32;3],width:u32,bottom:u32,top:u32,h:[u32;4],coverage:u32,expected:Option<[u32;8]>,samples:&'static [[u32;2]],path:&'static [[u32;2]]}
const CASES:&[Case]=&[''']
for c in cases:
    def arr(xs):return '['+','.join(str(x)+'u32' for x in xs)+']'
    expected='Some('+arr(c['output'])+')' if c['output'] else 'None'
    out.append('Case{q:'+arr(c['q'])+',p:'+arr(c['p'])+',width:'+str(c['width'])+',bottom:'+str(c['bottom'])+',top:'+str(c['top'])+',h:'+arr(c['heights'])+',coverage:'+str(c['coverage'])+',expected:'+expected+',samples:&['+','.join(arr(x) for x in c['terrain_coordinates'])+'],path:&['+','.join(arr(x) for x in c['path_coordinates'])+']},')
out.append('''];
fn main(){let mut bad=0usize;
for (index,c) in CASES.iter().enumerate(){
 let q=Quat::from_array(c.q.map(f32::from_bits));let p=Vec3::from_array(c.p.map(f32::from_bits));
 let h=c.h.map(f32::from_bits);let mut sampled=Vec::new();let mut path=Vec::new();
 let result=actual::sample_door_stairs(q,p,f32::from_bits(c.width),f32::from_bits(c.bottom),f32::from_bits(c.top),
 |at|{let n=sampled.len();sampled.push(at.to_array().map(f32::to_bits));h[n]},
 |at|{path.push(at.to_array().map(f32::to_bits));f32::from_bits(c.coverage)});
 let output=result.map(|a|a.map(f32::to_bits));
 if output!=c.expected || sampled!=c.samples || path!=c.path {bad+=1;if bad<12{println!("case {index} output {:?} expected {:?} coords {:?} expected {:?}",output,c.expected,sampled,c.samples);}}
}
println!("actual-source native comparison: {} cases, {bad} mismatches",CASES.len());assert_eq!(bad,0);
}''')
(here/'door-stairs-harness.rs').write_text('\n'.join(out),encoding='utf-8')
(here/'actual-source-manifest.json').write_text(json.dumps({'source':str(source),'sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
 'native_fixture_sha256':hashlib.sha256((here/'door-stairs-native.json').read_bytes()).hexdigest()},indent=2))
