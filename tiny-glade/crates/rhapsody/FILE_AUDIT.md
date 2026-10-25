---
doc: rhapsody-file-audit
schema: findings-v1
date: 2026-10-24
scope: crates/rhapsody/src
files_total: 145
prod_loc: 63896
test_loc: 15251
passes:
  - pass-1: 逐文件走查（9 个并行批次 + 5 个人工补查文件）
  - pass-2: 213 个行引用逐条回读源码复核
counts:
  findings: 226
  critical: 1
  major: 70
  minor: 152
  refuted: 2
  info: 1
  suspected: 59
related_tasks: IMPROVEMENT_TASKS.md
---

# rhapsody 逐文件走查（机器可读版）

## 0. 给 AI 读者的说明

- 本文是**发现台账**，不是任务清单。可执行动作、验收标准与执行顺序在 [IMPROVEMENT_TASKS.md](IMPROVEMENT_TASKS.md)；任务侧用 `关联发现` 字段回指本文 ID。
- 每条发现一行，字段固定：`ID | 严重度 | 状态 | 类别 | 位置 | 发现`。
- 位置格式：`src/<path>.rs:<start>[-<end>]`，多条用 `;` 分隔；`group:<file>,...` 表示该条属于多文件分组的整体结论（分组内未逐文件区分）。
- 严重度：`critical` > `major` > `minor` > `refuted` > `info`。
  - critical：可导致 GPU 崩溃、内存不安全、数据竞争或永久泄漏
  - major：明确行为错误、记账漂移、校验缺口或明显性能问题
  - minor：不影响正确性的设计/规范/可观测性问题
  - refuted：复核后判定不成立，保留防止再次误报
  - info：记录性说明
- 状态：`confirmed`（机制已在源码核实）| `suspected`（机制成立，触发条件或影响需运行验证）| `refuted`（误报，pass-2 已修正）。
- 类别：`correctness` `memory-safety` `concurrency` `resource` `perf` `validation` `api` `error-handling` `observability` `test` `doc` `dead-code` `correction` `note`。
- §1 是**唯一权威索引**（226 行，每行一个 ID）；§2 按文件给出结论；§3/§5/§6 中的 ID 均为引用，可能重复出现。
- §1 按严重度、再按 ID 排序。ID 一经分配不再改变。

统计：**226 条发现**（critical 1 / major 70 / minor 152 / refuted 2 / info 1；suspected 59）。

## 2026-10-24 实施复核

原 226 条是审查基线，行号与 confirmed/suspected 不代表修复后的状态。逐行的“当前核实”说明优先；未带此说明的条目仍须继续处理，性能候选不等于已经测到热点。

新增运行发现 N-001：真实 synchronization2 多队列回归发现 HOST 被放入 semaphore wait stage。已修正 symbolic wait 与 StagedDependencies 的设备阶段语义；独立 bound checker 同时区分 host barrier 和 device wait。失败证据 `/tmp/tg-audit-gpu9.log`、`gpu10.log`、`gpu11.log`；修复后 `gpu12.log` 的 multi_queue_graph 5、native_ranges 5、submission_adapter 4 全部通过。未添加生产验证层。


新增运行发现 N-002：合法 Cube/CubeArray 创建时会补 CUBE_COMPATIBLE，而默认视图校验只看调用方原始 flags，导致合法视图被拒绝。已统一使用有效 image flags；真实 Vulkan 纹理上传与 temporal 组 6 项通过，保留最初失败及不合法维度夹具的修正日志。

新增运行发现 N-003：删除旧设备助手后，现代 Device::new 未建立 physical-device 元数据。已将元数据改为构造时必备 Arc，删除 Option/OnceLock 与公开 attach 接口；复用初始化属性和队列快照，预算发布直接使用该所有者。canonical-and-bindless43 的设备结构 3 项通过；全工作区和最终窗口验收另外登记。

## 1. 发现索引

| ID | 严重度 | 状态 | 类别 | 位置 | 发现 |
| --- | --- | --- | --- | --- | --- |
| F-135 | critical | confirmed | memory-safety | `src/raw/descriptor_set.rs:745-763` | `with_create_info` 为每个 binding 只构造 1 个 sampler，却把 `p_immutable_samplers` 指向 `&samplers[i]`；Vulkan 会按 `descriptor_count` 读取连续数组。`descriptor_count > 1` 时驱动读相邻句柄，最后一个 binding 处越界堆读。`new_with_samplers:142` 才是按 count 复制的正确版本。该函数当前零引用（见 IMP-44），属于死接口里的潜伏 UB。 **当前核实：** 已修：零消费者的 with_create_info 接口已删除，生产只用按 descriptor_count 持有 sampler 数组的现有构造器。 |
| F-004 | major | confirmed | resource | `src/raw/suballoc.rs:305-316` | `(created=true, have_slot=false)`：gpu-allocator 已创建新块，但 native slot 预留失败；随后 `release_locked` → `free_locked:388-390` 无条件 `release_native_object()`，而该次创建从未 `acquire` → `native_objects` 少计 1（`allocator.rs:1094-1100` 用 `checked_sub` 静默饱和）。后果：`maxMemoryAllocationCount` 护栏被逐步削弱，最终由驱动报错而非库内提前 `Exhausted`。 **当前核实：** 已修：NativeBlock 单独记录是否取得 native-object 配额；回收仅归还已取得的配额。 |
| F-005 | major | suspected | error-handling | `src/raw/suballoc.rs:405-409` | 非法 `memory_type` 直接 `return`，`Allocation` 未交回 allocator（索引实际恒有效，当前不可达）。 **当前核实：** 已核实不成立于当前调用链：memory_type 由私有分配 owner 创建并随租约保存，free 不接受外部重写该字段；不加逐次检查。 |
| F-006 | major | suspected | api | `src/raw/suballoc.rs:509-559` | `flags` 只用于 `DEVICE_ADDRESS` 能力校验；子分配路径不传 flags（回落 `allocate_dedicated` 时才传），`DEVICE_ADDRESS` 依赖 gpu-allocator 的全局设置。 **当前核实：** 已核实当前支持范围：生产 Buffer 只传 empty/DEVICE_ADDRESS，后者由 allocator 创建时统一启用；本接口 pub(crate) 不向外暴露任意 Vulkan allocation flags。不存在当前消费者请求其它 flags 后被静默忽略。 |
| F-020 | major | confirmed | concurrency | `src/raw/managed/texture.rs:39-53` | 持 `views.entries` 互斥锁期间调用 `Texture::image_view`（创建 Vulkan 对象、申请 host 预算）与 `reserve_slot`。 **当前核实：** 已修：纹理视图创建和缓存 backing 申请放在共享锁外，发布时合并竞争结果。 |
| F-021 | major | confirmed | concurrency | `src/raw/managed/texture.rs:127-137` | 持同一把锁调用 `device.poll()`（可能阻塞于队列/围栏）并做 LRU 扫描。 **当前核实：** 已修：GPU 完成快照在共享缓存锁外查询，缓存内只判断已有快照。 |
| F-045 | major | confirmed | correctness | `src/raw/swapchain.rs:1444-1456` | `SwapchainImageLease::present` 在 `present_unnoted` 返回后无条件 `self.released = true`；而 `present_unnoted` 在原生调用前有多条早退：`policy.allow_present():1344`、索引越界 `:1345`、`presentation.prepare(ACQUIRE_TIMEOUT_NS):1358`。这些路径都不 `mark_stale`。Drop 因 `released=true` 不再处理该图像 → 图像仍处 acquired，代际既不 stale 也不重建，之后每次 present 该索引都失败。与 `:1446` 注释"retires on every failed operation"不符。对照内部路径 `:2318-2330` 会 `mark_stale`。 **当前核实：** 已修：原生 present 前失败不把租约标为已消费，由原 Drop 协议令代际 stale。 |
| F-046 | major | suspected | concurrency | `src/raw/swapchain.rs:2299` | `serial = issued + 1` 在取 `wsi` 锁之前计算，`previous_present` 记录的是 acquire 时的 `issued`；若同一 image 被并发 present（无断言禁止），`enqueued`/`completed(serial)` 会误判。 **当前核实：** 已核实安全边界：unsafe present 要求同图像外部同步；安全 AcquiredImageLease 独占该次 acquisition，不能合法并发呈现同图像。 |
| F-047 | major | confirmed | concurrency | `src/raw/swapchain.rs:1358` | 持 `wsi` 与 `presentation.progress` 锁做最长 3s 的栅栏等待；`collect_presentations`（`:492`、`:505`）需要同一 `progress` 锁。 **当前核实：** 已修：等待前复制 pending fence，等待不持 progress/WSI 锁；完成后仅结算同一 serial。 |
| F-048 | major | confirmed | correctness | `src/raw/swapchain.rs:1436` | `image()/image_view()` 直接索引 `self.swapchain.images[index]`，不校验驱动返回索引（`acquire_frame:1275` 才校验），越界 panic。 **当前核实：** 已修：外部 acquire_next_image 只在接纳驱动返回索引时验证一次，失败通过租约退役。 |
| F-049 | major | confirmed | perf | `src/raw/swapchain.rs:1307-1326` | `can_release_images` 每次查询 properties2 并新建 maintenance1 loader；`release_image` 每次重建 loader，放弃帧路径（`:2021`）可反复触发。 |
| F-053 | major | confirmed | correctness | `src/raw/instance.rs:434-441` | `vulkan_debug_messenger` 先构造 `ash::ext::debug_utils::Instance::new(...)`，再检查 `VK_EXT_debug_utils` 是否启用。ash 0.38 的 `Instance::new` 对缺失入口点直接 panic，预期的 `Err(Unsupported)` 不可达。对照 `ValidationMessenger::new:511→518` 顺序正确。（该函数当前零引用，属潜伏缺陷。） **当前核实：** 已修：先核对 extension，再创建 ash loader。 |
| F-054 | major | confirmed | perf | `src/raw/instance.rs:150-167` | `build_with_allowlist` 校验后调 `build()` 又新建一次 `GraphicsInstanceLoader`，白名单 loader 被丢弃。 **当前核实：** 已核实当前结论不成立：build_with_allowlist 仅检查请求层名，随后 build 只创建一个 GraphicsInstanceLoader；当前无双 loader 构造。 |
| F-055 | major | confirmed | correctness | `src/raw/instance.rs:472-498` | `ValidationCollector` 消息无上限，长时间开校验层无界增长。 **当前核实：** 已修：显式开启诊断时消息保留最多 8192 条；未添加生产验证层。 |
| F-078 | major | confirmed | perf | `src/rg/vulkan.rs:2525-2536` | `collect_frame_imports` 的闭包对每个回收 token 调 `backend.clear_pass_resources()`（`projection.snapshot.take()` + `pool.clear_idle()`，见 `:768-772`）；而 `collect_named_retired:2727` 的注释明确说"按 token 失效会导致全量重建"。应改为 collect 之后统一清一次。 **当前核实：** 已修：一次 collect 结束只失效一次 pass-resource projection。 |
| F-079 | major | confirmed | error-handling | `src/rg/vulkan.rs:3454` | 非失败帧 `device.poll().unwrap_or_default()`：poll 出错时以空 `CompletedValues` 继续 `mark_cached`，缓存记账基于错误的完成集合。 **当前核实：** 已修：poll 错误保留并走失败回收，不以空完成集合发布 cache 状态。 |
| F-080 | major | suspected | concurrency | `src/rg/vulkan.rs:1194` | `bind_import_named_owned_by` 先 `resources.insert(token)` 再 `reserve_token(token)`；宿主显式 token 进入全局计数器前，另一线程 `try_allocate_token`（Relaxed `fetch_update`）可能返回同号。 **当前核实：** 已修：公开显式导入限定 host 高位编号域，内部自动编号保持低位域，temporal 通过私有已有 token 绑定；在接纳边界验证一次。真实导入/投影和 temporal 回归通过，失败夹具日志保留。 |
| F-081 | major | confirmed | validation | `src/rg/vulkan.rs:2354` | 重复导入同一物理资源时只比较所有权/descriptor/sharing，忽略 `initial_access`，第二次以不同边界状态导入会静默复用首个别名声明。  **当前核实：** 已修：同物理对象重复导入必须具有相同 initial_access；只在已有导入接纳边界比较，冲突返回 InvalidUsage。 |
| F-082 | major | confirmed | perf | `src/rg/vulkan.rs:3470` | 每个 backing 资源 `exported.iter().any(...)` 扫描全部导出并做 mapping 查找，O(N×E)。  **当前核实：** 已修：导出物理索引一次排序去重，资源回收使用二分查询。 |
| F-092 | major | confirmed | perf | `src/rg/sync_plan/bind.rs:1068` | `wait()` 用线性 `find` 去重，跨队列等待多时每帧 O(W²)，属每帧绑定热路径。 |
| F-093 | major | confirmed | perf | `src/rg/sync_plan/bind.rs:665` | `static_dependency` 内 `uses(source).any(...)` 对每个 target usage 重扫 source 全部 usage，且 `uses()` 每次重新 flatten，O(U²)。 |
| F-094 | major | confirmed | resource | `src/rg/sync_plan/bind.rs:1147` | `patch_slots`、`by_resource`、`key_scratch` 的增长不经 `known_cpu_bytes`/预算，规避 CPU 记账。  **当前核实：** 已修：patch_slots/by_resource/key_scratch 使用现有预算 Storage，SmallVec 索引溢出空间按几何容量预留并计入 known_cpu_bytes。 |
| F-095 | major | confirmed | api | `src/rg/sync_plan/bind.rs:559` | 公开 `bind` 依赖调用方先 `begin()`，无"已 begin"标志；漏调时把上一帧的 initial/shadow 状态继续追加，静默产生重复表项。  **当前核实：** 已修：bind/bind_cached 在既有绑定边界消费一次 ready 状态；下一帧必须 begin，避免追加上一帧的输出。初次 new 可直接绑定。 |
| F-096 | major | suspected | test | `src/rg/sync_plan/bind.rs:710` | `discard_alias_contents` 只合并直接 `alias_previous` 的写前沿，而 `transient_resources.rs:705-710` 生成的是环；正确性依赖"别名集成员每帧都参与绑定"。若某成员被 `vulkan_queue.rs:339-343` 的 `bound()` 过滤跳过，后继成员可能漏掉更早成员的写。n>2 的环无测试。  **当前核实：** 已核实绑定不变量：AliasCache::build_key 排除无 retained 使用者，lend 的全部成员由当前图分配；缺失 token 表示 population 失败，不能到成功绑定。补三成员环及 wrap-edge 的独立 bound-check 回归。 |
| F-097 | major | confirmed | perf | `src/rg/queue_schedule.rs:421` | `validate_resource_queue_contracts` 对每个资源扫描全部 retained pass/access，每对跨族重叠访问跑一次 `precedes` BFS（`:430-442`、`:465-479`），单资源 A 次访问最坏 O(A²)（叠加 O(A²·E)），是计划编译路径上唯一明显超线性的检查；可用一次拓扑序/传递闭包序号替代。 |
| F-098 | major | confirmed | perf | `src/rg/queue_schedule.rs:326` | 最终导出的跨队列等待由"任何访问导出资源的 pass"触发（含只读消费者），比必要更保守，会额外串行化 compute 读分支。 |
| F-099 | major | confirmed | resource | `src/bindless.rs:556` | 生产路径唯一的 `Slots::retire` 调用固定传 `SyncSet::EMPTY`（其余调用点全在 `#[cfg(test)]`），因此 `:404-451` 的 retiring 队列、`:757-760` 的 `poll+collect` 及全部 `retiring/peak_retiring` 统计在生产中恒为空。文件头 `:1-3` 与 `:749-752` 声称"GPU 完成后再回收"，实际完全依赖命令 `retain_*` 持有 Arc 到完成点。不是正确性 bug，但机制与文档/统计不一致。 **当前核实：** 已修：删除冗余 retiring 队列、容量预分配、模拟退役协议及不适用统计；最后一个命令 Arc 释放后直接归还槽位。canonical-and-bindless43 的 11 项真实 Vulkan heap/ABI 回归通过，含 pending shader 保活与完成后复用；无新增 GPU 等待。 |
| F-100 | major | confirmed | concurrency | `src/bindless.rs:767-800` | `resident_lease` 是 check-then-act：锁外 `make()` 期间另一线程可插入同 key，双双分配描述符，后插入覆盖先前 Weak。同页由 `page.group` 串行，但 `BindlessParameters::new/write` 是公开 API。 **当前核实：** 已修：锁外创建后、发布之前再次读取同 key 的活租约；竞争结果不覆盖已有 owner。 |
| F-101 | major | suspected | concurrency | `src/bindless.rs:917-923` | `update_descriptor_sets` 全程持有全局 `slots` 锁（buffer/image/sampler 三处同构），所有描述符写入被串行化；slot 已被新 lease 独占，驱动调用本身不需要该锁。 **当前核实：** 已修：对新租约独占槽的 descriptor update 不持 slots 全局锁；槽分配/回收仍由原锁保护。 |
| F-102 | major | confirmed | correctness | `src/bindless.rs:1316` | `Owner::Sampler` 分支不可达（`:1273-1276` 已提前 return）。 **当前核实：** 已核实：第二个 Owner::Sampler 匹配是 Rust 穷尽匹配的总分支；前序提前返回允许它静态不可达，不构成行为错误。 |
| F-103 | major | confirmed | correctness | `src/bindless.rs:224` | `Slots::exhausted` 只增不读（统计读的是 `table_exhausted`）。  **当前核实：** 已修：删除未读取的 Slots::exhausted，保留真实对外 table_exhausted 统计。 |
| F-104 | major | confirmed | perf | `src/bindless.rs:1240` | `build_coverage` 每个 access 线性 `semantics.iter().find`，首次构建 O(accesses×semantics)（结果按 pass 缓存）。 |
| F-108 | major | confirmed | error-handling | `src/pipeline.rs:234-240` | 取消以 `Status::Failed(Timeout)` 发布，`try_get` 把 Pending 与 Cancelled 都映射为 `Timeout`，调用方无法区分"还在编译"与"已被取消、永不会完成"。 **当前核实：** 已修：取消发布 terminal InvalidUsage，而 Pending 仍为 Timeout，消费者可区分重试与已终止。 |
| F-109 | major | confirmed | resource | `src/pipeline.rs:748-750` | `HotPipeline::stage` 直接覆盖 `pending`，不取消也不去重旧请求；连续 stage 会让旧任务继续编译并占满 `max_pending`/缓存配额，新请求在 `:468-476` 被拒。  **当前核实：** 已修：HotPipeline 同 completion 去重，被替换且无其它订阅者的 queued 请求取消并释放额度；共享请求和已开始编译的 owner 保留。 |
| F-110 | major | confirmed | perf | `src/pipeline.rs:448` | 除已知全表扫描外，`matches_key` 对每个 cache 条目做整段 SPIR-V 字节比较，逐条 O(payload)。 |
| F-111 | major | confirmed | observability | `src/pipeline.rs:658` | 失败也 `compiled += 1`，与字段语义不符。 **当前核实：** 已修：compiled 仅计成功创建，失败单列 failed；65 次无效 SPIR-V 重试的真实测试断言同步改为 compiled=0、failed=65。 |
| F-112 | major | confirmed | resource | `src/transfer.rs:756-769` | `record_transfer_textures` 在 `submit_to` 成功后逐纹理 `record::queue_acquires(...)?`；任一失败经 `?` 上抛，调用方（`:767-769`）的 `slots.uploads[index].last_use` 不再写入 → 暂存槽立即被 `free_upload` 视为空闲，GPU 仍在读取时可能被下一次上传覆盖。对照 buffer 路径 `:1084-1086` 的成功提交后写 `last_use`。触发条件是"提交成功 + 之后的主机侧错误"，概率低但后果是数据损坏。 **当前核实：** 已修：transfer submit 成功立即写回 point；后续 acquire 注册失败仍更新 staging last_use 后传播错误。 |
| F-113 | major | confirmed | api | `src/transfer.rs:472-474` | 预算压力下 `next()` 返回 None 直接 break，`capacity_deferred` 仍为 false（仅 staging 满的 Timeout 才置 true），调用方会误判为"无需重试"。 **当前核实：** 已修：预算使 next_batch 为空且队列仍有工作时返回 capacity_deferred=true。 |
| F-114 | major | suspected | resource | `src/transfer.rs:787-791` | `flush_acquires` 中 `RecordedAcquires::begin` 已 `command.begin()`，为空时直接返回而未 `end()`，依赖 CommandBuffer 的 Drop 复位。 **当前核实：** 已修：flush_acquires 空结果也 end 已开始的命令，再返回无提交。 |
| F-115 | major | suspected | concurrency | `src/transfer.rs:707` | staging 互斥锁跨命令录制与 `submit_to` 持有，期间 `evict_idle` 的 `try_lock` 只能返回 0，预算回收被阻塞。  **当前核实：** 已修：领取 staging slot 后以 UploadLease 独占，锁外打包/录制/提交；Drop 归还租约并发布已发生的 submit point，空闲其它 slot 可以回收。 |
| F-120 | major | confirmed | error-handling | `src/resource/range_state.rs:292` | 容量/预算上限、`expected_revision` 不匹配、pending release 一律返回 `Error::Timeout`；修订耗尽返回 `Unsupported`。这与 `error.rs:11-14` 的分类表冲突（`Exhausted` 专指容量耗尽、`InvalidUsage` 专指状态顺序错误），且测试已把当前选择固化。 **当前核实：** 已修：容量为 Exhausted，revision/ownership 顺序冲突为 InvalidUsage，耗尽 revision 不再复活为旧值。 |
| F-121 | major | suspected | concurrency | `src/resource/range_state.rs:182` | `source_scope` 只并当前队列的 Frontier（含 host），而 `dependencies` 可跨队列；`combined()` 无生产调用者，`transfer/record.rs:163` 只取目标队列作用域，跨队列 `srcStage` 可能缺失（若由 semaphore 内存依赖兜底则无害）。 **当前核实：** 已核实：跨队列内存依赖由 submission semaphore 的目标 stage 承担，本队列 barrier source 只包含本队列范围；不能将 foreign scope 混作本队列执行历史。 |
| F-122 | major | confirmed | perf | `src/resource/range_state.rs:257` | `Storage::new/grow` 用 `try_reserve_exact` 精确容量，每条新分裂边界都整表重分配 + 复制（一次 `prepare` 内可能多次），新旧两份同时记账。 |
| F-123 | major | suspected | correctness | `src/resource/range_state.rs:423` | revision 取 `max+1` 后又 `.min(u64::MAX-1)`，把饱和毒化的 `u64::MAX` 降回 `MAX-1`（需 2^64 次修订，实际不可达）。 **当前核实：** 已修：discard 保留饱和 revision，不再从 MAX 降至 MAX-1。 |
| F-124 | major | confirmed | test | `src/resource/range_state.rs:1553` | 文件末尾注释声称"完整 prepare 路径作为前缀重放 oracle"的测试，但模块内无该测试；`discard_idle_contents`、`prepare_host` 对 Released 的拒绝、`coalesce` 合并均无用例。  **当前核实：** 已补 Released 范围拒绝 host 写、discard 清除状态且保留饱和 revision 的回归；原有前缀重放证据使用 plan_check/native_state 的独立 oracle，不把末尾注释当已存在测试。 |
| F-125 | major | suspected | resource | `src/resource/tracked.rs:225` | `enter_plan_private` 只在"当时已有表"时丢弃；`with_range_state`/`prepare_range_state` 未检查该标志。`:42-44` 的注释承诺"plan-private 期间不保留原生状态"，实际只保证"进入时清空"。需要确认是否存在"plan-private 期间新建表"的路径（`command_buffer.rs:658` 已用 `is_plan_private` 过滤采集）。  **当前核实：** 已核实实际消费者：单队列 Create 且不导出的资源进入 private 分支后直接 planner.add_initial 并 continue，不调用 with_range_state；提交同样过滤 private。外部导入先 leave_plan_private。该 AtomicBool 不是独立的并发所有权协议。 |
| F-126 | major | suspected | correctness | `src/resource/tracked.rs:280` | `initial.writer = EMPTY; initial.writer.points = summary` 是覆盖而非并集，`initial` 中原有的 writer 前置点被丢弃。 **当前核实：** 已修：summary 与初始 writer 的实际点集合合并，保留已有前置点。 |
| F-127 | major | confirmed | concurrency | `src/resource/tracked.rs:610` | `lock_submission_uses`/`_in` 不校验 cells 按指针升序（只有 `collect_submission_uses:510` 校验）；模块文档称指针序用于防锁倒置，未排序调用会静默死锁。 **当前核实：** 已核实需继续封装：加锁入口已是 pub(crate)，生产使用归一化/合并后的指针序；不按原建议增加每次锁的验证层。 |
| F-128 | major | confirmed | error-handling | `src/resource/tracked.rs:201` | 同一 ownership 冲突，一处返回 `Unsupported`、一处返回 `InvalidUsage`。  **当前核实：** 已修：缺少匹配 queue-family transfer 的参数/顺序错误统一 InvalidUsage。 |
| F-129 | major | confirmed | doc | `src/resource/tracked.rs:337` | 注释说"Drop the native state"，实现只 `discard_idle_contents` 保留并重置表。  **当前核实：** 已修注释：discard_idle_range_state 保留并重置已安装状态表。 |
| F-136 | major | confirmed | correctness | `src/raw/descriptor_set.rs:812` | `persistent_pool_plan` 用 `wrapping_add` 汇总数量，溢出静默缩小池容量。 **当前核实：** 已删除零消费者的 PersistentPoolPlan/persistent_pool_plan 旧助手；生产池继续使用已验证 LayoutPlan 的现有构造器，不保留饱和计数兼容实现。 |
| F-137 | major | confirmed | correctness | `src/raw/descriptor_set.rs:693` | tag 5 在 `:726` 强制 `descriptor_count=1`，但 `:693` 已按原 count 置 `PARTIALLY_BOUND`，`count>1` 的 sampler 得到自相矛盾的布局。 **当前核实：** 已修：immutable sampler 的实际 count 在生成 binding flags 前确定为 1。 |
| F-138 | major | confirmed | concurrency | `src/raw/descriptor_set.rs:483` | `reset` 取 `lock_submission` + 分配锁；`descriptor_pool_ring` 在持有 ring Mutex 时调用它（`pool_ring:132/196`）。  **当前核实：** 已修：ring 仅短锁领取 Arc<Mutex<RingPool>>，native reset/分配都在独立池租约下；device collect 和 native pool 创建均不持 ring 锁。 |
| F-139 | major | confirmed | api | `src/raw/command_buffer.rs:361-373` | `reset_completed` 先 `vkResetCommandPool`，之后才清 `recording` 标志；Vulkan 要求池内命令缓冲不处于 recording/pending。当前依赖"调用方保证无在录命令"的隐含不变量，代码没有断言。 **当前核实：** 需纠正审计：vkResetCommandPool 要求没有 pending 命令；可将 recording 命令整体重置为 initial，不能仅由 recording=true 判定违反规范。 |
| F-140 | major | confirmed | resource | `src/raw/command_buffer.rs:339` | `detach` 后 Drop 已把 `live_commands` 减一，detached 句柄不再计数；`commands.len() == live_commands` 在空切片下成立，可对仍被 setup 缓存持有的缓冲整池重置；`attach:193` 允许多个 wrapper 共用 allocation，但计数是每 wrapper 的。  **当前核实：** 已修：native allocation 共享 last_use 与仍拥有的命令计数；detach 保留计数，重新 attach 不产生另一份完成历史；已退役命令仍由共享 timeline 保护。 |
| F-141 | major | confirmed | concurrency | `src/raw/command_buffer.rs:678` | `prepare_submission_states` 在逐 cell 准备（含分配、cell 锁）全程持有首命令池的 `cpu_state_patches` 互斥量。  **当前核实：** 已修：通用 CachedTempList 租约从锁内取出 scratch，所有 cell 准备/预算工作在锁外进行，退出再短锁归还容量。 |
| F-142 | major | confirmed | perf | `src/raw/command_buffer.rs:587` | 线程局部 `BucketScratch` 的 HashMap/Vec 无上限、不计预算、永不 trim。  **当前核实：** 已修：删除无界线程局部 HashMap/Vec，桶索引使用命令池的预算 TempList 租约并支持 trim；对已排序 use 列表二分定位，不保留第二张指针 HashMap。 |
| F-143 | major | confirmed | concurrency | `src/raw/descriptor_pool_ring.rs:118-173` | 整段分配（含原生池创建/分配）持有 `state` 锁，并在 `:122` 于锁内调用 `try_collect_garbage`（可能轮询/回收 GPU）。  **当前核实：** 已修：分配和创建不持 ring state，creating 额度在锁内预留且 RAII 归还；驱逐 try_lock 忙池，原生退役放到 ring 锁外。 |
| F-144 | major | confirmed | correctness | `src/raw/descriptor_pool_ring.rs:181` | `recycle` 只重置 `allocated != 0` 的池；瞬时池首次分配即被驱动判满（`full=true, allocated=0`，见 `:234-236`）时永不回收，与文件头文档不符。 **当前核实：** 已修：driver full 且 allocated=0 的 transient pool 也纳入 recycle。 |
| F-145 | major | confirmed | correctness | `src/raw/access_table.rs:78` | `0x05 VertexShaderReadUniformBuffer` 用 `SHADER_READ`，而同族 `0x08/0x0b/0x0e/0x11/0x18`（tess/geom/frag UBO 读）都用 `UNIFORM_READ`。后果：`resource_usage::buffer_usage_for_access` 把 `SHADER_READ` 映射为 `STORAGE_BUFFER`，vertex UBO 会被要求带 STORAGE 用法；`rg/vulkan.rs:154` 的 `UNIFORM_READ` 拓宽也覆盖不到它。 **当前核实：** 已修：vertex UBO 使用 UNIFORM_READ，专项码表测试已通过。 |
| F-146 | major | confirmed | correctness | `src/raw/access_table.rs:412-431` | `0x3d AccelerationStructureBuildWrite`、`0x3f AccelerationStructureBufferWrite` 是写，但 `is_write` 只覆盖 `0x28..=0x37`，故 `scope.write=false`；`rg/vulkan.rs:141` 直接用该字段分类读写，这两个码会被当成读，丢失 writer 顺序与初始化判定。（当前无光追路径，属表内一致性缺陷。） **当前核实：** 已修：0x3d/0x3f 按写码处理，专项码表测试已通过。 |
| F-148 | major | confirmed | perf | `src/raw/shader.rs:1455` | 每次 `register` 都 `layouts.clear()` + `cache.invalidate()`，注册/热替换单个 shader 会丢弃全部已建布局与缓存管线。 |
| F-149 | major | suspected | correctness | `src/raw/shader.rs:1660` | `from_compiled_pipeline` 统一用 `ALL_GRAPHICS`（描述符与 push 范围），JSON 路径 `from_reflection` 按 stage 精确合并；同一 shader 两条路径产出不同 `LayoutPlan`。  **当前核实：** 已核实为两种反射来源的保守可见性差异：compiled schema 未携带逐 stage visibility，ALL_GRAPHICS 覆盖真实图形使用且不是无效 Vulkan LayoutPlan；不能凭不同即认定行为错误。 |
| F-150 | major | confirmed | doc | `src/raw/shader.rs:1314` | 注释称 format 取 `scalar.nbyte()*N`、stride 取 `Type::nbyte`，代码两者都用 `input.ty.nbyte()`；实现正确、注释误导。 **当前核实：** 已修注释：format 与 stride 均按完整 Type::nbyte；不改已有正确实现。 |
| F-165 | major | confirmed | resource | `src/rg/transient_resources.rs:602` | `lend` 以 `ResourceName` 为键把成员放进 `BTreeMap`；`create_raw_resource`（`graph.rs:272-295`）不查重，两个同名成员会互相覆盖：被覆盖的 `GpuResource` 直接丢弃、槽位永久为 `None`，`set.lent` 回不到 0，整组 backing 无法回收。二次复核补充：常规路径由 `VulkanGraphBackend::declare`（`rg/vulkan.rs:1082-1084`）对照 `specifications` 拒绝重名，只有直接调用公开的 `create_raw_resource` 才能触发，属防御性缺口。 **当前核实：** 已修：现有 GraphDeclaration::validate 边界拒绝重复 ResourceName，别名成员不再按同名覆盖。 |
| F-166 | major | suspected | concurrency | `src/rg/transient_resources.rs:620` | `evict()` 在持有 AliasCache 锁（`vulkan.rs:1363` 的 guard）时就地 drop AliasSet/vk 对象；同文件 `evict_idle` 特意用 `EvictedSets` 延迟到锁外释放。  **当前核实：** 已修：LRU evict 返回 EvictedSets，实际 native owner 退役发生在 alias cache guard 释放后。 |
| F-167 | major | confirmed | resource | `src/rg/transient_resources.rs:408` | `give_back` 在 `retiring.try_reserve` 失败时丢弃成员且不计数，set 永远 `lent`；仅注释说明，调用方无法观测。  **当前核实：** 已修：lend 事先预留所有在借成员的 retiring 返回槽；give_back 不再申请内存、不会因 OOM 永久丢成员。 |
| F-168 | major | confirmed | test | `src/rg/transient_resources.rs:737` | 测试只覆盖 `PreparedTransients`；`lend`/`give_back`/`evict`/`collect_retiring` 的时序无单元测试。 |
| F-195 | major | suspected | correctness | `src/rg/result_cache.rs:85-95` | `ResultKeyPayload` 不含 `GraphId`，而 `ResultCache` 由 session 持有、`frames` 按图分桶（`:170`），缓存本体跨图共享。两个图若使用相同 `ResultNodeId` 且依赖/参数/描述符一致（生产侧 `render_host.rs:4092` 的 `node = ResultNodeId(source)` 由宿主提供），会互相命中。建议把 `graph_id` 纳入 key（成本一个 u64）。 **当前核实：** 已核实原建议不适用：ResultNodeId 是 session 内跨帧稳定节点身份；每帧 GraphId 都变化，把它加进 key 会关闭全部跨帧命中。依赖/参数/实现/设备代际仍完整比较；需在 API 文档明确命名空间。 |
| F-196 | major | confirmed | perf | `src/rg/result_cache.rs:261` | `owns_token` 全表扫 cache+frames；`collect_cached_results:802-811` 对每个 retired 条目调用 `release_import`→`owns_token`，构成每帧 O(retired×entries)，`:781` 另有一次全表标记。 |
| F-199 | major | confirmed | perf | `src/rg/attachments.rs:526-539` | `cache_refs` 在每个 view 的 `candidate` 内重扫全表，`iter().any(candidate)`（`:535`）与新视图 `dead` 映射（`:539`）均 O(n²)；每次未命中都经 `reserve_views` 触发，附件多的帧代价显著。 |
| F-001 | minor | confirmed | perf | `src/raw/allocator.rs:695-700` | `materialize_peak` 在全局 `state` 锁内克隆整张 `records`（含 `String`）；每次新峰值后的首个 `release` 都会做一次 O(n) 分配 + 复制。 |
| F-002 | minor | suspected | concurrency | `src/raw/allocator.rs:866-884` | `set_total_budget` 未先 `drain_pending`，发布的 `BudgetAction` 可能基于过期的 in-flight 分类。  **当前核实：** 已修：set_total_budget 在现有 state 锁内先 drain_pending，再发布预算动作。 |
| F-007 | minor | confirmed | correctness | `src/raw/buffer.rs:751-753` | `deadline` 是 `Option<Option<Instant>>`：`wait_ns` 的 `checked_add` 溢出时内层为 `None`，`map_or(u64::MAX)` 把"有界等待"变成约 584 年。仅当调用方传入接近 `u64::MAX` 纳秒时触发，实际不可达但语义错误。  **当前核实：** 已修：按 elapsed 从原纳秒预算扣减，不再用嵌套 Option<Instant>，时间点溢出不会扩大等待。 |
| F-008 | minor | suspected | api | `src/raw/buffer.rs:1035-1087` | idle 快路径绕过 `prepare/publish_buffer_host_range` 与 `host_access` 互斥，host 版本簿记不更新，依赖"页游标独占"不变量。 **当前核实：** 已修并发缺口：页游标不能阻止公共 chunk/buffer owner 再次访问同一区间，快速写入现按既有 host_access→submission 顺序持门，通过 idle 证明后复制与登记延后 flush。保留常量页省略逐写范围表事务的优化；新增持有真实 host transaction 时常量写入必须阻塞的回归，dynamic_constants_ring 4、host_buffer_ranges 3 真实 Vulkan 通过。 |
| F-009 | minor | confirmed | validation | `src/raw/texture.rs:719-748` | `image_view` 未校验：`view_type` 与 `kind` 一致、改格式需 `MUTABLE_FORMAT`、`aspect` 属于格式、`D3` 不允许 `layer_count > 1`。 **当前核实：** 已修：在现有 TextureViewDesc::validate 构造边界核对 kind、format、aspect 和层数，缓存命中不重复验证。 |
| F-010 | minor | confirmed | validation | `src/raw/texture.rs:198-215` | `validate` 未限制 `mip_levels <= full_mip_chain_length`；`image_extent:144-158` 对 D1/D2 静默忽略 height/depth。 **当前核实：** 已修：TextureDesc 构造边界拒绝超过完整 mip 链及与 kind 冲突的 extent。 |
| F-011 | minor | suspected | concurrency | `src/raw/transient_backing.rs:167-174` | `add_member` 用 Relaxed、`retire_member` 用 AcqRel，无配对保证：挂接与最后成员退休并发时 `fetch_sub` 可在 0 上回绕，导致 `mark_retiring` 不执行；`Drop:178` 只有 `debug_assert`。  **当前核实：** 已核实原判断不成立：原子 RMW 具有单一 modification order，Relaxed 不会把已经发生的 add 移到对应 retire 后；所有成员在 UnboundResource::bind/MemoryBlock::from_shared 的私有创建链登记，不能合法退休未登记成员。 |
| F-012 | minor | suspected | resource | `src/raw/transient_backing.rs:176-199` | 仅持有 `ash::Device` 克隆；其 Drop 中 `free_memory` 可能晚于最后一个 `Arc<Device>` 释放，正确性依赖上层保活。  **当前核实：** 已核实 owner 合同：实际成员的 MemoryBlock 持 Arc<Device>；最后成员退役后，共享 backing 通过 Device 的退役排空销毁。backing 自身不持 Device 避免设备退役队列形成环。 |
| F-014 | minor | suspected | correctness | `src/raw/texture_upload.rs:471` | `slices.len() == mip_levels` 但链在 1×1×1 提前结束时 `zip` 静默丢弃多余切片，不报错。 **当前核实：** 已修：per_mip_regions 拒绝在完整链提前结束后剩余的切片。 |
| F-015 | minor | confirmed | correctness | `src/raw/texture_upload.rs:430` | `array_elements == 0` 被 `layers.max(1)` 静默当成 1 层。 **当前核实：** 已修：零层数返回 InvalidUsage，不再改写成一层。 |
| F-016 | minor | confirmed | api | `src/raw/temp_list.rs:38-40` | 容量上限与预算超限都用 `Error::Timeout` 表达，与超时语义混淆（`Exhausted` 已存在）。  **当前核实：** 已修：TempList/TempListPool 的容量耗尽统一 Exhausted；frame import 的容量回收分支和专项断言同步迁移，仍先 collect 再重试。 |
| F-017 | minor | confirmed | api | `src/raw/temp_list.rs:157-165` | 从未分配过时 `trim_idle` 也返回 `true`。  **当前核实：** 已修：从未分配 backing 的 trim_idle 返回 false。 |
| F-018 | minor | suspected | resource | `src/raw/temp_list_pool.rs:111-118` | `list.reserve(0)?` 失败时已出栈的 idle 列表被 drop，但 `stats.bytes` 未减、`cached_*` 已减 → 记账漂移（当前不可达但脆弱）。  **当前核实：** 已修：checkout 删除无意义的 reserve(0)，不会在已弹出缓存后出现该不可达错误分支。 |
| F-019 | minor | confirmed | correctness | `src/raw/temp_list_pool.rs:174-178` | `available` 用无检查减法，仅靠上方 `saturating` 比较保证不 panic。  **当前核实：** 已核实不是 bug：available 的普通减法仅在现有 saturating 上限比较通过之后执行；不添加重复热路径检查。 |
| F-022 | minor | confirmed | api | `src/device/budget.rs:193` | `limit == 0`（文档称"不设上限"）时 `available()` 恒返回 0，仅 `raw/allocator.rs:900` 显式绕过。  **当前核实：** 已修：无限预算 available 返回 u64::MAX，与 limit=0 的合同一致。 |
| F-023 | minor | confirmed | resource | `src/device/budget.rs:230` | `sub()` 用 `saturating_sub`，重复/超额释放静默清零，无 `debug_assert` 暴露。  **当前核实：** 已核实为内部记账诊断建议；实际少计来源 F-004 已修。依用户要求不为每次释放增加验证层。 |
| F-024 | minor | confirmed | api | `src/device/init.rs:104` | 拒绝 `memory_budget_bytes == 0`，而同结构 `host_memory_budget_bytes == 0` 表示不限；两个字段的 0 语义相反且未文档化。  **当前核实：** 已核实当前构造合同：GPU budget 必须非零，host budget 的 0 表示无限；不是错误接纳或实际越界，文档应明确差异。 |
| F-025 | minor | confirmed | api | `src/device/init.rs:107` | `frames_in_flight` 硬限 1..=3，字段文档未说明。  **当前核实：** 已核实 1..=3 是实际 frame-bank 所有权的构造边界约束，不是缺陷；不移除该容量限制。 |
| F-026 | minor | confirmed | api | `src/device/init.rs:495` | `extensions.contains(&name.as_ptr())` 比较指针而非内容，依赖 `619-620` 行的 CStr 去重才没产生重复项。  **当前核实：** 已核实最终 enabled extension 集合在进入 Vulkan 前按 CStr 内容去重；前序 pointer contains 仅是避免重复追加的快捷判断，不能单独推断重复启用。 |
| F-027 | minor | suspected | error-handling | `src/device/lifetime.rs:120` | `wait_idle_and_collect` 中队列 `check_alive` 在设备丢失时提前 `?`，不走 `try_collect_all_completed`（该路径把丢失视为全部完成），可回收对象滞留到 drop。 **当前核实：** 已修：等待与回收结果分别保存；等待失败仍执行已完成资源回收，再保留原错误。 |
| F-028 | minor | suspected | resource | `src/device/lifetime.rs:145` | `drain_retirement_for_drop` 是单趟遍历 8 个 FIFO；销毁 `Doomed::Owners` 期间新入列的对象落在已访问队列则不会被本趟销毁，而 `Retirement` 没有 Drop 兜底。仅在 `Device::drop` 调用，进程退出时无害；若将来用于设备重建则成为泄漏。 **当前核实：** 已修：设备最终析构反复排空已完成退役条目，处理析构过程中追加的条目。 |
| F-029 | minor | suspected | resource | `src/device/logical.rs:66` | 构造失败路径按局部变量逆序析构，`core`（VkDevice）先于 `caches` 析构，与模块文档的 `caches → … → core` 相反；当前 `Caches::new` 不建 Vulkan 对象故无害。  **当前核实：** 已核实当前构造路径安全：Caches::new 只建 CPU 空容器，未持原生对象；正常 Device 析构仍按字段 owner 顺序。 |
| F-030 | minor | confirmed | api | `src/device/logical.rs:248` | `queue_family_index_for(kind: u8)` 的 `kind == 1` 只返回"第一个非 primary 族"，compute/transfer 取决于 plan 顺序且未定义；无生产调用者。  **当前核实：** 已删除：不再保留 u8 类别选择的 queue_family_index_for；使用已有类型化 QueueClass/真实队列元数据。 |
| F-031 | minor | confirmed | error-handling | `src/device/memory.rs:266` | `evict_idle_caches` 对 `poll()` 的任意错误（含 DeviceLost）静默返回 0，无日志。  **当前核实：** 已修：poll 失败记录实际错误，不再静默返回 0。 |
| F-032 | minor | confirmed | perf | `src/device/memory.rs:271` | 淘汰命中后 `try_collect_all_completed()` 绕过 `ReclamationConfig` 预算，可能一次性销毁大量对象。 |
| F-033 | minor | suspected | correctness | `src/device/memory.rs:207` | `refresh_memory_budget_if_due` 先把时间戳置为 now 再查询，查询失败也会抑制 250ms 内的重试。  **当前核实：** 已修：驱动查询失败时只清除本次预占的时间戳，允许下一次重试且不覆盖并发更新。 |
| F-034 | minor | suspected | concurrency | `src/device/pending_acquire.rs:193` | 持 pending 列表锁遍历全部条目并逐条 `record_state_patch`（取 SyncCell 锁），生产者在此期间阻塞，持锁时间随批量线性增长。  **当前核实：** 已修：仅在短锁内领取本族候选并更新计数；released_revision、resource gate 和命令录制全部在 pending 列表锁外，失败/保留候选重新入队。 |
| F-035 | minor | confirmed | resource | `src/device/pending_acquire.rs:152` | `requeue_pending_acquires` 在 `try_reserve` 失败时静默丢弃整批 drained，与其它 OOM 路径的 `log::error!` 不一致。 **当前核实：** 已修：requeue OOM 的既有恢复分支记录具体错误；真实释放状态继续保留在资源上。 |
| F-036 | minor | confirmed | doc | `src/device/queue_config.rs:253` | 模块文档称传输类要求 1×1×1 粒度，但 `Prefer` 无候选时直接别名到 graphics 且不复查粒度（仅 `Require` 过滤），实际靠 `transfer.rs:1122` 自查兜底。 |
| F-037 | minor | suspected | correctness | `src/device/queues.rs:69` | `sort_by_key` 未匹配项 `unwrap_or(usize::MAX)` 静默排到末尾，`QueueId` 与 plan 错位不报错（当前 `create_counts` 保证一一对应）。  **当前核实：** 已核实私有不变量：queues 来自同一 plan.create_counts 创建，没有外部未匹配项入口；不加逐次验证。 |
| F-038 | minor | suspected | correctness | `src/device/queues.rs:47` | `plan.queues[0]` 直接索引，依赖 `plan_queues` 必产出 graphics 的不变量。  **当前核实：** 已核实私有不变量：plan_queues 在设备构造边界保证 graphics 队列非空，queues::new 不接受任意外部计划。 |
| F-039 | minor | confirmed | validation | `src/device/submission.rs:184` | 只检查 wait/signal 列表内部重复，未拒绝同一批次中同一信号量既 wait 又 signal（VUID-vkQueueSubmit-pWaitSemaphores-00068），也未拒绝等待本队列自身时间线信号量。  **当前核实：** 原规范依据有误：VUID-00068 要求 binary wait 的 signal 已提交，并不禁止等待自身队列此前已提交的 timeline 点；现有 timeline wait 已拒绝未来值。不能照原建议拒绝合法依赖。规范：https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html 。 |
| F-040 | minor | confirmed | api | `src/device/submission.rs:336` | `native_count(infos.len())?` 的返回值被丢弃，仅作校验副作用。  **当前核实：** 已删除 legacy submission 实现，原 native_count 副作用调用随该路径消失。唯一提交入口使用 SubmitInfo2。 |
| F-041 | minor | confirmed | concurrency | `src/sync/retire.rs:89` | `len` 在 FIFO 锁释放后才 `fetch_add`，而 `collect_limited:147` 弹出后 `fetch_sub`；交错时 `len` 可瞬时下溢为 `usize::MAX`，`retirement_stats().pending` 会读出荒谬值（随后抵消）。也可能瞬时读 0 导致本轮跳过回收。 **当前核实：** 已修：队列条目可见前先发布计数，入列失败归还计数，避免 collector 先减后加。 |
| F-042 | minor | confirmed | error-handling | `src/sync/timeline.rs:301` | `poll()` 逐队列查询，任一队列出错即 `?` 返回，丢弃本次其它队列已取得的进度。  **当前核实：** 已核实并非丢失进度：每个成功查询即写入对应队列的原子完成缓存；后续队列错误只阻止本次完整快照返回，已观察进度仍保留。 |
| F-043 | minor | confirmed | observability | `src/command/cpu_arena.rs:301` | 租约存活期间 `bump_requests`/`failures`/`peak_used` 只在 Drop 时汇总，`pool.stats()` 对已借出块长期显示 0。 **当前核实：** 已明确统计语义：bump 请求在归还租约时汇总，活跃 worker 保留本地非原子计数；不为读取中间统计给每次 bump 分配增加原子开销。已写入字段文档，计数由泛型池的 payload 钩子汇总。 |
| F-044 | minor | confirmed | api | `src/command/cpu_arena.rs:210` | `trim_idle` 以整块为粒度，返回值可超过 `target_bytes`（未文档化）。 **当前核实：** 已文档化：target_bytes 是裁剪后剩余缓存目标，按不可分割整块释放；实际释放字节可超过差额。四类池的裁剪循环已统一到 collections/cpu_pool.rs。 |
| F-050 | minor | confirmed | perf | `src/raw/swapchain/fullscreen.rs:154-213` | 每次配置查询在循环内新建 `get_surface_capabilities2`/`full_screen_exclusive` loader，并对同一 surface 做两轮查询。 |
| F-051 | minor | confirmed | test | `src/raw/swapchain/graph_presentation.rs:40-63` | `arm → submitted → present/abandon` 是保持二值信号量平衡的唯一线性协议，本文件无测试，只有 swapchain.rs 的底层片段测试。 **当前核实：** 原“只有底层片段测试”断言不成立：tests/raw_fixes_swapchain.rs 已覆盖单批/多批 acquire/rendered 信号边界、获取后构图失败、部分提交失败及恢复；entries38 该组 16 项通过。 |
| F-052 | minor | suspected | concurrency | `src/raw/swapchain/graph_presentation.rs:45` | `prepare` 在无 wsi 锁下做最长 3s 栅栏等待。  **当前核实：** 已核实锁外等待是正确方向；F-047 已移除另一入口的 WSI/progress 锁内等待。 |
| F-056 | minor | confirmed | api | `src/raw/init.rs:679` | `available_for_surface` 用 `surface.is_some()` 覆写调用方的 `policy.requires_surface`，显式配置被静默改写。  **当前核实：** 已核实此 helper 的 surface 参数定义枚举上下文，requires_surface 被派生为实际是否给出 surface；生产设置枚举传 Some(surface)，最终设备接纳仍由 DeviceConfig/caps 统一处理。此项属于 API 表达改进，不是当前呈现错误。 |
| F-057 | minor | confirmed | error-handling | `src/raw/init.rs:776-778` | `create_graphics_device` 把 `attach_physical_device_info` 的所有错误折叠成 `invalid_usage("物理设备包装已挂接")`。  **当前核实：** 原结论不成立：attach_physical_device_info 的错误类型就是重复设置时退回的 Arc<PhysicalDevice>，不存在被折叠的其它错误类别；该旧创建助手列入兼容入口清理。 |
| F-058 | minor | confirmed | perf | `src/raw/init.rs:616` | `create_device_configured` 克隆整张设备表（含 extensions Vec）只为取引用。 **当前核实：** 已修：create_device_configured 及旧选择/创建助手已删除；设备创建统一 Device::new，不再克隆设备列表。 |
| F-059 | minor | confirmed | test | `group:src/raw/init.rs` | `device_order_key`/`driver_support`/`meets_original_memory_threshold`/`missing_devices` 无测试（本文件无 test 模块）。 **当前核实：** 原“无测试”断言不成立：tests/raw_init_devices.rs 已覆盖排序/同键稳定性、内存阈值、缺失设备匹配和筛选。最新入口清理前 entries38 的该组 10 项通过；此处不是新增生产检查。 |
| F-060 | minor | confirmed | memory-safety | `src/raw/physical_device.rs:559-568` | 安全 fn `string_from_c_array` 对驱动字节用 `from_utf8_unchecked`，非 UTF-8 即 UB（该符号零引用）。 **当前核实：** 已修：C 字符串用 UTF-8 有损转换，不再用 unchecked。 |
| F-061 | minor | suspected | correctness | `src/raw/physical_device.rs:305` | 直接用驱动给的 `memory_heap_count/memory_type_count` 切 `[_;16]` 定长数组，>16 会 panic 而非报错。 **当前核实：** 已核实边界：Vulkan 驱动给定 count 受规范定义的固定数组上限约束；内存类型数组与 heap 数组上限不同，不能将两者都视作 16。 |
| F-062 | minor | confirmed | test | `group:src/raw/physical_device.rs` | 仅 `clamp_budget_hysteresis` 一条；`DriverVendor`/`DeviceDriverVersion` 解码无覆盖。 |
| F-063 | minor | suspected | resource | `src/raw/device_api.rs:89-121` | `create_mapped_buffer` 先创建并注册缓冲（`:102`），再在 `:105-112` 对非 `HOST_COHERENT` 返回 Err，白做一次分配与命名。  **当前核实：** 该旧助手没有生产消费者；已删除create_mapped_buffer及MappedBufferParts，不保留兼容接口。测试改走唯一managed buffer构造、映射与读写接口，覆盖三类host-visible位置；check77全工作区全目标通过，native78四组共29项通过，含三类宿主可见内存的实际分配/字节往返。 |
| F-064 | minor | confirmed | perf | `src/raw/device_api.rs:221-249` | `update_buffer_bytes` 先把 ranges 收拢进 `gathered` 再写 staging，多一次整段拷贝。 |
| F-065 | minor | confirmed | test | `group:src/raw/device_api.rs` | 无测试（初始上传、映射错误、越界更新未覆盖）。  **当前核实：** “无测试”不成立：tests/device_api.rs与tests/raw_core_buffer_texture.rs实际覆盖初始数据、更新越界、managed映射和原生读回。旧映射接口现已删除，三类host-visible位置改测canonical roundtrip；native78四组29项通过；不以inline测试有无推断覆盖。 |
| F-066 | minor | confirmed | resource | `src/raw/dxgi.rs:169-178` | `SystemDxgi::open` 直接覆盖 `self.factory`，不 `Release` 旧 COM 工厂；`enumerate` 每次调用 `open`，复用同一 source 即泄漏。Windows-only，泄漏量小。 **当前核实：** 已修：SystemDxgi::open 发布新工厂时 Release 旧的拥有引用。Windows 运行验证尚未执行。 |
| F-067 | minor | confirmed | correctness | `src/raw/dxgi.rs:103` | `index += 1` 无上界，注入源永不返回 `End` 时 debug 溢出 panic。  **当前核实：** 已修：DXGI 枚举 index checked_add，耗尽返回 E_BOUNDS 而不回绕或 debug panic；错误上下文改为 DXGI，覆盖工厂和枚举失败。 |
| F-068 | minor | confirmed | test | `group:src/raw/dxgi.rs` | 提供了 `FixedDxgi` 注入点却无测试。 **当前核实：** 原“无测试”断言不成立：tests/crates_domain.rs 已有 FixedDxgi 注入的枚举成功、错误与空列表覆盖；尚未将该历史测试组计为本次最终回归。 |
| F-069 | minor | confirmed | correctness | `src/raw/aftermath.rs:380-384` | `info.len() as u32` 对 >4 GiB 输入静默截断后交 SDK。 **当前核实：** 已修：SDK 输入长度在调用前 checked u32 转换；超长输入拒绝，不截断。 |
| F-070 | minor | confirmed | test | `group:src/raw/aftermath.rs` | `wait_for_status_with` 专为注入设计却无测试。 **当前核实：** 原“无测试”断言不成立：tests/crates_domain.rs 已覆盖 Aftermath 注入 sleep 的成功/错误状态及有界超时；尚未将该历史测试组计为本次最终回归。 |
| F-071 | minor | confirmed | validation | `src/raw/crash.rs:194-211` | `record_capacity` 无上限；只对 word 数 `checked_mul`，`vec![0; debug_words * 4]` 可达数十 GB，OOM 前无拒绝。  **当前核实：** 已修：GPU buffer 容量仍由现有设备预算接纳；初始化改直接填充已映射内存，不再另分配同样大小的零 Vec。 |
| F-072 | minor | confirmed | perf | `src/raw/crash.rs:166-182` | 与 `:297-328` 重复一遍 wait_idle + 读回 + 解析，此处多一次 `wait_idle`。 |
| F-073 | minor | confirmed | api | `src/raw/debug.rs:19-22` | `DebugUtils::new` 无扩展启用校验，ash 在未启用 `EXT_debug_utils` 时 panic；调用点（`device/logical.rs:265/273`）已先检查，但 `pub` 构造器本身不成立。  **当前核实：** 已修公开面：DebugUtils::new 改 pub(crate)，只由已确认 debug-utils 扩展启用的现有设备入口构造；不在逐次命名/标签操作重复验证。 |
| F-074 | minor | confirmed | perf | `src/raw/debug.rs:20` | `device/logical.rs:273` 每次 `set_debug_name` 都新建 `DebugUtils`（重新取函数指针）。 |
| F-075 | minor | confirmed | perf | `src/raw/profiler.rs:36-46` | 每次读回新建 8 KiB 缓冲并产出 1024 项 `Vec<u64>`。 **当前核实：** 已修：raw/profiler 整个零消费者旧模块已删除，包括空操作 send_to_puffin；生产图后端继续拥有 timestamp 查询与读回。 |
| F-076 | minor | suspected | perf | `src/raw/semaphore.rs:71-78` | 队列语义量的 `completed()` 调 `Timelines::poll()`，后者逐队列查询全部队列；逐个语义量轮询即 O(n²) 次驱动查询。 |
| F-077 | minor | confirmed | api | `src/raw/semaphore.rs:86-95` | 队列语义量 `wait` 走 `timelines.wait`，先 `validate_submitted`，对未经本设备提交的值直接 `invalid_usage`，与非队列路径的 `vkWaitSemaphores` 语义不同且未文档化。  **当前核实：** 已核实 queue-owned timeline 只能等本设备已提交点，否则可能永久等待；用户自建 TimelineSemaphore 才允许外部 signal。不同来源确实需要不同合同，不合并为任意值等待。 |
| F-083 | minor | confirmed | api | `src/rg/graph.rs:816` | `ResourceReuseInfo.reused` 在 `populate_resources_into` 中恒为 `false`（池命中记在 `pool_hits`），因此公开的 `PhysicalMapping.reused`（`:1106`）与 `ResourceRegistry::resource().2` 永远报告"未复用"。 **当前核实：** 已修：物理映射 reused 来自实际 pool_hits，不再恒 false。 |
| F-084 | minor | confirmed | perf | `src/rg/graph.rs:1138` | `reclaim_failed_resources` 对每个资源 `pool_hits.contains(...)`，失败路径 O(N²)（低频）。 |
| F-085 | minor | confirmed | api | `src/rg/plan.rs:124` | `differing_key_reason` 的 subgraphs 分支直接 `zip`，长度不同时比较被截断，只影响诊断原因归类。  **当前核实：** 已核实只影响 miss 诊断标签，最终 subgraphs != new.subgraphs 仍拒绝复用，长度差不会命中错误计划。 |
| F-086 | minor | confirmed | perf | `src/rg/plan.rs:914` | gated resource 首次使用者用 `retained_passes.iter().position(...)` 内层再扫 accesses，O(G×P×A) 编译期扫描。 |
| F-087 | minor | suspected | correctness | `src/rg/executor.rs:1471` | 串行屏障分支 `payload_projection(...).1.unwrap()`，依赖 `record_compiled:1233` 的前置校验；校验放宽会在此处 panic 而非返回错误。 **当前核实：** 旧串行录制分支已删除；当前只保留共享的 worker 录制流程。 |
| F-088 | minor | confirmed | error-handling | `src/rg/executor.rs:1144` | 录制失败时用 `backend.take_error()` 覆盖原 `Error::Record/::CallbackPanic`，丢失失败阶段信息。  **当前核实：** 已修：录制后端直接返回 Result；删除 recording_error/take_error 错误旁路及全局覆盖逻辑。CallbackPanic 保持原语义，部分提交保留最初错误和真实回执；check49 通过；result51 新增故障回归及其余 CPU 组通过，native52 的多队列/部分失败/并行图 19 项通过。 |
| F-089 | minor | confirmed | test | `src/rg/executor.rs:2280` | 仅 `pgo_slot_tests`、`exact_image_layout_tests`；`record_compiled`、并行分块、`PartialSubmission`/`abandon_unsubmitted` 状态机无单元测试。 **当前核实：** 原“仅内联测试”不足以推出缺覆盖：tests/rg_fixes_vulkan.rs 覆盖部分提交后命令回收与唯一退役，rg_fixes_cpu.rs 覆盖 PartialSubmission，parallel_chunks.rs 覆盖并行分块。GPU 原生矩阵与取消边界仍需按最终构建验收。 |
| F-090 | minor | confirmed | correctness | `src/rg/sync_plan.rs:611` | `schedule.domains[index]`/`dependencies[index]` 直接索引，长度不一致时 panic 而非返回错误。  **当前核实：** 已修：RangeProgram 编译边界一次核对 schedule 与 batches 长度，拒绝不匹配结构。 |
| F-091 | minor | confirmed | perf | `src/rg/sync_plan.rs:332` | `PhaseIndex` 失效时 `transitions_for/patches_for/waits_for` 退化为每次线性过滤全表（每 pass O(T)）。 |
| F-105 | minor | suspected | validation | `src/bindless_parameters.rs:403` | `written` 下标依赖各 slot `parameter_offset` 严格连续无 padding，而 `written` 长度只由 `parameter_count` 求和；ABI 有洞时越界 panic。  **当前核实：** 已核实已有防护：BindlessParameters::new 调用 ABI.validate，明确拒绝 parameter_offset 非连续、参数数目与尺寸不一致；不是未校验下标。后续移除生产热路径重复验证时应由已验证 ABI 类型承接。 |
| F-106 | minor | confirmed | api | `src/bindless_parameters.rs:426-437` | 只校验 `Fixed` 数组；`Runtime` 未写元素保留 `0xFF` 哨兵（`:272`），shader 索引即 `0xFFFFFFFF` 描述符，无诊断，仅靠 `:501-503` 的 safety 文档兜底。 **当前核实：** 已核实为显式 sparse 合同：Runtime 数组允许未被 shader 索引的元素不绑定；bind_pass 是 unsafe，并要求所有实际索引元素已绑定。补充 finish 文档，不能把 Runtime 全元素验证加到逐 draw 路径或拒绝合法稀疏数组。 |
| F-107 | minor | confirmed | perf | `src/bindless_parameters.rs:487-498` | `with_push` 每次重建 bytes/written 两个 SmallVec 并克隆全部 lease，是逐 draw 常量更新路径。 |
| F-116 | minor | confirmed | perf | `src/transfer/queue.rs:637-654` | `next()` 对每个候选扫描 `jobs[..i]` 找同 target，O(n²)（n≤256）；每次 submit 都调用。 |
| F-117 | minor | confirmed | perf | `src/transfer/queue.rs:374-403` | `task()` 每次入队线性扫描全部 ticket 槽（≤1024）。 |
| F-118 | minor | confirmed | correctness | `src/transfer/queue.rs:694` | `fail_all` 直接取 `jobs[0]`，空队列 panic；两个调用点靠隐式非空保证。  **当前核实：** 已核实私有调用合同：首调用受 queued_jobs!=0 保护，另一调用持有非空 next_batch，且没有并发修改该 manager 的入口；fail_batch 同样只接受已选非空 batch。不增加逐任务验证。 |
| F-119 | minor | suspected | api | `src/transfer/record.rs:358-369` | `image_release_program` 清空 `patches`/`released` 却不清 `scope`，复用同一 scope 会残留上一次的 ownership barrier。  **当前核实：** 重新核实：该函数实际由transfer/texture.rs::release_written_texture调用，之前“无生产调用”的记录有误。生产消费者每次新建scope/patches/released，当前不存在复用残留；该helper是唯一纹理release实现，不属于可删除兼容入口。复用合同仍可收敛，相关真实上传由transfers/native_ranges覆盖。 |
| F-130 | minor | confirmed | correctness | `src/resource/handle.rs:41` | `from_ffi` 不规范化输入：slotmap 1.1 的 `KeyData::from_ffi(0)` 得到 idx=0、version 被 `NonZeroU32` 强制为 1，与首次 `insert` 产生的键相同。外部用 0 表示"无句柄"时会静默命中槽位 0。`is_null` 文档（`:59`）称空句柄可由 `from_ffi` 得到，但真实 `as_ffi` 输出永不为 0。 **当前核实：** 已修：from_ffi(0) 显式产生 null key，不会别名首次插入。 |
| F-131 | minor | confirmed | api | `src/error.rs:59` | `SharedError` 在私有 `error` 模块内 `pub` 却未再导出，却出现在公开的 `rg::graph::BuildError::BackendFailure`（`rg/graph.rs:216`，经 `rg/mod.rs:58` 再导出）；公开签名包含外部无法命名的类型。  **当前核实：** 已修：SharedError 在规范错误公开面再导出，公开 BuildError 的源类型可以命名；不恢复 raw::error 兼容别名。 |
| F-132 | minor | confirmed | error-handling | `src/error.rs:201` | `usage_error!` 把整组 `BufferUpdateError`（区间/容量/缺 `TRANSFER_DST` 等参数错误）统一写成"渲染状态无效"，误导日志读者。  **当前核实：** 已修：BufferUpdateError 单独转换为 buffer update 上下文，并保留结构化源错误。 |
| F-133 | minor | suspected | error-handling | `src/error.rs:112` | `From<Arc<Error>>` 对 `InvalidUsage` 把原 Arc 塞进 source；是否重复打印取决于消费端是否遍历 source 链，`Display` 本身不重复。  **当前核实：** 已核实非缺陷：Arc<Error> 的 source 保留原错误及其唯一 owner，Display 不重复；去掉 source 反而丢失原始信息。 |
| F-134 | minor | confirmed | api | `src/lib.rs:49` | 只再导出 `dump_log/dump_log_vec/init_logging/print_log_dump`，未导出 `write/record_event/record_named`；`:14` 把 log ring 列为公开面，但外部只能 init+dump 而无法写入——这正是 IMP-01 "全仓无写入者"的公开面成因。  **当前核实：** 已修：真实 fern 日志接入有界 record_log；初始化移至 logger 安装处。删除零消费者 record_event/record_named/dump_log/print_log_dump 旧入口。崩溃 render.log 有实际生产者。 |
| F-151 | minor | confirmed | api | `src/raw/submit.rs:209` | `with_vk` 不校验六个数组长度一致，字段均为 `pub`（`:65-72`）；安全方法可把长度不等的数组交给驱动，回调内提交即越界读。当前零引用。  **当前核实：** 已删除零消费者 with_vk，禁止继续通过旧 callback 适配器绕开规范提交构造。 |
| F-152 | minor | confirmed | perf | `src/raw/setup.rs:293` | 每次 `record` 都 `attach` 新建 `VulkanCommandPool` wrapper（含 SyncCell、两个 TempList、元数据），仅作临时句柄。 |
| F-153 | minor | suspected | correctness | `src/raw/dynamic_constants.rs:330` | `stats.backing_bytes -= ...` 未 checked，账目偏差时 debug 下溢 panic。  **当前核实：** 已核实非算术缺陷：backing_bytes 与 pages 在同一 owner 锁内加入和移除，减数来自被移除的实际 page；原建议只是重复校验未知记账偏差，依用户要求不增加。 |
| F-154 | minor | confirmed | correctness | `src/raw/dynamic_constants.rs:160` | `MAX_ALLOCATIONS_PER_PAGE = 1000` 依赖"每写 ≤2 cell"与外部 `RangeStateConfig::max_cells = 4096`。  **当前核实：** 已核实当前常量安全且已有明确注释：每次写至多新增两 cell，1000 写小于默认 4096 的一半；当前构造器不允许替换为较小的 RangeStateConfig。 |
| F-155 | minor | confirmed | error-handling | `src/raw/resource_access.rs:349` | `emit_barriers` 失败只返回，未写 `range_failure`；此时 `record_patches:188` 已落补丁，调用方若忽略错误再 `end()`（只查 `range_failure`）会提交"有补丁无屏障"的录制。当前调用方均传播错误。  **当前核实：** 已修：emit_barriers 的失败也写入命令原有 range_failure，忽略返回值仍不能 end/submit。 |
| F-156 | minor | confirmed | perf | `src/raw/resource_access.rs:360` | 每轮 `issued.contains(stages)` + `find`，O(k·n)，k 通常很小。 |
| F-157 | minor | suspected | correctness | `src/raw/format.rs:108` | `try_vk_format_info` 令 `bytes[9] = bytes[8]`；观测表只含 word00/word04/byte08，第 10 字节是推断值。  **当前核实：** 已核实消费者：country-core texture_library 使用 word04 与 byte08；未使用推断的 byte09。该投影并非零消费者，不能直接删除业务格式表。byte09 的精确原侧证据尚未确认，不把该字段提升为已恢复语义；后续应缩窄未使用的公开字节投影。 |
| F-158 | minor | confirmed | correctness | `src/raw/format.rs:34` | `vk_format_all_aspects` 用 `0x10u32 << i` 造 `PLANE_n`，未限定 `i <= 2`；注册表扩容到 >3 planes 会 shift 溢出。 **当前核实：** 已修：固定三 plane 数组的迭代按数组上限截断。 |
| F-159 | minor | confirmed | perf | `src/raw/shader_interface.rs:138` | `validate_spirv_layout` 每次重新做 SPIR-V 反射 + 每 entry 的 JSON schema 序列化，未复用 `ShaderModule.interfaces`。 |
| F-160 | minor | confirmed | concurrency | `src/raw/shader/pipeline_cache.rs:257` | `cached_compute_count` 持表锁时对每个 slot 取阻塞锁；某线程正持同 slot 锁做原生创建时会连带阻塞 `slot_for`/`invalidate`。 **当前核实：** 已修：compute cache 只在锁内取 Arc 快照，等待每个 slot 在表锁外。 |
| F-161 | minor | confirmed | test | `src/raw/shader/pipeline_key.rs:5` | `cfg(test)` 的 `append_dynamic_rendering_key` 复写生产分支，但测试只与自身比较，生产键漂移不会被发现。 |
| F-162 | minor | confirmed | perf | `src/raw/shader/reflection.rs:181` | 每个 entry point 都 `module.clone()` + `view.assemble()` 重序列化整模块，O(entries×module)；仅加载期。 |
| F-163 | minor | suspected | correctness | `src/raw/shader/reflection.rs:348` | Matrix 的 JSON 只存 `"vector"` 字符串 + count + stride，丢元素标量类型，`canonical_type` 无法区分不同元素类型的矩阵。  **当前核实：** 原结论不成立：锁定的 spirq 0.6.2 VectorType::Display 生成 vecN<scalar>，包含 f32/f64/i32 等元素类型；matrix.vector 字符串没有丢标量类型。 |
| F-164 | minor | suspected | correctness | `src/raw/shader/reflection.rs:297` | `descriptor_type = format!("{desc_ty:?}")` 与 `shader.rs:1105-1110` 的字面串精确匹配，耦合 spirq 的 Debug 输出。  **当前核实：** 已核实为版本锁定的反射投影耦合，不是当前类型解析错误；所有支持 DescriptorType 的字符串均由同一 pinned spirq 枚举产生。移除 JSON 中间投影属于结构清理。 |
| F-169 | minor | confirmed | correctness | `src/rg/access_ledger.rs:40` | 组号计数器 u32 回绕到 0（测试 `:437-443` 明确断言），而 0 既是 `ResourceSyncGroup::NONE` 又是 upgrade 规则写死的 group 0（`:199-269`）；长期运行回绕后不同组访问会被当作同组去重/升级。  **当前核实：** 已修：组号使用 checked fetch_update，耗尽拒绝分配而不回绕到保留组 0/1 或仍存活编号。 |
| F-170 | minor | confirmed | perf | `src/rg/resource.rs:158` | `current_version` 每次调用全量扫描所有 pass 的全部访问；`PassBuilder::latest_version` 与每次 `texture_id`/`buffer_id` 都走它，typed 访问密集的构图呈 O(访问总数²)。 |
| F-171 | minor | confirmed | api | `src/rg/resource_registry.rs:180` | `backing()`/`resource()` 在未填充资源（`backing_index == u16::MAX`）时越界 panic；只有 `is_populated` 的文档提到该前提。 **当前核实：** 已核实为准备后索引访问合同：执行器在资源 population/validate 边界拒绝不可用资源，访问器供准备完成的物理表索引；补充 resource 的 Panics 与 is_populated 文档，不为每次资源读取新增重复验证。 |
| F-172 | minor | suspected | correctness | `src/rg/resource_registry.rs:238` | `into_exports` 用 `?` 把未映射导出静默丢弃，与"执行器已拒绝"的注释不符。  **当前核实：** 已核实执行链边界：record_compiled 在首次 begin 前统一拒绝导出资源无 payload；into_exports 处理有效 registry 的实际资源移动，不为同一映射再加一层检查。 |
| F-173 | minor | confirmed | perf | `src/rg/resource_registry.rs:59` | `take_texture_where`/`take_texture` 取空后保留空 Vec 键，反复变化的 key 会累积空桶（只有 `forget_handles` 修剪）。 **当前核实：** 已修：取走最后一项时移除空的资源 key，避免变化规格遗留空桶。 |
| F-174 | minor | suspected | correctness | `src/rg/temporal.rs:332` | buffer 命中分支对所有描述符差异一律 `replace=false`，请求不同 size 的 buffer 会静默复用旧分配；image 同类差异则报 `DescriptorMismatch`，行为不对称。 **当前核实：** 已修：具名 buffer 描述符变化创建新分配，不冒充旧分配的大小。 |
| F-175 | minor | confirmed | perf | `src/rg/temporal.rs:295` | `requested()` 每次分配并 clone 一个 Vec；`vulkan.rs:2793/2805` 同帧调用两次。 |
| F-176 | minor | confirmed | perf | `src/rg/temporal.rs:180` | `owns_token` 线性扫描，`vulkan.rs:3572` 遍历全部后端 token 时逐个调用，O(n·m)。 |
| F-177 | minor | suspected | resource | `src/rg/temporal.rs:459` | `set_resource_sharing` 失败时已记录插入并置位 `frame_requested`，但 `sharing` 未回滚，下一帧会误判策略变化而重建分配。  **当前核实：** 已核实当前不可触发：lookup.handle 刚由同一 graph 创建，sharing 由已验证 QueueFamilies 表达，set_resource_sharing 不会在此路径失败；不新增逐帧回滚验证层。 |
| F-178 | minor | confirmed | error-handling | `src/rg/frame_loop.rs:435` | `wait_idle` 对每个 ticket 直接 `ticket.wait(timeout)?`，未走 `observe`，这里的 DeviceLost 不会写回 timelines。 **当前核实：** 已修：wait_idle 的呈现等待错误经 FrameLoop::observe 发布。 |
| F-179 | minor | suspected | resource | `src/rg/frame_loop.rs:341` | `end_display_frame` 在 `execute_frame_protocol` 返回后无条件调用；该函数只对 presentation 回调做 `catch_unwind`，主图 `execute_graph` panic 展开时 display frame 不会被关闭。  **当前核实：** 已修：FrameLoop::execute 无论协议返回还是 unwind 都先 end_display_frame，再返回或恢复原 panic。 |
| F-180 | minor | confirmed | resource | `src/rg/profiling.rs:165` | CPU 预算只按容器元素计费，漏掉 `slot_names` HashMap 与每条 sample 的 `pass_name: String`（`:421-428` 每次 `to_string` 分配）。  **当前核实：** 已修：slot_names 改预分配并计费的按 slot 数组；样本引用 ResourceName 的静态 intern 字符串，不再为每条 sample 分配 String。 |
| F-181 | minor | confirmed | error-handling | `src/rg/profiling.rs:259` | 无空闲槽时 `begin` 返回 `Ok` 而不计时（仅 `stats.capacity_skips += 1`），调用方以为时长有效。  **当前核实：** 已核实统计合同：无槽返回不影响主渲染；capacity_skips 明确报告缺样本，samples 只输出真实结果，未伪造零时长。 |
| F-182 | minor | confirmed | correctness | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `dispatch_threads_3d` 用 `group_size.max(1)` 静默吞掉 0 组大小，实际组数偏大而非报错。  **当前核实：** 已修：线程 dispatch API 接受 NonZeroU32 工作组尺寸，零尺寸在类型边界不可表达；删除 max(1) 静默修正。 |
| F-183 | minor | confirmed | validation | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `render_with_context` 未调用 `check_pass_capacity`（对比 `:109/151/193`），纯 `side_effect()` 无访问的 pass 不受 `MAX_PASSES` 早退保护。  **当前核实：** 已核实非越界缺陷：GraphDeclaration::validate 与 ResourceInfo 构建边界统一检查 MAX_PASSES；render 回调无访问也被覆盖。依用户要求不在每个 pass 再重复检查。 |
| F-184 | minor | suspected | correctness | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `reads_previous` 硬编码 `0x36 \| 0x37`，表内新增同类码时 typed 写会静默丢失内容依赖。  **当前核实：** 已核实当前 access 码表：只有 0x36/0x37 是此 typed 写访问隐含读取前值的码；新增码时应更新语义表，但当前没有遗漏的已支持码。 |
| F-185 | minor | suspected | validation | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `retain_pass_resources` 直接下标 `registry.resources[...]`，不一致时 panic。  **当前核实：** 已核实 registry 只能来自已通过 graph declaration 与资源映射接纳的 FrameInstance；每访问下标无须重复验证。 |
| F-186 | minor | suspected | validation | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | 按未经边界检查的 `resource_index` 下标 `graph.resources`。  **当前核实：** 已核实 validate_ranges/validate_contents 位于统一 GraphDeclaration::validate 合同之后；引用先经资源索引边界检查，原文忽略了调用顺序。 |
| F-187 | minor | confirmed | perf | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `covers_iter` 对每个 read 线性扫描全部前序 write 且反复 `writes.clone()`。 |
| F-188 | minor | confirmed | correctness | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `MemoryRequirement.prefers_dedicated` 被采集但规划器只用 `requires_dedicated`，偏好专用分配的资源仍会被别名合并。 **当前核实：** 已核实：prefers_dedicated 是驱动建议，requires_dedicated 才是必须独占的正确性条件；是否采纳偏好留到性能测量，不能作为安全错误。 |
| F-189 | minor | confirmed | validation | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | 交叠判断用未检查的 `b_offset + b.size`（同函数前文对 placement 用了 `checked_add`），极端 size 在 debug panic、release 回绕误判。 **当前核实：** 已修：独立 placement 检查器用 checked_add 拒绝偏移溢出。 |
| F-190 | minor | confirmed | api | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `end_frame` 用 `push_back` 可能分配，而 `reserve_frame:69` 只是建议性预保留。 **当前核实：** 已核实当前唯一生产消费者在 compile/submit 前 reserve_frame；后续没有并发插入同一个 &mut throttle，end_frame 的 push 不再分配。 |
| F-191 | minor | suspected | correctness | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `required[2] += 1` 无注释地假定 scratch 对应 binding 2 且需多一个槽。  **当前核实：** 已核实 required 下标 2 是 DescriptorKind::StorageBuffer，并非 shader binding 2；额外的一槽对应参数块 descriptor，scratch 已在 resources 中计数。 |
| F-192 | minor | confirmed | perf | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | 每次 `add_pass` 都重查 `get_physical_device_properties`，并用 `graph.buffer_id` 触发 `current_version` 全量扫描。 |
| F-193 | minor | suspected | validation | `src/rg/pass_api.rs:61-62` | `split` 的 `load + w` 与 `:61-62` 的 `sum/high` 均未用 checked 运算。  **当前核实：** 已修：均衡切分的累加/预算使用 u128，u64 观测耗时与固定开销的总和不会溢出；已删除只为均值建立的 known Vec。 |
| F-194 | minor | confirmed | test | `group:src/rg/pass_api.rs,pass_builder.rs,pass_bindings.rs,resource_contracts.rs,transient_memory.rs,transient_memory_check.rs,frame_throttle.rs,indirect_count.rs,parallel.rs` | `balanced_chunks`/`plan_chunks` 无单元测试。 **当前核实：** 已纠正：tests/parallel_chunks.rs 已有真实分块算法测试，当前 8 项通过；旧结论过时。 |
| F-197 | minor | confirmed | api | `src/rg/cache.rs:198` | `classify_miss` 仅靠 `debug_assert` 保证此前有 Cold miss；release 下 Cold=0 时 `saturating_sub` 吞掉减量，miss 总数虚增。 **当前核实：** 已核实当前私有调用不触发：仅 executor 和 subgraph 两处生产调用，均紧随同一 enabled lookup 的 Cold miss 分支；命中、禁用和 lookup 错误不会调用。保留既有 debug 不变量，不新增热路径状态或检查。 |
| F-198 | minor | confirmed | correctness | `src/rg/cache.rs:243` | 单条超预算返回 `Inserted::Rejected` 时不再执行替换，同键旧条目继续生效；调用方可从返回值区分，但若忽略返回值会读到旧值。 **当前核实：** 已核实为显式 API 合同：Rejected 返回新 value，原条目仍存；调用方必须检查 Inserted，不能当作无条件替换。 |
| F-200 | minor | suspected | correctness | `src/rg/subgraph.rs:558` | `normalized().unwrap_or(range)` 吞掉规范化错误，非法 range 静默进入缓存键与 `analyze_passes`，可能缓存错误分析。  **当前核实：** 已修：规范化 range 的错误直接传播，不缓存未经规范化的旧输入。 |
| F-201 | minor | confirmed | perf | `src/rg/graph_caches.rs:165` | `evict_idle` 无条件丢弃投影快照与投影池（即使 parked 已够 target），任何一次压力驱逐都强制下帧重建 CPU 投影。 |
| F-202 | minor | suspected | resource | `src/rg/graph_caches.rs:95` | 同 token 重复 `park` 会替换并直接 drop 旧 `GpuResource`，不推入 `evicted`，会话池仍保留该 token 的旧视图。  **当前核实：** 已核实当前单 owner 入口：park_pooled 先从 backend.resources remove，同 token 不能被第二次合法 park；claim 必先从 cache remove 才能重新归还。 |
| F-203 | minor | suspected | resource | `src/rg/frame_storage.rs:258-265` | `return_tables` 在 `actual > bytes` 时只置 `reusable=false`，不回写 `live_bytes`/reservation，Drop 前宿主预算少计；`:265` 的 `expect("shrinking CPU reservation")` 在 resize 失败时 panic。  **当前核实：** 已修：异常增长超过已接纳容量的返回表立即释放，不以旧的小 reservation 保留到 lease Drop；缩小 reservation 本身是不可失败的释放动作。 |
| F-204 | minor | suspected | correctness | `src/rg/receipt_storage.rs:253` | `publish` 用 `assert!` 校验容量，而容量来自 `vulkan.rs` 的启发式估计（`batches + 4×(accesses+exports)`），估计偏小时是 release panic 而非返回错误。  **当前核实：** 已修：receipt storage 移到绑定 backend，由真实 bound.batches 数量在首次 submit 前补齐容量；删除资源/访问数启发式。ownership prelude 循环上限也采用 bound batch 数。 |
| F-205 | minor | confirmed | api | `src/rg/receipt_storage.rs:155` | `statistics()` 名为只读统计，却把独占槽 `reclassify` 为 `Cache`，读取会改变预算分类。 **当前核实：** 已修：statistics 改为 &self 只读快照，预算分类只在池借还/裁剪维护时更新；新增 Weak 引用禁止复用、取消归还及重复查询预算不变回归。 |
| F-206 | minor | suspected | validation | `src/rg/plan_check/bound.rs:200-207` | 等待点只校验 `value != 0`；`covers:246` 对 `Point` 只比数值，未用 `known` 前沿作上界——指向未提交/未来 point 的 wait 会被判为已满足。校验器是 debug 兜底，漏检意味着运行期可能死锁而非构建期报错。  **当前核实：** 已修既有可选校验器：Point wait 必须在 initial native frontier 的已知上界内，未来点不能作为覆盖证明。生产未新增验证层。 |
| F-207 | minor | suspected | correctness | `src/rg/plan_check.rs:586` | `passes == 0` 时 `bounds` 取 `pass_bounds`（None），但该分支要求外部资源 `first_pass.is_some()`，可能误拒"仅含导入/导出、无 pass"的空计划。  **当前核实：** 原结论不成立：resource_info 对零 pass 外部资源约定 first_pass=Some(0)、last_pass=None；plan_check 正是检查此合同，非误拒。 |
| F-208 | minor | suspected | correctness | `src/rg/plan_check/native_state.rs:24` | `wait_for` 对同队列 point 完全不比较数值，重放中同队列非单调（acquire 早于 release）不会被拒，仅靠 `state.owner` 相等兜底。  **当前核实：** 已修：同队列等待拒绝大于实际本次提交的 point；等于本次 point 允许表达同一 command 内较早 patch 的前沿。 |
| F-209 | minor | confirmed | api | `src/rg/owner_storage.rs:62` | Local 内联 4 槽满时返回 `Error::Timeout`，把容量耗尽伪装成超时/背压。  **当前核实：** 已修：Local owner inline 容量满返回 Exhausted。 |
| F-210 | minor | confirmed | api | `src/rg/attachment_contract.rs:172-186` | `from_flat`/`pass_samples` 不校验各附件采样数一致；`pass_samples` 直接取深度（否则首个颜色）附件的采样数，混合采样静默通过纯契约校验。  **当前核实：** 已修：pass_samples 拒绝颜色与深度附件的混合采样数；现有纯合同测试改为验证拒绝与一致样本通过。 |
| F-211 | minor | confirmed | resource | `src/rg/resource_pool.rs:13` | `_ => {}` 静默丢弃非 `OwnedTexture/Buffer` 的 `RegistryResource`，失去回池复用且无断言。  **当前核实：** 已核实正确 owner 区分：Unowned/Empty 等条目没有回池所有权，必须不插入拥有资源池；不是遗漏 Owned 分支。 |
| F-212 | minor | suspected | resource | `src/rg/vulkan_presentation.rs:459-489` | `present()` 先 `self.presentation.take()` 再执行；错误路径不回插槽位，图像归还完全依赖租约 Drop。结合批次 3 的 `SwapchainImageLease` 缺陷（`released=true` 无条件置位），present 失败时图像可能卡在 acquired。 **当前核实：** 已随 F-045 修正；present 槽取走后出错依赖独占租约 Drop 是既有正常失败协议。 |
| F-213 | minor | confirmed | resource | `src/rg/vulkan_presentation.rs:510-536` | `reap_presentation` 只回收 `!executed && alive` 已失效的槽；`executed` 但既未 present 也未 cancel 的槽要等 session Drop。属设计选择，但缺少"执行后未消费"的显式诊断。  **当前核实：** 已核实为显式呈现租约合同：执行后图像保持 session 独占，等待 present 或 cancel；槽只有一份，不会逐帧无界积累。自动因图声明 alive 消失取消已执行呈现会抢走仍可合法消费的图像，不能照原建议自动回收。 |
| F-214 | minor | confirmed | test | `src/rg/vulkan_presentation.rs:621-700` | 有 5 个针对多队列/丢弃/所有权移交的测试（本文件测试覆盖相对较好）。 **当前核实：** 正面覆盖记录，不是缺陷；现有 5 个内联测试保持，运行结果以各轮矩阵为界。 |
| F-215 | minor | confirmed | error-handling | `src/rg/vulkan_queue.rs:175` | `Arc::get_mut(planner)` 失败时返回 `Error::Timeout`；这是"状态被共享"而非超时，应为 `InvalidUsage`/`Exhausted`。  **当前核实：** 已修：绑定时 planner 仍有其它 Arc owner 返回 InvalidUsage，区别于等待超时。 |
| F-216 | minor | confirmed | perf | `src/rg/vulkan_queue.rs:190` | `for pass in batch.passes.clone()` 每个 batch 克隆一次 pass 列表；计划编译期，规模小但可借用切片。  **当前核实：** 原“克隆 pass 列表”的推断不成立：BoundBatch 的 passes 是 std::ops::Range<usize>，此处 clone 仅复制两个端点，随后直接迭代，不分配 Vec 或复制 pass 元素；无需修改生产路径。 |
| F-217 | minor | confirmed | validation | `src/rg/vulkan_queue.rs:201-210` | 非 graphics 族的 stage 支持性检查只按 `COMPUTE`/`TRANSFER` 二分；新增队列类（如视频编解码）时需要同步扩展，代码无提示。  **当前核实：** 已核实当前支持队列仅 graphics/compute/transfer；视频队列并未暴露为已支持入口，不为尚不存在类型加热路径验证。 |
| F-218 | minor | confirmed | test | `group:src/rg/vulkan_queue.rs` | 本文件无内联测试，且无类型被 integration tests 直接引用（见覆盖缺口清单）。 |
| F-219 | minor | suspected | correctness | `src/transfer/texture.rs:156-166` | `pack_regions` 直接 `staging[start..start + region.data.len()]` 且 `base + copy.buffer_offset` 未 checked；长度不符时 panic，32 位宿主 `as usize` 会截断。 **当前核实：** 已核实当前私有 producer：plan_texture_upload 与 layout 预先校验并以 checked u64/usize 构造同一地区表；pack 不接受独立外部 plan。 |
| F-220 | minor | suspected | correctness | `src/transfer/texture.rs:169-183` | `pack_all`/`pack_regions` 用 `zip` 对齐 `plan.copies` 与 `regions`，长度不一致时静默少拷（不报错）。 **当前核实：** 已核实当前私有 producer：plan 与 regions 来自同一上传请求；zip 不接受任意配对。边界校验留在计划生成处，不逐份复制重复检查。 |
| F-221 | minor | confirmed | correctness | `src/transfer/texture.rs:125` | 与 `raw/dynamic_constants.rs:567`、`raw/suballoc.rs:491` 重复的第四份 `align_up`（见 IMP-09）。  **当前核实：** 已核实为工具重复，不是算术正确性缺陷；收敛安排在兼容/泛型清理阶段。 |
| F-222 | minor | confirmed | test | `group:src/transfer/texture.rs` | 仅 `layout_aligns_each_texture_to_its_copy_alignment` 一条；`pack_regions`/`record_transfer_dst_copies` 无测试。 |
| F-223 | minor | confirmed | dead-code | `src/raw/sampler.rs:320-323` | `CREATE_SAMPLER_EXPECT_MSG`、`SAMPLER_NOT_FOUND_PREFIX` 零引用（见 IMP-44）。 **当前核实：** 已修：两条无消费者的历史 expect/错误文本常量及其失效注释已删除。 |
| F-224 | minor | confirmed | perf | `src/raw/sampler.rs:325-327` | `builtin_sampler_spec` 线性查 13 项；调用点在创建期，可接受。 |
| F-225 | minor | confirmed | test | `group:src/raw/sampler.rs` | 无测试：`create_info()` 对 13 个内置 spec 的映射、`new_builtin` 的存储记账均未覆盖。 |
| F-226 | minor | confirmed | api | `src/raw/error.rs:1-2` | 整个文件是 `pub use crate::error::*` 的兼容壳，唯一实体是 `observe_vulkan`。建议把 `observe_vulkan` 移到 `crate::error`，删除该模块，避免"raw::error 与 crate::error 是两个东西"的误解。  **当前核实：** 已完成：observe_vulkan 迁至规范 crate::error；删除 raw/error.rs 及全部实际消费者和测试中的旧命名空间。 |
| F-013 | refuted | refuted | correction | `src/raw/texture_upload.rs:202-299` | 二次复核判定为误报：`image_access_borrowed` → `steps()` → `family()`（`raw/resource_access.rs:38-41` 检查 `is_recording()`）且 `raw/resource_access.rs:222` 显式比较 `texture.device` 与 `command.pool.device`。清零路径已间接覆盖这两项检查，不构成缺陷。 **当前核实：** 原审查已驳回；无新增缺陷或修复动作，保留原间接设备/录制检查。 |
| F-147 | refuted | refuted | correction | `src/raw/shader.rs:825-861` | 二次复核判定为误报：`try_vec` 用 `try_reserve_exact`，`colors` 由 `try_vec(len)+extend` 构造，因此 `capacity()==len()`；两处 `pipeline_array_bytes` 的四个参数实际相等，`reserve_shared_cpu_metadata` 也已含 `shared_cpu_bytes::<Self>()`，无记账差异。 **当前核实：** 原审查已驳回；当前容量及 Arc 元数据计费结论不变，无新增修复动作。 |
| F-003 | info | confirmed | note | `group:src/raw/allocator.rs` | 已复核：`allocate`/`register` 失败路径均正确归还 handle、native slot 与记账。 **当前核实：** 正面记录；失败分配回滚仍归还 native 所有权和配额，不为既有回滚加入重复检查。 |

## 2. 逐文件结论

| 批次 | 文件/分组 | 结论 | 发现数 | 发现 ID |
| --- | --- | --- | ---: | --- |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/allocator.rs` | 轻微 | 3 | F-001 F-002 F-003 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/suballoc.rs` | 值得修 | 3 | F-004 F-005 F-006 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/buffer.rs` | 轻微 | 2 | F-007 F-008 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/texture.rs` | 轻微 | 2 | F-009 F-010 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/transient_backing.rs` | 轻微 | 2 | F-011 F-012 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/texture_upload.rs` | 轻微 | 3 | F-013 F-014 F-015 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/temp_list.rs` | 轻微 | 2 | F-016 F-017 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/temp_list_pool.rs` | 轻微 | 2 | F-018 F-019 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/managed/texture.rs` | 值得修 | 2 | F-020 F-021 |
| 批次 1：raw 内存 / 缓冲 / 纹理 | `src/raw/managed/buffer.rs`, `handle.rs`, `mod.rs`, `resources.rs`, `stats.rs`, `bytes.rs` | 无问题 | 0 | — |
| 批次 2：device / sync / command | `src/device/budget.rs` | 轻微 | 2 | F-022 F-023 |
| 批次 2：device / sync / command | `src/device/init.rs` | 轻微 | 3 | F-024 F-025 F-026 |
| 批次 2：device / sync / command | `src/device/lifetime.rs` | 轻微 | 2 | F-027 F-028 |
| 批次 2：device / sync / command | `src/device/logical.rs` | 轻微 | 2 | F-029 F-030 |
| 批次 2：device / sync / command | `src/device/memory.rs` | 轻微 | 3 | F-031 F-032 F-033 |
| 批次 2：device / sync / command | `src/device/pending_acquire.rs` | 轻微 | 2 | F-034 F-035 |
| 批次 2：device / sync / command | `src/device/queue_config.rs` | 轻微 | 1 | F-036 |
| 批次 2：device / sync / command | `src/device/queues.rs` | 轻微 | 2 | F-037 F-038 |
| 批次 2：device / sync / command | `src/device/submission.rs` | 轻微 | 2 | F-039 F-040 |
| 批次 2：device / sync / command | `src/sync/retire.rs` | 轻微 | 1 | F-041 |
| 批次 2：device / sync / command | `src/sync/timeline.rs` | 轻微 | 1 | F-042 |
| 批次 2：device / sync / command | `src/command/cpu_arena.rs` | 轻微 | 2 | F-043 F-044 |
| 批次 2：device / sync / command | `src/device/{caches,caps,generation,policy,reclamation,mod,native}.rs`, `src/sync/mod.rs`, `src/command/{mod,recording}.rs` | 无问题 | 0 | — |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/swapchain.rs` | 值得修 | 5 | F-045 F-046 F-047 F-048 F-049 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/swapchain/fullscreen.rs` | 轻微 | 1 | F-050 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/swapchain/graph_presentation.rs` | 轻微 | 2 | F-051 F-052 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/instance.rs` | 值得修 | 3 | F-053 F-054 F-055 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/init.rs` | 轻微 | 4 | F-056 F-057 F-058 F-059 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/physical_device.rs` | 轻微 | 3 | F-060 F-061 F-062 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/device_api.rs` | 轻微 | 3 | F-063 F-064 F-065 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/dxgi.rs` | 轻微 | 3 | F-066 F-067 F-068 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/aftermath.rs` | 轻微 | 2 | F-069 F-070 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/crash.rs` | 轻微 | 2 | F-071 F-072 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/debug.rs` | 轻微 | 2 | F-073 F-074 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/profiler.rs` | 轻微 | 1 | F-075 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/semaphore.rs` | 轻微 | 2 | F-076 F-077 |
| 批次 3：raw 设备 / 实例 / 交换链 | `src/raw/device.rs`, `src/raw/graveyard.rs` | 无问题 | 0 | — |
| 批次 4：rg 核心执行链 | `src/rg/vulkan.rs` | 值得修 | 5 | F-078 F-079 F-080 F-081 F-082 |
| 批次 4：rg 核心执行链 | `src/rg/graph.rs` | 轻微 | 2 | F-083 F-084 |
| 批次 4：rg 核心执行链 | `src/rg/plan.rs` | 轻微 | 2 | F-085 F-086 |
| 批次 4：rg 核心执行链 | `src/rg/executor.rs` | 轻微 | 3 | F-087 F-088 F-089 |
| 批次 4：rg 核心执行链 | `src/rg/sync_plan.rs` | 轻微 | 2 | F-090 F-091 |
| 批次 4：rg 核心执行链 | `src/rg/sync_plan/bind.rs` | 值得修 | 5 | F-092 F-093 F-094 F-095 F-096 |
| 批次 4：rg 核心执行链 | `src/rg/queue_schedule.rs` | 值得修 | 2 | F-097 F-098 |
| 批次 4：rg 核心执行链 | `src/rg/recorder.rs`, `src/rg/compiled.rs`, `src/rg/transaction.rs` | 无问题 | 0 | — |
| 批次 5：bindless / pipeline / transfer | `src/bindless.rs` | 值得修 | 6 | F-099 F-100 F-101 F-102 F-103 F-104 |
| 批次 5：bindless / pipeline / transfer | `src/bindless_parameters.rs` | 轻微 | 3 | F-105 F-106 F-107 |
| 批次 5：bindless / pipeline / transfer | `src/pipeline.rs` | 值得修 | 4 | F-108 F-109 F-110 F-111 |
| 批次 5：bindless / pipeline / transfer | `src/transfer.rs` | 值得修 | 4 | F-112 F-113 F-114 F-115 |
| 批次 5：bindless / pipeline / transfer | `src/transfer/queue.rs` | 轻微 | 3 | F-116 F-117 F-118 |
| 批次 5：bindless / pipeline / transfer | `src/transfer/record.rs` | 轻微 | 1 | F-119 |
| 批次 6：resource 与根文件 | `src/resource/range_state.rs` | 值得修 | 5 | F-120 F-121 F-122 F-123 F-124 |
| 批次 6：resource 与根文件 | `src/resource/tracked.rs` | 值得修 | 5 | F-125 F-126 F-127 F-128 F-129 |
| 批次 6：resource 与根文件 | `src/resource/handle.rs` | 轻微 | 1 | F-130 |
| 批次 6：resource 与根文件 | `src/error.rs` | 轻微 | 3 | F-131 F-132 F-133 |
| 批次 6：resource 与根文件 | `src/lib.rs` | 轻微 | 1 | F-134 |
| 批次 6：resource 与根文件 | `src/resource/{mod,name,sharing}.rs`, `src/log_ring.rs`, `src/interop.rs`, `src/present.rs` | 无问题 | 0 | — |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/descriptor_set.rs` | 严重（潜伏，当前无调用者） | 4 | F-135 F-136 F-137 F-138 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/command_buffer.rs` | 值得修 | 4 | F-139 F-140 F-141 F-142 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/descriptor_pool_ring.rs` | 值得修 | 2 | F-143 F-144 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/access_table.rs` | 值得修 | 2 | F-145 F-146 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/shader.rs` | 值得修 | 4 | F-147 F-148 F-149 F-150 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/submit.rs` | 轻微 | 1 | F-151 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/setup.rs` | 轻微 | 1 | F-152 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/dynamic_constants.rs` | 轻微 | 2 | F-153 F-154 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/resource_access.rs` | 轻微 | 2 | F-155 F-156 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/format.rs` | 轻微 | 2 | F-157 F-158 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/shader_interface.rs` | 轻微 | 1 | F-159 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/shader/pipeline_cache.rs` | 轻微 | 1 | F-160 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/shader/pipeline_key.rs` | 轻微 | 1 | F-161 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/shader/reflection.rs` | 轻微 | 3 | F-162 F-163 F-164 |
| 批次 7：raw 命令 / 描述符 / 着色器 | `src/raw/barrier.rs`, `src/raw/binding.rs`, `src/raw/resource_usage.rs`, `src/raw/format_table.rs` | 无问题 | 0 | — |
| 批次 8：rg 其余模块 | `src/rg/transient_resources.rs` | 值得修 | 4 | F-165 F-166 F-167 F-168 |
| 批次 8：rg 其余模块 | `src/rg/access_ledger.rs` | 轻微 | 1 | F-169 |
| 批次 8：rg 其余模块 | `src/rg/resource.rs` | 轻微 | 1 | F-170 |
| 批次 8：rg 其余模块 | `src/rg/resource_registry.rs` | 轻微 | 3 | F-171 F-172 F-173 |
| 批次 8：rg 其余模块 | `src/rg/temporal.rs` | 轻微 | 4 | F-174 F-175 F-176 F-177 |
| 批次 8：rg 其余模块 | `src/rg/frame_loop.rs` | 轻微 | 2 | F-178 F-179 |
| 批次 8：rg 其余模块 | `src/rg/profiling.rs` | 轻微 | 2 | F-180 F-181 |
| 批次 8：rg 其余模块 | `src/rg/pass_api.rs`, `pass_builder.rs`, `pass_bindings.rs`, `resource_contracts.rs`, `transient_memory.rs`, `transient_memory_check.rs`, `frame_throttle.rs`, `indirect_count.rs`, `parallel.rs` | 轻微 | 13 | F-182 F-183 F-184 F-185 F-186 F-187 F-188 F-189 F-190 F-191 F-192 F-193 F-194 |
| 批次 8：rg 其余模块 | `src/rg/pass_bindings/projection.rs`, `resource_info.rs`, `frame.rs`, `frame_imports.rs`, `import_export.rs`, `indirect_count_spirv.rs`, `binding.rs`, `mod.rs` | 无问题 | 0 | — |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/result_cache.rs` | 值得修 | 2 | F-195 F-196 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/cache.rs` | 轻微 | 2 | F-197 F-198 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/attachments.rs` | 值得修 | 1 | F-199 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/subgraph.rs` | 轻微 | 1 | F-200 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/graph_caches.rs` | 轻微 | 2 | F-201 F-202 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/frame_storage.rs` | 轻微 | 1 | F-203 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/receipt_storage.rs` | 轻微 | 2 | F-204 F-205 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/plan_check/bound.rs` | 轻微 | 1 | F-206 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/plan_check.rs` | 轻微 | 1 | F-207 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/plan_check/native_state.rs` | 轻微 | 1 | F-208 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/owner_storage.rs` | 轻微 | 1 | F-209 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/attachment_contract.rs` | 轻微 | 1 | F-210 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/resource_pool.rs` | 轻微 | 1 | F-211 |
| 批次 9：rg 校验器与缓存/存储 | `src/rg/plan_check/range_oracle.rs`, `range_program.rs`, `rg/token_map.rs`, `rg/util.rs` | 无问题 | 0 | — |
| 补漏批次：代理未覆盖的 5 个文件（本人复核） | `src/rg/vulkan_presentation.rs` | 轻微 | 3 | F-212 F-213 F-214 |
| 补漏批次：代理未覆盖的 5 个文件（本人复核） | `src/rg/vulkan_queue.rs` | 轻微 | 4 | F-215 F-216 F-217 F-218 |
| 补漏批次：代理未覆盖的 5 个文件（本人复核） | `src/transfer/texture.rs` | 轻微 | 4 | F-219 F-220 F-221 F-222 |
| 补漏批次：代理未覆盖的 5 个文件（本人复核） | `src/raw/sampler.rs` | 轻微 | 3 | F-223 F-224 F-225 |
| 补漏批次：代理未覆盖的 5 个文件（本人复核） | `src/raw/error.rs` | 轻微 | 1 | F-226 |

## 3. 二次复核（pass-2）记录

方法：把 §1 中全部 213 个行引用重新读回源码（含上下文；函数头引用展开读函数体），逐条判断断言是否成立。下表只列**发生变更**的条目。

| 类型 | 发现 | 修正 |
| --- | --- | --- |
| refuted | F-013 | 清零路径的 `is_recording()` 与设备一致性检查由 `family()` / `image_access_borrowed` 覆盖，原判断不成立 |
| refuted | F-147 | `try_vec` 使用 `try_reserve_exact`，`capacity()==len()`，两处预算参数实际相等 |
| softened | F-006 | `flags` 在 `allocate_dedicated` 回落路径会传入，原文"从未传给 gpu-allocator"过强 |
| softened | F-198 | `Inserted::Rejected` 是显式返回值，只有忽略返回值才会读到旧条目 |
| softened | F-133 | 是否重复打印取决于消费端是否遍历 source 链，`Display` 本身不重复 |
| severity-down | F-165 | 常规 `declare()`（`rg/vulkan.rs:1082-1084`）已拒绝重名，仅直接调用公开 `create_raw_resource` 可达 |
| line-fix | F-100 | `bindless.rs` 781→767（`resident_lease` 实际起点） |
| line-fix | F-078 | `rg/vulkan.rs` 2524→2525 |
| line-fix | F-215 | `vulkan_queue.rs` 176→175 |
| line-fix | F-216 | `vulkan_queue.rs` 187→190 |
| line-fix | F-206 | `plan_check/bound.rs` 196-203→200-207 |
| runtime | F-011 | `SharedBacking::drop` 是否可能晚于最后一个 `Arc<Device>`：取决于资源字段析构顺序，静态读不出结论 |
| runtime | F-195 | 跨图命中是否真实发生：取决于宿主 `ResultNodeId` 是否跨图重复 |

复核结论：除上表外，其余 200 条断言与代码一致；9 个最高优先问题（F-045、F-112、F-135、F-195、F-020/F-021/F-047/F-141/F-143、F-145/F-146、F-004、F-120、F-099）已逐行确认。

## 4. 方法与限制

- **覆盖**：145/145 个源文件；生产代码 63,896 行、内联测试 15,251 行（按 `#[cfg(test)]` 项剥离后统计）。
- **pass-1**：按目录分 9 批并行阅读（raw 内存/设备/命令、device+sync+command、rg 执行链、rg 校验与缓存、bindless/pipeline/transfer、resource+根文件、rg 其余），另有 5 个批次未覆盖的文件由人工补查。
- **pass-2**：全部行引用回读；误报与措辞过强项已在上表修正，正文不再保留错误断言。
- **限制**：
  1. 静态阅读无法证明并发时序、GPU 行为与 Drop 顺序；`suspected` 条目需要运行验证。
  2. 行号以 2026-10-24 工作树为准；重构后需按 ID 重新定位（`grep` 发现文本中的特征串）。
  3. "无问题"不等于已证明正确，只表示本次走查未发现可报告缺陷。
- **复核命令**（行号漂移后可用）：`grep -rn "<发现文本中的特征串>" src`。

## 5. 需要运行验证的条目

| 发现 | 为什么静态不可判定 | 建议验证方式 |
| --- | --- | --- |
| → F-011 | `SharedBacking` 与 `Arc<Device>` 的析构先后取决于结构体字段顺序与最后一次 `Arc` 释放点 | 写一个 drop 顺序探针测试：创建资源→释放 Device→观察 `free_memory` 是否在 `vkDestroyDevice` 之后调用 |
| → F-195 | 宿主 `ResultNodeId` 是否跨图重复只有上游代码可知 | 构造两个图、同 node id、同依赖，断言不命中 |

其余 `suspected`（共 59 条）可由 §1 的状态列筛出，多数需要构造注入或并发压力测试。

## 6. 优先关注（按处理顺序）

| 顺序 | 发现 | 一句话 |
| ---: | --- | --- |
| 1 | F-045 | raw 交换链租约在 present 失败路径上仍标记 released，图像卡在 acquired |
| 2 | F-112 | 纹理上传提交成功后 `queue_acquires` 失败会丢失暂存槽 `last_use` |
| 3 | F-135 | `with_create_info` 的 immutable sampler 数组按 `descriptor_count` 越界读（潜伏，当前无调用者） |
| 4 | F-195 | 结果缓存键缺 `GraphId`（需先确认宿主 node id 作用域） |
| 5 | F-020 / F-021 / F-047 / F-141 / F-143 | 持锁跨 Vulkan 调用或阻塞等待（5 处） |
| 6 | F-145 / F-146 | 访问码 0x05 用错 access 位；0x3d/0x3f 写码未被 `is_write` 覆盖 |
| 7 | F-004 | 子分配器 `native_objects` 少计，`maxMemoryAllocationCount` 护栏失效 |
| 8 | F-120 | 容量/状态错误被返回为 `Timeout`（另有 5 处同类） |
| 9 | F-099 | bindless retiring 队列在生产中恒为空，文档与统计不符 |
| 10 | F-165 | 别名集同名成员互相覆盖（仅 raw 入口可达） |


