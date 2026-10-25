# Decorator save capture and canonical event delivery (B)

Scope: repair existing DecoratorStorage snapshot data and the existing Deferred queue output. Work candidate only; storage claim TG-889aa76ca030a2e3cd is held by terrain-curve-20261018. decorator.rs remains main-chain-boot owned. No production files, Cargo files or target execution were changed/run.

## Original evidence

Identity-bound PE/PDB are recorded in every native pack JSON and manifest. Every decoded instruction in each pack is checked against the original PE bytes. These are static bounded traces, not original runtime validation.

- capture_save_data 0x1417a0430, pdata main body3927B; at09c7..0a52 constructs the wall/terrain/stair chained iterator and calls snapshot collector1412fbcd0. Optional buckets are included when Some even if empty; removed registries/tracking/d2s/immutability do not forbid capture.
- collector1412fbcd0, pdata1522B emits40B address+Vec rows; bucket mapper141559890, pdata218B calls item collector1412ff7b0, pdata778B, and sorts each bucket by actual u32 DecoratorId. Item stride88=ID+HistoryState. There is no rotate/XOR ID representation; the masks in old restore are hashing, not snapshot ID encoding. Capture keeps outer iterator order; explicit host sort is a deterministic convenience, not a claim of original hash iteration order.
- existing HistoryState::from_info140b3f930,371B, is reused. Its nonzero info.state preview panic is original and remains. Native capture normalizes extra fields, not all padding or transient Info fields.
- restore snapshot140a20d70 remains an empty mutable-destination operation. Bounded projection spans 140a20d70+700B, 140a21023+320B, 140a21409+25B and original jump tables142ac9bdc prove: get/create bucket before child count check; reserve each actual ID; select extras by type, not nondefault field inference; write state0. Types38..3a→Window6,3b→Chimney4,3c→Lantern7,3d→Flag5,3f→state extra,other→Empty3. Ignored native extra padding is zero-normalized. These spans do not claim recovery of the complete26KB function or GPU/collider path.
- unleash1408337c0, pdata444B, plus exact typed generic caller identity: ResMut<DeferredDecoratorOpEvents>, EventWriter<DecoratorOpEvent>. Native drains queue len before iterating136B rows; tag5 discards remainder of that drained batch; emits payload136+EventId8 and increments real Events counter. Bevy owns the counter in the new actual system. Empty invocation still marks resource changed, matching its native unconditional mutable dereference. The old raw Vec compatibility method retains its local counter semantics and is not used by actual delivery.
- Canonical DecoratorOpEvent, compatibility Kind and historical Diff refer to one136B host type via pub use. There is one authoritative Deferred queue; no new payload/queue. Native kind ID dispatch is full-u64 tag3→+40/tag4→+60/other→+68; historical accessor now delegates it.

## Candidate changes

1. Existing DecoratorAddrSnapshot now contains Vec<(DecoratorId,DecoratorHistoryState)>; existing DecoratorStorageSnapshot contains Vec<(DecoratorStorageAddr,DecoratorAddrSnapshot)>. This repairs the same model rather than adding a second snapshot. Empty owners survive. Real take_snapshot now traverses live contents, sorts each bucket, and leaves source tracking/mutability untouched.
2. Existing restore_snapshot adapts that same model and retains its compatibility signature. New restore_snapshot_with_id_gen accepts the real authoritative generator; actual load must use it. The old entry has no generator argument, so cannot advance a runtime generator: it is compatibility only. Original reserve is verified before each child restoration, including u32 arithmetic. The existing to_info conversion is corrected in place with the verified type-based rule and state0.
3. Existing136B payload gains Event derive as DecoratorOpEvent. Kind/Diff names remain tuple-constructor aliases, without duplicate TypeIds. Actual unleash_decorator_op_events uses EventWriter and the same Deferred resource. The obsolete upper wrapper has a separate owner adapter patch reducing it to pub use of the authoritative Deferred.

## Validation

- Three candidates parse through rustfmt without touching source files (skip_children=true, no formatting rewrite).
- root.patch applies in an isolated temporary directory to all three frozen bases and yields each candidate byte-for-byte.
- Authored core regressions: nonempty all-address capture while immutable/with pending removal tracking; sorted real high u32 IDs; absent versus present-empty buckets; real preview panic; capture→restore→capture and next generated ID; default-extra/type dispatch; canonical TypeIds/full-u64 ID dispatch; actual World/Schedule drains with monotonic Bevy IDs and no duplicate read; empty drain change tick; existing send_state_diff_events→real canonical Events; tag5 remainder drop and next-batch recovery.
- All Rust tests are authored but unexecuted. No Cargo/test execution in this agent. No original dynamic execution, due previously reported usage approval failure; no retry or bypass. Rust expected values are not original fixtures.

## Required integration and boundaries

Root/owners must apply the shared decorator.rs/decorator_edit.rs changes together with owned storage. Save owner must update existing SnapshotDomainData/DecoratorsSectionJson to this same bucket schema and actual original serde, handled independently by history_audit. State's raw derive serde is not original wire schema. Original Addr JSON is {"Wall":u64}, "Terrain", "Stairs" (plural); static serializers are supplied in addr-serde-pack to the save owner.

Actual host must add_event::<DecoratorOpEvent>(), register unleash_decorator_op_events after actual queue producers and before event consumers, and ensure usual Events maintenance. Exact host stage integration belongs to main-chain owner; this candidate does not invent a global stage number or register a side queue. Save apply must pass its real DecoratorIdGen to the new restore entry. external-owner.patch collapses the current handle_external_decorator_cmds wrapper, preserving its name/API uses while fixing resource identity.

Not included/accepted: full original restore snapshot collision/GPU work; live consumer audio/visual completeness; upper cull_oob_decorators currently creating a local Deferred and dropping it; host queue/schedule wiring before owner integration; default serde of projected State; wall merge and its cull_allow_wall gap (paused per root). No unknown fields are filled to make these chains appear complete.
