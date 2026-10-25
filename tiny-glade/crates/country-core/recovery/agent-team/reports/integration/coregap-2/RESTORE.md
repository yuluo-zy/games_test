# RESTORE — coregap-2：core-type-gap / core-api-missing 第二组

> restorer-coregap-2，2026-10-17。范围按
> `recovery/agent-team/planning/2026-10-17-unimplemented-ledger.md` §1 core-type-gap /
> §2 core-api-missing 的**第二组**（onboarding GameProfile 提示区 + NotifyHintSystem 族 +
> ColorIdSsbo 族 + Decorator 载荷 + SteamInput/命令事件）。
> 方法：逐类型 fresh 反编译定位原布局/语义（Ghidra shared worker + 组内已落 evidence），
> 不发明字段；**未**触碰他组在途文件（`lib.rs`/`translate_input.rs`/`steam.rs`/rhapsody）。
> 身份：B patched EXE tiny-glade.exe（sha256 cf048e27…6323）。

## 实现清单（含 fresh 证据）

| # | 类型/API | 文件 | 证据（地址 / 反编译面） | 状态 |
|---|---|---|---|---|
| 1 | **GameProfile 提示区**：相机字形模式 u16 @0x1bc/0x1c0/0x1c4 + 计时区 + 完成标志区 | `src/game_profile.rs` | `camera_rotate_hint` 0x14227d2f0 / `camera_pan_hint` 0x14227d000 / `make_focus_or_camera_binding_hint` 0x141c3de00 读 `*(short*)(profile+0x1bc/0x1c0/0x1c4)` 比 1/2；`exponential_back_off` 0x142284800（0x1e0/0x1e4/0x1e8/0x1ec/0x1f0 f32，720.0=0x44340000）；`tutorial_completion` 0x14227c230 18 路 switch 写 0x1f4..0x205；`reset_hints` 0x141c1c8d0 置位/清零 0x1f4..0x206 + 0x1e0..0x1ef | 实现 |
| 2 | **TutorialCooldowns 计时区**（20B：5×f32） | `src/game_profile.rs` | exponential_back_off 写面（两对计时/冷却 + 累加器） | 实现 |
| 3 | **16× `NotifyHintSystem*` 事件**（unit；CameraMoved 带 `{tag u32, value u32}`） | `src/resources/onboarding.rs` | `add_onboarding` 0x140650d50 register_event 序列（16 个 country-core 路径事件）；CameraMoved 载荷 = `hint_camera_progress_update` 0x14227bd30 0x10B 步长读 +8/+0xc | 实现 |
| 4 | **`TutorialOp`**（18 变体 + id→偏移表） | `src/resources/onboarding.rs` | tutorial_completion 0x14227c230 18 路 switch case 表（**非连续偏移**） | 实现 |
| 5 | **`WriteSettingsFileCmd` / `SaveGameProfileCmd`**（unit） | `src/resources/events.rs` | exponential_back_off / reset_hints / tutorial_completion `EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>` 8B 步长（unit 载荷）；SaveGameProfileCmd 同 app_config 域（I） | 实现 |
| 6 | **`DisplayRollercoasterPrompt`**（unit） | `src/resources/events.rs` | add_onboarding register_event 顺序 21，`country_core::resources::events` 路径 | 实现 |
| 7 | **`ModeJustChanged` 载荷**（0x20B payload：{tool u32, +0xc..0x20 U}） | `src/resources/events.rs` | hint_focused_edit_condition 0x14227da70 0x28B 步长遍历、`tool-1<2`；autoclose_color_menu 0x142298d30 同 0x28B | 实现 |
| 8 | **`StructureType::is_fence`** | `src/resources/walls/inner_wall_state.rs` | fresh 反编译 0x1408c5680 + RNE `0fb601c3` = `movzx eax,[rcx];ret`（字节 0 直通）；调用面 hint_camera_display_condition 0x14227c910 | 实现 |
| 9 | **`WallColorIdSsbo`/`GateColorIdSsbo`/`HalftimberColorIdSsbo`** + **`SsboType`**（StorageElement NAME=复数槽名、BUFFER_LEN=20480、TypedStorageBuffer） | `src/resources/color_id.rs` | init_color_ids_buffer 0x142290750 add_new；set_wall_color_id 0x142290820 memcmp 0x32 / set_gate 0x142299840 0x32 / set_halftimber 0x142299250 0x38；槽名 "WallColorIds"（SsboId 0xaf42315d263aa22a 测试命中）/ "GateColorIds" / "HalftimberColorIds" | 实现 |
| 10 | **`RoofColorIdSsbo`**（color_id_roof 域） | `src/resources/color_id_roof.rs`（新） | write_roof_color_id_to_gpu 0x142298520 memcmp 0x37=55B（全名 55 字符命中）；槽名 "RoofColorIds"（SsboId 0x8cce418b17d89f9b 测试命中） | 实现 |
| 11 | **事件载荷**：墙族 SetWallColorIdCmd/SetGateColorIdCmd/SetHalftimberColorIdCmd = `{id u32, +4 U, color u32@+0x10, custom@+0x14}` 0x18B；装饰/窗族 SetDecoratorColorIdCmd/SetWindowLightCmd/SetLadderColorIdCmd/SetMagicColorIdCmd/SetDecoratorBrightnessCmd = `{id u32, value u32@+0xc, custom@+0x10}`（**无 gap**）0x18B；SetStairRailingCmd/SetFlagConnectionCmd/SetChimneySmokeCmd = `{id u32, value u8, custom u8}` 0x10B | `src/resources/color_id.rs` + `src/resources/walls/decorator.rs` | 各 set_* 消费面 0x18B/0x10B 步长（set_flag_connection_and_chimney_smoke 0x142298970 读 value@+0xc/custom@+0xd；set_window_light 0x142298260 读 value@+0xc/custom@+0x10；set_decorator_color_id 0x1422af0c0 同）；reviewer-coregap-2 §3 fresh 反编译纠错（删 _gap4、改名）| 实现 |
| 12 | **`ChimneyExtra` +4 字段**（{f32, smoke u8}） | `src/resources/walls/decorator.rs` | set_flag_connection_and_chimney_smoke `as_chimney()` 后写 `*(u8*)(+4)` | 实现 |

编译/测试：`cargo check -p country-core` **通过**（9 warnings 全在 save/snapshot.rs 等
非本批文件）+ 14 单测全过——在依赖可编译的时间窗内复验（2026-10-17，rhapsody 组修复后）。
**编译状态归属**：外部在途改动多次打断依赖/同 crate 编译（rhapsody 组
`crates/rhapsody/*`：PushConstantRange PartialEq → result_cache match → import_unowned
实参；后另一组 08:50 修改 `render/shader.rs` 引入 `into_boxed_dyn_error`（anyhow 版本不符）
级联 pass_kit.rs 推断错）——均非本组文件，未代改。

## reviewer-coregap-2 需修 2 组——已落实（2026-10-17，REVIEW.md）

### ③ 事件载荷族 `_gap4` 删除 + 类型改名（5 条）
- `SetDecoratorColorIdCmd`/`SetWindowLightCmd`/`SetLadderColorIdCmd`/`SetMagicColorIdCmd`/
  `SetDecoratorBrightnessCmd` = `#[repr(C)] {id u32, value u32, custom u8, _pad[7]}`，
  **删除 `_gap4`**（消费面 fresh 读 value@+0xc、custom@+0x10 无 gap——墙族
  SetWallColorIdCmd 的 +4 间隙不适用于装饰/窗族）。
- 二进制 register_event 全路径名校正：`SetLadderCmd`→**`SetLadderColorIdCmd`**、
  `SetMagicCmd`→**`SetMagicColorIdCmd`**、`SetBrightnessCmd`→**`SetDecoratorBrightnessCmd`**
  （type_name/register_event 身份与二进制一致）。
- 单测 `color_commands_have_the_observed_event_strides` 扩至 8 个 0x10B 断言全过。

### ④ GameProfile serde 形状（player_id / game_installation_id 相反）
- `serialize<ron>` 0x140a658a0 fresh 证实写 **player_id**（第 3 位）、**不写**
  game_installation_id；`deserialize` 0x1404b0ed0 名表 case 0x14 把旧档
  `game_installation_id`（20 字符）作为 player_id 别名。Rust 已改：删除
  `game_installation_id` 字段，`player_id` 序列化 + `#[serde(alias = "game_installation_id")]`。
- 测试改 `profile_ron_writes_player_id_and_aliases_legacy_installation_id`（含旧档别名
  反序列化断言），全过。

### 附带登记（次要不符 / 模块路径）
- `reset_hint_flags()` 二进制清零分支只清 **0x1e0..0x1ef**（两对计时/冷却 → 0）、
  **不动 +0x1f0** 累加器——已按此修正（原实现整块重置 TutorialCooldowns 不符）。
- 事件类型二进制模块域 = `system_color::set_decorator_color_id` / `set_window_light`；
  coregap-2 按总账分派落 `country_core::resources::color_id`（事件注册身份以类型名为准，
  模块路径差异登记不迁移）。

## 冲突跳过 / 他组覆盖

| 项 | 归属 | 说明 |
|---|---|---|
| **`SteamInput` + `get_action_origin_glyphs`** | rebuild-steam（他组）| 我方创建 `src/resources/steam.rs` 后被该组完整重建覆盖（SteamInput 资源 + 动作集 + `get_action_origin_glyphs(&self, ActionOriginGlyphQuery) -> Option<Vec<String>>`，含 fresh 证据），比占位边界更全。我方仅保留 `resources/mod.rs` 的 `pub mod steam;` 注册（无重复），不再代改。SteamInput 项由该组落地，**非本组实现**。 |
| `src/game_profile.rs` / `src/resources/color_id.rs` / `src/resources/render_ssbo.rs` | 并发编辑 | 他组在他文件有并发写；我方编辑 cleanly applied，`cargo check` + 单测复验通过，未冲突。 |
| rhapsody `pipeline.rs` PushConstantRange 编译错 | rhapsody 组（在途）| 曾阻断 `cargo check`，后该组修复（复验通过）；期间未代改。 |

## 待证 / 登记（未实现部分）

- **`DisplayHintRequest` 布局**：`system_onboarding::hint_orchestrator::DisplayHintRequest`
  （非 country-core 类型，上层域）。已知：byte@+0 = hint id（3/6/7/10 等）、flags
  0x100..0x1200 随 BindingHintDisplay 载荷装配、+0x49 = HintType、0x50B 载荷。
  完整布局留给 system-onboarding restorer 按 batch3/batch5/batch6 反编译续证。
- **SetWallColorIdCmd 载荷 +0xc..+0x10 u32**（re-analyst §2 未定）——保留 `_gap4`。
- **GameProfile 相机三标志 0x1f6/0x1f7/0x1f8 与 pan/rotate/zoom 的精确对应**——仅
  0x1f6..0x1f8 簇确认，字段名按 CameraPan/CameraRotate/CameraZoom 标注（I）。
- **TutorialCooldowns ron 序列化形状** vs 真实存档（serde 仍派生；旧档兼容待取证）。
- **0x206 复位字节**语义 U（reset_hints 两分支都写，不在 18 路 switch）。
- **SaveGameProfileCmd** 证据为 app_config 域推断（I）；tutorial_completion 实际发
  WriteSettingsFileCmd（RESTORE-onboarding 表中"SaveGameProfileCmd"表述据 fresh 反编译修正）。
- **GraphicsDeviceResource**（render 边界）：gpu_flush_buffers 属 system-roof 消费端；
  库侧设备经 `rhapsody_contract::GraphicsDevice`（render/ssbo.rs WIRING §4.8 已登记缺口），
  本组未改该在途边界。

## 他组文件：未触碰

`src/lib.rs`、`src/systems/backend/input/translate_input.rs`、`src/resources/steam.rs`
（rebuild-steam 重建）、`crates/rhapsody/*`、`src/resources/history/*`（StairsHistoryEdit
在途测试错已由该组修复）。

## 证据索引

- 本组 fresh 反编译：`evidence/result-fresh.json`（0x1408c5680 is_fence RNE `0fb601c3`、
  make_focus_or_camera_binding_hint 0x1c4 读面、hint_position_update）。
- 复用组内 evidence：onboarding `evidence/results/batch{3,4,5,6}-*.txt`（add_onboarding
  register_event 序列、GameProfile 提示区/字形/偏移、stairs/terrain 门控位）；
  color `logic/decomp/fresh1_*.c`（init_color_ids_buffer / set_wall/gate/halftimber /
  write_roof / set_flag_connection_and_chimney_smoke / set_window_light 的 memcmp 长度与
  事件步长）。
