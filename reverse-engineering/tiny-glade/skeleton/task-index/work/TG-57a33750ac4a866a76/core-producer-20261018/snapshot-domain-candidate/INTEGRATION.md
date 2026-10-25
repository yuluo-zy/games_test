# Owner handoff

Do not replace production with an old frozen file. Snapshot family is still claimed by main-chain-boot; apply candidate.patch incrementally against the current base as that owner or have that owner apply it. Root may use the minimal shared integration-owner.patch for the StairsSnapshot read-only checksum accessor and existing motomoto dependency. No production or Cargo has been modified here.

Required peers: canonical DecoratorStorage snapshot69 (true IDs and buckets), canonical Roof139 and its narrow ShapeParameters rotation owner hunk, current canonical WallState serde. These are existing owners/types, not new domain implementations. The exact manifest identities and original boundary evidence are in CONTRACT.md.

Actual save/thread API:

```rust
let domain = SnapshotDomainData::collect(
    actual_trees, actual_walls, actual_deleted_auto_clutter_ids,
    Some(take_stairs_snapshot(&actual_accurate_stairs)),
    actual_decorator_storage.take_snapshot().states,
    (actual_wall_colors, actual_gate_colors, actual_halftimber_colors),
    actual_terrain_editing.clone(), actual_named_rasters, actual_roofs,
).with_terrain_and_supports(
    actual_raw_height_buffer_bytes, actual_terrain_height_range,
    actual_removed_supports.clone(),
);
let bytes = encode_game_snapshot(&domain)?; // one Motomoto encoder
// The existing owner writes bytes and derives the real blob hash. Only after
// success does it assign the same pending snapshot-ID owner held by History.
```

The argument names above mean actual resource captures. They are not instructions to build a Default world or to substitute empty values for missing resources. Actual empty captures are allowed; absence of raw terrain/range/removed supports is an explicit error. The old collector signature's `stairs: None` now panics instead of creating an empty accurate snapshot. Existing unbacked stairs must follow the original accuracy invariant, not be silently omitted.

`SnapshotSaveData::encode()` is a convenience delegate. `to_json()` and `try_to_json()` are metadata projections only and must not be written directly as the complete file. Current save_thread's JSON-text write is owned by oschain and must be switched to encode's Result by that owner. No second chunk/JSON writer belongs in save_thread. Arc SnapshotSaveData remains the current owned interface adaptation; original payload ownership is not an excuse to replace the current architecture.

Do not wire this writer while leaving the actual producer's typed vectors as the old empty placeholders. The current collect_snapshot_domain still needs live colors, decorators, trees, rasters, DeletedAutoClutter, removed supports, true terrain bytes/range and exact stairs data. Save/input production remains its actual owner's task. The existing main-chain COORDINATION is the permitted cross-chat handoff, with no message tool sent to that chat.

Root should run core library tests plus the actual save/worker integration suites, including a new nonempty captured world -> Result encoder -> worker file bytes/hash -> resolved History reference test. The five local source tests are not a substitute for that integration. Neither GPU-to-CPU capture synchronization nor whole snapshot loading is claimed by this package.

The legacy JSON-only load path remains incomplete. Proper Motomoto readback, full typed State/Roof/Wall restoration, actual decorator ID generator reserve and all domain consumers need their existing owner. Never use SnapshotPrefix native pointer descriptors as a disk header. This encoder emits semantic JSON and raw owned byte payloads through the existing format crate.
