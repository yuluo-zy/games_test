# B 基础库并行交付

3个agent已按原函数与目录恢复8库到 shared-foundation workspace。C按用户指示后置；8库均为候选部分实现，真实游戏生产消费链与完整原入口未全部验收。

| 库 | 本轮功能 |
|---|---|
| [tg-iced](../../reconstruction/shared-foundation/crates/tg-iced/src/) | B手写对话框选项/结果与文本入口；真实原iced/winit/cosmic-text后端可运行，窗口接线为重建适配。 |
| [tg-integrity-report](../../reconstruction/shared-foundation/crates/tg-integrity-report/src/) | B报告三列表保持顺序，all-empty归一，单文件删除与状态/建议/UI联接。 |
| [crash-reporter-gui](../../reconstruction/shared-foundation/crates/crash-reporter-gui/src/) | B诊断优先级、前8帧匹配、复制提示、一次callback/thread与Tick完成联接，内嵌完整性报告。 |
| [tg-gpu-selector](../../reconstruction/shared-foundation/crates/tg-gpu-selector/src/) | B游戏私有设备选择/driver建议/graphics错误策略及Vulkan allowlist六项workaround状态，未重建枚举/驱动库。 |
| [feedback-reporter](../../reconstruction/shared-foundation/crates/feedback-reporter/src/) | B等待/替换/601计数、选定文件ZIP、benchmark、multipart/HTTP失败规则与Bevy Update事件插件。 |
| [theme-fetcher](../../reconstruction/shared-foundation/crates/theme-fetcher/src/) | B主题HTTP/ETag/date/cache-control、backoff/half-open活动、CAS/ZIP校验与库内producer。 |
| [theme-server-proto](../../reconstruction/shared-foundation/crates/theme-server-proto/src/) | B协议ActiveEvent(start/end i64,cid String)、EventsResponse(events)字段契约；serde派生不算手写算法。 |
| [geforcenow](../../reconstruction/shared-foundation/crates/geforcenow/src/) | B固定DLL/三导出全部加载后initialize(0)，成功query/shutdown/unload；失败不吞为false。 |

共享 --offline --locked 测试32项、严格all-targets Clippy通过；GFN本地DLL另1项覆盖14组。原UI叶131案例、GFN入口18例（注入SDK边界）、反馈等待切片48例均保留限定范围，不合并为完整原功能完成率。

原iced/winit/cosmic-text fork已直接复用；414项公共依赖version/source/checksum与原B锁匹配，零差异。此审计只覆盖本8库闭包，已有tg-text可选GPU路径的gpu-allocator0.27候选在范围外。协议serde派生、标准库、控件/HTTP/ZIP/hash实现不算游戏手写算法。

五个真实窗口已渲染、截屏、自动退出；父视觉复核通过。隐藏窗口全背景、Basic shaping缺字形的首次截图均拒收；最终Advanced shaping回退修复字形。窗口调度/字体glue属于重建适配，精确原像素未验收。

- [整合与源码SHA](integration/summary.json)
- [原依赖核对](integration/dependency-validation.json)
- [父视觉复核](integration/visual-review.json)
- [UI交付](ui/REPORT.md)
- [主题交付](themes/REPORT.md)
- [反馈/GFN交接](feedback-cloud/HANDOFF.md)

剩余重点：tiny-glade主题plugin消费、country-core/UI反馈producer、rhapsody真实设备/driver_support与device创建消费、启动完整性与panic回调payload。完整原run/report/ZIP oracle、原GUI默认值/调度/样式和A/C适配继续保留；未请求真实报告服务或加载真实GFN SDK。专属worker保存独立B副本并停止。
