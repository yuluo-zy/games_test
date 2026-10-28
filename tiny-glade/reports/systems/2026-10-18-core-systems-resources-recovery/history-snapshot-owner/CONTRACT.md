# History snapshot pending-ID owner candidate (not production)

Original source family TG-3846e898cf43c29d09 belongs to main-chain-boot-20261018. This is a minimal existing-module handoff, not a second History/resource engine. Native evidence freeze and independent review pending. Do not apply as accepted before these complete.

Canonical API in resources::history, reexported from snapshots:
- HistorySnapshotId(pub u64), serde transparent, no invented >=2 deserialize validation.
- MaybePendingSnapshotId::new()/default(): same Arc<AtomicU64> initialized0. Clone shares writer completion; assign_invalid() writes1; assign(HistorySnapshotId) writes raw ID. snapshot_id()->Option<HistorySnapshotId> only >=2.
- HistoryChainSnapshot = existing ResolvedHistorySnapshot<MaybePendingSnapshotId>, retaining current generic container.
- ResolvedHistoryChainSnapshot { chain_index:i32, snapshot_id:HistorySnapshotId }, required serde fields. TryFrom<&HistoryChainSnapshot> filters unresolved/invalid. From<ResolvedHistoryChainSnapshot> reconstructs same pending owner with raw ID, preserving native load acceptance of0/1.
- snapshots::resolved_snapshot_references(&[HistoryChainSnapshot])->Vec<ResolvedHistoryChainSnapshot> preserves order/filter.
- History default S becomes MaybePendingSnapshotId; generic HistoryTransactions<M,S> architecture kept. Current unit/raw-state fixtures remain generic but unit no longer falsely claims a resolved file ID.

**Confirmed bug:** old resolution_state<2 interpreted the Result Err discriminant as success; native TryFrom returns success for ID>=2. Actual complete procedure is37B, PDB source line263, not263B. Old tests asserted the wrong predicate and are corrected with explicit0/1/2/MAX cases. New tests source covers a writer thread completing the exact owner inside real default History, failed ID filtering, required serde fields and load reconstruction; tests not executed here.

Ordering: native x86 alignedMOV cannot distinguish Relaxed/Release. Current API chooses Release stores/Acquire loads as a safe host synchronization adaptation, not a recovered original Rust memory-order claim. Original snapshot data allocation/tag9 firstpayload is Box<SnapshotSaveData>; current snapshot producer Arc return is existing architecture, not proven original Arc. Reuse that API if useful, but share the separate pending-ID owner with the ledger and writer.

Original capture ID is generated from the snapshot blob via utils::calculate_hash, not chain_index cast. Save/thread owner must issue tag9 with actual captured data plus same pending owner, set_current_snapshot on the real History, assign hash ID on successful blob write, invalid on failure. Never mirror to SnapshotCaptureState. Saved references are metadata, not duplicate retained domain data.

Production propagation belongs to current owners: load/startup actual History construction, relevant history/terrain/wall/garden/mode registrations must use same canonical S; host/run_game explicit S is main-chain owner. Core tests that intentionally use History<()> must read that exact type rather than bare History after the default changes. No broad source migration has been applied by this candidate.

## Replay counter necessary owner adapter

`replay-counter.patch` is separate and belongs to main-chain-boot-owned TG-d63bc50864713c7344. Original pop_front1409f18f0 full395B/107instructions extracted directly from B;1409f190d increments +0x20 before the empty check. Current counter field defaults0 in new(speed), Clone preserves it, pushes do not change it. Existing Option-returning pop API is retained; counter increments even on an empty attempt and wraps like INC. Existing pop consumers use that same method. One small regression source verifies these cases; not run. No new Default speed is invented because this current type has no Default. Saved capture must compare this accessor, not a separate tracker resource.
