# systems/debug/human_scale_ref.rs

> 文件作用：**人体比例参考调试可视化（多模式跳表）。**

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::debug::human_scale_ref::impl$0::clone_behavior | 0x140002370 | 9 | bevy Component clone_behavior 派生空实现（全局共享地址，非业务） | O（派生胶水） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| country_core::systems::debug::human_scale_ref::impl$0::register_required_components | 0x1408b3d50 | 9 | HumanScaleRef 组件注册（bevy 胶水，TypeId 查表） | O（胶水） | audio-misc\decomp\decomp-0x1408b3d50.txt |
| country_core::systems::debug::human_scale_ref::human_scale_ref | 0x1408b2b30 | 15 | 0x1408b2b30：5 参 bevy 系统；*param_5+0xf8 调试开关门禁；[*param_2+0x110] u32 模式→跳表 0x142a9f3b0；puffin scope | O | audio-misc\decomp\decomp-0x1408b2b30.txt |

## 实现方式（算法级）（O，decomp-0x1408b2b30）

**human_scale_ref @0x1408b2b30**：
1. puffin scope 包帧；先读 `*param_5+0xf8` 调试开关——关则整帧跳过（门禁）。（O）
2. `[*param_2+0x110]` u32 模式值 → 跳表 **0x142a9f3b0** 分派各模式画法（人形比例参考
   不同表示）。（O）
3. 每模式画法细节 U（debug 工具面，未逐分支深挖）。O/I 混合
- clone_behavior @0x140002370 / register_required_components @0x1408b3d50 = bevy 胶水（O）。
