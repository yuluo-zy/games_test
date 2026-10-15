pub mod calculate_input_delta;
pub mod translate_input_to_cursor_moved;

use bevy_ecs::prelude::*;
use translate_input_to_cursor_moved::{InputCursorDelta, InputCursorMoved};

pub fn register_input_events(world: &mut World) {
    world.init_resource::<Events<InputCursorMoved>>();
    world.init_resource::<Events<InputCursorDelta>>();
}

/// Local runnable order for the recovered consumers. Supply actual cursor,
/// capture, window, ActionSetValues, ActionSetStates and bevy Time resources.
/// OS/device translation and replay producers are not installed by this helper.
pub fn add_input_cpu_consumers(schedule: &mut Schedule) {
    schedule.add_systems((
        translate_input_to_cursor_moved::translate_input_to_cursor_moved,
        calculate_input_delta::calculate_input_delta,
    ).chain());
}
