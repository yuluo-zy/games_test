# systems/history/replay_wall.rs

> 文件作用：墙回放系统：前桶判别 4 命令，每 WallReplayEditCmd（0x18B）经 process（0x140abfb30，自递归，8 参）分派应用（instant/动画两路径）；done 标志 @+0x15；全部完成时经 niche vtable 0x142ab4910 收集已改墙。附 _deprecated_morph_curve（已弃用形态曲线应用）。

函数总计 12：业务 4 / 内联无独立符号 8 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_wall::_deprecated_morph_curve` | 0x140952570 | 0 | 已弃用的形态曲线应用 | O | decomp-0x140952570.txt |
| `country_core::systems::history::replay_wall::replay_wall` | 0x140951b80 | 33 | 见文件作用 | O | decomp-0x140951b80.txt |
| `country_core::systems::history::replay_wall::replay_wall::closure$0` | 0x140abfaa0 | 72 | 墙命令应用闭包（replay_wall 内联宿主） | I | decomp-0x140abfaa0.txt |
| `country_core::systems::history::replay_wall::WallReplayEditCmd::process<country_core::systems::history::replay_wall::replay_wall::closure_env$0>` | 0x140abfb30 | 271 | 见文件作用（自递归 8 参分派） | O | decomp-0x140abfb30.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_wall::WallReplayCmd::is_single_adjust_height_cmd`（行 152）— 单 AdjustHeight 命令判定（元素数==1）（I；内联）
- `country_core::systems::history::replay_wall::WallReplayCmd::new`（行 158）— 构造墙回放命令（自 ConcurrentEdits）（I；内联；源行 158）
- `country_core::systems::history::replay_wall::impl$0::new::closure$0`（行 170）— 构造器内组处理闭包（I；内联）
- `country_core::systems::history::replay_wall::impl$0::new::closure$0::closure$0`（行 174）— WallReplayCmd::new 从 ConcurrentEdits 展开的嵌套闭包（I；内联；源行 170/174）
- `country_core::systems::history::replay_wall::WallReplayEditCmd::process_instant`（行 307）— instant 路径应用（cubic_in_out 后按判别分派）（I；内联；源行 307）
- `country_core::systems::history::replay_wall::impl$1::process_instant::closure$0`（行 353）— process_instant 内联闭包（I；内联；源行 353/502）
- `country_core::systems::history::replay_wall::impl$1::process_instant::closure$1`（行 502）— process_instant 内联闭包（I；内联；源行 353/502）
- `country_core::systems::history::replay_wall::WallReplayEditCmd::process_animated`（行 618）— 动画路径应用（I；内联；源行 618）

<!-- 未覆盖函数数：0 -->
