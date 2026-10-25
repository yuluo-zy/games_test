# systems/history/history_recording.rs

> 文件作用：输入录制启停系统族：history_start_recording / history_finish_recording 驱动 History::start_input_recording / finish_input_recording（0x1409dd790 为 finish，多事件游标 + AppMode 检查）。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::history_recording::history_finish_recording` | 0x1409dd790 | 53 | 结束输入录制（多事件游标+AppMode 检查 → History::finish_input_recording） | O | decomp-0x1409dd790.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::history_recording::history_start_recording`（行 23）— 启动输入录制（委托 History::start_input_recording；无独立符号=内联）（I；CLOSURE §1）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **history_finish_recording 0x1409dd790**：多事件游标（param_7/8/9）+ AppMode
  检查（0x140b2cb30 等辅助）→ `History::finish_input_recording 0x140a86730`
  （取 pending → 四条件空判定 → 上链 + cursor+1 + 快照追加，见 resources/history/
  mod.rs 算法节）。
- history_start_recording 无独立符号（内联进调用点；start_input_recording
  0x140a86270 的 AppMode!=Replay 断言）。

