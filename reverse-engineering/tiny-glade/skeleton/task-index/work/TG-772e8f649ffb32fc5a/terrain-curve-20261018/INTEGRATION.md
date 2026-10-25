# Integration handoff (root only)

Apply `candidate.patch` from the workspace root after checking `frozen-inputs.json`. It contains six new source files plus a two-line metadata field on the current `InProgressCurve`; no existing wall/archive/history implementation is replaced. Exact destination/source SHA values and production/test line counts are in `source-map.json`. Keep frozen work inputs unchanged; record any compiler adaptation in a separate integration patch.

Add module exports at the existing shared owners:

* `resources/mod.rs`: `pub mod drawing_input_smoother;`
* `resources/walls/mod.rs`: `pub mod in_progress_wall;`
* `systems/wall/mod.rs`: `pub mod draw_wall; pub mod ui_draw_wall; pub mod finish_wall; pub mod ui_continue_wall;`

Keep one event identity. If the existing events compatibility module needs original paths, re-export `systems::wall::draw_wall::RecordMouseInputIntoWall` and `systems::wall::finish_wall::FinishWall`; do not declare a second event struct. Keep the existing `InProgressCurve` resource as sole active-curve owner (`InProgressWallResource` is an alias).

Register new resources `CurveNursery`, `WallDrawState`, `WallDrawingIssues`, `ContinueWallOriginalState` and `SuppressContinueWall`, retaining already inserted values. Add `Events<RecordMouseInputIntoWall>` and `Events<FinishWall>`. Reuse existing resources/events for InnerWalls, InProgressCurve, snapping, real cursor raycast, GameTime, MainCamera, WindowSize, GameConfig, GladeColorCustomization, UiState, real GladeBorder, LiveDecoratorArchivist and DecoratorStorage; no synthetic live camera/border successful fallback is part of this contract.

The new systems also require the existing events for wall/halftimber color, wall height, material audio, bend reset, tree hiding, out-of-border/feedback, debug history, record-wall history and tool-used onboarding. Shared APIs agreed with their owner are `RequestWallMaterialAudio {id:u64,position:Vec3,kind:u8,flag:u8}` and `HideTreesNextToWallCmd {wall:WallId,finished:bool}`. `WallDrawingIssues` is retained diagnostic state, not a success channel.

The actual CPU closure requires UI input before `draw_wall`, drawing before `finish_wall`, and finish before the already existing `record_wall_history::<()>`; recording must precede input-history completion. The regression uses this explicit chain. Choose the existing host phase/set integration at the shared owner; this worker does not assert that one guessed numeric set recovers the entire original scheduler. Avoid duplicate installation of existing record/color/height/publish consumers.

Run root's standard country-core compilation/tests, in particular the new original-entry UI/smoother fixture tests, ordinary-finish gate test and four draw World regressions. Candidate source has not been executed. A root integration test should use actual host resources for the wall/fence UI path and existing history lifecycle, then inspect the resulting authority/history, rather than just asserting module names or registration strings.

The supported fresh path deliberately rejects snap/continuation/cancel via retained issues until their actual original producers/topology are restored. Do not mark that boundary complete merely because ordinary drawing compiles. Likewise an event sent to material audio/tree hiding does not establish their whole upper consumer or renderer lifecycle; those are separately owned tasks.
