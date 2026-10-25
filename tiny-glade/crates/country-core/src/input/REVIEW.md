# 最新修复（2026-10-18）

D1–D3 已修复：按钮边沿复用 Bevy ButtonInput，宿主使用标准 press/release/clear；右键释放帧计入拖动距离；回退选择跳过末尾空同步点；捕获状态和 OS 命令通过统一入口更新，播放结束、Stop 或错误时释放，晚到的实时请求不能覆盖本帧回放策略。

验证：17 个输入流水线测试、2121 个核心单元测试全部通过。新增回归还验证了停止后恢复实时捕获。具体兼容性、验证边界和日志见 [输入边界修复报告](../../../../reports/ui-input/2026-10-18-input-edge-fixes/README.md)。

---

# 诊断记录（2026-10-18，修复前）

结论：**changes_required**。当前 12 个输入流水线回归通过，但三个新探针已复现：D1 真实宿主的释放标记清零了右键拖动距离，拖动松开误发点击；D2 末尾空同步点遮住已有帧，合法保存文件的默认加载/回放失败；D3 回放改写捕获资源时未同步操作系统捕获及显隐命令。

精确位置、实际事件序列、运行证据和验证边界见 [再次诊断报告](../../../../reports/ui-input/2026-10-18-input-diagnosis/README.md)。本轮仅诊断并更新记录，没有修改业务代码。

---

# 前次修复结果（2026-10-18）

下列复审的 R1–R5 已修复：使用实际 Toolbar/AppMode 快照和模式通知；Clock 阶段在命令后完整更新时间；窗口游标限幅并保留捕获锚点；Derive 明确早于 Consume；文件校验累计时间，运行时溢出安全停止并屏蔽当前帧。

核心验证通过：2105 个单元测试、12 个输入流水线测试；附加 Clippy/工具栏验证被工作区其他依赖与锁文件变更阻断。完整变更、兼容性及验证边界见 [五项修复报告](../../../../reports/ui-input/2026-10-18-input-fixes/README.md)。

---

# 重构后复审记录（2026-10-18，修复前）

该轮结论：**changes_required**。重新检查当时调用链发现：同步点没有恢复实际 `Toolbar`/`AppMode`；回放控制与完整时钟更新缺少顺序约束；光标使用原始增量导致越过窗口边界；Consume 内派生输入更新与游戏消费者无序；异常录制的累计时间仍可导致时钟 panic。

位置、反例和建议见 [本轮完整审查](../../../../reports/ui-input/2026-10-18-input-rereview/README.md)。本轮探针编译被磁盘空间不足阻断，未运行成功。以下为上一轮修复记录，不能解释为当前完整消费链已通过审查。

---

# 上一轮修复记录（2026-10-18）

用户授权优先保证功能性和正确性，并允许调整逆向设计。本轮已改为类型化状态、拥有数据的快照、完整文件协议和有序输入流水线。下文保留修复前审查，旧行号、代码形态和“当前缺口”描述不再用于判断新实现。

| 问题 | 当前修复 |
|---|---|
| F1 工具栏快照不拥有列表 | InputSnapshot 持有完整工具栏和类型化相机参数；恢复完整列表及索引；工具栏查询使用受检查 get |
| F2 加载后缺同步点/追加旧流 | 完整文件解析和校验后再整体替换；失败保留旧流；回退错误返回 Result |
| F3 动作丢失/无实际消费者 | InputFrame 直接保存固定动作状态；回放覆盖现有 ActionSetStates 和 ActionSetValues；禁用输入同样作用于真实动作 |
| F4 调度无顺序 | 明确 Prepare → Control → Translate → Resolve → RecordReplay → Consume → Finish；消费后推进索引 |

新增回归与工程边界见 [重构报告](../../../../reports/ui-input/2026-10-18-input-refactor/README.md)。本轮设计不再声明与旧原生布局/文件格式等价；旧缓存缺口探针只验证修复前快照，不可用于验收新源码。

---

# input 正确性审查

日期：2026-10-18。审查者：当前 Codex 主代理。范围：本目录 4 个 Rust 文件，扩展检查实际 backend 输入、同步点恢复、文件转换、调度和宿主注册。功能说明见 [README.md](README.md)。

结论：**changes_required**。局部动作谓词、稀疏打包、队列裁剪和模式比较未发现新的已确认算法错误；完整回放消费链仍有下列明确缺口。P1 是关键功能失效或崩溃，P2 是正确性/集成问题。它们不是对原版游戏的缺陷判断。本轮只新增文档和审查证据，没有修改业务源码。

## F1 · [P1] 同步点不拥有工具栏列表，seek 可能恢复出越界索引

位置：[input_stream.rs](input_stream.rs) 40–41、72–92 行；消费者 [seek_to_sync_point](../systems/backend/input/input_record_replay.rs) 273–281 行、[ToolbarCategoryModes::last_selected_mode](../resources/toolbar.rs) 75–82 行。

`from_sync_resources` 保存当前 `toolbar.modes` 的 capacity、地址、length，却没有克隆/拥有列表内容。整个状态和同步点都只做按值复制；seek 读取后只恢复 `current`，将 modes 头读出后丢弃。因而原快照中的列表既没有被保留，也没有恢复到消费者。

反例：保存 `modes=[Path, Water], current=1`；随后改成 `modes=[Path], current=0`；seek 将 current 写回 1，却保留只有一项的现列表。后续 `last_selected_mode` 对非空列表直接索引，触发越界 panic。即使长度没变，模式内容也可能错误。

原证据：[140949660-fresh.c.txt](../../recovery/agent-team/reports/interaction/input/evidence/logic-input-rebuild-2026-10-17/140949660-fresh.c.txt) 释放最老同步点时遍历并析构工具栏列表；[140941a30-fresh.c.txt](../../recovery/agent-team/reports/interaction/input/evidence/logic-input-rebuild-2026-10-17/140941a30-fresh.c.txt) 的 `clone<ToolbarCategoryModes>` 恢复路径表明该部分包含所有权行为。数值 Vec 头不能替代这个合同。

建议：同步点持有类型化、拥有列表的状态快照，并在 seek 恢复完整 `ToolbarCategoryModes`；将原生布局观察与运行时所有权模型分开。不要从保存的裸地址重新构造 Vec。目前没有解引用该地址，**本条不是现有 unsafe/内存 UB 报告**。

验证：探针 `gap_snapshot_does_not_restore_toolbar_modes` 执行上述操作并捕获消费者 panic。

## F2 · [P1] 成功加载后没有同步点，Load 控制流程仍必然 seek

位置：[try_load](../systems/backend/input/input_record_replay.rs) 218–222 行；[control_input_replay](../systems/backend/input/input_record_replay.rs) 570–590 行；seek 的空队列检查在 250–253 行。

`try_load` 解析 `RecordingFile` 后只向**现有流**追加一个 first_frame；它不消费 `replay_initial_state`，不建立同步点，也不清理旧流。调用方随后设置 Replay 并无条件 seek。

反例：默认状态下加载一个由当前 `RecordingFile` serializer 生成的合法文件。`try_load` 返回 Ok，帧数为 1，同步点仍为 0；随后与 Load 分支相同的 seek 调用 panic `No sync points found in replay state`。若旧流本来有同步点，seek 会使用旧录制的锚点/状态；再次加载则帧数从 1 增为 2，未整体替换。

因此现有 `try_load_reads_a_recording_file_into_the_stream` 只证明“读入首帧”，无法证明“文件可回放”。历史报告的 Load 控制流复核也不能覆盖加载器的产物合同。

建议：先确认完整文件/帧流格式，将初始状态、同步点和所有帧组装为新回放状态，成功后整体替换；seek 只能接受已构造完毕的回放状态。此处不能只取消空同步点 panic，因为那会继续播放缺失初始状态的数据。

验证：`gap_load_on_a_fresh_stream_cannot_seek` 使用临时合法 JSON，确认成功返回、空同步点、seek panic 及重复加载追加行为。未声称已兼容原作录制文件格式。

## F3 · [P1] 帧转换丢弃动作记录，回放缓冲没有写入真实动作消费者

位置：[InputFrameSerde 转换](../systems/backend/input/input_record_replay.rs) 132–164 行；录制/播放分支 431–506 行；[calculate_input_delta](../systems/backend/input/calculate_input_delta.rs) 87–108 行。

这里有两层未完成的 producer → consumer 合同：

- `InputFrameSerde -> InputFrame` 完全不读取 `main_action_set_value`，并把输出、游标和 divisor 置零；反向转换无条件输出空动作 Vec。因此即使文件带非空合法动作记录，当前转换也会将它永久丢掉。运行时非零光标/输出经该转换往返同样变零。
- 实际动作消费者读 `ActionSetStates`，由 `ActionSetValues` 推进；录制/回放系统仅操作另一份 `InputTranslationStateResource` 和 `InputReplayOutputState`，没有读写上述动作资源，也没有调用 `MainActionSet::{deconstruct,reconstruct}`。检查当前 core/宿主资源引用后，没有找到将回放块转交这些真实动作资源的桥。因而目录内打包接口存在，不代表播放文件能够重现编辑动作；`DisableInput` 同样只清理这份回放翻译资源。

建议：保留动作记录，并依据真实帧/schema 合同建立两种表示的映射；再接到现有唯一动作状态所有者，明确回放与实时设备输入的覆盖顺序。原证据对帧两种视图仍有布局歧义，不能把原始块直接强转成动作资源，也不能仅为了往返测试编造字段映射。

验证：`gap_frame_conversion_discards_packed_actions_and_cursor` 输入非空 camera_pan 动作，转换后记录为空；非零运行时光标/输出经 serde 视图往返变零。资源链缺口通过源码引用和系统参数核对，未运行完整游戏。

## F4 · [P2] CPU 注册 helper 未声明输入生产、计算和复位的先后顺序

位置：[add_input_cpu_consumers](../systems/backend/input/mod.rs) 60–79 行。

多个系统通过普通 tuple 注册，没有 `.chain()`、`.before/.after` 或有序 SystemSet；reset 和录制系统又独立注册。对资源的读写冲突只避免同时访问，不能表达生产者必须先于消费者。

`translate_input → calculate_input_delta → 消费者`、输入前后帧轮换，以及 `seek → advance → replay` 都依赖明确次序。例如 reset 若在设备写入与增量计算之间执行，计算看到的 current 就已归零；seek 到 frame_index=0 会得到 u64::MAX，若录制回放分支先于 advance，就在帧索引检查处 panic。

建议：按原调度证据建立阶段和依赖边，至少覆盖这些实际数据依赖；不能仅按 tuple 文本顺序宣称串行。当前真实宿主只单独注册了部分输入系统，本条限定在调用该 helper 的路径，不声称已在生产主循环观察到某一种错误顺序。

验证：`gap_cpu_schedule_has_no_ordering_edges` 使用锁定 Bevy 0.16.0 和绑定指纹的缓存 core 编译产物，初始化 helper 产生的 Schedule，确认 dependency graph 为零条顺序边。没有运行缺失外部资源的完整帧，也未据偶然执行顺序判定正确性。

## 文档及尚未证实的合同

以下不计作新增运行时缺陷：

- `input_stream.rs` 文件头第 11 行仍说裁剪目标是“被弹同步点索引”；正文 178–198 行和保留原证据是**新头**。README 已明确该差异；本轮保留已有业务文件不动。
- 文件头第 15–16 行“帧内容/try_load 未做”已不足以描述当前工程：后端已有部分类型和加载函数，但尚未满足完整合同。历史报告的“宿主待接”正是 F1/F3 当前仍存在的限制，不能作为消除问题的依据。
- `ActionSetAction::from_bytes` 接受所有非零 kind 为 digital，reconstruct 忽略越界槽；原反编译有坏指令段，尚不能断言对无效记录域与原作等价。需补原分支/格式合同，而非臆造拒绝策略。
- `MainActionSetSerde` 的未知键行为、重复字段策略由当前手写 visitor 决定；原字段名 visitor 的 unknown 哨兵本身不能证明外层 MapAccess 必须报错。未将其列为已证实偏差。
- `MainActionSet::just_pressed` 的整套原槽跳表仍未单独核对；现有按槽查状态不是对原跳表覆盖的证明。
- Rust enum/结构的实际布局不由注释中的原大小保证；`to_niche` 是显式转换，不是 Rust ABI 声明。
- `PanicReproState` 仅有零标签谓词；完整复现状态机、原版完整运行、原文件互通与整游戏调度仍未验收。

## 本轮验证与证据范围

复用 2026-10-15 算法文档、2026-10-17 的原反编译与重建记录，独立阅读了裁剪释放逻辑及其消费者。没有新开 MCP worker，没有修改数据库或发行文件，没有执行原版 runtime oracle。历史文件中的占位 `void()` 不作为真实 ABI 依据。

- `cargo test -p country-core --lib input:: --offline --locked`：46 passed，0 failed，0 ignored；覆盖本目录及 backend 输入测试。日志见 [input-tests.log](../../../../reports/ui-input/2026-10-18-input-review/input-tests.log)。
- [probes.rs](../../../../reports/ui-input/2026-10-18-input-review/probes.rs)：针对 F1–F4 的审查探针。首轮可执行探针复现 F1–F3，F4 的探针误读构建后已转移的 graph.systems；改用公开 Schedule::systems。最新 Cargo 重建被目录外 `render/sparse_instance_buffer.rs:425` 的 E0502 阻断，见 [probes.log](../../../../reports/ui-input/2026-10-18-input-review/probes.log)；此前另一次构建曾被 descriptor_table.rs 的 E0502 阻断。没有修改这些文件。
- 最终改用已编译缓存，通过 Cargo fingerprint 选择相同的直接依赖，以 `rustc --test` 编译探针：**4 passed，0 failed**，见 [cached-probes.log](../../../../reports/ui-input/2026-10-18-input-review/cached-probes.log) 和 [产物身份](../../../../reports/ui-input/2026-10-18-input-review/cached-artifacts.json)。这证明该编译快照中的四项缺口；不是最新完整源码的重建通过，也不是原版 oracle。**探针通过表示缺口被复现，不表示问题已修复**；它们不加入默认回归套件。
- 新鲜源码复现命令：在 workspace 根运行 `python3 reports/ui-input/2026-10-18-input-review/run_probes.py`。脚本独占创建临时 Cargo test target，结束后移除。原缓存复现用 `run_cached_probes.py`，先检查产物 SHA；缓存改变则拒绝运行。两种方式均不修改业务源码。
- 审查源码、依赖和原证据指纹见 [verification.json](../../../../reports/ui-input/2026-10-18-input-review/verification.json)。结论仅适用于该快照；源码变化后应重核受影响结论。

本轮未做修复，也没有将历史 `implemented_unverified` 升级为 `verified`。后续优先闭合工具栏快照所有权，再完成加载器/帧映射和真实动作消费接线，最后以有明确顺序的完整输入帧验证。
