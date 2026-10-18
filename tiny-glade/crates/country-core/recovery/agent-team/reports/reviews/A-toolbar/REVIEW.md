# REVIEW — A-toolbar：toolbar/purelogic/consume 12 项判等独立复核

> 角色：reviewer-A-toolbar。日期：2026-10-17。只读复核，未改 src。
> 方法：对每个条目 **fresh 反编译/反汇编**（Ghidra `--worker reviewer-a-toolbar` 只读，
> B patched EXE，`program=tiny-glade.exe`）→ 独立复述控制流 → 与 Rust body 逐语句/逐字段对照
> → verdict。证据落盘 `work/reviewer-A-toolbar/{jobs,results,evidence}/`。
> 复核对象：`planning/2026-10-17-evidence-audit-interaction.md` A 清单中
> toolbar 4 / purelogic 3 / consume 5，共 **12 项**。
> 判定口径沿用硬标准：**判等** = 实现语句/字段与原生逐条一致（仅余已登记 I/U 且不改实现行为）；
> **需修** = 发现具体分歧或缺失写面/路径，应修；**未实现** = 主体缺失。

## 0. 结论速览

| # | 条目 | VA | 现判等依据 | 本次 verdict | 核心结论 |
|---|---|---|---|---|---|
| 1 | `toolbar_button_keep_focus` | 0x1423414b0 | gap REVIEW 判等 | **需修** | 四步算法判等；但 keep/selected 属原生 `UiInteractionState`（3B）本身，Rust 拆出的 `WidgetByteState` **未注册/未挂载**，四系统也未接 schedule → 无可运行等价面 |
| 2 | `switch_toolbar_category_button` | 0x1423427f0 | gap REVIEW 判等 | **需修** | 门控/载荷判等；同上 Query 空集，且 click 分支 `query.get` 恒 Err → `RequestToolbarCategoryChange` 从不发射 |
| 3 | `tod_v2_button` | 0x142342f30 | gap REVIEW + FIX-3 | **需修** | 清 visible 门读错事件流（原生读 **ModeJustChanged 首事件 tag≠0xd**，Rust 读 click 非空）且顺序相反；UiNavState 写面缺失；per-widget 写 no-op |
| 4 | `ui_animation_state_update` | 0x14233b3f0 | gap REVIEW 判等 | **需修** | leave/enter/state/clamp/appear 公式判等；appear sign 源应为 `UiInteractionState+0`；`WidgetByteState`/接 schedule 缺失 |
| 5 | `FishSpawnLocations::new` | 0x140f909f0 | purelogic RESTORE 判等 | **需修** | **行列转置**（原生首分量=外层→x，Rust x=内层）；公式非位等（25 档中 7 档差 1 ULP）；used 字节未建模（已登记） |
| 6 | `DuckSpawnLocations::new` | 0x140f8ee90 | purelogic RESTORE 实现 | **需修** | **同行列转置**；行公式逐位一致、timer=25.0、used 向量正确 |
| 7 | `draw_water` 注册（add_water_cpu_systems + register_water_events） | 0x1406501d0 | purelogic RESTORE 判等 | **判等** | 原生 build 注册 2 系统+3 事件+WaterRaster；Rust 同链 + 追加 draw_water/额外事件（已登记自足接线，state/water REVIEW 独立复验） |
| 8 | `replay_tree` | 0x141bc34f0 | consume RESTORE 重建 | **需修** | 时序/逆序移除/Delete/Insert/Place 判等；**Move undo 读 edit+0x14/+0x1c 旧位整条未实现**（G2/T2 已登记） |
| 9 | `gpu_flush_buffers` | 0x140de50a0 | consume RESTORE 判等 | **判等** | get_mut→unwrap→deref→gpu_flush 逐语句一致；ResMut tick 自动 |
| 10 | `maintain_dormer_tiles` | 0x1407ce920 | consume RESTORE 判等 | **判等** | 游标预写/每瓦四步/drop 逐语句一致；`id_slot` null guard 为非空安全缝（原生无条件解引用） |
| 11 | `process_tree_cmds`（Insert 分支） | 0x142267430 | consume RESTORE 重建 | **需修** | 名称门/`max(scale,0.7)` 门/α/4 组件/冲突日志/声音载荷判等；**SpawnPrefabCmd 0x50 载荷只发 default（T5）**，T4 宿主输入以 false 登记 |
| 12 | `ui_move_tree` | 0x14226b1a0 | consume RESTORE 重建 | **需修** | 门禁/状态机（含 state 0/1 亦钳 case3）/目标位/双事件/Move+History 判等；T3 旧位缺、候选源代理；**MoveTreeCmd 原生载荷 0x18 而 Rust 0x20** |

**三态：判等 3 / 需修 9 / 未实现 0。**

---

## 1. toolbar 4 项

### 1.1 共同结构发现（决定 1–4 的 verdict）

四函数的 caller mangled 名（fresh `get_xrefs_to`）给出真实 Query：

- keep_focus：`Query<(&mut ToolbarButtonCooldown, &mut UiInteractionState)>`
- switch_category：`Query<(Entity, &SwitchToolbarCategoryButton, &mut UiInteractionState)>`
- tod_v2：`Query<(&TimeOfDayButtonV2, &mut UiInteractionState)>`
- ui_animation：`Query<(&UiInteractionState, &mut UiAnimationState)>`

即原生 Query **只有 2 个组件**；逐实体 3 字节记录（`+0` appear 符号 / `+1` keep / `+2` selected）
就是 **`UiInteractionState` 组件自身**：

- keep_focus 0x14234153d..：`*(char*)(col+1+id*3)`（keep）——col = `UiInteractionState` 数据列；
- switch_category 0x1423428xx：`*(bool*)(col+2+id*3) = (toolbar.current==category)`；
- tod_v2 0x14234303e..：`*(char*)(col+2+id*3) = visible`；
- ui_animation 0x14233b130..：`pcVar20=id*3+col`，`pcVar20[0]` appear 符号、`pcVar20[1]` keep。

`register_component<UiInteractionState>` 两次独立注册（0x14084fcc0 / 0x141e34530）的
`ComponentDescriptor` 均存 size 立即数 **3**（`MOV qword [RBP+0x200],3`），与 3 字节面一致。
另注意类型归属：原生 mangled 名为 **`country_core::components::UiInteractionState`**，而 Rust 现于
`toolbar::resouces_and_components` 本地定义同名 type（注释“暂置”）；country-core 目前只有 registry
名表、无该 struct。FIX-T1 应同时在 country-core 落类型并让 toolbar 引用。

**Rust 现状**：把同一 3 字节拆成两个组件——`UiInteractionState {state,_pad,flag}`（3B）+
`WidgetByteState {appear_sign,keep,selected}`（3B），并把四系统 Query 改为三组件。
致命点：**`WidgetByteState` 既不在 `register_local_components`（26 项台账无此类型）也未在任何
spawn/insert 中出现**（grep `crates/**`；`toolbar_layout` spawn 只插 `UiInteractionState/
UiAnimationState/UiMouseInteractable/...`），且四系统**未加入任何 schedule**（`ToolbarPlugin::build`
未实现，`configure` 只 `init_resource<MeiContext>`）。
→ 即使将来注册，`Query<&mut WidgetByteState>` 也因组件未注册/未挂载而无法匹配；
当前不存在与原生等价的四系统运行面（不能计“判等落库”）。

另：原生多处 `*(u32*)(col_changed_ticks + id*4) = Query.this_run` 不是业务写面，而是 bevy
`&mut T` 的 changed-tick 记账（`TicksMut::set_changed`）；Rust 使用 `&mut` 组件并在
mutation 时自动等价。RESTORE 的“tick 写面 U”属误读，可在修类型后删除该登记。

**FIX-T1**（高）：`UiInteractionState` 恢复为原生 3B `{state@0(=appear 符号/交互层), keep@1, selected@2}`；
四系统 Query 去掉 `WidgetByteState`，直接读/写 `interaction.keep/selected/state`；
删除或保留 `WidgetByteState` 为非 Query 备用容器均可。修后四系统才与原生同构且可匹配实体。

### 1.2 `toolbar_button_keep_focus`（0x1423414b0）

**独立复述**（fresh 0x14234153d..0x142341dc5）：①逐项：keep 字节（interaction+1）≠0 →
cooldown=0.3、`any_keep=1`；=0 → `cooldown=max(cooldown-dt,0)`；②reset 事件非空 → 读游标推进、
全部 cooldown=0；③`any_keep` → keep=0 项 cooldown=0；④cooldown>0 → keep=1。

**对照**：Rust 四步逐条一致（0.3 常量、clamp 顺序、事件门、any_keep 累积、写 keep=1 均对）。
分歧即 §1.1 的组件承载（keep 写进未挂载的 `WidgetByteState`，主 Query 空集）。

**verdict：需修**（算法判等，接线/类型需修）。

### 1.3 `switch_toolbar_category_button`（0x1423427f0）

**独立复述**：click 事件 → 解析实体是否命中 Query（bundle 含 button/`&mut UiInteractionState`）→
读按钮 1B category → push `EventInstance<WindowFocusEvent>{event_id,payload@+8}`；
随后逐项：动画插入门 = `(ModeJustChanged unread 存在) && (interaction+2 != 1) &&
(toolbar.current == button.category)` → `UiInsert<UiPlaySelectAnimation{0.1,0.2}>`；
每项写 `interaction+2 = (current==category)`。常量 `0x3dcccccd/0x3e4ccccd` 逐位对上。

**对照**：Rust 门/载荷/选中写面/插入顺序一致。但 click 分支 `query.get(click.entity)` 的 Query
含 `&mut WidgetByteState` → 恒 `Err`，**事件从不发射**；逐项循环空集。

**verdict：需修**（同 FIX-T1；修类型后自动闭合）。

### 1.4 `tod_v2_button`（0x142342f30）

**独立复述**（参数 3=ResMut<TimeOfDayScreen>、4=ResMut<UiNavState>、5=EventReader<ModeJustChanged>）：
①先逐条处理 **UiClickEvent**：实体命中 Query → `visible ^= 1` + insert `UiPlaySelectAnimation{0.1,0.2}`；
若 toggle 后 visible==1 → 写 UiNavState 变更 tick 并 `UiNavState+0x10 = 0`；
②click 读尽后处理 **ModeJustChanged**：若存在未读事件且**首事件判别 `(int)tag != 0xd`** 且
`visible==1` → `visible=0`（写 TimeOfDayScreen 变更 tick）；
③逐项写 `interaction+2 = visible`（在 ①② 之后）。

**对照**：Rust 的 `if !click_events.is_empty() && visible` 清 visible——**事件源错误**（应为
ModeJustChanged 首 tag≠0xd），且顺序放在 click 之前（原生在 click 之后）。即
- 无 click、仅 mode change 时：原版清、Rust 不清；
- 同帧 click 且 mode change 时：原版先 toggle 后清，Rust 先清后 toggle，末态可相反。
另外 Rust 未写 `UiNavState+0x10=0`/UiNavState tick（`UiNavState` 布局 U，已登记数据源但未登记该写面）；
per-widget 选中写 no-op（§1.1）。

**verdict：需修**（FIX-T1 + FIX-T2）。

**FIX-T2**：清 visible 改读 `EventReader<ModeJustChanged>` 的首个未读事件判别（≠0xd），
并把排序改为「click 处理 → mode-change 清 visible → 逐项写 selected」；`UiNavState+0x10=0`
在布局取证后补写（当前至少登记）。

### 1.5 `ui_animation_state_update`（0x14233b3f0）

**独立复述**（逐句）：每项 `(interaction, anim)`：
- `interaction+1 == 0`（离开）：state==0 → state=2；否则 `select += dt/-0.2`、state=1；
- `interaction+1 != 0`（进入）：state!=1 → `select += dt/0.2`、state=0；否则 state=2；
- `state = uVar22`；`select=clamp01`；
- `appear += 4·dt·sign`，sign = `interaction+0 == 0 ? -1 : +1`；`appear=clamp01`。

**对照**：Rust `update_progress` 的四分支/写序/常量逐条一致；`(appear + 4dt·sign).clamp(0,1)` 一致。
唯一：sign/keep 源取 `WidgetByteState`（未挂载）；clamp 对 NaN 的语义（原生 max/min 序列 NaN→0，
`f32::clamp` NaN→NaN）仅在 dt 非有限时可见，属低危 I。tick 写为变化检测记账（§1.1）。

**verdict：需修**（FIX-T1；公式判等）。

---

## 2. purelogic 3 项

### 2.1 `FishSpawnLocations::new`（0x140f909f0）

**独立复述（含反汇编）**：外层 i=0..24、内层 j=0..24 共 625；元素 stride 0xc；
`MOVSS [base+0]=outer 值`、`MOVSS [base+4]=inner 值`、`MOV byte [base+8]=0`（used）。
行/列值公式均为 `(k/24)*52 - (1-k/24)*52`（DIVSS 24、SUBSS 1、MULSS 52 两条、SUBSS）。

**对照（两处分歧 + 一处已登记缺口）**：
1. **行列转置**：原生元素 Vec2 =（外层值→+0→世界 x，内层值→+4→世界 z）；
   消费者 `fish` 0x140f90cd0 以候选 Vec2 直接 `bilinear_sample_at_world_xz(x,z)` 并 `x0y()`
   （`x0y: Vec3(x,0,y)`，`utils/swizzlers.rs`）。Rust `Vec3::new(fx(col),0,fy(row))` →
   **x=内层、z=外层，与原生互换**（eco/critters REVIEW 早先的转置结论成立；purelogic RESTORE
   “行/列序一致”不成立——其依据是 decompiler 偏移被 +4 移位的读数）。
2. **公式非位等**：Rust `((k/24)*2-1)*52` 与原生 `(k/24)*52-(1-k/24)*52` 代数相同但
   f32 舍入不同：k=1,2,7,8,10,11,13 共 7/25 档差 1 ULP（如 k=1：原生 `-47.666664`
   =0xc23eaaaa，Rust `-47.666668`=0xc23eaaab）。RESTORE 的“恒等”不成立。
3. used 字节（+8）未建模（RESTORE 已登记 DIVERGE）。

**verdict：需修**（FIX-P1）。

### 2.2 `DuckSpawnLocations::new`（0x140f8ee90）

**独立复述（反汇编）**：外层 i=0..9、内层 10 档列常量；元素 stride 0xc：
`[+0]=outer 行公式值`、`[+4]=inner 列常量`、`[+8]=used(0)`；尾部 `+(0x18)=0x41c80000(25.0)`。
10 档常量 `0xc2500000/c221c71c/c1e71c73/c18aaaa9/c0b8e390/40b8e390/418aaaac/41e71c73/4221c71c/42500000`。

**对照**：Rust `duck_spawn_steps()` 用原生表达式，逐位一致；timer=25.0 一致；used:vec![false;100] 一致。
**同为行列转置**：Rust `Vec3(col,0,row)` → x=内层、z=外层；原生 x=外层、z=内层。
消费者 `spawn_ducks` 0x140f8f210 以 `(Vec2,bool)` 候选直接水面/地形采样（Vec2.x→x）。

**verdict：需修**（FIX-P2：`Vec3::new(row,0.0,col)`）。

### 2.3 `draw_water` 注册（`add_water_cpu_systems` + `register_water_events`）

**独立复核**：原生 `system_water::impl$0::build` 0x1406501d0（`reports/state/water/REVIEW.md` §6
独立 fresh 复验）注册 = `record_water_history`（GameSystemSet）+ `update_water_audio_grid`
（`Option<Res<WaterAudioGrid>>` run_if）+ `insert_resource<WaterRaster>` + `register_event×3`；
**不含 draw_water**。Rust `build` 走 `register_water_events`+`add_water_cpu_systems`，同链注册上述
项，并**追加** `draw_water` 与额外事件资源（crate 自足接线，源码注释与 RESTORE 已登记）。
`state/water` REVIEW 独立判为 glue-legitimate。

**verdict：判等**（基础注册面；追加项为已登记自足接线，不是原生注册面）。RESTORE 的
“draw_water 注册判等”措辞应改为“基础 build 判等 + draw_water 自足追加（I）”。

---

## 3. consume 5 项

### 3.1 `replay_tree`（0x141bc34f0）

**独立复述**：`front_mut_bucket()`→逐 0x30 槽；tag==8：disc 2=Delete / 1=Move / 其它=Insert|Place：
- 仅 `elapsed(+0x18)==0.0 && !NaN` 重发；
- Delete 非 undo→Delete(id)；Delete undo→0x40 Insert 拷贝（含 name clone/position/scale/rotation/variant/flags/id/height）；
- Move 非 undo→0x18 载荷 {索引、新位置(+0x20/+0x28)}；**Move undo→同一写法但取 edit+0x14/+0x1c 旧位**；
- Place 非 undo→Insert；Place undo→Delete；
- flags(+8) bit0=0→立即移除；bit0=1→`delay=(+0xc)/speed`，下钳 0.05，`delay<elapsed`→移除，
  `elapsed += GameTime.delta`；移除集**逆序** Vec::remove。

**对照**：Rust 的时序/逆序删除/事件门/各分支/0.05 钳/elapsed 推进逐条一致。
**Move undo 整条不发射**（Rust 注释 G2）——原生该分支确实存在且读旧位（`local_118=*(u64*)(edit+0x14)`、
`+0x1c`）；Insert 0x48 载荷 T2 缺位。两者均已登记。

**verdict：需修**（FIX-C1：`TreeEditData` 补 previous position 后按旧位发 MoveTreeCmd；T2 随 core 类型）。

### 3.2 `gpu_flush_buffers`（0x140de50a0）

**独立复述**：Res<GraphicsDeviceResource>+ResMut<AssetSsboLibrary>；
`get_mut<RoofTileSsbo>()`→None→`unwrap_failed`；`GraphicsDeviceResource::deref`→
`SparseInstanceBuffer<RoofTileSsbo>::gpu_flush`；写 ResMut 变更 tick（+0x150）。
**对照**：Rust `library.get_mut::<RoofTileSsbo>().unwrap(); buffer.gpu_flush(device.device());`
逐语句一致；`mut library` 的 DerefMut → tick 自动。
**verdict：判等。**

### 3.3 `maintain_dormer_tiles`（0x1407ce920）

**独立复述**：prev=*cursor；startA/B=saturating_sub(countA/B)；`*cursor=total-lenB'-lenA'` 预写；
遍历 A[skip(startA)] 链 B[skip(startB)]；每瓦 `*cursor+=1` → `*id_slot = state+0x1c 值`（单槽 store，
不递增）→ `hash_one(&elem+8)` + `RawTable::remove_entry` → 返回值非空 `drop_in_place<
SparseInstanceBufferAllocation<RoofTileSsbo>>`。
**对照**：Rust 预写/链式 skip/每瓦四步/drop 逐条一致；`if !state.id_slot.is_null()` 是重建安全缝
（原生无条件解引用；非空时行为相同，I）。
**verdict：判等。**

### 3.4 `process_tree_cmds`（Insert 分支，0x142267430）

**独立复述（逐段 + 反汇编）**：事件元素 stride 0x48；
- 名称门：len 25/26/39 三档 memcmp → 5 个 `clearing/naked_tree/*` 名；
- `deny_load`（0x140a9fa70）、`is_corrupted`（0x140ae8320）；`MAXSS(scale,0.7)` 仅
  `naked && deny_load`（asm `AND BL,DIL; JNZ`，DIL=deny_load 返回值）；
- α：`inverse_lerp(0.5,0.9,scale)`（原始 scale）×0.75 +(1-r)×0.55；
- spawn_empty + 依次 insert TreeComponent/TreeAnimation/TreeMoveBend(x,z)/TreeAmbience
  `(x, y+clamped_scale*10, z)`（fVar52=clamped，`*0.0` 项为编译器乘零）；
- 名 == `MISSING_TREE_PREFAB_NAME`(24B) → `random_tree_prefab(id 低位, ...)`（0x14226a170
  内部读 deny_load/halloween/is_corrupted/DragState::is_started）；
- **SpawnPrefabCmd 0x50 载荷**（prefab/entity/position/scale/{variant,rotation}/flags/mode）入队；
- `TreeStorage::insert`：Some(previous) → `warn!("Tree {} already exists...")` + DespawnPrefabCmd；
- 水样 `bilinear_sample_at_world_xz` → PlayTreeSoundCmd{kind=0, selector=in_water,
  extra=naked&&(deny||corrupted), value=r, position}（槽内偏移 +8/+9/+10/+0xc/+0x10 逐项对上）。

**对照**：Rust 名称门/门控/α/四组件/冲突日志/声音载荷/顺序一致。
缺口：**SpawnPrefabCmd 只 `write(default())`**（计算出的 `_prefab_name` 被丢弃，T5 载荷 0x50 未落库）；
`is_corrupted` 恒 false、`drag_started` 恒 false（T4 宿主资源未取证）；InsertTreeCmd 0x48
rotation/variant/flags/height 不可表达（T2）。均已在 RESTORE 登记，非伪报。

**verdict：需修**（FIX-C2：T5 载荷类型；T4 资源取 evidence；T2 随 core）。

### 3.5 `ui_move_tree`（0x14226b1a0）

**独立复述**：门禁 `mode∈{0x10,0} && !DEMO`（0x14362c098）；候选循环（`Decorators::iter_removed_ids`
+ PrefabName 匹配 + 每树 Transform）→ 生成/取 `move_tree_{id}` widget + sounds；
`CursorTerrainRaycast::position`，y<0 → `(y/-2.5)*-1.2 + (1-y/-2.5)*0`；
`dragged()` → `bVar7 = state-2; if 2<bVar7 {3}`（**u8：state 0/1 也落到 case3**）：
case1 记旧位+偏移；case2 界内 MoveTreeCmd、界外双事件；case3 界内 MoveTreeCmd + History TreeEdit Move
（载荷旧位+新位），界外无动作；**不清 Local**。

**对照**：Rust 门禁/水下重映射/`wrapping_sub(2)`+`>2→3`（与 u8 下溢一致）/case1/2/3/双事件/
History 调用顺序逐条一致；不清 Local 正确。缺口：
- T3：History Move 载荷缺旧位（原生 payload+0x14/+0x1c 写 Local 旧位）；
- 候选源以 `storage.iter()` 代理 `iter_removed_ids`（已登记分歧 9）；
- **MoveTreeCmd 载荷布局**：原生事件槽 stride 0x20 = EventInstance 头 8B + payload **0x18**
  `{u64 索引@+8, Vec3@+0x10}`（消费端 0x142267430 反汇编：`TreeStorage::get([ev+8])`、
  写位 +0x10/+0x18）；Rust `MoveTreeCmd{id,index,position}` = payload 0x20 → EventInstance stride 0x28，
  多一个 u64。生产/消费在 Rust 内部自洽，但与原生布局不等（coregap-1 将 +0 记为 id 属把
  EventInstance event_id 当载荷字段）。
- WidgetParams 的 sounds/volume 细化未承载（已登记）。

**verdict：需修**（FIX-C3：补旧位；FIX-C4：MoveTreeCmd 去多余 id → payload 0x18；候选源按
`iter_removed_ids` 落库后替换）。

---

## 4. FIX 清单（按优先级）

| # | 级别 | 条目 | 修法 | 涉及文件 |
|---|---|---|---|---|
| FIX-T1 | **高** | toolbar 1–4 | 在 `country_core::components` 落 `UiInteractionState` 3B `{state,keep,selected}`（原生类型归属），toolbar 引用它；四系统改用它并移除 `WidgetByteState` Query 依赖；随后把四系统接入 schedule（当前 ToolbarPlugin build 未实现） | `crates/country-core/src/components/` + `crates/systems/toolbar/src/{toolbar_button.rs,ui_interaction.rs,resouces_and_components/mod.rs,lib.rs}` |
| FIX-T2 | 高 | tod_v2 | 清 visible 门改读 `ModeJustChanged` 首事件 `tag!=0xd`；顺序改 click→clear→selected；补 `UiNavState+0x10=0`（布局取证后） | `toolbar_button.rs` |
| FIX-P1 | 高 | Fish | `Vec3::new(row,0.0,col)`；公式改 `t*52.0 - (1.0-t)*52.0`；补 used 字节或保持 DIVERGE 登记 | `crates/systems/critters/src/components.rs` |
| FIX-P2 | 高 | Duck | `Vec3::new(row,0.0,col)`（其余已判等） | 同上 |
| FIX-C1 | 中 | replay_tree | `TreeEditData` 补 previous position；Move undo 用旧位发 MoveTreeCmd | `crates/country-core/src/resources/history/tree_edit.rs` + `crates/systems/tree-tool/src/replay_tree.rs` |
| FIX-C2 | 中 | process_tree Insert | T5：`SpawnPrefabCmd` 0x50 全载荷落库并写入（prefab/entity/pos/scale/rot/variant/flags/mode）；T4：is_corrupted/drag_started 资源取证 | `crates/systems/tree-tool/src/process_tree_cmds.rs` + core 类型 |
| FIX-C3 | 中 | ui_move_tree | History Move 载荷补旧位（同 FIX-C1 类型）；候选源替换 | `crates/systems/tree-tool/src/ui_move_tree.rs` |
| FIX-C4 | 中 | MoveTreeCmd | 去掉多余 `id` 字段：payload = `{index:u64@0, position:Vec3@8}` = 0x18（EventInstance 0x20）；同步 replay/ui_move/process 三处 | `crates/country-core/src/resources/tree_storage.rs` |
| FIX-P3 | 低 | purelogic RESTORE 措辞 | `FishSpawnLocations::new` 的“公式恒等/行列一致”、`draw_water 注册判等` 需更正为本次结论 | `reports/integration/purelogic/RESTORE.md` |

> 说明：FIX-T1 与 FIX-C4 属 core 类型改动，需集成 owner 协调；本次只报告，未改 src。

---

## 5. 证据索引（`work/reviewer-A-toolbar/`）

- `jobs/fresh-decomp.json` + `results/fresh-decomp.result.json`：11 函数 fresh 反编译。
- `evidence/fresh-0x1423414b0-toolbar_button_keep_focus.c`、
  `fresh-0x1423427f0-switch_toolbar_category_button.c`、`fresh-0x142342f30-tod_v2_button.c`、
  `fresh-0x14233b3f0-ui_animation_state_update.c`、`fresh-0x140f909f0-new.c`、
  `fresh-0x140f8ee90-new.c`、`fresh-0x141bc34f0-replay_tree.c`、`fresh-0x140de50a0.c`、
  `fresh-0x1407ce920.c`、`fresh-0x142267430.c`、`fresh-0x14226b1a0.c`。
- `jobs/disasm-fish-ctor.json` + `results/...`：Fish/Duck 构造器逐条 store（行列/used/timer 定案）。
- `jobs/disasm-insert.json` + `results/...`：Insert 门控 `deny_load`/`MAXSS`/selector 反汇编。
- `jobs/decomp-consumers.json` + `evidence/fresh-0x140f90cd0-consumer.c`、
  `fresh-0x140f8f210-consumer.c`：Fish/Duck 候选消费面（Vec2→x/z）。
- `jobs/xrefs-toolbar.json` + `results/...`：四系统 caller mangled Query 签名（2 组件定案）。
- `jobs/desc-uiinteraction.json`、`desc2.json`：`UiInteractionState` 两次注册 descriptor size=3。
- `jobs/decomp-rtp.json` + `evidence/fresh-14226a170-random_tree_prefab.c`：随机 prefab 分支。
