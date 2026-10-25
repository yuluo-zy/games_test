# B canonical history snapshot payload — read-only handoff

Status: direct B PE/PDB evidence and minimum current-architecture interface. No production, Cargo, JIT or database mutation. This is not whole snapshot-stream/domain recovery. Target PE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 / PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911.

## Confirmed owner and state

Original HistoryChainSnapshot is a chain-index record owning a cloned MaybePendingSnapshotId Arc, not SnapshotSaveData. The Arc allocation is24 bytes: two Rust Arc counters followed by AtomicU64. State0 is pending,1 is invalid; get/TryFrom accepts only unsigned values>=2 as the actual HistorySnapshotId. Native TryFrom is37 bytes, source line263 (not263 bytes): @140a84260..84285. It loads [Arc allocation+16], defaults result discriminant1, cmp value,2 / jb skip, writes id+chain and returns discriminant0 only for>=2. Current trait state<2 reverses this condition; its unit comment claiming native0 resolved is false. Arc counters themselves are at+0/+8; the observed+16 is the atomic payload, not a strong count.

Save caller @140a40124..401e8 captures world SnapshotSaveData, separately allocates Arc{strong1,weak1,state0}, clones this ID owner for AsyncSaveCmd tag9 (data pointer at+8, pending owner at+16), sends, then inserts that same owner through History::set_current_snapshot. Worker inline assign_invalid @141725b6c writes1; assign @141726d78 stores RSI (the computed real snapshot ID). Module1686 inline records124028/129244 identify MaybePendingSnapshotId::{assign_invalid,assign}; their children identify AtomicU64::store. Existing save source's cursor-as-u64 snapshot ID is unsupported and contradicts original hash-from-blob call @14172407c→utils::calculate_hash14145b1a0.

The native assignment/load instruction is an aligned plain MOV, no LOCK/XCHG/MFENCE in those spans. PDB names prove atomics but this x86 code does not distinguish Relaxed from Release/Acquire. Current Acquire/Release is a safe host adaptation, not recovered source ordering. The hash function body/serialization format is not recovered by these small spans; no cursor hash approximation is authorized.

Native SnapshotSaveData is a distinct owned Box payload (precise typed drop @140a3e7a0/module545), whereas current capture_game_snapshot_save_data returns Arc as an existing hand-written API. Retaining current Arc is a safe interface adaptation; do not claim original tag9's data pointer itself was proved Arc or use it as the ledger payload.

## Saved references and hydration

Original HistoryChainSnapshots deserialize @140b363e0 is116 bytes: deserialize Vec<ResolvedHistoryChainSnapshot> then call the exact conversion iterator @141737510 (452 bytes). That conversion reads each snapshot ID + chain index, allocates24 bytes, writes Arc{1,1,ID}, then stores Arc pointer and index in its16-byte output record. This does not load world snapshot data and does not duplicate History state.

Original reference object visitor @1415ce1c0 (1351 bytes) compares actual bytes for chain_index@142aee01f and snapshot_id@142aee0ac, calls typed deserialize_i32 @14159a8f0 / deserialize_u64 @14159d340, and has explicit missing/duplicate-field errors. Both fields are required; unknown fields go through deserialize_ignored_any. The conversion does not reject input ID0/1: retain their u64 bits on deserialize, then getter filters them as unresolved. Do not impose a new>=2 JSON/constructor validation absent from the original branch.

## Minimum current API, single authority

Retain existing ResolvedHistorySnapshot<S>{chain_index,value}, ObservedHistoryLedger and current/nearest/set helpers. Add HistorySnapshotId(u64), MaybePendingSnapshotId(Arc<AtomicU64>) with pending/default0, from assigned raw ID, get()->Option<HistorySnapshotId>, assign and assign_invalid. Implement HistoryChainSnapshotPayload on this owner with resolved>=2; preserve generic helpers/tests without pretending () is a real persisted snapshot owner. Production host must explicitly use History<MaybePendingSnapshotId> (or one canonical alias/default chosen by its owner) through every generic replay/recording/registration path. A new mirror resource is unnecessary.

Use the same ledger slice for saved references: serialize only resolved records to required {chain_index:i32,snapshot_id:u64}; deserialize those records to shared pending-ID owners initialized with each raw ID. A HistoryChainSnapshots serde adapter should own or borrow that existing slice/vector, not establish a second mutable authoritative Vec. Original resolved representation is a scalar ID plus index, not the captured domain Arc.

Tag9 must carry current owned SnapshotSaveData plus a clone of the same pending-ID owner. Only its worker can assign real blob-derived ID on durable capture success, or1 on failure. Insert owner into current ledger using its current cursor after successful command send; this makes later worker assignment visible automatically. Save snapshots JSON and GC referenced-ID selection must read this ledger and filter pending/invalid IDs. The present SnapshotCaptureState mirror has no synchronization producer and must not be treated as authority.

Load currently keeps Vec<Value>, parses private SavedSnapshotReference, then intentionally errors when a snapshot file opens; ignore_snapshot with nonempty refs also errors. Canonical pending-ID hydration fixes reference ownership only. It does not authorize silently skipping a valid original motomoto stream, missing domains, world writes, or failed load. Reuse eventual canonical reference type and retain explicit unsupported-stream error until its owner restores the real path.

## Ownership / verification

All families are history_and_persistence. At this read-only checkpoint: TG-3846e898cf43c29d09 history/mod, TG-3adf041a3108edb59a snapshot, TG-7babc3de8943103ac6 load_game, TG-a9bdfe0c3ca82529fb save/mod are main-chain-boot-20261018 claimed. Save_game TG-941c23b174ac2b4408 and save_thread TG-cefc5b92bca0aef69a are now oschain-20261018 claimed. Never claim over them. Root's resource-only candidate must be handed to the existing history owner; os worker owns tag9/flush/host capture adapters.

native-ranges.json binds9 ranges2378B630 instructions to direct B bytes; 4 small functions have complete PDB boundaries, with Box drop a fifth; caller/assign spans are explicitly bounded. pdb-records.json retains exact procedure and atomic inline records, symbol-identities.txt binds typed helper identities. existing-asm-verification.json additionally verifies existing actual save17300B / execute38878B / flush1934B transcripts (12781 instructions) against PE. No file named fullsave.asm was found; this handoff does not claim such a file exists or use a summary as independent evidence.

Suggested acceptance: same-owner pending→assign state visible from a cloned ledger payload;0/1 filtered,2/u64::MAX resolved; nearest filters pending before choosing; serde required/missing/duplicate/int width and0/1 hydration; real tag9 worker assignment changes saved references without a copy; invalid capture excludes its ID. Actual snapshot binary round trip remains a separate owner task and cannot be credited by this API test alone.
