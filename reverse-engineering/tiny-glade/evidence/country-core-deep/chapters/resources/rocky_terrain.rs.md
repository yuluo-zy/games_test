# resources/rocky_terrain.rs

> 文件作用：**岩质地层网格：RockyTerrainMesh 资源 + JSON 中间结构（LoadedRockyTerrainVertex/Triangle）→ 两个 SSBO（RockyTerrainVertexSSBO/TriangleSSBO），displace_rocky_terrain 每帧 GPU 变形。**

函数总计 10：业务 2 / 内联无独立符号 0 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::rocky_terrain::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>` | 0x1417c3650 | 18 | JSON 反序列化 visitor（derive 胶水） | O | terrain-water/DIGEST.md
| `country_core::resources::rocky_terrain::RockyTerrainMesh::load` | 0x1408bbf50 | 34 | fs::read_to_string → serde_json 解析 → add_new_with_data<VertexSSBO/TriangleSSBO>（顶点源 0x24B→0x30B、三角形 0x10B→0x20B） | O | terrain-water\decomp\decomp-0x1408bbf50.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`deserialize`、`expecting`、`visit_map`、`visit_seq`、`visit_str` 等 8 项

## 实现方式（算法级）

`RockyTerrainMesh::load`（0x8bbf50）：`fs::read_to_string`（FUN_140d59140）→ serde_json 解析 → `add_new_with_data<RockyTerrainVertexSSBO/TriangleSSBO>`；顶点源记录 0x24B→0x30B、三角形 0x10B→0x20B。JSON 中间结构 LoadedRockyTerrainVertex/Triangle（derive 胶水）。displace_rocky_terrain 每帧 GPU 变形。
