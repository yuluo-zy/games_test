# Complete brush startup and main-loop consumer candidate

Production untouched; no Cargo/codegen. Source parses with rustfmt, Rust typecheck/execution remains central's next validation window. Apply narrow patches, not whole historical source copies.

## Startup

`brush-startup-runtime.patch` contains nine source candidates under `proposed/`. `source-manifest.json` records each current base and candidate SHA. `host-startup-registration.patch` adds the actual Startup closure using the same installed root, ShaderBackend, AssetMeshLibrary, MeshWatch, ApplicationShaderLibrary and Commands. All original common shader PreStartup completes before Startup. Do not retain the historical OnEnter/Game mesh-only wrapper: previews persist KeepOnLevelUnload and spawn once per App.

Original B spawn140924b40/8361B SHA ec8a22e50b32fbeb1adcf84b11e301b35cacca7956534a24e00f829f1f1fb247. `native-brush-oracle.py` executes the entire unchanged original body; boundaries are asset I/O, mesh color method, shader compile/lookup, formatting/allocation and Bevy entity reservation. 26 runs comprise success,22 distinct mesh-result failures and3 missing-shader failures. Final native-startup-receipt.json and native-semantic-spawn-receipts.json capture22 registrations (21 JSON files plus Sphere),18 explicit color overwrites and17 typed commands. `oracle-debug.json` is an obsolete16-command debugging trace, never acceptance evidence.

Names and ordering matter: hand -> pet_hand; dashed_circle -> circle; source stem single_foward retains original typo. Cross/hand/circle/snapping line retain source colors; scribble overwrite is(0.5,0,0). The first14 entities queue before procedural Sphere/hand/circle/line loading. Later failures therefore preserve their original partial registrations and deferred spawns. Three vertex-color shader lookups occur at their original boundaries. Startup test candidate loads actual original mesh assets through existing TestDevice and matches native typed receipt, including absent Transform on14 entities, signed-zero snapping origins and zero scales; no GPU claim.

## Actual runtime

`runtime-registration-tuples.json` decodes the original PDB tuple, `runtime-registration-xrefs.json` binds its caller. run_game_thread1405a019c writes set7,1405a019e calls140363710 in_set, then immediately add_systems(Update) at1405a01ec. Six unchained leaves, no run_if:
1. human_scale_ref(Query Entity/Option Transform/HumanScaleRef,ActionSetStates,CursorTerrainRaycast,Commands,DebugGui)
2. brush_preview(Query Entity/BrushPreview/Option mut Transform,Overwrite reader,CursorTerrainRaycast,Commands,DebugGui,EraserSizeMultiplier,BrushSizes)
3. ui_2d_follow_mouse(Query mut Ui2dExtent/Ui2DFollowMouse,CursorPositionSS)
4. look_at_camera(Query mut Transform/LookAtCamera,MainCamera)
5. arrow_gizmo_animation(Query Transform/mut ArrowGizmo,EnableArrow reader,CursorMovedSS reader)
6. mesh_follow_terrain(Query Handle<Mesh> With MeshFollowTerrain,mut AssetMeshLibrary,TerrainHeightsData)

`runtime-registration.rs` mounts the three complete brush/look/arrow leaves only. The other three tuple members currently lack complete canonical source; do not register empty functions or call this the entire original tuple recovered.

PathPlugin.build140b68e40 calls chain_ignore_deferred140b4abb0(draw_path,path_brush_preview_size), sets16 at140b68f06, in_set_inner141453000, then add_systems(Update)140b69040. No run_if. Real draw_path must precede size in that existing chain. This delivery does not promote incomplete path raster/edit producers.

There is no separate FollowMouse system: original1870B mouse_terrain_raycast1408ef320 updates those components in its tail and already runs in actual set10 camera/raycast tuple. Current source preserves captured early-return, override-height/terrain-plane choice, cursor deltas/speed and true=>camera-ray45 vs false=>terrain hit; no duplicate owner or fabricated standalone follow system is introduced.

## Actual semantic corrections

Original full brush5355B executes in `native-runtime-oracle.py` with real archetype/query traversal, real event reader logic, direct and deferred Transform writes. Only profiler tracing is disabled.80 cases cover zero/multiple rows, absent/existing Transform, last-event wins, both flags, hit/miss, hidden reader retention and mixed kinds. Source previously iterated every event; fixed to read().last(). Original104B deferred record is insert<Transform> (PDB1412db8c0), not an unimplemented extra game bundle. Deferred identity insertion remains identity during this frame's scale phase, exactly native; do not eagerly scale newly inserted previews.

The original32B EventInstance includes EventId8. Event business fields are flag@0,position@4,kind@16 (20B align4); Bevy owns event identity. Remove invented head and excess trailing padding. All existing actual writers use typed flag/position/kind with Default, so no writer business changes. Old layout assertions migrated accordingly.

Original path608B requires exactly one matching entity. Existing source incorrectly used first matching row. Native64 cases (0/1/2/3 rows x4 modes x0/0.8/2/NaN) prove multiple entities cause no writes. Candidate uses query.single_mut() success branch; actual ECS regression consumes all64 original bit fixtures. NaN vertical max yields1 while x/z retain NaN.

Original Look1476B full query executes0/1/2/3 matching rows; only camera-eye getter and already recovered Transform.look_at helper are call-recording boundaries. Each actual call has maintained eye target and +Y up, preserves row translation. Candidate reuses canonical modern Transform.look_at, not a reconstruction of glam/old tools. LookAtCamera original descriptor proves ZST alignment1; SignifierReplayIndex proves usize8/alignment8. No guessed required-Transform registration was added.

Existing arrow2056B source is retained. Its per-row event consumption, shared axis guards, abs(delta*0.02) increase,0.1 per-invocation decay and no-clock interface match original assembly; existing original_loop_tests remain, no whole-file replacement. Original body bytes and hashes are included for independent review.

Remaining explicit limits: SignifierReplayIndex's original replay-active consumer140b1b860 is not in this delivery; no empty substitute. Snapping marker consumers and the other three set7 leaves are not implicitly recovered by adding their producer. Source tests are candidates, not passes. Independent color-menu owner is reviewing path/Look oracle and source separately.

Run native probes with Python313, not312. No package installation or LLVM/Cargo compilation was performed by this owner.
