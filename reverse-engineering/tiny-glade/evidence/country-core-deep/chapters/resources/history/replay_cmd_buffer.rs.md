# resources/history/replay_cmd_buffer.rs

> 文件作用：回放命令缓冲：ReplayBuffer（桶 Vec cap-first 元素 0x40 + 活跃计数）、ReplayBucket 0x40B（decorators/stairs Arc + cmds Vec 0x30B）、ReplayCmd 0x30B（判别@0/instant@8/duration@c/负载@10/elapsed@18/undo@1c）、pop_front/front_mut_bucket/make_animations_instant/len。

函数总计 19：业务 4 / 内联无独立符号 15 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::pop_front` | 0x1409f18f0 | 174 | 弹前桶 + counter++ | O | 符号 0x1409f18f0 |
| `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::len` | 0x140949170 | 191 | 桶数 | O | 符号 0x140949170 |
| `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::front_mut_bucket` | 0x1409f1a80 | 198 | 前桶可变引用 | O | 符号 0x1409f1a80 |
| `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::make_animations_instant` | 0x1409f1ab0 | 207 | 全部命令即时完成 | O | 符号 0x1409f1ab0 |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::resources::history::replay_cmd_buffer::impl$15::eq`（行 26）— derive 胶水（Debug/Clone/Eq/Default）（派生；复用 derive，不计业务）
- `country_core::resources::history::replay_cmd_buffer::impl$0::from`（行 50）— From 转换（RasterEdit 构造）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$1::from`（行 56）— DecoratorHistoryState::From（type@+0x41==8 preview panic）（O；decomp-0x140b3f930.txt）
- `country_core::resources::history::replay_cmd_buffer::impl$2::from`（行 62）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$3::from`（行 68）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$4::from`（行 74）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$5::from`（行 80）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$6::from`（行 86）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::impl$7::from`（行 92）— From 转换（负载构造/归一）（I；内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayCmd::make_animation_instant`（行 114）— 单命令即时完成（I；内联；make_animations_instant 0x1409f1ab0 内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::new`（行 161）— 空缓冲构造（I；内联；insert_history_resources 内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::add`（行 169）— 尾部 push 桶 + counter++（I；内联；replay_edit 0x140af6170 内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::is_empty`（行 194）— 判空（I；内联；finish_replay/history_process 内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::current_replay_cmd`（行 202）— 前桶首命令（I；内联；signifier_replay_active 内联）
- `country_core::resources::history::replay_cmd_buffer::ReplayBuffer::modification_counter`（行 223）— 修改计数访问器（I；内联；save_game 读取）

<!-- 未覆盖函数数：0 -->
