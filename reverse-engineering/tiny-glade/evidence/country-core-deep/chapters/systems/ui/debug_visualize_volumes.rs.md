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

## 实现方式（算法级）

- `debug_visualize_volumes` @0x1414558c0（O，本域 decomp）：调试体积线框绘制。
  - 门控：`DebugVolume 字节@+0xf2 == 0x01` 才继续，否则早退。
  - 分发：`switch(byte@+0x102)` = 体积类型判别（case 0..0x2f，含 0/1/3/5 绘制分支、2/4 早退）——按体积 tag 走不同图元（三角列表 → line/sphere debug viz）。
  - 取 UiState 写锁（`LOCK [p+0x60]==0→写8`，1e9ns 超时），hashbrown 控制字节 SIMD 扫描 HashMap，逐 widget 读类型字节@+0x26 继续分发；线框/球经 `FUN_140d79f90`/`FUN_14146c880` 发 signifier。
- `debug_visualize_roof_uvs` @0x141454e60（O，wp15 b7）：屋顶 UV 线框调试绘制，2 callers。
- `register_required_components` @0x141458c20（O）：bevy ECS 组件注册胶水。
