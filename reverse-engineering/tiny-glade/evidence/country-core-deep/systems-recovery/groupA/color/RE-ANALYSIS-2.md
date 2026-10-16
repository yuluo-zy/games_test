# groupA RE-ANALYSIS-2 — color_menu 逐分支契约 + show_color_options 机器码取证（task ①）

dbid: tg_xa1 · 2026-10-16
证据: `color/decomp/decomp-0x142291230.txt`（137KB 逐段分析）+ rne-disasm 机器码（show_color_options 0x14229a7f0..0x1422a0e18，632 call 全量）

## 1. color_menu（0x142291230）分支契约（O 结构级）

### 输入面（15+ 参数）
- param_1 = ResMut<通用 entity storage 容器>：A@[0] = {位集@+0x28/+0x38, 范围@+0x110..0x128}，B@[1] = {id→索引表@+8/+0x10(0x14B), 0xa0B 条目@+0x100/+0x108, 0x48B 记录@+0x1a8}（**与 tree-tool 同形**，墙/装饰共用，见 tree-tool RE-ANALYSIS-2 §容器）。
- param_6 = 动作容器（+0x98/+0x9c = 确认动作对；+0xa0/+0xa4、+0xa8/+0xac = 导航动作对；just_pressed 0x14093fa90）。
- 输出 ResMut Vec（0x10B/0x20B/0x30B 三类）+ tick 回写（`*puStack_XXX = uStack_YYY` 模式）。

### 0xa0B 图标条目字段（O）
flag@+0x00（bit0=活跃，`*(int*)entry == 1` 判选中态）、子表@+0x18（float 指针列表）、kind@+0x30（判别字节）、值 u32@+0x34、二级哈希@+0x80/+0x88、标志@+0x86/+0x87。

### 分支分派（O）
```
for 每个 0xa0B 条目（经 id→索引表 0x14B + 位集 + 0x48B 记录定位）:
  if kind@0x30 == 9:  push {u64, u32@+0x34} → Vec(lStack_650)   // 选择目标记录 A
  if kind@0x30 == 4:  push {u64, u32@+0x34} → Vec(pfStack_5b8)  // 选择目标记录 B
  if kind@0x30 == 10: 其它分支（编辑命令 kind=10 块）
  交互判定：clicked(0x14092e5f0) || just_pressed(动作+0x98/+0x9c) 且 entry 活跃
    → push 0x20B 选择记录 {u32×4, f32 亮度(pfStack_800), u64} → Vec(lStack_628)
    → 构建 0x30B 编辑命令 {u32 1, u32 0, u32 1, u32 0, f32 值, u8 sel@+0x14, u8 sel^1@+0x15, u8 kind@+0x18} 
    → History::add(0x140a84ec0, edit 类型 0x9) ×13 处
```
- 13 处 History::add 全部 edit 类型 **0x9**（`uStack_2d8 = 0x9`），命令均 0x30B 分配（0x140613c10(0x30,8)）。
- 动作门禁：`just_pressed(+0x98/+0x9c)` 为主选择动作；`+0xa0/+0xa4`、`+0xa8/+0xac` 为导航/副动作（bVar22|bVar23 块，行 665）。
- 亮度：`ColorMenuBrightnessValue::to_f32`（0x140b06770）×3 + default_f32 ×?（0.25/1.0/4.0 表，audio-misc 章节 O）；`AngleRange::clamp`（0x1425f24d0）+ polar/cartesian（0x140c90e20/0x140c90e70）—— 图标角度布局；`FreehandHeightType::toggle`（0x1408c5660）—— 自由高度切换项。
- 目标判定数据面：PublicWalls::get（0x140a91ef0）×4、roof_entity（0x140a922b0）×1、History::try_start_input_recording（0x140a86510）×1。

## 2. show_color_options（0x14229a7f0）机器码取证（O 受限 → 升级为调用序 O）

函数边界：0x14229a7f0 .. 0x1422a0e18（首 ret）。**632 条 call 全量提取**，去重统计：

| 目标 | 次数 | 身份 |
|---|---|---|
| 0x140613c10 / 0x140613c20 | 165 / 161 | alloc / dealloc |
| 0x1428d8fe3 / 0x1428d9000 | 146 / 16 | alloc 错误 / handle_alloc_error |
| 0x140a1c930 | 13 | DecoratorStorage::lookup_decorator_info |
| 0x14014a860 / 0x14087b030 | 11 / 6 | Vec<0x28B> drop 双单态 |
| 0x1428d9310 / 0x1428d9518 | 10 / 9 | unwrap_failed / panic_bounds_check |
| 0x1400885d0 | 7 | id→索引（墙/装饰） |
| 0x140a922b0 / 0x140a91ef0 | 7 / 6 | PublicWalls::roof_entity / get |
| 0x1429107f0 / 0x142923c00 | 7 / 6 | Vec reserve / memcpy |
| 0x140911dd0 | 5 | MainCamera::eye_position（菜单锚点） |
| **0x1425f27c0** | **5** | **ColorRadialMenu::create（5 个菜单）** |
| 0x140562c30 / 0x140562ab0 | 5 / 4 | Vec grow |
| **0x1422acce0** | **4** | **closure$0（图标串构造，已反编译）** |
| 0x140839490 | 4 | DecoratorExtraInfo::try_as_stair |
| 0x140b06780 / 0x140b26a70 / 0x140b26970 | 4 / 3 / 3 | brightness default / DerivedDecoratorInfo 双单态 |
| 0x1408392d0 / 0x140839340 / 0x1408393b0 / 0x140839420 / 0x1408394b0 | 2 / 2 / 1 / 1 / 1 | as_chimney_mut / as_flag / as_window_mut / as_lantern_mut / as_stair（**7 种装饰目标**） |
| 0x1408c5670 / 0x1408e3380 / 0x1408fe8c0 | 2 / 2 / 2 | toggle / Roof::is_flat / StairsState::graph |
| 0x140833230 | 1 | enum→String（freehand 类型名） |
| 0x1425f2680 / 0x1425f26f0 | 1 / 1 | IconInfo::new_separator / new_flat_roof_rim_ty |
| 0x1428d9430 | 2 | panic_str |
| 0x141c5a140 / 0x141c5a430 / 0x140ec60f0 | 1×3 | UI 辅助（U 身份） |

### 5 个菜单的判别链（调用序 O，机器码上下文）

| create 调用点 | 前置判别 | 目标 |
|---|---|---|
| 0x14229f2c9 | 装饰 kind 链（`cmp al,{2,6,9,0xa,0xb,0xc}` → as_stair/as_flag/as_lantern_mut/as_chimney_mut 各单态；Roof::is_flat 0x14229f4c0） | **墙/装饰混合菜单**（屋顶扁平分派） |
| 0x14229f644 | kind==0xc → try_as_stair → **StairsState::graph**（0x14229f70b） | **楼梯菜单** |
| 0x14229fb88 | as_window_mut（0x14229fab1）+ brightness default（0x14229fabf）；rsp+0x20 = 0x3ec51eb7(0.3856f32) | **窗户菜单** |
| 0x1422a000b | kind 0xb 标记（[rbp+0xb0]=0xb）+ brightness（0x14229ff26） | **楼梯变体 2 菜单**（0xb = 连通楼梯） |
| 0x1422a0ae1 | closure$0（`cmp bl,3`）+ kind 0xc 标记（[rbp+0x158]=0xc）+ brightness（0x1422a0a09） | **半木/屋顶样式菜单**（0xc） |
| — 0x1422a0bea/0x1422a0bfd（紧随第 5 菜单） | new_separator + new_flat_roof_rim_ty(freehand_ty) | freehand 类型分段 |

装饰 kind 字节取值（entry+0 判别，经 0x140b26970/0x140b26a70 = DerivedDecoratorInfo 双单态解析）：{2=墙自由手, 6=旗, 9=灯, 0xa=烟囱, 0xb=楼梯, 0xc=连通楼梯}（I：kind↔目标映射按访问器证据，6→as_flag 与 0xa→as_chimney_mut 两处上下文均实证；2→roof_entity 分支）。

### 菜单参数（create 调用上下文，O）
每 create 前：eye_position（锚点）+ 4 f32 栈参（rsp+0x20/0x28/0x30/0x38 = {0 或 0x3ec51eb7, 亮度, …}）+ xmm1-3（中心/半径）—— ColorRadialMenu::create(center, radius, icon_size, 亮度…)。

## 3. 修正/升级

- show_color_options 由"O 受限"升级为 **O（调用序级）**：完整 call 图 + 5 菜单判别链 + 7 目标访问器 + 图标串全表（RE-ANALYSIS-1 §2）。唯一缺口 = 逐图标内部布局（closure$0 已反编译，40+ 图标串已定位）。
- color_menu 分支契约补全（kind 9/4/10 分派 + 13×edit type 0x9 + 动作对 + 0x30B 命令布局）。

## U 项（保留）

- 0x141c5a140/0x141c5a430/0x140ec60f0 身份。
- kind 字节↔目标完整映射的逐条双证（当前 6/0xa 已双上下文，2/9/0xb/0xc 单上下文）。
- 菜单 4 f32 栈参的逐位语义。
