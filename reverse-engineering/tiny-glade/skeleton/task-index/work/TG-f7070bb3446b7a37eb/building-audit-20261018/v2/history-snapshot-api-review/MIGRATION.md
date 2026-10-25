# Minimal live History snapshot payload migration

Read-only current API/source inventory for root and the existing owners. Exact native payload/schema belongs to history_audit's frozen38 contract SHA7c162225bc960a796c77f786a22c715aeedda4908a803a114a030bd7faf98316. The agreed live S is `MaybePendingSnapshotId` (`Arc<AtomicU64>`), not `Arc<SnapshotSaveData>` and not a parallel cursor/table Resource.

## Required owner propagation

| current site | minimal necessary change | ownership boundary |
|---|---|---|
|resources/history/mod.rs:26 and snapshots.rs|Canonical alias default → MaybePendingSnapshotId, reference conversion and exact >=2 predicate; use root frozen12 candidate.|TG-3846e898cf43c29d09 main-chain-boot; root only handoff candidate|
|startup/run_game.rs:127/128|contains/insert must use that canonical S. Keep the same existing HistoryTransactions owner.|current shared core startup adapter, root coordinates|
|startup/run_game.rs:76/85/181/184/186/188|Same S for configure_mode_manager and its function-type ordering target, add_history_cpu_systems, add_terrain_tool_systems, add_wall_drawing_cpu_systems; garden changes only **second** type argument `<C,S>`.|current shared core startup adapter; no global schedule reconstruction implied|
|tiny-glade/src/run_game.rs:509/592/618|Same S in actual resource insertion, mode_manager<EditType,S>, history_process_system<S>. Line991 is a test fixture, not another production insertion.|actual host main-chain owner|
|save/load_game.rs:929|Loaded History replacement must use canonical S. Convert saved `{chain_index,snapshot_id}` into the pending-ID owner, retaining raw0/1 acceptance; do not substitute unit snapshots or silently erase retained references.|TG-7babc3de8943103ac6 main-chain-boot|
|save/load_game.rs:194/218/225/235|Current Vec<Value>/private SavedSnapshotReference are transport DTOs. Reuse the canonical reference DTO or adapt them into it once. require_no_valid_snapshot currently rejects nonempty ignore-snapshot inputs and every openable snapshot; those are explicit unresolved boundaries, not a completed typed snapshot loader.|same load owner|
|save/save_game.rs:270/470/422|Both live capture consumers must borrow the **same History ledger** for cursor/current/set_current_snapshot. Stop using/initializing SnapshotCaptureState as an independent runtime table; there is no existing History→mirror synchronization producer.|TG-941c23b174ac2b4408 oschain|
|save/save_game.rs:215 + save_thread.rs:355/505|Tag9 carries captured data **and the same pending-ID clone** stored in History. Worker completion publishes actual blob-hash ID or invalid1 to that shared owner. Current `cursor as u64` capture ID and data-only command cannot implement this. Keep current domain-data APIs where usable; do not recreate whole save engine.|save_game TG-941... and save_thread TG-cefc5b92bca0aef69a, both oschain|
|save/snapshot.rs:241/305|Existing SnapshotSaveData/capture returns data Arc and can remain the current data boundary. Native original Box versus current Arc is an explicit adaptation. The live ledger stores only pending IDs; captured domain data is not copied into History.|TG-3adf041a3108edb59a main-chain-boot|
|save/save_game.rs:243/765|SaveGameSyncData/GameSaveSections are serialized transport/cache. Their chain_index/history_start/snapshots projections must come from this actual ledger, with resolved_snapshot_references filtering pending/failed. The cache is not a second mutable History authority.|save owner/host collection owner|

`register_core_startup<B,S>` uses S for **audio Studio**, not history payload. Do not replace that generic parameter or audio registrations. Generic recorder/replay APIs already take `History<S>` and forward S; source algorithms need no new parallel implementations. Water/garden helpers are `<C,S>`, decorator helper is `<S,C>`, and tree/color/UI-clutter helpers take history S alone. Only the actual caller monomorphizations need to agree.

Bare default consumers automatically follow the canonical alias: paths::record_path_history::RasterHistoryRecordResources, roof_record_history, record_switch_roof_ridge_history, decorator culling and external command handlers. This makes the canonical alias change necessary; retaining a live History<()> beside it would split ECS identity. Reusable HistoryTransactions<M,S=()> and RimHistoryTransactions need not be rewritten to carry game snapshot state; keep generic fixtures explicit.

## API reuse and boundaries

Existing generic functions nearest_snapshot_index/current_snapshot/set_current_snapshot/closest_resolved_snapshot operate on `ResolvedHistorySnapshot<S>` and can stay. The exact predicate correction and MaybePending impl are in root12. `ledger()/ledger_mut()` already expose cursor/table for a narrow canonical wrapper. `nuke_future_states` already clears/truncates that same table, so a mirror would miss normal undo/branch cleanup.

There is **no current runtime ECS `HistorySnapshots` Resource**. The current symbol of that name is a private DeserializeSeed in snapshot_gc.rs; utils::SaveHistorySnapshots is a selective JSON-path alias. They are not suitable live owners and need no broad rewrite. snapshot_gc::SaveFileSnapshot remains a saved-reference DTO. tg-history::HistoryDocument is a separate file/model utility, not the live Bevy resource; core's current use is roof typed enums, not a second snapshot authority.

No ownership claim was taken. The active source group snapshot is in api-propagation-inventory.json. Current split transactions.rs/snapshots.rs have no independent original source-file group in the B index; they belong with the canonical History family, not unclaimed new parallel modules. No new original global scheduling order is inferred from the API audit; adding ResMut<History> to save requires its current owner to preserve/verify real ordering rather than evade data dependencies through a mirror.

## Test propagation (important runtime identity risk)

After changing the alias default, `History::<()>::empty_glade()` and `world.resource::<History>()` are **different valid Resource types**. This can compile and then fail at runtime. Inventory lists every explicit unit insertion/query and bare default query, plus mixed fixture files; classify rows as source inventory, not a Rust AST proof. Pure transaction/recorder fixtures may remain unit if all their calls/queries explicitly use that same unit S. Integration fixtures should use canonical S consistently, including mode function-type ordering targets. Do not solve failures by inserting both History types or by restoring the wrong default/predicate.

Snapshot field-state tests may change only with original native evidence:0pending/1invalid/2andMAXresolved. Missing required serde fields still fail, but raw saved0/1 must deserialize and remain filtered after conversion. The root candidate now tests those boundaries. Original snapshot file/chunk/domain restoration and async physical write/hash completion remain separate owner work.
