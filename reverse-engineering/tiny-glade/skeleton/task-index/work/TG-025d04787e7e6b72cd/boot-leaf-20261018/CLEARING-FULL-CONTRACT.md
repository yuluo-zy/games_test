# Complete clearing / Forest handoff (original B)

**Frozen candidate; no production edit, Cargo or GPU run.** `clearing-full-candidate-manifest.json` binds five patches and exact SHA, all `git apply --check`0. Forest downstream is separately bound by TG-d94f98bcd53745bd1e/boot-leaf-20261018/forest-consumer-candidate-manifest.json. RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; no A/C equivalence claim.

## Real input owners and initial state

Original init_clearing140a78aa0/6913B consumes real device, mesh library/watch, ActiveSession, shader library/watch, Commands, placement library, SSBO library, emitter count and InteractableForestPlacements. Current helper maps the existing host ShaderBackend to original shader compiler; it does not reproduce Rhapsody internals. Root path explicitly replaces original working directory. Same World/device/library identities are required. ShaderWatch/dev hot-reload remains a separate current-host missing capability; no fake watcher is constructed. MeshWatch uses its real library source path.

Initial LoadedParticleEmitterCount0 producer is original run_game_thread1405a194a..194f calling typed App.insert_resource14012c910. Its initial zero does not stand in for completed clearing. The init_clearing producer overwrites this same owner only after real data upload.

`trees.json` uses original Config Deserialize141416520/5827B. prefab and placements are required strings. particle_emitter is optional/defaultNone; billboard_expansion_scale defaults **1.0**, independently of per-placement scale. No guessed defaults for missing required files. Actual parser/I/O failures remain visible.

The actual placement library loads JSON into its canonical 48-byte type, registers a same-device GPU buffer and keeps path+handle ownership. This producer intentionally retains those registrations even when its returned handle is unused by the Forest record. Original RelativePath::to_path behavior is reused. Only exact prefab `clearing/tree/bush` is skipped, so `clearing/tree/autumn/bush` is not skipped.

## Complete producer behavior

1. Winter4: real assets/meshes/ice_cracks.json registered as ice_cracks, real /water/ice_cracks.raster.hlsl registered as ice_cracks, actual identity-transform DrawableMeshBundle spawned. Original command vtable1412dd460 is DrawableMeshBundle alone, so no invented KeepOnLevelUnload or shadow markers.
2. Flowery2: complete startup_flowery141553120/2224B. Real billboard_plants_2 mesh + existing billboard_plant_shader, DontCastShadow. Load actual glade plants/lavender.json and create one NaniBundle<PreplacedFlowerInstanceData> per actual PlacementData, original subset14 and literal mesh hash13fa53c58fdddee8 (actual atlas flowery_lavender), real placement transform. Load meadow_lowpoly_flowers mesh under meadow_test, existing meadow_flower shader, actual AlbedoMap(NameHash("meadow_flowers")). No fake mesh/entity/zero batch.
3. Olden1: complete startup_olden1415539d0/3588B. Six placement files: rocks/clearing-edge-rocks kind0 dimensions(1.3,1.3), stumps/clearing-edge-stumps kind2 dimensions(1.8,1.9), logs/clearing-edge-logs kind1 dimensions(1.8,4.4). Actual transformed local-X direction is projected to XZ then normalized; rectangle size=dimensions*placement.scale, center=actual position.xz. Return canonical PreplacedAutoClutter Vec of actual Rectangle24+kind byte (no fake names/host/rotation/placed flag). Clover mesh uses existing meadow_flower and actual Albedo/Alpha/Normal hashes23ae4b5c6ce0d14d/691e5a80737217f2/fbf3a82f352c1401. Clover flowers same shader, Albedo hash2bb813728bcb5dd4. Other glades remove this same resource through Commands.
4. Clear same Forest Vec and initialized=false. For each non-skipped config load real PlacementData, create real PrefabName, append only points `!(abs(x)<65 && abs(z)<65)`. Preserve all48 placement bytes and explicit billboard expansion; entity=None until the actual Forest consumer spawns one. No prefab index/id fabrication.
5. Optional emitter source is a real JSON mesh below assets/meshes. Original materializes **positions_without_indices**, drops temporary mesh, then for each placement and each position invokes original closure140a123a0/521B. Only actual placement.position.length()<45 and scale>0.7 emit. Transform is real placement.to_transform().compute_affine().transform_point3(vertex). Output is16B XYZ plus explicitly written pad0=0 (native140a12562). No mesh registration, index expansion or origin substitution.
6. Original get_mut ParticleEmitter None or empty rows skips upload. Otherwise full bytes must fit the actual buffer capacity; overflow fails **before** truncation. Upload exact prefix range0..lenBytes, offset0 through same real device. Only then LoadedParticleEmitterCount=min(actualrows,1100). This is not an entire-buffer clear or fake initialized contents declaration.
7. Real /clearing/distant_billboard.raster.hlsl, registered name distant_billboard_shader (original fmt pieces142ad6ca0/cc8); real distant_billboard.json registered under distant_billboard, actual identity-transform DrawableMeshBundle spawned.

Map components AlbedoMap/AlphaMap/NormalMap are canonical one-hash components in existing drawable.rs. Root should replace old draw-only view identities with reexports/actual reads. No host should insert mirrors. Original command PDB explicitly identifies each tuple; no original ABI cast is used.

## Actual frozen fixture inputs

`clearing-actual-inputs.json` hashes all five trees.json files, every actual placement/emitter mesh, six additional drawable meshes, lavender and six olden placement inputs, **25 concrete prefab RON inputs plus4 inherited default RONs**. Default files are not counted as prefab instances. Current asset::prefab::scan original dependency/inheritance walker remains the implementation; the manifest is input identity, not a second RON resolver.

Finite shipped-input predicates yield:

| Actual glade | Forest records | Emitter records | Uploaded prefix bytes |
|---|---:|---:|---:|
| Summer |1777|0|0|
| Olden |682|0|0|
| Flowery |629|330|5280|
| Autumn |1777|610|9760|
| Winter |1390|0|0|

Counts are static actual-input evaluation, not Rust/native/GPU validation. Autumn is320+240+20+30, Flowery140+140+20+30. A future strict GPU fixture must call this actual producer, verify real upload/readback, emitter count and XYZ/pad record data, then call W18 using actual GladeSettings and Leaf default producer. No shader output expected value is asserted here.

## Reload/OnExit facts and remaining integration

Full uninit_clearing140a7a5b0/1527B is retained. Original order: actual placement buffers.clear and paths.clear; audio-trigger collection drain; actual RtWorld meshes.clear and topology_dirty=true; clear only owned GpuWorld instance condenser/count (preserve GPU bytes, nextID, external/None, global frame counts); clear Forest records and initialized=false; optional WaterAudioGrid clears per-cell bits@8 and4-byte sample state@16; actual loading-audio snapshot branch. **This lifecycle is required before reloading duplicate placement paths.**

Root requested/owns exact RtWorld hunk in clear-runtime-owners/rt.rs.patch (only clear+dirty, preserve pending upload/counter). Render owner authorized GpuWorld narrow patches at TG-11d607188d6724d9f3/render-nani-main-20261018/boot-clearing, with core condenser adapter delegating existing shared clear. Neither is applied by this agent.

Remaining hard dependencies are explicit:

- Foreign prefab canonical SpawnPrefabCmd must carry real Placement48 and Despawn onlyEntity; Forest consumer batch intentionally requires those corrected fields. Root coordinates owner, no foreign overwrite.
- Root must add actual new modules/dependencies and call this producer at original registered session stage, then the actual Forest chain at set36. The candidate is not evidence that a runtime host currently calls it. Exact broader OnEnter/OnExit ordering must be verified before claiming lifecycle complete.
- Original uninit WaterAudioGrid has a sample field absent from current approximate canonical struct; no zero-fill or invented whole-grid clear is substituted. Real FMOD/None loading-audio state also requires owner integration. This package does not invent an SDK instance.
- Existing clutter add_preplaced_autoclutter has a false duplicate resource and simplified region geometry; fresh full RNE2180B (not old truncated1566B) proves actual Rectangle+kind source and real NewSessionStarted/SessionLoaded event readers plus Local FramesSinceNewSession. It is not an idempotent one-shot placed flag. This family is legally claimed for its subsequent correction; do not route canonical resource through the old fake consumer.
- Actual GPU/VAO maintenance, Nani instance flush, RT/audio Prefab writer ownership and live world-frame validation remain root integration/validation work. No full scene or complete gameplay success claim.
