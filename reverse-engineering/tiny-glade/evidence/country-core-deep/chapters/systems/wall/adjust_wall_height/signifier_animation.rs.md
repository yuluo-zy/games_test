# systems/wall/adjust_wall_height/signifier_animation.rs

> 文件作用：墙高 signifier 动画：平屋顶无 rim（Roof::is_flat_w_no_rim）到 adjust_wall_height_curve 重算 + crashy_new_from_points；否则 SignifierAnimation::t 驱动。

函数总计 8：业务 2 / 内联无独立符号 1 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::adjust_wall_height::signifier_animation::impl$1::register_required_components` | 0x14085d900 | 16 | 构造/注册/生成 | U |  |
| `country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation` | 0x14085bb50 | 39 | signifier_animation（flat_w_no_rim → 重算曲线；否则 SignifierAnimation::t） | O | wall\decomp\decomp-0x14085bb50.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier::new`（行 27）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 5 项

## 实现方式（算法级）

### ui_adjust_wall_height_signifier_animation 0x14085bb50（O）
- **遍历源**：param_1 双路迭代器（lVar5=*param_1，+0xf8/+0x100 项索引、+0x108 模式字节）：模式==0 走 **Roofs 数组**（lVar6+0x100，元素 0xa0B：+0x94 索引、+0x88 计数、+0x80 实体表、+0x50）；模式!=0 走 roof-map 数组（lVar6+0x1a8，元素 0x48B）。每元素解析出 4 个并行数组：`0x60B 步长（动画状态 plVar37）、0x1cB 步长（u 记录 lVar36，+0x18 存 t f32）、0x8B 步长、f32 u 数组`。
- **PublicWalls 查墙**（param_3，0x2a8 步长）：`raw 计数(value+0x10)<2 或 raw 长度(value+0x30)<=0` → 断言 `w.simplified_curve_2d.is_valid()`（signifier_animation.rs:50，UNK_142ada193/142a95208）。
- **flat_w_no_rim 判定**：`roof_entity(0x140a923a0)!=0 && roof+0x30(高度01)<=0.1 && roof+0x3d 字节!=0` → `fVar42 = -0.29`（_UNK_142a94920），否则 0。
- **重算曲线**：读墙 `points@value+0x90、heights@value+0xe0、heights 长度@value+0xe8`（取 min(ptr, len) 为拷贝量）；分配 `len×0xc` Vec3；`out = {x=points.x, y=heights[i]+fVar42, z=points.z}`；`FUN_141469230`（曲线构造）。
- **动画参数 u**（SignifierAnimation::t）：状态 `plVar37+0x38 字节` 判定：已动画态 → u 取 `plVar37+0x3c` 存储值或 `clamp01(FUN_141469950)`（曲线取 u）；否则默认 **0.5**。`new_t = 1.0 - clamp01(u ± 0.01)`（+0.01=0x142960674 / -0.01=0x142a82410 双路径）；`t>0.95（0x142a7d8d0）` 分支跳过动画；两路结果 `out = t0 + (1-t0)*t1` 合成。u 校验：越界 → `panic!("Tried to init CurveU from an invalid u: ")`（UNK_142aa53c0，fmt），NaN → `!u.is_nan()`（UNK_142aa53d0，line 0x1d）。
- **曲线切片**：`FUN_141466300(plVar37, u_lo, u_hi, &out)` 按 u 区间切曲线，写回动画状态（0x60B 步长 plVar37 区）。
- 常量：0.5 默认、0.95 阈值、±0.01 步进、-0.29 平屋顶偏移、0.1 高度01 阈值、1.0。
