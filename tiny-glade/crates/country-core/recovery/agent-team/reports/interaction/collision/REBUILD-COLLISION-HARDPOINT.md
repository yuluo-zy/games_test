# REBUILD-COLLISION-HARDPOINT — collision 剩余收口（hardpoint + rt 设备面 + find_mesh 跨域）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（fresh 反编译 + 逐语句判等 + 重建前分类）
依据: reports/interaction/collision/{REBUILD-DEEP,REBUILD-COLLISION-TYPES,REVIEW-COLLISION-TYPES}.md
证据: planning/impl-collision-hardpoint/results/
  - b1-rt-findmesh.result.json（fresh：update_rapier_colliders callee 面 41 + rt 7 函数 +
    find_mesh_mesh_intersections 7783B）
  - b2-hardpoint-disasm.result.json + hardpoint-update_rapier_colliders.disasm.txt（4630 指令）
  - h1-reva-hardpoint.result.json（`--write` worker `impl-collision-hardpoint` 签名修复 + save）
  - h2-reva-pcode.result.json（High P-code 尝试：同 varnode 错误）
  - b4-upload-disasm + b5-read-strings（RtTrianglesSsbo/RtTriangleMetaSsbo/RtBvhSsbo 名锚定）
  - find_mesh_mesh_intersections.dc.txt（fresh 全量反编译）
编译: 全仓 `cargo check -p country-core --lib`（default + `--features native-physics`）**0 error**
（rt.rs/collision.rs 无新增 warning）。rt.rs 8 单测 + collision 13 单测 PASS
（`--features native-physics`）。期间并行 agent 的 rhapsody `swapchain.rs present_mode` WIP
曾临时阻塞全仓 check（15:12 修改），随后自愈——非本模块，红线未改他模块。

## 批 1 — rt 设备面 fresh 锚定 + 实现（resources/rt.rs）

### 类型/布局修正（fresh 锚定，修正旧登记）

| 类型 | 旧登记 | fresh 锚定 | 处置 |
|---|---|---|---|
| `RtMesh` | 0x50 B 但字段错（+0x08=(1.0,0.0)、+0x38=4） | `get_mut` 0x1400cce50 默认写序列：`+0x00 scale=1.0、+0x04 0、+0x08 u64 0、+0x10 0x3f80000000000000(f64 1.0)、+0x18 0、+0x20 0、+0x28 (1.0,1.0)、+0x30 0、+0x38 0、+0x40 0、+0x48 kind=4`；`#[repr(C)]` 0x50 B | **修正并落地**（scale/kind 语义锚定，其余 `_unk*` 登记） |
| `RtMeshHandle` | 1 B 单元枚举 | 键 2×u64（hash_one + 键比较）；`impl$2::from` = `{1,值}` | **修正为 16 B `{tag,payload}`** |
| `DirtyCow` | `{dirty@+0, inner@+8}` | `get_mut` 0x1415626f0 写 +8 脏字节、读 +0 Arc 指针 | **修正为 `{inner@+0, dirty@+8}`** |
| SSBO 名 | 未锚定 | upload_rt_world disasm `LEA RCX,[0x142a9dc30/0x142a9cf75/0x142a9e726]` + read_memory → `RtTrianglesSsbo`(0x2c)/`RtTriangleMetaSsbo`(0x2f)/`RtBvhSsbo`(0x26) | **锚定落地**（0x18/4/0x50 元素） |

### 函数实现

| VA | 函数 | verdict | 实现 |
|---|---|---|---|
| 0x1400cce50/0x141562aa0/0x1415626f0 | `RtWorld::get_mut`/`DirtyCow::get_mut` | 结构判等 | `HashMap::entry` + `or_insert_with(RtMesh::empty)` + `DirtyCow::get_mut`（脏字节 + Arc COW）；Rust 侧用标准 HashMap 等价 hashbrown SIMD 探测（哈希表内部登记） |
| — | `RtWorld::get` | 结构判等 | 只读 `meshes.get().inner`（`bvh_builder_worker` `Values<HashMap>` 消费面） |

### 登记（rt 设备面，未实现）

| VA | 函数 | 登记原因 |
|---|---|---|
| 0x1408bc650 | `update_rt_world` | `HashMap` 移除循环（`RawTable::remove_entry`）+ `RwLock` + `from_iter<Map<IterMut,consume>>` 重建 + log 写 inner +0x68..+0xa7；`consume` 语义与 inner 精确偏移登记，body 未落地 |
| 0x1408bcff0 | `upload_rt_world` | `FrozenMap::get<SsboId>`（键 0x142c2e3d8/0x142c2e270/0x142c2e350）+ 名字断言 + `rhapsody::raw::device::GraphicsDevice::update_buffer_bytes`（0x18/4/0x50·count）+ 计数写 + "Out of pre-allocated buffer space for ray tracing-data" panic；设备/库接线 host |
| 0x140b0da50 | `build_gpu_bvh` | `obvhs::cwbvh::builder::build_cwbvh_from_tris`（obvhs 未入 workspace）+ 计时 log（"Griffin has built us a GPU BVH i…"）+ `from_iter<GpuRtTriangle, Map<Iter<u32>,closure_env_1>>`（GpuRtTriangle=InputTriangle+0x24） |
| 0x140b24d50 | `bvh_builder_worker` | `RwLock` 循环（+0x14 状态字节）+ `from_iter<InputTriangle, FilterMap<FlatMap<Values<HashMap>,Zip<ChunksExact<Vec3>,Repeat<u32>>>,…>>`（RtMesh 顶点 `ChunksExact<Vec3>` 消费）+ `build_gpu_bvh`；InputTriangle/closure 语义未锚定 |

单测: 8 PASS（探针验证：DirtyCow 偏移、RtMesh 0x50 默认值、RtMeshHandle 16B、get_mut 插删）。

## 批 2 — update_rapier_colliders decompile-hardpoint（未收敛，行为级骨架）

- **收敛尝试**：ghidra shared + ReVa `--write` worker（签名修复 `void update_rapier_colliders
  (longlong * this)` + save，h1 step0 success）fresh 仍 `Unable to find unique hash for varnode`；
  ReVa High P-code（h2）在 HighFunction 构建阶段同错误，不可得。
- **分段 disasm**：b2 + `hardpoint-update_rapier_colliders.disasm.txt`（4630 指令）→ call 序列
  13 段分析（puffin scope + 5 组件族 collider 生命周期），骨架落盘
  `update_rapier_colliders-skeleton.md`。
- **verdict**：`未实现-需重做`（行为级骨架，登记不伪造）。41 callee 面（b1）已备，后续可经
  typed prototype 逐段判等。

骨架核心（简）：bevy `Changed/Added<…>` 查询循环 → 5 组件族（矩形 cuboid、arch、
圆/地板 new_indexed、屋顶触发体积 new_unindexed）构造 `RaycastMeshData` → `upsert_collider
<Entity,(Vec3,Quat)>` → `dispose_collider_mesh_instance` 释放移除实体。

## 批 3 — find_mesh_mesh_intersections fresh 反编译 + 跨域登记

- **fresh 反编译**：b1 + find_mesh_mesh_intersections.dc.txt（7783B）全量分析。
- **域内 mesh 相交部分**：图剪枝（`Subscriber::take<ColliderDiff>` → `GraphMap::remove_node`）、
  逐节点 AABB（`ColliderSet::get_mut_internal` + `compute_aabb` + `SimdAabb::splat`）+ 候选槽
  BVH 遍历（`Qbvh::traverse_depth_first_node<…, intersect_newly_added_colliders::closure_env_0>`）
  ——**构建块均已实现**（`should_intersect_collider_meshes`/`intersect_mesh_mesh`/
  `merge_segments_into_polylines`，REBUILD-DEEP 批 E），装配依赖下列跨域/未锚定类型。
- **跨域消费登记**：
  1. `BirdNestCandidateLocation`（`country_core::systems::collision::BirdNestCandidateLocation`，
     类型未恢复；spec_extend `FlatMap<AllEdges<GraphMap>, Cloned<Iter<BirdNestCandidateLocation>>>`
     从图边载荷收集）——边载荷 `MeshIntersectionArtifacts` 需含 `Vec<BirdNestCandidateLocation>`。
  2. `SparseInstanceBuffer<BrickTransformSsbo>`（FrozenMap::get + name 0x3a 断言）+ 折线→砖
     铺放（`SparseInstanceBufferWriter::push` + 中点步进 0.5 + 法线/切向量）×100 取整）+ 
     `take_out_waste<BrickTransformSsbo>`——`BrickTransformSsbo` 为 geometry::instanced_wall 跨域。
  3. `SparseInstanceBufferAllocation<SparkleSsbo>`（drop_in_place）——resources/sparkle 跨域。
  4. `ColliderDiff` 载荷（移除句柄列表）未恢复 → 图剪枝入口待类型。
- **verdict**：`未实现-跨域登记`（域内装配 + 构建块就绪；跨域类型到位后按本报告装配）。

## 编译状态

- 全仓 `cargo check -p country-core --lib`（default + native-physics）0 error；rt.rs 8 +
  collision 13 单测 PASS。期间 rhapsody `swapchain.rs` WIP 曾短暂阻塞（并行 agent，已自愈）。
- rt.rs 独立探针验证（glam+std）：8 单测 PASS（DirtyCow 布局 16B、RtMesh 0x50、
  RtMeshHandle 16B、get_mut 插删），与全仓结果一致。

## 剩余登记汇总

- update_rapier_colliders：行为级骨架 + disasm 落盘，待 typed prototype 判等。
- find_mesh_mesh_intersections：跨域类型（BirdNestCandidateLocation / BrickTransformSsbo /
  ColliderDiff 载荷 / Sparkle）到位后装配。
- rt 设备面：update_rt_world / upload_rt_world（host GraphicsDevice/AssetSsboLibrary 接线 +
  元素字段布局）、build_gpu_bvh / bvh_builder_worker（obvhs 入 workspace）。
