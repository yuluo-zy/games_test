# systems/wall/eraser/animate_wall_bulldoze.rs

> 文件作用：墙推土动画：AnimateWallBulldozeBuffer 152B 命令行{Source112, WallId+0x70, low/high+0x78/+0x7c, anchor+0x80, phase+0x8c, finished+0x90}；每帧 phase=clamp01(phase+dt/0.2)（先推进后应用 CubicInOut）；增长写 Source 切片+enclosure tag2+style；完成移除 authority（128 场景/768 全 RET 先验验证）。

函数总计 7：业务 1 / 内联无独立符号 4 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze` | 0x140ac27e0 | 74 | animate_wall_bulldoze（phase 推进 CubicInOut + 曲线切片 + authority 移除） | O | wall\decomp\decomp-0x140ac27e0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeCmd::new`（行 30）— 构造
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::new`（行 59）— 构造
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::is_empty`（行 63）— 谓词/校验
- `country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer::contains`（行 67）— 谓词/校验

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
