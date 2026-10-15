# systems/color 模块内部实现分析（B 版）

- 二进制：B patched EXE，sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`，程序名 `tiny-glade.exe`。
- 映像基址 0x140000000；地址 = 0x140000000 + 骨架 RVA（骨架 `skeleton/B/crates/systems/color/src/*.rs`）。
- 反编译来源：Ghidra `get_functions`（fields=signature,decompiled_code），结果落 `jobs/decomp/*.c`；batch 见 `jobs/batch{1..4}.json`、`jobs/result{1..4}.json`。
  - `color_menu`(0x142291230) 与 `show_color_options`(0x14229a7f0) Ghidra 反编译返回空（函数过大 / 反编译器失败），ReVa 10s 超时，radare2 无该函数符号；两者仅以 PDB 候选接口 + 原始反汇编确证存在，内部逐行逻辑 = unknown。
- 标注：**O**=观测/原指令证实；**I**=结构/上下文推断；**U**=未知。
- 复用基础（不重复恢复）：`country-core` 的 `resources::color_id`、`resources::color_id_roof`、`resources::walls::decorator`、`render::ssbo`、`asset_libraries::ssbo_library`；Bevy 0.16 ECS；egui 0.27（UI）；utils（geometry/aabb/swizzlers/easing/angle_range）。

---

## 0. 共享事实

### 0.1 颜色 ID 的 SSBO 缓冲区（O）

`init_color_ids_buffer` 用 `AssetSsboLibrary::add_new::<T>()` 注册 4 个 SSBO，`set_*_color_id` 用 `FrozenMap::get<SsboId>` + `memcmp(name,len)` 取回。memcmp 长度与类型全名长度**逐字匹配**，据此确定 SSBO 名字字符串（O）：

| SSBO 类型 | 名字字符串 | 长度 | 使用系统 |
|---|---|---|---|
| `country_core::resources::color_id::WallColorIdSsbo` | 同左 | 0x32 = 50 | set_wall_color_id |
| `country_core::resources::color_id::GateColorIdSsbo` | 同左 | 0x32 = 50 | set_gate_color_id |
| `country_core::resources::color_id::HalftimberColorIdSsbo` | 同左 | 0x38 = 56 | set_halftimber_color_id |
| `country_core::resources::color_id_roof::RoofColorIdSsbo` | 同左 | 0x37 = 55 | write_roof_color_id_to_gpu |

证据：反编译 `memcmp(&anon_..., ptr, 0x32/0x37/0x38)` + 二进制字符串存在性核对（WallColorIdSsbo=50、GateColorIdSsbo=50、HalftimberColorIdSsbo=56、RoofColorIdSsbo=55）。

### 0.2 颜色 ID 资源（O）

`init_color_ids_buffer` 另 `insert_resource`：`WallColorIds::new()`、`GateColorIds::new()`、`HalftimberColorIds::new()`（均 `country_core::resources::color_id::*`）。三者的 `set_*` 系统按**索引**读写为 `Vec<u32>`：

- ResMut<T> 布局：`+0x08` = data ptr、`+0x10` = len、`+0x1c` = 变更 tick（写 `*res[2]` 触发 set_changed）。O：`set_wall_color_id` 中 `*(lVar16+8)+idx*4` 读旧值、`*(lVar16+0x10)` 为 len、`idx*4` 步长。
- 每个 `Set*ColorIdCmd` 事件携带（事件 stride 0x18）：`+0x08` = 实体、`+0x10` = 新颜色 id(u32)、`+0x14` = 是否 halftimber/触发布局重算的 bool。

### 0.3 IconInfo（O，来自 new_separator/new_flat_roof_rim_ty 逐字节）

`system_color::radial_menu::IconInfo`，stride **0x28 = 40 字节**：

| 偏移 | 含义 | 依据 |
|---|---|---|
| 0x00..0x18 | 名称（String：len@0 / ptr@8 / len@0x10，Ghidra 渲染顺序） | O：`new_separator` 写 "white"，`new_flat_roof_rim_ty` 写 "color_icon/freehand_ty_constant" |
| 0x18 | **图标类型 tag（u8）** | O：`new_separator`=5、`new_flat_roof_rim_ty`=8 |
| 0x19 | u8（部分构造器使用） | O |
| 0x1a | u16 | O |
| 0x1c | u32 / f32 值（颜色 id 或开关值；demo 分支写 0x309=777） | O |
| 0x20 | u32 | O |
| 0x24 | u32（new_* 均置 0） | O |

`new_separator`(0x1425f2680, 106B)：构造 `IconInfo{ name:"white", tag=5 }`。O
`new_flat_roof_rim_ty`(0x1425f26f0, 201B)：按 `ClutterVersion::is_initial()` 构造 `IconInfo{ tag=8, name:"color_icon/freehand_ty_constant"(len 31) 或 "color_icon/freehand_ty_terrain"(len 30) }`。O

### 0.4 其它结构（O/I）

- `country_core::resources::color_icon::CustomizationIcon`：stride **0xa0 = 160 字节**（图标实体组件）。动画进度 f32 在 `+0x18 + icon_index*0x24`（O，见 color_button_animations / delete_color_icon_entities）。
- 图标实体额外组件（`ColorRadialMenu::create` spawn 的 tuple）：`ColorIcon` / `WallStyleIcon` / `ColorWheelSeparator` / `FlagConnectionIcon` / `FreehandHeightTypeToggleIcon` / `FlatRoofRimTypeToggle` / `ColorMenuBrightnessIcon` / `ChimneySmokeToggle` / `FlatRoofStyleIcon` / `PlasterPeelIcon` / `StairRailingIcon` / `MagicColorIcon`（O，来自 create 的分支 insert 泛型名）。
- `country_core::resources::walls::decorator::DecoratorExtraInfo`：子类型/kind tag 在 **+0x46**（O，`set_*` 里 `*(char*)(info+0x46)`）。
- `country_core::resources::roofs::roof_shape::Roof`：stride **0x58**（O，`set_roof_customization` 里 roof_index*0x58；与 roof 模块一致）。字段：`+0x3d` 屋顶自定义/样式字节、`+0x40` flat rim 样式、`+0x41`、`+0x44` int、`+0x48` int(颜色 id)、`+0x50` 颜色 id(u32)。
- `PublicWalls`：元素 stride **0x14 = 20 字节**（`+0x04` 稀疏位索引、`+0x0c` 分块索引、`+0x10` mesh/roof 索引）；另有两处子表（`+0x100` 装饰器目录 stride 0xa0、`+0x1a8` 墙目录 stride 0x48），分块表 `+0x18`/`+0x38` 配 `~idx*0x30` 偏移（O，set_roof_customization / color_button_animations 中反复出现）。

### 0.5 颜色编辑事件清单（O，来自 add_color 注册顺序）

`add_color` 依序注册 15 个事件：ShowColorMenuCmd、HideColorMenuCmd、SetWallColorIdCmd、SetRoofColorIdCmd、SetDecoratorColorIdCmd、SetDecoratorBrightnessCmd、SetGateColorIdCmd、SetHalftimberColorIdCmd、SetFlagConnectionCmd、SetWindowLightCmd、SetChimneySmokeCmd、SetFlatRoofStyleCmd、SetLadderColorIdCmd、SetStairRailingCmd、SetMagicColorIdCmd。命名空间：`SetWall/Gate/HalftimberColorIdCmd` ∈ `country_core::resources::color_id`；`SetRoof/FlatRoofStyleColorIdCmd` ∈ 本模块 `set_roof_customization`；其余 decorator 类 ∈ `set_decorator_color_id` 或 `country_core::resources::walls::decorator`。

### 0.6 “命令 push” 约定（O）

各 `set_*` 系统在重算布局后，把「受影响实体」推进一个 `Vec`（元素 stride **0x20**，尾字节为**命令 tag**）。tag 值：6=wall（set_wall_color_id）、7=gate、0xc=12 halftimber、9=flag、0xf=15 chimney smoke、0xb=11 roof、8/10/0xd/0xe=window/其它 decorator（set_window_light、set_decorator_color_id）。这些 tag 是后续「视觉重建」的类别判别值。O（各函数尾部 `local_xxx = CONCAT..(...,tag)` 与 `grow_one<...>` push）。

---

## 1. 颜色 ID 写入族（状态 → GPU / 权威）

### 1.1 init_color_ids_buffer — 0x142290750（198B）
- **内部逻辑**：Startup 系统。依次 `AssetSsboLibrary::add_new::<WallColorIdSsbo>()`→`Commands::insert_resource(WallColorIds::new())`→`add_new::<RoofColorIdSsbo>()`→`add_new::<GateColorIdSsbo>()`→`insert_resource(GateColorIds::new())`→`add_new::<HalftimberColorIdSsbo>()`→`insert_resource(HalftimberColorIds::new())`。无事件读取。O
- **算法**：资源/SSBO 初始化，无算法。
- **结构/常量**：见 §0.1/§0.2。候选签名 `void(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, Commands)`（O，PDB）。
- **未知 U**：`WallColorIds::new()` 等默认容量；SSBO 初值。

### 1.2 set_wall_color_id — 0x142290820（1817B）
- **内部逻辑**：`EventReader<SetWallColorIdCmd>` → 计算可读事件数 → `FrozenMap::get` 取 `WallColorIdSsbo`（memcmp 50 字节，不符则 `assert_failed`）。主循环（事件 stride 0x18）逐事件：`idx = 事件实体索引`；`WallColorIds[idx] = 事件+0x10 的颜色 id`（`*(data+idx*4)=uVar12`）；写 tick；随后 `GraphicsDevice::update_buffer_bytes` 上传整段（长度 = `len<<2`，与 SSBO 容量比较，失败 `unwrap_failed`）。若事件 `+0x14==1`（halftimber 墙）：读墙几何（`PublicWalls::get()`，取墙的装饰器折线 `+0x40`/`+0x48`，每点 3×u32）用 SIMD `minps/maxps`（含 NaN 掩码，即 glam `Vec2::min/max`）算 AABB → `Aabb2::center()` + `PublicWallState::max_y()` + `swizzlers::xvy()` 得世界坐标；push 一条命令（stride 0x20，tag=6），并在 `+0x10` 位置写 AABB 中心等。
- **算法**：颜色 id 按墙索引写权威 Vec + 整体上传 SSBO；halftimber 墙的装饰器 AABB 中心重算（SIMD min/max + swizzle xvy）。O
- **常量/结构**：事件 stride 0x18；颜色 id 在事件 +0x10；halftimber 标志 +0x14；墙折线点 stride 3×u32；命令 tag=6；候选签名（O，PDB）`void(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, EventReader<SetWallColorIdCmd>, EventWriter<PlayWallUiSound>, ResMut<WallColorIds>, Res<PublicWalls>, EventWriter<CreatePlasterCmd>, EventWriter<DeletePlasterCmd>)`。
- **未知 U**：`PlayWallUiSound`/`CreatePlasterCmd`/`DeletePlasterCmd` 的触发细节（被内联/交错，未逐条定位）；`0x4fff` 上界常量含义；AABB 折线在墙结构中的字段名。

### 1.3 write_roof_color_id_to_gpu — 0x142298520（999B）
- **内部逻辑**：取 `RoofColorIdSsbo`（memcmp 55 字节）。两路查询：一路元素 stride **0x40**（候选 `EventReader<SetRoofColorIdCmd>` 或屋顶组件），一路 stride **0x18**（`EventReader<CreateRoofCmd>`）。逐元素：`idx*4+4` 写入颜色 id → `GraphicsDevice::update_buffer_bytes`（用 `gpu_write_element`/`gpu_write_with_offset` 内联）上传。
- **算法**：屋顶颜色 id 逐元素写入 GPU SSBO（增量上传），无几何。
- **常量/结构**：SSBO 名 55 字节；两查询 stride 0x40 / 0x18；候选签名（O，PDB）`void(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, EventReader<SetRoofColorIdCmd>, EventReader<CreateRoofCmd>)`。
- **未知 U**：stride 0x40 查询的确切组件/事件名；`+4` 偏移语义（是否含 header）。

### 1.4 set_decorator_color_id — 0x1422af0c0（2879B）
- **内部逻辑**：5 层嵌套事件/查询循环（in_RDX/in_R8/in_R9/in_stack_28/in_stack_30，事件均 stride 0x18）。逐装饰器：`DecoratorStorage::lookup_decorator_info_mut()` → 依 `DecoratorExtraInfo+0x46` 子类型分派：
  - `'?'(0x3f)` = 楼梯：`as_stair()`；再对 `StairsState::get_all_connected_nodes()` 的每个连接节点（stride 4）写颜色（不同循环写 +0xc/+0x10/+8 不同字段）。
  - `'<'(0x3c)` = 灯笼：`as_lantern()` 写 +0xc。
  - 默认：写 `+0x28`。
  然后若事件 `+0x10==1`：`CachedDecoratorTransforms::position()`；按子类型与 (子类型-0x38) 选动画 tag（默认 10、子类型==4 时 0xd、否则 8）；push 命令（stride 0x20）。O
- **算法**：装饰器颜色 id 写入 `DecoratorExtraInfo`（按子类型选字段）+ 楼梯连接节点级联；命令分发。
- **常量/结构**：子类型 tag `'?'=0x3f` 楼梯、`'<'=0x3c` 灯笼；子类型基 0x38；命令 tag ∈{8,10,0xd,0xe}；候选签名（O，PDB）含 SetDecoratorColorIdCmd/SetDecoratorBrightnessCmd/SetLadderColorIdCmd/SetStairRailingCmd/SetMagicColorIdCmd 对应写入。
- **未知 U**：5 层循环分别对应的资源（墙体/装饰器/楼梯多种来源）；`DecoratorExtraInfo` 各写字段（+0xc/+0x10/+0x28/+8）的字段名；`0xcf` 位掩码语义。

### 1.5 set_gate_color_id — 0x142299840（1311B）
- **内部逻辑**：取 `GateColorIdSsbo`（memcmp 50 字节）。事件循环 stride 0x18：`GateColorIds[idx] = 事件+0x10`；写 tick；上传 SSBO（`len<<2`）。事件 `+0x14==1` 时同 §1.2 算墙 AABB 中心 + max_y + xvy，push 命令 tag=**7**。O
- **算法**：同 set_wall_color_id，针对门（tag 7）。
- **常量/结构**：SSBO 名 50 字节；命令 tag=7。
- **未知 U**：与 wall 版差异仅 SSBO 名与 tag（是否还有门专属几何未观测到）。

### 1.6 set_halftimber_color_id — 0x142299250（1296B）
- **内部逻辑**：取 `HalftimberColorIdSsbo`（memcmp 56 字节）。事件 stride 0x18：`HalftimberColorIds[idx]=事件+0x10`；写 tick；上传。事件 `+0x14==1 && PublicWalls::get()!=0` 时算 AABB 中心 + max_y + xvy，push 命令 tag=**0xc=12**。O
- **算法/结构**：同 set_gate_color_id 结构；SSBO 名 56 字节；tag=12。
- **未知 U**：同 1.5。

### 1.7 set_flag_connection_and_chimney_smoke — 0x142298970（929B）
- **内部逻辑**：两阶段。阶段一（事件 stride **0x10**）：`DecoratorStorage::lookup_decorator_info_mut()` + `DecoratorExtraInfo::as_flag()`，写 flag 连接值（事件 +0xc）；若事件 `+0xd==1`：`CachedDecoratorTransforms::position()` 并 push 命令 tag=**9**。阶段二（另一事件集 stride 0x10）：`as_chimney()` 写烟雾值（`+4`）；若 `+0xd==1` push 命令 tag=**0xf=15**。O
- **算法**：旗帜连接颜色与烟囱烟雾开关，各自写 DecoratorExtraInfo 对应字段 + 命令分发。
- **常量/结构**：事件 stride 0x10；flag 值 +0xc、使能 +0xd；tag 9(flag)/15(chimney)；候选签名（O，PDB）`void(EventReader<SetFlagConnectionCmd>, EventWriter<PlayWallUiSound>, ResMut<DecoratorStorage>, EventReader<SetChimneySmokeCmd>, ...)`。
- **未知 U**：`as_flag`/`as_chimney` 返回结构偏移（+4 等）的字段名。

### 1.8 set_window_light — 0x142298260（582B）
- **内部逻辑**：事件循环 stride 0x18。逐事件：写窗口灯值（事件 +0xc）到 `DecoratorStorage::lookup_decorator_info_mut()` 经 `DecoratorExtraInfo::as_window()` 得到的字段（`*extraout_RAX_00 = uVar4`）；若事件 `+0x10==1`：读 `DecoratorExtraInfo+0x46`（子类型），算动画 tag `uVar21`（默认 10；`0xcf>>sub & 1 == 0` 且 sub==4 时 0xd，否则 8）；`CachedDecoratorTransforms::position()`；push 命令（tag=uVar21，含窗口颜色 +0x28）。O
- **算法**：窗光颜色/状态写入 + 动画命令分发。
- **常量/结构**：事件 stride 0x18；窗值 +0xc、使能 +0x10；tag 集合 {8,10,0xd}；子类型偏移 0x46，基 0x38；候选签名（O，PDB）`void(EventReader<SetWindowLightCmd>, EventWriter<PlayWallUiSound>, ResMut<DecoratorStorage>, ...)`。
- **未知 U**：`as_window()` 字段名；`0xcf` 位掩码语义。

### 1.9 set_roof_customization — 0x142299dd0（2367B）
- **内部逻辑**：4 层嵌套循环（in_RCX/in_RDX/in_stack_28/in_stack_30，事件 stride 0x18；另 in_stack_38/40）。逐项：`PublicWalls::roof_entity()` → 在 PublicWalls 表中查墙（元素 stride 0x14：`+0x0c` 分块、`+0x10` 索引、`+0x04` 稀疏位），位集测试命中 → 经分块表定位 roof 元素（stride 0x58）：
  - 比较 `roof+0x3d`(样式) / `roof+0x40`(flat rim) / `roof+0x41` / `roof+0x44` / `roof+0x48`(颜色 id) 与事件数据（事件 +0x10/+0x11/+0x12/+0x14/+0x18）；不同则写回并 push 命令 tag=**0xb=11**；`+0x1c==1` 时 `Roof::pivot_3d()` 后 push。
  - 另一分支写 `roof+0x50` = 事件 +0x10 的颜色 id。
  - 事件 `+0x12==1` 时构造历史条目（4 字节 tag，`History::add()`）——回放历史。
- **算法**：屋顶颜色/样式写入 roof 组件 + 变更检测 + 历史记录；用稀疏位集定位墙→roof。
- **常量/结构**：事件 stride 0x18；PublicWalls 元素 stride 0x14；roof stride 0x58；命令 tag=11；候选签名（O，PDB）`void(EventReader<SetRoofColorIdCmd>, EventWriter<PlayWallUiSound>, ResMut<...>, Res<PublicWalls>, Query<&Roof>, ...)`。
- **未知 U**：事件字段 +0x10..+0x1c 命名（样式/颜色/rim/历史标志）；roof `+0x3d/+0x40/+0x41/+0x44/+0x48` 字段名；`~idx*0x30` 分块公式语义（roof 模块已知同型公式）。

---

## 2. 菜单 UI

### 2.1 color_menu — 0x142291230（20492B）
- **内部逻辑（部分未知）**：本模块最大的系统，逐图标交互 → 命令分发的中枢。Ghidra 反编译失败（返回空）；radare2 原始反汇编确证其为大型函数（序言压 8 个通用寄存器、`sub rsp,0x848`、保存 xmm6–xmm15）。**内部逐行逻辑 = unknown**。
- **签名（O，PDB 候选接口）**：`void(Query<(&mut CustomizationIcon, Option<&ColorIcon>, Option<&WallStyleIcon>, Option<&FlagConnectionIcon>, Option<&FreehandHeightTypeToggleIcon>, Option<&FlatRoofRimTypeToggle>, Option<&ColorMenuBrightnessIcon>, Option<&ChimneySmokeToggle>, Option<&FlatRoofStyleIcon>, Option<&PlasterPeelIcon>, Option<&StairRailingIcon>, Option<&MagicColorIcon>, &Ui2dExtent, &mut Ui2dImage)>, Query<(&Entity,&mut ColorRadialMenu)>, ResMut<GladeColorCustomization>, ResMut<UiNavState>, Local<Option<(usize,usize)>>, (Res<AppMode>, Res<CursorPositionSS>, Res<GameConfig>, Res<WallColorIds>, Res<HalftimberColorIds>, Res<PublicWalls>, Query<&Roof>, Res<ActionSetStates>), (ResMut<History>, Res<UiState>), (EventWriter<SetWallColorIdCmd>, ..., EventWriter<SetMagicColorIdCmd>), EventWriter<SetChimneySmokeCmd>, EventWriter<PlayUiSound>, EventWriter<SetFlatRoofStyleCmd>, EventWriter<SetPlasterPeelAmountCmd>, EventWriter<SetLadderColorIdCmd>, EventWriter<SetStairRailingCmd>, EventWriter<SetMagicColorIdCmd>)`。
- **作用（I，由签名推断）**：遍历径向菜单图标实体，读取当前颜色/样式状态（WallColorIds/HalftimberColorIds/PublicWalls/Roof）与输入（ActionSetStates、光标），在选中/点击时写出对应 Set*Cmd（墙/屋顶/门/半木/装饰器/亮度/旗/窗光/rim 样式/削皮/梯/栏杆/魔法色）并操作历史。
- **未知 U**：逐图标分支判定与命令组装、`Local<Option<(usize,usize)>>` 用途、`GladeColorCustomization`/`UiNavState` 更新、`0x848` 栈帧内各临时量。

### 2.2 color_button_animations — 0x1422979b0（1577B）
- **内部逻辑**：`Query<(&mut CustomizationIcon, &mut Ui2dExtent, &mut UiDrawOrder, Option<&CustomizationIconMarkedForDeletion>)>`。每图标：动画进度字段（CustomizationIcon `+0x18`，即 `puVar1[3]`）按 dt 推进——删除态 `fVar17 = dt/-0.15`，否则 `dt*4.0`；累加并 clamp 到 1.0；`utils::easing_functions::EasingFunction::apply()` 后按进度缩放 `Ui2dExtent`。非删除态再做二次缓动：缩放混合 `(1-e)*0.8+e`，并对旋转/偏移字段 `+0x1c` 按 `±0.15` 推进 clamp[0,1]、缓动后整体放大 `eased*0.2+1.0`。O
- **算法**：UI 图标出现/消失缓动（scale + 位移），删除时反向收缩。
- **常量**：`-0.15`、`4.0`、`0.8`、`0.15`、`0.2`、clamp 1.0。
- **未知 U**：`EasingFunction` 具体曲线；`CustomizationIcon` 各字段名；两段缓动对应的视觉量。

### 2.3 delete_color_icon_entities — 0x142297ff0（611B）
- **内部逻辑**：查询 CustomizationIcon（两分支：有/无 `CustomizationIconMarkedForDeletion`）。逐图标取动画进度 `*(float*)(field + 0x18 + icon_index*0x24) <= 0.0` 时 `EntityCommands::despawn()`。O
- **算法**：图标动画结束（进度≤0）后销毁实体。
- **结构**：CustomizationIcon 动画进度 +0x18，步长 0x24；图标目录 stride 0xa0。
- **未知 U**：`0x24` 步长元素类型。

### 2.4 show_color_options — 0x14229a7f0（687B）
- **内部逻辑（部分未知）**：Ghidra 反编译返回空；ReVa 超时；radare2 无符号。**内部逐行 = unknown**。功能由签名推断：读 `ShowColorMenuCmd`，按命令携带的类别/索引，从 WallColorIds/GateColorIds/HalftimberColorIds/Roof/DecoratorStorage/PublicWalls 取当前状态，构造图标列表并调用 `ColorRadialMenu::create`（Commands 生成径向菜单实体）。
- **签名（O，PDB 候选接口）**：`void(EventReader<ShowColorMenuCmd>, Res<WallColorIds>, Res<GateColorIds>, Res<HalftimberColorIds>, Query<&Roof>, Res<DecoratorStorage>, Res<PublicWalls>, Query<&Roof>, Res<GameConfig>, Res<MainCamera>, ResMut<UiNavState>, DerivedDecoratorInfoResources, Res<StairsVisualState>, Res<StairsState>, Commands)`。
- **未知 U**：图标列表按类别分支的构造、菜单定位、closure$1..$21 各自职责。

### 2.5 hide_color_menu — 0x1422adcf0（2408B）
- **内部逻辑**：读 `HideColorMenuCmd`；若无命令则 `DigitalActionState::just_pressed()` 判定（`cVar64`）。命中后：查询所有 CustomizationIcon（两分支），`insert(CustomizationIconMarkedForDeletion)`；并按图标子类型（`lVar56+0x30`）`switch`，向约 5 个事件写者（in_stack_38/40/48、in_RDX）写出各自的「重置/隐藏」命令（stride 0x18/0x20/0x10 不同布局），最后 `EntityCommands::despawn()` 销毁图标实体。O
- **签名（O，PDB 候选接口）**：`void(EventReader<HideColorMenuCmd>, EventWriter<PlayUiSound>, Query<(&Entity,&ColorRadialMenu)>, Query<(&Entity,&CustomizationIcon)>, Res<ActionSetStates>, Commands, EventWriter<SetFlatRoofStyleCmd>, (EventWriter<SetWallColorIdCmd>, SetRoofColorIdCmd, SetDecoratorColorIdCmd, SetLadderColorIdCmd, SetDecoratorBrightnessCmd, SetGateColorIdCmd, SetHalftimberColorIdCmd, SetWallStyleCmd, SetFlagConnectionCmd, SetWindowLightCmd, SetFlatRoofRimStyleCmd, SetChimneySmokeCmd, SetMagicColorIdCmd), EventWriter<SetPlasterPeelAmountCmd>)`。
- **算法**：隐藏菜单 = 标记+销毁全部图标实体，并对每类图标写出收尾/复位命令。
- **未知 U**：每 case（0..0xc）写出的具体命令与目标写者映射；`just_pressed` 的输入动作名。

### 2.6 autoclose_color_menu — 0x142298d30（1207B）
- **内部逻辑**：读两个事件源 + 一个 CustomizationIcon 查询 + 一个命令写者。若任一事件 `*piVar8 == 0xc`（kind 12）→ 直接进入关闭路径；否则遍历图标：取图标世界位置（图标组件 +0x50 经装饰器目录 stride 0xa0 定位），算与光标（in_R9）的 2D 距离，若 `size.y*2.0 + size.x*1.2 < dist` 跳过；再与 `MainCamera::eye_position()` 的 3D 距离比较，`> 10.0` 则关闭。关闭 = 向命令写者写 1 字节 bool。O
- **算法**：菜单自动关闭——点击远处（kind 12）或所有图标离摄像机 >10 单位时发关闭事件。
- **常量**：kind 判别 0xc；距离阈值 10.0；尺寸系数 2.0/1.2。
- **未知 U**：`kind 12` 事件语义；两个事件源类型；命令写者类型。

### 2.7 ColorRadialMenu::create — 0x1425f27c0（1947B）
- **内部逻辑**：输入 `Vec<IconInfo>`（stride 0x28）。先统计非分隔符图标数（`tag!=5` 计数）。算角步长：`step = (start+span)*(count-1)/denom`，`AngleRange::from_min_and_len()`（`utils::angle_range`）。分配实体 id Vec。遍历每个 IconInfo（stride 0x28）：读字段 `+0x18`(tag)、`+0x19`、`+0x1a`、`+0x1c`(f32)、`+0x20`、`+0x24`；按 tag-3 分派：
  - 用 `cosf/sinf` 算径向位置（角度 `fVar26`，半径偏移 `size*0.75`/`size*0.25`，角步 `+0.7853982`=π/4）；`Commands::spawn<tuple<Ui2dExtent,Ui2dImage,CustomizationIcon,UiDrawOrder>>`；再按 tag 插入对应图标组件（tag 3→ColorIcon、4→WallStyleIcon、5→ColorWheelSeparator、6→FlagConnectionIcon、7→FreehandHeightTypeToggleIcon、8→FlatRoofRimTypeToggle、9→ColorMenuBrightnessIcon、10→ChimneySmokeToggle、11→FlatRoofStyleIcon、12→PlasterPeelIcon、13→StairRailingIcon、14→MagicColorIcon）。
  - 分隔符（tag 5）缩放系数 0.3，其余 1.0（`__real_3e99999a3f800000` 选择表）。
  - 选中态判定：与当前值比较（子分支 case 6/8/9/0xa/0xb 比对亮度/样式/颜色等），命中则记录。
  - 每图标 push 实体 id；最后 `Commands::spawn<ColorRadialMenu>()` 携带图标/分隔符 id 列表。
- **算法**：**径向菜单布局**——N 个图标在角范围内均匀分布，极坐标 `(cos,sin)*radius` 定位；`utils::angle_range::AngleRange::from_min_and_len`；图标类型分派。
- **常量**：IconInfo stride 0x28；角步 π/4=0.7853982；半径偏移 0.75/0.25；缩放 0.3(分隔符)/1.0；tag 表 3..14。
- **未知 U**：`cosf/sinf` 角度基准与 `AngleRange` 输入来源；`__real_3e99999a3f800000` 常量表语义；选中态比较字段含义。

### 2.8 IconInfo::new_separator / new_flat_roof_rim_ty
见 §0.3。均为纯构造器，无算法。

---

## 3. UI 颜色编辑（egui 控件构建）

### 3.1 ui_color_wall_and_roof — 0x141c9ab70（580B）
- **内部逻辑**：`*(int*)*in_RCX == 0xc` 守卫（UI 编辑模式 12）。构造 `WidgetParams`（`clearing_canvas`、`occlude_by_world`）、`UiState::add<&str>("color_wall_...")`、signifier closure_env_0。点击时：`RaycastWorld::raycast()` → 命中类型 `local_1d8`：0/6/7→`bVar20=3`，2→`bStack_1d7&1`，8→`bVar20=2`；据命中在 PublicWalls 定位墙实体（两分支按 `+0x108`）；`format!("...WallId")`。构建 slider 控件（WidgetParams + sounds on_hover_start/on_click + signifier closure_env_2 + `UiState::add<String>`）。`is_active` 时向命令写者写值 `0x3f800000`(1.0) 与 `bVar20>1` 的 bool；点击时算 `cVar21`（`bVar11=clamp(bVar20-2,..)`：0→8、1→0、否则 `(bVar20&1)+1`）写入 stride 0x20 命令。
- **算法**：墙/屋顶颜色 UI 控件 + 射线拾取 + 交互 → 命令。
- **常量**：模式 12；命中类型 0/2/6/7/8；`0x3f800000`=1.0；`cVar21` 映射 {8,0,1/2}。
- **未知 U**：命中类型枚举名；命令目标写者类型；`cVar21` 命令 tag 语义。

### 3.2 color_cursor_icon — 0x141c9b510（580B）
- **内部逻辑**：`SignifierEntityBuilder::new/animation_duration/build`；据 `in_R8B` 选择插入 `UiInactive`；`__rust_alloc` 后 `builtin_strncpy(...,"color_icon/color_cursor",0x17)`，`EntityCommands::insert<tuple<Ui2dExtent,Ui2dImage>>`，再插入 `Ui2DFollowMouse`。O
- **算法**：构建跟随鼠标的「颜色光标」图标实体。
- **常量**：贴图名 `"color_icon/color_cursor"`（长度 0x17=23）。
- **未知 U**：`in_R8B`（是否 inactive）来源。

### 3.3 ui_color_gates — 0x1422afe70（2600B）
- **内部逻辑**：模式 12 守卫。从 `in_R9` 的位集（每 0x20 字节一组 16×16）迭代置位——稀疏墙实体集；逐墙：查 PublicWalls（stride 0x14）→ 取 mesh（`AssetMeshLibrary::get`）→ `Mesh::positions()`（顶点/3=三角形数）→ `fold<Range<usize>, closure_env_0>` 产出 `utils::geometry::intersect::Triangle` 列表（拾取用）。`format!("color_gates_"+WallId)`。构建控件（WidgetParams + `UiState::add<String>`）；`is_active` 时 `SignifierFactory::new`+`UiSignifierStream::write`（含 mesh 顶点数据 + 格式化字符串）；点击时向两写者写命令（其一 tag 字节=**5**，含 WallId/颜色）。O
- **算法**：门颜色 UI——遍历有门几何的墙，三角化 mesh 供拾取，交互 → 命令。
- **常量**：位集步长 0x20；mesh 顶点 stride 3（每三角形）；命令 tag=5。
- **未知 U**：`in_R9` 位集来源；`closure_env_0` 产出三角形细节；signifier 输出格式。

### 3.4 ui_color_gates::closure$2 — 0x1422b0ba0（522B）
- **内部逻辑**：拷贝一个 slice（stride 0xc=12 字节），`SignifierEntityBuilder::new/animation_duration/build`，插入 `DrawTopology`，值 `0x4040000000000001`（两 u32/f32：1.0 与 3.0），返回单元素 Vec。O
- **算法**：门颜色 signifier 的实体构建（拓扑/描边）。
- **常量**：元素 stride 0xc；`0x40400000`=3.0、`0x00000001`=1.0。
- **未知 U**：12 字节元素类型；DrawTopology 参数含义。

### 3.5 ui_color_windows_and_flags — 0x1422b0e00（2168B）
- **内部逻辑**：模式 12 守卫。`RaycastWorld::legacy_raycast_w_params()` → 命中。命中类型 `local_200`：
  - 0xd/0xe：`DecoratorStorage::lookup_decorator_info()` + `lookup_storage()` + `stairs_assembly_params::from()`；用 hash 集（`hash_one<WidgetId>`，16 字节 SIMD 比较）判定楼梯相关标志 `bVar21/bVar19`。
  - 4：`DecoratorStorage::decorator_info()` → 子类型（`+0x46-0x38`）；`StairsState::graph()` → `IndexMap::get<DecoratorId, Vec<(DecoratorId,CompactDirection)>>`；遍历连接节点 `StairSegment::new`/`stairs_assembly_params::from`，判定 `bVar21`。
  构建控件：`format!("color_decorator_"+index)`，WidgetParams（on_hover_start/on_click，音量随 `bVar19`），signifier closure_env_3，`UiState::add<String>`。`is_active` 时：楼梯子类型 `'?'`→`EventWriter::write<OutlineStairsCmd>`，否则 `Transform::compute_affine`+`DecoratorMeshLibrary::get_outline`+`DecoratorOutline::display`。点击时：`write<HideColorMenuCmd>`；按子类型 `switch(lVar18+0x46)`：'8'/'9'/':'→`cVar9=3`、';'→10、'<'→9、'='→6、'?'→`bVar21+0xb`；`write<ShowColorMenuCmd>`（携 `(cVar9, cVar20, index)`）。O
- **算法**：装饰器（窗/旗）颜色 UI——射线拾取 + 楼梯图遍历判定，点击打开对应类别颜色菜单。
- **常量**：命中类型 0xd/0xe/4；子类型 `'8'=0x38,'9',':',';'=0x3b,'<'=0x3c,'='=0x3d,'?'=0x3f`；菜单类别码 {3,6,9,10,11/12}。
- **未知 U**：菜单类别码 cVar9 与 ShowColorMenuCmd 字段映射语义；`bVar19/bVar21` 具体含义；`WidgetId` 哈希集用途。

---

## 4. 回放 / 插件

### 4.1 replay_color — 0x141c7f270（1450B）
- **内部逻辑**：`InputStream::sync_point_count()`；`ReplayBuffer::front_mut_bucket()` 取当前帧桶（元素 stride 0x30）。遍历桶：仅处理 `byte0==6`（DecoratorHistoryEdit）的元素，`_Dst = *(elem+0x10)` 为负载；`sub = clamp(_Dst[0x18]-3, 0..13)`，`switch(sub)`（0..0x17）把负载字段派发到约 14 个事件写者（in_RDX/in_R8/in_R9/in_stack_28..70），分别对应 Set*ColorIdCmd 等颜色事件；部分 case 按 `pcVar63[0x1c]`（undo 方向）选不同偏移。时间推进：若 `elem[8]&1`，`t=elem[0xc]/elapsed`，clamp≥0.1，`elem[0x18] += dt`。收集需移除元素下标，反转后从后往前 `Vec::remove` 并按 tag `drop`（tag<10 分派到 WallReplayCmd/RasterEdit/TerrainEdit/DecoratorHistoryEdit/RoofEdit/TreeEdit，else PhysicalSize mutex）。O
- **算法**：帧回放调度——把颜色类 DecoratorHistoryEdit 负载重放为颜色事件；处理完从桶中移除。与 roof 模块 `replay_roof` 同型。
- **常量**：桶元素 stride 0x30；payload sub = `_Dst[0x18]-3`；时间 clamp 0.1。
- **未知 U**：14 个事件写者 ↔ sub 值逐一映射；`_Dst[0x18]` 真实枚举名；drop 分派 tag 表（部分已知 0..9）。

### 4.2 lib.rs::add_color — 0x140651870（1916B）
- **内部逻辑**：`Components::get_resource_id` 去重后 `EventRegistry::register_event::<T>()` 注册 15 个事件（见 §0.5）；最后一个 `0x8e1bd251962ba784` 为 `GameSystemSet`（`SystemSet`），`Schedules::add_systems<..., Startup, void(*)(Res<GraphicsDeviceResource>, Res<AssetSsboLibrary>, Commands)>(...)` 挂 `init_color_ids_buffer`（`in_set(GameSystemSet)` + `chain_ignore_deferred`），再 `SubApp::add_systems<..., Update, ...>` 挂 Update 系统。O
- **算法**：插件注册（事件 + Startup + Update 系统），无业务算法。
- **未知 U**：Update 挂载系统确切列表与顺序（被内联）；GameSystemSet 子集。

---

## 逻辑总览

颜色状态（权威 → GPU）：
- Startup：`init_color_ids_buffer` 注册 4 SSBO + 3 颜色资源。
- `SetWallColorIdCmd → set_wall_color_id`：WallColorIds[idx]=id，上传 WallColorIdSsbo；halftimber 墙重算装饰器 AABB（tag 6）。
- `SetGateColorIdCmd → set_gate_color_id`：GateColorIds（tag 7）；`SetHalftimberColorIdCmd → set_halftimber_color_id`：HalftimberColorIds（tag 12）。
- `SetRoofColorIdCmd → set_roof_customization`：写 roof+0x3d/0x40/0x48/0x50（tag 11）+ 历史。
- `SetRoofColorIdCmd/CreateRoofCmd → write_roof_color_id_to_gpu`：增量上传 RoofColorIdSsbo。
- `SetDecoratorColorIdCmd → set_decorator_color_id`：写 DecoratorExtraInfo（按子类型/楼梯连接节点）。
- `SetFlagConnectionCmd/SetChimneySmokeCmd → set_flag_connection_and_chimney_smoke`（tag 9/15）；`SetWindowLightCmd → set_window_light`。

菜单 UI：
- 点击世界物 → `ui_color_wall_and_roof`/`ui_color_gates`/`ui_color_windows_and_flags` 发 `ShowColorMenuCmd`。
- `ShowColorMenuCmd → show_color_options` → `ColorRadialMenu::create` 生成径向菜单（图标实体 + 组件）。
- `color_menu`：每帧处理图标交互 → 发各 Set*Cmd；`color_button_animations` 动画、`delete_color_icon_entities` 清理。
- `HideColorMenuCmd → hide_color_menu` 销毁图标；`autoclose_color_menu` 远距离/点击远处自动关闭。

回放：`replay_color` 把 DecoratorHistoryEdit 的颜色负载重放为颜色事件；`add_color` 注册全部事件与系统。

## 算法清单（可复用/已恢复）

| 算法 | 出现函数 | 现成实现 |
|---|---|---|
| SSBO 名 = 类型全名（memcmp 校验长度） | 全部 set_* | 本报告 §0.1 |
| 颜色 id 索引 Vec<u32> 写入 + 整体/增量上传 | set_wall/gate/halftimber, write_roof | core `color_id`；§1.2/1.3 |
| 墙装饰器 AABB（SIMD minps/maxps + NaN 掩码）→ center + max_y + xvy | set_wall/gate/halftimber | utils::geometry::aabb、swizzlers |
| 装饰器子类型分派（0x46 tag）写字段 | set_decorator_color_id, set_window_light, set_flag... | core `walls::decorator` |
| 径向菜单均匀分布（cos/sin + AngleRange） | ColorRadialMenu::create | utils::angle_range（复用） |
| UI 图标出现/消失缓动 | color_button_animations | utils::easing_functions |
| 帧回放调度 + 桶移除 + drop 分派 | replay_color | 与 roof `replay.rs` 同型 |
| 稀疏位集定位墙→roof | set_roof_customization, ui_color_gates | 与 roof 模块同型分块公式 |

## 未知清单（U）

1. `color_menu`(0x142291230, 20492B) 与 `show_color_options`(0x14229a7f0, 687B)：三种反编译器均失败，内部逐行逻辑未恢复（仅签名+作用）。
2. 各 `set_*` 命令 push 的 stride 0x20 元素结构（`grow_one<...>` 泛型名是被复用的其它类型，非真实类型）与 tag 消费者。
3. `SetWallColorIdCmd`/`SetGateColorIdCmd`/`SetHalftimberColorIdCmd` 事件字段命名（+0x10 颜色、+0x14 标志、+8 实体）与 `0x4fff` 上界。
4. `set_roof_customization` 事件字段 +0x10..+0x1c 命名、roof `+0x3d/+0x40/+0x41/+0x44/+0x48/+0x50` 字段名、`~idx*0x30` 分块公式语义。
5. `set_decorator_color_id` 5 层循环对应资源、`DecoratorExtraInfo` 各写字段名、`0xcf` 位掩码。
6. `set_window_light`/`set_flag...` 的 `as_window/as_flag/as_chimney` 字段名与动画 tag 集合。
7. `color_button_animations` 的 `EasingFunction` 曲线、各字段视觉语义；`CustomizationIcon` 完整布局（stride 0xa0）。
8. `ColorRadialMenu::create` 角度基准/AngleRange 输入、`0.3/1.0` 选择表、选中态比较字段。
9. `hide_color_menu` 每 case 命令/写者映射；`autoclose_color_menu` kind 12 事件与命令写者类型。
10. `ui_color_*` 命中类型枚举名、命令写者类型、菜单类别码 `cVar9` 语义。
11. `replay_color` 14 个事件写者 ↔ sub 值映射、`_Dst[0x18]` 枚举名。
12. `add_color` Update 系统列表/顺序；`GameSystemSet` 子集。

## 依赖（直接复用，不逆向）
std/alloc、glam 0.29.3、bevy_ecs 0.16、egui 0.27.2、utils（geometry/aabb/swizzlers/easing_functions/angle_range）、rhapsody（GraphicsDevice::update_buffer_bytes、raw::bytes）、country_core（color_id/color_id_roof/walls::decorator/render::ssbo/asset_libraries::ssbo_library）。

## 证据路径
- 本报告：`upper-modules-plan/systems/color/ANALYSIS.md`
- 反编译原文：`upper-modules-plan/systems/color/jobs/decomp/*.c`
- job/result：`jobs/batch{1..4}.json`、`jobs/result{1..4}.json`、`jobs/reva1.json`、`jobs/revaresult1.json`、`jobs/radare{1..5}.json`、`jobs/radareresult{1..5}.json`、`jobs/extract.py`
- 骨架：`skeleton/B/crates/systems/color/src/*.rs`（含 PDB 候选接口）
