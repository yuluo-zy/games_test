# Root integration requirements

Apply `continuation-candidate.patch` after ordinary-wall v1 and roundf-v2. Preserve root's color command constructor changes; the draw production algorithm is not altered by this patch. Add `pub mod deferred_wall_history` at the existing wall module root and init the one `PendingWallHistory` resource.

Integrate history_audit's separately owned Delete.restored_height schema/serde/replay increment first. Required constructor API: `RecordWallHistory::deleted_wall_with_height(wall, &previous, current_color, desired_height)`, preserving the existing three-argument wrapper for unrelated producers. Do not substitute source.extra for this independent field.

The existing finish system now requires mutable ContinueWallOriginalState, PendingWallHistory, WallColorIds and HalftimberColorIds. Retain existing live resources, install missing event owners as before, and register `history_record_wall_changed` after finish and before `record_wall_history::<()>`/input-history completion. No default source snapshot is an acceptable replacement for a missing continuation.

Install `suppress_continue_wall` in the existing input/wall stage using the current Modifiers, GameTime and CursorMovedScreenSpace stream. It must run before the eventual endpoint UI consumer so suppression uses the same frame's real modifier and movement input. It is also the actual reset consumer for the draw UI's already installed suppression write. This does not assert a numeric original set without schedule evidence.

Run root's standard checks, the new finish continuation tests and suppression World test, and the existing draw tests whose required resource list changes. Tests were authored but not executed by this worker. An eventual endpoint UI integration test must prove actual registered spatial widget→begin snapshot→continued draw→false finish→deferred history→save/replay; manually supplying a previous snapshot only verifies the consumer interval covered here.
