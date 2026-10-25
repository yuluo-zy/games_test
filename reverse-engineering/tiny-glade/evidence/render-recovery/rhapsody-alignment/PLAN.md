# rhapsody 对齐原版的修复清单

日期：2026-10-15。范围：`tiny-glade/crates/rhapsody`（含 `crates/shader-pipeline`）与原版 B 构建的 rhapsody 栈对齐。
地址：B patched EXE（sha256 `cf048e27…`）与原版 RNE（`f507a6ce…`，机器码 oracle）共用同一份 PDB 与 VA；
`producer-closure/` 下的旧取证绑定 RNE，引用前对 B 抽查函数头字节（已抽查 `0x140c07950` 一致）。
级别沿用 O（机器码直接核对）/ I（推断）/ U（未知）。

## 0. 基线事实（本轮核对）

* **原版源文件清单**（`evidence/multi-baseline/pdb-exports/b_files.txt`）：`crates/rhapsody/src` 51 个文件；子包
  `aftermath-rs`、`dxgi-enumerate-gpus`、`gpu-profiler`、`shader-pipeline`、`vk-sync-rs`；仓库内另有
  `crates/gpu-allocator`（12 个文件，带 `visualizer`）与 `crates/shader-variants`。**`shader-compiler` 不在 EXE 里**（构建期工具）。
* **原版函数**：`rhapsody::`/`shader_pipeline::` 522 个、388 KB（逐项见 [original_functions.tsv](original_functions.tsv)，
  `workspace_name_match` 只是末段名粗筛，`yes` ≠ 已对齐）。末段名都对不上的约 98 KB。
* **依赖**（原版 `skeleton/B/Cargo.lock`）：`gpu-allocator 0.26.0`（依赖 egui/egui_extras/presser/puffin，
  与 crates.io 0.26.0 的 visualizer 形态一致；EXE 内 97 个函数 / 52 KB）、`vk-sync-fork 0.4.0`（8 个函数）、
  `gpu-profiler 0.1.0`（11 个）、`aftermath-rs`（9 个）、`dxgi-enumerate-gpus`（1 个）。
  工作区现用 `gpu-allocator 0.27.0`（候选 feature），其余均未引入。本机 registry 已有 `gpu-allocator-0.26.0`。
* **游戏实际调用面**（在游戏 crate 内单态化的 rhapsody 泛型，决定公共签名）：

| 原函数 | 泛型实参（PDB） |
|---|---|
| `RenderGraphState::compile` | `<ResourceExportHandle<Texture>, PassResources, compile_main_render_graph::{closure}>`（0x140161cd0）、同上 `compile_loading_screen_render_graph`（0x1401647b0）、`<(), PassResources, execute_presentation_render_graph::{closure}>`（0x140166510）|
| `GraphicsDevice::create_buffer` | `<Option<&[u8]>>` ×4、`<&[u8]>`、`<()>` |
| `GraphicsDevice::create_texture` | `<&[u8]>` ×3、`<TextureInitialData>` ×4 |
| `GraphicsDevice::bind_descriptor_set` | `<I: Iterator<Item = …DescriptorSetBinding / (BindingIndex, PassResourceSlot)>>` ×4 |
| `GraphicsDevice::with_global_allocator` | `<F, R>`，R 含 `gpu_allocator::AllocatorReport` |
| `GraphicsDevice::with_setup_cb` | `<F>`：`create_buffer` / `update_buffer_bytes` / `init_texture_data` / `update_texture_impl` 的闭包 |
| `temporal::get_or_create_temporal(_pair)` | `<PassResources, &str / String>` |

  顺带闭合旧 SYNTHESIS 遗留项 6：compile 实例 #3 的调用者是 `execute_presentation_render_graph`（O，PDB 泛型名）。

* **已对齐与未对齐的例子**：`BufferDesc`（16 B，`memory_location` 1–4）已对齐；`TextureDesc`（原 32 B：
  `usage@0 flags@4 format@8 w/h/d@0xc.. array_elements@0x18 mip_levels:u8@0x1c samples:u8@0x1d kind:u8@0x1e`，kind 0–6）
  与 `TextureViewDesc`（原 6 个 `Option` 字段 + `base_mip@0x30 base_layer@0x34 bind_individual_mips@0x38`）**未对齐**——
  工作区用 `vk::ImageType` 等字段、无 `Option`、无 `bind_individual_mips`。

## 0.5 渲染机制对照（逐层，2026-10-15 核对当前工作区）

函数对齐不保证机制一致，这里按机制逐层对照。原版结论取自 `producer-closure/SYNTHESIS.md` §2–§7、§11.5–§11.6；
§11.6 的 15 条"候选与原作的系统性偏差"写于 2026-10-12，下表是对当前代码的复核。

| # | 机制 | 原版（O，除注明外） | 当前工作区 | 状态 | 清单项 |
|---|---|---|---|---|---|
| 1 | 帧在途节流 | `GraphExecutor` 首字段 `VecDeque<PendingSubmission>`（元素 0x90）；只有 `begin_frame` 会 pop 并 `wait_submit_done`，`wait_gpu_queue_trim` 只 peek、有界等待；`end_frame` push | `rg/vulkan.rs` 的候选 session 有 `pending` 队列，`begin_frame` 在 `len > 1` 时等 10 s | 部分：有节流，但结构、等待条件、超时值都不是原版的 | M5.3 部分 ✅ |
| 2 | 提交的 wait/signal | 来自设备级模板 `dev+0xa00..0xa4f`（`get_setup_cb_submit_info`），不在图数据里 | 内部时间线由提交方管理；`GraphBatchSync` 只带外部信号量（`MECHANISM_DIFF.md` 第三次修订，依据 `p4-sync-setup-cb`） | 部分：设备级模板本身未恢复 | M4.5、M5.4 |
| 3 | 图编译 | `compile<Export, UD, F>` 单态化，直调 `ResourceInfo::calculate` / `populate_resources` | `compile_frame` 注入 `ResourceBackend` / `AccessDependency` trait | **不一致** | M5.1 |
| 4 | 图资源的键 | u64 TypeId → swiss table → u32 resource_index | `vulkan.rs` / `pass_bindings` 用 `BTreeMap<String, …>` 按名字查 | **不一致**（core 那边已按 TypeId 写） | M5.1、M9 |
| 5 | 描述符集 | 空闲池复用；池耗尽时重试 | 有耗尽重试，内部用 `BTreeMap` 计数与归属 | 部分 | M3.4 |
| 6 | 光追 | 无硬件光追（四次独立确认） | 无 `ray_tracing.rs`，`rt()` 返回 `None` | 一致 | — |
| 7 | 跟踪集合 | 单个 atomic u64 高水位，没有 domain | `last_sequence: AtomicU64`；但 `tracking.rs:174` 仍有 `domain` 字段 | 部分 | M4.2 |
| 8 | managed store | 共享 store + 共享待回收 Vec | 文件头写"七类 store 共用退役来源及销毁桶" | 大体一致，布局为候选 | M3.9 |
| 9 | 销毁时机 | 延迟入桶（9 个句柄 Vec + 1 个 0x48 记录 Vec）；**每次 `queue_submit` 尾部**排出，条件 `seq ≤ queue+0x80`，排出时不等待 | 有销毁桶；排出由 `wait_idle_and_recycle_resources` 等路径调用 | **时机不一致** | M4.1、M4.2 |
| 10 | 校验字 | `resource_checksum@+0x10`，在 `dispose<T>` 里递增 | 有 `resource_checksum` 字段 | 待核对递增点 | M3.9 |
| 11 | 屏障判定 | `(code-0x28) < 0x10` 挑出要同步组的访问码 + 上一索引 + `can_be_concurrent_with` | `barrier.rs:111` 同一区间判定 | 一致 | — |
| 12 | 交换链生命周期 | 完整实现 | `managed/swapchain.rs` 106 行 | 待核对 | M3.6 |
| 13 | 交换链同步 | 逐槽复用信号量、队列锁、`validate` 后重建 | `FrameSlots` 按游标逐槽复用；有 `validate_swapchain_and_acquire_image` | 大体一致，队列锁待核对 | M3.6 |
| 14 | Surface | 通过 trait 对象绑定 winit | `from_raw` / `from_win32` | 部分 | M6.6 |
| 15 | `BufferDesc` | 16 B，`u64 + u32 + u16 + u8` | 同 | 一致 | — |
| 16 | 动态常量 | bump 分配器，每页 256 KiB，usage `0x20033`，覆盖 uniform 与 storage | 文件头写"原页大小/复用策略仍待整链对照" | **不一致** | 新增 M3.11 |
| 17 | 图资源池 | 两张 swiss table，每个键一个 LIFO 空闲栈 | `ResourcePool{textures, buffers: HashMap<Desc, Vec<_>>}` | 一致（LIFO 顺序待测） | — |
| 18 | temporal 状态机 | SYNTHESIS 写作 `new(0) → export → 2 → 下一帧 reimport → 1`；export 访问码 `0x37` | `TemporalRecordState::original_tag`：RawTexture=0 / RawBuffer=1 / Exported=2（文件头称"原跨帧合同"） | 两份记录的说法不同，用 `parallel-batch-06/temporal-lifecycle/native-oracle.json` 判定 | M5.7 |
| 19 | 实例 | apiVersion 1.2；`VK_LAYER_KHRONOS_validation`、`VK_EXT_debug_utils` 按条件开 | apiVersion 1.2 | 部分（扩展与层数组见 C8） | M6.5 |
| 20 | 一帧的顺序 | `compile → begin_frame → execute_compiled → retire → take_resource → acquire → execute(展示图) → end_frame → present → 统计 → wait_gpu_queue_trim`；**交换链图像在主图执行之后才获取** | core 的 `render_frame.rs` 按原版顺序写成 `RenderHost` 调用序；rhapsody 侧没有对应的执行器 | core 一致，rhapsody 缺 | M5、M9 |
| 21 | 录制粒度 | 每个 batch 一个命令缓冲（`ensure_active_cb`），批间 `submit_pending_cbs` | 候选 session / `ExecutionChain` | 待核对 | M5.4 |

**country-core 这一层**（`c1-country-core/frame-composition.json`，按 VA 或函数名在 core 源码中查找）：
* `render_world` 的 59 步：59 步都能在 core 里找到对应函数。
* 构图闭包的 36 步：找到 27 步。缺 `GpuLutTracker::render_luts`（0x14090e280）、`screenshot::handle_screenshot_opportunity`（0x14031d140，4 处条件调用）；
  另有 `DynamicConstants::allocate` / `into_uniform_and_storage_buffer_binding`、`ResourceInfo::calculate`、`populate_resources` 四步属于 rhapsody。
* "找得到"只说明函数存在。这些函数现在都通过 seam trait 调用，后端没接，所以执行顺序本身还没跑过。

### 机制等价怎么验收

静态手段只能证明"同样的函数、同样的调用顺序、同样的常量"，证明不了运行时行为。分三级：
1. **静态序列测试**（本机可做）：给 rhapsody 与 core 的适配层加记录型后端，跑一帧，把调用序列
   （编译 → 批次 → 屏障 → 提交 → 销毁桶排出 → acquire → present）和上表 + `frame-composition.json` 的原版顺序逐项比对。
2. **lavapipe 出帧**（本机可做）：证明能跑通，不证明与原版一致。
3. **原版运行时抓取**（本机做不了，需要你的 Windows 机器）：用 `VK_LAYER_LUNARG_api_dump` 或 RenderDoc 抓原版游戏一帧的
   Vulkan 调用序列（建资源、屏障、提交、信号量），再和我们在 lavapipe 上的同一帧比对。这是唯一能把"机制一致"从静态推断升到动态证据的办法，
   旧取证 D3 记录的"零动态证据"也只能靠它解决。

## 0.6 算法逐项核对（2026-10-15）

工作区 rhapsody 来自 `reconstruction/rhapsody-rebuild`。核对迁移记录 `migration-sources.json`：旧工程迁移后再没改过；
工作区迁移后又改了 24 个文件（含 `barrier.rs`、`resource_info.rs`、`resource_rules.rs`、`compiled.rs`、`resource_registry.rs`、
`managed/lifecycle.rs`）。所以旧验证报告里的原指令差分**不能直接算到工作区头上**，要重放。

**原版 oracle 数据（原版机器码的输入/输出对）与重放状态：**

| 数据 | 条数 | 覆盖 | 对工作区重放 |
|---|---:|---|---|
| `resource-info-oracle.jsonl.gz` | 78,867 | `can_be_concurrent_with` 77,752 / `link_up_next_access_chains` 649 / `calculate_lifetime_events` 466 | **已重放，78,867 全部一致**（临时程序 `/tmp/rh-oracle`，取自旧 `runner.rs`，未入库） |
| `parallel-batch-02/gpu-lifecycle/leaf-oracle.jsonl.gz` | 29,692 | managed `track` 等生命周期叶子 | 待重放 |
| `parallel-batch-04/device-counters/native-projection-oracle.jsonl.gz` | 2,840 | 设备计数器 / 序列号 | 待重放 |
| `parallel-batch-05/pipeline-descriptors/native-layout-oracle.jsonl.gz` | 4,438 | 描述符布局 | 待重放 |
| `parallel-batch-05/pipeline-descriptors/native-boundary-oracle.jsonl.gz` | 742 | 管线布局边界 | 待重放 |
| `parallel-batch-06/descriptor-binding/native-oracle.jsonl.gz` | 3,785 | 稀疏描述符 / tag / 动态偏移 | 待重放 |
| `parallel-batch-06/sequence-runtime/native-observations.jsonl.gz` | 908 | 提交序列运行时 | 待重放 |
| batch-03 屏障（1,344 例）、population（97 例）、batch-06 temporal | — | 屏障规划、资源分配、跨帧 | 先找回生成脚本再重放 |

**算法对照：**

| 算法 | 原版 | 工作区 | 结论 |
|---|---|---|---|
| 访问并发判定、访问链、生命周期事件 | 0x140bece70 / 0x140bd47d0 / 0x140bd4860 | `rg/resource_rules.rs` | 完整，oracle 全过 |
| `ResourceInfo::calculate` 整体 | 0x140530430 | `rg/resource_info.rs`（迁移后改过） | 叶子已验证，整体待重放 |
| `populate_resources` | 只处理获取事件，**跳过释放事件**（帧内不复用资源）；按精确键 LIFO 取池；usage 为 0 → Empty | 同 | 一致（工作区与原版都不做帧内别名） |
| 图资源池 | 两张表，每键一个 LIFO 栈 | 同 | 一致 |
| 屏障规划 | classify 区间 + 上一索引；每个阶段边界一次提交 | 同（`barrier.rs` 迁移后改过） | 待用 batch-03 重放 |
| 并行录制分块 | `budget = clamp(max, total/16, total/2)`，贪心累加（`parallel-recording/FINDINGS.md`，H） | `ChunkStrategy::Original` 逐条复刻；另有候选 `Balanced` 与 `ADAPTIVE`（feature 关闭） | 原版算法完整 |
| PGO 双缓冲 | `end_frame` 交换两份 0x40 统计并清空本帧 | `PassPgoFrames` | 一致 |
| managed 跟踪 | 单个 atomic u64 高水位 | atomic（`lifecycle.rs` 迁移后改过） | 待用 batch-02 重放 |
| 销毁 | 序列号桶，在 `queue_submit` 尾部按 `seq ≤ queue+0x80` 排出，不等待 | 销毁桶 + 候选 `graveyard`（epoch，有提交时做一次整设备 idle） | 不一致；graveyard 比原版差（原版从不整设备等待） |
| 显存分配 | 分叉 gpu-allocator 0.26 子分配 | 默认每个资源一次 `vkAllocateMemory`；0.27 子分配在 feature 后面 | 默认比原版差 |
| 动态常量 | bump，每页 256 KiB | 候选，无页策略 | 未实现 |
| 设备排序、黑名单 | 原表 | 候选 | 未实现 |
| 着色器哈希、反射读取 | `calculate_hash` / `read_from` | 无 | 未实现 |

**"比原版更好"的部分：**
* 候选实现里大多数"优化"（`PERFORMANCE_ANALYSIS.md`：去掉每帧 2 次整设备 idle、去掉资源 Drop 时逐个 `wait_idle`、
  宿主读写免等）修的是候选实现自己引入的问题，原版本来没有，**不算比原版好**。
* 可能真比原版好、但没证明的：
  * `Balanced` 分块：原版第一帧统计为空时 `budget = 0`，每个 pass 各占一条命令缓冲；新出现的 pass 耗时记 0。`Balanced` 限制块数、用均值估计未知耗时。
  * `ADAPTIVE`：轻帧退回串行。旧实测在每个 pass 0 µs 时，原版式并行比串行慢 1.5 倍。
  * 管线创建按键加槽位、同键去重：4 线程从 24.1 降到 11.1 µs/管线。原版怎么做未知。
  * 这些数字都来自人造回调和 lavapipe，没有和原版比过。
* 原版也没有、可以做的改进（都没做）：帧内瞬时资源别名、屏障合并/分割屏障、异步计算队列。

**测试迁移缺口**：旧工程 156 项测试只迁了 63 项。没迁的测试文件：`parallel_equivalence`（含"打乱屏障→回调顺序会失败"的变异检查）、
`parallel_gpu`、`graveyard`、`suballoc`、`temporal`、`setup_upload`、`chain`、`device_policy`、`texture_regions`、`perf_guards`、
`real_pass_blur`，以及 shader 相关几项。`benches/perf.rs` 与 21 个 GPU 示例也没迁。

## 1. 工作方式

每一项：读旧取证 →（不清楚处）读 B 的机器码 → 写 Rust（模块头列"函数 | 地址 | 级别"表，沿用 `country-core` 惯例）→
单元测试锁住机器码可观察的常量/布局/分支 → `cargo test/clippy/fmt` →（涉及 Vulkan 的）lavapipe 跑通。
不重实现 ash / gpu-allocator / vk-sync-fork / Bevy；`candidate` 实现在对应原版函数落地后删除或降为适配层。

旧取证入口（`evidence/render-recovery/producer-closure/`）：`n1-device`（53 个 `.c`）、`n2-texture-buffer`（32 个）、
`n3-init-instance`、`n4-rg-modules`、`a1-executor`、`a2-graph-closure`、`a3-barrier-recorder`、`a4-managed`、
`a5-descriptor-shader`、`a6-allocator-swapchain`、`a7-queues-raytracing`（含 `queue_submit` 38 KB 伪代码）、
`r1`–`r4` 复核、`g1-pe-leaves`（159 个叶子 PE 直读）、`g2-diagnostics`、`g8-semantics`；缺口登记 `GAPS.md`、结论 `SYNTHESIS.md` §11。
着色器链：`evidence/shader-toolchain-2026-10-13/pipeline/`。

## 2. 修复清单（按原版模块）

状态：☐ 未开始。"需分析"列为落码前必须先解决的不清楚点。

### M0 验证底座（最先做，后面每项都靠它判定）

| # | 项 |
|---|---|
| M0.1 ✅ | （已完成 2026-10-15：`tests/original_oracle*.rs` + `tests/common/mod.rs`，6 个文件共 8 组 121,272+1,344 例全部通过；解压走系统 `gzip -dc`，无新依赖）把 oracle 重放器入库（`tests/original_oracle.rs`），7 组 jsonl 数据各写一个解析分支，读 `evidence/` 下的 gz 文件；先完成已验证的 resource-info 一组 |
| M0.2 部分 | （barrier 1,344 例已重放，**发现并修复 1 处偏差**：迁移后 `barrier::Access` 丢了 `write`，`concurrent` 改成按码区间判断，80 例与原版不符；已恢复 `write` 字段，由 `AccessDependency::is_write` 提供。仍缺 population 97 例、temporal）找回 batch-03 屏障（1,344 例）、population（97 例）、batch-06 temporal 的生成脚本，转成可重放数据 |
| M0.3 | 把旧工程没迁的测试迁过来（并行等价性及其变异检查、graveyard、suballoc、temporal、setup 上传、chain 等），适配当前 API |
| M0.4 | 候选优化（`Balanced` / `ADAPTIVE` / graveyard / 0.27 子分配）保持在 feature 后面；默认路径只走原版算法。原版落地后用同一组 bench 对比，有实测收益的才保留为可选项 |

### M1 依赖对齐（先做，其余项依赖它）

| # | 项 | 依据 | 需分析 |
|---|---|---|---|
| M1.1 ✅ | （2026-10-15 已做：PDB 中 `with_global_allocator` 返回 `AllocatorReport`，该类型只在 0.27 系出现，故分叉算法按公共 0.27 复用，不必从 0.26 移植；开关改名 `suballocation` 并默认开启，测试通过。分叉相对 0.27 的 egui/puffin/arc-swap 附加项仍待查）`gpu-allocator`：原版是 0.26.0 的本地分叉（改到 ash 0.38，另加 arc-swap / puffin；公开 0.26.0 只支持 ash ≤0.37，不能直接替换）。做法：vendor 公开 0.26.0 → 改到 ash 0.38 → 对比 EXE 里 97 个 `gpu_allocator::` 函数（集合、大小、常量）找出分叉的其余改动 → 只补差异；默认启用子分配（原版就是子分配，工作区默认每个资源一次 `vkAllocateMemory`） | Cargo.lock、b_files（12 个文件含 visualizer）、`suballoc.rs` 文件头 | arc-swap / puffin 用在哪 |
| M1.2 部分 ✅ | 引入 `vk-sync-fork 0.4.0`；`raw::barrier` 的访问码→(access, stage) 改由它提供（旧 GAPS C4 指出原版经 `FUN_140c10dd0` 调用它） | Cargo.lock、C4 | 确认 crates.io 版与 EXE 8 个函数一致 |
| M1.3 | `gpu-profiler 0.1.0`、`aftermath-rs`、`dxgi-enumerate-gpus`：确认来源（公开 crate / 私有），Windows 专属的两个在 Linux 下给出明确的 cfg 边界 | Cargo.lock、b_files | 公开版本是否存在、与 EXE 是否一致 |

### M2 `raw::texture` / `raw::buffer` / `raw::format` / `raw::barrier`

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M2.1 | `TextureDesc` 改为原 32 B 布局与 kind 枚举（0–6），补 `format`/`mip_levels`/`array_elements` 等 builder、`get_image_create_info` | 0x140bd5430 / 0x140bd5460 … | n2 REPORT.json（布局 O） | builder 叶子在 Ghidra 无函数体，用 PE 直读核对 |
| M2.2 | `TextureViewDesc` 改为 6 个 `Option` + `base_mip`/`base_layer`/`bind_individual_mips`，补 7 个 builder | 0x140bd57e0–0x140bd59a0 | g1 §TextureViewDesc | `level_count`/`layer_count` 的双参语义（g1 遗留 3） |
| M2.3 | `TextureInitialData` 枚举 | — | create_texture 实例 | 变体与布局：读 0x140c07950 的分支 |
| M2.4 | `image_aspect_mask_from_format`、`impl Barrier::{image_barrier, buffer_barrier}` | 0x140bd6630 / 0x140bd66a0 / 0x140bd6810 | g1（表与分支已解） | — |
| M2.5 | `update_buffer_bytes::closure$0`、`init_texture_data::closure$1`、`update_texture_impl::closure$2`（setup CB 内的实际拷贝/布局转换） | 0x140c03a00 / 0x140c08560 / 0x140c08830 | n2 `.c` | 逐语句归纳（旧取证只有伪代码） |
| M2.6 | `ManagedTexture::view`（按 `TextureViewDesc` 缓存视图） | 0x140bdc5d0 | a4 | 缓存键与容器 |

### M3 `raw::device`（资源创建入口，core 直接调用）

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M3.1 | `create_buffer<D>`（`Option<&[u8]>` / `&[u8]` / `()`），带名字、可选初始数据经 `with_setup_cb` 上传 | 0x140c03460 等 6 个 | n1/n2 `.c`；GAPS A 遗留"已反汇编未归纳" | 逐语句归纳；`()` 实例（306 B）的差异 |
| M3.2 | `create_texture<D>`（`&[u8]` / `TextureInitialData`）；`desc.usage==0` 时补 TRANSFER_DST 等规则 | 0x140c07950 等 7 个 | n2 | 同上 |
| M3.3 | `update_buffer_bytes(buffer, data, ranges, offset)`、`update_texture_impl`、`init_texture_data` 的设备侧入口 | 0x140beefa0 / 0x140bfeac0 / 0x140bfe270 | n1、country-core 的两处调用点 ABI（`ssbo.rs` 注释） | — |
| M3.4 | `update_persistent_descriptor_set`、`create_persistent_descriptor_set`（bindless，`"bindless_textures"`）、`get_vk_descriptor_set` | 0x1416a2320 / 0x140bfef70 / 0x140bff400 | n1、a5 | 更新迭代器的元素类型 |
| M3.5 | `get_vk_image_view(texture, mip)`、`get_vk_buffer`、`set_texture_debug_name` / `set_buffer_debug_name` | 0x140bfeea0 / 0x140bef470 / 0x140bfe650 / 0x140beec30 | n1 | — |
| M3.6 | `create_command_buffer`、`create_render_pass`、`create_framebuffer`、`create_swapchain` | 0x140bef4a0 / 0x140bfc0d0 / 0x140bfbef0 / 0x140bfd720 | n1、batch-05 queue-callers | — |
| M3.7 | `bind_descriptor_set<I>`（原为 `GraphicsDevice` 方法，工作区是自由函数且签名不同） | 0x1416a5380 / 0x1416a5910 / 0x1416a6010 | a5 | 迭代器项的两种形态 |
| M3.8 | `GraphicsDevice::create`（24 KB：扩展/特性链/队列协商在函数内，不由调用者传入） | 0x140bf2560 | n1、n3 | 扩展与特性清单；GAPS C7（`+0x168/+0x3f8/+0x400/+0x408`）|
| M3.9 | `GraphicsDevice` 结构布局对齐（替换候选布局） | — | n1、g8 §2、C6 | C7、C10（`O+0x680`） |
| M3.10 | `profiler::create_query_result_buffer`、`mapped_slice` | 0x140bff5a0 / 0x140bd3c80 | n1 | — |
| M3.11 ✅ | `DynamicConstants`：bump 分配器、每页 256 KiB、usage `0x20033`；`allocate` / `into_uniform_and_storage_buffer_binding` | 0x140c02740 / 0x140c02370 | SYNTHESIS §11.5 | 页满后的换页与回收时机 |

### M4 提交 / 回收 / 销毁（`raw::device` 后半、`raw::submit`、`raw::disposal`）

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M4.1 | `queue_submit` 真体（11 KB）替换"数组构造模型 + 外部闭包" | 0x140c04e20 | a7（38 KB 伪代码）、batch-04 contract | GAPS C17：反编译 5 参 vs 调用点 3 参 |
| M4.2 | `immediate_empty_bucket`（11 KB，现为一行转发） | 0x140bf92f0 | n1、r3 | 9 个桶与资源类型 0–7 的对应（C2） |
| M4.3 | `Queue::update_pending_submissions`（现为空函数）、`reclaim_command_buffer_resources`（6 KB） | 0x140bf0b90 / 0x140bf0cc0 | g8（C1 已闭合：`queue+0x80` 的写者） | — |
| M4.4 | `wait_submit_done` / `wait_duration_or_until_submit_done` / `wait_idle` 按原体核对 | 0x140bfc780 / 0x140bfcf80 / 0x140bf8980 | n1 | — |
| M4.5 | `get_setup_cb_submit_info`（README 记为未迁入的异步占位） | 0x140bf84e0 | n1 | 调用者与用途 |

### M5 帧主链路（`rg::state` / `rg::compiled` / `rg::executor` / `rg::import_export`）

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M5.1 | `RenderGraphState` 原布局 + `compile<Export, UD, F: FnOnce(&mut RenderGraph<UD>) -> Export>`；单态化直调 `ResourceInfo::calculate`、`populate_resources`、`CompiledRenderGraph::new`、`on_retire`（取代 trait 注入的 `compile_frame`） | 0x140161cd0 / 0x1401647b0 / 0x140166510 | a2 §1、frame-composition.json | **三个实例都内联了游戏闭包**：要先把 rhapsody 外壳与内联的构图闭包切开（对比三实例的公共前后缀）；返回类型与 OOM 错误路径（render_frame.rs 的降分辨率重试依赖它） |
| M5.2 | `CompiledRenderGraph::add_pre_pass_barriers`、`record_batch::closure$1` | 0x1402c3d00 / 0x1402c3240 | a3、batch-04 record-submit | — |
| M5.3 | `GraphExecutor`：首字段 `VecDeque<PendingSubmission>`（元素 0x90）；`new`/`begin_frame`/`end_frame`/`wait_gpu_queue_trim`/`last_known_frame_duration_ns`（帧在途节流） | 0x140bea2d0 / 0x140beabd0 / 0x140bec400 / 0x140bea680 / 0x140beaae0 | a1 §3–4 | 统计块 `+0x48/+0x88` 的字段 |
| M5.4 | `GraphExecutionRecorder::{ensure_active_cb, execute_compiled, submit_pending_cbs, execute}` | 0x140bebc90 / 0x1400fa190 / 0x140bebdf0 / 0x1400fb5b0 | a1 §2/§4b、r1 | GAPS C18：`submit_pending_cbs` 的 20 ms 阈值来源 |
| M5.5 | `ExecutedRenderGraph::{retire, take_resource}`、`import_export::from_owned_rg_resource` | 0x140be9d00 / 0x140be9c80 / 0x140bd63f0 | a1 | — |
| M5.7 | temporal 状态机按原版 `0 新建 → 2 已导出 → 1 重新导入` 重核 `TemporalRecordState` 的取值含义 | `TemporalRenderGraphRecorder::export_temporal` 等 | SYNTHESIS §11.5 | 工作区"0=纹理 / 1=缓冲"与原版是否同一字段 |
| M5.6 | 工作区独有的 `rg/vulkan.rs`（串行 session）、`rg/parallel.rs` 等：原版链路落地后，降为测试/适配层或删除 | — | — | 逐个确认无原版对应 |

### M6 初始化与兼容性策略（`raw::init` / `raw::instance` / `raw::workarounds` / `raw::surface`）

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M6.1 | `apply_vulkan_blocklist`（10 KB）：原版层/驱动黑名单表 | 0x140c0a530 | n3 | 从 `.rdata` 读出全部名单字符串与匹配规则 |
| M6.2 | 设备排序比较器（现为候选排序）、`driver_support`、`missing_devices` | 0x140bdb090 / 0x140bda990 | n3 | 比较器所在函数与字段 |
| M6.3 | 新建 `raw/workarounds.rs`：`Workarounds` 结构 + `Serialize` | 0x140082a00 | g1 | 字段名从 serialize 的 `.rdata` 字符串取（O） |
| M6.4 | `init::serialize`、`AvailableGraphicsDeviceInitState::is_error` | 0x140340a10 / 0x140a9fa70 | g1 | 同上 |
| M6.5 | 实例扩展/层数组、`applicationName`/`engineName` | 0x140c08f10 | n3 | GAPS C8 |
| M6.6 | `raw/surface.rs` 独立出来（原版是单独文件）；`Surface::create` 的 `0xc/0xf` 枚举 | 0x1400315e0 | a6 | GAPS C9 |

### M7 `shader_pipeline` / `raw::shader`

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M7.1 ✅ | `compiled::calculate_hash`（着色器产物缓存键） | 0x1420cfbf0 | shader-toolchain pipeline/path.asm.txt | 哈希算法与输入字段顺序 |
| M7.2 ✅ | `reflection_ty::read_from`（3.5 KB，从产物读反射类型） | 0x14147a7e0 | type-decoder-region.asm.txt | 逐类型标签 |
| M7.3 | `create_descriptor_set_layout` ×2 | 0x1418c1a10 / 0x1418c2150 | a5 | 两实例的差异 |
| M7.4 ✅ | `RasterPipelineOpts` 改为原 4 字节位域 | — | README 已登记 | 位分配（从 `create_raster_pipeline` 0x1416a3720 读） |
| M7.5 | `create_raster_pipeline` / `create_compute_pipeline` 参数改为原形（编译产物 + key） | 0x1416a3720 / 0x1416a4b20 | n1 | — |

### M8 诊断（`leek.rs` / `raw::crash` / 子包）

| # | 项 | 原地址 | 依据 | 需分析 |
|---|---|---|---|---|
| M8.1 | 新建 `leek.rs`：环形缓冲 + 钩子 `begin_rg_pass`/`end_rg_pass`/`create_*_pipeline`/`update_persistent_descriptor_set`，`init_logging`/`dump_log`/`print_log_dump` | 见 tsv `rhapsody::leek` | g2 | 环形缓冲布局与容量 |
| M8.2 | `leekspin_print_dump`（11 KB） | 0x140be2b40 | GAPS A 遗留"已反汇编未归纳" | 逐语句归纳 |
| M8.3 | `log_gpu_crash_info` 与 aftermath 的接线（Windows 专属，Linux 下明确禁用） | 0x140befd60 | g2 | — |

### M9 接上 country-core（M2–M5 完成后）

| # | 项 |
|---|---|
| M9.1 | `country-core` 依赖 `rhapsody`，删除 `rhapsody_types.rs` / `rhapsody_contract.rs` / `ssbo::rhapsody_boundary` / `rg_helpers::rg_boundary` / `shader::boundary` 里与 rhapsody 重复的占位类型，改为复用（原版就是直接调用） |
| M9.2 | 游戏侧的 seam trait（`RenderHost`、`PassHost`、`TextureDevice` 等约 20 个）在 `tiny-glade` 应用 crate 里用 rhapsody 实现 |
| M9.3 | 补 core 构图闭包缺的两步：`GpuLutTracker::render_luts`（0x14090e280）、`screenshot::handle_screenshot_opportunity`（0x14031d140） |
| M9.4 | 静态序列测试：记录型后端跑一帧，调用序列与 §0.5 的原版顺序、`frame-composition.json` 逐项比对 |
| M9.5 | lavapipe 上出第一帧（加载画面图 → 主图 → 展示图） |
| M9.6 | （需要 Windows 机器）抓原版一帧的 Vulkan 调用序列，与 lavapipe 的同一帧比对 |

## 3. 顺序与依赖

```
M0 验证底座（oracle 重放 + 测试迁回）── 贯穿所有后续项
M1 依赖 ─┬─> M2 纹理/缓冲描述 ──> M3 资源创建 ──┐
         │                                      ├─> M5 帧主链路 ──> M9 接 core
         └─> M7 着色器产物 ─────────────────────┘
M4 提交/回收（与 M3 并行，M5 依赖）
M6 初始化策略、M8 诊断：不挡第一帧，可并行或后置
```

## 4. 风险

| 风险 | 可能性 | 应对 |
|---|---|---|
| `compile` 三实例内联了游戏闭包，外壳边界难切 | 高 | 先对比三实例的公共前缀/后缀与对 `ResourceInfo::calculate` 等的调用序列；闭包部分留给 country-core 的 `compile_main_render_graph` |
| 旧伪代码有已知失真（C17 参数个数、C18 阈值、Ghidra 跳表误解码） | 中 | 关键分支一律回 PE 机器码核对（`/tmp/dis.py`），不单凭伪代码 |
| 改 `TextureDesc`/`TextureViewDesc` 等公共类型会打断工作区现有 66 项测试与 `tg-text` 的 gpu feature | 中 | 每次只改一个类型，同步修测试；`candidate` 路径保留到替换完成 |
| `gpu-allocator` 分叉有未知改动 | 中 | M1.1 先做函数级对比再决定 |
| 无原版运行时可做差分，Vulkan 行为只能在 lavapipe 上验 | 高 | 布局/常量/分支用单元测试锁住；GPU 行为只声明"lavapipe 通过"，不声明与原版一致 |

## 5. 验证

```bash
cd tiny-glade && export PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/tmp/tg-target
cargo test  --config /tmp/m-ustc.toml --locked -p rhapsody -p shader-pipeline --all-features
cargo clippy --config /tmp/m-ustc.toml --locked -p rhapsody -p shader-pipeline --all-features --all-targets -- -D warnings
cargo fmt -p rhapsody -p shader-pipeline --check
# M9 之后加：-p country-core -p tiny-glade，以及 lavapipe 出帧
```


## 4.5 实施记录
- M4 销毁（2026-10-15）：`graveyard` 改为设备级“退役时间线”信号量（每次主队列提交末尾 signal epoch，非阻塞 `get_semaphore_counter_value` 判完成，宿主访问用 `wait_semaphores`），去掉为销毁而整设备 idle；非主队列提交保守回退 idle。覆盖 4 个提交点（command_buffer / setup / swapchain / rg::vulkan）。
- M3.11 DynamicConstants（2026-10-15，按 0x140c02740 / 0x140c029f0 机器码重写）：页 0x40000、usage 0x20033（恒含 BDA）、名 `dynamic constants buffer`；只有“当前页”线性 bump，放不下新建页；单次分配 > 页大小 `assert!` panic；对齐 `max(请求>16 ? 请求 : 16, 设备对齐)`；删除工作区独有的 `reset()`（整设备 idle）、`allocate_buffer`、首次适配。仍未证实：设备对齐取自属性结构 `+0x340` 的哪个 limit（现取 uniform/storage 偏移对齐较大者）。
- 阻塞点（M3.1–M3.9 / M5.1）：原版资源 API 是“设备持有的槽位 + 句柄（device id@+0x14、kind@+0x16、generation@+0x10）”模型，工作区是 `Arc<Texture/Buffer>`。继续还原 `create_*`/`get_vk_*`/`update_*` 前需先定：采用原句柄模型替换 `Arc` 模型，还是在 `Arc` 之上做句柄适配层。
- 方案 2（句柄适配层，2026-10-15 起）：
  - `raw/device_api.rs`：在 `GraphicsDevice.resources`（`ManagedResources` 槽位）之上提供原名入口 `create_buffer<D>`（`()`/`&[u8]`/`Option<&[u8]>`；有非空数据时 `usage |= TRANSFER_DST`，依 0x140c03460）、`create_texture<D>`（`()`/`&[u8]`）、`get_vk_buffer`、`get_vk_image_view`、`update_buffer_bytes(buffer,data,ranges,offset)`、`set_{buffer,texture}_debug_name`；校验句柄属于本设备；GPU-only 缓冲经暂存缓冲 + setup CB 上传。未做：`TextureInitialData` 枚举（M2.3）、`create_texture` 的 usage 补全规则（M3.2）、`update_persistent_descriptor_set`/`bind_descriptor_set`（M3.4/3.7）。
  - `RenderGraphState::compile<Export>(backend, dependency, build)`：闭包构图 → export temporal → build → population → 校验（M5.1 入口形态；三个实例内联的游戏闭包尚未切分）。
  - M9.2 起步：`crates/tiny-glade/src/render_host.rs` 的 `BufferHost` 实现 country-core 的 `rhapsody_boundary::GraphicsDevice`（`create_buffer`/`update_buffer_bytes`/`mapped_slice_mut`/`device_address`），core 的 `Buffer{id,generation}` 映射到槽位表，`release` 递增代数；烟测 `tests/render_host.rs` 通过。
  - 修正 M1.1 的遗漏：运行时默认分配模式原来仍是专用分配，现在默认子分配（`RHAPSODY_ALLOCATION=dedicated` 回退）。
- M2.1 TextureDesc（2026-10-15）：按原 32 B 布局重写（`usage@0 flags@4 format@8 width@0xc height@0x10 depth@0x14 array_elements@0x18 mip_levels@0x1c sample_count@0x1d kind@0x1e`），`TextureKind` 0..6、cube 层数 ×6、`array_elements` 仅数组种类允许 >1；构造器 usage 恒 0（0x140bd5270/52a0 清零 qword），由 `.sampled()` 等追加；`TextureDesc::new(extent, kind, format)`。**机器码证实 core 占位里"含义 U"的文件尾两字节：−3 = kind、−4 = format**（core 的 `arg3/arg4` 已改为 `kind/format`，测试调用点同步）。`rg::vulkan` 的图 TextureDesc 编码改为原布局。
- M3.2 `create_texture`（0x140c07950）：`assert!(desc.usage != empty)`；有非空数据 `usage |= TRANSFER_DST`；extent 三维 `assert!(>0)`，消息从 .rdata 读出。
- M7.1/M7.2 纠正：`calculate_hash` 0x1420cfbf0 是 std `DefaultHasher`（SipHash-1-3）对 `(String,String)` 的逐对哈希，即已有的 `ShaderDefines::artifact_hash`；`reflection_ty::read_from` 是 speedy 派生。两项原先标"缺失"有误。
- M9.2 续：`tiny-glade::render_host` 新增 `TextureHost`（core `TextureDevice` → `create_texture`，槽位序号+代数编码进 `Texture.id`）与 `ShaderHost`（core `ShaderBackend`：读发行 .bin、解码、反射布局、管线布局、计算管线）。光栅管线（固定状态/渲染通道，M7.4/M3.6）与 `raster_shader_variants`（0x1417388a0）、Sampler 名→内置采样器映射仍未接。烟测 3 项通过（lavapipe）。
- M5.3 帧节流（2026-10-15）：`rg/frame_throttle.rs`（`FrameThrottle<R>`）= 原 `GraphExecutor` 首字段 `VecDeque<PendingSubmission>` 的三处用法：`begin_frame` 弹出并等待到只剩 1 个在途；`end_frame` `push_back`；`wait_gpu_queue_trim` 超时非零且在途 >1 时对队首带超时等待（不弹出）。尚未接入 `ExecutionChain`（工作区没有"帧"边界），也未实现 `last_known_frame_duration_ns`（伪代码形态是 `(end−begin)×timestampPeriod < 1e9` 的布尔判断，PDB 名与行为不符，待核）。
- M1.2 访问码表（2026-10-15）：`FUN_140c10dd0` 是 64 项三表查找（`.rdata` 0x142b1a6c8 stage / 0x142b1a7c8 access / 0x142b1a8c8 layout，下标 = code，输出 +0/+4/+8），`is_write`（0x140c10e10）= `code−0x28 < 0x10`。**公开 `vk-sync-fork 0.4.0` 编号与原版不同**（原版前部多 6 项：`TransferWrite` 原 0x34 / 公开 0x2e，`Present` 原 0x27 / 公开 0x21），故不引入该 crate，表逐项读出落成 `raw/access_table.rs`（含 spot check 测试），`AccessTable::original()` 提供全部 64 码。原先 PLAN 假设"改由 vk-sync-fork 提供"作废。
- M9.2 完成度（`tiny-glade/src/render_host.rs`，lavapipe 烟测 6 项）：`BufferHost`/`TextureHost`/`ShaderHost`（计算管线 + 反射登记表）/`DeviceHost`（`ComputePassDevice` + `pass_resource_access::TextureDevice`）/`GraphHost`（`BufferGraphBackend`：import_unowned/export 落到 rhapsody 图）/`GraphPasses`（`rg_boundary::RenderGraph`：`add_pass` + `allocate_dynamic`，回调内用 rhapsody `RenderPassApi` 做 `fill_buffer`/`clear_color_image`/`copy_buffer`，暂存页经 `PassContext::keep_alive` 保活）。**端到端已验证**：core 的 `clear_rg_buffer` + `rg_buffer_copy_from_slice_u32` → rhapsody `VulkanGraphSession` → lavapipe 真实执行，读回 `[1,2,3,4]`。
  - 为此：core 的 `RenderFn` 增加 `+ Send`；`Buffer` 保存创建时的完整 `BufferDesc`，图里缓冲描述按原字节布局编码 alignment/memory_location；`ManagedPassResources::{buffer_for,texture_for}`；`Managed::shared()`；`Texture.id` 编码为 `TAG|代数<<32|槽位`。
  - 仍未接：光栅管线/渲染通道、`ComputeExec`（dispatch 路径）、`WorldAccess`、`PassHost`（draw）、`RenderHost` 帧级。
- 计算 Pass 执行（2026-10-15）：`GraphPasses::add_compute(RecordedComputePass, &ComputePipeline)`（需 `with_tables(HostTables)`）。声明期：访问进图、uniform 块写入动态常量页；回调内：按 `AccessRange` 把 `PassBinding`（RG 缓冲/图像视图，类型取自管线布局的该 binding）与 `ExternalBinding`（缓冲/图像）解析成 `DescriptorSetBinding`，从 set 0 的 `DescriptorSetState` 分配描述符集，`cmd_bind_pipeline` + `cmd_bind_descriptor_sets`，再由 core 的 `ComputeExecution::execute`（push constants + 5 种 dispatch 模式，从 `RecordedComputePass::execute` 拆出以便 `Send`）经 `ComputeExecHost` 发命令。`ShaderHost` 的计算管线条目新增 `ComputeEntry{local_size, set0, uniform_binding}`。**端到端已验证**：rspirv 生成 `out[i]=i+100` 的 compute SPIR-V → 发行容器 → `RecordedComputePass` → 会话 → lavapipe，读回 `[100,101,102,103]`。
  - 教训（两次同类崩溃）：回调里录制的命令引用的对象（暂存页、管线及布局、描述符集）必须经 `PassContext::keep_alive` 交给命令拥有者，不能靠回调内 `Arc` 的生命周期——graveyard 只保护"已提交"的工作，未提交时设备静止会就地销毁。
  - 未证实/简化：`full_barrier_enabled`（`[device+0xaba]` 调试开关）恒 false；`ExternalSlot::Other` 未接；uniform 的 binding 号取反射 `uniform_block.binding.binding`（core 只存 set）；每次 dispatch 分配瞬时描述符集（原版的持久集/`bind_descriptor_set<I>` 迭代器形态未对照）。

- M7.4/M7.5 光栅管线（2026-10-15，按 0x1416a3720 / 0x140be67f0 / 0x1417388a0 机器码）：
  - `RasterPipelineKey.selector` 就是 `RasterPipelineOpts` 的 4 字节位域（key+0x18），`handle` 是渲染通道句柄（kind=4；通道对象 +0x10 `VkRenderPass`、+0x18 颜色附件数、+0x20 采样数）。
  - 位分配：b0 拓扑(0–1: 三角列表/线列表/线条带，表 0x142c88ff0) | 剔除(2–3) | 深度比较(4–5: LESS/GREATER/EQUAL，表 0x142c89000) | 深度写(6) | 深度测试(7)；b1 模板测试(0)、fail(1–3)、pass(4–6)、depth-fail 低位(7)+b2(0–1)；b2 compare op(2–4)、compare mask 低 3 位(5–7)；b3 compare mask 高 5 位(0–4)、混合模式(5–6)、线框(7)。core 里既有的 `byte0 &= 0x7f`（关深度测试）、`byte3 |= 0x80`（线框）与此一致。
  - 模板 front/back 同值，write mask 0xffffffff，reference 0（动态）；深度边界 [0,1] 且关闭；front_face 0；线宽 1.0；动态状态 {VIEWPORT, SCISSOR, STENCIL_REFERENCE}（+LINE_WIDTH 当 `[device+0xad9]`）。混合附件对每个颜色附件复制同一份（`get_vk_blend_mode`）。
  - 顶点输入：每个 location 一个 binding（binding==location，stride=类型字节，offset 0，VERTEX），只接受 32 位标量/向量。
  - `raster_shader_variants`（0x1417388a0）：desc 克隆两份，defines 末尾追加 `("SHADER_VARIANT_DEPTH_ONLY","0"/"1")`。
  - 工作区：`rhapsody::raw::shader::RasterPipelineOpts(pub u32)` 取代 Option 构建器；`RasterPipelineDesc` 增加 stencil/line_width_dynamic，`create_raster_pipeline` 带模板、动态状态；`vertex_input_from_reflection`；`tiny-glade::render_host` 增加 `RenderPassHost`、`RasterEntry`、`ShaderHost::create_raster_pipeline`/`raster_shader_variants`。lavapipe 上用 rspirv 生成的 vs/ps 容器建出图形管线。
  - 未证实：`[device+0xad9]` 对应的设备特性（现恒不加 LINE_WIDTH）；子通道号恒 0；`create_raster_pipeline` 前半段（0x1416a3764 起的布局创建）与 rhapsody 现有布局规划的逐项对应；`load_compiled_raster` 的 vertex_entry 恒空串；渲染通道本身的原创建路径（M3.6）。

## M10（依据 RHAPSODY-DEEP-SUMMARY §2.2 结构偏差 D1–D10，2026-10-15）

证据：`country-core-deep/rhapsody/RHAPSODY-DEEP-SUMMARY.md` 及各域 ALGORITHMS / rne 闭合文档。

| 偏差 | 处理 | 位置 | 状态 |
|---|---|---|---|
| D1 Buffer 40B | 新增 `BufferRecord`（repr(C) 40B，偏移 0/8/0x10/0x14/0x16/0x18/0x20）+ `Buffer::record` 转换层；内部 `Buffer` 仍自带 Arc | `raw/buffer.rs` | ✅ 布局由机器码偏移表断言；转换所需的 ctx/slot 来自调用方槽表 |
| D2 GpuOwned 双泛型 | 未动 | — | ⏳ 依赖句柄模型改造，见下 |
| D3 refcount | `ControlBlock.references` 改 `AtomicI32`，`clone_reference` 返回 `as u32` 保持 oracle 接口 | `raw/managed/tracking.rs` | ✅ 原 oracle 回放测试仍过 |
| D4 disposal | **有意保留** `BTreeMap` + `Result`：外层 Mutex 已由 `device.rs`/`resources.rs` 包住；原版重复/缺桶是 panic，这里返回 `Failure`，行为超集 | `raw/disposal.rs` | ⚠️ 登记差异，不改 |
| D5 PassPgoNameId | **有意保留** FNV-1a：原版是 `&'static str` 指针恒等，而 pass 名在这里是自有 `String`，指针身份无法跨帧；注释已更正 | `rg/executor.rs` | ⚠️ 登记差异 |
| D6 apply_vulkan_blocklist | 重写为 18 次 `set_var` 链：`VK_LOADER_LAYERS_DISABLE=~implicit~` → `VK_LOADER_LAYERS_ALLOW=<VK_LAYER_* join ",">` → 16 个 `DISABLE_*=1`；失败 panic `failed to set environment variable`。删除原先的空表 modern/legacy/doubletap 候选 | `raw/instance.rs` | ✅ 顺序/名长（32B 的 EOS 名）有测试；**未证实**：入参过滤里 “bit0=1” 条件的含义，当前只按 `VK_LAYER_` 前缀筛 |
| D7 init_logging | 新增 64 KiB 日志环：重复 `init_logging` panic `assertion failed: prev.is_none()`；`dump_log` 未初始化 panic；`print_log_dump` 写 stderr | `log_ring.rs` | ✅ 行为；**环内部布局（FUN_1418c1310/1430）未恢复**，`record` 写入口是候选，无人调用 |
| D8 refresh_memory_budget | 逐堆滞回 `clamp(new, old-max(1,old>>3), old+max(1,old>>3))`；首次无旧快照原样采用 | `raw/physical_device.rs` | ✅ 公式有测试；0x110B 快照/ArcSwap 布局未照搬（沿用 `ArcSwapOption<PerformanceInfo>`） |
| D9 RenderPassAttachmentDesc | `raw::render_pass` 版改为 0x10B `{format@0, clear_input@4, u32@8, sample_count@0xc, 3B}`；无 depth_stencil 字段，由格式（124..=130）推出 | `raw/render_pass.rs` | ✅ 偏移测试；`rg::attachments` 里另一个同名类型未动 |
| D10 temporal 复用 | 复用与新建都改 `import_unowned`，所有权留在时态表 | `rg/temporal.rs` | ✅ 全工作区测试通过 |

验证：`cargo test -p rhapsody -p tiny-glade -p country-core --all-features`（lavapipe）全部通过。

### 仍未做（按深挖 §2.3 / §6）
- D2 `GpuOwned<T,Tracker>` 双泛型与 `get_tracked` “获取并跟踪”语义（现返回序号）。
- `GraphicsDevice` 方法身份：`get_vk_buffer/get_vk_image_view/create_render_pass(+0x188 缓存)/create_framebuffer/bind_descriptor_set×4/...`（free fn 已有）。
- `TemporalRenderGraphRecorder` 类型、`ImportExportToRenderGraph`/`IntoSampledRenderGraphTexture` trait、`from_owned_rg_resource`。
- country-core `rhapsody_contract` 占位类型替换（§6 第 1–9 点）。
- 设备排序比较器精确权重序（242KB helper 未重解）、blocklist 过滤条件 bit0。

### M10 续（同日第二批，无需新证据的部分）
| 项 | 处理 | 位置 | 状态 |
|---|---|---|---|
| D2 GpuOwned | 改为双泛型 `GpuOwned<R, Tracker>`，`GpuOwnedState{Owned=0,Tracked=1,Discarded=2}`；`track_unwrap` 先置状态再登记命令并交出资源；`map_tracker` 换 tracker；`mark_unused` 只置状态。trait `TrackGpuLifetime`；`ManagedHandle::into_gpu_owned`→`GpuOwned<(),ManagedHandle<T>>`、`into_raw_gpu_owned`→`GpuOwned<(),RawManagedHandle<T>>` | `raw/managed/handle.rs` | ✅ 结构；⚠️ `Tracked=1` 是候选值（原 `Cell::set` 的值未读） |
| `ManagedRef::get_tracked` | 改为“先跟踪再取资源”（原为返回序号） | 同上 | ✅ |
| `create_render_pass` | `GraphicsDevice` 增加 `render_pass_cache`（对应 +0x188），相同附件序列命中同一通道；上限 8 附件 | `raw/device.rs` `raw/device_api.rs` | ✅ lavapipe 测试；⚠️ 缓存键用 `RenderPassKey`，原键含 `ManagedHandle@0 + u32@0x18` 未对上 |
| `create_framebuffer` | 校验通道属于本设备，登记到 framebuffers store | 同上 | ✅ lavapipe 测试 |
| `bind_pipeline` / `bind_descriptor_set` / `bind_persistent_descriptor_set` | `GraphicsDevice` 方法身份，委托 `raw::binding` free fn | 同上 | ✅ 仅编译，无独立测试 |
| `ImportExportToRenderGraph` | trait + `PhysicalTexture`/`PhysicalBuffer`：`import_unowned/import_owned/export/from_owned_rg_resource` | `rg/import_export.rs` | ✅ 测试；⚠️ 原 `ImportedBuffer` 20B（+0xe=5）未做 |
| `IntoSampledRenderGraphTexture` | trait 名按 0x140bd6470 更正，对 `Ref<TextureRef>` 实现 | 同上 | ✅ |
| `TemporalRenderGraphRecorder` + `GetOrCreateTemporal` | 薄包装 state+graph，键仍是 Ustr | `rg/temporal.rs` | ✅ 测试 |

仍需新证据或跨 crate 才能继续：`GraphicsDevice::create` 本体、`queue_submit`/提交模板、设备排序权重序、`RenderPassDesc` 缓存键的 ManagedHandle 字段、`ImportedBuffer`、country-core 契约替换、5 组未回放 oracle。

## M11 按模块推进：raw-core（缺口最多的域，依据 RHAPSODY-DEEP-SUMMARY + domains/raw-core/ALGORITHMS）

排序依据（各域 ALGORITHMS 对照表 ✗/半 行数）：raw-core ✗7 半16（184 compiled）> raw-init ✗8 半16 > managed ✗6 半6 > crates ✗4 半1 > rg ✗1 半4。先做 raw-core。

### GraphicsDevice::create 设备创建段（0x140bf2560，机器码 + 字符串池 0x142b16000..0x142b168c8）
`GraphicsInstance::create_device` 按原版重写，lavapipe 上默认设备仍可创建，全工作区测试通过：
- **必需扩展 8 个**（池 0x142b16726 起，按序）：swapchain / maintenance1-3 / descriptor_indexing / image_format_list / shader_demote_to_helper_invocation / buffer_device_address；缺失 → 带名字的 `Error::Missing`。
- **可选扩展（支持才启用）**：full_screen_exclusive / memory_budget / subgroup_size_control / external_memory_win32（探测函数 `FUN_140bf8fb0` 传 `len+NUL`）。`VK_NV_device_diagnostic_checkpoints` 另受设置字节 `[opt+3]==1` 门控，当前无该选项故不启用。
- **必需特性链**（校验顺序即反编译 0x140bf3470.. 的 if 链）：core 11 项 → `PhysicalDeviceVulkan12Features` 9 项 → `ShaderDemoteToHelperInvocation`；第一个缺失项报 `组名.字段名`，名字取自字符串池。
- **描述符上限**：`maxDescriptorSetUpdateAfterBindSampledImages < 0x2000` 报 “… is too low (N). Must be at least 8192”（比较在 `physical+0x680`）。
- **队列**：通用队列取首个带 GRAPHICS 位的族，1 个，**优先级 0.5**（`0x3f000000`）；独立 compute 取首个 `flags&3==COMPUTE` 的族，**创建该族全部队列，优先级 1.0**（原先只建 1 个、通用队列优先级 1.0）。
- 去掉旧默认里的 `VK_KHR_timeline_semaphore` 额外扩展名（原必需列表里没有，timeline 靠 1.2 特性）。
- `get_vk_blend_mode` 的 Vk 枚举映射：表值 `{0,1,7,0x10}/{0,1,7,0x12}` 直接是 `BlendFactor` 原始值（7=ONE_MINUS_SRC_ALPHA，0x10=ONE_MINUS_SRC1_COLOR，0x12=ONE_MINUS_SRC1_ALPHA），与必需特性 `dual_src_blend` 一致，该“半”项闭合。

### 本模块仍未对齐（未动）
- `Buffer::mapped_slice_mut(&self, device)->Option<&mut [u8]>` 与 `device_address` 的缓存读取语义（API 形状不同）。
- `create_command_buffer`（池空闲链 `dev+0x9b0`）、`validate_swapchain_and_acquire_image`、全局退役哈希表 0x1436308f0、`init_texture_data` 跳表各格式写入。
- 设备创建尾段：gpu-allocator 初始化日志（device_memblock_size 等）、`vkCreatePipelineCache`、crash tracking buffer、“Backend device fully initialized.” 日志；`GraphicsDevice` 真实字段布局。
- 必需特性里 `vertex_pipeline_stores_and_atomics` 前有一个选项位门控（反编译 `(bVar88&1)`，bVar88 来自 `[opt+2]`），当前按“必需”处理——这是**假设**，未证实。

## M12 全域并行推进（raw-core / raw-init / managed / crates / rg，两批 9 个 agent + 收口）

各域逐项状态见 `reports/`：`raw-core-buffer-texture.md`、`raw-core-commands-device.md`、`raw-core-descriptors-shader.md`、`raw-init-devices.md`、`raw-init-instance-disposal.md`、`managed.md`、`crates.md`、`rg.md`、`rg-round2.md`、`rg-pass-entries.md`、`wave2-structure.md`、`wave2-queue-submit.md`、`wave2-texture-upload.md`、`wave2-samplers-binding.md`。
验证：`cargo test -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）= 48 个测试目标，1741 passed / 0 failed；original_oracle* 回放不回归。

### 按机器码推翻/更正的旧结论（以后以这里为准）
| 旧说法 | 实际 | 证据 |
|---|---|---|
| AvailableGraphicsDevice `+0x99` 是 shared 标志；排序是 introsort；内存门槛用 budget | `+0x99` 是设备类别字节（0 离散/1 集成/2 虚拟/3 CPU/4 其他）；排序是稳定 driftsort；门槛和排序用堆大小。排序键 `base-(local>>30)` | reports/raw-init-devices.md |
| `performance_info` 是堆预算 | 是 NVAPI 厂商遥测文本（6 段，换行连接），无厂商信息返回 "Unsupported" | 同上 |
| 预算快照是 16×{u128,u128} | 就是一份 `VkPhysicalDeviceMemoryBudgetPropertiesEXT`（0x110B） | 同上 |
| `inner+0x30` 是 device_address 缓存；`update_buffer_bytes` 有直写路径 | `+0x30` 是 VkBuffer，device_address 每次调 vk；一律走暂存缓冲 | reports/raw-core-buffer-texture.md |
| Texture 40B；`"GpuOnly texture"` 是默认名 | Texture 0x38B；该串只出现在分配失败载荷 | 同上 |
| `create_render_pass` 键 = ManagedHandle@0 + u32@0x18 | 那是 `RasterPipelineKey`；真键是 0x98B `RenderPassDesc`（颜色 ArrayVec<8> + 可选深度）；无子通道依赖；pass +0x18=颜色附件数、+0x20=采样数；`RenderPassAttachmentDesc+0x08`=storeOp | reports/raw-core-descriptors-shader.md |
| `[device+0xad9]` 未知 | 创建设备时实际启用的 wideLines | 同上 |
| 0x140d4d740 是描述符布局缓存 | 是 std RandomState 线程局部密钥；原版无布局缓存；池 maxSets=100 | 同上 |
| bind_descriptor_set 有第 4 个单态 | 只有 3 个 | 同上 |
| 全局退役哈希表 0x1436308f0 属 rhapsody | 是 parking_lot_core 全局 HASHTABLE，依赖库实现，不复现 | reports/raw-core-commands-device.md |
| disposal 外层是 Mutex；缺桶 panic | 外层是 RwLock；缺桶写日志并放回退桶、立即销毁 | reports/raw-init-instance-disposal.md |
| `ManagedResources+0xa50` 是销毁桶表 / 是 owning_device（两份记录互相矛盾） | `ManagedResources+0xa50` 是 u16 owning_device；销毁桶在 `GraphicsDevice+0xa50` | reports/managed.md（后者读到 7 个 dispose<T> 的比较点） |
| swapchain 锁字是 Bevy 分叉 | 是 stock parking_lot 0.12.1（写位 8、读步长 0x10） | 同上 |
| 日志环 0x143634e40 布局未知 | `leekspin::RingBuffer`，无锁 `{ptr, cap=0x10000, cursor}`；写入者是 dispose_cb_resource / dispose_in_order | raw-init-instance-disposal.md |
| apply_vulkan_blocklist 入参过滤 “bit0=1” | 入参是 allowlist 条目表：Vendor→`VK_LAYER_{v}*`，Layer→原名 | 同上 |
| `GpuOwnedState::Tracked` 值未知 | Owned=0/Tracked=1/Unused=2（原名 Discarded 作废）；tag 3 是 ResourceType::TextureView，跳表里 `todo!()` | managed.md |
| `last_known_frame_duration_ns` / `PassPgoStats::duration_ns` 返回 bool | 都返回 `Option<u64>`（Ghidra 丢了 rdx）；`duration_ns` 调用点后 `unwrap_or(0)` | rg-round2.md（rne/rg-notfound-closure.md §12 的 contains_key 作废） |
| `queue_submit` = 0x140bfc750；`dev+0xa00..0xa4f` 是 wait/signal 模板 | `queue_submit` = 0x140c04e20（0x140bfc750 是 queue_family_index）；`dev+0x9f0..0xa4f` 是 setup 命令缓冲槽 `Mutex<Option<CB>>` | wave2-queue-submit.md |
| 内置采样器 9 个 | 13 个（9 个反射名 + 4 个 `_iso`），状态读自 0x140bf5208..0x140bf5823，vkCreateSampler 调用点 0x14213604d | wave2-samplers-binding.md |
| 视图缓存带私有位图/校验和；“纹理全局 id 计数器” | 标准 `RwLock<HashMap>`（SipHash-1-3）；“计数器”是 std RandomState 线程密钥 | wave2-texture-upload.md |
| `device+0xab0` 描述符更新计数器 | 4 处 `lock inc`，没有读取者，纯统计 | wave2-samplers-binding.md |
| crates 域四个本地 crate 是否依赖库 | vk-sync-fork / aftermath-rs / gpu-profiler / dxgi-enumerate-gpus 在 Cargo.lock 无 source，是原版本地 crate；DXGI 用 `EnumAdapters`+`GetDesc`（非 1 版） | crates.md |

### 本批新增/对齐（摘要）
- raw-core：必需扩展/特性链、描述符上限、队列配置；命令缓冲池；提交引擎语义；设备创建尾段；交换链校验与 RwLock 合并；Buffer/Texture 原版形状与多 mip/多层/BC 初值上传（读回验证）；`RenderPassDesc` 缓存键与创建本体；13 个内置采样器；`RenderPassBinding` 0x44；queue_submit 本体（`SubmissionEngine::queue_submit`）。
- raw-init：设备排序键、missing/available、`create_device_by_uuid`、预算滞回；disposal 原结构（RwLock、桶移到 GraphicsDevice）；blocklist 入参；日志环；leek；crash 日志行。
- managed：`RawManagedHandleAbi`(24B)/`DisposedRawResourceHandleAbi`(16B)/`GpuOwned` repr(C) + 偏移测试；无锁空闲链；`track` tag≠1 panic（保留 `try_track`）。
- crates：vk-sync 64 项表 0 差异；aftermath/gpu_profiler/dxgi 新模块。
- rg：FrameThrottle 接入、FrameLedger/帧顺序检查、`PassPgoNameId` 驻留池、`ImportedBuffer`、CompiledRenderGraph 布局常量、temporal/population oracle 回放（1005 + 48+48 + 95 例）、59 个 pass 调用点清单。
- 测试 demo：`tests/triangle_draw.rs`（三角形像素读回）、`tests/triangle_animation.rs`（48 帧旋转，2 帧在飞，逐像素对比 CPU 参考光栅化 + 质心轨迹 + 清除色不泄漏）。

### 仍未闭合（原版证据不可达 / 刻意不做）
- `dev+0x3c` / `dev+0x20` bit0 写入点；`FUN_1418c0a70` 的 jumptable 分支；setup 记录最终拼接顺序；时间线信号量全局一个还是每队列一个。
- `validate_read_access`（无符号）、`PassPgoStats::extend`（无内联体）、`ResourceExportHandle<Texture>` 的字段级定义、每个 pass 调用了哪些 RenderPassApi 方法（需逐回调体反编译）。
- raster/compute 管线创建 Err 分支文案、`RasterPipelineDesc@0x30` 字符串语义、`RenderPassAttachmentDesc +0x0d..0x0f`、多宏 defines 折叠的数值样本。
- 设备排序键只读了机器码常量，没有用多设备数据在原版上复核；`GraphicsDevicesForSurface::new` 在原版内联；NVAPI/DXGI 本体由调用方提供。
- 访问码表 0/0x34/0x22 的精确语义（仍用公开访问掩码近似）；setup CB 并入图提交仍是同步版。
- 刻意不做：槽内资源内联（改 `Managed<T>` 模型 + unsafe，风险大于收益，见 wave2-structure.md）；`queue_lock` 改读写锁（会破坏 VkQueue 外部同步）；真实 GPU 提交路径从 fence+设备退役时间线迁到原版 NULL fence+队列时间线（方案见 wave2-queue-submit.md §4，需单独迁移）。
- Windows 外壳（`aftermath::windows_api`、`dxgi::SystemDxgi`）未在 Windows 目标编译验证。
- 工作区里 asset / asset-pipe / shader-compiler `original_contract` / tg-text 的测试依赖原作发行目录（`D:/game/ljxsj_92385/Tiny Glade` 或作者 mac 路径），本机没有，失败与本批改动无关。

## M13 动画经渲染图执行器 + 执行器接口修复
- `tests/triangle_animation_graph.rs`：48 帧经 `VulkanGraphSession` 执行（每帧一张图、图内创建颜色目标、temporal 历史纹理、3 个 pass、2 帧在飞）。与 CPU 参考光栅化 0 个像素不一致，质心误差 ≤0.09px，temporal 94 次复用命中、47 次历史内容逐字节一致，池命中 44/48，帧账本无告警。报告 `reports/animation-through-executor.md`。
- 该测试发现的接口问题处理结果：
  1. **token 空间冲突（已修）**：后端导入与 temporal 各自从 1 分配，会撞号（`tests/token_space.rs` 先复现：两边都拿到 1）。改为 `rg/vulkan.rs` 里的全进程单调分配器 `allocate_token/reserve_token`；外部指定 token 时把计数器推过它。
  2. **导入资源只能整体释放（已修）**：新增 `VulkanGraphSession::release_import(token)`：仍被在飞帧引用时拒绝（无副作用），帧完成后立即释放，不需要整设备 idle；同时放掉 `pass_resources` 缓存并推进完成前缀，使托管资源真正离开 store（测试用弱引用证明缓冲被销毁）。`FrameThrottle` 新增只读 `iter()`。是否匹配原版跨帧导入生命周期仍未证实。
  3. **视口翻转 Y / 无 `bind_pipeline`（核对后不是 bug，不改）**：`set_default_view_and_scissor` 的负高度视口就是原版（0x1416f7870 机器码，ALGORITHMS §RenderPassApi）；原版 `RenderPassApi` 没有 `bind_pipeline`，管线绑定在 `GraphicsDevice::bind_pipeline`（本仓库已有）。
  4. **`Buffer::read_bytes` 会等整设备（不改）**：这是该 API 的文档化语义；多帧路径用 `read_bytes_after_completion`（unsafe，调用方已等时间线）。
- 验证：rhapsody + tiny-glade + country-core + shader-pipeline（`--all-features`，lavapipe）= 50 个测试目标，1806 passed / 0 failed。
- 仍未覆盖：Khronos 校验层（本机没有，实例创建也无开关）、辅助队列/跨队列族、多 pass 大图并行分块、`candidate-optimizations` 开关对比、`begin_frame_bounded` 容量 2 语义在会话路径上。

## M14 第三批（剩余未读项 + 覆盖缺口 + 接入 core）
报告：`reports/wave3-submit.md`、`wave3-rg.md`、`rg-pass-methods.md`、`wave3-coverage.md`、`wave3-misc-reads.md`、`core-integration.md`。
验证：`cargo test -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）= 58 个测试目标，1888 passed / 0 failed（收口时自己重跑确认）。

### 再次更正的旧结论
| 旧说法 | 实际 | 证据 |
|---|---|---|
| 时间线信号量全局一个（wave2） | 每队列一个：主队列在 create 里一个（0x1418c8710 仅此一处调用），每个辅助队列在 0x1418b48f0 循环里各一个 | wave3-submit.md |
| setup 记录排在最先（wave2） | 顺序是 `[prior, setup, current]`（0x140c051b6..0x140c05597） | 同上 |
| `dev+0x3c`/`dev+0x20` 写入点找不到 | `dev+0x20..0x3f` 是 32B `Option<QueueFamilyInfo>`：bit0=Some 标签，`+0x3c`=族索引；create 里 0x140bf2e10 取首个 `flags&3==2` 的族，尾段 0x140bf6388 用 `movups` 整块写（所以字节级扫描没命中） | 同上 |
| PGO 同名耗时累加 | `FUN_1402ca150` 是 hashbrown insert，同名是**覆盖**；total 普通加，max 取大 | wave3-rg.md |
| compile 的 `ResourceExportHandle` = 0x218B | `Ok` 载荷 0x218B 编译图；加载图/世界图后跟 4B u32 导出句柄（整体 0x220），展示图无句柄；`Err` 只写首 qword = `i64::MIN`，失败条件是 `populate_resources` 返回非 0。u32 为逻辑资源下标属推断 | 同上 |
| `validate_read_access` 未知 | 两个实例都折叠到 0x14003bb80，只有一条 `ret`：原版就是空操作 | 同上 |
| `RenderPassAttachmentDesc +0x0d..0x0f` 被 new 清零 | 是尾部填充，`new` 不清零 | wave3-misc-reads.md |
| 多宏 defines 折叠只有机器码依据 | `original-compute-tuples.json` 里 5 组含 2 个宏，10 个键用 `artifact_hash` 逐位对拍一致；折叠与顺序无关，原版不排序不去重 | 同上 |
| 访问码 0/0x34/0x22 用公开掩码近似 | 访问码表给出精确 stage/access/layout，旧近似数值上恰好正确；`setup.rs` 改为查表驱动 | 同上 |
| `DxgiAdapter.device_id` | 原 40B 记录里不存在，原版只取 name/luid/vendorId | 同上 |
| `Type::nbyte` Vector/Matrix/Array 未对拍 | 14 项跳表逐 case 解出并全网格对拍，实现无需改 | 同上 |
| `mark_stale` 取锁方式未知 | 写锁，`@0x30==2` 才拷贝 desc；`is_stale` 原版不存在；`@0xa8`：2 活动/3 已退役/4 已释放 | wave3-submit.md（仅更新文档，行为代码未改） |

### 本批新增/对齐
- 提交：`SubmissionEngine` 记录顺序改 `[prior, setup, current]`；`rg/frame.rs::debug_sync_wait`（慢提交两条 Info 日志 + sleep 2s + wait_idle；**未接入执行器提交路径**）；`begin_frame` 容量 2 断言；ash 函数表偏移（`dev+0x78` 起，VkDevice@+0x640，`+0x4f0` get_semaphore_counter_value、`+0x4f8` wait_semaphores）用 `offset_of!` 测试核对。
- GPU 提交迁移：**部分对齐**。会话路径本来就走 `DeviceState::queue_submit`（NULL fence + 每队列时间线）；fence 只在 `submit_to`（宿主读回、tg-text 用）。`retire_timeline` 与 `queue_lock` 保留：`queue_lock` 仍被 `setup.rs`/`buffer.rs` 使用，`PendingCommand.fence` 是 pub，辅助队列无时间线可替；没有每队列字节锁就把它改成读锁会让 `vkQueueSubmit` 与 present 在同一个 VkQueue 上并发。迁移前提清单见 wave3-submit.md。
- rg：`record/extend` 覆盖语义；compile 返回布局 `repr(C)` + 偏移测试；`original_slots`（pass 回调的设备槽常量）；59 个 pass 的 RenderPassApi 方法清单（原版只有 6 种共享回调体，全经 `PassBuilder::render` 装箱）；TemporalRenderGraphRecorder 总长 0x110（`+0xa8` &GraphicsDevice，`+0xb0..` 两张表，第二张奇偶翻转）。resource key 不改（剩下的 String 键是反射描述符名，原版在构建时解析成 `BindingIndex`）。
- raw 杂项：管线创建 panic 文案（`original_panics` + 4 个 `InterfaceError` 变体）、两条 framebuffer 文案接入 `Framebuffer::from_desc`、`flag@0x16`=`ResourceType` 字节、设备排序全矩阵测试（5 类别×4 名字×6 堆大小）。
- 覆盖：`InstanceBuilder::enable_validation()` + 收集器；40 pass 大图在串行/`batch_per_pass`/原作分块/`Balanced`/`ADAPTIVE` 下逐字节一致（预算 `clamp(20,30,240)=30`，15 块 16 命令缓冲，并行录制确实发生）；`candidate-optimizations` 开/关下相关 6 个目标通过。
- Windows 外壳：msvc 目标 lib 检查发现 `aftermath.rs`/`dxgi.rs` 的 `extern` 需 `unsafe extern`，**已修**；msvc 测试编译/链接/运行未验证。

### 接入 core（`reports/core-integration.md`）
- core 的 `RenderPassApi` 只有 fill/clear/copy；光栅绘制走 `draw_stream::PassHost`，接线前宿主一个 `PassHost` 方法都没实现。
- 新增 `crates/tiny-glade/src/raster_host.rs`（`RasterPassHost`/`RasterShared`/`GraphRgResources`）：`RasterPass::render` → 通道缓存（`RenderPass::from_desc`，0x98B 键去重）→ 管线 → 描述符集 → draw，在 `VulkanGraphSession` 执行。`NoVao`/`NoIndexBuffer`/`NoVaoIndexed` 真画；`InstancedGlDrawFlag`、`MultiInstanced`、各种 indirect 在声明期 panic（文案含“未证实”，不猜映射）。
- `tests/core_raster_draw.rs`：单帧 509 像素、不一致 0、质心误差 0.049px；30 帧 2 帧在飞，每帧 512 像素、不一致 0、质心最大误差 0.09px，在飞峰值 2，30 帧共用 1 个通道 1 条管线；深度附件远近两种绘制顺序，重叠区均 180/180；另有 4 个桥接/边界测试。
- 修的 bug（先写失败测试）：`DrawBuilder` 的 uniform 字节随 builder 丢弃到不了 GPU（新增 `PassHost::flush_dynamic_constants`）；`TextureDesc::new_2d/new_2d_array/new_3d` 没写 kind 字节导致图内创建的 2D 纹理成 1D；缓冲导入 token 与后端图内资源 token 撞车（宿主加 `graph_token()`）。
- 契约 9 条：宿主槽表方案保留（core 句柄是 `Copy` 且 core 不依赖 rhapsody）；#7（Buffer 40B）已做，`transmute` 测试证明与 `BufferRecord` 逐字节一致；#3/#4/#8/#9 部分落地；#1/#2/#5/#6 不做（理由在报告）。
- 没做到：未证实的 draw 种类、push constants、temporal `RgResources`、资产库对接、MSAA/深度变体的像素验证、上层 `FramePasses`/`WorldAccess` 一族。

### 仍未闭合
- 原版证据不可达：`inner+0x7c==2` 的 take 写者；`RasterPipelineDesc@0x30` 的读取者；`ResourceExportHandle` 的 Rust 类型名；Record 少数字段名（`+0x24..0x30`、`+0x67`）；`FUN_1418c0a70` 7 个分支各对应哪种资源；加载图/展示图闭包内入口清单；`dev+0x20` bit0 以外 `QueueFamilyInfo` 的其余字段名。
- 本机无法验证：Khronos 校验层（有层机器上运行 `VK_LAYER_PATH=<层目录> cargo test -p rhapsody --test wave3_validation -- --nocapture`）；辅助队列执行测试（已写好、可编译，但 lavapipe 只有一个通用族，从未实际运行）；Windows 目标的测试运行。
- 刻意不做：槽内资源内联；`queue_lock` 改读锁；整体迁到 NULL fence + 队列时间线（见上，前提清单在 wave3-submit.md）。
