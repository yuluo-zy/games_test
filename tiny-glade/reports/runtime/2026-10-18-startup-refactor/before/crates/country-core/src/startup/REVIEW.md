# startup 代码正确性审查（2026-10-18）

审查对象：本目录 14 个 Rust 文件，以及会话、存档、音乐、预制件和真实宿主的相关调用方。actor：当前聊天主审查者。用户授权为分析、README 和代码 review，本轮未修改业务源码或数据库。

结论：**changes_required（2 P1 + 2 P2）**。这是当前实现的源码正确性审查，不是完整原作恢复验收；没有冻结交付包，不输出原生等价 `passed`。源文件 SHA256、原文件身份和运行日志见 [证据目录](../../../../reports/startup-review-2026-10-18/README.md)。

## F1 — [P1] 聚合注册入口无法保持会话请求和初始化的生命周期

- 位置：[run_game.rs:118](run_game.rs#L118)–122；外层 [mod.rs:91](mod.rs#L91)–97。
- 事实：同一个 Schedule 无条件注册 `music_audio_init_system`、`load_game_session` 与大量每帧系统；会话加载会延迟删除 `NextSession`，却以必需的 `ResMut<NextSession>` 访问它。`commander_update_system` 同样必需读取该资源。没有相应 run condition 或整体阶段边。
- 触发：宿主按注释使用聚合入口构建反复运行的游戏 Schedule。首次加载成功并应用延迟命令后，下次运行会因缺少 `NextSession` 而产生系统参数校验错误；音乐初始化每次运行都会覆盖 `MusicAudioState`。外层还会反复加载花园/ivy 网格。如果只运行一次，则已塞入同表的输入、历史和域运行系统不会持续推进。
- 影响：该公共入口没有一种简单的“一次/每帧”调用方式能满足所有已注册系统的生命周期；源码中按顺序调用注册函数不保证执行顺序。
- 建议：把资源装配、一次性资产初始化、带请求门控的会话阶段与每帧系统拆为明确入口，在宿主建立顺序与 deferred 边；会话请求删除后不得继续必需访问它。
- 证据：当前注册源码；`startup_session.rs:136` 的删除；`music_system.rs:46` 的资源覆盖；已有 B 注册反编译显示 PreStartup / OnEnter / OnExit 分表。当前真实宿主已单独分表并为会话加载设置条件，没有调用这个旧聚合入口，故不把该问题报告为当前生产宿主必现崩溃。本项为静态闭包审查，未整表运行。

## F2 — [P1] 会话加载器拒绝本模块生成的 on_load 请求

- 位置：[startup_session.rs:139](startup_session.rs#L139)–142；生产者 [run_game.rs:66](run_game.rs#L66)–72。
- 事实：`build_new_session_on_load` 压入初始命令和回放终止项；`MetaCommander` 的会话切换生产者也生成含 fence 的 `on_load`。加载入口却要求 `next_session.on_load.is_empty()`，否则 panic。
- 触发与影响：请求经过现有辅助函数或命令器填入 on_load 后进入加载器，在读取存档前即中断，初始命令和 fence 都无法到达加载后的消费者。注释已坦承 owned conversion 未恢复，因此这是明确的功能阻断；不能简单删掉 assert 后丢弃请求。
- 建议：将拥有的命令/fence 转换并转交 `LoadGameManager` 的后置命令链，保证所有权、顺序与完成信号；在实现之前由可失败接口明确拒绝该功能，避免公共生产者与消费者表现为可组合却崩溃。
- 验证：`session_loader_rejects_its_own_on_load_producer` 调用当前生产者，再运行当前加载系统，捕获并核对该断言文本。未访问真实存档目录。

## F3 — [P2] FMOD 参数名误拼接，主题参数错误复用 0.25 音量标量

- 位置：[load_glade_audio.rs:28](load_glade_audio.rs#L28)；[load_glade_audio.rs:77](load_glade_audio.rs#L77)–83。
- 事实：当前第一参数名为 `GladeTypeIsHalloween`，两个参数值都使用 `halloween_theme_flag`，仅为 0 或 0.25。
- 原指令核对：B 原发行 RNE SHA256 为 `f507a6ce…2b7d03`。`0x1408bea83` 装入 `0x142aa0ce8`，下一指令设长度 **9**，实际参数名是 **`GladeType`**；`IsHalloween` 位于紧邻地址 `0x142aa0cf1`，长度 11。不能将相邻字符串在内存中的拼接视为一个参数名。第一参数值在 `0x1408bea6c`–73 按主题索引读取表 `[0, 3, 1, 2, 4]`。0.25 是此前写入 player 的独立标量。
- 第二参数：原指令 `0x1408beaca`–af2 独立计算 0/1，再调用第二个参数；它同样不是 0.25。精确的主题字符串判别语义不在本轮运行验证范围。
- 影响：Winter 当前发送 `(GladeTypeIsHalloween, 0.25)` 和 `(IsHalloween, 0.25)`，正确的第一参数应为 `(GladeType, 4.0)`。后端可能报告参数不存在，或保持错误的主题混音值；当前调用还忽略错误结果。
- 建议：将参数名改为 `GladeType`，主题值复用 `GladeType::audio_id()`，另行保留 player 标量与 IsHalloween 的条件值，统一同名加载入口。
- 验证：后端记录探针确认当前 Winter 两次写值；原发行文件局部反汇编与 PE 常量读取确认名字边界和主题表，记录于 `original-load-glade-audio.asm` / `original-audio-data.json`。这是静态取证，没有 FMOD 原进程执行。
- 相关影响：`systems/audio/music/load_glade_audio.rs` 虽按 `audio_id()` 写主题值，也引用同一个错误常量，修正常量时须检查该消费者。两份文件对停止/暂停也有差异，但本轮未把该差异单独认定为新缺陷。

## F4 — [P2] 预制件重载失败提前丢弃唯一通知

- 位置：[load_prefabs.rs:158](load_prefabs.rs#L158)–162。
- 事实：`take_changed()` 在读取、解析 RON 之前清空集合，随后 `load_prefabs_impl(...) ?` 可以返回错误，失败路径不归还通知。
- 触发：编辑器正在写入文件，或者 RON 暂时无效。第一次调用返回 Err 并消费通知；调用方随后仅重试同一 API，即使文件已经修好，也因集合为空而直接成功返回，不再扫描。
- 影响：库保持旧状态，宿主没有成功事件，也没有待处理变更；必须依赖另一个文件通知才能恢复。最小探针不需要真实 watcher。
- 建议：失败时将取出的集合合并回待处理集合，保留期间新到的通知；成功后才提交对应事件。不要通过持锁做整段文件 I/O 来阻止通知。
- 验证：`failed_prefab_reload_consumes_the_only_retry_notification` 先写坏 RON、入队并重载，再修复文件重试；第二次返回空集合，库仍为空。

## 已确认的功能限制与待补证

这些项单独列出，不将所有恢复占位重复计作新算法缺陷：

| 范围 | 当前事实 / 后续条件 |
|---|---|
| prefab 删除 | 成功重扫只 add/replace，不移除磁盘已删除的条目；探针已复现。原监听只收 Write，尚未确立完整增删同步合同；宿主不能把它当作磁盘镜像 API |
| prefab 实体资源 | `mesh` 为空、bounds 为零、部分音频字段为空；完整 mesh/atlas/RT/音频链未成立。扫描依赖表也未注册到 watcher |
| watcher 原子保存 | 仅接受 Write；Rename/Create 不触发。源码明确标注原枚举映射未确认；作为编辑器兼容缺口保留，不宣称已证明原版接受这些事件 |
| glade 音频 | 此目录入口忽略目录，不加载 audio.json；音频域已有 GladeAudio 读取器，故“schema 完全未知”的旧注释过期 |
| ToD/天气 | 文件已导出，全文检索没有生产调用；标准 Cargo 验证被锁文件阻断，缓存支持库尚无这套最新 Weather/Deserialize 接口，未用替身补造通过 |
| 完整会话加载 | 下游 `try_load_game_system` 在解析后仍明确返回“快照/历史应用未恢复”，没有发加载完成；本轮不以启动请求成功为整场景完成 |
| 新会话持久性 | 写 meta/发事件/检查 history 的过程没有回滚，snapshot 复制错误被忽略；其中复制忽略有原证据注释，未当作无依据的新偏差 |
| 地形边界 | 使用 wrapping u32，调用方负责顶点范围和尺寸；现有测试没有覆盖极大整数溢出与原版索引位宽等价 |
| 旧文档 | mod.rs 对原 28 系统、单调度/多调度存在不同轮次描述，部分注释称未实现的文件现在已存在；新 README 以当前可执行代码与实际宿主为准 |

## 验证结果与边界

- `cargo test -p country-core --lib startup:: --offline --locked`：exit 101，工作区锁文件需更新，未进入编译。没有自动改写共享锁文件。
- 六个当前文件的独立源码探针：22 passed（18 个既有测试、4 个缺陷/限制探针），source unchanged；支持库为已绑定 SHA256 的缓存产物。首次 harness 因未设置 derive 所需的环境和缓存缺最新 ToD 类型失败，保留失败日志；调整后只编译明确支持的六个文件，没有伪造 ToD 接口。
- F1 是静态注册审查；F2/F4 与删除限制有可执行反例；F3 有后端调用记录及 B RNE 原指令/数据静态核对。
- 未执行当前完整 Cargo 构建、完整原生业务差分、FMOD/GPU 或整游戏验证；测试数不支持原作恢复进度升级。
