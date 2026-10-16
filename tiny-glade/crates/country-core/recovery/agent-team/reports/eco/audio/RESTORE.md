# restorer-eco — 卡F audio 域 RESTORE 报告（第一+二批）

日期：2026-10-16
状态：in_review（第一+二批：audio_resources 主闭包 + audio_update/ivy_audio 落地；剩余 32 个 audio 系统文件登记批量续派）
涉及文件：`resources/audio_resources.rs`（新建）、`resources/mod.rs`、`systems/audio/audio_update.rs`（新建）、`systems/audio/ivy_audio.rs`（新建）、`systems/audio/mod.rs`

## 一、原函数 → Rust 位置 → 证据

| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| AudioInstance::start | 0x140a57240 | audio_resources.rs（AudioInstance::start） | 章节 audio_resources.rs.md |
| AudioInstance::start_if_not_playing | 0x140a57300 | audio_resources.rs（位掩码 0xb） | decomp-0x140a57300 |
| AudioInstance::stop / stop_on_drop / is_playing | 0x140a57470 / 0x140a57540 / 0x140a57550 | audio_resources.rs | 章节 |
| impl$10::set_volume / set_parameter / set_position | 0x140a575a0 / 0x140a57670 / 0x140a57da0 | audio_resources.rs | decomp |
| Bank::load / load_sample_data | 0x140a53640 / 0x140a53870 | audio_resources.rs | decomp |
| AudioCore::new | 0x140a54a40 | audio_resources.rs（Studio 注入+扫描） | 章节 |
| AudioCoreInner::update / set_vca_volume | 0x140a53a00 / 0x140a54810 | audio_resources.rs（update/set_vca_volume/ListenerAttributes） | 章节 |
| audio_update_system | 0x1409c8dd0 | audio_update.rs:62（5×VCA+相机听者+update） | decomp-0x1409c8dd0 + 章节 |
| boot_audio_update_system | 0x1409c9320 | audio_update.rs:82（固定默认听者） | decomp-0x1409c9320 + 章节 |
| ivy_audio_init_system | 0x140ad5350 | ivy_audio.rs:29（event:/SFX/IvyGrow） | decomp-0x140ad5350 + 章节 |

## 二、实现说明
- AudioInstance 判别（Disabled/Active/Err）Rust enum；Active 臂 {event@+8, name@+0x10, state u8@+0x28}。
- FMOD 经注入 EventInstance/Studio trait（同 LoopInstance 模式），真实绑定=libfmod（库外）。
- audio_update：5 VCA（vca:/Master /vca:/Music /vca:/Music-DawnJingle /vca:/SFX /vca:/Amb）音量+相机听者（旋转@+0xc0/平移@+0xf0→forward(0,0,-1)/up(0,1,0)/vel0）→ AudioCore::update；boot 变体固定默认听者。AudioSettings 资源（5 音量字段）。
- ivy_audio：单瞬发（event:/SFX/IvyGrow），growth 事件→start_if_not_playing。
- 测试：audio 域 39 全过（含 audio_resources 3 + audio_update 2）。

## 三、仍缺失/待取证（登记，不伪造）
1. **剩余 32 个 audio 系统文件**：terrain_audio/clutter_audio(39 kind 跳表)/garden_audio(4 循环)/decorator_audio/path_audio/roof_audio/tree_audio/water_audio/weather_audio/wall_audio_*/signifier_audio/timberframe_audio/firstperson_steps/forest_audio/game_loading_audio/menu_audio/trigger_audio/building_audio/elevation_supports_audio/playermade_stairs_audio/replay_audio——登记批量续派（本批上下文已尽）。
2. **AudioCore 其余**：instantiate_audio（create_instance→Active 布局）、AudioSamplesLoadedPromise::recv（crossbeam 三通道 1s 超时）、DelayedBankLoader、set_master/music/sfx/ambience_volume、visualize_sounds——续批。
3. **clutter 39 项 kind→event 名表**：抽查名已证（DoorBell/BoxFish/RackBrooms/Veggiestall/ClothlineAppear/Box 等），全表待续批 rne-read。
4. **music native.rs/FMOD 接线**：库外，登记。
5. **AudioSettings 原生布局**：5 音量字段偏移已证（+0xf8/+0xfc/+0x100/+0x104），完整 struct 待取证。

## 一、原函数 → Rust 位置 → 证据

| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| AudioInstance::start | 0x140a57240 | audio_resources.rs:106（AudioInstance::start） | 章节 audio_resources.rs.md |
| AudioInstance::start_if_not_playing | 0x140a57300 | audio_resources.rs:117 | decomp-0x140a57300（位掩码 0xb） |
| AudioInstance::stop | 0x140a57470 | audio_resources.rs:132（STOP_IMMEDIATE） | 章节 |
| AudioInstance::stop_on_drop | 0x140a57540 | audio_resources.rs:167（Drop impl） | decomp-0x140a57540 |
| AudioInstance::is_playing | 0x140a57550 | audio_resources.rs:140 | 章节 |
| impl$10::set_volume | 0x140a575a0 | audio_resources.rs:146 | 章节 |
| impl$10::set_parameter | 0x140a57670 | audio_resources.rs:153（PDB 名 pause 错位） | decomp-0x140a57670 |
| impl$10::set_position | 0x140a57da0 | audio_resources.rs:163（forward(0,0,-1)/up(0,1,0)/vel 0） | decomp-0x140a57da0 |
| Bank::load | 0x140a53640 | audio_resources.rs:185（panic 文案 {path} {err}） | decomp-0x140a53640 |
| Bank::load_sample_data | 0x140a53870 | audio_resources.rs:197 | decomp-0x140a53870 |
| AudioCore::new | 0x140a54a40 | audio_resources.rs:216（Studio 注入+扫描 assets/audio/Desktop .bank+逐行 load） | 章节 |

## 二、实现说明
- **AudioInstance 判别 u64@+0**（0=Disabled/1=Active/2=Err）以 Rust enum 建模；Active 臂 {event@+8, name@+0x10, state u8@+0x28}。
- FMOD EventInstance/Studio 调用经注入 trait（EventInstance/Studio，同 audio_instance::LoopInstance 模式）；真实绑定=游戏 libfmod（库外）。
- start_if_not_playing 位掩码 0xb（{Playing,Sustaining,Starting} 跳过）复用 audio_instance::should_start_if_not_playing。
- Bank::load loaded&1 早退 + panic 文案逐字。
- 3 测试过（Disabled 不转发/start_if_not_playing 跳过集/Bank load）。

## 三、仍缺失/待取证（登记，不伪造）
1. **34 个 audio 系统文件**（inventory：clutter_audio/terrain_audio/ivy_audio/garden_audio/terrain_stairs_audio/sheep_audio/decorator_audio/path_audio/roof_audio/tree_audio/water_audio/weather_audio/wall_audio_*/signifier_audio/timberframe_audio/firstperson_steps/forest_audio/game_loading_audio/menu_audio/trigger_audio/audio_update/building_audio/elevation_supports_audio/playermade_stairs_audio/replay_audio/menu_audio 等）——**登记批量续派**（本批上下文已尽）。
2. **AudioCore 其余方法**：update/set_parameter/set_vca_volume/instantiate_audio/各 set_*_volume（内联族）与 AudioSamplesLoadedPromise::recv（crossbeam 三通道）、DelayedBankLoader——部分依赖 FMOD Studio 绑定，续批。
3. **music native.rs/FMOD 接线**：库外，登记。
4. **Unresolved**：AudioInstance 完整字段（name/state 语义）、Bank 句柄@+8 表示。

## 四、inventory 建议
- `resources/audio_resources.rs` 从 `no_formal_mapping` 提升为 `present_requires_semantic_review`。
- 34 个 audio 系统文件维持 `no_formal_mapping`，待批量续派。
