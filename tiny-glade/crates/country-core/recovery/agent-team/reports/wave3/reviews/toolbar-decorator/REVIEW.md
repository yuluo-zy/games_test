# REVIEW — wave3 toolbar 布局/交互族 + decorator visual 族（reviewer-wave3-toolbar-decorator）

> reviewer-wave3-toolbar-decorator · 2026-10-17。独立复核 `reports/wave3/toolbar/RESTORE.md` 与
> `reports/wave3/decorator/RESTORE.md`。方法：ghidra `--worker shared` fresh 反编译（本 reviewer
> 独立 job，`reports/wave3/reviews/toolbar-decorator/jobs/`：rev-1423414b0 / 142341fe0 /
> 1423427f0 / 142342f30 / 14233b3f0 / 142344560 / 1407ce920 / 1407b5980 / 140818df0 /
> 14080c8d0 / 1412ab270）→ 独立复述 → 与 Rust 逐语句对照 → verdict。身份：B patched EXE
> （ghidra shared 核对通过，`cf048e27…`）。未改 src。
>
> 硬标准（真实逻辑非形状）：近似/高估一律标未实现或需修；判等标签只在逐语句对上的范围成立。

## 0. 结论速览

| 函数（VA） | 报告判定 | 复核判定 | 说明 |
|---|---|---|---|
| `toolbar_alignment`（0x142344560）| 重建（前置+分派）+未实现 | **重建+分派判等；前置掩码保真建议** | 前置掩码缺显式 `& 3`（release 行为等价、debug 健壮性，见 §3）|
| `toolbar_button_keep_focus`（0x1423414b0）| 重建（核心判等）+分歧 | **重建（近似）+分歧** | keep 字节以 `cooldown>0` 代理，非判等；keep 分支应写 0.3；步骤 3/4 未实现 |
| `switch_app_mode_button`（0x142341fe0）| 重建（核心）+分歧 | **未实现（判定过强）** | Rust body 为空壳；二进制事件 push/判别比较/动画插入均未实现 |
| `switch_toolbar_category_button`（0x1423427f0）| 重建（核心）+分歧 | **重建（近似）+分歧** | 动画插入循环漏 2 个门（ModeJustChanged 存在、未选中）；click→WindowFocusEvent 未实现 |
| `tod_v2_button`（0x142342f30）| 重建（核心）+分歧 | **重建（近似）+分歧** | toggle/insert 未按 widget 命中门控；漏 `event!=0xd` 过滤 |
| `ui_animation_state_update`（0x14233b3f0）| 重建（公式判等）+分歧 | **公式判等；实现近似** | 公式对；hardcode entering=false/sign=+1；state 字节写面未登记 |
| `maintain_dormer_tiles`（0x1407ce920）| 结构判等；GPU 退役未实现 | **结构判等+需修（游标写序）** | 游标"写-end-前置"与 Rust"循环后写-end"末值不同（见 §3 FIX-E）|
| `window_lights`（0x1407b5980）| 常量/灯态/帧末判等；收集未实现 | **判等（部分）+未实现（收集）** | 0.51/0.44/0.72、retain 次序、帧末均判等；NaN 检查遗漏（minor）|
| `chimney_smoke_anim_thickness`（0x140818df0）| 窗口/目标/更新判等；收集未实现 | **判等（部分）+未实现（收集）** | 公式全部对上（ReVa+fresh 双证）|
| `replay_decorator`（0x14080c8d0）| 未实现（core-gap）| **判等（结构）+未实现（core-gap）** | DURATION_FLOOR 0.05、cull 表、tag-7、反序删除全对上；Rust 诚实壳 |
| `visual_entity_builder build`（0x1412ab270/0x1412a9b30）| 未实现-需重做（近似体）| **判等（结构）+未实现-需重做** | mesh 分派/impl$4-5/条件链全对上；NaniBundle 未落库登记准确 |

**类型校正复核**：`UiAnimationState` 16B（appear@0/select@4/state@0xc）与 `UiInteractionState`
4B（stride-4 u32 写面）**均判等**（本 reviewer fresh 读面独立核对，见 §2）。

**近似核验结论**：5 处重建含近似/判定过强需修（switch_app_mode 降级、keep_focus 代理、switch_category/tod
漏门控、maintain 游标写序），另有 toolbar_alignment 掩码缺显式 `& 3`（release 行为等价，保真/debug 建议）、
ui_animation_state_update state 字节写面登记不完整——其余未实现/判等登记诚实准确。

## 1. 判等面（fresh 独立复核成立）

- **`toolbar_alignment` 分派跳表 0x14304e6d8**：case 0=`taffy_mut_style<closure_env_12/11/10/11>`、
  1=`<closure_env_0/undo_redo::1/2/13>`、2=`<undo_redo::0/1/2/11>`、3=`<12/13/10/13>`、9/10/0xb=直设、
  0xc/0xe=Easing、0xd=EscMenuState::is_closed+Aabb2+DigitalActionState::pressed+`(DecoratorId,
  Vec<PreferredNodeAlignment>)` grow_one——逐 case 与 fresh 反编译对上。
- **taffy_mut_style 骨架**（0x142375b40..0x142376300）：`style→Result==4→clone→set_style→unwrap`，
  closure 调用被 LLVM 内联不可见（登记 setter 目标 U 诚实）。
- **`ui_animation_state_update` 公式**：`select=clamp01(select±dt/0.2)`、`appear=clamp01(appear+4dt·sign)`
  ——`pfVar18[1]`/`*pfVar18`/`pfVar18[3]` 读面对上。
- **`window_lights`**：`deny_load()?0.44:0.51` 下界、0.72 上界、`lights_on` 写 +0x30、灯态变化
  `target!=lights_on→timer=2·rng.f32()`、帧末 `timer-=dt 钳0，==0且非NaN→actual=target`、
  `time_continuous_lerp`；**retain（LAB_1407b62ed）确在帧末循环之前**（报告次序修正成立）。
- **`chimney_smoke_anim_thickness`**：窗口 `t<0.72\|\|t>=0.9`、目标 `r*3+(1-r)`/`r*4+2*(1-r)`、
  符号恒-1/按窗口±1、`new=clamp01(old+sign·dt/target)`——与 ReVa 全量及 fresh 双证一致。
- **`replay_decorator`**：`front_mut_bucket`→`Arc<[DecoratorId]>` cull 表（指针置 0 取出，逐 id
  insert 到 archivist 集合，Arc drop）→ 扫描 tag-7 0x30B 命令（flags bit0 立即完成 /
  `max(duration/speed, **0.05**)`<elapsed 完成 / elapsed+=delta）→ 完成索引反序 `Vec::remove` +
  tag 释放跳表（0/1/2=RasterEdit、3=TerrainEdit、4=WallReplayCmd drop、5=RoofEdit、6/7=DecoratorHistoryEdit、
  8=TreeEdit、9=CrashMiscInfo、default=winit PhysicalSize）。**DURATION_FLOOR 0.05 修正成立**
  （fresh `if (fVar22 <= 0.05) fVar22 = 0.05`）。
- **`maintain_dormer_tiles` 结构**：`prev=*cursor`、startA/startB 钳 0、len' 钳 0、`*cursor+=1`、
  ids push（`+0x1c` 值）、`hash_one<RandomState,&GlobalId>`+`RawTable::remove_entry`+条件
  `drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>`、回写表达式 `total-lenB'-lenA'`
  与 fresh 逐语句对上。**已删虚构"瓦变换写 FUN_1405b0d70"注释确认**（fresh 无此调用）。
- **`visual_entity_builder build`**：pos/rot（+0xc0..+0xcc）、旋转分量（+0xd0/+0xd8）→ 四元数旋转；
  mesh 名 len switch 9..0x24 → 12 类 `NaniBundle<T>` spawn + 默认 `DrawableMeshBundle`；条件组件链
  14 位偏移（+0xf0 bit0/+0x40==1/+0x114==2/+0x80==1/+0x14e/+0x10 bit0/+0x14f/+0x14c!=2/+0x20==1/
  *+0 bit0/+0xf8==1/+0x13c!=2/+0x118==1/+0x90==1）全对上；build<Persistent> 末尾=impl$4（0x1408119e0）、
  build<Transient> 末尾=impl$5（0x140811c30）——**两 build 同体、impl$4/5 目标登记准确**。

## 2. 类型校正复核（本 reviewer fresh 读面独立核对）

| 类型 | 校正 | 复核 |
|---|---|---|
| `UiAnimationState` | 16B `{appear f32@0, select f32@4, _gap4, state u8@0xc, _pad3}` | **判等**：ui_animation_state_update `pfVar18` stride 0x10、appear=*pfVar18@0、select=pfVar18[1]@4、`*(pfVar18+3)` 字节@0xc 逐语句对上 |
| `UiInteractionState` | 4B `{state@0,_pad@1,flag@2,_pad@3}` | **判等**：keep_focus/animation/switch 系均 `*(u32*)(idx*4+base)` stride-4 写面；ui_2d_pass 读 +2 字节佐证 |
| `ToolbarButtonCooldown` | f32@0 | 判等：keep_focus `*(f32*)(idx*4)` 写面 |
| `UiPlaySelectAnimation` | 16B `{a f32@0,b f32@4,u32@8,byte@0xc}` | 判等：命令记录 0x28B payload@+0x18，观测 `{0.1,0.2}`(0x3dcccccd3e4ccccd)/`{0.2,0.2}`(0x3e4ccccd3e4ccccd) 与 switch_app(0x3e4ccccd3e4ccccd)、switch_category/tod(0x3e4ccccd3dcccccd) 对上 |

`UiClickEvent{entity}`（EventInstance @+8）判等。`UiNav`/`HistoryButton`/`WindowFocusEvent` 未落库
登记准确（ledger §1 interaction/toolbar 已有）。

## 3. 需修（近似/判定过强）

### FIX-A：`toolbar_alignment` 前置清位掩码缺显式 `& 3`（0x142344560，保真/健壮性）
二进制：`uVar25 = 0x1020300 >> ((*(byte*)(GameConfig+0x1e) & 3) << 3)`；
Rust（toolbar_systems.rs:85）：`0x1020300u32 >> (alignment << 3)`——**缺显式 `& 3`**。
本 reviewer 逐值核对：`(a << 3) & 31 ≡ (a & 3) << 3`（u32 release 移位掩 5 bit），故 **release
行为等价**（alignment 0..19 全同），**非功能 bug**；但①debug 下 `u32 >> 32`（alignment≥4）panic，
②依赖掩移恒等式而非原样表达。建议改 `0x1020300u32 >> ((alignment & 3) << 3)` 保真 + debug 稳健。
另：Rust 为 helper 签名（alignment, items, taffy），非二进制 SystemParam（Res<GameConfig>/Res<ToolbarLayout>/
Res<PhotomodeToolbarLayout>/ResMut<TaffyLayout>）——签名差异应登记。

### FIX-B：`switch_app_mode_button` 判定"重建（核心）"过强 → **未实现**
Rust（toolbar_button.rs:95-113）body 为空壳（两个空循环 + `let _ = &mut commands`）。二进制核心
（点击→`grow_one<(ref str, SettingsMenuState)>`、AppMode 判别 0..6 变体比较、相等→
`insert(UiPlaySelectAnimation{0.2,0.2})`）**均未实现**。判定应降级为"未实现（二进制已描述、分歧
登记）"。

### FIX-C：`switch_toolbar_category_button` 动画循环漏 2 门
二进制条件 = `(ModeJustChanged 事件存在) && (per-widget +2 != 1 未选中) && (toolbar.current==category)`；
Rust（toolbar_button.rs:142-151）只判 `toolbar.current == button.category`——**漏事件存在门与未选中门**
（未登记）。click→WindowFocusEvent push 未实现（登记）。per-widget +2 写面/`interaction=tick` 未实现
（登记）。

### FIX-D：`tod_v2_button` toggle/insert 未按 widget 命中门控
二进制 toggle `visible^=1` + `insert(UiPlaySelectAnimation{0.1,0.2})` 在 **点击实体命中
TimeOfDayButtonV2 Query 内**才执行；Rust（toolbar_button.rs:185-188）对**每个 click** 无条件
toggle+insert（未登记）。`清 visible` 条件二进制为 `首事件 tag!=0xd && visible==1`，Rust 为
`!click_events.is_empty() && visible`——漏 `!=0xd` 过滤（注释有提但未登记为判定分歧）。
`visible` 偏移 +0xa vs country-core `{visible}`@+0 已登记（正确）。

### FIX-E：`maintain_dormer_tiles` 游标写序
二进制：**循环前**写 cursor=`total-lenB'-lenA'`，每瓦 `*cursor+=1` → **末值=total**；
Rust（maintain_dormer_tiles_draw.rs:118-120）：循环后写 cursor=`total-len_b-len_a` →
**末值=total-(lenA'+lenB')**。末值不同（除非 prev 恰等于末值）。"游标回写判等"须补注写序/末值分歧。

### FIX-F：`ui_animation_state_update` 实现面近似 + 登记不完整
- Rust 恒 `update_progress(dt, false)`（hardcode 离开分支）与 `appear += 4dt`（hardcode sign=+1）；
  二进制按 per-widget 字节记录（U）判别 enter/leave、appear sign=字节+0。已标 U，但"判等（公式）"
  仅指公式算术本身，实现为单路径近似——标签建议"公式判等；分派/符号未实现"。
- **state 字节（+0xc）写入未实现未登记**：二进制每迭代 `*(pfVar18+3)=uVar22`（0/1/2 转写）；
  Rust `update_progress` 不写 state。登记"state 语义名 U"≠"state 写入未实现"，须补。

### minor（登记级）
- window_lights 帧末 Rust 未做 `!NaN` 检查（二进制 `==0.0 && !NAN`；Rust `f32::max` 会把 NaN 归 0）——补登记。
- rng 种子（window_lights `rng_seed()=0`、chimney `delta.to_bits()`）、retain 谓词、lerp 实参
  以代理承载——已登记 U（正确）。

## 4. 未实现（登记诚实、证据准确）

- `toolbar_auto_hide`（0x142343a10）：常量/公式已取证，系统参数接线 U——未实现诚实。
- `ui_sounds`（0x14233a7f0）：9 SystemParam 签名 + `switch` ~18 case 截断
  （"Sanity check requires truncation of jumptable"，fresh 复验）——未实现诚实。
- `ui_slider`（0x1423486d0）：callees + `switch` ~28 case 截断（fresh 复验）——未实现诚实。
- `render_taffy_ui_to_widgets`（0x142338990）：Query init 面 `(Entity,&Ui2dExtent,&UiMouseInteractable,
  &TaffyNode,&mut UiInteractionState,Option<&HistoryButton>,Option<&UiNav>)` + WidgetParams 链
  （is_toolbar/new/no_history_recording/occlude_by_world/sounds/is_active/clicked/into_drag_state/write）
  + `grow_one<EventInstance<WindowFocusEvent>>`/`grow_one<(ref str,SettingsMenuState)>` + format_inner
  ——全对上，接线未实现诚实。
- `window_lights`/`chimney_smoke` 收集主循环（PublicWalls 密集窗桶表 stride 0xa0/0x48，core 未暴露）
  ——未实现诚实。
- `maintain_dormer_tiles` 每瓦 GPU 退役（rhapsody 回调注册表 + RoofTileSsbo SparseSsbo）——未实现诚实。
- `replay_decorator`（core ReplayCommand 无 Decorator 变体 + ReplayBucket 无 Arc<[DecoratorId]> cull
  字段 + 时序内嵌 0x30B）——core-type-gap 登记准确。
- `visual_entity_builder build`（NaniBundle/DrawableMeshBundle 结构体未落库，drawable.rs 仅
  NaniInstanceMeshDesc+DrawableMeshBundle::build 胶水；instance_data 12 类 Ssbo 元素已落）——
  近似占位降级诚实；build_transient→瞬态表 vs 二进制 impl$5 autoclutter 表分歧登记准确。

## 5. core-gap 登记核对（更新建议给 coordinator）

- ✅ 已登记准确：UiNav/HistoryButton/WindowFocusEvent（§1 interaction/toolbar）、rhapsody 回调
  注册表/RoofTileSsbo SparseSsbo/autoclutter 表 Vec<DisplayChildAutoClutterInfo>（§1 state/decorator）、
  PublicWalls 密集表（§1 state/color 待证项延续）。
- **需新增**（本次复核佐证）：
  - **per-widget stride-3 字节记录容器**（keep 字节+1/appear sign+0/选中标志+2；focused-window 子树，
    toolbar_button_keep_focus / ui_animation_state_update / switch_*_button 共享）——§1 interaction/toolbar。
  - **EventWriter 载荷面分歧**：`RequestAppModeChange`/`RequestToolbarCategoryChange` 实写
    `(ref str, SettingsMenuState)`（stride 0x18）/`EventInstance<WindowFocusEvent>`（stride 0x10）
    ——§1 interaction/toolbar（render_taffy 亦写两者，佐证载荷面）。
  - **replay_decorator**：core `ReplayCommand` Decorator 变体 + `ReplayBucket` Arc<[DecoratorId]> cull
    表字段——§1 state/decorator（新条目）。

## 6. 证据索引（本 reviewer fresh）

- `reports/wave3/reviews/toolbar-decorator/jobs/rev-{1423414b0,142341fe0,1423427f0,142342f30,
  14233b3f0,142344560,1407ce920,1407b5980,140818df0,14080c8d0,1412ab270}*.c.txt`（fresh 反编译全文）
- `reports/wave3/toolbar/evidence/*.c.txt`（既有 fresh，与本 reviewer 独立 job 一致）
- `reports/wave3/decorator/evidence/decomp_0x14080c8d0.c`、`decomp_0x1412ab270.c`/`0x1412a9b30.c`
  （既有，与本 reviewer 独立 job 一致）
