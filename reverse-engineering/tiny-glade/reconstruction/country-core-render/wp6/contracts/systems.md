# nani ECS 系统契约 — systems.rs（WP6：22 系统，含新增 `nani_flush_instance_updates`）

标注：**O**=反编译/PDB 直接支持；**I**=调用链/行号/装配组合；**U**=未恢复（typed gap）。
系统签名来自 PDB `FunctionSystem<void(*)(...)>` 单态化（wp3 `evidence/ecs-system-signatures.txt`，
24 条唯一签名）+ WP6 新恢复的 `nani_flush_instance_updates`（反编译参数推断）。
源 SHA256 `aa300bc4204c722243361b6615fb01789de07a3d0d1a0cffd6aa9c5e8b17bc51`；PDB `2d9686b8...`。
所有地址为 B patched EXE（cf048e27...）。

## 0. 系统清单（22）

| 系统 | 地址 | 源行 | 说明 |
|---|---|---|---|
| `nani_maintain_instances` | 0x14206f300 | 36 | 建/删/改实例（O；closure$1 0x141565950 行 84） |
| `nani_update_solid_vertex_color` | 0x142070ac0 | 119 | 纯色 → 0x40 |
| `nani_update_solid_vertex_color_window` | 0x142071590 | 152 | 窗纯色 → 0x40 |
| `nani_update_solid_albedo_map` | 0x142072010 | 185 | albedo → 0x40 |
| `nani_update_solid_albedo_map_emissive` | 0x142072b50 | 223 | albedo+自发光 → 0x40 |
| `nani_update_preplaced_flower` | 0x142073690 | 261 | 花/水判定（O 详细） → 0x40 |
| `nani_update_plant` | 0x142074740 | 313 | 植物 → 0x30（O 代表） |
| `nani_update_solid_vertex_color_window_glass` | 0x142075070 | 337 | 窗玻璃 → 0x40 |
| `nani_update_solid_vertex_color_bricks` | 0x142076230 | 393 | 砖 → 0x40 |
| `nani_update_gothic_window_bricks` | 0x142076e40 | 425 | 哥特窗砖 → 0x70 |
| `nani_update_curved_halftimber` | 0x142077d20 | 0* | 半木 &mut 直写（唯一非复制式） → 0x40 |
| `nani_update_stone_floor` | 0x142078780 | 0* | 石地板（屋顶切线，O 详细） → 0x40 |
| `nani_update_wren` | 0x1420797b0 | 671 | 鹪鹩 → 0x70 |
| `nani_update_lantern_emissive` | 0x14207a4f0 | 700 | 灯笼 → 0x40 |
| `nani_update_plaster` | 0x14207b290 | 750 | 石膏剥离 → 0x30 |
| `nani_update_cloth` | 0x14207c0a0 | 800 | 布料 → 0x40 |
| `nani_update_smoke` | 0x14207cd60 | 835 | 烟 → 0x40 |
| `nani_update_balcony_door` | 0x14207d770 | 868 | 阳台门 → 0x40 |
| `nani_update_uber` | 0x14207e1f0 | 903 | uber → 0x40 |
| `nani_update_velocity_only` | 0x14207f060 | 954 | 速度-only → 0x60 |
| `nani_update_stairs_floor` | 0x14207fcc0 | 979 | 楼梯地板 → 0x30 |
| `nani_flush_instance_updates` | 0x142080640 | 1011 | **WP6 新恢复**；GPU 冲刷 |

\* `nani_update_curved_halftimber` / `nani_update_stone_floor` 的 PDB source_line = 0（行号缺失），
仅以签名+地址为准（wp3 已标注）。

## 1. `NaniActiveInstanceCollection`（Local，O）

`nani_maintain_instances` 的 `Local`：`HashMap<Entity, InstanceHandle>`（Entity→实例句柄，
O：`HashMap<Entity, InstanceHandle>::insert` / `remove_entry`）+ 每 kind 的变更 tick
切片/缓存缓冲（U）。

## 2. `nani_maintain_instances`（0x14206f300，行 36）

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

**业务逻辑**（O，反编译 773 行）：
1. puffin 作用域（系统名经 rfind/UTF-8 解码，同 `nani_update_*`）。
2. **新实例**（`Without<NaniActiveInstance>`）：每实体经 `MeshAtlasLibrary::get_by_name<NameHash>`
   取 atlas part（缺失 → `panic_fmt`，路径含 "Instance limit reached while trying ..."）；
   计算 `InstanceKey`（`half::f16::from_f32` sort_key，chunk_id 来源 I）；
   `GpuWorld::owned_subset_mut(kind)` + `OwnedGpuWorldSubset::extend_no_data(keys, callback,
   kind)` 建实例；`local.entity_to_handle.insert(entity, handle)`；`Commands` push
   `Insert<NaniActiveInstance>`（0x28B command：fn ptr + panic handler + 16B 负载）。
3. **删除**（`RemovedComponents<NaniActiveInstance>`）：`local.remove_entry` →
   `GpuWorld::remove_instance(key)`（key 由 handle 构造，chunk_id U）。
4. **变更**（`Changed<NaniInstanceDesc>`）：经 `Table::get_changed_ticks_slice_for` 遍历 →
   `GpuWorld::update_instance_mesh(entity)`（0x140991d60；字段流 U）。
   **装配注（I）**：PDB changed 查询 tuple 为 `(desc, active)`（无 Entity）；装配版补 Entity
   以对接 `update_instance_mesh(entity)`——PDB 记录的是非 Entity 访问器 tuple，加 Entity 是
   忠实对接选择。
5. 上限（O）：`extend_no_data` 内部 `instance_count >= max_instances[kind]` → panic
   "Max instance count reached for {kind}"。

**InstanceKey 计算**（O 结构 / I 字段）：
- row 读取：`*(row+0x10)` u64（chunk/高层键）、`*(row+0x18)` u32（实例索引）、
  `half::f16::from_f32` 的 sort_key（→ `NaniActiveInstance` 16B 负载：`{chunk_id u32,
  instance_index u32, kind u8, sort_key f16, flags u8, reserved u8}`，13B 有效 + 3B 填充，O）。
- chunk_id 是否 u64（对应 `InstanceKey.chunk_id`）I；`from_key`/`into_key` 打包 U。

## 3. `nani_update_<T>` 系统族（×20，O，统一模式）

**统一模式**（`nani_update_plant` 0x142074740 为 O 代表，189 行）：
1. puffin 作用域。
2. `GpuWorld::owned_subset_mut(kind)` 拿写锁（每系统一个 kind；kind 常量 U，除
   `CurvedHalftimber`/`Plaster`/`Smoke`/`StairsFloor` 由 shader 家族 O 确定）。
3. 经 `Table::get_changed_ticks_slice_for`（world +0x110 tick / +0x118 last_run 阈值）迭代
   已变更实例；每实例从 ECS 组件计算 `T` → `OwnedGpuWorldSubset::update_instance_data<T>(key,
   data)`（0x14156db40 系，T 尺寸对应元素尺寸）。
4. 释放锁（parking_lot deadlock_impl release + RawRwLock::unlock_exclusive_slow）；
   结束作用域。

**各系统 ECS 查询签名**（O，PDB；完整见 wp3 evidence）——wp3 §3 的表逐条有效，本表补
每系统组件→T 映射状态：

| 系统 | 查询组件（除 NaniActiveInstance/Transform） | Res/额外 Query | 组件→T 映射 |
|---|---|---|---|
| solid_vertex_color | Option<&SemiTransparent>, Option<&FlatLighting> | — | U |
| solid_vertex_color_window | &ColorId, Option<&SemiTransparent> | — | U |
| solid_albedo_map | &NaniSolidTexture | — | U |
| solid_albedo_map_emissive | &NaniSolidTexture | — | U |
| preplaced_flower | — | AssetSsboLibrary, **PathRastersResources**（裸 SystemParam）, Res<WaterRaster> | **O（水/花位图判定，见 §3.1）** |
| plant | — | — | U（transform → 0x30） |
| window_glass | &WindowLight, &ColorId, Option<&SemiTransparent>, Option<&AlbedoMap> | GlobalWindowLightsControl, AssetTextureLibrary | U |
| bricks | &BrickColor | — | U |
| gothic_window_bricks | &BrickColor, &GothicWindowBrickDeformation | — | U |
| curved_halftimber | **&mut CurvedHalfTimberInstanceData**, &CurvedHalfTimber | PublicWalls, HalftimberColorIds, Query<&Roof> | **O（cap 掩码，见 §3.2）** |
| stone_floor | Option<&Roof>, Option<&RoofAnimation>, Option<&Floor> | — | **O（动态组件位 + 切线，见 §3.3）** |
| wren | &PrevTransform, &Wren | — | U |
| lantern_emissive | &LanternLightParams, Option<&SemiTransparent>, Option<&ColorId> | — | U |
| plaster | &Plaster（Or<(Changed<Plaster>, Changed<NaniActiveInstance>)>） | PublicWalls, AnimatedPlasterPeel | U（剥离动画 → 0x30） |
| cloth | &ClothlineCloth, &NormalMap | AssetTextureLibrary | U |
| smoke | &Smoke | — | U |
| balcony_door | &ColorId, Option<&SemiTransparent> | — | U |
| uber | Option<&SemiTransparent>, Option<&AlbedoMap> | AssetTextureLibrary | U |
| velocity_only | &PrevTransform | — | U（prev/current → 0x60） |
| stairs_floor | &StairsFloorMeshChunk（Or<(Changed<Chunk>, Changed<NaniActiveInstance>)>） | — | U（mesh chunk → 0x30） |

**§3.1 `nani_update_preplaced_flower`（O，345 行）——花/水判定**：
1. `AssetSsboLibrary` 经 `SsboId` 取花位图 `BufferState`（FrozenMap get；`assert_failed`
   校验 name 常量，`memcmp` 0x3f 字节）。
2. 每变更实例：Transform 位置（row +0x10/+0x18 双分量，O；精确分量映射 I）→
   `x/130.0 + 0.5` 双分量 → clamp[0,1] → `*512.0` → 512×512 格点
   （`uVar11`/`uVar16`，`uVar15 = (y<<9|x)>>5`，bit = `(word >> (x&31)) & 1`）。
3. 若位图 bit 为 0 **且** `bilinear_sample_at_world_xz<RasterBufferRef<512>>` 4 邻居高度和
   `<= 0.01`（水阈值）→ 写真实 transform 的 `PreplacedFlowerInstanceData`；否则写
   `Transform::default()`（隐藏）。**这正是"花避开水面/路径"的 GPU 世界效果。**

**§3.2 `nani_update_curved_halftimber`（O，465 行）——半木端帽掩码**：
- 读 `CurvedHalfTimber`（几何）、`PublicWalls`、`HalftimberColorIds`、`Query<&Roof>`；
  反编译含 16 段 cap 掩码位提取：`auVar45[i] = -(byte == (u32>>(8*i)))`（u128 内逐字节
  比较），及 per-segment 越界/存在性检查（`panic_bounds_check`）。
- 效果：把每段端帽标志写入 `&mut CurvedHalfTimberInstanceData`（packed 字段）——唯一
  非复制式 update（`&mut` 直写 + 随后 `update_instance_data` 复制回 GPU 缓冲）。

**§3.3 `nani_update_stone_floor`（O，339 行）——屋顶切线**：
- 动态组件位（world +0x120/+0x128/+0x130）检测 `Roof`/`RoofAnimation`/`Floor` 存在性。
- 读屋顶数据（行 stride 0x58）、floor（stride 0x28）、roof_animation（stride 0x8）；
  `+8` 字节 bit0 判定是否调 `utils::geometry::circle::Circle2d::get_tangent_at_u()`
  （弯曲屋檐切线），之后 `update_instance_data<StoneFloorInstanceData>`。

## 4. `nani_flush_instance_updates`（0x142080640，行 1011）— WP6 新恢复

**签名**（I，反编译 849 行参数推断）：`fn nani_flush_instance_updates(mut gpu_world:
ResMut<GpuWorld>)`——无 Query；因此不在 wp3 的 24 条 `FunctionSystem` 签名中
（那些均为带 Query 的系统）。

**业务逻辑**（O）：
1. puffin 作用域。
2. 遍历 GpuWorld 脏槽（30 槽，stride 0xc0，从 +0x8 起；`for (lVar12=0x1440; lVar12!=0;
   lVar12-=0xc0)`）：若 `*(槽-0x48)` 脏标志通过哨兵判定（`-0x7fffffffffffffff < v`）→
   `ShadowedBuffer<GpuInstance>::gpu_flush()` + vtable+0x30 的"冲刷脏区间"回调
   `(handle, base+0x10)`。
3. `*(+0x1440) = *( +0x1448)`；`*(+0x1440) = 0`——把本帧实例变更计数（对应 maintain
   每建实例 `*(+0x1440)+=1`）搬到"已冲刷"计数并清零。
**注（I/U）**：30 槽 vs `GpuWorldSubsetKind` 27 变体——槽数组可能与 27 subset 不对应
（可能含 30 个实例数据缓冲脏区间；wp3 gpu_world.md 的 27×0xc0 布局与 30 槽有歧义，U）。

## 5. 接线（O/I）

这些系统在 bevy schedule 中运行于渲染前；产物为 `GpuWorld` subset 数据 +
`HashMap<Entity, InstanceHandle>` + 每帧冲刷的 GPU 缓冲。GPU 消费见 wp3 gpu_world.md §8。
`nani_flush_instance_updates` 是更新半周期（maintain/update 写 CPU 侧，flush 传 GPU）。

## 6. Unknown / typed gaps

- 各 `nani_update_*` 的组件→`T` 字段映射算法（transform 分解、色值打包、动画参数）U——
  除 preplaced_flower / curved_halftimber / stone_floor 的 O 级路径。
- 每系统 `kind` 常量（`GpuWorldSubsetKind`）U（除 shader 家族确定的 4 个）。
- `NaniActiveInstanceCollection` 内部缓冲布局 U（除 Entity→InstanceHandle map）。
- `nani_flush_instance_updates` 的 30 槽精确对应与 `PathRastersResources` SystemParam
  内部字段 U。
- 每系统`update_instance_data` 的 sort_key 语义与 chunk_id 来源 U。
