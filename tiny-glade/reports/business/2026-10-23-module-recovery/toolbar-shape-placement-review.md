# TB-R7 前置 C：形状放置恢复与复核

本轮恢复原 B `systems/wall/shapes/shape_placement.rs` 的权威放置步骤；没有把原来的六参数近似系统直接挂到生产宿主。当前结果是库内完整原分支、原侧指令差分和真实 ECS 生命周期通过，整段创建/绘制接线由中央与对应 owner 联合验收。Steam SDK 与真实 OS 输入不在本轮范围。

## 原证与纠正

- 原系统 `14097f970` 的 PDB module519/record87700 是 **5171B**，正常返回在 `1409807f2`，后方仍有墙向内吸附、排序、panic 与 unwind。共享反编译器只识别 2003B，并越过 17 项跳表误收入其他函数；本轮按原指令和 PE 相对跳表截断验证，未按错误 switch 扩写游戏分支。
- 系统真实输入为 AppMode、光标、公共墙、空间索引、Roof Query、音效、ShapePlacement、SnapMode、Modifiers、可选 RaycastWorld、CameraCursorRay、吸附提示、冻结 reader、指示符流，共 14 项。地形高度直接来自光标；没有原“地形资源重采样”参数。
- ShapePlacement 的原有效载荷为 80B 联合记录。现有 authoring 已使用其安全字节观察面，故在原 owner 增加有界观察字段访问器，并由中央导出同一类型。原默认只写 tag0、surface0、optional tag2、位置及尾载荷0；不存在旧注释猜测的 0.3 初值。inactive 联合载荷和 padding 只在 Rust 默认时确定性清零，不作为原 ABI 判等字段。
- `Public+280 = source+58 = authority.elevation()`；`Public+288 = source+5c = authority.arch_height()`。原 publisher module556/record48588 实际18515B，关键 `140a8d825/d904` 与 `140a8d82b/d914` 分别复制这两项。已补现有 `building_projection` 的对应写入，没有造零或重新采样。首轮读取未扣 hash bucket 的 key8B，曾把候选资格错读成+288；完整指令复核后确认 **候选检查+280 ≤1.5**，已纠正。
- 空间段原 wall ID 在+28，段序号在+30。`140aa5e71/ea2/eb2` 写入并递增该序号。原 AABB query 排序/去重读+30，消除同一段跨格重复，**不能按墙去重**。新增原 owner 的有界完整段查询并修正 AABB 点接口委托，保留同墙不同段。其他圆/矩形旧查询适配器未在本任务扩改，明确不宣称整个空间索引已恢复。

## 实现的业务

仅在圆/矩形 AppMode 中工作；模式 gate 之前不消费冻结 reader。收到冻结事件则消费未读整批并跳过该帧。无效光标使用零位置继续原流程。

真实物理查询使用 mask FB。类别0/9走墙：分别求原 mapped/source curve U、原形状法向、上限60、低于1.5时回落 mapped outline，并对矩形创建的规则墙按圆半径减0.925/矩形尺寸减1.25向内调整。可见放置保留碰撞 XZ，base 单独使用 mapped XZ 与当前Y，不能混为同一点。MINSS 原 NaN→60语义已恢复。

类别2走屋顶：规则形状、屋顶实体及组件均按原失败路径检查；求脊点/脊线、0.5阈值与端点夹取，保存固定基高、来源 elevation 和原注册项 byte1。实际 DebugSphere/DebugLine 工厂进入现有 UiSignifierStream。未靠旧数字 raw signifier 代替绘制请求。

其他可回落类别走完整 SnapProposal：每段引用有效公共墙，资格+280≤1.5；最近点取整条 mapped outline，矩形法向取轴，圆/自由墙法向取该原段方向。距离≤1.5保留，u 使用原 lerp；按 XZ 距离稳定排序取首项，NaN partial_cmp 失败保留原 panic。未把距离平方或“每墙最近边”视为等价。

有候选时禁用吸附发布 `NotifyHintSystemSnapping(true)`、回到光标无吸附，并保留先前 SnapMode；正常发布false，只有从0进入1/2时发送 kind3 音效。无候选按原不发送提示，未按住禁用时重置 SnapMode。高度+3c命名 elevation，calc 后写入+40命名 computed_height，分开向原创建消费者交接。

## 验证

- `toolbar_shape_placement_native.py` 使用绑定 SHA 的原 RNE，在 Unicorn 执行完整系统正文、候选闭包及实际几何 helper。**190例**包含圆/矩形/自由墙、墙/屋顶、两创建工具、1.5/60边界、无效光标、NaN/Inf、退化段、禁用、冻结、模式gate、17项碰撞类别及失败路径。
- 原侧桥接标准库 hash/alloc/memcpy、权威容器查找、已知物理查询输出、factory入口；CRT cosf/sinf/fmodf 显式用本机系统 libm 依赖桥接，不能据此宣称微软 CRT 数学实现逐位等同。自有曲线/屋顶/选择逻辑没有候选数学回填。
- 原 AABB query 完整394B的 **50例**核对排序键、跨格重复、同墙多段和20/21/511项分界；其 get_cells 收集边界用探针拥有的段引用桥接，bounds裁剪与完整排序/去重执行原指令。格成员生产仍复用现有空间 owner，不将这50例冒充其全模块差分。
- 当前生产源共用 `resolve_placement`/`finish_placement` 在真实 World Query 下与原夹具比较全部激活字节、提示、声音位置及真实 factory 数量。inactive/padding 排除，NaN 只比较NaN分类，其余逐位。190例通过。
- 真实 `Schedule(shape_placement_params)` 经过 authority→publication→spatial，验证重复帧不重复音效、禁用、模式抢占对冻结 reader 的顺序、冻结一次性消费、删除墙体与重建资源；没有注入生产测试 hit 资源。
- `cargo check --offline -p country-core` 通过；最终同一测试产物 shape_placement **5/5**、wall_spatial_hash **4/4**。最终产物/源码/夹具 SHA 与命令见 `toolbar-shape-placement-source-evidence.json`。首次展开探针缺 CRT 导入、一次其他 owner 的 UiState 初始化构建错误及修正后结果均保留日志。

## 性能与剩余边界

复用现有 source/mapped 曲线缓存，按借用切片求坐标，不逐候选克隆 Vec。空间查询返回拥有段ID缓冲的借用迭代器，消除额外引用 Vec；原候选稳定排序与完整分支保留。此轮没有发布帧率、release时延或GPU性能验收结论；190例用时是测试成本，不是游戏帧耗时。

中央还需完成生产 schedule、创建/高度/指示符消费者的共同接线；后续本 owner 已接管 shape_placement_signifiers 的完整恢复。原 OS 输入按用户要求暂缓。本报告不把局部通过称作 toolbar 或整游戏完成。

## 下一轮复用的经验

1. PDB procedure extent 优先校正反编译函数边界；相对跳表按真实条目数读取，不能沿反编译伪分支扩散。
2. 内联 hash lookup 的地址基准是 bucket end，须扣完整 value 与 key；先找 publisher/authoritative producer，再解释 consumer 的偏移，避免 +8 误差反转业务资格。
3. 相同墙不等于相同段。排序键必须追到 add_segment 的真实写入者；同键稳定性不可观察时可复用 std 依赖，不凭“原是复杂排序”重造依赖算法。
4. 原选择逻辑与副作用共同差分；只验证几何叶子会漏声音重复、hint bool、冻结消费及卸载。
5. 把 NaN/Inf 输入编码为位型，保留 MINSS/MAXSS 有序/无序差异；显式列 CRT 桥接与 ARM NaN 分类边界。
