# toolbar C 子范围分析：menu / photomode / text / 许可服务 / 插件注册

> 由批次 3 分析 agent 产出，父级代为落盘。证据：B patched EXE Ghidra 共享库反编译 37 个函数（job：tool-usage/jobs/toolbar-c/menu-text.json 与 credits-photo-server.json）。O/I/U 标注。

- 通用：puffin 作用域、mei 节点 SWAR 哈希探测（std HashMap raw_table 控制字节 16 字节 SWAR 比较）为库内联，非业务；全部 UI 系统共用节点 id 混淆常量 `0xb1695320004fb443`（`calculate_hash((&str,()))^const`）。骨架「文件级接口候选」下计真实参数（见各函数差异登记）。

## 1. menu/mod.rs
### impl$0::build（MenuFlowPlugin）@0x14068de50（1387）O
insert_resource 5 状态资源（EscMenuState/NextEscMenuState/PrevEscMenuState/WelcomeScreenState/DailyWelcomeScreenState）；Update+in_state(AppLoadingState) 门控注册 4 系统（credits::spawn_third_party_license_http_server、daily_welcome::ui、corrupted_save_file::ui、save_error::ui，各 run_if 签名见函数节）；add_systems(Update,in_set) 注册 advance_menu_state（签名 (Res<ActiveSession>,Res<SavedSessionList>,ResMut<EscMenuState>,ResMut<NextEscMenuState>,ResMut<PrevEscMenuState>,EventWriter<EscapeMenuJustClosed>)）；TypeId 查重后 register_event<ShowDailyWelcomeCmd>。常量：TypeId hash 0x633d43f1bfc13234。
### impl$1::build（SplashScreenPlugin）@0x14068e570（56）O
仅 add_systems(Update,in_set(GameSystemSet)) 注册 splash::ui。
### advance_menu_state @0x14068e5b0（538）O
算法：ESC 菜单状态机推进器。NextEscMenuState 置 changed；取待切换枚举（判别子在[0]，负载[1..0xb]），临时把判别子写成 0xb 避开 Drop 搬移负载；待切换==2 → 推 EventWriter<WriteSettingsFileCmd>（离开设置写文件）；否则旧当前状态搬入 PrevEscMenuState，负载写回当前，调 NextEscMenuState::set()。常量：变体 2=设置、0xb=无菜单。差异登记：真实 ABI 第 6 参为 EventWriter<WriteSettingsFileCmd>（本体明确 grow_one<EventInstance<WriteSettingsFileCmd>>）；EscapeMenuJustClosed 未在本体写出，疑似在 NextEscMenuState::set()（库外）；ActiveSession/SavedSessionList 本体未读。

## 2. menu/welcome.rs
### welcome::ui @0x141ebe130（1858）O
签名（候选+校准）：(ResMut<MeiContext>,ResMut<WelcomeScreenState>,Res<VermeerState>,Res<GameTime>,Res<EscMenuState>,Res<GameConfig>,ResMut<GameProfile>,EventWriter<SaveGameProfileCmd>,Option<Res<SteamClientResource>>,Local<SteamdeckWelcomeMsg>) 另含 EventWriter<WriteSettingsFileCmd>。算法：欢迎屏状态机+smoothstep 淡入。某 UI 状态关闭期（+0xe5/+0x206 非零）→置 WelcomeScreenState=2 退出；SteamDeck 探测；state==0 且 EscMenuState==2→state=1 计数清零；state==1 且 vermeer 未激活且菜单开→safe_frame+template_with_id，t=clamp(counter/0.3)，alpha=(3-2t)t² 写节点+0xb0，桌面/SteamDeck 分流 closure_env_2/1；菜单关→+0x206=1、推 WriteSettingsFileCmd、state=2；vermeer 激活→复位计数。常量：0.3s；WelcomeScreenState={enum, f32@[1]}。未知：+0xe5/+0x206 归属资源；SteamdeckWelcomeMsg 判别语义。

## 3. menu/new_session.rs
### new_session::ui @0x141eb9690（~2400 行反编译）I
算法：新谷创建屏编排。顶部模板（closure_env_0 标题/主图）→中部模板（closure_env_3..5 名称/尺寸/图案）→每日/每周横幅（DailyThemeInfo::is_weekly 分流 + 连续 7 次 lang_lib::lookup（id 0xf/0x10/0x11），每段配颜色常量 0x100/0x200/0x300/0x400/0x500）→4×MenuButtonContent + LabelContent/ImageContent →SessionMeta::is_xmas2024() 圣诞模板分流。未知：模板 id/lang key、resolve_new_glade（独立符号未展开）。

## 4. menu/daily_welcome.rs
### daily_welcome::ui @0x141ebedd0（2900）O
签名：(ResMut<MeiContext>,ResMut<DailyWelcomeScreenState>,Res<GameTime>,Res<EscMenuState>,EventReader<ShowDailyWelcomeCmd>,Res<CurrentDaily>,Option<Res<SteamClientResource>>)。算法：每日欢迎弹窗（状态位+事件重置）。SteamDeck 上不显示；state 首字节 bit0=已展示标志、counter f32@+4；bit0==0 且 EscMenuState==2→建节点（字节+0x5d=0/+0xb7=1，事件判别子 1），t=clamp((elapsed-0.7)/0.3)，alpha 写+0xb8，is_weekly 分流+lang_lib 多段（Option<String> niche 0x8000000000000001 判空+clone/dealloc）；菜单关→bit0=1 清计数；bit0==1 且有新 ShowDailyWelcomeCmd→整结构清零重允许。常量：延迟 0.7s/时长 0.3s、0x41900000=18.0。

## 5. menu/splash.rs
### splash::ui @0x141ec1390（544）O
签名：(ResMut<MeiContext>,Local<f32>,Res<GameTime>,Res<LogoAnimation>)。算法：Logo 启动画面+时间累计。node_id=hash(("splash_logo",()))^const；reuse_or_create_node，字节+0x5d=0；推 iced Event(判别子 0)；SWAR 命中设 50×50（0x42480000）、位置清零、+0x5b 区写 0x303(u16)；NodeContext::with 建 logo；Local<f32>+=GameTime.elapsed。未命中 panic。未知：0x303 语义。

## 6. menu/load.rs
### load::ui @0x141e1fe80（~2400 行）O
算法：读档屏编排（缩略图滚动+装载校验+备份排序）。thumbnail_scroll 建容器并配置 ThumbnailScrollContent(+0x50/+0x54/+0x70/+0x76)；命中项→NextEscMenuState::set/推 WindowFocusEvent{7/0xc}/is_session_ok_to_load+deny_load 校验；装载链：load_session_meta（unwrap_or_default）→SessionMeta::assets_root→GladeSettings::load→组路径(save_folder_path+_join)→parse_saved_game_data；失败构造 SaveLoadError（Instant+1s、0x42f88210）；成功 insert_resource<NextSession>+NextEscMenuState::set+PlayMenuUiSound。备份：backup_folder_path+SaveBackupList::iter_backup_files→Vec<BackupSaveFile>（from_iter）→closure_env_6 排序（<0x15 用 insertion_sort_shift_left 否则 driftsort_main）→分页渲染 closure_env_7/8。尾部另有装载分支+SessionMeta::write_to_disk。未知：会话目录名校验（pcVar48 0x1b 串 vs ActiveSession+0x60）、DEMO 0x42f88260 分支。
### impl$4::thumbnail_scroll<&str> @0x140fa06a0（1036）O
算法：缩略图滚动视图构建（含共享度量 Arc）。child_with_global_id+get_content_mut_impl<ThumbnailScrollContent>；+0x40=slot 名；旧 Arc<Mutex<LoadMenuThumbnailMetrics>> 引用递减换新 Arc（线程安全度量供后台 session_selection 渲染共享）；+0x74=1/+0x58=1；三层节点查找 |0x40/|0x20，最终清零+alpha 1.0+字节+0x5a=4；句柄 TypeId 常量 -0x51a045a1c4472226。
### display_save_file_load_error @0x141e23b90（110）O
template_with_id+with_slot<closure_env_0>(标题)+closure_env_1(详情)。
### load_session_meta @0x141e23c00（156）O
SessionId::save_folder_path→Path::_join→SessionMeta::try_load_from_path→Option<SessionMeta>。未知：join 子路径串。
### show_contextual_buttons @0x141e23d20（451）O
node_id=hash("<6字符>")^const；推 iced Event(判别子 4，4 float：x、y+250、0、-1，u64=0x41f0000043480000=(200,30))；template_with_id+with_slot（传调用方+0x38/+0x49/+0x10=会话名/日期/元数据）。

## 7. menu/corrupted_save_file.rs
### ui @0x141ebfbf0（581）O
签名：(ResMut<MeiContext>,Local<MessageScreenState>,EventReader<DisplayCorruptedSaveFileWarning>)。EventReader 簿记：新事件→置首字节 0；已显示且无新事件→返回防重弹；否则建节点（字节+0x5d=0/+0xb7=1，判别子 1），alpha 0x40400000=3.0，template_with_id+with_slot<closure_env_1>。

## 8. menu/save_error.rs
### ui @0x141ec0ce0（1637）O
签名：(ResMut<MeiContext>,Res<AutoSaveState>,Res<EscMenuState>,Res<ActiveSession>,Local<Option<Instant>>)。算法：autosave 失败弹窗+恢复提示（时间门控）。有错：Instant::now 存 Local；lang_lib 取标题(0x18)/正文(0x1c 回退)+format_inner；建节点（id=hash("save_error")^const，判别子 1），alpha 0xc1200000=-10，with_slot<closure_env_2/3/4>；释放 String。无错：若 Local 哨兵 1000000000 或 elapsed>0xe 或 EscMenuState::is_closed→返回；否则渲染"已恢复"提示（closure_env_0）。未知：哨兵 1000000000 语义。

## 9. menu/credits.rs
### credits::ui @0x141dfa070（4398）I
EscMenuState!=9 直接返回。建滚动容器（credits::scroll），ScrollContent 初始化 0xb000000001，+0x1c=Local<f32>；遍历子节点统计 min(-3.4e38)/max(+3.4e38)，max>min 时写 Local<f32>=(max-min)-450.0（滚动上限=内容高-视口 450）；建"第三方许可"按钮（id=hash("<15字符>")^const，判别子 0x100000003，50×120，LabelContent set_text/metrics/family，LabelContent+600 写 0xfffffff0）；closure_env_1 渲染许可 URL（读 Local<Option<ThirdPartyLicenseServer>>）。未知：容器尺寸细节。
### spawn_third_party_license_http_server @0x141dfb410（520）O
签名：(Res<EscMenuState>,SteamOverlayOpenResources,Local<Option<ThirdPartyLicenseServer>>,ResMut<NextEscMenuState>,ResMut<MeiContext>,Local<f32>)。SingleFileServer::http 绑定；Err(判别子2)→format_inner+anyhow::Error::msg 存错误串+哨兵 0x8000000000000000；Ok→std::thread::spawn<closure_env_1> 跑 run，构造 url String "127.0.0.1:8123"（字节序还原），连同 server 写 Local。
### impl$3::scroll<u64> @0x140fa04f0（424）O
child_with_global_id+get_content_mut_impl<ScrollContent>；查节点（SWAR）命中则清零尺寸+alpha 1.0(+0x68)+字节+0x5a=4，返回句柄；未命中 panic。mei ScrollView 容器 helper。

## 10. menu/report_bug.rs
### report_bug::ui @0x141e1b8e0（1853，~2500 行反编译）I
签名（骨架候选+吻合）：(Local<SteamdeckKeyboardRequested>,Local<FeedbackReportEditorMemory>,Res<EscMenuState>,Res<PrevEscMenuState>,Res<ActiveSession>,Res<GameProfile>,(Res<GraphicsDeviceResource>,Res<WindowSize>,Res<FrameRenderConfig>),ResMut<MetaCommander>,ResMut<FeedbackReportResources>,ResMut<NextEscMenuState>,Local<FeedbackReportUiState>,ResMut<MeiContext>,Res<UiNavState>,EventWriter<ReportPlayerFeedbackCmd>,EventReader<PlayerFeedbackReportSendingFinished>,Option<Res<SteamClientResource>>)。算法：bug 报告表单+GPU 截图附加。FeedbackReportResources::reset_choices（关闭复位）；cosmic_text Editor Drop 管理；SteamDeck 探测；FeedbackReportUiState 大 switch：case0 建表单（字节+0x5d=0/+0xb7=1，判别子 1；template_with_id+closure_env_1..7；pack_srgba_u32 组色；FrameContent fill_color/corner_radius；CompoundEditBuilder::new_edit_sequence）；提交：锁 FeedbackReportResources 的 parking_lot Mutex 取内容（Option 取出），空报 log；截图附加：*(+0x45)==0 且 +0x46 → paths::tmp+_join 组两 tmp 路径+try_remove_file 删残留，随后 Vec::extend<[MetaCommand;8]>+GraphicsDevice::physical_device+with_global_allocator<closure_env_10>（rhapsody GPU 分配器报告）排队截图，u32 对转 f32（0x45300000/0x43300000）为截图尺寸。未知：提交格式、closure_env_10 细节。
### try_remove_file @0x141e1e5f0（399）O
path.is_file()→windows unlink；失败 log 记录 io::Error。容错删除 helper。
### impl$11::text_edit<&str> @0x140fa0bd0（1473）O
算法：多行文本框视图（共享编辑内存+futex 锁）。get_content_mut_impl<TextEditContent>；+0x270==0 未初始化→锁 TextEditMemory（futex Mutex+panic_count），状态==4 时 set_metrics/set_text 初始化否则 replace_editor（memcpy 0x198 字节 cosmic_text Editor）；Arc<TextEditMemory> 计数（content+600）；四层查找 |0x20/|0x4；命中点击区（&0x30）→+0x268=1 焦点；返回句柄。结构：TextEditContent(+0x268 焦点、+0x270 初始化、+600 Arc<TextEditMemory>)。
### impl$15::toggle_icon @0x14068cbf0（1276）O
get_content_mut_impl<ToggleIconContent>，+0x18=icon_off、+0x20=icon_on、+0x28=state；建 46×40 节点（0x42340000/0x42200000，字节+0x5b=2、u16+0x5a=1）；三层查找：|0x20、u16 0x200、点击区；命中→state^=1 写回 &mut bool+更新内容。开关图标构建器。

## 11. photomode/mod.rs
### do_photomode @0x14234b010（4478）I
签名（候选）：(ResMut<PhotomodeScreen>,8×Res tuple,ResMut<MainCamera>,ResMut<DebugGui>,ResMut<MeiContext>,ResMut<PhotomodeState>,ResMut<ImageFormationConfigs>,ResMut<VermeerState>,Res<UiState>,8×事件 tuple,2×Res tuple,EventWriter<DisplayRollercoasterPrompt>)。算法：拍照模式编排。EscMenuState 判别子 0x13=拍照态：DigitalActionState::just_pressed 切换 PhotomodeState+0xa；输入迭代 (*action==0x13&&action[4]!=0xd) Pressed→进入（on_enable_photomode+switch_kind+match_free_camera_to_normal 标记+0xb）；Released→退出（switch_kind+VermeerState::deactivate）；动作分发：*(+0x103)==3 推 WindowFocusEvent{1}、动作 2 推{0}+init_walk_camera（+0xc 标记）；拍照中：EscMenuState==0xd 推 WindowFocusEvent(0x3000300)、bVar42 推 WriteSettingsFileCmd、FOV 夹取 clamp(fov,exp2()*lens+8,lens+0x10)；自由相机拖拽：MainCamera::focus 得拖拽状态+slider3d::into_drag_state+视图矩阵 NDC→世界逆投影（divps 齐次除+aspect_ratio 校正）；准星：mode>3 且菜单关→建准星节点（判别子 4，位置((x+aspect)*0.5*1000,(1-y)*0.5*1000)，尺寸 fov*1000）；mode==3&&is_fence→围栏提示节点（判别子 1）；mode==3&&PROMPT_ROLLERCOASTER 静态==1→推 WriteSettingsFileCmd。结构：PhotomodeState(+4 u32、+9/+0xa/+0xb/+0xc/+0xd 字节)、PhotomodeScreen(+0x103 模式字节)、SSBO 名 0x57 字节断言。未知：AppMode/MainCamera::mode 值 1/3/4 语义、事件 buffer 与候选不完全一致。

## 12. photomode/photomode_buttons.rs
### photomode_buttons @0x14234c230（2648）I
EscMenuState!=0x13→置静态跳表 DAT_14304f3a8=0x20 返回。拍照态：建主按钮节点（id=hash("<24字符>")^const，字节+0x5d=0/+0xb7=1，判别子 1），menu_button+hotkey；按钮命中(&0x30)关主按钮；事件迭代+大跳表分发动作字节（byte-4，case 0..0xd）：动作 4→OsWindowCmd+paths::config+_join 组截图目录+create_dir+open::that（打开截图文件夹）；动作 5→PhotoModeCameraMode::next/next_demo（DEMO 分流）推 WindowFocusEvent{新模式}；动作 6/9→推 WindowFocusEvent；动作 0xb→NextEscMenuState::set（回菜单）；动作 0xc→VermeerState activate/deactivate；动作 0xd/0xe→iter_non_default_mut 翻 +0x3c/+0x3d；动作 0xf→翻 PhotomodeState+0xd；动作 0x10→MainCamera::focus 焦点目标循环；动作 0x11→mode_manager::default+推(&str,SettingsMenuState)事件+清+0x103/+1（回设置）。每动作 Commands 推 UiPlaySelectAnimation。未知：动作字节完整编号表。

## 13. photomode/setup_photomode_toolbar.rs
### setup_photomode_toolbar @0x1426246e0（6111）I
算法：拍照工具栏装配。ToolbarLayout::setup_screen_and_toolbar_nodes；逐项构造 ToolbarItem（标签 String、图标、icon-scale 0x3f8000003f800000、类别+索引 u16 对 0x302/0x303/0x603/0x202/0x102/0x802/0x703/0x702 等、类型字节 6/4/5/0x10/0xc/0xd/0xe；drop_in_place<ToolbarItemKind>）；DEMO 模式切换标签集（DAT_143149910 len 0x25 vs 0x20）；组 Vec<(label&str,ToolbarCategory,Vec<ToolbarItem>)> 大数组循环 add_category_button<closure_env_0>，每类按 ToolbarAlignment::is_horizontal 调 add_contextual_menu；link_up_nav 收尾。结构：ToolbarItem{kind@0、label、图标槽1..3}。未知：标签字符串内容、ToolbarItemKind 值。

## 14. photomode/ui_slider.rs
### ui_slider @0x1423486d0（4152）I
算法：拍照滑块渲染+拖拽交互+按类型写参数。遍历脏文本记录（同 text.rs 布局）；实体存在先 despawn；slider_val_01 取当前值；Aabb2::from_center_dims/contains 命中（fVar43/0.2 光标映射）；EasingFunction::apply；Commands::spawn<(Ui2dExtent,Ui2dImage,UiDrawOrder)> 建滑块条实体，纹理 "toolbar/slider_widget"（0x15 字节）；鼠标：DigitalActionState 双键 just_pressed/released/pressed → 拖拽标志+起始值+id 对；事件迭代：拖拽(字节3)→slider_val_01+inverse_lerp 算新值；按 widget 类型字节大 switch（0..0x1e）写目标：0..5→ImageFormationConfigs::iter_non_default_mut 写+0x1c/+0x20/+0x24/+0x30/+0x38/+0x34（仅 EscMenuState==0x13）；6→lens+8=lerp(lens+0xc)（FOV）；7→LogSlider::param_to_value+focus[6]、mode→3（焦点距离）；8→lens+0x18；9→非拍照 clamp 0.3..2.0 写笔刷大小；0xa/0xb/0xc→BrushSizes::set_path/water/garden；键盘导航推 WindowFocusEvent{4/1/8/9}。常量：min/max 表 DAT_14304ed8c/0x14304edc0、跳表 DAT_14304ee28。未知：0xd/0xe/0xf/0x10/0x14 等被截断分支。
### ui_slider_reset @0x142347f90（928）I
算法：滑块复位。事件迭代找目标；读类型+min/max+inverse_lerp 算默认；switch（0..0x2b）：0..2→iter_non_default_mut 复位+0x1c/0x20/0x24（默认取自 TweakerView::as_inner_mut 当前配置）；3..5→写+0x30/+0x38/+0x34=默认；6→lens+8=lens+0x14；7→focus[6]=exp2(clamp(v,-2,2)-3)；8→lens+0x18=lens+0x24；0xa/0xb/0xc→BrushSizes::reset_path/water/garden；0x11/0x15/0x16→set_water/path/garden（非拍照）；0x2a→嵌套 slider_val_01。提交 UiPlaySelectAnimation+焦点事件。未知：部分分支截断。
### slider_val_01 @0x142349de0（28）O
按滑块索引字节 switch：0..6→ImageFormationConfigs::current_mut；7→MainCamera::focus+log2f(*(lens+0x18))；8→MainCamera::lens+inverse_lerp；9..0xc 空。滑块当前值 getter。

## 15. photomode/lut_gallery.rs
### lut_gallery::ui @0x141ec0270（2283）O
算法：LUT 画廊+复用缩略图滚动。非 DEMO 且 EscMenuState==10 且[2]==1：建节点（字节+0x5d=0/+0xb7=1，判别子 0）；Steam Arc 计数；复用 load::thumbnail_scroll 配置 ThumbnailScrollContent(+0x50=0x3ed0e560、+0x54=0x3c9374bc、+0x70=0x40000000、+0x74=1、+0x58=2)；焦点管理失焦推 WindowFocusEvent{0}；选中（+0x10 非空）→NextEscMenuState::set 关画廊+推 WindowFocusEvent{2}，ImageFormationConfigs::get 取当前配置，iter_non_default_mut 对每项 clone_from 当前 LUT 名（index 0）或写 0x3f800000=1.0（index>0）；建底部模板（判别子 2）+closure_env_0。常量：EscMenuState==10。

## 16. text.rs
### text_startup_system @0x141e9ce50（2209）O
签名：(ResMut<tg_text::TextRenderer>)。算法：字体目录加载器。walkdir 遍历：扩展名匹配 "ttf"/"otf"（*(ushort*)==0x7474||0x746f 且字节2=='f'）；log "Loading font file {path}"；TextRenderer::load_font_from_file，失败 anyhow+unwrap panic；font_faces() 遍历逐面 ttf_parser::language::fmt+log。未知：字体目录路径来源。
### update_text_renderer_scale @0x141e9d960（764）O
set_changed+TextRenderer::set_pixels_per_point(...)。未知：pixels-per-point 计算式来源（候选 Res<WindowSize>）。
### prepare_text_fragments @0x141e9dd20（538）O
签名：(ResMut<TextRenderer>(RCX),Query<(&mut TextFragment,&Ui2dExtent)>(RDX))。算法：文本片段 prepare 遍历（双模式）。按 TextRenderer+0x108 模式字节分两支遍历脏索引（+0xf8/+0x100）：每记录取片段数（0xa0 步长+0x58 / 0x48 步长+0x10），经+0x1a8 映射表+槽位索引(+0x110/+0x118)取目标缓冲；每片段写 changed 哨兵 u32+TextFragment::prepare()。
### render_text_fragments @0x141e9df60（647）O
同上遍历，改调 TextFragment::render()。
### flush_text_render @0x141e9e1f0（416）O
算法：文本 SSBO 上屏。FrozenMap::get<SsboId,Box<BufferState>> 两次取两块命名字节串（0x49=73/0x4c=76 断言）；各调 vtable[0x18] 确认；country_core::render::deref()+TextRenderer::consume<closure_env_0>。未知：两块 SSBO 名称。

## 17. third_party_licenses_server.rs
### SingleFileServer::http<&str> @0x14234d690（612）O
log(level>2)；to_socket_addrs(url) 解析；空/失败→构造 io::Error 返回 Err；Ok→tiny_http::Server::new；写 {url String, server}；Err 则 *out=2。结构：SingleFileServer 含 url String(0..0x18)+Server+文件路径（run 从+0x30 读）。未知：完整布局。
### SingleFileServer::run @0x14286bf40（1888）O
算法：tiny_http 单文件服务循环。IncomingRequests::next 取请求，Err→清理退出；请求异常→Response::empty(404)（0x194）；正常：OpenOptions(read=1) 打开 self+0x30 路径；失败→log "Failed to serve licenses file"+empty(500)；成功→Response::from_file+Header::from_str+add_header<File,Header>+Request::respond_impl<File> 流式发送（mpmc array/list/zero Channel 内部）；退出释放 Server（CloseHandle+通道清理）。常量：404/500。未知：Content-Type 头值、+0x30 路径来源。

## 18. lib.rs
### impl$0::build（ToolbarPlugin）@0x14068d610（1436）O
taffy::Taffy::new+insert_resource<TaffyLayout>+insert_resource<ToolbarSizeMult>；TypeId 查重注册 9 事件：UiJustBecameInFocusEvent/UiClickEvent/UiDragEvent/UiToolbarButtonResetAllCooldowns/ToolbarUiIsInFocus/NotInDemoHint/ShowDemoBorderOverlay/MaxDemoElevationReached/ResetToolbar；add_systems(Startup) text_startup_system（chain_ignore_deferred）；add_systems(Update) 若干系统。常量：TypeId 0xb0e748f9e3aacb57/0x4d2642e55f9027d1/0x189e38b5bedf98fb/0x1ce85a7892ad8401/0x62bb6cad4efc598/0xd150ed151a658bf/0x5eaaca67868e0174/0x7e239f0526778219/0x73f303a4e7b78adc。未知：in_set 集合名。
### impl$1::build（Ui2dPlugin）@0x14068dc70（417）O
校验 SchedulesRequested 资源存在（TypeId 0x8e1bd251962ba784，缺失 panic "SchedulesRequested resource does not exist..."）；add_systems(OnEnter(AppLoadingState)) toolbar_setup（(ResMut<TaffyLayout>,Res<GameConfig>,Commands)，chain_ignore_deferred+in_set+run_if）；add_systems(Update,run_if(in_state(AppLoadingState)))。未知：Update 系统集内容。

## 逻辑总览
菜单由 EscMenuState 驱动：advance_menu_state 唯一推进器（pending→current→prev，变体 2=设置写文件）；各 UI 按状态门控（welcome=2、credits=9、photomode=0x13/0xd、lut_gallery=10）。photomode 由 action 字节跳表驱动进出+相机循环+滑块参数映射。文本链：加载(ttf/otf)→prepare/render 脏遍历→SSBO consume。许可服务：http 绑 127.0.0.1:8123→spawn 线程→run 单文件循环。

## 算法清单
ESC 菜单状态机推进（mem::replace 判别子技巧+变体 2 副作用）；welcome/daily/credits 的 smoothstep 淡入（(3-2t)t²，0.3s，每日延迟 0.7s）；滚动范围反馈 scroll_limit=max-min-450；缩略图滚动视图（Arc<Mutex<LoadMenuThumbnailMetrics>> 共享）；会话元数据装载+装载校验；备份排序（insertion_sort_shift_left/driftsort_main 阈值 0x15）；文本渲染链；tiny_http 单文件服务；拍照模式编排（NDC→世界逆投影拖拽+准星）；滑块归一化↔参数映射（inverse_lerp/param_to_value/按类型 switch）；LUT 画廊复用缩略图滚动；节点 id 派生 hash^0xb1695320004fb443；bug 报告 GPU 截图（MetaCommand 队列+rhapsody 分配器）。

## 未知清单
EscapeMenuJustClosed 发出点（NextEscMenuState::set 库外）；EscMenuState 完整变体表（2=设置/9=credits/0xd,0x13=photomode 已观察）；welcome +0xe5/+0x206 字段归属；各模板 id 字符串（DAT 串未解析）；report_bug 提交格式与 FeedbackReportResources 布局；setup_photomode_toolbar 标签串与 ToolbarItemKind 值；ui_slider/ui_slider_reset 被截断跳表分支；SingleFileServer 完整布局/Content-Type/路径来源；update_text_renderer_scale 计算式；flush_text_render 两块 SSBO 名。
