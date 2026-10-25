# REVIEW — B-interaction：clutter/onboarding/wall-constructor 逐函数独立复核

> 角色：**reviewer-B-interaction**。日期：2026-10-17/18。**只读复核，未改 src**。
> 范围：`planning/2026-10-17-evidence-audit-interaction.md` §3.1 B 清单中
> **clutter 1 + onboarding 25 + wall-constructor 4 = 30 个函数**。
> 方法（用户 2026-10-16/17 硬标准）：逐函数 **fresh 反编译**（Ghidra `--worker shared`，
> B patched EXE `tiny-glade.exe`，sha256 `cf048e27…6323`；`get_functions fields=signature,
> decompiled_code,callees`；反编译丢失浮点累计/跳转块处补 `disassemble_function` 与
> radare2 只读反汇编）→ **独立复述**原生控制流 → 与 Rust body **逐语句对照** →
> 按**分歧检查表 8 条**逐条检查 → verdict。
> 证据落盘：`reports/reviews/B-interaction/evidence/{dec,disasm,results}/`
> （30 函数 fresh 反编译、8 函数逐指令反汇编、跳表字节、批任务可重放）。

**判定口径**：`判等` = 分支/循环/调用/数据流逐条对应，仅余已登记 U 且不改实现行为；
`需修` = 找到具体语句/门控/数据源/载荷分歧，须修；`未实现` = 主体缺失。

---

## 0. 结论速览（30 项）

| # | 模块 | 函数 | VA | verdict | 核心结论（一句） |
|---|---|---|---|---|---|
| 1 | clutter | `populate_autoclutter_regions` | 0x1422d6ed0 | **需修** | 结构/槽表/过滤链在，但 `select_mesh` 越界错位、live 路径权重源、逐记录→逐格近似 |
| 2 | onboarding | `resolve_steam_impl` | 0x14227e5e0 | **判等** | 分支/递归/空-单-Alternate 逐句一致；0xcd 哨兵 panic 未建模（限定） |
| 3 | onboarding | `hint_highlight_shape` | 0x141c50120 | **需修** | 缺组件 `+0x11 != 0` 门控；WallId 源应为组件 `+8`（Rust 恒 0） |
| 4 | onboarding | `hint_position_update` | 0x141c50510 | **需修** | case 6/10、case 8 取 max-x 角、夹取/lerp 判等；case 7/8 的放置点 Y 与 default 跳表未实现 |
| 5 | onboarding | `exponential_back_off` | 0x142284800 | **需修** | 计时/冷却/门控/写面判等；reader 应取末事件并耗尽，Rust 取首且留余 |
| 6 | onboarding | `reset_hints` | 0x141c1c8d0 | **需修** | 全置 1/12 资源复位/提示区清零判等；state 为 u32 2/0 非 bool；reader 末事件+耗尽 |
| 7 | onboarding | `hint_camera_display_condition` | 0x14227c910 | **需修** | state/三标志/AppMode 门判等；逐 id 的 progress/duration 写面门控与 PlayUiSound 未复现 |
| 8 | onboarding | `camera_pan_hint` | 0x14227d000 | **需修** | 三段图标应为**单个** Graphic（3 图标 +0.7/100）；Row 四键取自 profile；默认键源 `+0x1c2` |
| 9 | onboarding | `camera_rotate_hint` | 0x14227d2f0 | **需修** | 同上三图标聚合；Steam 索引 1/1；默认键 `+0x1be`；尾项应为两 Key 的 adjacent 再 alternate |
| 10 | onboarding | `camera_zoom_hint` | 0x14227d680 | **需修** | Steam 索引 0x10/0x11；尾项 Graphic("hint/scroll",100) 非 MouseButton::Middle |
| 11 | onboarding | `hint_focused_edit_condition` | 0x14227da70 | **需修** | 门控/完成 id 1/焦点字节==1/PlayUiSound 0x13 判等；ModeJustChanged 应取末事件+耗尽 |
| 12 | onboarding | `hint_focused_edit_hint` | 0x14227dd60 | **判等** | 跳表 1→Wall(0x258≥2 return)/2→Roof、写面 0x0101/0x0100 逐指令核对；legacy_raycast 壳 seam（限定） |
| 13 | onboarding | `hint_clutter_focused_edit_condition` | 0x141c3d890 | **需修** | 门控/完成 id 9/焦点字节==9 判等；reader 末事件+耗尽 |
| 14 | onboarding | `hint_clutter_focused_edit_hint` | 0x141c3db80 | **判等** | UiState+0x18==2/raycast miss/geo_src 判别 3/写面 0x0100 逐句一致（限定壳 U） |
| 15 | onboarding | `make_focus_or_camera_binding_hint` | 0x141c3de00 | **需修** | 双图标单 Graphic +0.7/100 判等；默认 MouseButton 取自 profile+0x1c6 非 Left |
| 16 | onboarding | `hint_roof_shape` | 0x142284ed0 | **需修** | 逐事件 0/非0 分支结构在；展示门控缺 legacy_raycast，且 geo_src seam 恒 0 永不展示 |
| 17 | onboarding | `make_lbm_drag_binding_hint` | 0x142285270 | **需修** | 第二 Steam 应为 packed_set(1)；Graphic 应为 ["hint/lmb","hint/lmb-move"] +0.7/100 |
| 18 | onboarding | `hint_stairs` | 0x142285a20 | **需修** | 阶段 5 门控极性反；counter 源/状态条件(mode==0xb&&'?')/timer/事件载荷门均未还原 |
| 19 | onboarding | `hint_terrain_layering` | 0x142283cc0 | **需修** | 展示门/写面判等；count 仅 byte==0 时 +1（Rust 每事件+1）；载荷字节 seam |
| 20 | onboarding | `hint_terrain_focus` | 0x142283980 | **需修** | 展示门/写面判等；完成分支依赖载荷字节==2（seam 恒 0） |
| 21 | onboarding | `hint_terrain_steepness` | 0x142283640 | **需修** | 展示门/写面判等；完成分支依赖载荷字节非 0（seam 恒 0） |
| 22 | onboarding | `hint_undo` | 0x142284220 | **需修** | 计时/完成(2)/flag/写面 0x200 判等；just_pressed 与两键槽(0xfc/0x104)为 seam |
| 23 | onboarding | `hint_unlink_windows` | 0x14227d760 | **需修** | 逐事件/写面 0x800 判等；完成分支依赖载荷字节非 0（seam 恒 0） |
| 24 | onboarding | `hint_deselect_tool` | 0x1422853c0 | **需修** | 缺 reader C 事件存在门；reader D/E count++ 未实现；reader 末事件+耗尽 |
| 25 | onboarding | `hint_disable_snapping` | 0x14227e140 | **需修** | 写面/placement/0x300 判等；应取末事件+耗尽；键槽(0x120/0x128) seam |
| 26 | onboarding | `prompt_rollercoaster` | 0x14227bb70 | **需修** | placement 10/id 0x12/flags 0 判等；Steam 索引应 0x14，alternate 键取自 profile+0x15c |
| 27 | wall | `ArchWalker::new` | 0x14074c2d0 | **需修** | 字段偏移/0.1 常量对；实参 ABI 不匹配（字段0=R8D、字段1=XMM3、字段2=XMM1，签名少参数） |
| 28 | wall | `calculate_wall_path_segments_impl` | 0x14122ff90 | **需修** | 锚点/adjustment/cap/regular 两阶 min 判等；NaN min 语义、bounds panic、u 投影空间周期有差 |
| 29 | wall | `circle_categorize` | 0x14124af60 | **判等** | 半径早退/阈值/网格/inside·in_wall/DIVSS+UCOMISS+SETC 逐指令一致；FIX-G1 已落实 |
| 30 | wall | `BrickRowSine::new` | 内联 0x140769760 | **判等** | random_f 四值公式（12.9898/43758.547/×1000）逐句一致；无宿主调用点（限定） |

**三态：判等 5 / 需修 25 / 未实现 0。**

---

## 1. clutter（1）

### 1.1 `populate_autoclutter_regions` 0x1422d6ed0 — **需修**

fresh 全量 120KB（2958 行，`dec/0x1422d6ed0_populate_autoclutter_regions.c`）复核：

- kind switch（记录 `+0x41`，`switch(*(char*)(local_138-7))`）case 0/2/3/4 建
  9/25/25/37 槽 `{表指针,权重}`（stride 0x10），case 1 独立处理器，万圣节跳过；
  每槽 `[entry+0x30]==-1 → 权重 0`。Rust `KIND_MESH_SLOTS` 表与之逐槽对照（kind 0/2/3/4
  权重、1.2/1.5/2.5/2.0 等常量）一致。
- mesh-type 跳表（`DAT_14304382c`，记录 `+0x47`）0/1 加权随机、2 跳过、3/4 固定
  +0x25/+0x26、7 计数、错误路径。Rust `select_mesh` 分派骨架一致，但见下 FIX。
- 加权选中（fresh 行 2047-2065）：`uVar25` 从 -1 起，**先加权再 `idx++`**，`while(acc<=r)`；
  选中槽就是 `uVar25`（即刚累加的槽）；`local_160 <= uVar25` 才 panic_bounds_check。
  **Rust `select_mesh` 在 `idx += 1` 后断言 `idx < len` 并返回 `slots[idx]` → 整体后移一格，
  且选中末槽时错判越界 panic**（实为 off-by-one，非 NaN/边界微差）。
- 落点（fresh 2070-2100）：`fVar68/fVar54 = record@-3/-2 + w*f@-5...`、
  `maxps(0.01, Δ) → floorf → xvy`；基准来自**区域记录字段**，非格中心。
- 过滤：`legacy_raycast`（mask 0x30fb）结果==2 走水面，4 角 bilinear(130) 求和
  clamp[0,1]，阈值 0.0；Rust `wet = ray_hit && water_sum>0` 为 seam 近似。

**FIX-BI-01**（见 §5）：`select_mesh` off-by-one；`weighted_mesh_landing` 用
`r<=acc`（应为 `acc>r` 严格）且用 `MeshGroupWeights` 而非 `KIND_MESH_SLOTS` 权重；
kind/mesh_type 由每格 `halloween?4:0 / 0` 近似（原为逐记录 +0x41/+0x47），
dry 落点基准用格中心而非记录 +0x20/+0x24。结构层近似已登记 U，但上述两处为可实现的具体错误。

---

## 2. onboarding（25）

### 2.1 `resolve_steam_impl` 0x14227e5e0 — **判等（限定）**
fresh switch：default 深拷贝原样返回；0xc9 → `get_action_origin_glyphs`，
None→0xc4、collect 后 len 0→Empty、len 1→单元素（元素 tag==0xcd 时 unwrap_failed）、
len≥2→0xca(Alternate)；0xca/0xcb/0xcc 逐元素递归并保留列表 tag。Rust
`resolve_steam` 逐分支一致。限定：0xcd 未建模（枚举无此哨兵）；closure_env_1 的
Graphic size/color 未取证（U）。

### 2.2 `hint_highlight_shape` 0x141c50120 — **需修**
fresh：逐实体 `anim.eased(+4)>0.9` && HintType∈{1,2}（`*piVar1-6` 回绕 + `-1U<2` 同时成立）
&& **`*(char*)(piVar1+0x11)!=0`** → `PublicWalls::roof_entity(wall)`；Some → 写
HighlightRoofCmd（payload=组件的 qword@+8, color 2, stride 0x18）；随后**无条件**写
HighlightWallCmd（同 wall, ty=2）。Rust 的 eased/类型/两条写面一致，但：
① 完全省略 `+0x11` 门控（注释自认）；② WallId 恒 `WallId(0)`，原版取 HintType 组件 +8。

### 2.3 `hint_position_update` 0x141c50510 — **需修**
fresh：`WindowSize::as_vec2` 的 **XMM1=height** → margin20/30；switch placement
6=Cursor `(x+2*margin20,y)`、7=world point（eye_right 外推 + world_space_to_screen +
margin）、8=Rectangle2d 四角投影取 **屏幕 x 最大角并携带其 y**、10=窗口中心；公共块
`delta=size-position`、`upper=(window-pos2)-margin30`、NaN-aware maxps/minps 夹取、
`time_continuous_lerp(rate 10)`。Rust 6/10、8 的 max-x、夹取/lerp 逐句一致；case 7/8 的
native 放置点 Y（`PreviewCircle+0x20` / `PreviewRectangle+0x28`）以 0.0 占位，default
二级跳表 0x142f14844 未实现（已登记）。

### 2.4 `exponential_back_off` 0x142284800 — **需修**
fresh：`Local+=dt`；鼠标 reader **有事件→耗尽+Local=0+profile[0x1f0]+=dt，无事件 local<30 才加**；
A/B 冷却 `==0 && !NaN → 720(0x44340000)`；A reader 有事件→耗尽+timer_a=0+
`clamp(old*1.5,720,10800)`+WriteSettingsFileCmd；flag→local<10 才 timer+=dt 并
clamp[0,cooldown]（cooldown<0 走 f32::clamp panic），timer>=cooldown→命令+清 10B 资源+清标志；
B reader 取**末事件** `*piVar11-1U<2`（tool∈{1,2}）→同上。Rust 公式/常量/写面一致；
差异仅在 reader：Rust `.next()` 取**首**且不耗尽（多事件帧跨帧重复触发），原版取**末**并耗尽。

### 2.5 `reset_hints` 0x141c1c8d0 — **需修**
fresh：取末事件并耗尽；payload byte(+8)==0 → profile 0x1f4..0x206 全置 1
（`0x0101010101010101` 写 0x1f4/0x1fc + dword 0x203）且 state=2；!=0 → 12 资源复位
（terrain 9B、steepness 2B、snapping 1B、deselect 10B、focused 10B、
camera_progress 0/0/0 + 0.7/0.7/60、camera_state 20B、undo 5B、roof/clutter 2B、stairs 12B）、
profile 0x1f4..0x206 清零 + 0x1e0/0x1e8 清零（0x1f0 不动）、+0x206=0、state=0；末尾
WriteSettingsFileCmd。Rust 分支与资源集一致；差异：state 原生 u32 2/0，Rust bool；
terrain `event_byte@+9` 原生未写而 Rust default 清零；reader 首/末+耗尽差异。

### 2.6 `hint_camera_display_condition` 0x14227c910 — **需修**
fresh：state!=1 返回；三标志 `0x1f6==1 && 0x1f7!=0 && 0x1f8==1` → state=2；AppMode 门
（tag 6 且 shape∈{1,2}；is_fence 时另含 tag10）返回；随后**逐条 0xa/0xb/0xc**：
按阈值(<1)与 progress(>=1) 门控 duration（初值 0.4）——展示时重置 0.4，否则
`duration -= dt`；`duration<=0 && completed-bit` → PlayUiSound(0x13)+清 bit；
仅在 `duration>0 || progress<1` 时 `camera_*_hint` + 写 DisplayHintRequest（placement 6、
flags=1、id=0xa/0xb/0xc）。Rust 的 state/三标志/AppMode/三条写面参数一致，但**无条件写全部 3 条**，
duration/PlayUiSound 完成写面仅以 seam 空置 → 完成态仍持续写请求（过度近似）。

### 2.7 `camera_pan_hint` 0x14227d000 — **需修**
fresh 逐指令（`disasm_2_0x14227d000.txt`）：
`impl$30::from`=packed_set(0) → `alternate<[Option<KeyCode>;4]>`，四键载荷**从
profile+0x00..0x30 拷贝（4×12B）**，非 `[None;4]`；glyph mode=`word[profile+0x1c0]`；
==1：一次 `__rust_alloc(0x30)` 写 **3 个字符串**（"hint/mouse"(10)/"hint/rmb"(8)/
"hint/rmb-move"(0xd)）→ **单个 Graphic**，`+0x18=0.7(0x3f333333)`、`+0x1c=100.0(0x42c80000)`，
再**一次** `alternate<BindingHintDisplay>`；==2 同构（mmb/mmb-move）；default →
`alternate<MouseButton>(word[profile+0x1c2])`。
Rust：`[None;4]`（占位）、三个独立单图标 Graphic（values=0）、默认恒 Left → 三处载荷/结构分歧。

### 2.8 `camera_rotate_hint` 0x14227d2f0 — **需修**
fresh：`CL=1 → impl$31(plus_one(1))` + `CL=1 → impl$30(packed_set(1))` → `together`；
glyph mode=`word[+0x1bc]`；
==1/2 同上（**单个 3 图标 Graphic +0.7/100**，一次 alternate）；default
`alternate<MouseButton>(word[+0x1be])`；随后 `adjacent<Option<KeyCode>>`（self 为
profile+0x48 的 12B Key，item 为 profile+0x54 的 12B Key）→ 最后
`alternate<BindingHintDisplay>` 把该 Adjacent 挂到 with_mouse。
Rust：三独立单图标 Graphic、默认 Left、`with_mouse.adjacent(None)`（缺外层 alternate 与两个
profile 键源）→ 结构与载荷均不符。

### 2.9 `camera_zoom_hint` 0x14227d680 — **需修**
fresh：两次 `impl$31` 且 **CL=0x10 / 0x11** → Steam 动作码 plus_one(0x10)/plus_one(0x11)，
`adjacent`；尾项构造 **Graphic**：`Vec<String>` 单串 `0x1430385f8` len 0xb（"hint/scroll"）、
`+0x18=0x8000000000000000`（-0.0）、`+0x1c=100.0`，再 `alternate<BindingHintDisplay>`。
Rust：索引 0/0（应为 0x10/0x11）、尾项 `MouseButton::Middle`（应为 Graphic）→ 两处分歧。

### 2.10 `hint_focused_edit_condition` 0x14227da70 — **需修**
fresh：`profile[0x1f5]!=0 → hint_done@+8=1, return true`；ModeJustChanged 取**末事件并耗尽**；
`*piVar5-1U<2`（tool∈{1,2}）→ hint_done=1，否则 `hint_done&1==0 → return false`；
无事件 → return；随后写 TutorialHintComplete(payload byte@+8=1)；Query 焦点组件首字节==1 →
写 PlayUiSound（dword 0x13，stride 0x20）；return true。Rust 全部门控/id/声音载荷一致
（`FocusSubjectByte` seam 已登记），唯一分歧是 reader 取首且不耗尽。

### 2.11 `hint_focused_edit_hint` 0x14227dd60 — **判等（限定）**
fresh + radare2 跳表核对：跳表 0x1430386a0（6×i32）目标 = {0,3,4,5}→0x14227e113 epilogue、
1→0x14227e09f、2→0x14227e0c1。0x14227e09f：`PublicWalls::get(wall)` →
`test rax; je unwrap_failed` → `cmp dword [rax+0x258],2; jae epilogue`；公共写面：
`make_focus_or_camera_binding_hint(profile)`、subject 16B、word@+0xb8=**0x0100**
（flags=0/id=1）、word@+0x80=**0x0101**（placement+0x10）、EventWriter::write。
前置门控（hint_done@+8、reader 耗尽+count++/`4<count`@+9、Query 非空、state!=0、
AppMode tag==0、legacy_raycast +0x30!=2）逐句核对一致。Rust 与之逐语句对应；
限定：`legacy_raycast` 为 core 壳 seam（native `cmp [rsp+0xa0],2` vs Rust toi 位型/is_miss）。

### 2.12 `hint_clutter_focused_edit_condition` 0x141c3d890 — **需修**
fresh 结构与 2.10 同构：`profile+0x200` 门、ModeJustChanged **末事件+耗尽**、tool==3、
完成 id 9、焦点首字节==9、PlayUiSound 0x13。Rust 门控/载荷一致，reader 取首不耗尽。

### 2.13 `hint_clutter_focused_edit_hint` 0x141c3db80 — **判等（限定）**
fresh：hint_done@+0 门；ClutterWasMoved **耗尽**→state@+1=1；UiState+0x18==2 返回；
state!=0 且 AppMode tag==0；raycast(mask 0x100, +0x34==2 miss)；geo_src_w_deco_clutter
判别==3；make_focus + 写面（placement 判别 3、word@+0x10=0x0100、flags/id=0x0100）。
Rust 逐句一致（`count()` 耗尽、discriminant 门、Subject{word_0x10:0x0100}）。限定：
raycast/geo_src 为 core 已落库实现，其内部 ABI 语义另行验收。

### 2.14 `make_focus_or_camera_binding_hint` 0x141c3de00 — **需修**
fresh：`CL=1 → impl$31 = packed_set_plus_one(1) = 0x0101`；`word[+0x1c4]`==1/2 →
**单个** Graphic（2 串：mouse+rmb / mouse+mmb）`+0.7/+100.0` → `alternate<HintGraphicParams>`；
default `alternate<MouseButton>(word[profile+0x1c6])`。Rust 的 0x0101、单 Graphic、2 串、
0.7/100 一致；default 用 `MouseButton::Left`，原版读 profile+0x1c6 → 一处数据源分歧。

### 2.15 `hint_roof_shape` 0x142284ed0 — **需修**
fresh：逐事件（**全部耗尽**）`byte==0 → hint_done=1`；`byte!=0 → event_byte@+1=1 +
TutorialHintComplete(byte@+8=7)`；末位门控 = `profile[0x1ff]==0 && hint_done &&
event_byte!=1 && AppMode tag==0 && legacy_raycast 结果!=2 && geo_src==2 &&
PublicWalls::roof_entity Some && Roof::is_flat()==0` → make_lbm + 写 DisplayHintRequest
（placement 6、flags/id 0x0700）。Rust 事件分支结构一致但 payload seam 恒 0；展示门**缺
legacy_raycast 条件**，且 `geo_src_seam()==2` 恒假 → 永不展示；roof_ok 用 Query 任意
!is_flat 取代 roof_entity 定向查表。

### 2.16 `make_lbm_drag_binding_hint` 0x142285270 — **需修**
fresh：`CL=0 impl$31 = plus_one(0)` + `CL=1 impl$30 = packed_set(1)` → `together`；随后
`__rust_alloc(0x20)` 写 **2 串** `0x143038ea0` len 8（"hint/lmb"）+ `0x143038ea8` len 0xd
（"hint/lmb-move"）→ **单个 Graphic** `+0.7/+100.0` → `alternate<HintGraphicParams>`。
Rust：第二 Steam 用 `packed_set(0)`（应 1）；Graphic 只有 `"hint/lmb-move"` 且 values=0
（应双图标 +0.7/100）→ 三处分歧。

### 2.17 `hint_stairs` 0x142285a20 — **需修**
fresh（`dec/0x142285a20_hint_stairs.c` + 行 90-160/254-460/533-640）：
- 顶部从 stack reader（+0x30）**每事件 counter++**并耗尽；`AppMode tag==0xb && byte+1=='?'(0x3f)`
  时 `timer += dt`。
- 0x205：**placed reader 有事件 → TutorialHintComplete(17)**；placed 无事件而
  can_detach reader 有事件 → DisplayHintRequest(0x1100, id 17, placement 6, Steam+None)。
- 0x201：counter!=0 → complete(13)；否则 `(mode==0xb&&'?') && timer>1.0` → 写 0xd00
  （id 13，Steam+2 图标 Graphic）。
- 0x202：reader 有事件且 counter!=0 且事件 payload==0 → complete(14)；否则
  `(mode==0xb&&'?') && counter>2` → 0xe00（id 14，2 图标）。
- 0x203：AppMode tag==4 → complete(15)；否则 0x201!=0 且 0x202==1 且 reader 有事件 →
  写 0x100（id 1）。
- 0x204：detached reader 有事件 → complete(16)；否则 can_detach 有事件 → 0x1000（id 16，2 图标）。
Rust 阶段 5 **极性反**（要求 placed 有事件后才查 can_detach，且只有 placed 时完成 17）；
counter 取自 placed 且仅在阶段 1 累加（原生在顶部对固定 reader 每事件累加）；
状态条件 seam 恒 false；阶段 2/3 的事件/载荷门缺失；各 reader 用 `.next()` 不耗尽。

### 2.18 `hint_terrain_layering` 0x142283cc0 — **需修**
fresh：逐事件（耗尽）；`byte==1 && hint_done@+8==0` → complete(6)+hint_done=1；
`byte==0` → **count++**（仅此分支）；末位 `mode==10 && profile[0x1f9]==0 && count>1 &&
实体非空 && hint_done==0` → Steam(plus_one)+alternate(None)→ 写 0x600（placement 6、id 6）。
Rust 展示门/写面一致；count 每事件都 +1（应仅 byte==0）；payload seam。

### 2.19 `hint_terrain_focus` 0x142283980 — **需修**
fresh：逐事件 `byte==2 → hint_done=1 + complete(5)`；末位 `mode==10 &&
profile[0x1fb]==0 && 实体非空 && hint_done==0` → make_focus 写 0x100（placement 6、id 1）。
Rust 门/写面一致；payload seam 使完成分支不可达。

### 2.20 `hint_terrain_steepness` 0x142283640 — **需修**
fresh：逐事件 `byte==0 → hint_done=1`；`byte!=0 → event_byte=1 + complete(4)`；末位
`mode==10 && profile[0x1fa]==0 && hint_done && event_byte==0 && 实体非空` →
make_lbm 写 0x400（placement 6、id 4）。Rust 门/写面一致；payload seam。

### 2.21 `hint_undo` 0x142284220 — **需修**
fresh：`just_pressed()` → complete(2)；SomethingWasErased reader **耗尽**→flag@+4=1；
焦点首字节==2 → timer+=dt、`timer>10` → complete(2)、再次 just_pressed → complete(2)；
末位 `flag==1 && profile[0x1fe]!=1` → `Steam(plus_one).adjacent(Steam(plus_one))` →
`adjacent<Option<KeyCode>>(profile+0xfc 12B)` → `alternate<BindingHintDisplay>(第二键)` 写
0x200（placement 6、id 2）。Rust 方法链/门/写面一致；`just_pressed` 恒 false、两键槽 seam
None（读不到 profile+0xfc/+0x104）→ 行为差异。

### 2.22 `hint_unlink_windows` 0x14227d760 — **需修**
fresh：`profile[0x1fd]!=0` 返回；逐事件（耗尽）：`byte!=0 → complete(8)`；`byte==0` →
Steam(plus_one)+alternate(None) → 写 0x800（placement 6、id 8）。Rust 逐事件循环/写面一致；
payload seam 恒 0 → 完成分支不可达。

### 2.23 `hint_deselect_tool` 0x1422853c0 — **需修**
fresh：`profile[0x1f4]!=0` 返回；reader A（ToolWasJustUsed）**耗尽**→state@+9=1；
reader B **取末事件+耗尽**→state@+9=0；**reader C 有事件且 B 事件 payload==0** →
hint_done@+8=1 + complete(0)；reader D **或** E 有事件（各自耗尽）→ count@+0 += 1；
`mode tag==6 && shape!=0 && count!=0` → state@+9=1；`is_tool_selected() && !hint_done &&
!pressed() && state==1` → make_focus + 写面 placement=`6+(tag==6 ? shape : 0)`、
flags/id=0。Rust：缺 reader C 存在门（只查 B 非空 + seam==0）；D/E count++ 未实现
（恒 0 → 该门不触发）；reader A/B 用 `.next()` 不耗尽；pressed/事件类型为 seam。

### 2.24 `hint_disable_snapping` 0x14227e140 — **需修**
fresh：reader **取末事件+耗尽**；`byte==0` → `profile[0x1fc]==0 && hint_done==0 &&
mode==6` → Steam(plus_one)+alternate(profile+0x120/+0x128 12B) → 写 0x300
（placement=6+shape、id 3）；`byte!=0` → hint_done==0 时 complete(3)，hint_done=1。
Rust 门/placement/flags 一致；reader 取首不耗尽；键槽 seam None；Steam 索引 U。

### 2.25 `prompt_rollercoaster` 0x14227bb70 — **需修**
fresh：EventReader 有未读事件（耗尽）→ `CL=0x14` 调 `impl$31` = plus_one(0x14)；
alternate 的键元素为 **profile+0x15c 的 12B**（`[RAX+0x164]` dword + `[RAX+0x15c]` qword）；
写 DisplayHintRequest：placement dword=0xa(ScreenCenter)、word@+0x48=0x1200（flags 0/id 0x12）、
stride 0x58。Rust 的 placement/id/flags/结构一致；Steam 用 0x101（应 0x1401）、键用 None
（应 profile+0x15c）。

---

## 3. wall-constructor（4）

### 3.1 `ArchWalker::new` 0x14074c2d0 — **需修**
fresh：`in_RCX[2]=XMM1`、`in_RCX[3]=0x3dcccccd(0.1)`、`[0]=R8D`、`[1]=XMM3`。
字段偏移与 0.1 常量判等；但实参寄存器说明原生签名至少含 RDX 一个前置整数参数与
XMM1/2/3 三个浮点参数，且**字段 2 来自第一个浮点参数（XMM1）、字段 1 来自第三个浮点
（XMM3）**。Rust `new(kind:u32, extra:f32, brick_len:f32)` 的 ABI（RDX/XMM1/XMM2）与之不匹配，
字段/参数映射未证。

### 3.2 `calculate_wall_path_segments_impl` 0x14122ff90 — **需修**
fresh（782 行）：flags 扫描 `&0xf !=0` 为锚点（<2 锚点→空）；相邻锚点区间经 LoopRange
`index_at`/`LoopIdx::index`（越界 **panic_bounds_check**）；`low_y = min(0, prepared_y)`、
regular 时 `low_arch=min(f32::MAX, arch_height)`；`adjustment=(abs(low_y)-0.75).max(0)`、
`cap=low_arch+0.92`；每 index `height=segment_values+adjustment`，regular 时
`min(height, (arch_height - prepared_y) - 0.96000004)`（0xbf75c290）再 `min(cap)`；
边界以 `CurveU::sample_at_coord` + `wall_space::impl$1::subtract/project_clamp` 重算
position/u；输出 0x68B 段。Rust `materialize_wall_segments` 的锚点/两阶 min/cap/0x68 布局
对应；差异：
① `min_source(a,b)=if a>b {b} else {a}` 在 NaN 时返回 a（机器 blend 在 a=NaN 时取 b、
b=NaN 时取 b）——NaN 语义不同；
② `index_at` 返回 `Err`（函数提前返回），原生 panic_bounds_check（检查表 #1）；
③ u 投影用本地 `periodic_subtract/project(closed)` 且周期硬编码 1.0，原生调用
`WallSpace{is_loop,length}` 的 subtract/project_clamp（周期=墙长）；
④ 边界 position 以两点差 lerp 模拟 `sample_at_coord`，仅在折线/阈值坐标语义下等价，未逐句证明。

### 3.3 `circle_categorize` 0x14124af60 — **判等**
fresh 反汇编逐段核对：`UCOMISS 1.75,radius; JBE main`（NaN 亦走 main）；小半径
`CMP [R8+0x10],0; SETZ → walls.is_empty()`；`(radius-1.75)/1.25 → MAXSS 0 → MINSS 1`；
`smoothstep`、`threshold=s*0.6+(1-s)*0.3`（0x142925908/0x1429258d8）；`ceilf(2r)` 饱和
u64 序列（2^63/2^64 常量）→ `step=2r/n`；双层 `0..=n`；`is_point_inside`；
墙循环 stride 0x50 / `[+0x38]` id / `PublicWalls::get` 空→unwrap_failed /
`get_wall_shape`→`is_pos_inside`；`ADDSS inside/in_wall`；末 `DIVSS + UCOMISS + SETC`
（ratio<threshold 或 unordered）。Rust `maxss/minss` helper 与 `ratio<threshold ||
is_nan()` 逐语义一致；28B shape 偏移（tag@0、circle center@4/radius@0xc/rotation@0x10、
rect axis@4/center@0xc/size@0x14）与 `shape_contains` 一致。**FIX-G1 已落实并独立复核通过**。

### 3.4 `BrickRowSine::new`（内联于 0x140769760）— **判等（限定）**
fresh from_curve：`r0 = frac(sin((x*10+phase)*12.9898)*43758.547)`（行 919-921）；
`r1` 同式（923-925）；`p0 = frac(sin((r0+r0)*12.9898)*43758.547)*1000`（927-929/970）；
`p1` 同式（930-931/971）；`sine_wave_deform`：`s=0.5+0.5*sin(x*0.2)`、
`0.01<=width` 门、`0.06*sin((r0*s+1)*x+p0)`、`!closed` 门、`0.06*sin(x*(s*r1+1)+p1)`
（1196-1206）。Rust 公式逐句一致（`fract`=x-trunc(x)）。限定：`new` 无 Rust 调用点，
x/y 实参是宿主已缩放坐标（`fVar65*local_118` / `fVar65*local_190`），两相位来源 U。

---

## 4. 分歧检查表 8 条命中汇总

| # | 模式 | 命中条目 |
|---|---|---|
| 1 | panic-vs-return 反 | `calculate_wall_path_segments_impl`（index_at→Err vs panic_bounds_check）；`populate` select_mesh 末槽误 panic；`resolve_steam` 0xcd 未建模 |
| 2 | 门控极性反 | `hint_stairs` 阶段 5；`hint_roof_shape` 缺 raycast 门；`hint_highlight_shape` 缺 +0x11 门；`hint_deselect_tool` 缺 reader C 门；`hint_camera_display_condition` 缺 duration/progress 门 |
| 3 | 载荷平面近似 | 全部 `event_payload_byte_seam` 函数（terrain×3/unlink/disable_snapping/roof/stairs）；camera_pan/rotate 三图标聚合；make_lbm 双图标；Row/键槽 12B；populate kind/mesh_type 逐格近似 |
| 4 | 事件/命令自造 | `ResetHintsCommand`、`DeselectReset`、`FocusSubjectByte`、`undo_just_pressed/keys`、`stairs_show_condition` seam（均已登记 U，但不可判等） |
| 5 | callee 转发错 | 未发现（camera_*/make_focus/PublicWalls::get/EventWriter::write 调用面一致） |
| 6 | 返回结构不符 | `ArchWalker::new` 签名/ABI；`HintResetState` bool vs 原生 u32 2/0 |
| 7 | 排序取最近 vs 取首个 | exponential_back_off、reset_hints、disable_snapping、两个 focused condition、deselect（B）、undo、stairs、鼠标 reader：原生取**末个未读并耗尽**，Rust `.next()` 取首且留余 |
| 8 | 偏移表/字段错位 | `hint_highlight_shape` WallId 组件+8；camera profile 键偏移 0x00..0x30/0x48/0x54/0x120/0x128/0x15c/0x1c2/0x1be/0x1c6；ArchWalker 字段-实参映射；`hint_position_update` 7/8 的 Y 偏移 |

## 5. FIX 清单（供 restorer 落实；修后需二次独立复核方可升 C）

| # | 级别 | 文件/函数 | 修改点 |
|---|---|---|---|
| FIX-BI-01 | 高 | `clutter/.../populate_autoclutter_regions.rs` `select_mesh` | 选中槽应为 `slots[idx-1].0`；越界断言判 `idx-1 < len`（与原生 `local_160 <= uVar25` panic 对齐） |
| FIX-BI-02 | 高 | 同文件 `weighted_mesh_landing` | 选中算子 `acc > r`（严格，非 `r <= acc`）；权重源用 `KIND_MESH_SLOTS`（wave2 ③），非 `MeshGroupWeights` |
| FIX-BI-03 | 高 | 同文件 逐记录分派 | 恢复逐区域记录（stride 0x48）kind@+0x41/mesh-type@+0x47；dry 落点用记录 +0x20/+0x24，非格中心 |
| FIX-BI-04 | 高 | onboarding 事件类型（country-core） | `NotifyHintSystem{Terrain,TerrainSteepness,RoofShape,Window,Snapping,Stairs*}` 补事件载荷字节；删除 `event_payload_byte_seam()` 常量，按事件字段实现各 hint 分支 |
| FIX-BI-05 | 高 | onboarding 各 reader | 原生 reader 语义=取**末个未读**并**耗尽**；`exponential_back_off`/`reset_hints`/`disable_snapping`/两个 condition/`undo`/`deselect`/`stairs` 改为 `.read().last()`（或 count 后取 last） |
| FIX-BI-06 | 高 | `hint_camera_controls.rs` camera_pan/rotate | glyph 1/2 路径改为**单个** `HintGraphicParams{[mouse,rmb\|mmb,(rmb-move\|mmb-move)], 0.7, 100.0}` 一次 alternate；Row 四键从 GameProfile+0x00..0x30（4×12B）读取；default MouseButton 取 +0x1c2/+0x1be；rotate 的两次 Steam 索引均应为 **1**（plus_one(1)/packed_set(1)），Rust 为 0/0 |
| FIX-BI-07 | 高 | `camera_rotate_hint` 尾项 | 改为 `with_mouse.alternate(Key(profile+0x48).adjacent(Key(profile+0x54)))`（两 12B 键） |
| FIX-BI-08 | 高 | `camera_zoom_hint` | Steam 索引 0x10/0x11；尾项 `Graphic{"hint/scroll", -0.0, 100.0}` |
| FIX-BI-09 | 高 | `hint_roof_shape.rs` make_lbm | 第二 Steam `packed_set(1)`；Graphic `["hint/lmb","hint/lmb-move"] +0.7/100.0` |
| FIX-BI-10 | 高 | `hint_roof_shape` 展示门 | 补 `legacy_raycast` 命中码 !=2；`geo_src_to_edit_subject` 实际调用（替换 seam）；roof 用 `roof_entity` 定向查 `Roof` 并 `!is_flat()` |
| FIX-BI-11 | 中 | `hint_highlight_shape` | 补组件 `+0x11 != 0` 门；WallId 取 HintType 组件 `+8` |
| FIX-BI-12 | 中 | `hint_position_update` | typed preview 资源补 `PreviewCircle+0x20` / `PreviewRectangle+0x28` Y；实现 placement default 跳表 0x142f14844 |
| FIX-BI-13 | 高 | `hint_camera_display_condition` | 实现逐 id 的 threshold/progress 门、duration(0.4) 递减、`duration<=0` 时 PlayUiSound(0x13)+清完成位；不得无条件写 3 条 |
| FIX-BI-14 | 高 | `hint_deselect_tool` | 补 reader C 事件存在门；实现 reader D/E count++；reader 末事件+耗尽 |
| FIX-BI-15 | 高 | `hint_stairs` | 阶段 5 极性修正（placed→complete 17；无 placed+can_detach→0x1100）；counter 顶部按固定 reader 每事件累加；状态条件 `mode==0xb && byte+1=='?'` + timer；阶段 2 payload==0 门、阶段 3 reader 门；精确 reader→事件映射 |
| FIX-BI-16 | 中 | `hint_undo` | 实现 just_pressed 动作源与 profile+0xfc/+0x104 两键槽 |
| FIX-BI-17 | 中 | `prompt_rollercoaster` | Steam plus_one(0x14)；alternate 键取 profile+0x15c（12B） |
| FIX-BI-18 | 中 | `make_focus_or_camera_binding_hint` | 默认 MouseButton 取 profile+0x1c6（core 补字段） |
| FIX-BI-19 | 中 | `ArchWalker::new` | 按 ABI 补前置整数参数与三个浮点参数，字段0=R8D、字段1=XMM3、字段2=XMM1、字段3=0.1；与调用点核对 |
| FIX-BI-20 | 中 | `calculate_wall_path_segments_impl` | NaN-aware min（a=NaN 取 b）；bounds 改为原生 panic；u 投影改用 `WallSpace{is_loop,length}` 的 subtract/project_clamp；核对 `sample_at_coord` 等价 |
| FIX-BI-21 | 低 | `reset_hints` | state 用 u32 2/0（非 bool）；terrain `event_byte@+9` 保持原样（勿由 default 清零） |
| FIX-BI-22 | 低 | `resolve_steam_impl` | 0xcd 哨兵单元素 unwrap_failed 语义（枚举补变体或显式 panic） |

## 6. 账目影响（相对 `2026-10-17-evidence-audit-interaction.md` §1/§4）

| 模块 | 复核前 B/C | 复核后 B/C | 变化 |
|---|---|---|---|
| clutter | 1 / 8 | 1 / 8 | 不变（populate 仍 B，已独立复核为需修） |
| onboarding | 25 / 22 | 22 / 25 | `resolve_steam_impl`、`hint_focused_edit_hint`、`hint_clutter_focused_edit_hint` 升 C；其余 22 维持 B（需修） |
| wall-constructor | 4 / 12 | 2 / 14 | `circle_categorize`、`BrickRowSine::new` 升 C；`ArchWalker::new`、`calculate_wall_path_segments_impl` 维持 B |
| **三模块合计** | **30 / 42** | **25 / 47** | 判等 +5，需修 25 |

> 说明：C 的 5 项均带限定（U/seam）已在各条注明；需修 25 项不因"已登记 U"而视为判等，
> 因为存在具体语句/门控/数据源差异（见 §4）。

## 7. 三态回执

- **判等 5**：`resolve_steam_impl`、`hint_focused_edit_hint`、`hint_clutter_focused_edit_hint`、
  `circle_categorize`、`BrickRowSine::new`（均附限定 U）。
- **需修 25**：clutter 1 + onboarding 22 + wall 2；FIX-BI-01..22。
- **未实现 0**：30 项均有可执行 body；无空壳。

### 证据索引（本会话 fresh）
- `evidence/dec/0x*.c`：30 函数 Ghidra fresh 反编译。
- `evidence/disasm/disasm_*.txt`：camera_pan/rotate/zoom、make_focus、make_lbm、prompt、
  circle_categorize、resolve_steam 逐指令。
- `evidence/results/{batchA,batchB,batchC,jumptable,disasm1,branch,r2branch}.json`：可重放原始报告。
- 关键裁决：跳表 `0x1430386a0` 24B = `735a24ff ff5924ff 215a24ff 735a24ff 735a24ff 735a24ff`
  → 1→0x14227e09f(Wall)、2→0x14227e0c1(Roof)、其余→epilogue；matches Rust。
