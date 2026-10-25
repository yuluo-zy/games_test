# B 阶段 s_h：S 层（render_loop 21 函数）+ H 层（tiny_glade::render_loop 3 函数）

范围：`reconstruction/country-core/src/systems/render_loop/` 下 `mod.rs`（仅登记模块，保留 a4_compute 的 RenderContext/clear 辅助）、`painterly.rs`、`vermeer.rs`、`lut_gallery.rs`、`menu_background.rs`、`session_selection/mod.rs`，以及新建 `render_world.rs`、`render_frame.rs`（H 层）。
标注 O=机器码观测，I=推断，U=未知。所有负责文件已无 `todo!`/`unimplemented!`。未改任何共享文件（lib.rs / rhapsody_types.rs / rhapsody_contract）。

## 验证
- H：隔离测试台 `/tmp/h_s_h`，43 通过（render_world 18、render_frame 25）。
- S-A：`/tmp/h_s_h_a`，painterly/vermeer 36 个自有测试通过（连带 shader_uri 5 个共 41）。
- S-B：`/tmp/h_s_h_b`，47 通过。
- 全部文件 `rustfmt --check --edition 2024 --config max_width=120` 通过。
- 未做：把 7 个文件放进同一个测试台联合编译（各自独立通过）。

## 总体设计
所有业务逻辑写成对文件内 Host/seam trait 泛型的函数，trait 方法 = 机器码里的外部调用点（按调用序、带地址注释）。常量、哈希、字符串、日志/panic 文案、分支与纯算法（环缓冲、OOM 降级、缩略图池分配等）真实实现并有测试。接线到 A4 的 `RasterPass/DrawBuilder<'p>/PassHost` 与各 pass 入口属集成层。

## H 层：tiny_glade::render_loop（render_world / render / handle_screenshot_error）

PDB 名 `tiny_glade::render_loop::*` 属 tiny_glade 二进制 crate（`crates/tiny-glade/src/render_loop.rs`），不在 country-core；放在 `systems/render_loop/render_world.rs`、`render_frame.rs`，`mod.rs` 登记 `pub mod render_frame; pub mod render_world;`。

| 地址 | 原符号 | Rust |
|---|---|---|
| 0x1404ec110 (4314 B) | render_world | `render_world::render_world<P: FramePasses>`（54 个 pass 逐条按机器码调用序） |
| 0x1404e8010 (16279 B) | render | `render_frame::render<H: RenderHost>`、`recompile_at_reduced_scale`、`FramePacer/Ring4`、`reduced_scale/oom_message/out_of_video_memory/duration_from_secs_f64` |
| 0x1404ebfb0 (337 B) | handle_screenshot_error | `render_frame::handle_screenshot_error`（返回要记的 `(Error, "Error while capturing a screenshot: {err}")`；render 内并不调用它，O） |

### render_world（O）
- 纠正 `contracts/frame-wiring.md`：`create_raw_resource` 名是 "gbuffer"(`new_2d(dims,0x65)`)、"velocity"(0x61)；HZB 的 "main" 只是 `generate_hzb` 的 name 参数；DoF 设置字节 `+0xa9+(dof_mode==0x13)` **非零才**执行 `generate_mips("dof_mips",8)`+`calc_dof`（末参 bool=`dof_mode==0x13`）；`gpu_generate_draw_lists` 第三参是 `Option<&Hzb>`（第一轮 0）。
- TAA/TGSR：历史对 `taa_history`（尺寸取 `param_8.desc` extent）/`tgsr_history`（尺寸取分辨率资源 `+8` u64），均 `new_2d(..,0x7a).sampled().storage()`；输出 "taa_output"（0x7a，无用法位）；`calculate_tgsr` 比 taa 多一个深度参数。折射源 "refraction source" `new_2d(extent(color),0x7a)`；`is_winter`（0x140ae8320，ICF 别名）选 ice/water。
- `TextureDesc` 32 B 布局完全确定（usage@0、format@8、宽高@0xc/0x10、`[0x14]=[0x18]=1`、尾 `01 01 02`），已实现为真实结构。
- 参数：param_4=DrawLists(I)、param_5=`world_render_dims`（断言两维>0，文案原样）、param_6=设置字节（+0x9c/+0xa9/+0xaa/+0xab）、param_7=深度 RgTexture(I)、param_8=场景颜色 RgTexture(I)。返回体 0x6c B={深度拷贝, dof_prepare 输出, taa 输出}。
- panic 行号：965 断言、967 世界配置、978 prev_depth、1113/1302 TGSR 尺寸、1143 历史对、1254 DoF 资源。
- U：`*0x14362c0d8`/`*0x14362c098` 两个全局 bool 语义；36 B `PassHandle` 内部字段；半分辨率结果两张纹理的角色名；历史对中 cur/prev 传给 taa/tgsr 的具体次序（I）。

### render（O）
- puffin 作用域 → 设备资源 unwrap(行642) → `GameConfig` expect(行650) → `DrawLists::new/build/sort` → swapchain (format, extent) → 模式字节：0=加载画面图（缺 swapchain `expect("The loading screen must have a swapchain")`；编译失败 `expect("Loading screen render graph")`），否则世界图。
- 世界图编译失败 = 一次性 OOM 降级：`scale += -0.1f32`，Error 日志 "Out of memory while allocating render graph resources; reducing resolution scale {old} -> {new}"；`0.0>=new` → panic "Out of video memory. Swapchain info: {Option<(format,extent)>:?}"；否则写回缩放 → `ResourcePool::clear` → 两张 temporal 表换新 → `wait_idle_and_recycle_resources` → 降级状态资源首 u64:=2 → Info "Compiling a render graph for the reduced resolution scale" → `[32,32]` 兜底编译(失败 expect "Minimum-resolution fallback render graph compile failed") → Info "Reduced-resolution scale render graph compiled"。降级路径不取图像、不展示、不 present。
- 执行：`begin_frame`(模式 2 且 puffin 开→取 GPU profiler 缓冲) → profiler 结果 → `execute_compiled/retire` → (主编译成功且有 swapchain) acquire，状态字节 7=跳过 → 展示图("Failed to compile presentation render graph: " unwrap) → `end_frame` → present(swapchain 缺失 unwrap) → `finish_screenshot_capture` → `RenderStats::snapshot_and_reset`(写全局 0x143634ba8) → `refresh_memory_budget`。
- 帧节奏：GPU 环 `gpu_ns/1e9`、CPU 环 `间隔+执行耗时`，各 4 项(全局 0x143634bb8/be0，计数 bd8/c00)；`wait=max(min(GPU)-max(CPU),0)` → `Duration::from_secs_f64`(失败文案 "cannot convert float seconds to Duration: value is either too big or NaN") → `wait_gpu_queue_trim`；末尾写回上一帧结束时刻(0x143634bb0/0x14362b0e8，纳秒 `1e9`=None)。
- `Plugin::finish<MainCameraPlugin>`(0x14003bb80) 是 ICF 合并的 1 字节 `ret`，不建模。
- U/I：模式字节 2 的确切含义(I：世界+GPU 剖析)；视频设置里"节奏开关"(+0x87a 栈拷贝)的字段名；`GameConfig` 克隆的各字段。

### 缝与缺口
- `FramePasses`（每 pass 一个方法，参数=机器码调用点）、`RenderHost`；类型 `TextureDesc/RgTexture/TemporalTexture/TemporalPair/HalfResTargets/GBufferTargets/HzbHandles/LightBins/PassHandle`。
- **与现有 pass 入口签名对不上，未写适配层**（会随 D/C 重写失效）：原版入口形如 `(rg, world, <36B 句柄…>) -> 句柄`，当前源码多为 `(world:&mut Gap, _x:&Gap)` 占位或正改成 `DrawPassCtx/PassCx`。例：`calculate_reprojection_map/ssgi/eval_gi_screen_radiance(world,&FrameConstants)`、`ray_trace_gi(world,&history,&bvh)`、`restir/bin_lights(world,light_count)`、`dof_prepare(world,&DofAutofocusSsbo)`、`calc_dof(multires,&half,&cfg)`、`generate_hzb(name,depth,mips)`、`extract_half_res_depth_normals(world,recorder)`、`gpu_generate_draw_lists(world,round:u32)`（原版第三参 `Option<&Hzb>`）、`water/ice_surface_pass(world,view,&RtBvh,lists)`（原版 10 参）均不返回/接收 render_world 传递的句柄。
- 共享文件：无改动。

## S 层 A 组：painterly / vermeer

### 函数 → Rust 项
| 地址 | 原函数 | Rust |
|---|---|---|
| 0x1408a63d0 | PainterlyState::has_target | `PainterlyState::has_target`（O `byte[+8]&1` = Ready） |
| 0x1408a63e0 | PainterlyPlugin::build | `PainterlyPlugin::build<A: PainterlyApp>`（insert state(=2) + PreStartup） |
| 0x1408a7fe0 | startup_painterly | `startup_painterly<L: ShaderLoader>`（2 个 shader，路径/名字长度对 `r8d`） |
| 0x1408a6410 | painterly_pass 7118B | `painterly_pass` + `acquire_and_convolve` |
| 0x1408a8060/8140/81e0/8260/8270 | VermeerState::activate/deactivate/into_disable_hold/is_active/should_freeze_camera | 同名方法（into_disable_hold 按值，Disabled → panic "must be activated first"） |
| 0x1408a8280 | VermeerPlugin::build | `VermeerPlugin::build<A: VermeerApp>` |
| 0x1408a82f0 | vermeer_pass 2679B | `vermeer_pass`（返回 bool = 进入时阶段 != Disabled） |
| 0x1408a8d70 | vermeer_render_from_target 6569B | `vermeer_render_from_target` |
| 0x1408aa720 | startup_vermeer | `startup_vermeer` |
| 0x1408aab70 | GenerateStrokes::as_compute_pipeline | `GenerateStrokes::pipeline_desc` + `impl AsComputeShaderSource` |

### 关键 O 结论（纠正旧版）
- PainterlyState/VermeerState 是 enum2$：判别 `+8` / `+0x48`，0=Pending 1=Ready 2=Disabled（Default/插件 enabled=false → 2）。旧版字节占位数组全部换成真实字段。
- VermeerState+8 是 `Option<托管纹理>`；`Plugin::finish<MainCameraPlugin>` 只是 ICF 合并的 1 字节 `ret`，实际动作是 ManagedHandle::drop。旧版"释放主相机"的描述是误读。
- activate 参数顺序 `dl→hold(+0x54)`, `r8b→freeze_camera(+0x55)`；build 里 freeze_camera=1，hold=插件字节1。
- painterly_pass：instant = 资源(0xff)字节!=0 ? true : 资源(0x80).u32==0；instant 时 progress 步长=1024、timer uniform=100000.0；目标 extent = [(w<<8)/h, 256]（h==0 → div by zero panic）；采集条件 = 阶段==Pending || !(target&depth 均复用)；generate 的 push constants = [start u32, count u32, timer f32]，线程数 = count；raster 顶点 = 6×floor(progress')；timer += delta_f32。
- vermeer_pass 的 acquire pass **每帧无条件执行**（无复用跳过）；dt=min(delta,1/30)；进度 `(frame<2 || hold)` 不推进，否则 += dt/3；冻结取控制资源 f32；完成 = progress>=1 && !control.freeze && !hold → 释放纹理、phase=2、发 WriteSettingsFileCmd、Info 日志 "Game reveal complete."。
- vermeer combine pass 的名字在原版里就是 "Vermeer debug target"；笔数 photomode?0xa00:0x8000（旧文档的"toggle"= photomode 参数）；strokes SSBO = count<<4 字节。
- GenerateStrokes 是 `{photomode: u8}`，hash=calculate_hash(&(URI(0x29), &self))；pipeline 宏 `PERM_photomode`=u8 Display。

### O / I / U
- I：PainterlyState/VermeerState 在 Pending 阶段的 extent 取 0（原版为过期/未初始化）；`GenerateStrokes::as_compute_pipeline` 的 `ShaderDefValue(photomode)`（现有 trait 只携带 u64，完整定义在 `pipeline_desc`）；OnExit 变体 1=AppLoadingState 某值；vermeer 静态 0x143634d90（背景名切换）、0x143634c9c（prev_active）的语义命名；minsd/minss 的 NaN 行为未复刻（Rust min/比较写法）。
- U：资源 0xff / 0x80 两个资源类型名；aux_b/aux_c（painterly_pass 第 3、5 参数）与各绑定名哈希对应的 shader 变量名；log 记录里的行号；Time 资源名（+0x40 f64、+0x58 f32）；change-tick 回写（`world+0x5b0 → res+0x5c`）未复刻。

### 缝（stub）清单（都在 painterly.rs，vermeer 复用）
`PassHost`（app_mode / time / raster_shader / texture_extent / temporal_texture / temporal_buffer / create_raw_texture / texture_desc / log / submit_compute / submit_raster）、`VermeerHost: PassHost`（debug_control / alt_background / library_texture / blur_pyramid / send_write_settings_file）、`PainterlyApp`、`VermeerApp`、`ShaderLoader`；描述符 `ComputePassDesc/RasterPassDesc/ComputeOp/RasterOp/TexRef/BufRef/…`。集成层 host 把描述符翻译成 `ComputePass::new` / `RasterPass::new` / `DrawBuilder` 调用序（参数、哈希、访问码已在描述符里）。未改任何共享文件。

### 对公共 API 的改动
旧 `PainterlyState`(repr C 占位) / `VermeerState` / `VermeerDebugControl(Gap)` / `GenerateStrokes(Gap)` / `painterly_pass(&mut Gap)` / `vermeer_pass(&mut Gap)` / `raster_strokes` / `raster_combine` / `max_stroke_count(bool)` 等全部重写；`max_stroke_count` 保留；删除无 PDB 对应的 `raster_strokes`/`raster_combine`/`painterly_dispatch_closure`（原为 inline 闭包，逻辑并入 pass 函数）；`VermeerRenderFlags` 删除（机器码参数为 8 项见 `VermeerRenderParams`）。仓内无外部引用（grep 确认）。状态类型派生 `bevy_ecs::resource::Resource`。

### 测试
隔离测试台 /tmp/h_s_h_a（#[path] 挂载 painterly.rs、vermeer.rs、render/utils.rs、pass_builder/shader_uri.rs）：41 通过（含 shader_uri 的 5 个）/ 0 失败 / 0 警告；本范围 36 个：状态机、插件初值、阶段 2 panic、重置/restart、热身帧/hold/冻结/dt 夹紧、完成链（事件+日志顺序）、instant 模式、progress 封顶、extent 公式与 div0、temporal Err/shader 缺失 unwrap panic、每个 pass 的名字/URI/绑定哈希/访问码/uniform 值/顶点数/dispatch。`rustfmt --check --edition 2024 --config max_width=120` 通过（项目默认宽度下会改版面，沿用其它 B 报告的 120）。

### 遗留
- 两个文件含大量测试，分别 899/960 行（业务代码约 450/330 行），超出 800 行指引；可把 tests 拆到 `painterly/tests.rs`。
- painterly_debug_target shader 只在 startup 加载，pass 中无调用点（O：debug 用途 U）。

## S 层 B 组：lut_gallery / menu_background / session_selection（fork B）

文件（均在 `reconstruction/country-core/src/systems/render_loop/`）：`lut_gallery.rs`、`menu_background.rs`、`session_selection/mod.rs`。三个文件已无 `todo!`/`unimplemented!`，`rustfmt --check --edition 2024 --config max_width=120` 通过。

### 地址 -> Rust 项
| 地址 | 原符号 | Rust |
|---|---|---|
| 0x1408b04c0 | lut_gallery::impl$0::build | `LutGalleryPlugin::build`（O：0x48 B `Arc<Mutex<0x34 B>>` 全 0；资源 = 两个寄存器字 `{Arc, None}`；`startup` 加载 `/lut_gallery/thumbnail.raster.hlsl`=`lut_gallery_thumbnail`，无独立 PDB 过程，见 `STARTUP_SHADERS`） |
| 0x1408b0560 | render_photomode_lut_gallery | `render_photomode_lut_gallery<H: LutGalleryHost>` + `cell_uniforms` |
| 0x1408c99c0 | menu_background::impl$0::build | `MenuBackgroundPlugin::build`（0x50 B `Arc<Mutex<Option<tex>>>`，资源 0x40 B `{shared, current}`，niche=7 即 None） |
| 0x1408c9a30 | render_menu_bg | `render_menu_bg<H: MenuBackgroundHost>` + `menu_bg_mode_allows` + `take_pending_into_current` |
| 0x1408f68c0 | session_selection::impl$0::build | `SessionThumbnailsPlugin::build` |
| 0x1408f6900 / 0x1408a3bc0 | thumbnails_startup / closure$0 | `thumbnails_startup<H: ThumbnailStartupHost>`（4 着色器 + 6 张 512x512 格式 0x2b mip1 纹理，失败仅记日志） |
| 0x1408f6c00 | render_session_selection_previews | `render_session_selection_previews<H: SessionPreviewHost>`；内联宿主：`find_backed_thumbnail_range`、`extract_thumbnails_to_display`、`assign_pool_indices`、`pool_index_array`、`refresh_screenshot` |

### O（机器码观测）
- menu_bg：AppMode 门控真值表（state==2；0xa 且 flag!=1；k=state-2 (>=9 取 4)，位掩码 0xaf / 0x50 / flag==0）；锁共享槽后 `take`、写 changed tick、旧句柄 drop、`current` 为空则不画；pass 名 "Menu bg"、着色器 `blit_stretch_decode_srgb`、源纹理绑定 `0x11ba5697cc40b3dd` 访问 0x1d、raster state byte0&=0x7f、`draw_no_vao(3)`。`Plugin::finish<MainCameraPlugin>` 是 1 字节 `ret` 的 ICF 别名，不会释放相机（旧注释有误）。
- lut_gallery：demo 开关静态字节 0x143634d90；AppMode==(0xa,1)；预览槽 Arc 缺失/槽内 None 直接返回；pass "LUT thumbnail"；逐格 `zip(lut 列表, positions)`；绑定 3 张纹理（预览 `0x11ba…`、磁盘 `0xA647E80AF4E9395F`→`-0x3f4f…`、`NameHash(name)`→`0x0422b14c27be4803`）；uniform 序：extent、alpha=1.0、pos、size、highlight(=flag&&selected==i)、is_current(名字相等)、is_first(i==0)；state byte0&=0x7f、byte3=(b3&0x9f)|0x40；每格 `draw_no_vao(6)`；最后 `render`。
- session：门控 `(state-2)<9 && !=4` 时隐藏；窗口 `start=floor(min(len,max(0,scroll))-pool/2+0.5)`、`end=min(len,start+pool)`，断言文案 `backed_range.len() <= THUMBNAIL_POOL_SIZE.min(save_list.sessions.len())`；锁内 `mode_tag==1`（assert_eq）、`active!=0`（否则 `panic!("TODO")`）；pass D 按截图路径逐组件匹配复用池项，pass E 取第一个未分配项（`expect("thumb pool entry")`），新分配项 `modified` 复位 FILETIME 1970；`idx < self.texture_pool.len()` 断言；刷新：metadata("Reading metadata") → modified("Checking modification time") → 比较（有符号，`>=` 跳过）→ 解码("Loading the image") → `update_texture`("updating texture")，成功写回 modified 并 debug "Loaded session thumbnail image"，失败不更新 modified 且以 `"module_path! ()$crate :: Level :: Error" + 消息` 为键去重记 error（消息 `Failed to refresh screenshot {tail}: {err}`）；绘制序：背景(3 顶点) → "新建"卡(槽 0，6 顶点) → 全部会话卡 thumbnail 着色器(6 顶点) → 全部会话卡 painterly 着色器(0x300 顶点，多绑一张磁盘纹理 `-0x857a171164d2fc4`→`0x7337daaff5b5b2d5`)；位置 `(slot*spacing+origin.x, origin.y)`；高亮 `state&1 && slot==selected`；bool uniform `(index>=3 && demo) || session.flag`；窗口外会话用占位磁盘纹理 `-0x7f93d644083afcf3`。
- 全部 uniform/绑定哈希、着色器名与 URI、字符串长度（0x29/0x28/0x32/0x32 等）见各文件常量。

### I（推断）
- 各资源的角色名（hash `0xaca0…`=LutGalleryState、`0x4a14…`=当前 LUT 条目表、`0xd387…`=LUT 列表、`0x4da5…`=SavedSessionList、第 3 个资源=ThumbnailTextures）；`floor` 为 CRT 导入桩（0x142923fa0）的猜测；`SessionRow.flag`=painterly 开关；背景 Vec2 取 origin；窗口外会话取 `None` 纹理。
- `LutName` 的 Display（真实为 ICF 别名 0x140109e90，格式未核对）暂用定宽十六进制。

### U（未知，不编造）
- `DecodeOutcome::Skip`（`Reader::decode` 返回特定 ImageError 变体 10 且载荷 0 时静默跳过）的业务含义；`CarouselState` 的 +0x14/+0x30/+0x38 字段；`LutGalleryInner` 的 +0x08/+0x18/+0x2c；lut 条目 `[0]` 名字格式化后仅丢弃的用途；`PooledThumbnail.modified` 在启动闭包里未初始化（置 0）；纹理 0x38 B 内部布局（`ManagedTexture(u64)`）；`positions` 闭包 `gui::positions::closure_env_0`（属 gui 模块，经 `cell_positions` 缝）。

### 缝（Host trait）清单
`MenuBackgroundHost`、`LutGalleryHost`、`ThumbnailStartupHost`、`SessionPreviewHost`：world 资源取值（缺失=unwrap panic）、`get_raster_handle_by_name`、`NameHash::compute`、fs/image/`update_texture`、`utils::file::tail`、全局去重 `BTreeSet<String>`、日志、以及 RasterPass/DrawBuilder 调用（begin_pass、begin_draw、uniform/绑定、edit_raster_state、draw_no_vao、render）。集成者需把这些缝接到 A4 的 `RasterPass`/`DrawBuilder<'p>`/`PassHost`（调用序已按机器码固定）。

### 对公共 API 的改动
- 三个文件原先 `world: &mut Gap` 的系统签名全部改为泛型 Host；`MenuBackgroundResources` 由 `repr(C)` 位模拟改为 `{shared: Arc<Mutex<Option<ManagedTexture>>>, current: Option<ManagedTexture>}`；`LutGalleryState` 在 `resources/grading_lut.rs`（非本组所有）仍是旧占位，本组在 `lut_gallery.rs` 内另定义了带真实类型的 `LutGalleryState`/`LutGalleryInner`（建议集成时删旧并 re-export）；`*Plugin::build` 改为返回要插入的资源；`SessionThumbnailsPlugin`/`LutGalleryPlugin`/`MenuBackgroundPlugin` 改为单元结构体。无外部调用者（已 grep）。共享文件：未改。

### 测试
隔离测试台 `/tmp/h_s_h_b`（`#[path]` 挂载三个真实文件，仅用 parking_lot 垫片）：**47 passed, 0 failed, 0 warnings**。menu_bg 9（门控真值表对照独立参考模型、换入/重绘/旧句柄释放/顺序）；lut_gallery 9（demo/模式/预览缺失守卫、uniform 序与常量、zip 取短、空画廊、高亮位）；session 29（门控表、窗口夹取/NaN、池匹配/分配/复位、数组与越界断言、高亮与位置公式、flag 表、错误去重/重试/三种 context、Skip、窗口外占位纹理、"TODO" 与 assert_eq panic、启动着色器表与长度）。

### 遗留
- 与 A4 的真实 `DrawBuilder`/`PassHost` 接线；`SavedSession`→`SessionRow` 适配（截图路径 = `SessionId::save_folder_path(..).join("screenshot.jpg")`，flag=+0x18）。
- `session_selection/mod.rs` 约 850 行（含 ~300 行测试），略超 800 行指引。
