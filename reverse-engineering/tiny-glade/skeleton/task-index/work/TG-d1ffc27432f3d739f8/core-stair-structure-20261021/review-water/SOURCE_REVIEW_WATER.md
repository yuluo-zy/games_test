# Graph V1冻结源码独立审查

**通过，无静态阻断。** 绑定SOURCE_DELIVERY_V1.json f6ab11dad8839b2110a4b745ecaabb76aa65a00439a9576f7168565e6be90287、主source fdb30f02930444c40e9d5c8b3909500ddb1c48b8f115568c181093f3505c6e68、主patch99e842781ad1b1571c4c14b5a00d3727656651cfbd4f49a1e075b7620814250f及已准入合同V2 36219830。审者不是本源作者；本次重新读完整candidate、两patch/实际caller、原七methods/typed依赖/前独立PE逐字节复核，不以合同通过替代SOURCE审。

## 业务与失败顺序

1. tuple span明确带u32 sample，主双向表同一身份，无implicit sample0转换；StructuralGraphConnection只包装bool，graph自有weight负责永久false粘着，已删伪edge_index和register入口。两入口都通过同insert_connection实现原oldOption三态；None新边不额外查询索引或panic。StairColliderHandle原包装/Ord/Hash逐字节未改。
2. upsert原span_to_collider.remove先于inverse remove；每邻居即时pending，继之required graph.remove_node、旧height移除；新值add_node→inverse→forward→height→pending次序完整。没有邻接collect或事务回滚。
3. 87B纠正已真正入source：support仅collider实参，固定+0height，无finite guard/伪外来height参数，且add_node/height/pending顺序正确。
4. remove先height.remove，有旧才邻接pending/required graph节点删除；再inverse.remove与required forward.remove。故两个不一致失败测试验证的部分副作用符合原；不存在提前校验掩掉已发生移除。
5. disconnect按同graph.edges顺序，仅true边先pending(neighbor)后push owned pair，之后逐pair remove_edge；false、自环、query节点处理交pinnedpetgraph，没误标双端、没全边collect再mark，保原Vec增长前pending副作用。
6. graph+六业务表由同owner组成，RandomState类型与pinnedGraphMap默认stdRandomState核过；clear原顺序graph→inverse→forward→primaryheight→workingheight→pending→count，无shrink、镜像、默认geometry。parts_mut给七真实字段同生命周期mut借用，零clone/零分配；最终名相对合同拟assembly_parts仅接口定名，已在SOURCE_SCOPE记录。

## 现代闭包/测试/API

- 外部adapter patch唯一改卸载fixture输入为显式(edge,0)，beforec7deea3a→after10bffbae，原先单sample含义保留，未改卸载断言/其它fixture。现production实际唯一external directupsert caller迁移完整；Duck前阶段新caller另需以tuple/typedconnection和真实parts_mut消费，不能假定本包恢复整个前阶段。
- 14测试实际API逐项读核，覆盖原四核心闭包及新sample身份、NaN/-0height保留、support+0、missing spans、sticky false两入口/newedge、自环、只删true和单端pending、三失败部分状态、真实六表clear及parts_mut写入。未删测试放宽原断言。支持现pinnedpetgraph0.8.1 GraphMap.neighbors/edges/edge_weight_mut/Default与hashbrown0.14.5 RandomState用法；没有现代API已知静态不匹配。
- 独立正/反F0对两文件均rc0且每after完全匹配交付hash；edition2024 rustfmt skip_children解析两文件均rc0，不改冻结candidate。完整mechanical JSON另存。

本审查未运行Cargo/rustc/GPU、未修改生产或作者candidate；**运行测试仍归root执行**。静态通过不证明两62KB装配器/17KB distortion/真实Info63输入/调度/可见mesh卸载已恢复。B证据不泛化C、未声称HashMap随机迭代与OOM跨实现逐位/逐事件完全等价。
