# A 层实施规格：宿主侧止血（A1–A7）

2026-10-21。只读分析，未构建、未运行、未修改任何源码。前置阅读：`ANALYSIS.md`（同目录）与 `reports/logo-startup-repair-2026-10-21/render-cpu-hotpath-audit.md`。本文是给工程师照做的实施单：每项给出现状代码位置与引文、成本来源、目标改法（Rust 草图）、影响面、验证口径、风险与回退。行号以 2026-10-21 工作树为准（`git HEAD f98b40229`），符号名优先于行号。

通用约定：

- `TG_FRAME_TIMING=1` 的 `render_stages` 日志（`crates/tiny-glade/src/production_renderer.rs:1251-1254`）字段：`draw_lists_ms / declare_ms / prewarm_ms / compile_ms / record_submit_ms / collect_ms / present_ticket_wait_ms / acquire_ms / resolve_inclusive_ms / present_ms / queue_trim_actual_ms / queue_trim_budget_ms / observed_completion_wall_ms`。每项"验证"段指明哪个字段应当下降、需要新增哪个字段。
- `game_loop.rs:428` 的 `frame n=.. update_ms= render_ms= finish_ms= limiter_ms= total_ms=` 是 App.update 与整帧的口径。
- 每项先做再测，不要跨项合并提交；A1/A2/A4 改变帧内同步语义，必须单独过 lavapipe 多帧回归（`tiny-glade/reports/main-loop-render-recovery-2026-10-19/multiframe/`）。
- 标注【未核实】的内容是本次只读未能逐行证实的，实施前先 grep 确认。

推荐顺序：**A1 → A2 → A3 → A4 → A5 → A6(PGO) → A7 → A6(线程)**。A1–A4 直接消除帧内串行等待与重复扫描，A5/A6(PGO) 是一行量级的并行开关，A6(线程) 收益是响应性而非 FPS，放最后。

---

## A1 帧在飞：主图 + 展示图共用一个在飞槽

### 现状

**节流点**。`crates/rhapsody/src/rg/vulkan.rs:2412-2456` `VulkanGraphSession::begin_frame`：

```rust
// vulkan.rs:2432-2436
// 配置限制在飞帧数量；只等待最旧帧，不等待其他会话或无关队列。
while self.pending.in_flight() >= self.backend.device.frames_in_flight() as usize {
    let frame = self.pending.front().unwrap().frame;
    self.wait_frame(frame, 10_000_000_000)?;
}
```

`self.pending` 是 `super::frame_throttle::FrameThrottle<PendingSessionFrame>`（`vulkan.rs:2042`），内部是一个 `VecDeque<R>`，`in_flight()` 即 `len()`（`frame_throttle.rs:27-29`）。

**计数点**。每次 `execute_prepared` 成功（或部分提交）都 `end_frame` 一次：

```rust
// vulkan.rs:3044-3065
let frame = self.next_frame;
self.next_frame += 1;
...
self.end_frame(PendingSessionFrame { failed: false, frame, execution, retire, started_at: Instant::now() });
```

```rust
// vulkan.rs:3137-3147
fn end_frame(&mut self, pending: PendingSessionFrame) {
    if !pending.execution.transaction.receipts().is_empty() { self.ledger.note_submission(); }
    if let Err(error) = self.ledger.end() { log::warn!("end_frame 账本异常: {error:?}"); }
    self.pending.end_frame(pending);
}
```

**宿主一帧两张图**。`crates/country-core/src/systems/render_loop/render_frame.rs:297-380` `render_inner` 的顺序：`build_draw_lists → compile_world_graph → host.begin_frame → drain_finished_frames → execute_main_graph → acquire_swapchain_image → execute_presentation_graph → end_frame → present → … → refresh_memory_budget`。其中：

- `compile_world_graph` → `compile_full_world_graph`（`production_renderer.rs:510-557`）→ `renderer.bridge.build_frame(..)`（`render_bridge.rs:254-294`）→ `self.session.borrow_mut().begin_frame()`（`:263`）。**注意这一步被计入 `TimedStage::Declare`**（`production_renderer.rs:519-526`），所以当前 `declare_ms` 里混着在飞等待。
- `execute_main_graph`（`production_renderer.rs:1573-1579`）→ `bridge.execute_compiled` → `session.execute_prepared` → `end_frame` 第一次。
- `acquire_swapchain_image`（`production_renderer.rs:1641-1674`）：

```rust
// production_renderer.rs:1645-1661
self.renderer.collect()?;
if self.renderer.pending.len() >= self.renderer.device.frames_in_flight() as usize {
    let started = self.renderer.timing_start();
    let waited = self.renderer.pending.front().expect("nonempty").wait(timeout_ns());
    self.renderer.timing_end(TimedStage::PresentWait, started);
    waited?;
    self.renderer.collect()?;
}
let started = self.renderer.timing_start();
let result = Swapchain::validate_swapchain_and_acquire_image(&mut self.renderer.swapchain);
self.renderer.timing_end(TimedStage::Acquire, started);
```

  这里的 `pending: VecDeque<PresentTicket>`（`:744`）每个显示帧一张票，与 session 的 `pending` 是两套计数。

- `execute_presentation_graph_inner`（`production_renderer.rs:1305-1331`）再次 `bridge.build_frame` → `session.begin_frame()` → **第二次节流** → `import_swapchain_image` → `resolve::build` → `execute_compiled` → `end_frame` 第二次。

**默认 `frames_in_flight = 2`**（`crates/rhapsody/src/device/init.rs:71`）。

**rhapsody 自带的 `FrameLoop`**（`rg/frame_loop.rs:183-462`）生产路径没用，但结构相同：`FrameLoop::begin_frame`（`:287-303`）先按 `PresentTicket` 数节流再调 `graph.begin_frame()`，`render_presentation_inner`（`:472-507`）又调一次 `graph.begin_frame()`。

### 问题

把时间线写出来（`F=2`，单图形队列）：

| 步骤 | session.pending 内容 | 等待 |
|---|---|---|
| 帧 n 声明主图前 `begin_frame` | `[main_{n-1}, pres_{n-1}]`，len=2 | 等 `main_{n-1}` GPU 完成 |
| 执行主图 | `[pres_{n-1}, main_n]` | — |
| 帧 n 声明展示图前 `begin_frame` | len=2 | 等 `pres_{n-1}` GPU 完成（它排在 `main_n` 之前，通常刚好完成） |
| 执行展示图 | `[main_n, pres_n]` | — |

帧 n 的 CPU 声明不能开始，直到帧 n-1 的主图 GPU 跑完；而帧 n 的主图 GPU 要等帧 n 的声明+编译+录制完才提交。CPU 段与 GPU 段几乎没有重叠：**实际在飞 ≈ 1 帧**。原版 `PendingSubmission` 的 `len > 1` 才等（`frame_throttle.rs` 顶部注释就是照原版写的）是按"提交"计数的，但原版一帧只有一次提交。

### 目标改法

两个方案，**推荐 (a)**。

#### 方案 (a)：按"显示帧"计数（推荐）

核心：`PendingSessionFrame` 增加 `display: DisplayFrameId`，节流时数**不同 display id 的个数**而不是队列长度；同一显示帧内的多张图共用一个槽。

**rhapsody 侧**（`crates/rhapsody/src/rg/vulkan.rs`、`frame_throttle.rs`）：

```rust
// rg/vulkan.rs（新增，紧邻 PendingSessionFrame）
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayFrameId(pub u64);

struct PendingSessionFrame {
    failed: bool,
    frame: u64,
    display: DisplayFrameId,          // 新增
    execution: GraphExecution,
    retire: Vec<graph::RetireCallback>,
    started_at: std::time::Instant,
}

pub struct VulkanGraphSession {
    ...
    next_display: u64,                       // 新增，从 1 起
    active_display: Option<DisplayFrameId>,  // 新增：begin_display_frame..end_display_frame 之间
}
```

```rust
// rg/frame_throttle.rs（新增方法；pending 按提交顺序，所以同 display 连续）
impl<R> FrameThrottle<R> {
    /// 在飞的"组"数：按 `group(r)` 连续相等计一组。
    pub fn in_flight_groups<K: PartialEq>(&self, group: impl Fn(&R) -> K) -> usize {
        let mut count = 0;
        let mut last: Option<K> = None;
        for r in &self.pending {
            let k = group(r);
            if last.as_ref() != Some(&k) { count += 1; last = Some(k); }
        }
        count
    }
}
```

```rust
// rg/vulkan.rs：公开 API
impl VulkanGraphSession {
    /// 开启一个显示帧并在此处做在飞节流；之后同一显示帧内的 begin_graph 不再等待。
    pub fn begin_display_frame(&mut self) -> crate::Result<DisplayFrameId> {
        if self.active_display.is_some() {
            return Err(crate::Error::invalid_usage("end_display_frame before beginning another"));
        }
        self.collect_frame_imports();
        self.reap_presentation();
        // 原 begin_frame 2418-2431 的回收段保持不变（抽成 fn maintain_before_frame）
        self.maintain_before_frame()?;
        let limit = self.backend.device.frames_in_flight() as usize;
        while self.pending.in_flight_groups(|p| p.display) >= limit {
            let frame = self.pending.front().unwrap().frame;
            self.wait_frame(frame, 10_000_000_000)?;   // 弹出最旧一张图；循环直到最旧显示帧整组出队
        }
        if self.is_poisoned() { return Err(crate::Error::DeviceLost); }
        let id = DisplayFrameId(self.next_display);
        self.next_display += 1;
        self.active_display = Some(id);
        Ok(id)
    }
    /// 在当前显示帧内声明一张图：不节流，只做每图状态清理（原 2440-2455）。
    pub fn begin_graph(&mut self) -> crate::Result<graph::RenderGraph> {
        let Some(_) = self.active_display else {
            return Err(crate::Error::invalid_usage("begin_display_frame first"));
        };
        if self.active_prepared.is_some() { return Err(...同 2413-2417...); }
        self.begin_graph_state()   // = 原 2440-2455：named.begin_frame、ledger.begin、backend.*.clear、RenderGraph::new
    }
    pub fn end_display_frame(&mut self) -> crate::Result<()> {
        self.active_display.take().ok_or_else(|| crate::Error::invalid_usage("no active display frame"))?;
        Ok(())
    }
    /// 兼容：旧调用 = 一张图一个显示帧。现有 rhapsody 测试全部走这里，语义不变。
    pub fn begin_frame(&mut self) -> crate::Result<graph::RenderGraph> {
        if self.active_display.is_some() {
            return Err(crate::Error::invalid_usage("explicit display frame is active; use begin_graph"));
        }
        self.begin_display_frame()?;
        let graph = self.begin_graph()?;
        self.active_display = None;         // 单图显示帧立即关闭；execute_prepared 用 display 记在 PendingSessionFrame
        Ok(graph)
    }
}
```

`execute_prepared`（`vulkan.rs:3001-3127`）里构造 `PendingSessionFrame` 时填 `display: self.current_display_for_pending`，其中该值在 `begin_graph`/`begin_frame` 时记录（`self.pending_display = id`），不依赖 `active_display` 是否已关闭。

`wait_gpu_queue_trim`（`vulkan.rs:3168`）与 `frame_throttle.rs:66-72` 的 `len <= 1` 判断改为 `in_flight_groups <= 1`，否则 input-lag-reduction 路径会在同一显示帧的两张图上错误等待。

`FrameLoop`（`frame_loop.rs`）同步改：`begin_frame`（`:287-303`）→ `self.graph.begin_display_frame()?; self.graph.begin_graph()`；`render_presentation_inner`（`:482`）→ `begin_graph()`；`execute`（`:319-343`）结束处 `end_display_frame()`。`FrameLoop` 自己的 `PresentTicket` 节流（`:291-298`）保留。

**tiny-glade 侧**：

`render_bridge.rs` 增加透传：

```rust
impl CoreRenderBridge {
    pub fn begin_display_frame(&mut self) -> rhapsody::Result<rhapsody::rg::DisplayFrameId> {
        self.session.borrow_mut().begin_display_frame()
    }
    pub fn end_display_frame(&mut self) -> rhapsody::Result<()> {
        self.session.borrow_mut().end_display_frame()
    }
}
// build_frame（:254-294）第 263 行：
let graph = {
    let mut session = self.session.borrow_mut();
    if session.has_active_display_frame() { session.begin_graph()? } else { session.begin_frame()? }
};
```

`production_renderer.rs` 在帧的外层括起来——`GameRenderHostFactory::host`（`:1266-1299`，每帧入口）与 `finish_render`（`:1243-1263`，每帧出口）：

```rust
fn host<'a>(&'a mut self, world: &'a mut World, egui: &'a mut EguiBackend) -> Self::Host<'a> {
    ...现有 timing/activate_application...
    let started = self.timing_start();
    if let Err(error) = self.bridge.begin_display_frame() { self.record_error(error); }
    self.timing_end(TimedStage::FrameThrottle, started);      // 新增阶段
    ProductionRenderHost { ... }
}
fn finish_render(&mut self) -> anyhow::Result<()> {
    if let Err(error) = self.bridge.end_display_frame() { self.record_error(error); }   // 放在日志之前，错误路径也要关
    ...现有日志...
}
```

`activate_application` 切换 App 时 `self.bridge` 会换成另一张 session（`other_applications`，`:1125`），`begin_display_frame` 必须在 `activate_application` **之后**调用（上面顺序已保证）。`shutdown`（`:1138`）前若 `active_display` 仍开着，`end_display_frame` 一次（忽略错误）。

`acquire_swapchain_image` 的 `PresentTicket` 节流（`:1647`）保留：它是交换链图像配额，不是 GPU 在飞配额；二者现在各管各的。

**`frames_in_flight` 是否提到 3**：先不动。按显示帧计数后 `F=2` 已经是"CPU 帧 n+1 与 GPU 帧 n 重叠"，`F=3` 只增加一帧延迟。`DeviceConfig.frames_in_flight` 字段已存在（`device/init.rs:50`），测量时可以临时改生产构造点（【未核实】tiny-glade 侧 `DeviceConfig { .. }` 的构造位置，grep `frames_in_flight` 在 `crates/tiny-glade/src` 只命中 `production_renderer.rs:1647`，构造点可能在 `vulkan_config_main.rs` 或 `render_bridge.rs`）。

#### 方案 (b)：展示图并入主图（ARCHITECTURE §3.8 的目标形态）

机制已存在：`VulkanGraphSession::import_swapchain_image`（`rg/vulkan_presentation.rs:366-444`）把 `AcquiredImageLease` 导入**任意**图并 `graph.gate_first_use(resource)`（`:413`，`rg/graph.rs:404-418`），让 acquire 等待只挂在首次触碰交换链图像的批次上，前面的批次不受影响。所以把 `resolve::build` 的 pass 追加到主图末尾在 rhapsody 层面是合法的。

需要改的宿主顺序：`acquire` 必须移到 `compile_world_graph` **之前**（声明时就要有 lease），即 `render_frame.rs:297-380` 的 B 原版顺序被打破；`Acquire::Unavailable` 时主图照样跑但不加 resolve；OOM 重试（`recompile_at_reduced_scale`，`:383-411`，"重试的结果不再展示"）需要在重编译前 `cancel_frame` 归还图像再重新 acquire；`main_output/take_exported_texture/ExportExpectation/frame_observation/shared_texture_capture::submitted`（`:1587-1631`）全部要改成"图内资源"而不是"导出后再导入"；`loading_graph` 同样处理。

不推荐现在做的原因：(1) FIFO 下 `vkAcquireNextImageKHR` 可能阻塞到 vsync，放到声明前等于把阻塞挪到 CPU 段最前面；(2) 改动面覆盖 `production_renderer.rs` 1300–1700 的全部展示/截图/观测逻辑；(3) (a) 已经拿到同样的重叠收益。(b) 留给 C1 retained graph 一起做。

### 影响面

- rhapsody：`rg/vulkan.rs`（`begin_frame`、`execute_prepared` 两处 `PendingSessionFrame` 构造 3058/3084、`wait_gpu_queue_trim`、`pending_frame_count`）、`rg/frame_throttle.rs`、`rg/frame_loop.rs`、`rg/mod.rs` 导出 `DisplayFrameId`。
- tiny-glade：`render_bridge.rs`（`build_frame`、新增两个透传）、`production_renderer.rs`（`host`、`finish_render`、`shutdown`、`TimedStage` 枚举 + `RenderStageTiming` 新字段 + 日志格式串）。
- 现有测试：`crates/rhapsody/tests/device_config.rs:100 configured_frames_in_flight_controls_the_session`（走 `begin_frame`，语义不变，必须继续通过）；`crates/rhapsody/tests/raw_fixes_swapchain.rs:574 strict_frame_loop_presents_through_the_presentation_graph`、`:680 presentation_graph_failure_after_acquire_returns_the_image_and_recovers`（FrameLoop 路径）；`crates/tiny-glade/tests/core_raster_draw.rs:1137 core_raster_pass_chain_animates_over_frames_with_two_in_flight`；`crates/rhapsody/tests/p0_perf_baseline.rs:104 new_with_frames`。
- `frame_throttle.rs` 自带 3 个单元测试（`begin_frame_drains_to_the_latest_one_in_submission_order` 等）不受影响。

### 验证

- 新增单元测试 `frame_throttle.rs`：`in_flight_groups` 对 `[1,1,2,2,3]` 返回 3，空返回 0。
- 新增 rhapsody 测试（`tests/device_config.rs`）：`two_graphs_in_one_display_frame_share_one_slot`——`frames_in_flight=2`，循环 `begin_display_frame → begin_graph/execute → begin_graph/execute → end_display_frame`，断言第 3 个显示帧的 `begin_display_frame` 前 `pending_frame_count()==4`（两组），之后 `==2`；并断言在 `active_display` 存在时调用 `begin_frame` 返回 `InvalidUsage`。
- `render_stages`：新增 `frame_throttle_ms`；`declare_ms` 与 `resolve_inclusive_ms` 应各自减去原先嵌在里面的 GPU 等待；`observed_completion_wall_ms`（`last_known_frame_duration_ns`）上升是正常的（在飞变长）。总帧时间 `total_ms`（`game_loop.rs:428`）应接近 `max(CPU, GPU)` 而不是二者之和。
- lavapipe 多帧回归：`multiframe-frame02..30.png` 像素不变。

### 风险与回退

- 在飞真正变成 2 显示帧后，所有"上一帧结果本帧读"的路径（temporal 纹理、`result_cache`、`named` 资源、`frame_imports`）承受的是 rhapsody 本来就声称支持的 `frames_in_flight=2`，但生产从未真正跑到过。首个症状通常是 `frame import was invalidated before graph execution`（`vulkan.rs:2320`）或 named 资源 parity 错误。回退：`begin_display_frame` 里把 `in_flight_groups` 换回 `in_flight()` 即恢复旧行为（一行）。
- 动态常量页、bindless 参数页（`raw/dynamic_constants.rs`）是否按在飞帧数分环【未核实】；若按"2 个 session 帧"分环，现在同一显示帧的两张图会占两个环槽、而两个显示帧占四个——需要 grep `frames_in_flight()` 的其他用途（`vulkan.rs:828 let slots = self.device.frames_in_flight();`）并确认环大小 ≥ 2×F。
- `wait_gpu_queue_trim` 语义变化只影响 `input_lag_reduction=true` 的用户。

---

## A2 同步上传改异步票据 + W11 脏检查

### 现状

`crates/tiny-glade/src/render_host.rs:163-222` `BufferHost::update_buffer_bytes`（`BufferBackend` trait 实现，country-core 的 `GraphicsDevice::update_buffer_bytes` 经 `render/ssbo.rs:204-221` 转发到这里）：

```rust
// render_host.rs:173-221（节选）
if managed.desc.memory_location != MemoryLocation::GpuOnly && managed.persistent_host_ptr().is_some() {
    return self.device.update_buffer_bytes(&managed, data, ranges, offset);   // 映射缓冲：直接 memcpy
}
rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
// This host method promises completion on return. ...
let mut uploads = self.uploads.lock();
if uploads.is_none() {
    *uploads = Some(BufferUploads {
        manager: TransferManager::new_with_queue(self.device.clone(),
            TransferConfig { slots: 1, chunk_bytes: 1 << 20, max_bytes: 4 << 20 },
            UploadQueueConfig { max_keys: 1024, ..Default::default() })?,
        epoch: 0,
    });
}
...
service.manager.upload_buffer_ranges_wait(managed.shared(), data, ranges, offset as u64,
    UploadOptions { key: UploadKey(buffer.id), epoch: service.epoch, priority: UploadPriority::High },
    u64::MAX)
```

`crates/rhapsody/src/transfer.rs:510-608` `upload_buffer_ranges_wait`：每凑满 `chunk_bytes`（1 MiB）或 region 上限就 `finish_sparse_batch`：

```rust
// transfer.rs:592-608
fn finish_sparse_batch(&mut self, ticket: QueuedUploadTicket, timeout_ns: u64) -> crate::Result<()> {
    let outcome = (|| {
        self.flush_uploads(1)?;
        ticket.try_submitted()?.ok_or(crate::Error::Timeout)?.wait(timeout_ns)
    })();
    ...
}
```

`ticket.wait` = `device.wait(&SyncSet::from_point(point), timeout)`（`transfer.rs:126-129`）——一次完整的 CPU→GPU→CPU 往返。此外 `:548` 要求 `queued_jobs == 0` 否则 `Timeout`。

**已有的异步 API（同一个 `TransferManager`）**：

| API | 位置 | 语义 |
|---|---|---|
| `enqueue_upload(dest: Arc<Buffer>, offset, bytes, options) -> QueuedUploadTicket` | `transfer.rs:352-379` | 只排队、拷到队列侧缓冲，不触 GPU |
| `flush_uploads(max_submits) -> UploadFlush` | `:416-471` | 把排队任务打包进 staging 并提交；staging 满则 `capacity_deferred=true` 并停止；**从不等待** |
| `QueuedUploadTicket::try_submitted() -> Option<UploadTicket>` | `:182-197` | 提交后拿到带 `point: SyncPoint` 的票 |
| `UploadTicket::{wait,is_complete,import_buffer}` | `:125-158` | 等待/查询；`import_buffer` 走普通 `bind_import` |
| `cancel_key_before(dest, key, epoch)` | `:399-411` | 按 (缓冲, key, epoch) 撤销未提交的旧写 |

**图如何消费上传**：导入不携带显式 `SyncPoint`。`CoreGraphFrame::import_buffer(buffer, initial_access)`（`render_bridge.rs:733-755`）→ `session.import_frame_buffer` → `bind_import`。绑定时 `rg/vulkan_queue.rs:262-273`：

```rust
object.owner_cell().with_range_state(initial, stats, |table| {
    planner.add_initial(logical as u32, table.cells())
})?;
```

即规划器读取物理缓冲自己的 `last_use: SyncCell`（`resource/tracked.rs:31`）里的 `RangeStateTable`，上传器在 `queued.publish(index, point)`（`transfer.rs:450`）后该表记录了 transfer 写入的 stage/access 与 point；提交时 `prepare_submission_states` 再按表生成屏障（`raw/command_buffer.rs:640-711`，见 ANALYSIS §2.4）。跨队列族的 acquire 由"下一次图导入"或 `flush_acquires` 完成（`transfer.rs:110-116` 文档）。**结论：只要上传在读它的图之前被 `flush_uploads` 提交到同一图形队列，图就自动排在它之后且带正确屏障；宿主不需要等待。**【未核实】屏障的目标 stage 是否精确到 transfer→shader-read，但这与现在 `_wait` 路径走的是同一张表，不会更差。

**W11 每帧全量上传**。`crates/tiny-glade/src/world_terrain.rs:47-87` `rasterize_buildings`：

```rust
// world_terrain.rs:57-78
let sources = world.get_resource::<PublicWalls>()...all().filter_map(|(id, wall)| { ... Some((id, source)) })
    .collect::<std::collections::HashMap<_, _>>();           // 每帧分配
let count = sources.values().filter_map(GroundedSource::to_element).count();
let has_buffer = ...contains::<GroundedBuildingsSsbo>();
if has_buffer {
    assert!(count < buildings::MAX_BUILDINGS, "{}", buildings::TOO_MANY_PANIC);
    let device = world.get_resource::<GraphicsDeviceResource>()...clone();
    buildings::upload_buildings(&sources, &mut world.resource_mut::<AssetSsboLibrary>(), &*device);
}
```

`country-core/src/systems/render_loop/compute_passes/rasterize_buildings.rs:146-170` `upload_buildings`：再 collect 一个 `Vec<GroundedBuildingsSsbo>`，`count != 0` 则 `buffer.buffer.gpu_write(device, &elements)` → `update_buffer_bytes`。缓冲 `MemoryLocation`：`ssbo.rs:152-159` 把 memory code 非 2/3/4 映射为 `GpuOnly`，所以走同步路径。`PublicWalls`（`resources/walls/public_walls.rs:63-290`）没有自己的变化标记（`get_mut/consume/consume_with_paths` 直接改内部 map）。

### 问题

每次 `update_buffer_bytes` 到 GpuOnly 缓冲 = 至少一次 fence 等待；在 A1 之前就是"在声明阶段里再插一段 GPU 延迟"，A1 之后仍然会把声明线程卡住直到队列里**所有**先前提交（含上一帧主图）跑完——因为 timeline 等待是单调的，等上传点等于等它前面的一切。W11 还每帧分配 `HashMap` + `Vec` 并上传 ≤ 160 KB。其他调用者（`sparse_instance_buffer.rs:389`、`shadowed_buffer.rs:84`、`nani/buffers.rs:411`）同理，次数未测。

### 目标改法

#### A2.1 `BufferHost` 默认走排队，帧提交前统一 flush

```rust
// render_host.rs
struct BufferUploads {
    manager: rhapsody::transfer::TransferManager,
    epoch: u64,
    /// 本帧已接受、尚未确认提交的票据；flush 后清空。
    queued: Vec<rhapsody::transfer::QueuedUploadTicket>,
    queued_bytes: u64,
}

impl BufferHost {
    fn upload_service(&self) -> rhapsody::Result<MappedMutexGuard<'_, BufferUploads>>;  // 懒初始化，配置见下

    /// 非阻塞排队。帧内任意次调用；GPU 顺序由 flush_pending_uploads 保证。
    pub fn queue_buffer_update(&self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>], offset: usize)
        -> Result<(), BackendError>
    {
        let managed = self.lookup(buffer)...;
        rhapsody::resource::plan_buffer_update(&managed.desc, data.len(), ranges, offset as u64)?;
        let mut service = self.upload_service()?;
        service.epoch += 1;   // 保持 checked_add 语义
        let options = UploadOptions { key: UploadKey(buffer.id), epoch: service.epoch, priority: UploadPriority::High };
        for range in ranges.iter().filter(|r| !r.is_empty()) {
            let ticket = service.manager.enqueue_upload(
                managed.shared(), offset as u64 + range.start as u64, &data[range.clone()], options)?;
            service.queued_bytes += ticket.bytes;
            service.queued.push(ticket);
        }
        Ok(())
    }

    /// 帧提交前调用。把排队任务全部提交到图形队列；staging 容量不足时有界等待最旧 staging 槽释放。
    /// 返回统计供 TG_FRAME_TIMING 打印。永不等待本帧上传本身完成。
    pub fn flush_pending_uploads(&self, timeout: Duration) -> rhapsody::Result<UploadFlushSummary> {
        let mut service = self.upload_service()?;
        let deadline = Instant::now() + timeout;
        let mut summary = UploadFlushSummary::default();
        loop {
            let flush = service.manager.flush_uploads(usize::MAX)?;
            summary.submitted_jobs += flush.submitted_jobs;
            summary.submitted_bytes += flush.submitted_bytes;
            if flush.remaining_jobs == 0 { break; }
            if !flush.capacity_deferred { return Err(rhapsody::Error::invalid_usage("upload queue stalled without capacity pressure")); }
            // staging 环满：等最旧的一个已提交票据（不是本帧全部）
            let oldest = service.queued.iter().find_map(|t| t.try_submitted().ok().flatten())
                .ok_or(rhapsody::Error::Timeout)?;
            let remaining = deadline.checked_duration_since(Instant::now()).ok_or(rhapsody::Error::Timeout)?;
            oldest.wait(remaining.as_nanos() as u64)?;
            summary.capacity_waits += 1;
        }
        // 全部已提交：票据不再需要（Arc<Buffer> 由队列持有到提交完成）
        for t in service.queued.drain(..) { debug_assert!(matches!(t.status(), UploadStatus::Submitted(_))); }
        service.queued_bytes = 0;
        Ok(summary)
    }

    /// 显式同步版本：启动/资产加载/测试用。就是现在的 update_buffer_bytes 实现体，先 flush_pending_uploads 再走 _wait。
    pub fn update_buffer_bytes_wait(&self, ...) -> Result<(), BackendError>;
}

// BufferBackend for BufferHost
fn update_buffer_bytes(&self, buffer, data, ranges, offset) -> Result<(), BackendError> {
    // 映射缓冲分支不变（173-179）
    self.queue_buffer_update(buffer, data, ranges, offset)       // 默认异步
}
fn release_buffer(&self, buffer: &Buffer) {
    // 新增：撤销该缓冲所有未提交写；队列持有 Arc<Buffer>，内存安全无关，这是避免给即将释放的缓冲做无用拷贝
    if let Some(service) = self.uploads.lock().as_mut() {
        let _ = service.manager.cancel_key_before(&managed.shared(), UploadKey(buffer.id), u64::MAX);
    }
    ...原有释放...
}
```

`TransferConfig` 改为 `slots: 3, chunk_bytes: 1 << 20, max_bytes: 16 << 20`（staging 环 3 槽覆盖 2 显示帧在飞 + 1 当前帧；具体数值按真机 `upload_submitted_bytes` 再调）。`UploadQueueConfig.max_keys: 1024` 保持。

**flush 挂点**（必须在读它的图提交之前，且在所有宿主写入之后）：

1. `render_bridge.rs:254-294 build_frame`：`build(&mut frame)?` 返回后、`frame.take_graph()` 之前：`self.tables.buffers.flush_pending_uploads(UPLOAD_FLUSH_TIMEOUT)?`（计入新 `TimedStage::UploadFlush`）。声明阶段是所有 `update_buffer_bytes` 调用的所在（W5/W11/walls/nani 等都在 `build_full_world_stage` 内）。
2. `render_bridge.rs:327-339 execute_compiled` 与 `:296-302 execute_built`：`execute_prepared` 之前再 flush 一次（通常无任务，O(1)）。这覆盖了"编译之后、提交之前"又有写入的异常路径。
3. 启动路径 `warm_up_gameplay_with_world_builder`（`render_bridge.rs:1083+`）走 `build_frame`，自动覆盖。
4. 录制回调（`PassContext` 内）里若有 `update_buffer_bytes`【未核实是否存在】会在提交之后才 flush → 数据下一帧才到。grep `update_buffer_bytes\|gpu_write` 在 `draw_callbacks_*`/`record` 闭包里，若有则该处改用 `update_buffer_bytes_wait` 或改到声明阶段。

#### A2.2 W11 只在建筑变化时上传

`rasterize_buildings.rs` 新增资源与签名：

```rust
#[derive(Resource, Default)]
pub struct GroundedUploadCache {
    /// 上次上传的元素（按 WallId 升序），用于逐字节比较
    pub elements: Vec<GroundedBuildingsSsbo>,
    pub uploaded_once: bool,
}

/// 输入改为已按 WallId 排序的切片；返回 count。只有内容变化才 gpu_write。
pub fn upload_buildings(
    sources: &[(WallId, GroundedSource)],         // 调用方保证按 id 排序
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    cache: &mut GroundedUploadCache,
    scratch: &mut Vec<GroundedBuildingsSsbo>,     // 复用分配
) -> usize {
    scratch.clear();
    scratch.extend(sources.iter().filter_map(|(_, s)| s.to_element()));
    let count = scratch.len();
    let Some(buffer) = ssbos.get_mut::<GroundedBuildingsSsbo>() else { return count; };
    assert!(count < MAX_BUILDINGS, "{TOO_MANY_PANIC}");
    let unchanged = cache.uploaded_once && *scratch == cache.elements;
    if count == 0 {
        buffer.instance_count = 0;
    } else if !unchanged {
        buffer.buffer.gpu_write(device, scratch).unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        buffer.instance_count = count;
    }
    if !unchanged { std::mem::swap(&mut cache.elements, scratch); cache.uploaded_once = true; }
    count
}
```

`GroundedBuildingsSsbo(pub [u32; 8])` 需要 `derive(PartialEq, Eq)`（`:75`，现在只有 Copy/Clone【未核实其他 derive】）。

`world_terrain.rs:57-79` 改为：

```rust
let mut sources: Vec<(WallId, GroundedSource)> = scratch_vec_from_world(world);   // 复用：存成 Local 资源 GroundedScratch
sources.clear();
sources.extend(world.get_resource::<PublicWalls>()?.all().filter_map(|(id, wall)| { ...同现在... }));
sources.sort_unstable_by_key(|(id, _)| *id);           // HashMap 顺序不稳定，排序保证逐帧可比
let count = world.resource_scope(|world, mut ssbos: Mut<AssetSsboLibrary>| {
    let mut cache = world.get_resource_or_insert_with::<GroundedUploadCache>(Default::default);
    ...
    buildings::upload_buildings(&sources, &mut ssbos, &*device, &mut cache, &mut scratch)
});
```

`WallId` 需要 `Ord`【未核实】；若无则按 `u64` 原值排序。元素顺序变化是否影响着色器：`rasterize_buildings.cs` 对 `count` 个形状做掩码光栅化，顺序无关【未核实，实施时读 shader】；排序后的顺序本身是确定的，即使有关也只是和"旧 HashMap 随机序"的区别。

`upload_buildings` 的其他调用者：只有 `world_terrain.rs:78`（`grep upload_buildings(` 仅命中定义处与此处及本文件测试 `:288-364`）。

### 影响面

- `render_host.rs`：`BufferUploads`、`update_buffer_bytes`、`release_buffer`、新增 3 个方法；`HostTables`（`:2191-2195`）不变。
- `render_bridge.rs`：`build_frame`、`execute_built`、`execute_compiled`。
- `production_renderer.rs`：`TimedStage::UploadFlush` + 日志字段 `upload_flush_ms`、`upload_bytes`。
- `country-core/.../rasterize_buildings.rs`：签名、新资源、derive；本文件测试 `:288-364`（`lib.add_new::<GroundedBuildingsSsbo>` 等）按新签名改。
- `world_terrain.rs:47-87`。
- 依赖"返回即完成"的调用者审计：`country-core/src/render/{sparse_instance_buffer.rs:389, shadowed_buffer.rs:84, nani/buffers.rs:411, normals_debug.rs:84}`；它们写完后在**同帧图里读**（GPU），不做 CPU 回读，排队语义等价。CPU 回读路径（`read_buffer`，`transfer.rs:746`）是独立 API，不受影响。
- 测试里的 `BufferBackend` mock（`tests/geometry_gpu.rs:35`、`original_nani_gpu.rs:49`、`original_preplaced_startup.rs:62`、`render/test_device.rs:41`、`ssbo::tests::MockDevice`）实现的是 trait，不需要改；但 GPU 集成测试（`tests/render_host.rs`、`original_*_gpu.rs`）若在"写入后不经 build_frame 就 execute"会读到旧数据——需要逐个看是否经过 `build_frame`；不经过的改调 `flush_pending_uploads` 或 `update_buffer_bytes_wait`。

### 验证

- 现有：`crates/rhapsody/tests/transfers.rs:133 nonblocking_host_writes_reject_in_flight_use_and_recover_after_completion`、`raw_core_buffer_texture.rs:378 update_buffer_bytes_requires_transfer_dst_and_always_stages`、`host_buffer_ranges.rs`（全部不改）。
- 新增 `tests/render_host.rs`：`queued_update_is_visible_to_the_next_built_frame`——`update_buffer_bytes` 后 `build_frame` 里 `import_buffer` + 一个拷贝 pass 到可读回缓冲，`execute` 后读回等于写入；并断言 `flush_pending_uploads` 前 `queued_bytes > 0`、后 `== 0`。
- 新增 `rasterize_buildings.rs` 测试：用 `ssbo::tests::MockDevice` 计数 `update_buffer_bytes` 调用——同一输入连续两次 `upload_buildings` 只上传一次；改一个 `progress` 后再上传一次；`count==0` 只置 `instance_count`。
- `render_stages`：`declare_ms` 下降（去掉 fence 等待），新字段 `upload_flush_ms` 应为亚毫秒、`upload_bytes` 稳态趋近 0（W11 不再每帧上传）。
- 用 `TG_FRAME_TIMING` 另加一条计数：`update_buffer_bytes` 每帧调用次数与字节数（`BufferUploads` 里累加，`finish_render` 打印），用于定位其他每帧全量上传者。

### 风险与回退

- staging 环 3 槽 × 1 MiB 不够时 `flush_pending_uploads` 会等最旧上传完成——这是真正的 GPU 等待，但有计数（`capacity_waits`）可见；先把 `max_bytes` 调大再考虑分帧。
- 写-写顺序：同一缓冲同一帧多次 `update_buffer_bytes`，队列按 `epoch` 递增、同 key 合并（`UploadQueueConfig`，`queue::enqueue`【未核实合并规则是"覆盖"还是"追加"】）。若合并规则是"后 epoch 覆盖同区间"则语义与同步版本相同；实施时读 `transfer/queue.rs` 的 `enqueue` 确认，否则对同 key 多写改成先 `cancel_key_before`。
- 回退：`update_buffer_bytes` 改回调用 `update_buffer_bytes_wait`（一行）。W11 的缓存可用 `GroundedUploadCache` 资源缺省不插入来禁用（`get_resource_or_insert_with` 改 `get_resource_mut` + 无则每帧上传）。

---

## A3 prewarm 只扫待决票据

### 现状

`crates/tiny-glade/src/render_host.rs:1595-1666` `ShaderHost::prewarm`：

```rust
// render_host.rs:1605-1626
let mut targets = self.compute.lock().iter().cloned().map(Target::Compute).collect::<Vec<_>>();
targets.extend(self.raster.lock().iter().cloned().map(Target::Raster));
if let Some(adapter) = self.indexed_count_adapter.lock().as_ref() {
    targets.push(Target::IndexedCount(adapter.ticket()));
}
let total = targets.len();
for (index, target) in targets.into_iter().enumerate() {
    let remaining = deadline.checked_duration_since(Instant::now()).ok_or(rhapsody::Error::Timeout)?;
    let ticket = match &target { Target::Compute(e) => &e.pipeline, Target::Raster(e) => &e.pipeline, Target::IndexedCount(t) => t };
    if let Err(error) = ticket.wait(remaining) {
        let identity = ...format!(...)...;
        log::error!(target: "tiny_glade::pipeline", "{message}");
        ...
        return Err(error.into());
    }
}
Ok(())
```

注册表：`compute: Mutex<Vec<Arc<ComputeEntry>>>`、`raster: Mutex<Vec<Arc<RasterEntry>>>`（`:700-701`），上限各 4096。入口 push 点：compute `:1001-1005`（`table.push(entry)` 后返回 `ComputePipeline { id }`），raster `:1980-1986`（`create_raster_pipeline`），adapter `:1583-1589`。

`crates/rhapsody/src/pipeline.rs:226-268`：`PipelineTicket::poll` 取一次锁返回 `Pending/Ready(Arc)/Failed(Arc)`；`wait(timeout)` 对 Ready 立即返回克隆，Pending 则 condvar 等到 deadline 返 `Timeout`，Failed 返错误。

调用点（每帧 2 次）：`production_renderer.rs:532`（世界图）、`:635`（最小图）、`:1336`（展示图）、`:1453`（加载图）、`:171`（启动）、`render_bridge.rs:1105`（warmup）。

### 问题

每次调用：2 把锁、N 次 `Arc` clone、一个 `Vec<Target>` 分配、N 次 `wait`（每次一把 `Mutex` + 一次 `Arc` clone）。N 是**宿主生命周期内**注册过的全部管线数（上限 8193），稳态每帧两遍。单次成本不大（N≈数百时 ≈ 几十 µs），但纯浪费；真正的卡顿来自新变体的 Pending 等待，那部分本来就该保留。

### 目标改法

```rust
// render_host.rs
pub struct ShaderHost {
    ...
    /// 已请求、尚未被 prewarm 观察到终态的票据。请求时登记，Ready 时出队。
    pending_tickets: Mutex<Vec<PendingTicket>>,
}
struct PendingTicket {
    ticket: PipelineTicket,
    identity: PrewarmIdentity,
}
enum PrewarmIdentity {
    Compute(Arc<ComputeEntry>),
    Raster(Arc<RasterEntry>),
    IndexedCount,
}

impl ShaderHost {
    fn note_pending(&self, ticket: PipelineTicket, identity: PrewarmIdentity) -> rhapsody::Result<()> {
        // 已 Ready 的（编译器缓存命中）也登记，首个 prewarm 零等待出队；避免在这里 poll 以保持请求路径无锁竞争
        let mut pending = self.pending_tickets.lock();
        pending.try_reserve(1).map_err(|_| rhapsody::Error::OutOfHostMemory)?;
        pending.push(PendingTicket { ticket, identity });
        Ok(())
    }

    /// 语义保持：遇到第一个 Failed/Timeout 即返回同类错误并打同样的日志；Ready 的票据不再被扫描。
    pub fn prewarm(&self, timeout: Duration) -> rhapsody::Result<()> {
        let deadline = Instant::now().checked_add(timeout).ok_or(rhapsody::Error::Timeout)?;
        let batch = std::mem::take(&mut *self.pending_tickets.lock());   // 不持锁等待
        let total = batch.len();
        let mut iter = batch.into_iter().enumerate();
        while let Some((index, pending)) = iter.next() {
            let remaining = match deadline.checked_duration_since(Instant::now()) {
                Some(r) => r,
                None => { self.requeue(std::iter::once(pending).chain(iter.map(|(_, p)| p))); return Err(rhapsody::Error::Timeout); }
            };
            match pending.ticket.wait(remaining) {
                Ok(_) => {}   // Ready：丢弃登记
                Err(error) => {
                    let identity = describe(&pending.identity);     // 原 1627-1647 的 format! 逻辑
                    let message = format!("pipeline prewarm failed [{}/{}] {}: {} ({:?}); compiler={:?}",
                        index + 1, total, identity, error, error.kind(), self.compiler.stats());
                    log::error!(target: "tiny_glade::pipeline", "{message}");
                    if !log::log_enabled!(target: "tiny_glade::pipeline", log::Level::Error) { eprintln!("{message}"); }
                    // Timeout 与 Failed 都放回：下一帧 prewarm 仍会再次报告，与现状"每帧重新 wait 全部"一致
                    self.requeue(std::iter::once(pending).chain(iter.map(|(_, p)| p)));
                    return Err(error.into());
                }
            }
        }
        Ok(())
    }
    fn requeue(&self, items: impl Iterator<Item = PendingTicket>) {
        let mut pending = self.pending_tickets.lock();
        let mut rest: Vec<_> = items.collect();
        rest.append(&mut pending);   // 保持先请求先等待的次序：本批剩余在前，期间新登记的在后
        *pending = rest;
    }
    #[cfg(test)] pub(crate) fn pending_ticket_count(&self) -> usize { self.pending_tickets.lock().len() }
}
```

登记点：

- `:1001-1005` compute：`table.push(entry.clone()); self.note_pending(entry.pipeline.clone(), PrewarmIdentity::Compute(entry))?;`
- `:1980-1986` raster：同上 `PrewarmIdentity::Raster(entry)`。
- `:1583-1589` adapter：`self.note_pending(adapter.ticket(), PrewarmIdentity::IndexedCount)?;`

`ComputeEntry.pipeline`/`RasterEntry.pipeline` 类型为 `PipelineTicket`【未核实字段名，按 `:1622-1623` 的 `&entry.pipeline` 推断】。

**保留的错误语义**：Failed 票据会在每一帧的 prewarm 里重复报错并返回 `Err`（与现状一致，现状每帧都会 `wait` 到同一个 Failed）；录制侧 `try_get` 对 Pending 返回 `Timeout` 的行为（`pipeline.rs:229-235`）不变。

### 影响面

- 仅 `render_host.rs`（结构体、构造函数 `:1567-1571`、三处登记、`prewarm`）。调用点签名不变。
- 测试：`crates/tiny-glade/tests/render_host.rs` 与 `original_*_gpu.rs` 直接或间接调用 `prewarm`，行为不变。

### 验证

- 新增 `render_host.rs` 单元测试（需要设备，放 `tests/render_host.rs`）：注册 2 个 compute + 1 个 raster 后 `pending_ticket_count()==3`，`prewarm` 成功后 `==0`，再调 `prewarm` 成功且不阻塞；用一个故意无效的 SPIR-V 产生 Failed 票据：两次 `prewarm` 都返回 `Err` 且 kind 相同，`pending_ticket_count()` 保持 1。
- `render_stages.prewarm_ms`：稳态应从"两遍 N 次 wait"降到 ≈ 0（只剩两次 `mem::take`）。新变体出现的帧 `prewarm_ms` 仍会是编译时间，这是预期。

### 风险与回退

- 若有代码在注册**之外**创建 `PipelineTicket` 并依赖 prewarm 等它【未核实；grep `PipelineTicket` 在 tiny-glade 侧只应出现在 `ShaderHost`】，会漏等——录制时 `try_get` 仍会返回 `Timeout`，不会错画。
- 回退：`prewarm` 体换回全表扫描，保留 `pending_tickets` 字段无害。

---

## A4 VAO 快照：一帧一次，带版本号贯穿 W23–W53

### 现状

`crates/tiny-glade/src/raster_host.rs:399-444` `RasterShared::refresh_draw_vaos(world, lists)`：扫全部 `DRAW_LIST_COUNT` 列表收集 `wanted: BTreeSet<u64>`，`world.query::<&Handle<Vao>>()` 全量 ECS 查询建 `HashMap`，逐个解析后调 `refresh_vao_handles`。

`:448-506` `refresh_vao_handles(world, handles)`：对每个句柄在 `AssetVAOLibrary` 查条目、经 `self.buffers.managed_buffer` 校验 index/attribute 缓冲归属、克隆名字与 `Arc<Buffer>`，最后

```rust
// raster_host.rs:504
self.assets.lock().vaos = snapshots;     // 整表替换
```

表类型：`AssetTables.vaos: HashMap<u64, VaoSnapshot>`（`:288`），`struct VaoSnapshot { index: Arc<Buffer>, index_count: u32, bindings: Vec<(Arc<str>, Arc<Buffer>)> }`（`:291-296`）——**注意名字 `VaoSnapshot` 已被这个"单条目"结构占用**，下文新类型叫 `VaoTableToken`。

`crates/tiny-glade/src/scene_raster.rs:66-78`：

```rust
pub fn new(frame, world, lists) -> rhapsody::Result<Self> {
    frame.raster_resources().refresh_draw_vaos(world, lists)?;     // 无条件
    Ok(Self { frame, world, lists })
}
```

调用点（每帧）：`production_renderer.rs:1413-1417`（`build_draw_lists`，全量）、`world_terrain.rs:411`（W5，子集 `[floor_vao, rocky_vao]`，**整表替换**）、`world_graph.rs:123`（W23）、`:132`（W25）、`:140`（W30）、`:144`（W34）、`world_lighting.rs:234`（W43）、`world_surface.rs:58`（W44）、`:80`（W46）、`world_postprocess.rs:212`（W53）、`render_bridge.rs:1101`（warmup）。

`AssetVAOLibrary`（`country-core/src/asset_libraries/vao_library.rs:22-116`）：`add`（`:52-73`，已存在则早退不改）、`rebuild_vao`（`:76-102`，保留 id 换物理缓冲；空索引走 `remove`）、`remove`（`:109-115`）。没有版本号。VAO 变更发生在 ECS 系统 `build_missing_vaos/rebuild_vaos`（`game_loop.rs:237`，Update 调度 set 44），即在 App.update 内，不在渲染声明内；审计（`render-cpu-hotpath-audit.md` §VAO）已逐一核实 W23→W53 之间没有 CPU 侧 VAO 所有权变更。

### 问题

一帧 9 次全量重建（含 build_draw_lists），每次 O(列表条目 + ECS VAO 实体 + 唯一 VAO × 绑定数) 加 2 个 HashMap 与若干 Vec 分配、一次整表替换。`vao_full_refresh ... calls=` 日志行（`production_renderer.rs:1256-1259`）已能打印次数与耗时。

### 目标改法

#### A4.1 库版本号

```rust
// vao_library.rs
pub struct AssetVAOLibrary {
    vaos: HashMap<Handle<Vao>, VaoEntry>,
    mesh_to_vao: HashMap<Handle<Mesh>, Handle<Vao>>,
    revision: u64,                      // 新增：任何成功的 add/rebuild/remove 自增
}
impl AssetVAOLibrary {
    pub fn revision(&self) -> u64 { self.revision }
    // add: 早退分支不动；insert 成功后 self.revision += 1
    // rebuild_vao: 空索引分支调 remove（remove 自己 +1）；成功替换 entry.vao 后 +1
    // remove: 只有真的 remove 了条目才 +1
}
```

#### A4.2 宿主侧表版本与令牌

```rust
// raster_host.rs
pub struct RasterShared {
    ...
    /// 每次整表替换（refresh_draw_vaos / refresh_vao_handles）自增
    vao_table_revision: AtomicU64,
}

/// 一次全量快照的身份。持有者可以在同一帧内复用，直到库或表任一版本变化。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VaoTableToken {
    owner: usize,            // Arc::as_ptr(RasterShared)，拒绝跨 App 复用
    library_revision: u64,   // AssetVAOLibrary::revision()
    table_revision: u64,     // 快照写入后的 vao_table_revision
}

impl RasterShared {
    /// = 现在的 refresh_draw_vaos，但返回令牌
    pub fn snapshot_draw_vaos(&self, world: &mut World, lists: &DrawLists) -> rhapsody::Result<VaoTableToken> {
        self.refresh_draw_vaos(world, lists)?;
        Ok(self.current_token(world)?)
    }
    fn current_token(&self, world: &World) -> rhapsody::Result<VaoTableToken> {
        let library = world.get_resource::<AssetVAOLibrary>().ok_or_else(...)?;
        Ok(VaoTableToken { owner: self as *const _ as usize, library_revision: library.revision(),
            table_revision: self.vao_table_revision.load(Ordering::Acquire) })
    }
    /// 令牌仍有效则 O(1) 返回；否则做一次全量刷新并返回新令牌（永不静默用旧表）。
    pub fn ensure_draw_vaos(&self, world: &mut World, lists: &DrawLists, token: VaoTableToken) -> rhapsody::Result<VaoTableToken> {
        if self.current_token(world)? == token { return Ok(token); }
        self.snapshot_draw_vaos(world, lists)
    }
    // refresh_vao_handles 末尾（:504）：
    //   self.assets.lock().vaos = snapshots;
    //   self.vao_table_revision.fetch_add(1, Ordering::AcqRel);
    // refresh_draw_vaos 经由 refresh_vao_handles，自然也 +1
}
```

#### A4.3 `SceneRasterHost::with_snapshot`

```rust
// scene_raster.rs
impl<'f,'s,'w,'l> SceneRasterHost<'f,'s,'w,'l> {
    /// 保留：不相关调用者/测试用，语义不变（全量刷新）。
    pub fn new(frame, world, lists) -> rhapsody::Result<Self> { ... }

    /// 复用同帧快照；令牌失效时自动全量刷新并把新令牌写回 `token`。
    pub fn with_snapshot(frame: &'f mut CoreGraphFrame<'s>, world: &'w mut World, lists: &'l DrawLists,
                         token: &mut VaoTableToken) -> rhapsody::Result<Self> {
        *token = frame.raster_resources().ensure_draw_vaos(world, lists, *token)?;
        Ok(Self { frame, world, lists })
    }
}
```

#### A4.4 调用点

`world_graph.rs` 世界图函数（包含 W23/W25/W30/W34 的那个 fn，`:110-150` 附近）：

```rust
let mut vaos = frame.raster_resources().snapshot_draw_vaos(world, lists)?;     // W23 前，一次
SceneRasterHost::with_snapshot(frame, world, lists, &mut vaos)?.shadow_pass()?;
...
SceneRasterHost::with_snapshot(frame, world, lists, &mut vaos)?.main_pass(&targets, &attachment(color))?;
...
SceneRasterHost::with_snapshot(frame, world, lists, &mut vaos)?.main_pass_oc2(...)?;
...
SceneRasterHost::with_snapshot(frame, world, lists, &mut vaos)?.late_forward_and_deferred_pass(...)?;
```

然后把 `vaos: &mut VaoTableToken` 作为新参数传给 `world_lighting.rs` 的 deferred 函数（`:234`）、`world_surface.rs` 的两个函数（`:58`、`:80`）、`world_postprocess.rs`（`:212`），各自改 `SceneRasterHost::new` → `with_snapshot`。

W5（`world_terrain.rs:411`）的 `refresh_vao_handles` 保持不变：它在 W23 之前，替换表并 `table_revision+1`，不影响后面（W23 前重新 snapshot）。

`production_renderer.rs:1413-1417 build_draw_lists` 的全量刷新：**第一步保留**（加载图/最小图/warmup 路径依赖它）；第二步（可选，需测量）把 `refresh_vao_handles` 改成"合并写入而非整表替换"（`assets.vaos.extend(snapshots)`），W5 就不再破坏 build_draw_lists 的快照，W23 前的 `snapshot_draw_vaos` 可换成 `ensure_draw_vaos(.., token_from_build_draw_lists)`，一帧降到 1 次。合并语义下表里会残留本帧列表不再引用的旧条目，查找按 id，且 `library_revision` 变化时整表重建，不会取到失效缓冲。

### 影响面

- `vao_library.rs`（+revision，3 个方法）；其 `tests` 模块（`:124+`）加断言。
- `raster_host.rs`（字段、构造 `:577` 附近、`refresh_vao_handles`、三个新方法、`pub use VaoTableToken`）。
- `scene_raster.rs`（`with_snapshot`）。
- `world_graph.rs`、`world_lighting.rs`、`world_surface.rs`、`world_postprocess.rs`（签名各加一个参数；它们的调用者只有 `world_graph.rs` 的世界图函数【未核实是否还有测试直接调用 `world_lighting::*`，grep `deferred_raster\|pre_taa_transparency_pass\|post_taa_transparency_pass` 在 `tests/`】）。
- `render_bridge.rs:1101` warmup 与 `production_renderer.rs:1417` 不变。
- 测试：`crates/tiny-glade/tests/core_raster_draw.rs`、`original_raster_gpu.rs`、`original_scene_compute_gpu.rs` 用 `SceneRasterHost::new`【未核实具体行】，签名不变。

### 验证

- `vao_library.rs` 单元测试：`add` 新 mesh +1、重复 `add` 不变、`rebuild_vao` +1、`remove` +1、`remove` 不存在的不变。
- 新 GPU 测试（`tests/core_raster_draw.rs`）：`snapshot_draw_vaos` 后 `ensure_draw_vaos` 同令牌不触发刷新（用 `vao_refresh_timing().calls` 比较）；`rebuild_vao` 后同令牌触发刷新且令牌改变；`refresh_vao_handles` 后同令牌触发刷新。
- `vao_full_refresh ... calls=` 每帧从 9 降到 2（第二步后 1）；`declare_ms` 相应下降。
- lavapipe 多帧像素回归（W5 回读 → W23 的次序必须保持）。

### 风险与回退

- 令牌校验依赖 `AssetVAOLibrary::revision` 覆盖**所有**变更路径；若有代码绕过 `add/rebuild_vao/remove` 直接改 `VaoEntry`（库字段私有，`get_by_handle` 返回 `&`，所以只能经由这三个方法——已由可见性保证）。
- 回退：`with_snapshot` 体改为无条件 `refresh_draw_vaos`，一行。

---

## A5 Bevy ECS 多线程执行器

### 现状

`Cargo.toml:117`：

```toml
bevy_ecs = { version = "=0.16.0", default-features = false, features = ["std"] }
bevy_tasks = { version = "=0.16.0", default-features = false, features = ["std", "async_executor", "multi_threaded"] }
```

`bevy_ecs 0.16.0` 的 `Cargo.toml:67` 定义 `multi_threaded = ["bevy_tasks/multi_threaded", ...]`；未启用时 `Schedule` 的默认 `ExecutorKind` 为 `SingleThreaded`（`bevy_ecs/src/schedule/executor/mod.rs:41-45` 文档："MultiThreaded everywhere else" 指启用该 feature 且非 wasm；`schedule.rs:224` 按 kind 构造 `MultiThreadedExecutor`）。

工程内没有任何 `ExecutorKind`/`set_executor_kind` 调用（grep `crates/` 为空），没有 `NonSend`/`thread_local!` 使用（grep `crates/country-core/src crates/tiny-glade/src` 为空）。`ComputeTaskPool` 已被 rhapsody 并行录制（`rg/parallel.rs:118 ComputeTaskPool::get_or_init(TaskPool::default)`）与 `hover_intersection.rs:267/488`（2 线程）使用；`MultiThreadedExecutor` 自己也会 `ComputeTaskPool::get_or_init(TaskPool::default)`（`bevy_ecs/src/schedule/executor/multi_threaded.rs:221`）。

`App` 构造在 `run_game.rs:248 run_game_thread`；系统注册在 `app_systems.rs`、`game_loop.rs:237`、country-core 各插件；显式顺序用 `chain_ignore_deferred`/`before_ignore_deferred`/`in_set(GameSystemSet(n))`。

### 问题

debug/lavapipe 基线 App.update 24.6 ms 全在单线程；release 真机值未测（`game_loop.rs:428 update_ms`）。原版 PDB 有 `MultiThreadedExecutor::run`（ANALYSIS §5）。

### 目标改法

1. `Cargo.toml:117`：

```toml
bevy_ecs = { version = "=0.16.0", default-features = false, features = ["std", "multi_threaded"] }
```

   `bevy_app`（`:116`）不需要加 feature（它不拥有执行器选择）。

2. `run_game.rs:248 run_game_thread` 开头（`App::new()` 之前）显式初始化池，控制线程数并留核给渲染线程：

```rust
{
    use bevy_tasks::{ComputeTaskPool, TaskPoolBuilder};
    let cores = std::thread::available_parallelism().map_or(4, |n| n.get());
    let threads = std::env::var("TG_ECS_THREADS").ok().and_then(|v| v.parse().ok())
        .unwrap_or_else(|| cores.saturating_sub(1).clamp(2, 8));
    ComputeTaskPool::get_or_init(|| TaskPoolBuilder::new().thread_name("tg-compute".into()).num_threads(threads).build());
}
```

   `get_or_init` 幂等，先到先得；测试里的 2 线程初始化不受影响。rhapsody 并行录制共用同一池（`parallel.rs:92-93` 注释即此意）。

3. 诊断期打开调度歧义检测（只在 `TG_SCHEDULE_AMBIGUITY=1` 时）：

```rust
use bevy_ecs::schedule::{LogLevel, ScheduleBuildSettings};
for label in [bevy_app::Update, bevy_app::PostUpdate, bevy_app::PreUpdate] {
    if let Some(s) = app.get_schedule_mut(label) {
        s.set_build_settings(ScheduleBuildSettings { ambiguity_detection: LogLevel::Warn, ..Default::default() });
    }
}
```

   跑一次完整存档，把警告逐条用 `.before/.after` 固定。单线程执行器也不保证"插入顺序"，只保证图拓扑序，所以这些歧义在现状下是**确定但未声明**的顺序；多线程下会变成非确定顺序。

4. 不需要改任何系统签名：`&mut World` 独占系统与任何 `!Send` 资源（会被 Bevy 自动当作 `NonSend`）仍在主线程串行执行。`GraphicsDeviceResource`（`world_terrain.rs:76` clone 得到的 `Arc<dyn GraphicsDevice>`）要求 `Send + Sync`【未核实 trait object 的 bound；若无则该资源成 NonSend，含它的系统全部回主线程，不报错但无收益——实施时 `grep "dyn GraphicsDevice"` 看 bound】。

### 影响面

- `Cargo.toml`、`run_game.rs`。`Cargo.lock` 不变（feature 变更不改版本）。
- 全部 `App::new()` 的测试（`game_loop.rs:1469/1477/1484` 等）会在多线程执行器下跑；`ui_update_owner_lifecycle_tests.rs:64`、`ui_update_chain_tests.rs:40` 已自建 2 线程池。

### 验证

- `cargo test -p country-core -p tiny-glade`（非 GPU 部分）全绿；重复跑 5 次看是否出现顺序型不稳定。
- `game_loop.rs:428 update_ms` 对比开关前后（同一存档、同一相机）。
- 歧义检测日志为空后再合入。

### 风险与回退

- 顺序歧义导致的逻辑差异（输入边沿、事件读写顺序）。回退：去掉 feature，或保留 feature 但对 `Update` 调 `set_executor_kind(ExecutorKind::SingleThreaded)` 逐调度回退。
- 线程池与渲染线程争核：UHD 630 机器通常 4C/8T，`cores-1` 上限 8 已留余量；A6 线程化后把渲染线程计入。

---

## A6 PGO 按图分槽 + 主循环搬到专用线程

### A6.1 PGO 分槽

#### 现状

`crates/rhapsody/src/rg/executor.rs:15-31`：

```rust
pub struct ExecutionChain {
    ...
    /// 并行录制配置；默认自适应（无统计或轻帧时串行）。
    pub parallel: crate::rg::parallel::ParallelRecording,
    /// 双缓冲 PGO：分块读 `previous`，本帧耗时写入 `current`，每次执行结束交换。
    pub pgo: PassPgoFrames,
    ...
}
```

每次执行后交换（`:949-962`、`:825-837`）：

```rust
pub fn execute_prepared_validated(&mut self, prepared, ...) -> Result<GraphExecution, Error> {
    let result = self.execute_prepared_inner(prepared, pool, dependency, backend, callbacks, validate);
    self.pgo.end_frame();
    result
}
```

并行决策（`:1247-1262`）：

```rust
let worth_it = !matches!(self.parallel.strategy, ChunkStrategy::Serial)
    && self.pgo.previous.total_ns() >= self.parallel.min_total_ns
    && (self.parallel.min_total_ns == 0 || !self.pgo.previous.is_empty());
let chunks = if worth_it && backend.supports_parallel_recording() {
    plan_chunks(self.parallel.strategy, &pass_names[batch.pass_range.clone()], &self.pgo.previous, &[]).into()
} else { once(0..batch.pass_range.len()).collect() };
```

`PassPgoFrames::end_frame`（`:2082-2085`）`swap(previous, current); current.reset()`。`ADAPTIVE`（`parallel.rs:144-151`）：`max_chunks: 8, chunk_overhead_ns: 20_000, min_total_ns: 200_000`。一个 session 一条 `chain`（`vulkan.rs:2063`），主图与展示图交替执行。

#### 问题

主图录制时 `previous` 是展示图（1 个 resolve pass，几十 µs < 200 µs）→ `worth_it=false` → 主图永远串行；展示图录制时 `previous` 是主图 → 可能对 1 个 pass 做分块（白付 20 µs 开销）。

#### 目标改法

把 PGO 按"图身份"分槽；身份 = pass 名序列的哈希，在 `compile_graph` 已遍历 `pass_names` 时顺手算出。

```rust
// executor.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PgoKey(pub u64);

pub struct PreparedGraph { ..., pub pgo_key: PgoKey }   // compile_graph（:~900-934）填充：
//   let mut h = xxhash_rust::xxh3::Xxh3::new(); for n in pass_names { h.update(n.as_bytes()); h.update(&[0]); } PgoKey(h.digest())
//   （xxhash-rust 已是工作区依赖，Cargo.toml:110；rhapsody 若未引用则加 dep）

#[derive(Default)]
pub struct PgoTable {
    slots: Vec<(PgoKey, PassPgoFrames, u64 /*last_used*/)>,
    clock: u64,
}
impl PgoTable {
    const MAX_SLOTS: usize = 8;
    pub fn slot_mut(&mut self, key: PgoKey) -> &mut PassPgoFrames {
        self.clock += 1;
        if let Some(i) = self.slots.iter().position(|(k, _, _)| *k == key) {
            self.slots[i].2 = self.clock; return &mut self.slots[i].1;
        }
        if self.slots.len() >= Self::MAX_SLOTS {
            let victim = self.slots.iter().enumerate().min_by_key(|(_, s)| s.2).map(|(i, _)| i).unwrap();
            self.slots.swap_remove(victim);
        }
        self.slots.push((key, PassPgoFrames::default(), self.clock));
        &mut self.slots.last_mut().unwrap().1
    }
    /// 兼容旧测试/诊断：没有 key 时取最近使用的槽
    pub fn latest(&self) -> Option<&PassPgoFrames>;
}

pub struct ExecutionChain { ..., pub pgo: PgoTable }
```

`execute_prepared_inner`：开头 `let key = prepared.pgo_key;`，`:1251-1258` 的 `self.pgo.previous` → `self.pgo.slot_mut(key).previous`（借用冲突时先 `let previous = self.pgo.slot_mut(key).previous.clone()` 一次——它是两个 HashMap，160 项的克隆可接受，或把 `plan_chunks` 调用改成传 `&PassPgoStats` 的局部引用并把 `self.parallel` 先拷出）；`:1394-1405` 与 `:1887` 的 `self.pgo.current.extend` → `self.pgo.slot_mut(key).current.extend`；`:960`/`:835` 的 `self.pgo.end_frame()` → `self.pgo.slot_mut(key).end_frame()`（key 要在 `execute_prepared_inner` 消耗 `prepared` 前取出）。

录制线程数：`ADAPTIVE` 改成函数 `ParallelRecording::adaptive_for(cores: usize)`，`max_chunks = cores.saturating_sub(1).clamp(2, 8)`，在 `vulkan.rs:1257 new_execution_chain` 里用 `available_parallelism()` 调用。

#### 影响面与验证

- `executor.rs`、`vulkan.rs:1257`、`parallel.rs`；测试 `tests/parallel_chunks.rs:138 pgo_double_buffer_uses_only_the_previous_frame_and_never_accumulates`、`rg_frame_round2.rs:78`、`rg_wave3.rs:5`、`wave3_support/mod.rs:331 seed_pgo` 直接操作 `chain.pgo.{previous,current}` → 改成 `chain.pgo.slot_mut(key)`；`seed_pgo` 增加 key 参数（测试图的 key 可以通过 `compile_graph` 返回的 `PreparedGraph.pgo_key` 取得）。
- 新测试：两张 pass 集不同的图交替执行 3 轮，断言各自 `previous` 只包含自己的 pass 名；第 8+1 个不同图触发 LRU 淘汰最旧槽。
- `render_stages`：`record_submit_ms` 下降；把 `receipt.stats.{batches, parallel_batches, chunks}`（`GraphExecution.stats`，`vulkan.rs:3056`）加进日志，确认主图 `parallel_batches > 0`。

#### 风险与回退

- 并行录制此前在生产从未真正触发过，第一次打开可能暴露录制回调里的非 `Send` 假设（回调类型已要求 `Send`，`executor.rs:942`，编译期保证）或 `DynamicConstants` 三锁争用（ANALYSIS §2.4）。回退：`parallel.strategy = Serial`（`vulkan.rs:1257` 一行）。

### A6.2 主循环线程

#### 现状

`crates/tiny-glade/src/run_game.rs:190-240`：主线程 `EventLoop::<()>::new()` → `setup_window` → `run_game_thread(config, &cli, size) -> App`（在主线程构造 App）→ `run_event_loop(event_loop, window, app, options, shaders)`。

`crates/tiny-glade/src/event_loop.rs:76-99`：

```rust
pub fn run(mut self, event_loop: EventLoop<()>) -> anyhow::Result<()> {
    let lifecycle = self.lifecycle.clone();
    self.refresh_monitors(&event_loop);
    let loop_result = event_loop.run(move |event, target| {
        if let Err(error) = self.main_loop(event, target) { ... }
    });
```

`:136-163` `WindowEvent::RedrawRequested` → `self.frame(target)`（`game_loop.rs:992-1130`，含 App.update + 渲染）；`:267-293` `AboutToWait` → `ControlFlow::Wait` + `request_redraw()`，即"渲染完才请求下一次重绘"，帧率与 OS 事件泵绑死；`:173-253` 其余 `WindowEvent` 经 `egui_io.on_window_event` 后转成 `OsWindowEvent` 入队（`queue_os_event`），在下一次 `frame` 里 `dispatch_os_event(event, target)`（`game_loop.rs:1038-1045`，`event_loop.rs:330-430`）。

winit 版本：`crates/tiny-glade/Cargo.toml:84 winit = "0.29"`，工作区 `Cargo.toml:165` patch 到 `PounceLight/winit` fork（rev `d03de0a0…`）。0.29 约束：`EventLoop::run` 必须在主线程；`EventLoopWindowTarget` 是 `!Send`；`winit::window::Window: Send + Sync`，可 `Arc` 跨线程调用（macOS 上部分方法内部派发到主线程【未核实 fork 行为】）。

`!Send` 对象：

| 对象 | 位置 | 原因 |
|---|---|---|
| `bevy_app::App` | `bevy_app-0.16.0/src/app.rs:87,1339` | `runner: Box<dyn FnOnce(App) -> AppExit>` 无 `Send` |
| `GameLoop.lifecycle` | `game_loop.rs:494` | `Rc<LoopLifecycle>`；`LoopLifecycle.first_failure: RefCell<..>`（`:52`） |
| `CoreRenderBridge.session` | `render_bridge.rs:33` | `Rc<RefCell<VulkanGraphSession>>` |
| `CoreRenderBridge.pending_build` | `:39` | `Rc<Cell<Option<GraphId>>>` |
| `CoreGraphFrame.declarer/graph` | `:495-496` | `Rc<FrameDeclarer>`、`Rc<RefCell<RenderGraph>>` |
| `GraphPasses` | `render_host.rs:2198` | `Rc<RefCell<RhGraph>>` |

结论：**不要尝试把已构造的 `GameLoop`/`App` 移进线程；在游戏线程内构造它们**，`Rc` 永不跨线程。

#### 目标改法

```rust
// event_loop.rs（新）
pub enum ToGame {
    Window(winit::event::WindowEvent),        // 原样转发给 egui_io.on_window_event + 现有 match
    DeviceMouseMotion([f64; 2]),
    Suspended,
    Resumed,
    LoopExiting,
}
pub enum ToMain {                              // EventLoop<ToMain> 的 user event
    SetCursorPosition(winit::dpi::PhysicalPosition<f64>),
    SetCursorVisible(bool),
    SetCursorGrab(winit::window::CursorGrabMode),
    SetVisible(bool),
    SetFullscreen(Option<winit::window::Fullscreen>),
    Exit(Option<String>),                      // 游戏线程结束（含错误文本）
}

pub fn run_event_loop(event_loop: EventLoop<ToMain>, window: Arc<Window>, bootstrap: GameBootstrap) -> anyhow::Result<()> {
    let (to_game, from_main) = std::sync::mpsc::channel::<ToGame>();
    let proxy = event_loop.create_proxy();
    let game = std::thread::Builder::new().name("tiny-glade game".into()).spawn({
        let window = window.clone();
        move || -> anyhow::Result<()> {
            // 全部 !Send 对象在这里构造：App、ProductionRenderer、GameLoop
            let app = crate::run_game::run_game_thread(bootstrap.config, &bootstrap.cli, bootstrap.size)?;
            let mut game = GameLoop::new(app, window, bootstrap.options, bootstrap.shaders)?;
            game.run_threaded(from_main, proxy.clone())
        }
    })?;
    let mut game = Some(game);
    let result = event_loop.run(move |event, target| {
        use winit::event::Event;
        match event {
            Event::WindowEvent { event, .. } => { let _ = to_game.send(ToGame::Window(event)); }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => { let _ = to_game.send(ToGame::DeviceMouseMotion([delta.0, delta.1])); }
            Event::Suspended => { let _ = to_game.send(ToGame::Suspended); }
            Event::Resumed => { let _ = to_game.send(ToGame::Resumed); }
            Event::UserEvent(ToMain::Exit(_)) => target.exit(),
            Event::UserEvent(m) => apply_to_window(&window, m),
            Event::AboutToWait => target.set_control_flow(ControlFlow::Wait),   // 不再 request_redraw
            Event::LoopExiting => { let _ = to_game.send(ToGame::LoopExiting); if let Some(g) = game.take() { let _ = g.join(); } }
            _ => {}
        }
    });
    ...
}
```

游戏线程侧 `GameLoop::run_threaded`：

```rust
pub(crate) fn run_threaded(&mut self, rx: Receiver<ToGame>, proxy: EventLoopProxy<ToMain>) -> anyhow::Result<()> {
    loop {
        // 1. 非阻塞吸收 OS 事件；暂停或零尺寸时阻塞等待（不空转）
        let blocking = self.suspended || frame_waits_for_window_extent(self.driver.app.world());
        let first = if blocking { rx.recv().ok() } else { rx.try_recv().ok() };
        for msg in first.into_iter().chain(std::iter::from_fn(|| rx.try_recv().ok())) {
            match msg {
                ToGame::Window(event) => self.on_window_event(event)?,     // = 原 dispatch_running_event 的 WindowEvent 分支（去掉 RedrawRequested）
                ToGame::DeviceMouseMotion(d) => ...,
                ToGame::Suspended => { self.suspended = true; self.renderer.suspend()?; }
                ToGame::Resumed => { self.renderer.resume()?; self.suspended = false; }
                ToGame::LoopExiting => return self.shut_down(),
            }
        }
        // 2. 一帧（原 frame(target) 去掉 target 参数）
        let running = self.frame()?;
        if !running || frame_limit_reached { self.shut_down()?; let _ = proxy.send_event(ToMain::Exit(None)); return Ok(()); }
        // 3. 节拍：fps_limiter.rs 已有限帧逻辑；vsync 由 present 阻塞提供
    }
}
```

`frame`/`dispatch_os_event` 里对 `target` 的两处依赖：`refresh_monitors(target)`（`event_loop.rs:532-536`，用 `target.available_monitors()`）改用 `self.window.available_monitors()`/`current_monitor()`（`Window` 方法，0.29 存在【未核实 fork】）；`target.exit()` 改为 `proxy.send_event(ToMain::Exit)`。`window.set_cursor_position`（`event_loop.rs:223`）、`set_visible`（`:141`）等在 Linux/Windows 可直接从游戏线程调 `Arc<Window>`；为 macOS 统一走 `ToMain`。

`LoopLifecycle`（`game_loop.rs:49-60`）：`first_failure: RefCell<Option<anyhow::Error>>` → `Mutex`，`Rc` → `Arc`，主线程 `record_failure` 也能用；或主线程只通过 `JoinHandle` 结果拿错误（更简单，推荐）。

`hang_watchdog.rs` 是"主线程卡死检测"（文件头注释）——线程化后它监视的应是游戏线程，需把心跳调用挪到 `run_threaded` 循环【未核实其心跳 API】。

egui：`EguiEventIo::on_window_event(&Window, &WindowEvent)`（`event_loop.rs:169`）随 `GameLoop` 一起到游戏线程；要求 `winit::event::WindowEvent: Send`——0.29 的 `WindowEvent` 含 `KeyEvent`、`Ime`、`ScaleFactorChanged { inner_size_writer: InnerSizeWriter(Weak<Mutex<..>>) }`，应为 `Send`【未核实 fork；编译器会直接报错，不会静默错】。

`TG_SMOKE_FRAMES`（`run_game.rs:230`）与 `frame_limit` 语义不变。

#### 影响面

- `run_game.rs:190-240`（`EventLoop::<ToMain>::with_user_event()`，App 构造移入线程，`GameBootstrap { config, cli: CliArgs(需 Send，`#[derive(Parser, Debug, Default)]` 普通字段应满足), size, options, shaders }`）。
- `event_loop.rs` 全文件（`run`、`main_loop`、`dispatch_running_event` 拆分）；`game_loop.rs:992 frame(&mut self, target)` 去参数；`refresh_monitors`。
- `hang_watchdog.rs`、`fps_limiter.rs`（节拍从 RedrawRequested 改为循环内）。
- 测试：`game_loop.rs:1537` 等用 `GameFrameDriver` 的无窗口测试不受影响；任何构造 `GameLoop` 的测试【未核实】需要改用新入口。

#### 验证

- 功能：`TG_SMOKE_FRAMES=60` 跑完退出码 0；窗口拖动/缩放时 `Resized` 到 `renderer.resize` 的延迟 ≤ 1 帧；关闭按钮在渲染卡顿中（人为 `sleep` 一帧 500 ms）仍能在 1 s 内退出。
- 性能：`total_ms` 不应变差；`limiter_ms` 反映节拍。
- 这项不改变 `render_stages` 任何字段的期望值。

#### 风险与回退

- macOS：Vulkan surface（MoltenVK `CAMetalLayer`）与多数 `NSWindow` 操作要求主线程；`ProductionRenderer::new` 里创建 surface 的调用会在游戏线程发生。记忆库里 macOS 已实测可跑的是主线程版本；线程化后 macOS 需重新验证，必要时 `#[cfg(target_os = "macos")]` 保留单线程路径。
- Wayland/X11 下 `Arc<Window>` 跨线程调用是允许的。
- 回退：保留旧 `run` 入口，`TG_GAME_THREAD=0` 走旧路径。

---

## A7 小项：内存预算刷新与 PGO 计时门控

### A7.1 `refresh_memory_budget`

#### 现状

`crates/rhapsody/src/device/lifetime.rs:63-84` `try_collect_garbage`：

```rust
let completed = self.reclamation_completed()?;
if let Err(error) = self.refresh_memory_budget() { log::warn!("memory budget refresh failed: {error}"); }
let budget = self.lifetime.reclamation.budget(self.memory.stats.budget_action());
...
```

`device/memory.rs:150-182` `refresh_memory_budget`：`vkGetPhysicalDeviceMemoryProperties2` + `apply_driver_heap_budgets` + `publish_memory_budget`。

生产一帧调两次：`production_renderer.rs:1135 collect_inner → self.device.try_collect_garbage()`（`drain_finished_frames`、`acquire_swapchain_image` 两处 `collect()`，所以可能是三次）和 `render_frame.rs:364 host.refresh_memory_budget()` → `production_renderer.rs:1734-1738` 直接 `device.refresh_memory_budget()`。

#### 目标改法

```rust
// device/memory.rs
pub struct MemoryBudgetRefresh { last: Mutex<Option<Instant>>, pub min_interval: Duration }   // Device 新字段，默认 250 ms

impl Device {
    /// 到期或处于预算压力时才真的查驱动；否则 Ok(None)。
    pub fn refresh_memory_budget_if_due(&self) -> crate::Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        if !self.memory.budget_extension { return Ok(None); }
        let pressure = self.memory.stats.budget_action() != crate::device::BudgetAction::None;
        let mut last = self.budget_refresh.last.lock();
        if !pressure && last.is_some_and(|t| t.elapsed() < self.budget_refresh.min_interval) { return Ok(None); }
        *last = Some(Instant::now());
        drop(last);
        self.refresh_memory_budget()
    }
}
```

`lifetime.rs:65` 改调 `_if_due`；`production_renderer.rs:1735` 改调 `_if_due`；`try_collect_all_completed`（`:88`）与启动路径保持强制版。

验证：`collect_ms` 微降（UHD 630 上 `vkGetPhysicalDeviceMemoryProperties2` 单次几十 µs 量级，收益小但确定）；`tests/device_budget.rs`、`budget_snapshot.rs` 用强制版不变；新增单测：连续两次 `_if_due` 第二次返回 `Ok(None)`。

### A7.2 PGO `Instant`/哈希门控

#### 现状

`executor.rs:1339/1353`：每个 pass 回调前后 `Instant::now()`；`:1394-1405` 每 pass 一次 `Arc::clone(name)` + `PassPgoDuration::new`，再 `PassPgoStats::record`（`:2066-2071`）做两次 `HashMap<PassPgoNameId, u64>` 操作（`:2024-2025`，默认 `RandomState`/SipHash，键是字符串）。无开关。

#### 目标改法

- 门控：`ParallelRecording` 增加 `pub pgo: bool`；`SERIAL` 为 `false`，`adaptive_for` 为 `true`。`execute_prepared_inner` 开头 `let pgo_on = self.parallel.pgo && backend.supports_parallel_recording();`，`:1339/1353` 与 `:1394-1405`、`:1887` 全部 `if pgo_on`。串行配置下零成本。
- 哈希：`PassPgoStats` 的两个 map 改 `HashMap<PassPgoNameId, u64, BuildHasherDefault<xxhash_rust::xxh64::Xxh64>>`【未核实 `xxhash-rust` 是否提供 `Hasher` impl；若无则用 `nohash-hasher` 配合把 `PassPgoNameId` 改为 `Arc::as_ptr` 的整数键——同一计划缓存命中时 `pass_names` 的 `Arc<str>` 是同一组指针（`:1399 from_shared(Arc::clone(..))`），计划重建时只是统计失一帧】。

量级说明：160 pass × (2 × `Instant::now()` ≈ 50 ns + 2 次 SipHash ≈ 100 ns) ≈ 25–50 µs/帧。**这是 A 层里最小的一项，只在顺手改 A6.1 时一起做；不要单独排期。**

验证：`tests/parallel_chunks.rs`、`rg_frame_round2.rs:78`、`rg_wave3.rs:5` 继续通过；`record_submit_ms` 变化在噪声内。

---

## 附：各项与 `render_stages` 字段对照

| 项 | 预期下降字段 | 新增字段/计数 |
|---|---|---|
| A1 | `declare_ms`、`resolve_inclusive_ms`（去掉嵌入的 GPU 等待）；`total_ms` ≈ max(CPU,GPU) | `frame_throttle_ms` |
| A2 | `declare_ms`（去掉 fence 往返） | `upload_flush_ms`、`upload_bytes`、`upload_calls`、`capacity_waits` |
| A3 | `prewarm_ms` → ≈0（稳态） | — |
| A4 | `declare_ms`；`vao_full_refresh calls` 9→2→1 | — |
| A5 | `update_ms`（`game_loop.rs:428`） | — |
| A6.1 | `record_submit_ms` | `batches/parallel_batches/chunks`（来自 `receipt.stats`） |
| A6.2 | 无（响应性） | — |
| A7 | `collect_ms` 微降 | — |

## 附：实施前必须 grep 确认的【未核实】清单

1. tiny-glade 侧 `DeviceConfig { .. }` 构造点与 `frames_in_flight()` 的其他消费者（`vulkan.rs:828` 环槽）。
2. `transfer/queue.rs` 的 `enqueue` 同 key 合并规则（覆盖 vs 追加）。
3. 录制回调内是否有 `update_buffer_bytes`/`gpu_write`。
4. `GroundedBuildingsSsbo` 的 derive、`WallId: Ord`、`rasterize_buildings.cs` 对元素顺序的依赖。
5. `ComputeEntry/RasterEntry` 的票据字段名。
6. `world_lighting/world_surface/world_postprocess` 四个函数是否有测试直接调用。
7. `dyn GraphicsDevice` 的 `Send + Sync` bound。
8. winit fork 的 `Window::available_monitors`、`WindowEvent: Send`、macOS 跨线程窗口调用。
9. `hang_watchdog` 心跳 API。
10. `xxhash-rust` 的 `Hasher` 实现。
