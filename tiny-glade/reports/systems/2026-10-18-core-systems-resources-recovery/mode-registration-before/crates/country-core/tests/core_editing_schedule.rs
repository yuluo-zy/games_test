//! Validate the actual local registration graph after joining the recovered
//! input, history, terrain and wall consumers. This does not accept the
//! original engine's global schedule or execute a graphics backend.
use bevy_ecs::prelude::*;
use country_core::systems::{
    backend::input::add_input_cpu_consumers,
    history::add_history_cpu_systems,
    terrain::{add_terrain_cpu_systems, add_terrain_tool_systems},
    wall::registration::add_wall_cpu_systems,
};

#[test]
fn registered_editing_stages_have_no_dependency_cycle() {
    let mut world = World::new();
    let mut schedule = Schedule::default();
    add_input_cpu_consumers(&mut schedule);
    add_history_cpu_systems::<()>(&mut schedule);
    add_terrain_cpu_systems(&mut schedule);
    add_terrain_tool_systems::<()>(&mut schedule);
    add_wall_cpu_systems(&mut schedule);
    schedule
        .initialize(&mut world)
        .expect("editing schedule graph");
}
