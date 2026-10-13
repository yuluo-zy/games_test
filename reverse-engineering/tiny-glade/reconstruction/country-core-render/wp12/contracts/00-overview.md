# WP12 概览 — country_core::systems::wall（墙系统）

版本 B（target `ljxsj_92385`；EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。
恢复方法：业务逻辑分析（B 反编译 + PDB 骨架证据 + 调用链）→ 整理契约 → 骨架对齐 Rust 实现。
机器码差分仅用于关键数值/边界裁决，不作实现手段。

## 范围

`skeleton/B/crates/country-core/src/systems/wall/` 的 57 个源码单元（PDB 逐条 bundle：
`*.dependencies.json`、`*.pdb.jsonl.gz`、`*.enriched.json.gz`、`*.structures.json.gz`）。
未包含 wall 模块自身的 `mod.rs`（skeleton 无该单元）。

| 组 | 单元数 | 系统/过程 |
|---|---|---|
| 核心生命周期（root） | 13 | calc_wall_bend, delete_wall, draw_wall, finish_wall, merge_walls, set_wall_style, wall_splitter, wall_highlight, upload_instanced_wall_ssbo, deferred_wall_history, structure_outline, ui_continue_wall, ui_draw_wall |
| adjust_wall_height/ | 5 | adjust_wall_height_impl, ui_adjust_wall_height_mode(+clamp/history), ui_adjust_wall_height, adjust_wall_ui_mode, signifier_animation |
| eraser/ | 4 | animate_wall_bulldoze, ui_bulldozer, wall_eraser, archivist |
| shapes/ | 21 | mod(+add_wall_shape_ux), calc_shape_height, shape_placement, shape_placement_signifiers, utils/shape_signifier_lerp, circle/6, rectangle/10, ui_modes/2 |
| snapping/ | 7 | mod(SnappingMode), snapping_init, snapping_maintain(×3), snapping_to_ends, snapping_to_segments, snapping_audio, telegraph_walls_are_merged |
| valid_enclosure/ | 1 | ShapeParameters 数据族 |
| wall_state/ | 6 | publish_public_wall_state, spatial_hash_update, wall_spatial_hash, wall_roof_hashmap, spatial_hash_debug, debug_viz_threshold_curve |
| **合计** | **57** | FunctionSystem 签名全部来自 enriched `file_interface_candidates`（native_direct_generic_caller） |

## 方法证据

- FunctionSystem 签名：每个单元 `*.enriched.json.gz` 的 `file_interface_candidates`
  （`native_direct_generic_caller` / `candidate_only`），见 `evidence/inventory.json`。
- 过程地址：`*.pdb.jsonl.gz` 的 S_LINES/procedure 记录 → VA→B EXE 地址映射，
  见 `evidence/address-map.txt`。
- 反编译：18 个代表系统经 Ghidra 只读批量反编译（`evidence/decompiles/b1/*.c`）。
- 既有重建复用：`reconstruction/building/`（wall_bulldoze / wall_pending_changed /
  wall_rectangle_edit / wall_structures / wall_state）、`reconstruction/wall/height_rules.rs`
  （clamp_wall_height 数值）、`evidence/building-wall-history/findings.md`、
  `evidence/building-planning/walls-audit.md`、`evidence/building-integrate/`、
  wp6（Transform 布局 / resources 接线模式）。

## 契约标注

**O** = 反编译/PDB/既有证据直接支持；**I** = 调用链/行号/装配组合推断；**U** = 未恢复
（typed gap，不臆造）。全部系统标记 `implemented_unverified`：无 runtime/GPU 验收；
契约是当前可审阅依据，不因编译通过声称原作等价。

## 状态类型边界

墙状态（`InnerWalls` / `PublicWalls` / `WallState` / `PublicWallState` / 事件载荷）属 WP11；
本包以 `crate::resources::walls::*` typed-gap 门面引用（O 访问器面 + Gap 字段），不补造布局。
