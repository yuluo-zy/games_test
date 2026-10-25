# WP8 契约 1/4 — `resources::terrain_height`

版本 **B**。数据源：skeleton bundle `resources/terrain_height.rs.{pdb,structures,enriched,dependencies}`、
反编译（本轮 Ghidra：`mouse_terrain_raycast`@0x1408ef320 调用 `intersect_ray<TerrainHeightSsbo>`；
`insert_terrain_resources`@0x1409457c0 插入两个 `TerrainHeightsData` 实例）、
prior 地形业务恢复 `reconstruction/terrain/crates/tg-terrain`（height readback 差分/schema）。

标注：**O**=PDB/反编译直接支持；**I**=调用链/骨架组合；**U**=未恢复（typed gap，不臆造）。

## 1. 命名实体与来源文件

`crates/country-core/src/resources/terrain_height.rs`（original_path `.../pouncelight/country-slice-private/...`），
pdb_sha256 `2d9686b8…`，source_sha256 `1227dfdd…`。PDB 命名实体（structures.json.gz）：

| 实体 | 证据 |
|---|---|
| `TerrainHeightsData<T>` | 泛型资源（O 名；两次实例化：`TerrainHeightSsbo` / `VisualTerrainHeightSsbo`） |
| `TerrainHeightSsbo` | SSBO 标记类型（O） |
| `VisualTerrainHeightSsbo` | SSBO 标记类型（O；行 107 `impl$5::default`） |
| `TerrainHeightFloatGrid` | `utils::float_grid` 网格类型（O；`bilinear_sample_at_world_xz<TerrainHeightFloatGrid>`） |

## 2. 方法清单（PDB inlinee + 行号）

| 方法 | 行 | 语义 / 证据 |
|---|---|---|
| `impl$4::default` | 96 | `TerrainHeightSsbo` 的 Default（O 名；insert_terrain_resources 内联行 96） |
| `impl$5::default` | 107 | `VisualTerrainHeightSsbo` 的 Default（O 名；同上内联行 107） |
| `TerrainHeightsData::sample_at_world_xz` | 118 | 世界 XZ→高度采样；实例 `VisualTerrainHeightSsbo` 为独立编译（O）；`stairs_assemble` 消费（I） |
| `TerrainHeightsData::intersect_ray` | 126 | 高度场射线求交（O 名；`mouse_terrain_raycast`@0x1408ef320 调用 `<TerrainHeightSsbo>`） |
| `impl$6::intersect_ray::closure$0` | 137 | intersect_ray 内联闭包 |
| `TerrainHeightsData::heightfield_intersect_ray` | 161 | 高度场求交核心（O 名） |
| `TerrainHeightsData::subscribe` | 243 | 订阅地形变化（O 名；`system_wall_constructor::construct_elevation_supports` 消费） |
| `TerrainHeightsData::copy_from_gpu` | 247 | GPU→CPU 高度回拷（O 名；`delayed_terrain_changed_msg`@0x140aabdd0 调用两次） |
| `TerrainHeightsData::as_raw` / `as_mut_raw` | 321/325 | 原始字节视图（save snapshot load 消费，I） |
| `TerrainHeightsData::as_bytes` / `height_range` | 330/334 | 字节+高度范围（save snapshot capture 消费，I） |
| `impl$7::load_index_unchecked` | 347 | 网格索引加载（`TerrainHeightFloatGrid` 索引访问，I） |

## 3. 布局与消费者

### 布局（O 部分 + typed gap）

`TerrainHeightsData<T>` 整体布局 **U**（PDB `name_and_relations_only`）。已观察字段/行为（I/O 混合）：
- 内含 `TerrainHeightFloatGrid`（O：`sample_at_world_xz<TerrainHeightFloatGrid>` 存在；通过 `utils::float_grid` 双线性采样）。
- `copy_from_gpu` 业务：prior 恢复 `tg-terrain::schema::HeightReadback`（512×512 u16 编码高度 +
  `{minimum_y=-2.5, maximum_y=24.0}`；8×8 脏块位图 64×u64；边界变化全量失效）。diff 语义 O（prior 原机器码案例）；
  字段名/装箱 **I**。

高度场网格：512×512（O：`read_terrain_data`/`rasterize_terrain_stroke` 的 dispatch 尺寸 0x200 与 rock mask 采样 x/130+0.5 一致，I；prior schema GRID_SIDE=512）。

### 资源插入（O，insert_terrain_resources@0x1409457c0）

`insert_resource<TerrainHeightsData<TerrainHeightSsbo>>` 与
`insert_resource<TerrainHeightsData<VisualTerrainHeightSsbo>>` **两个实例**（权威 + 视觉）。两个实例各含
`RtTriangleMetaSsbo` Vec 扩展（`alloc::vec::Vec::extend_with<RtTriangleMetaSsbo>`）——即插入了基于高度场生成的 RT 三角形元数据。

### 消费者（O 名，FunctionSystem 签名）

| 消费者 | 签名要点 |
|---|---|
| 墙高度系统（publish wall 域） | `Res<PublicWalls>, Res<AssetSsboLibrary>, Query<&Roof>, Res<TerrainHeightsData<TerrainHeightSsbo>>, ResMut<WallEntities>, ...`（`clamp_wall_height`/`PublicWallState::max_y` 引用，I） |
| 地形高度更新系统 | `ResMut<RaycastWorld>, ResMut<TerrainHeightsData<TerrainHeightSsbo>>, ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>, Commands`（重建碰撞 RaycastWorld） |
| `delayed_terrain_changed_msg` | `Res<AssetSsboLibrary>, ResMut<ScheduledTerrainReadback>, Local<DelayedUpdateWallsMsgBuffer>, EventWriter<OnTerrainChange>, ResMut<TerrainHeightsData<TerrainHeightSsbo>>, ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>`（O） |
| prefab 吸附 | `Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>, EventReader<OnTerrainChange>`（O） |
| `mouse_terrain_raycast` | 调用 `intersect_ray<TerrainHeightSsbo>`（O） |
| save snapshot | `as_bytes`/`height_range`（capture）、`as_raw`/`as_mut_raw`（load）（I） |
| 楼梯装配 | `sample_at_world_xz`（I：stairs_assemble 内联宿主） |

## 4. 副作用 / 失败分支

- `intersect_ray`：无命中时走 `CameraCursorRay` 的 xz 平面回退（O：mouse_terrain_raycast 内 `intersect_with_xz_plane` 分支）。
- `copy_from_gpu`：SSBO 缺失（FrozenMap get 失败 / 尺寸断言 0x3a）→ `unwrap_failed`/`assert_failed`（O）。
- 非 None 时 `unwrap_failed`：rasterize/displace 的资源查不到即 panic（O）。

## 5. 输入/输出/未恢复（O/I/U 摘要）

- **O**：资源存在且双实例化、方法清单+行号、`intersect_ray<TerrainHeightSsbo>` 调用、`copy_from_gpu` 两次调用（0x3a/0x40 SSBO）、
  高度边界 -2.5..24.0、512×512 网格。
- **I**：`subscribe` 的订阅对象（墙支持构造）；高度更新系统宿主模块（collision 域）。
- **U**：`TerrainHeightsData<T>` 字段布局、`TerrainHeightFloatGrid` 内部（数据/范围/格式）、`intersect_ray` 求交数学细节、
  `subscribe` 的取消/生命周期、保存 schema 精确字节序。→ typed gap。
