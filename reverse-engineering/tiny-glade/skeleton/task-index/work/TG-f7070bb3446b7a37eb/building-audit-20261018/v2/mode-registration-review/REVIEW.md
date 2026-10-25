# Current core mode adapter: independent static review

No blocking defect was found in this root adapter. Reviewed the three actual before/current diffs (startup/run_game.rs, terrain/mod.rs, core_editing_schedule.rs), the existing mode_manager implementation, input/history phase registration and the latest full host. No production edit, Cargo invocation or runtime test was performed by this reviewer. Hash-bound inputs and the reduced dependency projection are in review-evidence.json; adapter.diff contains the exact reviewed changes.

## Resources and preservation of current design

| Mode-manager input | Provider in this core entry |
| --- | --- |
| AppMode, PreviousAppMode, RestoreAppTo | Existing configure_mode_manager init_resource; input's existing PreviousAppMode remains intact |
| Toolbar | Existing constructor with actual GameConfig.dev.demo, only when absent |
| UiState | Existing live value retained; missing value uses current UiState::at_startup constructor |
| ModeManagerUiContext | Projection from that actual UiState during initialization, then current synchronization function after input derivation and before mode |
| TimeOfDayScreen | Existing typed Default via init_resource, preserves an existing screen |
| HistoryTransactions<EditType,()> | Exactly the same type as existing History<()>; existing empty_glade/load owner is reused |
| MainActionSet | Existing register_input_events/register_history_events, no parallel state |
| Mode/request/history/sound/loaded events | Existing configure_mode_manager registration; LoadingGameComplete uses the save owner's identical reexport |

This does not manufacture an unhovered context or create a second synchronization implementation. UiState::at_startup is the existing real UI constructor with inactive interaction and empty registries; it is called only if the owner is absent. Replacing the derived context from live UI is appropriate, while preserving the owning UI avoids discarding host interaction/widgets. TimeOfDayScreen initialization uses the current established type/default and retains existing nonzero state. Current mode/toolbar/input/history algorithms, public APIs, manually refactored cursor/capture logic and event identity are untouched. No claimed mode_manager source was changed.

## Ordering and cycle audit

The actual local path is now:

`Input Derive -> terrain hover/focus producer -> mode_manager -> its late mode-event publisher -> terrain focus-mode/UI and History Cursor -> History Replay -> Terrain Edit -> Changed -> Pop -> Record -> Input Finish`.

The live UI projection independently precedes mode_manager after Input Derive. Focus requests are therefore seen by mode_manager in the current frame; focus-mode/UI waits for the resulting publication before reading the new AppMode. History Cursor waits for publication and can consume the same frame's HistoryCommand, while replay_mode retains its own after-publisher dependency. Existing replay-before-terrain-edit and changed-before-pop links remain consistent. The exact explicit-edge projection has36 edges and a valid topological ordering.

There are two current instances of send_mode_just_changed_event in this local schedule: the earlier manual input Control publisher and the later existing configure_mode_manager publisher. This is already inherent in joining the existing APIs. The mode->late-send edge produced by .chain() is instance-local. The terrain/history .after(send-function-type) dependencies wait for both publishers; the early one is already before Derive and adds no backward edge. Current code correctly avoids a global mode.before(send-function-type) edge, which would also target the Control instance and cause a cycle. This analysis binds current `<EditType,()>` joins; callers using another History snapshot type must configure the corresponding generic mode manager.

The revised core_editing_schedule test now registers the actual mode consumer before initializing the joined input/history/terrain/wall graph, which is the right additional cycle check. It does not run the business systems, exercise the new startup resource preservation, or include the startup helper's UI projection synchronization edge; no such runtime coverage is claimed here. Root's unified verification remains required.

## Full-host boundary

The latest tiny-glade/src/run_game.rs::install_history_and_wall_runtime already creates real UiState/context/toolbar and registers sync->mode<EditType,()>->send in its GameSystemSet10. It is a separate global host installer owned elsewhere. Current Rust call-site searches find no call from that host into core register_core_startup/add_game_startup_systems, nor into the terrain CPU/tool helper entry points. This patch improves the public local core scheduling path; it does not duplicate a mode manager in the full host and does not complete that host's terrain tool integration. No native global-set ordering is inferred from the local adapter.

The terrain focus producer still needs its actual optional navigation/camera/collision/cursor owners to emit a request; the new mode adapter does not fabricate those. Renderer/GPU/UI-interaction production, full-host terrain installation, original process execution and C parity remain separate boundaries.
