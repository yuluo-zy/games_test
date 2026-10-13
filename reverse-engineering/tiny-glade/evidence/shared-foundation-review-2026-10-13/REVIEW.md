# 共享基础模块：并行代码 review 与修复

**统计口径修正：** 下文609/314及原因拆分是首次review时的原始混合库存快照，包含宏、类型helper与待分类项。用户指出误计后已分离62自动实现/依赖转发、46 schema helper与33待分类记录；当前业务主表为468观察/280部分分析/188未触及/274候选映射，194项业务接口无映射实现。见[分类表](../../INTERFACE_CLASSIFICATION.md)与[当前缺口](../../UNRECOVERED_INTERFACES.md)。此修正没有修改源码/历史差分或新增功能验收；下方代码修复与测试证据继续保留原范围。

三名独立 review agent 分别负责 utils 顶层、几何/曲线/Transform、asset/asset-pipe；父 agent 审查 paths/lang-lib/tabloid/theme-defs 并做统一回归。范围是此前恢复的七包，不包含其他并行工程。69 份当前源码/测试/示例 SHA 已登记；这不是原作文件或函数完成数量。

## 结论

**不能认定业务与功能都已完善。** 找到并修复四项业务/性能缺陷，清理两处重复基础算法；排除一项 LoopRange 误报。仍有 314 项原接口无已审查的映射实现，且已实现接口的泛型策略、边界输入及原游戏消费者链没有全部验收。

| ID | 问题与修复 | 验证与限制 |
|---|---|---|
| GEO-01，P1 | 小尺度三角交叠被误判为 Disjoint；恢复原容差分类后的选边、延长边求交和端点顺序 | 原入口新增304＋旧257组对照通过，含有序端点；完整游戏几何调用链未验收 |
| UB-R03，P2 | JSON 字段筛选每个忽略键分配 String；改为 Serde 解码后的借用匹配 | 50,000 个忽略字段：50,003次分配→1；已有12组字段策略及escaped-key/late-error回归通过；不是完整原 visitor 执行 |
| SM-001，P2 | 本地化缺失键缓存命中仍复制 key；先借用查询，再在首次 miss 时分配 | 缓存预热后10,000次查询：10,000次分配→0；稳定文本地址与首次日志策略保留 |
| AR-01，P2 | 线性 RGBA8 压缩前复制两遍整图；直接消费已拥有图像并借用线性压缩视图 | 512×512 BC7：两次各1 MiB像素分配→0；与原压缩依赖输出逐字节一致，不代表原全图片 producer 差分 |
| UB-R02，P2 | Spline knots 手写向量 dot/sqrt；改为原 glam Vec2/Vec3 length | 原构造、位置与切线保留fixture通过限定数值门限；极端/非有限控制点未全部对照 |
| GEO-02，P2 | Curve 手写排序切片二分；改为 std::slice::partition_point | 原328组坐标/位置/切线样本通过，保持重复累计坐标的右偏策略 |

LoopRange 的 inherent len 与 ExactSizeIterator::len 是不同调用；后者通过 size_hint 已正确返回空反向区间的长度。先前误报及多余补丁已撤回，未作为缺陷修复计数。

三角热路径同时移除了两次 Vec 分配与排序，这是源码可确认的分配改进，没有测量游戏帧耗时或宣称提速倍率。

## 未恢复接口与原因

[完整314项清单](../../UNRECOVERED_INTERFACES.md)逐项列出原接口、原源码、原因及原审计缺口；[机器清单](../../tracking/unrecovered-shared-foundation-interfaces.json)便于后续推进。

- 199项尚未进行接受的业务分析；44项虽有伪码，但还未解释成业务合同。
- 64项trait/schema归属待定，需要区分游戏字段/规则与派生或依赖胶水。
- 7项已有分析但尚未实现：Curve的侧向判断、扩展、近似点数重采样；Circle2d转Curve2；Rectangle2d的普通/有符号距离与角点法线。

CurveSlice新对象存在不等于17个原接口都已验收；Thresholded3D、Curve3策略、Transform注册/required-components、as_vec2投影消费者、WallSpace/LoopSlice及资产默认/继承等仍有缺口。原图片源资源不足使完整纹理producer无法全面差分；发布archive回写一致不能替代producer验证。

## 原作共有或尚未实测的风险

- RDP负epsilon的两个边界输入在原入口也未于300万指令预算内返回；恢复实现有相同不进展风险。需沿原调用者确认非负阈值约束，当前未擅改算法。
- tabloid强订阅者保留、活reader阻塞publish和未消费批次累计；语言静态文本热替换累计保留；这些策略有原证据，尚无整游戏内存/帧耗时测量。
- 原逐资产并行策略可能提高大批次峰值内存/线程数；目录mtime与header-only缓存命中存在陈旧或不完整缓存风险，按原证据保留。
- Curve::reverse复制与重建有O(n)额外工作；原入口也有clone，尚未证明是消费者热点。

## 依赖与验证

本批七包可达的104个外部依赖版本/source/checksum全部匹配原Cargo.lock；没有新写解析器、压缩器、容器、线程池、RNG或向量库。发现的glam向量长度与std二分重复实现已清理。原feature集合、Bevy/fork配置及某些泛型绑定仍未全部核实，版本匹配不能替代这些验证。

父级统一回归为七包**85项测试、严格all-targets Clippy全部通过**，执行期间源码及锁未变化。geometry agent先前的57测试记录包含utils agent尚未撤回的误报测试；最终统一版本utils为56项，未修改历史日志冒充同一快照。详见[integration-validation.json](integration-validation.json)、[依赖核对](dependency-validation.json)及[当前review台账](../../tracking/shared-foundation-code-review.json)。测试覆盖限定合同与fixture，未执行完整原游戏。

| 分组 | 独立报告 |
|---|---|
| 小型模块 | [small-modules/REVIEW.md](small-modules/REVIEW.md) |
| utils 顶层 | [utils-base/REVIEW.md](utils-base/REVIEW.md) |
| 几何与曲线 | [geometry-curves/REVIEW.md](geometry-curves/REVIEW.md) |
| 资产与纹理 | [assets/REVIEW.md](assets/REVIEW.md) |

历史原指令报告保留原输入SHA。本轮改变的源文件由新的review与回归快照覆盖；不替换历史SHA、不提高原接口完成计数、不宣称原消费者链验收。
