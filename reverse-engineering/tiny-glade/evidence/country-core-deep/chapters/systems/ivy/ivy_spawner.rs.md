# systems/ivy/ivy_spawner.rs

> 文件作用：**常春藤生成：随机根点/光标旁根点**
>
> 生产者→GPU 验收链的**根**（producer）：ivy_spawner 每帧尝试生成新株 → IvyStorage::spawn_ivy 入存储 → grower 沿墙生长 → leaf_spawner 放叶 → send_to_gpu_{leaf,segments} 上传 GPU SSBO。

函数总计 5：业务 1 / 内联无独立符号 3 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_spawner::ivy_spawner` | 0x14094cf90 | 31 | 系统:生成 | O（2026-10-15 定点深挖，38KB 全解）：随机根点 + 光标旁根点双路径；内部三内联闭包 try_random_ivy_root / try_ivy_root_next_to_cursor / is_point_inside_wall_segment | collision-eco\decomp\decomp-0x14094cf90.txt；eco-reanalyst\decomp\decomp-0x14094cf90.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::ivy::ivy_spawner::try_random_ivy_root`（行 212）— O：随机根点尝试（RNG 采样，见算法）
- `country_core::systems::ivy::ivy_spawner::try_ivy_root_next_to_cursor`（行 245）— O：光标旁根点尝试（ivy_spawner 内联）
- `country_core::systems::ivy::ivy_spawner::is_point_inside_wall_segment`（行 294）— 谓词/校验（func_0x000140c9cdc0 墙段几何采样）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级，2026-10-15 O 全解）

**ivy_spawner（0x14094cf90，38KB，Bevy 系统，puffin scope 串 `crates/country-core/src/systems/ivy/ivy_spawner.rs` @0x142ab37f8）**

入口门控与计数：
1. **模式门**：`if (*(byte)(*param_7 + 0x1c) != 0x01) return;` —— AppMode 字节==1 才运行（ivy 编辑模式）。
2. **总量预算**：遍历 Query 命中实体数（Be​​vy archetype 位图扫描），若 `total_ivy_count > 39999 + 0x12` → return。**Ivy 株数上限 ≈ 40000**（机器码 `cmp 39999 < lVar19+0x12`）。

双路径分派（`uVar21 = *(u32*)(*param_5 + 0x120)`，即 IvyStorage 内计数）：
- **路径 A（`uVar21 < 3`，即株数<3 早期）= try_ivy_root_next_to_cursor**：
  1. 读光标资源 `pbVar4 = *param_4`（Option<Vec3>，`(*pbVar4 & 1)` 判别；`(x@+4, z@+0xc)`），有光标则叠加。
  2. 读墙空间哈希 `lVar11 = *param_6`（grid `{ptr@+8, len@+0x10}`，格步长 0x30B，格内含 {ptr@+8, data@+0x10} 两级）：`uVar12 = *(qword*)(lVar11+0x50)`（世界坐标）+ `*(qword*)(lVar11+0x48)`（grid dims 2×u32）→ `pos / dims` 归一化（0.5 常量 @0x1429258e0/e4）。
  3. **RNG 采样格**：`func_0x000142923ee0()` ×2（perchance f32）→ 格下标 `uVar15 = row*dims.w + col`，越界（`len@+0x10 <= uVar15`）→ return。**0.492307693（=32/65）** 与 **32.0**（@0x142925870/87c）为世界→格换算常量。
  4. 空格（`ptr == 0`）→ return。
  5. **`closest_segment_index`（0x14094c800）**：`FUN_14094c800(&out, xz, 0, segments_ptr, count, max_height=0.5f)` —— max_ivy_grow_height 取 0.5。`out.found != 1` → return。
  6. 命中段（0x38B=56B 步长）：投影 t 夹取 [0,1]（1.0@0x142925904），插值段高 `fVar30 = t*p1 + (1-t)*p0`。
  7. **墙查找**：hashbrown 遍历 PublicWalls（`FUN_1405b0d70` 哈希探测，条目步长 0x2a8=680B）：`curve_len < 2` 或 `墙高 <= 0.0` → 跳过（`*(lVar11-0x290) < 2` / `*(float*)(lVar11-0x270) <= 0.0`）。
  8. `FUN_141469a20`（PublicWallState 曲线采样）→ 投影点；`FUN_141467ce0`（曲线点数）；**func_0x000140c9cdc0 ×N（墙段几何采样）** 沿曲线方向/法向偏移得根位。
  9. **根位偏移常量（O）**：`0x142ab31e0..0x142ab31f0 = 0.68`（+侧）与 `0x142ab3200..0x142ab3210 = -0.68`（−侧），再乘 0.5（@0x142925870/8e0/e4）→ **垂直偏移 0.34 = 0.68×0.5**。
  10. **离既有根距离校验**：遍历既有根（`lStack_1b8` 个，指针数组 `pcStack_1c0` 步长 8）：点积判向 + `|root.pos − new|² < 0.25`（0x142925950=0.25）→ 过近 return。`uVar21<3` 用 0.25，否则用 **2.56**（0x142ab3214，即距离 1.6）。
  11. **垂直高度采样**：`fVar31 = (rand×1.5 − 0.75) / 墙高 + 段高`（1.5@0x1429258d0，−0.75@0x142ab3220），NAN/INF panic（CurveU 串 @0x142aa53d0/0x142aa5498），钳 [0.000928482, 1.0]（0x142960690 下限）→ 不合法则跳过该次尝试。
  12. **spawn 循环**：每轮尝试生成根位 → 距校验 → 再次 RNG 采样格/寻段，循环 `puVar24 != puStack_1b0`（次数上限 3/`0x142ab3218`）；成功 → `IvyStorage::spawn_ivy`。
- **路径 B（`uVar21 >= 3`）= try_random_ivy_root**：
  1. **随机墙**：`FUN_141625a50`（perchance `usize_less_than`，调用点 0x14094d2e7）→ 墙序号 ∈ [0, wall_count)。
  2. **逐墙 u**：`func_0x000141625ae0`（perchance `uniform_f32`，调用点 0x14094d394）→ **u ∈ [0,1) 单位曲线参数**；u 非法（<0 或 >1 或 NAN）→ panic（CurveU @0x142aa53c0/0x142aa5498）；合法 → `Curve::get_coord_at_u`（FUN_141467ce0）取曲线点。
  3. **噪声密度闸门（O）**：`FastNoise::get_noise(*param_2)`（FUN_141743760 = bracket_noise FastNoise）→ `noise < *(f32*)(*param_2+0x78)` → 跳过该墙（spawn 密度过滤）。
  4. 经共享墙 loop（0x14094d49f）处理 → 侧向掷硬币 → 根位 → spawn（见下）。

**共享 spawn 机制（两路径均到达，2026-10-15 补挖）**：
- **侧向掷硬币（左右法向选择）**：`func_0x000141625a00` = **perchance `get_bool`**（S_PUB32 `PerchanceContext::get_bool`，调用点 0x14094d6b0）——true → −0.68 侧，false → +0.68 侧。
- **func_0x000140c9cdc0 = `glam::Vec2::x0y`**（S_PUB32 `SwizzlerX0Y::x0y`，纯 swizzle 无 RNG）：`{x,z}→{x,0,z}` 展开 2D 垂向为 3D。**不是掷硬币**。
- **垂直/沿曲线 u**：`u_spawn = (uniform_f32×1.5 − 0.75)/curve_length + u_on_curve`（uniform_f32 调用点 0x14094dbb4；`curve_length` = 墙 entry −0x270 = PublicWallState 值+0x30，**非 /1.0 硬编码**；`u_on_curve` = `clamp01(Curve::get_approx_u_from_pos)`（FUN_141469a20），**非中点近似**）；钳 [0.000928482@0x142960690, 1.0@0x142925904]，越界放弃该次尝试。
- **距既有根校验**（shared 区，decomp 行 534-567）：判向点积 `0.0 <= existing.dir·(new−existing)` + `|Δpos|² < 0.25`（0x142925950，uVar21<3）或 `< 2.56`（0x142ab3214，uVar21≥3）→ 过近 return。**路径 B 距门槛 = 2.56（距 1.6）**。
- spawn 尝试次数 = `2 + usize_less_than(3)`（调用点 0x14094da34）∈ {2,3,4}；每次尝试 re-采样格（uniform_f32 @0x14094dbb4）→ closest_segment_index → 根位 → `IvyStorage::spawn_ivy`。

**IvyStorage::spawn_ivy（0x1408696f0，O 全解）**：
- 参数面：`(self, root_pos*{x,y,z}, dir*{x,y,z}, param_4/5=插值端点, param_6/7=u 范围 [start,end), param_8)`。
- 新建 Ivy 记录：**root 点 = 0x28B（40B）**：`{id u64@0=0, pos Vec3@8 (x,y@8,z@0x10), dir Vec3@0x14, u32@0x20=0}`（与 add_point 的点布局一致）。
- `FUN_141527820(self, next_id=*(self+0x40), &root_point)` → HashMap<usize,Ivy> 插入；`*(self+0x40) += 1`（**next_ivy_id 自增**）。
- RNG：`FUN_141625a50(self+0x30, end-start)`（perchance `range`，断言 **`assertion failed: end >= start`** @0x142aa5710，perchance-0.4.0/src/lib.rs）+ `FUN_141625ae0(self+0x30)`（perchance f32）→ `u = start + (end-start)×rand`，`fStack_a0 = lerp(param_4, param_5, rand)`。

**IvyStorage/IvyPoint 数据布局（O，spawn_ivy + add_point 机器码）**：
- `IvyStorage = HashMap<usize /*ivy_id*/, Ivy>`（表体 @+0）+ **RNG @+0x30**（perchance-0.4.0，`func_0x000141625a50`=range / `func_0x000141625ae0`=f32）+ **next_ivy_id @+0x40**（spawn 自增）。
- `Ivy = 可增长 40B 元素双端容器（IvyPoint）` + **point_id 计数 @+0x58**（add_point `id = [self+0x58]+1` 后自增）。
- `IvyPoint = 0x28B`：`{id u64@0, pos Vec3(x,y@8, z@0x10), dir Vec3@0x14, u32 flags@0x20=0}`（add_point 0x140869460 逐字段证实）。

**关键常量表（O，rne-read）**：

| 常量 | 值 | 用途 |
|---|---|---|
| 0x142925870/8e0/e4 | 0.5 | 根位偏移缩放 / 世界→格换算 |
| 0x142925874/878 | 0.492307693 / 0.000384615385 | 世界→格缩放（32/65）/ 低优先 |
| 0x14292587c | 32.0 | 格偏移 |
| 0x142925880/884 | 0.285 / 0.2475 | 低优先 |
| 0x1429258d0 | 1.5 | 垂直采样乘子 |
| 0x142925904 | 1.0 | u 夹取上限 |
| 0x142925950 | 0.25 | 光标路径距校验（距 0.5） |
| 0x142960690 | 0.000928482 | 垂直采样钳下限 |
| 0x1429a49bc | 2.14748352e9 | RNG f32→int 夹取（i32::MAX） |
| 0x142ab31e0..f0 | 0.68 | 根位 + 侧法向偏移 |
| 0x142ab3200..10 | −0.68 | 根位 − 侧法向偏移 |
| 0x142ab3214 | 2.56 | 随机路径距校验（距 1.6） |
| 0x142ab3218 | 3 | spawn 尝试次数上限 |
| 0x142ab3220 | −0.75 | 垂直采样偏置 |
| 0x142a171f0/f4 | −0.5 | 低优先 |

**panic/断言串（O）**：`assertion failed: end >= start`（0x142aa5710，perchance-0.4.0 RNG range）、`Tried to init CurveU from an invalid u: `（0x142aa53c0，墙曲线采样）、`assertion failed: !u.is_infinite()`（0x142aa5498）、`assertion failed: w.simplified_curve_2d.is_valid()`（0x142ada193，public_walls.rs 跨域）。

**跨模块消费者**：grower（0x14094a910 主闭包 0x14094ac20 沿墙推进）→ leaf_spawner（0x140996080 沿末段放叶）→ send_to_gpu_leaf/segments（0x1408c1580/0x140a9b4e0 上传 GPU）。
