# Canonical clutter cuboid and source receipt repair

## Scope and source freeze

Root delegated this historical claim continuation on 2026-10-20; HANDOFF.md records the authorization and unchanged old lock/artifacts. The sole candidate source is country-core systems/collision/world.rs, before SHA `8161e74e7b74bc3d128187c872a626505e8e68e32212f026843597004bb86ffe`. Root owns production integration and all Cargo/GPU validation. This is B evidence; no C equivalence or live Edit/Reload execution is claimed.

The exact dependency report is the helper's physics-review-20261020/REPORT.md plus UPSERT_SUPPLEMENT.md. All full bounded ranges and PDB records are retained there or in this directory's evidence. Dimensions*0.5 already is correct. The fixes are mesh-local pose and the actual mesh/source registration receipt, without a compound wrapper, second registry/world, fixed counts, default resource, or custom physical algorithm.

## Native contract

1. Cuboid @1408cfff0 (290B) hashes dimensions and translation float bit patterns, constructs Cuboid(dimensions*0.5), and stores from_translation(translation) at data+0x50..0x6f. No extra dimensions scaling is added.
2. Tuple(Vec3,Quat) builder impl21 @140b26a80 (323B) forms external pose, then calls nested_in(mesh.local, external), producing external*local. This is the actual decorator upsert builder.
3. IsoTransform transformed-shape impl45 @140b26d20 (86B) calls nested_in(external, mesh.local), producing local*external. It is a distinct adapter. Preserve this observed argument order. The current extra parent argument is a reconstructed compatibility surface: apply that explicit outer parent after the original local*external composition, so identity mesh local preserves the prior parent behavior.
4. Constructor invariant: indexed, unindexed, and convex constructors already store identity in their tail. Sphere @1408d0120 (267B) also stores from_translation(second raw12B), not radius/hash parameter scratch. Compound @1408d0230 (347B) stores its raw32B transform argument and builds translated/rotated child shapes; no invented sphere/compound geometry algorithm is needed.
5. Decorator upsert @141294980 (7047B) compares actual final collider pose, then generation-checks the registered RaycastMesh, then compares data.hash and selected source payload. Only all equal yields Kept. Otherwise remove old registration/collider and insert a new receipt/collider. Native get_collider_mesh returns the actual RaycastMesh.
6. Source equality @1408cea30 (215B) first compares data.hash and tag. For decorator tags3/4/5: id@4, storage discriminator@8, and only for discriminator0 storage payload@16. Source padding does not determine equality. The source-slot table maps these tags to slots2/8/14.
7. instantiate_decorator_colliders @140829850 (6470B) uses the same DecoratorId in two separate canonical maps. First source is tag3; second source is original interaction_source_flags:u8 OR4. Both contain actual DecoratorStorageAddr bytes. This clutter producer sets the flag byte0; the public factory retains the byte operation and validates its actual resulting tag instead of interpreting numeric flags as bool.
8. The original global registration allocator is SlotMap<RaycastMeshKey,RaycastMesh>; user_data stores its key, independently of the per-slot Rapier handle. Current MeshInstanceSources already wraps exactly this dependency type. Current upsert mistakenly uses the Rapier local index as global registry index, so two slots may overwrite index0. Current ColliderMeshInstance already has separate registry and Rapier fields, but handle() mistakenly returns registry fields.

## Proposed source implementation

### Local pose

Preserve the current RaycastMeshData layout and tail fields; expose a semantic local_transform() accessor and a private writer encoding translation.xyz plus quaternion.xyzw in the original 32-byte tail. Do not transmute current utils::IsoTransform, whose reconstructed size differs. The unused translation SIMD lane is explicitly encoded as zero and is not hashed as a new business value. Default/identity constructors get a valid identity pose, without adding a shape to Default.

Add new_cuboid_at(dimensions:Vec3, translation:Vec3) as the truthful API used by the restored clutter helper. Keep the existing new_cuboid(Vec3,UVec3) entry as an explicit float-bit translation adapter using f32::from_bits, preserving current callers without numeric as_vec3 conversion. Both reach the same implementation/hash. No dimensions*0.5 change.

The necessary shared-tail initialization fixes are limited: sphere's existing UVec3 argument is decoded as translation bits before writing the already-proven from_translation pose; compound's raw [u32;8] argument remains raw pose/hash bits, and each real child uses its own local pose in the public Parry compound constructor. No other constructor algorithm changes. Identity mesh constructors retain their existing geometric/hash behavior.

The upsert builder uses local.nested_in(external) and existing collider_isometry/to_offset_collider_builder (no quaternion renormalization). into_transformed_shape_iso uses transform.nested_in(local), followed by its explicit reconstructed parent. Do not automatically change RaycastMesh::raycast: the original method receives a query pose and does not itself read mesh.local, so that separate API is outside this repair.

### Source bytes and one canonical registry owner

Reuse the existing query_policy::RaycastGeoSourceProjection([u8;24]) as world::RaycastGeoSource's representation, preserving the existing named tag constants, tag/subworld and from_slot APIs. There is no second source-owner type. Add factories decorator(id:DecoratorId, storage:DecoratorStorageAddr) and decorator_interaction(id, storage, flags:u8). They copy exact semantic id/address fields into their observed offsets and use the existing source-slot table. Semantic comparison for upsert follows the original selected payload fields, rather than comparing padding.

Use existing MeshInstanceSources SlotMap for allocation, Arc ownership, generation validation and removal. Reuse pinned slotmap1.1.1 public Key::data().as_ffi()/KeyData::from_ffi; do not write a free-list allocator. Collider.user_data stores that actual key. The existing colliders Vec remains only the pre-existing public source/generation projection for its consumers, synchronized from real key insertion/removal; it is not an independent allocator or mesh owner. The existing free-head/live debug counters remain projection state and do not allocate resources. on_level_reload already replaces the same whole World, dropping the existing SlotMap.

get_collider_mesh(index,generation) returns the generation-checked &RaycastMesh from that SlotMap. Kept compares both the composed live pose and the registered mesh's hash/source receipt. Replacement/removal drops that real SlotMap entry and invalidates stale keys. ColliderMeshInstance.handle() returns its separate Rapier index/generation/slot. No new map or parallel collision resource is introduced.

Actual ray-hit/overlap source consumers resolve Collider.user_data back through the same SlotMap and emit/callback the full source bytes, retaining id/address. This replaces the existing synthesized source-from-slot result for registered geometry. A missing/stale registration fails explicitly rather than manufacturing a source receipt. Existing manually inserted test fixtures must register through the real upsert path; they may not fabricate successful metadata to keep old assertions passing. Unrelated mesh-intersection algorithm reconstruction is outside this source-only candidate; any remaining cross-file contract is reported rather than silently broadened.

### Caller interface

main_graph_team owns helper and blueprint files and will call new_cuboid_at(dimensions,translation), preserve the two PositionedDecoratorMesh records and the two existing DecoratorPhysicsColliders maps, and pass factories carrying DecoratorId/DecoratorStorageAddr/interaction_source_flags:u8. Production writes remain root-only. No source edit here overlaps those files.

## Required focused validation (not run)

Tests should exercise real Parry/Rapier/SlotMap paths: nonzero XYZ cuboid translation gives expected AABB/overlap; dimensions are halved once; distinct adapter operand order is demonstrated with a noncommuting rotation; identical final pose/hash/source is Kept without republishing; changed dimensions or local center under the same key replaces; changed source id/address/second-tag under the same pose replaces; two real maps in different slots can both have Rapier local index0 while their global receipts remain distinct; ray/overlap returns actual decorator id/address; remove/reinsert rejects old generation and drops the stored mesh Arc; reload releases the same owner's receipts. Keep original query/count/algorithm behavior and existing relevant regressions.

All tests and parser results must be labeled separately. No new test has run, no original-process comparison has run, and this candidate alone does not claim that a nonempty whole world frame or gameplay Edit/Reload is closed.
