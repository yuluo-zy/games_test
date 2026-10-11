# 减少必须恢复的代码：替代路线比较

日期2026-10-11。本轮仅分析，未修改业务代码、依赖、skill或共享计划，未构建、运行oracle、启动新反编译。已读 `NATIVE_RECONSTRUCTION_ACCELERATION_PLAN.md`。本地能力以现有脚本和此前PDB实测为准，外部工具边界参照下文官方资料；成本是相对工程投入，不是已测提速倍数。

## 最佳替代：先减少恢复对象，再改善取数速度

统一索引/常驻worker能减少等待，却未必减少要理解和重写的代码。更优先的组合是：**最终可观察输出约束下的业务切片 → 库/API边界替换 → 闭包和小策略恢复 → schema/组件胶水生成**。只为这一组合所需的函数提取inline和SSA，不先吞下全部内联树，也不先建通用反编译到Rust编译器。

最终目标仍是可独立构建和维护的Rust建筑业务项目，复用依赖、真实source→消费者→编辑→删除→历史链闭合。若运行时继续执行原PE或原指令字节，仅算过渡实现。生成Rust可以满足目标，但必须具有完整业务输入/输出合同，且不靠原binary、捕获的context/draw/proposal或手写库容器才能工作。

三版Rust DBI无LOCAL/REGREL/BPREL/DEFRANGE，普通建筑函数TPI为1001 void()；这排除了“导入PDB局部类型就自动恢复全源码”。丰富的inline可帮助分出library与game，但文件来源不自动提供参数、捕获环境或纯净API边界。依据为 [PDB审计](pdb-capability-audit.md)。

## 路线比较

|路线|实际节省来源|本地工具现状|实施成本|最终独立Rust目标|主要失败风险|最小可证伪试点|
|---|---|---|---|---|---|---|
|1. 可观察输出的业务切片|不再恢复与建筑输出、状态、保存、生命周期无关的分支；把依赖调用封成副作用摘要|有原C/ASM、CPU实例/World输出和已证字段；尚无带副作用的SSA slicer|低成本人工根集合；自动跨过程slice为中高|可满足，前提根集合覆盖整个建筑族及异常/清理|漏RNG推进、资源释放、事件顺序、未来帧状态；仅CPU根会漏GPU/保存|对一条Gate/Fence完整族建立状态与输出根、列删除候选；若任何候选会影响后续帧/输出/删除，则扩大slice，不能直接删除|
|2. 精确库/API替换|整批排除Vec/HashMap/ECS/RNG/math/serde展开体，保留真实调用策略|Cargo/PDB依赖索引、原crate源码缓存、当前std/Bevy/glam/Perchance复用已有|低到中；fork/ABI未知提高成本|最直接满足|同址别名、generic名包含Wall误归属、float顺序与RNG版本错误|一条包含glam+Perchance+Vec的Fence owner；只允许真实库源码/API对应的范围被封闭摘要，无法绑定参数就停止替换该节点|
|3. 从库wrapper提取真实game closure|只恢复capture→callback游戏读写，不翻译iterator/collect/allocator机制|已见141204df0 allocVec wrapper内fold gate callback；已有原callee/native证据，无closure自动提取器|一个bounded closure中成本；通用逃逸/生命周期分析高|可满足；callback语义为Rust，容器用原依赖|wrapper整体删掉会漏游戏行为；捕获环境无types，错误借用/顺序/短路|围绕已有Gate wrapper列capture load/store与调用序列，用标准iterator/Vec表达closure；若capture需猜默认或原输出来自wrapper外共享状态，就保gap|
|4. 序列化/Bundle/SystemParam机械生成|省去字段访问器、offset投影、derive、组件登记、事件封装和重复runtime胶水|已有serializer/字段名、原Bundle/fullgeneric metadata、semantic Rust与runtime样式；无统一生成器|小manifest+模板低；layout推断不算机械成本|可满足，但生成行为候选仍需原证据与后续验收|持久化schema≠内存ABI；属性/default/enum niche漏；Bundle不证明required组件/调度|选一个已证Wall metadata复制+一个真实三字段Nani Bundle；只输入明确字段与组件manifest，生成read/write/derive/register草稿；不能从zero-padding生成字段|
|5. raw/high pcode→field/call事实抽取|机械整理load/store的base+offset、常量、def-use、guard和call输入，减少逐行反编译阅读|Ghidra12.1.4有pcode/HighFunction API；现ExportFocusedFunctions只导C/ASM，不导SSA；本轮未实现/测试|限定ops事实抽取中；跨过程别名中高|辅助满足，事实表不是Rust实现|高pcode错误签名优化后丢/改call输入；phi与指针别名误解释|对同一public-field copy/accessor并导raw与high；已知4bindings必须保来源/宽度，unknown不得转字段类型。无法区分指针origin就仅输出候选|
|6. 有白名单的SSA小策略→Rust|省重复标量分支、位运算、拷贝、枚举政策的手工转写；复用已知schema|尚无pcode→Rust generator；可扩现Java导出再加小模板|一次小策略中成本；每新op/side-effect提高成本|限定范围可满足；整模块不能承诺|float MIN/MAX/NaN、wrapping/shift、alias、CALLIND、niche与panic边界；生成器会把错反编译固化|选已证publication metadatacopy/kind selection；限定固定读写、标量arith、branch、已绑定call，不含allocator/未知指针。遇unknown op显式拒绝；第二个不同owner仍需低人工修补才扩|
|7. Ghidra C→C2Rust|纯C兼容小块可省语法转写；不是新的语义恢复|本地未发现c2rust；Ghidra输出含Rust伪符号、undefined/CONCAT类型和空原型，不是作者C99源码|整理可编译C和shim中高，整族通常高|只在所有业务都真正落Rust且移除binary桥后满足；现阶段不优先|修C胶水比手写语义Rust更贵，UB/alias/ABI延续，库体被重复翻译|只选一个已绑定有限layout的整数政策，将修C耗时与直接Rust样例对照；若编译修补多于核心业务表达立即淘汰，不选from_curve全函数|
|8. 原native字节复用/模块服务|暂时免写复杂CPU owner，先让邻接状态链接真结果；复用现original executor|现Unicorn original CPU harness含RET检查/依赖桥；不是稳定应用服务。Mac host不能把PE字节当普通Rust函数直接调用|bounded纯CPU过渡中；ECS/GPU整个system很高|过渡不满足最终“恢复完整Rust业务”；Rust包装+原binary仍有缺边|指针/allocator/TLS/Win ABI、原资源池生命周期、宿主fork事件布局；captured state把gap藏起来|一个原Gate/Fence raw-input→CPU-output服务，只传语义input、不桥game helper，每次资源fresh。若需captureproposal/共享Bevy internals，则不能给World使用；后续须真正替换纯Rust|
|9. Remill等native→LLVM lifting|保留指令语义，可能复用复杂数值块；避免手工每条ASM|本机工具清单未见Remill/McSema，需安装/建额外runtime；现Ghidra/Unicorn已经能分析执行|纯函数试点高，跨过程memory/system很高|LLVM/FFI或memory-model执行不等于原业务Rust源码；只宜过渡/验证辅助|指令lift≠CFG完整恢复，导入/异常/间接跳/内存state未解决；新平台numeric差异|一个纯标量块，核对lifted IR的ops/外部调用/memory boundary；需整PE loader或大量依赖桥即终止。不得先选择Bevy完整system|

本表“低”指复用现资产即可完成一包；“中”指新增一种小抽取/manifest层；“高”指建设新的分析/编译runtime或通用别名/控制流系统。本轮没有实测任何加速器，不能报告百分比或假预计完成日。

## 应当改变的恢复流程

### A. 业务根集合与副作用摘要比全文索引更先

每族先声明最终root：Wall/Decorator/Roof权威与派生状态、storage/关联和碰撞、ordered96B砖/64B木板/真实mesh、真实事件、Nani/asset所有权、保存/回放与撤回/删除。GPU后置根单独保留，不能从“完整”中消失。倒推必须理解的game规则，再为其原依赖写摘要，如 `PerchanceContext.uniform_f32 mutates_rng`、`Commands.spawn queues_components`、`TransientMesh drop removes_asset`。

有些“值没用到”的代码仍不可删除：RNG一次draw会改变后续draw、Arc/Transient drop会改变资源、通知顺序会改变frame消费、source缓存会改变下次编辑。puffin/log通常能隔离，但不能用名字自动认定全部错误/状态分支无效。记录removed/summarized/preserved与根理由，使减少的对象可审计。

此处不是强制开发通用interprocedural slicer。第一包用现C/ASM与manual事实摘要即可；若根集合或alias复杂，继续完整owner恢复。节省衡量是少恢复了多少经证实的非业务体，以及capture/字段查询减少多少，不用百万inline条数作分母。

### B. 给库包装器建立“残余游戏代码”入口

仅给函数标dependency/game不够。分类最少四种：pure dependency、pure game、dependency wrapper containing game callback、mixed/unknown。调用或inline来源进入依赖，不代表整段无game policy。例如 std iterator的实际map closure、sort比较器、serde派生Visitor的schema映射、Bevy bundle的component list均可包含game事实。

正确替换步骤是：先确定依赖版本/API与调用边界，再绑定capture与callback输入/输出/副作用，保留遍历次序/短路/重复次数/borrow更新，然后生成调用原crate的Rust。不能把callback指针当opaque函数永久保存。当前私有 utils::threshold_range 和 Curve crashy_resample 是game规则，不因它们形式像iterator/math就排除；Fence的spacing、corner候选、Perchance播种/调用顺序都必须保留。

### C. 合同先行的机械生成，生成器不发明字段

最小manifest分 `semantic schema`、`observed memory projection`、`registration/components`、`lifecycle intent` 四层。known/unknown、版号、指令事实来源必填。serde字段名/variant名可能提供原名字，但 Serialize/Deserialize visitor还要核对rename、default、skip、自定义转换和持久化格式；**serde格式不决定repr(Rust)布局**。官方文档明确enum有不同表示，untagged按顺序尝试，不能自动统一为tag+payload。[Serde enum representations](https://serde.rs/enum-representations.html)

Bundle可生成Bevy `#[derive(Bundle)]`和真实组件登记；SystemParam名字可生成reader/resource/query类型候选。stage先后、Deferred flush、event跨帧、RemovedComponents、undo/redo行为不从derive推出，必须来自实际系统注册/消费者合同。源字段unknown生成必填/gap而不是Default0。这能减少父agent重复接接口，但不能让生成器替代owner判断。

### D. SSA先产事实，后只生成“小策略”

Ghidra raw pcode直接对应机器指令；high pcode引入MULTIEQUAL/INDIRECT等分析节点，CALL/CALLIND/RETURN输入也可能在分析中改变。因而“高pcode已经是可信Rust参数”不成立；保留ASM和raw origin，尤其当前void()占位。[Ghidra P-code reference](https://ghidra.re/ghidra_docs/languages/html/pcoderef.html)

首个自动产物应是字段读写/guard/call-param **候选facts**，不先输出可运行Rust。成功后，小策略generator只支持已知semantic schema、纯标量和可审计控制流，保数值顺序、wrapping、位宽与signedness。float优先保指令语义，不能统一 `max`、NaN policy、glam weighted/difference lerp，也不任意FMA；本项目已经有差异证据。原业务loops、动态Vec/HashMap、TLS/ECS allocator不走机械低级内存模拟；恢复它们的game callback再用原库。

“把所有pcode翻译成Rust寄存器/内存解释器”可保某些机器行为，却重造了运行环境，难维护且把确认依赖的容器和ECS又纳入翻译，不是当前目标的最佳减码方法。

## 为什么不把native bridge或通用lifting作为主路线

现original harness已解决部分boundedCPU执行；当原函数可达RET、有真实raw输入且game body没有被bridge，最小桥可作为邻接集成和oracle的备用。它省的是**暂时的实现等待**，不省最终必须恢复的游戏规则。当前用户仍禁止验证，本轮既不执行桥也不把结果当新证据。GPU handles、实际renderer进程、Bevy内存不能由CPU fixture假造。

Remill官方定位是instruction→LLVM bitcode库，另有memory/state和调用工具需求，不直接恢复Rust源码、serde/Bundle/ownership；工具支持的架构不等于本PE可直接移植运行。[Remill](https://github.com/lifting-bits/remill)

C2Rust官方针对C99源码，结果通常unsafe、非惯用Rust，非平凡项目还需真实编译命令。Ghidra伪C并非原作者C项目，补原型/CONCAT/borrow和ABI这一步通常才是主要工作，transpile不会替我们解决。[C2Rust](https://github.com/immunant/c2rust)

两条路线可以为少数难数值块留下proof/reference，均不应取代目前已经有大量semantic Rust的完整族恢复。若桥作为临时能力，每包清单要同时保留binary-required和PureRustReplacement，不允许从CPU-ready悄悄升到目标完成。

## 最小实施建议与淘汰条件

先做一个Fence/Gate资料包的“减码清单”，无需等全库索引：完整owner+必要callee→inline/library四类→game callback/capture事实→raw-input/CPU/生命周期root。用现有Perchance/glam/Bevy/mesh transport合同生成最多一套调用与组件胶水，保留纯Rust未知业务；同时让SSA抽取器只试一个既有public metadatacopy或kind政策。

该试点的实际比较：人工审阅的业务blocks/未绑定capture数、重写库体数（应0）、新增未知字段数、胶水代码人工修订数、资料等待时间、最终全链交付时间。保持已有失败和后续验证边界。若新工具耗时主要在通用解析/编译器runtime，或生成后每个owner仍需逐条手工修补，就撤回通用生成器路线，保留有效fact extractor与schema模板。

首包先列小型root清单；实际工具投入优先序：**2+3+4（复用/闭包/机械合同）→5（SSA事实试点）→6（小策略限定生成）**。路线1先人工执行，通用自动slice只在已证明有收益后投入。8只能临时解除单owner阻塞；7/9不进入当前主线。与统一plan兼容，但将资源从“收更多文本/搭更大基础设施”移到“减少必须恢复的体、重复ABI判断和父接胶水”。作者源码不可得不意味着只能逐函数手写，也不意味着可以跳过核心规则。
