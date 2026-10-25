---
doc: rhapsody-improvement-tasks
schema: tasks-v1
date: 2026-10-24
findings_doc: FILE_AUDIT.md
tasks_total: 66
by_priority: {P0: 6, P1: 16, P2: 29, P3: 15}
status_values: [pending, in_progress, done, dropped]
status_note: 本文件只登记任务定义；完成状态按项目规则写入 REFACTOR_PROGRESS.md
---

# rhapsody 改善任务台账（机器可读版）

## 0. 给 AI 读者的说明

- 本文是**任务台账**：每条任务回答"改什么、为什么、怎么验收"。问题证据与逐文件结论在 [FILE_AUDIT.md](FILE_AUDIT.md)，用 `F-xxx` 编号。
- 字段（§1 表格）：`ID | 优先级 | 类别 | 状态 | 批次 | 位置 | 关联发现 | 任务`。
- 优先级：`P0`（正确性/安全，立即处理）> `P1`（明确缺陷或高收益重构）> `P2`（规范/性能/测试）> `P3`（可选优化）。
- 状态：`pending`（本文件登记时的统一状态）| `in_progress` | `done` | `dropped`。实际完成情况以 `REFACTOR_PROGRESS.md` 为准。
- `关联发现` 为自动匹配的 FILE_AUDIT ID（同文件、行号 ±80），用于交叉检索；未匹配到显示 `—`。
- 方括号内标记 `[决策]` 的任务需要项目所有者先拍板（见 §5），不要自行实施。
- 执行顺序见 §4；不要合并/重构的"刻意设计"清单见 §3。

统计：**66 条任务**（P0 6 / P1 16 / P2 29 / P3 15），其中决策项 6 条。

## 1. 任务索引

| ID | 优先级 | 类别 | 状态 | 批次 | 位置 | 关联发现 | 任务 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IMP-01 | P0 | observability | pending | 第一批 | `../tiny-glade/src/main.rs:168` | — | 接通或删除 `log_ring` |
| IMP-02 | P0 | perf | pending | 第一批 | `src/rg/cache.rs:150` | F-020 F-197 F-198 | `BoundedCache` 命中路径 O(n) |
| IMP-03 | P0 | safety | pending | 第一批 | `—` | — | unsafe 缺 `// SAFETY:` 注释 |
| IMP-38 | P0 | test | pending | 第二批 | `—` | — | 34 处 SKIP 让 GPU 测试静默通过 |
| IMP-51 | P0 | correctness | pending | 第四批 | `src/raw/swapchain.rs:1444` | F-045 F-048 | 交换链租约在失败路径上被标记为已释放 |
| IMP-52 | P0 | correctness | pending | 第四批 | `src/transfer.rs:756` | F-112 F-114 F-115 | 纹理上传成功后丢失暂存槽完成点 |
| IMP-04 | P1 | perf | pending | 第一批 | `src/pipeline.rs:448` | F-110 F-109 F-111 | `pipeline.rs` 自带一套 LRU + 每次请求全表扫描 |
| IMP-05 | P1 | perf | pending | 第一批 | `src/rg/result_cache.rs:84` | F-195 F-197 F-198 | 缓存 key 每次全量哈希 |
| IMP-06 | P1 | duplication | pending | 第一批 | `src/rg/frame_storage.rs:111` | F-044 | 四个同构"有界 CPU 池"抽成泛型 |
| IMP-07 | P1 | duplication | pending | 第一批 | `src/raw/semaphore.rs:119` | F-224 F-223 F-077 F-076 | `GpuUse::visit_uses` 复制 5 份 |
| IMP-08 | P1 | error-handling | pending | 第一批 | `src/rg/pass_bindings.rs:55` | F-089 F-137 F-069 | 10 个模块错误枚举改用 `thiserror` |
| IMP-28 | P1 | architecture | pending | 第二批 | `src/resource/range_state.rs:3` | — | 范围状态机位于错误的层 |
| IMP-29 | P1 | architecture | pending | 第二批 | `src/raw/managed/texture.rs:10` | F-140 F-020 F-139 | `raw` 反向依赖 `rg` 的两个通用工具 |
| IMP-32 | P1 | concurrency | pending | 第二批 | `src/raw/dynamic_constants.rs:630` | — | （决策）四个并发协议没有模型检查 |
| IMP-36 | P1 | api | pending | 第二批 | `—` | — | 死公共 API 与过大的公开面 |
| IMP-39 | P1 | test | pending | 第二批 | `—` | — | （决策）仓库没有 CI |
| IMP-40 | P1 | perf | pending | 第二批 | `—` | — | 性能没有回归防线 |
| IMP-44 | P1 | dead-code | pending | 第三批 | `—` | — | 96 个零引用定义 |
| IMP-53 | P1 | memory-safety | pending | 第四批 | `src/raw/descriptor_set.rs:745` | F-135 F-137 F-136 | immutable sampler 越界读（潜伏） |
| IMP-54 | P1 | correctness | pending | 第四批 | `src/rg/result_cache.rs:85` | F-195 | 结果缓存键缺少图作用域 |
| IMP-55 | P2 | resource | pending | 第四批 | `src/rg/transient_resources.rs:602` | F-165 F-166 | 别名集同名成员互相覆盖 |
| IMP-56 | P1 | concurrency | pending | 第四批 | `src/raw/managed/texture.rs:39` | F-020 F-021 F-143 F-141 | 持锁跨 Vulkan 调用 / 阻塞等待 |
| IMP-57 | P1 | correctness | pending | 第四批 | `src/raw/access_table.rs:78` | F-145 F-146 | 访问码表内部不一致 |
| IMP-09 | P2 | duplication | pending | 第一批 | `src/rg/graph.rs:687` | F-221 F-006 F-039 F-219 | 小工具函数收敛 |
| IMP-10 | P2 | duplication | pending | 第一批 | `src/raw/buffer.rs:345` | — | `Buffer`/`Texture` 构造样板 |
| IMP-11 | P2 | api | pending | 第一批 | `—` | — | `#[must_use]` 全库 0 处 |
| IMP-12 | P2 | error-handling | pending | 第一批 | `src/rg/vulkan.rs:1801` | F-079 F-082 F-078 | 4 处无消息裸 `unwrap` |
| IMP-13 | P2 | observability | pending | 第一批 | `—` | — | 日志密度：`trace!` 为 0 |
| IMP-14 | P2 | observability | pending | 第一批 | `src/error.rs:151` | F-133 F-132 | `invalid_usage` 的日志级别与文档不符 |
| IMP-15 | P2 | api | pending | 第一批 | `src/raw/suballoc.rs:33` | F-024 F-025 | 库内读环境变量 + 全局可变默认 |
| IMP-16 | P2 | dead-code | pending | 第一批 | `src/lib.rs:30` | F-134 | `internal_mod!` 的宽泛 allow |
| IMP-17 | P2 | quality | pending | 第一批 | `src/raw/bytes.rs:10` | — | `raw/bytes.rs` 改用 `bytemuck` |
| IMP-18 | P2 | error-handling | pending | 第一批 | `src/error.rs:198` | F-132 | 扩展 `usage_error!` |
| IMP-19 | P2 | test | pending | 第一批 | `—` | — | 引入 pedantic 子集 |
| IMP-30 | P2 | architecture | pending | 第二批 | `—` | — | （决策）`device` ↔ `raw` 实际是同一层 |
| IMP-31 | P2 | doc | pending | 第二批 | `src/raw/init.rs:59` | F-132 F-133 | 文档规范与代码漂移（4 处已确认） |
| IMP-33 | P2 | concurrency | pending | 第二批 | `src/resource/tracked.rs:225` | F-125 F-141 F-128 F-126 | `plan_private` 的内存序需要确认 |
| IMP-34 | P2 | lifecycle | pending | 第二批 | `src/rg/frame_storage.rs:274` | F-203 F-019 | 两处 Drop 内的 `expect` |
| IMP-35 | P2 | lifecycle | pending | 第二批 | `src/device/lifetime.rs:44` | F-048 F-045 F-047 F-027 | `mem::forget` 与句柄所有权约定 |
| IMP-41 | P2 | test | pending | 第二批 | `src/pipeline.rs:861` | — | （决策）缺少模糊测试 |
| IMP-46 | P2 | duplication | pending | 第三批 | `src/rg/frame_imports.rs:40` | — | `PhysicalIdentity` 重复定义 |
| IMP-47 | P2 | duplication | pending | 第三批 | `src/raw/transient_backing.rs:14` | — | "原子计数 + 快照"双类型重复两套（适合宏） |
| IMP-48 | P2 | duplication | pending | 第三批 | `src/pipeline.rs:213` | F-108 | 同义类型合并 |
| IMP-58 | P2 | resource | pending | 第四批 | `src/raw/suballoc.rs:305` | F-004 | 子分配器 native object 少计 |
| IMP-59 | P2 | error-handling | pending | 第四批 | `src/resource/range_state.rs:292` | F-120 F-128 F-209 F-016 | 错误分类统一 |
| IMP-60 | P2 | concurrency | pending | 第四批 | `src/resource/tracked.rs:610` | F-127 | 提交使用锁的指针序校验 |
| IMP-61 | P2 | doc | pending | 第四批 | `src/bindless.rs:556` | F-099 | bindless 回收机制与文档不符 |
| IMP-62 | P2 | correctness | pending | 第四批 | `src/pipeline.rs:234` | F-108 F-109 F-111 | 管线取消与 staging 语义 |
| IMP-63 | P2 | validation | pending | 第四批 | `src/raw/texture.rs:719` | F-009 F-010 F-015 F-039 | 校验缺口 |
| IMP-64 | P2 | perf | pending | 第四批 | `src/rg/queue_schedule.rs:421` | F-097 F-092 F-199 F-196 | 超线性热路径 |
| IMP-65 | P2 | test | pending | 第四批 | `src/rg/plan_check/bound.rs:196` | F-206 | 关键状态机测试缺口 |
| IMP-20 | P3 | observability | pending | 第一批 | `src/error.rs:29` | F-131 | 日志语言与公共错误文案语言混用 |
| IMP-21 | P3 | observability | pending | 第一批 | `src/raw/swapchain.rs:1067` | — | `log` target 不一致 |
| IMP-22 | P3 | observability | pending | 第一批 | `—` | — | 日志无限制，可被重试循环刷屏 |
| IMP-23 | P3 | api | pending | 第一批 | `—` | — | 访问器样板 |
| IMP-24 | P3 | error-handling | pending | 第一批 | `src/rg/executor.rs:123` | F-137 | 错误枚举命名冲突 |
| IMP-25 | P3 | quality | pending | 第一批 | `—` | — | 超大文件 |
| IMP-26 | P3 | quality | pending | 第一批 | `src/transfer.rs:426` | F-113 | `&Arc<T>` 形参 148 处 |
| IMP-27 | P3 | quality | pending | 第一批 | `src/rg/graph.rs:283` | — | 窄化 `as` 转换 |
| IMP-37 | P3 | api | pending | 第二批 | `—` | — | 布尔盲参 61 处 |
| IMP-42 | P3 | quality | pending | 第二批 | `src/raw/swapchain.rs:275` | — | 平台路径与 32 位假设 |
| IMP-43 | P3 | api | pending | 第二批 | `—` | — | 超时语义分散 |
| IMP-45 | P3 | api | pending | 第三批 | `src/raw/shader.rs:424` | F-044 F-043 F-041 F-072 | 仅测试使用的生产 API（23 项） |
| IMP-49 | P3 | duplication | pending | 第三批 | `src/bindless.rs:577` | F-099 | `BindlessStats` 的扁平字段 |
| IMP-50 | P3 | correctness | pending | 第三批 | `src/raw/managed/texture.rs:141` | F-021 | `ViewKey` 手工镜像 `TextureViewDesc` |
| IMP-66 | P3 | api | pending | 第四批 | `src/error.rs:59` | F-131 F-130 F-083 F-134 | 公开面泄漏 |

## 2. 任务详情

### IMP-01 [P0] 接通或删除 `log_ring`

**问题**：`log_ring` 是**死线**。`init_logging()` 在 [main.rs:168](../tiny-glade/src/main.rs:168) 被调用，panic 报告在 [panic_dump.rs:164](../tiny-glade/src/panic_reporter/panic_dump.rs:164) 用 `dump_log_vec()` 写 `render.log`，但**全 workspace 没有任何代码往 ring 写**：`tiny-glade` 的 fern 日志直连 stdout/文件（[logging.rs:131](../tiny-glade/src/logging.rs:131)），没有 chain 到 ring。结果：崩溃报告里的 `render.log` 恒为 0 字节。

**旁证**：[raw/crash.rs:2](src/raw/crash.rs:2) 刚因为"整条管线无调用点"删掉了同类的标记环，说明这是同一类未接线残留。

**复核命令**

```bash
grep -rn "log_ring::\|record_event\|record_named" crates --include=*.rs | grep -v "crates/rhapsody/src/log_ring.rs"
```

预期：除 `lib.rs` 的 re-export 外无结果。

**动作（二选一，需与调用方确认原版语义后再定）**

- A. 接线：实现 `log::Log` 适配器或给 fern dispatch 追加一个 sink，把 `record.level()` 与 `record.args()` 编码成 ring 记录；`record_event`/`record_named` 需要找到真实生产者（原版 GPU 事件），否则只接文本记录。
- B. 删除：如果原版 `render.log` 本就只承载 GPU 事件而事件源尚未恢复，则删除模块与 `lib.rs` 的 re-export、`main.rs` 的初始化、`panic_dump` 的写出，并在 REFACTOR_PROGRESS 记录删除原因。

**验收**：制造一次 panic（`tests/crash.rs` 或手动），`render.log` 非空且内容可读；或模块已完整删除且无悬空 re-export。

**风险**：`dump_log_vec` 是原版符号（`0x140be6b50`）的还原面，删除会缩小 API 面，需要按 ARCHITECTURE "唯一标准：正确性与高性能" 判断是否仍有必要保留。

### IMP-02 [P0] `BoundedCache` 命中路径 O(n)

**问题**（[src/rg/cache.rs](src/rg/cache.rs)）：

| 位置 | 缺陷 |
| --- | --- |
| [:150](src/rg/cache.rs:150)、[:161](src/rg/cache.rs:161) | `contains_key` + `get` → 每次查找两次哈希 |
| [:315](src/rg/cache.rs:315) | `touch` 用 `iter().position()` + `VecDeque::remove(index)` → 每次命中 O(n)，且对完整 key 做 `Eq` |
| [:260](src/rg/cache.rs:260)、[:307](src/rg/cache.rs:307) | `remove`/`replace` 用 `order.retain()` → O(n) |
| [:309](src/rg/cache.rs:309) | 插入时 `key.clone()`，同一 key 在 map 与 deque 各存一份 |

**为什么是热路径**：

- `ResultCache` 默认 128 条，每个输出每帧查一次（[result_cache.rs:653](src/rg/result_cache.rs:653)）。
- `ViewCache` 上限 256，每次取纹理视图（[managed/texture.rs:40](src/raw/managed/texture.rs:40)）。
- `SubgraphPlanCache`、`plan_cache` 复用同一类型。

**动作**：改成侵入式 O(1) LRU——`Entry { value, bytes, prev: u32, next: u32 }`，`HashMap<K, u32>` 指向 `Vec<Entry>` 槽位；命中只改 4 个 `u32`。或者用 `Vec<Entry>` + 自由链表保持 `iter_lru`/`lru_key_where` 的现有语义。**不要引入 `lru` crate**（未在 B lock 中）。

**验收**：

- 现有 `rg/cache.rs` 内联测试（15 项）全绿，语义不变（尤其 `try_get_matching` 的"失败不改变最近性"）。
- `tests/result_cache_vulkan.rs`、`tests/managed_view_cache.rs`、`tests/graph_cache_eviction.rs` 通过。
- `tests/p0_perf_baseline.rs` 增加或复用探针，release 下记录命中路径耗时（改动前/后各一次）。

### IMP-03 [P0] unsafe 缺 `// SAFETY:` 注释

**数据**（生产代码）：`unsafe` 出现 419 次，其中 `unsafe {` 行 326、`unsafe fn` 73；`// SAFETY:` 仅 **65**。

| 文件 | unsafe | SAFETY |
| --- | --- | --- |
| [raw/swapchain.rs](src/raw/swapchain.rs) | 58 | 0 |
| [raw/command_buffer.rs](src/raw/command_buffer.rs) | 19 | 0 |
| [raw/physical_device.rs](src/raw/physical_device.rs) | 14 | 0 |
| [device/logical.rs](src/device/logical.rs) | 14 | 0 |
| [raw/descriptor_set.rs](src/raw/descriptor_set.rs) | 11 | 0 |
| [raw/swapchain/fullscreen.rs](src/raw/swapchain/fullscreen.rs) | 9 | 0 |
| [raw/buffer.rs](src/raw/buffer.rs) | 33 | 11 |
| [raw/aftermath.rs](src/raw/aftermath.rs) | 28 | 11 |

**动作**：启用 `clippy::undocumented_unsafe_blocks`（见 IMP-19），按文件推进；优先 `swapchain.rs`、`command_buffer.rs`、`physical_device.rs`。已经正确的部分：`unsafe_op_in_unsafe_fn = "deny"`、无手写 `unsafe impl Send/Sync`。

**验收**：上述 6 个文件的 unsafe 块全部有 `// SAFETY:` 或 `#[allow(clippy::undocumented_unsafe_blocks = "reason")]` 并附理由。

### IMP-38 [P0] 34 处 SKIP 让 GPU 测试静默通过

**现状**：

```rust
eprintln!("SKIP: no Vulkan device");
return; // 测试仍然 PASS
```

共 34 处（`src/` 内联测试 + `tests/`）。在没有 Vulkan 的机器上（CI 容器、其他开发者机器），这些测试**全绿但什么都没测**。

**动作（三选一，推荐 A）**：

- A. 改成 `#[ignore = "requires Vulkan"]`，让"跳过"在测试报告里可见，并用一个显式环境变量（如 `RHAPSODY_REQUIRE_VULKAN=1`）把忽略变成失败。
- B. 保留 SKIP，但测试末尾对全局计数器断言"本次至少执行了 1 个 GPU 用例"，否则 panic。
- C. 在 CI 里用 lavapipe 跑，本地允许 SKIP。

**验收**：无设备环境下，测试报告能区分"通过"与"跳过"；CI 中 GPU 路径无法静默跳过。

### IMP-51 [P0] 交换链租约在失败路径上被标记为已释放

- 位置：[raw/swapchain.rs:1444-1456](src/raw/swapchain.rs:1444)（`SwapchainImageLease::present`），早退点 `:1344`、`:1345`、`:1358`。
- 问题：`present_unnoted` 返回 `Err` 后仍无条件 `self.released = true`；这些早退路径都不 `mark_stale`。Drop 因此不再处理该图像，图像停留在 acquired，代际既不 stale 也不重建，之后每次 present 该索引都失败。
- 动作：只有 `Ok` 或"确实已退役代际"时才置 `released`；失败路径应 `mark_stale()` 或把 `released` 留给 Drop 统一决定。
- 验收：构造 `present_unnoted` 早退（如 `prepare` 超时）后，`is_stale()` 为真且下一帧能重建；补一个注入失败的测试。

### IMP-52 [P0] 纹理上传成功后丢失暂存槽完成点

- 位置：[transfer.rs:756-769](src/transfer.rs:756) 的调用点与 [transfer.rs:1248-1262](src/transfer.rs:1248) 的 `record_transfer_textures`。
- 问题：`submit_to` 成功后逐纹理 `record::queue_acquires(...)?`；任一失败经 `?` 上抛，调用方不再写入 `slots.uploads[index].last_use`。暂存槽立即被 `free_upload` 视为空闲，GPU 仍在读取时可能被下一次上传覆盖。
- 动作：把 `last_use` 的写入移到 `queue_acquires` 之前（提交成功即有完成点），或让 `queue_acquires` 失败不影响槽位记账（在提交前预保留全部容量，保证提交后不再失败）。
- 验收：注入 `queue_acquires` 失败后，`last_use` 仍非空；补测试。

### IMP-04 [P1] `pipeline.rs` 自带一套 LRU + 每次请求全表扫描

**问题**：

- [pipeline.rs:448](src/pipeline.rs:448)：`state.cached.iter().find(|(key,_)| job.matches_key(key))` —— 每次管线请求对全部缓存条目做语义比较（着色器字节、布局、状态），O(n·m)。
- [pipeline.rs:456](src/pipeline.rs:456)、[:486](src/pipeline.rs:486)、[:583](src/pipeline.rs:583)、[:646](src/pipeline.rs:646)、[:677](src/pipeline.rs:677)：第二套 `VecDeque` LRU，`position` + `retain`。

**动作**：

1. 让请求 key 可 `Hash + Eq`，命中路径改为哈希查找；`matches_key` 只用于确认或作为 fallback。
2. LRU 收敛到 IMP-02 修好后的同一实现（如果 pipeline 需要"只淘汰非 Pending 项"，给公共实现加一个 `evictable: FnMut(&V) -> bool` 谓词，BoundedCache 的 `lru_key_where` 已有类似能力）。

**验收**：`tests/pipeline_cache.rs`、`tests/async_pipeline.rs` 通过；同一请求重复提交时不再发生全表扫描（可用计数器断言）。

### IMP-05 [P1] 缓存 key 每次全量哈希

**问题**：`ResultKey = Arc<ResultKeyPayload>`（[result_cache.rs:84](src/rg/result_cache.rs:84)），`Hash` 派生到整个 payload（`Vec<DependencyVersion>`、`Vec<u8>` parameters、descriptor、sharing 等）。每次 `cache.get(&key)` 都重新哈希全部字段。

**自相矛盾点**：[token_map.rs:1](src/rg/token_map.rs:1) 的注释明确写"默认 SipHash 是可测量的每帧成本"，但这里对更重的 key 仍在用默认 SipHash。

**动作**：

- 在 key 构造时算一次 u64 指纹（`xxhash-rust =0.8.10` 已钉死，或 `std::hash::DefaultHasher`），存进 payload；`Hash` 实现只 `write_u64(fingerprint)`。
- `HashMap` 改用 `BuildHasherDefault<PassThrough>`（与 `TokenHasher` 同思路，或直接复用）。
- **必须保留**完整 key 的 `Eq`：命中判定不能只看指纹（现有注释与测试已强调这一点）。

**验收**：`tests/preserving_dedup.rs`、`tests/result_cache_vulkan.rs` 通过；人为构造指纹碰撞的测试仍能区分两个 key。

#### 附注（非任务）：`try_get_matching` 是文档化的设计

[try_get_matching](src/rg/cache.rs:176) 是全表扫描，但注释明确说明"不构造 owned key、命中不 clone 字符串"，且计划缓存只能用 `matches_declaration` 语义比较。**不改**；只在 IMP-02 改造时保持其语义。

---

### IMP-06 [P1] 四个同构"有界 CPU 池"抽成泛型

| 类型 | 位置 |
| --- | --- |
| `TempListPool` | [raw/temp_list_pool.rs](src/raw/temp_list_pool.rs) |
| `FrameStoragePool` | [rg/frame_storage.rs:111](src/rg/frame_storage.rs:111) |
| `ReceiptStorage` | [rg/receipt_storage.rs:55](src/rg/receipt_storage.rs:55) |
| `CpuArenaPool` | [command/cpu_arena.rs:124](src/command/cpu_arena.rs:124) |

**硬证据**：`trim_idle()` 在 [frame_storage.rs:161](src/rg/frame_storage.rs:161) 与 [cpu_arena.rs:207](src/command/cpu_arena.rs:207) **逐字节相同（13 行）**；`capacity_stalls`/`capacity_growths`/`max_cached_bytes`/`max_total_bytes` 词汇共 38 处。

**动作**：抽 `BoundedCpuPool<T>`（`checkout → Lease<T>`、`trim_idle(target)`、`stats()`、`config()`），四个调用点只保留 payload 的 `reserve/clear` 钩子。注意 `TempListPool` 的 lease 是 `!Sync` 的，泛型要保留这一点或由各 payload 自己约束。

**验收**：`tests/cpu_frame_allocations.rs`、`tests/managed_projection.rs`、`tests/budget_snapshot.rs`、`tests/frame_command_pools.rs` 通过；`trim_idle` 只剩一份实现。

### IMP-07 [P1] `GpuUse::visit_uses` 复制 5 份

**证据**（归一化后哈希相同）：

- 4 行 ×3：[semaphore.rs:119](src/raw/semaphore.rs:119)、[descriptor_set.rs:880](src/raw/descriptor_set.rs:880)、[sampler.rs:330](src/raw/sampler.rs:330)。
- 7 行 ×2：[buffer.rs:1144](src/raw/buffer.rs:1144)、[texture.rs:829](src/raw/texture.rs:829)。

**动作**：给 `GpuUse` 加必须实现的 `fn use_cells(&self) -> impl Iterator<Item = &Arc<SyncCell>>`，trait 提供默认 `visit_uses`。5 处实现缩减为 5 个 `use_cells`，并消除"新增资源类型忘记登记第二个 cell"的风险（buffer/texture 的 `allocation_use`）。

**验收**：`tests/retained_declaration.rs`、`tests/pass_bindings.rs`、`tests/resource_contracts.rs` 通过。

### IMP-08 [P1] 10 个模块错误枚举改用 `thiserror`

**违反** [ARCHITECTURE.md:112](ARCHITECTURE.md:112) §3.1 规则 5（"模块内部错误枚举用 `thiserror` 派生"）。全库 11 个错误枚举只有 crate `Error` 用了 thiserror，其余手写 `Display` + `Error`：

[pass_bindings.rs:55](src/rg/pass_bindings.rs:55)、[executor.rs:2270](src/rg/executor.rs:2270)、[graph.rs:229](src/rg/graph.rs:229)、[temporal.rs:681](src/rg/temporal.rs:681)、[descriptor_set.rs:646](src/raw/descriptor_set.rs:646)、[buffer.rs:128](src/raw/buffer.rs:128)、[reflection.rs:95](src/raw/shader/reflection.rs:95)、[recording.rs:91](src/command/recording.rs:91)、[transfer/queue.rs:116](src/transfer/queue.rs:116)、[aftermath.rs:305](src/raw/aftermath.rs:305)。

**动作**：逐个改 `#[derive(thiserror::Error)]` + `#[error("...")]`。注意：

- 需要 `PartialEq`/`Clone` 的枚举**可以同时 derive**，thiserror 不冲突。
- 带 `source` 的变体用 `#[source]` / `#[from]`，能顺带补上错误链（目前 `InvalidUsage.source` 只保留一层）。
- 文案语言按 IMP-20 的结论统一。

**验收**：`grep -rn "impl .*Display for" src` 只剩非错误类型（`ResourceName`、`DeviceDriverVersion`）；错误相关测试与 `tests/raw_fixes_graveyard.rs` 的断言不变。

### IMP-28 [P1] 范围状态机位于错误的层

**证据**（生产代码，`range_state::` 引用）：

| 引用方 | 次数 |
| --- | --- |
| [raw/command_buffer.rs](src/raw/command_buffer.rs) | 24 |
| [raw/resource_access.rs](src/raw/resource_access.rs) | 7 |
| [device/pending_acquire.rs](src/device/pending_acquire.rs) | 3 |
| [raw/access_table.rs](src/raw/access_table.rs) | 2 |
| `raw/{texture_upload,setup,binding,barrier}.rs`、`device/submission.rs` | 各 1 |
| **合计** | **40** |

被引用的类型：`AccessScope`(23)、`RangeStateTable`(9)、`NativeState`(9)、`StatePatch`(7)、`RangeOwner`(4)、`PatchAction`(4)、`RangeTransfer`(3)、`RangeCell`(2)、`Frontier`(2)，以及 `intersection`/`contains`/`validate_range` 等自由函数。

**为什么不能简单搬**：[range_state.rs:3-8](src/resource/range_state.rs:3) 依赖 `device::BudgetCategory` 与 `raw::allocator::{AllocatorStats, CpuMemoryReservation}`。状态机与"内存预算记账"耦合，所以它不是纯逻辑模块。

**动作**（分两步，可独立验收）：

1. 把 `AllocatorStats` / `BudgetCategory` / `CpuMemoryReservation` 从 `raw::allocator` 提到中性模块（建议 `src/budget.rs`），`raw` 与 `resource` 都向下依赖它。
2. 把 `range_state` 提到 `src/range.rs`（或 `state/`），只依赖 `budget`、`sync`、`ash`。

**验收**：`grep -rn "resource::range_state" src` 为 0；`raw` 对 `resource` 的引用降至 `OwnershipRange`/`ResourceSharing` 这类纯词汇类型（或也为 0）。

### IMP-29 [P1] `raw` 反向依赖 `rg` 的两个通用工具

**证据**（全部 `raw → rg` 的边只有这几处）：

- [raw/managed/texture.rs:10](src/raw/managed/texture.rs:10)：`rg::cache::{BoundedCache, Inserted}`
- [raw/command_buffer.rs:313](src/raw/command_buffer.rs:313)、[:437](src/raw/command_buffer.rs:437)、[:438](src/raw/command_buffer.rs:438)：`rg::token_map::TokenMap`

这两个类型本身与渲染图无关，只是放错了目录。

**动作**：新建 `src/util/`（或 `src/collections.rs`），移入 `BoundedCache` 与 `TokenMap`；`rg` 内部改为 `use crate::util::...`。这一步会让 `raw → rg` 引用归零，是 IMP-28 之外成本最低的分层修复。

**注意**：与 IMP-02 合并执行——`BoundedCache` 要改成 O(1) LRU，先移后改或先改后移都可以，但不要分两次改同一个文件。

**验收**：`grep -rn "crate::rg::\|rg::cache\|rg::token_map" src/raw` 为 0。

### IMP-32 [P1] （决策）四个并发协议没有模型检查

**可作为 loom 目标的协议**（都是手写发布/回收协议，最容易出"只在 ARM 上炸"的 bug）：

| 协议 | 位置 | 手写同步元素 |
| --- | --- | --- |
| 日志环发布 | [log_ring.rs](src/log_ring.rs) | 每槽 sequence CAS + `fence(Release/Acquire)` |
| 资源使用登记 | [resource/tracked.rs](src/resource/tracked.rs) | `values: [AtomicU64; N]` + submission gate |
| 退役队列 | [device/lifetime.rs](src/device/lifetime.rs)、[sync/retire.rs](src/sync/retire.rs) | 有序前缀 + try_reserve |
| 延迟 flush 队列 | [raw/dynamic_constants.rs:630](src/raw/dynamic_constants.rs:630) | `dirty: AtomicBool` + `swap(AcqRel)` 快路径 |

**依赖问题**：`loom` 未在 B lock 中，新增属于依赖变更，需要审批。可选折中：

- 短期：把上述协议的不变式写成文档 + 单测（已有部分），并在代码里标注"loom 候选"。
- 中期：申请 `loom` 作为 **dev-dependency**（不影响发布依赖身份），只在 `#[cfg(loom)]` 下编译。

**验收**：至少 `log_ring` 与 `HostFlushQueue` 有 loom 模型或等价的形式化说明。

### IMP-36 [P1] 死公共 API 与过大的公开面

**统计口径**：`lib.rs` 公开 10 个模块（`bindless/command/device/interop/pipeline/present/resource/rg/sync/transfer`），其余经 `internal_mod!` 只对集成测试可见。

| 指标 | 数值 |
| --- | --- |
| 公开模块内 `pub fn` | 583 |
| **其中全 workspace 零调用** | **34** |
| 公开模块内 `pub` 字段 | 684（全 crate 1141） |
| `pub` 项有文档注释（启发式） | 约 52% |
| `missing_docs` lint | 未启用 |

34 个疑似死接口（已排除 examples/tests）：

```
available_bytes                 collect_completed_commands     configure_cpu_arenas
configure_frame_storage         create_resource_handle         evict_cache
execute_graph_validated         export_texture_range           frame_storage_statistics
get_and_flip_parity             image_from_raw_handle          into_exports
invalidate_image                is_depth_stencil               is_under_pressure
named_buffer_with_options       named_buffer_with_sharing      new_with_watermarks
planned_batches                 populate_resources             prepared_dependencies
retired_tokens                  selected_queue                 set_batch_synchronization
set_lock_statistics             slice_copy                     take_resource
take_retire                     template_cpu_bytes             template_hits
temporal_texture_with_sharing   texture_ref                    write_texture_discard
clone_unchecked
```

**动作**：

1. 逐个人工判定：`pub(crate)` 还是保留为 API。多数（`configure_*`、`frame_storage_statistics`、`collect_completed_commands`）看起来是内部接线漏了 `pub(crate)`。
2. 启用 `missing_docs`（先 `warn`），给公开项补一句话文档。
3. 684 个公共字段集中在 `rg`（`graph.rs` 47、`executor.rs` 46、`sync_plan.rs` 44、`plan.rs` 35）——这些是"数据快照"型结构，公开字段可以接受，但应在 ARCHITECTURE 声明"哪些结构是不可变快照、哪些持有不变量"。

**验收**：死接口清零或降级；`missing_docs` 无新增告警。

### IMP-39 [P1] （决策）仓库没有 CI

**现状**：无 `.github/`、无 `justfile`/`Makefile`/`xtask`、无 `rust-toolchain.toml`、无 `deny.toml`。质量命令只写在 [OPTIMIZATIONS.md](OPTIMIZATIONS.md) §9 里，靠人执行。

**最小闸门建议**（按成本排序）：

1. `cargo fmt --check` + `cargo clippy -p rhapsody --lib -- -D warnings`
2. `cargo test -p rhapsody --all-features` + `--no-default-features --lib`（lavapipe 容器内）
3. MSRV 固定：加 `rust-toolchain.toml`（`1.88`），否则 `rust-version` 只是建议
4. `cargo deny check`（advisories/licenses/bans），并把 B-lock pin 政策写进 deny 配置

**验收**：任一 PR 违反闸门即失败；文档里的命令与 CI 一致。

### IMP-40 [P1] 性能没有回归防线

**现状**：

- `tests/core_perf.rs`、`tests/p0_perf_baseline.rs` 全部 `#[ignore]`。
- `p0_perf` 的断言只校验 manifest 字段与场景哈希，**不断言耗时**（[p0_perf_baseline.rs:824-836](tests/p0_perf_baseline.rs:824)）。
- `OPTIMIZATIONS.md` 的实测数字来自手动运行，无法在后续改动中复用。

**动作**：即使不做严格阈值，也应：

1. 让基线测试输出机器可读记录（已有 manifest，扩展为含耗时的 JSON）。
2. 提供 `tools/compare_perf.py`：对比两次运行，超过 N% 即失败（N 可配置，首次只报警）。
3. 与 IMP-02/04 的改造配对：改前跑一次存基线，改后对比。

### IMP-44 [P1] 96 个零引用定义

按目录分：`raw` 约 60、`rg` 约 30、`command/device/pipeline` 约 6。已抽样确认下列全部为真死（不是宏/反射）：

```
# command / device / pipeline / transfer
available_bytes                     command/cpu_arena.rs:286
pool_family                         command/recording.rs:49
defer_compute                       pipeline.rs:417
trim_upload_queue                   transfer.rs:415

# log_ring / rg
record_event                        log_ring.rs:179
record_named                        log_ring.rs:188      ← 与 IMP-01 合并处理
invalidate_image                    rg/attachments.rs:716
uniform_buffer                      rg/binding.rs:27
resource_name                       rg/executor.rs:2009
with_resources                      rg/executor.rs:2118
take_retire                         rg/executor.rs:2152
create_resource_handle              rg/graph.rs:298
populate_resources                  rg/graph.rs:651
set_plan_validation                 rg/mod.rs:92          ← README 文档化的运行时开关，需决策
write_texture_discard               rg/pass_builder.rs:362
texture_ref                         rg/pass_builder.rs:468
access_version                      rg/plan.rs:973
export_texture_range                rg/resource.rs:131
validate_read_access                rg/resource.rs:471
RuntimeResource                     rg/resource.rs:476    ← type alias
descriptor_from_buffer              rg/resource.rs:537
original_ref_view_binding           rg/resource.rs:632
original_ref_buffer_binding         rg/resource.rs:643
try_index                           rg/resource_info.rs:66
recycle_exports                     rg/resource_pool.rs:5  ← 整个文件的唯一内容
take_texture                        rg/resource_registry.rs:77
into_exports                        rg/resource_registry.rs:232
template_cpu_bytes                  rg/sync_plan/bind.rs:267
validate_handles                    rg/util.rs:6           ← 整个模块两个函数都死
checked_u16_index                   rg/util.rs:15
configure_cpu_arenas                rg/vulkan.rs:907
frame_storage_statistics            rg/vulkan.rs:930
configure_frame_storage             rg/vulkan.rs:933
selected_queue                      rg/vulkan.rs:1021
set_batch_synchronization           rg/vulkan.rs:1067
collect_completed_commands          rg/vulkan.rs:1134

# raw（历史 API / 诊断 / 格式表）
bind_descriptor_set                 raw/binding.rs:137
bind_persistent_descriptor_set_single  raw/binding.rs:164
CREATE_BUFFER                       raw/buffer.rs:89
BIND_BUFFER_MEMORY                  raw/buffer.rs:90
host_range_for_test                 raw/buffer.rs:668
bytes_as_slice                      raw/bytes.rs:19
set_stencil_reference               raw/command_buffer.rs:912
record_crash_marker                 raw/crash.rs:224
retain_debug_buffer                 raw/crash.rs:261
make_host_readable                  raw/crash.rs:265
log_vulkan_error_info               raw/crash.rs:333
log_error_info                      raw/crash.rs:350
begin_label                         raw/debug.rs:34
end_label                           raw/debug.rs:49
is_transient                        raw/descriptor_set.rs:510
with_create_info                    raw/descriptor_set.rs:735
with_compute_pipeline_layout        raw/descriptor_set.rs:776
persistent_pool_plan                raw/descriptor_set.rs:805
with_pool_create_info               raw/descriptor_set.rs:826
with_allocate_info                  raw/descriptor_set.rs:846
set_texture_debug_name              raw/device_api.rs:254
into_gpu_lifetime_tracker           raw/dynamic_constants.rs:52
into_uniform_buffer_binding         raw/dynamic_constants.rs:66
into_uniform_and_storage_buffer_binding  raw/dynamic_constants.rs:69
byte_at_09                          raw/format.rs:77
write_into                          raw/format.rs:80
try_vk_format_valid_aspect_flags    raw/format.rs:88
try_vk_format_block_bytes           raw/format.rs:113
GraveyardStats                      raw/graveyard.rs:56   ← type alias（RetirementStats 才是活的）
update_from_ron                     raw/init.rs:59        ← 同时违反 Result<_,String> 规则
refresh_memory_budgets              raw/init.rs:411
original_order_and_filter           raw/init.rs:514
legacy_disable_layer                raw/instance.rs:245
vulkan_debug_messenger              raw/instance.rs:429   ← 从未创建调试信使，与"无验证层"的已知缺口一致
new_buffer                          raw/managed/buffer.rs:12
is_amd                              raw/physical_device.rs:61
is_apple                            raw/physical_device.rs:64
BUDGET_SNAPSHOT_BYTES               raw/physical_device.rs:174
available_device_local_budget       raw/physical_device.rs:222
vendor_performance_info             raw/physical_device.rs:481
enumerate_vk_data                   raw/physical_device.rs:543
string_from_c_array                 raw/physical_device.rs:559
parse_driver_version                raw/physical_device.rs:571
gpu_profiler_output_buffer_desc     raw/profiler.rs:25
read_output_timestamps              raw/profiler.rs:36
CREATE_SAMPLER_EXPECT_MSG           raw/sampler.rs:320
SAMPLER_NOT_FOUND_PREFIX            raw/sampler.rs:322
shader_count                        raw/shader.rs:976
from_compiled_container             raw/shader.rs:1360
register_compiled_raster            raw/shader.rs:1881
UNABLE_TO_CREATE_GRAPHICS_PIPELINE  raw/shader/reflection.rs:70
UNWRAP_ON_ERR                       raw/shader/reflection.rs:74
failed_creating_shader_pipeline     raw/shader/reflection.rs:77
set_default_allocation_mode         raw/suballoc.rs:36    ← 与 IMP-15 一起处理
default_allocation_mode             raw/suballoc.rs:43
with_vk                             raw/submit.rs:209
submission_fence                    raw/swapchain.rs:1932
failure_policy                      raw/swapchain.rs:2087
mip_count_1d                        raw/texture.rs:20
CREATE_IMAGE                        raw/texture.rs:28
BIND_IMAGE_MEMORY                   raw/texture.rs:30
is_sampled_color                    raw/texture.rs:353
```

**分类与处置**：

| 类别 | 例子 | 建议 |
| --- | --- | --- |
| 历史/原版 API 残留 | `binding.rs`、`dynamic_constants::into_*`、`submit::with_vk`、`init::original_order_and_filter`、`shader::from_compiled_container` | 删除。ARCHITECTURE 已声明"原版地址/布局不再约束设计" |
| 诊断设施未接线 | `crash::log_*`、`debug::{begin,end}_label`、`profiler::*`、`create_failure` 常量、`format::write_into` | 删除或接入（`vulkan_debug_messenger` 与 IMP-01 同类：先决策） |
| 内部 helper 从未使用 | `rg/util.rs` 全部、`rg/resource_pool.rs` 全部、`resource_registry::take_texture`、`sync_plan::template_cpu_bytes` | 直接删除（两个文件可整体删） |
| 文档化的运行时开关 | `rg::set_plan_validation` | **保留**，但补一个测试或示例调用，避免下次又被判死 |
| **文档化但零调用的开关** | `raw/suballoc::{set,default}_allocation_mode` | `default_allocation_mode()` 全 workspace 零调用 ⇒ **OPTIMIZATIONS.md §8 的 `RHAPSODY_ALLOCATION=dedicated` 实际不生效**。与 IMP-15 一起修：并进 `DeviceConfig` 并在创建设备时真正读取 |
| 平台/厂商查询 | `is_amd`、`is_apple`、`vendor_performance_info`、`parse_driver_version` | 保留到 IMP-42 的支持矩阵落地时再用，或删除 |

**验收**：上述清单逐项 `git grep` 确认后删除；`cargo check --all-targets` 无新增告警；`rg/util.rs` 与 `rg/resource_pool.rs` 整文件删除并把 `internal_mod!` 列表同步去掉。

### IMP-53 [P1] immutable sampler 越界读（潜伏）

- 位置：[raw/descriptor_set.rs:745-763](src/raw/descriptor_set.rs:745)。
- 问题：每个 binding 只构造 1 个 sampler，`p_immutable_samplers` 指向单元素；`descriptor_count > 1` 时驱动按 count 读连续数组，最后一个 binding 处越界堆读。当前该函数零引用（IMP-44），属死接口里的 UB。
- 动作：删除 `with_create_info`（统一走 `new_with_samplers`），或在其中按 count 复制；同时加 `debug_assert!(descriptor_count == 1)`。
- 验收：`grep` 确认只有一份实现；死接口删除或修复。

### IMP-54 [P1] 结果缓存键缺少图作用域

- 位置：[rg/result_cache.rs:85-95](src/rg/result_cache.rs:85)。
- 问题：`ResultKeyPayload` 不含 `GraphId`，而缓存本体由 session 共享、`frames` 才按图分桶。两个图使用相同 `ResultNodeId`（宿主提供，`render_host.rs:4092`）且依赖/参数/描述符一致时会互相命中，复用错误的 GPU 结果。
- 动作：把 `graph_id` 纳入 key（一个 u64），或按图分区缓存。
- 验收：新增跨图同 node id 的测试，确认不再互相命中。

### IMP-55 [P2] 别名集同名成员互相覆盖

- 位置：[rg/transient_resources.rs:602](src/rg/transient_resources.rs:602)；`graph.rs:272-295` 不校验重名。
- 问题：`lend` 以 `ResourceName` 为键存成员，同名时后者覆盖前者：被覆盖的 `GpuResource` 直接丢弃、槽位永久 `None`，`set.lent` 回不到 0，整组 backing 不再复用也无法回收。
- 动作：在 `create_raw_resource` 增加重名拒绝（`declare()` 已在 `rg/vulkan.rs:1082-1084` 查重，但公开的 raw 入口没有）。
- 验收：直接调用 `create_raw_resource` 传入重名返回 `InvalidUsage`；补测试。
- 复核（2026-10-24）：常规 `declare` 路径已拦截，严重度从 P1 降为 P2（防御性缺口）。

### IMP-56 [P1] 持锁跨 Vulkan 调用 / 阻塞等待

| 位置 | 持锁期间做的事 |
| --- | --- |
| [raw/managed/texture.rs:39-53](src/raw/managed/texture.rs:39) | `views.entries` 锁内创建 `vk::ImageView`、申请预算、`reserve_slot` |
| [raw/managed/texture.rs:127-137](src/raw/managed/texture.rs:127) | 同一锁内 `device.poll()` + LRU 扫描 |
| [raw/descriptor_pool_ring.rs:118-173](src/raw/descriptor_pool_ring.rs:118) | `state` 锁内创建原生池、调用 `try_collect_garbage` |
| [raw/command_buffer.rs:678](src/raw/command_buffer.rs:678) | 逐 cell 准备全程持 `cpu_state_patches` 锁 |
| [raw/swapchain.rs:1358](src/raw/swapchain.rs:1358) | 持 `wsi` + `progress` 锁做最长 3s 栅栏等待 |
| [rg/sync_plan/bind.rs](src/rg/sync_plan/bind.rs) | 未持锁但每帧 O(W²) 去重（见 IMP-64） |

**动作**：统一成"锁内只做查表/占位，锁外做原生调用"，与 `graph_caches` 已有的 `try_lock` + 锁外释放模式一致。**验收**：每处给出锁范围注释，并用现有并发测试验证无回归。

### IMP-57 [P1] 访问码表内部不一致

- 位置：[raw/access_table.rs:78](src/raw/access_table.rs:78)（0x05）、[:412-438](src/raw/access_table.rs:412)（0x3d/0x3f）。
- 问题：0x05 vertex UBO 读用 `SHADER_READ`，同族其它 UBO 码用 `UNIFORM_READ` → `buffer_usage_for_access` 把 vertex UBO 算成 `STORAGE_BUFFER`；0x3d/0x3f 是写但 `is_write` 只覆盖 `0x28..=0x37`，`scope.write` 为 false，读写分类错误。
- 动作：0x05 改为 `UNIFORM_READ`；`is_write` 改为按表元数据判定（在 `AccessSpec` 里加 `write: bool` 或从 access 位推导），不再用码段范围。
- 验收：新增"每个码的 `is_write` 与 access 位一致"的表驱动测试。

### IMP-09 [P2] 小工具函数收敛

| 函数 | 副本 |
| --- | --- |
| `fn reserve<T>(&mut Vec<T>, len)` | [graph.rs:687](src/rg/graph.rs:687)、[frame_storage.rs:51](src/rg/frame_storage.rs:51)、[receipt_storage.rs:67](src/rg/receipt_storage.rs:67)（三者逐字节相同）；[submission.rs:154](src/device/submission.rs:154)、[command_buffer.rs:1406](src/raw/command_buffer.rs:1406)（SubmitArray 版，二者相同） |
| `fn bytes<T>(capacity)` | [frame_storage.rs:45](src/rg/frame_storage.rs:45)、[receipt_storage.rs:64](src/rg/receipt_storage.rs:64)、[submit.rs:93](src/raw/submit.rs:93) |
| 对齐 | [texture.rs:125](src/transfer/texture.rs:125) `/a*a` + 溢出检查；[dynamic_constants.rs:567](src/raw/dynamic_constants.rs:567) `&!(a-1)`（隐式要求 2 的幂）；[suballoc.rs:491](src/raw/suballoc.rs:491) `div_ceil*a`（**无溢出检查**） |

**动作**：

- `reserve`/`bytes` 收敛到 `raw/budget.rs`（或新建 `raw/small.rs`）。
- 对齐统一用 `u64::checked_next_multiple_of`（std 1.87，`rust-version = 1.88`）；`suballoc` 的 `round_up` 必须改用 checked 版本或就地断言。

**验收**：`grep -rn "fn reserve<T>\|fn bytes<T>" src` 只剩一处；`tests/suballocation_native.rs`、`tests/allocator.rs` 通过。

**注意**：不要把这些工具放进 `crates/utils`——`utils` 依赖 bevy_ecs/glam，rhapsody 是更底层的 GPU 库，反向依赖会倒置层次。

### IMP-10 [P2] `Buffer`/`Texture` 构造样板

**证据**（归一化标识符后哈希相同）：`Buffer::new` / `Texture::new_with_initial_data`（各 6 行）——[buffer.rs:345](src/raw/buffer.rs:345) vs [texture.rs:586](src/raw/texture.rs:586)；`new_concurrent`（各 9 行）——[buffer.rs:361](src/raw/buffer.rs:361) vs [texture.rs:598](src/raw/texture.rs:598)。差异只有 desc 类型与 `new_with_memory`/`new_with_sharing` 的被调函数名。

**动作**：抽 `fn concurrent_sharing(families: &[u32]) -> Result<ResourceSharing>`；`new_concurrent` 两处保留一行调用。

### IMP-11 [P2] `#[must_use]` 全库 0 处

**动作**：只给"返回裸值且丢弃即漏处理"的 API 加，不要遍地撒：

- `trim_idle` / `cancel_queued_before` / `cancel_key_before`（返回值是要处理的释放/取消量）
- `retain_object`、`drain_evicted`、`evict_idle_caches`、`set_enabled`
- 各池的 `checkout`/`lease`（返回的 lease 丢了就是泄漏）

`Result` 已自带 must_use，不必重复。

### IMP-12 [P2] 4 处无消息裸 `unwrap`

[rg/vulkan.rs:1801](src/rg/vulkan.rs:1801)、[:2575](src/rg/vulkan.rs:2575)、[:2639](src/rg/vulkan.rs:2639)、[:3429](src/rg/vulkan.rs:3429)。

其中 1801 行尤其不一致：同一个 `commands.get_mut(&handle)` 查找，5 行后就用了 `expect("owned command")`。

**动作**：全部改成带不变量文案的 `expect`。全库生产 `expect/unwrap` 共 137 处，仅 1 处带相邻注释；建议在 ARCHITECTURE §3.1 规则 4 补一句"消息文案即理由，裸 unwrap 禁止"（而不是要求 137 处都加注释）。

### IMP-13 [P2] 日志密度：`trace!` 为 0

**现状**：生产代码 59,563 行，`log!` 共 55 处（`trace` 0、`debug` 9、`info` 5、`warn` 23、`error` 15 左右）。除日志外另有 2 条在 `Error` 构造器内。

**缺口**：资源创建/销毁、队列族选择结果、描述符池增长、缓存命中率、回收年龄都没有日志；`OPTIMIZATIONS.md` §5 声称"记录缓存失效原因、锁等待、内存高水位"，实际只有 `stats()` 可拉取。

**动作**：在确有诊断价值的位置补 `trace!`/`debug!`，不要用 `info!` 刷屏。建议首批：

- 设备创建成功后：物理设备名、API 版本、启用能力数、队列族布局（`raw/init.rs`、`raw/physical_device.rs`）。
- 计划缓存命中/未命中原因（`CacheReason` 已有枚举，直接记）。
- 淘汰/驱逐（`Evictable::evict_idle` 的释放量与目标）。

**验收**：上述路径在 `RUST_LOG=trace` 下有输出；默认 Info 级别不增加任何行。

### IMP-14 [P2] `invalid_usage` 的日志级别与文档不符

**现状**：[ARCHITECTURE.md:110](ARCHITECTURE.md:110) 规则 3 写 "`Error::invalid_usage(..)`（同时记 error 日志）"；实现 [error.rs:151](src/error.rs:151) 记的是 `debug!`。

**判断**：实现更合理——误用常在重试循环里发生，`error!` 会刷屏。**动作：改文档**，把规则 3 改为"记 debug 日志，限流由调用方负责"，并在 IMP-22 决定是否加限流。

### IMP-15 [P2] 库内读环境变量 + 全局可变默认

**现状**：[suballoc.rs:33-51](src/raw/suballoc.rs:33)：`static DEFAULT_MODE: AtomicU8` + `std::env::var("RHAPSODY_ALLOCATION")`，在设备创建时读取。

**问题**：库代码读环境变量破坏可嵌入性与测试隔离；全局默认模式无法在同一进程内按设备区分。

**动作**：把分配模式并入 `DeviceConfig`（[device/init.rs:38](src/device/init.rs:38)）；环境变量只在 CLI（`tiny-glade`）层解析后传入。保留 `set_default_allocation_mode` 作为过渡期兼容，标注 deprecated。

### IMP-16 [P2] `internal_mod!` 的宽泛 allow

**现状**：[lib.rs:30](src/lib.rs:30)（`internal_mod!` 宏，[lib.rs:24](src/lib.rs:24)）：非 `test-support` 构建下对 `raw` 与 `log_ring` 整个模块允许 `dead_code, unused_imports`。

**影响**：掩盖死代码。IMP-01 的 `log_ring` 无写入者正是这类问题；虽然 `pub` 项本来不报 dead_code，但跨模块未使用项同样静默。

**动作**：改为只 allow 确有需要的具体项（`#[allow(dead_code)]` 已在其他文件用过并附 reason，照此风格）。

### IMP-17 [P2] `raw/bytes.rs` 改用 `bytemuck`

**现状**：[raw/bytes.rs:10](src/raw/bytes.rs:10) `slice_as_bytes` / [:19](src/raw/bytes.rs:19) `bytes_as_slice` 是手写泛型重解释；`bytemuck =1.16.0` 已在 workspace 钉死且被 tg-text/country-core 使用，但 rhapsody 未依赖。

**动作**：内部改用 `bytemuck::bytes_of` / `bytemuck::pod_read_unaligned` 或 `from_bytes`。注意 `country-core/src/render/ssbo.rs:176` 把这个符号当原版 API 记录，**保留对外签名**、内部转调，不要直接删除。

**验收**：`cargo check -p rhapsody` 无新增 unsafe；`tests/host_buffer_ranges.rs`、`tests/raw_core_buffer_texture.rs` 通过。

### IMP-18 [P2] 扩展 `usage_error!`

[error.rs:198](src/error.rs:198) 已有 `usage_error!` 宏，只覆盖 `BufferUpdateError` 与 `recording::Failure`。其余 `From<X> for Error`（`BuildError`、`TemporalError`、`executor::Error`、`PassBindingError`、`InterfaceError`、`GroupPreparationError`、`descriptor_set::Error`）各自手写。差异在于需要保留 Vulkan 类别，建议把宏扩展成"透传式 + 默认映射"两种形式。

### IMP-19 [P2] 引入 pedantic 子集

当前 `clippy::all = "warn"`（根 [Cargo.toml:179](../../Cargo.toml)，`[workspace.lints.clippy]`），实测 `-p rhapsody --lib` 仅 12 条警告。建议按文件白名单逐步启用：

- `clippy::undocumented_unsafe_blocks`（配合 IMP-03）
- `clippy::must_use_candidate`（配合 IMP-11）
- 评估后再开 `cast_possible_truncation`、`ptr_arg`

**验收**：`cargo clippy -p rhapsody --lib -- -D warnings` 通过；新增 lint 有白名单文件清单与移除条件。

---

### IMP-30 [P2] （决策）`device` ↔ `raw` 实际是同一层

**证据**：`device → raw` 76 次、`raw → device` 68 次。`raw` 的对象需要 `Arc<Device>` 创建与记账；`Device` 又持有这些 raw 对象。

**三个选项，需要你拍板**：

| 选项 | 做法 | 代价 |
| --- | --- | --- |
| A. 保持现状 + 文档化 | 在 ARCHITECTURE 声明"device 与 raw 构成同一层，内部允许双向" | 低，但分层图要继续容忍例外 |
| B. 合并 | `raw/*` 并入 `device/*`，`Device` 直接持有 `ash::Device` 封装 | 中高，移动 30+ 文件，公开面不变 |
| C. 反转依赖 | `raw` 只接收 `&ash::Device` + `&AllocatorStats`，不持 `Arc<Device>` | 高，所有 raw 生命周期与设备归属要重做 |

建议先 A（记录），等 IMP-28/29 完成后重新评估 B。

### IMP-31 [P2] 文档规范与代码漂移（4 处已确认）

| 文档断言 | 位置 | 代码现状 |
| --- | --- | --- |
| 依赖方向"只允许向下" | [ARCHITECTURE.md:59](ARCHITECTURE.md:59) | 6 对模块互引（见 IMP-28/29/30），且图中没有 `raw` 这一层 |
| 模块错误枚举用 `thiserror` | [ARCHITECTURE.md:112](ARCHITECTURE.md:112) | 10 个手写（IMP-08） |
| `invalid_usage` 记 error 日志 | [ARCHITECTURE.md:110](ARCHITECTURE.md:110) | 实现为 `debug!`（IMP-14） |
| 不得出现 `Result<_, String>` | [ARCHITECTURE.md:108](ARCHITECTURE.md:108) | [raw/init.rs:59](src/raw/init.rs:59) `update_from_ron -> Result<(), String>`；另有 2 处 `format!("{result:?}")` 字符串化（[error.rs:182](src/error.rs:182)、[:189](src/error.rs:189)） |

**动作**：把可机械检查的规则写成脚本（`tools/check_architecture.sh`），纳入 CI（IMP-39）；其余改文档。不要只改文档不改检查——否则下次仍然漂移。

**验收**：脚本覆盖上述 4 条；`cargo test`/CI 失败于新增违规。

### IMP-33 [P2] `plan_private` 的内存序需要确认

**现状**：[resource/tracked.rs:225-237](src/resource/tracked.rs:225)：

```rust
enter_plan_private: plan_private.store(true, Relaxed);
leave_plan_private: plan_private.store(false, Relaxed);
is_plan_private:    plan_private.load(Relaxed)
```

写入方在 `rg/vulkan_queue.rs:274/291`（规划期），读取方在 [raw/command_buffer.rs:658](src/raw/command_buffer.rs:658)（收集期）。

**判断**：如果读取发生在同一线程或提交门内，Relaxed 充分；若并发收集可能观察到过期的 `false`，则会去使用一个已被丢弃的 native state。**动作**：确认调用时序；若跨线程，改为 Release/Acquire 并在注释写明配对点。

### IMP-34 [P2] 两处 Drop 内的 `expect`

| 位置 | 代码 |
| --- | --- |
| [rg/frame_storage.rs:274](src/rg/frame_storage.rs:274) | `let mut cached = self.cached.take().expect("active frame tables");` |
| [raw/temp_list_pool.rs:217](src/raw/temp_list_pool.rs:217) | `let mut list = self.list.take().expect("active CPU list lease");` |

不变量成立时不会触发，但 **Drop 中 panic 会在 unwind 期间 abort 整个进程**。改成 `let Some(..) = .. else { return };` 只是多两行，换来"永不 abort"。

**验收**：`grep -rn "expect(" src --include=*.rs` 在 `impl Drop` 块内为 0（另有 3 处是 `debug_assert`/测试，可保留）。

### IMP-35 [P2] `mem::forget` 与句柄所有权约定

**`mem::forget` 共 10 处**：

- [device/lifetime.rs:44](src/device/lifetime.rs:44)：无法确认 GPU 完成时泄漏对象（已注释）。
- [rg/vulkan.rs:3673](src/rg/vulkan.rs:3673)：关闭时保留在飞帧（已注释）。
- [raw/swapchain.rs:1410-1414](src/raw/swapchain.rs:1410)、[:1665](src/raw/swapchain.rs:1665)、[:1817-1818](src/raw/swapchain.rs:1817)：8 处，集中在"无法证明完成即不销毁"的路径。

策略本身正确（**宁可泄漏不可 UAF**），但缺少统一表达。**动作**：抽一个 `LeakOnUnprovableCompletion<T>`（或等价注释 + 单一 helper），把 10 处收敛成同一语义；在 ARCHITECTURE 写一条不变式："所有 `mem::forget` 只能用于无法证明 GPU 完成的对象"。

**另**：句柄所有权没有统一约定——有的类型在 Drop 销毁（`Surface`、`VulkanCommandPool`），有的是借用视图（`PresentTarget`、`Attachment`）。静态配对审计（`vkCreate*` vs `vkDestroy*`）因此无法自动化。建议在类型文档里标注 `owns` / `borrows`，或在 ARCHITECTURE 增加"拥有原生对象的类型必须实现 Drop 或显式 `into_raw`"规则。

### IMP-41 [P2] （决策）缺少模糊测试

**最值得 fuzz 的两个入口**：

- `PersistentPipelineCache::decode`（[pipeline.rs:861](src/pipeline.rs:861)）：魔数 + 设备身份 + 长度 + FNV 校验，已有边界检查，但 FNV 不是防篡改校验；fuzz 能确认所有畸形输入都返回 `None` 而不是 panic/OOM。
- SPIR-V 反射（`raw/shader/reflection.rs`、`rspirv`/`spirq` 输入）：外部着色器字节。

**依赖问题**：`cargo-fuzz`/`libfuzzer-sys`/`arbitrary` 未在 lock 中，需审批；可作为 dev-only 工具放在 `fuzz/` 目录，不进入 crate 依赖。

### IMP-46 [P2] `PhysicalIdentity` 重复定义

| 位置 | 定义 |
| --- | --- |
| [rg/frame_imports.rs:40](src/rg/frame_imports.rs:40) | `enum PhysicalIdentity { Buffer(usize), Image(usize) }` |
| [rg/pass_bindings/projection.rs:34](src/rg/pass_bindings/projection.rs:34) | 同名的同形定义，另加 `Ord/PartialOrd` |

两者语义一致（物理资源的种类 + 索引）。**动作**：合并到一处（建议随 IMP-29 移到中性 `util` 模块），另一处 `use` 复用。

**验收**：`grep -rn "enum PhysicalIdentity" src` 只有 1 处。

### IMP-47 [P2] "原子计数 + 快照"双类型重复两套（适合宏）

| 计数组 | 快照组 | 字段 |
| --- | --- | --- |
| `TransientBackingCounters`（`AtomicU64`）[transient_backing.rs:14](src/raw/transient_backing.rs:14) | `TransientBackingStatistics`（`u64`）[:22](src/raw/transient_backing.rs:22) | allocations / frees / allocated_bytes / freed_bytes（4/4 同名） |
| `RenderStats`（`AtomicU64`，pub）[raw/stats.rs:9](src/raw/stats.rs:9) | `StatsSnapshot`（`u64`，pub）[:16](src/raw/stats.rs:16) | counter0..2（3/3 同名） |

两组都是"原子计数字段 + `snapshot()` 逐字段 load"的手写样板，且第二组的字段是 `pub`，外部可绕过 snapshot。

**动作**：写一个 `atomic_snapshot! { Counters, Snapshot, { allocations: u64, ... } }` 宏生成两个结构体、`snapshot()` 和 `snapshot_and_reset()`。这与第一批 IMP-08/IMP-18 的"宏替代手写"是同一类收益，但风险更低（纯数据）。

**验收**：两组结构由同一宏展开；现有统计测试（`tests/budget_snapshot.rs`、`tests/transient_buffer_aliasing.rs`）通过。

### IMP-48 [P2] 同义类型合并

| 重复 | 位置 | 说明 |
| --- | --- | --- |
| `Status` ↔ `PipelinePoll` | [pipeline.rs:213](src/pipeline.rs:213) / [:226](src/pipeline.rs:226) | 变体与载荷完全相同（`Pending`/`Ready(Arc<VulkanPipeline>)`/`Failed(Arc<Error>)`）；`Status` 只是锁内副本 |
| `GraphTimelineSignal` ↔ `SubmitSignal` | [rg/executor.rs:162](src/rg/executor.rs:162) / [sync/timeline.rs:54](src/sync/timeline.rs:54) | 字段完全相同（`semaphore: vk::Semaphore, value: u64`），两个名字描述同一件事 |

**动作**：`Mutex<Status>` 改 `Mutex<PipelinePoll>`，删 `Status`；`GraphTimelineSignal` 改成 `pub use crate::sync::SubmitSignal` 的类型别名。`GraphTimelineWait` 保留（多一个 `stage` 字段，语义不同）。

### IMP-58 [P2] 子分配器 native object 少计

- 位置：[raw/suballoc.rs:305-316](src/raw/suballoc.rs:305)。
- 问题：`have_slot=false` 时 gpu-allocator 仍可能创建新块；失败回收走 `free_locked` 无条件 `release_native_object()`，而该次创建从未 `acquire` → 计数少 1，`maxMemoryAllocationCount` 护栏逐步失效。
- 动作：`(created=true, have_slot=false)` 路径在 `release_locked` 之前补偿（或让 `free_locked` 接受 `accounted: bool`）。
- 验收：注入"slot 耗尽 + 强制新块"后计数不漂移。

### IMP-59 [P2] 错误分类统一

| 位置 | 现状 | 应为 |
| --- | --- | --- |
| [range_state.rs:292,345,497,536,612,621](src/resource/range_state.rs:292) | 容量/预算/修订不匹配 → `Timeout` | `Exhausted` / `InvalidUsage` |
| [tracked.rs:201](src/resource/tracked.rs:201) | ownership 冲突 → `Unsupported` | 与 `range_state.rs:759` 统一为 `InvalidUsage` |
| [owner_storage.rs:62](src/rg/owner_storage.rs:62) | 内联槽满 → `Timeout` | `Exhausted` |
| [vulkan_queue.rs:176](src/rg/vulkan_queue.rs:176) | `Arc::get_mut` 失败 → `Timeout` | `InvalidUsage` |
| [temp_list.rs:38](src/raw/temp_list.rs:38) | 容量上限 → `Timeout` | `Exhausted` |
| [pipeline.rs:234](src/pipeline.rs:234) | 取消 → `Failed(Timeout)` | 独立 `Cancelled` 状态 |

**动作**：按 `error.rs` 的分类表逐处修正；测试里固化的旧期望同步更新。**注意**：改动会触及调用方的重试策略，需要一起评审。

### IMP-60 [P2] 提交使用锁的指针序校验

- 位置：[resource/tracked.rs:610](src/resource/tracked.rs:610)。
- 问题：`lock_submission_uses`/`_in` 不校验 cells 指针升序（只有 `collect_submission_uses:510` 校验），未排序调用会静默死锁。
- 动作：加 `debug_assert!(cells.windows(2).all(|w| w[0].as_ptr() < w[1].as_ptr()))`，并在 release 下提供排序版本。

### IMP-61 [P2] bindless 回收机制与文档不符

- 位置：[bindless.rs:556](src/bindless.rs:556)、`:404-451`、`:749-760`。
- 问题：生产路径唯一的 `retire` 调用固定传 `SyncSet::EMPTY`，retiring 队列与统计恒为空；文件头声称"GPU 完成后再回收"。
- 动作：二选一——修正文档并删除 retiring 机制（依赖命令持有 lease），或让生产路径传入真实完成点。删除前确认所有 bind 路径都会 `retain` lease。

### IMP-62 [P2] 管线取消与 staging 语义

- 位置：[pipeline.rs:234-240,592](src/pipeline.rs:234)、[:748-750](src/pipeline.rs:748)、[:658](src/pipeline.rs:658)。
- 问题：Pending 与 Cancelled 都映射为 `Timeout`；`HotPipeline::stage` 直接覆盖 `pending`，旧任务继续编译占配额；失败也 `compiled += 1`。
- 动作：引入 `PipelinePoll::Cancelled`；`stage` 前取消旧 ticket 或去重；失败计入 `failed` 而非 `compiled`。

### IMP-63 [P2] 校验缺口

- [raw/texture.rs:719-748](src/raw/texture.rs:719)：`image_view` 未校验 view_type/kind、`MUTABLE_FORMAT`、aspect、3D layer 限制。
- [raw/texture.rs:198-215](src/raw/texture.rs:198)：未限制 `mip_levels <= full_mip_chain_length`；D1/D2 静默忽略 height/depth。
- [raw/texture_upload.rs:430](src/raw/texture_upload.rs:430)：`array_elements == 0` 被静默当成 1。
- [device/submission.rs:184](src/device/submission.rs:184)：未拒绝同批 wait+signal 同一信号量、等待本队列自身信号量。
- [device/init.rs:104](src/device/init.rs:104)：`memory_budget_bytes == 0` 与 `host_memory_budget_bytes == 0` 语义相反。

> 复核（2026-10-24）：原列的"清零路径缺录制/设备检查"为**误报**——`resource_access::family()` 检查 `is_recording()`，`image_access_borrowed` 显式比较设备。已从本项移除。

### IMP-64 [P2] 超线性热路径

| 位置 | 复杂度 | 场景 |
| --- | --- | --- |
| [rg/queue_schedule.rs:421](src/rg/queue_schedule.rs:421) | 单资源 O(A²) + 逐对 BFS | 计划编译 |
| [rg/sync_plan/bind.rs:1068](src/rg/sync_plan/bind.rs:1068) | O(W²) | 每帧绑定 |
| [rg/attachments.rs:526-539](src/rg/attachments.rs:526) | O(n²) | 附件视图未命中 |
| [rg/result_cache.rs:261](src/rg/result_cache.rs:261) | O(retired×entries) | 每帧 |
| [rg/resource.rs:158](src/rg/resource.rs:158) | O(访问总数²) | typed 构图 |
| [rg/temporal.rs:295,180](src/rg/temporal.rs:295) | 每次分配 + O(n·m) | 每帧 |
| [rg/vulkan.rs:3470](src/rg/vulkan.rs:3470) | O(N×E) | 帧结束 |

### IMP-65 [P2] 关键状态机测试缺口

- [rg/executor.rs](src/rg/executor.rs)：`record_compiled`、并行分块、`PartialSubmission`/`abandon_unsubmitted` 无单元测试。
- [rg/transient_resources.rs](src/rg/transient_resources.rs)：`lend`/`give_back`/`evict`/`collect_retiring` 时序无测试。
- [rg/parallel.rs](src/rg/parallel.rs)：`balanced_chunks`/`plan_chunks` 边界无测试。
- [rg/plan_check/bound.rs:196](src/rg/plan_check/bound.rs:196)：未来 point 的 wait 漏检无测试。
- [rg/vulkan.rs](src/rg/vulkan.rs)：`collect_frame_imports`、`collect_named_retired` 的导入/退役路径无测试。
- 无测试文件的完整清单见 FILE_AUDIT 的覆盖缺口（23 个文件零覆盖）。

### IMP-20 [P3] 日志语言与公共错误文案语言混用

**数据**：55 条日志中 10 条中文；错误消息 157 中文 / 326 英文；公共 `Error` 的 `Display` 全中文（[error.rs:29-52](src/error.rs:29)）。

**影响**：`Error::Display` 会进入崩溃报告与 Sentry，用户可见；中英混排在不同日志消费端表现不一致。

**动作**：定一条规则并写进 ARCHITECTURE §3.1：建议"日志与 `Display` 用英文，注释与文档用中文"；若坚持中文则需确认崩溃报告消费端编码正确。改动量大，分批做，先改 `Error` 的 10 条 `#[error]`。

### IMP-21 [P3] `log` target 不一致

**现状**：只有 `raw/swapchain.rs` 与 `rg/vulkan_presentation.rs` 用显式 `target:`；其余依赖 `module_path!`。[swapchain.rs:1067](src/raw/swapchain.rs:1067) 的 `target: "rhapsody::raw::swapchain"` 与模块路径相同，纯冗余。

**动作**：删除冗余 target；若需要稳定的对外 target（便于下游按模块过滤），统一成显式常量并写进文档。

### IMP-22 [P3] 日志无限制，可被重试循环刷屏

**现状**：`Error::invalid_usage` 每次构造都记一条 debug。workspace 已钉死 `log-once = 0.3.1` 但未使用。

**动作**：给"同一处反复失败"的路径加一次性或抽样日志；不建议全局改 `invalid_usage`，会丢失首次诊断。

---

### IMP-23 [P3] 访问器样板

16 个 `pub fn raw_handle(&self)`、15 个 `pub fn device(&self) -> &Arc<Device>`。可抽 `DeviceChild` / `RawHandle` trait 统一，收益在一致性而非行数。**排在 IMP-06/07 之后**。

### IMP-24 [P3] 错误枚举命名冲突

`rg::executor::Error`（[executor.rs:123](src/rg/executor.rs:123)）与 `raw::descriptor_set::Error`（[descriptor_set.rs:640](src/raw/descriptor_set.rs:640)）与 crate 级 `Error` 同名，调用点需别名。建议改名为 `ExecutorError` / `DescriptorSetError`，与 `BuildError`、`TemporalError` 的既有风格一致。

---

### IMP-25 [P3] 超大文件

`rg/vulkan.rs` 4139 行、`raw/swapchain.rs` 3259、`raw/shader.rs` 2361、`rg/executor.rs` 2324。前三者同时是 unsafe 最集中的地方。建议结合 IMP-03 拆分（例如 `rg/vulkan/{session,frame,recording}.rs`），不要为拆而拆。

### IMP-26 [P3] `&Arc<T>` 形参 148 处

`clippy::ptr_arg` 不覆盖 `&Arc`。不需要克隆的应传 `&T`；[transfer.rs:426](src/transfer.rs:426) 这类需要 `Arc::ptr_eq` 的必须保留。**逐个审计，不做机械替换**。

### IMP-27 [P3] 窄化 `as` 转换

`as u32` 102 处、`as u8` 12 处，例如 `graph.passes.len() as u32`（[graph.rs:283](src/rg/graph.rs:283)）。图形计数通常有 Vulkan 上限兜底，风险不高；对**用户可控长度**的路径改用 `u32::try_from`。`clippy::cast_possible_truncation` 未开启。

---

### IMP-37 [P3] 布尔盲参 61 处

`fn new_with_memory(device, desc, false, ...)` 这类调用读不出语义。**动作**：超过 2 个布尔参数或调用点出现字面量 `false/true` 的，改成具名 enum（如 `InitialContents::{Defined, Undefined}`）。优先级低，随其他重构顺手做。

### IMP-42 [P3] 平台路径与 32 位假设

- 平台相关代码只集中在 `raw/dxgi.rs`（3 处 cfg）、`raw/aftermath.rs`（1 处）；`Surface::create_win32`（[raw/swapchain.rs:275](src/raw/swapchain.rs:275)）在**所有平台编译**，靠 `unsafe` 契约要求调用方只在 Windows 使用。这是有意的逃生口设计，但应有一份"支持矩阵"（哪些平台/扩展组合被实际验证过）。
- `as usize` 367 处 vs `try_from` 26 处：当前假设 64 位宿主。若要支持 32 位（或 wasm），需要先加 `cfg(target_pointer_width)` 审计。建议在 README 明确"仅支持 64 位"。

### IMP-43 [P3] 超时语义分散

`Timeout`/`timeout_ns` 相关出现 145 次，`HOST_ACCESS_TIMEOUT_NS` 是一个常量，另有 `10_000_000_000` 之类的字面量（多在测试）。没有统一的 `WaitPolicy`（`None`/`Bounded(Duration)`/`Infinite`）。

**动作**：抽 `WaitPolicy` 并在公共 API 使用；`u64` 纳秒保留为内部表示。注意 `buffer.rs:752` 已正确使用 `checked_add`，这个模式应成为统一实现的样板。

### IMP-45 [P3] 仅测试使用的生产 API（23 项）

| 位置 | 说明 |
| --- | --- |
| [raw/shader.rs:424-540](src/raw/shader.rs:424) 的 12 个 `set_*` | `RasterPipelineOpts` 的位域 setter。生产路径走 `apply_to()`，只用到 getter；setter 只被内联测试用来构造值 |
| [device/budget.rs:99/178/201](src/device/budget.rs:99) | `new_with_watermarks`、`evict_cache`、`is_under_pressure` |
| [device/caps.rs:150](src/device/caps.rs:150) | `FeatureSet::from_bits` |
| [sync/retire.rs:154](src/sync/retire.rs:154) | `drain_all` |
| [sync/timeline.rs:158](src/sync/timeline.rs:158) | `queue_lock_stats` |
| [raw/crash.rs:90](src/raw/crash.rs:90) | `srcname_path` |
| [command/cpu_arena.rs:241](src/command/cpu_arena.rs:241) | `slice_copy` |
| [log_ring.rs:179/188](src/log_ring.rs:179) | `record_event`/`record_named`（与 IMP-01 一起决定） |

**处置**：给测试专用项加 `#[cfg(any(test, feature = "test-support"))]`，让"生产不使用"在编译期可见；不要直接删——它们承载了位域解码的验证价值。

#### 结构合并说明

### IMP-49 [P3] `BindlessStats` 的扁平字段

[bindless.rs:577](src/bindless.rs:577)：`BindlessStats` 同时保存 `capacity/live/retiring/free/peak_*` **和** `tables: [BindlessTableStats; 4]`，前 7 个字段是后者的聚合。两处必须同步更新。

**动作**：聚合字段改为按需求和（`fn capacity(&self) -> usize { self.tables.iter().map(...).sum() }`），或保留扁平字段但由构造器一次性算出并加测试断言两者一致。

### IMP-50 [P3] `ViewKey` 手工镜像 `TextureViewDesc`

[raw/managed/texture.rs:141](src/raw/managed/texture.rs:141)：`ViewKey` 有 8 个字段，与 [TextureViewDesc](src/raw/texture.rs:413) 的 8 个同名字段一一对应（类型换成 `i32/u32`）。新增 desc 字段时如果忘记同步 key，会静默产生错误命中。

**动作**（便宜且有效）：加一个测试断言"任意两个 desc，`ViewKey::new` 相等 ⇔ desc 相等"；或在 desc 上加 `#[derive(Hash, Eq)]` 后直接用归一化 desc 作 key（需要确认 `vk::*` 类型都实现了 Hash）。

### IMP-66 [P3] 公开面泄漏

- [error.rs:59](src/error.rs:59)：`SharedError` 私有却出现在公开 `BuildError::BackendFailure`。
- [resource/handle.rs:41](src/resource/handle.rs:41)：`from_ffi(0)` 与首次 insert 的键相同；`is_null` 文档与 `as_ffi` 实际输出不符。
- [rg/graph.rs:816](src/rg/graph.rs:816)：`ResourceReuseInfo.reused` 恒为 false。
- [raw/error.rs](src/raw/error.rs)：整文件是 `pub use crate::error::*` 的兼容壳，建议合并。
- [lib.rs:49](src/lib.rs:49)：log ring 只导出 init/dump，不导出 write（与 IMP-01 同源）。

## 3. 已复核、明确不改的项

以下是审查中判定为**刻意设计**的部分。后续重构不要把它们当重复代码合并，也不要按"统一风格"改写。

**已核实的正面结论（不要重复审查）**：19 个函数同时持多把锁，静态锁序检查未发现矛盾；`HostFlushQueue` 的 Release/Acquire/AcqRel 与 `SyncCell::snapshot` 的 Acquire 使用正确；无手写 `unsafe impl Send/Sync`；生产代码 `unwrap`/`panic` 仅 21/6 处；17 个直接依赖全部在使用。

| 项 | 原因 |
| --- | --- |
| `allocator.rs` / `suballoc.rs` | 抽象层 + gpu-allocator 后端；`AllocationBackend` 边界是设计 |
| `descriptor_set.rs` / `descriptor_pool_ring.rs` | 原生对象 vs 池增长策略 |
| `transfer.rs` / `transfer/queue.rs` | 门面 vs CPU 调度；`pipeline.rs` 的队列是另一套语义 |
| `rg/plan_check/*` | ARCHITECTURE §3.2 明确要求的**独立校验器**，禁止复用生产实现；`usage_flags` 调生产函数是有意收窄的例外 |
| `rg/vulkan.rs` 的 `AccessTable` / `raw/access_table.rs` | 运行期可覆盖策略 vs 常量码表 |
| `token_map::TokenHasher` | 有文档 + 测试，混合方式为 hashbrown 位分布定制；不换成 `nohash-hasher` |
| `FeatureSet` 手写位集（[caps.rs:141](src/device/caps.rs:141)） | `bitflags` 未钉死；实现只是 const 位运算 |
| `impl Clone for Managed<T>`（[handle.rs:35](src/raw/managed/handle.rs:35)） | derive 会强加 `T: Clone` 界 |
| `impl Default for ResourceSyncGroup`（[access_ledger.rs:34](src/rg/access_ledger.rs:34)） | 默认必须分配新 ID，derive 得到 0 |
| `AttachmentArray` 的手写 `PartialEq`/`Hash`（[attachment_contract.rs:150](src/rg/attachment_contract.rs:150)） | 只比较有效前缀，derive 会比较整个定长数组 |
| 37 个手写 `impl Default` 中的绝大多数 | 非零默认、新身份、转发 `new()`；derive 不适用 |
| `plan_cache` 的 `try_get_matching` 全表扫描 | 无法构造 owned key，注释已说明权衡 |
| 生产 `unwrap`/`panic` 数量（21/6） | 相对 59.5k 行已很低，问题只在 IMP-12 的 4 处裸 unwrap |

---

## 4. 执行顺序

（已合并四批审查的先后关系；同一行内的 `+` 表示同批实施。）

1. IMP-38：SKIP → 显式忽略。先让"测试通过"重新可信。
2. IMP-39：CI 闸门（fmt / clippy / test / MSRV / cargo-deny）。
3. IMP-03 → IMP-01：unsafe SAFETY 文档 → `log_ring` 决策。
4. IMP-02 + IMP-29 → IMP-04 → IMP-05：BoundedCache 先移出 `rg`，再改 O(1)，随后 pipeline 与 key 指纹。
5. IMP-28 → IMP-36：`range_state` 分层 → 公共面收窄（同一批文件）。
6. IMP-06 + IMP-07 + IMP-08：四个 CPU 池、`GpuUse` 默认方法、thiserror 宏。
7. IMP-51 + IMP-52 + IMP-53：两个 P0 正确性缺陷 + sampler 越界。
8. IMP-54 / IMP-55 / IMP-56 / IMP-57 / IMP-58 / IMP-59：缓存键、别名集、持锁、访问码、记账、错误分类。
9. IMP-32 / IMP-40 / IMP-41：loom、性能阈值、fuzz（先定依赖与流程）。
10. 其余 P2/P3 随相关文件的改动顺路完成。

## 5. 决策项（需项目所有者拍板）

| ID | 决策内容 | 选项/影响 |
| --- | --- | --- |
| IMP-01 | `log_ring` 接线还是删除 | 见任务详情（选项 A 接线 / B 删除） |
| IMP-30 | `device` 与 `raw` 是否合并为一层（A 记录现状 / B 合并 / C 反转依赖） | 见任务详情 |
| IMP-32 | 是否引入 `loom` 作为 dev-dependency 做并发协议模型检查 | 见任务详情 |
| IMP-39 | 是否建立 CI 闸门（fmt/clippy/test/MSRV/cargo-deny） | 见任务详情 |
| IMP-41 | 是否引入 fuzz（磁盘管线缓存解码、SPIR-V 反射） | 见任务详情 |
| IMP-44 | 96 个零引用定义：历史 API / 平台查询是删除还是保留 | 见任务详情分组表 |

## 6. 更新规则

- 新增任务：沿用 `IMP-<递增编号>`，在 §1 表格与 §2 详情各加一条，并回填 `关联发现`。
- 完成任务：不改本文件的定义，只在 `REFACTOR_PROGRESS.md` 登记完成日期与验证方式；可在 §1 将该行状态改为 `done`。
- 发现被证伪：在 [FILE_AUDIT.md](FILE_AUDIT.md) §1 把状态改为 `refuted`，并在本文件对应任务上标 `dropped`，不要直接删除（保留审计链）。

