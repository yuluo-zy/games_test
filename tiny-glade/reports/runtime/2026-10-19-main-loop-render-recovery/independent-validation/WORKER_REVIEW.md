# GameplayPreparation 取消与关闭独立审查

首审对象：`TG-c459c7f73345e24709/main-chain-boot-20261018/scene-startup-20261019/run_game.rs.patch`。before `bd4ab6360d8d03321d826561cf46b269c87e8afaec9ea2d1496c5876277a170b`；candidate `eba9fedb5e8f74f1a84b45a536bcd054e2fe9dab7f094e5564c855a78bd3c8b1`。此为当前宿主取消/所有权适配，未声称 B 原线程 API 等价。

当前生产丢弃 JoinHandle，只在 Drop 写 AtomicBool，不能证明后台 App/暖机结束先于 GPU teardown。候选保留 handle；update前、暖机前/后检查取消；poll 收结果/断连时 join；显式 cancel_and_join 和 Drop 共用结束路径。

独立核查：使用非有界 mpsc channel，join 不因等待发送队列被消费者排空而死锁。cancel_and_join 不持 session/device 锁；join 完成后才丢 pending 与 queued 成功 SubApp。只有私有 typed `GameplayPreparationCancelled` 被正常取消吞掉；producer/warmup 的原 anyhow 错误仍返回。completion 与 join 都先执行，`.and(joined)` 在两者失败时保留更早 producer 错误。

静态认可这一窄修方向，整关闭链仍待 01 候选证明：必须先取出 preparation、cancel_and_join，再执行 renderer.shutdown；前一步 Err 不能跳过后一步，原错误保留。暖机正在执行时取消必须等真实暖机返回，不能用取消标志声称 GPU 已停止。新增普通运行不增加每帧分配或锁；结束时 join 可能阻塞于真实工作完成，该成本不伪称零延迟。

已向作者提出必要回归补足：首候选中 `drop_joins_after_receiver_has_taken_the_world` 实际显式调用 cancel_and_join，未触发 Drop。增加真正 `drop(preparation)` 后 worker stopped 与 World 内 DropFlag 断言，分别覆盖 pending 与 channel 排队成功 World。poll 的 producer Err 分支可日志记录 join 的次级 panic，不能以该次级问题覆盖原失败。

尚未运行 Cargo/原进程/GPU；root 统一执行。此文不接受全部启动资产、源同步、完整加载或完整游戏关闭，仅记录这个候选的静态范围。

## 新版冻结结论

已核 candidate `5ce22c7f1d0878113f3e1cdf9e4cce879eba50008b571f435ec4fba0d2091d7c`：补入真正 Drop、pending/queued WorldDropFlag 与 worker stopped 断言，主错误与次级 join panic 分开报告。取消 kernel 为 `passed_scoped`。root 的 `validation/worker-shutdown-v2.json` 绑定同候选，使用真实缓存 Bevy App/World 直接抽取构建，四测试通过；范围明确不含 spawn/poll/OS/renderer。

01 patch `7bc0f87fa3e83059978abd6c3a4cce589c7d3c1c7706acac236b613ab952bace` 的 teardown 顺序已独立读到：take preparation、cancel_and_join、清光标、renderer.shutdown，前项失败仍执行后项，保留更早错误、日志次级错误；只有双成功写 done，Drop 可再次收尾。该顺序 `passed_scoped`。零尺寸窗口 CloseRequested 仍有独立主循环缺口，不包含在此通过结论内。
