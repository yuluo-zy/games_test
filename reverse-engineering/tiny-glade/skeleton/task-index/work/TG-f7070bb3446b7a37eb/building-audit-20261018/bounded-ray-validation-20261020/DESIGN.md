# Bounded RaycastWorld ray intersection — design only

This is a separate delta after the frozen decorator-overlap candidate. It does not modify overlap design/candidate manifests. Root owns TG-f707 source-family claim and explicitly authorizes this same-owner bounded analysis/candidate; production and all Cargo/GPU remain root-only. No candidate has been written for this delta. Current production is e06d4429…252a1c; frozen overlap candidate is f6cfabe0…e18ae. The future bounded candidate must bind its exact accepted post-overlap source, not overwrite from an obsolete before file.

## Concrete consumer and original proof

04's complete original B greedily_connect `1407e1430/4312B`, PDB438:18296, is reused without re-extraction. Its manifest d2c7bc17f7bfe24712d0f437588f4bcbdf4982d12d5a9d680c9a2fd6f4e68f8c binds RNE f507a6ce…b7d03 and matching PDB2d9686b8…d8911. The callsite constructs the actual Ray with unit direction and sends max_toi=distance-0.01 in XMM2, world RCX, Ray reference RDX, and mask0x30db in R9D to World::intersects_any8cd5b0. It is a finite segment query; scaling direction while keeping an infinite bound is not equivalent.

Only the two missing callees were extracted, complete PDB-sized ranges with original-byte verification: World::intersects_any8cd5b0/5B (496:38372) is a direct jump to Rapier::intersects_any8cdf80/503B (496:46232). The latter saves XMM2 to XMM6 at8cdfaf, then for every selected slot writes that exact float to max_toi at8ce077 and broadcasts it into the traversal bound at8ce111..11b. It scans17 slots in ascending order with mask bit-test8ce006, uses solid=true at8ce07d, stops on the first hit, and returns false after exhaustion. It does not normalize/scale direction or clamp max_toi at this boundary.

## Current defect and narrow API

Current RaycastWorld public method has only(origin,dir,mask), and private Rapier::intersects_any hardcodes f32::MAX in intersections_with_ray. It cannot represent the proven finite cloth/flag segment and may reject connections because of a collider beyond the endpoint. Fixing it does not require another World or a new query algorithm.

Add only this public native-physics API:

```rust
pub fn intersects_any_with_max_toi(
    &self,
    origin: glam::Vec3,
    dir: glam::Vec3,
    max_toi: f32,
    mask: u32,
) -> bool
```

It forwards the actual origin, unchanged direction, actual max_toi and mask to the current private helper. Extend that private helper with max_toi and replace only its hardcoded f32::MAX query argument with the parameter. Reuse the current17 slots, existing bodies/colliders/pipeline, solid=true, default QueryFilter, and early-stop callback unchanged. No implicit flush, additional source/receipt policy, shape change, normalization, clamping, direction multiplication, allocation layer or replacement physics implementation is part of this delta. This accepts any f32 input as the dependency does; do not silently sanitize negative/NaN/infinite values into a different ray.

Preserve current unbounded compatibility APIs: World::intersects_any(origin,dir,mask) and Raycasting::intersects_any(terrain,origin,dir,mask) pass f32::MAX explicitly to the extended private helper. The latter retains its existing terrain-first branch with f32::MAX and mask0x20. The new bounded API is World-only and does not add a terrain query. Preserve all other APIs, including spherecast, shape_overlaps_with, raycast_simple, and the frozen overlap method. It makes no additional claim about previously unresolved general query policies beyond the max_toi propagation.

The new finite API is available under native-physics only. Current nonphysics compatibility stubs are not altered or called by this new consumer; there is no bounded false-success implementation. system-decorator already requests country-core/native-physics in its current Cargo manifest.04 has accepted the exact API and will pass original unit direction, distance-0.01 and0x30db; its consumer must expose an explicit unavailable/error gate if that backend is unavailable, never report an untested free segment. Cargo dependency changes are not needed.

## Caller coverage and package order

The full workspace Rust reference search is retained, including archived source copies. Actual private helper callers are just World::intersects_any and Raycasting::intersects_any in this source. The real water_audio_grid caller of Raycasting::intersects_any retains the exact old signature and behavior. The currently simplified flag_connections blocked callback is not falsely counted as an existing backend consumer.04 owns the incoming real greedily_connect consumer and its feature/error boundary. The only source candidate in this package is world.rs.

First root/source reviewer accepts and applies the separate overlap package. Then generate this delta from its exact accepted world source, explicitly preserving the overlap hunk/tests and binding beforeSHA. This package must not replace the frozen overlap candidate or silently replay the old e06d source. If overlap diagnostics require a new accepted hash, rebase this separate delta and update only this package's own manifest.

## Focused real Rapier acceptance (not run)

Tests create actual canonical colliders via upsert_collider and flush the existing pipeline before calling production queries. No injected hits, synthetic World, direction shortening or mock dependency.

- A real collider beyond the endpoint is ignored at the finite bound and hit by the existing unbounded query; the exact endpoint/just-before bounds distinguish correct max_toi.
- Mask filtering remains active with a near collider in an unselected slot and a distant collider in slot16: the finite bound rejects distant-only hits, a longer bound accepts it, mask0/high unused bits reject all. This covers late-slot propagation.
- A direction with norm2 demonstrates parametric TOI: the physical extent is |dir|*max_toi. Scaling direction and using f32::MAX would fail this test.
- Starting inside an actual collider with max_toi0 preserves solid=true. No result is claimed for unexamined dependency edge cases merely because parameters are forwarded.

Root builds/runs the focused bounded-ray and existing overlap/physics regressions after independent review. No original target execution, source implementation pass, application consumer compile, game scene, or GPU acceptance is claimed here.
