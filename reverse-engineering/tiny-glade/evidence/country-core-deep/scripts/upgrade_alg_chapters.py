#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""在 chapters/** 就地追加"实现方式（算法级）"节（terrain-water + camera-time 域）。
幂等：已含"## 实现方式（算法级）"的文件跳过。用法：python upgrade_alg_chapters.py"""
import os, sys, io

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "chapters")

SECTIONS = {}

# ============================================================
# terrain-water 域
# ============================================================

SECTIONS["resources/world_raster.rs.md"] = r"""
## 实现方式（算法级）

### 处理链（O，本轮修正一处布局）

```
AllWorldRasterWrite::write(up_to)  每槽(0..up_to)：
  槽 Option=None → memset(composite,0,len)（资源缺失→清空）
  否则 len 断言("Length mismatch; expected ", 行43) → memcpy 槽数据→composite
  memset(+0x98, 0xff, 0x200)   ← 4096-bit 脏瓦片位图全置1（非"staging 缓冲"；每 8×8px 瓦片 1 bit）
  +0x299 = 1                    ← staging 脏标志
→ upload_to_staging（0x83fa60）：6 槽按 type-id 查 *StagingSsbo → copy_to_staging(selector=0)
→ copy_to_staging（0xa172d0）：+0x299 脏→memset(out,0,len)；+0x298=selector；
    DirtyTiles::retain(bitmap@+0x98, frame) 按脏瓦片拷入 SSBO 映射区 → tabloid publish → +0x29a=1
```

### 逐函数算法

| 函数 | 算法 |
|---|---|
| WorldRaster::new<512>（0xa15530） | alloc composite+scratch 各 0x40000B + 瓦片状态 Vec(4096×u32, 初值 0x07954dbb) + 拷贝 56B DirtyTiles 头模板 + memset(+0x98,0,0x200) 清位图 |
| clear_buffer（0xa15a10）/ clear_all（0xa15da0） | 按选择字节清单缓冲（0x200 段或全 0x40000）；clear_all 双缓冲全清 + 4096 瓦片状态重置 0x07954dbb（SIMD+标量尾） |
| copy_composite_to_scratch / commit_scratch_to_composite | 纯 memcpy + len 断言 |
| rasterize_stroke（0xa16960） | 点数<3 早退；点列→zeno Command 表（28B/条，首点 move-to 其余 line-to，grid=(w/130+0.5)*512）；blend 闭包按 (arg6,bit0) 选 min/max/饱和加×笔刷(16.16)；(0,1)→panic；目标层按模式字节选 composite/scratch |
| copy_to_staging（0xa172d0） | 见处理链；+0x50..0x5c 4×u32 脏瓦片元数据进 retain 帧 |
| build_tile_hasher（0x9393e0） | 写 4×u64 ahash 种子（0xbe734ee571db45b0 / 0xfca4335f3c9b800a / 0x36de4a1709f2df62 / 0xee5497aefccb8a97）到 +0/+8/+0x10/+0x18 |
| DirtyTilePublisher::unique | 38B：判别 0 + 双空 Vec(cap=8) |
| subscribe（0xa158e0） | primary 校验（secondary→panic "Can't subscribe…"）；0x28B Arc 记录 {Arc 计数@0, lock@0x10, Arc<闭包>@0x20} 压入 +0x78 Vec；tabloid publish 换装闭包消费 |
| write（0x83ebe0） | 见处理链 |
| would_write_succeed（0x83f410） | 按槽比对类型名做写冲突检测；缺失资源→Bevy panic |
| upload_to_staging（0x83fa60） | 6 槽 type-id 查 SSBO + memcmp 全名 → vtable 映射指针 → copy_to_staging(selector=0) |
| PathRaster::new（0x83eaf0） | 委托 WorldRaster::new |

### panic 分支

- "Length mismatch; expected "（0x142ac8648/0x142a93cf0，行 43 world_raster_defs.rs，unwrap Err 包装）
- "Can't subscribe to a secondary dirty tile publisher. Go for the primary one instead."（0x142ac85f8）
- "Expected raster " / "End of stream while expecting raster "（deserialize\<Value>，0x142a93d80/8）
- 尾部 panic/abort 胶水：`lea rcx,[world_raster_defs.rs fat]; call 0x1428d9310; int3`（0x14083f2d6）
"""
SECTIONS["resources/active_raster_stroke.rs.md"] = r"""
## 实现方式（算法级）

### 状态机（O）

```
start_new(brush, mode, flag, cnt)：清三 Vec → 写 +0x60/+0x64/+0x68/+0x6c → add_point(首点) → 推入平滑列
add_point(pt)：
  Vec2 空 → push；否则 arc(+0x40) += dist(last,pt)；push（cap 满先 grow）
  arc > 1.0 → solidify_fluid_into(Vec2.data, Vec2.len, &mut Vec1) 固化前缀 + memmove 剩余前移
resample_and_smooth()：
  1. Vec1 整列拷贝 → Vec3（12B SIMD）
  2. solidify_fluid_into(Vec2.data, Vec2.len, &mut Vec3)
  3. 10 遍 3-tap 平滑：new[i] = ((p[i-1]+p[i])·0.5 − p[i+1])·0.5 + p[i+1] = 0.25/0.5/0.25
     （0.5×4 @0x14293ac60..6c；外层 while lVar15 != 10）
solidify_fluid_into(points, len, &mut dst)：
  视图 FUN_141308760 → len<2 单点拷贝；否则 FUN_141467fa0(view, 0.5) 0.5 世界单位定距重采样
  → dst 空 grow+memcpy 全段 / 非空 FUN_1412f9e10 尾点合并追加；free 中间缓冲
InProgressRasterStroke::last：Vec2.len==0 → panic(Option::unwrap)；否则 &points[len-1]（12B 步长）
```

### 数据结构（0x470B）

+0x00 活动/流体标志；+0x08 流体游标指针；+0x10 Vec1(固化曲线 12B)；+0x28 Vec2(输入点)；+0x40 弧长 f32；+0x48 Vec3(平滑后曲线，draw_raster 消费)；+0x60 笔刷；+0x64 模式字节；+0x68 blend 标志(bit0)；+0x6c 计数；+0x70..+0x468 双 512B 脏位图。

### panic 分支

- `last` 空列 → Option unwrap panic；solidify 重采样溢出 panic（行 43，active_raster_stroke.rs）
"""
SECTIONS["resources/raster_stroke.rs.md"] = r"""
## 实现方式（算法级）

- `RasterStroke { points: Arc<[Vec3]>, radius: f32 }`；`iter_points_xz`（0xa97860）= points 迭代映射为 (x,z) 对（12B Vec3 → 8B (x,z)），rasterize_stroke 的输入迭代器（50 处内联宿主）。
- `add_point` = Arc 重建（不可变共享，push 新点）；`smooth` = 点列平滑（11 处内联，宿主同族）；`length` = 折线总长。
- `radius()`/`length()` 为 draw_raster 笔刷/时长消费者。
"""
SECTIONS["resources/brush_sizes.rs.md"] = r"""
## 实现方式（算法级）

| 槽位 | 默认(reset) | set=clamp | 常量地址 |
|---|---|---|---|
| path @+0 | 0.8（0x3f4ccccd） | clamp(v, 0.32, 1.2) | 0x142ac56c0 / 0x1429cd128 |
| water @+4 | 2.0（0x40000000） | clamp(v, 1.0, 5.0) | 0x142925904 / 0x142925988 |
| garden @+8 | 3.0（0x40400000） | clamp(v, 2.0, 4.0) | 0x14292e6f0 / 0x142934694 |

6 个 set/reset 全为 <40B 手解函数（`mov [rcx+off],const` / `maxss+minss` 钳位）。draw_water/draw_path/draw_garden 各读对应槽位作笔刷半径。
"""
SECTIONS["resources/terrain_height.rs.md"] = r"""
## 实现方式（算法级）

### 双实例（O）

`TerrainHeightsData<T>` 泛型：`TerrainHeightSsbo`（权威 0x1409e9640/0x1409e9a20/0x1409ea020）、`VisualTerrainHeightSsbo`（视觉双线性 0x141246fb0 / 视觉 intersect 0x1412773f0）。布局：+0x08 u16 512×512 网格（512KB）；+0x50/+0x54 min/max（默认 -2.5/24.0 @0x142a9f3f0）。

### 采样

```
sample_at_world_xz 权威（0x9e9640）：
  u = (world.xz/130) + 0.5
  *0x14362c308==0：u=clamp01(u) → idx = fptoui_sat(u*512) → grid[(z<<9|x) + 0x201]  ← +0x201 为正常分支（本轮修正 DIGEST）
  否则：idx 域 clamp → grid[z<<9|x]（无偏移）
sample_at_world_xz 视觉双线性（0x141246fb0）：
  load_index_unchecked ×4（0x1408b54c0，u16→f32 解码含 min/max）+ 3×lerp（func_0x000140c904d0）
```

### intersect_ray（0x1409e9a20）

```
AABB（xz ±65、y=[min,max]）slab 求进出 t（FUN_141240860）→ 无 hit miss
t_near=max(t_near,0)；t_far=min(t_max,t_far)
32 步行进（step=(t_far−t_near)/32，0x142ac4cc0）：ray.y(t) < height(t) → 命中
每命中步 3 次 0.5 二分精化（0x142925870）+ 尾次合并 → t_hit
与 y=min 平面交点（FUN_1412803f0）比距取近
out = {tag, y, x, z, t}
```

### copy_from_gpu（0x1409ea020）

```
memset(局部脏位图[64×u64],0)；min/max 非默认 → 复位 {-2.5,24.0}
for 64×64 个 8×8px 瓦片（行=8×512px unroll）:
  逐 16B 块（8×u16）diff：不同 → copy 16B + dirty[行] |= 1<<列
```

### panic / 调试

- debug 全局字节 *0x14362c308（0 走 +0x201 分支；+0x201 语义 U——halo/边际行缓冲候选）
- copy_from_gpu 未发现 panic（长度按全网格固定）
"""
SECTIONS["resources/terrain_rock_mask.rs.md"] = r"""
## 实现方式（算法级）

`sample_at_world_xz`（0x1409ea800）：`u=(x/130+0.5)` clamp[0,1] → ×512 索引；`(z*512|x)>>5` 字索引边界检查后取位（512×512 位栅格 = 8192×u32，位打包）。prior 已 O，本域未推翻。
"""
SECTIONS["resources/terrain_editing.rs.md"] = r"""
## 实现方式（算法级）

`TerrainEditingData {HashMap<TerrainStrokeId,TerrainStroke>（数据@+8/掩码@+0x10/计数@+0x20）, last_stroke_id u32@+0x38}`。

- `new_stroke`（0x924a10）：last_stroke_id < 0x3fffff 则 +1 并 HashMap::insert；否则 None。
- `get_last_mut_stroke`（0x924900）：按 last_stroke_id 查 HashMap 取可变引用（缺失→Option None，调用方 edit_terrain_stroke panic "Was requested to edit a terrain stroke  that doesnt exist"）。
- serde：TerrainEditingData 存档序列化（CompactFormatter 0x1400e7480 / Value 0x1408e6b70；derive 胶水复用）。
"""
SECTIONS["resources/terrain_stroke.rs.md"] = r"""
## 实现方式（算法级）

`insert_or_morph_last_point`（0xa97880）判定序列：
```
len==0 → push
len==1 且 dist > 0.001 → push
len≥2 且 dist(last, prev) ≥ spacing → push（<1024 上限）
否则覆盖 last（morph）——同一点刷新末点
```
0.001 单点阈值 + spacing 比较（prior 复验通过）。
"""
SECTIONS["resources/water.rs.md"] = r"""
## 实现方式（算法级）

无独立业务过程（PDB name-only）。RippleSourceSsbo = 鸭子入水波纹源（system_critters::duck::update_duck_ripples 生产 / water_planes_draw 消费）；WaterDrawEvent/WaterEraseEvent = draw_raster/replay_raster 写入。CPU 水位本体在 world_raster_defs::WaterRaster（WorldRaster<512> 槽 4，见 ALGORITHMS.md §1）。
"""
SECTIONS["resources/water_mask.rs.md"] = r"""
## 实现方式（算法级）

无独立业务过程（name-only）。WaterMaskSsbo = 512×512 水 mask 纹理 .y 通道打包回读（>0.5 水位位 / >0.2 clutter 位，read_water_mask.cs.hlsl O）；read_terrain_data 槽位回拷消费者。字段布局 U。
"""
SECTIONS["resources/water_exclude_clutter_mask.rs.md"] = r"""
## 实现方式（算法级）

无独立业务过程（name-only）。WaterExcludeClutterMaskSsbo = 水域 shoreline .y>0.2 位打包回读（水域内不摆 clutter/装饰物）。字段布局 U。
"""
SECTIONS["resources/garden_mask.rs.md"] = r"""
## 实现方式（算法级）

无独立业务过程（name-only）。GardenMaskSsbo = 花园掩码回读缓冲（世界栅格 512² 域，read_terrain_data 槽 0x14 回拷）。字段布局 U。
"""
SECTIONS["resources/mouse_terrain_raycast.rs.md"] = r"""
## 实现方式（算法级）

- `intersect_with_xz_plane`（0x8ef180）：t=(y−pos.y)/dir.y；返回 pos+t·dir。
- `camera_mouse_ray`（0x8ef1c0）：CursorPositionSS 屏幕位置 → `Ray::from_mouse_ss` 世界射线。
- `mouse_terrain_raycast`（0x8ef320）：CameraCursorRay 有效 → xz 平面解；否则 `TerrainHeightsData::intersect_ray`（32 步行进+3 二分）；与 y=0 平面交点比距取近；closure$0 网格精化。写 pos/dir-delta/length。
"""
SECTIONS["resources/cursor_terrain_raycast.rs.md"] = r"""
## 实现方式（算法级）

`CursorTerrainRaycast {tag u8@0, pos Vec3@4}`（Option<Vec3> 语义）：`position`（0xa12650）= tag&1 非零返回 pos，否则 {0,0,0}；`update` = 由射线写入命中位置（Some→tag=1+pos；None→tag=0）。地形/水/花园/路径工具读取光标落点（draw_raster 输入源之一）。
"""
SECTIONS["resources/read_mouse_depth_value.rs.md"] = r"""
## 实现方式（算法级）

`read_mouse_depth_value_from_gpu`（0xa1a480）：帧哨兵@+0x40 == i64::MIN+3 早退（帧有效性）；查 MouseDepthValueSsbo（名 0x47=71 字符）→ 读 2×float4 → NaN 过滤（`&0x7fffffff < 0x7f800000`）→ 写 {w f32@0, xyz@+8}。生产者 = mouse_depth_compute（frame compute）。
"""
SECTIONS["resources/in_progress_curve.rs.md"] = r"""
## 实现方式（算法级）

- `new`（0xad3070）：空曲线 Vec 初始化。
- `try_add_raw_input_points`（0xad3f50）：先调 FUN_141466860 输入验证（返回 3 为合法），非法分支清两字段；合法则批量加入原始输入点。
- `compute_final_curve`（0xad41a0）：由原始输入点计算终结曲线（重采样/拟合，16KB）。
- 辅助（内联）：last/first_raw_input_point、raw_input_curve_length_ws、add_weight_to_the_last_point、is_raw_input_curve_valid。wall 域 freehand 墙为主消费者。
"""
SECTIONS["resources/drawing_input_smoother.rs.md"] = r"""
## 实现方式（算法级）

`DrawingInputSmoother::process`（0x145c350）= **弧长均匀重采样 + 最小间距抽稀**（非拉普拉斯平滑）：
```
沿输入折线 t=i/N 参数步进 lerp 取样（fVar18 = t·fVar1 + (fVar5−t)·prev.x）
fVar6 < dist && fVar7 < dist·fVar11 → 跳过过密点
输出 8B (x,z) 对入 Vec；回写 param_2（首个输出点）+ 置位标志
```
"""
SECTIONS["resources/glade_border.rs.md"] = r"""
## 实现方式（算法级）

- `is_pos_inside`（0xad2950）= 跳板：`test [rax],1; jnz 共享主体` → 委派 `is_shape_inside`（0xad2970）主实现。
- `is_shape_inside`：CurveU 边界校验（u∉[0,1) → panic "Tried to init CurveU from an invalid u: "，0x142aa53c0，utils circle.rs）+ 形状采样比较。
- `search_lerped_shape_that_fits_into_glade`（0xad2ce0）：界内 lerp 缩放搜索可容纳形状（6.6KB）。
- 常量：1.5/0.3/0.4/0.5/1.0/2.0/4.0/0.8（draw_garden 用）。
"""
SECTIONS["resources/rocky_terrain.rs.md"] = r"""
## 实现方式（算法级）

`RockyTerrainMesh::load`（0x8bbf50）：`fs::read_to_string`（FUN_140d59140）→ serde_json 解析 → `add_new_with_data<RockyTerrainVertexSSBO/TriangleSSBO>`；顶点源记录 0x24B→0x30B、三角形 0x10B→0x20B。JSON 中间结构 LoadedRockyTerrainVertex/Triangle（derive 胶水）。displace_rocky_terrain 每帧 GPU 变形。
"""

SECTIONS["systems/world_raster.rs.md"] = r"""
## 实现方式（算法级）

### draw_raster<T> 状态机（10 实例；水 0x142260ae0 精读）

```
state==0 → start_new(stroke, cursor, mode, brush)
state==1：
  last()（空→panic）→ dist(last,point) ≤ 0.0005（0x1429cd060）早退
  add_point → resample_and_smooth
  DirtyTiles::clear(stroke.bitmap@+0x270)
  rasterize_world_space_stroke(bitmap, iter_points_xz(smoothed@+0x48), first, brush@+0x60)
  DirtyTiles::extend(worldraster.bitmap@+0x98, stroke.bitmap)（×2）
  swap(stroke.bitmap 0x70.. 与 0x270.. 双 512B 半)
  copy_composite_to_scratch(worldraster)
  rasterize_stroke(worldraster, iter_points_xz(smoothed), ..., mode, brush)
  map StagingSsbo → copy_to_staging(selector=0)
其余状态 → commit_scratch_to_composite 发布
按 +0x64 模式发 Draw/Erase 事件（WaterDraw/WaterErase 等）
```

### 输入源 / 槽位

- 点：CursorTerrainRaycast::position（func_0x000140a12650）；平滑后曲线列 Vec3@+0x48 是 rasterize_stroke 唯一输入（iter_points_xz 0x140a97860）。
- ActiveRasterStroke 槽位：index_mut 0x868cf0，imul 0x470；记录_raster_history 同槽位（record_raster_history 0x957990 = 12B 元素 Vec 克隆 → History::add → 重置 cap 4）。
- upload_world_raster_buffers_to_staging（0x957bf0）：脏输入流（EventReader 双缓冲差）非空 → upload_to_staging（6 光栅 composite→staging SSBO）；log/panic module path country_core::systems::world_raster。
"""
SECTIONS["systems/terrain/mod.rs.md"] = r"""
## 实现方式（算法级）

- `add_terrain_systems`（0x9455f0）：Bevy Plugin 系统注册（register_required_components 胶水 + add_systems 族）。
- `insert_terrain_resources`（0x9457c0）：TerrainHeightsData 双实例（权威/视觉）+ 网格等资源插入。
- `add_terrain_events`（0x945a20）：地形事件声明/注册。
"""
SECTIONS["systems/terrain/delayed_terrain_changed_msg.rs.md"] = r"""
## 实现方式（算法级）

`ScheduledTerrainReadback {counter AtomicUsize@0, flag bool@8, delays Vec<u32>}`：
- `on_terrain_modified`（0xaabdc0）= `LOCK inc [rcx]; ret`（修改计数原子自增）。
- `delayed_terrain_changed_msg`（0xaabdd0）：delays 逐项递减 → 0：FrozenMap 取 SSBO（尺寸断言 0x3a/0x40）→ cpu_readback → `copy_from_gpu×2`（权威+视觉）→ 压实完成项；flag 置位 push(2)（延迟 2 帧）发 OnTerrainChange。
"""
SECTIONS["systems/terrain/draw_terrain_stroke.rs.md"] = r"""
## 实现方式（算法级）

`terrain_stroke_changed`（0xa03ba0）：EvTerrainStrokeChanged 非空 → 发 RecalculateHeightMapCmd + PlayTerrainSound（转发生效链，非本域实现）。
"""
SECTIONS["systems/terrain/edit_terrain_stroke.rs.md"] = r"""
## 实现方式（算法级）

`edit_terrain_stroke`（0x94e000）：按命令 id 查 TerrainEditingData（缺失 panic "Was requested to edit a terrain stroke  that doesnt exist"）；falloff∈[0,1] / height（Abs≥0 / Delta≥0.1）应用（prior tg-terrain apply_resolved）→ RecalculateHeightMapCmd + PlayTerrainSound。
"""
SECTIONS["systems/terrain/terrain_hover_idx.rs.md"] = r"""
## 实现方式（算法级）

`terrain_hover_idx`（0xaff720）：AppMode 位掩码 0x421 门 → MouseTerrainRaycastSsbo（名 0x6c=108 字符）映射读 `id=*(i32*)(buf+0x10)`（-1 哨兵）；水位采样≤0 / 光标无效 / 距离≤0.5 不更新 → 写 HoveredTerrainStrokeId{tag=1, id}。
"""
SECTIONS["systems/terrain/ui_draw_terrain.rs.md"] = r"""
## 实现方式（算法级）

`ui_draw_terrain`（0xa9fdc0）：Res<AppMode, CursorTerrainRaycast, AssetSsboLibrary, WaterRaster, UiState, GladeBorder> + ResMut<TerrainEditingData, PendingTerrainHistory, DecoratorArchivist> + 事件组；点按新建笔画 `new_stroke` + EvTerrainStrokeChanged；界外反馈（DisplayOutOfBorder/FeedbackInputOutOfBorder）。状态机细节归 ui-input 域。
"""
SECTIONS["systems/terrain/ui_edit_terrain.rs.md"] = r"""
## 实现方式（算法级）

`ui_edit_terrain`（0xaef1f0）：按鼠标落点 + hover 笔画 id 生成 EditTerrainStrokeCmd/SetTerrainStrokePtsCmd；事件组含 CursorCaptureCmd/EnableArrowGizmoStretch/AddTerrainHighlightCmd 等；维护 MouseActivityTimer/TerrainStrokeParamsCache。ui-input 域主责。
"""
SECTIONS["systems/terrain/ui_terrain_focus_mode.rs.md"] = r"""
## 实现方式（算法级）

`ui_terrain_focus_mode`（0x9a4fb0）：167KB 近全内联（alloc/memcpy + HashMap helper 0x140c9cde0×9）；闭包 4 个 O 地址：0x9a3620（FUN_140c97e10 曲线采样，0.5 参数）/ 0x9a8640 / 0x9a88c0 / 0x9a8b40（状态转换，OutlineWallCmd/PublicWalls 相关）。内部状态名 U（ui-input 域 owns）。
"""
SECTIONS["systems/terrain/deferred_terrain_history.rs.md"] = r"""
## 实现方式（算法级）

`history_record_terrain`（0xac9770）：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史记录 History::add + DebugHistoryMsg；之后不重复记录。
"""
SECTIONS["systems/garden/draw_garden.rs.md"] = r"""
## 实现方式（算法级）

`draw_garden`（0xae4620）：工具 id 分发（`id-6<0xf` 且 `id-5∈{4,0xc}`；9=橡皮/17=花丛）；Circle2d 半径 = 笔刷 + 1.5（0x1429258d0）；GladeBorder::is_shape_inside 界内校验 → draw_raster<GardenRasterStagingSsbo>。
"""
SECTIONS["systems/garden/record_garden_history.rs.md"] = r"""
## 实现方式（算法级）

`record_garden_history`（0x141695e10）：把当前花园编辑记录（12B 编辑单元 Arc）封装历史项 History::add（record_raster_history 同族）。
"""
SECTIONS["systems/history/replay_terrain.rs.md"] = r"""
## 实现方式（算法级）

- `replay_terrain`（0x995150）：前桶判别 3 命令；按 ReplayCmd 0x40B 条目迭代（重量 f32@+0x14、标志字节）。
- `TerrainReplayCmd::forward`（0x994f30）/`backward`（0x995040）：duration = clamp(Σ点距·0.01, 0.4, 3.0)（无点 0.3；stroke_length 有限断言）；undo(+0x1c)==0 → reapply，!=0 → revert；动画 ease_out（系数歧义 U；reconstruction 用 1-(1-t)^3）。
"""
SECTIONS["systems/history/replay_water.rs.md"] = r"""
## 实现方式（算法级）

`replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, WaterFilter>` 薄包装（单态 0x1409f4650）+ 命令生成闭包（宿主 0x141444c70）。主算法在 history/replay_raster.rs。
"""
SECTIONS["systems/history/replay_garden.rs.md"] = r"""
## 实现方式（算法级）

`replay_raster<GardenRasterStagingSsbo, WorldRaster<512>, GardenFilter>` 薄包装（单态 0x1409f6ff0）+ 命令生成闭包（generate_replay_cmds_from_edit 宿主 0x14143d2c0）。主算法在 history/replay_raster.rs。
"""
SECTIONS["systems/mesh_follow_terrain.rs.md"] = r"""
## 实现方式（算法级）

`mesh_follow_terrain`（0xa68430）：Query<(&MeshFollowTerrain, &mut Transform, &Handle<Mesh>)> + 2×Res（地形高度数据）；FUN_14092d300 取 mesh、FUN_140a07b30 取几何；取 "Vertex_Position" 属性（0x142ad5120，0xf）→ FUN_1412fd550 位置计算（内联地形采样）→ Transform.y 更新；无效句柄 panic "MeshFollowTerrain got an invalid mesh handle"（0x142ad50d8）。
"""
SECTIONS["systems/render_loop/compute_passes/compute_water.rs.md"] = r"""
## 实现方式（算法级）

### water_rasterize_from_cpu（W2，0x140a12680）

```
ssbo = 库按 type-id 0x1ad182936286a0ee 查 WaterRasterStagingSsbo
if raster+0x29b（mask 重传脏）:
  ssbo.version(+0x5c) ← 库 version(+0x5b0)；清 +0x29b；memset(bitmap@+0x98, 0, 0x200)
  建纹理（格式 0x61 = rgba16fr16f）
  pass：puffin "water blur temp"/"Blur water mask" + /gauss_blur_mask.cs.hlsl
  → 水 draw mask 上传 → write_texture + dispatch_2d
```

### water_spawn_vegetation（W13，0xa13970）

两 compute pass："Spawn lilypads" /water/spawn_lilypads.cs.hlsl（LilypadSsbo+LilypadFlowerSsbo）、"Spawn reeds" /water/spawn_reeds.cs.hlsl（ReedSsbo）+ indirect draw 缓冲；ActiveSession::is_autumn + bind_terrain_heightmap + dispatch。帧序 W2→W13。
"""
SECTIONS["systems/render_loop/compute_passes/compute_garden.rs.md"] = r"""
## 实现方式（算法级）

### garden_rasterize_from_cpu（W12，0x140a22140）

同 water 模板：+0x29b 判 → ssbo.version(+0x5c) ← 库(+0x5b0)；清 +0x29b；memset(bitmap@+0x98, 0, 0x200)；建纹理（0x61）；on_texture_processed + write_texture + dispatch_2d。

### garden_spawn_vegetation（W16，0xa22b80）

"Garden Marching Squares" /garden/marching_squares.cs.hlsl（方格行进，CPU 掩码→SSBO）+ /garden/spawn_small_flowers_garden.cs.hlsl + /garden/spawn_{bushes_garden,flowers_garden}.cs.hlsl；bind_terrain_heightmap + write_draw_indirect_cmd_buffer + dispatch。

**round3 闭合**：Flower03Ssbo 库 SsboId = 0xF78C9C2EB9E1F6FB（add_new 立即数 O）；IvyFlowerSsbo=0x1d6de5e69ac50d2a、LilypadFlowerSsbo=0xAE37CBCAFBE88DA4。帧序 W12→W16。
"""
SECTIONS["systems/render_loop/compute_passes/upload_world_raster.rs.md"] = r"""
## 实现方式（算法级）

### upload_raster_staging_to_texture<T>（W1，6 单态）

```
ssbo = 库按 type-id hash（如 Path 0x85c987d23d58d2de）查 *RasterStagingSsbo
if ssbo && published(+0x29a) != 0:
  建纹理（格式 0x4c="r16f"→4 通道 / 0x61="rgba16fr16f"→7 通道；其它 → panic "not yet implemented"）
  ssbo.version(+0x5c) ← 库 version(+0x5b0)
  *(u16*)(+0x29a) = 0x100   → +0x29a=0（清已发布）、+0x29b=1（mask 重传置位）
  compute pass："Upload cpu raster" /upload_cpu_raster.cs.hlsl → write_texture + dispatch_2d
else: 不发布则跳过（无脏不上传）
```

**+0x29b 置位方**（闭合 round2 U）：本函数发布后的 word 写 0x100。消费方 = water/garden/path-masks 三处 rasterize_from_cpu（见 ALGORITHMS.md §1.5）。
"""
SECTIONS["systems/render_loop/draw_passes/draw_shaders/water_planes_draw.rs.md"] = r"""
## 实现方式（算法级）

`water_planes_draw`（0x9d9bc0）/`lilypad_draw`（0x9da580，读 LilypadSsbo）/`lilypad_flower_draw`（0x9da760，读 LilypadFlowerSsbo）/`reed_draw`（0x9da940，读 ReedSsbo）各 = DrawBuilder draw/draw_instanced 声明；水生植物 draw 消费 compute_water（W13）实例；冬季字节@+0x58==4 变体（prior）。
"""
SECTIONS["systems/audio/water_audio.rs.md"] = r"""
## 实现方式（算法级）

`water_audio_system`（0x873630）：按 WaterRaster 水位采样驱动音频混合（4 循环 + mode）；`water_audio_init_system`（0x873140）初始化 WaterAudioState/音频源。细节归 audio-misc 域。
"""
SECTIONS["systems/audio/water_ambience_audio.rs.md"] = r"""
## 实现方式（算法级）

`water_ambience_audio_system`（0x1409ecea0）：消费 WaterAudioGrid 入水计数（24B 单元/64 位滚动窗/6400.0² 阈值/0.5 水位阈值 prior O）驱动逐格 Amb 循环播放器；`impl$0::load_index_unchecked`（0x1409ebe80）网格单元装载；init 0x1409ebf20。
"""
SECTIONS["systems/audio/garden_audio.rs.md"] = r"""
## 实现方式（算法级）

`garden_audio_system`（0xa89480）：消费花园编辑/掩码事件驱动音频（4 循环 + update_draw_audio 模板）；init 0xa88fa0。细节归 audio-misc 域。
"""
SECTIONS["systems/audio/terrain_audio.rs.md"] = r"""
## 实现方式（算法级）

`terrain_audio_system`（0x1414618c0）：消费 PlayTerrainSound 事件播放地形音效（4 循环 + EditStateAudio 编辑状态机）；init 0x1414613e0（7.6KB）。细节归 audio-misc 域。
"""
SECTIONS["systems/audio/terrain_stairs_audio.rs.md"] = r"""
## 实现方式（算法级）

`terrain_stairs_audio_system`（0x141462dc0）：消费楼梯地形交互事件驱动音频（SuppressPathSound + StairsCounter 状态 + StairDraw_OneShot 瞬发）；init 内联。细节归 audio-misc 域。
"""

# ============================================================
# camera-time 域
# ============================================================

SECTIONS["systems/camera_rig.rs.md"] = r"""
## 实现方式（算法级）

### 模式→body/fov 分发（O）

`CameraModeStates::current_dyn`（0x1409149d0）：模式@+0x3d0 jumptable → {body 指针, 表, (fov_current,fov_target) 对}（Normal/TiltShift/Free/Walk/TopDown/Butterfly + 模式 6）。
focus 表 @0x142aabb68（13 项，相对 +0x100）；lens(fov) 表 @0x142aabb98（13 项，相对 +0x108）。
访问器（hex 手解）：mode 0x140911230 `movzx eax,[rcx+0x4d0]`；focus 0x140911240 / lens 0x140911260 表驱动；target_vertical_fov 0x1409114d0；immediately_set 0x1409114f0 尾调 body setter。

### 关键算法

```
match_free_camera_to_normal（0x14090e6f0）：
  读 Normal pos(+0x138..144)/yaw-pitch(+0x150) → 重建 Free rig@+0x270 = Position+YawPitch+Smooth(1,1)
  → dt=0 跑一遍存 final_transform（与 FreeCamera::set 0x140993100 同构建器）
zoom（0x140911280，582B 手解）：
  amount/anchor 有限性检查 → factor=exp2(clamp(amount·−0.08,−1,1))
  Normal→+0x100 / TiltShift→+0x1b8（lower 查表） / TopDown→+0x390
  active_rotation(+0x18)==1 → target_arm_length@+0x20=clamp(factor·target, lower, radial_max@+0x64)
  否则 offset=(pos+pending)−anchor × factor 写回 pending
store_parameters（0x140911710）：读 yaw@+0x154/pitch@+0x150/pos@+0x138/fov@+0x190 → 0x24B 记录；
  NaN→0（pos 默认 (0,5,0)@0x142aab4c0）+ 四条 "…invalid. Overriding param cache value." 日志
load_parameters（0x140911520）：mode=0；轨道四元数（π/180@0x142aab4b0）；+0x118=0；
  push 28B ActiveRotation 入 +0x100 Vec；flag 时 +0x194=记录分量
immediately_finish_interpolating（0x140911b50）：模式@+0x4d0+快照字节@+0x134 双 jumptable；
  fov 系数 +0xec×0.1；多 body 矩阵 0x100 拷贝；winter==4 分支
```

### dolly 接线

CameraRig {drivers Vec cap-first, final_transform@+0x18}；分叉 CameraTransform **rotation-first**（quat@+0x18 16B, pos@+0x28 12B）。driver 查找 vtable+0x38(type_id)/+0x40(driver_mut)；typeid Position=0x6b8463577afc1a66、YawPitch=0x718e4fd258330fe5。panic：`No <…> driver found in the CameraRig`（dolly-0.6.0 rig.rs）。
"""
SECTIONS["systems/camera_rig/camera_body.rs.md"] = r"""
## 实现方式（算法级）

- `CameraBody` trait：rotate_yaw_pitch / update（每 body 实现）。
- `CameraTransform`：**游戏 dolly 分叉 rotation-first**——rotation Quat@+0x18 (16B)、position Vec3@+0x28 (12B)（round2 CLOSED；stock dolly position-first）。
- `CameraMode` 核心 6 变体；模式字节另有 6..12 拍照/工具槽。
- `pitch_yaw_rot`：由 pitch/yaw（度）构造旋转四元数（YXZ 欧拉 × π/180，0.0174533@0x142aab4b0）。
- lerp / smooth(1−exp(−8dt)) / bounds_xz 共享数学（机器码实证）。
"""
SECTIONS["systems/camera_rig/free.rs.md"] = r"""
## 实现方式（算法级）

```
rotate_yaw_pitch（0x140993750，free/walk 共享）：
  查 YawPitch driver（typeid 0x718e4fd258330fe5，vtable+0x40）→ 未找到 panic "No <…YawPitch…> driver found in the CameraRig"
  yaw   = fmodf(yaw   + delta_yaw·0.5, 720.0)     // 0.5@0x142925870、720@0x142aba9ac
  pitch = clamp(pitch + delta_pitch·0.5, −90, 90) // ±90@0x142aba9b0/0x1429edb94
translate_screen_space（0x1409938b0）：
  quat 旋转移动 → clamp xz ±91（0x142aba9b4/8）→ terrain.max(0)+0.75 贴地
  smooth(dt)=1−exp(−8dt) → delta×20/60 → Position.translate
get_position/get_yaw_pitch：typeid 查 driver sret。
set（0x140993100）：重建 rig Position+YawPitch+Smooth(1,1)（Smooth 0x40B 初始化）→ dt=0 跑一遍。
update（0x140993d30，free/walk 共享）：rig 迭代 vtable+0x30 写回 final_transform@+0x18..0x34，sret 28B。
constrain_to_be_above_terrain → 折叠 0x14003bb80（ret，Free 不约束）。
```
"""
SECTIONS["systems/camera_rig/normal.rs.md"] = r"""
## 实现方式（算法级）

```
update（0x140949fe0）：smooth 推进、radial_max@+0x60 clamp、loose@+0x6c 地面投影
  （lookahead=forward·18、anchor 边界 46.0、radial_min 投影；TiltShift/TopDown 半正交：
    distance=((pos.y−10)/max(−fwd.y,0.15)).max(11.0125)、anchor 边界 33.45）
  ActiveRotation 弧长插值（28B：pitch/yaw_delta）；pending 应用；pos.y≥1.0；retain(|pitch_delta|<0.001 && |yaw_delta|<0.001)
rotate_yaw_pitch（0x140949cc0）：base=Σrotations.pitch_delta+pitch@+0x50；
  bounded=clamp(−(pitch+base), pitch_min@+0x58, pitch_max@+0x5c)；active.pitch_delta+=−bounded−base；yaw_delta+=yaw
constrain（0x140949dc0）：pending.y=max(pending.y, terrain(pos.xz+pending.xz)+clearance@+0x68−pos.y)
translate_screen_space（0x140949e70）：ratio=max(pos.y/−fwd.y,1)/50（0x1429cd070）；
  pending += 绕 Y(yaw) 旋转 movement×ratio
rotation：pitch/yaw（度）→ 四元数（YXZ × π/180，pitch_yaw_rot）
```
"""
SECTIONS["systems/camera_rig/walk.rs.md"] = r"""
## 实现方式（算法级）

```
translate_screen_space（0x140994700）：
  quat@+0x18..0x24 旋转 movement → flat（allow_vertical 时保留 y）→ step=8/60 归一
  → 8 次 spherecast（半径 0.5、filter 0x30cf、法向滑移 t·0.9；1e-10 阈值）→ clamp xz ±34
  → smooth → Position.translate
constrain_to_be_above_terrain（0x140994320）：
  两段 spherecast（filter 0x20 先、0x30cf/0 后，半径 0.45@0x142a7fca4，XZ clamp ±64.9）
  → 地面下界 y−1.9−1.4 → terrain walk_height_blend（skip_blend 分支 +0x298 选 vtable 对）
  → y=lerp(y,target,1−exp(−last_dt·20))；ground_hit kind 0xb/0x11
teleport（0x140994130）：Position driver 写 (x,y,z)；方向向量 → yaw/pitch 度（FUN_140334d80/FUN_141766a90 × rad2deg）
rotation（0x140994310）：16B 拷贝 [self+0x18] = 完整四元数（分叉 rotation-first）
rotate_yaw_pitch / update = free 共享（0x140993750/0x140993d30）
```
"""
SECTIONS["systems/camera_rig/butterfly.rs.md"] = r"""
## 实现方式（算法级）

`update`（0x140a0d090）：`selected(+0x3c) != applied(+0x38)` → applied=selected、dt=1000（0x14295b630 瞬时吸附）→ rig 迭代 vtable+0x30 → sret 28B。rig 装配 = Position+Rotation+Smooth(2,predictive)+Arm+Smooth(4)+LookAt(4)（I，vtable 遍历 O）。rotate/constrain/translate 三操作折叠 0x14003bb80（`ret`，蝴蝶不约束）。
"""
SECTIONS["systems/main_camera.rs.md"] = r"""
## 实现方式（算法级）

```
camera_snapshot（0x140911e40）：Mut tick 回写；mode=snapshot.mode u32@+0x18；
  jumptable(0x142aabc88)[mode](snapshot.f32@+0x1c, dt·camera.fov_coeff@+0xec, mode)
  // 每模式 body 快照应用函数；dt×+0xec = 插值步长
main_camera_update（0x140912c20）：
  aspect=w/h → camera+0x4d8
  new_session.flag(+0x18)：窗口>2px 且 cursor==(0,0) → return（等鼠标首动）；否则清 flag
  窗口<3px → 推 WriteSettingsFileCmd（Vec grow + 8B 事件 + counter@+0x40）
  camera+0x344 = 鼠标深度字节
  debug_gui 模式滑块（6→0）；free_camera 开关(+0x168)≠0 → 自由相机 jumptable(0x142aabd30)
  否则 jumptable(0x142aabd04)[action_states.mode(+0x10)]
paused_main_camera_update（0x1409145a0）：
  aspect=w/h；mode→6 body（+0x100/1b8/270/2f0/390/448）→ body 更新 fn
  → CameraProjectionParams::calc_matrices（FUN_140c8ff70）+FUN_140c97390 → memcpy 0x100 → MainCamera@+0x0
reset_camera（0x140912a40）：DebugGui 层>4 日志 "Resetting the camera." → MainCamera::new(旧 aspect@+0x4d8)
  → memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24B 元素 free，cap 留 4）→ 推 WriteSettingsFileCmd
previous（0x140914ac0）：表 [6,0,1,2,3,4,0,0]（CameraRotationPivot 7 变体前驱循环，previous(0)=6）
CameraRotationPivot ron 变体名：CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain（0x14292bb7c）
```
"""
SECTIONS["systems/look_at_camera.rs.md"] = r"""
## 实现方式（算法级）

`look_at_camera`（0x140873de0）：puffin scope；Query (LookAtCamera, Transform) 逐实体：FUN_140c96d40 提取目标变换 → FUN_140c9ce00 写回实体 Transform（位置+旋转四元数），Mut tick 回写。
`register_required_components`（0x1408756e0）= **Bevy derive required-components 注册胶水**（一次 build 期，非运行时系统；bevy_ecs-0.16.0 component.rs panic 串 + 0xd0B 记录拷贝）。
`clone_behavior`（0x140002370）= `xor eax,eax; ret`（返回 0：组件不克隆）。
"""
SECTIONS["systems/time_of_day.rs.md"] = r"""
## 实现方式（算法级）

```
time_of_day（0x140a73460，13 参数系统）：
  angle 插值：向 2π−angle（6.28319@0x142934674），wrap ±π，1−exp(−3·dt)（−3@0x142ad58e8），收敛<0.01 清标志
  hour 插值：wrap ±0.5（0.5@0x142925870），exp(−3·dt·max(speed,1))，收敛<0.001 清标志；瞬时吸附直接对齐 fmod 后小时
  播放状态机：
    dev 关：0→progress=0→2；2→progress+=dt/−5（下限 0）倒带
    dev 开：0→progress=0→1；1→progress+=dt/3（上限 1）前进
  小时推进：target_hour=fmodf(progress·24+8+dt·TIME_OF_DAY_SPEED(全局 0x14362c0b8)·speed·trig, 24)
    hour=fmodf(target·24+8, 24)（24@0x142934600、8@0x142934604）→ remap_hour_to_fmod_hour(hour,winter)
  日期/太阳：day_of_year∈1..365（越界 unwrap panic）
    encoded=day_of_year|0xFCE00（2023<<9）；unix_ms=decode·1000+hour·3600000（3600000@0x142ad58f0）
    suncalc::get_position(ms, lat=state@+0x10, lon=0.0) → colatitude=π/2−(azimuth+π) + 正交基常量（@142ad5910..5984）
    → 3D 太阳方向 → 写光照资源（TodLightingSnapshot 插值、太阳可见两套 SunSkyColors @0x140b18830）
  音频上报：set_parameter "TOD-Enabled"(0xb)/"Timetravel"(10)/"DayNight"(8)
remap_hour_to_fmod_hour（0x140a74790）：
  x={0,3.5,9,17,20,24}；y={0,3,11,17,22,24}（合同写 {48,17,11,24,22,…} 是错的）
  冬季：x 插 4.52；y 插 5.0/6.2/9.0（memmove 右移 0x142923c10）→ 线性插值查找（func_0x140c918b0）
```
资源面：当前态 {hour,angle,角度过渡@+3,小时过渡@+0xa,瞬时@+0xb}；播放态 {state,progress,target_hour,角度@+0xc,纬度@+0x10,日出@+0x14,日落@+0x18,太阳可见@+0x20/24,day_of_year@+0x28}；dev 态 {**小时倍率@+0x23**（合同写 @+0x118 是错的）, 热键@+0xe3}。
"""
SECTIONS["systems/time_update.rs.md"] = r"""
## 实现方式（算法级）

`time_update`（0x140922c30）：56B 游戏时钟 {secs@+0x0, nanos@+0x4, flag@+0x6, total **f64**@+0x10（纠错合同 f32）, 60ths@+0x18, frac@+0x20, frame_count@+0x2c, flag@+0x30}。
```
dt = replaying（InputRecordReplayState+0x40==i64::MIN+2）? replay_frame.delta@+0xc8（0xd0B 帧）
   : GameTime delta Duration（secs@+0x10/nanos@+0x18）
前两帧 clamp dt ≤ 1/60（0.016666668@142aad4a8）
```
panic：Duration 加法溢出 "overflow when adding durations"（0x142aad928）；f32→Duration "cannot convert float seconds to Duration…"（std 内联）。
"""
SECTIONS["systems/weather_system.rs.md"] = r"""
## 实现方式（算法级）

`weather_system`（0x1408e5b00）：
```
if glade_type@+0x58 != 4: return                    // 非冬季不写（连 tick 都不回写）
game_secs = GameTime+0x0(secs) + GameTime+0x8(nanos)/1e9（1e9@0x1429a7640）
s = sin(game_secs / 300 · π)（300@0x142aa5140、π@0x1429edba4）   // 10 分钟正弦
delta = GameTime+0x28(dt f32) · 0.1（0.1@0x142925984）
intensity(+0xc) = clamp(intensity + (s>0 ? +delta : −delta), 0, 1.0)（1.0@0x142925904）
```
"""
SECTIONS["systems/timelapse_replay.rs.md"] = r"""
## 实现方式（算法级）

`TimelapseState` 字节 0=idle/1=recording/2=finished；`is_active` 0x14092e6b0 = `state != 2`（骨架写 ==1 错）。
```
start_timelapse（0x140a8a310）：消费 StartTimelapseReplayCmd → 修剪输入流 → 有未消费帧 → mode_manager::check_if_can_switch<closure>
finish_timelapse（0x140a8a460）：state!=2 时：未消费帧且 state 偶数 → 推 MetaCommand（i64::MIN 哨兵 + 16B@+0x60..0x6c，FUN_140b59900）→ state=1
  命令队列空（+0x18==0）→ 推 {tag:i64::MIN, byte: state} 事件（stride 0x10）并清另一计数器
cleanup_timelapse（0x140a8a5d0）：有未消费帧 → 日志（level 3）"The timelapse has finished" → state=2
```
"""
SECTIONS["systems/ui_camera_mode_gizmos.rs.md"] = r"""
## 实现方式（算法级）

`ui_camera_mode_gizmos`（0x1409527a0）：四重门槛 ① AppMode∈{2,10}（10 需 flag=0）② (mode&6)==2 返回 ③ 窗口宽或高 ≤ 2 ④ 0x13>>mode&1（签名符模式∈{0,1,4}；模式 6 先映射 0）。通过后 UiState::add("camera_mode_activerotation_")（状态 int≠20 时）→ 0x20B UiAppendSignifier{1,1,id_string,size=窗口字段,fov=snapshot+0xe8} → AppendOnlyVec push。
闭包 0x140952b60 = 渲染回调：cursor_grab 部件 + vertex_color_shader；动画 value/1000·20，clamp 25.0@0x1429edb98。消费 CameraRotationPivot::use_camera_cursor_grab_icon 谓词。
"""
SECTIONS["systems/arrow_gizmo_animation.rs.md"] = r"""
## 实现方式（算法级）

`arrow_gizmo_animation`（0x140a92f80）：每实体（EnableArrowGizmoStretch）双 f32 拉伸值；有输入事件（0x10B 步长，8B 向量 payload）按主轴（|x|<|y| 取 y 轴）`stretch=min(stretch+|Δ|·0.02, 1.0)`（0.02@0x1429cd2d0）；无事件每帧 `stretch=max(stretch−0.1, 0)`（−0.1@0x1429eb6f8）；Mut tick 回写；puffin scope country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation。
"""
SECTIONS["systems/clear_canvas.rs.md"] = r"""
## 实现方式（算法级）

`clear_canvas`（0x140a7abd0）：事件驱动（EventReader 修剪流）；日志 "Clearing the canvas." → InnerWalls::remove_all（0x14085a730，flag 时 +0xc0=0）→ 世界光栅 clear_all<512>（0x14093bee0）变体 0x13/0x14/6/5/7/1/2/3/4（+9/0xb/0xd/0xe）→ 7×clear_preset_texture（0x140a15da0，3 个 color_id 0x5000=20KB 缓冲，panic 串 color_id.rs）→ 上传记录/杂项清零（0x140aaae80×2、0x140aa5150、0x1408950d0、0x14088acf0）。
"""
SECTIONS["systems/despawn.rs.md"] = r"""
## 实现方式（算法级）

`despawn_glade_entities`（0x1408b3510）：Query 迭代，world 字节 @+0x108 二分：A 支（stride 0xa0、count@+0x58、数组@+0x50、0x10B 元素）/ B 支（stride 0x48、count@+0x10、数组@+0x8、8B 元素）；逐槽逐实体 Commands::despawn（0x140d14770，file despawn.rs）；u64 实体 id 高 32 位==0 提前 return。
"""
SECTIONS["resources/time.rs.md"] = r"""
## 实现方式（算法级）

`GameTime` 实测布局：{game_time 累计 Duration（secs@+0x0/nanos@+0x8）、delta Duration（secs@+0x10/nanos@+0x18）、delta f32@+0x28}；消费方：weather（game_time 正弦）、time_update（delta）、time_of_day（dt@+0x28）。+0x20..0x27 与尾部 U。`GameTime::default`（0x140b64d00）零化构造；`Time` = bevy_time 0.16 离线门面。
"""
SECTIONS["resources/time_of_day.rs.md"] = r"""
## 实现方式（算法级）

`TodSequence` entries 0x50B/条：{start_hour f32, end_hour f32, snapshot 0x44B@+0xc}；`TodLightingSnapshot` 0x44B = {sun_sky@+0x0 8×f32, fog_tint@+0x20 2×u64（打包雾色，**不插值**——round2 CLOSED）, base_fog_thickness@+0x30 4×f32, speed_bias@+0x40}。
```
snapshot_at_tod（0x140b067d0）：len==1 拷贝；len==0 panic；t=tod.rem_euclid(1)（fmodf+负值 wrap）
  → 定段 t=clamp01((t−a.start)/(b.start−a.start)) → TodLightingSnapshot::lerp（0x140a97650；f32 组插值 + u64 对拷贝）
```
会话序列化：TodSequence{entries}、TimeOfDay{time_of_day,snapshot,temp_ui_ids}、目标态 {time_of_day, sun_xz_rotation, latitude_degrees}；TodEasing 默认 fastrand 选段。
"""
SECTIONS["resources/daily.rs.md"] = r"""
## 实现方式（算法级）

`DailyThemeInfo`：default 置两个 u64=0x8000000000000000 哨兵；`is_weekly`（0x14083c440）= `SystemTime::duration_since` 成功且 `elapsed > 0x6977f`（432511s ≈ **5.006 天**，非 7 天——合同/骨架 7*86400 不符）；`get_name`（0x14083c240）= kind==5 判别 + [rdx+4] 首字符匹配 locale 族（en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP，r8==5 门）。CurrentDaily/DailyThemeCredits 占位。
"""
SECTIONS["resources/glade_settings.rs.md"] = r"""
## 实现方式（算法级）

`GladeSettings::load`（0x1408bbbc0）：glade_settings.json serde_json Option → 失败回退 Default{+0 u32=0、+8 f32=0.01、+0xc f32=1.0、+0x10 u32=8、+0x18 f32=0.0、+0x1c f32=1.0、+0x20 u8=1}（Default 0x8c2240）。字段语义 U。
"""
SECTIONS["resources/weather.rs.md"] = r"""
## 实现方式（算法级）

`Weather` 空资源（O 名、impl$0::default 内联；audio weather 消费；布局 U）。雪强逻辑在 systems/weather_system.rs（SnowIntensity@+0xc，10 分钟正弦 ±0.1·dt）。
"""
SECTIONS["resources/photo_mode.rs.md"] = r"""
## 实现方式（算法级）

`PhotoModeCameraMode` u8 {0..3}：`next` 0x140b59720 表 {0→1,1→2,2→3,3→0}；`next_demo` 0x140b596b0 表 {0→1,1→2,2→0}，**变体 3 → panic "this camera mode is not part of the demo!"**（串 @0x142af1188/file @0x142af11c8）。`impl$1::from` 0x14090e500 = `lea eax,[rcx+1]; ret`（+1 恒等映射）。PhotomodeState::on_enable_photomode f32@0=3.0/1.0(demo)+u32@4=-1。与 CameraMode 槽 6..12 联动（round2 PARTIAL，写入点 toolbar::photomode）。
"""

def main():
    base = os.path.normpath(ROOT)
    if not os.path.isdir(base):
        print("missing chapters dir:", base); sys.exit(1)
    written, skipped, missing = [], [], []
    for rel, sect in SECTIONS.items():
        path = os.path.join(base, rel.replace("/", os.sep))
        if not os.path.isfile(path):
            missing.append(rel); continue
        text = io.open(path, "r", encoding="utf-8").read()
        if "## 实现方式（算法级）" in text:
            skipped.append(rel); continue
        block = "\n" + sect.lstrip("\n").rstrip("\n") + "\n"
        io.open(path, "a", encoding="utf-8").write(block)
        written.append(rel)
    print("written:", len(written))
    for r in written: print("  +", r)
    print("skipped:", len(skipped));
    for r in skipped: print("  =", r)
    print("missing:", len(missing))
    for r in missing: print("  !", r)

if __name__ == "__main__":
    main()
