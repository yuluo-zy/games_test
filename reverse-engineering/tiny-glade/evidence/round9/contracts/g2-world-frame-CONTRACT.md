# G2 世界帧构图与执行契约（B 版 Rust 还原）

- 轮次：2026-10-19 第 9 轮 G2 ①（逆向取证，只读源码/只写证据）
- 基线：`D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`
- 标签：**O** = 机器码/反编译/真实源码可观察；**I** = 有证据的推断；**U** = 未知或未闭合
- 关键既有证据（本轮复核后引用）：
  - `evidence/render-recovery/producer-closure/c1-country-core/frame-composition.json`（W1–W54 的 VA→PDB 名有序表、compile 闭包前后顺序）
  - `evidence/render-passes/cpu-frame/sequence.md`（render 外框 R1–R20、render_world W1–W54、条件表）
  - `evidence/render-architecture/0x140161cd0.c`、`evidence/render-architecture/0x1404ec110.c`
  - `evidence/render-recovery/producer-closure/a2-graph-closure/findings.md`（构图/编译/批次机制）
  - `evidence/render-recovery/parallel/compiled-state/findings.md`（CompiledRenderGraph 0x218、record_batch）
  - `evidence/audit-2026-10-19/RENDER_LOOP_B-AUDIT.md`
- 本契约不修改任何源码；不补造 rhapsody API。

---

## 0. 结论摘要（给 ②）

1. **世界帧的生产消费者已经有真实实现，只是没有接线**：`crates/tiny-glade/src/world_graph.rs:42 build_world` 已按 W1–W54 顺序调用全部 typed consumer（W1–W34 直接调用，W35–W43 在 `world_lighting`，W44–W46 在 `world_surface`，W47–W54 在 `world_postprocess`）。它当前没有任何生产调用者（全 workspace grep 仅 `render_bridge.rs:890` 的参数名）。
2. `production_renderer.rs:322 compile_world_graph` 的职责应是：**解析世界尺寸 → 用 `CoreRenderBridge::build_frame` 在同一 `VulkanGraphSession` 上构图 → `with_main_prelude` + `build_world` → export 最终纹理 → `compile_built`**；不得新建第二个 session，不得返回“成功但没有真实导出纹理”的假 Compiled。
3. 最小可运行世界帧**不需要 W1–W54 任何 seam**：一个 `main color` 资源 + 一次 transfer clear（原版 `clear_rg_color_image` 0x1409dde90，access `0x34`）+ 导出 access `0x22` 即可让 `render()` 走完 execute/acquire/presentation/present。**第一步必须先做这个**，再逐步接 W1–W54。
4. `render/execution_host.rs` 不是执行者：它新建独立 `VulkanGraphSession`、无 compile/prepared/swapchain 路径、零调用者。唯一执行者必须保持 `tiny-glade/src/render_bridge.rs::CoreRenderBridge`（内部 `VulkanGraphSession`）。
5. `FramePasses`（63 个方法）的生产实现**不是最小帧的前提**：其等价 typed 消费者已存在。两条路线都可行：
   - 路线 A（推荐，成本最低）：②把 `world_graph::build_world` 定为唯一世界帧消费者；`FramePasses`/`render_world` 保留为机器码级顺序证据/测试 oracle。
   - 路线 B（已在本轮并行发生）：新增 `systems/render_loop/world_frame.rs` 的 `FramePasses` 生产适配器。若采用路线 B，适配器必须是**薄转发**：每个 seam 委托给现有 `scene_compute`/`world_*`/`scene_raster` typed consumer，不得复制 pass 逻辑、不得自带 session，并且 `build_minimal_world_frame` 仍是第一个验收里程碑。
   - 两条路线不得同时维护两套 pass 实现（会分叉）。

---

## 1. 原版入口时序

### 1.1 外框 `tiny_glade::render_loop::render` 0x1404e8010（O）

证据：`sequence.md §A`（R1–R20）、`crates/country-core/src/systems/render_loop/render_frame.rs:258`。

按机器码顺序：

| # | 动作 | 证据 |
|---|---|---|
| R1 | 取 ECS 资源（GraphicsDevice、GameConfig/VideoSettings） | render_frame.rs:265-269；O 0x1404e82c3 |
| R2 | `DrawLists::new` → `build_draw_lists` → `sort` | render_frame.rs:273；O 0x1409ba340/540/4b0 |
| R3 | `Swapchain::format/extent` → `Option<SwapchainInfo>` | render_frame.rs:274；O 0x140c0e7e0/0x140c0ea90 |
| R4 | 模式字节 == 0 → **loading** compile 0x1401647b0；否则 **world** compile 0x140161cd0 | render_frame.rs:276-286；O `cmp dil,2` / `test dil,dil` |
| R5 | world compile 失败 → `recompile_at_reduced_scale`：scale-0.1、clear pool、reset temporal、wait idle、reduce state=2、再 compile `min_extent=[32,32]` | render_frame.rs:335-357；O 0x1404e8a3d..9516 |
| R6 | `GraphExecutor::begin_frame`（可选 GPU profiler 缓冲） | render_frame.rs:294；O 0x140beabd0 |
| R7 | 收集已完成帧的 profiler 结果 | render_frame.rs:295；O 0x140294cf0 |
| R8 | **执行主图** `PassResources::new_from` + execute_compiled + retire + 取回导出资源 | render_frame.rs:299；O 0x1400fa190（`sequence.md` R7–R9） |
| R9 | 仅当编译成功且 swapchain 存在：`validate_swapchain_and_acquire_image` | render_frame.rs:304；O 0x140bfd9d0 |
| R10 | 展示图 compile+execute（`resolve_to_swapchain`），取 present source | render_frame.rs:305-307；O 0x1400fb5b0/0x140c0f170 |
| R11 | `end_frame` → `present` | render_frame.rs:310-312；O 0x140bec400/0x140c0ef80 |
| R12 | screenshot / RenderStats / memory budget / GPU pacing / 记录帧尾 | render_frame.rs:314-327；O 0x14031ea20/0x140c02da0/0x140bf8f80/0x140bea680 |

**时序结论（O）**：主图在 `begin_frame` 之前完成**声明 + 编译（allocate/prepare）**；执行在 begin_frame 之后；swapchain acquire 在**主图执行之后**、展示图执行之前。当前 `production_renderer.rs` 的 `compile_*` / `begin_frame` / `execute_main_graph` / `acquire_swapchain_image` / `execute_presentation_graph` / `present` 已按此顺序实现（`production_renderer.rs:296-410`），缺口仅在 `compile_world_graph`。

### 1.2 主图 compile 实例 0x140161cd0（世界图，9204 B）

证据：`frame-composition.json` 的 `compile_closure_order`；`sequence.md §A R4'`；`0x140161cd0.c:835`（`FUN_1404ec110` 调用点）。

闭包顺序（O，地址→名）：

1. `AssetTextureLibrary::import_texture_presets_to_rg` 0x14093cd30
2. `bindless_textures_dset` 0x14093d040
3. `GpuWorld::import_to_rg` 0x1409921c0
4. `DynamicConstants::allocate` 0x140c02740 + `into_uniform_and_storage_buffer_binding` 0x140c02370
5. `GpuLutTracker::render_luts` 0x14090e280
6. `reset_indirect_args` 0x14090a870
7. `primary_rt_debug` 0x1409beca0（调试消费者；是否无条件 U，见 §1.5）
8. **`render_world` 0x1404ec110（整个世界块）**
9. `ColorGradingState::flattened` 0x140b00b20
10. `bidirectional_blur_pyramid` 0x1409ed860
11. `final_post` 0x14090d000
12. `handle_screenshot_opportunity` / `handle_screenshot_error`（条件）
13. `render_menu_bg`、`painterly_pass`、`vermeer_pass`（条件）、`render_logo_overlay`、screenshot、`render_session_selection_previews`、`render_photomode_lut_gallery`、`render_feedback_report_bg`、`wireframe_pass`、`ui_pass`、`ui_2d_pass`、`mei_pass`、`custom_cursor_pass`、`loading_screen_pass`
14. `RenderContext::finish`
15. `export_preset_textures_from_rg` 0x14093ce30、`AssetSsboLibrary::export_from_rg` 0x14091ebb0、`GpuWorld::export_from_rg` 0x140992220
16. `ResourceInfo::calculate` 0x140530430 → `ResourceRegistry::populate_resources` 0x140be7ba0 → `CompiledRenderGraph::new` 0x1402c4280

对应到当前源码：
- 1–6 由 `CoreGraphFrame::with_main_prelude`（`render_bridge.rs:510-565`）实现：`import_gpu_world`（:470）→ `frame_setup::with_frame_resources`（FrameConstants/预设/bindless）→ `GpuLutTracker::render_luts` + `reset_indirect_args`；15 由 `export_gpu_world`（:489）与 `with_frame_resources` 尾部的 preset/SSBO 导出实现；16 由 rhapsody `VulkanGraphSession::compile_frame`（`rg/vulkan.rs:2296`）内部完成。
  - **顺序偏差（I）**：原始顺序是 presets → bindless dset → `GpuWorld::import_to_rg` → DynamicConstants → LUT → reset；当前 `with_main_prelude` 先 `import_gpu_world`，再 `with_frame_resources`（其中才做 presets/bindless/constants）。差异只影响图内资源声明顺序/索引，不影响语义；接线时若要逐 index 对齐原始图，应调整为原始顺序并在交付中登记。
- **7 `primary_rt_debug` 当前未接**：`compute_passes/primary_rt_debug.rs:22` 是真实实现，但 `with_main_prelude` 未调用。它出现在 frame-composition 的无条件序列里；是否实际受 debug 开关保护为 **U**。最小帧不需要它，全量世界帧接线时须核实。

### 1.3 `render_world` 0x1404ec110 的输入/输出（O/I）

- 签名（Ghidra/反编译）：`(sret, rg, world, param_4, param_5, param_6, param_7, param_8)`（`0x1404ec110`，4314 B）。
- `param_4` = `&DrawLists`（**I**：`render_world.rs:20` 的参数注释；传给 shadow/main/transparent 的列表参数）。
- `param_5` = `world_render_dims: [u32;2]`（**O**：断言文本、两维 >0；`render_world.rs:442-447`）。
- `param_6` = 设置字节：`+0x9c` sharpen f32、`+0xa9/+0xaa` DoF 开关（下标=`dof_mode==0x13`）、`+0xab` TGSR 开关（**O**：`sequence.md §C`）。
- `param_7` = 深度 `RgTexture`（36 B：id + 32 B desc）；`param_8` = 场景颜色 `RgTexture`（**O**：`0x140161cd0.c:829-835` 的四个栈实参顺序 + call 前 `stack0x140c`/`stack0x1490` 的 36 B 拷贝；`render_world.rs:149-171`）。
- 输出：`RenderWorldOutputs` 0x6c B = `{depth, dof_prepare.coc, taa}`（**O**：sret 三个 36 B 片段；`render_world.rs:161-170`）。

`0x161cd0` 调 `render_world` 之前的世界目标资源（**O**，本轮新读取）：

| 名称（.rdata 实读） | 创建 | 依据 |
|---|---|---|
| `"main color"` | `TextureDesc::new_2d(world_extent, 0x7a)` + 0x140bd5460(...,6)（写 `+0x1c` = `mip_levels` = 6）→ `create_raw_resource` 0x140be94d0 | `0x161cd0.c:348/376`；字符串 `0x14295d6a8` = `"main color"`（rne-read 实读）；0x140bd5460 机器码 `44 88 42 1c` = `mov [rdx+0x1c], r8b` |
| `"main depth"` | `TextureDesc::new_2d(world_extent, 0x82)` → `create_raw_resource` | `0x161cd0.c:694/768`；字符串 `0x14295d798` = `"main depth"`；0x82 = `VK_FORMAT_D32_SFLOAT_S8_UINT`（I，按 Vulkan 枚举值） |
| `"coc"`（render_world 之后） | `TextureDesc::new_2d([1,1], 9)` → `create_raw_resource` | `0x161cd0.c:907-934`；字符串 `0x14295d8a8` = `"coc"` |

> 说明：`0x140bd6ce0` 是 32 B 结构复制（`movups` 两段 + ret），不是 builder。因此这些 desc 的 usage 位不来自该调用；当前 source-build 中 `TextureDesc::convert`（`render_host.rs:401-427`）要求至少一个 usage bool，故 ② 必须显式设 `color_attachment`/`depth_stencil_attachment`/`sampled`（**I**，属表示层适配，不改变原逻辑）。

### 1.4 W1–W54 的代表性逐点确认

`render_world.rs:442-638` 的调用顺序与 `frame-composition.json` 的 VA 序完全一致（O，逐行对照）。抽点确认：

| 点 | render_world.rs 行 | VA | 确认 |
|---|---|---|---|
| W1 | 465-467（6 槽循环 `UPLOAD_RASTER_SLOTS = [1,2,3,4,0x13,0x14]`） | 0x1403d8bd0/0x1403da050/0x1403da570/0x1403d90f0/0x1403d9610/0x1403d9b30 | O：6 个单态；槽位常量在 `render_world.rs:206` |
| W2 | 469 | 0x140a12680 | O |
| W10 | 477 | 0x1409014e0 | O（`compute_path_masks`） |
| W21/W22 | 489/490 | 0x14098aaf0 / 0x140987dd0（hzb=None） | O：第一轮 hzb 参数为 0 |
| W23/W25/W26 | 491/512/513 | 0x1409cc270 / 0x140833bc0 / 0x1409286f0 | O |
| W27/W28 | 514-517 | 0x14098b2a0 / 0x140987dd0（Some(hzb)） | O：条件 `!flag_c0d8` |
| W34 | 523 | 0x1408373b0 | O |
| W44/W45 | 565-590 | 0x1409bf320 / 0x1409bf800（水）或 0x1409c08d0（冰） | O：`!flag_c098`；冰/水按 `is_winter` |
| W52 | 609-628 | 0x140a3b2f0（TAA）/ 0x140a3ba60（TGSR） | O：`settings.tgsr` 互斥 |
| W54 | 637 | 0x14098f9f0 | O（职责 U，仅 RwLock+unwrap 可见） |

独立佐证：`sequence.md §B/§C` 已按同一 VA 序记录；两处独立证据一致。

### 1.5 条件与 U

- `flag_c0d8` = `*0x14362c0d8 → 0x143634da0`（freeze occlusion culling）；`flag_c098` = `*0x14362c098 → 0x143634d90`（demo）；两者加载时 0（O，`render_world.rs:231-236` 注释与 `render-gap/DIGEST.md §3.4`）。
- `is_winter` = `GladeType::is_winter`（0x140ae8320，ICF 别名）；水/冰选择（O）。
- `dof_mode` = `AppMode` native tag，0x13 = photomode（O `render_world.rs:184-187`）。
- `primary_rt_debug` 的条件性：**U**（见 §1.2）。
- `render_shader_refactoring` 的真实职责：**U**（机器码只见 RwLock/read_contended 与 unwrap）。

---

## 2. 两个执行门面：谁应是唯一执行者

### 2.1 现状证据

| 门面 | 位置 | 能力 | 调用者 |
|---|---|---|---|
| `country_core::render::execution_host::RhapsodyExecutionHost` | `crates/country-core/src/render/execution_host.rs:24-53` | `new(device)` 自建 `VulkanGraphSession`；`begin_frame` / `execute_graph`(=compile+execute) / `complete_primary` / `wait_idle` | **0**（全 `crates/` grep 无引用；仅模块声明 `render/mod.rs:27`） |
| `tiny_glade::render_bridge::CoreRenderBridge` | `crates/tiny-glade/src/render_bridge.rs:35-367` | 持有唯一 `Rc<RefCell<VulkanGraphSession>>`；`build_frame`(:214)、`compile_built`(:249)、`execute_compiled`(:260)、`execute_presentation_compiled`(:274)、`cancel_built`(:291)、`exported_texture`(:297)、`take_exported_texture`(:334)、`with_session`(:354)、`collect_completed_frames`(:358)/`wait_submits_done`(:362) | **生产主链**：`production_renderer.rs`（loading/main/presentation）、`loading_graph.rs`、`production_resolve.rs` |

`execution_host.rs` 自述“GPU 真机执行后置”（:7-8），且它的 `execute_graph` 直接走 `execute_frame`（compile+execute 一步），没有外层 `render()` 所需的“声明与物理编译先于 begin_frame、执行时只 execute_prepared”的分离；也没有 present/导出/swapchain 注入。**结论（O/I）**：它不是生产执行者，也不应成为第二个执行者。

### 2.2 推荐架构

```
country-core（只产出 pass 描述，不持有 session）
  RenderGraph / RgResources / PassFrame / ComputeExec / DrawBuilder
        │  create_raw_texture / add_resource_access / render_with_result_context
        ▼
tiny-glade::CoreGraphFrame（同一 App 的唯一声明门面）
  with_main_prelude / with_compute / with_raster / with_graph / import_*
        │  graph.build() → BuiltGraph
        ▼
tiny-glade::CoreRenderBridge（唯一执行者）
  session = VulkanGraphSession(VulkanGraphBackend(device, AccessTable::original()))
  compile_built → execute_compiled / execute_presentation_compiled
  exported_texture / take_exported_texture / collect / wait
        ▼
production_renderer::ProductionRenderHost（RenderHost 外框时序）
```

规则：
1. **一个 App 一个 session**。`render_bridge.rs` 头注释明确“same graph/session, device and asset-library identities”；temporal/named 资源归 session 所有，`warmup_owners` 甚至拒绝从已有 temporal owner 分叉（`render_bridge.rs:92-96`）。第二 session 会丢失 history 并使 export token 失效。
2. `execution_host.rs` 的后续处理（不在本轮②必要范围）：或删除，或改为 `fn bind(session: Rc<RefCell<VulkanGraphSession>>)` 的薄适配；**禁止**再 `VulkanGraphSession::new`。
3. `render_world` 的 63-seam 生产实现如果要做（路线 B），只能实现为“把 seam 翻译到 `CoreGraphFrame` 上的现有 typed consumer”，绝不能自带 session；未恢复的 seam 必须返回**带 seam 名**的显式错误，禁止静默 no-op / 占位成功。

---

## 3. 最小可运行世界帧（②第一步）

目标：world 模式不再 panic；加载路径不受影响；输出一张可被现有展示图消费的纹理（空场景 = 透明黑）。

### 3.1 最小 pass 集合

| 步 | 动作 | 机制 | 原版对应 |
|---|---|---|---|
| M1 | 计算尺寸 `dims = min_extent.unwrap_or(world_dims)`；`dims` 任一维为 0 → 原版 panic（I） | `FrameRenderConfig.extent_10`（见 §5.1） | `render_world` 断言（O） |
| M2 | `create_raw_texture("main color", TextureDesc::new_2d(dims, 0x7a).mip_levels(6).color_attachment().sampled())` | `RgResources::create_raw_texture` → `GraphPasses::create_raw_texture` → `VulkanGraphBackend::declare` | `create_raw_resource("main color", 0x7a)`（O） |
| M3 | 一个 transfer clear pass：`clear_rg_color_image(compute, color.resource)` | `rg_helpers` 原版 helper：pass 名 `"Clear image"`、access `0x34`、回调 `clear_color_image` | 原 helper 0x1409dde90（O） |
| M4 | `with_graph(|g| { let h = g.resource_handle(color.resource)?; g.export(h, 0x22)?; Ok(()) })` | `RenderGraph::export` | 主图最终 sampled 导出（0x22；I，由展示图 import 0x22 反推） |
| M5 | `prewarm(10s)` → `compile_built` → `CompiledGameGraph { graph, output: color.resource }` | `CoreRenderBridge` | 与 loading 相同路径（O） |
| M6 | 展示：现有 `production_resolve::build` + `execute_presentation_graph`（无需改） | `final_post::resolve_to_swapchain` | compile-present 0x140166510（O） |

> M3 的 clear 颜色是 `{0,0,0,0}`、布局 `TRANSFER_DST_OPTIMAL`（`rg/pass_api.rs:210-216`、`:490-500`）；access `0x34` 的原版策略 = stage `0x1000`、access `0x1000`、layout `7`（`rg/raw/access_table.rs:335-340`）。因此空场景呈现为黑，可验证“真实 GPU 执行 + 真实导出”。

### 3.2 需要的真实 API 签名（文件:行）

```rust
// tiny-glade/src/render_bridge.rs
pub fn build_frame<T>(&mut self, build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>)
    -> rhapsody::Result<(T, rhapsody::rg::graph::BuiltGraph)>;          // :214
pub fn compile_built(&mut self, graph: BuiltGraph) -> rhapsody::Result<CompiledCoreGraph>; // :249
pub fn execute_compiled(&mut self, compiled: CompiledCoreGraph) -> rhapsody::Result<SessionFrameReceipt>; // :260
pub fn cancel_built(&mut self, graph: BuiltGraph) -> rhapsody::Result<()>; // :291

// CoreGraphFrame
pub fn with_compute<T>(&mut self, build: impl FnOnce(&mut CoreComputeFrame<'_,'_>) -> rhapsody::Result<T>) -> rhapsody::Result<T>; // :425
pub fn with_graph<T>(&mut self, build: impl FnOnce(&mut RenderGraph) -> rhapsody::Result<T>) -> rhapsody::Result<T>; // :580

// country-core
fn create_raw_texture(&mut self, name: &str, desc: &TextureDesc) -> RgTexture;  // pass_builder/rg_resources.rs:66
pub fn clear_rg_color_image(graph: &mut dyn RenderGraph, image: u32);            // render/rg_helpers.rs:128
impl RenderGraph for CoreComputeFrame { fn add_pass(...); }                      // tiny-glade/src/compute_frame.rs:163

// rhapsody
pub fn new() -> RenderGraph;                                                     // rg/graph.rs:258
pub fn create_raw_resource(&mut self, name, origin: ResourceOrigin) -> Result<ResourceHandle, BuildError>; // :262
pub fn resource_handle(&self, index: u32) -> Result<ResourceHandle, BuildError>; // :310
pub fn export(&mut self, handle: ResourceHandle, access_code: u8) -> Result<(), BuildError>; // :362
pub fn build(self) -> BuiltGraph;                                                // :402
pub fn compile_frame(&mut self, built: BuiltGraph) -> Result<PreparedSessionFrame>; // rg/vulkan.rs:2296
pub fn execute_prepared(&mut self, frame: PreparedSessionFrame) -> Result<SessionFrameReceipt>; // :2633
```

### 3.3 最小实现形状（示意，不含完整错误处理）

```rust
fn compile_world_graph(&mut self, _swapchain: Option<&SwapchainInfo>, min_extent: Option<[u32; 2]>)
    -> rhapsody::Result<Self::Compiled>
{
    // 需新增的小 helper：min_extent 优先，否则 FrameRenderConfig.extent_10；0 维 → 原版 panic
    let dims = min_extent_or_world_dims(self.world, min_extent)?; // §5.1
    let (output, built) = self.renderer.bridge.build_frame(|frame| {
        let color = frame.with_compute(|compute| {
            let color = compute.create_raw_texture(
                "main color",
                &TextureDesc::new_2d(dims, 0x7a).mip_levels(6).color_attachment().sampled(),
            );
            country_core::render::rg_helpers::clear_rg_color_image(compute, color.resource);
            Ok(color)
        })?;
        frame.with_graph(|graph| {
            let handle = graph.resource_handle(color.resource)?;
            graph.export(handle, 0x22)?;
            Ok(())
        })?;
        Ok(color)
    })?;
    if let Err(error) = self.renderer.bridge.shader_backend().prewarm(Duration::from_secs(10)) {
        self.renderer.bridge.cancel_built(built)?;
        return Err(error);
    }
    let graph = self.renderer.bridge.compile_built(built)?;
    Ok(CompiledGameGraph { graph, output: output.resource })
}
```

注意：`clear_rg_color_image` 要求 `CoreComputeFrame` 的 `rg_boundary::RenderGraph` trait 在作用域内；`GraphPasses::add_pass` 会登记 `add_resource_access(resource, 0x34, 0)` 并在回调里经 `PassApiHost::clear_color_image`（`render_host.rs:2005-2019`）解析物理 image。

---

## 4. 63 个 seam 映射表

列含义：
- **状态**：`真实现` = core/tiny-glade 有真实 typed consumer，且从 `world_graph::build_world` 可达（但整条路线尚未被生产调用）；`内联` = 无同名函数，由调用方状态/资源提供；`部分` = 已有实现但内部仍有确认缺口；`测试 host` = 只有 `render_world.rs` 测试 `Rec` 实现（trait 本身）。
- **最小集**：仅 `create_raw_resource` 属于 §3 的最小帧；其余为全量目标。
- “原版职责”证据默认来自 `render_world.rs` 该行上方的 PDB 名/地址注释 + `frame-composition.json`。

| # | seam（render_world.rs 行） | 原版 VA | 原版职责 | 状态（当前实现位置） | 最小集 |
|---|---|---|---|---|---|
| 1 | `is_winter` (225) | 0x140ae8320 | 取世界类型，冬季走冰面 | 真实现：`resources/session.rs` `GladeType::is_winter`；`world_graph.rs:48` + `world_surface.rs` `select_water_surface` | 否 |
| 2 | `display_dims` (227) | 分辨率资源 `+8` | TGSR 输出/历史显示尺寸 | 真实现：`FrameRenderConfig.extent_8`（`world_postprocess.rs:82-84,97,133`） | 否 |
| 3 | `dof_mode` (229) | AppMode 资源 | photomode tag 0x13 | 真实现：`AppMode::native_tag()`（`world_postprocess.rs:70-75`） | 否 |
| 4 | `flag_c0d8` (233) | `*0x14362c0d8→0x143634da0` | 冻结遮挡剔除（跳过 W27/W28） | 内联：`WorldGraphControls.freeze_occlusion_culling`（`world_graph.rs:29-31,75`）；**无生产读取点**（缺口） | 否 |
| 5 | `flag_c098` (236) | `*0x14362c098→0x143634d90` | demo：跳过折射/水面 | 真实现：`GameConfig.dev.demo`（`world_graph.rs:105`） | 否 |
| 6 | `get_or_create_temporal` (240) | 0x1404f6e50 | prev_depth 历史 | 真实现：`RgResources::get_or_create_temporal_texture`（`rg_resources.rs:70`；`render_host.rs:1570`；`world_lighting.rs:83-105`） | 否 |
| 7 | `get_or_create_temporal_pair` (246) | 0x1404f7450 | TAA/TGSR、GI 历史对 | 真实现：`rg_resources.rs:76`；`render_host.rs:1603`（`session.temporal_texture`） | 否 |
| 8 | `create_raw_resource` (252) | 0x140be94d0 | gbuffer/velocity/折射源/输出 | 真实现：`rg_resources.rs:66`；`render_host.rs:1519` | **是** |
| 9 | `upload_raster_staging_to_texture` (255) | 6×（见 §1.4） | 世界光栅 6 槽上传 | 真实现：`scene_compute.rs:118/138`（`upload_world_rasters` 循环 6 槽） | 否 |
| 10 | `water_rasterize_from_cpu` (256) | 0x140a12680 | 水面 CPU 光栅化 | 真实现：`scene_compute.rs:153` → `compute_water.rs` | 否 |
| 11 | `rasterize_terrain_stroke` (257) | 0x1408c7120 | 地形笔触 | 真实现：`world_terrain.rs:92` | 否 |
| 12 | `displace_rocky_terrain` (258) | 0x140a3ee80 | 岩石地形位移 | 真实现：`world_terrain.rs:314` | 否 |
| 13 | `read_terrain_data` (259) | 0x14098c680 | 读回地形数据 | 真实现：`world_terrain.rs:382` | 否 |
| 14 | `mouse_read_terrain_stroke_index` (260) | 0x1409768b0 | 鼠标命中笔触 | 真实现：`world_terrain.rs:128` | 否 |
| 15 | `rasterize_foliage_exclusion_mask` (261) | 0x1409cb7b0 | 植被排除掩码 | 真实现：`world_terrain.rs:35` | 否 |
| 16 | `highlight_active_terrain_stroke` (262) | 0x140a58450 | 高亮活动笔触 | 真实现：`world_terrain.rs:147` | 否 |
| 17 | `object_grass_push_pass` (263) | 0x1409daaa0 | 草对象推送 | 真实现：`world_terrain.rs:227` | 否 |
| 18 | `compute_path_masks` (264) | 0x1409014e0 | 路径掩码 | **部分**：`world_terrain.rs:188`；内部 `blur_exclusion_mask` 缺失（见 §7） | 否 |
| 19 | `rasterize_buildings` (265) | 0x140a32770 | 建筑光栅化 | 真实现：`world_terrain.rs:47` | 否 |
| 20 | `garden_rasterize_from_cpu` (266) | 0x140a22140 | 花园光栅化 | 真实现：`scene_compute.rs:166` | 否 |
| 21 | `water_spawn_vegetation` (267) | 0x140a13970 | 水生植被 | 真实现：`world_vegetation.rs:30` | 否 |
| 22 | `spawn_vegetation_around_buildings` (268) | 0x1408e4cc0 | 建筑周边植被 | 真实现：`world_vegetation.rs:57` | 否 |
| 23 | `read_bush_positions_for_cpu` (269) | 0x140866540 | 读回灌木位置 | 真实现：`world_vegetation.rs:83` | 否 |
| 24 | `garden_spawn_vegetation` (270) | 0x140a22b80 | 花园植被 | 真实现：`world_vegetation.rs:103` | 否 |
| 25 | `path_pebbles_compute` (271) | 0x140958f70 | 路径碎石 | 真实现：`world_vegetation.rs:120` | 否 |
| 26 | `leaf_particles_compute` (272) | 0x1408f4d00 | 叶片粒子 | 真实现：`world_geometry.rs:30` | 否 |
| 27 | `roof_tiles_gravity_compute` (273) | 0x140a29b10 | 屋顶瓦重力 | 真实现：`world_geometry.rs:109` | 否 |
| 28 | `stairs` (274) | 0x140a68a50 | 楼梯 | 真实现：`world_geometry.rs:45` | 否 |
| 29 | `generate_wall_draw_lists` (275) | 0x14098aaf0 | 第一轮墙列表 | 真实现：`world_walls.rs:60` | 否 |
| 30 | `gpu_generate_draw_lists` (277) | 0x140987dd0 | GPU 剔除/分桶（两轮） | 真实现：`scene_compute.rs:182`（`generate_nani_draw_lists`，`None`/`Some(hzb)`） | 否 |
| 31 | `shadow_pass` (278) | 0x1409cc270 | 阴影图 | 真实现：`scene_raster.rs:116` | 否 |
| 32 | `calc_terrain_lighting_texture` (279) | 0x140a51440 | 地形光照纹理 | 真实现：`world_geometry.rs:143` | 否 |
| 33 | `main_pass` (282) | 0x140833bc0 | 主几何/深度/速度 | 真实现：`scene_raster.rs:87` | 否 |
| 34 | `generate_hzb` (284) | 0x1409286f0 | HZB mip 链 | 真实现：`scene_compute.rs:233` | 否 |
| 35 | `generate_wall_draw_lists_pass2` (285) | 0x14098b2a0 | 第二轮墙列表 | 真实现：`world_walls.rs:73` | 否 |
| 36 | `grass_compute` (286) | 0x140a60280 | 草间接绘制生成 | 真实现：`world_geometry.rs:54` | 否 |
| 37 | `main_pass_oc2` (287) | 0x140835a40 | 遮挡二次主绘制 | 真实现：`scene_raster.rs:103` | 否 |
| 38 | `run_visibility_queries` (288) | 0x140a2f2e0 | 可见性查询 | 真实现：`world_geometry.rs:157` | 否 |
| 39 | `calculate_dof_autofocus` (289) | 0x140a17880 | DoF 自动对焦 | 真实现：`world_geometry.rs:70` | 否 |
| 40 | `mouse_depth_compute` (290) | 0x140a01620 | 鼠标深度 | 真实现：`world_geometry.rs:170` | 否 |
| 41 | `late_forward_and_deferred_pass` (291) | 0x1408373b0 | 延迟光照 | 真实现：`scene_raster.rs:110` | 否 |
| 42 | `extract_half_res_depth_normals` (292) | 0x1408f6180 | 半分辨率深度/法线 | 真实现：`scene_compute.rs:239`；`world_lighting.rs:148` | 否 |
| 43 | `calculate_reprojection_map` (293) | 0x140a0d190 | 重投影图 | 真实现：`scene_compute.rs:253`；`world_lighting.rs:149` | 否 |
| 44 | `calculate_ssgi` (299) | 0x1408743b0 | SSGI | 真实现：`scene_compute.rs:275`；`world_lighting.rs:154` | 否 |
| 45 | `ray_trace_gi` (308) | 0x140a01cf0 | 软件 BVH GI | 真实现：`scene_compute.rs:291`；`world_lighting.rs:174` | 否 |
| 46 | `eval_gi_screen_radiance` (317) | 0x140a44cd0 | GI 屏幕辐射 | 真实现：`scene_compute.rs:317`；`world_lighting.rs:183` | 否 |
| 47 | `bin_lights` (324) | 0x1409e7770 | 光源分桶 | 真实现：`scene_compute.rs:346`；`world_lighting.rs:184` | 否 |
| 48 | `restir_sample_direct_lights` (325) | 0x140a6fec0 | ReSTIR 直接光 | 真实现：`scene_compute.rs:352`；`world_lighting.rs:185` | 否 |
| 49 | `apply_screen_shadows` (331) | 0x140a39e70 | 屏幕空间阴影 | 真实现：`scene_compute.rs:385`；`world_lighting.rs:186` | 否 |
| 50 | `deferred_raster` (337) | 0x1409d20b0 | 延迟光栅 | 真实现：`scene_raster.rs:119`；`world_lighting.rs:234` | 否 |
| 51 | `refraction_copy_pass` (353) | 0x1409bf320 | 折射源拷贝 | 真实现：`world_surface.rs:58` | 否 |
| 52 | `water_surface_pass` (360) | 0x1409bf800 | 水面 | 真实现：`world_surface.rs:65-70`（`prepare/raster/finish_water_surface`） | 否 |
| 53 | `ice_surface_pass` (371) | 0x1409c08d0（I：由 `sequence.md §B5` 给出） | 冰面 | 真实现：`world_surface.rs:71-75`（`prepare/raster/finish_ice_surface`） | 否 |
| 54 | `pre_taa_transparency_pass` (381) | 0x1409c3a80 | TAA 前透明 | 真实现：`scene_raster.rs:139`；`world_surface.rs:79` | 否 |
| 55 | `compute_shadow_mask_mean` (389) | 0x140a3a6f0 | 阴影均值 | 真实现：`scene_compute.rs:394`；`world_postprocess.rs:109` | 否 |
| 56 | `apply_volumetrics` (390) | 0x1408a44c0 | 体积光 | 真实现：`scene_compute.rs:402`；`world_postprocess.rs:110` | 否 |
| 57 | `dof_prepare` (391) | 0x140a5a020 | DoF 准备/CoC | 真实现：`scene_compute.rs:435`；`world_postprocess.rs:111` | 否 |
| 58 | `generate_mips` (396) | 0x1400fb890 | DoF mip 链（条件） | 真实现：`scene_compute.rs:414`；`world_postprocess.rs:118` | 否 |
| 59 | `calc_dof` (398) | 0x140a5a570 | DoF（条件） | 真实现：`scene_compute.rs:423`；`world_postprocess.rs:120` | 否 |
| 60 | `calculate_taa` (406) | 0x140a3b2f0 | TAA | 真实现：`scene_compute.rs:461`；`world_postprocess.rs:163` | 否 |
| 61 | `calculate_tgsr` (416) | 0x140a3ba60 | TGSR | 真实现：`scene_compute.rs:469`；`world_postprocess.rs:150` | 否 |
| 62 | `post_taa_transparency_pass` (426) | 0x1409c40b0 | TAA 后透明 | 真实现：`scene_raster.rs:150`；`world_postprocess.rs:174` | 否 |
| 63 | `render_shader_refactoring` (427) | 0x14098f9f0 | 整帧替换（职责 U） | 真实现：`world_postprocess.rs:175`（定义 :187）→ `compute_passes/shader_refactoring.rs` | 否 |

**Trait 级别状态（O）**：`FramePasses` 在 `render_world.rs:219` 定义，唯一实现者是同文件 `#[cfg(test)] Rec`（`render_world.rs:686` 起）。因此“63 个 seam 无生产 host”这一表述成立；但**功能**并非缺失，而是走 `world_graph::build_world` 的 typed 路径。

---

## 5. `compile_world_graph` 实现契约

### 5.1 输入

| 输入 | 语义 | 证据/置信度 |
|---|---|---|
| `swapchain: Option<&SwapchainInfo>` | 世界主图**不依赖** swapchain 格式/尺寸；`None` 时必须可编译并执行（只是不 present） | O：`render()` 只在 loading 分支 `expect` swapchain（`render_frame.rs:278-279`），world 传给 compile 的是 `swapchain.as_ref()`（:282） |
| `min_extent: Option<[u32;2]>` | OOM 降分辨率后的兜底尺寸覆盖；调用值固定 `[32,32]` | O：`render_frame.rs:25,356`；覆盖语义 **I** |
| `world_dims`（无 min_extent 时） | 内部分辨率 = `FrameRenderConfig.extent_10`（`inner_rendering_resolution` 的来源） | O：`prepare_frame_constants.rs:205`；I：extent_10 即 world 尺寸（与 `global_mip_bias=extent_10/extent_8` 一致，:214） |
| `DrawLists` | 已在 `render()` 的 R2 构建并排序；世界图直接使用 | O：`render_frame.rs:273`；`production_renderer.rs:296-302` |
| `World` 资源 | 至少：`GameConfig`、`FrameRenderConfig`、`AppMode`、`ActiveSession`、`MainCamera`、`GpuWorld`、`GpuLutTracker`、四个资产库、间接 SSBO 四个 | O（`world_graph.rs`/`world_postprocess.rs`/`with_main_prelude` 的 required 检查） |
| shaders / bridge session | 与加载/展示共用的同一 `AssetShaderLibrary` + `CoreRenderBridge` session | O：`CoreRenderBridge` 设计约束 |

### 5.2 输出

- `Self::Compiled = CompiledGameGraph { graph: CompiledCoreGraph, output: u32 }`（`production_renderer.rs:248`）。
- `output` = 最终输出纹理的**图内逻辑索引**，且必须被 `export(..., 0x22)` 导出；`execute_main_graph` 用 `take_exported_texture(&receipt, output)` 取物理 owner（`production_renderer.rs:359-367`）。
- `CompiledCoreGraph` 持有 `PreparedSessionFrame`；未执行即 Drop 会自动 `cancel_prepared`（`render_bridge.rs:369-387`）——失败路径不需要 ② 手工回收。

### 5.3 构建顺序（契约）

```
dims = min_extent.unwrap_or(world_dims); assert dims > 0             // O 原版断言
bridge.build_frame(|frame| {
    // 原版 prelude（O）：
    //   GpuWorld::import_to_rg → FrameConstants → LUT → reset_indirect
    frame.with_main_prelude(world, |frame, world| {
        // 世界目标（O）：
        //   "main color" = new_2d(dims, 0x7a).mip_levels(6)
        //   "main depth" = new_2d(dims, 0x82)
        // W1..W54（O 顺序；当前实现 = world_graph::build_world）
        world_graph::build_world(frame, world, &lists, dims, depth, color, controls)
    })?;
    // 原版闭包尾部（O）：grading / blur / final_post / UI（全量阶段再接）
    // 导出最终图（I：access 0x22，与展示图 import 0x22 对称）
    frame.with_graph(|graph| { graph.export(final_handle, 0x22)?; Ok(()) })
})?;
prewarm(10s)?;                    // 失败 → cancel_built（O：production compile_loading 同构）
compile_built(built)              // O
```

> 最小实现（§3）把 `with_main_prelude` + `build_world` 换成 M2–M4；**不能**把“未接场景”作为返回 `Err` 的理由继续保留。

### 5.4 错误路径

| 条件 | 行为 | 依据 |
|---|---|---|
| `dims` 任一为 0 | 原版 panic（`assertion failed: world_render_dims[0] > 0 && world_render_dims[1] > 0`）；生产可保持 panic | O `render_world.rs:442-447` |
| 缺少必需 World 资源 | `Err(rhapsody::Error::invalid_usage(...))`，由各 consumer 返回 | O `world_graph.rs:34-40`、`world_postprocess.rs:59` |
| `build_frame` 构建闭包 Err | `Drop for CoreGraphFrame` 取消声明；返回 Err | O `render_bridge.rs:692-708` |
| `prewarm` 失败/超时 | `cancel_built` 后返回 Err | O `production_renderer.rs:313-319` |
| `compile_frame` OOM/失败 | rhapsody 走 `on_retire`/`abandon_frame` 并 Err；`render()` 触发降分辨率重试 | O `compiled-state/findings.md`、`vulkan.rs:2296-2350` |
| 导出资源不存在/非纹理 | `execute_main_graph` 记录错误，不 present | O `render_bridge.rs:297-367`；`production_renderer.rs:359-367` |

**失败语义**：`compile_world_graph` 返回 Ok 只表示“真实图已声明、资源已物理分配、导出已登记”。禁止返回“空图 + 假 output”或 defer 到 execute 才补图的实现。

### 5.5 与 loading 图路径（0x1401647b0）的差异

| 维度 | loading（`compile_loading_graph`） | world（`compile_world_graph`） |
|---|---|---|
| swapchain | **必需**（extent/format） | 可选；不用于尺寸 |
| 尺寸 | `swapchain.extent` | `min_extent` 或 `extent_10` |
| 格式 | 固定 `0x2b`（`loading_graph.rs:5-7`） | color `0x7a` + depth `0x82` |
| prelude | 无 GpuWorld import / LUT / reset indirect | `with_main_prelude` 全部执行 |
| 内容 | Vermeer → MEI → loading draw list（`loading_graph.rs:169-259`） | W1–W54 + 后处理/UI |
| 导出 | `"loading screen"` target，access `0x22`（`loading_graph.rs:258`） | 最终图，access `0x22`（I） |
| 失败重试 | 无 | OOM → `min_extent=[32,32]` |
| 返回类型 | `CompiledGameGraph` | 同左（无独立 `CompiledLoadingGraph` 类型） |

---

## 6. rhapsody / host 接口核对（只列已存在）

### 6.1 rhapsody 公开 API（文件:行）

| API | 位置 |
|---|---|
| `RenderGraph::new` | `crates/rhapsody/src/rg/graph.rs:258` |
| `RenderGraph::create_raw_resource(name, ResourceOrigin)` | `graph.rs:262` |
| `RenderGraph::create_resource_handle` | `graph.rs:285` |
| `RenderGraph::resource_handle(index)` | `graph.rs:310` |
| `RenderGraph::add_pass(name)` | `graph.rs:328` |
| `RenderGraph::export(handle, access_code)` | `graph.rs:362` |
| `RenderGraph::batch_per_pass` | `graph.rs:391` |
| `RenderGraph::on_retire` | `graph.rs:394` |
| `RenderGraph::build` | `graph.rs:402` |
| `BuiltGraph::into_parts` | `graph.rs:426` |
| `PassBuilder::add_resource_access` | `rg/pass_builder.rs:39` |
| `PassBuilder::render` / `render_with_context` / `render_with_result_context` | `pass_builder.rs:380/384/396` |
| `RenderGraph::create_texture` / `texture_id` | `rg/resource.rs:64/92` |
| `VulkanGraphBackend::new` | `rg/vulkan.rs:615` |
| `VulkanGraphBackend::declare` | `vulkan.rs:1015` |
| `VulkanGraphBackend::resource_clone` / `resource` | `vulkan.rs:1066/3098` |
| `VulkanGraphSession::new` | `vulkan.rs:1894` |
| `VulkanGraphSession::begin_frame` | `vulkan.rs:2177` |
| `VulkanGraphSession::cancel_frame` | `vulkan.rs:2221` |
| `VulkanGraphSession::compile_frame` | `vulkan.rs:2296` |
| `VulkanGraphSession::execute_frame` | `vulkan.rs:2346` |
| `VulkanGraphSession::cancel_prepared` | `vulkan.rs:2351` |
| `VulkanGraphSession::execute_prepared` | `vulkan.rs:2633` |
| `VulkanGraphSession::named_texture` / `named_buffer` | `vulkan.rs:2369/2399` |
| `VulkanGraphSession::temporal_texture` | `vulkan.rs:2430` |
| `VulkanGraphSession::reset_temporal` / `reset_all_temporals` | `vulkan.rs:2515/2546` |
| `VulkanGraphSession::clear_idle_resource_pool` | `vulkan.rs:2530` |
| `VulkanGraphSession::collect_completed_frames` / `wait_submits_done` | `vulkan.rs:2746/2164` |
| `VulkanGraphSession::last_known_frame_duration_ns` | `vulkan.rs:2155` |
| `VulkanGraphSession::attach_presentation_submission` | `vulkan.rs:603/612` |
| `VulkanGraphSession::import_frame_texture(_owned)` / `import_frame_buffer` | `vulkan.rs:2010/2020/2000` |
| `PreparedSessionFrame::snapshot` | `vulkan.rs:1853` |
| `SessionFrameReceipt` | `vulkan.rs:1829` |
| `TemporalRenderGraphRecorder::get_or_create_temporal` | `rg/temporal.rs:317` |
| `TemporalRenderGraphRecorder::get_or_create_temporal_pair` | `temporal.rs:473` — **`#[cfg(test)]`** |
| `TemporalRenderGraphRecorder::export_temporal` | `temporal.rs:587` |
| `NamedTexture` / `TemporalTexturePair` | `temporal.rs:15/26` |
| `RenderPassApi::new` | `rg/pass_api.rs:345` |
| `RenderPassApi::fill_buffer` / `copy_buffer` / `clear_color_image` | `pass_api.rs:465/450/490` |
| `RenderPassApi::dispatch_groups_3d` / `dispatch_indirect` | `pass_api.rs:603/626` |
| `PassContext`（`command`/`managed`/`compiled`/`owners`/`errors`；`history_valid`） | `rg/executor.rs:386/398` |
| `ManagedPassResources::texture_for` / `buffer_for` | `rg/pass_bindings.rs:204/181` |
| `AccessTable::original`（含 0x22/0x34/0x37 原始策略） | `rg/vulkan.rs:127`；表值 `raw/access_table.rs:227/335/351` |

### 6.2 country-core / tiny-glade 生产适配（已存在）

| API | 位置 |
|---|---|
| `RgResources` 5 方法 | `crates/country-core/src/render/pass_builder/rg_resources.rs:64-87` |
| `clear_rg_buffer` / `clear_rg_color_image` | `crates/country-core/src/render/rg_helpers.rs:118/128` |
| `CoreRenderBridge::build_frame/compile_built/execute_compiled/execute_presentation_compiled/cancel_built/exported_texture/take_exported_texture` | `crates/tiny-glade/src/render_bridge.rs:214/249/260/274/291/297/334` |
| `CoreGraphFrame::with_main_prelude/import_gpu_world/export_gpu_world/with_compute/with_raster/with_graph/import_texture/import_native_texture(_owned)` | `render_bridge.rs:510/470/489/425/566/580/658/…` |
| `GraphPasses::create_raw_texture/add_pass/add_compute` | `crates/tiny-glade/src/render_host.rs:1519/1665/2244` |
| `PassApiHost::clear_color_image` | `render_host.rs:2005` |
| `CoreComputeFrame` 的 `PassFrame`/`RgResources`/`RenderGraph` 实现 | `crates/tiny-glade/src/compute_frame.rs:106/133/163` |
| `warm_up_gameplay_with_world_builder`（现无调用者） | `render_bridge.rs:887` |

### 6.3 接口缺口（不补造，登记）

1. **无 `rg::temporal::get_or_create_temporal_pair` 生产入口**：该函数 `#[cfg(test)]`；生产必须走 `VulkanGraphSession::temporal_texture`（`vulkan.rs:2430`）或 `RgResources::get_or_create_temporal_texture_pair`（`rg_resources.rs:76`）。
2. **无 `copy_image`/blit 命令面**：`RenderPassApi` 只有 `copy_buffer`/`fill_buffer`/`clear_color_image`；最终呈现是 shader pass `final_post::resolve_to_swapchain`（`draw_passes/final_post.rs:346`），不是 `vkCmdBlitImage`。
3. **无图内资源 → 物理 `vk::Image` 的公开直取**：pass 回调里只能经 `PassContext.managed.texture_for(logical)`（`pass_bindings.rs:204`）。
4. **无 `RenderGraph::import_*`**：外部/原生纹理导入必须经 session（`import_frame_texture*`，`vulkan.rs:2010/2020`）。
5. `render_bridge.rs:883-884` 注释称“complete compile_main graph”，但当前 `world_graph::build_world` 只覆盖 W1–W54，**不含**闭包尾部的 grading/blur/final_post/UI 段；接入 `warm_up_gameplay_with_world_builder` 前必须补齐或显式缩小语义。

---

## 7. “其他缺失”逐项复核（本轮实查）

| 项 | 结论 | 证据 |
|---|---|---|
| `render_loop::clear_buffer` | **非缺口**：PDB 无该过程；等价逻辑是 `rg_helpers::clear_rg_buffer`（0x1409ddbb0）与 `clear_rg_color_image`（0x1409dde90） | `systems/render_loop/mod.rs:10-14,117-130`；`render/rg_helpers.rs:118-136`；`RENDER_LOOP_B-AUDIT.md §3` |
| `lut_gallery::startup` | **真缺口**：只有 `STARTUP_SHADERS` 常量，无加载函数；plugin 不注册系统 | `lut_gallery.rs:207-217`；`RENDER_LOOP_B-AUDIT.md §6.6` |
| `compute_water::water_level_at_xz` | **存在但是占位**：签名在，恒返回 `0.0`；依赖 `utils::float_grid` 与 `WorldRaster<512>` 读取（typed gap） | `compute_passes/compute_water.rs:438-446` |
| `blur_exclusion_mask` | **真缺口**：无同名、无地址、无等价独立函数；属 `compute_path_masks` 内部 W10 子步骤（高斯模糊排除掩码） | `RENDER_LOOP_A-AUDIT.md §4/§6`；`evidence/country-core-deep/chapters/.../compute_path_masks.rs.md:16,29` |
| `logo_overlay::startup` | **命名误报**：原 `LogoOverlayPlugin::build`（0x1408a5f40，8 B）函数体仅 `ret`，不存在 startup 行为；`render_logo_overlay` 已实现 | `draw_passes/logo_overlay.rs:1-40` |

---

## 8. ② 按顺序实现清单（≤10 步）

1. **最小可运行世界帧**（第一步，必须先做）：按 §3 实现 `compile_world_graph`：`main color`(0x7a, mip6) + `clear_rg_color_image`(access 0x34) + `export 0x22` + `prewarm` + `compile_built`。验收：`render()` 在 world 模式完成 execute→acquire→presentation→present，无 panic；loading 路径回归不变。
2. **接入原版 prelude**：在构建闭包里用 `frame.with_main_prelude(world, |frame, world| ...)` 包住场景；验收 `GpuWorld` import/export、`GpuLutTracker.render_luts`、`reset_indirect_args` 均有真实调用且顺序为 O 序。核实 `primary_rt_debug` 条件（U）。
3. **世界目标资源**：在 prelude 内创建 `"main color"`（0x7a，mip 6）与 `"main depth"`（0x82）；把二者传给 `world_graph::build_world(dims, depth, color, controls)`；dims 取 `min_extent` 或 `FrameRenderConfig.extent_10`。
4. **接通 W1–W34**：直接调用现有 `scene_compute`/`world_terrain`/`world_vegetation`/`world_geometry`/`world_walls`/`scene_raster` 包装（§4 表 #9–#41）。此时应能跑通几何/深度，验证 HZB 与两轮 draw-list 分支。
5. **接通 W35–W43**：`world_lighting::build_world_lighting`（含 `begin_depth_history` 的 `reprojection.prev_depth` temporal、TAA/TGSR 历史对）。
6. **接通 W44–W46**：`world_surface::build_world_surface`（`refraction_copy` + 水/冰分支 + `pre_taa_transparency`），demo 门控用 `GameConfig.dev.demo`。
7. **接通 W47–W54 + 导出**：`world_postprocess::build_world_postprocess`；将最终输出（暂以 `taa` 作为 export 目标，**I**；全量后改 UI/final_post 目标）`export 0x22` 并作为 `output`。补 `freeze_occlusion_culling` 的生产读取（0x143634da0 对应的进程状态源）。
8. **预热链闭合**：把 `warm_up_gameplay`（`production_renderer.rs:37`）改为调用 `render_bridge.rs:887 warm_up_gameplay_with_world_builder`，并先补齐闭包尾部（grading/blur/final_post/UI）或把该函数语义缩到 W1–W54 并改注释/名称，禁止“带缺口却宣称完整”。
9. **降级路径验证**：验证 `min_extent=[32,32]` 下第 1–7 步资源尺寸/历史尺寸（TAA/TGSR）一致；确保 OOM 失败仍返回 Err 以触发原版 `recompile_at_reduced_scale`。
10. **剩余缺口登记/实现**：`blur_exclusion_mask`（W10 内）、`lut_gallery::startup`、`water_level_at_xz` 真实采样；`execution_host.rs` 去重（删除或改绑同一 session，禁止第二 session）。`clear_buffer`/`logo_overlay::startup` 不需要实现。

---

## 9. 验收门（②交付时必须逐条给证据）

1. `compile_world_graph` 不再包含 `world_consumer_missing()`；`production_renderer.rs:44` 的文案只允许出现在真实的、可复现的缺失资源错误中。
2. 每次 world 帧的构图/编译/执行都发生在 `CoreRenderBridge` 的同一个 `VulkanGraphSession` 上（可用 session `frame_ledger()`/`frame_import_count()` 或日志证明）。
3. `CompiledGameGraph.output` 能在 `SessionFrameReceipt.snapshot.exports` 中找到，且 `take_exported_texture` 成功返回 `GpuResource::Image`（真实物理分配，非逻辑号）。
4. 展示图 `resolve_to_swapchain` 的 source 就是该导出纹理（不是合成占位）。
5. W1–W54 的接入顺序与 `frame-composition.json` 一致；条件分支（c0d8/c098/tgsr/dof/is_winter）有测试或运行证据。
6. 最小帧与全量帧都保留 `min_extent` 降级路径；失败路径不产生未取消的 `BuiltGraph`/`PreparedSessionFrame`。

---

## 10. 证据索引

| 主题 | 路径 |
|---|---|
| 主图闭包顺序 / W1–W54 VA | `evidence/render-recovery/producer-closure/c1-country-core/frame-composition.json` |
| 外框 R1–R20 / W 表 / 条件 | `evidence/render-passes/cpu-frame/sequence.md` |
| compile 0x140161cd0 反编译 | `evidence/render-architecture/0x140161cd0.c`（render_world 调用 :835；main color :348/:376；main depth :694/:768；coc :907） |
| render_world 反编译 | `evidence/render-architecture/0x1404ec110.c` |
| 编译/批次机制 | `evidence/render-recovery/producer-closure/a2-graph-closure/findings.md` |
| CompiledRenderGraph 布局/record_batch | `evidence/render-recovery/parallel/compiled-state/findings.md` |
| render_loop 审计（缺口清单） | `evidence/audit-2026-10-19/RENDER_LOOP_B-AUDIT.md`；`RENDER_LOOP_A-AUDIT.md` |
| 名称实读 | `python scripts/rne-read.py 0x14295d6a8` → `"main color"`；`0x14295d798` → `"main depth"`；`0x14295d8a8` → `"coc"` |
| mip_levels builder | `crates/rhapsody/src/raw/texture.rs:400`；`0x140bd5460` 机器码 `mov [rdx+0x1c], r8b` |
| access 策略 | `crates/rhapsody/src/raw/access_table.rs:227(0x22)/335(0x34)/351(0x37)` |
