# B 零尺寸帧生命周期裁决与窄补丁

结论：原作零宽或零高时跳过整个模拟/渲染帧，休眠 1ms，再进入公共 OS/show/settings 收尾。不能改成只跳过 render 而继续 App::update/MEI，也不能因 swapchain extent 不可用而把 loading 模式当成缺少 swapchain 的逻辑错误。

## 原指令闭包

1. `Resize` 回调消费者先在 `1404dacb8` 调用真正 `WindowSize::set`，再查 TypeId `a4c02661e67b048b/eae761e6aa2a67ec` 的 `FrameRenderConfig`；`1404dad4f/51` 将同一宽高写入 `extent_0`。当前宿主此前只写 WindowSize 与 GameConfig，漏了这两个 store。
2. OS 队列 drain 在 `1404dc857` 清理完成。这里调用 `140609210`，PDB 确认为 `drop_in_place<Vec::Drain<OsWindowEvent>>`，不是业务更新函数。
3. `1404dc863..dc8d5` 查同一 `FrameRenderConfig`、读取首个 packed extent 并保存至 `[rbp+1498]`。
4. `1404dc94b..dc960` 分别检查低/高 32 位为非零。二者均非零才在 `dc963` 跳到正常帧 `dc980`。
5. 任一为零时：`dc965 xor ecx,ecx; dc967 mov edx,0xf4240; dc96c call 140d55980; dc971 jmp 1404de84d`。PDB 明确 `140d55980 = std::thread::sleep`；参数是 `Duration { secs:0,nanos:1000000 }`，即 1ms。
6. `de84d` 为正常帧 FPS limiter 后的公共尾段；随后 `de8c5..de938` 检查设置 dirty 并回到外层事件循环。此跳转跨过所有分辨率重建、Egui begin `de115`、App::update `de349`、Egui end `de3d2`、render `de5bc`、temporal/frame counter `de63a`、FPS limiter `de7da`。

`original-evidence.json` 绑定完整原 RNE SHA、三个定点反汇编 artifact SHA 及两个原 LLVM PDB 符号；原完整 main-loop 36,589 字节证据继续复用。三段是原文件直接 LLVM objdump，不经过 patched Ghidra body。

## 候选

- `event_loop.rs`：Resize 在 WindowSize 更新后立即同步真实 `FrameRenderConfig.extent_0`，后续 GameConfig/WSI 更新不变。错误 Close 注释改为注明零尺寸等待，不再声称最小化必须模拟最后一帧。
- `game_loop.rs`：在现有准备完成/请求 flush/ShutdownGame 检查之后、Egui 原输入消费之前，从真实 `FrameRenderConfig.extent_0` 检查任一分量为零。该分支睡眠 1ms，仅执行现有 settings 尾段并返回 OS owner；不清输入边沿、不增加 host 或 temporal 帧计数、不 begin/end Egui、不 update App、不 flush MEI、不构图/提交、不调用 FPS limiter。
- 保留 `Suspended/Resumed` 已有整帧禁用；这两个事件是 winit 平台适配，原 B Windows 事件表没有对应分支，本批不宣称其原生行为等价。
- 新回归只验证实际门控 owner 及三个零分量组合/恢复后的非零情况；WindowSize 故意与 FrameRenderConfig 不同，防止再次把资源身份混同。完整 OS 原生窗口最小化/恢复运行待中央验收。

补丁 `candidate.patch` 的两个文件有新 base SHA，见本目录 `manifest.json`。第一批四文件 manifest 未覆盖。`git apply --check --ignore-space-change` 已返回 0；未写生产，未跑 Cargo。root 集成后应在同一窗口验证 loading/gameplay → zero resize → restore 两条链，以及 pending settings 在等待分支仍被处理。

## 留存边界

原作 Resize 对 `width>=20000` 直接忽略（现宿主保留），不能为“安全”增加不同的 height 限制。零尺寸分支仍处理外层事件/会话准备及请求，但不模拟游戏；它保留输入事件队列至后续真实帧。原 ShutdownGameCmd→resource 的最小化交互需要完整 OS 队列验收，不能为了即时退出在本窄补丁中偷跑一次 App.update。
