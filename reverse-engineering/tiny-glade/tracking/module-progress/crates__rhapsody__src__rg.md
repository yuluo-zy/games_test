# crates/rhapsody/src/rg

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

graph/compiled/registry/pool、访问分析、barrier/record、executor/temporal/import-export/pass API。

## 接线与验证

接线：rg候选到Vulkan串行功能链；旧context入口未自动迁入新工程。

原CPU合同/叶函数限定差分；候选串行session/temporal GPU已运行；全图与原整帧未验收。

## 已恢复的函数和类型范围

- ResourceInfo::calculate与用途转换
- RenderGraph/BatchCompiler/CompiledGraph/Registry/Pool
- RenderPassApi draw/dispatch九命令、recorder、executor
- Parity/Temporal/import_export/ExecutedRenderGraph/PGO
- ExecutionChain::execute_graph(_with_context)旧共享入口

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| ResourceInfo::link_up_next_access_chains | [reconstruction/render/resource_info.rs](../../reconstruction/render/resource_info.rs)：`link_up_next_access_chains` | B：原0x140bd47d0访问链规则；计数与样本边界见子报告；定义文件精确归属待核实 | reported_verified_local：[evidence/render-recovery/verification.json](../../evidence/render-recovery/verification.json), [evidence/render-recovery/findings.md](../../evidence/render-recovery/findings.md) |
| ResourceInfo::calculate_lifetime_events | [reconstruction/render/resource_info.rs](../../reconstruction/render/resource_info.rs)：`calculate_lifetime_events` | B：原0x140bd4860生命周期事件规则；非完整ResourceInfo::calculate/图编译；定义文件精确归属待核实 | reported_verified_local：[evidence/render-recovery/verification.json](../../evidence/render-recovery/verification.json), [evidence/render-recovery/findings.md](../../evidence/render-recovery/findings.md) |
| PassResourceAccessType::can_be_concurrent_with | [reconstruction/render/resource_info.rs](../../reconstruction/render/resource_info.rs)：`can_be_concurrent_with` | B：原0x140bece70并发访问判断；classifier为外供公开依赖边界，定义文件精确归属待核实 | reported_verified_local：[evidence/render-recovery/verification.json](../../evidence/render-recovery/verification.json), [evidence/render-recovery/findings.md](../../evidence/render-recovery/findings.md) |

## 明确缺口

- 首个shader callback自动注入真实Command/managed descriptor并同一submit
- 原并行/跨族ownership transfer、完整compile/record原入口
- temporal淘汰/reset/drop、原游戏consumer

下一闭包：将游戏pass的registry与正在录制的Command接入首个真实shader callback，完成同一提交/回收

## 证据

- [reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)
- [reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json](../../reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json)
- [reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json)
- [evidence/render-recovery/verification.json](../../evidence/render-recovery/verification.json)
- [evidence/render-recovery/consumer-chain-review-2026-10-12/context-entry-validation.json](../../evidence/render-recovery/consumer-chain-review-2026-10-12/context-entry-validation.json)
- [reconstruction/rhapsody-rebuild/evidence/queue-graph-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/queue-graph-functional-validation.json)
- [reconstruction/rhapsody-rebuild/evidence/temporal-functional-validation.json](../../reconstruction/rhapsody-rebuild/evidence/temporal-functional-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/src/rg/binding.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/binding.rs](../../reconstruction/rhapsody-rebuild/src/rg/binding.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/compiled.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/compiled.rs](../../reconstruction/rhapsody-rebuild/src/rg/compiled.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/executor.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/executor.rs](../../reconstruction/rhapsody-rebuild/src/rg/executor.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/graph.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/graph.rs](../../reconstruction/rhapsody-rebuild/src/rg/graph.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/import_export.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/import_export.rs](../../reconstruction/rhapsody-rebuild/src/rg/import_export.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/pass_api.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/pass_api.rs](../../reconstruction/rhapsody-rebuild/src/rg/pass_api.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/pass_builder.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/pass_builder.rs](../../reconstruction/rhapsody-rebuild/src/rg/pass_builder.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/recorder.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/recorder.rs](../../reconstruction/rhapsody-rebuild/src/rg/recorder.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/resource.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/resource.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/resource_info.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/resource_info.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_info.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/resource_pool.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/resource_pool.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_pool.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/resource_registry.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/resource_registry.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_registry.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/state.rs` | B | [reconstruction/rhapsody-rebuild/src/rg/state.rs](../../reconstruction/rhapsody-rebuild/src/rg/state.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/temporal.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/temporal.rs](../../reconstruction/rhapsody-rebuild/src/rg/temporal.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/rg/util.rs` | A,B | [reconstruction/rhapsody-rebuild/src/rg/util.rs](../../reconstruction/rhapsody-rebuild/src/rg/util.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/render/execution.rs

[reconstruction/render/execution.rs](../../reconstruction/render/execution.rs)

SHA256：`63432c913fab74f428a13892aef1a6b873b8ee491278d1595819ba05ea475388`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `managed_resource` | 39 | 未由扫描证明；以证据限定范围为准 |
| `command_handle` | 46 | 未由扫描证明；以证据限定范围为准 |
| `begin_command` | 47 | 未由扫描证明；以证据限定范围为准 |
| `end_command` | 48 | 未由扫描证明；以证据限定范围为准 |
| `submit` | 49 | 未由扫描证明；以证据限定范围为准 |
| `access` | 68 | 未由扫描证明；以证据限定范围为准 |
| `execute_graph` | 81 | 未由扫描证明；以证据限定范围为准 |
| `execute_graph_with_context` | 98 | 未由扫描证明；以证据限定范围为准 |
| `new` | 281 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 291 | 未由扫描证明；以证据限定范围为准 |
| `record` | 313 | 未由扫描证明；以证据限定范围为准 |
| `submit_recorded` | 346 | 未由扫描证明；以证据限定范围为准 |
| `complete` | 435 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/render/graph.rs

[reconstruction/render/graph.rs](../../reconstruction/render/graph.rs)

SHA256：`d96d7bff0e1000c88b3cd9790a100093a0ff2d633a27179573080e0fd1b13de8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `kind_code` | 23 | 未由扫描证明；以证据限定范围为准 |
| `usage` | 29 | 未由扫描证明；以证据限定范围为准 |
| `with_usage` | 35 | 未由扫描证明；以证据限定范围为准 |
| `info_origin` | 61 | 未由扫描证明；以证据限定范围为准 |
| `access_mask` | 102 | 未由扫描证明；以证据限定范围为准 |
| `is_write` | 103 | 未由扫描证明；以证据限定范围为准 |
| `create_without_initial_data` | 111 | 未由扫描证明；以证据限定范围为准 |
| `set_debug_name` | 116 | 未由扫描证明；以证据限定范围为准 |
| `create_without_initial_data` | 138 | 未由扫描证明；以证据限定范围为准 |
| `set_debug_name` | 162 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 179 | 未由扫描证明；以证据限定范围为准 |
| `populate_resources` | 191 | 未由扫描证明；以证据限定范围为准 |
| `transition` | 326 | 未由扫描证明；以证据限定范围为准 |
| `initial_transitions` | 345 | 未由扫描证明；以证据限定范围为准 |
| `pre_pass_transitions` | 369 | 未由扫描证明；以证据限定范围为准 |
| `export_transitions` | 410 | 未由扫描证明；以证据限定范围为准 |
| `into_exports` | 471 | 未由扫描证明；以证据限定范围为准 |
| `compile_graph` | 475 | 未由扫描证明；以证据限定范围为准 |
| `payload_projection` | 599 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/render/graph_info.rs

[reconstruction/render/graph_info.rs](../../reconstruction/render/graph_info.rs)

SHA256：`4ccb1696e8847711c18527a75a5b46b5ac58ae25a1ad76b01c74f6445d9d4d2e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 13 | 未由扫描证明；以证据限定范围为准 |
| `from` | 22 | 未由扫描证明；以证据限定范围为准 |
| `type_code` | 43 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 83 | 未由扫描证明；以证据限定范围为准 |
| `calculate` | 93 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/render/resource_info.rs

[reconstruction/render/resource_info.rs](../../reconstruction/render/resource_info.rs)

SHA256：`d951a9916d093c902eee52831e8844ce2b3add245eb710da2b37dc97650a7953`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `can_be_concurrent_with` | 57 | 未由扫描证明；以证据限定范围为准 |
| `link_up_next_access_chains` | 76 | 未由扫描证明；以证据限定范围为准 |
| `calculate_lifetime_events` | 91 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/render/resource_usage.rs

[reconstruction/render/resource_usage.rs](../../reconstruction/render/resource_usage.rs)

SHA256：`0cbe51104545c9b40b7c977ed3c38919f794299824e7b5642293fccc04f82897`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `buffer_access_mask_to_usage_flags` | 15 | 未由扫描证明；以证据限定范围为准 |
| `image_access_mask_to_usage_flags` | 32 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/access_ledger.rs

[reconstruction/rhapsody-rebuild/src/rg/access_ledger.rs](../../reconstruction/rhapsody-rebuild/src/rg/access_ledger.rs)

SHA256：`4e19383ce2e74665847dd6cdd122ebb7e819f265b4437e6f035238ce82869a3d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 17 | 未由扫描证明；以证据限定范围为准 |
| `from_raw` | 21 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 25 | 未由扫描证明；以证据限定范围为准 |
| `is_equivalent` | 30 | 未由扫描证明；以证据限定范围为准 |
| `default` | 36 | 未由扫描证明；以证据限定范围为准 |
| `allocate_group` | 41 | 未由扫描证明；以证据限定范围为准 |
| `declaration` | 56 | 未由扫描证明；以证据限定范围为准 |
| `collect_resource_access` | 86 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/attachments.rs

[reconstruction/rhapsody-rebuild/src/rg/attachments.rs](../../reconstruction/rhapsody-rebuild/src/rg/attachments.rs)

SHA256：`62a11966ddb34e2a166d481126f1031c958982c302b9497c99cc0921b57e02b5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `layout` | 33 | 未由扫描证明；以证据限定范围为准 |
| `new` | 44 | 未由扫描证明；以证据限定范围为准 |
| `clear_input` | 52 | 未由扫描证明；以证据限定范围为准 |
| `sample_count` | 56 | 未由扫描证明；以证据限定范围为准 |
| `color` | 78 | 未由扫描证明；以证据限定范围为准 |
| `depth` | 103 | 未由扫描证明；以证据限定范围为准 |
| `depth_access` | 117 | 未由扫描证明；以证据限定范围为准 |
| `render_key` | 123 | 未由扫描证明；以证据限定范围为准 |
| `view_key` | 137 | 未由扫描证明；以证据限定范围为准 |
| `begin_info` | 181 | 未由扫描证明；以证据限定范围为准 |
| `keep_alive` | 192 | 未由扫描证明；以证据限定范围为准 |
| `new` | 205 | 未由扫描证明；以证据限定范围为准 |
| `counts` | 221 | 未由扫描证明；以证据限定范围为准 |
| `prepare` | 229 | 未由扫描证明；以证据限定范围为准 |
| `validate` | 296 | 未由扫描证明；以证据限定范围为准 |
| `invalidate_image` | 412 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 421 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/binding.rs

[reconstruction/rhapsody-rebuild/src/rg/binding.rs](../../reconstruction/rhapsody-rebuild/src/rg/binding.rs)

SHA256：`6aeced9b309e13b6082c1fac1d31372fa44735b84671fef3a38a8b9609ccacb3`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `uniform_buffer` | 28 | 未由扫描证明；以证据限定范围为准 |
| `storage_buffer` | 35 | 未由扫描证明；以证据限定范围为准 |
| `sampled_image` | 42 | 未由扫描证明；以证据限定范围为准 |
| `storage_image` | 45 | 未由扫描证明；以证据限定范围为准 |
| `rg_binding_to_descriptor_binding` | 64 | 未由扫描证明；以证据限定范围为准 |
| `create_descriptor_writes` | 121 | 未由扫描证明；以证据限定范围为准 |
| `bind_descriptor_set` | 190 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/compiled.rs

[reconstruction/rhapsody-rebuild/src/rg/compiled.rs](../../reconstruction/rhapsody-rebuild/src/rg/compiled.rs)

SHA256：`d61000d43212a33d0ff651d9a76c288663cbc2d49b38f8ccf22115a5bf508a32`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 21 | 未由扫描证明；以证据限定范围为准 |
| `flush_batch` | 25 | 未由扫描证明；以证据限定范围为准 |
| `finish` | 39 | 未由扫描证明；以证据限定范围为准 |
| `pending_initial_transition_resources` | 54 | 未由扫描证明；以证据限定范围为准 |
| `pre_pass_barrier_transitions` | 84 | 未由扫描证明；以证据限定范围为准 |
| `new` | 124 | 未由扫描证明；以证据限定范围为准 |
| `take_initial_transitions` | 138 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/executor.rs

[reconstruction/rhapsody-rebuild/src/rg/executor.rs](../../reconstruction/rhapsody-rebuild/src/rg/executor.rs)

SHA256：`81a1bf89bcc1ab9661e387e94258fc207676bd84580deb92bab24a62d0765c8f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `valid` | 93 | 未由扫描证明；以证据限定范围为准 |
| `record_transitions` | 128 | 未由扫描证明；以证据限定范围为准 |
| `count_barriers` | 149 | 未由扫描证明；以证据限定范围为准 |
| `prepare_compiled_resources` | 162 | 未由扫描证明；以证据限定范围为准 |
| `managed_pass_resources` | 166 | 未由扫描证明；以证据限定范围为准 |
| `command_started` | 171 | 未由扫描证明；以证据限定范围为准 |
| `retain_command_owners` | 174 | 未由扫描证明；以证据限定范围为准 |
| `collect_managed_waste` | 181 | 未由扫描证明；以证据限定范围为准 |
| `graph_queue` | 184 | 未由扫描证明；以证据限定范围为准 |
| `batch_synchronization` | 187 | 未由扫描证明；以证据限定范围为准 |
| `managed_resource` | 191 | 未由扫描证明；以证据限定范围为准 |
| `command_handle` | 197 | 未由扫描证明；以证据限定范围为准 |
| `chunk_command_handle` | 199 | 未由扫描证明；以证据限定范围为准 |
| `begin_command` | 202 | 未由扫描证明；以证据限定范围为准 |
| `end_command` | 203 | 未由扫描证明；以证据限定范围为准 |
| `transition` | 205 | 未由扫描证明；以证据限定范围为准 |
| `flush_barriers` | 208 | 未由扫描证明；以证据限定范围为准 |
| `submit` | 211 | 未由扫描证明；以证据限定范围为准 |
| `chunk_recorder` | 213 | 未由扫描证明；以证据限定范围为准 |
| `command_state` | 237 | 未由扫描证明；以证据限定范围为准 |
| `keep_alive` | 243 | 未由扫描证明；以证据限定范围为准 |
| `access` | 248 | 未由扫描证明；以证据限定范围为准 |
| `execute_graph` | 257 | 未由扫描证明；以证据限定范围为准 |
| `execute_graph_with_context` | 273 | 未由扫描证明；以证据限定范围为准 |
| `execute_graph_inner` | 287 | 未由扫描证明；以证据限定范围为准 |
| `new` | 604 | 未由扫描证明；以证据限定范围为准 |
| `begin` | 618 | 未由扫描证明；以证据限定范围为准 |
| `record` | 639 | 未由扫描证明；以证据限定范围为准 |
| `record_optional_resources` | 652 | 未由扫描证明；以证据限定范围为准 |
| `finish_record` | 674 | 未由扫描证明；以证据限定范围为准 |
| `record_chunked_batch` | 703 | 未由扫描证明；以证据限定范围为准 |
| `submit_recorded` | 873 | 未由扫描证明；以证据限定范围为准 |
| `submit_recorded_with_sync` | 881 | 未由扫描证明；以证据限定范围为准 |
| `complete` | 990 | 未由扫描证明；以证据限定范围为准 |
| `from_name` | 1021 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1038 | 未由扫描证明；以证据限定范围为准 |
| `reset` | 1053 | 未由扫描证明；以证据限定范围为准 |
| `extend` | 1059 | 未由扫描证明；以证据限定范围为准 |
| `duration_ns` | 1070 | 未由扫描证明；以证据限定范围为准 |
| `get` | 1073 | 未由扫描证明；以证据限定范围为准 |
| `call_count` | 1076 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 1079 | 未由扫描证明；以证据限定范围为准 |
| `total_ns` | 1082 | 未由扫描证明；以证据限定范围为准 |
| `max_ns` | 1085 | 未由扫描证明；以证据限定范围为准 |
| `record` | 1088 | 未由扫描证明；以证据限定范围为准 |
| `end_frame` | 1104 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1117 | 未由扫描证明；以证据限定范围为准 |
| `with_resources` | 1128 | 未由扫描证明；以证据限定范围为准 |
| `take_exported` | 1133 | 未由扫描证明；以证据限定范围为准 |
| `take_resource` | 1136 | 未由扫描证明；以证据限定范围为准 |
| `len` | 1140 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 1143 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1154 | 未由扫描证明；以证据限定范围为准 |
| `retire` | 1157 | 未由扫描证明；以证据限定范围为准 |
| `take_retire` | 1162 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/graph.rs

[reconstruction/rhapsody-rebuild/src/rg/graph.rs](../../reconstruction/rhapsody-rebuild/src/rg/graph.rs)

SHA256：`86fd1869e2aee1057d39f51d078723ff166353a27cc8881e1328a9351856f74a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `kind_code` | 24 | 未由扫描证明；以证据限定范围为准 |
| `usage` | 30 | 未由扫描证明；以证据限定范围为准 |
| `with_usage` | 36 | 未由扫描证明；以证据限定范围为准 |
| `new` | 68 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 71 | 未由扫描证明；以证据限定范围为准 |
| `index` | 74 | 未由扫描证明；以证据限定范围为准 |
| `info_origin` | 79 | 未由扫描证明；以证据限定范围为准 |
| `access_mask` | 120 | 未由扫描证明；以证据限定范围为准 |
| `is_write` | 121 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 138 | 未由扫描证明；以证据限定范围为准 |
| `new` | 159 | 未由扫描证明；以证据限定范围为准 |
| `create_raw_resource` | 163 | 未由扫描证明；以证据限定范围为准 |
| `create_resource_handle` | 183 | 未由扫描证明；以证据限定范围为准 |
| `check_handle` | 191 | 未由扫描证明；以证据限定范围为准 |
| `add_pass` | 202 | 未由扫描证明；以证据限定范围为准 |
| `export` | 212 | 未由扫描证明；以证据限定范围为准 |
| `batch_per_pass` | 227 | 未由扫描证明；以证据限定范围为准 |
| `on_retire` | 230 | 未由扫描证明；以证据限定范围为准 |
| `build` | 233 | 未由扫描证明；以证据限定范围为准 |
| `into_parts` | 244 | 未由扫描证明；以证据限定范围为准 |
| `create_without_initial_data` | 254 | 未由扫描证明；以证据限定范围为准 |
| `set_debug_name` | 259 | 未由扫描证明；以证据限定范围为准 |
| `create_without_initial_data` | 281 | 未由扫描证明；以证据限定范围为准 |
| `set_debug_name` | 305 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 322 | 未由扫描证明；以证据限定范围为准 |
| `populate_resources` | 334 | 未由扫描证明；以证据限定范围为准 |
| `transition` | 469 | 未由扫描证明；以证据限定范围为准 |
| `initial_transitions` | 488 | 未由扫描证明；以证据限定范围为准 |
| `pre_pass_transitions` | 512 | 未由扫描证明；以证据限定范围为准 |
| `export_transitions` | 553 | 未由扫描证明；以证据限定范围为准 |
| `into_exports` | 614 | 未由扫描证明；以证据限定范围为准 |
| `compile_graph` | 618 | 未由扫描证明；以证据限定范围为准 |
| `payload_projection` | 742 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/import_export.rs

[reconstruction/rhapsody-rebuild/src/rg/import_export.rs](../../reconstruction/rhapsody-rebuild/src/rg/import_export.rs)

SHA256：`9de6136fefd1fc1e8c5cf56f09b49bacb9372689eabd4a926f9e40e0a28eec0c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `import_unowned` | 7 | 未由扫描证明；以证据限定范围为准 |
| `import_owned` | 25 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/mod.rs

[reconstruction/rhapsody-rebuild/src/rg/mod.rs](../../reconstruction/rhapsody-rebuild/src/rg/mod.rs)

SHA256：`d2bd59fe5eb4848c60fa180703bf6972024a9421d3badf395600b48de6ae1451`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/rhapsody-rebuild/src/rg/parallel.rs

[reconstruction/rhapsody-rebuild/src/rg/parallel.rs](../../reconstruction/rhapsody-rebuild/src/rg/parallel.rs)

SHA256：`924632134c56f59f43c9919978964e732dc340236c63d8561f849174ae7f9838`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `original_budget` | 31 | 未由扫描证明；以证据限定范围为准 |
| `original_chunks` | 37 | 未由扫描证明；以证据限定范围为准 |
| `balanced_chunks` | 56 | 未由扫描证明；以证据限定范围为准 |
| `plan_chunks` | 107 | 未由扫描证明；以证据限定范围为准 |
| `map_chunks` | 137 | 未由扫描证明；以证据限定范围为准 |
| `default` | 185 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/pass_api.rs

[reconstruction/rhapsody-rebuild/src/rg/pass_api.rs](../../reconstruction/rhapsody-rebuild/src/rg/pass_api.rs)

SHA256：`305afdbbd014646b2252f226d850feef5283d151560ac063f9a160705a3d2252`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `dispatch` | 97 | 未由扫描证明；以证据限定范围为准 |
| `new` | 109 | 未由扫描证明；以证据限定范围为准 |
| `dispatch` | 115 | 未由扫描证明；以证据限定范围为准 |
| `find_resource_access_type` | 274 | 未由扫描证明；以证据限定范围为准 |
| `new` | 284 | 未由扫描证明；以证据限定范围为准 |
| `draw_call_count` | 300 | 未由扫描证明；以证据限定范围为准 |
| `emit` | 304 | 未由扫描证明；以证据限定范围为准 |
| `begin_render_pass` | 310 | 未由扫描证明；以证据限定范围为准 |
| `begin_attachments` | 316 | 未由扫描证明；以证据限定范围为准 |
| `end_render_pass` | 320 | 未由扫描证明；以证据限定范围为准 |
| `set_default_view_and_scissor` | 325 | 未由扫描证明；以证据限定范围为准 |
| `set_stencil_ref` | 340 | 未由扫描证明；以证据限定范围为准 |
| `set_line_width` | 347 | 未由扫描证明；以证据限定范围为准 |
| `bind_vertex_buffer_by_binding_point` | 354 | 未由扫描证明；以证据限定范围为准 |
| `bind_index_buffer` | 358 | 未由扫描证明；以证据限定范围为准 |
| `push_constants` | 363 | 未由扫描证明；以证据限定范围为准 |
| `copy_buffer` | 375 | 未由扫描证明；以证据限定范围为准 |
| `fill_buffer` | 390 | 未由扫描证明；以证据限定范围为准 |
| `clear_color_image` | 415 | 未由扫描证明；以证据限定范围为准 |
| `draw` | 428 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed` | 444 | 未由扫描证明；以证据限定范围为准 |
| `draw_indirect` | 462 | 未由扫描证明；以证据限定范围为准 |
| `draw_indirect_count` | 473 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect` | 493 | 未由扫描证明；以证据限定范围为准 |
| `draw_indexed_indirect_count` | 510 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_groups_3d` | 530 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_threads_3d` | 538 | 未由扫描证明；以证据限定范围为准 |
| `dispatch_indirect` | 554 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/pass_bindings.rs

[reconstruction/rhapsody-rebuild/src/rg/pass_bindings.rs](../../reconstruction/rhapsody-rebuild/src/rg/pass_bindings.rs)

SHA256：`5e347e3109ba49803b07deb32bfbde4632187967df70a65ae5cb28ece5343207`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 47 | 未由扫描证明；以证据限定范围为准 |
| `from` | 52 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 57 | 未由扫描证明；以证据限定范围为准 |
| `buffer` | 70 | 未由扫描证明；以证据限定范围为准 |
| `image` | 89 | 未由扫描证明；以证据限定范围为准 |
| `with_expected_generation` | 105 | 未由扫描证明；以证据限定范围为准 |
| `resource` | 109 | 未由扫描证明；以证据限定范围为准 |
| `from_registry` | 147 | 未由扫描证明；以证据限定范围为准 |
| `populate` | 229 | 未由扫描证明；以证据限定范围为准 |
| `resolve` | 298 | 未由扫描证明；以证据限定范围为准 |
| `resolve_raw` | 309 | 未由扫描证明；以证据限定范围为准 |
| `invalidate_views` | 468 | 未由扫描证明；以证据限定范围为准 |
| `view_count` | 473 | 未由扫描证明；以证据限定范围为准 |
| `descriptor_writes` | 478 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/pass_builder.rs

[reconstruction/rhapsody-rebuild/src/rg/pass_builder.rs](../../reconstruction/rhapsody-rebuild/src/rg/pass_builder.rs)

SHA256：`586d6d00c375e9526651aba0bbe035113dc526a544a5abcc556aebb92b33d709`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_resource_access` | 12 | 未由扫描证明；以证据限定范围为准 |
| `render` | 29 | 未由扫描证明；以证据限定范围为准 |
| `render_with_context` | 33 | 未由扫描证明；以证据限定范围为准 |
| `rg_mut` | 50 | 未由扫描证明；以证据限定范围为准 |
| `write_impl` | 54 | 未由扫描证明；以证据限定范围为准 |
| `buffer_ref` | 62 | 未由扫描证明；以证据限定范围为准 |
| `texture_ref` | 86 | 未由扫描证明；以证据限定范围为准 |
| `render_bound` | 111 | 未由扫描证明；以证据限定范围为准 |
| `render_bound_with_context` | 122 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/recorder.rs

[reconstruction/rhapsody-rebuild/src/rg/recorder.rs](../../reconstruction/rhapsody-rebuild/src/rg/recorder.rs)

SHA256：`3325be3a63aae6a5c6bb55e4afb433c200d01053708a82498f8c3eccf637ec35`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `validate` | 60 | 未由扫描证明；以证据限定范围为准 |
| `event` | 64 | 未由扫描证明；以证据限定范围为准 |
| `barrier` | 68 | 未由扫描证明；以证据限定范围为准 |
| `close_scope` | 72 | 未由扫描证明；以证据限定范围为准 |
| `record_batch` | 94 | 未由扫描证明；以证据限定范围为准 |
| `plan_chunked_batch` | 174 | 未由扫描证明；以证据限定范围为准 |
| `finish_chunked_batch` | 237 | 未由扫描证明；以证据限定范围为准 |
| `initial_barriers` | 250 | 未由扫描证明；以证据限定范围为准 |
| `finish_batch` | 292 | 未由扫描证明；以证据限定范围为准 |
| `submit_pending` | 344 | 未由扫描证明；以证据限定范围为准 |
| `ensure_active_cb` | 366 | 未由扫描证明；以证据限定范围为准 |
| `new` | 386 | 未由扫描证明；以证据限定范围为准 |
| `resource_index` | 392 | 未由扫描证明；以证据限定范围为准 |
| `access` | 395 | 未由扫描证明；以证据限定范围为准 |
| `create` | 406 | 未由扫描证明；以证据限定范围为准 |
| `new` | 415 | 未由扫描证明；以证据限定范围为准 |
| `recorder` | 418 | 未由扫描证明；以证据限定范围为准 |
| `record_pass` | 422 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/resource.rs

[reconstruction/rhapsody-rebuild/src/rg/resource.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource.rs)

SHA256：`6a3c3714051a6f7064826241b31fd0373e3858a1276577d49712b75bd9110a93`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_raw` | 9 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 12 | 未由扫描证明；以证据限定范围为准 |
| `new` | 25 | 未由扫描证明；以证据限定范围为准 |
| `desc` | 31 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 34 | 未由扫描证明；以证据限定范围为准 |
| `clone_unchecked` | 37 | 未由扫描证明；以证据限定范围为准 |
| `ty` | 59 | 未由扫描证明；以证据限定范围为准 |
| `validate_read_access` | 68 | 未由扫描证明；以证据限定范围为准 |
| `borrow` | 83 | 未由扫描证明；以证据限定范围为准 |
| `ty` | 114 | 未由扫描证明；以证据限定范围为准 |
| `descriptor_from_texture` | 128 | 未由扫描证明；以证据限定范围为准 |
| `descriptor_from_buffer` | 134 | 未由扫描证明；以证据限定范围为准 |
| `raw` | 150 | 未由扫描证明；以证据限定范围为准 |
| `bind_uniform_buffer` | 155 | 未由扫描证明；以证据限定范围为准 |
| `bind_storage_buffer` | 169 | 未由扫描证明；以证据限定范围为准 |
| `bind_view` | 186 | 未由扫描证明；以证据限定范围为准 |
| `into_sampled_rg_texture` | 201 | 未由扫描证明；以证据限定范围为准 |
| `original_ref_view_binding` | 211 | 未由扫描证明；以证据限定范围为准 |
| `original_ref_buffer_binding` | 222 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/resource_info.rs

[reconstruction/rhapsody-rebuild/src/rg/resource_info.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_info.rs)

SHA256：`e260d83b807083d54fa34bd8ab86c3e42ae068740a167b4f3b8300f797e1c61c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from` | 17 | 未由扫描证明；以证据限定范围为准 |
| `from` | 26 | 未由扫描证明；以证据限定范围为准 |
| `index` | 33 | 未由扫描证明；以证据限定范围为准 |
| `try_index` | 38 | 未由扫描证明；以证据限定范围为准 |
| `valid` | 42 | 未由扫描证明；以证据限定范围为准 |
| `is_acquire` | 57 | 未由扫描证明；以证据限定范围为准 |
| `resource_idx` | 61 | 未由扫描证明；以证据限定范围为准 |
| `pass_index` | 65 | 未由扫描证明；以证据限定范围为准 |
| `type_code` | 86 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 126 | 未由扫描证明；以证据限定范围为准 |
| `calculate` | 136 | 未由扫描证明；以证据限定范围为准 |
| `extract_creation_flags` | 182 | 未由扫描证明；以证据限定范围为准 |
| `extend_usage_flags_for_exports` | 201 | 未由扫描证明；以证据限定范围为准 |
| `extract_pass_resource_access` | 225 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/resource_pool.rs

[reconstruction/rhapsody-rebuild/src/rg/resource_pool.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_pool.rs)

SHA256：`71c1ff24d642aa1af02db5de861e80593226d3565efc6cfc2875727dc35408db`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `recycle_exports` | 5 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/resource_registry.rs

[reconstruction/rhapsody-rebuild/src/rg/resource_registry.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_registry.rs)

SHA256：`d6d5f193205abad54d227b74c552562d7225173011c28bacb128f1f001094c19`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 38 | 未由扫描证明；以证据限定范围为准 |
| `take_texture` | 47 | 未由扫描证明；以证据限定范围为准 |
| `insert_texture` | 51 | 未由扫描证明；以证据限定范围为准 |
| `take_buffer` | 55 | 未由扫描证明；以证据限定范围为准 |
| `insert_buffer` | 59 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 63 | 未由扫描证明；以证据限定范围为准 |
| `default` | 77 | 未由扫描证明；以证据限定范围为准 |
| `backing` | 108 | 未由扫描证明；以证据限定范围为准 |
| `resource` | 113 | 未由扫描证明；以证据限定范围为准 |
| `image_from_raw_handle` | 130 | 未由扫描证明；以证据限定范围为准 |
| `image` | 140 | 未由扫描证明；以证据限定范围为准 |
| `image_view` | 147 | 未由扫描证明；以证据限定范围为准 |
| `buffer` | 151 | 未由扫描证明；以证据限定范围为准 |
| `into_exports` | 160 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/resource_rules.rs

[reconstruction/rhapsody-rebuild/src/rg/resource_rules.rs](../../reconstruction/rhapsody-rebuild/src/rg/resource_rules.rs)

SHA256：`a8bdb8c23d99d512c92ab7424b45d9261563b432a3a15d8a68a0b838e890c8cd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 49 | 未由扫描证明；以证据限定范围为准 |
| `can_be_concurrent_with` | 56 | 未由扫描证明；以证据限定范围为准 |
| `link_up_next_access_chains` | 72 | 未由扫描证明；以证据限定范围为准 |
| `calculate_lifetime_events` | 83 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/state.rs

[reconstruction/rhapsody-rebuild/src/rg/state.rs](../../reconstruction/rhapsody-rebuild/src/rg/state.rs)

SHA256：`200ffff34077be2726be26848afd8f2ac001ac036373f432d20a7d9eee103f89`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `begin_frame` | 16 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 20 | 未由扫描证明；以证据限定范围为准 |
| `compile_frame` | 25 | 未由扫描证明；以证据限定范围为准 |
| `take_for_compile` | 76 | 未由扫描证明；以证据限定范围为准 |
| `restore_temporal` | 86 | 未由扫描证明；以证据限定范围为准 |
| `restore_temporal_owned` | 91 | 未由扫描证明；以证据限定范围为准 |
| `export_and_restore` | 103 | 未由扫描证明；以证据限定范围为准 |
| `export_and_restore_owned` | 111 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/temporal.rs

[reconstruction/rhapsody-rebuild/src/rg/temporal.rs](../../reconstruction/rhapsody-rebuild/src/rg/temporal.rs)

SHA256：`b02f6fc41eb0a9817a8226c28c4c6e8897f6c2bf1d4426e3462c8a2c74eef8fe`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `flip` | 16 | 未由扫描证明；以证据限定范围为准 |
| `value` | 19 | 未由扫描证明；以证据限定范围为准 |
| `new` | 34 | 未由扫描证明；以证据限定范围为准 |
| `into_inner` | 41 | 未由扫描证明；以证据限定范围为准 |
| `as_ref` | 44 | 未由扫描证明；以证据限定范围为准 |
| `split_pair` | 51 | 未由扫描证明；以证据限定范围为准 |
| `parity` | 54 | 未由扫描证明；以证据限定范围为准 |
| `into_pair_ordering` | 59 | 未由扫描证明；以证据限定范围为准 |
| `original_tag` | 80 | 未由扫描证明；以证据限定范围为准 |
| `create_temporal` | 111 | 未由扫描证明；以证据限定范围为准 |
| `release_temporal` | 112 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 126 | 未由扫描证明；以证据限定范围为准 |
| `clear_owned` | 132 | 未由扫描证明；以证据限定范围为准 |
| `release_retired` | 136 | 未由扫描证明；以证据限定范围为准 |
| `release_superseded` | 142 | 未由扫描证明；以证据限定范围为准 |
| `retired_tokens` | 159 | 未由扫描证明；以证据限定范围为准 |
| `len` | 162 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 165 | 未由扫描证明；以证据限定范围为准 |
| `get` | 168 | 未由扫描证明；以证据限定范围为准 |
| `record_state` | 171 | 未由扫描证明；以证据限定范围为准 |
| `parity` | 174 | 未由扫描证明；以证据限定范围为准 |
| `get_and_flip_parity` | 177 | 未由扫描证明；以证据限定范围为准 |
| `get_or_create_temporal` | 186 | 未由扫描证明；以证据限定范围为准 |
| `get_or_create_owned` | 258 | 未由扫描证明；以证据限定范围为准 |
| `get_or_create_temporal_pair` | 271 | 未由扫描证明；以证据限定范围为准 |
| `get_or_create_owned_pair` | 298 | 未由扫描证明；以证据限定范围为准 |
| `export_temporal` | 321 | 未由扫描证明；以证据限定范围为准 |
| `validate_compiled_exports` | 336 | 未由扫描证明；以证据限定范围为准 |
| `raw_state` | 358 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/util.rs

[reconstruction/rhapsody-rebuild/src/rg/util.rs](../../reconstruction/rhapsody-rebuild/src/rg/util.rs)

SHA256：`f28f2f6aedbaa7606a253e3a711c2c79e50f224fe6dac9755a515b045949e534`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `validate_handles` | 6 | 未由扫描证明；以证据限定范围为准 |
| `checked_u16_index` | 15 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/rg/vulkan.rs

[reconstruction/rhapsody-rebuild/src/rg/vulkan.rs](../../reconstruction/rhapsody-rebuild/src/rg/vulkan.rs)

SHA256：`2cf3d51ddede69956528525a196d9ed3aea7deb3426b760d59728834d0b2b800`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 37 | 未由扫描证明；以证据限定范围为准 |
| `access_mask` | 51 | 未由扫描证明；以证据限定范围为准 |
| `is_write` | 54 | 未由扫描证明；以证据限定范围为准 |
| `descriptor` | 65 | 未由扫描证明；以证据限定范围为准 |
| `get` | 96 | 未由扫描证明；以证据限定范围为准 |
| `buffer` | 102 | 未由扫描证明；以证据限定范围为准 |
| `image` | 108 | 未由扫描证明；以证据限定范围为准 |
| `add_transition` | 166 | 未由扫描证明；以证据限定范围为准 |
| `record_transitions` | 223 | 未由扫描证明；以证据限定范围为准 |
| `new` | 253 | 未由扫描证明；以证据限定范围为准 |
| `new_for_queue` | 256 | 未由扫描证明；以证据限定范围为准 |
| `command_for_slot` | 307 | 未由扫描证明；以证据限定范围为准 |
| `selected_queue` | 342 | 未由扫描证明；以证据限定范围为准 |
| `selected_family` | 345 | 未由扫描证明；以证据限定范围为准 |
| `top_level_pool_handle` | 349 | 未由扫描证明；以证据限定范围为准 |
| `begin_top_level_timestamp_query` | 367 | 未由扫描证明；以证据限定范围为准 |
| `end_top_level_timestamp_query` | 385 | 未由扫描证明；以证据限定范围为准 |
| `check_synchronization` | 415 | 未由扫描证明；以证据限定范围为准 |
| `set_graph_synchronization` | 440 | 未由扫描证明；以证据限定范围为准 |
| `set_batch_synchronization` | 450 | 未由扫描证明；以证据限定范围为准 |
| `declare` | 461 | 未由扫描证明；以证据限定范围为准 |
| `resource_clone` | 482 | 未由扫描证明；以证据限定范围为准 |
| `collect_completed_managed` | 485 | 未由扫描证明；以证据限定范围为准 |
| `bind_import_named` | 500 | 未由扫描证明；以证据限定范围为准 |
| `bind_import_named_owned_by` | 509 | 未由扫描证明；以证据限定范围为准 |
| `bind_import` | 546 | 未由扫描证明；以证据限定范围为准 |
| `bind_import_owned_by` | 549 | 未由扫描证明；以证据限定范围为准 |
| `validate_graph` | 556 | 未由扫描证明；以证据限定范围为准 |
| `new_execution_chain` | 627 | 未由扫描证明；以证据限定范围为准 |
| `execute` | 662 | 未由扫描证明；以证据限定范围为准 |
| `create_without_initial_data` | 686 | 未由扫描证明；以证据限定范围为准 |
| `set_debug_name` | 723 | 未由扫描证明；以证据限定范围为准 |
| `prepare_compiled_resources` | 726 | 未由扫描证明；以证据限定范围为准 |
| `managed_pass_resources` | 752 | 未由扫描证明；以证据限定范围为准 |
| `command_started` | 755 | 未由扫描证明；以证据限定范围为准 |
| `retain_command_owners` | 760 | 未由扫描证明；以证据限定范围为准 |
| `collect_managed_waste` | 773 | 未由扫描证明；以证据限定范围为准 |
| `graph_queue` | 776 | 未由扫描证明；以证据限定范围为准 |
| `batch_synchronization` | 779 | 未由扫描证明；以证据限定范围为准 |
| `managed_resource` | 805 | 未由扫描证明；以证据限定范围为准 |
| `command_handle` | 826 | 未由扫描证明；以证据限定范围为准 |
| `chunk_command_handle` | 829 | 未由扫描证明；以证据限定范围为准 |
| `chunk_recorder` | 832 | 未由扫描证明；以证据限定范围为准 |
| `begin_command` | 839 | 未由扫描证明；以证据限定范围为准 |
| `end_command` | 846 | 未由扫描证明；以证据限定范围为准 |
| `flush_barriers` | 854 | 未由扫描证明；以证据限定范围为准 |
| `transition` | 876 | 未由扫描证明；以证据限定范围为准 |
| `submit` | 883 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 926 | 未由扫描证明；以证据限定范围为准 |
| `wait` | 946 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 989 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 1018 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1037 | 未由扫描证明；以证据限定范围为准 |
| `is_poisoned` | 1053 | 未由扫描证明；以证据限定范围为准 |
| `last_known_frame_duration_ns` | 1057 | 未由扫描证明；以证据限定范围为准 |
| `begin_top_level_timestamp_query` | 1061 | 未由扫描证明；以证据限定范围为准 |
| `end_top_level_timestamp_query` | 1068 | 未由扫描证明；以证据限定范围为准 |
| `wait_submits_done` | 1076 | 未由扫描证明；以证据限定范围为准 |
| `begin_frame` | 1083 | 未由扫描证明；以证据限定范围为准 |
| `execute_frame` | 1104 | 未由扫描证明；以证据限定范围为准 |
| `end_frame` | 1173 | 未由扫描证明；以证据限定范围为准 |
| `pending_frame_count` | 1176 | 未由扫描证明；以证据限定范围为准 |
| `wait_gpu_queue_trim` | 1180 | 未由扫描证明；以证据限定范围为准 |
| `wait_frame` | 1193 | 未由扫描证明；以证据限定范围为准 |
| `collect_after_completion` | 1259 | 未由扫描证明；以证据限定范围为准 |
| `clear_after_completion` | 1319 | 未由扫描证明；以证据限定范围为准 |
| `resource_count` | 1322 | 未由扫描证明；以证据限定范围为准 |
| `command_count` | 1325 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 1330 | 未由扫描证明；以证据限定范围为准 |
| `new` | 1348 | 未由扫描证明；以证据限定范围为准 |
| `register` | 1356 | 未由扫描证明；以证据限定范围为准 |
| `resource` | 1360 | 未由扫描证明；以证据限定范围为准 |
| `import_into` | 1363 | 未由扫描证明；以证据限定范围为准 |
| `len` | 1374 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 1377 | 未由扫描证明；以证据限定范围为准 |
| `create_temporal` | 1382 | 未由扫描证明；以证据限定范围为准 |
| `release_temporal` | 1410 | 未由扫描证明；以证据限定范围为准 |
