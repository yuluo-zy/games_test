# G04 原 plain WallConstructor 完整源输入生产链

当前实现入口为 `reconstruction/building/wall_production.rs::produce_plain_wall`，适用原 construction kind=2、top/rim tag=2、fence-fit bool=false 的完整分支。输入仅允许真实 Curve2、三条 per-point arrays、FinalizedWallHoles 和原 caller scalar/options；不接受捕获的 proposal、context、RNG draw。父负责在真实 Bevy authority/publication 中绑定这些输入并完成本阶段 review。

## 完整恢复链与原码证明

原 B `construct_walls`140757800 在14075925f调用 `WallConstructor::from_curve`140769760。完整原 body13000余字节以及全部privatehelpers执行到return；oracle仅IAT确认的std/CRT/profiler桥。源输入初始化三个数组和callerABI，Rust独立完成 rows/trim、洞归一化、row collector、横向 partitions、全部seed混合、2条逐proposal RNG、row perturb RNG、sinf/truncf noise、顺序authority walker、支持/outline高度筛除、相邻row覆盖与上边噪声缓存、brick seed、per-piece洞/拆分及96BCPU实例。

当前 `B-wall-production-verification.json` 85/85案例、6476条完整有序96B数组，计数与每一字节均相同。矩形/圆圈、非均匀arch/second/third heights、0..4洞、open/closed、base、row-end perturbation、width padding/thickness scale options、wall_float开关、32/64bit大seed/source-id、arch全0无输出均覆盖。候选和oracle sourcehash及原PE SHA256绑定。合法ABI fixture不等于每种metadata组合都由当前World产生；sourceowner必须另有证据。

验证：reconstruction目录 `cargo build --release --bin wall-production-probe`；根目录 `/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_production.py`。

## 原模块与依赖过滤

三版私有源模块都是 `crates/systems/wall-constructor/src/wall_constructor.rs`，具体checksum/canonical身份见 `../building-constructor/three-version-provenance.json`（PDB source SHA256字段，不误称MD5）。本阶段恢复的是该原模块140769760完整plain分支，不是建新cargo依赖替代原游戏规则。

std Vec/VecDeque/sort复用、fastrand1.8复用、Curve2既有恢复模块复用。authority前进walker、noise/seed、相邻row覆盖、hole policy属于privategame code；不能把`crates/utils`视作第三方依赖筛除。A/C只完成source归属/版本候选对照，本报告行为验收仅B，不能推称三版本行为等价。

## Input schema 与真实caller来源

`PlainWallProductionInput`：curve、arch_heights、second_heights、third_heights、holes(origin tuple，kind在此consumer不用)、holes_length、closed、seed、brick_source_id、base、row_end_perturbation、width_padding Option、thickness_scale Option、wall_float。

authority worker已定位：first数组是PublicWall+D8 Vec，原ptr+E0/count+E8；second是prepared/public Curve3+90 points逐点Y，不能直接拿initialmapped+40的terrainCurve3代替；third是PublicWall+F0 Vec。原cache init14078b550在14078b80f依PublicWall+D0数分配zeroed(n*4)，在14078b846..854替换+F0/+F8/+100；因此zero third是原cache合法产物，不能任意用0替代未恢复source。BrickSourceId是R8D，并非material ID：from<WallId>140858790支持低32bit100000/100001/100002/100003/100011映射到0x80000000+偏移，其余截u32；冻结旧96Bschema的字段名`material`仅保留opaque位置，不更改其含义。

seed caller140758381经GlobalID方法1400885d0再14075838d乘0x4b28021，加PublicWall+48。base来自Public+274，wall_float来自Public+284。原完整source绑定由authority worker新模块证明，不能把本测试seed27等当World默认。

## 已纠正的真实差分失败

`first-authority-threshold-failure.json`保留first nonzero second/third案例：错误以row lower物理高度跳过列，导致122→82数组。回到14076ad22的[rbp+e0]、14076b41a/4f2/50e/517，证明比较对象是**row upper ×max_height**；改后整个12案例1470实例全一致，再扩85源输入案例。不是把输出容差放宽或桥接候选。

原研究fixture另有已明确IAT修正的truncf误认，当前oracle使用实际truncf，地址见`../building-constructor/B-CRT-IAT.json`。新生产不引用旧captured proposals，保留旧ancillary superseded记录。

## 第二阶段：完整 decoded construction 分支

`produce_wall(input, WallProductionBranch {kind, top, fence_fit})` 在相同源输入上恢复kind0..3、top.tag0..2、fence-fit false/true。原plain入口保持不变。top是24B辅助块的5f32+tag20：height、两对U interval、tag。tag2忽略前5个字段；tag0/1不是外部任意构造的屋顶schema，而是原caller建立的业务输入。

kind1最后row的横分宽度 .69、ceil后偶数count、跳过偶列但保留原draw/authority前进、上边噪声*.1+.1、最小宽 .184、末段压成width高度及下降center；kind3/tag2高度 `(arch_max-.79)+.15` 再以base+.1为下界；kind0与kind2仍保留真实clutter区别。top0/1的三高度band、interval子段split和特殊trim，top0 ground flags4，top1跨边界使用arch caps flags12；fence-fit=true按原arch+.46 trim，并按原比较在实际实例中选ground/arch caps。所有游戏规则保持私有模块归属，fastrand/glam/std不重新实现。

矩阵304 cases/21,846实例（最终rebuild后source/oraclehash重绑定全匹配）包含4×3×2全部组合、非均匀三条数组、真实圆圈/矩形/open2..5点路径、多洞、options、64bitseed；额外覆盖top boundary等于base或archmax的重复零高度band与interval端点0/.5/1。完整原body每次必须返回RIP==stop后才取数组，不能用instruction budget截断状态当输出。

`top-bounded-perturb-failure.json`保留128差分：原子段split逐段追加fullVec，故共享端点重复，但另有独立sum(subsegment_count)传perturb helper作为last_boundary。误以Vec.len-1替代会改变首砖尾参照和内部终端砖；修复后不删重复端点、不改seed/window index、不放宽浮点字节比较。`first-duplicate-band-failure.json`另保留重复高度row错误：无效高度row需消费rowdraw后不输出，不能提前重编号删除。

验证除原plain命令外：`python scripts/multi-version/verify_wall_production.py --branches`；证据 `B-wall-production-branches-verification.json`。A/C继续仅模块归属，不能声称三版行为等价。

## 实际construct_walls Roof/top owner

原码证据：`B-caller-branch-helpers.asm`，完整caller在 `../building-constructor/B-construct-walls-native.asm`。同VA可能多个foldedalias，按实参字段和publisher来源命名：

- 无Roof，1407581e1..201从Public+295（publisher对应source+66，即ClutterVersion）调用140833230(byte==0)，kind=2*is_initial。不能误称Roof rim。
- 有Roof，1407582ee..334，`Roof::ty`1408e3250比较Roof+30与.1（NaN按原CF=true）。若ty真，kind0；若ty假但Roof+3d的FlatRoofRimStyle::has_rim140833230假，kind0；否则kind=1+2*WallStyle::is_halftimbered(Public+299>=2)1408c5670。
- `Roof::is_gable`1408e33b0为(+30>.1或unordered)且+34==1。仅该分支在14075873c..7d5把默认top.tag2改为0/1，top.height来自PublicWall::max_y140b2a620(+108 validbit/+10c cached)，top.tag来自WallStyle bool。
- `Roof::gable_sides`1408e33e0输出Some((d,d|2))，d=Roof+3c。从PublicWall::get_wall_shape140b2a5d0取Public+258，ShapeParameters::as_rectangle1411ff740得到原rectangle，140c996d0 `Rectangle2d::get_point_u`依次取d,d+1,d|2,(d|2)+1构成两interval。既有 `building/wall_gable.rs::rectangle_point_u` 已恢复同helper，直接复用。
- caller1407585db..6ac对克隆arch数组做flat(+30<=.1且ordered)且无rim(+3d!=0)时每点max(arch + -0.64,.5)。这只修正本次constructor输入，不写回公共cache。
- 原from_curve调用14075925f传fence_fit=false；true分支有本体proof，但此caller的World不能任意打开。

## 零/负 effective scale 的未恢复边界

`negative-base-kind3-research.json` 在已冻结304验收之后另做5例只读边界审计：kind3/tag2, base=-.3/arch=.2 使effectiveheight<0，原返回0而Rust报Constructor(InvalidScale)；base=-.1/arch=.5使effectiveheight=0，原返回72而Rust仍报InvalidScale。其余3例匹配。现有trim/normalized-hole恢复API要求正scale，不能据此假设原body也拒绝零/负scale。正式分支矩阵base=0，不含这两失败。该未恢复域必须独立确认实际World能否产生，以及原输出是否含NaN/Inf，再恢复真实路径；不能以擅自clamp、默认base或忽略失败宣称完整ABI全域等价。

## 后续范围与集成退出标准

本阶段只恢复完整CPU WallConstructor实际输出，不等于完整建筑模块/renderer。父把真实Roof/Public/cache/sourcepolicy→produce_wall接World，并连续验原墙修改、flat/gable/rim/style切换、开口添加移动删除、WallRemoved/输出实体清理。捕获proposal/context API必须撤除；本体proof不能冒充rawsource producer、Nani upload、木墙/墙板/frames/支撑或其他consumer已完成。阶段review后冻结Rust，再绑定总报告。

## Oracle缓冲区修复与合法source边界

父发现原研究oracle三个数组静态间隔0x100仅支持64点；native_production已改为points/U/三数组/holes/两output reserve全部独立heap alloc，并assert输入count/cap/返回和输出边界。修复后85/6476与304/21846重跑全匹配；另65/129/257/513点、kind2/3、负base共16/1324全byteexact。shared native_fixture alloc_zeroed bridge随后由父改正，既有oraclehash报告需父freeze统一重绑定。

authority证明base是实际minimumY可负，不能禁负base；regularcircle/rectangle archcache floor原bit3f75c290，kind3必须flat+rim而不会走arch-.64。因此该真实regularOwner的effectiveheight至少约.32，即使base负仍positive。arch .5的零scale研究fixture不能来自该regularOwner，但其他source（freehand）仍需独立证明。
