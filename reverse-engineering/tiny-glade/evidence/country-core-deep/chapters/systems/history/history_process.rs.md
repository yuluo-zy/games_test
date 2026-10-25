# systems/history/history_process.rs

> 文件作用：历史命令中枢系统：消费 HistoryCmd 事件流（undo/redo/interrupt），移动 History 游标后对目标 MultiEdit 调用 replay_edit 展开 ReplayCmd 桶（undo 时反转），并把桶 push 进 ReplayBuffer；同时消费第二事件流（16B 记录，byte[+9] 跳表 0x142ae5420）做命令校验/动画即时化/移除，录制中仅记日志。generate_replay_cmds_from_edit 为按 HistoryEdit.tag 分派（跳表 0x142c5f248）的共享核心。

函数总计 17：业务 8 / 内联无独立符号 9 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env$0>` | 0x14143fb50 | 0 | 按 HistoryEdit.tag 经跳表分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同；round3 round2 修正：实函数约 10.4KB 非 316B 跳板——316B 为 Ghidra 跳表恢复失败截断；各单态跳表独立，0x142c5f248 仅属 0x14143d2c0 单态，water 单态 0x141444c70 跳表基址 0x142c5f2fc） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_garden::replay_garden::closure$1::closure_env$0>` | 0x14143d2c0 | 0 | 按 HistoryEdit.tag 经跳表分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同；round3 round2 修正：实函数约 10.4KB 非 316B 跳板——316B 为 Ghidra 跳表恢复失败截断；各单态跳表独立，0x142c5f248 仅属 0x14143d2c0 单态，water 单态 0x141444c70 跳表基址 0x142c5f2fc） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_path::generate_path_replay_cmds::closure_env$0>` | 0x1414423e0 | 0 | 按 HistoryEdit.tag 经跳表分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同；round3 round2 修正：实函数约 10.4KB 非 316B 跳板——316B 为 Ghidra 跳表恢复失败截断；各单态跳表独立，0x142c5f248 仅属 0x14143d2c0 单态，water 单态 0x141444c70 跳表基址 0x142c5f2fc） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::generate_replay_cmds_from_edit<country_core::systems::history::replay_water::replay_water::closure$1::closure_env$0>` | 0x141444c70 | 0 | 按 HistoryEdit.tag 经跳表分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同；round3 round2 修正：实函数约 10.4KB 非 316B 跳板——316B 为 Ghidra 跳表恢复失败截断；各单态跳表独立，0x142c5f248 仅属 0x14143d2c0 单态，water 单态 0x141444c70 跳表基址 0x142c5f2fc） | O | DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt |
| `country_core::systems::history::history_process::HistoryCmd::interrupt` | 0x140af5b20 | 94 | 构造中断命令字 0x02000200（mov eax,0x00020002;ret） | O | history\decomp\decomp-0x140af5b20.txt |
| `country_core::systems::history::history_process::history_process` | 0x140af5b30 | 147 | 见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log | O | decomp-0x140af5b30.txt；DIGEST §3.3 |
| `country_core::systems::history::history_process::replay_edit` | 0x140af6170 | 258 | undo/redo 游标+replay_edit+桶 push+第二事件流+log（round3 round2 修正：原作用列为 history_process 描述复制） | O | decomp-0x140af6170.txt（round3 round2 修正：原错引相邻函数 decomp-0x140af5b30.txt）；DIGEST §3.3 |
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
## 实现方式（算法级，2026-10-15 深挖；全文见 history/ALGORITHMS.md §2.3）

- **HistoryCmd 4B 记录**：byte[0]=tag、byte[1]=子命令（0=Undo、1=Redo、4=中断标志）、
  byte[2..3]=标志（传 generate_replay_cmds 的 u16 低位）。
- **history_process 0x140af5b30 主循环**：
  1. param_4 Local 双流（16B 记录）：`剩余 = 计数 - 游标`（负钳 0），计数 -= 两流剩余；
  2. 两流均空 → 守卫（ReplayBuffer+0x18 != 0 → return 让回放排空；param_9 队列 +0x10
     != 0 → return；Events<HistoryCmd> 空 → return）→ 消费一条 4B：
     byte[1]==4 或 byte[0]==2 → **Interrupt 直接 return**；byte[1]==0 → Undo 三重守卫
     （cursor>=0、cursor>history_start、cursor<len）后 cursor--；byte[1]==1 → Redo
     断言 cursor>=-1 后 cursor=目标；其余 → unreachable（history_process.rs:40）；
  3. 对 edit_chain[cursor] 的 edits 逐个 generate_replay_cmds（FUN_14143fb50 单态）
     写 0x30B 命令 Vec；Arc clone 两表（LOCK inc+溢出 panic）；
     **undo 且命令数>1 → 命令 Vec 整体反转**（0x30=6 qword 两两交换，双指针向中）；
  4. 0x40B 桶 push 进 ReplayBuffer（0x140af6381 区：桶 Vec 追加 + 命令区/索引 Vec
     维护 + 0x120 长度校验），逐命令经跳表 0x142ae545c 校验（重复/冗余丢弃：
     `tag&7!=0 → 无效`、tag0 instant+duration 全等 → 重复、tag4 count==1 置既有
     命令 +0x49 |= 0x10、tag>10 → 容量错误）；
  5. 两流非空 → 录制态检查（pending>=0 → log "Can't process History while recording
     a new edit is active!" @0x142ae5290，级别>1，return）→ `switch(byte[+9])` 经
     0x142ae5420：0x0/0x1 = 末 HistoryCmd byte[1]==1/0 门控、0x2 = 消费跳过、0x3 =
     make_animations_instant（0x1409f1ab0）、0x4-0xe = 校验/合并、0x13 = 按索引移除
     命令（拷贝 24B + memmove 压缩 + drop 分派）、0x17 = Arc dec、0x18 = return、
     0x19-0x1b 无效；消费动作 = 清 Events<HistoryCmd> + param_8 计数器 push 其
     Vec<u64>；每条记录（tag!=3）追加 param_6 的 Vec<16B> {计数器, byte[+0xb]}。
- **replay_edit 0x140af6170**：同构独立入口（生成→反转→桶 push→同套校验）。
- **generate_replay_cmds_from_edit ×4**：按 HistoryEdit.tag 经跳表（garden 单态
  0x142c5f248 / water 0x142c5f2fc，各单态独立）尾调共享核心（~10.4KB，非 316B
  跳板——316B 为 Ghidra 跳表恢复截断）；generate_path_replay_cmds 0x1416286d0 =
  niche 判别 0..3 过滤后尾调 0x1414423e0。

