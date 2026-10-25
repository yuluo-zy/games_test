# tg-tree notes

Reconstruction of Tiny Glade's tree-tool (B package `system-tree`) on the ORIGINAL dependencies (see Dependencies). Evidence docs:
`evidence/multi-baseline/tree-process-cmds-analysis.md` (A), `tree-tool-rest-analysis.md` (B),
`tree-tool-extra-analysis.md` (C). Build/test: `CARGO_TARGET_DIR=/tmp/tg-standalone-target cargo test -p tg-tree`;
demo: `cargo run -p tg-tree --example tree_demo`.

## Implemented (module -> source VA -> doc)
| Module | Function | VA | Doc |
|---|---|---|---|
| storage.rs | TreeStorage get/insert/remove (`next_index=max(next,idx+1)`) | 0x140a38460/560/5a0 | A "TreeStorage 布局" |
| process_cmds.rs | process_tree_cmds (Move->Insert->Delete) | 0x142267430 | A |
| despawn.rs | despawn_trees (f32 < 0.01) | 0x1422686d0 | B §1 |
| replay.rs | replay_tree | 0x141bc34f0 | B §2 |
| recalc.rs | recalc_tree_position_on_terrain_height_change | 0x142268dc0 | B §3 |
| bend.rs | tree_move_bend, time_continuous_lerp | 0x1422689c0 / 0x140c904e0 | B §4 |
| reset.rs | `fn reset_tree_tool(ResMut<TreeStorage>)` | inlined, run_unsafe 0x141bb19d0 | C §2 |
| convert.rs | convert_preplaced_trees: delay state machine + gate only | 0x14226be70 | C §1 (low evidence) |
| plugin.rs | `TreeToolPlugin` (events, TreeStorage resource, system registration, `GameSystemSet` UNKNOWN) | plugin build 0x140650520 | B §2, C |
| resources.rs | ActiveSession / GameTime / WaterRaster / TerrainHeights mirrors of resources owned by other modules | n/a | A "系统角色" |

## Unknown / not built (marked `UNKNOWN(evidence: ...)` in code)
- Insert: name matching against .data prefab names; whether deny_load/is_corrupted block the spawn
  (we only feed them into sound `flag`, following the old draft, and gate flags via is_dragging);
  low32 prefab index of TreeStorage entries (`PREFAB_INDEX_UNKNOWN`; Move looks up by entity);
  InsertTreeCmd @+0x28/+0x30/+0x40; SpawnPrefabCmd @+0x10 (entity stored as a harness hypothesis).
- Insert replaced-slot DespawnPrefabCmd: payload is the packed 8-byte insert return value (literal
  evidence), meaning unclear.
- Move: inverse_lerp use of PrefabInstanceData+0xc (sound height passes it through raw); behaviour when
  instance missing (skipped). Delete: in_water not sampled (unevidenced), sound skipped w/o instance.
- TreeAnimation initial value (1.0 assumed) and who decays it (outside module; demo harness does).
- replay: TreeEdit serde layout, divisor semantics, NaN/zero-divisor behaviour of the 0.05 clamp.
- recalc: +-65 semantics. bend: 3.22 origin, bend_step write when dist<0.1, int truncation.
- convert: everything beyond state machine/gate (arg2 event type, which samples form the gate, History).
- GridWaterRaster texel mapping is an assumption (130.0 world size is evidenced).

## Tests -> evidence
- storage: next_index max rule, insert-returns-old, packed value (A TreeStorage).
- process_cmds_tests: order Move->Insert->Delete, sound kinds 0/1/2, in_water `0.0 < sample`, dragging
  `flags&3`, missing-tree log string, Delete marks + no DespawnPrefabCmd, replaced-slot despawn (A).
- despawn: strict `< 0.01`, NaN, marker required (B §1). replay: variants, elapsed==0 emit, 0.05 clamp,
  `delay<elapsed` then `+=delta`, descending removal (B §2). recalc: event gate, |x|,|z|<=65, write only when
  changed, max(.,0) (B §3). bend: rate 5, dist>=0.1, 3.22, min(dist,5) (B §4). reset/convert: C §2/§1.

## Dependencies (real crates, no re-implementation)
Versions are the workspace pins, taken from skeleton/B/Cargo.lock / crates/systems/tree-tool/Cargo.toml.
- `bevy_ecs =0.16.0`: Event/Component/Resource derives, EventReader/EventWriter, Commands, Query, Res/ResMut.
  All systems are genuine system fns with the parameter lists from the analysis docs.
- `bevy_app =0.16.0`: `Plugin`, `App`, `Update`/`Startup`; tests and the demo drive a real `App`.
- `glam =0.29.3`: Vec2/Vec3. `fastrand =1.8.0`: random tree prefab pick (global RNG, as in the original).
- `log =0.4.27`: the two evidenced log messages (Move missing-tree, insert replaced-slot) via `log::warn!`;
  tests capture them with a test logger. Level is UNKNOWN (warn assumed).
- Not used: `bevy_state`, `lerp`, `itertools` (not needed), `hashbrown` (TreeStorage uses std `HashMap`;
  original uses hashbrown with refreshed random KEYS on reset, not modelled).
- Own code replacing nothing in the above: the tree tool's own resources (`ActiveSession`, `GameTime`,
  `WaterRaster`, `TerrainHeights`) are MIRRORS of types owned by other game modules, kept minimal.

## Bevy 0.16 mapping / deviations
- `process_tree_cmds` is a single system with the 11 documented params (Query<&mut PrefabInstanceData>, 3 readers,
  2 writers + Commands + EventWriter x1, Res<ActiveSession>, Res<WaterRaster>). `random_tree_prefab` and the
  log are called inside; no injected closure/Vec log.
- Spawned entity: `Commands::spawn_empty()` then separate `insert` calls (TreeComponent, TreeAnimation,
  TreeMoveBend, TreeAmbience); `SpawnPrefabCmd.field_10_unknown` = `Entity::to_bits()` (hypothesis).
- `TreeEntry` stores a real `Entity`; `raw()` packs `(entity.index() << 32) | prefab_index`.
  `DespawnPrefabCmd` holds `entity_bits: u64` (raw u64 @+8), since the replaced-slot path sends the packed
  value, which is not a valid `Entity`. The `high32 != 0` filter is literal evidence; with real Bevy the first
  entity of a world has index 0, so that path is skipped for it (tests burn index 0 first).
- Registration (plugin.rs): events + `init_resource::<TreeStorage>`; `replay_tree` `in_set(GameSystemSet)` +
  `chain_ignore_deferred` (evidenced). UNKNOWN: GameSystemSet is a local unit SystemSet (real one in country-core);
  schedule label (Update assumed); the `after` edges process->replay, despawn->process are harness assumptions.
  `reset_tree_tool` is a plain `Startup` system (UNKNOWN: original is `OnExit<AppLoadingState>`; bevy_state and the
  state type are not recovered, no hand-rolled states).
- Resources owned elsewhere (ActiveSession, WaterRaster, GameTime, ReplayBuffer, TerrainHeights) are NOT inserted by
  the plugin; the host/test inserts them.
- `convert_preplaced_trees` stays a pure state machine + gate (ConvertState is a Resource) and is not registered
  as a system (its params beyond the state resource are UNKNOWN).
