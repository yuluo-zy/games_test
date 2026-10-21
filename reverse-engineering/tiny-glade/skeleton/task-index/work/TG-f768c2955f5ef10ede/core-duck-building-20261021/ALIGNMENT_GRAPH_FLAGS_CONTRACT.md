# alignment directed graph 第二遍旗位传播合同（待独审）

原true full14027e470，source530..584，bounded范围14027f23f..14027f6bd；false同源块待配对核。此inline业务不是库容器实现，必须保留，不能仅将preferences/best写入图后直接走曲线。原pack `native-pack/request-true-full.asm.txt`，原身份同FULL_REQUEST_CHAIN_CONTRACT。候选尚未写。

## 第一遍：收集所有节点的边更新

迭代刚重建的 AlignKind directed graph 的nodes保原顺序，对每节点按原outgoing邻边顺序读取weight（140241b90真实typediterator）：

- tag0：只记 has_straight=true（14027f3a0）。
- tag1：取 first 与 second，原mul/shuffle/sub在14027f3c6..3db计算 `first.x*second.y - first.y*second.x`。原signum构造后ordered大于0（3df..3f8），`cross.signum()>0`设置has_positive（含cross=+0）；其它（cross=-0或NaN）设置has_nonpositive。这里不能把`cross.signum()>0`缩写成`cross>0`。这里只分类，不提前调用outgoing_dir或改变payload。
- 其它tag（包括None2）：不参与。

若没有tag0，或没有任何tag1，则不生成该节点的更新（14027f410..424）。否则第二次遍历这个节点的所有outgoing边，只有tag<=1加入临时Vec `(node,neighbor, flags)`，`flags=(has_nonpositive as u8) | ((has_positive as u8)<<1)`（14027f4e9..4f4；push500..51c，stride12）。**先扫描整个图并收集，再第二阶段写图**，不边迭代边借用变更。

## 第二遍：对真实有向边更改flags

按上面Vec原序查图内 `(node,neighbor)` 的required edge_weight_mut。缺失对应原282810 panic，不跳过/默认插边。

- tag2不写（14027f68a..68d）。
- tag1：`flag=1`，不写byte2（14027f69b..69e→5dd）。
- 其它tag（实际收集仅tag0）：`flag=flags&1; byte_2=(flags>>1)&1`（14027f6a4..6b4）。first/second原值保留。

这两个旗会在已恢复 `AlignKind::position_offset` 与compute端点padding被真实消费，不能用bool默认值或调用方随意猜朝向。虽然未知tag不会从收集阶段进入，写阶段仍按原2、1、其余分派，不能凭enum未证排除。

## 现代边界与验收

实现在同现stairs.rs私有helper，借canonical `DiGraphMap<DecoratorId,AlignKind,RandomState>`；不新增World资源/持久化镜像。常规Rust Vec保临时生命周期，不重写原petgraph依赖。

测试需实际DiGraphMap覆盖：无straight、无turn、straight+单正/单非正/双向turn、tag2无变更、原tag0已有flag被覆盖、tag1只改flag保byte2、NaN/-0归nonpositive而+0归positive，以及节点顺序/多边真实图。候选只在本task合法work，parent独审后方写，不运行Cargo。
