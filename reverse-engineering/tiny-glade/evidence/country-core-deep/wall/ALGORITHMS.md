# wall 域模块算法总览（2026-10-15 算法级深挖）

> 覆盖 `systems/wall/**` + `resources/walls/**` 共 **82 章节 / 431 业务函数**（role-map 312 VA，decomp 425 条）。
> 本文档把章节"一句话结论"升级为**算法级描述**：处理链、循环/分支/常量、数据结构偏移、状态转移、panic/错误处理。
> 分级沿用 O/I/U：O = 机器码/反编译直接观察；I = 推断（注明依据）；U = 未知（不臆造）。
> 证据：`wall/decomp/decomp-<va>.txt`（含本轮新增 tg_dw1 批 4 MCP + 15 手解 + W1/W4 补充 0x140aa5150/
> 0x140b772b0/0x141467ce0/0x141466bb0/0x1412f6a20/0x141469a20）；既有 DIGEST/role-map 为底层来源。
> 六大 worker（tg_dw2..7）就地升级全部 82 章节；本文档由协调 agent 汇总 worker 报告 + 自读机器码。
> 与本域共享几何/曲线的 `utils::curve` 域（FUN_141467ce0 get_curve_segment_from_u、FUN_141469e00 2D→3D 投影、
> FUN_141469a20 approximate_curve_u、FUN_141469950、FUN_1414671d0）与 `utils/geometry/circle.rs`
> （0x140c97xxx 域）见 utils 域证据，此处只记调用点。

---

## 0. 处理链总图（三大流）

```
┌─ 发布流（每帧内墙 → 公开墙 → 空间哈希）──────────────────────────────┐
│  draw_wall/finish_wall/shape_*/merge/split/adjust/eraser             │
│    → InnerWalls (WallState 0x70, map 条目 0x78)                      │
│    → publish_public_wall_state 0x140a8b530                          │
│        遍历 InnerWalls/PublicWalls（0x2a8 步长 hashbrown 条目）      │
│        → FUN_141458b20/141458a50 曲线/形状处理                       │
│        → FUN_140b772b0 AnimateWallBulldozeBuffer::contains 跳过动画墙│
│        → FUN_1409e9640 material_stats 相关                           │
│        → FUN_140969720 SparseInstanceBuffer::allocate（2 静态类型 id │
│           0x142c2e2b8/0x142c2e360）                                  │
│        → 事件 OnWallChanged/OnWallDeleted（16B 行 {u64 id, u64 ?}）  │
│    → spatial_hash_update 0x140a8fd90                                │
│        OnWallChanged → PublicWalls 查表 → simplified_curve_2d 采样    │
│            → WallSpatialHash::add_segment 0x140aa5dd0                │
│        OnWallDeleted → WallSpatialHash::remove_wall_id 0x140aa5360   │
│    → 消费：query_with_circle_callback / query_with_rectangle2d       │
│      （shape_placement_params 0x14097f970、snapping 等）              │
└──────────────────────────────────────────────────────────────────────┘

┌─ 编辑流（UI 命令 → 状态变更 → 历史）─────────────────────────────────┐
│  ui_*（circle/rectangle/adjust_height/snapping/draw）                │
│    → 事件/命令 → finish_wall/wall_splitter/merge_walls/             │
│      set_wall_style/adjust_wall_height_impl                         │
│    → InnerWalls::_try_update_state<闭包> 族（6+ 实例化，逐偏移写）   │
│    → deferred_wall_history：                                        │
│        PendingWallChanged::try_new 0x1408dc9f0（投影→clamp01→        │
│          严格 low<high 否则 0x8000000000000000 = i64::MIN 哨兵 None）│
│        history_record_wall_changed 0x1408dccf0 → ConcurrentEdits 组  │
│        CompoundEditBuilder::wall_deleted 0x140a876a0（3 分支 Delete）│
│    → eraser archivist（cache_if_original_wall → shrink_wall →       │
│      record_rewiring_event → finish 0x14092f990 组装 CompoundEdit）  │
└──────────────────────────────────────────────────────────────────────┘

┌─ 删除流 ─────────────────────────────────────────────────────────────┐
│  ui_bulldozer 0x1409de190 → wall_eraser_bulldoze（RoofDelete FIRST   │
│    双组、装饰排空、楼梯断开）                                        │
│  wall_eraser 0x140accab0（拖笔选段 + archivist + ChildWall::new +    │
│    平屋顶高度补偿 adjust_wall_height_if_erased_flat_roof_w_no_rims） │
│  delete_wall 0x140aac1e0（PublicWalls+0x30 / WallEntities /         │
│    WallPlasters 三 map remove_entry + Commands::despawn 双类型 id）  │
│    → OnWallDeleted → spatial_hash_update → remove_wall_id           │
│  animate_wall_bulldoze 0x140ac27e0（phase 推进 CubicInOut + 曲线切片 │
│    + _try_update_state 增长 + 完成时 authority 移除）                │
└──────────────────────────────────────────────────────────────────────┘
```

**关键共享输入**：`PublicWallState::simplified_curve_2d`（public_walls.rs:50 断言
`assertion failed: w.simplified_curve_2d.is_valid()`）——发布/空间哈希/高度钳制三系统内联消费。

---

## 1. wall_state/ 系统算法（O 级，本批机器码）

### 1.1 WallSpatialHash 结构布局（O）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x08 | cells 数组 ptr（cell 步长 **0x30 = 48B**） | add_segment/remove_wall_id 越界读 |
| +0x10 | cells 数组 len（u64） | add_segment `+0x10 <= idx → panic` |
| +0x18 | hashbrown **wall_id → Vec<cell_index>** map（ctrl） | add_segment 插入 / remove_wall_id find（FUN_140b768c0） |
| +0x20 | 该 map mask | 探针 `& mask` |
| +0x30 | 该 map items ptr（0x38 步长 {key u64, value Vec 头}） | `+0x30 == 0` 判空初始化 |
| +0x38 | 该 map RandomState/哈希种子 | FUN_1405b0d70(self+0x38, &key) |
| +0x48 | dims 计数 u64（低 32 = grid_x 计数，高 32 = grid_y 计数） | grid_coord_to_cell_index `imul [rcx+0x48]`；query 读高低 |
| +0x4c/+0x50 | 尺寸 f32 × 2（u32@+0x4c / u32@+0x50，span 双分量） | world_space_to_grid_coord_unclamped 读 +0x50（x）、+0x4c（y 计数） |
| +0x50/+0x54 | cell_size（+0x50=span_x/count_x？见 §1.2） | cell_size 手解 |
| +0x58 | **u32 版本/代计数器**（add_segment 每次读旧值写 +1，member 记录携带；查询按 version 去重） | add_segment `+0x58 = uStack_128+1` |

**构造（new = 0x140aa5150，O，W1）**：malloc **0x30000 = 4096×0x30B = 64×64 cells**；
`dims {64,64}@+0x48`、`extent {130,130}@+0x50`（**extent 常量 130.0f @0x1429a3c70**）；
cells Vec cap-first @+0x0（cap 0x1000）；版本计数@+0x58；wall→格列表 map@+0x18。
**RESAMPLING = 0.28f（0x3e8f5c29）**——add_segment 段采样步进（`RESAMPLING < cell_size` 断言前提）。

**cell（0x30B）= 两个 cap-first Vec 并排**（O，add_segment/remove_wall_id）：
- `+0x00..+0x18` member Vec：{cap@+0, ptr@+8, len@+0x10}，元素 **0x38B**（每段一条）。
- `+0x18..+0x30` seg-index Vec：{cap@+0x18, ptr@+0x20, len@+0x28}，元素 8B（该格内 **wall_id 去重列表**）。

**member 记录 0x38B**（add_segment 逐字段写，O，W1 字段名细化）：
| 偏移 | 内容 |
|---|---|
| +0x00/+0x08 | 16B 段点（raw a/b：param_4 两次 8B） |
| +0x10/+0x14/+0x18/+0x1c/+0x20/+0x24 | 6×u32 段元数据：**h_a/h_b 高度 + snapped_a/b**（插值点/插值高） |
| +0x28 | wall_id（u64；remove_wall_id 用它匹配删除） |
| +0x30 | 插入时代计数器（+0x58 旧值，版本去重用） |

**wall_id → Vec<cell_index> map**（param_1+0x18）：add_segment 对覆盖格的每个 cell 追加索引
（首 cell 未命中 → `FUN_1400d9b20(0)` 分配 Vec 头 + FUN_141524fe0 push；后续 cell 命中 →
FUN_141524fe0(entry+0x30, cell_idx) 追加）；remove_wall_id 遍历该 Vec 反向删除。

### 1.2 网格坐标换算族（O）

- **world_space_to_grid_coord_unclamped** 0x140aa6520：
  `grid_x = floor((0.5×span_x + pos_x) / (span_x / count_x))`；`grid_y = floor((0.5×span_y + pos_y) / (span_y / count_y))`。
  常量：`0.5f @0x142925870`（span 半宽，网格以世界原点为中心）；饱和界 `0x4effffff @0x1429a49bc`
  → 超界返回 `0x7fffffff`（i32::MAX 哨兵）；NaN → 0。round helper = `func_0x000142923ee0`（floor jmp thunk）。
- **grid_coord_to_cell_index** 0x140aa65d0：`cell = gy × dims.x + gx`（imul [rcx+0x48] 低 32）。
- **cell_size** 0x140aa65e0：`{span_x/count_x, span_y/count_y}`（cvtsi2ss +0x48/+0x50/+0x54）。
- **bounds_ws** 0x140aa6600：cell 中心/网格边界 f64（movddup × 常量，双精度路径）。
- 断言族（wall_spatial_hash.rs）：`assertion failed: bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y`
  （line 0x44）、`bbx_max.x <= grid_max.x && bbx_max.y <= grid_max.y`（line 0x44）、
  `RESAMPLING < self.cell_size().x`（line 0x31）——出现在查询闭包与 new 的 maxps/minps movmskpd==3 门控。

### 1.3 add_segment 0x140aa5dd0（O 全读）

1. 读代计数器 `+0x58`，写回 `+1`（member 携带旧值）。
2. `FUN_1409ef870(param_10, 点分量×4, …, &覆盖格列表)` → 生成段覆盖的 cell 索引小 Vec（cap 8 起步）。
3. 遍历覆盖格列表（每项 u64 cell 索引）：
   - 越界 `+0x10 <= idx` → panic（UNK_142adce80，cell index 越界）。
   - 取 cell（+8 + idx×0x30）；member Vec 内 **8B 步长扫描**（`lVar18 += 8`）找既有 `wall_id`：
     - 未找到（len×8 == 扫描偏移）→ 若 len==cap 用 `FUN_140562870`（8B 元素扩容）扩 seg-index Vec，
       写 `wall_id` 到 seg-index Vec，len++。
     - 找到或新写后：若 member Vec len==cap → `FUN_1406319d0`（0x38B 元素扩容）。
   - 在 member Vec `ptr + len×0x38` 写 0x38B 记录（§1.1 表），len++。
   - **wall_id map 维护**：probe param_1+0x18（key==wall_id，0x38 步长）→ 命中 `FUN_141524fe0(entry+0x30, cell_idx)`
     （追加 cell 到墙的 cell 列表，goto 处理下一格）；未命中 → `FUN_1400d9b20(0)` 分配 Vec 头并插入
     {wall_id, [cell_idx]}（alloc 失败 → panic FUN_1428d80d0 @UNK_142c3aaa0）。
4. 结束 drop 覆盖格临时 Vec。

### 1.4 remove_wall_id 0x140aa5360（O 全读）

1. `FUN_1405b0d70(self+0x38, &wall_id)` 哈希 → `FUN_140b768c0(&found, self+0x18, h, &wall_id)` hashbrown find。
2. 未命中 → 直接返回（无 panic）。
3. 命中 → 遍历 value Vec 的 cell 索引（lStack_a8 计数）：
   - 越界 → panic（UNK_142adce68）。
   - 取 cell；member Vec 扫描找 `+0x28 == wall_id` 的段（0x38B 步长）：命中后 **紧凑前移**
     （逐 0x38B 记录 memmove 式覆盖），`cell+0x10 = len - 移除数`。
   - seg-index Vec（8B 元素）同样扫描/紧凑删 wall_id，`cell+0x28 = len - 移除数`。
4. 收尾 drop map 条目与 value Vec（0x10 对齐清理）。

**删除是紧凑式（无墓碑）**：O(cells×members) 扫描 + memmove。

### 1.5 查询族（O，任务点名）

**query_with_circle_callback**（三个单态实例 0x1422785d0 / 0x1425defd0 / 0x1425df320，仅回调类型不同）：
1. `bounds_ws`（f64 网格边界）+ 查询圆 {center_x f32, center_y f32, radius f32}（param_2 12B）。
2. **AABB 钳制**：min = maxps(网格 min, center)，max = minps(网格 max, center)（含 NAN 兜底选网格侧）；
   movmskpd==3（x、y 两车道全通过）→ 否则 panic `bbx_min`/`bbx_max` 断言（line 0x44）。
3. `0.5f < cell_size` 门控后：`world_space_to_grid_coord_unclamped` 求 min/max 网格坐标，
   饱和到 [0, count-1]。
4. 行循环 y（min..max）内列循环 x：`cell = grid_coord_to_cell_index(x,y)`；
   `FUN_140aa5c70(cell)` 取 cell 中心世界坐标；`dx = cx - center_x, dy = cy - center_y`；
   **半径过滤阈值 = radius + cell_size × 0.5 × √2**（`0x14292e6f4 = 0x3fb504f3 ≈ √2`，
   = 半径 + cell 半对角线）——`SQRT(dx²+dy²) < 阈值` 才扫描该格。
5. 相交格内逐 0x30B 条目：非空 member → 回调（写标志字节 0 返回）——语义 = "圆内是否有墙段"。
6. cell 索引越界 → panic（FUN_1428d9518 @UNK_143037640 域）。

**query_with_rectangle2d::closure$0** 0x1409ef210（**role-map 误标为 WallSpatialHash::new**，PDB 名
`impl$4::query_with_rectangle2d::closure$0`；**WallSpatialHash::new 真身 = 0x140aa5150**——纠错）：
1. 矩形 AABB（parry3d Aabb FUN_14123fff0）+ 网格边界钳制 + 同款断言门控（bbx_min/bbx_max/RESAMPLING）。
2. 求网格坐标范围并饱和 [0, dims-1]；行×列双层循环：
   - cell 索引 = row×dims.x + col（`iVar31*iVar33 + iVar27`）；`row = floor(idx/dims.x)`、`col = idx % dims.x`。
   - cell 中心世界坐标 = `{span.x×(col/dims.x - (1-col/dims.x))×0.5?, …}`（divps 归一 + 半宽常量
     `0x1429258e0={0.5,0.5}`、`0x142a171f0={-1,-1}`、`0x14292e6b0={1,1}`）。
   - parry3d `Aabb::contains`（FUN_141240170）测 cell 小 AABB 是否被矩形包含 → 包含则把该格全部
     member（0x38B 记录）追加到输出 Vec（cap-first，`FUN_1428a9a60` 扩容 + 0x38B memmove 拷贝）。
3. cell 索引越界（`len <= idx`）→ panic（UNK_142ac5500）。

### 1.6 spatial_hash_update 0x140a8fd90（O，W1 细化）

- 参数面：param_1=EventReader<OnWallChanged>、param_2=EventReader<OnWallDeleted>、param_3=Res<PublicWalls>、
  param_4=ResMut<WallSpatialHash>（+0x1c tick 回写 ✓）。
- OnWallChanged（16B 行）→ PublicWalls 0x2a8 查表（**缺墙 gated panic** `SpatialQuery received OnWallChanged
  with wall index `，全局 0x14362c3a8>1 才触发；无效墙 panic simplified_curve_2d）→ `remove_wall_id` 清旧段 →
  逐相邻点对经 `func_0x00014157f370`（utils::curve 段迭代）求最近段
  `t = clamp(dot/len², 0, 1)`（越界 panic `(0.0..=1.0).contains(&t)` utils/curve:42）→ 建段
  {raw 点对, 高度, 插值点, 插值高} → `add_segment`（RESAMPLING=0.28f 步进）。
- OnWallDeleted → `remove_wall_id`（hashbrown erase FUN_140b768c0）。

### 1.7 publish_public_wall_state 0x140a8b530（O 结构，101KB）

- puffin scope（0x142ad8bd1 等 profile 串）→ 收集 lazy 更新源。
- 遍历 InnerWalls/PublicWalls（**0x2a8 步长** hashbrown SIMD 探针 = 8B key + 0x2a0 PublicWallState 记录）。
- 每墙：`FUN_141458b20/141458a50`（曲线/形状处理）、`FUN_140b772b0`（AnimateWallBulldozeBuffer::contains，
  动画中墙跳过）、`FUN_1409e9640`（material_stats 相关）、`FUN_140969720`（SparseInstanceBuffer::allocate，
  两个静态类型 id 0x142c2e2b8/0x142c2e360）、事件写 OnWallChanged/Deleted（16B 行 {id, WallId}）。
- panics：`assertion failed: !is_adding`、`w.simplified_curve_2d.is_valid()`；7×FUN_14087f390 临时值 drop。
- **typed gap**：inner→public 逐字段复制映射仍 U（大函数未逐行）。

### 1.8 wall_roof_hashmap 0x140a94cd0（O，W1 细化；47KB）

两遍 Bevy Query 内联 archetype/table 遍历（**roof 记录 0x58B、wall 记录 0x30B，wall_id=*rec**）→
PublicWalls 0x2a8 查表（有效断言）→ **shape tag@value+0x258 > 1（Freehand）跳过**，预置形状 →
FUN_140eec970 设 roof 映射；**>1 roof → panic `Only one roof should be associated with a wall id`**；
模式资源 *param_1==0xe → fmt panic `Tried deleting roof {entity}`，==0xd → 删除分支。
param_3/6/7 三 ResMut（+0x1c）。
**纠错**：DIGEST 称 FUN_140d64e00/d4d740 为 Query 迭代器——实为 **种子生成器**（W1 机器码）。

### 1.9 调试可视化（O 结构）

- spatial_hash_debug 0x1408ec0e0：逐格 DebugRectangle（格边界调试绘制）。
- debug_viz_threshold_curve 0x140a8b0c0：逐墙 DebugSphere（阈值曲线可视化）。

---

## 2. shapes/ + valid_enclosure（O，W2 已回填）

### 2.1 资源/常量（O，W2 已回填）
- `add_wall_shape_ux` 0x140945f50：4 资源注册（合同 O）。
- **`calc_shape_height` 0x140ac80e0（O 全解）**：7 参系统（param_5/6 回写 = 2 ResMut）。
  形状门控 `tag != 6 || (ty-1) > 1`（ty 1/2 合法）→ 否则禁用预览。
  Curve2 建：rect `FUN_140c98cf0(Rectangle2d::as_curve2)`；circle `FUN_140c97cf0(Circle2d::as_curve2, 0.2f)`
  （采样步 0.2 @0x1429cd068）。`crashy_resample_remap_per_segment`（FUN_14146afe0，u 上界 1.0）→
  高度数组（0xc 步长）求 **sum + NaN 保护 max**。base 分支：rect **6.0f @0x142934690** / circle **8.0f
  @0x142934604**（否则 panic_str 行 40）。高度公式：`height_ty==2 → base + terrain_sample(Option f32)`
  否则 `avg + base + (-0.46)`；再 `-= (max + (-0.46))`。**clamp[0.7f @0x1429cd030, 14.0f @0x142a7de00]**
  写预览 +0x40。插值 `FUN_140c904e0(rate 10.0f @0x1429345a8)` → 16B 记录
  {tag=1, cur, prev, delta}；**prev 初值 = -2.5f @0x1429df1c0**（旧高==0 时）。

### 2.2 UI 编辑族（O，W2 已回填；详细算法见 §2.5）
ui_edit_circle / ui_rotate_circle / ui_edit_rectangle_dims / ui_edit_rectangle_orientation /
finish_edit_rectangle / shape_placement_params 等全部 O 级算法化（详见 §2.5 各条）。

### 2.3 其余 shapes（O，W2 已回填）
- circle_signifier 0x14165cd40（FUN_14165bd40 圆圈预览 + 0x28B 记录 `{1,1,cx,cy,r,flag}` + `cursor_circle_` 前缀）、
  edit_arrow 0x140b35120（Freehand tag==2 跳过；FUN_140c90a40 圆方向 → spawn `edit_circle_` 0x30B；
  clone_behavior 0x140002370 = 恒 0）、edit_line 0x140a64a40（get_pos_at_u×2 + 采样 → 端点，常量
  0.23 @0x142a89600 / 0.75 @0x1429edb90 / 90.0）。
- rectangle_signifier 0x14155e460（FUN_141559ce0 闭包 + 0x28B + `cursor_rectangle_`）、
  signifier_edit_dims_line 0x1408dda10（矩形角/曲线绘制，0.01/1.0 常量）、ui_modes/edit_dims 0x14155eba0
  （矩形角×4 → widget）、ui_modes/edit_orientation 0x14083d3f0（旋转 box，1.5/0.3/0.25 常量，2D→3D）。
- shape_placement_signifiers 0x1408dec70 / preview_snapping_line 0x1408dfc60：expand（圆 +0.315 @0x142aa3280、
  矩形 +0.63 @0x142aa4550/4554）→ as_curve3 → 法向 → t=clamp（param_5/弧长）→ 切线端点 →
  断言 `!u.is_nan/is_infinite`。shape_signifier_lerp 0x1409ef010：跨形状 panic；circle→{0,r} rect→{1,x,y}，
  ease=func_0x142923ec0(rate·t)，增量钳 |Δ|。
- add_wall_shape_ux 0x140945f50：App 插件注册 8×FUN_1400dd6f0（add_systems，128 位类型哈希）+ 类型表兜底；
  资源 CalcShapeHeight/PreviewCircle/PreviewRectangle/ShapePlacement；哈希→系统映射 U。

### 2.4 ShapeParameters（valid_enclosure，O，W2 全解）
**28B 记录**：`tag u32@+0`（**位 0 判别**：0=Circle/1=Rectangle，`test byte[rax],1` 双路尾跳→叶函数）；
**Circle 载荷 16B** {cx, cy, r, spare@+0x10}；**Rectangle 载荷 24B** {cos, sin, cx, cy, hw, hh}。
几何叶函数在共享库 `crates/utils/geometry/circle.rs`（0x140c97xxx 域）：
- **圆采样** `count = clamp(ceil(2πr/0.2), 3, …)`，点 = `(cx + r·cos((1-u)·2π), cy + …)`；`perimeter = 2πr`
  （**2π = 0x40c90fdb @0x142934674**，步长 0.2 @0x1429cd068）。
- **contains_shape（W2 纠错：非 parry3d）**：圆 = other 中心距² < r²；矩形 = 采样点全含
  （circle other u∈{0,1/6,1/3,1.0,2/3,5/6}；rect other 4 角）旋转 AABB（K=1.0）。
- **expand**：圆 `r += amount` 且 f64@+0x14 += 2a；矩形 hw=hh=2a（U 调用方语义）。
- as_curve2/as_curve3/center/get_normal_at_u/get_pos_at_u/perimeter 位 0 双路分发；
  `get_closest_u_from_pos @0x1411ff640 / perimeter @0x1411ff660`（**PDB 表错位**，W2 纠错）。

### 2.5 create_circle / create_rectangle 共享管线（O，W2 全解）
- **create_circle** 0x14097e560：事件 48B/行；待建墙命令 0x30B {tag 0, ShapeParameters, 海拔 4.0, flag 1}
  → as_curve2（iterator unwrap panic curve/mod.rs）→ 2D→3D → new_wall（0x14085a370）→
  _try_update_state ×3 → colors → CreateRoofCmd 0x40B {height@+0x2c} → 历史 tag8(0x50B)/tag9(0x30B)
  CompoundEditBuilder::record。9 ResMut。
- **create_rectangle** 0x140a93790：事件 0x70B；命令 0x30B {tag1, ShapeParameters} → Rectangle2d::as_curve2
  → new_wall → ×3 更新 → colors → **脊长比例 `max(dx/dy, dy/dx) + (-2.25 @0x142aa5084)`** → / → clamp01 →
  屋顶钳（0.1 @0x142925984）→ CreateRoofCmd → 历史 tag8/9。与 circle 同构。
- **ui_create_circle** 0x14165bed0：门控 (6,1)；**光线地平面求交** `t=(cy−oy)/dy` → `P=o+t·d` →
  `r = SQRT(Δ²)` → **clamp[0.9 @0x142a7c880, 8.0 @0x142934604]** → PreviewCircle+0x18（center+0x10）；
  标记点 +0x348/+0x360；circle_signifier 0x28B；提交 CreateCircleCmd。
- **ui_edit_circle** 0x14165d460：hashbrown 查墙（0x78）→ 拖拽新半径 → as_curve2 → update curve
  （140aaa090）+ enclosure（140aa9610）→ 历史；释放时 `|Δr| <= 0.01 @0x142960674` →
  FUN_140b2c4b0 恢复装饰并跳过历史。
- **ui_rotate_circle** 0x1409ac910：光线交面 → FUN_140c90e20（圆旋转角）+ FUN_1416f3210（屏幕角度）→
  写回；ease（rate·0x142a16d10）；角度差 <= 0.01 跳过历史。
- **finish_edit_rectangle** 0x141464a60：变体 1（维度）/2（朝向）场偏移表；先发 0x58B 快照；
  **双判据 `|acos|×57.29578 @0x142b1d78c <= 0.01` 或 `SQRT(Δdims²) <= 0.01`** → 无变化 → 装饰恢复；
  否则 CompoundEditBuilder::record。变体非法 panic 行 40。
- **ui_edit_rectangle_dims** 0x140ad6250/6e00：get_distance_to_edge = 两角 → edge → `L²` 钳
  （~2.9e-20 除零 @0x142a7baa0）→ `t = dot/L²` → 新 dims = 光标投影 → EditRectangleCmd 0x28B {tag2, 边索引}。
- **ui_edit_rectangle_orientation** 0x140ad8000/8be0：`cosθ = dot/√(||·||)` → **acos 8 阶多项式 × √(1−|c|)**
  （系数 0x142ae0d44..60）→ cross 定象限 → π−acos（π @0x1429edba4）→ NaN 防护
  （0x7fc00000 @0x142a8d5c4）→ 0.25 量化（0x142925950）。
- **shape_placement_params** 0x14097f970：光标→模式分派（case0/9 vs 2）；case2 = PublicWalls 查墙
  （预置 tag<2）→ u + 法向（circle FUN_140c978e0 @+0x25c / rect 读 cos/sin）→ roof_entity →
  FUN_1409767a0（closest_segment 屋顶）→ snap_to_roof_ridge（1.0）→ new_unsnapped /
  FUN_1409766a0（closest 墙）+ FUN_140b04f70（空间哈希）×3 → 状态字节。阈值 60 钳制 / 1.5 吸附 /
  offset −6 @0x142aab4d4、−8 @0x1429cd074。

## 3. snapping/（O，W3 已回填）

### 3.1 snapping_init 0x140ada600（O 全读）

1. ResMut<SnappingMode> tick 回写（+0x1c → +0x2 字段）；mode 字节复位 0。
2. 早退：`*param_4+0xfd` 标志（抑制）；`*param_3` 进行中墙计数空。
3. **InnerWalls map（0x78 步长条目 {WallId, WallState 0x70}）**查进行中墙 id：
   - 未命中 → 兜底路径（UNK_142ae1968 静态点表）。
   - 命中：**`WallState+0x30` f32 = 曲线长度 ≤ **1.65f @0x142ae0d64** → 返回
     （W3 纠错：**非"首段"长度**，为总曲线长度）。
4. 读曲线点数组（WallState 双 Vec：ptr@+0x8、len@+0x10；点=8B {x,y} f32 对）。
   - len<2 → 跳过；逐点求到 p0 的**最近点**（平方距离 f32 运行最小）。
   - 距离钳制下限 epsilon **≈1e-20 @0x142a7baa0**（`0x1e3ce508`）。
5. **线段投影判定**：最近点 p1 构成线段 p0→p1；逐点投影
   `u = ((p-p0)·(p1-p0))/|p1-p0|²`，垂距 = p - (u·p1 + (1-u)·p0)；
   任一垂距 `SQRT` > **1.5f @0x1429258d0** → 吸附失败；全部 ≤ 1.5f → 吸附成立。
6. 结果：mode=1（段吸附态），`snapped = (全点通过)`；u 公式用 `1.0f @0x142925904` 权重。

### 3.2 snapping_maintain 0x140a6ad00（O，W3 手解，117B）
非大系统——是 **mode 跳表（0x142ad55cc）分发的快照打包** → 写 `+0x20..0x38 {mode, snapped, u64, f32, u64, byte}`。
**纠错**：章节"mod key 切换 + NotifyHintSystemSnapping"无机器码依据。check_for_disable 0x140a6a560：
mode∈{2,3} → 写 DisableSnapping 事件（**16B 行非 20B**），多数路径清 mode=0。

### 3.3 snapping_to_segments 0x140a6b5e0（O，W3 细化）
mode==1 前置；**光标 ±8f @0x142ab6630 AABB** → `WallSpatialHash::query_with_aabb_clamped`
（0x140aa5a10）；候选 **1.5f 内** + PublicWalls 断言 → 首个有效即写 mode=3 /
height=`TerrainHeightsData::sample_at_world_xz`（0x1409e9640）/ pos / wall_id。

### 3.4 其余 snapping（O，W3 已回填）
- snapping_signifiers 0x140a6a650：mode==2 写端预览 {pos, 1.5×1.5, 1.5f @0x142ad3650}；mode==3 →
  PublicWalls 探针 + preview_snapping_line（0x1408dfc60）+ DebugLine/DebugSphere::show
  （0x140b04f70/0x140b02520——**confirm：0x140b04f70 为 gizmo 绘制**）；unwrap panic @snapping_maintain.rs:43。
- snapping_audio 0x140a6a480：mode==2 且 +0x20==2 跳过 / mode==3 且 +0x20>1 跳过（**+0x20 = 上一帧发布 mode
  去重**）；否则 xvy 吸附点 → PlayWallUiSound 0x20B 行，kind=1/2。
- telegraph_walls_are_merged 0x14086ad70（**PDB 头名错位 register_required_components**）：逐墙 0x2a8
  校验后取 simplified 曲线（12B Vec3），按 `f32@value+0x284 × 0.5` 平移复制 → try_new_from_points
  （0x141469230）→ signifier 流 kind=5；断言 straighten_curve_cache.length_ws()!=0 与 CurveU u/t 范围。
- **SnappingMode 布局补全**：mode@0/snapped@1/u32@4/Vec2@8/WallId@0x10/end@0x18/快照+0x20..0x38；
  new（0x140ade610，11B）只写 mode=0 和 +0x20=0，**不初始化 snapped**；mode 2 vs 3 的 +4..0xc 按 mode 复用。

### 3.2 snapping_to_ends 0x140a6ad80（O，W3 细化）

1. `in_progress.+0x0 == i64::MIN`（溢出标志判 MIN）→ return（无进行中墙）。
2. 保存 `snapped@1`；**前置 `while snapped==0`**（已吸附时不做端吸附）。
3. 遍历 PublicWalls（0x2a8 步长）：有效断言（public_walls.rs:50，**4 个内联点**）；
   `shape tag@value+0x258 != 2` → 跳过（仅 Freehand）；`wall_id == in_progress.+0x80` → 跳过自身
   （改用进行中墙首点作端点，`+0x9e 标志&1` → 强制 snapped=1）。
4. **最近端判定**：`SQRT(|光标-首点|²)` vs `SQRT(|光标-末点|²)`；首近记 0、末近记 1；
   **最小距离初值 1.0e8f（0x4cbebc20 @0x142ad4950）**，`dist < min` 更新候选
   （wall id + 端点 + 端标志）。
5. `min >= 1.5f @0x1429258d0` 或从未命中 → return。
6. **投影**：`Curve::get_coord_at_u<Vec2>`（FUN_141467ce0，curve@value+0x38 → {段 u, 分数 t}）；
   越界 → panic_bounds_check；`t∉[0,1]` → panic curve_u:42；`u.is_nan()` → panic curve_u:29。
7. **3D 吸附点**：`lerp(point[u], point[u+1], t)`（12B Vec3）→ {x,y}（u64 打包）+ z。
8. 写 SnappingMode：`mode=2; snapped=wall+0x29e; pos(+4/+0xc); wall_id@+0x10; end@+0x18`。

## 4. wall_splitter / structure_outline（O，W3 已回填）

### 4.1 wall_splitter 0x140a74c20（O，W3 细化；61KB，~14 参）

1. **mode!=1 → 返回**；`UiState::add`（0x1409231f0）查状态。
2. `get_approx_coord_from_pos`（0x141469e00）→ `u = 0.5/x clamp[0,1] + 1-u`；
   切割点 lerp + 单位切线 + `xz_projective_rotation_bad`（0x140c911d0）。
3. 2 子墙 = `crashy_slice`（FUN_141466bb0）+ 地形采样 + HashMap insert（0x141529bb0）+
   wall_ids +0xc0 脏代 + `InnerWalls::remove`（0x14085aa60）。
4. **11×FUN_1428d9518 全为 panic_bounds_check 下标越界**（5×curve/mod.rs、6×wall_splitter.rs）——
   **非 11 条独立消息**（纠错 DIGEST §2.2）。
5. history/decorator/archivist 收尾（on_wall_split 0x140a1f590 等）。
6. signifier `split_wall`/`split_wall_scissors`。

### 4.2 structure_outline 0x140959fc0（O，W3 细化；98KB）
- ~15 参；param_1/2 事件流（0x18B 行）；**AppMode 门控 = `tag==2 || (tag==10 && 子字节==0)`**
  （**纠错：非 Esc 门控**，DIGEST §2.2 需改）。
- OutlineWallCmd/OutlineStairsCmd 行 0x18B {u32 id1@+1, f32@+2, u32 id2@+0xc, byte@+0x14, byte@+0x15}；
  `f32@+2 > 0` 才处理；byte@0x15==0 → 公共墙表（0x2a8 步长，双 id 匹配），否则 → StairsState 连通节点。
- 命中墙 → 有效断言（public_walls.rs:50）；**形状分支 tag@value+0 <2（预置）** → as_curve2/perimeter/
  Rectangle2d::new_centered_axis_aligned + 逐 u 采样闭包（closure$8/3/9/6 族）生成描边点；
  屋顶 profile_curve_normalized 参与；Freehand 另一采样路径。
- **signifier 名前缀表**（地址）：outline_wall_top_ 0x142ab4e70、outline_wall_bottom_ 0x142ab4e08、
  outline_roof_side_ 0x142ab5170、outline_roof_ridge_ 0x142ab5108、outline_circle_roof_ 0x142ab5080、
  outline_circle_side_ 0x142ab4ff8、outline_rect_corner_ 0x142ab51c8、outline_side_left/right
  0x142ab4ef0/4f38——对应墙顶/底/屋顶侧/脊/圆顶/圆侧/矩形角/侧边描边路径。
- 采样步长 `uVar66*0xb`（11，I：每段 11 采样）；高度/法向参与常量 0.1f/0.4f。

## 5. adjust_wall_height / eraser / deferred_wall_history（O，W4 已回填）

### 5.1 history_record_wall_changed 0x1408dccf0（O 全读）

1. ResMut tick 回写 ×2（param_1/param_2 +0x1c）。
2. 读资源 `+0x120` **Option 字段（i64::MIN = None 哨兵）**；`+0x120 < 0` → 早退（无待记变更）。
3. 置 `+0x120 = 0x8000000000000000`（清空 pending）。
4. 拷 pending 载荷：`+0x128..+0x1cc` 多个 8B/4B 字段 → 栈（224B 区域 = 0xe0 memmove 起点）。
5. 组装 CompoundEdit 记录：alloc 0x18 头（`{1, &inner, 1}`）+ alloc 8B 槽 + alloc **0x108B** 记录
   （从栈 0xe0 载荷拷出）+ `FUN_141696f50` 收尾 + `FUN_140a84ec0`（CompoundEditBuilder push）。
6. alloc 失败 → 直接 abort（FUN_1428d9000 + invalidInstructionException）。

### 5.2 CompoundEditBuilder::wall_deleted 0x140a876a0（O 全读）

1. 读输入结构：`+0x10` u64 计数 → 拷贝 `+0x8` 起 **u64 id 数组**（uVar3×8B）；
   `+0x28` u32 计数 → 拷贝 `+0x20` 起 **u32 id 数组**（×4B）。
2. 判别 `+0x38`：`clamp(x-2, 0, 2)` → 0=variant 2、1=variant 3、2=原生值（拷贝 `+0x3c/+0x44/+0x4c` 载荷）；
   `+0x30/+0x54/+0x58/+0x60/+0x68` 附带字段。
3. 组装记录头 `uStack_1a0 = 0x8000000000000001`（**enum niche：变体 1 = Delete 操作**，PLAYBOOK §2.3 编码）。
4. `FUN_140a82b40(builder, &record)` push。alloc 失败 → dealloc + abort。

### 5.3 animate_wall_bulldoze 0x140ac27e0（O，W4 细化）

Cmd 记录 **0x98B** {W@+0x70, lo@+0x78, hi@+0x7c, anchor@+0x80, phase@+0x8c, finished@+0x90}。
InnerWalls map（0x78 步长）逐墙；
**phase = clamp01(phase + dt/0.2f @0x1429cd068)** → CubicInOut ease（func_0x000140c961e0）→
`u_lo = lo*t`、`u_hi = 1-t + hi*t` → `FUN_141466bb0`（曲线切片）→ `_try_update_state<update_wall_state>`
0x140aa9bc0 写回；**完成时 `InnerWalls::remove` 0x14085aa60 + `FUN_1412f6a20`（retain_finished）**。
`AnimateWallBulldozeBuffer::new` 0x140a82b20 = {cap=0, ptr=8, len=0}。
**纠错（W4）**：publish/章节所称 `0x140b772b0 = AnimateWallBulldozeBuffer::contains` —— 实为
**hashbrown remove-and-return-entry**（0x2a8 条目拷出；两 worker 分歧待集成 owner 裁决，publish 处用法
为取动画墙条目）。

### 5.4 adjust_wall_height（O，W4 全解）

- **AdjustWallHeightCmd 0x30B 行**：{flags@0（bit0=follow-terrain）, pos@+4, y@+0xc, ty@+0x10
  （=1 否则 panic `not yet implemented` 行 19）, height@+0x14, WallId@+0x18, flag@+0x20}。
- 流程：hash 查 InnerWalls（0x78）→ 当前高 value+0x54 → `clamp_wall_height` 回退命令高 →
  follow-terrain 采样 TerrainHeightsData（FUN_1409e9640，512×512 u16）→
  `_try_update_state<update_wall_state_height>` 0x140aaaca0 ×2（机器码 0x140ac24b3/25a3 双调用同参，
  premade 且 flag@0x20==0 重入）→ 输出 0x28B（tag 3）/0x20B 双事件。
- **高度钳制规则（数值全解）**：有屋顶 → `roof01<=0.1 ? 0.7 : 1.6`；无屋顶 → `style==0 ? 0.5 : 0.3`；
  max = `style==0 ? 14.0 : 2.5`；两次钳制不等 → history + premade 入 Vec。
- **flat_w_no_rim 判定** = `roof+0x30<=0.1 && roof+0x3d!=0`（Roof::is_flat_w_no_rim）；偏移
  **-0.29 @0x142a94920**（signifier 动画）/ **-0.79 @0x142a7e320**（erase 补偿）。
- `clamp_wall_height` 0x140ac7f30：PublicWalls 0x2a8 查墙 → 曲线有效断言
  （ui_adjust_wall_height_mode.rs:50）→ roof_entity → bool + 钳制高。
- `get_closest_point_u_to_cursor` 0x140ac7400（**PDB 名 init_mode 错位**）：逐窗口 mat3 投影最近段 +
  u lerp；panic `!curve.points.is_empty()`(:42) / CurveU(:29)。
- `ui_adjust_wall_height` 0x140b2fc00：roof 链 fVar3==1.0 门 → widget 返回 ty@+0x18/+0x19 分派 4 类 16B 事件。

### 5.5 eraser（O，W4 全解）

- **wall_eraser** 0x140accab0：拖笔逐点投影 u → Vec<{u,f32}>（间距 **0.1 @0x142925984** 合并相邻），
  单点早退，u 越界 panic curve_u(:42)；每段 = 0x20B 事件 + decorator cull + cache_if_original_wall +
  `FUN_141466bb0` 切片 + proposed_curve.is_valid(:43) + shrink_wall；ChildWall::new = 曲线拷贝 +
  shape tag-2 三路 + update_wall_state_curve 0x140aaa090 + enclosure→2 0x140aa9610；
  高度 = `min(当前, lerp(0.75 @0x1429edb90, 14.0, t))`，flat_roof_w_no_rims 时 `+= -0.79`。
- **archivist**：记录 **0xf8B**；cache_if_original_wall（双 map 查）细节 U；shrink_wall 双 map
  （0x10/0x90 步长）+ FUN_140a24000；finish 0x14092f990 = 0xf8B 遍历 + FUN_141466bb0 切片
  （u=0.2/elapsed×0.5）+ 断言族 + build 0x141696f50 + commit 0x140a84ec0。
- **ui_bulldozer**：condition = `(*app).tag==9 && +4==3`；bulldoze = 光线拾取按 hit_type 分派
  （0x11 树 → delete_tree_/cursor_hammer_ signifier；墙型扫 AnimateWallBulldozeBuffer 防重；roof 3D 距离），
  尾跳表分派（RoofDelete FIRST / WallCompound StateChange+Delete）。
- 新 decomp（W4，tg_dw5）：0x140b772b0 / 0x141467ce0 / 0x141466bb0 / 0x1412f6a20 / 0x141469a20。

（W4 继续：adjust_wall_height_impl、clamp_wall_height、wall_eraser、archivist finish）

## 6. root 生命周期系统（算法级，W5 已回填）

### 6.1 calc_wall_bend 0x140a8a710（O）

签名：`(EventReader<OnWallChanged>, Res<PublicWalls>, ResMut<Events<WallBendChangedEvent>>)`。
puffin scope 串 `country_core::systems::wall::calc_wall_bend::calc_wall_bend`（0x142ad8b93）。

1. 事件行 16B `{event_count u64, WallId u64}`；逐事件。
2. **PublicWalls 查表**：hashbrown 条目步长 **0x2a8** = `{key WallId u64@+0, value PublicWallState 0x2a0@+8}`。
   未命中 → 跳过（无事件）。
3. **退化校验（硬 panic，不跳过）**：`value+0x10`（raw 曲线点计数 u64）`< 2` 或
   `value+0x30`（raw 曲线长度 f32）`<= 0.0` → `panic_str("assertion failed:
   w.simplified_curve_2d.is_valid()" @0x142ada193, line 0x32, public_walls.rs @0x142ad91d0)`。
4. **发布曲线**：`value+0x40` = 点数组 ptr、`value+0x48` = 计数；点 **12B `{f32 x@0, f32 y@4, f32 z@8}`**
   （本函数只用 x@0 与 +8 字段 = 地面 (x,z)）。计数 `< 6` → bend = 0。
5. **法向收集**：`for i in 0..4` 取 `P[n-1-i] - P[n-2-i]`（**从曲线末端**倒数 4 段），归一化
   `{dx/len, dy/len}`（len=SQRT(dx²+dy²)，1.0f @0x142925904），入 `SmallVec<[Vec2;4]>`
   （栈内联 8B 元素；grow FUN_140b81f90）。逐点索引边界检查（FUN_1428d9518 + calc_wall_bend.rs）。
6. **角度**：相邻 4 方向求 3 点积 d0·d1/d1·d2/d2·d3 累加 → `divss` 除以 (n-1) → `maxss`(-1)+`minss`(1)
   双夹取 → 单次 `acos`（机器码 0x140a8acf0 段：movsd/mulps/movshdup/addss 水平点积）→ 结果 `minss` ≤ 1.0。
7. **事件写出**：ResMut<Events<WallBendChangedEvent>>（param_3，`param_3[2]=param_3+0x1c` 回写）；
   行 16B `{event_count u64@+0, bend f32@+8}`；Vec 扩容 FUN_140562630（16B 元素）。

**纠错（vs 重建 calc_wall_bend.rs）**：重建 `compute_bend` 取 `points[i]→points[i+1]`（**首端** 4 段）；
机器码取 `P[n-2-i]→P[n-1-i]`（**末端** 4 段）。方向相反，需改。重建注释「map value +0x48/+0x50」
为条目相对偏移 = value+0x40/+0x48，与机器一致（表述需注明基）。其余（计数<6→0、3 点积均值、
双夹取、acos、min 1.0、16B 行）机器码全部证实。

### 6.2 wall_highlight 0x140981390（O）

签名：`(ResMut<WallHighlight>, ResMut<EditingHighlight>, EventReader<HighlightWallCmd>, ResMut<WallEntities>)`。

1. **每 tick 先清** WallHighlight **ty 字节 @+0x10 = 0**。
2. 事件行 **0x18B** `{u64@0, wall_id u64@8, ty u8@0x10}`。
3. **WallEntities 查表**（param_4 deref，map@+0x20，条目步长 **0x10** = `{key u64@0, value Entity u64@8}`）：
   value = `{index u32@0, generation u32@4}`；**高 32 位（generation）== 0 → 直接 return**（无高亮）——
   与 delete_wall 相同「空实体=0」编码。
4. 写 **WallHighlight** `{entity 8B@0, wall_id 8B@8, ty u8@0x10}`（param_1 回写）。
5. **EditingHighlight rgb**（param_2 回写，`u64@0` = r,g 打包 + `u32@8` = b）：
   - `ty==1` → `(0.10, 0.03, 0.00)`：u64 @0x142ab6650 = `{0.10f, 0.03f}`，b=0；
   - `ty!=1` → `(0.07, 0.08, 0.07)`：u64 @0x142ab6640 = `{0.07f, 0.08f}`，b=`0x142a7e354` = `0.07f`。
   重建 HighlightColor::V0/V1 数值完全一致 ✓（实际仅两色；「四常量」第四 = b 通道 0.07f）。

### 6.3 delete_wall 0x140aac1e0（O）

签名：`(EventReader<OnWallDeleted>, ResMut<PublicWalls>, ResMut<WallPlasters>, ResMut<WallEntities>, Commands)`。
事件行 16B `{id, WallId}`。每事件：
1. **PublicWalls 移除**：find 经 `*param_2+0x50` 区，`remove_entry` 于 **`*param_2+0x30`**（FUN_140b76f90）。
2. **WallEntities 移除**（param_4）：find@+0x20，remove（FUN_140b76f90）；返回墙实体 `Option<Entity>`，
   高 32 位==0 → 跳过。
3. **墙实体 despawn**：`Commands::despawn` 类型 id = **UNK_142add9c8**（FUN_140d14770，Commands 队列 = param_5[0..3]）。
4. **WallPlasters 移除**（param_3）：find@+0x20，用**另一 erase 助手 FUN_140b777f0**；返回
   石膏 `Vec<Option<Entity>>`（8B 条目，cap-first）。
5. **石膏逐实体 despawn 循环**：迭代，`while(entry>>0x20==0)` 跳过空位，命中 `>>0x20!=0` →
   `Commands::despawn` 类型 id = **UNK_142add9e0**；结束释放 Vec 缓冲（free(ptr, cap*8, 8)）。
6. 三 ResMut tick 回写内联（param_2/3/4）。无专用 panic。

### 6.4 draw_wall 0x140a7cba0（O，83KB 结构级）

签名（11 参）。入口：输入可用性 `func_0x000140c16a00`，**空 → 早退**。puffin scope
`country_core::systems::wall::draw_wall`（0x142ad7358）。param_4 = query 束；`param_4[4]` =
InProgressWallResource 的 temp 曲线区（大记录）。
ResMut/EventWriter（+0x1c，5 槽）：param_1（plStack_f0）、param_2、param_7、param_8、param_9
（**DIGEST 只列 4，param_1 亦有回写 —— 补充**；param_1 或为 EventWriter）。
InProgressWallResource 字段：wall id u64@+0x80、f32*@+0x94、u32@+0x98、**freehand_ty u8@+0x9c**、u8@+0x9d。

1. **吸附解析**：`FUN_140ad3f50` ×3（SnappingMode，返回 `{u8 mode, …}`），mode==3 判别。
2. **曲线输入记录**：`FUN_14145c350` ×2（记录鼠标输入进曲线/平滑）；`func_0x000142923fa0(1.0/fVar6)`
   （角度/比例，fVar6=平滑配置 f32，下限 `_UNK_142950d70`）；`_UNK_14295a0dc/14295a0e0` 区间裁剪。
3. **曲线更新**：`FUN_140892720` ×6（InProgressCurve 构造/吸附路径；temp 曲线非空时调用）。
4. **InnerWalls 查表**（param_1 deref，map@+0x20，条目步长 **0x78**）：key = InProgressWall id@+0x80；
   命中读既有墙高度 f32@value+0x54；`freehand_ty@+0x9c==0` 选 EventWriter param_8/param_9，push
   **0x18B 行** `{u64@0,u64@8,u32@0x10,u8@0x14=0}`（扩容 FUN_140562db0）；随后
   **update_freehand_ty FUN_140aa99e0 ×3**。
5. panic：panic_str（draw_wall.rs）多处；capacity FUN_1428d8fe3；曲线 `Result::unwrap` Err
   （0x142c63340，utils curve/mod.rs "Curve…"）。

### 6.5 finish_wall 0x140a800a0（O，59KB 结构级）

签名（13 参）。**6 个 +0x1c tick 回写槽**：param_3（plStack_78）、param_4、param_6、param_9、param_10、
param_11（**DIGEST 列 5 漏 param_3 —— 纠错**）。param_3 = ResMut<InnerWalls>。
事件 16B 行（行内 u8 标志@+8）；墙 id = InProgressWall+0x80。

1. **InnerWalls 查表**（0x78 步长）。命中：param_6 回写 + 读记录 ~0x78B 入栈 + **`*param_6 =
   0x8000000000000000`**（None，清 temp/历史游标）；复制墙 2D 点 Vec `{cap@value+8, ptr@value+0x10,
   len@value+0x18}`（8B 点）。
2. **历史记录**：`FUN_1408dc9f0`（history_record_wall_changed）→ 0xf8B 结构；
   **CompoundEditBuilder**（param_10）：`*param_10+0x120` 两段 **0xf8B 拷贝**（旧→新，func_0x000142923c00）。
   失败 → panic `assertion failed: pending_wall_history.wall_changed.replace(expand).is_none()`
   （0x142ad74bb，行 0x4d=77）—— **揭示 builder 字段 `pending_wall_history.wall_changed`**。
3. **Builder 族 ×3**：0x140a82b40（@744/1202/1305）、0x141696f50（@758/1215）、0x140a84ec0（@766/1223/1324）。
4. **InnerWalls 方法**：FUN_14085a560（get by id → 形状 tag@+0x38 变体分派）、FUN_14085aa60 ×2（采样）。
5. **围合失败 panic**：墙不存在 → fmt panic `Finish wall marked a wall as non-enclosed, but it doesn't
   exist`（0x142ad75d0；模块串与消息串相邻存储，FUN_1428d9390 带 WallId）。
6. **freehand_ty@+0x9c** 选择**稠密 f32 数组**（param_12/13，`Vec<f32>` 以 wall id 为下标）；
   越界 → FUN_1428d9518。`*plStack_88 = -0x8000000000000000`（None 哨兵写回）。
panic 串：`Was asked to finish a wall, but wall_manager has no temp curve`（0x142ad75a8）。

### 6.6 merge_walls 0x140999920（O，27KB）

签名（12 参）。事件 = param_7，**行 40B** `{…, id_a u64@+8, id_b u64@+0x10, …, u32@+0x1c}`。
panic/断言：`assertion failed: ev_count < 2`（0x142abb086，行 0x1e=30，**每帧至多一个合并事件**）；
`id_a==id_b` → panic（0x142abb0f0，行 0x28）；缺墙 → panic_str（0x142abb130/190）。
ResMut/EventWriter（+0x1c，7 槽）：param_3/4/5/6/8/9/11。

1. **地形采样**：对 wall_a/b 各调 `FUN_14085aa60`（InnerWalls 方法）→ 各 0x28B 采样结构。
2. **曲线拼接**：分配 f32 缓冲，**反转一侧 + 追加另一侧**（连接点去重）；
   `FUN_141469430(&out, points, 0)` = **crashy_new_from_points**。
3. **新墙创建**：`new_id = *(InnerWalls+0xc0) + 1`（+0xc0 代计数 +1 即新 id）→ `FUN_141529bb0`
   （+0x30 工厂）→ `FUN_1402ca2f0(+0x30, new_id)`（wall_ids 插入）。
4. **装饰归档放行**：`FUN_140b2bc00`（decorator_cull_allow_any）×2（wall_a/b）。
5. **事件 ×3**：`FUN_1413e7230`（tag 串 `merge a`/`merge b`/`merge target` @0x142abb1c0/1c7/1ce）。
6. **装饰表合并**：`FUN_140a1ea40` = DecoratorStorage::on_walls_merged(a, b, new_id)。
7. **PendingMergeWalls 构建**：拼 **0x120B 结构**（两 0x28B 采样 + `wall_a@+0xf8, wall_b@+0x100,
   merged_id@+0x108, u32@+0x110, u32@+0x114, u32@+0x118`）；drop 失败 → panic
   `assertion failed: pending_wall_history.merge_walls.replace(PendingMergeWalls { adjust_height,
   wall_a: ev.wall_a, wall_b: ev.wall_b, wall_state_a, wall_state_b, merged_wall_id,
   merged_color_id: merged_wall_color, wall_color_a: ev.wall_a_original_color_id,
   wall_color_b: merged_wall_color, }).is_none()`（0x142abb1da，行 0x1a0）—— **揭示字段序与颜色字段**。
8. **地形高度**：`FUN_1409e9640`（TerrainHeightsData 采样）于合并曲线**末点**。
9. **事件写**：param_8 EventWriter，**0x20B 行** `{u64@0, u8=4@8, {f32,f32}@0x10, f32@0x18}`（tag=4）。

### 6.7 set_wall_style 0x1409a2290（O，21KB）

签名（7 参）：param_1/2/3 = 三 EventReader；param_4 = ResMut<InnerWalls>；param_5 = Res<PublicWalls>；
param_6 = ResMut<CompoundEditBuilder>；param_7 = EventWriter<RequestWallMaterialAudio>。

**行宽核实（纠错）**：param_1 事件 **0x18B**（SetFreehandHeightTypeCmd）、param_2 **0x18B**
（SetWallStyleCmd）、param_3 **0x20B**（SetPlasterPeelAmountCmd）。章节头猜测
「SetFreehandHeightTypeCmd 0x20B」不符 —— freehand 行 0x18B、plaster 行 0x20B。

三个读器：
1. reader1：`FUN_140aa99e0`（update_freehand_ty，写 WallState+0x60）+ `FUN_140aa9eb0`
   （update_arch_type，+0x64）。行字节@0x11=freehand_ty、@0x12=arch_type 标志。
2. reader2：`FUN_140aaa530` = **_try_update_state<update_style> 实例**；行字节@0x18=标志。
3. reader3：`FUN_140aaa350` = **_try_update_state<update_plaster_peel> 实例**。

**CompoundEditBuilder 历史（0x140a82b40 ×4）**，niche 变体：
`-0x7ffffffffffffff6`（0x800000000000000A → **变体 11**）freehand_ty、
`-0x7ffffffffffffff9`（→ **8**）arch_type、`-0x7ffffffffffffff7`（→ **10**）style、
`-0x7ffffffffffffff8`（→ **9**）plaster_peel；`FUN_141696f50`（finish）×3、`FUN_140a84ec0`（commit）×3。

**样式改变后音频**（循环收尾，末次事件墙）：PublicWalls 查表（0x2a8 步长）：退化 → panic
`w.simplified_curve_2d.is_valid()`（**file set_wall_style.rs** 0x142abc958，行 50）；曲线 u 插值
`FUN_141467ce0` → 段索引+t，12B 点 `{x@0,y@4,z@8}` 三通道 lerp，越界 → FUN_1428d9518；
写 **RequestWallMaterialAudio**：param_7 EventWriter，**0x20B 行**
`{wall_id u64@0, u8 tag=0x10@8, u8 flag@9, {f32,f32} pos@0x10, f32 z@0x18}`。

### 6.8 upload_instanced_wall_ssbo 0x14082c170（O，16KB，PDB 名 gpu_flush_brick_buffers）

签名（3 参）：param_2 = ResMut<AssetSsboLibrary>（+0x1c 回写）；param_3 = Res<InstancedWalls>。

记录布局（param_3 deref）：id **连续区间** `{start u32*@+0xf8, end@+0x100}`（非 cap-first Vec）；
**compact 标志 u8@+0x108**（双路径）；**frame 索引 @+0x110**（每实例帧槽）；帧号 u32 = `*(param_3+0x14)`。
二级存储（param_3[1]）：每墙 0xa0B @ `+0x100`（+0x58=计数、+0x50=脏索引表、+0x94=帧索引）、
每帧 0x48B @ `+0x1a8`（+0x10=计数、+0x18=ptr、+0x38=帧槽指针数组）。

1. **get_mut<BrickTransformSsbo>**（FUN_1412c71b0）：None → panic_str（UNK_142a92100）+ trap。
2. **双路径**（@+0x108）：
   - 非 compact：逐 id：`*(storage+0x100)+id*0xa0` 项计数@+0x58==0 跳过；帧槽 idx=@+0x94；
     `src=*(*(0x48项+0x38)+frame*8)`、dst ptr（+0x10/+0x28 区）；遍历**脏索引表**（@+0x50，0x10 步长，
     u32 idx@+8）：`*(dst+idx*4)=frame`；`FUN_140858880(idx*0x30+src, ssbo)` 拷 **0x30B 砖**。
   - compact：逐 id：计数@0x48项+0x10==0 跳过；同帧槽取址；**顺序拷全部实例**
     `FUN_140858880(i*0x30+src, ssbo)`。
3. **gpu_flush**：`FUN_14161ab00(ssbo+0x68, …)`（SparseInstanceBuffer 域）+ puffin scope
   （0x142c2e9f0 len 0x50、0x142c2eb10 len 0x38）+ Commands FUN_140d79f90 +
   诊断 fmt panic `FUN_1428d9760(&UNK_142c2e7f0, 0x37, …)`。

### 6.9 ui_continue_wall 0x140aadd30（O，57KB 结构级）+ suppress_continue_wall 0x140aadbb0

**ui_continue_wall**（16+ 参）。ResMut/EventWriter（+0x1c）：param_5/6/7/14/16 等多处内联。
常量：**0.7f @0x1429cd030**、**0.1f @0x142925984**、**0.5f @0x142925870**（预览高亮色/透明度）。

1. **逐已发布墙查端点**：遍历 PublicWalls（**0x2a8 步长**，尾→头）：id=条目 key；退化 → panic
   `w.simplified_curve_2d.is_valid()`（**file ui_continue_wall.rs** 0x142ade058，行 50）；
   条目内 u8 标志（entry base -9）+ 第二查表（FUN_1405b0d70）筛选；InnerWalls 查表（0x78 步长）
   读墙曲线点（拷贝）。
2. **光标/端点判定**：query `FUN_1409231f0`（字节@[3]、ptr@[7]、int@[5]、+0x19 标志）；距端点近 → 续画。
3. **signifier 生成**：`Commands::spawn` ×2（FUN_140d79f90），signifier **`continue_wall_`**
   （0x142ade0c8/0x142ade0e8），display 回调（FUN_140b34ec0/FUN_140d8c890、&UNK_140abceb0/FUN_140d4ba00）；
   预览色 0.7f vs 0.5f。
4. **转发**：`FUN_14146c880`（RecordMouseInputIntoWall 域）带墙/曲线数据。
5. **状态机**（jump table 0x142ade214/0x142ade234）：case1 写 **WallContinueState active 字节
   （*ppppppplStack_d0 = 1）** + 事件写（多 EventWriter 8B 行）；case2/3 续。
6. `FUN_1413e7230` ×2、`FUN_140c9a630`（GladeBorder::is_pos_inside 域）×3、`FUN_140d79f90`（Commands）×2。
7. **GameTime panic**：`Resource does not exist` + `country_core::resources::time::GameTimeResource
   requested by … does not exist`（0x142ade244/254/277）。

**PDB 名错位**：decomp 头把 0x140aadbb0 命名为 `WallContinueState::finish`（模块 ui_continue_wall.rs），
DIGEST/任务称 `suppress_continue_wall`；同地址。

**suppress_continue_wall 0x140aadbb0**（O，2.9KB）：4 参：param_1 = Res<修饰键>（字节@0 != 0 → 激活）；
param_2 = ResMut<WallContinueState 记录> `{f32 time@0, f32 dist@4, u8 active@8}`（+0x1c 回写）；
param_3 = Res<GameTime>（**delta_secs @+0x28**）；param_4 = EventReader<MouseMotion>（16B 行，
**u64 打包 `{dx f32, dy f32}` @+8**）。若 active：`time += delta_secs`；逐事件 `dist += sqrt(dx²+dy²)`；
若 `time >= 1.0`（0x142925904）或 **`dist >= 50.0`（0x1429cd070 = 0x42480000，本轮新定值）** →
`active=0` 且双累加器清零；修饰键按下 → `active=1`。

### 6.10 ui_draw_wall 0x140aafe40（O，19KB）

签名（16 参）。**模式门**：param_1 = Res<AppMode>；`tag==0x12`（fence 画线）或
`(tag==6 && 字节@+4==0)`（wall 画线）才继续，否则 return。signifier：**`fences`**（0x142ade145，
len 6）与 **`wall`**（0x142ade14b，len 4）。
**WallDrawState**（param_2 deref，ResMut）：`state u8@0`、`u64@+0x10`（清零）、`bool is_fence@+0x18=
(tag==0x12)`；param_3 = 另一 ResMut 记录（+0x10/+0x18 同域写）。
游标结构（query `FUN_14092df50`，param_8 Query + param_16 组件）：`state u8@+0x18`、+0x19 字节、
`int@+0x28`、`signifier ptr@+0x38`、ptr@+0x90。param_6 = Res<射线命中> `{present u8@0, pos…}`。

1. **绘制激活**（state@+0x18 != 0）且 signifier 匹配且非擦除类（+0x28 != 2）：
   - `FUN_1409e9640`（TerrainHeightsData 采样，param_12 deref）；**高度 < 0.2f（0x1429cd068）→
     water hint**：`func_0x000140c9cde0` 投影位置，构 0x30B 束 `{1,1,{x,y}打包,z,距离 SQRT…}`，
     **Commands::spawn**（FUN_140d79f90，signifier `water_hint` 经胖指针 0x142ade160、display 回调
     FUN_140d4b7d0），`FUN_14146c880`（**RecordMouseInputIntoWall 转发路径**，param_13 deref）。
2. **状态机**（`cVar22 = min(游标state-2, 3)` → jump table 0x142ade224）：
   - case1 起笔：按下且 `FUN_140c9a630`（GladeBorder 内）→ WallDrawState=1；否则 push 8B 事件
     （param_10 EventWriter）+ 间接分发。
   - case2/3：状态迁移/设 3；case4/7：位旋转（模式标志）。
   - 共享 switch（0x142ade234）：
     - case1：`FUN_1413e7230`（param_16 事件写）+ param_4 push 8B + WallDrawState 记录写（param_3）。
     - case2：param_4 push 8B；temp 曲线（param_15 deref）非空 → param_14 push **0x18B 行**
       `{u64@0, wall_id@8, u8@0x10=0}`。
     - **case3 收尾**：param_5（EventWriter<FinishWall>）push **16B 行 `{u64@0, u8=1@8}`**；
       param_7 push 8B；WallDrawState=0；param_9 `byte@+8=1`；param_14 push 0x18B 行。
3. **观察到 9 个 +0x1c tick 回写槽**：param_2/3/4/5/7/9/10/14/16（param_4/16 为 EventWriter，
   param_2/3/9 等为 ResMut）。**纠错/补充**：DIGEST「5 ResMut（param_5/7/9/10/14）」之外，
   param_2（WallDrawState）、param_3、param_4、param_16 亦有回写。

## 7. resources/walls/ 资源算法（O，W6 已回填）

### 7.1 WallState（0x70）/ InnerWalls（O 布局）
- WallState 0x70：enclosure 记录@+0x38（tag u32 <2=premade）、height f32@+0x54、terrain floor@+0x58、
  freehand_ty u8@+0x60、arch_type u8@+0x64、removed_supports u8@+0x67、双 Vec 曲线（ptr@+0x8/len@+0x10 等）。
  serde 13 字段名表 `curve,height,is_enclosed,elevation,arch_height,arch_ty,style,structure_ty,clutter_version,
  plaster_peel,removed_supports,is_valid_enclosure,has_skirts`。
- InnerWalls = hashbrown map（条目 0x78 = {WallId u64, WallState 0x70}）+ **+0xc0 脏代计数**
  （wall_ids 每调 `+0xc0 += 1`）+ +0x30 工厂子结构（new_wall_with_id 尾调 FUN_1402ca2f0(self+0x30, id)）。
- `_try_update_state<闭包>` 族 0x140aa9800..0x140aaaca0：查墙（map 0x78 步长）→ 写 WallState 单字段 →
  脏代递增；更新失败静默。额外实例化 0x140aaaac0（elevation）、0x140aa9610（enclosure）、0x140aa9bc0（bulldoze）。
- 0x14085a730 = `remove_all`（round3 修正：清 map 头 4 qword + **128 位占用位图写 0**，PDB 名 max_id 错位）。

### 7.2 PublicWallState（0x2a0，O）
- raw Curve2 点 ptr@+0x8 / len@+0x10 / length f32@+0x30；3D 曲线@+0x88（FUN_141469e00 投影）；
  高度数组 f32 ptr@+0xe0 / len@+0xe8；max_y 缓存 @+0x108 位 + +0x10c f32；gable 缓存 @+0x110/+0x114；
  形状记录@+0x258（28B，tag u32<2）；published 长度 f32@+0x28c；style@+0x299。
- **get_wall_attachment_height** 0x140b2a890 全流程 O：max_y 缓存缺失 panic → premade（tag<2）→ 直返 +0x10c；
  Freehand → `u×raw_len` → NaN/inf panic（`!u.is_infinite()`）→ clamp01 → Curve2 段插值
  （FUN_141467ce0）→ 3D 投影（FUN_141469e00 self+0x88）→ 高度数组插值。
- `wall_space` = {tag!=2, f32@+0x28c}（Option<f32> 发布长度）；`u_space` = {tag!=2, 1.0f}（归一全区间）。

### 7.3 MaterialStats（O，CAS 原子族；W6 已回填纠错）
- prev 特征长度 7×f32 @+0x28..+0x40 / cur @+0x90..+0xa8（plain/lintels/arches/?/gates/stairs/sewers）；
  8 段/柱计数 u32@+0xac..+0xc8 + seed u64@+0x88；pillar placements Vec@+0x70{cap,ptr,len} + 自旋锁@+0x68。
- add_*_length：**CAS cmpxchg 重试循环**（加本特征、扣 plain@+0x90 或 lintels 侧@+0x9c 预算）。
- **which_material_changed_the_most 0x140a887d0（W6 纠错）**：7 对 |cur-prev| argmax（索引 0..6），
  plain 带 **-0.01 bias**（`0xbc23d70a @0x142ad8a14`），前 5 对增量之和 >0 门控；**索引 5=sewers / 6=stairs**
  （DIGEST 序反，纠错登记）。
- **init_this_update 0x140a88280（W6 纠错）**：**无 cur→prev 拷贝循环**，只重置 cur + 注入 plain/lintels 预算。
- set_pillar_placements 0x140a88320：LOCK bts/btr 自旋 + **futex 式等待队列（哈希 + 1000000000ns 超时，
  全局等待队列 plRam_1436308f0 唤醒，与 DeferredDecoratorOpEvents::send 0x1408336b0 同机制）** + 换入新 Vec。

### 7.4 DecoratorStorage（O，W6 已回填布局）
- **5 集合布局敲定**：`per_wall` map@+0x0（**0x98B 条目 {WallId, Decorators 0x90B}**）+ `+0x30` removed/注册表
  （terrain/stair 懒初始化经常数键 1/2 探槽）+ `+0x60..0x90` **decorator_to_storage 索引
  （24B 条目 {u32 id, u64 判别 0/1/2, u64 payload}**）+ 内联 terrain@+0x90 / stairs@+0x120 +
  可写标志 **immutable@+0x1b0**（`assertion failed: !self.immutable` 门控写）。
- `get_or_insert_default` 0x140a1cf30：+0x1b0 门控 + 0x98B 步长 per_wall 探针 + 地形/楼梯内联 Decorators
  懒初始化 + 常数键 1/2。
- `lookup_decorator_info` 0x140a1c930：+0x78 索引 24B 条目 → 墙 map / 地形+0x90 / 楼梯+0x120 →
  Decorators+0x30 内嵌 0x50B DecoratorInfo map（二级查）。
- `on_walls_merged`（merge_with 0x14155b170）/ `on_wall_split` 0x140a1f590（0x28B 记录 +
  approximate_curve_u(split_u) clamp[0,1] + {low,high} f32 区间搜索分侧 + 默认 Arc 克隆插入 + 标 removed）
  / `take_snapshot`（5 集合空断言 + 16B XOR 掩码 0x1429cd080/90 + 旋转 key 解码）/ `apply_decorator_remove`
  全链（map.remove 断言 + d2s update panic 串）。
- Decorators（单墙集合）：removed_ids Vec@+0x0、modified@+0x60、键值集合 update 0x140a1b5a0
  （hashbrown 0x18B 条目，miss → panic `d2s update: no such ` + decorator_storage.rs）。

### 7.5 WallHoles / WallHoleStorage（O，W6 已回填）
- WallHoleStorage 0x68B；RawWallHole 20B / FinalizedWallHole 20B（{Aabb2 16B, kind, origin}）。
- `finalize` 0x140f87300：phase==1 → panic；raw→Aabb2（FUN_14123fff0）、maxX 排序、**闭合墙跨 seam
  副本（±wall_length，origin 0/1）**；panic `finalize called on a wall hole storage that has not been
  cleared yet.`（**要求前帧已 clear**）。
- **InProgressHoleStorage checksum `{sum, xor}@+0x38/+0x40`**（add 0x140f87860 指纹 FUN_14145b910 累加）。
- `clear_for_next_frame` 0x140f867c0：逐墙 advance_frame + finalized 迁入 PrevWallHoles（hashbrown insert
  + 0x14 步长 memmove）。
- FinalizedWallHoles：holes Vec@0..0x18 + wall_length f32@+0x18 + closed@+0x1c；
  iter_holes_with_padding（closed 时 padding 判定跨界副本，normalize_holes 闭包缩放）。

### 7.6 DecoratorArchivist / DecoratorType 判别族（O）
- DecoratorType 位运算谓词族（0x38..0x3f 值域）：is_window {0x38,0x39,0x3a}、spawns_clutter {0x38,0x39}、
  can_place_on_flat_roof t≠0x3a、can_place_on_stair_top {0x3c,0x3d,0x3e} 或非窗、can_place_on_terrain
  t∉{0x38..0x3b}、should_offer_snap_to_corner t≠0x3f、can_change_seed {0x38,0x39,0x3a,0x3d}。
- decorator_cull_allow_any 0x140b2bc00：0x98 步长墙 map 遍历 + FUN_140b76260 回调 + +0x1e0 置位。
- **0x140b2bb00 = all_decorators（W6 纠错，非 validate_param）**：pmovmskb 非零槽位图描述符遍历五集合；
  validate_param 真身 = 0x1403f7af0。
- extract_diff 三断言（wall_decorators.is_empty / terrain.is_none / stair_decorators.is_none）+
  archive_storage_decorators 两 fmt panic 原文（见 W6 章节）。

### 7.7 其余资源（O 手解）
- wall_trigger_volume：quad 数组 {ptr@+8,len@+0x10}、元素 0x60B；get_tris 0x1408e80f0 quad→tri 0x48B 输出
  （panic `!triangles.is_empty()`）；recompute_quads 0x1408eae40（uvs/normals）。
- WallId::is_valid 0x140b34f20（u64!=0）；get_inner 0x140885d0（mov rax,[rcx]；**B patched 特有地址**，
  rne-read 原二进制该址不可映射——纠错记录）。
- split_height_into_skirts 0x140b34f80：>=5.9 起 4.5 每 +3.5，条件 height>f+1.4。
- DormerHoleSsbo：gothic 0x141286680 / cottage 0x1412866e0（0x30B 构造，尾区 4×f32 vs 2×f32+f64）。

---

## 8. 常量表（本批新增，O）

| 地址 | 位型 | 值 | 含义 |
|---|---|---|---|
| 0x142925870 | 0x3f000000 | 0.5f | 网格半宽/半 cell（世界原点中心） |
| 0x1429258e0/8e4 | 0x3f000000×2 | 0.5f,0.5f | cell 半对角线参与矩形查询 |
| 0x14292e6b0/6b4 | 0x3f800000×2 | 1.0f,1.0f | 归一化系数 |
| 0x14292e6f4 | 0x3fb504f3 | ≈1.414f (√2) | 圆查询半径边距 = cell_size×0.5×√2 |
| 0x1429a49bc | 0x4effffff | ≈2.147e9 | 网格坐标饱和界（超→0x7fffffff） |
| 0x142a171f0/1f4 | 0xbf000000 | -1.0f,-1.0f | 网格 min 角归一偏移 |
| 0x142a8cc60 | 0x40333333 | 2.8f | is_default_arch_height 比较常量（B 内 disp 偏移错位，见 decomp-0x1408c5690） |
| 0x142925904 | 0x3f800000 | 1.0f | u_space 归一全区间 |
| 0x142ad8a14 | 0xbc23d70a | -0.01f | which_material_changed_the_most plain bias |
| 0x1429cd2d4 | 0x41400000 | 12.0f | circle/rectangle create 常量（点数/长度候选，待 W2） |
| 0x142aa50dc / 0x142aa5088 | 0x40a80000 / 0x414c0000 | 5.25f / 12.75f | create_rectangle 角/尺寸常量（待 W2） |
| 0x1429df180 | 0xbfc00000 | -1.5f | create_rectangle 常量（待 W2） |
| 0x1429cd078 | 0x40400000 | 3.0f | create_rectangle 常量（待 W2） |
| 0x142934690 | 0x40c00000 | 6.0f | create_rectangle 常量（待 W2） |
| 0x142925984 | 0x3dcccccd | 0.1f | create_rectangle 阈值（待 W2） |
| 0x1429cd068 | 0x3e4ccccd | 0.2f | animate_wall_bulldoze phase 除数/步进；ui_draw_wall water hint 地形高度阈值 |
| 0x142960674 | 0x3c23d70a | 0.01f | finish_edit_rectangle 双判据阈值（dims 未变 + 距离 SQRT） |
| 0x142b1d78c | 0x42652ee1 | ≈51.3f | finish_edit_rectangle 像素→世界缩放（×i32::MAX 掩码 0x14292e980） |
| 0x1429cd064 | 0x40200000 | 2.5f | publish 栈装配默认高度（空记录路径换 0x142a7de00） |
| 0x1429258d8 | 0x3e99999a | 0.3f | publish 栈装配默认值 |
| 0x14292c76c | 0x42c80000 | 100.0f | publish 常量 |
| 0x142934600 | 0x41c00000 | 24.0f | publish 常量（行 1123/1227/2154） |
| 0x142c6bf00..18 | — | 32B 静态默认记录 | publish 默认 PublicWallState 片段（首 f32 ≈99.37 @0x42c6bef0） |
| 0x1429a3c70 | 0x43020000 | 130.0f | WallSpatialHash extent（格阵 ±130 世界单位） |
| 0x3e8f5c29 | — | 0.28f | RESAMPLING（add_segment 段采样步进，`RESAMPLING < cell_size` 断言前提） |
| 0x142925970 | 0x3ecccccd | 0.4f | circle query hide_trees 变体：成员点到段距离阈值 |
| 0x142a7baa0 | 0x1e3ce508 | 2.5e-20 | 距离分母 epsilon（snapping_init 亦用） |
| 0x142925980 | 0x7f7fffff | f32::MAX | 距离初始化上限 |
| 0x142934674 | 0x40c90fdb | 2π | 圆采样/周长常量 |
| 0x142a7c880 | 0x3f666667 | 0.9f | 圆半径下限（ui_create_circle clamp） |
| 0x142934604 | 0x41000000 | 8.0f | 圆半径上限 / circle base 高度 |
| 0x142aa3280 / 0x142aa4550 | 0x3ea147ae / 0x3f2147ae | 0.315f / 0.63f | signifier 圆/矩形 expand 量 |
| 0x142aa5084 | 0xc0100000 | -2.25f | create_rectangle 脊长比例常量 |
| 0x1429d18cc | 0x42700000 | 60.0f | ui_create_circle 光标悬停距离钳 |
| 0x142aab4d4 / 0x1429cd074 | 0xc0c00000 / 0xc1000000 | -6.0f / -8.0f | shape_placement offset |
| 0x142b1d78c | 0x42652ee1 | 57.29578f | 度弧度系数（finish_edit_rectangle 朝向判据） |
| 0x1429edba4 | 0x40490fdb | π | orientation π |
| 0x142a8d5c4 | 0x7fc00000 | NaN | orientation NaN 防护 |
| 0x142925950 | 0x3e800000 | 0.25f | orientation 角度量化 |
| 0x142ae0d44..60 | — | acos 8 阶多项式系数 | ui_edit_rectangle_orientation |
| 0x1429cd070 | 0x42480000 | 50.0f | suppress_continue_wall 移动距离阈值（本轮新定值） |
| 0x1429cd030 | 0x3f333333 | 0.7f | ui_continue_wall 续画预览高亮色 |
| 0x142925870 | 0x3f000000 | 0.5f | ui_continue_wall 默认预览色（续区 0x3efc0fc1≈0.492） |
| 0x142ab6650 | 0x3dcccccd 0x3cf5c290 | 0.10f, 0.03f | wall_highlight ty==1 rgb（V1，重建一致） |
| 0x142ab6640 | 0x3d8f5c29 0x3da3d70b | 0.07f, 0.08f | wall_highlight ty!=1 rgb（V0） |
| 0x142a7e354 | 0x3d8f5c29 | 0.07f | wall_highlight b 通道（V0） |
| 0x142ade145 / 0x142ade14b | cstr | `fences` / `wall` | ui_draw_wall 画线模式 signifier（len 6/4） |
| 0x142ade160（fat） | — | `water_hint` | ui_draw_wall 水提示 signifier |
| 0x142ade0c8 / 0x142ade0e8 | fat | `continue_wall_` | ui_continue_wall signifier 前缀 |
| 0x142abb1c0/1c7/1ce | fat | `merge a`/`merge b`/`merge target` | merge_walls 事件 tag 串 |
| 0x142ad74bb | cstr | `assertion failed: pending_wall_history.wall_changed.replace(expand).is_none()` | finish_wall builder 字段名证据 |
| 0x142abb1da | cstr | `assertion failed: pending_wall_history.merge_walls.replace(PendingMergeWalls {…})` | merge_walls 历史结构+颜色字段证据 |

## 9. 状态机（本批，O/I）

| 状态机 | 状态 | 转移 | 证据 |
|---|---|---|---|
| SnappingMode | mode u8@0 ∈ {0=无,1=段,2=端,3=禁用}，snapped u8@1 | snapping_init → mode=1/0；to_ends→2；check_for_disable→3 | 0x140ade610/0x140ada600/0x140a6ad80/0x140a6a560 |
| SnappingMode +0x20 | 音频抑制状态字节（上次发声模式/计数） | mode==2 且==2 跳过 / mode==3 且>1 跳过 | 0x140a6a480 |
| WallHoleStorage | phase（0=WIP,1=finalized） | new_in_progress/new_finalized；as_wip_mut/as_finalized panic；clone_and_reopen→0 | 0x140f86de0..0x140f86f50 |
| MaterialStats | cur/prev 7 长度对 | init_this_update 帧初 cur→prev；add_*_length CAS 累加 | 0x140a88280/0x140a88450.. |
| AnimateWallBulldozeBuffer | contains（动画中墙） | animate_wall_bulldoze phase 推进 CubicInOut；完成移除 authority | 0x140b772b0/0x140ac27e0 |

## 10. 跨模块调用（本域向外）

| callee | 用途 | 目标域 |
|---|---|---|
| FUN_141467ce0 | Curve2 取段+分数（get_curve_segment_from_u/get_u_at_coord） | utils::curve |
| FUN_141469e00 | 2D→3D 曲线投影 | utils::curve |
| FUN_141469a20 | approximate_curve_u | utils::curve |
| FUN_141469950 / FUN_1414671d0 | 曲线几何（shape_placement） | utils::curve |
| FUN_1409e9640 | TerrainHeightsData 采样 | terrain 域 |
| FUN_140969720 | SparseInstanceBuffer::allocate（2 静态类型 id） | render/gpu 域 |
| FUN_140b772b0 | AnimateWallBulldozeBuffer::contains | eraser 域（本域） |
| 0x140a923a0 | PublicWalls::roof_entity → Roof 高度参与 clamp | roof 域 |
| 0x1408c56b0 get_enclosure_params / tag<2 | 围合形状参数 | 本域 |
| 0x140aa99e0 / 0x140aa9eb0 | update_freehand_ty / update_arch_type | resources/walls/inner_wall_state.rs |
| 0x140aaa530 / 0x140aaa350 | _try_update_state<update_style / update_plaster_peel> 实例 | resources/walls/inner_wall_state.rs |
| 0x14085aa60 / 0x14085a560 / 0x1402ca2f0 / 0x141529bb0 | InnerWalls 采样/get/wall_ids/factory | resources/walls/inner_walls.rs |
| 0x1408dc9f0 | history_record_wall_changed | systems/wall/deferred_wall_history.rs |
| 0x140a82b40 / 0x140a84ec0 / 0x141696f50 | CompoundEditBuilder push/commit/finish | resources/history |
| 0x140b2bc00 / 0x140a1ea40 | decorator_cull_allow_any / DecoratorStorage::on_walls_merged | decorator_archivist / decorator_storage |
| 0x140c9a630 / 0x14092df50 / 0x140c980f0 | GladeBorder::is_pos_inside / 光标查询 / 鼠标输入域 | ui-input / terrain-water 域 |
| 0x140d14770 / 0x14146c880 | Commands::despawn / RecordMouseInputIntoWall 域 | bevy Commands / ui-input 域 |
| 0x1412c71b0 / 0x14161ab00 / 0x140858880 | SSBO get_mut<BrickTransformSsbo> / gpu_flush / 拷砖 0x30B | render/gpu 域 |
| 0x140b81f90 | SmallVec grow（bend 方向小缓冲） | utils/std 胶水 |
