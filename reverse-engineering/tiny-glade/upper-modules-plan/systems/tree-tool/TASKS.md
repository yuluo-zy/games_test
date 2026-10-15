# systems/tree-tool 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/tree-tool` |
| package | `system-tree` |
| 规模 | 11 文件 / 15 本 ns 函数 / 2 闭包 / **12 业务函数** |
| 难度 | 低 |
| 批次 | 1 |
| 上游依赖 | `country-core`、`rhapsody`、`serde_json`、`fastrand`、`glam`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**部分草稿**，6 文件有命令/删除/回放/地形跟随/弯曲/预置树草稿；有 reva/RNE 反汇编交叉核对，未编译/未系统验收 |

## 任务表（按 B src 层级）

### 命令与生命周期

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TREE-01 | `src/process_tree_cmds.rs` | 恢复 `process_tree_cmds`、`despawn_trees`：树编辑命令的分发与销毁。 | core 命令/实体 | 草稿 | 原指令样例 + 命令载荷判别值 | 草稿 |
| UM-TREE-02 | `src/convert_preplaced_trees.rs` | 恢复 `convert_preplaced_trees`：预置树转正式树。 | forest prefab | 草稿 | 原指令样例 | 草稿 |
| UM-TREE-03 | `src/replay_tree.rs` | 恢复 `replay_tree`：树的编辑回放。 | core 历史回放 | 草稿 | 原入口对照 | 草稿 |

### 地形/物理跟随

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TREE-04 | `src/recalc_tree_position_on_terrain_height_change.rs` | 恢复 `recalc_tree_position_on_terrain_height_change`：地形高度变化时重算树位。 | terrain 高度 | 草稿 | 原指令样例 + 地形采样一致 | 草稿 |
| UM-TREE-05 | `src/tree_move_bend.rs` | 恢复 `tree_move_bend`：拖动树时的弯曲变形。 | glam/utils | 草稿 | 原指令样例 | 草稿 |
| UM-TREE-06 | `src/hide_trees.rs` | 恢复 `hide_trees`：树的隐藏/显示切换。 | 实体可见性 | 草稿 | 原指令样例 | 草稿 |

### UI 交互

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TREE-07 | `src/ui_place_tree.rs` | 恢复 `ui_place_tree`、`place_tree`、`random_tree_prefab`：放置树（含随机 prefab 选取）。 | tree prefab、RNG | 无 | 原指令样例 + RNG 序列与原一致 | 未开始 |
| UM-TREE-08 | `src/ui_move_tree.rs` | 恢复 `ui_move_tree`：移动树的交互。 | 输入层 | 无 | 原指令样例 | 未开始 |
| UM-TREE-09 | `src/ui_delete_tree.rs` | 恢复 `ui_delete_tree`：删除树的交互。 | 输入层 | 无 | 原指令样例 | 未开始 |

### 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TREE-10 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- `PlaceTreeCmd` 载荷结构与 `TreeEdit` 跨系统判别值未定（MODULE_PROGRESS 已登记）；需先从原侧定位，不猜。
- RNG 用原版 `fastrand 1.8.0`，`random_tree_prefab` 需与原种子序列逐位对照。

## 验收标准

- 每函数 ≥ 原指令样例；命令载荷/判别值有原侧证据；插件注册顺序一致。
