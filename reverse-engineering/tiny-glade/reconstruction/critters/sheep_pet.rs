//! C sheep pet event -> five-phase source mutation -> locomotion predicate.
//! Original C14293a350, game per-row dispatch; whole ECS/UI upstream is separate.
//! Observed rows preserve all opaque fields; no startup/default is invented.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct SheepPetRowObserved(pub [u8;104]);
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PetAnimationError{InvalidObservedPhase(u32)}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PetConsumerState{
    pub amplitude:f32,
    /// C source+60 stores are proven; its other consumer semantics are unknown.
    pub observed_control_flag:u8,
    /// Exact country-core Sheep::being_petted140935910; caller skips walk animation.
    pub being_petted:bool,
}
impl SheepPetRowObserved{
    pub fn from_observed(row:[u8;104])->Self{Self(row)}
    pub fn target(&self)->u64{u64::from_le_bytes(self.0[0x20..0x28].try_into().unwrap())}
    pub fn phase(&self)->u32{u32::from_le_bytes(self.0[0x18..0x1c].try_into().unwrap())}
    pub fn progress(&self)->f32{self.f32_at(0x1c)}
    pub fn consumer_state(&self)->PetConsumerState{
        let amplitude=self.f32_at(0x58);
        PetConsumerState{amplitude,observed_control_flag:self.0[0x60],being_petted:amplitude>f32::from_bits(0x3c23d70a)}
    }
    fn f32_at(&self,offset:usize)->f32{f32::from_le_bytes(self.0[offset..offset+4].try_into().unwrap())}
    fn set_phase(&mut self,phase:u32){self.0[0x18..0x1c].copy_from_slice(&phase.to_le_bytes());}
    fn set_f32(&mut self,offset:usize,value:f32){self.0[offset..offset+4].copy_from_slice(&value.to_le_bytes());}
}
/// Returns the original explicit change-tick-write decision, separate from Eq.
/// Event membership is boolean: duplicate events do not advance twice.
/// Event in fade-out/complete does not restart; it must reach idle first.
/// NaN progress follows original UCOMISS/SETBE (unordered keeps the hold).
pub fn advance_sheep_pet(row:&mut SheepPetRowObserved,requested:bool,delta:f32)->Result<bool,PetAnimationError>{
    let phase=row.phase();
    match phase{
        0=>{
            if !requested{return Ok(false);}
            row.0[0x60]=1;row.set_phase(1);row.set_f32(0x1c,0.);
        },
        1=>{
            let progress=delta/f32::from_bits(0x3e99999a)+row.progress();
            row.set_phase(if progress>1.{2}else{1});
            row.set_f32(0x1c,if progress>1.{0.}else{progress});
            row.set_f32(0x58,progress);
        },
        2=>{
            let progress=delta/f32::from_bits(0x3e4ccccd)+row.progress();
            let retain=!(progress>1.)||requested;
            row.set_phase(if retain{2}else{3});row.set_f32(0x1c,if retain{progress}else{0.});
            row.set_f32(0x58,1.);
        },
        3=>{
            let progress=delta/f32::from_bits(0x3e99999a)+row.progress();
            row.set_phase(if progress>1.{4}else{3});
            if !(progress>1.){row.set_f32(0x1c,progress);}
            row.0[0x60]=0;row.set_f32(0x58,1.-progress);
        },
        4=>{row.set_phase(0);row.set_f32(0x58,0.);},
        other=>return Err(PetAnimationError::InvalidObservedPhase(other)),
    }
    Ok(true)
}
