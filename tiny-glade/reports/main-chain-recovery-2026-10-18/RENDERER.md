# 原渲染入口与生产宿主接线

日期：2026-10-18。此文以当前源码和原程序函数/指令为依据，不使用历史进度统计。

## 原证据

- B patched 数据库：`tiny-glade.exe`，MCP 请求使用显式 `program`，只读 shared worker；原始返回在 `render-present-original.json` 和 `renderer-evidence/resolve-present-original.json`，两份请求均 `ok=true`，每步 `isError=false`。
- 发行 RNE 的 `EguiBackend::draw` 指令保存在 `renderer-evidence/egui-draw-rne.asm`，resolve 闭包指令保存在 `renderer-evidence/resolve-draw-rne.asm`；它们是原指令，不是 Rust 作者源码。
- 外层 `render` 0x1404e8010 的顺序是：主图编译/分配 → 执行主图 → acquire → 展示图 → end_frame → present。不能把主图编译和提交合并后再宣称原 OOM 恢复分支已接通。
- 展示图 compile 实例 0x140166510 在 0x140166b07 调 `resolve_to_swapchain`，随后以访问 0x27 导出交换链图像。该实例没有 Egui/UI/final_post；这些出现在主图实例。
- `Swapchain::present` 的正确 B 地址是 0x140c0ef80。0x140c0f170 是 `into_present_source`，此前把它当 present 是错误映射。present 包装检查设备身份、资源类型与代数，再调用 ManagedSwapchain::present，并累计 any_presented。

## Egui 的实际调用闭包

原 draw 0x1400ddc80 创建两条 CPU_TO_GPU 缓冲：u32 索引（usage 0x40）和 20-byte 顶点（usage 0x20）。0x1400de535 的纹理格式立即数为 0x2b，即 R8G8B8A8_SRGB。

0x1400def7e..0x1400defd0 将 `screen_extent / pixels_per_point` 的两个 f32 放进 DrawPacket 的字节 Vec。随后绑定 color_tex（hash 0xd9978e541ce9a95e）和 vertices（0x12ef39a9447f1f0e），修改原光栅状态位，调用 indexed draw，最后 RasterPass::render。callback primitive 是失败分支，未观察到逐 primitive 的 scissor 写入，不能自行加入并宣称原语义。

`src/egui_gpu.rs` 通过现有 TextureHost、BufferHost、CoreGraphFrame.with_raster、RasterPass 和 DrawBuilder 执行这条链，没有新造 shader。`raster_host.rs` 以前固定向参数 ABI 传空 push，现改为传 DrawPacket.unknown_bytes，使原 8-byte 数据真正进入 GPU。

新 GPU 验证在 `tests/original_egui_gpu.rs`：原发行 shader、真实 atlas 上传、顶点/索引上传、push 参数和最终 GPU 像素读回。矩形与背景仅是显式验证输入，不能作为游戏场景恢复。

## 普通网格与 VAO 消费

原执行回调在 PDB module 1655 / procedure record 24528，B RNE 范围 `0x141697ab0..0x14169a64e`。这不是只有 `RasterPass::render` 声明函数的证据：fresh MCP 的 `vao-render-original.json` 与真实 RNE 指令 `vao-render-rne.asm` 对应实际命令执行回调。Ghidra 未展开其间接跳转表，因此另从原 RNE 的 `0x142c87b60` 读取 9 个有符号偏移，保存在 `vao-draw-switch.json`。

kind 3 跳到 `0x141698cae`，确认 VAO head 后，`0x141698d23` 将 VAO entry 的 index_count 放到 VkCmdDrawIndexed 的 indexCount；`0x141698cc2/0x141698d2a` 将 packet payload 放到 instanceCount；firstIndex/vertexOffset/firstInstance 都为 0。kind 4 的 `0x141698bd0..0x141698c32` 按每个 pair 的 firstInstance/instanceCount 多次 indexed draw，仍使用该真实 VAO 索引数。

原 VAO 的每个属性名匹配 shader reflection 中的字符串键，匹配项以反射 location 调 `bind_vertex_buffer_by_binding_point`（`0x1416988f8`），随后绑定 UINT32 索引（`0x141698b50`）。当前 ShaderHost 的 Vulkan vertex input 格式、location 与 stride 已来自同一发行 compiled reflection；生产 RasterHost 现在实际解析 typed ECS `Handle<Vao>` 到 `AssetVAOLibrary::get_by_handle`，再查同 App 的真实 BufferHost，保留强 Arc 到图退役。它声明并绑定这些真正的 vertex/index buffer，不将 asset u64 当 Vulkan 句柄，也不固定属性格式或顶点数。

调用方每帧通过 `RasterShared::refresh_draw_vaos(world, lists)` 刷新真实清单；ProductionRenderer 已接，独立 loading GPU 测试也需调用。间接绘制其余种类仍有明确未恢复错误，不扩称所有 DrawKind 已实现。

## 编译与执行的真实分离

`ExecutionChain::compile_graph` 返回 PreparedGraph，完成计划和物理分配，不调用 render callback、不提交命令。`VulkanGraphSession::compile_frame` 同时保留命名历史导出、结果缓存事务与回执容量预留。`execute_prepared` 继续使用原共享录制和提交逻辑；旧 execute_frame 包装同一条实现。

CoreRenderBridge 的 CompiledCoreGraph 在未执行时 Drop 会取消分配、运行退役回调并回收声明租约。`tests/compile_before_submit.rs` 检查编译时无 callback/receipt，取消恰好退役，随后同一会话继续实际 GPU 写入和读回。

compute 宿主现在也保留原 `Ref::bind_view` 的 aspect/mip/layer 范围：每条原访问对应一次真实 ImageView，声明时与录制时按同一视图描述解释，不再把局部 mip 的访问扩成整张纹理。原 access code、同步组和顺序保留；writer 标志不被擅自解释成内容 discard。legacy ledger 与显式 typed RMW 的内容初始化策略由对应原证据另行核对，未用清空新图像绕过加载图的失败。

原附件的 LOAD 还须成为真实内容依赖，不能只在 Vulkan 录制时选择 loadOp。RasterHost 根据实际颜色 tag0、可写深度 DepthClear::Load，且图中确有先前 writer 或非 undefined import 时，显式保留此前内容；CLEAR/DONT_CARE 保持覆盖语义。原版首次向新 target 做 fullscreen combine 时也可能指定物理 LOAD，此时没有可依赖的既有内容，宿主保留原行为，不新增假 clear。深度 CLEAR / stencil LOAD 的分 aspect 接线尚须单独审查。

## 生产 WSI 适配及界限

`ProductionRenderer` 持有实际 device 与严格 swapchain。loading / gameplay 按 `ApplicationShaderLibrary` 的真实 Arc 身份选择独立 CoreRenderBridge、ShaderHost、纹理/缓冲表、EguiGpu 和 DynamicConstants；两 App 只共享设备/窗口。Mac CAMetalLayer 是平台宿主适配，窗口/layer 生命周期覆盖 surface。展示使用当前 App 的同一 ShaderHost，直接调用 core 原 `resolve_to_swapchain`，保留三个顶点的 raster draw。

resolve 原闭包 0x1409d00a0 还在 0x1409d04f6/0x1409d0506 写 optional framebuffer_dims（hash 0x531bb9318eea4971），再绑源纹理和 transform_image（0x7b01c6bb382d8849）。生产适配保留这些参数；此前把该键称为 render_upside_down 是命名误述。

新鲜原证据在 `TG-ff3ad6b4ea06f53027/render-borrowed-20261018/present-import-original.json`，`ok=true`、所有步骤 `isError=false`。原 compile-present 在 0x140166aa6 以 access 0x22 `import_owned` 主图输出，在 0x140166aca 以 access 0 `import_unowned` acquired image；0x140166b07 调原 resolve，0x140166b4c 以 0x27 导出 target。原 import_unowned 0x140bd60a0 增加 managed owner 引用，import_owned 0x140bd6220 转移所有者，不能混作同一种声明。

`production_resolve::build` 现在实际建立同一 session 的第二张图并保留上述 owned/unowned/access/export。Texture 的 storage 区分真实 allocation 与真实 swapchain generation owner，后者不伪造 MemoryBlock、不调用 vkDestroyImage。`GraphImageLease` 在 backend 的一次真实 graphics submit 中加入 acquire binary wait、rendered signal 和 fence；图仍承担命令、资源状态和退役，后续 `present` 阶段只消费已提交的实际 image。tracked source 的真实 timeline dependencies 和 acquire wait 均先于所有命令，避免 FragmentShader 提前读主图。PresentTicket 与 swapchain 继续证明两个完成域；部分提交失败不会再次等待已经消费的 acquire。

原主图输出在转给展示图前从 backend 的 export owner 中取出，命令独立持有 Arc，避免每帧永久保留旧 export token。展示借用的 image wrapper 保留 swapchain generation，图 retire 后导入记录才释放，窗口/layer 最后销毁。此处是经过明确分型的 source-build WSI 宿主适配，未复造原 managed-handle ABI。

展示和 loading 共同通过 `FrameSetup::with_frame_resources` 导入真实 preset、原 descriptor table 请求、实际 0x1090 FrameConstants chunk，并在结束时导出 preset。ShaderBindings 的参数创建从当前 App ShaderHost 的同一 FrameGlobals owner 绑定原 set1/b0 与 set2/b0；required 参数无 producer 时仍由 ABI 检查失败，不向未知 set 写零。原 `0x14067dd80` 具名 Halftimber handle 实际仅 `mov rax,rcx; ret`，是身份函数的 ICF 别名；此处 TypeId 指向 DynamicConstantsResource，未额外创造 Halftimber 依赖。

主世界 FramePasses 仍需实际消费者闭包，生产入口和后台 warm-up 均保留原 compile_main 0x140161cd0 的明确错误，不能因此向前台发布 ready gameplay App。截图协调器仍缺原请求到 GPU 纹理关联，存在请求时不能把状态直接改成已完成。设备丢失被报告，未伪造重建设备成功。本文不把恢复 resolve/present 单条消费者扩称为整游戏主图已恢复。

## 验证记录

新增源码的统一编译和严格 GPU 测试由集成代理执行，最终命令/结果以本目录最终实施记录及日志为准。本文件尚不声称整游戏 GPU 帧或原整链 oracle 通过。

严格 host GPU v5 日志 `/private/tmp/tg-mainchain-strict-gpu-host-v5.log` 已实际通过 compile_before_submit 一项与 core_render_bridge 三项，包括两帧按需 SSBO 导入、GPU 写入和读回。随后原 Egui 测试在 ShaderHost 的旧“非零 push 一律拒绝”guard 失败，尚未进入绘制；现仅移除此旧 guard，仍经 `DrawPacket.unknown_bytes` → `ShaderBindings::parameters` → `BindlessParameters::new` 精确验证原 push 字节长度。该修正后的原 Egui GPU 结果待下一统一执行，不把源码修改当通过。

独立执行 v5 的真实普通 VAO/vertex/index/instanceCount 像素测试已通过。原 resolve 测试暴露了验证输入的错误：它曾要求 4×4 源填满 16×8 目标。fresh 原 packaged SPIR-V 明确是整数 FragCoord → 可选 `target.height-1-y` → OpImageFetch，完全没有缩放；旧断言坐标越过源尺寸。经原证据复核后，fixture 改成合法可见范围内的 32×16 源 / 16×8 目标，并同时检验 flag0/flag1；没有改渲染行为来迎合旧测试。原 stage、SHA、反汇编与说明保存在 `TG-ff3ad6b4ea06f53027/render-borrowed-20261018/resolve-load-semantics/`，修正后执行待统一验证。

主图原调用点、W1 真实 producer 类型身份、lazy SSBO 图/session 边界和逐批消费者缺口，见 [MAIN-GRAPH-CONSUMERS.md](MAIN-GRAPH-CONSUMERS.md)。该表直接核对指令和当前源码，不把 FramePasses 的录制测试当作生产世界图。


2026-10-18 后续真实 WSI 证据：独立核读 `/private/tmp/tg-original-loading-wsi-v12.log` 与 `tests/original_loading_present.rs`。harness=false 测试在 AppKit 主线程建立实际窗口、ProductionRenderer/device/surface/swapchain，以 loading-only World fixture 调同生产外层 `render(RenderMode::LoadingScreen)`；实际三帧通过 original loading→original resolve→acquired image lease submit→swapchain present，any_present_successful 检查和 shutdown 均成功。该 fixture 明确不运行后台 gameplay loader，不绕过真实 load_game 后声称游戏恢复。完整游戏启动此前暴露真实 history 缺口，完整 world consumer 仍明确错误；WSI 单链已执行与整个游戏可玩是两个不同证据范围。

main prefix 的新增源代码/未闭合边界见 MAIN-GRAPH-CONSUMERS：GpuWorld 原导入、原 LUT tracker 回调、真实四个 0x122 indirect producer 和 reset consumer 可在同一帧调用，完整 init_populate 与实际场景入口仍须接线。新的 LUT GPU fixture 不新增设备/缓冲的假句柄，也不向原 preset 附加非法读回用途；通过原 resolve 采样后复制宿主明确创建的合法测试 target。
