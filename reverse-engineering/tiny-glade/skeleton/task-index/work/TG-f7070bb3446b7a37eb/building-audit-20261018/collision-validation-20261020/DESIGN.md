# Canonical decorator overlap query — B design

Scope: replace only the existing simplified RaycastWorld decorator-overlap helper and add focused real Rapier regressions. The current world source SHA256 is `e06d44296c6b764221ce81bbedf5ef77fa66e25323a1d9e60a3b914657252a1c`. No production source/Cargo/GPU/native target is changed or run. Root owns the source-family claim TG-f7070bb3446b7a37eb under building-audit-20261018 and explicitly delegates this bounded candidate; the claim itself stays unchanged. The claim's historical target C does not turn this B evidence into C equivalence. Storage remains foreign and outside scope.

## Reused original evidence and resolved ABI

01's complete original B owner-result `14129be00/497B` (PDB1548:35052) and filter `14129c3d0/254B` (1548:40308) are immutable external inputs. They bind RNE `f507a6ce…b7d03` and matching PDB `2d9686b8…d8911`. Only four missing ABI helpers were additionally collected, complete PDB-sized original ranges, with verified source bytes: IntoTransformedShape38B at8d9960, slice Index43B at8cc5a0, SharedColliderShape accessor26B at8d9990, and SlotMap key data6B at8d99e0.

The owner-result arguments are self=RaycastWorld, owned transformed shape, current DecoratorId, current WallId, and ordered bool. Its actual validate call `1412b8928` supplies ordered=false. The caller uses a returned source to find the blocking decorator/derived outline, so the result cannot be reduced to bool.

At `129be6b`, EDX=17 and R8B=2 are passed to Index8cc5a0. That complete43B helper bounds-checks zero-extended R8B against RDX and returns base + index*0x390. **17 is the length of the slot slice, 2 is the selected slot. Neither is a mask.** This query only intersects canonical slot2 (decorator source tag3), preserving the original default QueryFilter except for its business predicate. IntoTransformedShape copies the actual input pose/shape; do not flatten the query, substitute a box, recompose local transforms, renormalize its quaternion, or make a new World. Use the already accepted current collider_isometry and actual shape.

## Predicate and result

For each dependency candidate handle, resolve the actual collider's user_data through the existing registered_mesh_for_collider helper and strict SlotMap receipt/generation checks. The complete original filter likewise obtains the actual collider, decodes its registry receipt, verifies slot bounds/full generation, then reads its source. Missing physical handles, stale/wrong receipts, and wrong source kinds must fail closed; they must not silently become a free location.

The original filter asserts source tag3 (`129c439..43c`) before inspecting source fields. Decode candidate ID at source+4, destination discriminant at+8, and WallId payload at+16. Acceptance is exactly:

```
candidate.destination == Wall
&& candidate.wall == current_wall
&& (ordered ? current_id > candidate_id : current_id != candidate_id)
```

The comparison is unsigned u32. ordered=false is the actual validation policy; true preserves the existing generic original mode for callers that need ID ordering. Self ID is excluded in either mode. A terrain/stair decorator in slot2 is excluded, even if its ID passes. A decorator on another wall is excluded. Do not use collisions from another slot to reject this query. Do not sort candidate IDs or substitute a fabricated first blocker: reuse the pinned Rapier intersection_with_shape traversal and return its accepted handle's actual source.

A successful original query resolves the returned physical handle through the same registry/gen path a second time and returns a borrowed source; no hit returns null/None. Proposed current API:

```rust
#[cfg(feature = "native-physics")]
pub fn decorator_overlaps_with_others(
    &self,
    decorator: &OffsetColliderTransform,
    current_id: crate::resources::walls::decorator::DecoratorId,
    wall: crate::resources::walls::WallId,
    ordered: bool,
) -> Option<&RaycastGeoSource>
```

This uses the current canonical24B RaycastGeoSource and existing source factory/mesh ownership. It introduces no duplicate source schema, fake source, parallel registry, generation relaxation, hidden owner transfer, or automatic pipeline flush. Output lifetime is borrowed from this World. The query receives the existing slot2 bodies/colliders/pipeline and QueryFilter::default().predicate(...). The predicate resolves the existing source, asserts tag3, and applies the exact fields above; after intersection returns, resolve again and borrow &registered_mesh.source. The dependency's predicate returns true to include a collider; this is not the inverse continue/stop convention of iter_shape_overlaps.

The public dependency is the existing PounceLight Rapier revision `7b5ece73c45a4d39bb983c48c283e773d7241050`, already locked in workspace Cargo. QueryFilter accepts a borrowed Fn(ColliderHandle,&Collider)->bool and default flags/groups/exclusions stay unchanged. Reuse the library rather than reconstructing its traversal. Keep root's integrated physics geometry/transforms/registry implementation untouched outside this helper and its tests.

## Existing defect and caller coverage

Current helper2750 accepts (shape,mask) and returns bool by querying every selected slot with no source predicate. It reports self-overlap and other-wall/terrain candidates, loses the actual hit owner, and cannot support original outline feedback. This mismatch is proven by both complete original bodies; no shader/render behavior is involved.

The full workspace source/test reference search is frozen in workspace-method-references.txt and caller-audit.json. There are no current production or test calls to this simplified method. All other matches are comments/definition and archived reports' before-source copies; those are not edited. Current shape_overlaps_with tests are for a separate API and remain unchanged.01 owns the incoming cull/location/validate consumer contract and has explicitly accepted this signature with ordered=false. That consumer will preserve Option/source and use the real transformed collider; no old mask compatibility wrapper is planned. Its source integration/compilation is a separate coordinated package, not claimed by this world patch.

## Focused actual Rapier regression plan

Tests live inside world.rs under test+native-physics, alongside the existing real upsert/receipt regressions. They create colliders with canonical upsert_collider, real shapes and sources, flush the existing pipeline at the actual boundary, and call the production helper. They do not inject query hits or create a shadow World/registry.

- Empty/self-only queries return None. A second actual same-wall ID returns its exact canonical borrowed source; wrong-wall/terrain destination and unrelated subworld overlaps are ignored. Test returned source identity/ID, not merely is_some.
- With only a larger ID overlapping, ordered=false returns it and ordered=true rejects it; a smaller ID is accepted by true. Self remains excluded. Avoid asserting unspecified traversal order among multiple eligible blockers.
- A translated/rotated elongated query distinguishes its actual pose from identity/flattened geometry. Preserve current shape and quaternion path.
- Actual removal + pipeline flush eliminates the blocker; reuse/upsert returns the new generation's actual source. Existing strict receipt checks remain in force.
- Faults at a real slot2 collider's receipt or canonical source tag cause panic through the actual query predicate, covering fail-closed stale/wrong generation and unexpected-kind branches. These are explicit invariant-fault tests, not claims that invalid original gameplay state is ordinary input.

Root alone builds/runs these tests after independent design/source review. No test result, native execution equivalence, application validate integration, full schedule, or GPU acceptance is claimed here.
