# Restore and diff candidate v2

Production remains unchanged. Apply both archival-owner.patch (two core files)
and decorator-restore.patch (one system file) only after central original review.
The manifests record exact base/proposed hashes. The restore system must be
registered as the concrete eight-parameter system in set42; remove the old
`::<C>` at its existing registration, and preserve the central registration work.
No Cargo or v2 codegen/test run was performed. rustfmt parsing passed. The earlier
two-file v1 passed direct rustc metadata against cached canonical dependency
metadata, but that result does not validate the added v2 code.

## Observed original contract

- PDB builder140322d40 names the exact8 parameters: AppMode, InnerWalls,
  DecoratorArchivist, DecoratorStorage, DecoratorPhysicsColliders, RaycastWorld,
  StairsState, ModeJustChanged EventReader. It is not a History cursor system.
- Original140817a40/4991B marks storage mutable at140817a9f, before the fast
  path. It consumes/clears the event reader. No-event plus AppMode tag9,
  payload1 (Eraser(Wall)) skips bucket/collision restoration. A new event forces it.
- all_storages -> get_mut ->141257900 clears whole live buckets. This clears
  modified, extends removed with current IDs, takes the item table, and removes
  d2s entries until the native niche2 sentinel. Current old helper only cleared
  map/modified, leaving d2s and removed wrong. Partial writes before panic matter.
- Original140817e78 calls InnerWalls::get for an archived wall and skips a deleted
  wall. Terrain/stair addresses bypass that lookup. Current old system ignored it.
- Each archived item inserts into the live map. The checked d2s insert/update
  choice depends on the HashMap::insert returned Option niche2, not incoming
  DecoratorInfo.state. The old helper confused these distinct values.
- Per bucket, all channelA entries then all channelB entries clone actual archived
  transform + RaycastMesh and call141298110 upsert. Candidate uses the existing
  world/mesh owner; the archived shape is passed separately and kept exactly.
- Stairs restore_backup1408ff210 always runs, even on the fast path. On an event,
  extract_diff140b2cb30 receives storage and bool1. Returned table is immediately
  dropped by140b743e0; that call is not a history record/event flush.

## Recovered diff algorithm

The earlier no-storage `extract_diff` cannot provide the original behavior.
The new API `extract_diff_against_storage(storage, allow_preview)` follows:

1. Clear both collision archives, take walls/terrain/stair before buckets in that
   order; convert each bucket's items to its own before map.
2. Read after items from the same address in live storage. Missing after is empty.
   Source fold140b2aa50 passes strict_missing_before=false and its caller bool as
   allow_preview. When false, any live after state!=0 panics with the original
   preview diagnostic. This validation is on live after, not archived before.
3. In1415237c0, compare each matching ID's destination active payload and every
   common state field; remove equal IDs from both maps. Preserve float ordered
   equality (NaN differs, signed zeros equal); ignore inactive destination bytes.
   The original4-way destination table142c6c070 targets de3/aa9/aa9/e98 and all
   branches join the common comparisons at141523ae8.
4. Filter the pair if both maps are empty, clear allowed_ids/allow_any and assert
   archives are drained. The source clears culling before the final pruning table;
   candidate clears it after non-panicking comparison/pruning. Allocation-failure
   timing is not claimed as an equivalence check.

Original AddrHistoryState is one48B map (pair96B); the current typed model's
additional removed HashSet has no native producer here. New API leaves it empty;
old callers and serialization layout require a separate central audit.
The legacy no-storage API is deliberately not used by the restored system and
still must be migrated for its other consumers. It is not a completion gate.

## Regression gates prepared, not executed

- mode_event_overrides_wall_eraser_once_and_restores_stairs_on_both_paths:
  persistent EventReader over3 runs, mutable reset, fast path retains old live
  item/archival state, event clears/removes index and restores real item, stairs
  reverts backup, new archive is retained after event has been consumed.
- vanished_wall_is_cleared_but_not_reinserted_and_channels_restore_exact_owners:
  deleted wall plus nonempty terrain, both real Rapier channels with actual
  shared RaycastMesh Arc and pose, deleted wall is not resurrected.
- diff_uses_each_live_address_prunes_unchanged_and_resets_culling:
  independent nonempty wall/terrain, deletion/addition, unchanged item removal,
  no cross-address overwrite, culling reset.
- diff_preview_validation_runs_on_live_after_and_flag_allows_it and
  history_equality_keeps_nan_and_respects_active_destination_and_common_fields.
- Earlier archival-owner actual multiwall/two-channel Arc ownership and invalid
  generation tests remain required. Independent root/roof author must review the
  original bytes and run these; author self-tests are not independent acceptance.

## Open limits

The original RaycastGeoSource has payloads omitted by current unit variants.
Inherited upsert Kept lacks original source equality. No pointer/tag substitute
is introduced here; original full source identity remains open. The archived
upsert adapter preserves shape and quaternion without renormalization, but the
whole collision domain cannot be accepted until source identity is recovered.
Missing-storage archival creates a temporary formatted reason diagnostic in the
original; exact text/allocation trace remains a separate review point. Original
roof reason is `edit_roof` (roof owner corrected the earlier transposed label).

Evidence: restore-original.asm, restore-registration.json, restore-callees.json,
extract-diff-original.json and its7 bounded original assembly extracts. Retained
decompiler text is a reading aid; its stale header identities are not evidence.
