# R11 rendering owner recovery (2026-10-19)

基线：`origin/main` / HEAD `1253c0ab`。本轮没有改动 `crates/rhapsody`；将原 round10 的 W1–W54 / 4×MSAA tail 移植到当前 public API / per-App owners / SessionFrameReceipt，不恢复旧底层实现。

## 本轮真实接线

- `production_renderer.rs` 与 `render_bridge.rs` 三方冲突逐块合并，继续复用当前 per-App owners、warm-up named resources、图取消/receipt/texture take 生命周期。`TextureDesc` 改用当前公开的 `rhapsody::resource` API。
- T21：ProductionRenderHost 的真实 EguiBackend 与当前 App 的 EguiGpu 通过参数进入主图；T10 UI 门允许时，在 T20 后、0x22 export 前向同一 `msaa color out` 绘制。不存在 ECS 中伪造的 egui owner，也没有 default-false 绕过；worker warm-up 没有 backend。
- T12：移除 U/off 开关。真实 EscMenuState/可选 preview 槽/当前 ImageFormationConfigs.name/GradingLuts.metas 进入实际 DrawPassCtx → RasterPass → DrawBuilder，使用原 cell_uniforms、zip 截断、highlight/current/first 判断、全部 uniform、原 raster state 和每格六顶点。preview 必须属于当前 App TextureHost，缺 owner 失败；原 preview None 则按原生规则早退。LutName 修为 String，并按 `grading_lut:` 前缀求资产名。root 同轮安装真实 gallery Resource/shader、LUT 与 menu/active_lut 纹理。
- T16：移除 U/off 开关。真实 ECS screen-space 六元组与 world-space 二元组进入现有原算法，保留 optional icon/interaction/order/inactive、稳定排序、三 shader、全部 uniform/绘制分支。UiInteractionState 提升到 canonical country-core 类型，toolbar 改 re-export，不制造第二 TypeId。UiInactive / Ui2dWsExtent / CustomizationIcon 的 read 面按新原发行证据修复。
- 修复原恢复中被丢弃的 UI 图片 array-layer view：core FromDisk 将显式 TextureViewDesc 传至 PassHost，生产 host 生成/缓存真实 Vulkan view，descriptor/owner 保留到提交，release 清理。未知 host 对显式 view 报错，不回退到默认 view。未改 rhapsody 实现。
- T13：移除 U/off 开关。原 EscMenuState tag 8 门内读取 FeedbackReportBg Resource 投影并调用已有原 pass；visible/page/resource texture/state/三角形规则保持。capture/state producer 仍缺，只有进入 ReportBug 后命名失败。
- T11：移除 U/off 开关。按原 EscMenuState `state - 2 < 9 && state - 2 != 4` 门求值。原本应进入 session-preview 的菜单状态现在明确失败，不能静默当作已画完；Closed/其它原早退分支正常早退。
- MEI：主图/worker/OOM retry 只检查并复用 Update44 的一次 text_flush 输出，不再次消费 TextRenderer。root 同轮补真实 gameplay text producers。
- Meadow 必经 placement 缓冲：frame_setup 每帧从当前 App 的 AssetPlacementDataLibrary.buffers 快照实际 handle/buffer/count，验证 BufferHost owner 后交给已有 placement draw consumer；不再留下始终空表，也不替换算法/数据。

## 新原证（O）

`native-gallery-identity.json`：实际读取原发行 RNE 的本地身份，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。所有本目录 asm 均由 LLVM objdump 对该文件指定 VA 范围读取，不来自 mutable patched 数据库。

- gallery-name.asm：0x1408b0b86 取 config kind、左移 6；0x1408b0b90 读 config +8 的 16 字节（String ptr/len）交 Display。不能将两字误建模成 u128。
- 原格式模板 0x142ae6ca8 → 指针 0x142ae6c98 / len 12，实际内容 `grading_lut:`。
- ui-inactive-registration.asm：0x14084e3c9 align 1，0x14084e3d4 size 0。
- ui-ws-registration.asm：0x14084f1a9 align 4，0x14084f1b4 size 0x14。
- customization-icon-registration.asm：PDB procs.tsv 定位 register_component<CustomizationIcon> @0x1422b58f0；0x1422b5b87 align 4，0x1422b5b92 size 0x24。旧“组件为0xa0”混淆了菜单内部记录。
- ui-2d-consumer.asm：0x1409c4c43 读 icon +0x18 alpha；0x1409c4c8f 读 +0x21 byte；0x1409c50a8 读 interaction +2；0x1409c4d44/60/82 构造 view_type(1)/base_layer(image.layer)/layer_count(Some(1))，0x1409c4de6 实际传给 TextureFromDisk 绑定。
- session-preview-gate.asm / feedback-gates.asm：两者先读同一 EscMenuState TypeId `d38dae617fb79c7a:22aadf5c5dad1eae`，不是此前注释的 AppMode。T13 0x1408a5b0a 比较 tag 8，0x1408a5b7e 比较 visible@+0x44 == 1，0x1408a5b89 比较 page@+0x3e < 7。
- session-carousel-metrics.asm：读 Arc+0x14..+0x44，与 canonical LoadMenuThumbnailMetrics 逐偏移吻合（见下一项 I）。

沿用原证：round10 g3 contract/delivery/review、render-architecture/0x140161cd0.c 的 T20→T21→export 顺序，以及 country-core-deep 的原 consumer 反编译。T21 不是新声称动态观测。

## 推断与剩余边界

- I：旧 CarouselState 是 LoadMenuThumbnailMetrics 的投影：Arc+0x14 highlighted_index、+0x1c origin、+0x24 size、+0x2c step.x、+0x34 scroll_offset.x、+0x40 cells_per_line（原 assert ==1）、+0x44 column_major（原 assert true）。LoadMenuState.hovered_index 的 tag/value 对应 selection state/selected slot。尚未把这项转换变成完整生产 SessionPreviewHost。
- T11 consumer 和实际交互均未完整：缺 ThumbnailTextures 实际 startup/每帧 refresh/图片 decode/upload/filetime/error_once 的 host；pool 纯算法和测试已经在正式 crate，不应再新造候选算法。SavedSession 原 +0x18 black-and-white 字节尚未与当前 screenshot_exists/loading_background_exists 语义绑定，不能凭名称瞎接。已检 `reconstruction/country-core-render/wp19`（只有 Gap）、`reconstruction/country-core` 与当前 crate（同一抽象 host）；未找到被遗漏的生产 GPU host。
- T12 consumer 已接；实际可触发交互仍需 LUT preview capture 生产者与 GUI metrics writer。None 的早退是原版可选分支，不代表画廊交互验收。
- T16 consumer 各分支已接；CustomizationIcon 除原读取字段外仍为 opaque，color-menu/icon-animation 生产者的恢复状态仍由其 owner 跟进，不能因为 draw consumer 编译就宣称完整颜色菜单。
- T13 consumer 已接；反馈捕获资源/页面状态生产者仍缺，ReportBug 活跃时明确失败。旧反馈模块排除不用于悄悄消掉本轮 T13。
- T21 owner/排序/目标接线完成，实际 GPU framebuffer/pipeline 尚未在本轮验证。
- 继承 U：painterly 辅助输入/部分控制字节、bVar43/LUT选择、menu background owner、截图关联等。仍不宣称完整 compile_main 或 warm-up 等价。

## 验证

- 首轮兼容修复后 `cargo check -p tiny-glade --lib --offline` 通过（round11-render-check.log）；该次先于新增 T12/T16/view 接线，仅代表当时状态。
- 后续统一编译/测试由 root 集成 owner 执行，结果以其最终 integrated-check / host/core tests 日志为准，不将前述旧通过外推到最终版。
- 新增/加强有效回归：LUT UTF-8 stem、UI query 对 toolbar canonical TypeId 与全部 optional 字段/WS 的投影、现有 UI pass stable order+三 shader+5 draw 额外验证每次实际 TextureFromDisk 绑定均保留 layer 2 的 descriptor。
- ReVa 尝试读取 /tiny-glade.exe @0x1400ddc80 返回 Program not found / no open programs（isError=true）；已记录失败，不据此报告新反编译成功。随后使用原发行 PE 限定字节/LLVM disassembly 取证。
