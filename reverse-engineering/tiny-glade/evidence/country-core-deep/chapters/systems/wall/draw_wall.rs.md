# systems/wall/draw_wall.rs

> 文件作用：徒手墙绘制系统（最大生命周期系统）：平滑输入空早退；SnappingMode 吸附光标位置；InProgressWall::new 起笔；CurveNursery 累积笔画；InProgressCurve::add_weight_to_the_last_point；Curve::crashy_new_from_points 到 InnerWalls::update_wall_state_curve + update_freehand_ty。4 ResMut（InnerWalls/InProgressWallResource/CurveNursery/InProgressCurve）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::draw_wall::draw_wall` | 0x140a7cba0 | 45 | draw_wall（4 ResMut；CurveNursery + crashy_new_from_points + update_freehand_ty） | O | wall\decomp\decomp-0x140a7cba0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
