# systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs

> 文件作用：墙高调整模式 UI：clamp_wall_height（机器码 = PublicWalls 查墙到 roof_entity 到 bool；数值钳制策略见 S02 height_rules，名实冲突待裁决）、history_record_adjust_wall_height（ev_height_changed）、get_closest_point_u_to_cursor（points_with_u/segments_with_u 投影）、模式进出 + 光线拾取 + TestCube + 装饰地形 cull。

函数总计 7：业务 4 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height` | 0x140ac79e0 | 0 | history_record_adjust_wall_height（ev_height_changed） | O | ui-input\decomp\decomp-0x140ac79e0.txt, wall\decomp\decomp-0x140ac79e0.txt |
| `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode` | 0x140ac6d60 | 65 | ui_adjust_wall_height_mode（模式进出 + 光线拾取 + TestCube + 装饰 cull） | O | wall\decomp\decomp-0x140ac6d60.txt |
| `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::get_closest_point_u_to_cursor` | 0x140ac7400 | 229 | get_closest_point_u_to_cursor（points_with_u/segments_with_u 投影；panic !curve.points.is_empty()） | O | ui-input\decomp\decomp-0x140ac7400.txt, wall\decomp\decomp-0x140ac7400.txt |
| `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::clamp_wall_height` | 0x140ac7f30 | 321 | clamp_wall_height（机器码=PublicWalls 查墙→roof_entity→bool；数值策略另在 S02 height_rules，名实冲突待裁决） | O | wall\decomp\decomp-0x140ac7f30.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjWallHeightModeCache::new`（行 54）— 构造
- `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::init_mode`（行 171）— 构造/注册/生成

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
