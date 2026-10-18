# Independent review: sheep snapshot correction

Reviewer: building-audit-20261018. Review target: root candidate under `TG-d2c94f036a54bdd78e/core-audit-sheep-20261018`. I read B/C original pack assembly before the contract/candidate and independently re-extracted every recorded instruction from each identity-bound executable. `sheep-review-bytecheck.json`: six packs, 841 instruction records, zero byte mismatches. This review did not execute Unicorn or Cargo and does not repeat the author's native execution claim.

## Result

No blocking semantic finding in the candidate's three-field correction, serde order, pure apply, or corresponding ECS apply adapter. The corrected binding follows original stores and serializers; it does not rely on Rust field declaration order.

- B capture 0x1412f064f..0x1412f0659 copies 12 bytes from first 48-byte column+0x10 into snapshot+0x1c. 0x1412f065d..0x1412f0660 copies the complete 16-byte third column into snapshot+0. 0x1412f0663..0x1412f066f copies second 104-byte column+0x30 into snapshot+0x10. Candidate maps these to Transform.translation / CritterGoal / Sheep.move_state. Current core Transform's translation is in fact at +0x10.
- B serializer 0x1416f9ee3 passes +0x1c with key position; 0x1416f9f15 passes +0 with key goal; 0x1416f9f22..0x1416f9f38 passes +0x10 with key move_state. C serializer independently uses the same offsets and key lengths. This validates the semantic reordering.
- B SheepMoveState serializer 0x1413ff52f selects Idle when discriminant byte is zero. Idle serializes timeout_s at +4. Walking serializes timeout_s+4 / goal_change_timeout+8 / force_reach_goal+1. Candidate derives exactly those enum names and wire fields in that order.
- B apply 0x1416f276a checks snapshot exhaustion before row writes. 0x1416f2774..0x1416f2794 writes position only, 0x1416f2798..0x1416f27a9 writes 16-byte goal, 0x1416f27ad..0x1416f27d0 writes 12-byte movement. No rotation/scale or sheep pet timer is modified. Candidate's shared apply_row preserves these unrelated semantic fields and zip retains the shorter-side policy.
- The three native change-tick writes precede the respective column writes. The ECS adapter obtains all three mutable components and passes each to apply_row, so deref mutation marks the paired components. The pure helper intentionally has no ECS tick contract.
- C apply has materially different table access/loop details, while matching these destination projections. The candidate correctly limits cross-version reuse to field mapping rather than declaring C scheduler/query equivalence.

## Non-blocking documentation correction

The inherited comment in candidate/resources/critters/sheep/control.rs lists key addresses `0x142c91f49/0x142c91f51/0x142c91f55`. Raw B serializer loads `0x142c91f4c/0x142c91f54/0x142c91f58`; independent executable read at 0x142c91f4c is `positiongoalmove_state`. Please correct that provenance comment. It does not affect wire output.

## Acceptance limits retained

The author's native_snapshot_probe.py drives capture's already-resolved dense cursor and apply with state+0x108=1 and one synthetic dense table. It tests field copies, ordered pairing, empty/shorter/longer snapshot counts and three tick arrays; it does not exercise the sparse/archetype branch or actual Bevy query discovery. The phrase complete apply should remain qualified by that dense-table boundary. Contract already excludes allocator/query discovery and full game execution.

None's inactive goal payload and Idle's inactive movement bytes are copied by the original but do not belong to the valid semantic schema; candidate deliberately canonicalizes the unused goal payload on None and does not declare byte-for-byte native ABI equality. That is consistent with the explicit contract. Full source of Sheep, system-critters/core type unification, debug UI capture/apply gating, actual save/load consumption and schedule remain unresolved and must not be inferred complete from this correction.
