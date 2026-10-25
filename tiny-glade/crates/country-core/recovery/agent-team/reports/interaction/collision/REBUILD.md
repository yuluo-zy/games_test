# REBUILD — systems/collision 真实逻辑重建（rebuild-collision）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
LOGIC.md: reports/interaction/collision/LOGIC.md（9 个 `未实现-需重做` + collision.rs `impl$0::build`）
证据: planning/logic-collision/rebuild-collision-fresh.result.json / -decomp.txt
（ghidra `--worker shared` 只读 fresh 反编译，2026-10-17 00:54）

## 分类结论（协议 §2026-10-17 用户补充）

9 个需重做函数 + 1 个缺实现函数全部判定为 **真实业务（重建）**，**0 个宏生成胶水**：
无一属于 derive/fmt/serde/注册/资源插入的宏胶水类别——RaycastWorld::new 是带布局语义的构造器，
mesh_intersect_collider_diff_publisher 是返回 Publisher 字段引用的访问器（机器码 `lea rax,[rcx+0x3cb0]; ret`），
SubworldIndex::from_usize_unchecked 是 `mov rax,rcx; ret` 的无检查包装（非 derive），均按业务重建。
`Default for RaycastWorld` / `Default for RaycastMeshData` 保持 glue（委托 new，不重建业务）。

## 逐函数：分类 → 重建 → 逐语句判等自检

### 1. Raycasting::raycast（0x1408cc2e0）— 真实业务，重建
- fresh：`RaycastWorld::raycast_w_terrain(); return;`（callee=raycast_w_terrain@0x1408cd430）。
- Rust：`self.world.raycast_w_terrain(terrain, origin, dir, mask)`。
- 判等自检：转发 callee 一致；返回 `RaycastWorldResult`（原返回值，O：判别@+0x34）。
- ABI 说明：raycast_w_terrain 需地形数据，`Raycasting` 方法新增 `terrain: &TerrainHeightsData<TerrainHeightSsbo>` 实参
  （原 PDB void() 占位，实参序按 terrain 子世界分支推理；无现成调用者，不破坏他模块）。

### 2. Raycasting::intersects_any（0x1408cc320）— 真实业务，重建
- fresh：`if (mask & 0x20) { TerrainHeightsData::intersect_ray<TerrainHeightSsbo>(); if (命中) return; } Rapier::intersects_any();`
- Rust：mask 含 0x20 位 → `terrain.intersect_ray` 命中即 `return true`（地形短路）；否则
  `Rapier::intersects_any`（17 槽 Qbvh 遍历）。
- 判等自检：地形短路分支逐条对应；bool 返回语义一致。

### 3. Raycasting::spherecast（0x1408cc390）— 真实业务，重建
- fresh：先 `RaycastWorld::spherecast()`（0x1408ccfc0），mask 含 0x20 时地形合并：
  `toi = (dist(origin,地形点) - radius) / max(dir_len, 4e-8)`；NaN：分子→0、分母→4e-8；
  `toi <= radius` 才并入；`rapier.toi < 地形.toi` 才保留 rapier（`local_70 < fVar9`），否则地形（0xb）覆盖。
- Rust：逐语句对应上述控制流与 NaN/边界语义。
- 判等自检：分支/比较方向/NaN 策略一致；返回 `RaycastWorldResult`（地形判别 0xb@+0x00、normal=(0,1,0)、toi@+0x30、判别 0@+0x34）。

### 4. RaycastWorld::mesh_intersect_collider_diff_publisher（0x1408cc670）— 真实业务，重建
- 机器码：`lea rax,[rcx+0x3cb0]; ret`（LOGIC + 任务锚定）。
- Rust：`&self.publisher`，`RaycastWorld` 新增 `publisher: Publisher<ColliderDiff>` 字段。
- 判等自检：返回 `&Publisher`（原=返回 `&self+0x3cb0` 字段引用）；删去自造常量 `return 0 as u64`。

### 5. RaycastWorld::legacy_raycast（0x1408cc680）— 真实业务，重建
- fresh：`Rapier::raycast(); if (判别==2) { *(out+0x30) = 2; } else { 整份拷贝 }`。
- Rust：`Rapier::raycast(self, origin, dir, u32::MAX)`；未命中再写 `toi = 2.0`（+0x30 槽）。
- 判等自检：未命中判别 2 写 +0x30 的 legacy 特例逐条对应（+0x34 由 Rapier::raycast 已写 2）；
  返回 `RaycastWorldResult`。mask=全槽为推理（decomp 无显式 mask 实参）。

### 6. RaycastWorld::raycast（0x1408cc870）— 真实业务，重建
- fresh：`Rapier::raycast(); return;`（callee=0x1408cdc60）。
- Rust：`Rapier::raycast(self, origin, dir, mask)`；mask 改 **u32**（原误 u64）。
- 判等自检：转发 callee 一致；返回类型改 `RaycastWorldResult`（~0x40B）。

### 6b. Rapier::raycast（0x1408cdc60，支撑实现；原属 LOGIC 未实现登记）
- fresh：17 槽 u32 mask 遍历；每槽 `QueryPipeline::cast_ray_and_get_normal(solid=1, QueryFilter::default())`；
  **最后一次命中槽覆盖（跨槽无 toi 比较）**；全未中写判别 2@+0x34。
- Rust：`[RapierSubWorld; 17]` 循环逐条对应；命中重写 handle/position/normal/toi/判别 0；结束未中→`RaycastWorldResult::miss()`。
- 分歧消除：position = origin + dir*toi、normal = `RayIntersection.normal`（对应 `rapier_hit_to_raycast_world_result`
  0x1408ce4a0 的**非三角路径**；三角网格 barycentric 法线插值属 LOGIC 未实现登记，登记在 REBUILD 待续）。

### 7. RaycastMeshData::new_convex（0x1408cec10）— 真实业务，重建
- fresh：`calculate_hash<ref$<slice$<UVec3>>>(points)` → hash@+0x70；`try_convex_hull` →
  `ConvexPolyhedron::from_convex_mesh` → `Box<dyn Shape>`@+0x20/+0x28；tag=4@+0、kind=4@+0x10、
  +0x68=0x3f80000000000000；None 写 `*out=0`。
- Rust：`calculate_hash(&points)` 存 hash（删硬编码 0）；`rapier3d::parry::transformation::try_convex_hull`
  + `ConvexPolyhedron::from_convex_mesh` → `Box<dyn Shape>`（替代 `SharedShape::convex_hull`→Arc）；
  入参按 PDB 改 `&[glam::UVec3]`（原 `&[[f32;3]]`；无现成调用者）。
- 判等自检：构造路径/存储表示/hash 数据流逐条对应；struct 重构为
  {tag, kind, shape: Option<Box<dyn Shape>>, scale, hash}（+0x00/+0x10/+0x20/+0x28/+0x68/+0x70 概念对应）。

### 8. RaycastWorld::new（构造器）— 真实业务，重建
- LOGIC：17×0x390 槽内联 @+0x20（RigidBodySet/ColliderSet/QueryPipeline/脏标志@+0x388）、Publisher@+0x3cb0。
- Rust：`slots: [RapierSubWorld; 17]`（原 `Subworld` 更名 `RapierSubWorld`，与 `impl$0::build` 的
  `default<RapierSubWorld>` 符号一致）+ `publisher: Publisher<ColliderDiff>`；`dirty: u64` 从 RaycastWorld
  移入每槽（脏标志语义）。
- 判等自检：17 槽 + Publisher 结构对应；**精确字节偏移（0x390/槽、+0x3cb0）为近似**——无法在不破坏
  rapier 用法的前提下强排 repr(C) 对齐（fork 类型尺寸未逐字段锚定），记入分歧说明。

### 9. SubworldIndex::from_usize_unchecked（0x14067dd80）— 真实业务，重建
- 机器码：`mov rax,rcx; ret`（无检查直通）。
- Rust：`from_usize_unchecked(slot: usize) -> Self { Self(slot as u8) }`；删除自造 checked `new(slot)->Option`。
- 判等自检：无边界检查、usize→u8 截断，语义与 `mov rax,rcx; ret` 一致；模块内测试同步为无检查语义。

### 10. collision.rs impl$0::build（0x140b685e0）— 真实业务，重建（原缺失）
- fresh：SubApp::add_systems（Startup/Update/OnExit）+ `insert_resource<RaycastWorld>`
  （`default<RapierSubWorld>`×17 + with_capacity<RigidBody> 内联构造）+ `insert_resource<MeshIntersectionGraph>`
  （`GraphMap<SubworldColliderHandle, MeshIntersectionArtifacts, Undirected, RandomState>::default`）。
- Rust：新增 `CollisionPlugin::build(&mut World)`（crate 不依赖 bevy_app，按 resources/session.rs 合同由宿主装配）：
  `insert_resource(RaycastWorld::new())` + `insert_resource(MeshIntersectionGraph::default())`；
  新增 `MeshIntersectionArtifacts` 与 `MeshIntersectionGraph`（GraphMap 新类型 + `Resource`）。
- 分歧说明：Startup/Update/OnExit 系统注册依赖的初始化系统属 LOGIC 未实现登记，本次不注册（登记待续）。

## 分歧消除说明（对照 LOGIC.md 严重分歧摘要）

| LOGIC 分歧 | 消除 |
|---|---|
| 1. Raycasting::raycast 转发目标错（纯 rapier） | 改转发 `raycast_w_terrain`（0x1408cd430），恢复地形 0x20 子世界分支 |
| 2. RaycastWorld::raycast 选择算法错（toi<t 最近） | 改**最后一次命中覆盖**（跨槽无 toi 比较）；mask u64→u32 |
| 3. 返回类型系统性不符（Option<(f32,u32)>/bool/None） | 全部改 `RaycastWorldResult`（~0x40B：handle/地形标记@+0x00、position@+0x18、normal@+0x24、toi@+0x30、判别@+0x34=0/2、extra@+0x38） |
| 4. new_convex 数据流错（hash=0、Arc 构造路径） | `calculate_hash` 计算 + `try_convex_hull`+`from_convex_mesh`→`Box<dyn Shape>` |
| 5. mesh_intersect_collider_diff_publisher 自造常量 | 返回 `&self.publisher`（`lea rax,[rcx+0x3cb0]`） |
| 6. RaycastWorld 布局错（Vec<Subworld>+dirty） | `[RapierSubWorld; 17]` + `publisher` 字段；精确字节偏移近似（fork 类型尺寸） |

## 跨模块机械适配（编译前提，唯一改动的他模块）

`RaycastWorld::raycast` 返回类型从 `Option<(f32,u32)>` 改 `RaycastWorldResult` 后，
`src/resources/roofs/roof_shape.rs` 两个消费点（`roof_uv_at_ws_pos` / `roof_uv_and_point_at_ws_pos`）
作**机械适配**：mask `u64::MAX`→`u32::MAX`、`?`/解构改 `is_miss()` 判别 + `hit.toi`，业务语义（向下射线取
toi/点）不变。这是"返回类型一致"与"cargo check 通过"共同要求的最小改动。

## 编译状态

- `cargo check -p country-core`：**通过**（默认特征，无 error）。
- `cargo check -p country-core --features native-physics`：**通过**（含 `Rapier::raycast/intersects_any/spherecast`、`new_convex`、`raycast_w_terrain` 的 rapier 实现）。
- collision.rs / collision/world.rs / roof_shape.rs：无 error、无 warning（两种特征均核验）。
- 说明：工作树有并行 agent 在改 ivy/terrain/ui 等模块，全仓 check 状态随之短暂波动（终态已验证通过）；
  本模块文件在两种特征下始终干净。`cargo test --lib systems::collision` 的编译被其他模块 test-cfg
  错误（render/nani `GraphicsDevice(1)`、ui_clutter 注册等）阻塞，collision 文件不在其错误列表中。

## 待续（不属于本次 9 函数，登记）

- `rapier_hit_to_raycast_world_result`（0x1408ce4a0）三角网格 barycentric 法线插值（本次 raycast 用非三角路径 normal）。
- `RaycastWorld::spherecast`（0x1408ccfc0）命中 witness/normal 方向翻转的精确组合（本次用 `ShapeCastHit::normal2` + origin+dir*toi 近似）。
- `RaycastWorld` +0x3d80 Qbvh 布局字段（本次经 `QueryPipeline::intersections_with_ray` 内部 qbvh 遍历等价）。
- `ColliderDiff`/`ColliderDelta` 载荷内容、`on_collider_upserted` publish 接线、`MeshIntersectionArtifacts` 内容（LOGIC 未实现登记）。
