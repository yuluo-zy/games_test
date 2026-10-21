# compute 完整业务源码独立复核（lead）

结论：通过原10808B计算器及两个真实game callbacks本轮源码范围，依赖原freeze和3个独立addendum/errata。已读完整增量源码与七项真实World输入测试，独立重放原生产before和已审node before两个patch，F0退出0、字节完全等于20386293。未编译/运行，未改生产。完整请求、插值、cache、mesh和canonical共享float差异不在本次接受范围。

- 真实输入按两端点元数据/实际StairPoint/两AlignKind/caller positions与raw dirs/有序IDs/owned Curve分组，不按无向key排序几何端点；原有current/previous不复制第二权威。None override是唯一默认算法入口，Some(empty)保持实际空；owned Vec直接move，未统一补长度。
- 默认tangent保raw normalize/零段NaN，不用Curve稳健normalized；width按真实points_u做u*W1+(1-u)*W0；VisualTerrain双侧采样按连续.5*.5、-.025/+.1完整原序，在端点dir normalize_or_zero覆盖前执行。原callback带NaN correction，故Rust f32::max在这里有原证，不与无修正MAXSS混淆。真实marker为TerrainHeightsData<VisualTerrainHeightSsbo>，测试同时插不同physics marker，避免错owner碰巧通过。
- None subsegments逐相邻原输入分类，NaN按已独立纠错为kind1；kind1/2保行且u16计数回绕，kind3不增count、替换末完整行（point/tangent/width/height/bit），仍更新原输入previous。全部后真实Curve重建，不只换points；BitVec借用by_vals避免owned iterator隐式缩容，SmallVec依原inline1复用。
- start后end padding按真实tag/bit门，两个SubSegment bool顺序相反、callerXZ减真实width*first，实际Curve再次重建、rawdirs写入、start mask前插false/end追加true；16组双端有效bool域测试已逐项核。dirty只用canonical两u32 StairGraphEdge成员查询，不把packed-u64另一个Hash输入替代，也不常true。
- 输出auxiliary Vec empty与OptionCurve None属于本原producer明确初始化，不作为省略复杂业务借口；其后真实消费者仍独立未完成。没有默认成功或错误吞掉。

测试API已按pin Bevy0.16/glam0.29.3/bitvec1.0.1与实际terrain height资源核对；七新增用真实World storage/cache/graph→collect/derive→compute，包含不同visual/physics格、短tangent失败、原u分配、指针移动、65536段计数、NaN/零段、kind3替换/新u及padding/dirty。它们尚未执行，Rustfmt不能替类型检查。

真实共享缺口继续阻止“全输入原等价”结论：canonical Curve构造器当前f64极小/极大量fallback与提前finite检查异于原纯f32和失败顺序；本源没有私造第二Curve或epsilon补偿。须root在其原owner协调后修共享函数或明确实际输入域验证。ROOT_CARGO_HUNK仅系统crate新增exact bitvec1.0.1，root统一Cargo整合；同源node/owner2a046da8以及foreign outline消费者需要原子配对。

冻结source after `20386293aa3a2a0121a75dd236b381d3dc0422b3b0356f5210100147c5a553da`；production before `87435b0f6c78d5695d7f1d89a2f8768585d7a62a34ab26a60d4e175ece0427c6`。source review通过不改变semantic_acceptance/runtime状态。
