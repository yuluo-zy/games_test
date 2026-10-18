# input 五项复审问题修复

日期：2026-10-18。范围：当前 Rust 输入模块及其工具栏、时钟、光标消费链。依据 [重新审查](../input-rereview-2026-10-18/README.md) 修复 R1–R5；使用功能正确性标准，不声明原生 B/C 等价或完整游戏验收。

| 问题 | 本次修改 | 验证 |
|---|---|---|
| R1 快照未恢复实际工具状态 | 快照拥有实际 Toolbar 的所有类别及索引、AppMode 和相机参数；恢复前校验，Control 复用 AppModeChanged/PreviousAppMode 通知链；去掉嵌套类别类型的 Resource 身份 | 文件回放恢复实际 Path/Toolbar；独立恢复验证所有权；Water → Path 仅发一次变更事件 |
| R2 控制命令与时钟脱节 | 明确 Control → Clock → Translate；Clock 每帧更新完整 GameTime 一次；回放应用阶段不再改写 delta | 首帧 100 ms、末帧 200 ms、结束返回实时及中途 Stop 均比较完整时钟结构 |
| R3 光标越界与捕获漂移 | 原始增量供动作使用，窗口坐标应用 analog[1] 并限幅；绝对输入同样限幅，捕获期间保持锚点；帧文件保存捕获锚点，回放覆盖实时捕获状态 | 99+5 限制为 100；越界绝对输入、捕获拖动、相反实时锚点下的回放 |
| R4 派生状态与消费者无序 | 新增 Derive 阶段，光标及修饰键明确早于 Consume | 开启 Schedule ambiguity_detection=Error，普通 Consume 观察系统无歧义，读到本帧游标和修饰键 |
| R5 累计时间溢出仍会 panic | 文件入口 checked_add 验证整段 Duration；运行时先更新时钟副本，失败保留累计状态、清零本帧步长、停止回放、保留录制并记录 last_error；屏蔽本帧动作 | 两个单独合法的 1e19 秒帧不能保存或加载；Duration::MAX 下回放无 panic/半更新/实时点击泄漏；非法内存帧也安全停止 |

## 兼容性与宿主接入

- 文件格式升级为 **2**。版本 1 缺少实际工具状态和捕获锚点，明确拒绝；不伪造兼容。
- `InputSnapshot::capture/restore`、`InputRecordReplayState::seek/load` 使用实际 `Toolbar` 和 `AppMode`。原先独立 `ToolbarCategoryModes` ECS 镜像不再使用；它仍是 Toolbar 内的普通数据类型。
- 完整阶段为 Prepare → Control → Clock → Translate → Resolve → RecordReplay → Derive → Consume → Finish。调用 `add_input_cpu_consumers` 后，不要另外重复注册 time_update。
- 宿主每帧维护 `bevy_time::Time`、按钮边沿、事件生命周期和 egui 帧；提供实际 Toolbar/AppMode、MainCamera、EscMenuState、ShaderRefactoringState、WindowSize 和光标资源。详见 [模块 README](../../crates/country-core/src/input/README.md)。
- 输入快照仍不包含完整世界和绝对游戏时钟初态。宿主准备对应场景初态；本次保证本帧时间源与完整时钟增量一致，没有将输入录制扩展成完整世界回滚系统。

## 验证结果

`CARGO_INCREMENTAL=0 cargo test -p country-core --lib --test input_pipeline --offline --locked`：**退出 0；2105 个单元测试、12 个输入流水线测试全部通过**，见 [tests.log](tests.log)。关闭本次增量构建以减少磁盘开销，没有清理其他任务的缓存。

本次修改文件 rustfmt 和 git diff --check 均通过。没有新增依赖。

随后执行 Clippy 时，工作区其他变更让 `utils/src/camera.rs` 引用了尚未声明的 bevy_ecs，构建退出 101，未进入本模块诊断。之后该依赖已加入 utils/Cargo.toml，但 Cargo.lock 未同步；工具栏附加测试两次均被 --locked 拒绝，尚未执行。这些结果分别保留于 [clippy.log](clippy.log)、[toolbar-tests.log](toolbar-tests.log)、[toolbar-tests-retry.log](toolbar-tests-retry.log)。没有修改或回滚其他任务的 manifest/lock。此前完成的核心测试通过，不代表当前新增依赖状态下的整工作区重新构建已通过。

机器可读结果与当前源码指纹见 [verification.json](verification.json)。旧审查探针仅为修复前反例，不作为当前测试入口。未运行全游戏/GPU或原版差分。
