# WP15 Contract — `systems::ui` (UI surface, egui-wiring boundary, signifiers)

Source: `skeleton/B/crates/country-core/src/systems/ui/{mod,ui_update,hover_intersection,debug_visualize_volumes,resources/*,signifiers/*}.rs`
Evidence: Ghidra decompiles in `analysis/b5_ui_state/`, `b6_widgets/`, `b7_signifiers/`.
Toolbar/tool state lives in the **toolbar crate** (out of scope); only the country-core UI surface is recovered here. egui itself is a reused dependency — no re-implementation.

## Observed (O)
- **`UiSystems::add_ui`** @0x140945c60 (App plugin method): schedules UI systems in `Update` (`chain_ignore_deferred` + `in_set_inner`), inserts `UiState`, `UiNavState` (1e9-ns timeout), `UiSignifierStream` (0x170 B), registers events `PlayUiSound` + `PlayPhotomodeSound`.
- **`GlobalId` / `WidgetId`**: u64 name-hash identity; `WidgetId::from_name` @0x14092df30 = `utils::calculate_hash(name)`.
- **`UiState`**: widget registry `RwLock<HashMap<WidgetId, Arc<WidgetParams>>>` (lock word @+0x60; each `WidgetParams` 0xb0 B / Arc 0xc0). Context bytes: hover @+0x18, click @+0x19, active/hover id @+0x38, intersection payload @+0x28..+0x58.
  - `add` @0x140922fb0 (54 callers), `add_with_id` @0x14092e3e0, `widget` @0x14067dd80 (1177 callers), `active_widget_state` @0x140002450.
  - `clearing_canvas` @0x14092e390 / `_w_layer` @0x14092e340 → `_w_layer_and_sound` @0x14092df50 (clearing-canvas widget volume **130×130** = `0x43020000,0x43020000`).
- **`UiWidgetContext`**: `clicked` @0x14092e5f0 (`hover>1 && click<2 && click&1`), `hovered` @0x14092e620 / `is_active` @0x14092e680 (`hover!=0 && id match`), `intersecting` @0x14092e640 (payload copy or tag-2 None), `id` @0x1406868b0; `HoverState::is_some`, `DragState::{is_some,is_started,is_finished}`, `ActiveWidget::is_dragging`.
- **`WidgetParams`** = 0xb0 B: `new` @0x1409a8e10 defaults u32 `{0,8,0,1,0,2,3,3}` at `{0,8,0x10,0x18,0x20,0x30,0x48,0x60}`, 16 B sound pair @0x78..0x88, 32 B volume @0x88..0xa8, `u16 @0xa8 = 0x101`; setters `sounds` @0x1409a8e80 (96 B @0x18..0x78), `occlude_by_world` @0x1409a8ee0 (byte @0xa8), `no_history_recording` @0x1409a8f10 (bit0 @0xa9), `allow_negative_distance` @0x1409a8f40 (bit2 @0xa9), `signifier` @0x1422bc980.
- **`UiSounds`** = 4 slots × 24 B; slot map `on_hover_start`→0-2, `on_drag_start`→3-5, `on_drag_release`→6-8, `on_click`→9-11; `empty`/`boop` @0x1409a8fb0.
- **`WidgetVolumeShape`** = 32 B `{tag u8@0(=4 clearing_canvas), layer u8@1, size Vec2@0x10=(130,130)}`; `clearing_canvas` @0x1409a9110.
- **`hover_intersection`** @0x1408383a0: `par_chunk_map` ray-vs-volume over `(WidgetId,&WidgetParams)` → `Vec<WidgetIntersection>` + `Vec<(&WorldCanvasLayer, WidgetIntersection)>` sorted by layer + distance.
- **`ui_update`** @0x140b2de60: per-widget hover/click/drag state machine (HoverState/DragState), `get_ui_volume`, `enter_hover`, clears intersection list each frame.
- **Signifiers** (`signifiers/`):
  - `SignifierEntityBuilder` = 0x90 B; `new` @0x14089d9b0 default shader `"vertex_color_shader"` (19 chars @0x60..0x70), position Vec2 @0x7c, defaults `{0,1.0f64,0x100 u16@0x40,0.2f32@0x78}`; builders `position` @0x14089dc00, `set_ui_pass` @0x14089dad0 (byte @0x8a), `custom_shader` @0x14089db00, `rotation`/`scale`/`animation_duration`/`minimum_lifespan`/`animation_easing`/`parent_to_widget`.
  - `build` @0x14089dd50 mesh selector tag: 0 = named (`AssetMeshLibrary::try_get_handle_by_name`), 1 = transient (`AssetMeshLibrary::add` + thread-local `Cell<(u64,u64)>`), 2 = `utils::geo::mesh_from_pts`; spawns signifier entity + `SignifierAnimation`.
  - `SignifierFactory` = 40 B (44 callers); `UiSignifierStream` = 0x170 B lock-free channel (`write` @0x140b2cf90 copies 0x28 B records; read cursor @+0x160, write counter @+0x168); `signifier_create` @0x140b2cfa0 drains → spawns; `signfier_mark_for_delete` @0x140929160 + `despawn_marked` @0x140929a90; `signifier_animation` @0x140a69c30 + `signifier_animation_t` @0x140a69340.
- **`Slider3d`**: bevy `SystemParam` (646 `new_archetype` callers); `Slider3dState::project_cursor_onto_line` @0x140a9bd40; `update<C,F>` @0x140b1cc80; `into_drag_state` (55 callers).
- **`debug_visualize_volumes`** @0x1414558c0 / `debug_visualize_roof_uvs` @0x141454e60 — triangle-list debug draw.

## Inferred (I)
- `DigitalActionState`-style hover/click byte meanings (`hover>1` = intersecting).
- `UiFocus` variant set (`None/Toolbar/Mei/World`).
- `Slider3dState` fields.

## Unknown (U / typed gaps)
- Exact `WidgetParams`/`UiSounds` slot payloads; `UiNavState` layout; `HoverIntersectionCache`/`WidgetIntersection` field layouts; `SignifierFactory` 40 B layout; `ui_update` full state machine; egui/toolbar bridge details.

Reused deps: bevy_ecs (World/Commands/events), bevy_tasks (par_chunk_map), glam, puffin. UI *draws* go through the render graph (wp4 asset libraries + final graph) — see `evidence/render-passes/INDEX.md` ui family (`_ui_2d`, `_egui`, `_text`, `_custom_cursor`, `_dashed_line`, `_timer_circle`).
