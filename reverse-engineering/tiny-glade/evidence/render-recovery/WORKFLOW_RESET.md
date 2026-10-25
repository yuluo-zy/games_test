# rhapsody 恢复流程重审与替代方案

> 2026-10-18 用户最新约束：`games_test/tiny-glade/crates/rhapsody` 已经重构，当前工程架构为基线。游戏恢复只允许基于当前 API 增加缺失功能、修复具体缺陷；不能按原版重新复现 rhapsody、复制旧 ABI 或新增平行后端。下文涉及旧 `reconstruction/render` 入口及源码组织的约定为历史背景，以此最新约束和 PROJECT_MEMORY 为准。原版证据继续用于核实真实游戏消费者的语义需求。

日期：2026-10-12。适用当前目标SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。此文件取代旧批次计划的任务分派和新增验收方式；历史源码、原码报告与证据保留，不提升原验收范围。

## 审查结论

旧计划已经提出类型族和端到端路径，但实际执行没有落实生产整合。共享 `reconstruction/render/lib.rs` 仍只导出 resource_info/resource_usage；work-packages 下有16个目录；多包独立定义 Command、QueueIdentity、TextureDesc、Event。R5B从0生成sequence，R6A原counter从1开始，R7录制仍注入sequence7。第五批integration明确production_gpu_connected=false。检查报告相容不能代替调用链接通。

原机器码差分有实际价值，但任务完成奖励偏向局部可验收结果：挑函数、写fixture、生成独立crate、扩案例、核对大量报告，然后把真实接口整合留到下一批。这个循环会不断生产局部成功，难以消除跨模块阻塞。

不能承诺一键PDB转Rust：当前发行文件未找到作者Rust文本；当前类型证据不能提供完整游戏布局；native void()是占位。不同构建、尤其C/rhapsody2的类型与实现不能直接移植为当前构建事实。也不能用自设计Vulkan渲染器替代原作私有行为。

## 新工作单位：可调用的源码闭包

每个任务由一个真实消费者、一个公开输入和一个可观察输出定义，必须包括所有必要的私有生产者。函数只作证据定位。以“图声明进入、资源映射与命令提交参数输出”为任务，不以“完成queue_submit几个函数”为任务。

一个闭包可以跨raw/rg文件。共享资源描述、句柄、Command、QueueIdentity、Sequence、Timeline由整合owner维护唯一语义定义；ABI镜像仅留在原码测试适配层。未知字段保持opaque，不为了统一而补造原名称或布局。

停止新增独立candidate crate。唯一恢复入口沿用reconstruction/render，必要时将其整理成单一workspace。旧work-packages冻结为证据与迁移来源；先迁移高收益链，再逐步减少旧依赖。不机械把16个包全部粘贴进主模块，也不为单函数新建子项目。

每次交付必须回答：哪个真实consumer现在调用了恢复实现？移除了哪个私有stub/固定fixture？哪些外部库/API边界仍被注入？如果这些问题没有变化，本轮不记作集成进度。

## 改变验证方式

1. 默认运行共享入口的少量链级场景：创建/导入、访问冲突、callback失败、export、跨帧复用、CPU retire和延迟回收。
2. 原侧与Rust侧使用同一个输入语义和API事件schema。只在std、锁定依赖、Vulkan等外部边界捕获；私有函数替身列为缺口，不因事件输出相似声称闭环。
3. 差分用于分支极性、字段解释、错误副作用或链输出差异的裁决。禁止仅为增加case总数建立新oracle。已有复杂数值规则保留其专用oracle。
4. 未改变的源码/依赖/输入hash复用已有有效结果；按依赖图重验受影响闭包。全面回归只在合并关键接口、发现回归或发布前运行。
5. 编译与接口检查不证明原作等价。链级candidate测试也不是原作整链oracle，必须独立标记。只有去除私有注入并得到原侧证据才扩大验收范围。

## 取证与工具投入

一次为完整闭包读取producer/consumer证据，集中记录类型约束、调用参数、所有权和失败路径。复用现有workgraph、限定反汇编、RE-MCP持久库、shader反射与SDK runner。批量步骤由工具完成，模型阅读整理后的相关片段；泛型实例仅对确有语义差异的分支另取证。

不重复全程序索引，不重做shader全量反射，不增加新调度/账本框架。当前只读查询库继续保护原证据。若需要类型注释改善反编译，在身份绑定的可写分析副本中操作并保存revision；不要把所有查询永久save:false误当成禁止持久知识。当前xref不完整与跳表越界继续用原码补证。

潜在大幅提效路线是从原作高层入口集中捕获Vulkan参数和资源事件，一次提供众多包装函数的合同，再重建源码闭包。先做一个受控入口可行性实验；记录是否需要启动原游戏、目标环境、线程与捕获覆盖。当前尚未实现，不许声称已抓到帧轨迹，也不先花大量时间开发通用注入器。GPU/场景测试可以作为后续取证手段，不要求整游戏场景恢复作为前置条件。

## 首轮迁移任务与退出条件

第一轮不再分发R8函数包，先完成以下闭包：

- graph声明 → ResourceInfo/Registry → 由compiled字段产生record输入 → 原counter规则产生command sequence → 顺序record → submit参数。禁止在中间手填first/last access、固定sequence7或手造compiled snapshot。设备与同步外部边界保留明确接口；完整queue前段尚未知时返回typed gap，不伪造生产提交。
- 下一闭包为反射生产者 → sparse/enumerated layout → pool/descriptor writes → compute pipeline消费。638声明候选adapter不能替代原shader到pass绑定。
- 随后完成retire → temporal下一帧 → completion/disposal，并接最小country-core consumer。多队列、并行路径按调用可达性证据恢复；不能因为难就删除，也不在主链未接通前无限扩张边缘fixture。

第一轮成功：单一公开入口可运行，输入仅为graph/资源声明、外部后端及callback；sequence由真实恢复counter产生；compiled→record映射自动发生；输出保留真实状态和错误；测试能揭示接口不相容。其CPU candidate闭环和原侧证据范围分别报告。

第一轮失败：明确指出阻塞producer/type/ownership，不新建另一个独立candidate包逃避整合。取证只针对这个阻塞。迁移改动不修改历史冻结crate，使旧oracle继续可复现；新源码的行为验证单独登记。

## 人员、吞吐与估算

多agent按共享接口已确定后的源码闭包分工，一名owner持续整合；每个worker交可合并模块和consumer接线，不能只交独立库+报告。需要并行时沿用户授权安排，不在审查阶段为了人数而启动新批次。

进度主指标为：接通的消费者链、已移除私有替身、可运行模块、阻塞闭包及有效原侧证据。case数量和哈希检查数量仅作验证细节。每轮记录取证/实现/验证/整合耗时、首次运行失败点、重复工作；用两轮实际数据比较新旧吞吐。没有测量前不承诺提升倍数或缩短到具体天数。

此前2–4周/6–12周估算没有实测吞吐支撑，不作为排期承诺。新流程完成首个共享入口和原侧链验证后重新估算。

## 本次交付界限

审查早期枚举为16个work-package目录；收尾重新枚举已有20个，新增descriptor-binding-r8、queue-submit-r8、sequence-runtime-r8、temporal-lifecycle-r8。它们不属于本轮创建，未审查其新实现和验收，不能把早期16当作最新固定数量。不要删除或覆盖这些并行新增工作；后续先读其证据，再选择迁移到共享入口。本轮不宣称停止了其他chat的任务。

本轮完成源码与报告结构审计、替代流程和项目执行规则更新。尚未完成上述共享入口迁移、统一真实类型、Vulkan帧捕获或新的原作行为验收。旧五批结果仍在各自范围内有效，M12仍verified_partial。
