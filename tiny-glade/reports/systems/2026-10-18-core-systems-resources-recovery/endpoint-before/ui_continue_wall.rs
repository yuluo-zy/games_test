//! B continuation suppression, 0x140aadbb0 (375B). The endpoint UI producer
//! is separate; this resource retains its actual release flag and reset inputs.
use crate::{
    resources::{time::GameTime, window::CursorMovedScreenSpace},
    systems::backend::input::mod_key::Modifiers,
};
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SuppressContinueWall {
    pub elapsed: f32,
    pub distance: f32,
    pub suppressed: bool,
}

pub fn suppress_continue_wall(
    modifiers: Res<Modifiers>,
    mut state: ResMut<SuppressContinueWall>,
    time: Res<GameTime>,
    mut motion: EventReader<CursorMovedScreenSpace>,
) {
    if state.suppressed {
        state.elapsed += time.delta;
        // Original clears every pending event, then measures only the last
        // payload. Events remain unread while suppression is inactive.
        if let Some(motion) = motion.read().last() {
            let point = motion.0;
            state.distance += (point.y * point.y + point.x * point.x).sqrt();
        }
        if state.elapsed >= 1.0 || state.distance >= 50.0 {
            state.suppressed = false;
            state.elapsed = 0.0;
            state.distance = 0.0;
        }
    }
    // This check is after the thresholds: holding the modifier re-arms the
    // flag in the same frame, without changing either accumulator here.
    if modifiers.disable_snapping() {
        state.suppressed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn last_movement_only_and_modifier_rearm_keep_original_event_cursor_order() {
        let mut world = World::new();
        world.insert_resource(Modifiers { flags: 1 });
        world.init_resource::<SuppressContinueWall>();
        world.insert_resource(GameTime {
            delta: 0.2,
            ..Default::default()
        });
        world.init_resource::<Events<CursorMovedScreenSpace>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(suppress_continue_wall);
        // Inactive suppression does not consume this event.
        world
            .resource_mut::<Events<CursorMovedScreenSpace>>()
            .send(CursorMovedScreenSpace(Vec2::new(50.0, 0.0)));
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
        world.resource_mut::<Modifiers>().flags = 0;
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(!state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
        world.resource_mut::<SuppressContinueWall>().suppressed = true;
        let mut motion = world.resource_mut::<Events<CursorMovedScreenSpace>>();
        motion.send(CursorMovedScreenSpace(Vec2::new(30.0, 40.0)));
        motion.send(CursorMovedScreenSpace(Vec2::new(3.0, 4.0)));
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.2, 5.0));
        world.resource_mut::<SuppressContinueWall>().elapsed = 0.9;
        world.resource_mut::<Modifiers>().flags = 1;
        schedule.run(&mut world);
        let state = world.resource::<SuppressContinueWall>();
        assert!(state.suppressed);
        assert_eq!((state.elapsed, state.distance), (0.0, 0.0));
    }
}
