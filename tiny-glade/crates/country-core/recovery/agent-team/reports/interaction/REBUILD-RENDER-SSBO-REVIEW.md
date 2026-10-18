# REVIEW — render-ssbo 收口(REBUILD-RENDER-SSBO 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: impl-render-ssbo REBUILD-RENDER-SSBO.md(src/resources/sparkle.rs 新增);
`cargo check --lib` exit 0(独立复验,9 警告均既有)、集成测试 3/3。

## 逐项复验

### ① SparkleSsbo 类型 — PASS
- 32 B 元素:{position Vec3@0, scale f32@0xc, time_offset@0x10, speed_mult@0x14, color_id
  i32@0x18, pad1 f32@0x1c}(= 重建 HLSL sparkles.raster.hlsl SparkleInstanceData)✓。
- `StorageElement`(NAME="SparkleSsbo", BUFFER_LEN=0x6000)+ `unsafe SparseSsbo`
  (CHUNK_COUNT=0x300,byte_map 0x301 哨兵[0x300]=0x20、影子 0xc0000B/0x6000 元素、free 栈降序、
  active/dirty 各 0xc 字)+ `SsboType::Storage=SparseInstanceBuffer<Self>` ✓。
- `push<SparkleSsbo>` 整块 4 qword(32B)与既有 SparseInstanceBufferWriter::push 同形 ✓。

### ② Env36 现算 — PASS
- env36(0x1409b7fa0)删 `SparkleDrawRangesView` 视图,改经 `AssetSsboLibrary::get::<SparkleSsbo>`
  → `to_draw_multi_instanced()` 现算 ranges → `[u32;2]` → `env36_body` ✓。
- `FrozenMap::get` 键 0x2bf3b1a4ac675864 = xxh3_64("SparkleSsbo")(read_memory 8B 小端)+
  名字断言 len 0x2d memcmp "country_core::resources::sparkle::SparkleSsbo" ✓。
- Env36 的 `<RoofTileSsbo>` 符号 = 与 `<SparkleSsbo>` 同形 ICF 合并(只读 byte_map/active,
  字段偏移跨 T 相同)——find_functions 证实 SparkleSsbo 单态化独立 ✓。
- `env36_body`(read_ssbo<SparkleTag> + draw_multi_instanced)绑定序保持不动 ✓。

### ③ 登记诚实 — PASS
- **RoofTileSsbo 存储形态不一致**:原版 `SparseInstanceBuffer<RoofTileSsbo>`(0x1412e4a90 +
  render/device.rs get_mut→gpu_flush 已登记),crate roof_tiles_gravity.rs 仍 `ssbo_carrier!`
  (TypedStorageBuffer)——属 roof/device 域,本批不改(红线)✓。
- **SparkleSsbo 写面**(push 的生产者,critters_particles 火花粒子域)库内未建,登记 ✓。
- **ice_surface / nani**:核对无 SparseSsbo/SparseInstanceBuffer 依赖(零命中),无需接线 ✓。

### ④ 编译/测试 — PASS
`cargo check --lib` exit 0(独立复验);集成测试 tests/impl_render_ssbo.rs 3/3
(元素合同/稀疏缓冲布局/env36 ranges 每帧现算)。

## 结论与 DISPATCH 建议

- render-ssbo 收口复验通过:**SparkleSsbo 类型 + SparseSsbo 落地 + Env36 现算闭合**,编译
  exit 0(独立复验)。
- 建议:render-ssbo → 闭合(实现层面);登记项(RoofTileSsbo 存储形态归 roof/device 域、
  SparkleSsbo 写面归 critters_particles 粒子域)由对应域接线;ice_surface/nani 确认无依赖。
