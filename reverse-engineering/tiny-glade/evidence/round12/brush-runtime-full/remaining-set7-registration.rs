// Complete the original unchained set7 tuple on the existing App.
// The other three leaves were delivered in brush-startup/runtime-registration.rs.
app.add_systems(bevy_app::Update, (
 country_core::systems::debug::human_scale_ref::human_scale_ref,
 country_core::systems::ui_2d_follow_mouse::ui_2d_follow_mouse,
 country_core::systems::mesh_follow_terrain::mesh_follow_terrain,
).in_set(GameSystemSet(7)));
// No run_if, no chain. Native Query/resource permissions are preserved.
