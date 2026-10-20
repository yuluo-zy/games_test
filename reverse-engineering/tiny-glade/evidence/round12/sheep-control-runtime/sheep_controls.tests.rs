#[cfg(test)]
mod original_control_tests {
    use super::*;
    use bevy_ecs::system::SystemState;
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};

    fn bytes(text: &str) -> Vec<u8> {
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i+2],16).unwrap()).collect()
    }
    fn word(data: &[u8], at: usize) -> u32 { u32::from_le_bytes(data[at..at+4].try_into().unwrap()) }
    fn float(data: &[u8], at: usize) -> f32 { f32::from_bits(word(data,at)) }
    fn vec3(data: &[u8], at: usize) -> Vec3 { Vec3::new(float(data,at),float(data,at+4),float(data,at+8)) }
    fn movement(data: &[u8]) -> SheepMoveState {
        if data[0] == 0 { SheepMoveState::Idle { timeout_s: float(data,4) } }
        else { SheepMoveState::Walking { timeout_s:float(data,4),goal_change_timeout:float(data,8),force_reach_goal:data[1]!=0 } }
    }
    fn snapshot(value: &serde_json::Value) -> Option<SheepControlSnapshot> {
        value.as_array().map(|rows| SheepControlSnapshot { sheep: rows.iter().map(|row| {
            let row=bytes(row.as_str().unwrap());
            control::SheepSnapshot { position:vec3(&row,28),goal:(word(&row,0)==1).then(||vec3(&row,4)),move_state:movement(&row[16..28]) }
        }).collect() })
    }
    fn populate() -> (World, [Entity;3], Entity, [Entity;2]) {
        let mut world=World::new();
        let markers=[world.spawn(Transform::identity()).id(),world.spawn(Transform::identity()).id()];
        let controls=world.spawn(SheepControl::new(markers[0],markers[1])).id();
        let sheep=std::array::from_fn(|i|world.spawn((
            Transform::from_translation(Vec3::new(10.+i as f32,20.+i as f32,30.+i as f32)),
            Sheep { behavior_state:0,goal_timer:0.,pet_phase:0,pet_progress:0.,pet_target:i as u64,
                control_flag:0,move_state:SheepMoveState::Idle {timeout_s:f32::from_bits(0x40000000+i as u32)},amplitude:5. },
            CritterGoal {goal_tag:1,position:Vec3::new(40.+i as f32,50.+i as f32,60.+i as f32)},
        )).id());
        (world,sheep,controls,markers)
    }
    fn compare(world: &World, sheep: [Entity;3], controls: Entity, expected: &serde_json::Value) {
        for (i,id) in sheep.into_iter().enumerate() {
            let t=world.get::<Transform>(id).unwrap();
            let p=&expected["transforms"][i];
            assert_eq!(t.translation,Vec3::new(p[0].as_f64().unwrap() as f32,p[1].as_f64().unwrap() as f32,p[2].as_f64().unwrap() as f32));
            let g=bytes(expected["goals"][i].as_str().unwrap());
            let actual=world.get::<CritterGoal>(id).unwrap();
            assert_eq!(actual.goal_tag,word(&g,0));
            if actual.goal_tag==1 { assert_eq!(actual.position,vec3(&g,4)); }
            assert_eq!(world.get::<Sheep>(id).unwrap().move_state,movement(&bytes(expected["movement"][i].as_str().unwrap())));
            assert_eq!(world.get::<Sheep>(id).unwrap().amplitude,5.);
        }
        let actual=world.get::<SheepControl>(controls).unwrap();
        assert_eq!(actual.snapshot,snapshot(&expected["snapshot"]));
        assert_eq!(actual.selected_sheep,expected["selected"].as_u64().unwrap());
        assert_eq!(actual.picking_mode as u8,expected["picking"].as_u64().unwrap() as u8);
    }

    #[test]
    fn actual_component_runtime_restores_before_unfreeze_on_original_nonempty_fixtures() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/tests/fixtures/original-sheep-controls-runtime-b.json"))).unwrap();
        let mut checked=0;
        for case in fixtures.as_array().unwrap() {
            // The no-key cases exercise the real system without modifying a
            // process-global developer-hotkey owner during parallel tests.
            if case["press"].as_bool().unwrap() || case["developer"].as_bool().unwrap() { continue; }
            let (mut world,sheep,controls,markers)=populate();
            world.get_mut::<SheepControl>(controls).unwrap().snapshot=snapshot(&case["snapshot"]);
            world.insert_resource(AppMode::default());
            world.insert_resource(CursorTerrainRaycast::default());
            world.insert_resource(UiState::at_startup());
            world.insert_resource(DebugGui::default());
            world.insert_resource(ButtonInput::<KeyCode>::default());
            world.init_resource::<Events<CmdUnfreezeSheep>>();
            if case["unfreeze"].as_bool().unwrap() {world.resource_mut::<Events<CmdUnfreezeSheep>>().send(CmdUnfreezeSheep);}
            let mut schedule=Schedule::default();schedule.add_systems(sheep_controls);schedule.run(&mut world);
            compare(&world,sheep,controls,&case["result"]);
            for marker in markers {assert!(world.get::<Transform>(marker).is_none());}
            checked+=1;
        }
        assert_eq!(checked,8);
    }

    #[test]
    fn actual_picking_consumer_matches_48_original_runtime_fixtures() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/tests/fixtures/original-sheep-controls-picking-b.json"))).unwrap();
        for case in fixtures.as_array().unwrap() {
            let input=&case["inputs"];
            let (mut world,sheep,controls,_)=populate();
            {
                let mut control=world.get_mut::<SheepControl>(controls).unwrap();
                control.snapshot=snapshot(&input["snapshot"]);
                control.selected_sheep=input["selected"].as_u64().unwrap_or(0);
                control.picking_mode=match input["picking"].as_u64().unwrap_or(0) {0=>PickingMode::None,1=>PickingMode::Goal,2=>PickingMode::Position,_=>unreachable!()};
            }
            let hit=&input["hit"];
            let name=match hit[2].as_u64().unwrap() {100=>"sheep_selection_0",101=>"sheep_selection_1",102=>"sheep_selection_2",999=>"sheep_pick_point",_=>"unused"};
            let ui=UiState::new(UiInteraction {native_tag:hit[0].as_u64().unwrap() as u8,native_phase:hit[1].as_u64().unwrap() as u8,widget:WidgetId::from_name(name)});
            let cursor=CursorTerrainRaycast {tag:u32::from(input["cursor_hit"].as_bool().unwrap_or(true)),pos:Vec3::new(7.,99.,8.),
                delta:input["delta_bits"].as_array().map(|v|Vec3::new(f32::from_bits(v[0].as_u64().unwrap() as u32),f32::from_bits(v[1].as_u64().unwrap() as u32),f32::from_bits(v[2].as_u64().unwrap() as u32))).unwrap_or(Vec3::X),speed:0.};
            let mut state=SystemState::<(SheepQuery,Query<&mut SheepControl>)>::new(&mut world);
            {
                let (mut sheep,mut controllers)=state.get_mut(&mut world);
                let mut controls=controllers.single_mut().unwrap();
                if let Some(snapshot)=&controls.snapshot { apply_sheep_snapshot(&mut sheep,snapshot); }
                let moved=handle_mouse_picking(&mut controls,&mut sheep,&cursor,&ui);
                if moved && controls.snapshot.is_some() { controls.snapshot=Some(capture(&sheep)); }
            }
            state.apply(&mut world);
            compare(&world,sheep,controls,&case["result"]);
            assert!(ui.widget(WidgetId::from_name("sheep_selection_2")).is_some());
        }
        assert_eq!(fixtures.as_array().unwrap().len(),48);
    }

    #[test]
    fn manual_restore_preserves_distinct_freeze_ownership_and_only_syncs_an_existing_freeze() {
        for frozen in [false,true] {
            let (mut world,entities,controls,_)=populate();
            let saved=SheepControlSnapshot {sheep:vec![control::SheepSnapshot {position:Vec3::X,goal:None,move_state:SheepMoveState::Idle {timeout_s:17.}}]};
            {
                let mut controller=world.get_mut::<SheepControl>(controls).unwrap();
                controller.manual_snapshot=Some(saved.clone());
                controller.snapshot=frozen.then(||SheepControlSnapshot {sheep:vec![]});
            }
            let mut state=SystemState::<(SheepQuery,Query<&mut SheepControl>)>::new(&mut world);
            {
                let (mut sheep,mut controls)=state.get_mut(&mut world);
                restore_manual(&mut controls.single_mut().unwrap(),&mut sheep);
            }
            assert_eq!(world.get::<Transform>(entities[0]).unwrap().translation,Vec3::X);
            assert_eq!(world.get::<Transform>(entities[1]).unwrap().translation,Vec3::new(11.,21.,31.));
            let controller=world.get::<SheepControl>(controls).unwrap();
            assert_eq!(controller.manual_snapshot.as_ref(),Some(&saved));
            assert_eq!(controller.snapshot.as_ref(),frozen.then_some(&saved));
        }
    }

    #[test]
    fn real_egui_window_populates_widgets_and_debug_transforms_for_selected_live_sheep() {
        let (mut world,_,controls,markers)=populate();
        world.insert_resource(AppMode::default());world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(UiState::at_startup());world.insert_resource(ButtonInput::<KeyCode>::default());
        world.init_resource::<Events<CmdUnfreezeSheep>>();
        world.insert_resource(DebugGui {enabled:true,show_sheep_controls:true,..Default::default()});
        let context=egui::Context::default();world.insert_resource(EguiContextResource(context.clone()));
        context.begin_frame(egui::RawInput::default());
        let mut schedule=Schedule::default();schedule.add_systems(sheep_controls);schedule.run(&mut world);
        let _=context.end_frame();
        assert_eq!(world.get::<Transform>(markers[0]).unwrap().translation,Vec3::new(10.,20.,30.));
        assert_eq!(world.get::<Transform>(markers[1]).unwrap().translation,Vec3::new(40.,51.,60.));
        assert_eq!(world.get::<Transform>(markers[1]).unwrap().scale,Vec3::splat(0.5));
        assert!(world.get::<SheepControl>(controls).unwrap().snapshot.is_none());
        assert!(world.resource::<UiState>().widget(WidgetId::from_name("sheep_selection_0")).is_some());
    }
}
