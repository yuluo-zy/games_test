# 主链取证与实施记录

日期：2026-10-18。本轮使用现有分析数据库与 B 原指令；没有复用历史完成率或作者的验收标签。

> 下文记录较早阶段的实现和当时的验证结果，不能代表最新工作树。用户要求“先逆向原版，不要自己创造”后的重新取证、双 App、真实加载图、生产宿主和 GPU 验证记录，见 [原版优先复核](ORIGINAL-FIRST-REVIEW.md)。

## 原始身份

- B patched EXE：`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。
- B 发行 RNE：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 两个 SHA 均现场读取文件核验。只读 Ghidra 返回 executable_path 为该 B patched EXE，revision `1791719479858:b17d372d:3`。
- 没有修改原发行文件、共享数据库或其他人的源码。

## 经验内容已落实的纠错

### 反编译缺失不等于主链不存在

旧 `restorer-batch4-A/0x140593b70_run_game.c` 为零字节。重新获取后，该函数有实际函数体，但单次 Ghidra 返回截断。`run_game_thread` 的 fresh 请求则返回 nested decompile timeout；顶层 isError=false 不足以证明成功。

保留的 `restorer-tinyglade-E/0x140593b70_run_game.c` 和 `0x14059d450_run_game_thread.c` 非空，可用于导航，仍需逐段核对。旧合同把 `0x1405a3440` 当独立 run_game_thread 入口；fresh 返回实际 entry_point 为 `0x14059d450`，前者在该函数内部，不能作为第二个函数实现。

`GameLoop::main_loop` 的伪代码在间接跳转处提前返回，漏掉核心更新/渲染段。直接读取 RNE 指令确认：

| 原调用点 | 实际调用 |
|---|---|
| `0x1404de115` | egui Context::begin_frame |
| `0x1404de349` | Bevy App::update |
| `0x1404de3d2` | EguiBackend::end_frame |
| `0x1404de464` | 发送 OS 平台输出 |
| `0x1404de4f4` | 可选 Steam Client::run_callbacks |
| `0x1404de5bc` | render_loop::render |
| `0x1404de63a` | 对尚需确认身份的资源 `+0x108` 计数器自增 |
| `0x1404de640` | log_time_to_first_frame |
| `0x1404de7da` | FpsLimiter::frame |

计数器 owner 尚未确认，没有擅自把它映射到 GameClock/FrameConstants。

### egui 的返回和所有权接口错误

`EguiBackend::end_frame`（`0x1400dd910`）原 sret 只有 15 qword，是 `PlatformOutput`，不是包含纹理/shape/viewports 的 `FullOutput`。native 对旧 shapes 逐项 drop，接收新 shapes，再向 self 的纹理增量 append，最后将 platform output 交给 OS 线程。

此前 Rust 返回 FullOutput，还把纹理增量从后端 take 出。已修正：

- end_frame 返回 PlatformOutput；形状与纹理增量留在同一 EguiBackend。
- draw 从同一 Context 读取 pixels_per_point。
- draw 先上传 set、绘制图元，最后处理 free。fresh draw 的纹理 remove_entry 位于图元循环之后；此前先 free 再 draw 的顺序已纠正。
- 外部 EguiRenderer 仍是设备接口，完整原 GPU 参数和生产实现尚未恢复，不能将本次所有权修正称为 egui GPU 已运行。

### 从真实资源选择帧模式

`0x1404de4f9..0x1404de587` 测试资源是否存在，然后形成 `2 * exists` 模式字节。资源 TypeId 是 `bba451ee6a08a3ff/8d008d15febcf29d`。保留的 `App::insert_resource<MainCamera>` 消费证据同样包含该完整 ID；因此 owner 是 MainCamera。

当前 `frame_render_mode` 直接查询当前 Bevy World 的 MainCamera：不存在→LoadingScreen，存在→WorldGpuProfiled。不使用猜测的 AppLoadingState tag，也不由 fixture 手填帧模式。

### 菜单限帧规则

`0x1404de6b4..0x1404de7da`：读取当前 GameConfig.video.fps_limit；菜单存在、is_open 且 can_limit_frame_rate，且命令器没有 profiling 时，将正 FPS 限到 60，非正/NaN 改为 60；其余分支保留原配置值。缺失菜单与缺失命令器具有不同门控语义，当前实现保留。

## 当前新增源码

| 源码 | 新增或修正 | 验收范围 |
|---|---|---|
| `crates/tiny-glade/src/egui_backend.rs` | PlatformOutput 返回、纹理增量留存、绘制后释放 | fresh 静态数据流 + 实际 egui 生命周期测试；无 GPU 验收 |
| `crates/tiny-glade/src/game_loop.rs` | frame_render_mode / frame_target_fps / GameFrameDriver | 真实 App/World/egui 的候选帧驱动；仍需生产 RenderHostFactory 与 OS 主循环调用 |
| `crates/tiny-glade/src/event_loop.rs` | 同一锁定 State 的 on_window_event / handle_platform_output 转发 | egui-winit 0.27.2 的 OS 输入/输出 API 接面；完整 run_event_loop 未恢复 |

GameFrameDriver 在 App::update 之后才构造借用同一 World 的 RenderHost，避免先构造陈旧资源快照。begin/end GUI、平台输出、core render、首帧日志与限帧按已确认顺序调用。它不是原 0x1118-byte GameLoop 的完整 ABI，也未包含未确认的计数器、Steam callback、OS 事件处理、设备恢复或关闭链。

## 现场验证

- `cargo test -p tiny-glade --lib --locked --offline`：27 项通过，0 失败。
- `cargo check -p tiny-glade --all-targets --locked --offline`：通过。
- 新增 egui 测试确实创建纹理、提交使用该纹理的 mesh、释放 TextureHandle，验证 upload→draw→free 与平台文本输出；不是仅检查空队列。
- 新增限帧测试覆盖缺失/关闭/打开菜单、配置更新、禁用限帧、NaN/无穷，以及 profiling 规则。
- 新增帧模式测试覆盖同一 World 上相机资源的插入和移除。
- RNE 限帧数值块的 Unicorn 实际执行尝试退出 132，无诊断；保存失败记录，未宣称原指令差分通过。
- 无真实窗口/GPU/present 运行验收。原来的 water 编译问题仍在，不属于上述 tiny-glade 库内通过范围。

日志与输入 hash 见 `verification.json` 和 `logs/`。旧报告原样保留，新改动不继承旧源码 hash 的验收。

## 主链尚未完成的阻碍

以下不是可忽略的边缘功能，必须继续完成后才能称主链打通：

1. `main`、`run_game`、`run_game_thread` 仍为未实现入口；启动配置/路径/语言/窗口/设备/资产/插件需要完整生产装配。
2. 原 GameLoop 的资源所有权、OSWindowEvent/GameEvent 载荷、线程移交、设置应用、设备恢复与关闭路径尚未恢复。GameFrameDriver 没有替代这些缺口。
3. `GameRenderHostFactory` 当前没有生产实现，`RenderHost`、完整 `FramePasses` 和 WorldAccess 的同一 World/资产/图身份仍须接入现有 CoreRenderBridge。
4. core 的启动 helper 混合了启动与逐帧系统。必须依据原 PreStartup/OnEnter/OnExit/Update 注册，不可整体塞入 Update。
5. 原始函数体还需要处理 decompiler timeout、截断与多段跳转；不能直接依据旧合同的函数签名或标签实现。
6. 14 包审查中的 prefab 消费、墙几何产出、屋顶通知、颜色初始化、动物组件身份等问题仍需在真实消费者路径逐项关闭。

因此本轮结果是分析文档及主链部分实现，**不是完整游戏恢复交付**。
