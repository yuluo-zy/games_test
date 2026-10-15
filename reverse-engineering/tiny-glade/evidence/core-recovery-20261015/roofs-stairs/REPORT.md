# B roofs/stairs recovery — in progress

## Source delivered

- `country-core/src/resources/roofs`: Roof component uses the confirmed 88-byte projection; animation, tile simulation, trigger geometry and commands have real Bevy owners. Trigger geometry owns typed Vec3/Vec2 triangles and reuses `utils::geometry::position_from_mesh_uvs`.
- `country-core/src/systems/roof`: creation, association, spatial update, editing, flat customization, invalid-roof processing, marking/fading/despawn, wall gable cache refresh, type-change gameplay effects and replay command routing. PublicWalls is the only wall-to-roof map. AppMode and GameTime use the canonical resources recovered by the other owners.
- `country-core/src/resources/stairs`: real petgraph 0.8.1 graph/backup, isolated-node stubs, dirty edges, endpoint degree guard, transient edge IDs, sorted snapshots, checksum, reduce/apply diff and undo. Assembly geometry implements wall normal, alignment offset/outgoing direction and path length. Unproven enum names/payload fields remain explicit observed inputs.

## Review corrections made

1. `create_roof` missing wall is an unwrap failure: original `1421ab0cd/0ed` branch to `1421ab5aa/5b6`. Corrected an initial skip. Creation now emits original sound kind 0/6 and geometry kind 0.
2. Flat style/rim lookup failure skips. This differs from edit/direction. Corrected initial panics using `building-edits/flat-findings.md` and B `142299dd0` consumer evidence.
3. Type change's flat/source_flag==1 branch does **not** set every height to 1.6. `1421acc7e..8a` computes `MINSS(14, MAXSS(1.6, old))`; exact 1.6 skips, other equal-clamped values still write, NaN propagates. This fixes the older runtime adapter's constant assignment. Its sound pivot is Roof::pivot_3d (stored floor), not floor plus roof height.
4. RoofAnimation is two f32 values (8-byte query stride); XMM register width in the constructor is not object width. `bypass_change_detection` prevents false Changed ticks on a zero-delta visit.
5. Stairs' self-pair means an isolated graph node. Dirty entries are not extra persistent stubs. Endpoint degree guard is >=2. Backup diff direction, removed-endpoint pruning and checksum schema follow original bodies.
6. StairWallAnchor::wall_normal is `(-sin(angle), -cos(angle))`. Original XOR sign mask and import targets contradict the older positive-sign chapter. AlignKind::position_offset has an f32 scale argument; tag 1 additionally multiplies it by 0.5.

## Initialization and evidence boundaries

Roof::new writes meaningful bytes 0..0x55; 0x55..0x58 is padding. TileSimulation constructor writes 0..21; 21..24 is padding. Initial zero arrays therefore choose padding bytes, not missing game values. Empty TriggerVolume is the original constructor state. Creation's Transform identity is explicit at `1421ab2bf..2f2`: zero translation, quaternion constant `14293e0c0=(0,0,0,1)`, scale constant `142ff76dc=(1,1,1)`. Spatial's temporary 48-byte projection is only read at translation bytes 16..28, all written by its recovered leaf.

Primary evidence is existing original-instruction recovery (`building-state`, `building-lifecycle`, `building-edits`, `building-history-owners`, `building-integrate/B-roof-switch`) and targeted fresh RNE disassembly here: `stairs-add-edge.asm.txt`, `stairs-checksum.asm.txt`, `stairs-alignment.asm.txt`. RNE baseline SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. No persistent database edits were made.

Claude's repaired helper succeeded (`claude-switch.result.json`, ok=true). Its clamp/NaN explanation was checked against the instructions and InnerWalls setter. Its caution that the pivot leaf alone cannot name the Y field is valid; naming comes from the separately verified constructor/spatial field stores. Claude output is not the fidelity oracle. Earlier Claude text analyses are retained; their unsupported container/change-tick claims were not implemented.

## Still open

The original full roof Nani/drawable bundle, RoofTiles/Spire/SupportBeams owners, visual type-switch VAO/transient mesh effects, trigger-geometry producer, roof noise and collision registration remain incomplete. Geometry/sound/hole/history notifications are actual queues but their remaining downstream owners are not claimed as executed. Replay routes genuine outputs; the shared history cursor/recording/bucket resource is not replaced by a roof-only synthetic owner. Stairs assembly request ownership, structural collider graph, visual/artifact invalidation and full producer remain open.

No module-completion claim. Parent owns unified compilation and final review; the existing numerical/original-instruction reports prove their documented ranges only. New tests were not added while implementation is prioritized.

## Unified history and local scheduling (continued)

- Added `resources/history::{EditType, History<S>}`. `History<S>` aliases the existing `HistoryTransactions<EditType,S>`; mode manager, raster recording, roof recording and cursor replay therefore use one Bevy resource. Actual tags 0..14 were decoded from original `0x142ad88d0`; supported payloads are typed, unknown variants are not represented by empty data.
- Added transactional `cursor_step_expanding`: existing cursor/anchor/recording guards, auxiliary-owner validation, complete expansion before cursor mutation, and original reversal at the command level. Failed expansion preserves the cursor.
- Added mixed `ReplayBucket` / `ReplayBuffer` owners, persistent ordinary Undo/Redo command queue, roof front-bucket consumer and three roof recording readers (all explicit/rim records, last visual record, all direction records). Replay goes back through the actual roof command systems and rim replay disables rerecording.
- Added local roof CPU registration into `WallCpuStage::RoofDependencies` and `HistoryCpuStage::Apply`. It flushes creation/deletion Commands before association readers and refreshes wall gable caches after spatial/association/edit effects. This order is runnable integration, not a claim to have recovered the original global schedule.
- Targeted tests cover real Bevy rim edit/record/commit/undo/redo with one History resource and no rerecording, plus cursor preservation on unresolved auxiliary owners. Parent runs compilation/tests and integrates wall/raster consumers.

Fresh original RNE reads also confirmed ordinary history command flags: main consumer `0x140af5ff2..0x140af6119` accepts byte1 0/1, uses byte2 bit0 for instant, and immediately returns for byte0==2 or byte1==4. The original interrupt constructor returns `[0,2,0,2]`, so connecting it directly to this branch is invalid; GUI producer only emits Undo/Redo. Full second event-stream/control processing remains open. `RoofReplayCommand::make_instant` follows original tag5 jump to `0x1409f1b70`: only mode is cleared, timing/undo/completion are preserved.

The second Claude call (`claude-history.result.json`) reports process success but returned only textual DSML pseudo-tool calls and no analysis. It was not used as evidence. This distinguishes successful CLI transport from a useful or verified reverse-engineering result.

Still incomplete: automatic UI recording start/finish (finish includes decorator and stairs diff owners), all nonempty auxiliary replay payloads, full second event stream, snapshot/input replay notifications, replay completion collision flush, and downstream roof visual/trigger construction. No module-completion claim.
