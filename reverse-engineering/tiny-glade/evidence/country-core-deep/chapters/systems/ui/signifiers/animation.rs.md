# systems/ui/signifiers/animation.rs

> 文件作用：指示符动画：signifier_animation 推进 SignifierAnimation::t 并按缓动 scale/alpha（父挂体积原点用 WidgetVolumeShape::get_origin）；signifier_animation_t 每帧 tick。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::signifiers::animation::signifier_animation_t` | 0x140a69340 | 0 | 动画时间 tick | O（wp15 b7）：2 callers | wp15/analysis/b7_signifiers/signifier_animation_t_0x140a69340.c |
| `country_core::systems::ui::signifiers::animation::signifier_animation` | 0x140a69c30 | 44 | 系统:周期性处理（signifier_animation） | O（wp15 b7）：2 callers | wp15/analysis/b7_signifiers/signifier_animation_0x140a69c30.c |
