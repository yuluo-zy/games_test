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
