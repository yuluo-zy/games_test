# 7 个反射差异与 132 个零 key 配置分析

日期：2026-10-13。输入为已发布 db1ecad 的最终 404 包／638 阶段验收结果。本轮只读分析，未改 shader、未重跑 DXC、未运行原游戏。逐资产与输入 SHA 见 [GAP_ANALYSIS.json](GAP_ANALYSIS.json)。

## 7 个包实际属于两类

| 范围 | 数量 | 已确认差异 | 恢复方向 |
|---|---:|---|---|
| nani_gothic_window_bricks 颜色／深度 | 2 | location4/5/6 的 TEXCOORD4/5/6 对应原 is_bevel/brick_id/bbx_x；位置和类型完全相同 | 只恢复 VS semantic 名称，保留原 location、类型、PS 输入与函数体 |
| nani_solid_albedo_map_emissive 颜色／深度 | 2 | TEXCOORD0/1/2/3 对应原 Vertex_Position/Vertex_Color/Vertex_Normal/Vertex_UV；位置和类型完全相同 | 同上，处理实际选择的 TGV VS 分支 |
| wall/wall_brick_lod0 颜色／深度 | 2 | TEXCOORD0/1/2 对应原 Vertex_Position/Vertex_Normal/is_bevel；位置和类型完全相同 | 同上 |
| final_post 深度 | 1 | 原 sampler_lnc(set0,binding0)、$Globals(0,13，96B)、input_tex(0,14) 在新包中消失 | 区分 lifting、DXC 优化及真实 consumer 使用，不能补造完整后处理函数 |

前六个包是名称差异，没有发现输入数量、location 或数据类型差异。DXC 从 HLSL semantic 生成 in.var.* 名字，反射生产者只剥离 in.var.，所以 TEXCOORD 占位名称直接进入 typed schema。这是三个源码族的名称恢复工作，不能计为六套着色算法缺失。修复应改源码并实编译验证，不能把原 metadata 复制进新包。当前恢复的运行时桥接消费 location/type，但原完整消费者及命名／排序要求尚未全核，不据此声明所有场景无影响。

final_post 需要纠正“新包空函数就等于完整后处理逻辑丢失”的解释。原深度 PS 虽保留一次 ImageGather、一次 GroupNonUniformAny 及若干条件分支，原指令直方图未见 Store、ImageWrite、Kill 等输出／写入；它本来就不是颜色变体的完整后处理实现。当前 TGV 深度分支的 lifting 文本已没有 WaveActiveAnyTrue，仍有未使用的 Gather/min/max 值，官方 DXC 进一步把这些计算全部消除，留下 Return；反射资源因此消失。这里存在 lifting 与优化两个环节，不能全部归因于 DXC 版本。

这更符合死计算及资源声明保留方式的差异，但仍未证明整个管线等价：原 CPU pass 是否按名字／binding 绑定资源、此深度变体是否实际被选中、原编译器是否刻意保留这些声明都未核实。下一步先查原 runtime 选择点和 binding consumer，再选择有证据的编译／源码适配；不追加虚假副作用，不注入旧反射，不把颜色 PS 强行套到深度变体。

## 132 代表配置证据未闭合

这 132 个是发行资产数，覆盖 126 个 source 文件，均为 compute 且 artifact key 为 0，不是 132 个未实现 shader 算法。六个 source 各有两个入口，例如 sample1/sample4、r16f/rgba16f；入口名已包含在文件路径中，入口不同本身不需要额外宏。

| 当前实编译证据 | 数量 |
|---|---:|
| 使用空 artifact defines 成功生成新包 | 132/132 |
| 完整 typed schema 相同 | 132/132 |
| 工作组 LocalSize 相同 | 132/132 |
| 完整公共接口相同 | 63/132 |
| 非 debug opcode 直方图相同 | 25/132 |

这里的空 artifact defines 与单独标记的 TGV 等源码还原配方是两层输入；不能说编译时完全没有任何宏，也不能把 TGV 当原游戏宏。上述相等指标支持“空列表是可运行的重建输入”，不唯一证明原列表为空，更不证明全部计算算法一致。

原 ShaderDefines key 的规则为对每个字符串对分别用 std::DefaultHasher 求 hash，然后 XOR；原列表保留重复项，不去重。因而空列表的 key 为 0，但同一对重复两次也会得到 h XOR h = 0，而重复的非空 #define 仍可能影响编译分支。不同列表也可能得到相同 key。这个数学反例不表示原作真的使用了重复宏，只说明不能用文件名的 0 倒推出原列表。不要逆向／重写 std hasher 或暴力猜宏。

确认原空列表需要原侧证据：核 ComputePipelineDesc／ShaderDefines 的构造和实际 vector 长度，沿游戏调用点检查后续追加／覆盖，再核编译预处理入口是否有额外宏；若找到原作者 manifest/HLSL/build log，也可交叉证明。当前恢复源码中 new() 使用 ShaderDefines::default() 只是候选 API，文件自身明确注明不能冒认为原默认值。

应按公共构造路径及真实调用策略批量覆盖这些资产；生成／转发 helper 用作类型与配置合同证据，不把每个 helper 单列为新的业务算法。只有 original producer／caller 被验证后才把对应条目从 unproven 改为空 tuple 已证实；不能因为重编译或反射通过就自动清零 132。

## 顺序与边界

先修三个族的 VS 命名并复验六个包，这是直接、有限的源码修复。随后查 final_post 深度变体的真实消费与优化策略。132 项从共用构造／追加宏路径推进，可以一条证明覆盖多资产；继续分开记录“空宏可重建”和“原空宏已证明”。本次没有关闭这些缺口，也没有提高 397/404 或全着色语义验收状态。
