# Round7 三链 DELIVERY（2026-10-16 第七轮 ②）

> 实现对象：① round7 三链契约（r7-chain1-save-load / r7-chain2-ecs-scheduler /
> r7-chain3-session-switch）。文件权内。**未碰 rhapsody/roof/wall 禁区**。
> country-core 自身 `cargo check` 零错误（workspace 级 rhapsody vulkan.rs:470 为 D组并发）。

## 真实代码接线（可直接落地项，跨轮累计闭合）

**链1（save/load）**：
- AsyncSaveCmd 判别 3..11 显式对齐（r6，repr u64；r7-chain1 确认现源 132-188 行已对齐）。
- Decorators 0x28B + ID 旋转 XOR 解码（r6，掩码 0x1429cd080 待①=0）。
- save_game_to_memory 10 节序 + write_json_sections（r4，与 GameSaveData 契约一致）。
- apply_loaded_snapshot 管线：JSON 节值 → parse_wall/decorator/terrain → 目标 API
  （Walls load_state / Decorators restore 接线 / Terrain inflate，r5/r6）。

**链2（ECS 调度器）**：
- 28-系统逐名（G1 10/G2 8/G3·G4 各1/G5·G6 各6，into_configs 权威）写入 startup/mod.rs
  add_game_startup_systems doc（宿主接线面，r6）。G2 mesh 系统（startup_garden_brush/
  startup_ivy）+ G6 音频 6 域 + 全音频域系统注册于 add_game_startup_systems/
  add_audio_systems（r5/r6）。register_core_startup<B,S> 装配序按 run_game_thread 359
  调用序对齐（r5）。

**链3（会话切换）**：
- 6 消费系统面 + 事件链（r7-chain3 确认可直接落地）：load_game_session 生产
  NewSessionStartedCmd/SessionLoadedCmd → 库内消费端已接线——
  `reset_input_state.run_if(on_event<NewSessionStartedCmd>)`、
  `time_of_day`/`input_record_replay`.run_if(`resource_exists<NextSession>`)
  （r6，public-session-names entry6/7）。
- **库外 3 消费系统**（add_preplaced_autoclutter [system_clutter] /
  detect_overhang_support_segments [system_wall_constructor] /
  sheep_unstuck [system_critters]）宿主注册点契约已登记到 run_game.rs
  （crate 供事件 + 资源，不实现库外系统，r7-chain3 §2）。

## candidate 链测试

- 本组文件 lib 类型检查零错误；country-core 自身零错误（排除 rhapsody）。
- on_event/resource_exists 为 bevy_ecs 0.16 common_conditions（编译验证）。

## 原侧验证

- ① r7 三链契约：r7-chain1（AsyncSaveCmd 判别 132-188 行确认）、r7-chain2（28-系统名 G1-G6）、
  r7-chain3（6 消费系统面 + on_event/resource_exists 条件）。
- 既有交叉：r6-28-system-names、r6-decorators-serde、r6-async-savecmd、r5 a/c 契约。

## 仍缺项（待①，登记不造）

1. 链1：AsyncSaveCmd 载荷字段名（OpenFile path/ChangeSection/ReplaceEdits/BackupSave/
   ManualBackup/CaptureSnapshot Arc）、GameSaveData 240B 逐字段、Decorators JSON/0x16B
   逐槽、Terrain strokes、5 域写入序（Roofs/WallColors/GateColors/Trees/Raster）。
2. 链2：28-tuple 槽位 fn_ptr 映射、GameLoop::run（0x1404d8250）内 Schedule.run 调用点、
   纯 Commands 系统身份（低优先，宿主/D组）。
3. 链3：库外 3 消费系统的宿主注册（country-core 只供事件+资源，不实现库外系统）。
4. 其余（跨轮待①）：sheep movement+X4、ui_clutter 0x58B、ivy count_range、mod_key 表2、
   camera_rig/weather/time_of_day 深层。

## 组外申请记录

- 无本轮组外修改。未碰 rhapsody/roof/wall 禁区。
