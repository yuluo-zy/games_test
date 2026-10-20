// Insert inside install_gameplay_gpu_startup, after the session set chain is
// configured and before installed/hosts are moved into PreStartup closures.
// The old mesh-only/run_once tuple was already removed centrally.
let critter_root = installed.clone();
let critter_backend = hosts.shaders.clone();
app.add_systems(
    bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
    (move |session: Res<country_core::resources::session::ActiveSession>,
           settings: Res<country_core::resources::glade_settings::GladeSettings>,
           audio: Res<country_core::resources::audio_resources::AudioCore<country_core::resources::native_audio::NativeStudio>>,
           meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
           shaders: Res<ApplicationShaderLibrary>, mut commands: Commands| {
        system_critters::startup_critters::startup_critters(
            &meshes, &shaders.0, critter_backend.as_ref(), &watch, &critter_root,
            &audio, &settings, &session, &mut commands,
        );
    })
        .in_set(GameplaySessionStartup::Critters)
        .after(country_core::startup::startup_session::load_game_session),
);
