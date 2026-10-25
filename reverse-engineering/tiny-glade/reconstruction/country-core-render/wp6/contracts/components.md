# 组件族契约 — components/（WP6：drawable + 消费组件集）

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
消费者：`render/nani/systems.rs`；产者：游戏各 CPU 系统（建筑/装饰/门窗等，属其他 WP）。
本族只恢复 nani systems 消费的组件；其余为 typed gap。

## 1. `components/drawable.rs`（源 SHA256 `drawable.rs` B skeleton；PDB 命名实体 15 个）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `NaniInstanceDesc` | 实例描述（maintain 建实例读） | O：ECS `&NaniInstanceDesc`；maintain 读 mesh selector + subset_kind | I：`{ mesh: NaniInstanceMeshDesc, subset_kind: u8 }`；行 stride 0x18 证据对 24B 候选 vs 16B 紧凑布局有歧义（见 §1.1） |
| `NaniInstanceMeshDesc` | 网格选择器 | O：B public-symbols `From<MeshAtlasPart>`；`Named(u64)`/`AtlasPart(u32)` 与 `tg-building-runtime` 候选同形 | I（16B enum） |
| `NaniActiveInstance` | 激活实例（maintain 插入；update 读 key） | O：`Insert<NaniActiveInstance>` 28B command 负载 16B（写入 +0x18..+0x24） | O 命令写入：`{chunk_id u32, instance_index u32, kind u8, sort_key_f16 u16, flags u8, reserved u8}`（13B+3B 填充）；chunk_id u64? I |
| `NaniSolidTexture` | albedo 纯色贴图选择 | O：ECS `&NaniSolidTexture` | U |
| `PrevTransform` | 上一帧变换（velocity/wren） | O：ECS `&PrevTransform` | U |
| `AlbedoMap` | albedo 贴图 | O：ECS `Option<&AlbedoMap>` | U |
| `AlphaMap` | alpha 贴图 | O：命名实体 | U |
| `NaniBundle<T>` | 生成 bundle | O：B `NaniBundle<SolidVertexColorInstanceData>` insert 命令 | I：`{descriptor, transform, marker}` |

### 1.1 `NaniInstanceDesc` 尺寸歧义（I/U）
- `tg-building-runtime` 候选 `{mesh: enum(16), subset_kind: u8}` = 24B。
- maintain 反编译 row 读取 stride 0x18（`Entity(8) + desc(16)` 时成立），指向 16B 紧凑
  `{mesh: u64, subset_kind: u8}`。
- 裁决：**候选未复用**（尺寸矛盾）；装配按候选枚举形状实现但标注布局 I；B 的精确
  打包 U。`NaniInstanceMeshDesc::from<MeshAtlasPart>`（B O）保留。

## 2. `components/mod.rs`（源 SHA256 `components/mod.rs`；PDB 命名实体 45 个）

| 类型 | 角色 | 证据 | 布局 |
|---|---|---|---|
| `SemiTransparent` | 半透明标记 | O：ECS `Option<&SemiTransparent>` | I：ZST marker |
| `FlatLighting` | 平直光照标记 | O：ECS `Option<&FlatLighting>` | I：ZST marker |
| `BrickColor` | 砖色 | O：ECS `&BrickColor`（bricks/gothic 读） | U |
| `GothicWindowBrickDeformation` | 哥特窗砖变形 | O：ECS `&GothicWindowBrickDeformation` | U |
| `ClothlineCloth` | 晾衣绳布料 | O：ECS `&ClothlineCloth`（cloth 读） | U |
| `NormalMap` | 法线贴图 | O：ECS `&NormalMap` | U |
| `LanternLightParams` | 灯笼光照参数 | O：ECS `&LanternLightParams` | U |
| `Smoke` | 烟 | O：ECS `&Smoke`（smoke 读） | U |

## 3. Unknown / typed gaps
- 除 `NaniActiveInstance`/`NaniInstanceDesc` 外，各组件字段布局 U（着色/动画参数语义属
  对应 CPU 系统的 WP）。
- `NaniBundle<T>` 的 T 与 InstanceDataMarker 关系 I。
