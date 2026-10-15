//! B mode manager: complete input ordering and toolbar transitions from original
//! 0x140a86b60..0x140a875d7; thirteen five-entry ActionState jump tables.
//! History::is_recording 0x140a86970 is the guard (not an event-queue length).
//! Event/audio/history payload ownership is explicit at the integration boundary.
use crate::input::action_set::MainActionSet;
use crate::recovered_building::history_transactions::HistoryTransactions;
use crate::resources::gui::TimeOfDayScreen;
use crate::resources::toolbar::{Toolbar, ToolbarCategory};
use bevy_ecs::event::{Event, EventReader, EventWriter, Events};
use bevy_ecs::prelude::{Res, ResMut, Resource, World};
use bevy_ecs::schedule::{IntoScheduleConfigs, Schedule};

mod types;
pub use types::{AppMode, ClutterMeshTy, DecoratorType, EditSubject, EraserLayer, WallShape};

impl Resource for MainActionSet {}
impl<M: Send + Sync + 'static, S: Send + Sync + 'static> Resource for HistoryTransactions<M, S> {}

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestAppModeChange {
    pub mode: AppMode,
}
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestToolbarCategoryChange(pub ToolbarCategory);
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadingGameComplete {
    pub restore_to: AppMode,
}
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RestoreAppTo(pub AppMode);

/// Exact four-byte command crossing the history-process boundary. That owner's
/// nested command enum is not guessed here. Known original constructors are
/// Undo=0x00000000, Redo=0x01000100 and Interrupt=0x02000200 (0x140af5b20).
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryCommand {
    pub bytes: [u8; 4],
}
impl HistoryCommand {
    pub const UNDO: Self = Self {
        bytes: [0, 0, 0, 0],
    };
    pub const REDO: Self = Self {
        bytes: [0, 1, 0, 1],
    };
    pub const INTERRUPT: Self = Self {
        bytes: [0, 2, 0, 2],
    };
}
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestHistoryCommand(pub HistoryCommand);
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiHistoryPlaySelectAnimation(pub bool);
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearCanvasCmd(pub bool);
/// The two PlayUiSound variants emitted here. Names describe their verified
/// use; numeric values preserve the original audio consumer boundary.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ModeManagerSound {
    SwitchBlocked = 4,
    TimeOfDayToggled = 14,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EscMenuProbe {
    pub hovered: bool,
    pub widget_id: u64,
    pub hit_tag: u32,
}
/// Projection from UiState. The owning UI supplies its real canvas identifier;
/// no guessed hash or fabricated UI state is inserted by configuration.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModeManagerUiContext {
    pub probe: EscMenuProbe,
    pub esc_menu_canvas_id: u64,
}
pub fn esc_menu_blocks(probe: EscMenuProbe, esc_menu_canvas_id: u64) -> bool {
    probe.hovered && probe.widget_id == esc_menu_canvas_id && probe.hit_tag != 2
}

#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppModeChanged {
    pub current: AppMode,
    pub previous: AppMode,
}
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PreviousAppMode(pub AppMode);
pub fn send_mode_just_changed_event(
    current: Res<AppMode>,
    mut previous: ResMut<PreviousAppMode>,
    mut events: EventWriter<AppModeChanged>,
) {
    if *current != previous.0 {
        events.write(AppModeChanged {
            current: *current,
            previous: previous.0,
        });
        previous.0 = *current;
    }
}

/// All ordinary category closures, including explicit RequestToolbarCategoryChange.
pub fn category_advance(category: ToolbarCategory, toolbar: &mut Toolbar, mode: &mut AppMode) {
    if let Some(next) = toolbar.advance_category(category) {
        *mode = next;
    }
}
/// RequestAppModeChange closure 0x140afa2c0. Editing and CloneClutter select
/// Editing's toolbar slot while preserving the exact requested editing target.
pub fn select_app_mode(request: AppMode, toolbar: &mut Toolbar, mode: &mut AppMode) {
    *mode = request;
    match request {
        AppMode::Editing(_) | AppMode::CloneClutter => {
            toolbar.advance_category(ToolbarCategory::Editing);
        }
        _ => toolbar.switch_mode(&request),
    }
}
/// Photomode shortcut closure 0x140afa9f0; selecting the category explicitly
/// uses ordinary cycling, while this shortcut toggles back to Editing(None).
pub fn toggle_photomode(toolbar: &mut Toolbar, mode: &mut AppMode) {
    if *mode == AppMode::Photomode {
        *mode = AppMode::Editing(EditSubject::None);
        toolbar.switch_mode(mode);
    } else {
        category_advance(ToolbarCategory::Photomode, toolbar, mode);
    }
}

/// The exact mode-manager ordering. It consumes the existing HistoryTransactions
/// owner, whose pending Option corresponds to native History+0x30.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn mode_manager<M: Send + Sync + 'static, S: Send + Sync + 'static>(
    mut mode: ResMut<AppMode>,
    mut restore: ResMut<RestoreAppTo>,
    mut toolbar: ResMut<Toolbar>,
    ui: Res<ModeManagerUiContext>,
    history: Res<HistoryTransactions<M, S>>,
    actions: Res<MainActionSet>,
    mut tod: ResMut<TimeOfDayScreen>,
    (mut loaded, mut requested_modes, mut requested_categories, mut requested_history): (
        EventReader<LoadingGameComplete>,
        EventReader<RequestAppModeChange>,
        EventReader<RequestToolbarCategoryChange>,
        EventReader<RequestHistoryCommand>,
    ),
    (mut clear, mut animation, mut history_commands, mut sounds): (
        EventWriter<ClearCanvasCmd>,
        EventWriter<UiHistoryPlaySelectAnimation>,
        EventWriter<HistoryCommand>,
        EventWriter<ModeManagerSound>,
    ),
) {
    if esc_menu_blocks(ui.probe, ui.esc_menu_canvas_id) {
        return;
    }
    let blocked = history.is_recording();
    if let Some(request) = requested_history.read().last().copied() {
        if blocked {
            sounds.write(ModeManagerSound::SwitchBlocked);
        } else {
            *mode = AppMode::Replay;
            history_commands.write(request.0);
            let direction = request.0.bytes[1];
            assert!(direction < 2, "internal error: entered unreachable code");
            animation.write(UiHistoryPlaySelectAnimation(direction != 0));
        }
    }
    // Undo then Redo, before the Replay early return. These hotkeys do not
    // emit UiHistoryPlaySelectAnimation; only the explicit request above does.
    for (slot, command) in [(16, HistoryCommand::UNDO), (17, HistoryCommand::REDO)] {
        if actions.digital[slot].just_pressed() {
            if blocked {
                sounds.write(ModeManagerSound::SwitchBlocked);
            } else {
                *mode = AppMode::Replay;
                history_commands.write(command);
            }
        }
    }
    if *mode == AppMode::Replay {
        return;
    }
    if let Some(loaded) = loaded.read().last().copied() {
        *mode = AppMode::Replay;
        assert!(
            !matches!(loaded.restore_to, AppMode::Replay | AppMode::LoadGame),
            "internal error: entered unreachable code"
        );
        restore.0 = loaded.restore_to;
        // Original continues after the loading branch; it does not recheck Replay.
    }
    if let Some(request) = requested_modes.read().last().copied() {
        if blocked {
            sounds.write(ModeManagerSound::SwitchBlocked);
        } else {
            select_app_mode(request.mode, &mut toolbar, &mut mode);
        }
    }
    if let Some(request) = requested_categories.read().last().copied() {
        if blocked {
            sounds.write(ModeManagerSound::SwitchBlocked);
        } else {
            category_advance(request.0, &mut toolbar, &mut mode);
        }
    }
    // Native order: eraser (+48) precedes terrain (+40), then garden/tree/water.
    for (slot, category) in [
        (4, ToolbarCategory::Building),
        (5, ToolbarCategory::Path),
        (6, ToolbarCategory::Decorators),
        (7, ToolbarCategory::Customization),
        (9, ToolbarCategory::Eraser),
        (8, ToolbarCategory::Terrain),
        (10, ToolbarCategory::Garden),
        (11, ToolbarCategory::Tree),
        (12, ToolbarCategory::Water),
    ] {
        if actions.digital[slot].just_pressed() {
            if blocked {
                sounds.write(ModeManagerSound::SwitchBlocked);
            } else {
                category_advance(category, &mut toolbar, &mut mode);
            }
        }
    }
    if actions.digital[13].just_pressed() {
        if blocked {
            sounds.write(ModeManagerSound::SwitchBlocked);
        } else {
            toggle_photomode(&mut toolbar, &mut mode);
        }
    }
    if actions.digital[14].just_pressed() {
        tod.visible = !tod.visible;
        sounds.write(ModeManagerSound::TimeOfDayToggled);
    }
    // Clear uses pressed, not just_pressed (native discriminant <=2).
    if actions.digital[26].pressed() {
        if blocked {
            sounds.write(ModeManagerSound::SwitchBlocked);
        } else {
            *mode = AppMode::Editing(EditSubject::None);
            clear.write(ClearCanvasCmd(false));
        }
    }
}

/// Adds callable ECS consumers. Callers insert the real HistoryTransactions,
/// ModeManagerUiContext, TimeOfDayScreen and MainActionSet resources and maintain Bevy's event
/// lifecycle. The original global schedule ordering is not asserted here.
pub fn configure_mode_manager<M: Send + Sync + 'static, S: Send + Sync + 'static>(
    world: &mut World,
    schedule: &mut Schedule,
    demo: bool,
) {
    world.init_resource::<AppMode>();
    world.init_resource::<PreviousAppMode>();
    world.init_resource::<RestoreAppTo>();
    if !world.contains_resource::<Toolbar>() {
        world.insert_resource(Toolbar::new(demo));
    }
    world.init_resource::<Events<RequestAppModeChange>>();
    world.init_resource::<Events<RequestToolbarCategoryChange>>();
    world.init_resource::<Events<RequestHistoryCommand>>();
    world.init_resource::<Events<LoadingGameComplete>>();
    world.init_resource::<Events<HistoryCommand>>();
    world.init_resource::<Events<UiHistoryPlaySelectAnimation>>();
    world.init_resource::<Events<ModeManagerSound>>();
    world.init_resource::<Events<ClearCanvasCmd>>();
    world.init_resource::<Events<AppModeChanged>>();
    schedule.add_systems((mode_manager::<M, S>, send_mode_just_changed_event).chain());
}

#[cfg(test)]
mod recovery_tests {
    use super::*;
    use crate::input::action_set::DigitalActionState;
    use crate::recovered_building::history_transactions::ObservedHistoryLedger;
    fn fixture() -> (World, Schedule) {
        let mut world = World::new();
        world.insert_resource(HistoryTransactions::<(), ()>::from_observed(
            ObservedHistoryLedger {
                entries: vec![],
                cursor: -1,
                undo_anchor: -1,
                resolved_snapshots: vec![],
                observed_count_58: 0,
                mutation_nonce: 0,
            },
        ));
        world.insert_resource(ModeManagerUiContext {
            probe: EscMenuProbe {
                hovered: false,
                widget_id: 0,
                hit_tag: 2,
            },
            esc_menu_canvas_id: 55,
        });
        world.insert_resource(MainActionSet::default());
        world.insert_resource(TimeOfDayScreen { visible: false });
        let mut schedule = Schedule::default();
        configure_mode_manager::<(), ()>(&mut world, &mut schedule, false);
        (world, schedule)
    }
    fn events<T: Event + Clone>(world: &World) -> Vec<T> {
        let events = world.resource::<Events<T>>();
        events.get_cursor().read(events).cloned().collect()
    }
    fn press(world: &mut World, slot: usize) {
        world.resource_mut::<MainActionSet>().digital[slot] =
            DigitalActionState { disc: 0, val: 0.0 };
    }
    #[test]
    fn last_mode_request_and_full_payload_event() {
        let (mut w, mut s) = fixture();
        let target = AppMode::Editing(EditSubject::Roof(0x123456789));
        w.resource_mut::<Events<RequestAppModeChange>>()
            .send(RequestAppModeChange {
                mode: AppMode::Path,
            });
        w.resource_mut::<Events<RequestAppModeChange>>()
            .send(RequestAppModeChange { mode: target });
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), target);
        assert_eq!(w.resource::<Toolbar>().current, ToolbarCategory::Editing);
        assert_eq!(
            events::<AppModeChanged>(&w),
            [AppModeChanged {
                previous: AppMode::default(),
                current: target
            }]
        );
    }
    #[test]
    fn history_recording_blocks_modes_but_time_of_day_remains_live() {
        let (mut w, mut s) = fixture();
        w.resource_mut::<HistoryTransactions<(), ()>>()
            .start_input_recording(0)
            .unwrap();
        press(&mut w, 5);
        press(&mut w, 14);
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::default());
        assert!(w.resource::<TimeOfDayScreen>().visible);
        assert_eq!(
            events::<ModeManagerSound>(&w),
            [
                ModeManagerSound::SwitchBlocked,
                ModeManagerSound::TimeOfDayToggled
            ]
        );
    }
    #[test]
    fn undo_preempts_mode_queue_and_preserves_unread_mode_request() {
        let (mut w, mut s) = fixture();
        press(&mut w, 16);
        w.resource_mut::<Events<RequestAppModeChange>>()
            .send(RequestAppModeChange {
                mode: AppMode::Tree,
            });
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::Replay);
        assert_eq!(events::<HistoryCommand>(&w), [HistoryCommand::UNDO]);
        assert!(events::<UiHistoryPlaySelectAnimation>(&w).is_empty());
        *w.resource_mut::<AppMode>() = AppMode::default();
        *w.resource_mut::<MainActionSet>() = MainActionSet::default();
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::Tree);
    }
    #[test]
    fn esc_menu_blocks_without_draining_and_loading_keeps_its_restore_target() {
        let (mut w, mut s) = fixture();
        w.resource_mut::<ModeManagerUiContext>().probe = EscMenuProbe {
            hovered: true,
            widget_id: 55,
            hit_tag: 0,
        };
        w.resource_mut::<Events<LoadingGameComplete>>()
            .send(LoadingGameComplete {
                restore_to: AppMode::Wall(WallShape::Circle),
            });
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::default());
        w.resource_mut::<ModeManagerUiContext>().probe.hovered = false;
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::Replay);
        assert_eq!(
            w.resource::<RestoreAppTo>().0,
            AppMode::Wall(WallShape::Circle)
        );
    }
    #[test]
    fn shortcut_photomode_toggle_and_clear_pressed_order() {
        let (mut w, mut s) = fixture();
        press(&mut w, 13);
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::Photomode);
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::default());
        assert_eq!(w.resource::<Toolbar>().current, ToolbarCategory::Editing);
        press(&mut w, 4);
        w.resource_mut::<MainActionSet>().digital[26] = DigitalActionState { disc: 2, val: 99.0 };
        s.run(&mut w);
        assert_eq!(*w.resource::<AppMode>(), AppMode::default());
        assert_eq!(events::<ClearCanvasCmd>(&w), [ClearCanvasCmd(false)]);
    }
    #[test]
    fn serde_preserves_original_nested_names_and_clutter_schema() {
        let mode = AppMode::Decorator(DecoratorType::Clutter {
            ty: ClutterMeshTy::PlantPotV1,
        });
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(json, r#"{"Decorator":{"Clutter":{"ty":"plant_pot_v1"}}}"#);
        assert_eq!(serde_json::from_str::<AppMode>(&json).unwrap(), mode);
        assert_eq!(
            serde_json::to_string(&AppMode::Editing(EditSubject::DecoClutter {
                decorator_id: 7
            }))
            .unwrap(),
            r#"{"Editing":{"DecoClutter":{"decorator_id":7}}}"#
        );
        for tag in 0..=63u8 {
            if tag != 62 {
                assert_eq!(
                    DecoratorType::from_native_tag(tag).unwrap().native_tag(),
                    tag
                );
            }
        }
        assert_eq!(AppMode::Editing(EditSubject::Roof(4)).native_tag(), 2);
        assert!(!AppMode::Replay.is_tool_selected());
        assert!(AppMode::Fences.is_tool_selected());
    }
}
