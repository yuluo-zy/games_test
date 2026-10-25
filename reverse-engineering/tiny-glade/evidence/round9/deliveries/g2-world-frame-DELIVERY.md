# G2 世界帧（world graph）闭环交付 — 2026-10-19

**复核修订（同轮）**：
- ②改：删除未接线的 `ProductionFramePasses` adapter（见 §3）；minimal frame 改为默认硬错误、
  仅显式诊断/demo 允许（见 §1.2、§2）。
- ③改：`main color` 补原生 `mip_levels=6`（P1）；`warm_up_gameplay` 接
  `warm_up_gameplay_with_world_builder`（P2）；world extent 消费 `world_resolution_scale`、
  `min_extent` 改为覆盖语义（P3）；删除过期 `world_consumer_missing` 文案（P5）。
  P2 的**闭包尾部缺口**列为阻断级继承缺口（§5.1）。

范围：`crates/country-core/src/systems/render_loop/**`、`crates/country-core/src/render/execution_host.rs`、
`crates/tiny-glade/src/production_renderer.rs`（`compile_world_graph`、warm-up 与必要 helper）、
`crates/tiny-glade/src/game_loop.rs`（仅 gameplay 发布/warm-up 接线；中央授权）。
未改 G1 文件（`systems/save/**`、`systems/wall|roof/**`、`resources/{walls,roofs}/**`、`rhapsody/**`）。
`run_game.rs` 未改：`GameplayPreparation` 的 warm-up 调用面已满足本接线。

## 1. 实际接线

### 1.1 `compile_world_graph`（`production_renderer.rs:602`）

1. `world_extent`（:110）：
   - `min_extent = Some(v)` → **直接覆盖**（契约 §5.1 M1：`min_extent.unwrap_or(world_dims)`，
     不是逐轴 `max`）。
   - 否则 `world_dims = base(window) × world_resolution_scale`；`scale` 读
     `GameConfig.video.world_resolution_scale`（`set_resolution_scale` 更新的正是这里），
     回退 `FrameRenderConfig.world_resolution_scale`。窗口基础尺寸优先 `WindowSize`，回退
     `extent_10`/swapchain。
   - 依据：`prepare_frame_constants.rs:205` 把 `extent_10` 当 `inner_rendering_resolution`，
     `:214` 用 `extent_10/extent_8`；但本恢复的 `FrameRenderConfig::at_startup`
     （render_config.rs:49-54）只拷贝窗口尺寸、未应用 scale（I），因此在消费点补上。
     这样 OOM 的 `recompile_at_reduced_scale` 才会真正缩小世界尺寸。
2. `missing_world_producer`（:158）检查 `GpuWorld`/`GpuLutTracker`/`FrameConstants`/`MainCamera`/
   `ActiveSession`/`DeferredRasterResources`/`FrameRenderConfig`：
   - 齐全 → `compile_full_world_graph`（:228）：`CoreRenderBridge::build_frame` →
     `with_main_prelude`（GpuWorld import → FrameConstants/LUT → reset_indirect，即 B 0x140161fbc..1622fd）
     → `world_targets`（:182）创建 `main depth`（`world_frame::main_depth_desc`，fmt 0x82，mip 1）与
     `main color`（`world_frame::main_color_desc`，fmt 0x7a，**mip_levels 6**）→
     `world_graph::build_world`（W1..W54，既有生产消费者）→ 导出 `out.taa` access 0x22 →
     prewarm → `compile_built`。
   - 缺任一 → **默认返回硬错误**，错误文本带缺失 producer 名，并提示
     `TG_ALLOW_MINIMAL_WORLD_FRAME=1`/demo；只有显式开关或 `GameConfig.dev.demo == true`
     时才走 `compile_minimal_world_graph`（:309，并写 warn 日志）。默认行为 = 不把空世界当成功。
3. 任一分支都返回 `CompiledGameGraph { graph, output }`，由既有
   `execute_main_graph`/`execute_presentation_graph` 提交与 present。

### 1.2 gameplay 预热（P2）

- `warm_up_gameplay`（production_renderer.rs:45）不再固定 Err，改为
  `render_bridge::warm_up_gameplay_with_world_builder(owners, world, builder)`；
  builder = `with_main_prelude → world_extent → world_targets → build_world → export TAA(0x22)`。
- `game_loop.rs:305` 在把 gameplay SubApp 移到 worker **之前**调用
  `renderer.bridge().warmup_owners()`，把 gameplay App 的同一组 HostTables/RasterShared/shaders
  交给 worker；发布路径 `game_loop.rs:695-700` 的 `activate_application` 会用
  `CompletedGameplayWarmup`（render_bridge.rs:887 内插入）复核 Arc 身份并接回命名资源。
- warm-up 只有在图真的 compile/execute/export 后才返回 Ok；失败会让 `GameplayPreparation::poll`
  返回 Err，不会发布假 ready App。
- **阻断级继承缺口**：`warm_up_gameplay_with_world_builder` 的文档要求“完整 compile_main 图
  （含 final22 export）”，目前 builder 只闭合 W1..W54 + TAA 导出，未包含原生闭包尾部的
  grading/blur/final_post/UI 段。该缺口已写入 `evidence/round9/COORDINATION.md` 台账；
  在补齐尾部前不得宣称 warm-up 等价于完整 compile_main。

旧 `world_consumer_missing()`（“FramePasses consumer is not connected”）已删除（P5）。

## 2. 最小可运行帧包含哪些 pass（仅显式诊断/demo）

`country-core::systems::render_loop::world_frame::build_minimal_world_frame`：

1. `create_raw_resource("main color", world_frame::main_color_desc(extent))`
   = `new_2d(extent, 0x7a)` + `mip_levels(6)`（O：0x140bd5460 `mov [rdx+0x1c], r8b`）+ color
   attachment + sampled；名称 O：0x14295d6a8 len 10；
2. `clear_rg_color_image`（0x1409dde90）= pass `"Clear image"`，access 0x34，
   回调 `clear_color_image(img)`；真实 transfer pass，无 shader；
3. `RenderGraph::export(id, 0x22)`。

`main color` 的 `mip_levels=6` 是 P1 修正点：W50 `generate_mips` 直接把 `desc.mip_levels`
作为 SPD mip 数并断言 `<= 13`、format=0x7a；旧代码误把原生 `+0x1c=6` 当作 usage，
会生成 mips=0/1 的纹理。`main_depth_desc` 显式 `mip_levels(1)`（原生 new_2d 的 +0x18=1）。

**选择条件**：`compile_world_graph` 只在 producer 缺失且
`TG_ALLOW_MINIMAL_WORLD_FRAME=1` 或 `GameConfig.dev.demo` 时调用；否则返回带缺失 producer 名的
`rhapsody::Error::unsupported`。最小帧不会掩盖正常（非 demo）运行时的资源缺口。

最终 blit 不在 world graph：既有 `production_resolve::build` → `final_post::resolve_to_swapchain`
（`production_resolve.rs:54`）已经在展示图里执行。最小帧因此是
**create + clear + export + 既有 resolve**；不绘制背景/场景内容，日志明确标注这是显式最小闭包，
不冒充 render_world 成功。

## 3. 世界帧的真实生产路径；`FramePasses` adapter 处置

**真实生产路径 = typed consumers**：`compile_world_graph` → `world_graph::build_world`（W1..W54，
`world_graph.rs:42`），它调用 `scene_compute`/`scene_raster`/`world_terrain`/`world_vegetation`/
`world_geometry`/`world_walls`/`world_lighting`/`world_surface`/`world_postprocess` 的已恢复消费者。
`render_world.rs` 的 63-seam `FramePasses` 仅由同文件测试 `Rec` 实现。

**复核结论**：上一版在 `world_frame.rs` 里的 `ProductionFramePasses` 是死代码——全 workspace
除自身与测试外无实例化/调用点，且 63 seam 中 56 个默认 `unsupported`。现已**删除**：
- 删除 `WorldFrameHost`、`ProductionFramePasses`、`render_world_frame` 及对应测试；
- `world_frame.rs` 只保留 `build_minimal_world_frame`（显式最小帧）、`WorldFrameError` 与
  `WORLD_FRAME_SEAMS` 名字表（报告/未来 seam host 用，不声称已实现）。

因此本组完成度**不包含** seam-level `FramePasses` 实现；world 帧通过 `build_world` 真实构图/执行。
若将来要求 seam 级路径，可新增 host 把 `scene_compute`/`scene_raster` 逐项转发，但那需要在
主路径中显式替换 `build_world`，不能只放一个未被调用的 adapter。

### 63 seam 现状

| 组 | seam | 状态 |
|---|---|---|
| 全部 63 | `WORLD_FRAME_SEAMS`（world_frame.rs） | 仅名字表；真实实现位于 `world_graph::build_world` 的 typed consumer 链，**不经过** `FramePasses` trait |

## 4. PassHost / ComputeExec

- `Painterly PassHost` 生产实现已存在：`crates/tiny-glade/src/effects_host.rs:177`
  （`submit_compute_impl`/`submit_raster_impl` 落到真实 ComputePass/RasterPass/DrawBuilder）。
- `ComputeExec` 生产实现已存在：`crates/tiny-glade/src/render_host.rs:2210`（真实 `cmd_dispatch`/
  `cmd_dispatch_indirect`）。
- `PassFrame` 生产实现已存在：`crates/tiny-glade/src/compute_frame.rs:106`。
- 本轮未新增这三者的第二份实现；`painterly_pass` 仍未接入任何系统（见 §5）。

## 5. 仍缺项

1. **阻断级继承缺口**：`warm_up_gameplay` 已接 `warm_up_gameplay_with_world_builder`，但 builder
   只覆盖 W1..W54 + TAA 导出，**未包含**原生 compile_main 闭包尾部的 grading/blur/final_post/UI。
   `render_bridge.rs:883-884` 的“complete compile_main graph”文档因此暂时不成立；在补齐尾部前
   不得把 warm-up 视为完整 compile_main（已写入 round9 中央台账）。
2. seam-level `FramePasses` host **未实现**（adapter 已删除，不计完成度）。生产路径是
   `world_graph::build_world` typed consumer；若后续要求 seam 级路径，需要新增 host 并在主路径
   显式替换 `build_world`，而不是放一个未调用的 adapter。
3. `painterly_pass`/`startup_painterly` 仍无生产调用；`PainterlyState` 未插入 gameplay App。
4. 进程 debug 全局无 producer：`*0x14362c0d8`、`*0x14362c080`、`*0x143634d50` 以加载时 0 使用，
   已注释；接入 debug 控制时需替换 `build_full_world_stage` 的 controls。
5. `main color`/`main depth` 名称由字符串长度 10 推断（I；契约记录 rne-read 实读为
   `"main color"`/`"main depth"`，可视为 O）。`mip_levels=6` 已按 0x140bd5460 修正；
   旧稿“usage 6”表述错误，已删除。usage 位仍由 `sampled()+color_attachment()` 与 rhapsody
   访问计算共同满足（I）。
6. **P3 证据缺口（I）**：原生“把 world_resolution_scale 应用到 extent_10”的更新函数尚未定位；
   当前在 `world_extent` 消费 `GameConfig.video.world_resolution_scale`，使 OOM 重试实际生效。
   取证点：`prepare_frame_constants.rs:205,214`（extent_10 为 inner resolution 且参与
   global_mip_bias）、`render_config.rs:49-54`（startup 未应用 scale）、契约 §5.1 M1
   （min_extent 为覆盖）。取得原生更新点后应改为唯一来源。
7. `world_graph::build_world` 与 `render_world.rs` 的顺序/分支尚未做逐条差分（既有审计风险，未变）。
8. 截图 GPU 纹理关联仍未恢复（production_renderer.rs 既有 `unsupported`）。

## 6. `execution_host.rs` 处置

**保留，但标注为非生产路径**（`crates/country-core/src/render/execution_host.rs` 模块文档）。
workspace 零调用者；生产唯一会话是 `tiny-glade::CoreRenderBridge`（同一种 `VulkanGraphSession`，
另绑定 App 的 Buffer/Texture/Shader/Raster 宿主）。保留原因：它记录了
`BuiltGraph → SessionFrameReceipt` 与 `pass_api_from_context` 的 rhapsody 边界形状，可作核对材料；
新增接线不得创建第二条会话。

## 7. rhapsody API 使用清单（文件:行）

本轮新增调用：

- `crates/tiny-glade/src/production_renderer.rs:49` `warm_up_gameplay_with_world_builder`
  （warm-up 主图；`game_loop.rs:305` 提供 `warmup_owners`）
- `:53` warm-up `with_main_prelude`；`:195-196` warm-up/main `main depth`/`main color` 创建
- `:234` `CoreRenderBridge::build_frame`（主世界图）
- `:235` `CoreGraphFrame::with_main_prelude`（GpuWorld import / FrameConstants / LUT / reset_indirect）
- `:220-222` `RenderGraph::resource_handle` + `export(0x22)`（最终 TAA 输出）
- `:239,:245` `ShaderHost::prewarm` + `CoreRenderBridge::compile_built`
- `:271` `RgResources::create_raw_texture`（最小帧 main color，含 mip 6）
- `:288` `rg_helpers::clear_rg_color_image`（0x1409dde90，RenderGraph::add_pass，“Clear image”/0x34）
- `:300-302` 最小帧 `resource_handle` + `export`
- `:313,:318,:324` 最小帧 `build_frame` / `prewarm` / `compile_built`

依赖侧（未改，仅使用）：`rhapsody::rg::vulkan::VulkanGraphSession::{compile_frame,execute_prepared}`
（经 `CoreRenderBridge`）、`rhapsody::rg::temporal::{get_or_create_temporal*}`（经
`CoreComputeFrame::RgResources`）。

## 8. 验证

- 真树验证（中央已修复 save 断言与 `duck.rs` 后）：
  - `cargo check -p country-core`：**通过**（Finished dev profile，60s）；
  - `cargo check -p tiny-glade`：**通过**（Finished dev profile，43s）；
  - `cargo test -p country-core --lib world_frame`：**7 passed / 0 failed**
    （含 P1 `main_color_mip_count_matches_generate_mips_input`）；
  - `cargo test -p country-core --lib generate_mips`：**10 passed / 0 failed**（DoF mip 链回归）；
  - `cargo test -p country-core --lib dof`：**23 passed / 0 failed**；
  - `cargo test -p tiny-glade --lib world_extent`：**3 passed / 0 failed**
    （scale 消费、min_extent 覆盖、zero-scale 硬错误）。
- 说明：早期验证曾在仓库外临时副本中绕过两个**非本组**既有错误
  （`save_thread.rs:63` 的 Windows 布局断言、`crates/systems/critters/src/duck.rs` 的
  `device.device()` 类型不匹配）；中央已分别修复（duck 改为 UFCS
  `GraphicsDevice::update_buffer_bytes`），上述结果为真树复跑。临时副本已删除。
- `game_loop` 发布门：publish 仍走 `GameplayPreparation::poll → activate_application`，
  只有 warm-up 成功插入 `CompletedGameplayWarmup` 且 Arc 身份匹配时才接回资源。
