# systems/history/replay_mode.rs

> 文件作用：回放节奏系统：检测 AppMode 进出 0xd(Replay)（log started/finished）、RestoreAppTo 记录回放前模式、长按快进（HoldToFastReplay）时发 redo HistoryCmd、驱动 ReplayModeElapsedTime 计时。

函数总计 4：业务 1 / 内联无独立符号 3 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_mode::replay_mode` | 0x140a7c550 | 0 | 见文件作用 | O | decomp-0x140a7c550.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_mode::impl$2::default`（行 19）— RestoreAppTo 默认（I；内联）
- `country_core::systems::history::replay_mode::RestoreAppTo::set`（行 24）— 记录回放前 AppMode（I；内联；源行 24）
- `country_core::systems::history::replay_mode::RestoreAppTo::get`（行 50）— 取回放前 AppMode（I；内联；源行 50）

<!-- 未覆盖函数数：0 -->
