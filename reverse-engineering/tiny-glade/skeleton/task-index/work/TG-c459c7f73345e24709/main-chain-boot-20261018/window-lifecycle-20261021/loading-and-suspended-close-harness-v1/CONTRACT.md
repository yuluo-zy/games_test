# 真实加载与暂停关闭窗口验收 V1

root 已明确委派此主循环必要 harness continuation；保留 TG-c459 原 owner，不写生产、Cargo、Git 或运行游戏/GPU。源 before 为已运行 13 world frame 的 `720141e082be2af65b2b7037f63ec64d62b8915779ab9b240f7ec9ba866753e8`。

## 三条独立验收

默认入口仍为原三帧加载 fixture，源区块不改。`TG_VALIDATE_GAMEPLAY_WSI=1` 的真实 run_game_thread→双 App→OS 枚举→GameLoop 路径、12 world frame/native present 和 save owner 门保留。新 `TG_VALIDATE_WINDOW_LIFECYCLE` 只允许两个精确值，未知值失败，优先于 gameplay 模式：

- `suspended_close`：首个真实 loading 图完成后，必须仍有 gameplay preparation owner；依次注入真实 dispatcher 的 Suspended、CloseRequested，不发送 Resumed。CloseRequested 返回时立即断言 target.exiting 且 preparation 已移除。该返回成功依赖现 shut_down 完成真实 cancel/join/save/screenshot/GPU；LoopExiting 再核保存 owner 无残留，不能充当成功起点。
- `loading_close`：同样在 loading 图完成、preparation pending 时注入普通 CloseRequested，保现队列→下一实际 frame→ShutdownGameCmd 真 App 消费者。下一 redraw 返回且 target.exiting 时记录已完成清理；退出必须存在真正 ShutdownGame marker、无 preparation/save owner。本模式可能在下一 frame 原先的 poll 顺序完成 handoff；不重排它，也不将此模式称为世界渲染验收。

各模式独立 PASS 名称，不以 loading_close 代替原 gameplay WSI 结果。首 loading 来自 ProductionGraphKind::Loading 与真实 completed_frames 增量，而非合成 World；图观察不是屏幕扫描完成证明。

## 超时与取消

只有生命周期模式创建一个测试 watchdog 线程。60 秒未正常退出，EventLoopProxy 只发送一次真实 UserEvent，即使当前 ControlFlow::Wait/没有 redraw 也能唤醒主线程。该事件立即产生失败并保存首错误，随后 target.exit 触发真实 LoopExiting 的现 shutdown；有错误时禁止打印 PASS。正常 run 返回后 guard 发 stop 并 join watchdog，不留 detached 定时线程。

它不将关闭强制转换成 Resumed、不调用 App.update、不制造 ShutdownGame、不禁系统，也不等待 idle 来伪造稳定性。GPU worker 可能本就需要真实完成后才能 join；watchdog 不能中断正在执行的 GPU/CPU 主线程函数，不能证明所有系统级 hang 有界。

## 证据与验证

使用现 `gameplay_preparation_pending()`、target.exiting()、真实 AutoSaveState/SaveThreadHandle，无生产公开 API 扩张。实际 join/清理效果来自返回 Result 的原链条，而不是仅 `take()` 的布尔值；任何 shutdown Err 都先记录失败。代码用 pinned winit0.29.15 当前公开 create_proxy/send_event/exiting API，已只读查源。

两个候选分别 rustfmt --emit stdout 语法解析 exit0；输出只写 /private/tmp，不复制 formatter 的文件名标题。无 Cargo/rustc/GPU/OS 运行，本报告不声明编译或运行通过。root 需要分别构建并保留二进制/源 SHA 后执行两个模式，同时复跑原 gameplay 模式。
