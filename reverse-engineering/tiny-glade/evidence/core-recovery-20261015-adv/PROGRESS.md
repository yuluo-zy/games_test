# country-core 优先模块推进台账（2026-10-15 第二轮）

协调简报：[COORDINATION.md](./COORDINATION.md)。三组隔离推进 9 个优先模块。
中央 agent 只做台账维护、跨组文件仲裁、进度提醒；不改 country-core 源码、不跑 cargo。

## 模块台账

状态取值：研究 / 实现 / 复核 / 待取证（placeholder 起于 研究，证据列初始为空）。
模块状态只有在收到该组实时复合 agent 的 DELIVERY 摘要后才推进；"编译通过≠完成"。

| 模块 | 组 | 状态 | 证据 | 合同剩余缺口 |
|---|---|---|---|---|
| ivy | A 植被 | SPEC 完成 | A-vegetation/ivy-SPEC.md | 完整 grower/root spawner、wall-change point-validity pruner、root/branch 删除策略、UI/ActionSet 门、wall jump/hole 约束、GPU 上传/渲染（后置）；`ivy_leaf_spawner` 需接 GladeSettings +1c 真实旗标 |
| garden | A 植被 | SPEC 完成 | A-vegetation/garden-SPEC.md | 完整 UI 仲裁/其他 WidgetParams 变体、signifier 所有者、原全局调度、56B ShowPinnedDecoClutter union 与 renderer、下游 garden scatter/audio 消费者 |
| sheep | B 生物 | SPEC 完成 | B-creatures-audio/sheep-SPEC.md | 完整 Sheep owner 与生命周期、真实 Sheep query 所有者、save/load 接线、spawn/movement |
| music | B 音频 | SPEC 完成 | B-creatures-audio/music-SPEC.md | 完整 GladeAudio track producer、AudioCore/device/bank 启动、live FMOD 集成（libfmod 2.222.6，锁 checksum 75defbe5…）；music_player::load_tracks 已实现，完整 GladeAudio 加载消费者未恢复 |
| input | B 组（sheep/music/input） | SPEC 完成 | B-creatures-audio/input-SPEC.md | 键盘/Steam 映射、reset/replay 集成、update_cursor_position、原全局调度、OS 事件生产者 |
| terrain | C 世界 | SPEC 完成 | C-world-services/terrain-SPEC.md | terrain UI/new-stroke 输入生产者、原 plugin 资源初始化与全局调度、terrain history/replay 生命周期、audio 播放、GPU 执行（后置）、下游 wall/collision/scatter 订阅链 |
| save | C 世界 | SPEC 完成 | C-world-services/save-SPEC.md | 完整 AsyncSaveCmd 变体、deferred 命令、主后台保存循环/unsaved-file 重试、每私有域的游戏线程捕获/序列化、快照二进制捕获/load、自动/手动备份生产、session 切换自动跑 GC、完整 save-session 生命周期 |
| session | C 世界 | SPEC 完成 | C-world-services/session-SPEC.md | 完整 NextSession(160B) 所有者、GameSaveData(240B)/MetaCommand/SessionOnLoadCmd 派发、SessionLoadAttempt、完整 world load/commit 事务；`create` 0x140b46960、`append_on_load` 0x1403b5150 等入口 |
| ui_clutter | C 世界 | SPEC 完成 | C-world-services/ui_clutter-SPEC.md | 原 RMB picking、parent/roof transform 解析、auto→decorator/stair 转换、UI 交互、消费删除 ID 的 generation/mesh owner（owner 后置可先留真接口） |

注：input 归属已由父确认为 B 组（sheep/music/input）；COORDINATION.md 所有权表已补 input 行（文件 `systems/backend/input/*`，证据 `evidence/core-recovery-20261015/input/`）。涉及组外文件时一律先仲裁。

## 跨组文件仲裁记录

共享但需仲裁的文件：`resources/history/*`、`systems/history.rs`、`resources/glade_settings.rs`、`resources/session.rs`、`resources/audio*`/FMOD。

| 时间 | 请求方 | 目标文件 | 请求 | 裁定 |
|---|---|---|---|---|
| （暂无） | | | | |

规则：
- 只加已观测字段/消费者且不与其它组冲突 → 批准并登记。
- 两组要写同一文件 → 指定先后顺序，通知父裁决。
- 未经批准的组外修改一律拒绝并记入本台账。

## 防水分

- "编译通过=完成" 或 "新增测试=完成" 一律打回。
- 每个 DELIVERY 必须逐项分离：真实代码接线 / candidate 链测试 / 原侧验证 / 仍缺项。
- 发现水分向父报告；不打进度、不改台账状态直至 DELIVERY 合规。
