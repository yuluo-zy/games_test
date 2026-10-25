# WallHoleStorage 业务生命周期恢复

新增 `reconstruction/building/wall_holes.rs`，接口为 `new_in_progress → add → finalize → clone_and_reopen/add/finalize`，以及真实外层 `advance_frame → 重新收集 → finalize`。原 Vec、HashMap、排序、分配器及 Bevy/CRT不重写；游戏 raw-record bit hash schema、sum/xor组合、缓存策略、几何/跨端复制与排序键恢复。

## 三版归属

原私有源 `crates/country-core/src/resources/walls/wall_holes.rs`：A checksum `6833be...39f969`；B/C均 `02a699...f3fdc`。B/C同源文件仍是游戏业务，不因共有而排除；同源也不直接证明所有caller/布局/调度一致。

| 游戏入口 | A | B | C |
|---|---|---|---|
| new_in_progress | 140628f90 | 140f86de0 | 140c4d920 |
| clone_and_reopen | 140629120 | 140f86f50 | 140c4d990 |
| finalize | 140629480 | 140f87300 | 140c4dd30 |
| InProgressHoleStorage::add | 140629a00 | 140f87860 | 140c4e2b0 |

`storage-three-version-comparison.json` 保存各原 binary SHA、PDB私有源校验和、入口调用与指令。B静态Ghidra出口来自既有analysis copy；原码执行以 tracking B RNE SHA `f507a6...b7d03`为准。

## 原记录与状态

- Raw record stride20：center.x/y f32@0/4，dimensions.x/y f32@8/12，HoleType byte@16。17..19是未确认padding，Rust不制造或比较其未定义字节。HoleType语义由门窗/屋顶producer另行确认，当前保留原byte。
- Finalized record stride20：Aabb2 [minX,minY,maxX,maxY] f32@0..16，HoleType byte@16，HoleOrigin byte@17。origin0是主记录，origin1是移位跨端副本；18..19 padding不纳入输出证明。
- B storage大小96：tag0 WIP/tag1 Finalized。WIP raw Vec@8、cached finalized Vec@20，参数length@38/closed@3c；Finalized cached Vec@8、参数length@20/closed@24、raw Vec@28。这些只用于oracle读写字段；Rust使用typed enum/Vec，不复制其header布局。
- 尾64..79是current(sum,xor)，80..95是previous(sum,xor)，**不是两个Aabb**。add的WIP子对象+38 wrapping-add record hash、+40 xor record hash。这两种聚合不依赖收集顺序，但重复记录仍计入sum，不意味着去重。

## 业务规则

`add` (`B140f87860`) 原私有 `utils::calculate_hash` wrapper (`14145b910`) 对tuple([center bits;2],[dims bits;2],HoleType)调用std固定DefaultHasher。原字节输入顺序为u64长度2、center两个u32 bit、u64长度2、dimensions两个u32 bit、HoleType discriminant u64。Rust逐字段喂给 `std::collections::hash_map::DefaultHasher`，没有恢复SipHash轮函数。Signed zero和NaN payload参与指纹。

`finalize`：

1. 已Finalized再调用触发原panic；Rust外层返回明确error。
2. current fingerprint==previous时直接转Finalized，复用cached geometry，**旧wall_length/closed参数也保留**。不能因为调用给了新参数就刷新；不能把“raw数组相同”替换成几何Aabb相同。
3. 否则对每raw record调用原Aabb2::from_center_dims。Dimensions向量sqrt必须ordered>=0；没有另行猜positive-width检查。
4. 主record origin0。closed且minX ordered<0时，加wall_length复制一条origin1；否则仅当maxX ordered>wall_length时减length复制origin1。最多一条复制；横跨两端的大洞优先左端分支，不进行任意次数wrap、不split、不clamp。
5. 采用std stable sort，游戏比较键是 **Aabb.maxX (+8)** 的partial_cmp：相同key（包括signed zeros）保留顺序；unordered比较触发原unwrap panic。没有merge或去重；不能以minX排序代替。
6. finalize保持current/previous两个fingerprint原值；**它不做previous=current**。

原 `140f873d6 SETNZ [RBP+56]` / `140f876bb MOVZX EAX,[RBP+56]` 返回fingerprint变化bool；Rust finalize的`bool rebuilt`同时对应此实际返回值，oracle也强比较AL。

`clone_and_reopen` (`B140f86f50`) 从两种phase都复制raw与cached finalized数组，参数和两个fingerprint照旧，仅将phase改WIP。它不会清空上一批raw输入，也不是下一帧的完整转换。

真正外层 `WallHoles::clear_for_next_frame` (`B140f867c0`)：已Finalized的row先把geometry/参数拷到PrevWallHoles，再把storage变WIP，raw清空，cached geometry/参数保留，previous=current_old，current归零。未完成WIP row清空raw/cache和两个fingerprint，但参数字段仍保持。Rust `advance_frame()->Option<PreviousWallHoles>`表达已恢复row行为；原global map查找/retain与事件/ECScaller由root另接，不重写map内部。

## 原码验证

`verify_wall_holes.py` / `wall-holes-probe`：B 768例通过，包含256原record hash、256完整finalize（含大于20条原std stable-sort路径）、128clone/reopen/add/finalize、128原frame-row clear/重收集/finalize。比较两份fingerprint、缓存参数、所有已确认f32 bit、type/origin和有序输出。原geometry、cache判断、hash字段schema与frame转换都执行实际原RNE指令。

依赖桥只提供标准Vec增长/分配/释放、CRT memory和Windows栈probe；原std排序实际执行，Rust侧直接std排序/DefaultHasher。Frame oracle在已解析storage/PrevWallHoles row边界进入 `140f86aa7..140f86c8f`，没有宣称原hash lookup/global retain或原scheduler也被验证。原phase错误在Rust适配层转换成Result，未宣称复现原进程panic机制。

这一模块交付可执行storage生命周期，并由root接门窗/屋顶开洞生产者和publish_public_wall_state实际消费者。原hole production schema、hole padding迭代/墙几何消费者、完整global资源/调度、A/C完整业务链验收仍分别跟踪。记录数与测试数不计算完整建筑/游戏完成率。最终workspace稳定后重新执行验证绑定源码hash，不能只改报告hash。
