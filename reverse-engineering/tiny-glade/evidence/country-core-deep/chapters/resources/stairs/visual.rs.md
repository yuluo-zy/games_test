# resources/stairs/visual.rs

> 文件作用：**楼梯可视化资源：MagicSupports/StairsVisualState/地板 mesh 块**

函数总计 11：业务 4 / 内联无独立符号 1 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::stairs::visual::MagicSupports::clear` | 0x140aa8210 | 52 | 系统:写入 | O：MagicSupports::clear | collision-eco\decomp\decomp-0x140aa8210.txt |
| `country_core::resources::stairs::visual::impl$6::register_required_components` | 0x140aacd30 | 99 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） |  |
| `country_core::resources::stairs::visual::StairsVisualState::remove_edge` | 0x140aa8620 | 107 | 系统:写入 | O：StairsVisualState::remove_edge | collision-eco\decomp\decomp-0x140aa8620.txt |
| `country_core::resources::stairs::visual::StairsVisualState::reset_edge` | 0x140aa8a60 | 138 | 系统:写入 | O：StairsVisualState::reset_edge（v1 force_rebuild 名错） | collision-eco\decomp\decomp-0x140aa8a60.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::stairs::visual::StairsVisualState::force_rebuild`（行 132）— I：强制重建视觉（v2 名 reset_edge=0x140aa8a60）

## 实现方式（算法级）

- `MagicSupports::clear`（0x140aa8210）：清空魔法支撑集合（重建/失效时调用）。
- `StairsVisualState::remove_edge`（0x140aa8620）：删除指定边的视觉状态（invalidate 系统经 contains_edge_or_stub 判定后调用）；`reset_edge`（0x140aa8a60）：边视觉重置（v1 force_rebuild 名错）。

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 6 项
