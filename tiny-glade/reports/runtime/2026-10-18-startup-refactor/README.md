# startup 工程修复与重构（2026-10-18）

用户明确授权以功能性、正确性和工程可读性优先，可以偏离逆向设计。本轮修复 F1–F4，并整理中文注释与业务命名；不重新执行逆向、不修改原文件或数据库、不记原作恢复信用。

业务说明：[startup/README.md](../../../crates/country-core/src/startup/README.md)。修复前审查：[startup/REVIEW.md](../../../crates/country-core/src/startup/REVIEW.md)。

| 问题 | 修复与回归范围 |
|---|---|
| F1：单 Schedule 混合生命周期 | 资源初始化、一次性启动、会话请求、每帧更新分开；聚合入口明确接受 world/startup/session/update 四参数；音乐状态和网格初始化有重复运行保护；命令器不再必需读取已删除的 NextSession |
| F2：非空 on_load 被拒绝 | 完整命令按所有权转移；Fence 使用 Arc 而非整数还原；完成动作按游标重试，不重复入队，成功后才切换模式与释放信号；ECS Loaded 消费端接入并只通知一次；旧裸地址 fence 明确拒绝 |
| F3：音频参数错误 | GladeType 使用正确名字和 audio_id；IsHalloween 独立为 0/1；冬季标量 0.25 单独保留；两个入口统一；普通主题读取真实音频配置，失败保留旧状态；曲库替换不积累旧实例 |
| F4：重载失败丢通知 | 锁外建立新快照，失败合并归还两类通知，旧库不变；成功后替换并处理删除；支持原子保存事件；图集单独变化跳过全目录扫描 |

其他整理：新会话在临时目录中完成 meta/snapshots 后发布，失败自动清理；优先使用各 World 的 demo 配置；shader Result 底层实际传播错误；时间序列用 BufReader、解析成功后替换；地形索引去递归、尺寸与溢出检查、一次预留容量。没有加入新的依赖算法或自建线程/监听系统。

预制件删除接入了现有实例内容清理函数，避免库条目移除后消费者 get/panic。网格、AABB、部分音频 schema 消费和全量依赖监听仍未接入，未把这些占位包装成完整资产。

## 兼容与依赖

- `register_core_startup` / `add_game_startup_systems` 改为三个独立 Schedule；真实宿主原本自行注册各 App 阶段，没有旧二参数调用点需要迁移。
- 音频类型入口和字节入口返回 Result；ECS 音频入口返回 Bevy Result，两个入口共享实现。
- NextSession/MetaCommander/LoadGameManager 的相关消费者同步适配拥有型动作。加载完成回调仍需先有真实世界应用，不能用它绕过快照/历史缺口。
- 将真实宿主已有的 glam 声明从 dev-dependencies 移到 dependencies，支持运行代码中的 Vec3 引用；没有变更版本。
- 为已有 `gpu_lut.rs` 中的 App/Plugin 引用补充 `bevy_app.workspace = true`；复用工作区锁定的 0.16.0，不选择新版本。
- 同期 PassHost 已将旧查询改名为 `ssbo_instance_count`；适配 deferred_raster 与其测试宿主两个遗留调用点，未添加伪造实现。
- 源码在共享目录持续变化。`changes.diff` 是已捕获基线与当前文件的观察差异，可能含同期修改；验证源清单用于确定实际输入，不声称整个工作区冻结或所有差异均由本轮产生。

## 验证

最新 startup 相关 **35 项全部通过**，真实宿主全部目标编译通过，startup 格式检查通过。较早完整快照为 **2121 passed，0 failed**；共享存档与渲染继续修改后的最新全库结果为 **2142 passed，4 failed**。四项失败分别位于 Nani buffers、Nani draw_ext（两项）和 transparency_pass 的 ice compute 测试，不将这些失败算作通过。

Clippy 首次遇到同期存档代码引用尚未添加的 stairs 校验标志，失败日志保留；标志随后出现，但本轮重试持续等待共享 Cargo 构建锁，已取消本任务队列进程，**未宣称最终 Clippy 通过**。临时独立缓存准备也已取消并清理，实际检查都使用原 target。

最新补验期间观察到 `startup/run_game.rs`、宿主 Cargo.toml、stairs_state.rs 又发生变化。保存验证起点及最新观察指纹，不声称持续变化的全工作区已冻结或最终全部通过。

| 记录 | 内容 |
|---|---|
| baseline.log | 最初构建缺少目录外 gpu_lut 所需 bevy_app 依赖 |
| startup-tests.log | 第一轮启动相关 20 项测试通过，属于中间源码快照 |
| core-tests.log / core-tests-retry.log | 中间接口/导入适配失败，保留真实失败记录 |
| core-tests-final.log | 当前源码完整 country-core 库测试结果，包含新增的动作、信号、重试、删除和溢出回归 |
| host-check.log | 首次宿主检查因磁盘无空间退出，未以此作为源码错误 |
| host-check-retry.log | 宿主运行代码引用 glam，但只声明了开发依赖，构建失败 |
| host-check-final.log | 调整已有依赖声明后，真实宿主全部目标编译通过 |
| core-tests-current.log | 同期历史恢复代码及完成动作预检后的最新全库回归 |
| clippy.log / clippy-current.log | 首次检查遇到同期 stairs 校验标志缺失；最新组合重查结果 |
| format-check.log | 限定格式检查 |
| validation-source-sha256.json | 库测试完成时的 27 项源码/清单/锁文件输入；后续验证核对变化 |
| before/ / before-sha256.json / changes.diff | 已读取的旧源码快照与观察差异，保留旧设计和注释追溯 |
| cache-cleanup.json / old-core-artifacts-cleanup.json | 仅清理可再生成的旧 country-core 增量缓存、旧测试程序和归档；未删除源码、资产、证据或最新产物 |

测试不连接真实用户存档、FMOD 或 GPU。性能改进仅作结构结论：减少重复初始化、无效配置扫描、临时字符串分配和索引扩容，没有量化基准。

共享存档模块同期接入无有效快照时的历史恢复分支，完整快照/全游戏不在本轮验收范围；不宣称整游戏能完成加载。原生等价、A/C 版本和真实音频/GPU 均未验收。修复前的隔离探针与旧原码差分不覆盖当前工程实现，历史报告原样保留。
