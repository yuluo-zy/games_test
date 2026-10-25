// Integration fragment for the already authoritative App; no replacement owner.
// B run_game_thread1405a019c/19e => in_set7, immediately add_systems(Update).
// These three leaves belong to the original unchained six-system tuple.
app.add_event::<country_core::systems::brush_preview::OverwriteBrushPreview>();
app.add_event::<country_core::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>();
app.add_systems(bevy_app::Update, (
    country_core::systems::brush_preview::brush_preview,
    country_core::systems::look_at_camera::look_at_camera,
    country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation,
).in_set(GameSystemSet(7)));
// PathPlugin.build140b68e40 chains draw_path -> size using chain_ignore_deferred,
// in_set16 (140b68f06), Update (140b69040); no run_if. Integrate this leaf AFTER
// real draw_path in that same chain if/when the complete producer is mounted.
// No separate follow_mouse registration: native follow is in existing set10
// mouse_terrain_raycast and already belongs to the actual camera/raycast tuple.
