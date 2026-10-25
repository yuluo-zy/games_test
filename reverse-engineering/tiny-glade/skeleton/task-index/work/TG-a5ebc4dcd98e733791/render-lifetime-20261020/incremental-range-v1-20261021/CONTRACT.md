# 提交范围状态前缀增量准备合同 V1

## 真实证据与范围

2026-10-21 实际 M3 单帧原世界 probe 测得 native_state_prepare=11,188,508,625 ns（635 physical cells、3536 declared patches、1478 private upload reads、4438 uses），rg_submit=11.307 s；真实 vkQueueSubmit2 约25 ms，vkWaitSemaphores 约101 ms。日志：`/private/tmp/tg-submit-trace-world-20261021.log`。原 trace binary SHA256=d35609e6498e7d2c49a384cbf1d00d03994b5145f93eb1d3e027c23c0ba84b6b。

本候选仅优化既有 Rust 工程实现的重复前缀计算，不声称此实现一一对应原 B，也不改变原业务、shader、pass、绘制数量、资源用途或同步正确性要求。现有 timer 保留。仅候选，不由本 agent 修改生产、运行 Cargo 或 GPU。

## 原路径与等价关系

当前对同一 physical cell，每个 upload_read 在检查其与 declared/opaque 的重叠后，调用 prepare(全部已累计 patches)，再从暂存 cells 为该 read 生成 Access patches；末尾再次 prepare(全部 patches)。每个 prepare 都取消前次暂存、从 committed cells 复制、依次重放整个前缀。prepare 中间不排序、不合并；只有 publish 合并 committed cells。因此，对相同 committed 状态和相同顺序 patches，将前一次成功的 scratch 保留、仅应用后缀，得到的 RangeCell 顺序、state、PreparedCell writer/readers/release 标记与 dependencies 与完整重放相同。

## 最窄 API

1. RangeStateTable::prepare 保持公开签名与初始取消/复制行为，将原 patch loop 提取成私有 apply_prepared_patches；共用最终 cells.grow + prepared_queue 发布资格准入。
2. 新 crate 内 append_prepared(patches, queue, family)，仅允许在同 queue 成功 prepare 的 scratch 上调用；开始追加前撤销 prepared_queue，任何错误都无发布资格，保留原 committed cells；完整后缀及最终 cells.grow 成功后恢复资格。每一 patch 继续执行原 range、revision、ownership、producer/frontier、layout、overflow、split 与预算检查。禁止旁路、忽略错误、假完成。
3. SyncCell 新 crate 内 append_prepared_range_state，只锁现 native_state，不再初始化，不另加 gate；调用者继续持有全部 submission gates。现 prepare_range_state、publish、cancel 和 preview 合同不变。
4. prepare_submission_states 用 Option<usize> 记录已成功准备的 patch 前缀长度。第一次仍完整 prepare，后面只 append 后缀。**仍在下一 read 的 declared overlap 检查之后才追加上一个 read 的生成 patches**；末尾追加最后后缀，再 merge dependencies。因此错误先后保持：先下一 read overlap，再前缀 replay/append，再当前 producer 检查，再当前 patch scratch add；不提早应用会改变错误先后的尾补丁。
5. 无 upload 路径继续原 prepare_cell。成功末尾只从最终已准备 table 合并 dependencies，不再次重放。

## 事务、预算与锁

- committed cells 直到真实 vkQueueSubmit 成功点 publish 才改变；prepared guard 的既有 Drop 取消整个提交所有 cells。追加失败不得遗留可 publish 的前缀。
- 每次原 prepare 边界都保留 cells.grow(scratch.len, scratch.bytes, config, budget)，不得仅末尾准入；每个 split 保留原 scratch.grow、旧 backing 与新 backing 同时计费的峰值、max_cells、max_cpu_bytes、allocator budget 与 Error 原样。
- 每个前缀相同 scratch 顺序与容量增长历史，因此相同边界的容量需求相同。原重复 prepare 开头 scratch.grow(committed.len) 在此前成功前缀后必为 no-op；取消只清长度，不释放容量。新路径省去的只是此 no-op 与复制/重放，未省去实际预算准入。
- 重叠私有 reads 允许；每次从已有暂存 revision 构造当前 read 的 patch，继续所有 GPU producer 检查，不将私有 read 并为一个大 range。
- submission gates、native_state mutex、cpu_state_patches mutex 的取得顺序与作用域不改，不添加全局锁或新资源持有。

## 验证矩阵（root 唯一执行）

- 差分 oracle：旧 prepare 的完整前缀路径 vs 新 append 路径，每边界比较完整 PreparedCell（含 flags）、staged dependencies、容量及 committed state；最终 publish 后逐 cell 与同步点一致。涵盖 disjoint/overlapping buffer reads、host writer、GPU writer/reader、多队列依赖、UNKNOWN、image layout、release/acquire、revision exhaustion。
- 负例：stale revision、越界/空范围、queue/family/ownership 错误、release range 不匹配、missing/host/foreign queue prefix、split/cpu byte/allocator budget失败。每次失败 committed 不变，无 prepared_queue，guard drop/cancel可重试；相同原合法调用的首错误类与内容相同。
- 集成：既有 raw native_state_submission_tests 全部；新增真实 guard append失败→取消及成功→一次 publish 测试，不制造 GPU对象。
- 正确性与性能：root 实际 Cargo tests/build 后，同一原世界单帧内容、同 timer 字段和实际像素验收，确认 native_state_prepare 降耗；再禁用 timer 计时。现30帧 live/memory/peak断言原样，不把性能改善当长稳通过。
