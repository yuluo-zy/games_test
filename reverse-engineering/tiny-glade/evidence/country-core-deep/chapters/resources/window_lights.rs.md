# resources/window_lights.rs

> 文件作用：**窗光/灯资源族（WindowLight 组件/LightInfo/Global*Control）。**

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::window_lights::impl$2::clone_behavior | 0x140002370 | 26 | bevy Component clone_behavior 派生空实现（全局共享地址，非业务） | O（派生胶水） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| country_core::resources::window_lights::impl$2::register_required_components | 0x14086cac0 | 26 | 0x14086cac0：bevy 组件注册胶水（TypeId 查表） | O（胶水） | audio-misc\decomp\decomp-0x14086cac0.txt |

## 实现方式（算法级）（O 胶水 / U 布局）

**window_lights.rs**：WindowLight 组件/LightInfo/Global*Control 资源族——仅
clone_behavior @0x140002370 与 register_required_components @0x14086cac0（bevy 组件注册
胶水，TypeId 查表）可见；各类型字段布局/默认值 **U**（audio-misc 批未覆盖独立函数体）。
O/I 混合

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 2 项
