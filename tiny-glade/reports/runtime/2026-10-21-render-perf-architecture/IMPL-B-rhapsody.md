# IMPL-B:rhapsody 每帧 CPU 复杂度降到 O(passes + accesses) 的实施规范

2026-10-21。只读分析产物,未构建、未运行、未改源码。对应 `ANALYSIS.md` §2.4 与 §3 "第 B 层"。所有行号以当前工作区 `crates/rhapsody/src/**` 为准(`git` HEAD `f98b40229`)。标注 **【未核实】** 的内容需要实施者先确认。

## 0. 范围、符号与验收门

### 0.1 符号

| 符号 | 含义 | 目标图(主图)的量级 |
|---|---|---|
| N | 保留 pass 数(`CompiledPlan::retained_passes`) | ~160 |
| M | 逻辑资源数(`GraphDeclaration::resources`) | ~200【未核实】 |
| U | 原生 use 数(`RangeProgram::uses`,一 pass 内重叠声明合并后) | ~3000 |
| S | 规划期 shadow cell 总数(`SynchronizationPlanner::shadow`),随 use 的范围切分增长 | 500~3000 |
| T | 绑定后转换数(`BoundSynchronization::transitions`) | ~3300(`MULTIFRAME-BASELINE`) |
| P | 绑定后补丁数(`BoundSynchronization::patches`),≥ U | ~3300 |
| B | 批次数(`BoundSynchronization::batches`,含 ownership prelude) | 1~3 |
| W | 跨队列等待数(`BoundSynchronization::waits`) | 单队列为 0 |
| C | 本帧提交的命令缓冲数 | 1~16 |
| K | 一页动态常量上的宿主写入次数 | 数十~数百 |
| I | 导入资源数(`OriginSource::Import*`,含 named/temporal/交换链) | 20~60【未核实】 |

### 0.2 当前每帧 CPU 复杂度(B 层前)

```
bind_queue_resources   : O(M·log M) 锁 + 2M 互斥量 + O(U·S + U·A + W²)      —— B1
recorder/batch 过滤     : O(N·T + B·(P+T+W))                                   —— B3
prepare_submission_states: O(cells·ΣP_cmd) + 每 cell 全量 prepare/coalesce 重放 —— B2
计划缓存查找            : O(LRU 深度·(N+M+U)),miss 时 O(128·(N+M))            —— B5
动态常量                : O(K²) 每页 + 3K 互斥量                               —— B6
字符串                  : M 次 String 比较/构造 + format! 每个 named/temporal   —— B4
```

目标:**稳态(计划缓存命中)每帧 O(N + U + I·cells) 且堆分配 ≈ 0**。

### 0.3 验收门(所有项共用)

1. **现有测试全绿**(`cargo test -p rhapsody`,lavapipe):以下文件与 B 层直接相关,每项的"验证"小节只列增量。
   - `tests/native_ranges.rs`(5 个用例,范围切分/多族/部分失败)
   - `tests/multi_queue_graph.rs`(5 个用例)
   - `tests/wave3_queues.rs`(3 个用例,其中 2 个直接调用 `check_bound_synchronization`)
   - `tests/transient_aliasing.rs`(8 个用例)、`tests/transient_buffer_aliasing.rs`
   - `tests/plan_checker.rs`(9 个用例)
   - `tests/cpu_frame_allocations.rs::fixed_graph_reports_real_build_and_cached_execution_allocations`(断言 arena 开启时稳态 `allocations == 0`,`#[ignore]` 需手动跑)
   - `tests/gpu_frame_allocations.rs::real_vulkan_session_reports_framework_execution_allocations`
   - `tests/rg_fixes_vulkan.rs`(`submission_captures_stay_bounded_without_begin_frame`、`partial_submission_failure_recycles_all_commands_and_runs_retire_once`、`named_slots_validate_scope_initialization_resize_and_reset`)
   - `tests/dynamic_constants_ring.rs`(2 个用例)、`tests/host_buffer_ranges.rs`(3 个用例)
   - `tests/graph_cache_eviction.rs`、`tests/subgraph_plan.rs`、`tests/managed_projection.rs`、`tests/parallel_chunks.rs`
   - `tests/p0_stability.rs`、`tests/state.rs`、`tests/triangle_animation_graph.rs`
2. **性能门**:`tests/p0_perf_baseline.rs`
   - `representative_graph_frame_cost_baseline`(`#[ignore]`,6 pass):打印 `CPU begin+构图+execute` mean/p50/p95、每帧 `commands_recorded`/`barrier_commands`/`barrier_transitions`/`vkQueueSubmit`/`submit batch`/`graph batch`。B 层前后 **barrier_commands / barrier_transitions / graph_batches 必须逐帧相等**(同步计划语义不变),CPU mean 下降。
   - `render_perf_throughput`(`#[ignore]`,release):每帧 `begin_ns`/`build_ns`/`execute_ns`、`allocation_counter::measured` 的 `allocations`/`allocated_bytes`、`gpu_allocations`、`wait_slot_ns`、`ExecutionStats{compile_ns, record_ns, submit_ns}`。门:稳态 `allocations == 0`,`compile_ns + record_ns + submit_ns` 在 160 pass 合成图上 < 2 ms(release,lavapipe 只看 CPU)。
   - `cargo bench --bench perf`:**rhapsody 目前没有 `benches/` 目录,`Cargo.toml` 无 `[[bench]]`**【未核实:工作区其他 crate 也未发现 `benches/*/perf.rs`】。B 层第一步需新增 `crates/rhapsody/benches/perf.rs`(用 `tests/cpu_graph_support` 的 CPU 后端,合成 160 pass / 3000 use / 50 资源 / 8 导入的图),计数器:`bind_ns`(新增到 `ExecutionStats`,见 B1)、`record_ns`、`submit_ns`、`allocations`(复用 `tests/allocation_counter`)、`shadow_cells_visited`(B1 新增 debug 计数)。基准必须在 B1 改动前先落地并记录数字。
3. **校验器**:`debug` 构建 / `validation` feature / `rg::set_plan_validation(true)` 下,`plan_check::check_plan`(缓存 miss)、`check_bound_synchronization`(B1 后每帧 bind 之后,见 B1 §4)、`check_state_publication`(B2 后每次成功提交,见 B2 §4)都必须通过。release 默认关闭。

### 0.4 实施顺序

B4(去字符串,独立、低风险)→ 基准落地 → B3(索引化,为 B1 缓存提供切片结构)→ B1(增量 bind)→ B2(提交不重放,依赖 B1 的"最终 cell"输出)→ B5 → B6 → B7。B1 与 B2 共享 "Create 资源不再持有运行期状态表" 这一决定,必须一起评审。

---

## B1 sync bind 增量化

### B1.1 现状

**数据结构**(`src/rg/sync_plan/bind.rs`):

```rust
// bind.rs:91-102
struct ShadowCell {
    resource: u32,
    range: NativeRange,
    layout: vk::ImageLayout,
    layout_known: bool,
    writer: PendingFrontier,     // [AccessScope; MAX_QUEUES] + host + [Option<DependencyRef>; MAX_QUEUES]
    readers: PendingFrontier,
    owner: PendingOwner,
    revision: u64,
    fresh: bool,
}
// bind.rs:138-146
pub struct SynchronizationPlanner {
    bound: BoundSynchronization,   // initial/aliases/batches/order/transitions/patches/waits,全部 Storage<T>(Vec + 预算票据)
    shadow: Storage<ShadowCell>,   // 所有资源的 shadow cell 混放在一个扁平 Vec 里
    budget: Arc<AllocatorStats>,
    config: SynchronizationConfig,
    next_pair: u64,
    owner_bytes: usize,
    _owner_memory: Option<CpuMemoryReservation>,
}
```

`bind_use` 的主循环对每个 use 扫全部 shadow cell(`bind.rs:411-420`):

```rust
let before = self.shadow.values.len();
let mut covered = false;
for index in 0..before {
    let mut current = self.shadow.values[index];
    if current.resource != access.resource {
        continue;
    }
    let Some(overlap) = range_state::intersection(current.range, access.range) else {
        continue;
    };
    covered = true;
    ...
    // bind.rs:431-436:每个命中的 cell 再扫一遍全部别名边
    if current.fresh
        && self.bound.aliases.values.iter()
            .any(|edge| edge.next == access.resource || edge.previous == access.resource)
        && (...) { return Err(...); }
    ...
    // bind.rs:557-568:再扫一遍
    if current.fresh {
        current.fresh = false;
        if self.bound.aliases.values.iter().any(|edge| edge.next == access.resource) {
            current = self.discard_alias_contents(current);   // 内部:对每条 edge 再扫全部 shadow(bind.rs:372-388)
        }
    }
```

其他线性扫描:`wait()` 去重用 `waits.iter_mut().find(...)`(`bind.rs:740-746`);`later()` 用 `order.iter().position`(`bind.rs:339-345`);`static_dependency` 对目标批次每个 use 做 `uses(source).any(...)`(`bind.rs:322-327`)。

**每帧物理绑定**(`src/rg/vulkan_queue.rs:209-322` `bind_queue_resources`):对 `physical_mapping` 的每个资源:`self.resources.get(&token)`(BTreeMap)、`import_families.get(&token)`(BTreeMap)、`domains.iter().find(...)`、`object.owner_cell()`(Arc clone)、`with_range_state(...)`(`tracked.rs:221-230`:取 `submission_gate` 再取 `native_state` 两把 `parking_lot::Mutex`,必要时 `ensure_range_state_locked` 新建 `RangeStateTable`)、`planner.add_initial(logical, table.cells())` 把 cell 复制进 `initial` 与 `shadow`。随后 `planner.bind(&compiled.plan.range_program, ...)` 对**每帧**重新把整个 `RangeProgram` 叠加一遍,不论计划缓存是否命中。`prepare_queue_plan`(`vulkan_queue.rs:156-208`)每帧对全部 use 重新做队列族 stage 合法性检查(纯计划属性,与帧无关)。

`BoundSynchronization`(`sync_plan.rs:179-187`)是 bind 的唯一输出,由 recorder(B3)、`attach_range_patches`、`queue_submission_dependencies` 消费。

### B1.2 复杂度

- `bind`:U 个 use × S 个 shadow cell = **O(U·S)**;每命中 cell 额外 O(A)(A 为别名边数)×2;`discard_alias_contents` O(A·S);`wait` O(W) 每次;`later` O(B);`static_dependency` O(U_src·U_dst)。主图 U≈3000、S≈1000~3000 → 每帧 3×10^6~10^7 次 `intersection` + 分支。
- 物理绑定:O(M·log M) BTreeMap + 2M 互斥量 + M 次 Arc clone + O(ΣI cells) 复制。
- `prepare_queue_plan`:O(U)(纯计划属性)。
- 稳态无堆分配(`Storage` 容量复用),但 CPU 时间是超线性的。

### B1.3 目标设计

#### (a) 按资源索引 shadow cell(消灭 O(U·S))

```rust
// bind.rs —— SynchronizationPlanner 新增字段
struct ResourceShadow {
    /// 指向 `shadow.values` 的下标;切分时新 cell 追加到尾部并登记到这里。
    cells: smallvec::SmallVec<[u32; 4]>,
    /// bit0 = 作为某条别名边的 `next`;bit1 = 作为 `previous`。add_alias 时置位。
    alias_flags: u8,
    /// 本资源作为 `next` 时的全部 `previous`(discard_alias_contents 用)。
    alias_previous: smallvec::SmallVec<[u32; 2]>,
}
pub struct SynchronizationPlanner {
    bound: BoundSynchronization,
    shadow: Storage<ShadowCell>,
    by_resource: Vec<ResourceShadow>,          // len == program.resources.len(),begin() 只 clear 内层、保留容量
    waits_by_batch: Vec<smallvec::SmallVec<[u32; 4]>>, // batch → waits 下标;wait() 去重只看本批次
    order_position: Vec<u32>,                  // batch → 在 order 中的位置,u32::MAX = 未排
    ...
}
```

算法改动(全部是局部替换,语义不变):

1. `begin()`:对 `by_resource` 每项 `cells.clear(); alias_flags = 0; alias_previous.clear()`,`resize_with(program.resources.len(), Default::default)`;`waits_by_batch` 同理;`order_position.fill(u32::MAX)`。SmallVec 溢出到堆后容量保留,稳态无分配。
2. `add_initial(resource, cells)`:每 `push_shadow` 后 `by_resource[resource].cells.push(index)`。
3. `add_alias(previous, next)`:`by_resource[next].alias_flags |= 1; by_resource[previous].alias_flags |= 2; by_resource[next].alias_previous.push(previous)`。
4. `bind_use`:把 `for index in 0..before` 改为

```rust
let list_len = self.by_resource[access.resource as usize].cells.len();
for slot in 0..list_len {
    let index = self.by_resource[access.resource as usize].cells[slot] as usize;
    let mut current = self.shadow.values[index];
    let Some(overlap) = range_state::intersection(current.range, access.range) else { continue; };
    ...
}
```

   `push_shadow` 产生的 outside 片段追加到 `by_resource[resource].cells`(位置 ≥ `list_len`,本轮不再访问,与现有"只访问原 cell"语义一致)。两处 `aliases.iter().any(...)` 改为 `alias_flags & 1 != 0`/`alias_flags != 0`;`discard_alias_contents` 改为遍历 `alias_previous` 再遍历 `by_resource[prev].cells`。
5. `wait()`:去重在 `waits_by_batch[batch]` 内查找;`push_wait` 后登记下标。
6. `later()`:`order_position[batch]`。`push_order` 时写 `order_position[batch] = order.len()`。
7. `static_dependency`:跨队列才进入(同队列已早退),保留现状;若 W>0 的场景实测仍慢,再按批次建 `SmallVec<u32>` 的已排序资源表做归并。

#### (b) 缓存绑定结果:`BoundTemplate` + `ImportStateKey`

**关键观察**:计划缓存命中时,`RangeProgram` 不变;bind 的输出里**与帧无关**的是转换(`BoundTransition` 不含时间线值)、批次、顺序、`DependencyRef::Batch` 型等待;**与帧有关**的只有:(1) 每个补丁的 `expected_revision`(每帧单调 +k),(2) 导入资源初始 frontier 的 `SyncPoint`(每帧新的提交点,进入 `DependencyRef::Point` 等待),(3) 导入资源初始布局/所有者/scope/cell 划分(稳态下第 2 帧起不变)。因此把 bind 输出拆成"模板"与"重定基"两部分。

```rust
// sync_plan.rs —— 新增
/// 一个导入资源初始 cell 的帧无关投影。`revision` 与 `points` 的数值不在键里,
/// 只保留"哪些队列上有点"的掩码(决定是否产生跨队列等待)。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImportCellKey {
    pub range: NativeRange,
    pub layout: vk::ImageLayout,
    pub layout_known: bool,
    pub owner: OwnerKey,                        // Unclaimed | Owned(family);Released 不可缓存
    pub writer_scopes: [AccessScope; MAX_QUEUES],
    pub writer_host: AccessScope,
    pub writer_queue_mask: u8,                  // SyncSet 中有点的队列位图
    pub readers_scopes: [AccessScope; MAX_QUEUES],
    pub readers_host: AccessScope,
    pub readers_queue_mask: u8,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerKey { Unclaimed, Owned(u32) }

/// 整帧的初始状态键:按 `physical_mapping` 逻辑序排列的全部 cell 键,
/// 加别名边身份与队列域。64 位哈希只用于快速判否,命中由全量比较确认(ARCHITECTURE §4.4)。
pub struct ImportStateKey {
    pub hash: u64,
    pub cells: Vec<(u32 /*logical*/, ImportCellKey)>,  // 稳态容量复用
    pub alias_edges: Option<Arc<[(u32, u32)]>>,        // Arc::ptr_eq 比较
    pub domains: [Option<QueueDomain>; MAX_QUEUES],
}

/// bind 的帧无关模板。所有 Storage 与今天相同,只是补丁/等待改为"相对"形式。
pub struct BoundTemplate {
    pub plan: Weak<CompiledPlan>,          // 命中条件 1:Arc::ptr_eq
    pub key: ImportStateKey,               // 命中条件 2:全量相等
    pub bound: BoundSynchronization,       // transitions/batches/order/waits(Batch 型)不动
    pub patch_bases: Vec<PatchBase>,       // 与 bound.patches 一一对应
    pub external_waits: Vec<ExternalWait>, // 由初始 frontier 点派生的等待(模板里不含具体 SyncPoint)
    pub final_cells: Vec<FinalCellTemplate>, // B2 使用:每个导入资源在每个批次之后的最终 cell
}
#[derive(Clone, Copy)]
pub struct PatchBase { pub initial_slot: u32, pub delta: u32 }  // expected_revision = initial[slot].revision + delta
#[derive(Clone, Copy)]
pub struct ExternalWait { pub batch: usize, pub initial_slot: u32, pub queue: QueueId, pub stages: vk::PipelineStageFlags }
```

`BoundSynchronization` 保持公开只读接口不变(`transitions()/patches()/waits()/...`),内部新增 `patches` 的 `expected_revision` 字段由重定基步骤回填,`waits` 由模板的 Batch 型等待 + 重定基后的 Point 型等待拼成(`waits` 末尾追加,`waits_by_batch` 同步)。

**每帧流程**(`bind_queue_resources` 重写):

```
1. 收集初始状态(仍需对 I 个导入资源取锁;Create 资源按 B2 规则合成常量初态,不取 native_state 锁):
     for logical in physical_mapping:
        if Create → initial = 合成态(见 B2.3(c)),cells = [整域 1 个 cell],revision 恒 0
        else      → cell.with_existing_range_state(|t| 复制 t.cells()) (1 把 gate + 1 把 native_state)
     同时增量计算 ImportStateKey.hash(FxHash/ahash 风格的 u64 混合,不用 SipHash)
2. 快路径判定:
     template.plan.ptr_eq(plan) && template.key.hash == key.hash && template.key.cells == key.cells
        && Arc::ptr_eq(alias_edges) && domains 相等
   → 重定基(O(P + W_ext)):
        for (i, base) in patch_bases: bound.patches[i].expected_revision = initial[base.initial_slot].revision + base.delta
        截断 waits 到模板长度;for ew in external_waits: bound.waits.push(BoundWait{batch, dependency: Point(initial[slot].writer/readers.points[queue]), stages})
        不调用 planner.bind
   → 否则:planner.begin(); add_initial; add_alias; planner.bind(...);  随后由 bind 产出 patch_bases / external_waits / final_cells 并替换模板
3. runtime.points.resize(batches.len())
4. 若 validation_enabled():check_bound_synchronization(&plan.range_program, planner.bound())
```

模板生成时 bind 内部需要知道每个补丁对应的"初始 slot":`ShadowCell` 增加 `initial_slot: u32`(切分出的子 cell 继承父 cell 的 slot),`revision` 改为相对值(从 0 起),`push_patch` 时同时 `patch_bases.push(PatchBase{initial_slot, delta: current.revision as u32})`。`PendingFrontier::from_native` 时把 `DependencyRef::Point(point)` 改记为 `DependencyRef::External { slot, queue }`(新枚举变体,仅规划期存在),`wait()` 遇到 External 时写入 `external_waits` 而不是 `bound.waits`;`BoundPatch::resolve` 遇到 `Acquire{release: External}` 视为不可缓存(模板 `cacheable = false`,见下)。

**不可缓存条件**(模板 `cacheable=false`,每帧走全量 bind,与今天行为一致):任一初始 cell `owner == Released`;bind 产生了 `OwnershipPrelude` 批次;`multi_queue` 且存在 Acquire 补丁。这三种只在多族独占资源上出现,默认别名队列设备永远走快路径。

**`prepare_queue_plan` 的 stage 合法性检查**移到计划缓存 miss 路径(它只依赖 `RangeProgram` 与 `queue_flags`,不依赖帧),结果随 `CompiledPlan` 存一个 `queue_flags_signature: u64`,命中时只比较签名。

**锁与查表**:`resources.get(&token)` 与 `import_families.get(&token)` 两个 BTreeMap 查找改为一次:`bind_import` 时在 `GpuResource` 旁存 `owner_family: Option<u32>`,或在 `QueueRuntime` 里维护 `resolved: Vec<(u64 /*token*/, *const SyncCell, u32 /*family*/)>` 随 `physical_mapping` 顺序(B7 的 projection 也用它)。`owner_cell()` 的 Arc clone 改为借用 `&SyncCell`(bind 期间 `self.resources` 不变)。

#### (c) 与 `CompiledPlan` 的关系

- 随计划缓存(不可变):`RangeProgram`、`queue_flags_signature`、B3 的 `transitions_by_phase`/`patches_by_batch` 的**顺序模板**。
- 随会话(`QueueRuntime`)每图种类一个槽(见 B5 的 `GraphKind`):`BoundTemplate`。模板持 `Weak<CompiledPlan>`,计划被 LRU 驱逐后自动失效。
- 每帧重算:初始 cell 复制(I 个导入)、`ImportStateKey`、重定基。

### B1.4 不变量

1. 快路径产出的 `BoundSynchronization` 必须与"对同一初始状态全量 `bind`"的产出**逐字段相等**(transitions、batches、order、patches 含 expected_revision、waits 的集合相等)。校验:`check_bound_synchronization(program, bound)`(`plan_check/bound.rs:326`)——它用独立稠密格重推导转换/等待/补丁。**目前生产帧路径没有调用它**(只有 `vulkan_presentation.rs:857` 的单元测试与 `tests/wave3_queues.rs`、`native_ranges.rs`、`transient_buffer_aliasing.rs`),B1 必须在 `bind_queue_resources` 末尾 `validation_enabled()` 下接入。另加一个 debug 断言模式 `RHAPSODY_BIND_SHADOW=1`:快路径后再跑一次全量 bind 到备用 planner,`assert_eq!` 两份 `BoundSynchronization`。
2. 所有权准入只有 `RangeOwner::admits`/`admits_discarding` 一处规则(ARCHITECTURE §3.2.1)。`bind_use` 第 471-553 行的 `match current.owner` 是平行实现;改造时把判定改为 `RangeOwner::from(current.owner).admits_discarding(sharing, domain.family, Self::contents_discarded(&current))`,只在 `Err(OtherFamily)` 时走 release/acquire 分支,`Err(Released)` 走既有 acquire 分支。
3. `expected_revision` 的重定基必须以 bind 时复制的初始 cell 为基:若两帧之间导入资源被非图路径(`transfer`、`raw::resource_access`、`pending_acquire`)触碰,其 cell 的 scope/布局/划分会变 → `ImportStateKey` 不等 → 走全量 bind;若只变了 revision/points(例如同一队列又提交了一次只读访问)→ 快路径但 revision 基值来自本帧复制,提交时的修订号校验(B2)仍然成立。
4. `by_resource[r].cells` 与 `shadow.values` 中 `resource == r` 的集合恒等(debug 断言)。
5. 稳态零分配:`by_resource`、`waits_by_batch`、`ImportStateKey.cells`、`patch_bases`、`external_waits` 全部 `clear()` 复用;`tests/cpu_frame_allocations.rs` 的 `allocations == 0` 断言覆盖。

### B1.5 改动清单

| 文件 | 改动 |
|---|---|
| `src/rg/sync_plan.rs` | 新增 `ImportCellKey/OwnerKey/ImportStateKey/BoundTemplate/PatchBase/ExternalWait`;`DependencyRef` 新增 `External { slot: u32, queue: QueueId }`(`pub(crate)`,`BoundWait` 对外仍只暴露 Point/Batch) |
| `src/rg/sync_plan/bind.rs` | `ShadowCell` 加 `initial_slot`;`SynchronizationPlanner` 加 `by_resource/waits_by_batch/order_position/patch_bases/external_waits/cacheable`;重写 `bind_use` 循环、`discard_alias_contents`、`wait`、`later`、`add_alias`、`begin`;新增 `rebase(&mut self, initial: &[InitialState])`;`push_patch` 同步 `patch_bases`;所有权判定改走 `RangeOwner::admits_discarding` |
| `src/rg/vulkan_queue.rs` | `QueueRuntime` 加 `template: [Option<BoundTemplate>; 2]`(按 `GraphKind`)、`resolved: Vec<ResolvedPhysical>`;重写 `bind_queue_resources` 为"收集 → 判定 → 重定基或全量";`prepare_queue_plan` 的 stage 检查移至 miss 路径 |
| `src/rg/plan.rs` | `CompiledPlan` 加 `queue_flags_signature: u64`(在 `compile_graph` miss 路径填) |
| `src/rg/executor.rs` | `ExecutionStats` 加 `bind_ns: u64`、`bind_fast_path: bool`;`compile_graph` 在 miss 时调用 stage 检查 |
| `src/resource/tracked.rs` | 新增 `with_existing_range_state_cells(&self, out: &mut Vec<RangeCell>)`(一次锁,复制到调用者缓冲) |
| `src/resource/range_state.rs` | `impl From<PendingOwner> for RangeOwner` 的等价转换放在 bind.rs;`RangeOwner::admits_discarding` 保持 `pub(crate)` |
| `src/rg/plan_check/bound.rs` | 无语义改动;若 `BoundWait` 新增 External 变体,`View` 需过滤或在重定基后才校验(推荐后者) |

### B1.6 验证

- 保持绿:0.3 节全部;重点 `tests/native_ranges.rs::exclusive_halves_keep_exact_release_pairs_after_failed_acquire_batch_and_recover`、`failed_later_batch_preserves_the_submitted_prefix_and_untouched_image_subresources`(不可缓存路径)、`tests/multi_queue_graph.rs::concurrent_disjoint_writes_do_not_inherit_whole_object_waits`、`tests/transient_aliasing.rs::partial_submit_does_not_publish_an_unreached_alias_initial_layout`、`tests/wave3_queues.rs::a_real_primary_owner_is_released_and_acquired_before_an_auxiliary_copy`、`bind.rs` 内置单元测试(`disjoint_unclaimed_ranges_need_no_cross_queue_or_ownership_wait` 等)。
- 新增单元测试(`bind.rs` `mod tests`):
  - `rebased_binding_equals_full_binding_across_three_frames`:同一 program,三帧模拟(每帧把 initial 的 revision +k、points 换新值),断言快路径与全量 bind 的 `BoundSynchronization` 相等。
  - `import_layout_change_invalidates_template`:第 2 帧改导入布局 → 必须走全量。
  - `released_owner_disables_template`。
  - `by_resource_index_matches_flat_shadow`(debug 断言的测试化)。
- 新增集成测试 `tests/bind_template.rs`:lavapipe 真实会话,temporal 纹理 + 交换链导入 + 池化 Create 资源,连续 30 帧断言 `receipt.stats.bind_fast_path` 自第 3 帧起为 true,且 `barrier_transitions` 逐帧相等。
- 基准:`benches/perf.rs` 的 `bind_ns` 在 160 pass / 3000 use 上应从 O(U·S) 降到 < 100 µs(release,估计值【未核实】);`shadow_cells_visited` 计数器在快路径为 0。

### B1.7 风险

- 模板与真实状态漂移:若某条非图路径修改了导入资源的 cell 但没有改变键里的任何字段(理论上只有 revision/points),重定基仍正确;若未来 `NativeState` 新增字段而没加进 `ImportCellKey`,会静默产生错误屏障。缓解:`ImportCellKey::from(&NativeState)` 用解构模式匹配(`let NativeState { layout, layout_known, writer, readers, owner, revision: _ } = *state;`),新增字段时编译失败。
- `DependencyRef::External` 泄漏到 recorder/`attach_range_patches`:`BoundPatch::resolve` 对 External 返回 `invalid_usage`,并在重定基后 debug 断言 `waits` 中无 External。
- 多队列(真独占族)路径几乎不被 lavapipe 覆盖;保持"不可缓存即全量 bind"的保守策略。

---

## B2 提交时不重放

### B2.1 现状

录制时规划器补丁挂到命令上(`vulkan_queue.rs:375-408` `attach_range_patches` → `command_buffer.rs:1146-1171` `record_state_patch`,每条补丁 `Arc<SyncCell>` clone + `TempList::add`):

```rust
// command_buffer.rs:524-527
struct RecordedStatePatch {
    cell: Arc<crate::resource::SyncCell>,
    patch: crate::resource::range_state::StatePatch,
}
```

提交时(`vulkan.rs:1831-1979` `VulkanGraphBackend::submit`):

```rust
// vulkan.rs:1841-1846  —— 无条件复制全部 PreparedSubmit 到捕获(含堆复制与预算票据)
let mut capture = GpuSubmissionCapture::try_new(queue, prepared, ..., &self.device.memory.stats)?;
...
// vulkan.rs:1874 与 1916 —— 同一组命令校验两次(锁前、锁后)
crate::raw::command_buffer::validate_submission_commands(&command_refs)?;
...
let _reservations = lock_submission_uses_in(&uses, ...)?;
crate::raw::command_buffer::validate_submission_commands(&command_refs)?;
// vulkan.rs:1919-1927 —— 全量重放
let prepared_states = unsafe { prepare_submission_states(&command_refs, &uses, queue_id, family, stats)? };
...
if let Ok(point) = result { prepared_states.publish(point); self.queue_submitted(point); ... }
// vulkan.rs:1974-1977 —— Vec::remove(0)
if self.submissions.len() >= SUBMISSION_CAPTURE_LIMIT { self.submissions.remove(0); }
self.submissions.push(capture);
```

`prepare_submission_states`(`command_buffer.rs:585-715`)对**每个物理 cell** 扫**每条命令的全部补丁**两遍(计数 + 收集),按 `Arc::ptr_eq` 过滤:

```rust
// command_buffer.rs:640-676
for index in 0..prepared.cells.len() {
    let cell = prepared.cells[index].clone();
    let count = commands.iter().try_fold(0usize, |total, command| {
        let explicit = command.state_patches.as_slice().iter().filter(|entry| Arc::ptr_eq(&entry.cell, &cell)).count();
        let opaque   = command.range_uses.as_slice().iter().filter(|entry| Arc::ptr_eq(&entry.cell, &cell)).count();
        ...
    })?;
    scratch.with_entries(|patches| {
        for command in commands {
            for entry in command.state_patches.as_slice().iter().filter(|entry| Arc::ptr_eq(&entry.cell, &cell)) { patches.add(entry.patch)?; }
            ...
        }
        ...
        prepared.prepare_cell(index, patches.as_slice(), family, stats)   // → RangeStateTable::prepare
    })?;
}
```

`RangeStateTable::prepare`(`range_state.rs:443-456`)把全部已提交 cell 复制进 `scratch`,`apply_prepared_patches`(`:479-574`)对每条补丁扫全部 scratch cell、校验 `expected_revision`、`apply_preview`(`:745-885`,含 `RangeOwner::admits_discarding`)、切分;`publish`(`:707-734`)把 scratch 写回并 `coalesce`(`:1141-1163`,循环 `sort_unstable_by_key` 直到不再合并)。这正是规划器在 bind 时已经模拟过一遍的状态机。

### B2.2 复杂度

- 过滤:O(cells × ΣP_cmd) 次 `Arc::ptr_eq`(cells ≈ M,ΣP ≈ P)→ O(M·P) ≈ 200 × 3300 = 6.6×10^5。
- 重放:每 cell O(P_cell × cells_cell) + `coalesce` O(cells log cells) 轮次。
- `GpuSubmissionCapture::try_new`:每次提交复制全部 `PreparedSubmit`(等待/信号/命令句柄数组)+ 预算票据。
- `validate_submission_commands` ×2:O(C × uses)。
- `submissions.remove(0)`:O(64) 移动——小,但无意义。
- 每条补丁 `Arc<SyncCell>` clone/drop(原子 RMW)×P。

### B2.3 目标设计

#### (a) 规划器直接产出最终状态

bind 结束时,每个资源在每个批次末尾的 shadow cell 集合就是该批次提交后 `RangeStateTable` 应当持有的 cell(只差把 `DependencyRef::Batch(b)` 换成真实 `SyncPoint`)。新增:

```rust
// sync_plan.rs
/// 资源 `resource` 在批次 `batch` 成功提交后的全部 cell(只含该批次触碰过的资源)。
pub struct FinalCells {
    pub batch: usize,
    pub resource: u32,
    pub cells: Range<u32>,            // 指向 BoundSynchronization::final_cells 的切片
    /// 该批次开始前这些 cell 的修订号基线(来自 initial),提交前用于一致性校验。
    pub base: Range<u32>,             // 指向 final_bases:(initial_slot, revision)
}
#[derive(Clone, Copy)]
pub struct FinalCell {
    pub range: NativeRange,
    pub layout: vk::ImageLayout,
    pub layout_known: bool,
    pub owner: FinalOwner,            // Unclaimed | Owned(f) | Released{transfer, point: DependencyRef}
    pub writer_scopes: [AccessScope; MAX_QUEUES],
    pub writer_host: AccessScope,
    pub writer_points: [Option<DependencyRef>; MAX_QUEUES],
    pub readers_scopes: [AccessScope; MAX_QUEUES],
    pub readers_host: AccessScope,
    pub readers_points: [Option<DependencyRef>; MAX_QUEUES],
    pub revision_delta: u32,          // 相对 initial[slot].revision
    pub initial_slot: u32,
}
```

生成方式:`bind()` 在每个批次的 use 绑定完后(`bind.rs:285` `push_order(batch)` 之前),对本批次触碰过的资源(用一个 `touched: Vec<u32>` 稳态复用 + `touched_mark: Vec<u32>` 批次戳)把 `by_resource[r].cells` 的 shadow cell 投影为 `FinalCell` 追加到 `final_cells`。模板缓存(B1)下这些也是帧无关的(revision/points 是相对形式)。

#### (b) 提交路径

`attach_range_patches(commands, batch)` 不再逐补丁 `record_state_patch`,改为给**批次最后一条命令**挂一条 `BatchStateHandoff`:

```rust
// command_buffer.rs
pub(crate) struct BatchStateHandoff {
    /// 本批次触碰的物理 cell;与 `FinalCells` 一一对应。
    cells: TempList<Arc<SyncCell>>,            // 仍需持有 Arc 以保证提交期间存活(每资源一次,不是每补丁一次)
    finals: Range<u32>,                        // 指向 planner.bound().final_cells()
    /// 由 planner 解析:Batch(b) → runtime.points[b];External → 初始点。
    template: Arc<SynchronizationPlanner>,     // 已有:命令本来就持有 planner 的 Arc(recorder.synchronization)
}
```

`prepare_submission_states` 分两条路:

```
for cell in uses(物理 cell,去重后):
    if cell 有 handoff(本批次规划器触碰)且命令上没有 range_uses / upload_reads 命中它:
        —— 快路径(图命令)
        table = cell.native_state.lock()      // gate 已由 lock_submission_uses_in 持有
        for (slot, base_revision) in finals.base:
            if table.cells()[slot].revision != base_revision → Err(Timeout)   // 与今天 apply_prepared_patches:510-515 同一错误
        table.stage_final(finals.cells, resolve)  // 把 FinalCell 解析为 RangeCell 写入 scratch,不切分、不排序;
                                                  // DependencyRef::Batch(current) 在 publish 时填入真实 point
        dependencies.merge(规划器已给的 queue_submission_dependencies)      // 不再从表里推导
    else:
        —— 慢路径(transfer/raw/opaque/上传读),保持今天的 prepare 重放不变
```

`publish(point)` 对快路径 cell:把 scratch 里 `DependencyRef::Batch(current)` 的槽位替换为 `point`,`swap(cells, scratch)`,**不 coalesce**(规划器输出已是最小划分;若要保持与慢路径一致的规范形,只在 debug 下 `coalesce` 后 `assert_eq!`)。`Released{point: Batch(current)}` 同理。

`RangeOwner::admits` 的位置:快路径不再在提交时调用 `apply_preview`,准入已在 bind 时经 `RangeOwner::admits_discarding` 判定(B1.4 第 2 条);提交时保留的唯一运行期检查是**修订号基线相等**——它证明"bind 时看到的状态就是现在的状态",所以 bind 时的准入结论仍然有效。导入资源的 `Released` 态只能来自非图路径(`transfer`/`pending_acquire`),它们改变 owner → 键不等 → 全量 bind → 规划器自己 acquire(今天的行为)。

#### (c) Create 资源不再持有运行期状态表

`OriginSource::Create` 的资源(池化或别名集成员)语义上是图内瞬态:内容不跨帧保留,所有访问都在计划里。今天它们也装 `RangeStateTable`(`ensure_range_state_locked`),每帧复制 cell 进 shadow、提交时重放。改为:

- `bind_queue_resources` 对 Create 资源不调用 `with_range_state`;合成初态 = `ensure_range_state_locked` 在 `summary` 非空时的做法(`tracked.rs:252-271`):`layout = UNDEFINED, layout_known = (buffer)`,`writer.points = cell.snapshot()`(每队列 `AtomicU64` 的无锁读取),`writer.scopes[q] = ALL_COMMANDS | MEMORY_READ|WRITE`,`owner = Unclaimed`,`revision = 0`。这正是"丢弃内容 + 等上一帧 GPU 用完"的保守语义,与别名集 `give_back` 后 `discard_idle_range_state` 得到的状态一致。
- 提交时 Create 资源不进 `handoff.cells`;只走 `SyncCell::insert(point)`(整体 last-use,已有)。`SyncCell::native_state` 对这些资源保持 `None`。
- `discard_idle_range_state`(`tracked.rs:338`)对 `None` 表仍返回 `is_reached` 结果,别名集 `give_back` 不受影响。
- 注意图像布局:Create 图像每帧第一次使用都是 `UNDEFINED → layout` 转换(今天池化资源命中时可能保留上一帧的布局从而省掉这次转换)。**这会让 `barrier_transitions` 计数在有池命中的图上略有变化**——这是有意的语义收紧(ARCHITECTURE §4.5 第 4 点:别名切换 `oldLayout = UNDEFINED`),基线门按"B2 前后各记录一次"处理,而不是要求相等。若实测 GPU 侧 UNDEFINED 转换带来可测开销【未核实】,可退回"池命中资源保留表"(只对 `pool_hits` 中的逻辑索引装表)。

#### (d) `GpuSubmissionCapture` 改为按策略捕获

```rust
// vulkan.rs
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SubmissionCapture { Off, Counts, Full }
pub struct GpuSubmissionCapture {
    pub queue: vk::Queue,
    pub result: i32,
    pub batch_count: u32,
    pub command_count: u32,
    /// 仅 `Full` 时填充。
    pub batches: smallvec::SmallVec<[submit::PreparedSubmit; 1]>,
    _memory: Option<CpuMemoryReservation>,
}
impl VulkanGraphBackend { pub fn set_submission_capture(&mut self, mode: SubmissionCapture); }
```

默认:`validation_enabled()` 为 `Full`,否则 `Counts`(不复制数组、不取预算票据)。`submissions: Vec<_>` → `VecDeque<_>`,满时 `pop_front()`。`amend_presentation_submit(&mut capture.batches)`(`vulkan.rs:1847`)今天在捕获副本上修改并提交该副本——`Counts` 模式下需要一个不进捕获的本地 `SmallVec<[PreparedSubmit;1]>` 工作副本(只在有呈现角色时构造,或改 `amend_presentation_submit` 为返回"追加的等待/信号"由 `submit_prepared` 直接消费【未核实:需读 `vulkan_presentation.rs` 的 `amend_presentation_submit`】)。

#### (e) 其他

- `validate_submission_commands` 只保留锁前一次(`vulkan.rs:1874`);锁后第二次(`:1916`)删除——命令由后端独占,两次之间无修改路径【未核实:确认没有 `Send` 给录制线程的命令在此窗口结束录制】。
- 命令上保存 `Arc<SyncCell>`:快路径每资源一次而不是每补丁一次(P → I)。

### B2.4 不变量

1. **发布后的表状态与"独立重放补丁"一致**:`plan_check::check_state_publication(StatePublication{domain, sharing, before, patches, submitted, family, waits, after})`(`plan_check/native_state.rs:282`)。快路径在 `validation_enabled()` 下:`before` = 校验基线时复制的 cell,`patches` = 本批次该资源的 `BoundPatch::resolve` 结果(模板里仍保留 `patches`,只是生产提交不再消费它们),`after` = 安装后的 cell。这是 B2 的核心校验。
2. 修订号基线校验等价于今天的 `expected_revision` 校验:任何非图路径在 bind 与 submit 之间触碰导入资源都会使 revision 变化 → `Timeout`(今天同一错误码,`range_state.rs:510-515`)。
3. `prepared_dependencies`(`range_state.rs:590-598`)今天由重放推导;快路径改由规划器给(`queue_submission_dependencies` 已包含 `BoundWait` 的全部点)。**必须证明两者集合相等**:规划器的 `wait_frontier` 对每个 use 把 `writer/readers` 的全部点作为等待(`bind.rs:571-574`),覆盖了 `NativeState::dependencies` 的集合;`Released.point` 由 acquire 的 `wait(batch, release, …)` 覆盖。debug 下在快路径里同时跑旧推导并 `assert_eq!(StagedDependencies)`。
4. `SyncCell::values[]`(整体 last-use)与表无关,所有资源照旧 `insert(point)`,回收/驱逐/宿主等待不受影响。
5. 慢路径(opaque/upload_reads/非图命令)行为不变,`tests/host_buffer_ranges.rs`、`tests/transfers.rs`、`tests/texture_upload_contract.rs` 覆盖。

### B2.5 改动清单

| 文件 | 改动 |
|---|---|
| `src/rg/sync_plan.rs` | `BoundSynchronization` 加 `final_cells: Storage<FinalCell>`、`finals: Storage<FinalCells>`;访问器 `final_cells_for(batch) -> &[FinalCells]`(用 B3 的按批次切片) |
| `src/rg/sync_plan/bind.rs` | `bind()` 每批次末尾投影 `touched` 资源的 shadow cell 为 `FinalCell`;`begin()` 清理 |
| `src/rg/vulkan_queue.rs` | `attach_range_patches` 改为挂 `BatchStateHandoff`(每资源一个 `Arc<SyncCell>`);`bind_queue_resources` 对 Create 资源合成初态 |
| `src/raw/command_buffer.rs` | 新增 `BatchStateHandoff`、`VulkanCommandBuffer::record_state_handoff`;`prepare_submission_states` 分快/慢路径;`PreparedResourceStates` 记录每 cell 的路径;`publish` 快路径调用 `RangeStateTable::publish_final` |
| `src/resource/range_state.rs` | 新增 `stage_final(&mut self, cells: impl Iterator<Item=RangeCell>, bases: &[(u32,u64)]) -> Result<()>`(基线校验 + 写 scratch,不切分)与 `publish_final(point)`(替换占位点,不 coalesce;debug 下 coalesce 后断言不变);`PreparedCell` 加 `pending_batch_point: bool` 位图或在 `FinalCell` 解析时用 `SyncPoint{queue, value: 0}` 占位并在 publish 替换(`publish` 已 `assert_ne!(point.value, 0)`,占位只在 scratch 内) |
| `src/resource/tracked.rs` | `publish_range_state_final`;`with_range_state` 对 Create 资源不再被调用;新增 `synthetic_discarded_state(&self, queue_scope) -> NativeState`(把 `ensure_range_state_locked:252-271` 的合成逻辑抽成函数,两处共用) |
| `src/rg/vulkan.rs` | `submit`:删第二次 `validate_submission_commands`;`GpuSubmissionCapture` 按 `SubmissionCapture` 策略;`submissions` 改 `VecDeque`;`begin_frame`/`execute_prepared` 的 `submissions.clear()` 不变 |
| `src/device/pending_acquire.rs:468` | 读 `session.backend.submissions.len()`,`VecDeque` 兼容 |
| `tests/*` | 读 `c.batches.len()` 的用例(`p0_perf_baseline.rs:545`、`native_ranges.rs:653/781`)改用 `batch_count` 或在 fixture 里 `set_submission_capture(Full)`;只读 `.len()`/`is_empty()` 的用例不变 |

### B2.6 验证

- 保持绿:`tests/native_ranges.rs` 全部(其中 `failed_later_batch_preserves_the_submitted_prefix_and_untouched_image_subresources` 读 `submissions`)、`tests/transient_aliasing.rs::partial_submit_does_not_publish_an_unreached_alias_initial_layout`、`failed_frames_return_alias_sets_only_when_idle_and_later_frames_discard_stale_state`、`tests/rg_fixes_vulkan.rs` 三个用例、`tests/gpu_frame_allocations.rs`、`tests/p0_stability.rs`、`tests/host_buffer_ranges.rs::generic_reference_cannot_be_overridden_by_an_earlier_or_later_precise_use`、`tests/multi_queue_graph.rs::successful_acquires_survive_later_recording_failure_and_next_frame_recovers`。
- 新增:
  - `range_state.rs` 单元测试 `publish_final_matches_prepare_publish`:随机补丁序列,一边 `prepare`+`publish`,一边把同一规划器的 `FinalCell` 用 `stage_final`+`publish_final`,断言 `cells()` 相等(coalesce 后)。
  - `tests/submit_fast_path.rs`:lavapipe 会话,`set_plan_validation(true)`,30 帧 temporal + 导入 + Create 混合图,断言每帧 `check_state_publication` 通过(通过 `validation_enabled` 自动触发)且 `receipt.stats.submit_ns` 稳定;在帧间用 `TransferManager` 对导入缓冲做一次上传,断言下一帧走全量 bind 且提交成功(revision 基线路径)。
  - `tests/submission_capture_policy.rs`:`Counts` 模式下 `submissions` 条目 `batches.is_empty()` 且 `batch_count` 正确;`Full` 模式与今天一致。
- 基准:`benches/perf.rs` 的 `submit_ns`;`p0_perf_baseline::representative_graph_frame_cost_baseline` 的 `vkQueueSubmit`/`submit batch` 数不变。

### B2.7 风险

- 规划器与状态机的"双实现"变为"单实现 + 校验器":若 `FinalCell` 投影遗漏了状态机的某个副作用(例如 `apply_preview` 对 `Unknown` 的 `layout_known=false`),快路径会产生错的表。缓解:快路径只覆盖 `Access/Release/Acquire` 三种规划器动作,`Unknown` 永远走慢路径;`check_state_publication` 在 CI 的 debug 作业里每帧跑。
- Create 资源 UNDEFINED 起始:对依赖"池命中保留内容"的宿主代码是破坏性的。rhapsody 的契约已经是 Create 资源内容未定义(`tests/undefined_contents.rs`、`inactive_contents.rs`),但 tiny-glade 宿主是否有隐性依赖【未核实】——`reverse_blur_content_*` 测试族可作为回归。
- `amend_presentation_submit` 的工作副本:呈现等待/信号必须进真实提交而不是捕获副本,改造时先读 `vulkan_presentation.rs`。

---

## B3 索引化

### B3.1 现状

```rust
// recorder.rs:34-41 —— 每个 pass / 每个 Initial / 每个 Export 调用一次,每次扫全部 T 条转换
fn phase(&self, phase: BarrierPhase) -> impl Iterator<Item = BoundTransition> + '_ {
    self.synchronization.bound().transitions().iter().copied()
        .filter(move |transition| transition.batch == self.batch && transition.phase == phase)
}
// 调用点:emit(recorder.rs:50)、plan_chunked_batch(:151)、finish(:198)
```

```rust
// vulkan_queue.rs:352-374 record_prelude:  transitions().iter().filter(|t| t.batch == prelude.tag)
// vulkan_queue.rs:388-394 attach_range_patches: patches().iter().filter(|p| p.batch == batch)
// vulkan_queue.rs:418-425 queue_submission_dependencies: waits().iter().filter(|w| w.batch == batch)
```

并行路径 `plan_chunked_batch` 把每 pass 的转换复制进 `planned: Vec<PlannedTransition>` + `ranges`(`executor.rs:33-40 ChunkScratch`)——复制本身 O(T) 可接受,但来源仍是 N 次全扫。

### B3.2 复杂度

- recorder:(N + 2B) 次全扫 → **O(N·T)** ≈ 160 × 3300 = 5.3×10^5 次比较。
- `attach_range_patches`/`queue_submission_dependencies`/`record_prelude`:O(B·(P + W + T))。

### B3.3 目标设计

bind 的输出顺序:`bind()` 按 batch → pass → use 推进,`push_transition` 在 `BeforePass(p)` 阶段连续,但 ownership prelude 会把 `Initial` 阶段的转换插到辅助批次(`bind.rs:512-526`),acquire 把转换放回当前批次当前阶段;Export 在最后一个批次末尾。因此**不能假设已分组**,在 bind 末尾做一次稳定计数排序生成索引:

```rust
// sync_plan.rs —— BoundSynchronization 新增(都是 Storage<u32>/Storage<Range<u32>>,容量复用)
pub(crate) struct PhaseIndex {
    /// 转换下标的置换:按 (order_position(batch), phase_rank, 原始下标) 稳定排序。
    transition_order: Storage<u32>,
    /// 每个 (batch, phase) 槽在 transition_order 里的区间。槽编号:
    ///   batch b 的 Initial  → slot_base[b] + 0
    ///   batch b 的 BeforePass(p) → slot_base[b] + 1 + (p - batches[b].passes.start)   (Graph 批次)
    ///   batch b 的 Export   → slot_base[b] + 1 + passes.len()
    /// OwnershipPrelude 批次只有 Initial 一个槽。
    transition_slots: Storage<Range<u32>>,
    slot_base: Storage<u32>,
    patches_by_batch: Storage<Range<u32>>,   // patches 本来就按批次推进?不保证(prelude 插入)→ 同样做置换
    patch_order: Storage<u32>,
    waits_by_batch: Storage<Range<u32>>,     // 直接复用 B1 的 waits_by_batch(SmallVec)或置换
    finals_by_batch: Storage<Range<u32>>,    // B2
}
impl BoundSynchronization {
    pub fn transitions_for(&self, batch: usize, phase: BarrierPhase) -> impl Iterator<Item = &BoundTransition> + '_;
    pub fn patches_for(&self, batch: usize) -> impl Iterator<Item = &BoundPatch> + '_;
    pub fn waits_for(&self, batch: usize) -> &[BoundWait];
}
```

构建(`SynchronizationPlanner::finish_index()`,在 `bind()` 末尾 / 重定基后调用):

1. `slot_base`:遍历 `batches`,Graph 批次占 `passes.len() + 2` 个槽,Prelude 占 1 个;总槽数 ≤ N + 2B。
2. 计数:`counts[slot] += 1` 遍历 T 条转换(`slot_of(transition)` 是 O(1) 算术);前缀和得 `transition_slots`;第二遍把下标写入 `transition_order`。O(T + N + B),稳定(按原始下标递增),因此**同一阶段内的转换顺序与今天 `filter` 产出的顺序完全一致**(屏障合并到同一 `vkCmdPipelineBarrier` 的顺序不影响语义,但保持一致可让 `barrier_transitions` 逐帧相等的门严格成立)。
3. 补丁、waits、finals 同法按 batch 分桶。

模板缓存(B1)命中时索引随模板复用,**不重建**;重定基只改 `expected_revision` 与追加 External 等待(等待索引在追加后对受影响批次做 O(W) 修补,或把 External 等待单独存 `external_waits_by_batch`)。

消费端:

- `Recorder::phase(phase)` → `self.synchronization.bound().transitions_for(self.batch, phase)`;`plan_chunked_batch` 的 `planned.extend(...)` 可以直接 `extend_from_slice` 置换后的切片(需要 `transition_order` 对应的连续数组:构建索引时顺便产出 `transitions_sorted: Storage<BoundTransition>`,即把转换按槽物理重排一次 O(T),此后所有消费都是切片,`planned` 复制可删除,`ChunkScratch.planned/ranges` 可改为借用切片范围)。
- `record_prelude` → `transitions_for(prelude.tag, Initial)`。
- `attach_range_patches` → `patches_for(batch)`(B2 后改为 `finals_for(batch)`)。
- `queue_submission_dependencies` → `waits_for(batch)`。

### B3.4 不变量

1. 对任意 `(batch, phase)`,`transitions_for` 产出的多重集合与 `transitions().iter().filter(...)` 相同,且顺序相同(稳定排序)。debug 断言 + 单元测试。
2. 每条转换恰好属于一个槽;`Σ slots.len() == T`。
3. `check_bound_synchronization` 不关心索引,但它读的 `transitions()` 若改为物理重排后的数组仍是同一多重集合,检查器按 `(batch, phase)` 自己分组【未核实:`bound.rs` 的 `check` 是否依赖输入顺序;若依赖,保留原序数组给检查器,索引只给 recorder】。

### B3.5 改动清单

| 文件 | 改动 |
|---|---|
| `src/rg/sync_plan.rs` | `BoundSynchronization` 加 `PhaseIndex`;新增 `transitions_for/patches_for/waits_for/finals_for`;`slot_of()` |
| `src/rg/sync_plan/bind.rs` | `finish_index()`;`begin()` 清理;`bind()` 末尾与 `rebase()` 末尾调用 |
| `src/rg/recorder.rs` | `phase()` 改切片;`plan_chunked_batch` 直接用切片范围(`ranges` 记录全局区间,`planned` 删除或保留为空) |
| `src/rg/executor.rs` | `ChunkScratch.planned` 删除;`RecordAction::Transition { transition }` 保持按值传递(`BoundTransition` 是 Copy,80 字节左右) |
| `src/rg/vulkan_queue.rs` | 三处 `filter` 改切片 |

### B3.6 验证

- 保持绿:`tests/parallel_chunks.rs`(5 个用例,`chunks_always_partition_the_batch_in_order` 等)、`tests/wave3_parallel_graph.rs`、`tests/native_ranges.rs::buffer_barriers_follow_disjoint_and_overlapping_bytes_on_serial_and_worker_recorders`。
- 新增单元测试(`bind.rs`):`phase_index_matches_linear_filter`——随机 program(复用 `plan_checker.rs:500` 的 `random` 生成器思路),对每个 (batch, phase) 比较两种取法的 `Vec<BoundTransition>`。
- 基准:`record_ns` 在 160 pass 上的下降;`p0_perf_baseline` 的 `barrier_commands`/`barrier_transitions` 逐帧相等。

### B3.7 风险

- 低。唯一的陷阱是 Prelude 批次插入导致 `slot_base` 与 `order` 不一致;槽按 `batches` 下标而不是执行顺序编号即可避免。

---

## B4 去字符串

### B4.1 现状

```rust
// graph.rs:53-59
pub struct ResourceDeclaration {
    pub sharing: crate::resource::ResourceSharing,
    pub name: String,                 // 每帧 RenderGraph::new() 后重新分配
    pub descriptor: Descriptor,
    pub source: OriginSource,
}
// graph.rs:333-347  add_pass(name: impl Into<Arc<str>>)  → PassDeclaration.name: Arc<str>
// plan.rs:22-28     ResourceDefinition { name: String, .. }   (进 GraphDefinition,是 Hash/Eq 键的一部分)
// plan.rs:164-166   matches_declaration: cached.name != frame.name   (String 比较 ×M,每帧)
// vulkan.rs:320     specifications: BTreeMap<String, ResourceSpec>   (declare 时 insert(name.into()),vulkan.rs:1040;
//                   create_without_initial_data 时 get(name),vulkan.rs:1372-1376;begin_frame 时 clear,vulkan.rs:2446)
// vulkan.rs:2330/2338   format!("{name}-{}", graph.declaration.resources.len())  帧导入名
// vulkan.rs:2678/2739   format!("slot/{}:{name}", name.len())                  named 键
// vulkan.rs:2774-2776   format!("pair/{}:{name}", ..) + format!("{key}:0") ×3  temporal 键
// vulkan.rs:3491-3511   NamedResources.specifications: BTreeMap<String, ResourceSpec>, zero_initialized: BTreeSet<String>
// transient_resources.rs:599-612   LentAliases.resources: BTreeMap<String, (GpuResource, AliasOrigin)>,键 = graph.resources[idx].name.clone()
// vulkan.rs:1364-1367   lent.resources.remove(name)
// graph.rs:686          backend.create_without_initial_data(descriptor, &declaration.name)
// executor.rs:1987      PassPgoNameId(Arc<str>)  → HashMap 键(SipHash 字符串)
// graph.rs:759          SnapshotStatic.pass_order: Vec<String>
```

`ustr`:工作区 `Cargo.toml:119` 已有 `ustr = "=1.0.0"`,`Cargo.lock` 已解析,但 **`crates/rhapsody/Cargo.toml` 的 `[dependencies]` 没有 `ustr`**,需加 `ustr.workspace = true`。`smallvec = "=1.13.2"` 已是依赖。

### B4.2 复杂度

- 每帧:M 次 `String` 分配(声明)+ M 次 `String` 比较(缓存查找)+ M 次 `BTreeMap<String>` 查找(规格、别名)+ (named + temporal×4) 次 `format!` + 每 pass 一次 `Arc<str>` 构造(宿主侧 `name.into()`,若传 `&str` 则分配)。
- 稳态堆分配:这些 `String`/`format!` 是 `cpu_frame_allocations` 的 `allocations == 0` 门在真实会话中**无法达成**的主要来源之一【未核实:该测试用的 `fixed_graph` 可能已用静态名】。

### B4.3 目标设计

```rust
// src/resource/name.rs(新)—— ARCHITECTURE §3.5 的 ResourceName 落地
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceName(pub ustr::Ustr);          // Eq/Hash 为指针;Ord 为指针序(只用于确定性遍历,不用于显示排序)
impl ResourceName {
    pub fn new(name: &str) -> Self { Self(ustr::ustr(name)) }
    pub fn as_str(&self) -> &'static str { self.0.as_str() }
}
impl From<&str> for ResourceName { .. }
impl std::fmt::Display / Debug { 委托 }

/// named/temporal 槽位键,替代 format!("slot/..")/("pair/..:0/1")。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamedKey { pub name: ResourceName, pub kind: NamedKind, pub parity: u8 }
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedKind { Slot, Pair }

/// 帧导入名,替代 format!("{name}-{idx}")。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImportName { pub base: ResourceName, pub ordinal: u32 }
```

落点:

1. `ResourceDeclaration.name: ResourceName`、`ResourceDefinition.name: ResourceName`、`PassDeclaration.name: ResourceName`(`add_pass(name: impl Into<ResourceName>)`;`&str` 与 `Arc<str>` 都提供 `From`)。`matches_declaration` 的名字比较变成 8 字节比较。`GraphDefinition` 的 `Hash` 变成指针哈希(B5 直接受益)。
2. `specifications: BTreeMap<String, ResourceSpec>` → **按逻辑索引的 `Vec<Option<ResourceSpec>>`**:`declare()` 已拿到 `handle.raw()`,`set_declared_sharing(built, name, ..)` 改为按 `ResourceHandle`(保留按名字的兼容封装:线性查 `declaration.resources` 的 `ResourceName` 相等)。`create_without_initial_data(descriptor, name)` 改为 `(descriptor, logical: u32, name: ResourceName)`(名字只用于调试名)。`begin_frame` 的 `clear()` 保留容量。
3. `LentAliases.resources` → `Vec<Option<(GpuResource, AliasOrigin)>>`,下标 = 逻辑索引(`lend()` 里已有 `*index`)。`build_key` 不变(已是索引结构);为 `sets.iter().position(|set| set.lent == 0 && set.key == self.key)` 加 64 位 `key_hash` 预过滤(`KeyEntry` 全部 Copy 字段,FxHash O(entries))。任务书提到的"排序 `SmallVec<[u32; 8]>` 资源 id 别名键"对应的是 `key.entries[*].index` 序列——它已经按 `index` 升序构建(`build_key` 按 `graph.resources` 顺序推进),无需再排。
4. `NamedResources.specifications/zero_initialized` → `HashMap<NamedKey, ResourceSpec>`(FxHash)/`HashSet<NamedKey>`;`TemporalState::get_or_create_owned(graph, key: &str, ..)`/`get_or_create_owned_pair(graph, key: &str, ..)`(`temporal.rs`【未核实签名】)改为 `NamedKey`。`temporal_texture_with_sharing` 的三次 `format!` 消失;`ensure_capacity(&format!("{key}:0"), 2)` 改为 `ensure_capacity(NamedKey{..parity:0}, 2)`。
5. 帧导入:`import_owned_with_sharing(format!(..), ..)` → `import_owned_with_sharing(ImportName{base, ordinal}, ..)`;`ResourceDeclaration.name` 对导入存 `ResourceName::new_import(base, ordinal)`——为避免每帧 intern 新字符串,`ResourceDeclaration` 改为 `name: ResourceName, ordinal: u32`(Create 资源 ordinal=0),`Display` 时拼接。`matches_declaration` 比较 `(name, ordinal)`。
6. `PassPgoNameId(Arc<str>)` → `PassPgoNameId(ResourceName)`(B7 的 PGO 改造后 HashMap 本身也去掉)。`SnapshotStatic.pass_order: Vec<ResourceName>`(公开类型变化:`tests/` 中读 `pass_order` 的用例按 `as_str()` 适配【未核实有哪些】)。
7. 调试名(`VK_EXT_debug_utils`)用 `name.as_str()`,零拷贝。

`ustr` 的全局 interner 是只增不减的:名字集合在 tiny-glade 里是有限的(109 个 pass 名、固定资源名、named/temporal 名),每帧不新增;**禁止**把帧号、指针、随机数拼进名字(lint:`ResourceName::new` 加 debug 断言拒绝包含 `0x`/纯数字后缀?不做,靠评审)。

### B4.4 不变量

1. 名字语义不变:`DuplicateResourceName`(`vulkan.rs:1030`)、`named_slots_validate_scope_initialization_resize_and_reset` 的按名查找/重置行为不变。
2. `GraphDefinition` 的 `Eq` 仍是"规范化声明相等":两个不同字符串永远得到不同 `Ustr`;同一字符串得到同一 `Ustr`。`PlanKey` 的 `Hash/Eq` 自动继承。
3. `plan_check::check_plan` 对 `definition != &plan.definition` 的检查(`plan_check.rs:376`)仍然成立。

### B4.5 改动清单

| 文件 | 改动 |
|---|---|
| `crates/rhapsody/Cargo.toml` | `ustr.workspace = true` |
| `src/resource/name.rs`(新)、`src/resource/mod.rs` | `ResourceName/NamedKey/NamedKind/ImportName` |
| `src/rg/graph.rs` | `ResourceDeclaration.name/ordinal`、`PassDeclaration.name`、`add_pass` 签名、`create_raw_resource(name: impl Into<ResourceName>)`、`import_*_with_sharing`、`SnapshotStatic.pass_order`、`create_without_initial_data` 调用 |
| `src/rg/plan.rs` | `ResourceDefinition.name/ordinal`、`matches_declaration`、`estimated_bytes` 去掉 `name.len()` |
| `src/rg/vulkan.rs` | `specifications: Vec<Option<ResourceSpec>>`;`declare/set_declared_sharing/create_without_initial_data/validate_graph`;`NamedResources` 键类型;`named_*`/`temporal_*`/`request_named` 去 `format!`;帧导入 `ImportName` |
| `src/rg/temporal.rs`、`src/rg/frame_imports.rs` | 键类型 `NamedKey`【未核实内部结构】 |
| `src/rg/transient_resources.rs` | `LentAliases.resources: Vec<Option<..>>`;`AliasKey` 加 `hash: u64` |
| `src/rg/executor.rs` | `PassPgoNameId(ResourceName)`;`pass_names: &[&Arc<str>]` → `&[ResourceName]` |
| `src/rg/subgraph.rs` | `SubgraphDeclaration` 若含名字字符串一并替换【未核实】 |
| 宿主 `crates/tiny-glade` | 调用面 `add_pass("..")`/`declare(.., "name", ..)` 的 `&str` 入参保持可用(`From<&str>`),无需改动;若有 `String` 入参需 `.as_str()` |

### B4.6 验证

- 保持绿:`tests/rg_fixes_vulkan.rs::named_slots_validate_scope_initialization_resize_and_reset`、`tests/temporal_reset_retirement.rs`、`tests/frame_imports.rs`、`tests/import_contract.rs`、`tests/subgraph_plan.rs::normalized_keys_reuse_shifted_parent_indices_names_and_version_bases`、`tests/graph_cache_eviction.rs`、`tests/transient_aliasing.rs`。
- 新增:`name.rs` 单元测试(同串同指针、不同串不同指针、`ImportName` 显示格式与旧 `format!` 一致以便日志可比);`tests/cpu_frame_allocations.rs` 增加一个用 `named_texture`/`temporal_texture` 风格名字的变体(CPU 后端无 named API,则在 `tests/gpu_frame_allocations.rs` 的真实会话里断言稳态 `allocations` 减少到 B4 前记录值以下,并把目标写成 0)。
- 基准:`render_perf_throughput` 的 `allocations`、`build_ns`。

### B4.7 风险

- 公开 API 变更(`ResourceDeclaration.name`、`pass_order`、`PassPgoNameId`):工作区内其他 crate(`tiny-glade`、`country-core`)若直接读这些字段需同步改;`grep -rn "\.pass_order\|declaration\.resources\[.*\]\.name" crates/` 先清点【未核实】。
- `Ustr` 的 `Ord` 是指针序:任何依赖名字字典序的输出(诊断、测试快照)会变;`SnapshotStatic.pass_order` 的顺序是 retained 顺序而非字典序,不受影响。

---

## B5 计划缓存查找

### B5.1 现状

```rust
// executor.rs:17   plan_cache: super::cache::BoundedCache<PlanKey, ()>   (128 项 / 16 MiB,executor.rs:1559)
// executor.rs:44-56
struct PlanKey(Arc<CompiledPlan>);
impl PartialEq for PlanKey { fn eq(&self, o: &Self) -> bool { self.0.definition == o.0.definition } }
impl Hash for PlanKey { fn hash<H>(&self, s: &mut H) { self.0.definition.hash(s) } }   // 整个 GraphDefinition 走 SipHash

// cache.rs:176-194 —— LRU 从新到旧线性调用闭包
pub fn try_get_matching<E>(&mut self, mut matches: impl FnMut(&K) -> Result<bool, E>) -> Result<Option<&V>, E> {
    for index in (0..self.order.len()).rev() {
        if matches(&self.order[index])? { let key = self.order.remove(index)...; self.order.push_back(key); ... }
    }
    ...
}
// executor.rs:865-879 —— 每个候选都跑 matches_declaration(plan.rs:141-209):
//   逐资源比较 sharing/name/descriptor/origin,逐 pass 逐 access 比较,逐语义码查 dependency.native_semantics,
//   最后 graph.validate() + validate_matching_initialization(...)   —— 这两项是契约检查,命中也必须跑一次
// executor.rs:892-900 —— miss 时对 128 项逐个 differing_key_reason(只为统计 CacheReason)
// plan.rs:679-683   compile_definition: check_definition(&definition) 每次 miss 都跑(非 validation 门控)
```

主图与展示图交替使用同一个 `ExecutionChain`(`VulkanGraphSession.chain`):主图的计划位于 LRU 倒数第二,因此每帧主图查找要先对展示图计划跑一次完整 `matches_declaration`(失败于 `passes.len()` 不等,早退,O(1))——目前代价不大;真正的代价是 (1) `matches_declaration` 内部对 `access_semantics` 的每个语义码调用 `dependency.native_semantics`(B4 前还有 M 次 String 比较),(2) miss 路径的 128 次 `differing_key_reason`,(3) `check_definition` 每 miss。

### B5.2 复杂度

- 命中:O(深度 × 早退) + 一次完整 O(M + N + U + |semantics|)。
- miss:O(128 × (M + N)) 分类 + `check_definition` O(定义规模 × 校验器内部) + 编译。

### B5.3 目标设计

```rust
// plan.rs
/// 声明的 64 位结构指纹:FxHash 风格混合(不是 SipHash),O(M + N + U)。
/// 覆盖 matches_declaration 比较的全部字段(context、batch_per_pass、gated、subgraphs、exports、
/// 资源 (sharing,name,descriptor,origin)、pass (name,queue_class,side_effect,accesses 去 version)、语义表)。
pub fn declaration_fingerprint(graph: &GraphDeclaration, context: PlanContext, semantics: &[AccessSemantics]) -> u64;

// executor.rs
struct PlanKey { plan: Arc<CompiledPlan>, fingerprint: u64 }   // Hash = fingerprint;Eq = definition 相等
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GraphKind { Main = 0, Presentation = 1 }
pub struct ExecutionChain {
    plan_cache: BoundedCache<PlanKey, ()>,
    /// 按指纹分桶:fingerprint → 该桶内的键下标;桶内通常 1 项。
    plan_buckets: HashMap<u64, SmallVec<[Arc<CompiledPlan>; 1]>, FxBuildHasher>,
    /// 每图种类的最近命中,O(1) 快路径。
    last_hit: [Option<(u64, Arc<CompiledPlan>)>; 2],
    ...
}
```

查找算法(`compile_graph`):

1. `fp = declaration_fingerprint(&graph, context, dependency)`(用 `dependency.native_semantics(code)` 的结果喂哈希,代替 `matches_declaration` 里对语义表的逐项比较——比较仍要做,但只对一个候选)。
2. `kind = backend.graph_kind()`(`VulkanGraphBackend`:`presentation.is_some()` → Presentation【未核实:展示图在 `compile_frame` 时 `presentation` 是否已设置;若否,由宿主在 `begin_frame_for(GraphKind)` 显式传入,默认 Main】)。
3. 若 `last_hit[kind] == Some((fp, plan))` → `plan.definition.matches_declaration(&graph, dependency, context)?` → true 则命中(`plan_cache.touch`),false 则继续 4。
4. `plan_buckets.get(&fp)` → 对桶内每个 plan 跑 `matches_declaration`,首个 true 命中,写 `last_hit[kind]`。
5. 否则 miss:`GraphDefinition::from_declaration` → 编译 → `insert`(同时写桶与 `last_hit`)。miss 原因分类(`differing_key_reason` ×128)只在 `validation_enabled()` 或 `cache_stats_enabled` 下做;默认只记 `CacheReason::Cold`。
6. `compile_definition` 中的 `check_definition` 保留(它是"公开定义合法性"门,`tests/plan_checker.rs::public_definitions_reject_unresolved_versions_unknown_sources_and_invalid_indices` 依赖);其内部重活(`inspect_definition` 的来源追踪)若实测在 miss 上 > 1 ms【未核实】,拆成"必要合法性"(总跑)与"来源校验"(validation 门控)。

`BoundedCache` 的 LRU `order: VecDeque<K>` 中 `remove(index)` 是 O(深度),深度 ≤ 128,保留。驱逐时同步从 `plan_buckets` 删除(`BoundedCache::insert` 返回被驱逐键的回调,或 `ExecutionChain` 包一层)。

`matches_declaration` 命中时仍执行 `graph.validate()` 与 `validate_matching_initialization`(契约每帧检查,ARCHITECTURE §4.4 "导入初始状态是运行时参数,由绑定阶段重新校验"),不删。

### B5.4 不变量

1. 哈希只定位候选,命中必须经 `matches_declaration` 全量确认(ARCHITECTURE §4.4 明文)。
2. 指纹覆盖的字段 ⊇ `matches_declaration` 比较的字段;否则会出现"指纹不等但定义相等"的伪 miss(只影响性能,不影响正确性)。反向(指纹相等但定义不等)由全量比较兜底。
3. `tests/graph_cache_eviction.rs` 与 `tests/subgraph_plan.rs::local_cache_is_bounded_and_disabled_or_zero_budget_does_not_retain_entries` 的预算/驱逐语义不变。
4. `stats.plan_cache_hit` 语义不变(`cpu_frame_allocations` 断言它)。

### B5.5 改动清单

| 文件 | 改动 |
|---|---|
| `src/rg/plan.rs` | `declaration_fingerprint`;`GraphDefinition` 加 `fingerprint: u64`(`from_declaration` 时计算一次,`PlanKey` 直接取) |
| `src/rg/executor.rs` | `PlanKey` 结构、`plan_buckets`、`last_hit`、`GraphKind`;`compile_graph` 查找顺序;miss 分类门控;`ExecutionBackend` trait 加 `fn graph_kind(&self) -> GraphKind { GraphKind::Main }` |
| `src/rg/cache.rs` | `insert` 返回驱逐键(`Inserted::Stored(Option<(K,V)>)` 已有 `Option<V>`,扩成 `Option<(K,V)>`);`CacheStats` 加 `fast_path_hits` |
| `src/rg/vulkan.rs` | `graph_kind()` 实现;可选 `begin_frame_for(kind)` |
| `tests/cpu_graph_support/*` | CPU 后端 `graph_kind` 默认实现即可 |

### B5.6 验证

- 保持绿:`tests/graph_cache_eviction.rs` 全部、`tests/subgraph_plan.rs` 全部、`tests/cpu_frame_allocations.rs`(`plan_cache_hit` 断言)、`tests/rg_wave3.rs`/`rg_frame_round2.rs` 中涉及 `CacheStats` 的用例【未核实具体名】。
- 新增:`executor.rs` 单元测试 `alternating_main_and_presentation_graphs_hit_last_slot_without_scanning`(两张结构不同的图交替 100 帧,断言 `fast_path_hits == 198`,`matches_declaration` 调用计数 == 帧数);`fingerprint_collision_still_requires_full_match`(人为构造两个定义相同指纹——用测试钩子强制 `fp` 相等——断言不同定义不命中)。
- 基准:`compile_ns`(命中帧)在 160 pass 上 < 50 µs【估计】。

### B5.7 风险

- 低。`GraphKind` 判定若错误只是退化到桶查找。

---

## B6 动态常量页

### B6.1 现状

```rust
// dynamic_constants.rs:22-31
pub fn write_bytes(&self, bytes: &[u8]) -> Result<()> {
    ...
    if self.buffer.is_dynamic_constants_page() {
        self.buffer.write_bytes_deferred_flush(self.offset, bytes)     // buffer.rs:970-986 → host_access_with(buffer.rs:707)
    } else { self.buffer.write_bytes_unsynchronized(self.offset, bytes) }
}
// buffer.rs:755-763 —— 每次写入:
let host = self.host_access.lock();                       // 锁 1
let reservation = self.last_use.lock_submission();        // 锁 2(submission_gate)
let pending = self.last_use.buffer_range_snapshot(...)?;  // 锁 3(native_state),tracked.rs:398-417,扫全部 cell
if pending.is_empty() {
    let result = self.copy_locked(&host_range, coherent, write, flush, copy);  // buffer.rs:800-812:
        // self.last_use.prepare_buffer_host_range(...)   → 锁 3 再取一次,range_state.rs:603-679 prepare_host:
        //   复制全部 cell 进 scratch,对重叠 cell 切分并 revision+1(切出 [offset,offset+len) 这一新 cell)
        // ... 拷贝 ...
        // self.cell.publish_buffer_host_range()           → 锁 3 第三次,publish_host:写回 + coalesce(sort)
```

`range_state.rs:644-652`:写入把该范围的 `writer = {host: HOST_WRITE}`,revision+1 —— 与相邻 cell 的 `state` 不再相等,`coalesce` 不会合并,所以**每次写入永久多一个 cell**,直到页被 `discard_idle_range_state` 重置(`dynamic_constants.rs:237-256` 的注释 241-244 已经承认这一点)。

### B6.2 复杂度

- 第 k 次写入:`buffer_range_snapshot` O(k)、`prepare_host` 复制 O(k) + 切分、`publish_host` 排序 O(k log k)。一页 K 次写入 → **O(K² log K)**,且 3 把互斥量 × 3~4 次加锁。`max_cells = 4096`(`RangeStateConfig::default`)后 `Timeout`。
- 每帧写入次数 = 每 pass 常量块数(≥ N)+ 每 draw 参数块(C2 之前是 O(draw))。

### B6.3 目标设计

**原则**:页在被 `allocate` 选为 `current` 的那一刻已经证明"GPU 对本页的全部既有使用已完成"(`can_append` 要求 `use_cell().snapshot() == page.use_mark && is_reached(&completed)`,`dynamic_constants.rs:214-217`;页重用路径要求 `discard_idle_range_state`,`:245-246`)。此后分配出去的每个 chunk 都是**此前没有任何 GPU 使用的新字节**(cursor 单调),而本帧录制的命令还未提交(提交会改 `snapshot`,使 `can_append` 失败 → 换页)。因此**逐写入的危险检查是恒真的**,可以整页做一次。

```rust
// dynamic_constants.rs
struct Page<B> {
    buffer: Arc<B>,
    cursor: u64,
    allocations: u32,
    use_mark: SyncSet,
    /// 新增:页打开(成为 current)时登记一次宿主写入 [0, capacity) 的 prepare;封存时 publish。
    host_lease: HostLease,                // Open | Sealed
}
```

流程:

1. `allocate()` 把一页设为 `current` 时(新建页 `:267`、重用页 `:245-255`、继续追加不变):调用一次 `page.buffer.open_host_lease()`:取 `submission_gate` + `native_state` 各一次,`prepare_host(Buffer{0, backing_bytes}, write=true)`(整页一个 cell,revision+1 一次),**不 publish**,`prepared_host = true` 保持到封存。期间表处于"已准备"状态——`RangeStateTable` 现有语义要求 `prepare_host` 与 `publish_host` 成对且其间不能 `prepare`(`cancel_prepared` 会清)。为避免与 GPU 提交路径冲突:动态常量页在打开期间**不会**出现在任何已提交命令的 `uses` 里(否则 `can_append` 失败),所以不会有并发 `prepare`;但 `attach_range_patches`/opaque use 在**录制**时只是登记,不碰表,安全。
2. `DynamicConstantsChunk::write_bytes` 对 `is_dynamic_constants_page()` 的缓冲改走新的 `Buffer::write_bytes_leased(offset, bytes)`:只做边界检查 + `copy_nonoverlapping` + 非 coherent 时 `HostFlushQueue::record`(已有,`dynamic_constants.rs:610`)。**零互斥量**(`host_access` 锁也不需要:chunk 之间互不重叠,由 `allocate` 的 cursor 保证;同一 chunk 被两个线程同时写是调用者 bug,与今天 `write_bytes_unsynchronized` 的契约一致)。
3. 封存(seal):页不再是 `current`(`allocate` 换页时)或设备提交前 flush(`HostFlushQueue::flush` 的调用点【未核实位置,应在 `Device::submit` 前】)时,对所有 `Open` 页 `publish_host()` 一次(锁 2 次)→ 表回到单 cell:`[0,cap)` writer.host=HOST_WRITE。提交后 GPU 的 opaque 使用补丁(`PatchAction::Unknown`,来自 `note_opaque_use` 的 `OpaqueUseRange::Bytes`)会把 `[offset,size)` 切出来——为保持"一页一个 cell",`ResourceOwner` 为动态常量页时 `note_opaque_use` 改用 `OpaqueUseRange::Whole`(页内容本就是整页一起生命周期,保守无害)。
4. `can_append` 保持;`page_reuses` 路径的 `discard_idle_range_state` 保持(对单 cell 表 O(1))。

为什么仍保留表而不是完全去掉:图 pass 通过 `into_uniform_buffer_binding`/`into_gpu_lifetime_tracker` 把页作为 `ResourceOwner` 挂到命令上,提交时 `prepare_submission_states` 需要它的表来登记 `Unknown` 使用(慢路径);去掉表需要给 `ResourceOwner` 加"仅整体 last-use"模式,改动面更大,留给 C 层。

### B6.4 不变量

1. 宿主写入发生时,该页上不存在未完成的 GPU 读:由 `can_append`/重用条件保证(今天也是靠它,逐写入检查只是重复证明)。新增 debug 断言:`write_bytes_leased` 时 `debug_assert!(page.host_lease == Open)`。
2. 宿主写对 GPU 的可见性:`vkQueueSubmit` 隐含宿主写可见(Vulkan 规范 7.9 "Host Write Ordering Guarantees");非 coherent 内存的 flush 仍由 `HostFlushQueue` 在提交前统一做——不变。
3. 表状态:封存后恰好 1 个 cell(`tests/dynamic_constants_ring.rs` 新增断言)。
4. `tests/host_buffer_ranges.rs::a_disjoint_host_write_succeeds_while_real_gpu_work_is_blocked_in_both_submit_apis` 针对的是普通宿主可见缓冲(`write_bytes`/`write_bytes_timeout`),路径不变。

### B6.5 改动清单

| 文件 | 改动 |
|---|---|
| `src/raw/dynamic_constants.rs` | `Page` 加 `host_lease`;`allocate` 打开租约;新增 `seal_pages()`;`write_bytes` 走 `write_bytes_leased`;页的 opaque use 改 `Whole` |
| `src/raw/buffer.rs` | 新增 `pub(crate) fn write_bytes_leased(&self, offset, bytes) -> Result<()>`(边界 + copy + flush record);`open_host_lease()/seal_host_lease()` 封装两把锁 |
| `src/resource/tracked.rs` | `prepare_buffer_host_range`/`publish_buffer_host_range` 保持;新增 `host_lease_open(&self, size)` |
| `src/raw/command_buffer.rs` | `note_opaque_use` 对 `is_dynamic_constants_page()` 的 owner 用 `OpaqueUseRange::Whole`(或在 `ResourceOwner` → `RecordedOpaqueUse` 转换处) |
| `src/device/submission.rs` 或 `HostFlushQueue::flush` 调用点 | 提交前 `seal_pages()`【未核实具体文件】 |

### B6.6 验证

- 保持绿:`tests/dynamic_constants_ring.rs::unsubmitted_constant_leases_bound_capacity_and_reuse_one_real_page`、`dropped_cpu_owners_cannot_reset_a_page_until_actual_gpu_copy_completes`;`tests/host_buffer_ranges.rs` 全部;`tests/triangle_draw.rs`/`triangle_animation.rs`(实际使用常量页);`bind.rs:1111-1122` 的 `discard_idle_range_state` 单测。
- 新增:`dynamic_constants_ring.rs::many_writes_keep_one_native_cell_and_take_no_lock`——一页写 2000 次,断言 `resource_native_state(token, |cells| cells.len()) == 1` 且 `allocations == 0`;`page_seal_publishes_before_submit`。
- 基准:`benches/perf.rs` 增加 `constants_write_ns`(每 pass 2 次 128 B 写入 × 160 pass);`render_perf_throughput` 的 `build_ns`。

### B6.7 风险

- 跨线程并行录制写同一页:`allocate` 本身在 `DynamicConstants` 的锁内分配不重叠区间,`write_bytes_leased` 无锁写不同字节——安全;但 `HostFlushQueue::record` 仍有锁(`dynamic_constants.rs:627`),非 coherent 设备上每写一次锁一次,可改为按线程本地聚合【低优先级】。
- 打开租约期间 `RangeStateTable.prepared_host = true`:任何对该页的 GPU 提交 `prepare` 会 `cancel_prepared` 把租约清掉而不报错 → 封存时 `publish_host` 的 `assert!` 触发 panic。按设计不会发生(页为 current 时无提交);为稳妥,`seal` 先检查 `prepared_host`,为 false 则重新 `prepare_host`+`publish_host`。

---

## B7 杂项(每项一段)

**(1) `validate_graph` 跑两遍** — `VulkanGraphSession::compile_frame` 在 `vulkan.rs:2932` 调 `self.backend.validate_graph(&declaration)`,随后 `chain.compile_graph` → `backend.prepare_declaration` 在 `vulkan.rs:1524-1525` 再调一次同一函数(`declaration.validate()` + 逐导入查 `resources` BTreeMap + `validate_import_descriptor`)。删除 `:2932` 的调用(保留 `:1525`,它是 trait 路径、CPU 后端也经过);`result_cache.prepare` 若依赖已校验的声明【未核实】,把顺序调整为 `compile_graph` 内先 `prepare_declaration` 再 `result_cache.prepare`。门:`tests/import_contract.rs`、`tests/result_cache_vulkan.rs`。

**(2) `collect_named_retired` O(retired × pending × M)** — `vulkan.rs:2488-2510` 对每个待退役 token 遍历 `self.pending` 每帧的全部 `physical_mapping`,命中则保留,否则 `clear_pass_resources()`(每个 token 一次!)+ 三个 BTreeMap `remove`。改为:`PendingSessionFrame` 入队时(`end_frame`)把 `physical_mapping` 里的 token 计入 `in_flight: HashMap<u64, u32, FxBuildHasher>` 引用计数,`finish_frame` 出队时递减;`collect_named_retired` 变为 `retired.retain(|t| in_flight.contains_key(t))` O(R),`clear_pass_resources()` 在循环外、仅当确有移除时调用一次。门:`tests/temporal_reset_retirement.rs`、`tests/rg_fixes_vulkan.rs::named_slots_validate_scope_initialization_resize_and_reset`。

**(3) `finish_frame` 的 `BTreeSet`** — `vulkan.rs:3225-3232` 每帧 `collect::<BTreeSet<u64>>()` 全部 Create 资源的 token,随后 `created.contains(&buffer.handle)` 查询;另 `:3240-3246` 对每个物理资源线性扫 `exported_logical_handles`。改为:在 `FrameStorageLease` 的池化表里加 `created_physical: Vec<bool>`(按 physical 索引,`instantiate_with_storage` 填)与 `exported_physical: Vec<bool>`;`finish_frame` 直接按下标查。零分配(表随 `FrameStoragePool` 复用)。门:`tests/transient_aliasing.rs`、`tests/inactive_contents.rs`。

**(4) PGO 的 `Instant` + SipHash** — 串行路径 `executor.rs:1339/1353` 每 pass `Instant::now()` ×2,`:1394-1405` 每 pass `PassPgoNameId::from_shared(Arc::clone(name))` + `PassPgoStats::record`(`:2066-2071`,两个 `std::collections::HashMap<PassPgoNameId,_>` 的 SipHash 插入 + Arc clone);并行路径 `:1847` 同样;`RecordScope::new`(`:1332`)在 trace 关闭时是只读 TLS,可忽略。改为:`PassPgoFrames` 存 `Vec<u64>`(按 scheduled pass 索引)+ `Weak<CompiledPlan>` 身份,`parallel.rs` 的分块规划按索引读上一帧耗时(名字查询只在 `PassPgoStats::iter()` 诊断时经 `plan.snapshot.pass_order` 映射);计时只在 `RecordingPolicy::Chunked` 且 `max_chunks > 1` 且该批次 pass 数 ≥ 2 时打开(`ExecutionChain.parallel` 已有配置),`Serial`/`PerBatch` 策略下不取时间。同时修 ANALYSIS A6 指出的"主图读到展示图统计"——按 `GraphKind`(B5)分槽。门:`tests/parallel_chunks.rs::pgo_double_buffer_uses_only_the_previous_frame_and_never_accumulates`、`stats_extend_overwrites_per_name_but_accumulates_total_and_max`(接口改为索引后需改写为等价断言)。

**(5) `refresh_memory_budget` 每帧** — `device/lifetime.rs:63-67` `try_collect_garbage` 每次调 `memory.rs:150-162` 的 `vkGetPhysicalDeviceMemoryProperties2`(驱动调用,集显上数十微秒【未核实】)。改为 `Memory` 加 `budget_refreshed_at: AtomicU64`(帧计数或 `Instant` 毫秒),间隔 ≥ 32 次 GC tick 或 250 ms 才查询;`budget_action() != Normal`、分配失败(`OutOfDeviceMemory`)、`evict_idle_resources_under_pressure` 入口强制刷新。门:`tests/device_budget.rs`、`tests/budget_eviction.rs`、`tests/budget_snapshot.rs`。

**(6) 时间线 `poll` 频率** — `sync/timeline.rs:278-294` 的 `poll` 对每个"已提交 > 已完成"的队列调一次 `vkGetSemaphoreCounterValue`;rg 内每帧调用点 ≥ 7 处(`vulkan.rs:2405/2422/2527/3150/3224`、`transient_resources.rs:578`、`attachments.rs:525/552`、`profiling.rs:372`)。加 `Timelines::poll_epoch(epoch: u64)`:`polled_epoch` 原子 == epoch 时直接返回 `completed()` 快照;`VulkanGraphSession::begin_frame` 把 `frame_epoch += 1` 并传入;阻塞 `wait` 返回后同样刷新快照。需要"刚刚完成"精度的调用(`wait_frame` 之后)用无 epoch 的 `poll`。门:`tests/sync_timeline_vulkan.rs`、`tests/command_retention.rs`。

**(7) `vkUpdateDescriptorSets` 每 binding 一次** — `raw/binding.rs:58-93` 每个 `DescriptorSetBinding::write` 一次驱动调用 + `SeqCst fetch_add`。加 `DescriptorWriteBatch { writes: SmallVec<[vk::WriteDescriptorSet<'a>; 16]>, buffer_infos/image_infos/texel_views: SmallVec<..> }`,`descriptor_set.rs` 的 set 构建路径先收集再一次 `update_descriptor_sets`;计数器改 `Relaxed`。bindless 路径(`bindless.rs`)已是批量【未核实】。门:`tests/raw_core_descriptors_shader.rs`、`tests/wave2_samplers_binding.rs`、`tests/descriptor_pool_ring.rs`。

**(8) 附件视图缓存线性查找** — `rg/attachments.rs:418-428` 对每个附件在 `self.views: Vec<(ViewKey, Arc<TextureView>)>`(上限 256)里 `iter().find`;每个光栅 pass 最多 9 个附件 → 2304 次 `ViewKey` 比较/pass。改为 `HashMap<ViewKey, Arc<TextureView>, FxBuildHasher>`(`ViewKey` 已 `Eq`,加 `Hash`;它含 `Arc<Texture>` 指针与 `ViewDesc`,按指针哈希);`reserve_views`/驱逐(`:505-`)改为按 `Arc::strong_count` 扫哈希表值,频率不变。门:`tests/attachments.rs`、`tests/managed_view_cache.rs`、`tests/separate_depth_stencil.rs`。

**(9) 管线表全局锁 + 线性扫描** — `raw/shader/pipeline_cache.rs:76-86` `slot_for` 持全局 `Mutex<TempList<PipelineCacheEntry<K>>>` 并 `iter().find(|e| matches(&e.key))`。改为 `RwLock<PipelineTable<K>>`,表内 `index: HashMap<u64 /*key hash*/, SmallVec<[u32; 1]>>` → 条目下标;命中路径只取读锁 + 哈希 + 一次全量键比较;miss 时升级写锁走现有驱逐逻辑。键哈希由 `pipeline_key.rs` 的 `PipelineKey` 派生(`Hash`)。另:按 ANALYSIS §6 确认生产启动加载 `PersistentPipelineCache`(`pipeline.rs:789`)【未核实】。门:`tests/pipeline_cache.rs`、`tests/async_pipeline.rs`。

**(10) 每帧投影重建** — `rg/pass_bindings/projection.rs:269-315` 每帧对全部物理资源 `resolve(token)`(BTreeMap + `GpuResource` Arc clone)、构 `Resolved` 行、`sort_unstable_by_key(identity)` O(M log M),再 `:329-380` 二分旧索引决定复用。改为:(a) `checkout` 先比较本帧 `physical_mapping[*].token` 序列与上一帧保存的 `last_tokens: Vec<u64>`(O(M) 整数比较),相等则整表复用,只 `std::mem::swap` 回去,零 Arc 操作;(b) 不等时只对变化的物理槽重解析,`index` 用 `HashMap<PhysicalIdentity, PhysicalIndex, Fx>` 替代排序 + 二分;(c) `resolve` 返回 `&GpuResource` 借用,`Arc` 只在真正新建 `ManagedRow` 时 clone。随 B1 的 `QueueRuntime.resolved` 共用一次 token → 资源解析。门:`tests/managed_projection.rs` 三个用例、`tests/pass_bindings.rs`、`tests/managed.rs`。

**(11) 其他已在 ANALYSIS §6 列出的小项**(不在本规范展开):`vulkan.rs:1794-1830` `transition()` 每条转换 `retain_recorded(Arc clone)` + `barriers.entry(command)` BTreeMap——按资源去重后每命令每资源一次(B2 的 `BatchStateHandoff.cells` 可直接复用为 owner 列表);`frame_pools/command_slot/commands/resources/import_families/history_validity` 等 `BTreeMap<u64, _>` 热路径查表换 `HashMap<_, _, FxBuildHasher>` 或按 token 低位的 slab。

---

## 附录 A:改动总表(按文件)

| 文件 | B1 | B2 | B3 | B4 | B5 | B6 | B7 |
|---|---|---|---|---|---|---|---|
| `Cargo.toml` | | | | ustr | | | |
| `src/resource/name.rs`(新) | | | | ● | | | |
| `src/resource/range_state.rs` | admits 调用 | stage_final/publish_final | | | | | |
| `src/resource/tracked.rs` | cells 复制 | synthetic state | | | | lease | |
| `src/rg/sync_plan.rs` | Key/Template | FinalCell | PhaseIndex | | | | |
| `src/rg/sync_plan/bind.rs` | ● | finals | finish_index | | | | |
| `src/rg/vulkan_queue.rs` | ● | handoff | 切片 | | | | resolved |
| `src/rg/recorder.rs` | | | ● | | | | |
| `src/rg/executor.rs` | stats | | ChunkScratch | PGO 名 | ● | | PGO |
| `src/rg/plan.rs` | 签名 | | | name | fingerprint | | |
| `src/rg/cache.rs` | | | | | 驱逐回调 | | |
| `src/rg/graph.rs` | | | | ● | | | created 表 |
| `src/rg/vulkan.rs` | | submit/capture | | spec/named | graph_kind | | (1)(2)(3) |
| `src/rg/transient_resources.rs` | | | | Vec 键 | | | |
| `src/rg/temporal.rs`、`frame_imports.rs` | | | | NamedKey | | | |
| `src/raw/command_buffer.rs` | | ● | | | | Whole | |
| `src/raw/dynamic_constants.rs`、`src/raw/buffer.rs` | | | | | | ● | |
| `src/device/lifetime.rs`、`memory.rs` | | | | | | | (5) |
| `src/sync/timeline.rs` | | | | | | | (6) |
| `src/raw/binding.rs`、`descriptor_set.rs` | | | | | | | (7) |
| `src/rg/attachments.rs` | | | | | | | (8) |
| `src/raw/shader/pipeline_cache.rs` | | | | | | | (9) |
| `src/rg/pass_bindings/projection.rs` | | | | | | | (10) |
| `benches/perf.rs`(新) | 基准 | 基准 | 基准 | | | 基准 | |

## 附录 B:风险矩阵

| 风险 | 影响 | 概率 | 缓解 |
|---|---|---|---|
| B1 模板重定基遗漏状态字段 | 错误屏障(静默) | 中 | `ImportCellKey` 解构构造;`check_bound_synchronization` 接入生产 debug 路径;`RHAPSODY_BIND_SHADOW` 双跑断言 |
| B2 快路径与状态机语义漂移 | 错误的运行期状态 → 后续非图路径屏障错误 | 中 | 只覆盖 Access/Release/Acquire;`check_state_publication` 每帧(debug);慢路径保留 |
| B2 Create 资源 UNDEFINED 起始 | 池命中资源多一次布局转换;宿主隐性依赖内容 | 低~中 | `reverse_blur_content_*` 回归;可退回"pool_hits 保留表" |
| B2 `GpuSubmissionCapture` 策略 | 依赖 `batches` 内容的测试失败 | 高(可预期) | 列出的 3 处测试改 `batch_count`/`Full` |
| B4 公开类型变化 | 宿主 crate 编译失败 | 高(可预期) | 先 grep 工作区;提供 `From<&str>` |
| B5 `GraphKind` 判定错 | 退化为桶查找 | 低 | 无 |
| B6 租约期间被 `cancel_prepared` | `publish_host` 断言 panic | 低 | seal 前检查并重新 prepare |
| 基准缺失 | 无法量化收益 | 高(当前事实) | 第一步先建 `benches/perf.rs` 并记录 B 层前数字 |
