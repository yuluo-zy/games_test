# REBUILD-RENDER-SSBO — SparseSsbo / SparkleSsbo 落地 + Env36 ranges 收口（批 5 实现）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类；只以 fresh 反编译为准）
依据: reports/impl-unimplemented/REBUILD-UNIMPLEMENTED.md 批 5 + reports/interaction/render_bridge/{LOGIC,REBUILD-REMAINDER}.md
红线: 只以 fresh 反编译为准；缺类型不编造；不改他模块（RoofTileSsbo 存储形态归 roof/device 域，见 §登记）。
证据: tool-usage/results/impl-render-ssbo-{env36-fresh,sparkle-reg,sparkle-const,sparkle-layout,sparkle-xrefs,env36-disasm}.json
      （ghidra `--worker shared` 只读，2026-10-17 12:53–13:01 fresh）。

## 目标

render_loop Env36（0x1409b7fa0）被 `SparkleSsbo` 缺 `SparseSsbo` 实现阻塞：
原版每帧经 `SparseInstanceBuffer::to_draw_multi_instanced`（0x1412e4250）现算绘制区间，
Rust 旧实现用预计算 `SparkleDrawRangesView` 视图（U）。本批按 fresh 反编译落地类型并改 Env36 现算。

## 关键取证（fresh 反编译逐函数）

### Env36 0x1409b7fa0（`call_once<closure_env$36>`）反汇编核对（0x1409b7fa0..0x1409b81a0）

- `Components::get_resource_id(TypeId 0x8226cc786e679355/0x65cfda6a55a841ac)` → 取 `AssetSsboLibrary`
  （缺失 → `unwrap_failed`）；资源槽非空校验（`[+0x218]` 边界、`[+0x210]` 稀疏表、`[+0x1e0]` 槽 `[id*0x60+0x20]`）。
- `FrozenMap::get<SsboId>(&SsboId @0x142c2e3f0)`：**SsboId = 0x2bf3b1a4ac675864**（内存 8 B 小端）=
  `xxh3_64("SparkleSsbo")`。缺失 → `unwrap_failed`。
- 名字断言：`state.name.len() == 0x2d` + `memcmp(state.name, "country_core::resources::sparkle::SparkleSsbo", 0x2d)`。
- `as_any`（Ssbo 虚表 +0x18）取具体缓冲指针 RAX；非空 → `to_draw_multi_instanced<RoofTileSsbo>(out_vec, RAX)`
  （0x1412e4250，**接收者 = as_any 返回的 Sparkle 缓冲**；符号名 `<RoofTileSsbo>` 系与 `<SparkleSsbo>` 同形 ICF 合并）。
- `memcpy` 重排闭包环境（0xf8 B）→ `read_storage_buffer<SparkleSsbo>(0xc3483cd70105de2f)`（0x141631990）→
  `draw_multi_instanced(ranges)`（0x140907050）。
- **结论**：ranges 来自 `SparseInstanceBuffer<SparkleSsbo>::to_draw_multi_instanced()`，非 RoofTile 缓冲
  （find_functions 证实 `SparseInstanceBuffer<SparkleSsbo>`/`push<SparkleSsbo>`/`new_storage_buffer<SparkleSsbo>`
  均独立单态化；Env36 的 `<RoofTileSsbo>` 只读 byte_map/active，字段偏移跨 T 相同 → ICF）。

### SparkleSsbo 元素布局（`new_storage_buffer<SparkleSsbo>` 0x1412e5f50 + `push<SparkleSsbo>` 0x14120c710）

- `new_storage_buffer`：byte_map 长 **0x301**（`[0x300] = 0x20` 哨兵 → `CHUNK_COUNT = 0x300`）；
  影子 **0xc0000 B / 0x6000 元素 → `size_of = 32`**；free 栈 `[0x2ff .. 0]` 降序；active/dirty 各 **0xc 字**（0x300/64）；
  内置 `TypedStorageBuffer::new<SparkleSsbo>`（0x141617a10，固定缓冲 `BUFFER_LEN × 32`）。
- `push<SparkleSsbo>`：整块拷贝 4 qword（32 B），写槽 `chunk<<5|count`，置 active+dirty——与
  `sparse_instance_buffer.rs` 既有 `SparseInstanceBufferWriter::push` 完全同形。
- 字段布局 = 重建 HLSL `shaders/sparkles.raster.hlsl` 的 `SparkleInstanceData`
  （`StructuredBuffer<SparkleInstanceData> instanced_data : register(t14)`，32 B 与机器码吻合）：
  `position: Vec3 @0`、`scale: f32 @0xc`、`time_offset: f32 @0x10`、`speed_mult: f32 @0x14`、
  `color_id: i32 @0x18`、`pad1: f32 @0x1c`。
- 写面（火花粒子系统，critters_particles 域，库内未建）：不在本批范围，登记。

### 注册面

- `init_populate_ssbo_library`（0x14090a2e7）`add_new<SparkleSsbo>`（0x1412c3a50）。SsboId 经
  `get_rg_buffer<SparkleSsbo>`（0x1412d2b00）核对 = 0x2bf3b1a4ac675864。

## 实现（本批）

| 项 | 文件 | 内容 |
|---|---|---|
| `SparkleSsbo` 类型 | `src/resources/sparkle.rs`（新增） | 32 B 元素 + `StorageElement`（NAME="SparkleSsbo", BUFFER_LEN=0x6000）+ `unsafe SparseSsbo`（CHUNK_COUNT=0x300）+ `SsboType`（Storage=`SparseInstanceBuffer<Self>`） |
| 模块注册 | `src/resources/mod.rs` | `pub mod sparkle;` |
| Env36 现算 | `src/systems/render_loop/draw_passes/draw_callbacks_b.rs` | 删 `SparkleDrawRangesView` 视图；`env36` 改经 `AssetSsboLibrary::get::<SparkleSsbo>` → `to_draw_multi_instanced()` → `(u32,u32)` 映射 `[u32;2]` → `env36_body` |
| 验证探针 | `crates/country-core/tests/impl_render_ssbo.rs`（新增） | 公开 API 复验：元素合同 / 稀疏缓冲布局 / env36 ranges 组合 |

`env36_body`（`read_ssbo::<SparkleTag>` + `draw_multi_instanced`）绑定序保持不动（REBUILD-REMAINDER 已判等）。

## 判等结论

- Env36 数据流偏差**闭合**：ranges 由 `SparseInstanceBuffer<SparkleSsbo>::to_draw_multi_instanced` 每帧现算，
  与反汇编（as_any → to_draw → read_storage_buffer → draw_multi_instanced）逐调用对应。
- 绑定序不变（`read_ssbo<SparkleTag>` 槽 "SparkleSsbo"、rg hash 0x2bf3b1a4ac675864 = SsboId）。

## 验证

- `cargo check -p country-core --lib`：**通过**（10 个既有警告，无本批错误）。
- `cargo test -p country-core --test impl_render_ssbo`：**3/3 PASS**
  （sparkle_element_contract_matches_machine_code / sparse_buffer_layout_matches_the_machine_code /
  env36_ranges_come_from_the_sparkle_sparse_buffer_each_frame）。
- crate `--tests` 全量构建被并行 agent WIP 阻塞（碰撞 intersect_meshes / history decorator_edit /
  prefab 的 `#[cfg(test)]` 或非 test 代码错误，错误集合随并发构建变化，非本模块），故用集成测试
  （只编译 lib 非 test cfg + 本测试文件）复验。

## 登记

- **RoofTileSsbo 存储形态不一致**：原版 `SparseInstanceBuffer<RoofTileSsbo>`（0x1412e4a90 new_storage_buffer、
  `render/device.rs` 的 `get_mut<RoofTileSsbo>` → `gpu_flush<RoofTileSsbo>` 已登记），crate
  `compute_passes/roof_tiles_gravity.rs` 仍为 `ssbo_carrier!`（TypedStorageBuffer）。属 roof/device 域，
  本批不改（红线）。
- **SparkleSsbo 写面**（`push<SparkleSsbo>` 的生产者，critters_particles 火花粒子系统）库内未建。
- **ice_surface / nani**：核对无 `SparseSsbo`/`SparseInstanceBuffer` 依赖（render_world/surface_ice/nani/* 零命中），无需接线。

## 批次汇总

| 项 | 实现 | 登记 |
|---|---|---|
| SparkleSsbo 类型 + SparseSsbo 落地 | 1（resources/sparkle.rs） | 写面（粒子域） |
| Env36 ranges 现算 | 1（draw_callbacks_b.rs） | — |
| ice_surface/nani 接线核对 | 0（无依赖） | — |
| RoofTileSsbo 存储形态 | 0 | 1（roof/device 域） |
