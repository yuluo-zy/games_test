# 半木梁数音频原指令复核

原 B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。`timberframe_beam_count_change_audio@1421b6610` 的 `66bb/66c3/66c6` 读取原 GameTime+44 并做32位减法与1比较；`time_update@140922e1e` 对同槽 `incl`。当前重构 GameClock 的 Rust 布局未保证原 ABI，本机独立 layout 探针观察 frame_count 在40、ran在44；旧实现裸读44且旧夹具裸写44，不能证明生产语义。现改读实际 frame_count 并按32位 wrapping 运算；真实 World 覆盖正常字段和 u32 MAX→0。原图截取 `/private/tmp/m6-audio-asm.json`、`/private/tmp/m6-clock-asm.json`，snapshot `053060f0c5d388b353567d8b`。

原 `663e..6697` 选取两个事件队列中最后未读项（优先新队列）；`66a5..66ac` 游标推进至尾。现使用 `EventReader::read().last()`，两位置反例验证取最后项和下帧不重复消费。

原 `get_building_size_01@1417d3f10`：矩形面积 `x*y` 并以 5.0625、225 归一化；圆面积 `(r*r)*π` 并以位型 `4022dc35`、`43490fdb` 归一化；`inverse_lerp@140c904a0` 为 `(t-a)/(b-a)`，不夹紧。结果写 `PlayTimberframeSound.value`（payload+0x14），事件序号不属于 payload。之前实现固定0，现按原计算。

独立原执行：`m06-timber-size-oracle.c` 在 x86_64 Rosetta 映射按 PE section 展开的 B RNE，校验入口，直接调用原完整 size 函数和内部面积/归一化函数，无外部桥。输入圆/矩形各0、0.9、2.25、8、15；原位型见 `m06-timber-size-oracle.txt`，生产 ECS 事件逐位相等。独立原执行不覆盖 NaN 或 Windows 实机。构造映像须验证 RNE SHA，然后复制 PE 各 section 至 VirtualAddress；不得将 raw 文件直接映射为 RVA。

`CARGO_INCREMENTAL=0 cargo test --offline -p timberframe --test numeric_assertions` 8/8，日志 `/private/tmp/m6-timber-audio-tests-v3.log`；首轮测试夹具临时 reader 借用编译失败见 `/private/tmp/m6-timber-audio-tests.log`。音频函数补齐不代表半木系统验收；curved_halftimber 实体和木地板边条 TODO、重算事件 producer、Nani 真实消费仍需闭环。

半木实体后续证据：`curved_halftimber_maintain_entities@1407706d0` typed 参数除墙、删除/屋顶删除、两图与 Commands，还含 `RecalculateHalfTimberMeshCmd`、`RecalculateWoodenTurretCmd`、WallHoles、UiSignifierStream、Roof Query；两事件各为 Vec<64B CurvedHalfTimberInstanceData> + WallId，M6 原木塔 producer 证实非空数据来自 `construct_turret@14124cc30` 的 `new_curved_xy`。父体从每个实例+0x30读 u32，经 140833250 的原指令直接传递，再经 14082bd60 写 mesh tag1和值，即 AtlasPart(instance.w30)，不得因 ICF 名 `DecoratorRank` 猜语义。现有 CurvedHalfTimberDrawables 两表不是旧注释所称 Gap，Nani实例数据和构造器已存在；父维护控制流后续已恢复，当时目标孔木边条仍未实现；后续首轮实现及验证见下文。

墙体 writer 原 `construct_walls@140757800` 经 finalized holes 与 style 门后调用 `half_timbering@141218e30`，gable 分支再调用 `roof_outline@141217be0`。后两函数及墙体事件实际输出目前未闭环，不可把空 Vec 或空实现视为原结果。原主函数 PDB 物理跨度35438B；旧默认反编译失败。独立只读 ReVa worker `m6-half-20261029` 对 B patched SHA `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323` 关闭 decompile/cleanup 后完成，`success=true`、`timedOut=false`、`stdoutTruncated=false`；完整C 183100B SHA `33057e51d02211da21856c3a7685bc8dc3c275c0b48f6f78d2eeecc351f3791b`，压缩结果见 `m06-half-timber-decompile-2026-10-29.json`。此结果不是签名、CFG 或算法验收，仍需与发行RNE原指令交叉。

原墙体 writer 数据流进一步以有界机器码确认：`140758d49..65` 初始化 Vec(0,4,0)；`758d71..81` 在 style is_halftimbered 且 roof 指针非空时进入；`758dc2..e2f` 把该 Vec 地址通过 `[rsp+104]` 传给 half_timbering；`758e3b..6f` 仅 Roof::is_gable 时调用 roof_outline；`758e74..8f11` 无条件向事件队列写 Vec 三字、WallId、Bevy event id，故非半木/缺 roof 要发布空 Vec 供旧实体删除。`half_timbering` 还在原权威 WoodenPlankSsbo 持久 allocation 上 clear/write，不是单纯返回 Vec。`wooden_floor_borders` 原要查 PublicWalls 的 roof entity、Roof、WallHoles finalized，筛选孔类型并按圆/矩形走 `new_curved_xz`/`new_straight`，不能用现存的 points/floor_y 简化签名替代。原父体到边条调用的寄存器及三个栈实参见 `/private/tmp/m6-curved-asm-200.json`；算法与边界尚未闭环。

父实体消费现实现原顺序：OnWallDeleted/ DeleteRoofCmd 删除两表和仍存活实体；半木事件先删除 primary，缺墙则跳过，存在墙才调用边条并插入新实体列表；木塔事件替换 secondary，空Vec仍留下空列表。实例使用原 `instance.w30` 作为 AtlasPart 和直标志 1/2。真实 World 测试 `/private/tmp/m6-curved-parent-tests-v2.log` 2/2：存在墙无屋顶的 primary 创建与屋顶删除、木塔实例创建/空替换/缺墙 half/墙删除。原屋顶门指令 `14124444a–471`：tag0用 `WallStyle::is_halftimbered`（`1408c5670 cmpb 2; setae`），tag1返回，其余tag测试+0x41 bit0；原 holes map 空/缺项直接返回，现接已有owner并保留 finalized 要求。hole kind1/4 的圆形或矩形几何尚未完成，现显式 TODO，禁止将有限World通过解释为全链通过；重复孔筛选尚未证明，有目标类型时保守阻断。

统一构建 epoch `/private/tmp/m6-curved-parent-tests-v3.log` 2/2，新增同帧删除墙后木塔重算仍创建新实体的顺序断言。此通过仍不含目标孔边条几何或实际半木墙砖 producer。

生产接线纠正：`curved_halftimber_maintain_entities` 已加入兼容 `add_wall_constructor_systems(Schedule)`，但正式 `startup_construction_runtime` 逐项注册建造系统，未调用该兼容注册入口；`wall_constructor_plugin::build` 只初始化资源。因此目前 World 直挂测试不证明正式 App 已执行该消费系统，需由宿主按原 set 和 producer 顺序接线并验收。统一epoch直接二进制 `/private/tmp/m6-curved-parent-same-epoch.log` 为2/2，但不含其后其他owner对detector的源码修改，不能当当前工作区整体验收。

木边条的独立 worker 关闭 cleanup 反编译成功（success、未超时、stdout未截断），C 23698B SHA `353b80888b37f94df07faa4ec21377f197aae15b43657fc5a9a1d3e8d3644499`，压缩原结果 `m06-floor-border-decompile-2026-10-29.json`；其默认 Ghidra body 3135B 不等于PDB物理跨度3842B，仍须依原指令验证控制流。圆分支从目标孔上下界在墙长空间换算为角、减 `CircularF32::as_f32`（原 fmod 后负值加TAU）得到角区间，位置 `(circle.x, hole.min_y, circle.y)`，调用原 `new_curved_xz(radius, 0.54, 0.095, start, end, 0)`；它自建的半木组件 direct 为0，使用roof颜色路径。矩形短孔用原 `fmodf(x/perimeter,1.0)` 与 circular_slice，长孔用 try_as_curve2；端帽、完整异常与字段顺序尚未验收。`roof_outline`、`add_underwindow_pattern` 原静默空实现已改为明确 TODO，避免未来错误接线伪成功；它们的原几何尚未恢复。

原 half_timbering 入口保护：读取原 demo override 为真或 PublicWallState 无 premade shape 即返回；随后解析 Roof gable/flat、获取 WoodenPlankSsbo 并对 PublicWallState+0x180 原 allocation 加锁并清空 writer。按 caller 与 callee 栈投影确认：RCX=PublicWallState、RDX=WallId，caller `[rsp+40]`（callee rbp+1768）为 SSBO library，caller `[rsp+88]`（callee rbp+1816）为 Roof，caller `[rsp+104]` 为输出 Vec。其余栈参数尚未确证。原体还调用 `utils::rng::init_rng`、fastrand::Rng::f32、SubtractiveBoolean1D、窗下图案；workspace 已有 fastrand=1.8.0 和 utils 对应实现，必须保留原种子与调用顺序。

set32 原注册：`1404934e4` 调 `140272c20` 的 chain_ignore_deferred，`1404934e9` 写 tag 32；PDB完整 tuple末项含 CurvedHalfTimberDrawables 与两重算事件参数，与本父体一致，前两项为石膏系统。原顺序不得用兼容 Bricks chain 代替。

caller原投影续核：`construct_walls` 将其caller第13参数（typed tuple `<Res<DecoratorStorage>, DerivedDecoratorInfoResources>`）首项保存 rbp+240，并作为 half_timbering R8；R9由 `decorator_subtype::impl$8::from@140b26a70` 的二字结果承载。假 `WindowAabbSource` 不是完整原生产依赖。图证 `/private/tmp/m6-wall-asm-0.json` 和 `m6-wall-asm-1.json`；余参及输入构造仍待核。

原 size 探针复现：在仓库根用 Python 读取 RNE 的 PE header `e_lfanew`，按 `SizeOfImage` 分配零缓冲、复制 header，再逐 section 把 `[PointerToRawData, +SizeOfRawData)` 写入 `[VirtualAddress, +SizeOfRawData)`，输出 `/private/tmp/m6-timber-rne-image`；先核上述 RNE SHA。然后 `clang -arch x86_64 -O0 tiny-glade/reports/module-recovery-2026-10-23/m06-timber-size-oracle.c -o /private/tmp/m6-timber-size-oracle`，受审查权限执行 `/private/tmp/m6-timber-size-oracle`。输出10行按圆/矩形、五个尺寸顺序；探针还核原 7 字节入口，避免误将 raw RNE 当映像。

图分页已取 half_timbering 全 6925 条原指令（B RNE/PDB 身份不变，snapshot `053060f0c5d388b353567d8b`）；两个 `init_rng@140c96240` call 位于 `14121ae49`、`14121bd89`。第二处先以已保存的 WallId 调 1400885d0（ICF alias 包含 WallId::get_inner，实参流而非另一个 minidump alias 给出身份），将低32位作为种子；第一处组合多个截断/饱和项与墙字段，尚未闭合，不能随意重置随机流。分页保存在 `/private/tmp/m6-half-start-asm.json` 和 `/private/tmp/m6-half-asm-{1..34}.json`，可由固定 snapshot 和 function:B:1218e30 按200条重查。

第一处种子 `14121ad9e..ae49` 调 WallId::get_inner 后将两个尚待追名的 f32 各乘 `10.0`、经原 x86 float→u64 饱和序列，再取低32位与墙ID低32、rbp+1216低32做 wrapping add，输入 init_rng。两个浮点及栈值的业务身份尚未确认；静态 ICF 名称不能替代实参追踪。

实例 allocation 分槽纠正：`construct_walls@14075819c–1aa` 与 support detector `140787162–170` 都从 PublicWalls 680B raw bucket 取 value -672，即同一 PublicWallState 基址。前者调用 half_timbering 后者调用 clear helper；原 half 在 `141218f8a–91` 读基址+0x180，support clear 在 `140787e0f`/`140787f1b` 分别读同基址+0x140/+0x150。+0x180 与支撑两个槽因此是不同字段，不能因为都访问 WoodenPlankSsbo 就合并。当前应在既有 ConstructorGpuOutput 权威 sidecar 以独立按墙 allocation 承载，保留不同清写和释放时点；原构造/删除分支仍需复核。原分页 `/private/tmp/m6-half-start-asm.json`、`m6-wall-asm-*.json`、`m6-detector-*.json`、`m6-clear-owner-asm.json`。

为原+0x180独立槽，现有 `ConstructorGpuOutput` 增加单独按墙的半木 plank allocation；沿现有 SparseInstanceBuffer 的 RAII 和 waste 队列，提供显式初始化、要求槽存在的 clear/append、查询和 remove_wall 释放；flush 检查半木槽也使用同一真实 SSBO。`/private/tmp/m6-half-allocation-tests.log` 1/1 CPU-shadow 验证同墙旧planks与half两个槽有数据、分别清除互不影响、墙删除后两分配进入waste并清空。初始化调用点、真实GPU及几何尚未接线，本结果不构成原生命周期全链验收。

原 half_timbering 构造 writer 后在 clear 和整段生成期间保持同一 allocation 锁；为避免未来调用把这段事务拆锁，sidecar 另提供借用同一原 allocation 的 `half_timber_writer`。CPU-shadow 复验 `/private/tmp/m6-half-allocation-tests-v2.log` 1/1，覆盖连续 writer clear→push 后原/半木分槽互不清除及 RAII 释放。它不证明真实 GPU 上传、几何或原初始化调用点。

当前源码重新编译，独立槽回归 `/private/tmp/m6-half-allocation-final.log` 1/1。另核调用参数：原 `construct_walls` 保存 PublicWalls bucket 指针于 rbp+1048，随后从该指针 -424/-416 读两个字并传 `[rsp+48]/[rsp+56]`；同 bucket 的 PublicWallState 起始为 -672，故它们对应 state+0xf8/+0x100 的指针和长度。既有 `PublicWallState::segment_heights` 文档将原 +f0/+f8/+100 标为第三数组，因此此 slice 与现存 segment_heights owner 的映射有明确位移证据；其元素产生时点及 half_timbering 全部消费尚需继续核对，不代表完整签名已恢复。

callee 消费进一步核对：`14121e980..9b7` 对该 slice 的索引 i、i+1 做边界检查，也检查 PublicWallState+0x98 对应长度；`e9bd..9df` 读取该 slice 相邻 f32 并按 `a + t*(b-a)` 执行标量插值。此处只能确认局部读取和位序，不能据此推断全函数输入或容错分支。

木边条目标孔逐项处理已开始：原圆分支按 `1412445e9–694` 的 f32 运算顺序计算中点、半角，构造曲面实例并以 direct=0 创建实体，当时矩形仍为 TODO，后续已按原体进行首轮实现。`holes_no_duplicates@140f86cd0` 只构造首尾指针，本身不能证明是否过滤 origin；木边条 loop `141244573–590` 每次直接跨 20B，只读取 +0x10 kind 并比较 4/1，没有该处读取 +0x11，其他 consumer 的 lazy filter 不能套用到这里。最终 iterator 合同仍待 PDB 返回类型及 loop 其他回边完整交叉核；原 finalize 指令分页 `/private/tmp/m6-hole-finalize-asm.json` 和 `m6-hole-finalize-asm-200.json`，包含原洞 origin0 和 wrap origin1 的产生及排序，不能仅按函数名猜过滤。新增真实 World 单孔测试待编译与原执行对照。

矩形目标孔实现进入首轮：短孔使用 `Curve2::circular_slice`，长孔使用 `Rectangle2d::try_as_curve2`；逐段调用现有 `new_straight`，以原矩形顶点距离 0.05 严格门分别得到端标志，direct=0。原 `1412447ed..87b` 对 x/perimeter 取 fmod 并在负余数加1，NaN 输入走 panic；`141244b00..c79` 计算方向/长度、中点和端标志。此轮记录时尚未编译；后续 World 回归结果见下文，逐位原执行对照仍未完成。原 PDB 还存在不同 owner `FinalizedHoleStorage::holes_no_duplicates` 与 `FinalizedWallHoles::holes_no_duplicates`，前者在抹灰 generator 为 Filter，不能按同名跨用；此边条调用后 loop 的直接步进与 kind 门仍需专门反例验证。

矩形 mesh selector 与数据内 rank 不同：`141244bd3` 传常量 4 给 mesh selector，`new_straight` 自身 w30 为1，已拆单实例显式 mesh 参数，事件/圆分支仍取各自 w30。半木主函数 caller `[rsp+64]/[rsp+72]` 是另一段 f32 slice：`1407583a2–530` 从 PublicWallState+0x90/+0x98 点数组按 Vec3 12B 步进取 +4（y），分配并按原顺序复制；不能将 Vec3 内存直接作连续 f32 切片。该输入的现有 prepared_path 身份仍待对照调用体与后续 consumer。

木边条当前源码定向真实 World `/private/tmp/m6-floor-border-world-v1.log` 4/4：原有父实体两个生命周期测试及新增圆、矩孔各一例；圆核 direct0 和 mesh=实例w30，矩核 direct0、实例 w30=1 与 mesh AtlasPart(4)。这验证当前生产实体组装与基本数值，不是原程序全字段逐位差分，不覆盖 wrap/NaN/退化/真实 Nani GPU，也未证明完整墙体 producer。

窗下原图案 `add_underwindow_pattern@141fd1c30` 已另取完整 ReVa 反编译（`/private/tmp/m6-underwindow-decomp.json`，169行）及 B 指令全 371 条（`/private/tmp/m6-underwindow-asm.json` 与 `-200.json`，末页 truncated=false、undecoded=[]）。原主调用 `14121ad37`，它直接向 WoodenPlankSsbo writer push，而非现占位签名的 Vec<[u8;64]>。原门 `byte+0x11 & 0x1e != 0x16`，且仅高度差严格大于0.5、小于2.0时进入；构造三组斜梁端点、使用 wall-space→world 和原 ShapeParameters 法线，逐梁检查圆弧偏差并写 SSBO。实际回调捕获、seed、矩阵和其余参数尚待指令数据流闭合，不能按现假签名直接恢复或将反编译完成视作业务实现。直接调用可用 ReVa 只读 MCP，旧独立 worker 已关闭；operator shared proxy 当前连接关闭，故上述文件来自已连接的只读 MCP，原代码身份仍须与 B 图分页交叉。

窗下端点有界原执行：`m06-underwindow-prefix-oracle.c` 在 Rosetta 载入展开的原 RNE，校验入口及截获点原字节，执行原 `141fd1c30..1e3e` 指令；在首次 writer 之前截取栈内 12 个 f32 并按原序恢复保存寄存器退出。六组门控/端点结果 `m06-underwindow-prefix-oracle.txt`。发现原偏移的精确位型是 `be23d70b`，不是常用 -0.16 字面量 `be23d70a`；已修墙面端点前段，并将非常规坐标的 12 位型写入当前单测。该探针主动截断在世界投影之前，不能验证生成真实木梁、seed或 writer，也不代表 Windows 实机；构建 `clang -arch x86_64 -O0 <源> -o /private/tmp/m6-underwindow-oracle`，受审查权限运行并先按前文 SHA 校验展开映像。

窗下 writer 输出末尾 `141fd22b5–2ed`：同一 caller `[rsp+72]` 栈槽经 `1400885d0` 两次取值，首值低32与 caller `[rsp+80]` 低32 wrapping add 写 record+52，第二值低32写 record+56；record+48/+60 写零。该 identity helper 含 ICF 别名，须回追两个调用者参数真实类型，当前不把猜测名称写入业务 API。

后续追参闭合 writer seed：`141218eb2` 把原 WallId RDX 存 rbp+1304，`14121acd6` 取该槽传窗下 caller `[rsp+72]`；`14121aad9` 把外层迭代索引 RBX 存 r15，`14121acea` 传 caller `[rsp+80]`。因此窗下记录 +52 为 WallId 低32与外层索引低32的 wrapping 加法、+56 为 WallId 低32，+48/+60 为0；后续必须保留该次序和位宽。

+180 生命周期已追至原有效墙更新：`140a8c7fc–848` 将 remove_entry 旧 PublicWallState 672B 拷至 rbp+2112；`140a8ca4e–a55` 对 rbp+2488（旧记录+0x178）调用 `14087f390` 的 allocation drop，后者 `14087f3a5` 检查结构+8，即旧记录+0x180。因而有效更新替换旧 public row 会释放半木槽；无效 source 路径先跳过，应保旧状态。当前 sidecar 是独立 owner，须由真实 publication 边界同步重建/释放，不能笼统在空事件清所有墙槽。原汇编 `/private/tmp/m6-public-asm-1000.json`、`/private/tmp/m6-allocation-drop-asm.json`。

roof_outline owner 已另核 `141217ceb` 同取 PublicWall+180，`141217cfe..d12` 加锁至 `141218924..943` 解锁；主 half 清并写后，roof_outline 重新加锁追加。现加 `half_timber_append_writer` 显式借用原槽的 `append_writer` 保留末块，避免重置哨兵；待下一批次编译和验证。

当前源码同一构建批次 `/private/tmp/m6-underwindow-prefix-tests-v1.log` 1/1：门控、12 个非典型端点逐位与原 prefix probe 一致；同一新编译测试二进制 `/private/tmp/m6-half-append-tests-v1.log` 1/1，重锁追加两条落同一尾块；`/private/tmp/m6-floor-border-world-v2.log` 4/4。原 roof 尾块选择由 `141217d17–d46` 与 `d63` 确证。上述均为有界结果，DeleteRoofCmd 是否必触发有效 publication 更新仍未证，世界坐标及主生成器未验收。

`add_underwindow_pattern` 现按原输入/输出改为候选真实 writer 路径：原端点前段后逐段做墙面参数 wrap、原 Curve2 到世界空间、形状法线与圆弧偏差门，再计算梁旋转/偏移/尺度并 push WoodenPlankSsbo；生成标志与容量失败标志分离，尾字使用已闭合的 WallId/索引。此版尚未生产接线，矩阵和特殊浮点顺序未与原程序整函数输出比较，也未编译，仍不能计作最终完成。

窗下完整记录有界原执行：`m06-underwindow-full-oracle.c` 以原 B 映像与原函数执行，构造原曲线内存（两点 `[0,0],[4,0]`、参数 `[0,1]`、长度4）、原矩形形状，桥接 fmodf，并仅在每次 push 前截取 64B、模拟写入成功后继续原循环；`m06-underwindow-full-oracle.txt` 为3条完整记录。生产实现同一曲线/形状和真实 Sparse writer 输出48个 u32与原逐位一致（`/private/tmp/m6-underwindow-original-diff-v2.log` 1/1）。首次 v1 因无 bytemuck 测试依赖未编译，v2 改安全按字段取位型。探针不验证真实 allocator 的 push、圆弧/失败路径、Windows libm、生产调用与其他输入；后续共享 rotation 有新改须复验。

删除屋顶需保留独立边界：原 `mark_roof_for_deletion@14062f3d0` 消费 DeleteRoofCmd 后插入 RoofMarkedForDeletion 和发送音效，`delete_roofs@14062f8c0` 动画结束才发 despawn；原反编译 `/private/tmp/m6-mark-roof-decomp.json`、`m6-delete-roof-decomp.json` 没提供直接墙重建证据。此前有效 PublicWalls 更新释放+180的结论不证明每个 DeleteRoofCmd 都必定立即产生该更新，真实路径仍待继续追。

原世界投影后的 SIMD 平移运算 y 轴也经过 x0y 生成的 +0 偏移两次求和后再加中点，后续源码已保留该显式顺序；这发生于上述逐位测试之后，须新一轮编译复验，尤其负零边界未覆盖。

`/private/tmp/m6-underwindow-original-diff-v3.log` 复验2/2（包括前述原3×64B逐位与 +0 顺序）。圆形原探针首轮退出139，原圆函数调用 sin/cos 的 PE 外部 thunk 尚未桥接；补受控 fmodf/cosf/sinf 桥后原程序正常退出，半径2圆形加直线墙样例结果 `0条、generated=1、failed=0`，源/结果为 `m06-underwindow-circle-oracle.c/.txt`；生产侧同样例测试已添加、待编译。此样例只验证圆弧偏差跳过，非圆形通过门时的矩阵逐位验收；所用宿主 libm 不是 Windows UCRT。

原屋顶关联系统 `wall_roof_hashmap@140a94cd0` 的 typed SystemParam 包含 `Query<(Entity,&Roof,Option<&RoofMarkedForDeletion>)>`、PublicWalls/WallEntities、`EventWriter<RecalculateWallBricks>` 与 `EventWriter<RoofIsInvalid>`；本地 `WallRoofPresenceChanged` 是适配，不是这条原事件。原体完整反编译 `/private/tmp/m6-wall-roof-map-decomp.json`；typed 包装来自 `/private/tmp/m6-wall-roof-map-search.json`（搜索项截断，仅引用已返回的具体包装）。其 map 差异后写原重算事件的分支还需追清同墙原+180槽在 DeleteRoof 单独发生时的显示/释放语义，不能据此凭空 drop。

半木主函数替换旧6参占位，使用墙/WallId、真实 DecoratorStorage 与 DerivedDecoratorInfoView、TerrainHeightsData、两段真实 f32 切片、Roof、finalized hole、demo、原库/独立分配和 typed curved 输出。当前仅入口和层高前段：原 demo/非 premade 早退，按原顺序读取 roof 两条件，先从真实库和分配取末块 writer、锁内 clear，再 `max_y_gable`/`max_y` 调既有 split_height_into_skirts、gable 时追加 `max_y` 稳定排序，过滤 `< floor+1`，平顶追加 `(max_y + beeb851f) + be4ccccd` 否则追加选择顶高，首插 floor。`14122cd30` 排序比较对 NaN 返回 unwrap panic（`/private/tmp/m6-skirt-sort-decomp.json`）。这个前段新改未复编译、整主函数仍明确 TODO，不得启用生产 callback。原第10机器参数仅窄链捕获且未消费，按中央要求不强行填假资源；原外层仍须保真实 SystemParam 合同。

主半木层区域继续接原闭包：圆形 `from_iter@141204ff0` 以层高相邻对生成 `[0,lo,wall_length,hi]`；矩形 `closure$5@141fd18a0` 外层4边、内层相邻高，x由 get_point_u(i/i+1) 乘墙长；gable边不裁高但仍调用 max_y，其余边按原 minnum 裁至 max_y，低边不小于 max_y 时不输出。原完整C `/private/tmp/m6-half-circle-box-decomp.json`、`m6-half-rectangle-box-decomp.json`，矩形机器码首200条 `/private/tmp/m6-half-rectangle-box-asm.json`，后续23条待取，分支及浮点边界仍待原执行差分；后续写梁主循环明确 TODO，不能接生产。

矩形区域闭包 `141fd18a0` 有界原执行：按原内存投影设定矩形4×2、墙长12、层高 `[0,2,5]`、max_y=3、gable边0/2，只桥接 Vec grow 后取原8条 AABB，共32个f32位型；源和输出 `m06-half-rect-area-oracle.c/.txt`。当前 `half_timber_areas` 添加完整8条逐位样例，待编译；原长尾23条指令已补 `/private/tmp/m6-half-rectangle-box-asm-200.json`。该闭包探针不验证半木后续随机和生成。

窗下写入失败：`m06-underwindow-exhausted-oracle.c/.txt` 模拟 push 返回失败，原循环继续3次，最终 generated=1、failed=1。生产真实 Sparse writer 的 free chunks 为空反例已加入同一测试，待编译；探针只模拟返回值，未复刻原 allocator。

当前源码 `half_timber` 过滤批 `/private/tmp/m6-half-regions-native-test-v1.log` 11/12：矩形区域原8 AABB逐位、窗下、roof及槽测试通过，旧 `overlapping_windows_reports_shifted_flag` 失败（0 vs 1）。原 `1412218a0` 完整反编译 `/private/tmp/m6-overlap-window-decomp.json` 和293条汇编 `/private/tmp/m6-overlap-window-asm{,-200}.json` 表明原直接相交追加后仍检查移位相交，原实现中的 continue 会丢失两次命中；已移除。旧测试选择的 center=1、半宽约0.31、expansion=2 满足原跳过右移门，本不应有结果，现改用 expansion=1、查询中心0的左移命中，并加 expansion=0 原位+移位双条反例。改动待复验，现有辅助函数仍使用调用者提供的 WindowAabbSource 投影，不代表真实 DecoratorStorage 主生产链已闭环。

`/private/tmp/m6-half-regions-native-test-v2.log`：当前源码半木过滤组12/12，包含矩形区域8条原32词、窗下3×64B原记录、圆弧跳过、真实 free chunks 耗尽标志及窗口双命中。`141221b1a–1e` / `1bd5–d8` 的 UCOMISS/JBE 在无序时也选择分支，已按该跳转恢复；这些样例仍不构成 half_timbering 完整生产验收。

主流程续核：首 RNG seed 的 `rbp+1216` 不是未知字段：`141219bff` 清0，`141219e48–53` 形成+1并在 `141219c40–47` 用于下一16B区域。`14121aa95–ab4a` 的 XMM10/XMM7 是该区域 splits 向量相邻f32；故 `14121ad9e–ae49` 为 WallId低32 + 区域索引低32 + 两个坐标各乘10后经原饱和 u64 转换取低32，依顺序 wrapping 加法。原首个 baseline 由 roof owner 完善所有公共状态字段后重跑为 `m06-half-main-oracle-flat-v2-20261029.json`，原入口正常返回57条 64B plank、0 curved；完整输入和桥接受限范围见该文件，原输出 `m06-half-main-oracle-flat-v2-wood.bin` SHA `dc8d789855552e2c918ee5d8166afa725ff5621c04318e00bc5fffa500737bdb`。原 terrain 512² 全0及原采样4次、原 RNG 均执行；不代表当前主算法已恢复。

主循环采样有界指令已分解（源码冻结期间只读取证）：`1412196c1–982c` 每层重置游标，count=`ceil(projected_curve.length/0.3).max(2)`，u 按序 `i/(count-1)`；在 points_u 上通过增量余量前进与求 t（`19784–7fb`），不是任意直接重查。按 t 插值调用者 segment_heights，相对0执行 UCOMISS/JBE；非正/无序不活跃，否则再插值 path_heights 相加、与该层高执行 UCOMISS/JBE；活跃段用 `u*curve.length` 创建或续写 `{start,end,flag}`，停用清层 flag（`19832–89d` 等）。切换/边界及错误路径仍须取完并与 v2 checkpoint 对照后落实现，不能以普通 point sampling 的近似运算代替增量状态。

主函数现补按原顺序分配逐层 flag/12B span，再构造高度轮廓 xz Curve2 并逐层增量采样；seg/path 两数组与原比较、区间创建/延伸已接成候选。此段当前尚未编译、非零原 checkpoint 尚待获取，异常或末段游标边界仍未验收；随机/splits/几何及调用全部未接。`height_outline()` 当前 None 路径回退 geometry.outline，原 PublicWall+90 未准备情况下的差异仍需主 owner 核对。

真实窗查询 owner 迁入半木：`DecoratorStorage::get(WallId)` 的真实桶迭代、Info+0x22 墙目的/Info+0x46 窗类型筛选、map key 实际 DecoratorId 查 DerivedDecoratorInfoView::subtype，非哥特窗查同 owner 的 `bbx_ls(subtype,rank)`，哥特窗调用已有原 profile，随后复用同一 overlap 算法与输出 Vec。原 141221a0e–52 显示 bucket -38=Info+0x22、bucket -2=Info+0x46、bucket -80=map key；原 bucket -4=Info+0x44，因此这处 leaf 为 rank（旧投影注释的 raw+0x0c 不是原 Info+0x0c）。真实 bbox leaf `141fdc7b0` 的单槽原 x86_64 执行由 bbox owner 交付：`m06-window-bbx-oracle.c` SHA `6ab2143dc7726377d03d9f1d8a8066a0ab34adde74fbc8eda8574cc34a858af1`，输出 `/private/tmp/m6-window-bbx-oracle.txt` SHA `5fdf755dbca52ddb9b46f95d45bf7b629a0f83ac545fba18deb7df7c582c2e67`，4组32B一致，不证明原hash/panic或主函数完整；该共享API由 bbox owner 维护。新主路径现按每区域全部真实查窗，再进入首区域原 splits；窗口合并/后段依旧 TODO，真实非空producer与原差分待验收。

原主样本进一步在 `14121aa95` 截获最终 splits：四区域分别 `[0,2]`、`[2,4,6]`、`[6,8]`、`[8,10,12]`，flags全0；完整原程序继续执行并仍返回57梁，探针及记录 `m06-half-final-splits-oracle.py/.json`。主函数已在真实 per-area 查窗后构造原初始分割，并接 `141219e90–1a110` 的未移位窗口直接/跨界合并；右界原指令 `maxss` 使用刚更新的 left 而非旧 right，不能按一般区间并集替换。该新增候选和真实非空墙桶/哥特窗测试均待编译及原对照，候选点过滤、洞扣除、随机、梁和 curved 仍未完成。

bbox owner 通知中央实跑 `bbx_ls` 原8词测试与 collision/interaction 四表冲突测试各1项通过，日志 `/private/tmp/root-m6-bbx-native-20261029.log`、`/private/tmp/root-m6-bbx-four-tables-20261029.log`；这不代替半木窗 producer 联调。

非零 segment/path 采样原入口在 `141219995` 截获：输入 segment `[1,2,3,1,0]`、path `[0,.2,-.1,.3,0]`，两层 flags均0、各一条 span；首层 `[00000000,413b13b1]`、次层 `[3fc4ec50,40e27628]`（16进制f32位型）。`m06-half-span-oracle.py/.json` 使用原B与固定v2投影，主动停在该 checkpoint；中央本轮lib 114/114已包含生产逐位对照。原平顶孔筛选闭包 `141206480` 的 mask 0x1a 取 kind 1、3、4，保留原次序；候选已在 writer clear 之后、层高计算之前接入，后续真实扣除仍待完成。

原全窗点查询 `141221f70` 另发现旧投影 helper 的移位盒高度错误：`141222028..05d` 从原窗口 center.y 组装移位 AABB，而不是当前的 query point.y。`141222151–157` 的 UCOMISS/JA 只在有序且超限时走左移；`141222173–177` 的 UCOMISS/JBE 包含无序跳过。源码冻结中已报中央，解冻后修复并加反例；原 C /private/tmp/m6-check-window-decomp.json、原指令 /private/tmp/m6-check-window-asm.json。

### 半木逐点竖梁候选（2026-10-29）
原 `14121b631` 将落在本层 span 的点标记后跳过竖梁；未命中进入 `14121b659–b9e6`，墙面坐标经原曲线与形状法线投影，比例为 `0.15 × abs(area.min.y-area.max.y) × depth`，矩形 depth 为 `0.15`、圆形为 `0x3d4cccce`，平移加两次法线乘 `0.15`。`14121b951/96e` 的 `1400885d0` 是读取同一 WallId 的 ICF getter：输出尾部为零、WallId 低字加点序号、WallId 低字、零；push 失败只设标记并继续。候选已接入未注册的主函数，但未编译或逐字节与原侧比较，矩阵细节和后续随机/区域分支仍待验证，不计主链完成。
原完整入口增补 push 返回地址追踪 `/private/tmp/m6-half-vertical-pushes.py` / `/private/tmp/m6-half-vertical-pushes.json`：57 条中 `14121b9de` 竖梁 6 条，`14121e413` 34 条、`14121d27a` 8 条、`14121dafd` 4 条、`14121f420` 4 条、`14121b5a2` 1 条。新 `vertical_plank_matches_original_entry_first_record` 读取冻结原输出首 64 字节与真实 wall/sparse writer 逐字对照；待集中编译执行，尚不计通过。
逐点循环后，`14121b9ee–aa95` 追加区域右端及未覆盖标志（山墙区域需通过高度门）；`14121aad0` 遍历相邻点，仅两端标志均为零且中点未落入平顶孔洞半开区间时进入后续窗口与随机几何。当前候选按此顺序推进到显式 TODO，尚待编译及原指令输出对照。
补充原 57 梁入口 `14121ae49` 随机初始化及 `14121ae68` 首值探针 `/private/tmp/m6-half-rng-samples.py` / `.json`：六次 seed `[143,184,224,265,306,346]`，首值位型分别 `[1043960104,1063262764,1064075548,1061742266,1057967616,1061666534]`，width/height 位型均 `1066082661`。仅原侧捕获；首次误把 `ae47` 指令执行前 RCX 当 seed，经 `ae49` 实际 call 边界纠正，未用于实现。
按仓内 `utils::rng::init_rng` 和 pinned `fastrand 1.8.0` 首次 `f32` 的计算式独立算这六组，六个结果与原侧全部逐位相同；这是首抽序列证据，不覆盖后续分支的随机消费顺序。
集中构建确认竖梁首条原 64B 对照通过（中央记录 full lib 133 项中该项通过；另有圆邻墙共享距离失败，非本测试）。区间候选继续按 `14121ac10–ad37` 选择尺寸差小于 0.05、中心在两端之间的真实窗；查询 area.min.y−0.1 后按原入参调用窗下图案（最后高度由 PublicWall.max_y 提供）。未生成图案时按 `14121ad9e–aec0` 初始化与保留同一个 RNG，概率门后续梁仍显式 TODO；该新增接线与六组随机差分尚未编译或对照，且不接生产。
`/private/tmp/m6-half-rng-second.py` / `.json` 在原 `14121af4f` 捕获同一真实流第二抽：仅 seed 143 的区间到达此点，位型 `0x3f2089a4`，新增测试要求保留首抽之后状态并继续匹配。其余五区间原概率门已退出；不据此推断非空 span/窗口分支的消费次数。
原 `14121af00–af4f` 首抽通过后先逐个调用 `141762b70` 检查当前层 span，再在全部不相交时消耗第二随机值；RNE 原 141762b70 机器码是 `span.end >= pair.left && pair.right >= span.start`（端点包含，无序比较均为 false）。候选按此接入，随后几何仍显式 TODO，未编译。
主流程新增原 `14121af3c–b5a2` 单区间斜梁候选：第二抽选端点高度、墙空间投影、圆弧偏差门、法线与稀疏 writer；同一真实 writer 的第二条与完整原入口第二条 64B 差分断言已加，尚未编译或验证。非空移位窗口相交仍显式 TODO，不能将空窗探针推广为完整路径。
移位窗排除补取原 `14121b010–b05f`：已查询的每个 20B 窗记录其 min.x 加 `0x3d4ccccd`、max.x 加 `0xbd4ccccd`，与 `[left, area.min.y, right, area.max.y]` 经原 Aabb2::intersects 测试；任一命中跳过斜梁，且此测试在第二抽之后。已用现有 Aabb2 原语接线，尚未原窗口执行差分。
`14121b9f0–ba3b` 仅 shape tag 0 将该区域的位置 Vec 以区域 index 为键插入 HashMap，矩形则释放 Vec；后续原圆墙遍历要求每项非空，若首值非零或 NaN 则前插 0。候选已保留这些区域状态并移动显式阻断到后续曲面/横梁/角柱，尚未原圆墙对照。
原 `14121bd13` 调 `141fd16d0`（closure11，292B）：依原 area index 先产出 `(index, area.max.y, area.center.x)`；若 `area.min.y == PublicWall.floor`，再产出 `(index, area.min.y, area.center.x)`，无序比较不相等。候选已按区域顺序展开并加有界单测；后续每条边界的扣除与真实梁生成仍显式未完。
后续边界非山墙门：原 `14121bd91–be8b` 平屋顶只在边界高度不等缓存最大高度时继续；非平顶为 `height < max−0.2 || max+0.2 <= height`。候选接此门并对山墙保留显式 TODO；边界几何尚未生成、测试待统一构建。
非山墙矩形边界的原初段 `14121c2eb–c4f7` 以 `center.x / wall_length` 查真实矩形边号，取该边两端 point_u、乘 wall_length 后创建 SubtractiveBoolean1D。候选复用已有原几何和区间 owner；洞窗扣除及输出仍未完成，圆墙仍明确 TODO。
区域边界原完整入口调用后检查点 `/private/tmp/m6-half-boundaries.py` / `.json`：矩形基准四区域各生成两条，依次 `(i,0x3feb851e,1/4/7/10)`、`(i,0,同中心)`；已加对应输出断言，待编译执行。
`/private/tmp/m6-half-boundary-ranges.py` / `.json` 在真实 `14121c505` 调用处捕获 8 次初始区间，依次 `[0,2]×2、[2,6]×2、[6,8]×2、[8,12]×2`；已针对 rectangle 的真实 edge/point_u 加逐位断言，待构建。
原 `14121cc30–cc71` 逐层扣除每个 12B span，返回完整区间后才生成边界梁。无洞窗候选已复用原 SubtractiveBoolean1D 逐项扣 span；非零真实原入口 `/private/tmp/m6-half-boundary-clips-nonzero.py` / `.json` 返回8组依次 `[0,0x3fc4ec50]`、空、空、空、`[0x40e27628,0x41000000]`、空、`[0x41000000,0x41400000]`、`[0x413b13b1,0x41400000]`，已加逐位断言待集中验证；有洞窗仍保留显式阻断。
非哥特窗边界扣除候选已依据 `14121c653–c690`（窗中心 x 与边界高度+0.2 contains）及 `14121ca20–cbcd`（原窗先扣、越周长重建镜像窗再扣）接真实窗和筛选后的平顶孔；随后才扣层 span。窗 6/9 原哥特截面仍显式 TODO。该有窗/洞候选尚无原执行样本，不计验证。
真实 finalized 孔样本 `/private/tmp/m6-half-boundary-clips-hole.py` / `.json`：一条 kind1 AABB `[0.5,0,1.5,2]`，原入口正常完成，两条首边界扣除结果均 `[0,0.5],[1.5,2]`，其他6条各保持原边区间。已将主流程的同一扣除函数抽成内部 helper 并以这8组逐位断言；尚待中央编译及执行。
非哥特窗局部原执行 `/private/tmp/m6-half-boundary-clips-window.py` / `.json`：在真实边界窗口遍历 `14121c653` 前注入 20B 查询结果 `[0.5,0,1.5,2], subtype0`，其余指令及一维扣除原样执行。首边上界高度+0.2 在窗外，结果 `[0,2]`；下界在窗内，结果 `[0,0.5],[1.5,2]`。已用相同候选 helper 加断言待编译；这只证明消费投影，不证明原 DecoratorStorage 的生产与枚举。
哥特窗 6/9 的边界截面依 `14121c6c2–c9b9` 使用真实 rank 剖面并反向，按窗口 y 映射查半开曲线段，再插值半宽；door subtype9 且 rank1 额外加 0.14，半宽须大于 0.05。原执行局部窗口注入 `/private/tmp/m6-half-boundary-clips-gothic.py` / `.json`（type6 rank1）下界区间得到 `[0,0x3f051eb8]` 与 `[0x3fbd70a4,2]`，上界未扣；候选已加逐位断言但尚未编译。样本仍是消费者投影注入，不是完整装饰器生产链。
同样注入 subtype9/rank1 原门窗消费者 `/private/tmp/m6-half-boundary-clips-gothic-door.py` / `.json`，下界结果 `[0,0x3ea3d70a]` 和 `[0x3fd70a3e,2]`，候选逐位断言已补，待构建；同样不证明完整生产者。
矩形边界主横梁 `14121cf84–d27a` 采集原寄存器和栈 `/private/tmp/m6-half-boundary-geom-full.py` / `.json`：上/下边界分别使用法线偏移 0.05/0.2、深度 bits `3f11eb86`/`3ea3d70b`，切线起点外移 0.12、Y 加 bits `3e23d70b`、长度加0.2，顶层 flags4。候选与原57条中第8条64B比较已加，尚未编译。其后细梁/曲面及角点未恢复，不能视为主链完成。
后续细梁门原 `14121d374–d388` 在末层调用 `ClutterVersion::is_initial`；其指针在 `14121bc44–bc78` 明确由 Roof 参数 +0x3d 取得，而非 wall 的 source_69。非初始版本及非末层不能套用基准细梁规则。矩形细梁原 `14121e...` 样本数取 ceil(端点实际距离 / 0.4)，最少2，逐点从同一 WallId 初始化的 RNG 消耗随机；现未完整实现或验证。
原细梁采样排序前检查点 `/private/tmp/m6-half-fine-points.py` / `.json` 得到4个末层区域 6/11/6/11 个 24B 点，其他4个下边界均0；`/private/tmp/m6-half-fine-inputs.py` / `.json` 捕获第一区域修正端点 z=±`0x3f921a18`，随机位移因子 `0x3cb36ce9`。候选已按原固定顺序生成6字段点列并加首6条逐字断言，尚未连接后续排序和 writer、尚未构建；不是完整主算法。首版探针曾因临时变量覆盖 points 在结果保存时抛 ValueError，已改唯一变量重跑并保留正常结果。
末层 Roof+3d 初始版本的矩形端点修正与顶层装饰梁候选接入主流程：按原角点周长位置 ±0.01 门，用切线乘 `0x3e10d0c3` 延伸；真实 writer 增原基准第9条 64B 比较，之后以同一 WallId RNG 生成细梁采样。顶梁与采样目前尚未运行对照，采样排序/逐根 writer、圆墙与山墙保留明确 TODO。
矩形末层后段候选现按原先累计采样、稳定按 u 排序、相邻点三维距离 <0.2 跳过、同一 RNG 再抽偏移后写细梁；原第一顶层6条完整 64B 已追加同一真实 writer 比较。源码尚未构建，排序/随机/位型及多段行为均待原对照；圆墙和山墙仍未完成。
矩形角柱候选按原 terrain 采样、prepared 曲线投影、segment/path 高度插值、flat 顶界和孔门构造区间，写 `.4 × height × .4` 木柱；非空角点装饰桶仍显式 TODO。原完整57条末4条64B比较已加，未编译或验证。另发现共享 `Roof::is_flat_w_no_rim` 与 B `1408e3390` 相反（原要求 roof+3d 非零），已报中央，本文件角柱暂直接执行原判定，不改共享 owner。
中央已核并修正共享 `Roof::is_flat_w_no_rim`，本文件现改用该唯一 owner。原角点装饰桶顺序为每角 terrain/区间初始化之后查询；只对 type8 且 derived subtype3 执行专属扣除/辅助梁，候选目前在该真正未恢复分支显式 TODO，其他装饰类型不误阻断。原侧空装饰角柱 4×64B 测试已就绪未编译。
新增真实 `half_timbering` 入口 fixture：经真实 PWS/terrain/空装饰/roof/ConstructorGpuOutput，要求全部 57 条原木板逐条64B及 curved=0；尚未运行、不声称通过。非零29、洞窗、圆/山墙与角点 subtype3 仍须完整验收。
圆墙原完整入口证据由协助 owner 保存 `m06-half-main-circle-native.json` / `-wood.bin` / `-curved.bin`：原 Circle2d::as_curve2(0.9) 14点，真实 PWS 长度 12.4444160461，flat/空洞窗返回36 wooden 与21 curved；非零高度输入另30 wooden 与19 curved。曾有 JSON 元字段因脚本变量覆盖误记 length=输出数，已由 owner 纠正，原机器输入和输出未变；这均是原侧样本，候选圆主链尚未实现。
圆墙区域路径已接到真实保存的墙距位置：原 `1412021c0` 每项除 PWS+28c 才得到 [0,1] u，`14121e4af–e52f` 排序去重后相邻成段；候选按原墙距归一化、排序、逐对乘回墙长后进入真实扣除。原空装饰基准对应7段；有装饰窗时额外插入坐标尚未闭合，显式 TODO；曲面生成/细梁未完。
圆墙无洞窗边界现接真实 CurvedHalfTimberInstanceData XY/XZ：原 `14121cea5–cefb` 保持 scale=TAU/L、mid=((b+a)*.5)*scale−rotation、half=((b−a)*.5)*scale 后减/加的次序；原直接常量主顶 XZ length .68/w bits3ea3d70b、地面 XY length .1、顶饰 XZ length/w bits3df5c290。顶饰要求 Roof+3d=0，no-rim 顶界下移 .2。基准首2顶与首地面3×64B原输出测试已加，待构建；窗口/孔扩展和圆细梁仍明确 TODO。
圆墙空洞窗主入口候选接入圆弧采样、按原周长法线计算和后续随机细梁 writer，并添加全入口36×64B wooden +21×64B curved 对照；完整原输入 `m06-half-main-circle-source.bin`（从同一 JSON `source_2560` 提取，SHA256 `5a80fce714460a3307c58d04861c8b3d13358b70e6277485cb5886288e90a5fd`）。该候选尚未构建或执行；孔洞跨接缝 extension、非空窗坐标和山墙仍显式未完。
圆孔端延长原 `14121d39a–d6f1`：对已筛 flat_holes 每端先除墙长并环绕，左右端分别与洞两端比较双向环距，任一严格 <0.05 则对应延长0.1、XZ flag置1/2；XY同角度、无flag。圆细梁数用扩展后的角跨度；已接候选并加跨缝/内部洞端点与 seam 首2曲面64B对照，待中央构建。原完整孔样本 `m06-half-main-circle-{seamhole,middlehole}-native.json` 与 `-curved.bin`，不能据此称窗/山墙完成。
原 half leaf 正常尾部 `14121f61a–f68f` writer mutex 解锁为 `mov al,1; lock cmpxchg [rdi+24],cl`，成功路径至 ret 不加载失败捕获字节；AL 是解锁指令遗留，不能称为外部容量返回。原 caller `140758e2f` 后直接以 Roof 调 is_gable，未读取 AL。原 push 失败仍设置局部捕获并继续，成功前缀保留；不添加 caller 失败门，慢解锁路径尚未另核。
中央 round9-v2 `/private/tmp/root-m6-three-support-half-parent-round9-v2-20261029.log`：独立圆曲面（含跨缝扩展）、六点细梁 checkpoint、边界/洞/哥特窗口与随机测试通过；矩形局部逐条对照前10条通过，角柱及圆全入口首木高度各比原少0.46。原因是 Rust `PublicWallState::collect` 重算缓存为2.04，而原探针 PWS+10c 明确为2.5；已仅修两个 fixture 的权威缓存成原输入2.5，不改 expected，待复验。
增加非零29×64B完整入口候选差分，`m06-half-nonzero-flags-wood.bin` 从既有原 JSON 提取（SHA256 `1981ac5774e34d627fe9128c40415f17268276bfe0e74f4fc12d181e5b132906`）。原探针独立传入非零 path 数组而 PWS prepared Y 为零，故只覆盖合成 ABI 入参，不证明真实 producer；未运行。
原协助 owner 取得真实 hashbrown wall→Decorators→id42、derived subtype9/rank1、墙距0.2/y1 的完整圆墙哥特窗入口 `m06-half-main-circle-gothic-{oracle.py,native.json,wood.bin,curved.bin}`，原返回34木+21曲面。现有圆墙入口测试增加真实 Rust 装饰桶对应输入并逐64B对照，未运行；该样本不同于先前注入消费者投影。
中央 round10 `/private/tmp/root-m6-support-and-half-full-round10-20261029.log`：矩形完整57条已全部通过，非零29首差在细梁第2条 Y 差1 ULP；圆36/21首差木板第15条 X 差1 ULP。复核 `14121e22e–25e` 为 `normal*.2 + (perp*lateral + point)`，不能交换结合顺序；写出 Y 使用边界高度而非采样插值点 Y，原非零首点 checkpoint Y=0x3feb851d、writer输入基准高度0x3feb851e。候选已修，待复验；宽度/近邻比较依 UCOMISS/JA 保留无序路径。
原矩形山墙六区域筛点样本 `m06-half-main-rect-gable-filter-native.json`：`14121a623–a756` 屋檐以上按 Roof.floor_y 与实际 Roof.height 做 inverse_lerp，再由真实 `shape_at` 的互补轴尺寸−0.3 截面筛选，>=1去除；末点追加另经 max_y 门。候选已接并对六区域加原 checkpoint 测试（roof 88B提取 SHA `74165449f4d652c1aab09d223d04ba5bcdf8853a75fa19c19899cbfb8d53988c`），未执行。边界后段与 gable/circle 等仍未完成。原 -0.16 高度修正的争议由 14121bf5b 机器码常量 `0xbe23d70b` 确认。
矩形山墙边界候选继续按 `14121bd66–bfb2` 的 regular/山墙边号/max_y_gable 门执行；高于 max_y 时 `14121c448–c4f7` 依 Roof.floor/height 与真实 shape_at 截面，互补轴尺寸−0.3，宽通过0.5门后居中生成区间。原末层标记只在 flat 分支设置，现已纠正。山墙角点和圆墙山墙高层仍 TODO，候选未执行对照。
中央 round12 `/private/tmp/root-m6-half-full-output-round12-20261029.log`：矩形完整57及合成非零29逐64B全过；圆墙36条第35条 Z 差 0x3d7b847f vs 0x3d7b8421。原闭合曲线首点 Z=3.4969e−7、末点 Z=0，原 `14121df53` 直接构造墙距坐标而非 fmod；候选误将 length 环绕成0。已去掉该多余环绕，保留端点与 expected；圆曲面及真实哥特窗尚待继续复验。
原 subtype3 角点装饰 `14121ebee–eca9`：真实 bbox 几何尺寸为 (footprint.size.x, top−bottom)，AABB 中心为 dst 墙空间 (u,y)，不叠 bbox center/bottom；用 (corner_wall_distance,AABB.center.y) 测包含才扣 y 区间并生成四条 f102。候选已推进真实查表、bbox/dst 与该包含门，仅命中分支保留显式 TODO；4梁与扣减仍未完成。
原 seam hole 的 `14121b9f0` 点列不含0。重核 `14121a440` 确认窗口内外两条路径最终共同检查 flat_holes；旧候选误只查区域外，已移到两分支之后，并用原 seam 的7个墙距 bits 加断言。区域内窗口判定也按原两项排除比较保留无序分支；待集中构建。
真实 Rust collect→prepare_paths→update_gable_heights 的相同导出输入已由原 half 入口执行返回21木/0 curved：`m06-half-main-gable-published-input.json` / `-native.json`；原非调用字段零填、仍不代表原 publisher 整链执行。原21木提取 `m06-half-main-gable-published-wood.bin` SHA256 `3bd45824f874e698a568e3e604fe1ad5977ba481e1ddedeb581066cc62e30805`；原真实 PWS 测试已追加 Rust 全入口逐64B比较，尚未运行。
已按原 subtype3 命中分支接纵向扣除、角点两条边方向×窗上下端4条辅助梁，再写剩余角柱；新增真实装饰桶/真实 bbox owner 的8×64B局部原对照，原合成 gable ABI fixture `m06-half-main-rect-gable-subtype3-wood.bin` SHA `23e63671d5f5c488eb7dc6a62322a78e37cd50ae7bc10605c2deae874d897a58`。上述候选未构建/对照，输入合成而非合法 publisher；完整真实输入样本另见 published fixture。
圆墙完整跨缝孔 fixture 现用真实 finalized kind1 输入并要求33木+21曲面逐64B；真实哥特窗 fixture要求34木+21曲面。两项均尚未构建，输入来自各自原完整入口 JSON。山墙宽截面筛点补原保留反例；roof 输入提取 SHA `4cb8f9fc02679b81e9736144d46a6b5857ee1285425439ad4fcacf4c9e879f4e`，同样待执行。
中央 round16 `/private/tmp/root-m6-full-gable-windows-round16-20261029.log` 全 wall-constructor lib 172/172：已核测试完整控制流，矩形平顶57、合成ABI非零29、真实bbox/装饰桶的合成gable subtype3角点8×64B；圆空窗36木/21曲面、真实哥特桶34/21、真实 finalized 跨缝洞33/21；真实 collect→prepare_paths→update_gable_heights PWS 相同输入的 gable 21木/0曲面，均逐条全64B通过。上述为界定样本，不是原完整 publisher执行、正式 App 画面或所有分支完成。
真实 published gable 相同PWS结合原真实 hash/bbox subtype3：`m06-half-main-gable-published-subtype3-native.json` / `-wood.bin`，原返回24木/0 curved，4辅助梁及3角柱；真实PWS测试已添加24×64B对照。另圆墙真实入口新增 SSBO free栈耗尽时木板0、后续21曲面仍逐64B不变的测试。两者尚未运行，耗尽曲面预期来自空洞非耗尽原输出及原 push失败后继续的指令合同，非完整原耗尽探针。
容量边界测试新增实际只留一个32槽块：要求保留原圆墙前32条木板并在后续 push失败后仍完成全部21条原曲面逐字比较；加上零块测试。预期依原 push 失败继续循环的机器码及同输入非耗尽原输出；新测试待构建，尚不是完整原耗尽执行样本。
补充有界原耗尽执行：`m06-half-main-circle-exhaust32-oracle.py` / `-native.json` 仅将原 SSBO push bridge 第33次起返回失败，余下业务指令原样执行；原正常返回、尝试36次、失败标记1、前32条与非耗尽逐字相同且21曲面全同。桥接模拟 push 结果而非真实原分配器；Rust真实单块/零块测试仍待运行。

164: 独立复核确认 B 原归属仍为 `crates/systems/wall-constructor/src/utils/half_timber.rs`；当前真实圆墙山墙导出经 collect→prepare_paths→update_gable_heights。后者对非矩形只设置普通最大高度，故导出断言两种最大高度相同，不应手工构造矩形缓存。圆墙山墙原入口、published subtype3 全输出和真实零块/单块耗尽候选仍等待中央集中构建与原侧对照；既有 round16 的172项结果不覆盖这些新测试。源码顶部过时的投影输入说明已修正为真实装饰表接线；没有把有界样本升级成 M6 验收。

165: 独立源码/既有证据复核清单：生产 `construct_walls` 已按 style/roof 门调用 half、山墙时追加 roof_outline、随后发布 curved 事件；underwindow 在区域窗匹配后实际调用，floor border 在 curved 事件消费者调用。当前 round16 对平顶矩形、圆、真实哥特桶、跨缝洞和真实出版矩形山墙提供有限完整64B对照；并未覆盖以下组合：
- 层高 ≤0.75、重复/相等高度、非有限和空位置序列；尤其圆山墙高层过滤后可能清空 positions，现实现补零时会失败，须用真实出版输入与原入口确定是否同样失败。
- 多窗重叠与跨周长镜像、多个不同 kind 洞、窗和洞共同裁剪、边界相等与排序；单洞或单窗成功不能推得组合正确。
- 区间 RNG 的概率门、span 排除后第二抽、顶层采样和写细梁的连续消费：已有六组首抽及单样本完整输出，但未覆盖多段裁剪、非初始版本及失败后随机序列。
- 圆弧下窗曲率允许写入的完整矩阵、矩形山墙角柱的多个 subtype3/洞/地形组合、斜梁圆弧门与旋转；已有窗下矩形3条和圆弧跳过探针并非这些组合。
- roof_outline 的退化轮廓/边长、方向和异常及容量耗尽；wooden_floor_borders 的绕周、端帽、退化、非有限和原64B逐位差分；真实 GPU、历史、保存及卸载由中央联合验收。
容量原桥只模拟 push 返回失败，已证32成功后继续曲面，真实原分配器与零容量原入口仍需分开验证；不可用局部 World 或样本通过替代全链验收。

166: 中央 round20 全 lib 175/175（`/private/tmp/root-m6-support-half-round20-20261029.log`），含真实圆墙山墙出版导出、published subtype3 和零/单块容量候选。导出经现有转换器和 `m06-half-main-export-circle-gable-oracle.py` 执行 B RNE `f507a6ce…`，正常返回7木/7曲面；真实出版输入 `m06-half-main-circle-gable-published-input.json` SHA256 `d93d26bfc63a524d6d4bebadbf544980f5f2381587a352d5b99800003c43a988`，原输出 `m06-half-main-circle-gable-published-native.json` SHA256 `ef7e6ad18dbaeb43a75663c6fa6b9867642d3996ccb9753b4a2f660dcc61066c`。原高层位置序列含一个末点，未触发空序列失败；对应7×64B木和7×64B曲面候选断言已加，待中央新构建。探针使用 Unicorn、宿主 Darwin libm 与模拟成功 push，不代表原 Windows CRT 或完整原 publisher 执行。首试沙箱进程 SIGILL，获得执行权限后同输入正常完成。

167: 复用既有 `m06-half-main-circle-rim-native.json` 的原平顶 roof+3d=1 样本，追加候选完整7木/14曲面逐64B断言，使用既有 `m06-half-main-circle-rim-{wood,curved}.bin`，SHA256 分别 `854b07e76ca5c235acc93f9d819f289c6ac6c6c0cbe8b6a385f32e584d0f5c31`、`364763f87cfa93ccb00200a9591e16e247da9e2ad67f8a6dd94e2e2f12adddb2`；尚待中央构建。该原输入为原内存投影，并非完整原 publisher 执行。新源码 SHA256 `c48f4c1086d460718817ee3c7ebaf2540cd005cf0a6b59d6819c048bb9de962c`。

168: 扩展矩形双窗双洞原入口差分：复用真实出版矩形 PWS，将 roof 高度改平顶；手工构造两个真实原 hashbrown 窗记录（id42/43，subtype3、rank1、墙距0/4、高1）及 kind1/4 finalized 孔 `[0.25,0,0.75,2]`、`[5,0,5.6,2]`。`m06-half-main-rect-multi-oracle.py` SHA `ba143dd888a40617f8e18b3c5b108b9d2e6a1bd0aa665fecb0b6a36756e3c336`，输入 SHA `842ea0ebda5c1088b217be0ed98900ece4f60371de43e3aee63c13387dd2eaf7`，Unicorn B RNE正常返回59木、0曲面；原记录 SHA `4008ee7650cfd9dbd167a6311e759a8a317957a72e3c0152c5b4504c623d37ce`，木板bin SHA `d7b1f07f2597819e055060cefb3c8b9e4d7f1352a7d9d261c913a577425e105e`。写站点分布 6竖、1斜、12主横、6顶横、27细梁、7角柱/辅助；RNG 原执行未桥接，数学库为 Darwin，push模拟成功。候选真实 Rust 装饰桶/洞与59条64B断言已接既有测试，待集中编译；容器布局手工投影并非原 publisher 整链。新增真实出版 height=6 平顶导出，待中央运行以继续多层差分。

169: round27全 lib 177/177，含圆山墙7木/7曲面、圆 roof+3d=1 的7木/14曲面，以及矩形双窗双洞59木（见日志 `/private/tmp/root-m6-half-branches-round27-20261029.log`）；round27之后新增的矩形双窗双洞 roof+3d=1 的26木断言尚未构建。其原完整输出 SHA `9866db812e42a413f699c586c916cb66717fc484495cf55a86eb0d1c52a04b64`，wood SHA `da70b1eff496a9e1a7fa7ff614e3fb4aa50b2dcd3991522d1387d4cce00be264`，原写站点无顶饰/细梁。
170: round27真实出版低墙及较高墙输入再用同一双窗双洞 oracle：前者输入高度0.6，实际出版 max_y=0.960000038，原52木；后者输入高度6，实际 max_y=5.53999996，原67木。原木板 SHA 分别 `f424e565be95bf3e6236c336e8b9ae080a01d1e19b518b3114313522cb08af32` 和 `85332ccb5f2080186fe8a3f5d118f9f9726cbca75772b7995721d88bcf8ab19d`；各自完整原结果、输入和bin存既有目录，逐64B测试待编译。注意6米输入的 max_y 未达5.9分层门，不能称多层；另增8米真实出版导出，待取实际结果。以上多窗 hashbrown 为手工构造原布局，原输入属性与 Rust 真实生产对象对应，但原 publisher 并未执行。

171: 中央 round34 全 lib 177/177，包含低墙52、高墙67、双窗双洞 roof+3d=1 的26条候选全记录比较（`/private/tmp/root-m6-half-multilevel-round34-20261029.log`）。八米真实 publication 输入 max_y=7.53999996，跨5.9分层门；同一双窗双洞原入口8次区域查窗、正常返回80木/0曲面，写站点13竖、4斜、16主横、6顶横、33细梁和8角柱/辅助。输入 SHA `14d25abc9992550710b9afd1c855b5ed44af3caac7643b553da2ae11870315fc`，原输出 SHA `3dda0032e00c87f054d81ba8392dc4c32fbc2eb4c10e677b9b8b4a046ae48439`，wood SHA `7e3148bc14d5b9baab98535c36fb3d14eea06c43de6df247ffadf5ba7bfa2c69`；80条逐64B断言已加、待下一集中批次。原 hashbrown 容器为手工原布局投影，Darwin libm 与模拟成功 push 限制不变；单个两层样本不能覆盖所有跨层随机状态或全部业务链。
172: 八米多层同输入改 roof+3d=1 的原入口正常返回41木/0曲面，写站点13竖、4斜、16主横、8角柱/辅助，顶饰和细梁不写；完整原输出 SHA `1b503d4bbb6bb1a6fa4bc7e7886d68e5b305a19b01f40430128de8f6d7e9a82c`，wood SHA `f49205e2d05ffe5fe65ce57eaeb3930f17d13f514efe2dfb6a81c106e659f882`；41×64B对照已与80条对照加到同一真实 publication 测试，均待编译。当前源码 SHA `6a3401862c69605c60cf97ad8e4b6d3288b4598fb97b19b6f51f96d139f2b255`。
173: 中央 round38 `/private/tmp/root-m6-complete-half-round38-20261029.log` 全 wall-constructor lib 177/177，包含真实出版八米墙、双窗双洞的80条及 roof+3d=1 的41条木板逐64B原对照；源码 SHA `6a3401862c69605c60cf97ad8e4b6d3288b4598fb97b19b6f51f96d139f2b255`。此为有限两层、多段、固定 hash/layout 与宿主 libm 的证据，不能替代真实生产 App/历史/保存/卸载联合验收或其他分支。
174: 圆窗下曲率通过样本：原 B RNE 经 Rosetta 执行 `141fd1c30`，14点原圆墙曲线、墙距1..1.8、半径2，生成3条64B、generated=1、failed=0；原 probe `m06-underwindow-circle-pass-oracle.c` SHA `73f5b58ef5068ca2f071f8af720ce5224b126438ab7129639699fe2292dd1ba7`，输出 `m06-underwindow-circle-pass-oracle.txt` SHA `ba7bff519ac558a74943b6c99a06e2fbc6728bea911bf10088fd72c19f5b4141`。原指令桥 Darwin sin/cos/fmod 与模拟成功 push；Rust 对照已接既有测试，待集中构建，不宣称Windows CRT全输入相同。
175: 木边条原 segment `141244ad5–c7e` 三组双端角/单端角输入生成3条64B；`m06-floor-border-segment-oracle.py` SHA `1d3ef2f08bb57818e46a605850c25b08cf7001fa38c3f9b8357532210c04f438`，输出 SHA `64979037b9717409ce55241c2c411a211e223880290dc3aeee8ecd4ed426cd7a`。仅跳过3个无关drawable辅助、原端帽比较和new_straight完整执行。进一步从原孔迭代 `141244573` 执行至 `141244ed7`：矩形4×2、墙距[-1,1]跨缝，kind1/4各生成相同2条64B；原 circular_slice/端帽/constructor执行，截实例后跳过 ECS，并桥接分配释放与 Darwin fmod。脚本 SHA `1e9673359bede878b4f937596b6109c989d46bd163edf2bd6c7657eb51dfb0ce`，记录 SHA `383924624487de278cb1d46bb01a1449c929fc09e8a4eedec0146013625b130d`。首两次分别缺堆释放桥及孔迭代结束槽而故障，修正后正常结束；这不是原完整 ECS/publisher 执行。现有实现的切片和单段提取为原逻辑 helper，2组原64B测试待集中构建。
176: round50 `/private/tmp/root-m6-pillar-cache-owner-round50-20261029.log` 全库181过3失败；木边条两个新增原64B断言均通过，圆窗下3条逐位也通过，随后旧耗尽断言失败：新增圆窗调用占用的块在 writer.clear 时归还 free，故测试意外不再耗尽。已改耗尽段使用新空 allocation 后清 free，待复编译；其余两失败是中央管理的支撑owner旧夹具，不混为半木数值差异。原 kind1/4 跨缝输出两条相同，代码图 snapshot `053060f0c5d388b353567d8b` 的 `function:B:1244310` 全764指令4页核完，`undecoded=[]`、末页不截断；此静态证据不代替 ECS运行派发。
