from pathlib import Path
import hashlib,json
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[3]
src=ROOT/'tiny-glade/crates/systems/toolbar/src/photomode/ui_slider.rs'
math=ROOT/'tiny-glade/crates/systems/toolbar/src/photomode/slider_math.rs'
easing=ROOT/'tiny-glade/crates/utils/src/easing_functions.rs'
aabb=ROOT/'tiny-glade/crates/utils/src/geometry/aabb.rs'
action=ROOT/'tiny-glade/crates/country-core/src/input/action_set.rs'
s=src.read_text(encoding='utf8');act=action.read_text(encoding='utf8');ab=aabb.read_text(encoding='utf8')
def fn(text,name):
 start=text.index('    pub fn '+name+'(');brace=text.index('{',start);depth=1;i=brace+1
 while depth:
  depth+=(text[i]=='{')-(text[i]=='}');i+=1
 return text[start:i]
head='''use glam::Vec2;
#[derive(Clone,Copy,Debug,PartialEq,Eq)] struct UiFocus {tag:u32,_pad:u32,payload:u64}
struct Ui2dExtent {pos:Vec2,size:Vec2}
struct Aabb2 {min:Vec2,max:Vec2}
impl Aabb2 { AABB }
#[path="MATH"] mod slider_math;
#[path="EASING"] mod easing_functions;
use easing_functions::EasingFunction;
'''.replace('AABB',fn(ab,'from_center_dims')+'\n'+fn(ab,'contains')).replace('MATH',math.as_posix()).replace('EASING',easing.as_posix())
head+=act[act.index('#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]'):act.index('/// 一条动作记录')]
head+=s[s.index('#[derive(Default, Debug)]'):s.index('/// The original sixteen SystemParams')]
a=s.index('        if drag.state == 3');b=s.index('\n    }\n\n    if let Some((entity, fraction))',a)
head+='''
struct CursorPositionSS(Vec2);struct UiDragEvent{state:u8}
fn drag_case(shape:SliderGeometry,fraction:f32,cursor:CursorPositionSS,state:u8,grab:&mut GrabOffset)->(Option<f32>,Option<f32>,bool){
 let drag=UiDragEvent{state};let entity=1u64;let mut candidate=None;let mut category_candidate=None;
'''+s[a:b]+'''
 ((*grab).0,candidate.map(|(_,value)|value),category_candidate.is_some())
}
'''
head+='''
fn f(b:&[u8],o:usize)->f32 {f32::from_le_bytes(b[o..o+4].try_into().unwrap())}
fn n(b:&[u8],o:usize)->u32 {u32::from_le_bytes(b[o..o+4].try_into().unwrap())}
fn phase(n:u32)->DigitalActionPhase {match n {0=>DigitalActionPhase::JustPressed,1=>DigitalActionPhase::Held,2=>DigitalActionPhase::Repeating,3=>DigitalActionPhase::JustReleased,4=>DigitalActionPhase::Idle,_=>panic!()}}
fn main(){
let visual=include_bytes!("visual-native.bin");
for (i,b) in visual.chunks_exact(52).enumerate(){
 let shape=geometry(Vec2::new(f(b,0),f(b,4)),Vec2::new(f(b,8),f(b,12)));
 let (hover,knob)=visual_state(shape,f(b,16),f(b,20),f(b,24),Vec2::new(f(b,28),f(b,32)));
 let actual=[hover,knob.pos.x,knob.pos.y,knob.size.x];
 for (j,v) in actual.into_iter().enumerate(){assert_eq!(v.to_bits(),n(b,36+j*4),"visual {i} field{j}");}
}
println!("visual original instruction cases passed: {}",visual.len()/52);
let navigation=include_bytes!("navigation-native.bin");
for (i,b) in navigation.chunks_exact(40).enumerate(){
 let entity=0x200000001;let mut focus=UiFocus{tag:1,_pad:0,payload:entity};let mut previous=focus;
 match b[13] {0=>{},1=>previous=UiFocus{tag:0,_pad:0,payload:0},2=>focus=UiFocus{tag:0,_pad:0,payload:0},3=>focus.payload=0x200000002,_=>panic!()}
 let mut armed=b[12]!=0;
 let (next,category)=navigation_input(entity,focus,previous,DigitalActionState{phase:phase(n(b,0)),timer_seconds:f(b,8)},DigitalActionState{phase:phase(n(b,4)),timer_seconds:f(b,8)},f(b,20),f(b,16),&mut armed);
 assert_eq!(armed as u32,n(b,24),"navigation {i} armed");
 assert_eq!(next.is_some() as u32,n(b,28),"navigation {i} candidate");
 if let Some(next)=next{assert_eq!(next.to_bits(),n(b,32),"navigation {i} fraction");}
 assert_eq!(category as u32,n(b,36),"navigation {i} category");
}
println!("navigation original instruction cases passed: {}",navigation.len()/40);
let drags=include_bytes!("drag-native.bin");
for (i,b) in drags.chunks_exact(40).enumerate(){
 let mut grab=GrabOffset((n(b,4)!=0).then_some(f(b,8)));
 let shape=geometry(Vec2::new(100.,100.),Vec2::new(400.,40.));
 let (grab,next,category)=drag_case(shape,f(b,12),CursorPositionSS(Vec2::new(f(b,16),100.)),n(b,0) as u8,&mut grab);
 assert_eq!(grab.is_some() as u32,n(b,20),"drag {i} grab state");
 if let Some(grab)=grab{assert_eq!(grab.to_bits(),n(b,24),"drag {i} offset");}
 assert_eq!(next.is_some() as u32,n(b,28),"drag {i} candidate");
 if let Some(next)=next{assert_eq!(next.to_bits(),n(b,32),"drag {i} fraction");}
 assert_eq!(category as u32,n(b,36),"drag {i} category");
}
println!("drag original instruction cases passed: {}",drags.len()/40);
let categories=include_bytes!("category-native.bin");
for (i,b) in categories.chunks_exact(16).enumerate(){
 let request=requested_category(n(b,0) as u8,n(b,4));
 assert_eq!(request.is_some() as u32,n(b,8),"category {i} emission");
 if let Some(request)=request{assert_eq!(request as u32,n(b,12),"category {i} payload");}
}
println!("category original instruction cases passed: {}",categories.len()/16);
}
'''
(HERE/'actual_source_probe.rs').write_text(head,encoding='utf8')
(HERE/'actual-source-manifest.json').write_text(json.dumps({str(x.relative_to(ROOT)):hashlib.sha256(x.read_bytes()).hexdigest() for x in [src,math,easing,aabb,action]},indent=2))
