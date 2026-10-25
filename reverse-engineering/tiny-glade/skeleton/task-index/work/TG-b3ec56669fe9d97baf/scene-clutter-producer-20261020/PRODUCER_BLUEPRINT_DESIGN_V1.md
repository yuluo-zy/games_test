# Clutter producer and shared blueprint candidate v1

This is B/Bevy0.16 evidence-backed source recovery. It is not C equivalence, a typecheck, or a completed visual/physics lifecycle. Root remains the sole production, Cargo, GPU and Git writer.

## Recovered current business surface

* The actual generator keeps the seven original SystemParam identities/order. It visits wall, terrain and stair buckets; a modified-ID lookup miss is a failure. It excludes types 0x38..0x3d and0x3f, admits destination tags2..4, performs the roof lookup before the type9 exception, reads the cached transform, and never invokes RNG or Commands.
* The canonical loader's real mesh metadata and original helper provide bounds, mesh name and nani name. Exact resize and birdspot float bits are retained. The solid selector comes from metadata+0x45, not the helper's third return field. The scale comes from the cached transform.
* Full and shrunk cuboids use full dimensions and original-height local translation. The canonical physics constructor halves dimensions internally. Constructor order is full then shrunk; blueprint slot order is shrunk collision then full interaction. The helper calls canonical `RaycastMeshData::new_cuboid_at`; there is no alternative physics algorithm or fake None result.
* The generator writes one actual builder into the blueprint visual list. It sets the cached outline transform once; it never directly spawns.
* Current and previous maps are separate resources, containing Arc<DecoratorBlueprint>. Snapshotting shares identity. `insert_for` preserves a unique allocation or creates a new allocation when shared, resets outputs and writes the actual storage address/interaction flags. This prevents edits from mutating the previous snapshot.
* The two mesh slots preserve native Mesh/Disabled/Unspecified tags. Positioned-mesh equality compares Arc identity plus ordered position and quaternion; it does not substitute mesh hash. Native first-slot Unspecified skips, while second-slot Unspecified fails. No consumer is supplied yet that conflates those cases.
* Clearing uses modified/removed sets of all three bucket classes and IDs in the removed-storage registry even after its bucket has disappeared. It does not invoke the current defective authoritative-storage cleanup. Previous-map copy has a real SystemParam interface but is not registered at an invented phase.

## Explicit compatibility and scope boundaries

Existing non-clutter generators still use legacy blueprint setters. Their legacy semantic fields/methods remain to avoid an unrelated rewrite, while `storage_addr=None` and Unspecified native slots make their unrecovered contracts visible. These values must not be consumed as a valid canonical source or treated as explicitly disabled. There is no overall blueprint PartialEq implementation and no fake implementation of full `instantiate_blueprints` in this package. The accepted builder PartialEq one-line API is only a comparison of its present host fields and is not used to claim native whole-blueprint equality.

The unexplained admitted0x3e type is outside the recovered56-variant ClutterMeshTy domain. Rust explicitly fails that invalid-domain input rather than choosing a default or another type. Its real valid-placement invariant is still an acceptance gap. Five helper/atlas pairings (type28 and30..33) remain unresolved: 04 has independently proved that Named resolution requires exact subset kind, so cross-kind lookup is not an explanation. Original helper output is unchanged. Runtime fixtures cannot claim arbitrary56-type acceptance.

Production uses the actual country-core/native-physics feature. `new_cuboid_at` is feature-gated by its canonical owner; this candidate requires that feature and the pending03 world candidate. Standalone system-decorator without that feature is not claimed to compile. No default-feature None/empty shape or success adapter is introduced. The old unrelated chimney helper stub remains outside this clutter candidate's recovered contract.

## Full consumer remains a separate gate

Original `instantiate_blueprints@140827910/7991B` has14 parameters: visual state, wall holes, Commands, RaycastWorld, DecoratorPhysicsColliders, WindowAutoClutterCandidates, two WallAttachedDecoTracker specializations, current/previous blueprints, three asset libraries and OnWallDeleted. It first invokes collider instantiation; handles wall deletion and wall tracker removal; accumulates holes, window candidates and cloth output; compares the complete blueprint; removes/despawns old persistent entities; builds and seals actual persistent entities; and updates chimney/door-stair trackers. Merely calling build_persistent does not recover it.

Current `WindowAutoClutterCandidate(s)` has no production definition. The retained collision.rs structures only name it and its Vec drain/equality relations; they explicitly do not define its fields. The namespace candidate source is `crates/country-core/src/systems/collision.rs`, task TG-c89f35d68343a58e6f. That existing source is not one of the missing-source declarations in the confirmed-sync inventory MISSING_SOURCE_SNAPSHOT.json (52 entries, including later toolbar additions). We have not created the resource or expanded into that source. Full consumer also uses the existing decorator-visual-state ClothInfo output whose current definition is expressly unresolved.

The existing tracker implementation at `decorator_visual/deco_differential_state.rs` (task TG-bd3d82a9ddc435cfb2) exposes only simplified with_diffs callbacks, not the original per-wall add/remove and wall-deletion behavior. Native calls are chimney remove1412ae1c0/add1412ad540, door-stairs remove1412add20/add1412acd70. Source mutations require a legal owner and a reviewed full side-effect contract. Clutter fresh blueprints have these optional arrays empty, but the actual generic consumer still owns their deletion/removal effects; this candidate does not silently skip them.

Original clear/generate/collider-only/flush are retained in set25 positions5/6/7/8. Full visual instantiate and previous-map copy are outside that19-member set. Actual startup companion is01's work, and full lifecycle registration awaits precise original registration evidence. Current storage clear_modification_tracking incorrectly clears the authority map and must not be used as an end-of-frame workaround.

## Evidence and validation

* Generator6631B/helper730B, loader5472B and two closures928/1028B, all exact constants and table paths are retained in the producer evidence directory and already independently verified in CORE_DIRECTION_REVIEW.json.
* New bounded full native ranges for insert1854B, clear3222B, copy957B are in the blueprint workdir's evidence/BLUEPRINT_NATIVE_RANGES.json. Every byte is continuously decoded; no shared DB was mutated or whole-program rescanned.
* Source-only rustfmt parsing succeeds. Added tests cover Arc snapshot isolation, inline/removed-bucket clearing, inverted predicate/helper boundary, exact half-unit shrink/NaN, preserved bounds center and actual birdspot payload. They have not run because root owns Cargo and current missing-source synchronization blocks a package build.
* Leaf canonical5314749b and loader v2 f3c82bdb have independent passed_scoped_static_requires_startup review; they are not sufficient to claim loaded atlases, generated entities, active Nani instances or nonempty draw lists.
