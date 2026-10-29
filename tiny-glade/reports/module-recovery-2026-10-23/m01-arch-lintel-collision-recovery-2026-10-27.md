# M01 拱/过梁真实段到碰撞恢复（2026-10-27）

## 原始身份、路径与字段

原 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。保留 B 原目录：`core/resources/walls/arches.rs`、`core/systems/collision/world.rs`、`core/systems/wall/construction/structures.rs`、`systems/wall-constructor/src/construct_lintels/mod.rs`，未建立第二套构造框架。

原 `construct_arch_geo1408d7ed0` 完整物理范围1165B。原 `construct_lintels14076d9f0` 物理11473B、fresh Ghidra body8202B，完整ASM另存 `m01-lintel-writer-14076d9f0-full.asm`；不能把该body长度当作完整含EH的物理函数。

ArchSegment/LintelSegment 原析构符号同址，两个主调用循环都以104B步进；更直接的原writer证据是14076e3xx按三个Vec及+72/+80源索引、+88的LoopRange逐字段move，14076e800替换height Vec，14076f368把完整有序Vec插入目标HashMap。因此复用现有真实 `MaterializedWallSegment`，`LintelSegments` 恢复为 `HashMap<WallId,Vec<LintelSegment>>`；不把同址ICF名字单独当字段证明，也不保留Gap/默认伪段。

## 完整几何叶函数

`world::construct_arch_geo(segment, normals, triangles)` 的原参数顺序：RCX段、RDX逐三角法线、R8三角形，后两者stride均36，不是索引。原机器码8174..81e7把 `Triangle::normal` 三次复制推RDX，81eb..8259把Triangle推R8。

原按 `heights.zip(positions).tuple_windows()` 截短到较小长度，0/1项不输出且不报错，每邻接对追加两三角，已有数组前缀保留。顶点顺序为 `(before,after,raised_before)`、`(after,raised_after,raised_before)`。升高的XY为 `(height*0,height)+position.xy`，Z为 `position.z+height*0`，不能以代数相等交换NaN来源。原14123ada0的法线取 `(p1-p0)×(p2-p1)`，严格保留原SIMD乘法左右来源、Z先加的平方和、倒数乘法；退化面与异常输入仍保留原NaN，不改零法线。有限分支const泛型消除异常兼容判断，只有非有限结果局部重演SSE新生invalid `0xffc00000`，不全局改math或末端归一化NaN。

`m01-arch-collision-rosetta.c` 按PE sections映射原RNE，完整原entry→ret，原tuple_windows/Triangle/normal不桥接。315输入覆盖两数组长度0..8各种不等长、原有前缀、192随机段、退化、±0、±∞、quiet NaN、subnormal、最大有限值；2072个新增三角与全部法线、旧前缀严格逐位一致。原夹具在 `m01-arch-collision-rosetta.json`，日志 `m01-arch-collision-tests-second.log`。

## 真实上游保存与失败合同

既有 `construct_lintel_supports` 已执行原几何/支持梁及height替换，但只返回木板，完整三Vec段被丢；上层又重复materialize只为统计。本轮极窄抽出同owner `construct_lintel_supports_with_segments -> (planks,segments)`，旧API委派，避免重复几何。上层按原mask/段顺序保存真正改高后的有序Vec，保持positions、points_u、indices、range，成功才替换旧行。

原14076e198为普通墙(296==0)时跳到最终insert空Vec；原实现continue遗留旧段，已修成空Vec替换。原14076dcaf在读事件前要求WoodenPlankSsbo，即使空帧缺失也失败；原PublicWalls缺事件目标会unwrap失败，不能continue吞掉。已保留这些失败顺序；原mask获取仍只在非0特殊分支，公共prepared/height先验证。

原函数不按删除事件对整个Lintel map做retain/clear，因此没有发明新全图清理。禁用/重新发布会用空行替换清旧段，整资源Drop/Reload按Rust真实所有权释放；真正消失的墙之碰撞删除由原主consumer的live-wall/removal分支负责。本轮未宣称补齐原mask内pillar proposal缓存/UiSignifier所有原用途，该较后构造业务仍需独立恢复，不能因段已保存把整个M06构造模块标完成。

## 原差分、真实World与协作接线

1. `m01-lintel-retained-native.py` 复用已有原指令执行器，按精确旧baseline路径重定向读取已迁移但SHA相同原RNE（不改共享metadata），原14122ff90→14076e65c..86e高度替换/141204400→14076f1f4..2e7第三数组→原完整支持梁/RDP/Walker/SparseWriter。98输入、304完整段三Vec及索引/range、1052条64B木板、第三数组全部严格逐位匹配，当前实现不是以候选自身为oracle。旧178统计仅用于取其原始输入表，本轮实际重新执行原码并存结果，未用统计打分。
2. `native_retained_lintel_tests` 2/2：上述原差分；原helper产生真实段→World/LintelSegments→construct_arch_geo→new_unindexed(NoneUV)→真实Rapier两次编辑替换、禁用移除、旧Arc Weak失效。日志 `m01-lintel-retained-native-tests-final.log`。
3. `retained_lintel_tests` 4/4：真实栅格/Publication/prepare paths→实际分段→实际construct_lintels；1.2→2.4合法高度编辑替换几何、静止帧不替换Vec、禁用空替换保留其它行；构造失败保留上次完整段和高度权威；缺墙原失败；空帧缺WoodenPlankSsbo原失败。CPU稀疏缓冲阴影明确拒绝GPU上传，未伪称GPU验收。日志 `m01-lintel-retained-tests-final-v2.log`。
4. 与GPU owner交叉发现真实接线反向风险：原writer `ArchSegments` 仅296==0有段，`LintelSegments` 非0有段；其早期main匿名临时标签恰反。GPU后续回原typed参数与RBP栈证：param9 Lintel `[rbp+4b0]→[rbp+290]`，param8 Arch `[rbp+4a8]→[rbp+2a8]`，已改source65==1选Lintel、else选Arch。这里没有修改其主map/pose/global raycast/upsert源。

生产主12参系统、宿主注册/原完整阶段排序、真正GPU上传与主窗口仍由root/GPU owner联合验收；以上局部真实World不是完整游戏启动声明。主消费可直接调用 `lintels.0.get(&wall_id)` 与 `construct_arch_geo(segment,&mut normals,&mut triangles)`，原host在constructor前必须初始化真实LintelSegments资源。

## 复用经验

- typed producer的相反门能反证consumer临时变量误命名，但最终仍必须回原参数/栈追踪决定容器身份。
- 以原分配中已被真实caller修改的Vec读取持久输出，能区分“算法算过”与“实际保留供下游消费”；现成API若丢字段，应抽同一实现而不是第二次重算。
- 原旧baseline路径迁移应精确重定向读取并核SHA，保留原metadata/脚本，不为跑oracle修改共享原证。
- fixture高度需按原base=max(Pub284+.5,Pub288)选择有效编辑；1.2→2.0仍被默认arch-height2.8支配，不该靠错误断言逼改算法。本轮选择1.2→2.4验证真实变化，保留失败日志。

2026-10-29 柱缓存补链：B 的 136B 掩码 +30/+48/+60 分别拥有 Vec<28B proposal>/Vec<f32>/Vec<usize>。Graph snapshot `053060f0c5d388b353567d8b` 的 `function:B:7534d0` 入邻接未截断，唯一直接业务调用 `construct_arches@14075f680` 站点 `1407610f3`；首端为同体内联。进一步查 `construct_lintels@14076d9f0` 原C与原指令证实其也内联写入三Vec，不能由没有直接调用判无写者。proposal 字段依次为 direction XZ、position XYZ、height、u；同key时 score累计最小而保留高度更大proposal。拱段首端 height>0.3，末端另外 length>0.3 且闭缝门控；过梁段两端无条件，height 加 `0xbe666667`。现有 Mask 安全投影补入三数组，arches 两种形状与 lintels 在原 materialized 104B 段后回填；无新增业务目录。

有界原指令差分：拱采集 `140760c20..140760190` 和过梁采集 `14076eaa6..14076eaab` 各51组（固定种子、多段共享key、方向和0.3边界），所有28B记录、score位型、key及顺序与提取生产helper驱动逐字节一致；预分配 Vec，只桥接容器容量，原数学与归并代码均执行。原RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。`m06-pillar-cache-oracle-2026-10-29.py/.jsonl` SHA `065d851956b39c10e708f96c8b67e86531e1ff1a22cbbe2f09005ca920c11772`／`36608e975042e49bd904a27afd41079e5e1da1a729ffb94812698523841d6c98`；`m06-pillar-lintel-cache-oracle-2026-10-29.py/.jsonl` SHA `aa88c55ecc6a4813159a68da7a0c029673b70d1f6752bcace0219e09b52500fe`／`224489434ed179e4ef1bbc3f7b5b2531f264b04ba6f73d4613f971c778c348b6`，提取驱动和输出同前缀保存。未证明非有限值、原完整系统、真实调度或生产交互；统一 Cargo 与真实World链由中央串行验。

`place_pillars` 石柱原侧：`m06-place-stone-oracle-2026-10-29.py/.jsonl` 分别 SHA `3c2211f3219c23255716201b54354a98429210e5c7eb064107d08a9fcf5ed952`／`4ea696678372fafb18e04dd6ef560102256edda0378fbff98a555e5d7ba7ad12`，执行原 14120d670 和真实写入闭包 14120d340，保存28组完整96B及最终RNG/counter；仅桥 allocator、Darwin ceilf、writer。分组覆盖高度阈值附近、负高度、0/123/u64::MAX seed、三个方向。材料从 WallId 取值，seed为每柱输出计数，wall_distance = proposal.u × Public+28c ± 0.230000004；正式候选是否匹配全部样本由主owner验证，不能以本原侧输出当差分通过。自由墙端点测试量纲原 Public+98 是 prepared Curve3 点数，延伸距离原 bits `3e400000` = 0.1875。

木柱原 `14075456d..1407546ad` 用具名输入及现场 SIMD 常量直接生成64B记录；6组含旋转、负零和负高度，结果与探针分别为 `m06-place-wood-oracle-2026-10-29.jsonl/.py`，SHA `3aa745c3bf3b71a2a8bf0ee09557f1984043038328a4ac85d9ff899541c153f0`／`1c6b4d23652fe3eecc6fd21d9854254529af4040dd76ef19a8655d37d478eb0e`。中央 round53 `/private/tmp/root-m6-place-original-records-round53-20261029.log` 为 wall-constructor 187/187通过，其中包含石柱28组及木柱6组逐字节测试；仅限有界原记录和本地lib，不能升级为父系统／生产App验收。拱闭缝两组追加原差分后总53/53（`m06-pillar-cache-seam-*`，原输出 SHA `64e44d4a13171740fb7ca23cfd2ad9e514e12fbaf1d6f8d7b80a794a1e20ab9b`）。

父 `place_pillars` 独立审查：原 `140753f51..1407547xx` 每项执行索引检查→位置采样→累积位置→发射砖/木，再处理下一项；此前候选先 collect 整个 mask 的 `SegmentPillar` 再发射，后项 panic 会错误丢掉已应完成前缀，已交主owner修正。空候选是否提前对当前重构 `prepared_path: Option` unwrap 亦与原 Vec 所有权/失败时序有关，已提醒单独核查。自由墙首/末分支按 loop start/end 与 flags 的门控、原距离0.1875 已核静态路径；短段、非有限和整个父体原侧差分仍未完成。

自由墙端点独立纠错：`140754092/0c7` 两次调用 `140bece40`，该函数为 `mov eax,[rcx]; ret`，读取 start/end 原始 i32；不同于拱闭缝采集使用的 `141200280` 模索引。先前将端点门控写成 rem_euclid 是错。`m06-place-freehand-oracle-2026-10-29.py/.jsonl` 执行 `140753f66..1407542bd`，5组覆盖首尾、无flag、模数等于点数和中段，原输入与坐标逐位保存，脚本 SHA `b19a1684341a4183890eb9fc7604759be6c037f6a52784565b27af070cf07f98`、输出 SHA `53249c43be496c73ca749d74f9a01c41edbcc4f4df51f4ca1667b0dd26363392`。原 `{start:0,end:3,period:3}`、三点末端flag4结果x=7.8125，错误取模会返回8。父入口空mask由 `140753f75..fef` 跳过候选，只有候选体 `140754037..46` 访问 Public 曲线；重构 Option 在空候选前无条件 unwrap 会改变失败顺序。石柱候选先整Vec再推送也改变中途失败的已写前缀，主owner已改为逐砖回调。

自由墙追加短段探针 `m06-place-freehand-short-oracle-2026-10-29.py/.jsonl` 输出 SHA `dcbe7048d5b30e0fc6f0ed2ac54e69a985c25370c7b248c432da306dd9034dc8`：长度0.1时0.1875延伸钳到另一端，start=1及raw start=−2/period2不误判为首端。中央 round56 `/private/tmp/root-m6-place-streaming-round56-20261029.log` 189/189通过，含已保存的前5原自由墙样本、真实lintel→木柱及后项错误保留先前写入；后4短段样本尚待候选对照。本人另在 construct_arches 新增实际arch→石柱World测试，尚待中央运行。

中央 round57 `/private/tmp/root-m6-place-arch-short-round57-20261029.log` 191/191通过：实际发布→arch 生成掩码柱缓存→place 系统→真实石砖分配和统计非空测试通过，另9组自由墙原端点及lintel→木柱/异常前缀均通过。此World使用CPU设备，不覆盖正式App、GPU绘制、全父原入口、编辑历史与删除卸载。
