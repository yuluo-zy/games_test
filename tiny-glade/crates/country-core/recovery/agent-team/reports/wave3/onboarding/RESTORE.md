# RESTORE — systems/onboarding（wave3 解锁落库）

> restorer-wave3-onboarding，2026-10-17。coregap-2 已落库 GameProfile 提示区
> （game_profile.rs）+ 16×NotifyHintSystem* 事件 + TutorialOp + 命令
> （WriteSettingsFileCmd/SaveGameProfileCmd/DisplayRollercoasterPrompt/ModeJustChanged）
> + SteamInput（rebuild-steam）→ 本批把被解锁的未实现函数逐函数 fresh 反编译落库。
>
> 方法（用户硬标准 2026-10-16/17）：**fresh 反编译**（Ghidra shared worker
> `get_functions fields=signature,decompiled_code`）→ 重建原函数完整控制流 → Rust body
> 逐语句对照。本批 fresh 证据：`evidence/fresh_*.c`（0x14227c230/0x14227b450/
> 0x14227c780/0x142284800/0x14227c910/0x14227e5e0/0x141c1c8d0/0x14227bb70，
> jobs/wave3-fresh-decomp.json 可重放）；其余函数沿用既有 batch1..7 证据并交叉核对。
> 身份：B patched EXE tiny-glade.exe（sha256 cf048e27…6323）。

## 编译/测试

- `cargo check -p system-onboarding`：**0 错误**（2026-10-17 复核完成时；随后 rhapsody
  被他组在途重构破坏（11:34 改 rg/sync_plan/bind.rs + vulkan_queue.rs，10 个类型错误），
  workspace 级 check 被其阻断——登记归属，本模块未触碰 rhapsody）。本批落库以
  rustc metadata 检查复证 **0 错误**（`--emit metadata` 对 prebuilt 依赖 .rmeta，
  bevy_ecs-970865816ef8625b 匹配 country_core-4dd78fa0a9702b17）。
- `cargo test -p system-onboarding`：**9/9 通过**（组合子/cleanup/quantize/HintType 值；
  rhapsody 破坏前复跑，reviewer 3 项修正不改测试面）。

## 逐函数状态

| 符号 | VA | 状态 | 证据 | 说明 |
|---|---|---|---|---|
| `tutorial_completion` | 0x14227c230 | **判等** | fresh_0x14227c230.c | 逐事件 switch id 0..0x11 → GameProfile 对应偏移置 1（`TutorialOp::from_id`+`mark_tutorial_op_complete`）+ 每事件写 WriteSettingsFileCmd（早前表 "SaveGameProfileCmd" 表述修正为同 app_config 域 I）。id≥0x12 无 default 不改标志但仍写命令。 |
| `hints_startup` | 0x14227b450 | **判等** | fresh_0x14227b450.c | 12 资源 insert_resource 序列逐命令字节核对：各条件资源值=对应完成标志（Deselect[0x1f4]/Focused[0x1f5]/Terrain[0x1f9]/Steepness[0x1fa]×2/DisableSnapping[0x1fc]/RoofShape[0x1ff]×2/Clutter[0x200]/TerrainFocus[0x1fb]）、Undo/Stairs 清零、CameraHintsState state=(0x1f6&&0x1f7&&0x1f8)?2:0、CameraHintProgress {进度=各标志?1:0, 阈值 0.7/0.7/60}。 |
| `exponential_back_off` | 0x142284800 | **重建** | fresh_0x142284800.c | Local+=dt；鼠标事件→归零、否则 local<30 才 profile[0x1f0]+=dt；两对(0x1e0/0x1e4, 0x1e8/0x1ec)冷却缺省 720、事件→计时归零+cooldown=clamp(old*1.5,720,10800)+写 WriteSettingsFileCmd；local<10 才计时+=dt+clamp[0,cooldown]（cooldown<0 走 f32::clamp panic）；timer>=cooldown→写命令+清条件资源(10B)+清标志。**分歧点**：对 A 读者事件类型 U（候选 DisplayHintRequest）；对 B 读者 = ModeJustChanged（0x28B 步长、首载荷 u32∈{1,2}，判等）。 |
| `reset_hints` | 0x141c1c8d0 | **重建** | fresh_0x141c1c8d0.c | 首事件字节==0 → `set_all_hint_flags`（0x1f4..0x206 全置 1）+ state=2；否则 12 资源复位 + `reset_hint_flags`（含 0x1e0/0x1e8 清零、不动 +0x1f0）+ state=0；写 WriteSettingsFileCmd。**分歧点**：触发事件类型 U（`ResetHintsCommand(pub u8)` seam）。 |
| `prompt_rollercoaster` | 0x14227bb70 | **重建** | fresh_0x14227bb70.c | 事件→`Steam(packed_set_plus_one) → alternate(Option<KeyCode>)` 写 DisplayHintRequest（**placement ScreenCenter(10)**，flags 0x1200）。**分歧点**：Steam index / 键码 / hint id@+0x49 U（reviewer 纠错：原 "id 10" 为 placement 误读）。 |
| `hint_camera_enable_condition` | 0x14227c780 | **判等** | fresh_0x14227c780.c | state!=0 返回；鼠标事件→counters[1]=0、[0]+=dt，无→[1]+=dt、<10 才[0]+=dt；相机事件（NotifyHintSystemCameraMoved）→[3]=0、[2]+=dt，无→[3]+=dt；`[2]>0.1 ? ([3]>1.5‖[0]>100) : [0]>100` → state=1。逐语句判等。 |
| `hint_camera_display_condition` | 0x14227c910 | **重建** | fresh_0x14227c910.c | state==1 才继续；三完成标志→state=2；AppMode 门控（1/2/6+shape1|2，is_fence 时含 10）；逐条 (0xa/0xb/0xc) 写 DisplayHintRequest(id 0xa..0xc, flags 1)。**分歧点**：duration 递减 + PlayUiSound(kind 0x13) 完成写面以 seam 表达（U）；is_fence 实参取 AppMode shape 字节近似。 |
| `camera_pan_hint` | 0x14227d000 | **重建** | batch4 | profile 0x1c0 → Steam+alternate：==1 [mouse/rmb/rmb-move]、==2 [mouse/mmb/mmb-move]、其它 MouseButton。**分歧点**：Steam index / Row 四键 / Graphic size/color U。 |
| `camera_rotate_hint` | 0x14227d2f0 | **重建** | batch4 | profile 0x1bc → together(steam,steam) → 三图/Mouse → adjacent(Key) → alternate。**分歧点**：Steam index / 键码 / 尾项 U。 |
| `camera_zoom_hint` | 0x14227d680 | **重建** | batch4 | adjacent(steam,steam) → alternate。**分歧点**：Steam index / 尾项 U（本重建 MouseButton::Middle 占位）。 |
| `hint_highlight_shape` | 0x141c50120 | **重建** | batch2 | eased>0.9 且 HintType∈{1,2}（`*piVar1-1U<2` 与 `*piVar1-6` 的 `4<uVar6‖uVar6==3` 对 {1,2} u32 回绕同时成立）→ roof_entity 有效 → HighlightRoofCmd/HighlightWallCmd tag 2。**分歧点**：+0x11 字节语义 / WallId 源 U（本重建 WallId(0) 占位）。 |
| `resolve_steam_impl` | 0x14227e5e0 | **重建** | batch1 + fresh | `Steam(query)` → `SteamInput::get_action_origin_glyphs`：None→Empty / Some(0)→Empty / Some(1)→单元素 / Some(≥2)→Alternate；列表递归。Steam 载荷改为 `ActionOriginGlyphQuery`（原 `crate::SteamInput` seam 删除）。**分歧点**：closure_env_1 的 Graphic size/color U。 |
| `make_focus_or_camera_binding_hint` | 0x141c3de00 | **重建** | batch5 | profile 0x1c4 → Steam(packed_set_plus_one) → ==1 [mouse/rmb]、==2 [mouse/mmb]、其它 MouseButton。**分歧点**：Steam index / Graphic size/color U。 |
| `make_lbm_drag_binding_hint` | 0x142285270 | **重建** | batch5 | together(steam,steam) → alternate(Graphic("hint/lmb-move"))。**分歧点**：Steam index U。 |
| `hint_focused_edit_condition` | 0x14227da70 | **重建** | batch5 | profile 0x1f5!=0 → hint_done=1；ModeJustChanged 工具∈{1,2} → hint_done=1；未 done 返回 false；写 TutorialHintComplete(1)；焦点字节==1 → PlayUiSound(kind 0x13)。**分歧点**：焦点字节 Query 组件 U（`FocusSubjectByte` seam）、PlayUiSound id U。 |
| `hint_clutter_focused_edit_condition` | 0x141c3d890 | **重建** | batch5 | profile 0x200!=0 → hint_done=1；ModeJustChanged 工具==3 → hint_done=1；写 TutorialHintComplete(9)；焦点字节==9 → PlayUiSound(kind 0x13)。同上分歧点。 |
| `hint_roof_shape` | 0x142284ed0 | **重建**（部分） | batch5 | 事件处理逐事件分支判等（字节==0 → hint_done；字节!=0 → event_byte + TutorialHintComplete(7)，reviewer 修正完成路径不再晚一帧）；末位展示门控（profile 0x1ff==0 && hint_done && event_byte!=1 && AppMode==0 && raycast && geo_src==2 && roof_ok）结构还原。**分歧点**：geo_src_to_edit_subject / legacy_raycast 命中码（ui_edit_focus 域未落库）→ `geo_src_seam()` 返回 0 → 展示写面不触发，登记 U。 |
| `hint_terrain_layering` | 0x142283cc0 | **重建** | batch6 | count++/`1<count` 门控；字节==1 && byte8==0 → TutorialHintComplete(6)；末位 AppMode==10 && 0x1f9==0 && count>1 && 有实体 && byte8==0 → Steam+Key 写 DisplayHintRequest(0x600, id 6)。**分歧点**：事件载荷字节 U（seam 0 → 完成分支不触发）。 |
| `hint_terrain_focus` | 0x142283980 | **重建** | batch6 | 字节==2 → hint_done + TutorialHintComplete(5)；末位 AppMode==10 && 0x1fb==0 && 有实体 && !hint_done → make_focus_or_camera 写 DisplayHintRequest(0x100, id 5)。**分歧点**：载荷字节 U。 |
| `hint_terrain_steepness` | 0x142283640 | **重建** | batch6 | 字节==0 → hint_done；!=0 → event_byte + TutorialHintComplete(4)；末位 AppMode==10 && 0x1fa==0 && hint_done && event_byte==0 && 有实体 → make_lbm_drag 写 DisplayHintRequest(0x400, id 4)。**分歧点**：载荷字节 U。 |
| `hint_undo` | 0x142284220 | **重建** | batch6 | just_pressed → TutorialHintComplete(id 2)；SomethingWasErased 事件 → flag=1；焦点字节==2 → timer+=dt、>10 → 完成；flag==1 && 0x1fe!=1 → Steam.adjacent(Steam).adjacent(Key).alternate 写 DisplayHintRequest（hint id 2=Undo、placement Cursor(6)、flags 0x200；hint id@+0x49 U，注释/RESTORE/Rust 统一 id 2）。**分歧点**：just_pressed 动作 / 键码槽(0xfc/0x104) U（seam）。 |
| `hint_unlink_windows` | 0x14227d760 | **重建** | batch6 | 0x1fd!=0 返回；字节!=0 → TutorialHintComplete(8)；==0 → Steam+Key 写 DisplayHintRequest(0x800, id 8)。**分歧点**：载荷字节 / 键码 U。 |
| `hint_deselect_tool` | 0x1422853c0 | **重建** | batch6 | 0x1f4!=0 返回；reader A（ToolWasJustUsed）→ state=1；reader B（0x28B seam）→ state=0 + 字节==0 → hint_done + TutorialHintComplete(0)；AppMode==6 && shape!=0 && count!=0 → state=1；is_tool_selected && !hint_done && !pressed && state==1 → make_focus_or_camera 写 DisplayHintRequest(id=shape+6 或 6)。**分歧点**：reader B/C/D/E / pressed 动作 U（reader D/E 计数 seam 保持 0）。 |
| `hint_disable_snapping` | 0x14227e140 | **重建** | batch6 | 首事件字节==0 && 0x1fc==0 && !hint_done && AppMode==6 → Steam+Key 写 DisplayHintRequest(shape+6, 0x300)；!=0 → TutorialHintComplete(3)+hint_done。**分歧点**：载荷字节 / 键槽(0x120/0x128) / Steam index U。 |
| `hint_stairs` | 0x142285a20 | **重建**（骨架） | batch6 | 五阶段状态机（0x201..0x205）骨架：阶段 5 放置→可脱离→0x1100 / 完成 17；阶段 1 计数→完成 13 / 0xd00；阶段 2 完成 14 / 0xe00；阶段 3 AppMode==4 → 完成 15 / 0x100；阶段 4 脱离→完成 16 / 0x1000。**分歧点**：reader→事件精确映射 / 计数器语义 / `*piVar8==0xb&&[1]=='?'` 状态条件 U → 6 事件按序接入、0xd00/0xe00 展示以 seam 门控。 |
| `hint_camera_progress_update` | 0x14227bd30 | **判等**（既有，事件 seam 切换） | batch4 | 事件读面由 `CameraMoved` seam 切换为 `NotifyHintSystemCameraMoved`（{tag u32, value u32}）；tag 2→rotate、1→zoom(value)、其它→pan；跨 1.0 写 TutorialHintComplete(0xa/0xb/0xc)。 |

## 交付差异摘要

- **判等 4**（本批落库）：tutorial_completion / hints_startup / hint_camera_enable_condition
  / hint_camera_progress_update（事件读面由 seam 切换为 NotifyHintSystemCameraMoved 后复核）。
- **重建 22**：exponential_back_off、reset_hints、prompt_rollercoaster、
  hint_camera_display_condition、camera_pan/rotate/zoom、hint_highlight_shape、
  resolve_steam_impl、make_focus_or_camera_binding_hint、make_lbm_drag_binding_hint、
  hint_focused_edit_condition、hint_clutter_focused_edit_condition、hint_roof_shape（部分）、
  hint_terrain_layering/focus/steepness、hint_undo、hint_unlink_windows、hint_deselect_tool、
  hint_disable_snapping、hint_stairs（骨架）。
- **未实现 6（阻塞，登记分歧点）**：hint_orchestrator（closure$0 排序键 + DisplayHintRequest
  完整布局 U）、hint_draw/render_hints（mei DSL 未落库）、hint_position_update（相机矩阵/
  几何未接线）、hint_focused_edit_hint / hint_clutter_focused_edit_hint（RaycastWorld
  legacy_raycast 命中码 + geo_src_to_edit_subject 跳转表未落库）。
- 资源布局对齐：DeselectToolHintConditions / FocusedEditHintConditions / HintTerrainConditions
  补 **u64 count@+0** 字段（native 10B/9B 布局，hints_startup 与各系统读面一致）；
  DisplayHintRequest 补 `flags: u32`（+0x48 区，各 hint 写面 0x100..0x1200）；
  Steam 变体载荷由 seam 改为 `ActionOriginGlyphQuery`；`HintType::from_id` 新增。
- **placement 判读纠错（reviewer-wave3-onboarding）**：DisplayHintRequest @+0 字节 =
  **placement**（6 Cursor / 10 ScreenCenter），非 hint id；hint id 在 @+0x49（相机族
  0xa/0xb/0xc 判等；其余 hint 的 id@+0x49 嵌入 CONCAT 未取证 → 本重建以 I 级目标 hint
  值承载，登记 U）。各 hint_* 写面 placement 默认 Cursor(6)、prompt_rollercoaster 为
  ScreenCenter(10)。

## REVIEWER FIX（reviewer-wave3-onboarding 复核，2026-10-17）

1. **hint_roof_shape 完成路径**：原用 `state.event_byte` 代理当前事件字节 → 完成晚一帧，
   非判等。**已修**：按反编译逐事件分支（字节==0 → hint_done；字节!=0 → event_byte +
   TutorialHintComplete(7)），结构判等。
2. **hint_undo "id 6" vs "id 2"**：模块注释/RESTORE/Rust 三者统一 hint id 2（=Undo，
   placement Cursor(6) 为 @+0 字节值；id@+0x49 U）。
3. **prompt_rollercoaster "id 10"**：@+0 字节 10 = placement ScreenCenter（非 id）；**已修**
   置 `with_placement(HintPlacement::ScreenCenter)`，hint id@+0x49 U 登记。

## 留给 reviewer 的核对点

1. exponential_back_off 对 A 读者类型 U（候选 DisplayHintRequest）与对 B 读者
   （ModeJustChanged 0x28B，首载荷 u32∈{1,2}）——B 判等、A 待 core 补事件类型。
2. hint_camera_display_condition 的 AppMode/is_fence 门控与 fresh_0x14227c910.c 逐分支
   对照（bVar19/bVar20/iVar2==10 组合）；PlayUiSound(kind 0x13) 写面 seam。
3. hint_highlight_shape 的 HintType∈{1,2} 判定（u32 回绕）与 +0x11 字节。
4. 条件资源 count 字段布局与各系统读面逐字节一致（deselect/terrain/focused）。
5. 各 hint_* 事件载荷字节 seam 默认 0 的展示路径——core 补载荷字段后按 fresh 反编译替换。
6. hint_stairs 骨架的 reader→事件映射（6 个 Stairs 事件按序接入，精确对应待 core 复核）。

## 未解决/阻塞

- hint_orchestrator / hint_draw / render_hints / hint_position_update / hint_focused_edit_hint
  / hint_clutter_focused_edit_hint：仍缺 closure$0 排序键、mei DSL、相机矩阵几何、
  geo_src 链——非本批 core 类型解锁面，登记上游缺口。
