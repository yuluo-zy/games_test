# 补充 B：冻结计时与计数语义

RecordScope::drop 进入后首先读取已启用 start 的 elapsed 固定 total_ns，再取/恢复 TLS、构造固定格式数组、获取 stderr 锁及输出。并行 logger 锁等待不计入新 pass total_ns。原 GPU timestamp 与 PGO elapsed 顺序保留，输出在原 elapsed 写入后。此修复来自独审；草稿 elapsed 原在 writeln 参数里会误计 stderr 锁等待，草稿不得用于归因。

旧 SubmitTrace 输出格式/实现（除提取同一开关）保持，owner-transfer 的 phases_ns 中 retain_resources 在原 callee 返回后、任何 stderr I/O 前固定，归因用此字段；旧 total_ns 仍可能含输出锁等待，不用它推断 transfer 真墙钟。新 record 字段均 inclusive；同 worker 嵌套 phase 不加总，不将 parallel pass 总和当 frame wall。counter/update 观测只覆盖当前 callback scope 发生的 mutation；Drop/submit/frame完成等 scope 外 counter 事件不在该行，不能据0值称这些阶段无成本。

RequirementsCalls / CompleteGroupCalls / DescriptorReserveCalls / FinishCalls / WithPushCalls 是真实函数进入次数（含失败）；RequirementResources 是惰性输入实际访问数，RequirementUnique 是成功准入的唯一key数。GroupAttempts/CurrentPageReuse/PageSearches/InsufficientRetries 是真实选择/尝试，PagesCreated仅成功物理页数。四类 Writes 与 NativeBindCalls/NativePushCalls 在实际 Vulkan void 调用返回后计数。BindingLeaseInputs是输入数，PassLeaseChecks是逐lease函数调用（含sampler/private-upload快速角色）；CoverageAccesses/CoverageSemantics仅计 covers_iter 实际惰性访问，不额外全扫。RetainSuccesses仅成功保留次数；LocalCacheMisses包含随后准入失败的 miss。

新增四个纯测试可强制私有 scope 启用并禁I/O，不修改进程环境缓存，覆盖off/nesting/false+panic/parallel线程归属与双局部 tally。生产只有 RHAPSODY_SUBMIT_TRACE=1 既有开关，首次 var_os 初始化可能分配（原机制）；关闭稳态没有新增 Instant、分配、RefCell或每descriptor原子/锁。RecordScope、RecordSpan、RecordTally均 !Send；启用计数饱和，scope id达到上限后停用新scope观测不环绕。私有scope遵守生产两入口栈式析构，不新增可公开乱序Drop协议。

探针本身会影响启用时的调度、短锁持有（固定TLS统计复制）、CPU时钟成本；需先用它定位量级，再用关闭探针的同输入优化构建测真实 wall。不会仅凭微小 inclusive ns 比例删严格验证或生命周期保证。
