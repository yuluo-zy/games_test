# input 再次诊断

日期：2026-10-18。审查者：当前 Codex 主代理。结论：**changes_required**。

本轮重新读取当前输入、录制队列、鼠标状态及真实 event_loop/game_loop 适配。没有修改生产源码。原有 12 个流水线回归全部通过；新增 3 个诊断探针全部复现缺陷。探针断言的是错误表现，不能将“3 passed”理解为生产行为正确。

## D1 · [P2] 右键拖动松开被误判为右键点击

位置：`crates/country-core/src/systems/backend/input/button_input.rs:64–66`、`translate_input.rs:350–357`；实际生产者 `crates/tiny-glade/src/event_loop.rs:422–428`。

MouseButtonInput 没有独立释放集合，用 `just_pressed && !pressed` 反推释放。为配合这个接口，宿主 Released 分支也向 just_pressed 插入按钮。翻译器先检查 just_pressed 并将右键累计移动清零，再检查 just_released；因此松开一帧必然先把拖动距离变成 0，然后满足 `<3` 的点击条件。

实际事件反例：右键按下 → 下一帧拖动 20 像素 → 松开（按宿主的 Released 分支更新状态）。结果 `digital[1] = JustPressed`，拖动结束多产生一次 right_click，可能触发以该动作执行的聚焦/取消/工具操作。原有左键按住/释放测试没有覆盖这种鼠标边沿表示。

建议复用 Bevy ButtonInput，或至少分离 pressed/just_pressed/just_released，宿主与核心统一使用 press/release 接口。清除拖动距离只发生在真实按下沿，不能在释放沿清除；再覆盖“长距离拖动无点击、小范围按放有点击”的测试。

## D2 · [P2] 末尾空同步点导致有内容的合法文件默认无法回放

位置：`input_record_replay.rs:87–96`；关联 `input_stream.rs:152–164,217`。

Sync 将当前帧数记为下一帧索引，结构校验允许索引等于帧数。但 seek 先从所有同步点的尾部选择 rewind=0，再要求选中索引严格小于帧数，没有跳过没有后续帧的尾同步点。

实际命令反例：StartRecording 并录制 1 帧 → 同一控制帧发送 Sync{record:true}、Stop → save_recording 成功 → load_recording 成功 → Load{rewind:0} 失败，错误为“同步点之后没有可回放的输入帧”。此时旧同步点仍可用，Replay{rewind:1} 能成功。面板只检查总帧数大于 0，也会允许用户点击默认回放。

建议 seek 从有后续帧的同步点中计数选择，或在停止/导出时统一剔除末尾空分段；文件验证、UI 可播放性和 seek 使用同一规则。不能让零长度尾段遮住已有录制。

## D3 · [P2] 回放恢复捕获锚点时遗漏操作系统副作用

位置：`input_record_replay.rs:225–228`；正常副作用入口 `resources/mouse_capture.rs:43–62`；宿主消费 `crates/tiny-glade/src/game_loop.rs:456–485`。

回放直接赋值 CursorCapturedPosition，没有发送 OsCursorCaptureCmd、OsCursorVisibilityCmd。正常捕获链除修改该资源，还必须捕获/释放窗口并维护鼠标显隐来源位。game_loop 自己持有 capture_anchor 等状态，只在处理对应 OS 命令时改变它们，不会自动观察核心资源的变化。

反例：宿主当前已捕获鼠标，开始回放一个未捕获的单帧录制。回放结束后内部捕获状态为 None，产生的 OS 捕获命令却为 []，没有 Release/Unsuppress；操作系统仍可能锁定或隐藏鼠标。反方向的状态切换同样缺少 Capture/Suppress。原有捕获回归只断言内部资源，未检查此命令链。

建议把捕获切换集中为一个同时更新资源、OS 捕获、锚点和显隐位的入口，实时控制与回放复用；定义并测试播放完成、Stop、错误停止后的释放/恢复策略。

## 验证

执行 `CARGO_INCREMENTAL=0 cargo test -p country-core --test input_pipeline --test input_diagnosis_probes --offline --locked -- --nocapture`，退出 **0**，原有回归 **12/12**，缺陷探针 **3/3**，日志见 [tests.log](tests.log)。上述探针运行未受磁盘或依赖锁文件阻塞；上轮 utils 的 Bevy 依赖已同步进锁文件。

[probes.rs](probes.rs) 保存反例源码。运行时通过临时 Cargo 测试入口 include，结束后已删除入口，不将“断言缺陷”的探针混入正常测试集。要重现可将 probes.rs 复制为 `crates/country-core/tests/input_diagnosis_probes.rs`，执行上述命令。当前读过的源码、命令及指纹记录见 verification.json。

Clippy 检查退出 0，country-core 有 225 条警告，其中输入相关的一条为 input_record_replay 的参数数量 8/7；日志见 [clippy.log](clippy.log)。工具栏附加编译随后因磁盘空间不足（os error 28）退出 101，未执行测试，见 [toolbar-tests.log](toolbar-tests.log) 和 verification.json。诊断范围内的源码指纹在验证后未变化。真实窗口/GPU与原生差分没有执行；D3 的运行证据止于核心资源与 OS 命令队列，窗口副作用依据真实宿主消费源码确认，未声称执行了真实鼠标锁定。
