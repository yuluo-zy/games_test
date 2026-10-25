# WP3 交付总结 — country-core nani GPU-driven 渲染器

日期：2026-10-13。目标版本 **B**（`tiny-glade.exe` cf048e27…，PDB `tiny_glade.pdb` 2d9686b8…）。

## 1. 交付物

```
wp3/
├── contracts/            # 8 份模块级业务契约（O/I/U 标注）
│   ├── gpu_world.md      #   7 个子功能（A 构造布局 / B 实例创建 / C 实例更新 / D 删除 /
│   │                     #   E RG 生命周期 / F 枚举序列化 / G 组件注册）
│   ├── instance_data.md  #   23 类型 + 元素尺寸 + CurvedHalfTimber 构造器 + 位域
│   ├── systems.md        #   nani_maintain_instances + 20 个 nani_update_*（完整 ECS 签名）
│   ├── mesh_atlas.md  trimesh.md  buffers.md  draw_ext.md  cull_shaders.md
├── src/nani/             # Rust 骨架，镜像 skeleton 模块树（gpu_world/instance_data/systems/
│                         #   mesh_atlas/trimesh/buffers/draw_ext/cull_shaders + mod.rs）
├── evidence/
│   ├── b{1..4}-result.json     # 61 个函数反编译结果（shared worker，只读）
│   ├── decompiled/             # 61 个 .c（地址命名）
│   └── ecs-system-signatures.txt  # 24 条 PDB FunctionSystem 唯一签名
└── evidence.json          # 地址、bundle SHA、判定注记、gaps
```

## 2. 分析内容（61 个反编译函数；~199 native procs 中的核心业务）

- **GpuWorld/OwnedGpuWorldSubset 布局**：27 subset 槽（0xc0 步长）、HashMap<InstanceKey,u32>、紧凑 key 数组（f16 sort）、min/max 原子、类型化数据缓冲（type-id 断言）、instance_count / next_index；两个 TypedStorageBuffer（NaniMeshAtlasPartInfo/NaniDrawListInfo）。
- **实例生命周期**：`create_instances`/`extend_no_data`（分配→登记→数据写→min/max→计数，上限 panic "Max instance count reached for …"）；`update_instance_data<T>`（fast-path + hash 查找 + f16 sort_key + T 复制）；`remove_instance`（IndexCondenser swap_remove）。
- **instance_data**：21 个实例类型，元素尺寸按 `copy<T>` 步长确认（0x30/0x40/0x60/0x70）；`CurvedHalfTimberInstanceData` 三个构造器（power-of-two 细分段数、端帽计数、打包字段）。
- **systems**：`nani_maintain_instances` 建/删/改三路；20 个 `nani_update_*` 统一模式（puffin + 锁 subset + 变更遍历 + update_instance_data）；完整 ECS 查询签名来自 PDB。
- **MeshAtlas::add**（半径 sqrt(max|p|²)、索引 base_vertex 偏移、属性布局一致性、重复名 panic）；**TrimeshBuilder** attrib 记录；**buffers** DrawListParams 0x70B 结构与 per-kind 表扫描；**draw_ext::nani_draw** 间接绘制接线（NaniDrawListLength/NaniDrawList/NaniInstanceIndirection + raw/count 分派）。
- **消费者接线**：`gpu_generate_draw_lists`（0x140987dd0）cull_and_bucket 闭包（复用 evidence/render-architecture/0x140987dd0.c 与 pass-declaration.md §2.2）；frame 序 W22/W28/W25/W30/W34/W46/W53。

## 3. 共享类型（WP3 OWN）

`GpuWorld` / `OwnedGpuWorldSubset` / `ExternalGpuWorldSubset` / `GpuWorldSubsetKind`（27 变体）/
`InstanceHandle` / `InstanceKey` / `NaniInstanceData` trait / `InstanceDataMarker<D>` /
`MeshAtlas` / `MeshAtlasPart` / `NaniMeshAtlasPartInfo` / `NaniDrawListInfo` /
`DrawListParams` / `NaniDrawListLengthSsbo` / `NaniDrawListSsbo` / `NaniInstanceIndirectionSsbo` /
全部 nani 实例数据结构体 / `TrimeshBuilder` / `NaniActiveInstanceCollection`。

## 4. 未验证 / 后续

- 每 kind 数值表（`DAT_142aa14c0` max_draws_per_list、`DAT_142aba790`/`142c77010` max_instances）未读内存；`GpuWorldSubsetKind` 18 个变体名 U。
- 各实例类型字段名/顺序（PDB 无 field 记录）与 GPU shader 布局只按元素尺寸对齐；`nani_update_*` 组件→T 算法 U。
- GPU cull/bucket/剔除/LOD 算法属 shader 侧，WP3 只覆盖 CPU 侧接线与缓冲。
- 未做机器码差分验收；本交付为证据对齐骨架，需集成 owner 在共享入口接线（nani_maintain_instances→GpuWorld→gpu_generate_draw_lists/nani_draw）后另行校验。
