# 主题三包与 tg-iced 源码审查（2026-10-17）

后续已完成合包和缺陷修复，当前实现与验证见 [重构报告](../2026-10-17-theme-ui-refactor/README.md)。下文保留审查时的旧路径、行号和结论，不能作为重构后仍存在这些问题的依据。

范围：theme-defs、theme-fetcher、theme-server-proto、tg-iced。仅阅读当前源码、梳理调用关系和运行离线检查，没有执行逆向、请求真实主题服务或打开 GUI。本轮更新四包 README，未修改业务实现。

## 当前业务与完善程度

| 包 | 业务与结论 |
| --- | --- |
| theme-server-proto | 服务端 JSON schema：有序 events 列表，每项包含有符号秒 start/end 与字符串 cid。serde 类型校验完整，但不负责 CID、时间区间和数量校验；未发现独立确定 bug。 |
| theme-defs | 本地活动目录、完整精度起止时间及 Pending/Error/NoActiveEvent 状态；半开区间判断和完整值相等比较供发布去重。职责小而清晰，未发现独立确定 bug。 |
| theme-fetcher | HTTP 条件请求、时差换算、失败重试、ZIP/BLAKE3、CAS 清理及活动变化发布已形成正常路径。输入校验、失败恢复和线程退出存在下列缺陷，不能认为完善。 |
| tg-iced | 对话框 builder/布尔结果状态机、本地化入口、可注入 GUI backend 和可选 iced native adapter 已实现。截图文件写入有缺陷，命令适配、视觉与测试覆盖仍有边界。 |

主题数据链为 proto → fetcher → defs；源码扫描尚未发现这三包接入游戏生产消费端，现有跨包 consumer 是离线测试/示例。tg-iced 已由 tg-gpu-selector 使用；它与主题三包没有直接业务依赖。

## 确认问题

下列问题是当前行为，部分已被现有测试明确保留，不是本轮引入的回归。

| 优先级 | 位置 | 触发与影响 | 建议 |
| --- | --- | --- | --- |
| P1 | `crates/theme-fetcher/src/lib.rs:330`–333 | CID 是任意字符串；传入 `..` 或已存在的绝对目录即可命中 is_dir，绕过下载和所有校验，并发布缓存根以外的活动目录。 | 在生产者入口限制为规范 BLAKE3 CID，并保持缓存路径受根目录约束。 |
| P2 | `crates/theme-fetcher/src/lib.rs:331`、352；`src/checked_zip.rs:48`–53 | ZIP 中前一文件成功、后一文件失败会留下部分目录；下轮仅凭目录存在就返回 Ok，发布损坏包。已有 failed_extraction_directory_is_a_cache_hit_on_retry 测试直接确认。 | 临时目录完整解包及验证后发布；失败清理，命中检查完成标记。 |
| P2 | `crates/theme-fetcher/src/lib.rs:294`–305 | 200 的新 ETag 在读取/校验 body 前保存。若读取、长度或 JSON 校验失败，重试得到304就保留旧列表或空列表，直到服务端版本变化。 | 在新列表成功接受时一并提交 ETag，或失败后清除条件请求状态。 |
| P2 | `crates/theme-fetcher/src/lib.rs:534`–536 | 首次成功发布后消费端断开，状态不变便不会再次 send，线程无法发现断开，继续轮询和网络工作。 | 增加显式取消/关闭信号，并让等待可被取消。 |
| P2 | `crates/tg-iced/src/gui_runner.rs:294`–297 | 输出路径为 capture.json 时 sidecar 路径等于原路径，元数据覆盖刚写的像素，仍返回 Ok。 | 生成不冲突的 sidecar 名称或拒绝路径冲突。 |

## 其他边界

fetcher 对响应、ZIP 解压和清单没有总量限制；串行预下载所有活动会推迟发布，活动判定时间取在 blob 下载之前。checksum 验证仅检查清单记录的路径，忽略额外文件，不构成来源认证；主题内容 schema 由消费端负责。

tg-iced 默认 native 命令适配只处理 None/CloseWindow，其余命令需要 consumer 覆写。截图 probe 会转发输入和 tick，可能执行 consumer 的 IO；源码的 no input/IO 注释与实际边界不符。小窗口、长翻译、真实字体和像素布局未验收；唯一默认测试没有覆盖确认/取消、直接关闭、后端错误和截图写入。

## 本轮检查

在 workspace 根执行：

```sh
cargo test --offline --locked -p theme-defs -p theme-server-proto -p theme-fetcher -p tg-iced --all-targets
cargo check -p tg-iced --features native-gui --all-targets --offline
```

统一测试退出码0，共27项通过：defs 3、proto 5、fetcher 18、iced 1。native-gui 全目标编译通过，依赖 block 0.1.6 有 future-incompat 警告。fetcher 独立 target 的另一轮构造器测试出现 system-configuration NULL object panic；统一运行未复现，记录为环境差异，不定性为稳定包内 bug。没有通过增加或修改测试掩盖现有问题。

各包 README 包含更详细的 API、执行流程、校验边界及历史来源；本次源码审查不升级原版等价性或完整游戏集成结论。
