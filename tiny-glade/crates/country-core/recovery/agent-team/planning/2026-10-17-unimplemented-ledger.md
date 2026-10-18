# 上层 13 模块未实现项总账（按根因分类）

> 生成：coordinator-ledger · 2026-10-17
> 来源：`reports/{eco,state,interaction}/{glade-border,forest,tree-tool,critters,water,timberframe,roof,color,decorator,clutter,onboarding,wall-constructor,toolbar}/RESTORE.md` + `REVIEW.md`（逐函数判等/重建/胶水/未实现+分歧点）。只读汇总，未改 src、未反编译。
> 用途：作为后续"全部实现和核验"的分派依据。总表条目为**未实现/待补证**项（判等与 glue 不计）。
> 上层计划参考：`games_test/reverse-engineering/tiny-glade/upper-modules-plan/`。

## 状态更新（2026-10-17 晚 · coordinator-quickref 汇总）

> 口径：**已闭环**=判等或重建落库且有 fresh 复核；**在途**=结构/公式判等但剩余 U、近似体或下游未落库；**登记**=未实现且已写明解锁条件。
> 来源：`reports/{wave2,wave2b,wave3,gap}/**` + `reports/integration/{coregap-1,coregap-2,purelogic,external-dep}/**` + `reports/host-wiring/**`。
> 说明：`reports/consume`、`reports/remaining` 不存在；对应轮次以 wave2b/wave3/gap 与 integration 报告为准。`reports/unlock/nani` 仅原始证据，NaniBundle 未落库，仍计在途。
> 上层 13 模块逐函数明细见 `2026-10-17-domain-quickref.md` U1-U13；进度见 `2026-10-17-upper-final-progress.md`。

### A. 已闭环（core 类型/API/注册面）

| 类别 | 已闭环项 | 证据 |
|---|---|---|
| coregap-1 | SignifierFactory 0x14089d8b0（40B）；SignifierEntityBuilder 链；PlayUiSound 变体#4（0x20B）；MoveTreeCmd 0x20；ReplayBucket.timing/CommandTiming；HighlightRoofCmd+HighlightColor；GraphicsDeviceResource；WoodenPlankSsbo SparseSsbo；split_height_into_skirts 0x140b34f80；AutoClutterState/Region(0x40)/Spawn(0x68)/Candidate(0x98)；utils::calculate_hash 0x1405bdda0；set_roof_customization 访问面 | reports/integration/coregap-1/{RESTORE,REVIEW}.md |
| coregap-2 | GameProfile 提示区 0x1bc..0x206；18×NotifyHintSystem*；WriteSettingsFileCmd/SaveGameProfileCmd；DisplayRollercoasterPrompt/ModeJustChanged；StructureType::is_fence；Wall/Gate/Halftimber/RoofColorIdSsbo ×4；ChimneyExtra +4；Set*/Decorator*/Ladder/Magic/Brightness 载荷族 | reports/integration/coregap-2/{RESTORE,REVIEW}.md |
| external-dep | taffy 0.3.13、tiny_http 0.12.0、image 0.25.1、open 5.0.0、webbrowser 0.8.10 已补；hotwatch/rapier3d 0.22/bytemuck 原已具备 | reports/integration/external-dep/RESTORE.md |
| gap-core-events | RequestAppModeChange=`(String,SettingsMenuState)` stride 0x18；RequestToolbarCategoryChange=`WindowFocusEvent` stride 0x10；WindowFocusEvent 落库；`HistoryTransactions::ledger_mut()`；`UiSignifierStream::write_raw/write_factory` 承接 40B | reports/gap/core-events/RESTORE.md |
| gap-utils-toolbar | rotation_from_normal 0x140c90a40、orientation_mat3 0x140c99570 **判等**；WidgetByteState/PerWidgetButtonState/FocusedWidgetSubtree/UiNav/HistoryButton 类型落库；keep_focus/switch_category/tod_v2/ui_animation_state 判等（补字节写面） | reports/gap/utils-toolbar/RESTORE.md |
| purelogic/registration | DuckSpawnLocations::new（实现）、FishSpawnLocations::new（判等）；dirty_walls 注册 + detect 分离；decorator add_decorator_systems\<S,C\> 4/5 接线；draw_water 注册验证 | reports/integration/purelogic/RESTORE.md |
| host-wiring | register_core_startup 并入墙域 + History 资源；快照采集 Option 域；Stairs/Raster 写回入口；RoofTileSsbo 存储形态已登记 | reports/host-wiring/REPORTS-HOST-WIRING.md |

### B. 在途（已有判等面，剩余 U/近似/下游未落库）

| 模块 | 在途项 | 剩余 |
|---|---|---|
| tree-tool | process_tree_cmds / replay_tree / ui_move_tree / convert_preplaced_trees | Insert 0x48 分支（T2）、Move undo 旧位置（T3）、slider3d 状态机（分歧 15/16）；~~`insert_starting_edit` 调用~~ **已接线（wave4-misc 判等：convert History 落库）** |
| critters | update_wrens / update_butterfly_states / ui_pet_sheep / ui_pet_ducks | 水/墙资源、state0/2 U、UI 交互 API；~~距离查询 0x140a12650 身份~~ **已解（wave4-misc）**，但 h0 模式门/阈值仍 U；wave4-misc 另列 FIX 4（h0 门/water2 组件/water8 两位置/butterfly flag 基线） |
| timberframe | construct_timberframe_rectangle / circle / beam_count_audio | RenderInstance 字段序、ResMut 库面、bit0 代理、主环 Quat::IDENTITY、size/id 字段 |
| roof | highlight_roof / gpu_flush_buffers / switch_roof_type | RoofTileSsbo SparseSsbo、GraphicsDeviceResource 槽位、NaniBundle mesh ops |
| color | show_color_options / init_color_ids_buffer / set_flag_connection_and_chimney_smoke | case 2..0x26、SSBO 写路径接线、烟值写面 |
| decorator | maintain_dormer_tiles / window_lights / chimney_smoke / replay_decorator / visual build / create_or_restore_backup / calc_gothic_profile | rhapsody 回调表、RoofTileSsbo SparseSsbo、PublicWalls 密集窗桶、ReplayCommand Decorator 变体、NaniBundle、DisplayChildAutoClutterInfo、wall-space 变换 |
| clutter | populate_autoclutter_regions / load_clutter_meshes | 网格表→资产、kind1/default 槽表、记录 stride 0x48、kind4 权重、路径表 I/U |
| onboarding | 22 重建族（hint_roof_shape 已按 REVIEW 修正） | 载荷字节 seam、Steam index/键码、DisplayHintRequest placement@+0/id@+0x49 |
| wall-constructor | BrickRowSine / merge_nearby_proposals / resolve_hole_overlap case0 | 其余族均为未实现，见 C |
| toolbar | place_childen_impl / add_toolbar_item / toolbar_alignment | justify>8/RON 尾区、`{cap,ptr,len}`、kind 载荷、closure_env_0..13 setter 目标 |

### C. 仍登记（未实现，解锁条件已在速查表逐条写明）

- **glade-border**：display_glade_border（DebugLine::show + stream 承接/唤醒）。
- **forest**：handplaced_tree_animation、get_gothic_hat_bones；manage_tree_prefabs 构造形状源 ShapeParameters。
- **tree-tool**：hide_trees、ui_place_tree、place_tree、ui_delete_tree、tree_move_bend。
- **critters**：despawn_fish、plan_duck_trajectories、move_ducks_along_trajectories、update_decorator_bird_spots、update_roof_bird_spots、ducks_react_to_being_inside_buildings；CritterGoal DIVERGE。
- **water**：draw_water::closure$0、橡皮 signifier 布局、WaterAudioGrid 语义、intersects_any max 参数。
- **timberframe**：ConstructTimberframeCmd bit0/+0x1c、GameClock 布局、timberframe_on_glade_unload（bevy_state）。
- **roof**：calculate_roof_trigger_volumes、ui_edit_roof/do_ui_edit_roof、ridge_dir/ridge_length/focused 族。
- **color**：color_menu、set_roof_customization、图标/动画/径向菜单族、replay_color。
- **decorator**：impl$5、apply_decorator_op、decorator_record_history、get_gothic_hat_bones、~146 业务函数。
- **clutter**：manage_autoclutter_entities、read_bush_positions_from_gpu、autoclutter_animation/放置族、merlon/stitch 族、candidates/subscribe/tick/reset 族。
- **onboarding**：hint_orchestrator、hint_draw、render_hints、hint_position_update、hint_focused_edit_hint、hint_clutter_focused_edit_hint。
- **wall-constructor**：half_timbering、arches_n_walls\<true\>、pillars 宿主步骤 4-6、ArchWalker×4、支撑梁 5、segmentation 3、立面支撑整族、门/栅栏/地板/楼梯族、半木/抹灰族、utils 近似体族、gpu_*、maintain_stone_column_colliders、DoorMeshInProgress::add_quad。
- **toolbar**：switch_app_mode_button、toolbar_auto_hide、ui_sounds、ui_slider、render_taffy_ui_to_widgets、mei_pre/post_update、widgets/taffy 族、photomode、menu/load 族、text 族、26 组件、Plugin build。

### D. 状态计数（行级，族条目按组计）

- 上层 13 模块速查 **137 行**：判等 **34** / 重建 **25** / 部分（判等面+未实现面）**3** / 未实现 **73** / 本地等效 1 / DIVERGE 1。
- 剩余登记（未实现 + 部分）**76 行**；按函数面展开约 **305 函数**（decorator ~146、wall-constructor ~69、toolbar ~90，其余模块 0-10）。
- 相对上一版总账，**已闭环**类型/API 约 40 项（A 表），**在途** 10 组，**仍登记** 12 模块。

### E. reviewer-A-toolbar 独立复核（2026-10-17）

> 报告：`reports/reviews/A-toolbar/REVIEW.md`；证据：`work/reviewer-A-toolbar/`。只读，未改 src。
> 范围 12 项（toolbar 4 / purelogic 3 / consume 5）：**判等 3 / 需修 9 / 未实现 0**。

| 条目 | 原账目 | 复核后 | 账目动作 |
|---|---|---|---|
| `gpu_flush_buffers` 0x140de50a0 | A4/未实现→已实现 | **C 判等** | 计 C |
| `maintain_dormer_tiles` 0x1407ce920 | A4/需修 | **C 判等** | 计 C（id_slot null guard 为安全缝 I）；reviewer-B-ecstate 2026-10-17 post-fix fresh 逐语句复核维持判等 |
| `draw_water` 注册 | A2 判等 | **C（限定）** | 原生 build 判等；draw_water 为自足追加（I） |
| `toolbar_button_keep_focus` 0x1423414b0 | C（gap REVIEW） | **需修** | 降 B：`WidgetByteState` 未注册/未挂载 + 四系统未接 schedule → 无可运行等价面；keep/selected 应属 `UiInteractionState`(3B) |
| `switch_toolbar_category_button` 0x1423427f0 | C（gap REVIEW） | **需修** | 降 B：同上空集；click `query.get` 恒 Err |
| `ui_animation_state_update` 0x14233b3f0 | C（gap REVIEW） | **需修** | 降 B：同上空集；appear sign 源=interaction+0 |
| `tod_v2_button` 0x142342f30 | B（FIX-3） | **需修** | 维持 B：清 visible 事件源/顺序错（原生 `ModeJustChanged` 首 tag≠0xd，click 之后） |
| `FishSpawnLocations::new` 0x140f909f0 | A2/判等 | **需修** | 降 B：行列转置 + 公式 7/25 档 1 ULP + used 缺 |
| `DuckSpawnLocations::new` 0x140f8ee90 | 实现 | **需修** | 维持实现面但行列转置（x/z 互换） |
| `replay_tree` 0x141bc34f0 | B | **需修** | 维持 B：Move-undo 旧位路径未实现（G2/T3）；reviewer-B-ecstate 2026-10-17 post-fix：该事件仍不发，维持 |
| `process_tree_cmds` Insert 0x142267430 | B | **需修** | 维持 B：T5 SpawnPrefabCmd 0x50 载荷只发 default；reviewer-B-ecstate 2026-10-17：G4 selector=id&1 判等，T2/T4/T5 仍缺 |
| `ui_move_tree` 0x14226b1a0 | B | **需修** | 维持 B：T3 旧位缺；**MoveTreeCmd 原生 payload 0x18（Rust 0x20 多一 u64）** |

**新增 FIX**：FIX-T1（`UiInteractionState` 3B 合并 / 四系统 Query 修正，高）、FIX-T2（tod_v2
事件源+顺序，高）、FIX-P1/P2（Fish/Duck 转置与公式，高）、FIX-C1..C4（replay 旧位 / Insert T5 /
ui_move T3 / MoveTreeCmd 布局，中）。其中 T1/C4 为 core 类型改动，需集成 owner 裁决。

---

### F. reviewer-B-ecstate post-fix 独立复核（2026-10-17）

> 报告：`reports/reviews/B-ecstate/REVIEW.md`；证据：`reports/reviews/B-ecstate/evidence/`。
> 范围：ecstate 审计 B 类「复核提 FIX 后已改但无 post-fix 复验」8 项。只读，未改 src。
> 方法：Ghidra fresh `get_functions`/`disassemble_function` + radare2 定点反汇编 + 跳表/常量/
> 字符串字节实读；`show_color_options` Ghidra/ReVa 反编译超时，改用 fresh 反汇编。

**三态：判等 1 / 仍有 FIX 7。**

| 条目 | 命名 FIX 复验 | 函数级 | 账目动作 |
|---|---|---|---|
| `maintain_dormer_tiles` 0x1407ce920 | verified（游标写序/id 单槽/GPU 退役） | **判等** | 维持 C（rhapsody 注册/RoofTileSsbo 生产者仍为宿主接线） |
| `move_critters` 0x142318fa0 | verified（water2=Sheep 双采样、spherecast 门、blocked 三件套） | **仍有 FIX** | 纯墙 relate 路未实现（0x142319cc3→query/get/max_y_gable/relate）、spherecast 半径 0.8/0.5 未传、PublicWalls::get panic → 维持 B |
| `critters_avoid_collisions` 0x1423179a0 | verified（Duck 4+4、hitcode==2 保留、loc<7 保留、relate 方向） | **仍有 FIX** | 半径未传、relate 形状 Circle2d+segment 近似、**零方向提前清目标（新）**、panic → 维持 B |
| `update_wrens` 0x14231e390 | verified（h0 模式门/阈值 1.0/3.0/0 与顺序；h3 判等） | **仍有 FIX** | h1 尾部：`qword [state+0x30]=2` 顺带清 timer、`+0x1c/+0x20/+0x24=1.0`、音频槽/call → 维持 B |
| `update_butterfly_states` 0x141d84d40 | verified（flag 基线 4/3、4/1；state0/1/3 判等） | **仍有 FIX** | **新发现**：state2 次态二进制 `movd(-1)+2 = 1`，Rust 写 3；state4-7 forwarder/audio u32 写 → 维持 B |
| `replay_tree` 0x141bc34f0 | 未实现（Move undo 旧位事件仍不发） | **仍有 FIX** | 维持 B：G2/T3 + Insert 0x48 T2 |
| `process_tree_cmds` 0x142267430 | verified（G4 selector=id&1；Move/Delete/Entities 门/音效） | **仍有 FIX** | T2（Insert 0x48 字段）、T4（is_corrupted/drag_started 固定 false）、T5（SpawnPrefabCmd 0x50 只发 default）→ 维持 B |
| `show_color_options` 0x14229a7f0 | 部分 verified（case0 13 swatch/plaster_peel/halftimber；case1 11 roof） | **仍有 FIX** | **新发现**：plaster swatch 副值 `+0x24`=0,7,1,6,4,5,2,3 未写；case2..0x26/demo/门源（get_instruction、+600、eye offset）GAP → 维持 B |

**新增/未闭合 FIX**：FIX-B1（butterfly state2 次态 3→1，高，影响 2↔1 状态链）、
FIX-B2（show_color_options plaster v24 副值，中，UI 载荷）、FIX-B3（avoid 零方向提前清目标，低）、
FIX-B4（move/avoid 半径 0.8/0.5 未传，中）、FIX-B5（move 纯墙 relate 路径，高）。

---

### G. reviewer-B-state 独立复核（2026-10-18）

> 报告：`reports/reviews/B-state/REVIEW.md`；证据：`reports/reviews/B-state/evidence/`。
> 范围：ecstate 审计 B 类中 **water/timberframe/roof/color/decorator** 全部业务函数
> （water 的 B 类为 0；实审 24 项 = timberframe 2 + roof 3 + color 11 + decorator 8）。
> 方法：Ghidra fresh `get_functions` 24 函数；`show_color_options` 反编译超时→radare2
> fresh 反汇编（入口/case0 demo+非 demo/case1 非 demo）+ 跳表 0x14303ca70 40×dword 实读；
> `apply_decorator_op` guard 以 radare2 反汇编 + caller edx 面补证。只读，未改 src。

**三态：判等 3 / 需修 13 / 未实现 8。**

| 条目 | 旧账 | 本轮 verdict | 账目动作 |
|---|---|---|---|
| `highlight_roof` 0x141a284b0 / 0x141a31280 | B（未实现→重建） | **判等** | 升 C：清 tag→逐事件 value/tag/to_rgb；8B store 覆盖 pad 为 I |
| `maintain_dormer_tiles` 0x1407ce920 | B（G3 未复验） | **判等** | 升 C（与 B-ecstate 结论一致；宿主接线不计） |
| `construct_timberframe_rectangle` 0x141a41ba0 | B | **需修** | 控制流/常量/计数判等；`RenderInstance` 逐轴尺寸（截面×段长）与单标量 `WoodenPlankSsbo.size` 不等；`rotation_from_normal` 代理 I |
| `construct_timberframe_circle` 0x1421b67a0 | B | **需修** | 同上；主环朝向 fresh L287-299 由主高构造，Rust `Quat::IDENTITY` 占位（FIX-BS4） |
| `generate_roof_stone_floor_and_roof_bottom` 0x1421a6d40 | B | **需修** | CPU 控制流（事件→roof→`(b8&1)==0`→log2f(r*3)→queue）判等；Nani insert 执行体 no-op |
| `init_color_ids_buffer` 0x142290750 | B | **需修** | 3×insert 顺序判等；4×`add_new<*ColorIdSsbo>` 缺（typed-gap） |
| `set_wall_color_id` 0x142290820 | B | **需修** | 事件/索引/写色/AABB/石膏分类判等；SSBO 上传段缺；selector 证据 U |
| `set_gate_color_id` 0x142299840 | B | **需修** | 墙命中门→写色→custom==1 tag7 判等；SSBO 缺 |
| `set_halftimber_color_id` 0x142299250 | B | **需修** | 无条件写色 + custom==1&&墙命中 tag0xc 判等；SSBO 缺 |
| `set_decorator_color_id` 0x1422af0c0 | B | **需修** | 5 层事件源/写偏移/tag 判等；L1 非 sub4 与 L3/L4/L5 记录 value、L1 selector 为栈 U（FIX-BS14） |
| `set_flag_connection_and_chimney_smoke` 0x142298970 | B | **需修** | **新发现**：记录 selector 字节 = `event+0xc`（`CONCAT11(value,kind)`），Rust 置 0；烟 `+4` 写缺（FIX-BS1） |
| `set_roof_customization` 0x142299dd0 | B | **未实现** | 外环 dense 索引 + 第二写 `*(色表+idx*4)` 缺；样式环/History 未建（FIX-BS3） |
| `show_color_options` 0x14229a7f0 | B | **需修** | **FIX-B2 关闭**：case0 13 swatch v1c/v24 逐记录与机器码相等；case1 11 roof 首值判等；仍缺 case2..0x26/事件写面/门源 |
| `hide_color_menu` 0x1422adcf0 | B | **未实现** | 仅事件门+全量 despawn；逐图标 subtype switch 缺（FIX-BS5） |
| `autoclose_color_menu` 0x142298d30 | B | **未实现** | kind==0xc 直闭、图标 2D 跳过、3D 图标位置均缺 |
| `ColorRadialMenu::create` 0x1425f27c0 | B | **未实现** | AngleRange/cos·sin/spawn 4 组件/tag-3 分派缺 |
| `apply_decorator_op` 0x141276e30 | B | **需修** | guard 二进制为 `(archivist==0 & kind_byte)!=0`（bit0），Rust `kind_byte!=0`（FIX-BS6）；历史 48B 仅首 qword 写、其余 U |
| `create_or_restore_decorator_backup` 0x1408238a0 | B | **需修** | 状态机/主存储 tracked clone/restore/+0x48 门控判等；辅助 0x30B 三 map 反向拷贝缺 |
| `decorator_record_history` 0x140817850 | B | **需修** | 循环/清空/kind6 break 判等；48B `{1,1,kind,f1,f2,f3}` 载荷未写（default edit，FIX-BS7） |
| `window_lights` 0x1407b5980 | B | **未实现** | 阈值/灯态/retain 序/帧末公式判等；窗实体收集主循环与形状变化更新缺（FIX-BS9） |
| `chimney_smoke_anim_thickness` 0x140818df0 | B | **未实现** | 窗口/目标/符号/`clamp01(old+sign*dt/target)` 判等；收集+初始插入缺；**新发现**：retain 在更新前，Rust 在更新后（FIX-BS2） |
| `replay_decorator` 0x14080c8d0 | B | **未实现** | tag7/0.05 floor/elapsed/索引反转模型判等；core 无 Decorator 变体与桶 cull 表，运行期 no-op |
| `calc_gothic_half_dormer_profile_ws` 0x1407eb160 | B | **未实现** | 仅 0..2 骨架；wall-space A/B/C/D lerp/法线/截断未还原 |

**新增/未闭合 FIX**：FIX-BS1（set_flag selector/烟 +4，高）、FIX-BS2（chimney retain 顺序 + 收集，高）、
FIX-BS3（set_roof 第二写/密集表，高）、FIX-BS4（circle 主环朝向，高）、FIX-BS5（hide subtype switch，高）、
FIX-BS6（apply guard bit0，中）、FIX-BS7（record_history 48B 载荷，中）、FIX-BS8（backup 辅助三 map，中）、
FIX-BS9（window 收集，中）、FIX-BS10（generate_roof 执行体，中）、FIX-BS11（show case2..，中）、
FIX-BS12/13（color SSBO 段，中）、FIX-BS14..17（U/合同，低）。
**关闭**：FIX-B2（show_color_options plaster `+0x24` 副值）——fresh 反汇编确认已正确落库。

---

## 根因分类定义

1. **core-type-gap**：缺 country-core 类型/字段（SSBO typed-gap、事件载荷、状态布局、GameProfile 提示区等）→ 输出"所需具体类型/符号"（core 团队直接领）。
2. **core-api-missing**：缺 core 方法/接线（DebugLine::show、draw_raster 参数面、History insert_starting_edit、GraphicsDeviceResource、WidgetParams 变体裁决、subscribe 族、geo_src 族、渲染边界等）。
3. **external-dep**：缺 workspace 依赖（taffy 0.3.13、tiny_http 0.12.0、image 0.25.1、open/webbrowser/hotwatch、rapier 等）。
4. **decompile-hardpoint**：反编译/跳表/内联卡点（Ghidra 空/碎片化、jumptable 破坏、inlinee 提取、泛型单态定位）。
5. **in-module-pure-logic**：依赖已具备（或本地可等效），只是尚未重建的纯模块内逻辑 → 应可直接实现。
6. **integration-registration**：函数已实现/判等但无调度注册点。

---

## 总表：模块 | 函数/条目 | 状态 | 根因分类 | 动作建议

### eco/glade-border

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `display_glade_border`（0x141be0ad0）| 未实现-需重做 | core-api-missing + core-type-gap | core-task（DebugLine::show 0x140b04f70 + SignifierFactory/perimeter 写路径）【coregap-1：SignifierFactory 已落库；DebugLine::show 仍缺】【reviewer-coregap-1 2026-10-17：SignifierFactory 判等——fresh 0x14089d8b0 写 {len,ptr,len,&inner,marker} 40B】【reviewer-wave3-tg-tr 2026-10-17：**未实现-需重做核验通过**——分歧① DebugLine::show（0x140b04f70 fresh：算折线总长+push&lt;UiAppendSignifier&gt;）core 无 show 属实；分歧② 48B 记录+SignifierFactory::new+UiSignifierStream::write 属实，core write 仅收 UiAppendSignifier（typed slots）不可承载；控制流 1-3/demo/格式串判等维持】【reviewer-wave4-misc 2026-10-17：**perimeter 写路径判等**——fresh r2 0x141be0d45-e97：`GameConfig+0x121==0` 门、perimeter 0x1411ff640、`format!("glade_border{}", GameTime+0x2c)`（字面量实读 12B 无空格）、alloc 0x30 写 `{1,1,cap,ptr,len,perimeter}`、`SignifierFactory::new(label,len,record,0x142efa150)`、write 0x140b2cf90 逐句吻合；`DebugLine::show` core 无（grep 确认）→ 未实现维持】|
| `display_glade_border::closure$1`（0x141be1080）| 重建（wave3 落库）| core-type-gap | core-task（SignifierEntityBuilder 链 + 2×Commands insert）【coregap-1：链已落库；Commands 组件入队 GAP】【reviewer-coregap-1 2026-10-17：**需修**——①new 未拆 kind（二进制 48B 载荷首 qword=mesh_kind，Rust 全写 mesh_payload 且 mesh_kind=0）②default 未装默认 shader 名 "vertex_color_shader"（len 0x13）③custom_shader 未释放旧串（二进制 len@+0x60!=0 则 dealloc）；set_ui_pass/animation_duration/build(GAP 诚实) 判等】【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——core new 已拆 mesh_kind（①已修）、custom_shader 已释放旧串（③已修）；closure_1 链（mesh_kind 0x8000000000000002 + 2×insert DrawTopology 0x4000000000000001 / DashedLineLength×0.7）判等；唤醒 vtable 0x142efa150 未接线（dead_code）诚实】【reviewer-wave4-misc 2026-10-17：**判等复核通过**——fresh r2 0x141be1080-1299：env{ptr@8,len@0x10,f32@0x18}、alloc 12*len+memcpy、builder 载荷 `{0x8000000000000002,len,ptr,len}`、custom_shader("dashed_line",len 11)、2×insert `0x4000000000000001`（Alt+2.0）与 `env.length*0.7`（0x1429cd030）逐句吻合】|
| `feedback_input_out_of_border`（inline 宿主 0x141be2280）| 判等（wave3 落库）| core-type-gap | core-task（PlayUiSound 变体 #4）【coregap-1：PlayUiSound 变体 #4 已落库（0x20B {kind,payload,id}）】【reviewer-coregap-1 2026-10-17：判等——fresh 0x141be2280 写判别 4@+0、id@+0x18、stride 0x20】【reviewer-wave3-tg-tr 2026-10-17：**判等核验通过**——fresh 0x141be2280 上升沿去抖（cache@+0x158==0）+ 写 kind=4@+0/id@+0x18/stride 0x20 + cache 置位；Rust 一致】|

### eco/forest

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `manage_tree_prefabs` 记录布局（0x141bf16c0）| 未实现（登记）| core-type-gap | core-task（InteractableForestTree 0x50B / SpawnPrefabCmd 0x50B）|
| `manage_tree_prefabs` 构造形状判定 | 判等（2026-10-17）| —（矩形分解/墙回调登记）| 【reviewer-wave4-misc 2026-10-17：**判等**——fresh r2 0x141bf1ba0-1ee0：6 路采样/后 2 路排除/rock/`bit108\|density\|rock` 门控/点 `radius+7>dist`/矩形 `closest_perimeter<7`/形状过近跳过墙查询逐句吻合。矩形分解 core `rectangle_closest_u_from_pos` 注释地址 0x140c99530 错（实为 perimeter；closest-u=0x140c9a9d0、rect pos_at_u=0x140c99800），拓扑一致、数值等价未逐指令对线，登记】|
| `manage_tree_prefabs` `has_construct_shape` 门控位来源 | 待证 | re-analysis | **已补证（2026-10-17）**：门控位=`*(char*)(lVar24+0x18)`（SSBO 缓冲/资源记录处理状态字节），首帧置 1 后后续帧走随机 scale 分支；非事件存在性（RE-ANALYSIS §6.1）|
| `handplaced_tree_animation`（0x142279500）| 重建+未实现（布局）| core-type-gap | core-task（TreeAnimation +4 / PrefabInstanceData 0x30）|
| `forest_tree_animation`（0x14227a0d0）| 判等+2 缺口 | core-type-gap | core-task（+0x44 有树标志 / DespawnPrefabCmd 载荷）|
| 盒扩展 Aabb3::intersect_segment 3D | 本地等效（标注）| core-api-missing | dep-task（utils）|

### eco/tree-tool

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `process_tree_cmds`（0x142267430）| 重建（wave3：Move/Delete 判等；Insert 未实现）| core-type-gap | core-task（MoveTreeCmd 0x20 / InsertTreeCmd 0x48 / Delete 含 Entities::contains 门控）【coregap-1：MoveTreeCmd 已落库；InsertTreeCmd 冲突跳过（布局合同见 §1 tree-tool）】【reviewer-coregap-1 2026-10-17：MoveTreeCmd 判等（fresh 0x142267430 消费 stride 0x20、0x14226b1a0 发射 {id,index,pos}）；InsertTreeCmd 冲突登记准确（fresh place_tree 0x142269970 逐字段核 0x48 合同）】【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——Move 分支判等（get(index@+8)→None log/Some 写位置+采样+PlayTreeSoundCmd{kind=1}，T1 in_water@+9/inverse_lerp@+0xc 缺位登记准确）；Delete 分支 Entities::contains 门控判等（fresh 行 834-835）；Insert 分支 decomp 全链（4 组件插入/0x50 SpawnPrefabCmd/冲突 Despawn）未实现 T2 诚实；查询失败 unwrap_failed panic→graceful 兜底已登记】|
| `convert_preplaced_trees`（0x14226be70）| 重建（wave3：Wait flag 判等；insert_starting_edit 未实现）| core-type-gap | core-task（History::insert_starting_edit；Wait 一次性门控；岩石掩码 buffer）【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——Wait flag（事件→count=1/flag=1；无事件且 flag 置位 count+1>2→flag=0 转块）判等（分歧 12 已修属实）；水/岩/花园/路径密度/墙冲突→DeleteTreeCmd 方向判等（墙查询闭包 fresh 0x1425defd0 补证：命中墙→清位→delete；无墙→保持→转换）；分歧 15 证实（insert_starting_edit 需 &mut ObservedHistoryLedger，HistoryTransactions 仅只读 ledger()）】【reviewer-wave4-misc 2026-10-17：**判等复核通过（含 History 落库）**——fresh decomp 0x14226be70 尾部 0x14226c1f0 `from_iter<EditType, Map<Drain<TreeEdit>>>` + `History::insert_starting_edit` 调用证实；core `ledger_mut()` 已存在（history_transactions.rs:135），本波 `HistoryEntry::from_edits + insert_starting_edit(ledger_mut,entry,fastrand u64)` 与控制流等价；残余①position=PlacementDataSsbo::position（Rust 以 Transform 桥接）②scale/rotation 记录 +0xc/+0x10③候选 Decorators::iter_removed_ids vs Query④rock buffer 空占位——均属实】|
| `replay_tree`（0x141bc34f0）| 重建（wave3：时序/Move/Delete/Place/undo 判等）| core-type-gap | core-task（ReplayCommand/ReplayBucket elapsed/delay/flags/speed）【coregap-1：ReplayBucket.timing + advance_timing 已落库】【reviewer-coregap-1 2026-10-17：CommandTiming 偏移判等（flags@+8/delay@+0xc/elapsed@+0x18/started@+0x1c）；**advance_timing 需修**——位极性反转（二进制只对 bit0 置位命令推进 elapsed、清位命令待移除；Rust 反向跳过置位）】【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——位极性已按二进制修（bit0=0 记移除 / bit0=1 delay/advance）；elapsed==0.0 非 NaN 门、Move 非 undo→MoveTreeCmd{index=edit id,position 新}、Delete 非 undo/Place undo→DeleteTreeCmd{id}、Delete undo/Place 非 undo→0x48 InsertTreeCmd（T2 缺 height/variant/rotation/flags）、Move undo edit+0x14/+0x1c 旧位置（T3 未实现）、逆序 Vec::remove——全部判等/诚实】|
| `tree_move_bend`（0x1422689c0）| 未实现-需重做 | core-type-gap | core-task（PrefabInstanceData +0x24/+0x28 step 写面）|
| `hide_trees`（0x14226a260）| 未实现-需重做 | core-type-gap（2026-10-17 转）| core-task（HideTreesinAreaCmd/NextToWallCmd 补 shape 字节@+0 bit0 + @+0x24 门控字节；树级 `Decorators` 候选源资源接线；两命令缓冲 stride 0x18/0x30）【reviewer-wave4-misc 2026-10-17：**未实现登记属实**——core `HideTreesinAreaCmd`/`HideTreesNextToWallCmd` 仍仅 `{center,radius}`（tree_storage.rs:102/112），shape 字节/@+0x24 门控缺；按硬标准不改 core】|
| `ui_place_tree`（0x142269170）| 未实现-需重做 | core-api-missing | core-task（raycast+geo_src_to_edit_subject 早退分支）|
| `place_tree`（0x142269970）| 未实现-需重做 | core-type-gap | core-task（InsertTreeCmd 全字段 + 会话门控接线）【coregap-1：InsertTreeCmd 冲突跳过，布局合同见 §1 tree-tool】【reviewer-coregap-1 2026-10-17：冲突登记准确——fresh place_tree 事件写块 stride 0x48 逐字段核对合同 {name@+0,position@+0x10,scale@+0x20,{variant,rotation}@+0x28,flags@+0x30,id@+0x38,height@+0x40}，tree_edit.rs 现由 history 组持有】|
| `ui_move_tree`（0x14226b1a0）| 重建（wave3：载荷判等；slider3d 近似）| core-type-gap + in-module | core-task（MoveTreeCmd 全载荷 + slider3d case1/2/3 + case2 界外双事件）【coregap-1：MoveTreeCmd 0x20 已落库】【reviewer-coregap-1 2026-10-17：判等——fresh 0x14226b1a0 事件写块 stride 0x20 {u64 id@+0,u64 index@+8,3×f32 position@+0x10}】【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——MoveTreeCmd 载荷（case2/3 写 +8=树 id、+0x10=x、+0x14=y=max(h,0)、+0x18=z）判等；分歧 15/16 诚实（case2 界外双事件 DisplayOutOfBorder+FeedbackInputOutOfBorder fresh 行 318-341 属实，Rust 顶部早退使不可达）；**补登记**：二进制门禁 mode∈{0x10,0}（0=None 亦放行），Rust 仅 Tree】|
| `ui_delete_tree`（0x141bd2510）| 未实现-需重做 | core-type-gap | core-task（PathDrawEvent flag@+0x18 / 中心半径源）|

### eco/critters

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `move_critters`（0x142318fa0）残留 | fix_verified+残留 | re-analysis | **已补证（2026-10-17）**：命中码 2=岩石（sample>0 阻塞）、3=跳过墙 gable 检查、非位集 loc∉{0,1,5,6} 才 spherecast；mult==0 时位移项=vel*dt*0 恒 0，仅墙 relate/朝向分支（RE-ANALYSIS §6.2）【reviewer-wave2-critters-anim 2026-10-17：命中码判等（fresh 0x142318fa0 核 `local_318!=2 && 球距<1.0` 才阻塞）；**补充登记**：spherecast ①外层门 `(local_fc\|\|local_100)`（Duck/Sheep Option 存在标志，映射 U）被 Rust FIX① `(loc>6)\|\|!wall_hash_only()` 省略；loc==2 路径 AL 标志未建模】【reviewer-wave4-misc 2026-10-17：**核心判等面复核通过**（loc2 `rock\|\|water>0\|\|(water<=0&&Duck)`、非 loc2 `water<=0&&Duck`、spherecast 门/命中码/纯墙路登记）；**新 FIX**：双水采样组件取错——二进制 `[rbp+0x23c]`=+0x138=**Sheep**（`(local_fc&1)&&loc!=4`，半径亦 0.8），Rust `duck.is_some()` 应改 `sheep.is_some()`；列映射 +0x128 Multiplier/+0x130 Duck/+0x138 Sheep/+0x140 BirdSpots 已证】【reviewer-B-ecstate 2026-10-17 post-fix：water2=Sheep 双采样/门/blocked 三件套 **fix_verified**；纯墙 relate 路仍缺（0x142319cc3→query/get/max_y_gable/relate）、半径 0.8/0.5 未传、PublicWalls::get panic → 维持 B】|
| `critters_avoid_collisions`（0x1423179a0）残留 | fix_verified+残留 | re-analysis | **已补证（2026-10-17）**：hitcode==2 且 rock_sample>0 → 跳过碰撞；`hit==2\|\|球距>=1.0` → 保留目标；`<0.4 && water8` → DespawnDuckCmd；water8=8×bilinear 求和 clamp01（RE-ANALYSIS §6.3）【reviewer-wave2-critters-anim 2026-10-17：命中码判等（fresh 0x1423179a0 核 `local_258==2 \|\| 球距>=1.0 → 保留`、`<0.4 && water8 → DespawnDuckCmd`）；water8 映射登记准确；墙 relate 循环语义（模块头已登记，RESTORE 残留表补单列）】【reviewer-wave4-misc 2026-10-17：**分支/方向判等**（Duck 栅格、Sheep spherecast、无→墙 relate、`loc==2 @ pos+dir*0.1` 水+岩、Intersection→清目标/遍历完→保留、`<0.4&&Duck→Despawn`、spherecast 门）；**新 FIX**：Duck 栅格是**两位置 4+4=8 次采样**——`c0=clamp(Σ4 sample(pos))`；`d=(c0>0)?\|goal.position\|:0.3`；`c1=clamp(Σ4 sample(pos+dir*d))>0` 才清目标。Rust 仅单点 4 次且以 c0 判清目标，需重写】【reviewer-B-ecstate 2026-10-17 post-fix：Duck 两位置 4+4/hitcode==2/loc<7/relate 方向均 **fix_verified**；半径 0.8/0.5 未传、relate 形状 Circle2d+segment 近似、**零方向提前清目标（新）**、panic → 维持 B】|
| `update_wrens`（0x14231e390）| 重建+未实现(跳表) | decompile-hardpoint | **已攻克（2026-10-17）**：状态跳表 DAT_14304b638 解码（5 handler）+ 常量全读；state 0=计时/1=音频+state→2/3=smoothstep 移动/2,4=扫描续（RE-ANALYSIS §1.1）【reviewer-wave2-critters-anim 2026-10-17：部分判等（跳表 5 handler 字节复算全对；h3 t/smooth/phase/vx/dvy 公式与高度更新尾部 `clamp01(h+dt*(state==0?-50:5))` 逐语句判等）；**需修 2**——①h3 尾部 anim_val=progress×7.0（0x142a7cd48=7.0，Rust `anim_val=progress` 缺 ×7，RESTORE 未登记）②移动精确式 `pos.y+=vx*dt*(dvy*dir_y+dir_z)`、pos.z 不变（Rust 现 y/z 分列）；距离查询 0x140a12650 / 音频 call / >6.0 panic 路径登记准确】【reviewer-wave4-misc 2026-10-17：**wave2 需修 2 项均已修且核实**——fresh r2 证实三轴式 `pos.x/z += vx*dir*dt`、`pos.y += vx*dvy*dir_y*dt`、anim_val=progress×7.0；`0x140d14770` ghidra fresh = `EntityCommands::despawn`（error handler=warn，非 panic；wren.rs 串是 track_caller Location）→ 旧 panic 结论推翻成立。**新 FIX**：h0 距离查询缺模式门/阈值——xmm7 由模式（EDX 6..20，byte[res+4] 子模式）取 1.0/3.0/0，`xmm7>0 && dist<xmm7` 才进 h1；Rust 无条件 `<1.0`】【reviewer-B-ecstate 2026-10-17 post-fix：h0 模式门/阈值（1.0/3.0/0 与顺序）**fix_verified**、h3 判等；h1 尾部 `qword [state+0x30]=2`（顺带清 timer）、`+0x1c/+0x20/+0x24=1.0`、音频槽/call 仍未建模 → 维持 B】|
| `update_decorator_bird_spots`（0x14231c7d0）/ `update_roof_bird_spots`（0x14231d6c0）| 未实现 | re-analysis（2026-10-17 转）| 补证（create_spots 布局/数量规则、random_splits 分段语义、双列表滑动窗口缓冲结构、装饰物注册表查询）|
| `plan_duck_trajectories`（0x14230ff00）| 未实现 | re-analysis（2026-10-17 转）| 大段重建（dev 热键 A/B + CursorTerrainRaycast + RaycastWorld::legacy_raycast 插值 + Frenet + Trajectory 4-f32 偏移布局 + spawn 10 组件 bundle）|
| `move_ducks_along_trajectories`（0x142312200）| 未实现(部分) | re-analysis（2026-10-17 转）| 补证（Frenet 帧 / 轨迹状态 ∈{0,2} vs 现{1,3}）|
| `despawn_fish`（0x140f90c10）| 未实现 | core-type-gap（2026-10-17 转）| core-task（`FishBoids(Resource)` 类型 + `OnExit<AppLoadingState>` bevy_state；真实=资源重置非实体清理）|
| `DuckSpawnLocations::new`（0x140f8ee90）/ `FishSpawnLocations::new`（0x140f909f0）| **已实现/已验证（2026-10-17）**| —— | DuckSpawnLocations::new 改精确网格 `(i/9)*52-(1-i/9)*52` + timer=25.0（0x41c80000）；FishSpawnLocations::new 公式/行列序与二进制一致（余 used 字节 DIVERGE 登记）|
| `ui_pet_sheep`（0x142324f80）/ `ui_pet_ducks`（0x142322e30）| 重建(核心)+未实现(UI) | core-api-missing | core-task（UiWidgetContext::intersecting / SignifierEntityBuilder / slider3d::into_drag_state）|
| `ducks_react_to_being_inside_buildings`（0x14231b140）| 未实现-需重做 | core-api-missing | core-task（PublicWalls::get / get_wall_shape / Roof::height / GladeBorder::is_pos_inside）|
| `sheep_walk_animation`（0x14231a630）/ `sheep_pet_animation`（0x14231ac20）| 重建+未实现(跳表) | decompile-hardpoint | **已攻克（2026-10-17）**：handler 跳表 0x14304b0f8（state 0/3=无 handler、1=walk fmod 帧索引 mod 30、2=过渡 dt/0.1→3）/ 0x14304b178（pet 状态机 {0..4}，参数 0.3/1.0/0.2，[spot+0x58] 进度+[0x60] 标志）（RE-ANALYSIS §1.2/1.3）【reviewer-wave2-critters-anim 2026-10-17：**判等**（两跳表字节复算全对；walk h1/h2 与 pet h0-h4 逐语句，含 mod30 魔数/饱和转换/NaN 分支；pet h0 实际是 qword store 清零 progress，RESTORE"h0 不写 +0x1c"说明不准但结论成立）】|
| `update_butterfly_states`（0x141d84d40）/ `set_butterfly_velocities_towards_goals`（0x141d867b0）| 重建+未实现(跳表) | decompile-hardpoint | **已攻克（2026-10-17）**：DAT_142f68008 解码（8 项，state≥4 转发 vel 表）+ 动画跳表 DAT_142f68018 解码（4 项：清零/衰减/钳制/固定 y）（RE-ANALYSIS §1.4/1.5）【reviewer-wave2-critters-anim 2026-10-17：部分判等（vel 表 4 handler 逐语句判等；states 主循环/step_state/state1/3 结构判等；state0/2 U、state1 水/墙/xmm1==xmm14 门 U 登记准确）；**补充登记**：state3 NaN 高度次差（二进制保持 3、Rust →2）；states slot5-7 为非对齐字节非有效 handler（仅 slot4 forwarder 有效，"state4-7 转发"表述略宽）】【reviewer-wave4-misc 2026-10-17：**state0-3 handler 实体判等**（fresh r2 4 handler + 跳表字节复算；minss/NaN、0.6/0.01/5.0/1.5 门、fmod 续段全对上）；**新 FIX**：dispatch flag 初值基线——二进制 `phase>0?4.0:baseline`，baseline 逐路为 **3.0**（dwell<=0 两路/vel.y>0/相位门外）或 **1.0**（0.2<=\|0.5-phase_ref\|<0.45 两路）；Rust 一律 0.0。影响 state0 `min(candidate,flag)` 与 `phase_ref += flag*dt`。RESTORE 的 4/0 mask 描述与机器码不符】【reviewer-B-ecstate 2026-10-17 post-fix：flag 基线 4/3、4/1 与 state0/1/3 均 **fix_verified**；**新 FIX**：state2 次态二进制 `movd(-1)+2=1`（Rust 写 3），state4-7 forwarder/audio u32 写仍缺 → 维持 B】|
| CritterGoal 字节序 / `sample_at_world_xz`→u32 | 语义桥接(DIVERGE) | core-type-gap | core-task（跨组类型变更，集成 owner 定夺）|

### state/water

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `draw_water::closure$0`（0x141ba9700）| 未落库-依赖 core signifier 域 | core-type-gap | core-task（SignifierEntityBuilder 链 + "brush_circle_scribble"）|
| 橡皮 signifier 记录布局 | 待 core 复核 | core-type-gap | core-task（SignifierFactory 40B {name_len,name_ptr,name_len,&inner,marker} vs UiAppendSignifier）【coregap-1：SignifierFactory 已落库，复核可续】|
| `update_water_audio_grid` intersects_any max=1.0 | 语义差（已登记）| core-api-missing | core-task（core API 加 max 参数则回填）|
| `WaterAudioGrid` 语义投影 | 契约差（待音频域裁决）| core-type-gap | core-task（24B 单元 {x,z,窗@+8,计数@+0x10} vs 语义模型）|

### state/timberframe

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `construct_timberframe_rectangle`（0x141a41ba0）/ `circle`（0x1421b67a0）梁写路径 | 重建+写路径落库（wave3）| core-type-gap | core-task（WoodenPlankSsbo SparseSsbo/SsboType + render::RenderInstance）【coregap-1：WoodenPlankSsbo SsboType 已落库；RenderInstance 待证】【reviewer-wave3-tg-tr 2026-10-17：**重建+写路径核验通过**——rect 相位/计数（n=⌊d/1.51⌋、截面 0.2/0.18、裙层两固定环、屋脊 min(lenA,lenB)+perimeter>0.5→+1、BeamCount+8 写回、event[0x1c] 门控）判等；circle count=max(6,⌊perimeter/1.5⌋)/u=i/(count-1)/屋顶环高度=roof_height+cache+8+cache+0xc/空槽 panic 判等；写路径（clear+逐梁 push+ran-out-of-memory）落库；分歧（ResMut 库面/RenderInstance 字段序 I/rotation_from_normal→beam_rotation I/bit0 代理/主环 IDENTITY 占位 GAP）诚实；circle 不 clear（与 rect 异）忠实】|
| rect/circle 裙层环数 / 屋脊环 | 重建+写路径落库（wave3）| core-type-gap | core-task（resources::walls::split_height_into_skirts 0x140b34f80）【coregap-1：split_height_into_skirts 已落库】【reviewer-wave3-tg-tr 2026-10-17：裙层两固定环 {0.0, cache+0xc} 环序 + split 表在前判等（fresh 行 567-569/319-321）】|
| `rotation_from_normal`（0x140c90a40）/ `Rectangle2d::orientation_mat3`（0x140c99570）| 未提供 | core-api-missing | dep-task（utils）【reviewer-wave3-tg-tr 2026-10-17：orientation_mat3（0x140c99570）fresh 判等——`Mat3::from_cols((axis.x,0,axis.y),Y,(-axis.y,0,axis.x))` 与 Rust 镜像逐字一致；rotation_from_normal（0x140c90a40）为法向→四元数（叉乘+归一+半 sqrt），Rust beam_rotation=Quat::from_mat3(orientation_mat3) 仅 I 级意图等价（输入未归一化为潜在次差），登记明确、补位后换回路径清晰】|
| `timberframe_beam_count_change_audio`（0x1421b6610）size 写面 | 判等(主体)+GAP | core-api-missing + core-type-gap | core-task（get_building_size_01 0x1417d3f10 + PlayTimberframeSound size/id 字段）|
| ConstructTimberframeCmd 首字节 bit0 / +0x1c 门控 | 未判等代理 | core-type-gap | core-task（事件契约暴露；payload 自 event+4）|
| `GameClock` frame_count 布局（+0x2c vs 字段 @0x28）| GAP | core-type-gap | core-task（结构体布局校正）|

### state/roof

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `calculate_roof_trigger_volumes`（0x1421a3220）| 未实现（诚实）| in-module-pure-logic | implement-in-module（细分/剖面采样/檐口/UV/噪声）|
| `ui_edit_roof`（0x141a18dd0）/ `do_ui_edit_roof`（0x141a19a90）| 未实现 | core-api-missing | core-task（WidgetParams/slider3d/box_trigger_volume + HighlightRoofCmd）|
| `ui_edit_roof_ridge_dir` 族（0x141a386b0/0x141a39490）/ `ridge_length` 族（0x1421a8210/0x1421a8c50/0x1421aa170）/ `focused`（0x1421ad440）| 未实现 | core-api-missing | core-task（TweakerView/project_cursor_onto_ray/CameraMatrices::eye_right/xz_rotation_bad + RoofRidgeDims::to_world_space + SignifierFactory）|
| `highlight_roof`（0x141a284b0/0x141a31280）| 重建（wave3 落库）| core-type-gap | core-task（HighlightRoofCmd + HighlightColor）【coregap-1：两类型已落库】【reviewer-wave3-tg-tr 2026-10-17：**重建核验通过**——tag=0 清位 + 逐事件（stride 0x18 wall@+8/color@+0x10）value←wall 低 u32、tag←color、to_rgb→EditingHighlight.color 数据流判等；8B 写覆盖 {value,pad} pad 不可表示 I 级诚实；iter 变体（0x141a284b0）读出偏移不同以 store（0x141a31280）为准登记准确】|
| `gpu_flush_buffers`（0x140de50a0）| 未实现（逻辑全可观测，~5 行）| core-type-gap | core-task（GraphicsDeviceResource）【coregap-1：GraphicsDeviceResource 已落库】|
| `switch_roof_type`（0x1421ac4f0）mesh ops | 部分重建+未实现 | core-type-gap | core-task（NaniBundle/DrawableMeshBundle/try_get_handle_by_name/Handle\<Vao\> 渲染边界）|
| RoofSound 载荷 / RoofTileSimulation 数组 / GameClock.delta | 边界登记 | core-type-gap | core-task（RoofSound 字段布局 / stride-0x18 数组模型 / 0x24 vs +0x28）【reviewer-wave3-tg-tr 2026-10-17：simulate_roof_tile_for_audio（0x1421a7090）**判等**（fresh：over=cur/dt<=-120、mix 掩码、angle'=dt*-45.126003+angle、decay=expf(dt*-13.300004)、new_h=(1-decay)*target+decay*mixed、kind5/kind4/accum 门控逐语句一致；**wave3 修正 angle' 三处属实**——panic/钳制用 angle'、accum 门用最终角）；RoofSound 载荷字段面 I 级登记成立】|

### state/color

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `init_color_ids_buffer`（0x142290750）SSBO add_new×4 + `set_wall/gate/halftimber_color_id` SSBO 段 + `write_roof_color_id_to_gpu`（0x142298520）| 重建+未实现（SSBO 段阻塞）| core-type-gap | core-task（Wall/Gate/Halftimber/RoofColorIdSsbo + SsboType + GraphicsDeviceResource）|
| `set_flag_connection_and_chimney_smoke`（0x142298970）烟值 +4 | 重建+未实现 | core-type-gap | core-task（ChimneyExtra 4B 需建模 +4）|
| `set_roof_customization`（0x142299dd0）| 未实现 | core-type-gap | core-task（PublicWalls 稀疏位集/分块密集表 stride 0x14/0xa0/0x48；外环第二写）【coregap-1：待证——访问面已解，表内嵌 InnerWalls/RoofShape（walls 组在途域），见 §1 state/color】|
| `color_menu`（0x142291230）| 未实现（登记准确/诚实）| decompile-hardpoint | **已攻克（2026-10-17）**：ReVa write worker fresh clone + 600s 定点 → 136KB 全量反编译（shared DB codelen=0 为 overlay 损坏）；menu state 0..0x15 switch + WidgetParams/UiState/History（RE-ANALYSIS §2.1）【reviewer-wave2-color 2026-10-17：未实现（登记准确/诚实）——主 switch case 表 0/1/2/4/5/7/8/0xc/0xd/0xe..0x15 + History tag 3/5/8/4/9/0xd/7 + Events 写者族 + ~12 参 ABI 与 revieww2color fresh 反编译一致；Rust 保持读取面不构造近似控制流】|
| `show_color_options`（0x14229a7f0）| 重建+未实现 → **需修（case 0 图标载荷）**| decompile-hardpoint | **已攻克（2026-10-17）**：ReVa write worker + 600s → 267KB 全量反编译；kind 0..0x26 选项构建 + DecoratorStorage/DecoratorExtraInfo 分派（RE-ANALYSIS §2.2）【reviewer-wave2-color 2026-10-17：**需修**——结构骨架判等（事件遍历 stride 0x20、两段跳表 0x14303ca70 entry0→0x14229b725、is_flat 门、create+spawn、事件写面），但 case 0 图标载荷登记失准：非 demo 主路径首图标=bricks_beige(kind 3) 非分隔符、bricks_* 为 kind 3 非 0xc、roof_dark/roof_grey 属 case 1(0x14229bde9) 非 case 0、plaster_peel 为 low/medium/high 三项且条件追加、缺主 kind-3 色块与 halftimber 条件块——Rust build_plaster_wall_roof_options 为近似体；应按 fresh 反编译改 case 0 或降级 GAP】【reviewer-B-ecstate 2026-10-17 post-fix（fresh 反汇编，Ghidra/ReVa 反编译超时）：case0 13 swatch/plaster_peel low/medium/high/halftimber(kind4,b19 2/1/0)/case1 11 roof 主载荷 **基本 verified**；**新 FIX**：plaster swatch 副值 `+0x24`=0,7,1,6,4,5,2,3 未写（Rust 只落 v1c）；case2..0x26/demo/门源（get_instruction、+600、eye offset）GAP → 维持 B】|
| `color_button_animations`（0x1422979b0）/ `delete_color_icon_entities`（0x142297ff0）/ `hide_color_menu`（0x1422adcf0）/ `autoclose_color_menu`（0x142298d30）/ `ColorRadialMenu::create`（0x1425f27c0）/ `color_cursor_icon`（0x141c9b510）/ `ui_color_wall_and_roof`（0x141c9ab70）/ `ui_color_gates`（0x1422afe70）/ `ui_color_windows_and_flags`（0x1422b0e00）| 未实现（typed-gap）| core-type-gap | core-task（CustomizationIcon / Ui2dExtent / Ui2dImage / UiInactive / RaycastWorldResult +0x1d8 / ActionSetStates + 图标目录）|
| `replay_color`（0x141c7f270）| 未实现 | core-type-gap | core-task（ReplayBuffer/ReplayCommand/DecoratorHistoryEdit 原始字节布局）|

### state/decorator

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `maintain_dormer_tiles`（0x1407ce920）| 未实现 | core-type-gap | core-task（RawTable::remove_entry\<u64,Box\<dyn FnOnce\>\> + drop SparseInstanceBufferAllocation\<RoofTileSsbo\>，rhapsody 类型）【reviewer-wave3-toolbar-decorator 2026-10-17：cursor/ids/遍历/回写表达式判等；**游标写序需修**——二进制循环前写 cursor=total-lenB'-lenA' 后每瓦+1→末值=total，Rust 循环后写-end→末值=total-(lenA'+lenB')】【reviewer-B-ecstate 2026-10-17 post-fix：fresh 反编译逐语句复核（预写/每瓦+1→末值=total、id 单槽 store、hash_one+remove_entry+drop）**判等**，维持 C】|
| `add_decorator_visual_entity` impl$5（0x140811c30）| 未实现 | core-type-gap | core-task（autoclutter 表 Vec\<DisplayChildAutoClutterInfo\>）|
| `apply_decorator_op`（0x141276e30）叶 | 未实现-需重做 | core-type-gap | core-task（archive_current_decorators_in_storage 缺口 / DecoratorLocator / build_op_event 0x88B）|
| `create_or_restore_decorator_backup`（0x1408238a0）辅助 map | 未实现-需重做 | in-module-pure-logic | implement-in-module（0x30B 三 map 反向拷贝；+0x48 门控已复刻）|
| `decorator_record_history`（0x140817850）48B 载荷 | 未实现-需重做 | core-type-gap | core-task（48B {1,1,kind,f1,f2,f3} 载荷与 DecoratorHistoryEdit 存档模型）|
| `window_lights`（0x1407b5980）| 未实现-需重做 | in-module-pure-logic | implement-in-module（墙/屋顶收集主循环；形状变化写；rng 种子/retain 待证）【reviewer-wave3-toolbar-decorator 2026-10-17：常量 0.51/0.44/0.72、灯态变化、retain 次序（帧末前）、帧末计时判等；收集主循环 U 诚实；帧末 NaN 检查 Rust 遗漏补登记】|
| `chimney_smoke_anim_thickness`（0x140818df0）| 重建+未实现 | decompile-hardpoint | **已攻克（2026-10-17）**：更新公式还原 `new=clamp01(old+dt/target)`，target=lerp(1,3) 或 lerp(2,4)；divisor 即目标值字节重拼；ReVa write 全量 31KB（RE-ANALYSIS §3）|
| ~146 业务函数（blueprint/cull_oob/merge/visual 族/stairs/ui_systems）| 未实现（未 fresh 复核）| in-module-pure-logic | implement-in-module（按族分批 fresh 后重建）|
| `clear_out_visual_entities`/`clear_out_wall_holes`/`maintain_dormer_tiles`/`decorator_record_history`/`create_or_restore_decorator_backup` | **4/5 已接线（2026-10-17）**| integration-registration | integration（`add_decorator_systems<S,C>` 已注册 clear_out_wall_holes/clear_out_visual_entities/decorator_record_history/create_or_restore_decorator_backup；maintain_dormer_tiles 待 rhapsody 注册表落库后重建为系统）|
| `replay_decorator`（0x14080c8d0）| 未实现 | core-type-gap | core-task（core `ReplayCommand` Decorator 变体 + `ReplayBucket` Arc\<[DecoratorId]\> cull-allow 表字段 + 0x30B 内嵌时序）【reviewer-wave3-toolbar-decorator 2026-10-17：结构判等——DURATION_FLOOR **0.05**（fresh `if (fVar22<=0.05) fVar22=0.05`）、Arc\<[DecoratorId]\> cull 表、tag-7 扫描、flags bit0 立即完成、`max(duration/speed,0.05)<elapsed` 完成、elapsed+=delta、反序 Vec::remove + tag 释放跳表全对上；core-gap 登记准确；Rust 诚实壳】|
| `WindowVisualEntityBuilder::build<Transient/Persistent>`（0x1412ab270/0x1412a9b30 等 6 实例）| 未实现-需重做（近似体）| core-type-gap | core-task（NaniBundle/DrawableMeshBundle 落库 + 构建器字段语义）【reviewer-wave3-toolbar-decorator 2026-10-17：mesh 名分派 12 类 NaniBundle + 默认 DrawableMeshBundle、条件组件链 14 位、build\<Persistent\>→impl$4 / build\<Transient\>→impl$5 全判等；近似占位降级诚实；build_transient 目标分歧登记准确】|

### interaction/clutter

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `populate_autoclutter_regions`（0x1422d6ed0）| 未实现 | decompile-hardpoint | re-analysis（44KB 碎片 16 段；跳表 0x14304382c/0x143043818）|
| `add_autoclutter_on_windows`（0x1422cd7d0）| 未实现 | decompile-hardpoint | re-analysis（Ghidra bad instruction data，radare2 全反汇编）|
| `manage_autoclutter_entities`（0x1422d04e0）| 未实现 | in-module-pure-logic | implement-in-module（宿主查询 +0xf8..+0x120 逐偏移/四元数/update_shutter）|
| `read_bush_positions_from_gpu`（0x140eebaa0）| 未实现 | core-api-missing | core-task（FrozenMap\<SsboId,Box\<BufferState\>\>::get GPU 回读）|
| `load_clutter_meshes`（0x1422c0760）| 未实现 | re-analysis | 补证（50 个网格名/资产 id 清单；现 56 路径表为 I 推断）|
| `autoclutter_animation`（0x1422ccef0）/ `add_autoclutter_around_gates`（0x1422cc390）/ `around_windows`（0x1422cefe0）/ `add_preplaced_autoclutter`（0x141ce5900）/ `add_tree_setdressing`（0x1422d3800）| 未实现 | in-module-pure-logic | implement-in-module（Easing 曲线/四元数/变更检测/门控序列）|
| `construct_inter_roof_merlons`（0x1422d4870）/ `spawn_stitch_bricks`（0x141ce7190）| 未实现 | core-type-gap | core-task（BrickTransformSsbo 字段 / SsboId 烘焙）|
| `request_inter_roof_merlon_audio`（0x1422d4230）/ `detect_inter_roof_merlons`（0x141ce3a70）/ `detect_intra_shape_corners`（0x141cd1a90）/ `add_hole_at_shape_intersection`（0x141cd3ce0）/ `spawn_stitches`（0x141ce65b0）| 未实现 | in-module-pure-logic | implement-in-module（indexmap Bucket kind 8/9 / intersect_shapes / WallSpaceCoord 链 / 四元数）|
| `process_autoclutter_candidates`（0x1422ce260）| 未实现-需重做 | re-analysis | 补证（closure_env_0 过滤谓词 / closure_env_1 排序键 / case5 key）|
| `autoclutter_subscribe_to_tabloids`（0x1422d6930）| 未实现(部分) | core-api-missing | core-task（RaycastWorld::subscribe_to_changes×8 / WorldRaster::subscribe\<512\>×3）|
| `set_current_tick`/`reset_autoclutter_for_new_frame`/`remove_region_from_lazy_grid` | 未实现-需重做 | core-type-gap | core-task（PDB 系统签名 Res\<GameTime\>,ResMut\<AutoClutterState\>,ResMut\<InterShapeBrickStitches\>；GlobalId 删除键）|
| `add_region`（0x140ee3fd0）/`mark_dirty`（0x140ee48c0）/`flush_candidates_pending_removal`（0x140ee4bf0）kind==4 Arc drop + 布局 | 判等-附U分歧 | core-type-gap | core-task（Arc\<[Vec3]\> payload 合同；AutoClutterState/Region/Spawn/Candidate 布局）|

### interaction/onboarding

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `hint_orchestrator`（0x14227fc30）| 未实现 | core-type-gap | core-task（GameProfile 完成标志 + DisplayHintRequest 布局 + closure$0 排序键）|
| `resolve_steam_impl`（0x14227e5e0）| 未实现 | core-type-gap | core-task（SteamInput::get_action_origin_glyphs）|
| `hint_highlight_shape`（0x141c50120）| 未实现 | core-type-gap | core-task（HighlightRoofCmd/HighlightWallCmd）|
| `hint_position_update`（0x141c50510）| 未实现 | decompile-hardpoint | re-analysis（跳转表未恢复 + 相机矩阵/几何接线）|
| `hint_draw`（0x141c51a60）+ closure$2（0x14228c260）+ `render_hints`（0x141c52700）+ closure$1/2/4 | 未实现 | core-api-missing | core-task（mei DSL / toolbar-mei 未落库）|
| `exponential_back_off`（0x142284800）+ 2 inline | 未实现 | core-type-gap | core-task（GameProfile 计时区 + WriteSettingsFileCmd）|
| `hints_startup`（0x14227b450）/ `reset_hints`（0x141c1c8d0）/ `tutorial_completion`（0x14227c230）| 未实现 | core-type-gap | core-task（GameProfile 0x1f4..0x206 + WriteSettingsFileCmd/SaveGameProfileCmd）|
| 相机 5（enable 0x14227c780 / display 0x14227c910 / pan 0x14227d000 / rotate 0x14227d2f0 / zoom 0x14227d680）| 未实现 | core-type-gap | core-task（GameProfile 0x1bc/0x1c0 字形 + NotifyHintSystemCameraMoved + SteamInput + lang）|
| focused/clutter 5 + roof 2 + `make_focus_or_camera_binding_hint`（0x141c3de00）/`make_lbm_drag_binding_hint`（0x142285270）| 未实现 | core-type-gap + core-api-missing | core-task（geo_src_to_edit_subject / geo_src_w_deco_clutter + ModeJustChanged 载荷 + profile 0x1f5/0x200/0x1ff/0x1c4）|
| stairs（0x142285a20）/ terrain×3 / `hint_undo`（0x142284220）/ `hint_unlink_windows`（0x14227d760）/ `hint_deselect_tool`（0x1422853c0）/ `hint_disable_snapping`（0x14227e140）/ `prompt_rollercoaster`（0x14227bb70）| 未实现 | core-type-gap | core-task（GameProfile 0x1f4..0x205 + 6×NotifyHintSystem\*Stairs\* + DisplayRollercoasterPrompt）|
| `hint_focused_edit_hint`（0x14227dd60）| 未实现 | decompile-hardpoint | re-analysis（跳转表 DAT_1430386a0 + geo_src 链）|

### interaction/wall-constructor

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `ArchWalker::walk` 族 4 单态（0x141232bb0/0x141233380/0x141231ea0/0x14048ab50）| 未实现-需重做（分歧已证）| in-module-pure-logic | implement-in-module（travelled 推进/周期 u/尾砖条件；wall_space subtract+project_clamp）|
| 支撑梁 5 函数（place_support_beams 0x141242530 / add_diagonal_support 0x141243740 / emit_support_beams 0x141243c20 / SupportBeamGrower::propose 0x141fe09c0 / construct_platform_stairs 0x1407744d0）| 未实现（零壳）| in-module-pure-logic | implement-in-module（双 Grower/cost=len·(‖Δdir‖+.5)/rdp/96B 砖链）|
| segmentation 3 函数（process_segment 0x140750420 / add_splits_in_empty_range 0x140751240 / score_split_candidate 0x140751610）| 未实现（近似体）| in-module-pure-logic | implement-in-module（score_split_candidate>0.4 / floor(len/span) / 2 单位弦）|
| 立面支撑整族（circle_categorize 0x14124af60 / construct_circle\* 0x14124ba30..0x14124eb80 / rect\* 0x14124fcb0..0x141253fc0 / find_stone_column_start_y 0x141250720 / place_pillars 0x140753670 / PillarProposals::add_proposal 0x1407534d0 / assemble_stone_pillar 0x14120d670 等）| 未实现（跳表已攻克）| in-module-pure-logic | implement-in-module（find_stone_column_start_y 跳表解码已核；残留 7 参 ABI/helper 语义）|
| 门/栅栏/地板/楼梯族（construct_gates 0x140756370 / construct_wooden_gate_doors 0x141210110 / construct_fences 0x140765830 / emit_fences 0x1412388f0 / construct_floors 0x1407661e0 / playermade_stairs 族）| 未实现 | in-module-pure-logic | implement-in-module（G06 冻结链接线 + 未冻结分支）|
| 半木/抹灰族（curved_halftimber 0x1407706d0 / wooden_floor_borders 0x141244310 / create_plaster 0x14075d460 / maintain_plaster 0x14075b810 / split_wall_into_sides 0x14075dc60 / generate_plaster_mesh 0x1412269b0 / roof_outline 0x141217be0）| 未实现 | decompile-hardpoint + in-module | half_timbering 走 re-analysis；其余 implement-in-module |
| utils 近似体族（stairs_signifiers_new 0x141223d10 / add_wall_corners 0x141215540 / add_skirts 0x1412221f0 / overlapping_windows 0x1412218a0 / get_window_aabb 0x141221da0 / check_for_overlap 0x141221f70 / add_underwindow_pattern 0x141fd1c30 / wrap_around）| 未实现（近似体）| in-module-pure-logic | implement-in-module（角检测 ±10° / split_height_into_skirts / SIMD 掩码遍历）|
| `sine_wave`（BrickRowSine，module 401 inlinee 24627/24641/24629）| 未实现 | decompile-hardpoint | re-analysis（无独立编译实例，需从 from_curve 体内按行号提取）|
| `half_timbering`（0x141218e30）| 未实现 | decompile-hardpoint | re-analysis（见 §4）|
| `process_playermade_stair_pillars_proposals`（宿主 0x140178a70）| 未实现 | decompile-hardpoint | re-analysis（bevy impl$3::call_mut 系统包装，内层业务块未定点）|
| `construct_playermade_stairs_arches_n_walls<true>`（0x140496800）| 未实现 | decompile-hardpoint | re-analysis（两泛型 \<true\>/\<false\> 翻转 + closure 绑定）|
| `DoorMeshInProgress::add_quad`（0x141214ed0）| G06 复用覆盖 | core-type-gap | core-task（原 3 数组 vs country-core 4 数组布局差异）|
| `gpu_clear_wall_gate_pieces`（0x140773c00）/ `gpu_flush_plank_buffers`（0x140774010）| 未实现 | core-api-missing | core-task（render 边界接线，后置）|
| `maintain_stone_column_colliders`（0x14078a6e0）| 未实现 | core-api-missing | core-task（rapier3d 0.22.0 已入 workspace；country-core native-physics 后端暴露）|
| `resolve_hole_overlap` case0 split_x 守卫残留 | 判等+残留 | re-analysis | 补定向冻结用例（split_x==1&&split_y==0&&Yoverlap&&!Xoverlap）|

### interaction/toolbar

| 函数/条目 | 状态 | 根因 | 动作建议 |
|---|---|---|---|
| `MeiContext::mei_pre_update`（0x1406842e0）/ `mei_post_update`（0x1406884c0）| 未实现 | in-module-pure-logic | implement-in-module（输入→handler 状态机 / render 两遍+taffy 最小尺寸）|
| widgets 构建器/视图/tweaker 族/`decorate_hovered`（0x14067df20）| 未实现 | core-type-gap + in-module | core-task（Label/MenuButton/Image/Pill/Frame/TweakerContent 内容类型落库）|
| taffy 族（TaffyLayout::style/perform_layout 0x141e4c650/0x141e4c6c0 / make_layout_absolute 0x142374e10 / taffy_mut_style 10 单态 / taffy_update_drawables 0x142340d20 / taffy_finalize_element_size 0x142340890）+ toolbar_systems 族 | 未实现 | external-dep | dep-task（taffy 0.3.13 入 workspace）|
| `toolbar_auto_hide`/`toolbar_alignment`/`undo_redo_alignment`/`undo_redo_auto_hide`/`toolbar_steal_mouse_input`（0x142343a10..0x142344df0）/ `ui_sounds`（0x14233a7f0 真地址）| 未实现 | decompile-hardpoint | re-analysis（jumptable 破坏）【reviewer-wave3-toolbar-decorator 2026-10-17：分派跳表 20 case 判等；**toolbar_alignment 前置掩码缺显式 `& 3`**（二进制 `0x1020300 >> ((alignment&3)<<3)`，Rust `>> (alignment<<3)`——release 行为等价、debug alignment≥4 会 panic，建议保真改）；ui_sounds 真地址 0x14233a7f0、switch ~18 case 截断复验】|
| `toolbar_button` 族（toolbar_button_keep_focus 0x1423414b0 / switch_app_mode_button 0x142341fe0 / switch_toolbar_category_button 0x1423427f0 / tod_v2_button 0x142342f30）+ `ui_animation_state_update`（0x14233b3f0）| 重建（近似）+分歧 | core-type-gap | core-task（per-widget stride-3 字节记录容器 + EventWriter 载荷面，见 §1）【reviewer-wave3-toolbar-decorator 2026-10-17：keep_focus keep 字节以 `cooldown>0` 代理非判等、keep 分支二进制写 0.3、步骤 3/4 未实现；**switch_app_mode 判定过强→未实现**（Rust body 空壳）；switch_category 动画循环漏 ModeJustChanged 存在门+未选中门；tod_v2 toggle/insert 未按 widget 命中门控；ui_animation_state_update 公式判等但 state 字节（+0xc）写入未实现未登记】|
| `render_taffy_ui_to_widgets`（0x142338990）| 未实现 | core-type-gap | core-task（13 参接线 + WidgetParams→UiState 写入链）【reviewer-wave3-toolbar-decorator 2026-10-17：Query init 面 `(Entity,&Ui2dExtent,&UiMouseInteractable,&TaffyNode,&mut UiInteractionState,Option<&HistoryButton>,Option<&UiNav>)` + WidgetParams 链 + grow_one<EventInstance<WindowFocusEvent>>/grow_one<(ref str,SettingsMenuState)> + format_inner 全对上，接线未实现诚实】|
| `place_childen_impl`（0x140fe12a0/0x140fe19a0）/ `add_toolbar_item`（0x141e4dc40）| 未实现 | decompile-hardpoint | re-analysis（跳转表 0x142bc223c/0x142bc2260 坏分支 / DAT_142f8f774）|
| photomode 族（do_photomode 0x14234b010 / ui_slider 0x1423486d0 等）/ lut_gallery | 未实现 | decompile-hardpoint + core-api-missing | re-analysis（滑块跳表截断分支 + MainCamera/BrushSizes 接线）|
| menu/load/corrupted/credits/report_bug 族 / `SingleFileServer::http`（0x14234d690）/`run`（0x14286bf40）| 未实现 | external-dep + core-type-gap | dep-task（tiny_http 0.12.0 / image 0.25.1 / open/webbrowser/hotwatch；SteamClient/GameProfile/mei DSL）|
| text 渲染族（text_startup_system 0x141e9ce50 等 5 函数）| 未实现 | core-type-gap | core-task（tg-text TextRenderer 接线 + SSBO 名）|
| 26× `register_required_components` + GlobalId/UiNav/TaffyNode/UiAnimationState/UiPlay\*Animation/WindowFocusEvent/WriteSettingsFileCmd/ToolbarSizeMult/UiInteractionState | 未实现（类型未落库）| core-type-gap | core-task（共享 UI 类型移入 country-core）【reviewer-wave3-toolbar-decorator 2026-10-17：UiAnimationState 16B（appear@0/select@4/state@0xc）与 UiInteractionState 4B 布局判等校正，toolbar 本地类型已落】|
| `utils::calculate_hash`（0x1405bdda0，零键 Sip13）| 镜像（未落库）| core-api-missing | dep-task（utils crate 落库后移镜像）【coregap-1：已 re-export 到 `country_core::utils`】【reviewer-coregap-1 2026-10-17：判等——fresh 0x1405bdda0 = Sip13Rounds write×2（std DefaultHasher SipHash-1-3）】|
| `ToolbarPlugin`/`Ui2dPlugin` 完整 build（0x14068d610/0x14068dc70）| 未实现（9 事件+系统集合）| integration-registration | integration（系统恢复后接线）|

---

## §1 core-type-gap —— 缺 country-core 类型/字段（core 团队可直接领）

> 每条 = 所需具体类型/符号（含地址/布局，O 面），落库后对应 in-module 函数即可重建。

### eco/glade-border
- **`SignifierFactory`**（0x14089d8b0）+ closure 槽位（vtable 0x142efa150）+ closure$1 唤醒——display perimeter 写路径、closure$1。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等】类型已落 `systems/ui/signifiers/signifier_factory.rs`（40B `{len,ptr,len,&inner,marker}`）；closure 槽位/唤醒仍 GAP。
- **`UiSignifierStream::write`**（0x140b2cf90，5×qword 槽）——display perimeter 载荷。【既有】
- **`SignifierEntityBuilder`** 链（new 0x14089d9b0 → set_ui_pass 0x14089dad0 → animation_duration 0x14089dc90 → custom_shader 0x14089db00 → build 0x14089dd50）——closure$1 / water closure$0 / color_cursor_icon。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：**需修** ①new 未拆 48B 载荷首 qword=mesh_kind（build switch 键）②default 未装默认 shader "vertex_color_shader"（len 0x13）③custom_shader 未释放旧串；set_ui_pass/animation_duration 判等、build 组件入队 GAP 诚实】链已落库；build 组件入队 GAP。
- **`widget_params::PlayUiSound` 变体裁决**——变体 #4（判别值 4、0x20B 步长）；现为单元结构，变体不可表达。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等】0x20B `{kind u32@+0, payload, id u64@+0x18}` + `variant4(id)`。

### eco/forest
- **`InteractableForestTree`** 扩 0x50B：{Arc\<String\> prefab@0, u64 打包 pos@0x10, f32 高@0x14, f32 z@0x18, u32 prefab 索引@0x1c, …entity@0x40, u8 有树标志@0x44}——manage 记录实体回写 + forest_tree +0x44 清标志。
- **`SpawnPrefabCmd`** 扩 stride 0x50 载荷（现 `_gap`）。
- **`DespawnPrefabCmd`** 载荷形状（8B Entity vs Option）。
- **`ConstructTimberframeCmd`** 载荷 **ShapeParameters**（0x28B）——manage 构造形状子判定源。
- **`Shape`/`Rectangle2d::closest_point_on_perimeter`** 合同——is_too_close_to_shape 矩形分派。
- **`TreeAnimation`** stride 0x10 布局 {时长@0, 进度@+4, 淡出@+8, 标志@+0xc}——handplaced +4 进度位。
- **`PrefabInstanceData`** stride 0x30（写 +0x20）——handplaced/forest_tree 写面。

### eco/tree-tool
- **`MoveTreeCmd`** 槽 0x20 = {id@+0, index u64@+8, position Vec3@+0x10}——process Move / replay Move / ui_move_tree 载荷。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh 消费(0x142267430 stride 0x20)+发射(0x14226b1a0)双证】`resources/tree_storage.rs`。
- **`InsertTreeCmd`** 槽 0x48 = {name String@+0, id@+0x38, position/scale/rotation/variant/flags/height 中段}——process Insert / place_tree / replay_tree 重建。【coregap-1 冲突跳过】【reviewer-coregap-1 2026-10-17：冲突登记准确——fresh `place_tree` 0x142269970 事件写块 stride 0x48 逐字段核对合同，tree_edit.rs 现由 history 组持有】`resources/history/tree_edit.rs` 正被他组（history 组）在途编辑，未触碰；布局合同已从 fresh `place_tree` 0x142269970 解出（stride 0x48：{name@+0, position@+0x10, scale@+0x20, {variant,rotation}@+0x28, flags@+0x30, id@+0x38, height@+0x40}），供后续照落。
- **`PrefabInstanceData`** 补 +0x24/+0x28 step 字段——tree_move_bend 变更写面（GPU 同步）。
- **`ReplayCommand`/`ReplayBucket`** 补 elapsed@+0x18 / delay@+0xc / flags@+8 / speed 时序——replay_tree。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：CommandTiming 偏移判等；**advance_timing 需修**——位极性反转，见 REVIEW §2】`ReplayBucket.timing: Vec<CommandTiming>` + `advance_timing`（`resources/history/replay_cmd_buffer.rs`）。
- **`TreeAnimation`** 浮点 α@+4（vs 共享型 +0）——布局分歧 5，despawn 字节级不等。
- **`History`** 支持 **`insert_starting_edit`**（ObservedHistoryLedger vs HistoryTransactions 不兼容）——convert 转换链。
- **`PathDrawEvent`** flag@+0x18 字段——ui_delete_tree 中心/半径源。
- **`HideTreesinAreaCmd`/`HideTreesNextToWallCmd`** 补 shape 字节（@+0 bit0：0=圆/1=矩形，`with_expanded_diameter/size`）+ @+0x24 门控字节；**树级 `Decorators` 候选源**（`iter_removed_ids`）系统签名接线——hide_trees（2026-10-17 转，fresh 证据 `work/purelogic-integration/evidence/0x14226a260_hide_trees.c`）。

### eco/critters
- **`CritterGoal`** 字节序（B active@0,pos@4 vs 现 {position,goal_tag}）——桥接 DIVERGE。
- **`terrain_rock_mask::sample_at_world_xz`** 返回 u32（vs bool）。
- **`WrenBoids`/`FishBoids(Resource)`/`CritterControl`** 类型缺失（`FishBoids(Resource)` 阻塞 despawn_fish 0x140f90c10 资源重置——2026-10-17 转 core-type-gap；另需 `OnExit<AppLoadingState>` bevy_state 见 §3）。
- **`Ui2dExtent`/`Ui2dImage`**——sheep emoji / color 菜单。

### state/water
- **`SignifierFactory`** 40B 记录布局 {name_len,name_ptr,name_len,&inner,marker} vs core **`UiAppendSignifier`**（name:u64 无法承载串）——橡皮 signifier 复核。
- **`WaterAudioGrid`** 24B 单元 {f32 x, f32 z, u64 窗@+8, u16×2 计数@+0x10} vs 语义模型 {water_level,position}——音频域裁决。

### state/timberframe
- **`WoodenPlankSsbo`** 的 **SparseSsbo/SsboType**——梁写路径。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh 0x141a41ba0 `push<WoodenPlankSsbo>`/`SparseInstanceBufferAllocation<WoodenPlankSsbo>`+AssetSsboLibrary memcmp 全名；Storage=SparseInstanceBuffer<Self> 成立；CHUNK_COUNT=0x800 为文档交叉引用】`resources/roofs/roof_buffers.rs`：元素 0x20B `{position, size, rotation}`（字段序 I 登记）+ `SparseSsbo CHUNK_COUNT=0x800` + `SsboType Storage=SparseInstanceBuffer<Self>`。
- **`render::RenderInstance`** 记录（country-core 未提供）。【coregap-1 待证】梁实例 0x20B 记录以 `WoodenPlankSsbo` 元素承载；`RenderInstance` 独立类型未立（字段序 U）。
- **`resources::walls::split_height_into_skirts`**（0x140b34f80）——裙层环数 N。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh：`height≥5.9 → push 4.5，f+=3.5 至 height≤f+1.4`，cap 固定 4】`resources/walls/mod.rs`：`height≥5.9 → [4.5, 8.0, 11.5, …]`。
- **`PlayTimberframeSound`** 补 size/id 字段（真实 0x20B {kind@0,pos@+8,size@+0x10,id@+0x18}）。
- **`ConstructTimberframeCmd`** 暴露首字节 bit0（rect 处理 1 / circle 处理 0）+ 0x1c 字节（payload 自 event+4）。
- **`GameClock`** 布局校正（frame_count 字段 @0x28、二进制读 +0x2c；结构体 doc 内部不一致）。
- **`AppLoadingState`**（bevy_state）——OnExit 接线。

### state/roof
- **`HighlightRoofCmd` + `HighlightColor`**（resources::roofs::commands / walls）。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh 0x140b34f30 to_rgb（==1→{0.1,0.03,0} else {0.07,0.08,0.07}）+ 0x141a31280 事件 stride 0x18 {wall@+8,color u8@+0x10}】`HighlightColor(u8)`+`to_rgb` 落 `resources/walls/mod.rs`；`HighlightRoofCmd`（stride 0x18：wall@+8、color@+0x10）落 `resources/roofs/commands.rs`。
- **`GraphicsDeviceResource`**（render）——gpu_flush_buffers。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh 0x140de50a0 系统签名 Res\<GraphicsDeviceResource\>→Deref→gpu_flush\<RoofTileSsbo\>；+0x188 管线缓存槽登记 GAP】`render/device.rs`：持 rhapsody 设备 + `Deref`；+0x188 管线缓存槽登记 GAP。
- **`RoofSound`** 事件载荷字段布局（entity/wall/pivot/flat_surface 精确偏移）。
- **`RoofTileSimulation`** stride-0x18 记录数组（vs 单 24B）。
- **`GameClock.delta`** 0x24 vs 二进制 +0x28。

### state/color
- **`WallColorIdSsbo`/`GateColorIdSsbo`/`HalftimberColorIdSsbo`/`RoofColorIdSsbo`** + **`SsboType`**。— **✅ coregap-2 2026-10-17 实现**（color_id.rs 墙/门/半木 + color_id_roof.rs 屋顶；StorageElement NAME=复数槽名 + BUFFER_LEN=20480 + LibrarySsbo/SsboType；SsboId 命中着色器立即数 0xaf42315d263aa22a/0x8cce418b17d89f9b，memcmp 长度 0x32/0x32/0x38/0x37 与各 set_*/write_roof 断言一致；render_ssbo.rs 占位移除）。
- **`GraphicsDeviceResource`**。【coregap-1 已实现】见 state/roof。
- **`ChimneyExtra`** 4B 建模 +4 字段（现 `{pub f32}` 无 +4）。— **✅ coregap-2 实现**（decorator.rs `{f,f32; smoke,u8}`；set_flag_connection_and_chimney_smoke `as_chimney` 写 +4）。
- **事件载荷族**（装饰/窗族 SetDecoratorColorId/SetWindowLight/SetLadderColorId/SetMagicColorId/SetDecoratorBrightness = `{id u32, value u32, custom u8}` **无 gap** 0x18B；墙族 SetWall/Gate/HalftimberColorIdCmd = `{id u32, +4 U, color u32, custom u8}` 0x18B；SetStairRailing/SetFlagConnection/SetChimneySmoke = `{id u32, value u8, custom u8}` 0x10B）。— **✅ coregap-2 2026-10-17 实现 + reviewer §3 已修**：删装饰/窗族 `_gap4`（消费面 set_window_light 0x142298260 / set_decorator_color_id 0x1422af0c0 读 value@+0xc/custom@+0x10）、按二进制 register_event 名改名 SetLadderColorIdCmd/SetMagicColorIdCmd/SetDecoratorBrightnessCmd；模块域 system_color 登记不迁移。单测 8×0x10B 断言过。
- **`PublicWalls`** 稀疏位集/分块密集表（stride 0x14/0xa0/0x48、`~idx*0x30`）——set_roof_customization。【coregap-1 待证】fresh `set_roof_customization` 0x142299dd0 已解访问面（墙记录表 stride 0x48、屋顶位集、屋顶记录 stride 0x30、色表=记录+0x28、色值源=Res<PublicWalls>+0x14），但表内嵌 `InnerWalls`/`RoofShape`（walls 组在途域）——未落库（对齐 color RESTORE FIX-6）。
- **`CustomizationIcon`/`Ui2dExtent`/`Ui2dImage`/`UiInactive`**。
- **`RaycastWorldResult`** +0x1d8 命中类型字段面。
- **`ActionSetStates`** + 图标目录（hide_color_menu / autoclose）。
- **`ReplayBuffer`/`ReplayCommand`/`DecoratorHistoryEdit`** 原始字节布局（replay_color 13 写者）。

### state/decorator
- **rhapsody 回调注册表**（RawTable\<u64,Box\<dyn FnOnce\>\>）——maintain_dormer_tiles。
- **`SparseInstanceBufferAllocation<RoofTileSsbo>`** drop 路径。
- **autoclutter 表 `Vec<DisplayChildAutoClutterInfo>`**——impl$5。
- **`ReplayCommand` Decorator 变体 + `ReplayBucket` Arc\<[DecoratorId]\> cull-allow 表字段 + 0x30B 命令内嵌时序**——replay_decorator。【reviewer-wave3-toolbar-decorator 2026-10-17 新登记，fresh 0x14080c8d0 全结构判等】
- **`archive_current_decorators_in_storage`**（archivist 缺口）——apply_decorator_op 归档叶。
- **`DecoratorLocator`** 实参合同 / **`build_op_event`** 0x88B 字段放置。
- **`DecoratorHistoryEdit`** 存档模型（48B {1,1,kind,f1,f2,f3} 载荷）。

### interaction/clutter
- **`AutoClutterState`** 二进制 Vec 字段序（data/len 前置）；**`AutoClutterRegion`** 0x40B；**`AutoClutterSpawn`** 0x68；**`AutoClutterCandidate`** 0x98（+0x30 byte==6、+0x68..+0x88 Rectangle3d、+0x88 region_key）。【coregap-1 已实现】【reviewer-coregap-1 2026-10-17：判等——fresh 0x140ee3fd0 State 区域表@+0x60/pending@+0x30..0x48/epoch@+0x90、Region 0x40B（kind@+0、Arc@+8、7×f32@+0x18..+0x30、i32@+0x34、u8@+0x38/39、candidate_index@+0x3a、epoch@+0x3c）、is_equivalent 逐 7f32+i32+2u8；0x140ee4bf0 Candidate 0x98B（kind@+0==4 Arc 释放、region_key@+0x88）；Spawn 0x68 交叉引用 interaction/clutter §类型合同】`systems/ui_clutter/autoclutter_state.rs`（含 `AutoClutterRegion::is_equivalent`、pending Vec<u16>、epoch、区域表）。
- **`Arc<[Vec3]>` payload 合同**——add_region/flush kind==4 递减。
- **`BrickTransformSsbo`** 字段——construct_inter_roof_merlons。
- **`InterShapeBrickStitches`**——set_current_tick/reset 系统签名。
- **`GlobalId`**（country_core::systems::ui）——remove_region_from_lazy_grid 删除键。

### interaction/onboarding
- **`GameProfile` 提示区 0x1bc..0x206**：完成标志（0x1f4..0x206）、相机字形模式（0x1bc/0x1c0/0x1c4）、各 hint 门控位（0x1f5/0x1f9..0x1ff）、计时区（0x1e0..0x1f0）。— **✅ coregap-2 2026-10-17 实现 + reviewer §2 已修**（game_profile.rs：3×u16 字形模式 + TutorialCooldowns 5×f32 + 19 完成标志 + mark_tutorial_op_complete/set_all 判等；reset_hint_flags 已按二进制只清 0x1e0..0x1ef、不动 +0x1f0；serde 已改 serialize 写 player_id + `#[serde(alias="game_installation_id")]`、删 game_installation_id 序列化字段）。
- **`WriteSettingsFileCmd` / `SaveGameProfileCmd`**。— **✅ coregap-2 实现**（events.rs，unit；tutorial_completion fresh 反编译实际发 WriteSettingsFileCmd，RESTORE-onboarding 表 "SaveGameProfileCmd" 表述修正为同 app_config 域 I）。
- **`SteamInput`**（get_action_origin_glyphs）+ SteamInput 资源——resolve_steam/camera 字形。— **🔄 他组已覆盖**（rebuild-steam 2026-10-17 完整重建 steam.rs 含 get_action_origin_glyphs；coregap-2 冲突跳过，仅保留 resources/mod.rs 注册）。
- **18× `NotifyHintSystem*` 事件**（ShapeEditWasPerformed/CameraMoved/Snapping/Terrain/Window/ToolWasJustUsed/SomethingWasErased/TerrainSteepness/RoofShape/ClutterWasMoved/StairsPlaced/HoveringOverStairs/StartedStairsContinue/CanContinueStairs/StairsDetachedFromWall/StairsCanDetachFromWall）。— **✅ coregap-2 实现 + reviewer 判等**（onboarding.rs 16 事件与 add_onboarding register_event 序列 1/3/4/5/9..20 一致，CameraMoved {tag,value} 由 hint_camera_progress_update 消费面证实）。
- **`DisplayHintRequest` 布局**、**`DisplayRollercoasterPrompt`**、**`ModeJustChanged` 载荷**。— **DisplayRollercoasterPrompt ✅ / ModeJustChanged 载荷 ✅ coregap-2 实现**（events.rs）；**DisplayHintRequest 布局 待证**（system_onboarding::hint_orchestrator 域非 core 类型；0x50B 载荷续证）。
- **`HighlightRoofCmd`/`HighlightWallCmd`**、**`StructureType::is_fence`**。— HighlightRoofCmd/WallCmd 已落库（roofs/commands.rs + wall_highlight.rs）；**`StructureType::is_fence` ✅ coregap-2 实现**（inner_wall_state.rs，fresh 0x1408c5680 = `movzx eax,[rcx];ret`）。

### interaction/wall-constructor
- **`gates::add_quad` 3 数组布局**（positions/colors/uvs stride 0xc/0xc/8，country-core 为 4 数组含 normals）——DoorMeshInProgress 覆盖差异。

### interaction/toolbar
- **`country_core::systems::ui::GlobalId`**（现驻 toolbar::mei::core，建议移入 country-core）。
- **`UiNav`/`TaffyNode`/`UiAnimationState`/`UiPlaySelectAnimation`/`UiPlayLockedAnimation`/`WindowFocusEvent`/`WriteSettingsFileCmd`/`ToolbarSizeMult`**。【reviewer-wave3-toolbar-decorator 2026-10-17：UiAnimationState 16B / UiInteractionState 4B 布局已判等校正并落 toolbar 本地；UiNav/HistoryButton/WindowFocusEvent 仍 U】
- **per-widget stride-3 字节记录容器**（keep 字节`+1`/appear sign`+0`/选中标志`+2`；focused-window 子树表）——toolbar_button_keep_focus / ui_animation_state_update / switch_*_button 共享。【reviewer-wave3-toolbar-decorator 2026-10-17 新登记，fresh 多函数读面佐证】
- **EventWriter 载荷面分歧**：`RequestAppModeChange`/`RequestToolbarCategoryChange` 实写 `(ref str, SettingsMenuState)`（stride 0x18）/`EventInstance<WindowFocusEvent>`（stride 0x10），与签名事件类型不符。【reviewer-wave3-toolbar-decorator 2026-10-17 新登记；render_taffy_ui_to_widgets 亦写两者，佐证】
- **`Label`/`MenuButton`/`Image`/`Pill`/`Frame`/`TweakerContent`** 内容类型。
- **`FeedbackReportResources`/`cosmic_text`**。
- 26× register_required_components 对应 **26 组件类型**。

---

## §2 core-api-missing —— 缺 core 方法/接线

| 模块 | 缺失 API | 阻塞函数 |
|---|---|---|
| glade-border | `DebugLine::show()`（0x140b04f70，debug_viz show 系列，把折线压入 append-only 调试流）| display_glade_border 绘制分支 |
| forest | `utils::Aabb3::intersect_segment` 3D 版 | handplaced 相机盒扩展 |
| tree-tool | `geo_src_to_edit_subject`（country-core）| ui_place_tree / hint_focused_edit_hint |
| critters | `UiWidgetContext::intersecting`、`SignifierEntityBuilder`、`slider3d::into_drag_state` | ui_pet_sheep/ui_pet_ducks |
| critters | `PublicWalls::get`、`get_wall_shape`、`GladeBorder::is_pos_inside` | ducks_react_to_being_inside_buildings |
| water | `intersects_any` 增加 max=1.0 参数面 | update_water_audio_grid 遮挡剔除 |
| timberframe | `utils::rotation_from_normal`（0x140c90a40）、`Rectangle2d::orientation_mat3`（0x140c99570）| rect/circle 逐梁朝向 |
| timberframe | `systems::audio::building_audio::get_building_size_01`（0x1417d3f10）| timberframe_beam_count_change_audio size 写 |
| roof | WidgetParams/slider3d/`box_trigger_volume`/`project_cursor_onto_ray`/`CameraMatrices::eye_right`/`xz_rotation_bad`/`TweakerView`/`SignifierFactory` | ui_edit_roof 族 |
| clutter | `RaycastWorld::subscribe_to_changes`（×8）、`WorldRaster::subscribe<512>`（×3）| autoclutter_subscribe_to_tabloids |
| clutter | GPU 回读（FrozenMap\<SsboId,Box\<BufferState\>\>::get + 92B 类型断言）| read_bush_positions_from_gpu |
| onboarding | mei DSL（toolbar mei_pre/post_update、render_hints/hint_draw 消费端）| hint_draw/render_hints 族 |
| wall-constructor | render 边界（SSBO 直写 / gpu_flush 接线）| gpu_clear_wall_gate_pieces / gpu_flush_plank_buffers |
| toolbar | `utils::calculate_hash`（utils 库未落库；现镜像 DefaultHasher）| WidgetId::from_name 同源。【coregap-1 已实现】`country_core::utils::calculate_hash`（re-export `render::utils`；0x1405bdda0 = Sip13Rounds = DefaultHasher 逐位一致）。 |

---

## §3 external-dep —— 缺 workspace 依赖

> 状态更新（2026-10-17，restorer-external-dep）：本类已处理，报告见
> `reports/integration/external-dep/RESTORE.md`。taffy/tiny_http/image/open/webbrowser 已补入
> workspace.dependencies + toolbar；hotwatch/rapier/bytemuck 原已具备非缺口；rapier 后端归 core-task。

| 依赖 | 版本（B 锁）| 阻塞函数 | 状态 |
|---|---|---|---|
| `taffy` | 0.3.13 | toolbar TaffyLayout 族 / toolbar_systems / toolbar_auto_hide 族 / render_taffy_ui_to_widgets / toolbar_button\* | 已补入（0.3.13）|
| `tiny_http` | 0.12.0 | toolbar credits::ui / spawn_third_party_license_http_server / SingleFileServer::http/run | 已补入（0.12.0）|
| `image` | 0.25.1 | toolbar photomode/lut_gallery | 已补入（0.25.1，还原 B 解析面 png/tiff/zune-jpeg）|
| `open` / `webbrowser` / `hotwatch` | open 5.0.0 / webbrowser 0.8.10 / hotwatch 0.4.6（git 7576ad4a…）| toolbar feedback-reporter / build-id | 已补入（open/webbrowser）；hotwatch 已具备，补引用 |
| rapier 后端 | rapier3d 0.22.0（git 7b5ece73…，已入 workspace）| wall-constructor maintain_stone_column_colliders | 依赖已具备；后端暴露归 **core-task**（country-core native-physics）|
| （提示）bytemuck 1.16.0 | 已入 workspace | forest 单次 rock 布尔 Pod cast（保留成立，非缺口）| 已具备（非缺口）|

---

## §4 decompile-hardpoint —— 反编译/跳表/内联卡点

| 函数 | 地址 | 失败原因 | 建议工具 | 状态(2026-10-17) |
|---|---|---|---|---|
| `half_timbering` | 0x141218e30 | Ghidra/ReVa 空反编译（void() 占位 + ReVa "Unable to find unique hash for varnode"）；radare2 pdg "No function at this offset" | 唯一 worker 修签名后定点重取（reva write）；或 radare2 raw disasm | **已攻克**：radare2 先 analyze 再 pdg 即出 13KB（见 RE-ANALYSIS §3.1） |
| `color_menu` | 0x142291230 | Ghidra 空（codelen=0）/ ReVa 超时 / radare2 无符号，三路失败 | reva 定点 + 手工签名修复 | **已攻克（hardpoints-ecstate）**：ReVa write worker fresh clone + 600s 定点 → 136KB 全量（shared codelen=0=overlay 损坏）；menu state 0..0x15 switch + WidgetParams/UiState/History（RE-ANALYSIS §2.1） |
| `show_color_options` | 0x14229a7f0 | 同上三路失败 | reva 定点 + 手工签名修复 | **已攻克（hardpoints-ecstate）**：ReVa write worker + 600s → 267KB 全量（真实函数体 687B→~0x38KB 已核）；kind 0..0x26 选项构建（RE-ANALYSIS §2.2） |
| `populate_autoclutter_regions` | 0x1422d6ed0 | 44KB 主体 Ghidra 碎片化（16 段重叠指令/跳表分派）| radare2 全函数反汇编 + 手工重建（跳表 0x14304382c/0x143043818 已读）| **已攻克**：两跳表解码完成，handler 反汇编落盘（RE-ANALYSIS §1.1） |
| `add_autoclutter_on_windows` | 0x1422cd7d0 | Ghidra bad instruction data（边界 364）| radare2 全函数反汇编 | **已攻克**：radare2 全函数反汇编（RE-ANALYSIS §1.2） |
| `update_wrens` | 0x14231e390 | 状态跳表 DAT_14304b638 handler 未取证 | radare2 读跳表 + 定点反汇编 | **已攻克（hardpoints-ecstate）**：跳表解码 5 handler + 常量全读；state 0=计时/1=音频/3=smoothstep 移动（RE-ANALYSIS §1.1） |
| `sheep_walk_animation` / `sheep_pet_animation` | 0x14231a630 / 0x14231ac20 | handler 跳表 0x14304b0f8 / 0x14304b178 未取证 | radare2 读跳表 | **已攻克（hardpoints-ecstate）**：两跳表解码 + 全部 handler 反汇编；walk speed 阈值 0.2 驱动 1↔2、pet 状态机 {0..4} 参数 0.3/1.0/0.2（RE-ANALYSIS §1.2/1.3） |
| `update_butterfly_states` / `set_butterfly_velocities_towards_goals` | 0x141d84d40 / 0x141d867b0 | handler DAT_142f68008 / 动画跳表 DAT_142f68018 语义 U | radare2 读跳表 | **已攻克（hardpoints-ecstate）**：DAT_142f68008 8 项解码（state≥4 转发 vel 表）+ DAT_142f68018 4 项解码（清零/衰减/钳制/固定 y）（RE-ANALYSIS §1.4/1.5） |
| `chimney_smoke_anim_thickness` | 0x140818df0 | 更新公式 divisor（fVar52）反编译打散（CONCAT13 字节重拼）| reva High P-code | **已攻克（hardpoints-ecstate）**：公式还原 `new=clamp01(old+dt/target)`；ReVa write 全量 31KB（RE-ANALYSIS §3） |
| `hint_position_update` | 0x141c50510 | 跳转表未恢复 + 相机矩阵/几何依赖 | 读跳表 + 定点反汇编 | **已攻克**：Ghidra fresh 完整反编译（case 6/7/8/10）+ 附加跳表 0x142f14844 解码 21 项（RE-ANALYSIS §2.2） |
| `hint_focused_edit_hint` | 0x14227dd60 | 跳转表 DAT_1430386a0 | 读跳表 + geo_src 链 | **已攻克**：完整反编译 + 跳表解码（RE-ANALYSIS §2.1） |
| `process_playermade_stair_pillars_proposals` | 宿主 0x140178a70 | bevy impl$3::call_mut 系统包装；内层业务块（提案→raycast→碰撞→rt 事件）未定点 | 定点反汇编解系统包装 | **已攻克**：宿主 Ghidra fresh 47KB，业务块内联（0.01 距离合并 + RawTable 查询 + 0x2cB 提案结构）（RE-ANALYSIS §3.2） |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | 两泛型 \<true\>/\<false\> 翻转、closure 绑定、ArchWalker+from_curve+raycast+StairsPillarProposals 装配 | 按单态逐一定点 | **已攻克**：Ghidra fresh 97KB，全流程 + closure \<true\>/\<false\> 语义（RE-ANALYSIS §3.3） |
| `sine_wave`（BrickRowSine::new/deform/random_f）| module 401 inlinee 24627/24641/24629 | 无独立编译实例，宿主 from_curve（source_line 12/45/82）| 从 from_curve 体内按行号提取 | **已攻克（hardpoints-ecstate）**：宿主 from_curve 0x140769760 55KB 全量；random_f（hash rng 12.9898/43758.547）+ deform（`0.5+0.5*sin(x*0.2)` 平滑 + 0.06 幅度两相）公式完整（RE-ANALYSIS §4.3） |
| toolbar `ui_sounds` 等 | 0x14233a7f0（**总账 0x14233c450 有误**，实为 ui_select_animation）| jumptable 破坏（ui_sounds 尾部边界错已坐实）| radare2 重建跳表 | **部分攻克**：真 ui_sounds=0x14233a7f0，switch ~18 case 已出（截断处待定点）（RE-ANALYSIS §4.4） |
| toolbar `toolbar_auto_hide` 族 5 函数 | 0x142343a10/0x142344560/0x1423448a0/0x1423449f0/0x142344df0 | jumptable 破坏；对齐 case 偏移公式 U | radare2 重建跳表 | **部分攻克**：toolbar_alignment 跳表 0x14304e6d8 解码 20 项；auto_hide 常量已取（RE-ANALYSIS §4.3/§4.6） |
| toolbar `place_childen_impl` | 0x140fe12a0/0x140fe19a0 | justify 全枚举+跳转表 0x142bc223c/0x142bc2260 坏分支未展开 | radare2 重建跳表 | **已攻克**：两单态反编译 + 两跳表解码（XY 33 项/YX 24 项）（RE-ANALYSIS §4.2） |
| toolbar `add_toolbar_item` | 0x141e4dc40 | kind 跳转表 DAT_142f8f774 U | radare2 读跳表 | **已攻克**：完整反编译 + 跳表解码（kind 4/5=容器，其余 spawn TaffyNode 实体）（RE-ANALYSIS §4.1） |
| toolbar photomode `ui_slider` 等 | 0x1423486d0/0x142347f90/0x142349de0 | 滑块跳表截断分支 | radare2 重建跳表 | **部分攻克**：ui_slider switch ~28 case 已出（截断处待定点）（RE-ANALYSIS §4.5） |

注：`ArchWalker::walk` 族 4 单态（0x141232bb0/0x141233380/0x141231ea0/0x14048ab50）与 `find_stone_column_start_y`（0x141250720）**已 fresh 反编译成功**（walk 分歧已证、跳表已核），归 §5 in-module 直接重建，不属本类。任务提示中的 `calc_gothic_half_dormer_profile_ws`（0x1407eb160）**已取证（hardpoints-ecstate）**：Ghidra 全量 + profile curve 3 档点表 + 半径表 {0.25,0.3,0.29}（RE-ANALYSIS §4.4）。

---

## §5 in-module-pure-logic —— 依赖已具备、可直接实现的模块内逻辑

> 逻辑已解/反编译已完成，仅需按证据重建；多数已 `todo!("未实现: …")` 标注。

| 模块 | 函数清单 |
|---|---|
| tree-tool | ~~`hide_trees`~~ → **转 core-type-gap**（命令 shape 字节@+0 bit0 / @+0x24 门控字节缺；候选源 `Decorators::iter_removed_ids` 系统签名无 Decorators 资源；两命令缓冲 stride 0x18/0x30 未建模）、`ui_move_tree` slider3d case1/2/3（依赖 MoveTreeCmd 落库后）|
| critters | `DuckSpawnLocations::new`/`FishSpawnLocations::new` → **已实现/已验证**（2026-10-17 purelogic-integration；网格公式 `(i/9)*52-(1-i/9)*52` + 十档常量 + timer=25.0）；~~`despawn_fish`~~ → **转 core-type-gap**（`FishBoids(Resource)` 缺失 + `OnExit<AppLoadingState>` bevy_state 未入 workspace）；`plan_duck_trajectories`/`move_ducks_along_trajectories`/`update_decorator_bird_spots`/`update_roof_bird_spots` → **转 re-analysis**（dev 热键 A/B+Frenet/raycast 插值、create_spots 布局、random_splits 分段语义、双列表滑动窗口缓冲结构 U）|
| roof | `calculate_roof_trigger_volumes`（曲面三角化，细分/剖面/檐口/UV/噪声）→ **转 re-analysis**（1580 行/12048B，细分计数/剖面采样/檐口/UV/噪声尺度逐语句重建需大段 fresh 反编译对线）|
| decorator | `create_or_restore_decorator_backup` 辅助 0x30B 三 map 反向拷贝（状态机/门控判等；live 资源身份 U）、`window_lights` 收集主循环（常量/灯态/帧末判等；墙/屋顶收集资源身份 U）、~146 业务函数（blueprint/cull_oob/merge/visual 族/stairs/ui_systems）分批 fresh 后重建 |
| clutter | `manage_autoclutter_entities`、`autoclutter_animation`、`add_autoclutter_around_gates/around_windows/add_preplaced/add_tree`、`request_inter_roof_merlon_audio`、`detect_intra_shape_corners`/`add_hole_at_shape_intersection`/`spawn_stitches`（宿主查询 +0xf8..+0x120 逐偏移/四元数/Easing/墙空间链均需逐语句重建）|
| wall-constructor | `ArchWalker::walk` 族 4 单态（travelled/周期 u/尾砖条件，wall_space subtract+project_clamp）、支撑梁 5 函数、segmentation 3 函数、立面支撑整族（circle_categorize/pillars/brackets/bottom_layer/turret/stone_column）、门/栅栏/地板/楼梯族、半木/抹灰族（除 half_timbering 卡点）、utils 近似体族（stairs_signifiers_new/add_wall_corners/add_skirts/overlapping_windows/get_window_aabb/check_for_overlap/add_underwindow）|
| toolbar | `mei_pre_update`/`mei_post_update`（mei 输入→handler 状态机/render 两遍+taffy 最小尺寸）→ **未在本批实现**（宿主查询/taffy 依赖，见 RESTORE.md purelogic 报告）|

> 更新：2026-10-17 restorer-purelogic-integration。hide_trees / despawn_fish / ui_move_tree 因依赖未满足 core 类型转 core-type-gap（见 §1 新增）；plan_duck_trajectories / move_ducks_along_trajectories / update_decorator_bird_spots / update_roof_bird_spots / calculate_roof_trigger_volumes 因 U 语义/结构差异转 re-analysis（fresh 证据见 `work/purelogic-integration/evidence/`）。

> 更新：2026-10-17 reviewer-wave3-tg-tr（wave3 tree-glade/timber-roof 复核，见 `reports/wave3/reviews/tg-tr/REVIEW.md`）。判等 2（feedback_input_out_of_border、simulate_roof_tile_for_audio——angle' 三处修正属实）；重建 8（process_tree_cmds、replay_tree、ui_move_tree、convert_preplaced_trees、closure$1、construct_timberframe_rectangle、construct_timberframe_circle、highlight_roof）；未实现-需重做 1（display_glade_border 分歧①/② 均证实）。分歧登记（T1/T2/T3、分歧①/②、12/15/16、ResMut 库面、orientation_mat3 镜像、IDENTITY 占位、bit0 代理）全部诚实准确。**补登记 2 项**：①ui_move_tree 门禁二进制 mode∈{0x10,0}（0=None 亦放行），Rust 仅 Tree；②convert_preplaced_trees 墙查询闭包位向已补证（fresh 0x1425defd0：命中墙→清 blocked 位→delete；无墙→转换），Rust 方向判等。

---

## §6 integration-registration —— 已实现/判等但无调度注册点

| 模块 | 条目 | 说明 |
|---|---|---|
| decorator | `clear_out_visual_entities` / `clear_out_wall_holes` / `maintain_dormer_tiles` / `decorator_record_history` / `create_or_restore_decorator_backup` | **4/5 已接线（2026-10-17 purelogic-integration）**：`clear_out_wall_holes` / `clear_out_visual_entities` / `decorator_record_history::<S>` / `create_or_restore_decorator_backup::<C>` 加入 `add_decorator_systems`（现泛型 `<S,C>`）+ 幂等 init_resource（DecoratorVisualState/DeferredDecoratorEditHistory/DecoratorBackup）。`maintain_dormer_tiles` 未注册——纯函数签名非 bevy 系统，且每瓦 GPU 退役主体待 rhapsody 回调注册表（core-type-gap §1）接线后重建为系统再注册。 |
| clutter | `dirty_walls`（0x141cd16b0，判等）| **已注册（2026-10-17）**：`configure` 链前序加 `dirty_walls`；`detect_inter_roof_merlons` 改读 `changes.dirty_walls`（移除自带三 EventReader，对齐二进制独立系统分离）。 |
| timberframe | `timberframe_on_glade_unload` | **仍阻断**：`OnExit<AppLoadingState>` 依赖 bevy_state 未入 workspace（external-dep）；函数已暴露待宿主接线。 |
| water | `draw_water` 注册 + 追加自身事件 | **已验证已接线**：`add_water_cpu_systems` 已注册 `draw_water` + `register_water_events` 已注册事件（自足接线）。 |
| toolbar | `ToolbarPlugin`/`Ui2dPlugin` 完整 build（9 事件+Startup/Update 系统集合）| **仍阻断**：对应系统/事件未落库（taffy/tiny_http/image 依赖缺），build 未重建；系统落库后接线。 |

---

## 统计（各根因条目数，以总表行计）

| 根因 | 条目数 |
|---|---|
| core-type-gap | 49（+2：hide_trees / despawn_fish，2026-10-17 转）|
| core-api-missing | 15 |
| decompile-hardpoint | 3（§4 共 19 行：16 行「已攻克」（eco/state 5 跳表 + chimney + color_menu/show_color_options + half_timbering + populate_autoclutter + add_autoclutter_on_windows + playermade_stairs + sine_wave + onboarding 2 + wall-constructor 2 + toolbar place_childen/add_toolbar_item，2026-10-17）；余 3 行 toolbar「部分攻克」（ui_sounds / toolbar_auto_hide / ui_slider）非 eco/state 范围）|
| in-module-pure-logic | 13（−8：DuckSpawnLocations::new+FishSpawnLocations::new 完成；hide_trees/despawn_fish 转 core-gap；plan_duck/move_ducks/bird_spots/calculate_roof_trigger_volumes 转 re-analysis）|
| re-analysis（待证，附于总表）| 12（+4：plan_duck_trajectories / move_ducks_along_trajectories / update_{decorator,roof}_bird_spots / calculate_roof_trigger_volumes；−3 已补证：manage_tree_prefabs has_construct_shape / move_critters / critters_avoid_collisions，hardpoints-ecstate 2026-10-17）|
| external-dep | 5（已处理，2026-10-17 见 §3 状态列）|
| integration-registration | 5（4 项已接线：decorator 4/5、clutter dirty_walls、water draw_water 验证；timberframe/toolbar 仍阻断）|
| **合计** | **≈121** |

> 说明：re-analysis（待取证/待证门控源）未单列根因，归总表对应函数附注；统计含跨类行按主根因计数。core-type-gap 为绝对大头（约 39%），次为 in-module-pure-logic 与 decompile-hardpoint——分派优先级建议：core-task（类型/API）→ dep-task（workspace 依赖）→ re-analysis（卡点补证）→ implement-in-module（纯逻辑）→ integration（注册接线）。

---

## re-analysis 更新（re-analyst-hardpoints-interaction · 2026-10-17）

> 报告：`reports/re-analysis/hardpoints-interaction/RE-ANALYSIS.md`（+ `evidence/` 跳表解码/反汇编/反编译）。未改 src、未改库。

### decompile-hardpoint 攻克 9 项
| 函数 | 地址 | 攻克结论 |
|---|---|---|
| `half_timbering` | 0x141218e30 | radare2 先 `analyze` 再 pdg 成功（13KB），含 split_height_into_skirts(0x140b34f80) 主链 |
| `populate_autoclutter_regions` | 0x1422d6ed0 | mesh 跳表 0x14304382c（26+ 项）+ kind 跳表 0x143043818（5 项）解码，handler 反汇编落盘 |
| `add_autoclutter_on_windows` | 0x1422cd7d0 | radare2 全函数反汇编（0x68B spawn 记录） |
| `hint_position_update` | 0x141c50510 | Ghidra fresh 完整反编译（case 6/7/8/10）+ 附加跳表 0x142f14844 解码 21 项 |
| `hint_focused_edit_hint` | 0x14227dd60 | 完整反编译 + 跳表 0x1430386a0 解码（subject 1/2 建 hint，0/3+ 无） |
| `process_playermade_stair_pillars_proposals` | 宿主 0x140178a70 | 宿主反编译 47KB，业务块内联（0.01 距离合并 + 0x2cB StairsPillarProposal） |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | 反编译 97KB，全流程 + closure \<true\>/\<false\> 语义 |
| toolbar `place_childen_impl` | 0x140fe12a0/0x140fe19a0 | 两单态反编译 + 跳表 0x142bc223c(33)/0x142bc2260(24) 解码；style +0x44 swizzle / +0x46 justify |
| toolbar `add_toolbar_item` | 0x141e4dc40 | 完整反编译 + kind 跳表 0x142f8f774 解码（kind 4/5=容器，其余 spawn TaffyNode 实体） |
| toolbar `toolbar_alignment` | 0x142344560 | 跳表 0x14304e6d8 解码 20 项（alignment@+0xf0） |

### 部分攻克 / 地址更正
- `ui_sounds`：**总账地址 0x14233c450 更正为 0x14233a7f0**（0x14233c450 实为 ui_select_animation）；switch ~18 case 已出。
- `ui_slider`（0x1423486d0）：switch ~28 case 已出；`toolbar_auto_hide`（0x142343a10）常量已取。
- `resolve_hole_overlap` case0：fresh 已证 split_x 门控，仅需定向冻结用例（测试任务）。

### 未攻克（方法已定）
- `sine_wave`（module 401 inlinee）：宿主 half_timbering 已可 pdg，按行号提取 BrickRowSine 内联体。
- `load_clutter_meshes`（0x1422c0760）：需 strings 提取网格名清单。
- toolbar photomode 族 `do_photomode`/`lut_gallery`：未展开。
- 关键机理：**Ghidra 函数边界在跳表处截断（body_end 恰在 `jmp rax`）是"jumptable 破坏"根因**；Rust 跳表编码统一为 `table_base + signed32`，可离线解码（见 RE-ANALYSIS §0）。

---

## re-analysis 更新（re-analyst-hardpoints-ecstate · 2026-10-17）

> 报告：`reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md`（+ `evidence/` 36 反编译/反汇编、6 张跳表解码、MANIFEST.json）。未改 src、未改库（ReVa write worker `hdpts-ecstate` 仅定点脚本）。

### decompile-hardpoint 攻克（eco/state + 任务点名）
| 函数 | 地址 | 攻克结论 |
|---|---|---|
| `update_wrens` | 0x14231e390 | 状态跳表 0x14304b638 解码（5 handler）+ 常量全读；state0=计时/1=音频+state→2/3=smoothstep 移动/2,4=扫描续 |
| `sheep_walk_animation` | 0x14231a630 | 跳表 0x14304b0f8 解码（0/3 无 handler、1=walk fmod 帧索引 mod 30、2=过渡 dt/0.1→3）；speed 阈值 0.2 驱动 1↔2 |
| `sheep_pet_animation` | 0x14231ac20 | 跳表 0x14304b178 解码（5 状态机），参数 0.3/1.0/0.2，[spot+0x58] 进度+[0x60] 标志 |
| `update_butterfly_states` | 0x141d84d40 | 跳表 0x142f68008 解码（8 项，state≥4 转发 vel 表）；距 4.0/3.0 门、phase 递减 |
| `set_butterfly_velocities_towards_goals` | 0x141d867b0 | 动画跳表 0x142f68018 解码（4 项：清零/衰减+向下/钳制/固定 y=0.8856） |
| `chimney_smoke_anim_thickness` | 0x140818df0 | ReVa write 全量 31KB；公式还原 `new=clamp01(old+dt/target)`，target=lerp(1,3)/lerp(2,4) |
| `color_menu` | 0x142291230 | ReVa write fresh clone + 600s → 136KB 全量；shared codelen=0=overlay 损坏；menu state 0..0x15 switch |
| `show_color_options` | 0x14229a7f0 | ReVa write + 600s → 267KB 全量；kind 0..0x26 选项构建 + DecoratorStorage 分派 |
| `half_timbering` | 0x141218e30 | radare2 `aa` 后 pdg 成功 13KB（与 interaction 报告交叉验证）；call 目标身份已查（center/width/max_y/WoodenPlankSsbo::push） |
| `populate_autoclutter_regions` | 0x1422d6ed0 | Ghidra 已全量 120KB（不再碎片）；kind switch 0..0x19 构建 AutoClutterSpawn 0x98 记录 |
| `add_autoclutter_on_windows` | 0x1422cd7d0 | radare2 全函数 + 跳表 0x143042370 解码（6 handler 四元数旋转 + spawn 0x68B 写） |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | Ghidra 全量 97KB + closure 单态清单（与 interaction 报告交叉验证） |
| `sine_wave`（BrickRowSine）| module 401 inlinee | **已攻克**：宿主 from_curve 0x140769760 55KB 体内提取；random_f（hash rng 12.9898/43758.547）+ deform（`0.5+0.5*sin(x*0.2)` 平滑 + 0.06 两相）公式完整 |
| `calc_gothic_half_dormer_profile_ws` | 0x1407eb160 | Ghidra 全量 + profile curve 3 档点表 + 半径表 {0.25,0.3,0.29}（任务点名补取证） |

### re-analysis 残留补证
- `manage_tree_prefabs`（0x141bf16c0）has_construct_shape 门控：**= `*(char*)(记录+0x18)` 处理状态字节**（SSBO 缓冲记录，首帧置 1 后走随机 scale 分支），非事件存在性。
- `move_critters`（0x142318fa0）：命中码 2=岩石（sample>0 阻塞）、3=跳过墙 gable 检查、非位集 loc∉{0,1,5,6} 才 spherecast；mult==0 时位移=vel*dt*0 恒 0，仅墙 relate/朝向分支。
- `critters_avoid_collisions`（0x1423179a0）：hitcode==2 且 rock_sample>0 → 跳过碰撞；`hit==2\|\|球距>=1.0` → 保留目标；`<0.4 && water8` → DespawnDuckCmd。

### 关键机理
- **shared Ghidra DB 的 color_menu/show_color_options codelen=0 是 overlay 损坏**（fresh ReVa clone 反编译正常）；restorer 重建须以 write worker clone 反编译为基。
- **half_timbering 的 Ghidra/ReVa "Unable to find unique hash for varnode" 在修签名后仍复现** → 固定走 radare2 `aa`+pdg 路径。

---

## coregap-2 REVIEW 更新（reviewer-coregap-2 · 2026-10-17）

> 报告：`reports/integration/coregap-2/REVIEW.md`（+ `review-evidence/` 两批 fresh 反编译）。未改 src。
> 方法：ghidra `--worker shared` fresh 反编译（0x142298d30/0x142290750/0x142298260/0x1422af0c0/0x14227bd30/0x14227da70/0x14227c230/0x140a658a0/0x1404b0ed0/0x140916420）+ 组内 evidence 交叉核对。

### 裁决
- **判等 10**：GameProfile 提示区字段/偏移（0x1bc/0x1c0/0x1c4 u16 + 0x1e0..0x1f0 计时 + 0x1f4..0x206 19 标志）、TutorialCooldowns、16× NotifyHintSystem* 事件 + CameraMoved {tag,value}、TutorialOp 18 变体 id→偏移表（逐 case 判等）、WriteSettingsFileCmd（unit 8B）+ SaveGameProfileCmd（I）、DisplayRollercoasterPrompt、ModeJustChanged 0x20B/tool-1<2/0x28B、StructureType::is_fence（0fb601c3）、Wall/Gate/Halftimber/RoofColorIdSsbo + SsboType（memcmp 0x32/0x32/0x38/0x37、BUFFER_LEN 20480、SsboId 命中）、ChimneyExtra+4 smoke。
- **需修 2 → ✅ 已修（restorer-coregap-2 2026-10-17 落实）**：③ 事件载荷族装饰/窗 5 条删 `_gap4`（消费面 value@+0xc/custom@+0x10 无 gap）、按二进制名改 SetLadderColorIdCmd/SetMagicColorIdCmd/SetDecoratorBrightnessCmd；④ GameProfile serde 改 serialize 写 player_id（`#[serde(alias="game_installation_id")]`）、删 game_installation_id 序列化字段。附带登记已同步（reset_hint_flags 只清 0x1e0..0x1ef、不动 +0x1f0；事件模块域 system_color 登记）。
- **附带登记 2**：事件类型名/模块（system_color 域 vs country-core 落库）、reset_hint_flags 次要不符（不清 +0x1f0、cooldown 置 0 非 720）。
- **冲突跳过核验 1**：SteamInput 登记准确（rebuild-steam 拥有 steam.rs；coregap-2 仅 `pub mod steam;`）。
- **cargo check 通过；本批单测 9/9 通过。**

---

## wave2 critters REVIEW 更新（reviewer-wave2-critters-anim · 2026-10-17）

> 报告：`reports/wave2/reviews/critters/REVIEW.md`（+ `evidence/` 7 函数 fresh 反编译）。未改 src、未改库。
> 方法：ghidra `--worker shared` fresh 反编译（0x142318fa0/0x1423179a0/0x14231a630/0x14231ac20/0x141d84d40/0x141d867b0/0x14231e390）+ 跳表字节实读复算（0x14304b0f8/0x14304b178/0x14304b638/0x142f68008/0x142f68018）。

### 裁决
- **判等 4 全量 + 2 部分判等**：sheep_walk（主循环+h1/h2 逐语句）、sheep_pet（h0-h4 逐语句，pet h0 qword store 确清零 progress）、butterfly vel 表（4 handler 逐语句）、shared 命中码（move_critters `hitcode!=2&&球距<1.0` 阻塞 / critters_avoid `hitcode==2||球距>=1.0` 保留、`<0.4&&water8` Despawn）；butterfly states 主循环/step_state/state1/3 结构与 wren h0/h1/h2/h4+h3 公式部分判等。
- **需修 2（wren h3）**：① anim_val=progress×7.0（0x142a7cd48=7.0，Rust 缺 ×7，RESTORE 未登记）；② 移动精确式 `pos.y+=vx*dt*(dvy*dir_y+dir_z)`、pos.z 不变（Rust 现 y/z 分列近似）。另补 move_critters spherecast ①外层门 `(local_fc||local_100)`（Rust FIX① 门不完整）。
- **残留登记核验 4 项准确**（wren 距离查询/h3 y 槽近似、butterfly state0/2 U、water2/water8 近似）。
- **补充登记 4**：wren h3 anim_val ×7、move_critters 外层门、butterfly state3 NaN 次差、critters_avoid 墙 relate 循环语义（仅模块头登记）。
- 未编译（src 未改，仅核验；RESTORE 的 /tmp 独立 crate 编译结论未复跑）。

---

## wave2-geometry REVIEW 更新（reviewer-wave2-geometry · 2026-10-17）

> 报告：`reports/wave2/reviews/geometry/REVIEW.md`（+ `jobs/` 本会话 fresh 反编译/反汇编）。
> 方法：ghidra `--worker shared` fresh（0x140769760/0x140178a70/0x1422cd7d0/0x1422d6ed0/
> 0x1422c0760/0x1407eb160/0x140818df0/0x141bf16c0）+ radare2 fresh（0x1422cd7d0 全函数、
> 0x1412ac9b0 半径选择、跳表/常量 hexdump）。未改 src。

### 裁决
- **判等 8**：BrickRowSine random_f/sine_wave_deform（宿主 0x140769760 体内逐语句）、
  merge_nearby_proposals（0x140178a70 步骤2，44B 合并语义）、add_autoclutter_on_windows
  （0x1422cd7d0 radare2 全函数：drain/跳表 6 项/反射公式/写块 0x68/未初始化读 +0x08/+0x31/+0x33/
  NOP 垫实读）、load_clutter_meshes 结构、gothic_half_dormer_radius（is_circle&&kind==7→表，
  **方向纠正成立**）、chimney_smoke_anim_thickness 公式（+0xd 分派/符号/`clamp01(old+sign*dt/target)`
  全落实）、manage_tree_prefabs 门控（**+0x18 处理字节补证成立**、阶段1 墙圆恒运行修正正确）。
- **需修分歧点 3**：① populate KIND_MESH_SLOTS kind-0 mesh-10 权重 = scale×1.5 链（RESTORE 表
  (10,1.0) 与自身"槽8权重=scale*1.5"注释矛盾，kind-2 slot-0 同槽取 1.5 内部不一致）→ 定点核对；
  ② select_mesh 边界 `acc>r`（二进制）vs `r<=acc`（Rust）+ 兜底首槽 vs 末槽；③ load_clutter_meshes
  closure 计数 55 vs "~59"（微高估）。
- **附带登记 2**：gothic radius rank∉1..3 二进制 panic vs Rust 返回默认 0.38；AutoClutterSpawn
  doc +0x38=position/+0x40=host_id 与二进制 +0x38=host_id/+0x40=cand+0x40 标注错位（已 U）。
- **未实现（诚实 U，无高估）**：pillars 宿主系统步骤 4-6、populate kind1/default 槽表/
  kind4 权重/mesh→资产、calc_gothic wall-space 变换/0..2 索引、chimney spot 收集主循环/
  rng 种子/retain 谓词、manage 构造形状源 ShapeParameters、load 路径表 I。

---

## wave3 onboarding REVIEW 更新（reviewer-wave3-onboarding · 2026-10-17）

> 报告：`reports/wave3/reviews/onboarding/REVIEW.md`（+ `evidence/fresh_reviewer/`
> 8 函数独立 fresh 反编译，`jobs/reviewer-fresh-decomp.json` 可重放）。未改 src。
> 方法：ghidra `--worker shared` fresh（0x14227c230/0x14227b450/0x14227c780/
> 0x14227bd30/0x142284800/0x14227c910/0x141c1c8d0/0x14227bb70）；独立反编译与
> restorer `evidence/fresh_*.c`（7 函数）+ batch4 逐字节一致（证据真实）。

### 裁决
- **判等 4 全量复验通过**：tutorial_completion（18 路 switch 偏移逐项一致 + 每事件写
  WriteSettingsFileCmd + id≥0x12 跳过置位仍写命令）、hints_startup（12 资源序列 +
  u64 count@+0/hint_done@+8/state@+9 逐字节）、hint_camera_enable_condition（四分支
  逐语句）、hint_camera_progress_update（tag 2→rotate/1→zoom(value)/其它→pan +
  跨 1.0 完成 0xa/0xb/0xc）。
- **需修 3**：① hint_roof_shape 完成路径非判等（`state.event_byte` 代理当前事件字节
  → 完成晚一帧/单事件不触发），RESTORE"完成事件判等"降级为近似；② hint_undo 模块
  注释 "id 6" 与 RESTORE 表 "id 2"、Rust 写 id 2 不一致（fresh element[0]=6 为
  placement、id@+0x49=0x02 与 id 2 一致）；③ prompt_rollercoaster "id 10=CameraPan"
  待核（element[0]=10 更可能是 placement ScreenCenter，布局落库时复核）。
- **重建 22 其余核验通过**：flags 全量逐字节核验（0x100/0x200/0x300/0x400/0x600/
  0x800/0xd00/0xe00/0x1000/0x1100/0x1200/1）、Steam packed_set/plus_one 载荷、
  resolve_steam case 0xc9/列表递归、camera_pan/rotate/zoom 字形模式、make_*_binding_
  hint 图串、ModeJustChanged 工具∈{1,2}、exponential_back_off 双冷却对 + 720/10800
  clamp、reset_hints 0x1e0..0x1ef 清（+0x1f0 不动）。
- **未实现 6 诚实**：hint_orchestrator/hint_draw/render_hints/hint_position_update/
  hint_focused_edit_hint/hint_clutter_focused_edit_hint 均为空体 stub + 分歧点登记，
  无近似混过。
- **补充注记 4**：exponential_back_off 对 B 读者取末个事件载荷 vs Rust `.next()` 首个；
  hint_camera_display_condition Rust 无条件下发全部 3 请求（原逐条 duration/progress
  门控，过度近似）；hint_camera_progress_update 事件 tag 字节偏移读面@+0 与写面
  {id@0,event@8}@+8 冲突待按 bevy fork EventInstance 确认；GameProfile Rust 内存布局
  与原生字节偏移不同（ron 字段序匹配，字节互操作需注意）。
- 编译/测试复跑：`cargo check -p system-onboarding` 0 错误；`cargo test -p
  system-onboarding` 9/9 通过。

---

## gap-rhapsody-resources 合同登记（restorer-gap-rhapsody-resources · 2026-10-17）

> 报告：`reports/gap/rhapsody-resources/RESTORE.md`（+ 10 份本会话 fresh 反编译证据 `*.c`）。
> 未改 src。范围：rhapsody 回调注册表 / RoofTileSsbo SparseSsbo 面 / NaniBundle/DrawableMeshBundle /
> InsertTreeCmd 0x48——冲突回避为主，全部出合同+登记（实现 0 / 合同 4 / 冲突跳过 4）。

1. **rhapsody 回调注册表（maintain_dormer_tiles）**：fresh 0x1407ce920 = `RawTable::remove_entry<(u64,
   Box<dyn FnOnce(*mut c_void, bool) + Send>)>`，key=u64（`hash_one<RandomState,&GlobalId>`）；
   返回 Box 非空 → 条件 drop `SparseInstanceBufferAllocation<RoofTileSsbo>`（归还 waste）。
   rhapsody 在途（REFACTOR_PROGRESS 全 IN_PROGRESS），rhapsody_contract.rs 亦在途 → 仅合同 §1。
2. **RoofTileSsbo SparseSsbo 面（gpu_flush_buffers 0x140de50a0，fresh）**：需
   `impl SparseSsbo CHUNK_COUNT=0x2000` + `SsboType::Storage=SparseInstanceBuffer<Self>`（替换现
   `ssbo_carrier!` TypedStorageBuffer）；gpu_flush_buffers 5 行逻辑全可观测（tick@+0x5b0、
   get_mut<RoofTileSsbo>→deref→gpu_flush→tick@+0x150）；GraphicsDeviceResource 缺 +8/+0x10/+0x150 槽（登记 U）。
   roof_tiles_gravity.rs 在途 → 仅合同 §2。
3. **NaniBundle/DrawableMeshBundle**：布局合同——NaniBundle<T> 12 类实例数据泛型（closure-env 0x50B
   观测）、DrawableMeshBundle = mesh handle + raster shader handle + Transform + bool（stars/clouds
   +0x58、billboard +0x48）；`DisplayChildAutoClutterInfo`=8B u64（impl$5 fresh）。消费组在途 → 仅合同 §3。
4. **InsertTreeCmd 0x48**（fresh 0x142269970 写块 + 0x142267430 读面复核）：stride 0x48
   `{name Arc<str>@+0/+8, position@+0x10, scale@+0x20, {variant,rotation}@+0x28, flags@+0x30,
   id@+0x38, height@+0x40}`；tree_edit.rs 由 history 组持有（在途）→ 仅合同 §4 + 登记。
5. **cargo check -p country-core 通过**（13 条 snapshot.rs 既有 non_snake_case warning，非本批引入）。

---

## gap 轮复核（reviewer-gap-round · 2026-10-17）

> 报告：`reports/gap/review/REVIEW.md`（fresh 证据 `work/reviewer-gap-round/`）。
> 范围：`reports/gap/core-events/RESTORE.md` + `reports/gap/utils-toolbar/RESTORE.md`。
> 方法：ghidra `--worker shared` fresh 反编译 + `disassemble_function` 机器码 + glam 0.29.3 逐分支对照；只读，未改 src。
> 结论计数：**判等 7 / 需修 3 / 未实现 2**。

### 需修（3）

1. **`RequestAppModeChange` 载荷面回退为 `AppMode`（16B）** —— core-events §1.1 误判。
   - fresh `switch_app_mode_button` 0x142341fe0：热路径写 `{16B payload@+0, event_count@+0x10}`，stride **0x18**
     ⇒ `EventInstance<AppMode>` = 8+16 = 24B；`(String, SettingsMenuState)` 会使 EventInstance 变 0x28，与原 stride 不符。
     实测 Rust 尺寸（链接现 rlib）：`AppMode=16`、`WindowFocusEvent=1`、`RequestToolbarCategoryChange=1`、
     `SettingsMenuState=1`、现 `RequestAppModeChange=32`（⇒ EventInstance 0x28，多 16B）。
   - 消费 closure `check_if_can_switch<mode_manager::closure_env_3>` 0x140afa2c0 fresh：把事件载荷 **16B 整块写回
     AppMode resource**，再按 tag 分派（tags 0..5 = Editing、0x14 = CloneClutter → toolbar 特路；其余
     `Toolbar::switch_mode()`），与 Rust `select_app_mode` 逐分支一致 ⇒ 载荷值就是 AppMode。
   - `grow_one<tuple<ref str, SettingsMenuState>>` 是 **ICF 折叠符号**（RawVec::grow_one 机器码只依
     size/align，24B/8 对齐的 `EventInstance<AppMode>` 与 `EventInstance<(str,SettingsMenuState)>` 同码）；
     caller mangled `EventWriter<...RequestAppModeChange>` 为名义类型（alias 会被擦除），不能由该符号推字段。
   - FIX：`pub struct RequestAppModeChange(pub AppMode)`；删 `named(...)`/`app_mode_from_request_name(...)`
     （名字映射为自造语义）；消费面 `select_app_mode(request.0, ...)`；同步
     `ui_focus_decoclutter.rs:105` 的 `named("DecoClutter", ...)` 与两处测试。
2. **`utils::rotation_from_normal` 归一化有效掩码** —— 原生位掩码 = 「有限且非零」
   （`absbits ∈ [1,0x7f7fffff]`，两处缩放同掩码）；Rust 仅 `is_finite()`，`inv==0` 时判有效（原生判退化）。
   FIX：两处改 `inv.is_finite() && inv != 0.0`。4 分支半 sqrt 映射本身判等（glam `from_rotation_axes` 逐分支对上）。
3. **`tod_v2_button` per-widget `+2` 写序** —— 原生 click 块内先 toggle `visible`，随后 per-widget 循环写
   `+2 = visible`；Rust 在 toggle 之前写。FIX：写循环移到 click 处理之后（`toolbar_button.rs:199-219`）。

### 判等（7，要点）

- `RequestToolbarCategoryChange(pub WindowFocusEvent)`（1B 载荷、EventInstance stride 0x10、caller 名义类型）。
- `WindowFocusEvent(pub u8)`（三写面 1B@+8、stride 0x10）。**描述纠正**：finish_timelapse 载荷是状态字节
  `*pbVar2`，非字面 0。
- `HistoryTransactions::ledger_mut()`；`insert_starting_edit` 0x140a844f0 机器码逐语句复核（assert cursor≥-1、
  ++、链首 memmove 插入、fastrand nonce@+0x60、count_58=0）。
- `UiSignifierStream::write_raw/write_factory`（0x140b2cf90=`JMP 0x14146c880`；记录 stride 0x28、流 0x170B；
  5 qword 位型保持）。**文档纠正**：RESTORE §3「drain 读 [read_cursor,write_counter)」与当前 `drain=take_all` 不符。
- `Rectangle2d::orientation_mat3`（9 store：列 0/1/2 逐字段，XOR 符号位 ≡ 负号）。
- toolbar 3 字节记录 + keep 四步 + ui_animation 公式/state + switch_category 事件/选中/动画门（常量 0.1/0.2 逐位对上）。
- `Set*/Decorator*/Ladder/Magic/Brightness` 族沿用 coregap-2 判等（本批未改；本轮未重新反编译，沿用既有 fresh 证据）。

### 未实现（2，诚实）

- `switch_app_mode_button` 空壳（载荷源未接；随 FIX-1 回退后按按钮组件 AppMode 落实现）。
- `UiNav`/`HistoryButton` typed gap 0x10（布局 U 登记）。

### 复核备注

- 「B dense 容器 vs per-entity」定性过强：fresh 各系统以 Query 列基址 + row*3 读写 3 字节记录，即
  per-entity 组件列；当前 Rust 实现更接近原生，`PerWidgetButtonState` 为备用容器（非分歧）。
- 8 条分歧检查表：命中 #3 载荷平面近似 1 处（FIX-1）、#4 事件/命令自造 1 处（名字映射，FIX-1 连带）；
  其余未命中。
- 复跑：`cargo test -p utils --lib rotation_from_normal` 2/2 通过；toolbar/core 复跑因构建盘 ENOSPC
  中断（环境原因，非断言失败），RESTORE 的 1846/2 测试声明本轮未独立复跑。

---

## wave4 geometry REVIEW 更新（reviewer-wave4-geometry · 2026-10-17）

> 报告：`reports/wave4/reviews/geometry/REVIEW.md`（+ `work/reviewer-wave4-geometry/`
> 本会话 fresh 反编译/反汇编/跳表/常量证据，jobs 可重放）。范围：
> `reports/remaining/wall-constructor/RESTORE.md` + `reports/consume/types/RESTORE.md`。
> 方法：Ghidra `--worker shared` 只读 fresh
> （0x141250720/0x14124af60/0x141bc34f0/0x140de50a0/0x1407ce920/0x142267430/
> 0x14226b1a0 + 0x1405b0d70/0x140053320/0x141259000/0x140ad2950/0x140c98980/
> 0x140c98ab0/0x140c97b10/0x14226a170；跳表 0x142c20af8、0x143035538；常量逐字节）。
> 未改 src。

### 裁决（三态：判等 2 / 需修 5 / 未实现-诚实 17 个 todo! 函数 + 1 个登记空壳）

- **判等 2**：`find_stone_column_start_y` 0x141250720（105 指令 + 17 项跳表逐项、
  None/Some 判别、`handle[0]&0x100`、Rectangle2d::new 机器实参序与 minss 链全对上）；
  `gpu_flush_buffers` 0x140de50a0（run_unsafe 体内 get_mut→unwrap→deref→gpu_flush→
  ResMut tick）。
- **需修 5**：
  1. `circle_categorize` 0x14124af60：控制流/常量/shape 28B 解释判等，但
     `.max/.min` 与 `<` 未复刻 `MAXSS/MINSS`、`UCOMISS+SETC` 的 NaN 语义
     （NaN radius 时机器 t=NaN/final=1，Rust t=0/false）。FIX-G1。
  2. `replay_tree` 0x141bc34f0：Delete/Place/Move(非 undo)/时序/逆序移除判等；
     **Move-undo 整条 MoveTreeCmd 事件缺失**（原生用 edit+0x14/+0x1c 旧位置），
     RESTORE「判等」高估 → 降 **重建**。FIX-G2。
  3. `maintain_dormer_tiles` 0x1407ce920：cursor 预写/双链遍历/hash_one+
     remove_entry+drop 均判等，但 Rust `ids.push(value)` 对原生**单槽 u32 store**
     （`[state+0x10]` 循环内不变）。FIX-G3。0x1405b0d70/0x140053320 纠错**成立**。
  4. `process_tree_cmds` Insert 0x142267430：名称门/α/四组件/存储/水面/声音逐语句
     判等；但 `random_tree_prefab` selector 原生=元素+0x30（共享 `id`）低位，
     Rust 传 0 且 T4 理由不成立 → `(cmd.id & 1)`。FIX-G4（`is_corrupted` 仍 T4）。
  5. `ui_move_tree` 0x14226b1a0：dragged=0x14092df10 确认、case1/2/3 偏移/双事件/
     History 判等；但缺 `!DEMO` 门（0x14362c098 指向的 DEMO 覆盖字节），且新增原生不存在的
     `cache.dragging` id 门/case3 清除、`state>=5→case3` 钳位未复刻。FIX-G5/G6。
- **未实现-诚实 17 个 todo! 函数（11 文件）+ 1 个登记空壳**：wall-constructor 降级表全部
  为 `todo!` 无近似残留；另 `overhang_subscribe_to_tabloids` 0x140786440 仍是
  文档化空壳（未进降级表）。RESTORE 计数「10 文件」应为 **11**。

### 高估/文档纠正

- `circle_categorize` fresh 为 **197 条/837B**，RESTORE 写 105 条（系
  find_stone 条数误抄）。
- `replay_tree` 的「判等」与 `maintain_dormer_tiles` 的「判等」各含一处实质差异，
  按上表降级。
- MoveTreeCmd 共享模型 `{id@0,index@8,position@0x10}` 与原生 wire（EventInstance
  头 8B 后 +8=id、+0x10=position）有 8B 偏移歧义；生产/消费同模型自洽，但非逐字节
  判等，登记待 coregap 面确认。
- 8 条分歧表：命中 #2 门控 1（DEMO）、#3 载荷 2（dormer ids / MoveTreeCmd）、
  #4 事件自造 1（selector=0 理由）、#8 字段 1（dormer ids 槽）；#1/#5/#6/#7 未命中。

### 复核备注

- 本轮仅只读反编译对照，未运行 cargo（建议复跑 wall-constructor/tree/roof/decorator
  check 与 circle_categorize 2 单测）。

---

## wave4 remaining-misc REVIEW 更新（reviewer-wave4-misc · 2026-10-17）

> 报告：`reports/wave4/reviews/misc/REVIEW.md`。方法：B patched EXE（SHA cf048e27…）radare2
> 定点 fresh 反汇编 + ghidra shared fresh 反编译（`0x140d14770`/`0x14011350`/`0x140d12d80`/
> `0x140a12650`/`0x14231e390`）+ 常量/跳表字节实读。**未改 src、未改 core。**

### 三态
- **判等 3**：forest `manage_tree_prefabs` 构造形状判定（0x141bf16c0）；tree-tool
  `convert_preplaced_trees`（0x14226be70，History 落库含 `insert_starting_edit`）；
  glade-border `display_glade_border` perimeter 写路径（0x141be0ad0）。
- **需修 4**（逐语句分歧，见 FIX）：
  1. `update_wrens` h0 距离查询缺模式门/阈值（xmm7 1.0/3.0/0 由 EDX 6..20 模式 + byte[res+4] 决定）。
  2. `move_critters` 双水采样组件取错：二进制 `+0x138`=**Sheep**（`(local_fc&1)&&loc!=4`，半径 0.8），
     Rust 用 `duck.is_some()`；应改 `sheep.is_some()`。RESTORE §6 “water2=Duck”与自身映射表矛盾。
  3. `critters_avoid_collisions` Duck 栅格为**两位置 4+4=8 次**采样：
     `c0=clamp(Σ4 sample(pos))`；`d=(c0>0)?|goal.position|:0.3`；
     `c1=clamp(Σ4 sample(pos+dir*d))>0` 才清目标。Rust 仅 `pos` 单点 4 次。
  4. `update_butterfly_states` dispatch flag 基线：`phase>0?4.0:baseline`，baseline 逐路
     **3.0**（dwell<=0 两路/vel.y>0/相位门外）或 **1.0**（0.2<=|0.5-phase_ref|<0.45 两路）；
     Rust 一律 0.0（影响 state0 min 与 phase_ref 增量）。RESTORE 的 “mask 0/4” 与机器码不符。
- **未实现 2（登记属实）**：`hide_trees`（core 命令仍 `{center,radius}`）；`DebugLine::show`
  （core `debug_viz` 无 `show`）。

### 被推翻旧结论的复核
- **wren h3 三轴式**（`x += vx*dx*dt; y += vx*dvy*dy*dt; z += vx*dz*dt`）——属实（fresh 0x14231ed04-ed62，
  xmm2/xmm3 分列 + 两次 shufps）。
- **progress>6.0 = despawn 非 panic**——属实（ghidra fresh 0x140d14770 =
  `bevy_ecs::system::commands::EntityCommands::despawn`，error handler `bevy_ecs::error::handler::warn`；
  wren.rs 串为 track_caller Location）。
- **anim_val=progress×7.0**——属实（mulss xmm15,[0x142a7cd48] 在 6.0 分支前完成）。
- **water2/water8 组件映射**：`+0x130`=Duck、`+0x138`=Sheep 的映射成立；但 RESTORE 把
  双水采样/0.8 半径写归 Duck 是错的（实为 Sheep），且 water8 并非“同点 4 次”。

### 高估检查
RESTORE 自报“判等/重建 8、未实现 2”。本批点名 7 项实为 **3 判等 + 4 需修**（第 8 项
tree_animation 不在点名范围，未复验）。高估点：wren “距离查询 U→关闭”、water2=Duck、
water8“原 8 偏移系误读”、butterfly state0-3“判等”。三处需修均为一两句可精确落地
（组件/位置/常量表），无需 core 类型变更；建议 restorer 按 REVIEW §7 修正并复跑
`cargo check -p system-critters`。

---

## wave4 FIX 落实复核（reviewer-wave4-fixes · 2026-10-17）

> 报告：`reports/wave4/reviews/fixes/REVIEW.md`；证据
> `work/reviewer-wave4-fixes/`（Ghidra fresh `disassemble_function`/`get_functions`/
> `read_memory`，wren h0 跳表目标 radare2 `pd`；jobs 可重放）。**未改 src。**

### 三态：fix_verified 16 / 仍有 FIX 0

- **geometry G1-G6 全部 fix_verified**
  1. G1 `circle_categorize`：fresh 0x14124af60 `MAXSS 0,XMM1 / MINSS 1.0,XMM0`
     与 `UCOMISS ratio,threshold + SETC` 逐条对上；NaN/∞ 语义与 `as usize` 饱和转换一致。
  2. G2 `replay_tree`：fresh 0x141bc34f0 证实 Move-undo 读 `edit+0x14/+0x1c` 旧位；
     Rust 整条路径不发射并显式登记，RESTORE verdict 已降 **重建**（解锁=旧位字段）。
  3. G3 `maintain_dormer_tiles`：fresh 0x1407ce920 `MOV [R13],EBP`（R13 循环内不变）
     → Rust 单槽 `*id_slot = value`，push 已删。
  4. G4 `process_tree_cmds` Insert：fresh `LEA RDI,[R12+0x38] → MOV RCX,[RDI]`，
     callee `TEST SIL,1` → Rust `(cmd.id & 1) as u8`。
  5. G5 `ui_move_tree`：fresh `[0x14362c098]` DEMO 字节门已补（编译期 const seam，
     与 ui_place_tree 同约定）。
  6. G6 `ui_move_tree`：`ADD AL,0xfe / CMP AL,3 / CMOVNC 3` + 跳表 0x143035538
     `{无动作,case1,case2,case3}` 与 Rust `wrapping_sub(2)`/`>2→3`/无 `dragging` 门一致。
- **visual F1-F6 全部 fix_verified**
  F1 容器 dealloc（0x141e4e794 `cap<<6/align8`，Vec 序 cap@+8/ptr@+0x10/len@+0x18 已复核）；
  F2 TaffyNode descriptor **align4/size0x14=20B**；F3 ShowOnly **align1/size0 ZST**；
  F4 build 调用点 `arg6=[RSP+0x178]=&mut Commands`、`arg7=[RSP+0x180]=&mut 状态表`、
  arg5 未读（callee RSP 偏移复核）；F5 impl$5 = 8B 步长 `Vec<Entity>` push，注释/实现已改；
  F6 Ui2dImage descriptor **align8/size0x20=32B**（core 仍 typed-gap，登记未虚报）。
- **misc 4 项全部 fix_verified**
  wren h0 `threshold 1.0/3.0/0`（mode 6..20 + `byte[+4]`）与 `>0 && dist<threshold` 门；
  move water2 组件=`+0x138 Sheep`（loc!=4、两次采样）；avoid Duck 两位置 4+4（c0→d→c1）；
  butterfly flag 基线 `4/3` 与 `4/1`（XMM6=1/XMM12=3/XMM15=4 实读）。

### 残留（非阻塞）
- `circle_categorize.rs:3` 注释仍写「105 条指令」，RESTORE 已更正 197（仅注释）。
- G5 DEMO 为编译期 `false` seam（宿主覆盖）；G3 `id_slot` null 安全 seam；
  G2 旧位路径按设计未实现（已登记）；F6 core `Ui2dImage` 仍 8B typed-gap。
- 本批无新增「仍有 FIX」项，RESTORE 计数/verdict 维持现状。
