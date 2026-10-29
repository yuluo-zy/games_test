# M01 自动clutter转换入口恢复

原路径为B骨架已有 `country-core/src/systems/ui_clutter/{ui_autoclutter,rmb_autoclutter}.rs`。实际阅读B目录及原文件后再修改，没有创建业务目录。B源SHA为 `c5d14ecd5adc36744a5c80eff41b26879259ff13d2e6be4a0a8b7f2fca35709e`；原RNE绑定 `f507a6ce…b7d03`。完整机器输入和边界见 `m01-ui-convert-evidence.json`。

## 原链与恢复内容

`ui_convert_autoclutter_to_decoclutter@14099ca60` 原来有13组SystemParam。现恢复mode、真实UiState/流/射线、CameraCursorRay，原事件tuple，Deleted/父映射/真实组件Query/ClutterMeshes，DecoratorStorage、缓存、Derived，PublicWalls/History/延迟历史/Roof/引导/MainCamera，真实WoodenGate查询及声音writer。仅流按现有安全API使用ResMut；没有unsafe别名。

每帧首先清父映射，从 `Query<(Entity,&AutoClutter,&AutoClutterConvertableToDecoClutter,&Transform),Without<ClutterMarkedForDeletion>>` 重建真实metadata、变换和progress。原source<3才加入DecoratorId父键，与是否点击无关。原人工ChildTable以及Record/EntityIndex/Direct/Hit四套替代owner均删除。

完整消费右键队列后只取最新目标，以真正的Entity世代和必需组件Query验证。合法目标直接调用原路径 `rmb_autoclutter::handle_rmb_autoclutter@140976ca0` 业务叶子，不再另注册假System或第二条事件转发链。创建载荷、源AutoClutter生成ID、删除集合和历史都取真实组件；OnBuilding的屋顶关联及Roof组件保持原required失败。

无有效最新右键时，模式仅允许普通编辑与Bulldoze，直接Rapier查询原mask0x8f3，要求source kind12。目标仍须通过真实Entity/组件过滤。按原Entity Display名字注册距离canvas，实际活动控件才运行来源描边。

原相对跳表142abbcdc四项已直接解为：

| 原目标 | 业务 |
|---|---|
|14099dcb4|source0..2以DecoratorId查原storage/cache/subtype/rank，调用原主DecoratorOutline::display|
|14099dd8a|source3匹配真实WoodenGate墙ID/seed，复制顶边并追加首点闭合|
|14099e067|source4从真实Arc点列取得owned点值，保持原顺序|
|14099e0c9|source5无来源描边；进入公共圆圈和link图标尾部|

四路共有圆圈使用原metadata平面尺寸求半径、原0.1采样步长和源位置高度，link尺寸按原MINSS选择次序。link实际工厂拥有Empty transient mesh、原vertex_color_shader、Ui2dWsExtent和Ui2dImage("link")；不输出原始数字记录。控件名使用Entity Display，工厂名使用Entity Debug，二者不可混用。

Started字节3触发录制与延迟AutoClutter历史。Bulldoze写原声音kind0x20及真实ClutterMeshTy selector，随后返回；普通移动写0x25及完整StartPlace，并通知引导。记录在真正finished阶段提交，不能把Started帧误判为已经commit。

## 事件与共享组件

原 `EventWriter<StartPlaceDecoClutterInteraction>@141327570` 明确复制96B业务值，EventInstance步长112B；旧“unit事件”判断错误。现安全类型保存relative_rotation、widget、完整RaycastWorldResult与ClutterMeshTy，不把当前Rust布局强行当原enum ABI。原reader属于decorator的ui_decorator_affordance_dispatch@1407af340，由circle owner恢复同一TypeId消费。

真实WoodenGate完整56B语义值统一在core::components：顶边点、WallId、两端方向、seed、版本。原constructor的WoodenGateState、draw回调View、clutter GateHost均转同一类型别名；生产只插入一个组件。draw借用完整组件，不逐门clone Vec。门几何及散布算法未在此次身份迁移中改写。开发用tg-building-runtime的ordinal扩展类型不作为生产替代。

PlayDecoratorSoundCmd补原+1的selector，默认不活动值0，Bulldoze传真实mesh type；没有偷塞入ratio/context。实际声音派发由中央按原表恢复。

## 数值差分和真实验收

`m01-ui-convert-heading-native.py/json` 执行真实RNE，不桥接算术：

- 515组原140976fc9..14097705b完整SSE heading计算，两项atan2输入逐word相同。Windows CRT atan2/sin/cos函数体不在此oracle执行范围，未声称其跨平台位型也已证明。
- 515组原14099c420完整Quat::mul_vec3叶子，实际旋转轴为Z；逐三word相同。
- 515组原14099e61d..14099e67d，源Quat与相机Quat非共轴、包含非单位值，逐四word相同。相对旋转必须是clutter * conjugate(camera)，次序相反在第三组即可失败。

真实World覆盖父映射跨帧重建/变换/删除，最新失效世代抑制旧合法事件，删除marker排除，创建与源ID历史，四个来源跳表/工厂，Bulldoze selector，真实Rapier命中→控件Started→完整事件→Finished提交历史→实体/碰撞回收，来源Arc释放。额外验证link工厂真的创建Ui2dImage/Extent及transient mesh并回收。

## 当前边界与后续

此次恢复的是M01入口及必要真实依赖。原StartPlace reader/放置后续由circle，InstaCreate真实消费者由height，音频/GPU/生产调度由中央联合验证。不能以本文件事件存在代替这些消费者。

system-clutter当前真实manager仍只接受Window selector0/1/2，非Window selector显式未恢复，而且没有生产AutoClutterConvertableToDecoClutter写者。真实Query入口因此不能自动证明整游戏已有可转换clutter；该上游缺口属于M11，必须继续逐族恢复，不再建立人工索引表掩盖。

## 下轮复用经验

1. 先核原FunctionSystem参数，再解释ECS查询和ABI；0xa0/0x60/0x30实体表不是用户业务镜像表，映射的正确安全替代是原真实Query。
2. Ghidra未恢复跳表不意味着分支不存在。读绑定RNE相对表并保留所有目标块，主函数fresh C可能丢掉尾部复杂代码。
3. typed writer必须检查copy长度与EventId偏移。相邻ICF折叠unit writer的名字不足以判断调用处事件身份。
4. Quat内存按xyzw读取；误导字段名w/x/y/z会让identity变成π。法线旋转轴必须看原专门叶子，不能凭“地面/建筑”语义猜Y。
5. identity与共轴输入会隐藏四元数乘法次序错误，至少加入非共轴和非单位输入；先保留首失败，再修算法并逐bit重跑，不放宽精度。
6. Started与Finished分帧测试，事件reader跨帧保留同一Schedule状态；每帧重建Schedule会使错误重复消费被测试掩盖。
7. 业务生成→canonical组件→Query→extract/draw必须核同TypeId；字段相似的地方View不能证明生产可达。

## 最终冻结收据

`m01-ui-convert-final-validation.json` 保存四条实际命令、日志及前后源/manifest SHA，全部一致。native-physics完整ui_clutter回归26/26；canonical门→clutter查询1/1；原门生成载荷2/2；原门绘制顺序1/1，合计30项通过。没有发送OS输入。逐bit三个native组各515例；CRT精度证明范围仍按前述边界保留。

历史注册泛型合同随后已补正：ui_convert::<S>与原RMB叶子::<S>使用唯一History<S>，注册器及history before约束同步选择同一个S，实际生产使用S=()。新增带u64字段的非unit snapshot类型，移除World中的History<()>后，真实RMB→创建→录制提交仍成功，运行前后都没有第二History<()>资源。`m01-ui-convert-generic-validation.json/log` 记录补正后的完整ui_clutter27/27通过及前后SHA一致；算法和native差分范围没有改变，这不是新的业务恢复量。

## Window来源的只读反证复核

原B路径 `systems/clutter/src/autoclutter/manage_autoclutter_entities.rs`（源SHA `2e12f261…e3d6b188`）对应原1422d04e0；原FunctionSystem仍是现有五组参数，PDB接口aed08be5933f0d0d1777a62b。此前“manager没有Convertable写者”不能进一步推导成“Window路径缺组件”。

原1422d1be9..1422d1bfb的真实指令，对256种marker全部执行后证明：仅 `marker >= 3 && (marker & 6) != 4` 进入插入分支。Window0/1/2明确跳过；4/5也跳过。不得为了让M01查询非空给Window额外插入可转换组件。

原1422d1d37调用1422cba20的typed `insert<AutoClutterConvertableToDecoClutter>`，复制96B实际业务值：

| 原偏移 | 真实值 |
|---|---|
|0x00..0x18|原source union；tag4真实Arc点列增加共享引用，tag3/0..2保留ID字段|
|0x18..0x30|直接复制原spawn destination完整24B|
|0x30..0x40|实际选中ClutterMesh的mesh_name借用字符串值|
|0x40..0x60|同一个实际ClutterMesh的Rectangle3d bounds|

这与已存在的core canonical组件语义相通，但现非Window生产数据不足：ResolvedClutterMesh只留hash/radius/group/selector，丢mesh_name和bounds；AutoClutterSpawn及process_autoclutter_candidates还把真实source/destination分别缩成整数/Prefab字符串及无语义标量。不能从hash、host_id或默认Terrain反造缺失值。

必要修改范围应由中央按M11单一目标producer收口：先恢复该producer到candidate/output的真实source所有权、destination和原选中mesh metadata，再让manager仅在原marker门内插入canonical组件。Window分支无需改动；不能直接删除现marker<=2前置失败后靠默认值让非Window通过。本次保持manager及上述生产文件只读，未扩散M11实现。

证据为 `m01-manager-convertible-readonly-review.json`（256原指令输入、当前manager SHA）、`m01-manager-convertible-{original.c,insert-original.c,insert-original.asm}`。后续验收必须同时证明Window仍无Convertable、原合法非Window真实producer能进入M01，以及来源Arc/实体/历史/卸载所有权正确。

## 2026-10-29 完整场景阻断定位

真实App失败日志 `/private/tmp/root-m6-production-world-20261029.log` 的 `kind=3,key=0x8e53fbdb` 可精确追至当前 `populate_autoclutter_regions::process_dirty_cell`：`hash_cell_key(31,27)=0xfff779ed8e53fbdb`，随后 `process_autoclutter_candidates::accept_candidate` 将默认 candidate kind 0 改写为3并截低32位。复现为独立 rustc 小程序 `/private/tmp/m6-clutter-key.rs`，遍历64×64格，仅此格匹配。这个 kind 不能证明 gate/preplaced 来源；当前程序直接为脏格构造记录并丢弃选网结果。

原B 1422d6ed0（保留反编译 `crates/country-core/recovery/agent-team/reports/re-analysis/hardpoints-ecstate/evidence/decomp/populate_autoclutter_regions_1422d6ed0.c` SHA256 `a851f6308fad3e34a47d9c4dd8c8f4e136885c0548ebc3e17b5bee74ae43b788`）先取全部订阅并逐脏格调用 mark_dirty，冲刷待移除候选，再遍历真实区域表、只处理候选索引为0xffff的区域；非按脏格创建区域。当前代码只读取第一个订阅、对脏格伪造区域/候选、没有提供原 source、destination、选中 mesh 或实际方向。生产修复须闭合原 region producer→选中真实 ClutterMeshes→candidate→manager→Nani/转换/动画/删除，不能补默认值或过滤伪记录来宣称完成。

Neo4j通过受审查本地连接实查，snapshot `053060f0c5d388b353567d8b`，B binary SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。沙箱第一次查询为 PermissionError，受审查连接成功；图关联只给静态调用，不能证明动态分支。分析时源码SHA256：populate `bd474bdecc8311c5bf935c449c7faead6cf430c22a706a5c889c983396380160`；process `8b99028c583a31e0823f732c3bf9da5b16c5e01c2c84fe5a2705a325f4f6afa2`；mod `5a14f92afa8ce427e17c69c53febb28947ed581cb4114d888a2c1db90006e186`；manager `4a135200c298f1da7d9804c853aef556a94ae673827239b6e91c72d06b63a9fa`。这次只定位问题，没有业务代码变更、原侧差分或场景验收。

### 后续原证与阶段性代码（未验收）

共享 Ghidra 只读查询 `141cea460` 的 filter_map：用 candidate+0x40 查 `DeletedAutoClutter`，命中则排除；PDB FunctionSystem 参数核对该资源。`141cf9b50` 比较候选+0x90 的 u32，较大优先；相同再按+0x88 的 u64 较大优先，稳定保持完全相等的顺序。`1422ce260` 把 candidate+0x40 原样拷入输出 key、+0x48..+0x53 拷入位置；前文日志溯源只描述修改前错误代码，不是原B的key构造。现 process 已按该证据更正，source owner仍由上游补齐。`140eecf00` 确认过期区域先从懒网格删除且候选有效时入移除队列；当前实现补该生命周期。主函数遍历全部订阅，当前已补全并集与 mark_dirty。

增加了 typed 已选 mesh/source/destination 承载供 candidate→manager 传递，manager 针对已证 selector 3、5、6 的转换门控、对应 Nani 类型和延时分支；它们现在还没有真实区域生产者，不能以手工载荷测试代替 producer。weighted selector 修正原累加后当前索引及 unordered 比较；机器指令复核见下。以上改动仍待中央 Cargo 测试、原侧差分及实际App；旧伪脏格 producer和其他族仍未恢复，此处不宣称解除阻断。

原门区域 producer `1422cc390` 调用 `add_region` 前写 tag3、WoodenGate 的 WallId 和 seed；已将这三项保留为 canonical `AutoClutterSource::Id`。预置区域的 tag5 表示无来源载荷。窗口区域生产者的旧近似模型尚不能完整提供其原 tag/ID，因此保留未解状态；区域中含 source 不意味着 populate 已能生成真实候选。

共享 ReVa 只读分页 `get_decompilation(/tiny-glade.exe,1422d6ed0,offset=4310,limit=175)` 绕过旧120KB结果截断（首次10秒超时，重试成功；总4504行）。原末段先按候选矩形 AABB 查询 `WallSpatialHash`，逐墙检查高度和 footprint，再调用 `RaycastWorld::shape_overlaps_with<Rectangle3d>`；只有未碰撞才检查候选数上限（>0xfffd panic）、写回区域索引并拷贝完整0x98候选。输出尾+0x88来自区域键、+0x90来自区域的+0x34。当前 `process_dirty_cell` 完全缺此链，不可用一个mesh选择修复代替。分页结果还须与原字节复核；这是静态分析而非行为差分。

本批冻结后只读复核发现 manager 的新建延时阈值须以原 `local_250`（扣除已存在 key 并经 sheep 距离门控后的待新建列表长度）判断，当前阶段实现错误使用全部 targets 长度；等待中央构建冻结解除后修正。绝不将当前批次的该分支视为原侧通过。

`1408b8e90` 确认 ClutterMeshes 原 +0x18 起是固定槽指针，flowerbed selector29 指向独立 vector；结合 populate case0 的指针索引和 `FIXED_LOADS`，其九项依序是 watering_can、box_w_stuff、plant_pot_v1、ladder、wheel、pitchfork、shovel、firewood、bucket。原权重在对应 mesh 的 Option 为空时归零，firewood 还受区域 selector 比例影响。`region+0x39` 是组选择、`+0x38` 是 mesh 路由，与前面的 source union tag 分离；旧伪生产者把这些概念混在一起。以上是静态交叉映射，不代表所有组、权重或落点已恢复。

原候选构造的保留 C L2333–2395 把真实 region source union 逐 tag 复制（tag4增加 Arc 引用），把 `local_108` 选中 mesh 写 +0x38、`hash(GlobalId, placement ordinal)` 写 +0x40；destination 条件及其余字段按真实碰撞结果构造。生成 marker 为 `3 + 3 * (u8(b38 - 2) < 3)`，故 b38 2..4 为6，其余为3。实际写候选前仍须通过完整障碍检查。

后续阶段修正了 manager 延迟所用数量（仅过滤后待新建数），门区域 b38/b39 写序以及真实原点、轴和尺寸，并按原 140ee432d..43df 由原点加半旋转对角线计算懒网格矩形。旧每脏格假 `process_dirty_cell` 已删除：真正待生成区域在未完成原几何/拒绝/写入前显式报错，不能用空场景通过作非空链验收。普通组0且 selector0/1 已读取 canonical ClutterMeshes 与 ActiveSession，按 native 九槽权重、mesh Option及 `hash(GlobalId,0)` 选中具体资产；写候选仍待完成。新增选单一可用mesh与空区域不制造候选的局部测试，尚需中央运行。中央先前成功编译的是本段修改之前的快照，不适用于新修改。

普通组0选网后，已继续按原 `1422d7f4a..8041` 将同一 RNG 后续两次采样用于 mesh 半径收缩、0.01 下限、逆变换、floor/wrap 和正变换；计算仅用于明确 pending 的真实区域诊断，未将点充作候选。现有设备读取的候选系统不再用过时脏格假数据，原射线、地形/水/障碍拒绝及候选真正写入仍待恢复。该段只有静态机器指令对照，没有原侧执行差分。门区域几何、两字段和 manager 待新建计数修正也均待本轮测试。

原指令复核修正反编译误读：1422d7f0d 的 UCOMISS/JBE 在无序时继续，候选权重列表耗尽时 1422d7f14 重置索引0 再检查长度；非空列表全0选首槽，空列表才在该处失败。选网函数与辅助函数已按此改写并加全0反例；尚无执行差分。

原 1422d8107..82d3 指令确认向下射线 origin=(候选x, region原点y, 候选z)，direction=(-0,-1,-0)，max=f32::MAX，mask=0x30fb；miss 后采真实地形高度，并通过 PathRastersResources 对 path/cobblestone1/2/3 在同一坐标各采一次再相加夹到[0,1]。此前旧F3“水面四角”的解释错误。上述真实输入已接到显式 pending 前的诊断；hit destination、路径门、障碍拒绝及完整candidate写入仍未完成。

原侧落点 oracle：工作区根运行 `tools/venv/bin/python tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-landing-native.py`，执行原 `tiny-glade.rne`（SHA256 f507a6ce…b7d03）1422d7f5f..8048，256组覆盖轴、原点、尺寸、半径和两次随机实参；仅桥接两处 CRT floorf，输入均有限。沙箱JIT首轮退出132，受审查重试成功生成 `m06-clutter-landing-native-cases.rs`（SHA256 3fab8c11b438dc49e50d884b9a3559476e9151f1987db4ed47c5909e930b8a1a）。生产函数已增加 Rust 逐bit对照测试，尚待中央运行；该有界差分不覆盖选网、随机数、射线、路径、碰撞、producer或完整App。

原侧落点与生产算术的初步位型对照：从 `populate_autoclutter_regions.rs` 直接提取 `weighted_region_landing_samples` 原文到 `/private/tmp/m6_landing_standalone.rs`（SHA256 `51766a049ab70fb0d06ca4c7a11075571d5054f90ced9124caaf0354e39e6df6`），在 tiny-glade 根用 `rustc --edition 2024 /private/tmp/m6_landing_standalone.rs -L dependency=target/debug/deps --extern glam=target/debug/deps/libglam-6a2ccb3cbedb9281.rlib -o /private/tmp/m6_landing_standalone`，执行该程序获得 256/256 逐位相同。当前源 SHA256 `82d6643b98040f55d848d8cf89c41b26e61621487a709eb7d911e91801eecad2`、原 oracle 脚本 SHA256 `e898e3a55a1f7b466c100879ce22d4ae3a02746d5c5b10ac64738d1a08b67e90`。此驱动使用精简参数结构并直接提取函数体，不是 Cargo 编译真实生产模块；实际集成测试仍待运行。原指令执行和对照范围仍仅为上述落点算术。

加权选槽单独对照：`tools/venv/bin/python tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-pick-native.py` 执行同一原RNE `1422d7ef0..7f32`，不桥接任何调用，256组含全0权重与阈值；当前函数原文提取到 `/private/tmp/m6_pick_standalone.rs`（SHA `9b0954d826b6563a9f1c59bda148728d507f9917e72bb186d8877f004c6a9057`），`rustc --edition 2024 /private/tmp/m6_pick_standalone.rs -o /private/tmp/m6_pick_standalone` 后执行，256/256与原选择下标一致。fixture SHA `c1807c24ecac46cfcc17335c554316f141c97246f96a895b18b31f4ec4d66c32`；当前 populate 源 SHA `67e09e881f5492f5d7dc4a97f247dcea0b97f25b66aa84a91d03761622f3508f`。原组装权重/RNG、完整真实资源处理、候选生成未被这段证明；正式crate测试另排。

后续地形坡差的原指令 `1422d85f5..88ea`：先将选中 mesh 的真实 Rectangle3d 平移并按方向设置轴；区域 y 高于落点 y 超过4则拒绝。destination 分支小于2时，从 rectangle footprint 中心沿两个归一轴分别偏移半尺寸，共4个端点采真实 terrain height；`max-min` 与 `min((top-bottom)*0.75, 0.6)` 比较，原 UCOMISS/JBE 表示大于等于或无序进入拒绝。之后仍有 water/garden/rock/bush、墙与 shape 判定，不能用坡差局部通过替代完整 producer。

原 `141ced720` SystemParam 验证错误串直接确认 DecoratorRejectionRes 的 `ssbo_library`、`path_rasters`、`water_mask`、`garden_mask`、`bush_grid`、`spatial_query_grid`、`public_walls` 和 `phys`；`1422db2cd` 实参证实候选矩形最终 shape overlap mask 为 `0x308c`。`phys` 在末段有实际 Bevy Query 遍历，字段名不应被假定为单一刚体资源。静态偏移和错误串为后续对齐提示，不构成完成的业务验证。

本轮继续恢复 miss、普通组 0/selector 0..1 的拒绝前半段：实际 mesh 矩形、地形坡差、water/garden/rock、可选 BushGrid、真实 BalconyDoor 和 WoodenGate，以及 WallSpatialHash 实段逐一查询。墙段 `+0x28` 是 WallId；`1422dae18..dae63` 读取 PublicWallState `+0x280`（正式 owner 为 authority.elevation）与 max_y_gable，和候选 `[bottom+0.1, top]` 做闭区间相交；`1422dae65..daf14` 由端点生成宽 0.56、长为段长的矩形并比较 footprint。之后形状碰撞和候选写入仍显式 pending，其余 hit/group/墙朝向也未恢复。实现未完成，不能以空场景或手工构造 candidate 判全链。

BushGrid 原 `141cf3ee0` 写入器与 `141cf3900` 读取器均按 x+64*y；坐标迭代使用 f32 FMA、floor/ceil、夹至63且半开。原机执行 `m06-clutter-bush-grid-native.py` 共 128 组，与独立提取的生产坐标函数逐项一致，fixture `m06-clutter-bush-grid-native-cases.rs`；只覆盖有限输入的局部迭代，不证明完整 producer。区域 lazy-grid 现在也复用该坐标映射。中央此前 autoclutter 37/37 是上述新增拒绝代码之前快照 `/private/tmp/root-m6-clutter-producer-round4-20261029.log`；新批 Rust 源只通过 rustfmt 语法解析，尚待中央 Cargo 验证。

进一步复核 `1422d84df..8507`：path>0 对 selector 0/1 仍随机方向，但 mode 1 在 region 的 b38 !=1 时还进入墙朝向拒绝分支；仅从前面的 u8 回绕判定不能省略这个后续门。当前送中央编译批次暂时删了该门，已标记为未完成，下一批恢复。clutter 独立构建现显式开启 country-core 原有 native-physics feature，确保后续 shape overlap 可以调用真实实现；feature 本身不代表碰撞或候选写回已完成。

后续普通组 miss 分支源码已恢复该路径门，接 `shape_overlaps_with(真实 rectangle, 0x308c)`，随后从真实区域克隆来源 union、以 Terrain 目的地和选中 ClutterMesh 创建候选；全宽 key 为 hash(region key, 0u32)，依据 `u8(selector-2)` 原回绕该族 marker=3，不使用强制接受 marker6；写区域候选索引及原字段后经 process 与 manager。新增真实 region→选择→拒绝→candidate→accept→Nani 的非空集成测试及来源缺失显式失败检查；尚待中央 Cargo，不能宣称成功或覆盖原生其他分支。populate SHA256 `40ba2966840c133f2c6c4f7bec3e5218c6a13bdd645a2b31bd10f37e694c036b`，Cargo.toml SHA256 `7ed198ffb76a4a48db1da587d91ab9ef5591b7962301d74d4545139e7989c78e`。
原 `1422d84aa` / `1422d84b6` 分别经导入槽 `0x143193118` / `0x1431931a8` 调用 `cosf` / `sinf`；forward 的先余弦后正弦顺序有指令和 PE 导入表证据。Windows CRT 与当前 Rust 平台函数的逐位一致性仍未验证。
墙朝向门继续按 `1422d8b4f..8b62` 调真实区域矩形的 WallSpatialHash 查询；该查询为空时原分支直接拒绝区域，现已恢复。非空时的最近段、墙高度和投影尚未闭合，仍明确 pending。新批 populate SHA256 `520e5d945b8c3ef4c6d1d751c0c6cd8cb2fcea1f79460e57d9a8247d830ccc05`，待中央验证。
性能修正：本批将逐次从表头 find 改为一次按 HashMap 当前遍历顺序收集未处理区域，逐项保持容量门，避免多区域二次扫描；最新 populate SHA256 `1df50f4c6341e114e7e483fde4163def5d72a1b0285314e4197969d77fec80d8`，仅 rustfmt 语法解析，Cargo 等中央轮转。
墙朝向筛选继续对应原反编译 3184..3226：逐段将候选投影到端点线段、比较平方距离，并通过 PublicWalls::get_unchecked 获取真实墙高，在 `[elevation,max_y_gable)` 中检查落点 y+0.99；没有合格段按原拒绝，找到合格段后的法向/投影/重采样依然明确 pending。最新 populate SHA256 `238cd59cf71d39560505188a2b771817789a1fe7ab7c570ca85449356c89834b`，尚未 Cargo 验证。
普通地面模式的合格墙段后续 `1422d952b..96ce`：端点以候选 y 构造3D段、投影点、取得 `(-dz,dx)/length`，按真实 mesh bounds+0x14 的半尺寸加 0.28 加 0.05 位移，再对新 XZ 重采地形；已接生产分支并修正 SSE MAXSS/MINSS 无序次序。此段目前仅机器指令静态对照，未执行原侧差分或中央 Cargo；并不覆盖碰撞 hit 和其他组。最新 populate SHA256 `c44f24941588bd1f65eabb68d0baae95477368aa7de6afdc4b0cb3fea714d4b9`。
墙边投影原侧差分：`tools/venv/bin/python tiny-glade/reports/module-recovery-2026-10-23/m06-clutter-wall-landing-native.py` 在 Unicorn 内运行 B 原 `1422d952b..96d8`，128 个有限实参，不桥接被测算术；PE SHA 为前述 f507a6ce…b7d03。oracle SHA `80c398b93914450fcaa274ef138f38a776f65dc059da6b71538c0b90b26eab89`，fixture SHA `c8891b71c55ea034f1211531f2aab5720c3d12ddb3805df68151577c88664bca`。将生产 `wall_landing` 函数原文提取为 `/private/tmp/m6-wall-production.rs`（SHA `0169dadc6ac5bf1b0a9bd547830fa02bd0de2148018740931338877438717085`），通过独立 rustc/glam 运行，128/128 落点与法向逐位一致；正式 crate 同款测试待中央 Cargo。当前生产 populate SHA `b4651e78ff3a07008cd2a64bed4e99ce0ece75f5c14e235d2177fcc105f45cfa`。不包含段查询、筛选、重采样、其他分支或完整 App。
射线命中跳表 `143043840` 经 PE RVA 逐槽解析：tag 11 到 `1422d89a7` 并接回地形流程，使用 hit.position.y；tag 0/1/3/6/7/8/9/10/13 到 `1422d7da2` 直接拒绝；tag2 建筑处理及其它非法/未证槽保留 pending。已接上述已证路径，最新 populate SHA256 `c1e92050eb888529b9c581d8f9ac8b78d7e46e35ad5163743364b89ed055f1b3`，尚待中央 Cargo 和真实场景。
原 `1422d83b5..83c9` 对 hit tag2 再测 handle 字节索引1 的 bit0，清零直接拒绝；置位时建筑落点/目标仍 pending。已接该门，最新 populate SHA `f8470007e7f680fbde52f1c2a1093aa7b737ad2443de08f51951dd4065e98ce6`，Cargo 和完整 App 未验证。
区域 b39=1、selector0/1 的普通地面路径接入原21槽真实 mesh：`m06-clutter-group1-native.py` 直接运行原 `1422dd1d7..1422d7d23`，覆盖128组随机资产缺失及selector0/1，原槽号/权重逐位与当前表一致；脚本 SHA `108080dba0605737b688e5acbaa58610568cd7fe1f5187729fb0170ef598c07a`，fixture SHA `698c199961b4b18b435e7b1e5e84866064d80644dc39bda03a530c59a9166721`。正式crate新增根据原表、真实 ClutterMeshes 和 RNG 选网的测试，待中央执行；这不证明 RNG/其他组/完整场景。最新 populate SHA `b881c677ab43f6bf94fd6460f91f3df15751257cc55afe031163e8d79636bbf5`。
原执行 `1422dc370` / `1422dca9f` 采集非节庆 b39=2/3 的25槽建表（各128组资产可用性与selector）；旧 `KIND_MESH_SLOTS` 对组3多处误值，生产选择没有复用该旧表。真实 mesh 类型映射及精确权重已接入，新增从原权重 fixture 推算期望选择与生产实际选择的 crate 测试。脚本 `m06-clutter-group23-native.py` SHA `096677a654326bb397010a3b95171ac813ddcc2674b4070661228bcb0d0189c9`，fixture SHA `3d8948f5105f3f8a80f0831507b5feab9090f72f0bed72528489ab55ad1535d6`；生产测试、真实 App 待中央运行。最新 populate SHA `a234aa6aa65116a3e32dc5111c0d502e582fa2f4d3f56b1c49006155c6170a46`。
非节庆 b39=4 的37槽 `1422db824..1422d7d2a` 已运行原机收集128组可用资产/selector，槽序与权重接 canonical mesh，旧近似表若干错误被替代；脚本 `m06-clutter-group4-native.py` SHA `78f1ca5d523a48629fc7b45067cb041005d442bc219673733682515a17a74e04`，fixture SHA `0e5a86225052aca497306dc8fa54901f77591fcaf9b8b0ffdca7f10ae039a7dd`，新增 crate 选网测试尚待集中 Cargo。最新 populate SHA `7ce83dd103b2bb470e1358fb20019e2708607b8a02b427ba518fd1af5a91af63`。
中央 round7 `cargo test -p system-clutter autoclutter --lib`：45/45 通过，日志 `/private/tmp/root-m6-clutter-real-producer-round7-20261029.log`；包括真实区域到 Nani、墙投影、组1/2/3/4 测试。随后额外恢复非节庆 b39>4 默认38槽：与组4相比在槽序第11项插入 slot47/FabricRolls；原 `1422dd7ca..1422d7d2a` 构表128组，fixture `m06-clutter-groupdefault-native-cases.rs` SHA `c622ef41b56c6b2fb956664933ad064b181ac9cc20b872605bcfeddc80af17a9`，脚本 SHA `8762559175cca7a1181e975a1af3aeabba80b910932e20d2a94498e1ad12b784`，新增测试未被 round7 覆盖。最新 populate SHA `8a162f4eb18f062998e68e15dc9b87d6c0c6c4db1385b98ba60db385a21efd13`。
原 selector 跳表 `14304382c` 和 `1422d7e22..8090`：selector 2/3/4 分别读取 ClutterMeshes 指针槽36/37/38（OldenRock、OldenTreeLog、OldenTreeStump），区域中心落点；原 `1422d830a` 对正路径直接拒绝，干地前向为区域轴逆时针旋转90°，候选 marker6。已接其真实地形拒绝及候选生产路径，新增三个区域的真实 ECS 集成测试；尚待中央 Cargo、机器原侧落点对照和真实 App。最新 populate SHA `207d7f304d19e659a2d0265b0fa5c9ce8a08672fdcd1c9b691b68dd8a528076a`。
节庆 selector0/1 原 `1422d78b2..1422d7d23` 的17槽已按实际类型/权重接入，不使用非节庆组；128组原机缺失资源样本脚本 SHA `df9b148464f09568bc6308ca3479cc3a15e1896052d8528a48a6a2f89dd80ee6`、fixture SHA `161a274158c61993951defbf11fb38429e2b4876baf8598978d965ddb15d8d4d`，新增 crate 选网测试待中央 Cargo。当前 populate SHA `045791c34e391f37ede3b24074d4c0f971d669f936c53eb97215166f9683068c`；建筑有效 hit 仍明确 pending。
有效平屋顶 hit（RaycastGeoSource tag2、flat 字节 bit0=1）已连 mode3：从实际 hit 位置及 WallId 构建 OnBuilding，原UV判别仅在最终位置逐分量等于 hit 时保留，否则清0；仅当 PublicWalls 查得圆/矩形墙时按原缩圆0.28/矩形 padding -0.56 检查内部；mode3 跳过地形水/岩/门/坡差专属过滤，仍经过原墙段和物理shape拒绝。新增真实 Rapier roof collider→region→candidate→manager 夹具，墙表缺席分支按原逻辑通过，存在墙的过滤及原侧整体差分仍需验证。最新 populate SHA `8bb309691625dec5de2f13828dbdf2b82bcc20f0a25c056b1566b4a9b6f14534`，尚未中央 Cargo。
建筑 mode3 的墙边高度按 `1422d97b7..97d9` 保留原 0×offset + 两端高度插值的运算次序，不直接沿用原高度；该低位差还会改变 UV 判别。原 Unicorn 128组输入包含 mode3 全部 xyz 与法向，脚本 SHA `77ed600cd9009d99b5e8a00a3b63001d5aab73dc56455118ee1aeedf3ba5d544`、fixture SHA `17adc92bb366409dacbc520027017b1574f04b3010b708035ccac24f4b650a2e`，生产函数原文提取独立驱动逐位128/128；正式测试/完整链仍待中央。最新 populate SHA `110a18aaa33c5c6f5cb90fc18505d26af564806f4836c3ed88fd72cd3628ed92`。
鸟巢消费链复核：原 `1422c9c70` 遍历实际 `BirdNestCandidateLocations`，脏格只更新 `Local<BirdNestCollisionCache>`；旧按脏格捏造 Arc/实体已删除。m01 协作恢复 canonical 64B 字段后，clutter 使用真实 Arc<[Vec3]>、GlobalId、destination WallId、位置和 forward；原输出 source tag4、marker3、Birdnest 资产、y+0.15。`1422ca8c0` 扫描实际 Roof 的候选高度截面，距离轮廓严格小于 mesh radius×0.7 时阻止，脏格则刷新缓存；`140960b20` 的 f32 FMA 后饱和截断及无符号64格检查在消费者本地按原实现。新增真实候选→manager Arc、目标墙和 dirty 刷新测试，尚待中央 Cargo；上游实际碰撞交线生产者仍由 m01 恢复，不能以手工夹具当真实全链。当前 birdnests SHA `11e2fcc1b7ba8044daa680eaec810abf400e0f61516cb893ea471c2d8a62362d`，core collision SHA `2d759ac41b72a7081ff6356f0fe7bf812b1aca60cb2ffa9075524252977c59b7`。
鸟巢夹具进一步覆盖相同真实 Arc 贯穿 manager、删除候选后标记退役、动画销毁并归还 Arc，以及有真实 Roof component 时订阅脏格触发碰撞缓存重算；现 birdnests SHA `d3ceb1644700d3398420fddd866f8226d577f2e7c896d030a4310fb1dd4d902a`，等待中央验证。
独立树/水装饰生产者风险只读取证：ReVa 原 PDB caller 显示 `1422d2fa0` 精确参数为 AutoClutterState、ClutterMeshes、WaterRaster、DecoratorStorage、CachedDecoratorTransforms、ActiveSession；`1422d3800` 为 AutoClutterState、UiSignifierStream、ClutterMeshes、WaterRaster、Query<(TreeComponent,PrefabInstanceData)>、ActiveSession。原尾段均写真实 ClutterMesh 指针、source tag5、marker5、完整 key/方向/scale；当前代码却读局部默认空 TerrainDecoratorPositions/TreePositions 并以 Prefab 字符串输出，不能证明实际原生产者。冻结期没有修改这两个业务文件，待真实 App/中央调度明确下一阻断；不能将空输出算完整。
鸟巢脏格 FMA/截断补充原侧差分：Unicorn 执行 `140960b20`，只桥接 CRT fmaf，256组覆盖有限边界、NaN、无穷；提取生产 `birdnest_dirty_cell` 原文的 rustc/glam 驱动逐项256/256。脚本 SHA `021c04d4c329060c60668759f8b732a14a87fb7d15d1072997f7ad079c410fc3`，fixture SHA `fbcb4d1fa37bbe1f1eedbd3a434698dfa838f11b00538ae1ff5b3a307b208e8b`，提取驱动 SHA `b74d76ec64450b2c39a128efc7d2c70e7f34f31d77cc67e5111e0c33f90b2b49`。单格算术不覆盖 cache 全链；源码冻结中未加 crate 测试。
中央 round9 `/private/tmp/root-m6-clutter-complete-producer-round9-20261029.log`：autoclutter 52/52 通过，含鸟巢真实候选 Arc/Wall、缓存重算及退役，selector2..4、建筑 roof hit、默认及节庆权重。`/private/tmp/root-m6-production-world-round9-20261029.log` 的实际 App/生产 world graph 1/1 通过，记录502墙砖上传、480×270非空画面；日志没有证明鸟巢碰撞交线上游或树/水装饰 producer 的真实非空链，仍需另证。
round9结束后将同一原侧256组脏格 fixture 接到 crate 单元测试；该新增测试尚待下一轮 Cargo，birdnests 新 SHA `1c1443578c59057c3532f3a966a6459013ba7aacd69a230b4bd59231ece0b1fa`，生产行为相同。

窗口实际宿主增量（2026-10-29）：原 `1422cefe0` 的系统参数为真实 `DecoratorStorage`、`PublicWalls`、`CachedDecoratorTransforms` 与 `DerivedDecoratorInfoResources`；此前 `Query<WindowHost>` 无原生产者，不能作为通过证据。墙面分支已改为每面真实墙的真实装饰，检查 `spawns_clutter` 与墙 shape，沿真实 outline 按墙空间 u 延伸，使用装饰 ID 和业务 seed 的原键，写回 tag 0 来源、墙 ID、装饰 ID、kind 0、版本和 epoch。原 `1422cf84a..1422cfc8d` 逐指令运行 128 组，左右区域轴、尺寸、角点逐位对照 128/128；`m06-window-regions-native.py` SHA 2797d86d1e67ebf54a857b52fccc9f731bd819c9b07c250f2e1ee713489f77dc，fixture SHA 8fdb4724e3771e501632870219782d7be619af2f7c096c040bdce6e690dfe563。样本不覆盖曲线延伸、真实资源集成与 populate/manager。屋顶窗 subtype 5/10 的几何尚未完成，明确 pending；不以空资源或墙面局部数字差分宣称完整。

屋顶窗后续增量：B `1422cf521..1422cf7c3` 原四元数到两个 region 轴/尺寸/角点取 128 组（含非单位四元数）逐位 128/128；只桥接原 hash 和 add_region，不桥接被测几何。oracle SHA 9f553b204ed434f0f6473b3467e3b5be3a55ef26da738fbfca1e4160099dfbc7，fixture SHA a61fd8b6820696c58043ac4141884d700757c81e8520cf6b3cde806d8a6394d9；已接真实 transform 和 subtype 表以及原 trap_door 键与 source。真实墙装饰存储到 region/source、populate、manager 非空及删除退休测试已加入，当前源码 SHA deb5e169a7760725fe885c1e26f499370e58d60c4ad7ee05e7fd166605285dbf，待中央 Cargo 验证；本测试用显式 CPU mesh fixture，不能代替正式 App 的真实资产与画面验收。

2026-10-29 交线生产者续查：B 原路径 `skeleton/B/crates/country-core/src/systems/collision/intersect_meshes.rs.pdb.json`；图 snapshot `053060f0c5d388b353567d8b` 搜索与入邻接 `/private/tmp/m6-merge-graph.json`、`/private/tmp/m6-merge-neighbors.json`（未截断），确认 `1417d1440` 在 `1417d167c` 调用 `1408f2850`；B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。共享 Ghidra 只读反编译 `/private/tmp/m6-merge-query-result.json` SHA `eb8b90962ee783d2c1aaa730461de2c80d87b05352f6fd87e36d16e2fd675b07`，候选据此改为活动端点表、同键忽略、闭环复制首点、四方向拼接与空槽保留；新增四方向及已消费端点回归。尚未完整核原机器码／执行原侧多样本，生产调度和真实鸟巢非空链也未验，不能称完成；需中央串行执行 `CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib systems::collision::intersect_meshes -- --test-threads=1` 并核实际测试数。源码 `intersect_meshes.rs` SHA `b0573fcf00e272ae632a63b6c497125873a53a143b9e59f79c595fc49f41a2a4`，无本轮 Cargo 验证。

2026-10-29 窗口与鸟巢续审：B 路径为 `crates/systems/clutter/src/autoclutter/add_autoclutter_around_windows.rs`、`add_birdnests.rs` 和 `populate_autoclutter_regions.rs`。窗口墙／屋顶候选现使用真实装饰资源，屋顶 subtype 5/10 的区域计算已有原指令夹具；此前“屋顶几何尚未完成”是旧状态，并不表示整函数已完成原侧差分。真实墙装饰测试已延伸到非空 Nani 描述、源移除后的全部实体退役和动画销毁；尚未编译或运行。它使用测试资产句柄和构造的权威墙数据，不能当成完整 App／GPU／历史验收。鸟巢直接引用 collision 的 `Arc<[Vec3]>`，手工候选测试覆盖引用保持、脏格刷新及移除，但尚未从实际交线生产者端到端验证。tree／water lantern 仍是独立未闭合依赖，不能以脏格合成候选或 mesh 代替。

本批源码 `add_autoclutter_around_windows.rs` SHA256 `a82872bb5dffa87a7cb42f3f4f7bd1ed0f2a6aa569c1d25b10ad9fdc98cafa4b`；`add_birdnests.rs` SHA256 `1c1443578c59057c3532f3a966a6459013ba7aacd69a230b4bd59231ece0b1fa`。中央串行待执行：在 `tiny-glade/` 下 `CARGO_INCREMENTAL=0 cargo test --offline -p system-clutter --lib -- --test-threads=1`；结果及之后源码变化须另记，旧 52 项不覆盖新源码。M06 仍未完成。

交线增量原侧执行：`tools/venv/bin/python /private/tmp/m6-merge-oracle.py`（本机 Unicorn JIT 需受审查权限）完整进入 B RNE `1408f2850` 并返回；仅桥接 profiler、端点整数键哈希映射、预分配 Vec/VecDeque 和 memset/free，未桥接业务拼接。四方向、闭环后再用端点、同键和分叉，另固定 seed `0x6a2850` 的96组多段，共102组，与提取当前生产函数的独立 rustc 驱动对照全部折线槽顺序、首尾ID、点及法线 f32 位型 102/102 一致；未覆盖 hashbrown、allocator 扩容、非有限值或生产调度。探针 SHA `e335aaf4b37d7634f37e9a66f74dd575dbfab034797cd1fd31222aa9c3947854`、原输出 SHA `2744060e5e41fe8e001c68c19de5ef9dfef54e0d23e4fc4d2d36c92d79bdf5bc`、Rust 驱动 SHA `b1e91206b34229f336dd0639a84000157e5ee918c68f78aba4bf2c277512d8dc`、候选输出 SHA `236e4f747c97e785079a3f8f473d4c331344320dbe303e44b99cf2fc01563a63`。中央 round21 失败是新增测试对反向折线法线次序断言错误，已按原输出修正，尚待重跑。

父入口和闭包复核：`/private/tmp/m6-closure-result.json` SHA `fce7cfeb8b1ea84b1a9f740728d1554623943827904928fa0e94d6506fd97e80`；闭包完整比较 slot/index/generation 后仅在 added>=candidate 时查集合，父体初始化空集合并未见插入，闭包只读。因此删除此前按 added 集合过滤反向候选的推断，保留自句柄排除。原父体步进两处按 `<=0` 显式选择0，已修正 Rust `max` 对 NaN 的不同选择；未完成该路径原侧数值差分。当前 `SubworldColliderHandle::Ord` 只比较 slot，而闭包内联比较完整三字段；GraphMap 实际比较合同仍须原 add_edge 指令核验，不凭闭包结论修改。当前 `intersect_meshes.rs` SHA `c2eb7c3f8f01e6047501e80ee7a8ac4bf048088f789d089ff9795a94cde1c377`；全部修改仍待中央串行 Cargo、正式调度及非空鸟巢全链验证。

句柄排序后续已核原指令：`GraphMap::add_edge@14093a980` 的 `14093a9ca..14093aa0c` 确实按 slot→index→generation 比较后规范化无向边；原来将 `1408cc5d0` 的单槽比较推及该调用是错误。现有 `world.rs` 的 `Ord/PartialOrd` 改为完整顺序，新增同槽反序边插入／查找／替换测试。该改动和前述断言修正均待中央串行测试；当前 intersect SHA `dba50a272a8a7d494053c4c87e9497369f033aca39509d35d7e33380fcba761e`、world SHA `89593c7a8d4ee9c50f14d4679b616419b7d556027730a2a4`。

续审定位 round22 窗口失败：发布墙使用高度 0 的场，而测试 `TerrainHeightsData::new()` 的零编码是 -2.5；窗口 y=2 对应 4.5 高差，按现有原 `>4` 门控拒绝。测试已改用与墙发布一致的零高度场，生产算法未变，待中央重跑。鸟巢测试新增两面已发布墙及两片相交浅斜屋顶的真实 `RoofTriggerVolume` → `update_roof_rapier_colliders` → flush → `find_mesh_mesh_intersections_system` → 鸟巢消费者/Nani → 触发体移除/退役链，未手造候选或发布；候选几何、编译和实际非空仍待串行测试。它覆盖屋顶碰撞分支，不代表墙碰撞生产者、GPU 上传、原侧完整算法或 M06 通过。当前 `add_birdnests.rs` SHA256 `ee804eb1b4faee15f80c92e53ab991279b1bb0f615643a8810d94a6dfb97ac67`，窗口源码 SHA256 `f01cc2ad4d109d35999a3832c8a2795e6270043db28960a84f91ee770e7034fd`。

排序与步进独立复核：`1408cc5d0..612` 同槽时继续比较 index 与 generation，先前“只比较槽”的反编译结论错误；图 inbound 未截断，两个 caller `14129c000`／`14129c210` 均是 `iter_shape_overlaps<SubworldColliderHandle>` 的具名泛型实例。这个完整比较与 `GraphMap::add_edge` 内联指令一致。先前以 C 伪代码推测两个 carry 分支保留 NaN 也不成立：`140a98e20`、`140a990f9` 为第二操作数 +0 的 `MAXSS`，已撤销该改动。

BirdNest 步进独立原执行：`tools/venv/bin/python /private/tmp/m6-bird-step-oracle.py`，入口 `140a98ada` 到 `140a99133`，合成真实线段 VecDeque 和源 WallId，仅桥接 `SparseInstanceBufferWriter::push` 并捕获 96B 砖；生产 Rust 函数体提取驱动独立运行。4组边界/跨段/转角加 seed `0x6b98ad` 64组有限多段，共68/68 全部砖记录逐字节相同；最佳姿态及 `score > 0.9` 候选存在位68/68相同，其中6组产生候选。范围不含父体墙查询、Arc/hash、环形 deque、非有限值与完整生产调度。探针 SHA `4e60a6729709840cbb9115bd12ff8691b36511ee064dabe1df36d285a588effc`，原输出 `18bf0ccfa6f3ed077e72540ab453545306803295d9ba6417e82f3b27d67e6d15`，提取驱动 `7d86ec563b116cdee0455293b57d78326413025f18bf09548fc89b7d21269296`，候选输出 `97554a478acad487c9f22d7a135d24393090cd81ab7c3bcb55107720a5cff339`。当前 intersect SHA `bac0a62d63a38c4e5fe4a209eb7b434e668c67b0e57dee5c1dedbb039157958e`，world SHA `89593c7a8d4ee9c50f14d4679b616419b7d556027730a2a4`；中央串行编译和真实非空链仍待验。

证据已持久保存于本目录：`m06-intersection-merge-oracle-2026-10-29.py/.jsonl`、`m06-intersection-merge-candidate-2026-10-29.rs/.txt`、`m06-bird-step-oracle-2026-10-29.py/.jsonl`、`m06-bird-step-candidate-2026-10-29.rs/.txt`。后者原输出包含每组同输入的连续和跨环两种布局，68组两布局原结果完全相同；探针 SHA `34be86aaaf67e1a6f0c1ecae08a666e9f67128426866f9f597b4f2a59902630f`，原输出 SHA `1052628df1fef1ed34e3949792aa916a1cc9bcc518ba393d33e79a9b4597cc8c`。这些探针以 RNE SHA 绑定；merge 桥接端点键、容器和 runtime；step 直接执行原数学、比较及砖写入构造，仅桥接 writer；模拟器不证明目标 CPU 非有限 payload。源码驱动是本轮快照，不能冒充之后源码测试。

中央 round25：窗口实际源→非空 Nani→销毁测试通过；clutter 全库 61 通过、鸟巢新增真实交线测试 1 失败（日志 `/private/tmp/root-m6-clutter-round25-20261029.log`）。鸟巢空候选的输入问题已定位：测试 Roof 的 `height_01` 字节默认 0，`is_flat()` 为真，生产者生成 tag2 的 flat flag 1，原 `should_intersect_collider_meshes` 必然拒绝；现在测试设置为 1.0，与浅斜几何一致，并在非空断言记录图边及各边候选数量，待中央重跑。没有更改生产资格算法；实际几何通过、Nani 及清理仍待结果。

真实上游验证：中央 `/private/tmp/root-m6-clutter-round28-20261029.log` 共62/62通过，其中 `roof_colliders_generate_birdnest_and_retire_it_on_removal` 用真实 Roof 碰撞生产图边、鸟巢并验证删除退役。round25 首次失败因夹具默认平顶使 tag2 flag1，被原 gate 正确拒绝；修正墙输入后通过。此为真实 World 组合验证，正式 App set36 调度及完整游戏仍不由此证明。

中央 round33 显式 `native-physics` 暴露 collision 测试模块缺少 8 处引用的导入；已修测试作用域，源码 SHA `aa28d5b14c053f5bef77403beb5022462b97b01f0d8e98f923a763ff841b283d`，待中央重跑。此前不启 feature 的 core 测试不证明 Rapier 分支；Reload publisher 所有权由独立 owner 依据 `1408d8710` 指令纠错，旧“重建并重订阅”结论撤销。
