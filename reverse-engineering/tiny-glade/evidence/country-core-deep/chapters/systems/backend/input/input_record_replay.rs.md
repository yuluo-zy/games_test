# systems/backend/input/input_record_replay.rs

> 文件作用：输入录制/回放状态机：mode@+0x40（^0x8000000000000000 后 case 0/2/3/4 = Idle/Replaying/Recording/Finished）。Idle 满足条件即 store camera + 克隆 ToolbarCategoryModes + 重建流；Replaying 按 0xd0B InputFrame 写回状态（@+0x30 u64/@+0x38 u32/@+0x3c u8/@+0x40 双 f32/@+0x44 除数/@+0x58..0xc0 十 qword）；越界 panic 'Out of bounds access'。

函数总计 8：业务 3 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::input_record_replay::control_input_replay` | 0x140941a30 | 45 | 键盘控制录制/回放 | O：control_input_replay（键控 mode 切换 + seek sync point + camera 参数存取） | ui-input\decomp\decomp-0x140941a30.txt |
| `country_core::systems::backend::input::input_record_replay::advance_input_replay_frame` | 0x140942280 | 109 | 推进回放帧 | O：advance_input_replay_frame：Replaying 时 index@+0x48++，帧尽→Finished | ui-input\decomp\decomp-0x140942280.txt |
| `country_core::systems::backend::input::input_record_replay::input_record_replay` | 0x1409423d0 | 123 | 系统:输入处理 | O：完整状态机（见上）；流重建（frames 容量 8 + 1 sync point）；delta 经 2×2 矩阵 _UNK_142a171f0 + 偏移变换；advance 帧尽置 Finished(0x8000000000000004) | ui-input\decomp\decomp-0x1409423d0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::input::input_record_replay::normalize_cursor_pos`（行 223）— 待推断
- `country_core::systems::backend::input::input_record_replay::unnormalize_cursor_pos`（行 228）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项
