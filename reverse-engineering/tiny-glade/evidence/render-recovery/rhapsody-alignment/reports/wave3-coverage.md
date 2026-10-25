# wave3 覆盖补齐：校验层 / 辅助队列 / 大图并行分块 / candidate-optimizations / Windows 外壳

范围仅 rhapsody 内部。本批没有 git commit。环境：lavapipe，单个通用队列族；磁盘一度写满（见末尾）。

## 改动的文件（均为本包拥有的文件）
- `crates/rhapsody/src/raw/instance.rs`：新增 `InstanceBuilder::enable_validation()`、`KHRONOS_VALIDATION_LAYER`、`DebugMessage`、`ValidationCollector`、`ValidationMessenger`。
  - `enable_validation()` 总是启用 `VK_EXT_debug_utils`；`VK_LAYER_KHRONOS_validation` 存在才加入层列表，返回是否启用。
  - `ValidationMessenger` 订阅 WARNING+ERROR，把消息收集到可查询列表（`errors()/warnings()/all()/clear()`），Drop 时销毁。
  - 它还提供 `submit()`（走 `vkSubmitDebugUtilsMessageEXT`），不需要任何层就能验证收集器本身。
  - 旧的 no-op `vulkan_debug_messenger` 保持不变。
- 新测试：`tests/wave3_validation.rs`、`tests/wave3_queues.rs`、`tests/wave3_parallel_graph.rs`、装置 `tests/wave3_support/mod.rs`。
- `Cargo.toml` 未改。

## 逐项结果

### 1. 校验层 —— ⚠️ 部分（本机无层，收集器本身已覆盖）
- 本机确认：`/usr/share/vulkan/{explicit,implicit}_layer.d` 只有 `VkLayer_INTEL_nullhw`、`VkLayer_MESA_overlay`、`VkLayer_MESA_device_select`；全盘没有 validation 的 json；无网络，装不了。
- ✅ 收集器：`the_collector_records_messages_by_severity_without_any_layer`，ERROR 1 条、WARNING 1 条，INFO 被订阅过滤，`clear()` 生效；创建实例与设备后收集器为空。
- ✅ 构建选项：`the_builder_enables_the_validation_layer_only_when_it_exists`，层是否加入与 loader 枚举一致（本机为 false），debug_utils 总是启用。
- ❌ 无法在本机验证：`deliberate_api_misuse_produces_a_validation_error_when_the_layer_exists`（`vkCreateBuffer size=0`）。无层时它打印“跳过：无校验层”并返回。
- ⚠️ `the_animation_and_the_big_graph_run_without_validation_errors`：直接录制的三角形动画 12 帧逐像素校验，加 40-pass 图（串行与并行各 2 帧）逐字节校验，全部通过。0 条 error 的断言只在有层时执行；本机只证明了流程在带收集器的实例上正常，收集到 0 条 error 仅说明 loader 没报错，不能当作同步正确性证据。
- 在有层的机器上运行：
  `VK_LAYER_PATH=<含 VkLayer_khronos_validation.json 的目录> cargo test -p rhapsody --test wave3_validation -- --nocapture`
  （层随 Vulkan SDK 或 `vulkan-validationlayers` 包提供；有层时 4 个测试都会走完整断言。）
- 限制：校验层测试没有包含 `triangle_animation_graph` 的完整 48 帧流程。该文件的 `Harness` 是私有的，且 `draw_support::Rig::new` 固定用 `create_headless_device()`，无法注入带层的设备，而这两个文件不归本包。我改为用 `wave3_support::rig_on(device)`（复制 `Rig::new`，换成带层设备）跑直接录制版，另加 40-pass 渲染图。建议 `draw_support` 的 owner 增加 `Rig::on(device)`，之后即可复用完整的 48 帧测试。

### 2. 辅助队列 / 跨队列族 —— ⚠️ 部分（真实跨族执行在本机不可能）
- 本机：1 个队列族，数量 1，flags `GRAPHICS|COMPUTE|TRANSFER|SPARSE_BINDING`，没有独立 compute 族，`additional_queues` 为空。
- ✅ `queue_families_are_listed_and_auxiliary_queues_exist_only_for_an_independent_compute_family`：打印所有族。断言 `additional_queues.len()` 等于独立 compute 族（`flags&3==COMPUTE`）的队列总数，主队列自身除外，且这些队列的族都属于它们。无辅助队列时 `new_for_queue(Auxiliary(0))` 必须失败，即不伪造辅助队列。
- ✅ `an_import_owned_by_another_queue_family_is_rejected_without_a_release_acquire_plan`：导入资源归属的族号不等于图所在族时，`execute_frame` 返回含 “release/acquire” 的错误，会话不被 poison。用不存在的族号触发，走的是真实的 `validate_graph` 规则。
- ❌ `a_graph_runs_on_the_auxiliary_queue_when_the_device_has_one`：本机打印“跳过：设备没有辅助队列”。代码已写好并通过编译，但从未实际执行过。内容为辅助族归属的缓冲 `cmd_fill_buffer` + `cmd_copy_buffer` 两个 pass，读回比对，辅助队列时间线计数 >0，另有主族资源被拒绝的断言。
  - 这段代码未经任何设备验证，在有独立 compute 族的机器上首次运行时可能需要调整。
  - 没有真正的“compute 着色器 dispatch”，用的是传输命令；`import_families` 的 release/acquire 迁移只验证了“拒绝”一侧，“放行并迁移”一侧没有实现可测。

### 3. 多 pass 大图并行分块 —— ✅
图：32 个独立填充 pass，加 8 个拷贝 pass（读第 2k 个填充缓冲，产生跨块依赖和屏障），共 40 pass、1 个 batch。测试 `all_chunking_strategies_agree_byte_for_byte_and_match_the_budget_algorithm`（实测）：
- (a)/(b) 串行、`batch_per_pass`、原作分块（ComputePool 与 Inline）的 40 个缓冲逐字节一致，且都等于期望的常量与拷贝内容。
- (c) 预置上一帧统计（填充各 10ns、拷贝各 20ns）：total=480，max=20，budget=`clamp(20, 480>>4=30, 480>>1=240)`=30。手算分块为 10 块×3 个填充，加 `30..33`，再 `33..35, 35..37, 37..39, 39..40`，共 15 块。`original_chunks` 与手算一致，执行器实际 `stats.chunks==15`，命令缓冲数 16（块数+1，多出的是执行器先开的首命令）。
- 第一帧无统计：budget=0，每 pass 一块，`parallel_batches=1, chunks=40`，命令缓冲 41。
- 串行：`batches=1, parallel_batches=0, chunks=0`，命令缓冲 2（首命令+batch 命令）。
- `batch_per_pass`：`batches=40, parallel_batches=0`，`snapshot.batches.len()==40`。
- 屏障：串行与 15 块并行的 `barrier_commands=10`、`barrier_transitions=88`，两种 runner 都相同。
- (d) 并行确实发生：`ExecutionStats.parallel_batches=1`；填充回调实际运行的线程数 ComputePool=3（40 块时 5），Inline=1。线程数只打印，不作断言（调度相关会有波动）。
- `the_second_frame_chunks_by_the_first_frames_measured_durations`：第 1 帧实测统计覆盖全部 40 个 pass，第 2 帧实际块数等于按该统计用 `plan_chunks` 预测的块数（本机实测 4–5 块，随计时波动，断言只比较两者相等）。第 2 帧内容是新的，没有读到上一帧残留；同一会话切到串行后结果逐字节一致。
- `balanced_and_adaptive_candidates_match_the_serial_bytes`（需 `candidate-optimizations`）：
  - `Balanced{max_chunks:4}` 有/无统计都是 4 块，块数等于 `plan_chunks` 预测，字节与串行一致，命令缓冲=块数+1。
  - `ADAPTIVE` 无统计时走串行，480ns 统计（低于 200µs 门槛）仍走串行，统计放大 1000 倍到 480µs 才分块，且仍是 15 块，字节与串行一致。

### 4. `candidate-optimizations` 开/关对比 —— ✅
`cargo test -p rhapsody <相关 6 个目标>`：

| 构建 | parallel_chunks | triangle_animation | triangle_animation_graph | wave3_parallel_graph | wave3_queues | wave3_validation |
|---|---|---|---|---|---|---|
| `--no-default-features --features suballocation` | 11 passed | 1 | 2 | 2 | 3 | 4 |
| `--all-features` | 13 passed | 1 | 2 | 3 | 3 | 4 |

两种构建的 0 失败。直接版与渲染图版三角形动画（逐像素、质心、历史纹理）均通过。大图在两种构建下都等于同一组期望常量，所以结果逐字节相同。开启 feature 后多出的 2 个 parallel_chunks 测试和 1 个 wave3 测试是 `Balanced`/`ADAPTIVE` 专属。

注意：`VulkanGraphSession::new` 在开启该 feature 时默认用 `ADAPTIVE`（有 200µs 门槛），而关闭时用 `ORIGINAL`。测试里凡要确定走分块的地方都显式设置 `session.chain.parallel`。

### 5. Windows 外壳编译检查 —— ⚠️ 部分（lib 的 msvc `check` 已验证；windows-gnu 无法验证）
- `rustup` 只装了 `x86_64-unknown-linux-gnu`，`rustup target add` 无网络失败（`static.rust-lang.org` 连接失败）。
- 用 `RUSTC_BOOTSTRAP=1 cargo check -Zbuild-std=std,panic_abort --target x86_64-pc-windows-msvc -p rhapsody`（`/tmp/m-ustc.toml` 取 std 的依赖）绕过。
- `x86_64-pc-windows-gnu` 不行：`build-std` 编译 `std` 时要 `x86_64-w64-mingw32-dlltool`，本机没有。
- msvc 目标（未改动的源码）共 2 个编译错误，均为 edition 2024 的 “extern blocks must be unsafe”：
  - `src/raw/aftermath.rs:286`，`aftermath::windows_api` 里的 `extern "C" { fn GFSDK_Aftermath_EnableGpuCrashDumps … }`
  - `src/raw/dxgi.rs:145`，`dxgi::SystemDxgi` 用到的 `extern "system" { fn CreateDXGIFactory … }`
- 最小修复：这两处改为 `unsafe extern "C" {` / `unsafe extern "system" {`。我没有改这两个文件，只在 `/tmp` 的工作区副本上试过。副本上 `cargo check ... --target x86_64-pc-windows-msvc -p rhapsody`（lib）通过，没有其他错误。副本和 msvc 构建产物已删除。
- 没验证：Windows 目标的测试（`--tests`）编译、链接（`raw-dylib` 的实际链接）和运行。`aftermath.rs:280` 的注释“未在本机编译验证”现在可以更新为“lib 层面已用 build-std 检查，修复上述 2 处后通过”。

## 发现的 bug / 异常与处理
1. **Windows 外壳 2 个编译错误**（上节）。不在本包拥有的文件内，按规则只报告，没有改。复现：上面的 `cargo check -Zbuild-std ... --target x86_64-pc-windows-msvc` 命令。
2. 未发现 `rg/**`、`raw/**` 的行为 bug。大图在串行、batch_per_pass、原作分块（ComputePool/Inline）和 `Balanced`/`ADAPTIVE` 下结果逐字节相同，跨块拷贝依赖和屏障正确。
3. 观察，不是 bug：串行批次的命令缓冲数是 2，其中一条是执行器先开的“首命令”（承接初始屏障），这与“并行 = 块数+1”的结构一致。
4. 测试装置的限制：`wave3_support::take_thread_count` 用全局静态收集线程，多个测试并行时会互相污染，只用于打印，没有断言。

## 全量验证
`cargo test --offline --config /tmp/m-ustc.toml -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）：**58 个目标，1875 passed，0 failed**。其中 58 个目标包含并行的其他 agent 新增的测试（`wave3_submit`、`wave3_device_order` 等），不全是本包的。第一次全量运行因其他 agent 的 `wave3_submit.rs` 暂时编译不过，等待后重跑通过。

## 环境事故
中途磁盘写满（`/tmp` 设备 0 空闲，msvc 的 `build-std` 构建加其他 agent 的构建共同造成），一个 `syn` 构建失败。我立即删除了自己产生的 `/tmp/tg-target/x86_64-pc-windows-{gnu,msvc}` 和 `/tmp/w3-win`，之后空间恢复，后续全量测试正常。若其他 agent 在那一刻有构建失败，原因是这次磁盘满。
