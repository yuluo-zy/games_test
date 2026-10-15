# resources/roofs/roof_shape.rs

> 文件作用：**屋顶形状权威类型**——`Roof` 组件（0x58 布局）、`RoofShapeParams`（28 字节参数块）、
> `GableRoofStyle`/`FlatRoofStyle` 风格枚举（JSON 字符串 "Matching"/"Override" 变体）、RoofSection 截面枚举与全部几何查询。
> **28 个业务函数中绝大多数已由 reconstruction/roof 恢复**（surface.rs/ridge.rs/roof_state.rs + evidence/roof-surface、roof-ridge、roof-tiles），
> 本域逐函数标映射不重挖；未入重建的（roof_uv 族、serde、style 谓词、pivot/rotation）给出定点反编译/手解结论。
> panic 串确认源码 `crates/country-core/src/resources/roofs/roof_shape.rs`；依赖 `crates/utils/src/curve/mod.rs`（Curve2）。

函数总计 61：业务 28 / 内联无独立符号 4 / 派生胶水 29

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `roof_uv_and_point_at_ws_pos` | 0x1408e4090 | 0 | 世界位置→{roof uv, 点}：经哈希表（FUN_1405b0d70=collider 注册表）找 roof collider，调用 FUN_1408ce4a0/0x140c9cde0/0x140c91b60/0x140b93940 求 uv；panic "collider exists in the physics world but not its collider registry"（源 systems/collision/world.rs） | O | roofs-rt/decomp/decomp-0x1408e4090.txt |
| `deserialize<Value>` | 0x1416e8680 | 48 | 风格枚举 serde 反序列化（JSON，变体串比较） | O | roofs-rt/decomp/decomp-0x1416e8680.txt |
| `GableRoofStyle::is_stepped` | 0x140833230 | 62 | `tag==0`（`cmp byte[rcx],0; sete`；该字节序列与 wall `FreehandHeightType::is_follow_terrain` 同一函数去重体） | O | 手解（rne-read --hex）+ wall\decomp 对照 |
| `GableRoofStyle::is_overhang` | 0x1408c5680 | 66 | `tag!=0`（`movzx byte[rcx]`；与 wall `StructureType::is_fence` 同去重体） | O | 手解（rne-read --hex）+ wall\decomp 对照 |
| `deserialize::visit_str<Error>` | 0x1416e98c0 | 70 | serde visitor：按串长匹配枚举变体（"Matching"/"Override"） | O | roofs-rt/decomp/decomp-0x1416e98c0.txt |
| `serialize<CompactFormatter>` | 0x140259760 | 70 | 风格枚举 serde 序列化（compact Vec） | O | roofs-rt/decomp/decomp-0x140259760.txt |
| `serialize<Value::Serializer>` | 0x1416e8f20 | 70 | 风格枚举 serde 序列化（Value 序列化器） | O | roofs-rt/decomp/decomp-0x1416e8f20.txt |
| `FlatRoofStyle::is_wooden` | 0x1408e31a0 | 93 | 机器码手解：`if tag==0 ret *other; if tag==1 ret 0; ret tag==2? field@+1`——按变体返回木质标志（变体 1 恒 false） | I | 手解（rne-read --hex 0x1408e31a0） |
| `impl$37::clone_behavior` | 0x1408e6af0 | 137 | Roof 组件 CloneBehavior：返回 `(Some(closure@0x14113144), 2)`（`lea rdx,closure; mov eax,2`）；非 0x140002370 空桩 | O | 手解（rne-read --hex 0x1408e6af0） |
| `impl$37::register_required_components` | 0x1408e6400 | 137 | Bevy 注册胶水（FUN_140d40d70 + required components 追加） | O（胶水） | roofs-rt/decomp/decomp-0x1408e6400.txt |
| `Roof::new` | 0x1408e31c0 | 152 | 28 字节参数块拷到 +0x24..+0x40，几何体 +8..+0x24，floor@+0x4c，动画初态@+0x50，tag@+0x54，帧体 +0x40..+0x4c；不写 +0x55..0x57。**= roof_state.rs `new_observed`** | O | 已由 reconstruction/roof/roof_state.rs 恢复；roofs-rt/decomp/decomp-0x1408e31c0.txt |
| `Roof::pivot_3d` | 0x1408e3220 | 172 | `{frame[+0xc+tag*8], frame[+0x10+tag*8], floor_y(+0x4c)}`——按 tag 取 xz 枢轴中心 + 楼层高 | O | 手解（rne-read --hex 0x1408e3220） |
| `Roof::ty` | 0x1408e3250 | 177 | `height_01 > 0.1`（UCOMISS 无序含 NaN）。**= roof_state.rs `ty_observed`** | O | 已由 reconstruction/roof/roof_state.rs 恢复 |
| `Roof::height` | 0x1408e3260 | 186 | `h≤0.1→0`；否则 `h*12+(1-h)*0.4`，受尺寸相关 smoothstep 上限（circle 8→12，rect 9→12）约束。**= surface.rs `height`**（2,500 案例验证） | O | 已由 reconstruction/roof/surface.rs 恢复；roofs-rt/decomp/decomp-0x1408e3260.txt |
| `Roof::is_flat` | 0x1408e3380 | 193 | `height_01 <= 0.1`（`ucomiss 0.1,[+0x30]; setae`） | O | 手解（rne-read --hex 0x1408e3380） |
| `Roof::is_flat_w_no_rim` | 0x1408e3390 | 200 | `is_flat && byte@+0x3d == 0`（rim 标志） | O | 手解（rne-read --hex 0x1408e3390） |
| `Roof::is_gable` | 0x1408e33b0 | 207 | `ty && ridge==1.0`（CMPNLE+CMPEQ 与）。**= roof_state.rs `is_gable_observed`** | O | 已由 reconstruction/roof/roof_state.rs 恢复 |
| `Roof::gable_sides` | 0x1408e33e0 | 212 | 返回 `{sides[0]=dir_byte, sides[1]=dir_byte\|2, found=(ridge==1 && ty)}`——两轴侧序号位掩码 | O | evidence/roof-tiles/0x1408e33e0.c（已拷 roofs-rt/decomp） |
| `Roof::profile_curve_normalized` | 0x1408e3420 | 225 | 生成 20 个原始 f32 常量 u，纵坐标 u^q（q=1.5p+(1-p)0.5），末点强制 (1,1)。**= surface.rs `profile_curve_normalized`**（209 案例） | O | 已由 reconstruction/roof/surface.rs 恢复；roofs-rt/decomp/decomp-0x1408e3420.txt |
| `Roof::roof_tip_offset_xz` | 0x1408e3780 | 241 | circle→2r*tip*0.5；rect→旋转(tip*dim*0.5)。**= ridge.rs `roof_tip_offset_xz_observed`** | O | 已由 reconstruction/roof/ridge.rs 恢复；evidence/roof-tiles/0x1408e3780.c + roof-ridge/tg_...roof_tip_offset_xz.c |
| `Roof::shape_dims` | 0x1408e3800 | 251 | circle→{2r,2r}（+0x14 加倍）；rect→{dim@+0x1c, dim@+0x20} | O | evidence/roof-surface/tg_...Roof_shape_dims.c（已拷 roofs-rt/decomp）+ 手解 |
| `Roof::roof_shape_at_intersection_y` | 0x1408e3820 | 260 | 高度 y 处截面：y 越界→tag 2；圆→0x140c97560 截面（tag 0），矩→0x140c98980（tag 1）；y==floor 时直接拷帧体 | O | roofs-rt/decomp/decomp-0x1408e3820.txt（截面构造器 0x140c97560/0x140c98980 见 evidence/roof-tiles/*.c） |
| `Roof::rotation_yaw_radians` | 0x1408e3b10 | 298 | 矩形→`atan2f(frame.z^sign, frame.x)`；圆→角度函数后取补角 | O | roofs-rt/decomp/decomp-0x1408e3b10.txt |
| `ridge_length_01_from_rectangle_ratio` | 0x1408e3b60 | 379 | 矩形纵横比→ridge_length_01：max(w/h,h/w)、smoothstep 尺寸混合，返回 smoothstep 形函数 | O | roofs-rt/decomp/decomp-0x1408e3b60.txt |
| `RoofShapeParams::eave_length_ws` | 0x1408e3c30 | 434 | 返回单一 f32：`(sqrt(p)+(1-sqrt(p))*0.5)*(1.5e+(1-e)*0)`，保留乘零项。**= surface.rs `eave_length_ws`** + parameters.rs（2,500 案例） | O | 已由 reconstruction/roof/surface.rs + parameters.rs 恢复；evidence/roof-surface/tg_...eave_length_ws.c |
| `roof_height_ws_from_wall_shape` | 0x1408e3c70 | 476 | 墙形状→屋顶世界高度：尺寸归一 smoothstep（0x140c95ef0 曲线采样）后映射 9..12 高度带 | O | roofs-rt/decomp/decomp-0x1408e3c70.txt |
| `profile_curve_ws` | 0x1408e3d40 | 480 | 把 20 点变 `(u*top+(1-u)*bottom, v*height)`，重算 Curve2 弧长/归一。**= surface.rs `profile_curve_ws_points` + Curve2**（1,512 全链案例） | O | 已由 reconstruction/roof/surface.rs 恢复；evidence/roof-surface/tg_...profile_curve_ws.c |
| `roof_uv_at_ws_pos` | 0x1408e4040 | 508 | 包 roof_uv_and_point_at_ws_pos，返回 `{found, uv}` | O | roofs-rt/decomp/decomp-0x1408e4040.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `FlatRoofStyle::pattern_id`（行 86）— 风格花纹 id（匹配序列化串）
- `RoofShapeParams::height_ws`（行 394）— 参数高度→世界高度（= height 的上游）
- `RoofShapeParams::from_wall_shape`（行 418）— 由墙形状构造 RoofShapeParams
- `roof_height_01_from_wall_shape`（行 444）— 墙形状→height_01

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_enum`、`visit_map`、`visit_seq`、`visit_str` 等 29 项（Roof/风格枚举的 Clone/Eq/Debug/serde derive 族）

## 关键布局（已证实）

- `Roof` 0x58 字节：`+8` tag（0=圆/1=矩）、`+0xc/+0x10` 圆 frame（cos/sin）、`+0x14/+0x18` 圆 center、`+0x1c/+0x20` 矩 dims、`+0x24/+0x28` tip_offset_01、`+0x2c` profile、`+0x30` height、`+0x34` ridge、`+0x38` eave、`+0x3c` ridge_dir bit、`+0x3d` rim、`+0x4c` floor 高、`+0x50` 动画初态、`+0x54` tag 字节。
- `RoofShapeParams` 28 字节块 = `{tip[2]@0, profile@8, height@12, ridge@16, eave@20, ridge_dir@24}`（parameters.rs 已建立 B 序列化映射）。
- 风格枚举 serde 变体串：`"Matching"`（0x676e69686374614d）与 `"Override"`（0x656469727265764f），均 8 字节。

## 实现方式（算法级）

完整算法见 `roofs-rt/ALGORITHMS.md` §6.2。要点：

### roof_uv_and_point_at_ws_pos（0x1408e4090）全管线（collider 注册表接线）
1. 注册表（RaycastWorld+0x48 非空守卫）→ FUN_1405b0d70(world+0x50) 哈希 + 双哈希探测（16B 组字节比较 + 占用掩码 + 第二哈希序列）；
2. 命中 → 值 {slot u8 区, index u32} → 竞技场注册表（world+0x3e18，同探测）→ slot 字节 != 0x11 → `func_0x000140b93940(world + slot·0x390 + 0x20, index, gen)`（17 槽 0x390 布局复核）；
3. 有效性断言（0x142aa3d50/0x142aa3d98 = world.rs 一致性串）；读命中视图 {mesh ptr@+0x34/+0x3c/+0x44, f32@+0x4c} → FUN_140c91b60 装配网格视图 + FUN_140c9cde0 由 ws_pos 建 ray；
4. shape vtable [view+0x38] 虚调 cast_ray_and_get_normal → 命中 → FUN_1408ce4a0（barycentric 插值）→ 输出 {found, uv@+0x4, point@+0xc}；未命中 → 0x301 标记。

### roof_uv_at_ws_pos（0x1408e4040）
包 roof_uv_and_point_at_ws_pos 返回 {found, uv}。
