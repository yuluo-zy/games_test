# REVIEW — systems/onboarding wave3 onboarding（reviewer-wave3-onboarding）

> 2026-10-17。独立核验 restorer-wave3-onboarding 对 `crates/systems/onboarding` wave3
> 解锁落库是否真正还原 B 版原二进制业务逻辑。
> 方法（用户 2026-10-16/17 硬标准）：**逐函数 fresh 反编译**（Ghidra shared worker，
> `get_functions fields=signature,decompiled_code`）→ **独立复述**原函数完整控制流 →
> 与落盘 Rust body **逐语句对照**。本审查独立证据：`reviews/onboarding/evidence/
> fresh_reviewer/*.c`（8 函数，与 restorer `evidence/fresh_*.c` 及 batch4 逐字节一致，
> 证实证据非伪造）；`reviews/onboarding/jobs/reviewer-fresh-decomp.json` 可重放。
> 身份：B patched EXE tiny-glade.exe（sha256 cf048e27…6323）。

## 0. 结论摘要

- **判等 4：全部复验通过**（tutorial_completion / hints_startup /
  hint_camera_enable_condition / hint_camera_progress_update）。独立 fresh 反编译
  逐语句对照无分歧（1 处事件字节偏移注记见 §4.4）。
- **重建 22：抽查核验通过**（flags/Steam/布局/seam 诚实）；**1 处子声明需修正**：
  hint_roof_shape 的"完成事件判等"过高——实际为近似（完成事件晚一帧触发，见 §2.1）。
- **未实现 6：诚实**——stub 均为空体 + 分歧点登记，未发现以近似/待取证混过。
- **需修 3**：① hint_roof_shape 完成路径降级为近似；② hint_undo 模块注释 "id 6"
  与 RESTORE 表 "id 2"、Rust 实际写 id 2 不一致；③ prompt_rollercoaster "id 10=
  CameraPan" 解释待核（element[0]=10 更可能是 placement，见 §5）。
- **编译/测试核验**：`cargo check -p system-onboarding` **0 错误**；`cargo test -p
  system-onboarding` **9/9 通过**（复跑确认）。
- **证据真实性**：审查者独立 fresh 反编译 8 函数（0x14227c230/0x14227b450/
  0x14227c780/0x14227bd30/0x142284800/0x14227c910/0x141c1c8d0/0x14227bb70）与
  restorer `evidence/fresh_*.c`（7 函数）逐字节一致；0x14227bd30 与 batch4-camera.txt
  一致（仅尾部分隔符差异）。restorer 的 fresh 证据为真实 Ghidra 输出。
- 无过度断言面：判等 4 的"逐语句判等"成立；重建 22 内各 seam/U 分歧点登记与
  反编译一致，未夸大。

## 1. 判等 4（独立 fresh 复述 → 与 Rust 逐语句对照）

### 1.1 tutorial_completion（0x14227c230，判等）

原逻辑（fresh，逐事件 0x10 步长、id@+8）：18 路 switch id∈0..0x11 →
GameProfile 对应偏移置 1；**无 default**（id≥0x12 跳过置位但仍执行后续
EventWriter<WriteSettingsFileCmd> 写）；每事件写一次命令。
对照 Rust `tutorial_completion`：`TutorialOp::from_id(event.id)` → None（≥18）时
`mark_tutorial_op_complete` 跳过，但 `writer.write(WriteSettingsFileCmd)` 无条件执行。
**逐 case 判等**：COMPLETION_FLAG_OFFSET 18 项与 fresh switch 写面偏移
（0x1f4/0x1f5/0x1fe/0x1fc/0x1fa/0x1fb/0x1f9/0x1ff/0x1fd/0x200/0x1f6/0x1f7/0x1f8/
0x201/0x202/0x203/0x204/0x205）**逐项一致**；country-core `TutorialOp::
completion_flag_offset` 与 `GameProfile::mark_tutorial_op_complete` 映射一致。
命令事件为 `WriteSettingsFileCmd`（fresh grow_one 泛型名证实），非早前表
"SaveGameProfileCmd" 表述——RESTORE 已修正，属实。

### 1.2 hints_startup（0x14227b450，判等）

原逻辑（fresh，Commands::insert_resource 序列 12 项 + 逐命令字节）：
- DeselectToolHintConditions/FocusedEditHintConditions/HintTerrainConditions 含
  **u64 count@+0**（closure 载荷 +8=0）、hint_done@+8=profile[0x1f4]/[0x1f5]/[0x1f9]、
  Deselect/FocusedEdit state@+9=0；
- HintTerrainSteepnessConditions 2B=[0x1fa]×2；HintDisableSnappingConditions
  1B=[0x1fc]；UndoHintConditions 8B（+0 u32=0、+4 u8=0）；HintRoofShapeConditions
  2B=[0x1ff]×2；FocusedClutterHintConditions 2B（[0x200]、+1=0）；
- CameraHintsState state=(0x1f6&&0x1f7&&0x1f8)?2:0，counters 清零；
- CameraHintProgress 进度初值=各相机完成标志?1.0:0.0（0x3f800000/0），阈值
  0.7/0.7/60（0x3f333333/0x3f333333/0x42700000）；StairsHintConditions 清零。
对照 Rust `hints_startup`：**12 项插入序一致**，count@+0/hint_done@+8/state@+9 布局
逐字段一致，CameraHintsState/CameraHintProgress 初值与常量一致。**判等**。
（注：HintTerrainConditions 的 event_byte@+9 原生未显式写，Rust 显式置 0 —— 微小
近似，不影响本判等面。）

### 1.3 hint_camera_enable_condition（0x14227c780，判等）

原逻辑（fresh）：state!=0 返回；鼠标事件（CursorMovedScreenSpace）有 →
counters[1]=0、counters[0]+=dt；无 → counters[1]+=dt、`<10` 才 counters[0]+=dt；
相机事件（NotifyHintSystemCameraMoved）有 → counters[3]=0、counters[2]+=dt；无 →
counters[3]+=dt；`counters[2]>0.1 ? (counters[3]>1.5 || counters[0]>100) :
counters[0]>100` → state=1。
对照 Rust `hint_camera_enable_condition`：**四分支逐语句一致**。相机事件分支的
bVar14=false 使 `counters[3]>1.5` 恒假（[3] 刚归零），Rust 合并公式等价。**判等**。

### 1.4 hint_camera_progress_update（0x14227bd30，判等）

原逻辑（fresh+batch4）：先快照 prev=当前；逐相机事件 tag 2→rotate+=dt/
rotate_threshold、tag 1→zoom+=value/zoom_threshold、其它→pan+=dt/pan_threshold；
跨 1.0 边沿（prev<1 且 cur>=1）→ TutorialHintComplete(id 0xa/0xb/0xc)。
对照 Rust `hint_camera_progress_update`：快照序、tag 分支、value 用于 zoom、三组
跨 1.0 判定与写面（id@+8、0x10 步长）**逐语句一致**。**判等**。
（注见 §4.4：事件 tag 字节偏移存在读面/写面不一致待核，不影响功能语义。）

## 2. 重建 22（抽查核验）

### 2.1 hint_roof_shape（0x142284ed0）—— 完成路径**非判等**，需降级

RESTORE 表"事件处理+完成事件（字节==1 → event_byte + TutorialHintComplete(7)）
判等"**过高**。fresh（batch5-focused.txt 0x142284ed0）：
- 事件字节==0 → 仅 `*pcVar8[0]`（hint_done）=1；
- 事件字节!=0（==1）→ `pcVar8[1]`（event_byte）=1 + TutorialHintComplete(7)；
- 读的是**当前事件**的载荷字节（`cVar2=*(char*)(event+8)`）。
Rust `hint_roof_shape` 却以 `state.event_byte`（资源字段，=上一事件字节）代理当前
事件字节：`if state.event_byte != 0 { complete }` → **完成事件晚一帧**（首事件
字节==1 时本帧不触发，次帧才触发；单事件则永不触发），且字节==0 分支也被置
event_byte=1。属**近似**非判等。展示链门控（geo_src_seam()==2 恒假 → 不触发）登记
U，诚实。**建议**：状态改为"重建（近似）"，模块头/表声明同步降级。

### 2.2 exponential_back_off（0x142284800，重建，核验通过）

fresh 逐段对照：Local+=dt；鼠标事件→归零+`profile[0x1f0]+=dt`，无事件仅 local<30
累加；两对 (0x1e0/0x1e4, 0x1e8/0x1ec) 冷却缺省 720（0x44340000），事件→计时归零+
`cooldown=clamp(old*1.5,720,10800)`+写命令；对 A 读者类型 U（候选
DisplayHintRequest，stride 未取证）；对 B 读者 ModeJustChanged（0x28 步长）首载荷
u32∈{1,2}（`*piVar11-1U<2`）；flag 分支 local<10 才 timer+=dt、clamp[0,cooldown]
（cooldown<0 → f32::clamp panic 路径）、timer>=cooldown → 写命令+清 10B 条件资源+
清标志。Rust 结构与 fresh **一致**。分歧点登记诚实。
**注记**：对 B 读者 fresh 取**末个**未消费事件载荷，Rust 用 `.read().next()`（首个）
——多事件帧下选择不同，未登记（次要）。

### 2.3 reset_hints（0x141c1c8d0，重建，核验通过）

fresh：首事件载荷字节==0 → 0x1f4..0x206 全置 1（0x0101010101010101）+ state=2；
!=0 → 12 资源复位 + 0x1f4..0x206 清零 + **0x1e0..0x1ef 两对计时/冷却清零**
（0x1f0 累加器不动）+ state=0；最后写 WriteSettingsFileCmd。Rust `reset_hints`
分支序与 `GameProfile::set_all_hint_flags`/`reset_hint_flags`（0x1e0..0x1ef 清、
+0x1f0 不动）一致。触发事件类型 U（`ResetHintsCommand` seam）登记诚实。**核验通过**。

### 2.4 prompt_rollercoaster（0x14227bb70，重建，核验通过——id 解释待核）

fresh：事件→`impl$31::from()`（packed_set_plus_one）→
`alternate<Option<KeyCode>>` → 写 DisplayHintRequest（步长 0x58，flags 0x1200@+0x48）。
Rust `prompt_rollercoaster`：`Steam(packed_set_plus_one(0)).alternate(None)` +
flags 0x1200。结构与 flags **一致**。Steam index / 键码 U 登记诚实。
**注记**（§5）：fresh element[0]=10 更可能是 **placement（ScreenCenter=10）** 而非
RESTORE 所称 "id 10=CameraPan"（其余 hint 写面 element[0]=6=Cursor）。若成立，
Rust 的 `HintType::CameraPan` 与 `placement: Cursor` 两处都需在布局落库时复核。

### 2.5 hint_camera_display_condition（0x14227c910，重建，核验通过——过度近似注记）

fresh：state==1 门控；三完成标志（0x1f6==1 && 0x1f7!=0 && 0x1f8==1）→ state=2；
AppMode 门控（iVar2==1/2、==6 且 shape∈{1,2}、is_fence 时另含==10）；逐条
(0xa/0xb/0xc) 按进度/duration 门控写 DisplayHintRequest（flags 1@+0x48、
placement 6@+0、id@+0x49）+ PlayUiSound(kind 0x13)+清标志。
Rust：state/三标志/AppMode 门控结构与 fresh 一致；flags=1、id 0xa..0xc 一致；
duration 递减与 PlayUiSound 完成写面以 seam 表达（`let _ =`）诚实。
**注记**：Rust **无条件下发全部 3 个 DisplayHintRequest**，原函数逐条按
`duration>0 || progress<1.0` 门控（完成态后停止）——过度近似，完成态下仍持续写
请求。已按重建 seam 登记，但建议在表/注释补一句"写面未复现门控"。

### 2.6 其余重建项（核验通过，简要）

- camera_pan/rotate/zoom_hint（batch4）：profile 0x1c0/0x1bc 字形模式 ==1/2 分支、
  Steam(packed_set_plus_one/packed_set)+together/adjacent/alternate 组合、图串
  "hint/mouse"（10）/ "hint/rmb"（8）/ "hint/rmb-move"（0xd）/ "hint/mmb" 长度核对
  一致。Steam index/尾项 U 诚实。
- hint_highlight_shape（batch2）：eased>0.9 且 HintType∈{1,2}（`*piVar1-1U<2` 与
  `*piVar1-6` 回绕）→ roof_entity 有效 → HighlightRoofCmd/HighlightWallCmd tag 2。
  +0x11 字节/WallId 源 U 登记诚实。
- resolve_steam_impl（fresh_0x14227e5e0）：case 0xc9 `get_action_origin_glyphs`
  None→0xc4(Empty)、len 0/1/≥2 → Empty/单元素/0xca(Alternate)；0xca/0xcb/0xcc 列表
  递归。Rust `resolve_steam` 逐分支一致。
- make_focus_or_camera_binding_hint（batch5 0x141c3de00）：profile 0x1c4 字形模式
  ==1 [hint/mouse,hint/rmb]、==2 [hint/mouse,hint/mmb]、其它 MouseButton；
  `impl$31::from` 前置。Rust 一致。
- make_lbm_drag_binding_hint（batch5 0x142285270）：
  together(Steam(packed_set_plus_one),Steam(packed_set)).alternate(Graphic
  "hint/lmb-move" 0xd)。Rust 一致。
- hint_focused_edit_condition / hint_clutter_focused_edit_condition（batch5）：
  profile 0x1f5/0x200 门控、ModeJustChanged 工具∈{1,2}（`*piVar5-1U<2`）/
  ==3、TutorialHintComplete(id 1/9)、PlayUiSound(kind 0x13)。结构与 fresh 一致；
  焦点字节 Query 组件类型 U（seam）、PlayUiSound id U 诚实。
- hint_terrain_layering/focus/steepness（batch6）：flags 0x600/0x100/0x400 逐字节
  核验；count 门控/载荷字节 seam 诚实。
- hint_undo（batch6）：just_pressed→complete(2)、擦除事件→flag=1、焦点字节==2 →
  timer+=dt、>10→complete、flag==1 && 0x1fe!=1 → Steam.adjacent(Steam).adjacent(Key)
  .alternate 写 flags 0x200。结构与 fresh 一致；just_pressed/键槽(0xfc/0x104) seam 诚实。
- hint_unlink_windows（batch6）：0x1fd!=0 返回；载荷字节 seam 分支；flags 0x800
  核验一致。
- hint_deselect_tool（batch6）：reader A（ToolWasJustUsed）→state=1、reader B（0x28B
  seam）→state=0+载荷==0→完成(0)、reader D/E 计数 seam；AppMode==6&&shape!=0&&
  count!=0 → state=1；is_tool_selected&&!done&&!pressed&&state==1 →
  make_focus_or_camera 写 DisplayHintRequest(id=shape+6 或 6)。结构与 fresh 一致。
- hint_disable_snapping（batch6）：flags 0x300 核验一致；载荷字节/键槽(0x120/0x128)/
  Steam index seam 诚实。
- hint_stairs（batch6，骨架）：五阶段状态机（0x201..0x205），flags
  0xd00/0xe00/0x100/0x1000/0x1100 逐字节核验一致；reader→事件映射/计数器/状态条件
  （`*piVar8==0xb&&[1]=='?'`）U 诚实。

## 3. 未实现 6（诚实性核验）

| 函数 | stub 形态 | 分歧点登记 |
|---|---|---|
| hint_orchestrator 0x14227fc30 | 空体（参数 `let _`）| closure$0 排序键 U、DisplayHintRequest 完整布局 U |
| hint_draw 0x141c51a60 | 空体 | mei/toolbar DSL 未落库 |
| render_hints 0x141c52700 | 空体 | mei DSL 未落库 |
| hint_position_update 0x141c50510 | 空体 | 跳转表+相机矩阵/几何未接线 |
| hint_focused_edit_hint 0x14227dd60 | 空体 | RaycastWorld::legacy_raycast 命中码 + geo_src 跳转表未落库 |
| hint_clutter_focused_edit_hint 0x141c3db80 | 空体 | 同上（geo_src_w_deco_clutter）|

均未伪造文本/实体增删/渲染逻辑。**诚实**。

## 4. 重点 seam/载荷/布局核验

### 4.1 事件载荷字节 seam 常量 0
`event_payload_byte_seam() -> u8 { 0 }`（lib.rs）：terrain_layering（==1 完成不触发）、
unlink（!=0 完成不触发、==0 展示触发）、disable_snapping（==0 展示触发、完成不触发）、
deselect（reader C ==0 → 完成触发）、terrain_focus/steepness 等按此建模。常量 0
与各 hint 的"展示路径"建模一致，RESTORE 明确"core 补载荷字段后按 fresh 反编译替换"。
**诚实**（展示分支 = 原字节==0 语义，完成分支被抑制）。

### 4.2 DisplayHintRequest flags
全部与 fresh/batch 反编译 **逐字节核验**：prompt_rollercoaster 0x1200、camera display 1、
terrain 0x600、terrain_focus 0x100、terrain_steepness 0x400、undo 0x200、unlink 0x800、
disable_snapping 0x300、stairs 0xd00/0xe00/0x100/0x1000/0x1100。**一致**。
（Rust `DisplayHintRequest.flags: u32`@+0x48 区与原生一致。）

### 4.3 Steam 载荷
`packed_set(x)=x<<8`、`packed_set_plus_one(x)=(x<<8)+1` 与 `impl$30::from`
0x1409e8d80 / `impl$31::from` 0x1409e8d90 一致（steam.rs tests 覆盖）；各 hint 用
`steam_query(packed_set_plus_one(0))` = 0x001（digital 下标 0）。Steam 动作 index U
（占位 0）登记诚实。resolve_steam 载荷已由 seam 改为 `ActionOriginGlyphQuery`，
与 `BindingHintDisplay::Steam` 载荷语义一致。

### 4.4 资源布局 u64 count@+0
DeselectToolHintConditions/FocusedEditHintConditions/HintTerrainConditions 的
`{u64 count@+0, u8 hint_done@+8, u8 state/event_byte@+9}` 与 hints_startup fresh
（closure 载荷 +8=0、+0x10/+0x11 字节）**逐字节一致**；steepness {u8,u8}、
disable_snapping {u8}、undo {u32,u8}、stairs {u64,u32} 亦与 fresh 一致。

### 4.5 事件字节偏移注记（不影响判等面）
hint_camera_progress_update 读面将相机事件 tag 读于 element+0、value@+4；而写面与
tutorial_completion 读面（id@+8）一致暗示 bevy `EventInstance={id u64@0, event@8}`
→ tag 应 @+8。onboarding.rs 注释 `{tag@+8,value@+0xc}` 与后者一致，与读面 @+0
冲突。**功能语义不受影响**（Rust `{tag,value}` 建模正确），但需按 bevy fork
EventInstance 布局确认后统一注释。

### 4.6 其他注记
- GameProfile Rust 结构体内存布局与原生字节偏移不同（字段注释记录原生偏移
  0x1f4/0x1e0/0x1c8/0x000 等；ron 序列化字段序 5 项匹配原生 serialize 序）。逻辑
  重建与 ron schema 兼容；字节级互操作（若有）需注意。
- exponential_back_off 对 B 读者取末个事件载荷 vs Rust `.next()` 首个（多事件帧）。

## 5. 需修清单

1. **hint_roof_shape 完成路径**：非判等，为近似（`state.event_byte` 代理当前事件
   字节 → 完成晚一帧/单事件不触发）。RESTORE 表"完成事件判等"降级为"近似"；模块头
   注明延迟语义。
2. **hint_undo id 文档不一致**：模块注释"写 DisplayHintRequest（id 6）" vs RESTORE
   表"id 2" vs Rust 实际写 `HintType::Undo`(2)。fresh element[0]=6 为 placement
   （Cursor）、id@+0x49=0x02 与 "id 2" 一致；"id 6" 为残留/误抄，需统一。
3. **prompt_rollercoaster id/placement 解释**：RESTORE "id 10=CameraPan" 待核——
   element[0]=10 更可能是 placement（ScreenCenter）。DisplayHintRequest 布局落库
   时复核；若成立，Rust `HintType::CameraPan` 与 `placement: Cursor` 均需修正。

## 6. 编译/测试

- `cargo check -p system-onboarding`：**0 错误**（复跑；country-core 9 warnings 与
  timberframe 无关错误与他组在途，本批未触碰）。
- `cargo test -p system-onboarding`：**9/9 通过**（quantize + 组合子 +
  cleanup + HintType 值）。

## 7. 证据文件

- 本审查 fresh：`reviews/onboarding/evidence/fresh_reviewer/fresh_reviewer_0x*.c`
  （8 函数）；job `reviews/onboarding/jobs/reviewer-fresh-decomp.json`。
- 复核用既有：`onboarding/evidence/fresh_*.c`（wave3）、`reports/interaction/
  onboarding/evidence/results/batch4-camera.txt`、`batch5-focused.txt`、
  `batch6-rem.txt`。
