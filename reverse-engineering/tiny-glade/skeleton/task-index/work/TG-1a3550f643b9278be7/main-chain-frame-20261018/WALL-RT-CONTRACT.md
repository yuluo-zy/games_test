# B construct_walls → real RT mesh

Scope: original B `system_wall_constructor::construct_walls::construct_walls` at `0x140757800`, full 15336-byte PDB procedure. `construct-walls-B-rne-manifest.json` binds the entire RNE and complete range; decoded length equals the recorded native length. Fresh shared Ghidra read failed with `Corrupted BufferMgr state`; no restart, mutation or new shared analysis was performed.

The current authoritative seeded brick producer lives in core's `systems/wall/construction` and is consumed by `construct_wall_bricks`. The external crate's old `construct_walls` computes and discards records, corners and skirts. This hand-written external stub is not accepted as the original function. This patch recovers the missing RT side effect in the actual core consumer; it does not claim the entire external constructor or its original registration group is complete.

## Original producer and consumer

- `0x14075943c` and `0x140759609` call `RtWorld::get_mut<WallId>` on the same world. That helper calls `impl$5::from@0x1408bc520` to make tag0 and retain the wall identity. It does not use a prefab or compatibility tag1 key.
- `0x140759459` / `0x140759626` call `impl$2::from@0x1408bc510`. Its 14-byte body writes an sret discriminant u32=1 and payload at+8. It belongs to the albedo conversion consumed here, not evidence of `RtMeshHandle::from_handle`. The previous source attribution of that function to a handle constructor was wrong and has been removed. The compatibility API remains, explicitly without that native claim.
- The albedo branch resolves the wall's numeric ID through an accessor whose physical body is folded with `0x1400885d0` (`mov rax,[rcx];ret`), ORs `0x01000000`, and writes `RtMesh+0x58`. The enum payload is the WallId value, placed in a temporary stack wrapper before the folded accessor reads that wrapper. It is not a pointer stored inside WallId. `set_wall_albedo` encodes the same value directly, without dereferencing it. The OS worker independently checked the complete 14/4/6-byte helpers and both producer sites; its raw evidence is in TG-aca2b4fd535d278cb4/os-bvh-review-20261018/*wall-albedo-review.asm.
- `0x14075966c` / `0x1407599c9` clear positions length at mesh+0x50. The subsequent `WallTriggerVolume::default_tris@0x1408e80e0` returns real triangle storage, stride36. The PDB flatten iterator yields `[Vec3;3]` per triangle and extends mesh.positions. Existing mesh matrix is preserved.
- Both existing/new entity branches append the actual zero-sized `UpdateRtWorldEvent` to the owning Bevy event store after filling geometry. The current core constructor now performs the same write and event; update_rt_world then consumes the true dirty snapshot, and the original CWBVH builder consumes those triangles.

## Architecture boundary

Current `InstancedWall` keeps the source build's existing deferred sparse allocation/upload. No raw original Arc/lock offsets are copied into current resource owners. The complete original corners/skirt/signifiers, allocation timing and all nine members of the original set28 chain remain separate integration obligations. No incomplete group is installed as a ready world.

The local CPU registration helper supplies real empty RtWorld/event owners when absent. Production startup already inserts its one RtWorld and shared BVH worker, so init_resource preserves that owner. The fixture adds the real update system after its actual wall-brick producer; this local schedule is not claimed as the original global schedule.

## Validation

Pending new execution: the existing wall create/height-edit/delete fixture now checks actual trigger geometry equals RtMesh positions, wall colour reference, real CWBVH nonempty output and matching metadata, replacement without appending stale geometry, then deletion produces an empty actual snapshot. Earlier v19 RT and v22 intro results do not validate this newer patch.
