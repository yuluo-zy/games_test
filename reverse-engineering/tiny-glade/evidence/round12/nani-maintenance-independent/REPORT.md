# Independent original Nani stage44 scheduling review

Read-only production audit. No source edits, Cargo, or runtime acceptance is claimed. Primary evidence is original B PDB type identity plus original unmodified add_backend_systems machine code, not current schedule layout.

## Exact original contract

`country_core::systems::backend::impl$0::add_backend_systems`140942c80/763B. At140942e28 it calls1415f5310, whose PDB identity is chain_ignore_deferred of SIX outer items. At140942e2d the set byte is0x2c=44, interned at140942e49 and attached by141453000::in_set_inner at140942e58. At140942e9c it calls140b67280::SubApp::add_systems(Update). There is no run_if between chain construction and this add. The optional debug branch starts AFTER this add at140942ea1 and does not gate this chain.

Outer order:
1. Query(Entity,Handle<Mesh>) Without<Handle<Vao>>, ResAssetMeshLibrary, ResGraphicsDevice, ResMutAssetVAOLibrary, Commands: creates/attaches missing VAOs.
2. ResMutAssetMeshLibrary, ResMutAssetVAOLibrary, ResGraphicsDevice: maintains modified mesh VAOs.
3. nani_maintain_instances: newQuery(Entity,Desc,Transform) Without<Key>; changedQuery(Desc,Key) Changed<Desc>; RemovedComponents<Key>; MeshAtlasLibrary; ResMutGpuWorld; LocalNaniActiveInstanceCollection; Commands.
4. **Explicit bevy_ecs::schedule::executor::ApplyDeferred.** This applies the new VAO and NaniActiveInstance commands from earlier leaves before payload queries.
5. An ordinary unchained tuple of20 real update systems, all their original typed filters/resources retained in update-tuple.json. They are parallel eligible, not a chain. StoneFloor/Kind9 is index5, the sixth entry.
6. nani_flush_instance_updates(ResGraphicsDevice,ResMutGpuWorld), after all20 updates. Existing canonical helper flushes all owned subset buffers before rolling total_instance_count into previous_instance_count and clearing current count.

Outer chain_ignore_deferred does not remove the explicit ApplyDeferred node. Mounting only maintain.before(update) without the explicit flush cannot make a newly spawned entity match Query<&NaniActiveInstance> that same frame. Auto deferred at schedule end is too late for this consumer chain. Conversely, no deferred flush is implied between the20 parallel update leaves.

20 update marker order: SolidVertexColor, SolidVertexColorWindow, SolidVertexColorWindowGlass, SolidVertexColorBricks, GothicWindowBricks, StoneFloor, Plant, PreplacedFlower, SolidAlbedoMap, SolidAlbedoMapEmissive, Wren, LanternEmissive, Plaster, Cloth, Smoke, BalconyDoor, Uber, CurvedHalfTimber, VelocityOnly, StairsFloor. Original filtering for Plaster/StairsFloor additionally contains Changed predicates; do not discard them. Full query/resource identity is in update-tuple.json, not this abbreviated list.

Wrapper1420691c0 is848B and binds the seven maintenance params above. Wrapper142069810 is372B: Query(Key,Transform,Option<Roof>,Option<RoofAnimation>,Option<Floor>) With<InstanceDataMarker<StoneFloorInstanceData>>,Res<GpuWorld>. Their independent typed PDB identity also exists in startup-floors/nani-query-pdb-lines.json maintained by history_wire_fix. That owner exclusively implements canonical ecs.rs and complete Kind9/Floor producer, avoiding a second active-table owner.

## Current-source integration implications

Core render/nani/systems.rs already has the recovered pure row algorithms and real GPU storage calls. They are not ECS systems; registering row helpers directly or leaving empty adapters would not restore original queries. The actual gameplay App currently lacks this maintenance/update/flush chain.

The current modern host already registers asset_libraries::runtime::add_asset_library_systems in PostUpdate (game_loop.rs99, plus gameplay warm-up316). runtime::rebuild_mesh_vaos165 combines missing-VAO attachment and dirty rebuild and preserves newer retry/dedup/empty-mesh handling. Do not restore old VAO libraries or blindly register a second full AssetMaintenance schedule. Central integration must explicitly reconcile current maintenance placement with original44 before claiming the exact full chain; the Nani adapter can reuse the existing actual GraphicsDevice via AssetLibraryRuntime.buffer_device without a duplicate device/resource.

Maintenance changed-query permissions are originally read-only &Key; the pure row helper takes &mut key for index-hint resolution. Adapters should preserve query permissions and determine whether to use temporary copies/authoritative local key rather than silently changing the original query to &mutKey. Full native maintenance6068B execution and this hint-write policy remain the maintenance owner's validation work, not independently passed by this scheduling review.

Nonempty acceptance requested from owner: actual ECS maintain -> explicit ApplyDeferred -> actual Kind9 update -> GPU flush, backed by canonical MeshAtlasLibrary/OwnedStoneFloor storage plus a real recording device boundary; assert same-frame Key publication, nonzero typed payload and uploaded dirty data. Exercise removal detection and full-capacity no-Key/warn behavior separately. A CPU mock device validates calls/ownership, not raster output. Other19 real ECS update adapters remain necessary before claiming all20 complete.
