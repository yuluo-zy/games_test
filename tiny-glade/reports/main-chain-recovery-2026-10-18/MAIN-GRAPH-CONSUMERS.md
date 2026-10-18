# 主图真实调用路径与接线缺口

2026-10-18，直接核对原 B 指令、PDB 泛型实例与当前 Rust 源码。本文不以历史统计、文件存在或 CPU 录制测试判定生产链完成。B RNE SHA：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

## 当前实测断点（v37 / v38）

直接复读本目录真实执行日志：`logs/original-nani-v37-gpu.log` 的必要18个原pass全部保留并完成真实1实例的命令、instance indirection、next visibility回读；`logs/original-scene-v37-gpu.log` 的六个实际mapped raster两帧上传及首帧reprojection+depth copy均通过。旧v26缺ColorLibrary、v30 InvalidUsage、v33描述符耗尽及v35 initializer被裁剪均为已保留的失败历史，不代表当前断点。v38实际view-alias GPU回归也通过；普通未初始化读取仍被严格拒绝。

`logs/original-window-startup-v37.log` 当前生产世界入口明确报告原compile_main的FramePasses未连接。局部GPU通过没有发布后台gameplay ready，也不能将loading-only的WSI呈现作为完整游戏验收。

本轮按原机器码又发现W43延迟光照把color/depth附件接反，并查出缺失的原`DeferredRasterResources`和`startup_deferred_raster@140a97a30`。窄修候选、原完整6203B/310B、PE字符串及原Startup八项tuple注册链交根整合。新的`world_lighting`候选将W35..43串成真实连续图：half-res→reprojection→SSGI→color history→RT三纹理→radiance→光源分格/DI→screen shadows→deferred。公共输入/输出均为现工程真实typed值；候选尚未编译或GPU执行，W2..34和W44..54缺口没有被隐藏。

## 主图外壳不是只调用 render_world

原 `RenderGraphState::compile` 主图实例 `0x140161cd0` 的有界指令与符号调用表，在 `TG-0ef1874c29a03d0dec/main-chain-frame-20261018/evidence/compile_main.asm`、`compile_main-calls.json`。同目录反编译仅作辅助；条件跳转以原指令为准。

| 原调用点 | 原操作 | 当前真实接入边界 |
|---|---|---|
| `0x140161efe` / `0x140161f14` | 导入 texture presets / 发布 bindless table | FrameSetup 已用真实 AssetTextureLibrary、同 App TextureHost；原索引保留，不能截断全局数组 |
| `0x140161fbc` | `GpuWorld::import_to_rg` | 在 show_world 判断之前，无条件；现有 GpuWorld 遍历 Owned subsets。CoreGraphFrame 新 helper 可导入同 session 的真实 BufferHost，主图调用仍须安装 |
| `0x14016210a..0x140162193` | 分配并绑定 FrameConstants `0x1090` 字节 | FrameSetup 使用真实 DynamicConstantsChunk；ShaderBindings 在 compute/raster 的共同参数路径消费 |
| `0x1401622e7` | `GpuLutTracker::render_luts` | 已按 fresh RNE 恢复原插件 producer、两条原 LUT shader callback、source-hash 记录及首次构图标志；同帧 main prelude 可调用，完整主图尚未接入 |
| `0x1401622fd` | `reset_indirect_args` | Core consumer 已有真实四块 staging 初值与四次 copy；main prelude 调同一 CoreComputeFrame、按需导入四个真实 SSBO；专用 0x122 producer 已修，但完整 init_populate_ssbo_library 启动调度尚缺 |
| `0x140162af9` | `render_world` | 当前生产 compile_world_graph 明确错误；FramePasses 只有 CPU 录制实现，不能将接口数量当完成度 |
| `0x140162f4e` / `0x14016302b` | bidirectional blur / final_post | core 算法与 RasterHost 有消费者；仍需主图准确输入、图内导入和配置资源。final_post 需要真正的 4× MSAA 目标，不能按单采样替代 |
| `0x14016310d..0x140163277` | menu background → Painterly → Vermeer | EffectsHost 已承载 Painterly/Vermeer 描述；主图仍缺 menu 缓存、状态和条件接线 |
| `0x1401632d3..0x14016350a` | logo / previews / gallery / feedback / world UI / 2D / MEI / cursor / loading UI / Egui | MEI、普通 typed VAO、Egui 与部分原 UI consumer 已接；不能跳过 previews、gallery 或条件后宣称原主图完成 |
| `0x1401635be..0x140163788` | 导出输出 → RenderContext.finish → presets / SSBO / GpuWorld export | FrameSetup 结束导出 presets 和实际按需导入的 SSBO；主图必须另调用 GpuWorld 对称 export。不得在 loading/present 乱导入 Nani subsets |
| `0x1401637e7..0x140163c1f` | temporal export / 资源登记 / retire / CompiledRenderGraph | CoreRenderBridge 与同 session 的编译、执行、取消和退役分别处理；不执行 builder 就不能称业务图存在 |

原世界分支还受 AppMode、EscMenuState、Painterly 捕获状态、FrameConstants 重置、MetaCommander profiling 和 debug RT override 影响。原指令 `0x1401624ad..0x1401624c7` 先检查 AppMode 字节与 `should_show_world`；`0x1401627be..0x140162840` 还处理 profiling。菜单可跳过部分世界计算，但上表的 Nani 导入、LUT、indirect reset 仍在判断之前，因此“先展示菜单”也不能绕开它们。

## render_world 的可接入批次

下列组次来自 `render_world` 原调用顺序，源码位置 `country-core/src/systems/render_loop/render_world.rs`。它的 `FramePasses` 方法只是游戏消费者的接口；生产适配必须在每次方法中调用已有 core 入口并录入同一帧，不能补空方法。

| 顺序 | 真实业务与已有 core 入口 | 首先要补齐的 producer / 类型关系 |
|---|---|---|
| 地形上传与更新 | upload_world_raster；water/garden rasterize；terrain strokes、rock displacement、read terrain、mouse stroke、foliage mask、highlight、grass push、path masks、buildings | 六个真实 WorldRaster 与六个 mapped staging SSBO、脏标志、tick、预设纹理及队列。不能把 source-build WorldRaster cast 成原布局 RasterFlags |
| 植被和动态数据 | water/building/garden vegetation、bush CPU positions、path pebbles、leaf particles、roof gravity、stairs | 使用真正实例 SSBO 类型、数量、更新窗口及 GPU→CPU 回读；部分文件仍含零大小 ssbo_carrier，不能创建零字节缓冲冒充原数据 |
| GPU 剔除与主绘制 | generate_wall_draw_lists、gpu_generate_draw_lists、shadow、terrain lighting、main_pass、HZB、第二轮剔除、grass、main_pass_oc2 | GpuWorld subsets、真实 mesh atlas、间接命令/分桶计数、同 App VAO、实际相机矩阵。普通 kind3/4 已有真实 GPU 证据；kind5/7/8 已按原命令分支接现 API、真实能力位和边界校验；kind5实际像素已通过，M3未启用DrawIndirectCount，kind7/8仅验证明确拒绝 |
| 深度消费者与基础 GI | visibility、autofocus、mouse depth、late forward/deferred、half_res、reprojection、SSGI | 前段实际生成的深度/G-buffer/velocity、相机和真实读回缓冲。half_res 已按 fresh 原调用恢复同一深度 owner 的 stencil aspect=4 视图；生产宿主保留默认深度与模板视图的区别，构图回归待统一验证 |
| GI / 光源 / deferred | ray_trace_gi、eval_gi_screen_radiance、bin_lights、restir_di、apply_screen_shadows、deferred_raster | 软件 RT 三缓冲和真实 BVH、墙颜色、Lantern 实例、阴影历史。bind_resources 已统一到 resources::rt 的 canonical producer 类型，并按原读实际元素数量；真实 BVH 构建 producer 仍由原锁定 obvhs fork 恢复，不能以空 BVH 冒充场景数据 |
| 水 / 冰 / 透明 | refraction_copy、water/ice surface、pre_taa transparency、shadow mask mean、volumetrics | 真正反射/折射输入、surface plan 的每个动作、同一深度视图和 preset；不能只消费 plan 的名字 |
| DoF / TAA / TGSR | dof_prepare、generate_mips、calc_dof、calculate_taa/tgsr、post_taa transparency、shader_refactoring | 可复用现有多 pass 算法；参数必须来自真实相机、景深/temporal/config producer。原返回包含多纹理/缓冲，适配应保存真实 typed 输出而非向 opaque 数组塞伪句柄 |

后半段纯纹理算法较适合先做有界真实 GPU 验证，例如 reprojection→SSGI 或 TAA；但验证输入必须标注为 fixture，它们不能替代前半段真实世界 producer。

## 已发现并修正的当前具体边界

`ComputePass::try_new` 原先初始化一个默认 `RecordingPassBuilder.render_graph`，该图在生产没有 backend。真实 ColorLibrary 等第一次按需 SSBO 读取会直接 panic。现在 WorldAccess 提供当前 graph factory，FrameWorldAccess 使用同 CoreGraphFrame 的 facade，解析同 App HostTables 后经同 FrameDeclarer/session 导入；不是提前导入整个库。GraphPasses 改为短借 shared session，避免 with_compute 整段借用期间真实 lazy import 再借导致 RefCell panic。结束只导出库内实际已导入的条目，并清该周期缓存。

对应 GPU 回归是 `tests/core_render_bridge.rs::library_ssbo_lazy_import_inside_compute_uses_the_same_frame_and_reimports_next_frame`：一个使用、一个未使用 SSBO；with_compute 内按需导入，GPU copy 写入，下一帧更换 graph generation 后再次写入和读回。统一验证结果由根整合日志记录，本文不预填通过。

W1 六个暂存消费者原先是另一组零大小 `*StagingBuf`。原 PDB 的六个 `read_storage_buffer` 单态完整名称均为 `resources::world_raster_defs::*RasterStagingSsbo`（`0x140588650..0x140588f60`）。现直接 re-export 原 producer 的真实类型，保留 512×512 字节和映射所有者，不用同 NAME 的另一个类型。新增 `upload_live_world_raster<R:LiveRasterKind>` 收真正 Bevy Mut；pending 读取不标记 changed，格式校验后才依次标记 changed、更新 published/reupload，再调用共享原上传算法。fresh RNE `0x140a1767c`、`0x1403d8c7a`、`0x1403d8cd7`、`0x1403d8ce3` 支持此状态关系和时点，证据/合同在 `TG-d92ebaf1679099cd4d/render-raster-typed-20261018/`。生产 FramePasses 对应调用尚未安装，不计世界W1链已闭合。

`indirect_params.rs` 已修实际 producer 规格差异：四个原 `new_storage_buffer`（`0x141610820` / `0x141613db0` / `0x1416148e0` / `0x141614d30`）均明确 `BufferDesc::new(size,0x122,1).alignment(4)`。新专用创建器保留原描述及 Some 数据检查，不再走通用 new 的 uniform/storage 选择；通用 new 保留原其他实例行为。原 atomic 的 `%4` 和 `<12` 检查还纠正了先前的 8B×1 推断，实际为 4B×2。None 初始内容分别是 atomic 不上传、draw 全零16B、dispatch `[0,1,1,0]`×8、indexed 全零480B；不能将 reset 的 instanceCount/indexCount 初值提前到 startup。fresh 原 producer xrefs 均来自 `resources::init_populate_ssbo_library@0x140909e80`，四调用顺序 Dispatch→Indexed→Draw→Atomic（`0x14090a057..87`）；该完整启动函数的其余注册尚未恢复，不在宿主任意时点补四个 add_new 冒充整个 producer。

LUT 原闭包已恢复在 `resources/gpu_lut.rs`：BRDF always，非 demo 另注册 water；分别写 preset0/preset0x18、线程总数64×64/256×256，water 还读取原磁盘纹理 NameHash `6ec95f7a9d9f0faf`。`render_luts@0x14090e2f6` 同步执行 CPU 图声明回调后，`0x14090e2f9` 立即写 rendered=true；该状态发生在物理 compile/submit 之前，失败/取消图不会自创回滚。`CoreGraphFrame::with_main_prelude` 已按原 GpuWorld import→FrameConstants→LUT→indirect reset→下游 consumer→对称 export 顺序提供同图可调用闭包。生产 compile_world_graph 不仅跑此前缀就假成功或提前发布 gameplay ready；完整 scene consumer 仍缺。验证 fixture `original_main_prelude_gpu` 使用真实原 shader、真实 owned GpuWorld 子集、原四缓冲和原 resolve 读回 LUT，严格 GPU v15 已通过两帧真实 LUT/indirect/GpuWorld 及读回（`logs/strict-main-prelude-v15.log`）；不能将测试输入当作场景恢复。

## 版本界限

本次编译主图与真实 B packaged shader 的证据范围是 B。C 的 upload_world_raster 存在，但其 shaderface/rhapsody2 调用和 source checksum 已不同；上述 B 接线与 GPU 结果不自动构成 C 的业务验证。

## 历史有效标志的实际宿主错误

原 B named texture `0x1404f6e50` 在缓存命中且描述兼容时，`0x1404f702e` 写 flag=1；新建/尺寸变化时 `0x1404f72d6` 写 0。buffer 实例 `0x1404f62d0` 的 `0x1404f643d` / `0x1404f665a` 同极性；pair `0x1404f7450` 把两个 named 结果合成后返回。core SSGI、ReSTIR、TAA 消费者直接把该位传给 `history_valid`，因此宿主原先的 `!history_valid` 是反向错误，已修为直接传递。接缝的旧字段名 `TemporalTexturePair.fresh` 为兼容保留，但含义已明确为历史有效。

保留用户当前 rhapsody 的事务所有权与内容有效性合同：宿主取 session 已提交内容的 `history_valid`，pair 取实际上一帧成员；不将原缓存“分配已存在”位重造为平行 legacy 状态，也不把取消/未写入历史发布为有效。严格回归 `core_raster_draw::host_temporal_validity_tracks_submitted_history_resize_and_cancellation` 覆盖真实 GPU 初始化、跨帧、尺寸变化和取消；严格GPU v26已实际运行这两条宿主历史回归，2 passed / 0 failed，覆盖首次、跨帧、resize/cancel；见logs/strict-host-temporal-v26.log。原完整有界指令和合同见 `TG-06f4abbe570c5f0ee3/render-prelude-lut-20261018/scene-adapter/`。

## 可调用的 typed scene compute 批次

`tiny-glade/src/scene_compute.rs` 提供同 `CoreGraphFrame` / 当前 App shader library / 六个真实库的 callable consumer。W1 `upload_world_rasters` 按原 `[1,2,3,4,0x13,0x14]` 顺序借用六个实际 ECS `Mut` 和各自 mapped staging SSBO；不会把 native-layout `RasterFlags` cast 到当前资源。`SceneComputeHost` 真实调用并 record HZB、half-res、reprojection 两 pass、SSGI 三 pass、RTGI 两 pass、screen-radiance 三 pass、Lantern binning/ReSTIR、screen shadows/mean、DoF prepare/非 multires、TAA/TGSR 的 core 算法。RTGI 保留 `RestirTextures` 三纹理，DoF 保留 `DofPrepared` 两纹理，光源 bin 保留 `LightBinning` 两缓冲。

这些闭包尚未构成完整 `FramePasses`：前半段地形/植被/间接剔除/主绘制的 producer 和 consumer 仍缺；DoF multires 的 DynamicConstants tile-offset 上传也不能忽略。没有补 54 个空方法或统一错误代理，没有仅跑部分就让 production `compile_world_graph` 返回成功。`original_scene_compute_gpu` 分别验证实际 mapped tile 数据经六个原上传 shader→原 resolve→读回，以及首帧 reprojection 的无有效历史语义。v26两项实际进入GPU后均在原resolve必需的ColorLibrary查找失败，见logs/strict-scene-compute-v26-failed.log。fixture补原init_color_library经同BufferHost实际上传后，v37两项已真实GPU通过；六raster读回期望按发行resolve的线性→sRGB传递函数纠正，而非按未经resolve的输入值断言。

SSGI等剩余首帧历史绑定还需实际验证图合同；reprojection已经完成该闭包：原 packaged SSGI temporal SPIR-V 在 `history_valid!=0` 且 frame-index 非 0 的分支里才采样 history，reprojection 也有同类门控。原reprojection在v30真实触发UninitializedRead；现工程已增默认关闭的contents_unused意图，保留binding/同步/usage/barrier/lifetime而只跳过已证明不读内容的依赖，v37实测通过。其他消费者必须先保留原 shader 分支证据、实际触发当前图错误，再使用这一有界能力；不得给历史补假 clear、全局禁用未初始化检查或恢复平行 legacy backend。

## 每个世界节点的真实剩余接缝

下面逐个核对 `render_world` 的调用和当前源码，W 是原消费顺序，W1 含六次真实上传、W45 含季节分支。这里的“可调用”只表示已在同帧声明真实 core consumer，不能代替列出的数据生产者，也不表示完整 FramePasses 已安装。

| 节点 / 入口 | 当前可调用内容 | 还必须连接或恢复的实际输入与生命周期 |
|---|---|---|
| W1 upload_world_raster ×6 | scene_compute 按真实 Mut + canonical mapped SSBO 接原上传算法 | 六 raster 的编辑/回放产物、published/reupload tick 和同帧预设；六上传 GPU fixture 已在v37通过两帧；完整编辑producer仍需真实场景消费 |
| W2 water_rasterize_from_cpu | 原四 pass 已有；live Mut 候选已取 full RNE | 当前 WaterRaster 的 reupload flag 消费/清 dirty，必须在原时点写 changed；不能 cast RasterFlags；通道保留的原 RMW 依赖 |
| W3 rasterize_terrain_stroke | 原筛选、点/StrokeInfo 上传和下游 pass 算法 | 实际 ActiveStrokes、StrokeEditCounts、DynamicConstants BDA 分配与 GPU lifetime、NormalDebug；容量上限及原 half 打包不能替成默认数据 |
| W4 displace_rocky_terrain | core pass 已有 | 实际 RockyTerrain mesh/vertex owner、真实 instance head 和 RG 导入；现 RockyTerrainMeshHead(pub u64) 不能当裸 Vulkan 句柄 |
| W5 read_terrain_data | canonical 六 readback SSBO 类型集合已存在 | 原 VisualHeightRaster 真正绘制地形/岩石，ScheduledTerrainReadback 的请求、GPU 完成后映射消费和 world tick |
| W6 mouse_read_terrain_stroke_index | 原8B光标坐标 push + mapped SSBO consumer | canonical CursorTerrainRaycast、MouseTerrainRaycast mapped owner；原 dispatch 后何时 CPU 读映射与 UI 使用不能脱开 |
| W7 rasterize_foliage_exclusion_mask | core dirty/optional stairs pass | 实际 stairs foliage exclusion 顶点 stream 与 vertex_count，更新/删除触发；不能固定 None 跳过 |
| W8 highlight_active_terrain_stroke | 原 mask + 条件两遍 blur | 真实 AppMode、AddTerrainHighlightCmd 双缓冲事件、抑制资源字段；原 PDB EventWriter 证明 Event 身份，当前 derive 已同步 |
| W9 object_grass_push_pass | core raster consumer | 原 (&Transform,&ObjectGrassPush) ECS 查询转 Vec4，真实 draw target 和刷写时点 |
| W10 compute_path_masks | core 多 mask pass | 真实四 path/cobble Mut、PathStrokeQueues、逐通道读改写依赖、编辑/删除后的 dirty 状态 |
| W11 rasterize_buildings | core grounded-mask 算法 | 实际 grounded ECS 行和 MaskDesc、BuildingsRaster 数据上传、可变 SSBO 库借用与真实 GPU device，不能空 HashMap 替代 |
| W12 garden_rasterize_from_cpu | 原两遍 mask blur；live Mut full RNE 候选 | 与 W2 相同的实际 reupload 消费时点及通道保留，当前 GardenRaster source layout 适配 |
| W13 water_spawn_vegetation | core 真实分支与 compute passes | Season/demo 原资源、Lilypad/LilypadFlower/Reed 等仍缺真实元素/容量 producer；DrawIndexedIndirectBuffer 不得伪造 |
| W14 spawn_vegetation_around_buildings | core 算法，Bush/Flower 已有48/64B canonical 类型 | 真实原 gate 字节/建筑预设、实际 Bush/Flower 库注册和生成计数 |
| W15 read_bush_positions_for_cpu | core compute consumer | BushCpu mapped payload 的真实布局/容量和完成后 CPU 位置消费；现 Gap carrier 不能创建零字节缓冲 |
| W16 garden_spawn_vegetation | core 四 GPU producer | Bush/Flower 已有实际元素，Flower03/GardenStone 仍有 Gap，Flower03 debug NAME 未证；不能用同名空 carrier |
| W17 path_pebbles_compute | core compute consumer，48B PebblePlacement 类型 | 实际注册容量与 preset 来源、对应 draw/indirect consumer；输入 path masks 必须来自 W10 |
| W18 leaf_particles_compute | core 时间更新 pass | LeafParticle/Emitter 的真实数据 owner、计数资源、CPU emitter 上传及粒子绘制消费；当前 typed gaps |
| W19 roof_tiles_gravity_compute | core 行遍历/重力 compute | ECS RoofTileRow 收集（first/count/pivot/动画）、RoofGravityFrame、canonical sparse RoofTile owner；不能静态空 rows |
| W20 stairs | core counter/transform compute | StairsTransform 与 counter 真实字段/容量和 ECS 初始化、GPU结果消费；原 SSBO 注册不能用 ZST |
| W21 generate_wall_draw_lists | 现叶 consumer，尚非真实整组 CPU 收集 | 原 UnsafeWorldCell 壁行查询、Sparse hide/show 时点、Brick canonical visibility/draw-list 与实际 batches；现统一20B batch还需按原墙闭包复核 |
| W22 gpu_generate_draw_lists(None) | 新完整原11539B编排接 scene_compute；四平面间接 dispatch 真消费 | startup_nani 的实际 atlas/subsets/两 visibility；WoodenPlank已纠正原64B/4096chunk；原子/partial-write精确preserve后v37真实1实例必要18pass及读回通过，完整App atlas/数据producer仍需连接 |
| W23 shadow_pass | SceneRasterHost 真实三 cascade core 绘制 | 实际 DrawLists/VAO/indirect counts、灯光相机与正确 shadow depth targets；不能以三次空 draw 充数 |
| W24 calc_terrain_lighting_texture | core compute consumer | 原相机矩阵真实 producer 与 CameraMatrices typed view 身份、实际高度/法线预设 |
| W25 main_pass | SceneRasterHost 调原主绘制及 refactoring gate | 真实 GBuffer/depth/velocity/color 目标、DrawLists/mesh/材质和原资源 producer；前段 Nani 计数必须实际可见 |
| W26 generate_hzb | SceneComputeHost 调 core HZB | 实际 W25 depth、原 inner dimensions、完整 mip view/lifetime；不能拿输出物理尺寸替换元组 prefix |
| W27 generate_wall_draw_lists_pass2 | core reset + OC/2 叶 pass | 同 W21 实例查询/typed buffers，读取 W26 HZB 和原 freeze flag；复用首轮真实可见性状态 |
| W28 gpu_generate_draw_lists(Some HZB) | 与 W22 同真实 master，交换 visibility、只 main lane | W22 producer + W26 HZB；不能复用首轮4lane或把两个visibility都清零 |
| W29 grass_compute | core 多阶段分配/scan/间接计算算法 | GrassBladeDataSsbo 仍为 Gap；实际主题/模式 typed identity需核（旧 GladeThemeId TypeId 与 AppMode 重用），磁盘brush/heightmap、HZB和GPU绘制consumer |
| W30 main_pass_oc2 | SceneRasterHost 真原消费者 | W27/W28 真实生成的补绘 indirect 数据及相同 GBuffer/depth/color owner |
| W31 run_visibility_queries | core compute consumer | 实际 Firefly、Visibility canonical SSBO 和数量、GPU→CPU回读完成；不能随意给0数 |
| W32 calculate_dof_autofocus | core autofocus 算法 | AutofocusInputs 从真实相机/config/cursor产生、Autofocus/FocusReadback 原注册、完成后 CPU focus 回写 |
| W33 mouse_depth_compute | core compute consumer | 真窗口/cursor/focus 输入、MouseDepthValue mapped producer和 OS 消费/lease；不能只保存裸指针跨 App |
| W34 late_forward_and_deferred_pass | SceneRasterHost 真原消费者 | 前段真实 targets、actual draw lists 和 RT/material inputs，原 forward/deferred 分支 |
| W35 extract_half_res_depth_normals | SceneComputeHost 真原 pass；root已恢复 stencil aspect4 | 同一 depth owner 的默认视图与 stencil 视图，不能给 stencil 另造纹理；相关实际回归待根日志 |
| W36 calculate_reprojection_map | SceneComputeHost 原 map+copy 两 pass | W25 depth/velocity与事务化 PrevDepth历史；首帧flag0的原CFG已证，当前binding-only能力经v37真实GPU验证；不制造历史内容 |
| W37 calculate_ssgi | SceneComputeHost 原三 pass | half/GBuffer/reprojection真实输出和原历史有效位；history_valid=false 不应伪清历史或错误宣称读取过旧内容 |
| W38 ray_trace_gi | SceneComputeHost 原两 pass，保存三 RestirTextures | canonical RT buf/count 已修；真实 obvhs fork BVH/几何/albedo producer由root接，不能空BVH；TAA/TGSR prev history来源 |
| W39 eval_gi_screen_radiance | SceneComputeHost 原三 pass | 实际 W35/37/38 outputs、原 ShaderRefactoringState 分支和历史行为 |
| W40 bin_lights | SceneComputeHost 真原 LightBinning 两 buffer | canonical Lantern 真实元素/实例数量、实际灯光 ECS 写入与库安装 |
| W41 restir_sample_direct_lights | SceneComputeHost 真原 consumer | LightGrid两 buffer、Lantern实际count、上一帧 reservoir 和 reprojection；首帧条件内容意图仍需核 |
| W42 apply_screen_shadows | SceneComputeHost 真原 pass链 | W25/W36真实输入，ShadowHistory/噪声/实际光照和相机参数 |
| W43 deferred_raster | SceneRasterHost 实际 ComplexDeferredInputs | RT/GI/shadows/ReSTIR/binning的真实 typed输出、canonical Lantern/SpatialResolveOffsets；不能将多个buffer/texture塞伪32B LightBins |
| W44 refraction_copy_pass | SceneRasterHost 真原 copy draw | color/depth与新 refraction 目标真实 owner；原 flag_c098 决定是否执行 |
| W45 water_surface_pass / ice_surface_pass | core 已有 SurfaceEnv/SurfaceCompute 真算法 | 需要同帧 compute+raster+staging/clear adapter消费每个实际动作，季节和 history真实输入；不能只遍历 SurfacePlan 名字 |
| W46 pre_taa_transparency_pass | SceneRasterHost 真原绘制 | color/depth/half/prev-history、透明draw lists及真实history flag |
| W47 compute_shadow_mask_mean | SceneComputeHost 真原 consumer | 实际 ShadowMask预设与mean输出后续uniform/历史写入 |
| W48 apply_volumetrics | core pass算法 | 原 camera-mode 字节的真实 resource producer与同 color/depth；尚未独立SceneCompute wrapper |
| W49 dof_prepare | SceneComputeHost 真原两texture输出 | canonical DofAutofocus 真实更新与 depth，不允许将两个输出截成一个伪handle |
| W50 generate_mips | core SPD/mipviews/atomic算法 | 同一 color mip target、实际 counter清理与push、原 DOF_MIP_COUNT；尚未生产wrapper |
| W51 calc_dof | SceneComputeHost 已接非multires | multires路径原 DynamicConstants tile-offset bytes必须上传并被consumer使用；真实 mode/config开关，不能丢弃返回Vec |
| W52 calculate_taa / calculate_tgsr | SceneComputeHost 两原算法入口 | actual history pair/输出尺寸/相机配置与 subgroup capability；taa_param原字段语义仍需绑定，不能任意填0 |
| W53 post_taa_transparency_pass | SceneRasterHost 真原consumer | W52实际输出、后透明 DrawLists/VAO/材质，透明原shader pass状态 |
| W54 render_shader_refactoring | core try_render_shader_refactoring | 真实StateMut、tick/capture/cancel、frame handle与GPU完成消费；不能仅记pass名或忽略异步结果 |

生产 FramePasses 还要处理真实返回类型的转接。原 opaque PassHandle36B能表示单个native texture，但 RTGI为三纹理、DoF prepare为两纹理、LightBinning为两buffer；应由宿主保存这些真实typed对象及唯一图资源ID关系，不能按旧占位数组大小猜原ABI。原 core TextureDesc 的 sample/usage差异也须精确转换，特别是主图4×MSAA与后期。

## Nani 顶层本轮纠错

`gpu_generate_draw_lists@140987dd0` 已完整取原11539B、21下游实例与跳表，原合同和完整源码快照在 `TG-5f034e6af1260241e4/render-nani-main-20261018/CONTRACT.md`。它原来并未恢复顶层，部分间接闭包为空；现在 `IndirectWith` 正确提供原 ComputeDispatchIndirectApi，绑定同一声明的图buffer，并逐次消费原byte offset。Nani master 恢复19类通用/4类custom/4类external剔除、细/粗排序、四平面108-entry GPU dispatch参数、allocate/group/subtotal/generate/fill-indirection阶段。首轮4lane、HZB补绘1lane；filtered binding实际复用 NaniInstanceIndirectionSsbo；Flower/Bush的GPU计数槽为11/10，原参数是64线程而非4。

真实 OwnedGpuWorld cull 输入也已补：原 `TypedStorageBuffer<GpuInstance>@141616f30` 使用 count×16B、usage0x20022，当前CPU Vec缺GPU owner不能提供BDA。现在复用当前ShadowedBuffer/dirty范围上传、同设备真实Buffer地址，保留已有实例分配/删除算法。新增CPU回归与原始指纹均已保存，根v25库编译通过；Nani master在后续v37真实1实例GPU与必要pass/readback已通过；原startup的完整App资产producer/调度仍不能据此升级为验收完成。


## 启动数据和 GPU 验证的最新边界

Nani原startup/reload的完整取证和真实producer已落生产可调用函数 `startup/startup_nani.rs`，合同在 `TG-a31ead12d06d2bb8a2/render-nani-main-20261018/CONTRACT.md`。27原shader按注册顺序加载；实际registered atlas决定建/不建subset及实体；16B布局以kind*1024上传；原params决定Prev/Next两项各473600B初始上传。当前PlacementData从旧8B简化恢复为原48B，含原serde四required、可缺省字段、动画pack默认与Owned隐藏默认的区别。GpuDrivenDrawable复用已有三组件Bundle。源码的原输入和顺序已恢复，不代表Boot调度已消费。

原普通间接绘制条数也得到独立更正：Owned构造参数atlas.layouts.len写+a8，External相同参数写+8；它不是live_instance_count。GpuWorld已增加独立mesh_part_count，draw_ext合法owner接消费者。原fine路径仍使用真实countbuffer。原startup属于PreStartup外部四项chain最后一项：create_preset_textures→十项配置tuple→load_mesh_atlases→startup_nani；load_mesh_atlases内确先register_meshes。该原调度证据由boot owner在TG-c459.../nani-registration交付，尚不能据此推定本机App已执行全部前置。

严格真实GPU v26：宿主temporal两项通过；原resolve和typedVAO像素两项通过。VAO测试含实际kind5 multi_draw_indirect；M3设备DrawIndirectCount未enabled，kind7/8本次仅验证明确拒绝，不宣称GPU执行。日志分别为logs/strict-host-temporal-v26.log与logs/strict-raster-indirect-v26.log。scene两项失败及修正边界见上；没有修改像素期待或制造内容绕过失败。

## 连续世界lighting候选的审查范围

本轮候选及逐参数原调用映射见`TG-0ef1874c29a03d0dec/main-chain-frame-20261018/world-typed-candidate/WORLD-LIGHTING-CONTRACT.md`。原最终返回仍为depth/CoC/TAA三纹理：DoF内部两纹理必须保存到calc_dof，最终只返回CoC；RT GI内部三纹理、LightBinning两个buffer必须完整传到deferred。旧opaque数组不是可靠ABI，不再用它截断实际结果。该候选不含54个空方法，不新建rhapsody后端，不把局部段连接标成生产compile_world_graph完成。
