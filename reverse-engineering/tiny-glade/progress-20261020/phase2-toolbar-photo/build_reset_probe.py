from pathlib import Path
import json,struct
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[3]
p=ROOT/'tiny-glade/crates/systems/toolbar/src/photomode.rs';s=p.read_text(encoding='utf8')
shared=(ROOT/'tiny-glade/crates/systems/toolbar/src/resouces_and_components/mod.rs').read_text(encoding='utf8')
brush=(ROOT/'tiny-glade/crates/country-core/src/resources/brush_sizes.rs').read_text(encoding='utf8')
def fn(t,name):
 a=t.index('fn '+name+'(');b=t.index('{',a);d=1;i=b+1
 while d:d+=(t[i]=='{')-(t[i]=='}');i+=1
 return t[a:i]
records=bytearray()
for x in json.loads((HERE/'reset-native.json').read_text()):
 records+=struct.pack('<3I',x['kind'],x['mode'],x['focus_mode'])+bytes.fromhex(x['config'])+bytes.fromhex(x['camera'])+bytes.fromhex(x['brushes'])
(HERE/'reset-native.bin').write_bytes(records)
h='''const FOV_OFFSETS:[usize;6]=[0x8c,0x144,0x1c4,0x264,0x31c,0x3a4];
struct MainCamera {raw:[u8;0x4e0]}
impl MainCamera {
fn read_f32(&self,o:usize)->f32{f32::from_le_bytes(self.raw[o..o+4].try_into().unwrap())}
fn write_f32(&mut self,o:usize,v:f32){self.raw[o..o+4].copy_from_slice(&v.to_le_bytes())}
fn bytes(&self)->&[u8]{&self.raw}fn mode_byte(&self)->u8{self.raw[0x4d0]}}
#[derive(Clone)]struct Row{params_18:[f32;4],params_30:[f32;2],word_38:u32}
struct ImageFormationConfigs{configs:[Row;3]}
impl ImageFormationConfigs{fn get(&self,n:usize)->Option<&Row>{self.configs.get(n)} fn iter_non_default_mut(&mut self)->impl Iterator<Item=&mut Row>{self.configs.iter_mut().skip(1)}}
'''
h+=brush[brush.index('pub const PATH_DEFAULT'):brush.index('#[cfg(test)]')].replace('Resource, ','')
h+=shared[shared.index('#[derive(Clone, Copy, Debug, PartialEq)]\npub struct LogSlider'):shared.index('// ------------------------------------------------------------------\n// 26 个共享 UI 组件')]
h+='\n'+fn(s,'focus_offset')+'\n'+fn(s,'set_focus_distance')+'\n'+fn(s,'reset_slider')
h+='''
fn n(b:&[u8],o:usize)->u32{u32::from_le_bytes(b[o..o+4].try_into().unwrap())}
fn main(){let data=include_bytes!("reset-native.bin");
for (i,b) in data.chunks_exact(1472).enumerate(){
let kind=n(b,0) as u8;let mode=n(b,4) as usize;let focus_mode=n(b,8);
let mut camera=MainCamera{raw:[0;0x4e0]};camera.raw[0x4d0]=mode as u8;
let lens=0x100+FOV_OFFSETS[mode];let focus=focus_offset(&camera);
for (o,v) in [(8,12.),(0x14,73.),(0x18,2.5),(0x24,7.5)]{camera.write_f32(lens+o,v);}
camera.write_f32(focus,f32::from_bits(focus_mode));
for (o,v) in [(4,-10.),(8,25.),(0x10,17.),(0x14,-23.),(0x18,0.125)]{camera.write_f32(focus+o,v);}
let mut formation=ImageFormationConfigs{configs:[
Row{params_18:[0.,-0.75,1.25,0.85],params_30:[0.6,0.005],word_38:0.07f32.to_bits()},
Row{params_18:[0.,10.,20.,30.],params_30:[40.,60.],word_38:50.f32.to_bits()},
Row{params_18:[0.,11.,21.,31.],params_30:[41.,61.],word_38:51.f32.to_bits()}]};
let mut brushes=BrushSizes{path:0.1,water:0.2,garden:0.3};
reset_slider(kind,&mut formation,&mut camera,&mut brushes);
assert_eq!(camera.raw,&b[212..1460],"reset {i} camera");
for (row,c) in formation.configs.iter().enumerate(){
let values=[c.params_18[1].to_bits(),c.params_18[2].to_bits(),c.params_18[3].to_bits(),c.params_30[0].to_bits(),c.word_38,c.params_30[1].to_bits()];
for (v,offset) in values.into_iter().zip([0x1c,0x20,0x24,0x30,0x38,0x34]){assert_eq!(v,n(b,12+row*64+offset),"reset {i} config row{row} offset{offset}");}}
for (j,v) in [brushes.path,brushes.water,brushes.garden].into_iter().enumerate(){assert_eq!(v.to_bits(),n(b,1460+j*4),"reset {i} brushes {j}");}
}
println!("native reset parameter cases passed: {}",data.len()/1472);}
'''
h=h.replace('50.f32.to_bits()','50.0f32.to_bits()').replace('51.f32.to_bits()','51.0f32.to_bits()')
(HERE/'reset_actual_source_probe.rs').write_text(h,encoding='utf8')
