# RESTORE — forest（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/forest/`。
> 证据：`groupA/forest/RE-ANALYSIS.md`、`groupA/forest/decomp/`、BASELINE。

## 实现清单

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `manage_tree_prefabs` | 0x141bf16c0 | 已实现（两阶段，语义级） | 14 参按 BASELINE；常量 O：界 65.0（0x142a83814）、树-形间距 7.0（0x142a7cd48）、缩放 0.3..0.75、高抖 10.0、栅格尺度 130.0；`is_too_close_to_shape`（内联，圆距离 + 7.0）；墙冲突 = `WallSpatialHash::query_with_circle`（0x1422785d0 单态族）；新树 fastrand 缩放 + `SpawnPrefabCmd`；既有树修剪（实体 despawn 登记宿主） |
| `handplaced_tree_animation` | 0x142279500 | 已实现（语义级） | 7 参按 BASELINE；相机距离 → canopy 剔除（`should_remove_tree_canopy_from_view` 行 152 内联）；AudioTriggerLibrary 面 |
| `forest_tree_animation` | 0x14227a0d0 | 已实现（语义级） | 4 参；标记树 → DespawnPrefabCmd + placements 修剪；sway 相位累进（I） |
| `impl$0::build` | 0x140650c50 | 已实现（O 面） | init_resource（InteractableForestPlacements）+ add_systems ×2 |

## core 侧接线（delta）

- `components/trees.rs`（新）：TreeMarkedForDeletion / TreeAnimation / TreeMoveBend / TreeComponent / ConvertedPreplacedTree / PrefabInstanceData / Ui2d*（typed gap）。
- `resources/tree_storage.rs`：InteractableForestTree / InteractableForestPlacements / MoveTreeCmd / HideTreesinAreaCmd / HideTreesNextToWallCmd + `clear()`。
- `resources/history/tree_edit.rs`：TreeCanopyMaxCullDistance（Resource，render 侧"无 Resource"观察登记差异）/ InsertTreeCmd / DeleteTreeCmd / SpawnPreplacedTrees / TreeReplayCmd / TreeEdit::kind/id 访问器。
- `systems/prefab.rs`（新）：PrefabInstanceData / SpawnPrefabCmd / DespawnPrefabCmd；`resources/prefab.rs`（新）：PrefabInstance（BASELINE 路径）。
- `resources/events.rs`（新）：ConstructTimberframeCmd（0x28B 载荷面）。
- `systems/paths.rs`：PathRastersResources（derive SystemParam，as_float_grid 0x140ae4c20）。
- `systems/wall/wall_state/wall_spatial_hash.rs`：`query_with_circle`（0x1425defd0/0x1425df320/0x1422785d0 单态族）+ 线段距离辅助。

## 未实现项 / 待 reviewer

1. 6 栅格采样的 2 附加栅格身份（water/garden 候选，I）与密度累计的精确落位。
2. 0x50B 树记录 / 0x38B spawn bundle 字段明细（U）。
3. 候选 flag 的精确编码（越界/栅格/形状分支的 0/1 语义，re-analyst §2 两处读数出入）。
4. 树-形状近距的矩形分派细部（0x1409ea800）。
5. handplaced 的 canopy 剔除阈值族（canopy 可见性写面 U）。

## 编译

`cargo check -p forest`（离线）通过。

## 修复轮 2（reviewer-A）

- 新树记录 position 改真实（候选评估位置随候选条目携带）；6 栅格采样/密度/形状淘汰、canopy 算法、easing 动画 = GAP（挂 re-analyst 补证）。

## 修复轮 4（reviewer-A 第三轮，2026-10-16）

- 真实树同步编译：process_tree_cmds commands 补 mut、ui_color_decorator outlines 补 mut（真实树实修——前轮仅 worktree）。
- ui_place_tree 生产者：点击位置按 pack_2x16f_uint 打包（f16 位格式，half crate）——解包链闭环；PlaceTreeCmd 改 0x18B 三槽 {id, packed_position, extra}。
- ui_move_tree case2 拖动中：拖动帧实时移动命令 + 目标位置维护（Local target）。
- hide_trees 圈源语义：命令圈 vs 机器码相机圈/双哈希（0x18B+0x30B 层）登记 I，待 re-analyst 核对。
- forest/convert：6 路栅格采样落地（RE-ANALYSIS-2）——4 路地面栅格密度 clamp(Σ,0,1) + Water/Garden 排除面 >0 跳过 + 墙冲突 + 间距 7.0。
