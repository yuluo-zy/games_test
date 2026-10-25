// Central-only narrow insertion in install_gameplay_gpu_startup. Original
// register order is the already declared GameplaySessionStartup::Sheep.
app.init_resource::<system_critters::bird_spots::BirdSpotManager>();
let sheep_root = installed.clone();
let sheep_backend = hosts.shaders.clone();
let sheep_device = hosts.buffers.clone();
app.add_systems(
    bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
    (move |meshes: Res<AssetMeshLibrary>, watch: Res<MeshWatch>,
        shaders: Res<ApplicationShaderLibrary>, ssbos: Res<AssetSsboLibrary>,
        audio: Res<country_core::resources::audio_resources::AudioCore<country_core::resources::native_audio::NativeStudio>>,
        mut birds: ResMut<system_critters::bird_spots::BirdSpotManager>,
        mut pending: Option<ResMut<country_core::resources::critters::sheep::control::PendingSheepSnapshot>>,
        mut commands: Commands| {
        let assets = country_core::startup::sheep_assets::load_sheep_startup_assets(
            &sheep_root, &meshes, &shaders.0, sheep_backend.as_ref(), &watch,
            &ssbos, sheep_device.as_ref(),
        ).expect("original complete sheep animation asset producer");
        let snapshot = pending.as_mut().and_then(|p| p.0.take());
        commands.remove_resource::<country_core::resources::critters::sheep::control::PendingSheepSnapshot>();
        system_critters::startup_sheep::spawn_startup_sheep(
            &assets, &meshes, &shaders.0, &mut birds, &mut commands, snapshot,
            |name| {
                use country_core::resources::audio_resources::AudioInstance;
                match audio.instantiate_audio(name) {
                    AudioInstance::Disabled => Ok(system_critters::audio::AudioHandle::Disabled),
                    AudioInstance::Err(error) => Err(error),
                    active @ AudioInstance::Active { .. } =>
                        Ok(system_critters::audio::AudioHandle::Active(Box::new(active))),
                }
            },
        ).expect("original complete sheep ECS producer");
    }).in_set(GameplaySessionStartup::Sheep)
        .after(country_core::startup::startup_session::load_game_session),
);
