# Sequence runtime 收尾与共享入口合同

日期：2026-10-12。目标 SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

本包恢复的是共享执行入口所需的 sequence 消费接口；`sequence-runtime-r8` 为本批已有代码的迁移来源，不作为新的独立产品入口。旧 R6A/R3/R7 包未修改。源码重用 hash、原码 oracle 输入 hash 与 trace 文件 hash 保存于 `native-validation.json`；编译门禁保存于 `validation.json`。

## 接通内容

- `begin_after_backend(DeviceCounters, Command, Queue, backend_result)` 直接消费原 R6A 从 1 初始化的 per-queue counter。没有把真实 sequence 减 1，也没有固定 sequence7。
- `Command::record_command()` 和 `caller_snapshot()` 转交同一条 sequence、handle、state；`import_record_tracker()` 将 R7 录制消费得到的资源 tracker 导入相同生命周期。
- `accept_submission(SubmissionReceipt, Vec<PendingCommand>)` 接收 queue 后端产生的实际 per-CB timeline threshold。`Sequence(u64)` 和 `Timeline(u64)` 分域；threshold 不能由 sequence 猜造。`accept_queue_success()` 仅是明确标注的成功结果物化适配器。
- `observe_timeline()` 保存查询值，`reclaim_ready()` 按 threshold 弹出并验证 pending。完成序列 gap 不延后 CB 回池；仅延后资源 bucket drain。
- 全局 bucket 的 key 保持 plain `u64`。不同队列同序号的 duplicate insert 先替换 bucket 再报告错误；未改成 `(queue, sequence)`。

共享入口 owner 负责将 graph/compiled/record/queue 后端接入本合同。本包测试中的 recorder fixture 是局部候选接口测试，不计作原 graph consumer 接通；整链成果由共享入口报告。

## 原侧验证与边界

联合 oracle 共 908 组新输入，零差异。它们属于选定原入口的 CPU 合成差分，不是原作完整 submit→资源销毁运行。旧 2840 原案例未重计。

| 原入口 | 验证的私有行为 | 908 组中到达入口的组数 |
|---|---|---:|
| `0x140bed230` | begin 状态、backend 结果后 counter 生产、command 写入与 bucket wrapper 调用 | 908 |
| `0x1418c4260` | bucket insertion wrapper 与重复断言；底层 map 容器注入 | 876 |
| `0x140bd6a70` | handle track 的状态与最大 sequence 更新 | 584 |
| `0x140be24b0` | last reference drop 与 waste 入队 | 556 |
| `0x140bf0b90` | timeline query 结果保存、已完成 timeline 排除 | 864 |
| `0x140bf0cc0` | pending pop、状态/队列检查、sequence disposal、pool 入库 | 864 |
| `0x1418c7ae0` | sequence prefix/ready-gap 与连续 bucket drain 顺序 | 848 |

输入包括 792 组单队列排列/逐步或批量完成/早或晚 drop/0–2 resource，56 组 backend、begin state、counter bounds、pending state、queue mismatch、pool-family bounds 错误，48 组重复 ready/prefix，12 组跨队列 bucket collision。异常副作用按原指令保存：pending 先 pop 后验证；pool-family 错误发生在 sequence disposal 后。原 panic 被测试适配器转换为 `Failure`；Rust unwinding 行为未验证。

外部边界：Vulkan reset/begin/get-counter 结果、固定容量标准容器的 map insert/remove 与 VecDeque resize、锁无争用。私有缺口：`0x140c16a00` guard 固定 false，`0x1418c1380` telemetry sink 被抑制，`0x140c81c70`、`0x140c81e10`、`0x140bdfcf0` 被抑制（callee 完整身份与副作用待核实），`0x140bdef60` 拦截已登记但本组未到达，`0x140bf92f0` 资源 destructor 仅捕获 dispose 事件。不得把这些私有 cutpoint 归入公共 Vulkan 边界。

queue success 的 tag3/threshold/pending 队列本组手工物化；未执行完整 `queue_submit`。CPU retire 事件、waste→bucket attachment 由测试适配器组合；资源销毁本体未执行。trace 保存所有实际到达 PC，hook 到达次数另列，避免将未到达路径宣称已验证。

## 未完成与使用限制

- 本 oracle 没有真实 Vulkan/GPU、线程争用或原 caller reachability 验证；synthetic auxiliary/collision 输入只限定局部语义。
- resource 模型的 id/generation 创建与 `cpu_retire` 是候选后端合同，不证明原设备分配或完整所有权。
- 没有恢复完整 begin backend reset/begin 所有参数、queue-submit producer、资源 destructor 或 profiling/debug 路径。共享入口必须保留 typed gap 或明确 external/private capture 分类。
- CPU 原指令不能推出源代码 atomic ordering；候选代码采用已恢复边界的 ordering，不证明多线程调度等价。
- M12 仍 `verified_partial`。本包不按案例数量提升整体完成百分比。

复现：`python evidence/render-recovery/parallel-batch-06/sequence-runtime/verify.py`；编译门禁：同目录 `finalize_validation.py`。未打开 Ghidra 数据库或启动原游戏。
