# 暂停期间关闭窗口：必要宿主闭包 V1

原 claim 继续由 `oschain-20261018` 持有，本子目录是 root 于 2026-10-21 明确委派的必要窗口闭包；不改 claim、共享数据库、生产文件或 Cargo。版本只限 B，A/B 文件 checksum 不同，不能外推 A。

## 可触发的反例

当前 `Event::Suspended` 设置 `suspended=true`；`AboutToWait` 进入 Wait 并停止请求 redraw；即使出现 redraw，分支仍不调用 `frame`。但 `WindowEvent::CloseRequested` 仅写 `pending_os_events`。唯一消费点在 `GameLoop::frame`，因此暂停后关闭没有正常终止路径，只能等额外 Resumed 或外部 LoopExiting。异步 preparation/save/GPU owner 均可能继续存在。

## 原证与适配边界

复用原 B `main_loop@0x1404d9580`，完整 asm 位于父目录 `main-loop.asm`。原 `0x1404dc857` 结束本批 OS 消费，`0x1404dc94b..971` 的零尺寸门仅跳过 App/egui/render，而外层控制路径继续。原 Windows 事件表不提供当前 macOS Suspended/Resumed 策略。本候选明确是现有主线程平台适配的终止修复，不能称原作 Suspended 算法。B RNE/PDB 身份来自只读 claim receipt；不新增原反汇编或跨版本推断。

## 最窄修改

仅 suspended CloseRequested 分支：调用已有真实 `dispatch_os_event(CloseRequested)`，随后现 `shut_down()` 完成 worker cancel/join、save stop/join、截图 hurry、GPU completion/shutdown，再 `target.exit()`。不运行暂停中的 App，不创建 ShutdownGame，不等待恢复。普通 CloseRequested 原队列和 frame 顺序不变。

`shut_down` 原错误顺序不变：先保 preparation 错误，再保 save/screenshot/GPU 的首错误；失败不会走本分支的 exit，交给现 runner 的错误清理。正常 LoopExiting 的 shutdown 是幂等复查，不能作为本候选验收的成功起点。

## 验收边界

配套真实 harness 应在首个 loading frame、确认 preparation 仍 pending 后注入 Suspended→CloseRequested，无 Resumed；CloseRequested 返回时必须已经移除并 join preparation、完成 cleanup 且 target.exiting。一次 UserEvent watchdog 的超时只允许失败并 cleanup。普通加载中关闭单独验证。源码候选不代表编译或 GPU/OS 运行通过。
