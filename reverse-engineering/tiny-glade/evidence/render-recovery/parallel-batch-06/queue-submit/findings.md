# Queue submission 模块交接

目标：原构建 SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。当前交付是共享执行链可消费的 CPU 状态投影，不是整个 queue 模块或 Vulkan 后端完成。遵循 `WORKFLOW_RESET.md` / `AGENT_EXECUTION_GUIDE.md`，停止扩大原 640 输入集合与另建独立 crate。

## 1. 模块及必要 caller 的集中证据

复用原模块类 C、PDB 工作图与原码反汇编，索引见 `module-evidence-index.json`。`0x140c04e20.c` 为整段 queue_submit，`0x140bebdf0.c` 为 executor submit_pending_cbs；create/begin、device create、ensure_active_cb 和各 caller 索引一起登记。类 C 只作字段/控制流线索，实际差分从未经修改的 `0x140c04e20` 第一条原指令开始，不注入预造 SubmitInfo、timeline 数组或栈 frame。

原序列化输入为设备/queue 状态和尚未提交的 88 字节 command、可选旧 batch、外部 wait/signal、可选 active primary command。queue prefix 自行生成数组，执行其私有 flush、range 注册、command 迁移及 receipt store。此前 barrier oracle 的 begin_label/end_label 跳过已清除；debug 扩展未启用时仍执行这两个原 helper。实际 Vulkan、分配、公开依赖及诊断/错误转换边界见 `consumer-interface.json`。

## 2. 字段、枚举、所有权和错误顺序

- QueueId 的正常语义为 Primary(tag 0) / Auxiliary(tag 1, index 为辅助 family 内 ordinal)，不是 family index；原生无效 tag 位型不属于恢复 API 支持范围。
- Command 的 sequence 是 begin 时产生的 per-queue counter 值；queue timeline 是独立 device 全局分配器。queue 消费真实 sequence，不重算或平移它。
- 批次顺序为 **prior → flushed primary command → current**。旧 batch 及 flush 同时存在时，先前候选顺序错位；16 个已有输入揭示了这处差异，已修复。
- 全部 command 数决定预注册 timeline range 和 next_timeline。原作先改变这些状态，再逐个校验 command state 2 和 queue identity；后续失败不撤销之前的 range 或已移动 command。
- 每个 command 按遍历顺序获得一个 timeline 值，追加该 queue 的 timeline semaphore/value，state 改为 3 并进入 pending deque。每个 batch 保留自身 wait/stage/value、signal/value 顺序。不同 batch 不合并。
- executor caller 先 take commands/wait/retirement payload，再检查/结束最后 state 1 的 command；结束成功改 state 2 后才委派 queue。此顺序属于调用者合同，不能在 queue 内预检查全部 command 后再原子提交。
- backend 非零后才进入 handle_error；转换结果 tag 11 表示可继续，其余已测试 tag 9 停止。pending command 已经 state 3；queue 不回滚，也不产生成功 receipt。转换成功后完成 post-submit，再返回 `{QueueId, last_timeline}`。
- Rust `Outcome::panic` 表示原 panic 入口处的观察停止，候选用正常返回传出状态。未模拟 native unwind/destructor 或 88 字节 command 的所有 opaque managed 字段，不能把这等同于完整失败所有权恢复。

## 3. 整体 Rust 接口与真实 consumer 接线

迁移来源：`reconstruction/render/work-packages/queue-submit-r8/src/lib.rs`，接口 `DeviceState::queue_submit(QueueId, SubmissionInput, backend) -> Outcome`，消费者由整合 owner 放在 `reconstruction/render/execution.rs::ExecutionChain::submit_recorded`。

输入是已录制并结束的真实 command、wait/signal 及 queue identity；输出是 typed PreparedSubmit、pending 状态和 receipt。`PreparedSubmit::with_vk` 在 callback 存续期内用锁定原版 ash 创建 VkSubmitInfo/pNext。Command handle/sequence/timeline 在 pending 上保留关联。shared source 不调用 queue fixture()。当前 owner 的 graph→compiled→record 上游接线和统一类型属于根流程验收，此报告不替代它。

移除的固定注入：queue 原侧预造 submit 数组与尾段 frame；候选共享 consumer 不再输入人工 command sequence 或 VkSubmitInfo。尚未去除的私有边界：handle_error 返回 tag、诊断 ringbuffer/backtrace、active-device-command end-error 的 typed 消费缺口。当前公共输入 `flushed_command` 只投影 end 成功路径；真实 device active-command 生产者接线需 owner 显式约束或返回 typed gap。

## 4. 模块入口集中验证与依赖

`native-validation.json` 保存既有 640 输入的完整原 CPU 入口结果，598 正常返回，623 捕获 backend 参数，42 在原 panic 入口停止；零差异。比较 wait/stage/value、signal/value、command handle、receipt、next_timeline、pending timeline/command、发布 oldest_pending 以及停止时锁字段和事件顺序。正常/辅助 queue、旧 batch、flush 成功、外部 wait/signal、backend 错误、部分 command-state/queue-identity 失败均在该集合中。

依赖校验发现上一个未收尾 Cargo.lock 自动选了较新 libloading/cfg-if；已改回原锁 libloading 0.8.3、cfg-if 1.0.0、windows-targets 0.52.6 及匹配 checksum/source。原 CPU 观察不变；重新构建候选并重放同一输入核对，避免重复跑未变原侧。全部 source/harness/reuse input hash 与 locked/offline release check、2 个现有 glue/失败状态测试、strict clippy、fmt 日志见 `validation.json`。这些不是共享入口或 GPU 验收。

## 5. 当前阻塞

共享失败接线必须保留 backend 错误前已经入队的 state 3 command 与 timeline range，不能因没有 receipt 就当作“从未提交”或允许重用。已向 owner 报告 `submit_recorded` 的早返回及 lifecycle 状态反馈缺口；完整错误恢复需 handle_error/设备失效语义和 unwind ownership 证据。真实锁竞争、native end-error、existing pending 输入、多队列真实可达性、完整 graph ABI、game pass、live Vulkan/GPU 仍未验收。

本次接手续用中断任务。没有可靠分阶段计时记录，因此不补造取证/实现/验证/整合耗时；所有已有结果与旧验收范围保留。
