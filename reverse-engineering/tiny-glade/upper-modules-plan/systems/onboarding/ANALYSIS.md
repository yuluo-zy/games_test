# systems/onboarding 模块分析（B 版）

> 由批次 2 分析 agent 产出，父级代为落盘。

> 模块：`crates/systems/onboarding`（crate ns `system_onboarding`）。二进制 B patched EXE `tiny-glade.exe`（sha256 cf048e27…6323），基址 0x140000000（VA=RVA+0x140000000）。
> 方法：ghidra shared 批量反编译（get_functions fields=signature,decompiled_code）+ read_memory 读常量/字符串 + PDB 骨架（skeleton/B/.../src/*.rs）系统签名/源码行。
> 证据：O=反编译/原指令直接观察；I=PDB 符号/签名/源码行推断；U=unknown。仅模块内部实现，不展开接线。

## 0. 公共数据模型

### 0.1 BindingHintDisplay 枚举（hint_orchestrator.rs）O/I
每个变体 0x30=48B，判别式 i32 在 +0（由 cleanup_impl/resolve_steam_impl/filter_to_mode_impl/render_hints 的 switch 读出，判别值=tag-0xc4）：
- 0xc4 Empty；0xc5 Graphic(HintGraphicParams)；0xc6 Key(Option<KeyCode>)（数据 8B@+0x34）；0xc7(199) Mouse(MouseButton)（2×u16@+0x34）；0xc8 组合（4 键，U）；0xc9(201) Steam(...)；0xca(202) Alternate(Vec<BindingHintDisplay>)；0xcb(203) Adjacent(Vec<...>)；0xcc(204) Together(Vec<...>)；0xcd(205) 哨兵（unwrap_failed）。
- 组合方法 alternate/adjacent/together<T>：self 已是同类列表则 push，否则新建 2 元素列表。元素类型可为 Option<KeyCode>(→0xc6)、MouseButton(→0xc7)、HintGraphicParams(→0xc5)、BindingHintDisplay（嵌套）、[Option<KeyCode>;4]。
- 其它方法：steam、is_some、is_row、group、filter_to_mode、resolve_steam、cleanup。

### 0.2 GameProfile 提示状态（共享资源，O，来自 tutorial_completion/reset_hints/hints_startup/exponential_back_off 反编译）
- 0x1bc i16 rotate 相机字形模式；0x1c0 pan；0x1c4 focus（1/2）
- 0x1e0 f32 ExpBackOff 计时 A；0x1e4 f32 冷却 A（缺省 720.0=0x44340000）；0x1e8 计时 B；0x1ec 冷却 B（720.0）；0x1f0 f32 累加器
- 0x1f4..0x205 共 18 个 bool「提示已完成」，索引=提示 id；0x206 bool（reset 清零，语义 U）
提示 id→偏移（tutorial_completion switch，O）：0→0x1f4, 1→0x1f5, 2→0x1fe, 3→0x1fc, 4→0x1fa, 5→0x1fb, 6→0x1f9, 7→0x1ff, 8→0x1fd, 9→0x200, 10→0x1f6, 11→0x1f7, 12→0x1f8, 13→0x201, 14→0x202, 15→0x203, 16→0x204, 17→0x205。
归属（I）：0 DeselectTool,1 FocusedEdit,2 Undo,3 DisableSnapping,4 TerrainSteepness,5 TerrainFocus,6 Terrain(layering),7 RoofShape,8 UnlinkWindows,9 ClutterFocus,10 CameraPan,11 CameraRotate,12 CameraZoom,13-17 Stairs 五阶段。〔注：批次2复核确认 id 11=CameraRotate、12=CameraZoom，与其自身 §7.2/§8.1（0xa pan/0xb rotate/0xc zoom）一致；原写反已改正〕

### 0.3 事件/请求（O/I）
- DisplayHintRequest（0x58B）：+0x00 i32 低字节=提示 id；+0x18 i32 与 0xc4 比较；+0x48 u8=带进度条；+0x49 u8=提示 id；另有 flags（0x100/0x200/0x300/0x400/0x600/0x800/0xd00/0xe00/0x1000/0x1100/0x1200，低字节多为 placement）。细节 U。
- 完成事件 TutorialHintComplete/提示完成（0x10B）：+0x08 u8=id（tutorial_completion、hint_camera_progress_update、各 condition 读）。O
- LingerHintRequest：与 DisplayHintRequest 同族。O
- add_onboarding 注册事件：DisplayHintRequest、LingerHintRequest、TutorialOp、TutorialHintComplete、NotifyHintSystem{CameraMoved,Snapping,Terrain,TerrainSteepness,RoofShape,ClutterWasMoved,ShapeEditWasPerformed,Window,ToolWasJustUsed,SomethingWasErased,StairsPlaced,HoveringOverStairs,StartedStairsContinue,CanContinueStairs,StairsDetachedFromWall,StairsCanDetachFromWall}、DisplayRollercoasterPrompt。O

### 0.4 冷却表 DAT_143038ae8（按提示 id 索引 f32，O，读 80B）
[0]=0.0 [1]=0.3 [2]=0.0 [3]=0.0 [4]=0.3 [5]=0.3 [6]=0.3 [7]=0.3 [8]=0.0 [9]=0.0 [10]=0.3 [11]=0.3 [12]=0.3 [13]=0.0 [14..]=0.0 → 提示 1/4/5/6/7/10/11/12 删除冷却 0.3s，其余 0.0s，写入 HintDeleteAfterCooldown。

### 0.5 语言键（lang-lib，O，read_memory）
hint/mouse(10)、hint/lmb(8)、hint/rmb(8)、hint/rmb-move(13)、hint/mmb(8)、hint/mmb-move(13)、hint/scroll(11)、hint/lmb-move(13)；UI 模板/键：hint_beta_warning、other/hinttext、hint、icons、row、keys、other/hint_one_key、other/hint_wasd（forward/left/back/right）。

### 0.6 条件资源（hints_startup 注册，O/I）
DeselectToolHintConditions、FocusedEditHintConditions、HintTerrainConditions、HintTerrainSteepnessConditions、HintDisableSnappingConditions、UndoHintConditions、HintRoofShapeConditions、FocusedClutterHintConditions、CameraHintsState、HintFocusTerrainConditions、CameraHintProgress、StairsHintConditions。多为「已满足」bool+计时/计数。

## 1. hint_orchestrator.rs
### 1.1 hint_orchestrator（0x14227fc30, 3035）O
输入 Res<AppMode>、Res<GameProfile>、EventReader<DisplayHintRequest>、EventReader<LingerHintRequest>、Query<(Entity,HintType,Option<BindingHintDisplay>,…)>、Commands。逻辑：EventIterator 收集本帧请求到 Vec，按 closure$0（行152）排序；读当前提示实体（稀疏集游标）；对排序后首请求：同 id → insert 新 HintPlacement + remove HintMarkedForDeletion/HintDeleteAfterCooldown（warn）；类型变 → insert HintMarkedForDeletion；否则按 DAT_143038ae8[id] 取冷却 → insert HintDeleteAfterCooldown；无同 id 且未完成 → spawn(HintLayoutMemo,HintSpring,HintAnimation,HintText,HintType,HintPlacement)，*(int*)req==10 再插 InteractionPrompt，*(char*)(req+0x48)!=0 再插 HintProgressBar；文本 lang_lib::lookup()。算法：事件收集→稳定排序（insertion_sort_shift_left ≤0x15，否则 driftsort_main）→实体增删迁移状态机。常量：冷却表、格式化模板 DAT_1430388f0/0x1430388b8/0x143038918（含 "Rollercoaster.Ride"/"Hint"）、判别 0xc4。未知：closure$0 排序键 U；DisplayHintRequest 完整布局 U；id==0x12/id==2 文案 U；HintPlacement/HintType 值集 U。

### 1.2 alternate<…>（5 单态化，O）
<Option<KeyCode>> 0x141c4e4f0；<MouseButton> 0x141c4e6d0；<HintGraphicParams> 0x141c4e8b0；<BindingHintDisplay> 0x141c4ecb0；<[Option<KeyCode>;4]> 0x141c4ead0。逻辑：self 判别==0xca 则 push item（容量不足 grow_one），否则 alloc 2 元素 [self,item]，返回 0xca。元素按类型写叶子 tag 0xc5/0xc6/0xc7 或嵌套 48B。常量：tag 0xca、步长 0x30。

### 1.3 adjacent<…>（0x141c4ee80 / 0x141c4f060，O）：同 1.2，列表 tag=0xcb。
### 1.4 together<…>（0x141c4f230，O）：同 1.2，列表 tag=0xcc。

### 1.5 filter_to_mode_impl（0x1422808d0, 125）O
fn filter_to_mode_impl(&mut self, is_gamepad: bool)。uVar1=tag-0xc4（截断 4）。is_gamepad 分支：0xc5→清空子向量后置 0xc4；0xc9 保留；0xca/0xcb/0xcc→对每个 0x30 元素递归；default→0xc4。非手柄：uVar1<5 返回；6/7/8（0xca/0xcb/0xcc）递归过滤；否则 0xc4。算法：按输入设备模式递归裁剪显示树。未知：is_gamepad 来源与两分支完整规则 U。

### 1.6 resolve_steam_impl（0x14227e5e0, 87，符号归 hint_display_system.rs）O
fn resolve_steam_impl(self, &SteamInput)->BindingHintDisplay。0xc9→SteamInput::get_action_origin_glyphs() 得 PathBuf 列表，in_place_collect 映射为 Vec<BindingHintDisplay>：0→0xc4；1→取该元素（0xcd 哨兵则 unwrap_failed）；≥2→0xca 列表。0xca/0xcb/0xcc 逐个递归。其它 memcpy 0x30。算法：递归把 Steam 抽象字形替换为具体绑定，空集退化 Empty。未知：get_action_origin_glyphs 映射（country-core/steam 侧）U；0xc8 处理路径 U。

### 1.7 cleanup_impl（0x14227f4a0, 73，符号归 hint_display_system.rs）O
fn cleanup_impl(&mut self)：0xca/0xcb/0xcc→递归清理每元素，把非 Empty(0xc4) 原地前移压缩；剩 1 个则提升为单元素；全空置 0xc4。0xc5→drop_in_place<HintGraphicParams> 后置 0xc4；0xc9→置 0xc4。末尾按变体释放堆；in_RDX==0 分支把 self 深拷贝到新分配（Cow 语义 I）。算法：递归树规范化。未知：末尾深拷贝分支触发条件/用途 U。

### 1.8 is_row（inline，无独立发射）I：PDB 仅 inlinee（行843），疑「是否列表变体」。U。

## 2. hint_display_system.rs
### 2.1 hint_animation_system（0x141c4f400, 870）O
Query<(HintLayoutMemo?,HintAnimation,…)>。对每个动画项（0xcB：{f32 value,f32 eased,f32 time}）：progress += dt/0.3*dir（dir=±1，删除中 -1），clamp[0,1]；utils::easing_functions::EasingFunction::apply 写 +4；+8 += dt；写 change-tick。算法：归一化进度(0.3s)→缓动→正/反向。常量：0.3s、步长 0xc。未知：EasingFunction 曲线（依赖库）、HintAnimation 是否多属性 U。

### 2.2 hint_deletion_cooldown_system（0x141c4f790, 866）O
Query<(Entity,HintDeleteAfterCooldown)>、Res<Time>、Commands。timer -= dt；<=0 → remove<HintDeleteAfterCooldown> + insert<HintMarkedForDeletion>（warn）。算法：倒计时→标记删除状态机。未知：HintDeleteAfterCooldown 布局 I。

### 2.3 hint_audio（0x141c4fb00, 909）O
Query<(HintAnimation,Option<HintMarkedForDeletion>)>、EventWriter<PlayUiSound>。动画值==1.0 → PlayUiSound tag 0x15(21)；==0.0 → tag 0x14(20)。算法：动画边界触发音效。未知：PlayUiSound 枚举（音频跨切面）U。

### 2.4 hint_deletion_system（0x141c4fea0, 621）O
Query<(Entity,HintMarkedForDeletion,HintAnimation)>、Commands。动画项 +4==0.0 → despawn()。算法：标记+动画归零后销毁。未知：entity 解析 U。

### 2.5 hint_highlight_shape（0x141c50120, 988）O
Query<(HintPlacement,HintAnimation)>、EventWriter<HighlightRoofCmd>、EventWriter<HighlightWallCmd>。动画值>0.9 且 HintType∈{1,2,6,7,8,9}(I) 且 +0x11!=0 → PublicWalls::roof_entity() 取屋顶实体，向两 writer 各写 tag 2 高亮命令。常量：0.9。未知：roof_entity 返回与两 writer 区分 I/U。

### 2.6 hint_position_update（0x141c50510, 1538）O
Query<(HintLayoutMemo,HintSpring,HintPlacement)>、Res<CursorPositionSS>、Res<WindowSize>、Res<camera/window>。边距 width/1000、20×、30×；按 HintPlacement：6=光标+边距；7=实体世界点经 CameraMatrices::eye_right+world_space_to_screen_space；8=Rectangle2d::as_points3 四角→屏幕→min/max；10=屏幕中心 window*0.5；其它跳转表未恢复。maxps/minps 夹取，time_continuous_lerp 平滑，写 HintSpring{pos,target,initialized}。常量：基准 1000、边距 20/30。未知：HintPlacement 完整枚举、跳转表 U。

### 2.7 hint_draw（0x141c51a60, 3202）O
ResMut<MeiContext>、Query<(HintLayoutMemo,HintAnimation,Option<BindingHintDisplay>,HintSpring,Option<HintText>,Option<HintProgressBar>,Option<InteractionPrompt>)>、Res<ActiveInputModeTracker>、Option<Res<SteamInput>>、Option<Res<SteamClientResource>>、Res<WindowSize>、EventReader<DisplayBetaWIPWarning>、Res<CursorPositionSS>。逻辑：mei IMGUI 绘制，每 hint（动画>0）new_at_pos+模板+尺寸，with_slot<closure_env_0..3> 分槽渲染绑定/文本/进度条/交互。常量：模板 hint_beta_warning/other/hinttext/hint/icons/row。未知：mei DSL（toolbar crate）、DisplayBetaWIPWarning 用途 U。

### 2.8 hint_draw::closure$2（0x14228c260, 119）O
克隆 BindingHintDisplay（按变体深拷贝堆）→ filter_to_mode_impl → Steam 资源存在则 resolve_steam_impl → cleanup_impl → render_hints。算法：克隆→模式过滤→Steam 解析→规范化→渲染。

### 2.9 render_hints（0x141c52700, 106）O
按判别 switch：0xc4 无操作；0xc5 me_image 帧索引 floorf(t/frames)（t@RDX+8，frames=in_RCX[8]）；0xc6 Key→game_key_bindings::keycode_to_str()+mei 模板（0xc3=None 哨兵→占位）；0xc7 Mouse→鼠标键标签；0xc9 Steam→panic（"Call resolve_steam() and cleanup()…"）；default(0xc8 等)→4 键组合(in_RCX[1]/[3]/[6]/[9] 各 keycode_to_str)；0xca/0xcb/0xcc→列表 with_slot<closure_env_2/3/4>。常量：other/hint_one_key、other/hint_wasd、keys、键名占位 DAT_142f147b0。未知：0xc8 语义 U；closure_env_0/1 具体 U。

### 2.10 render_hints::closure$1（0x14228c870, 303）O
渲染鼠标键标签：mouse_button_to_string() → mei::widgets::label + 颜色/字体（tg_text::TextFragment::set_metrics）。

### 2.11 render_hints::closure$2 / closure$4（0x14228ca20 / 0x14228cc00, 各475）O
渲染列表：每元素建全局 id 子节点递归 render_hints，label_with_id 分隔。两者结构相同。

### 2.12 progress_bar_update_system（0x141c530f0, 1065）O
Query<(HintType,HintProgressBar)>、Res<CameraHintProgress>、EventWriter<PlayUiSound>。按 id 0xa/0xb/0xc 取进度，量化档位：v==0→0；v<0 或 v>0.4→3；0.4≤v≤0.99→1；否则→2（阈值 0.4/0.99）。档位变化写 PlayUiSound tag 0x16(22)。未知：档位→外观映射 U。

## 3. exponential_back_off.rs
### 3.1 exponential_back_off（0x142284800, 1294）O
ResMut<GameProfile>、Local<TimeSinceMouseMoved>、EventReader<CursorMovedScreenSpace>、Res<GameTime>、ResMut<…>。维护两对 (计时器,冷却)（GameProfile 0x1e0/0x1e4、0x1e8/0x1ec）与标志 0x1f4/0x1f5，累加器 0x1f0：dt 累加 Local；读事件（3 reader）有事件→计时器归零、0x1f0+=dt；newCooldown=clamp(old*1.5,720,10800)；对每标志：timer+=dt（仅 dt<10）、clamp[0,cooldown]，timer>=cooldown→写 WriteSettingsFileCmd 并清标志。算法：指数退避 ×1.5 封顶 10800s。常量：缺省 720.0(0x44340000)、上限 10800、因子 1.5、dt<10 门控；ExpBackOff 类型（country-core game_profile.rs，I）。未知：0x1f4/0x1f5 类别名 U；0x1f0 用途 U；三 reader 对应输入 U；inline helper has_cooldown_elapsed(133)/advance_exp_backoff(137) 语义 I。

## 4. hints_startup.rs
### 4.1 hints_startup（0x14227b450, 1433）O
Res<GameProfile>、Commands。Commands::insert_resource 逐个插入 §0.6 的 12 个条件资源，初值来自标志：DeselectTool←0x1f4；FocusedEdit←0x1f5；HintTerrain←0x1f9；TerrainSteepness←0x1fa；DisableSnapping←0x1fc；Undo←0；RoofShape←0x1ff；ClutterFocus←0x200；TerrainFocus←0x1fb；CameraHintsState：state=(0x1f6&&0x1f7&&0x1f8?2:0)其余清零；CameraHintProgress：进度初值=各相机完成标志(0x1f6/0x1f7/0x1f8→1.0/0.0)，阈值初值=(0.7,0.7,60.0)；Stairs 清零。常量：0.7/0.7/60.0。未知：启动阈值(0.7) 与 reset(1.4) 2× 不一致原因 U；条件资源字段布局 U。

## 5. reset_hints.rs
### 5.1 reset_hints（0x141c1c8d0, 838）O
EventReader、多个 ResMut<条件资源>、ResMut<GameProfile>、EventWriter<WriteSettingsFileCmd>。读事件(+8)：==0 → GameProfile 0x1f4..0x206 整段置 1(0x0101010101010101)，状态=2；否则 → 12 条件资源复位（CameraHintProgress 阈值写 0.7/0.7/60=0x3f333333…、0x42700000，与 hints_startup 相同；批次2复核确认原「0.33(0x3eaaaaab)」为字节误读），GameProfile 0x1f4..0x206 清零、0x1e0/0x1e8 清零，状态=0；最后写 WriteSettingsFileCmd。常量：0x0101010101010101；0.33/0.33/60。未知：触发事件来源、0x206 用途、状态 2/0 语义 U。

## 6. tutorial_completion.rs
### 6.1 tutorial_completion（0x14227c230, 296）O
EventReader<TutorialHintComplete>、EventWriter<SaveGameProfileCmd>、ResMut<GameProfile>。读完成事件（0x10B，id@+8，0..0x11）：按 id 置 GameProfile 对应偏移（§0.2）为 1；写 SaveGameProfileCmd。算法：事件 id→完成标志（18 路 switch）→落盘。

## 7. hint_camera_controls.rs
签名（I）：enable：ResMut<CameraHintsState>、EventReader<CursorMovedScreenSpace>、EventReader<NotifyHintSystemCameraMoved>、Res<GameTime>；display：Res<AppMode>、Res<GameProfile>、EventWriter<DisplayHintRequest>、ResMut<CameraHintsState>、Query<…>。
### 7.1 hint_camera_enable_condition（0x14227c780, 396）O
更新 CameraHintsState（[0]=state int，[1..4] f32 计数器，阈值 100.0/10.0/0.1/1.5）。读鼠标/相机移动事件游标，有事件累加 [1..4]（按 dt），阈值判定满足则 *state=1，标 changed。未知：各 f32 字段与阈值对应 U（state@0，f32@+4/+8/+0xc/+0x10）。
### 7.2 hint_camera_display_condition（0x14227c910, 1646）O
读 CameraHintsState 与 GameProfile(+0x1f6/0x1f7/0x1f8)：三满足组合→state=2；读 AppMode(*piVar3，值 6/2/1/10) 与 StructureType::is_fence() 判定显示；构造 3 提示 [(0xa,camera_pan_hint),(0xb,camera_rotate_hint),(0xc,camera_zoom_hint)] 逐条按条件与持续时间 dt 判定，duration-=dt，完成写 DisplayHintRequest 与完成事件 tag 0xa/0xb/0xc。未知：AppMode 值语义、判定细节 U。
### 7.3 camera_pan_hint（0x14227d000, 637）O
读 profile+0x1c0：==1→alternate([Graphic("hint/mouse"),Graphic("hint/rmb"),Graphic("hint/rmb-move")])；==2→[mouse,mmb,mmb-move]；其它→alternate(MouseButton)。结果 memcpy 0x30。
### 7.4 camera_rotate_hint（0x14227d2f0, 702）O
读 profile+0x1bc：==1→together(steam,alternate([mouse,rmb,rmb-move]))；==2→mmb 变体；其它→together(steam,alternate(MouseButton))；再 adjacent(Option<KeyCode>)+alternate。
### 7.5 camera_zoom_hint（0x14227d680, 169）O
adjacent(steam,steam)→alternate（滚轮缩放，键 hint/scroll I）。

## 8. hint_camera_progress.rs
签名（I）：update：ResMut<CameraHintProgress>、EventReader<NotifyHintSystemCameraMoved>、EventWriter<TutorialHintComplete>、Res<GameTime>；reset：ResMut<CameraHintProgress>、Query<ref<HintType>>、Local<DisplayedHintDelta>。
### 8.1 hint_camera_progress_update（0x14227bd30, 782）O
字段：+0 pan进度,+4 rotate,+8 zoom,+0xc..+0x14 上一帧快照,+0x18 pan阈值,+0x1c rotate阈值,+0x20 zoom阈值。先快照；读相机事件（tag 2/1/其它）：pan += dt/阈值[+0x18]；rotate += 事件值/阈值[+0x1c]；zoom += dt/阈值[+0x20]；进度跨 1.0 → 写 TutorialHintComplete(tag 0xa/0xb/0xc)。算法：事件累积+边沿完成判定。未知：tag 1/2 与 pan/rotate/zoom 精确对应 I/U。
### 8.2 hint_camera_progress_reset（0x14227c070, 436）O
读当前提示 id（cVar5）与上一帧(*in_R8)，id 变化时归零并恢复阈值：0xc(zoom)→+8=0,阈值+0x20=120.0(0x42f00000)；0xb(rotate)→+4=0,阈值+0x1c=1.2(0x3f99999a)；0xa(pan)→+0=0,阈值+0x18=1.4(0x3fb33333)；无匹配记 19。最后 *in_R8=cVar5。常量：pan1.4/rotate1.2/zoom120（与 §4.1 0.7/0.7/60 相差 2×）。

## 9. hint_focused_edit.rs
签名（I）：condition：Res<GameProfile>、ResMut<FocusedEditHintConditions>、EventReader<ModeJustChanged>、EventWriter<PlayUiSound>、Query<…>；hint：Res<AppMode>、Res<PublicWalls>、Query<(Roof,RoofTriggerVolume)>、ResMut<…>。
### 9.1 hint_focused_edit_condition（0x14227da70, 737）O
GameProfile+0x1f5!=0 → 置状态 1 返回。否则读事件(工具变化)，工具∈{1,2}(*piVar5-1U<2) 判定；满足写完成事件 tag 1；读焦点实体工具字节==1 → 写事件 tag 0x13。未知：工具枚举值、tag 0x13 用途 U。
### 9.2 hint_focused_edit_hint（0x14227dd60, 826）O
count++；count>4 置标志。Query<(Roof,RoofTriggerVolume)>+RaycastWorld::legacy_raycast（命中且 !=2），ui_edit_focus::geo_src_to_edit_subject → 跳转表(DAT_1430386a0) 生成提示。条件 *in_RCX==0。未知：跳转表、count>4 门控 U。

## 10. hint_clutter_focus_edit.rs
签名（I）：condition：Res<AppMode>、ResMut<FocusedClutterHintConditions>、EventWriter<DisplayHintRequest>、EventReader<NotifyHintSystemClutterWasMoved>、…；hint：Res<GameProfile>、ResMut<FocusedClutterHintConditions>、EventReader<ModeJustChanged>、EventWriter<PlayUiSound>、Query<…>。
### 10.1 make_focus_or_camera_binding_hint（0x141c3de00, 536）O
读 x+0x1c4：==1→alternate([Graphic("hint/mouse"),Graphic("hint/rmb")])；==2→[mouse,mmb]；其它→alternate(MouseButton)。未知：x 类型（GameProfile vs SteamInput）U。
### 10.2 hint_clutter_focused_edit_condition（0x141c3d890, 737）O
GameProfile+0x200!=0 → 置 1 返回。读事件；工具==3 置状态并写完成事件 tag 9；实体工具字节==9 写事件 tag 0x13。未知：工具值 3/9 U。
### 10.3 hint_clutter_focused_edit_hint（0x141c3db80, 578）O
读事件；Query+RaycastWorld::raycast，ui_edit_focus::geo_src_to_edit_subject_w_deco_clutter 结果==3 → make_focus_or_camera_binding_hint 写 DisplayHintRequest(flags 0x100, id 3)。

## 11. hint_roof_shape.rs
签名（I）：hint：Res<AppMode>、Res<GameProfile>、ResMut<HintRoofShapeConditions>、EventReader<NotifyHintSystemRoofShape>、EventWriter<DisplayHintRequest>、…。
### 11.1 hint_roof_shape（0x142284ed0, 900）O
遍历事件；GameProfile+0x1ff==0 且事件字节==1 且 [+1]!=1、*in_RCX==0(无工具)、legacy_raycast 命中、geo_src_to_edit_subject==2、PublicWalls::roof_entity 有效且 Roof::is_flat()==0 → make_lbm_drag_binding_hint 写 DisplayHintRequest(tag 7)。
### 11.2 make_lbm_drag_binding_hint（0x142285270, 233）O
together(steam,alternate(Graphic("hint/lmb-move")))。

## 12. hint_stairs.rs
### 12.1 hint_stairs（0x142285a20, 3217）O
输入（I）：Res<AppMode>、Res<GameProfile>、ResMut<StairsHintConditions>、EventWriter<TutorialHintComplete>、EventWriter<DisplayHintRequest>、多 EventReader<NotifyHintSystem*Stairs*>。楼梯多阶段（标志 0x201..0x205，id 13..17）。生成 alternate(Option<KeyCode>)/alternate(Graphic("hint/rmb"))，写 DisplayHintRequest(id 6，flags 0xd00/0xe00/0x1000/0x1100/0x100)；按条件写 TutorialHintComplete(tag 0xd/0xe/0xf/0x10/0x11)。算法：多阶段楼梯教程状态机。未知：五阶段触发条件/文案、0x1f0 类计时器语义 U。

## 13. hint_terrain.rs
签名（I）：Res<AppMode>、Res<GameProfile>、ResMut<HintTerrainConditions>、EventReader<NotifyHintSystemTerrain>、EventWriter<DisplayHintRequest>、…。
### 13.1 hint_terrain_layering（0x142283cc0, 844）O
工具==10 且 GameProfile+0x1f9==0 且事件计数>1 且存在实体且事件字节[+1]==0 → alternate(Option<KeyCode>) 写 DisplayHintRequest(id 6，flags 0x600)；事件字节==1 且 [+1]==0 → 写 TutorialHintComplete(tag 6)。常量：工具10、标志0x1f9、flags0x600、tag6。

## 14. hint_terrain_focus.rs
### 14.1 hint_terrain_focus（0x142283980, 738）O
工具==10 且 GameProfile+0x1fb==0 且存在实体且事件字节==0 → make_focus_or_camera_binding_hint 写 DisplayHintRequest(id 6，flags 0x100)；事件字节==2 → 写 TutorialHintComplete(tag 5)。

## 15. hint_terrain_steepness.rs
### 15.1 hint_terrain_steepness（0x142283640, 754）O
工具==10 且 GameProfile+0x1fa==0 且事件字节==1 且 [+1]==0 且存在实体 → make_lbm_drag_binding_hint 写 DisplayHintRequest(id 6，flags 0x400)；事件字节!=0 且 [+1] → 写 TutorialHintComplete(tag 4)。

## 16. hint_undo.rs
签名（I）：Res<GameProfile>、EventReader<NotifyHintSystemSomethingWasErased>、EventWriter<TutorialHintComplete>、EventWriter<DisplayHintRequest>、…。
### 16.1 hint_undo（0x142284220, 1280）O
DigitalActionState::just_pressed() → 写事件(tag 2)。读事件/工具 cVar7：==2 → timer+=dt；timer>10.0 → 写事件(tag 2)；再判 just_pressed → 写事件。末尾：GameProfile+4==1 且 +0x1fe!=1 → adjacent(steam,adjacent(Option<KeyCode>))→alternate 写 DisplayHintRequest(id 6，含 0xc9)。常量：+0xfc/+0x104 键码、阈值 10.0、tag 2、标志 0x1fe。未知：键码来源 I、+4 标志语义 U。

## 17. hint_unlink_windows.rs
### 17.1 hint_unlink_windows（0x14227d760, 707）O
GameProfile+0x1fd!=0 → 返回。事件字节!=0 → 写 TutorialHintComplete(tag 8)；==0 → alternate(Option<KeyCode>) 写 DisplayHintRequest(id 6，flags 0x800)。

## 18. hint_deselect_tool.rs
签名（I）：Res<AppMode>、ResMut<DeselectToolHintConditions>、Res<GameProfile>、EventReader<ModeJustChanged>、EventReader<…>、…。
### 18.1 hint_deselect_tool（0x1422853c0, 1116）O
GameProfile+0x1f4!=0 → 返回。读事件与 AppMode::is_tool_selected()、DigitalActionState::pressed()：工具已选、数字动作按下、鼠标未按住 → make_focus_or_camera_binding_hint；id = (*piVar27==6)? 6+*(byte*)(piVar27+1) : 6，写 DisplayHintRequest。未知：*(byte*)(piVar27+1) 子类型 U。

## 19. hint_disable_snapping.rs
签名（I）：Res<AppMode>、Res<GameProfile>、ResMut<HintDisableSnappingConditions>、EventReader<NotifyHintSystemSnapping>、EventWriter<DisplayHintRequest>、…。
### 19.1 hint_disable_snapping（0x14227e140, 661）O
事件字节[+8]==0 且 GameProfile+0x1fc==0 且某输入状态 0 且工具==6 → alternate(Option<KeyCode>)（键来自 GameProfile+0x120/+0x128）写 DisplayHintRequest(id=bVar2+6)；事件字节!=0 且某条件 → 写 TutorialHintComplete(tag 3)。未知：工具值 6 语义 U。

## 20. lib.rs
### 20.1 add_onboarding（0x140650d50, 2693）O
impl Plugin：向 Startup/Update add_systems（chain_ignore_deferred、in_set）；EventRegistry::register_event 注册 §0.3 的 21 个事件（前有 Components::get_resource_id 幂等检查）。算法：Bevy 插件注册。未知：add_systems 完整系统清单与顺序（大部分优化成 extraout_RAX 占位）、interned SystemSet 名 U。
### 20.2 prompt_rollercoaster（0x14227bb70, 395）O
输入（I）：EventReader<DisplayRollercoasterPrompt>、EventWriter<DisplayHintRequest>、Res<GameProfile>。有事件 → alternate(Option<KeyCode>) 写 DisplayHintRequest(id 10，flags 0x1200)。常量："Rollercoaster.Ride"。

## 21. 逻辑总览 + 算法清单 + 未知清单
### 21.1 逻辑总览
事件驱动 + ECS 状态机 + IMGUI 渲染的新手提示系统：(1) 各 hint_* 输入检测→构造 DisplayHintRequest(显示树+id+flags)；(2) hint_orchestrator 收集排序+实体增删迁移+冷却；(3) hint_display_system 动画(0.3s 缓动)/位置(光标·世界点·包围盒·屏幕中心+夹取+插值)/高亮/音效/进度条/mei 绘制；(4) tutorial_completion/hints_startup/reset_hints/exponential_back_off 记忆与退避；(5) BindingHintDisplay::alternate/adjacent/together 组合 + filter_to_mode/resolve_steam/cleanup 归一化。
### 21.2 算法清单
指数退避 clamp(c*1.5,720,10800)；事件累积+边沿完成(progress+=dt/阈值 跨1.0)；time_continuous_lerp 平滑；easing_functions 缓动 clamp(v+dt/0.3*dir)；分段量化/滞回(0.4/0.99→档位0..3)；显示树递归(构造/变换/渲染同构遍历)；精灵帧 floorf(t/frames)；稳定排序(insertion/driftsort)；实体生命周期状态机；多锚点屏幕投影+min/max 夹取；id↔完成标志 18 路映射。
### 21.3 未知清单
1 HintType/HintPlacement 完整值集；2 DisplayHintRequest/LingerHintRequest/TutorialHintComplete 完整布局与 flags 位义；3 0xc8/0xcd 语义；4 CameraHintsState/CameraHintProgress 字段语义与阈值两套（启动与 reset_hints 同为 0.7,0.7,60；reset 为 1.4,1.2,120，即 2×）不一致根因；5 各 hint_*Conditions 资源字段/判定；6 AppMode(6/2/1/10)、工具值(3/6/9/10)、is_fence 语义（country-core）；7 closure$0 排序键、id==0x12/id==2 文案；8 add_onboarding 系统排程清单/顺序；9 hint_stairs 五阶段条件/文案；10 各 flags 位域义；11 依赖库/跨切面细节(EasingFunction、mei DSL、PlayUiSound、Steam glyph)；12 纯内联无符号 helper(has_cooldown_elapsed、advance_exp_backoff、is_row、group、From impl)。

## 附：函数地址速查（VA）
hint_orchestrator 0x14227fc30；alternate<[Option<KeyCode>;4]> 0x141c4ead0；alternate<Option<KeyCode>> 0x141c4e4f0；alternate<MouseButton> 0x141c4e6d0；alternate<HintGraphicParams> 0x141c4e8b0；alternate<BindingHintDisplay> 0x141c4ecb0；adjacent<Option<KeyCode>> 0x141c4ee80；adjacent<BindingHintDisplay> 0x141c4f060；together<BindingHintDisplay> 0x141c4f230；filter_to_mode_impl 0x1422808d0；resolve_steam_impl 0x14227e5e0；cleanup_impl 0x14227f4a0；hint_animation_system 0x141c4f400；hint_deletion_cooldown_system 0x141c4f790；hint_audio 0x141c4fb00；hint_deletion_system 0x141c4fea0；hint_highlight_shape 0x141c50120；hint_position_update 0x141c50510；hint_draw 0x141c51a60；hint_draw::closure$2 0x14228c260；render_hints 0x141c52700；render_hints::closure$1 0x14228c870；render_hints::closure$2 0x14228ca20；render_hints::closure$4 0x14228cc00；progress_bar_update_system 0x141c530f0；exponential_back_off 0x142284800；camera_pan_hint 0x14227d000；camera_rotate_hint 0x14227d2f0；hint_camera_enable_condition 0x14227c780；hint_camera_display_condition 0x14227c910；camera_zoom_hint 0x14227d680；hint_camera_progress_update 0x14227bd30；hint_camera_progress_reset 0x14227c070；hint_focused_edit_condition 0x14227da70；hint_focused_edit_hint 0x14227dd60；make_focus_or_camera_binding_hint 0x141c3de00；hint_clutter_focused_edit_condition 0x141c3d890；hint_clutter_focused_edit_hint 0x141c3db80；hint_roof_shape 0x142284ed0；make_lbm_drag_binding_hint 0x142285270；hint_stairs 0x142285a20；hint_terrain_layering 0x142283cc0；hint_terrain_focus 0x142283980；hint_terrain_steepness 0x142283640；hint_undo 0x142284220；hint_unlink_windows 0x14227d760；hint_deselect_tool 0x1422853c0；hint_disable_snapping 0x14227e140；hints_startup 0x14227b450；reset_hints 0x141c1c8d0；tutorial_completion 0x14227c230；add_onboarding 0x140650d50；prompt_rollercoaster 0x14227bb70。
