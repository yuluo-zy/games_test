# tg-text 行为候选实现

提供 FontTable/ShapeStream、TextFragment 编辑与排版、TextRenderer 字体/字形缓存/atlas、48 字节 Primitive 与 TextRenderStream。可选 gpu 模块使用 rhapsody 资源、批量上传与发行 SPIR-V；独立离屏验收不等同于 toolbar 已接线。

通用整形、编辑、fallback、栅格化直接使用原 PounceLight cosmic-text 0.11.2 commit 3da2a14ea3d063875fdc55695431e74405302b0f，区域分配直接使用 etagere 0.2.10。serde/RON、sys-locale、Unicode grapheme 与 bytemuck 同样使用原锁依赖；不还原通用库。

独立 CPU 流程：load_font_from_file → TextFragment::with_font_table/new → set_text/queued editor_action → prepare/render → upload_pending_glyphs + atlas → consume。消费时推进缓存帧；consume_with 可借用数据后清空并复用容器容量。返回的上传数据由消费者保留至上传成功，失败时可重试。

原字节/PDB 范围证明默认 1024 atlas、scale 清理、延迟 overflow 倍增、上一帧以前分配回收、编辑动作队列、take/replace 所有权与 primitive 的 glyph stream=0/solid stream=FFFFFFFF。回收后的 allocation 序号额外保证陈旧引用不会因 etagere 同 ID 重用而通过验证。

原 opacity 包装提供 rect_with_opacity，原center/extent包装提供 render_in_box；另保留 packed-color/左上角独立适配。initfont defaults 与 locale/fallback 直接复用原fontdb/Cosmic fork，不加载未要求的系统字体。Component/Resource直接由原Bevy0.16候选提供；具体分叉repo/commit未知，不能宣称原 Rust ABI、源码或分叉ECS 1:1等价。selection rect 是独立 Editor 适配能力，不冒称原 toolbar 选区输出已验收。

CPU回归包含原6项协议与新增11项字体/编辑/role scale/cache/atlas/原20组rect与88组opacity边界差分、居中box、fallback、ECS、独立GPU恢复测试。原51shape/21metrics数值证据复用。rect harness初始缺少VCRUNTIME memcpy桥接失败，修正导入后真实原指令20组成功；两次失败证据保留。最终测试/Clippy/GPU运行以本轮根集成报告为准，原游戏、A/C与toolbar入口未执行。

证据入口：[完整候选业务](../../../../evidence/tg-text-full-2026-10-13/REPORT.md)、[原 CPU 数值证据](../../../../evidence/small-libs-batch-01/tg-text/BUSINESS_RULES.md)、[历史 schema 勘误](../../../../evidence/small-libs-review-2026-10-13/integration/ERRATA.md)。

独立RTL验收额外加载C:/Windows/Fonts/arial.ttf，仅用于Hebrew非zero glyph和视觉顺序验证；它不计入原发行字体或初始化策略。具体font SHA见本轮RTL证据。
