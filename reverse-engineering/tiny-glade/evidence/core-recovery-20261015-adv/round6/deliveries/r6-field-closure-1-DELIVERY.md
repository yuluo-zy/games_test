# Round6 字段级闭合 DELIVERY（2026-10-16 第六轮 ② 第一批）

> 实现对象：① round6 三契约（r6-walls-0x78b / r6-gamekeybindings-modkey /
> r6-sheep-layout）。文件权内（resources/walls/inner_walls.rs、game_profile.rs、
> resources/critters/sheep.rs + audio G6 补全）。**未碰 roof**。整 crate `cargo check` 零错误。

## 真实代码接线

1. **Walls 0x78B O 字段解码**（`resources/walls/inner_walls.rs`，① r6-walls-0x78b 契约 §1/§2）：
   - `InnerWallStateRecord` = `{WallId u64@0, WallState 0x70B@8}`（HashMap 条目 stride 0x78 权威）。
   - 新增访问器：`wall_state_bytes()`（0x70B）、`height()`（WallState +0x54 f32，
     update_wall_state_height 0x14085ad60）、`freehand_type()`（+0x60 u8，0x140aa99e0）、
     `arch_type()`（+0x64 u8，0x140aa9eb0）、`removed_supports()`（+0x67 u8，0x140aa9800）。
   - `load_state` 逐记录解码 WallId + 四个 O 字段；**曲线区（WallState +0x00..0x53）待①**，
     WallState 完整构造待①——不伪造。
2. **GameKeyBindings 40×12B**（`game_profile.rs`，① r6-gamekeybindings-modkey 契约 §1/§2）：
   - `KeyBinding {key u32@0, value u64@4}`（12B 槽）+ `GameKeyBindings {bindings: [KeyBinding; 40]}`。
   - `default`：40 槽键码（capstone 0x14089b110 逐槽，槽 36=0xc3 未绑、37/38/39 鼠标）；
     `value` 动作 id 待①（默认 0）。
   - serde 经 Vec 中转（`[T;40]` 无标准 serde/Default impl，N>32 限制）；GameProfile ron
     兼容（crate 语义模型）。
3. **sheep 布局确认**（`resources/critters/sheep.rs`，① r6-sheep-layout 契约 §1）：
   - 已证偏移确认：+0x30 move_state（判别 0=Idle/1=Walking、+0x31 force_reach_goal、
     +0x38 timeout_s）、+0x58 being_petted（>0.01）；未知区间不填。
   - movement 状态机（Idle↔Walking 转移 + picking move_state=0x101）与 X4 活羊 Query 待①
     （r6 契约 §2/§3）。
4. **audio G6 补全**（`systems/audio/mod.rs`，① audio-G6-alignment 契约 §3 其余未注册
   音频系统）：补入 roof_audio（init+system）、wall_audio_ui、wall_audio_orchestrator、
   wall_audio_path、ivy_audio_init_system。

## candidate 链测试

- 本组文件在 lib 类型检查零错误；整 crate `cargo check` 零错误。
- GameKeyBindings::default 键码与 BindingsTable（translate_input.rs KEYS）一致（同源
  capstone 0x14089b110 逐槽）。

## 原侧验证

- ① r6-walls-0x78b：map 条目 stride 0x78 章节权威 + 写闭包 0x14085ad60/0x140aa99e0/
  0x140aa9eb0/0x140aa9800。
- ① r6-gamekeybindings-modkey：capstone 0x14089b110 default 构建器逐槽 + 键码表。
- ① r6-sheep-layout：sheep-SPEC §2.1/§2.6 + being_petted 0x14089d4c0。

## 仍缺项（待①，登记不造）

1. Walls 0x78B 曲线区（WallState +0x00..0x53）+ JSON 形状（0x141308b40 采集器 serde）。
2. Decorators JSON↔0x28B/0x16B serde 架构 + Terrain strokes 序列化格式（r6-field-status 登记）。
3. GameKeyBindings value 动作 id 语义、mod_key 表2 键码枚举。
4. sheep movement 状态机 + picking move_state=0x101 + X4 活羊 Query。
5. GameSaveData 240B / AsyncSaveCmd 变体 / session create 96B+32B+24B 字段名、
   ivy count_range RNG 界、send_to_gpu seed/wall_normal、ui_clutter AutoClutterRecord 0x58B、
   5 域写入序、on_event 应用对象（r6-field-status 交叉引用）。

## 组外申请记录

- 无本轮组外修改（内 wall/game_profile/critters 均在②文件权；audio/mod.rs 注册面）。
  未碰 roof（用户重构区）。
