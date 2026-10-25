# 可见楼梯结构图必要 canonical 语义合同

状态：完整7方法/必要类型源码准入待独审。TG-d1ffc27432f3d739f8原live NULL由core-stair-structure-20261021正常claim；root明确同链必要授权，唯一source候选writer core_business_lead。当前生产before74bb16d977340dca160aa117ee18ced6dd4fadf7f411b199badbfc9b6726b529，原B RNE f507/PDB2d9686，现Rust手工结构为起点，非旧layout复刻。仅本源及必要现代测试调用适配，不碰world/生产/Cargo/GPU。

## 原完整证据与实际调用

PDB482七完整procedure：clear140869a30/743B，upsert_span140869d20/623B，upsert_support_source_collider140869f90/87B，add_edge_by_span140869ff0/609B，add_edge_by_collider14086a260/181B，remove_node_by_collider14086a320/413B，disconnect_node_by_collider14086a4c0/692B（含原EH funclets）。native-pack/report.json全部ok、指令与PE逐byte绑定；PDB_PROCEDURES.json原边界独立于collector boundary=false。依赖typed-dependency-pack原GraphMap::add_edge490B(PDB510:20916)及IndexMap::get_mut210B(PDB1637:44268)仅锁类型/游戏callback上下文，不复刻依赖内核。connection-schema-pack原IndexMapCore::insert_full1144B(PDB590:46104)证明weight仅1byte搬运（入口保存R9B，桶+20只movb），不是R9D寄存器copy所暗示的4B字段。此前4B猜测撤回。

真实前阶段consumer：create_structural_colliders_and_initial_graph1407d20c0/10442B调用upsert_span与add_edge_by_span；calculate_distortion_from_structural_graph1407d49d0/17934B及其overlap callbacks调用其它方法。两62KB main本身没有此Res参数，不能把前阶段图冒作main参数。caller完整包由Duck在TG0ae3/mesh-producer-20261021/helpers-native-pack、structural-weights-native-pack保存。原两个addSpan站1407d3131/3726权重0，两个overlap addCollider站142008f5f/92ec显式R9B=1；完整原调用切片两个STRUCTURAL_*_CALL_SLICES.json。

## 类型/schema必要纠错

1. 原span key是tuple<StairGraphEdge,u32>，由hash_one141256660与双向insert14152c190/14152ba60实际alias共同证明，native每次搬三个u32。现仅8Bedge会覆盖同一边不同sample，必须改为既有语言tuple或同义typed alias StructuralStairSpan=(StairGraphEdge,u32)，不提供隐式sample=0兼容转换。现实际createStructural/staircollider registry已用该tuple，统一此身份。
2. 原GraphMap<SubworldColliderHandle,StructuralGraphConnection,Undirected,RandomState>。保持现StairColliderHandle安全包装及原slot/index/generation字典排序（原addCollider及GraphMap排序全核），复用pinnedpetgraph0.8.1。StructuralGraphConnection是一个bool包装，安全字段名transient代表本链已观察的行为：false结构连线保留，true重算时可断；该人读字段名不是PDB原字段名声明。原bool取值/Option None niche2由完整依赖搬运与业务AL门证明，不能改unit或任意u8。
3. 原graph+48 IndexMap就是GraphMap自己的edges，不是第二个业务索引。删除手工edge_index/预置register_edge_index假前提，直接用同graph.edge_weight_mut，不能有平行表。
4. 六原业务表：collider→span @90，span→collider @c0，collider→height f32 @f0，另一Handle→f32工作表@120，pending collider set@150，StairGraphEdge→u32 sample段数@180。后两缺表由actualconsumer 1407d244e..2459绑定+180、30be..30d2 typedHashMap<Edge,u32>插入；+120由1407d55d3/579d/5880..588e绑定Handle12+f32桶16、movss值参与高度计算。工作表人读名称working_heights不宣称原字段名/整distortion算法已恢复。补同owner两个真实typed表，不伪opaque/空方法。
5. maps/sets与GraphMap均复用std RandomState/pinned容器，不恢复哈希/表内核。保已有public wrapper，必要accessor按真实consumer提供同表借用，不能第二registry或从其它表合成数据。clear要清graph与上述六个真实表，不能因当前尚无field writer省略未知表；所有原值为owned Rust/POD，RAII保持所有权。

## 七方法完整业务顺序

clear：先graph.clear（保依赖nodes/edges各自容器及邻接Vec释放），再@90/@c0/@f0/@120/@150/@180依次clear，保分配策略不另shrink。

upsert_span(span,collider,height)：先从span_to_collider **remove**旧值（不是get）；Some且非原None niche时，先collider_to_span.remove旧，随后按graph邻接原顺序逐个把邻居插pending，再required graph.remove_node(old)==true，最后collider_to_height.remove旧。无旧不做解绑。之后graph.add_node(new)、collider_to_span.insert(new,span)、span_to_collider.insert(span,new)、collider_to_height.insert(new,height)、pending.insert(new)，顺序固定。不提前收集邻居Vec；它会多分配并改OOM/部分副作用顺序。graph邻接借用与pending可安全分字段操作。两个新typed工作表在此函数原未写，不能顺手清。

upsert_support_source_collider：仅graph.add_node→collider_to_height.insert→pending.insert，无span映射；保原f32，不加finite guard。

add_edge_by_span(span_a,span_b,connection)：先optional查A，无映射直接返回；再optional查B，无映射直接返回，原并不unwrap span。拿到两handle后调用同graph.add_edge(a,b,connection)，原返回Option<Connection>：只有Some(old.transient==false)才required同graph.edge_weight_mut(a,b)，将新weight.transient设false；None/Some(true)均保本次插入值。保原“永久false不会被后续临时true覆盖”的规则，不能误把新边None当required索引缺失。

add_edge_by_collider(a,b,connection)：同add_edge及old Some(false)粘着规则；原通过规范ordered pair访问graph自有IndexMap等价于同graph.edge_weight_mut，借pinnedgraph接口即可；不再自己登记第二map。真实游戏overlap caller传true，但公共入口仍保该第三connection实参，不擅默认true。

remove_node_by_collider：先height.remove；有值才依原邻接顺序逐项pending.insert，required graph.remove_node==true。随后collider_to_span.remove；有span则required span_to_collider.remove(span).is_some，保前面已发生清理，缺值不报错。仍不改工作表/段数。

disconnect_node_by_collider：同graph.edges(node)顺序，只对connection.transient==true的边，先pending.insert(neighbor b)，再push owned(node,b)待删Vec（增长失败前先前pending保留）。迭代完成后依收集顺序graph.remove_edge；false边留存、节点留存，不两端都标pending。原Vec/EH drop由Rust RAII承接，不复制容器实现。

## 调用迁移/验收与不能蒙混的缺口

实际当前生产调用除本源unit，仅systems/decorator/tests/decorator_unload_lifecycle.rs:181的upsert_span旧8B fixture需要显式改为(edge,0)，由该owner/中央协调最窄hunk；不是生产回退默认sample。Duck正在恢复的实际createStructural/calculateDistortion将直接用真实tuple与connection，不是测试预置edge_index。原2个全62KB mesh的前序调度/完整support算法、真实Info63/UI、visual/RT/Drawable清理仍各自闭合，不因这7方法完成宣称整个可见链。

测试应有同edge不同index共存/单span替换、缺span A/B早退、新边无需registry、两入口Some(false)永久规则/Some(true)保新值、自环、disconnect只删true并单端pending、remove/upsert故意图不一致时panic前双向表/height副作用、clear所有6表与graph真实数据。不放宽原数值或删除旧测试。root唯一actualCargo/nativephysics/GPU；source独审与F0不是runtime。B证据不外推C。
