# system-color（crates/systems/color）审核报告

- Reviewer：**reviewer-state-color**（state 组 reviewer）— 2026-10-17
- 对象：restorer-state-color 对 `crates/systems/color`（package `system-color`）的重建
- 方法：全函数 **独立 fresh 反编译**（Ghidra `get_functions` fields=signature,decompiled_code，
  worker=shared；本审核 3 路作业核对）→ 独立复述原控制流 → 与落盘 Rust body 逐语句对照。
- 证据核对：restorer 的 `logic/decomp/fresh*.c` 与 `logic/results/fresh_batch{1..3}.json`
  原始返回**逐字一致**（归一化比较 True）；本审核另发 2 批 fresh 查询（new_separator/
  new_flat_roof_rim_ty/0x1400885d0/init_color_ids_buffer/set_window_light/
  set_flag_connection_and_chimney_smoke/register_required_components/show_color_options），
  与 restorer 证据可复现一致。
- 编译复核：`cargo check -p system-color` 本审核实测 **通过**（Finished dev profile；country-core
  告警与他组并发编辑无关）。

## 总表（独立 verdict）

| 函数（地址） | 原逻辑复述结论 | 与 Rust 对照 | verdict |
|---|---|---|---|
| `IconInfo::new_separator`（0x1425f2680） | 构造 `{name:"white", kind=5, +0x24=0}`；其余字段 Default 零 | Rust `{name:"white",kind:5,..default}` 等价 | **判等** |
| `IconInfo::new_flat_roof_rim_ty`（0x1425f26f0） | `ClutterVersion::is_initial()` true→`"color_icon/freehand_ty_terrain"`(30B)，false→`"color_icon/freehand_ty_constant"`(31B)；kind=8，+0x24=0。串字节手工校验（"color_icon/free"+"ehand_ty"/"hand_ty_"+"_terrain"/"_constant"）与 Rust 字面量逐字一致 | 等价；is_initial 输入源在反编译丢失，Rust 用 `ClutterVersion::default()`（恒 0→true），登记 U | **判等**（输入源 U 登记） |
| `init_color_ids_buffer`（0x142290750） | 4×`add_new<SSBO>`+3×insert_resource，序：Wall→Roof→insert(Wall)→Gate→insert(Gate)→Halftimber→insert(Halftimber) | insert_resource×3 顺序忠实；**SSBO add_new×4 未落地**（core `Wall/Gate/HalftimberColorIdSsbo`=typed-gap、无 SsboType，实测确认） | 未实现-需重做（SSBO 段阻塞）；insert 面判等 |
| `set_wall_color_id`（0x142290820） | 事件循环 stride 0x18：idx 断言（>0x4fff panic）→ 读旧色→`WallColorIds[idx]=新色`（**无条件**）；`PublicWalls::get` 命中才：custom!=0→is_halftimbered(返回不参与分支,U)+折线 AABB(minps/maxps NaN 掩码)→center+max_y+xvy→推 0x20B 记录 tag=6；石膏：新色∈{4..10,12}→Create，否则旧色∈该集→Delete；SSBO get+memcmp 50B+update_buffer_bytes 上传 | Rust 逐句等价（含石膏分类 `! (6<new-4U && new!=0xc)` 与 `old∈set&&new!=0xc` 的 unsigned 判等，已核）；SSBO 段 blocked；记录 selector/value=0（U 登记） | 未实现-需重做（SSBO 段阻塞）；CPU 面判等 |
| `set_gate_color_id`（0x142299840） | 同 wall 族；**墙命中才写色**（do-while 以 wall!=0 退出才处理当前事件）；custom==1→AABB→tag=7；无石膏分类 | Rust `if let Some(wall)` 门控写色+tag7 等价；无石膏等价；custom `==1`（FIX-5 已改，与反编译 `=='\x01'` 一致） | 未实现-需重做（SSBO 段阻塞）；CPU 面判等 |
| `set_halftimber_color_id`（0x142299250） | **逐事件无条件写色**；custom==1 **且** wall 命中→AABB→tag=0xc；无石膏 | Rust 无条件写色+`custom==1&&wall` 等价（FIX-5）；custom 与反编译一致 | 未实现-需重做（SSBO 段阻塞）；CPU 面判等 |
| `write_roof_color_id_to_gpu`（0x142298520） | FrozenMap::get(memcmp 55B)→双事件源(0x40/0x18)逐元素 `update_buffer_bytes(4B,idx<<2)` 增量上传 | Rust 仅签名面+空循环+GAP 登记 | 未实现（GPU 段阻塞，诚实） |
| `set_decorator_color_id`（0x1422af0c0） | 5 层事件源（0x18/0x18/0x18/0x10/0x18）**顺序处理**：L1=SetDecoratorColorIdCmd 写 +0x28、sub==7('?')→逐连接节点写 +0x28、tag{8,10,0xd}；L2=亮度 '<'→as_lantern +0x0、tag 0xe；L3=梯子 '?'→节点 +8、tag10；L4=扶手(0x10) '?'→节点 +0x10、tag10；L5=魔法 '?'→节点 +0xc、tag10；每层 lookup 判空方式（L1 unwrap、L2-5 if-let）各异 | 5 层写面/判空/tag 全部等价（含 +8/+0xc/+0x10 字节写）；记录 value 非 sub==4 分支为反编译栈值 U（Rust 取事件色，登记假设）；custom 判别 FIX-5 已对齐（L1 `!=0`、L2-5 `==1`） | 未实现-需重做（记录 value U 面未完全还原）；分派/写入面判等 |
| `set_flag_connection_and_chimney_smoke`（0x142298970） | 双事件源 stride 0x10：旗 `as_flag()+0=event+0xc`、custom!=0→tag9；烟 `as_chimney()+4=event+0xc`、custom==1→tag0xf | flag 面等价；**烟值写 +4 未落地**（core `ChimneyExtra{pub f32}`=4B 未建模 +4，实测确认）；custom FIX-5 已对齐（flag `!=0`、烟 `==1`） | 未实现-需重做（chimney +4 阻塞）；flag 面判等 |
| `set_window_light`（0x142298260） | 逐事件 `as_window()+0=event+0xc`（f32 位模式）；custom==1→sub=type-0x38(>7→6)、tag 默认10、`0xcf>>sub&1==0` 时 sub==4→0xd 否则 8→推 tag 记录 value=info+0x28 | tag 计算/写面/记录 value 全部等价；custom `==1`（FIX-5 已对齐） | 未实现-需重做（无阻塞；整体判等） |
| `set_roof_customization`（0x142299dd0） | 4 层嵌套（0x18/0x20/0x18/0x18）稀疏位集/分块表（stride 0x14/0xa0/0x48/0x58、`~idx*0x30`）定位 roof；外环 SetRoofColorIdCmd：`roof+0x50=色`+第二写（色表）+custom→pivot_3d→tag 0xb；内环样式比较+History::add(0x50B {1,1,4,..}) | Rust 仅外环近似（`roof_entity`+`roof+0x50`+pivot_3d+tag0xb）；**遗漏第二写**；密集表内环 blocked（core PublicWalls=HashMap） | 未实现（密集表阻塞）；外环近似（遗漏第二写，登记） |
| `color_menu`（0x142291230） | 三路反编译失败（Ghidra 空/ReVa 超时/radare2 无符号），内部逐行 unknown | Rust 读取面 stub+PDB 候选接口登记 | 未实现（反编译失败，诚实） |
| `color_button_animations`（0x1422979b0） | 数学已解出：progress(+0x18+idx*0x24) 删除态 dt/-0.15 否则 dt*4、clamp1.0、EasingFunction、二次缓动 0.8/0.2 | Rust 空循环+GAP | 未实现（typed-gap，诚实） |
| `delete_color_icon_entities`（0x142297ff0） | 两目录分支遍历图标，**仅 `progress(+0x18+idx*0x24)<=0` 才 despawn** | Rust 已改：循环体留空（FIX-1 fix_verified），不再无条件 despawn；progress 门控 typed-gap GAP | 未实现（typed-gap；原分歧已修复） |
| `show_color_options`（0x14229a7f0） | 三路反编译失败，内部 unknown | Rust 空循环+候选接口登记 | 未实现（反编译失败，诚实） |
| `hide_color_menu`（0x1422adcf0） | 读 HideColorMenuCmd；无则 `DigitalActionState::just_pressed()`；**命中才**标记+逐子类型复位命令+despawn | Rust 已改：按 `HideColorMenuCmd` 事件存在门控 despawn（FIX-2 fix_verified）；just_pressed 动作名 U 登记 | 未实现（typed-gap；原分歧已修复） |
| `autoclose_color_menu`（0x142298d30） | 双事件源 stride 0x28 任一 kind==0xc→直闭；图标世界位置（目录 stride 0xa0/0x48+`~idx*0x30`）算光标 2D 距离 `size.y*2+size.x*1.2<dist`→跳过；与 eye_position 3D 距离>10→关闭 | Rust 用 `menu.center` 近似图标位置、省略光标跳过、逐 menu 判 3D 距离→写 Hide | 未实现（事件源/目录 typed-gap；eye 闭合面近似，登记） |
| `ColorRadialMenu::create`（0x1425f27c0） | 非分隔符计数、`AngleRange::from_min_and_len`、cos/sin 极坐标、按 tag-3 分派 spawn | Rust 仅数据组装面（center/radius/icons+死计数），spawn 面 GAP | 未实现（UI spawn typed-gap，诚实） |
| `ui_color_wall_and_roof`（0x141c9ab70） | 模式 12 守卫→raycast(mask 0xcb)→命中类型 0/2/6/7/8→控件+命令 | Rust 仅模式守卫 | 未实现（widget/raycast 面未对齐，诚实） |
| `color_cursor_icon`（0x141c9b510） | SignifierEntityBuilder+"color_icon/color_cursor"+Ui2DFollowMouse | Rust 空体 | 未实现（SignifierBuilder 面未对齐，诚实） |
| `ui_color_gates`（0x1422afe70） | 模式 12 守卫→稀疏墙位集→mesh 三角形→控件 tag5 | Rust 仅模式守卫 | 未实现（诚实） |
| `ui_color_windows_and_flags`（0x1422b0e00） | 模式 12 守卫→legacy_raycast→命中 0xd/0xe/4→楼梯图遍历→ShowColorMenuCmd 类别码 | Rust 仅模式守卫 | 未实现（诚实） |
| `replay_color`（0x141c7f270） | sync_point_count+front_mut_bucket，桶元素 stride 0x30，byte0==6 才处理；sub=clamp(payload[0x18]-3,0..13) switch 13 写者；时间推进/移除/drop 分派 | Rust 仅 front_mut_bucket 空循环+13 写者登记 | 未实现（ReplayBuffer 原始布局未建模，诚实） |
| `add_color`（0x140651870） | 15×`register_event`（各带 get_resource_id 幂等门）序：Show/Hide/SetWall/SetRoof/SetDecorator/SetBrightness/SetGate/SetHalftimber/SetFlag/SetWindow/SetChimney/SetFlatRoofStyle/SetLadder/SetRailing/SetMagic；GameSystemSet+Startup(init_color_ids_buffer, chain_ignore_deferred)+SubApp::add_systems<Update>(2 system configs) | Rust `register_color_events` 用 `init_resource::<Events<T>>`（**非 register_event**）且 **init 顺序与反编译 register 顺序不一致**，另追加 6 个本模块系统写出的他域事件；Startup-in_set-GameSystemSet-chain_ignore_deferred 与 Update 两批被折叠进单一 schedule 参数（water crate 同约定）。**FIX-4 已纠正表述**：措辞为"事件集合一致 15、注册面按约定落地、未逐事件对齐" | 胶水-非业务（注册面，workspace 约定；表述已修正） |
| `register_required_components`（0x1425f3d60） | 含 `enforce_no_required_components_recursion`+类型名哈希（0x7f 逐字节）——bevy required-components 递归注册 | Rust `world.register_component::<CustomizationIcon>()` | 胶水-非业务（glue-legitimate，确认） |

## 独立复述核对要点（关键函数）

### set_wall_color_id 石膏分类（unsigned 语义核对）
反编译 `if ((6 < iVar13-4U) && (iVar13 != 0xc)) goto 删除检查`：new-4U 在 new<4 时无符号下溢→大值→直接进删除检查，故 Create 仅在 new∈{4..10,12}；删除检查 `(old-4U<7 || old==0xc) && new!=0xc`（new∉set 已隐含 new!=0xc，等价 Rust else-if `contains(&old)`）。Rust `CUSTOM_COLOR_SET=[4..10,12]` 判定与机器码等价。

### 事件布局（跨函数一致性）
消费面 stride 全核对：`{id u32,color u32,custom u8}`+EventInstance 8B = 0x18；`{id u32,value u8,custom u8}` = 0x10。set_decorator 5 层依次 0x18/0x18/0x18/0x10/0x18、set_roof_customization 4 层 0x18/0x20/0x18/0x18（RESTORE 写 0x20/0x20 的第 2/3 层有 1 处不一致：实测第 3 层为 0x18 非 0x20——登记微差，不影响未实现结论）。0x1400885d0 实测为 **4B 恒等函数**（`{return;}`），`cmd.id as usize` 直索正确。

### 0x20B 记录（PlayWallUiSound）
kind@+8、selector@+9、value@+0xc、position@+0x10 与 core `PlayWallUiSound` 布局一致；set_window_light 的 value=DecoratorInfo+0x28 已正确接入；wall/gate/halftimber 的 selector/value 反编译为未解出字节（Rust 置 0，U 登记，不作判等）。

## FIX 清单（原始提出项，全部已被 restorer 处理，见上"FIX 复核状态"）

1. **`delete_color_icon_entities`**（高危）：Rust 无条件 despawn 全部图标；原仅 `progress<=0`。→ 已留空循环体（FIX-1）。
2. **`hide_color_menu`**（高危）：Rust 无条件 despawn 全部菜单+图标；原按 HideColorMenuCmd/`just_pressed` 命中才动。→ 已按事件存在门控（FIX-2）。
3. **`autoclose_color_menu`**：eye 距离闭合面用 `menu.center` 近似（原=图标世界位置+光标跳过），忽略 kind==0xc 直闭。→ 已注明 stub 非判等（FIX-3）。
4. **`add_color` 表述**：RESTORE.md "15 事件注册顺序…忠实"不准确。→ 已改"事件集合一致（15）、注册面按 workspace 约定 init_resource 胶水落地、未逐事件对齐"（FIX-4）。注：`replay_color` 的 id 派发依赖 register_event 的真实注册（未实现）。
5. **custom 判别微差**：gate/halftimber/decorator-L2~L5/window/chimney/set_roof 反编译 `==0x01`，wall/decorator-L1/flag 为 `!=0`。→ 已逐函数对齐（FIX-5）。
6. **set_roof_customization 外环第二写**：遗漏 `*(色表+roof_index*4)=uVar4`（uVar4 读自 in_R9+0x14）。→ 已登记 GAP（FIX-6）。

## 过度断言核查

- 无把"未实现/阻塞段"当判等通过的情况；RESTORE.md 对 14 个未实现与全部 blocked 段登记诚实。
- 2 个判等函数为**真判等**（new_flat_roof_rim_ty 的 is_initial 输入源 U 已登记）。
- 唯一过强断言：`add_color` 的"15 事件注册顺序忠实"（已由 FIX-4 纠正措辞，见下）。
- 8 个"重建"中 7 个的 CPU/写入面经逐句对照**判等-quality**，其未落地段均为 core typed-gap（SSBO SsboType、ChimneyExtra+4、PublicWalls 密集表）——按硬标准整体仍标 未实现-需重做（阻塞段），非 restorer 谎报。

## FIX 复核状态（restorer 已处理，2026-10-17 复验）

| # | 项 | 复验结论 |
|---|---|---|
| FIX-1 | `delete_color_icon_entities` 无条件 despawn | **fix_verified**：循环体已留空（`commands` 改 `_commands`），无副作用；progress<=0 门控因 typed-gap 未落地（GAP 登记）。 |
| FIX-2 | `hide_color_menu` 无条件 despawn 菜单/图标 | **fix_verified**：改按 `cmds.read().next().is_some()` 事件存在门控；just_pressed 动作名 U 登记。 |
| FIX-3 | `autoclose_color_menu` menu.center 近似 | **fix_verified**：注明 **stub 非判等**（menu.center 近似图标位置、省略光标 2D 跳过、忽略 kind==0xc 直闭）。 |
| FIX-4 | `add_color`"顺序忠实"表述 | **fix_verified**：RESTORE.md 改为"事件**集合**一致 15、注册面按 workspace 约定 init_resource 胶水、未逐事件对齐、勿读作顺序忠实"。 |
| FIX-5 | custom 判别 | **fix_verified（逐函数核对反编译）**：`==1` 者（反编译 `=='\x01'`）= gate/halftimber/window/chimney/decorator-L2/L3/L4/L5/set_roof 外环；`!=0` 者（反编译 `!='\0'`）= wall/decorator-L1/flag。与 7 个函数的反编译字节级条件逐一相符。 |
| FIX-6 | set_roof 外环第二写遗漏 | **fix_verified（登记类）**：`*(色表+roof_index*4)=uVar4`（uVar4 读自 in_R9+0x14）GAP 登记，未近似补写。 |

补充：set_roof_customization 的 4 层 stride 已在 RESTORE.md 更正为 `0x18/0x20/0x18/0x18`（与本审复测一致）。
复验后 `cargo check -p system-color` 通过（0 error/0 warning 属本 crate）。**FIX 1-6 全部 closed**，无遗留分歧。

## 统计

- 判等 2；未实现-需重做 22（其中 8 个"已实现面判等+阻塞段"，14 个诚实未实现，3 个原占位分歧已修复）；胶水-非业务 2。
- FIX 数 6（1-5 代码/表述修复 + 6 登记类）；复核后全部 fix_verified。
- 编译：cargo check -p system-color 通过（本审核两次复验）。
