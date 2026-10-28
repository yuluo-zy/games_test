---
doc: rhapsody-improvement-tasks
schema: tasks-v1
date: 2026-10-28
findings_doc: FILE_AUDIT.md
tasks_total: 43
by_priority: {P0: 2, P1: 7, P2: 19, P3: 15}
status_values: [pending, in_progress, done, dropped]
status_note: 本文件只登记当前仍未完成的任务定义；完成状态按项目规则写入 REFACTOR_PROGRESS.md
---

# rhapsody 改善任务台账（机器可读版）

## 0. 给 AI 读者的说明

- 本文是**任务台账**：每条任务回答"改什么、在哪里、怎么验收"。问题证据与逐项核实结论在 [FILE_AUDIT.md](FILE_AUDIT.md)，用 `F-xxx` 编号。
- 字段（§1 表格）：`ID | 优先级 | 类别 | 状态 | 批次 | 位置 | 关联发现 | 任务`。
- 优先级：`P0`（正确性/安全，立即处理）> `P1`（明确缺陷或高收益重构）> `P2`（规范/性能/测试）> `P3`（可选优化）。
- 状态：`pending`（未开始）| `in_progress` | `done` | `dropped`。本文件只列 `pending`；完成后不在此文件保留 `done` 条目，完成记录写入 `REFACTOR_PROGRESS.md`。
- 位置以**符号名与当前路径**为准（行号仅供参考，源码移动后以 `rg` 重新定位）。源码已按 `accounting/`、`bindless/`、`collections/`、`command/`、`device/`、`native/`、`present/`、`resource/`、`rg/`、`tracking/`、`transfer/` 等目录重组，旧的 `src/raw/*` 路径已不存在。
- 方括号 `[决策]` 的任务需要项目所有者先拍板（见 §5），不要自行实施。
- 执行顺序见 §4；不要合并/重构的"刻意设计"清单见 §3。
- 统计口径：`rg` 计数均针对 `crates/rhapsody/src`（含内联测试），2026-10-28 重算。

## 1. 任务索引

| ID | 优先级 | 类别 | 状态 | 批次 | 位置 | 关联发现 | 任务 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IMP-38 | P0 | test | pending | 1 | `src/**`、`tests/**` 中 `eprintln!("SKIP` 共 32 处 | — | GPU 测试无设备时静默通过 |
| IMP-03 | P0 | safety | pending | 1 | `src/native/aftermath.rs`、`src/native/instance/mod.rs` 等 | — | unsafe 块缺 `// SAFETY:` 说明 |
| IMP-04 | P1 | perf | pending | 4 | `src/pipeline/compiler.rs` `request_inner`；`src/pipeline/request.rs` `matches_key` | F-110 | pipeline 缓存线性扫描 + 第二套 VecDeque LRU |
| IMP-05 | P1 | perf | pending | 4 | `src/rg/result_cache/mod.rs` `ResultKeyPayload` | — | 结果缓存 key 每次全量 Hash |
| IMP-07 | P1 | duplication | pending | 3 | `src/tracking/tracked/mod.rs` `trait GpuUse` | — | `visit_uses` 无默认实现，21 处手写实现 |
| IMP-32 | P1 | concurrency | pending | 4 | `src/log_ring/mod.rs`、`src/tracking/tracked/mod.rs` 等 | — | （决策）并发协议无模型检查 |
| IMP-39 | P1 | test | pending | 1 | 仓库内无 `.github`、`rust-toolchain*`、`xtask` | — | （决策）仓库没有 CI 闸门 |
| IMP-40 | P1 | perf | pending | 1 | `tests/p0_perf_baseline.rs`、`tests/core_perf.rs`、`tests/parallel_dispatch_perf.rs` | — | 性能基线没有回归防线 |
| IMP-44 | P1 | dead-code | pending | 3 | 42 个零引用定义（见详情）+ 1 个文档化开关 | — | （决策）死代码清理 |
| IMP-09 | P2 | duplication | pending | 3 | `src/rg/frame_storage/mod.rs:41,47`、`src/rg/receipt_storage/mod.rs:74,77` 等 | — | `reserve<T>` / `bytes<T>` 辅助函数复制 |
| IMP-10 | P2 | duplication | pending | 3 | `src/resource/buffer/create.rs:67`、`src/resource/texture/create.rs:152` | — | `new_concurrent` 构造样板 |
| IMP-11 | P2 | api | pending | 2 | 全库 `#[must_use]` 仅 3 处 | — | 对"丢弃即漏处理"的 API 补 `#[must_use]` |
| IMP-12 | P2 | error-handling | pending | 2 | `src/rg/vulkan/session/display.rs:26`、`compile.rs:40`、`execute.rs:302` 等 | — | 裸 `unwrap` 改为带不变量文案的 `expect` |
| IMP-13 | P2 | observability | pending | 2 | 全库 `trace!` 0 处，`debug!` 11 处 | — | 诊断日志密度不足 |
| IMP-14 | P2 | observability | pending | 2 | `src/error/mod.rs` `invalid_usage` / `invalid_usage_with` | — | 日志级别与 ARCHITECTURE 文档不符 |
| IMP-15 | P2 | doc | pending | 2 | `OPTIMIZATIONS.md:110`（代码中对应符号已不存在） | — | 文档声称的分配模式开关不存在 |
| IMP-16 | P2 | dead-code | pending | 2 | `src/lib.rs` `internal_mod!`（`log_ring, retire, trace`） | — | 模块级宽泛 `allow(dead_code, unused_imports)` |
| IMP-17 | P2 | quality | pending | 2 | `src/resource/bytes.rs:10,19` | — | 手写 `unsafe` 字节重解释，未用 `bytemuck` |
| IMP-19 | P2 | test | pending | 1 | 根 `Cargo.toml` `[workspace.lints.clippy]` | — | 没有 pedantic/unsafe 文档类 lint 子集 |
| IMP-28 | P2 | architecture | pending | 3 | `src/tracking/range_state/storage.rs:4-6` | — | 范围状态机依赖 accounting 预算类型 |
| IMP-30 | P2 | architecture | pending | 5 | `src/device/**` ↔ `src/native/**` | — | （决策）device 与 native 双向依赖 |
| IMP-31 | P2 | doc | pending | 2 | `ARCHITECTURE.md` 的「错误模型」「不变量」；`tools/` | — | 文档中可机械检查的规则没有脚本固化 |
| IMP-33 | P2 | concurrency | pending | 3 | `src/tracking/tracked/mod.rs` `enter/leave/is_plan_private` | — | `plan_private` 使用 Relaxed，跨线程语义需确认 |
| IMP-35 | P2 | lifecycle | pending | 3 | `src/device/lifetime.rs:80,85`、`src/present/swapchain/mod.rs:270-273` 等 | — | `mem::forget` 的使用约定未统一表达 |
| IMP-36 | P2 | api | pending | 3 | 公开模块（`lib.rs` 再导出的 10 个模块） | — | 公开面收窄与 `missing_docs` 未启用 |
| IMP-41 | P2 | test | pending | 5 | `src/pipeline/disk.rs` `decode`；无 `fuzz/` 目录 | — | （决策）缺少模糊测试 |
| IMP-59 | P2 | error-handling | pending | 2 | `src/transfer/queue/mod.rs` 多处；`src/transfer/route.rs`、`submit.rs`、`flush.rs` | — | 容量错误仍映射为 `Timeout` |
| IMP-64 | P2 | perf | pending | 4 | 见详情表（rg 计划/绑定/缓存热路径） | F-092 F-093 F-097 F-170 F-175 F-176 F-187 | 超线性热路径 |
| IMP-20 | P3 | observability | pending | 2 | `src/error/mod.rs` `Error` 的 `Display` | — | 日志语言与公共错误文案语言混用 |
| IMP-21 | P3 | observability | pending | 2 | `src/present/swapchain/frame_sync.rs:154`、`progress.rs:63,225` | — | `log` target 为过期的 `rhapsody::raw::swapchain` |
| IMP-22 | P3 | observability | pending | 2 | 根 `Cargo.toml:97` `log-once` 已声明但未使用 | — | 重复失败日志无限制 |
| IMP-23 | P3 | api | pending | 3 | 全库 `pub fn raw_handle` 16 处、`pub fn device` 15 处 | — | 访问器样板 |
| IMP-24 | P3 | error-handling | pending | 3 | `src/rg/executor/error.rs:7`、`src/command/descriptor_set/mod.rs:639` | — | 错误枚举名称与 crate 级 `Error` 同名 |
| IMP-25 | P3 | quality | pending | 3 | `src/format/registry.rs`（1642 行）、`src/transfer/queue/mod.rs`（890）、`src/rg/result_cache/mod.rs`（818） | — | 非测试文件超过 800 行（共 3 个） |
| IMP-26 | P3 | quality | pending | 3 | 全库 `&Arc<` 形参 178 处 | — | 逐个审计，不做机械替换 |
| IMP-27 | P3 | quality | pending | 3 | 全库 `as u32` 107、`as u8` 15、`as usize` 396 | — | 用户可控长度的窄化转换 |
| IMP-37 | P3 | api | pending | 3 | 全库 `: bool` 形参 307 处（含字段，未逐个筛选） | — | 布尔盲参 |
| IMP-42 | P3 | quality | pending | 3 | `src/native/surface.rs:105` `create_win32` | — | 平台支持矩阵与 32 位假设未文档化 |
| IMP-43 | P3 | api | pending | 3 | 全库 timeout 相关 116 行，无 `WaitPolicy` | — | 超时语义分散 |
| IMP-45 | P3 | api | pending | 3 | `pipeline/shader/raster.rs` `set_*`、`accounting/budget/mod.rs:99,178,201` 等 | — | 仅测试使用的生产 API |
| IMP-49 | P3 | duplication | pending | 3 | `src/bindless/stats.rs:17` `BindlessStats` | — | 扁平字段与 `tables` 聚合并存 |
| IMP-50 | P3 | correctness | pending | 3 | `src/resource/managed/texture.rs:128` `ViewKey` | — | 与 `TextureViewDesc` 手工镜像 |
| IMP-65 | P3 | test | pending | 3 | `src/rg/transient_resources/tests.rs` | F-168 | `lend`/`give_back`/`evict`/`collect_retiring` 时序无测试 |

## 2. 任务详情

### IMP-38 [P0] GPU 测试无设备时静默通过

**问题**：无 Vulkan 设备时测试打印 `SKIP` 后直接 `return`，报告为 PASS。`rg 'eprintln!\("SKIP'` 在 `src/` 与 `tests/` 合计 32 处（例：`src/command/pending_acquire/tests.rs:197`、`src/transfer/tests.rs:104`、`tests/raw_fixes_swapchain.rs:120`、`tests/gpu_test_support/mod.rs:22`）。CI 容器或无 GPU 的开发机上，这些测试全绿但未测任何东西。另有 `#[ignore]` 12 处，属显式手动运行，不在本任务内。

**动作（推荐 A）**：

- A. 把 `SKIP` 改为 `#[ignore = "requires Vulkan"]`，并在共享测试支撑（`tests/gpu_test_support/`）中读取显式环境变量（如 `RHAPSODY_REQUIRE_VULKAN=1`），设置后无设备即失败。
- B. 保留 SKIP，但在测试进程末尾断言本次至少执行了 1 个 GPU 用例。
- C. 在 CI 中用 lavapipe 运行，本地允许 SKIP。

**验收**：无设备环境下测试报告能区分"通过"与"跳过"；设置强制变量后无设备必须失败。

### IMP-03 [P0] unsafe 缺 `// SAFETY:` 说明

**数据**（`src/` 全目录，`rg` 口径）：`unsafe {` / `unsafe fn` / `unsafe impl` / `unsafe trait` 共 399 处；`// SAFETY:` 仅 62 处。

| 文件 | unsafe 块/fn/impl 行 | SAFETY 注释 |
| --- | ---: | ---: |
| `src/native/instance/mod.rs` | 24 | 2 |
| `src/present/swapchain/recreate.rs` | 15 | 0 |
| `src/native/aftermath.rs` | 17 | 11 |
| `src/native/physical_device/mod.rs` | 13 | 0 |
| `src/resource/buffer/host.rs` | 12 | 2 |
| `src/resource/buffer/create.rs` | 12 | 1 |
| `src/rg/profiling/mod.rs`、`src/native/surface.rs`、`src/device/init/mod.rs` | 11 各 | 0 |
| `src/native/dxgi.rs` | 7 | 6 |

已正确的部分：根 `Cargo.toml` `[workspace.lints.rust]` 中 `unsafe_op_in_unsafe_fn = "deny"`；未发现手写 `unsafe impl Send/Sync`。

**动作**：按文件推进，优先 `native/instance/mod.rs`、`present/swapchain/recreate.rs`、`native/physical_device/mod.rs`；配合 IMP-19 启用 `clippy::undocumented_unsafe_blocks`。

**验收**：上表前 6 个文件的 unsafe 块全部有 `// SAFETY:` 或 `#[allow(clippy::undocumented_unsafe_blocks = "reason")]` 并附理由。

### IMP-04 [P1] pipeline 缓存线性扫描与第二套 LRU

**位置**：`src/pipeline/compiler.rs` `request_inner`（`state.cached.iter().find(|(key, _)| job.matches_key(key))`，约 :177）；`state.lru` 为 `VecDeque`，用 `retain` 维护（约 :329、:394）；`src/pipeline/request.rs` `matches_key`（:99）。

**问题**：每次请求对全部缓存项做语义比较，O(n)；LRU 的 `retain` 也是 O(n)。`collections/cache/mod.rs` 已有 O(1) 侵入式 LRU，pipeline 没有复用。

**动作**：

1. 请求 key 实现 `Hash + Eq`，命中路径改哈希查找；`matches_key` 只作确认。
2. LRU 改用 `collections::cache::BoundedCache`（它已支持 `lru_key_where` 作为"只淘汰非 Pending 项"的谓词）。

**验收**：`tests/pipeline_cache.rs`、`tests/async_pipeline.rs` 通过；重复请求同一 key 时不发生全表扫描（计数器断言）。

### IMP-05 [P1] 结果缓存 key 每次全量哈希

**位置**：`src/rg/result_cache/mod.rs` `type ResultKey = Arc<ResultKeyPayload>`（:84），`ResultKeyPayload` `#[derive(Hash)]`（:85-86），包含依赖版本列表、参数字节、描述符、共享信息等全部字段。

**问题**：每次 `get` 都重新哈希整个 payload。`collections/token_map/mod.rs:5` 注释已写明默认 SipHash 是可测量的每帧成本，但更重的 key 仍走默认哈希路径（**未核实**：缓存 `HashMap` 的 hasher 类型需在改动前确认）。

**动作**：构造 key 时计算一次 `u64` 指纹并存入 payload；`Hash` 只写指纹；map 使用与 `TokenMap` 相同的 `BuildHasherDefault`。**必须保留**完整 key 的 `Eq`，命中判定不能只看指纹。

**验收**：`tests/preserving_dedup.rs`、`tests/result_cache_vulkan.rs` 通过；人为构造指纹碰撞的测试仍能区分两个不同 key。

### IMP-07 [P1] `GpuUse::visit_uses` 无默认实现，实现分散

**位置**：`src/tracking/tracked/mod.rs` `pub trait GpuUse`（:736）：`fn visit_uses` 必须实现，已有 `held_until_completion` 默认方法。`visit_uses` 的实现点 21 处（`rg` 口径），例如 `src/device/semaphore.rs:124`、`src/resource/buffer/create.rs:40`、`src/resource/texture/create.rs:39`、`src/resource/sampler.rs:330`、`src/command/descriptor_set/mod.rs:739,750,756`、`src/resource/managed/texture.rs:163`、`src/bindless/lease.rs:27,56`。

**问题**：`visit_uses` 各实现的函数体是否仍逐字节相同（semaphore/descriptor_set/sampler/buffer/texture）**未核实**；但现状是每个资源类型都手写 cell 遍历，新增资源类型时容易漏登记 `SyncCell`。

**动作**：增加 `fn use_cells(&self) -> impl Iterator<Item = &Arc<SyncCell>>`（或等价的数组视图）作为必须实现的方法，`visit_uses` 由 trait 提供默认实现；逐个迁移 21 处实现，并核对重复体。

**验收**：`tests/retained_declaration.rs`、`tests/pass_bindings.rs`、`tests/resource_contracts.rs` 通过；`rg "fn visit_uses"` 的非默认实现数量下降且每个资源类型只声明一次 cell 列表。

### IMP-32 [P1] [决策] 并发协议缺少模型检查

**候选（手写发布/回收协议）**：

| 协议 | 位置 | 手写同步元素 |
| --- | --- | --- |
| 日志环发布 | `src/log_ring/mod.rs`（`record_log`） | 每槽 sequence + fence |
| 资源使用登记 | `src/tracking/tracked/mod.rs` | 原子值 + submission gate |
| 退役队列 | `src/retire/mod.rs`、`src/device/lifetime.rs` | 有序前缀 + 预留 |
| 延迟 flush 队列 | `src/device/host_flush/mod.rs` | `dirty` 标志 + 原子快路径 |
| 动态常量页 | `src/resource/dynamic_constants/mod.rs` | 页游标与 owner 锁 |

**依赖问题**：`loom` 未在依赖中，属于依赖变更，需审批。

**选项**：短期把不变式写入文档并加单测；中期申请 `loom` 作为 dev-dependency，仅在 `#[cfg(loom)]` 下编译。

**验收**：至少 `log_ring` 与 `HostFlushQueue` 有 loom 模型或等价的形式化说明。

### IMP-39 [P1] [决策] 仓库没有 CI 闸门

**现状**：`rhapsody` 与 `tiny-glade` 根下没有 `.github/`、`rust-toolchain*`、`deny.toml`（仅 `third_party/winit-*/deny.toml`）、`justfile`、`Makefile`、`xtask/`。质量命令只写在 `OPTIMIZATIONS.md` 中，靠人执行。

**最小闸门建议（按成本排序）**：

1. `cargo fmt --check` 与 `cargo clippy -p rhapsody --lib -- -D warnings`。
2. `cargo test -p rhapsody`（含 `--no-default-features --lib`），GPU 用例在 lavapipe 容器中执行。
3. 固定 MSRV：`rust-toolchain.toml` 或 CI 矩阵，使 `rust-version` 可被验证。
4. `cargo deny check`（advisories/licenses/bans）。

**验收**：任一 PR 违反闸门即失败；`OPTIMIZATIONS.md` 中的命令与 CI 一致。

### IMP-40 [P1] 性能没有回归防线

**现状**：

- `tests/core_perf.rs:13`、`tests/parallel_dispatch_perf.rs:27`：`#[ignore = "manual performance probe"]`。
- `tests/p0_perf_baseline.rs`：多个用例 `#[ignore]`；`:513` 的基线"打印数字不做断言"；`:825` 起的严格验收仅在 release 手动运行。
- `OPTIMIZATIONS.md` 中的实测数字来自手动运行，无法在后续改动中复用。

**动作**：

1. 基线测试输出机器可读 JSON（含耗时），沿用现有 manifest 结构。
2. 提供比较脚本（`tools/` 下）：对比两次运行，超过阈值（首次只报警）即失败。
3. 与 IMP-04、IMP-05、IMP-64 的改造配对：改前存基线，改后对比。

**验收**：比较脚本能对两份基线给出 pass/warn/fail 结论，并在文档中说明运行方式。

### IMP-44 [P1] [决策] 42 个零引用定义（+1 个文档化开关）

**说明**：2026-10-28 用 `rg -w` 在 `crates/` 与 `tests/` 中统计引用。下列 42 项在当前代码中只剩定义，无任何引用；同名以外的旧符号在当前代码中已不存在。`set_plan_validation` 是 `ARCHITECTURE.md:182` 与 `OPTIMIZATIONS.md:64` 文档化的运行时开关，**保留**，但需补一个调用示例或测试，避免下次被判死。

| 类别 | 符号（位置） |
| --- | --- |
| 原生调用/调试设施 | `begin_label`、`end_label`（`device/debug.rs:34,49`）；`record_crash_marker`、`retain_debug_buffer`、`make_host_readable`、`log_vulkan_error_info`、`log_error_info`（`command/crash/mod.rs`）；`set_stencil_reference`（`command/command_buffer/mod.rs:186`）；`set_texture_debug_name`（`resource/device_api.rs:88`）；`vulkan_debug_messenger`（`native/instance/mod.rs:426`） |
| 绑定/注册 | `bind_descriptor_set`、`bind_persistent_descriptor_set_single`（`command/binding.rs:137,164`）；`is_transient`（`command/descriptor_set/mod.rs:502`）；`register_compiled_raster`（`pipeline/shader/library.rs:251`）；`from_compiled_container`（`pipeline/shader/interface.rs:360`）；`shader_count`（`pipeline/shader/pipeline.rs:448`）；`failed_creating_shader_pipeline`、`UNABLE_TO_CREATE_GRAPHICS_PIPELINE`、`UNWRAP_ON_ERR`（`pipeline/shader/reflection.rs:70-77`） |
| 资源/常量 | `CREATE_BUFFER`、`BIND_BUFFER_MEMORY`（`resource/buffer/memory.rs:59,60`）；`CREATE_IMAGE`、`BIND_IMAGE_MEMORY`（`resource/texture/info.rs:23,25`）；`new_buffer`（`resource/managed/buffer.rs:11`）；`is_sampled_color`（`resource/texture/desc.rs:194`）；`bytes_as_slice`（`resource/bytes.rs:19`，见 IMP-17） |
| 设备/物理设备查询 | `available_device_local_budget`、`BUDGET_SNAPSHOT_BYTES`、`is_amd`、`is_apple`、`vendor_performance_info`、`enumerate_vk_data`、`string_from_c_array`、`parse_driver_version`（`native/physical_device/mod.rs`） |
| 格式 | `try_vk_format_block_bytes`、`try_vk_format_valid_aspect_flags`、`write_into`（`format/mod.rs:77,85,110`） |
| 呈现/配置 | `failure_policy`、`submission_fence`（`present/swapchain/lease.rs:120,276`）；`configure_cpu_arenas`、`configure_frame_storage`（`rg/vulkan/backend/configure.rs:66,87`）；`frame_storage_statistics`（`rg/vulkan/backend/stats.rs:46`） |

**处置**：历史/原版 API 残留直接删除；诊断设施（崩溃标记、调试标签）需先决策接入或删除；查询函数保留到支持矩阵（IMP-42）落地前再决定。

**验收**：逐项 `rg -w` 确认后删除；`cargo check --all-targets` 无新增告警。

### IMP-09 [P2] 小工具函数收敛

| 函数 | 位置（rg 口径） |
| --- | --- |
| `fn reserve<T>(&mut Vec<T>, len)` | `src/rg/graph/backend.rs:149`、`src/rg/frame_storage/mod.rs:47`、`src/rg/receipt_storage/mod.rs:77`；`src/device/submission/mod.rs:152`、`src/command/command_buffer/submission.rs:365`（内部版本）；`src/transfer/queue/mod.rs:297`；`src/rg/pass_bindings/projection/mod.rs:102` |
| `fn bytes<T>(...)` | `src/rg/frame_storage/mod.rs:41`、`src/rg/receipt_storage/mod.rs:74`、`src/native/submit/mod.rs:93,148` |
| 对齐 | `src/collections/align.rs:5` 的 `align_up` 已存在；`src/device/suballoc/mod.rs:458` 的 `round_up` 仍独立（**未核实**其溢出语义） |

**动作**：`reserve`/`bytes` 收敛到 `collections/` 下的一个模块；对齐统一到 `collections::align`，使用 `checked_next_multiple_of` 或 `Option` 版本。不要放进 `crates/utils`（它依赖 bevy/glam，会倒置层次）。

**验收**：`rg "fn reserve<T>|fn bytes<T>"` 各只剩一处；`tests/suballocation_native.rs`、`tests/allocator.rs` 通过。

### IMP-10 [P2] `Buffer` / `Texture` 的 `new_concurrent` 样板

**位置**：`src/resource/buffer/create.rs:67`、`src/resource/texture/create.rs:152`。两者在归一化标识符后结构相同，只有 desc 类型与被调函数名不同。

**动作**：抽 `fn concurrent_sharing(families: &[u32]) -> Result<ResourceSharing>`，两处保留一行调用。

**验收**：两处 `new_concurrent` 体均不超过一行逻辑；现有 buffer/texture 测试通过。

### IMP-11 [P2] `#[must_use]` 几乎为 0

**现状**：全库 `must_use` 仅 3 处。

**动作**：只给"返回裸值且丢弃即漏处理"的 API 加：`trim_idle`（各池，返回释放量）、`cancel_queued_before`（`pipeline/compiler.rs:296`）、各池的 `checkout`/lease。`Result` 已自带 `must_use`。

**验收**：上述 API 带 `#[must_use]`；`cargo clippy -p rhapsody --lib` 无新增告警。

### IMP-12 [P2] 裸 `unwrap` 改为带理由的 `expect`

**位置（生产代码，非测试）**：`src/rg/vulkan/session/display.rs:26`、`compile.rs:40`、`execute.rs:302`（`pending.front()/pop_front().unwrap()`）；`src/rg/temporal/mod.rs:359`；`src/command/crash/mod.rs:49`；`src/retire/mod.rs:106,112`；`src/native/instance/mod.rs` 中 `CString::new(...).unwrap()` 共 5 处。非测试代码 `unwrap()` 约 36 处（`rg` 口径，粗略计数）。

**动作**：每处改为带不变量文案的 `expect`（与 `src/rg/vulkan/backend/execution.rs` 已有的 `expect("owned command")` 风格一致）。在 `ARCHITECTURE.md` 规则 4 补充"消息文案即理由"。

**验收**：上述位置不再出现裸 `unwrap()`；`rg "\.unwrap\(\)"` 在非测试代码中仅剩带注释的例外。

### IMP-13 [P2] 日志密度：`trace!` 为 0

**现状**：`src/` 中 `trace!` 0 处，`debug!` 11 处。`OPTIMIZATIONS.md` 声称"记录缓存失效原因、锁等待、内存高水位"，实际可拉取的只有 `stats()`。

**动作**：在有诊断价值的位置补 `trace!`/`debug!`，不要用 `info!` 刷屏。建议首批：设备创建成功后的物理设备名与队列布局（`native/physical_device/mod.rs`、`device/init/mod.rs`）；计划缓存命中/未命中原因；淘汰/驱逐释放量（`rg/graph_caches.rs` 的 `evict_idle`）。

**验收**：上述路径在 `RUST_LOG=trace` 下有输出；默认 Info 级别不增加任何行。

### IMP-14 [P2] `invalid_usage` 的日志级别与文档不符

**现状**：`ARCHITECTURE.md:110` 规则 3 写"`Error::invalid_usage(..)`（同时记 error 日志）"；实现 `src/error/mod.rs` `invalid_usage`（约 :151-155）与 `invalid_usage_with`（约 :158-163）记的是 `log::debug!`。

**判断**：实现更合理（误用常在重试循环中发生，`error!` 会刷屏）。**动作：改文档**，把规则 3 改为"记 debug 日志，限流由调用方负责"；是否限流见 IMP-22。

**验收**：`ARCHITECTURE.md` 与实现一致。

### IMP-15 [P2] 文档声称的分配模式开关已不存在

**现状**：代码中 `RHAPSODY_ALLOCATION`、`set_default_allocation_mode`、`default_allocation_mode` 的出现次数为 0；原 `raw/suballoc.rs` 已不存在。但 `OPTIMIZATIONS.md:110` 仍写"运行时用环境变量 `RHAPSODY_ALLOCATION=dedicated` 或 `set_default_allocation_mode` 切到每资源专用分配"，`OPTIMIZATIONS.md:64` 亦相关。

**动作**：二选一。(a) 修正文档，说明当前只有 `suballocation` 特性与 `DeviceConfig` 的分配选项（先核实其字段）；(b) 若需要运行时切换，把分配模式并入 `DeviceConfig`，环境变量只在 CLI（`tiny-glade` 层）解析后传入。**库代码不应直接读环境变量。**

**验收**：文档中不再出现不存在的符号；若实现 (b)，`DeviceConfig` 测试覆盖两种模式。

### IMP-16 [P2] `internal_mod!` 的宽泛 `allow`

**位置**：`src/lib.rs` 宏 `internal_mod!`（:24-30，`#[allow(dead_code, unused_imports)]`），调用于 :51（`internal_mod!(log_ring, retire, trace);`）。

**影响**：掩盖跨模块未使用项（例如 `log_ring` 的公开面只有 `record_log` 与 dump 类入口，任何未使用项都不会告警）。

**动作**：把 allow 下沉到确有需要的具体项，附 reason，照其他文件已用的风格。

**验收**：`cargo check` 去掉宏内 allow 后，无新增告警或仅剩附理由的具体项。

### IMP-17 [P2] `resource/bytes.rs` 改用 `bytemuck`

**现状**：`src/resource/bytes.rs:10` `pub unsafe fn slice_as_bytes<T>`、`:19` `pub unsafe fn bytes_as_slice<T>` 为手写重解释；根 `Cargo.toml` 的 workspace 依赖中已钉死 `bytemuck`，但 rhapsody 未依赖。`bytes_as_slice` 目前零引用（见 IMP-44）。

**动作**：内部改用 `bytemuck::bytes_of` / `pod_read_unaligned`。`slice_as_bytes` 若被外部 crate 当作 API 使用，保留对外签名并内部转调；`bytes_as_slice` 零引用则直接删除。

**验收**：`cargo check -p rhapsody` 无新增 unsafe；`tests/host_buffer_ranges.rs`、`tests/raw_core_buffer_texture.rs` 通过。

### IMP-19 [P2] 引入 pedantic 子集

**现状**：根 `Cargo.toml` `[workspace.lints.clippy]` 仅 `all = "warn"`；`[workspace.lints.rust]` 仅 `unsafe_op_in_unsafe_fn = "deny"`。`undocumented_unsafe_blocks` 未启用。

**动作**：按白名单逐步启用：`clippy::undocumented_unsafe_blocks`（配合 IMP-03）、`clippy::must_use_candidate`（配合 IMP-11）；评估后再开 `cast_possible_truncation`、`ptr_arg`。

**验收**：`cargo clippy -p rhapsody --lib -- -D warnings` 通过；新增 lint 在 `rhapsody/Cargo.toml` 的 `[lints]` 中列出白名单文件与移除条件。

### IMP-28 [P2] 范围状态机依赖预算/分配记账类型

**现状**：范围状态机已位于 `src/tracking/range_state/`（不再是 `resource/`）。`storage.rs:4-6` 直接依赖 `crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation}`。引用方集中于 `tracking/tracked/mod.rs`（19 处）、`command/command_buffer/recorded_state.rs`（16）、`rg/sync_plan/`（13+）。

**问题**：状态逻辑与预算记账耦合，无法单独测试状态机。

**动作**：把存储/预算相关的记账（`storage.rs` 的容量与预留）留在 `accounting` 侧的适配器，`range_state` 的核心（`table.rs`、`ops.rs`、`patch.rs`）只依赖纯词汇类型。先做依赖评估，若成本高则在 `ARCHITECTURE.md` 记录现状。

**验收**：`tracking/range_state/` 的核心模块不再 `use crate::accounting`；或文档说明保留理由。

### IMP-30 [P2] [决策] device 与 native 双向依赖

**证据**（`rg` 口径，按引用行计）：`src/device/**` 中 `crate::native` 引用 18 处；`src/native/**` 中 `crate::device` 引用 3 处。

**三个选项**：

| 选项 | 做法 | 代价 |
| --- | --- | --- |
| A. 保持现状 + 文档化 | 在 `ARCHITECTURE.md` 声明 device 与 native 的双向引用范围 | 低，但分层图需要容忍例外 |
| B. 合并 | `native` 中的设备相关类型并入 `device` | 中高，移动文件较多，公开面需保持 |
| C. 反转依赖 | `native` 只接收 `&ash::Device` 等参数，不持有 `Arc<Device>` | 高 |

**建议**：先 A，完成 IMP-28 后再评估 B。

### IMP-31 [P2] 文档中可机械检查的规则没有脚本固化

`ARCHITECTURE.md` 写下了若干可以用 `rg` 直接检查的规则，目前只靠人工遵守：

| 规则 | 检查方式 | 当前状态 |
| --- | --- | --- |
| 模块错误枚举用 `thiserror`（例外：`SharedError`、`UploadFailure`） | `rg "impl .*Display for"` 的结果只剩例外与非错误类型（`ResourceName`、`DeviceDriverVersion`） | 已满足，无检查 |
| `invalid_usage` 只记 `debug` 日志 | `src/error/mod.rs` 中的 `log::` 级别 | 已满足，无检查 |
| 不出现 `Result<_, String>` | `rg "Result<[^>]*, String>"` | 已满足，无检查 |

**动作**：把上表写成脚本（例如 `tools/check_architecture.sh`，该文件目前不存在），并接入 CI（IMP-39）。不要只改文档。

**验收**：脚本覆盖上表规则；违规时 CI 失败。

### IMP-33 [P2] `plan_private` 的内存序需要确认

**现状**：`src/tracking/tracked/mod.rs:219-230`：`enter_plan_private` / `leave_plan_private` 使用 `Relaxed` 存储，`is_plan_private` 使用 `Relaxed` 读取。写入方在 `src/rg/vulkan/queue_runtime.rs:269,286`（规划期），读取方在 `src/command/command_buffer/submission.rs:99`（提交期过滤）。

**判断**：若读写在同一线程或同一提交门内，Relaxed 充分；若规划与提交可并发，过期的 `false` 会导致使用已丢弃的 native state。

**动作**：确认调用时序；若跨线程，改为 Release/Acquire，并在注释中写明配对点。

**验收**：注释写明线程模型；必要时加跨线程测试。

### IMP-35 [P2] `mem::forget` 的使用约定未统一

**位置（`rg` 口径，共 9 处）**：`src/device/lifetime.rs:80,85`；`src/present/swapchain/frame_sync.rs:155,156`；`src/present/swapchain/progress.rs:226`；`src/present/swapchain/mod.rs:270-273`。

**判断**：策略（宁可泄漏不可 UAF）正确，但缺少统一表达。

**动作**：抽 `LeakOnUnprovableCompletion` helper 或统一注释；在 `ARCHITECTURE.md` 写不变式："`mem::forget` 只能用于无法证明 GPU 完成的对象"。拥有原生对象的类型在文档中标注 owns/borrows。

**验收**：所有 `mem::forget` 调用经同一 helper 或带统一注释；`rg "mem::forget"` 的每处都能对应到不变式。

### IMP-36 [P2] 公开面收窄与 `missing_docs`

**现状**：`src/lib.rs` 再导出 10 个公开模块（`bindless`、`command`、`device`、`interop`、`pipeline`、`present`、`resource`、`rg`、`sync`、`transfer` 等，以 `lib.rs` 为准）。`pub fn` 出现约 1438 次（`rg` 口径，未区分再导出路径）。`lib.rs` 未启用 `missing_docs`。

**动作**：(1) 将仅内部使用的 `pub` 项降为 `pub(crate)`（例：`rg/vulkan/backend/configure.rs` 的 `configure_*`，见 IMP-44）；(2) 启用 `#![warn(missing_docs)]`，先对公开项补一句文档；(3) 在 `ARCHITECTURE.md` 声明哪些公开结构是不可变快照、哪些持有不变量。

**验收**：`missing_docs` 无新增告警（或只剩白名单模块）；死接口已按 IMP-44 处理。

### IMP-41 [P2] [决策] 缺少模糊测试

**候选入口**：

- `PersistentPipelineCache::decode`（`src/pipeline/disk.rs:146`）：魔数、设备身份、长度与校验；fuzz 确认畸形输入只返回 `None`，不 panic、不 OOM。
- SPIR-V 反射（`src/pipeline/shader/reflection.rs`）：外部着色器字节。

**依赖问题**：`cargo-fuzz`/`libfuzzer-sys` 未在依赖中，需审批；可作为 dev-only 工具放在 `fuzz/` 目录，不进入 crate 依赖。

**验收**：fuzz 目标可在 CI 或手动运行，并有至少一个语料库。

### IMP-59 [P2] 错误分类统一（容量/所有权错误仍为 `Timeout`）

**位置与现状**：

| 位置 | 现状 | 应为 |
| --- | --- | --- |
| `src/transfer/queue/mod.rs:163,188` | 预算 `fetch_update` 失败 → `Timeout` | `Exhausted` |
| `src/transfer/queue/mod.rs:326,384,413,513,587` | 容量/队列上限（`max_cpu_bytes`、`max_keys`、`max_jobs`）→ `Timeout` | `Exhausted` |
| `src/transfer/route.rs:65`、`src/transfer/submit.rs:33`、`src/transfer/flush.rs:285` | 需逐处核实（**未核实**） | 按语义 |

**动作**：按 `error/mod.rs` 的分类表逐处修正；测试中固化的旧期望同步更新。**注意**：调用方的重试策略依赖 `Timeout`，需要一起评审。

**验收**：上表的容量/所有权错误不再返回 `Timeout`；`rg "Error::Timeout"` 的剩余位置都是真正的等待超时。

### IMP-64 [P2] 超线性热路径

| 位置 | 复杂度 | 场景 |
| --- | --- | --- |
| `src/rg/queue_schedule/mod.rs:174` `validate_resource_queue_contracts` | 单资源 O(A²) + 逐对 BFS | 计划编译 |
| `src/rg/sync_plan/bind/access.rs:368` `wait()` 的 `find` 去重（F-092） | O(W²) | 每帧绑定 |
| `src/rg/sync_plan/bind/full.rs:134` `uses(source).any(...)`（F-093） | O(U²) | 每帧绑定 |
| `src/rg/temporal/mod.rs:180` `owns_token` | O(n·m) | 每帧 |
| `src/rg/temporal/mod.rs:295` `requested()` 每次分配 `Vec` | 分配 | 每帧 |
| `src/rg/resource/mod.rs:148` `current_version` 全表扫描（F-170，含 `buffer_id` 路径） | O(访问总数²) | typed 构图 |
| `src/rg/resource_contracts/mod.rs:130` `covers_iter` 反复 `writes.clone()`（F-187） | O(R·W) | 计划编译 |

**动作**：每项先用计数器或基准确认热点（审计只给出静态复杂度，不等于测到热点，见 FILE_AUDIT §4）；再改为索引/预计算。

**验收**：配合 IMP-40 的基线，改动前后有可比较的数字。

### IMP-20 [P3] 日志语言与公共错误文案语言混用

**现状**：`src/log_ring`、`src/error/mod.rs` 的 `Error` `Display` 为中文；`rg` 统计日志调用 56 处，混合中英文。`Error::Display` 会进入崩溃报告，用户可见。

**动作**：定一条规则写入 `ARCHITECTURE.md` §3.1。建议"日志与 `Display` 用英文，注释与文档用中文"；若坚持中文，先确认崩溃报告消费端编码。分批改，先改 `Error` 的 `#[error]` 文案。

**验收**：规则写入文档；`Error` 文案按规则统一。

### IMP-21 [P3] `log` target 为过期的模块路径

**现状**：`src/present/swapchain/frame_sync.rs:154`、`src/present/swapchain/progress.rs:63,225` 使用 `target: "rhapsody::raw::swapchain"`，但该模块已位于 `present::swapchain`；`rg/vulkan/presentation` 亦有显式 target。使用模块过滤的下游会漏掉这些日志。

**动作**：统一为当前模块路径，或若需要稳定的对外 target，定义常量并写入文档。

**验收**：`rg 'target: "rhapsody::raw'` 无结果；文档说明 target 约定。

### IMP-22 [P3] 日志无限制，可被重试循环刷屏

**现状**：`invalid_usage` 每次构造都记 debug；根 `Cargo.toml:97` 声明了 `log-once = "=0.3.1"`，但 rhapsody 未使用。

**动作**：对"同一处反复失败"的路径加一次性或抽样日志；不建议全局改 `invalid_usage`，会丢失首次诊断。

**验收**：重复失败路径在 `RUST_LOG=debug` 下只输出有限条数。

### IMP-23 [P3] 访问器样板

**现状**：`pub fn raw_handle(&self)` 16 处，`pub fn device(&self) -> &Arc<Device>` 15 处（`rg` 口径）。

**动作**：抽 `DeviceChild` / `RawHandle` trait 统一。收益在一致性而非行数，排在 IMP-07、IMP-09 之后。

**验收**：trait 统一后公开签名不变或有文档说明。

### IMP-24 [P3] 错误枚举命名冲突

**现状**：`src/rg/executor/error.rs:7` 与 `src/command/descriptor_set/mod.rs:639` 均为 `pub enum Error`，与 crate 级 `Error`（`src/error/mod.rs:29`）同名，调用点需别名。

**动作**：改为 `ExecutorError` / `DescriptorSetError`，与 `BuildError`、`TemporalError` 的风格一致。

**验收**：`rg "pub enum Error"` 只剩 crate 级一处。

### IMP-25 [P3] 超过 800 行的非测试文件

**现状**（`wc -l`，不含测试文件）：`src/format/registry.rs` 1642 行（数据表，可接受但需确认）、`src/transfer/queue/mod.rs` 890 行、`src/rg/result_cache/mod.rs` 818 行。

**动作**：仅对职责混杂的文件拆分（如 `transfer/queue/mod.rs` 的任务调度与预算），不为拆而拆。

**验收**：列出的文件逐个给出"保留理由"或已拆分。

### IMP-26 [P3] `&Arc<T>` 形参 178 处

**动作**：不需要克隆的应传 `&T`；需要 `Arc::ptr_eq` 的必须保留。**逐个审计，不做机械替换**。

**验收**：审计清单完成；无不必要的 `&Arc<T>` 形参。

### IMP-27 [P3] 窄化 `as` 转换

**现状**（`rg -o` 次数）：`as u32` 107、`as u8` 15、`as usize` 396、`try_from` 50。

**动作**：对用户可控长度的路径改用 `u32::try_from`；图形计数有 Vulkan 上限兜底，风险不高。`clippy::cast_possible_truncation` 未启用（见 IMP-19）。

**验收**：用户可控长度的路径均使用 checked 转换。

### IMP-37 [P3] 布尔盲参

**现状**：`: bool` 形参 307 处（`rg` 行数，含字段，未逐个筛选）。

**动作**：超过 2 个布尔参数或调用点出现字面量 `true/false` 的，改为具名 enum（如 `InitialContents::{Defined, Undefined}`）。优先级低，随相关改动顺手做。

**验收**：新增代码不再引入布尔盲参。

### IMP-42 [P3] 平台路径与 32 位假设

**现状**：`src/native/surface.rs:105` `create_win32` 在所有平台编译（`unsafe` 契约要求调用方只在 Windows 使用），这是有意的逃生口。`cfg(target_pointer_width)` 仅出现 1 次；`as usize` 396 处，当前假设 64 位宿主。

**动作**：写一份支持矩阵（哪些平台/扩展组合被实际验证过）；README 明确"仅支持 64 位"。

**验收**：支持矩阵与 README 一致。

### IMP-43 [P3] 超时语义分散

**现状**：timeout 相关 116 行（`rg` 口径）；没有统一的 `WaitPolicy`（`None` / `Bounded(Duration)` / `Infinite`）。

**动作**：抽 `WaitPolicy` 用于公共 API；`u64` 纳秒保留为内部表示。`buffer` 中已正确使用 `checked_add` 的模式应成为统一实现的样板。

**验收**：公共 API 的超时参数统一为 `WaitPolicy`。

### IMP-45 [P3] 仅测试使用的生产 API

**位置**：

- `src/pipeline/shader/raster.rs`：`RasterPipelineOpts` 的 `set_*` 系列（如 `set_topology` :161、`set_cull_mode` :178、`set_depth_compare_op` :191）。
- `src/accounting/budget/mod.rs`：`new_with_watermarks` :99、`evict_cache` :178、`is_under_pressure` :201。
- `src/device/caps/mod.rs:150` `FeatureSet::from_bits`；`src/retire/mod.rs:350` `drain_all`；`src/sync/timeline/mod.rs:158` `queue_lock_stats`；`src/command/crash/mod.rs:88` `srcname_path`；`src/command/cpu_arena/mod.rs:185` `slice_copy`。

**说明**：使用面未逐个核实（**未核实**是否全部仅测试使用）。

**动作**：仅测试使用的项加 `#[cfg(any(test, feature = "test-support"))]`；不要直接删，它们承载了位域解码的验证价值。

**验收**：列出的项在非测试构建中不可见，且测试通过。

### IMP-49 [P3] `BindlessStats` 扁平字段与聚合并存

**位置**：`src/bindless/stats.rs:17-24`：`peak_live`、`capacity`、`live`、`free` 扁平字段与 `tables: [BindlessTableStats; 4]` 并存。

**动作**：扁平字段改为由 `tables` 求和得出的方法；或保留字段但由构造器一次算出，并加测试断言两者一致。

**验收**：只有一个事实来源，或测试断言两者一致。

### IMP-50 [P3] `ViewKey` 手工镜像 `TextureViewDesc`

**位置**：`src/resource/managed/texture.rs:128` `struct ViewKey`，字段与 `TextureViewDesc` 一一对应（类型换成 `i32/u32`；字段数量未重算）。

**动作**：加测试断言"两个 desc 的 `ViewKey` 相等 ⇔ desc 相等"；或在 desc 上派生 `Hash/Eq` 并直接以归一化 desc 作 key（需确认 `vk::*` 类型实现了 `Hash`）。

**验收**：测试存在并通过。

### IMP-65 [P3] 瞬时别名与资源时序测试缺口

**位置**：`src/rg/transient_resources/tests.rs` 包含缓存与增长相关用例，但未覆盖 `lend` / `give_back` / `evict` / `collect_retiring` 的时序（`rg` 检索无对应测试名，见 FILE_AUDIT F-168）。

**动作**：补充时序单元测试：借出后归还、驱逐后的退役、OOM 时已有成员不丢失。

**验收**：四个时序路径各有至少一个测试。

## 3. 已复核、明确不改的项

以下是审查中判定为**刻意设计**的部分。后续重构不要把它们当重复代码合并，也不要按"统一风格"改写。

| 项 | 原因 |
| --- | --- |
| `allocator` / `suballoc`（`device/allocator/`、`device/suballoc/`） | 抽象层 + gpu-allocator 后端；分配后端边界是设计 |
| `descriptor_set` / `descriptor_pool_ring`（`command/descriptor_set/`） | 原生对象 vs 池增长策略 |
| `transfer.rs` / `transfer/queue/` | 门面 vs CPU 调度；`pipeline/` 的队列是另一套语义 |
| `rg/plan_check/*` | `ARCHITECTURE.md` 明确要求的**独立校验器**，禁止复用生产实现；`usage_flags` 调生产函数是有意收窄的例外 |
| `rg/vulkan/access_table.rs` 与 `native/access_table.rs` | 运行期可覆盖策略 vs 常量码表 |
| `collections/token_map` 的 `TokenHasher` | 有文档与测试，为 hashbrown 位分布定制 |
| `FeatureSet` 手写位集（`device/caps/mod.rs`） | `bitflags` 未钉死；实现只是 const 位运算 |
| `impl Clone for Managed<T>`（`resource/managed/handle.rs`） | derive 会强加 `T: Clone` 界 |
| `impl Default for ResourceSyncGroup`（`rg/access_ledger/mod.rs`） | 默认必须分配新 ID，derive 得到 0 |
| `AttachmentArray` 的手写 `PartialEq`/`Hash`（`rg/attachments/contract.rs`） | 只比较有效前缀，derive 会比较整个定长数组 |
| 绝大多数手写 `impl Default` | 非零默认、新身份、转发 `new()`；derive 不适用 |
| `plan_cache` 的 `try_get_matching` 全表扫描（`collections/cache/mod.rs`） | 无法构造 owned key，注释已说明权衡 |
| 生产代码的 `unwrap` 整体数量 | 数量较低，只需修正 IMP-12 列出的位置 |

## 4. 执行顺序

（同一批次内的任务可并行；批次之间有依赖。）

1. **批次 1 — 测试可信度与基线**：IMP-38、IMP-39、IMP-40、IMP-19、IMP-03。先让"测试通过"重新可信，再建立 CI 与性能基线。
2. **批次 2 — 文档、诊断与错误契约**：IMP-11、IMP-12、IMP-13、IMP-14、IMP-15、IMP-16、IMP-17、IMP-20、IMP-21、IMP-22、IMP-24、IMP-31、IMP-59。这些改动主要影响错误分类与文档，互不依赖。
3. **批次 3 — 去重与死代码**：IMP-44、IMP-36、IMP-45、IMP-09、IMP-10、IMP-07、IMP-35、IMP-33、IMP-28、IMP-49、IMP-50、IMP-23、IMP-37、IMP-43、IMP-26、IMP-27、IMP-25、IMP-42、IMP-65。
4. **批次 4 — 热路径与缓存**：IMP-04、IMP-05、IMP-64（依赖批次 1 的基线）。
5. **批次 5 — 决策项**：IMP-30、IMP-32、IMP-41，在项目所有者拍板后执行。

## 5. 决策项（需项目所有者拍板）

| ID | 决策内容 | 选项/影响 |
| --- | --- | --- |
| IMP-30 | `device` 与 `native` 的双向引用是否保留（A 记录现状 / B 合并 / C 反转依赖） | 见任务详情 |
| IMP-32 | 是否引入 `loom` 作为 dev-dependency 做并发协议模型检查 | 见任务详情 |
| IMP-39 | 是否建立 CI 闸门（fmt/clippy/test/MSRV/cargo-deny） | 见任务详情 |
| IMP-41 | 是否引入 fuzz（磁盘管线缓存解码、SPIR-V 反射） | 见任务详情 |
| IMP-44 | 42 个零引用定义：删除还是保留（崩溃/调试设施需先决定接入或删除） | 见任务详情分组表 |

## 6. 更新规则

- 新增任务：沿用 `IMP-<递增编号>`（不复用已删除编号），在 §1 表格与 §2 详情各加一条，并回填 `关联发现`。
- 完成任务：在 `REFACTOR_PROGRESS.md` 登记完成日期与验证方式；本文件中删除该条目（或在 §1 标为 `done` 后下次重写时移除）。
- 发现被证伪或已修复：在 [FILE_AUDIT.md](FILE_AUDIT.md) 中移除或改为不再关注；本文件对应任务改为 `dropped` 并删除详情，不保留历史叙述。
