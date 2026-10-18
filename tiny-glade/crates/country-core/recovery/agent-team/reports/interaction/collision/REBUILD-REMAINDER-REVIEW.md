# REVIEW — collision 余量(REBUILD-REMAINDER 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-collision-remainder REBUILD-REMAINDER.md(fresh 反编译逐批 ≤20);
默认 + `--features native-physics` 双构建 exit 0(独立复验,10 警告均他模块既有)。

## 逐项复验

### ① 分类 — PASS(诚实)
world.rs ~66 = **11 实现** + **~48 登记**(缺类型/布局:RaycastMeshData 布局、RaycastGeoSource/
OffsetColliderTransform/DirtyTiles 类型族、collider 生命周期 monolith)+ **~8 宏胶水**(derive/
fmt/Index/注册)。intersect_meshes(21,业务 4)/visualize_colliders(6,业务 2)/rt(13)全登记
(依赖缺失类型)。模块文件已建立(分类表落盘)✓。

### ② 11 实现 — PASS(全部在 src)
- `intersects_any`(0x1408cd5b0):转发 Rapier::intersects_any(0x1408cdf80,17 槽 Qbvh,命中
  置 found 即返回)✓。
- `raycast_simple`(0x1408ccc20)→ Rapier::raycast_simple(0x1408ce180,17 槽 cast_ray solid=1
  最后命中覆盖,未中首字节 0x11)+ `RaycastSimpleResult{handle[3],position,toi}` ✓。
- `raycast_simple_exclude_wall_id`(0x1408ccc50):转发 + `QueryFilter::exclude_collider`
  (wall→collider handle 映射在调用方,登记)✓。
- `legacy_raycast_w_params`(0x1408cc770):Rapier::raycast + 未中判别 2 写 +0x30 ✓。
- `legacy_spherecast`(0x1408ccc90):17 槽 cast_shape + 判别 0/2 写 +0x30、命中 normal 写 0 ✓。
- `raycast_intersections_with_ray`(0x1408cc8a0):地形**无条件** intersect_ray(判别 0xb)+
  17 槽 Qbvh 遍历收集 + `sort_unstable_by` toi 升序;+0x3d80 Qbvh 经
  QueryPipeline::intersections_with_ray 等价(沿用 REBUILD.md 约定)✓。
- `Rapier::subworlds`(0x1408cdc40):fresh {start=+0x20,end=+0x3cb0,0}=17 槽区间;Rust 返回
  0..17(消费方登记待接)✓。
- `xz_flatten_aabb`(0x1408d98a0):disasm 8 条指令→Aabb2{min.x,min.z,max.x,max.z},跳过 y ✓。

### ③ 登记关键结构分歧 — PASS(诚实,不伪造)
- `RaycastMeshData` 现 {tag,kind,shape,scale,hash} 与机器码 new_indexed/new_unindexed 输出
  (+0x00..+0x70:顶点 Vec@+0/shape@+0x20/法线@+0x30/uv@+0x40…)**不符,已登记"布局须先锚定",
  不按新布局实现**(new_cuboid/new_sphere/new_compound 同)✓。
- `SubworldColliderHandle` partial_cmp:fresh 仅按 +8 slot 字节比较(Less/Equal/Greater),
  Rust `#[derive(PartialOrd)]` 按字段序比较——**分歧登记**(graph key 排序语义未验证,不改动)
  ✓。
- `RaycastWorld` 原含 collider 存储 Vec@+0x08/+0x10 + DirtyTiles 槽缓冲——依赖类型缺失,
  登记 ✓。

### ④ 编译 — PASS
双特征(默认 + native-physics)exit 0,10 警告均他模块(audio/save/ui);此前 rhapsody E0061
并行 WIP 暂态已消。collision rapier API 面(cast_ray/cast_shape/intersections_with_ray/
from_raw_parts/exclude_collider/ShapeCastOptions)探针单独验证 ✓。

## 结论与 DISPATCH 建议

- collision 余量重建复验通过:**11 实现判等 + 分类诚实 + 结构分歧登记**,双构建 0 error。
- 建议:collision 余量 → 闭合(实现层面);待续登记项(RaycastMeshData 全布局锚定→5 构造器
  +RaycastMesh 族+rapier_hit_to_raycast_world_result barycentric、collider 生命周期、
  RaycastGeoSource/OffsetColliderTransform 类型)归后续卡。
