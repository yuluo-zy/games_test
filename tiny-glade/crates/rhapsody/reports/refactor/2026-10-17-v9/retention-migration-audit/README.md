# 保活接口删除与下游迁移验收映射

本报告记录 2026-10-17 本轮源码审计结果及待统一复验的测试映射，**不登记最终 PASS，也不宣告 M9 完成**。统一矩阵正在运行；旧失败日志保留，不能用测试进程退出码替代 validation、源码身份及环境门。

## 删除与正式接线

以下源码路径均相对于 `games_test/tiny-glade`；表中测试是验收入口，不表示本报告已重新执行。

| 删除的入口或旧消费者行为 | 正式生产路径 | 关键验收入口 |
|---|---|---|
| `GpuUse::collect_uses`、`keep_alive`、`keep_alive_known`、`track_borrowed*` 等旧保活接口；不保留同名兼容 alias | `resource::GpuUse::visit_uses -> Result`；RawCB 按 `retain_object`、crate 内 `retain_recorded`/借用版本、原生 `retain_opaque` 区分合同。录制元数据失败 sticky，`end`/提交不得绕过；成功提交前完成准入，真实提交成功才发布使用点 | `tests/command_retention.rs`：owner/cell 容量、嵌套 visitor 失败、忽略非法范围后的 sticky 拒绝、外部 semaphore owners/数组 OOM、无完成点发布；`cpu_frame_allocations`、`gpu_frame_allocations` |
| 图回调手工添加 memory owners，代替图访问声明 | `rg/pass_bindings.rs::retain_pass_resources` 只保留本 pass 声明资源；`PassContext::retain_object` 与 RawCB Object 模式拒绝 memory domain。bindless `bind_pass` 核验真实资源身份、shader access、布局和完整绑定范围；附件 `retain_for_pass` 作对应核验后才走受控 Recorded | `tests/bindless.rs`、`bindless_shader_abi.rs`、`command_retention.rs`；tiny 外部 compute/raster fixtures |
| 普通 staging 和缓存 upload 的 callback 中捕获隐藏 source；`RenderPassApi::copy_buffer` 旧桥 | country `StagingCopy` 数据包与 `CorePassGraph::copy_from_staging`；tiny `GraphPasses` 在构图时解析正式 staging lease、导入 source、声明精确 source/destination 范围。缓存 `cached_result` 仅保留一个正式 miss callback `(session, graph, output)`，miss 内导入 source；consumer 亦声明读取 | `tests/frame_imports.rs::declared_frame_sources_copy_real_bytes_and_failed_recording_does_not_leak_bindings`；tiny `render_host` 的 `core_passes_execute_on_lavapipe_through_the_rhapsody_session`、`versioned_country_upload_uses_real_session_subgraph_and_result_caches`；country `render::rg_helpers::`、`systems::render_loop::`（含四目标 indirect reset） |
| compute external bindings 只在 callback 查 HostTables、创建 descriptor，构图无 source 声明 | `tiny-glade/src/render_host.rs::add_compute` 在 callback 之前解析 Arc owner/raw identity/view，使用 Session frame import；buffer 保守 MEMORY RW、sampled image ShaderReadOnly、storage image General，均声明实际范围；外部写明确 side effect。callback 只消费已解析 typed bindings；indirect dispatch builder 先声明参数缓冲访问 | tiny `render_host`：`external_compute_buffers_are_declared_before_recording_and_survive_host_release`；`external_compute_sampled_and_storage_images_have_declared_views_and_real_pixels`（无 builder RG access、执行前释放宿主句柄、真实像素）；普通 compute/push-rebind fixtures |
| raster external buffer/image、index fallback 在 callback 才查宿主并手工保活 | `tiny-glade/src/raster_host.rs` 在声明期 `import_draw_bindings`，Session typed import 复用真实物理资源；逐 view/byte range 声明。index buffer 预先导入并声明实际 index bytes；callback 只用 managed 资源及受校验 bindless/附件 | tiny `core_raster_draw`：`indexed_draw_declares_the_actual_index_bytes_and_outlives_the_host_handle`、`owned_external_draw_resources_are_declared_before_recording_and_outlive_host_handles`，以及 triangle/depth/multiframe fixtures |
| 手工 token/临时 owner 无绑定回收，或同物理对象重复引入两套 logical 账本 | `VulkanGraphSession::import_frame_buffer` / `import_frame_texture` 共用有预算有界 Weak lease registry；按真实 Arc identity 去重，包括已手工导入本图的对象；图直接 drop/失败自动退 lease，实际提交由 command owners 承接 | `tests/frame_imports.rs` 三项：物理复用与取消、真实 copy/失败录制与部分提交、texture 导入去重与取消释放 |
| whole-resource scalar transition / snapshot 作为第二套 native 状态路径 | 旧 `graph::Transition`、`CompiledPlan.pre`、snapshot initial/pre/export/pending_initial 及其复制删除；唯一生产 native 区间程序/绑定屏障/提交 gate 内 StatePatch 预检与真实 point publication。独立 checker 不调用生产 split/merge 实现 | `tests/native_ranges.rs`、`host_buffer_ranges.rs`、`multi_queue_graph.rs`、`texture_upload_contract.rs`；`rg/plan_check/{range_program,bound,native_state,range_oracle}.rs` 的 oracle/负例 |
| `DescriptorSetState` / `PoolQueue` / `DiscardPile` 及无消费者的旧 persistent-set forwarding | 正式共享 `BindlessHeap`、lowered shader ABI 与 `BindlessParameters`；tiny ShaderHost/RasterHost、tg-text 和 Window Lab 均走正式 binding。country `rhapsody_contract` forwarding 与 `rhapsody_types` 旧再导出删除，不保留 rhapsody compatibility stub | `bindless_shader_abi`；tiny `render_host`/`core_raster_draw`；tg-text `gpu_pipeline` 两个原文字像素对照；Window Lab S03/S04/S08 |

文字的 `crates/tg-text/src/gpu.rs` 是正式 raw 渲染消费者：上传/采样/目标/回读先通过 `raw::resource_access` 声明实际 scope、布局、范围与 expected revision，附件再经 `retain_for_command` 核验。没有用 memory `retain_object` 或图回调隐藏资源替代这些步骤。

## 仍必须保留的明确协议

- **Raw Opaque** 是原生命令接口的显式合同，不是旧图路径：`raw/setup.rs` / `device_api` 的同步 staging 服务、`TransferManager` 的精确 buffer ranges、原生 debug marker，以及 raw bindless binder 在未能证明显式 patch 覆盖时，都使用它。Opaque 登记使对应 native range 进入 Unknown，不能冒充图已知布局/可见性；其资源使用仍参加预算、实际提交点、等待及 Retirement。图 `PassContext` 没有公开 Opaque memory 入口。
- **UploadRead** 只用于正式 constants/parameter ring 的已填充、flushed、不可变 host upload 区间，携带实际 offset/size/scope，由提交 gate 内原生状态合同登记。它不接受隐藏 GPU producer，不是任意 staging/cached result 的豁免；普通业务 staging 已迁为前述 typed source 声明。
- **WSI display** 是 `FrameLoop` 单独的 RawCB + `AcquiredImageLease` 阶段。Window Lab 对图输出 source 先作显式 `image_access`，目标 swapchain image 由 acquire/present 租约与 frame sync 管理；它不是未声明的图 pass memory callback。swapchain/frame-sync 对象使用 Object retention。

只读审计覆盖 tiny 当前上传、clear、compute、支持的 raster draw；tg-text；Window Lab fixture/raster/temporal/transient/result/transfer/display 回调及其实际绑定链。未发现剩余的生产 callback HostTables 临时查 raw owner 或手工 memory owner 绕声明路径。任意调用者自行编写的 unsafe Vulkan 命令并非本结论的防错保证范围。

## 证据与未完成门

- [retention-closure manifest](../retention-closure/manifest.json) 保留早期 `bindless-normal` 非零退出以及 build 源码变化记录；不能作为本轮最终同源通过证明。
- [retention-consumers manifest](../retention-consumers/manifest.json) 及 [raster 原始日志](../retention-consumers/core_raster_draw.log) 保留首次 raster **9 条 VUID**。虽然 Rust 测试退出 0，该次仍是验收失败；对应修复已落，等待统一复验，不能抹掉失败或沿用退出码宣称通过。该次 tiny build 的 source-stable 也为 false。
- 新 compute 图像 fixture 首次合并类型检查缺少 `ash::vk` 导入，已窄修并冻结；其最终编译、原生像素及 validation 结果由统一矩阵登记。本报告没有运行 Cargo/GPU，也没有改写上述历史日志。
- country 的 default CPU 与 optional all-features 链接分开记录。已有 FMOD optional SDK 链接 BLOCKED 不可用 default CPU 结果替代；indirect-reset 首轮 mock 恒 id=0 的失败及后续编译中间态记录亦保留在 [retention-migration](../retention-migration/country-manifest.json)，正式 fixture 已改为独立 id 并保持四目标/顺序断言，最终重验归统一矩阵。
- country `TextureLibraryDevice` 的原游戏纹理库业务状态尚未恢复，保留明确日志占位；未来真实宿主须适配 `BindlessHeap`。这是未恢复业务边界，**不是** rhapsody 保留 `DescriptorSetState`、旧 persistent descriptor API 或替代引擎的理由，也不能登记为该业务已完成。

最终验收仍须对应冻结源码/二进制身份、实际 GPU 无 SKIP、validation 0、像素/字节对照、失败恢复与统一矩阵结果；本报告仅提供映射。
