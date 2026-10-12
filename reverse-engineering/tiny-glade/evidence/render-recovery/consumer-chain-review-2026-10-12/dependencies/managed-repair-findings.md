# 托管资源消费链修正

日期：2026-10-12；对应输入及检查见 `managed-repair.json`。本轮修改用户已授权的 managed/disposal 模块与功能链测试，不修改 allocator、Vulkan raw 对象创建/析构、lifecycle 调度器或 manifest。

1. **实际 consumer**：`Managed::track/Clone/Drop` 消费同一 `ControlBlock`，tracker 是单个 `AtomicU64`，CPU refs 为 `AtomicU32`，generation 保存在控制块内。`ResourceManager::get` 返回的拥有句柄也增加同一计数。七类 `ManagedResources` 共用一个带 tag/store_id 的退役 Vec 与 u64 命令桶；单个 typed store 的消费入口也能取走共享来源。
2. **移除机制差异**：不再维护每资源 `BTreeMap<domain,seq>`，非零 domain 返回 `CrossDomainMappingUnknown`。不再等 GPU 完成才递增 slot generation；CPU refs 归零后入共享来源，`dispose_control_kind` 转移 raw 拥有权、检查 refs==0、在 inner 内 wrapping+1 generation，然后归还槽位。旧 raw 对象仍留在命令桶，可与复用槽位的新对象同时存活。未使用的影子 `EventOracle` 已从新工程移除。
3. **根入口合同**：begin 成功后 `begin_command_bucket(seq)`；queue 尾部 `take_out_dropped_waste_checked()`，返回的是 CPU dispose 记录数；根现有 `lifecycle.prefix(queue)` 在通过完成/gap 检查后逐序号调用 `drain_completed_sequence(seq)`，返回 raw 销毁数。后一个参数严格是 command sequence，不能传 queue timeline/queue+0x80。缺 begin 桶返回 `MissingBucket` 并保留退役记录；没有第二个 counter 或 ready/gap 调度器。
4. **边界与结果**：功能测试使用真实 DeviceCounters 与实际句柄控制块，检查 clone/drop、CPU 提前复用、延迟 raw 析构、缺私有 producer 重试、late drop、非法 domain、失败 wait。单队列旧 `record_submission/complete` 保留为明确候选兼容路径；新主图不使用该 receipt map。完整共享 Vulkan consumer/GPU 验收由 root 负责。旧 908 原入口组合报告未被重计为新工程整链原侧验收，本轮没有新增局部 oracle。
5. **当前阻塞**：跨队列序号与单 tracker 的可达性及原 caller 调度未闭合；安全 Arc/Mutex/Vec 容器、raw Arc 延迟 drop 仍不能宣称等于原 slab/Treiber 栈、9 类句柄 Vec + 0x48 字节记录及固定销毁顺序；tag3/case9 仍未知。其他 raw Arc 拥有者会延后实际析构，因此桶排出数量不是 vkDestroy API 调用数量。

## 对旧 writer 未知结论的限定纠正

SYNTHESIS §11 将 queue+0x80 的写者列为未知。但已保存 `parallel-batch-06/sequence-runtime/update-pending.asm.txt` 包含入口 `0x140bf0b90`：`0x140bf0b99` 将 param1 放到 rsi；间接查询后 `0x140bf0bbd` 从输出位置取值到 rdi；`0x140bf0c0f` 执行 `mov [rsi+0x80],rdi`。本轮只读同构建 PE 直接复核这条指令 bytes=`48 89 be 80 00 00 00`，并保存目标 hash。

因此，该限定函数是一个已证实的字段写者；真实 caller 可达性、帧调度与完整 GPU 完成接线仍不能由这一条写入推出。代码继续把完成消费者输入与序号通知分开，没有猜测新写者或令 timeline 等于 sequence。

## 原锁依赖审计

新工程现有 registry/git 外部包按 name/version/source/checksum（git 按 repository+最终 commit）与原锁核对，结果保存在 JSON。原 ash 为 `0.38.0+1.3.281`；ron 复用 PounceLight commit `2787d8c05250c4dd706dfb358a2e9f16969d2968`。

原 `gpu-allocator 0.26.0`、`vk-sync-fork 0.4.0` 的锁条目无 source/checksum，不能从版本号编造 fork 仓库/commit。当前专用 Vulkan 分配适配是明确候选边界，未复现其次分配/池策略。shader-compiler/shader-pipeline `0.1.0` 也为 local/path 来源待分类；shader-prepper `0.3.0-pre.3` 是已知 registry 依赖。候选 SPIR-V 公共解析复用 rspirv、反射库存作为数据源，不等同完整原工具链。缺失库不能因现在有可运行手写适配就计作原依赖恢复完成。
