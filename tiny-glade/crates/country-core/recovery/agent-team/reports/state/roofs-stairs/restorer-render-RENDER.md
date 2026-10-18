# RENDER — roofs/stairs + render 设备面剩余项（restorer-render，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`、`planning/2026-10-17-recovery-sop.md`。
> 证据：`recovery/agent-team/work/restorer-render-2026-10-17/`
> （jobs/results + `decomp/` + `disasm_0/1.txt`，全部 ghidra `--worker shared` fresh 只读，
> 2026-10-17 21:34–21:37）。
> 红线：只以 fresh 反编译/机器码为准；缺类型登记不编造；宏胶水不重建；未改
> `src/asset_libraries/ssbo_library.rs`、`src/geometry/**`。

## 一、逐项 verdict

| # | 项 | verdict | 落点 |
|---|---|---|---|
| 1 | `roof_uv_at_ws_pos` 0x1408e4040 / `roof_uv_and_point_at_ws_pos` 0x1408e4090 | **管线锚定 + 两级表 seam 落地；表实现登记** | `resources/roofs/roof_shape.rs` |
| 2 | `MagicSupports::clear` 0x140aa8210 | **落地**（泛型 T 承接；`BrickTransformSsbo` 元素登记） | `resources/stairs/visual.rs` |
| 3 | `reset_edge` 0x140aa8a60 / `force_rebuild` | **写面落地**（`reset_edge_with`/`force_rebuild_with`）+ 步骤 5 登记 | `resources/stairs/visual.rs` |
| 4 | `RoofTileSsbo` 存储形态 | **落地**（64 B 元素 + `SparseInstanceBuffer<Self>`） | `resources/roofs/roof_tile_ssbo.rs`、`render/sparse_instance_buffer.rs` |
| 5 | 14 个 build 闭包 | **登记**（域外 + render host 解锁条件） | 见 §五 |

## 二、fresh 证据与实现

### 1. roof_uv 两函数（0x1408e4040 / 0x1408e4090）

- `0x1408e4040`（74 B）：薄包装。调 `0x1408e4090`；`out[0] = (discriminant != 0)`；
  失败写 `out[1] = code`；成功写 `out+4 = uv (2×f32)`。
- `0x1408e4090`（1152 B）全链逐分支：
  1. 第一级表（`in_stack_00000030` = ctx）：`+0x30` data / `+0x38` mask / `+0x48`
     len；命中条件含 `len != 0`，entry stride **0x10** = `{GlobalId key @+0, handle u64 @+8}`；
  2. 槽值 `handle >> 32 == 0`（generation 0）→ `{1,0}`（码 **0**；与缺键同）；
  3. `world(in_stack_00000028)+0x3e10 == 0` → `{1,1}`（码 **1**）；
  4. 第二级表：`+0x3df8` data / `+0x3e00` mask，bucket stride **0x20**；键 = 第一级 handle，
     槽控制字节 `+0x18 != 0x11` 才命中；缺/空 → 码 **1**；
  5. `ColliderSet::get_mut_internal` → `None` → `expect_failed` panic；
     `index < arena.len(+0x10)` 且 `record(+0x20) == (generation | 1)`，否则
     `begin_panic`；
  6. `IsoTransform::from_translation_rotation` + `swizzlers::xvy`；方向
     `(0, −1, −0)`（`0xbf80000080000000` / `0x80000000`）、`max_toi = 0x7f7fffff`、solid=1；
  7. `rapier_hit_to_raycast_world_result` 之后：判别 `== 2`（未命中）或 shape cast
     失败 → `*out = 0x0301`（码 **3**）；判别 `& 1 == 0` → `{1,4}`（码 **4**）；
     成功 → uv=`extra[0..2]`、point=`position`。
- 实现：
  * `RoofUvFailure`（码 0/1/3/4，`code()` 逐值）；
  * `RoofUvColliderIndex` seam：`lookup_global_id` / `arena_present` / `lookup_collider`，
    文档锚定上述两级表算术；
  * `roof_uv_and_point_indexed` / `roof_uv_indexed`：按机器顺序执行 1–4 与结果映射；
  * `roof_uv_from_raycast_hit`：码 3/4/成功的唯一映射点；
  * 旧 `roof_uv_at_ws_pos` / `roof_uv_and_point_at_ws_pos` 保留**全槽射线回退**
    （无两级表实例；签名不变）。
- **登记（跨域）**：两级表类型 + `ColliderSet` 按句柄单选仍未落库。当前 fallback 以
  `RaycastWorld::raycast(ws_pos, −Y, u32::MAX)` 代替第 5–6 步（全槽 vs 单一 collider）。
  解锁：collision/rt 域实现 `RoofUvColliderIndex` + 按 collider 句柄的 shape cast。

### 2. MagicSupports::clear 0x140aa8210（219 指令）

- 机器型（逐指令，`disasm_0.txt`）：`+0x10 u64`、`+0x18 u32`、`+0x20/+0x28` 第一写面、
  `+0x30/+0x38` 第二写面、`+0x40 u8`。
- 每写面槽（懒初始化）：`Arc::downgrade(*(host+0x60))`（原子增 `[ArcInner+8]`）存 `+0x20`；
  `__rust_alloc(0x20,8)` 得 `{0,4,0,0}`（空 `Allocation`：Vec cap/ptr/len + RawMutex
  @+0x18）存 `+0x28`。
- clear 主体：LOCK `[alloc+0x18]` → 用 `host+0x20..0x58`（byte_map/active/data/dirty 的
  ptr+len）与分配指针拼 writer → `SparseInstanceBufferWriter::clear<BrickTransformSsbo>`
  ×2（先第一写面，再第二写面）→ `+0x10/+0x18/+0x40` 归零 → 逆序 unlock。
- 实现：`MagicSupports { count, count32, first: Option<MeshAtlasAllocation>,
  second: Option<MeshAtlasAllocation>, flag }`；`MeshAtlasAllocation`（类型擦除的
  `{Weak<waste>, Box<Allocation>}`，Drop 语义同 `SparseInstanceBufferAllocation`）；
  `clear<T: SparseSsbo>(&mut self, first, second)` 逐序执行。
- `SparseInstanceBuffer::writer_for(&Allocation)`：机器 writer 从**裸分配指针**构造的
  直接落点（起始 chunk = `byte_map.len()-1`）。
- **登记**：原版 `T = geometry::instanced_wall::BrickTransformSsbo`（元素/`StorageElement`
  本 crate 未落库；`src/geometry/**` 属他 agent）。解锁：geometry 域补该元素后把泛型
  实例化为 `MagicSupports::clear<BrickTransformSsbo>`。

### 3. reset_edge 0x140aa8a60 / force_rebuild

- fresh 主体顺序（`decomp/0x140aa8a60_reset_edge.c` + `disasm_1.txt`）：
  1. `hash_one<RandomState,&StairGraphEdge>` 探 edges（bucket stride 0x110；键 = 两 u32）；
     未命中 → `RawTable::reserve_rehash<tuple<…, StairsEdgeVisualState>>`；
  2. 第一写面槽懒初始化 → LOCK → writer → `clear<BrickTransformSsbo>`；
  3. `MagicSupports::clear()`；
  4. 第二写面槽懒初始化 → LOCK → writer → `clear<BrickTransformSsbo>`；
  5. `Vec<Entity>` despawn 循环（高 dword != 0）、清内部表/向量；以值返回 0x110+ B 状态。
- 实现：`reset_edge_with<T>(edge, first, second, magic)` 逐序 1–4；`force_rebuild_with`
  为同体内联别名。原 `reset_edge`/`force_rebuild` 保留存储 upsert（无宿主时的消费面）。
- **登记**：步骤 5 的实体表/render host 与 0x110 B 缓存字段（U）；`dirty` 置位仍无本函数
  机器证据（保留为消费侧标记，U）。

### 4. RoofTileSsbo 存储形态

- fresh `new_storage_buffer<RoofTileSsbo>` 0x1412e4a90：byte_map `memset 0x2000` + `[0x2000]=0x20`
  → **CHUNK_COUNT=0x2000**；空闲栈 0x2000 项降序 `0x1fff..0`；影子 **0x40000 元素**；
  active/dirty 各 0x80 字。
- `TypedStorageBuffer::new<RoofTileSsbo>` 0x1416191a0：`0x40000 < len` 越界 → **BUFFER_LEN=0x40000**。
- `push<RoofTileSsbo>` 0x141a3c070：槽 stride **0x40**、8 qword 整块拷贝 → **元素 64 B**。
- 布局（着色器 `RoofShingle` 偏移 + push qword 序一致）：
  `base_position Vec3 @0`、`current_height f32 @0xc`、`clipping_plane [u32;2] @0x10`、
  `scale_t [u32;2] @0x18`、`rotation_quat [u32;2] @0x20`、`roof_id u32 @0x28`、
  `velocity_rotation u32 @0x2c`、`animation_t f32 @0x30`、`seed i32 @0x34`、
  `is_circle i32 @0x38`、`is_main_roof_body i32 @0x3c`。
- 实现：`src/resources/roofs/roof_tile_ssbo.rs` 新文件（`StorageElement` NAME="RoofTileSsbo"、
  BUFFER_LEN=0x40000、`SparseSsbo` CHUNK_COUNT=0x2000、`SsboType::Storage =
  SparseInstanceBuffer<Self>`；原版完整类型名常量 0x35 B）；`roof_tiles_gravity.rs` 删除
  `ssbo_carrier!` Gap 载体，改用真实类型；`RoofTiles.allocation:
  Option<SparseInstanceBufferAllocation<RoofTileSsbo>>` + `draw_ranges()`（机器查询行
  `allocation → allocation_to_draw_multi_instanced`）。
- `render/sparse_instance_buffer.rs`：新增 `writer_for(&Allocation)`（原生裸分配构造面）与
  `MeshAtlasAllocation`；新增 2 个测试（detached allocation clear/drop）。

## 三、验证

- `cargo check -p country-core --lib`（在 `/Users/yuluo/Desktop/game/games_test/tiny-glade` 下）：
  **Finished, 0 error**（15 既有警告；期间并行 agent WIP 曾短暂打断，稳态复跑通过）。
- `cargo check -p country-core --lib --features native-physics`：**Finished, 0 error**。
- 全库单测 `cargo test -p country-core --lib`：**2048 tests / 2047 passed / 1 failed**。
  唯一失败 `components::drawable::tests::nani_bundle_layout_matches_write_components`
  （`components/drawable.rs:155`，left 80 vs right 72）属**他域并行 WIP**，非本组文件。
  > 协调者给出的基线 2024 passed 与本轮观测 2048 不一致：差值来自并行 agent 新增测试
  > 与本组新增（本组新增 10 个测试函数、净 +9：roof_uv 2、RoofTileSsbo 3、
  > MagicSupports/reset_edge 3（替换 1 个旧测试）、writer_for/detached 2）；工作树仅为
  > `/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core`，
  > 未触碰 `reconstruction/**` 或 `skeleton/B/**`。
- 本域过滤单测：
  - `cargo test -p country-core --lib roofs::` → **25 passed / 0 failed**
  - `cargo test -p country-core --lib stairs` → **26 passed / 0 failed**
  - `cargo test -p country-core --lib render::` → **396 passed / 0 failed**
  - `cargo test -p country-core --lib roof_tiles_gravity` → **8 passed / 0 failed**
  - `cargo test -p country-core --lib sparse_instance_buffer` → **17 passed / 0 failed**
    （含 geometry 的 1 个既有稀疏测试）
- 新增测试（10 个函数）：roof_uv 2；RoofTileSsbo 布局/常量/存储 3；
  MagicSupports 2 + reset_edge_with 1；writer_for/detached drop 2。

## 四、登记（含解锁条件）

| 项 | 证据 | 解锁条件 |
|---|---|---|
| `RoofUvColliderIndex` 两级表实现 | 0x1408e4090（±§二.1 偏移） | collision/rt 域补 `GlobalId→handle` + arena 两级表；`ColliderSet` 按句柄 shape cast |
| `BrickTransformSsbo` 元素 | 0x140aa8210/0x140aa8a60 的 `clear<BrickTransformSsbo>` | geometry 域补元素类型；`MagicSupports::clear::<BrickTransformSsbo>`/`reset_edge_with::<…>` 实例化 |
| reset_edge 步骤 5 | 0x140aa8a60 despawn 循环 + 0x110 B 返回结构 | `Vec<Entity>`/RtMesh/render host 接线 |
| `reset_edge` dirty | remove_edge +0x30 有据；本函数未见 | 消费侧脏字节取证 |
| `RoofTiles.allocation` 生产者 | 0x140a29b10 查询读 | `assemble_roof_tiles` GPU 装配（roof system 域） |

## 五、14 个 build 闭包（item 5）

- 清单（`reports/eco/walls/restorer-walls4-REBUILD4.md` §57）：`0x14165cf80`/
  `0x14155e6b0`/`0x140a65510`/`0x140b35650`/`0x1409ae3d0`/`0x1408de7e0`/`0x1408df6c0`/
  `0x1408dfa00`/`0x14155f620`/`0x14155f9b0`/`0x14083df50`/`0x141660600`/`0x141660b30`/
  `0x140957390`。
- **本轮不可落地（文件分工）**：闭包本体在 `src/systems/wall/shapes/**`，宿主
  `SignifierEntityBuilder::build`（0x14089dd50）在 `src/systems/ui/signifiers/`，均不在本
  agent 允许修改的 `resources/roofs/**`、`resources/stairs/**`、`render/sparse_instance_buffer.rs`、
  `systems/render_loop/**`。
- **解锁条件**（fresh 已核实 `build` 当前只 spawn + `SignifierAnimation`/`SignifierId`）：
  落库 `DrawableMeshBundle`、`UiPass`/`WireframePass`（+0x89/+0x8a）、`DontCastShadow`、
  `SignifierMinimumLifespan`（+0x58 bit0）、`SignifierParentToVolumeOrigin`（+0x88）、
  `TransientMesh`（mesh_kind 载荷）+ `AssetMeshLibrary`/`AssetShaderLibrary` 解析面，然后
  按各闭包常量接线（circle build mesh 选择 flag=0→"vertex_color_shader"、1→"dashed_line" 已据）。
- 归属：walls/ui render-host owner；本组仅登记。

## 六、自检

- 未改 `src/asset_libraries/ssbo_library.rs`、`src/geometry/**`。
- 未改任何他域文件；`cargo test` 全部在本域 filter 下执行。
- 报告与证据路径：`work/restorer-render-2026-10-17/`。
