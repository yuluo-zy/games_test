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
