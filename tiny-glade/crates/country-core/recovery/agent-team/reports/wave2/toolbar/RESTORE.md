# RESTORE — wave2 toolbar（RE-ANALYSIS 解码成果落库复核）

> restorer-wave2-toolbar，2026-10-17。任务：把第 1 轮 RE-ANALYSIS
> （`reports/re-analysis/hardpoints-interaction/RE-ANALYSIS.md` §4 toolbar +
> §2 onboarding + §3 wall-constructor；`hardpoints-ecstate/RE-ANALYSIS.md` §8）已解码
> 逻辑实现进 `crates/systems/toolbar/src` 函数体（逐语句判等，硬标准）。
> 方法：fresh Ghidra 反编译/签名（shared worker，`jobs/wave2-signatures.json` /
> `wave2-xrefs.json` / `wave2-find.json`）+ 既有 radare2 全函数反汇编 + 跳表 .rdata
> 离线解码对回。证据：`wave2/toolbar/`（本目录）与 re-analysis `evidence/` 子目录。
> 身份：B patched EXE（ghidra shared DB 核对通过）。

## 0. 结论速览

**6 个 toolbar crate 内函数全部 `未实现（重建阻塞）`**——不是"没解码"，而是跳转表
解码之外的真实**类型/参数布局/依赖**仍缺，硬标准禁止近似体。各函数的具体缺口与
下一步分派见下表与各节。onboarding / wall-constructor 4 个函数不属本 crate 交付
范围（其各自 crate 已有 restorer 实现/登记），见 §7。

| 函数 | VA | 解码状态（wave1） | wave2 复核判定 | 阻塞点（下一步） |
|---|---|---|---|---|
| `place_childen_impl<XY/YX>` | 0x140fe12a0 / 0x140fe19a0 | 已攻克（跳表 0x142bc223c/0x142bc2260） | **未实现-重建阻塞** | 真实参数布局（parent 指针 12B、children stride 0x18 `{ptr,ptr,f32}`）未解析；LayoutStyle gap/size/min 读面为 LengthPercentage（非 Vec2）；justify idx9..32 case 体未逐段取证 |
| `add_toolbar_item` | 0x141e4dc40 | 已攻克（kind 跳表 0x142f8f774） | **未实现-重建阻塞** | 8 spawn 组件中 5 个未落库（TaffyNode/UiScaleWithUserSetting/UiAnimationState/UiMouseInteractable/UiInteractionState）+ UiShowIfParentIsInFocus/ShowOnlyInFpvAndFree/ToolbarItemKind 枚举未落库；kind handler（0x60B/0x20B 记录、朝向 f32 表）载荷类型 U |
| `toolbar_alignment` | 0x142344560 | 已攻克（跳表 0x14304e6d8 20 项） | **未实现-重建阻塞** | taffy_mut_style 单态 0x142376050/0x142375ce0/0x142375b40 需 taffy+ToolbarItem 组件；对齐位 0x1020300 语义 I |
| `toolbar_auto_hide` | 0x142343a10 | 部分（常量已读） | **未实现-重建阻塞** | 多 SystemParam 接线（栈参密集）；taffy setter 目标语义 U |
| `ui_sounds` | 0x14233a7f0 | 部分攻克（地址更正） | **未实现-重建阻塞** | ~18 case switch 多处截断（"Bad instruction data"）；音频事件/组件类型未落库 |
| `ui_slider` | 0x1423486d0 | 部分攻克 | **未实现-重建阻塞** | ~28 case switch 多处截断；MainCamera/BrushSizes/mei DSL 接线缺 |

**本 crate src 变更**：`mei/layout.rs` 与 `lib.rs` 文档更新（记录 wave2 验证的
布局读面/跳表/参数分歧）；`cargo check -p toolbar` **0 错误 0 警告**。

## 1. fresh 复核证据（wave2 本会话）

- `jobs/wave2-signatures.json` → Ghidra shared：8 函数 signature/size/decompiled_code。
- `jobs/wave2-xrefs.json` → `place_childen` 0x14068ea20 仅 DATA 引用
  （0x143674afc，函数指针表）；`place_childen_impl<XY>` 0x140fe12a0 调用自
  0x14068ea4a（place_childen）+ DATA 0x1436d008c。
- `jobs/wave2-find.json` → `mei_post_update` 0x1406884c0（7958B，place_childen
  潜在调用侧）未实现。
- 跳表离线解码（planning/jobs/r2-jumptables.result.json 原始字节）：XY 表
  0x142bc223c idx0..32、YX 表 0x142bc2260 idx0..23（两表物理重叠=LLVM 合并尾区，
  base 不同解释出不同 target）。

## 2. `place_childen_impl<SwizzleXY/YX>`（0x140fe12a0/0x140fe19a0）· 未实现-重建阻塞

判等/重建：**未实现**。Ghidra decompile 在跳表处截断（§0.3 机理，`void(void)`
占位）；radare2 全函数反汇编（`evidence/140fe12a0-place_childen_impl-full.disasm.txt`）
为本函数唯一可信读源。

**已重建的前段算法（逐语句，从反汇编）**：
1. 解父 padding[0..3]（LengthPercentage flag@+0/+8/+0x10/+0x18，value@+4/+0xc/+0x14/
   +0x1c）与 gap（flag@+0x20，value@+0x24，置位时 × 可用）。
2. `可用 = parent主轴 − (padding_l+r) − (n−1)·gap`。
3. 遍历子（stride 0x18 = `{ptr@+0, ptr@+8, f32@+0x10}`）：
   - `[child+8]` → 子 padding[0..1] 读面（LengthPercentage）；
   - `[child+0]` → 子 style 读面：size flag@+0x28/value@+0x2c、min flag@+0x30/
     value@+0x34、expand value@+0x38（权重累加源）；
   - `[child+0x10]` → 上一遍计算主轴尺寸（f32）；
   - NaN-safe max 聚合 → sum。
4. `剩余 = parent主轴 − (pad_l+r) − (n−1)·gap − sum`；`weight = clamp01(1−Σexpand)·剩余`
   （0x140fe14b4..0x140fe14cc）。
5. `switch(byte style+0x46)` → 跳表 0x142bc223c（XY）/0x142bc2260（YX）：
   - idx 0→0x140fe1569（start 直排）、1→0x140fe155c（×0.5）、2→0x140fe1564（同 1）、
     3→0x140fe14ef（÷(n−1)）、4→0x140fe1801（÷n）、5→0x140fe1713、6→0x140fe179e
     （center ×0.5）；
   - 各 case 内按 `style+0x45` align 子跳表 0x142bc2250（XY）/0x142bc2274（YX）
     逐子装配 ComputedLayout（stride 0x10 写出）。
6. 尾部 Vec {ptr, cap, len} 经 sret（rcx 隐藏返回指针）写回。

**分歧点（未解析 → 阻塞）**：
- **真实参数布局**：parent_size 以指针传入（读 12B：x@+0/y@+4/8B@+8）；children
  元素 stride **0x18** `{ptr,ptr,f32}`——与本文件 Rust 签名 `(parent_size: Vec2,
  children: &[ComputedLayout])`（stride 0x10）不符。调用侧 `place_childen`
  （0x14068ea20）仅 DATA 引用（0x143674afc 函数指针表），需 mei_post_update
  （0x1406884c0，7958B 未实现）或数据表接线解析。
- **LayoutStyle 字段类型**：gap/size/min 读面为 LengthPercentage（flag+value，非
  Vec2）；expand 以 value@+0x38 作权重源。现有 `gap: Vec2 / size: Vec2 / min: Vec2 /
  expand: Vec2` 为骨架推断，**分歧已登记**（src 未改，default/horizontal 全零字节
  两解释等价，仍判等）。
- justify idx 9..32 case 体（Ghidra 截断区）与对齐子跳表 handler 未全部逐段取证
  （radare2 定点反汇编可续）。

src 位置：`crates/systems/toolbar/src/mei/layout.rs`（占位 `Vec::new()`，注释已更新）。

## 3. `add_toolbar_item`（0x141e4dc40）· 未实现-重建阻塞

路径：`toolbar::resouces_and_components::toolbar_layout::add_toolbar_item`，3341B。
Ghidra decompile **完整可用**（`evidence/ghidra-0x141e4dc40-*.c.txt`）+ kind handler
定点反汇编（`evidence/add_toolbar_item-*kind*.disasm.txt` ×6）。

**已解码**（判等级可核对）：
- `*in_R9` = ToolbarItemKind 判别子。
- kind 4/5 = 容器：`Taffy::new_leaf` + `add_child`（each `.unwrap()`）→ 遍历子项
  （stride 0x40，`*(char*)(item+0x38) != 2` 为遍历界）递归 `add_toolbar_item()` →
  `drop_in_place<ToolbarItemKind>`。
- 其余 kind：克隆 String（in_R9+0x20/+0x28）→ `Commands::spawn<TaffyNode,
  UiScaleWithUserSetting, UiInteractionState, UiAnimationState, UiMouseInteractable,
  Ui2dExtent, Ui2dImage, UiDrawOrder>` → 按 `in_R9[0x38]` 插 `UiShowIfParentIsInFocus`
  （+ 条件 `ShowOnlyInFpvAndFree`）→ kind 跳表 0x142f8f774。
- kind 跳表（8 项）：kind0→0x141e4e5f3（构造 **0x60B** 记录
  `{fn@0x140fdf9a0,0,0,0,0,0,rbx,r13}`）；kind7→0x141e4e7e0（**0x20B** 记录
  `{fn@0x140fdf500,r13,r12,0}`）；kind2→0x141e4e817（子判别 + 三张 f32 朝向表
  0x142f8f6d8/0x142f8f70c/0x142f8f740，调 0x140c904a0）；kind1/3/6→
  0x141e4e9fe/0x141e4e95b/0x141e4e911。

**分歧点（阻塞）**：
- 8 spawn 组件中 **5 个未落库**：`TaffyNode`/`UiScaleWithUserSetting`/
  `UiAnimationState`（toolbar::resouces_and_components）、`UiInteractionState`/
  `UiMouseInteractable`（country_core::components）——已落库的仅 Ui2dExtent/Ui2dImage/
  UiDrawOrder（country-core components/trees.rs，typed gap）。
- `UiShowIfParentIsInFocus`/`ShowOnlyInFpvAndFree`（toolbar::resouces_and_components）、
  `ToolbarItemKind` 枚举（判别子 0..7，变体名 U）未落库。
- 0x60B/0x20B 记录、`[rax+0x18]` vtable 回调、0x140c904a0（朝向）载荷类型 U。

src：无（本函数属 `resouces_and_components::toolbar_layout`，未建档）。

## 4. `toolbar_alignment`（0x142344560）· 未实现-重建阻塞

路径：`toolbar::toolbar_systems::toolbar_alignment`。Ghidra body_end 0x142344647
在跳表处截断（§0.3）；头反汇编可用（`evidence/142344560-toolbar_alignment.disasm.txt`）。

**已解码**：前置遍历工具栏项（stride 0x28，`byte[item+0x24] != 4` 时清对齐位：
`0x1020300 >> (byte[r13+0xf0]<<3)` 写 `[item+0x24]`）→ `switch(byte[r13+0xf0])` 经
跳表 0x14304e6d8（20 项）。alignment 0→0x142344648：调 0x142376050/0x142375ce0/
0x142375b40（taffy_mut_style 单态）设宽高。

**分歧点（阻塞）**：0x142376050/0x142375ce0/0x142375b40 依赖 `taffy` 树 + ToolbarItem
组件（stride 0x28、+0x24 对齐位）未落库；alignment 枚举（0..19）→ 锚点名 U；
`0x1020300 >> n<<3` 位掩码语义 I。

## 5. `toolbar_auto_hide`（0x142343a10）· 未实现-重建阻塞

路径：`toolbar::toolbar_systems::toolbar_auto_hide`，1675B。多 SystemParam（栈参
密集：`in_stack_00000028..0x48`）。常量已读：`110.0`、`-1.0`、`10.0`、`0.3`、`-0.5`、
`0.5`、`{80,40}`/`{40,80}`（0x14304e300/0x14304e310）、`200.0`——自动隐藏阈值/边距。

**分歧点（阻塞）**：系统参数接线（taffy setter 目标、工具栏尺寸资源）、
`[0x142376050 族]` setter 目标语义 U；Ghidra "Control flow encountered bad
instruction data" 截断区未取证。

## 6. `ui_sounds`（0x14233a7f0）· 未实现-重建阻塞

路径：`toolbar::ui_interaction::ui_sounds`，2608B。**地址更正**：总账原 0x14233c450
实为 `ui_select_animation`；真 `ui_sounds`=0x14233a7f0。Ghidra decompile 33KB
（`evidence/ghidra-0x14233a7f0-*.c.txt`）。

**已解码**：`switch(bVar6)` ~18 case（0..0x14），前段两路遍历窗口/聚焦项（stride
0xa0/0x48），`ucomiss xmm6,[r13+8]` 比较、`seta byte[rbx+2]`；另有 0x14233fc6a /
0x14233fe2d 两处未恢复跳表。

**分歧点（阻塞）**：多 case 截断（"Bad instruction data"/"Sanity check requires
truncation"）需 radare2 定点反汇编 case target；音频事件/组件类型（PlayUiSound
载荷面）未接线。

## 7. 范围外函数（onboarding / wall-constructor crate）

任务范围列出的 4 个函数**不属 toolbar crate**（交付约束：只改
`crates/systems/toolbar/src/`）。其 crate 已有对应 restorer 实现/登记，wave2 未改动：

| 函数 | VA | 归属 crate | 现况（抽查） |
|---|---|---|---|
| `hint_position_update` | 0x141c50510 | `crates/systems/onboarding`（hint_display_system.rs:156） | 已建档（stub/重建由 onboarding restorer 负责） |
| `hint_focused_edit_hint` | 0x14227dd60 | `crates/systems/onboarding`（hint_focused_edit.rs:35） | 已建档 |
| `process_playermade_stair_pillars_proposals` 宿主 | 0x140178a70 | `crates/systems/wall-constructor` | 已建档 |
| `construct_playermade_stairs_arches_n_walls<true>` | 0x140496800 | `crates/systems/wall-constructor`（construct_playermade_stairs/mod.rs:45） | 已建档（`<const WALLS: bool>`） |

其 RE-ANALYSIS 解码成果（§2.1/§2.2/§3.2/§3.3）由对应模块 restorer 消费。

## 8. 未实现总账更新建议（给 coordinator）

- toolbar §4 六函数：`decompile-hardpoint` 已攻克（跳表已解码）但**转 `core-type-gap`**——
  真实阻塞是类型/参数布局，不是反编译。
  - `place_childen_impl`：需 core-task（调用侧 mei_post_update 接线 + LayoutStyle
    gap/size/min 字段类型按 LengthPercentage 校正）。
  - `add_toolbar_item`：需 core-task（TaffyNode/UiScaleWithUserSetting/
    UiAnimationState/UiInteractionState/UiMouseInteractable/UiShowIfParentIsInFocus/
    ShowOnlyInFpvAndFree/ToolbarItemKind 落库）。
  - `toolbar_alignment`/`toolbar_auto_hide`：taffy 已入 workspace，需 toolbar item
    组件类型 + setter 语义。
  - `ui_sounds`/`ui_slider`：需 radare2 定点反汇编截断 case 体 + 音频/相机类型接线。
- **既有 判等 声明需复核**：`LayoutStyle::place_childen` 0x14068ea20 分派本身判等
  成立，但 impl 参数布局与 `LayoutStyle` 字段类型（gap/size/min=Vec2）为骨架推断，
  与反汇编读面分歧——建议 reviewer 在类型校正后复核 `default`/`horizontal` 字节声明。

## 9. 证据索引（本目录 + re-analysis evidence）

- `jobs/wave2-signatures.json` / `wave2-xrefs.json` / `wave2-find.json`（本目录，可重放）。
- `r2-jumptables.result.json`（planning/jobs）→ 本报告 §2 跳表解码原始字节。
- `140fe12a0-place_childen_impl-full.disasm.txt`、`ghidra-0x140fe12a0-*` /
  `ghidra-0x140fe19a0-*`（XY/YX decompile）。
- `ghidra-0x141e4dc40-*.c.txt` + `add_toolbar_item-*kind*.disasm.txt` ×6。
- `142344560-toolbar_alignment.disasm.txt`、`142343a10-toolbar_auto_hide.disasm.txt`、
  `ghidra-0x14233a7f0-*.c.txt`、`ghidra-0x1423486d0-*.c.txt`。
- 编译：`cargo check -p toolbar` exit 0（0 错误/0 警告）。
