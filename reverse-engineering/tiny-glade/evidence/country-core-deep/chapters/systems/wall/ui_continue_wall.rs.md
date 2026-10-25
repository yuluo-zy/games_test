# systems/wall/ui_continue_wall.rs

> 文件作用：续画墙 UI：光标接近已结束墙端点到 InProgressWall::new 续画（GladeBorder::is_pos_inside + 装饰 cull + WallContinueState::display_signifier/finish）；suppress_continue_wall 在修饰键/光标移动时抑制（Res<GameTime>）。

函数总计 7：业务 2 / 内联无独立符号 2 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::ui_continue_wall::ui_continue_wall` | 0x140aadd30 | 0 | ui_continue_wall（端点续画 + GladeBorder + WallContinueState） | O | ui-input\decomp\decomp-0x140aadd30.txt, wall\decomp\decomp-0x140aadd30.txt |
| `country_core::systems::wall::ui_continue_wall::suppress_continue_wall` | 0x140aadbb0 | 64 | suppress_continue_wall（修饰键/光标移动抑制；Res<GameTime>） | O | ui-input\decomp\decomp-0x140aadbb0.txt, wall\decomp\decomp-0x140aadbb0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::ui_continue_wall::WallContinueState::finish`（行 34）— 生命周期控制
- `country_core::systems::wall::ui_continue_wall::WallContinueState::display_signifier`（行 43）— UI 交互

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 3 项

## 实现方式（算法级）（O，decomp-0x140aadd30 + decomp-0x140aadbb0）

**ui_continue_wall**（16+ 参）：ResMut/EventWriter（+0x1c）：param_5/6/7/14/16 等多处内联。
常量：**0.7f @0x1429cd030**、**0.1f @0x142925984**、**0.5f @0x142925870**（预览高亮色/透明度）。

1. **逐已发布墙查端点**：遍历 PublicWalls（**0x2a8 步长**，尾→头）：id=条目 key；退化 → panic
   `w.simplified_curve_2d.is_valid()`（**file ui_continue_wall.rs** 0x142ade058，行 50）；
   条目内 u8 标志（entry base -9）+ 第二查表（FUN_1405b0d70）筛选；InnerWalls 查表（0x78 步长）
   读墙曲线点（拷贝）。
2. **光标/端点判定**：query `FUN_1409231f0`（字节@[3]、ptr@[7]、int@[5]、+0x19 标志）；距端点近 → 续画。
3. **signifier 生成**：`Commands::spawn` ×2（FUN_140d79f90），signifier **`continue_wall_`**
   （0x142ade0c8/0x142ade0e8），display 回调（FUN_140b34ec0/FUN_140d8c890、&UNK_140abceb0/FUN_140d4ba00）；
   预览色 0.7f vs 0.5f。
4. **转发**：`FUN_14146c880`（RecordMouseInputIntoWall 域）带墙/曲线数据。
5. **状态机**（jump table 0x142ade214/0x142ade234）：case1 写 **WallContinueState active 字节
   （*ppppppplStack_d0 = 1）** + 事件写（多 EventWriter 8B 行）；case2/3 续。
6. `FUN_1413e7230` ×2、`FUN_140c9a630`（GladeBorder::is_pos_inside 域）×3、`FUN_140d79f90`（Commands）×2。
7. **GameTime panic**：`Resource does not exist` + `country_core::resources::time::GameTimeResource
   requested by … does not exist`（0x142ade244/254/277）。

**PDB 名错位**：decomp 头把 0x140aadbb0 命名为 `WallContinueState::finish`（模块 ui_continue_wall.rs），
DIGEST/任务称 `suppress_continue_wall`；同地址。

**suppress_continue_wall 0x140aadbb0**（O，2.9KB）：4 参：param_1 = Res<修饰键>（字节@0 != 0 → 激活）；
param_2 = ResMut<WallContinueState 记录> `{f32 time@0, f32 dist@4, u8 active@8}`（+0x1c 回写）；
param_3 = Res<GameTime>（**delta_secs @+0x28**）；param_4 = EventReader<MouseMotion>（16B 行，
**u64 打包 `{dx f32, dy f32}` @+8**）。若 active：`time += delta_secs`；逐事件 `dist += sqrt(dx²+dy²)`；
若 `time >= 1.0`（0x142925904）或 **`dist >= 50.0`（0x1429cd070 = 0x42480000，本轮新定值）** →
`active=0` 且双累加器清零；修饰键按下 → `active=1`。
