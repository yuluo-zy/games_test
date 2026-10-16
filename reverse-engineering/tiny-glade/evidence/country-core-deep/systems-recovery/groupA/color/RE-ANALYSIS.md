# groupA RE-ANALYSIS — system_color（调色系统）

dbid: tg_xa1 · 2026-10-16 · decomp: `groupA/color/decomp/`

## 1. 覆盖盘点（26 业务）

| VA | PDB 名 | 结论 | 说明 |
|---|---|---|---|
| 0x142298d30 | autoclose_color_menu | O | 相机远离→菜单自动关闭 |
| 0x1422979b0 | color_button_animations | O | 颜色按钮动画（easing） |
| 0x142291230 | color_menu | O* | 径向调色菜单交互主系统（*结构级，见 §4 限制） |
| 0x142297ff0 | delete_color_icon_entities | O | 图标实体 despawn |
| 0x14288e990 | GraphicsDevice::create_buffer | O(dep) | rhapsody 渲染器 create_buffer —— dependency_reuse（渲染器库），game 调用策略见 §5 |
| 0x140651870 | impl$0::add_color | O | ColorPlugin::build（17 系统注册 + 资源） |
| 0x141c7f270 | replay_color | O | 历史回放（颜色编辑命令） |
| 0x1422af0c0 | set_decorator_color_id | O | 装饰物（楼梯等）颜色写入 |
| 0x142298970 | set_flag_connection_and_chimney_smoke | O | 旗连接/烟囱烟开关 |
| 0x142299840 | set_gate_color_id | O | 门颜色写入（tag 7） |
| 0x142299250 | set_halftimber_color_id | O | 半木颜色写入（tag 0xc） |
| 0x142299dd0 | set_roof_customization | O | 屋顶样式/颜色同步（tag 4 历史） |
| 0x142290750 | init_color_ids_buffer | O | 三色表资源注册 + 菜单登记 |
| 0x142290820 | set_wall_color_id | O | 墙颜色写入（tag 6） |
| 0x142298260 | set_window_light | O | 窗灯颜色写入 |
| 0x1422adcf0 | hide_color_menu | O | 菜单隐藏（action 门禁+despawn） |
| 0x14229a7f0 | show_color_options | O(受限) | 颜色选项构建 —— Ghidra 反编译超时（165 alloc）；以 closure$0+callee 表+串取证，见 §4 |
| 0x1422b0e00 | ui_color_windows_and_flags | O | 窗/旗 UI（decorator 拾取→菜单） |
| 0x1422afe70 | ui_color_gates | O | 门 UI（mesh 位置→菜单锚点） |
| 0x141c9ab70 | ui_color_wall_and_roof | O | 墙/屋顶 UI（color_canvas widget） |
| 0x141c9b510 | color_cursor_icon | O | 光标取色图标（signifier entity） |
| 0x142298520 | write_roof_color_id_to_gpu | O | 屋顶/墙颜色逐项脏写 GPU SSBO |
| 0x1425f26f0 | IconInfo::new_flat_roof_rim_ty | O | 图标（freehand_ty 串，PDB 名错位候选） |
| 0x1425f3d60 | register_required_components | O(胶水) | bevy required-components 注册（递归） |
| 0x1425f2680 | IconInfo::new_separator | O | 分隔图标（'white' 串，kind=5） |
| 0x1425f27c0 | ColorRadialMenu::create | O | 径向菜单实体生成 |

## 2. 关键契约

### 共享面
- **SSBO 写链**：CPU 侧 Vec<u32>（按墙/门/半木/屋顶索引，索引上限断言 `MAX_NUMBER_OF_WALLS=20480`，0x4fff=20479）→ `0x140beefa0`（**StagingSsbo 脏区上传**：参数 {out, 类型名, 缓冲信息, 数据, len, 脏区 {start,end} 对, 区数, 偏移}；0xb=缓冲不足错误码；panic 串区 0x142b15ae8/0x142b15b00 为 rustc slice.rs）→ GPU。缓冲按 **类型名** 从 `0x1425f18b0`（HashMap<TypeId, Arc<String>>，hashbrown 16B 键 + Arc 计数）取 Arc<String>（值 = 'country_core::resources::color_id::WallColorIdSsbo' 等 50B 串，0x143136830/89a/862/8cc）；`0x140987ca0` = 该表取值辅助（<40B 未建函数，U 精确体）。
- **实体颜色记录 0x20B**（ResMut<Vec>，grow 0x140562c30）：{u64 实体/id, u64 打包中心（Aabb2::center 0x1412400c0 + pack 0x140c9cde0）, f32 max_y×0.4922（PublicWallState::max_y 0x140b2a620）, u8 tag}：**tag 6=墙, 7=门, 0xc=半木, 10=装饰物**（set_decorator 分支）。AABB 由实体点表（entity[8]/[9]）minps/maxps 求（常量 0x142ac2e20 族 = ±f32::MAX 初始化）。
- **图标网格路径**（.rdata 0x3039d78/0x3135358 打包区，全量）：
  flags{red,pink,yellow,green,dark_sea,blue,light_purple,purple,dark_bourbon,dark,flag_yellow}; windows{white,wood,dark_wood,purplestone,red,blue,no_color,…}; lights{light_1,2,3}; railings{railing_1..4}; chimney_smoke_{inactive,active}; flag_{no_}connection; freehand_ty_{constant,terrain}; bricks_{beige,yellow,light,grey,dark}; plaster_peel_{low,medium,high}; skirt/no_skirt; halftimber; locked; roof_{ochre,red,purple,blue,light_blue,turquoise,green_light,green,dark_wood,dark,grey}; color_cursor。
- **IconInfo**（0x1425f2680/26f0）：{String 名(len/ptr/cap), u8 kind@+0x18, u32 @+0x24}；kind 5=separator('white'), 8=类型图标（freehand_ty_constant(31B)/freehand_ty_terrain(30B)）。PDB 名 new_flat_roof_rim_ty vs 串 freehand_ty —— **名实错位候选**（restorer 按 freehand_ty 语义落码）。

### 逐函数
- **init_color_ids_buffer (0x142290750)**（3 参, param_3=Commands）：`WallColorIds::new`(0x140b19890)+insert_resource(0x1422bedb0, 位置 0x14303b0e8)；`GateColorIds::new`(0x140b19900)+insert_resource(0x1422bef20, 0x14303b100)；`HalftimberColorIds::new`(0x140b19970)+insert_resource(0x1422bec40, 0x14303b118)；+ 4×0x140987ca0→菜单登记（0x1425f1640/13d0/0ef0/1160）。
- **set_wall_color_id (0x142290820)**（8 参）：空间哈希（0x18B 条目 {id@+8, 新色 u32@+0x10, custom u8@+0x14}）遍历：`idx = 0x1400885d0(id)`（PDB RoofRidgeDims::get_inner；机器码=id→索引哈希，U 精确）；断言 `idx < 0x4fff`（panic 'assertion failed: wall_index < MAX_NUMBER_OF_WALLS' 0x14303b160）；写 `SsboVec[idx]=色`；PublicWalls::get（0x140a91ef0）→ custom 时 AABB/center/max_y 打包推 0x20B 记录（tag 6）；**脏色分类**：新色 ∈ {4..10,12} → 推 {id,色} 到"自定义列表"（param_7），旧色 ∈ 该集且新色非 → "回默认列表"（param_8）—— 供 write_roof_color_id_to_gpu 合并；末尾整表 flush（0x140beefa0，len==0 提前返回）。
- **set_gate_color_id (0x142299840)**：同构；key 0x143136740→'…GateColorIdSsbo'；panic 串复用 wall_index 文案（**门与墙共享索引空间**）；tag 7；无脏色分类（整表 flush）。
- **set_halftimber_color_id (0x142299250)**：同构；key 0x143136710→'…HalftimberColorIdSsbo'（0x143136862）；tag 0xc；panic 同文案。
- **set_decorator_color_id (0x1422af0c0)**：DecoratorStorage 遍历（lookup_decorator_info_mut 0x140a1cbf0 ×9）；`DecoratorExtraInfo::as_stair(info+0x30)`（0x1408394b0）→ 命中写 `stair.color@+0xc`；custom 时 CachedDecoratorTransform（0x140a1a9f0）→ 推 0x20B 记录 tag 10；panic 位置全部 set_decorator_color_id.rs（0x14303cb70 族）。
- **set_window_light (0x142298260)**：`as_window_mut`（0x1408393b0）→ 窗灯色；CachedDecoratorTransform（0x140a1a9f0, 位置 0x14303bad0）+ push（tag 候选 9，I）。
- **set_flag_connection_and_chimney_smoke (0x142298970)**：双支：`as_chimney_mut`（0x1408392d0）烟囱烟开关 + `as_flag`（0x140839340）旗颜色/连接；各推 CachedDecoratorTransform 记录（位置 0x14303bc48/0x14303bc78）。
- **set_roof_customization (0x142299dd0)**（8 参）：屋顶哈希（0x20B 条目 {id@+8, 样式 u8@+0x10, 变体 u8@+0x11, custom@+0x12}）→ `PublicWalls::roof_entity`（0x140a922b0）→ 屋顶记录（0x48B）：样式字节比较（record+0x3d vs entry+0x11）不同 → push {实体, id} 到"样式变更列表"（0x10B）+ 写颜色数组 + 更新样式字节；custom → History::add（0x50B {1,1,4,…}, edit=8, 位置 0x14303bfc8）。Roof::pivot_3d（0x1408e3220）×2 参与定位。
- **write_roof_color_id_to_gpu (0x142298520)**（4 参）：key 0x143136738→'…RoofColorIdSsbo'（0x1431368cc, 55B）；双哈希遍历：屋顶（0x40B 条目）逐项 `0x140beefa0(…, 4 字节, offset=idx<<2)`；墙（0x18B 条目）同 —— **逐元素脏写**（区别于整表 flush）。
- **ui_color_wall_and_roof (0x141c9ab70)**：widget 'color_canvas'（0x142f25026, 12B）；WidgetVolumeShape::clearing_canvas(3)（0x1409a9110）；RaycastWorld::raycast（0x1408cc870, mask 0xcb, max=f32::MAX=0x142925980）；双 widget 工厂（0x1422bc980/0x1422bcc20 = WidgetParams::signifier 单态 + 0x1422bd3c0/0x1422bd120 = widget 工厂）；UiSounds 全接；点击→push 0xb0B widget 参数拷贝到输出 Vec。
- **color_cursor_icon (0x141c9b510)**：SignifierEntityBuilder::new(0x14089d9b0)+animation_duration(0, 0x14089dc90)+build(0x14089dd50)+EntityCommands::insert（0x1422beab0，名字串 0x142f25098 区）；mesh 路径 'color_icon/color_cursor'（0x142f25080）—— 取色光标跟随。
- **ui_color_gates (0x1422afe70)**：AssetMeshLibrary::get（0x14092d2e0）→ Mesh::positions（0x140a07b30）→ 门网孔位置（菜单锚点/图标布局）；widget（0x1422bd120）+ clicked 门 → 菜单事件；signifier 生成（0x14089d8b0, 位置 0x14303ccd8）+ UiSignifierStream::write（0x140b2cf90）。
- **ui_color_windows_and_flags (0x1422b0e00)**：legacy_raycast_w_params（0x1408cc770）拾取；DecoratorStorage::lookup_decorator_info（0x140a1c930）/lookup_storage（0x140a1c3e0）/decorator_info（0x140a1c5f0）；StairsState::graph（0x1408fe8b0）；DerivedDecoratorInfo::subtype（0x140b26970）+ 0x140b26a70；0x1425f4610/0x1425f4950（color crate 装饰物→颜色 UI 辅助）；EventWriter ×3（0x142291100 —— 菜单打开/应用事件）；widget 0xb0B 参数。
- **color_menu (0x142291230)**（结构级 O，3351 行/137KB）：15+ 系统参数；0xa0B/图标状态条目数组（0x142291230 内 uVar31*0xa0 布局：{flag@0, …@+0x60…}）；驱动：`DigitalActionState::just_pressed`（0x14093fa90）×14（动作对 {id,state} 位于动作资源 +0x98/+0xa0/+0xa4/+0xa8/+0xac —— 径向菜单 选择/确认/取消 动作族）；widget 工厂 0x1422bd120 ×12（11 图标 + 背景）；每图标块：is_active→clicked→应用；`cartesian_to_polar`（0x140c90e20）+`polar_to_cartesian`（0x140c90e70）+`AngleRange::clamp`（0x1425f24d0）—— **图标角度布局**；`History::try_start_input_recording`（0x140a86510）×1 + `History::add`（0x140a84ec0）×13 —— 每次颜色选择记录编辑；`FreehandHeightType::toggle`（0x1408c5660 单态）+ Roof/PublicWalls 读取（roof_entity 0x140a922b0、get 0x140a91ef0）判定可染色目标；`ColorMenuBrightnessValue::to_f32`（0x140b06770）×3（亮度图标 0.25/1.0/4.0 表，见 audio-misc 章节）。
- **show_color_options (0x14229a7f0)**（O 受限）：Ghidra 反编译超时 ×2（165×alloc 巨型 UI）；证据面：callee 全表（skeleton）+ closure$0（0x1422acce0, 已反编译 = 每图标字符串构造：17..28B 长 mesh 路径分配 ×20）+ 调用 ColorRadialMenu::create ×5 / IconInfo::new_flat_roof_rim_ty / new_separator / closure$0 ×4 / Roof::is_flat（0x1408e3380）×2 / StairsState::get_all_connected_nodes（0x1408fe8c0）×2 / DecoratorExtraInfo::try_as_stair（0x140839490）×4 / DecoratorStorage::lookup_decorator_info ×13 / DerivedDecoratorInfo::subtype ×3 / MainCamera::eye_position ×5 —— 按目标类型（墙/屋顶/楼梯/窗/旗/半木）组装对应图标菜单。**restorer 落码依据 = 图标串全表 + 分支目标判定链**。
- **ColorRadialMenu::create (0x1425f27c0)**（13 参，float 族 = 中心/半径/尺寸）：AngleRange::from_min_and_len（0x1425f2460）+ to_f32；Commands::spawn ×2；图标实体生成（含亮度值 → 位置角）；0x140ec30e0/0x140ec4530/0x141c5a430（UI 辅助，U）。
- **color_button_animations (0x1422979b0)**：QueryIterationCursor::init（0x1425f25a0 = bevy Query 迭代）+ `EasingFunction::apply`（0x140c95ef0）×3，easing 数据 0x14303b9f0/14303b9f8 —— 按钮缩放/透明度动画。
- **autoclose_color_menu (0x142298d30)**：MainCamera::eye_position ×2 → 与菜单锚距离超阈值 → 关闭菜单（隐藏命令），Vec grow 0x140562630。
- **hide_color_menu (0x1422adcf0)**：`just_pressed(动作资源+0x90/0x94)`（隐藏动作 = 右键/取消 候选）→ EntityCommands::despawn ×2 + 多列表清理（grow 0x140562db0/630/c30 混合 = 图标实体表/状态表清空）。
- **replay_color (0x141c7f270)**：ReplayBuffer::front_mut_bucket（0x1409f1a80）+ 0x140949170 → 逐命令重放：多目标 Vec（grow 0x562db0/630/c30 ×16 —— 墙/门/半木/屋顶各色表 + 图标实体表）—— 颜色编辑历史回放（与 set_* 对偶）。
- **register_required_components (0x1425f3d60)**：bevy required-components 注册（ComponentInfo::new 0x140d3f870 / QueuedRegistration::register 0x140d3fa40 / enforce_no_recursion 0x140d40d70 / hashbrown reserve 0x1428d1fc0）；递归注册链；TypeId 模式串 0x142950573 族（16B×13）。
- **add_color (0x140651870)**（=ColorPlugin::build）：17 × {0x1400dd6f0 哈希检查 + 0x140ecxxx 注册调用}（系统注册）+ App 方法族（0x140ed02c0 add_systems / 0x140ed1780 / 0x140ed2770 init_resource / 0x140ed3630）+ 0x140ec6670 —— 注册 §1 中全部 color 系统 + 资源。

## 3. 小函数（手解/机器码）

- **0x140beefa0 StagingSsbo 上传**（O，alias-decomp 全解）：参数 {out, 类型名串?, 缓冲头(param_3 含 +0x14 校验/+0x16 标志), 数据, len, 脏区对数组, 区数, 偏移}；校验失败/越界 → panic（0x142b15ae8/0x142b15ab8/ad0 = slice.rs 路径）；成功 0xb 传播。
- **0x1425f18b0 类型名表 get**（O）：Arc<HashMap<u64×2 键, Arc<String>>>；hashbrown 探测（h2 字节复制比较）+ Arc inc/dec；返回 Option<Arc<String>>。
- **0x14092e5f0/0x14092e680**：UiWidgetContext::clicked/is_active（见 water §3 手解）。
- **0x14093fa90 DigitalActionState::just_pressed({u32 id, u32 状态})**。
- **0x140987ca0**：类型名表取值辅助（<40B，U 体）。

## 4. 限制与纠错

- **0x14229a7f0**：Ghidra `DecompilationFailed: timeout`（600s 复测仍超时）—— 不做假反编译；契约以 callee 全表 + closure$0 + 串表取证（O 受限）。
- **IconInfo::new_flat_roof_rim_ty 名实错位**：PDB 名 flat_roof_rim，串 = freehand_ty_constant/terrain —— 机器码为准（freehand 类型图标）。
- **0x1400885d0**：sysres 标 RoofRidgeDims::get_inner；set_* 中实为 id→索引（wall/gate 共用），PDB 双名冲突登记。
- **0x140b34f80**：HighlightColor::to_rgb vs split_height_into_skirts（timberframe 域冲突，同址）。
- **0x140a84ec0**：sysres 标 nuke_future_states::closure$0；实际 = History::add 内部（多域同证）。
- **MAX_NUMBER_OF_WALLS = 20480**（0x4fff 断言上限）—— 墙/门/半木索引共用此界。

## 5. 跨域线索

- `rhapsody::raw::device::GraphicsDevice::create_buffer`（0x14288e990）= rhapsody 渲染器库 API（dependency_reuse；game 调用策略：color/SSBO 缓冲创建经它，rhapsody 域已有恢复入口）。
- SSBO 类型名四串（Wall/Gate/Halftimber/RoofColorIdSsbo）与 core 侧 chapters/resources/color_id*.md 的 `*ColorIdSsbo` 实体一一对应 —— **restorer 交叉点**；core 侧 `default_value` 纠错（0x14099ca50 属 roof=1）与 set_* 的 0..3 默认色语义相关。
- GladeColorCustomization（save 域章节）15 字段表（freeform_wall/building/roof/halftimber/…偏移 0x18..0x42）—— color_menu 写入目标（经 History 编辑）。
- 0x1408392d0/340/3b0/420/4b0（DecoratorExtraInfo 访问器）—— 墙域 decorator.rs 章节化面。
- EventWriter 0x142291100 ×3（ui_color_windows_and_flags）—— 菜单事件族（应用/取消/切换候选，U 事件类型名）。

## 6. U 项（不臆造）

- show_color_options 内部逐图标分支精确顺序（受限 O）。
- 0x140987ca0 函数体（<40B 未建函数）。
- color_menu 的 11 图标 ↔ 目标类型完整映射表（结构级证据足，逐项 U）。
- 事件类型名（0x142291100 族）。
- add_color 中 17 个 0x140ecxxx 注册调用的系统名映射。
