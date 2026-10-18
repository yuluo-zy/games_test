# Original B startup_terrain, complete producer candidate

Claim TG-1a28ec1a34fa5472d9, boot-terrain-20261018. Work-only candidate; no production edit, Cargo or GPU run. Original body140926bf0/5382B SHA62ce8229575ad2063374d4749c23428b6cfb8e816bfabec7deaefaf9e91197cf. All original typed Commands closures and both grid helpers are complete PDB-bounded RNE, not decompiler fragments. RNE file identity and per-body hashes in native-manifest.json.

Apply startup_terrain-direct.patch to the existing startup module, append typed-host-install-fragment.rs to world_terrain, and invoke that installer only for the gameplay App after actual same-device/asset library installation. Existing module already exported. Canonical TerrainEntities {floor,rocky} is now present; single StairsStep flags:u8 candidate was separately authorized by os_chain and root has integrated it. The current host's explicit installation/demo inputs replace original process CWD/global143634d90. ShaderBackend uses the existing shader host, not a second renderer. Original ShaderWatch explicit parameter is subsumed by the current loader boundary; live-watch mode is not independently validated by this candidate.

Exact stage: startup140926bf0 -> run_unsafe141768570 at141768894 -> unique run pointer142cd7cf0 in vtable142cd7ca0 -> into_configs14176a370 lea14176a4bb -> Startup eight-tuple14099b8b0 call14099baad, fifth of eight -> SubApp::add_systems<Startup>140b67b50 -> add_game_startup_systems140941110 at14094117f. This is not Grass's OnEnter(Game) eleven-tuple. No chain order among Startup peers is asserted. Common13 must already have been loaded by the original PreStartup stage.

Native sequence and owned outputs:

1. `grid cw`, CW90x90 -> watch/add `floor`; load floor_shader; spawn actual floor entity KeepOnLevelUnload+DrawableMeshBundle.
2. Only demo: CW45x45 -> watch/add demo_text_floor; load demo_terrain_overlay; spawn actual overlay with KeepOnLevelUnload+Drawable+DesignatedRenderPass16.
3. Load original assets/data/rocky_terrain.json through current restored RockyTerrainMesh loader using the same actual SSBO library/device. Publish actual vertex count. Load rocky_terrain shader. Spawn rocky using the SAME floor mesh handle. Publish TerrainEntities with the two actual spawn results.
4. Actual stairs_step.json -> stairs_step_mesh, shader stairs_step, StairsStep flags0.
5. Actual path_pebble.json -> path_pebble_mesh, shader path_pebble.
6. Actual stairs_pebble.json -> stairs_pebble_mesh, reuse SAME stairs_step shader, StairsStep flags1.
7. Actual terrain.json -> terrain, lookup existing solid_vertex_color_albedo_map, BackgroundTerrain+DontReceiveContactShadows+OverwriteRoughness exact bits3f666666 (0.9).
8. Actual terrain_rocks.json -> terrain_rocks, lookup existing solid_vertex_color, FlatLighting.
9. Actual billboard_plants.json -> billboard_plants, shader billboard_plant_shader, DontCastShadow.

All ordinary entities carry KeepOnLevelUnload and original identity Transform. Original typed command functions1412d7c70/1412d8b00/1412dc9f0/1412dbb00/1412d9790/1412db2a0/1412da440 prove component tuples. Raw original-rip-constants.json records every string address; no guessed file, empty mesh, zero metadata or placeholder entity is supplied. Actual normal drawables8 / demo9, registered normal meshes7 / demo8; these are assertions about this bounded producer, not global recovery statistics.

The existing handwritten grid helper had added a row-width restriction, checked-overflow rejection and eager total reserve not in either original function. The candidate keeps its four-column ordering architecture but restores original wrapping u32 index arithmetic, native `(end+1-start)>=6` block predicate and individual push order. Original only rejects end<start (unwrap), not end>cells_per_row. Existing normal-order tests remain; two native boundary examples verify wrap and columns beyond nominal row width. The source contains a bounded CPU integration test that loads the actual shipped JSON assets through the actual loader/libraries and applies Commands, checks two live entity identities sharing one actual floor mesh, two StairsStep flags sharing one actual shader, background roughness and demo branch. TestDevice is explicitly a CPU wiring recorder; this test is UNRUN and cannot prove actual GPU startup or W5 rendering.

Remaining downstream requirements: actual mesh-to-VAO maintenance must publish those entities' real Handle<Vao> components before W5. W5 direct candidate consumes these exact entities; full72 mapped outputs and ColorLibrary/texture producers must exist. This candidate does not claim W7 top-down stairs generation, full main graph, Steam or end-to-end game readiness.
