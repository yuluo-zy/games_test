# 主循环与渲染独立验收

本记录基于 `bd8f74a5cfb3c9cc279efb1abd27695bb1bfb8bf` 的当前工作树。`STATIC_REVIEW_INPUTS.json` 是静态审查输入，不是最终编译冻结。当前未执行 Cargo、GPU、生产程序或原发行程序；root 是唯一运行者。用户三处 critters 文件的当前指纹与 SYNC-BASELINE 相符。

## 当前可达边界

`tiny-glade` 普通入口的 `AppSteamIntegration::try_new(|_| None)?` 在配置/App/frame 之前失败；`main_with_steam` 需要真实已初始化 SDK。`tiny-glade-host` 是显式本机适配入口，经过 `standalone_main → launch → run_game_thread → GameLoop::new`。不能用本机 host 成功声明 Steam 主链成功。

`MISSING_SOURCE_SNAPSHOT.json` 记录四个运行 crate 中缺失的 external mod 声明，保留 cfg 属性及文件指纹。用户已确认新文件留在另一机器，属于同步缺口；这不是作者未实现清单。等待 root 同步后按变更源重核，禁止据此从零另造模块。名字候选仅供定位，不能直接升级为当前源。

现有 `original_loading_present` 的真实边界是 loading-only App、三帧、WSI present、shutdown。`TG_SMOKE_FRAMES` 按总帧数退出，可能只经过 loading；`any_present_successful` 是跨 loading/gameplay 的累计布尔。二者均不能接受 gameplay。

## 有界完整宿主 fixture

不另造 App factory。fixture 为 `harness = false`，进程主线程创建真实 winit EventLoop/Window；初始化临时账号 paths、原版 default_settings/language/starting-build；调用真实 `run_game_thread`，再调用 `GameLoop::new`。默认保持 splash 以经过后台准备和发布；不移除系统、不插入 LoadingGameComplete、不设置 `TG_ALLOW_MINIMAL_WORLD_FRAME=1`。

fixture 可逐事件调用已公开 `GameLoop::main_loop(event, target)`，无需公开或直接调用私有 `frame`。01 提供只读 `frame_progress`，02 提供本帧真实 world/graph/submission/present 观察值。两端接口尚未冻结，fixture 尚未可执行；不能给它预填 PASS。

完成条件必须同时成立：

1. 原 `LoadingGameComplete` 生产者完成，真实 gameplay SubApp 暖机后发布；状态为 `AppLoadingState::Game`，有真实 `ActiveSession`。
2. 当前物理 App/World owner 的非空 drawable/实际 DrawLists、实例数据进入完整 world graph；不能以实体数量或天空单独替代游戏场景。
3. 至少连续三个 Game 帧具有本帧 world graph 的 graph/receipt/submission 身份以及对应 WSI present；提交、排队展示、GPU/展示完成分别记账，不能重复使用上一帧或 loading 的事实。
4. 至少一个 Game 帧保存最终 resolve 图像/读回，记录尺寸、format、sample count 与像素统计；基线视觉比较只接受相同输入和明确色彩空间，不把非黑图像等同原版等价。
5. 向 canonical `ShutdownGameCmd` 发送真实关闭请求，沿 `main_loop`/`LoopExiting` 清理；后台 worker 已结束，GPU 工作已等待完成。提前关闭、总帧上限、超时、loader error、graph error 均不能返回成功。

诊断上限独立于完成条件。先限制总时长与总事件/帧数防悬挂；只有全部完成条件通过才接受，达到上限本身返回失败。

## root 运行输入

现有但不足以接受完整链的命令：

```sh
TG_INSTALL_ROOT='/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade' RHAPSODY_REQUIRE_VULKAN=1 TG_SMOKE_FRAMES=120 cargo run -p tiny-glade --bin tiny-glade-host --locked -- --override-save-root /private/tmp/tg-fullgame-review --override-save-root-user-id 0
```

它可用于收集首失败，不能据 exit 0 接受 Game。临时根由 root 选取新目录；不使用用户真实存档。完整 fixture 接口冻结后，预期运行命令为 `cargo test -p tiny-glade --test original_gameplay_present --locked`，目标尚未加入 Cargo，当前不要当作可运行命令。Vulkan loader/ICD/validation 层环境由 root 沿本机已验证配置继承，不能以 SKIP 代替真实 GPU。

## 源与运行身份

每次验收记录 Cargo.lock、实际 feature/target/profile、所有受影响源及候选哈希、实际 executable 哈希、argv/environment、安装资产/会话输入、设备/驱动、日志、退出码。运行前后对冻结清单重新取指纹；source drift 使受影响结论待重验，不修改旧失败记录。

编译、CPU schedule、真实 GPU 图、WSI、像素、原版行为比较和性能分别报告。性能先记录无读回的正常 Game 稳态，再独立记录读回开销；报告测量设备、样本、预热、分位值，未测不写高性能结论。工具连接、文档、缺源数量、测试数量均不计业务完成率。

## 其它实际闭包

相机验收只覆盖 set9 reset/snapshot → set10 相机与射线 → terrain/input → GPU depth submission → 对应映射回读 → 下一次输入消费，核同一 buffer/owner 与跨帧时序。旧 read_mouse_depth 候选必须在同步新源后重核。

save/load/reload 验收使用真实 starting-build/当前 canonical NextSession、LoadGameManager 和 history/save writer，核 Reload→Game 的 OnExit 清理与 OnEnter 重建以及后续 world graph；不以存档往返替代渲染验收。

## 本轮方法记录

实际复用：已有 run_game_thread/GameLoop 平台入口、同 CoreRenderOwners 暖机、现有原证与旧候选。已避免重复生成 App factory、另做引擎和重扫完整 PDB。首失败先区分同步缺源、Steam backend、类型 feature、运行 producer、图编译、设备执行、WSI和像素；每次窄修只升级其实际消费者边界。
