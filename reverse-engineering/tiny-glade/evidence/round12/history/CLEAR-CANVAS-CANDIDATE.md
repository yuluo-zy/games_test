# Original clear_canvas consumer candidate

**Status:** complete observed function/registration contract reconstructed as a reviewable candidate; production is frozen, Cargo and full-App/GPU regressions remain central acceptance. This does not claim missing curved/plaster geometry producers restored.

## Deliverables and identity

- `clear-canvas-candidate.patch`: four narrow files, including new `country-core/src/systems/clear_canvas.rs` and exact map owners replacing two Gap resources. No shared run_game edit.
- `clear-canvas-candidate/`: complete reviewable mirrors; `clear-canvas-candidate-manifest.json`: exact source bases and candidate hashes.
- `build-clear-canvas-candidate.py`: evidence-only materializer, rustfmt syntax parsing with output discarded. **No Cargo execution.** Patch application check with `--ignore-whitespace` passed.
- `verify-clear-canvas-original.py`: original RNE SHA validation, eight bounded original function ranges/hashes/calls, exact typed system contract, raw vtable references and original registration instructions. Results `clear-canvas-original.{asm,json}` and `clear-canvas-registration-original.asm`.
- Original RNE SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
- Depends on the separately frozen `startup-unload-owner-candidate.patch` WaterAudioGrid owner API (`clear_activity`); keep the same canonical resource and its frame-only producer Local.

## Original sixteen parameter groups

PDB run_unsafe140375310/4803B binds business140a7abd0/6525B:

1. GraphicsDeviceResource
2. InnerWalls
3. WallSpatialHash
4. EventReader<ClearCanvasCmd>
5. AssetSsboLibrary
6. AssetTextureLibrary
7. TerrainEditingData
8. Tuple(Path, Water, Garden, Cobblestone1/2/3 raster owners)
9. Tuple(Wall, Gate, Halftimber color IDs)
10. History
11. ReplayBuffer
12. DecoratorArchivist
13. WallHighlight
14. Optional WaterAudioGrid
15. CurvedHalfTimberDrawables
16. AnimatedPlasterPeel

The candidate uses the current `LiveDecoratorArchivist` alias, preserving its actual native-physics/CPU collision owner identity. It does not add a parallel archive.

The current device boundary separates the SSBO device from `TextureHost`; the legacy TextureLibraryDevice impl on the bare GraphicsDevice returns Unsupported. The candidate first parameter is therefore a tuple of the existing SSBO device and a `ClearCanvasTextureDevice` adapter retaining the actual existing TextureHost Arc. It keeps sixteen SystemParam groups and never creates a second GPU/device/texture owner.

## Exact original registration, now closed

| Link | Original proof |
|---|---|
| business → typed system | clear_canvas140a7abd0; run_unsafe140375310 |
| typed system vtable | base1429c8a80; run_unsafe slot1429c8ad0 |
| run_if builder | unique LEA14037723c → IntoScheduleConfigs::run_if1403770b0/503B |
| actual event condition | helper1400d25f0/1138B names `bevy_ecs::schedule::condition::common_conditions::on_event<...ClearCanvasCmd>` at1429a6b05 |
| condition system | vtable142939fa0+50 → bool run_unsafe1400d1c00/330B |
| run_game registration | unique direct call1405a0237 to1403770b0 |
| actual numbered set |1405a023c writes0x21=33;1405a0258 interns SystemSet;1405a026a calls in_set_inner1400d68c0 |
| actual schedule | same config moved0xb00→0x440;1405a031a calls1403276f0/608B, exact PDB SubApp::add_systems<...,Update,...> |

Integration contract:

```
Update: clear_canvas.in_set(GameSystemSet(33)).run_if(on_event::<ClearCanvasCmd>)
```

Reuse Bevy's existing condition implementation. **Do not add Game-state gating.** OnExit(Game) produces false before the Reload frame; delaying it until a new Game session can erase newly loaded state. The condition has its own event-reader cursor; the actual consumer still drains its unread events and selects the last payload.

Host resource hook is `register_clear_canvas_resources(world, existing_texture_host_arc)`. It inserts the adapter and initializes only the two exact empty-map owners if absent. Keep the existing texture library, SSBO library, terrain, history and archive resources; no default substitutes are installed for them.

## Complete operation/branch contract

No unread command returns immediately, before touching mutable owners. Otherwise drain all unread events and use the **last** bool. The bool is `preserve_history`, matching replay_mode's true producer and ordinary ModeManager/unload false producers.

Original order:

1. Log; drop/rebuild the complete DecoratorArchivist with empty maps/options and culling=false. This drops its actual tracked/collision owners, retaining no fabricated parallel state.
2. Clear only WallHighlight.ty(+10), preserving its ID/entity payload.
3. Clear AnimatedPlasterPeel's real map, retaining capacity/hash state.
4. InnerWalls.remove_all, preserving removed-ID publication. Reset max_id **only for false**.
5. Clear GPU preset19, then CPU WaterRaster; clear GPU preset20, then CPU GardenRaster.
6. Clear GPU presets `[6,5,7,1,2,3,4]`; then CPU Path/Cobblestone1/2/3. Reuse WorldRaster.clear_all's full two-buffer/tile-hash/dirty publication algorithm.
7. Clear the two curved-drawable maps in original0/30 order. Construct/drop/replace the spatial hash owner.
8. Replace History with its canonical empty-glade constructor **only for false**. For true keep the complete actual history, snapshots/Arcs/cursor/anchors/identity-related state unchanged.
9. Replace ReplayBuffer with its original new(1.0) state for either bool, dropping real command and auxiliary Arc owners and resetting its original counter.
10. For each of Wall/Gate/Halftimber, in order: rebuild the CPU color table with20480 zero u32s, require the actual typed SSBO, then update exactly0x14000 GPU bytes. Preserve original failure/partial-update order; no missing/short-buffer default fallback.
11. **Only for false**, clear TerrainEditingData.strokes, reset next_index and rasterized. Preserve its cursor-height override/unsubmitted UI state. For true preserve all terrain editing state.
12. Set the texture-library change tick at the original point before final GPU clears `[9,11,13,14]`.
13. Optional WaterAudioGrid: retain grid allocation/base positions and clear canonical bits/i16 sums. Absence is allowed; present-empty still receives the original mutable-owner tick.

There are exactly thirteen GPU preset clears and six CPU raster clears. Older chapter prose reversed helper addresses: actual texture clear is14093bee0, actual CPU clear_all<512> is140a15da0. The bounded fresh call list is authoritative.

## Missing resource types are now exact owned data

**CurvedHalfTimberDrawables:** clear_canvas140a7b2da and2e6 both call140aaae80, second owner at+30. The callee's PDB alias and both remove_entry calls in remove_wall_id140aa9380/652B prove **both maps are HashMap<WallId,Vec<Entity>>**. Candidate uses neutral field names primary/secondary to avoid inventing a geometry role. Clear retains table allocation and drops lists; it does not issue despawns absent from the original clear policy. The separate native remove_wall_id with Commands remains the entity-lifecycle consumer.

**AnimatedPlasterPeel:** actual maintain_plaster producer14075b810/7237B invokes1411fcad0/412B, exact PDB `HashMap::insert<WallId,PlasterPeelAmount>`. The actual input is XMM2, preserved as one float; native insert stores one scalar f32 in a16B key/value bucket. Nani consumer14207b290/3588B loads the map f32 and falls back to the actual wall's peel if absent. Candidate represents this as a transparent `PlasterPeelAmount(f32)` and the real HashMap, preserving arbitrary f32 bits. No zero-size Gap or fake entry remains in this owner.

The original default constructor locations/module naming can differ from this owned Rust representation; field semantics, keys, values and clear policies are preserved. This is an allowed module/design update, not an ABI-layout claim for std::HashMap.

## Validation prepared and still required

Two nonempty owner regression candidates exist (not run): curved maps hold two real ECS entities and clear while retaining table capacity and leaving entities alive, matching the native map-only clear; peel map holds finite and payload-NaN amounts, then clears without replacing allocation or synthesizing defaults.

Central full-App regression should exercise actual GameSystemSet33 and the real TextureHost/SSBO owner:

- Populate a nonempty original history with decorator/stair aux, replay queue/aux, walls/removed notices, terrain stroke+counter+rasterized+cursor override, archive/collision state, curved maps, peel map, all six rasters, colors, GPU presets and water activity.
- Queue false then true: last wins; retain the exact History/terrain/max-ID owner state, clear all unconditional outputs, and verify queued command/aux Arcs release correctly.
- Queue true then false: normal empty history/counter/rasterization reset, preserving cursor override and source removal notices. Do not assert broad Default replacement of terrain/UI resources.
- Verify all thirteen real GPU preset clears and three color buffers; an empty Autumn smoke cannot replace nonempty state/readback.
- Verify OnExit false consumes on Reload before next OnEnter loader, and a following new nonempty session survives. Verify no-event frames do not change these resource ticks or queue clear uploads.

## Explicit remaining boundaries

- Actual CurvedHalfTimber/plaster geometry producers and their entity lifecycle are still incomplete elsewhere; recovering their owned maps is not a claim that their full algorithms/meshes are restored.
- The current historical architecture also has `LiveDecoratorStorage` outside the original parameter list. The original InnerWalls removal/archivist ownership must be checked against that current adapter's downstream deletion consumer, so a mirror cannot retain stale wall decorators after canvas clear. This candidate does not invent blanket deletion of unrelated special storage addresses.
- Registration is now fully observed, but production installation/runtime execution remains with central source-window integration. Rustfmt and patch checks do not count as business/GPU acceptance.
- No T13 state/capture producer is included. No rhapsody/general dependency was changed or restored from old code.
