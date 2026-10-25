# systems/ui/resources/widget_params.rs

> 文件作用：WidgetParams 0xb0B 参数块：默认 u32 组 {0,8,0,1,0,2,3,3}@{0,8,0x10,0x18,0x20,0x30,0x48,0x60}、UiSounds 4×24B、WidgetVolumeShape 32B {tag,layer,size}、u16@0xa8=0x101、flags 字节@0xa9（bit0=no_history_recording、bit2=allow_negative_distance）；UiFocus u32 {None=0,Toolbar=1,Mei=2,World=3}。

函数总计 35：业务 20 / 内联无独立符号 4 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::resources::widget_params::WidgetParams::new` | 0x1409a8e10 | 66 | 构造（WidgetParams::new） | O：0xb0B 默认值序列（wp15 合同 + 本域复核） | hex 手解（NotFound；u32 {0,8,0,1,0,2,3,3}@{0,8,0x10,0x18,0x20,0x30,0x48,0x60} + u16@0xa8=0x101，默认数据拷贝 @0x78..0xa0） |
| `country_core::systems::ui::resources::widget_params::WidgetParams::sounds` | 0x1409a8e80 | 77 | 构造器（sounds） | O：写 96B sounds 区 @0x18..0x78 | ui-input\decomp\decomp-0x1409a8e80.txt（已补） |
| `country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_decorator::ui_color_windows_and_flags::closure_env$3>` | 0x1422bcec0 | 83 | WidgetParams::signifier 单态 | O：→ into_signifier_factory（ui_color_decorator 域闭包单态） |  |
| `country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$0>` | 0x1422bc980 | 83 | WidgetParams::signifier 单态 | O：→ into_signifier_factory（ui_color_wall_and_roof 域） | ui-input\decomp\decomp-0x1422bc980.txt |
| `country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_color::ui_color_wall_and_roof::ui_color_wall_and_roof::closure_env$2>` | 0x1422bcc20 | 83 | WidgetParams::signifier 单态 | O：同上（closure$2） | ui-input\decomp\decomp-0x1422bcc20.txt |
| `country_core::systems::ui::resources::widget_params::WidgetParams::signifier<system_tree::ui_move_tree::ui_move_tree::closure_env$0>` | 0x1425df760 | 83 | WidgetParams::signifier 单态 | O：同上（ui_move_tree 域） | ui-input\decomp\decomp-0x1425df760.txt |
| `country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world` | 0x1409a8ee0 | 92 | 构造器（occlude_by_world） | O：字节@0xa8 **整字节 bool 字段**（mov [rdx+0xa8],r8b；非 bit0；review-audit 纠错） | ui-input\decomp\decomp-0x1409a8ee0.txt（已补） |
| `country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording` | 0x1409a8f10 | 107 | 构造器（no_history_recording） | O：字节@0xa9 bit0 | ui-input\decomp\decomp-0x1409a8f10.txt |
| `country_core::systems::ui::resources::widget_params::WidgetParams::allow_negative_distance` | 0x1409a8f40 | 112 | 构造器（allow_negative_distance） | O：字节@0xa9 bit2 | ui-input\decomp\decomp-0x1409a8f40.txt |
| `country_core::systems::ui::resources::widget_params::UiSounds::boop` | 0x1409a8f80 | 166 | UiSounds::boop | O：折叠单行（NotFound；boop 音效槽） | ui-input\decomp\decomp-0x1409a8f80.txt |
| `country_core::systems::ui::resources::widget_params::UiSounds::empty` | 0x1409a8fb0 | 175 | UiSounds::empty | O：清 4 槽首 u32（偏移 0/0x18/0x30/0x48；wp15 b6 既有） | wp15/analysis/b6_widgets/empty_0x1409a8fb0.c |
| `country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start` | 0x1409a8fd0 | 184 | UiSounds::on_hover_start | O：折叠（NotFound；槽 0-2） | ui-input\decomp\decomp-0x1409a8fd0.txt |
| `country_core::systems::ui::resources::widget_params::UiSounds::on_click` | 0x1409a9020 | 189 | UiSounds::on_click | O：折叠（NotFound；槽 9-11） | ui-input\decomp\decomp-0x1409a9020.txt |
| `country_core::systems::ui::resources::widget_params::UiSounds::on_drag_start` | 0x1409a9070 | 194 | UiSounds::on_drag_start | O：写槽 3-5（wp15 b6 既有） |  |
| `country_core::systems::ui::resources::widget_params::UiSounds::on_drag_release` | 0x1409a90c0 | 199 | UiSounds::on_drag_release | O（round3 round2 复查修正：原“折叠（NotFound）”证伪）：真实 70B 结构复制/重排（槽 0..0x50 复制；[+0x30]=[src+0]、[+0x40]=[r8+0x10] 替换、0x48/0x50 直拷；“槽 6-8”与触及 0x30..0x48 部分吻合） | rne-read 0x1409a90c0；PDB size 70 |
| `country_core::systems::ui::resources::widget_params::WidgetVolumeShape::clearing_canvas` | 0x1409a9110 | 329 | 构造（WidgetVolumeShape::clearing_canvas） | O：{tag 4, layer, 130×130} 32B | hex 手解（NotFound；`mov [rax+1],dl`、`mov rcx,0x4302000002430000`（双 130.0 f32）→[+4]、`mov byte[rax],4`） |
| `country_core::systems::ui::resources::widget_params::UiFocus::is_none` | 0x140985f30 | 350 | UiFocus::is_none | O：== 0（u32 判别；与 is_null 折叠） |  |
| `country_core::systems::ui::resources::widget_params::UiFocus::as_mei` | 0x1409a9130 | 363 | UiFocus::as_mei | O：转 Mei（=2） | wp15/analysis/b6_widgets/as_mei_0x1409a9130.c |
| `country_core::systems::ui::resources::widget_params::UiFocus::is_mei` | 0x1409a9140 | 375 | UiFocus::is_mei（与 WelcomeScreenState::is_closed 折叠） | O：`== 2`（u32 判别；折叠=两处语义巧合相同） | ui-input\decomp\decomp-0x1409a9140.txt |
| `country_core::systems::ui::resources::widget_params::UiFocus::is_toolbar` | 0x1409a9150 | 383 | UiFocus::is_toolbar | O：`== 1`（u32 判别） | ui-input\decomp\decomp-0x1409a9150.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::resources::widget_params::WidgetFlags::record_history_when_used`（行 44）— 渲染/GPU
- `country_core::systems::ui::resources::widget_params::WidgetParams::with_flags`（行 117）— 构造
- `country_core::systems::ui::resources::widget_params::impl$2::into_signifier_factory`（行 134）— 待推断
- `country_core::systems::ui::resources::widget_params::WidgetVolumeShape::get_origin`（行 312）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`cmp`、`default`、`eq`、`fmt` 等 11 项

## 实现方式（算法级）

- `WidgetParams::new` @0x1409a8e10（O）：0xb0B 默认值序列——u32 `{0,8,0,1,0,2,3,3}`@{0,8,0x10,0x18,0x20,0x30,0x48,0x60}；sounds 区 @0x18..0x78（4×24B UiSounds）；volume @0x78（32B WidgetVolumeShape）；`u16@0xa8=0x101`。
- 构造器（O）：`sounds` @0x1409a8e80 写 96B sounds 区；`occlude_by_world` @0x1409a8ee0 = `mov [rdx+0xa8],r8b` 整字节 bool@0xa8（review-audit 纠错：非 bit0）；`no_history_recording` @0x1409a8f10 = 字节@0xa9 bit0；`allow_negative_distance` @0x1409a8f40 = 字节@0xa9 bit2。
- UiSounds 槽写入（O）：`empty` @0x1409a8fb0 清 4 槽首 u32（@0/0x18/0x30/0x48）；`boop` @0x1409a8f80 / `on_hover_start` @0x1409a8fd0（槽0-2）/ `on_click` @0x1409a9020（槽9-11）/ `on_drag_start` @0x1409a9070（槽3-5）/ `on_drag_release` @0x1409a90c0（round3：真实 70B 结构复制/重排，槽0..0x50）。
- `WidgetVolumeShape::clearing_canvas` @0x1409a9110（O）：`{tag 4, layer, 130×130}` 32B。
- `UiFocus` u32 枚举（O）：`is_none` @0x140985f30（==0）、`as_mei` @0x1409a9130（=2）、`is_mei` @0x1409a9140（==2，与 WelcomeScreenState::is_closed 同址折叠）、`is_toolbar` @0x1409a9150（==1）；变体序 None=0/Toolbar=1/Mei=2/World=3（I）。
- `WidgetParams::signifier` 单态 @0x1422bcec0/0x1422bc980/0x1422bcc20/0x1425df760（O）：→ `into_signifier_factory`（ui_color / ui_move_tree 闭包单态）。
