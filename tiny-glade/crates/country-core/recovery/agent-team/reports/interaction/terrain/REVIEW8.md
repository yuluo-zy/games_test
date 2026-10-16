# REVIEW8 — systems/terrain(卡B 终验 → 闭合·mask 资源+rocky_terrain+world_raster 已接部分)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md 七.卡B 节;`cargo check --lib` exit 0(独立复验;协调器所述
"全 lib 12 error"系 wall 组 decorator 在途暂态,现状态已消,仅 3 警告含 time_of_day
HOUR_TO_MS dead-code——suncalc 未接所致,良性)。

## 核对

### ① mask 四类 GAP 闭合 — PASS(卡7 阻塞解除)
- `TerrainRockMaskSsbo`/`GardenMaskSsbo`/`WaterExcludeClutterMaskSsbo`:StorageElement
  (NAME=stringify、BUFFER_LEN=0x8000、MAPPED_MEMORY_READBACK)+ ssbo_library::SsboType
  (MappedStorageBuffer)✓。
- `WaterMaskSsbo`:补库内 `SsboType`+`StorageElement`(原只有 pass_resource_access::SsboType)✓。
- `TerrainReadbackSsbos`:六类关联类型全映射(Height/VisualHeight/RockMask/WaterMask/
  WaterClutter/Garden),`read_terrain_data::<TerrainReadbackSsbos>` 宿主可调,卡7 阻塞解除 ✓。
- BUFFER_LEN=0x8000 = 512×512 位打包 mask /8,u32 元素 ✓(与 MASK_READBACK_THREADS=0x2000 一致)。

### ② rocky_terrain.rs — PASS(结构)
`RockyTerrainMesh::load`(0x1408bbf50):读 JSON → serde 解析 LoadedRockyTerrainVertex
0x24B→0x30B、LoadedRockyTerrainTriangle 0x10B→0x20B → add_new_with_data 注册两 SSBO
"Rocky Terrain Vertices"/"Rocky Terrain Triangles";首 qword = triangle_groups(displace
消费)✓。GPU 打包源布局(JSON 字段名)待取证(诚实)。

### ③ world_raster — PASS(已接部分)
record_raster_history + 上传消费既有;开始/续画/提交/历史提取已接,本批复核 OK 未加深。
完整状态机逐分支未重审(诚实登记)。

### ④ 待取证登记诚实 — PASS
RockyTerrain GPU 打包源布局、EditType::Terrain 真 tag(占位 14,待卡C 协调)、world_raster
逐分支未重审——全登记,无冒充。

## 结论与 DISPATCH 建议

- 卡B 复验通过:**mask 资源 + rocky_terrain + world_raster 已接部分闭合**,卡7 ReadbackSsbos
  阻塞项解除。
- 建议:卡B → 闭合;卡C(EditType::Terrain 真 tag 协调)续派;world_raster 完整状态机与
  RockyTerrain GPU 打包源布局由后续子卡补证。
- 全 lib 编译现状态 exit 0(3 警告,time_of_day HOUR_TO_MS dead-code 为 suncalc 未接的良性
  冗余)。
