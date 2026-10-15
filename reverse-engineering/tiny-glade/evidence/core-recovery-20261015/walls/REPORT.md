# B walls restoration — 2026-10-15

Status: active implementation, **not module complete**. The source tree now owns real wall state and consumes real path raster buffers. CPU creation/update/deletion and default trigger geometry are connected. Global schedule, complete decoration/hole producers and all supplemental constructor side effects remain open.

## Implemented and reviewed

- `resources/walls/public_wall_state.rs`: owned authority, initial terrain collection, prepared four-raster paths, regular/freehand height caches, gable cache, shape and attachment queries. Building byte projection writes identified fields only.
- `PublicWalls`: queued created/updated/removed changes, actual source validation, ordered change/delete notifications, explicit collection/path failures, real spatial dirty-tile candidate selection and terrain republish check. Roof associations share one map with roof lifecycle.
- `publish_public_wall_state`: consumes `PathRaster` and three cobblestone current buffers; no invented empty raster. Terrain and tabloid subscriptions remain explicit caller inputs.
- `wall_spatial_hash`: 64×64 cells, recovered radius-0.28 segment coverage, endpoint source-u/relative/terrain payloads, replacement/removal and event consumer.
- `calc_wall_bend`: final four XZ segment directions, three adjacent dot products, clamp/acos/min policy and f32 event.
- `delete_wall`: public roof-map removal, wall entity despawn and all optional plaster entity despawns.

## Evidence corrections from review

1. Publisher `0x140a8b530` calls `0x140b772b0`. Older wall chapter calls this animation-buffer `contains`; actual S_PUB32 identifies `hashbrown::raw::RawTable::remove_entry`. Its body erases a bucket, decrements length and copies 0x2a8 bytes; absence writes the i64::MIN niche. Thus there is no inferred animation-suppression predicate. The created and updated queues remain distinct; invalid source leaves an old public record untouched.
2. `PublicWalls::get_unchecked` `0x140a92010` is an optional lookup without curve validity checks. It is not an unwrap. `get` `0x140a91ef0` checks count >= 2 and positive length.
3. Missing-public branch of spatial update `0x140a8fd90` logs level 2 and continues. `0x140d4a330` is `log::__private_api::GlobalLogger::log`, not panic. Present invalid geometry still asserts.
4. Bend `0x140a8a710` reads the tail of the mapped Curve3, not an average over every segment. Its event contains only a float.

Primary local evidence: `evidence/country-core-deep/wall/decomp/decomp-<VA>.txt`, `evidence/render-architecture/public-symbols.json`, and already independently recovered `src/recovered_building` leaves with their byte/oracle provenance. Standard library / hashbrown / Bevy implementations are reused, not reverse engineered here.

## Claude aid

`claude-publication-response.txt` exited successfully under inherited DeepSeek configuration. It usefully questioned the `contains` name. It incorrectly interpreted `SBORROW8(0,cap)` as a general nonzero test and did not establish clone versus removal. Both claims were rejected: signed subtraction overflow here singles out i64::MIN, and original symbol/body prove removal. Claude output is not accepted as primary evidence.

## Verification and limits

Parent's central `cargo check -p country-core --lib` passed before the latest spatial/bend/delete additions; the only then-reported unused import has been removed. Those latest additions still await central compile/review. No claim of whole wall module completion follows from compilation. No fabricated GPU writer, unknown default terrain, placeholder trigger mesh, or default AppMode is introduced.

## Next dependency under active restoration

Original wall entity creator is external `system_wall_constructor::construct_walls::construct_walls`, B `0x140757800`, not country-core. At `0x1407598cc` it calls `InstancedWall::new`; `0x140759978` spawns `(InstancedWall, WallTriggerVolume)`. Update path replaces bricks and recomputes trigger volume, then updates RT/plaster consumers. Evidence: `evidence/building-integrate/B-core/140757800.c` and `.asm.txt`. Existing reconstruction `WallTriggerVolume { raw: Gap }` with `is_valid() == true` is rejected and will not be transplanted. CPU production uses the already recovered original seeded producer; GPU allocation and RT integration must remain explicit until their owners are available.

## Subsequent implementation

- Added real `WallHoles`/`PrevWallHoles`, checksum-preserving finalization and typed CPU `construct_wall_bricks`. ECS creates the original two-component CPU bundle `(InstancedWall<WallBrickInstance>, WallTriggerVolume)`; update replaces both outputs. Existing generic GPU upload is kept and no writer is fabricated.
- Added 120-byte semantic Quad construction, regular/freehand sides/top/end caps/special branch and cached triangles, normals, UVs. Native functions: `1408eae40`, `1408e9ed0`; private closure `1408e77a0` normalizes XYZ before projecting the perpendicular into XZ. `trigger-constants.json` contains original PE constants. Arbitrary `get_tris` query parameters and mesh-UV query remain outstanding; only the actual constructor's default-triangle path is connected.
- Corrected old source labels: source+66/Public+295 is FreehandHeightType (original CALL `140833230` at `1407581ec`); source+65/Public+296 is the separate raw flag. Core `WallState` derives the old portable snapshot's `special` predicate from its unique raw +65 byte. Publication preserves the raw byte; constructor tests nonzero, trigger branch tests exactly 1 where the native code does so.
- `special` nonzero skips from_curve and retains its initially empty brick Vec, but still recomputes collision and creates/updates the entity. The earlier report's claimed unknown alternate brick producer was incorrect (`140758117..12d`, `1407581b1..1b5 -> 14075936e`).
- Local resource/schedule registration exposes the required roof and opening dependency stages. Bound tabloid subscribers are gathered automatically; missing publishers are not invented. Added an ECS create/update/tabloid-dirty/delete test with explicit real terrain and four raster buffers, awaiting parent execution.
- Parent confirmed compilation of constructor/trigger additions. Claude constructor retry returned only inert DSML tool-call text despite `ok=true`; it supplied no usable analysis and was not used as evidence.

Remaining constructor side effects: supplemental half-timber/skirts/stair signifiers, RT-world and plaster callbacks, plus real application GPU allocation. These are not counted as complete by producing the base bricks.

## History and height integration

- `WallHistoryMetadata.source_66` now preserves original FreehandHeightType. Core `WallState` has one owner for this byte; every native/core snapshot retains it, and updates change that same metadata field. Existing portable imports may explicitly lack it (`None`); restoring a missing wall then fails, rather than fabricating zero. Actual new-wall zero is already supported by constructor evidence.
- `systems/history/replay_wall.rs` consumes only the unified front bucket and writes `InnerWalls` immediately in effect order. It reuses the recovered group interpreter, with an observation callback added to retain intermediate A→B→A state. The temporary per-command projection is not a second persistent authority. Original same-wall adjacent-group barrier and explicit-ID counter behavior remain intact.
- `systems/wall/adjust_wall_height.rs` consumes real height commands, resolves public flags and roof association, reuses the original clamp/terrain anchor/audio feedback algorithm, and performs each actual authority write. Local `WallCpuStage::Edit` precedes publication; GameTime is a required actual resource.
- Added ECS replay coverage for create/growth/delete boundaries, exact metadata retention, disjoint-group intermediate creation, and missing-field rejection. The existing lifecycle test now changes height through the real command consumer. These additions await central execution; no native differential claim is made for the whole assembled ECS pipeline.
- Color restoration and sound are exposed as explicit payload events. Their material/audio consumers remain required, and their emission is not counted as those effects having completed.
