# WP20 — country-core camera / remaining-UI / time-weather / backend-input systems

Tiny Glade original reverse-engineering, **version B**
(EXE sha256 `cf048e27…`, PDB sha256 `2d9686b8…`), target
`ljxsj_92385`.

Method per project workflow: **(1) analyze business logic** (Ghidra decompile
+ PDB skeleton evidence + call-chain), **(2) organize the contract** (O / I / U
+ typed gaps), **(3) implement Rust from the PDB source skeleton**. Machine-code
diff is used only to adjudicate numeric/boundary ambiguity — never as the
implementation method.

## Deliverables

- `contracts/*.md` — four business-logic contracts:
  - `camera.md` — `systems::camera_rig/*` + `main_camera` + `look_at_camera`
    (0x4e0 B MainCamera, dolly bodies, mode dispatch, snapshot).
  - `input-backend.md` — `systems::backend::input/*` (record/replay FSM, input
    translation, cursor position).
  - `time-weather.md` — `time_of_day`, `time_update`, `weather_system`,
    `world_raster`, `timelapse_replay`, `mode_manager`, `prefab`.
  - `ui-misc.md` — the `ui_*` + `ui_clutter/*` systems and the misc systems.
- `src/` — skeleton-aligned Rust (`systems/…`), standalone offline `cargo check`
  gate (`Cargo.toml` + `src/lib.rs`). Reused deps / wp15-16-owned types are
  declared as integration markers (`src/reused.rs`, `src/input.rs`,
  `src/resources.rs`, `src/utils.rs`, `src/asset_libraries.rs`,
  `src/systems/ui.rs`, `src/systems/backend.rs`) — the integration owner maps
  them to the real country-core modules.
- `analysis/` — 86 per-function Ghidra decompiles (B patched EXE, shared
  read-only worker).
- `manifest.json` — PDB procedure/type manifest for the 44 units.
- `evidence.json` — tool calls, decompiled functions, key O findings, gaps,
  gate.

## Coverage (44 units)

`backend/input/*` (7), `camera_rig/*` (5), `main_camera`, `look_at_camera`,
`ui_2d_follow_mouse`, `ui_camera_mode_gizmos`, `ui_clutter/*` (7),
`ui_edit_focus`, `ui_edit_freehand_walls`, `ui_focus_mode_shape_height`,
`ui_move_freehand_walls`, `ui_move_shapes`, `ui_raise_shape`, `time_of_day`,
`time_update`, `weather_system`, `world_raster`, `timelapse_replay`,
`mode_manager`, `prefab`, `arrow_gizmo_animation`, `brush_preview`,
`clear_canvas`, `despawn`, `mesh_follow_terrain`, `mod_key`, `utils`,
`utils_intersect`. `collision` is wp16-owned and only referenced.

## Reconciliations

- Camera bodies reuse the prior machine-code-verified
  `reconstruction/camera` (tiny-glade-camera-b) business logic and the locked
  `dolly 0.6.0` dependency (skeleton/B Cargo.lock checksum
  `1057c2875c640d0d6c961e0bfe5c31d03010d0e72ea96f6c40fffe6b7aebf235`).
- Input backends consume the wp15 `input/*` types
  (`InputRecordReplayState` mode tags, `InputStream`, `InputFrame` 0xd0);
  UI systems consume the wp15 `systems::ui` (`UiState`,
  `UiAppendSignifier`); misc systems consume the wp6 `Transform` and wp16
  asset libraries. No duplication of wp15/16-owned logic.

## Status

`implemented_unverified`: the offline `cargo check` gate closes the
skeleton-aligned surface (0 errors / 0 warnings); it is not a claim of
original-behaviour equivalence. Remaining typed gaps are enumerated in the
contracts and `evidence.json`.
