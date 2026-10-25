# 推送后的渲染推进与验收

上一轮核心业务与原证已提交并推送：`08ecee0011d9449ebe79dffcd2f9388c490d2f53`。本目录记录该提交之后的真实运行与新增实现；这些后续修改尚未包含在该提交中。

## 已通过的实际入口

| 入口 | 本机实际结果 | 证明边界 |
|---|---|---|
| 原始世界连续 30 帧 | 1/1 通过，52.56 秒；全部原断言及采样位置不变 | 原资源的缓存、资源回收、内存与像素收敛；不能外推长时间运行 |
| 真实 GameLoop 窗口 | loading 525 帧、非空世界 13 帧、游戏原生 present 成功 13 次 | 双 App 交接、同 World 渲染、OS 关闭、保存线程断开、GPU 退出；不等于编辑落盘和重启恢复 |
| 静态采样器目录 | 8/8 通过，1.69 秒；7 项真实 Vulkan descriptor、1 项纯错误分类 | 页缓存、预算回退与失效窗口；完整参数和世界执行另由世界验收证明 |
| 读回宿主与 bridge | 9/9 通过，2.03 秒，无 ignored/skip | 首次初值、真实 GPU 覆盖、错误不改内容、显式初值及既有桥接所有权 |
| 原方向变换 helper | 3/3 通过 | 原 607 字节算法；后续完整装饰物缓存仍须独立执行 |

世界稳态 live 为 3042..3048，跨度 6；同时峰值 3048，sampler 自 frame2 至 frame29 保持 72。设备内存 clean frame12→28 为 969166084→968647684 字节，原生内存块 9→9。退出时 submitted=completed=1696，pending=0，额外 completed drain 销毁 0 个对象，idle_waits 仍为 2。原 20→30 帧像素检查全部通过。未提高阈值、延后 steady 起点或修改统计值。

输入和结果分别绑定于 [SAMPLER-V3-EXECUTION-INPUTS.json](SAMPLER-V3-EXECUTION-INPUTS.json) 与 [ACTUAL-RESULTS-V2.json](ACTUAL-RESULTS-V2.json)。四个该版本实际二进制已另作本机 APFS 保留副本，路径和 SHA 在输入清单内。较早 9 项 bridge 的二进制在后续重建前没有单独保留，只能绑定运行前 SHA、实际 build JSON 和最终测试源 SHA；不以新二进制冒充旧输入。

## 解决的具体问题

窗口的 compile→drain→execute 顺序已接到 headless helper，保留新 compiled 的资源和回调。原来的大幅 live 跳变消失后，仍实际失败于 3021..3042，原跨度阈值 16 未变。分项和源码核对显示，主要是固定 8 页上惰性补齐同一批 sampler：52→70 后稳定，每页强租约缓存并非丢失退役。当前目录来自本 App 已成功安装管线的实际反射；每个实际资源组先完成原准入，再尽力建立额外静态目录。小容量、元数据或 descriptor 预算不足保留原按组路径；真实非预算错误继续传播。同名实际 owner 的版本、App 隔离和 invalidate 的最终同锁失效均有实际回归。该分页策略是对现有重构架构的工程优化，不称为原作的分页算法。

真实窗口此前在异步 App 第一次更新就因灌木头 count 越界退出；诊断值为 710191144，映射长度为 9601。原 B 的构造 None 路径确无初值上传，原 reader 的检查应保留。Vulkan 的新分配在未请求相应零初始化时，其初值没有定义，见 [Khronos 内存模型](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html#memory-model-allocation)。当前只在游戏 BufferHost 新创建、GpuToCpu、None 且尚未发布句柄时，用原主机写入事务定义零初值；原 canonical 构造仍传 None，generic rhapsody create_buffer、Some、已有映射和 GPU 已写结果均不重置。这是显式可移植性策略，具有一次 O(size) 写入及可能的预算/flush 错误成本，不是发现了原作零填充证据。

直接驱动 main_loop 的验收最初遗漏了生产 run() 的监视器枚举前置步骤，导致交接缺 AvailableMonitors。验收现调用同一真实 OS 枚举 helper，保留交接的 required 检查，不使用伪监视器。新增读回测试也曾漏 host-read export，上传被图正确裁剪，实际读到 A5 而失败；补真实 consumer 后才通过。首次 A5 失败、export 错误类型的编译失败及所有更早运行失败均保留。

## 正在处理的剩余工作

性能还没有完成。以上为 debug 构建，不能据此承诺最终 FPS。已审录制探针正在实际编译，随后用优化构建分别测量声明、编译、录制、提交、等待及实际 GPU 时间。并行 pass 的 inclusive 经过时间不能相加当作帧墙钟；开探针运行用于定位，关闭探针的运行才作性能基准。

画面链继续按真实 producer 和 consumer 闭合：完整 9331 字节装饰物缓存、楼梯共享写域与完整网格、原 structural graph、哥特窗完整 producer、canonical Nani 更新、set10/set25 顺序及卸载。已审源码不等于实际编译或场景可见，前提未齐时不挂旧宿主。

原始 33 份 history 的全字段语料验收仍有独立失败：缺失 prefab_name 的原兼容门已有修复，但后续遇 f32 数字表示差异，正在按原序列化链核对，原 payload 断言和 33/2120/408/63 数量门均未改。编辑、实际落盘、跨进程重启恢复和长时间运行仍分别待验。


### 2026-10-21 用户实际入口反馈后的验收边界

用户反馈当前只能看到初始页面。此前真实 GameLoop loading→world 13 次原生 present，仅证明双 App 交接、ActiveSession/墙/非空 drawlist 以及正常关闭保存 worker/GPU 可运行；测试没有执行初始页真实点击→新建/继续→可编辑状态，不能将该结果表述为完整进入并游玩。

本次直接查现源，`tiny-glade/src/gui/gui_escape_menu.rs` 的 `Menu.New` 只设置 `EscMenuState::NewSession`，同函数该状态走 `_ => {}`；`systems/toolbar/src/menu/mod.rs` 没有新建/欢迎页面模块。全 Rust 源 `WelcomeScreenState` 仅见其它函数的 ICF 注释。现存 load_ui 确有 `NextSession::from_parsed`→Reload→Game 的真实来源，不能一概说所有菜单都无实现。新建/欢迎的原版模块、完整入口与消费者正独立取证，未用自动跳场景或手工设置菜单关闭作为修复。

当前最高优先仍为渲染尾部和窗口生命周期。挂起后关闭原负例实际失败、修复后与普通加载中关闭均在真实 GPU 窗口通过；具体证据与仍待补齐分支见 `TAIL-WINDOW-ACTUAL-RESULTS-V1.json`。新建/欢迎、完整建筑与装饰物宿主、编辑→保存→重启恢复尚未整体验收。


用户随后确认是“厂商 Logo 或加载画面一直不退出”，因此当前即时断点应先沿 `GameplayPreparation` 的 LoadingGameComplete→warm-up→channel publish→Logo.finish_wait→SubApp handoff 追踪；新建/欢迎菜单缺口不是这次卡 Logo 的直接原因。已请求实际启动命令/程序路径，并构建真实 `tiny-glade-host` 进程入口复测，不用三帧 loading-only 验收冒充正常游戏启动。
