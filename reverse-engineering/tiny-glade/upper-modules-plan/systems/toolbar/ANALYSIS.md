# systems/toolbar 模块内部逻辑分析（B 版）

> 由批次 3 三个子范围 agent 产出合并。本文件 = A（mei 框架）+ B（布局/系统/交互）+ C（menu/photomode/text/许可/插件）。

## 目录
- [A mei UI 框架](#a-mei-ui-框架)
- [B 布局/系统/交互](#b-布局系统交互)
- [C 菜单/拍照/文本/许可/插件](#c-菜单拍照文本许可服务插件注册)

---

# A mei UI 框架

# systems/toolbar mei UI 框架模块内部实现分析（B 版，子范围 A：mei 框架）

> 由批次 3 分析 agent 产出，父级代为落盘。

- 二进制：B patched EXE，sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323，程序名 tiny-glade.exe。
- 地址 = 0x140000000 + 骨架 RVA（skeleton/B/crates/systems/toolbar/src/mei/*.rs）。
- 反编译：Ghidra get_functions，落 jobs/decomp/*.c。标注 O=观测/I=推断/U=未知。
- 复用：hashbrown、taffy、tg-text、ron、walkdir、lang-lib、bumpalo、country-core(GlobalId/WidgetParams/UiState/DragState/UiFocus/MainActionSet/WindowSize)、bevy 0.16。

## 0. 共享事实
### 0.1 GlobalId 隐式派生（O）
id = calculate_hash(&str,()) XOR 0xb1695320004fb443（safe_frame 0xf99230/new_at_pos 0xf992e0/decorate_hovered 0x67df20 逐字节）；子节点 id = calculate_hash<(u64,TypeId)> + calculate_hash<(u64,u64)>（child<&str> 0x32daf0 及 label/menu_button/tweaker_impl/image 等）。类型种子：Label=0xce69dbf0aee13da2、Image=0x1f6d60c8858b8ff、模板种子对 {0x41223169ff28813b,0xa79b7268a2a968d9}/{0xdc603d951b9380b2,0xce69dbf0aee13da2}。impl$0::as_id(0x683c10)=GlobalId 的 Hash（sip 写 16B）。

### 0.2 MeiContext（O）
+0x08/+0x10 有序 z 序列表（56B 条目）；+0x18/+0x20/+0x28 Vec<MeiInputEvent>（stride 0x70）；+0x30.. 每帧保存态（memcpy 0x170）；+0x1a0/+0x1a8/+0x1b0/+0x1b8 当前帧节点表 HashMap<GlobalId,Node>；+0x1d0/+0x1d8/+0x1e8 输入 handler 表；+0x200..+0x228 上一帧节点表（双缓冲）；+0x260 Arc<FontTable>；+0x268/+0x270 帧内临时。
关键设计：mei_pre_update 末尾把 +0x1a0/+0x1b0/+0x1c0 与 +0x200/+0x210/+0x220 互换并 RawTable::clear 旧表——未被 reuse_or_create_node 复用的节点每帧自动 GC。

### 0.3 Node（map 条目 stride 0xc0，键 GlobalId@+0，Node@+0x08）（O）
Node+0x00 children Vec<GlobalId>；+0x18 LayoutStyle(72B)；+0x60 状态区{u32,u16,u8[2]={4,0}}；+0x68/+0x70 计算 size(f32x2)；+0x78 计算 pos(f32x2)；+0x80 content Box 数据、+0x88 vtable；+0x90 u32 新建标记+旧字段；+0xa4 opacity=1.0；+0xb4/+0xb5 两声音 ID（默认 0x0d）；+0xb6 状态字节（bit0=hover,bit1/2/3=drag,bit4=clicked,bit6=focus）；+0xb7 行为标志（bit0=block_input/force_interactive,bit1=ignore_input,bit2=default_focus,bit5=with_nav）。
NodeContent 虚表：+0x30=type_name、+0x40=interactive、+0x48=process_input_action、+0x58=pre_update、+0x60=render、+0x68=draw。默认实现物理地址被别名到 0x3bb70/0x3bb80/0x2370（Ghidra 命名成 Plugin::ready/finish 等，空壳 return;）。
NodeContext=(&MeiContext,GlobalId,u64)。TemplateView +0x10=MeiContext、+0x18=id。

### 0.4 帧事件缓冲（O）
MeiInputEvent stride 0x70，tag@+0x58∈{0xc3,0xc4,0xc5,0xc6}（剪贴板=0xc5）。on_key_event 0x685fc0/on_clipboard_paste 0x6860d0 push。

## 1. core.rs
### 1.1 reuse_or_create_node — 0x140683ec0
hash_one 探 hashbrown（h2 控制字节 SIMD 组、-1 哨兵、掩码+0x1a8、桶+0x1a0），命中返 &Node；未命中：+0x1b0==0 时 reserve_rehash，remove_entry 清陈旧条目并把旧 id/标志(bVar7=旧&0xf3,local_da)带入新节点=身份回收；初始化 Node 各字段；len+1。U：+0x1b0 语义、0xf3 位、local_120/uStack_118 字段。

### 1.2 safe_frame<&str> — 0x140f99230
id=hash^0xb1695320004fb443→reuse_or_create_node→node+0x5d=0→(可选)node+0xb7=1→push tag=1 帧事件(stride 0x38,+0x30=id)→返回{events,id,0}。

### 1.3 new_at_pos<&str> — 0x140f992e0（同构）
tag=4，事件写 +4/+8/+0xc/+0x10(x/y/w/h)、+0x14=Vec2*、+0x1c、+0x30=id。登记：new_at_pos<String>(RVA 0x2289f80) 与 NodeContext::mei(0x885d0) 在 B 库无物理函数（Ghidra Function not found / ReVa 不在内存块 / 别名 get_instruction），算法以 <&str> 单态为准。

### 1.4 mei_pre_update — 0x1406842e0（约37KB）
序：puffin scope；存光标；清待清缓冲(memset 0xff)；遍历节点把 children id 插全局子集合；读光标+两输入队列(0x20)合并；路径解析；MainActionSet::deconstruct→handler 派发(数组 stride 0xc,逐个 process_input_action_handlers,处理掉压实)；Instant::now/elapsed<0x78(120ns)；MainActionSet::reconstruct；memcpy(+0x30,0x170)恢复；第二遍 content.pre_update(vtable+0x58)；第三遍输入/命中/状态机：interactive=!(flags&2)&&((flags&1)?1:content.interactive())；光标∈rect{pos@+0x78,size@+0x70}*scale→hover bit；交互节点组 WidgetParams::new().no_history_recording().occlude_by_world().allow_negative_distance()+UiSounds+sounds+UiState::add<String>；DragState::{is_some,is_started,is_finished}→state bit2/1/3；clicked 或(焦点&&just_pressed)→state|0x10；MeiInputEvent 派发(tag≠0xc6→vtable+0x48,KeyboardInput drop/Esc 处理)；两处条件写 WindowFocusEvent；双缓冲交换+clear。
U：子集合用途、0x78 阈值、tag0xc6、WindowFocusEvent 双条件。

### 1.5 mei_post_update — 0x1406884c0（94KB，最大）
序：puffin；读 WindowSize/mode/is_fence；fVar77=XMM1/1000；第一遍 render(vtable+0x60) 返回可见性(默认2)写回；bumpalo::Bump+compute_z_order_and_toposort：按+0x08 有序列表查表 push StackEntry(stride 0x40={id,pos,size,z,+0x38=1.0,+0x3c=flags&2})→反转→DFS 弹栈(iVar9==6 中止；转存 Vec<Event> stride 0x38；Node+0xac+=Node+0xa8；flags|=子flags；children 压栈 tag=5 z+1)；LayoutStyle::compute_min_size_with_children(taffy)+sort_unstable(ipnsort/insertion_sort)；排序后对 content 非空节点调 vtable+0x68(draw,center=0.5*+0x70++0x78)；UiFocus/toolbar 判定+just_pressed 组算 nav 方向(dx,dy)→焦点候选→flags|0x40；聚焦节点 ShapeStream(fill_color/corner_radius/encode_no_images)+TextRenderer::rect+CompoundEditBuilder::new_edit_sequence；清帧事件；state&=0xcf。
算法：渲染两遍+DFS 拓扑 z 序+焦点/nav。U：render 返回值、+0xa8/+0xac、焦点判据、nav 动作名。

### 1.6 on_key_event / on_clipboard_paste — 0x140685fc0/0x1406860d0
包装 MeiInputEvent push（cap@+0x18,data@+0x20,len@+0x28；grow_one 泛型名被 rhapsody 污染）。

### 1.7 Node 方法（0x140683c20..cd0）
set_sounds(+0xb4,+0xb5)；ignore_input(+0xb7|=2)；force_interactive(|=1)；default_focus(|=4)；with_nav(b<<5|&0xdf)；block_input(+0xb7=1)。hover/has_focus/prev_computed_layout 仅 return;（getter 内联）U。

### 1.8 NodeContext 系列
node/node_mut(0x681d50/0x681e50,期望失败 panic)；child_with_global_id(0x6861b0)/into_child(0x686300)=reuse_or_create_node 子节点+父 children append；expect_child_with_global_id(0x686530)=仅构造(断言在调用侧)；has_global_id(0x686450) 探表返 bool；layout(0x686550) 写 Node+0x18..+0x60；ignore_input(0x686690)/with_nav(0x686790)；child<&str>(0x32daf0)；with<closure>(0x32d8b0)=遍历 list(0x20) 逐个 menu_button+挂回调。

### 1.9 process_input_action_handlers — 0x140687eb0
从 +0x1d0/+0x1d8/+0x1e8 handler 表逐节点探表→应用 handler→处理掉移除。U：handler 条目结构。

### 1.10 impl$23::build — 0x1406868d0
insert_resource<MeiContext>；PreStartup 挂 (GraphicsDeviceResource,Commands) 系统；Startup 挂 ResMut<MeiContext>；Update 挂 mei 系统 chain()+in_set/after 排序。U：Update 系统集合。

## 2. config.rs
### 2.1 load_configs — 0x140682e70
walkdir 扫目录→extension()=="ron"→relative_to+file_stem→read_node_config_from_file→log"Loaded UI config from"→HashMap<String,Arc<NodeConfig>>。U：log 级别。
### 2.2 read_node_config_from_file — 0x14068fe40
fs::read_to_string→ron::Options::from_str_seed<NodeConfig>（失败 panic_fmt）。
### 2.3 load_font_table(config/MeiContext) — 0x14068fc60/0x140683a20
ron 解析 tg_text::FontTable→unwrap→Arc 存 +0x260（旧 Arc 递减）。
### 2.4 instantiate_ui_template_impl — 0x14068eb50
模板 id=hash(父,类型种子)→child_with_global_id→FrameContent+FrameView::draw；Frame 绘制参数 switch(tag0..6: tag2→1.0,tag3/4→字节,tag5→4f32,tag6→2f32,default→4 字符串)；子表(stride 0xc)递归 instantiate 子模板或文本节点：解析"{...}"→lang_lib::lookup→LabelContent→TextFragment::{set_text,set_metrics,set_max_width,set_family,set_align,set_weight}(可选字段 plVar19[4..7])。LabelContent 字体大小@+0x258。U：UiTemplate 字段布局。
### 2.5 TemplateView
template(0x68bee0)→template_with_id<u64>；template_with_id(0x32e210/0xf9e970)、slot(0x4fe340)/into_slot(0x4fe550)、with_slot 数十单态(0x4ff030..0x503950)；node_mut(0x68ea60)=TemplateView+0x10/+0x18 探表。

## 3. layout.rs
### 3.1 LayoutStyle（0x14068e850 default 等，stride 0x48=72B）
+0x00..0x20 4×LengthPercentage(padding,每项{标志,f32})；+0x20/+0x24 gap；+0x28/+0x30 size；+0x38 expand(f32,expand 写 1.0)；+0x40 u64；+0x44 u16 swizzle(0=XY,1=YX)；+0x45 align(xalign_center=2)；+0x46 justify(align_center=1,space_between=3)。horizontal(0x68e880)。
### 3.2 place_childen/place_childen_impl — 0x14068ea20/0x140fe12a0/0x140fe19a0
入参(self,parent_size,children(0x18))：解 padding(百分比×parent)→可用=parent-(pad_l+r)-(n-1)*gap→遍历子(content+4 size、style+0x2c min、weights+=style+0x38,max 含 NaN)→剩余=可用-Σsize→按+0x46 justify 跳转表(0x142bc223c/0x142bc2260)分派(start/center/space_between/around/evenly，RON 变体名 "SpaceBet…"/"ceEvenly" 混入 switch)→产出 Vec<ComputedLayout>(grow_one,{pos,size})。U：justify 全枚举、反编译大量坏分支。
### 3.3 ComputedLayout::center — 0x14068ea00 return;（getter）U。

## 4. widgets.rs
### 4.1 构建器（全部同构：hash→child_with_global_id→get_content_mut_impl<T>→填内容→返回*View）
label(0x68bf00,种子0xce69dbf0aee13da2)、label_with_id(0x228a0e0)、menu_button(0x68bfc0)、menu_button_no_nav(0x68c260)、menu_button_with_id(0x68c500)、settings_button(0x68c790,MenuButtonContent+set_text/metrics/weight)、image<&str>/<u64>(0x32e0b0/0x32df50,种子0x1f6d60c8858b8ff)、tab_switcher(0x32dce0)、space(0x68ca90,*(mei+0x10)+1 计数)、action_hint(0xf9fdf0)、pill_button(0x32e5d0)、back_button(0x6811c0,调 pill_button+改 flags)、frame。
内容结构：ImageContent +0x40=tint u32、+0x44=blur_extend bool；PillButtonContent +0x40=inactive_opacity f32、+0x44=hotkey u8；MenuButtonContent +0x25d=hotkey u8。
### 4.2 视图方法
LabelView::{max_width(0x67e390),align(0x67e3e0),family(0x67e420),weight(0x67e460),color(0x67e4a0)}；MenuButtonView::{on_click(0x67e5b0),hotkey(0x67e6d0),weight(0x67e8d0),max_width(0x67e910),align(0x67e960),color(0x67e9a0),force_highlight(0x67e9e0)}；ImageView::{tint(0x67eab0),blur_extend(0x67eaf0)}；FrameView::draw(0x6803f0)；TabSwitcherView::delta(0x680620)=getter；PillButtonView::{inactive_opacity(0x6814f0),size(0x681530),hotkey(0x6816c0)}；ActionHintView::{into_inner(0x681f40),as_inner_mut(0x67dd80)}；make_hotkey_tooltip(内联)。U：on_click 闭包槽位、metrics 字段名。

## 5. widgets/tweaker.rs
tweaker_impl(0x32e920) 通用构建。tweaker<TweakableBool>(0x330940) 结构：tweaker_impl→delta→hover→MaybeHint::confirm(悬停提示)→标题 label(metrics/family/max_width/weight/color)→horizontal 行→按 delta 重复 impl$6::decrease/increase→[−]menu_button_no_nav(on_click,Node::used 点击→decrease)→impl$6::format 值 label→[+]→increase。tweaker<FloatWithStep>(0x332010) 同构用 impl$4。
原语：impl$4::decrease/increase(0x67da10/0x67da90)=value=roundf((value∓step)/step)*step(吸附网格)→clamp(min,max,min>max panic)；状态{值,step,min,max}。impl$4::format(0x67db10)=value>=0.1?精度1:精度2。impl$6::decrease/increase(0x67dc20/0x67dc30)=置 0/1；impl$6::format(0x67dc40)=lang_lib.lookup(键长10/11 本地化是/否)。impl$7::format<DisplayMode>(0x1584d0)=tag0/1/2 三分支 lookup。impl$8::format<Resolution>(0x158770)=fmt::write"W×H"。TweakerView::delta(0x67d980)、HintTracker::text(0x67dcf0)、MaybeHint::confirm(0x67dd00)=push_str。U：enum 版 decrease/increase 取模逻辑、TweakerContent 字段。

## 6. widgets/decorate.rs
decorate_hovered — 0x14067df20：节点 hover(state bit0)时：子id=hash(GlobalId,u32)^0xb1695320004fb443→reuse_or_create_node→+0x5d=0→push tag=3 事件(Vec4{-5.0,(-0.0,-1.0),1.0},+0x28=1.0,+0x30=id)→image 子节点(写 Node+0xa8=100.0)；再子id+1 第二个 image(Vec4{5.0,1.0})。=悬停描边装饰。U：tag3 消费、+0xa8 字段。

## 7. audio.rs
仅 audio::impl$0::default(0x68e840,return;) 一个符号。O/U：音频逻辑为内联宿主；声音 ID 由 Node::set_sounds 写 +0xb4/+0xb5(默认 0x0d=13)，播放接线在 ui_interaction.rs（本子范围外）。U：0x0d 哨兵语义。

## 逻辑总览
mei=自研双缓冲保留模式 IMGUI：节点身份=名字/父/类型种子哈希 XOR 0xb1695320004fb443；节点树=HashMap<GlobalId,Node>(hashbrown,192B 条目)；每帧 safe_frame/new_at_pos/child_with_global_id 经 reuse_or_create_node 复用；mei_pre_update(输入→handler→hover/click/drag 状态机→事件派发→焦点)→用户画 UI(widgets/tweaker/模板)→mei_post_update(render 两遍+拓扑 z 序+taffy 最小尺寸+焦点/nav)；帧末双缓冲交换清空旧表自动 GC。配置=RON 目录扫描+模板递归实例化+lang_lib 本地化。布局=mei LayoutStyle 盒模型+taffy。控件=各构建器+视图；tweaker=步进器(网格吸附+clamp+精度 1/2+本地化)；decorate_hovered=悬停描边。

## 算法清单
GlobalId 隐式派生(名字/父+TypeId 哈希 XOR 种子)；hashbrown 探表+插入+陈旧身份回收；双缓冲节点表交换+清空(retained GC)；输入 handler 表派发+压实；光标命中+hover/click/drag 状态机；事件→content 派发(MeiInputEvent tag 0xc3..0xc6,stride 0x70)；DFS 拓扑排序 z 序(父先弹、子入栈 z+1)；taffy 最小尺寸+排序(ipnsort/insertion_sort)；RON 目录扫描+模板递归实例化+lang_lib 本地化；盒模型子布局(padding/gap/weight/justify/swizzle)；步进器(roundf 吸附+clamp+精度 1/2+本地化值)；悬停描边(计数器 id+1+tag3 事件)。

## 未知清单（U）
1) new_at_pos<String>(0x2289f80) 与 NodeContext::mei(0x885d0) 无物理函数（证据缺口，以同构单态覆盖）；2) NodeContent 默认方法真实返回值（地址别名）；3) reuse_or_create_node 的 +0x1b0/0xf3/旧字段；4) mei_pre_update 子集合用途、0x78 阈值、tag0xc6、WindowFocusEvent 双条件；5) mei_post_update render 返回值、+0xa8/+0xac、焦点判据、nav 动作名；6) handler 条目结构；7) LayoutStyle justify 全枚举与跳转表、place_childen_impl 坏分支；8) LabelContent/MenuButtonContent/TweakerContent 完整字段；9) FrameContent 字段映射、UiTemplate stride0xc 布局；10) enum 版 tweaker decrease/increase 取模；11) Node 三个 getter 体内逻辑；12) audio 声音 ID 哨兵语义与播放接线。

## 依赖（复用不逆向）
hashbrown、taffy、tg-text(TextFragment/TextRenderer/FontTable)、ron/walkdir/relative-path、lang-lib、bumpalo、iced_core(泛型名污染)、bevy 0.16、country-core(GlobalId/WidgetParams/UiState/DragState/UiFocus/MainActionSet/WindowSize)。

## 证据路径
本报告；反编译 jobs/decomp/*.c(132 份)；jobs/batch{1..8}.json 与 -result.json；jobs/reva-mei.json/-result.json；骨架 skeleton/B/crates/systems/toolbar/src/mei/{core,config,layout,widgets,audio}.rs、widgets/{decorate,tweaker}.rs。

---

# B 布局/系统/交互

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

---

# C 菜单/拍照/文本/许可服务/插件注册

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

---

## 批次 3 复核修正（登记，详见 BATCH3-LOGIC-REVIEW-toolbar.md）

1. 补遗漏发射函数 `taffy_finalize_element_size`(0x142340890，1152B，含 TextFragment::bounds + taffy_mut_style)——真实业务，B 段未写。
2. 卡点纠错：`new_at_pos<String>`(0x142289f80) **存在** 268B 物理函数且与 A1.3 逐字段吻合；仅 `NodeContext::mei`(0x885d0) 确无函数。原 unknown #1 修订。
3. `ui_sounds`(0x14233a7f0) 尾部边界错并坐实（并入 drop_ptr<StairsRemovedSupports/TreeStorage> + 组件索引 jumptable）。
4. 跨部分不一致：Node 偏移 A(+0xa4 opacity) vs C(+0xb0/+0xb8 alpha) 需统一；A 的 vtable 缺 on_input 槽（实际 8 个 on_input 单态）；tag/判别子三义混用；0x28B 为笔误；`param_to_value` 是否互逆待核。
5. 补 tweaker enum/list 族、菜单内容 trait（credits/load/report_bug）、NodeContext 剩余方法。
