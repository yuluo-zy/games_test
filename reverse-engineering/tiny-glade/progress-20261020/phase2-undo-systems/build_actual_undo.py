from pathlib import Path
import hashlib,json,subprocess
H=Path(__file__).resolve().parent;R=H.parents[3]
paths={k:R/v for k,v in {'undo':'tiny-glade/crates/systems/toolbar/src/undo_redo_systems.rs','toolbar':'tiny-glade/crates/systems/toolbar/src/toolbar_systems.rs','easing':'tiny-glade/crates/utils/src/easing_functions.rs','aabb':'tiny-glade/crates/utils/src/geometry/aabb.rs'}.items()};sources={k:v.read_text(encoding='utf-8') for k,v in paths.items()}
def fn(s,name):
 a=s.index('pub fn '+name+'(');b=s.index('{',a);d=1;e=b+1
 while d:d+=(s[e]=='{')-(s[e]=='}');e+=1
 return s[a:e],s[b:e]
p='''#![allow(dead_code)]
use glam::Vec2;
struct Aabb2 {min:Vec2,max:Vec2}
impl Aabb2 {'''+fn(sources['aabb'],'from_center_dims')[0]+fn(sources['aabb'],'contains')[0]+'''}
#[path="'''+paths['easing'].as_posix()+'''"] mod easing_functions;
use easing_functions::EasingFunction;
'''
for src,name in [('undo','undo_activation_geometry'),('undo','undo_redo_offset'),('toolbar','auto_hide_progress'),('toolbar','auto_hide_sound_kind')]:p+=fn(sources[src],name)[0]+'\n'
p+='''struct Config {ui:UiConfig} struct UiConfig {toolbar_alignment:u8,auto_hide_toolbar:bool,icon_size_mult:f32}
struct Clock(f32);impl Clock {fn delta_secs(&self)->f32{self.0}}
struct WindowSize(u32,u32);impl WindowSize {fn as_vec2(&self)->Vec2{Vec2::new(self.0 as f32,self.1 as f32)}}
struct Menu(bool);impl Menu {fn is_closed(&self)->bool{self.0}}
struct Digital(u32);impl Digital {fn pressed(&self)->bool{self.0<3}}
struct Actions {digital:[Digital;1]} struct Cursor(Vec2);
struct Undo {screen:u64,hide_progress:f32} struct Anchor {node:u64,offset:Vec2} struct Taffy {anchored:Vec<Anchor>}
struct PlayUiSound {kind:u32,_payload:[u8;16],id:u64} struct Sounds(Vec<u32>);impl Sounds {fn write(&mut self,event:PlayUiSound){self.0.push(event.kind);}}
fn frame(cursor:Cursor,menu:Menu,config:Config,time:Clock,window:WindowSize,actions:Actions,undo:&mut Undo,taffy:&mut Taffy,sounds:&mut Sounds)
'''+fn(sources['undo'],'undo_redo_auto_hide')[1]+'''
fn n(b:&[u8],a:usize)->u32{u32::from_le_bytes(b[a..a+4].try_into().unwrap())}
fn f(b:&[u8],a:usize)->f32{f32::from_bits(n(b,a))}
fn main(){
 let fixture=include_bytes!("native-auto-hide.bin");
 for (i,b) in fixture.chunks_exact(52).enumerate(){
  let mut undo=Undo{screen:0x2200000001,hide_progress:f(b,20)};let mut taffy=Taffy{anchored:vec![Anchor{node:undo.screen,offset:Vec2::ZERO}]};let mut sounds=Sounds(vec![]);
  frame(Cursor(Vec2::new(f(b,28),f(b,32))),Menu(b[2]==2||b[2]==10&&b[3]==0),Config{ui:UiConfig{toolbar_alignment:b[0],auto_hide_toolbar:b[1]!=0,icon_size_mult:f(b,16)}},Clock(f(b,24)),WindowSize(n(b,8),n(b,12)),Actions{digital:[Digital(n(b,4))]},&mut undo,&mut taffy,&mut sounds);
  let actual=[undo.hide_progress.to_bits(),taffy.anchored[0].offset.x.to_bits(),taffy.anchored[0].offset.y.to_bits(),sounds.0.first().copied().unwrap_or(u32::MAX)];
  let expected=[n(b,36),n(b,40),n(b,44),n(b,48)];assert_eq!(actual,expected,"native frame{i}");
 }
 println!("{} actual production undo frames exactly match full native original",fixture.len()/52);
}
'''
(H/'actual_undo_source.rs').write_text(p,encoding='utf-8');(H/'actual-source-manifest.json').write_text(json.dumps({'production_files':{k:hashlib.sha256(v.read_bytes()).hexdigest() for k,v in paths.items()},'business_body_replacements':0,'boundary_containers':'standalone named field owners, event collector, time/input/window getters'},indent=2))
rustc='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe';glam=R/'reverse-engineering/tiny-glade/progress-20261020/toolbar-layout/libglam.rlib'
subprocess.run([rustc,'--edition','2021',str(H/'actual_undo_source.rs'),'--extern',f'glam={glam}','-o',str(H/'actual_undo_source.exe')],check=True)
r=subprocess.run([str(H/'actual_undo_source.exe')],capture_output=True,text=True);print(r.stdout+r.stderr);(H/'actual-source-results.txt').write_text(r.stdout+r.stderr);raise SystemExit(r.returncode)
