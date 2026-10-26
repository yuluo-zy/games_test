# Toolbar TB-R7：Demo 空间/高度提示及真实抬升生产链

本轮按 B 原职责恢复，不改 rhapsody / render_host / render / SSBO，也不以平台SDK或真实OS输入作为本轮验收。真实输入已由用户暂停；Steam SDK由用户明确排除。

## 原证与责任链

| 原函数 | 原地址/范围 | 恢复职责 |
|---|---|---|
| demo_show_full_glade_size | 141eac540，3428 B | Demo/菜单/摄影门禁，视线地面距离，完整地块虚线工厂，真实地形边界采样，标签与锚点动画 |
| full-size closure | 141ead870 | 实际闭包工厂、原 dashed_line shader / Alt(5) 以及虚线长度 |
| demo_show_max_elevation | 14234a410，2434 B | Esc 不消费、取最后原Shape事件，真实地形起点、绝对60上限、三条实际线工厂与原提示模板 |
| ui_create_circle / rectangle | 14165c0e6..14165c197 / 14155d5a1..14155d658 | ShapePlacement.elevation（原+3c）>=60时，在无效光标门禁前发完整28B ShapeParameters事件；不是单位事件或UI代发 |
| ui_raise_shape | 140952e70，8742 B | Editing(None)入口、所有预制建筑原prepared Curve3、底边SphereCloud、真实轮廓和箭头工厂、活动Widget事件 |
| raise closures 2/4 | 1409550a0 / 1409553f0 | 真实Points与brush_arrow_flat网格、默认shader与零动画时长、原Alt(3)轮廓拓扑 |
| impl_raise_shape | 140955650，4211 B | 原Started/Held/Finished分支、真实墙及Terrain装饰物归档、光标射线投影、权威高度、历史、sheep命令、材质/建筑音频与Max事件 |
| project_point_to_screen_space | 140ac8780..140ac882e，174 B | 完整原指令差分；第一次矩阵生成的w不传播，负w/零w/零窗口不制造替代结果 |

FocusHeight、ShapePlacement、add_wall_shape_ux 完整调度分别由独立单owner恢复，沿用 HandleRaiseShapeUiInteraction 同一事件身份。最终注册不由本报告冒称完成。归档真实碰撞体/网格由 `decorator_archivist.rs` 原owner提供，impl_raise实际传递 DecoratorArchivalResources 和原 source="change_elevation"。

## 保留的原业务与边界

- Max事件携带全部形状参数，圆的旋转、矩形的axis/dims均保留。ShapePlacement.elevation等于60和正无穷都会发，NaN及小于60不发；这两producer没有擅加Demo门禁。
- 空间提示仅Demo、关闭Esc且非摄影模式渲染；距离<=70清除锚点，NaN保持原有分支。Demo边界必须是原Circle15，普通世界是原Rectangle130，入口读取启动Demo快照，不自行改成运行时全局同步。
- Full-size采用离窗口上中点最近的有效地形采样，严格比较保留首次相等项；锚点使用1-exp(-delta*5)，原上限框及标签位置不改成常规UI布局。
- Max选择原形状最右侧屏幕点，不以窗口裁剪重选；顶端是绝对世界y60，不是地形高度+60。Esc打开时不推进事件读游标。
- 普通抬升SphereCloud沿发布的prepared Curve3逐点Y-0.2、半径0.5，不能替换为未经路径处理的raw轮廓。箭头沿实际交点在缓存曲线的投影u与两次倒长度乘法计算；默认UI拾取、声音和材质不额外创造。
- Started先归档墙与Terrain，再允许真实装饰物ID裁剪，捕获交点/初始高度/原最低地面。Held使用原射线范围[-f32MAX,f32MAX]；Finished保留相等高度历史，不擅自跳过。
- World Reload复用同一Bevy App：原Local<Option<Vec3>>映射到可清理的TextAnchors Resource，这是明确宿主适配。clear_toolbar_session同时清锚点和旧Max事件，避免跨世界残留。

## 指令差分与验收状态

`review_toolbar_demo_raise_native.py` 固定RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，执行目标高度10000例、音频选择100例、箭头法向1000例。所有纯数值片段无业务桥接。

`review_toolbar_demo_raise_rust.py` 直接抽取当前生产三个函数，以rustc独立编译验证11100例；所有非NaN逐bit一致，NaN仅分类。结果见 toolbar-raise-arithmetic-rust.json。保留失败证据：最初Cargo测试曾因RON固定数组编码错误失败；已修编码后由独立rustc复验，未把旧失败写作通过。

Demo投影原函数夹具实际为36例（3点×3窗口×4种w）；测试计数已按真实夹具修正，未夸大为108。其RON编码已修复，Cargo测试已经逐bit通过；最新已删除Demo重复数学实现，改用core权威原投影，5/5复验通过。

已添加CPU真实业务联合测试：真实公开墙及路径准备→实际UiState SphereCloud→实际两工厂wake→原HandleRaise→Started归档→Held投影至60→Finished历史/sheep/audio/Max。另有真实圆/矩形producer到Max消费者、实际字体/模板与菜单门禁、Reload资源清理测试。上述Demo实际消费者/门禁测试含原字体模板共5/5通过；抬升包含普通编辑与FocusHeight真实producer到impl共2/2通过。之后创建工具进一步恢复与卸载验收见下文。真实GPU画面、OS鼠标键盘仍未验收。

## 可复用经验

1. 先找消费者事件的PDB参数和元素stride，辨明28B ShapeParameters，而不是按类型名猜单位事件。
2. 追到实际生产入口再判定“已接线”。legacy CPU Schedule有系统不能证明正式run_game有原add_wall_shape_ux。
3. Ghidra impl_raise显示1002B，PDB/原汇编实际4211B；误判会漏掉拖拽、历史和最低地面循环。遇CFG异常，以完整有界原字节和ret/funclet核对，不修改共享数据库来迎合反编译。
4. `.max`、逐分量除法、常规NDC、传播第一级w都可能看似合理但不等价。固定原数据与指令oracle，普通值及signed-zero/NaN/退化边界都记录。
5. 原工厂必须保留mesh/shader/Commands、实际实体与卸载所有权。把UiAppendSignifier raw记录塞进队列只能覆盖外形信息，不能证明画面生产。
6. 多agent按原文件owner分工，中央单owner负责lib/host注册。共享Cargo磁盘不足时暂停新构建，纯函数原字节差分仍可独立推进。


## 创建工具前置的进一步收口

入口审查证实原两个 UI producer 仍是代理壳，故本轮继续在原 `circle/ui_create_circle.rs`、`rectangle/ui_create_rectangle.rs` 内完整恢复：

- 圆形原12组参数、矩形原16组参数，真正读取 `AppMode`、`ActionSetStates.analog[0]`、`UiState` 的原 `wall_circle_canvas` / `wall_rectangle_canvas`，而不是 SelectedWallShape/CursorTerrainRaycast/CanvasInput 代理。
- 共享原 ShapesDragMouseMoved 保存屏幕起点、累计位移及 >4 的移动判据。Started重置并冻结放置，Held继续冻结与调整尺寸；Finish将OS光标请求还原到原起点。CPU测试只观察事件，不向OS发输入。
- 半径围绕原中心以原相机光标射线投到放置平面，并夹[0.90000004,8]；矩形按原axis投影两分量、双倍尺寸夹[2.25,15]，中心保持不动。保留拖动调整尺寸与随后点击放置的不同分支。
- Max读取ShapePlacement.elevation（+3c），创建命令分别传 `computed_height`（+40，calc写入）与 `elevation`，圆另保留rotation及previous_wall。此前把这两个原字段放在Preview的推断已纠正，不能把placement自身清+40当作整个调用链恒0。
- 原边界的反馈拒绝和持续显示分别发事件。真实ConstructTimberframeCmd使用圆半径+0.28 / 矩形尺寸+0.56，保留roof gate，树请求带elevation、扩张4和finished=false。音频保留创建、开始、移动、宽/长变化及原面积归一化分支，AudioCursorMoved读取边界分别保留。
- 工厂改为真正拥有Generated闭包和原网格采样、shader两分支（dashed_line/vertex_color_shader）、Alt(3)、完整DashedLineLength、MeshFollowTerrain和零动画时长，删除原来指向栈env的悬垂raw记录。

新原指令夹具：圆完整内联投影/半径240例、矩形完整内联投影/尺寸1440例，见 `toolbar-create-geometry-native.json`；原字节零业务桥接。Cargo原数值测试逐bit通过（NaN分类），两真实画布状态机、点击创建与边界拒绝、事件payload、实际工厂测试已8/8通过。加强版真实 signifier_create→活动参数拥有→mark→反向动画→despawn→动态Mesh回收也已包含在最终8/8通过结果中；未手动despawn掩盖卸载问题。

原创建消费者由独立owner在原create_circle/create_rectangle文件恢复，公共命令/Preview及完整host注册由中央单owner整合，避免冲突。FocusHeight/Calc分别由同一原owner独立恢复；本文件的抬升CPU测试已追加Focus sphere producer→impl→真实高度/历史/Max/sheep链，2/2通过。

经验补充：旧注释把140c99520称作rectangle_dims_from_cursor_pos，实际原体仅11B的Rectangle.area。真正尺寸逻辑完全内联在14155d99f..14155da45；调用名与历史章节均不能替代原字节核验。同样圆半径NaN分支应遵循MAXSS/MINSS，不照错误反编译有序比较写兜底。


## 最终可复核结果（本任务分支）

| 命令/产物 | 实际结果 |
|---|---|
| cargo test -p country-core --lib ui_create_ --offline | 8/8，包含240+1440原几何夹具、真实画布状态机与工厂卸载；详见toolbar-create-producers-final-validation.log |
| country_core-d26da56c989ea968 systems::ui_raise_shape::tests | 2/2，含11100原数值片段、普通与Focus两条producer→impl历史/高度/提示；详见toolbar-raise-final-validation.log |
| TG_INSTALL_ROOT=<B> toolbar-915c23623f7db7e1 demo_stuff --include-ignored | 5/5，包含36原投影夹具、真实字体/模板、菜单事件消费与真实ShapePlacement来源；详见toolbar-demo-final-validation.log |
| country_core-d26da56c989ea968 resources::glade_border::tests | 1/1，两原启动门禁分支；详见toolbar-demo-border-final-validation.log |
| review_toolbar_demo_raise_rust.py | 直接生产函数11100例，非NaN逐bit、NaN分类；不冒称完整原函数差分 |

原OS输入暂停，原Steam平台SDK用户排除；正式完整A/C注册及其余旧shape系统由中央继续整合。这里完成并验证的是本报告列出的真实分支，不能据此宣布整游戏或全部toolbar已完成。特别其他旧raw signifier producers在正式注册前仍需各owner按原证修复，避免真实signifier_create对未恢复raw记录显式失败。


## 中央生产接线的独立复核（2026-10-24）

本节只读追踪正式 App 构建、真实资源 owner、事件消费者、状态与持久化；不是重复依赖本报告作者自己写的 producer 测试。复核快照指纹见 `toolbar-central-wiring-independent-review-fingerprints.json`；其他 owner 仍在并行更新源码，因此行号只绑定该快照。未新增 Cargo 构建、OS 输入或 GPU 运行，不能称为真实窗口验收。

### 确认的生产接线

- `run_game.rs:301` 首先调用 App 级 `startup_event_runtime::install`，随后才执行插件/资源初始化，相关事件已有 Bevy 每帧维护。`run_game.rs:1110,1175` 正式安装形状前置，确实执行两个5项链：Placement→放置signifier→Calc→圆UI→圆创建；普通Raise→Focus→Raise impl→矩形UI→矩形创建。10项的相对次序与独立原注册合同一致，原剩余编辑叶子仍属后续建筑模块，不能宣称完整26项。`run_game.rs:1219` 的门禁严格为 `GameTime.ran && State<Game>`，与原140a90e00一致。
- Focus的高度事件有真实消费，不是注册一个空事件：`app_systems.rs:67,69,72` 在set19执行高度mode与impl，set34（`app_systems.rs:120`）记录高度历史。Init/Finish/Adjust事件分别由原启动事件入口和墙事件入口注册。Raise的独立impl也在set17实际执行。
- 创建后的木框架、树隐藏及颜色都有实际消费者。`add_original_plugins` 安装 TimberframeSystems/TreePlugin；木框架set27先Rectangle后Circle，Rectangle在读取事件前无条件建立共享allocation，排除了首个圆形工具访问None的疑虑。TreePlugin的set27包含hide_trees与process_tree_cmds，ColorSystems由 `run_game.rs:464` 安装，颜色set31链实际消费Wall/Gate/Halftimber事件。声音的原事件由startup_event_runtime维护，Native音频宿主有真实building/wall/timberframe消费者；不启用FMOD时属于禁用后端边界，不把事件有注册写成可听见。
- `run_game.rs:352` 安装Disabled音频，`gameplay_startup.rs:32,55,56` Startup运行原Trigger/Sheep构造并由Commands发布状态；TimePlugin和AudioTriggerLibrary的真正owner已在App中。Startup结束的deferred会在第一次Update前生效。`run_game.rs:596` 调用真实coaster::build；其set9且after(main_camera_update)与原build合同一致，未自行加Game gate或改为after整个相机组。
- `run_game.rs:697` 将真实设置总路由加入独立GUI schedule，在Escape菜单之后、advance_menu_state之前执行；各页读写同一GameConfig/GameProfile，键位变更发SaveGameProfileCmd，set43真实save_game_profile写入。视频Apply经PostUpdate gather聚合，由真实GameLoop宿主应用窗口/vsync，成功后才替换LastApplied完整快照并发写盘事件，未把点击按钮等同于已应用。

### 发现、反馈与修复复核

发现过一项确定P1：视频页原 `graphics.devices.as_deref().expect` 的AvailableGraphicsDevices在整个生产代码无任何构造/插入，进入Video必失败。已立即反馈中央。中央随后在 `game_loop.rs:706` 以真实窗口surface的设备枚举构造快照、记录实际选中设备成功状态并插入原gameplay App；该App随后整体移入准备线程，因此加载→游戏不需要假复制该资源，Reload也保留原同一App。源码层面已闭合原来的必缺资源路径；GPU运行和打开视频页尚未由本review执行。

修复还存在需要中央确认的候选一致性边界：`production_renderer::settings_graphics_devices` 采用GraphicsInstance.available_for_surface的过滤结果，而实际Device::new采用独立create_candidate策略；成功创建的UUID未从类型合同保证一定保留于过滤后的列表。`record_creation_success` 对缺UUID有显式expect。已反馈中央核对同一候选来源/诊断边界，不能以空Default或篡造成功行来掩盖。此处是读侧可证明的不一致可能性，不冒称本机已经触发。

### Reload及本轮未验证边界

coaster的Local ride跨同App Reload保留。OnExit的despawn_glade_entities删除非KeepOnLevelUnload实体，包含临时velocity实体；OnEnter的startup_session先切LoadGame，后续原coaster会进入finished清理。初查怀疑重复despawn会panic，经独立photo owner检查Bevy0.16源码，EntityCommands::despawn是queue_handled(...,warn)，缺实体只有warning，且原coaster退出确实也是despawn；该疑虑不构成已确认panic，不能擅改try_despawn或加gate。真实同App删除实体后Local退出、Trigger音频复位和附件恢复由photo owner补CPU验证，结果以其最终报告为准。

设置页实际ActiveSession来自OnEnter真实startup_session，并在加载完成前保持AppMode::LoadGame；FrameRenderConfig/AdaptiveRenderState/AvailableMonitors由加载→游戏六owner移动保持身份。LastApplied只在真实设置页首次需要时populate、Apply成功后替换，Reload未重建App，因此不能误称每次重进世界都重新加载设置或重置设备快照。

本节结论仅覆盖上述中央接线。完整B26项编辑UX、复杂建筑业务、真实鼠标键盘与菜单抢占、本机GPU画面/窗口与长稳验收均不因本节而变为已完成。复核方法经验：优先追SystemParam必需资源到真实App构造/交接点；再追EventWriter到实际调度consumer；最后核State/Local在同App Reload中的寿命。只搜到函数、只看到事件注册、或只有局部World测试都不足以证明生产接线。
