# WP8 契约 2/4 — `resources::rocky_terrain`

版本 **B**。数据源：skeleton bundle `resources/rocky_terrain.rs.{pdb,structures,enriched}`、
反编译（本轮 Ghidra：`RockyTerrainMesh::load`@0x1408bbf50、`displace_rocky_terrain`@0x140a3ee80、
`read_terrain_data`@0x14098c680 的 `rocky_terrain_shader` 绘制）。

标注：**O**=反编译/PDB 直接支持；**I**=调用链组合；**U**=未恢复（typed gap）。

## 1. 命名实体与来源文件

`crates/country-core/src/resources/rocky_terrain.rs`，pdb_sha256 `2d9686b8…`。
PDB 命名实体（structures.json.gz）：

| 实体 | 证据 |
|---|---|
| `RockyTerrainMesh` | 资源（O；`Resource` 插入于资源注册，I） |
| `LoadedRockyTerrainVertex` / `LoadedRockyTerrainTriangle` | JSON 反序列化中间结构（O 名；`deserialize` 行 12） |
| `RockyTerrainVertexSSBO` / `RockyTerrainTriangleSSBO` | SSBO 标记类型（O；displace 写、load `add_new_with_data`） |
| `RockyTerrainMeshVertCount` | 顶点数 newtype（O 名） |
| `TerrainEntities` | 地形实体查询集合（O 名；`read_terrain_data` 绘制地形/岩体用，I） |

## 2. `RockyTerrainMesh::load`（O，0x1408bbf50，行 34）

```
fn load(…path…, …) -> Result<RockyTerrainMesh>：
  s = std::fs::read_to_string(path)                       // O：std::fs::read_to_string::inner
  mesh = serde_json::from_str::<RockyTerrainMesh>(&s)     // O：from_str<RockyTerrainMesh>，错误→anyhow::Error
  顶点：读 loaded 数组（每元素 0x24B 源 → 0x30B 顶点记录），
        AssetSsboLibrary::add_new_with_data<RockyTerrainVertexSSBO>   // O
  三角形：读 loaded 三角形数组（每元素 0x10B 源 → 0x20B 记录，部分 16B 清零），
        AssetSsboLibrary::add_new_with_data<RockyTerrainTriangleSSBO> // O
```

顶点记录 0x30B：观察拷贝 {0x00: 8B, 0x08: 4B, 0x0c: 4B, 0x10: 8B(重复 pos), 0x14: 4B, 0x18: 8B,
0x20: 4B, 0x24: 4B}——布局 **U**（含法线/uv 猜测不成立，typed gap）。三角形记录 0x20B（O：alloc `count<<5`）。

## 3. `displace_rocky_terrain` 消费（O，0x140a3ee80，行 14）

帧 pass（W4，frame 外层 @0x1404ec350 调用）：
- 取 `RockyTerrainMesh` 资源（O：资源查找 + `unwrap`）。
- `uVar2 / 3` = 三角形数 → 顶点组数（I：dispatch 规模）。
- `ComputePass::new` + `write_storage_buffer<RockyTerrainVertexSSBO>` +
  `write_storage_buffer<RockyTerrainTriangleSSBO>`（O）。
- `bind_texture_to_compute_pass<TexturePreset>` ×5（preset 9、2×、0xe、5、0x13——高度图/噪声纹理，I）。
- dispatch 规模 `uVar2/3`（写入 PushConstants {dispatch_x=tri, …}，O）。

`read_terrain_data` 另用 `DrawBuilder` + `bind_common` + `rocky_terrain_shader`（字符串 `"rocky_terrain"`，O）
绘制岩体到回读栅格，随后 `write_storage_buffer<TerrainRockMaskSsbo>` 等 compute 回拷。

## 4. 副作用 / 失败分支

- load：文件读失败 / JSON 解析失败 → `anyhow::Error`（O）；SSBO 创建失败 → alloc handle_error。
- displace：资源缺失 → `unwrap_failed`（O）。
- load 由资源加载路径触发（I：asset loader），一次性构建两个 SSBO，随后每帧 displace 在 GPU 上变形顶点。

## 5. O/I/U 摘要

- **O**：load 流程（读文件→JSON→两个 SSBO）、displace pass 的读写声明与 dispatch 规模、read_terrain_data 的岩体绘制。
- **I**：RockyTerrainMesh 的插入位置（资源注册）；顶点记录布局含义。
- **U**：`RockyTerrainMesh`/`LoadedRockyTerrainVertex`/`LoadedRockyTerrainTriangle` 字段布局；
  顶点/三角形 SSBO 精确记录格式；`TerrainEntities` 内部；JSON 文件 schema。→ typed gap。
