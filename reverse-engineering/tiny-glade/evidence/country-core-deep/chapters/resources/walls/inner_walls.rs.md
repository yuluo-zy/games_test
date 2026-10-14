# resources/walls/inner_walls.rs

> 文件作用：内部墙集合资源：HashMap<WallId, WallState 0x70>（条目步长 0x78）+ +0xc0 脏代计数器 + +0x30 墙工厂子结构；new_wall/new_wall_with_id/get/get_unwrapped/remove/max_id/reset_max_id 与 _try_update_state 闭包族（6+3 实例，写偏移即 WallState 布局证据）。

函数总计 58：业务 26 / 内联无独立符号 16 / 派生胶水 16

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::inner_walls::InnerWalls::new` | 0x140859f90 | 0 | InnerWalls::new | O | wall\decomp\decomp-0x140859f90.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve` | 0x14085a200 | 47 | InnerWalls::wall_ids（+0xc0 脏代计数递增 + map 遍历） | O | wall\decomp\decomp-0x14085a200.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params` | 0x14085a370 | 59 | InnerWalls::new_wall_w_params（参数化建墙） | O | wall\decomp\decomp-0x14085a370.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id` | 0x14085a4a0 | 80 | InnerWalls::new_wall_with_id（FUN_141529bb0 收集 + 尾调 FUN_1402ca2f0(self+0x30,id) 工厂） | O | wall\decomp\decomp-0x14085a4a0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::reset_max_id` | 0x14085a550 | 94 | InnerWalls::reset_max_id | I | wall\decomp\decomp-0x14085a550.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::get` | 0x14085a560 | 107 | InnerWalls::get | O | wall\decomp\decomp-0x14085a560.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped` | 0x14085a650 | 116 | InnerWalls::get_unwrapped | O | wall\decomp\decomp-0x14085a650.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::remove_all` | 0x14085a730 | 120 | InnerWalls::max_id | O | wall\decomp\decomp-0x14085a730.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::remove` | 0x14085aa60 | 126 | InnerWalls::remove | O | wall\decomp\decomp-0x14085aa60.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_arch_height` | 0x14085ad20 | 148 | InnerWalls::update_arch_height（→ _try_update_state<update_arch_height> 0x140aaa710） | O | wall\decomp\decomp-0x14085ad20.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_arch_type` | 0x14085ad40 | 154 | InnerWalls::update_arch_type（→ 0x140aa9eb0 写 WallState+0x64） | O | wall\decomp\decomp-0x14085ad40.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height` | 0x14085ad60 | 202 | InnerWalls::update_wall_state_height（→ 0x140aaaca0 写 WallState+0x54） | O | wall\decomp\decomp-0x14085ad60.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::_deprecated_update_skirt::closure_env$0>` | 0x140aaa350 | 208 | _try_update_state<update_plaster_peel> 实例化 | I | wall\decomp\decomp-0x140aaa350.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::mark_as_non_enclosed::closure_env$0>` | 0x140aaa8f0 | 208 | _try_update_state<mark_as_non_enclosed>（写 enclosure tag 2 = Freehand） | O | wall\decomp\decomp-0x140aaa8f0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_arch_height::closure_env$0>` | 0x140aaa710 | 208 | _try_update_state<update_arch_height>（写偏移待解，候选 arch_height f32） | I | wall\decomp\decomp-0x140aaa710.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_arch_type::closure_env$0>` | 0x140aa9eb0 | 208 | _try_update_state<update_arch_type>（写 WallState+0x64 u8） | O | wall\decomp\decomp-0x140aa9eb0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_elevation::closure_env$0>` | 0x140aaaac0 | 208 | 更新 | I | wall\decomp\decomp-0x140aaaac0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_freehand_ty::closure_env$0>` | 0x140aa99e0 | 208 | _try_update_state<update_freehand_ty>（写 WallState+0x60 u8） | O | wall\decomp\decomp-0x140aa99e0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_plaster_peel::closure_env$0>` | 0x140aaa530 | 208 | _try_update_state<update_style> 实例化 | I | wall\decomp\decomp-0x140aaa530.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_style::closure_env$0>` | 0x140aaa350 | 208 | _try_update_state<update_plaster_peel> 实例化 | I | wall\decomp\decomp-0x140aaa350.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_supports::closure_env$0>` | 0x140aa9800 | 208 | _try_update_state<update_supports>（写 WallState+0x67 removed_supports u8） | O | wall\decomp\decomp-0x140aa9800.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state::closure_env$0>` | 0x140aa9bc0 | 208 | _try_update_state 实例化（bulldoze 增长路径） | I | wall\decomp\decomp-0x140aa9bc0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_curve::closure_env$0>` | 0x140aaa090 | 208 | _try_update_state<update_wall_state_curve>（drop 旧双 Vec 换入新曲线） | O | wall\decomp\decomp-0x140aaa090.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_enclosure::closure_env$0>` | 0x140aa9610 | 208 | 更新 | I | wall\decomp\decomp-0x140aa9610.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_height::closure_env$0>` | 0x140aaaca0 | 208 | _try_update_state<update_wall_state_height>（写 WallState+0x54 f32） | O | wall\decomp\decomp-0x140aaaca0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::load_state` | 0x14085ad80 | 218 | IO/解析 | I | wall\decomp\decomp-0x14085ad80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::inner_walls::InnerWalls::wall_ids`（行 43）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::new_wall`（行 69）— 构造
- `country_core::resources::walls::inner_walls::InnerWalls::max_id`（行 89）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::set_max_id`（行 99）— 设置/写入
- `country_core::resources::walls::inner_walls::InnerWalls::all`（行 103）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::contains`（行 111）— 谓词/校验
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state`（行 136）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_elevation`（行 142）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_style`（行 160）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_supports`（行 166）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel`（行 172）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_freehand_ty`（行 178）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve`（行 184）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::mark_as_non_enclosed`（行 190）— 设置/写入
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure`（行 196）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::_deprecated_update_skirt`（行 241）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 16 项
