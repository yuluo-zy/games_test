# DescriptorSetState 整体分配策略修正

已修改当前 `reconstruction/rhapsody-rebuild/src/raw/descriptor_set.rs`，没有改 manifest、shader、executor 或 managed 文件。原证据以 SYNTHESIS 第十一节、R4 复核与 A5 保留的 `batch05-0x140be5a40.c` 为准；本次没有打开新数据库、重反射库存或新增独立 crate。

原 `0x140be5a40` 的空闲栈保存的是 **descriptor pool 句柄**。栈顶池持续分配不同 descriptor set；共享来源返回一批池时整池 reset，然后继续 LIFO。遇 `OUT_OF_POOL_MEMORY` 或 `FRAGMENTED_POOL`，先移出耗尽池，将 `(pool, Arc<source>)` 交调用方保留，再循环挑选其它池或创建新池。它不是写入内容缓存，也不复用仍在录制中的 set。

当前实现据此移除 `BTreeMap<Vec<u64>, Arc<Set>>` 及内容 key。同内容请求始终产生独立集合；底层 Vulkan 调用保持可运行。瞬时池不启用逐集合 FREE_DESCRIPTOR_SET，也不在集合析构时逐 set free；共享来源返还后才整池 reset。持久集合继续原有显式 free 路径，未扩大本次修改范围。

`allocate_descriptor_set(writes) -> Arc<VulkanDescriptorSet>` 保持兼容。新增 `allocate_descriptor_set_with_retired(writes, &mut Vec<RetiredDescriptorPool>)` 直接表达原 caller 回收向量：容量失败之前已移出的池在后续创建/分配错误时仍交给调用方。所有成功集合和回收向量共同持有池租约，**最后一个 set/command/caller owner 释放以后**才把池归还来源。提交者必须持有这些 owner 到 GPU 完成；分配器不制造 wait_idle 来掩盖生命周期问题。仍活跃的池不 reset。

两类容量错误循环重试；其它 Vulkan 错误返回 Err，保留当前池。原侧其它错误是 fatal/panic；本工程用 Result 表达，原 unwind 未恢复。原 caller 的 managed 布局尚未迁入，因此 lease/RetiredDescriptorPool 是明确的候选适配接口，不能称完整原 ABI。底层池/布局析构既有 wait_idle、安全写入校验与资源 owner 保留也仍是候选边界。

`runtime_descriptor_count` 的参数已明确命名为 `max_descriptor_set_update_after_bind_sampled_images`，规则仍为 `min(saturating_sub(limit,32),8192)`。调用方 shader 的真实物理设备 property 选择由 root 独占修正。compact 原入口 `0x1418c2150` 的标签域仍为 0..5，未扩成另一单态化的 0..11；本次不按跳表相邻地址推断其它 shader 类型。

受影响验证通过：4 项真实分配循环共用代码的 mock Vulkan API 测试，检查两错误重试、调用方副作用、最后 owner 延迟 reset、整批返还 LIFO和非容量错误；既有 shader_production 4 项回归、locked/offline all-targets 编译与 strict clippy 通过。日志、源码与证据 SHA256 见 `validation.json`。这些是当前模块测试和已有静态原合同对照，**没有新增原作动态 oracle**；真实 GPU 示例由 root 修改旧缓存假设后统一运行。

root 接线要求：原同内容 `Arc::ptr_eq(set, again)` 断言改为不相等；提交所有者持有返回 Arc<Set>，如要显式对应原耗尽池记录则同时持有 `Vec<RetiredDescriptorPool>` 到完成。旧 `cached_set_count` 仅兼容返回 0，新的 `free_pool_count` 观察已归还共享来源的池，不能据它代替 GPU completion。
