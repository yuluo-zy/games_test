use bevy_app::App;
use bevy_ecs::change_detection::DetectChanges;
use tg_critters_c_pet_harness::{sheep_pet::SheepPetRowObserved,sheep_pet_runtime::*};
fn observed(target:u64)->SheepPetSource{
    let mut row=[0xa5;104];row[0x18..0x20].copy_from_slice(&0u64.to_le_bytes());
    row[0x20..0x28].copy_from_slice(&target.to_le_bytes());row[0x58..0x5c].copy_from_slice(&0f32.to_le_bytes());row[0x60]=0;
    SheepPetSource(SheepPetRowObserved::from_observed(row))
}
fn app()->App{let mut app=App::new();app.insert_resource(SheepPetGameTime{delta:0.15});register(&mut app);app}
#[test]
fn message_to_native_state_to_locomotion_and_finish(){
    let mut app=app();let e=app.world_mut().spawn(observed(17)).id();let other=app.world_mut().spawn(observed(18)).id();
    for _ in 0..2{app.world_mut().write_message(PlaySheepPetAnimation{observed_target:17});}
    app.update();let source=app.world().get::<SheepPetSource>(e).unwrap();assert_eq!(source.0.phase(),1);assert_eq!(source.0.progress(),0.);assert_eq!(source.0.0[0x60],1);
    let other_tick=app.world().entity(other).get_ref::<SheepPetSource>().unwrap().last_changed();
    assert_eq!(app.world().resource::<SheepPetDiagnostics>().changed,1);
    assert_eq!(app.world().get::<SheepPetSource>(other).unwrap().0,observed(18).0);
    app.update();assert!(!app.world().get::<SheepLocomotionPetGate>(e).unwrap().allow_walk_animation);
    app.update();assert_eq!(app.world().get::<SheepPetSource>(e).unwrap().0.phase(),1); // exact1 does not transition
    app.update();let source=app.world().get::<SheepPetSource>(e).unwrap();assert_eq!(source.0.phase(),2);assert!(source.0.consumer_state().amplitude>1.); // original overrun retained
    let tick=app.world().entity(e).get_ref::<SheepPetSource>().unwrap().last_changed();
    app.world_mut().resource_mut::<SheepPetGameTime>().delta=0.;app.update();
    assert_ne!(app.world().entity(e).get_ref::<SheepPetSource>().unwrap().last_changed(),tick); // explicit write despite dt0
    app.world_mut().resource_mut::<SheepPetGameTime>().delta=0.2;
    app.update();assert_eq!(app.world().get::<SheepPetSource>(e).unwrap().0.phase(),2);
    app.update();assert_eq!(app.world().get::<SheepPetSource>(e).unwrap().0.phase(),3);
    app.world_mut().resource_mut::<SheepPetGameTime>().delta=0.15;
    app.update();assert_eq!(app.world().get::<SheepPetSource>(e).unwrap().0.0[0x60],0);
    app.update();assert!(app.world().get::<SheepLocomotionPetGate>(e).unwrap().allow_walk_animation);
    app.update();let source=app.world().get::<SheepPetSource>(e).unwrap();assert_eq!(source.0.phase(),4);assert!(source.0.consumer_state().amplitude<0.);
    app.update();let source=app.world().get::<SheepPetSource>(e).unwrap();assert_eq!(source.0.phase(),0);assert_eq!(source.0.consumer_state().amplitude,0.);assert_eq!(source.0.0[0x60],0);
    let final_row=source.0.clone();let tick=app.world().entity(e).get_ref::<SheepPetSource>().unwrap().last_changed();
    app.update();assert_eq!(app.world().get::<SheepPetSource>(e).unwrap().0,final_row);
    assert_eq!(app.world().entity(e).get_ref::<SheepPetSource>().unwrap().last_changed(),tick);
    assert_eq!(app.world().entity(other).get_ref::<SheepPetSource>().unwrap().last_changed(),other_tick);
    assert_eq!(app.world().resource::<SheepPetDiagnostics>().changed,0);
}
#[test]
fn source_removed_with_live_entity_withdraws_derived_gate(){
    let mut app=app();let e=app.world_mut().spawn(observed(3)).id();app.update();assert!(app.world().get::<SheepLocomotionPetGate>(e).is_some());
    app.world_mut().entity_mut(e).remove::<SheepPetSource>();app.world_mut().write_message(PlaySheepPetAnimation{observed_target:3});app.update();
    assert!(app.world().get::<SheepLocomotionPetGate>(e).is_none());assert_eq!(app.world().resource::<SheepPetDiagnostics>().selected,0);
    app.world_mut().entity_mut(e).insert(observed(3));app.update();assert!(app.world().get::<SheepLocomotionPetGate>(e).is_some());
    app.world_mut().despawn(e);app.update();assert!(app.world().get_entity(e).is_err());
}
#[test]
fn event_during_fade_out_does_not_restart_and_unordered_hold_is_retained(){
    let mut row=observed(4).0;row.0[0x18..0x1c].copy_from_slice(&3u32.to_le_bytes());
    tg_critters_c_pet_harness::sheep_pet::advance_sheep_pet(&mut row,true,0.15).unwrap();assert_eq!(row.phase(),3);assert_eq!(row.0[0x60],0);
    row.0[0x18..0x1c].copy_from_slice(&2u32.to_le_bytes());row.0[0x1c..0x20].copy_from_slice(&f32::NAN.to_le_bytes());
    tg_critters_c_pet_harness::sheep_pet::advance_sheep_pet(&mut row,false,0.1).unwrap();assert_eq!(row.phase(),2);assert!(row.progress().is_nan());assert_eq!(row.consumer_state().amplitude,1.);
}
#[test]
fn unchanged_nan_payload_does_not_republish_adapter_gate(){
    let mut app=app();let mut source=observed(5);source.0.0[0x58..0x5c].copy_from_slice(&0x7fc12345u32.to_le_bytes());
    let entity=app.world_mut().spawn(source).id();app.update();
    let tick=app.world().entity(entity).get_ref::<SheepLocomotionPetGate>().unwrap().last_changed();
    app.update();assert_eq!(app.world().entity(entity).get_ref::<SheepLocomotionPetGate>().unwrap().last_changed(),tick);
}
