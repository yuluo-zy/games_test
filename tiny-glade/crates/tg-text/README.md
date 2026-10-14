# tg-text

字体角色、文本编辑/整形、字形 atlas/cache 与 GPU primitive 数据生成，复用原 cosmic-text fork 和 etagere 通用算法。

## 功能与目录

src/font_table.rs、shape_stream.rs、color.rs；src/fragment.rs（TextFragment）；src/renderer.rs（TextRenderer/RenderBatch）；src/gpu.rs 与 examples/text_gpu.rs（可选独立离屏适配）。

## 来源

- 恢复源码：[shared-foundation/tg-text](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-text/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/tg-text/`。
- 原 B 包/PDB 归属：`crates/tg-text/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`、`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/tg-text-full-2026-10-13/REPORT.md)；[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/REPORT.md)；[BUSINESS_RULES.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/tg-text/BUSINESS_RULES.md)。

## 已完成

此前遗漏的 TextFragment/TextRenderer CPU 候选已迁入：字体装载、编辑动作队列、prepare/render、glyph cache/atlas、48B primitive、consume 与上传数据协议。已包含 BufferEnd、line-height bounds、rect/opacity 与 replace/take 的后续原码修正和全部本地测试 fixture。

2026-10-14 内部收口：TextFragment 持有原共享 Arc<FontTable>，构造和 setter 按角色查表；基准 metrics、实际 buffer metrics 与 AttrsOwned 分开保存。原 set_family 不写回构造角色，后续 set_metrics 仍查原角色，此行为按原入口验证保留。frame、失效 stamp、片段生成/验证帧均使用 u32；恢复原无符号 <、wrapping 加法、usage slot/gen、零哨兵、顺序验证前缀副作用和本次回收集合。render 不再隐式 prepare/强制左对齐/清 redraw，consume 在 CPU glyph 上传处理后推进帧。证据及门禁见 [内部收口报告](../../../reverse-engineering/tiny-glade/evidence/text-internal-2026-10-14/REPORT.md)。早期适配说明仅是历史范围。

内部 API 现为 `TextFragment::new(metrics, role, Arc<FontTable>)`、`set_family(role)`、`set_metrics(metrics)`；`prepare(renderer)` 与 `render(renderer, center, extent, color)` 分开调用。移除旧字符串字体、每次显式传表和左上角 render 适配。`FontMetrics` 是两 f32 原状态的恢复命名，作者类型名未取回；仅实现已证的 `From<f32>`（两分量均为 scalar），不推断 tuple From。consume 返回的 CPU payload 包含 glyph_uploads，consume_with 返回 `(closure_result, glyph_uploads)`，保存原内部上传/帧推进顺序；这些数据载体不代表 native GPU 已提交。

## 尚差的功能与验收

本轮指定的字体角色/共享表与帧编号缓存规则已按原代码恢复；没有使用 u64 epoch 或自增 allocation counter 替代原状态。公开 AtlasDescriptor/PendingGlyphUpload 的旧 generation 数据字段仅承载 u32 stamp 的无损扩宽，不参与内部缓存判断。

TextRenderer::new 的真实 rhapsody Texture/全局设备所有权仍未接入；render 的 &AtlasDescriptor 仍是原 &Texture 的 CPU 数据边界。真实调用方/toolbar/SSBO/GPU 接线按用户要求不处理。gpu 特性为独立离屏适配，不能当原 toolbar 调度已恢复；本轮只更新本库测试和示例以编译新 API，没有运行 GPU。A/C、全平台 NaN 位模式、完整原进程和未覆盖的字体/渲染路径仍未验收。

在 workspace 根执行 `cargo test -p tg-text --all-targets` 验证 CPU 路径；`cargo check -p tg-text --features gpu --all-targets` 检查离屏适配。
