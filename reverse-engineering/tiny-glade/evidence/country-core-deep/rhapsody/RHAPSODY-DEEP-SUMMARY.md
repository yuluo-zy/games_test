# rhapsody 全量函数级深挖汇总（RHAPSODY-DEEP-SUMMARY，2026-10-15）

基线：B patched EXE（SHA cf048e27…）；原始指令 oracle = 发行 RNE。机器码为权威，PDB 名可错位。
范围：**原版 rhapsody 466 compiled 物理函数**（849 符号）逐函数/数据结构/业务算法深挖 + 两轮审查闭合。
产出：5 份 `domains/<域>/ALGORITHMS.md`（共 2035 行）+ 5 份 `REVIEW-UNEXPLORED.md` + 跨域 `REVIEW-UNEXPLORED/MERGED.md` + 本轮闭合各域新增 §5/§6 + 大量 rne-read 字节级证据。

## 0. 结论速览

1. **覆盖**：466 compiled 名义覆盖 >98%（仅 3 个派生 fmt + 1 业务闭包无行，均登记）；有地址 239 全进表；真实反编译 + rne-read 补证使核心结论大多到 O。
2. **审查轮推翻了 3 条错误/过期结论**（防集成轮改错代码）：`image_access_mask_to_usage_flags` 与 ash 常量数值一致（作废）、`allocate_descriptor_set` 的 rebuild 已是空闲池复用（STALE 改 ✓）、`bind_descriptor_set` free fn 已存在。
3. **闭合轮把大量 U 落到 O**：Buffer 40B 全链、退役分派跳表、temporal 状态机、OOM 降分辨率、env 屏蔽链（语义翻转）、disposal 锁拓扑、refcount 原子宽度、get_access_info 字符表等 12 个 P0 全闭合。
4. **rebuild 对照**：既有实现大部分对得上（设备筛选/格式映射/命令缓冲/swapchain/compiled 录制逐字节对上），但存在**结构级偏差**（Buffer 40B vs 16B、GpuOwned 双泛型、disposal BTreeMap、PassPgoNameId hash、blocklist 空表、init_logging 缺失）+ 原版有而 rebuild 无（aftermath/vk-sync/dxgi/ray_tracing/import-export trait 层）。
5. **接线断点仍在 country-core 侧**：`rhapsody_contract` 占位类型（GraphicsDevice(u64)/Handle/ReadOnlyHandle/RenderPassDesc 键）与原 rhapsody 真实类型（Arc<GraphicsDevice>/rg::resource::Handle/ManagedHandle<T>）不对齐——见 §6。

## 1. 各域深挖成果

| 域 | ALGORITHMS | compiled | 覆盖 | 关键定案（O） | 剩余 U |
|---|---|---|---|---|---|
| rg | 437 行 | 107 | ✓86/△14/✗7（6 派生+into_sampled_rg_texture） | CompiledRenderGraph 0x218B、compile 三实例链、record_batch 并行（[dev+0xabb]&1/budget clamp）、ResourceInfo 布局、temporal Record 0x68B+状态机、OOM 降分辨率全链、add_pre_pass_barriers=0x1402c3d00、11 NF 全 rne-read 补证 | validate_read_access ×2、TemporalRecorder +0xc8..、PassPgoStats reset/extend |
| raw-core | 485 行 | 184 | 高（1 无行 closure$0） | **Buffer 40B** {*ctx@0,slot@8,off@0x10,gen@0x14,flag@0x16,size@0x18,desc@0x20}、GraphicsDevice::create 7 段、RenderPassAttachmentDesc 字段序、retire 哈希表 fibonacci、init_texture_data 4 目标、DynamicConstantsChunk 共享绑定、17 U→O + 30 消费端→O | inner+0x28 位 1 半、RPAttachDesc +0x08/+0x0d..0x0f、get_vk_blend_mode 表→Vk 映射 |
| raw-init | 478 行 | 87 | ✓（1 fmt 无行） | **env 链 18 变量写语义**（FUN_140d6c690=set_var 包装）、排序比较器字段判据、refresh_memory_budget max(1,old>>3)、disposal 锁拓扑（Mutex<HashMap<Seq,Mutex<Bucket>>>）、implicit_layers 0x208B 切片、5 处偏差 O 确认 | 排序权重序（242KB 待重解）、bucket 8B 载荷 |
| managed | 327 行 | 42 | 全有行（深度补足） | 退役分派跳表 0x142d060f0（8 tag）、槽=控制块{seq,refcount I32,checksum,resource}、RawManagedHandle 24B、GpuOwned<T,Tracker> 字段序、refcount=AtomicI32、swapchain 锁字非 stock | +0xa50 双语义、GpuOwnedState 判别值、+0xf 写入方 |
| crates | 308 行 | 46 | ✓（6 fmt 无行） | get_access_info 三表 64 字符、read_from_buffer wire 0x140B、Aftermath Drop=jmp DisableGpuCrashDumps、gpu-profiler period=timestampPeriod、Type::nbyte 15 项、with_suffix_dims=wrapping_add(1) | defines 哈希（404 .bin 无复现）、RasterPipelineDesc 字段语义 |

## 2. 原版 ↔ rebuild 全量差距（466 级，扩展 CALL-CHAIN 的 30 入口）

### 2.1 对得上（rebuild 实现与原版行为/布局一致，抽查字节级）

- **设备筛选/物理设备**：driver_support 24 输入 0 mismatch；old_intel 11 串表；内存门槛 1.0GB/400MB/1.4GB；格式 3 入口 816 案例 0 mismatch（FormatInfo 10B）。
- **命令缓冲/提交**：CommandBuffer 0x58B 生命周期、SeqTracker CAS、SubmissionEngine 时间线弹环、submit_pending_cbs/ensure_active_cb。
- **compiled 录制**：CompiledRenderGraph::new/record_batch 并行、BatchCompiler 分块、ResourceInfo/Registry 布局。
- **swapchain**：format/extent/present/租约/OUT_OF_DATE 旋转。
- **descriptor 池**：allocate_descriptor_set = 空闲池复用 + OUT_OF_POOL/FRAGMENTED_POOL 循环重试（审查改判 ✓）。
- **vk-sync 枚举**：get_access_info 写变体 16 个与公开枚举对上；country-core ACCESS_* 6 常量对上。
- **Type::nbyte**：跳表 15 项与 rebuild 枚举序 0..13 一致。

### 2.2 结构级偏差（rebuild 有实现但布局/语义与原不符，**接线前必修**）

| # | 项 | 原版（O） | rebuild | 影响 |
|---|---|---|---|---|
| D1 | **Buffer** | 40B 整块 {*ctx@0, slot@8, off@0x10, gen@0x14, flag@0x16, size@0x18, desc@0x20}（Result Ok 载荷 40B） | 16B desc+句柄自有结构 | SsboType/MappedStorageBuffer 的 40B 契约对不上，需提供转换或改 game 侧 |
| D2 | **GpuOwned** | 双泛型 `GpuOwned<T, Tracker>`，字段 {resource@0, tracker@0x40, state@0x58} | 单参数 `GpuOwned<T>` | 泛型签名接线差异 |
| D3 | **refcount** | AtomicI32（`lock inc/dec dword [inner+8]`） | AtomicU32 | 并发语义一致但类型宽度错 |
| D4 | **disposal** | `hashbrown::HashMap<CommandBufferSeq, Mutex<DisposalBucket>>`（insert 断言、remove 缺桶 panic） | BTreeMap 无锁、允许重复 | 结构与方法语义均偏 |
| D5 | **PassPgoNameId** | = &str 指针（ICF 恒等），**无 hash** | FNV-1a hash | hash 是自造，PGO 键不匹配 |
| D6 | **apply_vulkan_blocklist** | 18 变量 set_var 写链（DISABLE→"1"、LAYERS_DISABLE→"~implicit~"、ALLOW→join） | 空表 | 显卡屏蔽策略缺失 |
| D7 | **init_logging** | 64KB 环 @0x143634e40 + print/dump | 缺失 | 崩溃转储链缺环 |
| D8 | **refresh_memory_budget** | 逐堆滞回 `max(1,old>>3)` 钳位 + 0x110B 快照 + ArcSwap@+0x608 | 只重查询无滞回 | 预算抖动 |
| D9 | **RenderPassAttachmentDesc** | 0x10B：clear_input@+0x04/sample_count@+0x0c | `{format,samples,clear_input,depth_stencil}` | 字段序错 |
| D10 | **temporal 复用路径** | import_unowned（所有权保留） | import_owned | 所有权语义差异未登记 |

### 2.3 原版有而 rebuild 无（接线需补）

- **rg 层**：`RenderGraphState::compile` 三闭包实例 + `ResourceExportHandle<Texture>`（=0x218B）、`TemporalRenderGraphRecorder` 类型 + `GetOrCreateTemporal(Pair)` trait（键应为 **Ustr**，rebuild 用 &str）、`ImportExportToRenderGraph`/`IntoSampledRenderGraphTexture` trait 与 `from_owned_rg_resource`/`export`、`RenderGraphRecorder`/`RenderGraphRecord<PassResources>` 命名、OOM 降分辨率语义、回调 `FnOnce(u64)->bool`（rebuild `FnMut(PassContext)->bool`）。
- **raw 层**：`GraphicsDevice::{get_vk_buffer(0x140bef470), get_vk_image_view, create_render_pass(+0x188 缓存), create_framebuffer, create_command_buffer, bind_descriptor_set×4, bind_pipeline, bind_persistent_descriptor_set, update_descriptor_set, update_persistent_descriptor_set}`（多数为"方法身份"缺，free fn 已存在）；`raw::shader` setup 命令缓冲体系、`full_screen_exclusive`（原版 0 编译体证据，Unsupported 合理）。
- **crates**：aftermath-rs、vk-sync-rs、dxgi-enumerate-gpus（rebuild 无）；gpu-profiler 原本地 crate（rebuild 未引依赖，仅候选 `GpuProfiler`）。
- **ray_tracing.rs**：全 inlinee_declaration 无地址——原版实现程度本身 U。
- **lib.rs 顶层**：init_logging/print_log_dump/dump_log。

### 2.4 rebuild 候选自造（非原作行为，登记不接）

- `raw/graveyard.rs` epoch 双轨销毁（原作按命令缓冲序号时间线）——行为差异已登记，保守可用。
- `rg/{parallel,attachments,access_ledger,resource_rules,vulkan}.rs` 新增候选文件（原版无）。
- `shader_compiler/` 新增候选（原版 shader 走 DXC 外部）；shader 打包 `defines` 哈希 U。

## 3. 审查+闭合推翻/纠错清单（集成轮必读，防改错）

1. ~~`image_access_mask_to_usage_flags` rebuild 数值全错位~~ → 逐值核对与 ash 常量一致，**作废**。
2. ~~`allocate_descriptor_set` rebuild BTreeMap 自造~~ → 已空闲池复用，**改 ✓/半**。
3. ~~`bind_descriptor_set` rebuild 缺~~ → free fn 存在，仅缺方法身份，**改半**。
4. `PassPgoNameId::from_name` → ICF 恒等，**无 hash**（rebuild FNV-1a 偏差）。
5. `FUN_1402bf880` = `<Vec as SpecFromIter>::from_iter`，**非** compute_batches（审查假设不成立）。
6. `last_known_frame_duration_ns` 实返 **bool**（contains_key，PDB 名错位）。
7. `into_sampled_rg_texture` trait 名 = **`IntoSampledRenderGraphTexture`**（非 ImportExportToRenderGraph）。
8. `apply_vulkan_blocklist` 语义：**写** env（set_var 包装），旧"值含 1 触发"读语义作废；`EOS_OVERLAY_DISABLE`+`VULKAN_WIN64` 实为单一 32B 名。
9. `with_suffix_dims` = `wrapping_add(1)`（O，非降 I）。
10. RawManagedHandle +0x16 = 资源类型 tag，**非** tracked 标志。
11. `refresh_memory_budget` 除数 = `max(1, old>>3)`，非 `old/8`。
12. crates 侧 `with_suffix_dims`/`Aftermath Drop` 引用卫生修正（悬空 extra-0x140c107b0 → rne 证据）。

## 4. 剩余 U 卡点（跨域，已登记各域 §5.7/6.6）

- **rg**：validate_read_access（无编译符号）、TemporalRenderGraphRecorder +0xc8..、PassPgoStats::reset/extend inline、ResourceSyncGroup 组级 can_be_concurrent_with inline。
- **raw-core**：inner+0x28 位 1 精确语义半、RenderPassAttachmentDesc +0x08/+0x0d..0x0f 身份、get_vk_blend_mode 表值→Vk 枚举映射半、退役哈希表条目字段表。
- **raw-init**：设备排序比较器**精确权重序**（242KB helper 待重解）、disposal bucket 8B 动作载荷、implicit 层来源、init_logging 环细节。
- **managed**：+0xa50 owning_device 双语义矛盾、GpuOwnedState 判别值、元素 +0x0f 写入方、栈桶 0xE0 vs DisposalBucket 0xF0。
- **crates**：defines 哈希算法（404 .bin 无复现）、RasterPipelineDesc 字段语义、dxgi 40B 的 device/subSys/rev 空位。

## 5. 数据卫生（回填建议，不阻塞接线）

- crates ALG 头注"补取 7 个"实 9 个；rg ALG 引用本域不存在 decomp（0x140bee030/090 在 n1-device）；§1.1 compiled 计数与骨架不符（texture 26 vs 47）；台账并发写竞态（按行追加勿整覆盖）。
- `get_maybe_tracked` 地址应挂单态行非 trait 声明行。

## 6. country-core `rhapsody_contract` 接线建议

契约占位（lib.rs:82–330 + rhapsody_types.rs + render/{ssbo,shader,rg_helpers}.rs）需替换为真实 rhapsody 类型，按优先序：

1. **替换 GraphicsDevice(pub u64) → `Arc<rhapsody::raw::device::GraphicsDevice>`**（原资源是 Arc<真实结构>，rebuild `from_device` 已按 Arc 建）。dispatch 方法删除（原经 RenderPassApi::dispatch_groups_3d）。
2. **Handle<T>/ReadOnlyHandle<T> 占位 → `rg::resource::Handle` + `ManagedHandle<T:Send+Sync>`**（原无 ReadOnlyHandle；`get()` 两个 todo! 接 rebuild Handle::get / ManagedRef）。
3. **RenderPassDesc{selector} → `RasterPipelineKey{ManagedHandle,u32}` 双字段缓存键**（+0x188 HashMap）；RenderPassAttachmentDesc 字段序按 D9 改。
4. **PassResourceSlot/RenderPassBinding 对齐 rebuild 布局**（0x44B 含 TextureViewDesc 0x3cB + 资源号 + 写标志；4 变体带 offset/size/layout）。
5. **temporal 键改 Ustr**（country-core `ustr_contract::intern` 现返回 0；rebuild 补 `TemporalRenderGraphRecorder` + trait）。
6. **补 `ImportExportToRenderGraph`/`IntoSampledRenderGraphTexture` trait**（Texture/Buffer 两 impl：import_unowned/export/from_owned_rg_resource）；ImportedBuffer 20B（+0xe=5 未导入判别）。
7. **Buffer 40B 契约**（D1）：游戏面 TypedStorageBuffer 需 40B 整块；rebuild 提供转换层。
8. **补 GraphicsDevice 方法身份**（get_vk_buffer/get_vk_image_view/create_render_pass/create_framebuffer/bind_descriptor_set 等，free fn 已存在）。
9. **帧级 trait 生产接线**：`FramePasses`/`RenderFrameSurface`/`PassHost`/`RgResources` 接 compile 闭包（54 pass 入口）——这是 RENDER_GAP_ANALYSIS #2/#6/#8 的核心，需要 tiny_glade render_loop 层（非本 crate 单独能接）。

## 7. 文件索引

- 5 份 ALGORITHMS.md：`domains/{rg,raw-core,raw-init,managed,crates}/ALGORITHMS.md`（各追加"审查轮闭合"节）
- 审查清单：`domains/{域}/REVIEW-UNEXPLORED.md` × 5 + `REVIEW-UNEXPLORED/MERGED.md`
- 反编译：`domains/{域}/decomp/`（238 批量 + 补取）；rne 字节证据：`rhapsody/rne/{rg-notfound,managed,raw-init/*,crates-notfound}-closure*.md`
- 工具：`scripts/run-rhapsody-decomp.py`、`scripts/build-rhapsody-domains.py`、`scripts/capdis.py`
- 台账：`round3/MODULE-DEEP-DIG.md`（rhapsody 登记 78–88 行）
- 调用链基础：`rhapsody/CALL-CHAIN.md`、`rhapsody/PLAYBOOK.md`
