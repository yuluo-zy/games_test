# BrickTransformSsbo 锚定与实现 — geometry::instanced_wall

日期: 2026-10-17
角色: restorer（impl-brick）
协议: `planning/2026-10-16-logic-verify-protocol.md` / `planning/2026-10-17-recovery-sop.md`
不变量: 只以 fresh 反编译/机器码为准；缺字段登记不编造；只改 country-core 内
geometry/render/asset_libraries 相关文件（外加一处必要的邻域测试类型 derive）。
证据目录: `recovery/agent-team/planning/impl-brick/results/`（r1–r11，ghidra `--worker shared`）。

## 一、锚定结论（全部 fresh）

| 项 | 值 | 证据 |
|---|---|---|
| 元素大小 | **0x60 B**（96） | `grow_one<BrickTransformSsbo>` 0x14078cac0（×0x60）；`push<BrickTransformSsbo>` 0x1412e4450 整块拷贝 12 qword |
| 对齐 | 4 | `new_storage_buffer` 0x1412e5800 影子 `__rust_alloc(size=0xC000000, align=4)` |
| `StorageElement::NAME` | **"BrickTransforms"**（15 B） | `TypedStorageBuffer::new` 0x141618890 `LEA R8,[0x142c7fd18]`+`R9D=0xf`；read_memory 直读 |
| `BUFFER_LEN` | **0x200000** | 0x141618890 `CMP R9,0x200001`（供给数据元素数上限）；缓冲字节 `0xC000000 = 0x200000×0x60` |
| `CHUNK_COUNT` | **0x10000** | 0x1412e5800 byte_map `memset 0x10000`+哨兵 @0x10000=0x20（长 0x10001）；free 栈 0x10000；影子 0x200000 |
| bitset 字数 | **0x400** each | 0x1412e5800 `from_iter<AtomicUsize>` 两次，长度 0x400 |
| SsboId | **0xeb575d061b90ebf5** = xxh3_64("BrickTransforms") | `add_new<BrickTransformSsbo>` 0x1412bf670 立即数 `-0x14a8a2f9e46f140b` |
| 完整类型名 | "country_core::geometry::instanced_wall::BrickTransformSsbo"（58 = 0x3a） | 0x141618890 panic 串 0x142a9d7ff/0x3a；`add_new` BufferState name |
| `data.is_none()` 断言 | 文案 32 B = "assertion failed: data.is_none()" | 0x1412e5800 `TEST R8,R8/JNZ panic`；串 0x142c2ed38 |

### 字段布局（default 0x1408587c0 逐写 + 铺砖写面 r2-decomp 逐偏移）

| 偏移 | 大小 | 字段（名 U，偏移 O） | 默认值 | 铺砖写面 |
|---|---|---|---|---|
| 0x00 | 0x30 | `affine_columns: [[f32;3];4]` | 全 0 | 12×f32 仿射（两基向量 + 深度轴 + 平移） |
| 0x30 | 0x10 | `deformation: [f32;4]` | 全 0 | 写 0 |
| 0x40 | 0x0c | `surface: [f32;3]` | **[-100.0; 3]** | 写 -100.0 ×3（常量 0x142a8c5e0 = 0xc2c80000c2c80000 + 0xc2c80000） |
| 0x4c | 0x04 | `seed: u32` | 0 | 逐砖序号 `uVar42` |
| 0x50 | 0x04 | `material: u32` | 0 | `BrickSourceId` 编码值（奇偶选路） |
| 0x54 | 0x08 | `wall_distance: [f32;2]` | [0,0] | [0.1, 0.1] |
| 0x5c | 0x04 | `flags: u32` | 0 | 6 |

96 B 全部有归属，无未锚定字节。字段语义名（seed/material/wall_distance/flags）为 U；
与既有 `recovered_building::wall_constructor::WallBrickInstance` 的 96 B 观察布局一致。

## 二、落地

1. `src/geometry/instanced_wall.rs`
   - 新增 `BrickTransformSsbo`（`#[repr(C)]`、96 B）+ `Default`（0x1408587c0 模式）。
   - `StorageElement`（NAME/BUFFER_LEN）、`unsafe impl SparseSsbo`（CHUNK_COUNT=0x10000）。
   - `impl asset_libraries::ssbo_library::SsboType`：`Storage = SparseInstanceBuffer<Self>`，转发
     `SparseInstanceBuffer::new_storage_buffer`（= 原 `add_new<BrickTransformSsbo>` 0x1412bf670 路径）。
   - `impl pass_builder::pass_resource_access::SsboType`：`rg_buffer_hash = xxh3_64(NAME)`（原
     `get_rg_buffer<BrickTransformSsbo>` 0x1412c9530 同键）。
2. `src/render/sparse_instance_buffer.rs`
   - `SparseSsbo` 增加 `Default` 超 trait；影子由 `unsafe zeroed::<T>()` 改为 **`vec![T::default(); chunks*32]`**。
     证据：0x1412e5800 的影子初值循环写 Brick 的 Default 模式（surface=-100），PDB inline 记录含
     `alloc::vec::from_elem`/`Vec::extend_with`（= `vec![elem; n]`）。
   - 新增 `NonZero` 测试类型，钉死“影子取 Default 而非零”。
3. 泛型面复核（均与 fresh 一致，无需改）：
   - `push` 0x1412e4450（整块 0x60 B、槽位 `chunk<<5|count`、active+dirty 置位）；
   - `clear` 0x1412e4760；`take_out_waste` 0x1412e3d00（逆序回填 free 栈、byte_map=0、清 active）；
   - `gpu_flush` 0x1412e2f10（先 take_out_waste、dirty 字合并为 `[start<<5, end<<5)` 元素区间）；
   - `buffer` 0x140005c50（@0x68）、`hide` 0x1412e3c00、`show` 0x1412e3c80。
4. 单测（`instanced_wall`）：
   - 布局/常量：0x60 B、align 4、逐字段 offset、NAME 长度 0xf、BUFFER_LEN 0x200000、CHUNK_COUNT 0x10000；
   - Default 逐字节 = 0x1408587c0；元素打包逐字节（0x00..0x30 仿射、0x4c/0x50/0x54/0x58/0x5c）；
   - SsboId = 0xeb575d061b90ebf5（库键与 pass 哈希一致）；
   - `AssetSsboLibrary::add_new::<BrickTransformSsbo>` 注册：byte_map 0x10001/哨兵、影子 0x200000 且
     全为 Default、bitsets 0x400、free 栈 [0xffff..0]、`push` 后 `data[0] == 实例`、缓冲字节 0xC000000。

## 三、登记（未锚定 / 未接线，写解锁条件）

1. **`TypedStorageBuffer::new` 无初始数据分支的零填阈值未锚定**。
   观测：MeshAtlasPartInfo（0x6c0 元素 × 0x10 B）在 `new` 内 `vec![..; n]` 零填并上传；
   BrickTransform（0x200000×0x60）、Sparkle（0xC0000 B）、RoofTile、WoodenPlank、Lantern（0x80000 B）
   均不构造初始数据（`create_buffer(data=None)`）。阈值被编译期折叠，两个方向都不可读出；
   由数据点只知：零填成立当字节 ≤ 0x6c00，不成立当字节 ≥ 0x80000（或元素 ≤ 0x6c0 vs ≥ 0x8000）。
   现实现对所有 `BUFFER_LEN > 1` 零填上传；对 BrickTransform **GPU 初始内容同为全零**，仅多一次
   0xC000000 B 初始上传。**解锁：锚定源阈值后对齐。**
2. **collision 铺砖写面未实现**（`find_mesh_mesh_intersections` 0x140a97b70，r2-decomp 逐写已锚定：
   中点步进 0.5、方向/法线归一化、×100 饱和转 i32 进 `calculate_hash<IVec3>`）。宿主类型现已存在，
   剩余为把 `SparseInstanceBufferWriter::push<BrickTransformSsbo>` 与折线循环接上（collision 域）。
3. **stairs reset_edge 的两次 `clear<BrickTransformSsbo>`**：类型与泛型 `clear` 已就绪；真正接线仍需
   `MagicSupports` 的两个 render 分配/写锁（render 域）。邻域 restorer 已在
   `resources/stairs/visual.rs` 用 `SparseInstanceBuffer<Elem>` 建立 clear 语义测试（现 4/4 通过）。
4. 字段语义名（seed/material/wall_distance/flags）与着色器语义映射 U；字节偏移已全部锚定。

## 四、验证

- `cargo check -p country-core --lib`（default）：**Finished，0 error**。
- `cargo test -p country-core --lib instanced_wall`：**10 passed / 0 failed**。
- `cargo test -p country-core --lib sparse_instance_buffer`：**17 passed / 0 failed**。
- `cargo test -p country-core --lib resources::stairs::visual`：**4 passed / 0 failed**。
- `cargo test -p country-core --lib`（全库 baseline）：**2046 passed / 1 failed**；唯一失败为
  `components::drawable::tests::nani_bundle_layout_matches_write_components`（80≠72，他域 WIP，非本改动）。
- `--features native-physics` lib check 当前被他组 WIP 阻断：`resources/prefab.rs`（Aabb3: Default）、
  `systems/collision/visualize_colliders.rs`（`.kind`）、`systems/collision/world.rs`（unsafe from_raw），
  均不在本任务改动文件内；本任务文件在该 feature 下早前一轮检查为 0 error。
- 附带一处必要的邻域编译修复：`resources/stairs/visual.rs` 测试 `Elem` 增加 `#[derive(Default)]`
  （`SparseSsbo` 新超 trait 所需；纯测试类型，无业务逻辑改动）。

证据文件：`planning/impl-brick/results/r1.json` … `r11.json`（含反编译/反汇编/read_memory 原始返回）。
