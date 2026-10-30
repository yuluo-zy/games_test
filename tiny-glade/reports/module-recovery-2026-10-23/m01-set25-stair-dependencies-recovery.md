# M01 slot3 楼梯请求与插值依赖核验

2026-10-27。范围限定 `generate_playermade_stair_requests<false>` 原140276e80，以及 `playermade_stairs_interpolate_and_compare` 原1407cf500。原B实际目录已读取，当前对应既有 stairs.rs / stairs_assemble.rs。未修改同文件由hints负责的maintain_stair_graph、height负责的两cull，也未扩楼梯装配、支撑工具或共享宿主注册。

原发行RNE+PDB身份复用完整校验；本轮新取30178B请求、8622B插值与1658B父六系统构造器的全指令/source/inline包，status全ok。请求codeSHA `daaf6d8ceba8096d195a35d8559203c238a625f62e9fa41d5e396a004f12fab5`，插值 `741d1435167b9c633ff02c1119500f9f486fa30981fa86f0c1b1e4cd2af3e313`，与此前只读identity-bound导出的完整范围一致。

当前共享Ghidra并不提供完整oracle：请求body17749B、C被120000字符截断，插值body6144B，均小于完整PDB范围。本轮保存此坏CFG审计，不用缺失的反编译推断业务已无分支。交叉依据使用原完整发行指令、原稳定B-seed只读导出的未改raw C、对应符号/参数及本轮有界执行；此前phase2 HANDOFF只是证据来源，其旧“空体”进度描述不作为当前实现结论。

## 请求生产器核验

现实现真实读取Storage定位bucket/id，required CachedTransforms先于stair extra；从权威StairsState收集所有节点，不限本轮modified。按真实墙法线和权威墙空间生成有序alignment候选，完整递归枚举邻接组合、按方向冲突扣10，严格score>替换，保NaN与原顺序；第二遍有向图旗位更新先收集后统一写入。

消耗权威无向图的副本，沿front/back扩路径，遇wall端停止并保闭环；双同墙走真实wall perimeter路径，其余走真实natural cubic spline，保端点tangent、周期环、source ID排序与逆序几何。真实CurveTry失败不伪造成功：只在整条path均失败时补首边短请求，孤点从未被消耗的权威原图枚举。短请求保存源节点非单位quaternion、负零、原中心地形探测、真实两端锚点和失败先后；不是简化成默认请求。

`false` specialization不接管旧dirty集合，`true`才读取旧dirty；两者保固定一次邻接脏传播，未递归扩大到全连通图。alignments/adjusted_edges移出清理后写回，真实GraphMap clear保allocation而不是new默认图。owned Vec/Spline与原Bump allocator的保留方式不同，此工程所有权差异保持透明，不声称allocator完全等价。

源代码已包含上述算法，无须为了“有修改量”重写为另一套实现。本轮54项现存及视觉关联测试重新运行，0失败/忽略，覆盖真实World的open/closed/同墙失败/重复control/孤点/false与true/门控，多数底层几何、失败顺序、NaN、计数溢出也实际执行。

原短请求 `14027a89d UCOMISS; 14027a8a1 JBE` 对unordered会清接触位，第二点14027a90c/910相同。因此源码 `sample+.1 > height` 与NaN→false正确；旧反编译写成 `if sample<=height ... else set` 会误导成NaN→true。核验保留正确源码并保现NaN测试，未照反编译错误“修复”。

## 插值与历史

原六资源接口完整对应Requests/GameTime/VisualState/RemovedSupports/Storage/Signifiers。现实现按time_continuous factor9插值，先take旧sampled map、增量发布sub_segments，保独立旧段表。逐request取首节点metadata四次optional lookup，保原顺序与raw bit；按原graph edge identity移出旧样本，旧Curve按XZ重采样，输出Curve按XYZ校验，两者不可合并。

同数直接读取旧样本，不同数按本轮u到旧XZ坐标采样；先要求当前position，再要求tangent，再读旧下标，缺向量保持原bounds失败。切线先修正方向再插值/有限正倒数归一化，运动阈值比较插值前delta；metadata/width/全局laziness/真实visual force-rebuild共同决定reuse。reuse保四个旧历史Vec及dirty原值；fresh先写history，再CurveTry，失败仍保history和dirty；本轮消失的旧identity由旧表drop，不清独立subsegments/arch map。

本轮将逐点算术定点提取为inline私有 `interpolate_stair_sample`，生产loop仍调用它。明确保原SSE y+x点积、平方长度、z²+(y²+x²)比较顺序，避免未来替换成不同归约函数。无新分配或动态派发。

新probe真实执行发行 `1407cfe5a..1407cff72`，不调用/替代任何业务callee，128输入包含±0、零/反向切线、factor0/.5/1及确定性随机有限向量。Rust逐位比较位置、切线5个word和moved bit，128全部相同。第一次沙箱内Unicorn JIT退出132，获准在本地非沙箱重跑后成功；该区别不包装成被测算法失败或GPU运行。

历史测试组最终11 passed，0失败/忽略，0.01s。整体stairs tests54 passed，0失败/忽略，0.06s；54含本轮6个节点visual测试，不能与其重复相加。过程中其它owner并发测试引入serde_json缺依赖与System.run返回unit错误，已报告owner修复，保首次失败日志，最终重跑通过。

## 验收界限与经验

此闭包证明请求与插值的原参数/算法及CPU历史消费者，没有独立证明全set25生产挂载或楼梯砖模型装配。stairs_assemble仍明确存在其他标U函数，不能因为请求有interpolated curve就声称支撑、砖、结构订阅全完成。原slot3的正确父顺序和仍在其它owner恢复的成员见 `m01-set25-slots1-3-audit.md`。

本轮使用同target、CARGO_INCREMENTAL=0、native-physics实际CPU数据结构；没有OS输入、没有GPU验收。源码与原范围哈希、最终命令/数量、native向量及原日志都在相邻validation/nativepack。

经验：恢复注册先追配置搬运，unknown slot可能是整条六元链；ctor名字必须从原名字字符串和typed metadata共同确认；同名cache在原树出现多次不得去重；unordered浮点分支以UCOMISS/Jcc为准，不照C伪码重写；已有复杂实现先独立核验，不为增加修改量另造算法；为关键数值核补真原指令对照，并用真实历史World测试承接消费语义。
