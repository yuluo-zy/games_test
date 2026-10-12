//! C Bevy0.17.3 Message -> real Query source -> locomotion consumer.
//! Adapter ordering is explicit; original whole scheduler and UI picking remain gaps.
use bevy_ecs::prelude::*;
use crate::sheep_pet::{SheepPetRowObserved,PetConsumerState,advance_sheep_pet};
#[derive(Component,Clone)]
pub struct SheepPetSource(pub SheepPetRowObserved);
#[derive(Component,Clone,Copy,Debug,PartialEq)]
pub struct SheepLocomotionPetGate{
    pub allow_walk_animation:bool,
    pub amplitude:f32,
    /// Retained presentation payload, not given an invented renderer meaning.
    pub observed_control_flag:u8,
}
#[derive(Message,Clone,Copy,Debug)]
pub struct PlaySheepPetAnimation{pub observed_target:u64}
/// Required authoritative GameTime+28 input. No default or guessed Frame.delta.
#[derive(Resource)]
pub struct SheepPetGameTime{pub delta:f32}
#[derive(Resource,Default)]
pub struct SheepPetDiagnostics{pub faults:Vec<String>,pub selected:usize,pub changed:usize}
pub fn register(app:&mut bevy_app::App){
    app.add_message::<PlaySheepPetAnimation>().init_resource::<SheepPetDiagnostics>()
        .add_systems(bevy_app::Update,(advance,consume_locomotion).chain());
}
fn advance(mut events:MessageReader<PlaySheepPetAnimation>,time:Res<SheepPetGameTime>,mut rows:Query<&mut SheepPetSource>,mut diagnostics:ResMut<SheepPetDiagnostics>){
    let targets:Vec<_>=events.read().map(|event|event.observed_target).collect();
    diagnostics.selected=0;diagnostics.changed=0;diagnostics.faults.clear();
    for mut source in &mut rows{
        let requested=targets.contains(&source.0.target());
        diagnostics.selected+=usize::from(requested);
        let mut next=source.0.clone();
        match advance_sheep_pet(&mut next,requested,time.delta){
            Ok(true)=>{source.0=next;diagnostics.changed+=1;},
            Ok(false)=>{},
            Err(error)=>diagnostics.faults.push(format!("Sheep {} pet pending: {error:?}",source.0.target())),
        }
    }
}
fn consume_locomotion(mut commands:Commands,rows:Query<(Entity,&SheepPetSource,Option<&SheepLocomotionPetGate>)>,mut removed:RemovedComponents<SheepPetSource>){
    for entity in removed.read(){
        if rows.get(entity).is_err(){if let Ok(mut entity_commands)=commands.get_entity(entity){entity_commands.remove::<SheepLocomotionPetGate>();}}
    }
    for(entity,source,previous)in &rows{
        let PetConsumerState{amplitude,observed_control_flag,being_petted}=source.0.consumer_state();
        let next=SheepLocomotionPetGate{allow_walk_animation:!being_petted,amplitude,observed_control_flag};
        // Adapter publication uses bits to avoid repeat NaN insertions while
        // preserving signed zero. This is not an original consumer tick claim.
        let unchanged=previous.is_some_and(|old|old.allow_walk_animation==next.allow_walk_animation
            &&old.amplitude.to_bits()==next.amplitude.to_bits()&&old.observed_control_flag==next.observed_control_flag);
        if !unchanged{commands.entity(entity).insert(next);}
    }
}
