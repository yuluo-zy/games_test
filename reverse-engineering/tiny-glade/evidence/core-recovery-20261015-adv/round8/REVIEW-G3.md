# 第八轮 G3 ③ REVIEW（2026-10-16）

> 审查者：G3 ③ review agent。基准：round8 G3① 契约（round8/contracts/G3-*）与 G3② 实现
> （round8/deliveries/G3-*），对照真实源码（`D:/game/tiny-glade/crates/country-core/`）与既有证据
> （round4/contracts/t3-input-update_mod_key_state、round6/contracts/r6-gamekeybindings-modkey、
> toolbar-origin-2026-10-16、round4/REVIEW）。
> 状态：**地面审查基线已建立（round8 契约/DELIVERY 尚未落盘时先行）**；契约/DELIVERY 落盘后
> 在本文件补对照复审段。只读审查，不改源码不跑 cargo。

---

## 1. 地面审查：G3 域源码现状（契约/DELIVERY 落盘前基线）

### 1.1 GUI 全链（PhotomodeScreen / DebugGui / EscMenuState）

| 源码 | 现状 | 消费端 | 缺口（地面审查发现） |
|---|---|---|---|
| `resources/gui.rs` | 数据模型较完整：EscMenuState（view tag 2..10/flag）、DebugGui（flags@+0xf0..0x102、camera_mode@+0x103、normal_length、camera_snapshot_interpolation_speed）、PhotomodeScreen{was_enabled,enabled}、ScreenLog 5 槽、LoadMenuState、TimeOfDayScreen | DebugGui→music_system.rs（debug）、draw_callbacks（suppress@+0xfe）；EscMenuState→translate_input/music；PhotomodeScreen→music_system.rs（Res<PhotomodeScreen>） | **G1-1** `configure_gui_consumers`（backend/gui.rs）与 `configure_photomode_visibility`（backend/photomode.rs）**未进生产 Schedule**——`register_core_startup`（run_game.rs）无 GUI 面注册；show_debug_controls/show_screen_log/flush_error_log_to_screen/update_photomode_visibility 只在测试跑。 |
| `resources/photo_mode.rs` | PhotoModeCameraMode（TiltShift/Free/Walk/TopDown 0..3）next/next_demo + From→CameraMode(tag+1) | DebugGui.camera_mode、PhotomodeState | 已对齐；未见缺失 |
| `startup/init_photomode_state.rs` | PhotomodeState（0x1408705b0 +0..0xd） | update_photomode_visibility、music_system | **G1-2** PhotomodeScreen.enabled **无 crate 内 producer**（原 do_photomode 写 enable/disable + 帧末拷贝；现 finish_frame()/just_enabled() 无调用者）。consumers 有、producer 缺（宿主/toolbar 面） |
| `systems/backend/gui.rs` | show_debug_controls（action 28 toggle enabled、action 33 toggle experimental_window）、show_screen_log（GameTime.delta 老化）、publish_pending_egui_context | —（未注册生产） | **G1-3** 同上：egui 消费系统未进 Schedule；GUI 面是 egui，原版 GUI 栈 = Taffy 0.3.13 + mei + tg-text（toolbar-origin 报告 §2/§3），**库面差异需登记，不是"分叉 egui"** |
| `systems/backend/flush_screen_log.rs` | 线程安全 producer buffer + flush（0x1408c0d90） | —（未注册生产） | **G1-4** flush_error_log_to_screen 只在 configure_gui_consumers（未注册） |
| `resources/ui_settings.rs` | UiSettings 7 字段 + serde（0x140b12f10 写/0x140a46ad0 序列化） | **零消费者（直接）** | **G1-5** UiSettings 无直接消费者；input_record_replay.rs `InputReplayInitialState.ui_settings` 是 `serde_json::Value` 占位（透传 JSON 值）——原回放初始态 serialize 键含 `ui_settings`（0x142c5b42e 长度 0xb），两链可能同 schema，待① 核对 |
| `resources/onboarding.rs` | NotifyHintSystemToolWasJustUsed（事件） | world_raster.rs:153 发、garden/draw_garden 读 | 已闭合（有 producer/consumer） |

### 1.2 input 剩余（mod_key 键码 / GameKeyBindings value）

| 源码 | 现状 | 消费端 | 缺口 |
|---|---|---|---|
| `systems/backend/input/mod_key.rs` | Modifiers{flags:u32@0}、disable_snapping（@0x1408c5680 byte0）、is_disable_snapping_just_pressed（@0x140af9bf0 byte1）、**update_mod_key_state producer**（0x140af9c00 表1 分派：组0 ON/1,3,4 OFF/2 setae[+0x7c] 阈值/5,6,7 键码待取证→OFF）、ModKeyConfig{state,threshold} 适配 | mod.rs register（init Modifiers/ModKeyConfig）+ add_input_cpu_consumers（挂调度） | **I-1** **组 5/6/7 键码映射待取证**（表2 @0x142ae5f64 动作码 `r11-5`）：现按 OFF 登记不造；`ModKeyConfig` 是适配资源，config 身份（哪个资源含 +0x78/+0x7c）待①。 |
| `game_profile.rs` | KeyBinding{key:u32@0,value:u64@4} `#[repr(C,packed)]` 12B、GameKeyBindings 40 槽固定数组、default KEYS 表与 BindingsTable 逐字节一致、serde Vec 中转（N>32）、GameProfile Resource | —（见 I-2） | **I-2** **GameKeyBindings.value 动作 id 语义待取证**（默认全 0="未绑定"）；**I-3** `key` 宽度 u32 vs translate_input BindingsTable 用 u16 —— 槽码宽度不一致，需①核对（r6 契约讲 40×12B `{key u32@0,value u64@4}`，translate_input 侧 ActionBinding{key:u16,mouse:u16} 是另一套）。 |
| `systems/backend/input/translate_input.rs` | 全 O 结构（Steam 门/EscMenu 门/模拟槽 10 项/数字槽 29 项/MouseWheel/右键手势/硬编码键/0xa7 恒启） | add_input_cpu_consumers 挂调度 | **I-4** **translate_input 仍用 `BindingsTable::default()` 硬编码**，未消费 `Res<GameProfile>`（PDB 签名第 12 参）的 key_bindings —— GameKeyBindings 成零消费字段。原版读 GameProfile 键位表逐槽；② 应在① 定 value 语义后接上。 |
| `systems/backend/input/update_cursor_position.rs` | 按 PDB 签名 4 参（states/captured/CursorPositionSS/EventWriter CursorMovedScreenSpace） | add_input_cpu_consumers 挂调度 | 已就位；捕获锚点交互（semantic I）登记 |
| `systems/backend/input/reset_input_state.rs` | 11 qword 置换 + 双事件排空 + reset_all（on_event<NewSessionStartedCmd> run 条件） | ⑥轮已接 | 已闭合 |
| `systems/backend/input/calculate_input_delta.rs` | UiHistoryButtonPressed 事件 + digital_delta 状态机（0.2s→0.1s repeat）+ auto_repeat 槽 20..25 | translate_input 消费 UiHistoryButtonPressed | **I-5** UiHistoryButtonPressed **生产者缺**（UI 历史按钮，组外）——消费端有、生产者无 |

### 1.3 paths / prefab / brush_preview 消费链

| 源码 | 现状 | 消费端 | 缺口 |
|---|---|---|---|
| `systems/paths.rs` | DrawState::update（0x140ae4bd0 三参 `(&mut mode, mode_id, raw)`）、ERASER_BRUSH_SCALE=0.5525、BRUSH_RECT_SCALE=0.8、PathRastersResources（SystemParam 四栅格） | DrawState→garden/draw_garden.rs（Local<DrawState>） | **P-1** **PathRastersResources 零真实消费者**：仅 doc 引用（wall_path_preparation/instance_updates/firstperson_steps 注释）；firstperson_steps 只实现 2 参（原生 8 参含 PathRastersResources + GroundType==3 bilinear 采样，"raster sampling 待接线"）。 |
| `systems/prefab.rs` + `resources/prefab.rs` | PrefabInstance、PrefabInstanceData、SpawnPrefabCmd、DespawnPrefabCmd（typed gap，布局 U） | — | **P-2** **systems::prefab::PrefabInstanceData 与 components::trees::PrefabInstanceData 双定义**（同字段 position/scale/rotation_degrees/_gap，两个类型）；SpawnPrefabCmd/DespawnPrefabCmd **零消费者**（原 producer forest manage_tree_prefabs 0x141bf16c0 / despawn_trees 0x1422686d0 未实现）。 |
| `systems/brush_preview.rs` | BrushPreview component、brush_preview 系统（读 CursorPositionSS+CursorTerrainRaycast+BrushSizes）、BrushPreviewUpdate 事件、register_required_components 空桩（0x140866800 glue）、CLONE_BEHAVIOR_DISC=2 | — | **P-3** **brush_preview 未进生产 Schedule**（无 register 调用）；**P-4** `_ => 1.0` 覆盖 Terrain(10)/Eraser(9) 等模式——原生 brush_size（行 94）读哪些 AppMode 待①；**P-5** DebugGui.hide_brush_previews（+0xfa）**未接入** brush_preview。 |

---

## 2. 已复核无水分项（地面审查）

- `Modifiers` 谓词两字节（byte0 disable_snapping / byte1 just-pressed）与 0x1408c5680/0x140af9bf0 机器码一致（read 谓词正确）。
- `GameKeyBindings::default` KEYS 40 表与 translate_input `BindingsTable::default` 逐槽一致（0x2a,0x26,…0xc3 哨兵槽36、鼠标 {1,2,1} 槽37..39），与 r6 契约 §2 对齐。
- `#[repr(C, packed)]` KeyBinding 12B 是③在 r6 复审要求，现结构真为 12B（非注释冒充）。
- `DrawState::update` 三参签名与纠错（raw∈{1,2} 写入 mode 而非 r）已按机器码落地；测试覆盖回绕分支。
- `brush_preview` 读的 AppMode tag（7=Path/15=Water/17=Garden）与 mode_manager/types.rs `native_tag` 一致。
- GUI 数据模型地址证据充分（EscMenuState view word/flag 表、DebugGui 各 flag +0xf0..0x102、PhotomodeState 0x1408705b0）。

---

## 2b. 功能缺失（消费者链断 / 无生产 / 身份未定）

- **F-G1** GUI 消费系统链未注册：`configure_gui_consumers`/`configure_photomode_visibility` 未入 `register_core_startup`，`show_debug_controls`/`show_screen_log`/`flush_error_log_to_screen`/`update_photomode_visibility` 只在测试跑；`PhotomodeScreen.enabled` 无 producer（`finish_frame`/`just_enabled` 无调用者）。
- **F-G2** `UiSettings` 零直接消费者（InputReplayInitialState.ui_settings 为 serde_json::Value 占位）。
- **F-G3** `translate_input` 未消费 `Res<GameProfile>` 键位表（`BindingsTable::default()` 硬编码）→ `GameKeyBindings` 零消费字段。
- **F-G4** `update_mod_key_state` 组 5/6/7 键码映射未取证（现按 OFF 登记）；`ModKeyConfig` 是适配资源（config 身份待①）。
- **F-G5** `PathRastersResources` 零真实消费者；`firstperson_steps` 只实现 2 参（原生 8 参含 PathRastersResources + bilinear 采样）。
- **F-G6** `systems::prefab` 的 `SpawnPrefabCmd`/`DespawnPrefabCmd`/`PrefabInstanceData` 零消费者；`PrefabInstanceData` 与 `components::trees` 双定义。
- **F-G7** `brush_preview` 未注册生产；`register_required_components` 空桩（0x140866800 glue）；`DebugGui.hide_brush_previews` 未接入。
- **F-G8** `UiHistoryButtonPressed` 生产者缺（UI 历史按钮，组外）。

---

## 3. 待① 深挖方向（G3 域，按优先级）

> 发给 G3①（a6690bb0fc2661701）。基于地面审查，契约未覆盖或证据不足项。

1. **update_mod_key_state 表2 @0x142ae5f64 键码枚举（组 5/6/7）**：动作码 `r11-5 ∈ 0..` 的键码→键组（ctrl/shift/alt/…）映射，及 `[config+0x78]` 状态值↔键组对应；config 资源身份（含 +0x78 状态 u32 / +0x7c 阈值 f32 的资源）。
2. **GameKeyBindings.value 动作 id 语义**：原生 value 是动作枚举判别还是槽索引？40 槽 value 默认值（是否非 0）？鼠标绑定槽 37/38/39 的 value？`key` 字段 u32 vs translate_input u16 两套宽度核对。
3. **translate_input 读 `Res<GameProfile>` 的确切面**：PDB 签名第 12 参；逐 digital 槽如何从 key_bindings 40 槽取键码（与 BindingsTable 下标对齐）。
4. **PhotomodeScreen.enabled producer**：原 do_photomode（toolbar/photomode）写 enable/disable 与帧末拷贝的宿主调度点；music_system 读 screen 的语义面。
5. **DebugGui.hide_brush_previews（+0xfa）消费面**：brush_preview 0x140865050（26KB）是否读它；brush_size 行 94 读 BrushSizes 的精确 AppMode 分支（Terrain/Eraser 是否纳入）。
6. **UiSettings 消费者**：0x140b12f10 写入者 / 0x140a46ad0 序列化的消费系统面。
7. **prefab 消费链**：SpawnPrefabCmd/DespawnPrefabCmd 原生产者（0x141bf16c0/0x1422686d0）载荷字段与 crate 内消费者；PrefabInstanceData 权威字段布局（0x58B 树命令/实例记录载荷族）。
8. **PathRastersResources 原字段 @+0x00/+0x20/+0x40/+0x60 身份**（四栅格引用确认）与 firstperson_steps 8 参全签名（B 组 audio 域接线面）。

## 4. 待② 修复项（G3 域，已确凿）

> 发给 G3②（a47cc91717efb0baf）。契约/DELIVERY 落盘后按契约复核增补。

1. **（I-4）translate_input 消费 `GameProfile.key_bindings`**：现硬编码 BindingsTable::default()，GameProfile 键位表零消费；① 定 value 语义后接 Res<GameProfile>（或至少登记缺口，不冒充已接）。
2. **（G1-1/G1-3）GUI 消费系统未注册生产**：configure_gui_consumers/configure_photomode_visibility 未入 register_core_startup；要么注册（需宿主 egui 上下文面说明），要么诚实登记"宿主面"。
3. **（G1-2）PhotomodeScreen.enabled 无 producer**：finish_frame/just_enabled 无调用者；DELIVERY 须登记宿主面。
4. **（P-2）PrefabInstanceData 双定义**：systems::prefab 与 components::trees 同字段两类型；建议合并/重导出（涉及 G4 components 域，需仲裁），SpawnPrefabCmd/DespawnPrefabCmd 零消费者登记。
5. **（P-1）PathRastersResources 零消费者**：登记；firstperson_steps 8 参未实现。
6. **（P-3）brush_preview 未注册生产 + register_required_components 空桩**：登记不造；`_ => 1.0` 分支语义标注 I。
7. **（I-5）UiHistoryButtonPressed 生产者缺**：已登记组外；DELIVERY 说明消费面已就位。
8. **M（已解决）**：round4 §7 M1（input/mod.rs doc 写"producer 未落"与代码矛盾）——**现 input/mod.rs doc 已更新为"producer 已落地"，M1 已解决**，无需再修。mod_key.rs 自身注释与代码一致。

## 5. 父结论（给父/中央）

- **G3 域源码总体质量**：数据模型/输入链证据充分、无编造常量；多个"零消费者"项诚实登记（PathRastersResources/firstperson_steps、prefab 命令、brush_preview 注册、GameKeyBindings 消费、UiSettings）。
- **最大的结构性缺口**：GUI 消费系统（egui 面）与 brush_preview 未进生产 Schedule；translate_input 未消费 GameProfile 键位表；mod_key 组 5/6/7 键码与 GameKeyBindings.value 语义仍待①。
- **库面差异需登记**：GUI 用 egui，原版 GUI 栈是 Taffy 0.3.13 + mei + tg-text（toolbar-origin-2026-10-16 报告）——不是"egui 分叉"，后续 GUI 恢复应走 mei/Taffy 面。
- **G3 域与 G4（components）边界**：PrefabInstanceData 双定义牵涉 components 域，需中央仲裁。
- 禁区（rhapsody/roof/wall）未触碰，本审查只登记未评审。
- 地面审查深挖方向 8 条已发 G3①；修复项 7 条已发 G3②；round8 契约/DELIVERY 落盘后补对照复审段。
