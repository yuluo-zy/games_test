# ui-input 域 第二轮审查表（round2，tg_a5，2026-10-14）

逐文件对照 `reconstruction/country-core/src` 与 DIGEST O 条目。状态：
✓ = 与机器码证据一致；~ = 部分一致（列出具体错）；✗ = 核心事实错/关键结构缺失。
**本表只列问题和缺失，不重复 DIGEST 已述的每处细节。** 证据地址见
`evidence/country-core-deep/ui-input/DIGEST.md` 与 `decomp/`。

| file | 状态 | 具体错误 | 缺失项 |
|---|---|---|---|
| systems/mode_manager.rs | ✗ | ① AppMode 枚举缺 **Replay** 变体（判别 0xd，check_if_can_switch 写 *state=0xd）；② `AppMode::is_tool_selected @0x140a8a5d0` 是伪声明（该地址 = `timelapse_replay::cleanup_timelapse`，PDB 清单无此名）；③ `send_mode_just_changed_event` 事件类型错（实为 0x28B `{prev: AppMode 16B, current: AppMode 16B, id}` 双 AppMode 事件，且 PDB 名 = `mode_manager::closure$0`）；④ mode_manager 早退规则未实现（`hash("esc_menu_canvas") + hover!=0 + id 匹配 + 交点 tag!=2 + mode!=2`）；⑤ AppMode 判别序：2=Timelapse、6..9=Wall 形状、10+5k=Tool(工具栏类别 k)、0xd=Replay | check_if_can_switch 语义（见 CLOSURE.md 已闭合）、per-mode 跳表分发、Tool(工具栏类别) 载荷结构 |
| systems/mod_key.rs | ✗ | `Modifiers` 布局错：代码 4 个独立 bool；机器码 `disable_snapping` = **byte [rcx+1]**（位标志字节组）。`update_mod_key_state` 地址（0x140af9000 实为 Toolbar::new） | 位标志完整位定义、update_mod_key_state 真实地址 |
| input/mod.rs | ~ | ActiveInputModeTracker 32B 结构对；但 `on_controller_used/on_mouse_keyboard_used` 只改 mode，缺 **Instant::now()+Duration 时间戳逻辑**（wp15 O）；MouseMoved/InputCursorMoved 实为 16B `{id u64, Vec2}` 记录（回放路径产出），代码用 Gap 包装 | 时间戳/冷却字段语义、事件精确字段 |
| input/action_set.rs | ~ | DigitalActionState 访问器 ✓（pressed=Pressed\|JustPressed 等）；ActionSetAction tag0=Analog/tag1=Digital ✓；但 0xcB 记录 `(action-type u16, value)` 的 tag 编码（0x100/0x101/…）未恢复 | deconstruct/reconstruct 打包、ActionSetStates/ActionSetValues 布局、MainActionSet 字段 |
| input/input_stream.rs | ~ | 布局声明与 wp15 一致 ✓；InputFrame 0xd0B 各字段（DIGEST §2.3：@+0x30 u64、@+0x38 u32、@+0x3c u8、@+0x40 双 f32、@+0x44 f32、@+0x58..0xc0 十 qword）未写入结构 | add_frame/on_history_sync/drop_oldest_sync_point/seek 实现、trim 边界（9999/50000/9/10000 已有注释） |
| input/panic_repro_state.rs | ~ | `is_inactive` 硬编码 true；机器码 = `byte *self == 0` | 状态字节的其它取值语义 |
| resources/toolbar.rs | ✗ | ① Toolbar::new 地址 0xaf8c50 → 实 **0x140af9000**；② 类别表：缺 **id 0 类别**；`Roofs=8` 存疑（8 未出现在 Toolbar::new 的 11 类 id 中，屋顶=Decor 0x38..0x3f 区间模式）；Buildings 列表含 (0xb,0x3f)/(0x12,0) 两条（置信中，Ghidra 类型混淆风险）；③ switch_mode 签名 `(u32,u8)` → 实 `(&mut Toolbar, &{cat u32, mode u8, arg u64})`；④ contains 区间 0x38..=0x3d → 实 **0x38..=0x3f**；⑤ panic 串原文不对（"mode but no toolbar" 片段）；⑥ next_mode 地址是 serde unknown_variant thunk（名错位） | ModeEntry 真实布局 `{cat u32, mode u8, pad3, arg u64}`（代码写 (u32,u8,u64) 元组可读性差）、真实 next_mode 地址、panic 完整原文 |
| resources/photo_mode.rs | ~ | next/next_demo 判别与 panic 串 ✓；但 PhotoModeCameraMode 应 **repr(u8)**（代码默认 i32 布局） | PhotomodeState（on_enable_photomode 0x140870570：f32@0=3.0/1.0 + u32@4=-1 + 2 字节）资源、CameraMode 6..12 槽映射（见 CLOSURE.md） |
| resources/gui.rs | ✗ | ① EscMenuState = 8B `{disc u32@0, sub u32@4}`（is_open：qword∈{3,4} 或 disc==7&&sub==1；allow_time_passage：!(disc==7&&sub==1)）——代码的 {Closed,Open} 二变体枚举完全不符；② 缺 **NextEscMenuState / PrevEscMenuState / PhotomodeScreen** 三个同文件资源（PDB 类型证实）；③ DebugTriggerProxyVis::as_string 行为（串表索引生成）未写 | EscMenuState 全变体名（判别 0..7+）、DebugGui 布局 |
| resources/mouse_capture.rs | ✗ | ① 注释 "SettingsMenuState" 错——mouse_capture 推的是 **EscMenuState 值 {disc2,sub2}（未捕获）/ {disc3,sub2}（捕获）** + WindowFocusEvent `{focused u32, window u64}` 16B（代码注释与事实不符）；② CursorCapturedPosition 实为 `{u32 captured, u64 pos(2×f32)}` 12B（代码 bool+f32+f32 数值一致但语义注释不足） | 与 EscMenuState/NextEscMenuState 的联动说明、WindowFocusEvent 精确字段 |
| resources/onboarding.rs | ✗ | 仅 NotifyHintSystemSnapping；缺 **WelcomeScreenState**（is_closed = 判别==2，0x1409a9140 折叠处证实）与 **TutorialOp** 事件（tiny_glade settings 系统 EventWriter<TutorialOp>，PDB 类型证实） | WelcomeScreenState 变体集、TutorialOp 载荷 |
| resources/ui_settings.rs | ✗ | **ToolbarAlignment 变体名错**：代码 Horizontal/Vertical/Bottom/Side → 实 **Bottom/Left/Right/Top**（串表 0x142acec66）；UiSettings 字段无名（实 7 字段：icon_size_mult f32@0、custom_cursor_size f32@4、toolbar_alignment u8@8、auto_hide_toolbar u8@9、input_lag_reduction u8@10、game_logo u8@11、custom_cursor u8@12，serialize 0x140a46ad0） | 字段名/默认值语义（默认 auto_hide=true、alignment=变体1） |
| resources/editing_highlight.rs | ✓ | 无实质错误（facade） | wall_highlight 写 rgba 的具体字段（跨 wall 域） |
| systems/ui/mod.rs | ~ | GlobalId::from_name ✓；signifier_config_hash 返回 0（stub）；add_ui 未实现 | UiState/UiNavState/UiSignifierStream(0x170B) 资源 + PlayUiSound/PlayPhotomodeSound 事件 + Update 调度链 |
| systems/ui/resources/ui_state.rs | ~ | ① `is_active`/`hovered` 缺 **id 匹配**（证据：hover!=0 && id@+0x38 == ctx id）；② clicked 条件 ✓（hover>1 && click<2 && click&1）；③ `clearing_canvas_w_layer` 忽略 layer 参数（实参应传入 WidgetVolumeShape.layer）；④ UiWidgetContext 布局：hover@0x18/click@0x19/id@0x38/交点@0x28..0x58 未写全 | add_with_id（RwLock@0x60 + HashMap@0x68 + 0xb0B 拷贝 + 0xc0 Arc，已解出）、widget/active_widget_state/origin、clearing_canvas 130×130 体积 |
| systems/ui/resources/widget_params.rs | ✗ | ① 结构体字段序与真实 0xb0B 布局不符（真实：u32 组 @0..0x60、sounds 96B @0x18..0x78、volume 32B @0x78..0xa8、u16@0xa8=0x101——代码的 {sounds, volume, ...} 序不匹配）；② WidgetFlags 位值（DECORATOR=1<<3、HIDE_OS_CURSOR=1<<4）未验证（U） | 真实布局表（DIGEST §2.4）、UiSounds 槽位（3qword×4）payload |
| systems/ui/resources/slider3d.rs | ~ | Slider3dState 字段 {origin, direction} 为 I；其余 stub | SystemParam 包装的资源集（UiState/UiSignifierStream/CameraCursorRay 等）、project_cursor_onto_line、update<C,F> 泛型 |
| systems/ui/hover_intersection.rs | ~ | WidgetIntersection 字段 {id, point, distance} 为 I | par_chunk_map 体积测试 + layer/distance 排序、HoverIntersectionCache 布局 |
| systems/ui/ui_update.rs | ~ | 均为 todo；get_ui_volume 返回 default（语义错） | hover/click/drag 状态机（HoverState/DragState）、enter_hover、PlayUiSound 触发 |
| systems/ui/debug_visualize_volumes.rs | ✓ | 无实质错误（stub，头注释与 wp15 一致） | 三角面片/UV 线框绘制 |
| systems/ui/signifiers/mod.rs | ~ | ① SignifierEntityBuilder 0x90B 真实布局（shader String@0x60..0x70、position Vec2@0x7c、默认 {0,1.0f64,0x100 u16@0x40,0.2f32@0x78}）与代码字段序不匹配；② **UiAppendSignifier 实为 0x28B 紧凑记录（流元素），不是 SignifierEntityBuilder(0x90B) 的包装** | SignifierFactory 40B 布局、build 三分支（0=命名网格/1=transient/2=mesh_from_pts） |
| systems/ui/signifiers/signifier_create.rs | ✗ | **UiSignifierStream = 0x170B 无锁环形流**（read cursor@+0x160、write counter@+0x168、0x28B 槽）——代码写成 `Vec<UiAppendSignifier>` | write 自旋追加 + signifier_create 排空生成实体 |
| systems/ui/signifiers/signifier_mark_for_delete.rs | ✓ | PDB 拼写 signfier 已注明 ✓ | signifier_config_hash 联动 |
| systems/ui/signifiers/animation.rs | ~ | stub | t 推进 + 缓动 scale/alpha + get_origin 父挂 |
| systems/ui_edit_focus.rs | ✗ | 头注释"geo_src 映射 EditSubject"过于简略；核心行为缺失：mode<6 注册 **"edit_focus_canvas"** widget（volume tag=5、size f32::MAX）；mode 1/2 跟随路径（FUN_140a1c930 谓词）；0x2a8 反向 wall 数组遍历 + 两条 panic（DIGEST §2.5）；mode==10 特判；结尾 push `{tag0, id}` 0x18B 事件 | 4 张旋转跳表 = 24 变体 GeoSource 匹配、EditSubject 变体 serde 名（WallId/Floor/Roof/flat/gable/DecoratorCollision/…，DIGEST §2.5 已列） |
| systems/ui_move_shapes.rs | ~ | widget 名 "move_shape" 未记录 | MoveShapeCache 布局、gizmo delta→shape 位置 + volume 触发 |
| systems/ui_raise_shape.rs | ~ | 三个 widget 名（outline_wall_bottom_/arrow_raise_shape_/raise_wall_elevation_）未记录 | project_onto_ray 抬升 + History::add 细节 |
| systems/ui_2d_follow_mouse.rs | ~ | 头注释与证据一致 ✓；body todo | 双分支 Query 索引写槽细节 |
| systems/ui_camera_mode_gizmos.rs | ~ | widget 名 **"camera_mode_active"** + signifier **"rotation_"** 未记录；`(byte@4d0 & 6)==2` 判定注释有 | 窗口尺寸门（w/h>2）、0x14 判定、AppendOnlyVec 记录 {tag,size,pos,fov} |
| systems/ui_edit_freehand_walls.rs | ✓ | stub；头注释与 wp20 一致 | EditFreehandWallCache 布局 |
| systems/ui_focus_mode_shape_height.rs | ✓ | stub；LerpedGizmoPosition 字段为 I | 高度射线投影 |
| systems/ui_move_freehand_walls.rs | ✓ | stub | MoveWallCache 布局 |
| systems/ui_clutter/*（8 文件） | ✓ | stub 族；头注释与 wp20 一致；DeletedAutoClutter 事件已声明 | 各缓存/命令载荷 |

## 汇总

- ✗ 严重（7）：mode_manager、mod_key、toolbar、gui、mouse_capture、onboarding、ui_settings、
  widget_params、ui_edit_focus、signifier_create（按条目计 10 处核心事实错/关键结构缺失）。
- ~ 部分（12）：input/action_set、input_stream、panic_repro、photo_mode、ui/mod、ui_state、
  slider3d、hover_intersection、ui_update、signifiers/mod、signifiers/animation、ui_move/raise/
  2d_follow/camera_gizmos 等。
- ✓ 一致（6）：editing_highlight、debug_visualize_volumes、signifier_mark_for_delete、
  ui_edit_freehand_walls、ui_focus_mode_shape_height、ui_move_freehand_walls、ui_clutter 族。

最高优先修复建议（按业务价值）：① mode_manager 的 AppMode 判别/Replay/早退；② Toolbar 布局与
签名族；③ EscMenuState/NextEscMenuState/PrevEscMenuState/PhotomodeScreen 资源族；④ UiSettings
字段与 ToolbarAlignment；⑤ UiSignifierStream 环形流与 UiAppendSignifier 0x28B 记录。
