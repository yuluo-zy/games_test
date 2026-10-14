# systems/ui/resources/slider3d.rs

> 文件作用：Slider3d：bevy SystemParam（646 new_archetype callers），包装 UiState/UiSignifierStream/CameraCursorRay/游标相机资源 + 拖拽 EventWriter；Slider3dState 世界空间线段拖拽状态；project_cursor_onto_line 游标投影。

函数总计 17：业务 14 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::resources::slider3d::_::impl$0::apply` | 0x14003bb80 | 35 | SystemParam get_param/queue | O：折叠（NotFound）——SystemParam 状态获取/入队（bevy 胶水面） | ui-input\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::ui::resources::slider3d::_::impl$0::get_param` | 0x1403f7810 | 35 | SystemParam get_param | O：bevy SystemParam 状态获取（胶水面，U 字段细节） |  |
| `country_core::systems::ui::resources::slider3d::_::impl$0::init_state` | 0x140a9be80 | 35 | SystemParam init_state | O：bevy SystemParam 初始化（胶水面） |  |
| `country_core::systems::ui::resources::slider3d::_::impl$0::new_archetype` | 0x14003bb80 | 35 | SystemParam get_param/queue | O：折叠（NotFound）——SystemParam 状态获取/入队（bevy 胶水面） | ui-input\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::ui::resources::slider3d::_::impl$0::queue` | 0x14003bb80 | 35 | SystemParam get_param/queue | O：折叠（NotFound）——SystemParam 状态获取/入队（bevy 胶水面） | ui-input\decomp\decomp-0x14003bb80.txt |
| `country_core::systems::ui::resources::slider3d::_::impl$0::validate_param` | 0x1403f7700 | 35 | SystemParam validate_param | O：bevy SystemParam 校验（胶水面） |  |
| `country_core::systems::ui::resources::slider3d::impl$1::into_drag_state` | 0x14092df10 | 55 | SystemParam validate_param | O：折叠（NotFound）——参数校验（与 UiWidgetContext::dragged 同址折叠） | ui-input\decomp\decomp-0x14092df10.txt |
| `country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged` | 0x14092df10 | 55 | SystemParam validate_param | O：折叠（NotFound）——参数校验（与 UiWidgetContext::dragged 同址折叠） | ui-input\decomp\decomp-0x14092df10.txt |
| `country_core::systems::ui::resources::slider3d::Slider3d::update<enum2$<country_core::systems::ui::resources::ui_state::DragState>,country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims::closure_env$0>` | 0x140b1cc80 | 60 | Slider3d::update<C,F> | O：泛型 update（widget 上下文 + 拖拽闭包；wp15 b7 既有） | wp15/analysis/b7_signifiers/update_enum2_country_core_systems_ui_resources_ui_state_DragState_country_core_systems_wall_shapes_rectangle_ui_edit_rectangle_dims_do_ui_edit_rectangle_dims_closure_env_0__0x140b1cc80.c |
| `country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height::closure_env$0>` | 0x141276b20 | 60 | Slider3d::update 单态 | O：ui_change_stair_height 的拖拽更新单态（system_decorator 域） | ui-input\decomp\decomp-0x141276b20.txt |
| `country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs::closure_env$21>` | 0x141276ca0 | 60 | Slider3d::update 单态 | O：ui_focus_stairs closure$21 单态 | ui-input\decomp\decomp-0x141276ca0.txt |
| `country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs::closure_env$5>` | 0x141276990 | 60 | Slider3d::update 单态 | O：ui_focus_stairs closure$5 单态 | ui-input\decomp\decomp-0x141276990.txt |
| `country_core::systems::ui::resources::slider3d::Slider3d::update<ref$<country_core::systems::ui::resources::ui_state::UiWidgetContext>,system_decorator::ui_systems::ui_focus_stairs::ui_focus_stairs::closure_env$7>` | 0x141276800 | 60 | Slider3d::update 单态 | O：ui_focus_stairs closure$7 单态 | ui-input\decomp\decomp-0x141276800.txt |
| `country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line` | 0x140a9bd40 | 146 | 游标→线段投影 | O：Slider3dState::project_cursor_onto_line（wp15 b7 既有） | wp15/analysis/b7_signifiers/project_cursor_onto_line_0x140a9bd40.c |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::resources::slider3d::Slider3d::origin_ws`（行 84）— 待推断
- `country_core::systems::ui::resources::slider3d::Slider3dState::update`（行 98）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
