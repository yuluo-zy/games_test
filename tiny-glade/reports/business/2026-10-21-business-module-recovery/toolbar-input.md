# Toolbar input recovery — 2026-10-21

## Menu visibility / focus ownership

`ui_interaction::hide_ui` now restores original B `0x14233eb50` (894 bytes). Static extraction verifies original SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; exact body hashes and assembly are in `toolbar-input-evidence/manifest.json`. No original process was executed. Existing PDB-derived seven-SystemParam signature is `reverse-engineering/tiny-glade/evidence/core-recovery-20261015/gui/hide-ui-parameters.json`.

The second folded predicate at `0x1409a9140` receives the fourth parameter (`WelcomeScreenState`) and tests Closed. Historical pseudo-C names it `UiFocus::is_mei`; following that alias would wrongly hide/unhide the toolbar from MEI focus rather than the Welcome lifecycle. Original `0x14233eb85` loads the fourth parameter through saved R9.

All three global gates are preserved: Esc closed, Welcome closed, Steam Deck screenshot message inactive. Ordinary toolbar entries are visible outside photomode; photo-only entries require photomode enabled and `hide_ui` false. Only `HideIfMenuIsOpen` entities participate. Hidden entries invalidate matching current and staged Toolbar focus; an open menu also clears the current Toolbar focus even if its entity is absent from this query. Inactive focus payload remains unchanged, as in native qword-only discriminant stores. Other focus variants and interaction keep/selected bytes remain intact.

`SteamdeckScreenshotMsgScreen` was absent from production. Exact original App insertion caller `0x14059f3d9` supplies EDX zero (`0x14059f3d7`); `App::insert_resource` at `0x14011d170` copies BL and has one-byte descriptor size/alignment. Business central owns the canonical core resource and host initialization. The original photomode screenshot-message writer remains a separate open path; default false is the observed startup value, not proof that the Steam Deck screenshot flow is restored.

Production files: `crates/systems/toolbar/src/ui_interaction.rs`, `ui_interaction/hide.rs`, `ui_interaction/hide_tests.rs`. Original registration order requires this system before `render_taffy_ui_to_widgets` in Update/set7 (`progress-20261020/phase2-toolbar-layout/schedule-source-map.json`). Host registration is central-owned.

Validation status: three regression tests written, not yet executed. They cover 32 gate combinations, focus tags/retained payloads, and close/despawn/reenter lifecycle. They do not seed clicks and do not claim physical-input, undo, persistence, GPU or full-module acceptance. Central test command: `cargo test -p toolbar --lib ui_interaction::hide::tests --locked --offline -- --nocapture`.

## Highest remaining input blocker

The current production `UiState` has no actual hover picker / input arbitration writer. Existing round12 `ui-picker`, `ui-update`, and `roof-ui` evidence contains bounded native recovery and candidate code; it must be reconciled against present APIs and independently verified, rather than rewritten as a toolbar-only AABB/click shortcut. Business central owns geometry/hover/state resource integration; this input worker owns `ui_update` state machine/event side effects. Current toolbar ScreenAabb production also drops `Ui2dExtent.pos`; restoring full WidgetGeometry must fix that producer before input chain acceptance.

T13 feedback capture is excluded throughout. No rhapsody changes, Cargo, GPU, or application launch were performed by this worker.

## UI interaction owner restoration in progress

The complete existing `round12/ui-update/ui_update.rs` candidate has been reconciled into the new production file `country-core/src/systems/ui/ui_update.rs`, pending central owner/type and schedule integration. This restores all native interaction phases (idle, hover, started, continuing, finished, cancelled), current versus captured widget ownership, last-focus-event semantics, Escape/shutdown termination, brush-preview geometry origins, cursor suppression origin1, all four complete sound-slot payloads, and end-of-frame registration clearing. `get_ui_volume` remains the native registry-Arc getter with missing-widget panic; it is not a replacement picker. Continuing drag keeps its original intersection and refreshes only its parameters, retaining the captured Arc when the current registration disappears.

The current original image was independently rehashed and the original `ui_update`7577B body SHA matched `78070209e56873b445bcecd65c7b1ca67fe64a8e82065a3beff6b3b264c5b5c2`; `get_ui_volume`503B matched `b0bd8122e4c3ccc3a8f30e201794c1b784f8ffdea8efd624bb7b6ab8e9adf955`. Original instructions were reread at the hover owner replacement, event cancellation/release, drag Arc retention and brush/cursor branches. See the new static assembly alongside the earlier round12 native fixtures. No new original execution was performed.

New tests compare production Rust against the frozen 600 interaction and 150 preview/owner output fixtures, including valid state, Arc identity, complete24B sound payload, brush geometry origin, cursor source and registry clearing. These are explicit state-machine boundary tests, not physical-input acceptance. Three separate tests drive the real picker plus action lifecycle into UI interaction and history recording without seeded click/drag results. All tests are currently unexecuted; formatting/parser check succeeded. Old oracle fixtures bridged standard library/Bevy storage operations and the registry getter, so their acceptance scope is the native business state machine, not the entire original game.

The toolbar geometry producer has three further statically confirmed defects pending its central API migration: screen center is dropped, native `UiMouseInteractable` f32 distance is dropped, and world occlusion is incorrectly enabled. At original `0x1423395cb` distance is loaded from the component; `0x1423395e6` writes center±half-size; `0x142339637` passes false to occlude_by_world. `interaction.state` is not the ScreenAabb depth or a layer. A full real-system geometry regression is prepared in `widget_geometry_tests.rs`.

## Replay-specific denied-input branch

The original non-history branch at `0x1423397b7` compares AppMode with **13/Replay**, not Photomode19. The historical implementation both misnamed and inverted this condition. In Replay, an active ordinary toolbar item emits the denied-input marker; both active and inactive paths then jump directly to the next query item (`0x142338f48`), skipping focus/click/drag emission. History buttons retain their separate branch and remain usable. Ordinary tools and Photomode retain normal click handling and do not emit a denied marker.

The complete604B original closure at `0x142339f10` builds `SignifierMesh::Empty` with animation_duration0, then adds `Ui2dExtent{cursor,size,size}`, `Ui2dImage("icon_cross",layer0)` and `Ui2DFollowMouse`, returning one Entity. Original name bytes `0x14304dd58/18` are `deny_input_toolbar`. The two leading qwords in the original allocation are the Arc header, not a fabricated signifier tag/kind. New `ui_interaction/replay.rs` uses the current `SignifierFactory::from_closure` owner and existing builder, preserving the user-rewritten architecture.

These producer/Replay changes and five revised `tb_e_render_taffy` tests are staged in the report's `candidate-ui_interaction.rs` and `candidate-tb_e_render_taffy.rs` while root's release build freezes exported sources. They are not yet installed or tested. The original branch/604B closure was independently crosschecked by business central's audit worker. Tests previously asserted the incorrect Path signifier/raw payload and `occlude=true`; those assertions are replaced by explicit original contracts rather than weakened.

## 物理按钮资源到工具栏模式：未挂载工程验证候选

`candidate-toolbar-physical-chain.rs` 已准备，目标为 `crates/tiny-glade/tests/toolbar_physical_chain.rs`，当前只在报告目录；未编译、未运行。复用现 `country-core/tests/input_pipeline.rs` 的 `register_input_events` 与 `add_input_cpu_consumers`，从 canonical `MouseButtonInput.press(0)/release(0)` 和 `InputCursorMoved::Absolute` 起步，未手工写动作 phase、UiInteraction、UiClickEvent 或 RequestAppModeChange。

- 正向：空帧预热真实 toolbar producer，下一帧真实 hover→ui_update 取得注册；press→hold→release 后真实 toolbar/button consumer 发送一次模式请求，实际 mode_manager 同步 AppMode 与 ToolbarCategory。
- 反向：移出后释放取消；捕获时删除按钮，即使前帧 geometry 尚存也不能产生已删除实体的点击/模式请求，下一帧 active owner 释放。
- 阶段：input translate→resolve→record→derive；Consume 内按 host set6→7→8→10 相对顺序排列，模式消费者后才 advance replay。没有将同帧 toolbar/MEI geometry 提前到 picker 之前。
- 测试构造的边界是实际 Taffy node 与给定的已布局 Ui2dExtent；不验证启动建树/布局算法。没有 MEI 实体，因此不声明菜单遮挡通过；原 MEI pre 须在 ui_update 之后同 set6，本候选不改该顺序。
- 新依赖请求仅限 host `Cargo.toml` 的 `[dev-dependencies] bevy_tasks.workspace = true`，复用 workspace/lock 现有版本来初始化 picker 的 ComputeTaskPool。未修改 manifest 或 lock。
- 待 source 冻结解除、中央接通 core/toolbar 候选后可排队：`cargo test -p tiny-glade --test toolbar_physical_chain --locked --offline -- --nocapture`。禁止以 rustfmt 成功代替编译/执行通过。

新发现独立断口：`send_mode_just_changed_event` 写 `AppModeChanged`，`switch_app_mode_button` 读独立 `resources::events::ModeJustChanged`。工作树静态搜索未找到后者生产 writer；已报中央。候选只验权威 mode/category，不手发 ModeJustChanged 来伪造 selected/动画闭合。菜单遮挡、选择动画、OS 鼠标输入、真实小窗口画面与性能验收仍未验证。

遵照 `D:/game/PERFORMANCE-STANDARD.md`：60 FPS 最低、120 FPS 合格，必须真实 960×540 小窗口 Release、完整业务/画质，报告持续平均、1% low、p95/p99；CPU 测试不代表 FPS 通过。Replay factory 生产已按原闭包准备，但下游 signifier owner/动画/生命周期缺口仍在，不能据此声称真实 deny 图标已验收。

物理输入候选追加第 4 个测试（循环 undo/redo 两例）：真实 HistoryButton 的 press/hold/release 生产 UiHistoryButtonPressed，严格下一帧 set5 才进入动作槽16/17，实际 mode_manager 发 HistoryCommand 并进入 Replay；Replay 中历史按钮仍工作，最终释放不重复命令。仅验证 command 生产；空历史 fixture 不代替 history_process 或实际编辑撤销/重做验收。仍未挂载、未编译执行。

## 物理链候选逐项资源审计（冻结期间静态核对）

结论：Harness 原先**没有漏 MouseDepthValue、PreviousAppMode、Events<AppModeChanged>**。`backend/input/mod.rs::register_input_events` 明确注册三者：MouseDepthValue init_resource；PreviousAppMode 从已插入的 Path 初始 AppMode 捕获；AppModeChanged 经 EventRegistry 注册。候选现已在真实注册器返回后立即 assert 三者存在且 previous=Path，不额外兜底初始化来遮盖注册器缺陷。

来源记号：I = 实际 `register_input_events`；H = Harness 明确插入；L = Bevy Local/Commands/Query 自动状态；P = fixture 初始化 ComputeTaskPool。下表列每个已安装系统的所有运行依赖，R/W 为资源/事件读写的合并列，不省略嵌套 SystemParam。

| 实际系统 | 资源依赖及来源 | 事件／其余依赖及来源 |
| --- | --- | --- |
| begin_input_frame | CursorCapturePolicy I | Exclusive World |
| reset_input_state | ActionSetValues、KeyButtonInput I | NewSessionStartedCmd、SessionLoadedCmd I |
| record_and_replay_input_for_trailers | InputRecordReplayState、KeyButtonInput、InputReplayForTrailers I | InputReplayCommand I |
| do_input_replay_ui | InputReplayForTrailers、InputRecordReplayState I；EguiContextResource 未插入 | resource_exists 条件明确跳过；这是调试 UI，不代表被验证 |
| control_input_replay | Toolbar、AppMode、MainCamera H；InputRecordReplayState、CursorCapturePolicy I；CursorCapturedPosition H | InputReplayCommand、OsCursorCaptureCmd、OsCursorVisibilityCmd I；后两者与 capture resources 来自嵌套 CursorCaptureState |
| send_mode_just_changed_event（Control 与 Consume 两实例） | AppMode H、PreviousAppMode I | AppModeChanged I；完整 current/previous |
| time_update | Time、InputRecordReplayState、GameTime I；ShaderRefactoringState、EscMenuState H | 无 |
| translate_input_to_cursor_moved | CursorPositionSS、WindowSize、CursorCapturedPosition H；ActionSetValues I | InputCursorMoved、InputCursorDelta I |
| translate_input | GameTime、Time、KeyButtonInput、MouseButtonInput、ActionSetValues、ActiveInputModeTracker、SteamInputActive、DemoModeEnabled I；GameProfile、EscMenuState H | MouseWheel、InputCursorDelta、UiHistoryButtonPressed I；Local MouseMoved L；绑定表从真实 GameProfile 投影 |
| calculate_input_delta | ActionSetValues、Time、ActionSetStates I | 无 |
| input_record_replay | GameTime、MouseDepthValue、CursorPosition、ActionSetStates、ActionSetValues、InputRecordReplayState、CursorCapturePolicy I；Toolbar、AppMode、MainCamera、CursorPositionSS、CursorCapturedPosition H | OsCursorCaptureCmd、OsCursorVisibilityCmd I（嵌套 CursorCaptureState） |
| update_cursor_position | ActionSetStates I；CursorCapturedPosition、WindowSize、CursorPositionSS H | CursorMovedScreenSpace I |
| update_mod_key_state | ActionSetStates 两个只读参数、Modifiers I；AppMode H | DecoratorOpEvent I；Local ToggleSnapping L |
| hover_intersection | UiState、MainCamera、CursorTerrainRaycast、CursorPositionSS、WindowSize H；MouseDepthValue I | Local HoverIntersectionCache L、全局 ComputeTaskPool P |
| ui_update | UiState H；ActionSetStates I | WindowFocusEvent、EscapeMenuJustOpened、ShutdownGameCmd、OverwriteBrushPreview、PlayUiSound H；OsCursorVisibilityCmd I |
| render_taffy_ui_to_widgets | UiState、UiNavState、AppMode、UiSignifierStream、CursorPositionSS、GameConfig、ToolbarSizeMult H；ActionSetStates I | UiClickEvent、UiDragEvent H、UiHistoryButtonPressed I；实体 Query 与 Commands L，实体由 Harness 明确建真实 Taffy node |
| switch_app_mode_button | AppMode H | UiClickEvent、RequestAppModeChange、ModeJustChanged H；Query+Commands L |
| synchronize_mode_manager_ui_context | UiState、ModeManagerUiContext H（从真实 UiState 初始化） | 无 |
| mode_manager<EditType,MaybePendingSnapshotId> | AppMode、RestoreAppTo、Toolbar、ModeManagerUiContext、History<MaybePendingSnapshotId>、TimeOfDayScreen H；MainActionSet=ActionSetStates I | LoadingGameComplete、RequestAppModeChange、RequestToolbarCategoryChange、RequestHistoryCommand、ClearCanvasCmd、UiHistoryPlaySelectAnimation、HistoryCommand、ModeManagerSound H |
| advance_input_replay_frame | InputRecordReplayState、CursorCapturePolicy I；CursorCapturedPosition H | OsCursorCaptureCmd、OsCursorVisibilityCmd I（嵌套 CursorCaptureState） |

本候选有限帧运行保留 Events 供最终断言，未模拟无限会话的 Events 周期维护；下一层 App fixture 使用实际 EventRegistry/App 维护，不能拿这里的短链结论冒充长时间生命周期验收。

### 完整模式事件合同

lifecycle 独立核对原 publisher140a875e0：current AppMode16B、previous AppMode16B，不是 tool+gap。候选新增 TypeId 相等断言，要求 ModeJustChanged canonical alias 到 AppModeChanged；不加丢 previous 的桥。release 帧验证实际 publisher 精确发送 `{current:Water,previous:Path}`，下一帧 set8 验 selected=1 与 UiPlaySelectAnimation 真正生成。当前 live 尚未迁移，候选因此明确依赖该迁移；不手发事件。

## 下一阶段最小完整 fixture：真实建树、布局及 MEI 输入交界

按以下依赖扩展，不能仅多加一个 screen rectangle 就声称菜单链完整：

1. **启动与 owner**：复用 `toolbar_startup_tests` 的真实 `StatesPlugin`、`AppLoadingState`、OnEnter(Game) `toolbar_setup` 和 OnExit(Game) `despawn_glade_entities`。由 setup 真实创建 ToolbarLayout、PhotomodeToolbarLayout、UndoRedoLayout、Taffy entities/nav links；不手造按钮、不手写 extent。GameConfig、TaffyLayout 必须来自正常注册。让已有输入注册器与 canonical 模式事件迁移先到位。
2. **可见性与输入**：补真实 PhotomodeScreen、PhotomodeState、WelcomeScreenState、SteamdeckScreenshotMsgScreen（默认只作为 startup 合同），hide_ui 在 set7 renderer 前。set7 完整焦点传播/keep cooldown/父子显示/appearance/selection动画；set8 实际 toolbar_size_mult、auto_hide、undo auto_hide、alignment、button_size_and_padding。新增所需资源主要是启动产出的三 layout owner、菜单/photo 状态、UiToolbarButtonResetAllCooldowns、UiJustBecameInFocusEvent、ToolbarUiIsInFocus 和 PlayUiSound；Query 组件来自真实 setup。不得为通过点击测试强制所有按钮 visible/展开。
3. **真实布局输出**：set44 依原顺序 `prepare_text_fragments → taffy_finalize_element_size → taffy_perform_layout → taffy_update_drawables → debug_hide_ui → render_text_fragments`。需要真正 TextRenderer（字体加载）、DebugGui、WindowSize、GameConfig、ToolbarSizeMult；不能预填 extent。测试从目标组件读取真实计算后的可见矩形中心来发送 InputCursorMoved；要先跑至 set44 产 extent，再下一 set7 注册，再后续 set6 才命中。首帧零/旧 extent 是时序事实，不通过提前 layout/producer 隐藏。
4. **MEI 菜单仲裁**：采用实际 EscMenu/Welcome menu producer 创建 node 和 handler，而非在测试放一个高优先级假挡板。真实 `mei_pre_update` 必须在 hover→ui_update 后同 set6，读取前一帧 Node geometry，生产下一帧 WidgetParams；它需要 MeiContext、MouseWheel、UiNavState、WindowSize、Time、ActionSetStates、UiState、CursorPositionSS、PlayMenuUiSound。菜单真实 post_update 负责 prepare/layout/draw，须走加载 assets/ui 与 assets/fonts/table.ron 的 MeiContext，真实 TextRenderer、AssetTextureLibrary、GraphicsDeviceResource、ActiveInputModeTracker；Steam资源是Option。不能凭手写 Node.pos/size 宣称真实菜单布局闭合。
5. **验证目标**：普通工具栏真实分类/模式选择→canonical full mode event→选中动画；EscMenu 出现时目标选择不穿透；关闭后可再次选择；跨帧 hover/capture/escape/focus-loss；Game→Reload→Game 不保留旧实体/nav/active owner。屏幕缩放需真实 WindowSize 与 layout/renderer scale 同步。MEI GPU资源与字体真实准备后再上真实窗口验证，CPU substitute 不能当成真实画面验收。
6. **性能边界**：依 `PERFORMANCE-STANDARD.md`，完整业务/画质真实 960×540 小窗口 Release 下持续 >=60 FPS 为最低、>=120 FPS 才合格，并报告1% low/p95/p99；本候选及下一层CPU布局fixture都不是FPS验收。不得省 MEI、动画或绘制来提升表面数值。

静态审计未运行 Cargo/游戏。生产源码、manifest、lock 继续冻结。

下一层 fixture 的可用菜单 producer 进一步核对：当前 `toolbar::menu::welcome::ui` 已有实际实现，SystemParams 为 MeiContext、WelcomeScreenState、VermeerState、GameTime、EscMenuState、GameConfig、GameProfile、SaveGameProfileCmd、Option<SteamClientResource>、Local<SteamdeckWelcomeMsg>。可从此验证真实菜单 node/handler/布局；不能用 skip_welcome 或假dismiss事件让菜单消失。当前源码搜索未定位 EscMenu 的完整真实 UI producer，故 EscMenu 遮挡应保留独立 U，不能将资源枚举置 Main 后宣称真实 EscMenu geometry 已生成。

## 下一紧邻 P1：toolbar_steal_mouse_input 完整窄候选

已准备 `candidate-toolbar-steal-mouse-input.rs`，只在报告目录。原142344df0/2706B完整正文 SHA256 `9b00afb82fd97223222be7f4c04718ec342a75df07675620972b92f8882e115c`；静态提取自SHA f507原B，没有执行原版。证据在 `toolbar-input-evidence/toolbar-steal-mouse-input.asm`、`steal-manifest.json`、`toolbar-steal-system-signature.txt`、`toolbar-steal-constants.json`。已有phase2-toolbar-layout原schedule档案已复用，未重复执行旧native probe。

确认它**没有 ActionSetStates 参数，不清任何动作**。六原参数依次为 ToolbarLayout、PhotomodeToolbarLayout、TaffyLayout、UiState、DebugGui、ToolbarSizeMult。功能是背景屏幕volume producer，填充下一帧picker；不能误实现为菜单开时reset点击，也不能单靠此系统宣称EscMenu完整挡世界。

- 原set8 chain140ff0be0：toolbar_size_mult_update → toolbar_steal_mouse_input → button三元组。原set6 hover→ui_update先消费/清理前帧registry，set7按钮producer后set8背景producer为下一帧注册。
- DebugGui+f7 Show2DUI关闭直接返回；只有puffin profiler前后样板位于门外，不是业务分支。两个layout总是按normal→photo执行，没有AppMode/是否展开等发明的门控。
- 每个layout迭代 `once(node_b@20)` 再所有非空menu.node@10（定义表stride20）；node_b即便无效也走真实Taffy::layout unwrap，保留原panic，不静默过滤。无业务查询或资源被省略。
- 命名原fmt pieces14304e698指向14304e688/12B `steal_input_`，formatter141e5fa30为slotmap::DefaultKey Debug；安全format保留语义，不复制旧RustABI。
- 布局字段原Layout+0是size，+8是location，已用同B `taffy_update_drawables`142340d20静态交叉验证。运算严格：padding=multiplier*10.0（常量1429345a8）；double=padding+padding；half=(size+double)*0.5；center=location+half；min=center-half/max=center+half。不是以旧中心对称扩展；不能把min简化location而改变舍入。
- tag3 ScreenAabb distance精确0x3ca3d70a（0.02）；WidgetParams::new默认word101、no_history清bit0、UiSounds::empty→sounds；没有occlude_by_world/allow_negative_distance调用，故保留true/false。与按钮producer occlude=false不能混用。
- 三候选测试：真实taffy计算normal/photo/contextual三root产实际geometry/name/flags；DebugShow2DUI关闭在无效node前返回；无效必迭代node保留unwrappanic。没有Actions/AppMode/GameConfig资源也能执行，因为原合同没有这些参数。未编译/未执行，中央尚待独审与集成。

恢复经验补充：函数名中的steal不等于“修改ActionSet”；必须先读完整SystemParam名义类型，再读所有write/call。图形producer要同时查下一帧清空/消费顺序、真实layout字段、命名与flags默认，防止恢复形状却漏注册仲裁语义。

ui_nav_update：已定位并复用 `progress-20261020/phase2-toolbar-focus/UI-NAV-NEXT.md`、完整有界6017B asm、签名及80个历史空Query边界fixtures。空Query只证入口门/confirm/cancel/opposite-direction；完整nearest/spatial/UiNav各variant/Taffy树/anchor跨root/iter_subtree算法仍未恢复，不能把入口门短实现导出为完整导航。
