# 楼梯三缓冲原锁序的安全 canonical 写入接口合同

范围仅现 SparseInstanceBuffer，TG-f4e8ea3c0995e850c0 在 live NULL 后按用户完整恢复与 root/lead 本轮明确三buffer必要API授权正常claim，owner core-stairs-sparse-20261021。source唯一候选 writer为tree_commands；不改生产/Cargo或复刻依赖。不是另造稀疏buffer、allocation registry或业务私有替身。

## 原调用与实际类型

已完整原证：reset_edge140aa8a60/2322B、MagicSupports.clear140aa8210/1039B、remove_edge140aa8620/1087B、dropState785B/dropMagic210B。归属TGad966旧work/native-pack及delegated-stair-consumer-20261021/native-pack。Duck THREE_SSBO_OWNERS.json进一步锁main true/false实际调用：B1 BrickTransformSsbo、B2 SparkleSsbo、B3 WoodenPlankSsbo；fetch顺序B1→B3→B2，reset输入顺序B1/B2/B3。

原reset先取得state Brick allocation锁并clear（8b87..8c1b），仍持该锁时内嵌Magic.clear(B1,B2)（8c20..8c31），之后取得state WoodenPlank allocation锁并clear（8c36..8ce0），最后组合返回两个writer和state的4个字段借用。Magic.clear自己先锁Brick再锁Sparkle，两者均clear后才清本地Vec长度/计数/flag，最后释放两个guard。不能提前释放/重拿state Brick guard，也不能同时制造两个同buffer &mut、用新buffer规避或用通用同T两个buffer替代三个真实owner。

## 最窄现架构 API

1. `pub fn clear_and_lock_allocation<'alloc>(&mut self, allocation: &'alloc Allocation) -> MutexGuard<'alloc, Vec<ChunkId>>`：锁allocation，只短借buffer；按现canonical清理副作用执行后返回仍持有的同guard。guard生命周期只绑定allocation，不绑定buffer，不复制Vec、不重新lock。
2. `pub fn writer_for_locked<'a>(&'a mut self, chunks: MutexGuard<'a, Vec<ChunkId>>) -> SparseInstanceBufferWriter<'a,T>`：消费已有guard，把同一buffer各字段的安全借用放入当前writer；不重新lock/clear，不分配，不改chunks。现writer_for先lock再委派此构造，避免第二套字段拼装。
3. 私有 `clear_parts(current_chunk:&mut u32, byte_map:&mut[u8], active:&[AtomicUsize], allocation_chunks:&mut Vec<ChunkId>, free_chunks:&RwLock<Vec<ChunkId>>)`：直接从现Writer::clear移动原顺序业务，既有Writer::clear和新clear_and_lock都调用它。首步checked_sub置sentinel；依allocation原顺序byte_map[id]=0、清active；再取得free write锁并逆序drain chunks入free。dirty/data不触碰。panic先后/已发生副作用/保Vec allocation与现算法完全一致，无复制clear算法。

clear_and_lock的局部cursor只用于调用共享clear；真实state writer在返回时通过writer_for_locked重建cursor=byte_map.len()-1。原clear已将其设为同sentinel；Magic.clear只改共享buffer各chunk计数/active/free，不改变buffer长度或state已空chunk Vec，因此该cursor值没有被Magic影响。两walker分配表的guard不重lock；同一allocation误重lock仍原mutex约束，不新增默认成功/绕锁路径。

该API全部安全Rust。guard可继续存在而buffer借用结束，因此reset合法执行：stateBrickGuard=brick.clear_and_lock(stateAlloc)；Magic在同brick另一allocation及sparkle上完成原锁序清理；woodGuard=wood.clear_and_lock(woodAlloc)；按原次序drain实体/清元数据；最终以这两个guard组两个不同buffer writer。没有同时活跃的同buffer &mut引用，也没有unsafe alias。原core字段寿命/实际library owner始终同一份。

## 验证计划

同真实SparseInstanceBuffer及两不同allocation测试：先有非空chunk，持state guard清空后同buffer再清magic allocation，二guard均不可重锁；先释放magic guard时state仍锁住；消费state guard进writer并push验证同free栈、哨兵领新chunk和真实data/active/dirty。另一测试独立验证现Writer::clear与clear_and_lock两路径对于多chunk/free顺序/dirty保留一致（不是新算法oracle，只确认共享改造没有分支差异）。普通CPU TestDevice只验证buffer业务，不计GPU验收。测试由root执行；本合同不声称visual全schema已恢复或宿主已mount。
