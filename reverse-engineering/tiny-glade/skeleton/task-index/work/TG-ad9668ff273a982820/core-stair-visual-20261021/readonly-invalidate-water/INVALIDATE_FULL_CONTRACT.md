# invalidate_dirty_stair_artifacts 全控制流只读合同

任务边界：TG-1a71033571847c7c7a 本轮未 claim、未写源码。此合同与必要有界原证落在已授权 TG-ad9668ff273a982820/core-stair-visual-20261021/readonly-invalidate-water。没有 Cargo/rustc/GPU、生产改动或新的业务族。用于 root 决定同一 lazy closure 的最窄成组恢复，不作为该系统已实现的证明。

## 原身份和范围

- B RNE：`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- PDB：同目录 `tiny_glade.pdb`，SHA-256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`，GUID09F1A02A-49F2-F8C8-4C4C-44205044422E，age1。
- 主函数 `140b37180/4290B`，PDB `586:12512`。复用 `../producer-native-pack/invalidate-lazy-producer-full.{json,asm.txt}`，原 instructions 逐字节 verified，range 无 overhang，包含正常路径、bounds panic 和 EH cleanup；不沿现文件头3455B宣称完整。
- 原 remove visual `140aa8620/1087B`，PDB `562:24656`，复用 `../native-pack/remove-edge-full.{json,asm.txt}`。
- 旧反编译仅辅助导航：`tiny-glade/crates/country-core/recovery/agent-team/work/logic-roofsstairs/decomp/invalidate_dirty_stair_artifacts.c`。以下裁决以原汇编、PDB 类型和真实 owner 为准。
- 本轮仅新增4个必要完整依赖范围，`native-pack/report.json` 四项全 ok：dirty params fold `141477ff0/460B`（内含游戏 filter_map callback）；structural collider retain `141268520/583B`（内含游戏 retain predicate）；remove collider `140252ad0/438B`；contains edge/stub `140900e80/335B`。不恢复 std/hashbrown/petgraph 内部算法，游戏 callback 单列。

## 输入 owner 和键的三种身份

匹配实际寄存器/栈访问的八参 typed 函数指针库存 ID `f576eb5984a3d27bff6e342f`：RaycastWorld、StairColliderRegistry、StairsState、StairsStructuralGraph、StairsVisualState、RtWorld、PlayermadeStairsAssemblyRequests、Commands，均前七 ResMut，最后 Commands。入口 RCX/RDX/R8/R9 依次为前四个 wrapper；caller stack20/28/30/38 为后四个（callee 以 RBP+220/228/230/238 访问）。`typed-owner-navigation.json` 保留完整拼写与 wrapper 索引；原库存本身的 binding_to_game_function=unknown，不能单凭它声称调用点已绑定。此处 owner 识别同时使用真实表类型/偏移/下游调用：R9+180 的 `(StairGraphEdge,u32)` 表、Requests+168/+198 的已证 prev maps、RT typed remove 和 Commands::despawn。系统注册真实 callsite 的直接 type/function-pointer 绑定留给原注册 owner，无新注册候选。

1. `StairGraphEdge`：两个有序 u32 `(a,b)`，canonical 构造 unsigned min/max，表示无向边或 `(a,a)` isolated stub；哈希逐字段读两 u32。源 dirty 集合/三 collider 表/visual 表/StructuralGraph+180 表使用此键。
2. 邻接记录：权威 `StairsState.graph` 的节点 IndexMap value `Vec<(DecoratorId,CompactDirection)>`，stride8；读取邻接 id，CompactDirection 不参与本函数过滤。后续每边用 unsigned min/max 重新生成 `StairGraphEdge`。
3. `StairGraphEdgeIdentity`/RT payload：`u64 = (u64(a)<<32)|u64(b)`，原 `140b379dc` shl EBX32 后 or R15。这与内存 `{a:u32,b:u32}` 的原生 qword low-a/high-b 不同，不可 bitcast 替代。RT key 是 `{tag:1,payload:identity}`，Requests 两个 previous map 使用同 packed identity。

## 两套集合：不能合并成一个 union

定义 S=本轮图编辑脏边；T=本轮装配参数受影响的边。两个集合均为真实 `HashSet<StairGraphEdge,RandomState>`，去重采用锁定容器，遍历维持容器顺序，不排序。

1. `140b371cd..3731c` 从 StairsState+148 `HashSet::drain()` 取全体，收集到新 S（最终 header 复制至 RBP-50）；对源 dirty set 的 drain/归还容量和 unwind 使用依赖实现，不用假 clear callback。源集合在后面副作用开始前已 drain。
2. `140b37356..37377` 取得 visual mut owner，**无条件把 visual 全局 dirty(+30) 写0**。S为空也执行，不可用 for-loop 非空前提提前 return。
3. `140b3738e..37435` 对 Requests.requests 全部 Params 以0x150原stride迭代，调用 `141477ff0`，生成新 T，S只作为只读捕获。原处理没有从 Vec 删除或清空 Params，也没有清空 S。
4. callback 的完整语义：`p.dirty = p.dirty || S.contains(edge(p.segment)); if p.dirty {T.insert(edge)}`。原非空 S 时 `141478093` dirty==1 直接保true；否则查S后 `14147806d` 写1，未命中 `141478149` 写0。S为空快分支仍逐项读取/回写 dirty，原true入T，原false保持false。写 dirty 先于向T插入，因此原分配失败不能被改成先完成整个T再写 bool。
5. **T 可以含 S 之外的边，也可能完全不含某个没有Params的S边。** S/T不能 union 或替代。后面 `140b375ac` 明确重新从 RBP-50 读取S主循环，既不是T，也不是union。仅 Params dirty 而不在S的边只触发下节结构清理，不能额外删其三主碰撞体/visual/RT/prev maps。

## T 结构失效阶段，先于每条 S 的物理/visual处理

当T非空：

1. `140b37491..37553` 遍历T。对 **同一个 StairsStructuralGraph owner +180** 的 `HashMap<StairGraphEdge,u32>` 调用 `140b77c80 remove_entry`，忽略原 Option 返回 u32。该 map 的业务字段名未证，但键值与 owner/偏移已证；不因此改删整个结构图、不调用 remove_node_by_collider。表不存在该键时无操作。
2. `140b37559..375a0` 调用 `141268520` 对 `StairColliderRegistry+90` 第四张表 `HashMap<(StairGraphEdge,u32),ColliderMeshInstance>` retain。真实 predicate 为 `!T.contains(key.0)`；sample index不参与T命中比较（原 `141268635..668` 仅比较键的前两u32）。不是只匹配index0，也不是清整表。
3. 命中T时，先 `1412686bd -> 1408d9590 dispose_collider_mesh_instance(world,instance)`，原返回mesh Arc随后 `1412686c8..6d2` drop，再由 HashMap retain删除当前条目并继续；未命中条目和真实碰撞体均保持。world副作用必须复用真实dispose：Rapier collider释放、registry/generation/free list、槽/瓦片dirty、ColliderDiff移除发布。不允许只删表或记录日志冒充删除。
4. T空时完全跳过以上两步，包括第四表 retain；然而上节 visual globaldirty reset 与 Params逐项 dirty更新仍已执行。

**API缺口**：当前 StructuralGraph 没有+180真实表；现独立 `TransientStairEdgeIdAllocator` resource 不能作为该owner替身。当前 StairColliderRegistry 已建真实前三lane和structural_colliders，但没有本predicate的retain接口；可借用现world::dispose_collider_mesh_instance逐条真实删除，需保持同registry和world，不新建镜像表。

## S 主循环：准确顺序和三个静态 force 站点

原S为无序HashSet遍历，`140b376e2..3774e` 每次取 `(a,b)` 到 EBX/R15D，不排序 S。

1. **三个物理 collider owner逐一删除**：`140b37787/377d7/37827 -> 140252ad0`，registry lane偏移+0、+30、+60，全部用当前同一个 `StairGraphEdge(a,b)`，world为入口RCX所有者。每项 missing 返回None（native tag0x11）且不触碰其他lane；命中先dispose再在 caller drop返回的mesh Arc，然后才处理下一lane。三lane语义名称仍未证，不能据位置命名成虚构的不同功能。结构第四表已在T阶段独立处理。
2. `140b3785a -> 140900e80 contains_edge_or_stub(stairs,a,b)`，**在三次物理删除之后**查询。
3. 存在（AL!=0）：`140b37876..379c3` 仅对visual表的**当前自身边 `(a,b)`** existing entry 写 request byte1。该表空或找不到键时跳到汇合点。**没有 reset 调用、没有 upsert、没有邻接遍历**。原fullbody没有 `140aa8a60` reset call。
4. 不存在（AL==0）：`140b37960..37973` 调 `StairsVisualState::remove_edge(visual,a,b,Commands)`，然后继续：
   - 先读取权威 graph 的 node a 邻接 Vec，按其实际迭代顺序遍历每个 n。`140b37c23..c49` unsigned min/max生成 `(min(a,n),max(a,n))`；表命中才在 **140b37c10** 写 request byte1，miss无操作。
   - a 的所有邻接完成后，`140b37cb0..d74` 才读取 node b 邻接；`140b37e03..e2b` 生成 `(min(b,n),max(b,n))`，在 **140b37df0** existing-only mark。
   - 没有方向过滤、sort、dedup、contains_edge二次门，也不创建missing visual。node缺失邻接为空。a==b时两个loop仍分别运行，相同边可能重复标记，不为性能做未经证明的去重。原IndexMap坏索引分别在140b37f0b/7f1f bounds panic；合法Rust iterator复用容器契约即可，不造fallbackgeometry。
5. 三个 store 是三个**静态站点**：自身一次站点、a邻接循环站点、b邻接循环站点；邻接每个站点可以运行多次，不是固定只标三条边。两分支最终都在 **140b379d0** 汇合。

`contains_edge_or_stub` 的335B完整函数同时核过：a!=b规范化后查GraphMap edge；a==b要求节点存在且邻接Vec长度0（孤立stub），有邻边的同节点不是stub。current StairsState 的该API/graph getter可复用，不能用“节点存在就stub”代替。

## 每条 S 的汇合删除：RT → sub_segments → arch_wall_supports

1. `140b379dc..37a1a` 构造 `{tag1,payload:(a<<32)|b}`，从真实 **RtWorld.meshes** remove_entry；native returned Option niche==3表示None，None完全不写RT dirty，Some先drop DirtyCow/RtMesh Arc，再 **140b37a57** 写RtWorld+38 topology dirty=1。不新增empty RT mesh、不删tag0/tag2或邻边RT。
2. `140b37a67..37a93` 用同packed identity从真实 **Requests.previous.sub_segments**（native +198）移除，Some的SmallVec按真实inline/heap状态析构，不能用Vec::clear替换全部历史。
3. `140b37ad0..37af8` 同identity从 **Requests.previous.arch_wall_supports**（native +168）移除；Option返回按原trivial值忽略。
4. **没有删 previous.sampled_points(+138)**，没有清全部Requests/graphs，没有删neighbor的这两个map。末尾释放S/T临时表；无显式Demo或global laziness gate。

**API缺口**：现RtWorld::remove_mesh会在missing时也无条件置topology_dirty=true；该系统不能直接调用它并称missing行为等价。须最窄恢复“actual removal才dirty”的真实RtWorld API或经其owner修正同语义，然后调用。不要从本consumer直接另建dirty镜像。最新完整 Requests候选已由duck恢复同owner previous两表，可直接typed remove；生产旧Vec-only模型暂不满足，必须原包成组集成。

## remove_edge 的必要副作用与边界

原remove_edge完整1087B证明它接Commands，不接RtWorld；RT删除由上节本系统单独完成。当前source文档把这里的RT列“胶水”不能替代调用。

- 先从visual HashMap移除整个edge value；None直接返回（140aa8680/86ad跳8842），**不置 globaldirty**。当前 `visual.remove_edge` 对missing也dirty=true存在实际差异。
- Some依次析构两个真实分配slot（140aa86e7/86fd→14087f390，相同type-erased释放机制）；旧值中的实际集合/向量、MagicSupports（140aa8787→14088f920）按原拥有关系析构；再140aa87ae置visual globaldirty=1。用真实allocation Drop可入waste，但当前 `_opaque` 尚无那些owner，不能拿 empty unit 假称完整释放。
- 原保留的 Vec<Entity> 长度/ptr先搬出，随后140aa87e0..881e逐Entity构建 EntityCommands，**140aa880e→140d14770 EntityCommands::despawn**；原loop遇Entity高32位为0提前结束。命令延迟在原schedule消费，不用同步World fake替代。缺实物Vec就不能用无操作 callback 交差。
- 本轮只能明确这些缺口及完整原范围；value中其余 cache collection 的业务字段精确schema仍须由其真实62KB assemble producer/consumer闭包统一给出。没有为这项只读合同新恢复未知缓存家族。

## 当前源码与已审候选的最窄配套清单

| 对象 | 当前实际能力 | 必要配套与边界 |
|---|---|---|
| invalidate.rs | 只有2 Res，drain→present reset/absent remove；测试要求present凭空创建 | 应恢复上述两集合、原八owner与完整顺序；原S只mark已有，当前测试的“present reset创建”应改真实原行为，不保错误兼容 |
| StairsState | canonical graph getter、drain、contains已具备 | 可复用同graph.neighbors；不加平行neighbor表，不更改graph membership |
| visual.rs lazy候选 | aa5e84f63204e496991be005ae6a312a80f3ade509a3135322bdef6e96449bec已独审：force existing-only/getter/reset0 | 原进入系统globaldirty reset可用现clear_dirty；remove missing行为/实体Vec及其余owner仍缺，不能因此验收remove完整 |
| StairColliderRegistry | native-physics三lane+structural表真实owner；remove_edge_colliders按lane顺序可用 | 第四表真实retain predicate/世界dispose需补；无新的资源镜像 |
| world.rs | remove_collider/dispose释放实际mesh_sources和ColliderDiff | current ownedArc在dispose返回时析构可作现代所有权适配；不能再clone一份Arc模拟nativecallerdrop；具体源对照/原publisher正确性由world唯一owner验收 |
| StairsStructuralGraph | 真实图和若干表，但+180表仍缺 | 必须同owner补 `(StairGraphEdge,u32)` table及真实producer；只删一个凭空空map不能算清理闭环，不能借独立TransientAllocator替代 |
| RtWorld | 同一meshes/COW已存在 | actual removal才dirty的API缺口；现remove_mesh missing过度dirty需owner最窄修正/明确调用 |
| Requests | 生产旧Vec-only | 配对duck complete-owner候选after `2a046da8ee6e909304adc12dc4901a64e1bd4a44cb4aa506b7b4085ce29f5e1c`，requests pub Params.dirty/segment、previous两个typedmaps已在同owner；不删sampled_points |
| Commands/host | 当前系统根本不接 | 必须恢复真实Entity ownership和deferred despawn，接原统一调度顺序；不以日志、占位callback或新建World替代 |

## 建议验收场景（未执行）

- S空、Params也空：visual dirty从true变false，owners均无无关变化。
- 仅Params dirty：T包含该边，保S空；只删除同边StructuralGraph+180/第四structural colliders，三lane/visual/RT/prev两表保持。
- S边仍存在、visual不存在：三lane删除，force不创建，RT/prev两表按该边删除，neighbor不标记。
- S非existent边且a/b各有多个真实邻居：先remove自身，依a邻接顺序再b邻接顺序 existing-only；无关边、缺失visual、sampled_points保持。a==b stub消失分支保重复遍历语义。
- RT/visual删除missing：不额外置各自dirty；RT实际命中后才topology dirty。验证既有Arc强计数/owner析构以及真实collider世代和publisher增量。
- 非emptyEntity真实owner释放：按原命令queue despawn而非日志；slot/waste、MagicSupports、缓存拥有关系完整验证。

以上只读事实已冻结；未声明此系统、lazy闭包或游戏恢复完成。注册/完整visual value schema/world真实副作用仍需相应唯一writer给出可运行闭包。
