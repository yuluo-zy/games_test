# Decorator collider registration: bounded B contract

This supplements REPORT.md without changing its frozen artifact. No production source, claim lock, or earlier handoff was edited. The source before SHA remains `8161e74e7b74bc3d128187c872a626505e8e68e32212f026843597004bb86ffe`.

## Confirmed original behavior

The exact decorator monomorph is `RaycastWorld::upsert_collider<DecoratorId,(Vec3,Quat)>` at 0x141294980, complete 7047 bytes, SHA `5ab5d932857f586ca4872aed4ea04357b94a824295897029dd6289050cb9a4fe`. The retained 0x140b40b50 is instead the Entity entry wrapper (3447 bytes), and is not substituted for this monomorph.

At 0x141295067..0x1412951eb the live collider pose is compared with ordered float equality. Once that succeeds, 0x141295208 resolves the registered mesh and 0x141295225 calls `RaycastMesh::eq`. Only an equal mesh enters 0x14129523c, writing Kept=2. A different mesh enters replacement. Thus retaining the collider solely because its pose is unchanged is a concrete source divergence; there is no need to assume the key always changes when geometry changes.

`RaycastMesh::eq` at 0x1408cea30 is complete 215 bytes, SHA `0bf62e75b0a09a4e3fc18184a86121b4df3f8f77221e10662a6781d903a756dc`. It first compares the mesh hash (raw Arc allocation+0x80, equivalent to data+0x70), then the source tag and the selected source payload. The extracted jump table is bound in `source-equality-tables-B.json`. In tags 3,4,5, the compared fields are u32 at source+4, u64 at +8, and, only when +8==0, u64 at +0x10. Padding is not a new equality condition.

`get_collider_mesh` at 0x1408cdbd0, complete 66 bytes SHA `0965a159a994dcfd0e4f854b9859c04c4bd7686a0f26b7b96a66f68f6b4f99db`, checks exact registry generation and returns the live RaycastMesh pointer. The current Rust method only checks a vector and returns (), so it cannot perform this comparison or return the actual source receipt.

`instantiate_decorator_colliders` at 0x140829850 is complete 6470 bytes, SHA `3c184f6f8fc5935fd245bddc9e1bedf86f32a620c61529e9c34682b6f9fc900c`. The calls at 0x140829f2e and 0x14082a2db reach the same monomorph with the same DecoratorId and two distinct maps (second map base+0x30). The first source is tag3; the second source uses blueprint byte+0x139 OR4. Both copy blueprint+0x50/+0x58 to source+8/+0x10, and use DecoratorId at source+4. The source-to-slot table maps tags3/4/5 to slots2/8/14. They are separate owned colliders, not two shapes merged into a compound.

The original function has an early key/blueprint-pointer equality skip. This review does not assume arbitrary in-place blueprint mutation. When an updated blueprint reaches the upsert path with the same key, the original explicitly compares mesh/source identity as above.

## Current canonical ownership consequence

Current `upsert_collider` uses the Rapier per-slot index/generation as the global registry index/generation, while `ColliderMeshInstance` already has separate registry and Rapier fields. In two distinct slots, both Rapier arenas may allocate index0. Therefore the current `ensure_registry_entry` path is insufficient for the two real decorator maps: one entry can overwrite the other. The original uses its existing `SlotMap<RaycastMeshKey,RaycastMesh>` for global allocation and packs that key into Collider.user_data. Reusing the existing Rust `MeshInstanceSources` is the native dependency path, not a second collision owner.

The current `ColliderMeshInstance::handle()` also returns its registry fields rather than its distinct Rapier handle fields. A correct lifecycle must keep these identities separate and use the recorded source through the registry key.

## Transform scope correction to REPORT.md

Further complete ranges distinguish two native adapters. `impl21::to_offset_collider_builder` at 0x140b26a80 (323 bytes) calls nested_in with rdx=mesh.local and r8=external pose: external*local. `impl45::into_transformed_shape` at 0x140b26d20 (86 bytes) uses rdx=external transform and r8=mesh.local: local*external. The operand order differs and must be retained per entry point; a common guessed order is not sufficient.

The complete `RaycastMesh::raycast` at 0x1408ceb40 (194 bytes) consumes a caller-provided query pose and does not itself read mesh+0x50. The observation that the current simplified raycast helper passes identity does not independently prove that this method must automatically apply mesh-local pose. The canonical local-pose repair should not change this unrelated API without its own caller contract.

## Validation and limits

All statements are bounded static B evidence and current source inspection. No Cargo, CPU, GPU, original-process Edit/Reload, or full gameplay validation ran. No C equivalence is claimed. A new root delegation permits follow-up design/candidate work in the old collision claim's new subdirectory; it does not retroactively alter this read-only dependency report or its original ownership boundary.
