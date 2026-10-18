# REVIEW — systems/onboarding（reviewer-interaction-onboarding）

> 2026-10-17。独立审查 restorer-interaction-onboarding 对 `crates/systems/onboarding`
> 的重建是否真正还原 B 版原二进制业务逻辑。
> 方法（按用户 2026-10-16 硬标准）：对每个关键函数 **fresh 反编译**（Ghidra shared
> worker，`get_functions fields=signature,decompiled_code` + 定点 `disassemble_function` /
> `read_memory`），独立复述原函数完整控制流，再与落盘 Rust body 逐语句对照。
> 本审查证据：`reports/interaction/onboarding/review/{jobs,results}/`（rv-batch1..11，
> 可重放，worker shared 只读）。身份：B patched EXE tiny-glade.exe
> （sha256 cf048e27…6323）。

## 0. 结论摘要

- **判等 16**（restorer 判等 9 全部复验通过 + 其余重建项升判）：
  alternate×5 / adjacent×2 / together×1 / filter_to_mode_impl /
  is_some / is_row / hint_animation_system / hint_deletion_cooldown_system /
  hint_deletion_system / progress_bar_update_system / hint_camera_progress_update /
  hint_camera_progress_reset。
- **未实现-需重做 2**（restorer 标"重建"，审查发现具体分歧点）：
  `cleanup_impl`（owned 顶层包裹缺失）、`hint_audio`（tag 0x15 缺删除门控）。
  **复验（2026-10-17 二次审查）：2 处 FIX 均已落实 → 状态 fix_verified（见 §6A）。**
- **glue-legitimate 1**：`add_onboarding`（21 事件注册逐一核对通过）。
- **未实现 36**：分类诚实，stub 为空体+分歧点登记，未发现以近似/待取证混过的函数。
- **FIX 清单 2 条（见 §6）均已由 restorer 落实并二次复验通过（§6A fix_verified）**；
  **文档修正 2 处**（filter_to_mode 0xcd、cleanup 0xc8 默认分支）。
- 编译核验：隔离 harness `cargo test` **9/9 通过**（含新增 cleanup_owned_wraps）、
  src 与 workspace 逐文件一致；workspace `cargo check -p system-onboarding` **0 错误**
  （当前已完成，无 country-core 阻断）。
- 无过度断言：restorer 报告未夸大判等面；但 `cleanup_impl`/`hint_audio` 标"重建"偏高，
  应降级为未实现-需重做。

## 1. 判等 16（逐函数独立复述 → 与 Rust 对照）

### 1.1 组合子泛型 alternate×5 / adjacent×2 / together×1（判等）

原逻辑（fresh 0x141c4e4f0/6d0/8b0/ead0/ecb0、0x141c4ee80/f060、0x141c4f230）：
`if (*in_RDX == 0xca/0xcb/0xcc)` 列表判别成立 → `len==cap` 则 `RawVec::grow_one`，
在 `ptr+len*0x30` 写元素（tag 0xc6/0xc7/0xc5/嵌套 0xc8，元素 48B），`len+1`；
否则 `__rust_alloc` 2 元素，memcpy self 48B 到 [0]，item 写 [1]，`len=2,cap=2`；
末尾写判别 0xca/0xcb/0xcc。
对照 Rust `alternate/adjacent/together<T: Into>`：`Self::Alternate(mut list) => push`
（Vec::push 内部等价 grow_one）／`other => Self::Alternate(vec![other, item])`。
**逐分支等价**：push-or-new、grow_one≡Vec::push 扩容、元素类型映射（Key/Mouse/
Graphic/Row/嵌套）一致。**判等**。

### 1.2 filter_to_mode_impl（0x1422808d0，判等）

原逻辑（fresh）：`uVar3=tag-0xc4`，`uVar1=min(uVar3,9→截断4)`。
手柄（`in_DL!=0`）：0xc5→释放载荷置 0xc4；0xc9 保留；0xca/0xcb/0xcc 逐元素递归
（`len*0x30` 倒序）；其余→0xc4。非手柄：`uVar1<5`（0xc4..0xc8）保留；
`uVar1-6<3`（0xca/0xcb/0xcc）递归；否则（0xc9/0xcd）→0xc4。
对照 Rust `filter_to_mode_impl`：手柄 Graphic→Empty、Steam→keep、列表递归、其余→Empty；
非手柄 keep(Empty/Graphic/Key/Mouse/Row)、列表递归、Steam→Empty。**对 Rust 枚举全部
变体等价**。
**文档修正 1**：非手柄 0xcd（tag-0xc4=9 截断→uVar1=4→`uVar1<5`→return）为**保留**，
非报告所称"0xc9·0xcd→Empty"；Rust 枚举无 0xcd 变体，代码不受影响，仅报告/ANALYSIS 文字
需改。

### 1.3 is_some / is_row（inline，判等-语义）

PDB 仅 inlinee（无独立发射）。语义经二进制使用处反证一致：cleanup 末尾
`*in_RCX - 0xcaU < 3` 即 is_row（0xca/0xcb/0xcc）；各递归/压缩处 `!= 0xc4` 即 is_some。
Rust `!matches!(Empty)` 与 `matches!(Alternate|Adjacent|Together)` 与之等价。**判等（I 级）**。

### 1.4 hint_animation_system（0x141c4f400，判等-逐属性计算）

原逻辑（fresh）：逐属性（元数据表 +8 步长 0x10，数据 {f32 value,f32 eased,f32 time}
步长 0xc）：`fVar=dt/0.3`；`if (deleting) fVar=-fVar`（dir=-1）；`value=value+fVar`；
clamp[0,1]；`eased=EasingFunction::apply(value)`；`time+=dt`；写 change-tick。
对照 Rust `hint_animation_system`：`dir=deleting?-1:1; value=(value+dt/0.3*dir).clamp(0,1);
eased=CubicInOut.apply(value); time+=dt`。**逐属性计算逐语句等价**。
注明：多属性数组结构（属性数/元数据表）未恢复，Rust 取单属性 —— 已登记为 I 级简化，
不构成错误逻辑。EasingFunction 变体 CubicInOut 为 I 级推断（依赖 utils 库已恢复）。

### 1.5 hint_deletion_cooldown_system（0x141c4f790，判等）

原逻辑（fresh）：`timer -= dt`；`<=0` → 队列 remove<HintDeleteAfterCooldown>
（error handler=warn）+ insert<HintMarkedForDeletion>（error handler=panic）。
对照 Rust：`cooldown.timer -= dt; if timer <= 0.0 { cmd.remove; cmd.insert }`。
等价。注：二进制 remove 用 warn、insert 用 panic 错误处理器（bevy 内部细节，命令效果一致）。

### 1.6 hint_deletion_system（0x141c4fea0，判等）

原逻辑（fresh）：Query 以 `With<HintMarkedForDeletion>` 过滤（无 Option 分支）；
逐属性 `eased==0.0（且非 NaN）` → `EntityCommands::despawn`。
对照 Rust：`Query<(Entity,&HintAnimation), With<HintMarkedForDeletion>>` + `eased==0.0`
→ despawn。**等价**（单属性模型同上注明）。

### 1.7 progress_bar_update_system（0x141c530f0，判等）

原逻辑（fresh）：读 HintType u8：`0xa(pan)/0xb(rotate)/0xc(zoom)` 取 CameraHintProgress
进度，非三值 → `core::panicking::panic`。量化：`p==0→0`；`0<p≤0.4→1`；
`0.4<p≤0.99→2`；`p<0 或 p>0.99→3`；**NaN→1**（逗号表达式 `uVar21=1` 副作用）。
档位变（`uVar21 != *bar`）→ 写 PlayUiSound tag 0x16(22)，写回档位。
对照 Rust `quantize` + `progress_bar_update_system`：逐阈值等价（含 NaN→1、0.4→1、
0.99→2、<0/>0.99→3），档位变写 tag 0x16。**判等**。
注：panic 消息文本为重建（原字符串未取读），panic 路径一致。

### 1.8 hint_camera_progress_update（0x14227bd30，判等）

原逻辑（fresh）：快照 `prev_pan/rotate/zoom = 当前`；逐事件累积：`tag==2→rotate +=
dt/rotate_threshold`、`tag==1→zoom += value/zoom_threshold`、其它→`pan += dt/pan_threshold`；
`(prev<1 && cur>=1)` 跨 1.0 边沿 → 写 TutorialHintComplete id 0xa/0xb/0xc。
对照 Rust `hint_camera_progress_update`：快照→match tag 累积→`prev<=1 && prev!=1 && 1<=cur`
写完成事件。**等价**。
注明（轻微）：二进制在事件循环内逐事件检查边沿（一帧多事件可重复写同一完成事件，
下游 tutorial_completion 幂等，无功能差异）；Rust 在循环后检查一次。非 FIX。

### 1.9 hint_camera_progress_reset（0x14227c070，判等）

原逻辑（fresh）：Query 取首个 HintType（无实体→0x13/19）；`prev != cur` →
`0xa(pan): pan=0,pan_threshold=1.4(0x3fb33333)`；`0xb: rotate=0,threshold=1.2(0x3f99999a)`；
`0xc: zoom=0,threshold=120(0x42f00000)`；写回 prev。
对照 Rust `hint_camera_progress_reset`：`query.iter().next().map_or(0x13,...)` +
match 0xa/0xb/0xc 归零+阈值。**逐语句等价**。

## 2. 未实现-需重做（2，restorer 标"重建"）

### 2.1 cleanup_impl（0x14227f4a0）—— 分歧点 1：owned 顶层包裹未复刻【fix_verified】

**独立复述原逻辑（fresh 反编译 + 定点反汇编）**：
- 递归/借位路径（case 0/1/3=Empty/Graphic/Mouse 保留；case 2=Key 载荷==0xc3→Empty；
  default(uVar8=4)=0xc8/0xcd：`*+0` `+0xc` `+0x18` 三处均 0xc3 才→Empty（真 0xc8 恒假，
  保留）；case 5=Steam→Empty；case 6/7=Alternate/Adjacent：逐元素递归→非 Empty 原地前移
  压缩→`len==0→Empty`、`len==1→提升单元素(丢 Vec)`、否则保留；case 8=Together：
  逐元素递归→全空→Empty，否则保留（**不做压缩/提升**）。）
- **owned 顶层路径**（函数末尾）：`if (in_RDX != 0) return;`（借位提前返回）；
  `if (*self - 0xca < 3) return;`（结果是列表则返回）；否则把 self 深拷贝到新分配，
  置 `*self = 0xca; len=1; ptr=new; cap=1` —— **把非列表结果包成 1 元素 Alternate 列表**。

**调用上下文核验（关键）**：xrefs 到 0x14227f4a0 仅 1 处外调 —— `0x14228c555`
（hint_draw::closure$2）。该处反汇编：
```
14228c54f LEA RCX,[RBP-0x30]   ; &mut display
14228c553 XOR EDX,EDX          ; RDX=0（owned！）
14228c555 CALL 0x14227f4a0     ; cleanup_impl
```
递归调用（case 6/7 体内，`e8 84 ff ff ff`）之前为 `LEA RBX,[RSI+1]`（RSI=入口 RDX），
即递归传 **RDX=1（借位）→ 不包裹**。因此包裹仅发生在 owned 顶层。

**对照 Rust**：`cleanup_impl(&mut display)` 对递归/借位路径（Key None→Empty、Steam→Empty、
Alternate/Adjacent 压缩/提升/全空、Together 全空）逐分支等价；但 `cleanup(self)` 返回
叶子本身，**未包裹成 `Alternate(vec![leaf])`**。原二进制经 closure$2 实际执行包裹。
→ **未实现-需重做（分歧点：cleanup() owned 顶层把非列表结果包成 1 元素 Alternate）**。

### 2.2 hint_audio（0x141c4fb00）—— 分歧点 1：tag 0x15 缺删除门控【fix_verified】

**独立复述原逻辑（fresh）**：Query=(&HintAnimation, Option<&HintMarkedForDeletion>)。
逐属性：`if (bVar17/*实体有 HintMarkedForDeletion*/) && (eased(+4)==1.0)` → 写
PlayUiSound tag **0x15**；随后（无门控，逐属性）`if (time(+8)==0.0)` → 写 tag **0x14**。
bVar17 由 archetype 可选组件存在性算出（`0x80` 组件列非 0），即"有删除标记"。
**对照 Rust**：
```rust
for (anim, _deleting) in &query {
    if anim.eased == 1.0 { let _ = sounds.write(PlayUiSound); }  // tag 0x15 ← 缺门控
    if anim.time == 0.0 { let _ = sounds.write(PlayUiSound); }   // tag 0x14 ✓
}
```
**tag 0x15 的写入未门控在 `deleting.is_some()` 上**：原二进制只在"有 HintMarkedForDeletion
且 eased==1.0"（满显后被标记删除，即播放"关闭"音）时发 0x15；Rust 会对任意 hint 在
eased==1.0（含正常浮现完成）也发 0x15。**行为分歧 → 需重做**（FIX 见 §6.1）。
（注：hint_animation_system 的 dir 门控 restorer 做对了；hint_audio 漏了同类门控。）

## 3. glue-legitimate（1）

### 3.1 add_onboarding（0x140650d50，glue-legitimate）

原逻辑（fresh）：`Components::get_resource_id` 幂等检查（`uVar2=0x8e1bd251962ba784`
插件组件 id，已注册则跳过）→ `Schedules::add_systems<Startup>`（hints_startup，
fn(Res<GameProfile>,Commands)）→ `chain_ignore_deferred` + `in_set` → `SubApp::add_systems
<Update>`（提示系统）→ 21× `EventRegistry::register_event`（各事件带 get_resource_id
幂等门控）。
21 事件逐一核对（与 restorer 清单一致）：DisplayHintRequest、LingerHintRequest、
TutorialHintComplete、TutorialOp、NotifyHintSystem{ShapeEditWasPerformed,CameraMoved,
Snapping,Terrain,Window,ToolWasJustUsed,SomethingWasErased,TerrainSteepness,RoofShape,
ClutterWasMoved,StairsPlaced,HoveringOverStairs,StartedStairsContinue,CanContinueStairs,
StairsDetachedFromWall,StairsCanDetachFromWall}、DisplayRollercoasterPrompt。
Rust `configure` 以 `init_resource`/`Events` 表达可落部分（bevy_app 未入 workspace），
未落库事件登记。**glue-legitimate 成立**（确认是 Bevy 注册胶水，非业务逻辑）。

## 4. 未实现 36（分类核验，诚实）

fresh 抽查确认 stub 为空体+分歧点登记，无"近似体通过"：
- **hint_orchestrator**（0x14227fc30，fresh）：事件收集 `from_iter<EventIterator>` →
  `1<len`：`len<0x15` → `insertion_sort_shift_left` 否则 `driftsort_main`（稳定排序）→
  实体游标迁移状态机。stub 空体，分歧点（closure$0 排序键/请求布局/GameProfile 标志）
  与真逻辑一致。✓ 未实现诚实。
- **render_hints**（0x141c52700，fresh）：判别 switch 已核对（Empty 无操作/Graphic
  `floorf(t/frames)`+me_image/Key keycode_to_str（0xc3→占位）/0xc8 四键 4×with_slot/
  Steam `panic_fmt`（"Call resolve_steam() and cleanup()…"）/列表 with_slot closure_env_2/3/4）。
  依赖 toolbar/mei，stub 空体。✓ 未实现诚实。
- **hint_highlight_shape**（0x141c50120，fresh）：`eased>0.9 && HintType 条件 && +0x11!=0`
  → `PublicWalls::roof_entity()` → 向两 writer 写 tag 2 高亮命令。依赖 roof_entity/
  HighlightRoofCmd/HighlightWallCmd，stub 空体。✓ 未实现诚实。
- 其余（resolve_steam_impl、hint_position_update、hint_draw+closure$2、render_hints
  closure$1/2/4、exponential_back_off+2 inline、hints_startup、reset_hints、
  tutorial_completion、相机 5、focused/clutter 5、roof 2、stairs、terrain 3、undo、
  unlink、deselect、disable_snapping、prompt_rollercoaster）：stub 空体，分歧点指向
  共享基础缺口（country-core GameProfile 提示区/18 NotifyHintSystem*/SteamInput/
  WriteSettingsFileCmd/SaveGameProfileCmd/toolbar-mei）。分类与报告一致。✓

## 5. 附加核验

- **冷却表 DAT_143038ae8**（read_memory 80B）：[0..]=0,0.3,0,0,0.3,0.3,0.3,0.3,0,0,
  0.3,0.3,0.3,0,0,0,0,0,0,0 —— 与 Rust `DELETION_COOLDOWN_TABLE` 逐元素一致。✓
- **closure$2 链**（0x14228c260，fresh）：clone → filter_to_mode（RCX=&mut, DL=is_gamepad
  取自 ActiveInputModeTracker）→ SteamInput Option 两资源均 Some 则 resolve_steam →
  cleanup（RDX=0）→ render_hints。与报告描述一致；cleanup 调用点为 owned（见 §2.1）。
- 隔离 harness `cargo test` 9/9 通过（新增 `cleanup_owned_wraps`）；`diff -rq`
  确认 verify/src 与 workspace src 一致。
- workspace `cargo check -p system-onboarding` **0 错误**（2026-10-17 复核完成）。

## 6. FIX 清单

1. **hint_audio**（`src/hint_display_system.rs`）：tag 0x15 写入补删除门控：
   `if deleting.is_some() && anim.eased == 1.0 { write PlayUiSound }`（tag 0x14 不变）。
2. **cleanup() owned 顶层**（`src/hint_orchestrator.rs`）：`cleanup(self)` 结果非列表时
   包成 1 元素 Alternate：
   ```rust
   pub fn cleanup(mut self) -> Self {
       cleanup_impl(&mut self);
       if !self.is_row() {
           self = Self::Alternate(vec![self]);
       }
       self
   }
   ```
   （对应 0x14227f4a0 末尾 in_RDX==0 分支 + closure$2 `XOR EDX,EDX` 调用。）

## 6A. FIX 复验（2026-10-17，fix_verified）

restorer 已落实 2 条 FIX，二次审查核验：

1. **hint_audio**（`src/hint_display_system.rs` L113-114）：
   ```rust
   for (anim, deleting) in &query {
       if deleting.is_some() && anim.eased == 1.0 {  // tag 0x15，对应二进制 bVar17 门控
   ```
   与 0x141c4fb00 反汇编 `if (bVar17) { if (eased==1.0) push 0x15 }` 逐分支一致；
   tag 0x14 无门控保持不变。✓ **fix_verified**。

2. **cleanup() owned 顶层**（`src/hint_orchestrator.rs` L137-143）：
   ```rust
   pub fn cleanup(mut self) -> Self {
       cleanup_impl(&mut self);
       if !self.is_row() {
           self = Self::Alternate(vec![self]);
       }
       self
   }
   ```
   包裹条件 `!is_row()`（{0xca,0xcb,0xcc}）与二进制 `*self-0xca>=3 → 包 1 元素 Alternate`
   逐句等价；`vec![self]` 移动语义产生的值与二进制深拷贝结果一致（深拷贝为 codegen 细节）。
   新单测 `cleanup_owned_wraps_non_list_into_single_alternate` 覆盖"非列表→包裹"与
   "已是列表→不包裹"两分支。✓ **fix_verified**。

复验证据：verify harness src 与 workspace `crates/systems/onboarding/src/` 逐文件一致
（`diff -rq` 空），`cargo test` **9/9 通过**（新增 1 条）；workspace `cargo check
-p system-onboarding` 0 错误。文档修正（filter_to_mode 0xcd 保留/0xc9 才 Empty、
hint_audio 门控注释）均已更新至 Rust 源码注释。**无新过度断言**。

## 7. 文档修正

1. `RESTORE.md`/`ANALYSIS.md`：非手柄 filter_to_mode 对 0xcd 为**保留**（uVar1=4→
   `uVar1<5`→return），非"0xc9·0xcd→Empty"；0xc9 才→Empty。
2. `ANALYSIS.md` §2.3 hint_audio：tag 0x15 需**有 HintMarkedForDeletion 且** eased==1.0。

## 8. 过度断言评估

- 无：restorer 未把 36 个未实现当作实现；判等 9 全部成立；胶水分类正确。
- 有（轻微，已闭环）：`cleanup_impl`/`hint_audio` 曾标"重建"偏高 —— 二者均有可判定的
  行为分歧，审查要求按硬标准标"未实现-需重做"；restorer 已落实 2 条 FIX 并经二次审查
  **fix_verified**（§6A），现两函数递归/门控行为与原二进制逐句一致。
