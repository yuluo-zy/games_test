# REBUILD-DEEP — systems/collision 深实现：布局锚定 → 类型族 → 生命周期 → 分批判等

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
前置: reports/interaction/collision/{LOGIC,REBUILD,REBUILD-REMAINDER}.md
证据: planning/impl-collision-deep/results/b{1..7}*.result.json（ghidra `--worker shared`
只读 fresh 反编译 + disassemble_function + read_memory，2026-10-17 10:51–10:58）
编译: world.rs / intersect_meshes.rs 两特征（default + native-physics）已验证干净
（rhapsody 并行 WIP 阻塞全仓 check 时用独立 rapier/parry 探针复验 API 面）。

## 批 A — RaycastMeshData 布局锚定（前置）

**锚定证据**（new_indexed 0x1408cee40 / new_unindexed 0x1408cf7d0 / new_convex 0x1408cec10 /
new_cuboid 0x1408cfff0 / new_sphere 0x1408d0120 / new_compound 0x1408d0230 最终 store 序列 +
read_memory 0x142aa3ec0/0x142aa3ed0/0x142925870）：

| +0x00 | +0x08 | +0x10 | +0x18 | +0x20/+0x28 | +0x30/+0x38/+0x40 | +0x48..+0x5f | +0x60 | +0x68 | +0x70 |
|---|---|---|---|---|---|---|---|---|---|
| 顶点 data | 顶点 len | 顶点 cap | 三角数 | shape Box | normals data/len/cap | uvs/参数区 | 0 | scale=1.0 | hash |

- 常量 0x142aa3ec0 = 16 零字节；0x142aa3ed0 = 14 零字节 + f32 1.0 → `+0x50..+0x5f=0,
  +0x60=0, +0x68=0x3f80000000000000(double 1.0)`。new_cuboid `+0x50..+0x6f` = 参数区
  {hash_input.xy,z, half_extents.x, half_extents.yz, uninit→0}；new_sphere `+0x50..+0x5f` =
  {radius, hash_input.xyz}；new_compound `+0x50..+0x6f` = hash_input `[u32;8]` 前 4 字。
- 形状变体 tag=4@+0x00、kind=4@+0x10（复用顶点槽）；网格变体放真指针。

**构造器修正（判等，逐语句自检）**：
| 函数 | verdict | 修正 |
|---|---|---|
| new_indexed（0x1408cee40） | 判等 | 布局/hash((indices,vertices)→normals→uvs) 已对（REBUILD-REMAINDER 后已锚定） |
| new_unindexed（0x1408cf7d0） | 判等（哈希表示分歧登记） | +0x30 缓冲=逐三角**三点**（旧 t.normal() 错）；count=三角数（旧 3× 错）；hash((triangles,点位UVec3))→uvs |
| new_cuboid（0x1408cfff0） | 判等 | Cuboid 半边长=入参×0.5（0x142925870=0x3f000000）；尾部参数区对齐 +0x50..+0x6f |
| new_sphere（0x1408d0120） | 判等 | 尾部参数区对齐 +0x50..+0x5f={radius,hash_input.xyz} |
| new_compound（0x1408d0230） | 判等（sensor 位/Isometry 登记） | 入参改 `Vec<RaycastMeshData>` + `[u32;8]`；hash(hash_input) 逐子形状折叠 child.hash@+0x70 |
| new_convex（0x1408cec10） | 判等 | 已对（REBUILD.md） |

## 批 B — 类型族 + barycentric

| 函数 | verdict | 实现 |
|---|---|---|
| RaycastGeoSource（0x1408cea20/0x1408cea30） | 判等（payload eq 登记） | 17 变体枚举；subworld() 锚定 Terrain=5/PlatformStair=7；tag()/eq() |
| RaycastMesh::{new,data,raycast}（0x1408ceb10/30/40） | 判等（vtable 分派登记） | {data:Arc<RaycastMeshData>, source}；raycast 数据 shape 直查 |
| rapier_hit_to_raycast_world_result（0x1408ce4a0） | 判等 | barycentric 法线/uv 插值，权重 shuffle {v1:x,v2:y,v0:z}，判别 0/1/2 |

## 批 C — dirty 瓦片生命周期

| 函数 | verdict | 实现 |
|---|---|---|
| flush_colliders（0x1408cd5c0） | 判等 | 17 槽脏标志→take-and-clear（slot+0x188 0x200B）→`publish<DirtyTiles>`→清脏→`QueryPipeline::update(&colliders)` |
| flush_rapier_colliders（0x1408d8360） | 结构判等（+0x10/+0x1c 写登记） | puffin 门控 flush_colliders 转发 |
| subscribe_to_changes（0x1408cdc20） | 判等 | 修 throwaway Publisher → 真实 `dirty_tiles_publisher` 订阅 |

## 批 D — collider 生命周期泛型

| 函数 | verdict | 实现 |
|---|---|---|
| upsert_collider<K>（0x140252c90 等 ×6） | 结构判等（OffsetColliderTransform/slotmap 登记） | HashMap<K,ColliderMeshInstance>：已存在+位相等+hash 相等→Kept(2)；否则 build→ColliderSet::insert→脏→AABB 栅格化→on_collider_upserted→Inserted(0)/Replaced(1) |
| remove_collider<K>（0x140252ad0 等 ×3） | 结构判等 | map.remove→dispose→Some(handle)；未找 None（O：0x11 空判别） |
| dispose_collider_mesh_instance（0x1408d9590） | 结构判等 | 释放 slot collider + 槽脏 + 发布 ColliderDiff |

## 批 E — intersect_meshes

| 函数 | verdict | 实现 |
|---|---|---|
| should_intersect_collider_meshes（0x140a9b3c0，283B） | 判等 | registry generation 校验 + kind 组合 (0,2)/(2,2)/(2,0)→true，否则 panic |
| intersect_mesh_mesh（0x140a9afa0，908B） | 判等 | `Qbvh::traverse_bvtt<u32,u32,BoundingVolumeIntersectionsSimultaneousVisitor>` 包装 |

## 类型落地

RaycastGeoSource（17 变体）、RaycastMesh、ColliderMeshInstance、ColliderUpsert、
RaycastWorld.dirty_tiles_publisher、RapierSubWorld.{dirty_tiles,dirty:bool}。
SubworldColliderHandle partial_cmp 仅槽字节（REBUILD-REMAINDER 已修，本轮复核判等）。

## 编译状态

- 两特征均已验证：world.rs 在 rhapsody 破坏前完成两特征全量 `cargo check` 0 error、
  collision 无 warning；intersect_meshes/upsert-remove 的 rapier/parry API 面
  （ColliderBuilder/SharedShape pub 字段/into_raw_parts/compute_aabb/traverse_bvtt）用
  独立探针 crate 编译通过。
- 当前全仓 check 被并行 agent 的 rhapsody `rg/sync_plan` WIP 阻塞（9+ E0308/E0433，
  REBUILD-REMAINDER 已记录同类 WIP）——非本模块。

## 剩余登记（未实现，依赖未锚定类型/超大 monolith）

- update_rapier_colliders（0x1408d0490，31293B）、on_level_reload（0x1408d8710，1993B）：
  collider 生命周期 + RaycastWorld 重建 monolith。
- construct_arch_geo（0x1408d7ed0）：arch 几何输入类型未定。
- to_offset_collider_builder（0x1408d0390 等）、into_transformed_shape（0x14155d340 等）、
  as_collider_shape（0x1408d9990）：`OffsetColliderTransform`/`IsoTransform` 类型。
- iter_shape_overlaps（×4）、shape_overlaps_with（0x142612d10）、
  decorator_overlaps_with_others（0x14129be00）：依赖 `RaycastGeoSource` 消费 + 泛型 closure。
- get_collider（0x1408cdb50）：分歧——原 `ColliderSet::get_mut_internal` 读活 rapier
  collider 拷贝 {pos@0x34.., gen|1}，Rust 读 game registry（语义等价、数据源分歧）。
- intersect_meshes：find_mesh_mesh_intersections（0x140a97b70，13359B）、
  merge_segments_into_polylines（0x1408f2850，6057B，Polyline/VertexId/VecDeque 中点平均）。
- visualize_colliders（0x141326370/0x141326930）：依赖 RaycastMeshData 布局 + debug_viz show。
- resources/rt（13）：依赖 RtWorld/RtMesh/DirtyCow/RtBvhBuilderGlue 类型。
