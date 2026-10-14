# systems/ui/debug_visualize_volumes.rs

> 文件作用：调试可视化：三角形体积线框/球体调试绘制 + 屋顶 UV 线框；DebugRoofs/DebugVolume 资源 + 组件注册（bevy 胶水）。

函数总计 11：业务 3 / 内联无独立符号 0 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::debug_visualize_volumes::impl$0::register_required_components` | 0x141458c20 | 18 | 组件注册 | O：register_required_components（bevy ECS 注册胶水） | ui-input\decomp\decomp-0x141458c20.txt |
| `country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs` | 0x141454e60 | 28 | 屋顶 UV 线框调试绘制 | O：debug_visualize_roof_uvs（2 callers；wp15 b7 既有） | wp15/analysis/b7_signifiers/debug_visualize_roof_uvs_0x141454e60.c |
| `country_core::systems::ui::debug_visualize_volumes::debug_visualize_volumes` | 0x1414558c0 | 89 | 调试绘制 | O：debug_visualize_volumes 主体（三角列表 → line/sphere debug viz） | ui-input\decomp\decomp-0x1414558c0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 8 项
