# systems/ui/ui_update.rs

> 文件作用：UI 主状态机：遍历 UiState 注册表（HashMap<WidgetId, Arc<WidgetParams>>），每 widget 算 get_ui_volume（世界空间体积）、hover/click/drag 状态转移（HoverState/DragState，按键码 0x5c/0x2f 分支）、PlayUiSound 触发、每帧清交点表。

函数总计 5：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::ui_update::ui_update` | 0x140b2de60 | 0 | 系统:周期性处理（ui_update） | O（75KB decomp）：状态机主体（wp15 合同 + 本域复核）；细节分支见 decomp 行注释 | ui-input\decomp\decomp-0x140b2de60.txt |
| `country_core::systems::ui::ui_update::get_ui_volume` | 0x140b2dc60 | 16 | 计算 widget 世界体积 | O：get_ui_volume（4 callers）——由 WidgetParams 计算世界空间体积 | ui-input\decomp\decomp-0x140b2dc60.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ui::ui_update::enter_hover`（行 295）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
