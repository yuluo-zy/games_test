# REVIEW — consume 三线（geo-src / hint / debugline）独立复核

> reviewer-consumers，2026-10-17 晚。方法：逐函数 **fresh 反编译 + fresh 反汇编** →
> 独立复述控制流/数据流 → 与 Rust body 逐语句/逐常量对照 → verdict。
> **未采信**任何 restorer 结论作为判等依据；restorer 报告只作交付面清单。
> 硬标准：逐个分支/调用/载荷/常量逐条判等；近似体、占位、跨模块类型替换一律降级。
> 红线遵守：shared Ghidra 只读；radare2 只读；未改 src。

## 0. 身份与证据（本次现场）

- B patched `tiny-glade.exe`，base `0x140000000`，sha256 `cf048e27…6323`（get_metadata）。
- Ghidra `--worker shared`（反编译/函数反汇编）；radare2 `--worker reviewer-consumers`
  直接打开 **B patched EXE**（非 RNE；`show_info` 确认 pe64/baddr 0x140000000）读数/读表/区间反汇编。
- 现场证据（可重放）：
  - `work/reviewer-consume/result-fresh-a.json` / `result-fresh-b.json` / `result-fresh-c.json`
  - `work/reviewer-consume/decomp-0x14227dd60.c`、`-0x141c3db80.c`、`-0x142269170.c`、
    `-0x14227fc30.c`、`-0x141c50510.c`、`-0x141be0ad0.c`、`-0x141be1080.c`、`-0x141ba9700.c`、
    `-0x141c3de00.c`、`-0x140a91ef0.c`
  - `work/reviewer-consume/disasm-0x14227dd60.txt` / `-0x141c3db80.txt` / `-0x142269170.txt` /
    `-0x141c50510.txt`（Ghidra 函数体）
  - `work/reviewer-consume/disasm2-0x*.txt`（0x141be0ad0/0x141be1080/0x141ba9700/0x14089d8b0/
    0x14089d9b0/0x14089dc00/0x14089dc60/0x14089dd50）
  - `work/reviewer-consume/result-r2-{a,b,c,d,e,f,g}.json`（区间反汇编 / 跳表 / 常量 / 字符串）

## 1. 结论表

| # | 函数 / 项 | verdict | 依据摘要 |
|---|---|---|---|
| 1 | `hint_focused_edit_hint` 0x14227dd60 | **需修** | 门控/跳表/写面逐条判等；**EventReader 语义不符**（FIX-4） |
| 2 | `hint_clutter_focused_edit_hint` 0x141c3db80 | **需修（低）** | 门控/UiState 门/raycast/geo/写面判等；EventReader 同 FIX-5（状态幂等，功能无差） |
| 3 | `ui_place_tree` geo_src 门 0x1422692b5 | **判等** | `+0x30==2` 跳过 / subject≠None 早退 / 顺序与原生一致 |
| 4 | `HintPlacement::Subject` 模型 | **判等** | 0x0101（Wall/Roof）/0x0100（DecoClutter）+ flags0/id1 逐字节 |
| 5 | `hint_orchestrator` 0x14227fc30 | **判等** | 收集/稳定排序(+0x49)/抑制位/首实体/无实体 spawn/同 id/异 id/冷却全对上 |
| 6 | `hint_position_update` 0x141c50510 | **需修 + 未实现** | 公共块/6/10 判等；**边距取 height 而非 width（FIX-1）**；**case8 取 argmax-x 而非 min（FIX-2）**；7/8 的 Y 仍 0.0 占位（未实现）；二级跳转表未做 |
| 7 | `display_glade_border` 0x141be0ad0（本批范围） | **判等** | 模式门/as_curve2/事件门/DebugLine 链(Red,4.0,ui_pass false)/show/perimeter 门/from_closure 记录逐条 |
| 8 | `display_glade_border::closure$1` 0x141be1080 | **判等（主体）** | env/载荷/builder 链/2 insert 顺序/0.7/返回 Vec1 全对上；跨模块类型 FIX-9 |
| 9 | `draw_water::closure$0` 0x141ba9700 | **判等** | env `{pos,scale}`/名字/载荷/position+scale 字段/顺序/返回 Vec1 |
| 10 | `make_focus_or_camera_binding_hint` 0x141c3de00（前批遗留，影响 #1/#2 显示载荷） | **需修** | Steam index 实为 1（FIX-6）；鼠标键取自 profile+0x1c6（FIX-7）；图标打包/0.7/100.0 结构不符（FIX-8） |

## 2. 三态回执

**判等 6 项**（#3 #4 #5 #7 #8主体 #9）· **需修 5 项**（#1 #2 #6 #8组件类型 #10）·
**未实现 2 项**（#6 的 7/8 放置 Y；#6 的 placement 二级跳转表 tags 0..5/7..9/11..）。

## 3. 逐函数复核

### 3.1 hint_focused_edit_hint 0x14227dd60

Fresh 反汇编事实（0x14227dd60..0x14227e126，211 条）：

- `CMP byte [RAX+8],0; JNZ 0x14227e113` → `hint_done@+8`。
- EventReader 读后：**未读>0 → 游标置 event_count**（`MOV [R10],R11`），再 `INC [RAX]`、
  `CMP R11,0x5; SETNC [RAX+9]` → `state = count>=5 = 4<count`。✔ Rust 数值表达式判等。
- Query 非空门：原生遍历**全部**条目，末项 item 指针 `TEST RBX,RBX; JZ` → 仅作非空判据（登记 I）。
- `CMP byte [RAX+9],0` / `CMP dword [*RCX],0`（AppMode==0）。
- `CALL 0x1408cc680`（legacy_raycast，**固定 mask 0xFB + max_toi f32::MAX**，未命中把
  **u32 2 原样写 +0x30**）；`CMP dword [RSP+0xa0],2; JZ` → 与 `toi.to_bits()==2` 位型等价。✔
- `CALL 0x140977f50`（geo_src_to_edit_subject，入参 `{handle 24B}`），跳表
  `0x1430386a0` 6 项（现场解码）：
  `[0]=→0x14227e113, [1]=→0x14227e09f, [2]=→0x14227e0c1, [3..5]=→0x14227e113`。✔ 与 Rust match 一致。
- Wall 分支 0x14227e09f：`PublicWalls::get(id)`（0x140a91ef0）→`TEST RAX; JE panic(0x14227e127)`；
  `CMP dword [RAX+0x258],2; JAE epilogue`。Rust `shape.tag()>=2`（core `WallShapeSnapshot::tag`
  即该 +0x258 首 u32）。✔
- 公共写面 0x14227e0c1：`make_focus_or_camera_binding_hint` → `MOVUPS subject16B→[rsp+0x70]`、
  `MOV word [rsp+0x80],0x101`（placement+0x10）、`MOV word [rsp+0xb8],0x100`
  （+0x48 flags=0 / +0x49 id=1）→ `EventWriter::write`（0x14227ba70，步长 0x58）。✔

**分歧（FIX-4）**：原生事件读取是“**未读>0 则一次性消费到 event_count**”，每帧最多
`count += 1`；Rust `shape_edit_events.read().next().is_some()` 每帧只推进 **1 个事件**。
若一帧内产生 n>1 个 `NotifyHintSystemShapeEditWasPerformed`，Rust 会在后续帧继续 +1
（事件双缓冲存活两帧），`4<count` 阈值可提前一帧触发，与原版不同。
正确写法：`.read().count() > 0` 或 `.read().last().is_some()`（均一次性耗尽）。

其余展示载荷依赖 §3.10 的 helper（前批遗留问题，见 FIX-6/7/8）。

### 3.2 hint_clutter_focused_edit_hint 0x141c3db80

Fresh 事实逐条对上：

1. `state.hint_done@+0`；2. EventReader 未读>0 → `state@+1=1`；3. `UiState+0x18==2 → return`
（0x140002450 = `lea rax,[rcx+0x18]; ret`，现场复核字节）；4. `state@+1==0`、`[*AppMode]==0`；
5. `CALL 0x1408cc870`（raycast，**mask=0x100 栈参 + XMM3=f32::MAX**），`CMP dword [RBP+0x64],2`
（结果 +0x34 判别）；6. `CALL 0x140978000`（geo_src_w_deco_clutter），`CMP dword [RBP-0x30],3`
仅 DecoClutter；7. `make_focus_or_camera_binding_hint`；placement `{tag=3@0, decorator_id@4,
+8 未初始化, word 0x0100@+0x10}`，+0x48 word=0x0100（flags0/id1）→ 事件写。✔

**分歧（FIX-5，低）**：同 FIX-4 的 reader 语义差异。本函数只把 `state@+1` 置 1（幂等），
且无计数器，故**可观察行为等价**；仍应改为一次性消费以逐语句判等。

### 3.3 ui_place_tree geo_src 门 0x142269170

Fresh 反汇编（0x142269170..0x1422695f6）：

- `CMP dword [*RCX],0x10`（Tree）且 `*0x14362c098` 指向字节 = 0（现场读 0x143634d90 = 0）。
- WidgetParams/sounds/`UiState::add("tree_canvas")`（len 0xb）→ 顺序与原版一致。
- `CALL 0x1408cc680` → `CMP dword [RBP+0xf8],2; JZ 0x1422692f6`（结果 +0x30 == 2 = miss 跳过 geo）。
- 命中：复制 handle 24B → `CALL 0x140977f50` → `CMP dword [RBP+0x178],0; JNZ 0x1422695f6`
  （subject≠None → 直接 return）✔ Rust `if !matches!(subject, None) return`。
- 之后才 `CursorTerrainRaycast::position()` → `GladeBorder::is_pos_inside`；界外/界内分支与
  clicked/is_active 顺序均与 Rust 一致；水下重映射常量现场核对：
  `0x1429df1c0=-2.5`、`0x142925904=1.0`、`0x142a8cc64=-1.2`，公式
  `(y/-2.5)*-1.2 + (1-y/-2.5)*0` ✔；PlaceTreeCmd 事件步长 0x18，写 `{event_meta, x,y, z}` ✔。

**结论**：geo_src 门逐语句判等。函数其余面（界内 signifier 写面：原生 `format!` +
`SignifierFactory::new` + 内层 `{1,1,Vec2,f32,f32}`，Rust 仍 `UiAppendSignifier::new(0,0,pos,1.0)`）
为**既有登记 U**，本轮未改、不判等；不影响本轮门控结论。

### 3.4 HintPlacement::Subject

- focused：placement+0x10 = **0x0101**；clutter：**0x0100**（0x141c3dce6）。
- 两者 +0x48 word 均为 **0x0100** → flags=0、id=1（FocusedEdit）。
- 判别 = EditSubject tag（1 Wall / 2 Roof / 3 DecoClutter）；Rust `native_tag()` 同表。✔
- 0x12..0x18 未初始化字节（padding）按 I 登记，Rust 结构化建模，不冒充。✔

### 3.5 hint_orchestrator 0x14227fc30

Fresh 反编译 + 反汇编（0x14227fd00 起）逐条：

1. `from_iter<&DisplayHintRequest, EventIterator>` 收集；`len<0x15` insertion / 否则 driftsort；
   比较器两处均为 `byte [+0x49]` 升序（0x1425e5b70 / 0x1425e57e0）→ Rust `sort_by(id)` 稳定排序。✔
2. Linger reader：全部未读消费；未读>0 → 抑制=1；否则 `mode==0xd`→1，否则
   `(mode==0x13)|is_open()`（0x14093fbc0）。✔
3. Toolbar reader：未读>0 时游标置 event_count，若 `!抑制` → return。✔
4. Query 取**首个**匹配（0x14227fd60 循环仅在 item 空时回跳，首命中即落出）→ `iter().next()`。✔
5. 无实体：sorted 空→return；`抑制 && id<0x12`→return；否则 spawn
   `(HintLayoutMemo,HintSpring,HintAnimation,HintText,HintType,HintPlacement)`；`tag==10`→InteractionPrompt；
   无条件 display clone insert；`+0x48!=0`→HintProgressBar。✔
6. 有实体：`抑制 && existing_id<0x12`→insert Marked；否则：
   同 id→insert placement + `+0x18!=0xc4` 时 insert display + remove 两个标记（handler=warn）；
   异 id→insert Marked；无请求→已有标记/冷却则 return，否则 `DAT_143038ae8[id]` 冷却。✔
7. 文本模板（现场读表）：
   - pieces `0x1430388f0` = `[("",0),(0x1430388ea,5)]` = `["", " 🎢"]`，key
     `0x1430388d8 "Rollercoaster.Ride"`（0x12）；
   - pieces `0x1430388b8` = `[("",0),(0x1430388b0,1)]` = `["", "/"]`，keys
     `"KeyBindings.Undo"` / `"KeyBindings.Redo"`（各 16）；
   - pieces `0x143038918` = `(0x143038910,5)` = `"Hint."`；名称表长度 `0x143038b38`
     = `8,9,4,15,16,12,7,9,13,7,9,12,10,17,18,11,14,20,19`，偏移表 `0x143038bd0` 指向
     `0x143038970..0x143038a53`，字符串现场核对与 Rust `hint_type_display_name` 19 项**逐项一致**；
     默认支随后 `to_uppercase()`。✔
8. 冷却表 `0x143038ae8` 19×f32 现场核对 = `[0,.3,0,0,.3,.3,.3,.3,0,0,.3,.3,.3,0,0,0,0,0,0]`。✔

**结论：判等**（保留既有 U：ToolbarUiIsInFocus 生产者接线；replace handler 差异；
id>18 名称串越界；见 §5）。

### 3.6 hint_position_update 0x141c50510

Fresh 反汇编/常量（本次重读）：

- 入口 `CALL 0x140949c10` = `WindowSize::as_vec2`，返回 **XMM0=width、XMM1=height**
  （0x140949c10 现场反汇编：`[rcx]→xmm0, [rcx+4]→xmm1`）。
  `DIVSS XMM1,1000`；`margin20 = XMM2 = 20*XMM1`；`margin30 = XMM1*30`。
  → **原生边距用 height，不是 width**。Rust 用 `dims.x`（hint_display_system.rs:241-242）。
- 公共块 0x141c515e0：`delta=size-position`；`target=raw+delta`；
  `upper=(w,h)-position_2-margin30`；NaN-aware `max(margin30,target)`/`min(upper)`
  与 `f32::max/min` 语义一致；`final=clamped-delta`；`spring.pos=final`；
  `initialized` 假→`target=final`；真→两次 `time_continuous_lerp(a,b,rate=10,dt=GameTime.delta)`
  （0x140c904e0 现场反汇编 = `a+(b-a)*(1-exp(-rate*dt))`，xmm7=10.0@0x1429345a8）。✔
- case 6（0x141c50acb）：`cursor + (2*margin20, 0)`（xmm6={2m,0}）✔。
- case 10（0x141c50ae0）：`as_vec2 * {0.5,0.5}`（xmm14@0x1429258e0={0.5,0.5}）✔。
- case 7（0x141c50dc6）：`xvy(out, PreviewCircle+0x10, [PreviewCircle+0x20])`
  → `{center.x, Y, center.y}`；`+ eye_right*( [PreviewCircle+0x18] + 0.5 )`
  （xmm15@0x142925870=0.5）；`world_space_to_screen_space(.,w,h,+0x80,+0x00)`；`+margin20`。✔
  **Y = PreviewCircle+0x20**，Rust 固定 0.0（未实现）。
- case 8（0x141c50af9）：`Rectangle2d::as_points3(Y=[PreviewRectangle+0x28])` 四角各投影，
  再以 **`ucomiss/seta` + `andps/andnps` 选取“x 更大的那个角”**（比较三次，y 随点携带），
  最后 `+(margin20,margin20)`。0x141c50bc3/0x141c50c8b/0x141c50cba 三处均为
  `ucomiss new.x, acc.x; seta` → 选中 new 当且仅当 `new.x > acc.x`（并列保留先者）。
  **不是分量 min**。Rust `min = min.min(screen)` 全错。
- default：第一级表 `0x142f14830` 把 (tag-6) 截到 idx（>5 → idx3）；
  idx3 落二级表 `0x142f14844`（按原 tag 索引；含 PublicWalls/多个代码分支与
  `unreachable!()`/`not yet implemented` 串 0x142f14560/0x142f145d0）。Rust `continue`。未实现。

**FIX**：
- FIX-1：`margin20/margin30` 改用 `dims.y`（连同两个单测期望：200×100 时
  cursor 期望 `(14,20)`；clamp 期望 `(-103,-203)`）。
- FIX-2：case 8 改为“投影四角 → 取屏幕 x 最大的角（并列取先）→ 用该角 x,y”后加 margin20。
- FIX-3：7/8 的 Y 需 core 类型补字段（`PreviewCircle+0x20` / `PreviewRectangle+0x28`，
  由 ui_create_circle/rectangle 写命中 y），否则该函数不得标判等。

### 3.7 display_glade_border 0x141be0ad0（本批范围）

Fresh 反汇编逐条：

- `CMP dword [*RCX],0x13; JZ epilogue`（Photomode 才跳过）→ Rust 一致。
- `as_curve2`(0x1411ff2b0) → `from_iter<Vec3, Map<Iter<Vec2>, closure$0>>`(0x141be6110) → 折线 Vec。✔
- EventReader 未读>0 → 复制折线（alloc/ memcpy）→ `DebugLine::new`(0x140b04e70)；
  `MOV dword [RBP-0x50],1` = `DebugColor::Red`（tag1）；`MOVSS XMM2,[0x142934694]=4.0`
  → line_width；`XOR R8D,R8D` → set_ui_pass(false)；**`MOV RDX,[RSI]; CALL 0x140b04f70` = show**。✔
- demo 门 `*0x14362c098` 指向字节 == 1（B 现场为 0）→ overlay。Rust 常量 false ✔。
- perimeter 门 `CMP byte [*config+0x121],0; JZ 画`：Rust `if !config.border_signifier_enabled` ✔（语义字段）。
- perimeter：`ShapeParameters::perimeter`；`format!("glade_border{}", GameTime+0x2c frame_count)`
  （pieces `0x142efa140 = (0x142efa130,12)` = "glade_border"；formatter 0x140d8d560 = u32 Display）；
  `alloc 0x30` 写 `{1,1, points.cap, points.ptr, points.len, perimeter}`；
  `SignifierFactory::new`(0x14089d8b0) (name 自有拷贝) + `UiSignifierStream::write`(0x140b2cf90)；
  随后释放临时名字串。✔
- Rust 以 `from_closure(&label, Arc::new(closure))` 表达同一记录（名字仍自有拷贝、
  闭包持折线 Vec/perimeter），语义等价；不再 `mem::forget`。✔

**结论：判等**（本批范围）。

### 3.8 closure$1 0x141be1080

Fresh 反汇编（155 条口径一致，本次重读关键段）：

- env（RDX）：len=`[+0x10]`、ptr=`[+8]`；`alloc 12*len`+memcpy；len==0 → dangling `0x4`。✔
- 载荷 `[RBP+0x40]=0x8000000000000002`，`+0x48=cap(=len)`、`+0x50=ptr`、`+0x58=len`；
  Rust 用 len 填 cap 槽：新分配 cap==len，等价。✔
- 顺序：`new`(0x14089d9b0) → `set_ui_pass(0)`(0x14089dad0) → `animation_duration(0.0)`(0x14089dc90)
  → `custom_shader("dashed_line")`（0x142efa180, len 0xb）(0x14089db00) → `build`(0x14089dd50)。✔
- 2 条 insert 原顺序：`DrawTopology` 值 `0x4000000000000001`（tag1=Alt, f32 2.0）；
  `DashedLineLength` = `[+0x18] * [0x1429cd030]=0.7`。✔
- 返回 `{cap,ptr,len} = {1, heap(entity), 1}`。✔

**FIX-9（跨模块）**：原生插入的是真实组件
`country_core::components::DashedLineLength`（TypeId 0x4782fcee8a8c578b）；crate 插入
`DashedLineLengthView`（另一 TypeId）。渲染消费面 `draw_callbacks_a.rs:583` 只读 View，
原生是同一类型生产/消费。两类型不互通 ⇒ 虚线长度**不会**进入渲染。应改为插真实
`DashedLineLength`（并让渲染读取面改读真实类型），否则本项只能算“主体判等 + 集成未闭合”。

### 3.9 draw_water::closure$0 0x141ba9700

Fresh 反汇编逐条：

- `alloc(8,8)` 返回 Vec 槽；`alloc(0x15,1)` + 拷 `"brush_circle_scribble"`（0x142eebc98，
  现场字符串核对）；载荷 `{0x8000000000000000, 0x15, ptr, 0x15, 未初始化, 未初始化}`；
- `animation_duration(0.0)` → `position(env+0)`（0x14089dc00：`[+0x7c]=[R8]`(x,y)、
  `[+0x84]=[R8+8]`(z)）→ `scale(env+0xc)`（0x14089dc60：`[+0x50]=1`、`[+0x54]=XMM2`）
  → `set_ui_pass(0)` → `build`（R8=mesh lib、R9=shader lib、栈=Commands）→ 返回
  `{cap=1, heap(entity), len=1}`。✔
- Rust 字段直写 `u7c/u84/flag50/f54` 与 0x14089dc00/c60 逐字段一致（core 无同名方法，登记 I）。
- 名字用 `&'static str` 而非原生每次 alloc 拷贝（I-3b），build 解析面前语义不变。

**结论：判等**（附带 I-3/I-3b）。

### 3.10 make_focus_or_camera_binding_hint 0x141c3de00（前批遗留）

Fresh 反汇编（0x141c3de00..）：

1. `MOV CL,1; CALL 0x1409e8d90`；0x1409e8d90 = `(cl<<8)+1`（现场反汇编）⇒ Steam 查询
   = **0x0101**（kind=1,index=1）。Rust `packed_set_plus_one(0)` ⇒ 0x0001（index 0）。**不符**。
2. `MOVZX R8D,word [profile+0x1c4]` 比 1/2；图形名：0x142f0ee08 "hint/mouse"(10) +
   0x142f0ee12 "hint/rmb"(8) / 0x142f0ee1a "hint/mmb"(8)。✔ 名称正确。
3. 非 1/2 分支：`MOVZX R9D,word [profile+0x1c6]` → `alternate<MouseButton>(r9d)`。
   **鼠标键取自 profile+0x1c6，不是常量 Left**；core `GameProfile` 无 +0x1c6 字段。
4. 1/2 分支只调用 **一次** `alternate<HintGraphicParams>`：item = 32B
   `{2, 0x142f0ee08..., 2, 0.7f(0x3f333333), 100.0f(0x42c80000)}`（两个图标串打包进
   **同一个** Graphic 载荷）。Rust 链两次 `alternate(graphic(...))` ⇒ 三个元素的 Alternate、
   size/color=ZERO/0。**结构/常量均不符**。

**FIX-6**：`STEAM_FOCUS_ACTION` 改 1（→0x0101）。
**FIX-7**：GameProfile 增加 +0x1c6（u16）字段并用作 MouseButton；缺字段前不得把“其它”分支判等。
**FIX-8**：按原生 HintGraphicParams 真实载荷（双图标 + 0.7/100.0）重建 Graphic 元素，
不要把两个图标拆成两个 Alternate 元素。需 re-analyst 定 HintGraphicParams 布局。

## 4. FIX 清单（交给 restorer；按严重度）

| FIX | 文件 | 内容 |
|---|---|---|
| FIX-1 | `hint_display_system.rs` | `margin20/margin30` 用 `dims.y`（窗口 height/1000×20,30）；同步两个单测期望 |
| FIX-2 | `hint_display_system.rs` | case 8 选 **投影屏幕 x 最大**的角（并列取先）并携带其 y，非分量 min |
| FIX-3 | core `PreviewCircle`/`PreviewRectangle`+`hint_display_system.rs` | 补 +0x20/+0x28 放置 Y 字段（ui_create_circle/rectangle 写命中 y）并在 7/8 使用；未落地前 7/8 标未实现 |
| FIX-4 | `hint_focused_edit.rs` | EventReader 改为一次性耗尽（`.read().count()>0` / `.last().is_some()`） |
| FIX-5 | `hint_clutter_focus_edit.rs` | 同上（低；状态幂等） |
| FIX-6 | `hint_clutter_focus_edit.rs` | `packed_set_plus_one(1)`（原生 cl=1 → 0x0101） |
| FIX-7 | core `GameProfile` + `hint_clutter_focus_edit.rs` | +0x1c6 鼠标键字段；替掉常量 `MouseButton::Left` |
| FIX-8 | `hint_clutter_focus_edit.rs` | `HintGraphicParams` 单元素双图标 + 0.7/100.0；勿拆成两个 Alternate 元素 |
| FIX-9 | `display_glade_border.rs` | closure$1 插真实 `components::DashedLineLength`（或统一 View 与渲染读取面） |

## 5. 登记观察（降级 / 不阻塞，但不得据此宣称判等）

1. **O-1** 核心 `GameTime` 布局与原生不一致：原生 delta@+0x28 / frame_count@+0x2c；
   core `GameClock` delta@+0x24 / frame_count@+0x28（`Duration` 16B 所致）。语义消费正确，
   字节级共享类型缺口。
2. **O-2** `GameConfig.border_signifier_enabled` 对应原生 +0x121 字节；core 字段在结构体
   偏移 0 且命名是“enabled/正逻辑”。宿主装配需按 `byte != 0` 映射，否则极性反转。
3. **O-3** orchestrator 对 id>18：原生把原始 id 字节作为 HintType 组件插入；Rust
   `from_id` 返回 None 时**不插组件**（合法写面只到 0x12，实际不可达）。
4. **O-4** insert/remove 错误处理器：原生 insert=panic、remove=warn；Rust 依赖 bevy 默认。
5. **O-5** `ui_place_tree` 界内 signifier 写面仍是既有近似（format 串 + SignifierFactory
   + 内层 `{1,1,Vec2,f32,f32}`），本轮未改、不判等。
6. **O-6** `hint_position_update` 二级跳转表 tags 0..5/7..9/11..：原生有 PublicWalls 等
   真实分支及 `unreachable!()`/`not yet implemented` panic；Rust `continue` 跳过。
7. **O-7** `hint_focused_edit_hint` 的 Query 非空判据原生取“末项 item 指针”，Rust
   `next().is_some()`；非空域恒同（既有 I）。
8. **O-8** `hint_orchestrator` 的 ToolbarUiIsInFocus 为本地承接事件，生产者未接线（既有 U）。

## 6. 高估检查

- geo-src RESTORE：“重建 3 / 判等 0”整体诚实；但**未发现** EventReader `next()` 与
  原生“耗尽全部”的差异，且把 helper 的显示载荷描述为已判等形状。**低估了缺口**，不是高估。
- hint RESTORE：`hint_orchestrator` 判等**成立**。`hint_position_update` 自标“重建 + Y 占位”，
  未冒充判等；但 **两处结论错误**：“margin = window.width”实为 **height**；
  “case 8 取 min x/min y”实为 **argmax-x 的角**。这两点必须在台账中更正。
- debugline RESTORE：三项“判等”在函数主体层面**成立**；`DashedLineLengthView` 的跨模块
  不互通已登记，但结论仍写“判等”，应加限定“主体判等，渲染集成未闭合”。

## 7. 验证建议（给 restorer 回归）

- FIX-1/2 的数值断言：window 200×100；case 6 → `(cursor.x+4, cursor.y)`；
  case 8 用一组四角投影已知的矩形，断言取 x 最大角的 y（构造 x 与 y 最值不同角以区分 min/max）。
- FIX-4 断言：一帧写 2 个 shape-edit 事件 → count 只 +1（跨两帧）。
- FIX-6 断言：`make_focus...` 的 Steam 载荷低 16 位 = 0x0101。
