from pathlib import Path
import json
here=Path(__file__).resolve().parent
fixtures=json.loads((here/'native-threshold-fixtures.json').read_text())['cases']
policy=here/'proposed/tiny-glade/crates/country-core/src/systems/wall/wall_state/debug_threshold_policy.rs'
source='''use glam::Vec3;
mod utils { pub mod debug_viz {
    use glam::Vec3;
    #[derive(Clone,Copy,Debug)] pub enum DebugColor { White, Red, Blue }
    #[derive(Clone,Copy,Debug)] pub struct DebugSphere { pub color:DebugColor,pub radius:f32,pub center:Vec3,pub alpha:f32,pub solid:bool }
}}
'''
source+=f'#[path = {json.dumps(str(policy).replace(chr(92),chr(47)))}] mod policy;\n'
source+='''struct Case { enabled:bool, lengths:[usize;4], pos:[u32;3], threshold:u32,arch:u32,third:u32,expected:&'static [[u32;7]] }
fn main(){let cases=&[
'''
for c in fixtures:
    rows=[[r['color'],r['radius_bits'],*r['position_bits'],r['line_width_bits'],r['filled_in']] for r in c['native']]
    source+='Case{enabled:'+str(c['enabled']).lower()+',lengths:'+str(c['lengths'])+',pos:'+str(c['position_bits'])+',threshold:'+str(c['threshold_bits'])+',arch:'+str(c['arch_bits'])+',third:'+str(c['third_bits'])+',expected:&'+str(rows)+'},\n'
source+=''' ];let mut count=0;
for (index,c) in cases.iter().enumerate() {
 let positions=vec![c.pos.map(f32::from_bits);c.lengths[0]];
 let thresholds=vec![f32::from_bits(c.threshold);c.lengths[1]];
 let arches=vec![f32::from_bits(c.arch);c.lengths[2]];
 let third=vec![f32::from_bits(c.third);c.lengths[3]];
 let mut actual=Vec::<[u32;7]>::new();
 if c.enabled {policy::threshold_spheres(&positions,&thresholds,&arches,&third,|s| {
   let color=match s.color {utils::debug_viz::DebugColor::White=>0,utils::debug_viz::DebugColor::Red=>1,utils::debug_viz::DebugColor::Blue=>2};
   let [x,y,z]=s.center.to_array().map(f32::to_bits);
   actual.push([color,s.radius.to_bits(),x,y,z,s.alpha.to_bits(),s.solid as u32]);
 });}
 assert_eq!(actual,c.expected,"original fixture case {index}");count+=actual.len();
}
println!("PASS: exact Rust production threshold policy; {} original whole-function cases, {} emitted spheres, all semantic bits match",cases.len(),count);
}
'''
(here/'leaf_validation.rs').write_text(source,encoding='utf8')
print(len(fixtures),'native cases prepared')
