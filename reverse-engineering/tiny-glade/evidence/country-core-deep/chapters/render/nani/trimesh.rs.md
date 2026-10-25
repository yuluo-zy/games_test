# render/nani/trimesh.rs

> 文件作用：Trimesh / TrimeshBuilder：CPU 三角网格增量构造（≤64 三角形 meshlet 切块、属性对齐降序、build 打包 GPU 缓冲并算包围球）。

函数总计 8：业务 5 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::trimesh::TrimeshChunk::index_count` | 0x14129dbc0 | 24 | movzx eax, word [rcx+8]：u16 索引数（5B） | O | hex 手解 0x14129dbc0（rne-read --hex 本批；原 hex 手解 0x14129dbc0（rne-read --hex 本批；原 B_n_world §1（0x14129dbc0））） |
| `country_core::render::nani::trimesh::Trimesh::builder` | 0x14129dbd0 | 32 | 把索引缓冲切成 ≤64 三角形 meshlet（块内顶点去重成局部 u8 下标；Vec::with_capacity(0x40/0xc0)）；旧骨架完全缺失的主体 | O | render-top/decomp/decomp-0x14129dbd0.txt（原 B_n_world §1/§2.9（0x14129dbd0）） |
| `country_core::render::nani::trimesh::TrimeshBuilder::positions` | 0x14129e890 | 106 | 置 positions 属性下标（= 当前 attribs.len()）→ attrib<Vec3>("positions", …)（98B） | O | render-top/decomp/decomp-0x14129e890.txt（原 B_n_world §1/§2.9（0x14129e890）） |
| `country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>` | 0x140905080 | 111 | 0x18B 记录 {data ptr+byte_len, elem_size u16, align u8}（**无属性名**；+0x12=对齐=4；按对齐降序强制）；data 空/不足 expected → panic；vertex_count 首个定值后续必须相等 assert_failed<usize,usize> | O | render-top/decomp/decomp-0x140905080.txt（原 B_n_world §1/§2.9（0x140905080/0x1412a90c0 两份 ICF）） |
| `country_core::render::nani::trimesh::TrimeshBuilder::build` | 0x14129e900 | 140 | 打包每块"三角形+逐顶点属性"进 GPU 缓冲（usage 0x20020），算包围球，device_address 回写；TrimeshChunk{address,index_count:u16,…} 0x20B | O | render-top/decomp/decomp-0x14129e900.txt（原 B_n_world §1/§2.9（0x14129e900）） |

## 派生胶水（3 项：closure 等）

3 项 glue = builder::closure$0 ×2 + attrib::closure$0（meshlet 去重/属性比较闭包）。

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `Trimesh::builder`（0x14129dbd0）—— meshlet 切块
算法（O）：把索引缓冲切成 **≤64 三角形 meshlet**（TRIANGLES_PER_CHUNK；块内顶点容量 0xc0 = 64×3）→ **块内顶点去重成局部 u8 下标** → 三角形把 3 局部下标打包 `l0 | l1<<8 | l2<<16` → `Vec::with_capacity(0x40/0xc0)`。旧骨架完全缺失的主体（B_n_world §2.9）。
- `TrimeshBuilder` 0x50B：positions_index: Option<usize>@0、chunks Vec@0x10、attribs Vec@0x28、input_max_index: isize@0x40、vertex_count@0x48。

### 2. `positions` / `attrib<T>`（0x14129e890 / 0x140905080）
- `positions(self, &[Vec3])`：置 `positions_index = attribs.len()`（记录**属性下标**，不是 bool）→ `attrib<Vec3>("positions", …)`。
- `attrib` 0x18B 记录 `{data ptr+byte_len, elem_size u16, align u8@0x12}`（**无属性名**；Vec3 = elem 0xc / align 4）；**强制对齐降序**；四个断言：data 空 → panic、不足 expected → panic、vertex_count 首个定值后续不等 → `assert_failed<usize,usize>`。

### 3. `build`（0x14129e900）—— GPU 打包
每块打包"三角形 + 逐顶点属性"进 GPU 缓冲（usage 0x20020）→ 算包围球 → `device_address`（`Buffer::device_address` 0x140bd4c00）回写 → `TrimeshChunk{address@0（先缓冲内偏移，建缓冲后 += device_address）, index_count u16@8, …}` 0x20B。
- 消费方：软件 BVH（`RtTriangles`/`RtBvh`）绑定与 `ray_trace_gi`/`water_surface` 的 `bind_ray_tracing`（CALL-CHAIN 的软件光追数据源）。

## 证据注记（tg_rn1）
- **TrimeshBuilder 0x50B**（O）：positions_index: Option<usize>@0、chunks Vec@0x10、attribs Vec@0x28、input_max_index: isize@0x40、vertex_count@0x48；`positions` 记录 attrib 下标（[rdx+8]=attribs.len()），不是 bool。
- **attrib 0x18B 记录**（O）：{data ptr+byte_len, elem_size u16, align u8 @0x12}，**无属性名**；Vec3 = elem 0xc / align 4；强制对齐降序；四个断言（空/不足/不等）。
- **meshlet**（O）：每块 ≤64 三角形（TRIANGLES_PER_CHUNK），顶点容量 0xc0 = 64×3；三角形把 3 局部下标打包 `l0|l1<<8|l2<<16`；块内顶点去重成局部 u8 下标。
- **TrimeshChunk 0x20B**（O）：address@0（先缓冲内偏移、建缓冲后 += device_address）、index_count u16@8、…；`build` usage 0x20020；包围球。
- `BufferDeviceAddress`（rhapsody::raw::buffer::Buffer::device_address 0x140bd4c00 的占位）—— B_i_nani §4 已并入 `render/ssbo.rs::rhapsody_boundary::GraphicsDevice`。
