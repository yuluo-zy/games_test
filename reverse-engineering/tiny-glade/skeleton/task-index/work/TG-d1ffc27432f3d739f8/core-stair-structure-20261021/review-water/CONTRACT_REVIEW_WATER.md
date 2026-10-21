# StairsStructuralGraph 七方法独立合同审查

审者Water未写本source或候选。本审查绑定V1合同595e1382dd5162e446dece7ea199b018f74d73ec832490aa02cc9b0b65fc8a13、当前生产74bb16d977340dca160aa117ee18ced6dd4fadf7f411b199badbfc9b6726b529与现有完整7方法/3依赖原包。独立重读原ASM，并逐指令重新对同f507原PE比字节，全部吻合；PDB七个独立procedure边界总3348B。不是仅相信旧章节O标记。未新采集、未Cargo、未写生产或候选。

## 发现并已由作者接受的必要纠正

**V1的support叶不能照写。** 原140869f90/87B只接图和collider，fca的height插入前fc7明确 `xorps xmm2,xmm2`，因此写入+0.0，不接外来height。真实API应为 `upsert_support_source_collider(&mut self, collider)`，顺序add_node→height.insert(collider,+0)→pending.insert。两个真实overlap caller142008f3d/1420092c8均仅准备RCX图/RDXhandle，再在后续addEdge设置R9B=1，亦没有height实参。作者已明确接受补V2并撤回V1这一句；不保留忽略任意外来height的伪兼容参数。应加非零旧height被本叶覆盖为+0的测试。

除该已接受纠正，下列合同内容可直接源码化；最终准入必须与作者V2补充合读，**不标旧V1全文通过**。

## 逐项原证复核

- **类型：** span原每次三u32（旧两ID+sample）在addSpan查A a055/a05a及查B a14b/a14f均完整比较12B；upsert eea/f20两个反向insert各搬12B。原真实caller7d3107..3120分别构造sample i/i+1；(edge,u32)不可降为edge或兼容隐含sample0。registry现同tuple，统一合理。
- **bool weight：** 原GraphMap.add_edge93add0入口虽搬R9D，调用IndexMapCore.insert_fullb4ea10后真正value只movR9B→栈37、桶+20按byte读写（b4eb89..95/b4ec58），None niche2，不能误立4B struct或unit。addSpan a210..219及addCollider a275..27f truth表仅oldSome(false)进入weight_mut并强制false；None与Some(true)均保本次新weight。已有false永久连线不会被true覆盖，新边None不panic。+48由同GraphMap.add_edge93adec和93ae7a明确指向自己的edges IndexMap，删除平行edge_index正确。
- **clear：** 完整a30..d17先清GraphMap节点表/逐节点邻接owned Vec/边表，随后90、c0、f0、120、150、180顺序clear。补两真实表必须，不能因现代producer暂空而省略。依赖clear保outer allocation、释放被清节点的邻接Vec；复用pinned容器，不照汇编复制hash。
- **upsert_span：** d47..d74从span map先remove旧（原b78290），d a0..dc9再移除collider inverse，de8求邻接并e3e逐个pending.insert，之后e65 required remove_node、e9b height.remove。无旧直跳ea5。新值依次graph.add_node→inverse.insert→forward.insert→height.insert→pending.insert，无误；不能改get，也不提前收集邻居分配Vec。原未写working_heights/counts，不能补顺手清。
- **remove_node：** a356先height.remove，有旧才取邻接逐项pending并a3f5 required remove_node；然后a426 inverse.remove，有span才a470 forward.remove并断言Some。失败以前已经发生的map/graph/pending副作用必须保留，不事务回滚。
- **disconnect：** a57a迭代真实graph.edges(query)；a589 byte0跳过，非零才a5a8 pending.insert邻接b，随后a5c8容量判定/Vec grow并push(query,b)。完成迭代后a671按Vec顺序remove_edge；false永久边不删、查询node不删、不额外标query端点。空邻接/不存在node复用依赖空迭代。EH funclets释放owned Vec，没有回滚先前pending。
- **新增表：** 原create7d244e等绑定graph+180、30be..30d2取Params两个ID作Edge键、EDI u32作value调用typedinsert，真实Edge→u32计数表成立。distortion7d55d3/55de为graph+120及其hasher、579d读取该表、5880 shift4桶stride16/5887 movss(-4)并588e加浮点偏移；key比较slot+双u32，故Handle→f32 working_heights成立。名称是现代语义标签，不冒充PDB字段名。完整distortion计算不因此通过。

## 实际现代调用闭包与源码验收要求

当前真实生产除本源unit，只有decorator_unload_lifecycle.rs约181调用upsert_span，显式换为(edge,0)即可；该fixture给graph填数据随后真实owner卸载，不能新增假生产sample默认值。现stairs_assemble的createStructural/distortion仍是旧空实现，Duck本轮唯一writer正在恢复；现代真实registry已为(edge,u32)，其Graph接口应消费本canonical tuple/connection与两表借用，不能留第二registry。现lifecycle替换Graph::default的同owner顺序不因schema变更需要重排。

新增source应保持Public StairColliderHandle现安全包装与slot/index/generation排序，connection使用bool值类型、petgraph/std RandomState按原owner；所有真实表由同结构拥有并可借用，不能从其它表临时合成working_heights/counts。对remove失败副作用、永久false覆盖、missing spans早退、自环、同edge多sample、pending单端与clear6表补实际测试。新support叶无height参数必须同步所有caller（当前生产无外调，Duck候选已通知）。

**结论：有一项已接受纠正，V2按真实2arg/+0 support叶后七方法准入；其余内容无静态阻断。** 这是合同审查，尚无本source源码或运行验收。
