# Archival owner candidate — central review required

This package is a new implementation-owner assignment after the separate
independent verification report. It does **not** count as independently reviewed
or executed. No production source or Cargo target was written. Candidate source
passed rustfmt parsing with `skip_children=true` and a direct rustc metadata
check against cached country-core453e83dd454432af.rmeta and its fingerprint
dependencies (precheck.json/log). No Cargo/test execution was run.

`candidate.json` records both exact base/proposed hashes and whether production
is still at that base. `archival-owner.patch` is the two-file unified diff.
Other team owners confirmed no overlapping edits to these two files.

## Original evidence and corrected contracts

The direct original extracts and hashes remain under
`../independent-verification/original-evidence.json`:

-Roof do_ui141a19a90 at141a1a0bb calls WallId→DecoratorStorageAddr;141a1a0f4
  calls archive1421a0470/9894B with one address, one live storage, actual archival
  resources and the reason `roof_edit`. Focused roof editing calls the same owner.
-Archive1421a04c8 calls get(address). Already archived walls/special buckets
  return without replacing before-state. Source-present paths clone that single
  bucket; missing-source cold path0622..0c4c creates and archives an empty before
  bucket. It does not skip a missing wall or archive unrelated walls/terrain.
-The original's two collision-channel loops look up each current decorator ID,
  skip absent/sentinel handles, read the actual live collider transform, retrieve
  its owned RaycastMesh and increment its Arc before inserting the pair into the
  corresponding archive collision map. Stale physical/mesh keys panic. The before
  bucket has already been installed at that point; failure is not transactional.
-Cull140b2bc00/292B mutates allowed_ids atself+30 by extending from the selected
  **live** wall bucket;140b76260/154B has a unit fold accumulator and calls
  HashSet insert1402c9200. It is not an `any(is_allowed)` query. Stairs140b2bd30
  is the equivalent live stair-bucket extension. The independent query remains
  is_decorator_cull_allowed140b2bd90.
-Original get_collider_mesh1408cdbd0/66B checks the SlotMap index/version and
  returns RAX pointing to its RaycastMesh. The previous source's unit return
  discarded this essential owner. Archive reads its fields and clones its Arc.
-Original upsert entry140b40b50/3447B calls141558ea0, whose PDB identity is
  SlotMap<RaycastMeshKey,RaycastMesh>::insert. It writes that actual key to
  ColliderBuilder::user_data. The physical collider handle and mesh registry key
  are distinct. Each Rapier subworld can allocate collider index0 independently.

## Candidate changes

Only `resources/walls/decorator_archivist.rs` and `systems/collision/world.rs`
change. The candidate reuses the current `MeshInstanceSources`/slotmap
owner and actual Rapier colliders; no second collision world, std/slotmap clone,
rhapsody implementation or source-pointer reconstruction is introduced.

-`DecoratorArchivalResources<'w>` derives SystemParam and borrows the canonical
  RaycastWorld and DecoratorPhysicsColliders resources. The roof UI API is
  `archive_current_decorators_at_address(address, storage, &resources, reason)`.
-The old misleading whole-world archive wrapper is removed. Plural archival
  accepts addresses into the same live storage/resource pair. Pure tracking
  helper `archive_storage_decorators` remains for existing CPU callers; those
  callers are explicitly **not** full collision archival acceptance.
-The complete native-physics address method preserves source/absent/already
  archived branches and both channels, including partial writes before panic.
-Cull wall/stairs now mutate allowed_ids using live storage keys; they return().
-Mesh upsert retains the actual RaycastMesh in existing SlotMap, writes its raw
  key to user_data and mirrors the compatibility header with that key. Getter
  returns a live borrowed RaycastMesh, with the dependency's exact generation
  validation. Disposal removes the same key, releasing that owner once.
-Physical instance.handle() now derives the Rapier index/generation. Ray-result
  registry lookup reads the mesh key from the actual collider's user_data.

The original lock has slotmap1.0.6; current user code has1.1.1. Both local public
sources implement `as_ffi=(version<<32)|idx` and `from_ffi` forces `version|1`
(`slotmap-ffi.json` records excerpts and source hashes). The getter first compares
the exact compatibility-header generation, so an invalid even generation cannot
be normalized into a valid odd key. No dependency downgrade is proposed.

The original missing-storage reason creates a temporary formatted diagnostic
String which is dropped locally; it does not persist as an archive field. This
candidate keeps the API's reason parameter but does not manufacture diagnostic
text or claim an exact allocation/formatting trace. Check whether preserving that
non-persisted diagnostic work is required before integration.

## Regression gates prepared for the central owner

Run current country-core with native-physics, selecting these tests explicitly:

1.address_archive_keeps_both_real_collision_channels_and_meshes_after_live_deletion:
  two nonempty selected-wall colliders across both channels, unrelated wall and
  terrain retained outside the archive, absent collider ID skipped, actual pose
  checked, first before-state stable, absent wall archived empty, Arc survives
  live deletion and is released on archive drop.
2.invalid_actual_collider_generation_panics_after_the_before_state_is_archived:
  real physical collider generation invalidation, canonical before-state side
  effect retained, no invented collision pair accepted.
3.owned_mesh_keys_are_global_across_subworlds_and_reject_removed_generations:
  two physical index0 colliders in different subworlds map to distinct real
  meshes, user_data key roundtrip, removed key rejected, reused index with new
  version rejects old version and preserves the other subworld's owner.
4.decorator_cull_allow_wall_and_stairs_fold_buckets:
  wall and stair calls extend the set independently from live buckets; a missing
  wall preserves it. The old tests asserting the incorrect query semantics were
  corrected, not used as original behavioral evidence.

The existing disposal test now indexes the registry by its actual mesh key
instead of assuming it equals the local physical collider handle.

## Remaining boundaries — do not call the whole collision domain complete

-The inherited `RaycastGeoSource` still lacks original payloads for some source
  variants. This package does not invent them or claim full source serialization.
-The inherited upsert `Kept` policy currently omits the original source-equality
  predicate. This package restores mesh ownership and identity, not that larger
  already-open comparison contract. It must be resolved before full equivalence.
-The compatibility registry still has legacy raw source-header consumers and a
  pre-existing missing-registry result fallback. Fresh proof is needed for all
  source payloads and intersections; no acceptance is raised for them here.
-Old wall UI callers use the CPU tracking helper without full archival resources;
  roof UI must use the complete new interface, and those other callers remain open.
-The resource pair is a current design adapter based on the actual owner accesses;
  stripped PDB TPI has no recoverable DecoratorArchivalResources field schema.
  Confirm producer/registration identity in central original review.

T13 remains excluded. Next central gate is original-byte independent review,
followed by compilation/native-physics regression and actual roof edit→history
commit/cancel→undo/redo across multiple walls. These tests were prepared, not run.
