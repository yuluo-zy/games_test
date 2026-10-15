# systems/arrow_gizmo_animation.rs

> 文件作用：**箭头 gizmo 拉伸动画**——`EnableArrowGizmoStretch` 组件实体的双 f32 拉伸值：
> 有输入事件（8B 向量 payload）按**主轴** `stretch=min(stretch+|δ|·0.02, 1.0)`（0.02@1429cd2d0）；
> 无事件每帧衰减 `stretch=max(stretch−0.1, 0)`（−0.1@1429eb6f8）；Mut tick 回写。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `arrow_gizmo_animation` | 0x140a92f80 | 0 | 见文件作用；puffin scope `country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation` | O：实证 | camera-time\decomp\decomp-0x140a92f80.txt |

## 实现方式（算法级）

`arrow_gizmo_animation`（0x140a92f80）：每实体（EnableArrowGizmoStretch）双 f32 拉伸值；有输入事件（0x10B 步长，8B 向量 payload）按主轴（|x|<|y| 取 y 轴）`stretch=min(stretch+|Δ|·0.02, 1.0)`（0.02@0x1429cd2d0）；无事件每帧 `stretch=max(stretch−0.1, 0)`（−0.1@0x1429eb6f8）；Mut tick 回写；puffin scope country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation。
