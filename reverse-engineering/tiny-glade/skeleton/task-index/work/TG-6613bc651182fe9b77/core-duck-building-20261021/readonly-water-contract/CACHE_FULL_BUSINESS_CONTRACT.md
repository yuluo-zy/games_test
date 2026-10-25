# cache_decorator_transforms 完整原业务合同（只读交付）

## 身份、范围与状态

TG-6613bc651182fe9b77 保留 core-duck-building-20261021 claim；duck 是唯一源码 writer。本 worker 只交本合同与有界原证，不修改 cache、utils、资源或生产，不运行 Cargo。之前 PAUSED_CHECKPOINT 是当时未完成断点；本合同现在将完整9331 B业务控制流整理到可源码化边界，不能据此说候选已实现、已测试或全游戏已闭环。

- 原 B RNE SHA=f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB SHA=2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。
- 主函数1408153d0/9331 B，PDB458:2156。原同 TG 的 `../native-pack/cache-decorator-transforms-full.{json,asm.txt}` 和 TG10dd 的 `window-host-contract-v1/native-pack/canonical-cache-full` 全9331 B均与本机原 PE逐 byte 相符，见 INPUT_BINDING_AND_CONSTANTS.json。主包自身 boundary_verified=false，由独立PDB完整size锁定，不将工具缺字段说成成功。
- nine-param candidate=f9ea3bd7ad9711ee362baef4，完整9类型保存在 cache-typed-interface.json。PDB25:7824 的完整1565 B run_unsafe1400c5100 中 **1400c552b直接call1408153d0**，将类型候选与真实函数绑定；全文字节对PE一致，见 TYPED_WRAPPER_AND_FORWARD_BASIS.json。这是类型/ABI取证，不把Bevy泛型包装器计作业务恢复。
- 合同只证明 B；A/C同文件 checksum 不等价，未验证其它版本全行为相等。原PE未执行，现Rust未做CPU/GPU验证。

## 唯一资源与当前可用API

原系统顺序严格是：

| 参数 | 原类型/所有权 | 现接口 |
|---|---|---|
|1|ResMut DecoratorStorage|同一 wall/terrain/stair 桶，不能新镜像|
|2|ResMut CachedDecoratorTransforms|同一权威 current map|
|3|Local CachedDecoratorTransformStorage|同一系统的 previous buffer|
|4|Res PublicWalls|required，无默认墙|
|5|Res TerrainHeightsData<TerrainHeightSsbo>|**physical** terrain；不是VisualTerrainHeightSsbo|
|6|Query<(&Roof,&RoofTriggerVolume)>|真实 ECS Query，不按ID造屋顶|
|7|Res PlayermadeStairsAssemblyRequests|同一 requests/adjusted_edges/previous owner|
|8|Res ClutterMeshes|真实 clutter元数据|
|9|Res StairsState|同一权威楼梯图|

九资源都是原参数要求；不把新增楼梯数据改Option来静默跳过。正文入口没有Demo业务门（前导是profile scope）；不从profiler全局状态猜Demo/ActiveSession门。

当前 Params 完整owner 2a046da8（及已整合生产）提供 curve、heights、widths、tangents、auxiliary_samples、interpolated_curve、nodes、terrain_contacts、segment、sub_segments、dirty；Requests提供 requests、adjusted_edges、previous.sub_segments。此次 cache **实际读取 curve、widths、tangents、interpolated_curve、nodes、segment、sub_segments**；不读 heights、auxiliary_samples、terrain_contacts、dirty，不能把别的Vec机械接成缓存采样源。

原动态指针 r15 在找到请求后等于 Params+0x58：r15+0x40/+0x48/+0x50 是 OptionCurve3/points.ptr/points.len，+0x60/+0x68/+0x70 是该Curve3.points_u.ptr/len/length；不是 Params+0x80 辅助f32 Vec。原metadata布局与现代安全struct的数值字段对应即可，不复制native指针/填充。

依赖现有 pinned glam=0.29.3、petgraph=0.8.1、workspace edition=2024。借用现容器、Curve、轮廓和碰撞元数据，不恢复依赖容器算法或新造注册表。

## 全帧共同顺序与生命周期

1. 原1408156af先 `cache.swap_buffers_and_clear(previous)`：两册交换，current清空且保分配，previous留前一帧。每帧重建current，删除项自然不再重插；不是两张平行权威cache，也不是clone整表。
2. 原1408156cc开始按现有 per_wall 桶顺序、每桶map迭代顺序处理 wall/roof；这些条目直接cache.insert，不做terrain/stair的dirty比较。
3. 原14081600b取terrain可变桶；若不存在直接下一域，存在则逐原迭代全部条目。每项required dst3，计算→previous对比→插入→必要modified标记。
4. 原140816792取stair可变桶，若不存在结束；存在逐项required dst4，完整处理。不能复用当前shared iter_stair_decorators后漏dirty。
5. terrain/stair共同尾部：先查previous exact id；有旧值且 `Transform::abs_diff_eq(new, f32::from_bits(0x3a83126f))` 为真时，**插回旧的完整数值**（不插近似new），不标modified；否则先 `cache.insert(new)`，成功后立即 `DecoratorMaybeMut::get_mut` 标该id，再取下一项。epsilon约.001，不能改容差或用距离单值替代10分量比较。
6. insert自身保现原有限性失败（translation→rotation→scale）；若insert失败，不标此id；此前项已插/已标的副作用不回滚。当前terrain `changed Vec` 最后batch extend会把中途panic前其它项的dirty丢掉，与原不同。

现 DecoratorMaybeMut 手工enum不是原逐条item模型，不要为了借名假用它。原29B get_mut实为 `modified_set.insert(id)` 后返回Info指针（已存旧原decomp）。同文件可通过 `let Decorators{decorators,modified_ids,..}=bucket;` 对两个现有字段分离借用，在 `cache.insert` 成功后立即 `modified_ids.insert(id)`，无新收集Vec、无whole-storage镜像、无foreign storage API改造。

若计算/插入中途panic，已swap/current部分结果/previous原表/之前dirty均应保真实发生顺序；没有整帧事务回滚。profile退出/标准容器展开由依赖承担。

## per_wall：wall/corner与roof全部分支

### raw wall tag 与墙空间

原140815875..894将 rawtag2/3/4之外的值归wall分支；rawtag2归roof；rawtag3/4出现在wall桶原unreachable。当前rawtag归类和实际 PublicWalls required读取保持。wall分支先required墙，按已有attachment/corner真实位置与法线，否则同 `wall_space_coord.get_tangent` / `.to_world_space`：side0 normal=(tz,-tx)，side1=(-tz,tx)。用现 `utils::xz_rotation_bad`，scaleONE；不改已恢复corner/rank/Curve失败行为。

### roof关联、UV与默认值

原1408158c2屋顶关联不存在或140815b3d UV取不到位置→fallback；关联存在但实体generation/Query实际没有Roof和RoofTriggerVolume则required失败（140817423/435分支），不能fallback。

fallback（140815c00..cb2）精确保前一缓存的 rotation/translation/scale；不存在旧值时 identity quat、translation=(-100,-100,-100)、scaleONE。该值是原分支，不是本次发明的默认几何。与后述dst4缺request的 (-0,-100,-0)/ZERO不同。

UV成功时既有 roof_uses_yaw 条件决定初始rotation为Info yaw+roof yaw的Quat或identity；保现 `cache_roof_orientation` 数值内核。若roof非flat，或type位于56..63且不等62，进入法线分支：

- type !=63：保原已有 rectangle rank1..3 required门→RoofSection rectangle→closest edge normal；非rectangle按真实中心/距离.01门选择+X或归一化径向；rotation=base*normal，不复制第二套算法。
- **type63**（140815cbc..ce9及e86..ee8）：在同 Requests.adjusted_edges 图中取当前 decorator id 的**第一个Outgoing邻居**，按原节点adjacency Vec顺序过滤Incoming；没有节点/没有Outgoing时保初始rotation。若取得邻居，则required edge_weight(current,neighbor)，取其outgoing_dir，`utils::xz_rotation_bad(dir)` **直接替换**rotation，不乘base。不按最小id/全边HashMap扫描选择、不用StairsState邻居替代。
- 随后原140815ef4再次看flat；仅flat且type63时required `info.extra().as_stair().height` 加入translation.y（140815f11/16）。非flat63不读取extraheight；不能为了统一先读extra。
- 完成后scaleONE，直接插current，无terrain/stair dirty compare。

`adjusted_edges.neighbors_directed(DecoratorId(id), petgraph::Direction::Outgoing).next()` 能对应原filter-first语义；随后仍required `edge_weight`，不吞图内部不一致。只借同一图，不能补反向边或重排邻接。

## terrain：原type63与已有clutter分支

原140816129 required dst3。先从实际destination读xz和yaw，**先采physical terrain基准Y**（140816180），构造世界点，再按yaw得到初始quat。type56..61不做clutter支撑；type62或56..63之外走现已恢复clutter bounds、orientation/translation、四边中心采样、原min/mean/smooth/最高点约束；此数值内核保留。

**type63**（1408161fc..24a，665..6ab）顺序：

1. required StairExtra，先在terrain基准Y加height；
2. 同 adjusted_edges/current id 取首Outgoing，无项保初始yaw；
3. 有项required edge_weight，取outgoing_dir，调用 **utils::xz_rotation_from_forward140c913e0** 替换rotation。它与roof的bad不同，不能统一成一个方向。
4. scaleONE；走前述previous exactcompare与每项insert→modified尾部。

## dst4：完整选择、采样、量化与最终变换

### 选择顺序

原1408168c3 required dst4；从同Info读取 pair(两个原u32)，lateral=Info+8，normalized u=+c，yaw=+10。

按 Requests.requests Vec 顺序线性找**第一条 `params.segment.nodes()` 两个ID逐项等于Info pair**的请求（140816909..92f）。不构造新的排序pair后匹配，不借无向packed history key，不额外检查其它id或addr。

**先匹配、后校验u**。完全无匹配时直接创建：QuatIDENTITY；translation三个bits=(80000000,c2c80000,80000000)，即(-0,-100,-0)；scaleZERO。原1408169d0..a06直接进共同compare/insert/dirty。不得None/earlycontinue，也不能为了“非有限输入安全”提前校验无请求项的u/lateral/yaw。

找到请求后140816939..953校验u∈[0,1]，越界与NaN均走range失败（UCOMISS unordered的JB已先失败）；随后独立NaN分支在正常控制流不可达。不可clamp或默认u0。可复用canonical CurveU/Curve的同失败域；若要保持原fmt还需保实际u诊断，不声称现共享panic文案逐字一致。

再用 **params.curve.get_coord_at_u(u)** 求 i/t（原140816963，RDX/RCX真实指向Params+0）。同VA有Curve2与Curve3等ICF别名，PDB主名称不能覆盖真实实参位置。这里不用 interpolated_curve 求i/t。

### 子段选择与通用预读取

原14081696d..a58：依原SmallVec顺序累计每段count(u16)；首个累计end>i命中。若全未命中，index钳至len-1，**start仍是原已累计总值**，不能“修正”为最后一段起点。空列表在required indexing处panic。实际count累加用原usize行为，无自创finite或覆盖性检查。

按原顺序required：sub_segments[index] → tangents[i] → tangents[i+1] → interpolated_curve.as_ref().expect → interp.points[i] → interp.points[i+1]。不先检查所有数组长度，不在可选Curve3缺失时从heights/Curve2合成3D点。

切线 `tangent=(next-current)*t+current`，normalize_or_zero后 normal=(-tangent.y,tangent.x)，包括失败归零时normal=(-0,+0)。这是平面**侧向法线**，后面mode3不覆盖它。

3D点按原weighted顺序 `point = (1-t)*p[i] + t*p[i+1]`（不是差分lerp）；先算点，之后分kind。原跳表142a8ff00直接读PE确认：kind2→816bc2，kind3→816cdf，kind4及其余→816ba8（只做通用点）。其它kind也要完成上述required读取。

令 selectedStart=start、selectedEnd=start+selected.count；两专用模式均取含两端的range，保原边界/空片/None失败，不补默认子段。

### kind2：按约.5世界步长量化高度

140816bc2..cda与f27..f91：

1. 从 **interp.points_u[start..=end]** required取last，保slice顺序/end边界及last unwrap；不是Params.auxiliary_samples。`span=(u[end]-u[start])*interp.length`。
2. 用SSE源操作数政策 `span = span > 1e-8 ? span : 1e-8`（NaN→1e-8，原bits322bcc77）；`fraction = minss(0.5/span,1)`（NaN→1）。
3. `count=max(round(1/fraction) as usize,1)`，原UCRT roundf ties away；原f32→usize为饱和语义，NaN/负→0，+Inf/溢出→usizeMAX。保持64位转换次序，不加2^24或有限门。
4. 从interp.points同一inclusive slice取首末Y（读取发生在count计算后），`low=min_numeric(y0,y1)`、`high=max_numeric(y0,y1)`。原一个NaN选数值，tie保左值，包括±0；不要依赖一个未经证实的自制NaN传播策略。
5. `step=(high-low)/(count as f32)`；`denom=maxss(step,1e-5)`（NaN→1e-5，原bits3727c5ac）；`point.y=ceil((point.y-low)/denom)*step+low`。步长本身不被denom覆盖。xz不在这一步改动。

### kind3：真实梯端调整与有序高度量化

140816cdf..d86：先required inclusive points[start..=end]，**assert len==2**，然后调用既有真实leaf：

```
calculate_adjusted_ladder_positions(
    &params.nodes, p0, p1, &stairs_state,
    &requests.previous.sub_segments,
    [params.segment.nodes().0, params.segment.nodes().1],
)
```

原ABI：RCX out；RDX Params+d0 nodes；R8/R9两个选中Vec3；stack20=原第9SystemParam StairsState；stack28=Requests+198 previous.sub_segments；stack30/38=Params+128/+12c两个ID。不是Info pair重读，也不自行swap nodes。manual failed-path geometry/nodes曾按路径原序而Params.segment由原sortleaf规范化；cache保持此原读取关系，不凭“现代归属一致”修正排序。

令调整结果为a/b（仍原序）：

- `direction = (if b.y>a.y { a.xz-b.xz } else { b.xz-a.xz }).normalize_or_zero()`，即原ordered比较下的下坡方向；NaN比较为false。原140816dca..e6c，length求和算序与Vec2现glam复用相符。
- `steps=max(ceil(length3d(a-b)/f32::from_bits(0x3f266666)) as usize,3)`，即.65；3D length平方求和原是 `(dy²+dx²)+dz²`；f32→usize仍饱和。原使用steps-1作分母。
- **有序signed** `step=(b.y-a.y)/((steps-1) as f32)`，不能abs或按高低重排；`point.y=ceil((point.y-a.y)/maxss(step,1e-5))*step+a.y`。
- 此后给整Vec3加 `direction.x0y()*0.2`（3e4ccccd），包括y分量加+0的浮点行为，原140816fdb..7006；该shift不重算或替换前面的侧向normal。

### 所有匹配kind共同尾部

最后才required widths[i]、widths[i+1]（原140817016..035）。宽度差分lerp `(next-current)*t+current` →乘.5 →乘Info.lateral，按原顺序；point加该值乘早先的normal.x0y()。

rotation= `DecoratorRotationXZ::from_xz_forward_vector(normal).offset(info.yaw).to_quat()`（140817092/09a/0b5）；复用现canonical旋转接口，不用atan2另一符号约定或新增私有矩阵。scaleONE。最后进共同previous compare/insert/modified。required widths、旋转/finite错误不可因near-equal旧cache而跳过：原先计算new，再比较。

## 唯一尚缺共享607 B leaf接口

本体terrain63已确证需要 `pub fn xz_rotation_from_forward(direction: glam::Vec2)->glam::Quat`；现utils仅bad，没有该API。精确源任务TG-1d8b7844066af51d6b（utils/src/lib.rs，旧cloth owner），需要中央协调最窄leaf候选；本worker不写foreign lib。已有旧完整缓存 utils-tabloid/140c913e0.asm.txt、utils-base/0x140c913e0.c，本次607 B全原字节又对同f507 PE/PDB759:11376绑定，见同目录xz-rotation-from-forward.asm.txt。

原具体算法可复用pinned glam构件：先 `direction.try_normalize().unwrap_or(Vec2::X)`；forward=(x,0,z)，up=Y，right=up.cross(forward).normalize()；构成**列(-right,up,-forward)**的Mat3，再由同版Quat::from_mat3生成Quat。原140c914cf/14d9分别对right/forward翻signmask，PE1429258f0四lane均80000000；上向量构造常量1429cd0c0=(0,0,1,0)属原SSE重排，不可直接把它当最终up=Z。

这与bad的(+right,up,+forward)相差方向；尤其不能实现为 `bad(-raw_direction)`：raw零/NaN/Inf时会先fallback+X而缺少原fallback后的翻向。零/非有限或倒长度非正/非有限均原normalize_or(+X)，没有Result/None/earlypanic；最终validbasis由依赖quat转换，不重写607 B依赖矩阵分支。输入有限但平方溢出同样fallback，保glam原try_normalize判断。

上述basis是完整静态合同；没有原程序运行oracle来声称所有NaN payload、±0及libm/编译目标全bitwise等价。可建议root做真实glam方向/零/NaN/Inf/大有限值与既有bad比较测试，不能以普通非零朝向样例代替失败域。

## 当前源码缺口与最小可执行范围

1. 现cache源码type63 terrain直接None；roof63直接None；dst4全部None，确实丢权威cache及dirty业务。恢复上述三分支，其余wall/roof/clutter已有内核复用，不新增生产文件。
2. per-entry dirty时序恢复，stair桶借用同get_mut，不再只插不标；保旧current/previous资源身份及原schedule cache→derive顺序。不能为满足当前输入强改原链顺序。
3. 加同一utils/lib.rs必要607 B leaf导出，由中央唯一writer协调，不串其它lib家族。其余实际callee（Curve2定位、Curve3数据、梯598/603B、DecoratorRotationXZ、Transform compare、物理terrain、真实Roof）已有当前typedAPI或现已审候选。
4. 原Info.ty63的上游真producer并未在此闭合：原raw transport事件的+68=3f不能证明最终Info生成。这是原业务链剩余未知，不应在cache加默认StairExtra或伪Info掩盖。
5. dirty/删除的cache册生命周期可在本源CPU验证，完整stairs assemble结构/visual/physics及GPU恢复不在此合同验收。不可因为合同完整就把长装配器、历史四Vec语义或renderer计完成。

建议duck最小World测试覆盖：exact有序首请求/no-match哨兵signed-zero且dirty；无请求NaN u不提前panic、matched越界才panic；empty subsegments与tangent/OptionCurve3/point/width失败先后；kind2真实interp.u和length、kind3真实同图/旧子段helper、kind4默认仍required几何；terrain63/flat roof63/nonflat63的extra懒读与outgoing首项；missing requirededge；epsilon相等复用old exactbits；某后项insert panic后前项dirty仍在；下一帧删除项不重插。测试需真实Params/Query/typed owner，不默认geometry、不用mock成功替代这些读取。
