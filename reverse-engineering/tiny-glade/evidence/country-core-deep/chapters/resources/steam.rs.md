# resources/steam.rs

> 文件作用：**Steam 动作集定义：GameControls 28 动作/Menu 2 动作/CottageWindow+GothicWindow 层。**

函数总计 22：业务 9 / 内联无独立符号 9 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::steam::SteamMainActionSet::read_state | 0x1409e86b0 | 67 | SteamMainActionSet::read_state：读当前数字/模拟动作状态→游戏状态结构 | O |  |
| country_core::resources::steam::SteamMenuActionSet::read_state | 0x1409e8bf0 | 67 | SteamMenuActionSet::read_state：读菜单动作集状态 | O |  |
| country_core::resources::steam::SteamMainActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> > | 0x1417c3720 | 102 | SteamMainActionSet::try_load<Input>：动作名表(0x142ce2049：GameControls+28 动作)逐名查句柄→状态结构 | O | audio-misc\decomp\decomp-0x1417c3720.txt |
| country_core::resources::steam::SteamMenuActionSet::try_load<steamworks::input::Input<steamworks::ClientManager> > | 0x1417c5770 | 102 | SteamMenuActionSet::try_load<Input>：Menu 集 ui_tab_prev/ui_tab_next 句柄加载 | O | audio-misc\decomp\decomp-0x1417c5770.txt |
| country_core::resources::steam::SteamMainActionSetState::is_any_active | 0x1409e8aa0 | 128 | SteamMainActionSetState::is_any_active：任一数字/模拟动作非零 | O | audio-misc\decomp\decomp-0x1409e8aa0.txt |
| country_core::resources::steam::impl$30::from | 0x1409e8d80 | 250 | impl$30::from：动作名→C 串转换（name_c_str 内联） | I（胶水） | audio-misc\decomp\decomp-0x1409e8d80.txt |
| country_core::resources::steam::impl$31::from | 0x1409e8d90 | 256 | impl$31::from：动作名转换 | I（胶水） |  |
| country_core::resources::steam::impl$37::from | 0x1409e8da0 | 280 | impl$37::from：动作名转换 | I（胶水） |  |
| country_core::resources::steam::SteamInput::get_action_origin_glyphs | 0x1409e6280 | 385 | SteamInput::get_action_origin_glyphs：动作源字形查询（into_action_value 内联） | I | audio-misc\decomp\decomp-0x1409e6280.txt |

## 实现方式（算法级）（O，DIGEST §2.11）

**SteamMainActionSet::try_load<Input> @0x1417c3720**：动作名表（0x142ce2049 起连续串）：
动作集 `"GameControls"` + **28 个数字/模拟动作**（CameraPan、cursor_move、camera_height、
left_click、right_click、undo、redo、tool_building、tool_path、tool_decorator、tool_customize、
tool_eraser、tool_terrain、tool_garden、tool_tree、tool_water、time_of_day、photomode、
toggle_snapping、zoom_in、zoom_out、main_menu、ui_cancel、ui_confirm、ui_up、ui_left、
ui_right、ui_down）逐名查句柄 → 状态结构。（O）
**SteamMenuActionSet::try_load<Input> @0x1417c5770**：菜单集 `"Menu"`：`ui_tab_prev`/
`ui_tab_next` 句柄加载。（O）
- 另两组 `"CottageWindow"`/`"GothicWindow"`（窗口层，I）。
- `read_state @0x1409e86b0`/`@0x1409e8bf0`：读当前数字/模拟动作状态 → 游戏状态结构（O）；
  `is_any_active @0x1409e8aa0`：任一动作非零（O）。
- `get_action_origin_glyphs @0x1409e6280`：动作源字形查询（into_action_value 内联，I）。
- impl$30/31/37::from：动作名 → C 串转换胶水（I）。O/I 混合

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::steam::SteamMainActionSet::action_handle`（行 89）— 待推断
- `country_core::resources::steam::SteamMenuActionSet::action_handle`（行 89）— 待推断
- `country_core::resources::steam::SteamMenuActionSetState::is_any_active`（行 126）— 谓词/校验
- `country_core::resources::steam::SteamMainActionSetAnalogAction::name_c_str`（行 154）— 待推断
- `country_core::resources::steam::SteamMainActionSetDigitalAction::name_c_str`（行 168）— 待推断
- `country_core::resources::steam::SteamMainActionSetAction::name_c_str`（行 194）— 待推断
- `country_core::resources::steam::SteamMenuActionSetAction::name_c_str`（行 194）— 待推断
- `country_core::resources::steam::SteamActionSetAction::is_analog`（行 231）— 谓词/校验
- `country_core::resources::steam::impl$0::into_action_value`（行 354）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
