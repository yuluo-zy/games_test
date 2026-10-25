# 双请求前段、直接墙路径与失败转入短请求合同（进行中）

唯一source writer duck。两全体原false30178/true30226B完整native-pack仍governing，不拿decomp的`SBORROW8`简写猜成功/失败。本合同完整已锁前段与新分支，普通/闭环Spline、manual每值和末尾dirty仍按原ASM补齐后再整体source，不先发半系统或假上游。

## 前段已闭

Demo原body入口全局门：现需要canonical DemoModeEnabled adapter，不constfalse或另造原全局。原7SystemParam中实际第四为VisualTerrain；现代是`TerrainHeightsData<VisualTerrainHeightSsbo>`。第7UiSignifierStream只传walk最后70槽，walk完整body未读取，不新增UI副作用，但真实依赖不能丢。host外19的member0/1 configs当前未绑定两个X caller，peer正在追；不能将member4通用derive当本系统注册。

X=true先以旧requests dirty字段收两个u32 StairGraphEdge key；false没有此carry。两者随后current Vec.clear保cap/drop旧值。用真实graph.nodes顺序collect_stair_points；memtake两个DiGraphMap，原trueE9F2..EC1E的PDB内联明确`GraphMap::clear`287/288（不只是clear_edges），保旧图allocation而清节点/边。按各实际邻居find_preferences→empty写None(tag2)/非空全组合best，并完整执行flags source530..584第二遍。

实际权威StairsState.graph clone作为remaining_edges；VecDeque每路径clear保cap。seed首all_edges端点（无向key规范化），remove后push a,b；从front再back各取剩余图第一neighbor继续移除，遇实际墙端/无neighbor/首尾闭环门停止。不能人为degree2截断、排序或每条seed新分配queue。两Spline原constructor共享Bump并各路径reset；现手工Spline2D拥有Vec，保用户重构不重造Bump版平行值，allocation/失败域差异须独核。

## 同墙直接walk的精确门（纠正只看“两个墙”概述）

要求path恰两节点。两端StairPoint.addr判别均Wall0，**WallId必须同值**：true14027fe52检查第二addr disc、fe64比较两个qword WallId、fe68不同→280322普通Spline。from addr门在此前同序，非墙亦普通路径。

随后按显式addr/id先required firstInfo、再required secondInfo。原feB7..fecf为`tag.wrapping_sub(5)>252`，即已知tag2/3/4转普通路径；不能粗写未知tag>=2都默默跳过，原未知>=5继续后续required/derive失败门。正常有效enum域这只允许0/1。读两Info实际WallSpaceCoord(u,y)。

closure1原完整140285a90/661B，两specialization都调用**同一地址**（false278951/278995/2795a4、true27ff77/27ffbb/280bc9）。RCXout28B；RDX本轮pointmap、R8alignmentgraph、R9a、stack20b：required pointmap[a]→required graph[a,b]复制AlignKind20→再次借同edge求position_offset(width)→返回 `(point.position.xz+offset, kind)`。Point map桶56=key4+value48+tailpad，不能混淆成value56。调用无X业务差异、没有查询b的Point或用normalized Segment排序几何端点。

双向分别closure(a,b)/closure(b,a)，先各自outgoing_dir，再写AdjustedEdge(a,b)/(b,a)真实position/dir，发布发生在CurveTry前；失败不能把已经写的图边回滚/删掉。然后derive两个实际anchors，required同Wall PublicWalls.get后调用完整walk。第二端传给walk的raw outgoing direction先两分量取负，不能与AdjustedEdge反向记录混用。

walk输出points由原Curve2::try_new_from_points(points, **true**)校验，成功才move四个Options与两端节点/ordered IDs/caller位置方向给完整compute并push原current Vec。

## CurveTry失败并非直接丢弃

直接walk失败原true280201之后将(a,b)追加临时`failed_edges Vec<(DecoratorId,DecoratorId)>`，释放walk各owned optional Vec/SmallVec后继续路径。普通Spline每段的CurveTry失败在2810附近也把对应段(a,b)放此同Vec（准确地址后续配对）。**不得写Err=>continue而丢失此业务**。

原source983..1078同手工短请求代码有两种入口：先按actual authority graph.nodes顺序处理原邻接为空的孤点 `(a,a)`；再按failed_edges插入序处理 `(a,b)`（true281899..18b9）。不能用消完边的working graph检查孤点，否则所有连通节点重复生成。

手工短请求会required pointmap[a]/pointmap[b]（后者只供metadata/anchor身份）；几何从a实际cache translation出发：

- a.wall_normal Some（原optiontag0）：direction=Vec3(normal.x,0,normal.y)，y额外量=-1，xz额外量=-0。
- None：required cached.transform(a).rotation乘原basis **(-0,-0,-1)**，y额外量=+1，xz额外量=+0。保现pinned Quaternion原raw旋转，不normalize它。

真实常量由RNE数据只读并sha绑定`request-fallback-constants.json`：14295a0e4=-1、142934698=-0、142925904=+1；basis1429a9080..a8。故即failededge(a,b)，几何不是简单连接a与b当前位置，不能用b.position替代原短出射段。

两点CurveTry(true)，失败跳过此短请求；成功后H0=a.position.y、H1=H0+direction.y+y_extra，width两项都a.width，tangent两项为direction.xz.normalize_or_zero（281d68..dd2），BitVec如下逐值绑定。determine_segment_type两返回(ALkind、DLflag1)均进入一个count1段；两次真实VisualTerrain采样写mask后derive a/b anchors，sorted IDs与旧dirty集合一并生成完整Params。这里不调用compute用默认参数覆盖手工业务；细节待全ASM收口。

## 末尾dirty传播精确边界

原true2824ed..2556先按生成后的current Vec收集所有 `dirty=true`请求的两端点到本轮HashSet<DecoratorId>。然后28257b非空才第二遍所有请求：任一endpoint命中此**固定快照集合**，在2826a7把dirty置true。不是传递闭包、不是边迭代边扩大集合，不因此改StairsState权威图/dirtyset。最后归还同两个本轮DiGraph owner（旧替身默认空图drop），scratch RAII与Bump释放。

全请求仍待普通/闭环Spline每段position/dir/IDs、手工mask与tangent原值、两specialization全分支配对确认后方出整体source。当前已交叶source不冒替两30KB系统；此doc不能作为完成统计。

## manual 最后寄存器绑定补充（已闭，可独立实施完整短请求 helper）

true281d4b..d63调用1402b6ad0，XMM1是原a.width、R8D=2，故widthVec是[a.width;2]。281d68..dd2按direction.x²+direction.z²求sqrt/rcp并finite&&>0门，否则两分量清0；281df9/dfd将XMM9/XMM12送1402b6c60，R9D=2，故tangentVec=[direction.xz.normalize_or_zero();2]。不是原raw3D dir，也不是端点b的方向或width。

281e2f的determine_segment_type读这真实Curve.points与两heights；281e34保存AL kind、e3a保存DL flag1。原SmallVec最终一项count1（packedkind|flag<<8|1<<16），不是只取kind丢flag。

281e4e repeat返回2-bit BitVec；e72/e75以原首XZ调用VisualTerrain.sample，f21前第二次以原末XZ调用同owner。两次都sample+.1与各自height做ordered严格`>`：281ecc UCOMISS H0后**JBE**跳清位，unordered同清；281f39/3d末点同。旧decomp `if <= else`对NaN解释不可靠，本合同已按ASM修正。这里没有width/4左右采样、没有-.025，也不max高度到terrain。

sample/mask完成后才按原locator a再b required lookup Info，并依次derive anchors（281fa0与282014前后完整原包）。metadata/dirty sorted IDs同已有canonical，auxiliary_samples明确empty、interpolated_curve明确None是本producer实际初始化。pointmap[a]→pointmap[b]→必要cachedQuat（仅normalNone）→CurveTry true→heights/widths/tangents→determine两返回→mask两采样→lookup/derive a/b→dirty→Params的顺序必须保留。

现代完整helper可在同stairs.rs借真实pointmap/storage/cache/walls/stairs/VisualTerrain/oldDirty并取有序[a,b]，返回Option<Params>；只把原CurveTry Err对应为None，不默认填字段。它既供孤点(a,a)也供failed_edges(a,b)，上游顺序由整体producer实现。此helper可独立冻结而不宣称两30KB系统完成。

## 前段和路径门进一步精化

原 source512、14027f023..04a：当前节点 a 对邻居 b 的 preference Vec 为空，仅 `alignments.add_edge(a,b,None/tag2)`，不立即覆写反向边。反向节点轮次自有候选，可能是墙侧有对齐、terrain侧None，不能将双方强制一起None。非空候选才进入本节点完整组合 best，随后按 best 输出更新该节点的 directed outgoing。

front 延伸每轮先 lookup_decorator_info(front)，Some 且 is_wall 则记录 front_reached_wall 并停（f966..97a）；否则比较 front 与 back，闭合则记录 closed（f9be..c1→faa6），再取剩余图 first neighbor。back 延伸同先墙门再相等门。直接 walk 的外门不仅 len==2：原 fc6c..7d还要求front停于墙、并且back也停于墙；然后才进入两Wall addr同ID/两Info门。closed 分支使用实际遍历得到的旗，不能一概用最终front==back替代，尤其潜在self-loop/不一致输入的失败先后不能推改。

普通Spline失败回退旧推测现由 REQUEST_SPLINE_SEGMENTS_CONTRACT.md 全体原ASM独立勘误取代：仅整path没有任何成功段才append其front/second一次，不是每个失败段各append。
