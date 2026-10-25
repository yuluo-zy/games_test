from pathlib import Path
import json,struct,hashlib,subprocess
O=Path(__file__).resolve().parent;data=json.loads((O/'native-butterfly-state-fixtures.json').read_text())
def bits(v):return v['f32'] if isinstance(v,dict) else struct.unpack('<I',struct.pack('<f',v))[0]
def arr(v):return '['+','.join(hex(bits(x)) for x in v)+']'
rows=[]
for r in data['cases']:
 c=r['input'];out=r['output'];initial=[c['state']]+[bits(c[k]) for k in ['state_t','goal_timer','dwell','phase','phase_ref']]
 rows.append('StateCase { label:'+json.dumps(c['name'])+', initial:['+','.join(hex(x) for x in initial)+'], position:'+arr(c['position'])+', velocity:'+arr(c['velocity'])+', delta:'+hex(bits(c['dt']))+', camera_mode:'+str(c['camera_mode'])+', eye:'+arr(c['eye'])+', cursor:'+arr(c['cursor'])+', sheep:&['+','.join(arr(x) for x in c['sheep'])+'],goal:'+str(c['goal'])+', water:'+hex(bits(c['water']))+',rock:'+str(c['rock'])+',random:&'+arr(c['random'])+',expected:['+','.join(hex(x) for x in out['state_bits'])+'],draws:'+str(out['draws'])+',calls:&['+','.join('('+json.dumps(x['kind'])+',['+','.join(hex(b) for b in x['xz'])+'])' for x in out['calls'])+'] },')
fixture='const STATE_CASES:&[StateCase]=&[\n'+'\n'.join(rows)+'\n];\n';(O/'butterfly_state_original_b.rs').write_text(fixture,encoding='utf8')
tests='''
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct StateCase {
        label: &'static str, initial:[u32;6], position:[u32;3], velocity:[u32;3],
        delta:u32, camera_mode:u8,eye:[u32;3],cursor:[u32;3],sheep:&'static [[u32;2]],
        goal:u32,water:u32,rock:u32,random:&'static [u32],expected:[u32;6],draws:usize,
        calls:&'static [(&'static str,[u32;2])],
    }
    struct Environment<'a> { case:&'a StateCase,calls:RefCell<Vec<(&'static str,[u32;2])>> }
    impl ButterflyEnvironment for Environment<'_> {
        fn rock(&self,p:Vec2)->u32 { self.calls.borrow_mut().push(("rock",[p.x.to_bits(),p.y.to_bits()]));self.case.rock }
        fn water(&self,p:Vec2)->f32 { self.calls.borrow_mut().push(("water",[p.x.to_bits(),p.y.to_bits()]));f32::from_bits(self.case.water) }
    }
    fn vec3(bits:[u32;3])->Vec3 { Vec3::from_array(bits.map(f32::from_bits)) }
    include!("tests/fixtures/butterfly_state_original_b.rs");
    #[test]
    fn complete_original_state_cfg_cases() {
        for case in STATE_CASES {
            let b=case.initial;
            let mut st=Butterfly {state:b[0],state_t:f32::from_bits(b[1]),goal_timer:f32::from_bits(b[2]),dwell:f32::from_bits(b[3]),phase:f32::from_bits(b[4]),phase_ref:f32::from_bits(b[5])};
            let env=Environment {case,calls:RefCell::new(Vec::new())};let mut draws=0;
            let sheep:Vec<_>=case.sheep.iter().map(|p|Vec2::from_array(p.map(f32::from_bits))).collect();
            update_state(&mut st,vec3(case.position),vec3(case.velocity),case.goal!=0,f32::from_bits(case.delta),case.camera_mode,vec3(case.eye),vec3(case.cursor),&sheep,&env,&mut || {let v=f32::from_bits(case.random[draws%case.random.len()]);draws+=1;v});
            let actual=[st.state,st.state_t.to_bits(),st.goal_timer.to_bits(),st.dwell.to_bits(),st.phase.to_bits(),st.phase_ref.to_bits()];
            for (i,(a,e)) in actual.into_iter().zip(case.expected).enumerate() {
                if i!=0 && f32::from_bits(e).is_nan() {assert!(f32::from_bits(a).is_nan(),"{} field{}",case.label,i);}
                else {assert_eq!(a,e,"{} field{}",case.label,i);}
            }
            assert_eq!(draws,case.draws,"{} RNG",case.label);
            assert_eq!(&*env.calls.borrow(),case.calls,"{} environment reads",case.label);
        }
    }
}
'''
policy=(O/'butterfly_state_policy.rs').read_text();(O/'butterfly_state_full.rs').write_text(policy+tests,encoding='utf8')
# Scalar differential adapter: same unmodified policy+test bodies and cached
# real glam. Mirrors only input record fields; ECS/owner admission isn't tested.
adapter='''extern crate self as country_core;
pub mod resources { pub mod critters {#[derive(Clone,Copy,Debug)] pub struct Butterfly {pub state:u32,pub state_t:f32,pub goal_timer:f32,pub dwell:f32,pub phase:f32,pub phase_ref:f32}}}
'''+policy.replace('//! Original complete state policy141d85655..141d85317.','// Original policy body retained verbatim.')+tests.replace('include!("tests/fixtures/butterfly_state_original_b.rs");',fixture)
(O/'state_scalar_adapter.rs').write_text(adapter,encoding='utf8');print('prepared',len(rows),'scalar original differential cases')
