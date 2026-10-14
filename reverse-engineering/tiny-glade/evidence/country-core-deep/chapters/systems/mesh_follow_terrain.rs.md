# systems/mesh_follow_terrain.rs

> 文件作用：**网格贴地系统：Query<(&MeshFollowTerrain, &mut Transform, &Handle<Mesh>)> + 地形资源；取 mesh 'Vertex_Position' 属性计算位置 → 采样地形高度 → Transform.y 更新；句柄无效 panic 'MeshFollowTerrain got an invalid mesh handle'。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::mesh_follow_terrain::mesh_follow_terrain` | 0x140a68430 | 12 | ECS Query 迭代 + 2×Res（地形高度数据）；FUN_14092d300 取 mesh、FUN_140a07b30 取几何、FUN_1412fd550 位置计算（内联地形采样）→ Transform 更新；无效句柄 panic | O | terrain-water\decomp\decomp-0x140a68430.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
