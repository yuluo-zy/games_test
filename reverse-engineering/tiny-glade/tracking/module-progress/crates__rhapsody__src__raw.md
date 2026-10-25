# crates/rhapsody/src/raw

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

设备、buffer/image/view/sampler、descriptor/pipeline/attachments、command/submit/WSI/诊断/格式候选可运行。 2026-10-13：真实compiled asset直接stored reflection生产布局/compute，删除catalog offset生产fixture。

## 接线与验证

接线：真实Vulkan候选资源/WSI链；原完整ABI/行为未验收。

10/12候选统一64测试/13GPU探针；10/13补接口69测试和严格Clippy、未重跑GPU。报告当前性逐项检查。

## 已恢复的函数和类型范围

- GraphicsDevice资源创建/屏障/扩展和create_raw_device
- RasterPipelineOpts、descriptor/binding/shader/cache
- CommandBuffer/SubmitInfo/SubmissionEngine/Queue
- Swapchain acquire→submit→present及诊断/格式原入口
- register_compiled_compute/raster→retained schema→pipeline layout

## 明确缺口

- get_setup_cb_submit_info/with_setup_cb、init_texture_data/update_texture、fullscreen exclusive
- 原设备完整策略、allocator/vk-sync/winit分叉和全pipeline/pass关联

下一闭包：get_setup_cb_submit_info/with_setup_cb、init_texture_data/update_texture、fullscreen exclusive

## 证据

- [reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)
- [reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json](../../reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json)
- [reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json)
- [reconstruction/rhapsody-rebuild/evidence/raw-resource-implementation.json](../../reconstruction/rhapsody-rebuild/evidence/raw-resource-implementation.json)
- [reconstruction/rhapsody-rebuild/evidence/shader-production-validation.json](../../reconstruction/rhapsody-rebuild/evidence/shader-production-validation.json)
- [reconstruction/rhapsody-rebuild/evidence/swapchain-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/swapchain-functional-validation.json)
- [evidence/shader-toolchain-2026-10-13/integration.json](../../evidence/shader-toolchain-2026-10-13/integration.json)
- [evidence/shader-toolchain-2026-10-13/runtime/runtime-contract.json](../../evidence/shader-toolchain-2026-10-13/runtime/runtime-contract.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/src/raw/allocator.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/allocator.rs](../../reconstruction/rhapsody-rebuild/src/raw/allocator.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/barrier.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/barrier.rs](../../reconstruction/rhapsody-rebuild/src/raw/barrier.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/binding.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/binding.rs](../../reconstruction/rhapsody-rebuild/src/raw/binding.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/buffer.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/buffer.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/bytes.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/bytes.rs](../../reconstruction/rhapsody-rebuild/src/raw/bytes.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/command_buffer.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/command_buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/command_buffer.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/crash.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/crash.rs](../../reconstruction/rhapsody-rebuild/src/raw/crash.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/debug.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/debug.rs](../../reconstruction/rhapsody-rebuild/src/raw/debug.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/descriptor_set.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs](../../reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/device.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/device.rs](../../reconstruction/rhapsody-rebuild/src/raw/device.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/disposal.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/disposal.rs](../../reconstruction/rhapsody-rebuild/src/raw/disposal.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/dynamic_constants.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/dynamic_constants.rs](../../reconstruction/rhapsody-rebuild/src/raw/dynamic_constants.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/error.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/error.rs](../../reconstruction/rhapsody-rebuild/src/raw/error.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/format.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/format.rs](../../reconstruction/rhapsody-rebuild/src/raw/format.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/framebuffer.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/framebuffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/framebuffer.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/init.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/init.rs](../../reconstruction/rhapsody-rebuild/src/raw/init.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/instance.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/instance.rs](../../reconstruction/rhapsody-rebuild/src/raw/instance.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/physical_device.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/physical_device.rs](../../reconstruction/rhapsody-rebuild/src/raw/physical_device.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/profiler.rs` | B | [reconstruction/rhapsody-rebuild/src/raw/profiler.rs](../../reconstruction/rhapsody-rebuild/src/raw/profiler.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/ray_tracing.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/ray_tracing.rs](../../reconstruction/render/original-skeleton/crates/rhapsody/src/raw/ray_tracing.rs)（缺失）：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | recorded_missing；not_file_level_verified |
| `crates/rhapsody/src/raw/render_pass.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/render_pass.rs](../../reconstruction/rhapsody-rebuild/src/raw/render_pass.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/sampler.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/sampler.rs](../../reconstruction/rhapsody-rebuild/src/raw/sampler.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/semaphore.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/semaphore.rs](../../reconstruction/rhapsody-rebuild/src/raw/semaphore.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/shader.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/shader.rs](../../reconstruction/rhapsody-rebuild/src/raw/shader.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/stats.rs` | B | [reconstruction/rhapsody-rebuild/src/raw/stats.rs](../../reconstruction/rhapsody-rebuild/src/raw/stats.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/submit.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/submit.rs](../../reconstruction/rhapsody-rebuild/src/raw/submit.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/surface.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/src/raw/swapchain.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/swapchain.rs](../../reconstruction/rhapsody-rebuild/src/raw/swapchain.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/temp_list.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/temp_list.rs](../../reconstruction/rhapsody-rebuild/src/raw/temp_list.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/texture.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/texture.rs](../../reconstruction/rhapsody-rebuild/src/raw/texture.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/workarounds.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/raw/allocator.rs

[reconstruction/rhapsody-rebuild/src/raw/allocator.rs](../../reconstruction/rhapsody-rebuild/src/raw/allocator.rs)

SHA256：`54dd86c9ae7e1df14c17e85cefa30d131d1a8fd648c80b459abca8a0a195ccc7`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `choose_memory_type` | 14 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 48 | 未由扫描证明；以证据限定范围为准 |
| `allocate_with_flags` | 62 | 未由扫描证明；以证据限定范围为准 |
| `allocate_dedicated` | 76 | 未由扫描证明；以证据限定范围为准 |
| `into_deferred` | 135 | 未由扫描证明；以证据限定范围为准 |
| `accounting_id` | 151 | 未由扫描证明；以证据限定范围为准 |
| `is_suballocated` | 154 | 未由扫描证明；以证据限定范围为准 |
| `host_ptr` | 159 | 未由扫描证明；以证据限定范围为准 |
| `host_range` | 167 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 175 | 未由扫描证明；以证据限定范围为准 |
| `properties` | 198 | 未由扫描证明；以证据限定范围为准 |
| `report` | 259 | 未由扫描证明；以证据限定范围为准 |
| `materialize_peak` | 265 | 未由扫描证明；以证据限定范围为准 |
| `report` | 273 | 未由扫描证明；以证据限定范围为准 |
| `peak_allocator_report` | 278 | 未由扫描证明；以证据限定范围为准 |
| `total_registered` | 284 | 未由扫描证明；以证据限定范围为准 |
| `peak_totals` | 287 | 未由扫描证明；以证据限定范围为准 |
| `set_heap_budget` | 291 | 未由扫描证明；以证据限定范围为准 |
| `register` | 300 | 未由扫描证明；以证据限定范围为准 |
| `release` | 371 | 未由扫描证明；以证据限定范围为准 |
| `rename` | 393 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 417 | 未由扫描证明；以证据限定范围为准 |
| `free` | 418 | 未由扫描证明；以证据限定范围为准 |
| `report` | 419 | 未由扫描证明；以证据限定范围为准 |
| `new` | 429 | 未由扫描证明；以证据限定范围为准 |
| `inner` | 437 | 未由扫描证明；以证据限定范围为准 |
| `inner_mut` | 440 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 443 | 未由扫描证明；以证据限定范围为准 |
| `observe_report` | 451 | 未由扫描证明；以证据限定范围为准 |
| `free` | 462 | 未由扫描证明；以证据限定范围为准 |
| `peak_total_allocated_bytes` | 465 | 未由扫描证明；以证据限定范围为准 |
| `peak_device_local_bytes` | 468 | 未由扫描证明；以证据限定范围为准 |
| `peak_system_bytes` | 471 | 未由扫描证明；以证据限定范围为准 |
| `peak_allocator_report` | 474 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 483 | 未由扫描证明；以证据限定范围为准 |
| `free` | 497 | 未由扫描证明；以证据限定范围为准 |
| `report` | 504 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L195：Unknown,
- L212：Self::Unknown => (
### reconstruction/rhapsody-rebuild/src/raw/barrier.rs

[reconstruction/rhapsody-rebuild/src/raw/barrier.rs](../../reconstruction/rhapsody-rebuild/src/raw/barrier.rs)

SHA256：`4f05994e5721430af6909c444e1f62271ed882d2be6492694ff7fadff19858cd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 18 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new` | 38 | 未由扫描证明；以证据限定范围为准 |
| `new` | 62 | 未由扫描证明；以证据限定范围为准 |
| `concurrent` | 109 | 未由扫描证明；以证据限定范围为准 |
| `validate` | 117 | 未由扫描证明；以证据限定范围为准 |
| `pre` | 129 | 未由扫描证明；以证据限定范围为准 |
| `initial` | 165 | 未由扫描证明；以证据限定范围为准 |
| `export` | 194 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/binding.rs

[reconstruction/rhapsody-rebuild/src/raw/binding.rs](../../reconstruction/rhapsody-rebuild/src/raw/binding.rs)

SHA256：`cb5f7c0a926e455bdab99be4fab599d971a5199b019a269df7dc1014b3d1792b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `descriptor_type` | 29 | 未由扫描证明；以证据限定范围为准 |
| `owners` | 34 | 未由扫描证明；以证据限定范围为准 |
| `write` | 43 | 未由扫描证明；以证据限定范围为准 |
| `bind_pipeline` | 74 | 未由扫描证明；以证据限定范围为准 |
| `bind_persistent_descriptor_set` | 89 | 未由扫描证明；以证据限定范围为准 |
| `bind_descriptor_set` | 112 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/buffer.rs

[reconstruction/rhapsody-rebuild/src/raw/buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/buffer.rs)

SHA256：`7f9ff2f0b906ce3f7acc0f60bf2ce3f18b00a5d74e870dd0a7e2e9937628dbb6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_raw` | 21 | 未由扫描证明；以证据限定范围为准 |
| `from_raw` | 24 | 未由扫描证明；以证据限定范围为准 |
| `memory_properties` | 34 | 未由扫描证明；以证据限定范围为准 |
| `new` | 56 | 未由扫描证明；以证据限定范围为准 |
| `new_gpu_only` | 64 | 未由扫描证明；以证据限定范围为准 |
| `new_cpu_to_gpu` | 67 | 未由扫描证明；以证据限定范围为准 |
| `new_cpu_to_gpu_low_prio` | 71 | 未由扫描证明；以证据限定范围为准 |
| `new_gpu_to_cpu` | 74 | 未由扫描证明；以证据限定范围为准 |
| `with_alignment` | 78 | 未由扫描证明；以证据限定范围为准 |
| `alignment` | 82 | 未由扫描证明；以证据限定范围为准 |
| `new` | 95 | 未由扫描证明；以证据限定范围为准 |
| `mapped_slice_mut` | 158 | 未由扫描证明；以证据限定范围为准 |
| `unmap` | 205 | 未由扫描证明；以证据限定范围为准 |
| `host_range_for_test` | 215 | 未由扫描证明；以证据限定范围为准 |
| `device_address` | 219 | 未由扫描证明；以证据限定范围为准 |
| `transfer` | 230 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes` | 303 | 未由扫描证明；以证据限定范围为准 |
| `write_bytes_unsynchronized` | 309 | 未由扫描证明；以证据限定范围为准 |
| `read_bytes_after_completion` | 318 | 未由扫描证明；以证据限定范围为准 |
| `read_bytes` | 321 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 326 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/bytes.rs

[reconstruction/rhapsody-rebuild/src/raw/bytes.rs](../../reconstruction/rhapsody-rebuild/src/raw/bytes.rs)

SHA256：`f0ddd06638c8c4e8f40ed9745597b662621a8cd70fc0380b34e8581cbe7f5123`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `byte_count` | 3 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_slice` | 4 | 未由扫描证明；以证据限定范围为准 |
| `slice_as_bytes` | 10 | 未由扫描证明；以证据限定范围为准 |
| `bytes_as_slice` | 22 | 未由扫描证明；以证据限定范围为准 |
| `byte_count` | 28 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_slice` | 31 | 未由扫描证明；以证据限定范围为准 |
| `byte_count` | 36 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_slice` | 39 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/command_buffer.rs

[reconstruction/rhapsody-rebuild/src/raw/command_buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/command_buffer.rs)

SHA256：`3e296c77aab9d152053d2c45a13041d840be5997050ffdba3fa07ec004e52b61`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `create` | 14 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 15 | 未由扫描证明；以证据限定范围为准 |
| `ensure_active_cb` | 19 | 未由扫描证明；以证据限定范围为准 |
| `create` | 39 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 44 | 未由扫描证明；以证据限定范围为准 |
| `new` | 67 | 未由扫描证明；以证据限定范围为准 |
| `new_for_family` | 71 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 88 | 未由扫描证明；以证据限定范围为准 |
| `recycle` | 104 | 未由扫描证明；以证据限定范围为准 |
| `new` | 115 | 未由扫描证明；以证据限定范围为准 |
| `current` | 119 | 未由扫描证明；以证据限定范围为准 |
| `track` | 123 | 未由扫描证明；以证据限定范围为准 |
| `set` | 126 | 未由扫描证明；以证据限定范围为准 |
| `record_default` | 140 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 149 | 未由扫描证明；以证据限定范围为准 |
| `is_recording` | 163 | 未由扫描证明；以证据限定范围为准 |
| `reset` | 167 | 未由扫描证明；以证据限定范围为准 |
| `reset_after_completion` | 190 | 未由扫描证明；以证据限定范围为准 |
| `is_executable` | 208 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 211 | 未由扫描证明；以证据限定范围为准 |
| `end` | 226 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_reference` | 237 | 未由扫描证明；以证据限定范围为准 |
| `keep_alive` | 246 | 未由扫描证明；以证据限定范围为准 |
| `keep_resource_owner` | 249 | 未由扫描证明；以证据限定范围为准 |
| `release_completed_owners` | 254 | 未由扫描证明；以证据限定范围为准 |
| `submit` | 257 | 未由扫描证明；以证据限定范围为准 |
| `submit_to` | 266 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 336 | 未由扫描证明；以证据限定范围为准 |
| `wait` | 354 | 未由扫描证明；以证据限定范围为准 |
| `is_complete` | 365 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 378 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/crash.rs

[reconstruction/rhapsody-rebuild/src/raw/crash.rs](../../reconstruction/rhapsody-rebuild/src/raw/crash.rs)

SHA256：`afb071b9b048e92c9cdad4fe129b237b3441c8b1c74372da9f8afe276090ddf9`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `has_any_gpu_assertion_failed` | 34 | 未由扫描证明；以证据限定范围为准 |
| `parse_gpu_debug_info` | 43 | 未由扫描证明；以证据限定范围为准 |
| `new` | 115 | 未由扫描证明；以证据限定范围为准 |
| `insert_name` | 118 | 未由扫描证明；以证据限定范围为准 |
| `get_name` | 129 | 未由扫描证明；以证据限定范围为准 |
| `len` | 132 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 135 | 未由扫描证明；以证据限定范围为准 |
| `log_gpu_debug_info` | 142 | 未由扫描证明；以证据限定范围为准 |
| `new` | 167 | 未由扫描证明；以证据限定范围为准 |
| `record_crash_marker` | 197 | 未由扫描证明；以证据限定范围为准 |
| `keep_debug_buffer` | 230 | 未由扫描证明；以证据限定范围为准 |
| `make_host_readable` | 234 | 未由扫描证明；以证据限定范围为准 |
| `log_gpu_crash_info` | 269 | 未由扫描证明；以证据限定范围为准 |
| `needs_vulkan_error_info` | 300 | 未由扫描证明；以证据限定范围为准 |
| `log_vulkan_error_info` | 303 | 未由扫描证明；以证据限定范围为准 |
| `require_vendor_crash_dump` | 318 | 未由扫描证明；以证据限定范围为准 |
| `log_error_info` | 323 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/debug.rs

[reconstruction/rhapsody-rebuild/src/raw/debug.rs](../../reconstruction/rhapsody-rebuild/src/raw/debug.rs)

SHA256：`e8a7d92a5006492c1293385c27a3fc90fea3b3d4d68da856fe40c59d7e9b64d1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 15 | 未由扫描证明；以证据限定范围为准 |
| `set_object_name` | 21 | 未由扫描证明；以证据限定范围为准 |
| `begin_label` | 34 | 未由扫描证明；以证据限定范围为准 |
| `end_label` | 49 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs

[reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs](../../reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs)

SHA256：`c318971fd04182bfafcb384b59cb8fe4d17fa24d020eb7f1df86e82789c4373c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new_with_samplers` | 34 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 108 | 未由扫描证明；以证据限定范围为准 |
| `new` | 120 | 未由扫描证明；以证据限定范围为准 |
| `new_transient` | 129 | 未由扫描证明；以证据限定范围为准 |
| `create` | 137 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 183 | 未由扫描证明；以证据限定范围为准 |
| `reset` | 221 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 231 | 未由扫描证明；以证据限定范围为准 |
| `update` | 249 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 276 | 未由扫描证明；以证据限定范围为准 |
| `runtime_descriptor_count` | 316 | 未由扫描证明；以证据限定范围为准 |
| `descriptor_set_layout` | 322 | 未由扫描证明；以证据限定范围为准 |
| `with_create_info` | 385 | 未由扫描证明；以证据限定范围为准 |
| `with_compute_pipeline_layout` | 426 | 未由扫描证明；以证据限定范围为准 |
| `persistent_pool_plan` | 455 | 未由扫描证明；以证据限定范围为准 |
| `with_pool_create_info` | 476 | 未由扫描证明；以证据限定范围为准 |
| `with_allocate_info` | 496 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 532 | 未由扫描证明；以证据限定范围为准 |
| `new` | 554 | 未由扫描证明；以证据限定范围为准 |
| `push` | 557 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 560 | 未由扫描证明；以证据限定范围为准 |
| `len` | 563 | 未由扫描证明；以证据限定范围为准 |
| `reclaim_descriptor_pools` | 567 | 未由扫描证明；以证据限定范围为准 |
| `default` | 577 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 588 | 未由扫描证明；以证据限定范围为准 |
| `new` | 637 | 未由扫描证明；以证据限定范围为准 |
| `allocate_descriptor_set` | 653 | 未由扫描证明；以证据限定范围为准 |
| `allocate_descriptor_set_with_retired` | 665 | 未由扫描证明；以证据限定范围为准 |
| `validate_writes` | 700 | 未由扫描证明；以证据限定范围为准 |
| `invalidate` | 739 | 未由扫描证明；以证据限定范围为准 |
| `cached_set_count` | 744 | 未由扫描证明；以证据限定范围为准 |
| `free_pool_count` | 747 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/device.rs

[reconstruction/rhapsody-rebuild/src/raw/device.rs](../../reconstruction/rhapsody-rebuild/src/raw/device.rs)

SHA256：`f0efa038a5ccfe8705145a4731a1620ebaa656f5b4994dcb52ad80c2f10a026f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `max_descriptor_set_update_after_bind_sampled_images` | 51 | 未由扫描证明；以证据限定范围为准 |
| `create` | 62 | 未由扫描证明；以证据限定范围为准 |
| `wait_idle` | 165 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 171 | 未由扫描证明；以证据限定范围为准 |
| `counter_slot` | 192 | 未由扫描证明；以证据限定范围为准 |
| `select` | 215 | 未由扫描证明；以证据限定范围为准 |
| `counters` | 225 | 未由扫描证明；以证据限定范围为准 |
| `new` | 235 | 未由扫描证明；以证据限定范围为准 |
| `len` | 243 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 247 | 未由扫描证明；以证据限定范围为准 |
| `begin_sequence` | 251 | 未由扫描证明；以证据限定范围为准 |
| `next_value` | 257 | 未由扫描证明；以证据限定范围为准 |
| `raw_begin_projection` | 264 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 287 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 291 | 未由扫描证明；以证据限定范围为准 |
| `create` | 307 | 未由扫描证明；以证据限定范围为准 |
| `from_device` | 342 | 未由扫描证明；以证据限定范围为准 |
| `device_uuid` | 357 | 未由扫描证明；以证据限定范围为准 |
| `limits` | 360 | 未由扫描证明；以证据限定范围为准 |
| `min_subgroup_size` | 364 | 未由扫描证明；以证据限定范围为准 |
| `physical_device` | 368 | 未由扫描证明；以证据限定范围为准 |
| `queue_family_index` | 371 | 未由扫描证明；以证据限定范围为准 |
| `debug_utils` | 374 | 未由扫描证明；以证据限定范围为准 |
| `max_bindless_descriptor_count` | 378 | 未由扫描证明；以证据限定范围为准 |
| `rt` | 382 | 未由扫描证明；以证据限定范围为准 |
| `wait_idle_and_recycle_resources` | 386 | 未由扫描证明；以证据限定范围为准 |
| `allocate_command_buffer_seq` | 397 | 未由扫描证明；以证据限定范围为准 |
| `command_buffer_seq` | 400 | 未由扫描证明；以证据限定范围为准 |
| `disposal` | 403 | 未由扫描证明；以证据限定范围为准 |
| `create_mapped_buffer` | 409 | 未由扫描证明；以证据限定范围为准 |
| `create_buffer_impl` | 419 | 未由扫描证明；以证据限定范围为准 |
| `create_gpu_profiler_output_buffer` | 426 | 未由扫描证明；以证据限定范围为准 |
| `init_texture_data` | 439 | 未由扫描证明；以证据限定范围为准 |
| `update_texture` | 446 | 未由扫描证明；以证据限定范围为准 |
| `create_samplers` | 454 | 未由扫描证明；以证据限定范围为准 |
| `get_built_in_sampler` | 495 | 未由扫描证明；以证据限定范围为准 |
| `query_device_extension_support` | 499 | 未由扫描证明；以证据限定范围为准 |
| `validate_physical_device_requirements` | 522 | 未由扫描证明；以证据限定范围为准 |
| `with_global_allocator` | 540 | 未由扫描证明；以证据限定范围为准 |
| `issue_pipeline_barrier` | 547 | 未由扫描证明；以证据限定范围为准 |
| `issue_and_reset_pipeline_barrier` | 565 | 未由扫描证明；以证据限定范围为准 |
| `issue_full_pipeline_barrier` | 576 | 未由扫描证明；以证据限定范围为准 |
| `wait_submit_done` | 590 | 未由扫描证明；以证据限定范围为准 |
| `wait_duration_or_until_submit_done` | 594 | 未由扫描证明；以证据限定范围为准 |
| `create_persistent_descriptor_set` | 606 | 未由扫描证明；以证据限定范围为准 |
| `get_setup_cb_submit_info` | 615 | 未由扫描证明；以证据限定范围为准 |
| `with_setup_cb` | 619 | 未由扫描证明；以证据限定范围为准 |
| `with_raw` | 635 | 未由扫描证明；以证据限定范围为准 |
| `update_pending_submissions` | 642 | 未由扫描证明；以证据限定范围为准 |
| `reclaim_command_buffer_resources` | 645 | 未由扫描证明；以证据限定范围为准 |
| `create_raw_device` | 655 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/disposal.rs

[reconstruction/rhapsody-rebuild/src/raw/disposal.rs](../../reconstruction/rhapsody-rebuild/src/raw/disposal.rs)

SHA256：`82e37c7fced4901598449965f0c7359f2276deaeb98267b262522402e7a8b4af`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `begin_command_bucket` | 11 | 未由扫描证明；以证据限定范围为准 |
| `contains` | 20 | 未由扫描证明；以证据限定范围为准 |
| `append_existing` | 23 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 37 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 43 | 未由扫描证明；以证据限定范围为准 |
| `dispose_cb_resource` | 46 | 未由扫描证明；以证据限定范围为准 |
| `drain_completed_sequence` | 50 | 未由扫描证明；以证据限定范围为准 |
| `dispose_in_order` | 58 | 未由扫描证明；以证据限定范围为准 |
| `pending` | 68 | 未由扫描证明；以证据限定范围为准 |
| `pending_sequences` | 72 | 未由扫描证明；以证据限定范围为准 |
| `drain_after_idle` | 75 | 未由扫描证明；以证据限定范围为准 |
| `immediate_empty_bucket` | 82 | 未由扫描证明；以证据限定范围为准 |
| `new` | 90 | 未由扫描证明；以证据限定范围为准 |
| `dispose` | 93 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 98 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/dynamic_constants.rs

[reconstruction/rhapsody-rebuild/src/raw/dynamic_constants.rs](../../reconstruction/rhapsody-rebuild/src/raw/dynamic_constants.rs)

SHA256：`34e2bf410b969949782e64d427466b9f18ea409be53709c070e3154c246ddf4e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `write_bytes` | 18 | 未由扫描证明；以证据限定范围为准 |
| `write` | 25 | 未由扫描证明；以证据限定范围为准 |
| `with_data` | 29 | 未由扫描证明；以证据限定范围为准 |
| `device_address` | 43 | 未由扫描证明；以证据限定范围为准 |
| `into_gpu_lifetime_tracker` | 49 | 未由扫描证明；以证据限定范围为准 |
| `binding` | 52 | 未由扫描证明；以证据限定范围为准 |
| `into_uniform_buffer_binding` | 63 | 未由扫描证明；以证据限定范围为准 |
| `into_uniform_and_storage_buffer_binding` | 66 | 未由扫描证明；以证据限定范围为准 |
| `into_uniform_buffer_binding_and_data` | 75 | 未由扫描证明；以证据限定范围为准 |
| `new` | 98 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 109 | 未由扫描证明；以证据限定范围为准 |
| `allocate_with_data` | 142 | 未由扫描证明；以证据限定范围为准 |
| `allocate_buffer` | 147 | 未由扫描证明；以证据限定范围为准 |
| `push` | 159 | 未由扫描证明；以证据限定范围为准 |
| `push_slice` | 163 | 未由扫描证明；以证据限定范围为准 |
| `reset` | 167 | 未由扫描证明；以证据限定范围为准 |
| `buffer_usage` | 178 | 未由扫描证明；以证据限定范围为准 |
| `page_count` | 185 | 未由扫描证明；以证据限定范围为准 |
| `align_up` | 189 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/error.rs

[reconstruction/rhapsody-rebuild/src/raw/error.rs](../../reconstruction/rhapsody-rebuild/src/raw/error.rs)

SHA256：`b7758b85d16d2fdd84acec02067957199e0ae786e2695b4e610bd202fe73c9e8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 12 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 17 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/format.rs

[reconstruction/rhapsody-rebuild/src/raw/format.rs](../../reconstruction/rhapsody-rebuild/src/raw/format.rs)

SHA256：`0ec62399b051d9919c7fd87cfd59369dc10a2b5da420999921fcdc6ace4633e3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `vk_format_info` | 18 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_valid_aspect_flags` | 21 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_block_bytes` | 24 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_planes` | 27 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_all_aspects` | 31 | 未由扫描证明；以证据限定范围为准 |
| `word_at_00` | 68 | 未由扫描证明；以证据限定范围为准 |
| `word_at_04` | 71 | 未由扫描证明；以证据限定范围为准 |
| `byte_at_08` | 74 | 未由扫描证明；以证据限定范围为准 |
| `byte_at_09` | 77 | 未由扫描证明；以证据限定范围为准 |
| `write_into` | 80 | 未由扫描证明；以证据限定范围为准 |
| `try_vk_format_valid_aspect_flags` | 88 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_valid_aspect_flags` | 99 | 未由扫描证明；以证据限定范围为准 |
| `try_vk_format_info` | 102 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_info` | 114 | 未由扫描证明；以证据限定范围为准 |
| `try_vk_format_block_bytes` | 119 | 未由扫描证明；以证据限定范围为准 |
| `vk_format_block_bytes` | 135 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/format_table.rs

[reconstruction/rhapsody-rebuild/src/raw/format_table.rs](../../reconstruction/rhapsody-rebuild/src/raw/format_table.rs)

SHA256：`131ed1d3ff646c6d15e11e72c0f439adad8287430150bab53d580ce0296e0779`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `registry_info` | 5 | 未由扫描证明；以证据限定范围为准 |
| `registry_planes` | 1255 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/framebuffer.rs

[reconstruction/rhapsody-rebuild/src/raw/framebuffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/framebuffer.rs)

SHA256：`50fe8c1bdddde859c2fec5353ce8acadac2b1c98f9d83c90dbc908be95019a47`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 19 | 未由扫描证明；以证据限定范围为准 |
| `new` | 41 | 未由扫描证明；以证据限定范围为准 |
| `from_desc` | 49 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 78 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/graveyard.rs

[reconstruction/rhapsody-rebuild/src/raw/graveyard.rs](../../reconstruction/rhapsody-rebuild/src/raw/graveyard.rs)

SHA256：`a196da705bf524711355baa105a709e8f490fb0827c2ff4b7d4aa156e70fcd64`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `note_submit` | 69 | 未由扫描证明；以证据限定范围为准 |
| `stats` | 72 | 未由扫描证明；以证据限定范围为准 |
| `is_quiescent` | 81 | 未由扫描证明；以证据限定范围为准 |
| `push` | 84 | 未由扫描证明；以证据限定范围为准 |
| `take_all` | 90 | 未由扫描证明；以证据限定范围为准 |
| `put_back` | 93 | 未由扫描证明；以证据限定范围为准 |
| `note_submit` | 105 | 未由扫描证明；以证据限定范围为准 |
| `bury` | 109 | 未由扫描证明；以证据限定范围为准 |
| `collect_garbage` | 123 | 未由扫描证明；以证据限定范围为准 |
| `wait_idle_and_collect` | 128 | 未由扫描证明；以证据限定范围为准 |
| `idle_if_in_flight_locked` | 140 | 未由扫描证明；以证据限定范围为准 |
| `garbage_stats` | 152 | 未由扫描证明；以证据限定范围为准 |
| `collect_locked` | 156 | 未由扫描证明；以证据限定范围为准 |
| `drain_graveyard_for_drop` | 182 | 未由扫描证明；以证据限定范围为准 |
| `destroy_now` | 192 | 未由扫描证明；以证据限定范围为准 |
| `free_deferred` | 219 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/init.rs

[reconstruction/rhapsody-rebuild/src/raw/init.rs](../../reconstruction/rhapsody-rebuild/src/raw/init.rs)

SHA256：`0d802ff1a43181af059fecf8825216812ff0c1bfaaa4db05537ebefd495e3ebc`；缺口注释标记数：11。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `vendor` | 21 | 未由扫描证明；以证据限定范围为准 |
| `contains_layer` | 43 | 未由扫描证明；以证据限定范围为准 |
| `contains_vendor` | 48 | 未由扫描证明；以证据限定范围为准 |
| `permits` | 53 | 未由扫描证明；以证据限定范围为准 |
| `update_from_ron` | 57 | 未由扫描证明；以证据限定范围为准 |
| `original_tag` | 87 | 未由扫描证明；以证据限定范围为准 |
| `is_emulated` | 129 | 未由扫描证明；以证据限定范围为准 |
| `is_old_intel` | 132 | 未由扫描证明；以证据限定范围为准 |
| `driver_support` | 150 | 未由扫描证明；以证据限定范围为准 |
| `is_compatible` | 194 | 未由扫描证明；以证据限定范围为准 |
| `is_supported` | 218 | 未由扫描证明；以证据限定范围为准 |
| `default` | 236 | 未由扫描证明；以证据限定范围为准 |
| `refresh_memory_budgets` | 253 | 未由扫描证明；以证据限定范围为准 |
| `from_candidates` | 274 | 未由扫描证明；以证据限定范围为准 |
| `missing_devices` | 287 | 未由扫描证明；以证据限定范围为准 |
| `available_devices` | 290 | 未由扫描证明；以证据限定范围为准 |
| `order_and_filter_devices` | 293 | 未由扫描证明；以证据限定范围为准 |
| `create_device` | 369 | 未由扫描证明；以证据限定范围为准 |
| `new_with_allowlist` | 420 | 未由扫描证明；以证据限定范围为准 |
| `available_for_surface` | 428 | 未由扫描证明；以证据限定范围为准 |
| `new` | 516 | 未由扫描证明；以证据限定范围为准 |
| `create_device` | 521 | 未由扫描证明；以证据限定范围为准 |
| `create_headless_device` | 605 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L84：Unknown,
- L96：Self::Unknown => 3,
- L104：Unknown(String),
- L149：/// 原限定跳表与版本比较；编码尚未确认时返回 Unknown，不能按厂商猜通过。
- L155：return DriverSupport::Unknown;
- L183：DriverVendor::Unknown => return DriverSupport::Unknown,
- L196：DriverSupport::Unknown => {
- L197：return DeviceDecision::Unknown("原驱动版本编码/支持条件未确认".into());
- L203：return DeviceDecision::Unknown(
- L329：&& matches!(compatible, DeviceDecision::Unknown(_)));
- L334：DeviceDecision::Unknown(reason) => {
### reconstruction/rhapsody-rebuild/src/raw/instance.rs

[reconstruction/rhapsody-rebuild/src/raw/instance.rs](../../reconstruction/rhapsody-rebuild/src/raw/instance.rs)

SHA256：`da3da6e983ddddd1903aed8d89d771aaa9fd3e9ee055950c077e16aa1d5d0d41`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `layer_names` | 16 | 未由扫描证明；以证据限定范围为准 |
| `new` | 33 | 未由扫描证明；以证据限定范围为准 |
| `from_candidates` | 37 | 未由扫描证明；以证据限定范围为准 |
| `implicit_layers` | 104 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 109 | 未由扫描证明；以证据限定范围为准 |
| `default` | 122 | 未由扫描证明；以证据限定范围为准 |
| `build` | 132 | 未由扫描证明；以证据限定范围为准 |
| `build_with_allowlist` | 135 | 未由扫描证明；以证据限定范围为准 |
| `build_with_loader` | 150 | 未由扫描证明；以证据限定范围为准 |
| `debug_markers` | 210 | 未由扫描证明；以证据限定范围为准 |
| `legacy_disable_layer` | 217 | 未由扫描证明；以证据限定范围为准 |
| `apply_vulkan_blocklist` | 223 | 未由扫描证明；以证据限定范围为准 |
| `apply_vulkan_modern_blocklist` | 232 | 未由扫描证明；以证据限定范围为准 |
| `apply_vulkan_legacy_blocklist` | 237 | 未由扫描证明；以证据限定范围为准 |
| `apply_vulkan_doubletap_blocklist` | 242 | 未由扫描证明；以证据限定范围为准 |
| `vulkan_debug_messenger` | 253 | 未由扫描证明；以证据限定范围为准 |
| `callback` | 267 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/mod.rs

[reconstruction/rhapsody-rebuild/src/raw/mod.rs](../../reconstruction/rhapsody-rebuild/src/raw/mod.rs)

SHA256：`36a382d66dca565b7c877a64fa09949522fabd7880592e4083938fd86e83c15d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/rhapsody-rebuild/src/raw/physical_device.rs

[reconstruction/rhapsody-rebuild/src/raw/physical_device.rs](../../reconstruction/rhapsody-rebuild/src/raw/physical_device.rs)

SHA256：`87785a927216cdd4093baed3a48e308bdb78a0e1a7335d3a01fd9ff393fd5438`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `original_tag` | 20 | 未由扫描证明；以证据限定范围为准 |
| `vendor_string` | 31 | 未由扫描证明；以证据限定范围为准 |
| `from_parts` | 50 | 未由扫描证明；以证据限定范围为准 |
| `vendor_string` | 58 | 未由扫描证明；以证据限定范围为准 |
| `is_amd` | 61 | 未由扫描证明；以证据限定范围为准 |
| `is_apple` | 64 | 未由扫描证明；以证据限定范围为准 |
| `from_vulkan` | 68 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 110 | 未由扫描证明；以证据限定范围为准 |
| `available_device_local_budget` | 153 | 未由扫描证明；以证据限定范围为准 |
| `driver_version` | 175 | 未由扫描证明；以证据限定范围为准 |
| `extension_names` | 189 | 未由扫描证明；以证据限定范围为准 |
| `performance_info` | 204 | 未由扫描证明；以证据限定范围为准 |
| `cached_performance_info` | 211 | 未由扫描证明；以证据限定范围为准 |
| `memory_snapshot` | 214 | 未由扫描证明；以证据限定范围为准 |
| `snapshot_or_refresh` | 263 | 未由扫描证明；以证据限定范围为准 |
| `device_local_memory_budget_bytes` | 272 | 未由扫描证明；以证据限定范围为准 |
| `host_memory_budget_bytes` | 275 | 未由扫描证明；以证据限定范围为准 |
| `name` | 278 | 未由扫描证明；以证据限定范围为准 |
| `device_local_memory_bytes` | 283 | 未由扫描证明；以证据限定范围为准 |
| `refresh_memory_budget` | 291 | 未由扫描证明；以证据限定范围为准 |
| `supports_memory_budget` | 306 | 未由扫描证明；以证据限定范围为准 |
| `with_presentation_support` | 321 | 未由扫描证明；以证据限定范围为准 |
| `host_memory_bytes` | 338 | 未由扫描证明；以证据限定范围为准 |
| `find_vendor_specific_device_handle` | 346 | 未由扫描证明；以证据限定范围为准 |
| `enumerate_vk_data` | 359 | 未由扫描证明；以证据限定范围为准 |
| `string_from_c_array` | 375 | 未由扫描证明；以证据限定范围为准 |
| `parse_driver_version` | 385 | 未由扫描证明；以证据限定范围为准 |
| `enumerate_physical_devices` | 389 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L17：Unknown,
- L28：Self::Unknown => 6,
- L38：Self::Unknown => "Unknown",
- L99：(DriverVendor::Unknown, standard, false)
### reconstruction/rhapsody-rebuild/src/raw/profiler.rs

[reconstruction/rhapsody-rebuild/src/raw/profiler.rs](../../reconstruction/rhapsody-rebuild/src/raw/profiler.rs)

SHA256：`69e1cacf4517ce734d28d6cb2adc69017156f7ce3ab2b1bb52b65685b672744d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 13 | 未由扫描证明；以证据限定范围为准 |
| `send_to_puffin` | 21 | 未由扫描证明；以证据限定范围为准 |
| `new` | 34 | 未由扫描证明；以证据限定范围为准 |
| `reset` | 61 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 71 | 未由扫描证明；以证据限定范围为准 |
| `end` | 90 | 未由扫描证明；以证据限定范围为准 |
| `results` | 101 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 132 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/render_pass.rs

[reconstruction/rhapsody-rebuild/src/raw/render_pass.rs](../../reconstruction/rhapsody-rebuild/src/raw/render_pass.rs)

SHA256：`19e6ebbb228ba43b0614afa39c907afb959e39f67415be055292d24671fde518`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compatibility_key` | 30 | 未由扫描证明；以证据限定范围为准 |
| `new` | 47 | 未由扫描证明；以证据限定范围为准 |
| `clear_input` | 55 | 未由扫描证明；以证据限定范围为准 |
| `sample_count` | 59 | 未由扫描证明；以证据限定范围为准 |
| `depth_stencil_attachment` | 63 | 未由扫描证明；以证据限定范围为准 |
| `to_attachment_key` | 67 | 未由扫描证明；以证据限定范围为准 |
| `from_key` | 98 | 未由扫描证明；以证据限定范围为准 |
| `new` | 146 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 152 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/resource_usage.rs

[reconstruction/rhapsody-rebuild/src/raw/resource_usage.rs](../../reconstruction/rhapsody-rebuild/src/raw/resource_usage.rs)

SHA256：`30800df36a9f8447085625b6c1b950365bb1e8ec253b7d0ad53ef9041e76541a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `buffer_access_mask_to_usage_flags` | 13 | 未由扫描证明；以证据限定范围为准 |
| `image_access_mask_to_usage_flags` | 27 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/sampler.rs

[reconstruction/rhapsody-rebuild/src/raw/sampler.rs](../../reconstruction/rhapsody-rebuild/src/raw/sampler.rs)

SHA256：`bfe873eda191b66fdb8c86806888f0b11854139a3b5412e82511c698321ddb77`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 10 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 16 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/semaphore.rs

[reconstruction/rhapsody-rebuild/src/raw/semaphore.rs](../../reconstruction/rhapsody-rebuild/src/raw/semaphore.rs)

SHA256：`f9fbd09b5586f2bccf6102191a569b2131c3f83de791f73ea5ced2dd3628f81a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 10 | 未由扫描证明；以证据限定范围为准 |
| `completed` | 18 | 未由扫描证明；以证据限定范围为准 |
| `wait` | 21 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 31 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/setup.rs

[reconstruction/rhapsody-rebuild/src/raw/setup.rs](../../reconstruction/rhapsody-rebuild/src/raw/setup.rs)

SHA256：`9a25689ce2732c1db5033f52bd223ba4caea125c8d52bca8098832e91cd7c477`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `with_setup_cb` | 20 | 未由扫描证明；以证据限定范围为准 |
| `upload_size` | 58 | 未由扫描证明；以证据限定范围为准 |
| `upload` | 74 | 未由扫描证明；以证据限定范围为准 |
| `init_texture_data` | 148 | 未由扫描证明；以证据限定范围为准 |
| `update_texture` | 157 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/shader.rs

[reconstruction/rhapsody-rebuild/src/raw/shader.rs](../../reconstruction/rhapsody-rebuild/src/raw/shader.rs)

SHA256：`53cf46557b2ba24afb4522feed201188701e59a9571c3f8040d5b303c8f30822`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_spirv` | 23 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 35 | 未由扫描证明；以证据限定范围为准 |
| `new` | 49 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 100 | 未由扫描证明；以证据限定范围为准 |
| `slot_for` | 121 | 未由扫描证明；以证据限定范围为准 |
| `new` | 128 | 未由扫描证明；以证据限定范围为准 |
| `data` | 139 | 未由扫描证明；以证据限定范围为准 |
| `invalidate` | 144 | 未由扫描证明；以证据限定范围为准 |
| `cached_compute_count` | 152 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 161 | 未由扫描证明；以证据限定范围为准 |
| `new` | 172 | 未由扫描证明；以证据限定范围为准 |
| `new` | 206 | 未由扫描证明；以证据限定范围为准 |
| `new` | 251 | 未由扫描证明；以证据限定范围为准 |
| `topology` | 254 | 未由扫描证明；以证据限定范围为准 |
| `topology_or_err` | 257 | 未由扫描证明；以证据限定范围为准 |
| `set_topology` | 260 | 未由扫描证明；以证据限定范围为准 |
| `set_topology_checked` | 263 | 未由扫描证明；以证据限定范围为准 |
| `polygon_mode` | 275 | 未由扫描证明；以证据限定范围为准 |
| `set_polygon_mode` | 278 | 未由扫描证明；以证据限定范围为准 |
| `set_polygon_mode_checked` | 281 | 未由扫描证明；以证据限定范围为准 |
| `cull_mode` | 288 | 未由扫描证明；以证据限定范围为准 |
| `cull_mode_or_err` | 291 | 未由扫描证明；以证据限定范围为准 |
| `set_cull_mode` | 294 | 未由扫描证明；以证据限定范围为准 |
| `set_cull_mode_checked` | 297 | 未由扫描证明；以证据限定范围为准 |
| `depth_compare_mode` | 309 | 未由扫描证明；以证据限定范围为准 |
| `depth_compare_mode_or_err` | 312 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_compare_mode` | 315 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_compare_mode_checked` | 318 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_write` | 322 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_write_checked` | 325 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_test_enable` | 329 | 未由扫描证明；以证据限定范围为准 |
| `set_depth_test_enable_checked` | 332 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_test_enable` | 336 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_test_enable_checked` | 339 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_pass_op` | 343 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_pass_op_checked` | 346 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_compare_op` | 356 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_compare_op_checked` | 359 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_compare_mask` | 363 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_compare_mask_checked` | 366 | 未由扫描证明；以证据限定范围为准 |
| `set_blend_mode` | 371 | 未由扫描证明；以证据限定范围为准 |
| `set_blend_mode_checked` | 374 | 未由扫描证明；以证据限定范围为准 |
| `apply_to` | 379 | 未由扫描证明；以证据限定范围为准 |
| `create_compute_pipeline` | 418 | 未由扫描证明；以证据限定范围为准 |
| `create_raster_pipeline` | 463 | 未由扫描证明；以证据限定范围为准 |
| `bind_pipeline` | 551 | 未由扫描证明；以证据限定范围为准 |
| `shader_count` | 558 | 未由扫描证明；以证据限定范围为准 |
| `retains_render_pass` | 561 | 未由扫描证明；以证据限定范围为准 |
| `stage` | 565 | 未由扫描证明；以证据限定范围为准 |
| `finish_pipeline` | 575 | 未由扫描证明；以证据限定范围为准 |
| `raster_pipeline_key` | 593 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 655 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 708 | 未由扫描证明；以证据限定范围为准 |
| `reflected_stage` | 719 | 未由扫描证明；以证据限定范围为准 |
| `from_reflection` | 728 | 未由扫描证明；以证据限定范围为准 |
| `create_descriptor_set_layouts` | 884 | 未由扫描证明；以证据限定范围为准 |
| `create_descriptor_set_states` | 900 | 未由扫描证明；以证据限定范围为准 |
| `create_pipeline_layout` | 909 | 未由扫描证明；以证据限定范围为准 |
| `canonical_type` | 919 | 未由扫描证明；以证据限定范围为准 |
| `get_vk_blend_mode` | 937 | 未由扫描证明；以证据限定范围为准 |
| `map_vertex_attribute_format` | 953 | 未由扫描证明；以证据限定范围为准 |
| `from_compiled_container` | 976 | 未由扫描证明；以证据限定范围为准 |
| `pipeline_desc` | 1045 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1062 | 未由扫描证明；以证据限定范围为准 |
| `register` | 1071 | 未由扫描证明；以证据限定范围为准 |
| `create_compute_pipeline` | 1082 | 未由扫描证明；以证据限定范围为准 |
| `invalidate` | 1146 | 未由扫描证明；以证据限定范围为准 |
| `prepare_raster_program` | 1153 | 未由扫描证明；以证据限定范围为准 |
| `reflect_spirv_entries` | 1214 | 未由扫描证明；以证据限定范围为准 |
| `reflected_type_json` | 1264 | 未由扫描证明；以证据限定范围为准 |
| `compiled_pipeline_entries` | 1279 | 未由扫描证明；以证据限定范围为准 |
| `from_compiled_pipeline` | 1297 | 未由扫描证明；以证据限定范围为准 |
| `serialized_stage_name` | 1344 | 未由扫描证明；以证据限定范围为准 |
| `stored_pipeline_entry` | 1351 | 未由扫描证明；以证据限定范围为准 |
| `from_compiled_pipeline_stage` | 1397 | 未由扫描证明；以证据限定范围为准 |
| `register_compiled_compute` | 1410 | 未由扫描证明；以证据限定范围为准 |
| `register_compiled_raster` | 1420 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/stats.rs

[reconstruction/rhapsody-rebuild/src/raw/stats.rs](../../reconstruction/rhapsody-rebuild/src/raw/stats.rs)

SHA256：`1e5095d5f37fe1c971eadd081ab64577f315cf2af9464d23669b5efe40fe2c64`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `snapshot_and_reset` | 18 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/suballoc.rs

[reconstruction/rhapsody-rebuild/src/raw/suballoc.rs](../../reconstruction/rhapsody-rebuild/src/raw/suballoc.rs)

SHA256：`f7145f7ea352593d41e9d889a62fe877e2159df84041287ece68930f729725f0`；缺口注释标记数：2。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `set_default_allocation_mode` | 37 | 未由扫描证明；以证据限定范围为准 |
| `default_allocation_mode` | 44 | 未由扫描证明；以证据限定范围为准 |
| `new` | 62 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 87 | 未由扫描证明；以证据限定范围为准 |
| `free` | 102 | 未由扫描证明；以证据限定范围为准 |
| `block_count` | 110 | 未由扫描证明；以证据限定范围为准 |
| `shutdown` | 117 | 未由扫描证明；以证据限定范围为准 |
| `location_from_flags` | 124 | 未由扫描证明；以证据限定范围为准 |
| `locate_memory_type` | 143 | 未由扫描证明；以证据限定范围为准 |
| `round_up` | 158 | 未由扫描证明；以证据限定范围为准 |
| `allocate_sub` | 165 | 未由扫描证明；以证据限定范围为准 |
| `suballocation_enabled` | 226 | 未由扫描证明；以证据限定范围为准 |
| `free_suballocation` | 229 | 未由扫描证明；以证据限定范围为准 |
| `memory_block_count` | 235 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 250 | 未由扫描证明；以证据限定范围为准 |
| `free` | 266 | 未由扫描证明；以证据限定范围为准 |
| `report` | 273 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L138：L::Unknown
- L252：let linear = !matches!(request.location, MemoryLocation::Unknown);
### reconstruction/rhapsody-rebuild/src/raw/submit.rs

[reconstruction/rhapsody-rebuild/src/raw/submit.rs](../../reconstruction/rhapsody-rebuild/src/raw/submit.rs)

SHA256：`31b046c793883ba3197ec06e5d579c23e4dc5e21c670c6c75d0c966e004dd5f1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `packed` | 13 | 未由扫描证明；以证据限定范围为准 |
| `new` | 87 | 未由扫描证明；以证据限定范围为准 |
| `command_buffer` | 90 | 未由扫描证明；以证据限定范围为准 |
| `command_buffers` | 94 | 未由扫描证明；以证据限定范围为准 |
| `wait_semaphore` | 98 | 未由扫描证明；以证据限定范围为准 |
| `signal_semaphore` | 102 | 未由扫描证明；以证据限定范围为准 |
| `with_wait_stages` | 107 | 未由扫描证明；以证据限定范围为准 |
| `with_wait_values` | 111 | 未由扫描证明；以证据限定范围为准 |
| `with_signal_values` | 115 | 未由扫描证明；以证据限定范围为准 |
| `with_vk` | 120 | 未由扫描证明；以证据限定范围为准 |
| `carve_off_front` | 135 | 未由扫描证明；以证据限定范围为准 |
| `with_vk` | 140 | 未由扫描证明；以证据限定范围为准 |
| `selected` | 166 | 未由扫描证明；以证据限定范围为准 |
| `queue_submit` | 174 | 未由扫描证明；以证据限定范围为准 |
| `new` | 269 | 未由扫描证明；以证据限定范围为准 |
| `queue` | 272 | 未由扫描证明；以证据限定范围为准 |
| `queue_mut` | 279 | 未由扫描证明；以证据限定范围为准 |
| `update_queue_pending_submissions` | 287 | 未由扫描证明；以证据限定范围为准 |
| `update_pending_submission_timestamp` | 302 | 未由扫描证明；以证据限定范围为准 |
| `reclaim_command_buffer_resources_all_queues` | 311 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/swapchain.rs

[reconstruction/rhapsody-rebuild/src/raw/swapchain.rs](../../reconstruction/rhapsody-rebuild/src/raw/swapchain.rs)

SHA256：`e455e3de745a4e7b3fc3f92adef9256c627b1bd04f7690710ab320b16112edd0`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `acquire_result` | 25 | 未由扫描证明；以证据限定范围为准 |
| `present_result` | 41 | 未由扫描证明；以证据限定范围为准 |
| `from_raw` | 58 | 未由扫描证明；以证据限定范围为准 |
| `from_win32` | 73 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 87 | 未由扫描证明；以证据限定范围为准 |
| `any_present_successful` | 113 | 未由扫描证明；以证据限定范围为准 |
| `new` | 118 | 未由扫描证明；以证据限定范围为准 |
| `new_with_old` | 136 | 未由扫描证明；以证据限定范围为准 |
| `enumerate_surface_formats` | 183 | 未由扫描证明；以证据限定范围为准 |
| `recreate` | 190 | 未由扫描证明；以证据限定范围为准 |
| `resize` | 336 | 未由扫描证明；以证据限定范围为准 |
| `set_vsync` | 346 | 未由扫描证明；以证据限定范围为准 |
| `desc` | 356 | 未由扫描证明；以证据限定范围为准 |
| `replacement` | 360 | 未由扫描证明；以证据限定范围为准 |
| `validate_swapchain_and_acquire_image` | 375 | 未由扫描证明；以证据限定范围为准 |
| `mark_stale` | 387 | 未由扫描证明；以证据限定范围为准 |
| `is_stale` | 390 | 未由扫描证明；以证据限定范围为准 |
| `on_focus` | 393 | 未由扫描证明；以证据限定范围为准 |
| `acquire_next_image` | 396 | 未由扫描证明；以证据限定范围为准 |
| `acquire_frame` | 420 | 未由扫描证明；以证据限定范围为准 |
| `present` | 448 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 472 | 未由扫描证明；以证据限定范围为准 |
| `image` | 496 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 499 | 未由扫描证明；以证据限定范围为准 |
| `present` | 502 | 未由扫描证明；以证据限定范围为准 |
| `new` | 524 | 未由扫描证明；以证据限定范围为准 |
| `reserve` | 556 | 未由扫描证明；以证据限定范围为准 |
| `release` | 581 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 588 | 未由扫描证明；以证据限定范围为准 |
| `image` | 617 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 620 | 未由扫描证明；以证据限定范围为准 |
| `index` | 623 | 未由扫描证明；以证据限定范围为准 |
| `slot_index` | 626 | 未由扫描证明；以证据限定范围为准 |
| `acquire_semaphore` | 629 | 未由扫描证明；以证据限定范围为准 |
| `render_semaphore` | 632 | 未由扫描证明；以证据限定范围为准 |
| `submission_fence` | 635 | 未由扫描证明；以证据限定范围为准 |
| `extent` | 638 | 未由扫描证明；以证据限定范围为准 |
| `submit_and_present` | 643 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 707 | 未由扫描证明；以证据限定范围为准 |
| `new` | 720 | 未由扫描证明；以证据限定范围为准 |
| `into_sync_data` | 723 | 未由扫描证明；以证据限定范围为准 |
| `frame` | 730 | 未由扫描证明；以证据限定范围为准 |
| `into_frame` | 733 | 未由扫描证明；以证据限定范围为准 |
| `new` | 744 | 未由扫描证明；以证据限定范围为准 |
| `image` | 747 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 750 | 未由扫描证明；以证据限定范围为准 |
| `index` | 753 | 未由扫描证明；以证据限定范围为准 |
| `into_present_source` | 756 | 未由扫描证明；以证据限定范围为准 |
| `wait` | 769 | 未由扫描证明；以证据限定范围为准 |
| `is_complete` | 779 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 784 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/temp_list.rs

[reconstruction/rhapsody-rebuild/src/raw/temp_list.rs](../../reconstruction/rhapsody-rebuild/src/raw/temp_list.rs)

SHA256：`73814940330422b27461d1b793d779c4de7daf6edd04e11b93a22c2efd7aaf34`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 7 | 未由扫描证明；以证据限定范围为准 |
| `add` | 10 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 14 | 未由扫描证明；以证据限定范围为准 |
| `as_slice` | 17 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/texture.rs

[reconstruction/rhapsody-rebuild/src/raw/texture.rs](../../reconstruction/rhapsody-rebuild/src/raw/texture.rs)

SHA256：`060e4baf93b86cb385942c4d48a5ab6b69f5d66bafd06bd124e2caceae4a4aca`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `mip_count_1d` | 12 | 未由扫描证明；以证据限定范围为准 |
| `to_vk` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new` | 49 | 未由扫描证明；以证据限定范围为准 |
| `next` | 58 | 未由扫描证明；以证据限定范围为准 |
| `get_image_create_info` | 70 | 未由扫描证明；以证据限定范围为准 |
| `new_2d` | 97 | 未由扫描证明；以证据限定范围为准 |
| `new` | 114 | 未由扫描证明；以证据限定范围为准 |
| `array_elements` | 131 | 未由扫描证明；以证据限定范围为准 |
| `sample_count` | 134 | 未由扫描证明；以证据限定范围为准 |
| `div_up_extent` | 138 | 未由扫描证明；以证据限定范围为准 |
| `div_extent` | 157 | 未由扫描证明；以证据限定范围为准 |
| `full_mip_chain_length` | 177 | 未由扫描证明；以证据限定范围为准 |
| `sampled_with_different_format` | 187 | 未由扫描证明；以证据限定范围为准 |
| `new_2d_array` | 191 | 未由扫描证明；以证据限定范围为准 |
| `new_3d` | 197 | 未由扫描证明；以证据限定范围为准 |
| `usage` | 208 | 未由扫描证明；以证据限定范围为准 |
| `sampled` | 212 | 未由扫描证明；以证据限定范围为准 |
| `storage` | 216 | 未由扫描证明；以证据限定范围为准 |
| `transfer_dst` | 220 | 未由扫描证明；以证据限定范围为准 |
| `color_attachment` | 224 | 未由扫描证明；以证据限定范围为准 |
| `depth_stencil_attachment` | 228 | 未由扫描证明；以证据限定范围为准 |
| `all_mip_levels` | 232 | 未由扫描证明；以证据限定范围为准 |
| `extent_2d` | 242 | 未由扫描证明；以证据限定范围为准 |
| `half_res` | 248 | 未由扫描证明；以证据限定范围为准 |
| `new` | 266 | 未由扫描证明；以证据限定范围为准 |
| `bind_individual_mips` | 285 | 未由扫描证明；以证据限定范围为准 |
| `usage` | 288 | 未由扫描证明；以证据限定范围为准 |
| `new` | 299 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 342 | 未由扫描证明；以证据限定范围为准 |
| `allocation_bytes` | 374 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 379 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 392 | 未由扫描证明；以证据限定范围为准 |
| `new` | 403 | 未由扫描证明；以证据限定范围为准 |
| `next` | 412 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/texture_upload.rs

[reconstruction/rhapsody-rebuild/src/raw/texture_upload.rs](../../reconstruction/rhapsody-rebuild/src/raw/texture_upload.rs)

SHA256：`78dd9514df83a4744709308f036c46c4a36ca32ada1356074de69fec40a387fe`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `validate_texture_regions` | 12 | 未由扫描证明；以证据限定范围为准 |
| `record_texture_regions` | 113 | 未由扫描证明；以证据限定范围为准 |
