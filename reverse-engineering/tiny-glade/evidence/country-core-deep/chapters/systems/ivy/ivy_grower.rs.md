# systems/ivy/ivy_grower.rs

> 文件作用：**常春藤生长：10 相状态机（grower 7.9KB 单函数）沿墙推进折线**

函数总计 11：业务 4 / 内联无独立符号 3 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_grower::ivy_grower` | 0x14094a910 | 47 | 系统:生长 | O：ivy_grower（0x14094a910..0x14094c7fb，7915B 单函数）——10 相状态机（跳表 0x142ab3be0，索引 world+0x118）；主闭包 0x14094ac20：last_point→get_direction→closest_segment_index→墙跳→add_point | collision-eco\decomp\decomp-0x14094a910.txt |
| `country_core::systems::ivy::ivy_grower::closest_segment_index` | 0x14094c800 | 401 | 几何/碰撞 | O：closest_segment_index（666B）——**56B 步长**墙段（循环 `add r9,0x38`；xy0@0/xy1@8/h0@+0x18/h1@+0x1c）+ 高度≤max_ivy_grow_height 过滤（h0<=max 或 h1<=max 跳过）【review-audit 纠错：步长 28→56、h0@+0xc→+0x18】 | collision-eco\decomp\decomp-0x14094c800.txt |
| `country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal` | 0x14094caa0 | 471 | 几何/碰撞 | O：get_wall_double_sided_normal——双面法线（v2 名址） | collision-eco\decomp\decomp-0x14094caa0.txt |
| `country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment` | 0x14094cc60 | 513 | 几何/碰撞 | O：intersect_ivy_growth_w_wall_segment——生长线是否穿墙段（v2 名址） | collision-eco\decomp\decomp-0x14094cc60.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ivy::ivy_grower::get_growth_point_proposal`（行 227）— 查询/计算
- `country_core::systems::ivy::ivy_grower::check_for_wall_jump`（行 308）— 谓词/校验
- `country_core::systems::ivy::ivy_grower::get_ivy_x_direction_relative_to_wall_direction`（行 491）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
