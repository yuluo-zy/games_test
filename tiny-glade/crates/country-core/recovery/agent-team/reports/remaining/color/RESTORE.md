# system-color 剩余函数 — RESTORE（restorer-color-remaining）

- Agent：**restorer-color-remaining** — 2026-10-17（wave2/state 之后，剩余项收口）
- 范围：`crates/systems/color/src` 的剩余未实现函数（color_menu / set_roof_customization /
  color_button_animations / delete_color_icon_entities / ColorRadialMenu::create /
  ui_color_* / replay_color）+ B-ecstate post-fix（show_color_options case 0 副值）。
- 方法：**本轮 fresh 反编译**（Ghidra `--worker shared` 只读，2026-10-17 23:10），
  逐函数重建控制流并与 Rust body 逐语句/逐字段对照；还原不了的不构造近似体，
  标未实现并写解锁条件。证据：`reports/remaining/color/evidence/` +
  `work/restorer-color-remaining/result-read.json`（sha256 `d4633e86…a561`）。
- 身份：`tiny-glade.exe` sha256 `cf048e27…6323`（B patched）。

## 三态回执

- **完成（判等/重建）1**：`replay_color`（0x141c7f270）。
- **完成（B-ecstate FIX 闭环）1**：`show_color_options` case 0 plaster 副值 v24（并移除
  case2..0x26 的 roof 色错误回落）。
- **登记（未实现 + 解锁条件）9**：见下表；其中 `set_roof_customization` 外环保持
  已重建，剩余密集表面登记。
- **阻塞**：无（未遇到 402/余额问题）。

## 逐函数判定

| 函数（地址） | 本轮判定 | 说明 / 分歧点 / 解锁条件 |
|---|---|---|
| `replay_color`（0x141c7f270） | **判等/重建** | 逐语句重写：桶 stride 0x30、`tag==6` 过滤、`sub=clamp(edit[8]-3,0..13)`、`elapsed==0` 派发门、13 个颜色写者（Wall/Gate/Roof/FlatRoof/Decorator/Window/Halftimber/Flag/Brightness/Chimney/Ladder/Railing/Magic）、`undo?from:to` 方向、`custom/play_sound=1`、`required=max(duration/speed,0.1)`、严格 `<` 移除判据、移除反序 `Vec::remove`。实现见 `src/replay_color.rs`。 |
| `color_menu`（0x142291230） | **未实现** | 主循环 = 逐图标 0xa0 状态机；`CustomizationIcon` 仍为 `_gap: u64`（core 未落 0xa0 字段）。解锁：core 落 `CustomizationIcon` 0xa0 布局（+0x30 menu state、+0x18 图标列表、+0x50 中心、+0x90 半径等）+ 墙/屋顶图标密集表访问面 + `Events<(&str,SettingsMenuState)>`（`SettingsMenuState` 已在 `resources/gui.rs`）+ History Color 字节记录。结构/case 表登记保持不变。 |
| `set_roof_customization`（0x142299dd0） | **未实现**（外环已重建） | 本轮 fresh 复核：外环 `SetRoofColorIdCmd` 两处写——① `roof_elem +0x50 + idx*0x58 = event.color`；② **色表 `*(color_table + idx*4) = Res.in_R9+0x14`**（FIX-6 第二写）。②与全部密集定位依赖 `InnerWalls +0x1a8`（chunk stride 0x48、`+0x110` slot、`~slot*0x30` 叶）与 `roof stride 0x58`；core `PublicWalls` 仍 HashMap 抽象。解锁：`reports/unlock/publicwalls-tables` 合同落库后按 fresh 重建样式比较环（+0x3d/+0x40/+0x48）与第二写。 |
| `color_button_animations`（0x1422979b0） | **未实现** | 数学已复核：`dt/-0.15`（删除态）或 `dt*4.0`，进度 `+0x18+idx*0x24`，clamp 1.0，`EasingFunction::apply`，删除态二次缓动 `(1-e)*0.8+e`，副进度 `+0x1c` ±0.15 clamp[0,1] 后 `e*0.2+1.0`。分歧点：`CustomizationIcon`/`Ui2dExtent` typed-gap + 墙/屋顶密集表（+0x100/+0x1a8）。 |
| `delete_color_icon_entities`（0x142297ff0） | **未实现** | fresh 复核：`=*(char*)(walls+0x108)` 分墙/屋顶两条密集表路径，`progress(+0x18+idx*0x24) <= 0.0` 才 `despawn`。当前保持空循环体（不做无条件销毁）。解锁同 `color_button_animations`。 |
| `ColorRadialMenu::create`（0x1425f27c0） | **未实现** | 已解：非分隔符计数、`step=(start+span)*(count-1)/denom`、`AngleRange::from_min_and_len`、`cosf/sinf`、半径 `size*0.75/0.25`、角步 π/4、按 kind 3..14 插入组件。**解锁缺口**：core 缺 10 个组件类型（`ColorWheelSeparator`/`FlagConnectionIcon`/`FreehandHeightTypeToggleIcon`/`FlatRoofRimTypeToggle`/`ColorMenuBrightnessIcon`/`ChimneySmokeToggle`/`FlatRoofStyleIcon`/`PlasterPeelIcon`/`StairRailingIcon`/`MagicColorIcon`），且 `Ui2dExtent`/`Ui2dImage`/`CustomizationIcon` 仍 `_gap`；AngleRange 输入参数（start/span/denom）在反编译内联丢失。 |
| `ui_color_wall_and_roof`（0x141c9ab70） | **未实现** | fresh 复核：`AppMode==0xc` 门 → `WidgetVolumeShape::clearing_canvas(3)`/`WidgetParams::new/sounds/occlude_by_world/signifier<closure_env_0>` → `UiState::add("color_canvas…")` → `clicked` → `RaycastWorld::raycast(mask 0xcb)` → 命中类型 0/2/6/7/8 分派 → 密集墙表定位 → `format!("…WallId")` 控件 → `clicked` 写 stride 0x20 命令。分歧点：`WidgetParams::signifier<closure>` 未对齐、`RaycastWorldResult` 无命中类型面（+0x1d8）、closure_env_0/2 未恢复。 |
| `color_cursor_icon`（0x141c9b510） | **未实现** | fresh 复核：`SignifierEntityBuilder::{new,animation_duration,build}` 已在 core（coregap-1），`in_R8B==0` 时 `insert(UiInactive)`；串 `"color_icon/color_cursor"`；`insert<(Ui2dExtent,Ui2dImage)>` + `insert(Ui2DFollowMouse)`。**唯一剩余缺口**：core 无 `UiInactive`/`Ui2DFollowMouse` Rust 类型（仅 registry 名），`Ui2dExtent`/`Ui2dImage` 仍 `_gap`。 |
| `ui_color_gates`（0x1422afe70） | **未实现** | fresh 复核：`AppMode==0xc` → 占用位集取反/遍历置位（`*in_R9` 位数组）→ `AssetMeshLibrary::get`/`Mesh::positions` → `Triangle` 相交 fold → `WidgetParams::new` + `UiState::add<String>` + `SignifierFactory::new`/`UiSignifierStream::write` → `clicked` 写事件。分歧点：位集/墙目录密集表、mesh/triangle 数据面、signifier 流未对齐。 |
| `ui_color_windows_and_flags`（0x1422b0e00） | **未实现** | fresh 复核：`legacy_raycast_w_params`/`StairsState::graph` 遍历/outline 面；菜单类别码 {3,6,9,10,11/12} 已解。分歧点同上（raycast 结果/楼梯图/密集装饰表 core 未暴露）。 |

## show_color_options — B-ecstate post-fix FIX 闭环

依据：`reports/reviews/B-ecstate/REVIEW.md` §8（fresh radare 反汇编 + 跳表/字符串实读；
Ghidra/ReVa 超时，未复用旧 `full.c`）。

- **已修（本轮）**：case 0 主 13 个 kind-3 swatch 中，8 个 plaster 条目的副值
  `+0x24` 原实现恒 0。fresh 反汇编实读（0x14229d03e 起，步长 0x28）：
  前 6 项（5 bricks + 首个 plaster v1c=4）为 0；后 7 项 plaster
  （v1c=0xc,5,10,8,9,6,7）依次 **7,1,6,4,5,2,3**。
  即 plaster 序列 **v24 = [0,7,1,6,4,5,2,3]**。已写入
  `push_case0_main_swatches`（`IconInfo { kind:3, v1c, v24 }`）。
- **同时修正**：`options_for_kind` 的 `_ =>` 原对 **所有** case 2..0x26 回落 roof 色
  （近似体）。现改为 `1 => roof 色`，`_ => Vec::new()` 并登记 GAP——不伪造未实现 case。
- **case 0/1 主载荷**（review §8.2/8.3）：13 swatch name/kind/v1c、plaster_peel
  low/medium/high、halftimber/skirt/no_skirt、is_flat 门、11 roof swatch 均
  fresh 复核判等，本轮未改。
- **仍登记 GAP**（review §8.4）：
  1. case 2..0x26 逐 case 精确表（Rust 现为空 Vec + GAP，不近似）。
  2. demo 分支（0x14229b744 起）——当前全局 `_DEV_OVERRIDE_IS_DEMO_MODE=0`，未实现。
  3. 门源：plaster（`get_instruction` 索引）、halftimber 计数（+600）、
     `MainCamera::eye_position` SIMD 偏移 / radius。
  4. 非平顶 else 块（5 图标）与尾部事件写者族。
  5. 跳表实读 39 项（review 口径）与 wave2 的 40 项登记差异——按实际 39 项为准，
     不影响本函数已判等 case。

## replay_color 判等细节（实现对照）

- 原生命令记录 stride **0x30**（`ptr + len*0x30`），本实现匹配 `ReplayCommand::Color`。
- `sub` 越界夹取：原生 `if 0xd < sub { sub = 3 }`（未知 target 当 FlatRoof 派发）；
  Rust `ColorEdit` 为闭合枚举，该分支不可表示 → 登记 U，不构造。
- `StairSupport(16)` → `sub 0xd`，switch **无 case 0xd** → 不派发；Rust 侧对应空臂。
- FlatRoof 写 `SetFlatRoofStyleCmd(FlatRoofStyleRequest{wall_id, style: undo?from:to, observed_play_sound:1})`
  —— 与 core 已落 `FlatRoofStyleRequest` 的 event 布局（id@8、style@16、sound@28）逐字段一致。
- Wall/Gate/Halftimber 的 event +8 为整 8 字节 wall id：core 命令字段拆为
  `id: u32 + _gap4: u32`，本实现按低/高 dword 写入（逐字节等价）。
- Brightness 写 f32 位模式（u32 存储）→ `value: from.to_bits()/to.to_bits()`。
- 时序：`flags.bit0==0` 立即移除且不推进；否则 `required=duration/speed`，
  `if required <= 0.1 { required=0.1 }`，`required < elapsed` 记移除，随后**无条件**
  `elapsed += delta`；移除下标 `len>1` 时原地反转后逐个 `Vec::remove`。
- 尾部释放跳表（WallReplayCmd/RasterEdit/CrashMiscInfo/PhysicalSize…）为共享 drop
  glue 的 PDB 名称恢复结果，Rust `Drop` 语义等价，不手写。

## 顺带编译修复（本 crate，非他模块）

core 类型演进导致本 crate 3 处编译错误；均按 fresh 反编译语义机械修正：

1. `replay_color.rs`：`FlatRoofStyleRequest.wall_id` 使用
   `recovered_building::building_wall_state::WallId`（与 `resources::walls::WallId`
   为两个同名 u64 newtype，wire 相同）→ 显式转换并登记。
2. `set_window_light.rs`：`WindowExtra` 已改为 `pub struct WindowExtra(pub u32)`；
   原生整 4 字节写事件 +0xc 位模式 → `window.0 = cmd.value`。
3. `set_decorator_color_id.rs`（L2 灯笼）：原生整 4 字节写事件 +0xc 位模式到
   `LanternExtra(f32)` → `lantern.0 = f32::from_bits(cmd.value)`（逐字节一致）。

## 编译证据

- `cargo check -p system-color`：**Finished dev profile，0 error；本 crate 0 warning**
  （country-core 14 条 warning 属该 crate 既有，非本轮引入）。
- 依赖窗口：本轮检查时 country-core 已由他组修复至可编译；未代改他组文件。

## Fresh 证据索引（`reports/remaining/color/evidence/`，sha256）

| 文件 | sha256 |
|---|---|
| `0x141c7f270_replay_color.c` | `699e5928…e5cd` |
| `0x142291230_color_menu.c` | 未取（shared DB 该函数 codelen=0；以 wave2 写 worker 全量反编译 + review 为准） |
| `0x142299dd0_set_roof_customization.c` | `c75832de…02dc` |
| `0x1422979b0_color_button_animations.c` | `dbdee765…4026` |
| `0x142297ff0_delete_color_icon_entities.c` | `0c154706…7d1a` |
| `0x1425f27c0_create.c` | `82b36359…4085` |
| `0x141c9ab70_ui_color_wall_and_roof.c` | `e3edfcbd…336a` |
| `0x141c9b510_color_cursor_icon.c` | `af3b1c7f…e8a0` |
| `0x1422afe70_ui_color_gates.c` | `18b56df8…9930` |
| `0x1422b0e00_ui_color_windows_and_flags.c` | `53024116…c02d` |
