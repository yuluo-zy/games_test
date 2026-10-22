# 实施进度

最后更新 2026-10-21。每项的验证口径与完成定义见 IMPLEMENTATION.md。**耗时收益全部待真机**:本机只有 X11 显示器和 llvmpipe 软件渲染,软件渲染里 GPU 工作发生在提交调用内,不能外推。

## A 层状态

| 任务 | 状态 | 提交 | 说明 |
|---|---|---|---|
| P0 计时与计数接线 | 完成 | f1a7d841c | `TG_FRAME_TIMING_EVERY`、`frame_counters`、`TG_GPU_TIMING`(逐 pass GPU 耗时,带 pass 名) |
| P0 0.6 真机采集 | **待用户** | — | 需要 Windows 真机 |
| A1 帧在飞按显示帧计数 | 完成 | 86216d4f8,审阅修复见下 | `begin_display_frame/end_display_frame`,新增 `frame_throttle_ms` |
| A2.1 上传异步 | 完成 | 4531d4de9,审阅修复见下 | 排队 + flush,`TG_SYNC_UPLOAD=1` 回退 |
| A2.2 W11 仅变化时上传 | 完成 | c6de66272 | |
| A3 prewarm 待决集 | 完成 | e0a2d2fb5 | |
| A4 VAO 快照一次 | 完成 | 9cdf1fc37 | 真实运行 `vao_full_refresh calls` 9→2 |
| A5 ECS 多线程 | **改为可选特性** | f0c9bf469,审阅后改 | 见下"偏差" |
| A6.1 PGO 按图分槽 | 完成 | f96d67f04 | 真实运行主图录制 8 块并行 |
| A6.2 主循环专用线程 | 完成,**默认关闭** | 4a9373d0b,审阅修复见下 | `TG_GAME_THREAD=1` |
| A7.1 预算查询节流 | 完成 | b79f1bf0d | |
| A7.2 PGO 计时门控 | 完成(串行策略) | f96d67f04 | 未做按核数调整 `max_chunks` |

## 真实运行观测(X11 + llvmpipe,debug,480×270,tiny-glade-host)

| 项 | 改前 | 改后 |
|---|---|---|
| VAO 全量刷新 / 帧 | 约 9 | 2 |
| prewarm 扫描 ticket / 帧 | 全部,每帧两次 | 0 |
| 同步上传等待 / 帧 | 3(约 1.4 ms) | 0 |
| 主图录制 | 基本串行 | 1 批 8 块并行 |
| 窗口截图(排队 vs `TG_SYNC_UPLOAD=1`) | — | 平均绝对差 0.6/255,无像素差超 16 |

## 与实现文档的偏差(需要知道)

1. **A5 不再默认开启。** 用 `TG_SCHEDULE_AMBIGUITY=1` 在真实运行里审计:`Update` 调度有 **525 对**冲突且无声明顺序的系统,`PreStartup` 23 对,`Startup` 3 对。单线程执行器按确定顺序运行它们,多线程会变成不确定顺序。因此 `multi_threaded` 改为 `tiny-glade` 的可选编译特性 `ecs-multithreaded`,默认仍是单线程执行器。线程池按核数初始化(核数−1,夹在 2 到 8,`TG_ECS_THREADS` 可覆盖)仍保留,rhapsody 并行录制使用它。审计工具 `TG_SCHEDULE_AMBIGUITY=1` 与 `TG_ECS_EXECUTOR=single` 覆盖全部调度(含 gui 内部调度)。启用特性前要先把这些顺序固定下来。
2. **A6.2 默认关闭。** 窗口调用移到非主线程只在 X11 验证过;macOS 被硬禁用;Windows 未验证。实测**关闭不会更快**(线程模式 5.6 s、旧模式 5.9 s,均退出码 0):关闭请求仍要等游戏线程下一次处理事件。收益是 OS 事件泵不再被一帧渲染堵住,未量化。
3. **A6.1 的 `max_chunks` 按核数没做:** `device_config` 测试要求默认配置恒等于 `ADAPTIVE`。
4. **A2.1 的 staging 配置** 从 1 槽 4 MiB 改为 3 槽 16 MiB;staging 满时的等待由自旋改为 100 µs 休眠并计数(`upload_capacity_waits`)。

## 审阅与修复

三位独立审阅者各读一层提交。已确认并修复:

| 发现 | 严重度 | 处理 |
|---|---|---|
| A2.1:队列满触发中途 flush 后,重试与同调用后续块仍用旧 epoch;同帧下一次写同一缓冲会把这些未提交任务取消为"被取代",而调用方已清脏标记,**写入永久丢失** | 高 | 每次入队读取最新 epoch。回归测试 `queue_pressure_flush_in_the_middle_of_a_frame_never_cancels_earlier_writes`,已验证在旧行为下失败、修复后通过 |
| A1:一个显示帧提交超过 2 张图(重试、部分失败留在 pending)会耗尽命令槽,`execute` 返回 NOT_READY | 中 | 显示帧内的 `begin_frame` 也按命令槽数约束在飞图数。新增测试 |
| A1:`FrameLoop::begin_frame` 在 `graph.begin_frame` 失败时遗留显示帧,之后的图跳过节流 | 中 | 失败时关闭显示帧。新增测试 |
| A6.2:唤醒主线程依赖 `request_redraw`,窗口隐藏/最小化时可能收不到 | 中 | 改用 `EventLoopProxy` 用户事件;并且循环结束后不再调用任何窗口函数(曾因此在退出时 abort,已修) |
| A6.2:在回调内 `join` 游戏线程,Windows 上其收尾的窗口调用需主线程泵消息,可能死锁 | 中 | 汇合移到事件循环返回之后 |
| A6.2:`CursorMoved`/`MouseMotion` 在卡顿帧后全量回放,相机输入被多算 | 低 | 相邻鼠标移动事件合并 |
| A5:`TG_ECS_EXECUTOR=single` 只覆盖三个调度 | 中 | 覆盖 `Schedules` 中全部调度及 gui 内部调度 |
| P0:`TG_GPU_TIMING` 只对第一个桥配置,加载 App 与游戏 App 的会话不同 | 低 | 按桥身份重配置 |

**已知未修,供后续:**

- A2.2 的 W11 缓存以入队时刻为准,未随 flush 失败清除。F1 修复后已无已知触发路径。
- `prewarm` 对并发调用者可能提前返回(当前全部在同一线程,线程模式下也是)。
- 帧计数器是进程全局,加载帧与后台线程的计数会并入第一个游戏帧的日志行(低严重度)。
- `ScaleFactorChanged` 的 `InnerSizeWriter` 在游戏线程上已失效,当前代码不使用它,将来若通过它改尺寸会静默无效。
- `hang_watchdog::tick` 在两种模式下都没有被调用(原有问题)。
- 若 `plan_cache` 关闭或计划被淘汰,计划地址每帧变化,PGO 槽永远为空,主图不会并行录制;"命中时指针稳定"没有被断言测试覆盖。

## B 层进展(2026-10-22)

端到端实测(真游戏 `tiny-glade-host`,X11 + llvmpipe,debug,480×270,同机顺序各跑一次,`TG_FRAME_TIMING=1`):

| 阶段(gameplay 帧,ms) | B 层之前(A 层末) | B 层之后 |
|---|---|---|
| record_submit | 934–970 | 368–407 |
| declare | 143–173 | 126–129 |
| compile | ~15 | ~13–15 |

画面平均每通道差 2.6/255(两次停在不同动画帧),无破损。llvmpipe 的 `collect_ms`(软件光栅)约 2 s 占主导,**不代表真 GPU**;这组数字只说明 CPU 侧录制/提交确实变少,不能推出真机帧率。

| 项 | 状态 | 说明与实测 |
|---|---|---|
| B0 基准 | 完成(bind 部分) | `bind.rs` 内 ignored 测试 `bind_scaling_baseline`(release,合成 U=3040 / S=2679):改动前 bind 中位数 **75.3 ms** |
| B1(a) shadow 按资源索引 | 完成 | bind 降到 **1.24 ms**;`synthetic_binding_output_is_stable` 用单/双队列三组合成图的输出指纹证明与旧实现逐字节一致 |
| B1(b) bind 模板缓存 | 完成(受限版) | 仅"无所有权转移、无跨队列等待"时存模板;键 = 导入初始状态的帧无关部分(范围/布局/所有者/访问域/各队列有无点的掩码)+ 别名边;模板用 `Arc<CompiledPlan>` 钉住地址;命中只重定基各补丁 `expected_revision`。命中 bind **1.33 → 0.19 ms**(含 200 资源 add_initial)。`RHAPSODY_BIND_SHADOW=1` 时命中后再跑全量 bind 并断言一致;真实会话回归(p0_stability 1996 次命中等)全程通过。存在所有权转移的多队列路径永远走全量 bind |
| B2 提交不重放 | **部分** | 只做 `prepare_submission_states` 补丁分桶(O(M×P)→O(P),线程本地缓冲,`framework_execute` 仍 0 次分配)。**未做**:规划器直出最终 cell、Create 资源不持表、`SubmissionCapture` 策略、删第二次 `validate_submission_commands`——会把状态机"双实现"变"单实现+校验器",错了会产生错误屏障;提交阶段没有规模化基准,等真机数据 |
| B3 索引化 | 完成 | bind 末尾稳定计数排序建 (批次,阶段) 索引;recorder/prelude/patch/wait 取切片;任何无法定槽的项回退线性过滤。`phase_index_matches_linear_filter` 证明内容与顺序一致;真实会话 `valid=true` |
| B4 去字符串 | **未做** | 要改 `ResourceDeclaration.name`、`PassDeclaration.name`、`pass_order`、`PassPgoNameId` 等公开类型并引入 `ustr`;收益是每帧约 M(~200) 次 String 分配/比较,量级远小于已解决项 |
| B5 计划缓存查找 | **评估后不改** | 稳态命中已是"LRU 新→旧 + 结构不等 O(1) 早退 + 一次完整比较";声明指纹本身也是 O(M+N+U),不会更省。miss 路径的 128 次原因分类只是诊断,相对编译开销可忽略。是否每帧 miss 需真机日志(`plan_cache_hit`) |
| B6 动态常量页 | 完成(简化版) | 页上无未完成 GPU 使用时,chunk 宿主写直接拷贝并记延迟 flush,不走 3 把锁的范围表事务;否则回落原路径。lavapipe 下 900 次写 debug 5.5→3.0 ms。没有按设计文档做"整页租约"(会改 `RangeStateTable` prepare/publish 配对语义) |
| B7(1) validate_graph 重复 | 完成 | 会话校验后置 `declaration_prevalidated`,`prepare_declaration` 跳过一次 |
| B7(3) finish_frame BTreeSet | 完成 | 改排序 Vec + 二分 |
| B7(7) 描述符 | 部分 | 只把计数器 `SeqCst`→`Relaxed`;批量 `vkUpdateDescriptorSets` 会改变逐 binding 写入/回滚语义且测试按次数断言,未做 |
| B7(8) 附件视图查找 | 完成 | 游标命中(从上次命中位置起找),稳态 O(1);保持原淘汰顺序 |
| B7(2)(4)(6)(9)(10)(11) | **未做** | (2) 仅在有退役 named 资源时发生;(4)(6) 每帧 ≤数十微秒;(9) 管线表需改泛型键与锁结构;(10)(11) 需 BTreeMap→哈希的大面积替换。均无实测证明是瓶颈 |
| B7(5) | A 层已做 | `refresh_memory_budget_if_due` |

**测试说明(纠正一次误读)**:`cargo test` 不带 `--no-fail-fast` 时,第一个失败的测试文件之后的文件不会运行,单次输出只会显示一行 FAILED。完整失败集用 `--no-fail-fast` 取得:rhapsody 共 11 项,B 层之前(`aee24c5d2`)与现在逐项相同:`multi_queue_graph` 5、`native_ranges` 2(lavapipe 无独立计算队列)、`setup_context` 2、`command_retention::descriptor_owner_row_oom_precedes_native_update_and_preserves_retry`、`host_buffer_ranges::metadata_admission_failure_stays_on_the_command_and_publishes_no_completion_point`。

## 全框架拷贝/零拷贝/常规优化审计(2026-10-22)

三路只读审计(rg 层、raw/device/transfer 层、宿主层)+ 逐项核实后实施。耗时估算来自代码结构推导,真机仍需 `TG_FRAME_TIMING=1` 复测。

| 类别 | 项 | 结果 |
|---|---|---|
| declare | 全局纹理表每 raster pass 重复导入 + O(R) 线性去重 | 已改:`FrameImports` 增 (graph,identity) 哈希索引;纹理表读声明按 graph 缓存 |
| declare | 每 draw 重复导入帧常量缓冲;每 draw 克隆 `VaoSnapshot`;u64 键 SipHash 表;`DrawLists` 每帧新建 | 已改:每 graph 一次;`Arc<VaoSnapshot>`;`IdMap`;跨帧复用 |
| declare | 分发表线性比字符串、`select_lists` 堆分配、每 uniform 先 `Vec::new`、SSBO 命中仍建 `RenderGraph`、重名标签 O(k²)、compute accesses 克隆 | 已改(country-core 单测 draw_lists 29 / draw_builder 54 / draw_stream 18 通过) |
| 提交 | `validate_submission_commands` 无人读取的 O(U²) 去重(每次提交两次);`add_use_cell`/owners 线性扫描;`upload_reads` 对每个 cell 扫全部命令 | 已改:删除冗余;指针集合;并入分桶 |
| 零拷贝 | 缓冲/纹理上传先打包到临时 Vec 再写映射内存;`Payload` 先清零再覆盖 | 已改:`Buffer::write_with_unsynchronized` 直写映射 staging;`Payload::from_slice`;删除 `Scratch` |
| 零拷贝 | 状态回放拷贝 300–480 字节 cell;模板命中复制 transitions/batches/order | 已改:原地修改(−33%);`Arc<SharedBound>` 共享 |
| 杂项 | 动态常量 append 路径每次 `mark_live`+`trim_idle`;`barriers` BTreeMap;`submissions` `remove(0)`;PGO/`import_families`/`history_validity` 用 SipHash/BTree;`export_temporal` 深克隆声明;`resource_access::steps` 的 `TempList` | 已改 |
| 分配器 | 发布二进制用系统分配器 | 已改:`mimalloc`(feature,默认开,仅 bin;库与测试不变) |

**评估后不做(附证据)**

- **F3 补丁链索引**:图路径的补丁在批次末尾才挂到命令上,录制回调期间 `state_patches` 只有 helper 的寥寥几条,审计假设的 P≈1000 不成立。
- **F5 `coalesce` 预判有序**:实现后在合成基准上无收益(p95 略差),已撤销。
- **F12 `poll` 缓存**:新增 `Timelines::poll_stats`,真游戏每帧 `poll` ≈ 37 次,真正的驱动查询 ≈ 9 次(已有缓存),估计 < 20 µs/帧。
- **H9/H10**(`texture_slots`、`frame_setup::begin` 重建):涉及的表只有几十个元素,合计远小于 20 µs,且失效条件有风险。
- **H7 常量暂存**:`DynamicConstantsBinding.data` 是 country-core `PassHost` 的契约,改成直写映射要动整个 trait 与 DrawBuilder 单测;只做了 uniform 暂存复用。
- **R2**:`add_recorded` 只是一次 Arc 克隆,下游 `retain` 重复已被指针集挡掉。

**真游戏端到端(debug + llvmpipe,同机顺序运行)**

| 阶段 | A 层末 | 现在 |
|---|---|---|
| declare | 143–173 ms | 44–48 ms |
| record_submit | 934–970 ms | 225–252 ms |
| compile | ~15 ms | ~12.5–14 ms |

整帧仍被 llvmpipe 软件光栅(约 1.8 s)占主导,2.1–2.4 s 的整帧波动主要来自场景帧不同,不能据此推 FPS。**这是 CPU 侧阶段耗时下降,不是真机帧率。**

**测试说明**:rhapsody 全量(含 `RHAPSODY_BIND_SHADOW=1`)失败集与最初提交逐项相同(11 项);tiny-glade 集成测试失败集与改动前逐项相同(32 项,均为既有)。

## 独立复审与修复(2026-10-22)

三路只读复审(同步规划/状态机、rg/raw/transfer、宿主)覆盖 `aee24c5d2..HEAD`。结论:无高严重度缺陷;已修复的问题:

| 来源 | 问题 | 修复 |
|---|---|---|
| rg/raw | `declaration_prevalidated` 置位后到 `compile_graph` 之间有两个 `?` 提前返回,标志会泄漏并让下一个 graph 跳过校验 | 置位移到紧挨 `compile_graph` 之前,窗口内无提前返回 |
| rg/raw | 公开 `trim_idle` 经 `refresh` 把当前常量页标成 Cache,之后 append 快路径不再 `mark_live` | `trim_idle` 之后重新 `mark_live` 当前页 |
| rg/raw | `pack_regions` 注释仍写"padding 为零" | 改为"padding 未写入,无拷贝读取它" |
| 同步规划 | 模板 rebase 只检查 `revision+delta`,漏了该 patch 自身的 increment 溢出边界 | 同时要求 `+1` 不到保留最大值;新增测试 |
| 同步规划 | `resource_shadow` 对未校验的 resource 编号无界扩容(`u32::MAX` 会尝试分配数十 GB) | `add_initial`/`add_alias` 先做上界检查;新增测试 |
| 同步规划 | 模板内存不可见 | 新增 `template_cpu_bytes()`(上界为 4 个模板) |
| 宿主 | 纹理表缓存假设同 graph 内表不变 | 命中时 debug 断言表长度一致 |

接受不改:模板钉住旧 plan 的 Arc(最多 4 个,LRU 挤出后释放);单槽 `table_reads`/`frame_constants` 缓存在两个 graph 交错声明时只退化为重新计算;`default = ["mimalloc"]` 让库构建也编译 libmimalloc-sys(需要 C 编译器),macOS/Windows/iOS 需在 CI 各验证一次。

复核后的测试:rhapsody lib 424 通过;全量失败集与最初提交逐项相同(11 项,均为既有);`framework_execute` 仍为 0 次分配;tiny-glade 集成测试失败集与改动前逐项相同。

## 本机构建环境(Linux,无 sudo)

- 依赖镜像:`cargo ... --config /tmp/ustc.toml --locked`(去掉 `--offline`,上游新增了 `lcms2`)。
- tiny-glade 依赖 GTK 系统库(经 rfd)。占位 `.pc` 用 `PKG_CONFIG_PATH=/tmp/stub-pc`,链接用 `LIBRARY_PATH=/tmp/stub-lib`(指向系统 `.so.0` 的符号链接)。
- 严格 GPU 测试需要校验层:`source ~/.local/vulkan-tools/env.sh`,并设 `RHAPSODY_REQUIRE_VULKAN=1 VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation`。**没有校验层时,依赖 `validation: true` 的宿主测试会静默跳过并显示通过。**
- 真实运行游戏窗口:二进制用 `tiny-glade-host`(`tiny-glade` 需要 Steam),`TG_INSTALL_ROOT="…/Tiny Glade"`,需要 `libxkbcommon-x11` 与 `libxcb-xkb`(用 `apt-get download` 解压到 `~/.local/xkb`,设 `LD_LIBRARY_PATH`)。该宿主缺 `SteamdeckScreenshotMsgScreen` 资源会在 `hide_ui` panic,需要临时 `init_resource`(未提交)。

## HEAD(f98b40229)上已存在的断裂,与本次改动无关

- `country-core` lib 无法编译:`systems/ui/ui_update.rs:9` 从 `resources::gui` 导入的 `EscapeMenuJustOpened` 实际在 `resources::events`。验证时需要临时修补,提交前还原。
- `country-core` lib 测试目标无法编译:`ui_update_chain_tests.rs` 依赖 `use super::*` 带出的导入;`hover_intersection.rs:349` 闭包需 `|i: usize|`。
- `tiny-glade` lib:142 通过 / 6 失败(缺 `TerrainHeightsData` 夹具),与改动前一致。
- 宿主集成测试已有失败:`core_raster_draw` 4 项(声明契约漂移)、`core_raster_pipeline_creates_from_key_handle_and_opts_bitfield`(着色器夹具被编译器拒绝)、`original_gameplay_assets_reach_the_same_app_atlases_and_nani_gpu_owners`、`original_shared_captures_keep_msaa_pixels_and_actual_slot_owners`。
- `rhapsody` 的 `multi_queue_graph` 5 项在严格模式下失败:lavapipe 只有一个队列。
