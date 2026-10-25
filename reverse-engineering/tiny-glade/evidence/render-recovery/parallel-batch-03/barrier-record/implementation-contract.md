# Candidate implementation contract

The R5 candidate is a CPU decision projection, not a Vulkan implementation. It preserves the R2 cross-package invariants:

1. Resource lookup owns `initial` and `reused`; a persistent pool hit is still a physical slot with `reused=false`.
2. Population consumes the current origin before acquisition and does not add graph-local reuse information.
3. Pre-pass no-predecessor barriers are emitted only for `reused=true`; predecessor barriers use the original concurrency predicate.
4. Record pending work is drained before access processing. `reused=true` is skipped by initial recording and handled by pre-pass logic.
5. A final batch gate is required before export transitions; an export falls back to `initial` when no last access exists.

`barrier-record-r5` intentionally exposes only numeric semantic fields and avoids claiming the original ABI, padding, Vulkan handles, or allocator behavior.

`snapshot-contract-checks.json` reads R2 snapshots without modifying them and projects two initial transitions, one predecessor transition, one export transition, the empty-pass final-batch shape, and backend-error partial consumption into native checks. It does not upgrade the complete R2 compiler from `candidate_static`.
