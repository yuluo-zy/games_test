# WSI 消费链修复

依据 `producer-closure/SYNTHESIS.md` 第十一节、R4 复核、A6/N3 的整段类 C 证据，修改了 `reconstruction/rhapsody-rebuild/src/raw/swapchain.rs`、`src/raw/managed/swapchain.rs` 和 `examples/swapchain_gpu.rs`。未修改 manifest、descriptor、managed 核心、executor 或总 tracking。

## 已接通的实际消费者

`swapchain_gpu`：隐藏 Win32 窗口 → surface/设备/交换链 → 逐槽 acquire → Vulkan command 清屏与读回 → binary wait/signal submit → present → fence 完成 → 同槽复用。resize/vsync/遗弃租约使描述或状态变化，再由 raw/managed 的 `validate_swapchain_and_acquire_image` 创建替代链并重取图像。managed 例最后执行 store 的退役/销毁排出，不直接丢弃未处理的 managed 废弃记录。

raw 保留现有 acquire 参数签名以兼容调用者，但 Vulkan acquire 始终使用原固定 `3_000_000_000` ns。managed `acquire_next` 消费持有的实际 raw 资源 Arc，返回同一个 Vulkan 图像租约；不是独立 fixture 后端。resize/set_vsync 写入请求描述，不创建 swapchain、不调用 wait_idle。实际 extent/handle 在 validate 成功替换前保持原值；重建失败时旧链保持 stale，不能冒充有效图像。

## 修复的跨函数差异

| 问题 | 修复后行为 | 原侧证据 |
|---|---|---|
| acquire 使用调用者无限超时 | Vulkan 调用固定 3 秒 | `0x1418cb520` |
| acquire SUBOPTIMAL 标记 stale | 交付图像、保留 suboptimal 标志、不置 stale | `0x1418cb520` / R4 §3 |
| NOT_READY/TIMEOUT 与 OUT_OF_DATE | 都返回错误；只有 OUT_OF_DATE 触发 validate 重建 | `0x1418cb520`、`0x140bfd9d0` |
| present 非致命错误范围错误 | OUT_OF_DATE 及 **-1000255000** 非致命；SURFACE_LOST 继续为错误 | `0x1418cc1b0` / R4 对 A6 的更正 |
| resize/vsync 立即重建并等待设备 | 只登记描述；validate 才 retire→replacement→acquire | `0x140c0e7f0`、`0x140c0e8a0`、`0x140bfd720` |
| 每帧新建信号量/fence | 创建时逐槽分配，成功 acquire 才推进游标；同槽复用同一组句柄 | `0x1418cb520` 的 cursor、两组同步数组；0x38 槽成员不补造 |
| managed swapchain 仅 type alias | 增加 desc、resize、set_vsync、acquire_next、retire、validate 的实际 raw/store 消费 | A6 三层调用结构 |

原 cursor 变量和 Vulkan 返回的 image index 是独立变量；源码按**同步槽 cursor + 1**轮转，不用 image index 覆盖 cursor。

## 真实 GPU 与集中验证

UHD 630 上实际运行：320×240 clear/readback/present，3 个同步槽轮转后回到槽 0，acquire/render semaphore **及 submission fence**均保持相同句柄；resize 登记保持旧 handle，再 validate 为 384×256；vsync 登记、遗弃租约恢复、managed 新 key 重建均完成。所有读回像素为 `[0,255,0,255]`。窗口从未显示，结束后 DestroyWindow 成功。

2 项 WSI 状态政策测试检查 acquire SUBOPTIMAL 不失效、NOT_READY/TIMEOUT 返回错误，以及 present 的原非致命数值，不用实际驱动 SUCCESS 冒充异常分支验证。当前GPU实测没有产生 SUBOPTIMAL/OUT_OF_DATE；重建经过 candidate resize/stale 生产者触发的 OUT_OF_DATE 通道。

`validation.json` 保存当前源码、实际依赖消费者及原类 C 哈希、各命令日志；项目 `evidence/swapchain-functional-validation.json` 同步受影响 WSI 范围。之前报告备份为 `previous-swapchain-functional-validation.json`。共享接口整合曾导致 resources 模块缺失及回调签名不相容，owner 对齐后当前源码重跑 GPU 和政策测试；未复用改动前的成功日志。

## 明确保留的边界

- 原 0x38 槽成员、desc/error 枚举、raw version/store/managed ABI 未复刻；这些是安全 Rust 拥有权与语义 API。
- 原 acquire 返回两项同步数据的完整类型仍待恢复；当前 fence 是候选 GPU 提交完成保护。它不证明呈现消费已完成。
- 后端未启用 present-wait/maintenance 扩展。候选在**同槽再用或析构**用 `queue_wait_idle` 保护二进制 semaphore；既不每帧等整台设备，也不把此保守边界声称为原 PendingSubmission/销毁桶策略。resize、新链创建不等待设备。
- native `-1000255000` 还受 `managed+0xb9` 条件影响，其语义未知。当前候选将此非致命结果标 stale 以重建，不能宣称所有分支副作用逐字等价。
- native 其他 present 错误执行设备崩溃处理并 panic；候选仍返回 typed Error，没有伪造原 panic/error ABI。
- 此 GPU 提交使用现有 raw Vulkan submit/fence 后端，未接 country-core 的 present 图或完整原 queue timeline/disposal 闭包。
- 实际使用 primary 呈现队列。辅助队列、分离呈现族、全屏/失焦/驱动规避与 winit 分叉 trait 流程仍未验收；未把 0xc/0xf 作 Win32 枚举。

结论为“原静态证据约束的 WSI 候选功能链可运行”，不是原二进制 WSI 动态差分通过，更不是 rhapsody 完成。
