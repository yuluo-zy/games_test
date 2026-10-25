// Install only on the actual gameplay App after the same renderer installs
// its GraphicsDeviceResource, AssetSsboLibrary and shader owner. Original phase
// is Startup eight-tuple item5. Original PreStartup already loads COMMON13;
// there is no added guessed chain relation among Startup peers.
pub fn install_terrain_startup(
    app: &mut bevy_app::App,
    backend: std::sync::Arc<crate::render_host::ShaderHost>,
    installation: std::path::PathBuf,
) {
    app.add_systems(
        bevy_app::Startup,
        move |device: bevy_ecs::prelude::Res<country_core::render::GraphicsDeviceResource>,
              config: bevy_ecs::prelude::Res<country_core::game_config::GameConfig>,
              meshes: bevy_ecs::prelude::Res<
            country_core::asset_libraries::mesh_library::AssetMeshLibrary,
        >,
              watch: bevy_ecs::prelude::Res<
            country_core::asset_libraries::meshwatch::MeshWatch,
        >,
              shaders: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>,
              ssbos: bevy_ecs::prelude::Res<
            country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
        >,
              mut commands: bevy_ecs::system::Commands|
              -> Result<(), bevy_ecs::error::BevyError> {
            country_core::startup::startup_terrain::startup_terrain(
                &installation,
                config.dev.demo,
                device.device(),
                backend.as_ref(),
                &meshes,
                &watch,
                &shaders.0,
                &ssbos,
                &mut commands,
            )
            .map_err(Into::into)
        },
    );
}
// GameLoop gameplay branch after the actual asset libraries are installed:
// crate::world_terrain::install_terrain_startup(&mut app,
//     renderer.bridge().hosts().shaders.clone(),
//     options.compiled_shader_root.parent().context("compiled shader directory has no installation root")?.to_path_buf());
