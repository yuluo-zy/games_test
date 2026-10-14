# terrain-water 域 DIGEST（tg_a4 · 2026-10-14）

对象：`country-core` 地形/水域/栅格域业务过程。数据源：本域 92 函数批量反编译 + 18 函数补充批
（`decomp/`，111 个文件，含 83+15=98 成功）、PDB 元数据骨架（`skeleton/B/crates/country-core/src/
{resources,systems}/**`，含权威 RVA/源码行）、rne-read 字节/字符串直读（`strings-extracted.txt`）、
prior 证据（`evidence/multi-baseline/water-cpu-analysis.md`、`water-extra-analysis.md`、
`evidence/terrain-editing/B-original-*.asm.txt`）。标注：**O**=机器码/PDB 直接支持；**I**=调用链组合；
**U**=未恢复（不臆造）。

版本：B patched EXE（`workspace/input/tiny-glade.exe`，sha256 cf048e27…），PDB 2d9686b8…。

---

## 1. 模块清单

| 源文件（skeleton 路径） | 业务过程 | 关键地址（PDB 权威 RVA，本域反编译） |
|---|---|---|
| resources/world_raster.rs | new / subscribe / clear_buffer / clear_all / rasterize_stroke / copy_composite_to_scratch / commit_scratch_to_composite / copy_to_staging / build_tile_hasher / DirtyTilePublisher::unique / deserialize\<Value> | 0xa15530 / 0xa158e0(+水实例 0x2612be0) / 0xa15a10 / 0xa15da0 / 0xa16960(+水 0x2261d10) / 0xa161a0(+水 0x2262680) / 0xa16580(+水 0x2262a60) / 0xa172d0(+水 0x2261760) / 0x9393e0 / 0x9393b0 / 0xa197c0 |
| resources/world_raster_defs.rs | PathRaster::new / AllWorldRasterWrite::{ordered_names,write,would_write_succeed,upload_to_staging} / SystemParam {apply,queue,new_archetype(ICF),init_state,get_param} / deref,deref_mut(ICF) | 0x83eaf0 / 0x83eb10 / 0x83ebe0 / 0x83f410 / 0x83fa60 / 0x3bb80(ICF) / 0x83fe20 / 0x1507ce0,0x1508130 / 0x67dd80(ICF) |
| resources/active_raster_stroke.rs | index_mut / start_new / resample_and_smooth / InProgressRasterStroke::{last,add_point,solidify_fluid_into} | 0x868cf0 / 0x868d00 / 0x868e40 / 0x868f70 / 0x868fb0 / 0x8691b0 |
| resources/raster_stroke.rs | iter_points_xz / fmt(Debug) | 0xa97860 / 0x9f1c90 |
| resources/brush_sizes.rs | reset/set path/water/garden（6 个，NotFound 已手解） | 0x9f1860/70/90/a0/c0/d0 |
| resources/terrain_height.rs | sample_at_world_xz / intersect_ray / copy_from_gpu / load_index_unchecked | 0x9e9640(+0x1246fb0) / 0x9e9a20(+0x12773f0) / 0x9ea020 / 0x8b54c0 |
| resources/terrain_rock_mask.rs | sample_at_world_xz | 0x9ea800（prior 已验） |
| resources/terrain_editing.rs / terrain_stroke.rs | new_stroke / get_last_mut_stroke / insert_or_morph_last_point / serde 族 | 0x924a10 / 0x924900 / 0xa97880 / 0x8e5bc0 等（prior 已验） |
| resources/mouse_terrain_raycast.rs | intersect_with_xz_plane / camera_mouse_ray / mouse_terrain_raycast / closure$0 | 0x8ef180 / 0x8ef1c0 / 0x8ef320 / 0x8efa70 |
| resources/cursor_terrain_raycast.rs | position | 0xa12650（prior 已验） |
| resources/read_mouse_depth_value.rs | read_mouse_depth_value_from_gpu | 0xa1a480 |
| resources/in_progress_curve.rs | new / try_add_raw_input_points / compute_final_curve | 0xad3070 / 0xad3f50 / 0xad41a0 |
| resources/drawing_input_smoother.rs | DrawingInputSmoother::process | 0x145c350 |
| resources/glade_border.rs | is_pos_inside / is_shape_inside / search_lerped_shape_that_fits_into_glade | 0xad2950 / 0xad2970 / 0xad2ce0 |
| resources/rocky_terrain.rs | RockyTerrainMesh::load | 0x8bbf50 |
| systems/world_raster.rs | record_raster_history / upload_world_raster_buffers_to_staging / draw_raster\<T>×10 | 0x957990 / 0x957bf0 / 0xabba80 等 |
| systems/history/replay_{terrain,water,garden}.rs | replay_terrain / TerrainReplayCmd::{forward,backward} / replay_water::closure$2 / replay_garden::closure$1::closure$0 | 0x995150 / 0x994f30 / 0x995040 / 0xa01560 / 0x1447500 |
| systems/terrain/* | terrain_stroke_changed / edit_terrain_stroke / terrain_hover_idx / ui_draw_terrain / ui_terrain_focus_mode(+4 闭包) / mod::{add_terrain_systems,insert_terrain_resources,add_terrain_events} / history_record_terrain（prior asm） | 0xa03ba0 / 0x94e000 / 0xaff720 / 0xa9fdc0 / 0x9a4fb0 / 0x9455f0 / 0x9457c0 / 0x945a20 / 0xac9770 |
| systems/terrain/delayed_terrain_changed_msg.rs | on_terrain_modified / delayed_terrain_changed_msg（prior asm B-original-140aabdd0） | 0xaabdc0 / 0xaabdd0 |
| systems/mesh_follow_terrain.rs | mesh_follow_terrain | 0xa68430 |
| systems/garden(+draw_garden,record_garden_history) | build / draw_garden / record_garden_history | 0xb6d290 / 0xae4620 / 0x1695e10 |
| systems/render_loop/compute_passes/{compute_water,compute_garden,displace_rocky_terrain,highlight_active_terrain_stroke,mouse_read_terrain_stroke_index,terrain_lighting_texture,upload_world_raster,read_terrain_data,rasterize_terrain_stroke} | 各 1–6 | 0xa12680,0xa13970 / 0xa22140,0xa22b80 / 0xa3ee80 / 0xa58450 / 0x9768b0 / 0xa51440 / 0x3d8bd0 等 6 实例 / 0x98c680 / 0x8c7120 |
| systems/render_loop/draw_passes/draw_shaders/{water_planes_draw,rocky_terrain_draw} | water_planes_draw / lilypad_flower/lilypad/reed_draw / rocky_terrain_shader | 0x9d9bc0 / 0x9da760 / 0x9da580 / 0x9da940 / 0x9cb4e0 |
| systems/audio/{water_audio,water_ambience_audio,garden_audio,terrain_audio,terrain_stairs_audio} | 8 系统 + 2 init | 0x873630 等 |
| system_water（跨 crate） | draw_water / update_water_audio_grid / build（prior 已验） | 0x141ba9130 / 0x141baefe0 / 0x1406501d0 |

域内 sysres 清单 93 过程；补充批补齐 PDB 有而 sysres 缺的 18 项（见 §7 错位表）。

---

## 2. 布局表

### 2.1 WorldRaster\<512>（O，new 0xa15530 + copy_to_staging 0xa172d0 + copy_* 0xa161a0/0xa16580 + subscribe 0xa158e0 + write 0x83ebe0）

总尺寸 ≥ 0x29c（0x98+0x204），估计 0x2a0。u64 视角：

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | composite cap（=0x40000） | new 写 `*p1=0x40000` |
| +0x08 | composite 数据指针（0x40000B alloc） | new + memcpy |
| +0x10 | composite len（=0x40000） | copy_* len 比对 |
| +0x18 | scratch cap | new |
| +0x20 | scratch 数据指针 | new/copy |
| +0x28 | scratch len | copy_* len 比对（不符→panic "Length mismatch; expected "） |
| +0x30..+0x47 | DirtyTiles 位图 Vec：{cap=0x1000@+0x30, ptr@+0x38, len=0x1000@+0x40}（u32 单位；0x4000B = **4096 个 u32 = 64×64 瓦片（每瓦片 8×8 px）状态字**） | new 写 {0x1000,ptr,0x1000}；write 路径 memset(+0x98…) |
| +0x48..+0x5f | 次级 Vec {0x40@+0x48, ptr@+0x50, 0x40@+0x58}（0x100B？）+ 4B；copy_to_staging 读 +0x50..+0x5c 为 4×u32“脏瓦片元数据” | new + copy_to_staging |
| +0x60..+0x97 | DirtyTiles 头部（7×u64，由 param_2 原型拷贝 56B 初始化；含 parking_lot Mutex@+0x60/+0x68——subscribe/copy_to_staging 走 lock_slow，skeleton 依赖注解 parking_lot 0.11/0.12 O） | new 的 7 次 u64 拷贝 |
| +0x98..+0x297 | 512B 内部 staging 缓冲 | memset 0x200（new 清零；write 置 0xff 全脏） |
| +0x298 | 读缓冲选择字节（0→composite，1→scratch） | copy_to_staging 第 45 行；terrain_hover_idx 同读 |
| +0x299 | staging 脏字节（rasterize_world_space_stroke 置位；**AllWorldRasterWrite::write 置位（0x83ebe0，O）**；copy_to_staging 清零） | 三处 O |
| +0x29a | published 字节 | copy_to_staging 尾置 1 |
| +0x29b | 再上传脏字节（water_rasterize_from_cpu 判 0 后清；置位方 = upload_to_staging 路径之外，仍 U——本轮未定位新置位点） | 0xa12680 第 129-131 行 |

- 栅格：**512×512 u8**（composite/scratch 各 0x40000B；bilinear_sample 行/列 clamp 0x1ff，行距 SHL 9——prior O，本域复验一致）。
- 世界映射：`grid = (world/130.0 + 0.5) * 512`（O，0x142a7b658=130、0x1429258e0=0.5、0x1429a3c80/84=512）。
- u8→f32 归一 /255.0（prior O，0x14293ac70）。
- 初始 DirtyTiles 瓦片状态字 = **0x07954dbb**（O，0x142ac7e50..5c 4×u32，new 填满 4096 槽）；语义 U（疑似“无水/未脏”编码，含标志位）。
- `WorldRaster::new<512>(&mut self, proto: &DirtyTiles)`——param_2 为原型（7×u64 拷贝）。调用点仅 DATA xref（static 表），动态调用链 U。
- serde：`deserialize<serde_json::Value>`（0xa197c0）产出存档反序列化；错误串 "Expected raster " / "End of stream while expecting raster "（O，0x142a93d80/0x142a93dc8）——存档里 WorldRaster 按 raster 名序列化。

### 2.2 DirtyTilePublisher（O，unique 0x9393b0 手解 + subscribe 0xa158e0）

`DirtyTilePublisher::unique` 38B 构造：`{[+0]=0(u32), [+8]=8(u32), [+0x10..1f]=0, [+0x20]=8(u32), [+0x28..2f]=0}`（判别 0 + 两个空 Vec{cap=8}）。判别式位于首 u64（i64::MIN 基址编码，subscribe 的 `lVar2 - 0x7fffffffffffffff` 判 0/1；>1 → panic fmt）。
subscribe 在**非 primary** 发布器上 panic：**"Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."**（O，0x142ac85f8）。PDB 源码行：unique=64 / shared=68 / secondary=72。

### 2.3 subscribe 的订阅记录（O，0xa158e0）

0x28B Arc 记录：{u32@0=1（计数/tag）, u32@8=1, u32@0x10, u32@0x14, u32@0x18=0, u32@0x1c=0, 0x1e/0x1f=0}；Arc 克隆 = LOCK inc；压入订阅者 Vec@+0x78（游戏 Vec {cap@+0x78, ptr@+0x80, len@+0x88}，8B 元素，grow FUN_140562870）。返回记录指针即订阅句柄。

### 2.4 ActiveRasterStroke（0x470B 记录，O：index_mut=base+idx*0x470 + start_new + add_point + resample_and_smooth）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | u64 活动/流体标志（add_point 判 `*p1==1` 门控 solidify 分支；start_new 置 0） | round2 修正 |
| +0x08 | 流体游标指针（add_point 读作指针判非空；solidify 后置 0 或剩余点数） | round2 修正（round1 误标 max-points 配置） |
| +0x10/+0x18/+0x20 | Vec1（12B Vec3 元素）：cap/ptr/len——**已固化曲线** | start_new free/copy |
| +0x28/+0x30/+0x38 | Vec2：**进行中原始输入点**（12B Vec3） | add_point push 于此 |
| +0x40 | 累计弧长 f32（>1.0 触发 solidify） | add_point SQRT 累加 |
| +0x48/+0x50/+0x58 | Vec3：**重采样+平滑后曲线**（resample_and_smooth 输出，draw_raster 的 rasterize_stroke 消费此列） | resample_and_smooth |
| +0x60 | 笔刷宽度 f32 | start_new param_4 |
| +0x64 | 模式字节（0/非0 → erase/add；rasterize_stroke 用它 + bit0 选 blend 闭包；draw_raster 尾部按它发 WaterDraw/WaterErase 事件） | start_new param_3 |
| +0x68 | u32 blend/权重标志（bit0） | start_new param_5；rasterize_stroke param_7 |
| +0x6c | u32 计数器 | start_new param_6 |
| +0x70..+0x468 | 双份 512B 脏瓦片位图（64 u64 ×2；draw_raster 交换两半，prior O） | prior |
| +0x470 | 记录总长 | index_mut imul 0x470 |

行为链（O）：
- `start_new(brush, mode, flag, cnt)`：清三个 Vec（cap 置 0、ptr=dangling(4)）→ 写 brush/+0x64/+0x68/+0x6c → `add_point(首个点)` → 再把同一点推入 Vec3。
- `add_point`：Vec2 空→push 首点；否则累计弧长 += dist(last,new)，push（cap 满先 grow）。**累计弧长 >1.0 → solidify**：`solidify_fluid_into(ptr, max_points, &Vec1)` 把一段流体点固化进 Vec1，剩余点 memmove 前移。
- `resample_and_smooth`：Vec1 整列复制进 Vec3 → `solidify_fluid_into(Vec2.ptr, Vec2.len, &Vec3)` 把剩余流体点也固化 → **对 Vec3 全列做 10 遍 3-tap 平滑**（`p'ᵢ = ((pᵢ₋₁+pᵢ)·0.5 − pᵢ₊₁)·0.5 + pᵢ₊₁` = 0.25/0.5/0.25，O：常量 0.5×4 @0x14293ac60..6c、外层 `while (lVar15 != 10)`）。
- `InProgressRasterStroke::last`：Vec2.len==0 → **panic（Option::unwrap None）**；否则返回 &points[len-1]（12B 步长，O 手解 0x868f70）。

### 2.5 BrushSizes（O，6 函数全部手解）

| 偏移 | 工具 | 默认（reset） | clamp 范围 | 常量地址 |
|---|---|---|---|---|
| +0x00 | path | 0.8（0x3f4ccccd） | [0.32, 1.2] | min 0x142ac56c0；max 0x1429cd128 |
| +0x04 | water | 2.0 | [1.0, 5.0] | 0x142925904 / 0x142925988 |
| +0x08 | garden | 3.0 | [2.0, 4.0] | 0x14292e6f0 / 0x142934694 |

### 2.6 TerrainHeightsData\<T>（部分 O，sample_at_world_xz 0x9e9640 + copy_from_gpu 0x9ea020 + intersect_ray 0x9e9a20）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x08 | 高度数据指针（**u16 网格**，512×512，行距 512 u16 = 512KB） | sample：`*(u16*)([p1+8] + ((z<<9|x))*2)` |
| +0x50/+0x54 | min/max 高度 f32（默认 **-2.5 / 24.0**，O：0x142a9f3f0 打包对，与 prior schema 一致） | copy_from_gpu 比对+复位 |
| （其余：脏瓦片位图 64×u64 等） | copy_from_gpu 的 auStack_270[64] 8×8 瓦片位图（O） | 16B 块 diff 拷贝 |

- `sample_at_world_xz`：`u=(x/130+0.5)` clamp[0,1] → `idx = u*512 − 0.5 + 1`（0x14295a0e8=-0.5）clamp[0,511]；权威实例返回 **u16 原始值**；视觉实例（0x141246fb0）为**双线性变体**（4 角 `load_index_unchecked`（0x1408b54c0，O）+ 3×lerp，u16→f32 解码用 +0x50/54 min/max）。debug 路径（全局字节 *0x14362c308≠0）用 `+0x201` 偏移索引（视觉/调试网格变体）。
- `intersect_ray`：先对高度场 AABB（min/max@+0x50/54）求进出 t（FUN_141240860），再 **32 步行进 + 每步 3 次 0.5 二分**逼近交点，逐步调 sample_at_world_xz 采样（O）。命中写 {t, …} 到 out。
- `copy_from_gpu(gpu)`：min/max 非默认 → 复位；**8×8 瓦片粒度 diff**（每块 16B = 8×u16），不同才拷贝并置 64 位位图位（O）。

### 2.7 射线/深度资源（O）

- `CameraCursorRay`：{tag u32@0(=1 有效), pos@4, dir@0x10, length@0x1c}；`intersect_with_xz_plane` t=(y-pos.y)/dir.y（O 0x8ef180，prior）。
- `CursorTerrainRaycast`：{tag u8@0, pos@4}（prior O）。
- `MouseDepthValue`：read_mouse_depth_value_from_gpu（0xa1a480）从 `MouseDepthValueSsbo`（type-id 0x…348，名称 0x47=71 字符）读 **2×float4**（NaN 过滤：`&0x7fffffff < 0x7f800000`），写 {w@0, xyz@+8}；资源 +0x40 == i64::MIN+3 哨兵时早退（帧有效性）。
- `HoveredTerrainStrokeId`：{tag, id u32}；terrain_hover_idx 从 MouseTerrainRaycastSsbo（名 0x6c=108 字符）映射缓冲读 `id=*(i32*)(buf+0x10)`（-1 哨兵），水位采样 ≤0 / 光标无效 / 距离≤0.5 时不更新（O）。

### 2.8 AllWorldRasterWrite / AllWorldRasterRead（O）

- 6 槽位顺序（O，would_write_succeed 0x83f410 的胖指针名表 @0x142a93ca8/d20/d30/d40/d50/d60）：**PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster, WaterRaster, GardenRaster**。
- `would_write_succeed`：按槽位与类型名（"PathRaster" 0xa 字符等）比对；缺失资源 → Bevy panic 格式 "Resource does not exist…Resource requested by … does not exist: "（O，0x142a92ed0 区）。
- `write`（0x83ebe0）：每槽：borrow 标志置位（SBORROW8）→ 清 composite（memset 0）；否则 len 比对（不符→panic "Length mismatch; expected "，源文件 crates/country-core/src/resources/world_raster_defs.rs，O 0x142a93c90）→ memcpy 入 composite；然后 **memset(+0x98, 0xff, 0x200) + `+0x299=1`（全脏 + staging 脏）**。
- `upload_to_staging`（0x83fa60）：每槽按 type-id（0x142c2e210 等 6 个）在 AssetSsboLibrary 查 `*RasterStagingSsbo`，memcmp 全名（如 "country_core::resources::world_raster_defs::PathRasterStagingSsbo"，0x41=65 字符 @0x142a9d65d）→ vtable 取映射指针 → `copy_to_staging(raster, selector=0, mapped, len)`。
- SystemParam 骨架：apply/queue/new_archetype = **ICF 折叠到裸 `ret`**（0x3bb80）；init_state = 0x83fe20（6 个 get_resource 注册）；get_param = 0x1507ce0/0x1508130（RasterDrawResources / RasterHistoryRecordResources）。

### 2.9 GladeBorder（部分 O）

- `is_pos_inside`（0xad2950）= 跳板：`test [rax],1; jnz 共享主体` → 委派进 `is_shape_inside`（0xad2970）主实现。
- 常量：半径 1.5/0.3/0.4/0.5/1.0/2.0/4.0/0.8（draw_garden 用）；panic **"Tried to init CurveU from an invalid u: "**（O，0x142aa53c0，来源 utils/src/geometry/circle.rs —— CurveU∈[0,1) 校验，与 shared-foundation 的 CurveU 范围合同一致）。

### 2.10 其他资源

- `RockyTerrainMesh::load`：`fs::read_to_string`（FUN_140d59140）→ serde_json 解析 → `add_new_with_data<RockyTerrainVertexSSBO/TriangleSSBO>`；顶点源记录 0x24B→0x30B、三角形 0x10B→0x20B（O，prior 一致）。
- `TerrainEditingData`：{HashMap@+0x8(数据)/+0x10(掩码)/+0x20(计数), last_stroke_id u32@+0x38}（prior O，本域复验 new_stroke/get_last_mut_stroke 未变）。
- `InProgressCurve`：new=0xad3070（Vec 初始化）；try_add_raw_input_points=0xad3f50（先调 FUN_141466860 输入验证，非 3 分支清两字段）；compute_final_curve=0xad41a0（曲线重采样终结）。freehand 墙输入曲线（wall 域主消费者）。
- `DrawingInputSmoother::process`（0x145c350）：输入点列平滑处理（param_2=平滑参数, param_3=首点, param_4=缓冲）。

---

## 3. 方法/系统行为（O/I/U）

### WorldRaster 家族

| 函数 | 行为 | 级别 |
|---|---|---|
| WorldRaster::new\<512> | 分配 composite+scratch（0x40000B×2）+ DirtyTiles 位图（0x4000B=4096 u32，初始 0x07954dbb）；初始化 7×u64 DirtyTiles 头；清零 0x204B staging+flags | O |
| WorldRaster::subscribe\<512> | primary 校验（secondary → panic 串见 §4）；建 0x28B Arc 记录压入订阅 Vec（+0x78）；锁竞争走 parking_lot slow path | O |
| impl$2::clear_buffer\<512> | 按选择字节清 composite 或 scratch（0x200 或全 0x40000，param_2 字节选） | O（0xa15a10） |
| impl$2::clear_all\<512> | 双缓冲全清（0xa15da0） | O |
| copy_composite_to_scratch / commit_scratch_to_composite | 纯 memcpy + len 断言（"Length mismatch; expected "） | O |
| rasterize_stroke\<512> | 点数<3 早退；点列→zeno Command 表（28B/条，首点 variant 0、其余 1，grid=(w/130+0.5)*512）；(arg6=0,bit0=0)→env$1(min/擦除)；(≠0,0)→env$0(max)；(≠0,1)→env$2(饱和加×笔刷 16.16)；(0,1)→panic；目标层按 mode 字节选 composite/scratch | O |
| copy_to_staging | +0x299 脏→清零 out 区并清 flag；+0x298=选择字节；DirtyTiles::retain 按脏瓦片把选中缓冲拷入 out(SSBO 映射)；+0x29a=1 | O |
| build_tile_hasher | 写 4×u64 常量（0xbe734ee571db45b0 / 0xfca4335f3c9b800a / 0x36de4a1709f2df62 / 0xee5497aefccb8a97）到 +0/+8/+0x10/+0x18 —— 瓦片哈希器种子（ahash 风格 4-key RandomState，I） | O 字节/I 语义 |
| DirtyTilePublisher::unique | 38B 判别 0 + 双空 Vec(cap=8) 构造 | O |

### 系统

| 系统 | 行为 | 级别 |
|---|---|---|
| record_raster_history（0x957990） | 按 variant·0x470 槽位 Arc 克隆 12B/条笔画 Vec → History::add → 重置 Vec(cap 4) | O（prior + 本域复验调用面） |
| upload_world_raster_buffers_to_staging（0x957bf0） | 脏输入流非空 → AllWorldRasterWrite::upload_to_staging（6 光栅全部发布 composite→staging SSBO） | O |
| draw_raster\<T>（10 实例，水 0x2260ae0） | 状态机：0=start_new；1=last 距>0.0005 → add_point+resample_and_smooth+DirtyTiles::clear+rasterize_world_space_stroke+extend×2+交换双位图+copy_composite_to_scratch+rasterize_stroke+SSBO map+copy_to_staging；其余状态 commit_scratch_to_composite 发布；按 +0x64 发 Draw/Erase 事件 | O（prior 精读 + 本域布局佐证） |
| terrain_stroke_changed | EvTerrainStrokeChanged 非空 → RecalculateHeightMapCmd + PlayTerrainSound | O |
| edit_terrain_stroke | 按命令 id 查 TerrainEditingData；缺失 → panic（§4）；falloff/height 应用（prior tg-terrain apply_resolved）→ RecalculateHeightMapCmd + 声音 | O |
| terrain_hover_idx | AppMode 位掩码 0x421；SSBO 读 stroke id；水位/光标/距离门控 | O |
| ui_draw_terrain / ui_edit_terrain | 事件/命令生成面（ui-input 域细查）；新笔画 new_stroke + EvTerrainStrokeChanged | O 签名/I 内部 |
| ui_terrain_focus_mode（+closure 0x9a3620/8640/88c0/8b40） | 地形聚焦 UI 状态机；167KB 反编译几乎全内联（alloc/memcpy + HashMap helper 0x140c9cde0×9）；闭包 4 个 O 地址；内部状态机 U | I/U |
| delayed_terrain_changed_msg / on_terrain_modified | counter 原子自增；delays 每帧递减→0 触发 copy_from_gpu×2（0x3a/0x40 SSBO 尺寸断言）；flag 置位 push(2) 延迟 2 帧 | O（prior asm） |
| mesh_follow_terrain | Query<(&MeshFollowTerrain,&mut Transform,&Handle<Mesh>)>+2×Res；mesh 句柄无效→panic（§4）；取 "Vertex_Position" 属性（0x142ad5120，0xf）→ 位置计算（FUN_1412fd550，内联地形采样）→ Transform.y 更新 | O |
| water_rasterize_from_cpu | WaterRasterStagingSsbo（type-id 0x1ad182936286a0ee）存在且 +0x29b 脏 → 同步 u32 版本(+0x5c←库+0x5b0)、清 0x29b、memset(+0x98,0,0x200) → 纹理 + **"water blur temp" 作用域**（puffin）4 compute pass（含 "/gauss_blur_mask.cs.hlsl"）上传水 mask | O |
| water_spawn_vegetation | 两 ComputePass："Spawn lilypads/water/spawn_lilypads.cs.hlsl"（LilypadSsbo+LilypadFlowerSsbo）与 "Spawn reeds/water/spawn_reeds.cs.hlsl"（ReedSsbo）+ indirect draw 缓冲 | O |
| garden_rasterize_from_cpu / garden_spawn_vegetation | 同构（GardenRaster 版；Flower03Ssbo/BushSsbo 注册名见 0x142a9d65d 大表） | O |
| garden::build | 注册 draw_garden/record_garden_history/replay 等系统（FunctionSystem 记录佐证：RasterReplayResources+EventWriter\<GardenDrawEvent>…） | O 签名/I 全表 |
| draw_garden | 工具 id 分发：id 9（橡皮）/id 17（花丛）分支（`id-6<0xf` 且 `id-5∈{4,0xc}`）；半径=笔刷+1.5；GladeBorder::is_shape_inside 界内校验 → draw_raster\<GardenRasterStagingSsbo,…> | O |
| replay_terrain / TerrainReplayCmd::{forward,backward} | ReplayCmd 0x40B 条目迭代（重量 f32@+0x14、标志字节）；forward/backward 两个 1482B 函数（pair）应用/撤销笔触 | O 框架/I 细节 |
| water/terrain/garden/stairs 音频系统 | 消费水位栅格/笔触事件做音频（audio-misc 域细查）；update_water_audio_grid 的 24B 单元/64 位滚动窗/6400.0² 阈值/0.5 水位阈值 prior O | O |
| upload_raster_staging_to_texture\<T>×6 | "r16f/upload_cpu_raster.cs.hlsl"（fmt 0x4c）或 "rgba16fr16f/…"（fmt 0x61），未实现分支 panic "not yet implemented"（prior） | O |
| read_terrain_data | 地形/岩体/水/花园回读 + ScheduledTerrainReadback.flag 置位（prior） | O |
| rasterize_terrain_stroke / displace_rocky_terrain / calc_terrain_lighting_texture / highlight_active_terrain_stroke / mouse_read_terrain_stroke_index / mouse_depth_compute | 各 compute pass 声明与 dispatch（prior 契约 O，本域复验未推翻） | O |
| water_planes_draw / lilypad(_flower)/reed_draw / rocky_terrain_shader | draw 声明 + 冬季字节@+0x58==4 变体（prior） | O |

### U（关键 unknown，不伪造）

- 0x07954dbb 瓦片状态字语义；+0x48..+0x5f 次级 Vec 用途；subscribe 记录的消费方（谁读订阅 Vec@+0x78——candidate：AllWorldRasterRead::read，未证实）。
- WorldRaster::new 的 param_2 原型来源（静态表 DATA xref 0x1436953d0）。
- +0x29b 的**另一置位点**（write 只置 +0x299；0x29b 谁置仍 U）。
- ActiveRasterStroke +0x00/+0x08 字段名；solidify_fluid_into 的固化数学（间隔/权重）。
- ui_terrain_focus_mode 四闭包体语义；drawing_input_smoother 具体平滑核参数。
- rasterize_stroke 三 blend 闭包写入公式的 zeno 覆盖度定点细节（prior 已标边界，本域不重开）。
- WaterMaskSsbo/WaterExcludeClutterMaskSsbo/GardenMaskSsbo/RippleSourceSsbo 字段（PDB name-only）。

---

## 4. panic / 日志串清单（O，rne-read 直读）

| 原文 | 地址 | 出处 |
|---|---|---|
| "Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead." | 0x142ac85f8 | WorldRaster::subscribe（非 primary 发布器） |
| "Length mismatch; expected " | 0x142ac8648 / 0x142a93cf0 | copy_*/write 的 len 断言 |
| "Expected raster " | 0x142a93d80 | WorldRaster deserialize\<Value> |
| "End of stream while expecting raster " | 0x142a93dc8 | 同上 |
| "called `Result::unwrap()` on an `Err` value" | 0x142a93390 | write 的 len_mismatch 包装 |
| "Resource does not exist" / "Resource requested by " / " does not exist: " | 0x142a92ef0 区 | would_write_succeed（Bevy 资源缺失） |
| "Tried to init CurveU from an invalid u: " | 0x142aa53c0 | GladeBorder/Circle2d（utils circle.rs） |
| "MeshFollowTerrain got an invalid mesh handle" | 0x142ad50d8 | mesh_follow_terrain |
| "Was requested to edit a terrain stroke  that doesnt exist" | prior（edit_terrain_stroke 行 156） | edit_terrain_stroke |
| "water blur temp" | 0x142ac7cb0 区 | water_rasterize_from_cpu puffin scope |
| "Blur water mask" + "/gauss_blur_mask.cs.hlsl" | 0x142ac7cc8 区 | 同上 pass 名 |
| "Spawn lilypads" + "/water/spawn_lilypads.cs.hlsl" | 0x142ac7cfe 区 | water_spawn_vegetation pass1 |
| "Spawn reeds" + "/water/spawn_reeds.cs.hlsl" | 0x142ac7d48 区 | pass2 |
| "Out of bounds access" | 0x142ab3224 | str pattern（world_raster 路径） |
| "not yet implemented"（r16f/rgba16fr16f 分支） | prior | upload_raster_staging_to_texture |
| 源路径常量：crates/country-core/src/resources/{world_raster_defs,active_raster_stroke,terrain_height,terrain_editing}.rs、systems/{mesh_follow_terrain,history/replay_terrain,render_loop/compute_passes/compute_water}.rs、原仓 /home/h3/code/pouncelight/country-slice-private/…、puffin-0.14.1、rustc 05f9846f893b… | 多处（见 strings-extracted.txt） | panic location |

---

## 5. 与现有合同/代码的差异（“代码写 X，证据为 Y”）

### 5.1 vs `reconstruction/country-core-render/wp7/contracts/*`（water 3 份）

1. **water.md §2 布局表与 new() 实读不符**：合同写“+0x30 DirtyTiles/retain 闭包状态 + 16B 脏块元数据@+0x50..0x5f、+0x60/+0x68 Mutex”。机器码：+0x30..+0x40 是**位图 Vec {cap=0x1000,ptr,len=0x1000}（4096 u32 瓦片状态）**；+0x48..+0x5f 是次级 Vec {0x40,ptr,0x40}；+0x60..+0x97 是**从原型拷贝的 56B DirtyTiles 头**（含 Mutex——parking_lot 依赖 O，但初始化方式为拷贝而非原地）。→ 布局表需重写。
2. **water.md §2 “初始水位值/解码语义 unknown”**：初始瓦片字 0x07954dbb 已 O 提取（语义仍 U，但值已定位 0x142ac7e50）。
3. **water.md 栅格行数**：已由 512×512 澄清（prior water-extra），本域复验一致（0x40000B 分配）。
4. **water-systems.md §1b draw_raster 状态机**：与 0x2260ae0 反编译一致；补充状态机字段位置（+0x60 笔刷/+0x64 模式/+0x68 blend flag/+0x6c 计数，O）。
5. **water-extra-analysis 的“+0x29b 置位点未定位”**：仍 U，但注意 **AllWorldRasterWrite::write 置的是 +0x299 并 memset(+0x98,0xff)**——全脏上传由 +0x299 驱动，+0x29b 是另一个（更上层）再上传标志。
6. 合同 §1 “typed gap：BrushSizes” 已闭合（§2.5）：path 默认 0.8、garden 默认 3.0 为新 O 事实。

### 5.2 vs `reconstruction/country-core-render/wp8/contracts/*`（terrain 3 份 + world-raster-defs）

1. **world-raster-defs.md §1 DirtyTilePublisher 行号**：“行 23/29/47？”→ 实为 **64/68/72**（unique/shared/secondary）。
2. **world-raster-defs.md §2 “每帧 W1 上传”**：upload_to_staging 的 6 槽名序 O 确认（Path, Cob1..3, Water, Garden）；would_write_succeed=ordered_names 邻接（合同把 0x83eb10 标 validate_param——见 §7）。
3. **terrain-height.md §3 布局**：新 O：+0x08 为 **u16** 网格数据指针（合同写“内含 TerrainHeightFloatGrid，布局 U”——数据格式已 O）；+0x50/+0x54 min/max 复位逻辑 O（默认 -2.5/24.0 重确认）。
4. **terrain-height.md sample_at_world_xz 返回 u16**（合同未载）；debug 全局字节 *0x14362c308 走 +0x201 偏移路径（新 O）。
5. **terrain-systems.md §4 mouse_terrain_raycast**：复验一致；补充：高度场命中后与 **y=0 平面交点比距取近**（新 O 细节，0x8ef320 内 sqrtps 比距）。
6. **terrain-systems.md §4 CursorTerrainRaycast 布局**：一致。
7. **terrain-systems.md rasterize_terrain_stroke 的 active 笔画“0x30B”**：本域确认 ActiveRasterStroke 记录 = **0x470B**（index_mut imul 0x470），0x30B 是其中一列 Vec 的步长误读（每笔画含三个 Vec 列 + 位图）。

### 5.3 vs `reconstruction/country-core/src/**`（现有代码）

1. **brush_sizes.rs**：现有代码把 `water` 放结构体 +0（`pub water: f32` 为首字段）。原布局 **path@+0, water@+4, garden@+8**（代码写 X，证据为 Y——重排 + 补 path/garden clamp 与默认）。
2. **active_raster_stroke.rs**：stub 无布局。可落：0x470B 记录、三 Vec 列（+0x10/+0x28/+0x48）、+0x60 笔刷/+0x64 模式/+0x68 flag/+0x6c 计数、+0x70 起双位图；resample_and_smooth=**10 遍 0.25/0.5/0.25 平滑**（代码是 unimplemented）。
3. **world_raster.rs / world_raster_defs.rs**：现有 `WorldRaster(pub Gap)` 与 `*Raster(pub RasterBufferRef<512>)` 都是占位。O 事实：WorldRaster 本体 = composite/scratch 双缓冲 + DirtyTiles（位图+头+staging 512B+4 flags），named rasters 是 Deref 到 WorldRaster 的资源。`AllWorldRasterWrite` 6 槽名序可落。
4. **terrain_height.rs**：`copy_from_gpu(&mut self, &[u16])` 签名方向对（u16 O）；补：+0x50/54 min/max 与默认复位、8×8 瓦片 diff、sample_at_world_xz 的 u16 返回与 -0.5/+1 索引。
5. **mouse_terrain_raycast.rs**：现有 `camera_mouse_ray` 是 unimplemented；O 事实：0x8ef1c0 布局可补（CursorPositionSS→世界射线，Ray::from_mouse_ss 依赖）。
6. **read_mouse_depth_value.rs**：现有纯 Gap；O：MouseDepthValue = {w f32@0, xyz@+8} + 帧哨兵@+0x40，来源 MouseDepthValueSsbo 2×float4。
7. **garden_mask.rs / water_mask.rs / water_exclude_clutter_mask.rs**：`rg_buffer_hash() = 0` 占位与真实 type-id 不符（如水 staging 0x1ad182936286a0ee、WaterMaskSsbo 槽 0x13…），保持 typed gap 但应注明 hash 家族在 0x142c2e210 区常量表。
8. **systems/world_raster.rs**：`RasterStrokeCmd` 注释“elem 0xc B”正确（12B=Vec3）；可补 0x470 槽位与 6 槽名序。
9. **terrain_stroke.rs**：insert_or_morph_last_point 复验通过（0.001/1024 阈值 O）。
10. **terrain_rock_mask.rs**：prior 已 O，本域未推翻（sample 0x9ea800 位索引 `(z*512|x)>>5`）。

---

## 6. 跨领域线索（供中央索引）

| 本域函数/资源 | 跨域对象 | 地址 |
|---|---|---|
| WorldRaster::subscribe | tabloid DirtyTiles（依赖 crate tabloid 0.1.0，skeleton 依赖注解） | 订阅 Vec@+0x78 |
| ActiveRasterStroke::index_mut 0x470 | systems/world_raster.rs record_raster_history 的 idx·0x470 槽位（history 域） | 0x957990 |
| rasterize_stroke 闭包×3 | zeno::mask::Mask::blend_into（依赖，内联进 rasterize_write 8 实例） | 0x1425dcda0/570/db00 |
| DirtyTiles::retain/rasterize_world_space_stroke/extend/clear | utils::dirty_tiles（shared-foundation 域） | 0x140adfcb0 等 |
| bilinear_sample_at_world_xz\<RasterBufferRef\<512>> | utils::float_grid | 0x140a14df0 |
| GladeBorder::is_shape_inside | utils::geometry::circle（Circle2d/CurveU panic） | 0x142aa53c0 |
| draw_raster\<Water…> | system_water::draw_water（跨 crate，water 工具 id=15） | 0x141ba9130 |
| draw_raster\<Garden…> | systems::garden::draw_garden（工具 id=17） | 0xae4620 |
| draw_raster\<Path/Cobblestone…> | systems::paths（paths 域，closure_env$0..14） | 0xabba80 等 |
| replay_raster\<Water/Garden/…> | systems::history::replay_raster（history 域）+ replay_water/replay_garden | 0x9f4650 等 |
| TerrainHeightsData::copy_from_gpu | delayed_terrain_changed_msg（collision/wall 域消费 OnTerrainChange） | 0xaabdd0 |
| water_rasterize_from_cpu | frame W2 外层 render_world（0x1404ec110 调用点 0x1404ec2f6） | prior |
| upload_raster_staging_to_texture×6 | frame W1（render_world） | 0x3d8bd0 族 |
| update_water_audio_grid | WaterAudioGrid/WaterAmbienceAudio（audio 域） | 0x141baefe0 |
| terrain_hover_idx | ui_edit_terrain（ui-input 域，HoveredTerrainStrokeId 消费） | 0xaef1f0 |
| read_mouse_depth_value_from_gpu | mouse_depth_compute（frame compute，MouseDepthValueSsbo 生产者） | 0xa01620 |

---

## 7. 错位 / 纠错（sysres-procedures.json 地址↔PDB 权威对照）

| sysres 索引地址 | sysres 名称 | PDB 权威（骨架“编译实例 RVAs”） | 级别 |
|---|---|---|---|
| 0x140a15530 | DirtyTilePublisher::unique | **WorldRaster::new\<512>（行 136）**；真 unique=0x1409393b0 | 错位 |
| 0x140a15da0 | impl$0::serialize\<CompactFormatter> | **impl$2::clear_all\<512>（行 287）**；真 serialize=0x140a195a0(+0x3c3b0) | 错位 |
| 0x1409393e0 | copy_composite_to_scratch\<512> | **build_tile_hasher（行 562）**；真 copy_composite_to_scratch=0x140a161a0（+水 0x142262680） | 错位 |
| 0x140a197c0 | WorldRaster::as_float_grid | **impl$0::deserialize\<serde_json::Value>（行 261）**；as_float_grid 仅内联（71 实例，无编译实例） | 错位 |
| 0x14083eb10 | impl$0::validate_param | **AllWorldRasterWrite::ordered_names（行 703）** | 错位 |
| 0x14003bb80 | get_param + queue | **apply/new_archetype/queue（行 678，ICF 折叠=裸 ret）**；真 get_param=0x1401507ce0/0x1508130 | 错位 |
| 0x14067dd80 | impl$4::load_index_unchecked | 名同址 O，但**同址还有 impl$45::deref_mut / impl$69::deref（ICF 身份函数 `mov rax,rcx; ret`）**；多模块共享，勿当业务函数 | 警示 |
| 0x140868fb0 | ActiveRasterStroke::resample_and_smooth | **InProgressRasterStroke::add_point（行 107）**；真 resample_and_smooth=0x140868e40 | 错位 |
| 0x140ad3f50 | InProgressCurve::last_raw_input_point | **InProgressCurve::try_add_raw_input_points（行 134）** | 错位 |
| 0x14145c350 | RawInputSmoothed::point_count | **DrawingInputSmoother::process（行 30）** | 错位 |
| 0x140ad2970 | GladeBorder::is_pos_inside | **GladeBorder::is_shape_inside（行 54）**；真 is_pos_inside=0x140ad2950（跳板） | 错位 |
| —（索引缺） | — | WorldRaster::subscribe 0xa158e0、commit/copy_composite 模板实例 0xa16580/0xa161a0、copy_to_staging 0xa172d0、intersect_ray 0x9e9a20、sample_at_world_xz 0x9e9640、copy_from_gpu 0x9ea020、read_mouse_depth_value_from_gpu 0xa1a480、start_new 0x868d00、InProgressCurve::new 0xad3070 等 18 项（本域补充批已补齐，落盘 decomp/） | 缺项 |
| 0x1408e5bc0 | terrain_editing::serialize | 与 color_id::serialize 同址（ICF 合法，两名字皆真） | 说明 |

> 注：以上“真”地址均已反编译落盘于 `decomp/`，PDB 骨架 RVA 表另见 `../proc-locs.txt`（临时输出）；sysres 索引未修改（只读权威清单，修正登记于此）。

### 7b. v2 索引（S_PUB32 权威）交叉核对（协调方 2026-10-14 升级）

`sysres-procedures-v2.json`（rustc-demangle S_PUB32）与本域清单对照：**上表全部“真”地址与 v2 逐一吻合**（如
0x140a15530=WorldRaster::new、0x1409393e0=build_tile_hasher、0x140868fb0=InProgressRasterStroke::add_point、
0x14145c350=DrawingInputSmoother::process、0x140ad2970=is_shape_inside 等），本域错位表升级为双重确认。

v2 域内 136 行 vs 本域 124 份反编译（92+18+13 批，含 NotFound 手解）。v2 特有增量与处置：

| v2 地址 | 名称 | 处置 |
|---|---|---|
| 0x140257960 / 0x141246fb0 | sample_at_world_xz 多个哈希实例（含 VisualTerrainHeightSsbo 变体） | 已补反编译 0x141246fb0：**双线性变体**（4 角 load_index_unchecked + 3 lerp），权威实例 0x9e9640 返回 u16 |
| 0x1412773f0 | intersect_ray（视觉实例） | 已补反编译（9KB） |
| 0x142612be0 | WorldRaster::subscribe 第二实例 | 已补反编译，与 0xa158e0 同构 |
| 0x1408b54c0 | TerrainHeightFloatGrid::load_index_unchecked | 0x141246fb0 内 **4 次调用**证实其角色 = 网格单元装载（u16→f32 解码，含 +0x50/54 min/max）；0x14067dd80 只是 ICF 折叠的身份函数（索引残留） |
| 0x140957990 / 0x140958040 / 0x1409580e0 | record_raster_history / RasterDrawResources::init_state / RasterHistoryRecordResources::init_state | 已补反编译（record = 0x30B Arc 记录 + 12B 元素 Vec 克隆 + History::add 闭包 + reset cap 4，O） |
| 0x140aabdc0 / 0x140aabdd0 / 0x140ac9770 | on_terrain_modified（=`LOCK inc [rcx]; ret`，O 手解）/ delayed_terrain_changed_msg / history_record_terrain | 已补反编译（delayed 5.9KB 与 prior asm 一致） |
| 0x141695e10 / 0x1408c7120 / 0x14098c680 / 0x140aef1f0 | record_garden_history / rasterize_terrain_stroke / read_terrain_data / ui_edit_terrain | 已补反编译落盘（ui_edit_terrain 属 ui-input 域共享，此处只做地址确认） |
| 0x1404f41b0 / 0x1417c1590 / 0x1417c1ac0 / 0x14003c3b0 / 0x140a195a0 / 0x1400e7480 / 0x1408e6b70 / 0x14157d970 | serde Serialize/Deserialize 单态（terrain_edit / terrain_editing / terrain_stroke / world_raster） | 按项目规则复用 derive 胶水，不反编译 |

---

## 8. 方法学备注

- 批量反编译 92（83 OK / 9 NotFound 全为 <40B 小函数，rne-read --hex 手解）+ 补充批 18（15 OK / 3 NotFound 手解或 ICF 说明）。
- 字符串批量提取用 `rne-read.py --unks`（152 行原始，已过滤存 strings-extracted.txt）。
- 未修改 reconstruction 源码；全部产出为文档与证据文件。
