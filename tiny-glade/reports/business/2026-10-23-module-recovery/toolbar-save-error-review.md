# TB-R3 保存错误链独立复查

范围：真实异步保存错误 → 游戏线程错误快照 → 原保存错误模板／备份信息 → 成功恢复后 15 秒提示。真实 Steam SDK 按用户要求排除；未发送 OS 输入，未声明 toolbar 或整游戏验收。

## 原证与修正

- 原 B `save_error::ui` `0x141ec0ce0`：失败时每帧重置 Local<Instant>，展示第一条错误；错误清空后仅菜单关闭且 elapsed.as_secs()≤14 时显示恢复提示，透明度 `1−clamp(elapsed.as_secs_f32()/15,0,1)`。错误／恢复根节点忽略输入。当前使用同一真实 `SaveThreadHandle.error_state` 接收端，保留原有异步线程 owner。
- 原 `AutoSaveError::from` `0x141720490`：anyhow downcast 首个真实 `std::io::Error`；原 Windows OS 码 39、112，或标准 StorageFull，选 tag0，**只格式化该 I/O 错误**；其他选 tag1，格式化完整 anyhow 显示链。修复先前 StorageFull 错含外层上下文、以及遍历任意嵌套 source 误分类的行为。字符串“disk full”不能改变分类。
- 原存档线程 `0x141720823/83d`：以 `ShadowFile.last_error` 保留重试。原对象基址 `rbp-0x48`，错误槽 `+0x78`，即 `rbp+0x30`。当前 `sync_all` 先 flush 清除脏块，再执行 OS sync；仅检查 dirty 会在 sync 失败后误报恢复。已拓宽现有重构线程门控为 `last_error.is_some() || dirty_blocks.any()`，保留当前正常脏写路径，同时恢复错误重试。
- 原错误页才进入备份slot；将备份路径改为懒求值，健康帧不再格式化UUID／分配PathBuf，也不枚举磁盘。无资产健康帧测试约束此边界；没有将静态分支检查称为性能基准。
- **复查新增的实际数据丢失缺口**：CloseFile／OpenFile 旧档失败原来仅登记路径后丢弃 ShadowFile；下一 tick 因无 current 或新 current 正常而发布空快照，错误页会误显示保存恢复。原主循环 `retain_mut<ShadowFile>`、`UnsavedFileList::add` `0x14171ff80` 的完整 128 字节移动、闭包 `0x14130f900` 的 sync→失败保留／成功移除证明应保留文件 owner。现同一共享 UnsavedFileList 增加真实 Pending(ShadowFile)，两处关闭旧档失败均转移完整文件；每秒重试，并按“当前文件→旧文件插入顺序”合并错误，成功后卸载并解除 contains_file 加载保护。原成功日志也保留。
- 现有 UnsavedFileEntry／add_path 只作既有加载保护的兼容路径标记；它的 128 字节尺寸测试是兼容性检查，**不是当前 pending owner 的原生 ABI 证明**。不为只有路径的标记编造文件数据，也不伪造其成功重试。实际生产关闭失败已全部转入真实 ShadowFile，保留已应用 deferred 的完整字节。
- 原三个 slot 闭包 `0x140fc8100/0x140fc8530/0x140fc87b0`：错误／路径为红色、15、居中；计划为 19、左对齐；备份为 13、居中；Hand 字体、最大宽度 450。备份按完整 SystemTime 降序比较，不截成整秒。
- 日期静态 `0x142bb4010` 是 `{ptr=0x142bb3f08,len=11}`，11 个 24 字节 time 格式项依次为 Year、`-`、Month、`-`、Day、空格、Hour、`:`、Minute、`:`、Second。当前本地时间 `YYYY-MM-DD HH:MM:SS` 与该表一致。原始字节见 `toolbar-save-error-review-date-format.json`。

## 验证设计与边界

- core 新增6项合同测试、save_thread模块共16项回归（包含6项）：旧档CloseFile失败后完整deferred JSON仍可落盘；两个旧档部分恢复时保持另一错误、全部恢复才清空并卸载；真实 I/O kind 分类／上下文／嵌套 I/O 不误分类；有序快照替换与断链保持；真实临时目录缺失导致失败，再创建目录触发线程自行重试并发布空快照；真实空文件创建失败没有脏块，仍应重试，直接约束本次门控修复。
- toolbar 测试：健康帧不读取备份路径；原模板／字体；持续失败更新时间、菜单门控、14.999 秒与 15 秒边界、恢复透明度、再次失败；真实 Bevy Schedule 的 `ui` 系统、Local 生命周期、同一接收端最新快照、第一条错误、备份计数／最新本地日期、错误清空与重新关菜单。
- 实际 adapter 测试在独立子进程内初始化临时账号 paths，读取原 en-US 与模板资产。先通过真实队列验证连续快照、备份和菜单分支；继而真正spawn存档线程并发送OpenFile，缺会话目录导致真实I/O失败，经该线程自己的接收端进入同一ui系统；创建目录后线程自行重试落盘、UI接收空错误快照并显示恢复提示。这一段是连续真实worker→channel→Bevy ui链（无需注入预设错误文字）。临时备份／会话文件均在独立临时账号内，不触碰用户存档；未将该CPU链称为完整游戏窗口验收。
- 没有注满真实磁盘，也未注入真实 OS sync 失败；StorageFull 属于标准 I/O 类型分类测试。原分类／重试门控及样式／日期为静态指令证据，不是原 Windows 运行时与当前 Mac 全帧差分。

中途14项回归日志仅是尚未修复旧档保留前的历史候选，最终验收使用16项日志，不叠加为30项。

最终测试和源码指纹见 `toolbar-save-error-review.json`。

测试首轮adapter在加载语言包时因未显式configure而使用cwd默认assets/lang失败；失败日志保留为toolbar-save-error-review-consumer-initial-failure.log。修复测试初始化为原安装目录assets/lang后重跑，不修改生产语言库或放宽业务断言。

经验：不能把“脏块为空”等同于保存成功；flush和OS同步是两个失败边界，错误状态本身必须驱动重试。anyhow链内的错误身份与其source链不同，原downcast对象不能改成任意递归搜索。原日期常量先解fat slice再识别锁定time格式项，避免凭邻近字符串猜格式。真实adapter测试须隔离paths/lang的全局OnceLock并显式configure原资产根；共享Cargo一次构建可复用同一toolbar lib测试bin，避免每个owner重复抢锁。

最终结果：save_thread模块16项回归通过（其中新增6项），旧路径标记兼容性1项通过；toolbar原资产／真实异步worker→UI链3项通过（2.15秒），显式include-ignored运行，0项忽略。最终toolbar no-run构建通过。
