# systems/wall/adjust_wall_height/signifier_animation.rs

> 文件作用：墙高 signifier 动画：平屋顶无 rim（Roof::is_flat_w_no_rim）到 adjust_wall_height_curve 重算 + crashy_new_from_points；否则 SignifierAnimation::t 驱动。

函数总计 8：业务 2 / 内联无独立符号 1 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::signifier_animation::impl$1::register_required_components` | 0x14085d900 | 16 | 构造/注册/生成 | U |  |
| `country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation` | 0x14085bb50 | 39 | signifier_animation（flat_w_no_rim → 重算曲线；否则 SignifierAnimation::t） | O | wall\decomp\decomp-0x14085bb50.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier::new`（行 27）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 5 项
