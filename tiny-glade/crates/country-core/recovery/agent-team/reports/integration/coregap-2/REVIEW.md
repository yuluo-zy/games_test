# reviewer-coregap-2 — core-type-gap / core-api-missing 第二组 REVIEW

日期：2026-10-17
角色：core 集成 reviewer（coregap-2）
范围：独立核验 restorer-coregap-2 在 `crates/country-core/src/` 补齐的 12 个 core 类型/API + SteamInput 冲突跳过登记，逐条 fresh 反编译/字节直读对回二进制真实语义（硬标准：不发明字段）。

方法：ghidra `--worker shared` 两批 fresh 反编译（`review-evidence/result-review1.json`：
0x142298d30 / 0x142290750 / 0x142298260 / 0x1422af0c0 / 0x14227bd30 / 0x14227da70 /
0x14227c230；`result-review2.json`：0x140a658a0 / 0x1404b0ed0 / 0x140916420）+ 复用
组内 fresh `.c`/`.txt` 交叉核对（onboarding batch3-core/batch4-camera/batch5-focused、
color fresh1_*、render_bridge LOGIC）。仅评审，未改 src。

**`cargo check -p country-core`：通过（Finished dev，41.5s；10 warnings 全在
save/snapshot.rs 等非本批文件）。`cargo test --lib -- color_id::tests
onboarding::tests game_profile::tests decorator_extra_info_roundtrip color_id_roof`：
9 passed / 0 failed。**

## 裁决汇总（12 条 + 1 冲突跳过）

| # | 类型/API | 裁决 | 关键证据（fresh） |
|---|---|---|---|
| 1 | **GameProfile 提示区**（0x1bc/0x1c0/0x1c4 u16 + 0x1e0..0x1f0 计时 + 0x1f4..0x206 19 标志） | **判等**（含 2 处次要不符登记，见 §1/§2） | `camera_rotate_hint` 0x14227d2f0 读 `*(short*)(p+0x1bc)`、`camera_pan_hint` 0x14227d000 读 `*(short*)(p+0x1c0)`、`make_focus_or_camera_binding_hint` 0x141c3de00 读 `*(short*)(p+0x1c4)`，均比 1/2；`exponential_back_off` 0x142284800（+0x1e0/0x1e4/0x1e8/0x1ec/0x1f0 f32、`0x44340000`=720.0、`clamp(old*1.5,720,10800)`）；`tutorial_completion` 0x14227c230 18 路 switch；`reset_hints` 0x141c1c8d0 置位/清零 0x1f4..0x206 + 0x1e0..0x1ef |
| 2 | **TutorialCooldowns**（20B 5×f32） | **判等** | 写面/清零面 +0x1e0/+0x1e4/+0x1e8/+0x1ec/+0x1f0 全命中；`default_cooldowns` 720.0 与 `0x44340000` 一致 |
| 3 | **16× `NotifyHintSystem*` 事件**（CameraMoved 带 `{tag u32, value u32}`） | **判等** | `add_onboarding` 0x140650d50 register_event 序列 1/3/4/5/9/10/11/12/13/14/15/16/17/18/19/20 + TutorialOp(7) + DisplayRollercoasterPrompt(21)；`hint_camera_progress_update` 0x14227bd30 事件 stride 0x10、tag==2→rotate(dt)、==1→zoom(用 value=`piVar13[1]`)、其它→pan(dt) |
| 4 | **`TutorialOp`** 18 变体 + id→偏移表 | **判等** | 我 fresh 0x14227c230 18 路 switch 逐 case：0→0x1f4、1→0x1f5、2→0x1fe、3→0x1fc、4→0x1fa、5→0x1fb、6→0x1f9、7→0x1ff、8→0x1fd、9→0x200、0xa→0x1f6、0xb→0x1f7、0xc→0x1f8、0xd→0x201、0xe→0x202、0xf→0x203、0x10→0x204、0x11→0x205 —— 与 `completion_flag_offset()` 逐项一致 |
| 5 | **`WriteSettingsFileCmd` / `SaveGameProfileCmd`** | **判等**（SaveGameProfileCmd I 登记） | `exponential_back_off`/`reset_hints`/`tutorial_completion` 三函数均 `grow_one<EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>>`、8B 步长（unit 载荷）。tutorial_completion 发的是 WriteSettingsFileCmd（**更正** onboarding 组旧表 "SaveGameProfileCmd" 表述，fresh 证实）|
| 6 | **`DisplayRollercoasterPrompt`**（unit） | **判等** | register_event 顺序 21、路径 `country_core::resources::events::DisplayRollercoasterPrompt`；`prompt_rollercoaster` 0x14227bb70 消费（不读载荷）后发 `DisplayHintRequest(0x1200, id 10)` |
| 7 | **`ModeJustChanged`** 载荷（0x20B {tool u32, +0xc..0x20 U}） | **判等** | `hint_focused_edit_condition` 0x14227da70 事件 stride 0x28、`*piVar5 - 1U < 2`（tool∈{1,2}）；`autoclose_color_menu` 0x142298d30 同 0x28 步长 |
| 8 | **`StructureType::is_fence`** | **判等** | 0x1408c5680 字节 `0fb601c3` = `movzx eax,[rcx];ret`（RNE 直读，evidence `result-fresh.json`）；调用面 `hint_camera_display_condition` 0x14227c910 以 `==0` 判别 |
| 9 | **`WallColorIdSsbo`/`GateColorIdSsbo`/`HalftimberColorIdSsbo`** + `SsboType` | **判等** | `init_color_ids_buffer` 0x142290750 add_new×4（Wall/Roof/Gate/Halftimber，fresh）；`set_wall_color_id` 0x142290820 memcmp 0x32、`set_gate` 0x142299840 0x32、`set_halftimber` 0x142299250 0x38；全名长 0x32/0x32/0x38（测试命中）；槽名复数 "WallColorIds" 命中 read_storage_buffer 0xaf42315d263aa22a（render_bridge LOGIC）；BUFFER_LEN=20480（`0x4fff < idx` panic 断言）|
| 10 | **`RoofColorIdSsbo`** | **判等** | `write_roof_color_id_to_gpu` 0x142298520 memcmp 0x37=55B（全名 `…color_id_roof::RoofColorIdSsbo` 55 字符，测试命中）；槽名 "RoofColorIds"（ssbo_tags RoofColorIdTag 0x8cce418b17d89f9b）|
| 11 | **事件载荷 0x18B/0x10B 两族** | **部分判等**（5 条需修，见 §3） | set_wall/gate/halftimber = `{id u32, +4 U, color u32@+0x10, custom@+0x14}` ✓；set_stair_railing/flag/chimney = `{id u32, value u8@+0xc, custom u8@+0xd}` ✓；**SetDecoratorColorIdCmd/SetWindowLightCmd/SetLadderCmd/SetMagicCmd/SetBrightnessCmd 消费面 value@+0xc/custom@+0x10（无 gap），Rust 却含 `_gap4`** |
| 12 | **`ChimneyExtra` +4 smoke** | **判等** | `set_flag_connection_and_chimney_smoke` 0x142298970 `as_chimney()` 后 `*(u8*)(extra+4) = value@+0xc`；Rust `{f f32, smoke u8}` + `as_chimney_mut` 成立 |

## 冲突跳过核验

**SteamInput / get_action_origin_glyphs —— 登记准确（rebuild-steam 覆盖成立）**：
- `src/resources/steam.rs`（19686B，2026-10-17 06:57）含 `SteamInput` 资源 + `get_action_origin_glyphs(&self, ActionOriginGlyphQuery) -> Option<Vec<String>>`（0x1409e6280 载荷注释）。
- coregap-2 仅保留 `src/resources/mod.rs` 的 `pub mod steam;`（第 63 行），未代改 steam.rs。冲突跳过行为正确。
- `add_onboarding` 中 `country_core::resources::steam::impl$31::from()` 引用证实 steam 路径已注册。

## §1 事件载荷族 —— 需修（5 条）

**`SetDecoratorColorIdCmd`/`SetWindowLightCmd`/`SetLadderCmd`/`SetMagicCmd`/`SetBrightnessCmd`
的 `_gap4: u32` 是错误发明** —— 把墙族（wall/gate/halftimber 确有 +4 间隙）布局错误套用到装饰/窗族。

我 fresh 反编译消费面逐字段：
- `set_window_light` 0x142298260（stride 0x18）：`uVar4 = *(u32*)(rec+0xc)` → `as_window().f`；`if (*(u8*)(rec+0x10)==1)` custom 门控。→ 载荷 `{id@+8, value@+0xc, custom@+0x10}`，**无 gap**。
- `set_decorator_color_id` 0x1422af0c0（L1/L2/L3/L5 均 stride 0x18）：value 读 +0xc 写 DecoratorInfo+0x28 / LanternExtra+0 / StairExtra 节点；custom 读 +0x10。→ 同 `{id, value@+0xc, custom@+0x10}`。
- color 组 RESTORE 纠错 #4 明确："`SetWindowLightCmd/SetDecorator*/SetLadder/SetMagic/SetBrightness` = `{id u32, value u32, custom u8}` stride 0x18；旧实现 `_gap: u64` 全部不符"。coregap-2 未落实该纠错。

**应改为** `#[repr(C)] { id: u32, value: u32, custom: u8, _pad: [u8;3] }`（0x10B 载荷/0x18B 步长），删除 `_gap4`；doc 注释同步。

### 附带（名称/模块不符登记）
- 二进制类型名（`color_menu` register_event 全路径）：`SetLadderColorIdCmd`（**非** `SetLadderCmd`）、`SetMagicColorIdCmd`（**非** `SetMagicCmd`）、`SetDecoratorBrightnessCmd`（**非** `SetBrightnessCmd`）；且这五个 + `SetStairRailingCmd` 均在 `system_color::set_decorator_color_id` / `system_color::set_window_light` 域（非 `country_core::resources::color_id`）。coregap-2 放在 country-core 属总账分派，但**事件类型名应与二进制一致**（type_name/register_event 身份）。SetWall/Gate/HalftimberColorIdCmd 与 SetFlagConnectionCmd/SetChimneySmokeCmd 的模块路径则正确。

## §2 GameProfile serde 形状 —— 需修登记（1 项）

我 fresh `serialize<ron>` 0x140a658a0 / `deserialize` 0x1404b0ed0：
- serialize 写 5 字段：`tutorial`(TutorialCompletionState)、`tutorial_cooldowns`(TutorialExponentialBackOff)、**`player_id`**(PlayerId)、`key_bindings`、`new_glade_count`(usize)。**不写 game_installation_id**。
- deserialize 名表：tutorial(0)/tutorial_cooldowns(1)/**player_id(2)**/key_bindings(3)/new_glade_count(4)；case 0x14 把 **`game_installation_id`(20 字符) 作为 player_id(2) 的旧档别名**（跳同一 code 分支）。

Rust `game_profile.rs` 却 `#[serde(skip_serializing)]` player_id、把 `game_installation_id: Uuid` 放第 3 序列化位 —— **与二进制 ron 形状相反**。应改为：序列化字段为 `player_id`（含反序列化 `game_installation_id` 别名），`game_installation_id` 不作为独立序列化字段。测试 `profile_ron_skips_player_id_but_roundtrips_others` 断言当前（错误）形状，需同步。

### 次要不符（登记，非阻断）
- `reset_hint_flags()`：二进制 `reset_hints` 清零分支只清 0x1e0..0x1ef（cooldown→0）且**不清 +0x1f0**；Rust 把整个 `TutorialCooldowns` 重置为 default（cooldown→720、+0x1f0→0）。`set_all_hint_flags` 与置位分支逐字节一致（0x1f4..0x206=1）。复位字节 +0x206 语义 U（两分支都写，18 路 switch 不含）已诚实登记。

## 观察

- 所有"未做/待证/GAP"登记均诚实（SaveGameProfileCmd I、DisplayHintRequest 布局留给 system-onboarding、SetWallColorIdCmd +0xc..+0x10 U、0x206 U、GraphicsDeviceResource render 边界），无冒充。
- `TutorialOp`/`GameProfile` 提示区是本批最扎实的部分：18 路 switch 的 id→偏移表与 `completion_flag_offset()` **逐项判等**（非连续偏移含 0x1fe/0x1fc/0x1fa 等）。
- `WriteSettingsFileCmd` 的 unit/8B 步长由三个消费函数交叉证实，onboarding 组旧 "SaveGameProfileCmd" 表述更正正确。
- 事件记录内部 EventInstance 的 id/payload 相对序（project 约定 "+8 payload"）在两处读面上出现 ±8 表象差异（hint_camera_progress_update 读 tag 于记录 0 位 vs 其余函数读载荷于 +8），属 bevy 分叉记录布局细节，不影响本批 Rust 类型的字段语义判定；如后续做字节精确承接，需在事件层统一核实。

## 统计

判等 10 / 需修 2（③ SetDecoratorColorIdCmd 等 5 条事件载荷 `_gap4`；④ GameProfile serde player_id/game_installation_id）/ 附带登记 2（事件类型名与模块、reset_hint_flags 次要不符）/ 冲突跳过核验 1（SteamInput 登记准确）。
cargo check：通过；本批单测 9/9 通过。

## 证据

- `review-evidence/result-review1.json`：0x142298d30 / 0x142290750 / 0x142298260 / 0x1422af0c0 / 0x14227bd30 / 0x14227da70 / 0x14227c230（我 fresh）
- `review-evidence/result-review2.json`：0x140a658a0 / 0x1404b0ed0 / 0x140916420（我 fresh）
- `evidence/result-fresh.json`：0x1408c5680 `0fb601c3` + 0x141c3de00 +0x1c4 读面
- 交叉引用：`reports/interaction/onboarding/evidence/results/batch{3,4,5}-*.txt`、`reports/state/color/logic/decomp/fresh1_*.c`、`reports/state/color/RESTORE.md` 纠错 #4、`reports/interaction/render_bridge/LOGIC.md`（0xaf42315d263aa22a）
