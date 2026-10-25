# crates/country-core/src/render

状态：记录有交付，本机缺失。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

pass_builder、SSBO/Shadowed/SparseBuffer、Mesh/Shader、nani GPU-world/atlas/实例与21 ECS系统有交付记录。

## 接线与验证

接线：not_established。

项目记忆报告offline check/clippy通过；本机源缺失，不计本机已复核实现。

## 已恢复的函数和类型范围

- ShaderUri::parse、ComputePass/DrawBuilder全族
- TypedStorageBuffer/ShadowedBuffer/SparseInstanceBuffer
- GpuWorld/MeshAtlas/实例更新及nani_flush_instance_updates

## 明确缺口

- 记录中的country-core/country-core-render交付在当前工作区缺失，先同步并复核
- pass_resource_access/draw_stream managed/temporal边界、component→T多数字段映射

下一闭包：找回原交付与合同，核对身份/源码hash，再接pass到rhapsody真实Command与managed资源

## 证据

- PROJECT_MEMORY.md
- [reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md](../../reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/render/adaptive_render_state.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/atmosphere.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/debug_info_buffers.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/directional_light.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/dirty_index_range.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/frame_constants.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/fsr_plugin.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/indirect.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/load_texture.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/mesh.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/buffers.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/capacity.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/cull_shaders.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/draw_ext.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/gpu_world.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/instance_data.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/maintenance.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/mesh_atlas.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/systems.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/trimesh.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/nani/uber_materials.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/bind_resources.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/compute_pass_builder.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/draw_builder.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/draw_list.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/draw_stream.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/framebuffer_binding.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/pass_resource_access.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/pass_resources.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/shader_uri.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/pass_builder/uniform_binder.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/raster_single.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/render_config.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/shader.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/shadowed_buffer.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/sparse_instance_buffer.rs` | A,B,C | [skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs](../../skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs)：explicit_candidate_delivery_mapping | country_core::render::sparse_instance_buffer::AllocationHandle::is_null（implemented_unverified；业务等价以限定证据为准）<br>country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push<country_core::resources::roof_tile_ssbo::RoofTileSsbo>（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/country-core/src/render/ssbo.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/texture_export.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/uniform.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/vao.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/render/view_constants.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/country-core/src/render/adaptive_render_state.rs

[reconstruction/country-core/src/render/adaptive_render_state.rs](../../reconstruction/country-core/src/render/adaptive_render_state.rs)

SHA256：`7ac15d0c072ca2bb6e39c40ce6f33250eee41715940ea9492738c973efcb82ee`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L6：//! internal-resolution scaling. Retained as a typed gap.
- L8：/// Adaptive render state (U — typed gap; inferred to hold current/max internal
- L11：_gap: crate::typed_gap::Gap,
### reconstruction/country-core/src/render/atmosphere.rs

[reconstruction/country-core/src/render/atmosphere.rs](../../reconstruction/country-core/src/render/atmosphere.rs)

SHA256：`5f610afd5a17cc9c84f598484daef6e306a6c17a81d0ccfb0f755e8c23627f5c`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `lerp` | 27 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：//! exact mapping is a typed gap.
### reconstruction/country-core/src/render/debug_info_buffers.rs

[reconstruction/country-core/src/render/debug_info_buffers.rs](../../reconstruction/country-core/src/render/debug_info_buffers.rs)

SHA256：`02f3a975c8a531bfed49112f395fa1e859a171adf4a90add341e18520add9eac`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L5：//! (no PDB procedures overlap this file). Retained as a typed gap with an empty
- L8：/// Debug buffer state (U — typed gap).
- L10：_gap: crate::typed_gap::Gap,
### reconstruction/country-core/src/render/directional_light.rs

[reconstruction/country-core/src/render/directional_light.rs](../../reconstruction/country-core/src/render/directional_light.rs)

SHA256：`df093be42eba731c4a123ca60f24b05a76b18f816d5af70d213d8cb78c75a224`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `look_at` | 31 | 未由扫描证明；以证据限定范围为准 |
| `set_direction_towards_light` | 52 | 未由扫描证明；以证据限定范围为准 |
| `translate` | 59 | 未由扫描证明；以证据限定范围为准 |
| `update` | 64 | 未由扫描证明；以证据限定范围为准 |
| `view_to_clip` | 69 | 未由扫描证明；以证据限定范围为准 |
| `default` | 75 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/dirty_index_range.rs

[reconstruction/country-core/src/render/dirty_index_range.rs](../../reconstruction/country-core/src/render/dirty_index_range.rs)

SHA256：`302190390c83e02b6bba0b2bb5ef2be9781edae0e7b5ee45bf3703e08423d530`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `empty` | 20 | 未由扫描证明；以证据限定范围为准 |
| `add` | 28 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 34 | 未由扫描证明；以证据限定范围为准 |
| `to_range` | 39 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 48 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 56 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/frame_constants.rs

[reconstruction/country-core/src/render/frame_constants.rs](../../reconstruction/country-core/src/render/frame_constants.rs)

SHA256：`63b47280aa4b875c05eb2a0f79d343c653695dc745728f6d0e5058d2d5295bbc`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_shadow_view` | 22 | 未由扫描证明；以证据限定范围为准 |
| `sun_cascade` | 27 | 未由扫描证明；以证据限定范围为准 |
| `main_view` | 57 | 未由扫描证明；以证据限定范围为准 |
| `sun_view` | 62 | 未由扫描证明；以证据限定范围为准 |
| `should_reset_temporal` | 67 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L40：/// direction, time, frame index, …) is a typed gap informed by shader
- L52：_gap: crate::typed_gap::Gap,
### reconstruction/country-core/src/render/fsr_plugin.rs

[reconstruction/country-core/src/render/fsr_plugin.rs](../../reconstruction/country-core/src/render/fsr_plugin.rs)

SHA256：`22898209479e611dae1b98802f70cdb1245d0e3f46199aab0f60440741e2a6c6`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 20 | 未由扫描证明；以证据限定范围为准 |
| `default` | 26 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：//! `_fsr_init_reactive`). This module is retained as a typed gap with the
- L11：/// FSR reactive/history pass state (U — typed gap).
- L13：_gap: crate::typed_gap::Gap,
### reconstruction/country-core/src/render/indirect.rs

[reconstruction/country-core/src/render/indirect.rs](../../reconstruction/country-core/src/render/indirect.rs)

SHA256：`9c830e54c941076716acaa411a52d44fd61d07aed6ea805445610a380489c88f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `empty` | 17 | 未由扫描证明；以证据限定范围为准 |
| `w_vertex_count` | 21 | 未由扫描证明；以证据限定范围为准 |
| `empty` | 39 | 未由扫描证明；以证据限定范围为准 |
| `w_instance_count` | 43 | 未由扫描证明；以证据限定范围为准 |
| `empty` | 59 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/load_texture.rs

[reconstruction/country-core/src/render/load_texture.rs](../../reconstruction/country-core/src/render/load_texture.rs)

SHA256：`eac0de78a854812d2ce5ef6c0266bf875faa4e110862b2906815c39817ea9652`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `load_compiled_texture_image` | 26 | 未由扫描证明；以证据限定范围为准 |
| `compiled_image_path` | 74 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/mesh.rs

[reconstruction/country-core/src/render/mesh.rs](../../reconstruction/country-core/src/render/mesh.rs)

SHA256：`53615f13a0756cf60570aeda9531768cad845d68cc34f32c01335c0bb6bb31d6`；缺口注释标记数：7。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `array_length` | 44 | 未由扫描证明；以证据限定范围为准 |
| `stride` | 56 | 未由扫描证明；以证据限定范围为准 |
| `size_in_bytes` | 68 | 未由扫描证明；以证据限定范围为准 |
| `component_count` | 73 | 未由扫描证明；以证据限定范围为准 |
| `append` | 84 | 未由扫描证明；以证据限定范围为准 |
| `retain_by_index` | 111 | 未由扫描证明；以证据限定范围为准 |
| `as_f32` | 158 | 未由扫描证明；以证据限定范围为准 |
| `as_f32x2` | 164 | 未由扫描证明；以证据限定范围为准 |
| `as_f32x3` | 170 | 未由扫描证明；以证据限定范围为准 |
| `as_f32x4` | 176 | 未由扫描证明；以证据限定范围为准 |
| `as_i32` | 182 | 未由扫描证明；以证据限定范围为准 |
| `from` | 194 | 未由扫描证明；以证据限定范围为准 |
| `new` | 226 | 未由扫描证明；以证据限定范围为准 |
| `set_source_file` | 231 | 未由扫描证明；以证据限定范围为准 |
| `is_valid` | 236 | 未由扫描证明；以证据限定范围为准 |
| `has_attribute` | 241 | 未由扫描证明；以证据限定范围为准 |
| `set_indices` | 246 | 未由扫描证明；以证据限定范围为准 |
| `arrayed_indices` | 251 | 未由扫描证明；以证据限定范围为准 |
| `vertex_count` | 259 | 未由扫描证明；以证据限定范围为准 |
| `positions` | 265 | 未由扫描证明；以证据限定范围为准 |
| `positions_without_indices` | 277 | 未由扫描证明；以证据限定范围为准 |
| `normals_without_indices` | 282 | 未由扫描证明；以证据限定范围为准 |
| `uvs_without_indices` | 290 | 未由扫描证明；以证据限定范围为准 |
| `set_attribute` | 301 | 未由扫描证明；以证据限定范围为准 |
| `add_color` | 314 | 未由扫描证明；以证据限定范围为准 |
| `add_color_self` | 321 | 未由扫描证明；以证据限定范围为准 |
| `aabb_bounds` | 332 | 未由扫描证明；以证据限定范围为准 |
| `collision_bbx` | 343 | 未由扫描证明；以证据限定范围为准 |
| `collision_radius_xz` | 348 | 未由扫描证明；以证据限定范围为准 |
| `as_triangles` | 359 | 未由扫描证明；以证据限定范围为准 |
| `retain_vertices` | 379 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 413 | 未由扫描证明；以证据限定范围为准 |
| `translate_hack_slow_dont_use` | 419 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：//! +0x58..+0x70 are typed gaps.
- L28：/// 4 = `Vec<[f32;4]>`, i32 = `Vec<i32>` (I; exact order beyond 3 is a typed gap).
- L36：/// Typed gap: additional variants exist in the original (O: `append`
- L153：// U: unknown variant payload semantics.
- L213：/// +0x18/+0x20 in the original layout. Semantics U (typed gap).
- L218：/// +0x58..+0x70 in the original layout. Semantics U (typed gap).
- L232：// O: trivial store into the +0x68/+0x70 region (typed gap).
### reconstruction/country-core/src/render/mesh_atlas.rs

[reconstruction/country-core/src/render/mesh_atlas.rs](../../reconstruction/country-core/src/render/mesh_atlas.rs)

SHA256：`e8999be411e5b59c4a219a1451ae533babce7f64f47a2c400c1d8589c7d20be8`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L6：//! a typed gap (in the original it is a ustr-style interned name).
### reconstruction/country-core/src/render/mod.rs

[reconstruction/country-core/src/render/mod.rs](../../reconstruction/country-core/src/render/mod.rs)

SHA256：`46ce8b34add5583a84df244972bca5e325ca67b7ac469f8c1ada778597769a56`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `deref` | 58 | 未由扫描证明；以证据限定范围为准 |
| `from_preference_order` | 79 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L15：//! Marking legend: O = observed (machine code / PDB), I = inferred, U = unknown (typed gap).
- L66：/// swaps 0xa0-byte elements). Exact field layout is a typed gap.
- L68：/// O: opaque 160-byte record; fields unknown.
### reconstruction/country-core/src/render/nani/buffers.rs

[reconstruction/country-core/src/render/nani/buffers.rs](../../reconstruction/country-core/src/render/nani/buffers.rs)

SHA256：`5cffeeb5e01a920fa9772aabfe8093d5558ae1db6cf04b4c9249b37748df35e5`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `nani_calculate_draw_list_params` | 38 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L39：// per-kind 常量表 DAT_142aa14c0 未读内存（typed gap）。
- L40：todo!("recover per-kind table scan; see contracts/buffers.md §2")
### reconstruction/country-core/src/render/nani/cull_shaders.rs

[reconstruction/country-core/src/render/nani/cull_shaders.rs](../../reconstruction/country-core/src/render/nani/cull_shaders.rs)

SHA256：`be497d2ce328dbbd77ed77d3b1678f61279c552ec800fe01b0ddd0929399a639`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `cull_and_bucket_for_kind` | 44 | 未由扫描证明；以证据限定范围为准 |
| `group_draw_cmd_prefix_sum` | 49 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L45：todo!("recover cull_and_bucket closure; see contracts/cull_shaders.md §2")
- L50：todo!()
### reconstruction/country-core/src/render/nani/draw_ext.rs

[reconstruction/country-core/src/render/nani/draw_ext.rs](../../reconstruction/country-core/src/render/nani/draw_ext.rs)

SHA256：`32b5250efb1557d21a4c66b433744e0a68213554dc3494b10c36661fd1dbbf29`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `nani_draw` | 21 | 未由扫描证明；以证据限定范围为准 |
| `nani_draw` | 25 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L26：todo!("recover nani_draw; see contracts/draw_ext.md §1")
### reconstruction/country-core/src/render/nani/gpu_world.rs

[reconstruction/country-core/src/render/nani/gpu_world.rs](../../reconstruction/country-core/src/render/nani/gpu_world.rs)

SHA256：`5081583a05c7882ea9438d4469828899fc2f0e15b75037d7f34dce78c8a1f36a`；缺口注释标记数：31。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `eq` | 61 | 未由扫描证明；以证据限定范围为准 |
| `hash` | 67 | 未由扫描证明；以证据限定范围为准 |
| `shader` | 110 | 未由扫描证明；以证据限定范围为准 |
| `try_from_shader` | 126 | 未由扫描证明；以证据限定范围为准 |
| `max_instances` | 132 | 未由扫描证明；以证据限定范围为准 |
| `max_draws_per_list` | 137 | 未由扫描证明；以证据限定范围为准 |
| `use_fine_grained_sorting` | 142 | 未由扫描证明；以证据限定范围为准 |
| `draw_list_index` | 150 | 未由扫描证明；以证据限定范围为准 |
| `all` | 156 | 未由扫描证明；以证据限定范围为准 |
| `gpu_world_subset_desc` | 161 | 未由扫描证明；以证据限定范围为准 |
| `new` | 205 | 未由扫描证明；以证据限定范围为准 |
| `instance_count` | 210 | 未由扫描证明；以证据限定范围为准 |
| `extend_no_data` | 217 | 未由扫描证明；以证据限定范围为准 |
| `update_instance_data` | 234 | 未由扫描证明；以证据限定范围为准 |
| `remove_instance` | 239 | 未由扫描证明；以证据限定范围为准 |
| `gpu_flush` | 244 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 249 | 未由扫描证明；以证据限定范围为准 |
| `new` | 260 | 未由扫描证明；以证据限定范围为准 |
| `new` | 287 | 未由扫描证明；以证据限定范围为准 |
| `owned_subset_mut` | 310 | 未由扫描证明；以证据限定范围为准 |
| `get_subset` | 315 | 未由扫描证明；以证据限定范围为准 |
| `try_get_subset_mut` | 320 | 未由扫描证明；以证据限定范围为准 |
| `owned_subset_exclusive_mut` | 325 | 未由扫描证明；以证据限定范围为准 |
| `assign_subset` | 330 | 未由扫描证明；以证据限定范围为准 |
| `create_instances` | 337 | 未由扫描证明；以证据限定范围为准 |
| `update_instance_mesh` | 349 | 未由扫描证明；以证据限定范围为准 |
| `remove_instance` | 354 | 未由扫描证明；以证据限定范围为准 |
| `import_to_rg` | 359 | 未由扫描证明；以证据限定范围为准 |
| `export_from_rg` | 364 | 未由扫描证明；以证据限定范围为准 |
| `list_index_to_cmd_offset` | 369 | 未由扫描证明；以证据限定范围为准 |
| `mesh_atlas_info_buf` | 374 | 未由扫描证明；以证据限定范围为准 |
| `draw_list_info_buf` | 379 | 未由扫描证明；以证据限定范围为准 |
| `type_name` | 395 | 未由扫描证明；以证据限定范围为准 |
| `as_any` | 396 | 未由扫描证明；以证据限定范围为准 |
| `as_any_mut` | 397 | 未由扫描证明；以证据限定范围为准 |
| `gpu_flush` | 398 | 未由扫描证明；以证据限定范围为准 |
| `rg_handle` | 399 | 未由扫描证明；以证据限定范围为准 |
| `import_to_rg` | 400 | 未由扫描证明；以证据限定范围为准 |
| `export_from_rg` | 401 | 未由扫描证明；以证据限定范围为准 |
| `copy` | 402 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 406 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L74：/// 前 9 个变体名与 cull_and_bucket shader 家族对应；其余 18 个为 typed gap。
- L87：// 18 个未命名变体（typed gap）：变体 9..=26 语义 U（PDB 仅给方法证据，无变体名）。
- L121：_ => "unknown", // typed gap
- L128：todo!("从 shader 名匹配 27 变体；未恢复")
- L133：todo!("读 per-kind 常量表；见 contracts/buffers.md §5")
- L138：todo!("读 per-kind 常量表")
- L163：todo!("owned/external/owned_custom 描述；未恢复具体布局")
- L167：/// subset 描述（尺寸/布局；typed gap——字段 U）。
- L206：todo!("construct OwnedGpuWorldSubset")
- L227：todo!("recover extend_no_data body; see contracts/gpu_world.md §2")
- L235：todo!("recover update_instance_data<T>; element size = size_of::<T>()")
- L240：todo!("recover remove_instance; see contracts/gpu_world.md §4")
- L245：todo!()
- L250：todo!()
- L256：// typed gap：外部缓冲句柄/引用字段 U。
- L261：todo!()
- L273：/// 第二个计数/保留字段（+0x1448；typed gap）。
- L297：mesh_atlas_info: todo!(
- L300：draw_list_info: todo!(
- L311：todo!("lock subsets[kind]; unwrap_failed if None")
- L316：todo!()
- L321：todo!()
- L326：todo!()
- L331：todo!()
- L345：todo!("recover create_instances; see contracts/gpu_world.md §2")
- L350：todo!("recover update_instance_mesh; called from nani_maintain_instances")
- L355：todo!("recover remove_instance")
- L360：todo!("for each subset slot: subset.import_to_rg(rg)")
- L365：todo!()
- L370：todo!()
### reconstruction/country-core/src/render/nani/instance_data.rs

[reconstruction/country-core/src/render/nani/instance_data.rs](../../reconstruction/country-core/src/render/nani/instance_data.rs)

SHA256：`cea50baeaa7799700c9e9f9a76c8f731c19ff13458b3de3ac35bceefb597a304`；缺口注释标记数：11。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `marker` | 123 | 未由扫描证明；以证据限定范围为准 |
| `new_curved_xy` | 197 | 未由扫描证明；以证据限定范围为准 |
| `new_curved_xz` | 225 | 未由扫描证明；以证据限定范围为准 |
| `new_straight` | 238 | 未由扫描证明；以证据限定范围为准 |
| `curved_halftimber_mesh_atlas_part_idx` | 263 | 未由扫描证明；以证据限定范围为准 |
| `type_name` | 292 | 未由扫描证明；以证据限定范围为准 |
| `as_any` | 293 | 未由扫描证明；以证据限定范围为准 |
| `as_any_mut` | 294 | 未由扫描证明；以证据限定范围为准 |
| `gpu_flush` | 295 | 未由扫描证明；以证据限定范围为准 |
| `rg_handle` | 296 | 未由扫描证明；以证据限定范围为准 |
| `import_to_rg` | 297 | 未由扫描证明；以证据限定范围为准 |
| `export_from_rg` | 298 | 未由扫描证明；以证据限定范围为准 |
| `copy` | 299 | 未由扫描证明；以证据限定范围为准 |
| `register_required_components` | 328 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：//! per-class GPU 实例布局（见 evidence/render-passes/INDEX.md §3）。字段名/顺序多为 typed gap；
- L26：/// typed gap：{transform 行/平移, 缩放, 色彩, 位域, ...} 字段 U（对齐 0x40 布局）。
- L146：/// typed gap：{prev/current transform, ...} 字段 U。
- L184：/// +0x24..+0x2b：打包字段（typed gap；new_straight 写 `0x3dc28f5c00000001`）。
- L233：todo!("recover new_curved_xz (363B)")
- L264：todo!()
- L296：fn rg_handle(&self) -> u64 { todo!() }
- L297：fn import_to_rg(&self, _rg: &mut crate::rhapsody_contract::RenderGraph) { todo!() }
- L298：fn export_from_rg(&self, _rg: &mut crate::rhapsody_contract::RenderGraph) { todo!() }
- L299：fn copy(&self, _src: u32, _dst: u32) { todo!("element size = size_of::<Self>(); CAS min/max") }
- L329：todo!("bevy component registration; see contracts/instance_data.md §3")
### reconstruction/country-core/src/render/nani/mesh_atlas.rs

[reconstruction/country-core/src/render/nani/mesh_atlas.rs](../../reconstruction/country-core/src/render/nani/mesh_atlas.rs)

SHA256：`172eebc165fe5eb8d7f6ae7ddfb78c5ee665c943e59fa2118a3475b03938a836`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `finish` | 20 | 未由扫描证明；以证据限定范围为准 |
| `write` | 23 | 未由扫描证明；以证据限定范围为准 |
| `write_u64` | 26 | 未由扫描证明；以证据限定范围为准 |
| `new` | 47 | 未由扫描证明；以证据限定范围为准 |
| `add` | 78 | 未由扫描证明；以证据限定范围为准 |
| `compute_radius` | 84 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L80：todo!("recover MeshAtlas::add; see contracts/mesh_atlas.md §2")
- L96：/// typed gap：{base_vertex, vertex_count, index_offset, ...} 字段 U。
### reconstruction/country-core/src/render/nani/mod.rs

[reconstruction/country-core/src/render/nani/mod.rs](../../reconstruction/country-core/src/render/nani/mod.rs)

SHA256：`10d4a280eef0eef605fc95849ed42e94299268808c48f23a852551fa2a4d7dec`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L39：use crate::typed_gap::Gap;
### reconstruction/country-core/src/render/nani/systems.rs

[reconstruction/country-core/src/render/nani/systems.rs](../../reconstruction/country-core/src/render/nani/systems.rs)

SHA256：`d1547d00849a948705eb59c5b5003e0bd57fa08ec10788333dc03e9412fc8a3b`；缺口注释标记数：26。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `kind_from_u8` | 35 | 未由扫描证明；以证据限定范围为准 |
| `nani_maintain_instances` | 78 | 未由扫描证明；以证据限定范围为准 |
| `instance_key_from_desc` | 143 | 未由扫描证明；以证据限定范围为准 |
| `instance_key_from_handle` | 153 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_vertex_color` | 171 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_vertex_color_window` | 197 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_albedo_map` | 222 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_albedo_map_emissive` | 246 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_preplaced_flower` | 279 | 未由扫描证明；以证据限定范围为准 |
| `preplaced_flower_instance_data` | 301 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_plant` | 313 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_vertex_color_window_glass` | 331 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_solid_vertex_color_bricks` | 361 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_gothic_window_bricks` | 385 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_curved_halftimber` | 414 | 未由扫描证明；以证据限定范围为准 |
| `update_curved_halftimber_caps` | 445 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_stone_floor` | 456 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_wren` | 482 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_lantern_emissive` | 507 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_plaster` | 535 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_cloth` | 567 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_smoke` | 594 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_balcony_door` | 611 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_uber` | 636 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_velocity_only` | 663 | 未由扫描证明；以证据限定范围为准 |
| `nani_update_stairs_floor` | 689 | 未由扫描证明；以证据限定范围为准 |
| `nani_flush_instance_updates` | 733 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：//! 业务逻辑标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
- L39：todo!("invalid subset kind u8 {v}")
- L53：/// typed gap：每 kind 的变更 tick 切片/迭代缓存缓冲 U。
- L149：todo!("recover InstanceKey derivation (chunk_id + instance_index + f16 sort_key); see contracts/systems.md §2")
- L154：todo!("recover remove-instance key construction")
- L190：let data: SolidVertexColorInstanceData = todo!("component → 0x40 instance data");
- L215：let data: SolidVertexColorWindowInstanceData = todo!();
- L239：let data: SolidAlbedoMapInstanceData = todo!();
- L263：let data: SolidAlbedoMapEmissiveInstanceData = todo!();
- L309：todo!("recover flower bitmap + water-height gating; see contracts/systems.md §3")
- L324：let data: PlantInstanceData = todo!("transform → PlantInstanceData (0x30)");
- L354：let data: SolidVertexColorWindowGlassInstanceData = todo!();
- L378：let data: SolidVertexColorBricksInstanceData = todo!();
- L403：let data: GothicWindowBricksInstanceData = todo!();
- L446：todo!("recover cap-mask update; see contracts/systems.md §3")
- L475：let data: StoneFloorInstanceData = todo!("roof tangent + animation → stone floor");
- L500：let data: WrenInstanceData = todo!("wren pose (prev/current + critter params)");
- L526：let data: LanternEmissiveInstanceData = todo!();
- L560：let data: PlasterInstanceData = todo!("plaster peel animation → 0x30");
- L587：let data: ClothInstanceData = todo!();
- L604：let data: SmokeInstanceData = todo!();
- L629：let data: BalconyDoorInstanceData = todo!();
- L656：let data: UberInstanceData = todo!();
- L680：let data: VelocityOnlyInstanceData = todo!("prev/current transform → 0x60");
- L711：let data: StairsFloorInstanceData = todo!("mesh chunk → 0x30");
- L737：todo!("recover per-kind ShadowedBuffer<GpuInstance> gpu_flush + counter swap; see contracts/systems.md §5")
### reconstruction/country-core/src/render/nani/trimesh.rs

[reconstruction/country-core/src/render/nani/trimesh.rs](../../reconstruction/country-core/src/render/nani/trimesh.rs)

SHA256：`3ebec1bd25340164cc2492cf9960f4e2578cf753862d1c4c005743f0d611ed34`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `positions` | 41 | 未由扫描证明；以证据限定范围为准 |
| `attrib` | 51 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/pass_builder/bind_resources.rs

[reconstruction/country-core/src/render/pass_builder/bind_resources.rs](../../reconstruction/country-core/src/render/pass_builder/bind_resources.rs)

SHA256：`dfac3cc9f23fd47166facd65bb261aa16309129cf904f5415881059547333408`；缺口注释标记数：9。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `bind_color_library` | 26 | 未由扫描证明；以证据限定范围为准 |
| `bind_ray_tracing` | 33 | 未由扫描证明；以证据限定范围为准 |
| `bind_shadowmap` | 40 | 未由扫描证明；以证据限定范围为准 |
| `resource_slot` | 44 | 未由扫描证明；以证据限定范围为准 |
| `push_access_range` | 46 | 未由扫描证明；以证据限定范围为准 |
| `read_wall_color_ids` | 48 | 未由扫描证明；以证据限定范围为准 |
| `as_pass_builder` | 50 | 未由扫描证明；以证据限定范围为准 |
| `bindings_placeholder` | 54 | 未由扫描证明；以证据限定范围为准 |
| `bind_color_library_impl` | 60 | 未由扫描证明；以证据限定范围为准 |
| `bind_ray_tracing_impl` | 87 | 未由扫描证明；以证据限定范围为准 |
| `resolve_resource_hash` | 119 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 140 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 145 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 150 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 155 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 160 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：//! 光线追踪 / 墙色 SSBO 载体保持为 typed-gap 占位并实现 `SsboType`（WP1 不重定义 SSBO 结构体；
- L67：//   PassBuilder 落地（as_pass_builder 为 todo 边界）；typed-gap 形态。
- L99：//   双 `&mut pass` 借用待 rhapsody PassBuilder 落地（typed-gap 形态）。
- L118：/// typed gap：经 *(pass+0xe8) 注册表解析；接入 rhapsody 时落实。
- L125：// SSBO 载体为 typed-gap 占位（WP1 不重定义游戏 SSBO 结构体）。hash 为观测注册表键。
- L130：/// 光线追踪三角形缓冲（typed-gap 载体；hash O）。
- L132：/// 光线追踪三角形元数据缓冲（typed-gap 载体；hash O）。
- L134：/// 软件 BVH 节点缓冲（typed-gap 载体；hash O）。
- L136：/// 墙色 ID 回读缓冲（typed-gap 载体）。
### reconstruction/country-core/src/render/pass_builder/compute_pass_builder.rs

[reconstruction/country-core/src/render/pass_builder/compute_pass_builder.rs](../../reconstruction/country-core/src/render/pass_builder/compute_pass_builder.rs)

SHA256：`aa40a51fa1230599193ad82e606e75f9b2330412c3fa09c2e4d31e274d697bfd`；缺口注释标记数：10。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get` | 70 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_1d` | 108 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_2d` | 109 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_3d` | 110 | 未由扫描证明；以证据限定范围为准 |
| `push_constants` | 111 | 未由扫描证明；以证据限定范围为准 |
| `track_gpu_lifetime` | 112 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_indirect` | 117 | 未由扫描证明；以证据限定范围为准 |
| `push_constants` | 118 | 未由扫描证明；以证据限定范围为准 |
| `new` | 127 | 未由扫描证明；以证据限定范围为准 |
| `empty` | 140 | 未由扫描证明；以证据限定范围为准 |
| `read_rg_storage_buffer` | 154 | 未由扫描证明；以证据限定范围为准 |
| `write_rg_storage_buffer` | 160 | 未由扫描证明；以证据限定范围为准 |
| `write_rg_storage_buffer_concurrent` | 166 | 未由扫描证明；以证据限定范围为准 |
| `declare_rg_access` | 172 | 未由扫描证明；以证据限定范围为准 |
| `bind_external_storage_buffer` | 194 | 未由扫描证明；以证据限定范围为准 |
| `bind_external_raw` | 207 | 未由扫描证明；以证据限定范围为准 |
| `write_to_cpu_mapped_storage_buffer` | 225 | 未由扫描证明；以证据限定范围为准 |
| `write_storage_buffer` | 231 | 未由扫描证明；以证据限定范围为准 |
| `write_storage_buffer_concurrent` | 237 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_1d` | 245 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_2d` | 255 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_groups` | 265 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_indirect_raw` | 275 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_with` | 281 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_indirect_with` | 286 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_impl` | 290 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_impl_mode` | 295 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_impl_with` | 301 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_3d_immediate` | 308 | 未由扫描证明；以证据限定范围为准 |
| `read_storage_buffer` | 326 | 未由扫描证明；以证据限定范围为准 |
| `read_texture` | 334 | 未由扫描证明；以证据限定范围为准 |
| `write_texture` | 340 | 未由扫描证明；以证据限定范围为准 |
| `read_write_texture` | 346 | 未由扫描证明；以证据限定范围为准 |
| `bind_texture` | 351 | 未由扫描证明；以证据限定范围为准 |
| `push_constants` | 357 | 未由扫描证明；以证据限定范围为准 |
| `as_pass_builder` | 364 | 未由扫描证明；以证据限定范围为准 |
| `resource_slot` | 380 | 未由扫描证明；以证据限定范围为准 |
| `push_access_range` | 387 | 未由扫描证明；以证据限定范围为准 |
| `read_wall_color_ids` | 390 | 未由扫描证明；以证据限定范围为准 |
| `as_pass_builder` | 391 | 未由扫描证明；以证据限定范围为准 |
| `bindings_placeholder` | 396 | 未由扫描证明；以证据限定范围为准 |
| `bind_texture_to_compute_pass` | 407 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L39：/// 字段布局按机器码观测（contracts/compute_pass_builder.md §1）；未恢复区间为 typed gap。
- L51：// typed gap：+0x48..+0x130 字段（shader 程序句柄、UniformBinder、PassResources 引用、
- L56：/// 资源槽表载体（typed gap：真实 HashMap<&str, PassResourceSlot> 位置未恢复）。
- L223：/// 的双 `&mut self` 借用待 rhapsody PassBuilder 落地（as_pass_builder 为 todo 边界）；
- L224：/// 此处保留访问登记语义的 typed-gap 形态。
- L328：// 双 `&mut self` 借用待 rhapsody PassBuilder 落地（typed-gap 形态）。
- L353：// typed gap：纹理注册表定位。
- L367：todo!("wired through rhapsody PassBuilder in integration")
- L384：// typed gap：ComputePass 资源槽表载体；接入时落到实际 HashMap。
- L394：todo!("wired through rhapsody PassBuilder in integration")
### reconstruction/country-core/src/render/pass_builder/draw_builder.rs

[reconstruction/country-core/src/render/pass_builder/draw_builder.rs](../../reconstruction/country-core/src/render/pass_builder/draw_builder.rs)

SHA256：`ded92e9d98a726eddd390b0a2996da007f05eff45fbb6b38be875a6088018455`；缺口注释标记数：14。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `uniform_offset` | 82 | 未由扫描证明；以证据限定范围为准 |
| `take` | 92 | 未由扫描证明；以证据限定范围为准 |
| `try_take` | 95 | 未由扫描证明；以证据限定范围为准 |
| `new` | 103 | 未由扫描证明；以证据限定范围为准 |
| `read_rg_storage_buffer` | 129 | 未由扫描证明；以证据限定范围为准 |
| `write_rg_storage_buffer` | 135 | 未由扫描证明；以证据限定范围为准 |
| `declare_rg_access` | 140 | 未由扫描证明；以证据限定范围为准 |
| `bind_external_storage_buffer` | 157 | 未由扫描证明；以证据限定范围为准 |
| `draw` | 162 | 未由扫描证明；以证据限定范围为准 |
| `draw_no_vao` | 168 | 未由扫描证明；以证据限定范围为准 |
| `draw_no_vao_indexed` | 177 | 未由扫描证明；以证据限定范围为准 |
| `draw_no_index_buffer` | 185 | 未由扫描证明；以证据限定范围为准 |
| `draw_multi_instanced` | 193 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_from_placement_data` | 204 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced_w_gl_draw_flag` | 210 | 未由扫描证明；以证据限定范围为准 |
| `multi_draw_indirect` | 218 | 未由扫描证明；以证据限定范围为准 |
| `multi_draw_indexed_indirect` | 229 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect_raw` | 240 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect_count` | 248 | 未由扫描证明；以证据限定范围为准 |
| `discard` | 262 | 未由扫描证明；以证据限定范围为准 |
| `push_snapshot` | 268 | 未由扫描证明；以证据限定范围为准 |
| `optional_uniform` | 284 | 未由扫描证明；以证据限定范围为准 |
| `uniform` | 295 | 未由扫描证明；以证据限定范围为准 |
| `read_storage_buffer` | 303 | 未由扫描证明；以证据限定范围为准 |
| `read_texture` | 313 | 未由扫描证明；以证据限定范围为准 |
| `read_texture_view` | 316 | 未由扫描证明；以证据限定范围为准 |
| `push_constants` | 319 | 未由扫描证明；以证据限定范围为准 |
| `stencil_ref` | 322 | 未由扫描证明；以证据限定范围为准 |
| `draw_instanced` | 325 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect` | 328 | 未由扫描证明；以证据限定范围为准 |
| `draw_indirect` | 331 | 未由扫描证明；以证据限定范围为准 |
| `bind_dummy_shadowmap` | 335 | 未由扫描证明；以证据限定范围为准 |
| `get_indirect_args_buffer` | 339 | 未由扫描证明；以证据限定范围为准 |
| `as_pass_builder` | 344 | 未由扫描证明；以证据限定范围为准 |
| `draw_commands_placeholder` | 349 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 356 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 362 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 379 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 388 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 397 | 未由扫描证明；以证据限定范围为准 |
| `write_at` | 406 | 未由扫描证明；以证据限定范围为准 |
| `rg_buffer_hash` | 417 | 未由扫描证明；以证据限定范围为准 |
| `resource_slot` | 426 | 未由扫描证明；以证据限定范围为准 |
| `push_access_range` | 433 | 未由扫描证明；以证据限定范围为准 |
| `read_wall_color_ids` | 436 | 未由扫描证明；以证据限定范围为准 |
| `as_pass_builder` | 439 | 未由扫描证明；以证据限定范围为准 |
| `bindings_placeholder` | 444 | 未由扫描证明；以证据限定范围为准 |
| `resource_slots_entry` | 451 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L53：/// 资源槽表载体（typed gap：真实 HashMap<&str, PassResourceSlot> 位置未恢复）。
- L55：// typed gap：+0x60..+0xb0 之间字段（indirect 参数、shader 句柄 Arc 等）。
- L75：pub uniform_bindings: u64, // +0x118/+0x120 表基址（typed gap：具体表类型）
- L80：/// 按 name_hash 查 uniform 绑定表偏移（+0x118/+0x120 表，typed gap）。
- L93：todo!("NoDropOnlyTake::take 行 11 宿主")
- L96：todo!("NoDropOnlyTake::try_take 行 1 宿主")
- L171：// 直接 draw（无间接）；mode 判别值未观测（typed gap），保持 0。
- L273：// 序列化 mode/draw_args/variant/state 到 payload（0xb0B）——typed gap：
- L305：// — the PassBuilder borrow is a todo!() integration point; the read access is
- L340：0 // typed gap
- L345：todo!("wired through rhapsody PassBuilder in integration")
- L430：// 真实表为 HashMap<&str, PassResourceSlot>（RandomState）；entry 接入为 typed gap。
- L442：todo!("wired through rhapsody PassBuilder in integration")
- L450：/// 资源槽表 entry 载体（typed gap：真实 HashMap entry 接入）。
### reconstruction/country-core/src/render/pass_builder/draw_list.rs

[reconstruction/country-core/src/render/pass_builder/draw_list.rs](../../reconstruction/country-core/src/render/pass_builder/draw_list.rs)

SHA256：`0f753346836b0fd6a601d7a4a572eb3b019fe34168e8af0aa6b0dd4d0211a0fd`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `to_draw_packet` | 28 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L13：/// 排序键/字段布局 U（typed gap）。
### reconstruction/country-core/src/render/pass_builder/draw_stream.rs

[reconstruction/country-core/src/render/pass_builder/draw_stream.rs](../../reconstruction/country-core/src/render/pass_builder/draw_stream.rs)

SHA256：`2518274b67d48a0321821bf9820cb6c92652bce84bbe7bc97c497e0bd299bf02`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 43 | 未由扫描证明；以证据限定范围为准 |
| `variant` | 52 | 未由扫描证明；以证据限定范围为准 |
| `stencil_ref` | 57 | 未由扫描证明；以证据限定范围为准 |
| `default` | 61 | 未由扫描证明；以证据限定范围为准 |
| `with_view` | 75 | 未由扫描证明；以证据限定范围为准 |
| `to_texture_binding_key` | 86 | 未由扫描证明；以证据限定范围为准 |
| `to_texture_binding_key` | 90 | 未由扫描证明；以证据限定范围为准 |
| `to_texture_binding_key` | 99 | 未由扫描证明；以证据限定范围为准 |
| `to_texture_binding_key` | 108 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 121 | 未由扫描证明；以证据限定范围为准 |
| `new` | 148 | 未由扫描证明；以证据限定范围为准 |
| `get_render_pass` | 164 | 未由扫描证明；以证据限定范围为准 |
| `render` | 176 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L22：/// Rust 枚举名未恢复 —— typed gap。
- L53：0 // typed gap
- L140：// typed gap：+0x3e8 内其余字段（shader 句柄、纹理视图描述等）。
- L169：todo!("render pass dedupe via rustc_entry (wired in integration)")
### reconstruction/country-core/src/render/pass_builder/framebuffer_binding.rs

[reconstruction/country-core/src/render/pass_builder/framebuffer_binding.rs](../../reconstruction/country-core/src/render/pass_builder/framebuffer_binding.rs)

SHA256：`f2fd857fd5b9f74647cc952c06404ea7b00b0b985effd48cf5bc3b4de64caa9e`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `desc` | 21 | 未由扫描证明；以证据限定范围为准 |
| `handle_mut` | 26 | 未由扫描证明；以证据限定范围为准 |
| `layer` | 40 | 未由扫描证明；以证据限定范围为准 |
| `new_color` | 55 | 未由扫描证明；以证据限定范围为准 |
| `new_depth` | 65 | 未由扫描证明；以证据限定范围为准 |
| `new_color_depth` | 73 | 未由扫描证明；以证据限定范围为准 |
| `texture` | 85 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L4：//! - `ColorAttachment::desc` 0x1409092f0（行 90；反编译空——真实逻辑被内联，typed gap）。
### reconstruction/country-core/src/render/pass_builder/mod.rs

[reconstruction/country-core/src/render/pass_builder/mod.rs](../../reconstruction/country-core/src/render/pass_builder/mod.rs)

SHA256：`bae555ba97a1a87a82d1752c00e5f84f736d6e118886e95d31a59de0c9980ac6`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|

缺口注释摘录（最多30条）：

- L31：/// 绘制/dispatch 模式判别（原机器码观测值，Rust 枚举名未恢复 —— typed gap）。
### reconstruction/country-core/src/render/pass_builder/pass_resource_access.rs

[reconstruction/country-core/src/render/pass_builder/pass_resource_access.rs](../../reconstruction/country-core/src/render/pass_builder/pass_resource_access.rs)

SHA256：`6e8f357ac582a1fdcb8fd3ef43338601a751a0ca6b1468ad037171ef21ee6b13`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `rg_buffer_hash` | 32 | 未由扫描证明；以证据限定范围为准 |
| `append_resource_binding` | 39 | 未由扫描证明；以证据限定范围为准 |
| `read_access_buffer` | 68 | 未由扫描证明；以证据限定范围为准 |
| `write_access_buffer` | 76 | 未由扫描证明；以证据限定范围为准 |
| `write_access_buffer_concurrent` | 90 | 未由扫描证明；以证据限定范围为准 |
| `ssbo_to_rg_handle` | 106 | 未由扫描证明；以证据限定范围为准 |
| `as_texture` | 117 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 118 | 未由扫描证明；以证据限定范围为准 |
| `as_texture_mut` | 123 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 124 | 未由扫描证明；以证据限定范围为准 |
| `as_texture` | 128 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 131 | 未由扫描证明；以证据限定范围为准 |
| `as_texture` | 140 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 143 | 未由扫描证明；以证据限定范围为准 |
| `as_texture` | 149 | 未由扫描证明；以证据限定范围为准 |
| `dims` | 152 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L141：todo!("managed Handle<Texture>::as_texture — wired to rhapsody managed handle")
- L144：todo!("managed Handle<Texture>::dims — wired to rhapsody managed handle")
- L150：todo!("ReadOnlyHandle<Texture>::as_texture — wired to rhapsody temporal pair")
- L153：todo!("ReadOnlyHandle<Texture>::dims — wired to rhapsody temporal pair")
### reconstruction/country-core/src/render/pass_builder/pass_resources.rs

[reconstruction/country-core/src/render/pass_builder/pass_resources.rs](../../reconstruction/country-core/src/render/pass_builder/pass_resources.rs)

SHA256：`b751b57d84bc20545447410fc1ddcc55fe896bd260308695105a6e26b4351c75`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_from` | 25 | 未由扫描证明；以证据限定范围为准 |
| `get_resource_ptr` | 39 | 未由扫描证明；以证据限定范围为准 |
| `missing` | 44 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L9：/// 名称映射未恢复（typed gap）；推断为 frame_constants / frame_constants_storage_buffer /
- L40：None // typed gap：bevy world 内部资源表布局；经 unsafe_world_cell::get_resource 落实
### reconstruction/country-core/src/render/pass_builder/shader_uri.rs

[reconstruction/country-core/src/render/pass_builder/shader_uri.rs](../../reconstruction/country-core/src/render/pass_builder/shader_uri.rs)

SHA256：`3753187ca4bdccbb696766ac38f048373bf106dc4d6ad01b2b24c5d6fb294c98`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `parse` | 26 | 未由扫描证明；以证据限定范围为准 |
| `full` | 38 | 未由扫描证明；以证据限定范围为准 |
| `split_last` | 45 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 52 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 59 | 未由扫描证明；以证据限定范围为准 |
| `as_shader_def_value` | 73 | 未由扫描证明；以证据限定范围为准 |
| `compute` | 88 | 未由扫描证明；以证据限定范围为准 |
| `source_hash` | 98 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 99 | 未由扫描证明；以证据限定范围为准 |
| `source_hash` | 105 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 108 | 未由扫描证明；以证据限定范围为准 |
| `source_hash` | 117 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 120 | 未由扫描证明；以证据限定范围为准 |
| `source_hash` | 130 | 未由扫描证明；以证据限定范围为准 |
| `as_compute_pipeline` | 133 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L66：/// 具体编码 U —— typed gap：仅作 trait 返回类型。
- L89：// 算法未恢复（typed gap）：由 utils::calculate_hash 承担。
### reconstruction/country-core/src/render/pass_builder/uniform_binder.rs

[reconstruction/country-core/src/render/pass_builder/uniform_binder.rs](../../reconstruction/country-core/src/render/pass_builder/uniform_binder.rs)

SHA256：`3f85af39f21c4291ea282e9f3435d442ff984e5a84122588746d672e5a35ef99`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 28 | 未由扫描证明；以证据限定范围为准 |
| `write_uniform` | 37 | 未由扫描证明；以证据限定范围为准 |
| `write_optional_uniform` | 50 | 未由扫描证明；以证据限定范围为准 |
| `insert_binding` | 57 | 未由扫描证明；以证据限定范围为准 |
| `default` | 63 | 未由扫描证明；以证据限定范围为准 |
| `len` | 70 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes` | 71 | 未由扫描证明；以证据限定范围为准 |
| `len` | 75 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes` | 78 | 未由扫描证明；以证据限定范围为准 |
| `len` | 84 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes` | 87 | 未由扫描证明；以证据限定范围为准 |
| `len` | 93 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes` | 96 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/raster_single.rs

[reconstruction/country-core/src/render/raster_single.rs](../../reconstruction/country-core/src/render/raster_single.rs)

SHA256：`50eeaa0c44379aa7defa0c56cfaca27f626d41c9f747074688dc29c50e3f9312`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raster_single_object` | 25 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L10：//! helper body is an integration gap (todo marker; O entry addresses preserved).
- L23：/// not expose (device + world params, builder-mode capture). Body is a todo marker; the real
- L32：todo!("raster_single_object — real wp1 RasterPass/DrawBuilder API differs (integration gap)")
### reconstruction/country-core/src/render/render_config.rs

[reconstruction/country-core/src/render/render_config.rs](../../reconstruction/country-core/src/render/render_config.rs)

SHA256：`8c5de46fe883b7a17d37ee8bb34dc41fdff9537a7a1a554fcef1edefee6d30f1`；缺口注释标记数：7。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_fullscreen` | 23 | 未由扫描证明；以证据限定范围为准 |
| `resolution` | 43 | 未由扫描证明；以证据限定范围为准 |
| `tgsr_resolution_scale` | 66 | 未由扫描证明；以证据限定范围为准 |
| `upgrade` | 103 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：//! typed gaps (U); the two observed functions are implemented.
- L88：/// Saved video settings block (serde RON). Layout U (typed gap) — only the
- L97：_gap: crate::typed_gap::Gap,
- L114：_gap: crate::typed_gap::Gap,
- L119：_gap: crate::typed_gap::Gap,
- L124：_gap: crate::typed_gap::Gap,
- L131：_gap: crate::typed_gap::Gap,
### reconstruction/country-core/src/render/shader.rs

[reconstruction/country-core/src/render/shader.rs](../../reconstruction/country-core/src/render/shader.rs)

SHA256：`1c0ec4a5f785cb3e843a40b4634548cdc643df56ca072ba29eccfe8f59b35365`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 38 | 未由扫描证明；以证据限定范围为准 |
| `new_draw` | 70 | 未由扫描证明；以证据限定范围为准 |
| `desc` | 84 | 未由扫描证明；以证据限定范围为准 |
| `recompile` | 89 | 未由扫描证明；以证据限定范围为准 |
| `default_handle` | 95 | 未由扫描证明；以证据限定范围为准 |
| `compile_shader_variants` | 116 | 未由扫描证明；以证据限定范围为准 |
| `new` | 147 | 未由扫描证明；以证据限定范围为准 |
| `get_or_generate` | 159 | 未由扫描证明；以证据限定范围为准 |
| `default` | 180 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L94：/// used by wp1 `DrawPacket::new`). Typed gap (assembly-completed static).
### reconstruction/country-core/src/render/shadowed_buffer.rs

[reconstruction/country-core/src/render/shadowed_buffer.rs](../../reconstruction/country-core/src/render/shadowed_buffer.rs)

SHA256：`d38d6ffbdb4d52618f30add216e4eb9545f037d6ed937397731706dd8040b879`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 29 | 未由扫描证明；以证据限定范围为准 |
| `buffer` | 53 | 未由扫描证明；以证据限定范围为准 |
| `get_mut` | 58 | 未由扫描证明；以证据限定范围为准 |
| `copy` | 64 | 未由扫描证明；以证据限定范围为准 |
| `write` | 77 | 未由扫描证明；以证据限定范围为准 |
| `gpu_flush` | 92 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/sparse_instance_buffer.rs

[reconstruction/country-core/src/render/sparse_instance_buffer.rs](../../reconstruction/country-core/src/render/sparse_instance_buffer.rs)

SHA256：`e9d75419e53e19183071a3f6a4818d2eb0fef38a9339bbb0786f6e5335efd09a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_null` | 56 | 未由扫描证明；以证据限定范围为准 |
| `new_storage_buffer` | 89 | 未由扫描证明；以证据限定范围为准 |
| `buffer` | 121 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 126 | 未由扫描证明；以证据限定范围为准 |
| `write_allocation` | 131 | 未由扫描证明；以证据限定范围为准 |
| `hide` | 136 | 未由扫描证明；以证据限定范围为准 |
| `show` | 148 | 未由扫描证明；以证据限定范围为准 |
| `take_out_waste` | 164 | 未由扫描证明；以证据限定范围为准 |
| `gpu_flush` | 184 | 未由扫描证明；以证据限定范围为准 |
| `allocation_to_draw_multi_instanced` | 228 | 未由扫描证明；以证据限定范围为准 |
| `to_draw_multi_instanced` | 256 | 未由扫描证明；以证据限定范围为准 |
| `push` | 320 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 368 | 未由扫描证明；以证据限定范围为准 |
| `as_any` | 396 | 未由扫描证明；以证据限定范围为准 |
| `as_any_mut` | 397 | 未由扫描证明；以证据限定范围为准 |
| `handle` | 398 | 未由扫描证明；以证据限定范围为准 |
| `as_any` | 402 | 未由扫描证明；以证据限定范围为准 |
| `as_any_mut` | 405 | 未由扫描证明；以证据限定范围为准 |
| `handle` | 408 | 未由扫描证明；以证据限定范围为准 |
| `bitmap_set` | 415 | 未由扫描证明；以证据限定范围为准 |
| `bitmap_unset` | 421 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/country-core/src/render/ssbo.rs

[reconstruction/country-core/src/render/ssbo.rs](../../reconstruction/country-core/src/render/ssbo.rs)

SHA256：`285b4bd82d9e84412fcd4bd584675f295fdf9095cb0265870b972bf7d7502d52`；缺口注释标记数：7。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 34 | 未由扫描证明；以证据限定范围为准 |
| `alignment` | 38 | 未由扫描证明；以证据限定范围为准 |
| `new_gpu_only` | 42 | 未由扫描证明；以证据限定范围为准 |
| `create_buffer` | 58 | 未由扫描证明；以证据限定范围为准 |
| `update_buffer_bytes` | 60 | 未由扫描证明；以证据限定范围为准 |
| `slice_as_bytes` | 69 | 未由扫描证明；以证据限定范围为准 |
| `mapped_slice_mut` | 74 | 未由扫描证明；以证据限定范围为准 |
| `buffer_cpu_ptr` | 81 | 未由扫描证明；以证据限定范围为准 |
| `new` | 128 | 未由扫描证明；以证据限定范围为准 |
| `new_arbitrary` | 146 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write` | 151 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write_with_offset` | 156 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write_ranges_nonoverlapping` | 177 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write_element` | 200 | 未由扫描证明；以证据限定范围为准 |
| `deref` | 209 | 未由扫描证明；以证据限定范围为准 |
| `new_storage_buffer` | 229 | 未由扫描证明；以证据限定范围为准 |
| `set_instance_count` | 247 | 未由扫描证明；以证据限定范围为准 |
| `instance_count` | 251 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write` | 255 | 未由扫描证明；以证据限定范围为准 |
| `gpu_write_subset` | 259 | 未由扫描证明；以证据限定范围为准 |
| `new_storage_buffer` | 292 | 未由扫描证明；以证据限定范围为准 |
| `mapped_data` | 339 | 未由扫描证明；以证据限定范围为准 |
| `mapped_data_mut` | 343 | 未由扫描证明；以证据限定范围为准 |
| `cpu_write` | 349 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L5：//! Every field is tagged O (observed) / I (inferred) / U (unknown typed gap).
- L9：//! reimplement. Unknown fields are typed gaps (`_gapN`), never guessed.
- L119：/// U — typed gap (not referenced by any decompiled WP2 consumer).
- L147：unreachable!("new_arbitrary: typed gap — not recovered")
- L201：unreachable!("gpu_write_element: typed gap — not recovered")
- L265：unreachable!("gpu_write_subset: typed gap — not recovered")
- L280：/// U — typed gaps (not referenced by decompiled WP2 consumers).
### reconstruction/country-core/src/render/uniform.rs

[reconstruction/country-core/src/render/uniform.rs](../../reconstruction/country-core/src/render/uniform.rs)

SHA256：`aa1dde53b087a0e28d04945e6dcc8b9ad181fe6df1a7283bdd1c3a3450a45c55`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `bind_lazy_uniform` | 18 | 未由扫描证明；以证据限定范围为准 |
| `write_uniform_to_buffer` | 27 | 未由扫描证明；以证据限定范围为准 |
| `write_uniform_to_buffer_aligned` | 36 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L8：//! (its body is a todo marker; the O call shape is preserved in the doc).
- L17：/// (`u64 hash, &T value`) does not match, so the body is a todo marker (never fabricated).
- L23：todo!("bind_lazy_uniform — real UniformBinder keys by BindingIndex (integration gap)")
### reconstruction/country-core/src/render/utils.rs

[reconstruction/country-core/src/render/utils.rs](../../reconstruction/country-core/src/render/utils.rs)

SHA256：`f222e49e279847b8532248384be323c9ebadb84fc7ae69f495214b431f7b83f8`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calculate_hash` | 7 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L5：/// Exact algorithm U (typed gap); std DefaultHasher is a behavior-equivalent
### reconstruction/country-core/src/render/vao.rs

[reconstruction/country-core/src/render/vao.rs](../../reconstruction/country-core/src/render/vao.rs)

SHA256：`68b32ef48204c547e1bd99854e843325e172c2287dbb01aedae37920bd2d046b`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `size_in_bytes` | 12 | 未由扫描证明；以证据限定范围为准 |
| `as_bytes` | 13 | 未由扫描证明；以证据限定范围为准 |
| `build_vao` | 38 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L30：/// U — typed gaps (plausibly vertex count / stride / attribute meta; not observed).
### reconstruction/country-core/src/render/view_constants.rs

[reconstruction/country-core/src/render/view_constants.rs](../../reconstruction/country-core/src/render/view_constants.rs)

SHA256：`1732a13de53ec34ef9843f91482ce4ed4b49854cd3c017cdecc60af0848c956d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `set_pixel_offset` | 46 | 未由扫描证明；以证据限定范围为准 |
| `new` | 79 | 未由扫描证明；以证据限定范围为准 |
| `pixel_offset` | 94 | 未由扫描证明；以证据限定范围为准 |
| `build` | 101 | 未由扫描证明；以证据限定范围为准 |
| `eye_position` | 126 | 未由扫描证明；以证据限定范围为准 |
| `prev_eye_position` | 130 | 未由扫描证明；以证据限定范围为准 |
| `fov_y_radians` | 134 | 未由扫描证明；以证据限定范围为准 |
| `near_plane` | 138 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs

[skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs](../../skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs)

SHA256：`a11ca50f0ed4826dca3c6c5b80fe6f1642f8b2bc8e01a5da15582899a4e26762`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `is_null` | 15 | 未由扫描证明；以证据限定范围为准 |
