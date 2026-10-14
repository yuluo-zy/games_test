# systems/wall/ui_draw_wall.rs

> 文件作用：墙绘制 UI 入口：glade 内拖动且非清画布到 RecordMouseInputIntoWall 转发进 CurveNursery；释放到 WallDrawState::finish + FinishWall。5 ResMut（含 WallDrawState/CurveNursery）。

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::ui_draw_wall::ui_draw_wall` | 0x140aafe40 | 0 | ui_draw_wall（5 ResMut；RecordMouseInputIntoWall 转发 + FinishWall） | O | ui-input\decomp\decomp-0x140aafe40.txt, wall\decomp\decomp-0x140aafe40.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::ui_draw_wall::WallDrawState::finish`（行 40）— 生命周期控制

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
