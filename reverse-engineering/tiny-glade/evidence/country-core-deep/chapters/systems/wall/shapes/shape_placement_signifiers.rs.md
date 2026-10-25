# systems/wall/shapes/shape_placement_signifiers.rs

> 文件作用：形状放置 signifier：吸附时 preview_snapping_line（ShapeParameters::expand + Curve::get_tangent_at_u/try_slice）+ 形状 outline；断言 !u.is_nan()/!u.is_infinite()。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line` | 0x1408dfc60 | 0 | shape_placement_signifiers（!u.is_nan()/!u.is_infinite() 断言） | O | wall\decomp\decomp-0x1408dfc60.txt |
| `country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers` | 0x1408dec70 | 0 | shape_placement_signifiers（吸附线 + outline） | O | wall\decomp\decomp-0x1408dec70.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项

## 实现方式（算法级）

**preview_snapping_line（O，0x1408dfc60）**：5 参（param_2 = PublicWallState 记录）。构建"吸附预览线"（形状到吸附点的辅助线）。处理链：

1. 读墙形状：`tag = *(u32*)(param_2 + 600)`（@+0x258）。
2. tag<2（预置）：
   - **circle**：`fVar7 = *(f32*)(param_2+0x264) + 0.315f`（**expand**：半径 +0.315=0x3ea147ae@0x142aa3280）；以 payload 基 +0x25c 处 {center, r} 构参数，`FUN_140c97e10`（Circle2d::as_curve3）→ Curve3，u=1.0。
   - **rect**：dims 读 `+0x26c` qword（hw,hh）+ **expand 常量 0.63**（0x3f2147ae@0x142aa4550/4554，每轴 +0.63=2×0.315）；`FUN_140c991f0`（Rect::as_curve3）+ `FUN_14146a2a0` → Curve3。
3. `FUN_141469af0(&lStack_1a0, ...)` 取曲线法向/切线；`t = clamp(param_5 / 弧长, 0, 1)`；
4. 切线端点 = pos(t) ± t·1.0（沿切线方向偏移）；**断言** `!u.is_nan()` / `!u.is_infinite()`（`FUN_1428d9430(&UNK_142ac7ad0, 0x1d, &UNK_142ac7b48)`，行 29——curve_u.rs 断言族 0x142aa53d0 见 utils 域）。
5. `FUN_1414677f0`（曲线段取点）→ `FUN_140d56700`（画线命令）→ 返回 signifier 流记录；无吸附点（i64::MIN 哨兵 0x8000000000000000）时返回空。

**shape_placement_signifiers（O，0x1408dec70）**：系统入口。调用 `FUN_1408dfc60`（吸附线）×1、`FUN_14146a060`；经 `FUN_140d79f90 ×3 / FUN_140d4ba00 ×3 / FUN_14146c880 ×3`（Commands::spawn + signifier 流）生成吸附线 + 形状 outline signifier。

**关键点**：`+0x25c` 为**扁平 ShapeParameters payload 基**（record+4），`+0x264`=圆半径（payload+0xc）、`+0x26c`=矩形 hw（payload+0x14）——与 valid_enclosure 布局一致（Ghidra 把 +0x25c 的 8B 读误标为指针，实为 {center} 或 {cos,sin} 的不透明拷贝）。
**常量**：圆 expand 0.315 @0x142aa3280、矩形 expand 0.63 @0x142aa4550/4554、u=1.0 @0x142925870。
**边界/panic**：曲线 u 非有限 → panic_str 行 29。
