# systems/forest 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/forest` |
| package | `forest` |
| 规模 | 3 文件 / 4 本 ns 函数 / 0 闭包 / **3 业务函数** |
| 难度 | 低 |
| 批次 | 1 |
| 上游依赖 | `country-core`（TreeStorage）、`fastrand`、`glam`、`utils` |
| 复现工程现状 | 未逐项评估，无本机源码；core 侧 TreeStorage（hashbrown + next_index）已有 |

## 任务表

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-FOREST-01 | `src/manage_tree_prefabs.rs` | 恢复 `manage_tree_prefabs`：树 prefab 的加载/生命周期管理（供森林散布与手放树共用）。 | core TreeStorage、asset | 无 | 原入口对照 + prefab 资源关系与原一致 | 未开始 |
| UM-FOREST-02 | `src/tree_animation.rs` | 恢复 `handplaced_tree_animation` 与 `forest_tree_animation`：手放树与森林树的摆动动画（风/季节）。 | 时间/天气状态 | 无 | 原指令样例 + 动画参数与原一致 | 未开始 |
| UM-FOREST-03 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- 树 prefab 资源 schema 属 asset 域（core 之下），需确认边界：本模块只做「管理策略」，不重写 prefab 解码。
- `forest_tree_animation` 与 `tree-tool` 的树动画可能有共享；确认归属后再实现，避免重复。

## 验收标准

- 三函数 ≥ 原指令样例；动画参数（频率/幅度/相位来源）与原一致。
