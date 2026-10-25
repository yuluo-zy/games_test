# 资源族契约 — resources/（WP6：nani systems 消费的集合）

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
只恢复 nani systems 消费的类型；其余为 typed gap（不臆造）。

## 1. `color_id.rs`（源 SHA256 B skeleton；PDB 命名实体 10 个）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `ColorId` | 颜色索引（组件） | O：ECS `&ColorId`；serde newtype（`visit_newtype_struct`） | I：`u32` 颜色/材质索引 |
| `HalftimberColorIds` | 半木色索引集合（Res） | O：`Res<HalftimberColorIds>`（curved_halftimber 读） | U |
| `WallColorIds` / `GateColorIds` | 墙/门色集合 | O：命名实体 | U（未消费） |

## 2. `window_lights.rs`（源 SHA256 B skeleton；PDB 命名实体 4 个）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `WindowLight` | 窗内灯光（组件） | O：ECS `&WindowLight`（glass 读） | U |
| `GlobalWindowLightsControl` | 全局窗光开关（Res） | O：`Res<GlobalWindowLightsControl>` | U |
| `LightInfo` / `GlobalSmokeControl` | 命名实体 | O | U（未消费） |

## 3. `walls/`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `Floor`（组件） | 楼板 | O：ECS `Option<&Floor>`；stone_floor 读 +8 字节 bit0 | I：≥0x28B；+0x8 bit0 控制切线路径 |
| `public_walls::PublicWalls`（Res） | 公共墙集合 | O：`Res<PublicWalls>` | U |
| `plaster::Plaster`（组件） | 石膏墙 | O：ECS `&Plaster`；0x14207b290 读多字节段 | U |
| `plaster::AnimatedPlasterPeel`（Res） | 剥离动画 | O：`Res<AnimatedPlasterPeel>` | U |
| `curved_halftimber::CurvedHalfTimber`（组件） | 弯曲半木墙 | O：ECS `&CurvedHalfTimber`；0x142077d20 u128 位提取 | I：含 16 段端帽掩码 |

## 4. `roofs/`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `roof_shape::Roof`（组件） | 屋顶形状 | O：ECS `Query<&Roof>`/`Option<&Roof>`；stone_floor 行 stride 0x58、+8 bit0 | I：≥0x58B |
| `roof_animation::RoofAnimation`（组件） | 屋顶动画 | O：ECS `Option<&RoofAnimation>`；stride 0x8 | I：~8B（动画高度） |

## 5. `critters.rs`（源 SHA256 B skeleton；PDB 命名实体 19 个）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `Wren`（组件） | 鹪鹩状态 | O：ECS `&Wren`（wren 读 prev/current + 参数） | U |

## 6. `stairs/visual.rs`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `StairsFloorMeshChunk`（组件） | 楼梯地板 mesh 块 | O：ECS `&StairsFloorMeshChunk`（`Or<(Changed<Chunk>, Changed<Active>)>`） | U |

## 7. `world_raster.rs` / `world_raster_defs.rs`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `RasterBufferRef<512>` | 512×512 光栅引用 | O：`bilinear_sample_at_world_xz<resources::world_raster::RasterBufferRef<512>>` | U |
| `WaterRaster`（Res，`world_raster_defs`） | 水光栅 | O：`Res<WaterRaster>`（preplaced_flower 水位 ≤0.01 判定） | I：newtype 到 `RasterBufferRef<512>`（Deref 证据） |
| `PathRaster`/`GardenRaster`/`Cobblestone1/2/3Raster` | 命名光栅 | O：game_profile 依赖 Deref/DerefMut | U（未消费） |

## 8. `render/mesh_atlas_library.rs`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `MeshAtlasLibrary`（Res） | mesh 图集库 | O：`Res<MeshAtlasLibrary>`；`get_by_name<NameHash>`（maintain 查 part） | U |
| `AtlassedMeshName` | 图集化名 | O：命名实体（FrozenMap 键） | U |

## 9. `systems/paths/draw_path.rs`（源 SHA256 B skeleton）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `PathRastersResources`（**裸 SystemParam**） | 路径光栅 | O：preplaced_flower 签名中裸参数（非 Res）；`bilinear_sample_at_world_xz` 宿主 | U；装配以 `Res<'w, PathRasterData>` 内部载体实现（I） |
| `DrawState`（Res） | 绘制状态 | O：命名实体 | U |

## 10. `utils/components/transform.rs`（源 SHA256 `7959b9d1...`；utils crate）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `Transform` | 游戏自有变换 | O：`compute_affine` 0x1407a45b0 读 self 10×f32；方法 PDB LF_FUNC_ID（from_translation/from_scale/from_translation_scale/identity/compute_matrix/compute_affine/forward/look_at/abs_diff_eq/default） | **O**：`{translation Vec3 @0x0, rotation Quat @0xC, scale Vec3 @0x1C}` = 40B（bevy 兼容） |

**归属裁决**：`utils::components::transform::Transform` 属 **utils crate 游戏自有 util**
（非 bevy/glam 依赖），按项目边界恢复外壳（方法签名 + 布局），数学复用 glam 0.29.3
（`Mat4::from_scale_rotation_translation` = compute_affine，O 结构一致）。

## 11. Unknown / typed gaps
- 除 `Transform` 布局 O 外，各资源内部字段 U；色值/动画/几何语义属对应 CPU 系统的 WP。
- `PathRastersResources` SystemParam 内部形状 U（装配用推断载体）。
