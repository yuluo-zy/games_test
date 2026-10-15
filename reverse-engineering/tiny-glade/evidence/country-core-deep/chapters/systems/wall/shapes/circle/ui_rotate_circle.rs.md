# systems/wall/shapes/circle/ui_rotate_circle.rs

> 文件作用：圆形旋转 UI：world_space_to_screen_space + DebugLine 旋转到更新曲线/enclosure；history 复合编辑。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle` | 0x1409ac910 | 53 | ui_rotate_circle（屏幕空间旋转 → curve/enclosure） | O | wall\decomp\decomp-0x1409ac910.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）

**ui_rotate_circle（O，0x1409ac910）**：16 参系统。屏幕空间旋转圆形墙（改变圆参数中的旋转角，影响屋顶脊朝向）。处理链：

1. **光标→世界**：`FUN_1416f3210 / 1416f31d0 / 1416f3260 / 1416f3330`（world↔screen 空间换算 + 角度工具）；光线地平面求交：`t = (cursor_y − o.y)/d.y`，交点 `P = o + t·d`（`FUN_140c90e20` 入参）。
2. **角度计算**：`FUN_140c90e20(CONCAT44(fVar32, fVar51 − center.y), fVar38)`（**Circle2d 旋转**：由光标相对圆心方向定旋转角）；`fVar49 = FUN_1416f3210(...)`（屏幕空间角度）。
3. **写回**：param_14（Mut）写 `{fVar50−x, x, angle, center qword, z, flag}`。
4. **平滑**：`ease = func_0x142923ec0(*(f32*)(*param_16+0x28) · _UNK_142a16d10)`（旋转速率缓动）。
5. **变更阈值**：`fVar49 <= 0.01f`（@0x142960674）→ 无变化跳过历史；否则 push 0x28B 事件行 + CompoundEditBuilder 历史（`FUN_1416f3260` 角度值、`FUN_140c904a0` ease）。
6. **曲线重建**：`func_0x000140c9cde0 ×5`（圆几何采样）更新曲线数据。

**常量**：变更阈值 0.01 @0x142960674、旋转速率 _UNK_142a16d10（U）、1.0 @0x142925870/142925904。
**跨模块**：`FUN_1416f3xxx` 为 ui-input/camera 域屏幕空间工具（central 关联）；`func_0x142923ec0` = ease；`FUN_140c90e20/c90fd0` = utils/geometry 圆旋转。
**边界**：角度差 ≤0.01 不记历史。
