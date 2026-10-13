# nani ECS 系统契约 — systems.rs（24 procs + 1 Local）

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复。系统签名来自 PDB `FunctionSystem<void(*)(...)>` 单态化（见 evidence/ecs-system-signatures.txt，24 条唯一签名）。

## 1. `NaniActiveInstanceCollection`（Local，O）

`nani_maintain_instances` 的 `Local` 状态：`HashMap<Entity, InstanceHandle>`（Entity→实例句柄）+ 每 kind 的变更 tick 切片/缓存缓冲。PDB 唯一实体（systems.rs 结构文件 1 个）。

## 2. `nani_maintain_instances`（0x14206f300，行 36；closure$1 0x141565950）

**PDB 签名**（O）：
```
fn nani_maintain_instances(
    new: Query<(Entity, &NaniInstanceDesc, &Transform), Without<NaniActiveInstance>>,
    changed: Query<(&NaniInstanceDesc, &NaniActiveInstance), Changed<NaniInstanceDesc>>,
    removed: RemovedComponents<NaniActiveInstance>,
    mesh_atlas: Res<MeshAtlasLibrary>,
    gpu_world: ResMut<GpuWorld>,
    local: Local<NaniActiveInstanceCollection>,
    commands: Commands,
)
```
**业务逻辑**（O）：
1. puffin 作用域（`begin_scope`，系统名经 rfind/UTF-8 解码，同 `nani_update_*` 模式）。
2. **新实例**（无 `NaniActiveInstance`）：遍历 `NaniInstanceDesc`（mesh 名 + subset kind），经 `MeshAtlasLibrary::get_by_name<NameHash>` 取 atlas part（缺失 → `panic_fmt`）；计算 InstanceKey（含 f16 sort_key，`half::from_f32`）；`GpuWorld::owned_subset_mut(kind)` + `OwnedGpuWorldSubset::extend_no_data(keys, callback, kind)` 建实例；`Local` 记录 `HashMap<Entity, InstanceHandle>::insert`；向 `Commands` push `Insert<NaniActiveInstance>`（28B command：`insert::closure_env` + `panic` handler + 句柄）。
3. **删除**（`RemovedComponents<NaniActiveInstance>`）：从 `Local` HashMap `remove_entry<Entity, InstanceHandle>` → `GpuWorld::remove_instance(key)`。
4. **变更**（`Changed<NaniInstanceDesc>`）：经 `Table::get_changed_ticks_slice_for` 遍历变更实体 → `GpuWorld::update_instance_mesh(entity, desc, transform)`。
5. 上限：`extend_no_data` 内部 `instance_count >= max_instances[kind]` → panic `"Max instance count reached for {kind}"`（格式串）；本系统容量用尽 → panic `"Instance limit reached while trying ..."`。
6. 遍历使用 `Local` 的逐 kind 索引迭代器（tick 阈值 `iVar25 - tick < uVar13` 过滤），closure$1 为 FnOnce 调用薄封装。

## 3. `nani_update_<T>` 系统族（×20，O，统一模式）

| 系统 | 地址 | 行 | 实例类型 | 尺寸 |
|---|---|---|---|---|
| `nani_update_solid_vertex_color` | 0x142070ac0 | 119 | SolidVertexColorInstanceData | 0x40 |
| `nani_update_solid_vertex_color_window` | 0x142071590 | 152 | SolidVertexColorWindowInstanceData | 0x40 |
| `nani_update_solid_albedo_map` | 0x142072010 | 185 | SolidAlbedoMapInstanceData | 0x40 |
| `nani_update_solid_albedo_map_emissive` | 0x142072b50 | 223 | SolidAlbedoMapEmissiveInstanceData | 0x40 |
| `nani_update_preplaced_flower` | 0x142073690 | 261 | PreplacedFlowerInstanceData | 0x40 |
| `nani_update_plant` | 0x142074740 | 313 | PlantInstanceData | 0x30 |
| `nani_update_solid_vertex_color_window_glass` | 0x142075070 | 337 | SolidVertexColorWindowGlassInstanceData | 0x40 |
| `nani_update_solid_vertex_color_bricks` | 0x142076230 | 393 | SolidVertexColorBricksInstanceData | 0x40 |
| `nani_update_gothic_window_bricks` | 0x142076e40 | 425 | GothicWindowBricksInstanceData | 0x70 |
| `nani_update_curved_halftimber` | 0x142077d20 | — | CurvedHalfTimberInstanceData（&mut） | 0x40 |
| `nani_update_stone_floor` | 0x142078780 | — | StoneFloorInstanceData | 0x40 |
| `nani_update_wren` | 0x1420797b0 | 671 | WrenInstanceData | 0x70 |
| `nani_update_lantern_emissive` | 0x14207a4f0 | 700 | LanternEmissiveInstanceData | 0x40 |
| `nani_update_plaster` | 0x14207b290 | 750 | PlasterInstanceData | 0x30 |
| `nani_update_cloth` | 0x14207c0a0 | 800 | ClothInstanceData | 0x40 |
| `nani_update_smoke` | 0x14207cd60 | 835 | SmokeInstanceData | 0x40 |
| `nani_update_balcony_door` | 0x14207d770 | 868 | BalconyDoorInstanceData | 0x40 |
| `nani_update_uber` | 0x14207e1f0 | 903 | UberInstanceData | 0x40 |
| `nani_update_velocity_only` | 0x14207f060 | 954 | VelocityOnlyInstanceData | 0x60 |
| `nani_update_stairs_floor` | 0x14207fcc0 | 979 | StairsFloorInstanceData | 0x30 |

**统一模式**（以 `nani_update_plant` 0x142074740 为 O 代表）：
1. puffin 作用域（系统名解析，同 maintain）。
2. `GpuWorld::owned_subset_mut(kind)` 拿写锁。
3. 经 `Table::get_changed_ticks_slice_for`（`(world +0x100)[entity_index]`、`+0x1a8` 表）迭代已变更实例；每实例从 ECS 组件计算 `T` → `OwnedGpuWorldSubset::update_instance_data<T>(key, data)`。
4. 释放锁（`parking_lot` deadlock_impl release + RawRwLock::unlock_exclusive_slow）；结束 puffin 作用域。

**各系统 ECS 查询签名**（O，PDB；完整见 evidence/ecs-system-signatures.txt）：
- `nani_update_curved_halftimber`：`Query<(&NaniActiveInstance, &mut CurvedHalfTimberInstanceData, &CurvedHalfTimber), With<InstanceDataMarker<CurvedHalfTimberInstanceData>>>` + `Res<GpuWorld>` + `Res<PublicWalls>` + `Res<HalftimberColorIds>` + `Query<&Roof>` —— 唯一一个 `&mut InstanceData` 直写（非复制式 update）。
- `nani_update_plant`/`preplaced_flower`/`smoke`/`bricks`/`gothic_bricks`/`velocity_only`/`wren`/`albedo_map(_emissive)`：`Query<(&NaniActiveInstance, &Transform [, 额外组件]), With<InstanceDataMarker<T>>>` + `Res<GpuWorld>`。
- `nani_update_solid_vertex_color`：`Query<(&NaniActiveInstance, &Transform, Option<&SemiTransparent>, Option<&FlatLighting>), With<...SolidVertexColorInstanceData>>`。
- `nani_update_uber`：`Query<(&NaniActiveInstance, &Transform, Option<&SemiTransparent>, Option<&AlbedoMap>), With<...UberInstanceData>>` + `Res<AssetTextureLibrary>`。
- `nani_update_cloth`：`+ &ClothlineCloth + &NormalMap` + `Res<AssetTextureLibrary>`。
- `nani_update_lantern_emissive`：`+ &LanternLightParams + Option<&SemiTransparent> + Option<&ColorId>`。
- `nani_update_window_glass`：`+ &WindowLight + &ColorId + Option<&SemiTransparent> + Option<&AlbedoMap>` + `Res<GlobalWindowLightsControl>` + `Res<AssetTextureLibrary>`。
- `nani_update_balcony_door`/`solid_vertex_color_window`：`+ &ColorId + Option<&SemiTransparent>`。
- `nani_update_plaster`：`Query<(&NaniActiveInstance, &Plaster), Or<(Changed<Plaster>, Changed<NaniActiveInstance>)>>` + `Res<PublicWalls>` + `Res<AnimatedPlasterPeel>`。
- `nani_update_stairs_floor`：`Query<(&NaniActiveInstance, &StairsFloorMeshChunk), Or<(Changed<StairsFloorMeshChunk>, Changed<NaniActiveInstance>)>>`。
- `nani_update_stone_floor`：`Query<(&NaniActiveInstance, &Transform, Option<&Roof>, Option<&RoofAnimation>, Option<&Floor>), With<...StoneFloorInstanceData>>`。

## 4. 接线（O/I）

这些系统在 bevy schedule 中运行于渲染前；产物为 `GpuWorld` 的 subset 数据 + `HashMap<Entity, InstanceHandle>`。GPU 消费见 gpu_world.md §8。`nani_update_stairs_floor` 与 `nani_update_curved_halftimber` 接受非 Transform 直接组件，说明二者对几何/mesh 数据变化敏感（Or<(Changed<source>, Changed<NaniActiveInstance>)> 过滤）。

## 5. Unknown

- 各 `nani_update_*` 的组件→`T` 字段映射算法（transform 分解、色值打包、动画参数）U——需结合对应 source 组件结构（RoofAnimation/ClothlineCloth/WindowLight 等，属其他 WP）与 shader 反射恢复。
- `NaniActiveInstanceCollection` 内部缓冲布局 U（除 Entity→InstanceHandle map 外）。
- `nani_update_curved_halftimber`/`nani_update_stone_floor` 的 source_line 为 0（PDB 行号缺失），仅以签名+地址为准。
