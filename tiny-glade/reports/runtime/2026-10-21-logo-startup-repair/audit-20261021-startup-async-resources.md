# Logo→gameplay：启动资源异步只读审计

审计当前生产源码与既有 `tmp/logo-startup-diagnostic-process.log`；未改生产、未跑 Cargo/GUI。耗时是该 Intel UHD630/480×270/显式兼容 WSI 日志的阶段墙钟，不代表 CPU 独占时间。

## 已能确认的事实

- 日志8→147：gameplay worker 第一次 SubApp.update 在15:39:52.735→15:40:09.251，16.516s。该次更新包含全部 gameplay PreStartup/Startup/首 OnEnter/首Update；**这些加载已经在 worker，不能简单再“搬去异步”**。
- 日志151→178：worker完整原world warmup15:40:09.378→27.935，18.557s。期间loading持续进帧；其耗时包含world graph构造、pipeline请求/等待、graph compile/record/submit和真实GPU完成，日志未分项，不能全归因I/O。
- 日志77→138：loading frame240→270的30帧耗6.409s；前后30帧约0.7s。不能据30帧汇总证明是单帧停6秒，也不能从worker UI配置日志证明主线程在读该文件。
- owner handoff15:40:28.346→28.363，仅17ms。当前证据不支持handoff资源搬运是主要卡顿。
- handoff后28gameplay帧直到15:41:10.302，约42s；后30帧42.5s，说明还存在稳态世界帧耗时，不仅是首帧资源加载。

## 主线程仍执行的重活

1. `main.rs::launch`：paths/settings/allowlist文件读写、`load_language_library`；发生在创建窗口之前。`run_game_thread`主要注册owner/调度，也创建BVH worker；不能因名字称“thread”就当后台线程，实际由`run_game`直接同步调用。
2. `game_loop.rs::GameLoop::new:449`→`ProductionRenderer::new:683`：Vulkan instance/device/surface/swapchain创建必须保留平台约束；然后`ApplicationGpu::new:597`→`CoreRenderBridge::new`及全量load_shaders/UI/blit/painterly/vermeer。`render_host.rs::ShaderHost::read:1787`同步fs::read；create_*同步decode/reflection/ABI/layout准备，再投递编译worker。不是全部非阻塞。
3. 同一`GameLoop::new:523 activate_application(loading)`又为独立loading shader library执行`ApplicationGpu::new`。两App owner/句柄命名空间必须独立，不能合并私有库来避耗时。独立库可共享明确只读源字节缓存，但当前没有该层。
4. `gameplay_startup.rs::install_gameplay_device_runtime:250`调用`audio_host::install:25`，其中create_audio_resources的Studio/boot banks初始化同步在主线程；remaining bank samples另开线程，不能将整个音频都判为同步。当前log load_audio等待仅21.2µs，不支持其占主卡顿。
5. loading第一次`App.update`在OS线程执行`loading_graph::install_loading_startup`注册的`initialize_common_assets:99`：扫描bootup textures、AlignedFile::read、parse_compiled_image、create_texture、全部字体文件解析；还有palette/minimalSSBO/logo shader注册。此处可以预读/解析异步，但loading首次显示仍必须等它真正需要的资源齐备。
6. `production_renderer.rs::compile_loading_graph:1169`、`compile_full_world_graph:455`、`execute_presentation_graph_inner:1054`均在每次graph声明后调用`ShaderHost::prewarm`。`render_host.rs:1595`收集**此host全部**compute/raster/adapter tickets，`:1626 ticket.wait`；这与`:1536`注释“No frame callback waits”不符。已完成ticket只是快查，但任何新/未完ticket可卡住该帧，甚至包含当前graph不需要的pipeline。
7. `render_host.rs::with_compile_backpressure:2941`在compiler queue满时每2ms sleep重试，最长180s；因此甚至graph声明/启动shader注册阶段也可能同步阻断。默认compiler每host1worker、64pending（rhapsody/pipeline.rs:35），底层已有有界队列/优先级，不应无界增worker解决。

## Worker与GPU共享造成的瓶颈

- `GameplayPreparation::spawn:1285`真实移动整个SubApp至唯一worker。`gameplay_startup.rs:598` gameplay全部textures/fonts/LUT、prefabs、atlases、decorators、Nani等在该worker首次update。Bevy独占World的closure使部分阶段串行，16.5s尚无单资源计时，不能预判哪个loader最大。
- `render_bridge.rs::warm_up_gameplay_with_world_builder:1083`在worker构建真实graph→prewarm→compile→execute→`complete_gameplay_warmup`等真实GPU完成后，才发布SubApp。这个ready边界必须保留。
- **最新rhapsody纹理上传已无CPU等待**：raw/device_api.rs:142 create_texture→raw/setup.rs:464 init_texture_data，setup:346 upload_regions返回SyncPoint，资源state记录依赖；不能恢复旧上传库，也不能把它当逐纹理wait_idle。
- 但raw/setup.rs:277 SetupContext::record和:346 upload_regions均选同一Graphics queue。gameplay worker批量纹理/zero-fill/mesh上传与loading渲染争同设备/队列/驱动/内存带宽。异步CPU不代表Logo GPU可抢占已提交的大批工作；此处是frame240→270异常区间的合理候选，尚缺实际队列及帧分项计时，不可断言已证明原因。
- `render_host.rs::BufferHost::update_buffer_bytes:163`对mapped host-visible直接更新；device-local fallback`:210 upload_buffer_ranges_wait`显式等待自己的上传点。若首帧/持续Update仍触发该路径，主线程会等待，但目前未统计命中owner/字节量。
- pipeline workers是两App各自独立，但共享驱动设备；compile任务CPU/驱动锁与GPU上传可相互干扰。当前每host仅1worker，不能归因“几十编译线程抢CPU”。

## 正确异步优化切分点（沿现有架构）

优先补低开销分段计时：文件read/parse、texture create+bytes+setup submit count、shader请求queue-full耗时、prewarm unresolved数量和wait时长、worker update各Startup阶段、loading帧CPU compile/record/submit与receipt完成。日志采样/聚合，不每资源持续刷主线程日志。先区分6.4s窗口是CPU wait、driver submit还是GPU前序依赖。

1. **bootup与world分开准备**：loading最小必需源预读/字体解析先形成有界owned结果，其他全场景I/O/解码留worker。将`ShaderHost::read`的immutable compiled bytes读入后台缓存，主线程只消费完整校验结果；仍注册到各自真实shader/library owner，保留原错误/取消/文件身份。
2. **pending pipeline状态显式化**：使用既有AsyncPipelineCompiler/ticket.try_get与原优先级；将prewarm等待留启动worker，把loading每帧的全host等待缩成“当前graph必要ticket已就绪”的前提。遇pending继续渲染上一张合法loading帧或保持loading准备状态，不能无shader继续执行或报ready。BuiltCoreGraph当前持有Rc session，不能直接丢到thread；需由同线程保留/取消其lease并在下一帧验证，不得跳生命周期。
3. **上传预算与依赖**：复用rhapsody现有TransferManager/TextureUpload（已有multi-subresource plan、timeline与release程序）或在worker按大小/提交预算分批准入，给Logo图形提交留机会。不得把所有原格式降成RGBA、删mip/层、漏zero-fill、抹状态依赖；如果使用独立transfer queue必须支持实际family ownership/release/acquire，不能直接换queue字段。
4. **整world后台初始化所有权**：可把游戏侧shader文件read/decode/申请pipeline进一步移worker，完成后通过现有CoreRenderOwners握手给main本地新建/恢复Rc session。CoreRenderBridge含Rc<RefCell>，不可假Send跨线程；必须保同一BufferHost/TextureHost/ShaderHost/RasterShared/AssetShaderLibrary身份。取消仍先join/save cleanup再销毁GPU owner。
5. **避免首gameplay额外冷pipeline**：worker原warmup是32×32、无egui，且无swapchain presentation；normal main包括egui和MSAA resolve到真实swapchain。因此warmup完成不证明首次正常帧全部variant已编译。可在loading等待期间请求确切必要的首帧额外pipeline并真实完成；保持原32×32 warmup不冒充全normal帧，不能用假ready或删T21/resolve避编译。

handoff本身17ms，先保留现有真实CompletedGameplayWarmup owner adoption/六资源移动，不优先拆它。稳态42.5s/30帧需要独立CPU/GPU frame profile，不能承诺仅异步资源即可消除。
