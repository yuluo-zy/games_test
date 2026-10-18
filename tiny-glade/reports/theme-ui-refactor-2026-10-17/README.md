# 主题与辅助 GUI 重构（2026-10-17）

后续二次复审发现并修复的边界问题与最新验证见 [复审报告](../theme-ui-rereview-2026-10-17/README.md)。下文保留首轮重构交付记录。

原 theme-defs、theme-server-proto、theme-fetcher 合并为 [tg-theme](../../crates/tg-theme/README.md)；[tg-iced](../../crates/tg-iced/README.md) 按业务策略、窗口适配、截图与文件输出拆分。删除旧主题包和多余生成 fixture，迁移 tg-gpu-selector 的原生命令适配，更新 workspace、Cargo.lock、当前包表和受影响的来源清单。没有执行逆向、请求真实服务或打开 GUI。

## 修复与行为变化

- CID、活动顺序和时间运算在边界验证，阻止路径越界及时间溢出。
- ETag 与列表验证后一起提交，失败可恢复；304 根据新 Date 重算时间，成功刷新重置退避。
- 响应头到达后采本地时间，I/O 完成后安排截止时间，避免把网络耗时算成时差或吃掉退避。
- 内容包流式校验，临时目录解包并完整验证后原子发布；失败清理，损坏缓存重建。清单相对路径，拒绝额外文件、遗漏父目录、符号链接、异常深树、重复和平台不兼容路径。
- 列表、归档、展开尺寸、文件数量及清单内存均有限额；每个缓存根由自动释放的操作系统文件锁独占。
- 同 CID 去重并跨刷新保留包状态，按最久未尝试安排下载，失败包不会饿死其他包。状态变化才通知，订阅 Drop 可唤醒取消，显式 shutdown 回收线程。
- GUI 命令错误返回调用方，剪贴板使用 iced 原生命令；浏览器/日志打开错误沿相同路径传播。首次对话框选择形成不可覆盖终态。
- 截图不执行业务输入、tick 或 update；输出名追加 .json，两个文件仅新建，失败回滚本次文件并保护已有输出。
- 按钮使用静态切片，布局适应窄窗口和长文本；源内注释说明职责和不变量，历史地址留在来源文档。

## 性能依据

主题数据不再使用每事件 ArcSwap；可变状态由 worker 单独拥有。网络及 ZIP 使用固定32 KiB缓冲流式处理，协议重校不复制整份 wire 列表。离线测试断言刷新同 CID 不重复下载，并验证故障包调度公平性。没有用未经测量的吞吐倍数描述本轮优化。

## 验证

工具链为 macOS aarch64 rustc/cargo 1.98.1。tg-theme 因使用标准库文件锁，单包 MSRV 为1.89；未以本机运行宣称已在1.89工具链验证。没有新增第三方依赖或主动升级依赖版本。

- 默认 all-targets 测试34项通过：tg-theme 17、tg-iced 5、tg-gpu-selector 12。
- native-gui all-targets 测试37项通过：tg-theme 17、tg-iced 8、tg-gpu-selector 12；示例编译通过。
- tg-theme 文档示例1项编译通过，不执行网络。
- 受影响三包 all-targets 严格 Clippy 和格式检查通过，含 native-gui。
- 保留外部依赖 block 0.1.6 的 future-incompat 提示；本轮没有更换上游 iced fork。

全局 `tools/verify_inventory.py` 仍报告本轮范围外的 wall-constructor 桩、其他系统包 README 与既有依赖身份/候选配置问题。结果见 [workspace-inventory.json](workspace-inventory.json)，不能把受影响包的检查通过描述为整个工作区验收通过。

## 使用与迁移

调用方统一引用 tg_theme::{Config, ThemeClient, ThemeState}，旧三包无转发壳。原始导入快照不改写；当前来源清单中的受影响文件指向本次重构，旧条目保存于 [import-history.json](import-history.json)。历史原版验证不升级为新实现的等价性声明。

默认服务地址改为 HTTPS；未验证真实主机当前部署，地址、限额和超时可配置。自定义传输负责自身 I/O 截止时间；shutdown 会等待当前文件系统操作。完整游戏消费端、真实字体/像素及真实服务部署仍需各自集成验收。
