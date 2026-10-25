# input 重构后重新审查

日期：2026-10-18。审查者：当前 Codex 主代理。结论：**changes_required**。

本次从当前源码重新检查 `country-core/src/input`，沿调用链读取 backend 输入、时钟、真实工具栏/模式消费者和宿主注册。以用户授权的功能正确性为标准，不要求继续遵循逆向设计，不声明与原生 B/C 等价。没有修改运行时代码；上一轮通过的测试不能排除以下集成问题。

## R1 · [P1] 快照恢复了没有业务消费者的工具栏副本

位置：`crates/country-core/src/input/input_stream.rs:25–38`，`systems/backend/input/input_record_replay.rs:111–114`。

`InputSnapshot.toolbar` 和控制系统使用独立 ECS 资源 `ToolbarCategoryModes`。当前真实工具切换使用另一资源 `Toolbar`（内含所有类别、各类别索引和当前类别），并修改独立的 `AppMode`：参见 `systems/mode_manager.rs:119–125,142–147` 和 `resources/toolbar.rs:87–90,186–203`。当前全仓库没有将独立 `ToolbarCategoryModes` 同步到实际 `Toolbar`/`AppMode` 的生产链。

反例：通过实际 `select_app_mode` 在 Path 模式开始录制；改为 Water 后 seek。恢复只改变独立类别资源，实际 `AppMode` 和 `Toolbar.current` 仍是 Water。后续录制中的点击按 Water 解释；`ui_edit_terrain` 等消费者也直接按 `Res<AppMode>` 分支。当前 `input_pipeline` fixture 仅检查独立类别副本，没有创建实际 `Toolbar`/`AppMode`，因此无法暴露此问题。

建议：快照直接保存并恢复实际 `Toolbar` 与 `AppMode`，按既有模式切换通知链更新消费者；去掉没有同步关系的镜像资源。相机/工具栏/模式应在回放首帧消费前一致。

## R2 · [P2] 控制命令与时钟更新没有形成完整顺序

位置：`systems/backend/input/input_record_replay.rs:193–201`，`systems/backend/input/mod.rs:64–105`；关联 `systems/time_update.rs:21–34`。

输入阶段不包含或约束 `time_update`。后者根据调用时的 `replay.current_frame()` 更新完整时钟；控制命令随后才可能进入或退出回放。回放应用阶段只覆盖 `GameTime.delta`，不会重算 `elapsed`、`total_secs`、`ticks_60`、`frac`。

反例：宿主先更新时间再运行输入（也满足当前 README 的宿主负责更新时间约定），实时步长 16 ms，本帧收到 Replay，首帧录制步长 100 ms。`time_update` 按实时步长累计 16 ms；Control 切换 Replay，RecordReplay 把 delta 改为 100 ms。消费者同一帧读到 `delta=0.1`，但累计时钟仅增加 0.016。Stop/Load/重定位也会令已经选取的时间源失效。把 `time_update` 放到 Finish 之后同样会提前读取下一帧。

建议：明确 `Control → 时间源选择/完整时钟更新 → Translate → RecordReplay → Consume → Finish`，每帧只更新一次完整时钟；不要用后续单字段覆盖补救错误时间源。

## R3 · [P2] 光标更新丢弃已经限幅的窗口位移

位置：`systems/backend/input/update_cursor_position.rs:104–107`；生产者 `translate_input_to_cursor_moved.rs:52–70,81–90`。

翻译阶段同时生成原始输入空间增量 `analog[0]` 和限幅后窗口位置差 `analog[1]`。最终系统始终把前者累加到 `CursorPositionSS`，只限制下界，没有窗口上界；捕获锚点也被直接忽略。

反例：窗口 100×100、起点 (99,50)、Relative(5,0)。翻译得到窗口位移 (1,0)，但最终位置为 (104,50)，而不是 (100,50)。此错误位置继续作为下一帧绝对事件的差分起点，并发送给屏幕光标消费者。捕获期间窗口位移应保持不变时，原始拖动增量也会错误移动屏幕光标。

建议：分别处理原始拖动增量和窗口游标位置。未捕获时应用限幅后的窗口位置差；捕获时遵循锚点策略。保留原始增量供相机旋转等动作使用。

## R4 · [P2] Consume 内部的派生状态与消费者仍然无序

位置：`systems/backend/input/mod.rs:98–104`；文档约定 `input/README.md:64–70`。

`update_cursor_position` 和 `update_mod_key_state` 都放入 Consume，文档又要求宿主游戏消费者加入同一个集合。SystemSet 只约束集合之间的顺序，不为集合内部系统建立依赖边。读 `CursorPositionSS`/`Modifiers` 的消费者与对应写入者发生资源冲突，只会被串行化，先后顺序没有保证；独立读取 `CursorMovedScreenSpace` 的事件消费者甚至没有资源冲突。

反例：按文档加入一个 Consume 系统读取本帧屏幕光标。调度图允许其在 `update_cursor_position` 前执行，读到旧坐标或错过本帧事件。即使 ActionSetStates 已正确回放，依赖这些派生输入的射线/工具仍可能落后一帧。当前回归的观察器只读取 ActionSetStates，未覆盖此依赖。

建议：将光标/修饰键派生计算放在独立阶段，并明确早于游戏 Consume；对需要本帧 UI/射线结果的系统继续显式建立依赖。

## R5 · [P2] 文件验证未检查可播放时间总量，合法加载后仍可 panic

位置：`input/recording.rs:55–62`；关联 `resources/game_clock.rs:128–132` 和 `systems/time_update.rs:33–34`。

每帧只检查能否单独转换为 Duration。含两个 `time_delta_seconds=1.0e19` 的小文件，每帧均可转换，保存和加载校验均接受；播放第二帧时 Duration 累加超过上限，返回 DurationOverflow，并被 `time_update` 转成 panic。这是异常文件的边界问题，普通录制时长不会触发，但文件入口当前并没有兑现“校验后可安全播放”的要求。

建议：限制合理的单帧步长，并在加载时检查时间累加；运行时仍将时钟错误转为回放停止及 last_error，避免文件输入触发进程崩溃。

## 验证记录与边界

- 本轮生产源码结论来自实际 producer/consumer 和调度源码检查，相关文件指纹见 `source-snapshot.json`。
- 已准备 `probes.rs`，包含上述反例及“只有 egui Context 资源但未开始 egui 帧”的额外接入探针。探针故意断言缺陷表现，测试成功也不能解释为修复通过。
- 本轮执行 `cargo test -p country-core --test input_rereview_probes --offline --locked -- --nocapture`，退出码 **101**。在构建 country-core 的 rlib 时遇到 **No space left on device (os error 28)**，未进入探针执行。完整输出为 `probes.log`；不能声称这些探针已经运行通过。
- 临时 Cargo 测试入口已经移除，报告保留可复用源码。磁盘空间恢复后，可将 `probes.rs` 复制为 `crates/country-core/tests/input_rereview_probes.rs` 再执行上述命令。未清理其他任务的缓存或构建产物。
- Egui Context 的帧生命周期由 GUI 宿主负责，实际 game_loop 在 App::update 前开始帧；本轮不把未开始帧的接入方式单独列为已确认生产缺陷。
- 未运行全游戏、GPU 或原版行为差分。CLI/crash 导出的独立宿主接线、完整世界初态和自动同步点对应的场景恢复仍在既有边界内，不能用本次 review 升级整游戏验收状态。
