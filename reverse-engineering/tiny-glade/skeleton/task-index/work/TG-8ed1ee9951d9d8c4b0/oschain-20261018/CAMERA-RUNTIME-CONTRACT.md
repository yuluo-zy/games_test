# Original B camera → world ray → terrain/depth chain

Scope: ten frozen candidate files in `camera-runtime-candidate-manifest.json`, separate mouse-depth/input delta, borrowed collision API, and host hunk. No production edits, Cargo or GPU performed by oschain. These are implemented candidates awaiting root review/integration/actual validation. Source/compiler success is not asserted. Current rhapsody API/architecture is preserved.

## Original authoritative evidence

B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; B PDB SHA2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Full exact PDB-length bodies and direct-call symbols are retained in this claim's camera-native-manifest and camera-registration-projection-native-manifest, Normal TG-c1de normal-native-manifest, Free/Walk TG-e97 free-walk-native-manifest, ray TG-1db ray-native-manifest, CPU depth TG-a544 mouse-depth-native-manifest. Old truncated decompilations are navigation only. Old guessed-length registration helper asm files are superseded by the exact-length `*-full-B-rne.asm` in the new manifest.

Observed physical business: camera_snapshot140911e40/1723B; reset_camera140912a40/468B; main_camera_update140912c20/6525B; paused1409145a0/646B; Normal update140949fe0/2339B, translate140949e70/365B; Free translate1409938b0/1146B; sharedFree/Walk rotate140993750/352B; Walk translate140994700/2084B/constrain140994320/990B; camera_mouse_ray1408ef1c0/339B; mouse_terrain_raycast1408ef320/1870B+closure1408efa70/717B; CPU readback140a1a480/1390B. Dependency/Bevy wrappers establish real registration, not additional recovered business functions.

## Exact schedule

Plugin1409125e0: reset_camera in9@140912613, on_event<NewSessionStarted>@622, before(camera_snapshot)@64e. Before target142c96f70 has PDB SystemTypeSet fmt1417114d0/system_type1417009f0 exactly the five camera_snapshot inputs. Then `(camera_snapshot.active, (main_camera_update.active, paused.not_active)).chain_ignore_deferred()` in9. Active predicate is actual GameTime.ran && StateGame. Original snapshot does not run while paused.

run_game_thread14059f9d9 calls1403c9610, actual chain `camera_mouse_ray -> mouse_terrain_raycast -> (mouse_capture,read_mouse_depth_value_from_gpu)`. The independent UI chain1400b18f0 is the second member of a tuple, in10@14059fb02/call05 and shared is_gameplay_active@14059fb57. Host hunk removes only mouse_capture from its incorrect PostUpdate location; shutdown remains its existing original owner. Current second UI/mode chain is preserved as the current adapter, **not asserted equivalent to the complete original independent UI/tool chain**.

## Camera ownership and corrected false interpretations

MainCamera owns the existing real Dolly rigs; no new fake trait or physics owner. Vec rotation header is cap100,ptr108,len110, established by full Normal update rather than equal cap/len1 in load. Original six valid modes0..5; the seven qwords after the six lens offsets are pivot field-offset tables, not modes6..12. previous140914ac0 belongs to CameraRotationPivot; removed its false CameraMode attribution/API. Actual+344 receives Modifiers low byte into Walk.allow_vertical; it is not a MouseDepth flag. MainCameraMoved is ZST (EventInstance8B writes EventId alone), not Vec3; two wall test constructors require the narrow unit-event patch.

Normal bodies retain one queue+one active orbit per real mode, smoothed lengths/rotations, completed-queue compaction, immediate load interpolation, current projection/inverse projection/view/inverse view. Free/Walk use current actual Dolly Position/YawPitch drivers. Free samples the actual terrain at the smoothed horizontal target. Walk uses eight collision-slide iterations, masks20 versus30ef for translation and0 versus30cf for floor, .5/.45 spheres, actual native ShapeCastOptions (max1/MAX,target0,stop=false,geometry=false), 34/68 soft bounds, real PathRaster floor blend and stored dt. Ground source writes only native tag when that was all the original wrote, preserving other bytes. Original collision-plus-terrain algorithm is extracted once into canonical RaycastWorld::spherecast_w_terrain; the old owned wrapper delegates.

Snapshot commands are owned by the existing DebugGui (native+0/+8), consumed only after keyboard37/38 priority. State durations6→2→4, one transition per update, selected-index insert/remove/select/panic boundaries retained. Reset drops vector storage and sets state/index0 while preserving old irrelevant elapsed. Current config.dev.demo supplies original process-global demo flag to body methods; this is a host projection, not an extra original ECS parameter.

## Main controller ordering

Aspect writes first. Active snapshot early-stop checks only digitalPan3 and analogZoom4; input cancels state without clearing elapsed. Emit moved unit while rotation2 or pan3 is held; copy the real modifier byte. Rotation pivot uses actual MouseDepth CPU groups/terrain and original inverse-projection cursor expression; keyboard analog5 uses CenterTerrain. Begin orbit is radius<130, mouse press/repeat captures, held consumes only the last cursor delta with finite/inversion gates, keyboard rotation only applies to Normal and sends hint2, release sends capture-release and finishes Normal/Tilt orbit (original excludes TopDown here).

Pan consumes the same cursor reader after rotation, combines actual analog2/3 with mode factors1/.25/2/.2, rejects nonfinite movement, invokes actual selected body and hint0. Wheel calls actual Normal/Tilt/Top zoom or changes the existing Butterfly index, retains original sign/wrapping/max0 semantics and hint1 float bits. Butterfly query uses canonical real24B component, never creates an animal. Mode comes from DebugGui unless actual selected Butterfly transform was followed. Write this frame matrices before terrain constraints update future body state, retain .2*dt rotation-notification threshold. Vermeer freeze uses0dt while preserving the input branches.

## Ray, hit and mapped depth

Original backend constructor140940db1..e30: CursorTerrain tag0/delta0/speed0 (inactive position bytes unspecified; safe Rust initialization0), CameraCursorRay origin0 and direction(-0,-0,-1), CPU MouseDepth56B all0. camera_mouse_ray always refreshes actual world ray, drains its own cursor reader, sends one unit AudioCursorMoved for movement/analogPan/Height. FollowMouse bool true follows ray at45, false follows actual hit; no hit leaves transforms and existing CursorTerrain state intact. Captured cursor early-stops terrain processing. TerrainEditingData originalOption<f32>@0/+4 overrides the plane; otherwise actual CPU terrain hit is compared to y0 plane by world distance. Delta only derives from an existing valid hit; zero/NaN Time.delta leaves old speed.

Separate frozen mouse-depth delta records/restores depth+mouseWorld+centerWorld in actual input frame, rejects incomplete version2 recordings using current version3 format. It does not pretend to restore native208B recording ABI. The real mapped64B SSBO is distinct from its56B CPU owner. No empty/default buffer substitutes for missing GPU resources.

## Integration requirements / acceptance limits

1. Root must downshift the existing system-critters Butterfly24B to core canonical identity and reexport, without dual Component identities.
2. Foreign TerrainEditingData owner must restore original cursor_height_override Option<f32> and its real constructor/editor producer, rather than a second resource or alwaysNone substitute.
3. Export canonical FollowMouse and resources::read_mouse_depth_value; GUI add the real request field. Apply borrowed collision API, frozen input delta and precise host hunk after checking before hashes.
4. Existing shared utils camera/ray helpers assert positive dimensions/FOV whereas original B236B projection has no asserts; valid runtime inputs use the same algorithm. No invalid-window behavioral equivalence is asserted. Window/minimize host scheduling must prevent unsupported viewport inputs or original helper guards must be independently corrected.
5. The independent second set10 UI/tools chain and save encoder/domain producers remain root/foreign integration work; this camera pack is not full game acceptance.

Meaningful new tests include actual_normal_camera_ecs_input_produces_the_updated_world_ray (original intro camera + original metadata envelope fixture, real CPU owners and actual ECS/controller/ray producer), native interpolation state boundary, real Dolly/terrain motion, full orbit soft boundaries, and live per-group mapped finite/panic-prefix/replay behavior. They are candidate tests; root runs them after integration.
