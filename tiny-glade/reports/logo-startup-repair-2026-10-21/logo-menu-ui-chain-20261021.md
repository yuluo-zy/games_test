# Logo 后菜单 / MEI 输出链只读定位

2026-10-21。只读生产源码与保留原证，没有改生产源、跑 Cargo 或启动 GUI。

## 确认的产品缺口与不能推出的结论

生产 `toolbar/src/menu/mod.rs` 无 welcome/new_session/daily_welcome；canonical
`country-core/resources/onboarding.rs` 无 WelcomeScreenState；完整 MenuFlowPlugin
未安装。已经有准确原证合同：
`reverse-engineering/tiny-glade/skeleton/task-index/work/TG-700584f34568812faf/decorator-menu-entry-20261021/MENU_ENTRY_HARD_GAPS_CONTRACT_V1.md`。
该合同的原范围 byteproof 可复用，旧 patched-C/placeholder void() 不可直接翻译。

这解释了欢迎面板缺失，以及 Main.New 切到 tag5 后没有 new_session 页消费者。
**不能解释当前所有 UI（包括版本/settings）均不可见。**

- `run_game.rs` 已挂 GUI39 的 do_gui_escape_menu，并在它之后推进 NextEsc。
- EscMenuState 默认 Closed。do_gui_escape_menu 的 Game gate 只传给按 Esc 的
  opening/closing 逻辑，不屏蔽全部构图。
- build_escape_menu 在非摄影时总创建版本 label；Closed 时还创建原
  menu/back_w_bug_report 模板及 settings pill。缺 welcome 不能令这两者自动消失。
- SessionFlowPlugin 自动请求最近会话；空账号选 intro，不是先打开 Esc Main。
  Handoff 移六owner、Started、Vermeer、Egui，没有强制 Esc Main 的原逻辑。
  不应以强制 Main、伪造点击或替换背景菜单掩盖当前空输出。

## 首次欢迎的真实最小完整恢复范围

可单独恢复 welcome，而不先做完整 New/Daily/Settings 页面：

1. canonical WelcomeScreenState 的原三态及 timer，MenuFlow 原初始化值0；
2. 原10参数 welcome::ui，按 in_state(Game)/set36 接现有同一 MeiContext；
3. skip_welcome（当前 DevSettings 默认 false）及真实 GameProfile +206 的门；
4. Closed 菜单下0→1、Vermeer 活跃复位timer、原0.3秒 smoothstep淡入；
5. full/demo真实模板与桌面/SteamDeck两套完整slot，保原确认键19、真实文本，
   确认后写canonical profile +206、真实SaveGameProfileCmd、Welcome Closed；
6. 菜单在欢迎期间改变时的原关闭副作用/Local SteamdeckWelcomeMsg；
7. 使用完整真实模板/字体的实际 MEI prepare/layout/draw→flush 验证，而不只数节点。

现 GameProfile 默认 tutorial=false、+206=false。tutorial 是+1f4，不等于+206；
不能拿 tutorial bool 替代原 welcome 完成字节。+206当前字段名
hint_reset_flag_206 是历史临时语义，当前 serde skip；须窄补命名/合同而不造镜像。
原typed签名只有SaveGameProfile writer，旧分析声称另写WriteSettings不可信。

完整 New 页面另需 CurrentDaily、owned NextSession创建载荷、demo限额与音乐/profile
副作用，不属于修复版本/settings现有输出的最小范围。

## GUI39 → 实际 MEI 像素链

| 阶段 | 当前真实生产源码 | 结论 |
|---|---|---|
| 准备框架 | gameplay_startup::install_gameplay_gpu_startup → mei_host::install_mei_frame_systems；PreStartup attach_loading_mei从canonical TextPrim/ShapeStream owner构建 | 已接，非假空buf |
| 本帧MEI起点 | mei_pre_update set6处理上一帧输入→recycle_frame_nodes交换current/previous | 已接 |
| UI producer | do_gui_escape_menu GUI39创建原模板、版本label/settings；Load/LUT为set36独立producer | 已接既有页面 |
| 本帧draw | mei_host set44：prepare_text_fragments→Taffy final/layout/drawables→debug_hide_ui→render_text_fragments；之后mei_post_update→flush_mei_frame chain | 已接 |
| MEI节点树 | mei_post_update content.prepare_layout→perform_layout→z排序→NodeDrawContext(opacity/pos/size)→content.draw写真实TextRenderer | 无发现全局默认opacity0；Node::new opacity/local_opacity均1 |
| stream/atlas | flush_mei_frame → MeiHost::prepare(renderer.consume→upload_batch→renderer.render) 保存last_output含prim_count/atlas owner | 此处若无producer是命名错误，不能默认count0冒成功 |
| 图import | world_postprocess::prepare_mei从本World LoadingGpuResources取output；import_into_frame绑定同buffer hash；绑定blue noise+真实glyph atlas | 已接 |
| T17 | mei_pass_with_shadows写MSAA或temp→shadow combine；6*prim_count draw | 已接，但受T10 reveal门 |
| 版本/工具栏区别 | 版本/settings为MEI；工具栏为ECS Ui2dImage/Extent，collect_ui_2d后T16 | 两者都消失要优先查共同gate/target，而非一个producer |

debug_hide_ui默认show_2d_ui=true，且仅zero UiMouseInteractable 的 ECS extent，
不清MEI文字树。当前已读源码未见名为 MeiOverlay 的 canonical资源；不要另造
“显示菜单”布尔值或用UiState.replace_interaction伪造真实命中。

## 最强共同门：真实 Vermeer reveal

world_postprocess::build_world_tail 的 T7 运行原 vermeer_pass，T10 明确：
`ui_visible = !vermeer_active || photomode`。T16 UI2D和T17 MEI都在此门内。
Loading→gameplay 转移的是活跃 VermeerState，into_disable_hold只清hold。
因此 Logo 消失后还应先走真实世界笔触 reveal，再出现正常UI/欢迎。

原 vermeer_pass：前2帧不推进；随后按 `min(Time.delta_secs,1/30)/3` 累加；
只有progress>=1且非freeze/hold才Disabled并发StartupRevealFinished。普通完成至少
约92个实际render帧。若重资源导致每帧很慢，按原上限累计会延长现实等待；若
Time.delta_secs为0或hold未被真实清除，UI可以长期不出现。不得直接跳这条原门。

headless world capture在world_postprocess T4（overlay前）的正常草/树/墙，不能证明
T7之后及T10/T17输出可见。当前没有实际帧状态证明究竟是这一门、空primitive还是
MSAA合成问题，因此尚无“UI根因修复”结论。

下一次真实帧只需有界观察同一帧：Esc tag、PhotomodeScreen、Vermeer phase/hold/
freeze/frame_count/progress、Time.delta、MeiContext nodes/frame_events、实际
MeiHost.output prim_count，以及T10 ui_visible。分类：

- nodes/prim_count非零而ui_visible=false：先核reveal推进/资源异步帧耗时；
- nodes非零但prim_count=0：核layout/content.prepare/draw/opacity；
- nodes为空：核GUI39调度/实际World与producer；
- ui_visible=true且primitive非零仍无像素：核T17 bindings/attachment/load/blend/export，
  不能加Welcome面板替代GPU故障定位。

## 异步边界

gameplay完整资产/会话加载已经随同一个SubApp移到async loader，主线程保loading
App；成功warmup后才接回该SubApp，不能重跑Startup。loading App自身的common
bootup纹理/字体/UI配置仍在自己的Startup同步做，glyph新上传仍走现有设备
with_setup_command；这可能是资源准备/帧等待的排查面，但尚无耗时证据。
应保原资源owner与依赖关系拆实际异步准备，不能以空纹理/假ready、删图或全局
换Resource掩盖等待。此文未改该路径。
