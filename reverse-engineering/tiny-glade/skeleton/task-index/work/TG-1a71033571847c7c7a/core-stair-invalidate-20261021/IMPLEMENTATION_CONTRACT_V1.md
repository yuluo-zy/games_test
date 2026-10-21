# 楼梯脏边失效完整实现合同 V1

本轮 TG-1a71033571847c7c7a 已 normal claim 为 core-stair-invalidate-20261021，实际 live receipt 随包。当前现有 mapped 文件内恢复一个完整业务系统，不新建生产模块。原始证据只覆盖 B；索引 C 的同源 checksum 不作为 C runtime 等价证明。

## 原证和实现边界

完整原 140b37180/4290B，PDB586:12512；当前头3455B是错误旧注释，应纠正。RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。原完整 asm/json 与四个必要叶已经在旧合法工作目录，本轮只复用，不再采集整程序。

本合同完整继承 /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ad9668ff273a982820/core-stair-visual-20261021/readonly-invalidate-water/INVALIDATE_FULL_CONTRACT.md（hash cb646a2fff69f67b2c9b679bf2a004c1dc00e5e220797a637e60a303603af6dd）的原地址、字段和全部控制流，下面把旧报告所列真实缺口绑定到已落地 API，而不改旧冻结。source before 与当前依赖见 inputs.json；root 唯一生产整合、Cargo/GPU/Git 执行者。

原八个 owner 顺序保留：ResMut<RaycastWorld>、ResMut<StairColliderRegistry>、ResMut<StairsState>、ResMut<StairsStructuralGraph>、ResMut<StairsVisualState>、ResMut<RtWorld>、ResMut<PlayermadeStairsAssemblyRequests>、Commands。实际碰撞注册资源仅 native-physics 可用，所以本系统和必要 re-export 也 cfg(native-physics)，不保无资源两参数假兼容函数。无新增注册；原宿主由 root/peer 配对。

## 完整顺序

1. StairsState.drain_dirty_edges 全部 drain 到 HashSet<StairGraphEdge,RandomState> S；保源容量归属与 iterator 析构。无提前空集 return。
2. visual.clear_dirty 无条件执行。
3. 对唯一 requests.requests 按 Vec 原顺序：p.dirty = p.dirty || S.contains(edge(p.segment))；先写 dirty，再把 true 项 edge 插入独立 T。S/T不合并，不删除 Params。
4. T 非空才逐 T 删除同 StairsStructuralGraph.parts_mut().segment_counts 对应项；不删图节点/边、不使用独立allocator。然后同 registry.structural_colliders.retain：仅 T.contains(key.0) 的项先 world::dispose_collider_mesh_instance(&mut world, instance)，再由 retain 删除值并析构 ownedArc。index 不参与命中、未命中 collider 不动。T 空不跑 retain。
5. 每个 S 按其 HashSet 原迭代顺序处理：registry.remove_edge_colliders(world,edge) 的真实三 lane 顺序 0→1→2；每 lane 先从真实 map remove，dispose 真实 Rapier/世代/槽/dirtytile/publisher，再 drop 当前 owned mesh，之后下一 lane。
6. 在三物理删除之后 stairs.contains_edge_or_stub(a,b)。true 仅 visual.force_rebuild(edge) existing-only；false 先 visual.remove_edge(edge,&mut Commands)，随后依权威 stairs.graph().neighbors(a) 顺序逐项 force canon(a,n)，再独立 neighbors(b) 顺序 force canon(b,n)。不 reset、不新增 state、不排序/去重/方向过滤，a==b仍两循环。
7. 同 S 当前 edge 用 canonical StairSegment::new(DecoratorId(a),DecoratorId(b)) → StairGraphEdgeIdentity；packed=(a<<32)|b，与内存 qword 不同。RT key tag1，调用本轮必要 consuming helper remove_mesh_if_present：Some先drop实际 DirtyCow/Arc后写 topology_dirty；None不写。不能用旧 remove_mesh 的无条件 dirty。
8. 同 identity 依次 requests.previous.sub_segments.remove、arch_wall_supports.remove，各返回值在下一步前析构。保 sampled_points、其它 history、相邻 RT/previous；不清 graph/current requests。

所有标准容器和泛型的分配、panic 与 unwind 由 pinned 依赖承担，不复现容器内部。原 bounds/index 不合法输入在现代安全迭代器域被类型排除，无 fallback geometry 或错误吞并。没有 Demo gate。

## 依赖实际配套

- Current structural_graph 已有真实 segment_counts 和 parts_mut 7借用 API，after fdb30f02930444c40e9d5c8b3909500ddb1c48b8f115568c181093f3505c6e68；不再写该 foreign 源。
- Current Requests 已完整拥有 Params.dirty/segment 和 previous 三表；按实际类型借用，不新镜像。
- Current stairs_collider.rs 已有三个 edge map 与第四结构 map、实际 world remove/dispose；直接复用，无新 fake deletion callback。
- Tree 唯一 visual source writer。必须配对 VISUAL_SCHEMA_AND_LIFECYCLE_CONTRACT_V2 / RESET_ORDER_V2_ADDENDUM：force existing-only；remove(edge,&mut Commands) Some先drop真实 allocation/cache，再 globaldirty=true，再排队实体 despawn；None不dirty。invalidate 从不调用 reset。Tree未冻结 source 前不能宣称此候选整合已运行。
- RT 旧任务 TG-81dc0cd75bc98816f4 owner main-chain-frame-20261018 保持原 claim。最窄 direct helper 合同位于 /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-81dc0cd75bc98816f4/main-chain-frame-20261018/delegated-stair-invalidate-20261021/CONSUMING_RT_REMOVE_CONTRACT_V1.md；不修改旧 remove_mesh/remove_meshes 及既有 prefab/update_rt_world/BVH 调用顺序。
- stairs/mod.rs 当前无独立 source_path task，经 readonly SQLite 确认。只在本工作包产 one-line native-physics re-export cfg adapter，交 root 同批整合，不虚构 task/claim。除旧同文件测试，当前 rg 未找到其它该函数 direct caller；宿主注册仍 root/peer 负责。

## 有意义 CPU 测试计划（未执行）

使用真实 Bevy World/Schedule 与八资源，真实 RaycastWorld 及真实 mesh/collider，Commands deferred 实体删除；不以日志/计数器替代物权。
- S空但视觉dirty：无条件clear；already-dirty Params形成 T，结构计数/第四表真实删，前三lane/RT/visual/previous保持。
- S存在但视觉缺失：三lane真实dispose、世代失效/Arc强计数释放；force不创建state；缺RT不新增dirty；仅自身两previous删除，sample历史保留。
- S缺边且两端各邻接：先remove自身并deferred despawn，邻边existing-onlyforce，缺失visual不造；不同tag RT与无关edge/history保留。
- 同一edge两个结构sample都删除、别edge保持；T/S分别构造以防错误union；already dirty短路与多请求共享edge保持。
- source drain完成、missingRT不会dirty、existingRT在返回前drop；RT独立4边界测试覆盖同owner真实Arc、同payload其它tag和旧API行为保留。

本合同准入后才写候选；冻结 patch/after/源依赖、独立 -F0 正反重放、rustfmt parse。Cargo/真实GPU/全游戏验收留 root，不能以本合同或静态检查当实现完成。
