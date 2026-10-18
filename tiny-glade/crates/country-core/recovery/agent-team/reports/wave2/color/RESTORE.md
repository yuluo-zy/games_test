# system-color wave2 — color_menu / show_color_options 全量反编译还原

- Agent：**restorer-wave2-color** — 2026-10-17（reviewer wave2 复核后已修正 case 0/1）
- 范围：`crates/systems/color/src/color_menu.rs` 与 `src/show_color_menu.rs`
  （B `color_menu` 0x142291230、`show_color_options` 0x14229a7f0）
- 依据：`reports/re-analysis/hardpoints-ecstate/RE-ANALYSIS.md` §2.1/§2.2 + evidence
  `decomp/color_menu_142291230_full.c`（136KB）、`show_color_options_14229a7f0_full.c`
  （267KB），均为 **ReVa 专属写 worker fresh clone + 600s 定点**产出；shared Ghidra
  DB 的 codelen=0 为 overlay 损坏，**实现以写 worker clone 反编译为基**，不采信
  shared 空反编译。reviewer `revieww2color` 独立 fresh clone 全量复核，证据 sha256
  与本文档引用逐字节一致（`show_color_options_14229a7f0_full.c` b4453c39…、
  `color_menu_142291230_full.c` 6d3e9ab7…）。

## 判等 / 重建 / 未实现 总表

| 函数（地址） | 分类 | 说明 |
|---|---|---|
| `show_color_options`（0x14229a7f0） | **重建（结构面 + case 0/1 图标载荷面）+ 未实现（其余 case/事件写面）** | 全量反编译结构恢复并实现：事件遍历（`ShowColorMenuCmd` EventInstance stride 0x20）→ kind 字节 0..0x26 分派（跳表 0x14303ca70 相对偏移，40 项已核对）→ `Vec<IconInfo>` 选项 → `ColorRadialMenu::create` + 菜单实体生成。**case 0**：非 demo 主路径 = 主 13 个 kind-3 色 swatch（bricks_beige/yellow/light/grey/dark ×5 + plaster_colors ×8）+ plaster 门追加 separator + plaster_peel_low/medium/high（kind 0xc）+ halftimber/skirt 门（separator + halftimber/skirt/no_skirt，kind 4）+ `Roof::is_flat` 门 separator + flat_roof_rim_ty（reviewer 复核修正）。**case 1**：11 个 kind-3 roof swatch（roof_ochre/red/purple/blue/light_blue/turquoise/green_light/green/dark_wood/dark/grey）。case 2..0x26、平顶 else 块、demo 分支、事件写面 GAP。 |
| `color_menu`（0x142291230） | **未实现（登记准确，reviewer 复核通过）** | 全量反编译结构/字段布局/主 switch 表已解出并完整登记（见 `color_menu.rs` 模块头）；per-icon 0xa0 状态机依赖 core `CustomizationIcon` 布局 + `Events<(&str,SettingsMenuState)>` + History Color 字节记录 + ~12 参数系统 ABI，均为 typed-gap → 逐语句判等编译阻断。 |

## show_color_options（0x14229a7f0）重建内容

已实现进 `show_color_menu.rs` 的函数体：
1. **事件遍历**：`for cmd in cmds.read()`（EventInstance stride 0x20，kind 字节 `(byte)*puVar19`、payload `*(*pauVar43+8)`）。
2. **kind 分派**：`options_for_kind(kind, …)` 按 kind 构建 `Vec<IconInfo>`。
3. **case 0（plaster/wall 颜色，reviewer 复核修正）**：
   - 非 demo 主路径首图标 = `color_icon/bricks_beige`（kind 3，v1c 0）——**非分隔符**（分隔符仅 plaster/平顶条件块内）。
   - 主 13 个 kind-3 色 swatch：bricks_beige(0)/yellow(1)/light(3)/grey(0xb)/dark(2) + plaster_colors(4/0xc/5/10/8/9/6/7)。
   - plaster 门 `(iVar9-4U<7)||(iVar9==0xc)`（行 689）→ 追加 separator + plaster_peel_low/medium/high（kind 0xc，v1c=0/0x3f000000/0x3f800000）。
   - halftimber/skirt 门 `*(puStack_250+600)<2`（行 793）→ 追加 separator + halftimber(kind 4,b19 2) + skirt(kind 4,b19 1) + no_skirt(kind 4,b19 0)。
   - `Roof::is_flat()` 门（行 884-940）→ 追加 separator + `new_flat_roof_rim_ty`。
4. **case 1（roof 色）**：11 个 kind-3 roof swatch（roof_ochre/red/purple/blue/light_blue/turquoise/green_light/green/dark_wood/dark/grey，v1c=8/0/10/2/9/3/6/4/1/5/7）。
5. **`ColorRadialMenu::create(center, radius, icons)`**：center 源自 `MainCamera::eye_position()`（精确 SIMD 偏移 GAP，以 `.truncate()` 近似）+ `Commands::spawn((menu, CustomizationIcon::default()))`。

**GAP 面**（未实现，逐条登记）：
- case 2..0x26 逐图标串/值精确表（装饰器分类 `DecoratorStorage::lookup_decorator_info` + `DecoratorExtraInfo::try_as_*`、`StairsState::get_all_connected_nodes`、`ColorMenuBrightnessValue` 载荷——core 侧为高层抽象，未对齐）。
- 平顶 else 块（行 941-1088：separator + skirt/no_skirt + separator + kind-7 30 字符，`ClutterVersion::is_initial` 门）与 demo 分支（行 1094+）。
- plaster 门 `iVar9` 索引源（`Res<WallColorIds>` + `get_instruction()`）、halftimber 门 `puStack_250` 源——core typed-gap。
- `MainCamera::eye_position` 偏移/radius 的 SIMD 逐分量、demo 全局静态字节。
- 尾部 6+ 个 `Events<(&str, SettingsMenuState)>` / `Events<(DecoratorId, Vec<PreferredNodeAlignment>)>` 事件写面（core 无该事件类型）。

## color_menu（0x142291230）未实现根因 + 已解结构

**未实现（reviewer 复核通过，登记准确）**：反编译证明真实控制流为**逐图标 0xa0B
状态机**（menu state byte @+0x30 的 switch 0..0x15）。实现须读取/写入
`CustomizationIcon` 的 0xa0 字节布局与 per-kind 值记录，core `CustomizationIcon`
为 `_gap: u64` typed-gap；`Events<(&str, SettingsMenuState)>`、History Color 字节
记录（`{1,1,tag,...}`）、~12 参数系统 ABI 均未建模 → 无法逐语句判等编译。
**不构造近似控制流**（保持 PDB 候选签名与读取面）。

已完整登记（`color_menu.rs` 模块头，供 core 落库后重建）：
- CustomizationIcon 0xa0 字段布局表（+0x30 menu state、+0x68 kind、+0x50 中心、+0x90 半径等）。
- 主循环 7 段结构（双 `just_pressed` 门 / 选中推进跳分隔符 / widget 交互 / 菜单态收尾 / 菜单布局 polar 变换 / 主 switch）。
- 主 switch case 表（0/1/2/4/5/7/8/0xc/0xd/0xe/0xf/0x10/0x11/0x12/0x13/0x14/0x15 → 事件写者、图标值记录偏移、History tag 3/4/5/7/8/0xa/0xb/0xc/0xd）。
- History::add 记录布局（`{1,1,<tag>@+3,…}`，第二参 `{9,0}` = EditType::Color tag 9）。

## Reviewer 复核修正（wave2/reviews/color/REVIEW.md）

- 结构骨架（事件遍历/两段跳表/demo 门/is_flat 门/创建/spawn/串）判等通过。
- 修正项：case 0 首图标非分隔符（=bricks_beige kind 3）；bricks 为 kind 3 非 0xc；
  roof_dark/roof_grey 移入 case 1；plaster_peel 为 low/medium/high 三项且条件追加；
  补主 kind-3 色块（13 swatch）与 halftimber/skirt 条件块。
- `color_menu` 未实现登记复核通过。

## 证据

- 反编译：`reports/re-analysis/hardpoints-ecstate/evidence/decomp/color_menu_142291230_full.c`、
  `show_color_options_14229a7f0_full.c`（写 worker fresh clone）；reviewer 复核版：
  `reports/wave2/reviews/color/evidence/decomp/`（`revieww2color` fresh）。
- 本报告证据：`reports/wave2/color/evidence/results/read-strings.json`、
  `read-extra.json`（shared 只读 read_memory，worker=shared）——选项串确证：
  - `0x14303c158`：color_icon/bricks_beige/yellow/light/grey/dark
  - `0x14303c23d`：color_icon/skirt + color_icon/no_skirt + color_icon/ha…(lftimber)
  - `0x14303c36f`：color_icon/roof_dark + roof_grey
  - `0x14303c1e8`：color_icon/plaster_peel_low + plaster_pe…
  - `0x14303c260`：color_icon/halftimber
  - `0x14303c2a1`：color_icon/roof_ochre + roof_red + …
  - `0x14303c356`：color_icon/roof_dark_wood + roof_dark…
  - `0x14303ca70`：kind→handler 跳表（40 项相对偏移）。
- 身份：`tiny-glade.exe` sha256 `cf048e27…6323`（B patched，与 re-analysis MANIFEST 一致）。

## 顺带修复（本 crate 内部，country-core 事件类型合同变更后遗留）

`cargo check -p system-color` 曾因 country-core 的 `SetWallColorIdCmd/SetGateColorIdCmd`
字段重构（`wall_id/gate_id` → `id: u32`）导致本 crate 4 处字段名错误。已机械修正
`crates/systems/color/src/set_wall_color_id.rs` / `set_gate_color_id.rs` /
`set_halftimber_color_id.rs`：`cmd.wall_id/gate_id` → `cmd.id`，`WallId(cmd.id)` →
`WallId(u64::from(cmd.id))`（对齐 `WallId(pub u64)`）。未改他 crate 文件。

## 编译证据

- `cargo check -p system-color`：**通过（时间窗内）**——reviewer 复核修正后复测
  `Finished dev profile …`，**0 error，本 crate 0 warning**（country-core 依赖
  10 条 warning 属该 crate 既有，非本 crate）。
- 依赖窗口期：本次交付时点 workspace 被他组并发编辑持续波动（save 组
  `load_game.rs` 曾错、rhapsody 组 `crates/rhapsody/src/rg/pass_api.rs` E0425
  `PreparedRenderPass` 现错）——`cargo check -p system-color` 的失败全部位于
  **他组 crate（rhapsody）**，`grep crates/systems/color` 无 error/warning。按
  color RESTORE 既有约定（"在依赖通过的时间窗内本 crate 已完整通过检查，
  未代改他组文件"）登记，不代改他组文件。
- 涉及文件：`crates/systems/color/src/color_menu.rs`、`src/show_color_menu.rs`
  （重建/登记），`src/set_wall_color_id.rs`、`src/set_gate_color_id.rs`、
  `src/set_halftimber_color_id.rs`（顺带字段名修复）。
