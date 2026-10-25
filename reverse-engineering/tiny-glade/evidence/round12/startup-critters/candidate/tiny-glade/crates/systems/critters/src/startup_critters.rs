//! Complete original1409d49a0 session producer on canonical species components.
use crate::{
    audio::AudioHandle,
    butterfly::ButterflyAudio,
    components::{
        Butterfly, Critter, CritterGoal, CritterLocomotion, CritterVelocityMultiplier,
        CritterVelocityTarget, Firefly, Transform, Velocity,
    },
};
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{
        mesh_library::AssetMeshLibrary, meshwatch::MeshWatch, shader_library::AssetShaderLibrary,
    },
    render::shader::boundary::ShaderBackend,
    resources::{
        audio_resources::{AudioCore, AudioInstance, Studio},
        glade_settings::GladeSettings,
        session::ActiveSession,
    },
    startup::startup_critters as assets,
};
use glam::{Vec2, Vec3};
use std::path::Path;

fn butterfly_birth(rng: &fastrand::Rng) -> (Transform, Butterfly) {
    let x = (rng.f32() + (-0.5)) * 40.0;
    let z = (rng.f32() + (-0.5)) * 40.0;
    let transform = Transform::from_translation(Vec3::new(x, 1.0, z));
    let butterfly = Butterfly {
        state: 2,
        state_t: rng.f32() * 30.0 + 1.0,
        goal_timer: 0.0,
        dwell: 0.0,
        phase: 0.0,
        phase_ref: 0.0,
    };
    (transform, butterfly)
}
fn firefly_birth(index: u32, rng: &fastrand::Rng) -> Transform {
    let sequence = utils::sampling::radical_inverse(index, 2);
    let radial = (rng.f32() * 0.25 + sequence) % 1.0;
    let angular = (rng.f32() + index as f32) / 96.0;
    let disk = utils::sampling::uniform_sample_disk(Vec2::new(
        radial * f32::from_bits(0x3f0c_ccce) + f32::from_bits(0x3db8_51ec),
        angular,
    ));
    let height = rng.f32() + 1.0;
    Transform::from_translation(Vec3::new(disk.x * 65.0, height, disk.y * 65.0))
}
fn movement_bundle() -> (
    Critter,
    CritterLocomotion,
    CritterGoal,
    CritterVelocityTarget,
    Velocity,
    CritterVelocityMultiplier,
) {
    (
        Critter,
        CritterLocomotion(3),
        CritterGoal {
            position: Vec3::ZERO,
            goal_tag: 0,
        },
        CritterVelocityTarget {
            direction: Vec3::ZERO,
            retention_per_second: 0.05,
        },
        Velocity(Vec3::ZERO),
        CritterVelocityMultiplier(1.0),
    )
}

/// Original audio instantiation precedes every birth's three RNG draws. Errors
/// retain unwrap failure; Disabled and active instances both remain owned by
/// the canonical ButterflyAudio wrapper and drop with the entity.
fn spawn_butterflies<S: Studio>(audio: &AudioCore<S>, commands: &mut Commands)
where
    S::Event: Send + Sync + 'static,
{
    let rng = fastrand::Rng::with_seed(0x2a729);
    for _ in 0..assets::BUTTERFLY_COUNT {
        let instance = audio.instantiate_audio(assets::BUTTERFLY_SHOOSH_EVENT);
        if let AudioInstance::Err(message) = &instance {
            panic!("original butterfly audio initialization failed: {message}");
        }
        let (transform, butterfly) = butterfly_birth(&rng);
        commands.spawn((
            transform,
            butterfly,
            ButterflyAudio {
                handle: AudioHandle::Active(Box::new(instance)),
            },
            movement_bundle(),
        ));
    }
}
fn spawn_fireflies(commands: &mut Commands) {
    let rng = fastrand::Rng::with_seed(0x29a);
    for index in 0..assets::FIREFLY_COUNT {
        commands.spawn((
            firefly_birth(index as u32, &rng),
            Firefly { goal_timer: 0.0 },
            movement_bundle(),
        ));
    }
}

/// The original nine-param system is adapted to the existing physical asset
/// owner and canonical core audio resource. Must run per OnEnter(Game), after
/// load_game_session publishes the new ActiveSession/GladeSettings.
#[allow(clippy::too_many_arguments)]
pub fn startup_critters<S: Studio>(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    root: &Path,
    audio: &AudioCore<S>,
    settings: &GladeSettings,
    session: &ActiveSession,
    commands: &mut Commands,
) where
    S::Event: Send + Sync + 'static,
{
    if settings.butterflies {
        spawn_butterflies(audio, commands);
        assets::startup_butterfly_drawable(meshes, shaders, backend, watch, root, commands);
    }
    if !session.is_winter() {
        spawn_fireflies(commands);
        assets::startup_firefly_drawables(meshes, shaders, backend, watch, root, commands);
    }
    assets::startup_duck_assets(meshes, shaders, backend, watch, root);
}

#[cfg(test)]
mod original_birth_tests {
    use super::*;
    include!("../tests/fixtures/startup_original_b.rs");
    #[test]
    fn all_original_seeded_butterfly_and_firefly_births_match_native_components() {
        let rng = fastrand::Rng::with_seed(0x2a729);
        for (i, expected) in BUTTERFLIES.into_iter().enumerate() {
            let (t, b) = butterfly_birth(&rng);
            assert_eq!(t.translation.to_array().map(f32::to_bits), expected);
            assert_eq!((b.state, b.state_t.to_bits()), (2, BUTTERFLY_TIMERS[i]));
            assert_eq!([b.goal_timer, b.dwell, b.phase, b.phase_ref], [0.0; 4]);
            assert_eq!(t.rotation, glam::Quat::IDENTITY);
            assert_eq!(t.scale, Vec3::ONE);
        }
        let rng = fastrand::Rng::with_seed(0x29a);
        for (i, expected) in FIREFLIES.into_iter().enumerate() {
            let actual = firefly_birth(i as u32, &rng).translation.to_array();
            for (a, b) in actual.into_iter().zip(expected.map(f32::from_bits)) {
                assert!(
                    (a - b).abs() <= 4.0 * f32::EPSILON * b.abs().max(1.0),
                    "firefly{i}: {a} != {b}"
                );
            }
        }
    }

    use country_core::resources::{
        audio_instance::PlaybackState,
        audio_resources::{EventInstance, ListenerAttributes, VcaHandle},
    };
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    #[derive(Default)]
    struct Counts {
        created: AtomicUsize,
        released: AtomicUsize,
    }
    struct RecordingEvent(Arc<Counts>);
    impl EventInstance for RecordingEvent {
        type Error = String;
        fn start(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn stop(&mut self, _: i32) -> Result<(), String> {
            Ok(())
        }
        fn get_playback_state(&self) -> Result<PlaybackState, String> {
            Ok(PlaybackState::Stopped)
        }
        fn set_volume(&mut self, _: f32) -> Result<(), String> {
            Ok(())
        }
        fn set_parameter_by_name(&mut self, _: &str, _: f32) -> Result<(), String> {
            Ok(())
        }
        fn has_parameter(&self, _: &str) -> Result<bool, String> {
            Ok(false)
        }
        fn set_3d_attributes(
            &mut self,
            _: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
            _: [f32; 3],
        ) -> Result<(), String> {
            Ok(())
        }
        fn get_timeline_position(&self) -> Result<i32, String> {
            Ok(0)
        }
        fn pause(&mut self, _: bool) -> Result<(), String> {
            Ok(())
        }
        fn install_beat_callback(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn release(&mut self) -> Result<(), String> {
            self.0.released.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }
    struct Vca;
    impl VcaHandle for Vca {
        type Error = String;
        fn set_volume(&self, _: f32) -> Result<(), String> {
            Ok(())
        }
    }
    struct RecordingStudio {
        counts: Arc<Counts>,
        fail: bool,
    }
    impl Studio for RecordingStudio {
        type Error = String;
        type Event = RecordingEvent;
        type Vca = Vca;
        fn load_bank_file(&mut self, _: &str) -> Result<(), String> {
            Ok(())
        }
        fn load_sample_data(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn get_vca(&self, _: &str) -> Result<Vca, String> {
            Ok(Vca)
        }
        fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), String> {
            Ok(())
        }
        fn update(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn set_parameter_by_name(&self, _: &str, _: f32) -> Result<(), String> {
            Ok(())
        }
        fn create_event_instance(&self, name: &str) -> Result<Option<RecordingEvent>, String> {
            assert_eq!(name, assets::BUTTERFLY_SHOOSH_EVENT);
            if self.fail {
                return Err("fixture FMOD error".into());
            }
            self.counts.created.fetch_add(1, Ordering::SeqCst);
            Ok(Some(RecordingEvent(self.counts.clone())))
        }
    }
    #[test]
    fn births_reach_canonical_ecs_queries_and_entity_drop_releases_every_real_instance() {
        let counts = Arc::new(Counts::default());
        let audio = AudioCore {
            studio: Some(RecordingStudio {
                counts: counts.clone(),
                fail: false,
            }),
            banks: Default::default(),
        };
        let mut world = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        {
            let mut commands = Commands::new(&mut queue, &world);
            spawn_butterflies(&audio, &mut commands);
            spawn_fireflies(&mut commands);
        }
        queue.apply(&mut world);
        assert_eq!(
            world
                .query::<(
                    &Butterfly,
                    &ButterflyAudio,
                    &Critter,
                    &CritterLocomotion,
                    &CritterGoal,
                    &CritterVelocityTarget,
                    &Velocity,
                    &CritterVelocityMultiplier,
                    &Transform
                )>()
                .iter(&world)
                .count(),
            24
        );
        assert_eq!(
            world
                .query::<(
                    &Firefly,
                    &Critter,
                    &CritterLocomotion,
                    &CritterGoal,
                    &CritterVelocityTarget,
                    &Velocity,
                    &CritterVelocityMultiplier,
                    &Transform
                )>()
                .iter(&world)
                .count(),
            96
        );
        assert_eq!(counts.created.load(Ordering::SeqCst), 24);
        assert_eq!(counts.released.load(Ordering::SeqCst), 0);
        let entities = world.iter_entities().map(|e| e.id()).collect::<Vec<_>>();
        for entity in entities {
            world.despawn(entity);
        }
        assert_eq!(counts.released.load(Ordering::SeqCst), 24);
    }
    #[test]
    fn original_audio_error_panics_before_a_birth_is_queued() {
        let audio = AudioCore {
            studio: Some(RecordingStudio {
                counts: Arc::default(),
                fail: true,
            }),
            banks: Default::default(),
        };
        let world = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            spawn_butterflies(&audio, &mut Commands::new(&mut queue, &world))
        }));
        assert!(result.is_err());
        assert!(queue.is_empty());
    }
}
