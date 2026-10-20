//! Original-native birth goldens and real ECS admission, without a GPU claim.
use bevy_ecs::{prelude::*, system::SystemState};
use country_core::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        shader_library::AssetShaderLibrary,
    },
    components::drawable::PrevTransform,
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::*},
        vao::Vao,
    },
    resources::critters::sheep::{Sheep, SheepWalkRecord},
    startup::sheep_assets::SheepStartupAssets,
    systems::{
        debug::sheep_controls::SheepControl,
        render_loop::{
            compute_passes::grass::ObjectGrassPush,
            draw_passes::{
                draw_callbacks_a::read_env2,
                draw_lists::{self, DrawLists},
            },
        },
    },
    utils::components::transform::Transform,
};
use std::sync::{Arc, Mutex};
use system_critters::{
    audio::{AudioHandle, CritterAudioSink},
    bird_spots::{BirdSpotManager, BirdSpots},
    sheep::sheep_behavior::SheepAudio,
    startup_sheep::spawn_startup_sheep,
};

// Only shader-library CPU metadata is needed by build_draw_lists. No physical
// pipeline or compiled-shader correctness is claimed by this test boundary.
struct MetadataBackend;
impl ShaderBackend for MetadataBackend {
    fn load_compiled_compute(
        &self,
        _: &ComputePipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        unreachable!()
    }
    fn load_compiled_raster(
        &self,
        _: &RasterPipelineDesc,
    ) -> Result<CompiledShaderPipeline, Error> {
        Ok(CompiledShaderPipeline(vec![]))
    }
    fn raster_shader_variants(&self, d: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
        [d.clone(), d.clone()]
    }
    fn create_compute_pipeline(
        &self,
        _: &str,
        _: Arc<CompiledShaderPipeline>,
    ) -> rhapsody::Result<ComputePipeline> {
        unreachable!()
    }
    fn create_raster_pipeline(
        &self,
        _: Arc<CompiledShaderPipeline>,
        _: &RasterPipelineKey,
    ) -> Result<RasterPipeline, Error> {
        unreachable!()
    }
}
fn assets() -> (AssetMeshLibrary, AssetShaderLibrary, SheepStartupAssets) {
    let meshes = AssetMeshLibrary::new();
    let shaders = AssetShaderLibrary::new();
    let mesh =
        |name: &'static str| meshes.add(MeshAsset::new(Mesh::new(name), Some(name.to_owned())));
    let shader = |name, path| {
        country_core::startup::load_shaders::load_shader_into_library(
            &shaders,
            &MetadataBackend,
            path,
            name,
        )
    };
    let sheep_shader = shader("sheep_shader", "/sheep.raster.hlsl");
    let accessory_shader = shader("solid_vertex_color", "/solid_vertex_color.raster.hlsl");
    shader("vertex_color_shader", "/vertex_color.raster.hlsl");
    let a = SheepStartupAssets {
        sheep_mesh: mesh("sheep_mesh"),
        sheep_shader,
        heart_mesh: mesh("sheep_heart"),
        umbrella_mesh: mesh("umbrella_mesh"),
        floatie_mesh: mesh("floatie_mesh"),
        accessory_shader,
    };
    (meshes, shaders, a)
}
#[derive(Debug)]
struct Audio(Arc<Mutex<Vec<(String, u32)>>>);
impl CritterAudioSink for Audio {
    fn start_if_not_playing(&mut self) {
        panic!("startup does not start Baah")
    }
    fn stop(&mut self) {
        panic!("startup does not stop Baah")
    }
    fn set_position(&mut self, _: glam::Vec3) {
        panic!("position is updated by its runtime consumer")
    }
    fn set_volume(&mut self, _: f32) {
        panic!("startup does not set volume")
    }
    fn set_parameter(&mut self, n: &str, v: f32) {
        self.0.lock().unwrap().push((n.into(), v.to_bits()));
    }
}
#[test]
fn original_three_births_reach_real_env2_and_velocity_draw_lists() {
    let (meshes, shaders, assets) = assets();
    let mut world = World::new();
    let mut birds = BirdSpotManager::default();
    let mut commands = SystemState::<Commands>::new(&mut world);
    let audio = Arc::new(Mutex::new(Vec::new()));
    let mut events = Vec::new();
    let born = spawn_startup_sheep(
        &assets,
        &meshes,
        &shaders,
        &mut birds,
        &mut commands.get_mut(&mut world),
        None,
        |name| {
            events.push(name.to_owned());
            Ok(AudioHandle::Active(Box::new(Audio(audio.clone()))))
        },
    )
    .unwrap();
    commands.apply(&mut world);
    assert_eq!(events, vec!["event:/SFX/Sheep/Baah"; 3]);
    assert_eq!(
        *audio.lock().unwrap(),
        vec![
            ("Pitch".into(), 0),
            ("Pitch".into(), 0x3e99999a),
            ("Pitch".into(), 0x3f800000)
        ]
    );
    // Golden values copied from complete original native policy, not from the
    // Rust producer: sheep-spawn-native-policy.json Transform and audio slots.
    let xz = [
        [-4.672393798828125f32, 1.7606306076049805],
        [-6.147923469543457, 14.931960105895996],
        [11.838197708129883, -13.561511039733887],
    ];
    let scales = [0x3f23d70b, 0x3f19999a, 0x3f051eb8];
    for (i, entity) in born.sheep.iter().copied().enumerate() {
        let t = *world.get::<Transform>(entity).unwrap();
        assert_eq!(
            t.translation.to_array().map(f32::to_bits),
            [xz[i][0].to_bits(), 0, xz[i][1].to_bits()]
        );
        assert_eq!(t.scale.to_array().map(f32::to_bits), [scales[i]; 3]);
        assert_eq!(
            *world.get::<PrevTransform>(entity).unwrap(),
            PrevTransform::from(t)
        );
        assert_eq!(world.get::<Sheep>(entity).unwrap().pet_target, i as u64);
        assert_eq!(
            world.get::<SheepWalkRecord>(entity).unwrap().last_pos,
            t.translation
        );
        assert_eq!(
            world.get::<SheepAudio>(entity).unwrap().elapsed.to_bits(),
            0
        );
        assert_eq!(
            world.get::<ObjectGrassPush>(entity).unwrap().0.to_bits(),
            (t.scale.x * 3.5).to_bits()
        );
        let spots = world.get::<BirdSpots>(entity).unwrap();
        assert_eq!(spots.spots.len(), 1);
        assert_eq!(spots.spots[0].id.0, i as u32);
        assert_eq!(spots.spots[0].pos, t.translation);
        assert_eq!(spots.spots[0].occupant, None);
        let env = read_env2(&world, entity);
        assert_eq!(env.sheep.q28, 0);
        assert_eq!(env.sheep.f58.to_bits(), 0);
        assert_eq!(env.sheep.b60, 1);
        // CPU list admission; the physical runtime is responsible for VAOs.
        world.entity_mut(entity).insert(Handle::<Vao>::new());
    }
    assert!(world.get::<SheepControl>(born.control).is_some());
    assert!(world.get::<Transform>(born.wireframe).is_none());
    assert!(world.get::<Transform>(born.solid).is_none());
    let mut lists = DrawLists::new();
    draw_lists::build_draw_lists(&mut world, &shaders, &mut lists);
    assert_eq!(lists.list(draw_lists::list::GBUFFER_VELOCITY).len(), 3);
    assert_eq!(lists.list(draw_lists::list::SHADOW).len(), 3);
    assert_eq!(lists.total_len(), 6);
    for e in born.sheep {
        world.despawn(e);
    }
    let mut returned = Vec::new();
    birds.recycle(|p| returned.push(p.id.0));
    assert_eq!(returned, [0, 1, 2]);
}
#[test]
fn audio_error_keeps_the_original_successful_command_prefix() {
    let (meshes, shaders, assets) = assets();
    let mut world = World::new();
    let mut birds = BirdSpotManager::default();
    let mut commands = SystemState::<Commands>::new(&mut world);
    let mut calls = 0;
    let result = spawn_startup_sheep(
        &assets,
        &meshes,
        &shaders,
        &mut birds,
        &mut commands.get_mut(&mut world),
        None,
        |_| {
            calls += 1;
            if calls == 2 {
                Err("original audio creation failure".into())
            } else {
                Ok(AudioHandle::Disabled)
            }
        },
    );
    assert!(result.is_err());
    commands.apply(&mut world);
    assert_eq!(world.query::<&Sheep>().iter(&world).count(), 1);
    assert_eq!(world.query::<&SheepControl>().iter(&world).count(), 0);
    assert!(
        meshes
            .try_get_handle_by_name("sheep_control_cube")
            .is_none()
    );
}
