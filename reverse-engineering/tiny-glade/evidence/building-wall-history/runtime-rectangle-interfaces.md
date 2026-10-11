# Parallel rectangle/pending/decorator runtime interfaces — UNVERIFIED

This patch has not been built or tested. Its existing core algorithms have separate earlier B native evidence; that evidence does not accept this new World wiring.

## Parent registration

Declare runtime modules `wall_rectangle_edit`, `wall_pending_changed`, `wall_decorator_archivist`; call each module's public `register(app)` once before `wall_authoring::register(app)`. The new modules initialize resources and events, and the existing Authoring chain calls their consumers. Parent retains shared `lib.rs`, stage graph, Nani and CLI ownership.

## Actual rectangle finish owner

Write `ObserveRectangleEditUi([u8;80])` with the original observed state1/dimensions or state2/orientation payload. The existing `EditAuthoredRectangle([u8;64])` updates actual Raw Source first. Write `FinishRectangleEdit` only for an actual new finish action; Bevy owns unread-event tracking. `WallRectangleEditState.ui` starts as `None`, so unknown UI is not treated as a fabricated inactive/default state.

`consume_and_finish` reads actual staged or collected Raw Source by observed WallId, excludes sources removed in the same adapter frame, and invokes the recovered finish decision. It emits active UI80 before finish processing. A changed edit creates the actual EnclosureChange owner using UI previous shape and **actual Source next shape**, sends its two actual notification payload types and clears UI. A nochange finish executes real decorator storage restoration before clearing UI; missing archive/storage or a typed core error retains the original UI and pending finish. A pending retry is bound to the captured UI state; newer UI observations queue until that retry commits, preventing a later wall from inheriting an older finish action. This retry mechanism is an explicit adapter guard.

Only active recording frames enqueue the actual owner in `BuildingHistoryRequests`. Otherwise the real owner is retained in `unrecorded_history` for inspection and is never admitted by a later Start. Enclosure command playback is handled by the parent's existing unified Wall history consumer; this patch does not introduce another ledger.

## Actual nonempty decorator cancellation

Write `ObserveDecoratorStorage(Arc<DecoratorStorage>)` with real three-tracking containers, global live-ID index and saved removed-by-address state. Write `ObserveDecoratorArchive(Arc<ObservedDecoratorArchive>)` with actual wall/special archive contents. Both resource options start as `None`; absence is distinct from an explicitly observed empty archive. Collision entries retain real dependency-owned resource references as `Arc<dyn Any+Send+Sync>` and are released by the recovered clear; collider handle creation/renderer ownership is not fabricated here.

The recovered core mutates removed/live/added containers, global ID→StorageAddress, saved removed IDs and actual archive storage. All source/archive options drain on successful restoration. Missing input or invalid-index/niche errors preserve the pending rectangle finish. `restored_this_frame` and `restored_decorators_this_frame` are inspection state, not substitutes for data restoration.

Info72 remains opaque. To connect a known wall attachment, write `ObserveDecoratorOpeningBinding { decorator_id:u32, observed_info:[u8;72], opening:PlaceWallOpening }` from the actual original placement/opening request owner. This is an explicit identity binding between an observed native record and an already resolved CPU opening request. Raw bytes identify the supplied adapter binding; they do not define original game equality or guessed Info fields. Multiple observed payload versions can be retained for the same ID, so cancelling to an older archived record can recover its original request.

For every restored/live wall-owned record with a matching real binding and matching wall address, the bridge emits actual `PlaceWallOpening`. Existing Placement resolves the shipped asset against fresh Raw/Prepared/Public/Roof state, then real Openings/Holes/Bricks consume the resulting geometry. Known old opening IDs no longer live emit `RemoveOpening`, reaching entity/intent/hole cleanup. IDs retained by the new restored version are never removed in the same frame, because Openings consumes upserts before removal. The bridge remembers known pre-cancel links even when storage seed and finish arrive in the same adapter frame. Missing Info binding or unresolved special storage produces `pending_projection` and removes only known stale links; a later actual binding republishes the record. No field offset or ID cast is invented.

## Actual deferred free-wall history owner

Write `ApplyObservedFreeWallChange { wall, next:WallAuthoritySnapshot, extend_start }` with the original producer's resolved next Source and metadata. The current previous Raw Source and metadata are read from real authoring/collection state, never from defaults. The original projection/strict range rule constructs one `PendingWallChanged` slot. All valid next Sources, including actual None-history and equal-snapshot cases, publish through `UpsertRawWall`, update staged metadata/dirty state and therefore reach source publication, cache, placements, holes and wall production.

After all requests the actual `take_pending_wall_changed` clears the original pending slot and yields one real StateChange concurrent group. Active history records it through existing `BuildingHistoryRequests`; inactive observations are retained without later backfill. Multiple same-frame requests overwrite the single pending slot; this follows the recovered deferred-owner cardinality and does not manufacture a per-event ledger. Upstream freehand UI selection/deformation and original cross-system schedule remain explicit caller boundaries.

## Shared merge checklist

See `tracking/parallel-implementation-conflicts/rectangle-history.md`. No `history.rs` edits are required. Parent owns final registration, compilation, tests and independent review. These new runtime files and authoring tuple changes must remain labeled unverified until that stage completes.


## Native Info field binding extension (unverified)

Parent registers `decorator_info_binding::register` and the `building_decorator_info` core alias. `ObserveNativeDecoratorOpeningId {decorator_id,opening_id}` supplies only identity; geometry/rank/family/attachment sidecar is no longer required for supported native wall records. The consumer reads real tracked Info72 and live storage address, uses actual wall coordinate bits, derives subtype with platform!=0/corner!=0, and sends `UpsertOpening` directly from the actual selected asset. Its system runs after Placement/before Openings. Both the old sidecar publisher and parent legacy placement resolver opt out for IDs owned by this map. `ForgetNativeDecoratorOpeningId` releases the known derived opening, including while storage is temporarily unavailable.

Do not interpret platform as host WallId or is_bottom_door. Native field/producer evidence and remaining unknown fields are in `evidence/building-decorator-info/findings.md`. This extension is not compiled, tested or accepted.
