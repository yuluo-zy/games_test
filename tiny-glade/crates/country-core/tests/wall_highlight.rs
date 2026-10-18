use bevy_ecs::{
    entity::Entity,
    event::{EventReader, Events},
    schedule::Schedule,
    system::{Res, ResMut, SystemState},
    world::World,
};
use country_core::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        editing_highlight::EditingHighlight,
        gui::DebugGui,
        walls::{EraserSizeMultiplier, WallHighlight, WallId, public_walls::WallEntities},
    },
    systems::{
        render_loop::draw_passes::common_draw_params::BrickEraseInputs,
        wall::wall_highlight::{HighlightWallCmd, wall_highlight},
    },
};
use glam::Vec3;
use serde_json::Value;

fn world_with_wall(enabled: bool) -> World {
    let mut world = World::new();
    world.insert_resource(WallHighlight {
        wall_entity: Some(Entity::from_bits((1 << 32) | 7)),
        wall_id: Some(WallId(9)),
        ty: 3,
    });
    world.insert_resource(EditingHighlight {
        color: Vec3::new(0.3, 0.4, 0.5),
        enabled,
    });
    let mut entities = WallEntities::default();
    entities
        .0
        .insert(WallId(1), Entity::from_bits((1 << 32) | 42));
    world.insert_resource(entities);
    world
}

#[test]
fn ecs_highlight_matches_original_last_event_and_state_writes() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/wall-highlight-b.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut world = world_with_wall(case["enabled"].as_bool().unwrap());
        let inputs = case["events"].as_array().unwrap();
        let split = case["split"].as_u64().unwrap() as usize;
        let mut events = Events::default();
        let command = |value: &Value| HighlightWallCmd {
            wall: WallId(value[0].as_u64().unwrap()),
            ty: value[1].as_u64().unwrap() as u8,
        };
        for input in &inputs[..split] {
            events.send(command(input));
        }
        events.update();
        for input in &inputs[split..] {
            events.send(command(input));
        }
        world.insert_resource(events);
        let mut params = SystemState::<(
            ResMut<WallHighlight>,
            ResMut<EditingHighlight>,
            EventReader<HighlightWallCmd>,
            Res<WallEntities>,
        )>::new(&mut world);
        let (highlight, editing, mut reader, entities) = params.get_mut(&mut world);
        let consumed = case["consumed"].as_u64().unwrap() as usize;
        assert_eq!(reader.read().take(consumed).count(), consumed);
        wall_highlight(highlight, editing, reader, entities);
        params.apply(&mut world);
        let highlight = world.resource::<WallHighlight>();
        assert_eq!(highlight.ty, case["ty"].as_u64().unwrap() as u8, "{case}");
        assert_eq!(
            highlight.wall_entity.unwrap().to_bits(),
            case["entity_bits"].as_u64().unwrap(),
            "{case}"
        );
        assert_eq!(
            highlight.wall_id.unwrap().0,
            case["wall"].as_u64().unwrap(),
            "{case}"
        );
        let editing = world.resource::<EditingHighlight>();
        assert_eq!(
            editing.color.to_array().map(f32::to_bits),
            std::array::from_fn(|i| case["color_bits"][i].as_u64().unwrap() as u32),
            "{case}"
        );
        assert_eq!(
            editing.enabled,
            case["enabled_after"].as_bool().unwrap(),
            "{case}"
        );
        assert_eq!(
            params.get_mut(&mut world).2.len(),
            inputs.len() - case["cursor_after"].as_u64().unwrap() as usize
        );
        // EventReader advances its own cursor without draining other consumers.
        assert_eq!(
            world.resource::<Events<HighlightWallCmd>>().len(),
            inputs.len()
        );
    }
}

#[test]
fn scheduled_highlight_clears_kind_next_frame_and_keeps_eraser_state() {
    let mut world = world_with_wall(true);
    let mut events = Events::default();
    events.send(HighlightWallCmd {
        wall: WallId(1),
        ty: 1,
    });
    world.insert_resource(events);
    world.insert_resource(DebugGui::default());
    world.insert_resource(CursorTerrainRaycast {
        tag: 1,
        pos: Vec3::ONE,
        delta: Vec3::ZERO,
        speed: 0.0,
    });
    world.insert_resource(EraserSizeMultiplier(2.0));
    let mut schedule = Schedule::default();
    schedule.add_systems(wall_highlight);
    schedule.run(&mut world);
    assert_eq!(world.resource::<WallHighlight>().ty, 1);
    assert!(BrickEraseInputs::from_world(&world).to_a4().enabled);
    let color = world.resource::<EditingHighlight>().color;
    schedule.run(&mut world);
    assert_eq!(world.resource::<WallHighlight>().ty, 0);
    assert_eq!(world.resource::<WallHighlight>().wall_id, Some(WallId(1)));
    assert_eq!(world.resource::<EditingHighlight>().color, color);
    // A valid earlier command must not win when the last command has no entity.
    world
        .resource_mut::<Events<HighlightWallCmd>>()
        .send(HighlightWallCmd {
            wall: WallId(1),
            ty: 2,
        });
    world
        .resource_mut::<Events<HighlightWallCmd>>()
        .send(HighlightWallCmd {
            wall: WallId(99),
            ty: 1,
        });
    schedule.run(&mut world);
    assert_eq!(world.resource::<WallHighlight>().ty, 0);
    assert_eq!(world.resource::<EditingHighlight>().color, color);
}
