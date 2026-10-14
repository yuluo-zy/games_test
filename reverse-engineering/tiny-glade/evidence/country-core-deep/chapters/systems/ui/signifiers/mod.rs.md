# systems/ui/signifiers/mod.rs

> 文件作用：Signifier 指示符工厂族：SignifierEntityBuilder 0x90B（默认 shader 'vertex_color_shader'@0x60..0x70、position Vec2@0x7c、默认 {0,1.0f64,0x100 u16@0x40,0.2f32@0x78}）；build 三分支（0=命名网格/1=transient/2=mesh_from_pts）生成实体 + SignifierAnimation；SignifierFactory 40B。

函数总计 22：业务 18 / 内联无独立符号 3 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::signifiers::impl$3::register_required_components` | 0x1408a0f00 | 23 | 组件注册 | O：register_required_components（bevy 胶水） |  |
| `country_core::systems::ui::signifiers::impl$6::clone_behavior` | 0x140002370 | 41 | clone_behavior | O：折叠单行（NotFound） | ui-input\decomp\decomp-0x140002370.txt |
| `country_core::systems::ui::signifiers::impl$6::register_required_components` | 0x1408a15f0 | 41 | register_required_components | O：bevy 组件注册（SignifierAnimation 族） | ui-input\decomp\decomp-0x1408a15f0.txt |
| `country_core::systems::ui::signifiers::impl$10::register_required_components` | 0x1408a1ce0 | 57 | register_required_components | O：bevy 组件注册 | ui-input\decomp\decomp-0x1408a1ce0.txt |
| `country_core::systems::ui::signifiers::impl$11::register_required_components` | 0x1408a23d0 | 65 | register_required_components | O：bevy 组件注册 | ui-input\decomp\decomp-0x1408a23d0.txt |
| `country_core::systems::ui::signifiers::impl$12::register_required_components` | 0x1408a2ac0 | 99 | register_required_components | O：bevy 组件注册 | ui-input\decomp\decomp-0x1408a2ac0.txt |
| `country_core::systems::ui::signifiers::SignifierFactory::new` | 0x14089d8b0 | 112 | 构造（40B 工厂） | O：40B 工厂（拷贝切片；44 callers；wp15 b6 既有） | wp15/analysis/b6_widgets/new_0x14089d8b0.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::new` | 0x14089d9b0 | 153 | 构造（SignifierEntityBuilder::new） | O（wp15 b6）：35 callers，默认值序列见文件作用 | wp15/analysis/b6_widgets/new_0x14089d9b0.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass` | 0x14089dad0 | 173 | set_ui_pass | O：字节@0x8a 置位（UI pass 标志） |  |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::custom_shader` | 0x14089db00 | 184 | custom_shader | O：String 写 @0x60..0x70 | wp15/analysis/b6_widgets/custom_shader_0x14089db00.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::position` | 0x14089dc00 | 189 | position | O：Vec2 @0x7c（24 callers） |  |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation` | 0x14089dc30 | 194 | rotation | O：f32 写（13 callers） | wp15/analysis/b7_signifiers/rotation_0x14089dc30.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::scale` | 0x14089dc60 | 199 | scale | O：f32 写（21 callers） | wp15/analysis/b7_signifiers/scale_0x14089dc60.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration` | 0x14089dc90 | 211 | animation_duration | O：f32 写（35 callers） | wp15/analysis/b7_signifiers/animation_duration_0x14089dc90.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::minimum_lifespan` | 0x14089dcc0 | 217 | minimum_lifespan | O：f32 写 | wp15/analysis/b7_signifiers/minimum_lifespan_0x14089dcc0.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_easing` | 0x14089dcf0 | 222 | animation_easing | O：f32 写 | wp15/analysis/b7_signifiers/animation_easing_0x14089dcf0.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::parent_to_widget` | 0x14089dd20 | 227 | parent_to_widget | O：父挂标志（1 caller） | wp15/analysis/b7_signifiers/parent_to_widget_0x14089dd20.c |
| `country_core::systems::ui::signifiers::SignifierEntityBuilder::build` | 0x14089dd50 | 233 | build（生成 signifier 实体） | O：91 callers——网格选择器三分支 + spawn + SignifierAnimation | wp15/analysis/b7_signifiers/build_0x14089dd50.c |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::signifiers::SignifierAnimation::t`（行 74）— 待推断
- `country_core::systems::ui::signifiers::SignifierEntityBuilder::mark_as_debug`（行 178）— 设置/写入
- `country_core::systems::ui::signifiers::SignifierEntityBuilder::disable_default_animation`（行 206）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone` 等 1 项
