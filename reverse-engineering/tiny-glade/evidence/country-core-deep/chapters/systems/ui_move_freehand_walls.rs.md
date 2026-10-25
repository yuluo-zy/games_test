# systems/ui_move_freehand_walls.rs

> 文件作用：移动自由墙：拖拽移动选中自由墙段，MoveWallCache 记录在途移动（wp20 合同）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls` | 0x1409781d0 | 47 | 系统:周期性处理（ui_move_freehand_walls） | O：7.8KB（wp20 既有复核） | wp20/analysis/0x1409781d0_ui_move_freehand_walls.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

- `ui_move_freehand_walls` @0x1409781d0（O，7.8KB，wp20 既有复核）：移动自由墙段。
  - 拖拽移动选中自由墙段；`MoveWallCache` 记录在途移动（I）。
  - widget 名 "move_freehand_wall" @0x142ab6a28 区（串表 0x142ab6a0f.. 区间，含 `add_support_`/`move_shape_line`/`move_shape_outline` 等）；自由墙实体 SIMD 匹配（O，ALGORITHMS §2.7）。
  - panic 位置串 `crates/.../ui_move_freehand_walls.rs` @0x142ab6a10 区（O）。
  - 算法级主体（墙段位移/贴合）在 wall 域（I）。
