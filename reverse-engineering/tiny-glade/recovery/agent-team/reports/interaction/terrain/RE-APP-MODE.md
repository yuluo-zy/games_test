# RE-APP-MODE：AppMode 判别映射裁决（terrain 域卡点①闭合）

> 会话：interaction re-analyst（2026-10-15）。目的：裁决「ui-input §3.3 认为 10+5k=Tool(视觉序
> Buildings…/Terrain)，terrain 域实测 mode==10 是工具门」的冲突，给出 AppMode 变体表、mode==10
> 对应工具、sub!=1 语义、0x421 掩码的 {0,5,10} 关系。
> 依据（全部 B EXE 机器码，本轮定点反编译/手解落盘在
> `evidence/country-core-deep/interaction-group/decomp/` + `results/out-appmode*.json`）：
> - `mode_manager::impl$0::default` @0x140a86b30（10B，原字节手解）
> - `AppMode::is_tool_selected` @0x140a86b40（24B，原字节手解）
> - `check_if_can_switch<closure_env$0/3/4/5/6>` @0x140afa8a0/0x140afa2c0/0x140afa770/
>   0x140afab70/0x140afa3f0（本轮反编译）
> - `send_mode_just_changed_event` @0x140a875e0（181B，PartialEq::ne<AppMode> 两 qword 比较）
> - mode_manager 入口 @0x140a86b60 + 跳表 0x142ad890c
> - AppMode serde（impl$0 族）serialize @0x1400368a0 / deserialize visit_str @0x140afbe10
> 记号：O=机器码证实；I=推断；U=未恢复。**机器码为准，行为裁决。**

## 1. AppMode 结构（O）

- **AppMode = 16B（2 qword）enum**：`{disc u32@0, payload u64@4, extra u32@0xc}`。
  证据：`send_mode_just_changed_event` 用 `PartialEq::ne<AppMode>` 比较两 qword（16B）；
  `check_if_can_switch<env_5>` 写 `{*out=iVar1(disc), out[1]=uVar4(payload), out[3]=iVar2(extra)}`。
- **AppMode::default = disc 0**（@0x140a86b30：`mov dword [rcx], 0; ret`）。
- **AppMode::is_tool_selected @0x140a86b40**（24B，原字节手解）：
  ```
  mov ecx,[rcx]        ; disc
  lea eax,[rcx-6]      ; disc-6
  cmp eax,0xf
  setb dl              ; dl = (disc-6 < 15) = disc∈6..20
  add cl,-5            ; cl = disc-5
  mov eax,0x3cfe
  shr eax,cl           ; 0x3cfe >> (disc-5)
  and al,dl
  ret
  ```
  → **`is_tool_selected = disc ∈ {6,7,8,9,10,11,12,15,16,17,18}`**（0x3cfe=0b0011110011111110，
  位 1..7 与 10..13 置位 ⇒ disc−5∈{1..7,10..13}；dl 门 disc∈6..20；**13=Replay/14 排除**）。
  ⇒ 工具类 AppMode 判别值落在 6..12 ∪ 15..18；disc 10（terrain 门）在工具区 ✓。

## 2. 已定判别值 → 语义（O）

| 判别 | 语义 | 证据 |
|---|---|---|
| 0 | All（默认；AppMode::default 写 0） | default@0x140a86b30 + serde visit_str@0x140afbe10 `"All"→0` |
| 1 | Wall | visit_str `"Wall"→1` |
| 2 | Path | visit_str `"Path"→2` |
| 3 | Bulldoze | visit_str `"Bulldoze"→3` |
| 4 | Water | visit_str `"Water"→4` |
| 5 | Garden | visit_str `"Garden"→5` |
| 6 | Tree（工具区起点） | visit_str `"Tree"→6` + is_tool_selected 含 6 |
| 0xd (13) | Replay（**非工具**，is_tool_selected 位 8 清零） | check_if_can_switch<env_0> 写 `*pending = 0xd`；wp14 音频 `==13` 音量 1.0；wp17 REPLAY_TAG=0xd |
| 2 | Timelapse | ui-input DIGEST §2.2（`mode@+0x28 != 2` 早退） |
| 7..12、15..18 | 其余工具类（含 10） | is_tool_selected 工具区 {6..12,15..18} |

> 注：visit_str@0x140afbe10 的 7 个 unit 名（All/Wall/Path/Bulldoze/Water/Garden/Tree）机器码
> 硬；**与 0x421={0,5,10} 完全自洽**：0=All（默认/总览）、5=Garden（园艺/地形邻近）、10=Terrain 工具。

## 3. mode==10 裁决（★）

**裁决：AppMode disc 10 = 地形绘制/编辑工具模式（Terrain tool）。** 依据（行为仲裁）：
1. `ui_draw_terrain` @0x140a9fdc0 门 `*(i32*)*param_1 == 10`（Res<AppMode> 首参，wp8 合同证实）。
2. `ui_edit_terrain` @0x140aef1f0 门 `== 10`（同）。
3. `terrain_hover_idx` @0x140aff720 门 `mode<0xb && (0x421>>mode)&1`，**0x421 = bits{0,5,10}**
   （0x1|0x20|0x400——本轮算术复核：0x400=bit10，非 bit9）。
4. `is_tool_selected`（工具区 6..18）含 10。
5. 三个独立系统同一门值 → 无歧义。

**旧结论 X → 新证据 Y**：
- ui-input §3.3「10+5k=Tool(类别 k，视觉序 Buildings/Walls/…/Terrain)，故 mode 10=Buildings、
  mode 55=Terrain」——**与机器码行为冲突，应作废/改判**。ui-input DIGEST §2.2 自己的
  env↔类别映射（env_10=cat6 Buildings、env_3=cat7 Walls、env_13=cat8 Roofs、env_11=cat9 Paths、
  env_14=cat10 Water；env_5=动态类别）与「10=Buildings」并不自洽。裁决：**mode 10 = Terrain 工具
  （地形绘制模式）**；模式→工具类别 的精确线性映射（mode=5+5·cat 候选，cat1=10/cat6=35 Buildings/
  cat10=55 Water）需工具栏类别 id 编号表终裁，但 10 的语义已定。

## 4. sub!=1 语义（translate_input 门）

`translate_input` @0x1417cd0a0 analog 相机翻译门：
```
lVar28 = *(i64*)*param_5;                  # AppMode qword0 = {disc u32, payload低 u32}
if (lVar28 == 2) || ((int)lVar28 == 10 && ((char*)*param_5)[1] != 1):
    运行相机平移翻译
```
- `lVar28 == 2` = Timelapse（payload 0）。
- `(int)lVar28 == 10 && byte1 != 1`：byte1 = AppMode 数据 +1 = **disc u32 的 bits 8..15**。
  纯 disc 10（0x0000000A）byte1=0 ⇒ 恒真 ⇒ **相机平移实际在 {2, 10} 两模式运行**。
- 若 AppMode disc 编码含 `{类别 u8@0, 子模式 u8@1}` 形态，则子模式 1（disc=0x010A）会跳过相机平移
  ——子模式 1 候选语义（I）：地形聚焦/放置锁定态（相机接管），未证实 U。

## 5. 变体名（部分 U）

AppMode serde（impl$0 族）的融合串池 @0x14292a584..0x14292a63c 含候选名：
`None Wall Roof DecoClutter decorator_id StairPoint Terrain All Path Bulldoze Water Garden Tree
Editing Cobblestone Eraser Decorator Color Replay LoadGame Fences Photomode CloneClutter Freeform
Circle Rectangle`。**精确 disc(7..18)→名 映射未闭合（U）**：serialize@0x1400368a0 的 jumptable
目标（0x1400368a0..0x1400370b4）因融合池共享、disc→表索引（`lVar1=disc−5`）与多类型 impl$0
复用而无法逐一钉死；disc-10 目标的串引用（"Fences"/"Decorator"，len 6/9）与行为裁决（10=Terrain）
冲突——判该 serialize 目标族或属其他类型/或跳表索引非 disc−5，**不采信为 10 的名字**。
行为裁决（10=Terrain）为权威。

## 6. 0x421 = {0,5,10} 的意义

0（All）、5（Garden）、10（Terrain 工具）三模式都允许地形笔画悬停——三者皆「地面编辑」语境：
- All：总览模式可编辑已有笔画；
- Garden：园艺/种植物模式（地形邻近，可调整地面）；
- Terrain 工具：地形笔刷主模式。
**旧结论 X → 新证据 Y**：本组 terrain/RE-ANALYSIS 早先误记「{0,5,9}」→ 机器码 0x400=bit10 ⇒ **{0,5,10}**；
wp8 合同 `bits{0,5,10}` 亦证实（已同步修正）。

## 7. 对 restorer / 集成 owner 的交付

- `AppMode` 恢复建议：16B 枚举 `{disc u32, payload u64, extra u32}`，disc 0=All（默认）、1=Wall、
  2=Path、3=Bulldoze、4=Water、5=Garden、6=Tree、0xd=Replay、2=Timelapse、**10=Terrain 工具**；
  `is_tool_selected = disc ∈ {6..12,15..18}`。
- 修正项：reconstruction `mode_manager.rs` 的 `AppMode::{None,Timelapse,Wall,Focus,Tool}` 与 serde
  impl$0 名不符，需按本节判别空间重建；`is_tool_selected` 现返回 false（折叠 default）→ 应按
  `disc∈6..18` 实现。
- 交叉验证建议：toolbar 类别 id 编号表（cat1..10 → 模式 = 5+5·cat?）由 toolbar 域补证，闭合
  §3 的线性映射。

## 8. 证据落盘

- `evidence/country-core-deep/interaction-group/decomp/decomp-0x140a86b30.txt`（default 10B）
- `decomp-0x140a86b60.txt`（mode_manager 入口 405B）
- `decomp-0x140afab70.txt` / `decomp-0x140afa3f0.txt` / `decomp-0x140afa8a0.txt` / 
  `decomp-0x140afa2c0.txt` / `decomp-0x140afa770.txt`（env_5/6/0/3/4）
- `decomp-0x1400368a0.txt` / `decomp-0x140afbe10.txt`（AppMode serde 族）
- `results/out-appmode.json` / `out-appmode2.json` / `out-appmode4.json`
- is_tool_selected 24B 原字节：`8b098d41fa83f80f0f92c280c1fbb8fe3c0000d3e820d0c3`（RNE oracle）
