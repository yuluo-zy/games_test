//! Main-thread host shutdown maintenance while no drawable frame can run.
//!
//! B's zero-extent branch skips App::update. Its real close producer emits
//! ShutdownGameCmd, consumed by save_game@140a3f470 and
//! initiate_game_shutdown@140b2ba40. This host-only maintenance route reuses
//! those same persistent systems; it is not an additional native B branch.
use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
use country_core::{
    resources::{events::ShutdownGameCmd, session::ActiveSession, shutdown::ShutdownGame},
    systems::save::{save_game::{AutoSaveState, save_game_system}, save_thread::SaveThreadHandle},
};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct SaveMaintenance;
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct ShutdownMaintenance;

/// Install exactly one save system, including its EventReaders and replay
/// tracker Local. Normal set43 and a zero-extent close run this same instance.
pub(crate) fn install_save_maintenance(app: &mut App) {
    app.init_schedule(SaveMaintenance);
    app.add_systems(SaveMaintenance,
        save_game_system.run_if(bevy_ecs::schedule::common_conditions::resource_exists::<ActiveSession>));
    app.add_systems(Update, run_save_maintenance.in_set(crate::run_game::GameSystemSet(43)));
}

/// Loading has no ActiveSession or complete save SystemParams. It installs
/// only this actual command-to-marker consumer, on its own owning World.
pub(crate) fn install_shutdown_maintenance(app: &mut App) {
    app.init_schedule(ShutdownMaintenance);
    app.add_systems(ShutdownMaintenance,
        country_core::resources::shutdown::initiate_game_shutdown);
    app.add_systems(PostUpdate, run_shutdown_maintenance);
}

fn run_save_maintenance(world: &mut World) {
    world.run_schedule(SaveMaintenance);
}
fn run_shutdown_maintenance(world: &mut World) {
    // A schedule run applies this exact system's Commands before returning,
    // just as the native PostUpdate flush does before main_loop sees marker.
    world.run_schedule(ShutdownMaintenance);
}

/// Preserve real save publication, metadata, async FIFO Flush/reply and its
/// error handling before the real deferred quit marker. No simulation, time,
/// MEI, rendering or invented loading resources run on this host-only path.
pub(crate) fn finish_pending_shutdown_without_frame(world: &mut World) -> bool {
    if world.contains_resource::<ShutdownGame>() {
        return true;
    }
    let pending = world.get_resource::<Events<ShutdownGameCmd>>()
        .is_some_and(|requests| !requests.is_empty());
    if !pending {
        return false;
    }
    if world.contains_resource::<ActiveSession>() {
        run_save_maintenance(world);
    }
    run_shutdown_maintenance(world);
    world.contains_resource::<ShutdownGame>()
}

/// End the real sender owners, then wait for the original save worker's
/// Disconnected terminal branch (141720f8c...), which drains queued commands
/// before flushing deferred sections. No fabricated shutdown/ack command.
pub(crate) fn disconnect_save_worker(world: &mut World) -> anyhow::Result<()> {
    let handle = world.remove_resource::<SaveThreadHandle>();
    drop(world.remove_resource::<AutoSaveState>());
    if let Some(SaveThreadHandle { join, command_sender, error_state }) = handle {
        drop(command_sender);
        let result = join.join();
        drop(error_state);
        result.map_err(|_| anyhow::anyhow!("async save worker panicked during shutdown"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Default)]
    struct Updates(u32);

    #[test]
    fn loading_close_consumes_real_request_without_save_params_or_app_update() {
        let mut app = App::new();
        app.add_event::<ShutdownGameCmd>();
        install_shutdown_maintenance(&mut app);
        app.init_resource::<Updates>();
        app.add_systems(Update, |mut count: ResMut<Updates>| count.0 += 1);
        assert!(!finish_pending_shutdown_without_frame(app.world_mut()));
        app.world_mut().send_event(ShutdownGameCmd);
        assert!(finish_pending_shutdown_without_frame(app.world_mut()));
        assert_eq!(app.world().resource::<Updates>().0, 0);
        assert!(!app.world().contains_resource::<ActiveSession>());
        assert!(!app.world().contains_resource::<AutoSaveState>());
        assert!(!app.world().contains_resource::<SaveThreadHandle>());
        // Retained EventReader is shared with PostUpdate, not re-created for
        // each zero frame. An already-read buffer cannot emit marker twice.
        app.world_mut().remove_resource::<ShutdownGame>();
        assert!(!finish_pending_shutdown_without_frame(app.world_mut()));
        app.update();
        assert!(!app.world().contains_resource::<ShutdownGame>());
        app.world_mut().send_event(ShutdownGameCmd);
        app.update();
        assert!(app.world().contains_resource::<ShutdownGame>());
    }
}
