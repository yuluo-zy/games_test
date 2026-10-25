# 初始/欢迎/新建→可编辑会话入口合同 V1

当前结论：有真实场景载入/渲染基础不等于用户从初始页面能够完整进入任意新建场景。已经确认原版入口的两个producer和必要状态/注册面缺失；不能用强制注入session的CPU/GPU诊断替代按钮可达性。本文没有声称用户当前停留的每一帧都由这一项解释，实际窗口状态/加载完成/输入/渲染仍由root验证。

## 当前源码确定断点

1. `tiny-glade/src/gui/gui_escape_menu.rs:169..172` 创建Menu.New按钮，只写 `NextEscMenuState=NewSession`。`advance_menu_state_system`在同GUI阶段消费Next使Esc当前状态切到NewSession(tag5)。build_escape_menu只绘制Main/Quit，其它_=>{}是其自身职责边界；原版另有独立toolbar UI系统，所以不能凭这个_=>{}断言原Escape重建漏了一个大match分支。
2. `systems/toolbar/src/menu/mod.rs`当前只pubmod credits/load/report_bug/splash/load_ui，没有原new_session/welcome/daily_welcome源，也没有完整MenuFlowPlugin。`run_game.rs:614..620`只挂load_ui::ui到Update/GameSystemSet36/in_state(Game)；new_session的tag5没有真实绘制/选择/请求消费者。点击“新建”后无法走到所选模板的NextSession发布。
3. 原canonical WelcomeScreenState位于 `country-core/resources/onboarding.rs`，当前该源只有hint事件/教程操作，没有WelcomeScreenState；全workspace实际source未见类型或消费实现。它的原welcome::ui亦未存在或注册。不能随意在gui.rs再造同名状态类型。
4. 原new_session::ui必需 `CurrentDaily`，该原mapped资源daily.rs当前缺失；ShowDailyWelcomeCmd和DailyWelcomeScreenState也缺。不能以默认daily空体或资源不存在run_if把整个New菜单悄悄跳过。Steam客户端在原welcome签名本来是Option，必须区分这种真实可选边界与必需资源。
5. startup下游当前已有真实 `SessionFlowPlugin::build`发布首个LoadSavedGame请求、OnEnter(Game)的load_game_session、LoadGameManager、完成事件mode manager及Reload→Game桥。初始空账号路径会选择intro，不等于手工新建UI已完整；具体选择/复制模板原算法仍应对原类型复核。

## 原B身份、合法scope与证据

TG-700584f34568812faf `crates/systems/toolbar/src/menu/welcome.rs`、TG-036ffc7cc830b9722f `.../new_session.rs`均经live show确认NULL后正常claim，owner decorator-menu-entry-20261021。后续源码只在各合法work出candidate。必要host `menu/mod.rs` TG-ad679b6f7470015eb3仍meihost-20261018；onboarding TG-ab43622efad901bc85、gui TG-cfac1f498c427933d2原owner保留，必要增量走委派子目录，不抢claim。daily TG-aa950dbc8de46ac433、daily_welcome TG494当前NULL，仅只读，未把claim工具连接算业务完成。

ENTRY_NATIVE_BYTE_PROOF_V1.json核9完整PDB范围，全部指令原RNE字节与范围连续匹配。关键：

- MenuFlowPlugin build14068de50/1813B PDB339:1468；当前source遗漏其完整注册。
- new_session::ui141eb9690/16011B PDB2839:3860，closure0 140fc74c0/643B。
- welcome::ui141ebe130/2158B PDB2842:2036，closure1 140fc9230/771B，closure2 140fc9540/691B。
- daily_welcome::ui141ebedd0/3601B PDB2843:2036。
- 新建run_if constructor140ff2b10/491B，欢迎140ff7a30/523B，二者原PDB完整typed SystemParams存于MENU_TYPED_CONSUMER_REFERENCES_V1.json。

ReVa保留decomp仅作导航：MenuFlow provider body1387B小于完整1813B，welcome1858B小于2158B；new_session provider body10875B小于16011B并明确有bad instruction/type propagation/restarts警告，不能用它当完整算法或自动翻译成Rust。后续必须以完整RNE、原typed caller、rodata/跳表核对分支。原B资料没有作者源码，skeleton本身只是元数据。

## 原注册与共享状态

原MenuFlowPlugin依次插Esc Closed(tag2)、Next/Prev空tag11、WelcomeScreenState字0、DailyWelcomeScreenState字1；原构造分别68de7c/90/a4/ae/bb。多个UI tuple使用in_state(AppLoadingState::Game)，原68e29e写rawset0x24=36，68e2a1调用in_set，68e2ac添加Update。这应落在现同一App/set36，保现ESC39/advance路径，不另开UI World或用新页面绕过。欢迎是独立state consumer，不能用点击New来假初始化它。

欢迎原参数十项：ResMut<MeiContext>、ResMut<WelcomeScreenState>、Res<VermeerState>、Res<GameTime>、Res<EscMenuState>、Res<GameConfig>、ResMut<GameProfile>、EventWriter<SaveGameProfileCmd>、Option<Res<SteamClientResource>>、Local<SteamdeckWelcomeMsg>。原状态0在Esc Closed时变1/timer0，实际Vermeer门与SteamDeck分支决定是否绘制，必须保0.3秒淡入与模板/两个真实slot。原profile+206/default字段语义要对当前canonical GameProfile复核，不按注释unknown猜名称。

## 新建的真实请求副作用与下游类型硬门

原new UI十五SystemParams：ActiveSession、SavedSessionList、Time、Esc、CurrentDaily、Local<HoveredGlade>、NextEsc、Mei、NextState<AppLoadingState>、Commands、Local<DemoSaveLimitWarning>、MusicAudioState、GameProfile、SaveProfile writer、ShowDailyWelcome writer。包括demo数量限制、hover/选择、daily分支、主题/起始模板、profile和音乐状态，不能把16KB简写成一个New按钮。

已精确锁到实际点击后的关键正常路径：

- 141ebb200增长原GameProfile.new_glade_count(+1d8)；
- 141ebb21a写NextState字节2（Reload）；
- 141ebb5ad调用NextSession::create140b46960；
- 141ebb5ce调用原Commands::insert_resource<NextSession>140fe0190（PDB1222:37628）；
- 141ebb5ff调用NextEsc::set14093fad0，输入Closed=2；
- 后续profile-save/daily通知/音乐状态分支仍需全路径恢复，不能只认上述五个写点就称完整。

非常重要的当前下游缺口：create实参的96B模板在141ebb50f..524首三个qword来自分配并复制的owned文本缓冲（cap/ptr/len形态），而现NextSessionTemplate只允许固定NextSessionKind枚举加9个未知u64，create只是搬运。非niche的拥有载荷分支尚无安全Rust所有者。这里暂不凭布局猜其最终原字段名；完整类型/原load消费分支是必须核的合同，不能把原指针塞u64模板或用load(nil)/空ID/default模板伪装当前用户选择。原NextSession新建请求→load_game_session路径需要与原已恢复LoadParsedGame等安全owner一起定型。

当前已有桥为：真实Menu选择→[缺新建请求producer/拥有载荷]→NextSession+Reload→原PostUpdate Reload请求Game→下一OnEnter(Game) load_game_session→LoadGameManager实际导入/回放/完成→AppMode恢复至编辑。入口两端已有许多真实实现，但中间新的用户选择请求没有闭合，不能用后台intro注入来声称从菜单完整可达。

## 有界实现顺序与验收

1. 先冻结Welcome状态/typed入口与NextSession拥有创建分支的完整原合同，和canonical resource owner协调；不复制现有SessionFlow/LoadManager。
2. 在原welcome/new_session源恢复完整MEI生产树、事件、原数字/用户选择/失败次序；daily如果走到必需支路则原mapped必要源同步恢复。父级音频/会话/资源API只窄patch，不能造第二层compat state。
3. 唯一menu/mod注册增量及run_game窄caller接同set36与已有Esc39；全部必要producer完备后才挂，不加resource_exists掩盖缺资源。
4. CPU实际MEI点击和同App资源/事件顺序：首次welcome确认→关闭→可选toolbar；Main.New→真实选项→NextSession持有选择、NextEsc关闭、Reload→Game、完成事件/进入编辑；demo边界、back、daily和旧session切换须保原分支。root最后在本机窗口真实点击、编辑、保存/重启验证。仅helper测试或强制NextSession诊断不替代该端到端验收。

本V1是准确断点和实现准入合同，尚未源码候选或独审通过，也未Cargo/GPU。旧history数字候选及所有此前业务包保持冻结，未触同test/ICC。
