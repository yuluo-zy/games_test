# UI / Startup 静态进度审计（2026-10-21）

基线：main f72938c81，仅读当前真实生产源与既有报告；未运行 Cargo、未重新测试、未读二进制、未改生产源。T13 排除。百分比为“原业务功能与完整调用链覆盖”的保守估计区间，不是函数数量完成率；尚无全模块统一需求分母，不能作为精确统计。置信度描述证据充分程度。

| 模块 | 保守业务恢复区间 | 置信度 | 已闭合 / 最大缺口 |
|---|---:|---|---|
| 启动→装配→主循环（非渲染合计） | 75–85% | 中低 | 实际 gameplay App、异步装配、Startup/OnEnter 真实资产 owner 和 set 分组已存在；缺完整 plugin tuple 原排序、部分加载进度更新及整游戏首会话→卸载→第二会话验收，不能宣布完成。 |
| 会话 meadow / grading 启动子链 | 85–95% | 中 | meadow 完整 placements→mesh/shader→drawable 及 grading LUT 常驻资产+每会话三槽配置均已接；完整加载重入、颜色自定义/最终画面仍依赖别的模块。旧“meadow/grading 未接”已不适用。 |
| 主/Free/Walk 相机 | 80–90% | 中 | 六 body、Dolly owners、set8 模式切换、set9 reset/snapshot/main/paused、set10 实际 terrain ray/捕获/depth；真实摄影按钮 producer 现已接。缺物理输入→UI→模式→碰撞→最终画面的联合验收、debug freecam save UI 仍 TODO。 |
| toolbar 核心 + 摄影工具栏 | 65–75% | 中 | full/demo 真实树、普通/摄影 owner、滑块13类、撤销布局、焦点/动画/声音/排序、OnEnter/set7/set8；完整导航、菜单遮挡 hide、鼠标抢占、demo policy 未接，物理 picker/signifier 仍边界。 |
| MEI / Taffy UI 框架 | 70–85% | 中低 | 真实 assets/font config、pre/post、prepare/layout/draw、focus/confirm、实际GPU上传桥已具备；原 load system schedule label 尚 OPEN，未实现 widget族、端到端多页面/导航/原输入命中验收不能由宿主注册推导。旧mod头“主体全未实现”也已过时。 |
| LUT gallery UI | 70–85% | 中低 | 真 Scroll metrics owner、demo gate、hover/click音效、写两份 editable formation、set36 Game 接线；测试直接seed clicked，只证明UI消费者，物理输入→缩略图→修改后实际画面尚未闭合。 |
| Escape 主菜单 producer | 75–90% | 中 | 原主/quit、键盘27、timelapse/replay/photo、save/profile/URL/shutdown、17slot 均落源 GUI39；Settings/New/Credits等下一页消费者须独立判定，不能把producer完成当整个菜单完成。T13两项按要求不计。 |
| Load 菜单 / 备份事务 | 65–80% | 中低 | 原15KB UI及 modal/backup/error/history/session owner候选已入真实set36；须复核非空会话/备份实际选择、错误/取消、异步handoff整链。 |
| Settings / language / Credits / TOD 等 GUI 次级页 | 10–30% | 中 | Settings 6页、language、TOD circle/GUI、control GUI仍大量todo；Credits只scroll/HTTP生命周期，credits::ui仍未恢复。Escape入口可打开这些状态不代表其消费者存在。 |
| T16 颜色自定义交互（system-color） | 20–35% | 高（缺口判断） | 15事件、set31链/replay40、typed icons/radial spawn/hide/部分set消费者有源；真实show producer仍旧不完整create，主color_menu状态机、wall/gate/decorator触发和roof customize仍stub。原208+15纯候选fixture不算生产恢复。 |
| 输入 / 录制回放 / 捕获 | 75–90% | 中 | 真实Toolbar+AppMode+camera快照已替代旧镜像，宿主set1/2/3/5、翻译/derive/record/modifier/cursor及OS捕获；真实窗口/设备/Steam及原生差分未验收。旧input-rereview R1已被当前源码修复，不能照抄旧缺陷表。 |
| onboarding / hints | 35–50% | 中 | orchestration/条件/冷却/部分位置/动画/音频有真实系统；hint_draw/render_hints空体，payload常量0 seam、WallId(0)、位置Y0等仍在；toolbar focus事件身份不一致造成抑制链断点。 |
| tg-text 独立库 | 85–95% | 中 | 工程重写后CPU17、严格MoltenVK GPU2/2九帧旧报告有凭据，atlas/owner/shape/布局完整；不代表当前Windows宿主和整游戏文本已验收，保留新架构。 |

## 当前静态接线依据（行号对应 f72938c81）

- tiny-glade/crates/tiny-glade/src/run_game.rs:97：原plugin tuple顺序OPEN；:404装ToolbarPlugin；:414 add_color；:478摄影链；:514相机资源，:523 reset，:533 snapshot/main/paused，:544 ray/capture/depth。
- run_game.rs:612 Load UI set36；:624 Escape GUI，:635 GUI set39；:662会话grading；:749 install_gameplay_device_input（set1、set5链）。
- tiny-glade/crates/tiny-glade/src/gameplay_startup.rs:401共享MEI；:405 LUT UI Game/set36；:571真实meadow；:601 initialize_grading_assets，非空假owner。
- tiny-glade/crates/tiny-glade/src/toolbar_plugin.rs:26 OnEnter工具栏；:31 set7；:48 set8；首部明确4大缺口（hide/nav/steal/demo）。
- tiny-glade/crates/tiny-glade/src/mei_host.rs:51 install；:65原load schedule label OPEN；:70 pre/set6；:75文本/Taffy/set44；:84 post/flush。
- tiny-glade/crates/tiny-glade/src/gui/gui_escape_menu.rs:60真实producer；:172 NewSession；:188 Settings(Audio)；:307真实SystemParam消费者。
- tiny-glade/crates/tiny-glade/src/gui/settings.rs:124及之后有7个TODO函数；language_menu.rs:11；gui_tod.rs:15/22；toolbar/src/menu/credits.rs:11明确credits::ui未恢复。
- tiny-glade/crates/systems/color/src/lib.rs:251真实App扩展；:278 add_systems；show_color_menu.rs:197明确case缺口；color_menu.rs:183主循环未实现；ui_color_wall_and_roof.rs:32、ui_color_decorator.rs:26、ui_color_gates.rs:21触发缺口。
- tiny-glade/crates/systems/onboarding/src/hint_display_system.rs:170 WallId0、:258/:277 Y0、:340/:353绘制空体；lib.rs:134 event_payload_byte_seam返回0。
- **确定接线缺口**：toolbar/src/ui_interaction/focus.rs:6/:87写core::resources::toolbar::ToolbarUiIsInFocus；onboarding/src/hint_orchestrator.rs:656定义本地同名ZST，:710读取本地事件；startup_construction_runtime.rs:112只是注册该本地事件，无桥。实体焦点事件不会自动跨类型到达提示抑制reader。

## 已有验证证据及边界

- progress-20261020/CENTRAL-REPORT.md：本轮364测试，toolbar87、decorator249、core17、save11。它不覆盖随后宿主photomode接线窄修；宿主普通check缺FMOD，metadata check随后critters bare CR失败。此次未重跑。
- evidence/round12/escape-ui/REPORT.md：4个original_escape_menu测试源码（真实MEI与非空模板）明确当时未编译/执行；不得当4项已通过。
- evidence/round12/color-menu/REPORT.md：原机器码208 primary +15 snapshot候选fixture；生产show/main仍缺，候选成功不能折算T16完成。
- evidence/round12/independent-ui-camera/REPORT.md：6body/set9的静态观察仍成立；其“未装ToolbarPlugin/无Photomode producer”已被本轮源码覆盖，不能继续列缺口。
- evidence/round12/color-startup/REPORT.md：真实LUT与grading生命周期已接；九native grading fixtures不覆盖非Halloween I/O/JSON与GPU，且当时GCC schema报告需由存档审计者核对最新合入状态。
- reports/input-edge-fixes-2026-10-18：17 pipeline +2121 corelib历史执行，hostcheck通过；不能声称当前源码全量重验。reports/gpu-text-refactor-2026-10-18：text17CPU/2严格GPU，不等于游戏窗口。
