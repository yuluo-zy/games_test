# 原版优先：启动、加载、渲染与 present 复核

日期：2026-10-18。状态：正在实施和验证，完整游戏尚未恢复。

本次直接阅读当前生产函数体，并重新查询 B 原版数据库及发行 RNE 的限定指令范围。没有按历史完成率、统计资料或测试数量推断业务完成。源码中的旧“完整恢复”标签也不构成通过证据。

用户最新架构约束：原版用于核实游戏业务和调用语义；`crates/rhapsody` 保留已经重构的架构，只基于现 API 增功能、修缺陷。具体边界见 [RHAPSODY-BOUNDARY.md](RHAPSODY-BOUNDARY.md)。

## 证据身份和方法

- 发行 RNE SHA-256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 只读分析数据库对应的 patched EXE SHA-256：`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- 原程序版本为 B。A/B/C 索引领取用于文件族互斥，不把索引默认 C 当作 B 的行为证据。
- MCP 的顶层成功不等于函数完整：逐项查看嵌套结果，并按 PDB 的完整函数长度补读 RNE。`compile_loading` 为 7519 字节、`compile_main` 为 10961 字节、`compile_present` 为 4867 字节，均长于 Ghidra 的首段函数体。
- PDB 的 ICF 同址名必须检查指令。`0x14067dd80` 虽被命名为 `HalftimberColorIdSsbo::handle`，实际只有 `mov rax, rcx; ret`；present 调用前取的是 DynamicConstantsResource，不能据此编造一个额外半木颜色资源依赖。

## 原版启动并不是一个 App

`run_game_thread` 先创建 gameplay App，标准启动再创建独立 loading App。`dev.skip_splash` 是直接进入 gameplay 的原旁路，不能为了让渲染报错消失而临时删除相机或固定加载状态。

| 原版事实 | 原证据 | 当前生产装配 |
|---|---|---|
| gameplay Vermeer `(enabled=false, hold=false)` | `0x14059dedf..0x14059deea` | gameplay World 持有禁用初态 |
| loading Vermeer `(true,true)` | `0x1405a4518/0x1405a451a → 0x1400cd450` | 独立 loading World 持有启用、保持初态 |
| 两套 shader/texture/mesh/SSBO/VAO/placement/dynamic owners | loading App 原资源构造和插入 | 每 App 独立 CoreRenderBridge/资产库，共用实际 GraphicsDevice |
| gameplay 后台反复 `SubApp.update` | `async load` 闭包 `0x1400d7440` | 真正的 SubApp 工作线程；没有固定帧数 ready |
| 收到 LoadingGameComplete 后先 offscreen warmup render，成功才发送 SubApp | `0x1400d76e3` 后的原完成分支 | 世界预渲染仍依赖未恢复的原 world graph consumer，不以空图代替 |
| Logo waiting 时接收、finish_wait，finished 时移动资源并切换 SubApp | 原 GameLoop 主循环 | 原两阶段切换条件已落实；未制造完成事件 |

`FrameRenderConfig` 启动的三个尺寸都来自实际窗口；两个缩放由原设置决定。`AdaptiveRenderState` 只恢复已证的启动变体和 enabled 标志，不把未初始化的非活动枚举载荷填成业务数据。RuntimePlatform 的原分支为 Steam Deck 0、GeForce Now 1、默认 2。

当前后台明确拒绝缺失的 gameplay 启动资源，不能把源码出现了线程、事件或接口称为标准游戏已经启动。

## 加载画面真实链

原 `compile_loading`：固定 SRGB 格式 `0x2b` 的 loading target → Vermeer → MEI → loading DrawLists → export `0x22`。这条路径原版没有 Egui draw，不把 Egui 测试矩形塞进加载画面。

- `SavedSessionList.sessions.first()` 的可选背景只尝试一次；图像/GPU 可选加载失败保持 absent。物理纹理 owner 保留，逐帧导入的图索引不会冒充 GPU 句柄。
- 读取真实 `bevy_time::Time<()>`，Vermeer timer 每帧只增加 `min(dt,1/30)`。未启用 Vermeer 是原断言失败，不插入假激活状态。
- 原版 Logo Startup 加载两份 JSON 网格，注册原 shader，使用 `T=(0,6,0)`、`S=(5,5,5)`、默认旋转，生成 DrawableMeshBundle、DesignatedRenderPass 18、DontCastShadow 和 KeepOnLevelUnload。绘制回调读取同一个 LogoAnimation owner。
- MEI 的 NodeContent 上下文连接真实 TextRenderer。LabelContent prepare 实际文字、返回 bounds；draw 实际追加文字图元。FrameContent 用真实纹理数组索引编码 ShapeStream，向同一 TextRenderer 追加矩形。
- TextPrim 6 MiB、ShapeStream 4 MiB、字形 atlas、上传和本帧输出均由真实 GPU owner 产生。没有在宿主制造图元数量。
- 原调度：scale 在 Update set 7；prepare text → 三个 Taffy 系统 → debug_hide_ui → render text 在 set 44；mei_post_update → flush 在 render text 之后。编译 OOM 重试复用已经上传的该帧文字结果。

本轮进一步按原函数恢复了 frame_events、双缓冲节点复用、最小尺寸与位置布局、递归 RON 模板/slot/本地化 Label。加载 Splash 并非模板演示：原 `SplashScreenPlugin` 注册 set 36 的 `ui@141ec1390`，调用 `140f99420`，创建四个实际纹理节点 `menu/pounce_light_logo`、`menu/heart`、`menu/noknok_logo`、`menu/fmod_logo`，保留原尺寸、间距和 LogoAnimation 淡出公式。对应 toolbar 的 22 项库内测试已通过；`file:` 外部图片缓存/上传分支仍未恢复。

MEI 的确认/导航动作在原版与 actions 的 digital 19..23 是同一存储。宿主保留其完整状态投影，不再让独立 nav/click 字段永久停在 Idle。

## core 与 rhapsody 的实际边界

`FrameWorldAccess` 将原 PassResources 的六个确认身份映射到真实类型借用：shader、texture、SSBO、VAO、placement、同一个 DynamicConstants。原 ShaderWatch 是可选项。没有把任意 Bevy 资源强转成旧编译器布局。

共享 FrameSetup 恢复原 preset import、纹理数组发布、实际 FrameConstants 0x1090 字节复制、动态常量分配和 preset export。FrameConstants 的完整头部与五个 view 的偏移由原 CPU 写入和原 SPIR-V 反射相互验证；知道布局并不等于已恢复逐帧 producer。

原 `init_color_library@14082c000` 已恢复：37 项默认调色板 → `add_new_with_data` 上传一个元素 → CPU resource → `watch("assets/colors.json")`。此函数没有读取 JSON。加载宿主使用同 App 的真实 BufferHost 执行该生产者，避免仅手工插入 CPU 颜色或伪造 SSBO。

SSBO 按需导入现在取得同一 frame/session 的真实 RenderGraph facade；compute 构建器不再在整个闭包期间持有 RefMut，避免库回调重借失败。帧结束只导出实际已导入的 SSBO 并清其图句柄缓存，不预导入全库，也不让上一张图的 ID 混入下一张图。

宿主 shader heap lowering 是明确的平台/接口适配：保留原 shader 参数位置、纹理数组索引和实际资源 owner。不能用资产 ID 当 Vulkan image/buffer 句柄。

原全屏 resolve 闭包缺失的 `framebuffer_dims` 已按 `0x14090d875/0x14090d935` 与 `0x1409d04f6/0x1409d0506` 恢复为交换链输出尺寸。MEI 的全屏合成实例没有此操作，所以没有修改公共 raster_single 助手去影响它。

普通 mesh draw 从真实 typed VAO handle 查 AssetVAOLibrary，按 shader 反射名/location 绑定实际顶点缓冲，绑定真实 UINT32 index buffer。原 kind 3 payload 是 instance_count，原绘制数量来自 VAO entry 缓存；不能把 mesh handle 当 VAO handle，或固定一组顶点格式。

## 编译、执行与展示

原外层 render 顺序为主图编译 → 主图执行 → acquire → presentation 图 → present。编译阶段必须可以失败/降低分辨率重试，不能在“compile”时已经调用 GPU pass callback。

当前桥接拆分 declaration、物理 compile/allocation、execute。未执行的 compiled graph 被放弃时会取消并退役；执行返回的是真实 GPU receipt，GPU 完成通过真实 timeline/fence 验证。

原 presentation 证据：`0x140166aa6` import_owned 主图源 `0x22` → `0x140166aca` import_unowned acquired target `0` → `0x140166b07` 原 resolve → `0x140166b4c` export `0x27`。

交换链图像借用明确持有真实 Swapchain Arc，不制造 MemoryBlock、不销毁借用 VkImage。同一图的实际提交等待 acquire、发出 rendered signal/fence，present 消费该实际提交结果。CAMetalLayer/winit/WSI 借用封装属于 macOS 宿主适配，不宣称原 Windows OS ABI 已原样移植。

## 尚不能称游戏恢复的直接阻碍

1. 原 compile_main/world graph 的生产 FramePasses consumer 尚未完整实现；它还包含 render_world 54 个调用之外的 LUT、菜单、painterly、Vermeer、截图、光标和 UI 分支。当前返回明确错误，没有用清屏或空 pass 实现它。
2. `prepare_frame_constants@0x140af40b0` 的十四项真实输入、canonical OutlineStairs、五个 view 与完整 `0x1090` 头部已恢复；共享 ECS 测试已实际通过。生产天气/TOD/帧常量注册和相机真实初始 FOV 已合入；完整 gameplay 图仍未执行，不能将有界 ECS 验证扩大为游戏相机已完整运行。
3. 原 `init_populate_ssbo_library` 的完整 72 项注册仍待全部恢复并按原顺序接入。Nani、Brick 子集、MouseDepth、NormalDebug、草叶/落叶/发射器和 Ripple 的实际构造已纠错，但这些子集不能替代完整初始化。CustomCursor mapped pointer 与同 BufferHost 的强 lease、切 App writer 退役和 EventCursor 重置已接入源码，尚待当前整体验证。`gpu_generate_draw_lists@0x140987dd0` 的 11539 字节原主函数、真实分配/索引/参数生成和旧 indirect 空闭包现已重构；其 startup_nani、真实 48 字节 Placement、16 字节 atlas 布局和 mesh-part count 也已补入。原调度安装、完整世界 producer 与 GPU 场景运行仍待闭合，不能把可调用函数当作整个世界图已执行。
4. v22 已实际运行真实 intro 的 13 条历史命令，经过生产 History/replay/墙体系统后得到原存档对应的三面墙（27、28、29）。这关闭了该 fixture 的历史到 live wall authority 链；它不包含后台 ready、完整 snapshot 装载、世界图 warmup 和 App 切换。存档恢复按当前源码继续独立审核，不能沿用旧统计或旧缺口判断。
5. 输入录制格式、自定义光标、设备丢失后的原窗口/设备重建，以及截图真实读回仍有未恢复分支。
6. 上层包中的墙/楼板几何产出、prefab 消费、屋顶瓦片、动物类型身份等，需要依据当下函数体继续逐链复核。此前十四包表是审查基线，不是最新工作树的完成统计。

## 验证记录

工作树在多人并行修改；每项结果只约束其日志时刻的源码。新增功能合入后重新检查，不能继承较早通过结果。以下是实际运行结果，测试数量不代表游戏恢复比例。

| 范围 | 实际结果 | 直接证据 |
|---|---|---|
| country-core 启动状态/session 生产者 | `cargo check` 通过；未证明世界应用 | `logs/core-producer-check.log` |
| tiny-glade 全目标（该日志时刻） | `cargo check --all-targets --locked --offline` 通过 | `logs/original-chain-all-targets-check.log` |
| MEI/Splash/布局/动作身份 | toolbar 22 项库内测试通过 | `logs/original-mei-tests.log` |
| RG 原 access mask 与内容依赖 | rhapsody 21 项资源合同测试通过 | `logs/original-ledger-content-tests.log` |
| 编译、取消、真实执行边界 | 本机 GPU 测试通过 | `logs/strict-gpu-v7.log` 中 compile_before_submit |
| SSBO 同图及跨帧真实读写 | core_render_bridge 3 项本机 GPU 测试通过，含真实 staging/readback 和未使用库项保持 | 同上 core_render_bridge |
| Egui 原 shader、顶点/索引/字体上传 | 本机 GPU 像素回读通过 | 同上 original_egui_gpu |
| 原 mesh VAO/实例数与 resolve | 两项本机 GPU 像素回读通过；resolve 依据原 OpImageFetch 验证整数坐标及翻转，不期待缩放 | `logs/strict-raster-v7.log` |
| 原 loading 背景、Vermeer、MEI 与 Logo | v11 本机 GPU 测试通过，实际两份原 Logo 网格、shader、绘制回调、真实 ColorLibrary SSBO 均参与 | `strict-loading-original-logo-v11.log` |
| 加载画面实际 WSI | v12 连续三帧实际 present 与 GPU/交换链 shutdown 完成；测试输入为明确的 loading World | `logs/strict-loading-wsi-v12.log` |
| 原主图前缀实际 GPU | v15 两帧真实 GpuWorld 子集导入、LUT 原 shader 与纹理、四个 indirect reset、原 resolve 回读均通过；第二帧 LUT 不重复声明 | `logs/strict-main-prelude-v15.log` |
| canonical 帧常量 ECS | v13 真实 Schedule 连续运行，验证五个 view、jitter、OutlineStairs 早停后保留未读事件 | `logs/canonical-prepare-ecs-v13.log` |
| RT 上传/绑定与 indirect 规格 | v13 RT 14 项、canonical 身份 5 项、实例数绑定和 indirect 启动规格通过；未验证完整 BVH 生产 | `logs/original-rt-unit-v13.log`、`logs/original-rt_bind-unit-v13.log`、`logs/original-rt_identity-unit-v13.log`、`logs/original-indirect-unit-v13.log` |
| 原 RT BVH 算法与 worker | v19 RT 共 19 项 CPU 测试通过，包含实际 CWBVH 构建、三角形/metadata 同步重排、原 half 打包、变换/非有限过滤、异步删除后的空结果；非原进程 GPU 差分 | `logs/original-rt-bvh-unit-v19.log` |
| 实际 intro → live wall authority | v22 运行真实 13 条命令、64 次调度，最终墙 27/28/29 的形状、尺寸、队列和错误事件均验证通过；1 passed / 0 failed | `logs/original-intro-live-authority-v22.log` |
| canonical core 设备 → 同一 BufferHost | v23 三项严格本机 GPU 测试通过，canonical ECS 资源实际执行 create/upload/map/readback/release，跨帧 lazy import 及取消/失败退役同样通过 | `logs/strict-core-device-bridge-v23.log` |
| temporal 历史有效性 | v26 两项严格本机 GPU 测试通过，首帧、提交后的历史、尺寸变化和取消均覆盖；此前 v18 真实失败被保留 | `logs/strict-host-temporal-v26.log` |
| 实际 typed raster / 间接命令 | v26 两项本机 GPU 测试通过，含 typed VAO、kind5 同库按需导入与多帧像素结果；本机未启用 DrawIndirectCount，kind7/8 只验证明确拒绝，未执行其 GPU 绘制 | `logs/strict-raster-indirect-v26.log` |
| 六路上传 / 首帧 reprojection fixture | v26 两项均失败于 fixture 没有调用真实 init_color_library；已补原 producer 和同 BufferHost 上传，待重新运行，不记通过 | `logs/strict-scene-compute-v26.log` |
| 完整实际窗口启动 | v11 创建 OS 窗口、Vulkan surface/device（Apple M3）并进入事件循环；后台于 frame 0 在真实 intro 历史应用缺口报错退出 | `original-window-startup-v11.log` |
| 新统一 History → intro 墙与空间发布 | v30 重跑实际通过 1 项；13 条原命令经过修正后的共享入口、墙 authority 与空间哈希；不包含加载 ready 或场景 warmup | `logs/original-intro-live-authority-v30.log` |
| 屋顶尖顶、真实构造与 mesh-part 绘制数量 | v31 有界 CPU 5 项通过，含尖顶过滤、实际上传数据、失败时不更新数量及空结果；未执行完整屋顶 GPU 图 | `roof-algorithm-validation-v31.json` |
| 原 Nani master 的真实一实例 GPU | v30 已进入真实设备，执行时因 bindless storage buffer 的 device/usage/range 检查失败；0 通过，1 失败。正在核对原 uniform owner 到当前 storage heap 的物理用法适配 | `logs/strict-nani-failed-v30.log` |
| 补真实 ColorLibrary 后的 scene GPU | v30 两项实际失败：首帧 reprojection 的历史被初始化合同拒绝；六路栅格 readback 使用原 resolve 却按未编码字节比较。原发行 SPV 另证实 resolve 做线性到 sRGB 转换，修正 fixture 待重跑 | `logs/strict-scene-failed-v30.log` |
| 新源码完整窗口启动 | v30 原生窗口、Apple M3、OS 事件循环真实进入；frame0 后台加载 channel Disconnected，退出码1，未收到 SubApp。日志未提供真实后台退出原因，不能据此声称当前唯一错误仍是旧 intro 缺口 | `logs/original-window-startup-v30.log`、`window-startup-validation-v30.json` |
| 六路真实栅格 GPU 上传与复用 | v32 严格本机 GPU 1 项通过；六路实际输入、原 shader、resolve 的原 sRGB 公式及连续两帧复用均验证。未包含 reprojection 或其余场景 | `logs/strict-six-rasters-v32.log` |
| Ivy 三构造与宿主 uniform 用法适配 | v32 有界 CPU 3 项通过；NAME、canonical 类型、容量、None/Some 与独立 count，以及原 uniform 到当前 storage heap 的物理 usage 适配；不包含 Ivy CPU 生产算法 | `gpu-and-storage-validation-v32.json` |
| Nani 一实例 GPU 后续 | v32 原绑定用法检查已不再报错，执行入口返回 Timeout；实际阶段尚未定位，不能认定是 GPU 等待或已闭合 draw-list 生产 | `logs/strict-nani-failed-v32.log` |
| Nani 实际 GPU 内容链修复 | v33 确认参数描述符256槽耗尽；v35 实际图仅9pass且缺原clear，GPU前拒绝。按发行shader的atomic/in-place/分区写显式保留先前内容后，v37 图18pass保留，真实一实例命令、indirection、visibility回读通过1项；不包含custom/external子集或HZB第二轮 | `logs/original-nani-v37-gpu.log`、`gpu-and-host-validation-v37.json` |
| 首帧历史与场景上传 GPU | v37 严格本机GPU2项通过：history_valid0时仍绑定同步但不读旧深度，原reprojection和Copy-depth实际执行；六路栅格两帧复用也通过。一般未初始化读检查保留，新RG能力CPU5项通过 | `logs/original-scene-v37-gpu.log`、`logs/original-content-v36-cpu.log` |
| 最新业务与实际宿主 | v37 最新History生命周期下，原intro→墙体/空间发布1项通过。完整OS宿主到达真实完成事件后的world warmup，明确返回compile_main FramePasses consumer未接通，未交接SubApp。v30未知Disconnected未复现，不能倒推其原始原因 | `logs/original-intro-v37-cpu.log`、`logs/original-window-startup-v37.log` |
| 条件 history 的图像视图隔离 | v38 资源合同 CPU21项通过；实际Vulkan设备上新增视图声明检查1项通过（4种排列），无效history只跳过对应mip内容，交错buffer不偏移绑定索引，有效mip仍拒绝未初始化读取；该项没有GPU提交 | `view-alias-validation-v38.json`、`logs/original-view-alias-v38-gpu.log` |
| 水/花园真实遮罩消费者 | v40 严格GPU4项通过中的新两帧链：W1上传→Water4pass/Garden2pass→原resolve回读，R/G/B通道和clean-frame跳过均正确；原SPV的vertical固定读G与第三次temp部分写已保留。未替代W3..W11 | `gpu-and-source-validation-v40.json`、`logs/scene-v40-gpu.log` |
| 图集实际发行资产与原程序网格 | v40 CPU2项通过：真实RON/JSON→Atlas→AssetMeshLibrary，19个程序网格及encoded indices；roof按原IAT证明的sinf纠正，未并入未齐的PrefabPart启动/热重载 | `logs/shipped-atlas-v40-cpu.log` |
| 原deferred启动与类型输出 | v40 四材质原Startup producer已接真实GameLoop gameplay Startup；设备fixture成功且CPU构造/附件/typedworld等相关54项通过。W35..43可调用连续段已编译；整帧caller仍缺。v40没有重跑完整OS宿主 | `gpu-and-source-validation-v40.json` |
| 下一组原版 SSBO 与零长度常量页 | v43 水/花园/船/楼梯/UI 构造及动态常量共39项选定CPU测试通过；新SSGI测试另有5通过、1失败，失败为测试硬编码反射绑定下标，原日志保留 | `constructor-and-constants-validation-v43.json` |
| BDA 实际页 owner、精确 range 与提交完成 | v44 严格本机 Vulkan 1项通过；CPU chunk/recorded pass释放后图仍持同页两range，取消与真实提交完成前不能复用页，完成后可复用并读回实际GPU结果。诊断shader不解引用BDA输入，不算W3原算法验证 | `bda-owner-validation-v44.json` |
| SSGI 条件历史声明 | v46 六项CPU合同测试通过；仅无效历史的那个绑定免旧内容要求，保持原access/group。v43失败的测试断言已按真实owner修正；v45被另一owner进行中的TerrainEditingData迁移编译阻断，v46迁移完成后执行通过。未验证完整SSGI GPU链 | `ssgi-contract-validation-v46.json` |

严格 GPU 命令设置 `RHAPSODY_REQUIRE_VULKAN=1`，使用本机 Vulkan loader/MoltenVK。普通沙箱不可访问 Metal，因此实际 GPU 测试在可访问设备的宿主权限下执行；没有把跳过测试计作通过。

v11 loading 测试保留有界文字验证输入，但 DrawLists 已不再为空：从原 JSON 网格 Startup 和实际资产维护生成两项 pass 18 绘制。它验证原加载图的真实 GPU 路径。随后 v12 独立 WSI 可执行测试通过：原生 OS 事件循环中的加载 World fixture 调用同一 ProductionRenderer 与原外层 render，连续三帧实际 loading → 原 resolve → 交换链提交/present，再等待并正常释放资源。日志 `logs/strict-loading-wsi-v12.log`。该测试不启动 gameplay 后台线程，不发送 LoadingGameComplete，也不证明 gameplay 世界恢复。

窗口 v8 的旧 objc2 0.4.1 在系统 NSScreen 数组的 `q/Q` 方法编码检查上退出；开发 profile 对该精确旧包使用其 release 校验行为后，v10/v11 已实际创建窗口。此项是 macOS 依赖适配，不计作原游戏业务恢复。v10 暴露 Logo 的颜色 SSBO 栅格绑定缺失；已改为同帧、同图真实库按需导入，并由 v11 的实际 Logo GPU 测试覆盖。

rhapsody 日志中的“内存统计溢出/ID耗尽”等多条信息来自 eager error 构造的可疑路径；不能仅凭这些行认定 GPU OOM。实际 v11 唯一 panic 是 `load_game_system` 的显式原业务缺口。独立的 v12 加载画面 WSI 验证现已通过；完整 host 仍按真实 intro 业务缺口失败。

逐帧 TOD 的无音频入口及 canonical 帧常量在 v19 的有界 ECS 运行中通过；canonical LoadingGameComplete、intro typed History 与相机变更已进入 v22 编译和实际 intro 调度验证，仍未执行整个标准启动。main prelude 的真实 GPU 验证此前两次分别被磁盘不足与应用 glam 依赖缺项阻断，均未运行测试；修正后 v15 严格 GPU 运行通过，1 passed / 0 failed / 0 ignored。该验证是明确的主图前缀 fixture，不包含世界54调用、后台 ready、完整 host 切换或 gameplay present。temporal 验证 v16/v17 因验证层路径失败；v18 成功进入真实 GPU 后暴露 RasterHost 尚存三处历史标志取反，随后修正并在 v26 严格 GPU 重跑通过。旧失败日志保留，不再将该取反列为当前未修缺口。

### 手写算法的原证重构

原版复杂度高于手写实现时，在现有模块中替换错误算法，保留当前架构和真实 owner；不以外层包装或并行旧实现代替。RT 是已落地的例子：原锁定的 `h3r2tic/obvhs` commit `a3f3b3bfad18f3dce0b0505a927200b3575477d0` 以未改动源码复用，实际 CWBVH 构建参数、原始三角形变换/过滤、primitive index 重排、24 字节 GPU triangle 与 4 字节颜色 metadata 均由完整原函数范围核对。此前的 4 字节 triangle、opaque BVH、任务计数式 glue 已被真实几何、80 字节 CWBVH node 和 request/completed/busy 状态替换。原六项正常运行依赖与原锁一致；单独依赖比较中的十项可选/开发依赖版本差异仍明确保留，未宣称整个依赖锁完全一致。

canonical `GraphicsDeviceResource` 原先仍走 `Unsupported` 的 SSBO 兼容入口。现在将同一现有 BufferHost 接回这个资源，所有 create/update/map/release 使用同一设备与句柄登记；v23 已通过实际 GPU 验证。此项是当前重构架构的调用适配，不能将适配代码本身计作原游戏算法恢复。

本轮还直接重构了错误的数据生产：WoodenPlank 从 32 字节/错误分块改回原 64 字节和 4096 chunks，矩形/圆形 timberframe 使用原完整仿射变换、基高、主高、裙边、ridge 与 radial 分支；原事件的屋顶 gate 位于第 28 字节，不能再读取 shape 内的第 24 字节。草叶从零尺寸占位改为 32 字节×0x90000；落叶 32 字节×0x340 并恢复原非零初值；发射器 16 字节×1100，Ripple 为完整 528 字节聚合。None 是否上传逐个按原函数恢复，未用统一清零代替。v28 的有界 core 验证实际通过 50 项，覆盖真实 RT worker、这些构造器、Nani 数据合同、墙体与历史；随后另跑 WoodenPlank 稀疏默认值 1 项通过，v29 矩形/圆形 timberframe 实际 ECS 算法 2 项通过。日志和固定可执行文件 SHA 见 `core-algorithm-validation-v28.json` 与 `logs/original-{wooden-v28,timber-v29}.log`。这些是明确选定的 CPU 验证，不表示全部 core 测试或整个场景通过。

墙体构造原先只产出砖和 trigger，现补实际 trigger triangles → 同 RtWorld 的位置替换、墙 albedo 引用和 UpdateRtWorldEvent。独立原指令复核同时纠正此前将 `impl$2::from@1408bc510` 误标为 handle 构造的归属：该处是 albedo 变体；真正 wall handle 来自 `impl$5::from@1408bc520`。当前保留既有工程 API，撤销错误原证注释，不为错误归属维持双业务实现。原 set28 完整九系统组尚未全部恢复，因此这项不被虚报为标准应用中的墙绘制已完成。

历史处理也发现了仅靠 intro 几何测试无法揭示的错误：byte3 控制原 PlayReplayAudio，真正 NewReplayCommandEvent 是 ZST 并在实际 queue pop 后发送；InterruptTimelapse 是另一个 ZST 身份。原录制门位于新命令 read 之后，且未有新命令时不应阻断已有队列消费。旧拆分、额外门控的手写实现已合回共享的真实业务入口；v28 对新入口的 14 项历史测试实际通过。v30 已重新通过真实 intro 及墙空间发布；同版完整 host 仍因后台加载 channel 断开退出，原因尚待新诊断确定。

## 原始证据入口

相对于 `games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/`：

- `TG-0ef1874c29a03d0dec/main-chain-frame-20261018/`：完整 render/compile_loading/compile_main/compile_present RNE、原 shader producers、RenderContext finish。
- `TG-727475d65610eb845f/main-chain-frame-20261018/`：原资源 TypeId 到具名插入函数的核对。
- `TG-284eb5c66a03249603/main-chain-frame-20261018/`：三个 raster_single 原实例及 resolve 入口完整 RNE。
- `TG-379258a80656c7937d/main-chain-frame-20261018/`：prepare_frame_constants 的 fresh 原函数。
- `TG-07278b540192d36392/main-chain-frame-20261018/`：ColorLibrary 初始化完整 RNE、默认构造、fresh 反编译和 producer 合同。
- `TG-4c44632743afd6f0a3/main-chain-frame-20261018/`：AnimatedLogo build/startup/animate fresh 原函数。
- 各 boot、OS、MEI、renderer claim 目录保留其独立原函数包、合同和源码 manifest；汇总文档不能替代这些原证据。

详细原主图调用点与当前真实消费者见 [MAIN-GRAPH-CONSUMERS.md](MAIN-GRAPH-CONSUMERS.md)。其中无条件 `GpuWorld.import_to_rg → FrameConstants 上传 → render_luts → reset_indirect_args` 在 `show_world` 判断之前；菜单路径也不能用空图绕过它们。


## 普通 Steam 入口与场景启动生产者的独立实查

普通 binary 当前仍是 `AppSteamIntegration::try_new(|_| None)`，因此必定失败；`main_with_steam` 只是 embedding 成功入口，显式 `tiny-glade-host` 是另一条宿主适配路径。GPU/WSI 已可运行不能消除这个启动阻断。当前 Cargo.lock 没有 steamworks；原锁为 steamworks 与 steamworks-sys 0.11.0、PounceLight git revision `d05eaa60db328156aeee172b0b157f3f69c70b30`。本机检查 Applications/用户 Applications/Steam数据目录/Cargo源码缓存与工作区，只找到Windows x64 steam_api64.dll，没有可用的Mac Steam client/SDK库。未安装或下载SDK，也未伪造初始化成功。

当前 SteamSdk trait 还没有真实 callback pump、callback注册/owner 与游戏语言方法。`add_steam_integration(World)` 只初始化资源，没有安装原 startup/input 调度；GameLoop没有callback pump。原RNE明确 `GameLoop::main_loop@1404de4f4` 调 `Client::run_callbacks@1400e6f90`，位于实际 render 调用之前；原App语言来自同客户端ISteamApps。故后续真实backend集成还必须恢复这些消费者，不是替换工厂一行即可。完整限定证据在main claim `steam-backend-audit`。

场景原 `add_game_startup_systems@140941110` 注册的PreStartup是外层四项chain：`create_preset_textures` → 十项startup tuple → `load_mesh_atlases` → `startup_nani`。内层十项是普通into_configs tuple，不把它们误称已证明的十项顺序链。outer chain@140a97110真实调用Chain::set_chained@140d2fa20；内层@140a2bf40写count10和未chain初态。load_mesh_atlases@1412a00ae实际调用MeshAtlasLibrary::register_meshes，随后外层才进入startup_nani。

该文件族已合法领取TG-1a0d194affaf7f0d17，生产 `startup/load_mesh_atlases.rs` 现已加入基于原指令的安全图集主体，并在v40用真实发行RON/JSON验证。它保留现有MeshAtlasLibrary架构；完整PrefabPart初始/重载wrapper与PreStartup注册仍未闭合。完整72 SSBO、真正prefab及原startup调度未齐前，不能把startup_nani可调用算成真实场景启动成功。

2026-10-18的端到端可玩主链进度图为约40%（人工工程估算，合理区间30%–50%），依据当前源码、原版证据和已执行的实机结果，不按测试数或文档标记计数。窗口/loading呈现已验证，局部GPU消费者已连接，完整world图和可玩存档闭环仍未通过。权重及各项判断保留在 `progress-estimate-2026-10-18.json`，图为 `progress-estimate-2026-10-18.png`；后续尚未验收的代码不自动提高百分比。

## 完整启动与实际消费后续核验（v50–v63）

v50 原 loading Startup 两项canonical SSBO producer及MEI强owner接回实际App；CPU1、真实loading GPU1、真实窗口三帧present均通过。v52 W3原terrain stroke shader实际三帧提交、完成与读回通过：首帧四pass，clean帧零pass，真实XYZ修改+重算事件再四pass；仅W3，不代表W4..W11或完整世界。Gauss PERM_fmt必须含原literal引号，原formatter是Display，不能猜Debug。详见对应loading-startup-validation-v50.json、terrain-gpu-validation-v52.json。

完整72项SSBO producer已按原140909e80全部72个add调用核实并接回gameplay PreStartup。v56 CPU真实记录后端验证72项，72次物理创建、380772588字节；publisher actual bulldoze membership三阶段测试1pass。v58实际M3 GPU创建72个canonical资源并核同设备/非零真实owner/唯一buffer及一次cursor映射事件，通过1项；这不是shader执行或GPU专用内存初值验收。v56曾错误试图直接map GPU_ONLY初值，实际失败Unsupported内存不可映射，失败日志保留；后续owner fixture删去不合法的映射假设，不更改原usage。

PrefabPart/loader/atlas初始与重载wrapper已在现架构增量整合真实Mesh Arc/subset与独立RT/audio AABB，保留当前PrefabInstance完整48字节PlacementData及hot reload消费者。音频必须exact Tree，+28是String长度4，不是kind tag；磁盘移除不自动删除原library条目，reload只重建通知或新条目。v61 bounded CPU共35通过（DoF7、prefab loader3、startup13、prefab system12）。实际发行prefab→atlas→Nani及完整PreStartup接线正在实机验，不从这35项推定场景启动。

DoF原12099B和实际发行SPV核出offset初值需补到8word/32B、init push含7、fullres派发slots为1/2/0。相关九pass已接真实同allocator/同session uniform范围。v62实际GPU未通过：同32B既声明native graph read又自动当private upload read，提交guard正确拒绝。当前已窄改用现write_lease保留声明角色，不停用guard、不新建allocator，重验待完成。

本机官方vulkaninfo实际drawIndirectCount=false（Apple M3，MoltenVK），旧尾命令不会自动清除。平台fallback必须按实际GPU count处理有效前缀并清scratch尾instanceCount，不能最大条数直接画旧buffer。首次TAA/TGSR原shader确有无条件历史采样，原新temporal分配也明确无初始化；不能假造clear或误标inactive，当前严格未初始化内容合同仍在对齐。

最新完整OS GameLoop验收仍停在原compile_main world consumer未接；新W4/W5/W9/Grass、gameplay资产调度只是源码接线，尚未代替完整世界图/交互保存闭环。各版本日志与不可变测试程序指纹见startup-prefab-dof-validation-v56-v62.json。


## 主链实际增量（v64–v70）

v64 景深九个原版 pass 已在 Apple M3 GPU 成功执行，真实 32B 初值表、7 个间接调度槽、两个坐标分区与最终 resolve 像素均通过。修复是显式 native UniformBytes 改以真实 lease 绑定，避免同范围误判 private uniform 二次上传；保留严格读写 guard。

v64 真实发行资产 PreStartup 失败后，完整原指令确认空 trunk 删除 id 后跳过 appear_pos/age 补齐，Mesh 非空断言本身正确。窄修后 v69 同 GPU 启动通过：25 个 prefab、39 个部件、27 个 Nani 子集，真实 mesh Arc→atlas→实际 GPU owner。完整 Startup 的 Terrain 扩展正在另验，不能据此声称所有启动阶段完成。

v69 Apple M3 的 GPU DrawIndirectCount 平台适配通过原始命令偏移/stride、计数 2→1→0→2→3(cap2)和两个独立 scratch 的像素测试。原 Nani 不清旧命令尾部，不能把 count 简化成固定 max；适配复用同 heap/compiler/图/实际 GPU counter，不回读 CPU。

v70 显式允许原版未定义历史读取的当前图元数据已合入，5 个集成和 1 个真实缓存边界 CPU 测试通过。仍保留 reads_contents/依赖/同步/owner，不把 history 清零或虚报初始化。原 TAA 两帧 GPU 通过；TGSR 首帧为 [255,0,22,255] 触发旧纯红断言，原 shader Frexp/Exp2 写出的蓝通道 2^-7 经 resolve 恰为22，最窄预期已按原证修正，第二帧仍待重验，不能预填通过。

History 默认已迁 canonical MaybePendingSnapshotId，v69 整库实跑 2403 通过、22 失败，失败均为旧 fixture 的 History<()> 与新默认资源 TypeId 混用；不插入两份历史来隐藏。旧 grid 测试传 u32::MAX 行给恢复后的原循环，停止了本任务独占测试进程并改为小尺寸验证原 wrapping 运算，初次日志保留。

完整世界 W47..54 已接入当前 typed 后处理入口，W54 原副作用是反馈完整36B纹理替换，不是 desc_tail_copied 布尔；新同批编译/GPU正在进行。此前 geometry/植被/墙/草/水面、compile_main 与真实 warmup、相机工具交互以及 Motomoto 存档恢复仍需逐个接通；production world guard 仍在。详见 main-chain-validation-v64-v70.json 与 logs。


## 主链实际增量（v74–v77）

v74 真实发行资产 Startup 已含 Terrain：25 prefab/39 parts/27 Nani 子集和8个地形实体在同M3设备/owner链通过；TAA2帧与TGSR2帧真实像素通过，两次TGSR蓝22均来自原SPV偏移量，不是放宽容差。输入/profile/事件维护与typed roof/mouse/visibility接线编译通过，host29CPU通过。

v75 后台真实GPU frame0→主线程原App frame1通过，同named历史owner、actual Arc、奇偶帧和像素连续；无unsafe Send和第二套asset表。W54实际捕获参考→整36B纹理替换→差异像素GPU通过；temporal15CPU通过。九级SPD首次失败InvalidBounds，随后按原1623B+535B消费侧证实None视图表示base起剩余级数，原bind_individual_mips须逐级数组；v77修当前host后真实九级两帧、最后两级mip回读、24B实际零buffer与同counterArc通过。当前fullybound ABI未用尾槽复用最后真实view，原SPV gate证实无越界访问，差异有界记录。纹理历史仍首帧false且None未初始化，buffer实际原zero上传与可用history独立。

v74整core2432pass/3个植被新fixture误期望重复resource0；独审保留生产guard，期望依原当前builder去重。v77整core2435pass/0fail。v76 autofocus Vec2→array编译错保留日志，修转换后通过。

新连续world_graph入口已按原W1..54逐项串接实际typedconsumer，W21/27追加原墙8B push/highlight高位和Prev/Next第二轮交换，W7仅查询canonical optional StairsTopDownMesh，绝不插默认Some；Surface已原地提取prepare/raster/finish安全借用同World，并增量canonical flags0x108与fragment/RMW元数据。**这些新源在下一批编译/真实整帧验证中，不能据局部GPU通过称整图已成功。** 当前production compile_main/warmup完整世界guard仍保留，canonical draw callbacks/完整clearing/相机交互/存档读回闭环继续推进。

证据与失败日志：main-chain-validation-v74-v77.json / logs；冻结二进制SHA在JSON，未复用编译中可变target二进制。
