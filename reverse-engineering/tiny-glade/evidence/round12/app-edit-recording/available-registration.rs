//! Evidence-only candidate for the current canonical owners. No fake missing
//! splitter/merge/bulldozer/signifier systems are installed. See REPORT.md.
use crate::run_game::GameSystemSet;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use country_core::resources::history::MaybePendingSnapshotId;
use country_core::systems::{history, wall};

pub fn install_available_wall_recording(app: &mut App) {
    wall::registration::register_wall_drawing_resources(app.world_mut());
    history::recording::register_recording_events(app.world_mut());
    // Original add_history_systems @1409438ce: set14, never after wall19.
    app.add_systems(
        Update,
        history::recording::history_start_recording::<MaybePendingSnapshotId>
            .in_set(GameSystemSet(14)),
    );
    // Exact relative order of implemented drawing/snapping leaves from native
    // main18. This is an explicitly incomplete slice, not the full native tuple.
    app.add_systems(
        Update,
        (
            wall::ui_draw_wall::ui_draw_wall,
            wall::ui_continue_wall::suppress_continue_wall,
            wall::ui_continue_wall::ui_continue_wall,
            wall::snapping::snapping_init::snapping_init,
            wall::snapping::snapping_to_ends::snapping_to_ends,
            wall::snapping::snapping_to_segments::snapping_to_segments,
            wall::snapping::snapping_maintain::check_for_disable_snapping,
            wall::snapping::snapping_audio::snapping_audio,
            wall::snapping::snapping_maintain::snapping_signifiers,
            wall::snapping::snapping_maintain::snapping_maintain,
            wall::draw_wall::draw_wall,
            wall::finish_wall::finish_wall,
            wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged,
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(19))
            .before(wall::adjust_wall_height::adjust_wall_height),
    );
    // Existing typed-event adaptation of the original direct History.add.
    // The event bridge and drain belong before transaction finish, in set34.
    app.add_systems(
        Update,
        (
            wall::deferred_wall_history::history_record_wall_changed,
            wall::history_recording::record_wall_history::<MaybePendingSnapshotId>,
        )
            .chain_ignore_deferred()
            .in_set(GameSystemSet(34)),
    );
    // Attach this before the CURRENT set35 process; do not install all history
    // again, which duplicates replay40 and Pop41 consumers.
    app.add_systems(
        Update,
        history::recording::history_finish_recording_system::<MaybePendingSnapshotId>
            .in_set(GameSystemSet(35))
            .before(history::history_process_system::<MaybePendingSnapshotId>),
    );
}
