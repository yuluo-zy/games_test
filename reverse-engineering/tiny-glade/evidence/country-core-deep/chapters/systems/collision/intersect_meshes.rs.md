# systems/collision/intersect_meshes.rs

> 文件作用：**网格-网格相交检测：parry QBVH 两两三角相交→交点段合并为折线→伪影图+tabloid 发布**

函数总计 21：业务 4 / 内联无独立符号 6 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections` | 0x140a97b70 | 56 | 系统:相交 | O：find_mesh_mesh_intersections——新 collider 与相关网格相交→伪影图更新+折线化+发布 | collision-eco\decomp\decomp-0x140a97b70.txt |
| `country_core::systems::collision::intersect_meshes::intersect_mesh_mesh` | 0x140a9afa0 | 269 | 几何/碰撞 | O：intersect_mesh_mesh——parry traverse_bvtt 两网格三角两两相交 | collision-eco\decomp\decomp-0x140a9afa0.txt |
| `country_core::systems::collision::intersect_meshes::should_intersect_collider_meshes` | 0x140a9b3c0 | 444 | 查询/计算 | O：should_intersect_collider_meshes——按几何源过滤 | collision-eco\decomp\decomp-0x140a9b3c0.txt |
| `country_core::systems::collision::intersect_meshes::polyline::merge_segments_into_polylines` | 0x1408f2850 | 527 | 几何/碰撞 | O：polyline::merge_segments_into_polylines（44KB）——交点段按端点连通性合并为折线 | collision-eco\decomp\decomp-0x1408f2850.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::collision::intersect_meshes::subscribe_mesh_mesh_intersections`（行 46）— I：网格相交订阅（tabloid 消费者）
- `country_core::systems::collision::intersect_meshes::from_parry_triangle`（行 308）— I：parry 三角→游戏三角转换（intersect_mesh_mesh 内联）
- `country_core::systems::collision::intersect_meshes::intersect_newly_added_colliders`（行 324）— 几何/碰撞
- `country_core::systems::collision::intersect_meshes::remove_invalid_polylines`（行 497）— 删除/清理
- `country_core::systems::collision::intersect_meshes::polyline::VertexId::as_usize`（行 817）— O：VertexId::as_usize——u32→usize
- `country_core::systems::collision::intersect_meshes::on_exit_game_state`（行 852）— 系统:事件响应

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`eq` 等 11 项

## 实现方式（算法级）

处理链（新 collider 到达 → 伪影图更新 → 折线化 → tabloid 发布）：
1. `find_mesh_mesh_intersections`（0x140a97b70，系统）：订阅/消费 collider 变更 → 对新 collider 与其相关网格 `intersect_mesh_mesh` → 交点段收集 → `merge_segments_into_polylines`（0x1408f2850，44KB）按端点连通性合并 → 更新 MeshIntersectionGraph（GraphMap<SubworldColliderHandle, Artifacts> 待核）+ 发布。
2. `intersect_mesh_mesh`（0x140a9afa0）：parry `traverse_bvtt` 两网格三角两两相交（from_parry_triangle 转换）。
3. `should_intersect_collider_meshes`（0x140a9b3c0）：按几何源（RaycastGeoSource 判别）过滤是否参与相交检测。
4. `merge_segments_into_polylines`（0x1408f2850）：VertexId u32 端点 → 段-端点邻接 → 贪心链式合并（重建 stub 返空，真实 44KB）。
