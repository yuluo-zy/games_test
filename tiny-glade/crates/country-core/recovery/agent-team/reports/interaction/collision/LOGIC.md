# LOGIC — systems/collision + resources/rt 逐函数逻辑判等（logic-verifier-collision）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md
标准: 只以 ghidra fresh 反编译(shared 只读)+ 机器码为准;结构/签名对上了但函数内部逻辑与
调用链不符 = 未实现-需重做。

## 审计范围与证据

- 已实现文件:`src/systems/collision.rs`、`src/systems/collision/world.rs`。
- **未实现(无 Rust body)**:`collision/intersect_meshes.rs`(21 函数,业务 4)、
  `collision/visualize_colliders.rs`(6 函数,业务 2)、`resources/rt/`(mod.rs 31 + bvh.rs 5 +
  bvh_builder.rs 6,业务 13)——无 src,本次仅登记。
- `world.rs` inventory 170 函数,业务 75;Rust 侧已实现业务函数仅 9 个。
- 证据落盘: planning/logic-collision/results/decomp-b01-core.txt / decomp-b02-core.txt /
  b0{1..7}*.result.json(ghidra shared 只读 fresh 反编译 + read_memory 机器码 + disassemble_function)。

## 函数表(已实现业务函数逐函数判等)

| VA | 原函数 | Rust | verdict | 分歧点 |
|---|---|---|---|---|
| 0x1408cc2e0 | Raycasting::raycast | world.rs `Raycasting::raycast` | **未实现-需重做** | 原=转发 `RaycastWorld::raycast_w_terrain`(0x1408cd430,含地形 0x20 位处理);Rust 转发 `self.world.raycast`(0x1408cc870 纯 rapier)。callee 错 → 缺地形子世界分支。返回类型:原=RaycastWorldResult(~0x40B:position/normal/toi/tag@+0x34),Rust=Option<(f32,u32)>。 |
| 0x1408cc320 | Raycasting::intersects_any | world.rs `Raycasting::intersects_any` | **未实现-需重做** | 原:`if (mask&0x20) TerrainHeightsData::intersect_ray<TerrainHeightSsbo>(); 地形命中即返回` + 否则 `Rapier::intersects_any`(Qbvh traverse_depth_first_node + RayIntersectionsVisitor,0x1408cdf80)。Rust=`self.raycast(...).is_some()`:缺地形短路分支、算法不同(cast_ray_and_get_normal 循环 vs Qbvh visitor)、返回语义不同。 |
| 0x1408cc390 | Raycasting::spherecast | world.rs `spherecast`(native=返 None 桩) | **未实现-需重做** | 原=完整实现:转发 RaycastWorld::spherecast(0x1408ccfc0)+ 地形处理 + 逐槽最近命中(带 discriminant=0xb 地形结果)。Rust 直接 `None`。RESTORE 自登记为桩。 |
| 0x1408cc620 | SubworldColliderHandle::new | world.rs `SubworldColliderHandle::new` | **判等**(附签名注) | 机器码 `mov rax,rcx; mov [rcx+8],dl; mov [rcx],r8d; mov [rcx+4],r9d; ret`。3 个字段写(index@+0←R8、gen@+4←R9、slot@+8←DL)逐条对应。注:原参数序=(slot:u8, index:u32, generation:u32);Rust=(index,generation,slot) 参数位置不同(字段映射一致)。另:原 struct 报 24B,Rust 12B,布局级未核对。 |
| 0x1408cc670 | RaycastWorld::mesh_intersect_collider_diff_publisher | world.rs 同函数 | **未实现-需重做** | 机器码 `lea rax,[rcx+0x3cb0]; ret` = 返回 `&self+0x3cb0`(tabloid Publisher 字段引用)。Rust 自造 `return 0 as u64`。返回值是编造的常量,且当前 struct 无 +0x3cb0 字段。 |
| 0x1408cc680 | RaycastWorld::legacy_raycast | world.rs `legacy_raycast`(返 None) | **未实现-需重做** | 原=调 Rapier::raycast(0x1408cdc60)返回 RaycastWorldResult(tag@+0x30)。Rust 直接 `None`,无任何查询。 |
| 0x1408cc870 | RaycastWorld::raycast | world.rs `RaycastWorld::raycast` | **未实现-需重做** | 原=转发 Rapier::raycast(0x1408cdc60):17 槽 u32 mask 遍历,每槽 `QueryPipeline::cast_ray_and_get_normal`,**最后一次命中槽覆盖(无跨槽 toi 比较)**,返回 RaycastWorldResult。Rust=内联循环但 **`toi < t` 取最近命中**(算法分歧)、mask u64(原 u32,disasm `MOV EAX,[RSP+0x70]` 读 32 位)、返回 Option<(f32,u32)>(无 position/normal)。 |
| 0x1408cec10 | RaycastMeshData::new_convex | world.rs `RaycastMeshData::new_convex` | **未实现-需重做** | 原:先 `utils::calculate_hash<ref<slice<UVec3>>>(points)` 存 hash@+0x70;`try_convex_hull(points)`→Some→`ConvexPolyhedron::from_convex_mesh`→**Box<ConvexPolyhedron>(0xd0)+vtable 存 @+0x20/+0x28**;tag=4@+0、kind=4@+0x10、+0x68=0x3f80000000000000、hash@+0x70;None 写 *out=0。Rust:`hash:0`(硬编码,不计算)、`SharedShape::convex_hull`→Arc<SharedShape>(构造路径/存储表示不同,非 Box<dyn Shape>)、struct 布局{tag,Option<Arc>,hash}与原~0x78B 布局不符。 |
| (无独立 VA;Default 为 glue) | RaycastWorld::new(构造器) | world.rs `RaycastWorld::new` | **未实现-需重做** | 原 RaycastWorld 布局:17×0x390 槽内联 @+0x20(至 +0x3cb0),Publisher@+0x3cb0(mesh_intersect 返回该处),+0x3d80 另有字段(Qbvh);slot 含 RigidBodySet/ColliderSet/QueryPipeline@+0x0/0x30/脏标志@+0x388。Rust=Vec<Subworld>{bodies,colliders,pipeline,dirty_tiles}+dirty:u64——布局与构造均不同,无 Publisher/0x3d80 字段。 |

### collision.rs(当前为改写,非原 collision.rs 结构)

| 原符号 | Rust | verdict | 分歧点 |
|---|---|---|---|
| SubworldIndex::from_usize_unchecked(0x14067dd80,机器码 `mov rax,rcx; ret` 无检查) | `SubworldIndex::new(slot)->Option<Self>`(检查 <17) | **未实现-需重做** | 原=无检查直接包装(usize→u8);Rust 自造带边界检查+Option 的构造,语义不等(checked vs unchecked)。 |
| SubworldIndex::to_bitflag(inline,1<<slot) | `to_bitflag()->u32`(1<<self.0) | **判等** | 位运算一致。 |
| (原 collision.rs 唯一业务) impl$0::build(0x140b685e0) | 无 | **未实现** | 当前 collision.rs 未实现原文件唯一业务函数。 |
| SubworldIndex 槽 17 / 空判别 0x11 / Terrain=5(mask 0x20) | 常量 | **判等**(常量锚定) | 17 槽:反编译 0x3c90/0x390=17、循环至 0x3cb0;空 0x11:raycast_simple 无命中写 *out=0x11;Terrain=5/0x20:intersects_any `(in_R9 & 0x20)`、raycast_w_terrain `(mask&0x20)`。 |

## 严重分歧摘要(供集成 owner)

1. **Raycasting::raycast 转发目标错**:原=raycast_w_terrain(0x1408cd430,含 Terrain 0x20 地形
   TerrainHeightsData SSBO 相交),Rust=纯 rapier 的 world.raycast。整条地形子世界查询链缺失。
2. **RaycastWorld::raycast 选择算法错**:原=跨槽最后一次命中覆盖(无 toi 比较);Rust=`toi<t`
   取最近。mask 原 u32,Rust u64。
3. **返回类型系统性不符**:原 RaycastWorld::raycast / legacy_raycast / Raycasting::raycast /
   spherecast 全部返回 `RaycastWorldResult`(~0x40B:position/normal/toi/discriminant@+0x34,
   地形命中 discriminant=0xb、无命中=2);Rust 侧统一 Option<(f32,u32)>/bool/None。库外语义
   (rapier 调用点参数与返回使用)因此无法判等。
4. **new_convex 数据流错**:hash 原为 calculate_hash 计算存 @+0x70,Rust 硬编码 0;构造路径
   try_convex_hull+from_convex_mesh→Box<dyn Shape> vs SharedShape::convex_hull→Arc。
5. **mesh_intersect_collider_diff_publisher 自造**:原返回 &self+0x3cb0(lea rax,[rcx+0x3cb0];ret),
   Rust 返回常量 0。
6. **RaycastWorld 布局错**:原 17 槽内联@+0x20 + Publisher@+0x3cb0 + 0x3d80 字段;Rust Vec<Subworld>
   动态布局,无对应字段。

## 未实现登记(无 Rust body,超出本次逐函数范围)

- world.rs:75 业务中仅 9 个有 Rust body 且其中 8 个判 未实现;余 ~66 业务未实现,包括
  raycast_w_terrain(0x1408cd430)、RaycastWorld::intersects_any(0x1408cd5b0)、Rapier::raycast
  (0x1408cdc60)、Rapier::intersects_any(0x1408cdf80)、Rapier::raycast_simple(0x1408ce180)、
  raycast_intersections_with_ray(0x1408cc8a0,结果 Vec<RaycastWorldResult2> 排序)、raycast_simple/
  raycast_simple_exclude_wall_id、legacy_spherecast(0x1408ccc90)、spherecast(0x1408ccfc0)、
  shape_overlaps_with/iter_shape_overlaps/decorator_overlaps_with_others、get_collider/
  get_collider_mesh/get_rapier_collider、on_collider_upserted(publish<ColliderDiff>)、
  rapier_hit_to_raycast_world_result(0x1408ce4a0)、upsert/remove_collider 族、flush_colliders、
  update_rapier_colliders、construct_arch_geo、on_level_reload、RaycastMesh 族、
  RaycastMeshData::new_indexed/new_unindexed/new_cuboid/new_sphere/new_compound、
  to_offset_collider_builder 族、into_transformed_shape 族、as_collider_shape、xz_flatten_aabb、
  dispose_collider_mesh_instance 等。
- intersect_meshes.rs(21,业务 4:find_mesh_mesh_intersections 0x140a97b70、intersect_mesh_mesh
  0x140a9afa0、should_intersect_collider_meshes 0x140a9b3c0、merge_segments_into_polylines
  0x1408f2850)。visualize_colliders.rs(6,业务 2:visualize_colliders 0x141326370、
  visualize_shape 0x141326930)。resources/rt(mod.rs 业务 11、bvh 1、bvh_builder 1)。
