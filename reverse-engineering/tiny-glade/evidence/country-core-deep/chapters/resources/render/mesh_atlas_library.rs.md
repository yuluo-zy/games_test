# resources/render/mesh_atlas_library.rs

> 文件作用：**MeshAtlasLibrary 资源**——按"世界子集(world subset)→图集(atlas)→名称→网格"组织 GPU 网格图集，
> 并把各图集注册进 rt BVH 构建器。实现是哈希表容器：子集图为字节 key 哈希表（base\*+0x0、mask+0x8、len+0x18），
> 名称图集为 PrefabName/NameHash key。`register_meshes` 的 panic 串确认源码路径
> `crates/country-core/src/resources/render/mesh_atlas_library.rs`，字符串：
> "Updating mesh atlas for world subset " / "Not registering atlas for world subset "。
> 消费方：render loop 的 mesh 图集绑定、rt BVH（register 时调 RtBvhBuilderGlueResource::new）。

函数总计 23：业务 11 / 内联无独立符号 1 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::render::mesh_atlas_library::impl$1::from` | 0x140ae6430 | 25 | From 转换（PrefabName→图集 key 族）：经 func_0x000141556130（name→hash）、FUN_140d79f90（String/name 处理）产 key，尾部 Arc 引用计数减（LOCK/DEC/UNLOCK，归零释放 0x14064eb30） | O | roofs-rt/decomp/decomp-0x140ae6430.txt |
| `country_core::resources::render::mesh_atlas_library::impl$2::from` | 0x140ae6570 | 31 | 同族 From 转换（另一 key 类型） | O | roofs-rt/decomp/decomp-0x140ae6570.txt |
| `country_core::resources::render::mesh_atlas_library::impl$3::from` | 0x140ae6660 | 37 | 同族 From 转换 | O | roofs-rt/decomp/decomp-0x140ae6660.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>` | 0x140a37760 | 74 | 名称→图集插入（哈希表 insert，含 PrefabName 字符串逐字节散列/比较，容量增长） | O | roofs-rt/decomp/decomp-0x140a37760.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >` | 0x140a371d0 | 74 | 同上（&str key 变体） | O | roofs-rt/decomp/decomp-0x140a371d0.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_world_subset` | 0x140ae6750 | 104 | 字节子集 key 哈希查找（FUN_1412e7650 hash），返回子集图集 | O | roofs-rt/decomp/decomp-0x140ae6750.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas` | 0x140ae6880 | 113 | 清空指定子集图集（同 key 查找 + clear） | O | roofs-rt/decomp/decomp-0x140ae6880.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> >` | 0x140a37e50 | 123 | 按名称清空图集 | O | roofs-rt/decomp/decomp-0x140a37e50.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<ref$<asset::prefab::PrefabNameRef> >` | 0x140a382d0 | 135 | 按 PrefabName 查找图集 | O | roofs-rt/decomp/decomp-0x140a382d0.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>` | 0x140a38150 | 135 | 按 NameHash 查找图集（utils::name_hash::NameHash 作 key） | O | roofs-rt/decomp/decomp-0x140a38150.txt |
| `country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes` | 0x140ae6b90 | 143 | 逐世界子集注册网格：子集变更时重建图集并调 RtBvhBuilderGlueResource::new（0x14067dd80×2）接入 rt；panic "Updating mesh atlas for world subset "/"Not registering atlas for world subset "，源 mesh_atlas_library.rs | O | roofs-rt/decomp/decomp-0x140ae6b90.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::render::mesh_atlas_library::impl$4::from`（行 43）— 第 4 个 From 转换（内联进调用点）

## 派生胶水（derive/宏生成；不计独立业务）

`PrefabName>`、`closure`、`default`、`eq`、`fmt`、`hash` 等 11 项（图集 key/句柄类型的 derive 族 + add/get_by 的闭包 + impl$6::default 等）

## 布局/结构要点

- 子集图：`{entries_base@+0x0, bucket_mask@+0x8, … , len@+0x18}`，查找经 FUN_1412e7650（hash，字节/字符串 key）→ bucket 探测。
- `add`/`get_by_name` 均为 0x140a37* 邻近地址族（同一哈希表单态化的不同 key 实例）。
- 交叉域：与 `utils::name_hash::NameHash`（audio-misc 域）、`asset::prefab::PrefabName`（session-save 域）、rt BVH 构建器（本域 rt/bvh_builder）关联。

## 实现方式（算法级）

- `register_meshes`（0x140ae6b90）：逐世界子集注册网格 —— 子集变更时重建图集（字节 key 哈希表，FUN_1412e7650 hash）并调 `RtBvhBuilderGlueResource::new`（0x14067dd80）×2 接入 rt BVH；panic "Updating mesh atlas for world subset "/"Not registering atlas for world subset "（0x142ae3438/0x142ae3518）。
- `add`/`get_by_name`/`clear_*`：同一哈希表族的 key 单态化（PrefabName 串逐字节散列/比较、NameHash 直接 key）；`impl$1::from`（0x140ae6430）尾部 Arc 引用计数减（归零释放 0x14064eb30）。
