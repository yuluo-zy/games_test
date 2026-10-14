# systems/history/history_process.rs

> 文件作用：历史命令中枢系统：消费 HistoryCmd 事件流（undo/redo/interrupt），移动 History 游标后对目标 MultiEdit 调用 replay_edit 展开 ReplayCmd 桶（undo 时反转），并把桶 push 进 ReplayBuffer；同时消费第二事件流（16B 记录，byte[+9] 跳表 0x142ae5420）做命令校验/动画即时化/移除，录制中仅记日志。generate_replay_cmds_from_edit 为按 HistoryEdit.tag 分派（跳表 0x142c5f248）的共享核心。

函数总计 17：业务 8 / 内联无独立符号 9 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0>` | 0x14143fb50 | 0 | 按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>` | 0x14143d2c0 | 0 | 按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_path::generate_path_replay_cmds::closure_env$0>` | 0x1414423e0 | 0 | 按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_water::replay_water::closure$1::closure_env$0>` | 0x141444c70 | 0 | 按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::HistoryCmd::interrupt` | 0x140af5b20 | 94 | 构造中断命令字 0x02000200（mov eax,0x00020002;ret） | O | history\decomp\decomp-0x140af5b20.txt |
| `country_core::systems::history::history_process::history_process` | 0x140af5b30 | 147 | 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log | O | decomp-0x140af5b30.txt；DIGEST §3.3 |
| `country_core::systems::history::history_process::replay_edit` | 0x140af6170 | 258 | 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log | O | decomp-0x140af5b30.txt；DIGEST §3.3 |
| `country_core::systems::history::history_process::replay_edit::closure$0` | 0x14143d220 | 267 | 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log | O | decomp-0x140af5b30.txt；DIGEST §3.3 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::history_process::PendingHistoryCommands::is_empty`（行 126）— 待处理回放命令队列判空（ReplayBuffer 消费侧门控）（I；内联）
- `country_core::systems::history::history_process::PendingHistoryCommands::new`（行 130）— 空队列构造（I；内联）
- `country_core::systems::history::history_process::raster_edit_to_replay_cmd`（行 544）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::schedule_replay_commands`（行 569）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::schedule_replay_commands::closure$0`（行 591）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::schedule_replay_commands::closure$1`（行 626）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::schedule_replay_commands::closure$2`（行 647）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::schedule_replay_commands::closure$2::closure$0`（行 650）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）
- `country_core::systems::history::history_process::history_clear_previous_state`（行 691）— 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log（O；decomp-0x140af5b30.txt；DIGEST §3.3）

<!-- 未覆盖函数数：0 -->
