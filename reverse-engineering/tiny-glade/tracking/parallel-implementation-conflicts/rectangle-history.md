# Rectangle/history parallel integration — UNVERIFIED

Owned writes: runtime `wall_rectangle_edit.rs`, `wall_pending_changed.rs`, `wall_decorator_archivist.rs`, and the Authoring system tuple in `wall_authoring.rs`. No edits to shared `history.rs`, runtime `lib.rs`, CLI or Nani. No Cargo/build/tests/oracle runs in this parallel implementation phase.

Parent needs to declare all three runtime modules in `lib.rs` and call each `register(app)` before `wall_authoring::register`. Core aliases `building_wall_rectangle_edit`, `building_wall_pending_changed`, `building_wall_decorator_archivist` are parent-owned and were reported present. New `register` functions initialize resources/events only. The existing Authoring tuple invokes their consumers; do not additionally schedule the same consumers in another set.

The tuple is now: creation/removal → real rectangle edit → observe native decorator storage/archive/bindings → rectangle finish/cancel → publish restored opening requests/removals → observed free-wall change/deferred history → actual bulldoze → metadata cleanup. This is explicit CPU adapter scheduling, not recovery of the original fork's complete order. Parent may relocate these calls while keeping actual Source/staged writes before collect and actual placement before Placement/Openings.

Potential shared-file merge point: `wall_authoring.rs::register` Authoring tuple and its module comment. Parent should preserve these four added consumer calls alongside any Nani/CLI stage additions. Other agents must not install a second same-stage observer/deferred recorder. History uses existing `BuildingHistoryRequests` and requires no edits to `history.rs` in this patch.

Untyped fields/owners remain gaps: Info72 internal placement layout; special storage1/2 downstream renderer owner; actual UI archive producer; freehand UI next-source/range selection; collision-handle creation. Inputs for these are explicit observed DTOs/resources, not constants or guesses. The runtime never casts DecoratorId into OpeningId. Unknown restored Info removes only a known stale opening link and stays in `pending_projection` until an actual payload→opening binding arrives. Native storage/history restoration is not replaced by that diagnostic.

Current APIs and limits are documented in `evidence/building-wall-history/runtime-rectangle-interfaces.md`. All runtime integration in this patch is intentionally **unbuilt/unverified** pending the final parallel merge stage.
