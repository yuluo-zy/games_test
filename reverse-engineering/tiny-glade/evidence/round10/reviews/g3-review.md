# R10 G3 复核（中央直接审查，2026-10-19）

审查对象：`contracts/g3-compile-tail-CONTRACT.md`、`deliveries/g3-compile-tail-DELIVERY.md`、
`crates/tiny-glade/src/{world_postprocess,render_bridge,production_renderer}.rs` 的尾部改动。

## 结论：**通过（范围限定）**，水分 0

通过的是“W1–W54 + 已实现尾部子集”，**不是完整 compile_main**。交付对此的表述诚实。

## 证据

1. **顺序真实**：`world_postprocess.rs:244 WORLD_TAIL_STEPS` 22 项，`build_world_tail:618` 按
   flattened(CPU) → bidirectional_blur_pyramid → final_post(msaa) → menu/painterly/vermeer/logo →
   wireframe/ui/mei/cursor/loading → 调用方 export 的顺序声明；有 `tail_steps_keep_native_order` 测试。
2. **MSAA 目标真实**：`declare_native_image`（`render_bridge.rs:665`）用原生
   `rhapsody::raw::texture::TextureDesc` 声明 `"msaa color out"`，format 0x2b、`sample_count=4`、
   display extent；避免 core 占位 `TextureDesc` 无 sample_count 被静默降为 1。
3. **导出修正**：`production_renderer.rs` 的 `build_full_world_stage` 改为导出 `tail.msaa`
   （0x22），不再导出 `out.taa`；present 端按 sample_count=4 选择
   `raster_resolve_msaa_to_swapchain`，有 `msaa_target_desc_selects_msaa_resolve` 测试。
4. **未恢复项的诚实处理**：`warn_deferred_tail_steps` 一次性告警；session/LUT/反馈/ui_2d/egui
   等为显式 U 控制，置真时返回命名硬错误；painterly/logo/cursor 条件为 U。没有静默 no-op 冒充执行。
5. **warm-up 共用**：`warm_up_gameplay` 与 `compile_world_graph` 共用 `build_full_world_stage`，
   文档明确“不等于完整 compile_main”。

## 仍缺 / 继承缺口

1. T11 session 预览、T12 LUT 画廊、T13 反馈背景、T16 `ui_2d_pass`、T21 egui：Host/组件视图未恢复，
   默认 U=off；生产帧会缺这些 UI 元素。
2. gameplay App 未注册 MEI text flush；`load_glade_grading` 未恢复（tail 会提前返回带名硬错误，
   不伪造图）。
3. `bVar43`、menu flag、painterly aux、cursor `+0xf4` 等条件为 U。
4. GPU 真机仍未验证（项目后置）；本轮只证明图声明/编译/导出身份。

## 编译与测试（G3② 记录 + 中央抽查）

- `cargo check -p country-core`、`-p tiny-glade`：Finished。
- `cargo test -p country-core --lib`：2509 passed / 0 failed。
- `cargo test -p tiny-glade --lib`：37 passed / 0 failed。
- 中央抽查：`production_renderer.rs` 导出为 `tail.msaa`；`world_postprocess.rs` 的顺序表与
  MSAA desc 真实存在；未发现伪造 rhapsody API。

## 给下一轮的方向

1. 补 T11/T12/T13/T16/T21 的 Host 与组件视图，使 U 控制可置真。
2. 恢复 `load_glade_grading` 并在 gameplay App 注册 MEI text flush。
3. 把 compile_main 尾部与原版逐 pass 做一次顺序/资源差分，最终判定 warm-up 等价性。
