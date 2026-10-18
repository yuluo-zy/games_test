# country-core / rhapsody 调查：完整游戏恢复的剩余阻塞

日期：2026-10-18。范围：当前 `games_test/tiny-glade/crates/{country-core,rhapsody,tiny-glade}` 的源码与调用者静态检查。本报告只记录可证实的行为与接线缺口，不修改生产代码，不重新验收原作等价性，不把编译、测试、工具连接或原有报告的完成标签当作整游戏恢复完成。本轮其他 worker 正在实现调用桥；其新增实现及验证结果以本目录主报告为准。

## 已有真实实现，不能误列为“没有后端”

| 源码入口 | 已实现的调用关系 | 当前边界 |
|---|---|---|
| [render_host.rs:23](../../crates/tiny-glade/src/render_host.rs#L23) `BufferHost` | core 缓冲描述与代数句柄 → rhapsody 托管缓冲；支持映射写入及设备本地缓冲的 transfer 上传 | 需要真实 Device 与调用宿主持有其槽位生命周期。 |
| [render_host.rs:259](../../crates/tiny-glade/src/render_host.rs#L259) `TextureHost` | core 纹理描述 → rhapsody ManagedTexture / 默认视图缓存 | 宿主负责创建与持有资源，不能用任意数字 ID 代替。 |
| [render_host.rs:892](../../crates/tiny-glade/src/render_host.rs#L892) `impl ShaderBackend for ShaderHost` | 读取编译包、产生两个 depth-only 变体、反射与 bindless 参数、创建真实 compute / raster pipeline | 这是正式宿主后端。core 的 `GraphicsDevice` 便捷包装中返回 Unsupported / panic 的路径不是唯一后端，不能据此断言实际所有 shader 构造都会失败。 |
| [render_host.rs:1156](../../crates/tiny-glade/src/render_host.rs#L1156) `impl ComputePassDevice for DeviceHost` | core ManagedHandle →真实宿主缓冲 / 视图；ComputeShaderProgram → ShaderHost 中的真实 registry | 全 core 范围搜索会漏掉此宿主实现，调查必须覆盖 tiny-glade crate。 |
| [render_host.rs:1203](../../crates/tiny-glade/src/render_host.rs#L1203) `GraphPasses` | core helper pass、动态常量、上传、资源声明/temporal、RecordedComputePass → 真实 rhapsody 图与命令回调 | 需要相同 frame/session 和 HostTables。独立图/独立 session 中的资源号不能相互通用。 |
| [raster_host.rs:1](../../crates/tiny-glade/src/raster_host.rs#L1) | core RasterPass/DrawBuilder → 图访问声明、附件缓存、真实管线与 bindless 参数、`vkCmdDraw*` | 这是已有正式光栅桥。仍需世界资源/VAO/着色器库及高层 frame consumer 提供真实输入。 |
| [execution_host.rs:24](../../crates/country-core/src/render/execution_host.rs#L24) `RhapsodyExecutionHost` | 使用真实 VulkanGraphSession 的 begin / execute / completion / wait 链 | 具有真实调用，不等于已有完整游戏入口。 |

本轮正在新增的 `tiny-glade/src/compute_frame.rs` 将 core 的 `PassFrame::record` 接到 `GraphPasses::add_compute`，并从同一个 `AssetShaderLibrary` 解析 program。它补的是已录制计算 pass 的生产落点；不是完整 `FramePasses` 实现，也不替代 WorldAccess、ECS 域资源或窗口主循环。错误需要先经 `finish()` 返回，才能继续 build/submit。

## 可证实的完整游戏阻塞

### G1：宿主入口与主循环未实现（P0）

- [tiny-glade/src/main.rs:91](../../crates/tiny-glade/src/main.rs#L91)：`main()` 直接 `todo!()`。
- [tiny-glade/src/run_game.rs:111](../../crates/tiny-glade/src/run_game.rs#L111)、[run_game.rs:119](../../crates/tiny-glade/src/run_game.rs#L119)：`run_game` / `run_game_thread` 直接 `todo!()`。
- [game_loop.rs:92](../../crates/tiny-glade/src/game_loop.rs#L92)、[game_loop.rs:100](../../crates/tiny-glade/src/game_loop.rs#L100)、[game_loop.rs:108](../../crates/tiny-glade/src/game_loop.rs#L108)：GameLoop 的 new / run / main_loop 未实现。
- [country-core/startup/run_game.rs:94](../../crates/country-core/src/startup/run_game.rs#L94) 已注册多域真实系统；当前 crates 中没有宿主调用 `register_core_startup` / `add_game_startup_systems` 的生产位置。

消费者影响：没有完整入口能装配权威 World、更新业务调度、驱动渲染并处理窗口事件。已经可独立调用的图/绘制桥不能消除这项缺口。

### G2：世界到整帧编排的适配仍缺（P0）

- [render_frame.rs:91](../../crates/country-core/src/systems/render_loop/render_frame.rs#L91) 的 `RenderHost` 与 [render_world.rs:225](../../crates/country-core/src/systems/render_loop/render_world.rs#L225) 的 `FramePasses` 在当前 crates 源码中只见各自测试 mock 实现。
- [tiny-glade/render_loop.rs:25](../../crates/tiny-glade/src/render_loop.rs#L25) 只是归属 re-export，未提供上述 trait 的生产实现。
- [draw_passes/draw_lists.rs:346](../../crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs#L346) 已有真实 ECS 查询、shader 名分发表与 DrawLists 生产，但当前无生产调用。
- [compute_pass_builder.rs:154](../../crates/country-core/src/render/pass_builder/compute_pass_builder.rs#L154) 的 `WorldAccess` 需要 `ResourceLookup` 与真实 `ComputePassDevice`。当前没有面向游戏 World 的生产实现；DeviceHost 的存在不能自动提供 World 内资源、shader library 和 uniform 分配器。

消费者影响：W1–W54 的编排没有真正把每个游戏 pass 的资源、业务状态与 DrawLists 提供给宿主。新调用桥可执行实际声明/已录制 pass，仍不能声称整游戏 frame 已贯通。E/G 类未知资源指针应保留明确边界，不能填假资源、固定句柄或空输入后宣布完成。

### G3：首次会话不创建 ActiveSession（P1）

[startup/startup_session.rs:96](../../crates/country-core/src/startup/startup_session.rs#L96) 接收 `Option<ResMut<ActiveSession>>`；[185 行](../../crates/country-core/src/startup/startup_session.rs#L185) 只修改已有资源，资源缺席时不插入，却在 [193 行](../../crates/country-core/src/startup/startup_session.rs#L193) 继续发 NewSessionStartedCmd 与 SessionLoadedCmd。注册端注释明确称 ActiveSession 应由该加载器生产，但实现没有完成。

消费者影响：[save_game.rs:275](../../crates/country-core/src/systems/save/save_game.rs#L275) 强制要求 `Res<ActiveSession>`。依照当前注册链启动的新鲜 World，后续系统无法获得已成功加载的会话权威资源。

### G4：保存载荷与 History 同步无生产者（P1）

[save_game.rs:251](../../crates/country-core/src/systems/save/save_game.rs#L251) 的 `SaveGameSyncData.sections` 默认 None，保存系统在 [339 行](../../crates/country-core/src/systems/save/save_game.rs#L339) 只有 Some 才发送完整 OpenFile。当前 crates 中只发现此资源的定义、初始化和读取，没有采集并写入完整 10 节值的系统。`SnapshotCaptureState` 也只有默认游标和消费路径，没有从 History 同步游标/快照链的生产者。

消费者影响：自动保存缺少完整 history.json 数据源，快照索引也不能保证与当前编辑账本一致。底层 JSON/shadow/thread codec 已实现不代表世界保存链闭合。

### G5：自动快照序列化不完整，自己的加载器会拒绝（P1）

[save_game.rs:362](../../crates/country-core/src/systems/save/save_game.rs#L362) 的采集器虽收集墙、屋顶、地形、部分楼梯，但 Trees/Decorators/Colors/Raster 传空；`SnapshotDomainData::collect` 将 section_values 置空。[snapshot.rs:221](../../crates/country-core/src/systems/save/snapshot.rs#L221) 只自动序列化非空 Trees 和 section_values，其余已采集域不进入 JSON。

消费者影响：该自动路径的 `SnapshotSaveData::to_json()` 得到 `{}`；[save_thread.rs:362](../../crates/country-core/src/systems/save/save_thread.rs#L362) 原样写盘，而 [snapshot.rs:434](../../crates/country-core/src/systems/save/snapshot.rs#L434) 强制要求 Version。自身生成的自动快照不能被自身加载器接受。

### G6：加载尚未应用世界就发送完成（P1）

[load_game.rs:683](../../crates/country-core/src/systems/save/load_game.rs#L683) 与 [684 行](../../crates/country-core/src/systems/save/load_game.rs#L684) 的 Apply / WriteManager 分支为空，之后 [691 行](../../crates/country-core/src/systems/save/load_game.rs#L691) 仍发送 LoadingGameComplete。当前 `init_game_state_from_snapshot` 没有生产调用。

即使手工调用快照写回，[snapshot.rs:747](../../crates/country-core/src/systems/save/snapshot.rs#L747) 对 Roofs/WallColors/GateColors/Trees 仍仅记日志；[parse_wall_section:803](../../crates/country-core/src/systems/save/snapshot.rs#L803) 只保留墙 ID 和 height，再经 [inner_wall_state.rs:163](../../crates/country-core/src/resources/walls/inner_wall_state.rs#L163) 构造空 points/points_u。

消费者影响：成功事件不能证明世界已经还原；屋顶、树、配色与墙几何不能完整恢复。应恢复 owned/typed 曲线数据通道，不能尝试把 JSON 点列塞进原生 Rust Vec 指针记录。

### G7：默认构建不提供碰撞查询（P1，宿主特性装配）

[country-core/Cargo.toml:11](../../crates/country-core/Cargo.toml#L11) 将 rapier 置于可选 native-physics 特性，默认不开启。[collision/world.rs:1342](../../crates/country-core/src/systems/collision/world.rs#L1342) 与 [1420 行](../../crates/country-core/src/systems/collision/world.rs#L1420) 的非特性实现分别固定 miss / false。

消费者影响：使用默认特性运行的宿主不能获得真实拾取与碰撞；这不是“物理算法完全没实现”，而是必须在实际宿主明确选择并编译对应特性，随后接入真实 collider 生产者。

## 已有宿主中的具体能力边界

这些是有条件的能力限制，不应扩大成“没有正式后端”：

- [render_host.rs:994](../../crates/tiny-glade/src/render_host.rs#L994)：raster shader 包的 original push payload 非零时明确返回 Unsupported。需要按真实 reachable shader 确认哪些游戏 pass 触发此边界。
- [raster_host.rs:881](../../crates/tiny-glade/src/raster_host.rs#L881)：未支持的 DrawKind 进入正式错误通道。应由真实 DrawLists 的消费者覆盖判断后续优先级，不能只按 enum 数量估进度。
- core GraphicsDevice 包装中保留的未连接 descriptor / managed handle 路径仍不能直接当正式宿主使用；应选择 ShaderHost / DeviceHost / GraphPasses 等已有实现，而不是据此重复开发第二套后端。

## 最小可行接线顺序

1. **形成唯一 World 与宿主生命周期**：创建真实会话/资源库；明确 startup、update、render 和事件维护调度；让高层入口实际调用 core 注册系统。首先消除 G1/G3。
2. **接真实世界资源消费者**：WorldAccess / ResourceLookup / uniforms、域 SSBO、VAO、shader/texture library → build_draw_lists 与一个有真实输入的游戏 pass。复用现有 DeviceHost / ShaderHost，不填充固定资源来隐藏未知合同。
3. **形成可调用游戏帧闭包**：本轮计算/光栅桥 → 同一图、同一 session 的资源声明与回调 → build/execute/retire；再实现 FramePasses 与 RenderHost 以覆盖真实可达的整帧编排、swapchain 与 present。失败通道须阻止后续成功发布。
4. **持久世界往返**：完整 SaveGameSyncData / History 同步 → 自动快照全域序列化 → typed owned 世界写回 → replay Apply / WriteManager；只有状态恢复成功后发送 LoadingGameComplete。
5. **宿主启用实际交互能力**：选择 native-physics，接实际 collider 生产与查询；据真实消费者解决 push payload、DrawKind 等具体宿主能力边界。

## 验证范围

本报告执行的是源码阅读与有限 `rg` 调用点检索，没有执行编译、GPU、窗口、场景或发行原作行为差分。`render_host` / `raster_host` 中的真实库调用是代码证据；其既有测试结果和本轮新增桥验证由整合 owner 单独报告。当前不能由本报告推导画面、存档兼容或整游戏可运行已经完成。
