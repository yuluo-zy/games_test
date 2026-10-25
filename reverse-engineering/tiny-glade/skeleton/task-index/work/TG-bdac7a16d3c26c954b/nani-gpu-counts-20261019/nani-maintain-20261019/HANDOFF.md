# Nani actual ECS → flush → graph handoff

Candidate only. Author: `/root/scene_startup_team`, under root's explicit delegation of stopped `TG-bdac7a16d3c26c954b/nani-gpu-counts-20261019`; no claim changed. Production, Cargo, GPU and Git were not mutated by this delivery. Full source/patch/dependency hashes are in `handoff-manifest.json`.

The current reachable blocker was concrete: `NaniInstanceDesc` and `InstanceKey` are already the real components, but all twenty current `systems.rs` updates accepted row slices and had no actual ECS producer or schedule registration. Thus actual descriptor entities could not allocate a canonical key, fill the actual GpuWorld payload, or reach the existing GPU count consumer through these functions.

## Original B facts

Identity: original release RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`, PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`. This is B evidence, not an A/C equivalence claim. The retained procedure index provided bounded code sizes/names; `extract_native.py` read only the specified native ranges after checking the binary's full identity. Thirteen complete functions / 43,976 bytes are recorded in `native-manifest.json` and adjacent disassemblies.

- `add_backend_systems@140942c80/763B`: call `1415f5310` at `140942e28`, literal `0x2c` at `140942e2d`, then in_set and Update add_systems. Set 44 is proven.
- `1415f5310/2122B` full PDB type: outer **six-item chain_ignore_deferred**, build_missing_vaos → rebuild_vaos → maintain → **explicit ApplyDeferred** → **unordered twenty-update tuple** → flush. `backend44-registration-pdb.json` retains all twenty complete function types, including filters/resources.
- `maintain@14206f300/6068B`: removals first, new descriptors next, changed descriptors last. Named creation matches requested kind among atlas parts; named change requires exactly one part and matching kind. Existing missing-part/unknown-remove/capacity failure behavior remains. Total request count increments even on capacity failure.
- Seven update bodies are bounded in the pack. Each obtains its owned-subset lock before iterating; the seven native queries have only `With<InstanceDataMarker<T>>`, no Changed filter. Preplaced flower obtains kind14 before TerrainRockMask lookup even with no rows (`14207396a..142073982`).
- `flush@142080640/1053B`: existing owned buffers flush, then request count is rolled into the prior-frame field and reset. Graph input remains actual live `subset.instance_count()`, distinct from capacity and request count.

## Current architecture adaptation

`systems.rs.candidate` adds the actual queries and shares the existing create/remove/change algorithms with the row API. The same `InstanceKey` component is inserted by Commands; no mirror component/resource exists. The explicit barrier makes the new key visible to updates in the same frame. Each update locks its own subset once and writes the existing `InstanceDataBuffer<T>` plus real cull data/dirty ranges. `nani_flush_instance_updates_ecs` uses the existing `GraphicsDeviceResource`, which `CoreRenderBridge::install_asset_libraries` already binds to the same App's BufferHost.

Before normal maintain mutation, a host completeness preflight examines only actual `NaniInstanceDesc` entities, including already keyed entities. It requires the real matching marker, Transform, and required ColorId/PrevTransform; a live key cannot change subset kind. Unsupported actual descriptors fail explicitly. Registered empty subsets do not fail the completeness guard and retain their real zero count. This guard is an integration completeness boundary, not an original B branch. Original subset/texture/mask failures are retained; no resource_exists skip or invented fallback payload was added.

The seven complete mappings are:

| Update tuple index | Kind discriminant | Actual mapping |
| --- | --- | --- |
| 0 | 3 | SolidVertexColor: Transform, optional SemiTransparent/FlatLighting |
| 1 | 4 | SolidVertexColorWindow: Transform, ColorId, optional SemiTransparent |
| 6 | 10 | Plant: Transform |
| 7 | 14 | PreplacedFlower: Transform, actual TerrainRockMaskSsbo, four PathRastersResources grids, WaterRaster |
| 15 | 22 | BalconyDoor: Transform, ColorId, optional SemiTransparent |
| 16 | 23 | Uber: Transform, optional SemiTransparent/AlbedoMap; real texture asset → actual bindless descriptor index |
| 18 | 25 | VelocityOnly: Transform plus current PrevTransform's named rotation/translation/scale fields |

A concrete caller identity gap is also retained: the only current decorator AlbedoMap writer (`visual_entity_builder.rs:655`) still inserts its local pending_core raw8B component. The canonical `drawable::AlbedoMap(pub u64)` already exists. Root must delete/re-export the local type and insert `AlbedoMap(handle)` to preserve the actual decorator → Uber optional texture input; otherwise the canonical optional query cannot observe that raw component. This adapter implements the seven canonical component contracts but does not claim that this caller is already migrated. No circular dependency from country-core back to decorator was added.

The remaining thirteen mappings are explicitly detailed in `payload-mapping.json`: glass (WindowLight/global control), bricks, gothic bricks, stone floor (Floor), two solid-texture variants, Wren, lantern, Plaster(pub Gap), cloth, smoke, curved half timber, stairs-floor chunk. Existing numeric views or similarly named owners are not accepted as canonical components. This delivery does not recover or silently skip those contracts.

`gameplay_startup.rs.patch` adds only the real Update set44 registration. Apply the hunk after matching `00c76412b9f23f4c450d163e8baa7706b56804bbd8513d2d749a82f83c6a4adc`; preserve previous Forest/canonical candidate hunks and newer root edits. Do not copy the entire host candidate over other work. Current canonical VAO maintenance stays in PostUpdate; the original first two outer-tuple members are therefore a disclosed ordering difference of the current host, not reimplemented here.

## Graph consumer and lifecycle

The actual current consumers already read this same GpuWorld: `render_bridge::import_gpu_world` imports Owned typed buffers; `scene_compute::generate_nani_draw_lists` forwards same World/device; `gpu_generate_draw_lists::owned_cull_batch` packs actual live count and same cull buffer BDA, and special cull uses the same typed data RG handle. Existing export must remain paired even on builder failure. The current `world_graph.rs` already calls this typed host at W22 with None HZB and W28 with Some HZB. That caller is present; the earlier limited-file search was incorrectly generalized as absence. Actual GPU execution remains unverified. No fixed max, CPU readback or synthetic zero count was added.

RemovedComponents drives the original Local active-map removal, including after level-unload clears actual GPU instances; the existing GpuWorld remove behavior logs a failed removed id rather than reallocating. This delivery does not fabricate a reload owner. Original current warmup/frame scheduling determines when set44 runs.

## Performance and validation limits

No per-frame row Vec or scratch allocation was introduced. The preflight adds one descriptor-query pass with a stack boolean array; allocation remains the native per-new-entity command/hash work. Each of seven update functions takes one subset lock, including an empty query, then existing flush writes only real dirty intervals. The plain current dense-index hint requires `&mut InstanceKey`; updates bypass change detection for cache repair but Bevy conservatively serializes the seven queries. Original native PDB queries read `&NaniActiveInstance`. This cost is disclosed, not measured or claimed faster.

Five narrow regression tests exercise actual ECS state: same-frame key insertion/barrier/payload/flush, removal+dense repair+changed mesh, unsupported descriptor before publication, registered unmapped empty subset retaining zero, and existing marker removal failing before stale-payload reuse. They use the existing recording CPU boundary and do not claim GPU execution.

`syntax-validation.json`: both candidates parse with rustfmt 1.9.0, exit 0. No Cargo typecheck, test execution or GPU run was performed. Root should first compile the two real crates against its synchronized source and run the `render::nani::systems::tests` filter, then perform the shared real-asset nonempty frame acceptance through the existing world-graph W22/W28 callers and import/export lifetime. Seven mappings plus explicit gaps are not a complete twenty-update or full-game equivalence claim.
