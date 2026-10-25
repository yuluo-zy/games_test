# Round6 收官 DELIVERY（2026-10-16 第六轮 ②，可直接落地 5 组全闭合）

> 实现对象：① round6 收官契约（r6-28-system-names / r6-decorators-serde /
> r6-async-savecmd-gamesavedata，累计 9 份）。文件权内。**未碰 rhapsody/roof/wall 禁区**。
> country-core 自身 `cargo check` 零错误（workspace 级 rhapsody vulkan.rs:470 为 D组并发）。

## 真实代码接线（可直接落地 5 组全闭合）

1. **Walls load_state（0x78B 已解）**（`resources/walls/inner_walls.rs`）：
   `InnerWallStateRecord = {WallId u64@0, WallState 0x70B@8}`；O 字段访问器
   height(+0x54 f32)/freehand(+0x60 u8)/arch(+0x64 u8)/removed_supports(+0x67 u8)
   （写闭包 0x14085ad60/aa99e0/aa9eb0/aa9800）；load_state 解码 WallId+4 字段，曲线区待①。
2. **GameKeyBindings 直读**（`game_profile.rs`）：40 槽 × 12B `KeyBinding{key u32@0,
   value u64@4}` `#[repr(C, packed)]`（真 12B，③ 要求）；serde/Debug 手工；default 键码
   capstone 0x14089b110 逐槽；value 动作 id 待①。
3. **Decorators restore（0x28B + 旋转 XOR）**（`systems/save/snapshot.rs`）：
   `DecoratorSectionRecord`（{u32 id@0, 8B×4, u32×2}）+ encoded_id/encoded_u64
   （`u64::from(id).rotate_right(32) ^ mask`，掩码 0x1429cd080 待①=0）+ side_from_state
   公式；parse_decorator_section 组装；restore 逐槽接线待①。
4. **AsyncSaveCmd 判别值**（`systems/save/save_game.rs`）：`#[repr(u64)]` + 显式 3..11
   （3 OpenFile/4 CloseFile/5 ChangeSection/6 ReplaceEdits/7 BackupSave/8 ManualBackup/
   9 CaptureSnapshot/10 Flush/11 Shutdown）。
5. **28-系统逐名 + on_event 应用对象**（`startup/mod.rs` + `input/mod.rs` + `run_game.rs`）：
   - 28-系统名表（G1 10/G2 8/G3·G4 各1/G5·G6 各6）写入 add_game_startup_systems doc
     （宿主接线面；实体由宿主/D组提供）。
   - `reset_input_state.run_if(on_event::<NewSessionStartedCmd>)`（entry6，O）。
   - `input_record_replay`/`time_of_day` `.run_if(resource_exists::<NextSession>)`（entry7，O）。

## candidate 链测试

- 本组文件 lib 类型检查零错误；country-core 自身零错误（排除 rhapsody）。
- on_event/resource_exists 为 bevy_ecs 0.16 common_conditions（on_event condition.rs:886 /
  resource_exists condition.rs:474，编译验证）。

## 原侧验证

- ① r6-28-system-names：procs.tsv into_configs（0x140a2bf40/0x140839b80/0x1415f4750/
  0x1409c8600/0x141736760/0x140b6bd50）+ public-session-names entry6/7。
- ① r6-decorators-serde / r6-async-savecmd / r6-walls-0x78b / r6-gamekeybindings（见各批次）。

## 仍缺项（待①，登记不造，r6-closing-status 交叉）

Decorators JSON serde 逐字段名 + 0x16B 子记录、GameSaveData 240B / session create
96B+32B+24B 字段名、sheep movement 状态机 + X4 Query（0x140a3f470）、Terrain strokes、
ui_clutter AutoClutterRecord 0x58B、ivy count_range RNG 界（spawn_ivy [rbp+0x190]）、
mod_key 表2 键码、camera_rig/weather/time_of_day 深层、快照 5 域写入序、28-tuple 槽位
映射 + 纯 Commands 身份 + Schedule::run 调用点（宿主/D组）。

## 组外申请记录

- 无本轮组外修改。未碰 rhapsody/roof/wall 禁区。
