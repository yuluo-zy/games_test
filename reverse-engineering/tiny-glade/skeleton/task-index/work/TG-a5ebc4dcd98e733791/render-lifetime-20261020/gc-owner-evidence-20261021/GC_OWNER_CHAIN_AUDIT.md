# GC、常量 owner 与真实回收观察（2026-10-21）

## 结论与边界

当前不修改 GC 预算。真实共享常量版 30 帧完成后，原有 `wait_submits_done` 已清理完成对象，额外诊断为 pending 0→0、destroyed 0、allocated_bytes 964383592→964383592、submitted=completed=1696、idle_waits 2→2。这不支持用扩大回收预算解释当前内存起伏。帧中 pending 仍从 frame12 的143升到 frame29 的264；这些是所有 native 对象的个数，既包含内存对象也包含无分配字节的句柄，不能等价为设备内存泄漏。最终0也不能代替更长运行验证。

本轮已修复的主要 owner 接缝是每次 `with_compute` 独建 DynamicConstants；现在复用同 App、同 Device 的 RasterShared 既有 Arc owner。原真实30帧中常量 pages/creations 始终4/4，reuse继续递增，不存在日志支持的“两页常量缓冲持续新增”。本报告不声称全部场景或无限长稳通过。

## 原版 B 证据

直接对身份绑定的 B RNE+匹配 PDB 收集5个有界机器码包，入口在 native-manifest.json，native-pack/report.json status=ok、target_executed=false。RNE SHA256=f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB SHA256=2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911，RSDS GUID/age匹配。函数范围由现有 PDB procedure size提供；pack保守标记 boundary_verified=false，不把包处理成功算游戏行为验证。

| 原入口 | 本次核实的最窄语义 |
|---|---|
| Queue::reclaim VA 0x140bf0cc0 / 6290字节 | 0x140bf0f4b读取queue+0x80完成序号；0x140bf0fa0读取FIFO长度；0x140bf0fd5比较队头record+0x58序号，未完成即退出；0x140bf17d2调用SequentialDisposal。 |
| SequentialDisposal VA 0x1418c7ae0 / 1959字节 | 窗口中记录乱序完成并推进已连续完成的前缀；0x1418c7e13与0x1418c8008调用原销毁桶函数0x140bf92f0。不能把它解释为只允许按序完成。 |
| DynamicConstants::allocate VA 0x140c02740 / 685字节 | 0x140c02759拒绝>0x40000请求；按至少16及设备/请求对齐；0x140c027ad检查当前页+请求<=0x40000，否则0x140c027c4调用allocate_buffer；0x140c028a4对真实buffer owner加引用后返chunk。 |
| DynamicConstants::new / allocate_buffer | 本次保留463/502字节原包，作为owner与页大小证据补充，未将全部失败语义扩张为已复现。 |

旧持久反编译 `evidence/render-recovery/producer-closure/r3-managed-disposal/0x140bf0cc0.c`、`0x1418c7ae0.c` 与此次机器码的关键条件吻合。原 `0x140c04e20` 提交路径在取出 dropped waste 后调用reclaim；原已完成桶循环没有观察到“每轮最多64个对象”限制。当前工程的多queue SyncSet与normal 256扫描/64销毁、pressure 4096/1024属于重构后的工程策略，不能凭原版桶结构要求复制旧架构。旧 `reconstruction/rhapsody-rebuild` 目录的AUTO_THRESHOLD64是历史候选，不作为原证。

## 当前工程路径

1. Buffer/Image等最后owner Drop得到真实 last_use SyncSet；无提交依赖直接销毁，有依赖进入 Device::retire→Retirement::try_enqueue，即使cached completed已到也保持维护预算策略。入列存储OOM才沿已有真实等待fallback处理。
2. Retirement按queue和value排序；collect_limited从已完成前缀弹出，再检查全部SyncSet。其余queue未完成则转到下一queue；ready回调在retirement FIFO锁外执行。
3. Device::try_collect_garbage先真实poll完成值、更新driver budget，再按当前预算扫描/销毁。try_collect_all_completed同样真实poll，但只清理snapshot已完成项，无新增idle等待。
4. VulkanGraphSession::finish_frame先observe completion、recycle command owners、做一次bounded GC，再回收/停放图资源、处理retire callbacks和frame imports；后半段owner释放可在早期GC后入列。
5. VulkanGraphSession::wait_submits_done等待每个pending frame，处理result cache/imports后已经调用try_collect_all_completed。不能把测试后另一次drain当作证明此现有入口缺失。
6. ProductionRenderer::collect处理各App session、完成的present ticket和swapchain后再bounded GC；shutdown分别等各App、present和swapchain，保App隔离。

GPU已完成不意味着资源必须销毁。AttachmentCache、page pool、graph resource pool、FrameGlobals、当帧/命令所有者可合法持有仍能复用的对象。常量Chunk必须同时满足CPU不再引用与所有真实queue完成，才能安全重用；不因“poll完成”忽略仍在CPU声明或录制的range。

## 实际观察与数据解释

- prefix-alone单帧 `/private/tmp/tg-prefix-world-v2-20261021.log`：53→0、释放34648064字节，submitted=completed1426，idle2→2。诊断在warmup局部bridge命名handoff完成及Drop、截图finish/shutdown之后，不是紧邻最初wait入口；对象具体种类未采集，不能直接称64预算不够或常量池积压。
- shared30 `/private/tmp/tg-shared-world-thirty-20261021.log`：原live drift断言已过，首个失败是旧pool peak统计；本次修正另外提交，未改阈值。pool旧实现相加各页不同时刻的峰值；不能据这个和推断同时活跃增长。
- 同次constant pool pages=4、creations=4（frame1..29），reuse到58；物理分配报告中262144字节条目count10在frame05和29均相同。
- 截图输入480×270×4=518400字节。screenshot_capture原 `remaining=3` 在其递减到0后的下一次finish才读取/Drop，capture9的buffer可保留到约frame12，capture19到约frame22。原clean谓词仅排除capture与其后一帧，不等于所有截图owner都已释放。
- 原settled clean index为12..18、21..28。显示首frame12=922.4MiB，13..18=921.9，21/22=922.4，23..28=921.9；这些四舍五入值没有支持“first921.9→peak922.4”的结论。原精确memory断言被peak失败挡住，仍须真正执行，不能离线宣布通过。

## 必要时才添加的诊断合同

| 字段/采样边界 | 能区分什么 |
|---|---|
| 同一边界 before/after pending、scanned、destroyed、real submitted/completed、idle_waits | 已完成回收吞吐与真实GPU未完成；不把cached completion当刚poll结果。 |
| 入列/销毁按Doomed kind计数，Buffer/Image deferred range bytes独列 | 无内存句柄积压与实际device range字节；pending总数不能替代。 |
| resource_range_bytes、retiring/inflight/live、application_allocated_bytes、reserved_unused_bytes、CPU metadata字节分开 | 子分配保留块、活owner范围、退休范围和CPU记录，避免不同统计域混比。 |
| App/桥接session边界、命令recycle前后、图回收后、截图finish后、显式drain前后 | 早GC后新owner释放、跨App合法池与真正缺少drain。 |
| constant pages/creations/reuses与不可变Chunk/真实allqueue完成 | 池预热vs反复新owner；不以强清池掩盖。 |

保持现有日志已能回答的问题不再添探针。只有后续长稳显示completed pending或retiring bytes持续增加，才建议补按kind的有界采样；不能预先改64预算或每帧all-completed drain。

## 独立错误路径

只读发现 `sync/retire.rs` 在跨queue移动insert失败时未恢复原queue value，可能把下一queue的数值与旧queue completion比较。root已另行委派 `retirement-move-rollback-v1-20261021` 窄修复与注入测试；属于现工程OOM安全问题，不是当前pending0或截图内存变化的已证根因。

## 性能复现边界

已集成incremental prefix保持原635 physical cells / 3536 patches / 4438 uses / 1478 private reads，真实native_state_prepare由11188508625ns降到121863083ns，rg_submit由11307128667ns到231499875ns，原world像素mean154.4/colors41882一致。它证明真实CPU前缀重复重放瓶颈被移除，不证明其余record/submit或全部画面完成。后续应以相同输入分段测量剩余CPU热点，保持原range/producer/revision/预算验证。
