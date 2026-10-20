#![allow(dead_code)]
use glam::Vec2;
struct Aabb2 {min:Vec2,max:Vec2}
impl Aabb2 {pub fn from_center_dims(center: Vec2, dimensions: Vec2) -> Self {
        let h = dimensions * 0.5;
        Self {
            min: center - h,
            max: center + h,
        }
    }pub fn contains(self, p: Vec2) -> bool {
        self.min.x <= p.x && p.x <= self.max.x && self.min.y <= p.y && p.y <= self.max.y
    }}
#[path="D:/game/tiny-glade/crates/utils/src/easing_functions.rs"] mod easing_functions;
use easing_functions::EasingFunction;
pub fn undo_activation_geometry(alignment: u8, window: Vec2, icon_scale: f32) -> (Vec2, Vec2) {
    let button = 110. * icon_scale;
    let total = 30. * icon_scale + 4. * button;
    let (center, dimensions) = match alignment {
        0 => (
            Vec2::new(window.x * 0.5, button * 0.5),
            Vec2::new(total, button),
        ),
        1 => (
            Vec2::new(window.x - button * 0.5, window.y * 0.5),
            Vec2::new(button, total),
        ),
        2 => (
            Vec2::new(button * 0.5, window.y * 0.5),
            Vec2::new(button, total),
        ),
        3 => (
            Vec2::new(window.x * 0.5, window.y - button * 0.5),
            Vec2::new(total, button),
        ),
        _ => unreachable!("native ToolbarAlignment"),
    };
    let margin = 40. * icon_scale;
    (center, dimensions + Vec2::splat(margin + margin))
}
pub fn undo_redo_offset(alignment: u8, icon_scale: f32, progress: f32) -> Vec2 {
    let offset = (icon_scale * 100.) * EasingFunction::CubicInOut.apply(progress);
    match alignment {
        0 => Vec2::new(0., -offset),
        1 => Vec2::new(offset, 0.),
        2 => Vec2::new(-offset, 0.),
        3 => Vec2::new(0., offset),
        _ => unreachable!("native ToolbarAlignment"),
    }
}
pub fn auto_hide_progress(
    old: f32,
    dt: f32,
    inside_or_keep: bool,
    menu_closed: bool,
    action_pressed: bool,
) -> f32 {
    let next = if !inside_or_keep || !menu_closed {
        // Cursor outside the activation zone, or a menu is open → hide.
        old + dt / 0.3
    } else if !action_pressed {
        // Cursor inside, menu closed and click not held → show.
        old + dt / (-0.3)
    } else {
        old
    };
    next.clamp(0.0, 1.0)
}
pub fn auto_hide_sound_kind(old: f32, new: f32) -> Option<u32> {
    if old == 0.0 && new > 0.0 {
        Some(0x10)
    } else if old == 1.0 && new < 1.0 {
        Some(0x11)
    } else {
        None
    }
}
struct Config {ui:UiConfig} struct UiConfig {toolbar_alignment:u8,auto_hide_toolbar:bool,icon_size_mult:f32}
struct Clock(f32);impl Clock {fn delta_secs(&self)->f32{self.0}}
struct WindowSize(u32,u32);impl WindowSize {fn as_vec2(&self)->Vec2{Vec2::new(self.0 as f32,self.1 as f32)}}
struct Menu(bool);impl Menu {fn is_closed(&self)->bool{self.0}}
struct Digital(u32);impl Digital {fn pressed(&self)->bool{self.0<3}}
struct Actions {digital:[Digital;1]} struct Cursor(Vec2);
struct Undo {screen:u64,hide_progress:f32} struct Anchor {node:u64,offset:Vec2} struct Taffy {anchored:Vec<Anchor>}
struct PlayUiSound {kind:u32,_payload:[u8;16],id:u64} struct Sounds(Vec<u32>);impl Sounds {fn write(&mut self,event:PlayUiSound){self.0.push(event.kind);}}
fn frame(cursor:Cursor,menu:Menu,config:Config,time:Clock,window:WindowSize,actions:Actions,undo:&mut Undo,taffy:&mut Taffy,sounds:&mut Sounds)
{
    let alignment = config.ui.toolbar_alignment as u8;
    if config.ui.auto_hide_toolbar {
        let (center, dimensions) =
            undo_activation_geometry(alignment, window.as_vec2(), config.ui.icon_size_mult);
        let inside = Aabb2::from_center_dims(center, dimensions).contains(cursor.0);
        let old = undo.hide_progress;
        undo.hide_progress = auto_hide_progress(
            old,
            time.delta_secs(),
            inside,
            menu.is_closed(),
            actions.digital[0].pressed(),
        );
        if let Some(kind) = auto_hide_sound_kind(old, undo.hide_progress) {
            sounds.write(PlayUiSound {
                kind,
                _payload: [0; 0x10],
                id: 0,
            });
        }
    } else {
        undo.hide_progress = 0.;
    }
    let offset = undo_redo_offset(alignment, config.ui.icon_size_mult, undo.hide_progress);
    if let Some(anchor) = taffy
        .anchored
        .iter_mut()
        .find(|entry| entry.node == undo.screen)
    {
        anchor.offset = offset;
    }
}
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
