# RESTORE — wave2b toolbar 类型落库 + 6 函数（place_childen 签名裁决）

> restorer-wave2b-toolbar-types，2026-10-17。任务：把 wave2 判为"未实现（重建阻塞）"
> 的 toolbar 本地组件类型按反编译布局落库，并按反汇编实现 6 个函数（逐语句判等，
> 硬标准；定义不了/还原不了标未实现）。方法：fresh Ghidra/radare2 反汇编 + 跳表
> 离线解码（0x142bc223c/0x142bc2260/0x142bc2250/0x142bc2274/0x142f8f774/
> 0x14304e6d8）+ 原字节核对（`mov rcx,r13` 等 ABI 字节确认）。证据：`wave2b/toolbar/`
> （本目录 jobs/results）与 re-analysis evidence。身份：B patched EXE（ghidra shared
> 核对通过，`cf048e27...`）。

## 0. 结论速览

| 函数 | VA | 判定 | 交付 |
|---|---|---|---|
| `place_childen_impl<XY/YX>` | 0x140fe12a0/0x140fe19a0 | **重建（核心）+ 分歧（尾区）** | 签名裁决 + 核心算法重建（padding/gap/available/sum/remaining/weight + justify 0..8 + 装配） |
| `add_toolbar_item` | 0x141e4dc40 | **判等（控制流）+ 分歧（kind 载荷）** | 类型落库 + 递归/spawn/插入/kind 分派重建 |
| `toolbar_alignment` | 0x142344560 | **未实现（部分重建前置）** | 前置清位循环重建；taffy setter 目标 U |
| `toolbar_auto_hide` | 0x142343a10 | **未实现** | 常量/前段登记 |
| `ui_sounds` | 0x14233a7f0 | **未实现** | 地址更正复核 + switch 结构登记 |
| `ui_slider` | 0x1423486d0 | **未实现** | switch 结构登记 |

**类型落库 8 个**（toolbar 本地，反编译布局）：`TaffyNode`/`UiScaleWithUserSetting`/
`UiAnimationState`/`UiMouseInteractable`/`UiInteractionState`/`UiShowIfParentIsInFocus`/
`ShowOnlyInFpvAndFree`/`ToolbarItemKind`（+ 布局输入 `ParentLayout`/`ChildLayoutInput`，
LayoutStyle 字段类型按反汇编校正）。

**`place_childen_impl` 签名裁决（结论）**：以反汇编为准，签名改为
`(style: &LayoutStyle, parent_size: &ParentLayout, children: &[ChildLayoutInput]) ->
Vec<ComputedLayout>`。原 `(Vec2, &[ComputedLayout])` 骨架推断作废。

**编译**：`cargo check -p toolbar` **0 错误 0 警告**（toolbar crate 本身；
country-core 依赖侧 9 既有警告非本批）。

## 1. 类型落库（`resouces_and_components/mod.rs` + `toolbar_layout.rs`）

按 B 反编译布局（add_toolbar_item spawn 8 组件 tuple + kind 读面）定义，字段语义
I/U 处按 typed gap：

| 类型 | 模块（B） | 布局依据 |
|---|---|---|
| `TaffyNode` | `toolbar::resouces_and_components` | spawn 组件；taffy Node=slotmap DefaultKey 8B |
| `UiScaleWithUserSetting` | 同上 | 16B `{scale, _gap}`（I） |
| `UiAnimationState` | 同上 | 16B `{state, _gap}`（读侧 0x14233b3f0） |
| `UiMouseInteractable` | `country_core::components`（本批 toolbar 暂置） | ui_2d_pass 读 +2 面；16B（I） |
| `UiInteractionState` | `country_core::components`（本批 toolbar 暂置） | 16B `{state,_pad,flag,+2,_gap}` |
| `UiShowIfParentIsInFocus` | `toolbar::resouces_and_components` | add_toolbar_item 无条件 insert |
| `ShowOnlyInFpvAndFree` | 同上 | 按 `item[0x38]` 条件 insert |
| `ToolbarItemKind` | `toolbar::resouces_and_components::toolbar_layout` | 判别子@+0；容器 Vec@+8（stride 0x40）；label String@+0x20；flag@+0x38；kind2 子判别@+1 |

依赖 core 已补类型优先用：spawn tuple 复用 `country_core::components::Ui2dExtent/
Ui2dImage/UiDrawOrder`（country-core components/trees.rs）；toolbar Cargo.toml 增加
`country-core.workspace = true` 依赖。

## 2. `place_childen_impl`（0x140fe12a0/0x140fe19a0）· 签名裁决 + 核心重建

**签名裁决（以反汇编为准）**。原 Rust 签名 `(parent_size: Vec2, children: &[ComputedLayout])`
（stride 0x10）与二进制不符，改判如下：

- `parent_size` 以**指针**传入（`r8`），16B 读面 `{main f32@+0, cross f32@+4,
  offset Vec2@+8}` → `ParentLayout`。
- `children` 元素 stride **0x18** = `{style: &LayoutStyle@+0, padding: &[LengthPercentage;2]
  @+8, prev_x f32@+0x10, prev_y f32@+0x14}` → `ChildLayoutInput`；ABI = `&[ChildLayoutInput]`
  （r9 ptr + [栈] len，`r13=[rbp+0x110]` 原字节核对）。
- 返回 `Vec<ComputedLayout>` 经 sret（`rcx`）。**分歧登记**：二进制对返回容器按
  **{cap, ptr, len}** 写出（`0x140fe184a movups [rax],xmm0`，xmm0={cap,ptr}；drop 函数
  `0x140fe18c0` 读 `[rbp-0x30]`=cap×16 为 dealloc size、`[rbp-0x28]`=ptr 为 dealloc 目标），
  与 std `Vec={ptr,cap,len}` 字段序不符——真实容器/LLVM 局部重排需消费侧
  mei_post_update（0x1406884c0，7958B 未实现）最终裁决。

**核心算法重建（逐语句，XY 主循环 0x140fe12a0..0x140fe18a6 + YX 0x140fe19a0..）**：
1. padding[0..3] LengthPercentage × 基准（主轴 padding 对 XY=[0]左/[2]右、YX=[1]上/[3]下）；
   gap percent → × (parent.main−主轴pads)。
2. `available = parent.main − (pad_start+pad_end) − (n−1)·gap`。
3. `sum = Σ max(prev[主轴], max(padding[主轴], size))`（NaN-safe，percent × available）；
   `sum_expand = Σ style.expand`。
4. `remaining = parent.main − pads − (n−1)·gap − sum`；`weight = clamp01(1−sum_expand)·remaining`。
5. justify 跳表（0x142bc223c XY 33 项 / 0x142bc2260 YX 24 项，两表物理重叠=LLVM 合并）：
   0→start、1/2→`start+=weight×0.5`、3→space-between（`gap+=weight/(n−1)`，n≤1 走 ×0.5）、
   4→`gap+=weight/(n+1)` 且 `start+=weight/(n+1)`、5→start、6→center；
   各 case 跳共享装配（对齐子表 0x142bc2250/0x142bc2274 逐子写）。
6. 装配：`main_size = (expand/max(sum_expand, ε))·parent.main + max(prev, size, pad)`；
   `pos = parent.offset + style.offset + {cursor, cross_pos}`。

**分歧（未实现）**：
- justify idx 9..32（XY）/9..23（YX）case 体位于共享尾区 0x140fe1be1..0x140fe2a34
  （LLVM 合并 YX case + RON `Justify` 反序列化路径：unknown_variant/字符串匹配，见
  ghidra-0x140fe12a0 反编译 case 0x1a/0x1d）。本实现 idx>8 按 start 取整，登记。
- SUM/装配循环字段读面不对称（XY: sum 读 size/prev_x、装配读 size/prev_y；YX: sum 读
  min/prev_y、装配读 size/prev_x）——装配侧 prev/字段配对 I。
- 输出写出 `{f32@+0, f32@+4, Vec2@+8}`（xmm6=主轴尺寸、xmm7=交叉、xmm9=位置偏移）
  与 `ComputedLayout{pos,size}` 字段序未完全对齐（经消费侧确认）。
- `LayoutStyle` 字段校正：gap/size/min 按 LengthPercentage、expand=f32、+0x3c offset
  Vec2、无 weight 字段（反汇编为准）；`default`/`horizontal` 全零字节两解释等价仍判等。

## 3. `add_toolbar_item`（0x141e4dc40）· 判等（控制流）+ 分歧（kind 载荷）

`toolbar_layout.rs`。**已判等**（decompile 0x141e4dc40 + kind 跳表 0x142f8f774）：
- 容器 kind 4/5：new_leaf+add_child（Result 判别子 4=Ok）→ 递归子项（stride 0x40，
  `byte[child+0x38] != 2` 界）→ 逐子 drop → 释放 Vec 缓冲。
- 叶子：new_leaf+add_child → 克隆 label → `Commands::spawn<8 组件>` → 无条件 insert
  `UiShowIfParentIsInFocus` → 条件（`item[0x38]`）insert `ShowOnlyInFpvAndFree` → kind 分派。
**分歧（未实现）**：kind 处理器命令记录载荷（0x60B 记录 {fn@0x140fdf9a0,...}、0x20B 记录
{fn@0x140fdf500,...}、[rax+0x18] vtable、kind2 三张 f32 朝向表 0x142f8f6d8/70c/740 +
0x140c904a0）类型 U——`kind_handler` 占位，不伪造记录编码。

## 4. `toolbar_alignment`（0x142344560）· 未实现（前置重建）

前置遍历（stride 0x28，`byte[item+0x24]!=4` 时 `item.align = 0x1020300 >> (alignment<<3)`
字节）→ 按 `byte[r13+0xf0]` 跳表 0x14304e6d8（20 项）分派。**分歧（阻塞）**：
`taffy_mut_style` 单态（0x142376050/0x142375ce0/0x142375b40，b7 已证
`taffy_mut_style<toolbar_alignment::closure_env$0>` 等）setter 目标语义、alignment 枚举
0..19→锚点名、ToolbarItem 完整布局（本批只落最小读面）U。

## 5. `toolbar_auto_hide`（0x142343a10）· 未实现

常量/前段已登记：`110.0`/`-1.0`/`10.0`/`0.3`/`-0.5`/`0.5`/`{80,40}`/`{40,80}`/`200.0`
（自动隐藏阈值/边距）。多 SystemParam 栈参密集。**阻塞**：Ghidra "bad instruction data"
截断区未取证；系统参数接线 U。

## 6. `ui_sounds`（0x14233a7f0）· 未实现

地址更正复核：总账 0x14233c450 实为 ui_select_animation；真 ui_sounds=0x14233a7f0。
switch ~18 case（0..0x14）+ 0x14233fc6a/0x14233fe2d 两未恢复跳表。case 体截断
（"Bad instruction data"）需 radare2 定点反汇编；PlayUiSound 载荷面未接线。

## 7. `ui_slider`（0x1423486d0）· 未实现

符号 `toolbar::photomode::ui_slider::ui_slider`（photomode 模块，非 ui_interaction）。
switch `byte[entry+0x10]` ~28 case（0..0x1b）+ MainCamera/BrushSizes 接线；截断 case 体
需 radare2 定点反汇编；widget kind 枚举 U。

## 8. 证据索引（`wave2b/toolbar/`）

- `jobs/ghidra-bytes.json` + `results/`：place_childen_impl init 原字节
  （`4c 89 e9` = `mov rcx,r13` 确认）、epilogue/drop 字节（`{cap,ptr}` 写出确认）。
- `jobs/ghidra-callers.json`：place_childen 分派、`__rust_alloc`、`grow_one<ComputedLayout>`
  （读 `*rcx`=len/cap、`rcx[1]`=ptr → 佐证 `{cap,ptr}` 布局）。
- `jobs/r2-place-tail.json` / `r2-place-tail2.json` + `results/`：跳表 0x142bc223c/
  0x142bc2260/0x142bc2250/0x142bc2274 原始字节 + 尾区反汇编（共享 0x140fe1be1..）。
- `jobs/r2-consts.json`：0x142a7baa0=`0x1e3ce508`（expand 除零保护）、0x142925870=0.5、
  0x142925904=1.0。
- 复用：re-analysis `evidence/ghidra-0x141e4dc40-*.c.txt`、
  `add_toolbar_item-*kind*.disasm.txt`×6、`140fe12a0-*-full.disasm.txt`、
  `142344560-*.disasm.txt`、`142343a10-*.disasm.txt`；wave2 RESTORE.md。

## 9. 未实现总账更新建议（给 coordinator）

- toolbar §4 六函数：`place_childen_impl` 转 **核心重建 + 尾区未实现**（签名裁决完成，
  尾区 justify>8/RON 路径 + 输出字段序 + `{cap,ptr,len}` 返回容器需 mei_post_update
  消费侧裁决）；`add_toolbar_item` 转 **控制流判等 + kind 载荷未实现**（需 0x60B/0x20B
  记录与 fn@0x140fdf9a0/0x140fdf500 语义）；`toolbar_alignment`/`toolbar_auto_hide` 仍
  core-task（ToolbarItem 完整布局 + taffy setter 语义）；`ui_sounds`/`ui_slider` 需
  radare2 定点反汇编截断 case + 音频/相机类型接线。
- **既有判等声明复核**：`LayoutStyle` 字段（gap/size/min=LengthPercentage、expand=f32、
  offset Vec2、无 weight）已按反汇编校正——建议 reviewer 复核 `default`/`horizontal`
  字节声明（全零两解释等价，仍成立）。
