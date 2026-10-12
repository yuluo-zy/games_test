# 35组恢复前独立合同审查

审查者：`contract_review_35`；目标 C，A/B 仅作明确范围的参照。结论：`needs_evidence`（完整ABI及消费者）；允许按照下表的观察投影开始写候选。本文仅合同审查，不是候选最终review，也不是行为验收。作者本批尚无冻结候选，故不签发 passed_scoped，不升级 verified。

## 实际读取范围

读取 REVIEW_SCOPE、selection 全35项、contracts-observations 全35组A/B/C原始指令与寄存器、manifest A/B/C、依赖排除指导与机器索引、项目记忆、任务规则及reviewer门槛；对照既有 `reconstruction/building/decorator_placement.rs`、`wall_spatial_index.rs` 与上一批 Triangle<T> 候选。未执行构建、测试、原码、数据库查询/修改或生产源码修改。通过逐组拼接指令bytes，105个观察体均在所属组内完全相同；这是保留证据一致性检查，不是执行证明。

本批35组均为 `function_boundary_verified=false`，范围来自PDB记录。没有指令越界，体内均看到ret，但不由此升级原始作者签名、完整边界或跨版本调用者兼容性。同址别名仍是不同语义映射（如DecoratorId/new、FinalizedHoleStorage/inner），不可据共享地址归并成相同业务类型。

审查输入指纹：
- `selection.json` SHA256 `e2a12fd2197c22dc588d6260ea96e3bd823027a3402fcde9a84c5b35890e3690`
- `contracts-observations.json` SHA256 `23948038d70af15afc6a0388fe0373b7d2d36ee6787eb903f5928b808a89e8e1`
- `manifest-A.json` SHA256 `5c592f6abbbb7d19029f6ccbf53b37a68f24d7b86a651cf3aaf30c184f3236e1`
- `manifest-B.json` SHA256 `42eb72f134360be76b802a945c7ce26b17649dfb9b7b87e0227a1bc9743d698d`
- `manifest-C.json` SHA256 `3f44027f6b6f2aa72c30fe8f7a1ce22e1bbe4559c971ea3646eea78f8314cfad`

## 必須执行的约束及缺口

| 严重性 | 原证据/位置 | 义务、消费者风险与最小补证 |
|---|---|---|
| major | WallSpatialHash::grid_coord_to_cell_index，C 0x140ac2d80 | `y * [self+0x48] + x`先32位回绕，再符号扩展64位。既有模块GRID_SIDE=64仅B固定配置；本合同动态width不得偷偷代入64。cell_size分母为有符号i32转f32，不是usize/u32。C实际构造者/合法范围尚待接入。 |
| major | DecoratorLibKey::new，C 0x140c579b0 | 观察返回eax=输入edx、edx=输入ecx，两槽均32位。现有subtype/rank u8结构不是此短体完整ABI证明。可保留“返回槽0=第二参数，槽1=第一参数”的投影；semantic subtype/rank对应顺序需真实调用者补证。 |
| blocker（若重写底层） | CompoundEditBuilder::new_edit_sequence，C 0x140b8f4d0 | 输出0/8/0只支持空Vec的布局观察，必须用std Vec::new复用容器。不得手写三个word、dangling pointer=8模拟Vec，或为未知edit元素类型宣称完整布局。 |
| major | FinalizedWallHoles::holes_no_duplicates，C 0x140c4e290 | 只读取+8 ptr/+0x10 count并构造end=ptr+20*count。未执行过滤、不证明按id/几何去重。可输出20B记录迭代范围/包装器构造；next及实际重复策略缺证。 |
| major | ShapeParameters::center，C 0x140a623d0 | 按32位tag寻址+4+8*tag的两个float；不证完整variant、tag合法范围或每版枚举名。须有合法调用输入投影，不接受静默越界兜底或猜造缺失分支。 |
| major | HistoryCmd::interrupt，C 0x140c21ed0 | 仅证eax=0x02000200，不证payload结构/中断所影响队列。可保留常量投影，不能把现有完整history命令直接等同。 |
| major | RoofAnimation::new，C 0x1409e1790 | xmm0/xmm1置零返回；不据此补完整动画类型、时长或phase语义。允许两个float零槽投影。 |

以上不是要求所有工作停下：独立的算术/读写观察足够形成有边界候选；这些缺口阻止完整型ABI或真实业务链验收。

## 几何与数值合同（14组）

| 函数 | 可恢复的观察与必须保留的边界 |
|---|---|
| xvy | 输入两个32位float与插入float，写[x,v,y]共12B；复用上游向量或数组投影，不重写glam。 |
| lerp | `a + (b-a)*t`，先sub再mul再add，不能换成权重和或mul_add；保留NaN/无穷/零及平台浮点差异待验。 |
| init_rng | seed从ecx零扩展，64位回绕`seed*0x759839da217f+0x13c7ca9293765`后xor `0x4d595df4d0f33173`；与上一批new_seed只复用证实算式，最终使用原fastrand API。 |
| Aabb2 width/height | max.x-min.x、max.y-min.y；不abs、不强制正值。 |
| Aabb2 contains | 四个闭区间比较：x>=min.x且max.x>=x且y>=min.y且max.y>=y；每个NaN都拒绝，不能用“!小于”实现（NaN会漏过）。 |
| Rectangle2d area | +0x10 * +0x14，保留负值。 |
| Rectangle2d perimeter | 先两extent相加，再与自身相加；不要重排成两项各乘2。 |
| Circle2d contains_point | strict `radius*radius > dy*dy + dx*dx`（原addss把高分量平方作为目的）；负radius仍平方，边界false、NaN false，不能增加非负半径guard或sqrt。 |
| Rectangle3d extent | 输出[+0x10,+0x1c - +0x18,+0x14]；中心、朝向字段依据实际读写定位，不以直觉把axis解释为size。 |
| Rectangle3d translate | +8/+c分别加位移x/z，+18/+1c分别加位移y；+0/+4朝向与+10/+14平面extent保持。允许显式32B字段投影，不宣称完整原类型。 |
| Rectangle3d set_orientation_xz | 直接写+0/+4两个float，没有归一化/重建/通知。 |
| Triangle new | 输入三份12B，以0/12/24存放，完整写36B。支持[[f32;3];3]或已证glam Vec3，不能用16B Vec3A（数组间距错误）；不能凭此补Triangle其他算法。 |
| LoopRange len | end32-start32回绕，再符号扩展64位；不能饱和、abs或unsigned扩大后相减。 |

Rectangle3d字段布局由三函数互证：0/4朝向xz，8/c水平位置，10/14水平extent，18/1c上下y。语义名为推断，偏移、读写顺序为观察事实。Triangle构造将上一批points_mut泛型投影缩窄到有证36B实例时须保留上一批泛型函数本身未证具体型的说明。

## 建筑与历史合同（其余21组）

| 函数 | 可开始候选的范围与未知 |
|---|---|
| WallCornerAttachment to_rectangle_corner_id | 返回present=(cl!=0)，另槽为零扩展cl后64位减1；None的槽值回绕MAX不可转成有效角ID。不根据字节范围猜所有角enum。 |
| TransientDecoratorIdGen new | +0为输入rdx u64，+8清u64零，+10为r8b；最少17B有意义字段、padding未知；seed/计数/类别命名仍推断。 |
| FinalizedWallHoles wall_length | 读+self+18 f32，不能把整墙洞结构补造。 |
| grid_coord_to_cell_index | 动态signed32行宽，32位回绕并符号扩展，无边界检查。 |
| RoofAnimation new | 两个float返回零，仅槽观察。 |
| DecoratorArchivist decorator_colliders/interactions | 分别借用+self+60/+90处字段；容器型、泛型参数未知，允许调用者提供的字段借用投影，不手写HashMap布局。 |
| HistoryCmd interrupt | 原32位打包值0x02000200投影，完整enum缺证。 |
| DecoratorId new | u64输入恒等返回，不能降u32。 |
| HistoryChainIndex new | ecx→eax，32位新型包装，不能依名字用usize/u64。 |
| ReplayBuffer len | +18的u64长度值，未知buffer内部不可补造。 |
| DecoratorRank new | ecx→eax，32位新型包装，不能沿用rank:u8的语义投影当完整型。 |
| WallCornerAttachment impl$1 default | eax=0，仅tag值；不能仅凭impl编号识别完整trait Self。 |
| FinalizedHoleStorage inner | rcx恒等指针/借用字段，未知完整T。 |
| FinalizedWallHoles holes_no_duplicates | ptr/end构造且stride20；去重由迭代next或producer承担，缺证保持显式。 |
| InProgressHoleStorage holes | +8 ptr、+10 count，返回切片两槽，元素型未知。 |
| DecoratorLibKey new | 两32位槽，交换传入ecx/edx，不猜字段语义。 |
| CompoundEditBuilder new_edit_sequence | 原空Vec构造，std容器复用；element需consumer补证。 |
| ShapeParameters center | 合法tag选择两个float投影，不猜枚举完整性。 |
| WallId get_inner | 从指针读u64，现有WallId复用需确认表示而非因名字匹配。 |
| cell_size | extent.x/(width_i32 as f32)、extent.y/(height_i32 as f32)，零分母/负分母原样允许，没有clamp。 |

## 后续门槛

允许作者在合法claim目录内逐组写上述限定投影，并在中文注释中分清观察/推断/未知，统一A/B/C映射。不得把包装投影称为完整恢复的producer/consumer链。每组合同还需填真实调用者证据、适用的编辑/历史/删除边界；本批纯访问器没有通知不代表父consumer没有义务。

冻结后须独立读最终Rust再审查，尤其检查floats没有guard/reassociation、32位类型与return槽、20B洞迭代边界及std Vec复用。审查时可在暂停验证范围内记录待运行反例：动态width≠64；x/y负与乘加回绕；NaN/边界圆、负radius；LoopRange i32 MIN/MAX；Rectangle3d垂直translation两端；DecoratorLibKey互异高位输入；重复洞记录；empty编辑序列容器drop。不得运行这些案例或以记录案例冒充通过。

## 补充消费者证据审查（同轮增补）

独立读取support-pack-C的holes_filter_21465088.asm.txt：C 141478841检查self+1c，非零分支141478853调用构造器140c4e290；返回ptr/end后1414788a3每次前进20B，1414788a7检查record+17==0，否则跳回下一条。零分支141478958也调用同地址，但没有这项过滤。A 14177f912、B 14123fcf9的相同20B前进/record+17比较亦独立读取确认。因此允许候选额外提供`record[17]==0`过滤迭代适配，并标为来自该消费者closed路径的规则。不得称为按ID/几何做HashSet去重，不得将未过滤holes别名强制同样过滤；byte+17的“primary origin”含义仍属命名推断。原短构造体没有过滤指令，原作者Filter类型/next是否另有独立函数仍缺证。

独立读取support-pack-C/decorator_key_caller.asm.txt的1408441fe–140844222：r13b来自info字节；CL=0x11后调用Rank同址32位恒等构造器，140844211 ECX=r13d、EDX=EAX，140844216调用DecoratorLibKey::new；AL和DL分别写入邻接两个字节。可观察该调用者取低字节顺序为[0x11,info_byte]。高位寄存器内容不由这段movb证明清零，完整key类型也不因低字节存储自动成为2*u8 ABI。候选保留32位new槽投影，另说明该实际调用者的低8位key适配；已有placement的[subtype,rank]仅其已证场景的业务适配，C lantern全字段语义不直接外推。

补充读取不改变needs_evidence与未验证状态，但关闭了“完全不知道过滤谓词”缺口，允许上述字节谓词的限定候选。

- 补充ASM `support-pack-A/decorator_key_caller.asm.txt` SHA256 `de3e6980b6308b70e9c66128ccb57684d286f29621ba8f63a82e1db7ccf96250`
- 补充ASM `support-pack-A/holes_filter_24639552.asm.txt` SHA256 `b07b52f4d0d923ed16c9ae812c4fb8b7feabfb296b28a0c280d9091a6baf76a3`
- 补充ASM `support-pack-B/decorator_key_caller.asm.txt` SHA256 `b7b19cbcf8341d7fdb5405feae59fdb66dee41c499b16ada2ff43244efce6830`
- 补充ASM `support-pack-B/holes_filter_19135600.asm.txt` SHA256 `5ad9a952ec1aefb9e3cf743b868a9808382300640113a32b242baa09052b5364`
- 补充ASM `support-pack-C/decorator_key_caller.asm.txt` SHA256 `17f0bae00e926230c77d84e6519751309a9ae1ab9418cf3e819a300f08bc606b`
- 补充ASM `support-pack-C/holes_filter_21465088.asm.txt` SHA256 `e467b5b1aee8ef8f1dad5591d467a5de1dec2be49658b4e6a0ee4bbfdf46ec66`
