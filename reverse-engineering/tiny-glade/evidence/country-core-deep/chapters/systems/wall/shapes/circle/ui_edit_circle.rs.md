# systems/wall/shapes/circle/ui_edit_circle.rs

> 文件作用：圆形编辑 UI：update_circle_radius（拖触发体 WallTriggerVolume::is_valid/edit_shape_default）到 update_wall_state_curve/update_wall_state_enclosure；finish_circle_edit 历史。

函数总计 13：业务 2 / 内联无独立符号 3 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::ui_edit_circle::ui_edit_circle` | 0x14165d460 | 73 | ui_edit_circle（update_circle_radius → curve/enclosure；finish 历史） | O | wall\decomp\decomp-0x14165d460.txt |
| `country_core::systems::wall::shapes::circle::ui_edit_circle::show_full_circle_signifier` | 0x141660880 | 435 | UI 交互 | I | ui-input\decomp\decomp-0x141660880.txt, wall\decomp\decomp-0x141660880.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::circle::ui_edit_circle::finish_circle_edit`（行 463）— 生命周期控制
- `country_core::systems::wall::shapes::circle::ui_edit_circle::update_circle_radius`（行 521）— 系统:周期性处理
- `country_core::systems::wall::shapes::circle::ui_edit_circle::get_trigger_vol`（行 607）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 7 项
