# 原证驱动的增量恢复经验

适用：Tiny Glade 当前重构工程中的算法纠错、core/rhapsody 调用接线、启动与真实 GPU 验证。来自 2026-10-18 的原函数与发行 SPIR-V 核对。具体完成状态看 `tiny-glade/reports/main-chain-recovery-2026-10-18/ORIGINAL-FIRST-REVIEW.md`，不能用本参考代替当前源码审查。

## 保留工程架构，重构错误算法

2026-10-19 用户进一步明确：游戏逻辑、算法和设计的正确性优先，必须保证整个游戏的交互完整性。可以进行多态等设计更新，但不允许简化任何原作复杂业务或算法。不能通过删减分支、状态、副作用、生命周期或跨模块联动获得进度；未闭合路径必须继续取证，不以默认值、空操作、假数据或跳过功能代替。验收应覆盖真实 producer → 权威算法/状态 → consumer → 交互结果，而不是仅检查接口、编译或局部输出。

以当前 `crates/rhapsody`、`country-core` 和各业务 crate 为起点。用户要求保留重构后的 API、资源所有权和图执行架构；不要求保留过度简化的手写业务。

发现原版反例时，先写出具体差异：输入、分支、状态、副作用和目标消费者，再在现实现中修正。同步迁移调用者和真实数据类型。不要添加一层包装掩盖空网格、零尺寸类型、只增加任务计数的 worker 或空绘制回调；也不要另造 legacy 后端。

例：RT 不能以 opaque 节点/任务计数替代真实 CWBVH；木构架不能以平移缩放替代原完整仿射和屋顶分支。原锁定的公开依赖直接复用，游戏自身的参数、过滤、打包与通知另行恢复。

## 用两端消费者纠正字段误读

1. 绑定实际发行二进制/PDB 的身份，保存完整记录长度的原函数范围与哈希；不能只读截断反编译。
2. 先核原构造者，再核 CPU/GPU 消费者、Serde 和事件注册。PDB 名字只作导航，注释也可能延续旧误解。
3. 区分 native 内存布局、GPU record、Serde 文件格式和当前 Rust 语义接口；不要把它们当作同一 ABI。

已遇到的反例：Prefab 的 `String.len()==4` 曾被误作独立 tag；其 audio AABB 被误作 name/rank。原 schema 应有准确字符串和独立包围盒。Boat 的 native record 与 GPU shader stride 也可能不同，不能只看 shader 改 CPU 类型大小。

## 2026-10-19 多 agent 推进与取证效率

- 按完整且互相独立的调用链分工，明确共享文件的集成 owner。每个 agent 必须交付 producer、权威状态、consumer、失败分支和剩余项；不能用三份接口补丁冒充三条完整流程。跨域字段/注册建议即时发送，避免先各造一个同名 Resource。
- 编译前约定源码冻结点；冻结期间继续读取原证，不持续修改公共 core 令整批编译结果过期。由一个 owner 运行 Cargo；其它独立任务占用 target 时先只读确认进程/命令，不终止不属于自己的任务。需要隔离时使用独立 CARGO_TARGET_DIR，依赖缓存复制优先 rlib/rmeta、fingerprint/build，避免复制大量旧测试 exe/PDB；不要硬链接会被并发写入的缓存或复制整个 incremental 目录。
- Windows 长编译日志优先用 Python `subprocess.Popen(stdout=直接打开的日志文件, stderr=STDOUT)`，同步记录 PID/exit code；PowerShell 重定向的日志暂不更新不能独自证明编译停止。先查子进程与实际 CPU/锁文件，再决定是否处理；不要仅因日志空或低 CPU 反复重启重编。Cargo1.97 还存在 `.cargo-build-lock`/`.cargo-artifact-lock`，只查旧 `.cargo-lock` 不完整。Restart Manager 可只读定位实际占用者；不得删除其它任务的锁或终止不属于本轮的进程。缓存复制与编译不能同时写同一目录；复制中的 sharing-violation 不能当成完整缓存成功。
- 搜索不到保留证据先检查 Git 稀疏排除。当前 agent-team 被 sparse-checkout 排除，可用 `git show HEAD:<已核实路径>` 精确取出已有反编译，保存新工作目录并登记来源；不要重新全文扫描 PDB 或为已保存原函数反复启动新工具后端。
- MCP open 返回 opening 不是成功；必须检查 wait 的 SpawnFailed、内层 isError 和真实程序身份。相同缺程序/worker失败不要重复查询。保留失败记录，直接复用身份绑定的 .c/PDB，并用 LLVM 对完整函数长度的 RNE 范围复核新增事实。
- 优先从真实资源/注册 TypeId 和调用点参数证明语义，名字只用于定位。已确认 `render_menu_bg` 和 T11/T13 实际读 EscMenuState；Time+0x58 是 delta_secs，不能因旧字段名或数值为 f32 猜成 elapsed/EditingSpeed；光看寄存器 r9=0 也不能猜成 solid=false，须进入射线 leaf 验真实 bool。
- 系统的“属于 Update”与具体数字 set 是两件事。必须核 in_set helper 的完整 PDB 参数和 caller 立即数；本轮 camera9、photo8、GUI39 的不同顺序不能由注释/相邻反编译推导。显式宿主适配顺序与原证顺序分别记录。
- 存档解码先枚举发行包真实变体/字段/非空 aux，用真实序列触发残差，再追原 visitor/serializer/replay jump table。不能只扩 serde 外层或把未知 variant 存 raw 后丢弃来宣布 save/load 完成。形状 MorphPremadeShape 与封闭状态 No/Yes 曾被误合并，必须连同 runtime 消费者纠正。
- 先把真实 App、平台事件注册、设备装配封装为生产与验收共用入口。回归使用临时账号和原版初始场景，保持真实全部系统；缺参数/资源就在生产装配修复，不从测试移除系统或假造完成事件。CPU墙集合正确仍须核实际墙实体、几何上传和 DrawLists，否则图能导出只证明渲染执行。
- 画廊/菜单等 UI 同时核创建者、完整格子数、首格与后续格子的区别及点击写回。T12 的首格来自 config0 自带强度，后续来自 LUT metas 且强度1；iter_non_default 必须真正跳过配置0，不能让默认值被覆盖。
- 捕获结果同帧被 UI 读取时，在当前资源图用同一物理纹理声明读写依赖，不能仅绑定 raw descriptor。异步结果必须保留原共享 Arc，清除/重开后的旧结果不能写入新槽；同 id 重复发布不得因旧 owner Drop 释放当前纹理。生命周期与同步都要验。
- 优化的是取证/编译/通信开销，原作业务算法和交互范围不因此缩减。用户明确排除的 T13 单独记 excluded_by_user；其它未恢复项不能借此自动排除。
- 每次纠错同时记录原推断为何失效和新证据闭合到哪一层。本轮 FMOD bool 须沿 `run_game` 的 `CliArgs+0x149`、闭包捕获和 `AudioCore::new` 连续核实，原默认是同步更新；不能因后台加载 bank 就将 Studio 初始化设为异步。
- 原始注册特化可串起 wrapper、run_if 和整组 in_set：画廊 UI 的 `140ff9df0` 特化证明函数身份，MenuPlugin caller 的 `dl=1` 证明 Game 门控，`14068e29e r8b=0x24` 才证明 set36。这三种事实分别核实，避免把邻近 splash 的组号误用到 UI。
- 多采样 UI 验收应在真实4样本 attachment 上执行原 shader，并解析到单采样后回读中心及背景像素。仅核 AttachmentTexture.sample_count 或沿用单样本 fixture 不能证明 T21 实际输出；新增测试未执行前只登记“待验证”。
- 即使只有11B也须核完整指令与返回寄存器：`Node::used@140683c80` 是 `testb $0x30,+0xb6;setne al;ret` 的 bool getter，旧 void/no-op 判定会使菜单激活链永久缺失。反编译中的 ICF 名字也不能替代 wrapper 的 SystemParam 映射；Escape 的所谓 DragState 检查实际属于 TimelapseState。
- 高影响字段更名/bit交换必须独立复核完整值流，而且现有函数名称也可能错误。升级flag曾多次因沿用旧snapshot upgrader名字而误判；wrapper+d/+e→bit40/48是位序事实，Window/Arrow名称还必须由原subtype身份和具体修改字段/日志独立证明。先建立无语义数值表，再命名；候选保留独立目录/基础SHA，中央裁决全部算法后再落，不能把中间agent报告当最终事实。
- 字段名检索不命中不能证明权威owner缺失。原History+60 identity在当前架构已由ledger.mutation_nonce表达且保存writer在用；先对照原写者、现生成/修改/加载/保存消费者，提供typed投影，不能再造第二identity字段或资源。
- 用 PDB 函数全长验证反编译覆盖。ColorRadialMenu::create 原5531B，旧1947B文本的 switch 跳入 rdata；不能照抄缺 case 的文本。恢复 create 仍不等于闭合74991B show 与13路 hide提交。浮点 easing 端点的舍入误差应由验证容差表达，不能在原算法上补 clamp 迎合零值断言。

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

## 本轮追加独立复核经验

- 原跳表必须先证明界限：相邻地址间隔与全部合法tag、每个target是否留在函数范围共同校验。Color show真实13项，不能把下一张table和hide函数当39个业务分支；菜单目标22项又是另一层enum。
- 否定分支前沿全部CFG后继检查非邻接冷块。仅截连续范围会漏掉window snapshot Decorator循环，原全函数hash与范围仍不足以代替CFG覆盖。
- 保存往返与原语料字段保留、实际权威undo/redo互补；HashMap向量顺序可规范化，但其它数组和业务顺序不允许排序抹平差异。nullable浮点必须区分缺字段与显式null→NaN。
- 同名组件必须核register_component真实size；本轮原PrefabInstanceData48B和PrefabInstance88B被旧实现颠倒。构造/Query/原writer一起迁移，不能再加代理类型掩盖。
- 可以用实际生产源+缓存依赖的独立rustc harness降低核心重编成本；也可只映射原PE调用有界纯函数或用原指令模拟几何。仍须注明allocator/import桥边界，不能把原版fixture生成成功算Rust通过或GPU通过。
- 每个恢复者之外再分独立复核owner。结论出错要保留更正，不能靠两个agent引用同一截断反编译互证。

## 提交性能与真实观察点

- 先分别测量 CPU 范围准备、声明验证、真实驱动提交和等待。优化重放热点时保留原事务顺序、错误优先级、预算准入和取消回滚；用同规模输入、负例差分及逐字节像素确认，不能把某个阶段的倍数称作游戏 FPS 倍数。
- 各页在不同时刻的历史峰值之和不等于整个池的同时峰值。需要高水位时在真实状态变化处记账，保留退役、失败回滚、所有权释放和并发更新，不能用当前值覆盖历史峰值来通过断言。
- GPU 完成、CPU 命令 owner 释放和统计采样是不同事件。对照窗口与无窗口入口的实际调用次序；维护发生在编译前还是编译后，会改变失败副作用和错过完成的时机。按原顺序复用非阻塞完成维护，不追加 idle、放宽漂移门槛或改变稳态起点掩盖问题。
- 可恢复 present 错误映射出的成功返回不能作原生成功证明。观察原始结果，游戏验收还要绑定真实 App/World、非空绘制、唯一帧收据和实际 present 增量。关闭线程返回成功前，必须证明该 owner 确实启动并连接；线程生命周期不等于编辑落盘和跨进程恢复。
- 跨指令集算术 NaN 的符号可能不保证一致。只有绑定原指令及失败输入后才能作测试侧的精确平台区分；有限数、无穷、带符号零和有效 payload 仍按原合同检查，不改冻结原语料或生产数学以迁就测试。
- 集成前校验实际 before、候选、补丁和依赖，检查工具退出码后再启动下一步。使用 manifest 中的真实路径和 strip/cwd；零条测试、旧源码测试及未完成日志不算新证明。后续构建覆盖 executable 前，保留实际运行副本及哈希。
