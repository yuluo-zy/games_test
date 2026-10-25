# REVIEW — collision 深实现(REBUILD-DEEP 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-collision-deep REBUILD-DEEP.md(5 批,fresh 反编译 + disassemble + read_memory);
编译注:REBUILD-DEEP 已证两特征(collision world.rs/intersect_meshes.rs)0 error、无 warning
于 rhapsody 破坏前;现全仓被 rhapsody `rg/sync_plan` WIP 阻塞(10 error,他组在途),非本模块。

## 逐批复验

### 批 A RaycastMeshData 布局锚定 — PASS
布局表(+0x00 顶点 data/len/cap、+0x10 三角数、+0x20 shape、+0x30 normals、+0x40 uvs、
+0x48..+0x5f 参数区、+0x60=0、+0x68=1.0、+0x70 hash)经 read_memory 常量锚定
(0x142aa3ec0/3ed0/0x142925870)✓。六构造器:
- new_indexed(0x1408cee40)判等;new_unindexed(0x1408cf7d0)+0x30=三角三点(旧 t.normal() 错)、
  count=三角数(旧 3× 错)、hash((triangles,UVec3))→uvs;new_cuboid 半边长×0.5 + 尾部参数区
  对齐;new_sphere 参数区{radius,hash_input.xyz};new_compound Vec<RaycastMeshData>+[u32;8]、
  hash 逐 child;new_convex 复核已对 ✓。

### 批 B 类型族 + barycentric — PASS
- `RaycastGeoSource` 17 变体枚举(变体名 U,subworld 锚定 Terrain=5/PlatformStair=7)✓。
- `RaycastMesh::{new,data,raycast}`({data:Arc<RaycastMeshData>, source},raycast shape 直查)✓。
- `rapier_hit_to_raycast_world_result`:barycentric 法线/uv 插值,**权重 shuffle
  {v1:x,v2:y,v0:z}**,判别 0/1/2 ✓。

### 批 C dirty 瓦片 — PASS
`flush_colliders`(0x1408cd5c0):17 槽脏标志→take-and-clear(slot+0x188 0x200B)→
publish<DirtyTiles>→清脏→QueryPipeline::update ✓;`flush_rapier_colliders` puffin 门控
转发(+0x10/+0x1c 写登记);`subscribe_to_changes`(0x1408cdc20)修 throwaway→真实
`dirty_tiles_publisher` 订阅 ✓。

### 批 D collider 生命周期 — PASS(结构判等)
`upsert_collider<K>`:HashMap<K,ColliderMeshInstance> 已存在+位相等+hash 相等→**Kept(2)**;
否则 build→ColliderSet::insert→脏→AABB 栅格化→on_collider_upserted→**Inserted(0)/Replaced(1)**
(ColliderUpsert enum)✓;`remove_collider` map.remove→dispose→Some/None(0x11 空判别);
`dispose_collider_mesh_instance` 释放 slot collider + 槽脏 + publish<ColliderDiff> ✓。

### 批 E intersect_meshes — PASS
`should_intersect_collider_meshes`(0x140a9b3c0):registry generation 校验 + kind 组合
(0,2)/(2,2)/(2,0)→true 否则 panic ✓;`intersect_mesh_mesh`(0x140a9afa0):
`Qbvh::traverse_bvtt<u32,u32,BoundingVolumeIntersectionsSimultaneousVisitor>` 包装 ✓。

### 剩余登记诚实 — PASS
update_rapier_colliders(31293B)/on_level_reload(1993B) monolith、construct_arch_geo、
to_offset_collider_builder/into_transformed_shape/as_collider_shape(OffsetColliderTransform/
IsoTransform 类型)、iter_shape_overlaps×4/shape_overlaps_with/decorator_overlaps、
get_collider(读 registry vs 活 rapier collider 数据源分歧)、find_mesh_mesh_intersections/
merge_segments_into_polylines、visualize_colliders 2、resources/rt 13——全登记,不伪造 ✓。

## 结论与 DISPATCH 建议

- collision 深实现复验通过:**5 批(布局锚定/类型族/脏瓦片/生命周期/intersect_mesh)闭合**,
  两特征于 rhapsody 破坏前 0 error;现 rhapsody `rg/sync_plan` WIP 阻塞为他组在途。
- 建议:collision 深收口 → 闭合(实现层面);剩余 monolith(update_rapier_colliders/
  on_level_reload/find_mesh_mesh_intersections)与 OffsetColliderTransform/IsoTransform
  类型族由后续卡推进;rhapsody 断点解除后重跑全量。
