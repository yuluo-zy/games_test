# crates/rhapsody/src/raw/managed

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

handle/ref/generation/checksum、资源持有/CPU退役/GPU保护与managed资源包装。

## 接线与验证

接线：候选descriptor持有与GPU消费已接，provider主图桥仍两阶段。

限定原生命周期合同和候选测试/GPU链；新增接口不是原完整ABI恢复。

## 已恢复的函数和类型范围

- GpuOwned全族、ManagedHandle/ManagedRef
- ManagedResourceMeta/checksum/owning_device/get_maybe_tracked
- managed buffer/texture/view/swapchain包装

## 明确缺口

- 原全域producer/序号桶、真实游戏持有关系与性能
- raw析构保守等待、全ABI和原complete disposal

下一闭包：原全域producer/序号桶、真实游戏持有关系与性能

## 证据

- [reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)
- [reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json](../../reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json)
- [reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json)
- [reconstruction/rhapsody-rebuild/evidence/managed-implementation.json](../../reconstruction/rhapsody-rebuild/evidence/managed-implementation.json)
- [reconstruction/rhapsody-rebuild/evidence/pass-bindings-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/pass-bindings-functional-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/src/raw/managed/buffer.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/buffer.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/descriptor_set.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/descriptor_set.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/descriptor_set.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/framebuffer.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/framebuffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/framebuffer.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/handle.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/render_pass.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/render_pass.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/render_pass.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/resources.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/sampler.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/sampler.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/sampler.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/swapchain.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/raw/managed/texture.rs` | A,B | [reconstruction/rhapsody-rebuild/src/raw/managed/texture.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/texture.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/raw/managed/buffer.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/buffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/buffer.rs)

SHA256：`30b201acb8fc4015656e211caaf95ee2f1edfb6c8fdae9c924056a57fb206444`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_buffer` | 12 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 19 | 未由扫描证明；以证据限定范围为准 |
| `allocation` | 23 | 未由扫描证明；以证据限定范围为准 |
| `mapped_slice_mut` | 28 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/descriptor_set.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/descriptor_set.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/descriptor_set.rs)

SHA256：`555864e424c784e1117053b92f5312df25b2974e436fd709bc4722d66bfd5509`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `allocate_descriptor_set` | 11 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 18 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/framebuffer.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/framebuffer.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/framebuffer.rs)

SHA256：`01697d386c5c8f36e8967856a2bfec31e912cfb59b09630daaf603b082465c1b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_framebuffer` | 10 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 19 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/handle.rs)

SHA256：`32bbad607ed4e48624397a3ec4b8df15efa64b629326410eb3e68061011f0fd2`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `key` | 22 | 未由扫描证明；以证据限定范围为准 |
| `track_on` | 26 | 未由扫描证明；以证据限定范围为准 |
| `track` | 32 | 未由扫描证明；以证据限定范围为准 |
| `tracked_sequence` | 35 | 未由扫描证明；以证据限定范围为准 |
| `tracked_commands` | 39 | 未由扫描证明；以证据限定范围为准 |
| `reference_count` | 50 | 未由扫描证明；以证据限定范围为准 |
| `resource` | 53 | 未由扫描证明；以证据限定范围为准 |
| `clone` | 58 | 未由扫描证明；以证据限定范围为准 |
| `deref` | 65 | 未由扫描证明；以证据限定范围为准 |
| `eq` | 70 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 76 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 84 | 未由扫描证明；以证据限定范围为准 |
| `invalidate` | 98 | 未由扫描证明；以证据限定范围为准 |
| `value` | 101 | 未由扫描证明；以证据限定范围为准 |
| `new` | 112 | 未由扫描证明；以证据限定范围为准 |
| `resource` | 115 | 未由扫描证明；以证据限定范围为准 |
| `from_managed` | 126 | 未由扫描证明；以证据限定范围为准 |
| `into_gpu_owned` | 129 | 未由扫描证明；以证据限定范围为准 |
| `into_raw_gpu_owned` | 132 | 未由扫描证明；以证据限定范围为准 |
| `as_managed_handle` | 135 | 未由扫描证明；以证据限定范围为准 |
| `new` | 146 | 未由扫描证明；以证据限定范围为准 |
| `map` | 149 | 未由扫描证明；以证据限定范围为准 |
| `map_tracker` | 153 | 未由扫描证明；以证据限定范围为准 |
| `track_unwrap` | 157 | 未由扫描证明；以证据限定范围为准 |
| `mark_unused` | 161 | 未由扫描证明；以证据限定范围为准 |
| `drop_unused` | 165 | 未由扫描证明；以证据限定范围为准 |
| `drop_duplicate` | 169 | 未由扫描证明；以证据限定范围为准 |
| `new` | 180 | 未由扫描证明；以证据限定范围为准 |
| `as_owner` | 183 | 未由扫描证明；以证据限定范围为准 |
| `into_owner` | 186 | 未由扫描证明；以证据限定范围为准 |
| `as_managed_handle` | 189 | 未由扫描证明；以证据限定范围为准 |
| `get_untracked` | 192 | 未由扫描证明；以证据限定范围为准 |
| `get_tracked` | 195 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/lifecycle.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/lifecycle.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/lifecycle.rs)

SHA256：`e0d256c825ab96e9e687eee8b7c2ef2c5f933d52d6bd76f1175db5e8d5822438`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raw` | 17 | 未由扫描证明；以证据限定范围为准 |
| `producer` | 23 | 未由扫描证明；以证据限定范围为准 |
| `from_raw` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new` | 46 | 未由扫描证明；以证据限定范围为准 |
| `state` | 52 | 未由扫描证明；以证据限定范围为准 |
| `set_state` | 55 | 未由扫描证明；以证据限定范围为准 |
| `sequence` | 58 | 未由扫描证明；以证据限定范围为准 |
| `queue` | 61 | 未由扫描证明；以证据限定范围为准 |
| `handle` | 67 | 未由扫描证明；以证据限定范围为准 |
| `pool_family` | 70 | 未由扫描证明；以证据限定范围为准 |
| `record_command` | 74 | 未由扫描证明；以证据限定范围为准 |
| `caller_snapshot` | 81 | 未由扫描证明；以证据限定范围为准 |
| `from_caller_snapshot` | 84 | 未由扫描证明；以证据限定范围为准 |
| `new` | 153 | 未由扫描证明；以证据限定范围为准 |
| `begin_after_backend` | 165 | 未由扫描证明；以证据限定范围为准 |
| `accept_begun` | 203 | 未由扫描证明；以证据限定范围为准 |
| `create_resource` | 219 | 未由扫描证明；以证据限定范围为准 |
| `track` | 229 | 未由扫描证明；以证据限定范围为准 |
| `import_record_tracker` | 252 | 未由扫描证明；以证据限定范围为准 |
| `cpu_retire` | 267 | 未由扫描证明；以证据限定范围为准 |
| `clone_handle` | 270 | 未由扫描证明；以证据限定范围为准 |
| `drop_handle` | 278 | 未由扫描证明；以证据限定范围为准 |
| `collect_waste` | 296 | 未由扫描证明；以证据限定范围为准 |
| `accept_submission` | 311 | 未由扫描证明；以证据限定范围为准 |
| `accept_queue_success` | 324 | 未由扫描证明；以证据限定范围为准 |
| `observe_timeline` | 345 | 未由扫描证明；以证据限定范围为准 |
| `drain_bucket` | 355 | 未由扫描证明；以证据限定范围为准 |
| `dispose_sequence` | 366 | 未由扫描证明；以证据限定范围为准 |
| `reclaim_ready` | 403 | 未由扫描证明；以证据限定范围为准 |
| `recycle` | 432 | 未由扫描证明；以证据限定范围为准 |
| `prefix` | 435 | 未由扫描证明；以证据限定范围为准 |
| `ready_markers` | 438 | 未由扫描证明；以证据限定范围为准 |
| `pending_len` | 443 | 未由扫描证明；以证据限定范围为准 |
| `pool_sequences` | 446 | 未由扫描证明；以证据限定范围为准 |
| `bucket_keys` | 452 | 未由扫描证明；以证据限定范围为准 |
| `resource_state` | 455 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/mod.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/mod.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/mod.rs)

SHA256：`3e5d4597b786df72f8027c9de2b02d377d72535ce97cda3bcd6e07c60f3bd92f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/rhapsody-rebuild/src/raw/managed/render_pass.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/render_pass.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/render_pass.rs)

SHA256：`173cec85ff85964745fa7c2d1daad307a2aa9f05a6d125005b49aff19959d7cf`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_render_pass` | 8 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 15 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/resources.rs)

SHA256：`1859e31480f216358a4c58108ad79b01ed4536aeb1b1b1bff06f72c15d81e7dc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 40 | 未由扫描证明；以证据限定范围为准 |
| `checksum` | 43 | 未由扫描证明；以证据限定范围为准 |
| `get_maybe_tracked` | 51 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 60 | 未由扫描证明；以证据限定范围为准 |
| `prune` | 76 | 未由扫描证明；以证据限定范围为准 |
| `new` | 100 | 未由扫描证明；以证据限定范围为准 |
| `already_drained` | 110 | 未由扫描证明；以证据限定范围为准 |
| `take_out_dropped_waste` | 120 | 未由扫描证明；以证据限定范围为准 |
| `begin_command_bucket` | 142 | 未由扫描证明；以证据限定范围为准 |
| `drain_completed_sequence` | 147 | 未由扫描证明；以证据限定范围为准 |
| `record_submission` | 163 | 未由扫描证明；以证据限定范围为准 |
| `complete` | 204 | 未由扫描证明；以证据限定范围为准 |
| `slot` | 244 | 未由扫描证明；以证据限定范围为准 |
| `enqueue_retirement` | 249 | 未由扫描证明；以证据限定范围为准 |
| `clone` | 310 | 未由扫描证明；以证据限定范围为准 |
| `default` | 317 | 未由扫描证明；以证据限定范围为准 |
| `new` | 322 | 未由扫描证明；以证据限定范围为准 |
| `with_shared` | 351 | 未由扫描证明；以证据限定范围为准 |
| `shared` | 362 | 未由扫描证明；以证据限定范围为准 |
| `allocate` | 365 | 未由扫描证明；以证据限定范围为准 |
| `allocate_shared` | 368 | 未由扫描证明；以证据限定范围为准 |
| `get` | 412 | 未由扫描证明；以证据限定范围为准 |
| `get_tracked` | 428 | 未由扫描证明；以证据限定范围为准 |
| `get_maybe_tracked` | 439 | 未由扫描证明；以证据限定范围为准 |
| `begin_command_bucket` | 447 | 未由扫描证明；以证据限定范围为准 |
| `drain_completed_sequence` | 450 | 未由扫描证明；以证据限定范围为准 |
| `record_submission` | 453 | 未由扫描证明；以证据限定范围为准 |
| `complete` | 461 | 未由扫描证明；以证据限定范围为准 |
| `retained_submission_count` | 464 | 未由扫描证明；以证据限定范围为准 |
| `pending_disposals` | 471 | 未由扫描证明；以证据限定范围为准 |
| `pending_destroy` | 481 | 未由扫描证明；以证据限定范围为准 |
| `generation_at` | 487 | 未由扫描证明；以证据限定范围为准 |
| `take_out_dropped_waste_checked` | 490 | 未由扫描证明；以证据限定范围为准 |
| `take_out_dropped_waste` | 494 | 未由扫描证明；以证据限定范围为准 |
| `dispose_after_wait` | 498 | 未由扫描证明；以证据限定范围为准 |
| `default` | 532 | 未由扫描证明；以证据限定范围为准 |
| `new` | 548 | 未由扫描证明；以证据限定范围为准 |
| `set_device` | 552 | 未由扫描证明；以证据限定范围为准 |
| `owning_device` | 556 | 未由扫描证明；以证据限定范围为准 |
| `throw_out_managed_resource` | 560 | 未由扫描证明；以证据限定范围为准 |
| `begin_command_bucket` | 573 | 未由扫描证明；以证据限定范围为准 |
| `drain_completed_sequence` | 576 | 未由扫描证明；以证据限定范围为准 |
| `record_submission` | 579 | 未由扫描证明；以证据限定范围为准 |
| `complete` | 587 | 未由扫描证明；以证据限定范围为准 |
| `take_out_dropped_waste_checked` | 590 | 未由扫描证明；以证据限定范围为准 |
| `take_out_dropped_waste` | 593 | 未由扫描证明；以证据限定范围为准 |
| `pending_disposals` | 597 | 未由扫描证明；以证据限定范围为准 |
| `pending_destroy` | 603 | 未由扫描证明；以证据限定范围为准 |
| `get_maybe_tracked` | 613 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/sampler.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/sampler.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/sampler.rs)

SHA256：`81e76be4d96a8a97ee54441623a793af5f2d3e46c8b2424e433003ab0d95f2a6`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_sampler` | 8 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 15 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/swapchain.rs)

SHA256：`7133a023c3e8e4ff0857947f28ed9698f83f2bfac6ce1ddca329f8f7da1dae2a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `manage_swapchain` | 13 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 16 | 未由扫描证明；以证据限定范围为准 |
| `desc` | 19 | 未由扫描证明；以证据限定范围为准 |
| `resize` | 22 | 未由扫描证明；以证据限定范围为准 |
| `set_vsync` | 25 | 未由扫描证明；以证据限定范围为准 |
| `acquire_next` | 28 | 未由扫描证明；以证据限定范围为准 |
| `on_focus` | 32 | 未由扫描证明；以证据限定范围为准 |
| `present` | 36 | 未由扫描证明；以证据限定范围为准 |
| `retire` | 40 | 未由扫描证明；以证据限定范围为准 |
| `mark_stale` | 43 | 未由扫描证明；以证据限定范围为准 |
| `is_stale` | 46 | 未由扫描证明；以证据限定范围为准 |
| `validate_swapchain_and_acquire_image` | 49 | 未由扫描证明；以证据限定范围为准 |
| `new` | 72 | 未由扫描证明；以证据限定范围为准 |
| `new_impl` | 75 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 78 | 未由扫描证明；以证据限定范围为准 |
| `acquire_next_image` | 81 | 未由扫描证明；以证据限定范围为准 |
| `present_image` | 84 | 未由扫描证明；以证据限定范围为准 |
| `on_focus` | 87 | 未由扫描证明；以证据限定范围为准 |
| `resize` | 90 | 未由扫描证明；以证据限定范围为准 |
| `set_vsync` | 93 | 未由扫描证明；以证据限定范围为准 |
| `mark_stale` | 96 | 未由扫描证明；以证据限定范围为准 |
| `retire` | 99 | 未由扫描证明；以证据限定范围为准 |
| `set_fullscreen_exclusive` | 102 | 未由扫描证明；以证据限定范围为准 |
| `dispose` | 105 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/texture.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/texture.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/texture.rs)

SHA256：`5d4d86a4e6c5e1e429d33011f47cc745197e4dcff0128ef41d47488fec80f4ae`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_texture` | 12 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 19 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 22 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 38 | 未由扫描证明；以证据限定范围为准 |
| `track_on` | 41 | 未由扫描证明；以证据限定范围为准 |
| `convert_image_type_to_view_type` | 51 | 未由扫描证明；以证据限定范围为准 |
| `view_desc_impl` | 65 | 未由扫描证明；以证据限定范围为准 |
| `create_vk_image_view` | 70 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/raw/managed/tracking.rs

[reconstruction/rhapsody-rebuild/src/raw/managed/tracking.rs](../../reconstruction/rhapsody-rebuild/src/raw/managed/tracking.rs)

SHA256：`b63099bbe70fb31d2ab5c80e673e677ad8f5485b814fd06cd3a8a5f541ca1bec`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 13 | 未由扫描证明；以证据限定范围为准 |
| `checksum` | 20 | 未由扫描证明；以证据限定范围为准 |
| `track_sequence` | 53 | 未由扫描证明；以证据限定范围为准 |
| `track_handle` | 61 | 未由扫描证明；以证据限定范围为准 |
| `clone_reference` | 73 | 未由扫描证明；以证据限定范围为准 |
| `clone_if_live` | 85 | 未由扫描证明；以证据限定范围为准 |
| `drop_reference` | 102 | 未由扫描证明；以证据限定范围为准 |
| `dispose_control` | 113 | 未由扫描证明；以证据限定范围为准 |
| `dispose_control_kind` | 137 | 未由扫描证明；以证据限定范围为准 |
