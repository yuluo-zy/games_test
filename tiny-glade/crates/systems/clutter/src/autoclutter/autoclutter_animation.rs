//! B1422ccef0/2261B: animate actual Nani transforms and send real clutter audio.
//! One canonical 24B AutoClutter component is shared with the manager.

use bevy_ecs::prelude::*;
use country_core::{
    components::drawable::AutoClutter,
    resources::{clutter_meshes::ClutterMarkedForDeletion, time::GameTime},
    systems::audio::PlayClutterSoundCmd,
    utils::components::transform::Transform,
};
use glam::{Quat, Vec3};
use utils::easing_functions::EasingFunction;

use super::WindowShutter;

pub fn autoclutter_animation(
    mut query: Query<(Entity, &mut AutoClutter, ComponentWrite<Transform>,
        Option<&WindowShutter>, Option<&ClutterMarkedForDeletion>)>,
    mut commands: Commands,
    time: Res<GameTime>,
    mut sounds: EventWriter<PlayClutterSoundCmd>,
) {
    for (entity, mut clutter, mut transform, shutter, marked) in &mut query {
        let appearing = marked.is_none();
        let delta = if !appearing {
            time.delta / -0.3
        } else if clutter.timer > 0.0 {
            clutter.timer -= time.delta;
            if clutter.timer < 0.0 {
                sounds.write(PlayClutterSoundCmd { kind: clutter.native_group,
                    position: transform.translation, context: 0 });
            }
            0.0
        } else {
            // Unordered timer takes this path, as in native UCOMISS/JBE.
            time.delta / if shutter.is_some() { 0.4 } else { 0.3 }
        };
        // Native MAXSS/MINSS preserve an unordered source and signed zero.
        // Rust f32::min/max would choose the numeric operand on NaN.
        let mut progress = clutter.progress + delta;
        if progress < 0.0 { progress = 0.0; }
        if progress > 1.0 { progress = 1.0; }
        clutter.progress = progress;
        if let Some(shutter) = shutter {
            if appearing {
                let eased = EasingFunction::BackOut.apply(progress);
                let sign = if shutter.side { 1.0 } else { -1.0 };
                let angle = ((eased * sign * 60.0) - ((1.0 - eased) * sign * 50.0))
                    * f32::from_bits(0x3c8e_fa35);
                transform.rotation = shutter.base_rotation * Quat::from_rotation_y(angle);
                transform.scale = Vec3::ONE;
            } else {
                transform.scale = Vec3::splat(EasingFunction::CubicInOut.apply(progress));
            }
        } else {
            transform.scale = Vec3::splat(
                EasingFunction::BackOut.apply(progress) * clutter.target_scale);
        }
        if !appearing && progress == 0.0 {
            // IEEE equality is false for NaN; waiting new entities stay alive.
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(timer: f32, progress: f32, delta: f32) -> (World, Entity, Schedule) {
        let mut world = World::new();
        world.insert_resource(GameTime { delta, ..Default::default() });
        world.init_resource::<Events<PlayClutterSoundCmd>>();
        let entity = world.spawn((AutoClutter { key: 0xffff_0000_0000_0001, timer,
            progress, target_scale: 2.0, native_group: 7, native_flag_15: true },
            Transform { rotation: Quat::IDENTITY, translation: Vec3::new(3.0, 4.0, 5.0), scale: Vec3::ZERO })).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(autoclutter_animation);
        (world, entity, schedule)
    }

    #[test]
    fn real_timer_emits_once_then_actual_transform_animates_and_waiting_entity_survives() {
        let (mut world, entity, mut schedule) = fixture(0.01, 0.0, 0.02);
        schedule.run(&mut world);
        assert!(world.get_entity(entity).is_ok());
        let sounds: Vec<_> = world.resource_mut::<Events<PlayClutterSoundCmd>>().drain().collect();
        assert_eq!(sounds.len(), 1);
        assert_eq!(sounds[0].kind, 7);
        assert_eq!(sounds[0].context, 0);
        assert_eq!(sounds[0].position, Vec3::new(3.0, 4.0, 5.0));
        schedule.run(&mut world);
        assert!(world.resource::<Events<PlayClutterSoundCmd>>().is_empty());
        assert!(world.get::<AutoClutter>(entity).unwrap().progress > 0.0);
        assert!(world.get::<Transform>(entity).unwrap().scale.x > 0.0);
    }

    #[test]
    fn shutter_uses_noncommuting_base_times_y_and_cubic_removal() {
        let (mut world, entity, mut schedule) = fixture(0.0, 1.0, 0.0);
        let base = Quat::from_rotation_x(0.7) * Quat::from_rotation_z(0.2);
        world.entity_mut(entity).insert(WindowShutter { base_rotation: base, side: true });
        schedule.run(&mut world);
        let expected = base * Quat::from_rotation_y(core::f32::consts::PI / 3.0);
        assert!(world.get::<Transform>(entity).unwrap().rotation.abs_diff_eq(expected, 1e-6));
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, Vec3::ONE);
        assert_eq!(world.get::<WindowShutter>(entity).unwrap().base_rotation, base);
        world.entity_mut(entity).insert(ClutterMarkedForDeletion);
        world.resource_mut::<GameTime>().delta = 0.15;
        schedule.run(&mut world);
        assert_eq!(world.get::<Transform>(entity).unwrap().scale, Vec3::splat(0.5));
        schedule.run(&mut world);
        assert!(world.get_entity(entity).is_err());
    }

    #[test]
    fn unordered_timer_divides_and_progress_saturation_preserves_nan_and_signed_zero() {
        let (mut world, entity, mut schedule) = fixture(f32::NAN, 0.0, 0.03);
        schedule.run(&mut world);
        assert!(world.get::<AutoClutter>(entity).unwrap().progress > 0.0);
        world.get_mut::<AutoClutter>(entity).unwrap().progress = -0.0;
        world.resource_mut::<GameTime>().delta = -0.0;
        schedule.run(&mut world);
        assert_eq!(world.get::<AutoClutter>(entity).unwrap().progress.to_bits(), (-0.0f32).to_bits());
        world.resource_mut::<GameTime>().delta = f32::NAN;
        schedule.run(&mut world);
        assert!(world.get::<AutoClutter>(entity).unwrap().progress.is_nan());
        assert!(world.get_entity(entity).is_ok());
    }
}

use country_core::render::extract::ComponentWrite;
