//! Game-owned edit_roof rules recovered from RE-MCP decompilation and original
//! instructions. ECS lookup/MessageWriter storage is outside this module.
use glam::{Vec2,Vec3};

/// Observed discriminants are 0=Keep, 1=Absolute, 2=Delta. The original profile
/// branch treats any other nonzero value as Delta; other fields as Absolute.
#[derive(Clone,Copy,Debug)]
pub struct ScalarEdit {pub mode:u32,pub value:f32}
#[derive(Clone,Copy,Debug)]
pub struct TipEdit {pub mode:u32,pub value:Vec2}
#[derive(Clone,Copy,Debug)]
pub struct RoofEditEvent {
    pub profile:ScalarEdit,pub height:ScalarEdit,pub ridge:ScalarEdit,pub eave:ScalarEdit,
    pub wall_id:u64,pub tip:TipEdit,pub source_flag:u8,
}
impl RoofEditEvent {
    pub fn from_observed(b:&[u8;64])->Self {
        let u=|o|u32::from_le_bytes(b[o..o+4].try_into().unwrap());let f=|o|f32::from_bits(u(o));
        let scalar=|o|ScalarEdit{mode:u(o),value:f(o+4)};
        Self{profile:scalar(0),height:scalar(8),ridge:scalar(16),eave:scalar(24),
            wall_id:u64::from_le_bytes(b[32..40].try_into().unwrap()),tip:TipEdit{mode:u(40),value:Vec2::new(f(44),f(48))},source_flag:b[52]}
    }
}

#[derive(Clone,Copy,Debug)]
pub struct EditChanges {pub profile:f32,pub height:f32,pub ridge:f32,pub eave:f32,pub tip_distance:f32}
#[derive(Clone,Copy,Debug)]
pub enum EditFeedback {
    GableEnter {pivot:Vec3}, // observed event tag 10
    GableLeave {pivot:Vec3}, // observed event tag 11
    Scalar {gable_after:bool,ceil_height:f32,ceil_other:f32,weight:f32,pivot:Vec3}, // tag 2
    Tip {current_distance:f32,change_distance:f32,pivot:Vec3}, // tag 3
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum HeightCrossing {Down,Up}
#[derive(Clone,Debug)]
pub struct EditOutcome {
    /// Active mode bits, not a comparison of the resulting float values.
    pub changed_fields:u8,pub changes:EditChanges,pub previous_height:f32,pub new_height:f32,
    /// These are sampled AFTER height/eave edits, immediately before/after ridge.
    pub gable_before_ridge:bool,pub gable_after_ridge:bool,pub rebuild_gable:bool,
    pub feedback:Vec<EditFeedback>,pub height_crossing:Option<HeightCrossing>,
    /// Every successfully resolved event requests this notification, even Keep.
    pub resolved_update:bool,
}
fn get(b:&[u8;88],o:usize)->f32 {f32::from_le_bytes(b[o..o+4].try_into().unwrap())}
fn put(b:&mut[u8;88],o:usize,v:f32){b[o..o+4].copy_from_slice(&v.to_le_bytes());}
fn maxss(a:f32,b:f32)->f32 {if a>b {a}else{b}}
fn minss(a:f32,b:f32)->f32 {if a<b {a}else{b}}
fn clamp_observed(v:f32,lo:f32)->f32 {minss(1.0,maxss(lo,v))}
fn unordered_ne_zero(v:f32)->bool {!v.is_nan() && v!=0.0}
fn gable(b:&[u8;88])->bool {(get(b,0x30)>0.1||get(b,0x30).is_nan())&&get(b,0x34)==1.0}
fn limited_tip(v:Vec2)->Vec2 {
    // Exact pinned dependency API: glam 0.29.3 performs division by sqrt,
    // matching the original DIVPS branch. The game-owned bound is 1.0.
    v.clamp_length_max(1.0)
}
fn pivot(b:&[u8;88])->Vec3 {
    let tag=u32::from_le_bytes(b[8..12].try_into().unwrap()) as usize;
    Vec3::new(get(b,0x0c+tag*8),get(b,0x4c),get(b,0x10+tag*8))
}

/// Complete resolved parameter-edit core and its numerical notification rules.
/// `changed_tick` is the Roof component's change marker, passed after ECS lookup.
pub fn apply_edit(roof:&mut[u8;88],event:&RoofEditEvent,changed_tick:&mut u32,current_tick:u32)->EditOutcome {
    let mut mask=0u8;let mut changes=EditChanges{profile:0.0,height:0.0,ridge:0.0,eave:0.0,tip_distance:0.0};
    let mut mark=|bit:u8|{mask|=bit;*changed_tick=current_tick;};
    if event.profile.mode!=0 {
        mark(1);let old=get(roof,0x2c);
        let value=if event.profile.mode==1 {changes.profile=event.profile.value-old;event.profile.value}
            else {changes.profile=event.profile.value;old+event.profile.value};
        put(roof,0x2c,clamp_observed(value,0.0));
    }
    let previous_height=get(roof,0x30);
    if event.height.mode!=0 {
        mark(2);let (value,lo)=if event.height.mode==2 {changes.height=event.height.value;(previous_height+event.height.value,0.1)}
            else {changes.height=event.height.value-previous_height;(event.height.value,0.0)};
        put(roof,0x30,clamp_observed(value,lo));
    }
    let new_height=get(roof,0x30);
    if event.tip.mode!=0 {
        mark(16);let old=Vec2::new(get(roof,0x24),get(roof,0x28));
        let (value,delta)=if event.tip.mode==2 {(limited_tip(old+event.tip.value),event.tip.value)}
            else {(event.tip.value,event.tip.value-limited_tip(old))};
        put(roof,0x24,value.x);put(roof,0x28,value.y);changes.tip_distance=delta.length();
    }
    if event.eave.mode!=0 {
        mark(8);let old=get(roof,0x38);
        if event.eave.mode==2 {put(roof,0x38,clamp_observed(old+event.eave.value,0.0));changes.eave=event.eave.value;}
        else {put(roof,0x38,event.eave.value);changes.eave=event.eave.value-clamp_observed(old,0.0);}
    }
    let gable_before_ridge=gable(roof);
    if event.ridge.mode!=0 {
        mark(4);let old=get(roof,0x34);
        if event.ridge.mode==2 {put(roof,0x34,clamp_observed(old+event.ridge.value,0.0));changes.ridge=event.ridge.value;}
        else {put(roof,0x34,event.ridge.value);changes.ridge=event.ridge.value-clamp_observed(old,0.0);}
    }
    let gable_after_ridge=gable(roof);
    let rebuild_gable=gable_before_ridge!=gable_after_ridge ||
        ((unordered_ne_zero(changes.profile)||event.height.mode!=0)&&gable_after_ridge) ||
        (unordered_ne_zero(changes.eave)&&gable_after_ridge);
    let mut feedback=Vec::new();
    if gable_before_ridge && !gable_after_ridge {feedback.push(EditFeedback::GableLeave{pivot:pivot(roof)});}
    if !gable_before_ridge && gable_after_ridge {feedback.push(EditFeedback::GableEnter{pivot:pivot(roof)});}
    if [changes.profile,changes.height,changes.ridge,changes.eave].iter().any(|&v|unordered_ne_zero(v)) {
        let h=changes.height.abs().ceil();let p=changes.profile.abs().ceil();let r=changes.ridge.abs().ceil();let e=changes.eave.abs().ceil();
        let weight=if h>0.0 {changes.height.abs()} else {(changes.ridge.abs()+changes.profile.abs())+changes.eave.abs()};
        feedback.push(EditFeedback::Scalar{gable_after:gable_after_ridge,ceil_height:h,ceil_other:e+(r+p),weight,pivot:pivot(roof)});
    }
    if changes.tip_distance>0.0 {
        let tip=Vec2::new(get(roof,0x24),get(roof,0x28));
        feedback.push(EditFeedback::Tip{current_distance:tip.length(),change_distance:changes.tip_distance,pivot:pivot(roof)});
    }
    let height_crossing=if previous_height>0.1 && new_height<=0.1 {Some(HeightCrossing::Down)}
        else if previous_height<=0.1 && new_height>0.1 {Some(HeightCrossing::Up)} else {None};
    EditOutcome{changed_fields:mask,changes,previous_height,new_height,gable_before_ridge,gable_after_ridge,rebuild_gable,feedback,height_crossing,resolved_update:true}
}

pub fn apply_resolved_roof_edit(roof:&mut[u8;88],event:&[u8;64],changed_tick:&mut u32,current_tick:u32)->EditOutcome {
    apply_edit(roof,&RoofEditEvent::from_observed(event),changed_tick,current_tick)
}
