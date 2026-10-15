# render/nani/mesh_atlas.rs

> 文件作用：MeshAtlas / MeshAtlasPart：把多个 Mesh 合并进一张大网格（≤0x40 part；属性名集一致断言；重复名 panic；包围半径）。

函数总计 10：业务 3 / 内联无独立符号 1 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::mesh_atlas::MeshAtlasPart::new` | 0x140833250 | 65 | 空构造（3B 薄构造，O） | O | hex 手解 0x140833250（`mov eax,ecx; ret` 3B 薄构造；原 wp3 合同 §1） |
| `country_core::render::nani::mesh_atlas::MeshAtlas::add` | 0x140a36200 | 75 | 8 步（O）：indices 空→panic "invalid mesh"；Vertex_Position 包围半径 sqrt(max(x²+y²+z²))；parts.len()<0x40 断言；base_vertex=现有元素数；首个 mesh 直接 clone 否则索引+base 后比较排序属性名集（不同 → Err anyhow "Mesh attribute mismatch…"，不回滚）→ 追加 MeshAtlasPart{index_offset,index_count,radius,0}；重复名 → panic "Duplicate atlas mesh name" | O | render-top/decomp/decomp-0x140a36200.txt（原 B_n_world；wp3/contracts/mesh_atlas.md §2（0x140a36200）） |
| `country_core::render::nani::mesh_atlas::impl$10::clone_behavior` | 0x140002370 | 154 | Component 派生薄封装（3B；与 is_exclusive<FunctionSystem> 同址 ICF，0x140002370） | O | hex 手解 0x140002370（`xor eax,eax; ret` 3B；原 wp3 合同 §3） |

## 内联/无独立符号函数

- `country_core::render::nani::mesh_atlas::MeshAtlas::clear`（行 141）— 清空图集（行 141）—— I

## 派生胶水（6 项：closure、default、fmt 等）

6 项 glue = default×2 + fmt + add 的 3 个 closure（半径/属性比较闭包）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `MeshAtlas::add`（0x140a36200）—— 8 步合并算法
处理链（O，wp3/contracts/mesh_atlas.md §2）：
1. indices 空 → panic "invalid mesh"；
2. `Vertex_Position` 包围半径 = `sqrt(max(x²+y²)+z²)`（f32::max 语义；属性判别式非 Float3 → panic "incorrect attribute type"；属性名缺失 → expect "no entry found for key"）；
3. `parts.len() < 0x40` 断言（MAX_PARTS=64）；
4. `base_vertex` = 现有元素数；
5. 首个 mesh 直接 clone；否则**索引 + base 后比较排序属性名集**（不同 → `Err anyhow "Mesh attribute mismatch…"`，**不回滚**——索引已追加才比较，O 细节）；
6. 追加 `MeshAtlasPart{index_offset, index_count, radius, 0}`（16B）；
7. 重复名 → panic "Duplicate atlas mesh name"（**此时布局已追加**，O 细节）。
- 布局：mesh 0x00（0x78B）、layouts Vec<MeshAtlasPart(16B)>@0x78、parts HashMap<AtlassedMeshName,u32>@0x90；PDB `ComputedLayout` 是链接器别名。
- 消费方：`nani_maintain_instances` 的 `NaniInstanceDesc::Named` 经 MeshAtlasLibrary `get_by_name`（part.subset==kind 线性查找）；`GpuWorld::new` 建 `NaniMeshAtlasPartInfo` 缓冲（BUFFER_LEN=0x6c0）。

## 证据注记（tg_rn1）
- **MeshAtlas 布局**（O）：mesh 0x00（0x78B）、layouts Vec<MeshAtlasPart(16B)>@0x78、parts HashMap<AtlassedMeshName,u32>@0x90；PDB 的 `ComputedLayout` 是链接器别名，元素就是 `{index_offset, index_count, radius, 0}`。
- **add 签名**（O）：`add(&mut self, name, &Mesh) -> Result<u32, anyhow::Error>`（u32 = 新 part 序号；调用方 MeshAtlasLibrary::add 包 "invalid mesh: " 上下文）。
- 半径 = `sqrt(max(x²+y²)+z²)`（f32::max 语义）；属性判别式非 Float3 → panic "incorrect attribute type"；属性名缺失 → expect "no entry found for key"。
- **属性不一致不回滚**（索引已追加才比较）；**重复名 panic 时布局已追加** —— 两个 O 细节。
- MAX_PARTS = 0x40（一张图集最多 64 part）。
