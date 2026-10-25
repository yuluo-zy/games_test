# system-water REVIEW 报告（state 组 reviewer，2026-10-17 独立复核）

- 目标：独立审查 restorer-state-water 对 `crates/systems/water` 的 B 还原是否真正还原原二进制业务逻辑。
- 方法：**全部函数 fresh 反编译 + 原指令反汇编/字节直读**（本 reviewer 自建 job：`_review_scratch/job_fresh.json` 等，结果 `_review_scratch/result_*.json`）。未复用 restorer 描述作依据；逐函数独立复述控制流后与 Rust body 逐语句对照。
- 结论概览：**5 判等 / 1 未落库 / 2 胶水**；新发现 2 处 restorer 过度断言（closure$0"重建"但未落库、"name 槽恒 0"被机器码反驳）；1 处语义差（intersects_any max=1.0）为已登记但需注意。

---

## 1. `draw_water`（0x141ba9130）— 判等（附 2 项修正）

### 1.1 独立复述原函数完整控制流（fresh decomp `result_fresh.json` + 原指令 `draw_water_asm.txt` + jumptable/tail 字节复核）

1. **门禁**（0x141ba915e/61）：`CMP byte ptr [0x14362c098],0`；`JNZ return`。非 0 → 早退。
2. **模式分派**（0x141ba9175..9b）：`uVar1=*mode`（tag）；`sel = tag-6 < 0xf ? tag-5 : 0`（tag∈[6,20]→sel=tag-5）；`sel==4`（tag 9 Eraser）、`sel==10`（tag 15 Water），其余 return。
3. **橡皮**（sel==4，0x141ba926c）：`layer=*byte mode[4]`，`layer&3!=0 → return`（层 {0,4}={All,Water} 通过；core `EraserLayer` All=0/Water=4 证实）；`inverse_lerp(0.3,2.0,eraser_size)`（0x140c904a0，机器码 `SUBSS/SUBSS/DIVSS` 三指令 **无钳制**）；`brush_size = t*4 + (1-t)`（常量 0x1429258d8=0.3、0x14292e6f0=2.0、0x142934694=4.0、0x142925904=1.0）；有限性断言（0x141ba92c4 `CMP EAX,0x7f800000; JGE panic`，panic 串 0x142eebba0=`assertion failed: brush_size.is_finite()crates/systems/water/src/draw_water.rs`）；`clearing_canvas("eraser"@0x142eebc08, argument=6, layer=0)`（R8D=6、R9D=0）；`position()` 死调用（0x141ba930a→0x141ba9318 结果被覆盖）；`intersect_with_xz_plane(0.0)`（XMM2=0）；`format!`（格式串 = "eraser_water_"@0x142eebc2d，13 字节）+ `SignifierFactory::new`（0x14089d8b0，拷贝名字串）+ `UiSignifierStream::write`（0x140b2cf90）写入 40B 记录；临时名字串随后 `__rust_dealloc`；`inside=true, draw=false`。
4. **水**（sel==10，0x141ba91a9）：`brush_size = [brush+0x20]+4`（`BrushSizes.water` 偏移 4）；有限性断言（0x141ba91bb `CMP 0x7f7fffff; JG panic`）；`clearing_canvas("watereraser_water_"@0x142eebc28, argument=5, layer=4)`（R8D=5、R9B=4）；`position()`×2（第一次→[RBP+0x18]=draw_raster 点，第二次→[RBP+0x28]=界内圈圆心）；`Circle2d::new(center, 0.8*brush_size)`（XMM3=0x142a16cb8=0.8 × brush_size）；`GladeBorder::is_shape_inside`（0x140ad2970）；`!inside` → 写 `DisplayOutOfBorder`（事件 push 0x141ba9432）+ `into_drag_state()`+`DragState::is_some()`（拖拽中）→ 写 `FeedbackInputOutOfBorder`（0x141ba9480）；`inside=inside, draw=true`。
5. **共用尾**（0x141ba94bc）：`into_drag_state()`（0x14092df10）→ `DrawState::update(mode_id=dragged, raw=inside)`（0x140ae4bd0，EDX=EAX、R8D=EBX）；`switch byte [state]` 经 jumptable 0x142eebcb0（4 项：case0→0x141ba9640 return、case1→0x141ba94ed、case2→0x141ba9502、case3→0x141ba958b）。
   - case1（0x141ba94ed）：`[RBP-0x30]=0`（discriminant 0）、`[RBP-0x28]=radius`、`[RBP-0x24]=draw`。
   - case2（0x141ba9502）：`TEST BL,BL; JE return`（!inside→return）；`TEST R12B,R12B; JE skip`（!draw→仅 discriminant=2）；否则 `position()`→[RBP+0x28]（圈圆心）+`Circle2d::new(半径=brush_size)`+`position()`→[RBP-0x48]（高=pos.y）+`ADDSS 0x1429258d0(=1.5)`（pos.y+1.5）+`MOV word [RBP-0x38],0x100`（union 标志）+`EventWriter::write<ShowPinnedDecoClutter>`（0x142260a60）+`[RBP-0x30]=2`。
   - case3（0x141ba958b）：`[RBP-0x30]=3`。
6. **尾**（0x141ba958c..）：`RecalculateHeightMapCmd` 单元事件 push（grow_one 模式）→ `ScheduledTerrainReadback::on_terrain_modified`（0x140aabdc0）→ 组装 draw_raster 参数（cmd@[RBP-0x30]、point@[RBP+0x28]、三个闭包@[RSP+0x20/28/30]）→ `draw_raster<WaterRasterStagingSsbo, WorldRaster<512>, env$1..3>`（0x142260ae0）。

### 1.2 与 Rust body 逐语句对照

Rust `draw_water`（`draw_water.rs` 行 109-257）逐句对应：
- 门禁 `if !water_systems_enabled() return`（常量 true 建模，差登记）↔ 原 `flag!=0→return`（I：重建库不可直读宿主内存；语义方向一致）。
- `match *mode { Eraser(All|Water) =>, Water =>, _ => return }` ↔ sel==4/10。
- 橡皮：`t=(x-0.3)/(2.0-0.3)`（无钳制）↔ inverse_lerp 机器码；`t*4+(1-t)` ↔；`assert!(is_finite())` ↔（water 分支 `>0x7f7fffff`、橡皮 `>=0x7f800000` 均≡!is_finite）；`clearing_canvas_w_layer_and_sound("eraser",6,0)` ↔ R8D=6/R9D=0；`cursor.position()` 丢弃 ↔ 死调用；`camera_ray.intersect_with_xz_plane(0.0)` ↔ XMM2=0；`signifiers.write(UiAppendSignifier{tag:1,kind:1,position:point,scale:0.5*brush_size,name:0})` ↔ 原记录 tag=1/kind=1/position/scale=0.5×brush_size（**name 分歧，见 FIX-1**）；`inside=true, draw=false` ↔ BL=1/R12B=0。
- 水：`brush.1.water` ↔ [brush+0x20]+4；`assert!` ↔；`clearing_canvas_w_layer_and_sound("watereraser_water_",5,4)` ↔ R8D=5/R9B=4；`point=cursor.position()`、`center=二次 position` ↔ 两次 position（[RBP+0x18]/[RBP+0x28]）；`Circle2d::new(center, 0.8×)` ↔ XMM3=0.8×；`is_shape_inside` ↔；`!inside`→`DisplayOutOfBorder`+`dragged()!=2`→`FeedbackInputOutOfBorder` ↔（`dragged()!=2`≡`into_drag_state().is_some()`，UiWidgetContext::dragged 返回 2=未拖拽）；`draw=true`。
- 尾：`phase=widget.dragged(); state.update(phase, inside)` ↔；`match state.0{0→return,1→0,2→…,3}` ↔ jumptable 4 项；case2 `!inside→return`、`if draw {重取 position、Circle2d(半径=brush_size)、height=pos.y+1.5、flag 0x100、write}` ↔（原两次 position 语义等价于 Rust 一次复用）；`recalc.write(RecalculateHeightMapCmd)` ↔；`readback.on_terrain_modified()` ↔；`draw_raster::<…>(cmd, point, slot=4, borrowed, water, library, 3 闭包)` ↔（slot=4 与 ActiveRasterStrokesStorage 槽位一致，WaterRaster 槽=4）。

**判等**。两处需修正/登记：见 FIX-1（橡皮 signifier name）、FIX-5（RasterStrokeCmd.strength=1.0 为建模补充，已登记）。

## 2. `closure$0`（0x141ba9700）— **未落库（依赖 core signifier 域）**，restorer 判定"重建（业务）"为过度断言

Fresh decomp（独立复核）：`__rust_alloc`（Vec）→ `__rust_alloc`（名字串）+ `builtin_strncpy(ptr,"brush_circle_scribble",0x15)` → `SignifierEntityBuilder::new().animation_duration().position().scale().set_ui_pass().build()` → 返回 `{len=1, ptr=entity, cap=1}` 单元素 Vec。与 restorer 描述一致。

**分歧**：restorer 汇总表标"重建（业务）"，但 RESTORE.md §1.1 自述"闭包体未落库内"。本 reviewer 核查 `draw_water.rs`：**crate 内无该闭包任何实现**。逻辑已全量分析（O）且依赖 core `SignifierEntityBuilder`（core signifier 域未恢复），属合理依赖挂起——但**判定应为"未落库-依赖 core 域（O 面登记）"而非"重建"**，过度断言。若验收要求本 crate 承载全部业务符号，此项为缺口。

## 3. `closure$1/2/3`（inline）— glue-legitimate

Rust `draw_water.rs` 行 241-255 三个闭包与 core `draw_raster` 的 `on_draw(Vec3,f32,bool)/on_erase/on_record` 载荷一致：`EventWriter::write<WaterDrawEvent/WaterEraseEvent>{position,radius}`、`write_default<RecordWaterEditToHistoryCmd>`（单元事件）。核心事件类型 core `resources/water.rs` WaterDrawEvent{position:Vec3,radius:f32}/WaterEraseEvent 同构。确认胶水。

## 4. `update_water_audio_grid`（0x141baefe0）— 判等（含 3 项已登记契约差）

### 4.1 独立复述（fresh decomp + `draw_water_asm.txt` 的 uwag 段，369 指令逐条核对）

1. **puffin 门控**（胶水）：`are_scopes_on()`；开→rfind 作用域名（"stem_water::…update_water_audio_grid::f"）→`begin_scope`；门禁 `*0x14362c098`：scopes-off 面 `!=0→return`（0x141baf0cd JZ body）、scopes-on 面 `==1→end_scope+return`（0x141baf295）。
2. **帧计数**（0x141baf315）：`state.frame = f+1`（pre-increment）。
3. **帧位/抖动**（0x141baf32b..379）：`frame_bit = f+(f+1)*10+1`（=`11(f+1)`）；`bit = 1 << (frame_bit&63)`（SHL R13,CL）；增量对 `PAND [0x142eed8c0]=0x000e000e` + `PADDW [0x142eed8d0]=(-7,-7)`：`d={((2*fb)&0xe)-7, ((fb>>2)&0xe)-7}`（i16 回绕）；`CVTDQ2PS→MULPS [0x142eed8e0]=0.0625 × [0x142eed8f0]=13.0` → jitter=d*0.8125。
4. **眼位**：`MainCamera::eye_position`（0x140911dd0）。
5. **遍历 24B 单元**（`[grid+8]=base、[grid+0x10]=len`，步长 0x18，0x141baf3f0）：
   - `MOVSD XMM0,[cell]`（单元基址{x,z}）+ `ADDPS XMM9`（+jitter）→ `x0y`（0x140c9cdc0）→ 点 {x,0,z}；
   - `dist² = (ex-px)²+ey²+(ez-pz)²`（SUBPS/MULPS/MOVSHDUP 重排），`UCOMISS [0x142eed900]=6400; JBE →清位`（拒 NaN，见 FIX-4）；
   - `bilinear_sample_at_world_xz(point.x,point.z, XMM3=130.0@0x142a7b658, [RSP+0x20]=0x43020000=260.0)`（0x140a14df0），`UCOMISS XMM13=0.5@0x142925870; JBE →清位`；
   - `intersects_any(ray={origin=点, dir=眼-点}, max=1.0@0x142925904, mask=0x32a3)`（0x1408cc320，R9D=0x32a3）；命中→清位；
   - 置位：`window |= bit`（OR R13）；0→1 时 `counter += d`（`PADDW XMM8`，+0x10 两 u16 打包）；
   - 清位：`window &= ~bit`（RSI=-2 ROL CL，0x141baf4e8）；1→0 时 `counter -= d`（`PSUBW`）。
6. **profiling 尾**：scopes-on 面 end_scope + 计数回滚（glue）。

### 4.2 与 Rust body 对照

`water_audio_grid.rs` 行 84-173 逐句对应：门禁↔；帧计数↔；frame_bit↔；bit↔；delta 对↔；jitter↔；eye↔；三重剔除↔（`delta.length_squared()>=6400`、`bilinear_sample_at_world_xz(px,pz,130,260)<=0.5`、`!raycasting.intersects_any(&terrain, point, delta, 0x32a3)`）；置位/清位与 ±d↔（`wrapping_add/sub`）；count_ones/64→water_level、质心→position 为投影。

**判等**，3 项契约差为**已登记**但须在验收中显式确认其行为影响：
- **intersects_any max=1.0 无法经 core API 表达**（core `intersects_any(terrain,origin,dir,mask)` 用 `f32::MAX`；原版 max=1.0 只测采样点附近 1.0 距离）。行为差：core 版本会命中更远地形 → 清位更多。此为**真实语义差**，非纯登记项。
- 窗口/计数器原存单元 +8/+0x10（24B），本 crate 移入 `WaterAudioGridUpdateState` Local 并投影 `water_level=count_ones/64`、`position=质心`。原版消费方从窗口位/计数器推导（激活=当前帧位），投影用 64 帧均值——**语义近似**，音频域待裁决。
- 单元基址原为栅格构建方预写 +0/+4；本 crate `cell_coord(index)*spacing`（I）重建。

## 5. `record_water_history`（inline，宿主 0x140e92a80）— 判等

### 5.1 独立复述（fresh disasm `result_disasm_rwh.json`，171 指令）

1. Bevy 系统 tick：`XADD [RDX+0x5b0],EDI`（last_run++）。
2. 取三资源：`ActiveRasterStrokesStorage`、`History`、`Events<RecordWaterEditToHistoryCmd>`（缺失→三处 panic 串，非业务）。
3. EventReader 双缓冲未读计数（saturating_sub + CMOVC）：`new_cursor = total - (b1_unread+b2_unread)`。
4. 门控：`TEST CL,R11B; JNZ skip`（两缓冲**全空**才跳过 → 调 record_raster_history）。
5. 调用（0x140e92c6c..7d）：`MOV [RSI+0x18],total`（游标前移）、`LEA R8,[0x1416bb7f0]`、`MOV DL,0x4`（**slot=4**）、`CALL 0x140957990`（record_raster_history）。**结果未检查、无 HistoryRecordingFailed 发射**。
6. 收尾：`MOV [RSI+0x160],last_run`（last_event_count）。

### 5.2 对照

Rust `record_water_history.rs`：`events.read().last().is_none()→return` ↔ 双缓冲全空门控（语义等价）；`record_raster_history(&mut strokes, 4, &mut history, |s| EditType::Water(s))` ↔ slot=4 + `EditType::Water` 构造器 0x1416bb7f0（标签 `_ZN…EditType5Water…`，写判别 5，core EditType::Water=5）；结果 `let _=` 忽略 ↔ 无错误检查；无 HistoryRecordingFailed ↔ 确认删除近似正确。

**判等**。restorer 关键纠错（删 HistoryRecordingFailed、slot=4、EditType::Water）全部属实。

## 6. `WaterPlugin::build`（0x1406501d0）— glue-legitimate

Fresh decomp（`result_fresh.json`）确认注册面：
`IntoScheduleConfigs::in_set<record_water_history, GameSystemSet>` → `SubApp::add_systems<Update>`（#1）→ `run_if<update_water_audio_grid, bool(Option<Res<WaterAudioGrid>>)>` → `Interner::intern<dyn SystemSet>` + `in_set_inner` → `add_systems<Update>`（#2）→ `DirtyTilePublisher::unique()` + `PathRaster::new()` + `insert_resource<WaterRaster>` → `register_event`×3（RecordWaterEditToHistoryCmd/WaterDrawEvent/WaterEraseEvent；第三个命中"已存在"→return）。

Rust `build/register_water_events/add_water_cpu_systems`：注册顺序（资源→事件）一致；追加 `draw_water` 注册与额外事件资源为 crate 自足接线（已登记）。纯 Bevy 注册胶水。**glue-legitimate**。

---

## 未实现/分歧点清单

| # | 函数/点 | 判定 | 说明 |
|---|---|---|---|
| 1 | `draw_water` 橡皮 signifier name | **分歧（需修正）** | 原版 `format!` 生成名字串（格式串 "eraser_water_"@0x142eebc2d）经 SignifierFactory 拷贝写入 40B 记录；Rust 写 `name:0`。RESTORE.md "name 槽恒 0" 与机器码不符（0x141ba9320..369 构造格式串、0x14089d8b0 拷贝名字）。另：原版写记录实为 `{name_len,name_ptr,name_len,&内层记录,marker}`（factory 输出），core `UiAppendSignifier{tag,kind,position,scale,name}` 布局是否对应原记录，需 core signifier 域复核。 |
| 2 | `closure$0` 判定 | **过度断言** | 汇总表"重建（业务）"，实际 crate 内未实现（依赖 core SignifierEntityBuilder，O 面登记）。应标"未落库-依赖 core 域"。 |
| 3 | `update_water_audio_grid` intersects_any max | **语义差（已登记）** | 原版 max=1.0；core API 无 max（f32::MAX）。遮挡剔除范围不同。 |
| 4 | NaN 边界 | 微差 | 原版 dist²/水位 `UCOMISS+JBE` 在 NaN 时清位；Rust `>=`/`<=` 在 NaN 时不清位。输入有限时不可达。 |
| 5 | RasterStrokeCmd.strength | 建模补充（已登记） | case1 原版不写 strength（未初始化），Rust 补 1.0。core 结构带字段所致。 |
| 6 | 门禁建模 | I（已登记） | 原版读宿主全局 0x14362c098；Rust 常量 true。 |
| 7 | WaterAudioGrid 语义投影 | 契约差（已登记） | 24B 单元{窗,计数}→Local + 投影 water_level/position；音频域待裁决。 |

## FIX 清单（针对 restorer 交付）

- **FIX-1（必须）**：修正 RESTORE.md 橡皮 signifier 断言——原版 name 非恒 0（format! 串）；选择建模 name 或准确登记。同时向 core 登记 UiAppendSignifier 布局疑点。
- **FIX-2（必须）**：修正 closure$0 判定为"未落库-依赖 core signifier 域"，并在交付缺口清单单列。
- FIX-3（登记强化）：max=1.0 语义差在验收时需显式纳入 core API 合同（若 core 增加 max 参数则回填）。
- FIX-4（可选）：Rust 剔除比较改用 `>=` 语义对齐原版（当前不可达，记录即可）。

## 证据

- `_review_scratch/result_fresh.json`（5 函数 fresh decomp）、`result_disasm_rwh.json`（record_water_history 宿主 171 指令）、`result_mem.json`（panic 串/串区/常量/类型表）、`result_tail*.json`/`draw_water_asm.txt`（尾块与 jumptable）、`result_ilmem.json`（inverse_lerp 机器码）、`result_verify_closure.json`（EditType::Water 构造器标签）、`result_factory_mem.json`（SignifierFactory 机器码）。
- `cargo check -p system-water` 通过（仅 core 侧 13 告警）。

## 复验状态（2026-10-17 restorer 落实 FIX 后复核）

**fix_verified**（四项 FIX 全部落实，逐项对机器码复核通过）：

1. **FIX-1（橡皮 signifier name）**：Rust 现以 `ERASER_SIGNIFIER_NAME="eraser_water_"`（13 字节）+ `name = len()` 建模；`draw_water.rs` 行 139-151 注释准确（format! 名字串 @0x142eebc2d、SignifierFactory 0x14089d8b0 decomp `{name_len,name_ptr,name_len,&inner,marker}`、临时串释放、core `name:u64` 无法承载串→core 域登记）。对照机器码：原写记录首 qword = format 结果长度 = 13（格式串无占位符 → 输出恒 "eraser_water_"）；`ERASER_SIGNIFIER_NAME.len()==13` 一致。内层记录 tag/kind/pos/scale 现正确表达。RESTORE.md §1 已改（无 "name 槽恒 0" 旧断言），§1.2 登记布局疑点归 core signifier 域。**通过**。
2. **FIX-2（closure$0 判定）**：RESTORE.md 汇总表改为"**未落库-依赖 core signifier 域**（O 面登记）"，§1.1 自述"crate 内无该闭包实现"，§5 交付缺口清单 item 1 单列。不再过度断言。**通过**。
3. **FIX-3（max=1.0）**：RESTORE.md §2 标"**语义差，验收需显式确认**"，明确"core 版本会命中更远地形→清位更多；若 core 增加 max 参数则回填"。登记充分（此为核心 API 合同依赖，crate 内不可自愈）。**通过**。
4. **FIX-4（NaN 边界）**：`water_audio_grid.rs` 行 127/133 改用 `!(delta.length_squared() < MAX_CAMERA_DIST_SQ)` 与 `!(level > WATER_LEVEL_THRESHOLD)`。对照 UCOMISS+JBE：JBE=CF∨ZF（dist²≥6400 / unordered）→ `!(x<y)` 对 NaN 亦真（NaN<x 为假→!假=真→清位），逐位对齐原版 unordered-NaN 清位语义。**通过**。

四项 FIX 均 fix_verified，无残留 FIX。
