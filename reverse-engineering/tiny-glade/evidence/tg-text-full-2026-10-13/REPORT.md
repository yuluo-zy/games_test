# tg-text 完整 CPU 候选开发

已实现字体系统/真实文件、TextFragment 的所有权/文本/metrics/family/weight/width/anchor/editor queue、prepare/bounds、Cosmic layout→physical glyph→Swash image→etagere atlas→48B primitive、缓存与过期回收、scale失效、overflow延迟倍增、render/stream/consume。GPU adapter 由根集成 owner 开发与独立GPU验收；真实toolbar不在本次接线范围。

## 原证据和修正

module624/1185/1782 的 PDB 过程范围、patched原字节SHA、专属text-full-20261013 Ghidra checkpoint反编译见同目录jobs/results/模块txt。默认atlas 1024方形；RGBA8格式0x2b；上传时overflow尺寸*2并清缓存/pending；get_glyph先回收last_frame+1<frame再重试。scale变更清空缓存/atlas/pending。

原 consume 推进frame，原 set_text 同文本也重置cursor/selection并排队BufferStart，原 take_editor 留下相同metrics的空Editor。已据此修正候选。

原 generate_primitives closure2 的+32 u64 0xffffffff证明glyphcolorFFFFFFFF/stream0；读取142cecb60 constants证明solid cursor streamFFFFFFFF、uvextent1/1。原cursor宽4 logical。原 LayoutGlyph::physical调用与baseline分开缩放/截断；候选已修正。

原rect第四参数是 xmm3 float opacity、调用游戏utils::scale_u32_packed_srgba_color，当前独立公开API直接接packedcolor。不冒称该adapter保持原接口ABI。原anchor常量-0.5/-0.5已读，完整参数语义尚未核实。为避无界宽度RTL溢出，独立排版固定左起、render进行anchor；这是显式候选策略。

## 验证

独立same-source workspace原锁 --offline --locked测试通过既有6项+新增5项，strictClippy通过。真实字体Ubuntu/NotoSansSC/NotoColorEmoji直接来自发行assets；编辑动作延迟/序列、metrics角色scale、ownership、warm cache不重复raster/upload、scale再生成、overflow倍增后恢复、atlas回收和stale ref拒绝通过。详见cargo-test.log/cargo-clippy.log。

回收测试发现etagere某些同slot复用相同AllocId，仅contains不足；候选增加allocation序号与glyph记录比较，复用原allocator，不手写allocator。

新rect原指令harness两次exit3221225477，均不计通过，保留脚本和失败JSON。此轮新原证据为静态原字节/PDB/反编译；旧72组原数值证据未重复运行。

## 仍需区分

完整独立GPU证据由root集成提供；原字体初始化/fallback完整顺序、metrics极端输入、完整anchor/opacity策略、原serde运行、ECS分叉注册/生命周期、A/C、toolbar未验收。普通Bevy0.16不能当已知原分叉等价。不存在成功stub，也不能将当前可用候选升级为完整原行为验收。

## 后续闭合（本节覆盖上文初轮缺口/失败结论）

见 [CONTINUATION.md](CONTINUATION.md)。已补原opacity/centerextent包装、font initialization/fork fallback策略、ECS候选和GPU cache-reset恢复。新增原20rect+88opacity指令成功；初始rect失败仅诊断历史。代码已freeze，最终16项候选测试/strictClippy/GPU由root集成门禁重新执行。
