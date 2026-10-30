# M01 楼梯失效清理：原8参、分离脏集合与真实回收

日期：2026-10-27。范围仅原slot14第3叶 `invalidate_dirty_stair_artifacts` 及必要生命周期helper；没有接管宿主注册或其余17叶，没有进行OS输入/GPU验收。

## 原版身份与证据

B骨架 `reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/resources/stairs/invalidate.rs:1266` 保存了完整FunctionSystem参数：RaycastWorld、StairColliderRegistry、StairsState、StairsStructuralGraph、StairsVisualState、RtWorld、PlayermadeStairsAssemblyRequests、Commands，均为真实资源身份，不能用原先2参投影替代。B原源文件摘要为 `2df6a65f318e64092045d693466db99099201f3b89202adb459da3285a03de85`。

身份锁定：RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。本地分析用EXE摘要 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323` 不冒充未修改RNE。完整区间/代码摘要见 [原函数身份](m01-invalidate-stair-original-identities.json)。

| 原函数 | VA | PDB物理跨度 | Ghidra正常body | 本轮用途 |
| --- | --- | ---: | ---: | --- |
| invalidate_dirty_stair_artifacts | 140b37180 | 4290B | 3455B | 完整主控制流与8参 |
| StairsVisualState::remove_edge | 140aa8620 | 1087B | 567B | 真实value生命周期、实体释放 |
| StairsVisualState::reset_edge | 140aa8a60 | 2322B | 2099B | 排除错误调用关系，未在本轮宣称完整恢复 |
| dirty请求fold | 141477ff0 | 460B | 438B | 原指令有界差分 |

对应 `m01-invalidate-stair-*-full.asm` 是LLVM直接读取原PE区间，包含EH物理区间。不能拿body较短判定原实现只有相应字节，也不能将EH反编译片段补成新业务。

## 恢复的业务合同

主系统按原顺序处理两个不同集合，源码位于 `crates/country-core/src/resources/stairs/invalidate.rs`：

1. **A是图脏边**：drain真实StairsState dirty_edges。随后清视觉全局dirty。
2. **B是请求脏边**：遍历真实请求，写回 `request.dirty = old_dirty || A.contains(segment)`，将true请求的边收集到B。
3. **只按B** 删除结构segment_counts，retain结构采样collider并逐个释放实际物理实例。原 `140b37471..474 → 140b375a5` 在B空时跳过整个阶段，避免静止帧扫描所有结构collider和误写资源ChangedTick。
4. **只按A** 移除三路edge collider。图中仍存在的边只请求 `force_rebuild`，保留真实分配；图中消失的边释放自己的visual，并分别给两端当前邻居的已存在visual请求重建。缺少visual不能新建假条目。
5. 按原 `{tag=1,payload=(min_node<<32)|max_node}` 移除对应RT mesh。命中才drop DirtyCow/Arc，随后置topology dirty；未命中不能制造拓扑变化。
6. 只移除previous.sub_segments和previous.arch_wall_supports，不删sampled_points、请求Vec、adjusted_edges或alignments。

因此 A\\B 在此函数中保留结构采样/counts；B\\A 保留三路edge collider、RT和visual。维护图的上游函数承担它自己的删除责任。合并A/B后全清会破坏原保留规则。

**更正旧注释：原invalidate没有调用reset_edge。** 原存在边分支是force_rebuild，不能以“重置得更彻底”为由释放仍存活的实例。原先2参实现及其调用reset_edge的注释不能作为原证。

## 真实owner与窄API

`visual.rs` 的已证实owner恢复为真实 `Vec<Entity>`、两路MeshAtlasAllocation与内嵌MagicSupports（自身另含两路分配）。`remove_edge_with_commands` 在命中时取出实体，drop其余owned state使分配进入原buffer waste队列，再置dirty并提交实体despawn。缺边不置dirty。原RT mesh由invalidate独立删除，不重复塞入visual helper。

旧 `remove_edge` 保留为无实体条目的存储兼容入口，有实体时拒绝静默漏掉despawn。生产失效路径使用带Commands的新入口。必要访问面只有 `get_mut`、`allocations_mut` 与MagicSupports `allocations`，均借用现有owner，没有默认GPUhost或第二套实例资源。

`RtWorld::remove_mesh_if_present(&RtMeshHandle) -> bool` 只做一次map.remove，内部先drop命中项，再置topology dirty，匹配 `140b37a1a..a57`。其它caller的原remove_mesh合同保持不变。

完整8参系统依赖真实native physics资源，因此本函数和reexport均明确受 `native-physics` feature约束。无该feature的构建不提供假2参降级系统。

## 差分与多帧验收

原请求fold探针执行 `141477ff0` 完整入口至自然ret；原PE按section映射，并断言RNE SHA。只有hash_one和输出集合insert被定义为库边界，显式构造原请求stride0x150、segment+128/+12c、dirty+148和HashSet数据。全部dirty判断、membership比较、写回与迭代来自原机器码，候选实现没有充当oracle。[探针](m01-invalidate-stair-request-predicate-native.py)、[18条原结果](m01-invalidate-stair-request-predicate-native.json)。

18条原ABI样本包括empty/matching/different集合、old bool两值、ordered/reversed/stub节点。当前安全 `StairSegment::new` 与原构造函数一致规范化端点，不能安全构造6条原始reversed布局。因此保留全部18条原证，仅将**12条合法规范输入**生成Rust夹具并经过真实World请求→结构count删除路径比较；没有悄悄规范化oracle或把18条都记为候选通过。

最终命令均使用 `CARGO_INCREMENTAL=0`、现有target、offline：

| 验收 | 结果 | 日志 |
| --- | --- | --- |
| `cargo test -p country-core --lib --offline --features native-physics resources::stairs::invalidate -- --test-threads=1` | **6/6**，0.11s | `m01-invalidate-stair-tests-final-empty-gate-v2.log` |
| 对应visual既有helper定向测试 | **7/7** | `m01-invalidate-stair-visual-helper-tests.log` |
| `cargo check -p country-core --offline` | 通过 | `m01-invalidate-stair-default-cfg-check.log` |

6项真实World覆盖：原12条有效请求夹具；A/B不同保留域；三路真实edge collider和真实结构采样collider释放；RT真实Arc弱引用生存期；删边邻居force与未命中不创建；真实实体despawn；四路BrickTransformSsbo实际稀疏分配Drop→waste→take_out_waste回收；多帧保留；空B时Registry/RaycastWorld/StructuralGraph/RtWorld ChangedTick不变。用TestDevice创建实际buffer做CPU生命周期断言，没有宣称真实GPU绘制。

首次稀疏回收测试错误地要求byte_map全0，失败原因是原末尾满哨兵32必须保留。随后改成与分配前完整byte_map逐字节一致，业务代码未为测试降低条件。最后一轮曾被并行owner的ui_autoclutter测试签名中间态阻断，owner修复后最终编译及6项均通过；该中间态不计成本函数业务缺陷。

## 边界与下一依赖

本轮恢复失效清理及必要真实释放，不代表整个18叶装配组、生产App注册或整游戏已完成。StairsEdgeVisual原value的其它纯数据缓存仍需对应assemble producer恢复；没有凭缺失字段构造raw占位数组。reset_edge旧兼容路径的完整实体/缓存重建仍未在本轮恢复，且未被重新挂入invalidate。各slot14其余叶的具体缺口仍见 `m01-slot14-stairs-order-and-gaps.json`。

后续经验：先区分“图脏”和“请求脏”两个数据域，再恢复释放；用原分支核验空集合性能合同；实体、Arc、稀疏allocation分别验收；不能从旧注释或函数名猜调用；完整物理跨度与正常CFG分别保留；原ABI可构造域与安全Rust业务可构造域必须明确分开记录。
