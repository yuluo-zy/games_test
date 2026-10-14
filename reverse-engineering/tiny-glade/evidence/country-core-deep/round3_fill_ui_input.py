# -*- coding: utf-8 -*-
# round3 chapter filler for ui-input domain (tg_a5).
# Updates evidence/country-core-deep/chapters/<file>.md in place:
#  - 文件作用 header
#  - per-business-function 作用/逆向结论/证据 columns (matched by VA)
#  - 内联函数 bullets (matched by name)
# Evidence: ui-input DIGEST/CLOSURE + decomp/*.txt + wp15/wp20 analysis + hex hand-decodes.
import io, json, os, re, sys

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"
DEC = "ui-input\\decomp\\decomp-"

# ---------- per-file knowledge ----------
# key: chapter path (posix, relative to CH). value: (file_purpose, {va: (作用, 结论, 证据)}, {inline_name: 结论})
# 结论 uses O/I/U markers.

KB = {}

KB["systems/mode_manager.rs.md"] = (
 "AppMode 模式状态机中枢：哈希 esc_menu_canvas 阻塞早退、排空 RequestAppModeChange 双缓冲队列、"
 "check_if_can_switch 校验/延后（pending=0xd Replay）、按模式 id 跳表分发到工具栏类别闭包 env_3..14、"
 "send_mode_just_changed 事件（prev/current AppMode 对）。AppMode 判别：2=Timelapse、6..9=Wall 形状、"
 "10+5k=Tool(类别 k，视觉序)、0xd=Replay；事件队列 = bevy 0.16 Events 双 VecDeque（EventInstance "
 "{id u64,event}，官方 registry 布局）。",
 {
  "0x140afbe10": ("序列化/serde", "O：AppMode serde_json deserialize visit_str（派生胶水过滤线内的业务锚，U 细节）", f"{DEC}0x140afbe10.txt"),
  "0x140afbba0": ("序列化/serde", "O：serde_json deserialize（AppMode 反序列化入口，Compact/Pretty 对应序列化侧）", ""),
  "0x1400368a0": ("序列化/serde", "O：AppMode CompactFormatter 序列化（u8 判别-0xfa 界 0x10 的跳表分发，变体名在跳表外）", f"{DEC}0x1400368a0.txt"),
  "0x140afc7c0": ("序列化/serde", "O：AppMode PrettyFormatter 序列化（同上，跳表 at rip+0x1fe99fc 区）", f"{DEC}0x140afc7c0.txt"),
  "0x140a86b40": ("谓词/校验", "O：AppMode::is_tool_selected 真身（SOURCE_MAP 行 87；重建代码写的 0x140a8a5d0 是 cleanup_timelapse，伪）", ""),
  "0x140afa8a0": ("check_if_can_switch 分发器", "O（round2 capstone 全解）：force→push {u32 4, 计数} 0x20B 记录；非 force→*模式状态=0xd(Replay)、push {id, 请求mode}、byte@1<2 再 push {id,byte}；byte@1>=2 = unreachable!() panic（toolbar.rs:40，串 0x142ae5650）。分发在 mode_manager 跳表块内", f"{DEC}0x140afa8a0.txt"),
  "0x140afa190": ("类别闭包 env_10", "O：工具栏类别 idx6(Water) 的模式索引推进（current+1 % len、Toolbar.current=N、拷 ModeEntry 0x10B 入 pending、cat==0x15 哨兵跳过）；force 段 push {4,计数}", f"{DEC}0x140afa190.txt"),
  "0x140afa640": ("类别闭包 env_11", "O：idx9(Animals) 同 env_10 模式（entry@0x130）", f"{DEC}0x140afa640.txt"),
  "0x140afa510": ("类别闭包 env_12", "O：idx5(Trees) 同 env_10 模式（entry@0xb0）", f"{DEC}0x140afa510.txt"),
  "0x140afa060": ("类别闭包 env_13", "O：idx8(Meadow) 同 env_10 模式（entry@0x110）", f"{DEC}0x140afa060.txt"),
  "0x140afa9f0": ("类别闭包 env_14", "O：idx10(Terrain)：请求.cat==0x13 时清零 cat 直调 switch_mode；否则同 env_10（entry@0x150）；越界 panic FUN_1428d9518", f"{DEC}0x140afa9f0.txt"),
  "0x140afa2c0": ("类别闭包 env_3", "O：idx7(cat0 空类别)：switch_mode 直调路径 + 同模式推进", f"{DEC}0x140afa2c0.txt"),
  "0x140afa770": ("类别闭包 env_4", "O：动态类别变体（类别 id 取自参数 bVar13），索引推进同族", f"{DEC}0x140afa770.txt"),
  "0x140afab70": ("类别闭包 env_5", "O：idx0(Buildings)：`current != 0` 判定的 idx0 变体（entry@+0x10）", f"{DEC}0x140afab70.txt"),
  "0x140afa3f0": ("类别闭包 env_6", "O：idx1(Walls)（byte==1 判定）", f"{DEC}0x140afa3f0.txt"),
  "0x140afadb0": ("类别闭包 env_7", "O：idx2(Decor)（byte==2）", f"{DEC}0x140afadb0.txt"),
  "0x140afac90": ("类别闭包 env_8", "O：idx3(Misc)（byte==3）", f"{DEC}0x140afac90.txt"),
  "0x140af9f30": ("类别闭包 env_9", "O：idx4(Paths)（byte==4）", f"{DEC}0x140af9f30.txt"),
  "0x140af9e00": ("start_timelapse 的检查闭包 env_0", "O：非 force：写 pending AppMode 对、组 40B 记录调 FUN_140b59be0（timelapse 元命令，meta_commander.rs）、推 {u32 1, f32 时标} 事件、清标志字节", f"{DEC}0x140af9e00.txt"),
  "0x140a86b60": ("系统:周期性处理（mode_manager）", "O：hash('esc_menu_canvas') 早退（hover+id+交点 tag!=2 且 mode!=2）→ 排空请求队列 → check_if_can_switch → 按状态@+0x80 模式 id 走跳表 @0x142ad890c（0..64 → 函数体各块；mode=10+5k 直调 env 闭包）", f"{DEC}0x140a86b60.txt"),
  "0x140a875e0": ("系统:事件发射（send_mode_just_changed_event，PDB 名 closure$0）", "O：prev/current AppMode ne → push 0x28B {prev 16B, current 16B, id}，current:=prev。载荷是双 AppMode 事件（非 RequestAppModeChange）", f"{DEC}0x140a875e0.txt"),
 },
 {
  "check_if_can_switch": "O（round2）：泛型校验器——force 时推 {4,id} 重置事件；否则置 pending=0xd(Replay) 并推 {id, 请求mode}；byte@1>=2 为 unreachable panic。per-mode 逻辑在各闭包",
  "switch_app_mode": "I：按名推断 = 写入 AppMode 状态 + PrevAppMode 更新的内联核心（逻辑并入 mode_manager 跳表块，U 独立地址）",
 })

KB["systems/mod_key.rs.md"] = (
 "Modifiers 修饰键状态与吸附禁用开关：update_mod_key_state 系统读 ButtonInput<KeyCode>，按 7 组键"
 "跳表计算 ctrl/shift 组合 + 工具栏 Decor 模式区间（0x38..0x3f）判定，写 Modifiers{u32@0, byte@1="
 "disable_snapping}（含 f32 阈值/按键沿触发）。",
 {
  "0x1408c5680": ("谓词/校验", "O：Modifiers::disable_snapping（ICF 折叠单行，与 ActiveInputMode::is_controller 同码；语义=读位标志组）", "wall\\decomp\\decomp-0x1408c5680.txt"),
  "0x140af9bf0": ("谓词/校验", "O（手解）：`0f b6 41 01 c3` = 返回 byte [rcx+1] —— 位标志组第 1 字节某位（SOURCE_MAP 归名 is_disable_snapping_just_pressed）", f"{DEC}0x140af9bf0.txt"),
  "0x140af9c00": ("系统:周期性处理（update_mod_key_state）", "O（手解）：ResMut<Modifiers> tick 回写；读键态 @+0x78 计数与 f32@+0x7c 阈值；键码索引 0..6 跳表 7 handler；Decor 模式 0x38..0x3f 区间判定并入；写 {u32@0, byte@1}。SOURCE_MAP 地址（重建代码写的 0x140af9000 = Toolbar::new，错）", ""),
 })

KB["systems/backend/input/calculate_input_delta.rs.md"] = (
 "输入增量计算：对每个 analog action 按活动/死区状态（kind 0/1/2 = released/moving/deadzone）"
 "累加 cursor delta（死区 0.1/0.05），输出 {tag,value} 对 + 10 qword 原始镜像（wp20 合同复核）。",
 {
  "0x1416feeb0": ("系统:输入处理", "O：wp20 既有反编译（analysis/0x1416feeb0 同名）+ 合同复核：kind2 走 +0.1/0.05 死区、kind1 累加 value+prev_delta、kind0 保留 prev；输出区 @+0xa0 起 10 qword 镜像", "wp20/analysis/0x1416feeb0_calculate_input_delta.txt"),
 })

KB["systems/backend/input/input_record_replay.rs.md"] = (
 "输入录制/回放状态机：mode@+0x40（^0x8000000000000000 后 case 0/2/3/4 = Idle/Replaying/"
 "Recording/Finished）。Idle 满足条件即 store camera + 克隆 ToolbarCategoryModes + 重建流；"
 "Replaying 按 0xd0B InputFrame 写回状态（@+0x30 u64/@+0x38 u32/@+0x3c u8/@+0x40 双 f32/"
 "@+0x44 除数/@+0x58..0xc0 十 qword）；越界 panic 'Out of bounds access'。",
 {
  "0x1409423d0": ("系统:输入处理", "O：完整状态机（见上）；流重建（frames 容量 8 + 1 sync point）；delta 经 2×2 矩阵 _UNK_142a171f0 + 偏移变换；advance 帧尽置 Finished(0x8000000000000004)", f"{DEC}0x1409423d0.txt"),
  "0x140941a30": ("键盘控制录制/回放", "O：control_input_replay（键控 mode 切换 + seek sync point + camera 参数存取）", f"{DEC}0x140941a30.txt"),
  "0x140942280": ("推进回放帧", "O：advance_input_replay_frame：Replaying 时 index@+0x48++，帧尽→Finished", f"{DEC}0x140942280.txt"),
 })

KB["systems/backend/input/input_replay_for_trailers.rs.md"] = (
 "trailer 用输入录制/回放：配置字节@+0xfc==1 且命中按键表时在 Idle/Recording/Replaying 间切换"
 "（'Start recording'/'Finished recording'/'Replay...' 日志），do_input_replay_ui 提供 egui 面板"
 "（'Enable input recording'、'`O` - start/stop recording'、tmp/input_replay.json 保存）。",
 {
  "0x14083c620": ("系统:输入处理", "O：record_and_replay_input_for_trailers：键切状态 + MainCamera::store/load_parameters + 恢复 InputTranslationState", f"{DEC}0x14083c620.txt"),
  "0x14083cdd0": ("egui 面板", "O：do_input_replay_ui：checkbox 写 @+0xfc、'Stream frame count: ' 标签、'`O`/`E`' 键提示、Replay sheep 按钮", f"{DEC}0x14083cdd0.txt"),
 })

KB["systems/backend/input/reset_input_state.rs.md"] = (
 "输入状态复位：清零并重排 InputTranslationState（11 qword 置换）、按最后 sync point 修剪双 "
 "VecDeque、ButtonInput::reset_all<KeyCode>()（wp20 合同复核）。",
 {
  "0x140942b00": ("系统:输入处理", "O：wp20 合同复核通过（0x140942b00 无本域 decomp，证据=wp20 analysis）", "wp20/analysis/0x140942b00_reset_input_state.txt"),
 })

KB["systems/backend/input/translate_input.rs.md"] = (
 "输入映射主系统：Analog→ActionSetValues、Digital→ActionSetStates、MouseMoved 事件消费"
 "（wp20 合同复核，34KB 反编译已落盘）。",
 {
  "0x1417cd0a0": ("系统:输入处理", "O：输入映射主系统（wp20 合同 + 本域 34KB decomp 复核一致）", f"{DEC}0x1417cd0a0.txt"),
 })

KB["systems/backend/input/translate_input_to_cursor_moved.rs.md"] = (
 "录制事件双队列（stride 0x18，tag@8：0=mouse_moved/1=moved/2=跳过）→ 累加游标 delta，NaN 安全"
 "clamp，推 InputCursorMoved（0x10B {id, Vec2}，bevy 0.16 EventInstance {id@0, event@8} 布局）"
 "并累加进输出状态 @+0x58/+0x60。",
 {
  "0x140b17a80": ("系统:输入处理", "O：wp20 合同复核 + 本域 decomp：{计数@0, Vec2@8} push 证实官方 EventInstance 布局", f"{DEC}0x140b17a80.txt"),
 })

KB["systems/backend/input/update_cursor_position.rs.md"] = (
 "游标位置合并（全内联无独立符号）：把 InputCursorMoved 合并进游标位置资源。",
 {
  "0x140a86b60": None,  # placeholder never matched
 })

KB["systems/ui/mod.rs.md"] = (
 "country-core UI 面：GlobalId（u64 name-hash）、UiSystems::add_ui 插件（Update 调度链 + UiState/"
 "UiNavState(1e9ns)/UiSignifierStream(0x170B) 资源 + PlayUiSound/PlayPhotomodeSound 事件注册）。",
 {
  "0x140945c60": ("插件注册（add_ui）", "O（wp15 合同）：schedule UI 系统（chain_ignore_deferred + in_set_inner）、插 3 资源、注册 2 事件", "wp15/analysis/b4_startup/add_ui_0x140945c60.c"),
  "0x14092df30": ("构造（WidgetId::from_name）", "O：= utils::calculate_hash(name)（u64 name-hash 身份）", f"{DEC}0x14092df30.txt"),
 })

KB["systems/ui/ui_update.rs.md"] = (
 "UI 主状态机：遍历 UiState 注册表（HashMap<WidgetId, Arc<WidgetParams>>），每 widget 算 get_ui_volume"
 "（世界空间体积）、hover/click/drag 状态转移（HoverState/DragState，按键码 0x5c/0x2f 分支）、"
 "PlayUiSound 触发、每帧清交点表。",
 {
  "0x140b2de60": ("系统:周期性处理（ui_update）", "O（75KB decomp）：状态机主体（wp15 合同 + 本域复核）；细节分支见 decomp 行注释", f"{DEC}0x140b2de60.txt"),
  "0x140b2dc60": ("计算 widget 世界体积", "O：get_ui_volume（4 callers）——由 WidgetParams 计算世界空间体积", f"{DEC}0x140b2dc60.txt"),
 })

KB["systems/ui/hover_intersection.rs.md"] = (
 "悬停相交检测：par_chunk_map 射线×体积（(WidgetId,&WidgetParams)）→ Vec<WidgetIntersection> + "
 "按 layer/distance 排序的 Vec<(&WorldCanvasLayer, WidgetIntersection)>，结果写入 UiState 上下文。",
 {
  "0x1408383a0": ("系统:输入处理", "O（wp15 b5 既有）：并行体积测试 + 排序（sort_by + stable merge）", "wp15/analysis/b5_ui_state/hover_intersection_0x1408383a0.c"),
 })

KB["systems/ui/debug_visualize_volumes.rs.md"] = (
 "调试可视化：三角形体积线框/球体调试绘制 + 屋顶 UV 线框；DebugRoofs/DebugVolume 资源 + "
 "组件注册（bevy 胶水）。",
 {
  "0x141458c20": ("组件注册", "O：register_required_components（bevy ECS 注册胶水）", f"{DEC}0x141458c20.txt"),
  "0x1414558c0": ("调试绘制", "O：debug_visualize_volumes 主体（三角列表 → line/sphere debug viz）", f"{DEC}0x1414558c0.txt"),
 })

KB["systems/ui/resources/ui_state.rs.md"] = (
 "UiState 权威注册表：RwLock<HashMap<WidgetId, Arc<WidgetParams>>>（锁字@+0x60、表@+0x68）；"
 "UiWidgetContext（hover@0x18/click@0x19/id@0x38/交点载荷@0x28..0x58）；clearing_canvas 专用 "
 "widget（130×130 体积，tag 4）。",
 {
  "0x14092df50": ("注册 clearing_canvas widget", "O：体积 0x43020000²=130×130（tag 4）、哈希名、alloc 0xc0 Arc、RwLock 插表（1e9ns 写锁超时）", f"{DEC}0x14092df50.txt"),
  "0x140922fb0": ("UiState::add(name, params)", "O：54 callers——WidgetId::from_name + add_with_id", f"{DEC}0x140922fb0.txt"),
  "0x14092e3e0": ("UiState::add_with_id", "O：LOCK [p+0x60]==0→写 8 取写锁；拷贝 0xb0B WidgetParams；alloc 0xc0B Arc；HashMap@+0x68 insert", f"{DEC}0x14092e3e0.txt"),
  "0x14067dd80": ("UiState::widget 查询", "O：1177 callers（ICF 折叠——与 AssetMeshLibrary::transient_meshes/PendingEguiContextResource::into_egui_context_resource 同码；按 WidgetId 查 Arc<WidgetParams>）", f"{DEC}0x14067dd80.txt"),
  "0x140002450": ("active_widget_state", "O：折叠单行（NotFound），语义=取当前 active widget 状态对（U 细节）", f"{DEC}0x140002450.txt"),
  "0x14092e5f0": ("UiWidgetContext::clicked", "O：hover>1 && click<2 && click&1（wp15 合同）", f"{DEC}0x14092e5f0.txt"),
  "0x14092e620": ("UiWidgetContext::hovered", "O：hover 字节 != 0 且 id@+0x38 == ctx id（注意重建代码缺 id 匹配）", f"{DEC}0x14092e620.txt"),
  "0x14092e640": ("UiWidgetContext::intersecting", "O：交点载荷 7 qword 拷贝或 tag-2 None", f"{DEC}0x14092e640.txt"),
  "0x14092e680": ("UiWidgetContext::is_active", "O：hover!=0 && id 匹配（同 hovered）", f"{DEC}0x14092e680.txt"),
  "0x1406868b0": ("UiWidgetContext::id", "O：返回 ctx 的 WidgetId（NotFound 折叠，语义由调用点证实）", f"{DEC}0x1406868b0.txt"),
  "0x14092e6a0": ("ActiveWidget::is_dragging", "O：折叠单行（NotFound）", f"{DEC}0x14092e6a0.txt"),
 })

KB["systems/ui/resources/widget_params.rs.md"] = (
 "WidgetParams 0xb0B 参数块：默认 u32 组 {0,8,0,1,0,2,3,3}@{0,8,0x10,0x18,0x20,0x30,0x48,0x60}、"
 "UiSounds 4×24B、WidgetVolumeShape 32B {tag,layer,size}、u16@0xa8=0x101、flags 字节@0xa9"
 "（bit0=no_history_recording、bit2=allow_negative_distance）；UiFocus u32 {None=0,Toolbar=1,"
 "Mei=2,World=3}。",
 {
  "0x1409a8e10": ("构造（WidgetParams::new）", "O：0xb0B 默认值序列（wp15 合同 + 本域复核）", f"{DEC}0x1409a8e10.txt"),
  "0x1409a8e80": ("构造器（sounds）", "O：写 96B sounds 区 @0x18..0x78", f"{DEC}0x1409a8e80.txt"),
  "0x1409a8ee0": ("构造器（occlude_by_world）", "O：字节@0xa8 bit0", f"{DEC}0x1409a8ee0.txt"),
  "0x1409a8f10": ("构造器（no_history_recording）", "O：字节@0xa9 bit0", f"{DEC}0x1409a8f10.txt"),
  "0x1409a8f40": ("构造器（allow_negative_distance）", "O：字节@0xa9 bit2", f"{DEC}0x1409a8f40.txt"),
  "0x1409a9110": ("构造（WidgetVolumeShape::clearing_canvas）", "O：{tag 4, layer, 130×130} 32B", f"{DEC}0x1409a9110.txt"),
  "0x1409a9130": ("UiFocus::as_mei", "O：转 Mei（=2）", "wp15/analysis/b6_widgets/as_mei_0x1409a9130.c"),
  "0x1409a9140": ("UiFocus::is_mei（与 WelcomeScreenState::is_closed 折叠）", "O：`== 2`（u32 判别；折叠=两处语义巧合相同）", f"{DEC}0x1409a9140.txt"),
  "0x1409a9150": ("UiFocus::is_toolbar", "O：`== 1`（u32 判别）", f"{DEC}0x1409a9150.txt"),
  "0x1409a8f80": ("UiSounds::boop", "O：折叠单行（NotFound；boop 音效槽）", f"{DEC}0x1409a8f80.txt"),
  "0x1409a8fd0": ("UiSounds::on_hover_start", "O：折叠（NotFound；槽 0-2）", f"{DEC}0x1409a8fd0.txt"),
  "0x1409a9020": ("UiSounds::on_click", "O：折叠（NotFound；槽 9-11）", f"{DEC}0x1409a9020.txt"),
  "0x1409a90c0": ("UiSounds::on_drag_release", "O：折叠（NotFound；槽 6-8）", f"{DEC}0x1409a90c0.txt"),
 })

KB["systems/ui/resources/slider3d.rs.md"] = (
 "Slider3d：bevy SystemParam（646 new_archetype callers），包装 UiState/UiSignifierStream/"
 "CameraCursorRay/游标相机资源 + 拖拽 EventWriter；Slider3dState 世界空间线段拖拽状态；"
 "project_cursor_onto_line 游标投影。",
 {
  "0x14003bb80": ("SystemParam get_param/queue", "O：折叠（NotFound）——SystemParam 状态获取/入队（bevy 胶水面）", f"{DEC}0x14003bb80.txt"),
  "0x14092df10": ("SystemParam validate_param", "O：折叠（NotFound）——参数校验（与 UiWidgetContext::dragged 同址折叠）", f"{DEC}0x14092df10.txt"),
  "0x140a9bd40": ("游标→线段投影", "O：Slider3dState::project_cursor_onto_line（wp15 b7 既有）", "wp15/analysis/b7_signifiers/project_cursor_onto_line_0x140a9bd40.c"),
  "0x140b1cc80": ("Slider3d::update<C,F>", "O：泛型 update（widget 上下文 + 拖拽闭包；wp15 b7 既有）", "wp15/analysis/b7_signifiers/update_enum2_country_core_systems_ui_resources_ui_state_DragState_country_core_systems_wall_shapes_rectangle_ui_edit_rectangle_dims_do_ui_edit_rectangle_dims_closure_env_0__0x140b1cc80.c"),
 })

KB["systems/ui/signifiers/mod.rs.md"] = (
 "Signifier 指示符工厂族：SignifierEntityBuilder 0x90B（默认 shader 'vertex_color_shader'@0x60..0x70、"
 "position Vec2@0x7c、默认 {0,1.0f64,0x100 u16@0x40,0.2f32@0x78}）；build 三分支（0=命名网格/"
 "1=transient/2=mesh_from_pts）生成实体 + SignifierAnimation；SignifierFactory 40B。",
 {
  "0x14089d9b0": ("构造（SignifierEntityBuilder::new）", "O（wp15 b6）：35 callers，默认值序列见文件作用", "wp15/analysis/b6_widgets/new_0x14089d9b0.c"),
  "0x14089dad0": ("set_ui_pass", "O：字节@0x8a 置位（UI pass 标志）", ""),
  "0x14089db00": ("custom_shader", "O：String 写 @0x60..0x70", "wp15/analysis/b6_widgets/custom_shader_0x14089db00.c"),
  "0x14089dc00": ("position", "O：Vec2 @0x7c（24 callers）", ""),
  "0x14089dc30": ("rotation", "O：f32 写（13 callers）", "wp15/analysis/b7_signifiers/rotation_0x14089dc30.c"),
  "0x14089dc60": ("scale", "O：f32 写（21 callers）", "wp15/analysis/b7_signifiers/scale_0x14089dc60.c"),
  "0x14089dc90": ("animation_duration", "O：f32 写（35 callers）", "wp15/analysis/b7_signifiers/animation_duration_0x14089dc90.c"),
  "0x14089dcc0": ("minimum_lifespan", "O：f32 写", "wp15/analysis/b7_signifiers/minimum_lifespan_0x14089dcc0.c"),
  "0x14089dcf0": ("animation_easing", "O：f32 写", "wp15/analysis/b7_signifiers/animation_easing_0x14089dcf0.c"),
  "0x14089dd20": ("parent_to_widget", "O：父挂标志（1 caller）", "wp15/analysis/b7_signifiers/parent_to_widget_0x14089dd20.c"),
  "0x14089dd50": ("build（生成 signifier 实体）", "O：91 callers——网格选择器三分支 + spawn + SignifierAnimation", "wp15/analysis/b7_signifiers/build_0x14089dd50.c"),
  "0x1408a0f00": ("组件注册", "O：register_required_components（bevy 胶水）", ""),
 })

KB["systems/ui/signifiers/signifier_create.rs.md"] = (
 "UiSignifierStream：0x170B 无锁环形流（read cursor@+0x160、write counter@+0x168、0x28B 槽）；"
 "write 自旋追加；signifier_create 每帧排空并生成指示符实体。（重建代码写成 Vec<UiAppendSignifier>"
 "是错误的——真实是环形流。）",
 {
  "0x140b2cf90": ("UiSignifierStream::write", "O（wp15 b7）：拷贝 0x28B 记录进槽 + 自旋推进游标（36 callers）", "wp15/analysis/b7_signifiers/write_0x140b2cf90.c"),
  "0x140b2cfa0": ("系统:排空生成（signifier_create）", "O（wp15 b7）：drain → spawn signifier 实体", "wp15/analysis/b7_signifiers/signifier_create_0x140b2cfa0.c"),
 })

KB["systems/ui/signifiers/signifier_mark_for_delete.rs.md"] = (
 "指示符删除：signfier_mark_for_delete（PDB 拼写如此）按 signifier_config_hash + UiState 标记删除；"
 "despawn_marked 销毁 SignifierMarkedForDeletion 实体。",
 {
  "0x140929160": ("标记删除", "O（wp15 b7）：2 callers", "wp15/analysis/b7_signifiers/signfier_mark_for_delete_0x140929160.c"),
  "0x140929a90": ("销毁已标记实体", "O（wp15 b7）：3 callers", "wp15/analysis/b7_signifiers/despawn_marked_0x140929a90.c"),
 })

KB["systems/ui/signifiers/animation.rs.md"] = (
 "指示符动画：signifier_animation 推进 SignifierAnimation::t 并按缓动 scale/alpha（父挂体积原点用 "
 "WidgetVolumeShape::get_origin）；signifier_animation_t 每帧 tick。",
 {
  "0x140a69c30": ("系统:周期性处理（signifier_animation）", "O（wp15 b7）：2 callers", "wp15/analysis/b7_signifiers/signifier_animation_0x140a69c30.c"),
  "0x140a69340": ("动画时间 tick", "O（wp15 b7）：2 callers", "wp15/analysis/b7_signifiers/signifier_animation_t_0x140a69340.c"),
 })

KB["systems/ui_edit_focus.rs.md"] = (
 "编辑焦点系统：按模式 id 分发（mode<6 注册 'edit_focus_canvas' widget（tag5/f32::MAX）；mode 1/2 "
 "跟随路径过 FUN_140a1c930 谓词；0x2a8 反向步长遍历 wall 数组并断言 simplified_curve_2d 有效性；"
 "mode==10 特判；4 张旋转跳表 = 24 变体 GeoSource 匹配；结尾推 {tag0, 计数} 0x18B 事件）。"
 "geo_src_to_edit_subject(_w_deco_clutter) 做 GeoSource→EditSubject 映射。",
 {
  "0x1409771f0": ("系统:周期性处理（ui_edit_focus）", "O：完整结构（DIGEST §2.5）；两条 panic 串：'assertion failed: w.simplified_curve_2d.is_valid()'（0x142ada193）+'internal error: entered unreachable code: ui_edit_focus shouldn't include '（0x142ab6a70）", f"{DEC}0x1409771f0.txt"),
  "0x140977f50": ("GeoSource→EditSubject 映射", "O：172KB（含内联）；EditSubject 变体 serde 名全套（WallId/wall_id/Floor/Roof/flat/gable/StairGraphEdge/DecoratorCollision/DecoratorInteraction/StairInteraction/ColumnStairs/Fence/Arches/Tree/Terrain/AutoClutter/entity 等，串 @0x142ab7268 起）", f"{DEC}0x140977f50.txt"),
 })

KB["systems/ui_move_shapes.rs.md"] = (
 "移动形状 gizmo：widget 名 'move_shape'；ui_move_shapes 主系统（12.8KB）处理 gizmo 拖拽→形状位置；"
 "move_gizmo_and_trigger_vol 施加 delta 并触发 volume 重算；MoveShapeCache 记录在途移动。",
 {
  "0x14097a060": ("系统:周期性处理（ui_move_shapes）", "O：12.8KB 主系统（wp20 + 本域复核；'move_shape' widget @0x142ab6c90）", f"{DEC}0x14097a060.txt"),
  "0x14097d280": ("施加 delta + volume 触发", "O：move_gizmo_and_trigger_vol（wp20 既有）", "wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt"),
 })

KB["systems/ui_raise_shape.rs.md"] = (
 "抬升形状：widget 名 'outline_wall_bottom_'/'arrow_raise_shape_'/'raise_wall_elevation_'；"
 "impl_raise_shape 用 ProportionalScreenSpaceCoord::project_onto_ray 算抬升量 + History::add 记录。",
 {
  "0x140952e70": ("系统:周期性处理（ui_raise_shape）", "O：8.7KB 主系统（wp20 + 本域复核）", f"{DEC}0x140952e70.txt"),
  "0x140955650": ("抬升实现", "O：impl_raise_shape（project_onto_ray + 高度 delta + History::add；wp20 既有）", "wp20/analysis/0x140955650_impl_raise_shape.txt"),
 })

KB["systems/ui_2d_follow_mouse.rs.md"] = (
 "2D UI 标记跟随鼠标：双分支 Query（标志@+0x108），源索引 @+0x94（stride 0xa0）/@+0x10（stride "
 "0x48）选每实体槽，写入游标 Vec2 + u32 layer（wp20 合同复核）。",
 {
  "0x14092a100": ("系统:周期性处理（ui_2d_follow_mouse）", "O：puffin scope（函数全名）+ 双分支写槽（wp20 + 本域复核）", f"{DEC}0x14092a100.txt"),
 })

KB["systems/ui_camera_mode_gizmos.rs.md"] = (
 "相机模式 gizmo：模式门（mode==2 或 mode==10 带标志）且相机模式字节@+0x4d0 非 Free（(byte&6)!=2）"
 "且窗口过小（w/h<=2）时：UiState::add 注册 'camera_mode_active' widget + AppendOnlyVec 推 "
 "'rotation_' gizmo 指示符 {tag,size,pos,fov}。",
 {
  "0x1409527a0": ("系统:周期性处理（ui_camera_mode_gizmos）", "O：widget 'camera_mode_active'（0x12 字符）+ 工具模式字节@+0x11c 位测 {0,1,4}（wp20 + 本域复核）", f"{DEC}0x1409527a0.txt"),
  "0x140952b60": ("closure$0", "O：'rotation_' signifier 构建闭包（FUN_141281be0 环境）", f"{DEC}0x140952b60.txt"),
 })

KB["systems/ui_edit_freehand_walls.rs.md"] = (
 "自由手绘墙编辑：拖拽笔划进 EditFreehandWallCache，松手提交墙段给 wall-constructor（wp20 合同）。",
 {
  "0x140ac98e0": ("系统:周期性处理（ui_edit_freehand_walls）", "O：12.1KB（wp20 + 本域复核一致；细节分支未逐行）", f"{DEC}0x140ac98e0.txt"),
 })

KB["systems/ui_focus_mode_shape_height.rs.md"] = (
 "焦点模式形状高度 gizmo：游标射线投影调整所选形状高度，LerpedGizmoPosition 记录 lerp 位置"
 "（wp20 合同）。",
 {
  "0x140a623e0": ("系统:周期性处理（ui_focus_mode_shape_height）", "O：9.2KB（wp20 + 本域复核一致）", f"{DEC}0x140a623e0.txt"),
 })

KB["systems/ui_move_freehand_walls.rs.md"] = (
 "移动自由墙：拖拽移动选中自由墙段，MoveWallCache 记录在途移动（wp20 合同）。",
 {
  "0x1409781d0": ("系统:周期性处理（ui_move_freehand_walls）", "O：7.8KB（wp20 既有复核）", "wp20/analysis/0x1409781d0_ui_move_freehand_walls.txt"),
 })

KB["resources/toolbar.rs.md"] = (
 "工具栏资源：Toolbar {categories: Vec<ToolbarCategoryModes> (cap,ptr,len), current u8@0x18}；"
 "ToolbarCategoryModes 0x20B {modes Vec, current u64@0x18}；ModeEntry 0x10B {cat u32, mode u8, "
 "pad, arg u64}。new 建 11 类（含 id0 类别；Roofs 为 Decor 0x38..0x3f 区间模式）；contains 的 "
 "装饰区间 = 0x38..=0x3f；switch_mode 入参 = (&mut Toolbar, &record 16B)；serde 按类别名 "
 "（'Path'…）序列化（input replay 持久化）。",
 {
  "0x140af8c50": ("构造（Toolbar::new）", "O(部分)：SOURCE_MAP 地址；此变体 alloc 0x30×8B 元素表（与 0x140af9000 的 11 类 0x10B 表不同——两地址为 new 的不同单态/内联，关系待核 I）", ""),
  "0x140af9000": ("构造（Toolbar::new 完整体）", "O（wp20 既有）：11 类表（非 demo/demo 对比）、{len=0xb, ptr, cap=0xb, byte@0x18=0} 结构尾写", "wp20/analysis/0x140af9000_new.txt"),
  "0x140af89d0": ("ToolbarCategoryModes::contains", "O（手解）：入参 &{cat u32, mode u8@4}；SIMD 16B 块遍历 0x10 步长；cat<=5 时 mode 区间 0x38..=0x3f（mode-0x38<8）", ""),
  "0x140af9850": ("Toolbar::switch_mode", "O（手解）：(&mut Toolbar, &{cat u32, mode u8, arg u64}) 逐类 contains 后写 current；panic 串含 'mode but no toolbar'（0x142ae3cbc 区，完整原文 U）", ""),
  "0x140afb5b0": ("序列化/serde", "O：visit_str 比较 'Path'（类别名序列化）；toolbar 持久化供 input replay", f"{DEC}0x140afb5b0.txt"),
  "0x140afb280": ("序列化/serde", "I：ToolbarCategoryModes deserialize（serde_json，细节并入 visit_str）", ""),
  "0x140afaed0": ("序列化/serde", "I：serde_json Pretty serialize（类别名写出；Compact 对应 0x1404f5100）", ""),
  "0x1404f5100": ("序列化/serde", "I：serde_json Compact serialize（对称 Pretty）", ""),
 },
 {
  "ToolbarCategoryModes::next_mode": "I：类别内下一模式（(current+1)%len 逻辑已并入 check_if_can_switch 闭包；PDB 名错位——0x140afb590 实为 serde unknown_variant thunk）",
  "ToolbarCategoryModes::last_selected_mode": "I：读 categories[i].current（内联进闭包/switch_mode）",
  "impl$1::from": "I：ToolbarCategory 转换构造（id→枚举，From 派生）",
  "Toolbar::switch_category": "I：按类别 id 找 Vec 下标写 current（内联进闭包 'byte@0x18=N' 写）",
 })

KB["resources/photo_mode.rs.md"] = (
 "拍照模式资源：PhotoModeCameraMode u8 {0..3}（next 0→1→2→3→0；next_demo 3→panic 'this camera "
 "mode is not part of the demo!'）；PhotoModeCmd 事件；PhotoModePlugin 注册；PhotomodeState::"
 "on_enable_photomode f32@0=3.0/1.0(demo)+u32@4=-1。与 CameraMode 槽 6..12 联动见 round2 CLOSURE"
 "（写入点在 toolbar::photomode，skeleton 缺该 crate）。",
 {
  "0x140b59640": ("插件注册（PhotoModePlugin build）", "O：register_event + 组件注册（bevy event/registry.rs 路径证据）", f"{DEC}0x140b59640.txt"),
  "0x140b596b0": ("循环下一模式（next_demo）", "O：u8 判别 0→1→2→0；3→panic（串 0x142af1188，photo_mode.rs）", f"{DEC}0x140b596b0.txt"),
  "0x140870570": ("PhotomodeState::on_enable_photomode", "O（wp15 b10）：f32@0=3.0（demo 1.0）、u32@4=-1、byte@10=0、byte@11=0", "wp15/analysis/b10_startup2/on_enable_photomode_0x140870570.c"),
 })

KB["resources/mouse_capture.rs.md"] = (
 "光标捕获命令处理：读 CursorCaptureCmd（utils::get_last_event 取最后一条）；未捕获→"
 "CursorCapturedPosition={1,pos}+WindowFocusEvent{focused=0}+EscMenuState{2,2}；捕获→{0}+"
 "{focused=1}+EscMenuState{3,2}。命令族 CursorCaptureCmd/OsCursorCaptureCmd/OsCursorVisibilityCmd/"
 "CustomCursorSsboCreatedCmd。",
 {
  "0x140b17830": ("系统:事件处理（mouse_capture）", "O：完整语义（DIGEST §2.6 + round2 CLOSURE）；EscMenuState 值 0x200000002/3 = {sub2,disc2}/{sub2,disc3} 布局待中央定序", f"{DEC}0x140b17830.txt"),
 },
 {
  "CursorCapturedPosition::new": "I：{captured=1, pos}（内联进 mouse_capture）",
  "CursorCapturedPosition::is_currently_captured": "I：captured 标志读（内联）",
 })

KB["resources/onboarding.rs.md"] = (
 "新手引导：WelcomeScreenState（is_closed = 判别==2，0x1409a9140 折叠处证实）+ NotifyHintSystemSnapping "
 "事件 + TutorialOp 事件（tiny_glade settings 系统消费，PDB 注解证实）。",
 {
  "0x1409a9140": ("WelcomeScreenState::is_closed", "O（手解）：`cmp dword [rcx],2; sete al`（与 UiFocus::is_mei 同址折叠）", f"{DEC}0x1409a9140.txt"),
 })

KB["resources/gui.rs.md"] = (
 "GUI 状态资源族：EscMenuState 8B {sub u32@0, disc u32@4}（is_open: qword∉{2,0xa} 或 0xa 时 sub!=1；"
 "is_closed: qword==2 或 (0xa 且 sub!=0xff)；cast_ui_shadows: qword∈{3,4} 或 (disc7&&sub1)；"
 "should_show_world: 0xfe..0x106 状态位掩码 0xaf/0x50 判定；can_limit_frame_rate: !(disc7&&sub1)）；"
 "NextEscMenuState::set（哨兵 0xb；字符串@+0x30 释放 + 16B@+0x50 拷贝）、PrevEscMenuState::get"
 "（0xb→None）；LoadMenuThumbnailMetrics、VideoSettingsSubMenu、DebugGui::log_to_screen、"
 "ShowDebugRaycastGeo、DebugTriggerProxyVis。",
 {
  "0x14093ffd0": ("DebugTriggerProxyVis::as_string", "O(结构)：u8 索引串表（FUN_1428d8fe3 长度 0x12/0xe/0x10 查询 + 不可恢复跳表 match），生成调试代理可见性描述", f"{DEC}0x14093ffd0.txt"),
  "0x14067dd80": ("PendingEguiContextResource::into_egui_context_resource", "O：ICF 折叠（与 UiState::widget 同码；egui Context 转换桥）", f"{DEC}0x14067dd80.txt"),
  "0x14093fad0": ("NextEscMenuState::set", "O（手解）：当前状态==0xb 或 ∈{0xfe..0x106} 且 !=4 时走不同分支；释放 self+0x30 字符串资源；拷贝新状态 {qword@0, 16B@+0x50}", ""),
  "0x14093fbb0": ("PrevEscMenuState::get", "O（手解）：状态==0xb → None（返回 null），否则 Some(&self)——0xb 为哨兵", ""),
  "0x14093fbc0": ("EscMenuState::is_open", "O（手解）：qword∈{2,0xa} 之外→true；==2→false；==0xa→(sub!=1)（mouse_capture 推 {sub2,disc3} 时 true ✓）", ""),
  "0x14093fbe0": ("EscMenuState::is_closed", "O（手解）：qword==2→true；==0xa→(sub!=0xff)；其余 false（与 is_open 互补）", ""),
  "0x14093fc00": ("EscMenuState::cast_ui_shadows", "O（手解）：qword∈{3,4}（单位变体）或 (disc==7 && sub==1)——注：sysres-procedures 把此地址标为 is_open，SOURCE_MAP 归 cast_ui_shadows，以 SOURCE_MAP 为准", f"{DEC}0x14093fc00.txt"),
  "0x14093fc20": ("EscMenuState::should_show_world", "O（手解）：qword-0xfe<9 时按 0xaf 位掩码（位 4/6→true）、再按 0x50 位掩码（位 4/6→(sub+1)）判定；越界按位 4(=0)→true", ""),
  "0x14093fc60": ("EscMenuState::can_limit_frame_rate", "O（手解）：!(disc==7 && sub==1)（SOURCE_MAP 名；sysres 清单标为 allow_time_passage——同名异址错位）", f"{DEC}0x14093fc60.txt"),
  "0x14093fc70": ("LoadMenuThumbnailMetrics::positions", "O（手解）：构造 4×u64 {…}（缩略图位置度量构造，无分支）", ""),
  "0x14093fcd0": ("VideoSettingsSubMenu::is_in_submenu", "O（手解）：`cmp byte [rcx],0; sete al` = 判别 0", ""),
  "0x14093fe50": ("DebugGui::log_to_screen", "O（手解结构）：读全局开关字节（rip+0x2cf4d80 区）→ 格式化行追加（屏上调试日志）", ""),
  "0x140002370": ("impl$0::first", "O：折叠单行（NotFound；容器 first 元素）", f"{DEC}0x140002370.txt"),
  "0x140940170": ("ShowDebugRaycastGeo::as_string", "O（手解结构）：`movzx eax,[rcx]; 两张相对偏移表 {rel_ptr u32, rel_len i32} 查表拼 &str`（表址校准 U）", ""),
 },
 {
  "EscMenuState::allow_time_passage": "I：内联；语义由 can_limit_frame_rate（0x14093fc60 同码区）佐证：!(disc7&&sub1)",
  "EscMenuState::can_use_static_world_capture": "I：按名推断——菜单静态世界截图可用性谓词（内联）",
  "EscMenuState::allow_painterly_overlay": "I：按名推断——笔触叠加层开关谓词（内联）",
 })

KB["resources/ui_settings.rs.md"] = (
 "UI 设置：UiSettings 7 字段 {icon_size_mult f32@0, custom_cursor_size f32@4, toolbar_alignment "
 "u8@8, auto_hide_toolbar u8@9, input_lag_reduction u8@10, game_logo u8@11, custom_cursor u8@12}；"
 "ToolbarAlignment = Bottom/Left/Right/Top（串表 0x142acec66）；默认 {1.0, 24.0, 0x101, 0}"
 "（auto_hide=true）。pretty serde_json 持久化。",
 {
  "0x140a46ad0": ("序列化/serde", "O：pretty serde_json 字段序 = 布局表（7 字段串 @0x142acec66 起）", f"{DEC}0x140a46ad0.txt"),
  "0x140b12f30": ("previous", "U：NotFound（折叠）；语义待证", f"{DEC}0x140b12f30.txt"),
 })

KB["resources/editing_highlight.rs.md"] = (
 "编辑高亮资源：EditingHighlight（wall_highlight 写 rgba），facade。",
 {
  "0x14093fc70": None,
 })

# files with no business-function table needing fills beyond header (ui_clutter family etc.)
KB["systems/ui_clutter/debug_spawn_clutter.rs.md"] = (
 "调试批量生成 clutter：消费 DebugSpawnClutterCmd/DebugSpawnAllClutterCmd 事件批量生成。",
 {
  "0x1409a1f50": ("系统:事件处理（debug_spawn_all_clutter）", "O：消费两命令批量 spawn（wp20 合同复核）", f"{DEC}0x1409a1f50.txt"),
 })

KB["systems/ui_clutter/decoclutter_show_pins.rs.md"] = (
 "显示/隐藏已固定 deco-clutter 图钉（DecoClutterLock、ShowPinnedDecoClutter；wp20 合同）。",
 {
  "0x1412357c0": ("系统:周期性处理（decoclutter_show_pins）", "O：6.7KB（wp20 + 本域复核）", f"{DEC}0x1412357c0.txt"),
 })

KB["systems/ui_clutter/defered_autoclutter_history.rs.md"] = (
 "延迟记录 auto-clutter 历史（RecordAutoClutterHistory 事件 → history）。",
 {
  "0x1409aa3e0": ("系统:事件处理（history_record_auto_clutter）", "O：消费事件入 history（wp20 合同）", "wp20/analysis/0x1409aa3e0_ui_focus_decoclutter.txt"),
  "0x1409a9160": ("history_record_auto_clutter", "O（wp20 既有；346B）", ""),
 })

KB["systems/ui_clutter/rmb_autoclutter.rs.md"] = (
 "右键移除 auto-clutter（handle_rmb_autoclutter；wp20 合同）。",
 {
  "0x140976ca0": ("系统:输入处理（handle_rmb_autoclutter）", "O：1.4KB（wp20 + 本域复核）", f"{DEC}0x140976ca0.txt"),
 })

KB["systems/ui_clutter/ui_autoclutter.rs.md"] = (
 "右键把 auto-clutter 转 deco-clutter（InstaCreateDeco/InstaCreateStairPoint/StartPlaceDecoClutter"
 "Interaction/EvRightClickedAutoClutter；wp20 合同）。",
 {
  "0x14099ca60": ("系统:输入处理（ui_convert_autoclutter_to_decoclutter）", "O：8.1KB（wp20 + 本域复核）", f"{DEC}0x14099ca60.txt"),
 })

KB["systems/ui_clutter/ui_focus_decoclutter.rs.md"] = (
 "焦点模式 deco-clutter 移动/旋转（RequestMoveClutter/RequestRotateDecorator/LastClonedClutter，"
 "4 个 closure；wp20 合同）。",
 {
  "0x1409a92c0": ("系统:周期性处理（ui_focus_decoclutter）", "O：11.3KB（wp20 + 本域复核）", f"{DEC}0x1409a92c0.txt"),
 })

# ---------------- writer ----------------
def update_chapter(rel, purpose, fns, inlines):
    path = os.path.join(CH, rel.replace("/", os.sep))
    with io.open(path, "r", encoding="utf-8") as f:
        text = f.read()
    orig = text
    # 1) purpose line
    text = re.sub(r"^> 文件作用：\*\*待推断\*\*（由章节 agent 按函数名\+证据填写）",
                  f"> 文件作用：{purpose}", text, count=1, flags=re.M)
    # 2) business rows: match by VA column
    def fix_row(m):
        row = m.group(0)
        # extract VA (2nd column)
        cols = [c.strip() for c in row.strip().strip("|").split("|")]
        if len(cols) < 6:
            return row
        va = cols[1].lower()
        entry = fns.get(va)
        if not entry:
            return row
        act, concl, ev = entry
        # rebuild row: keep 函数/地址/源码行, replace 作用/结论/证据
        newcols = cols[:3] + [act, concl, ev or ""]
        return "| " + " | ".join(newcols) + " |"
    text = re.sub(r"^\| `[^`]+` \| 0x[0-9a-f]+ \| [0-9]+ \| [^|]*\| [^|]*\| [^|]*\|$",
                  fix_row, text, flags=re.M)
    # 3) inline bullets
    if inlines:
        def fix_inline(m):
            line = m.group(0)
            for name, concl in inlines.items():
                if name in line and "— 待推断" in line:
                    return line.replace("— 待推断", f"— {concl}")
            return line
        text = re.sub(r"^- `[^`]+`（行 [0-9]+）— 待推断$", fix_inline, text, flags=re.M)
    if text != orig:
        with io.open(path, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        return True
    return False

done, missed = [], []
for rel, v in KB.items():
    purpose, fns = v[0], v[1]
    inlines = v[2] if len(v) > 2 else {}
    if os.path.exists(os.path.join(CH, rel.replace("/", os.sep))):
        if update_chapter(rel, purpose, fns, inlines):
            done.append(rel)
        else:
            missed.append(rel + " (no change)")
    else:
        missed.append(rel + " (missing file)")

print("UPDATED:", len(done))
for d in done:
    print("  ", d)
print("SKIPPED:")
for m in missed:
    print("  ", m)
