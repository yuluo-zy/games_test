# Legacy preplaced-tree conversion: candidate checkpoint

Status: candidate only. No production source changed and no Cargo command run by this owner. `production-review.patch` applies cleanly to the current shared source; `candidate-checkpoint.json` records all 15 base/proposed SHA256 pairs. Central integration must recheck every base before applying.

## Original identity and complete extents

Original B RNE SHA256 is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. `native-ranges.json` binds original bytes and addresses; `extract_native.py` extracts directly from that original file. Retained complete functions include:

| Function | VA | Bytes |
|---|---|---|
| spawn_preplaced_trees | 140862270 | 6013 |
| convert_preplaced_trees | 14226be70 | 4827 |
| occupied-cell circle callback | 1425defd0 | 837 |
| PlacementConfigEntry visitor | 141416520 | 5827 |
| insert_starting_edit main body | 140a844f0 | 216 |
| grid cell center | 140aa5c70 | 345 |

The original registration code explicitly writes `0x30` for `systems::prefab::PrefabInstanceData` at 140b7e6ed and `0x58` for `resources::prefab::PrefabInstance` at 1415522dc. These are opposite to current production definitions. The candidate moves the existing owned aggregate to its canonical resources module and restores the placement wrapper in systems. No prefab mesh, RT, Nani, audio, watcher, or dependency algorithm is replaced.

## Recovered request-to-history chain

The `<19` parser flag causes the real load system to publish `SpawnPreplacedTrees` after replacing History. Both original consumers are event readers in Update, gated by Game state. The asset producer is not an OnEnter startup function. A whole event batch triggers one conversion request and both readers advance to its end.

The producer loads the theme directory's `trees.json`; `ActiveSession::assets_root()` already owns `assets/glade/<theme>`, so joining that prefix again would be wrong. The configuration visitor has four fields, not two: prefab, placements, optional particle_emitter, and billboard_expansion_scale defaulting to one. The exact bush name `clearing/tree/bush` is skipped before its placements file is opened. Relative placements are resolved with the existing RelativePath dependency.

Each asset record is the existing full 48B PlacementDataSsbo. Accept only strict `abs(x)<65 && abs(z)<65`, preserving stored rotation without conversion. The accepted counter alone determines storage identities `100001 + accepted`; insertion must not replace an old owner. Each reserved entity receives TreeComponent, ConvertedPreplacedTree, preplaced TreeAnimation, TreeMoveBend and TreeAmbience. SpawnPrefabCmd retains every placement channel and overrides custom0 with the group billboard scale. Existing prefab runtime produces both canonical components on that entity. Asset read/decode failures propagate instead of silently skipping groups.

The converter coalesces events, sets wait count to one, then converts once only after two subsequent no-event updates. It iterates authoritative TreeStorage `(id, Entity)` entries; a dead entity or missing required query component is a skip. Query placement is exactly systems::PrefabInstanceData, not the old aggregate or an inferred TreeStorage position. It reads the actual typed, mapped TerrainRockMaskSsbo from AssetSsboLibrary and retains its name/type assertion and bounds behavior; no empty mask or CPU proxy is created.

The native rejection order is water/garden positive, then the actual rock bit; otherwise add the four path samples left to right with no synthetic initial zero, apply native MAXSS/MINSS unordered policy and positive comparison, and finally perform the original radius-seven occupied-cell query. Rejections emit DeleteTreeCmd; acceptances preserve unique storage identity and original placement's first 24 bytes/name/preplaced flag in TreeEdit::Place. One starting MultiEdit is inserted even for zero accepted trees.

The complete original 216B insert_starting_edit body confirms: assert cursor >= -1, increment cursor, prepend the 40B entry, regenerate identity and clear count58. There are no writes to snapshot references/indices; the existing shared helper already matches this behavior.

## Original occupied-cell oracle

`original_wall_probe.py` runs the untouched entire 837B native callback plus original AABB, grid-coordinate, cell-index and cell-center helpers in Unicorn. Only the imported CRT floorf boundary is modeled using the host math dependency. Assertions, iteration, clipping, expansion, strict comparison and occupancy reads are original instructions. All 168 fixed/random f32 fixtures matched the independently projected policy; 60 reject. Full inputs and native outputs are in `original-wall-fixtures.json`.

The query clips the circle AABB against world bounds, floors grid coordinates, clamps lower coordinates only against zero and upper coordinates only against 63, and tests cell centers strictly inside radius + half-cell diagonal. A selected nonempty cell rejects even without a segment geometry record. It must not call the other public query's exact segment-distance/sort/dedup behavior. Eight native boundary vectors are added as candidate Rust regression tests, including radius-zero equality, world edges, out-of-world and an AABB candidate excluded by the circle test.

## Files and integration ownership

The 15 files are enumerated in candidate-checkpoint.json: resource/system prefab definitions, prefab runtime, typed mapped rock sampling adapter, six tree-tool readers, forest tree animation, save snapshot capture, producer, converter and bounded WallSpatialHash method. Existing component reexports and wren queries automatically resolve to the restored canonical placement type. No Cargo dependency is added; mapped sampling reuses core's existing bytemuck and rock bit sampler.

Central owner must replace the shared run_game OnEnter registration with Update/in_state(Game) and register the converter against the same canonical SpawnPreplacedTrees event. This shared file is intentionally absent from this patch. No other agent owns any file in the candidate list; independent_verification owns collision/world.rs and decorator_archivist.rs, neither touched here.

## Verification limits

Candidate files pass rustfmt parsing and git apply --check; 168 original machine-code fixtures pass. This is not a Cargo or App execution result. Remaining integration validation must cover actual App scheduling, deferred prefab insertion before the converter's delayed query, the real mapped buffer owner, event lifecycle, DeleteTreeCmd replay and history/save capture. The Game-state gate and converter registration are required before calling the complete App chain closed. T13 is excluded by user instruction.
