# M01 shape-edit UX — implementation status (owner: impl agent, 2026-10-28)

Spec: `work/m01-scratch/shape-edit-spec.md`. Only READY sections hit.

## Command evidence
- `cargo test -p country-core --lib adjust_wall_height` → **55 passed** (`impl_adjust_wall_height.log`).
- `edit_arrow` → **5 passed**; `ui_move_shapes` → **2 passed**; `ui_edit_circle` → **1 passed**.
- `cargo check -p country-core --tests` → exit 0 for all of the above.

## sign-off (adjust trio)
- **§1.1 `ui_adjust_wall_height` = TESTED** (independent review): transmission/authority/history + height-signifier picture
  PASS; canonical `HistoryCommand::UNDO/REDO`; real replay. §1.2/§1.3 PASS. Trio TESTED, scope = in-source real CPU chain
  (not window/OS input, GPU picture, new-process save/restore).

## §3 `ui_move_shapes` — PARTIAL (case1+case2+case3 done; move_gizmo full body pending)
File `crates/country-core/src/systems/ui_move_shapes.rs`. 30-param PDB signature; `MoveShapeCache`/`MoveShapeEntry`
(map1/map3 + state/activation/shape_rec/variant/curve/committed); `OnShapeMoved` event; `0x142ab7514` dispatch.
- **case3 commit = IMPLEMENTED + unit-tested** (`case3_commit_updates_wall_and_records_undoable_history`):
  `update_wall_state_curve(curve)` → `CompoundEditBuilder::new_edit_sequence → next(ChangeSupports{wall,supports=1}) →
  build` → `History::add` (EditType::Wall, type 7) → emits `OnShapeMoved` (0x30 row) + `ShowPinnedDecoClutter` (0x18 row).
  **Corrected from StateChange → ChangeSupports** per analysis (variant 0x8000…000b); test asserts ChangeSupports. 5 tests.
- **case1 drag = IMPLEMENTED + unit-tested** (`case1_drag_writes_working_record_and_decorator_triples`):
  `archive_current_decorators_in_storages_with_resources(...,"move_shape")` → `decorator_cull_allow_wall` →
  `wall_decorators_mut(wall)` triples `{id, hi(u18)-cx, u24-cz}` → `ShapeParameters::center()` +
  `TerrainHeightsData::sample_at_world_xz` anchor → writes working record: `+0x00 cap/0, +0x08 ptr, +0x10 len,
  +0x18 Δscreen (ray∩y=anchor.y), +0x20..0x3c 28B SP copy, +0x3c anchor.y`; Vec triples owned on the entry.
  (6 tests in module.)
- **move_gizmo_and_trigger_vol**: `camera_rotate_pressed = actions.digital[2].phase < 3` implemented + tested;
  full 8-param body (Named `move_shape`/`move_shape_outline` axis records + activation `Points` + box_trigger_volume
  TriangleMesh widget + hover) NOT yet written. ActionSetStates(#15) slot 2 = RESOLVED.
- **case2 hover-snap = IMPLEMENTED + unit-tested**: added `spatial: Res<WallSpatialHash>` seam (CF: native
  `param_2[4]` ≈ WallSpatialHash; PublicWalls not extended); `SnapProposal`/`nearest_proposal` now `pub(crate)`;
  `hover_snap` = `nearest_proposal` → roof-ridge offset (tag2→-6.0 / tag1→-8.0) → `DebugSphere`+`DebugLine`
  signifiers (NO InnerWalls). Test `case2_hover_snap_finds_candidate_near_wall` builds a spatial hash and asserts
  a candidate. (5 tests pass in this module.)
- 5 unit tests pass. Status: **PARTIAL (case2 + case3 + digital[2]/move_gizmo TESTED-unit) — not module-TESTED, not scheduled.**

## §6 `ui_edit_circle` — PARTIAL (S0/S2-clamp/S3/S4/S5-threshold done; S1/S2-proj/S5-history pending)
File `.../circle/ui_edit_circle.rs`. 30-param PDB signature; `AnimatedCircleSignifierResource` (#[repr(C)] size 64);
`CircleGrabParams`/`CircleSignifierParams` Locals (empty, fields/default UNKNOWN); kept `show_full_circle_signifier`.
- **S0 gate** (mode>5 tail; only Editing(Wall/Roof) proceed) applied.
- **S2 clamp** `clamp_circle_radius` (0.9/8.0) + **S5 threshold** `circle_radius_changed` (|Δr|>0.01, NaN→changed) tested.
- **S3 fit-search** `GladeBorder::search_lerped_shape_that_fits_into_glade` (0x140ad2ce0) implemented + tested: lerp
  `lo→hi` over `t=step/20` (20 steps), returns largest shape inside the glade; Circle via `Circle2d::lerp`
  (native `FUN_140c98170`); Rectangle lerp deferred (§10-12). (2 glade_border tests)
- **S4 apply** `apply_circle_radius` (`Circle2d::new(...).as_curve2(0.2)` → `update_wall_state_curve` →
  `update_wall_state_enclosure` circle snapshot) tested.
- **PENDING (evidence)**: S1 wall source (`UiState`→wall: WallTriggerVolume has NO wall id; UiState has no wall
  accessor; WidgetId→entity→WallEntities mapping unresolved). S2 projection **AVAILABLE**
  (`utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray`) — needs S1 first. S5
  `CompoundEditBuilder/ConcurrentEdits` history wiring.
Status: **PARTIAL (S0/S2-clamp/S5-threshold/S4 unit-tested)** — not module-TESTED.

## §7 `signifier_edit_circle_arrow` — DONE (geometry per flat_arrow evidence; 5 tests)
System/Transform/layout/registration implemented+tested (5). Geometry now uses **`SignifierMesh::Named("flat_arrow")`**
(builder tag 0 → Named asset via `FUN_14092d570`; missing asset panics) + `vertex_color_shader`, record 0x30
`{1,1,Quat,Vec3}` carried by closure; name = `edit_circle_{WidgetId:?}_arrow` with WidgetId at resource `+0x00`.
Rectangle/other lever not applicable. Not yet scheduled (hook step).

## Remaining (await analysis)
- §3 case1 drag — RESOLVED to implement: step5 `param_6+0x00/+0x08/+0x10` = `{capacity, pointer, length}` Vec
  header (element 12B `{u32, f32, f32}`), NOT 6 f32; helpers (archive/cull/wall_decorators_mut) + projection
  formula resolved. Next implement.
- §6 — S1 needs the `edit_circle_wall_{WallId}` widget geometry/params (flow known: iterate PublicWalls → register
  widget → match UiState +0x18/+0x38/+0x28 → WallEntities→Query<&WallTriggerVolume>.is_valid); S5 history wiring
  (`CompoundEditBuilder/ConcurrentEdits`) pending. S2 projection available; S3 `search_lerped…` done.
- move_gizmo — needs full 8-param signature (only given: #5 WallId, #7 signifier_y, #8 stream; #1–4,#6 unspecified);
  body (`box_trigger_volume`/TriangleMesh widget/axis records `move_shape`/`move_shape_outline`/activation 0x28/
  tail anchor=xvy(center,terrain)/rotation=shape) otherwise specified.
- §2/§13 — BLOCKED on PENDING cache-type consumers/producers.
- §4/§5/§8–§12/§14 — analysis paused (scope shrink).
- Step 6/hook: after the 4 partials close, hook them into `install_toolbar_shape_dependencies` at a/c group relative
  positions (no full tree rebuild, no shell deletion) + one real schedule test.

## Checkpoint 2026-10-28（用户要求暂停并保存进度）

> 上面 "Remaining" 一节部分已被本检查点取代（case1 已完成；move_gizmo 8 参已确认）。

**已完成并签核**
- adjust 三件套（`ui_adjust_wall_height` + `_mode` + `history_record`）：**TESTED**（真实输入链、权威、History、真实 UNDO/REDO、空 signifier 画面）
- `signifier_edit_circle_arrow`：**PASS**（`Named("flat_arrow")`、WidgetId 名、组件注册；5 tests）
- `ui_move_shapes`：case1（drag，写 shape_rec / Vec 头 / 装饰三元组）、case2（hover-snap 接 `WallSpatialHash`）、case3（`ChangeSupports` + `History::add(7)`）均实现；6 tests
- `ui_edit_circle`：S0 门控、S2 clamp、S3 `GladeBorder::search_lerped_shape_that_fits_into_glade`、S4 应用、S5 阈值已实现并测试
- `move_gizmo_and_trigger_vol`：`digital[2].phase<3` 门控、smoothstep 已实现

**已知未闭合（暂停点）**
- §3 的 9 个 writer 中目前只有 `OnShapeMoved`、`ShowPinnedDecoClutter` 真正 emit，其余 7 个仅签名（审查线结论）
- move_gizmo 完整 body（8 参已确认；字面规格见 spec §2）
- §6 S1 widget 几何（已定：有效墙 `TriangleMesh(default_tris)`，否则 `Empty`）与 S2 Δr、S5 `MorphPremadeShape` 尚未接线
- 四项均未挂进 `install_toolbar_shape_dependencies`，尚无真实调度测试
- 真实窗口/OS 输入、GPU 画面、保存/新进程恢复、性能均未验收

**下一位接手入口**
- 规格：`shape-edit-spec.md`（§1.1/§2/§3/§6/§7）、`shape-edit-registration.md`
- 审查：`review/shape-edit-review.md`
- 证据方法：PDB `S_INLINESITE`（`llvm-pdbutil dump --symbols --modi=519`）定位内联读取；`flat_arrow`、`search_lerped` 已定位

## Logs
`impl_{adjust_wall_height,edit_arrow,ui_move_shapes,ui_edit_circle}.log`.
