# rhapsody rg 域（render graph）算法级深挖

- agent：rhapsody-rg。日期：2026-10-15。基线：B patched EXE（SHA cf048e27…）。
- 域清单：`domains/rg.json`（15 文件 / 107 符号 / 57 有地址，其中 11 个 Ghidra NotFound）。
- 反编译：`domains/rg/decomp/decomp-<va>.txt`（57 个已有 + 本轮补 5 个：0x1402c4280 / 0x140166510 / 0x1404f7450 / 0x1404f7ca0 / 0x140bea680，RE-MCP `tg_rh_rg`）+ 调用链调查的 `rhapsody/decomp/*.c`（compile×3 / execute / execute_compiled / record_batch / calculate / get_or_create_temporal / render / render_world 等）。
- 对照基准：`reconstruction/rhapsody-rebuild/src/rg/**`（含候选新增 parallel/attachments/access_ledger/resource_rules/vulkan）。
- 标注：**O**=机器码/PDB 实测；**I**=推断；**U**=未知。不改任何 reconstruction 源码。

---

## 1. 模块总览

### 1.1 文件 → 职责

| 原文件 | 职责 | 关键入口（地址） |
|---|---|---|
| `rg/state.rs` | `RenderGraphState` 跨帧状态：持有图/时态/swapchain 尺寸，`compile` 三实例驱动整帧编译 | compile×3：0x1401647b0（加载图）/ 0x140161cd0（世界图）/ 0x140166510（展示图，execute 内部） |
| `rg/graph.rs` | `RenderGraph` 图声明（资源/Pass/导出/退役回调）；`ExportableResource` | create_raw_resource 0x140be94d0、on_retire 0x140be9420 |
| `rg/recorder.rs` | 录制器 `RenderGraphRecord<PassResources>` 与 Pass 声明（add_resource_access/render/import/export）；`ResourceSyncGroup`、`PassResourceAccessType` | execute_compiled 内 drop_in_place 证实类型；0x140bece40..0x140bece70 |
| `rg/pass_builder.rs` | `PassBuilder<'graph,UD>` 构图 DSL：add_resource_access / render（6 个游戏闭包实例）/ write_impl / rg_mut | 6 个 render<PassResources,closure> 实例（va 由闭包单态化内联） |
| `rg/pass_api.rs` | `RenderPassApi<'a,B>` 21 方法命令下发（draw/dispatch/push_constants/fill/clear/copy/barrier） | begin_render_pass 0x1416f6e90 等 12 个地址 |
| `rg/binding.rs` | `PassResourceSlot`（uniform/storage/sampled/storage-image 槽）→ `rg_binding_to_descriptor_binding` | 0x14205eff0 |
| `rg/resource_info.rs` | `ResourceInfo::calculate` + 5 方法：把图声明折算成访问记录/生命周期/usage 标志 | calculate 0x140530430、clear 0x140bd4410、extract_creation_flags 0x140bd44b0、extend_usage_flags_for_exports 0x140bd4560、link_up_next_access_chains 0x140bd47d0、calculate_lifetime_events 0x140bd4860、image/buffer_access_mask_to_usage_flags 0x140bd49d0/0x140bd4b00 |
| `rg/resource_pool.rs` | `ResourcePool` 描述符精确匹配 + LIFO 复用（HashMap<Desc,Vec<Resource>>） | take/insert/clear：0x140be6b90 / 0x140be6d70 / 0x140be71e0 / 0x140be7320 / 0x140be76f0 |
| `rg/resource_registry.rs` | `ResourceRegistry` 逻辑→物理映射、populate（池/创建/导入）、into_exports | resource 0x140be78b0、image_from_raw_handle 0x140be7940、buffer 0x140be79d0、image_view 0x140be7a60、populate_resources 0x140be7ba0、into_exports 0x140be9040 |
| `rg/resource.rs` | `Handle`/`Ref`/`ResourceHandle`/`ResourceOrigin`/`RuntimeResource` 句柄族 | RuntimeResource::borrow 0x140bd6c80、ResourceOrigin::ty 0x140bd6d10 |
| `rg/compiled.rs` | `CompiledRenderGraph<UD>` 0x218B 布局、`BatchCompiler` 分块、record_batch 并行录制 | new 0x1402c4280、record_batch 0x1402c1d40、BatchCompiler::new 0x140be95e0（NF）、flush_batch 0x140be9600、pass_access_range_indices 0x140be9690（NF） |
| `rg/executor.rs` | `GraphExecutor`（帧级）与 `GraphExecutionRecorder`（命令缓冲链） | begin_frame 0x140beabd0、end_frame 0x140bec400、execute 0x1400fb5b0、execute_compiled 0x1400fa190、ensure_active_cb 0x140bebc90、submit_pending_cbs 0x140bebdf0、ExecutedRenderGraph::new 0x140bea180、retire 0x140be9d00、take_resource 0x140be9c80、wait_gpu_queue_trim 0x140bea680、last_known_frame_duration_ns 0x140beaae0、PassPgoStats::duration_ns 0x140bea1e0、GraphExecutor::new 0x140bea2d0 |
| `rg/temporal.rs` | `TemporalRenderGraphRecorder` 跨帧时态纹理（键=Ustr） | get_or_create_temporal 0x1404f6e50、get_or_create_temporal_pair 0x1404f7450、export_temporal 0x1404f7ca0、get_and_flip_parity 0x140beced0 |
| `rg/import_export.rs` | `ImportExportToRenderGraph` trait（Texture/Buffer 两个 impl）：import_unowned/import_owned/export/from_owned_rg_resource/into_sampled_rg_texture | 0x140bd60a0（Texture import_unowned）、0x140bd6380（Texture export）、0x140bd63f0（from_owned_rg_resource）、0x140bd4fc0（Buffer import_unowned） |
| `rg/util.rs` | 辅助 `same` | — |

### 1.2 主要数据结构 + 布局（O=机器码实测，I=推断，U=未定）

**游戏/原版 Vec = cap-first** `{cap@+0, ptr@+8, len@+0x10}`（PLAYBOOK 规则，全表按此对齐）。

- **ResourceOrigin（图资源条目，0x48 = 72B，O）**：`{u64 tag@+0, payload[64]@+8}`。descriptor 判别字节在 **+0x36**（=payload[46]）；tag 0=Create（创建期 usage 标志 image@+0x18 / buffer@+0x20），1=ImportUnowned，2=ImportOwned。`ResourceOrigin::ty`（0x140bd6d10）= 按 tag 跳表，descriptor 判别 6→0(image)/7→1(buffer)/8→panic。
- **ResourceInfo（O，≥0x90B）**：`accesses Vec<ResourceAccessRecord>@+0x00`、`pass_access_ranges Vec<(u32,u32)>@+0x18`、`lifetimes Vec<ResourceLifetimeRecord>@+0x30`、`image_usage_flags Vec<u32>@+0x48`、`buffer_usage_flags Vec<u32>@+0x60`、`lifetime_events Vec<u32>@+0x78`、`exported 位集@+0x90`（+0x90 memset 0xff、+0x98/+0xa0 容量、+0xa8 长）。
  - **ResourceAccessRecord（0x14 = 20B，O）**：`{resource_index u32@0, previous_access_index u32@4, next_access_index u32@8, synchronization_group u32@0xc, access_code u8@0x10, pad[3]}`。无前驱/后继 = `0xffffffff`。
  - **ResourceLifetimeRecord（0x18 = 24B，O）**：`{first_pass PassEndpoint{tag u32, pass_index u32}, last_pass 同, first_access_index u32, last_access_index u32}`。
  - **lifetime 事件编码（O）**：`(pass_index << 17) | resource_index`，release 事件再 `| 0x1_0000`；结果**插入排序**（<21 项手写、≥21 走 sort）。
- **ResourceRegistry（0x108B，O）**：`resources Vec<4B/{u16 backing_index@0, u8 @2, u8 reused@3}>@+0x00`、`resource_initial_access Vec<u8>@+0x18`、`backing_resources Vec<RegistryResource 0x40B>@+0x30`（`{tag u64@0, payload 0x38@8}`，tag 0/1=纹理/外来纹理、2/3=缓冲/外来缓冲、4=Other、5=Empty）、`exported 集@+0x48`、**0x18-stride per-resource 记录@+0x78**（record_batch 用它查 prev access）。
- **CompiledRenderGraph（0x218B，O）**：`+0x00..0x90` 图记录段（execute_compiled 拷给 ExecutedRenderGraph）、`+0x90` 5 个 qword Pass 头、`+0xa8..0x1b0` ResourceRegistry（0x108B）、`+0x1b0..0x200` SynchronizationScope（0x50B）、`+0x200..0x218` `Vec<u32>` 待发初始过渡资源索引 `{cap@0x200, ptr@0x208, len@0x210}`。record_batch 另用 `+0x98` Pass 数组（0x40-stride）、`+0xf8/+0x100` accesses、`+0x128/+0x130` per-resource 表、`+0x1b0` barrier scope。
- **Batch（O，0x18B）**：`{start u64@0, end u64@8, export_barriers u64@0x10}`；BatchCompiler：`{batches Vec, current_range start@0x18/end@0x20, current_export_barriers@0x28}`（flush_batch 0x140be9600 实证）。
- **GraphExecutionRecorder（O，部分）**：`+0x00` 帧提交计数（置 1，非 0 即 panic）、`+0x28` pending submissions Vec（0x18-stride）、`+0x31` batch_per_pass 标志、`+0x40..0x58` commands Vec（CommandBuffer 0x58B 元素，ptr@+0x40/cap@+0x48/len@+0x50）、`+0x108` 帧状态字节、`+0x170` PassPgoFrames（previous@+0x48 / current@+0x88，各 0x40B）、`+0x178` GraphicsDevice。
- **PassPgoStats（0x40B，O）**：`{HashMap<PassPgoNameId,u64>（0x30B：table@0, mask@8, …@0x10, len@0x18）, total_ns@0x30, max_ns@0x38}`。
- **TemporalRenderGraphRecorder（O，部分）**：`HashMap<Ustr, Record>@+0xb0`（table@+0xb0/mask@+0xb8/len@+0xc8，**Record 0x68B：key Ustr@-0x68、state u32@-0x60、handle u32@-0x5c…**）；`get_or_create_temporal` 输出（0x28B）：`{4×u64, u32@+0x20, tag u8@+0x24}`，tag 0=fresh/1=reused/2=error。
- **RenderPassApi（O，部分）**：`*api` 对象含 `cb@+0x48`、`stencil_ref 缓存@+0x56`；命令直接经 device vtable：+0x2e0=vkCmdSetViewport、+0x2e8=SetScissor、+0x320=SetStencilReference、+0x330=BindIndexBuffer、+0x338=BindVertexBuffers、+0x370=CopyBuffer、+0x3a0=FillBuffer、+0x3a8=ClearColorImage、+0x410=PushConstants。
- **RenderGraph（0x110B，I）**：`resources Vec<ResourceOrigin 0x48B>@+0x00`、`retire callbacks Vec（0x48B 闭包）@+0x08/+0x10`、`passes/exports Vec@+0x18..`；on_retire（0x140be9420）逐条调闭包并清零 +0x28/+0x40。
- **ExportedRenderGraphResources（I）**：`Vec<ExportedResource>` 及 HashMap 位于 +0x20（take_resource 0x140be9c80 经 hashbrown 移除）。
- **ExecutedRenderGraph（0xd8B，O）**：new（0x140bea180）把 0x90B 图记录 + 9 个 qword 导出容器写进 `+0x00..+0x90` 与 `+0x90..+0xd8`；retire（0x140be9d00）跑退役回调（`+0x08` Vec、0x48B 闭包）并回收导出。

### 1.3 处理链图（帧级，O）

```
tiny_glade render (0x1404e8010)
  R4: RenderGraphState::compile<E=ResourceExportHandle<Texture>,UD=PassResources,closure>
       加载图 0x1401647b0 / 世界图 0x140161cd0
       ├─ 闭包前段: import_texture_presets_to_rg / GpuWorld::import_to_rg / render_luts /
       │           reset_indirect_args / DynamicConstants::allocate / into_uniform_and_storage_buffer_binding
       ├─ RenderGraph::create_raw_resource ×4
       ├─ render_world (0x1404ec110): 54 pass，GetOrCreateTemporal*（键=Ustr）、PassBuilder::render、PassResources::new_from
       ├─ 闭包尾部: Texture::export(0x140bd6380) / RenderContext::finish / export_preset_textures_from_rg /
       │           AssetSsboLibrary::export_from_rg / GpuWorld::export_from_rg(0x140992220, 最后一个 game 函数)
       ├─ TemporalRenderGraphRecorder::export_temporal(0x1404f7ca0)   ← 第一个 rhapsody 函数
       ├─ ResourceInfo::calculate(0x140530430)                        （clear→extract_creation_flags→extend_usage→逐 pass extract_pass_resource_access→link_up_next→calculate_lifetime_events）
       ├─ ResourceRegistry::populate_resources(0x140be7ba0)           （池命中/创建/导入 + set_*_debug_name）
       └─ CompiledRenderGraph::new(0x1402c4280)                       （组装 0x218B + 待发初始过渡）
  R5: GraphExecutor::begin_frame(0x140beabd0)（puffin scope、top-level timestamp、弹在飞帧、建帧命令缓冲）
  R8: GraphExecutionRecorder::execute_compiled(0x1400fa190)
       ├─ BatchCompiler::new + batch_per_pass 逐 pass flush + flush_batch(pass_count)
       ├─ submit_pending_cbs(0x140bebdf0)（清上一批）
       ├─ 每 batch: ensure_active_cb(0x140bebc90) → CompiledRenderGraph::record_batch(0x1402c1d40)
       │     ├─ 待发初始过渡屏障（+0x200 Vec 逐资源 image/buffer_barrier → issue_pipeline_barrier）
       │     ├─ budget=clamp(max,total/16,total/2) 贪心分块（PassPgoStats::duration_ns）
       │     ├─ [device+0xabb]&1 或全局覆写 0x14362c228 → 串行 FUN_1402c3240 或并行:
       │     │     bevy_tasks::ComputeTaskPool::get + TaskPool::scope_with_executor_inner(0x1401058a0)
       │     │     （块=一条命令缓冲，块内逐 pass：屏障→回调→PassPgoDuration）
       │     └─ 结束本批命令缓冲（CommandBuffer::end）; 最后命令 buffer 发 export 屏障（record_postamble）
       └─ ResourceRegistry::into_exports(0x140be9040) → ExecutedRenderGraph::new(0x140bea180)
  R13: GraphExecutionRecorder::execute(0x1400fb5b0，展示图)
       └─ RenderGraphState::compile<(),PassResources,execute_presentation_render_graph::closure>(0x140166510)
            → execute_compiled → ExecutedRenderGraph::retire
  R15: GraphExecutionRecorder::end_frame(0x140bec400)
       └─ ensure_active_cb → 回收已提交帧（atomic CAS 时间线）→ vtable+0x400/+0x408 结束+提交
            → submit_pending_cbs → PassPgoFrames 前后帧交换（previous↔current、current 清零）
  R19/20: last_known_frame_duration_ns(0x140beaae0) / wait_gpu_queue_trim(0x140bea680)
```

---

## 2. 逐函数表

列：函数（骨架名） | 地址 | role | 算法/行为要点 | verdict | evidence

### 2.1 rg/state.rs（跨帧编译）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `RenderGraphState::compile<ResourceExportHandle<Texture>,PassResources,compile_main_render_graph::closure_env$0>` | 0x140161cd0 | 世界图编译（主链核心） | 前段：取 `Option<RenderGraph>`（tag 置 i64::MIN）、拷 swapchain 0x88B、`get_resource_id` 取 World 资源；闭包 = render_world 54 pass + 尾部导出（§1.3）；后段：export_temporal → calculate → populate_resources → CompiledRenderGraph::new；成功返回 **0x218B `ResourceExportHandle<Texture>`**（= 编译图对象）；失败走 on_retire + 逐对象 drop 后返回 | O | 0x140161cd0.c 全文 1923 行；resolved-calls（export_temporal/calculate/populate/new 单实例） |
| `…::compile<…,compile_loading_screen_render_graph::closure_env$0>` | 0x1401647b0 | 加载图编译 | 同构，闭包 = 加载图（image::open→to_rgba8→create_texture→vermeer/mei/loading_screen pass，**不调 render_world**）；同样导出序列 → calculate → populate → new | O | 0x1401647b0.c 全文；skeleton state.rs 依赖注解（calculate 0x140530430 / export_temporal 0x1404f7ca0 / CompiledRenderGraph::new 0x1402c4280） |
| `…::compile<tuple$<>,PassResources,execute_presentation_render_graph::closure_env$0>` | 0x140166510 | 展示图编译（execute 内部） | 返回 `()`；产出 0x218B 编译图直交 execute_compiled；含 Texture::export(0x140bd6380) / import_unowned(0x140bd60a0) / populate_resources / on_retire / DynamicConstants | O | 0x140166510.c（本轮补 41KB）；execute 0x1400fb5b0 调用点 |
| `RenderGraphState::impl$0::default` | — | 构造 | 全零初始化 | I | rg.json |

### 2.2 rg/executor.rs（帧执行链）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `GraphExecutor::new` | 0x140bea2d0 | 构造 | device vtable+0x168 查询 → create_buffer(0x140c03460)（PGO/查询缓冲，desc 0x10B usage 0x4000000000002）；lazy 全局初始化 Arc 型 device（FUN_140d4d740 静态 +0x8/+0x10）；填 ≥0xc8B 字段 | O | decomp-0x140bea2d0 |
| `GraphExecutor::begin_frame` | 0x140beabd0 | 帧开始 | puffin scope + gpu_profiler begin/end top-level marker；弹在飞帧队列（0x90B/条，device 回收）；为帧建首条命令缓冲（CommandBuffer::new 0x140bef4a0 + begin 0x140bed230，标签 "begin frame"） | O | decomp-0x140beabd0（582 行） |
| `GraphExecutor::wait_gpu_queue_trim` | 0x140bea680 | 帧尾等待 | 若有待清命令且帧数>1：取最后一帧记录（0x90B/条）→ device wait（0x140bfcf80）；puffin 收尾 | O | decomp-0x140bea680（本轮补） |
| `GraphExecutor::wait_submits_done` | — | 同步 | inline；设备等待 | I | skeleton |
| `GraphExecutor::last_known_frame_duration_ns` | 0x140beaae0 | 帧时长查询 | 实际返回 **bool**（PDB 名与行为错位）：读 PGO 查询缓冲 `Buffer::mapped_slice_mut`（0x140bd4cd0）取差值，乘 device+0x658+0x3a8 浮点系数后与阈值（_UNK_1429a25a8）比较 | O | decomp-0x140beaae0 |
| `GraphExecutor::begin/end_top_level_timestamp_query` | — | 时间戳 | inline；0x140c11090/0x140c110f0/0x140c11150（ActiveProfiler） | O | begin_frame 内联调用点 |
| `GraphExecutionRecorder::execute_compiled<PassResources>` | 0x1400fa190 | **录制/提交链核心** | BatchCompiler::new→batch_per_pass 逐 pass flush→flush_batch(pass_count)；submit_pending_cbs 清上一批；`SwapchainLeaseSync::into_sync_data`（0x140c0f040）记帧同步（+0x28 Vec、帧计数断言）；逐 batch：ensure_active_cb→record_batch；批次间 submit_pending_cbs；尾部 into_exports→ExecutedRenderGraph::new，drop `RenderGraphRecord<PassResources>` | O | 0x1400fa190.c（554 行）；resolved-calls（flush_batch×2/submit×2/ensure/record_batch/into_exports/ExecutedRenderGraph::new/drop RenderGraphRecord） |
| `GraphExecutionRecorder::execute<tuple$<>,PassResources,presentation_closure>` | 0x1400fb5b0 | 展示图执行 | 内部 compile(0x140166510)→execute_compiled→ExecutedRenderGraph::retire；错误路径 drop ManagedHandle | O | 0x1400fb5b0.c |
| `GraphExecutionRecorder::ensure_active_cb` | 0x140bebc90 | 保证活动命令缓冲 | 无活动（+0x50==0）时 CommandBuffer::new(device,label"rg executor cb",device+0xad4 标志)+begin，写 +0x48/+0x50 | O | decomp-0x140bebc90 |
| `GraphExecutionRecorder::submit_pending_cbs` | 0x140bebdf0 | 提交命令缓冲 | 取 commands Vec（+0x40）；末条状态==1 时 device vtable+0x2c8（end cb）；构造提交（0x140c04e20/0x140bf8980）；20ms/错误阈值分支 puffin scope | O | decomp-0x140bebdf0 |
| `GraphExecutionRecorder::end_frame` | 0x140bec400 | 帧结束 | ensure_active_cb；处理 pending submissions（原子 CAS 收时间线）；vtable+0x400 结束、+0x408 提交；submit_pending_cbs；**PassPgoFrames 交换**（+0x170 结构 +0x48↔+0x88 各 0x40B，current 清零 0xff）；0x140be0590 收尾 | O | 0x140bec400.c（382 行） |
| `ExecutedRenderGraph::new` | 0x140bea180 | 构造 | 0x90B 图记录 + 9 qword 导出容器 | O | decomp-0x140bea180 |
| `ExecutedRenderGraph::retire` | 0x140be9d00 | 退役 | 处理导出（跳表）；跑退役回调（+0x08 Vec，0x48B 闭包）；回收导出容器；拷 0x90B 到输出 | O | decomp-0x140be9d00 |
| `ExportedRenderGraphResources::take_exported` | — | 全取导出 | inline；mem::take | I | skeleton |
| `ExportedRenderGraphResources::take_resource` | 0x140be9c80 | 按逻辑取导出 | hashbrown 移除（+0x20 结构）；Result 判 err==6 panic | O | decomp-0x140be9c80 |
| `PassPgoNameId::from_name` | 0x14067dd80（NF） | 名称→ID | 编译实例存在；rebuild FNV-1a 为候选（原 hash 算法 U） | U | rg.json；rebuild executor.rs |
| `PassPgoDuration::new` | 0x140be9c70（NF） | 构造 | 编译实例存在 | I | rg.json |
| `PassPgoStats::duration_ns` | 0x140bea1e0 | 按名查耗时 | hashbrown get；未命中返 0（分块按 0 计） | O | decomp-0x140bea1e0 |
| `PassPgoStats::reset` / `extend` | — | 统计维护 | inline；reset 清表+total/max；extend 并集 | I | rebuild；end_frame 交换逻辑佐证 |
| `GraphExecutionRecorder::impl$6::drop` | — | 析构 | 编译实例；销毁命令缓冲 | I | rg.json |

### 2.3 rg/compiled.rs（编译图 + 并行录制）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `CompiledRenderGraph::new<PassResources>` | 0x1402c4280 | 组装编译图 | 由 ResourceInfo 生命周期事件构建 **待发初始过渡 Vec**（+0x200）；拷 0x90B 图记录/Pass 头/0x108B Registry/SynchronizationScope（0x140bd6650 构造）到 0x218B 输出 | O | decomp-0x1402c4280（本轮补） |
| `CompiledRenderGraph::record_batch<PassResources>` | 0x1402c1d40 | **并行录制核心** | 见 §1.3。前段清空+0x200 Vec 逐资源发初始屏障（prev access 表 +0x128 0x18-stride→access 记录 +0xf8 0x14-stride→image/buffer_barrier 0x140bd66a0/0x140bd6810→issue_pipeline_barrier 0x140bee030）；[device+0xabb] 取反 &1 或全局覆写 **0x14362c228**（==2 时用 device 值）选串行 FUN_1402c3240 / 并行；并行预算 `total>>4 / total>>1 / max` clamp、贪心按 PassPgoNameId::from_name + duration_ns 累积分块；块>1 走 ComputeTaskPool；每条命令缓冲收束（CommandBuffer::end 0x140bed3d0），末条发 postamble 屏障并 end_label | O | 0x1402c1d40.c（661 行）；resolved-calls（track×4/borrow×3/barrier×2/issue×2/scope_with_executor_inner/ComputeTaskPool::get） |
| `CompiledRenderGraph::impl$1::record_batch::closure$1<PassResources>` | — | 并行块回调 | 编译实例；bevy_tasks 并行块内逐 pass 回调 | O | rg.json；record_batch 并行段 |
| `CompiledRenderGraph::add_pre_pass_barriers<PassResources>` | — | 批前屏障 | 编译实例 | I | rg.json |
| `CompiledRenderGraph::compute_batches` | — | inline | 编译图内批次计算（CompiledRenderGraph::new 内 0x1402bf880 对应段） | I | skeleton line 123 |
| `CompiledRenderGraph::gather_batch_preamble_barriers` | — | inline | 批首部屏障收集 | I | skeleton line 352 |
| `CompiledRenderGraph::record_postamble` | — | inline | 批末导出屏障（末条命令缓冲 end 前） | O | record_batch 末段（end_label + per-resource 0x30B 屏障表） |
| `BatchCompiler::new` | 0x140be95e0（NF） | 构造 | 编译实例；全零 | I | execute_compiled 调用点 |
| `BatchCompiler::flush_batch` | 0x140be9600 | 关批 | `range.start==boundary` 早退；否则 push Batch{start,end,export_barriers}（0x18B） | O | decomp-0x140be9600 |
| `BatchCompiler::compile` | — | inline | 关批逻辑 | I | skeleton line 75 |
| `pass_access_range_indices` | 0x140be9690（NF） | 辅助 | 编译实例（Pass 访问范围索引） | U | rg.json |

### 2.4 rg/resource_info.rs（资源生命周期折算）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `ResourceInfo::calculate<PassResources>` | 0x140530430 | **图→访问/生命周期主算法** | clear()；按图资源数扩容；extract_creation_flags；extend_usage_flags_for_exports；逐 pass（0x40B/条）extract_pass_resource_access（内联：per-resource 0x18B 记录更新 first/last access、0x14B access 记录追加、usage 折算）；link_up_next_access_chains；calculate_lifetime_events | O | 0x140530430.c（230 行，5 方法调用点全证） |
| `ResourceInfo::clear` | 0x140bd4410 | 清空 | 5 个 Vec 长度清零 + 事件表 0xff 复位 | O | decomp-0x140bd4410 |
| `ResourceInfo::extract_creation_flags` | 0x140bd44b0 | 创建期 usage | 逐图资源：tag==0（Create）时按 descriptor 判别（+0x36）把 image@+0x18 / buffer@+0x20 创建标志写入对应 usage 表 | O | decomp-0x140bd44b0 |
| `ResourceInfo::extend_usage_flags_for_exports` | 0x140bd4560 | 导出 usage 并入 | 逐导出声明：dependency mask → image/buffer_access_mask_to_usage_flags 并入 usage 表；记导出索引 | O | decomp-0x140bd4560（3.6KB） |
| `ResourceInfo::extract_pass_resource_access` | — | 单 pass 折算 | inline（见 calculate 内联体）；记录 prev 链、first/last pass、first/last access、usage | O | calculate 内联段（行 100–215） |
| `ResourceInfo::link_up_next_access_chains` | 0x140bd47d0 | 链接 next | 逐 access：prev!=NO_ACCESS 时写 `accesses[prev].next_access=当前` | O | decomp-0x140bd47d0 |
| `ResourceInfo::calculate_lifetime_events` | 0x140bd4860 | 生命周期事件 | first/last pass tag 均==1 时 push `pass<<17|idx`（last 再|0x10000）；**插入排序**（<0x15 手写，≥走 0x140958a90） | O | decomp-0x140bd4860 |
| `image_access_mask_to_usage_flags` | 0x140bd49d0 | 访问掩码→ImageUsage | 精确匹配表：0x20→4、0x40→8、0x80/0x100/0x180→0x10、0x200/0x400/0x600→0x20、0x220→0x24、0x800→1、0x1000→2、0x18000→8；未命中 panic | O | decomp-0x140bd49d0（**rebuild 表与此不符，见 §3**） |
| `buffer_access_mask_to_usage_flags` | 0x140bd4b00 | 访问掩码→BufferUsage | `(m&6)<<5 | (m&1)<<8`、`|(m&8)<<1`、`m&0x18060!=0 时 |0x20`、`|(m>>11)&3` | O | decomp-0x140bd4b00（rebuild 一致 ✓） |
| `ResourceAccessIndex::{index,try_index,valid}` | — | 访问索引包装 | inline；valid=!=NO_ACCESS | I | rebuild |
| `ResourceLifetimeEvent::{is_acquire,resource_idx}` | — | 事件解码 | inline；release=bit16 | I | rebuild |
| `impl$13/impl$23::from` 等 | — | 转换 | 编译实例 | I | rg.json |

### 2.5 rg/resource_registry.rs + resource_pool.rs

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `ResourceRegistry::resource` | 0x140be78b0 | 逻辑→物理解析 | 越界 panic；返回 `(&backing 0x40B, initial_access u8, reused bool)` | O | decomp-0x140be78b0 |
| `ResourceRegistry::image_from_raw_handle` | 0x140be7940 | 取纹理 | backing tag<2 返回 payload+8；否则 panic | O | decomp-0x140be7940 |
| `ResourceRegistry::image` | — | 别名 | inline；同 image_from_raw_handle | I | rebuild |
| `ResourceRegistry::buffer` | 0x140be79d0 | 取缓冲 | tag 2/3 返回 payload+8；否则 panic | O | decomp-0x140be79d0 |
| `ResourceRegistry::image_view` | 0x140be7a60 | 取视图 | 校验纹理格式（desc+0x1c vs device+0x648+0xa50）后现建 VkImageView（0x140bdc5d0） | O | decomp-0x140be7a60 |
| `ResourceRegistry::populate_resources` | 0x140be7ba0 | **物理资源填充** | 按生命周期事件（跳过 release）：Create→usage 非 0 时 `ResourcePool::take_texture(0x140be6b90)/take_buffer(0x140be71e0)` 命中或 `create_texture(0x140c07950)/create_buffer(0x140c03460)` + `set_texture_debug_name(0x140bfe650)/set_buffer_debug_name(0x140beec30)`；Import→直接持 token；错误 9/10 判 fatal、<9 返回 PopulationFailure（调用方 on_retire+drop）；返回 bool Result | O | decomp-0x140be7ba0（30KB）；resolved-calls |
| `ResourceRegistry::into_exports` | 0x140be9040 | 剥离导出 | 从导出位集（+0xd8）取导出逻辑→物理，backing 中 Replace Empty 后**原地压缩**；返回 `ResourceExports{remaining Vec, exported HashMap<u16,RegistryResource>}`；drop 导出集 | O | decomp-0x140be9040 |
| `ResourcePool::take_texture` | 0x140be6b90 | 池取纹理 | hashbrown get(desc)→Vec::pop | O | decomp-0x140be6b90 |
| `ResourcePool::insert_texture` | 0x140be6d70 | 池还纹理 | hashbrown entry + push | O | decomp-0x140be6d70 |
| `ResourcePool::take_buffer` | 0x140be71e0 | 池取缓冲 | 同上 | O | decomp-0x140be71e0 |
| `ResourcePool::insert_buffer` | 0x140be7320 | 池还缓冲 | 同上 | O | decomp-0x140be7320 |
| `ResourcePool::clear` | 0x140be76f0 | 清池 | 两个 HashMap 全清（drop 全部资源） | O | decomp-0x140be76f0 |

### 2.6 rg/pass_api.rs（21 方法命令下发）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `RenderPassApi::begin_render_pass` | 0x1416f6e90 | 开始渲染通道 | 查 render pass 缓存（device+0x648 HashMap，0x1418aef70）；逐附件经 registry 解析纹理并校验格式（+0x1c vs device+0x648+0x14a）；组 VkRenderPassBeginInfo | O | decomp-0x1416f6e90（16KB） |
| `RenderPassApi::end_render_pass` | 0x1416f7850（NF） | 结束 | 编译实例 | I | rg.json |
| `RenderPassApi::set_default_view_and_scissor` | 0x1416f7870 | 默认视口/裁剪 | viewport `{0, height, width, -height}`（负高翻转 Y）经 vtable+0x2e0；scissor 全幅经 +0x2e8 | O | decomp-0x1416f7870 |
| `RenderPassApi::set_stencil_ref` | 0x1416f7920 | 模板参考值 | 与 +0x56 缓存不同才发 vtable+0x320（face=3） | O | decomp-0x1416f7920 |
| `RenderPassApi::set_line_width` | 0x1416f7960（NF） | 线宽 | 编译实例；device+0xad9 门控 | I | rebuild |
| `RenderPassApi::bind_vertex_buffer_by_binding_point` | 0x1416f7980 | 绑顶点缓冲 | 0x1418aed30 解析 buffer→vtable+0x338 | O | decomp-0x1416f7980 |
| `RenderPassApi::bind_index_buffer` | 0x1416f79f0 | 绑索引缓冲 | 同上→vtable+0x330 | O | decomp-0x1416f79f0 |
| `RenderPassApi::push_constants` | 0x1416f7a50 | 推送常量 | pipeline+0x30 判 0 图形(0x1f)/1 计算(0x20)；vtable+0x410 | O | decomp-0x1416f7a50 |
| `RenderPassApi::copy_buffer` | 0x1416f7ac0 | 拷贝缓冲 | 双缓冲校验后 vtable+0x370 | O | decomp-0x1416f7ac0 |
| `RenderPassApi::fill_buffer` | 0x1416f7c30 | 填充缓冲 | None→(0,WHOLE_SIZE)；显式范围校验 4B 对齐；vtable+0x3a0 | O | decomp-0x1416f7c30 |
| `RenderPassApi::clear_color_image` | 0x1416f7d80 | 清颜色图 | 透明零色；mips=texture+0x2c、layers=+0x28；vtable+0x3a8 | O | decomp-0x1416f7d80 |
| draw/draw_indexed/draw_indirect[_count]/draw_indexed_indirect[_count]/dispatch_groups_3d/dispatch_threads_3d/dispatch_indirect | — | 绘制/计算 | inline（编译实例）；命令直接映射 vkCmd*；dispatch_threads 先 div_ceil 再 dispatch_groups | O | rg.json；rebuild pass_api 对应 |
| `RenderPassApi::find_resource_access_type` | — | 查访问码 | 扫描当前 pass 访问，未声明 panic | I | rg.json line 16 |
| `resource::Ref::bind_view` / `bind_uniform_buffer` / `impl$5::bind` | — | 绑定 | 编译实例；bind_view 把 0x3cB TextureViewDesc 写进 RenderPassBinding | O | rg.json；CALL-CHAIN §3.1 |

### 2.7 rg/pass_builder.rs（构图 DSL）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `PassBuilder::render<PassResources,dispatch_impl::closure>` 等 **6 实例** | — | 提交 pass | 每个游戏闭包一个单态化实例（dispatch_impl / draw_stream::render / reset_indirect_args / clear_rg_buffer / clear_rg_color_image / rg_buffer_copy_from_slice / handle_screenshot_opportunity 等实际 7 实例）；闭包签名 `impl FnOnce(u64)->bool`（CALL-CHAIN §3.1，rebuild 改 FnMut(PassContext)） | O | rg.json；recorder impl$1::render 7 实例对应 |
| `PassBuilder::add_resource_access<PassResources>` | — | 声明访问 | 进访问账本（0x1409dca90/0x140533b10 区，access_ledger 重建）：同码同组去重、{0x17,0x1d}→0x1e 原地合并、{0x1a→0x02}、{0x19/0x1a→0x37} 升级追加、不兼容 panic | O/I | access_ledger.rs；render_world 调用面 |
| `PassBuilder::write_impl` / `rg_mut` / `impl$0::drop` | — | 辅助 | inline；write_impl=add_resource_access | I | rebuild |
| `impl$2::{write,write_concurrent,raster,read,raster_read}` | — | 访问分类 | inline（rebuild access_ledger 语义） | I | rg.json |

### 2.8 rg/temporal.rs（跨帧时态）

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `impl$11/impl$12::get_or_create_temporal<PassResources,String/&str>` | 0x1404f6e50 | **时态纹理获取** | 键 `Ustr::from(ptr,len)`（0x140c0f640）；hashbrown get（0x1405b1640）；命中 Exported(state==2) 且 desc 匹配 → `Texture::import_unowned`(0x140bd60a0) 复用（tag=1）；未命中 → `Texture::create`(0x140311cc0)+import_unowned+insert（tag=0）；desc 不匹配/状态错误走 error 路径（tag=2） | O | 0x1404f6e50.c（380 行） |
| `impl$20::get_or_create_temporal_pair<PassResources,&str>` | 0x1404f7450 | **双缓冲对** | 以 `"{name}:0"/"{name}:1"`（Ustr 拼后缀）各调一次 get_or_create_temporal；`get_and_flip_parity`(0x140beced0) 翻转；按 parity 排序 first/second | O | decomp-0x1404f7450（本轮补） |
| `TemporalRenderGraphRecorder::export_temporal<PassResources>` | 0x1404f7ca0 | 图导出时态 | 逐记录：RawTexture→`Texture as ImportExportToRenderGraph::export`(0x140bd6380, **access 0x37**)、RawBuffer→Buffer export(0x140bd5120)；标记 Exported；回写 recorder 0xa8B | O | decomp-0x1404f7ca0（本轮补） |
| `TemporalRenderGraphState::get_and_flip_parity` | 0x140beced0 | 奇偶翻转 | hashbrown 查 parity map（+0x30）；命中 `new=1-old`；未命中 insert(key,0) 返 0 | O | decomp-0x140beced0 |
| `Parity::flip` / `Temporal::{into_inner,as_ref,split_pair}` | — | 包装 | inline；parity=1−v | I | rebuild |
| `impl$14::fmt` / `impl$3::from` / `impl$18::default` | — | 调试/转换 | 编译实例 | I | rg.json |

### 2.9 rg/resource.rs / rg/graph.rs / rg/binding.rs / rg/import_export.rs / rg/recorder.rs / rg/util.rs

| 函数 | 地址 | role | 要点 | v | evidence |
|---|---|---|---|---|---|
| `RuntimeResource::borrow` | 0x140bd6c80 | 借用载荷 | RegistryResource tag 跳表分派（0/1→纹理、2/3→缓冲、4→Other、5→panic） | O | decomp-0x140bd6c80 |
| `ResourceOrigin::ty` | 0x140bd6d10 | 类型码 | tag 跳表；tag0 时判 +0x36（7→1、8→panic、其余→0）；tag1/2 取 +8 | O | decomp-0x140bd6d10 |
| `ResourceHandle::raw` | 0x140833250（NF） | 取序号 | 编译实例 | I | rg.json |
| `Handle::{desc,raw,clone_unchecked}` | — | 句柄 | inline；clone_unchecked 免检复制 | I | rebuild |
| `validate_read_access`（impl$0/impl$1） | — | 校验 | 编译实例（rebuild：空操作） | I | rebuild |
| `RenderGraph::create_raw_resource` | 0x140be94d0 | 建图资源 | 扩容后 push 0x48B ResourceOrigin{name/desc/origin}，返回逻辑 ResourceHandle | O | decomp-0x140be94d0 |
| `RenderGraph::create_resource_handle` | — | 别名 | inline；同 create_raw_resource | I | rebuild |
| `RenderGraph::on_retire` | 0x140be9420 | 退役回调 | 逐条调 0x48B 闭包并清零 +0x28/+0x40 | O | decomp-0x140be9420 |
| `ExportableResource::{raw,index}` | — | 导出载体 | inline；raw=ResourceHandle | I | rebuild |
| `rg_binding_to_descriptor_binding` | 0x14205eff0 | 槽→描述符 | PassResourceSlot 判别跳表（*in_R9−3<3→1..3 else 0） | O | decomp-0x14205eff0 |
| `PassResourceSlot::new` | — | 槽构造 | inline | I | rebuild |
| `import_export::{Texture,Buffer} as ImportExportToRenderGraph::{import_unowned,import_owned,export,from_owned_rg_resource}` | 0x140bd60a0 / 0x140bd6380 / 0x140bd63f0 / 0x140bd4fc0 | 图↔物理资源 | 原 impl$0（buffer）/impl$1（texture?）两套：import 带 `(token, access)`、export 带 access（temporal=0x37）；from_owned_rg_resource（0x140bd63f0）执行后把物理对象从导出容器取出 | O | resolved-calls（render 调用点）；skeleton import_export.rs |
| `recorder::RenderGraphRecorder::{new,create,add_pass,record_pass,import_unowned,import_owned,export}` | — | 录制器 | 编译/内联实例；原对象名 **`RenderGraphRecord<PassResources>`**（execute_compiled drop_in_place 实证）；与 rebuild 的 `Recorder` 同名不同物 | O | resolved-calls drop_in_place；skeleton recorder.rs |
| `recorder::impl$1::render<…,PassResources>` **7 实例** | — | 记录 pass | 与 pass_builder render 6+1 实例对应；回调 FnOnce(u64)->bool | O | rg.json |
| `PassResourceRef::resource_index` | 0x140bece40（NF） | 取资源号 | 编译实例 | I | rg.json |
| `ResourceSyncGroup::new` | 0x140bece50（NF） | 组号分配 | 编译实例；`AtomicU32 fetch_add` 从 2 起（access_ledger 重建，O/I） | I | access_ledger.rs |
| `ResourceSyncGroup::is_equivalent` | 0x140bece60（NF） | 等价 | 编译实例；u32 相等 | I | access_ledger.rs |
| `PassResourceAccessType::new` | 0x140b25c90（NF） | 构造 | 编译实例 | I | rg.json |
| `PassResourceAccessType::is_write` | — | 写判定 | inline；经 vk_sync_fork AccessType::is_write（0x140c10e10） | O | can_be_concurrent_with 调用点 |
| `PassResourceAccessType::can_be_concurrent_with` | 0x140bece70 | 并发判定 | 双非写→access 相等；否则 access 相等且双组非 0 且相等 | O | decomp-0x140bece70 |
| `util::impl$0::same` | — | 等价 | inline | I | rg.json |

---

## 3. 原版 ↔ rebuild 对照表（rg 层）

逐项对照 CALL-CHAIN §3.1，重点复核"半对上"项。✓=对齐 / 半=部分偏差 / ✗=缺或错。

| 原函数/类型 | 地址 | rebuild 对应 | 判定 | 差异说明 |
|---|---|---|---|---|
| `RenderGraphState::compile` ×3 | 0x140161cd0/0x1401647b0/0x140166510 | `state.rs::compile_frame` + `PersistentCompileState::take_for_compile`（注释引用 0x140166510 段） | **半** | 原 ABI：`compile(state, swapchain_ctx, closure) -> 0x218B`（前两实例返回 `ResourceExportHandle<Texture>`）；rebuild `compile_frame(graph, backend, dependency) -> Result<CompiledFrame>` 无 `ResourceExportHandle` 类型、无三个游戏闭包实例、**Result 语义 vs 原 OOM 降分辨率路径**（render 0x1404e8010 0x1404e8a3d..0x1404e9516 重试，panic 串 0x1429ebc17 族）。§CALL-CHAIN 差距#1 未闭合 |
| `ResourceInfo::calculate` + 5 方法 | 0x140530430 等 | `resource_info.rs` | **✓ 主体 / 半 API** | 行为逐段对上（access 记录、生命周期、usage 折算、事件排序）；原签名 `(&mut self, &graph, passes, pass_count)`，rebuild `(origins, exports, passes, mask闭包)` 重排——等价格式不同。**注意 rebuild 把 `exported_indices`/`lifetime_events` 挂 ResourceInfo 上，原事件 Vec 在 +0x78、导出位集在 +0x90** |
| **`image_access_mask_to_usage_flags`** | 0x140bd49d0 | `raw/resource_usage.rs` | **✗** | **rebuild 表与原机器码不符**：原 0x20→4、0x40→8、0x80/0x100/0x180→0x10、0x200/0x400/0x600→0x20、0x220→0x24、0x800→1、0x1000→2、0x18000→8；rebuild 0x20→SAMPLED、0x40→STORAGE、0x80→COLOR_ATTACHMENT、0x800→TRANSFER_SRC、0x1000→TRANSFER_DST… 数值全错位。直接影响实际创建 TextureDesc 的 usage——**高优先修正项** |
| `buffer_access_mask_to_usage_flags` | 0x140bd4b00 | 同文件 | **✓** | 位运算逐位一致 |
| `ResourceRegistry::{populate_resources, into_exports, resource, buffer, image_from_raw_handle, image_view}` | 0x140be7ba0 等 | `resource_registry.rs` + `graph.rs` populate | **✓ 主体** | 行为对上；偏差：原 exported 是**位集**（into_exports 从 +0xd8 位集取），rebuild 用 `HashSet<u16>`；rebuild `populate_resources` 参数面 `(info,declarations,pool,backend)` vs 原 `(registry, graph, pool, device)->bool Result`（同功能）；原 Create+usage==0 → Empty 与 rebuild 一致 |
| `ResourcePool::{take/insert_texture,take/insert_buffer,clear}` | 0x140be6b90..0x140be76f0 | `resource_registry.rs::ResourcePool` | **✓** | HashMap<Desc,Vec> + LIFO pop，布局一致 |
| `CompiledRenderGraph::new/record_batch/compute_batches/add_pre_pass_barriers/record_postamble/gather_batch_preamble_barriers` | 0x1402c4280/0x1402c1d40 | `compiled.rs` + `parallel.rs` | **✓ 核心 / 半布局** | 分块算法（`clamp(max,total/16,total/2)`、贪心）与 bevy_tasks 并行逐条对上（parallel.rs `original_budget`/`original_chunks` 已按 0x1402c1d40 内联体重构）；**偏差：原 Batch=0x18B {start,end,export_barriers} 且 execute_compiled 内由 BatchCompiler 现算；rebuild `Batch{pass_range:Range,export_barriers}` 语义同但布局不同**；rebuild 无 `CompiledRenderGraph` 本体（`CompiledGraphSnapshot`/`CompiledGraph` 为候选名） |
| `BatchCompiler::{new,flush_batch,compile}` | 0x140be9600 | `compiled.rs::BatchCompiler` | **✓** | flush_batch 早退/关批逻辑一致 |
| `GraphExecutor::{new,begin_frame,wait_gpu_queue_trim,last_known_frame_duration_ns,begin/end_top_level_timestamp_query}` | 0x140bea2d0 等 | `executor.rs` | **✓ / 半** | begin_frame/end_frame/wait_gpu_queue_trim/PassPgo 交换对齐；**`last_known_frame_duration_ns` 原返回 bool**（rebuild 无对应，且原读 PGO 查询缓冲的语义与 rebuild `ExecutionStats` 不同）；rebuild `ExecutionChain` 是整合候选名 |
| `GraphExecutionRecorder::{execute_compiled,execute,submit_pending_cbs,end_frame,ensure_active_cb}` | 0x1400fa190/0x1400fb5b0/0x140bec400 | `executor.rs::ExecutionChain` + `recorder.rs::Recorder` | **✓ 主体 / 半名** | 提交/录制链对齐；**原录制器对象 = `RenderGraphRecord<PassResources>`**（drop_in_place 实证）；rebuild `Recorder`（`record_batch`/`plan_chunked_batch`/`finish_chunked_batch`）是行为重建同名不同物；原命令缓冲 0x58B、rebuild `Command{handle,sequence,state}` 候选 |
| `ExecutedRenderGraph::{new,retire}` / `ExportedRenderGraphResources::{take_exported,take_resource}` | 0x140bea180/0x140be9d00/0x140be9c80 | `executor.rs` | **✓** | 行为对齐；原 ExecutedRenderGraph 0xd8B、retire 跑 0x48B 闭包，rebuild 等价 |
| `RenderGraph::{create_raw_resource,create_resource_handle,on_retire}` + `ExportableResource` | 0x140be94d0/0x140be9420 | `graph.rs` | **✓ 主体 / 半布局** | create_raw_resource 返回逻辑 handle 一致；原图资源 0x48B `ResourceOrigin{tag@0,payload@8,kind@0x36}`，rebuild `ResourceOrigin{descriptor,source}` 语义同、布局不同 |
| `PassBuilder::{add_resource_access,render×6,write_impl,rg_mut}` | — | `pass_builder.rs` + `access_ledger.rs` | **✓ 主体 / 半回调** | 访问账本规则（去重/合并/升级）已按 0x1409dca90 区重建；**原回调 `FnOnce(u64)->bool`，rebuild `FnMut(PassContext)->bool`**（CALL-CHAIN §3.1 明确偏差，差距#2） |
| `RenderPassApi` 21 方法 | 0x1416f6e90 区 | `pass_api.rs` | **✓** | vtable 偏移、0x1f/0x20 阶段掩码、负视口翻转、stencil 缓存、对齐校验全部对上；rebuild `Command` 枚举为候选表达 |
| `rg::binding::{PassResourceSlot::new, rg_binding_to_descriptor_binding}` | 0x14205eff0 | `binding.rs` | **✓ / 半布局** | 判别跳表对上；rebuild 槽带 offset/size/layout（原 RenderPassBinding 0x44B 布局未恢复，INDEX §5.9） |
| `rg::temporal::{get_or_create_temporal, get_or_create_temporal_pair, export_temporal, get_and_flip_parity}` | 0x1404f6e50/0x1404f7450/0x1404f7ca0/0x140beced0 | `temporal.rs::TemporalRenderGraphState` | **半** | 键 = **`ustr::Ustr`**（0x140c0f640 实证），rebuild 已用 Ustr ✓；行为（state 机、:0/:1 后缀、parity 翻转、export access 0x37）对齐；**原类型是 `TemporalRenderGraphRecorder<UD>` + trait `GetOrCreateTemporal(Pair)`**，rebuild 只有 `TemporalRenderGraphState`，无 recorder/无 trait 化 |
| `rg::resource::{Handle,Ref,ResourceHandle,ResourceOrigin,ResourceOrigin::ty,RuntimeResource::borrow}` | 0x140bd6c80/0x140bd6d10 | `resource.rs` | **✓ 主体 / 半** | 布局/跳表对上；rebuild `ResourceOriginLayout::ty`（payload[46] 判别）与 0x140bd6d10 一致 ✓；rebuild `Handle` 带 Descriptor（原 `Handle{raw,desc}` 同款） |
| `rg::recorder::RenderGraphRecorder / RenderGraphRecord<PassResources>` | — | `recorder.rs::Recorder` | **✗ 名 / 半行为** | 原对象名/类型参数与 rebuild 不同（见上） |
| `rg::import_export::{impl$0/impl$1 export, import_unowned, import_owned, from_owned_rg_resource}` + `raw::texture::Texture as ImportExportToRenderGraph` | 0x140bd4fc0/0x140bd60a0/0x140bd6380/0x140bd63f0 | `import_export.rs` | **半** | rebuild `import_owned/unowned(name,desc,token,access)` 参数面多了 name/desc（原 impl$1 为 `(&mut RG, token, access)` 更近 handle）；**无 `from_owned_rg_resource`、无 `ImportExportToRenderGraph` trait、无 `export` 两套** |
| `ResourceSyncGroup`/`PassResourceAccessType` | 0x140bece50/0x140bece60/0x140bece70 | `access_ledger.rs` + `resource_rules.rs` | **✓** | can_be_concurrent_with 0x140bece70 逐分支一致；sync group 从 2 起 fetch_add 一致 |

**核对结论（CALL-CHAIN §3.1 半对项）**：`RenderGraphState::compile`（半）保持；`temporal`（半）保持（recorder/trait 命名缺）；`RenderGraphRecorder/RenderGraphRecord`（半）保持；`import_export`（半）保持；`image_access_mask_to_usage_flags` 发现**新 ✗**；其余"对得上"项经机器码复核基本成立。

---

## 4. U 卡点清单

1. **compile 三实例的前段参数面**：`RenderGraphState::compile` 的 ABI 参数（swapchain_ctx/closure 捕获布局）只见到状态字段 +0x08..0x90 拷贝与 `Option<RenderGraph>` tag；`RenderGraphState` 完整字段名与布局 U。
2. **`ResourceExportHandle<Texture>` 类型形态**：0x218B 返回值确认为编译图对象（sret 拷贝），但该类型的 Rust 定义（是否=`CompiledRenderGraph` 包装）U。
3. **`last_known_frame_duration_ns` 行为错位**：PDB 名说"返回 ns"，机器码返回 bool（阈值比较）；rebuild 无对应方法，语义卡点。
4. **`BatchCompiler::new`/`pass_access_range_indices`/`PassPgoNameId::from_name`/`PassPgoDuration::new` 等 11 个 Ghidra NotFound**：`PassPgoNameId::from_name` 的原 hash 算法 U（rebuild FNV-1a 是候选）；`pass_access_range_indices` 用途 U。
5. **record_batch 的 batches 迭代字节**：execute_compiled 内批量状态字节（batch+4 与 0x02 比较）Ghidra 恢复模糊，确切语义 U（不影响 Batch 0x18B 布局结论）。
6. **`ResourceRegistry::resource` 返回的 byte@2**：`{u16 backing@0, u8@2, u8 reused@3}` 中 @2 的语义（initial-access 冗余/flag）U。
7. **CompiledRenderGraph 内 `+0x128` 0x18-stride per-resource 表**：与 `ResourceInfo::lifetimes`（0x18 同样）是否同一份/何时复制 U。
8. **RenderPassApi::begin_render_pass 附件解析细节**：render pass 缓存键（RenderPassDesc）字段与 0x44B RenderPassBinding 布局 U（INDEX §5.9）。
9. **`export_temporal` 里 Buffer 导出 `0x140bd5120`** 的精确 trait 归属（Buffer as ImportExportToRenderGraph）U。
10. **`TemporalRenderGraphRecorder` 完整字段**：map@+0xb0、+0xc8..+0x108 其余字段语义 U；Record 0x68B 中 -0x48..-0x14 字段（desc/token/access 拼装）未逐字段定名。
11. **`image_access_mask_to_usage_flags` 的输入 mask 语义**：mask 值（0x20..0x18000）与 vk_sync_fork AccessType 位的对应表未核对到 fork 源码，表数值本身已 O；rebuild 需按 0x140bd49d0 修正。
12. **`GraphExecutor`/`GraphExecutionRecorder` 结构体完整布局**：只恢复本域访问的字段；+0x60..0x170 区间字段 U。
13. **`last_known_frame_duration_ns`/`end_frame` 的 0x140be0590**：该收尾函数不在 rg 域，语义 U。
14. **compile 失败/CompileError 类型**：原错误经 `on_retire`+drop 传递（成功返回 0x218B），失败分支的 Result 载荷类型 U。

---

## 证据索引

- `domains/rg/decomp/`：57 个原始反编译（11 个 NotFound 标注在 summary.json）+ 本轮补 5 个（0x1402c4280 / 0x140166510 / 0x1404f7450 / 0x1404f7ca0 / 0x140bea680，dbid `tg_rh_rg`）。
- `domains/rg/rg-supp-decomp.json` / `rg-supp-out.json` / `rg-supp-console.log`：本轮补反编译 job（可复现）。
- `../decomp/*.c`：调用链调查的 compile×3 / execute / execute_compiled / record_batch / calculate / get_or_create_temporal / render_world / render（dbid `tg_rl2`）。
- `../resolved-calls.json`：各关键函数调用目标→PDB 名解析。
- `skeleton/B/crates/rhapsody/src/rg/*.rs`：PDB 元数据（函数/行号/依赖）。
- `reconstruction/rhapsody-rebuild/src/rg/**`：对照基准。
