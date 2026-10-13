# WP15 — country-core UI / input / startup / app-shell (version B)

Business-logic-first reconstruction of the Tiny Glade `country_core` **non-render**
domain: `input`, `startup`, `systems::ui`, app-shell root state
(`app_loading_state`, `game_config`, `game_profile`, `game_key_bindings`,
`meta_commander`), `geometry`, and `utils`. Target version = **B**
(EXE sha256 `cf048e27…`, PDB sha256 `2d9686b8…`).

## Deliverables
- `contracts/{input,startup,ui,app-shell,geometry-utils}.md` — business-logic
  contracts (Observed / Inferred / Unknown + typed gaps).
- `src/…` — skeleton-aligned Rust (paths mirror
  `skeleton/B/crates/country-core/src/`):
  - `src/input/{mod,action_set,input_stream,panic_repro_state}.rs`
  - `src/startup/*.rs` (31 files + `glade_themes/{mod,flowery,olden}.rs`)
  - `src/systems/ui/{mod,ui_update,hover_intersection,debug_visualize_volumes,resources/*,signifiers/*}.rs`
  - `src/{app_loading_state,game_config,game_profile,game_key_bindings,meta_commander}.rs`
  - `src/geometry/{cube,instanced_wall,plane,sphere}.rs`
  - `src/utils/{mod,audio_utils,child_wall,debug_viz,dirty_tiles,geo,load_json,project_cursor_onto_ray}.rs`
- `Cargo.toml` + `src/lib.rs` — standalone offline `cargo check` gate
  (**0 errors**, edition 2024 / rust 1.86; dep versions match the original
  Cargo.lock baseline).
- `evidence.json` + `README.md`.
- `analysis/` — the raw Ghidra decompiles per batch (`b1_input` … `b11_startup3`)
  that back the contracts.

## Highlights recovered (O evidence)
- **Glade bounds**: release clamp half-extent `f32 0x42c28200 ≈ 97.2538`;
  demo-mode circle radius 65 (`4225`); `is_within_glade_shape`/`clamp_to_glade_shape`.
- **GameConfig::upgrade** migration (int@0x80 / float@0x84 / flag@0x88 / float@0x8c).
- **GameProfile** = 0x208 B; default key-binding code table (26 slots + mouse `{1,2,1}`);
  atomic `write_profile_via_tmp`.
- **MetaCommander**: 0x38-B `MetaCommand` queue; full `commander_update` tag dispatch
  (session load, time strategy, profiling, GPU/memory logs, despawn, quit).
- **UiState / WidgetParams / UiSounds / WidgetVolumeShape / SignifierEntityBuilder**
  layouts + the clearing-canvas 130×130 widget.
- **startup**: `grid_gen_cw/ccw`, water pass selectors `0xf/0xe` + winter ice,
  9 boundary-wall brick sources → 7 draw proxies, flowery/olden themes
  (flower kind 0xe, olden name hashes), `init_clearing`/`uninit_clearing`,
  `load_game_session`, `add_ui`.
- **utils**: `DirtyTiles` 64×64 grid, `ChildWall` 16 B, `AudioLoop`,
  `project_onto_ray` 1e-10 clamp.

## Reused-deps boundary
bevy_app/bevy_ecs/bevy_tasks, winit, egui, glam, serde/ron, puffin, rhapsody,
libfmod, tempfile, hotwatch/notify, `conservative_line_raster` are **reused** —
only game-owned logic is reconstructed; their surfaces are typed gaps. The
render-domain `Mesh` is a placeholder (owner re-points to wp4
`render::mesh::Mesh`); `Transform` (wp6) and `CameraCursorRay` (wp8) are not
redefined.

## Remaining gaps
- Full `ui_update` hover/click/drag state machine, `add_game_startup_systems`
  registration order, `load_game_session` deep wiring, `assign_key` conflict
  resolution, keycode→string tables, `calculate_hash` exact algorithm, and all
  field layouts marked `Gap` (see contracts). All `implemented_unverified`; no
  runtime/integration acceptance.
