# 完整双楼梯请求 v2：独立源码审查

结论：**通过本次有界源码审查，未发现新的源码偏移阻项**。这是实际 candidate/stairs.rs 与原函数分支/最新已勘误合同的逐项审查，另有三条独立 F0 补丁复现；不等于运行验收或整个楼梯链完成。作者 duck，复核者 water。未改作者候选、生产、claim/Cargo，没有 Cargo/rustc/GPU/原目标执行。

绑定 v2 `before-frozen-20261021-full-requests-v2/candidate/stairs.rs` after SHA-256 `767ace742418ea85268bf5b36b15635731b6e80d359ed8626714039db5773305`；完整 production patch `15d3e259a5d0919df735853272dcec907a66a2b428ded877082c0b2f6592e1f2`，production before `87435b0f6c78d5695d7f1d89a2f8768585d7a62a34ab26a60d4e175ece0427c6`。原 B false140276e80/30178B、true14027e470/30226B 完整包与现原身份 f507a6ce/2d9686b8 governing，C 未独立核全。

## 实际代码闭包复核

| 候选实际路径 | 原证与审查结论 |
|---|---|
| 8参数现代 adapter，前7个原资源 | 独读 caller-native-pack 两个 FunctionSystem typed wrappers：6个Res+1个ResMut为同一 DecoratorStorage/Cached/PublicWalls/**VisualTerrain**/StairsState/Requests/UiSignifierStream，只有 Requests 可变。第8个Res Demo是复用现唯一App owner的适配；body早返在clear前，保原资源内容。没有改成默认高度、删UI依赖或constfalse门。 |
| true继承/false重置与清本轮Vec | X唯一分支收旧dirty的真实两个u32 StairGraphEdge；之后 Vec.clear保cap/drop元素。不同于 packed u64 history Identity。points从权威graph节点required构造，未跳过异常节点。 |
| 两图暂take、清空、重建、归还 | alignments与adjusted_edges为同Requests owner两张有向GraphMap；clear清节点/边，所有None偏好只写当前方向；非空候选用现完整best与flags叶。最后赋回原两图；native尾 true2826f3..2788 / false27b0c6..b15b为实际90B move/drop/move，不是每帧新persistent镜像。正常复用旧buffer；panic时本轮scratch由Rust drop，旧resource里临时空图保留，未伪造成功归还。 |
| 路径消边与前后墙门 | 实读 true27f925..fc83、false2782fe..865c：seed原工作图firstedge、先移除，再front→back按剩余firstneighbor延伸；每轮Info Some且wall先停，随后才头尾相等closed，之后才邻接取值。源码保两个wall旗与独立closed旗；不按degree2截断、不排序邻接，不把最终首尾相等直接代closed。 |
| 同墙direct外门及真实顺序 | true27fc6c..7d / false278645..56 两墙旗+len2，随后unsigned IDs排序。true27fe32..68 / false27880a..40 两Wall addr且同WallId；两Info required后原wrapping_sub(5)<=252门保留，不吞未知tag。双closure→双outgoing→双AdjustedEdge发布→两derive→required wall→完整walk→CurveTry(true)→compute/push。CurveErr保已发布两边并进failed，而非回滚或直接丢弃。 |
| 开闭Spline控制点 | 每pair前端aligned point；open首tag0/1可选导数，末closure(last,prev)相应导数取负；closed原周期ctor后才补首控制点。没有去重、local finite门或私有line替身。原caller min3独立assert已单独核实，见下一节。 |
| 每段排序与值方向 | 原true280e08后unsigned比较触发完整Vec反转及端位置/端outgoing交换；候选同样三者同时反转，随后CurveTry。成功后required两Point与两个有向AlignKind，两个真实AdjustedEdge先发布；required两个Info（281328/1359）后依次derive（1398/13cb），再compute。候选传 first outgoing 与 **-second outgoing**，没有在交换时多neg或normalize。 |
| 路径失败转短请求 | true281627..1717与false279ffc..27a0d1实际any_success为固定path旗。单段Err仅continue，成功push后置true；仅整条path无成功才追加front/second一次。候选没有沿旧FULL/PREP“每Err补一次”的过期措辞实现。 |
| 孤点与failed顺序 | true28181a..18b9 / false27a1ed..a289先迭代**原权威图**中无neighbor的孤点(a,a)，然后failed按插入序；没有使用已消空remaining误把所有点当孤点。 |
| 真实manual输出 | `make_short_stair_request`保持point a→point b→normalNone才读cached Quat；真实(-0,-0,-1)原basis不擅自normalize，出射几何只由a，不接b.position；CurveTry Err才None。两个height/width/tangent、determine两返回、真实VisualTerrain两中心采样ordered严格比较、再a Info/derive→b Info/derive→dirty依次保留。segment排序仅作key，与原raw方向/anchor顺序不混。 |
| dirty快照一跳 | true2824ed..2556 / false27aee9..af29只从生成后dirty请求收两个端点；第二遍只查该固定集合，true28257b..26a7 / false27af45..b0c6不扩大集合。源码collect完成后才mut迭代，不产生传递闭包或全dirty。 |

此前已审核的compute/walk/point/flags/math叶在完整请求增量里没有重新造实现；本次对原本体必须的调用参数、归属、失败顺序实际核对。未扩取其它新callee或把std/容器内部算业务恢复。

## v1→v2真实差异与原min3格式

独读 `after-full-v1.patch`，增量只含三处fixture先真实移除旧addr/id再跨bucket place，以及临时path VecDeque从u32换为同义DecoratorId以保原Debug参数。所有graph接口仍显式解.0，不改权威图或遍历/排序算法；控制点、失败edge与compute的ID值相同。当前DecoratorId为pub u32 wrapper，derive Ord/Debug，unsigned排序不变。没有借此改其它production函数或删现业务断言。

min3 **确有原caller独立门**：true280875 len→087c cmp2→0880 jbe2829bc→282a23 panic_fmt，周期ctor在28089b；false279255/925c/9260→27b38f..b3f6，ctor在27927b。本次另外从同原PE直接读 fragments1429a9a08，验证ptr1429a99e0/len36和真实UTF8 `A loop must have at least 3 points; `；PDB procedure209:11220/140511ce0锁定VecDeque<DecoratorId>::fmt。记录 `MIN3_TYPED_FORMAT_CHECK_WATER.json`。候选assert先于ctor，保原literal和真实typed path格式；不能把它误删为现代多余门。这里复用标准Debug实现，不声称编译器panic runtime/整条fmt字节完全一致。

三个fixture真实 `apply_decorator_remove` 会在旧条目不存在时panic，且直接返回原Info；移除成功后再place，能与已修storage旧Option语义配套，不靠旧错误保持相同ID两桶。新增6个真实World测试保留：反ID几何与双向图、两specialization与固定一跳dirty、Demo保内容、环路+孤点、direct失败保图、重复控制点→真实短请求回退。测试源码有意义且API参数配现owner；**本次未运行，不声明通过**。

## 补丁可复现与运行边界

独立三次 `/usr/bin/patch -p1 --fuzz=0`：当前production before87435 + candidate.patch、旧manual d0309588 + after-manual.patch、v1 9962aa41 + after-full-v1.patch，均returncode0，输出逐字节等于v2 after767ace74。各临时绝对目录、patch全哈希、stdout/stderr在 `PATCH_F0_REPRO_WATER.json`；production检查仍为87435，未变。

必要依赖仍须root原子配套：完整canonical Params owner2a046da8、native Spline2D after811b088c（lead已独审，root运行结果另报）、bitvec=1.0.1既有Cargo hunk与pinned petgraph0.8.1/Bevy0.16。Curve2原f32失败策略、WallSpace原half-period与CurveUCircular失败时机属于已列共享依赖；不以本地校验/直线掩盖。作者freeze中旧pending字样不作当前运行结果。

宿主member0/1对应实际const/run_if尚未绑定；Info楼梯真正producer、cache/interpolate/62KB mesh和graph editor未因本包自动完成。此通过只许可root使用这份完整请求producer候选进入实际CPU/依赖/宿主验证，不能称游戏完整楼梯链或渲染主链已打通。
