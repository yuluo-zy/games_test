# terrain-water 域算法级文档（2026-10-15）

> 对象：country-core 地形/水域/栅格域。本文档把 DIGEST（布局/行为表）+ round2 CLOSURE + 各章节升级为
> **算法级**：处理链、逐函数算法、数据结构、常量表、状态机。证据：`decomp/decomp-*.txt`（本域 136 份）、
> round2/terrain-water-CLOSURE.md、rne-read 常量/串。标注 **O**=机器码直接支持；**I**=调用链组合；**U**=未恢复。

---

## 1. WorldRaster 家族处理链（核心）

### 1.0 数据结构（O，本轮修订一处）

`WorldRaster<512>` 本体 ≈ 0x2a0B：

| 偏移 | 字段 | 备注 |
|---|---|---|
| +0x00/+0x08/+0x10 | composite Vec {cap=0x40000, ptr, len=0x40000} | 512×512 u8 |
| +0x18/+0x20/+0x28 | scratch Vec | 同上 |
| +0x30..+0x47 | 瓦片状态 Vec {cap=0x1000, ptr, len=0x1000} | **4096 个 u32 瓦片状态字**，初始 0x07954dbb（tabloid 协议，语义 I） |
| +0x48..+0x5f | 次级 Vec {cap=0x40, ptr, len=0x40} + 4B | 含 4×u32@+0x50..0x5c（retain 帧元数据，U 语义） |
| +0x60..+0x97 | DirtyTiles 头（56B 原型拷贝；parking_lot Mutex@+0x60/+0x68、订阅者 Vec@+0x78/80/88） | tabloid |
| **+0x98..+0x297** | **4096-bit（512B = 64×u64）脏瓦片位图** | **DIGEST round1 误标"512B staging 缓冲"——本轮修正：write 的 memset 0xff = 全瓦片置脏** |
| +0x298 | 读回选择字节（0→composite，1→scratch） | copy_to_staging 写 |
| +0x299 | staging 脏标志 | write 置 1；copy_to_staging 清零 |
| +0x29a | 已发布字节 | copy_to_staging 尾置 1 |
| +0x29b | 再上传脏字节（mask 重传协议） | **置位方=upload_raster_staging_to_texture（本轮闭合 round2 的 U，见 §1.5）** |

### 1.1 AllWorldRasterWrite::write（0x14083ebe0，槽 0..5 全流程）

```
write(&mut self, up_to: usize):                 // param_3 = 1..5 = 只写前 up_to 槽；0 或 ≥6 → 尾部 panic 路径
  for slot in 0..6（编译器 unroll，逐槽退出条件 param_3 == slot+1）:
    tick 回写 Mut（param_1[2+i*4] = param_1+0x1c）
    if 槽 Option 为 None（SBORROW8 判别）:
        memset(composite.data, 0, composite.len)    // 资源缺失 → 该光栅清空
    else:
        if slot.len != composite.len:
            panic "Length mismatch; expected "（行 43，world_raster_defs.rs；包装 unwrap Err）
        memcpy(slot.data → composite.data, slot.len)
    memset(composite + 0x98, 0xff, 0x200)       // 4096-bit 位图全置 1（全瓦片脏）
    composite + 0x299 = 1                        // staging 脏标志
  // 尾部 = panic/abort 胶水（所有早退 goto 共享）：lea rcx,[world_raster_defs.rs fat]; call 0x1428d9310; int3
```

要点：**写 = 整缓冲替换 + 全脏**。调用方传入"写到第几个槽"，上游（save 加载、replay、draw_raster 发布）逐槽整体回填 composite。

### 1.2 upload_to_staging（0x14083fa60）

```
upload_to_staging(&mut self, ssbo_lib):
  for 槽 0..5:
    ssbo = AssetSsboLibrary 按 type-id 查 *RasterStagingSsbo        // 6 个 type-id 常量 @0x142c2e210 区
    memcmp 全名 "country_core::resources::world_raster_defs::{Path|Cobblestone1|...}RasterStagingSsbo"
    raster = vtable 取映射指针
    copy_to_staging(raster, selector=0, mapped, len)               // 发布 composite（不选 scratch）
```

配对 `would_write_succeed`（0x14083f410）：按槽比对类型名（"PathRaster" 0xa 字符等）做写冲突检测；缺失资源 → Bevy panic "Resource does not exist…/Resource requested by …/does not exist: "。

### 1.3 copy_to_staging（0x140a172d0，CPU → SSBO 映射协议）

```
copy_to_staging(worldraster, selector, out_mapped, out_len):
  if +0x299 != 0:                                  // staging 脏
      memset(out, 0, out_len); +0x299 = 0          // 先清 out：未脏瓦片读到 0
  +0x298 = selector
  src = selector==0 ? composite{ptr@+8, len@+0x10} : scratch{ptr@+0x20, len@+0x28}
  DirtyTiles::retain(bitmap@+0x98, frame{src_ptr, src_len, 4×u32@+0x50..0x5c, worldraster, out, out_len})
      // 按位图把脏 8×8 瓦片块从选中缓冲拷入 out（SSBO 映射区）
  tabloid publish：Mutex@+0x60/+0x68 慢路径、发布记录 flag@+0x10 LOCK/CAS、通知订阅者
  +0x29a = 1                                       // 已发布（W1 消费）
  // log/panic location：module path country_core::resources::world_raster
```

### 1.4 upload_raster_staging_to_texture<T>（W1，6 单态：0x1403d8bd0/3d90f0/3d9610/3d9b30/3da050/3da570）

```
fn upload_raster_staging_to_texture(world, ssbo_lib, fmt):
  ssbo = lib 按 type-id hash（如 Path 0x85c987d23d58d2de）查 *RasterStagingSsbo
  if ssbo && ssbo.published(+0x29a) != 0:
      tex = make_texture(raster, fmt)
      fmt==0x4c("r16f") → 4 通道（UNK_1429d361f）；fmt==0x61("rgba16fr16f") → 7 通道（UNK_1429d3618）
      其它格式 → panic "not yet implemented"（fmt panic 0x1403d8ef5）
      ssbo.synced_version(+0x5c) = lib.version(+0x5b0)
      *(u16*)(ssbo.raster + 0x29a) = 0x100          // +0x29a=0（清已发布）、+0x29b=1（mask 重传置位）
      compute pass："Upload cpu raster" /upload_cpu_raster.cs.hlsl（bind → set texture → dispatch_2d）
  else: 不发布则跳过（无脏不上传）
```

**+0x29b 置位方（闭合 round2 U 项）**：W1 发布后经 `*(undefined2 *)(+0x29a) = 0x100` 写入 → +0x29b=1。
直接寻址扫描曾断言"不存在置 1 指令"，实为 **2 字节合并写 0x100** 的宽写入（round2 排除性扫描覆盖了 0x290..0x2a0 的 dword/qword/向量，但 0x1403d8bd0 的 `66 c7 86 9a 02 00 00 00 01` word 写恰好落在此类中被当作"false positive 只到 +0x299"的那条——**本轮把它与消费端配对确认**）。

### 1.5 rasterize_from_cpu 三处（+0x29b 消费链）

统一模板（O）：`+0x29b != 0 → ssbo.version(+0x5c) ← lib.version(+0x5b0)；清 +0x29b；memset(bitmap@+0x98, 0, 0x200)；建纹理；bind shader；dispatch_2d`。

| 系统 | 地址 | SSBO type-id | 格式 | 备注 |
|---|---|---|---|---|
| water_rasterize_from_cpu（W2） | 0x140a12680 | WaterRasterStagingSsbo 0x1ad182936286a0ee | 0x61 | puffin "water blur temp"/"Blur water mask"/"/gauss_blur_mask.cs.hlsl" |
| garden_rasterize_from_cpu（W12） | 0x140a22140 | GardenRasterStagingSsbo | 0x61 | 同构 |
| compute_path_masks | 0x1409014e0 | Path/Cobble1/2/3 四 SSBO | 0x4c（r16f） | **一次同步 4 个光栅**（4×+0x29b 判 4×同步 4×memset） |

### 1.6 record_raster_history（0x957990）/ upload_world_raster_buffers_to_staging（0x957bf0）

- record：按 variant·0x470 槽位取 12B 笔画 Vec → Arc 克隆 → `History::add` → 重置 Vec(cap 4)。
- upload 系统：脏输入流（EventReader 双缓冲 {counter, start@+0x18, end@+0x40…} 差）非空 → 重置 counter → `upload_to_staging`（6 光栅 composite→staging SSBO）；log/panic module path `country_core::systems::world_raster`。

---

## 2. ActiveRasterStroke / InProgressRasterStroke 状态机

### 2.0 记录布局（0x470B，O）

| 偏移 | 字段 |
|---|---|
| +0x00 | u64 活动/流体标志（add_point 内 `*p1==1` 门控 solidify 分支；start_new 置 0） |
| +0x08 | 流体游标指针（add_point 读为指针判非空；solidify 后置 0 或剩余点数） |
| +0x10/+0x18/+0x20 | Vec1：**已固化曲线**（12B Vec3） |
| +0x28/+0x30/+0x38 | Vec2：**进行中原始输入点**（12B Vec3） |
| +0x40 | 累计弧长 f32（>1.0 触发 solidify） |
| +0x48/+0x50/+0x58 | Vec3：**重采样+平滑后曲线**（draw_raster 的 rasterize_stroke 消费此列） |
| +0x60 | 笔刷宽度 f32 |
| +0x64 | 模式字节（0/非0 → 擦除/添加；rasterize_stroke 用它+bit0 选 blend 闭包；draw_raster 按它发 Draw/Erase 事件） |
| +0x68 | u32 blend/权重标志（bit0） |
| +0x6c | u32 计数器 |
| +0x70..+0x468 | 双份 512B 脏位图（64×u64 ×2；draw_raster 交换两半） |

### 2.1 处理链（O）

```
start_new(brush, mode, flag, cnt):                // 0x140868d00
  清三 Vec（cap=0、ptr=dangling）→ 写 +0x60/+0x64/+0x68/+0x6c → add_point(首点) → 推入平滑列

add_point(pt):                                    // 0x140868fb0
  if Vec2 空: push(pt)
  else: arc(+0x40) += dist(last, pt); push(pt)     // cap 满先 grow
  if arc > 1.0:                                    // 阈值 1.0 世界单位
      solidify_fluid_into(Vec2.data, Vec2.len, &mut Vec1)   // 固化前缀
      memmove 剩余点前移；游标/标志更新

resample_and_smooth():                            // 0x140868e40
  1. Vec1 整列拷贝 → Vec3（12B 元素 SIMD 拷贝）
  2. solidify_fluid_into(Vec2.data, Vec2.len, &mut Vec3)    // 剩余流体点 0.5 间距固化并入
  3. 对 Vec3 全列做 10 遍 3-tap 平滑：
        new[i] = ((p[i-1] + p[i])·0.5 − p[i+1])·0.5 + p[i+1]
                = 0.25·p[i-1] + 0.5·p[i] + 0.25·p[i+1]
     （常量 0.5×4 @0x14293ac60..6c；外层 while lVar15 != 10）

solidify_fluid_into(points, len, &mut dst_vec):   // 0x1408691b0
  view = slice_view(points, len)                   // FUN_141308760
  if len < 2: 单点拷贝 append
  else:
      resampled = FUN_141467fa0(view, 0.5)         // 0.5 世界单位定距重采样（迭代器适配 FUN_141469230）
      // 溢出 panic 分支（行 43，active_raster_stroke.rs）
      if dst 空: grow+memcpy 全段（cap 检查 FUN_1428a9a60）
      else: FUN_1412f9e10 尾点合并追加
  free 中间缓冲（0x140613c20）
```

`InProgressRasterStroke::last`（0x140868f70）：Vec2.len==0 → **panic（Option::unwrap None）**；否则返回 &points[len-1]。

### 2.2 draw_raster<T> 状态机（10 实例：Path×3/Cobble1-3×2/Garden/Water；水实例 0x142260ae0 精读）

```
draw_raster<T, WorldRaster, F>(cmd, point, ...):           // cmd@param_1
  stroke = active_stroke_holder.index_mut(slot_index, &UNK_143033e10)   // 0x470 槽位
  state = cmd.state
  if state == 0:                                    // start_new
      start_new(stroke, cursor_pos, mode=cmd[3], brush=cmd[2])
  elif state == 1:                                  // 进行中
      last = InProgressRasterStroke::last(stroke)   // 空 → panic
      if dist(last, point) <= 0.0005 (0x1429cd060): return     // 距离阈值
      add_point(stroke, point)
      resample_and_smooth(stroke)
      DirtyTiles::clear(stroke.bitmap@+0x270)                 // 清活动脏位图
      rasterize_world_space_stroke(stroke.bitmap@+0x270,       // 水实例 FUN_1425d9e90
                                    iter_points_xz(smoothed@+0x48), first, brush@+0x60)
      DirtyTiles::extend(worldraster.bitmap@+0x98, stroke.bitmap)   // 并入世界光栅（×2 扩展）
      swap(stroke.bitmap[0x70..0x270], stroke.bitmap[0x270..0x470])  // 32×qword 交换双半
      copy_composite_to_scratch(worldraster)       // 水实例 0x142262680
      rasterize_stroke(worldraster, iter_points_xz(smoothed), ..., mode, brush)  // 水实例 0x142261d10
      ssbo = map StagingSsbo; copy_to_staging(selector=0)
  else:                                             // 其余状态
      commit_scratch_to_composite(worldraster)      // 发布 scratch
  if stroke.mode(+0x64) == 0: 发 Draw 事件；else 发 Erase 事件（WaterDraw/WaterErase…）
```

**draw_raster 输入源**：点从 `CursorTerrainRaycast::position`（func_0x000140a12650）来；平滑后曲线列 Vec3@+0x48 是 rasterize_stroke 的唯一输入（`iter_points_xz` = 0x140a97860 映射为 (x,z) 对）。

### 2.3 rasterize_stroke（0x140a16960，CPU 栅格化核心）

```
rasterize_stroke(worldraster, points_iter, ...):
  点数 < 3 → 早退
  点列 → zeno Command 表（28B/条；首点 variant 0=move-to，其余 1=line-to）
      grid = (world / 130 + 0.5) * 512            // 130@0x142a7b658、0.5@0x1429258e0、512@0x1429a3c80/84
  blend 闭包按 (arg6, bit0)：
      (0, 0)     → env$1  min / 擦除
      (≠0, 0)    → env$0  max
      (≠0, 1)    → env$2  饱和加 × 笔刷（16.16 定点覆盖度）
      (0, 1)     → panic
  目标层按模式字节选 composite（0）或 scratch（非0）
  u8→f32 /255.0（0x14293ac70）；行距 SHL 9
```

---

## 3. 地形（terrain）

### 3.0 TerrainHeightsData<T> 双实例（O）

- 布局：+0x08 = **u16 512×512 网格数据指针**（512KB，行距 512 u16）；+0x50/+0x54 = min/max f32（默认 **-2.5/24.0** @0x142a9f3f0 打包对）；其余 = 脏位图 64×u64 + 订阅。
- 实例：`TerrainHeightSsbo`（权威）、`VisualTerrainHeightSsbo`（视觉）。
- **+0x201 索引偏移（本轮修正 DIGEST）**：权威实例正常分支（`*0x14362c308 == 0`）读 `grid[(z<<9|x) + 0x201]`，另一分支无偏移——**不是**"debug 路径用 +0x201"。语义（halo/边际行缓冲）U。

### 3.1 sample_at_world_xz（权威 0x1409e9640）

```
u = (world.xz / 130) + 0.5                        // divps 130 + 0.5
if *0x14362c308 == 0（正常路径）:
    u = clamp01(u)
    idx = fptoui_sat(u * 512)                     // 饱和浮点→int（LLVM fptoui.sat 位编码）
    return grid[(idx.z << 9 | idx.x) + 0x201]     // u16 原始值
else（调试网格）:
    idx = clamp01(fract 索引 · 512 − 0.5 + 1 域) → 0..0x1ff
    return grid[idx.z << 9 | idx.x]
```

### 3.2 sample_at_world_xz 视觉双线性（0x141246fb0）

```
load_index_unchecked(grid, idx) ×4                // (x,z)、(x+1,z)、(x,z+1)、(x+1,z+1)
    // 0x1408b54c0：u16→f32 解码 = min + u16/65535·(max−min)（含 +0x50/54 min/max，O 调用面）
3× lerp（func_0x000140c904d0）                       // 双线性插值
```

### 3.3 intersect_ray（权威 0x1409e9a20）

```
intersect_ray(&mut self, ray_origin, ray_dir, t_max, out):
  1. 高度场 AABB（xz ±65、y=[min@+0x50, max@+0x54]）slab 求进出 t（FUN_141240860）
     // AABB 常量：65.0（0x42820000）、−0.0625；命中 0/1 判别
     if 无 hit → miss（out.tag = 0）
  2. t_near = max(t_near, 0)；t_far = min(t_max, t_far)
  3. 32 步行进：step = (t_far − t_near) · (1/32)（0x142ac4cc0）
      逐点 t 采样 sample_at_world_xz；ray.y(t) < height(t) → 命中 → 进入二分
  4. 每命中步 3 次 0.5 二分（lo/hi 折半采样，0.5@0x142925870）+ 尾次合并 → t_hit
  5. 与 y=min 平面交点（FUN_1412803f0）比距取近
  6. out = {tag, y, x, z, t}                        // tag 1=地形命中；0=miss；平面命中 iVar3 分支
```

### 3.4 copy_from_gpu（0x1409ea020，GPU→CPU 差分回拷）

```
copy_from_gpu(&mut self, gpu_mapped_u16):
  memset(局部脏位图 auStack_270[64 u64], 0)
  if min != −2.5 || max != 24.0: 复位 {min,max} = {−2.5, 24.0}（0x142a9f3f0）
  for 每个 8×8 瓦片（64 行 × 64 列，行=8 个 512px 行 unroll）:
      for 瓦片内 16B 块（= 8×u16 一列）:
          if gpu.block != cpu.block:
              copy 16B（4×u32）gpu → cpu
              dirty[行] |= 1 << 列                 // 每瓦片 1 bit（8×8px）
  // 差分粒度：64×64 瓦片 = 4096 bit；逐 16B 子块比对
```

### 3.5 拾取链（O）

- `mouse_terrain_raycast`（0x1408ef320）：CameraCursorRay 有效 → xz 平面解（t=(y−pos.y)/dir.y）；否则 `intersect_ray` 高度场求交，与 **y=0 平面交点比距取近**；closure$0 实例网格精化。写 pos/dir-delta/length。
- `terrain_hover_idx`（0x140aff720）：AppMode 位掩码 **0x421** 门 → MouseTerrainRaycastSsbo（名 0x6c 字符）映射读 `id=*(i32*)(buf+0x10)`（-1 哨兵）；**水位采样≤0 / 光标无效 / 距离≤0.5** 不更新 → 写 HoveredTerrainStrokeId{tag=1, id}。
- `delayed_terrain_changed_msg`（0xaabdd0）：`on_terrain_modified`=LOCK inc counter；delays 每帧递减 → 0：FrozenMap 取 SSBO（尺寸断言 0x3a/0x40）→ cpu_readback → **copy_from_gpu×2（权威+视觉）**；flag 置位 push(2)（延迟 2 帧）发 OnTerrainChange。
- `terrain_rock_mask::sample_at_world_xz`（0x1409ea800）：(x/130+0.5) clamp01 → ×512；(z*512|x)>>5 字索引边界检查后取位。

### 3.6 编辑/UI（O 面）

- `terrain_stroke_changed`（0xa03ba0）：EvTerrainStrokeChanged 非空 → RecalculateHeightMapCmd + PlayTerrainSound。
- `edit_terrain_stroke`（0x94e000）：按命令 id 查 TerrainEditingData（缺失 panic "Was requested to edit a terrain stroke  that doesnt exist"）→ falloff∈[0,1] / height（Abs≥0 / Delta≥0.1）应用 → RecalculateHeightMapCmd + PlayTerrainSound。
- `ui_draw_terrain`（0xa9fdc0）/`ui_edit_terrain`（0xaef1f0）：事件/命令生成面（ui-input 域细查）；点按 → new_stroke + EvTerrainStrokeChanged。
- `history_record_terrain`（0xac9770）：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史 History::add + DebugHistoryMsg。
- `ui_terrain_focus_mode`（0x9a4fb0）：167KB 近全内联 + 4 闭包（0x9a3620 曲线采样 0.5 参数 / 0x9a8640 / 0x9a88c0 / 0x9a8b40 状态转换，OutlineWallCmd/PublicWalls 相关）；内部状态名交 ui-input 域。
- `mesh_follow_terrain`（0xa68430）：Query<(&MeshFollowTerrain, &mut Transform, &Handle<Mesh>)> + 2×Res；取 mesh "Vertex_Position" 属性（0x142ad5120，0xf）→ 内联地形采样（FUN_1412fd550）→ Transform.y 更新；无效句柄 panic "MeshFollowTerrain got an invalid mesh handle"。

---

## 4. 水 / 花园

- `draw_garden`（0xae4620）：工具 id 分发（id-6<0xf 且 id-5∈{4,0xc}；9=橡皮/17=花丛）；Circle2d 半径 = 笔刷 + **1.5**（0x1429258d0）；GladeBorder::is_shape_inside 界内校验 → draw_raster<GardenRasterStagingSsbo>。
- `water_spawn_vegetation`（0xa13970）：两 compute pass——"Spawn lilypads" /water/spawn_lilypads.cs.hlsl（LilypadSsbo+LilypadFlowerSsbo）、"Spawn reeds" /water/spawn_reeds.cs.hlsl（ReedSsbo）+ indirect draw 缓冲；ActiveSession::is_autumn + bind_terrain_heightmap。
- `garden_spawn_vegetation`（0xa22b80）："Garden Marching Squares" /garden/marching_squares.cs.hlsl（方格行进，CPU 掩码→SSBO）+ /garden/spawn_{small_flowers,bushes_garden,flowers_garden}.cs.hlsl。
- 音频系统（water_audio/water_ambience/garden_audio/terrain_audio/terrain_stairs_audio）：消费水位栅格/笔触事件做音频（audio-misc 域细查）；update_water_audio_grid 24B 单元/64 位滚动窗/6400.0² 阈值/0.5 水位阈值 prior O。
- `RockyTerrainMesh::load`（0x8bbf50）：fs::read_to_string → serde_json 解析 → add_new_with_data<RockyTerrainVertexSSBO/TriangleSSBO>（顶点 0x24B→0x30B、三角形 0x10B→0x20B）。

---

## 5. 常量表（terrain-water，O）

| 常量 | 地址 | 用途 |
|---|---|---|
| 130.0 | 0x142a7b658 | 世界→栅格缩放（world/130） |
| 0.5 | 0x1429258e0..8ec（×4） | (world/130+0.5) 平移 / 二分步进 |
| 512 | 0x1429a3c80/84/88/8c | 网格尺寸/×512 |
| 1.0 | 0x14292e6b0 | clamp 上界 |
| 0x07954dbb | 0x142ac7e50..5c（4×u32） | 瓦片状态字初始魔数（denormal≈0.0 空瓦片哨兵，I） |
| 0xff × 0x200 | — | write 全瓦片置脏（4096-bit 位图） |
| 0.0005 | 0x1429cd060 | draw_raster 最近点距离阈值 |
| 0.5（固化间距） | 0x142925870 | solidify/resample 定距 |
| 0.5×4（平滑核） | 0x14293ac60..6c | resample_and_smooth 10 遍 3-tap |
| −2.5 / 24.0 | 0x142a9f3f0（打包对） | TerrainHeightsData min/max 默认 |
| 65.0 | 0x42820000 | intersect_ray AABB 半宽 |
| 1/32 | 0x142ac4cc0 | intersect_ray 步行进步长 |
| 0.85c987d23d58d2de | 0x1403d8bd0 内联 | PathRasterStagingSsbo type-id hash |
| 0x1ad182936286a0ee | 0x140a12680 内联 | WaterRasterStagingSsbo type-id |
| 0x4c / 0x61 | — | r16f / rgba16fr16f 格式串长 |
| 0x3f800000（1.0） | 0x14083eaf0 邻域 | PathRaster::new 默认 |
| path 0.8 / water 2.0 / garden 3.0 | 0x1409f1860/90/c0 | BrushSizes reset 默认 |
| path [0.32,1.2] / water [1,5] / garden [2,4] | 0x142ac56c0,0x1429cd128 / 0x142925904,0x142925988 / 0x14292e6f0,0x142934694 | BrushSizes clamp |
| 1.5 | 0x1429258d0 | draw_garden 半径加量 |
| 0.3/0.4/0.5/1.0/2.0/4.0/0.8 | 0x142aa53c0 邻域 | GladeBorder/圆常量族 |

---

## 6. 状态机汇总

| 状态机 | 转移 | 级别 |
|---|---|---|
| ActiveRasterStroke（弧长门控固化） | 输入点累积 → arc>1.0 solidify → resample 10 遍平滑 | O |
| draw_raster<T> | 0=start_new → 1=增量（距离>0.0005 才 add+平滑+栅格化+双位图交换+commit）→ 其余 commit 发布 | O |
| WorldRaster 脏标志协议 | write 置 +0x299+位图全脏 → copy_to_staging 消费清 +0x299 → W1 发布置 +0x29a=0/+0x29b=1 → W2/W12/path-masks 消费清 +0x29b | O（本轮闭环） |
| intersect_ray | AABB slab → 32 步进 → 3 二分 | O |
| copy_from_gpu | min/max 复位 → 8×8 瓦片 16B 差分 → 4096-bit 脏位图 | O |

---

## 7. 剩余 U（不臆造）

- +0x98 位图之外的 "staging 512B 缓冲" 不存在——**布局已澄清**；剩余 U：+0x30 瓦片状态字（0x07954dbb）精确语义；+0x48..0x5f 次级 Vec 语义；rasterize_stroke 三 blend 闭包 zeno 覆盖度定点细节（prior 已标边界）；WaterMaskSsbo 等回读 SSBO 字段布局；ui_terrain_focus_mode 闭包状态名（ui-input 域）。
