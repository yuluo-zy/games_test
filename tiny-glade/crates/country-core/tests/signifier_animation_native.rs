use bevy_ecs::prelude::*;
use country_core::{resources::time::GameTime,
    systems::ui::{resources::ui_state::UiState, signifiers::{
        animation::{SignifierAnimation, signifier_animation_t, signifier_animation},
        signifier_mark_for_delete::{SignifierMarkedForDeletion, SignifierMinimumLifespan}}},
    utils::components::transform::Transform};
use glam::Vec3;

fn same_float(actual: f32, bits: u32, label: &str, case: usize) {
    let expected = f32::from_bits(bits);
    if expected.is_nan() { assert!(actual.is_nan(), "{label} case {case}"); }
    else { assert_eq!(actual.to_bits(), bits, "{label} case {case}"); }
}

#[test]
fn advance_matches_original_instructions_and_real_easing() {
    let native = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/progress-20261020/central-signifier-animation/native-animation.bin"));
    assert_eq!(native.len(), 1728 * 28);
    for (case, bytes) in native.chunks_exact(28).enumerate() {
        let word = |offset| u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        let mut anim = SignifierAnimation { duration: f32::from_bits(word(4)),
            t: f32::from_bits(word(8)), ..Default::default() };
        anim.head[..4].copy_from_slice(&word(0).to_le_bytes());
        anim.advance(f32::from_bits(word(12)), word(16) != 0);
        same_float(anim.t, word(20), "progress", case);
        same_float(anim.eased, word(24), "eased", case);
    }
}

#[test]
fn immediate_signifier_becomes_visible_and_marked_one_reverses_with_real_lifespan_owner() {
    let mut world = World::new();
    world.insert_resource(GameTime { delta: 0.25, ..Default::default() });
    world.insert_resource(UiState::at_startup());
    let anim = SignifierAnimation { duration: 0.0, value: 0.3, flag_9: 1, ..Default::default() };
    let mut transform = Transform::identity();
    transform.scale = Vec3::ZERO;
    let live = world.spawn((anim, transform.clone(),
        SignifierMinimumLifespan { t: 3.0, min_lifespan: 4.0 })).id();
    let marked = world.spawn((anim, transform, SignifierMarkedForDeletion)).id();
    let mut schedule = Schedule::default();
    schedule.add_systems((signifier_animation_t, signifier_animation).chain());
    schedule.run(&mut world);
    assert_eq!(world.get::<SignifierAnimation>(live).unwrap().t, 1.0);
    assert_eq!(world.get::<Transform>(live).unwrap().scale, Vec3::splat(0.3));
    assert_eq!(world.get::<SignifierMinimumLifespan>(live).unwrap().t, 3.25);
    assert_eq!(world.get::<SignifierAnimation>(marked).unwrap().t, 0.0);
    assert_eq!(world.get::<Transform>(marked).unwrap().scale, Vec3::ZERO);
}
