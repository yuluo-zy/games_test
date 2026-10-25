# Actual prefab startup failure: original empty trunk branch omitted

Claim TG-d71445a8580100e7de / main-chain-boot-20261018. Original B only; work candidates; no production, Cargo/GPU execution. Root v64 actual-assets startup failed at process_prefab_mesh -> Mesh::From<Vec3> !empty. This audit does not weaken that assertion or insert a generic empty-mesh bypass.

## Proven root cause

The original trunk Code processor is filter_mesh_by_vertex_color140a08d30. load_prefabs_impl calls it at1412a2dec. The retained attribute map remains present, even when every vector becomes empty. Native @1412a2e47 reads one actual retained attribute's length into RBX (or EBX=0 if map empty @2f4c). It then deletes `id` @2f87 and drops the removed value. The complete original jump table142c2aaf8 (recorded in removed-id-drop-jump-tables.json) proves every tag0..3 drop path rejoins1412a3a91; the Float4 tag>3 path also rejoins it. `test rbx,rbx` @1412a3a91 and JE1412a4121 @3a94 skip ONLY appear_pos/age additions for a zero-length trunk, after deleting id. Nonzero trunks use RBX for the two defaults.

The independent canopy path stores its count at1412a2ccc in [rbp+308], optionally recalculates quad attributes, deletes id at1412a39e6, then adds missing defaults via1412a3ddc..411c using saved[308]. Its different removed-id jump table142c2ab08 is also recorded. There is no equivalent canopy count0 gate in this branch. The candidate preserves that distinction, rather than spreading the trunk gate over all processors.

Current handwritten process_prefab_mesh merged both paths and unconditionally added missing defaults, so actual empty trunk reached genuine From<Vec3> panic. The narrow patch moves count to its original position before canopy recalculation, always deletes id, then runs additions iff canopy OR count!=0. No source mesh, prefab part or atlas entry is fabricated/dropped by that patch.

## Mesh helpers independently verified; no helper change

Fresh complete PDB-bounded Mesh::retain_vertices140a08090/325B: first retain indices, then call VertexAttributeValues::retain_by_index for every existing map entry; it never removes the attribute entries themselves. It remaps keep==1 flags to sequential new indices, deleted flags to-1, and asserts that a retained index does not map to-1. Therefore an all-false trunk leaves empty indices AND empty vectors under existing attribute names, consistent with current safe Mesh::retain_vertices. The caller must observe length0, not infer absence of attributes.

Fresh From<Vec<[f32;3]>>140a0db90 has cmp[len],0 -> JE140a0dbc8 -> panic. Vec2 counterpart140a0db00 agrees. Full set_attribute<Vec<[f32;3]>>140a34c90/1449B has both insert @34e7b..e82 ->34fb5 and replace @34e0b..e0e ->34fd7 empty panics. Its trait/current conversion assertion is correct. Removing it globally would hide the loader's incorrect branch merging and change other genuine business failures.

## Actual shipped fixture and bounds

Original assets/meshes/far_distance_tree.json has152 positions and Vertex_Color blue exactly0 throughout; it contains id and prim_center. The real clearing/tree/far.ron has both canopy and trunk parts using this mesh, with inherited Code processors. The original trunk predicate blue>0 legitimately retains zero vertices; canopy blue<=0 retains them. Actual asset paths and hashes are in shipped-input-manifest.json. This is actual source-data evidence, not a statistics report.

Candidate includes a bounded real-JSON test: empty trunk preserves empty named vectors, has empty indices, deletes id, does not add absent defaults; canopy stays nonempty and adds defaults. A separate nonempty trunk fixture checks that the same path still creates per-vertex appear_pos zeros and age ones. These Rust tests are UNRUN here. The root's actual GPU-backed Startup fixture must be rerun; this patch only repairs this demonstrated CPU producer branch, and cannot itself prove the subsequent atlas/Nani/GPU chain completes.
