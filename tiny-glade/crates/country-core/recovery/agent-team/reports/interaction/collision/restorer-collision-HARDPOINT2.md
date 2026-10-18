# REBUILD-COLLISION-HARDPOINT2 — collision restorer2（restorer 第二轮收口）

日期: 2026-10-17
角色: restorer（worker `collision-restorer`）
协议: planning/2026-10-16-logic-verify-protocol.md（fresh 反编译 + 逐语句判等；重建前分类）
依据: reports/interaction/collision/{LOGIC,REBUILD-COLLISION-HARDPOINT,REBUILD-DEEP}.md
证据: planning/impl-collision-restorer2/results/（r1/r2 ReVa、g1 ghidra fresh）+ 复用
  planning/impl-collision-hardpoint/results/（b1 41-callee、b2 disasm 4630 指令、h1/h2 ReVa 前次）
  + planning/impl-collision-deep/results/b1-collider-lifecycle（remove_collider ×3 fresh）
  + planning/impl-walls-return/decomp/14155ba30-restore_decorators.txt（walls 消费面）
编译: `cargo check -p country-core --lib`（default + `--features native-physics`）**0 error**；
  collision 15 + rt 8 = **23 单测 PASS**（`--features native-physics`）。

## 批 1 — update_rapier_colliders（0x1408d0490）：hardpoint 第三次收敛尝试 + 逐段 call 序判等

### 收敛尝试（本轮新增，均失败——如实登记）

| 尝试 | 证据 | 结果 |
|---|---|---|
| ReVa `--write` worker `collision-restorer`（新 clone 从 seed）重新 set-function-prototype + save + get-decompilation | r1-reva-hardpoint-retry.result.json | `set-function-prototype` **success**（`void __cdecl update_rapier_colliders(longlong *this)`）；`get-decompilation` 仍 `Recoverable Error: Unable to find unique hash for varnode`（isError） |
| ReVa run-script `toggleParamID`（关闭 ParamID 分析，绕过已知 varnode-hash bug）+ setSimplificationStyle + decompile | r2-reva-paramid-off.result.json | 本 Ghidra 版本 `DecompInterface` 无 `toggleParamID` 属性（API 不存在）；仍同错误 |
| High P-code（h2 前次）：`getHighFunction().getPcodeOps()` | h2-reva-pcode.result.json（复用） | HighFunction 构建阶段同错误；`DecompileResults.getPcodeOps` 属性不存在（raw pcode 不可得） |

**verdict：`未实现-需重做`（decompile-hardpoint 不可逆，行为级骨架，逐段 call 序判等）。**
"Unable to find unique hash for varnode" 为 Ghidra decompiler 限制（22401 B / 4630 指令 /
bevy 查询 + 5 组件族内联 monomorphization），非签名/worker 状态问题（三个 worker 均复现）。

### 逐段 call 序判等（升级：行为骨架 → 逐段结构判等）

从 `hardpoint-update_rapier_colliders.disasm.txt`（4630 条）按 13 段提取 CALL 目标，
与 `update_rapier_colliders-skeleton.md` 骨架逐段对照：

| 段 | 区间 | disasm CALL 目标（提取） | 骨架描述 | 判等 |
|---|---|---|---|---|
| s0 | 0x490-0x736 | are_scopes_on / rfind×2 / slice_error_fail / VAL / begin_scope | puffin scope "update_rapier_colliders" | **判等** |
| s1 | 0x962-0xf3f | get_changed_ticks_slice_for×2 / calculate_hash<[u32;3],UVec3> / cuboid / from_translation / __rust_alloc / memcpy / hash_one<GlobalId> / upsert_collider_entry | 矩形 cuboid RaycastMeshData → upsert | **判等** |
| s2 | 0x11e5-0x12b9 | dispose_collider_mesh_instance / drop_slow<RaycastMeshData> / reserve_rehash / hash_one | 移除实体 collider 释放 + 槽脏 + 发布 | **判等** |
| s3 | 0x1605-0x1fd6 | get_changed_ticks_slice_for×2 + hash_one×5 / reserve_rehash / dispose×2 / drop_slow×2 | 第二组件族变更循环（多 dispose + reserve） | **判等** |
| s4 | 0x2239-0x269a | construct_arch_geo×2 / new_unindexed×2 | arch 几何 → new_unindexed 网格 | **判等** |
| s5 | 0x279a-0x3a81 | get_collision_geo / new_unindexed / upsert_collider_entry / get_point×N / xvy / as_points3/as_points2 / from_iter<Vec3,circle>/<Vec2,circle> / grow_one<Vec3> / grow_one<(u8,u32)> / from_iter<[u32;3],floor_mesh> / new_indexed / dispose×2 / drop_slow×2 | 圆/地板网格（get_collision_geo + circle + floor_mesh）+ new_indexed + upsert + dispose | **判等**（骨架 s5 未列 get_collision_geo，补记） |
| s6 | 0x35bc-0x36e6 | from_iter<Vec3/2,circle> / grow_one×2 / memmove / from_iter<[u32;3],floor_mesh> | 圆/地板点位装配 | **判等** |
| s7 | 0x398c-0x3e90 | new_indexed / hash_one / upsert_collider_entry×3 / reserve_rehash×4 | 圆/地板 collider upsert（×4 入口） | **判等** |
| s8 | 0x46e1-0x4a10 | get_added_ticks_slice_for×2 / hash_one / reserve_rehash / dispose / drop_slow | bevy `Added<…>` 查询循环 + dispose | **判等** |
| s9 | 0x4e9f-0x5262 | get_changed_ticks_slice_for×2 / hash_one×2 / reserve_rehash / from_iter<Triangle,roof_trigger_volume::triangles> | 屋顶触发体积三角收集 | **判等** |
| s10 | 0x5386-0x5615 | dispose / drop_slow / memcpy / __rust_alloc / new_unindexed / hash_one | 屋顶触发体积网格 new_unindexed + dispose | **判等** |
| s11 | 0x57cc-0x5aaf | upsert_collider_entry / reserve_rehash×2 / dispose / drop_slow | 屋顶触发体积 upsert + 最终 dispose | **判等** |
| s12 | 0x5b3a-0x5e3c | hash_one / VAL / end_scope / panic 系列 | puffin scope 结束 + panic | **判等** |

**结论**：13 段 CALL 目标与骨架**逐段判等**（含 41-callee 面的 `get_changed_ticks_slice_for` /
`get_added_ticks_slice_for` / `reserve_rehash<WallId, Vec<StoneColumn>>` / `upsert_collider_entry
<Entity,(Vec3,Quat)>` 锚定）。但**语句级 CFG 判等仍不可得**（typed prototype 不可逆）——按协议
`未实现-需重做`，不因逐段 call 判等宣布完成。s5 补充：含 `get_collision_geo`（0x1408e8d70），
骨架原未列。

## 批 2 — 跨域类型锚定（walls 解锁）：remove_collider 族 + ColliderDiff 载荷

### fresh 反编译锚定（remove_collider<DecoratorId> 0x140b409b0 / <WallId> 0x141245220 /
### <StairGraphEdge> 0x140252ad0，复用 impl-collision-deep/b1；dispose 0x1408d9590 本轮 fresh g1）

- **remove_collider ABI**：`(out: *mut 句柄 in RCX, map: &mut HashMap<K,V> in R8, key: K in R9)`。
  - 命中：hashbrown 探测（control 字节 = 0xff 删除标记、count--）→ `dispose_collider_mesh_instance()`；
  - 未命中：`*(u8*)(out+0x20) = 0x11`（`EMPTY_COLLIDER_HANDLE_TAG` = 槽数）。
  - K 布局：DecoratorId = i32 比较（`*(int*)(slot)`）、WallId = u64（`*(longlong*)(slot)`）、
    StairGraphEdge = 2×i32（slot 0x1c B）。
- **dispose_collider_mesh_instance（0x1408d9590）fresh 逐语句**：
  1. `ColliderSet::remove()`（query-only，无 parent wake）；
  2. registry (index,gen) 校验（`uVar9 < registry.len && registry[index].gen == gen`，失败 unwrap_failed
     panic）+ `registry[index].generation = gen+1`（失效旧句柄）+ free-list 推入
     （`world+0x18` head / `+0x1c` count 递减）——**Rust 未建模（登记分歧，见下）**；
  3. `compute_aabb` → `xz_flatten_aabb` → `DirtyTiles::iter_world_space_rect_cells`（栅格化）；
  4. `publish<ColliderDiff, ColliderDelta>`（removed 增量）+ 输出 64 B 句柄 + `Arc::drop_slow`。
- **walls 消费面（restore_decorators 0x14155ba30 fresh）**：每装饰器按 before/after tag 条件调
  `remove_collider<DecoratorId>` 2–4 次；每次命中（`ret[0] != 0x11`）后调用方对自有
  `Arc<RaycastMeshData>` 做 refcount 递减（`LOCK; *rc -= 1; if 0 drop_slow<RaycastMeshData>`）。

### 落地（collision 域，world.rs）

1. **`ColliderDiff` 载荷落地**：`{removed: Vec<SubworldColliderHandle>, added: Vec<SubworldColliderHandle>}`
   （锚定：find_mesh `Subscriber::take` → removed 图剪枝 + added AABB/Qbvh 遍历）。`ColliderDelta`
   = `{handle: Option<SubworldColliderHandle>, removed: bool}`，`apply_to` 按 removed 位推入对应列表。
2. **`dispose_collider_mesh_instance`**：补 AABB → 脏瓦片栅格化 + 发布 `ColliderDelta{handle, removed:true}`
   + 返回句柄。
3. **`remove_collider`**：返回 `dispose` 句柄（`Some`）；doc 写明 0x11 miss 判别 + walls 的 Arc-drop 合同。
4. **`on_collider_upserted`**：改为 `(delta: ColliderDelta)` 转发发布（fresh：薄 publish 包装）。
5. **`upsert_collider`**：发布 `ColliderDelta{handle, removed:false}`（added 增量）。

**接口语义（供 walls 接入，登记）**：`remove_collider<DecoratorId>(world, map, key) -> Option<SubworldColliderHandle>`；
命中后调用方需对自己的 `Arc<RaycastMeshData>` 递减一次；未命中（0x11）保留 Arc。

**分歧登记**：
- `ColliderMeshInstance` 机器布局 `{gen@+0, slot@+4, handle@+8, …}` vs Rust `{mesh@+0x00, …}`
  （语义字段一致，字节级未对齐）；registry free-list（`world+0x18`/`+0x1c`）与
  `registry[index].generation = gen+1` 的机械等价未建模（Rust 用 `Vec<ColliderRegistryEntry>`，
  无 free-list/无实例↔注册表链接）——**登记，不做半等价冒顶**。

## 批 3 — find_mesh_mesh_intersections（0x140a97b70）：域内装配实现

基于 `find_mesh_mesh_intersections.dc.txt`（7783 B fresh，复用）逐语句装配：

1. **图剪枝**（实现）：`Subscriber::take<ColliderDiff>()` → 逐 `removed` 句柄 `GraphMap::remove_node`
   —— `MeshIntersectionGraph::inner_mut()` 新增（collision.rs）；
2. **逐节点 AABB**（实现）：逐 `added` 句柄 `ColliderSet::get` + `Collider::compute_aabb`；
3. **候选槽 Qbvh 遍历**（实现）：逐候选槽 `Qbvh::traverse_depth_first_node<ColliderHandle,
   BoundingVolumeIntersectionsVisitor>`——Rust 经 fork 的公开 API
   `QueryPipeline::colliders_with_aabb_intersecting_aabb`（内部即 `Qbvh<ColliderHandle>` 遍历）
   等价，每候选句柄调用 `intersect` 回调（`RaycastWorld::slot_query_pipeline` 访问器新增）。

**跨域登记（未实现，装配阻塞点，写清消费点）**：
- `intersect_newly_added_colliders::closure_env_0`：三角对三角相交（`RaycastMeshData` 顶点/法线/uv
  缓冲）→ `Segment` → `merge_segments_into_polylines` → `SparseInstanceBufferWriter::push<
  BrickTransformSsbo>`（中点步进 0.5、法线/切向正交化、×100 取整）——`BrickTransformSsbo`
  属 geometry::instanced_wall（walls/device 域）；
- 图 `add_node`×2 + `add_edge`（`MeshIntersectionArtifacts` 载荷，含 `Vec<BirdNestCandidateLocation>`）；
- `spec_extend<BirdNestCandidateLocation, FlatMap<AllEdges<GraphMap>, …>>` 从图边收集鸟巢候选；
- `SparseInstanceBufferAllocation<SparkleSsbo>` drop（resources/sparkle）。

**verdict：域内装配已实现（图剪枝 + AABB + 候选槽遍历），跨域 closure 登记。**

## 批 4 — world.rs 9 需重做逐项复核（按 LOGIC.md 分歧点）

| LOGIC.md 项 | 现状 | verdict |
|---|---|---|
| Raycasting::raycast 0x1408cc2e0（转发错） | 转发 `raycast_w_terrain`（含地形 0x20 位），返回 RaycastWorldResult | 已修（REBUILD-REMAINDER 后），**复核判等** |
| intersects_any 0x1408cc320 | 地形短路 + Rapier::intersects_any | 已修，**复核判等** |
| spherecast 0x1408cc390 | 完整（cast_shape 17 槽 + 地形合并 toi 语义） | 已修，**复核判等** |
| mesh_intersect_collider_diff_publisher 0x1408cc670 | `lea rax,[rcx+0x3cb0]` = 返回 &Publisher | 已修，**复核判等**（本轮载荷落地后 Publisher<ColliderDiff> 带载荷） |
| legacy_raycast 0x1408cc680 | 转发 Rapier::raycast + 未中判别写 toi 槽 | 已修，**复核判等** |
| RaycastWorld::raycast 0x1408cc870 | 跨槽最后一次命中覆盖（无 toi 比较）+ mask **u32** + RaycastWorldResult | 已修，**复核判等** |
| new_convex 0x1408cec10 | calculate_hash + try_convex_hull + Box<dyn Shape> | 已修，**复核判等** |
| RaycastWorld::new | 17 槽 + Publisher + MeshInstanceSources | 已修（Vec 注册表布局分歧登记） |
| SubworldIndex::from_usize_unchecked 0x14067dd80 | 无检查包装（mov rax,rcx;ret） | 已修，**复核判等** |

9 项全部判等（前轮已修，本轮 fresh 复核无新增分歧；返回类型 RaycastWorldResult vs 旧 Option 已统一）。

## 编译与测试

- `cargo check -p country-core --lib`（default）：0 error（14 warning 均属 save/audio/ivy 等既有模块，collision 无新增）。
- `cargo check -p country-core --lib --features native-physics`：0 error。
- `cargo test -p country-core --lib --features native-physics -- systems::collision resources::rt`：
  **23 PASS**（collision 15 含新增 find_mesh 图剪枝 + ColliderDelta apply_to 单测；rt 8）。

## 剩余登记

- update_rapier_colliders：`未实现-需重做`（decompile-hardpoint 三 worker 不可逆；13 段 call 序
  与骨架逐段判等；语句级判等不可得）。
- find_mesh_mesh_intersections：域内装配已实现；跨域 closure（三角相交/BrickTransformSsbo 铺砖/
  BirdNestCandidateLocation/SparkleSsbo）登记。
- dispose 的 registry free-list + generation-bump：Rust 未建模（ColliderMeshInstance 布局分歧登记）。
- rt 设备面（update/upload_rt_world host 接线、build_gpu_bvh/bvh_builder_worker obvhs 入 workspace）：
  维持 REBUILD-COLLISION-HARDPOINT 登记，本轮未动。
