# 退休对象跨队列入列失败回滚合同

本候选属于现有 Rust 工程的错误路径修复，不声称原版业务恢复，也不作为本轮内存波动或 completed-drain 的根因。root 明确委派在原 rawbinding work 中提交候选；不修改生产源、claim、GC 预算或正常回收策略。

`Entry.value` 仅可解释为当前所属 FIFO 那条 queue 的等待值。`after` 保存所有真实依赖，`pending_queue` 查找仍未完成的下一条依赖。跨队列移动先赋下一 queue 的值；成功后正常入目标 FIFO。目标 `insert` 存储扩容失败时，必须将 `value` 恢复成移出原 queue 时的值，再放回原 FIFO 队头并结束该轮。此时原值已经被同一 completed snapshot 确认到达，下一轮可再次迁移，不能用下一条 queue 的数值与原 queue completion 比较。

成功路径调用、原有 queue 轮转、scanned/destroyed、len、预算上界、真实依赖与 ready 执行次序保持。失败路径不新增分配或等待，复用源 FIFO 刚刚 pop 后释放的容量。它仍保持原有失败时 push_front 的策略；不扩展修复到当前已有并发重入行为。

测试专用字段仅在 cfg(test) 下编译，用每实例原子值注入指定 queue 下一次 insert 的存储准入失败，进入与 try_reserve 失败相同的 Err(entry) 分支。生产构建无该字段或注入分支。主要回归先完成 source=5，保 target=100 未完成，注入移动失败，再以同一 source completed=5 重试，要求后继 source-ready 回收且 target 仍等待。随后完成所有队列，验证所有对象恰好一次。初次 try_enqueue 同时验证原 item 返还及 len 不变。

作者不运行 Cargo/GPU；root 需执行原 sync::retire::tests 与新增回归。
