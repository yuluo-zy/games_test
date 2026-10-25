# Original collider-only stage: complete business contract for review

Scope: B wrapper instantiate_blueprint_colliders@140827510/PDB1009B and helper instantiate_decorator_colliders@140829850/PDB6470B, including their cleanup ranges. Exact continuous native bytes/assembly/PDB records are in evidence/COLLIDER_ONLY_NATIVE_RANGES.json (helper bytesSHA3c184f6f, wrapper8252854e). This is separate from the full visual/holes/cloth/tracker consumer.

## Canonical interface and owners

The original registered system takes ResMut<RaycastWorld>, ResMut<DecoratorPhysicsColliders>, Res<DecoratorBlueprints>, Res<PrevDecoratorBlueprints>. It invokes one helper on those underlying owners. Candidate implementation belongs to this existing blueprint claim and should reuse03's canonical world e06d4429 (`upsert_collider`, `remove_collider`, source factories); no new collider registry, physics algorithm or shadow map is needed. Production requires country-core/native-physics. No fake no-feature success is permitted.

## Original business sequence

1. Clone the keys of live first/collision and second/interaction maps into two independent HashSet<DecoratorId>. The remaining keys form actual pending removals. These sets are built from the live maps, not from prev blueprints; this matters when a blueprint is absent or explicitly disables a slot.
2. Iterate every current Arc blueprint. If prev contains the same ID with the same Arc allocation, remove the ID from both pending-removal sets and continue without upserting. The pointer compare precedes the skip byte. This is an original identity assumption, not a generic "same hash" optimization.
3. For a changed/new blueprint, if native+0x138 is nonzero, skip both slot upserts and leave its old keys pending for deletion. Current host field u138 carries that byte; its existing comment must be corrected. This flag must not default to true to avoid a failure.
4. For collision slot0/Mesh: construct RaycastGeoSource::decorator(id, actual storage addr), clone the actual mesh Arc, and invoke canonical upsert with cached world position/quaternion. Native source preserves tag3, id@4, storage discriminant@8 and wall payload@16. On replacement/kept remove ID from first pending set; inserted IDs were not present there. Removing unconditionally after a successful upsert has the same set outcome, but preserving the native result branch avoids an extra lookup on insertion.
5. Collision slot1/Disabled and slot2/Unspecified both skip the upsert, leaving any prior collision pending deletion. Native comparison140829e88 branches directly to second-slot check14082a21d when first tag is not0; there is no first-slot2 panic. This is more precise than the initial addendum's general missing-slot statement.
6. For interaction slot0/Mesh: construct source::decorator_interaction(id, addr, raw flags byte+0x139), preserving original flags|4 and legal tag4/5 guard, and upsert into the second map. Replacement/kept removes ID from second pending set. Slot1/Disabled skips and leaves old key pending deletion. Slot2/Unspecified is an original panic. **Do not prevalidate all slots before processing**: the original can already have upserted first geometry when the second-slot failure occurs. Keep that failure ordering/partial side effect.
7. After every current entry is processed, remove all remaining first-map keys through canonical remove_collider, then remaining second-map keys. This runs when no current blueprints exist and deletes stale geometry even when prev is empty. Deletion publishes dirty tiles/ColliderDelta and retires registry/Arc state inside the canonical core owner.

Exact native control anchors: first byte flag140829e7b; first tag140829e88; collision source/mesh setup140829e92..9f27; first upsert140829f2e; second tag14082a21d; flags byte14082a23c; second upsert14082a2db; first remove14082a97f; second remove14082aadf. The complete asm retains identity/set-build/cleanup loops, rather than relying on the old decompiler's mangled local types.

## Diagnostics boundary requiring a decision

The original helper conditionally logs inserted/replaced/removed/kept results and mesh names when imported PHYSICS_DEBUG_LOG_ENABLE==1 and log level permits it. Current core has no public definition for that original flag. None of these branches choose geometry, source, map updates or removals; they are still an original observable side effect and cannot be called recovered by omitting them. A candidate may recover this business sequence only with that explicit limitation until the canonical core owner exposes the original control. Do not create a decorator-local flag as a second owner, assume the imported byte's initial value from EXE IAT data, or change the render flow to satisfy logging.

## Required validation

* Same Arc in current/prev must keep live identities and generate no new ColliderDelta.
* Fresh clutter creates one shrunken first-map collider and one full second-map collider, with distinct canonical registry receipts even when both Rapier slots use local index0.
* Pose-equal mesh or source changes reach03's true replace path; a new hash-equivalent mesh Arc can be kept physically even though visual differential equality sees it changed.
* Disable first but keep second removes only first; nonzero skip byte removes both; removal of the blueprint deletes both; removed live keys are not derived only from prev.
* First Unspecified with second Disabled is legal and removes stale geometry; second Unspecified panics after any preceding first upsert, preserving the original ordering.
* Local translation is baked through canonical local pose exactly once, composed with world pose; no half-dimension double reduction.
* The wrapper belongs at original set25 position7 after generation and before actual queue flush. Prev snapshot timing is independently unresolved; do not copy early merely to make unchanged-identity checks pass.

No implementation is frozen for this phase yet. No Cargo/GPU execution or production write has occurred. This contract does not make full visual instantiation, window candidates, cloth outputs, trackers, or Nani draw lists complete.
