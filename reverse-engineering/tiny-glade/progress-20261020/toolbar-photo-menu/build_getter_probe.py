from pathlib import Path
import json, struct, hashlib
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
source=ROOT/'tiny-glade/crates/systems/toolbar/src/photomode.rs'
s=source.read_text(encoding='utf8')
helpers=s[s.index('pub fn slider_val_01('):s.index('/// Native parameter dispatch:')]
math=ROOT/'tiny-glade/crates/systems/toolbar/src/photomode/slider_math.rs'
f=lambda x:f'f32::from_bits(0x{struct.unpack("<I",struct.pack("<f",x))[0]:08x})'
h='''#[path="MATH"] mod slider_math;
mod country_core {pub mod resources {pub mod walls {pub struct EraserSizeMultiplier(pub f32);}}}
const FOV_OFFSETS:[usize;6]=[0x8c,0x144,0x1c4,0x264,0x31c,0x3a4];
struct MainCamera {raw:[u8;0x4e0]}
impl MainCamera {
fn read_f32(&self,o:usize)->f32{f32::from_le_bytes(self.raw[o..o+4].try_into().unwrap())}
fn write_f32(&mut self,o:usize,v:f32){self.raw[o..o+4].copy_from_slice(&v.to_le_bytes())}
fn bytes(&self)->&[u8]{&self.raw}
fn mode_byte(&self)->u8{self.raw[0x4d0]}
}
struct Row { params_18:[f32;4],params_30:[f32;2],word_38:u32 }
struct ImageFormationConfigs {row:Row}
impl ImageFormationConfigs {fn current(&self)->&Row{&self.row}}
struct BrushSizes {path:f32,water:f32,garden:f32}
'''.replace('MATH',math.as_posix())+helpers+'\nfn main(){std::thread::Builder::new().stack_size(32*1024*1024).spawn(run).unwrap().join().unwrap();}\nfn run(){let mut passed=0;\n'
for i,c in enumerate(json.loads((HERE/'getter-native-fixtures.json').read_text())):
 v=c['values'];l=c['lens'];t=c['tool'];k=c['kind'];mode=c['mode']
 h+='{' + f'let mut camera=MainCamera{{raw:[0;0x4e0]}};camera.raw[0x4d0]={mode};\n'
 for off,val in zip([0xc,0x10,8,0x1c,0x20,0x18],l):h+=f'camera.write_f32(0x100+FOV_OFFSETS[{mode}]+{off},{f(val)});\n'
 h+=f'camera.write_f32(focus_offset(&camera)+0x18,{f(c["distance"])});\n'
 h+=f'let formation=ImageFormationConfigs{{row:Row{{params_18:[0.,{f(v[0])},{f(v[1])},{f(v[2])}],params_30:[{f(v[3])},{f(v[5])}],word_38:{f(v[4])}.to_bits()}}}};\n'
 h+=f'let brushes=BrushSizes{{path:{f(t[0])},water:{f(t[1])},garden:{f(t[2])}}};\n'
 expr=f'slider_current_position({k},&formation,&camera,&country_core::resources::walls::EraserSizeMultiplier({f(c["terrain"])}),&brushes).unwrap()'
 h+=f'assert_eq!(({expr}).to_bits(),0x{c["expected"]:08x},"native case {i}");passed+=1;}}\n'
h+='println!("native getter fixtures passed: {passed}");}\n'
(HERE/'getter_actual_source_probe.rs').write_text(h,encoding='utf8')
(HERE/'getter-source-hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [source,math]},indent=2))
