# resources/color_icon.rs

> 文件作用：**调色菜单图标组件族 + 亮度值枚举（三变体→{0.25,1.0,4.0} 表）。**

函数总计 17：业务 17 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::color_icon::impl$1::register_required_components | 0x140b06b20 | 8 | bevy 组件注册胶水（TypeId 查表样板，×15 图标组件同构） | O（胶水） | audio-misc\decomp\decomp-0x140b06b20.txt |
| country_core::resources::color_icon::impl$2::register_required_components | 0x140b07210 | 25 | 图标组件 bevy 注册胶水（×15 同构） | O（胶水） | audio-misc\decomp\decomp-0x140b07210.txt |
| country_core::resources::color_icon::impl$3::clone_behavior | 0x140002370 | 32 | bevy Component clone_behavior 派生空实现（全局共享地址，非业务） | O（派生胶水） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| country_core::resources::color_icon::impl$3::register_required_components | 0x140b07900 | 32 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b07900.txt |
| country_core::resources::color_icon::impl$4::register_required_components | 0x140b07ff0 | 39 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b07ff0.txt |
| country_core::resources::color_icon::impl$5::register_required_components | 0x140b086e0 | 46 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b086e0.txt |
| country_core::resources::color_icon::impl$6::register_required_components | 0x140b08dd0 | 53 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b08dd0.txt |
| country_core::resources::color_icon::impl$7::register_required_components | 0x140b094c0 | 60 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b094c0.txt |
| country_core::resources::color_icon::ColorMenuBrightnessValue::to_f32 | 0x140b06770 | 78 | 0x140b06770：判别式索引 f32 表 0x142ae785c={0.25,1.0,4.0}；三变体无载荷（重建代码 {Default,Custom(f32)} 错） | O | audio-misc\decomp\decomp-0x140b06770.txt |
| country_core::resources::color_icon::ColorMenuBrightnessValue::default_f32 | 0x140b06780 | 85 | 0x140b06780：常量 1.0（0x142925904）→缺省变体=1 | O | audio-misc\decomp\decomp-0x140b06780.txt |
| country_core::resources::color_icon::impl$14::register_required_components | 0x140b09bb0 | 90 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b09bb0.txt |
| country_core::resources::color_icon::impl$15::register_required_components | 0x140b0a2a0 | 96 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0a2a0.txt |
| country_core::resources::color_icon::impl$16::register_required_components | 0x140b0a990 | 101 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0a990.txt |
| country_core::resources::color_icon::impl$17::register_required_components | 0x140b0b080 | 106 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0b080.txt |
| country_core::resources::color_icon::impl$18::register_required_components | 0x140b0b770 | 111 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0b770.txt |
| country_core::resources::color_icon::impl$19::register_required_components | 0x140b0be60 | 116 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0be60.txt |
| country_core::resources::color_icon::impl$20::register_required_components | 0x140b0c550 | 120 | 图标组件 bevy 注册胶水 | O（胶水） | audio-misc\decomp\decomp-0x140b0c550.txt |

## 实现方式（算法级）（O，DIGEST §2.6 + decomp-0x140b06770/0x140b06780）

**ColorMenuBrightnessValue**（三变体**无载荷**枚举，重建 `{Default,Custom(f32)}` 错）：
- `to_f32 @0x140b06770`：判别式索引 f32 表 0x142ae785c —— `{0:0.25, 1:1.0, 2:4.0}`
  （Quarter/Normal/Quadruple 语义 I）。（O）
- `default_f32 @0x140b06780`：返回常量 1.0（0x142925904）→ 缺省变体=1(Normal)。（O）
- 15×`register_required_components` = bevy 组件注册胶水（TypeId 查表同构样板，非业务）。O
