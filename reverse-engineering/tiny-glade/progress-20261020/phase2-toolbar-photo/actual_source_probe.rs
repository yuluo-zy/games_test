use glam::Vec2;
#[derive(Clone,Copy,Debug,PartialEq,Eq)] struct UiFocus {tag:u32,_pad:u32,payload:u64}
struct Ui2dExtent {pos:Vec2,size:Vec2}
struct Aabb2 {min:Vec2,max:Vec2}
impl Aabb2 {     pub fn from_center_dims(center: Vec2, dimensions: Vec2) -> Self {
        let h = dimensions * 0.5;
        Self {
            min: center - h,
            max: center + h,
        }
    }
    pub fn contains(self, p: Vec2) -> bool {
        self.min.x <= p.x && p.x <= self.max.x && self.min.y <= p.y && p.y <= self.max.y
    } }
#[path="D:/game/tiny-glade/crates/systems/toolbar/src/photomode/slider_math.rs"] mod slider_math;
#[path="D:/game/tiny-glade/crates/utils/src/easing_functions.rs"] mod easing_functions;
use easing_functions::EasingFunction;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum DigitalActionPhase {
    JustPressed = 0,
    Held = 1,
    Repeating = 2,
    JustReleased = 3,
    #[default]
    Idle = 4,
}

/// 持续状态记录已按住的时间；连发状态记录距离下一次触发的时间。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DigitalActionState {
    pub phase: DigitalActionPhase,
    pub timer_seconds: f32,
}

impl DigitalActionState {
    pub fn pressed(self) -> bool {
        matches!(
            self.phase,
            DigitalActionPhase::JustPressed
                | DigitalActionPhase::Held
                | DigitalActionPhase::Repeating
        )
    }
    pub fn released(self) -> bool {
        !self.pressed()
    }
    pub fn just_pressed(self) -> bool {
        self.phase == DigitalActionPhase::JustPressed
            || (self.phase == DigitalActionPhase::Repeating && self.timer_seconds <= 0.0)
    }
    pub fn just_released(self) -> bool {
        self.phase == DigitalActionPhase::JustReleased
    }
}

#[derive(Default, Debug)]
pub struct GrabOffset(Option<f32>);

#[derive(Clone, Copy, Debug)]
pub struct SliderGeometry {
    pub lower: f32,
    pub upper: f32,
    pub y: f32,
    pub side: f32,
}

pub fn geometry(center: Vec2, dimensions: Vec2) -> SliderGeometry {
    let side = dimensions.y * 0.3;
    let half_travel = (side * -2.5 + dimensions.x) * 0.5;
    SliderGeometry {
        lower: center.x - half_travel,
        upper: center.x + half_travel,
        y: center.y,
        side,
    }
}

impl SliderGeometry {
    pub fn knob_center(self, fraction: f32) -> Vec2 {
        Vec2::new(
            fraction * self.upper + (1.0 - fraction) * self.lower,
            self.y,
        )
    }
    pub fn hit_knob(self, fraction: f32, cursor: Vec2) -> bool {
        Aabb2::from_center_dims(self.knob_center(fraction), Vec2::splat(self.side * 1.3))
            .contains(cursor)
    }
    pub fn fraction_at(self, cursor_x: f32, offset: f32) -> f32 {
        slider_math::clamp_01(slider_math::inverse_lerp(
            self.lower,
            self.upper,
            cursor_x + offset,
        ))
    }
}

pub fn visual_state(
    shape: SliderGeometry,
    fraction: f32,
    hover: f32,
    delta: f32,
    cursor: Vec2,
) -> (f32, Ui2dExtent) {
    let step = delta / 0.2;
    let hover = slider_math::clamp_01(
        hover
            + if shape.hit_knob(fraction, cursor) {
                step
            } else {
                -step
            },
    );
    // Original static EasingFunction discriminant 6 at 14304ed20.
    let eased = EasingFunction::CubicOut.apply(hover);
    let side = shape.side * (eased * 1.3 + (1.0 - eased));
    (
        hover,
        Ui2dExtent {
            pos: shape.knob_center(fraction),
            size: Vec2::splat(side),
        },
    )
}

/// Original f32->u64 saturation and truncation before comparing 14 bins.
pub fn sound_bucket(fraction: f32) -> u64 {
    (fraction * 14.0) as u64
}

/// Original final four-way switch 142349a85..142349b3c. The payload is a
/// toolbar-category command, despite its ICF EventInstance<WindowFocusEvent>.
pub fn requested_category(kind: u8, mode_tag: u32) -> Option<u8> {
    match kind {
        9 if mode_tag != 9 => Some(4),
        10 if !(7..9).contains(&mode_tag) => Some(1),
        11 if mode_tag != 15 => Some(8),
        12 if mode_tag != 17 => Some(9),
        _ => None,
    }
}

/// Native focused-slider navigation block 142348cfc..142348ef9.
#[allow(clippy::too_many_arguments)]
pub fn navigation_input(entity_bits: u64, focus: UiFocus, previous: UiFocus,
    left: DigitalActionState, right: DigitalActionState, delta: f32,
    fraction: f32, armed: &mut bool) -> (Option<f32>, bool)
{
    if focus.tag != 1 || focus.payload != entity_bits { return (None, false); }
    if (left.just_pressed() || right.just_pressed()) && previous == focus { *armed = true; }
    if left.released() && right.released() {
        *armed = false;
        (None, false)
    } else if *armed {
        let mut next = None;
        if left.pressed() { next = Some(slider_math::clamp_01(fraction - delta)); }
        if right.pressed() { next = Some(slider_math::clamp_01(fraction + delta)); }
        (next, left.just_pressed() || right.just_pressed())
    } else { (None, false) }
}


struct CursorPositionSS(Vec2);struct UiDragEvent{state:u8}
fn drag_case(shape:SliderGeometry,fraction:f32,cursor:CursorPositionSS,state:u8,grab:&mut GrabOffset)->(Option<f32>,Option<f32>,bool){
 let drag=UiDragEvent{state};let entity=1u64;let mut candidate=None;let mut category_candidate=None;
        if drag.state == 3 {
            (*grab).0 = Some(if shape.hit_knob(fraction, cursor.0) {
                shape.knob_center(fraction).x - cursor.0.x
            } else {
                0.0
            });
        }
        if drag.state < 2 {
            category_candidate = Some(entity);
        }
        if let Some(offset) = (*grab).0 {
            candidate = Some((entity, shape.fraction_at(cursor.0.x, offset)));
        }
 ((*grab).0,candidate.map(|(_,value)|value),category_candidate.is_some())
}

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
