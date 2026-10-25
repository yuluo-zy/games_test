# A3 res_render：`resources/render/*` 按机器码重写

标记：O = 机器码直接观察，I = 推断，U = 未知。测试台：`/tmp/h_res_render`（147 个测试全过，其中本组 22 个）。

## 过程清单（procs.tsv，`country_core::resources::render::*`）
Affine3Packed x4（0x14089d4d0/d500/d550/d560）；mesh_atlas_library：add<&str> 0x140a371d0、add<PrefabName> 0x140a37760、
add 的 closure 0x140a37cf0、clear_atlas_by_name 0x140a37e50、get_by_name<NameHash> 0x140a38150、get_by_name<&PrefabNameRef> 0x140a382d0、
Debug 0x140a3d2e0、From x3 0x140ae6430/6570/6660、get_by_world_subset 0x140ae6750、clear_world_subset_atlas 0x140ae6880、register_meshes 0x140ae6b90；
spatial_resolve_offsets::default_value 0x140b31d10。`lantern` 只有泛型实例，无自有过程。构造点：`insert_resources` 0x140940d80（from_iter 0x141449580 / fold 0x141478760）。

## Affine3Packed（O）
48 B = 12 个 f32（3 个旋转列 + 平移）。from_mat3_translation：Mat3 的 9 个连续 f32 + Vec3；from_cols：四个 Vec3 引用；add_y：`[+0x28] += y`；impl$1::from 来自 Affine3A（丢 Vec3A 填充）。

## spatial_resolve_offsets::default_value（O）
返回 `Option<Self>`：`[+0]=1`（最后写入），载荷 `+4..+0x2004` = 512 个 `int4(x,y,0,0)`。用脚本回放全部存储（movups/movsd/movaps/movss/mov/movabs）得到，覆盖 0x2004 B 无遗漏。
着色器 `int4 spatial_resolve_offsets_array[512]` 与 `TypedStorageBuffer::new_storage_buffer` 0x141611380（大小 0x2000、用途 0x12、对齐 4、名 "SpatialResolveOffsets"）吻合。
旧版错：`[u8;2048]`/128 项/起点 0x14/把 Some 判别式当“计数”。

## MeshAtlasLibrary（O）
- 布局 0x50 B：world_subsets（std HashMap<GpuWorldSubsetKind, WorldSubsetMeshAtlas>，桶 0xc8）@0；prefab_parts（nohash，桶 0x20，值 Vec<PrefabMeshAtlasPart 8B>）@0x30。
- WorldSubsetMeshAtlas 0xc0 B = `Option<Handle<Mesh>>`(@0/@8) + MeshAtlas(@0x10)。默认 = Mesh::new("MeshAtlas")（label 指针 "MeshAtlas"，9）、空 layouts/parts。
- 默认构造：对 27 个 subset 全部预置默认图集（from_iter），prefab 表空。
- add：先算名字哈希（&str 直接 `NameHash::compute`；PrefabName 经 "prefab:" 前缀），再取/补 subset 图集，`MeshAtlas::add`，Err -> `panic!("Error adding prefab {:?} to atlas of shader {:?}: {:#}")`（`{:#}` 由 spec flags=4 确认），最后在 prefab 表追加 `{part_index u32@0, subset u8@4}`。**旧注释说的 "invalid mesh: " 上下文包装不存在**（该串只是 "MeshAtlas" 字符串池相邻内容）。
- get_by_name：返回 `&[PrefabMeshAtlasPart]`（读桶 +0x10/+0x18 = ptr,len），缺失 panic "Atlas library: unknown prefab {:?}"。旧版返回单个 MeshAtlasPart，错。
- get_by_world_subset：返回 `&WorldSubsetMeshAtlas`，缺失 panic "Atlas library: unknown subworld {:?}. Make sure to register some meshes in nani_meshes.ron"。
- clear_atlas_by_name：prefab Vec `len=0`（键保留）；对每个 subset 图集的 `parts` 表 remove 该键（hashbrown erase），不动 mesh/layouts。
- clear_world_subset_atlas：attributes.clear、indices.len=0、parts.clear、layouts.len=0；再对所有 prefab Vec 做 `retain(subset != kind)`（原地压缩）。`registered` 句柄保留。
- register_meshes：遍历 subset；`indices` 空 -> warn!("Not registering atlas for world subset {:?}: fused mesh has no verts")；已登记 -> debug!("Updating mesh atlas for world subset {:?}")，`get_mut(handle).unwrap()` 后整体替换为 `atlas.mesh.clone()`；未登记 -> `AssetMeshLibrary::add({mesh.clone(), Some(format!("{:?}:atlas", kind))})` 并保存句柄。日志级别由 Record 里的 level 立即数 4/2 确认。
- AtlassedMeshName：newtype over NameHash，Debug = `AtlassedMeshName(NameHash { hash: N })`（vtable 指向 0x1414599b0 = NameHash 的 debug_struct）。
- MeshAtlasesModifiedCmd（I）：drop 与 HashSet<GpuWorldSubsetKind> 同一 ICF 代码，字段名 U。

## 仍为 I/U
nohash 哈希器（不可观察，用标准 HashMap）；PrefabName/PrefabNameRef 是 asset 外部 crate，这里放了最小占位；PrefabMeshAtlasPart 字段名；MeshAtlasesModifiedCmd 字段名；WorldSubsetMeshAtlas 的 Option 字段名；hashbrown 遍历序（register_meshes 日志/句柄顺序）。
