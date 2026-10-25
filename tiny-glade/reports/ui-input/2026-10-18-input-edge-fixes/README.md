# 输入边沿、同步点和捕获链修复

日期：2026-10-18。对应 [再次诊断 D1–D3](../2026-10-18-input-diagnosis/README.md)。本轮修复业务源码和实际宿主适配，不声明原生等价或完整游戏验证。

| 问题 | 修改 | 回归 |
|---|---|---|
| D1 右键拖动释放误触点击 | 键鼠资源直接复用项目锁定的 bevy_input 0.16.0 ButtonInput，去掉自行维护的 HashSet 边沿协议；宿主用 press/release 更新，用 clear 清理帧边沿。右键按下时复位距离，按住和释放帧均累计移动 | 按住帧移动 20px、释放帧移动 20px、按下帧移动 20px 均不产生点击；总移动 2px 正常点击 |
| D2 末尾空同步点遮住可播放内容 | seek 的 rewind 只在有后续帧的同步点中计数；空尾点可以保留在流中，但不会成为默认播放目标 | 1 帧录制 → Sync+Stop → 保存 → Load(rewind=0) 成功播放；超出可播放点数量时仍返回错误 |
| D3 捕获资源与 OS 状态不同步 | CursorCaptureState 集中更新内部锚点、Capture/Release、Suppress/Unsuppress；实时 mouse_capture 与回放复用同一入口，重复锚点不重复发送命令 | 从已捕获状态播放未捕获帧会发送 Release/Unsuppress；反向切换发送 Capture/Suppress |

回放策略持续到本帧结束：宿主晚于 Consume 执行的 mouse_capture 会消费并忽略实时请求。回放正常结束、Stop 中止回放、非法帧或时钟错误会释放捕获；本帧晚到的 Capture 请求不能重新抓住鼠标。下一帧恢复实时请求处理。回归覆盖了最后一帧、手动停止、非法帧，以及停止后的新实时请求。

## 验证

- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test -p country-core --test input_pipeline --offline --locked`：**17 passed，退出 0**，含原有回归和本次新增用例。见 [pipeline-tests-retry.log](pipeline-tests-retry.log)。
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline --locked`：**2121 passed，退出 0**，含实际 mouse_capture 系统单元测试。见 [unit-tests.log](unit-tests.log)。
- 首次编译被其他正在更新的 startup/render/save 源码中的 5 个错误阻断；后续这些编译点恢复后重试成功。首次日志保留为 [pipeline-tests.log](pipeline-tests.log)，未修改相关业务代码。
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo check -p tiny-glade --lib --offline --locked`：**退出 0**，确认实际 event_loop/game_loop 调用方已适配，见 [host-check.log](host-check.log)。
- Clippy 首次因磁盘空间不足退出 101；空间恢复后重试 **退出 0**。country-core 有 217 条警告，输入模块、捕获模块和输入回归没有诊断，见 [clippy-retry.log](clippy-retry.log)。
- 本轮修改的核心 Rust 文件 rustfmt 和 git diff --check 通过；源码及依赖指纹在最终检查后保持不变，见 [verification.json](verification.json) 和 [source-snapshot.json](source-snapshot.json)。

测试模拟了真实宿主 PostUpdate 的 mouse_capture，检查资源和 OS 命令队列；没有运行真实窗口鼠标锁定、GPU或原生差分。此前原生报告及旧边沿探针不覆盖当前实现，保留为历史记录。

## 接口与接入

country-core 新增对工作区已有 bevy_input 0.16.0 的引用，没有引入其他版本。键鼠包装资源保持两种独立 ECS 身份；调用方改用 Bevy 的 press/release/clear/reset_all/get_* 接口，不能再直接操作边沿集合。实际 event_loop、game_loop 和核心测试均已迁移。

`register_input_events` 注册捕获策略和三个捕获相关事件。宿主在 PostUpdate 运行一次 mouse_capture，再应用 OsCursorCaptureCmd/OsCursorVisibilityCmd；不得直接改写 CursorCapturedPosition 来代替窗口副作用。Stop/错误/结束的释放经输入阶段执行，见 [模块 README](../../../crates/country-core/src/input/README.md)。文件格式保持版本 2。

## 构建空间

开始时磁盘仅余约 172 MiB。仅回收本任务此前日志中记录、且 lsof 确认未使用的 7 个测试可执行文件，合计 207,515,120 字节。清单见 [owned-test-artifact-cleanup.json](owned-test-artifact-cleanup.json)。源码、报告、依赖库和其他增量缓存未清理。后续构建限制为单任务且关闭本次增量构建。
