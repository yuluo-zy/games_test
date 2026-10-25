from pathlib import Path
import json,struct
r=Path(__file__).resolve().parent
cases=json.loads((r/'native-picker-all-fixtures.json').read_text())
def f(x):return f'f32::from_bits({struct.unpack("<I",struct.pack("<f",x))[0]})'
def v(x):return 'Vec'+str(len(x))+'::new('+','.join(f(y)for y in x)+')'
def fields(b,off,n):return struct.unpack_from('<'+'f'*n,b,off)
def geom(w):
 b=bytes.fromhex(w['bytes']);tag=b[0x88]
 if tag==0:return f'WidgetGeometry::Sphere{{center:{v(fields(b,0x90,3))},radius:{f(fields(b,0x8c,1)[0])}}}'
 if tag==1:return 'WidgetGeometry::SphereCloud{centers:vec!['+','.join(v(x)for x in (w['extra']or []))+f'],radius:{f(fields(b,0x8c,1)[0])}}}'
 if tag==2:return 'WidgetGeometry::TriangleMesh(vec!['+','.join('utils::geometry::intersect::Triangle::new('+','.join(v(x[i:i+3])for i in range(0,9,3))+')'for x in(w['extra']or []))+'])'
 if tag==3:return f'WidgetGeometry::ScreenAabb{{min:{v(fields(b,0x90,2))},max:{v(fields(b,0x98,2))},distance:{f(fields(b,0x8c,1)[0])}}}'
 if tag==4:return f'WidgetGeometry::WorldCanvas{{extent:{v(fields(b,0x8c,2))},layer:{b[0x89]}}}'
 if tag==5:return f'WidgetGeometry::TerrainCanvas{{distance:{f(fields(b,0x8c,1)[0])},layer:{b[0x89]}}}'
 if tag==6:return f'WidgetGeometry::PlaneDisc{{point:{v(fields(b,0x8c,3))},normal:{v(fields(b,0x98,3))},radius:{f(fields(b,0xa4,1)[0])}}}'
 return 'WidgetGeometry::Empty'
out=['use std::sync::Arc;use glam::{Vec2,Vec3};use utils::geometry::Ray;use systems::ui::{hover_intersection::{pick_chunk,PickerInput},resources::{widget_params::{WidgetParams,WidgetGeometry},ui_state::WidgetId}};']
out+=['pub use country_core::resources;mod systems {pub use country_core::systems::camera_rig;pub mod ui {pub mod resources {#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/widget_params.rs"]pub mod widget_params;#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/ui_state.rs"]pub mod ui_state;}#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/ui-picker/proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs"]pub mod hover_intersection;}}']
out+=['#[test]fn unchanged_original_picker_fixtures(){']
for i,c in enumerate(cases):
 widgets=[]
 for w in c['widgets']:
  b=bytes.fromhex(w['bytes']);widgets.append(f'(WidgetId({w["id"]}),Arc::new({{let mut p=WidgetParams::default().geometry({geom(w)});p.occlude_by_world={str(bool(b[0xa8])).lower()};p.flags.0={b[0xa9]};p}}))')
 inp=c['inputs'];terrain='Some('+v(inp['terrain'])+')'if inp.get('terrain')is not None else'None'
 out+=['{let widgets=vec!['+','.join(widgets)+'];',f'let input=PickerInput{{ray:Ray{{origin:{v(inp.get("origin",[0,0,0]))},direction:{v(inp.get("direction",[0,0,1]))}}},screen:{v(inp.get("screen",[0,0]))},terrain:{terrain}}};', 'let(a,b)=pick_chunk(&widgets,input);']
 for name,variable,offset in [('ordinary','a',0),('canvas','b',8)]:
  out+=[f'assert_eq!({variable}.len(),{len(c["result"][name])},"case{i} {c["name"]} {name} count");']
  for j,h in enumerate(c['result'][name]):
   expected=bytes.fromhex(h)[offset:offset+56];out+=['{let actual='+variable+f'[{j}]'+('.1'if offset else'')+'.to_native();let expected=['+','.join(str(y)for y in expected)+'];']
   out+=['assert_eq!(&actual[..4],&expected[..4]);assert_eq!(&actual[16..32],&expected[16..32]);assert_eq!(&actual[48..50],&expected[48..50]);']
   offs=[32,36,40,44]+([4,8,12]if struct.unpack_from('<I',expected)[0]==1 else[])
   for off in offs:out+=[f'{{let x=f32::from_le_bytes(actual[{off}..{off+4}].try_into().unwrap());let y=f32::from_le_bytes(expected[{off}..{off+4}].try_into().unwrap());assert!((x-y).abs()<=1e-6||x.to_bits()==y.to_bits(),"case{i} {c["name"]} {name} off{off}: {{x}} vs {{y}}");}}']
   out+=['}']
 out+=['}']
out+=['}']
out+=['#[test]fn unchanged_original_arbitration_fixtures(){', 'use systems::ui::hover_intersection::{arbitrate,WidgetIntersection};', 'fn decode(b:&[u8])->WidgetIntersection{let f=|i:usize| f32::from_le_bytes(b[i..i+4].try_into().unwrap());WidgetIntersection{normal:(u32::from_le_bytes(b[..4].try_into().unwrap())!=0).then(||Vec3::new(f(4),f(8),f(12))),widget:WidgetId(u64::from_le_bytes(b[16..24].try_into().unwrap())),element:u64::from_le_bytes(b[24..32].try_into().unwrap())as usize,distance:f(32),position:Vec3::new(f(36),f(40),f(44)),occlude_by_world:b[48]!=0,flags:b[49]}}']
for i,c in enumerate(json.loads((r/'native-arbitration-fixtures.json').read_text())):
 def dec(h):return 'decode(&['+','.join(str(b)for b in bytes.fromhex(h))+'])'
 inp=c['inputs'];out+=['{let ordinary=vec!['+','.join(dec(h)for h in c['ordinary'])+'];','let canvases=vec!['+','.join('('+str(l)+','+dec(h)+')'for l,h in c['canvases'])+'];']
 matrix='glam::Mat4::from_cols_array(&['+','.join(f(x)for x in inp['view'])+'])'if 'view'in inp else'glam::Mat4::IDENTITY'
 out+=[f'let ray=Ray{{origin:{v(inp.get("origin",[0,0,0]))},direction:{v(inp.get("direction",[0,0,1]))}}};',f'let actual=arbitrate(ordinary,canvases,ray,{matrix},{f(inp.get("depth",0))});']
 out+=['let expected=vec!['+','.join(dec(h)for h in c['result'])+'];',f'assert_eq!(actual,expected,"arbitration case{i} {c["name"]}");','}']
out+=['}']
(r/'direct_candidate.rs').write_text('\n'.join(out),encoding='utf8')
print('generated fixture count',len(cases))
