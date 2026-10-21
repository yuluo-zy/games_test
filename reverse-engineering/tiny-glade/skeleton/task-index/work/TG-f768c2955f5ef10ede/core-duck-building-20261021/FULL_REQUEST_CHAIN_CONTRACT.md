# 楼梯请求与缓存必要闭包：实证合同及当前缺口

此文记录正在实现的真实 producer/consumer，**不是整链验收**。完整两个30KB系统尚未落源码；不把已交纯叶/工具取证算成主链完成。canonical owner只改既有TG280源，两个系统只改已claim的既有stairs.rs/stairs_assemble.rs/cache源，不另建生产目录或positions镜像。

## 原身份与真实入口

B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。两个full function原范围false140276e80/30178B、true14027e470/30226B均在native-pack。C未做完整行为核对；相同源/库/工具成功不作C恢复结论。旧decomp-manifest绑定full RNE，named C仅可读导航；raw ASM/PDB governing。

full caller-native-pack锁真正7个SystemParam：DecoratorStorage、CachedDecoratorTransforms、PublicWalls、**VisualTerrainHeightsData**、StairsState、ResMut<PlayermadeStairsAssemblyRequests>、UiSignifierStream。false wrapper140275590/387B→1402756e3，true wrapper140275720/387B→140275873。原UiSignifierStream是系统依赖，不能无证省略改裸函数无参。

两个body Demo最前返回。X=true在原source426..428先收集旧requests中dirty==true的**两个u32排序endpoint键**；X=false不执行该块。随后都clear当前Vec但保留cap并drop旧元素；不把clear改mem::take造成每帧丢分配。true14027e7da..821完整索引/148标志/128与12c/typed1402c8640已实证。source-line差异集合保存request-specialization-source-line-diff.json；只有该集合不足单独证明整函数specialization等价。

## 唯一owner与数据结构

Requests真实0x1c8：

| 原offset | 权威字段 | 原约束 |
|---|---|---|
|0..18|Vec<PlayermadeStairsAssemblyParams>|元素stride150，clear/drop保cap|
|18..a8|DiGraphMap<DecoratorId,AlignKind,RandomState>|**有向图**；typed add_edge140243e20/真实drop1405e24b0|
|a8..138|DiGraphMap<DecoratorId,AdjustedStairEdge,RandomState>|**有向图**；typed add_edge140243fa0/drop1405e2db0|
|138..168|HashMap<StairGraphEdgeIdentity,SampledStairsPoints,RandomState>|interpolate typed insert141266df0|
|168..198|HashMap<StairGraphEdgeIdentity,StairArchWallSupportType,RandomState>|旧retained assemble full-rne14001061e/14001fadf加168后typed insert1402c8dc0；两个真实输出组写tag1/0，语义名不猜|
|198..1c8|HashMap<StairGraphEdgeIdentity,SmallVec<[StairSubSegment;1]>,RandomState>|propagate1407d1982→typed fold141293420→typed insert1412670e0|

原prev_requests140274480仅返回self+138，是三个旧表组成的子对象，**不是Vec slice**。原空图default140244140同地址ICF有UnGraphMap别名，不能据此把上述两个有向图误改无向或IndexMap。

AdjustedStairEdge真实16B：`position:Vec2,outgoing_dir:Vec2`。原true14028000b..23构造16B后043 typed add_edge(a,b)；048..62反向构造后082 typed add_edge(b,a)。不要误把closure1返回28B临时`{positionVec2,AlignKind20}`当成该value。

StairGraphEdgeIdentity是u64 `(a<<32)|b`（140959e70完整10B），用于三个旧表。**dirty set不是此u64**：它用两个u32（StairGraphEdge或tuple<DecoratorId,DecoratorId>同结构）hash和compare；compute1408fe5c0先unsigned min/max输出EAX/EDX。父/水独立补充见readonly-compute-water/DIRTY-KEY-ADDENDUM.md。

Params完整字段见只读独审compute合同/output-layout.json：Curve2@0..38，height/width/tangent Vec@38/50/68，@80..98为元素4B trivially-drop辅助Vec（**元素业务类型仍未证**），Option<Curve3>@98..d0，两个(anchor24+degreebyte)32B端点@d0..110，BitVec<usize,Lsb0>@110..128，segment两u32@128/12c，SmallVec<SubSegment,1>@130..148，dirty@148。不得保positions mirror、把BitVec当额外Vec、用false/empty吞未知producer。Inactive payload/tailpadding没有原有效值，不能宣称原填零。

## 真实 StairsPoint 与派生

原node-fold14048ce90/840B（helpers-native-pack）是game closure嵌在std fold，不能当库derive跳过。按StairsState graph.nodes原顺序：required try_lookup_storage→required bucket→required DecoratorInfo→required cached.transform→required extra.as_stair；墙节点再required PublicWalls+WallSpaceCoord.get_tangent，任一缺失保持原panic。不可默认transform、StairExtra或wall-height。

原48BStairPoint：addr16，+10墙normal option（墙0，非墙1），+14/+18 normal，+1c/+20/+24 cache.translation Vec3，+28 DecoratorId，+2c StairExtra+4 width。墙normal按原tangent的side翻转并交换分量构成(ty,-tx)/(-ty,tx)，见14048d0ab..13b。非墙normal inactive；typed用Option<Vec2>表达，不制造0向量假朝向。

原derive14080c640/455B完整跳表已解：wall index0，roof index1，terrain index2，dst4 index3直接panic。度数取真实StairsState邻接长度as u8，无clamp。wall读取真实storage WallId和dst WallSpaceCoord切向量，按side翻转一个分量后用原rotation yaw→WallAnchor(WallId,angle)；roof/terrain各取required StairExtra首f32 height。原WallAnchor首qword已由as_wall75B→new7B→output+8锁WallId，旧Vec2解释错误。

## alignment graph 重建

1. 对各真实节点的邻居保原顺序建立候选。find_alignment_preferences14080beb0/1196B：非墙normal option入口即返回空；否则依次required两侧info、首dst必须wall、首addr必须wall。非同墙/对方非墙走3候选；同墙且delta²<1e-6只一个tag0；同墙其他走WallSpaceCoord.subtract权威周期差产生一个tag1。完整float/NaN/sign/0.414权重合同仍待逐项冻结，不能先造greedy轴对齐。
2. 无候选在directedgraph加AlignKind::None(tag2)真实边；有候选执行完整find_best_neighbor_alignments733B，递归全组合mask1/2/4、冲突-10、严格>、NaN与tie策略，不能只取最高分id。该叶合同已独审/source候选已交。
3. 原两directedgraph在source492/493暂take到scratch，按原clear/rebuild后，source1107/1108归还同owner。清理失败路径需Rust RAII保原drop；不另插替身图资源、无OnChanged假转发。

## 路径分解与全部场景分支（主体进行中）

原使用真实StairsState权威graph的clone作为待处理边工作图，VecDeque复用cap，Bump原依赖复用而不实现arena内部。不是自己构造另一个persistent authority。

- 每次取原图首待处理edge，remove_edge后先push两端。向front、再向back按**剩余图第一条邻居**继续消边；遇墙、无剩余邻居、头尾闭环等原门停止。不能按度数2人为截断或排序邻接。
- front与back墙、deque.len==2有独立直接墙间分支（source629..743），经实际storage信息/两个locators、closure1 offset、双向AdjustedEdge发布、derive两端；墙shape由as_wall→PublicWalls required取得，walk_shape8680B+closure1264B恢复边段，不得直接straight chord替代圆/矩形/闭合路线。
- 闭环/周期分支使用真实Spline2D::from_periodic_loop（14028089b/source824），保对齐position_offset、array和参数失败门；不能使用普通open spline闭合尾点代替。
- 普通开路径使用真实from_control_points_with_tangents（140280cc7/source877）、densely_sample_spline_segment140809ad0/source891、try_new Curve2/source905；逐段双向AdjustedEdge、derive端点后compute/source946。不能把曲线采样简化成节点线段或按index/N插值。
- **isolated nodes**还在source983..1078有独立手动构造：读取真实CachedTransform、actual graph neighbors、取caller端点位置/朝向，try_new Curve2、真实VisualTerrain两次采样、determine_segment_type、BitVec、两端derive。不能因为没有edge而跳过这些节点；亦不能让compute默认输入吞掉它。
- 每条轨迹追加完整Params到原Vec；真实geometry→aligned directed graph与请求必须同步生产，再归还两个图。true旧dirty传播只由实际edge集合决定，不能每次全部dirty/defaultfalse。

路径分解、wall/periodic/open/isolated上述四路每一条完整值/失败顺序仍须在独立复核前落精确合同/branchmatrix；现在没有源码候选，明确不以该概述冒算法完成。

## 已闭合compute合同与未闭共享边界

readonly-compute-water/COMPUTE-CONTRACT.md已由独立worker完整分析10808B及真实callbacks；默认tangent raw normalize、width按points_u、双侧VisualTerrain width/4 -.025/.1、先height采样再端点方向覆盖；分类连续kind3压缩、u16 wrapping、两端padding(bool互换/bit false前插和true后插)、每次真正重建Curve2、None/Some(empty)差别、dirty keys、RAII均保留。等待lead独审后由本唯一writer实现，worker不写候选。

原crashy_curve613B与当前utils robust f64补偿在极小metric/输入校验顺序有差异；只能由root协调TG9cab旧owner修canonical必要入口，不能本地第二curve或默认忽略失败。+80具体元素身份尚未知；types仅4B trivial drop，必须继续真实consumer读写追踪，不强命名成f32支持点。

## cache 生产消费顺序与未闭分支

原set25外19链：member2cache→4derive→5clear→6generators→7collider→8flush→14full/copy→15unleash→18UI，peer修host。cache先于request派生，读取的是同canonical请求/调整图的原帧生命周期，不要人为反序或构造单独last-request镜像。

cache完整9331B在TG6613 native-pack。三处仍缺真实实现：roof63、terrain63、dst4。其原dst4 no-match仍输出identity/(-0,-100,-0)/scaleZERO并按epsilon compare/dirty，不是None。mode2按SubSegment count/CurveU定位、台阶高度量化；mode3必2点→真ladder598+603、ceil(length/.65)至少3等原门；宽度/横偏和orientation/offset依原公式。roof63要actual directed adjusted graph与StairExtra.height，不能无向StairsState任意neighbor替代。

## 历史/删除与验收边界

清空原Vec/drop旧curves、旧3表在interpolate/propagate中的remove/insert及消失edge处理必须补；不能只append导致幽灵台阶/旧dirty永远残留。完整旧表生命周期由本stairs_assemble既有源收口；结构碰撞、mesh assemble两62KB本体不在当前已验收包，现还存在空体，绝不因cache恢复就声称游戏所有楼梯业务完成。

三已交source叶（size/math/typed canonical、best、ladder）仅必要基础，CPU验证由root统一。完整主链验收须同World真实producer→canonical request/graphs→cache→mesh消费者，并验证墙/地形/屋顶/环路/孤点/反向ID/编辑删除dirty及重启恢复。当前仍未运行这些验证。
