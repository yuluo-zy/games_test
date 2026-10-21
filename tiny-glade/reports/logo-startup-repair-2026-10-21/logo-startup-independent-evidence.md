# Logo真实启动与错误退出：独立只读证据

基线1f73dbc73；不运行Cargo/游戏/GUI、不改生产。此文仅读现有源码与完整失败/成功日志。后续安装ZIP解压准备由中央单独授权，不是新测试。

## 1. 旧完整日志不能恢复被隐藏的首次致命Err

证据根：D:/game/tiny-glade/reports/main-loop-render-recovery-2026-10-19/latest-audit-fa53d49a-20261020/postpush-render-20261021。

tg-full-process-unbounded-v3-20261021.log:1的Crash reporter unavailable是main.rs:166附近明确log后继续的适配边界；并非本次Logo卡住首因。log:162的intro snapshot文件missing按load_game现有逻辑可回退历史，不直接构成致命退出证据。log:356最后观察4470 completed loading帧、0 gameplay；357开始panic backtrace；483–484才是render_frame.rs:303 device_ready断言的终态。此前没有一条主循环首次返回Err的完整error chain输出。

现源码 event_loop.rs:94–111发现first-error仅存Rc<RefCell<Option<Error>>>，首次分支不log；随后shut_down再target.exit。回调没有检查failure已存在/target.exiting/shutdown_done来跳过后续事件。AboutToWait:252–259仍request_redraw；RedrawRequested:123–126仍frame。winit请求exit之后仍可能派发已队列回调，故不能将target.exit当作同步终止。

GameLoop::shut_down game_loop.rs:951–1001会取消join worker、关save、清截图、renderer.shutdown；ProductionRenderer::shutdown production_renderer.rs:882–920最终closed=true；host.device_ready:1096–1103在未deviceLost时返回!closed。因此存在完全不依赖GPU驱动故障的可复现链：某frame Err→保存首错→关闭renderer→遗留Redraw→再次frame/render→!closed为false→panic覆盖首错。

这证明一个明确的错误退出二次绘制风险，不证明最初Err到底是什么。STOP-FREEZE报告所称device_ready failure只是最终症状；应先保证首错完整记录/退出后不再进入frame，再同一启动取证。不能通过删除原render断言或伪置device_ready掩盖。

## 2. 成功13帧与真正入口的差异

成功实际日志tg-sampler-monitor-gameplay-wsi-20261021.log尾14行有world_frame1..13与非空draw列表；末行为loading525/world13/native13及OS-close/save/GPU关闭完成。该测试复用真实GameLoop::new、异步GameplayPreparation、main_loop和双App切换（original_loading_present.rs:228–301），不是手工强制Logo完成。

- 真实bin/tiny-glade-host.rs:3–9在任何线程之前无条件设置MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS=1；original_loading_present测试main:27起没有同样设置。测试外部进程环境是否已设1无独立完整收据，所以只能列为待控制变量，不能直接推断MoltenVK根因。
- main.rs:124–138通过installation_root与override_save_root初始化paths并set_current_dir；测试original_loading_present.rs:202–211亦canonicalize TG_INSTALL_ROOT、临时账号、set_current_dir同安装目录。失败log:162可读到assets/starting-builds/full/intro/history路径，资源加载继续，不支持“完全没有安装资源”的假说。
- 真入口main.rs:139–160可由TG_FULLSCREEN/TG_WINDOW_SIZE改window config；run_game.rs:181先建EventLoop/setup_window，从物理尺寸建App。测试:228先以480×270建App，再用固定480×270 WindowBuilder，正常Gameplay模式不显式改config fullscreen值。应比较有效config、actual physical size、DPI与swapchain，不仅比较环境变量名字。
- main入口额外update_vulkan_allowlist_defaults、logging、panic hook、developer-hotkeys/lang override；测试仅get_app_config、local time和language(None,None)。目前没有证据证明这些就是首因。
- 两者均OS主线程render+async load工作线程，测试也真实warm_up_gameplay，故不能声称测试完全绕开worker。测试手工驱动main_loop，需要补run()的monitor枚举；过去tg-readback-policy-gameplay-wsi日志最后确曾因AvailableMonitors缺失失败。当前测试original_loading_present.rs:247–251已执行extract_available_monitors_info并插入owner；真正GameLoop::run event_loop.rs:93调用refresh_monitors，二者都具备这一步。

## 3. 不能据此认定MoltenVK

失败log:215起大量R32G32_UINT blending不支持警告；成功13帧日志末端同样有该警告且随后13次present完成。故警告并不足以解释完整入口失败。真入口强制Metal argument buffers、不同实际window/config以及可能隐藏的load/warmup error均需要同binary/同安装/同保存源的对照证伪。

旧tg-postpush-gameplay-wsi日志是另一次更早失败：async load的read_bush_positions_from_gpu slice_end_index_len_fail；tg-bush-header-gameplay-wsi明确count710191144/mapped_len9601。不能把那次已后续修复readback策略的错误倒填成unbounded-v3日志未记录的首次Err。

## 4. 中央可实施的窄验证顺序

1. 不扩展业务；先使run的first-error立即写完整chain，并使首次错误/正常shutdown后所有后续Redraw/AboutToWait不能再次调用frame、request_redraw或提交GPU；LoopExiting仍完成一次幂等清理并返回原错误。
2. 独立验证注入一次frame Err后又收到Redraw/AboutToWait/LoopExiting：frame不再执行、关闭只一次、完整原Err保留，不需GPU。
3. 同原资产、固定临时账号、同window480×270、无frame-limit、原双App/Logo路径启动真实host，记录最初Err。首错日志至少含effective installation/save root/config、OS线程与worker阶段、shader/binary identity，不能以最后device_ready代替。
4. 若仍有平台差异，再对入口与测试唯一差异做受控对照；不要先重写rhapsody、改原渲染算法或把未证实MoltenVK推测变成结论。
