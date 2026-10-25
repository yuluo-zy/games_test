//! Full B startup sheep births (140a093e0), after the canonical asset branch.
//! Native policy is recorded in sheep-spawn-native-policy.json. Audio creation
//! is a fallible host boundary; an active backend error must not become Disabled.
use crate::{
    audio::AudioHandle,
    bird_spots::{BirdSpot, BirdSpotManager},
    components::{
        Critter, CritterGoal, CritterLocomotion, CritterVelocityMultiplier, CritterVelocityTarget,
        Sheep, SheepMoveState, Velocity,
    },
    sheep::{
        sheep_behavior::SheepAudio,
        sheep_floatie::FloatieHolder,
        sheep_unstuck::SheepGroundRecord,
        sheep_umbrella::UmbrellaHolder,
    },
};
use bevy_ecs::{entity::Entity, system::Commands};
use country_core::{
    asset_libraries::{
        mesh_library::{AssetMeshLibrary, MeshAsset},
        shader_library::AssetShaderLibrary,
    },
    components::drawable::PrevTransform,
    geometry::cube::{CubeBox, cube_mesh},
    resources::critters::sheep::{SheepWalkRecord, control::SheepControlSnapshot},
    startup::sheep_assets::SheepStartupAssets,
    systems::{
        debug::sheep_controls::SheepControl,
        render_loop::{
            compute_passes::grass::ObjectGrassPush, draw_passes::draw_lists::WireframePass,
        },
    },
    utils::components::transform::Transform,
};
use glam::{Quat, Vec3};

/// The original +0 u32 and baseline scale +0x3c are retained, rather than
/// dropping fields currently unused by the split animation component design.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]
pub struct SheepBirthState {
    pub native_00: u32,
    pub baseline_scale: Vec3,
}
pub struct SheepStartupEntities {
    pub sheep: [Entity; 3],
    pub control: Entity,
    pub wireframe: Entity,
    pub solid: Entity,
}

/// Original SheepControl::new registers one cube and spawns two handle tuples.
/// Neither debug entity gets a Transform until the real controls runtime shows
/// it. Adding an identity Transform here would incorrectly make it drawable.
fn spawn_control(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    commands: &mut Commands<'_, '_>,
    snapshot: Option<SheepControlSnapshot>,
) -> (Entity, Entity, Entity) {
    let cube = cube_mesh(&CubeBox {
        x: [-0.5, 0.5],
        y: [-0.5, 0.5],
        z: [-0.5, 0.5],
    });
    let mesh = meshes.add(MeshAsset::new(cube, Some("sheep_control_cube".into())));
    let shader = shaders
        .get_raster_handle_by_name("vertex_color_shader")
        .expect("vertex_color_shader must already be registered");
    let wireframe = commands.spawn((mesh, shader, WireframePass)).id();
    // Preserve the second original lookup and its failure boundary.
    let shader = shaders
        .get_raster_handle_by_name("vertex_color_shader")
        .expect("vertex_color_shader must already be registered");
    let solid = commands.spawn((mesh, shader)).id();
    let mut state = SheepControl::new(wireframe, solid);
    state.snapshot = snapshot;
    let control = commands.spawn(state).id();
    (control, wireframe, solid)
}

/// This is the ECS half of original startup_sheep. The caller first executes
/// load_sheep_startup_assets, then supplies the loaded-session snapshot DTO.
/// Three sheep commands precede SheepControl's geometry and entity commands.
pub fn spawn_startup_sheep(
    assets: &SheepStartupAssets,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    birds: &mut BirdSpotManager,
    commands: &mut Commands<'_, '_>,
    snapshot: Option<SheepControlSnapshot>,
    mut instantiate_audio: impl FnMut(&str) -> Result<AudioHandle, String>,
) -> Result<SheepStartupEntities, String> {
    let mut rng = fastrand::Rng::with_seed(0x16e5c);
    let mut entities = Vec::with_capacity(3);
    for index in 0..3 {
        let size = [0.8f32, 0.75, 0.65][index] * 0.8;
        let pitch = [0.0, 0.3, 1.0][index];
        let mut audio = instantiate_audio("event:/SFX/Sheep/Baah")?;
        audio.set_parameter("Pitch", pitch);
        let x = rng.f32();
        let z = rng.f32();
        let id = birds.gen_id();
        let position = Vec3::new((x - 0.5) * 40.0, 0.0, (z - 0.5) * 40.0);
        let transform = Transform {
            rotation: Quat::IDENTITY,
            translation: position,
            scale: Vec3::splat(size),
        };
        let bird_spots = birds.create_spots(vec![BirdSpot {
            occupant: None,
            pos: position,
            id,
        }]);
        let sheep = Sheep {
            behavior_state: 0,
            goal_timer: 0.0,
            pet_phase: 0,
            pet_progress: 0.0,
            pet_target: index as u64,
            move_state: SheepMoveState::Idle { timeout_s: 0.0 },
            amplitude: 0.0,
            control_flag: 1,
        };
        // Native's inactive pet progress / goal coordinates / locomotion cache
        // are uninitialized. Safe Rust initializes inactive storage to zero;
        // active discriminants and all subsequently read fields remain exact.
        entities.push(
            commands
                .spawn((
                    (
                        transform,
                        PrevTransform::from(transform),
                        assets.sheep_mesh,
                        assets.sheep_shader,
                    ),
                    (
                        sheep,
                        SheepWalkRecord {
                            last_pos: position,
                            ..Default::default()
                        },
                        SheepBirthState {
                            native_00: 0,
                            baseline_scale: Vec3::splat(size),
                        },
                    ),
                    SheepAudio {
                        handle: audio,
                        startup_pitch: pitch,
                        elapsed: 0.0,
                    },
                    (
                        Critter,
                        CritterLocomotion::new(2),
                        CritterGoal {
                            goal_tag: 0,
                            position: Vec3::ZERO,
                        },
                        CritterVelocityTarget {
                            direction: Vec3::ZERO,
                            retention_per_second: 0.01,
                        },
                        Velocity(Vec3::ZERO),
                        CritterVelocityMultiplier(1.0),
                    ),
                    SheepGroundRecord::default(),
                    ObjectGrassPush(size * 3.5),
                    UmbrellaHolder {
                        mesh: Some(assets.umbrella_mesh),
                        shader: Some(assets.accessory_shader),
                        spawned: None,
                        initialized: false,
                    },
                    FloatieHolder {
                        mesh: Some(assets.floatie_mesh),
                        shader: Some(assets.accessory_shader),
                        spawned: None,
                        initialized: false,
                    },
                    bird_spots,
                ))
                .id(),
        );
    }
    let (control, wireframe, solid) = spawn_control(meshes, shaders, commands, snapshot);
    Ok(SheepStartupEntities {
        sheep: entities.try_into().expect("three native sheep births"),
        control,
        wireframe,
        solid,
    })
}
