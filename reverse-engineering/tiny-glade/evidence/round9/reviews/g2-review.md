# G2 渲染构图/执行链 — Round 9 ③ Review（含增量复评）

- 审查对象
  - 契约：`evidence/round9/contracts/g2-world-frame-CONTRACT.md`（11:42）
  - 交付：`evidence/round9/deliveries/g2-world-frame-DELIVERY.md`（12:19:04，③ 反馈修订版）
  - 实现快照：`crates/tiny-glade/src/production_renderer.rs`（12:11:24）、
    `crates/tiny-glade/src/game_loop.rs`（12:10:04）、
    `crates/country-core/src/systems/render_loop/world_frame.rs`（12:15:56）、
    `crates/country-core/src/render/execution_host.rs`（11:46:28）
- 审查方式：只读 + 真树编译/测试。工作树未提交（执行组 4 个文件 `M` + `world_frame.rs` 未跟踪）。
- 增量复评时间：12:21–12:24；本轮无临时放宽（中央已在 12:03 修复 `save_thread` 布局、
  12:15 修复 `duck.rs` 类型错误）。

## 结论

**通过（G2 范围）**。水分计数：**0**。
前一轮唯一水分 W1（交付 §5.5 把原版 `+0x1c=6` 误读为 usage）已在本轮修正为
`mip_levels=6`，并新增回归测试；P1/P3/P5 全部修复，P2 接线完成且闭包尾部缺口已按中央要求
登记为**阻断级继承缺口**（不计本轮完成度）。GPU 真机执行仍未验证（项目既定后置）。

遗留阻断级继承项：`warm_up_gameplay` 的 builder 只闭合 W1..W54 + TAA(0x22) 导出，未包含原生
compile_main 闭包尾部的 grading/blur/final_post/UI；在补齐前不得宣称 warm-up 等价完整
compile_main（交付 §5.1、`COORDINATION.md` 已登记）。

## 1. 增量复评（P1/P2/P3/P5）

| 项 | 原问题 | 当前实现 | 独立验证 | 判定 |
|---|---|---|---|---|
| P1 | `main color` 缺 `mip_levels(6)`；W50 `generate_mips` 读到 mips=0（base mip 6 / push 255） | `world_frame.rs:43,49-54` 新增 `MAIN_COLOR_MIP_LEVELS=6` 与 `main_color_desc`（`new_2d(0x7a)` + mip6 + color_attachment + sampled）；full（`production_renderer.rs:193`）与 minimal（`world_frame.rs:142`）共用同一 desc；`main_depth_desc` 显式 mip1（`:58-60`） | `cargo test -p country-core --lib world_frame`：`main_color_mip_count_matches_generate_mips_input` 通过；`cargo test -p country-core --lib generate_mips` 10 passed。核对 `generate_mips.rs:109-153`：mips=6 → `last_view_base_mip=5`、push `5`、`<=13`、format 0x7a 全部成立 | **已修复** |
| P2 | `warm_up_gameplay` 恒 Err；默认 `skip_splash=false` 时 gameplay 永不发布，`compile_world_graph` 不可达 | `production_renderer.rs:45-68` 改为 `warm_up_gameplay_with_world_builder(owners, world, builder)`，builder = `with_main_prelude → world_extent → world_targets → build_world → export TAA(0x22)`；`game_loop.rs:305` 在移动 SubApp 前 `warmup_owners()`，`:309-311` 传给 worker；发布仍经 `GameplayPreparation::poll`（run_game.rs:919-940）→ `activate_application`（game_loop.rs:693-698）→ `CompletedGameplayWarmup` Arc 身份校验（render_bridge.rs:123-136、production_renderer.rs:430/452） | 静态核对：`poll` 只在 worker 的 `warm_up` 返回 Ok 后才返回 `true`（run_game.rs:893/908/925），失败显式 `Err`，不再“卡加载/假 ready”；`warmup_owners` 捕获的 HostTables/RasterShared/shaders 与实际 gameplay ApplicationGpu 为同一组 Arc，切换 loading App 时该 ApplicationGpu 保存在 `other_applications`（production_renderer.rs:438-450 既有逻辑）。GPU 运行时成功未验证 | **已接线**；闭包尾部缺口**登记为阻断级继承项** |
| P3 | `min_extent` 用 `max` 且 `extent_10` 静态，OOM 降分辨率重试实际不缩小 | `world_extent`（production_renderer.rs:110-153）：`min_extent=Some` 直接覆盖；否则 `WindowSize × GameConfig.video.world_resolution_scale`，回退 `FrameRenderConfig.world_resolution_scale`/`extent_10`/swapchain；零尺寸命名错误 | `cargo test -p tiny-glade --lib world_extent`：3 passed（0.5 → 640×360、min_extent 覆盖、0.0 → Err）。`set_resolution_scale`（production_renderer.rs:629-631）写入的 GameConfig 字段正是被消费项 | **已修复**（原生 scale 更新点仍为 I，交付 §5.6 已登记取证点） |
| P5 | `world_consumer_missing` 文案仍称 FramePasses consumer 未接 | 全 workspace grep 无 `world_consumer_missing`/旧文案 | 编译通过 | **已修复** |

补充：`warm_up_gameplay` 不再返回 `Err`，`compile_world_graph` 在默认 `skip_splash=false`
流程的**静态链路可达**——发布门现在要求 warm-up 成功；若 warm-up 因缺少依赖资源失败，启动会
显式报错而不是挂起。运行时能否成功取决于完整 producer 链与 GPU 执行，本轮未验证。

## 2. 水分检查（逐条 文件:行）

**计数 0**。四类硬水分逐项：

- 静默 no-op：无。未恢复项一律 `Result`/命名错误；minimal 帧是真实
  `create + Clear image(0x34) + export(0x22)`；缺失 producer 默认硬错误
  （production_renderer.rs:602-625，`TG_ALLOW_MINIMAL_WORLD_FRAME=1`/demo 才允许 minimal）。
- 测试 double 冒充生产：无。`world_frame.rs` 测试 host 仅测试用；生产 host 是
  `production_renderer.rs:252-307` 的 `MinimalWorldHost`，落到真实 `CoreGraphFrame`/rhapsody。
- 返回 Ok 但未执行：无。warm-up 只有实际 compile/execute/export 后才插入
  `CompletedGameplayWarmup`（render_bridge.rs:901-917）；minimal/full 均 `?` 传播失败。
- 伪造 rhapsody API：无（§6 逐项对照）。

前轮 W1（`main color` “usage 6”）已删除并改为 `mip_levels=6` 证据（交付 §5.5），不再计水分。

## 3. 链路真实性（最终树）

```
game_loop.rs:139  render(&mut host, pacer, mode)
└─ country-core render_frame.rs:258 render()
   └─ :282 host.compile_world_graph(...)
      └─ production_renderer.rs:602 ProductionRenderHost::compile_world_graph
         ├─ world_extent (:110)                    min_extent 覆盖 / WindowSize×scale
         ├─ missing_world_producer (:158)          7 个 producer
         ├─ [齐全] compile_full_world_graph (:228)
         │  └─ CoreRenderBridge::build_frame             render_bridge.rs:214
         │     └─ with_main_prelude                      render_bridge.rs:510
         │        └─ build_full_world_stage              production_renderer.rs:201
         │           ├─ world_targets (main depth/color, mip6)  :182
         │           ├─ world_graph::build_world         world_graph.rs:42 (W1..W54)
         │           └─ resource_handle + export(0x22)   :220-224
         ├─ [缺 producer 且显式开关/demo] compile_minimal_world_graph (:309)
         │  └─ build_minimal_world_frame                 world_frame.rs:133
         │     ├─ create_color_target -> create_raw_texture (mip6)
         │     ├─ clear_rg_color_image                    rg_helpers.rs:128 (access 0x34)
         │     └─ export(0x22)
         └─ prewarm + compile_built (:239/:245) -> VulkanGraphSession::compile_frame (vulkan.rs:2296)
   └─ :659 execute_main_graph -> execute_compiled (render_bridge.rs:260)
      -> VulkanGraphSession::execute_prepared (vulkan.rs:2633)
      -> take_exported_texture (render_bridge.rs:334)
   └─ :668 acquire_swapchain_image -> :682 execute_presentation_graph
      -> production_resolve -> final_post::resolve_to_swapchain (import main source 0x22)
   └─ :713 present
```

预热链（默认 `skip_splash=false`）：

```
game_loop.rs:305 warmup_owners() -> :307 GameplayPreparation::spawn
run_game.rs:884..893 gameplay.update 循环 -> LoadingGameComplete -> warm_up(world)
production_renderer.rs:45 warm_up_gameplay(owners, world)
  -> render_bridge.rs:887 warm_up_gameplay_with_world_builder
     -> build_frame -> with_main_prelude -> build_world -> export 0x22
     -> prewarm -> compile_built -> execute_compiled -> exported_texture
     -> complete_gameplay_warmup -> world.insert(CompletedGameplayWarmup)
game_loop.rs:693 poll==true -> :697 activate_application -> accept_gameplay_warmup
```

`ExecutionChain/execute_graph`（execution_host.rs:45-51）仍是非生产边界材料；真实执行点仍是
`execute_prepared`，与契约一致。

## 4. 最小帧

- 可 `Ok(Compiled)` 并提交：**是**。`main color` = `new_2d(extent,0x7a)+mip6+color_attachment+sampled`；
  `clear_rg_color_image` = pass `"Clear image"`、access `0x34`、回调解析物理 image 后
  `vkCmdClearColorImage`；`export(0x22)`；随后既有 execute/acquire/resolve/present。
- world graph 内实际 pass 仅有 `Clear image`（transfer，无 shader）；展示图另有既有
  `resolve_to_swapchain`。
- 选择条件：默认硬错误；`TG_ALLOW_MINIMAL_WORLD_FRAME=1` 或 `GameConfig.dev.demo` 才启用
  （production_renderer.rs:85-96、:484-500）。不构成静默降级。

## 5. 63 seam 三分类

最终代码**没有** seam-level 生产适配器；`world_frame.rs:151-236` 仅名字表。

| 分类 | 数量 | 证据 |
|---|---|---|
| 经 `FramePasses` 的真实生产实现 | **0** | `ProductionFramePasses`/`WorldFrameHost` 已删除（world_frame.rs:3-12）；唯一 `impl FramePasses` 是 render_world.rs:686 测试 `Rec` |
| 明确报错 | **0（seam 级）**；边界硬错误 | 缺 producer 时 production_renderer.rs:616-621 返回带名字的 `unsupported`；`build_world` 缺资源返回 `invalid_usage` |
| 静默 no-op | **0** | 无默认 `()` 实现可被调用；minimal host 3 个操作均 `Result` |
| 名字表 / 未实现 | **63** | `WORLD_FRAME_SEAMS` 63 项，顺序与 `render_world` 一致 |

生产等效覆盖（typed，不经 `FramePasses`）：契约 §4 映射为 **62 真实现 + 1 部分**
（#18 `compute_path_masks`，W10 的 `blur_exclusion_mask` 仍缺；全 workspace 无该符号）。
该状态不得折算为“63 seam 已实现”。

## 6. rhapsody API 核对

| 使用点 | 真实签名/位置 | 结论 |
|---|---|---|
| `warm_up_gameplay_with_world_builder` | render_bridge.rs:887 | 真实 |
| `CoreRenderBridge::build_frame` / `compile_built` / `execute_compiled` / `take_exported_texture` | render_bridge.rs:214 / 249 / 260 / 334 | 真实 |
| `with_main_prelude` | render_bridge.rs:510 | 真实 |
| `RgResources::create_raw_texture` | rg_resources.rs:66；实现 render_host.rs:1519 → `VulkanGraphBackend::declare` | 真实 |
| `clear_rg_color_image` | rg_helpers.rs:128；`CoreComputeFrame: RenderGraph` compute_frame.rs:163 → `GraphPasses::add_pass` render_host.rs:1665 → rhapsody graph.rs:328 | 真实 |
| `resource_handle` / `export(0x22)` | graph.rs:310 / :362 | 真实 |
| `prewarm` / `compile_frame` / `execute_prepared` | render_host.rs；vulkan.rs:2296 / :2633 | 真实 |
| `TextureDesc::{new_2d,sampled,color_attachment,mip_levels}` | rhapsody_types.rs:95/48/74/60 | 真实；mip6 已使用 |

物理 usage 由 rhapsody `graph.rs:627-633` 依据 access 计算后覆写，clear(0x34)/export(0x22)
的 TRANSFER_DST/SAMPLED 位由访问表补齐，与交付解释一致。

## 7. 编译与测试（真树，无需放宽）

| 命令 | 结果 |
|---|---|
| `cargo check -p country-core` | **Finished**（0 errors；9 warnings，均 render_loop 既有） |
| `cargo check -p tiny-glade` | **Finished**（0 errors；7 warnings） |
| `cargo test -p country-core --lib world_frame` | **7 passed / 0 failed** |
| `cargo test -p country-core --lib generate_mips` | **10 passed / 0 failed** |
| `cargo test -p tiny-glade --lib world_extent` | **3 passed / 0 failed** |

说明：前一轮的两处非 G2 阻塞已由中央修复（`save_thread.rs` 改用 24B `String` 承载路径、
断言保留；`duck.rs:818` 改为 `GraphicsDevice::update_buffer_bytes(device.device(), &buffer, ...)`），
本轮未做任何临时放宽。`COORDINATION.md` 另记录全套
`cargo test -p country-core --lib` 2467 passed。

## 8. 缺口继承核对

交付 §5 的 8 项均真实，未把未接 pass 标成完成：
1. **阻断级继承缺口**：warm-up builder 缺原生 compile_main 闭包尾部（grading/blur/final_post/UI）；
   已登记于交付 §5.1 与 `COORDINATION.md`。
2. seam-level host 未实现（adapter 已删，不计完成度）。
3. `painterly_pass`/`PainterlyState` 未接（全 workspace 零调用）。
4. debug 全局以加载时 0 使用（`freeze_occlusion_culling=false` 等，注释在案）。
5. `main color`/`main depth` 名称与 mip6 证据已修正；usage 由访问计算补齐（I）。
6. P3 的原生 scale 更新点未定位（I，取证点已列）。
7. `build_world` 与 `render_world.rs` 未逐条差分（既有审计风险）。
8. 截图 GPU 纹理关联未恢复（既有 `unsupported`）。

## 9. 剩余需中央/后续决定的问题（非本轮阻断）

| # | 位置 | 说明 |
|---|---|---|
| R1 | `production_renderer.rs:45-68` + `render_bridge.rs:883-884` | warm-up 闭包尾部缺口；补齐前 warm-up 只能称“W1..W54 首帧”，不能称完整 compile_main。 |
| R2 | GPU 运行时 | 本轮只验证编译/单测；warm-up 与 world 帧的真实 GPU 成功未验证（项目后置）。 |
| R3 | `production_renderer.rs:110-153` + `render_config.rs:49-54` | `WindowSize × scale` 与原生 `extent_10` 更新点可能双乘；待①定位原生更新函数后改为唯一来源。 |
| R4 | `execution_host.rs:34-62` | 保留的非生产 `RhapsodyExecutionHost::new` 仍可创建第二 session；有“禁止”文档，无调用者。 |
| R5 | `world_graph.rs` W10 | `blur_exclusion_mask` 缺失，63 seam 等效覆盖仍为 62 真 + 1 部分。 |

## 10. 给①的下一步取证方向（≤5）

1. **原生 world 分辨率更新点**：定位 `FrameRenderConfig.extent_0/8/10` 在缩放/resize/OOM 重试时的
   更新函数（0x1405a26bd..cd 附近的 scale 存储之后），确认 `extent_10` 是否已含 scale，解决
   R3 的双乘不确定性。
2. **compile_main 闭包尾部**：按 `0x161cd0.c` 补全 grading/blur/final_post/UI 的顺序、
   资源与最终 export 目标，为 warm-up 等价性与 `output` 选择（taa vs UI/final_post）提供证据。
3. **warm-up 闭包 0x1400d7440**：确认它要求的 ready 门与 export（0x22）语义，以及 GameplayPreparation
   的 publish 前是否还要求其它图（painterly/vermeer）。
4. **W10 `blur_exclusion_mask`**：给出原 VA/调用位置、输入输出与与 `compute_path_masks` 的串联，
   闭合 62/63 的最后 partial。
5. **main color/depth desc 字节**：复核 `+0x18/+0x1c/+0x1d/+0x1e` 的逐字节语义（depth/mips/tail），
   确认 `main_depth_desc` 只补 mip1 是否足够。

---

附：审查操作记录
- 前轮（12:00–12:04）曾临时注释 `save_thread.rs:63` 与 `duck.rs:818` 做过验证，均从备份逐字节还原
  （`git status` 当时对二者无修改）；12:03 后中央真正修复了这两处。
- 本轮增量复评（12:21 起）未修改任何源码，未做临时放宽；仅运行 check/test 与只读核对。
