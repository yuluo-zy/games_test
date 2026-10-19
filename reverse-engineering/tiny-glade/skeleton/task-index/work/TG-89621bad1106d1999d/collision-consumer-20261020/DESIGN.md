# Mesh-intersection receipt consumer companion — B scoped design

Root created and delegated claim TG-89621bad1106d1999d, owner collision-consumer-20261020, on 2026-10-20. The claim lock is unchanged. Production before SHA is `7667b03450b076ceea52bc5bb85d904719b22dd75b0e6ba57b296be2dde9bc8c`, saved as before/intersect_meshes.rs. Root owns all production edits and Cargo/GPU tests. This companion depends on the independently passed world local-pose/receipt design v2 SHA548bb5e0, same original MeshInstanceSources SlotMap owner. No new collision allocator, world, physical algorithm, registration system or gameplay scheduler is added.

## Actual current reachability

An exact search of all current tiny-glade/crates Rust source finds find_mesh_mesh_intersections only at its definition and the same file's graph-prune test; no current business registration/direct call was found. intersect_newly_added_collider is called only by the current world.rs regression test. Thus no live App intersection crash is claimed. These existing callable consumers nevertheless must accept the repaired canonical identity, rather than retain an incompatible Rapier-as-registry interpretation.

## Complete native evidence

evidence/native-manifest.json binds direct PE ranges to B RNE f507a6ce… and matching B PDB2d9686b8… via retained PDB procedures / identity-bound procs.tsv:

* find_mesh_mesh_intersections@140a97b70:13359B, raw SHAc62c4c4c420570c36bccff081d9ca95ddb375396db34711339331cf525045112.
* should_intersect_collider_meshes@140a9b3c0:284B, raw SHA2258cb3079bd9ed52020b6c9ed7e6ac1961580ae3695f35e428a13a3c240421e. Current source comment's 283B is not the complete PDB range.
* original newly-added call_mut closure@1417d1440:1274B, raw SHA19ab84fe8f7e4c4362a7d112f57c6448373df3ff45734bc7de60f80f79e3f134. Current comment's 1071B is not its complete procedure range.

The actual find routing is a handle SLOT gate. At 140a98388..140a983a5 it loads the diff handle's byte+8, compares 2, and skips added.slot>=2. It is not source.tag<2. At 140a983ae..3c9 it indexes the actual subworld arena and resolves that physical handle; missing physical collider goes to the original failure path, not continue. At 140a98401..426 it selects candidate slots: added.slot0 visits [1] only; added.slot1 visits [0,1]. The existing Rust loop over all17 slots is a divergence.

The complete closure resolves the candidate physical collider at 1417d15a4, requires it to exist, then at 1417d15bd..15c6 passes pointers to the actual physical colliders to should_intersect. That function reads each collider's first qword (user_data registry key), extracts its index/generation and resolves the source entry; it does not interpret the Rapier arena index as registry identity. Registered-source tags/payload then determine routing:

* tags(0,2) or (2,0): payload_u64@+8 must differ, and the tag2 source byte+1 bit0 must be clear.
* tags(2,2): payload_u64 must differ, source byte+1 values must be equal, and bit0 must be clear. Preserve the byte/bit operation, rather than inventing a general boolean flag interpretation.
* other tag combinations, including(0,0), retain the original explicit panic.

After nonempty polylines, the closure again resolves the two actual collider user_data keys, validates the source entries and tag&0xfd==0, and obtains artifact source payloads from entry+8. The current source's global-vector lookups using added.index/candidate.index therefore need the same receipt correction in this consumer as in the predicate. This does not change triangle intersection, transform, BVH, segment, polyline or artifact-building algorithms.

## Minimal companion implementation

The world candidate exposes its already-implemented actual resolver as `pub(crate) registered_mesh_for_collider(slot:u8, handle:ColliderHandle)->&RaycastMesh`. This method resolves the real Collider.user_data through the shared strict packed-key decoder (exact as_ffi round-trip before get), then the same existing MeshInstanceSources SlotMap. It neither allocates nor reads a parallel projection as authority.

The companion source uses that method for predicate and artifact source payloads. It retains the public SubworldColliderHandle function signatures, resolves their actual physical collider, then consumes the actual registered source tag/selected payload. The old projection-only lookup is removed from these real physics consumers. A non-native-physics call fails explicitly and does not return a fabricated successful predicate.

In find_mesh_mesh_intersections, keep removed-node graph pruning and the original existing callback API. Restore the observed added.slot gate and candidate slot list, and require the physical added collider to exist. Do not register a new system or claim the existing algorithm is complete. No intersection or polyline algorithm implementation is altered.

## Focused validation and coordination

Adapt the existing world.rs intersection regression from manually seeded projection entries to actual indexed mesh constructors, actual two-subworld Rapier insertions and the same canonical SlotMap receipt producer. Its two distinct source payloads reach the original predicate and artifact receipt path, so direct use of local Rapier index0 as global identity must fail the regression. Add a focused routing case showing real clutter slots2/8 are skipped before the callback and valid added slots0/1 visit only the observed candidate slots. Compare selected source payload/flag cases through real registered colliders; do not fabricate successful world receipts to preserve an old fixture.

The existing algorithm tests remain relevant but are not promoted to new runtime evidence. New tests, parser checks, independent static review and root execution must be reported separately. No C equivalence, original-process run, gameplay Edit/Reload, full collision algorithm closure or nonempty world-frame acceptance is claimed by this companion.
