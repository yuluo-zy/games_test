from pathlib import Path
import json,struct
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[3]
source=(ROOT/'tiny-glade/crates/systems/toolbar/src/photomode.rs').read_text(encoding='utf8')
parts=source[source.index('pub fn slider_val_01('):source.index('/// `ui_slider_reset`')]
core=(ROOT/'tiny-glade/crates/systems/toolbar/src/resouces_and_components/mod.rs').read_text(encoding='utf8')
log=core[core.index('#[derive(Clone, Copy, Debug, PartialEq)]\npub struct LogSlider'):core.index('// ------------------------------------------------------------------\n// 26 个共享 UI 组件')]
f=lambda x:f'f32::from_bits(0x{struct.unpack("<I",struct.pack("<f",x))[0]:08x})'
header=(HERE/'getter_actual_source_probe.rs').read_text(encoding='utf8').split('pub fn slider_val_01(')[0]
header=header.replace('impl ImageFormationConfigs {fn current(&self)->&Row{&self.row}}','impl ImageFormationConfigs {fn current(&self)->&Row{&self.row} fn iter_non_default_mut(&mut self)->std::iter::Once<&mut Row>{std::iter::once(&mut self.row)}}')
h=header+log+'''enum AppMode {Path}
// These dependencies are outside the camera-only test boundary. Panic if the
// actual camera dispatch unexpectedly reaches an unrelated brush branch.
impl BrushSizes {fn set_path(&mut self,_:f32){panic!("brush branch")} fn set_water(&mut self,_:f32){panic!("brush branch")} fn set_garden(&mut self,_:f32){panic!("brush branch")}}
'''+parts+'\nfn main(){std::thread::Builder::new().stack_size(32*1024*1024).spawn(run).unwrap().join().unwrap();}\nfn run(){let mut passed=0;\n'
fo=[0x170,0x228,0x2a8,0x348,0x400,0x488]
for i,c in enumerate(json.loads((HERE/'camera-write-native-fixtures.json').read_text())):
 m=c['mode'];fm=c['focus_mode'];k=c['kind'];v=c['value'];lens=0x100+[0x8c,0x144,0x1c4,0x264,0x31c,0x3a4][m]
 h+='{let mut camera=MainCamera{raw:[0;0x4e0]};'+f'camera.raw[0x4d0]={m};\n'
 for o,val in [(0xc,10.),(0x10,100.),(8,73.),(0x1c,.25),(0x20,8.),(0x18,3.5)]:h+=f'camera.write_f32({lens+o},{f(val)});\n'
 h+=f'camera.write_f32({fo[m]},f32::from_bits({fm}));\n'
 for o,val in [(4,-10.),(8,25.),(0x10,17.),(0x14,-23.),(0x18,.125)]:h+=f'camera.write_f32({fo[m]+o},{f(val)});\n'
 h+='let mut formation=ImageFormationConfigs{row:Row{params_18:[0.;4],params_30:[0.;2],word_38:0}};let mut brushes=BrushSizes{path:0.,water:0.,garden:0.};\n'
 h+=f'assert!(apply_slider_value({k},{f(v)},&mut formation,&mut brushes,true,AppMode::Path,&mut camera));\n'
 expected=bytes.fromhex(c['camera']);h+='let expected=['+','.join(str(x) for x in expected)+'];\n'
 h+=f'assert_eq!(camera.raw,expected,"native camera-write case {i}");passed+=1;}}\n'
h+='println!("native camera write fixtures passed: {passed}");}\n'
(HERE/'camera_write_actual_source_probe.rs').write_text(h,encoding='utf8')
