# G3① compile_main 尾部契约（grading / blur / final_post / UI）

- 轮次：2026-10-19 第 10 轮 G3①（逆向取证；只读源码，只写证据）
- 基线：`D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`
  （SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）
- 标签：**O** = 机器码/反编译/真实源码直接可见；**I** = 有证据的推断；**U** = 未闭合
- 不修改源码；不涉及 rhapsody 本体、roof/wall 用户区
- 主要证据：
  - `evidence/render-architecture/0x140161cd0.c`（compile_main 世界图实例，1916 行）
  - `evidence/render-architecture/0x1404e8010.c`（render 外框）
  - `evidence/render-passes/cpu-frame/sequence.md`（R1–R20、W1–W54）
  - `evidence/render-recovery/producer-closure/c1-country-core/frame-composition.json`
  - `evidence/country-core-audit/data/procs.tsv`（地址→PDB 符号）
  - `evidence/country-core-audit/B_s_h.md`、`evidence/country-core-audit/B_d_gap.md`
  - 当前源码：`crates/tiny-glade/src/{production_renderer,world_graph,world_postprocess,loading_graph,render_bridge,production_resolve}.rs`、
    `crates/country-core/src/systems/render_loop/**`

> 结论先行：**W54 之后不是“导出 TAA”，而是还有 9 类真实消费者**：
> `ColorGradingState::flattened`（CPU）→ `bidirectional_blur_pyramid`（GPU）→ `final_post`（生成 4×MSAA 目标）
> → 菜单/手绘/Vermeer/logo → 会话预览/LUT 画廊/反馈背景/wireframe/UI/UI2D/MEI/自定义光标/loading 列表/egui
> → panic 截图 → **export 0x22** → `RenderContext::finish`。
> 当前 `production_renderer.rs:201-226` 在 W54 后直接把 `out.taa` 以 0x22 导出，缺失上述全部尾部；
> `render_bridge.rs:883-884` 的 “complete compile_main” 文档在补齐前不成立。

---

## 1. 原生 compile_main 尾部（按调用顺序）

调用点全部位于 `0x140161cd0.c`；`render_world` 返回位置为 `:835`。下表地址来自机器码调用点，
符号来自 `procs.tsv`（O）；“资源身份”列注明该步是否真的产生 GPU 资源。

| # | 步骤 | 地址 | 调用点 | 输入 | 输出 / 资源身份 | 置信 |
|---|---|---|---|---|---|---|
| T0 | `render_world`（W1–W54）返回 `{depth,coc,taa}` | 0x1404ec110 | 0x161cd0.c:835 | main depth/color | `taa` RgTexture @ stack0x1898，格式 0x7a（`COLOR_HDR`），extent = color extent（TGSR 时 `extent_8`） | O |
| T1 | `ColorGradingState::flattened`（CPU） | 0x140b00b20 | :1085 | `ImageFormationConfigs` 当前档（`current_mut` 0x1408aea50，:1028） | **无 GPU 资源**；返回扁平化调色配置（name + 4×f32，bypass 清零），供 T3 环境 | O |
| T2 | `bidirectional_blur_pyramid` | 0x1409ed860 | :1109 | `taa`（stack0x1a00 = render_world 第三个 36B 输出） | 内部 `partial blur pyramid` + 输出 `full blur pyramid`，均 0x7a，mip 金字塔（storage 写、sampled 读） | O |
| T3 | `final_post` | 0x14090d000 | :1151 | src0 = `taa`（&0x1850）、src1 = `full blur pyramid`（&0x11b4）、grading/LUT 环境 | `"msaa color out"`：0x2b，`sample_count(4)`，extent = 目标显示 extent（`r9` 打包 `[w,h]`） | O |
| T4 | `handle_screenshot_opportunity` ×3 + `handle_screenshot_error` | 0x14031d140 / 0x1404ebfb0 | :1163、:1176（mode 0/1/2） | 刚生成的 msaa 目标 | 只登记截图请求；无固定资源 | O（调用）/U（mode 语义） |
| T5 | `render_menu_bg` | 0x1408c9a30 | :1187 | msaa 目标（&0x1a90） | 画到 msaa 目标；内部 AppMode 门控 + gauss blur | O |
| T6 | `painterly_pass` | 0x1408a6410 | :1197 | msaa 目标（&0x1a90）、settings | 画到 msaa 目标（渐进笔触） | O |
| T7 | `vermeer_pass` | 0x1408a82f0 | :1242 | msaa 目标、`mode==0x13` bool | 返回 bool（进入时非 Disabled）；画到 msaa 目标 | O |
| T8 | `render_logo_overlay`（条件） | 0x1408a5f50 | :1252 | msaa 目标 | 画到 msaa 目标；条件 = demo 字节 `*0x14362c098==1` 且（`bVar43∈{3,4}` 或 GameConfig+0xf3==1） | O（调用）/I（条件细节） |
| T9 | `handle_screenshot_opportunity` mode3 | 0x14031d140 | :1259 | msaa 目标 | 截图登记 | O |
| T10 | UI 块入口条件：`(vermeer_active & mode!=0x13)==0` | — | :1276 | T7 返回值、AppMode | 仅“Vermeer 未激活或处于 photomode”时执行 T11–T20 | I |
| T11 | `render_session_selection_previews` | 0x1408f6c00 | :1280 | msaa 目标 | 画到 msaa 目标 | O |
| T12 | `render_photomode_lut_gallery` | 0x1408b0560 | :1286 | msaa 目标 | 画到 msaa 目标 | O |
| T13 | `render_feedback_report_bg` | 0x1408a5a70 | :1292 | msaa 目标 | 画到 msaa 目标 | O |
| T14 | `wireframe_pass`（条件） | 0x1409ccb20 | :1299 | msaa 目标 + DrawLists | 画到 msaa 目标；条件 = `flag_0x1ac0 != 0` | O（调用）/U（flag 语义） |
| T15 | `ui_pass`（同一条件） | 0x140837c40 | :1307 | msaa 目标 + DrawLists(UI) | 画到 msaa 目标 | O |
| T16 | `ui_2d_pass` | 0x1409c4470 | :1316 | msaa 目标 + Ui2d 元素/WindowSize | 画到 msaa 目标 | O |
| T17 | `mei_pass` | 0x1409c6700 | :1329 | msaa 目标、MEI 文本输出（`uVar28`） | 内部 `"mei temp"`（0x2b）+ 字集图集；合成到 msaa 目标 | O（调用）/I（`uVar28` 生产） |
| T18 | `custom_cursor_pass` | 0x1409d1710 | :1335 | msaa 目标 + WindowSize + 光标可见字节 | 画到 msaa 目标 | O |
| T19 | `handle_screenshot_opportunity` mode4 | 0x14031d140 | :1341 | msaa 目标 | 截图登记 | O |
| T20 | `loading_screen_pass` | 0x140837ff0 | :1354 | msaa 目标 + DrawLists(LOADING_SCREEN) | 画到 msaa 目标（通常空列表） | O |
| T21 | `EguiBackend::draw`（条件：`plVar39[7]` Some） | 0x1400ddc80 | :1363 | msaa 目标、egui 后端 | 画到 msaa 目标 | O（调用）/I（世界帧必为 Some） |
| T22 | `handle_screenshot_opportunity` mode6 | 0x14031d140 | :1372 | msaa 目标 | 截图登记 | O |
| T23 | `panic_screenshot::capture_image` | 0x140589c90 | :1384 | msaa 目标 | 读取/复制最终图（非渲染 pass） | O（调用）/U（内部图像关联） |
| T24 | **导出最终纹理 0x22** | 0x140bd6380 | :1409 | msaa 目标 | `rhapsody::rg::import_export::export(handle, 0x22)`；供展示图 import | O |
| T25 | `RenderContext::finish` | 0x1409dda50 | :1419 | 已登记句柄 | 只做 `ManagedHandle::drop`，无渲染逻辑 | O |
| T26 | 资源导出：preset / SSBO / GpuWorld | 0x14093ce30 / 0x14091ebb0 / 0x140992220 | :1443 / :1466 / :1491 | rg | 库导出（不属于 pass 尾部） | O |
| T27 | `ResourceInfo::calculate` → `populate_resources` → `CompiledRenderGraph::new` | 0x140530430 / 0x140be7ba0 / 0x1402c4280 | :1736 / :1746 / :1776 | 声明完的图 | 编译产物 | O |

**顺序要点（O）**：grading 在 blur 之前；blur 无条件；final_post 在所有 UI 之前；**export 在全部 UI/egui 之后**，
因此最终 present 的图像包含 UI。`RenderContext::finish` 在 export 之后，只释放句柄。

---

## 2. 尾部每步的详细资源/访问

### 2.1 T1 grading（0x140b00b20）

- O：`FUN_140b00b20(sret, cfg_ptr)`，调用前 `0x1408aea50` 取 `ImageFormationConfigs::current_mut`（`:1028`）。
- O：源码 `crates/country-core/src/resources/image_formation.rs:174` 的 `flattened()`：`flag_28==0` 克隆 name + 4 f32；
  否则 name=`"default"`、params=`[1,0,1,1]`；输出 bypass 恒 0。
- O：结果装入 `final_post` 的闭包环境（`FinalPostEnv.cfg/grading`），**不创建 RgTexture、不产生 pass、无 access**。
- I：`bVar43`（来自 render_world 返回区 `0x1897`）决定是否走 `current_mut` 分支；与 photomode/模式相关，未闭合。

### 2.2 T2 blur（0x1409ed860）

- O：源码 `compute_passes/blur_pyramid.rs`：
  - `blur_pyramid` 0x1409ee1e0：`desc = src.half_res().format(0x7a).storage().sampled()`，`mips=max(m-1,1)`，
    资源 `"partial blur pyramid"`；pass `"blur0"` + `"blurN"`，读 `input_tex`（access 0x19），写 `output_tex`（access 0x37）。
  - `rev_blur_pyramid`（内联）：创建 `"full blur pyramid"`（0x7a，sampled+storage），`"rev_blurN"` 读 partial mip i 与
    full mip i+1，写 full mip i。
- O：输入 = T0 的 `taa`（调用点 `:1109` 的 `&stack0x1a00`，该区由 render_world 第三个 36B 输出复制而来）。
- 输出 = `full blur pyramid` RgTexture（mip 链），供 T3 `src1` 采样。
- I：在 `with_env`/`PassEnv` 上调用（与 `world_postprocess` 的 `scene_compute::with_env` 同路径）。

### 2.3 T3 final_post（0x14090d000）

- O：参数（`draw_passes/final_post.rs`，模块头 + `:322`）：
  - `rcx` sret → 返回 `"msaa color out"`（0x24B 句柄形态）；
  - `rdx` = `&mut RenderGraph`；`r8` = `&World`；
  - `r9` = 目标尺寸（打包 `[w,h]`；`0x161cd0.c:1112-1115` 对 `plVar39[6]` 做 `Option` 解包后取 u64）；
  - `[0x1b0]` src0 = **taa**（调用点 `&stack0x1850`，`:1122-1125` 从 `0x1a00` 复制；指针压栈在 `:1147-1148`）；
  - `[0x1b8]` src1 = **full blur pyramid**（调用点 `&stack0x11b4`，blur sret；指针压栈在 `:1145-1146`）；
  - `[0x1c0]` u8 = `flag_0x1ac0`（`:1143-1144`）；
  - `[0x1c8]` = LUT 覆盖（`&stack0x1490`，`:1126-1129`，压栈在 `:1141-1142`）；
  - `[0x1d0]/[0x1d8]` = 两个 f32（`settings+0x9c` / `settings+0xa4`，`:1133-1136`）。
- O：源码 `final_post.rs:322`：创建/使用 `"msaa color out"`（`msaa_color_desc`：0x2b、`sample_count=4`, `:203-215`），
  `raster_single_object` 写 color attachment（`ColorAttachment::Tag2`）。
- O：资源读取（源码模块头）：shader `Option<Handle<RasterShaderProgram>>`（无则 unwrap panic）、
  `ImageFormationConfigs` 当前档、`GladeTheme`、自定义 LUT（niche = none）。
- I：`r9` 的来源 `plVar39[6]` 语义 = 显示/目标 extent；在 B 源码中对应 `FrameRenderConfig.extent_8`
  （`render_config.rs:41`，TGSR 的 `display_dims` 也用它）。若 `r9` 为 inner extent，则 TAA 模式下不发生放大；
  该点列入 U，必须用 pass 录制对比复核。

### 2.4 T4–T23 UI 合成

- O：所有绘制调用的第三/第四参都是 `&stack0x1a90`，即 **final_post 的 `"msaa color out"`**；
  没有任何 UI 侧额外可 present 目标。
- O：`ui_pass`（`ui_pass.rs:34`）、`loading_screen_pass`（`:39`）、`ui_2d_pass`（`ui_2d_pass.rs:247`）、
  `mei_pass`（`:317`，含 `"mei temp"` + 字集图集）、`custom_cursor_pass`（`custom_cursor_pass.rs:113`）、
  `wireframe_pass`（`wireframe_pass.rs:20`）、logo/会话/LUT 画廊/反馈背景均在 country-core 已恢复。
- O：UI 块门控 = `vermeer_pass` 返回 false 或 `AppMode==0x13`（`:1276`，表达式见上表）。
- O：wireframe+ui_pass 共用 `flag_0x1ac0 != 0` 门控（`:1294`）；该 flag 在 `primary_rt_debug` 分支
  （`:680`，`*0x143634b90==1`）与另一路径（`:901`）被置 1。语义 **U**，不得自行假定“UI 恒开”。
- O：`EguiBackend::draw` 只在 `plVar39[7]` Some 时调用；世界帧该 Option 由 render 传入，加载帧为 None。
- O：`panic_screenshot::capture_image` 0x140589c90 后立刻复制最终句柄并 export 0x22。

### 2.5 T24 导出与 present

- O：`FUN_140bd6380(&handle, &rg, 0x22)`（`:1409`）= `rhapsody::rg::import_export::export`。
- O：展示图 `production_resolve.rs:24-58` 在 **0x22** 导入 main source，目标交换链 import access 0，
  `resolve_to_swapchain`，再把交换链图像 export **0x27**（`production_resolve.rs:44-51`）。
- O：`resolve_to_swapchain` 按 `source.sample_count` 选择 `raster_resolve_msaa_to_swapchain`
  （`final_post.rs:333-344`）；因此尾部输出必须是 **sample_count=4** 的 `"msaa color out"`，
  否则展示路径会错误地走非 MSAA shader。
- I：W54 的 `taa` 若继续作为 0x22 导出，则 present 的是一张 inner-resolution、未做 LUT/UI 的图；
  这正是当前实现的行为，属**可观察的缺失**，不是等价。

---

## 3. 与 loading 帧尾部的差异/可复用点

`build_loading_frame`（`loading_graph.rs:150-271`）当前消费者顺序（O，地址来自文件头注释）：

1. `with_frame_resources`（`loading_graph.rs:154`）——与主图共用 FrameSetup 前缀，但**不**走 `with_main_prelude`
   （无 GpuWorld import / LUT / reset indirect）。
2. 创建 `"loading screen"` 目标：`TextureDesc::new_2d(extent, 0x2b)`（`:190`），**无 MSAA**。
3. 可选背景纹理 import 0x22（`:191-194`）。
4. `EffectsHost` + `vermeer_render_from_target`（`:220-230`）——Vermeer 0x14016532a。
5. `MeiHost::import_into_frame` + `output()`（`:232-233`）——MEI 0x14016534a。
6. `"mei temp"` 0x2b + `ui_2d_pass::mei_pass`（`:239-248`）。
7. `ui_pass::loading_screen_pass`（`:261-268`）——loading draw list 0x14016536e。
8. `graph.export(target, 0x22)`（`:269`）——sampled export 0x1401653b7。

可直接复用（O）：

| 复用点 | 位置 | 说明 |
|---|---|---|
| `with_frame_resources` 前缀 | `loading_graph.rs:154`；`frame_setup.rs` | 主尾部仍需 `with_main_prelude` 之外的常量/预设/绑定；尾部可直接调用 |
| `EffectsHost`（painterly/vermeer/blur 原语） | `effects_host.rs:236-239`；`loading_graph.rs:220-230` | 提供 `TexRef` 注册、`compute`/`finish`，可复用为 T5/T6/T7 的 host |
| `MeiHost::import_into_frame` + `output()` | `loading_graph.rs:232-233`；`mei_host.rs:194-233,375` | T17 直接复用；要求先 `flush_mei_frame`（`loading_graph.rs:139-148`） |
| `DrawPassCtx` + `LibraryShaders` + `with_raster` 模式 | `loading_graph.rs:240-268` | T14–T20 的调用形状一致 |
| `ui_pass::loading_screen_pass` | `loading_graph.rs:266` | T20 原样复用 |
| 导出 0x22 模式 | `loading_graph.rs:269` | 主尾部改为导出 final_post 输出 |

不可复用/必须新增：

- loading 没有 `final_post`、grading、blur、MSAA 目标、菜单/手绘/UI2D/光标/logo/session/LUT/反馈/wireframe/egui。
  loading 的 `"loading screen"` 是 0x2b、单采样、直接 present；主尾部必须新建 `"msaa color out"`（0x2b、4×）。
- loading 不导入 GpuWorld/不渲染 LUT/不 reset indirect；主尾部必须已在 `with_main_prelude` 内。
- loading 的 MEI 生命周期属于 `LoadingGpuResources`；gameplay 由 `gameplay_startup.rs:85-89`
  的 `attach_loading_mei` 提供同名资源，可复用但必须确认 warm-up 时 `output()` 非 None。

---

## 4. 资源/顺序总表（尾部）

access 数值取自当前源码/原 helper 契约；pass 内部读取的 access 由 graph 按 `ColorSpec`/`BindTexture` 推导（I）。

| 资源 | 生产者 | 格式 | extent | usage/access | 消费者 | 置信 |
|---|---|---|---|---|---|---|
| `taa`（W54 输出） | `world_postprocess.rs:137-180` | 0x7a | TAA：color extent；TGSR：`extent_8` | sampled（T2/T3 src0） | T2、T3 | O |
| `partial blur pyramid` | `blur_pyramid.rs:85` | 0x7a | `half_res(src)`，mips | storage 写（0x37）、sampled 读（0x19） | rev_blur | O |
| `full blur pyramid` | `blur_pyramid.rs:150` | 0x7a | `half_res(src)`，mips | sampled+storage | T3 src1 | O |
| `msaa color out` | T3（`final_post.rs:203-215`） | 0x2b | 显示 extent（I） | 4×MSAA color attachment；随后 UI 同目标；export 0x22 | T4–T24 | O（调用）/I（extent 来源） |
| `mei temp` | T17（`loading_graph.rs:239`） | 0x2b | 显示 extent | color attachment tag2 | MEI | O |
| swapchain image | render R10 | swapchain | swapchain | import 0 → resolve 目标 → export 0x27 | present | O |

顺序依赖（O）：
`taa → full blur pyramid → msaa color out → (menu/painterly/vermeer/logo/UI*) → export 0x22 → resolve → present`。
`msaa color out` 在 T3 创建，所有 UI 共享同一资源；不存在“UI 单独目标再合成”的额外 step。
`mei temp` 仅在 T17 内使用。

---

## 5. warm-up 等价性判据

当前 `warm_up_gameplay_with_world_builder`（`render_bridge.rs:887-917`）只做：
`build_draw_lists/sort/refresh_vaos` → `build_frame(build_world)` → prewarm → compile → execute → `exported_texture`。
`warm_up_gameplay`（`production_renderer.rs:45-68`）的 builder 是
`with_main_prelude → world_extent → build_full_world_stage → 0x22(out.taa)`。**尾部缺失。**

可宣称“builder 等价 compile_main”的必要条件（全部满足才可）：

1. **顺序等价**：`with_main_prelude`（GpuWorld import、FrameSetup、LUT render、reset indirect）→ W1–W54 →
   T1 grading → T2 blur → T3 final_post → T4–T23 UI 尾 → T24 export 0x22 → T25 finish。
   以图录制/编译出的 pass 名序列与本节表逐项对齐（O 证据比对），而不是只看“编译成功”。
2. **导出身份等价**：最终 0x22 的资源必须是 `"msaa color out"`（0x2b、`sample_count=4`、显示 extent），
   且展示图走 `raster_resolve_msaa_to_swapchain`（`production_resolve.rs` 依 `sample_count` 选择）。
   继续导出 `taa` 不满足。
3. **输入等价**：T2/T3 消费的必须是 W54 返回的 `taa` 同一 RgTexture；不得另建替代纹理；
   blur 无条件执行（原生 `:1109` 无分支）。
4. **条件等价**：menu/painterly/vermeer 的内部 AppMode 门控、logo 条件、UI 块的
   `vermeer_active & mode!=0x13`、`flag_0x1ac0`、egui `Option`、截图请求状态均按原生谓词求值。
   未恢复的 flag 必须显式记录为 U，不能默认 true/false 后宣称等价。
5. **生产者严格性**：缺 `MainCamera`/`GameConfig`/`ImageFormationConfigs`/`DrawLists`/MEI 输出/
   final_post shader/`GladeTheme` 等任一真实生产者时返回硬错误（原生多处 `unwrap/expect`）；
   禁止静默 no-op、空列表冒充、“诊断成功”。
6. **前置/导出顺序**：GpuWorld export、preset/SSBO export 的位置与 native 一致或偏差已登记；
   `primary_rt_debug` 0x1409beca0 的条件性（U）已核实。

**诚实标注规则**：在 1–6 未全部满足前，warm-up 只能写
“world stage W1–W54（+已接的尾部件）”，并在交付中列出缺失的 T 步骤与 U 项；
不得使用 “complete compile_main” 或“与原生等价”。当前状态：**仅 W1–W54 + taa export**。

---

## 6. 入口契约（② 如何接）

推荐形状（不改变 rhapsody/session 身份）：

1. 新增尾部入口（可放 `tiny-glade/src/world_tail.rs` 或 `world_postprocess.rs` 同层）：
   ```rust
   pub fn build_world_tail(
       frame: &mut CoreGraphFrame<'_>,
       world: &mut World,
       lists: &DrawLists,
       taa: &RgTexture,          // W54 输出
       out_extent: [u32; 2],     // I: FrameRenderConfig.extent_8 / 显示 extent
       egui: Option<&mut EguiGpu>,
   ) -> rhapsody::Result<RgTexture>; // 返回 msaa color out
   ```
2. `build_full_world_stage`（`production_renderer.rs:201-226`）改为：
   `world_targets → build_world → build_world_tail → graph.export(final.resource, 0x22) → 返回 final.resource`。
   不得再导出 `out.taa`；返回类型仍是 `u32`，但输出资源换成最终目标。
3. `compile_world_graph`（`:602-625`）的 full 分支自动获得尾部；**不要**在 `compile_world_graph` 外面再接一遍
   （会重复声明/blur/final_post，导致双重 pass 与资源名冲突）。
4. `warm_up_gameplay`（`:45-68`）的闭包走同一 `build_full_world_stage`，因此同步获得尾部；
   `warm_up_gameplay_with_world_builder`（`render_bridge.rs:887`）的返回值描述改为最终目标的
   `RgTexture`（0x2b、extent_8、sample_count 4）。warm-up 仍只编译/执行/取回导出纹理，
   不 acquire/present（原生 loader 同样无 swapchain）。
5. **minimal/diagnostic 路径不接尾部**：`compile_minimal_world_graph`（`:309-326`）保持
   `clear + export`，它是显式诊断/demo（`ALLOW_MINIMAL_WORLD_FRAME_ENV`），
   既没有 W1–W38 的中间资源，也没有 UI/MEI 生产者；接了尾部只会制造假等价。
   `compile_world_graph` 的默认缺生产者路径仍必须硬错误（`:617-622`）。
6. 前置顺序：MEI 必须在构图前 flush（`loading_graph::flush_mei_frame`，`loading_graph.rs:139`）；
   `build_draw_lists/sort/refresh_vaos` 必须在 `build_frame` 前（`production_renderer.rs:576-582`、
   `render_bridge.rs:897-900`）；egui 后端不在 ECS world 时通过参数传入，缺失要显式记录。

---

## 7. ② ≤10 步实现清单

1. **grading**：在尾部先取 `ImageFormationConfigs` 当前档并调用 `flattened()`
   （`image_formation.rs:174`）；构造 `FinalPostArgs`（LUT 选择/GladeTheme/自定义 LUT/f0/f1/flag94）；
   缺 shader 资源 → 硬错误。
2. **blur**：`scene_compute::with_env(frame, world, |env| blur_pyramid::bidirectional_blur_pyramid(env, taa))`
   无条件执行，得到 `full blur pyramid`；校验 extent/format 0x7a。
3. **msaa 目标**：按 `final_post::msaa_color_desc(out_extent, id)` 用 `create_raw_texture("msaa color out", …)` 创建；
   `out_extent` 取 `FrameRenderConfig.extent_8`，0 维硬错误。
4. **final_post**：`frame.with_raster(|host| { DrawPassCtx::new(...); final_post::final_post(&mut ctx, &args, &attachment(msaa)) })`；
   进入时校验 src0=taa、src1=blur 均非空。
5. **菜单/手绘/Vermeer**：复用 `EffectsHost` 模式（`loading_graph.rs:220-230`）依次调用
   `render_menu_bg`/`painterly_pass`/`vermeer_pass`，全部以 msaa 目标为 target；保存 vermeer 返回 bool。
6. **logo + 条件 UI 块**：按 `demo && (mode∈{3,4} 或 GameConfig+0xf3)` 调 logo；
   以 `vermeer_active & mode!=0x13` 决定是否进入 UI 块；`flag_0x1ac0` 未恢复前先显式 U 并单独开关，
   不得默认开启后宣称等价。
7. **UI 合成**：依次 `wireframe_pass`（同 flag）、`ui_pass`、`ui_2d_pass`（元素查询）、
   `mei_pass`（`LoadingGpuResources.mei.output()` + `"mei temp"`）、`custom_cursor_pass`、
   `loading_screen_pass`，全部 target=msaa；构图前先 `flush_mei_frame`。
8. **egui + 截图钩子**：若世界帧提供 `EguiGpu` 则 `egui.draw(...)` 到 msaa；截图 opportunity/panic capture
   只在真实请求状态存在时执行，未恢复的关联返回明确错误，不伪造。
9. **导出/接线**：`graph.export(msaa.resource, 0x22)`；把 `build_full_world_stage`、`warm_up_gameplay`
   的返回/描述改为 msaa 目标；`compile_minimal_world_graph` 不变。
10. **等价性验证**：录制 pass 顺序与 §1 T0–T24 逐项 diff；`cargo check -p tiny-glade`；
    验证导出纹理 `sample_count==4` 且展示图选择 MSAA resolve；未过的条件（extent 来源、0x1ac0、
    primary_rt_debug、截图关联）列入 U，并在交付中把 warm-up 标注为“W1–W54 + 尾部子集”，
    直到 §5 六条全部满足。

---

## 8. 未闭合项（U/I）清单

- U：`plVar39[6]` 的 `Option<&[u32;2]>` 确切来源（显示 extent vs swapchain extent vs inner extent）。
  现按 `extent_8` 处理（I）。
- U：`flag_0x1ac0` 的真实语义（`primary_rt_debug` 分支与另一路径共同置 1）；它同时门控 wireframe 与 ui_pass。
- U：`bVar43`（render_world 返回区 `0x1897`）在 grading/logo 条件中的作用。
- U：`vermeer_pass` 返回 bool 与 photomode 组合的精确语义（`:1276` 表达式已按机器码抄录，语义为 I）。
- U：`panic_screenshot::capture_image` 的 request→GPU 纹理关联；当前源码 `capture_image()` 无句柄参数（适配缺口）。
- U：`handle_screenshot_opportunity` 的 mode 0/1/2/3/4/6 对应关系。
- I：`EguiBackend` 在世界帧的 `Option` 恒为 Some；加载帧为 None。warm-up 需显式传入。
- I：`final_post` 的 src0/src1 映射（taa / full blur pyramid）由栈复制与 blur sret 推断，建议用 pass 录制复核。

## 9. 未改源码声明

本契约只写证据文件；未修改 `crates/**`、rhapsody 本体、roof/wall 用户区。
