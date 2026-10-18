# 原证驱动的增量恢复经验

适用：Tiny Glade 当前重构工程中的算法纠错、core/rhapsody 调用接线、启动与真实 GPU 验证。来自 2026-10-18 的原函数与发行 SPIR-V 核对。具体完成状态看 `tiny-glade/reports/main-chain-recovery-2026-10-18/ORIGINAL-FIRST-REVIEW.md`，不能用本参考代替当前源码审查。

## 保留工程架构，重构错误算法

以当前 `crates/rhapsody`、`country-core` 和各业务 crate 为起点。用户要求保留重构后的 API、资源所有权和图执行架构；不要求保留过度简化的手写业务。

发现原版反例时，先写出具体差异：输入、分支、状态、副作用和目标消费者，再在现实现中修正。同步迁移调用者和真实数据类型。不要添加一层包装掩盖空网格、零尺寸类型、只增加任务计数的 worker 或空绘制回调；也不要另造 legacy 后端。

例：RT 不能以 opaque 节点/任务计数替代真实 CWBVH；木构架不能以平移缩放替代原完整仿射和屋顶分支。原锁定的公开依赖直接复用，游戏自身的参数、过滤、打包与通知另行恢复。

## 用两端消费者纠正字段误读

1. 绑定实际发行二进制/PDB 的身份，保存完整记录长度的原函数范围与哈希；不能只读截断反编译。
2. 先核原构造者，再核 CPU/GPU 消费者、Serde 和事件注册。PDB 名字只作导航，注释也可能延续旧误解。
3. 区分 native 内存布局、GPU record、Serde 文件格式和当前 Rust 语义接口；不要把它们当作同一 ABI。

已遇到的反例：Prefab 的 `String.len()==4` 曾被误作独立 tag；其 audio AABB 被误作 name/rank。原 schema 应有准确字符串和独立包围盒。Boat 的 native record 与 GPU shader stride 也可能不同，不能只看 shader 改 CPU 类型大小。

## 事件身份、外层保存和状态阶段

- 不给所有 EventInstance 套固定头偏移。Rust 可重排字段；必须核同类型 writer、reader、stride 与 EventId 位置。队列编号可能在末尾，不能把它恢复成未证的业务字段。
- 内层数据的 serializer 不代表整个操作的保存格式。继续核外层 enum 的 index/from/to/tree 等实际字段和 replay；唯一存储索引、placement 变体和 Bevy Entity 不能互换。无证据的独立数据解码不绑定虚构索引。
- 看到同码符号或候选 SystemParam 名称时，先核实际数据 stride、注册与全部消费者。若组件名字绑定仍有歧义，保留歧义，不能据名字把 placement 写入包含 Vec/Arc 的元数据 owner。
- 某系统在同一函数里先创建再删除对象，可能触及 reserved Entity 与尚未应用 Commands。按原 contains/query 失败分支恢复，不为合成测试添临时出生表或默认 placement 来制造成功。

## 实际调度与第一帧回归

静态调度无环不够。相同函数注册多次后，按函数类型 `.after(...)` 会有多实例歧义；给具体阶段普通 SystemSet，再真实调用 schedule.initialize。

新增输入 producer 时，测试从空 UI registry 和未开始的 History 出发，经过首帧 UI 注册权限、start、实际权威修改、finish、账本提交，直到下一帧消费同步事件。手工预先 start 或预注册 widget 的测试只能接受已准备状态，不能证明第一笔输入完整。

查所有原事件 reader 的消费范围：首个/最后一个、是否 drain、是否短路下一 reader，以及模式门只跳过 UI 还是返回整个系统。再用混合事件和下一帧无重复的回归覆盖；单事件测试不能证明批次和生命周期。

公共几何查询同样不能替代游戏 callback 的策略。先核格子过滤、ID 条件、排序/去重、严格边界和早停，再复用当前索引。数值 thunk 用真实 import/符号定名；不要因近似输出把 round、ceil、floor 混同。保留原运算顺序和 NaN 来源语义。

## SSBO 构造逐类型恢复

对每个真实初始化项核对：完整类型身份、NAME、stride、capacity、usage、memory、alignment、None 与 Some 的不同处理、初始 instance count、实际 producer。

- None 可能不上传，也可能有原非零默认数据，不能统一清零。
- Typed、Instance、Sparse、Mapped、动态长度存储不可互换；构造出资源不代表生产算法已恢复。
- 图绑定使用同一 canonical Rust 类型及其 SsboId；旧路径需要兼容时重导出，避免第二个同名但 TypeId 不同的 carrier。
- 映射指针必须来自实际设备分配，并保留同 BufferHost 的强 owner；切 App 后重置本地事件游标和旧 writer。
- 原 uniform 在当前 shader heap 下沉为 storage 时，修物理分配的所需 usage，保留原 core 描述符合同；不要改写原构造器来迎合宿主。

## 同步访问与内容依赖分开

原访问码是同步信息，不能由 raw `0x37` 或 shader 的 RW qualifier 自动推断是否读取旧内容。

- 原 SPIR-V 的 atomic、Load+Store 和跨分区的部分写，可能要求保留先前内容；在实际消费者上显式表达，用现有 preserving API，保留原同步组和范围。
- shader 无条件绑定资源，不表示每个 uniform 配置都读取内容。首帧 reprojection 的发行 CFG 表明 history 无效时跳过 prev-depth 内容读取。
- 遇到严格初始化拒绝，先核 CFG 与具体资源；不要假造 initialized import、补无原证清零或关闭一般检查。
- 新增绑定/内容意图时，检查去重、初始化、依赖、裁剪、缓存键、同步与生命周期的完整传播。同资源的 active 与 inactive binding 共存时，不能整资源标作不读。

## 真实 GPU 失败先定位阶段

区分 pipeline 未就绪、descriptor 背压、资源绑定误用、GPU 提交/等待失败和像素不符。保留原 ErrorKind，并记录实际 pass、binding、usage、offset、range 与阶段。

`Timeout` 本身不能证明 GPU 执行超时；可能是 pipeline ticket pending 或 descriptor 容量耗尽。没有定位前，不任意扩大 heap、延长等待或减少真实工作量。

验证数据要经过原 shader 的实际转换。原 resolve 会将线性值编码为 sRGB，因此 UNORM 输入 17 回读约 73 可以是正确行为；应按原公式核对或直接读上传结果，不能把任意失败值硬编码成期望。

## 有界验证与完整启动分开

优先使用最小但非空的真实数据，验证实际 buffer、shader、indirect command、indirection、可见性或像素回读。不要靠零实例、空 DrawLists 或合成 LoadingGameComplete 获得通过。

记录编译、CPU World、真实 GPU、原版行为比较和完整游戏启动各自范围。加载画面 present 不证明 gameplay warmup；一条 intro 成功不证明完整 snapshot；某构造器通过不证明其 CPU producer 或完整 72 项初始化。

共享工作树验证绑定明确的源码与可执行文件哈希。必要时复制本轮 executable 再运行，避免另一会话重编译替换文件。只冻结清单中列出的文件；源/依赖变化使受影响旧结论过期，失败日志也保留。

## 工具与本机条件

共享 Ghidra/ReVa 默认只读；出现数据库损坏错误时保留失败，不重启/修改其他工作者的库。可先用身份确认的 RNE、PDB 和保留反编译继续有界分析；修工具不计业务恢复。

本机已有 Vulkan/MoltenVK。loader 路径、验证层路径和 Metal 执行权限是三件不同的事。严格 GPU 测试保留 `RHAPSODY_REQUIRE_VULKAN=1`；当前设备路径和必要环境变量以项目记忆为准。SKIP、加载失败和硬件功能明确拒绝不能算真实执行通过。
