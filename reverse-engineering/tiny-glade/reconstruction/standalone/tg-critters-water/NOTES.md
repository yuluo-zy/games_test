# tg-critters-water — notes

Reconstruction of Tiny Glade's `critters` (package `system-critters`) and `water` (`system-water`) modules as real
`bevy_ecs` 0.16 systems on baseline B's own dependencies. No hand-written Bevy / RNG stand-ins: every business
function is a system with the documented parameter list, driven in tests and examples by a real `bevy_app::App`
(`CrittersPlugin`, `WaterPlugin`, `EnvPlugin`).

Build/test: `CARGO_TARGET_DIR=/tmp/tg-standalone-target cargo test -p tg-critters-water` (144 unit + 2 integration tests).
Examples: `... cargo run -p tg-critters-water --example critters_demo -- [frames]`, `--example water_demo`.

Sources: `evidence/multi-baseline/{critters-behavior,critters-extra,water-cpu,water-extra}-analysis.md` ("doc" below:
B/BH = behavior, X = extra, WC = water-cpu, WX = water-extra).

## Dependencies

Versions are those of `skeleton/B/Cargo.lock` / `skeleton/B/crates/systems/{critters,water}/Cargo.toml`, declared once in
the workspace `[workspace.dependencies]` (workspace `Cargo.lock` is pinned to B; this crate only adds `x.workspace = true`).

| Crate | Version | Why |
|---|---|---|
| `bevy_ecs` | =0.16.0 | `Component/Resource/Event/Bundle/SystemSet`, `Query`, `Res`, `Commands`, `EventReader/Writer`, `Local` |
| `bevy_app` | =0.16.0 | `App`, `Plugin` (`CrittersPlugin`, `WaterPlugin`, `EnvPlugin`), `Update`/`Last` schedules |
| `glam` | =0.29.3 | `Vec2/Vec3/Quat` (the same glam the game uses) |
| `perchance` | =0.4.0 | goal systems: `PerchanceContext::uniform_f32` (B "RNG=Perchance"), held in `PerchanceRes`, seeded by `CrittersPlugin::perchance_seed` |
| `fastrand` | =1.8.0 | boid random turns and wren/fish spawn positions (X "RNG 二分"), held in `FastRng` (a `Mutex`: `fastrand::Rng` is `!Sync`) |
| `lerp` | =0.4.0 | `Lerp::lerp` in `random_vel_change` (timer) and `sheep_walk_on_terrain` state 4 (`time_continuous_lerp`) |
| `itertools` | =0.14.0 | `sorted_by` over `IterMut<(Vec2,bool)>` in `spawn_ducks` (doc: `sorted_by<IterMut<(Vec2,bool)>>`) |
| `log` | =0.4.27 | wren tree shake message "Tree has been shaken! Spawning ..." |
| `smart-default` | =0.7.1 | `SmartDefault` for `BrushSizes` (2.0) and `WaterToolConfig` |

Not used (so not depended on): `lazy_static` (only the wren `RAY_DIRS` Once table, which is UNKNOWN/not modelled), `bevy_input`
(dev hotkeys of plan_duck, not modelled), `bevy_state`, `anyhow`, `dolly`, `parking_lot`, `puffin`, `ron`, `country-core`, `utils`, `rhapsody`.
(`perchance` keeps its default `macaw` feature exactly as in B's lock.)

The earlier remark "perchance not vendored / own PRNG, bit-stream differs" is resolved: for a given seed the streams are the libraries' own
(tests mirror a `PerchanceContext`/`fastrand::Rng` with the same seed and compare draw by draw). Which exact seed the game uses is not known.

Local stand-ins that remain (documented deviation): `env::Time` (`delta_secs`, the `f32` at `*Res<Time>+0x28`) and
`critters::Transform` (rotation + translation) because bevy_time / bevy_transform are not direct dependencies of `system-critters`
(the game gets them through country-core). Game-side resources (`TerrainHeights`, `GladeShape`, `RaycastWorld`, `WallSpatialHash`,
`MainCamera`, `CursorTerrainRaycast`, `DemoMode`, `BackupSaveFileKind`) are plain `Resource`s in `env.rs`, installed by `EnvPlugin`
without overriding anything already inserted.

## Implemented (systems and their registration)

`CrittersPlugin` set order (Update): `Plan` -> `Goals` -> `Velocities` -> `Collision` -> `Motion` -> `Gpu`; boid flocks unordered.

| Module | Systems / content | Doc |
|---|---|---|
| `water/raster.rs` | `WorldRaster` (= `WaterRaster`) `Resource`: 512x512 u8 double buffer, `world_to_grid`, `bilinear_sample_at_world_xz` (/255), copy_composite_to_scratch, commit_scratch_to_composite, copy_to_staging (+0x298/0x299/0x29a), +0x29b gate, dirty bitmap | WC 2, WX 1/3 |
| `water/stroke.rs` | zeno command list, write-mode dispatch, env0 max / env1 min / env2 saturating add x16.16 brush, `alpha = cov>>9` | WX 5, WC 2 |
| `water/tool.rs` | system `draw_water(WaterToolInput, DemoMode, BrushSizes, CursorTerrainRaycast, GladeShape, WaterToolConfig, ResMut<WorldRaster>, ResMut<ActiveRasterStroke>, LastDrawWater, EventWriter<WaterDrawEvent>, EventWriter<WaterEraseEvent>)`; helper `draw_raster` Start/Update/Release; `BrushSizes` | WX 1, WC 4 |
| `water/replay.rs` | system `replay_raster(ReplayQueue, EventWriter<WaterDrawEvent>, ResMut<WorldRaster>, EventWriter<RecalculateHeightMapCmd>, Res<ScheduledTerrainReadback>, Res<WaterToolConfig>)`; helper `replay_commands` | WC 4 |
| `water/audio_grid.rs` | system `update_water_audio_grid(Local<WaterAudioGridUpdateState>, Res<WorldRaster>, Res<RaycastWorld>, Res<MainCamera>, ResMut<WaterAudioGrid>, Res<DemoMode>)` with `run_if(resource_exists::<WaterAudioGrid>)`; 24 B cell, `bit = (n+(n+1)*10+1)&63`, 80^2 radius, >0.5 | WX 2 |
| `water/plugin.rs` | `WaterPlugin`: events, resources, `replay_raster -> draw_water -> update_water_audio_grid` (Update), `water_rasterize_from_cpu` (Last) | WC 2 |
| `critters/shared.rs` | systems `smooth_critter_velocities`, `move_critters(Query<(&mut Transform,&CritterLocomotion,&Velocity,Option<&CritterVelocityMultiplier>,Option<&Duck>,Option<&Sheep>,Option<&mut BirdSpots>)>, ...)`, `critters_avoid_collisions` (writes `EventWriter<DespawnDuckCmd>`), assumed consumer `despawn_duck_on_cmd` | B 2 |
| `critters/duck/*` | systems `plan_duck_trajectories`, `move_ducks_along_trajectories(Query<(Entity,&mut Duck,&mut DuckState,&mut Transform,&mut Velocity,&mut CritterVelocityMultiplier,&mut CritterGoal,&mut ObjectGrassPush)>)`, `update_duck_goals`, `set_duck_velocities_towards_goals`, `update_duck_ripples` (-> `DuckRipples`), `spawn_ducks`, `despawn_ducks` (opt-in, see below); `DuckBundle` | B 1, X 5, X 6 |
| `critters/sheep.rs`, `sheep_terrain.rs` | `update_sheep_goals` (eye+fwd*2 / cursor, wander), `sheep_walk_on_terrain` (state 3 `panic!` like the original) | B 3, 4 |
| `critters/boids.rs` | pure `Boid::frame_update`, `random_vel_change(&mut FastRng)`, `BoidFlock::remove` (swap_remove, returns `Entity`s) | X 1.2-1.5 |
| `critters/wren.rs` | system `sim_wren_boids` (cap 80, spawn ranges, keep volume, enable switch, `TreeShaken` burst `floor(u*5+1)`) | X 1 |
| `critters/fish.rs` | system `sim_fish` (25x25 grid, cap 200, 20 attempts, tiers, water bounce) | X 2 |
| `critters/butterfly.rs` | systems `update_butterfly_goals`, `set_butterfly_velocities_towards_goals`, `update_butterfly_states` (gate 4/8 + dwell reset only) | X 3 |
| `critters/firefly.rs` | systems `update_firefly_goals` (bias +0.5 / -1.0), `set_firefly_velocities_towards_goals` | X 4 |

Deviations from "original parameter list": unavailable game services are `Res<...>` stand-ins (see Dependencies); `ButtonInput<KeyCode>` / dev gates of
plan_duck are absent; render/GPU params (SSBO, `GraphicsDevice`, mesh handles, audio) are omitted. `Option<Res<RandomTurn>>` enables the wren random turn.

## Doc inconsistencies / open choices

- **Duck plans**: spawn_ducks (X 5.2) pushes arrival plans and despawn_ducks (X 5.3) departure plans into the same `Vec<TrajectoryPlan>`; plan_duck
  (B 1.1) turns every plan into a *new* duck with `progress = 1.0, state 3` (reverse -> despawns at the start), i.e. an arrival plan produces a duck that
  immediately walks off, and a departing duck spawns another. The 0x30 record has no direction field (`+0x20/+0x24/+0x28` unknown). Followed literally;
  `despawn_ducks` is therefore opt-in (`CrittersPlugin::with_duck_departures`) because it breeds one plan+duck per frame for a duck that stays out of range.
  Plan consumption is also tied to developer hotkeys in B 1.1; here it is unconditional.
- **DuckSpawnLocations `+0x18`**: X 5.1 says `+0x18 = 25.0`; X 5.2 says the spawn timer lives at the resource `+0x18`. Read as one field: the timer starts at 25.0
  (first duck after ~1.7 s with <15 ducks). Any other reading would start at 0.
- **`Sheep` column** in the butterfly / firefly velocity queries (X 3, X 4) is read as the `Butterfly` / `Firefly` component.
- X 3 puts `multiplier += 1.0` in butterfly velocities (not applied, semantics UNKNOWN); X 4 sets `multiplier = 1.0` for fireflies (applied).
- B 1.3 `+0xe4` "second half written when resting" and the wren `sqrt(x^2+y^2)` keep-radius text (xz is used) remain as flagged earlier.

## UNKNOWN (kept as `// UNKNOWN(evidence: ...)`, no behaviour built on it)

- Exact formulas of the game-own `uniform_sample_*` / `golden_spiral` utils (textbook samplers fed with perchance / fastrand uniforms) — B "时序/RNG". (The RNG bit-stream itself is now the real libraries'.)
- `CritterLocomotion` species mapping; only the proven per-value predicates exist (`==2` rock mask, `==4` skip water sample, `==3` skip PublicWalls, mask 99 -> {0,1,5,6}) — X 6. The rock-mask / water double sample result handling and BirdSpots in `move_critters` are not modelled.
- `CritterVelocityMultiplier` writes (`Query+0x14` env u32), `retention_per_second` initial value, duck locomotion value (all supplied by config) — B 6.
- Forward axis for `Quat::from_rotation_arc` (assumed +Z, `FORWARD_AXIS`) — B 2.1; trajectory curve smoothing (polyline used) — B 1.2.
- Duck: DuckState 0, the second timer half written when resting, 4-sample >=0.1 check offsets, plan dev-hotkey gate (KeyCode 0x1f/0x27, `_DEV_OVERRIDE_IS_DEMO_MODE`), ripple record layout / filter closure, DespawnDuckCmd consumer, DuckSpawnLocations `+0x18 = 25.0`, `sorted_by` key, despawn `DuckState==2` thresholds and 4+1 sample offsets.
- Sheep: `fVar13` (follow-close distance), wander radius (config), timer-expiry branch, set_sheep_velocities / audio / unstuck / floatie (not read); `time_continuous_lerp` rate in terrain state 4.
- Boids: scope of the acceleration clamp (applied per component to the velocity delta — interpretation), RAY_DIRS table + wren raycast steering, BirdSpots/FreeSpot/BirdHaven, cone half-angle / timer range of `random_vel_change`, fish wall repulsion / cursor+camera escape, fish `is_corrupted` tier meaning, fish initial velocity y.
- Butterfly state enum + both jump tables, `multiplier += 1.0`, bias-pair component; firefly direction formula degenerates at bias -1 (applied literally).
- Water: original `WorldRaster::new` fill block (0x4000B pair), dirty tile size (8 inferred), who sets `+0x29b`, zeno stroke width / coverage scale (capsule AA model + `255<<9` assumed), `resample_and_smooth`, easing/Curve in replay, which selector value `copy_to_staging` callers pass (Scratch live / Composite on release assumed), sub-cell offset of the bilinear sampler (`p = uv*512`), ActiveRasterStroke full layout, circle radius constant 0x142a16cb8, WaterMask/ExcludeClutter SSBO fields, WaterAudioGrid resource layout, audio distance dimension (xz used).
- Whole-frame ECS schedule order (documented numbering followed, see `CrittersSet`); `startup_critters` (24 butterflies / 96 fireflies), butterfly/firefly/duck audio, GPU upload systems and the dev-hotkey (`ButtonInput<KeyCode>`) paths of plan_duck are not ported.
- `Time` / `Transform` are minimal local types (see Dependencies); the original types live in bevy_time / bevy_transform, which `system-critters` does not list as direct dependencies.
- Consumer of `DespawnDuckCmd` (`despawn_duck_on_cmd` just despawns the entity); initial `DuckSpawnLocations` timer (25.0 from the `+0x18` tail is assumed to be the same field the spawn timer accumulates into).
- Doc inconsistencies (not resolved, see below): duck spawn/departure plans vs plan_duck, the `Sheep` column in the butterfly/firefly velocity queries.


## Test-to-evidence map

| Test (module) | Evidence |
|---|---|
| `raster::origin_maps_to_cell_256`, `bilinear_*` | WC 2 (130 / 0.5 / 512), WX 3 (clamp 511, /255) |
| `raster::double_buffer_copies`, `staging_publish_and_selector`, `gpu_upload_gated_by_reupload_flag` | WC copy_* / copy_to_staging / water_rasterize_from_cpu |
| `stroke::dispatch_matches_binary`, `write_formulas`, `coverage_alpha_and_brush_fixed`, `commands_first_move_rest_line` | WX 5, WC rasterize_stroke |
| `tool::tool_dispatch`, `brush_clamp_and_reset`, `full_stroke_lifecycle_*`, `tiny_move_is_ignored` | WX 1, WC BrushSizes / draw_raster |
| `replay::*` (incl. live == replay for weight 1) | WC replay_raster |
| `audio_grid::frame_bit_formula`, `counters_*`, `window_fills_over_64_frames_*` | WX 2 |
| `shared::*` | B 2.1-2.3 |
| `duck::movement::*`, `plan::*`, `goals::*`, `velocities::*`, `ripples::*`, `spawn::*`, `critters::plugin::*` (full-frame pipeline through `CrittersPlugin`) | B 1.1-1.5, X 5, X 6 |
| `sheep::*`, `sheep_terrain::*` | B 3, B 4 |
| `boids::*`, `wren::*`, `fish::*` | X 1, X 2 |
| `butterfly::*`, `firefly::*` | X 3, X 4 |
| `components::locomotion_mask_selects_0_1_5_6` | X 6 (`99U>>(v&31)&1`) |
| `rng::*` | perchance 0.4.0 / fastrand 1.8.0 streams are the libraries' (mirror comparison) |
| `env::*`, `water::plugin::*` | game-side resources, `WaterPlugin` end-to-end (stroke -> upload -> audio grid) |
| `tests/determinism.rs` | same seeds -> identical App state; perchance vs fastrand seeds affect only their own users |
