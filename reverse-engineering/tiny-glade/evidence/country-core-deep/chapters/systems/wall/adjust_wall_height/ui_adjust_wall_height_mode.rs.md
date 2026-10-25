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

## 实现方式（算法级）

### clamp_wall_height 0x140ac7f30（O）
- 入参：`(?, WallId, PublicWalls map, roofs)`；param_1 未用。PublicWalls 0x2a8 步长查墙：
  - 命中且 `raw 计数(value+0x10)>1 && raw 长度(value+0x30)>0.0` → 调 `PublicWalls::roof_entity(0x140a923a0)` → **返回 1（bool），XMM0 残留 = 钳制高**（被 impl 用 extraout 捕获）。
  - 命中但曲线无效 → `panic!("assertion failed: w.simplified_curve_2d.is_valid()")`（ui_adjust_wall_height_mode.rs:50，UNK_142ada193/142adfa58）。
  - 未命中 → 0。
- 数值策略：钳制高由调用方（impl）从 XMM0 取；实际 clamp 数值在 **history_record_adjust_wall_height**（见下）与 adjust_wall_height_impl 消费。**纠错**：断言文件指针=ui_adjust_wall_height_mode.rs（非 public_walls.rs），两处 inline 各自编译。

### ui_adjust_wall_height_mode 0x140ac6d60（O）
- 两个 EventReader（param_4 事件源 16B 行、param_3 事件源）→ 对每墙：PublicWalls 0x2a8 查墙 → `raw 计数<2 或 raw 长度<=0` → 断言 `simplified_curve_2d.is_valid()`（UNK_142ada193/142adf980）。
- 命中 → `func_0x000140c906d0()`（曲线 u 采样辅助）→ `AdjWallHeightModeCache`（puVar25，位标志@+2、u 缓存@+0x14/+0x24/+0x2c）未填时调 **get_closest_point_u_to_cursor(0x140ac7400)** 填充；随后写 signifier 事件 `FUN_1413e77f0(inner, &cache, 2, decorators, ...)`（UNK_142adf9b0，行 18）+ `decorator_cull_allow_any(0x140b2bc00)` + `FUN_141293280(&{1,entity}, inner+0x30)`。
- **曲线采样+预览事件**：`FUN_141467ce0(value+0x38 曲线)`（简化曲线，Vec3 点@value+0x40、计数@+0x48）取段 → 点间插值（uVar13/uVar15 相邻点，`y = lerp(z_i,z_j,frac)`）→ `FUN_140ac84a0`（高度换算）→ 写 **0x30B 预览事件** `{1@0, pos@+4, height@+0xc, 1@+0x10, delta@+0x14, WallId@+0x18, 0@+0x20, +0x28}`（FUN_1405629f0 扩容）。delta = `(FUN_140ac84a0 结果 + 缓存高) - 光标 y`。
- **PublicWallState 新增偏移**：`+0x38 简化曲线（FUN_141467ce0 入参）、+0x40 简化曲线点 ptr（Vec3 12B）、+0x48 点计数`（I，与 DIGEST 布局表互补）。

### get_closest_point_u_to_cursor 0x140ac7400（O；PDB 名 init_mode 错位）
- 入参：`(out, cursor_x, cursor_y, curve{points@+8(12B Vec3), count@+0x10, u 数组@+0x20(f32), u 数@+0x28}, mat3 param_5, cursor_y2)`。
- `count==0` → `panic!("assertion failed: !curve.points.is_empty()")`（ui_adjust_wall_height_mode.rs:42，UNK_142adf9e0/142adfa28）。
- **逐段投影**：对每个点 i（窗口 p[i],p[i+1],p[i+2] 三段几何）：mat3 仿射变换（param_5 读 12×f32 + fVar3/fVar4/fVar5 标量、`func_0x000140c96e60` 缩放）→ 齐次除（fVar17/fVar24）→ 光标到线段投影 t=clamp01(dot/|d|²) → 插值点、距离；**最近段**记录 best distance（初值 f32 max 0x7f7fffff @0x142925980）与 u=`lerp(u_i,u_{i+1},t)`（func_0x000140c904d0），t 归一 `lerp(1/count 参, 1/len, t)` 修正。
- 返回 `{u f32, pos Vec2, f32}`；u 越界[0,1] → `panic!("Tried to init CurveU from an invalid u: ")`（fmt，UNK_142aa53c0/142adfa10），NaN → `!u.is_nan()`（line 0x1d）。

### history_record_adjust_wall_height 0x140ac79e0（O）
- 输入事件（0x30B，ev_height_changed）：`flag@0、WallId@+8、ty@+0x20==1、h1 f32@+0x24、bit@+0x28、h2 f32@+0x2c`；三者不符 → 跳过。
- **高度钳制规则**（两次，h1/h2 各一）：`rule_min = 有屋顶 ? (roof+0x30<=0.1 ? 0.7 : 1.6) : (墙 style 字节@value+0x296==0 ? 0.5 : 0.3)`；`rule_max = style==0 ? 14.0 : 2.5`；`clamped = min(max(h, rule_min), rule_max)`（常量 0.7@0x142adf490、1.6@+4、0.5@0x142925870、0.3@0x1429258d8、2.5@0x1429cd064、14.0@0x142a7de00、0.1@0x142925984）。
- 两次钳制结果不等或 NaN → 记历史：`FUN_140a83fb0`（构造 pending）+ `CompoundEditBuilder 提交 FUN_140a84ec0(&UNK_142ad7e18=wall_history.rs)`；且预置形状（`shape tag(value+0x258)<2`）→ push WallId 到 ResMut Vec（8B 元素，FUN_140562870 扩容）。
- 处理完将事件行清零（pbVar5[0..0x2b]=0）。
- **纠错**：style 字节读位为 value+0x296（非 DIGEST 旧述 +0x299，两者可能同字段不同版本，待统一）；钳制规则即"墙高上下限"，非屋顶高度 clamp 本身。
