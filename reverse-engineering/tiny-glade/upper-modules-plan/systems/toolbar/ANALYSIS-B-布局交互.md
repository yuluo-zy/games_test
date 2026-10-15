# systems/toolbar 布局/系统/交互 子范围分析（B 版）

> 由批次 3 分析 agent 产出，父级代为落盘。

范围：taffy 布局（taffy_layout/taffy_utils/taffy_update）、工具栏布局/资源（toolbar_layout/undo_redo_layout/resouces_and_components/mod.rs）、工具栏系统与交互（toolbar_startup/toolbar_systems/toolbar_button/toolbar_update/ui_interaction）、demo_stuff。地址为 RVA（VA=RVA+0x140000000）。mei/menu/photomode/text/插件注册由并行 agent 覆盖，未重复。标注 O=充分理解 / I=部分理解 / U=未知。

## 1. resouces_and_components/taffy_layout.rs
### 1.1 TaffyLayout::style — 0x141e4c650 — O
薄包装：`taffy.style(self.node).unwrap()`。直调 taffy::node::Taffy::style，Result 判别 4=Ok 否则 unwrap_failed。无业务算法。

### 1.2 TaffyLayout::perform_layout — 0x141e4c6c0 — O（字段语义 I）
读 self.anchored_nodes（+0xa0/+0xa8，元素 AnchoredNode 0x28B）；对 anchored_nodes `sort_unstable_by`（closure=impl$1 closure_env$0，len<0x15 走 insertion_sort_shift_left 否则 ipnsort）；逐元素：tag(+0x24)==4 时 taffy_mut_style<closure_env$1>+compute_layout，否则 compute_layout+两次 Taffy::layout()；每元素调 make_layout_absolute；步进 0x28。算法：排序→compute→绝对化。未知：排序键/两次 layout 读法。

### 1.3 make_layout_absolute — 0x142374e10 — O（节点存储 I）
节点表 self+8/+0x10（元素 stride 0x330，节点 id 存 +0x328）按 id 查找；写偏移 `(self+0x90 & +0x91)!=0` 用 0x2fc 否则 0x310（两个 Vec2 候选字段）；position=old+add，mark_dirty_internal，经 child_count/child 递归下推同 add。算法：树形绝对化坐标传播。未知：0x2fc/0x310 语义、0x90/0x91 标志。

## 2. resouces_and_components/taffy_utils.rs
### 2.1 taffy_mut_style<T,F>（内联宿主，10 单态：0x142375b40..0x142376300）— O
各单态同形：`taffy.style(node).unwrap(); clone; f(&mut style); taffy.set_style(node, s).unwrap();`。closure 被内联进单态。实际样式改法要看 toolbar_alignment/undo_redo_alignment 各 case 的内联片段。

## 3. taffy_update.rs
### 3.1 taffy_update_drawables — 0x142340d20 — I
读 State(0xf8/0x100 的 u32 索引区间、+0x108 bool 切两套映射：stride 0xa0（id 在+0x94）或 0x48）与资源（+0x1a8）；经 `~u*0x30` 模式选「当前层级」数组对；每节点 Taffy::layout 后写 0x10B 元素 [center(origin+size*0.5), origin]+u32 数组（4B）。未知：uVar4 语义、两套映射区分。

## 4. resouces_and_components/toolbar_layout.rs
### 4.1 setup_screen_and_toolbar_nodes — 0x141e4ccb0 — O
new_leaf 屏幕根；anchored_nodes 追加 {0,0,Vec2,tag=4}（Vec +0x98/+0xa0/+0xa8，元素 0x28B，满 grow_one<Bucket<(DecoratorId,DecoratorId),AlignKind>>——共用泛型名）；new_leaf 工具栏节点 add_child；写 self[0]=0/screen、[1]=8/toolbar、[3/4]=Vec2。算法：节点树构建+anchored root 注册。
### 4.2 add_contextual_menu — 0x141e4d1e0 — I
stride 0x20 表中按 tag(+0x18)==in_R8B 查找，未中 Option unwrap_failed；new_leaf 建菜单根，anchored 追加 {1,0,Vec2,weight=0x41200000(10.0f),tag=0}；遍历 ToolbarItem（0x40B）tag(+0x38)!=2 递归 add_toolbar_item；drop 切片。未知：查找表类型、菜单 tag 集合。
### 4.3 link_up_nav — 0x141e4d880 — O
复制 u64 列表；逐 id：prev=ids[(i+n-1)%n]、next=ids[(i+1)%n]；Commands insert UiNav，in_DL bool 分水平（left/right 对 +0x28/0x30、+0x38/0x40）或垂直（up/down）。算法：导航环连接。3=Entity 判别、0=None。
### 4.4 add_toolbar_item — 0x141e4dc40 — I
递归构建器（ToolbarItem 0x40B：+0 kind、+0x20/+0x28 字符串、+0x38 tag、+8/+0x10/+0x18 子 Vec）。case4/5=子菜单容器（new_leaf+add_child+递归子项，tag==2 停）；其他=复制串→Commands::spawn<(TaffyNode,UiScaleWithUserSetting,UiInteractionState,UiAnimationState,UiMouseInteractable,Ui2dExtent,Ui2dImage,UiDrawOrder)>→插 UiShowIfParentIsInFocus，tag 非 0 再插 ShowOnlyInFpvAndFree→jumptable DAT_142f8f774 按 kind 分派。未知：kind 判别→按钮映射、串是图标还是文本。

## 5. resouces_and_components/undo_redo_layout.rs
### 5.1 UndoRedoLayout::setup — 0x140fad870 — O
根叶+anchored tag=4；第二叶 add_child；两个 item（stride 0x20）串 "icon/undo"(9)、"icon/redo"(9)、tag=1：new_leaf+add_child+复制串+spawn(同上 bundle)+插 HistoryButton{is_undo:tag&1}+HideIfMenuIsOpen；写 self[0]=screen、[1]=undo、[2]=redo。

## 6. resouces_and_components/mod.rs
### 6.1 LogSlider::param_to_value — 0x142383730 — O
`p=param.clamp(min,max)`（min>max 触发 do_panic::runtime）；返回 `exp2f(p+self[3])`。算法：对数映射 value=2^(clamp+log2_base)。未知：self[3] 语义、value_to_param 未反编译。
### 6.2 impl$2..impl$27::register_required_components — 0x142383770..0x14238bb40（26 个）— O（Bevy 胶水）
全部同构：enforce_no_required_components_recursion→hashbrown 表按 TypeId 查重（SIMD 控制字）→分配 ComponentId→ComponentInfo::new（type_name 静态串、TypeId 高低、size、align、标志）→写注册表+required-components 空表（DAT_143054b70）+递归自身→push 待注册。各实例 TypeId 常量不同（impl$2:0xcf934bebc18fcc2c、impl$3:0x7e66967e48a3dcf5、impl$4:0x58223c12a431f522、impl$5:0x7f910f54ee011d26、impl$6:0xbc93f02da55cdcea…）。按项目约束计为 Bevy 注册胶水，不计独立业务。

## 7. toolbar_startup.rs
### 7.1 toolbar_setup — 0x142345890 — I
Taffy::new；0xb0B 静态表 memcpy 进 ToolbarLayout（分类 (name,ToolbarCategory,Vec<ToolbarItem>)）；读 DEV_OVERRIDE_IS_DEMO_MODE 分叉 demo（count 7/串 0x23）与正式（count 1/串 0xe..）两套按钮组（串 DAT_14304e788/…，含 "toolbar/time_of_day"）；逐分类 add_category_button<closure_env_0>+add_contextual_menu（先 is_horizontal）；link_up_nav；photomode::setup_photomode_toolbar；UndoRedoLayout::setup；insert_resource ToolbarLayout/PhotomodeToolbarLayout/UndoRedoLayout。未知：各 DAT_ 串文本、add_category_button 入参。

## 8. toolbar_systems.rs
### 8.1 toolbar_auto_hide — 0x142343a10 — I（jumptable 重度破坏）
遍历节点区间（两套映射）；fVar56=scale×节点值；插值 fVar55=(count-1)*10*s+count*110*s；依 res+0x1e（ToolbarAlignment 0..3）选符号；写回 [pos,size]+u32 数组；EscMenu/Aabb2::contains/DigitalActionState 算隐藏进度（clamp 0..1，写某+0x2c，边界 push 0x11/0x10 条目）；EasingFunction::apply 后写 anchored 位置。算法：光标驱动自动隐藏+缓动。未知：各 case 偏移公式。
### 8.2 toolbar_alignment — 0x142344560 — I
anchored tag 改 `0x1020300>>((align&3)<<3)`；按 align switch 调 taffy_mut_style<env12/11/10 | env0/undo1/env2/env13 | undo0/undo1/env2/env11 | env12/13/10/13…>（样式组）；EasingFunction + 在 anchored（0x28）按 Vec2 movmskpd 定位写偏移；case0xd 读进度、EscMenu/mouse/Action（±1/0.3）、clamp、push 0x11/0x10、easing。未知：各 env 对应的节点与样式字段。
### 8.3 undo_redo_alignment — 0x1423448a0 — I
8.2 缩小版，只针对 undo/redo（closure_env$0/1 样式组）；同进度计算。
### 8.4 undo_redo_auto_hide — 0x1423449f0 — I
fVar19=res+0xe8；res+0xf1==0 直接进度=0；否则 fVar18=fVar19*110*4、fVar17=fVar19*30+fVar18；EscMenu/mouse/action 进度、clamp、easing、anchored 定位。未知：0xe8/0xf1 语义。
### 8.5 toolbar_steal_mouse_input — 0x142344df0 — I
puffin；读 State+0xf7；遍历两个 u64 列表（stride 0x20、有效位+0x10），取 taffy layout；format_inner 格式化；WidgetParams::new/no_history_recording/UiSounds::empty/sounds；UiState::add<String>。算法：把鼠标下 UI 注册进 UiState 抢占输入。未知：两列表来源、格式化串内容。

## 9. toolbar_button.rs
### 9.1 toolbar_button_keep_focus — 0x1423414b0 — I
puffin；遍历节点：flag(+1+idx*3)==0 时 fVar30=当前值-mouse.y clamp≥0 写 float 数组+uVar6 写 u32 数组；多段 Aabb 矩形运算判 hover 写 0；末段 value>0 写 uVar6 并 flag 置 1。算法：按鼠标距离分配焦点/draw order。
### 9.2 switch_app_mode_button — 0x142341fe0 — I
光标扫描命中→push tuple_(ref str,SettingsMenuState) 事件（0x18B）；比较节点工具 id uVar3 与选中 id uVar4：switch(uVar3) 依工具枚举 0..6（case0 相等、1/4 tag 字节、3 u64、6 热键类别 byte-0x38）→命中 push insert(UiPlaySelectAnimation)（0.2/0.2）。未知：枚举映射。
### 9.3 switch_toolbar_category_button — 0x1423427f0 — I
同上：push WindowFocusEvent（0x10B，entity+tag）；写选中 bool=`*(+0x18)==当前分类`；命中 push insert(UiPlaySelectAnimation)（0.2/0.1）。
### 9.4 tod_v2_button — 0x142342f30 — I
命中→`*(Node+10)` bool `^1` 翻转写回（昼夜开关）；push insert(UiPlaySelectAnimation)（0.2/0.1）；命中写某+0x10=0；开关状态写 flag+2+idx*3。

## 10. toolbar_update.rs
### 10.1 toolbar_button_size_and_padding_update — 0x141eaec50 — I
puffin；两上下文（local_188=2）；scale=res+0xe8*10**local_140；遍历节点：读 focus 组标志（+0x88 查 +0x118 层级）；未聚焦节点 EasingFunction+`*(node+0x10)=((1-ease)*0.8+ease)*当前`（尺寸→0.8）；末尾 taffy_mut_style<closure_env$1>+递归 style→clone→set_style。未知：closure_env$1 padding 数值。

## 11. ui_interaction.rs
### 11.1 render_taffy_ui_to_widgets — 0x142338990 — I
puffin；UiFocus::is_toolbar+鼠标速度>1；遍历全部节点（两映射+多层级 +0x22..0x27）：算矩形（中心±size/2）；WidgetParams::new/no_history_recording/occlude_by_world/allow_negative_distance/UiSounds::empty/sounds；UiState::add<String>（键=slotmap::impl$23::fmt+format_inner 的 entity 串）；active 且某计数==0xd 时 SignifierFactory+UiSignifierStream::write（激活框）；into_drag_state→DragState::is_some push WindowFocusEvent、==3 push insert(UiPlaySelectAnimation)；clicked push WindowFocusEvent；非拖拽 push tuple_(ref str,SettingsMenuState)。算法：taffy→widget 桥接+hover/拖拽/点击事件。
### 11.2 render_taffy_ui_to_widgets::closure$0 — 0x142339f10 — O
SignifierEntityBuilder::new/animation_duration/build；spawn "icon" 实体插 Ui2dExtent+Ui2dImage，再 insert(Ui2DFollowMouse)。跟随鼠标激活图标。
### 11.3 ui_send_focus_begin_events — 0x14233a170 — I
遍历节点：flag(+1+idx*3)!=0（进入聚焦）经 HashMap<WallId,()> 去重 push WindowFocusEvent（entity）；移除则 remove_entry。算法：焦点进入事件去重。
### 11.4 ui_sounds — 0x14233a7f0 — I（jumptable 破坏）
标准两映射扫描+命中位图；对命中节点 `bVar6=(tag-2)clamp 0..3`，push 声音事件（sound id 0xa/0xf/0x1a/0x1b/0x1d，0x20B 条目 (u32,ptr)）；尾段并入 iter_subtree/导航代码——疑反编译边界错并。未知：声音 id→音效映射、函数边界。
### 11.5 ui_animation_state_update — 0x14233b3f0 — O（字段 I）
UiAnimationState 数组 pfVar18=float[2]（+0 appear、+1 focus）+3 状态字节；flag(+1) 聚焦时 focus += dt/0.2 置状态0，失焦 += dt/-0.2 置1，否则 idle=2；clamp；appear += ±dt*4（依 appear 方向字节）clamp。算法：appear/focus 双通道动画状态机。
### 11.6 ui_focus_animation — 0x14233ba40 — I
Query(UiAnimationState,&mut TaffyNode,Option<UiSlider>,Option<UiSliderReset>,Option<UiCustomSize>)；三个 focus 组（+0x120/0x128/0x130 位图）→目标缩放全空 1.5 否则 1.2、UiCustomSize 自定义；EasingFunction 后 scale=(target*ease+(1-ease))*当前。
### 11.7 ui_appear_animation — 0x14233c020 — I
遍历节点：EasingFunction 后 appear 值 *= ease；写 draw order。
### 11.8 ui_select_animation — 0x14233c300 — I
带 UiPlaySelectAnimation 节点：p=clamp01(p+dt/duration)；`fVar14=((1-p)*0.1+p)*4*(1-p)`（弹跳）；scale=(fVar14^2*base+1)*当前；完成 push remove。算法：二次 pop 脉冲自移除。
### 11.9 ui_locked_animation — 0x14233c8b0 — I
p=clamp01(p+dt/duration)；`_X=(p*5-1)*π; amp=sin(_X)/_X`（sinc 抖动）；scale=(amp*base+1)*当前；完成 remove(UiPlayLockedAnimation)；命中交互 push insert（0.3/0.1）。
### 11.10 ui_show_if_parent_in_focus — 0x14233ce00 — I
查父（+0xc 索引）；读父 focus 位图（+0x118）bVar27；写 flag=父状态；父聚焦时 visible=(cVar8==0)||(state-1<2)。算法：父级聚焦驱动显隐。
### 11.11 ui_notify_hints_if_toolbar_in_focus — 0x14233d680 — I
找第一个聚焦节点（flag!=0 即止）；push WriteSettingsFileCmd 事件（0x8B）。工具栏聚焦时写设置文件。
### 11.12 ui_focus_parent_if_self_in_focus — 0x14233dcb0 — I
收集聚焦节点 id（HashMap 去重）→查父写 flag=1（向上传播焦点）→去重继续。算法：焦点上推。
### 11.13 sort_contextual_buttons_draw_order — 0x14233e760 — I
bVar2=flag(+idx*3)；写 u32 数组=uVar3（draw order）+u64 数组=bVar2（可见性排序键）。
### 11.14 debug_hide_ui — 0x14233e960 — I
读 res+0xf7；开关==0 时写 `*(node+8+idx*0x10)=0`（scale 置 0 隐藏）+draw order。算法：DEV 开关一键隐藏。
### 11.15 hide_ui — 0x14233eb50 — I
EscMenuState::is_closed×UiFocus::is_mei 求 bVar12；UiFocus::is_toolbar 清点击/聚焦标记（piVar22[4/5]、[0/1]）；命中隐藏组（+0x120 位图）节点 flag 置 0 或 ^1。
### 11.16 ui_nav_update — 0x14233eed0 — I（jumptable 重度破坏）
UiFocus::is_mei→清焦点返回；读 4 方向 just_pressed→local_8c/a0/9c/88；有焦点时读焦点节点、just_released 清；按方向选邻接槽（0x18/0x28/0x38），bVar29&目标 时按 tag jumptable DAT_14304ded8 分派；收集子树（taffy children 栈+slotmap 键反转+去重）；local_158=a0-8c、local_154=9c-88 方向向量；iter_subtree 搜索最近；命中写 focus（+0x10=1、+0x18=id）+EventWriter::write<UiClickEvent>。候选按 tag switch（case0..3 选方向标志、5/9/e 累积）。算法：4 方向导航状态机+几何最近搜索。
### 11.17 ui_nav_update::closure$0 — 0x142340660 — O
搭 iter_subtree 参数（0x7f7fffff 初值），命中写 focus。导航搜索包装。
### 11.18 iter_subtree — 0x141ea8c60 — O（核心几何）
hash_one<RandomState,&slotmap::DefaultKey>+开放寻址 SIMD 控制字扫描；槽位 id==目标（movmskpd）→vector=node_pos-focus_pos、dist=|v|、归一、dot(v,dir)>0&&dist<best 更新 best；taffy children 递归。算法：方向最近节点（点积+极小化）；best 初值 0x7f7fffff。

## 12. demo_stuff
### 12.1 demo_show_full_glade_size — 0x141eac540 — I
守卫 DEV_OVERRIDE_IS_DEMO_MODE==1、状态!=0x13、EscMenu 关；eye_position/forward+CameraCursorRay::intersect_with_xz_plane；距旧点≤70 清标记；否则 Rectangle2d::perimeter 画周界 Signifier（色5.0、alpha=ease*0.3）；ShapeParameters::as_circle+Circle2d::as_curve2 遍历圆周点 sample_at_world_xz+world_space_to_screen_space 求最近屏幕点（min 距离）；命中：time_continuous_lerp_vec3+MeiContext::reuse_or_create_node（键=calculate_hash^0xb1695320004fb443）+iced Event（0x38B：type4、pos、-1,-1、id1、尺寸 0x41f00000/0x43480000）+template_with_id<u64>+TemplateView::with_slot<closure_env$1/2>（半径 137.88582，y 偏移 -30/-50）。算法：demo 显示完整 glade 尺寸标记。137.88582=glade 半径(米)推断。
### 12.2 demo_show_full_glade_size::closure$0 — 0x141ead870 — O
SignifierEntityBuilder::new/animation_duration/custom_shader/build；spawn 实体插 DrawTopology+DashedLineLength。周界虚线实体。
### 12.3 demo_show_max_elevation — 0x14234a410 — I
EscMenu 开则返回；光标扫描；光标在世界内：ShapeParameters::as_curve2 取边界点，逐点 world_space_to_screen_space 求最大屏幕 Y；eye_right+TerrainHeightsData::sample_at_world_xz；单位向量 fVar26/27/23=0.5*unit；画 3 条 DebugLine（new/line_width/show，长 0.7）十字标记；MEI 标签 reuse_or_create_node+iced Event+template_with_id+with_slot<closure_env$0>。

## 13. 逻辑总览
静态定义（toolbar_setup 实例化分类→按钮→菜单→导航环→3 资源）→节点树构建（setup_screen/add_contextual_menu/add_toolbar_item/UndoRedoLayout::setup：taffy 树+AnchoredNode 表+spawn 8 组件 bundle）→布局（perform_layout 排序+compute+make_layout_absolute 绝对化；taffy_mut_style 通用改-写）→每帧桥接（taffy_update_drawables 写 [center,origin]+u32；render_taffy_ui_to_widgets 注册 UiState+hover/拖拽/点击事件+Signifier）→对齐/隐藏（toolbar_alignment/undo_redo_alignment 样式批改+重排；auto_hide 光标/Esc/动作缓动）→交互状态机（ui_animation_state_update 双通道+4 个 ui_*_animation 缓动；焦点传播/事件/排序/hide_ui/debug_hide_ui）→导航（ui_nav_update+iter_subtree 方向最近搜索+link_up_nav 环形链接）→按钮专用（keep_focus/switch_app_mode/switch_toolbar_category/tod_v2，各播 UiPlaySelectAnimation）→输入接管（toolbar_steal_mouse_input）→demo 标记。

## 14. 算法清单
taffy 树构建+绝对定位（make_layout_absolute 递归坐标传播）；稳定排序 ipnsort/insertion；通用样式改写 taffy_mut_style<T,F>；导航环形链接（(i±1)%n）；导航方向最近节点（点积+极小化+slotmap SIMD 扫描）；焦点/动画状态机（appear/focus 双通道+状态 0/1/2）；弹跳 pop `4x(1-x)` 与 sinc `sin(x)/x`；焦点上推与父聚焦显隐；自动隐藏缓动；对齐批改 tag 编码 `0x1020300>>((a&3)<<3)`；鼠标输入接管 UiState::add；对数滑块 exp2f；Bevy 组件注册胶水。

## 15. 未知清单
1) AnchoredNode 排序键（closure_env$0）；2) ToolbarItemKind 判别→按钮映射与串字段语义（jumptable DAT_142f8f774 未展开）；3) ToolbarAlignment 四值语义与各 case 偏移公式（jumptable 破坏）；4) taffy_mut_style 各 closure_env 修改的样式字段；5) 两套节点映射（0xa0/0x48、res+0x108）语义；6) make_layout_absolute 0x2fc/0x310 字段与 0x90/0x91 标志；7) 资源偏移（+0xe8/+0xf0/f1/+0xf7/+0x110..0x130/+0x14）未映射具名资源；8) ui_sounds 尾部并入导航代码疑反编译边界错并，需 radare2/定点核对函数边界；9) UiNav/UiPlaySelectAnimation/UiPlayLockedAnimation/WindowFocusEvent/WriteSettingsFileCmd 事件字段语义；10) demo 常数 137.88582 与 closure_env 模板；11) ui_show_if_parent_in_focus 可见性判定与焦点传播层级；12) LogSlider self[3] 语义与 value_to_param（未反编译）。
