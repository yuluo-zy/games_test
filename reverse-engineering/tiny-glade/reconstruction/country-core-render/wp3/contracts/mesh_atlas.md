# MeshAtlas 契约 — mesh_atlas.rs（3 procs）

标注：**O**=反编译直接支持；**I**=推断；**U**=未恢复。地址 B patched EXE。

## 1. 类型

- `MeshAtlas`（I）：网格图集，持有 `HashMap<AtlassedMeshName, MeshAtlasPart>`（`BuildHasherDefault<NoHashHasher<AtlassedMeshName>>`，O）与属性布局（sorted attribute name 列表）。`MeshAtlas::add` 0x140a36200 的 `in_RDX` 为 atlas。
- `MeshAtlasPart`（16B，O）：`{ u32 part/base_index; u32 count; f32 radius; u32 pad }`。`MeshAtlasPart::new` 0x140833250（3B 薄构造）。
- `NaniMeshAtlasPartInfo`（gpu_world.rs，`TypedStorageBuffer` 元素类型；字段 U）。

## 2. `MeshAtlas::add`（0x140a36200，行 75；3289B）

```
fn add(&mut self, name: AtlassedMeshName, mesh: Mesh) -> Result<(), anyhow::Error>（anyhow 证据：Error::msg "Mesh attribute mismatch ..."）
```

**业务逻辑**（O）：
1. `mesh.positions` 空 → `panic_fmt("invalid mesh ...")`。
2. 从 mesh 的属性表（`HashMap<String, VertexAttributeValues>`）计算**包围半径**：遍历 position 三元组（u64 对 {2×f32}，步长 0xc），`radius = sqrt(max(x²+y²+z²))`；空则 0.0。寻找 mesh 名匹配：hash 查找 `mesh` 关键字记录（字符串 `0x505f786574726556`=“Vertex_P” 等），`VertexAttributeValues::append` 合并。
3. 若 atlas 已初始化：比较**新增 mesh 的属性名集**与已有布局（两排序属性列表按长度+`memcmp` 一致性）；不一致 → 返回 `Err(anyhow!("Mesh attribute mismatch: Existing ..."))`。
4. 索引偏移：`base_vertex = atlas.vertex_count`；把 mesh 的索引全部 `+ base_vertex`（4B×count 偏移循环）。
5. `MeshAtlasPart` 记录写 `{part_index, count, radius, 0}` push 到 `ComputedLayout` Vec（`grow_one<ComputedLayout>`）。
6. `HashMap::insert<AtlassedMeshName, MeshAtlasPart>`：重复名 → `panic_fmt("Duplicate atlas mesh name ...")`；成功返回 `Ok`。

## 3. 其他

- `impl$10::clone_behavior`（0x140002370，3B）：Component 派生薄封装（Ghidra 解析为 WireframePass 实例，同址 clone_behavior 族）。
- `MeshAtlasPart::new`（0x140833250，3B）：`MeshAtlasPart::new()` 空构造（I）。

## 4. Unknown

- `ComputedLayout` 记录语义与消费方（gpu_generate_draw_lists 的 atlas part 分桶）U。
- `NaniMeshAtlasPartInfo` 布局与 `MeshAtlasLibrary`（另一文件）对接字段 U。
- mesh 属性名排序/比较的确切 trait 边界（`Keys<String, VertexAttributeValues>` 排序）O 但属性名集合的确切组合 U。
