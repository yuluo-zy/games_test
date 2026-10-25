// Original registration replacement for root-owned history_runtime::install.
// Remove the current chained CPU group in set40 and switch_roof_type in40.
// Keep replay_roof alone in40. Association/publication belongs wall-state and
// must be scheduled using original caller evidence; not part of this tuple.
app.add_systems(Update, (
    (
        (
            system_roof::delete_roof::mark_roof_for_deletion,
            system_roof::update_roof_spatial::update_roof_spatial,
            system_roof::wall_height_dt::wall_height_dt,
            system_roof::visual::roof_animation::roof_animation,
        ).chain_ignore_deferred(),
        system_roof::create_roof::create_roof,
        system_roof::edit_roof::edit_roof,
    ).chain(),
    system_roof::simulate_roof_tile_for_audio::simulate_roof_tile_for_audio,
    system_roof::highlight_roof::highlight_roof,
    system_roof::visual::switch_roof_type::switch_roof_type,
    system_roof::roof_trigger_volume::calculate_roof_trigger_volumes,
).chain().in_set(GameSystemSet(20)));
app.add_systems(Update,
    system_roof::delete_roof::delete_roofs.in_set(GameSystemSet(21)));
app.add_systems(Update, (
    system_roof::visual::assemble_roof_tiles::assemble_roof_tiles,
    system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom,
    system_roof::visual::place_spires::place_roof_spires,
).in_set(GameSystemSet(29)));
app.add_systems(Update,
    system_roof::deferred_record_roof_history::roof_record_history.in_set(GameSystemSet(34)));
app.add_systems(Update,
    system_roof::ui_edit_roof_ridge_dir::record_switch_roof_ridge_history.in_set(GameSystemSet(34)));
app.add_systems(Update,
    system_roof::replay_roof::replay_roof.in_set(GameSystemSet(40)));
app.add_systems(Update,
    system_roof::gpu_flush_buffers.in_set(GameSystemSet(44)));
// UI17/18 remains unrecovered: current ui_edit_roof/do_ui_edit_roof and focused
// UI files explicitly contain no-op placeholders. Registration alone does not
// restore those behaviors. Do not call that interaction chain complete.
