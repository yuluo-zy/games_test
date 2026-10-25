# systems/ui/signifiers/animation.rs

> 文件作用：指示符动画：signifier_animation 推进 SignifierAnimation::t 并按缓动 scale/alpha（父挂体积原点用 WidgetVolumeShape::get_origin）；signifier_animation_t 每帧 tick。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::signifiers::animation::signifier_animation_t` | 0x140a69340 | 0 | 动画时间 tick | O（wp15 b7）：2 callers | wp15/analysis/b7_signifiers/signifier_animation_t_0x140a69340.c |
| `country_core::systems::ui::signifiers::animation::signifier_animation` | 0x140a69c30 | 44 | 系统:周期性处理（signifier_animation） | O（wp15 b7）：2 callers | wp15/analysis/b7_signifiers/signifier_animation_0x140a69c30.c |

## 实现方式（算法级）

- `signifier_animation_t` @0x140a69340（O，wp15 b7 + 本域）：每帧 tick——Query 遍历 signifier 实体（槽 stride 0xa0，源索引@+0x94 经 +0x1a8 组件表），对每个 SignifierAnimation 推进时间：`t' = clamp((tick/duration)*target + t, [0, tick])`，写回 `t@+0x14` 并算 `FUN_140c95ef0` 写 `@+0x18`（缓动派生的 scale/alpha 值）。
- `signifier_animation` @0x140a69c30（O，wp15 b7 + 本域）：动画应用——同 Query 迭代，按 `byte@+0x98` 判别（switch 0..0x10，含 mesh 变体分发）把 t 映射到实体 scale/alpha 目标 `@+0x1c/+0x20/+0x24`；父挂体积原点取 WidgetVolumeShape::get_origin（I）。
- 各 2 callers（O）：由 add_ui 挂入 Update 帧循环。
