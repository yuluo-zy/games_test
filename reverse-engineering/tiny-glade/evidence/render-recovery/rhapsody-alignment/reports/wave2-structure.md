# rhapsody 第二批结构对齐（销毁桶 / 交换链锁 / track panic / 物理设备挂接 / 槽内联评估）

日期 2026-10-15。图例：✅原版对齐[证据地址]　⚠️候选　❌读不出。
验证：`cargo test --offline -p rhapsody -p tiny-glade -p country-core --all-features --no-fail-fast` = 1682 passed / 0 failed（lavapipe，含 `original_oracle*`、`managed*.rs`、新增 `wave2_structure`）。全工作区另有 asset/asset-pipe/shader-compiler/tg-text 的既有失败（缺发行数据文件/非本包，与本批改动无关，未碰）。

## 1 销毁桶归属
- ✅ 原版桶表在 `GraphicsDevice+0xa50`，`ManagedResources+0xa50` 是 u16 owning_device，`take_out_dropped_waste(&ManagedResources,&GraphicsDevice)` [managed.md 事实 3；0x1418c0a70 `lea rdi,[rsi+0xa50]`]。
- 改动：
  - `GraphicsDevice.disposal` 由 `Mutex<CommandBufferDisposalBuckets>` 改为 `Arc<CommandBufferDisposalBuckets>`（内部本来就是 `RwLock<HashMap<..,Mutex<Bucket>>>`，去掉重复外层 Mutex）；`disposal()` 返回 `&CommandBufferDisposalBuckets`（无外部调用方）。
  - `SharedResources.buckets` 改为 `Arc<...>`；`ManagedResources::with_device_buckets(arc)`（设备构造时共享设备的表）；新增 `ManagedResources::take_out_dropped_waste_on(&GraphicsDevice)` 与 `GraphicsDevice::take_out_dropped_waste()`，桶表取自设备参数（dispose 任务第二参数 = 调用方桶表，对应原版 `&GraphicsDevice` 传参）；新增 `owning_device_id() -> u16`。
  - `wait_idle_and_recycle_resources` 原来对 **另一张永远为空的** 设备表做“逐序号排空”（等于无效），现改为原版顺序里的 `take_out_dropped_waste(resources, self)`（0x140bf8e60 → 0x1418c0a70）；桶只由 `retire_in_order` 按完成序号排空。
- ⚠️ 偏差：Rust 里 `ManagedResources` 不能回指所属设备，所以设备拥有表、资源侧共享一份 `Arc`；脱离设备的独立 store（`ResourceManager::new()`、`rg/vulkan.rs` 的 `stores`、tests/managed.rs）持有私有表作“替身设备”，旧 API（`begin_command_bucket`/`take_out_dropped_waste()`/`drain_completed_sequence` 等）全部保留且行为不变。`rg/vulkan.rs` 未改（它自建 `ManagedResources::new()`，不经设备）。
- 测试：`wave2_structure::{graphics_device_owns_the_destruction_bucket_table, dropped_resource_goes_to_the_device_bucket_and_retires_with_the_sequence, owning_device_id_is_per_resource_table...}`。

## 2 交换链锁结构
- ✅ 锁取法按机器码（ALGORITHMS §1.2 / §6.5 与 managed.md 事实 9，stock parking_lot 0.12.1，写 0→8/读 +0x10）：`desc` 0x1418cb100 读锁；`resize` 0x1418cb1b0、`set_vsync` 0x1418cb370、`acquire_next` 0x1418cb520、`present` 0x1418cc1b0、`retire` 0x1418ccb50、`on_focus`（内联 `RwLock::write`）全取写锁。
- 改动：`raw/swapchain.rs` 两把 Mutex + 两个 AtomicBool → 单个 `parking_lot::RwLock<SwapchainInner>`。`SwapchainInner{desc(@0x08), state(@0x30，Created=2/NotCreated=0), leases(@0x60，每图一槽，len=image count@0x68), next_slot(@0xb0), stale(@0xa8 状态字节的候选对应), any_presented}`。acquire 整个持写锁（持锁内再取设备 queue_lock，顺序同原版“写锁 → 0x718 Mutex”），present 同序；因锁不可重入，acquire 体拆为 `acquire_image_locked(&mut inner)`，状态变更都走传入的 inner（`acquire_result` 改取 `&mut bool`）。`Swapchain` 公开字段/方法签名未变，`managed/swapchain.rs` 无需改动。
- ⚠️ 未对齐：字段是安全 Rust 容器，偏移 0x30/0x60/0x68/0xa0… 不做 `repr(C)` 断言；租约表 0x38 B/项的内部字段、@0xa8 的 2/3/4 三态含义、@0x34 旧 desc 旋转、`mark_stale`/`is_stale` 的取锁方式（原版内联转发，未读出，这里 mark=写、is=读）为候选；句柄/格式/图像/视图仍为 `Swapchain` 的公开字段（创建后不变，重建产生新 `Swapchain`），原版在 inner @0xa0。
- 测试：`swapchain::tests::swapchain_state_lives_in_one_rwlock_with_original_state_value`（读并发/写独占、状态值 2）。Vulkan 窗口系统路径在无 surface 的环境无法回放，acquire/present 的锁序只由编译与既有单元测试覆盖。

## 3 `track` 遇 tag≠1 panic
- ✅ `RawManagedHandle::track` 0x140bd6a70：校验和不等 → 静默返回；`[cmd]!=1` → panic，`fmt::Arguments` 片段 0x142b14b7b（51 B）= `"Can't use a resource for a command buffer in state "`，参数是 `CommandState` 的 Debug（0x140bed620：tag0 `Initial`、tag1 `Recording{queue,seq}`、tag2 `Recorded{..}`、tag3 `Submitted{..}`，tag4..7 跳表不可达）；否则 `fetch_max`。
- 改动：`tracking::{track_sequence, track_handle}` 改为 panic（`track_handle` 返回 `TrackOutcome`），保留 `try_track_sequence/try_track_handle`（Err）；`Managed::track`（panic）+ `try_track` + `track_outcome`；`track_on` 保持 Result（走 `try_track`，供 `rg/pass_bindings.rs` 与 `get_tracked` 的 Result 路径不变）；`TrackGpuLifetime` 增 `try_track`（必需）、`track`（默认 panic）；`GpuOwned::track_unwrap -> R`（panic）+ `try_track_unwrap`，且顺序改为机器码序“先 track（0x1415fb600）再置 Tracked（0x1415fb605）”；`ManagedRef::get_tracked -> &T`（panic）+ `try_get_tracked`；`lifecycle.rs` 的 runtime 记账层改用 `try_track_handle`（它本来就返回 Result 并映射 `InvalidRecordingState`）。`GpuOwnedState` Owned/Tracked/Unused 及 `Unused` 命名不变。
- oracle：`gpu-lifecycle` 重放期望 tag≠1 为 `panic`（`ok_or_panic` 把 Err 当 panic）；`tests/original_oracle.rs` 改为用 `catch_unwind` 判定，期望一致、无回归。`tests/managed.rs` 把 tag≠1 期望从 `Err` 改成 `try_track` 为 Err + `track` panic。
- ⚠️ panic 文案里 `Recording { queue: .., seq: .. }` 的 `queue` 字段：本投影 `CommandState{tag,sequence}` 没有该字段，文案缺 `queue`，其余逐字对原版。
- 测试：`tracking::tests::{track_handle_panics_when_state_is_not_recording, track_sequence_panics..., mismatched_generation_never_panics_even_for_bad_state}`、`handle::...::track_unwrap_panics_on_bad_state_like_the_original`、`wave2_structure::track_panics_on_non_recording_state_but_try_track_reports_err`。

## 4 `attach_physical_device_info` 接线
- ✅ `GraphicsDevice+0x658` 指向物理设备包装 [device.rs 字段表 0x140bf84c0]。
- 改动：`GraphicsInstance.devices: Vec<Arc<PhysicalDevice>>`（`enumerate_physical_devices` 仍返回 `Vec<PhysicalDevice>`，兼容）；新增 `GraphicsInstance::create_graphics_device(&Arc<PhysicalDevice>, family, ext) -> Result<Arc<GraphicsDevice>>`（`create_device` + `GraphicsDevice::finish_creation`（改 `pub(crate)`）+ `attach_physical_device_info`）与 `GraphicsDevicesForSurface::create_graphics_device_by_uuid`；`create_device_by_uuid` 抽出 `uuid_choice` 共用。旧 `create_device`/`create_device_by_uuid`（只返回 `VulkanDevice`）保留。`budget_snapshot.rs` 等既有测试经 Arc 自动解引用，无需改。
- 测试：`wave2_structure::create_graphics_device_attaches_the_physical_device_wrapper`（`Arc::ptr_eq`、二次挂接被拒、`refresh_memory_budget` 委托并发布快照；lavapipe 无 memory_budget 时断言 `Ok(None)`）。
- ⚠️ 旧入口 `GraphicsDevice::create(instance, vk::PhysicalDevice, family)` 与 `from_device` 仍不挂接（无 `Arc<PhysicalDevice>` 可用）；需要委托的调用方须走新入口。

## 5 槽内资源内联评估（只评估，未改代码）
现状：槽 = `Slot{control:Arc<ControlBlock>, next_free, handle:ArcSwapWeak<ManagedInner<T>>}`，资源在 `ManagedInner.resource: Arc<T>`；`Managed<T>(Arc<ManagedInner<T>>)`。原版 `ManagedResource<T>` 槽（0x28 B 例：`{seq@0,refcount@8,checksum@0xc,resource@0x10..}`），句柄只是 `{ctx 指针, 槽指针, checksum, device, tag}`，资源按值住在槽里，dispose 时 bitwise 拷出。
- 要改的类型：`Slot<T>`（资源内联，`AppendOnlyVec` 元素地址稳定，是可行的前提）、`ManagedInner`/`Managed<T>`（句柄改成 `{store Arc, index, checksum}` 的轻量句柄，`Deref` 需经 store 取 `&T`，生命周期受借用限制）、`ResourceKey`/`enqueue_retirement`（task 现在 move 一个 `Arc<T>`，改为 dispose 时从槽里 `ptr::read` 出 T，需要 unsafe 与“槽空闲时资源未初始化”的状态机）、`ControlBlock`（现被 `LegacyReceipts` 以 `Weak<ControlBlock>` 持有，需改指向槽）、`views` 视图缓存、所有 `Managed<T>` 使用点。
- 对所有权模型的影响：目前“资源存活 = Arc 计数”与“CPU 句柄数 = ControlBlock.references”是两套并存计数；内联后只剩 references 一套，`Managed<T>: Deref<Target=T>` 要么返回绑定 store 借用的守卫（破坏 `Managed<T>: Clone + Send + 'static` 的现有用法：几十处 `Managed<Texture>` 跨线程持有、`Arc<Swapchain>`/`allocate_shared(Arc<T>)` 共享路径、`rg` 的 `import_unowned`），要么保留 `Arc<Store>` 在句柄里并 unsafe 取指针。`allocate_shared(Arc<T>)` 与 swapchain 重建（`current.0.resource.clone()` 返回 `Arc<Swapchain>`）这类“同一资源两个托管句柄”的用法与内联按值语义直接冲突。
- 风险：高。涉及 unsafe 初始化/移出、AppendOnlyVec 槽复用的竞态（现版靠 ArcSwapWeak 保证读者无锁且不复活）、Drop 顺序（资源 Drop 里持有 `Arc<VulkanDevice>`）、跨 crate（country-core 通过 `Managed<T>` 公共 API）。收益：仅内存布局对齐（原版结构一致性），行为（refcount/checksum/seq/复用顺序）已对齐，且无功能或性能证据表明 Arc 间接层是瓶颈。
- 结论：**不值得现在做**，也不属于局部低风险改动，故未动代码。若日后必须做的最小步骤：① 先去掉 `allocate_shared`/swapchain 双句柄用法（swapchain 重建改为在旧槽上换新资源）；② 在 `ResourceState` 内引入 `MaybeUninit<T>` 槽并仅对“无共享需求”的 5 类（buffer/sampler/descriptor_set/framebuffer/render_pass）试点，句柄保持 `Arc<ManagedInner>` 外形只把 `resource: Arc<T>` 换成“指向槽的裸指针 + store Arc 保活”；③ dispose 路径从槽 `ptr::read` 并保持校验和判定序不变；④ 逐类迁移，以 `managed*.rs`、`original_oracle*` 与并发 churn 测试做回归。

## 6 与既有结论的一致性
`GpuOwnedState{Owned=0,Tracked=1,Unused=2}`、`Unused` 命名、`ResourceTag::TextureView=3 → todo!()`、栈桶 0xF0、`+0x0f` 无写入方、空闲链表 LIFO/CAS、`take_out_dropped_waste` tag 3 `todo!()`：均未改动，原测试全通过。

## 改动文件
`raw/device.rs`（disposal 字段/`disposal()`/`take_out_dropped_waste()`/`wait_idle_and_recycle_resources`/`finish_creation` 可见性，未碰 builtin_samplers 三处）、`raw/swapchain.rs`、`raw/init.rs`、`raw/managed/{resources,tracking,handle,lifecycle}.rs`、`tests/{original_oracle,managed}.rs`（断言随 API 变化）、新增 `tests/wave2_structure.rs`。`physical_device.rs`、`disposal.rs`、`managed/swapchain.rs`、`rg/vulkan.rs` 无改动。

## 遗留 / 对其他 agent 的需求
- `device_api.rs`（非我拥有）的 `create_*` 仍直接 `self.resources.xxx.allocate(..)`，与本批兼容，无需改。
- 若 country-core 用 `ManagedRef::get_tracked`/`track_unwrap`/`Managed::track` 的 Result 形态，需改调用 `try_*`（本仓库当前无此类调用）。
- 未证实：swapchain 租约表项内部字段、@0xa8 三态含义、`mark_stale` 取锁；`CommandState` 的 `queue` 字段。
