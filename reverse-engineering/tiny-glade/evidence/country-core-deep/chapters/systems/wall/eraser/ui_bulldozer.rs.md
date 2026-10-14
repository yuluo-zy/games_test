# systems/wall/eraser/ui_bulldozer.rs

> 文件作用：推土机 UI 入口：wall_eraser_bulldoze_condition = AppMode{tag==9, +4==3}；handle_decorators 排空墙属装饰 + terrain/stair cull；can_reconnect_stairs_if_midpoint_deleted；StairsState::disconnect_edge；命中到 AnimateWallBulldozeCmd 入 buffer（RoofDelete FIRST + WallCompound StateChange+Delete 双组）。

函数总计 14：业务 3 / 内联无独立符号 2 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze` | 0x1409de190 | 0 | wall_eraser_bulldoze（装饰排空 + 楼梯断开 + RoofDelete FIRST 双组） | O | ui-input\decomp\decomp-0x1409de190.txt, wall\decomp\decomp-0x1409de190.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze_condition` | 0x1409de170 | 49 | wall_eraser_bulldoze_condition（AppMode{tag==9,+4==3}） | O | ui-input\decomp\decomp-0x1409de170.txt, wall\decomp\decomp-0x1409de170.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::hammer_cursor_icon` | 0x1409e35a0 | 933 | UI 交互 | I | ui-input\decomp\decomp-0x1409e35a0.txt, wall\decomp\decomp-0x1409e35a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::ui_bulldozer::can_reconnect_stairs_if_midpoint_deleted`（行 817）— 谓词/校验
- `country_core::systems::wall::eraser::ui_bulldozer::handle_decorators`（行 836）— 系统:事件响应

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 8 项
