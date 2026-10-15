//! GUI resource consumers. The renderer supplies an egui frame/context; these
//! systems use the same DebugGui resource as the existing draw callbacks.
use bevy_ecs::{
    schedule::{IntoScheduleConfigs, Schedule},
    system::{Res, ResMut},
    world::World,
};

use crate::input::action_set::MainActionSet;
use crate::resources::{
    gui::{DebugGui, EguiContextResource, PendingEguiContextResource},
    time::GameTime,
};

use super::flush_screen_log::flush_error_log_to_screen;

/// Move the pending context into its active resource. This host integration
/// system preserves B's move conversion; it does not create a second context or
/// begin/end an egui frame behind the renderer's back.
pub fn publish_pending_egui_context(world: &mut World) {
    if let Some(pending) = world.remove_resource::<PendingEguiContextResource>() {
        world.insert_resource(pending.into_egui_context_resource());
    }
}

/// B tiny_glade::gui::gui_error_log::do_gui_error_log (0x140160410), moved into
/// core. Its original typed system takes DebugGui, optional EguiContextResource,
/// and GameTime; the closure subtracts GameTime +0x28, not wall-clock time.
pub fn show_screen_log(
    mut gui: ResMut<DebugGui>,
    context: Option<Res<EguiContextResource>>,
    time: Res<GameTime>,
) {
    // Preserve the original early-return change tick: hidden or context-less
    // frames do not mutably dereference DebugGui and do not age entries.
    if !gui.show_2d_ui || !gui.screen_log.entries.iter().any(Option::is_some) {
        return;
    }
    let Some(context) = context else {
        return;
    };
    gui.show_screen_log(Some(&context), time.delta);
}

/// The recovered panels that mutate DebugGui itself. The original outer window
/// is titled HAAAAX; larger cross-domain panels are still separate recovery work.
pub fn show_debug_controls(
    mut gui: ResMut<DebugGui>,
    context: Option<Res<EguiContextResource>>,
    actions: Res<MainActionSet>,
) {
    let Some(context) = context else {
        return;
    };
    // B 0x140148017..0x140148067: the context guard precedes action 28.
    if actions.digital[28].just_pressed() {
        gui.enabled = !gui.enabled;
    }
    if gui.enabled {
        egui::Window::new("HAAAAX").show(&context, |ui| gui.show_controls(ui));
    }
    // B 0x140148299..0x1401482d4 runs after the first window, even when it is
    // closed. The experimental camera/snapshot panel is a separate consumer.
    if actions.digital[33].just_pressed() {
        gui.experimental_window = !gui.experimental_window;
    }
}

/// Explicit local integration order, not a claim that the full original game
/// schedule has been recovered. MainActionSet, GameTime and egui frame ownership
/// come from the caller. Producer messages are flushed before rendering the log.
pub fn configure_gui_consumers(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<DebugGui>();
    schedule.add_systems(
        (
            publish_pending_egui_context,
            flush_error_log_to_screen,
            show_debug_controls,
            show_screen_log,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::backend::flush_screen_log::{log_error_to_screen, log_warning_to_screen};

    #[test]
    fn producer_queue_reaches_the_active_context_and_ages_by_game_delta() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        let context = egui::Context::default();
        let marker = egui::Id::new("pending-context-identity");
        context.data_mut(|data| data.insert_temp(marker, 37_u32));
        world.insert_resource(PendingEguiContextResource(context.clone()));
        world.insert_resource(GameTime {
            delta: 0.25,
            ..Default::default()
        });
        world.insert_resource(MainActionSet::default());
        configure_gui_consumers(&mut world, &mut schedule);
        log_error_to_screen("error");
        log_warning_to_screen("warning");
        let output = context.run(egui::RawInput::default(), |_| schedule.run(&mut world));
        assert!(world.get_resource::<PendingEguiContextResource>().is_none());
        assert_eq!(
            world
                .resource::<EguiContextResource>()
                .data(|data| data.get_temp::<u32>(marker)),
            Some(37)
        );
        let entries = &world.resource::<DebugGui>().screen_log.entries;
        assert_eq!(entries[0].as_ref().unwrap().message, "error");
        assert_eq!(entries[0].as_ref().unwrap().color, egui::Color32::RED);
        assert_eq!(entries[1].as_ref().unwrap().color, egui::Color32::YELLOW);
        assert_eq!(entries[1].as_ref().unwrap().remaining_seconds, 4.75);
        assert!(!output.shapes.is_empty());
    }

    #[test]
    fn hidden_or_absent_context_preserves_log_lifetime() {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.insert_resource(GameTime {
            delta: 2.0,
            ..Default::default()
        });
        let mut gui = DebugGui::default();
        gui.log_to_screen("kept", egui::Color32::WHITE);
        world.insert_resource(gui);
        schedule.add_systems(show_screen_log);
        schedule.run(&mut world);
        let context = egui::Context::default();
        world.insert_resource(EguiContextResource(context.clone()));
        world.resource_mut::<DebugGui>().show_2d_ui = false;
        let _ = context.run(egui::RawInput::default(), |_| schedule.run(&mut world));
        assert_eq!(
            world.resource::<DebugGui>().screen_log.entries[0]
                .as_ref()
                .unwrap()
                .remaining_seconds,
            5.0
        );
    }

    #[test]
    fn debug_actions_require_context_and_toggle_independent_windows_on_edges() {
        use crate::input::action_set::DigitalActionState;

        let mut world = World::new();
        let mut schedule = Schedule::default();
        world.init_resource::<DebugGui>();
        let mut actions = MainActionSet::default();
        actions.digital[28] = DigitalActionState { disc: 0, val: 0.0 };
        actions.digital[33] = DigitalActionState { disc: 2, val: 0.0 };
        world.insert_resource(actions);
        schedule.add_systems(show_debug_controls);
        schedule.run(&mut world);
        assert!(!world.resource::<DebugGui>().enabled);
        assert!(!world.resource::<DebugGui>().experimental_window);

        let context = egui::Context::default();
        world.insert_resource(EguiContextResource(context.clone()));
        let _ = context.run(egui::RawInput::default(), |_| schedule.run(&mut world));
        assert!(world.resource::<DebugGui>().enabled);
        assert!(world.resource::<DebugGui>().experimental_window);

        world.resource_mut::<MainActionSet>().digital[28].disc = 1;
        world.resource_mut::<MainActionSet>().digital[33].val = 0.1;
        let _ = context.run(egui::RawInput::default(), |_| schedule.run(&mut world));
        assert!(world.resource::<DebugGui>().enabled);
        assert!(world.resource::<DebugGui>().experimental_window);

        world.resource_mut::<MainActionSet>().digital[28].disc = 0;
        let _ = context.run(egui::RawInput::default(), |_| schedule.run(&mut world));
        assert!(!world.resource::<DebugGui>().enabled);
        assert!(world.resource::<DebugGui>().experimental_window);
    }
}
