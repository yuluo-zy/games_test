use super::*;
use country_core::resources::photo_mode::PhotoModeCameraMode;

#[path = "focus_native_fixtures.rs"]
mod native;

#[test]
fn full_visibility_system_matches_all_2048_original_byte_camera_branches() {
    let mut world=World::new();world.insert_resource(PhotomodeState::default());
    let root=world.spawn(UiInteractionState {state:77,keep:0,selected:88}).id();
    let child=world.spawn((UiInteractionState {state:99,keep:5,selected:66},UiShowIfParentIsInFocus {parent:root.to_bits()})).id();
    let mut schedule=Schedule::default();schedule.add_systems(ui_show_if_parent_in_focus);
    let mut had_flag=false;
    for &(flag,mode,keep,expected) in native::VISIBILITY {
        if flag!=had_flag {
            if flag {world.entity_mut(child).insert(ShowOnlyInFpvAndFree);} else {world.entity_mut(child).remove::<ShowOnlyInFpvAndFree>();}
            had_flag=flag;
        }
        world.resource_mut::<PhotomodeState>().camera_mode=match mode {0=>PhotoModeCameraMode::TiltShift,1=>PhotoModeCameraMode::Free,2=>PhotoModeCameraMode::Walk,3=>PhotoModeCameraMode::TopDown,_=>unreachable!()};
        world.get_mut::<UiInteractionState>(root).unwrap().keep=keep;
        world.get_mut::<UiInteractionState>(child).unwrap().state=99;
        schedule.run(&mut world);
        let actual=world.get::<UiInteractionState>(child).unwrap();
        assert_eq!(actual.state,expected,"original flag={flag} camera={mode} parent.keep={keep}");
        assert_eq!((actual.keep,actual.selected),(5,66));
        assert_eq!(inherited_visibility(keep,flag,mode),expected);
    }
}

#[test]
fn breadth_first_ancestor_propagation_matches_32_complete_original_graphs() {
    for (case,(input,expected)) in native::BFS.iter().enumerate() {
        let mut world=World::new();
        let entities:Vec<_>=(0..9).map(|_|world.spawn_empty().id()).collect();
        // Preserve native dense-table group ordering, while mapping original
        // opaque Entity generations to the real rebuilt ECS identities.
        for group in 0..3 {
            for (i,&(state,parent,fpv)) in input.iter().enumerate() {
                let actual_group=if parent<0 {0} else if fpv {2} else {1};
                if group!=actual_group {continue;}
                let mut entity=world.entity_mut(entities[i]);
                entity.insert(UiInteractionState {state:state[0],keep:state[1],selected:state[2]});
                if parent>=0 {
                    let parent=entities.get(parent as usize).copied().unwrap_or(Entity::PLACEHOLDER);
                    entity.insert(UiShowIfParentIsInFocus {parent:parent.to_bits()});
                    if fpv {entity.insert(ShowOnlyInFpvAndFree);}
                }
            }
        }
        let mut schedule=Schedule::default();schedule.add_systems(ui_focus_parent_if_self_in_focus);
        schedule.run(&mut world);
        for (i,entity) in entities.into_iter().enumerate() {
            let actual=world.get::<UiInteractionState>(entity).unwrap();
            assert_eq!([actual.state,actual.keep,actual.selected],expected[i],"full original graph {case} entity{i}");
        }
    }
}

#[test]
fn persistent_focus_entries_and_zst_hint_events_match_all_original_frames() {
    let mut world=World::new();
    world.init_resource::<ToolbarFocusTracking>();
    world.init_resource::<Events<UiJustBecameInFocusEvent>>();world.init_resource::<Events<ToolbarUiIsInFocus>>();
    let ids:Vec<_>=(0..6).map(|i|world.spawn(UiInteractionState {state:i,keep:0,selected:i+40}).id()).collect();
    let mut schedule=Schedule::default();schedule.add_systems((ui_send_focus_begin_events,ui_notify_hints_if_toolbar_in_focus));
    for (frame,&(keeps,expected_begin,expected_notify)) in native::EVENTS.iter().enumerate() {
        for (id,keep) in ids.iter().zip(keeps) {world.get_mut::<UiInteractionState>(*id).unwrap().keep=keep;}
        schedule.run(&mut world);
        let events:Vec<_>=world.resource_mut::<Events<UiJustBecameInFocusEvent>>().drain().map(|e|ids.iter().position(|id|*id==e.entity).unwrap()).collect();
        assert_eq!(events,expected_begin,"original persistent frame{frame}");
        assert_eq!(world.resource_mut::<Events<ToolbarUiIsInFocus>>().drain().count(),expected_notify,"original hint frame{frame}");
    }
}

#[test]
#[should_panic(expected="native parent UiInteractionState unwrap")]
fn missing_parent_state_preserves_original_visibility_unwrap_failure() {
    let mut world=World::new();world.insert_resource(PhotomodeState::default());
    let parent=world.spawn_empty().id();
    world.spawn((UiInteractionState::default(),UiShowIfParentIsInFocus {parent:parent.to_bits()}));
    let mut schedule=Schedule::default();schedule.add_systems(ui_show_if_parent_in_focus);schedule.run(&mut world);
}


#[test]
fn session_exit_releases_focus_membership_before_new_entity_generations_enter() {
    let mut world = World::new();
    world.init_resource::<Events<UiJustBecameInFocusEvent>>();
    world.init_resource::<ToolbarFocusTracking>();
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_send_focus_begin_events);
    let mut previous = None;
    for session in 0..32 {
        let entity = world.spawn(UiInteractionState { state: 1, keep: 1, selected: 0 }).id();
        assert_ne!(Some(entity), previous, "new session must carry a new entity generation");
        schedule.run(&mut world);
        assert_eq!(world.resource::<ToolbarFocusTracking>().len(), 1, "session {session}");
        let entered: Vec<_> = world.resource_mut::<Events<UiJustBecameInFocusEvent>>()
            .drain().map(|event| event.entity).collect();
        assert_eq!(entered, [entity]);
        schedule.run(&mut world);
        assert!(world.resource::<Events<UiJustBecameInFocusEvent>>().is_empty());
        assert!(world.despawn(entity));
        schedule.run(&mut world);
        // Preserve native normal-frame behavior: disappearing from the query
        // alone does not release the old generation or manufacture an event.
        assert_eq!(world.resource::<ToolbarFocusTracking>().len(), 1);
        world.resource_mut::<ToolbarFocusTracking>().clear_session();
        assert!(world.resource::<ToolbarFocusTracking>().is_empty());
        schedule.run(&mut world);
        assert!(world.resource::<Events<UiJustBecameInFocusEvent>>().is_empty());
        previous = Some(entity);
    }
}
