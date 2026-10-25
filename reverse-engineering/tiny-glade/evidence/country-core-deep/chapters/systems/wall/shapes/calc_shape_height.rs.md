# systems/wall/shapes/calc_shape_height.rs

> 文件作用：形状预览高度计算：shape{tag=6,ty} ty1=circle/2=rectangle 否则高度 0；as_curve2 到 crashy_resample_remap_per_segment 地形采样；base rect 6.0/circle 8.0；height-ty 2 到 +地形否则 avg+base-0.46；final-(max-0.46) clamp[0.7,14.0] 写预览+0x40 + time_continuous_lerp。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::calc_shape_height::calc_shape_height` | 0x140ac80e0 | 0 | calc_shape_height（base 6.0/8.0、-0.46、clamp[0.7,14.0]、写预览+0x40） | O | wall\decomp\decomp-0x140ac80e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）

**calc_shape_height（O）**：7 参系统（param_5/param_6 带 +0x1c tick 回写 = 2 个 ResMut/Mut）。处理链：

1. **形状门控**：`*param_1` = 形状选择资源 {tag u32@+0, ty u8@+4}。`tag != 6 || (ty-1) > 1`（ty 非 1/2）→ 写 param_6 Mut{tag@+0=0} 并返回（预览禁用）。
2. **建 Curve2**：ty==2（rect）→ `FUN_140c98cf0(&lStack_68, *param_2+0x10)`（Rectangle2d::as_curve2）；ty==1（circle）→ `FUN_140c97cf0(&lStack_68, *param_3+0x10, 0.2f)`（Circle2d::as_curve2，步长 0.2=0x3e4ccccd@0x1429cd068）。param_2/param_3 = PreviewRectangle/PreviewCircle 资源，形状参数在资源 +0x10 处内联。
3. **地形采样**：`FUN_14146afe0(&lStack_a0, &lStack_68, 1.0f, param_4, param_5)`（crashy_resample_remap_per_segment，u=1.0@0x142925870）。输出高度数组（0xc 步长元素，读 +4 f32）。循环求 **sum（fVar12）与 max（fVar11）**（NaN 保护的 max）。
4. **base 高度分支**：tag==6&&ty==2 → `fVar13 = 6.0f`（0x40c00000@0x142934690）；tag==6&&ty==1 → `fVar13 = 8.0f`（0x41000000@0x142934604）；否则 **panic_str**（`FUN_1428d9430(&UNK_142adf6e0, 0x28, &UNK_142adfab0)`，行 40）。
5. **高度公式**：preview.height_ty（*param_5 记录 +8 的 i32）==2 → `fVar13 = base + terrain_sample`（Option<f32>：+0 字节存在位、+4 值，无则 0）；否则 `fVar13 = avg(sum/count) + base + (-0.46)`。再统一 `fVar13 -= (max + (-0.46))`。
6. **钳制**：`clamp(fVar13, 0.7f, 14.0f)`（0x3f333333@0x1429cd030、0x41600000@0x142a7de00）→ 写 `preview + 0x40`（高度）。
7. **插值记录**：`FUN_140c904e0(lVar9, height+max-0.46, param_7+0x28 值, 10.0f)`（rate 10.0=0x41200000@0x1429345a8）。写 param_6 Mut 16B 记录：{tag u32@+0=1, cur f32@+4, prev f32@+8, delta f32@+0xc}。**prev 初值**：旧高（preview+0x3c）==0 时用 **-2.5f32**（0xc0200000@0x1429df1c0）。
8. 释放 4 个栈 Vec（元素 0xc/4B/8B/4B，`func_0x000140613c20`）。

**常量表**：circle base 8.0 @0x142934604、rect base 6.0 @0x142934690、-0.46 @0x142a7f250（0xbeeb851f）、clamp 0.7 @0x1429cd030 / 14.0 @0x142a7de00、圆采样步 0.2 @0x1429cd068、lerp rate 10.0 @0x1429345a8、prev 初值 -2.5 @0x1429df1c0、u 上界 1.0 @0x142925870。
**边界/panic**：ty 非法 → panic_str 行 40（模块路径 UNK_142adfab0）。
