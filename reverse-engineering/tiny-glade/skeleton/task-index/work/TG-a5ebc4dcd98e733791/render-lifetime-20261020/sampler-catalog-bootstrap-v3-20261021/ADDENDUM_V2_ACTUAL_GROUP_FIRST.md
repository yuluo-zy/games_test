# 采样器目录 V2：实际组优先、额外缓存尽力准入

本补充替代 V1 中「先将目录缺失加入实际组需求」和「register OOM 拒绝 entry」两项；V1 因预算兼容阻断不得整合。其余 App 归属、原 native13 仅证明静态启动目录、真实 (name, ArcSampler) key、同页 guard、epoch、所有权及原测试断言约束继续有效。此为现分页工程适配，不称原版分页恢复。

1. 实际 shader group 完全按原需求顺序执行：prepare 当前 sampler、原 FG/local/parameter 准入、原 sampler/FG/resources 写入、parameters.finish。这里任何真实失败仍沿原错误返回，不先分配目录 extras，也不重试实际组，不重放实际资源/FG，不再推进 DynamicConstants cursor。

2. finish 成功且仍持相同 per-page guard 后，才尝试可选目录 extras。目录快照只来自本 App 已安装管线真实反射；页 stamp 精确绑定不可变目录 Arc。目录项与实际准备项按 (name, ArcSampler) 去重；已在缓存的项只借用。完整目录加实际组唯一 sampler 总数若大于此 layout 真实 sampler capacity，跳过 extras。extras 只申请尚缺 sampler 槽，不影响已成功实际组的参数索引和私有块。外 App / 外 Device 的可选目录 owner 不登记到此页；实际组请求此 owner 仍保原 InvalidUsage。

3. extras 先准备自身 metadata 和为其 commit 预留 cache 容量，仍不发布。只对 extras 的 Vec 准入失败、GroupPreparationError::InsufficientSlots、或真实 sampler lease 分配产生 OutOfHostMemory / OutOfDeviceMemory / Exhausted 作安全回退；成功过的本次 extras 立刻 Drop，旧 cache/stamp 和实际 Prepared 保留。require_slots 的 Error 分支（含 Unsupported / Vulkan / DeviceLost / InvalidUsage）原样传播；这里已先验证布局容量，不推测 Unsupported 可以吞。sampler materialize 的其他错误也原样传播。真实错误传播时局部 Prepared、未提交实际和 extra owners 按既有错误取消路径 Drop，不提前提交/改写 push。

4. extras 全成功之后才把新项合入已预留 Pending，并允许完整目录 stamp 发布。任何部分失败不发布半目录或半 stamp。原 FG publication.commit 与 sampler.commit 仍在所有实际/可选可传播 fallible 操作之后；两者之间不再 fallible。目录已有 lease 的 clone 在取消后回到旧 owner，不释放在途命令需要的 owner。cache stamp 还需当前 epoch 相等且非 u64::MAX；invalidate 清同名所有版本，锁外 Drop。

5. 管线 entry 的原 fallible 准入和 table.try_reserve 后，目录登记仅作为可选优化。构造新不可变目录 Vec 的 metadata 失败不能使原可安装 entry 失败。失败时禁用本 App 后续目录 bootstrap，清其当前目录 Arc；已有实际 cache/在途 owners 不清，原每组路径保留。禁用是无分配标记，不在后续 entry 假称遗漏项已登记/全目录完整。所有旧目录 Arc 在 manager 锁外 Drop。已准备的旧目录快照至多完成自身真实 subset，不影响新 disabled 状态；后续 prepare 不再取得目录。可用 test-only 本实例一次故障注入覆盖 std Vec try_reserve 失败分支，生产不添加故障入口/预算策略。

6. 不依赖 Device 全目录，不对 loading/gameplay App 合池，不新建全局池、不扩大容量/GC预算，不改变原30阈值/采样点/绘制业务。完整目录在真实每页首次组后尽力补齐，因此可能首次多写少量 descriptors，但不再由后续不同 shader 才逐步增长。动态目录新增或 custom 版本变化仍允许合理真实分配，不承诺任意业务变化 live 恒定。

验证：新增私有真实 sampler descriptor 测试覆盖两页一次补齐、容量1回退、同名不同 owner 与 epoch 失效、实际组已成功后 extras 部分 lease OOM 自动释放且原 cache/actual owner 保留、metadata 登记失败禁用且实际组仍成功、明确准入不足回退与真实错误传播。helper 可以注入同一生产 extras admission closure，测试不冒称其手工 serial admission 等于真实并行 guard；actual Snapshot 完整组顺序由源码审和 root 既有 ABI/bridge/world GPU 闭包验证。作者不 Cargo/GPU/生产/Git，root 统一执行。
