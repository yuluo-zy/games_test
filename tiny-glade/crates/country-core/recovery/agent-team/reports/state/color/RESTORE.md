# system-color（crates/systems/color）恢复报告

- Agent：**restorer-state-color**（state 组 restorer）— 2026-10-17
- 范围：`crates/systems/color/src/*.rs`（package `system-color`）全部业务函数
- 方法：全函数 **fresh 反编译**（Ghidra `get_functions` fields=signature,decompiled_code,locals，
  worker=shared；结果落 `reports/state/color/logic/results/fresh_batch{1..3}.json`，逐函数提取至
  `reports/state/color/logic/decomp/fresh{1..3}_*.c`）→ 重建控制流 → Rust body。

## 判等 / 重建 / 胶水 / 未实现 总表

| 函数（地址） | 分类 | 说明 |
|---|---|---|
| `IconInfo::new_separator`（0x1425f2680） | **判等** | 逐字节写面一致：`"white"`(5B)、kind=5、+0x24=0；IconInfo 40B 布局恢复。 |
| `IconInfo::new_flat_roof_rim_ty`（0x1425f26f0） | **判等** | `ClutterVersion::is_initial()` 选 `"color_icon/freehand_ty_terrain"`(30B)/`"freehand_ty_constant"`(31B)，kind=8；**纠错**：原实现 `new_freehand_ty(terrain: bool)` 无实参，与 PDB 实参不符。 |
| `init_color_ids_buffer`（0x142290750） | **重建** | `WallColorIds::new`×3 insert_resource 顺序忠实；`AssetSsboLibrary::add_new::<4 SSBO>` 段 blocked（core `WallColorIdSsbo` 等 typed-gap、无 `SsboType`）。 |
| `set_wall_color_id`（0x142290820） | **重建** | 事件循环/`idx<=0x4fff` 断言/旧色读新色写/halftimber 折线 AABB(`Aabb2::from_points`+center+max_y+xvy)/脏色石膏分类(`new∈{4..10,12}`→Create 否则 old∈该集→Delete)/0x20B 记录 tag=6 全忠实；SSBO `get`+`update_buffer_bytes` 段 blocked。 |
| `set_gate_color_id`（0x142299840） | **重建** | 同 wall 族，tag=7；**差异**：墙命中才写色（反编译 do-while 结构）；无石膏分类。SSBO 段 blocked。FIX-5：custom 判别 `==1`（反编译 `=='\x01'`）。 |
| `set_halftimber_color_id`（0x142299250） | **重建** | 同 wall 族，tag=0xc；**差异**：逐事件无条件写色，halftimber 仅 custom&&墙命中。SSBO 段 blocked。FIX-5：custom 判别 `==1`。 |
| `write_roof_color_id_to_gpu`（0x142298520） | **未实现** | `RoofColorIdSsbo` typed-gap + `GraphicsDeviceResource` 未接线，双源（stride 0x40/0x18）逐元素 `update_buffer_bytes(4B, offset=idx<<2)` 无法编译；签名面保留。 |
| `set_decorator_color_id`（0x1422af0c0） | **重建** | 5 层事件源（0x18/0x18/0x18/0x10/0x18）分派忠实：Decorator+0x28 色、'?'楼梯级联（get_all_connected_nodes 写 +0x28/+0x8/+0xc/+0x10）、'<'灯笼 +0x0、tag 判定（`0xcf>>sub&1`）；记录 value 载荷多为未解出栈值（U）。FIX-5：L2-L5 custom 判别 `==1`（反编译 `=='\x01'`），L1 保持 `!=0`（反编译 `!='\0'`）。 |
| `set_flag_connection_and_chimney_smoke`（0x142298970） | **重建** | 旗写 `FlagExtra.0` tag 9（custom `!=0` 正确）；烟写 `chimney payload+4` blocked（core `ChimneyExtra{pub f32}` 4B 未建模 +4）tag 0xf（FIX-5：烟 custom 判别 `==1`，反编译 `=='\x01'`）。 |
| `set_window_light`（0x142298260） | **重建** | `as_window().f=值`、subtype tag 判定（默认 10/4→0xd/8）、记录 value=+0x28。FIX-5：custom 判别 `==1`（反编译 `=='\x01'`）。 |
| `set_roof_customization`（0x142299dd0） | **未实现** | 4 层嵌套（stride 0x18/0x20/0x18/0x18，reviewer 复测）+ `PublicWalls` 稀疏位集/分块表密集索引（stride 0x14/0xa0/0x48、`~idx*0x30`）——core `PublicWalls` 为 HashMap 抽象未建模；仅外环 `SetRoofColorIdCmd`：roof+0x50=色、custom→`pivot_3d` 推 tag 0xb。FIX-6：外环**第二写**（色表 `*(色表+roof_index*4)=uVar4`，uVar4 读自 Res<PublicWalls>+0x14）遗漏，core 未暴露，登记 GAP。 |
| `color_menu`（0x142291230） | **未实现** | Ghidra 空 / ReVa 超时 / radare2 无符号，三路反编译失败；仅 PDB 候选接口登记。 |
| `color_button_animations`（0x1422979b0） | **未实现** | 数学已解出（进度 +dt/-0.15 或 dt*4、clamp 1.0、EasingFunction 0.8/0.2 二次缓动），core `CustomizationIcon`/`Ui2dExtent` typed-gap 无法落地。 |
| `delete_color_icon_entities`（0x142297ff0） | **未实现** | `progress(+0x18+idx*0x24)<=0` despawn 已解出，字段 typed-gap。FIX-1：已去除无条件 despawn（原无条件销毁与原条件不符）——留空循环体，登记 GAP。 |
| `show_color_options`（0x14229a7f0） | **未实现** | 三路反编译失败；仅 PDB 候选接口登记。 |
| `hide_color_menu`（0x1422adcf0） | **未实现** | 事件/`DigitalActionState::just_pressed`/逐子类型 switch 写者映射依赖 core `ActionSetStates`+图标目录（typed-gap）。FIX-2：已按 `HideColorMenuCmd` 事件存在门控 despawn（原无条件销毁每帧运行即销毁，与原门控不符）。 |
| `autoclose_color_menu`（0x142298d30） | **未实现** | 事件源 stride 0x28/kind==0xc 与 core unit 事件不符；图标目录 typed-gap。FIX-3：eye_position 距离闭合面仅 **stub**（用 menu.center 近似图标位置、省略光标 2D 跳过、忽略 kind 直闭——行为不等原二进制，勿当判等）。 |
| `ColorRadialMenu::create`（0x1425f27c0） | **未实现** | 角度/半径布局已解出（非分隔符计数、`AngleRange::from_min_and_len`、cos/sin、π/4 角步），UI 实体 spawn 依赖 core typed-gap 组件无法编译。 |
| `ui_color_wall_and_roof`（0x141c9ab70） | **未实现** | 模式 12 守卫+raycast 面保留；WidgetParams::signifier 泛型/raycast 命中类型字段（core `RaycastWorldResult` 无 +0x1d8 面）/closure_env_0/2 未恢复。 |
| `color_cursor_icon`（0x141c9b510） | **未实现** | `SignifierEntityBuilder` 族+`UiInactive`+`Ui2dExtent/Ui2dImage` 插入面 core 未对齐。 |
| `ui_color_gates`（0x1422afe70） | **未实现** | 稀疏墙位集/mesh 三角形/signifier 数据面未恢复。 |
| `ui_color_windows_and_flags`（0x1422b0e00） | **未实现** | `legacy_raycast_w_params`/`StairsState::graph` 遍历/outline 面未恢复；菜单类别码 {3,6,9,10,11/12} 已解出。 |
| `replay_color`（0x141c7f270） | **未实现** | 桶元素 stride 0x30、`sub=clamp(payload[0x18]-3,0..13)` switch 13 写者派发、时间推进/移除/drop 分派已完整解出；core `ReplayBuffer/ReplayCommand/DecoratorHistoryEdit` 为高层抽象，原始字节布局未建模。 |
| `add_color`（0x140651870） | **重建（注册面）** | 事件**集合**一致 15 个（Show/Hide/SetWall/SetRoof/SetDecorator/SetBrightness/SetGate/SetHalftimber/SetFlag/SetWindow/SetChimney/SetFlatRoofStyle/SetLadder/SetRailing/SetMagic）。FIX-4（reviewer）：注册面按 workspace 约定 `init_resource::<Events<T>>` 胶水落地，**未逐事件对齐**反编译 `register_event` 顺序（另含 6 个他域事件），Startup-in_set-GameSystemSet-chain_ignore_deferred 与 Update 两批折叠——勿读作"顺序忠实"。 |
| `register_required_components`（0x1425f3d60） | glue-legitimate | bevy required-components 递归注册胶水。 |

## 关键纠错（fresh 反编译 vs 旧实现）

1. `IconInfo::new_flat_roof_rim_ty` 无参数、读 `ClutterVersion::is_initial()`（旧 `new_freehand_ty(terrain)` 自造签名，改）。
2. `set_wall_color_id` 脏色分类 **Create 无条件于 new∈custom_set**（旧 `new_custom && !old_custom` 多加了 `!old_custom`，错）；Delete 为 old∈set && new≠0xc。
3. `set_gate_color_id` 颜色写**仅墙命中时**（旧无条件）；`set_halftimber_color_id` 颜色写无条件、halftimber 记录须墙命中（旧两者混同）。
4. 事件布局（`EventInstance = {event_id u64, E}`，bevy 0.16 fork id 前置）：`SetWindowLightCmd/SetDecorator*/SetLadder/SetMagic/SetBrightness` = `{id u32, value u32, custom u8}` stride 0x18；`SetStairRailingCmd/SetFlag/SetChimney` = `{id u32, value u8, custom u8}` stride 0x10。旧实现 `_gap: u64` 全部不符，已按消费面重建。
5. 0x20B 记录 = `PlayWallUiSound` EventInstance（core audio 契约：kind@+8、selector@+9、value@+0xc、position Vec3@+0x10）；selector/value 载荷在反编译多为未解出字节（U，置 0 并登记）。

## 未实现根因（跨函数）

- core SSBO 类型（`Wall/Gate/Halftimber/RoofColorIdSsbo`）typed-gap、无 `SsboType` → 4 个 set_*/write_roof 的 GPU 段。
- core UI 组件（`CustomizationIcon/Ui2dExtent/Ui2dImage`）typed-gap → 菜单交互/动画/删除/create。
- core `PublicWalls` 为 HashMap 抽象、无原密集表 → set_roof_customization 位集/分块索引。
- core `ReplayBuffer/ReplayCommand/DecoratorHistoryEdit` 高层抽象 → replay_color 原始载荷。
- `color_menu`/`show_color_options` 三路反编译失败 → 内部逐行 unknown。

## 复核 FIX 记录（reviewer-state-color，REVIEW.md，2026-10-17）

| # | 复核项 | 处理 |
|---|---|---|
| 1 | 高危：`delete_color_icon_entities` 无条件 despawn | 已改：留空循环体（progress<=0 门控因 typed-gap 无法落地），不做无条件销毁。 |
| 2 | 高危：`hide_color_menu` 无条件 despawn 菜单/图标 | 已改：按 `HideColorMenuCmd` 事件存在门控（just_pressed 动作名 U 登记）。 |
| 3 | `autoclose_color_menu` menu.center 近似+省略光标跳过 | 已改：注明 **stub 非判等**（图标目录/事件源 typed-gap，真实逻辑待 core 恢复）。 |
| 4 | `add_color`"顺序忠实"表述过强 | 已改：措辞为"事件集合一致 15，注册面按 workspace 约定 init_resource 胶水、未逐事件对齐"。 |
| 5 | custom 判别微差（gate/halftimber/decorator-L4/window 反编译 `==0x01`） | 已改：上述 + 烟/decorator-L2/L3/L5/set_roof 改为 `==1`（反编译 `=='\x01'`）；wall/decorator-L1/flag 保持 `!=0`（反编译 `!='\0'`）。 |
| 6 | set_roof_customization 外环遗漏第二写（色表 `*(色表+roof_index*4)=uVar4`） | 已登记：core 未暴露该面（Res<PublicWalls>+0x14），未近似补写（GAP）。 |

## 编译证据

- `cargo check -p system-color`：**通过**（`Finished dev profile`，0 error / 0 warning 属本 crate）。
  2026-10-17 复验：修复 `set_roof_customization` 的 `mut roof` 借用与 5 处 unused import/variable
  后，`cargo check -p system-color` 在依赖通过时零错误零警告完成。
- 依赖（country-core）曾/现被他组并发错误阻断（save/audio/stairs/ivy/history 组：`load_game.rs`
  OnLoadEntry 字段、`meta_commander.rs` ResMut、`building_history_transactions.rs`/`world_raster.rs`
  HistoryRecordingValidation、`stairs_removed_supports.rs`/`replay_stairs.rs` StairGraphEdge serde、
  `path_edit.rs` StrokeBlendMode Default、`save_thread.rs` mut、`replay_mode.rs` GameTime、
  `save_game.rs` CameraRigParametersCache、`ivy_grower.rs` 等——错误随他组编辑持续变化）。**未代改
  他组文件**；在依赖通过的时间窗内本 crate 已完整通过检查。
- 语法/内部一致性：全部 17 文件经 rustc 独立检查无语法错误；类型错误仅剩跨 crate 解析（依赖阻断
  期间人工核对 import 路径/字段/借用全部通过）。

## Fresh 反编译证据索引

- jobs：`reports/state/color/logic/results/fresh_batch{1..3}.json`（共享 worker，2026-10-17）
- 逐函数反编译：`reports/state/color/logic/decomp/fresh1_{init_color_ids_buffer,set_wall_color_id,write_roof_color_id_to_gpu,set_decorator_color_id,set_flag_connection_and_chimney_smoke,set_gate_color_id,set_halftimber_color_id,set_roof_customization,set_window_light,replay_color}.c`
  `fresh2_{autoclose_color_menu,color_button_animations,delete_color_icon_entities,hide_color_menu,ui_color_gates,ui_color_windows_and_flags,ui_color_wall_and_roof,color_cursor_icon,new_separator,new_flat_roof_rim_ty}.c`
  `fresh3_{add_color,color_menu,show_color_options,create}.c`
- 已知失败确认：`color_menu`(0x142291230) 与 `show_color_options`(0x14229a7f0) fresh 反编译仍为空（codelen=0），radare2/ReVa 已知失败，维持 未实现。
