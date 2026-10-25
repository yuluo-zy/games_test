// Narrow insertion into the real gameplay runtime installer. Existing GameSystemSet gating applies.
app.init_resource::<country_core::resources::critters::CritterControl>();
app.add_systems(bevy_app::Update,
    system_critters::control_critters::control_critters.in_set(crate::run_game::GameSystemSet(36)));
app.add_systems(bevy_app::Update,
    (system_critters::butterfly::butterflies_send_to_gpu,
     system_critters::firefly::fireflies_send_to_gpu).in_set(crate::run_game::GameSystemSet(44)));
