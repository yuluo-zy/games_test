# 主题与 GUI 重构后二次复审

2026-10-17，重新审查 tg-theme、tg-iced 及 tg-gpu-selector 的适配。此前“发现问题均修复”的授权继续适用；本轮发现并修复以下确定缺陷，没有执行逆向、请求真实服务或启动窗口。

## 发现与修复

| 优先级 | 原触发条件与影响 | 当前修复位置 |
| --- | --- | --- |
| P2 | 当前主题到期时 worker 正在阻塞请求或预取，订阅仍返回旧 Active；同步 poll 后 spawn 还会把已知 Active 重置为 Pending，直到下一次 I/O 完成。 | `service/subscription.rs` 使用包含所有活动的不可变快照；订阅独立按边界重新选择，spawn 继承已有状态。 |
| P2 | Transport/get/reader panic 导致 worker 退出，但没有 stopped/通知，消费者持续等候最后状态。 | `service/subscription.rs` 的退出 guard 发布 Failed、标记停止并唤醒等待者，shutdown 保留 join 错误。 |
| P2 | 200 max-age=10 后，304 不带 Cache-Control，会回退默认10小时；Date 缺少 Age 修正导致缓存年龄被误算为时差，正文耗时又被加到新鲜期之后。 | `service.rs` 继承已接受策略、Date+Age 修正，刷新截止时间从响应头到达算并扣除 Age；正文读取会消耗新鲜期。失败退避仍从操作结束算。 |
| P2 | no-cache/no-store 与较大 max-age 同时出现时被忽略；no-store 仍保留条件请求标记。 | `service.rs` 使用最短1秒轮询，no-store 清除 ETag。活动内容作为应用运行状态使用，未宣称实现通用 HTTP 缓存。 |
| P2 | request_timeout=Duration::MAX 被接受，锁定 reqwest 在首次请求内进行 Instant 加法并 panic。 | `Config::validate` 与 `HttpTransport::new` 拒绝0或超过300秒的超时。 |
| P2 | ZIP 目录携带正文或错误 CRC 仍通过；普通文件中央目录声明长度与实际展开长度不符也能发布。 | `cache.rs` 要求目录为空且读取到 EOF 验 CRC，并核对实际文件长度。 |
| P2 | 已完成 GUI 仍创建后端并查询业务 tick；本可直接返回的结果可能因为后端不可用失败。 | `gui_runner/native.rs` 在创建窗口前直接返回终态，终态/错误状态不再查询 tick。 |
| P2 | 锁定 iced fork 用 expect 创建 event loop，初始化失败或同进程再次创建会越过 Result 接口并 panic。 | `gui_runner/native.rs` 在依赖运行边界捕获 Rust unwind，返回包含原始详情的 GuiError；不改全局 panic hook。 |

ZIP 两类损坏样例先复现失败，再修复为拒绝并确认无临时目录残留。订阅回归通过通道阻塞未来包预取，确认预加载主题按时过期，避免仅靠最终状态或冻结时钟掩盖问题。另验证 worker panic 唤醒、Age/304 策略继承、no-store validator、慢正文消耗新鲜期，以及已完成 GUI 完全不触达窗口/tick。

## 验证

在 workspace 根执行：

```sh
cargo test --offline --locked -p tg-theme -p tg-iced -p tg-gpu-selector --all-targets --features tg-iced/native-gui,tg-gpu-selector/native-gui
cargo test --offline --locked -p tg-theme --doc
cargo clippy --offline --locked -p tg-theme -p tg-iced -p tg-gpu-selector --all-targets --features tg-iced/native-gui,tg-gpu-selector/native-gui --no-deps -- -D warnings
cargo fmt -p tg-theme -p tg-iced -p tg-gpu-selector --check
```

原生特性下45项测试通过：tg-theme 23（13单元、10集成）、tg-iced 10、tg-gpu-selector 12；文档示例1项编译通过。严格 Clippy 与格式检查通过。依赖 block 0.1.6 保留 future-incompat 提示。

本轮修复后未留下上述已确认缺陷。测试运行于 macOS，未验证 Windows 或真实窗口视觉；锁定 GUI 后端仍要求主线程和同进程单事件循环，本轮保证失败转为可诊断错误，不声称支持多次独立创建 event loop。真实主题服务部署与整游戏消费接线未验收。全局工作区仍遵循[上轮报告的范围外检查说明](../theme-ui-refactor-2026-10-17/README.md)，本轮没有把局部检查升级为全库通过。
