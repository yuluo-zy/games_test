# systems/timberframe 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/timberframe` |
| package | `timberframe` |
| 规模 | 4 文件 / 4 本 ns 函数 / 0 闭包 / **3 业务函数** |
| 难度 | 低 |
| 批次 | 1 |
| 上游依赖 | `country-core`、`rhapsody`、`glam`、`utils` |
| 复现工程现状 | 未逐项评估，无本机源码 |

## 任务表

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-TIMBER-01 | `src/timberframe_rectangle.rs` | 恢复 `construct_timberframe_rectangle`：矩形墙面的半木结构（timber frame）木梁布局生成。 | wall 几何、utils | 无 | 原指令样例；梁位/间距/截面参数与原一致 | 未开始 |
| UM-TIMBER-02 | `src/timberframe_circle.rs` | 恢复 `construct_timberframe_circle`：圆形墙面的木梁布局生成。 | wall 几何 | 无 | 原指令样例；与矩形共享的规则显式复用 | 未开始 |
| UM-TIMBER-03 | `src/timberframe_beam_count_change_audio.rs` | 恢复 `timberframe_beam_count_change_audio`：木梁数量变化时的音频请求（音频跨切面）。 | 音频胶水 | 无 | 原指令样例 + 触发条件 | 未开始 |
| UM-TIMBER-04 | `src/lib.rs` | 恢复插件注册 `impl Plugin::build`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- 木梁几何依赖 wall-constructor 的墙面分割结果（批次 3）；本批次可用冻结契约先实现矩形/圆形两种基本形状。
- 与 `wall-constructor/src/curved_halftimber` 的职责边界需确认（谁产出梁、谁产出墙裙）。

## 验收标准

- 两构造函数 ≥ 原指令样例；插件注册顺序一致。
