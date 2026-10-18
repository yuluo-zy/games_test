# Scoped review — B components

Actor: components-review-20261018 (implementation and self-review).
Independent reviewer: not performed; no independent review completion asserted.

Review used raw retained writer/reader instructions, fresh B registration
descriptors, and direct Rust callers. Native query results were checked for
top-level/per-function failures; 26 analyzed instruction spans match release RNE.
PDB void() signatures are not treated as author ABI.

Resolved findings:

1. Major: deleting trees by duration instead of progress (+4).
2. Major: query-order Local progress assigned to a different entity after removal.
3. Major: identity-bearing TreeComponent and owned ConvertedPreplacedTree treated
   as ZSTs, losing tree index/flag/name.
4. Major: UI image name discarded; placeholder cannot represent owned String/layer.
5. Major: u32 order field truncates the original u64 sort key.
6. Major: Clone/Copy on components changes Bevy clone behavior from Default to Custom.
7. Minor: mesh selector duplicated and incorrectly registered as its own Component.
8. Minor: bundle copy extent confused with local allocation size, cross-domain
   UI types placed in trees, registry address naming/sort description misleading.

Implementation preserves native constructor values and per-entity animation
state, uses dependency types for ownership/math/ECS, and leaves padding implicit.
Proof obligations covered by focused integration checks include byte offsets,
real Component descriptor behavior, world entity/archetype movement, image
ownership, >32-bit order values, and actual deletion-system event output.

Remaining boundaries: placement GPU writes, complete preplaced-tree command
flags, global canopy resource, original schedule, missing registry payloads and
whole native runtime equivalence. Current typed fields do not close those chains.

Build failures outside this patch are retained in verification-logs: cfg(test)
texture import trait mismatch, workspace water grid field mismatch, and a
transient missing GraphImageLease while shared rhapsody source was changing.
The final component integration retry passed. Old reports are not reused as
acceptance of the changed types or the unrelated backend changes.
