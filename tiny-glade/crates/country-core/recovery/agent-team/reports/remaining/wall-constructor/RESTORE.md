# wall-constructor 剩余族还原报告（restorer-remaining-wallconstructor）

日期：2026-10-17。范围：`crates/systems/wall-constructor/src/`（package
`system-wall-constructor`）。标准：逐函数 fresh 反编译/机器码 → 重建真实控制流
→ 逐语句判等；还原不了如实标未实现（分歧点），近似体一律移除为 `todo!`。

工具：ghidra `--worker shared` 只读（`get_functions`/`disassemble_function`/
`read_memory`/`get_xrefs_to`）。证据落盘：
`work/restorer-remaining-wallconstructor/evidence/`（本会话 fresh：
`floor-plaster.result.json`、`elev.result.json`、`elevsys-*.c`、`fsy-dis.txt`、
`fsy-mem.result.json`、`fsy-const.result.json`、`abb-dis.result.json`、
`arch-cal.result.json`、`pc-xref.result.json`；复用 re-analysis
hardpoints-interaction 的 0x140178a70/0x140496800 fresh 反编译）。

---

## 1. 本次成果

### ✅ 重建（1）：`find_stone_column_start_y`（0x141250720）

`construct_elevation_supports/rectangle_pillars.rs`。fresh **105 条指令**全量
反汇编 + 跳表 `0x142c20af8`（17 项）+ 常量逐字节读取，逐语句重建：

- `RaycastWorld::raycast_simple_exclude_wall_id(position, (-0.0, -1.0, -0.0),
  0x1202, excluded)`（XMM3 = `f32::MAX`；`0x1408ccc50` = `Rapier::raycast_simple`
  尾转发）。
- 命中判别 `hit.handle[0] as u8`（`RaycastSimpleResult`：handle 24B@0、
  position@0x18、toi@0x24）：
  - `0x11`（未命中）→ `terrain.sample_at_world_xz(x,z)` + `xvy` 写回 hit 位置
    （结果随后被覆盖；调用保留）。
  - `0x01` → `None`；`0x02` → `Some(hit.position.y + (handle[0] bit8 ? -0.2 : -1.0))`
    （常量 `0x1429df16c=-0.2`、`0x14295a0e4=-1.0`；`TEST byte[handle+1],1`）；
    `0x0d` → `Some(hit.position.y - 0.2)`。
  - `0x00,0x06..=0x0b,0x0e` → 继续；`0x03,0x04,0x05,0x0c,0x0f,0x10` → panic
    （跳表 panic 桩）；`0x12+` 越界。
- 继续段：`axis = Option<Rectangle2d>（tag u32@0, payload@4）`：
  `None → (1.0,0.0)`，`Some → rect.axis`；`Rectangle2d::new(center=(x,z),
  size=(0.7,0.7), axis)`（常量 `0x1429cd030 = 0.7`）→ `as_points2()` 4 角 →
  逐点地形采样 → `minss(s0,1000.0)` 链（`0x14295b630 = 1000.0`）→
  `Some(min)`。`MINSS` 的 unordered 语义（返回第二操作数）用 `minss` 显式保留。

**登记（utils 合同）**：`Rectangle2d::new` 机器实参顺序为
（size.x, size.y, axis.x, axis.y, center.x, center.y），而 utils 现行签名为
`new(center, size, axis)`；本次按现行 Rust 签名调用（构造出的
`{axis,center,size}` 字段正确），若 utils 后续修正参数顺序需同步本调用点。

### ✅ 重建（2）：`circle_categorize`（0x14124af60）

`construct_elevation_supports/circle_categorize.rs`。fresh **197 条指令 /
837B** 全量反汇编 + 8 个常量逐字节（`evidence/cc-dis.result.json`、
`cc-const.result.json`；**更正**：本报告首版误写「105 条」，105 是
find_stone_column_start_y 的条数）。逐语句重建：

- `radius < 1.75` → `return walls.is_empty()`（`SETZ AL` on len==0；
  `UCOMISS 1.75,radius` 的 unordered 也走主路径，Rust 相同）。
- `t = MINSS(1.0, MAXSS(0.0, (radius − 1.75)/1.25))`（常量
  `1.75`/`-1.75`/`1.25`；**FIX-G1**：手写 `maxss/minss` 保留 NaN 传播，
  不用 `f32::max/min`）；`s = t²(3 − 2t)`（`3.0`）；
  `threshold = s*0.6 + (1−s)*0.3`。
- `n = ceil(2r) as usize`（饱和转换）；`step = 2r / n`；**双层 `0..=n`
  闭区间**网格 `(center − r) + i*step`。
- `circle.is_point_inside(p)` 命中 → `inside += 1`；逐 0x50B 墙条目
  （wall id @+0x38）→ `PublicWalls::get`（缺失 **unwrap panic**）→
  `get_wall_shape()` → `shape_contains(shape, p)`（0x140ad2950 语义）→
  命中则 `in_wall += 1` 并 break。
- `return UCOMISS(ratio, threshold) + SETC`：`ratio < threshold` **或
  unordered（任一 NaN）→ true**（FIX-G1；Rust 裸 `<` 对 NaN 返回 false）。
- `shape_contains`：`WallShapeSnapshot` 28B 字节即 `BorderShape` 内存像
  （tag u32@0，bit0==0 → `Circle2d{center@4,radius@0xc,rotation@0x10}`；
  否则 `Rectangle2d{axis@4,center@0xc,size@0x14}`），调用 core 的
  `contains_point`（未复制几何公式）。
- 单测 3 个（低半径早退 / NaN radius 的 MAXSS/MINSS+UCOMISS 语义 /
  circle+rect 形状字节解释）通过。

**登记（core-api-missing，非阻塞）**：建议 core 提供
`WallShapeSnapshot::contains_point(Vec2)`（或 `as_border_shape()`），
落库后本文件的本地 28B 解释可删除。机器实参面（RCX=&Circle2d、
R8=墙条目容器 +8/+0x10、R9=&PublicWalls）已写入函数文档。

### 🧹 近似体/空壳降级（11 个文件 / 17 个 `todo!`）

按协议移除近似体（原实现无机器码依据或只覆盖部分子链）：

| 文件 | 函数 | 原近似体问题 |
|---|---|---|
| `construct_elevation_supports/circle_pillars.rs` | construct_circle_pillars 0x14124ba30 | 缺 0x298 半木构门控/三向 raycast 净空/-0.4 外移/0x50B 实体校验 |
| `.../circle_turret.rs` | construct_turret 0x14124cc30 | 层数/收分率/逐层计数无证据 |
| `.../circle_bottom_layer.rs` | add_bottom_brick 0x141fe1de0 / construct_circle_bottom_layer 0x14124b2c0 | 11 参 ABI/random_splits/get_pos_at_u/BufferState 锁链 |
| `.../rectangle_bottom_layer.rs` | construct_rect_bottom_layer 0x14124eb80 | halftimber 四角/5 段边二路、0.69 计数、0.32 jitter |
| `.../rectangle_small_brackets.rs` | construct_rectangle_small_brackets 0x141250da0 | 1137 行托架几何未逐语句 |
| `.../rectangle_large_brackets.rs` | construct_rectangle_large_brackets 0x141253fc0 | 角部斜撑+横梁几何未定 |
| `.../util_pillar_construction.rs` | wooden_pillar / brick_columns / crossbrick_pillar | 1.1 钳/1.25 石基/0.3 斜撑/count=ceil((top-bottom)/(size*0.4)) 等未逐语句 |
| `.../find_overlap_segments.rs` | find_overhang_support_segments 0x141248620 | 43KB 输出结构/阈值/多 closure 未定 |
| `.../maintain_stone_column_colliders.rs` | maintain_stone_column_colliders 0x14078a6e0 | rapier 碰撞体撤除面未暴露（core-api-missing） |
| `.../rectangle_pillars.rs` | construct_rectangle_pillars 0x14124fcb0 | 4 角采样/3 向 raycast/0x40 提案/Rectangle3d 输出链未逐语句 |
| `.../mod.rs` | construct_circle_supports 0x140788160 / construct_rectangle_columns 0x140788a80 / construct_rectangle_brackets 0x1407896b0 / detect_overhang_support_segments 0x1407867d0 | 空壳/假循环已改为 `todo!` + 已解结构登记（三系统注册在 `WallCpuStage::Bricks`，未完成路径运行时将 panic——与 crate 内既有 todo 系统一致） |
| `.../mod.rs` | overhang_subscribe_to_tabloids 0x140786440 | **无 body 的诚实空壳 `{}`**（原 8×`RaycastWorld::subscribe_to_changes` + 1×tabloid subscribe 胶水；render/宿主装配层未接线），未改为 todo（不作为业务实现，仅登记） |

---

## 2. 优先项判等/登记

### 2.1 pillars 系统步骤 4-6（0x140178a70）——**登记：core-type-gap**

core 类型逐项核实（2026-10-17）：

| 类型 | 状态 | 位置 |
|---|---|---|
| `RtWorld` | ✅ 已落 | `resources/rt.rs`（get_mut/alloc_prefab_handle/三缓冲） |
| `RtMeshHandle` | ✅ 已落 | `resources/rt.rs`（16B {tag,payload}） |
| `StairGraphEdge` | ✅ 已落 | `resources/stairs/stairs_state.rs`（u32 对 min/max） |
| `StairColliderRegistry` | ❌ **缺** | 全 workspace 无（`stairs_collider.rs` 仅标记资源 `StairsCollider`） |
| `UpdateRtWorldEvent` | ❌ **缺** | 全 workspace 无（`resources/events.rs` 未注册） |

**解锁条件**：core 落 `StairColliderRegistry`（`HashMap<StairGraphEdge,
Vec<Vec3>>` 语义：`hash_one` 探测→命中直用/未命中 reserve_rehash 插入空 Vec）
与 `UpdateRtWorldEvent`（Event 载荷）；随后按宿主 47KB fresh 反编译实现
步骤 4-6（`Rectangle3d::new(size={(size_x−size_z)+0.3, +0x18−0.5})` +
`box_trigger_volume` + closure spec_extend<Vec3> → `RtWorld::get_mut<
RtMeshHandle>` 追加 collider 点 → `EventWriter::write<UpdateRtWorldEvent>`）。
步骤 2 合并块已判等（`merge_nearby_proposals`，本会话复核测试 3/3 通过）。

### 2.2 `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）——**登记：内部前置未完成**

fresh 97KB 反编译 + `callees` 泛型名逐字复核，9 个 closure 单态坐实
（写入 `construct_playermade_stairs/mod.rs` 模块文档）：

| env | 模板 |
|---|---|
| `$1<false>` | `Map<Iter<Vec2>, f32>` |
| `$2<false>` | `Map<Iter<(usize, StairArchType)>, (Option<f32>, usize)>` |
| `$3<true>` | `Map<Zip<Iter<Vec2>, Iter<f32>>, Vec3>` |
| `$4<true>` | `Map<Enumerate<Iter<Vec3>>, f32>`（smoothstep/0.5 混合） |
| `$5<false>` | `Map<Iter<CurveU<3D>>, CurveU<Thresholded3D>>` |
| `$6<false>` | `ArchWalker::walk<closure_env$6<false>>`（0x14048ab50） |
| `$7<true>` | `Map<Enumerate<Iter<Vec2>>, Vec2>`（±0.56*0.5*dir） |
| `$8<false>` | `Map<Iter<f32>, f32>`（min 1e6 / max 0.0 扫描） |
| `$9<false>` | `Map<Range<usize>, [u32;3]>` |

主流程亦已复核（SSBO 注册表 BrickTransformSsbo 0x3a 名 memcmp → drain
StairsArchSupportAssemblyParams → StairGraphEdge RawTable 探测 →
`preprocess_curve` → closure$1..5 装配 → `ArchWalker::walk`（raycast
0x11/0x01/0x02 分支 + bilinear_sample_at_world_xz + StairsPillarProposals::add）
→ closure$7/8/9 + `WallConstructor::from_curve` → `SparseInstanceBufferWriter::
push<BrickTransformSsbo>`）。
**解锁条件**：`ArchWalker::walk` 4 单态真实逻辑（现为登记分歧的近似体）与
`WallConstructor::from_curve`（行→提案中段未恢复）先落地；否则本系统无法
逐语句装配。

### 2.3 `preprocess_curve`（0x140780e20）——**登记：未实现**

fresh 5.3KB 已解结构（写入 `construct_playermade_stairs/mod.rs`）：
Vec3→Vec2 map → `try_new_from_points`（Err 早退）；`step0 =
min(0.368/len, 0.2)`、flag 门控；两路模式（stack bool）；`min(2/fVar12,
fVar14)` 间距 + `[0,1]` 断言；`slice_into`/`try_resample` 段循环 +
`(usize,u8)` 记录。未闭合：`closure_0` 语义、两路位组合、输出结构字段类型。

### 2.4 `construct_floors`（0x1407661e0）——**登记：core-type-gap**

- **签名已更正**（Ghidra `get_xrefs_to` 的 `run_unsafe` 泛型名逐字）：
  `(Res<PublicWalls>, EventReader<OnWallChanged>,
  EventReader<OnWallDeleted>, Local<FloorStorage>, Commands)`；旧 skeleton
  参数顺序错误，已改。
- 已解控制流（写入 `construct_floors.rs`）：deleted → FloorStorage
  remove + `Entities::contains` + despawn；changed → 先 remove/despawn 旧
  实体，再 PublicWalls 探测（stride 0x2a8）+ `0 < height（-0x20）` +
  `kind（-0x48）< 2` 门控 → `get_wall_shape` → shape bit0==0 时
  `log2f(max(shape+0xc,1.0)*3.0)` → `NaniBundle<StoneFloorInstanceData>` +
  `Floor` insert（0xb8 命令载荷）→ 回写 FloorStorage。
- **缺**：`country_core::resources::walls::Floor` 组件（约 0x50B 载荷，
  全 workspace 无）。解锁条件：core 落该组件布局后按上述全序实现。
  旧近似体（spawn_empty）已移除。

### 2.5 plaster 族——**登记：未实现**

fresh 反编译（0x14075b810 5273B / 0x14075d460 1686B / 0x14075dc60 3130B /
0x1412269b0 71.6KB 泛型单态）已解结构写入 `plaster_systems.rs`：

- `maintain_plaster`：puffin scope → deferred buffer `retain` →
  `Map<EventIterator<OnWallChanged>, closure_env$0>` → 逐墙
  `time_continuous_lerp` peel（`|δ|≥0.01` 门）→ shape center/max_y/xvy →
  0x28B indexmap bucket（tag 9）→ `insert<WallId, PlasterPeelAmount>` →
  `Decorators::iter_removed_ids` → remove_entry+despawn →
  `from_iter<PublicWalls keys>` + `sort_by_key<closure_env$4>` → 再 drain
  WallPlasters → 逐墙 `StructureType::is_fence` → `holes_no_duplicates` →
  `generate_plaster_mesh_and_entities`。
- `create_plaster`：逐事件 PublicWalls 探测（缺失 `unwrap_failed`）→
  `WallHoles::as_finalized`（stride 0x68）→ `holes_no_duplicates` → 另一
  map（stride 0x20）→ `generate_plaster_mesh_and_entities`。
- `split_wall_into_sides` / `generate_plaster_mesh_and_entities`：4 侧
  WallSide 24B/0.1m 格与 mesh 拓扑/UV 未逐语句。

旧近似体（create 自造实体/delete 直删/recalc 空转）全部移除。

### 2.6 elevation supports 其余

`circle_categorize`（0x14124af60）**已重建**（见 §1，含 2 单测）。

`construct_circle_supports`（0x140788160）宿主链已解：事件 stride 0x28
（wall id @+0x18）→ `is_circle()` + 字段==1 → `StoneColumnCollisionStorage::
clear_and_get_mut` → `circle_categorize()`（false→`construct_turret`，
true→`construct_circle_bottom_layer` + 0x28B stats bucket `{0,4,0,wall_id,u13}`
+ `construct_circle_pillars`）→ `ElevationSupports` 0x20B 记录复制/去重。
**阻塞**：`ElevationSupports`/`ElevationSupportStats` 在 country-core 仍是
`Gap`（布局 U，`resources/walls/elevation_support_stats.rs`），0x28B/0x20B
记录字段无法落库；且 `construct_turret`/`construct_circle_bottom_layer`/
`construct_circle_pillars` 未实现。解锁条件：core 落 ElevationSupports
布局 + 三个被调函数重建。

---

## 3. 验证

- `cargo check -p system-wall-constructor` 通过（Finished；9 条 warning 均
  pre-existing，本批文件未新增告警）。
- `cargo test -p system-wall-constructor --lib` 通过（3/3：merge 块测试）。
- `cargo test -p system-wall-constructor` 的 **doctest 2 个失败为
  pre-existing**：`utils/sine_wave.rs` 第 44/70 行的 C 风格伪代码被当作
  Rust doctest 编译（`sinf`/`truncf` 未定义），与本批无关，未改。

## 4. 计数（本会话）

- **判等**：0 新增（复核既有 merge 块：测试 3/3；core 类型核实 3 有/2 缺）。
- **重建**：2（`find_stone_column_start_y`，105 指令逐句；
  `circle_categorize`，197 指令/837B + 8 常量逐句，3 单测；G1 NaN/∞ 修正已落）。
- **未实现/登记**：25 个函数条目（pillars 4-6、arches_n_walls、preprocess_curve、
  construct_floors、maintain_plaster、create_plaster、split_wall_into_sides、
  generate_plaster_mesh、construct_rectangle_pillars、construct_circle_pillars、
  construct_turret、add_bottom_brick、construct_circle_bottom_layer、
  construct_rect_bottom_layer、small/large brackets、wooden/brick/crossbrick
  pillars、find_overhang_support_segments、maintain_stone_column_colliders、
  construct_circle_supports、construct_rectangle_columns、
  construct_rectangle_brackets、detect_overhang_support_segments）。
- **近似体/空壳移除**：11 个文件 / 17 个 `todo!`（含 `overhang_subscribe_to_tabloids`
  诚实空壳单列；详 §1 表）。
- **glue**：0 新增。

## 5. 交接注意

- `Rectangle2d::new` 机器实参顺序（size,axis,center）与 utils 现行
  `new(center,size,axis)` 签名不一致（§1 登记）；本实现按现行 API 调用，
  utils 若修正需同步。
- pillars 4-6 只差 `StairColliderRegistry`/`UpdateRtWorldEvent` 两个 core 类型；
  建议 core 优先落这两个（RtWorld/RtMeshHandle/StairGraphEdge 已具备）。
- `construct_floors` 只差 `resources::walls::Floor` 组件布局。线索：
  `reconstruction/country-core/src/resources/walls/mod.rs` 的占位注明
  `Floor(pub Gap)` 实际 **≥0x28B**、`+0x8 bit0` 控制屋檐切线路径（WP6 登记）；
  本会话命令载荷显示 Floor 段约 0x50B（payload +0x58 起：wall id / shape
  标志 / 高度等）。core 落布局后即可实现。
- `circle_categorize` 已重建；建议 core 吸收本文件的
  `WallShapeSnapshot` 28B → `BorderShape` 解释为正式 API
  （`contains_point`/`as_border_shape`），避免各模块重复解释。
