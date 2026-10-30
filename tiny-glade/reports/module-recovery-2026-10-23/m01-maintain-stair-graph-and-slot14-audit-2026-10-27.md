# M01 楼梯图维护与后续18叶只读审计（2026-10-27）

本轮在 B 既有 `decorator_visual/stairs.rs` 恢复 `maintain_stair_graph` 空壳；对后续 slot14 只调查原身份、顺序与当前业务体，不将空图、默认资源或挂接残体算作可用。没有扩展整个 M08，也没有修改其他 owner 的 requests/interpolate/楼梯输入函数。

## 已恢复的原入口

原 B 文件 `crates/systems/decorator/src/decorator_visual/stairs.rs`，函数 `140806110`，PDB compiled procedure 3963B（含 EH）。原 RNE SHA256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配 PDB 为 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。该函数完整原字节 SHA256 为 `484f17650dcc83848e71d7ec59e57d661478804567ae43c9b8b1a4351f675896`，见 [原汇编](m01-maintain-stair-graph-140806110-full.asm)。

B 的完整 FunctionSystem 单态证明九组参数，顺序为 `History`、`DecoratorPhysicsColliders`、mut `DecoratorStorage`、mut `DecoratorArchivist`、mut `StairsState`、mut `StairsStructuralGraph`、mut `RaycastWorld`、mut `StairColliderRegistry`、`DeferredDecoratorOpEvents`。旧实现只收三参数且空体，旧注释“8参”也错误。现在九组都真实消费；第十个现代 `DemoModeEnabled` 仅承接原进程静态 `_DEV_OVERRIDE_IS_DEMO_MODE`，没有创建镜像。

原入口 `140806149` 经导入槽 `14362c098` 读取 `143634d90`。已从匹配 PDB PublicSym32 section3 offset40336 核定其为 `_DEV_OVERRIDE_IS_DEMO_MODE`，不是回放或随意设计的开关。该地址位于 PE .bss，`pe.get_data` 原始文件返回空不代表类型/初值未知；正确映像映射包含零初始化。不能把 raw offset 当 RVA。

### 原业务与失败顺序

1. demo 开启立即返回，连损坏桶索引也不读取。
2. 读取 recording；遍历原图节点，缺地址或有桶而缺记录的节点收集到待删 Vec。有地址而缺桶 required panic，**先收集全部，再修改任何图**。
3. 从 Vec 尾 pop。仅 recording 时枚举当前邻居；邻居 degree 恰为1且 `is_decorator_cull_allowed` 才同时追加 cull Vec 与待删栈。degree0不等于1，degree2在当前节点移除后变1也不会倒推补删。用 petgraph0.8.1 无向 `Neighbors::size_hint().0` 读取精确 slice 长度，保持原 O(1) 检查，不逐邻居扫描其全部邻接表。
4. 调用同一 `StairsState::remove_node`，保留其 checksum/backup/dirty-edge副作用；原 endpoint 非unset且节点相同时清除。使用中央修好的32位原判别，padding不改变unset。
5. 只遍历原 registry **lane0**，把已不在图中的 edge/stub 收集为 stale set。原采用 RandomState，新实现用同类 std HashSet，不另建持久边表。
6. structural采样表先 retain：只对 stale set 中的 edge 调真实 `dispose_collider_mesh_instance`，保持真实 Rapier移除、generation/free-list、dirty tiles、ColliderDiff发布与 Arc释放。之后逐 stale edge 依次 remove三条 edge lane，再删同一 structural owner的 segment_counts。
7. **不额外清所有“图中不存在”的结构记录**：没有lane0键的 structural-only记录保持原样。也不提前 `structural_graph.remove_node`，原节点图由真实 ColliderDiff订阅消费者处理。
8. 最后按 cull Vec 顺序处理仍有真实存储地址的节点：归档原桶与两张真实装饰碰撞 owner，再走现 `apply_decorator_op` 的原 remove分支、kind0、History None和真实 DeferredEvents。已无地址者跳过，原缺桶/缺记录失败不隐藏。

原 `140806bff` 只写第三位置载荷+12为2（Option None）；xyz未写、属于inactive。当前初始化inactive字节为零是Rust安全表达，保留判别2，不能把它称作实际零位置，也不能写 `!recording` 0/1。原移出 Info由 `apply_decorator_remove` 取得并写事件，不用默认/陈旧Info替代实际记录。

### 唯一必要共享 API

按中央授权，在 `country-core/resources/walls/decorator_archivist.rs` 抽出 `LiveDecoratorArchivist::archive_current_decorators_at_with_owners(address, &storage, &RaycastWorld, &DecoratorPhysicsColliders, source)`。原 `...with_resources` SystemParam入口委派**同一实现**；保留先桶快照、整桶 first、整桶 second的顺序与首快照不覆盖。维护入口已持有ResMut世界，现可安全只读借用同一owner，无额外 clone、假世界或省略碰撞载荷。非native-physics维持原cfg。

## 原证修复与验收边界

shared fresh C曾移除大批原可达块，不能当完整依据。本人独占 worker `m01-hints-stair-20261027` 中，将九个 ABI carrier显式化并按 PDB有限范围修复 body，排除八段其他函数已拥有的EH区；body从3072B变3190B，不能把3963B物理跨度都强吞进body，也没有发现需要另造的jumptable override。剩余两条 removed-unreachable是被错误库效果推导隐藏的dealloc，不靠它解释业务，仍以原ASM为准。

[修复job](m01-maintain-stair-graph-repair-job.json)、[成功结果](m01-maintain-stair-graph-repair-result.json)、[修后C](m01-maintain-stair-graph-repaired.c)、[正常保存重开结果](m01-maintain-stair-graph-reopen-result.json) 保留完整记录。重开后原码SHA、3190B body、九carrier签名及C文本逐字相同；worker已正常关闭。shared未修改。

[原分支探针](m01-maintain-stair-graph-original-predicate.py) SHA校验并按PE映像加载RNE，直接执行 `14080665c..686` 及完整原 `is_decorator_cull_allowed@140b2bd90`，没有候选业务作为oracle、没有 helper桥。输入为原邻接Vec长度和原布局empty allowed-set/allow_any。14组 degree0/1/2/3/7/8/1024 × allow_any覆盖原等于1与许可边界，结果见 [原JSON](m01-maintain-stair-graph-original-predicate.json) 与 [Rust夹具](m01-maintain-stair-graph-original-predicate-cases.rs)。这是有界整型分支证据，**不是原完整World/HashMap/图删除动态执行**；degree0在实际图用例中是孤点，原邻居循环不进入，不能声称它是实际degree0邻居。

命令：`CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline maintain_stair_graph_recovery_tests -- --test-threads=1`。最终 [O(1)版本日志](m01-maintain-stair-graph-tests-final-o1.log) 为 **7 passed / 0 failed / 457 filtered / 0.02s**，包括14组原输出所对应的当前真实图；初次已通过版本 [native-final](m01-maintain-stair-graph-tests-native-final.log) 也为7/7。功能用例涵盖失败先后、recording与许可交叉、degree2保留、endpoint、真实72B移出事件、None位置、无额外历史、原Arc/实际含local-center世界位姿归档、真实五个楼梯collider回收、structural-only保留和第二tick稳定性。

共享委派另跑已有 country-core测试：普通cfg [1/1](m01-maintain-stair-archive-delegation-tests.log)；native-physics下 [真实双碰撞归档/销毁/恢复1/1](m01-maintain-stair-archive-native-delegation-final.log)。早期日志保留：first为测试History泛型/可变借用编译错误；second/third为夹具漏算mesh local center；native委派首轮被其他owner的UiState旧测试编译快照阻断。修夹具或等待owner落盘后重跑，没有放宽生产断言或清缓存。

## slot14 18叶的原顺序与当前缺口

以下是独立读取当前函数体所得，不采用旧统计/覆盖率。机器可读完整 ctor单态、PDB跨度、源路径/当前行、三处内联wrapper绑定及缺口在 [order-and-gaps](m01-slot14-stairs-order-and-gaps.json)。原组 ctor `140286610`，PDB物理3578B，[ASM](m01-slot14-stairs-140286610.asm) 中配置数组0x70 stride与最终18项直接写面确定顺序；不是只看callee列表或调用时机。17个普通构造调用和一个inline FnSystem共同组成18成员 `chain_ignore_deferred`，末尾明确 `set_chained_with_config<IgnoreDeferred>`。

| slot | 原leaf / body VA | 当前实际业务情况 |
| --- | --- | --- |
| 0 | maintain_stair_graph `140806110` | 本轮恢复；不是删掉parent slot11的重复 |
| 1 | generate_playermade_stair_requests<false> `140276e80` | move owner本轮核验；本文不代认证 |
| 2 | interpolate_and_compare `1407cf500` | move owner本轮核验；本文不代认证 |
| 3 | invalidate_dirty_stair_artifacts `140b37180` | 当前仅2参视觉reset/remove；原8参碰撞/RT/requests/Commands不完整 |
| 4 | propagate_dirty_flags `1407d16b0` | raw Vec参数空体，非原ECS/事件链 |
| 5 | create_structural_colliders_and_initial_graph `1407d20c0` | 当前3参空体 |
| 6 | playermade_stairs_assemble<true> `140008c90` | 当前零参空体，原62374B及几何consumer未恢复 |
| 7 | arches_n_walls<true> `140496800` | 当前todo、参数面不全；preprocess_curve亦todo |
| 8 | pillar_proposals<true> 内联 `14017a4c0` | 当前3参todo；另有merge helper不能代替完整6资源consumer |
| 9 | flush_rapier_colliders `1408d8360` | 当前真实world.flush_all，本轮未再认证整个backend |
| 10 | calculate_distortion `1407d49d0` | 当前pair slice→空Vec，非原6组资源consumer |
| 11 | playermade_stairs_assemble<false> `140018040` | 同泛型零参空体，原62822B |
| 12 | brackets `1407834f0` | 当前todo；原托架walker/RNG/砖SSBO输出缺失 |
| 13 | arches_n_walls<false> 内联 `14049ca20` | 当前同泛型todo；不能用ArchWalker `14048ab50` 冒充系统入口 |
| 14 | pillar_proposals<false> 内联 `1401788b0` | 当前同todo函数未表达两specialization |
| 15 | pillars_visuals `14077d520` | 当前todo；图/地形/原柱stub发射缺失 |
| 16 | floating_bricks `14077a2f0` | 当前5参todo；原13组含四类事件/Local状态/stats/地形/ClearCanvas |
| 17 | add_wall_holes `1407d1f80` | 当前raw参数空体，非原两组真实资源consumer |

原链重复调用图维护：parent19 slot11为独立 `140285ee0` ctor，slot14第一成员又是它，不能为“去重”删除。slot14外层是普通tuple2，兄弟为 `1403d6ce0` 的 fullvisual/tuple4/copy链。当前有大量明确未实现，**不得整体挂接后宣称可生产运行**，应由中央分配必要依赖逐条恢复。

三条没有独立业务名procedure的leaf不猜地址：原FnSystem名字确认true/false，继而取ctor的R8 vtable、+0x50 run_unsafe和其中真实call_mut：`1401786e0 → 142960cc0 → 140178310 → 14017a4c0`；`140178510 → 142960c08 → 1401781c0 → 1401788b0`；`14049c660 → 1429dfe08 → 140494f50 → 14049ca20`。原 [wrapper calls](m01-slot14-stairs-wrapper-calls.json)、[body信息](m01-slot14-stairs-inline-bodies.json)、[leaf ctors](m01-slot14-stairs-leaf-ctors-native.json)、[组构造器](m01-slot14-stairs-native.json) 都已保留。旧源码称`140178a70`只是false wrapper内部块，不能单独充当原入口。

## 本轮经验与交付

- PDB `void()`占位与旧注释参数数都不足，完整FunctionSystem单态+caller的物理ABI才可绑定真实资源。
- FnItem ZST可能完全内联到wrapper；用实际FnSystem名字和vtable追run/call_mut才能区分同签名true/false，不用ICF callee别名或旧正文中的内部地址替代身份。
- 生命周期保持原集合来源和先后：lane0集合与“所有不存在边”不同；结构图订阅回收与当前主动删图也不同。
- 原位置Option None只初始化判别时，inactive字节不能解释成真实业务零值；安全宿主初始化不等于原始位型证据。
- 归档需实际Rapier世界位姿（含mesh local center），不能仅核调用方external位姿。

源函数/测试窄块、共享API快照、原证、最终日志/二进制哈希见 [verification manifest](m01-maintain-stair-graph-verification-2026-10-27.json)。无OS输入、GPU声明、Git或STATUS变更。
