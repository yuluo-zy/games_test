# systems/debug/sheep_controls.rs

> 文件作用：**绵羊调试控制（拾取/快照）。**

函数总计 11：业务 5 / 内联无独立符号 2 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::systems::debug::sheep_controls::impl$2::clone_behavior | 0x140002370 | 25 | bevy Component clone_behavior 派生空实现（全局共享地址，非业务） | O（派生胶水） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| country_core::systems::debug::sheep_controls::impl$2::register_required_components | 0x1416f2910 | 25 | SheepControl 组件注册（bevy 胶水） | O（胶水） | audio-misc\decomp\decomp-0x1416f2910.txt |
| country_core::systems::debug::sheep_controls::SheepControl::new | 0x1416f0ea0 | 57 | 0x1416f0ea0：调试控制组件构造 | O（名） |  |
| country_core::systems::debug::sheep_controls::sheep_controls | 0x1416f11a0 | 97 | 0x1416f11a0：调试绵羊控制（PickingMode 拾取） | O（名） | audio-misc\decomp\decomp-0x1416f11a0.txt |
| country_core::systems::debug::sheep_controls::apply_sheep_snapshot | 0x1416f24f0 | 358 | 0x1416f24f0：应用绵羊快照 | O（名） | audio-misc\decomp\decomp-0x1416f24f0.txt |

## 实现方式（算法级）（O 名级 / I 结构）

**sheep_controls @0x1416f11a0**：调试绵羊控制——bevy PickingMode 拾取绵羊实体，
无 FMOD/游戏业务链（O 名）。handle_mouse_picking（行 267，I 内联）处理拾取事件。
**apply_sheep_snapshot @0x1416f24f0**：把快照数据应用到绵羊状态（O 名，逐字段拷贝 U）。
**SheepControl::new @0x1416f0ea0**：调试控制组件构造（O 名）。
- clone_behavior @0x140002370 / register_required_components @0x1416f2910 = bevy 胶水（O）。I

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::debug::sheep_controls::PickingMode::is_some`（行 51）— 谓词/校验
- `country_core::systems::debug::sheep_controls::handle_mouse_picking`（行 267）— 系统:事件响应

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
