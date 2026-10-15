# systems/glade-border 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/glade-border` |
| package | `system-glade-border` |
| 规模 | 3 文件 / 3 本 ns 函数 / 1 闭包 / **1 业务函数** |
| 难度 | 低 |
| 批次 | 1 |
| 上游依赖 | `country-core`、`glam`、`utils` |
| 复现工程现状 | 未逐项评估，无本机源码 |

## 任务表

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-BORDER-01 | `src/display_glade_border.rs` | 恢复 `display_glade_border`：林间空地边界的显示（边界网格/材质/可见性）。 | core 世界边界资源、rhapsody | 无 | 原入口对照 + 边界几何参数与原一致 | 未开始 |
| UM-BORDER-02 | `src/feedback_input_out_of_border.rs` | 恢复「输入超出边界」反馈：玩家把编辑笔刷移出边界时的提示/限制。 | 输入层 | 无 | 原指令样例 + 触发条件与原一致 | 未开始 |

## 卡点

- 边界资源（glade border 尺寸/形状）生产者可能在 core；需先确认定义归属，不重复实现。
- 反馈触发依赖输入层（批次 4 tiny-glade 的输入接线），本批次只恢复判定逻辑。

## 验收标准

- 两函数 ≥ 原指令样例；边界资源复用 core，不自建。
