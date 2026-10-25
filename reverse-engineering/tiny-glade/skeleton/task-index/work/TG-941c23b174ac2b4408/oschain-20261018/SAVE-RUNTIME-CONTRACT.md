# Original B save runtime candidate

Scope: existing save_game/save_thread owners and their current callers. Parent integrates; no production change, Cargo, GPU or original executable execution was performed here. `runtime-ready-manifest.json` records current source and exact candidate/patch hashes. Earlier `api-phase*` is superseded by this runtime batch.

## Main thread

Full B `save_game@140a3f470`, 17300 bytes, and all 147 direct-call identities are retained under TG-40ff43c62ef8791c53/oschain-20261018/host-edit-save-audit. Full generic system parameter identity is in helper-manifest.json (141750ab0). Original backend registration is set43 with Option<Res<ActiveSession>> presence condition. It does not require a fabricated AppMode gate.

The candidate's two SystemParam projections borrow the actual canonical World owners. First drain worker errors; publish the actual committed History suffix using observed_count_58 and advance the saved observations only after successful send. Then capture camera cache and real live sheep rows. Snapshot readiness requires an empty replay queue, unchanged actual replay pop-attempt counter, empty wall bulldoze queue, and no History recording. Update the local observed counter even when replay is busy. Only threshold<60 accumulates game time; backup consideration additionally requires stable readiness. Both SaveGameCmd and ShutdownGameCmd readers consume all unread events without short-circuiting one reader.

A stable request or backup with no same-index History snapshot captures the actual walls, roofs, trees, decorator buckets, three color tables, terrain strokes and current CPU terrain bytes/range, six current raster composites, deleted autoclutter IDs, accurate stair snapshot and removed stair supports. The original stair backup assertion is preserved. No optional-domain defaults or second snapshot ledger remain. The complete `SnapshotDomainData::collect(...).with_terrain_and_supports(...)` and `SnapshotSaveData::encode()` are the foreign domain owner's reviewed APIs; this batch does not duplicate that encoder.

The current CPU terrain owner stores u16 cells in the same 0x80000-byte native allocation whose original constructor uses u32 packed words (1409457c0). `bytemuck::cast_slice(actual.heights())` copies those actual bytes without converting heights or inventing a grid/range. `previous_position` is runtime-only and skipped by TreeData serialization; current placement is retained there.

The same new MaybePendingSnapshotId Arc owner is queued with tag9 and inserted into the canonical History ledger. Changed sections are queued in native order: Identity, Camera, TOD, Sheep, Colors, Snapshots, chain_index, history_start. Baseline comparison state is installed on first invocation; no per-frame OpenFile. Snapshot references are serialized on the worker after preceding capture commands have assigned IDs. The existing SaveGameData codec remains the sole history JSON writer.

Manual save/close writes current SessionMeta image override, logs meta errors but still attempts the real bounded-channel tag10 synchronous flush. A worker Ok emits SavingGameComplete with the actual session folder path; Err logs without reporting successful completion.

## Worker

Full `save_thread@141720680` 5139 bytes is retained in TG-cefc5b92bca0aef69a/oschain-20261018 with worker-deferred-native-manifest.json. The alleged Shutdown tag11 was the outer TryRecvError niche. Real Empty applies only due deferred slots and sleeps10ms; Disconnected flushes and exits. Current crossbeam ownership is preserved rather than mimicking original mpsc ABI.

`execute_save_cmd@141721aa0` 38878 bytes (same TG40 evidence): tag5 is ReplaceEdits; tag6 is ChangeSection. ChangeSection owns a deferred serializer trait object and calls its method on the worker (14172311a..12f). Camera/TOD/Sheep alone have120s deadlines, from constant1429cd0f8 and Duration::from_secs_f32@14171eb50; replacements preserve the first deadline. Other sections apply immediately. Explicit Flush and close/disconnection force every deferred slot. `flush_deferred_and_sync@14171f7f0` 1934 bytes ends in actual ShadowFile::sync_all@14171fc9f; the reply follows that result. Flush without an active save panics as in B.

Tag9 calls the unique Result encoder. Original `utils::calculate_hash<&[u8]>@14145b1a0` is full385 bytes: zero-key SipHash13, hash usize length then the complete blob. Current std DefaultHasher supplies that dependency implementation. Actual same-byte helper PDB, raw SHA and full asm are in snapshot-file-hash-native-manifest.json. Existing `<hash>.snapshot` files are reused (1417241b7). Otherwise recursively create snapshots folder, NamedTempFile::new_in@14096b1c0, write_all@14096b030, persist@14096b270. There is no added snapshot fsync call. Only then success141726d78 assigns the hash to the same pending owner; failure141725b6c assigns invalid1. No-active tag9 jumps panic1417271d4.

## Dependencies and remaining acceptance

Root must integrate snapshot61+testv2+WallIdv3, canonical DecoratorStorage69 true bucket state and Roof139 serializer before compiling this batch. The save/mod delta removes the fictional extra snapshot system and preserves real EventRegistry maintenance needed by input; apply it incrementally to current source. Host registration remains parent's numeric set43 integration responsibility.

The complete chunk loader/domain reapplication and nonempty MultiEdit auxiliary decorator/stair encoding are separate foreign-owner dependencies still being recovered. A successful writer test is not a claim that a modified whole game can already reload. New tests listed in runtime-ready-manifest.json exercise actual files/channels/shared pending IDs and are not yet executed; parent runs Cargo and end-to-end validation.
