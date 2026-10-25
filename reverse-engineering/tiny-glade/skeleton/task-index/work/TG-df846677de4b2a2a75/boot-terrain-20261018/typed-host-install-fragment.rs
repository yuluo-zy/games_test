// Append to tiny-glade/src/world_terrain.rs when integrating this family.
// Original schedule edge: outer OnEnter(Game) chain runs load_game_session
// before the inner tuple that contains startup_grass. Inner tuple itself is
// not chained; no extra ordering versus TOD/other peers is asserted here.
pub fn install_grass_startup(
    app: &mut bevy_app::App,
    backend: std::sync::Arc<crate::render_host::ShaderHost>,
) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    app.add_systems(
        bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        (move |meshes: bevy_ecs::prelude::Res<
            country_core::asset_libraries::mesh_library::AssetMeshLibrary,
        >,
               mesh_watch: bevy_ecs::prelude::Res<
            country_core::asset_libraries::meshwatch::MeshWatch,
        >,
               shaders: bevy_ecs::prelude::Res<crate::run_game::ApplicationShaderLibrary>,
               mut commands: bevy_ecs::system::Commands| {
            country_core::startup::startup_grass::startup_grass(
                &meshes,
                &mesh_watch,
                &shaders.0,
                backend.as_ref(),
                &mut commands,
            );
        })
        .after(country_core::startup::startup_session::load_game_session),
    );
}
// GameLoop::new, gameplay branch after actual device/asset installation:
// crate::world_terrain::install_grass_startup(
//     &mut app, renderer.bridge().hosts().shaders.clone());
// Do not install this on the separate loading App.
