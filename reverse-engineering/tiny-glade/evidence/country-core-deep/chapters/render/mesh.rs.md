# render/mesh.rs

> 文件作用：**CPU 网格（Mesh）+ 顶点属性值（VertexAttributeValues）**——`Mesh`（0x78 = 120 B）：索引 Vec<u32> +
> 属性 `HashMap<String, VertexAttributeValues>` + 源文件字段；提供位置/法线/UV/颜色访问、包围盒、碰撞盒/半径、
> 顶点保留重建（retain_vertices）、三角形视图（as_triangles）、属性追加。证据：
> `reconstruction/country-core-render/wp4/contracts/mesh.md`（O 全量）+ `evidence/building-gates/B-mesh-*` 反编译。

函数总计 46：业务 26 / 内联无独立符号 12 / 派生胶水 8

## 布局表（O，wp4 mesh 合同）

### `Mesh` — 0x78 B

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `indices: Vec<u32>`（ptr/cap/len） | set_indices 拷 3 u64；retain_vertices 读 +0x08/+0x10 |
| +0x18 | u64（`new` 置 `0x8000000000000000` 哨兵） | Mesh::new（语义 U） |
| +0x20 | u64 | I |
| +0x28 | `attributes: HashMap<String, VertexAttributeValues>`（RandomState +0x28/+0x30、buckets +0x38、mask +0x40、growth +0x48、items +0x50） | set_attribute/positions/aabb_bounds |
| +0x58 | u64（RandomState 键尾） | I |
| +0x60 / +0x68 / +0x70 | u64 ×3（`new` 把参数拷到 +0x68/+0x70；source_file 相关） | I（+0x68/+0x70 与 A3_mesh 的 `Option<PathBuf>` 对应，U 语义） |

属性键：`"Vertex_Position"`（len 15，O）；`"Vertex_Normal"`/`"Vertex_UV"`/`"Vertex_Color"`（I，由访问器名推出）。

### `VertexAttributeValues`（enum，O 判别域 0..3）

变体（I，按 set_attribute/positions/uvs 判别）：`Float(Vec<f32>)`、`Float2(Vec<[f32;2]>)`、`Float3(Vec<[f32;3]>)`、
`Float4(Vec<[f32;4]>)`、`Int(Vec<i32>)`，…（判别 2 = [f32;2]、3 = [f32;3] 由 uvs/positions 证实；变体数/序 U）。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `Mesh::new` | 0x140a07390 | 17 | 猜测：构造空网格；证实：空索引/空属性 + 哨兵 | **O**（166 B）：空 indices、空 attributes（RandomState）、`+0x18 = 0x8000…` 哨兵、参数拷到 +0x68/+0x70 | render-top/decomp/decomp-0x140a07390.txt（原 wp4 mesh） |
| `Mesh::set_source_file` | 0x140a07440 | 21 | 猜测：设源文件；证实：平凡赋值 | **O**（69 B）：源文件路径串赋值 | render-top/decomp/decomp-0x140a07440.txt（原 wp4 mesh） |
| `Mesh::set_indices` | 0x140a07490 | 24 | 猜测：设索引；证实：整体替换 Vec | **O**（60 B）：释放旧 indices、搬 {ptr,cap,len} | render-top/decomp/decomp-0x140a07490.txt（原 wp4 mesh） |
| `Mesh::arrayed_indices` | 0x140a074d0 | 31 | 猜测：三重化索引；证实：`Vec<[u32;3]>` from_iter ChunksExact | **O**（104 B）：`from_iter<[u32;3], Map<ChunksExact<u32>,…>>` | render-top/decomp/decomp-0x140a074d0.txt（原 wp4 mesh） |
| `Mesh::add_color` | 0x140a07540 | 36 | 猜测：加统一色；证实：位置数×color 属性写入 | **O**（316 B）：查 `Vertex_Position`；`vec::from_elem(color, pos_count)`；`set_attribute<Vec<[f32;3]>>` | render-top/decomp/decomp-0x140a07540.txt（原 wp4 mesh） |
| `Mesh::add_color_self` | 0x140a07680 | 41 | 猜测：逐顶点色；证实：color = f(position) | **O**（423 B）：逐顶点计算颜色并写属性 | render-top/decomp/decomp-0x140a07680.txt（原 wp4 mesh） |
| `Mesh::aabb_bounds` | 0x140a07870 | 50 | 猜测：AABB；证实：`Aabb3 { min:+FLT_MAX, max:-FLT_MAX }` 种子 + expand | **O**（505 B）：查 `Vertex_Position`（disc 3）；空 → 极端 Aabb3；返回 24 B `{min, max}` | render-top/decomp/decomp-0x140a07870.txt（原 wp4 mesh） |
| `Mesh::collision_bbx` | 0x140a07a70 | 64 | 猜测：碰撞盒；证实：xz 扩展碰撞盒 | **O**（181 B）：从位置构建（xz 扩展） | render-top/decomp/decomp-0x140a07a70.txt（原 wp4 mesh + building-integrate/B-core） |
| `Mesh::positions` | 0x140a07b30 | 69 | 猜测：位置视图；证实：`(&[u32], &[[f32;3]])` | **O**（361 B）：查 `Vertex_Position`（disc 3 否则 panic）；返回 `{indices.ptr, ptr+len*4, pos.ptr, pos.len}` | render-top/decomp/decomp-0x140a07b30.txt（原 wp4 mesh） |
| `Mesh::positions_without_indices` | 0x140a07ca0 | 76 | 猜测：仅位置；证实：位置切片 | **O** | render-top/decomp/decomp-0x140a07ca0.txt（原 wp4 mesh） |
| `Mesh::normals_without_indices` | 0x140a07e00 | 81 | 猜测：法线；证实：`Vertex_Normal` 属性 | **O/I**（名由模式推断） | render-top/decomp/decomp-0x140a07e00.txt（原 wp4 mesh） |
| `Mesh::uvs_without_indices` | 0x140a07f50 | 86 | 猜测：UV；证实：`Vertex_UV` 属性（disc 2） | **O/I** | render-top/decomp/decomp-0x140a07f50.txt（原 wp4 mesh） |
| `Mesh::retain_vertices` | 0x140a08090 | 98 | 猜测：顶点保留；证实：索引+属性+重映射四步 | **O**（318 B）：(1) indices.retain(keep) (2) 逐属性 `retain_by_index` (3) 重写标志（保留→新下标、丢→-1）(4) 逐索引重映射，-1 → panic | render-top/decomp/decomp-0x140a08090.txt（原 wp4 mesh） |
| `Mesh::collision_radius_xz` | 0x140a081e0 | 113 | 猜测：碰撞半径；证实：`sqrt(x²+z²)` 最大值 | **O**（149 B）：遍历索引→位置求 max | render-top/decomp/decomp-0x140a081e0.txt（原 wp4 mesh） |
| `Mesh::as_triangles` | 0x140a08280 | 119 | 猜测：三角形视图；证实：`Vec<Triangle>` | **O**（438 B）：索引 3 个一组 → `Triangle`（位置）collect | render-top/decomp/decomp-0x140a08280.txt（原 wp4 mesh） |
| `VertexAttributeValues::array_length` | 0x140949170 | 135 | 猜测：长度；证实：返回 Vec len | **O**（5 B）；PDB 别名 `sync_point_count`（ICF） | hex 手解 0x140949170（本批 rne-read：`mov rax,[rcx+0x18]; ret`；原 wp4 mesh） |
| `VertexAttributeValues::append` | 0x140a08440 | 143 | 猜测：追加；证实：同变体跳表分派 | **O**（570 B，round3 round2 复查修正：原写 170 B 与 PDB size 不符）：同变体追加（jump table，元素尺寸 4/0x10/8/0xc），异变体 → panic fmt | render-top/decomp/decomp-0x140a08440.txt（原 wp4 mesh；pdb_exact size 570） |
| `set_attribute<Vec<[f32;4]>>` | 0x140a34170 | 158 | 猜测：属性写入；证实：存在替换/缺失插入 | **O** | render-top/decomp/decomp-0x140a34170.txt（原 wp4 mesh） |
| `set_attribute<Vec<[f32;3]>>` | 0x140a34c90 | 158 | 同上 | **O**（已反编译，building-gates/B-mesh-extra） | render-top/decomp/decomp-0x140a34c90.txt（原 wp4 mesh） |
| `set_attribute<Vec<[f32;2]>>` | 0x140a355f0 | 158 | 同上 | **O** | render-top/decomp/decomp-0x140a355f0.txt（原 wp4 mesh） |
| `set_attribute<Vec<f32>>` | 0x140a35b80 | 158 | 同上 | **O** | render-top/decomp/decomp-0x140a35b80.txt（原 wp4 mesh） |
| `set_attribute<Vec<i32>>` | 0x140a34700 | 158 | 同上 | **O** | render-top/decomp/decomp-0x140a34700.txt（原 wp4 mesh） |
| `set_attribute<VertexAttributeValues>` | 0x140a35240 | 166 | 猜测：预构建枚举写入；证实：直接存判别 | **O**（687 B）：name 存在 → 原位替换（dealloc 旧值、置新判别）；否则克隆 name 插入 | render-top/decomp/decomp-0x140a35240.txt（原 wp4 mesh） |
| `retain_by_index<closure>` | 0x140a36110 | 174 | 猜测：按下标保留；证实：闭包保留（retain_vertices 用） | **O** | render-top/decomp/decomp-0x140a36110.txt（原 wp4 mesh） |
| `from`（Vec 属性 From impls） | 0x140a0db00 / 0x140a0db90 | 178 | 猜测：转换；证实：`From<Vec<…>> for VertexAttributeValues` | **O**（impl$6/7::from） | render-top/decomp/decomp-0x140a0db00.txt（原 wp4 mesh） |

## 内联/无独立符号函数

- `append_attrib`（行 27）— **I**：append 的助手
- `as_f32x2` / `as_f32x3`（行 45/51）— **I**：类型化切片访问器
- `clear`（行 61）— **I**：清空（inline 宿主）
- `has_attribute`（行 75）— **I**：属性存在谓词
- `is_valid`（行 93）— **I**：有效性谓词
- `vertex_count`（行 128）— **I**：顶点数
- `size_in_bytes`（行 149）— **I**：字节大小（build_vao 依赖，O）
- `retain_by_index`（行 174）— 泛型入口（O 行为见上）
- 其余 derive/胶水：as_f32/as_f32x4/as_i32/as_ptr/component_count/stride（O/I 名，无独立过程）

## 派生胶水（8 项：clone ×2、closure ×6）

- clone ×2（Mesh/VertexAttributeValues derive）、closure$0 ×5 + closure$1（as_triangles/retain 等映射闭包）——不补。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 关键算法（O，wp4 mesh）
- **`retain_vertices`**（0x140a08090，四步）：(1) `indices.retain(keep)`；(2) 逐属性 `retain_by_index`（按下标保留闭包）；(3) 重写标志（保留→新下标、丢→-1）；(4) 逐索引重映射，**-1 → panic**。
- **`aabb_bounds`**（0x140a07870）：`Aabb3 { min:+FLT_MAX, max:-FLT_MAX }` 种子 → 查 `Vertex_Position`（disc 3；空 → 极端 Aabb3）→ expand → 返回 24B `{min,max}`。
- **`add_color`**（0x140a07540）：查 `Vertex_Position` → `vec::from_elem(color, pos_count)` → `set_attribute<Vec<[f32;3]>>`（统一色）；`add_color_self`（0x140a07680）逐顶点 `color = f(position)`。
- **`set_attribute<V>`**（0x140a34170 系 6 单态化）：存在 → **原位替换**（dealloc 旧值、置新判别）；缺失 → 克隆 name 插入（HashMap）。`set_attribute<VertexAttributeValues>`（0x140a35240，687B）直接存判别。
- **`arrayed_indices`**：`Vec<[u32;3]> from_iter ChunksExact`；`as_triangles`：索引 3 个一组 → `Triangle`（位置）collect。
- **`append`**（0x140a08440，570B）：同变体跳表分派（元素尺寸 4/0x10/8/0xc）；**异变体 → panic fmt**。
- **`collision_radius_xz`**：遍历索引→位置求 `sqrt(x²+z²)` 最大；`collision_bbx`：xz 扩展碰撞盒。

### 数据结构
`Mesh` 0x78B：indices Vec<u32>@0、哨兵 u64@0x18（`0x8000000000000000`，语义 U）、attributes HashMap<String,VertexAttributeValues>@0x28、source_file 相关 u64@+0x68/+0x70。属性键仅 `"Vertex_Position"`（len 15）O 证实；`Vertex_Normal/UV/Color` I。
- 消费链：`AssetMeshLibrary::add`（0x78B 整块）→ `load_json_as_mesh`（utils::load_json，68 调用方，B_x_out 遗留）→ `build_vao`（closure 用 `size_in_bytes` + 字节索引）→ `Trimesh::builder`/`mesh_atlas` 合并。

## 纠错/差异（wp4 mesh 合同）

- `+0x18` 哨兵 `0x8000…`（i64::MIN niche 风格）语义 U——勿当 Option 判别猜。
- 属性键名仅 `"Vertex_Position"` 证实；`Vertex_Normal/UV/Color` 为推断。
- `VertexAttributeValues` 变体数/序（判别 0..3 之外）U。
- 消费者：`AssetMeshLibrary::add`（0x78 B 整块）、`build_vao`（closure 用 `size_in_bytes` + 字节索引）、`load_json_as_mesh`（utils::load_json，B_x_out 遗留）。
