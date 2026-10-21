# 完整请求的开/闭样条与逐段产出合同

原完整两 specialization false140276e80/30178B、true14027e470/30226B 与同 native-pack 中精确 PDB/sha 身份为依据。当前地址以下以 true 展示；false 配对待整体收口独核，不能用函数名/同源码 checksum 代替。

## 控制点

调用域与完整 ctor/采样叶见 spline-constructors-readonly/SPLINE_CALLER_DOMAIN_CONTRACT.md。周期 path 首尾同节点，按每个邻接 (a,b) 取 a 点 shift，控制点数=path.len-1，至少3点；先 original periodic ctor，再 append first control point 供 samples/endpoint zip。开路径同 pair 前端控制点并追加 closure(last, previous)；首导数对 tag0 取 first、tag1 取 second，tag2 None；末导数对同规则结果取负，tag2 None。没有去重或 finite 前置过滤。

## 每段数值与 ID 对齐

140280cd3..d58 对控制点 windows(2).enumerate 调完整 densely_sample_spline_segment(index)。实际 path[index]/path[index+1] 两 ID 均 required （d72..db3）；密采样输出 Vec points、start_outgoing=tangent(0)、end_outgoing=-tangent(span)。

140280e08 unsigned 比较两 ID。若原 a>b，反转整个 dense Vec，交换两控制点 position 与两 endpoint outgoing；如 a<=b 保序。之后所有 lookup、两个 directed graph key 和 compute locators 都以 sorted [min,max] 对应几何，不能只排序 key 而不反转数值。没有对 outgoing normalize，亦不能交换后额外整体取负。

140280f70..f84 Curve2::try_new_from_points(dense points,true)。Err 释放本段 Vec 并仅跳到下一个 segment（140280f89..faf），不触发本段 metadata/anchor/AdjustedEdge/compute。Ok 后保存 Curve，先 required min Point，再 max Point，required graph(min,max)、graph(max,min)，构造 locators；然后 add AdjustedEdge(min,max){position=sorted first control point,outgoing_dir=sorted first outgoing}，再反向 {position=sorted second control point,outgoing_dir=sorted second outgoing}，两 publication 发生在 metadata lookup/derive 之前。Info required min 再 max；derive min 再 max。

compute 实际输入：两真实锚点/Point/AlignKind、sorted control point position、**first outgoing 与 -second outgoing**、sorted locator IDs、成功真实 Curve，所有 override=None，dirty 来自本轮旧 dirty set。140281428..42d 是第二 outgoing 两分量 XOR sign，不能误将两个方向都 neg。完整 compute 已在 before-frozen-20261021-compute source 独审。

push 成功 Params 后 140281600..602 把 per-path any_success=true；该 flag 入口140280cf1明确0，每段 Err 不写。

## 重要勘误：整 path 失败回退

旧 REQUEST_PREP_DIRECT_FALLBACK_CONTRACT.md “普通Spline每段CurveTryErr都 append failed_edges” 是尚未配完 ASM 的错误推测，现以本合同取代。

完整 true140281627 `testb $1,0x948(%rbp)`，非0即直接清理结束本 path；只有 any_success=false 且 path 至少两 ID 时，1402816b1/6c4 取 **path.front 与第二元素**，1402816ea..16fc 追加一次 failed_edges（保原 path 顺序，不排序/不写每个 Err ID）。故同 path 有一个成功段+一个失败段，不补 manual；全部段失败时只补从 path 首节点出射的一个 manual。direct 同墙 walk 分支没有逐段循环，其 CurveTryErr 仍将原处理的 sorted (a,b) 追加同 failed Vec。

短请求完整数值/顺序仍见 REQUEST_PREP_DIRECT_FALLBACK_CONTRACT.md 末节，失败 edge 的 b 供 metadata/anchor，几何仅 a 出射，不错误连线。

## 当前边界

这是闭合 producer 必须采用的原业务，不是 source 完成统计。实际7参数与Demo adapter、graph clear/遍历、best/flags、directwalk、isolated/manual、fixed-snapshot one-hop dirty 与scratch return合同须和本合同整体组合。host member0/1尚未锁 actual caller specialization，必须保未知而不挂 constfalse假门。Spline shared guard/count差异单列 canonical 依赖，不能借本地降级曲线掩盖。

## false 同分支补充配对（非仅 checksum 推断）

已从完整 false 原 ASM 实读 CurveTry 与 any_success 分支：27995a 传 reject_duplicates=true、279960 调同140153900；279965..98b 的 Err 仅跳下一段或 path 收尾。成功 push 后 279fd5..fd7 写0x938 flag=1；279ffc..27a013 仅该flag=0且path非空进回退，27a086/099取首两个ID、27a0bf..d1只 append 一次。与 true 同三分支策略。false 原实际 source calls 已独立导出 request-false-source-calls.json（完整 instructions/symbolization join），不是拿 true地址偏移猜 false。

ctor真实同callee也已配对：false27927b periodic140c930f0、2796a2 open140c926b0、279734 dense140809ad0，closure1在2795a4仍140285a90。各 source813/838两次edge required→position_offset、source864末点closure、877/891 ctor/dense顺序与 true一致；source920/921 required点、923/924 requiredalignment、927/928 locator、931/932双AdjustedEdge、934/937Info、942/943 derive、946 compute、967push完整次序逐项一致。
