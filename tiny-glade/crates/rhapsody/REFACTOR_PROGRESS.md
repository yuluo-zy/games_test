# rhapsody 重构进度（2026-10-16）

本轮是 P1–P3 的生产路径接线，并修复图会话失败清理；目标架构仍有未完成项，不作为整个模块重构完成的声明。

## 已实现

- 设备创建为每个实际队列建立一条时间线。图执行、独立命令、setup 上传与交换链提交使用同一入口；提交成功后才递增已提交值，不以 CPU signal 填补失败值，不追加退役用空提交。不同会话共享设备完成点。
- 移除全局 `queue_lock`。提交、present、queue idle 按实际队列加锁；device idle 按 QueueId 稳定顺序锁住全部队列。acquire 只使用交换链锁；同一缓冲的宿主拷贝使用对象锁。
- `QueueConfig` 实际控制 `vkCreateDevice` 请求数量；默认仅一条通用队列，`Prefer` 回退为类别别名，`Require` 无法满足时拒绝。逻辑队列顺序与时间线槽位保持一致。通用族必须支持图形与计算，避免 Compute 别名落到无计算能力的族。
- `Retirement<Doomed>` 取代设备 epoch graveyard。旧资源尚未携带精确 last_use，因此保守等待 drop 时所有队列的已提交快照；这不是最终的逐资源依赖方案。
- 图执行器不再持有 `DeviceState`、复制 `SubmissionEngine` 状态或解析日志。移除 `attach_graphics_device`、`mirror_submission_state` 与无调用者的旧图提交入口；下游相应迁移。
- 移除 SequenceRuntime 无界字符串日志和 recorder 字符串统计；使用类型化命令序号与屏障计数。生命周期资源记录实际回收。
- 默认一个普通图批次使用一个命令缓冲，初始/内部/导出屏障共用录制。显式并行模式保留。
- 图编译/会话公共错误使用统一 Error，设备资源创建及录制调用保留原始设备错误类别。声明校验覆盖数量、资源索引与导出类型；mip 校验使用完整 u32，允许恰好覆盖剩余 mip，拒绝空范围。
- 录制回调失败或 panic：取消未提交命令、关闭序号缺口、释放本帧引用并执行退役回调；已有批次成功提交时先等待这些真实完成点，再回收。GPU 已产生的副作用不会撤销。普通失败后可以继续下一帧。
- 修复 Windows 全特性 Clippy 检查涉及的 Aftermath 原始指针入口契约，调用方以 unsafe 明确承担回调数据有效期；调整 DXGI 冗余返回。

## 尚未完成

- P1：其他 raw/managed 公共 API 的 vk::Result、内部错误及调用方可触发 panic 仍需逐个迁移。
- P2：VulkanDevice 与 GraphicsDevice 资源包装仍并存；新 DeviceCaps/ExecutionPolicy、代际与预算入口尚未落地。
- P3：SequenceRuntime、旧托管销毁桶与兼容 SubmissionEngine 仍存在；图已不依赖后者。帧 × worker × 队列族整池复位、Upload/ReadbackManager 及精确 last_use 尚未实现。
- P4：类型化资源身份、命名 temporal、独立编译计划/帧实例、缓存、瞬态内存计划、bindless、dynamic rendering 尚未实施。
- P5：主图与交换链呈现的 FrameLoop、AcquiredImageLease/PresentTicket 完成域尚未闭合。部分提交后的等待异常与呈现失败仍需完整事务协议；当前交换链仍有保守 queue idle 路径。
- P6：配置辅助队列不等于图内多队列调度；跨族独占资源仍要求显式协议，当前后端拒绝缺失协议的导入。
- P7：全模块历史注释/重复实现清理与完整性能 A/B 尚未完成。

## 验证

环境：Windows、rustc 1.97.1。命令从 `tiny-glade/` 执行。

| 检查 | 结果 |
| --- | --- |
| 三个渲染包 `cargo test --lib --tests --all-features --no-fail-fast` | 330 passed，0 failed，1 ignored（手动性能用例） |
| 三个渲染包 `cargo test --lib --tests --no-fail-fast` | 327 passed，0 failed，1 ignored |
| 三个渲染包 `cargo clippy --no-deps --lib --tests [--all-features] -- -D warnings` | 默认与全特性均通过 |
| `cargo build -p tiny-glade -p tg-text --features tg-text/gpu --all-targets` | 通过；保留 country-core/roof 原有警告 |
| 1000 帧实际 Vulkan 稳定性 | 在飞稳态资源记录 7→7、命令 2→2；排空后资源与生命周期桶均为 0 |
| 失败与共享状态 | 回调 false/panic、部分批次已提交后失败、两个会话共享时间线、设备丢失传播、拒绝外部 signal 设备时间线均通过 |
| 配置队列 | 实际创建与规划/类别别名匹配；本设备没有辅助队列，跨族 GPU 读回明确跳过。CPU 队列族规划单测通过 |

三个渲染包指 `rhapsody`、`shader-compiler`、`shader-pipeline`。最终检查限定 `--lib --tests`：工作期间新增的 `examples/window_lab.rs` 引用了尚未齐备的 `window_lab/win32.rs` / `app.rs`，不属于本次修改，保留该示例并单独排除。此前全目标检查曾通过，但不能据此声称当前新增示例也通过。

单独执行 `cargo test -p rhapsody --all-features --test p0_perf_baseline -- --ignored --nocapture`：300 帧、丢弃 10 帧预热、6 pass，debug mean 0.628 ms / p50 0.327 ms / p95 1.942 ms；每帧 1 个命令缓冲、5 条屏障命令 / 11 个转换、1 次 vkQueueSubmit、1 个提交批次。该次测量未控制系统负载，也未在同机重跑旧版本，**不作为 CPU 耗时优于旧实现的证据**。

本次接口迁移同步修改 `country-core/src/render/execution_host.rs` 与 `tiny-glade/tests/core_raster_draw.rs`。宿主改用真实辅助队列句柄，队列族池上界改为最大族号 + 1；另外对 `country-core/src/game_profile.rs` 做了局部编译修复：Debug 输出 packed 字段前先按值拷贝，保留布局与序列化行为。其他已有工作区改动未回退。

原始运行日志位于工作区 `tiny-glade/rhapsody-refactor-*.log`；这些是本机运行记录，不属于架构实现。
