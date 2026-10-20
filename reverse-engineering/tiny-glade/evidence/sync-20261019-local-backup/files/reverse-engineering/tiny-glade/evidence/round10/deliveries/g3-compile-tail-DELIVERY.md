# G3② compile_main 尾部实现交付 — 2026-10-19

范围（文件权内）：
- `crates/tiny-glade/src/world_postprocess.rs`（新增尾部闭包 `build_world_tail`）
- `crates/tiny-glade/src/render_bridge.rs`（`CoreGraphFrame::declare_native_image`；warm-up 文档）
- `crates/tiny-glade/src/production_renderer.rs`（`build_full_world_stage` 接尾部、导出改 MSAA；MEI flush 前置；warm-up 文档/返回描述）
- 未改 `crates/rhapsody/**`、roof/wall、save/history、startup、camera_rig。

依据：`evidence/round10/contracts/g3-compile-tail-CONTRACT.md` 与
`evidence/render-architecture/0x140161cd0.c:1028..1409`（B SHA f507a6ce…）。

## 1. 逐 pass 真实接线（文件:行）

| T | 原生步骤 | 本实现 | 接线 |
|---|---|---|---|
| T0 | `render_world` W1–W54 → `{depth,coc,taa}` | 既有 `world_graph::build_world` | `production_renderer.rs` `build_full_world_stage` |
| T1 | `ColorGradingState::flattened` 0x140b00b20 | `ImageFormationConfig::flattened()`（`resources/image_formation.rs:174`），无 GPU 资源 | `world_postprocess.rs` `build_world_tail` T1 段 |
| T2 | `bidirectional_blur_pyramid` 0x1409ed860 | `scene_compute::with_env` + `blur_pyramid::bidirectional_blur_pyramid(env, taa)`，无条件；校验 0x7a + mip>0 | 同上 T2 段 |
| T3 | `final_post` 0x14090d000 → `"msaa color out"` | `frame.declare_native_image`（新）创建 `new_2d(display,0x2b).sample_count(4).sampled().color_attachment()`；`final_post::final_post` 以 src0=taa、src1=full blur、`ColorAttachment::Tag2(msaa)` 声明 | 同上 T3 段 + `render_bridge.rs::declare_native_image` |
| T5 | `render_menu_bg` | 原生门控 `menu_bg_mode_allows(AppMode, flag)` 先求值；门控拒绝时不触资源（等价跳过）；门控允许时返回命名硬错误（Host/资源未恢复） | `world_postprocess.rs` T5 段 |
| T6 | `painterly_pass` | `enabled` 输入（stack0x1a88）为 U，默认 false（原生此时置 Disabled）；置 true 时返回命名硬错误（PainterlyState + 两个 aux 纹理未恢复） | T6 段 |
| T7 | `vermeer_pass` 0x1408a82f0 | 真实调用：`EffectsHost`（已实现 `VermeerHost`）+ `VermeerState`/`VermeerDebugControl`/`Time`，target=msaa；缺失状态为硬错误 | `world_postprocess.rs::run_vermeer` |
| T8 | `render_logo_overlay` | 条件输入为 U 的显式开关；打开时真实调用 `logo_overlay::render_logo_overlay` | T8 段 |
| T10 | UI 块门控 `(vermeer_active & mode!=0x13)==0` | 按机器码表达式实现 | T10 段 |
| T11–T13 | session 预览 / LUT 画廊 / 反馈背景 | Host/资源未恢复；显式 U 开关，置 true 时命名硬错误 | T11–T13 段 |
| T14 | `wireframe_pass` | `stack0x1ac0` 门控（O：两处写入均为 1），真实调用 `wireframe_pass::wireframe_pass` | UI 段 |
| T15 | `ui_pass` | 同门控，真实调用 `ui_pass::ui_pass`（DrawLists UI 段） | UI 段 |
| T16 | `ui_2d_pass` | `UiInteractionState`/`UiInactive` 组件视图未恢复；显式 U 开关，置 true 时命名硬错误 | UI 段 |
| T17 | `mei_pass` | `LoadingGpuResources.mei.import_into_frame` + 真实 `text_flush_system` 输出（`output()`）+ `"mei temp"` (0x2b,1×) + `ui_2d_pass::mei_pass`；缺输出为硬错误 | `world_postprocess.rs::prepare_mei` |
| T18 | `custom_cursor_pass` | 真实调用；可见性字节 `+0xf4` 为 U，默认 false | UI 段 |
| T20 | `loading_screen_pass` | 真实调用（gameplay 下列表通常为空） | UI 段 |
| T21 | `EguiBackend::draw` | EguiGpu 不在 ECS world 可达（I：原生世界帧为 Some）；显式 U 开关，置 true 时命名硬错误 | UI 段 |
| T24 | export 0x22 | **改为导出 `msaa color out`**（0x2b / 4×MSAA / display extent） | `build_full_world_stage` 末 |
| T25 | `RenderContext::finish` | 无对应对象（句柄随 frame 释放） | — |

T4/T9/T19/T22 截图 opportunity 与 T23 panic capture：原生只登记/读取请求，未见固定 pass；
request→GPU 纹理关联仍未恢复，`world_postprocess.rs` 末尾注释记录，未伪造 pass。

## 2. 与原生顺序的对应

`WORLD_TAIL_STEPS`（`world_postprocess.rs`，22 项）按 `0x140161cd0.c:1085..1409`
调用序排列，并有 `tail_steps_keep_native_order` 测试锁定
`flattened < blur < final_post < menu < painterly < vermeer < ui < mei < loading < export`。
实现路径严格按该顺序声明：grading(CPU) → blur → msaa 目标 → final_post → menu 门控 →
painterly 控制 → vermeer → logo → T10 门控 → UI wireframe/ui/mei/cursor/loading → 调用方 export 0x22。

## 3. 导出 / present 一致性

- `build_full_world_stage` 返回并导出 `tail.msaa.resource`（0x22），不再导出 `out.taa`。
- `execute_main_graph` 取回该导出纹理（`take_exported_texture`），并按
  `CompiledGameGraph.expected` 校验 `format/sample_count`：全量世界路径期望
  `0x2b/4`，minimal 期望 `0x7a/1`，loading 期望 `0x2b/1`。不一致时记录命名错误并拒绝
  把它交给 present（`production_renderer.rs` `execute_main_graph`）。
- `execute_presentation_graph` 把该纹理交给 `production_resolve::build`；
  `production_resolve.rs:27` 从原生 `Texture.desc.sample_count` 读取 4，因此
  `resolve_to_swapchain` 选择 `raster_resolve_msaa_to_swapchain`
  （`final_post.rs:337-344`）。
- 测试 `msaa_target_desc_selects_msaa_resolve` 断言 `msaa_color_desc(...).sample_count == 4`
  且 `resolve_shader_name(4) == RESOLVE_MSAA_SHADER`。
- 说明：core 的 `rhapsody_types::TextureDesc` 没有 `sample_count` 字段，
  `TextureHost::convert` 会固定为 1；因此新增 `CoreGraphFrame::declare_native_image`
  直接以原生 `rhapsody::raw::texture::TextureDesc` 声明该图资源，避免静默降为单采样。

## 4. warm-up 等价性判定

`warm_up_gameplay` 与 `compile_world_graph` 共用 `build_full_world_stage`，因此二者都覆盖
“W1–W54 + 已实现尾部子集”。**判定：不等于完整 compile_main**。理由（按契约 §5 六条）：

1. 顺序：已实现子集与原生顺序一致；但 T11/T12/T13/T16/T21 未声明。
2. 导出身份：满足（msaa 0x2b/4×/display extent，展示图走 MSAA resolve）。
3. 输入：T2/T3 消费同一 W54 `taa`；blur 无条件。
4. 条件：`flag_0x1ac0` 已按 O（两处写 1）；`bVar43`、`menu_bg_flag`、painterly `enabled`、
   logo 条件、session/LUT/feedback 门控、cursor `+0xf4`、egui Option 均为 U，由
   `WorldTailControls` 显式给出，交付不宣称等价。
5. 生产者严格性：`MainCamera`/`GameConfig`/`ImageFormationConfigs`/`AppMode`/WindowSize/
   `final_post` shader/MEI host+输出/VermeerState 缺失均返回带名字的硬错误。
6. 前置/导出顺序：MEI flush 在构图前（`flush_world_mei`）；GpuWorld export 仍在 tail 之后，
   与原生 `:1491` 一致；preset export 由 FrameSetup 既有路径处理。

因此 warm-up 文案标注为 “W1–W54 + 尾部子集”，未使用 “complete compile_main”。

**运行时阻断（诚实失败）**：当前 gameplay App 尚未注册 MEI 文本 flush 生产者，且
`load_glade_grading` 未恢复；`build_world_tail` 会在这两处返回带名字的硬错误
（`MEI text producers ...` / `... load_glade_grading is not recovered`），
不会静默产出缺 UI/LUT 的图。补齐这两个启动生产者后，warm-up 才可能真正执行到尾部。

## 5. minimal / demo 路径

`compile_minimal_world_graph` 未改：仍是 `main color (0x7a,mip6) + Clear image + export 0x22`，
不接尾部。present 会因 `sample_count == 1` 选择非 MSAA resolve。它不代表世界渲染，仅在
`TG_ALLOW_MINIMAL_WORLD_FRAME=1` 或 demo 下显式启用；默认缺生产者仍硬错误。

## 6. 仍缺项 / U

1. **MEI 生产者在 gameplay App 未注册**：`run_game.rs`/`gameplay_startup.rs` 未注册
   `text_flush_system` 与文本片段系统（文件权外）。`flush_world_mei` 现在会调用
   `flush_mei_frame`；若 TextRenderer 为空批次无法产出 atlas，flush 会 panic。
   这是真机 warm-up 的阻断项，需在 App 装配层补注册后复验。
1b. **grading LUT 资产生产者未恢复**：`final_post` 绑定 `grading_lut:<name>`、固定 LUT 键与
   默认 LUT；`GradingLuts::load`/`load_glade_grading`（G1 缺口）无调用者，且
   `initialize_common_assets` 的 `should_load` 白名单不含这些资产。`build_world_tail`
   在声明前用 `require_grading_lut_texture` 检查三处哈希，缺失时返回带
   `load_glade_grading is not recovered` 的硬错误（避免 raster 绑定的 “No such texture” panic）。
2. T11/T12/T13/T16/T21 未实现（Host/组件视图/Egui 所有权）。
3. `bVar43`（stack0x1897）无 producer：LUT 选择落到 `Keep` 分支（U）。
4. `menu_bg_flag`（AppMode+8）无 producer；`menu_bg_mode_allows` 的输入是 I。
5. painterly `enabled`/aux 纹理、logo 条件字节 `settings+0xf3`、cursor `+0xf4` 均 U。
6. `final_post` 的 LUT 自定义纹理（`Option<LutOverride>` 的 `+0x28` 纹理）无 producer，
   `custom_lut = None`。
7. `theme` 以 photomode(AppMode tag 0x13) 映射（I；契约记录资源名为 GladeTheme，值 0x13 与
   photomode 一致）。
8. `arg_f0/arg_f1` 取 `VideoSettings.sharpen/+0x9c` 与 `gamma_shift/+0xa4`（O 偏移，槽位顺序 I）。
9. T4/T9/T19/T22/T23 截图请求关联未恢复。
10. GPU 真机执行/图像对比仍后置（本轮仅编译/单元测试）。

## 7. 编译与测试

- `cargo check -p country-core` → **Finished**（1m47s，仅既有 warnings）。
- `cargo check -p tiny-glade` → **Finished**（1m07s，仅既有 warnings）。
- `cargo test -p country-core --lib` → **2509 passed / 0 failed**（含 G1/G2 同轮修复；
  本组未改 country-core 任何文件）。
- `cargo test -p tiny-glade --lib` → **37 passed / 0 failed**。
- 新增测试：
  - `world_postprocess::tail_tests::tail_steps_keep_native_order`
  - `world_postprocess::tail_tests::msaa_target_desc_selects_msaa_resolve`
- 并发说明：验证期间曾遇到 G2 在 `native-physics` feature 下的 `history.rs` 借用错误
  （`cargo check -p tiny-glade` 失败、`-p country-core` 默认 feature 通过），已由 G2 修复后
  复跑；本组文件权外未做任何修改。

## 8. rhapsody API 使用清单（新增）

- `CoreGraphFrame::declare_native_image` → `VulkanGraphSession.backend.declare(ResourceSpec::Image(native_desc))`
  （`render_bridge.rs`）。
- `RgResources::create_raw_texture`（`"mei temp"`，0x2b）与 `RgTexture` 真实句柄。
- `DrawPassCtx::new` + `PassHost`（`RasterPassHost::sibling`）驱动 `final_post` / logo / UI / MEI / cursor。
- `scene_compute::with_env` + `PassEnv`（blur pyramid 的 compute passes）。
- `EffectsHost`（`PassHost`+`VermeerHost`）驱动 `vermeer_pass`。
- `RenderGraph::export(msaa, 0x22)`（`build_full_world_stage`）。
