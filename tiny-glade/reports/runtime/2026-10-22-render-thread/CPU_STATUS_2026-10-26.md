# 渲染 CPU 当前状态与后续任务（2026-10-26）

**CPU 16ms 目标尚未达成。最新发布构建 V31 的干净测量：静态场景渲染线程均值 8.24ms/帧，大量绘制 22.66ms/帧；重场景距目标约 6.66ms。当前提交还包含 V31 冻结后完成的投影与压力回收修复，这部分尚未测出发布构建收益。**

## 最新实测

1280×720、world scale 1.0；真实 GameLoop/窗口/WSI；每场景预热600帧后测1000帧；每场景一组逆序 V31→V30；相同 schema2 场景、固定时钟与资源输入签名。关闭阶段日志、GPU timestamp、CPU栈采样；从进程外读取 Darwin 线程用户态+内核态 CPU 累计值。GPU执行及休眠不计入，忙等仍计入。

| 场景 | V30渲染线程CPU均值 | V31渲染线程CPU均值 | V31主线程CPU均值 | V31进程CPU工作量/帧 |
|---|---:|---:|---:|---:|
| 静态 | 14.66ms | **8.24ms** | 1.73ms | 20.49ms |
| 大量绘制 | 19.84ms | **22.66ms** | 2.02ms | 48.84ms |

进程CPU工作量包含并行线程，不能与渲染/主线程相加，也不能等同于帧关键路径。静态本组下降约43.8%，大量绘制本组上升约14.2%；只有一组、轮次波动明显，**尚无稳定收益结论，也没有证明新代码导致该全部变化**。V31还改变了macOS渲染线程QoS，不能将差异全部归因于算法。不得选取旧轮次较慢值拼接成漂亮收益。

单独开启完整诊断的1000帧CPU分布如下，含日志、统计和探针开销，不能当作干净生产p95：

| 场景 | 诊断CPU均值 | 诊断CPU p95 | 与同范围OS线程计数的均值差 |
|---|---:|---:|---:|
| 静态 | 16.67ms | 22.17ms | 约0.004% |
| 大量绘制 | 33.46ms | 43.93ms | 约0.11% |

逐帧值来自 `CLOCK_THREAD_CPUTIME_ID`，覆盖相邻渲染完成点之间实际owner CPU，包括先前日志/控制/轮询；不含其他worker/driver线程。后处理应按主图receipt关联观察帧，不能在失败、取消或切App时假设gameplay计数等同世界观察帧。当前两组各1000帧已逐receipt核对编号对应，见 [离线关联核对](performance-2026-10-24/cpu-clock-receipt-crosscheck-20261026.json)；干净均值来自独立OS计数，不依赖该编号假设。

证据：[对照摘要](performance-2026-10-24/cpu-occupancy-ownership-first-comparison-20261026.json)、[原始轮次索引](performance-2026-10-24/cpu-occupancy-ownership-measurements-20261026.json)、[V31构建指纹](performance-2026-10-24/production-v31-performance-build.json)。V31二进制SHA256：`43e73926188d82bccbab767f5276695b135d2ee74b3cadbece746ffdee549e82`。冻结源码归档、dSYM、原始日志与大栈文件保留本机；提交保存摘要、构建身份及当前CPU快照，未把发布二进制或符号归档放入Git。

## 本次已完成的代码

- 资产上传统一返回接纳结果；positions/data失败保脏、提取前重试，未全部接纳不发布新实例计数。原来的吞错/try双入口已删除。
- CPU binding别名改为u32并迁移Raster/Compute消费者；GPU descriptor容量和ABI不扩大。Raster使用显式binding-access映射，删除二次反向搜索。
- Nani Canopy/Trunk维护live mesh-bin频数，按事件更新安全命令上界；保留任意可见子集及LOD重排的正确性。拒绝按原CPU分组收紧这种不安全算法。
- 提交资源使用集在命令结束后封闭，合并/加锁只接收有序类型；删除每次提交全量复查。短集合保留inline scratch；scratch池进一步复用仍未完成。
- 删除没有稳定收益的descriptor绑定缓存及失效接口；生产只保留真实原生绑定路径。参数CPU存储、预算credit和租约生命周期继续使用当前统一架构。
- 删除每次descriptor租约变更的全池观察锁，唯一 `sampled_occupancy` 按请求逐heap采样；预算、槽generation和真实GPU lease保留。不再把逐页快照宣称为全池同一时刻或历史peak。
- Collect首次binding域接管worker容器、draw原地重定位，避免重复增长；保持原访问前缀、alias pruning与Commit顺序。PreparedDraw顶点槽使用inline容量，补丁排序改为唯一全key的不稳定排序，复现原稳定顺序。
- 墙发布统一为真实资源/路径链，删除历史发布缓存和consume旁路；静态帧避免无效修改tick。Prefab与atlas装配顺序确定化，测量夹具位于项目tests中。
- ECS固定并行执行器并传播初始化错误；删除可选并行feature、单线程空开关和旧默认fallback。删除无消费者启动/专用线程助手、空壳绑定接口、默认零part构造及历史别名。
- macOS常驻渲染线程启动时一次请求USER_INTERACTIVE QoS。正常构建没有逐帧CPU时钟；当前诊断时钟仅在profile+macOS且完整timing启用时运行。

### V31冻结之后、本提交新增

- frame import退役仅撤销当前CPU逻辑snapshot；最多复用两种idle projection形状，仍受既有snapshot/CPU字节预算限制。plan地址只是非拥有选槽提示，当期资源仍按真实物理身份解析；外部Arc/Weak快照不修改。
- projection只缓存CPU包装，但会持有实际GPU owner；显式释放、压力回收、重置及销毁仍释放它们。32MiB限制的是CPU投影元数据，不是所持GPU字节。
- 内存压力回收不再用“cache报告释放字节为零”推断未释放owner；沿同一可信完成快照销毁可回收对象。只有内存分配失败才最多重试一次，不重试上传或原生提交。子分配重新检查真实容量；不新增GPU idle等待。单趟回收不承诺清尽析构中新入旧队列的所有对象。
- 删除无人使用的 `ExecutionBackend::wait_submission`、`--single-threaded-executor`、`--disable-vram-budget`旧别名，以及入参契约已禁止的同步映射上传fallback。
- indirect资源临时列表改为固定两槽；每pass FrameConstants只在首次原位置声明，不重复每draw同一严格读。一般资源访问和冲突处理仍走统一账本。

这些新增代码的收益必须重新冻结发布构建后测量，不能使用V31数值代表提交最终源码。

## 验证范围

V31之前已完成：rhapsody433、核心库2887、应用库271项通过；真实GPU ABI/保活、30帧游戏退役/像素整链、文字GPU和工作区all-target检查通过。详细范围、失败记录与补测见 [V31验证索引](performance-2026-10-24/cpu-occupancy-ownership-verification-20261026.json)。历史通过结果不等同于本次最终源码全量重跑。

本提交最后一批新增代码的定向CPU/GPU测试及全工作区检查记录在 [提交前验证](performance-2026-10-24/cpu-prepush-verification-20261026.json)，以该文件实际退出码及下方最终记录为准；不把filtered=0或GPU初始化跳过算通过。

## 后续任务，按依赖推进

1. **先分离干净逐帧CPU测量。（代码已落地 2026-10-26）** 用唯一profile诊断入口替换当前依赖完整timing日志的时钟采样；构造时一次分配有界记录缓冲，每次反馈完成仅读取时钟和保存累计计数，结束后输出。记录World/主图receipt/attempt并关联真实观察帧；失败推进基线、跨App重置，截断明确报错。普通构建无新增验证层或时钟开销。
   - 已实现 `crate::render_cpu_probe::CpuIntervalRecorder`（`crates/tiny-glade/src/render_cpu_probe.rs`）：构造时 `try_reserve_exact` 一次分配有界缓冲，满后置 `truncated` 标志并计 `dropped`（显式截断报错，不静默丢尾）；时钟回退记为 `ClockRegressed` 错误样本并重新播种基线，不 panic；`reset_baseline` 跨 App 清零；每条样本带 `frame`/`receipt`/`attempt`/`interval_frames`/`cpu_ns`/`kind`；`to_json("<label>")` 输出 schema `render-cpu-clean-v1`（含 clean 均值/p95、truncated/dropped、逐 receipt 样本）。
   - `ProductionRenderer` 接线（`production_renderer.rs`）：`cpu_recorder` 字段 + `install_cpu_recorder`/`take_cpu_recorder`/`reset_cpu_recorder_baseline`/`record_cpu_interval`；在 `finish_render` 完成点调用，**不经 `log::info!`、不受 `frame_timing_every` 门控**；`observed = first_error.is_none() && !cancel_preparation`，失败/取消帧走 `record_unobserved` 推进基线。旧的 `record_cpu_interval`（日志+采样门控路径）已删除。
   - 门控：新增 `cpu-probe` feature，`profile` 隐含它（`profile = ["dep:pprof", "cpu-probe"]`）；recorder 与接线均 `#[cfg(feature = "cpu-probe")]`，普通构建无此类型、无时钟开销；`CLOCK_THREAD_CPUTIME_ID` 读取在模块内再 `#[cfg(target_os = "macos")]`，非 macOS 返回 `None` 记 `ClockError`。
   - 验证：默认 `cargo check -p tiny-glade --locked --offline` 通过（profile 代码被排除）；`cargo test -p tiny-glade --lib --features cpu-probe` 下 7 个 probe 单元测试全部通过（273 filtered）——覆盖有界截断、零容量拒绝、时钟回退记错误样本、未观察帧推进基线、跨 App 重置、receipt 关联+JSON 形状、混合失败/取消序列的 receipt 映射。**注意：`--features profile`（拉 pprof）在当前离线 Windows 环境无法编译（`aligned-vec` 不在离线缓存），故用等价的 `cpu-probe` feature 验证纯逻辑；`CLOCK_THREAD_CPUTIME_ID` 实际读取路径仍需在 macOS 上实跑验证。**
2. **冻结并测当前源码，核实收益与回退。（未开始，需 macOS 实跑）** 两个agent只读审查、中央修改测试；相同输入交替顺序重复V30/V31/新候选，单独确认QoS策略贡献。静态、大量绘制、局部编辑、连续编辑每轮预热后至少1000帧、三轮；同时报告owner CPU均值/p95、主线程和进程工作量。轻场景p95回退超过5%不作为默认候选；不混入GPU时间。
3. **按新栈继续减少分配与锁争用。** 先验证两种projection复用是否减少ManagedBuffer重建；再测每pass真实唯一ledger长度/容量，避免按draw总量过度预留；评估提交scratch复用。用新栈决定优先级，不重做已经仅占4/1941样本的补丁排序优化，不缓存未知reflection或任意callback。
4. **收敛剩余Collect调度与可变owner绘制循环。** 统一跨pass和单pass的worker分页规则，保持任务/常量预算与稳定Commit顺序；迁移透明水/冰surface、Ui2d/ws_elements、object_grass_push可变循环。surface的Rc<RefCell> trace需改为worker本地记录后有序合并，不能unsafe Send或加全局锁。固定单draw及真实依赖次序不是旧串行fallback。
5. **关闭压力下的同步与大回收缺口。** `Device::retire`在HostOOM仍无限等待GPU，需要在真实native对象创建前准入退役ticket；覆盖CommandBuffer双payload、SetupPool detach/adopt与transient Unbound提前创建，禁止Drop临时扩容或改成泄漏。F032压力全量析构仍无配额，应单独设计有界进度，不把未证明的等待称为死锁。
6. **最后完成窗口和整链长跑。** App切换、resize/全屏、暂停恢复、截图取消、Pending关闭、晚相机输入，验证实际worker及跨pass任务数；稳定/效果/resize长跑和独立队列/Windows继续验收。短跑未发现新锁反转，不代表已排除全部死锁。
7. **冷端独立缺口。** atlas热重载的完整part-count/remap/容量合同仍需核实接线；不要将确定性启动排序当作完整热重载修复。MoltenVK异步queue-submit只列候选，当前仍为同步提交条件1；若评估必须包含driver/process CPU、真实完成ACK及失败退出，不能靠移线程把CPU成本隐藏。

最终验收必须证明代表性场景的CPU关键路径与渲染owner预算满足16ms，并保留干净逐帧分布；当前8.24/22.66ms只说明最新已冻结构建的均值，**不宣布60FPS或整渲染框架优化完成**。


### 最终提交前回归（213）

- rhapsody全库：433通过、2忽略；图执行CPU：23通过。
- 实际GPU：预算驱逐2、图cache驱逐3、managed projection5、原生子分配4，全部通过；投影用例包含两形状复用、外部强弱快照不变、容量恢复和报告零字节时真实owner释放。
- 应用定向回归：Raster14、上传/资源host29通过，严格要求Vulkan初始化成功。
- `cargo check --workspace --all-targets -j2 --locked --offline` 通过。
- 本轮未重新跑四类场景三轮、30分钟窗口长跑或Windows；未为当前最后的源码冻结新性能二进制。
