//! B `resources::shutdown`, `initiate_game_shutdown` 0x140b2ba40 (185 bytes).
//! The native reader consumes all unread unit commands and queues insertion of
//! the zero-sized `ShutdownGame` resource once when at least one exists.
use bevy_ecs::prelude::*;

use super::events::ShutdownGameCmd;

/// Presence is the game-loop quit condition (B main_loop 0x1404da12..).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ShutdownGame;

pub fn initiate_game_shutdown(mut requests: EventReader<ShutdownGameCmd>, mut commands: Commands) {
    if requests.read().last().is_some() {
        commands.insert_resource(ShutdownGame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_request_becomes_shutdown_only_after_the_game_system_runs() {
        let mut world = World::new();
        world.init_resource::<Events<ShutdownGameCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(initiate_game_shutdown);
        schedule.run(&mut world);
        assert!(!world.contains_resource::<ShutdownGame>());
        world.send_event(ShutdownGameCmd);
        assert!(!world.contains_resource::<ShutdownGame>());
        schedule.run(&mut world);
        assert!(world.contains_resource::<ShutdownGame>());
        world.remove_resource::<ShutdownGame>();
        schedule.run(&mut world);
        assert!(!world.contains_resource::<ShutdownGame>());
    }
}
