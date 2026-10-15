# systems/history/mod.rs

> 文件作用：history 系统族注册面：add_history_systems / insert_history_resources / add_history_events 三个 Bevy App 扩展胶水（注册 ~10 个回放/录制系统、插入 History/ReplayBuffer 等资源、添加 HistoryCmd 等事件）。AppMode 回放判别 0xd 的边界定义也在此。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::impl$0::add_history_systems` | 0x1409438a0 | 43 | Bevy App 扩展：注册 history 系统集合（~10 系统） | O | decomp-0x1409438a0.txt |
| `country_core::systems::history::impl$0::insert_history_resources` | 0x140943e10 | 85 | 插入 History/ReplayBuffer 等资源 | O | decomp-0x140943e10.txt |
| `country_core::systems::history::impl$0::add_history_events` | 0x140943f40 | 97 | 添加 HistoryCmd 等事件类型 | O | decomp-0x140943f40.txt |

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- add_history_systems 0x1409438a0 / insert_history_resources 0x140943e10 /
  add_history_events 0x140943f40：Bevy 注册胶水（~10 系统注册、History/ReplayBuffer
  等资源插入、HistoryCmd 事件添加；0x14362c370 静态经 FUN_140b218a0 读写）——
  按依赖过滤不计业务；AppMode::Replay = 0xd 的边界定义在此（replay_mode 0x140a7c550
  同址复核）。

