# restorer-collision-REMAINDER2 — collision 剩余收口（rt 设备面 + find_mesh 跨域类型 + 可落地部分）

日期: 2026-10-17
角色: restorer（只读证据 ghidra `--worker shared`）
协议: `planning/2026-10-16-logic-verify-protocol.md`（fresh 反编译 + 逐语句判等 + 重建前分类）
不变量: 只以 fresh 反编译为准；缺类型锚定或登记，不编造；不改他模块。
证据: `planning/impl-collision-remainder2/results/`
- r1-decomp.txt（fresh：raycast 族 8 + RaycastMeshData 构造 5 + flush/get/dispose/upsert 12）
- r2-decomp.txt（fresh：find_mesh 7783B / update_rt_world / upload_rt_world / build_gpu_bvh /
  bvh_builder_worker / intersect_mesh_mesh / should_intersect / merge_segments / to_offset+into_transformed 8）
- r5-decomp.txt（closure_env$0 0x1417d1440、spec_extend 0x1412f9800、closure_env$2、remove_invalid_polylines、
  from_iter<PolylinePoint,2>、grow<PolylinePoint>）
- r7-decomp.txt / r8-disasm.txt（traverse_bvtt monomorph + **intersect_mesh_mesh 全量 disasm 175 指令**、
  new/grow/push<BrickTransformSsbo>）
- r10-decomp.txt（merge closure$0 0x1408f4670 = `roundf` 端点键 + BVH 输入 monomorph）
- r12/r13/r14-decomp.txt（Aabb3 expand/aabb/contains 复核；raycast wrapper 6；visualize_colliders）
编译/测试:
- `cargo check -p country-core --lib`（default）**0 error**（14 既有 warning）
- `cargo check -p country-core --lib --features native-physics` **0 error**（10 既有 warning）
- `cargo test -p country-core --lib -- systems::collision resources::rt` → **22 PASS**
- `cargo test -p country-core --lib --features native-physics -- systems::collision resources::rt` → **31 PASS**
（期间 20:04–20:06 他组 WIP 曾令全仓 test 构建瞬时失败于
`replay_cmd_buffer.rs`/`wall_history_replay.rs` 缺 `Default`，随后自愈，非本模块。）

## 批 1 — find_mesh_mesh_intersections 跨域类型 + 三角相交落地

### 1.1 已落地（fresh 逐条）

| 项 | 证据 | 实现 |
|---|---|---|
| `Segment` 语义修正 | `intersect_mesh_mesh` **disasm 0x140a9afa0**：输出元素 stride 0x24；`a` Vec3@+0x00、`b` Vec3@+0x0c、`position` Vec3@+0x18 = `normal(a)+normal(b)` | `Segment { a: Vec3, b: Vec3, position: Vec3 }`（旧实现误用 IVec3 端点，已修） |
| 三角对相交 closure | disasm：`triangle()`×2 → `intersect_triangle`（仅判别 1 = Segment）→ `aabb`×2 → `expand(0.1)`×2 → `inclusive_contains_point`×4（a∈A,b∈A,a∈B,b∈B）→ `normal`×2 → push | `intersect_triangle_pair` + `TRIANGLE_INTERSECTION_MARGIN=0.1`（常量 0x142930d60 = {0.1,0.1}，+0xd8=0.1；`Aabb3::expand` 内部 ×0.5 已复核 0x1412403c0） |
| TriMesh 对驱动 | `intersect_mesh_mesh` disasm：`traverse_bvtt` 先收集叶对，再逐对处理 | `intersect_mesh_mesh_segments(mesh_a, mesh_b, out)`（native-physics） |
| 端点网格键 | `merge_segments_into_polylines::closure$0` 0x1408f4670：`roundf(x/y/z)` 后饱和转 i32（NaN→0）作 `IVec3` 键 | `round_ivec3`；`merge_segments_into_polylines` 改用浮点端点取整 |
| `remove_invalid_polylines` | `retain_mut` closure 0x1412f3850：`points.len@+0x18 == 0` → 丢弃 | `remove_invalid_polylines`（空点折线删除） |
| `BirdNestCandidateLocation` | `spec_extend` 0x1412f9800 逐元素拷贝：stride **0x40**，+0x00 `Arc`（refcount++），+0x08/+0x0c/+0x10/+0x14 u32，+0x18/+0x20/+0x28/+0x30 u64，+0x38/+0x3c u32 | 类型落地 + 布局单测（0x40 B、Arc@0）；pointee 类型未锚定 → `Arc<()>` |
| `BirdNestCandidateLocations` | 资源符号 `insert_resource` 0x140ee9270 / `init_state` 0x141375610 | `pub struct BirdNestCandidateLocations(pub Vec<…>)`（Resource） |
| `MeshIntersectionArtifacts` 载荷 | closure_env$0 0x1417d1440 push：`Vec<Polyline>` + 两注册表 +0x08 qword + 两 `SubworldColliderHandle`（12B 紧凑）；spec_extend 从每条边展平收集鸟巢候选 | 字段落位 `polylines / bird_nest_candidates / mesh_ref_a / mesh_ref_b / a / b` |

### 1.2 跨域登记（宿主缺失，未实现）

1. **Brick 铺砖**（`intersect_newly_added_colliders::closure_env$0` 后半）：
   - 宿主：`country_core::geometry::instanced_wall::BrickTransformSsbo` **具体类型不存在**
     （`instanced_wall.rs` 仅有泛型 `InstancedWall<T>`/`BrickSourceId`）；`SparseSsbo` impl、
     `SparseInstanceBufferWriter::push` 的 T 绑定缺类型。
   - 已锚定事实：元素 **0x60 B**（`grow_one<BrickTransformSsbo>` ×0x60）；类型名断言长度 0x3a（58）；
     `take_out_waste<BrickTransformSsbo>`；步进 0.5、方向归一化，砖写入 96 B 观察序列
     （affine 12×f32 @+0x00..0x30；+0x30=0、+0x34=0、+0x38/-0x3c=-100、+0x40=-100、
     +0x44=来源 id、+0x54={0.1,0.1}、+0x5c=6；+0x48..+0x54 未写）；×100 取整用于 `calculate_hash<IVec3>`。
   - 解锁：在 `geometry::instanced_wall` 落地 `BrickTransformSsbo`（0x60，`SparseSsbo`/`SsboType`）
     并把 writer 接入 `find_mesh_mesh_intersections`。
2. **图边装配**：`add_node`×2 + `add_edge`（fresh 有界；`upsert_collider_entry`/registry→mesh
   链接未锚定，无法构造 mesh_ref/句柄载荷）。
3. **spec_extend 边收集**：需边载荷先填充 `bird_nest_candidates`（当前字段已落位，元素内容来源未锚定）。
4. `SparseInstanceBufferAllocation<SparkleSsbo>` drop：`SparkleSsbo` 类型已在
   `resources/sparkle.rs`（可直接接线，待图边构造落地）。

## 批 2 — rt 设备面

| VA | 函数 | verdict | 内容 |
|---|---|---|---|
| 0x1408bcff0 | `upload_rt_world` | **落地（设备 seam）** | 写锁内 `mem::take` 三 Vec；越界门 `(meta_len|tri_len)<0x40000 && record_len<0x10000`（否则 "Out of pre-allocated buffer space for ray tracing-data" panic）；依次 `FrozenMap::get` + 名字断言（**0x2c/0x2f/0x26** 锚定：`RtTriangleSsbo`/`RtTriangleMetaSsbo`/`RtBvhSsbo`）+ 容量检查 + `update_buffer_bytes`（len×0x18 / ×4 / ×0x50）。Rust：`upload_rt_world(rt, &AssetSsboLibrary, &dyn GraphicsDevice)` + 三个 `SsboType` 元素类型 + `TestDevice` 单测（含 too-small panic）。 |
| 0x1408bc650 | `update_rt_world` | **seam + 登记** | fresh：两段 0x10B `RtMeshHandle` 变更切片逐条 `RawTable::remove_entry` + `Arc::drop_slow`，然后 `from_iter<…, Map<IterMut, consume>>` 重建哈希表；RwLock 写锁后写 inner +0x68..+0xa7 log 与 +0x38 脏字节。Rust：`remove_mesh`/`remove_meshes`/`take_upload_data`；两段切片宿主（Bevy 查询）、log、脏字节/DirtyCow 计数登记未建模。 |
| 0x140b0da50 | `build_gpu_bvh` | **登记（obvhs 缺）** | 输入三角 0x28 B：`{a,b,c: Vec3, user_data u32 @+0x24}`（**已落地 `InputTriangle`**）；展开为 0x30 B `[Vec3;3]`（w=z）→ `obvhs::cwbvh::builder::build_cwbvh_from_tris`（选项常量已见：max leaf 4、0x10000、0.2/1.0）→ `Vec<GpuRtTriangle>`（`idx → input[+0x24]`）→ 输出 9 qword。**解锁：workspace 引入 obvhs（原 Cargo.lock 版本）**。 |
| 0x140b24d50 | `bvh_builder_worker` | **登记（obvhs + RtMesh 顶点面）** | RwLock 循环：state@+0xa0（先写 2，旧值 2 → sleep）、busy@+0xa8、输出 @+0x20..+0x5c；`Values<HashMap<RtMeshHandle, DirtyCow<RtMesh>>>` → `ChunksExact<Vec3>`（RtMesh 顶点）→ `Vec<InputTriangle>` → `build_gpu_bvh`。Rust：`RtBvhBuilderGlue{state,busy}` + 常量已锚定；RtMesh 顶点 Vec 的精确字段偏移未锚定（登记）。 |

## 批 3 — raycast 族余量（fresh 逐函数）

| VA | 函数 | verdict | 说明 |
|---|---|---|---|
| 0x1408cd430 | `raycast_w_terrain` | **判等（复核）** | 分支：mask&0x20；rapier 命中/未命中两路都查地形，地形命中无条件覆盖；未命中+无地形 → 判别 2；地形结果 position@+0x18、normal=(0,1,0)@+0x24、toi@+0x30、判别 0。Rust 等价。 |
| 0x1408cdc60 | `Rapier::raycast` | **部分（已修句柄区）** | 17 槽 u32 mask、`cast_ray`、最后一次命中覆盖；命中后校验注册表并拷贝条目前 24 B（`registry_handle_region`）。**登记**：原命中经 `rapier_hit_to_raycast_world_result` 做 barycentric 插值（其 mesh 结构字段 +0x18/+0x20/+0x28/+0x40/+0x48/+0x50/+0x58 与当前 `RaycastMeshData` 映射不一致，且注册表→mesh 链接未锚定）；Rust 内联合成法线/toi。 |
| 0x1408cdf80 | `Rapier::intersects_any` | **判等** | 17 槽 Qbvh 遍历 + 命中即返回（Rust `intersections_with_ray` 等价）。 |
| 0x1408ce180 | `Rapier::raycast_simple` | **已修** | fresh 无排除参数、`QueryFilter::default()`；命中后拷贝注册表条目前 24 B；未命中只写 +0x00=0x11。已删 `excluded` 过滤并改句柄区。缺失注册表条目时回退旧打包（登记：`upsert_collider_entry` 接线未落地）。 |
| 0x1408cc8a0 | `raycast_intersections_with_ray` | **判等（复核）** | 地形无条件查询（判别 0xb）→ 17 槽 Qbvh visitor 收集 → `sort_unstable_by` toi 升序（<0x15 insertion_sort）。Rust 等价。 |
| 0x1408ccc90 | `legacy_spherecast` | **已修句柄区** | 17 槽 `cast_shape`，命中写 handle+position、normal=0、+0x30=0；未中 +0x30=2。句柄区改注册表条目前 24 B（fresh `local_1b8=*puVar1`）。 |
| 0x1408ccfc0 | `spherecast` | **部分（法线修正登记）** | 17 槽 `cast_shape` 最后一次覆盖；命中后同样拷贝注册表条目前 24 B（已修）。**登记**：原版按点积符号翻转/选择法线分量（XMM14/XMM15 序列），Rust 用 `hit.normal2`。 |
| 0x1408ce4a0 | `rapier_hit_to_raycast_world_result` | **未实现-需重做（布局）** | fresh 明确输入 mesh 结构偏移：+0x18 顶点数、+0x20 索引 ptr（stride 0xc）、+0x28 索引 len、+0x40 法线 ptr/len @+0x48、uv ptr/len @+0x50/+0x58；与当前 `RaycastMeshData`（+0x30 法线/+0x48 uv）不符；Rust 实现按旧映射。需重新锚定该结构（可能是 `RaycastMesh` 内的另一布局）。 |
| 0x1408ccc20/50 | `raycast_simple` / `_exclude_wall_id` | **判等（转发）** | 两者 fresh 均纯转发 `Rapier::raycast_simple`；Rust 已同体转发，exclude 入参不消费。 |
| 0x1408cc770 | `legacy_raycast_w_params` | **判等** | 转发 + 未中写 +0x30=2。 |

## 批 4 — collider 族余量

| VA | 函数/项 | verdict | 说明 |
|---|---|---|---|
| 0x1408cd5c0 | `flush_colliders` | **判等（复核）** | 17×0x390；脏字节@slot+0x388；0x200 B take-and-clear + publish + 清脏 + `QueryPipeline::update`。Rust 逐条对应。 |
| 0x1408cdb50 | `get_collider` | **已修（数据源）** | fresh 读**活 rapier Collider**：拷贝 +0x34..0x50 的 Isometry 到 +0x00..0x1c，+0x20=(user_data 高 32)|1，+0x24=user_data 低 32；未命中 `expect_failed`。Rust 新类型 `RaycastCollider` 落地（Quat 对齐致 0x30 B，字节偏移登记）。 |
| 0x1408cdbd0 | `get_collider_mesh` | **已修** | fresh 用**精确** `entry.generation == generation`（无 `|1`），失败 `begin_panic`。 |
| 0x1408ce440 | `get_rapier_collider` | **判等（语义）** | `ColliderSet::get_mut_internal`；Rust `&self` 读取（可变性登记）。 |
| 0x1408d98a0 | `xz_flatten_aabb` | **判等** | `{min.x,min.z,max.x,max.z}` 逐字节对应。 |
| 0x1408d9590 | `dispose_collider_mesh_instance` | **部分** | `ColliderSet::remove(...).expect`（已补）、AABB 栅格化、发布 `ColliderDelta{removed:true}`。**登记**：registry 校验（精确 gen）→ `entry@+0` = free-list head、`world+0x18` head、`+0x1c` 计数--、`entry.generation = gen+1`、free-list 推入；`ColliderMeshInstance` 机器布局 {gen@+0, index@+4, slot@+0x10} 与 Rust 差异。 |
| 0x140b40b50 | `upsert_collider_entry<Entity,(Vec3,Quat)>` | **登记** | 2358B 泛型 monomorph；registry 插入/更新与 kind 来源未逐句锚定 → raycast 句柄区在条目缺失时回退旧打包（明确登记）。 |
| 0x1408cee40 | `new_indexed` | **待复核（布局/哈希门控）** | 哈希链：`hash(tuple<&[[u32;3]], &[UVec3]>)`，随后仅在 normals 空（fresh `!SBORROW8(0,len_ptr)` 分支语义）与 uvs 空时折叠；输出 +0x00..+0x48 与当前语义模型大体一致，但 **uvs 槽在 fresh 写 0**、Vec 字段顺序（{cap,ptr,len} 观感）需重新锚定。当前实现无条件折叠哈希。 |
| 0x1408cf7d0 | `new_unindexed` | **待复核（布局/哈希门控）** | 顺序索引、`Vec<Point<f32>>` 顶点、`Vec<Vec3>` 法线、`Vec<Vec2>` uvs、shape、count 已见；哈希第二折仅在 uv 指针为 0 时执行（同款门控歧义）；Rust 以 UVec3 位模式填充 +0x00/+0x30 需重锚。 |
| 0x1408cfff0/0x1408d0120/0x1408d0230 | `new_cuboid`/`new_sphere`/`new_compound` | **判等（结构）** | 存储序列 `[0]=4,[1]=0,[2]=4,[3]=0,[4]/[5]=shape,[10..0xd]=参数,[0xe]=hash` 与 Rust 布局逐槽对应；参数区（hash_input/半径/半边长）位置一致。 |
| 0x1408d0390/0x140b26a80/0x140b26bd0 | `to_offset_collider_builder` ×3 | **判等（结构）** | shape Arc 克隆 + `ColliderBuilder::new` + position + sensor；Rust 等价。 |
| 0x14155d340/0x1408d98c0/0x1408d9960/0x140b26d20 | `into_transformed_shape` 族 | **部分** | impl$42 从活 collider 拷贝变换+shape（已实现）；impl$45 `nested_in` + mesh shape（已实现）；impl$41 Rectangle3d 的 `from_rotation_arc` 取向仍以 identity 近似（登记）。 |
| 0x1408d9990 | `as_collider_shape` | **判等** | 取 shape Arc。 |

## 批 5 — update_rapier_colliders

语句级仍不可逆（前轮三 worker varnode hash 失败）。本轮未重试，维持
`未实现-需重做（13 段 call 序判等，语句级不可得）`。

## 剩余登记汇总（解锁条件）

1. Brick 铺砖：`geometry::instanced_wall::BrickTransformSsbo`（0x60 B + `SparseSsbo`）落地。
2. `find_mesh` 图边构造：registry→mesh 链接 / `MeshIntersectionArtifacts` 的 mesh_ref 语义。
3. rt `build_gpu_bvh`/`bvh_builder_worker`：obvhs 入 workspace + RtMesh 顶点 Vec 字段锚定。
4. rt `update_rt_world`：Bevy 两段变更切片宿主 + inner log/脏字节建模。
5. `Rapier::raycast` 的 barycentric 链接：`rapier_hit_to_raycast_world_result` 输入 mesh 结构重锚。
6. `RaycastWorld::spherecast` 命中法线方向修正语义。
7. `merge_segments_into_polylines` 的 scratch 端点索引 Vec 与跨折线拼接/闭环精确语义（当前语义近似）。
8. `remove_invalid_polylines` 同 closure 的逐点法线重写段。
9. `new_indexed`/`new_unindexed` 的 Vec 字段顺序与哈希折叠门控重锚。
10. registry free-list/计数/`ColliderMeshInstance` 机器布局；`upsert_collider_entry` 接线。
