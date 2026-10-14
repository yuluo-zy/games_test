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

2026-10-14 续轮恢复 render getter 的借用 atlas + 最近 consume 图元数合同；consume 在回调前保存图元数，回调 unwind 保留 pending 流且不推进 frame。metrics 变化按原作移动 lines 到 Buffer::new_empty 后重新创建 Editor，保留文本/属性/动作队列并重置 cursor、selection、scroll、尺寸和编辑设置。scale 失效保留已请求的 atlas 增长；glyph upload 仅扩展 alpha mask，Color/SubpixelMask 的 RGBA bytes 原样保留。证据及新边界见 [续轮报告](../../../reverse-engineering/tiny-glade/evidence/text-followup-2026-10-14/REPORT.md)。

## 尚差的功能与验收

TextRenderer::new 的 CPU 初始化和原 native atlas 装配已取静态证据；真实 rhapsody Texture/全局设备所有权仍未接入 CPU renderer。render 返回 (&AtlasDescriptor, u64)，descriptor 是原 &Texture 的 CPU 适配，不能作为原 native 句柄 ABI。原 set_family 接受角色字节并通过 Arc<FontTable> 查找，候选 set_family(String)/set_font_role 是适配入口，原角色状态/共享表合同仍待恢复。

原 atlas invalidation stamp/frame 为 u32 帧编号，候选采用 u64 generation/frame 并对同帧重置额外失效；原 rollover 与同帧顺序尚未等价验收。真实 toolbar/SSBO/GPU consumer 和 A/C 未验收。gpu 特性为独立离屏适配，依赖 rhapsody，不能当作原 toolbar 异步调度已恢复。NaN 位模式及 cosmic/etagere 逐字节 fork 等价保留边界。

在 workspace 根执行 `cargo test -p tg-text --all-targets` 验证 CPU 路径；`cargo check -p tg-text --features gpu --all-targets` 检查离屏适配。
