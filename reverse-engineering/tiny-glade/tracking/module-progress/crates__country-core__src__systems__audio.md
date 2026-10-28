# crates/country-core/src/systems/audio

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

16事件注册；wall/building/stairs/music/menu/sheep/clutter/water局部事件映射。

## 接线与验证

接线：not_established。

clutter+audio候选check/build/test5/5；非FMOD运行或原作完整音频等价。

## 已恢复的函数和类型范围

- add_audio_events
- wall/building/stairs kind→FMOD参数
- music_audio_system richness/time规则
- update/listener/volume候选

## 明确缺口

- FMOD资源类型/实例和真实后端接线
- 部分kind、资源字段和环境音公式未钉死；占位不得计恢复

下一闭包：FMOD资源类型/实例和真实后端接线

## 证据

- [evidence/multi-baseline/compile-clutter-audio.json](../../evidence/multi-baseline/compile-clutter-audio.json)
- [evidence/multi-baseline/audio-events-analysis.md](../../evidence/multi-baseline/audio-events-analysis.md)
- [evidence/multi-baseline/audio-extra-analysis.md](../../evidence/multi-baseline/audio-extra-analysis.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/country-core/src/systems/audio/audio_ui.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/audio_update.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/building_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/clutter_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/decorator_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/elevation_supports_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/firstperson_steps.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/forest_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/game_loading_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/garden_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/ivy_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/menu_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/mod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/music/music_debug_gui.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/music/music_player.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/music/music_system.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/music_audio.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/music_fade_in_on_start.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/path_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/playermade_stairs_audio.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/replay_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/roof_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/sheep_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/signifier_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/sound_style.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/stairs_audio.rs` | A | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/terrain_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/terrain_stairs_audio.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/timberframe_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/tree_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/trigger_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_audio_fmod.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_audio_orchestrator.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_audio_path.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_audio_ui.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_audio_utils.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/wall_group_move_audio.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/water_ambience_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/water_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/country-core/src/systems/audio/weather_audio.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/audio/src/events.rs

[reconstruction/audio/src/events.rs](../../reconstruction/audio/src/events.rs)

SHA256：`1809ccbeae639ec3c2973fea1ce4a27541dce4cbe66afc862e2e50ad122e2f33`；缺口注释标记数：9。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `kind_is_loop` | 35 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：//! - 未确认字段保留 `unknown`/`_private`，不伪造。
- L15：/// - f32 @+0xc（部分瞬发 case 读取；unknown）
- L22：/// f32 @+0xc（unknown 语义）。
- L28：/// @+0x20 字节（unknown）。
- L72：/// f32 @+0xc（unknown，部分 case 读取）。
- L79：/// f32 @+0x28（unknown）。
- L81：/// f32 @+0x2c（unknown）。
- L110：/// 元素参数缓冲区（unknown 内部布局）。
- L117：/// 内部布局 largely unknown（事件系统按 id 查 0x2a8 条目，不常驻状态）。
### reconstruction/audio/src/music.rs

[reconstruction/audio/src/music.rs](../../reconstruction/audio/src/music.rs)

SHA256：`0f5e8699b7cd8108a0481b57918baabb7c4ad85c70e4327c3d67a7d7de4beae4`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compute_music_richness` | 24 | 未由扫描证明；以证据限定范围为准 |
| `snap_off_boundaries` | 44 | 未由扫描证明；以证据限定范围为准 |
| `compute_new_glade_time` | 61 | 未由扫描证明；以证据限定范围为准 |
| `music_audio_system_step` | 83 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/audio/src/stairs.rs

[reconstruction/audio/src/stairs.rs](../../reconstruction/audio/src/stairs.rs)

SHA256：`fa335b1d9be70f0061b96fe89f3af26a5d1aed3306ac56bc0939e1ddbd776c0e`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `stair_kind` | 47 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_system` | 67 | 未由扫描证明；以证据限定范围为准 |
| `terrain_stairs_audio_system` | 99 | 未由扫描证明；以证据限定范围为准 |
| `stair_position` | 115 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L104：_suppressed: bool,       // [evidence] SuppressPathSound 事件（unknown 精确字段）
### reconstruction/audio/src/update.rs

[reconstruction/audio/src/update.rs](../../reconstruction/audio/src/update.rs)

SHA256：`3f66434d777605ace798931a3390037cb74affe2b4cc41dc8dee57981802ed38`；缺口注释标记数：4。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_audio_events` | 23 | 未由扫描证明；以证据限定范围为准 |
| `add_audio_systems` | 47 | 未由扫描证明；以证据限定范围为准 |
| `music_debug_gui` | 57 | 未由扫描证明；以证据限定范围为准 |
| `water_audio_system` | 58 | 未由扫描证明；以证据限定范围为准 |
| `water_ambience_audio_system` | 59 | 未由扫描证明；以证据限定范围为准 |
| `water_audio_condition` | 60 | 未由扫描证明；以证据限定范围为准 |
| `water_ambience_condition` | 63 | 未由扫描证明；以证据限定范围为准 |
| `audio_update_system` | 81 | 未由扫描证明；以证据限定范围为准 |
| `boot_audio_update_system` | 100 | 未由扫描证明；以证据限定范围为准 |
| `wall_audio_init_system` | 115 | 未由扫描证明；以证据限定范围为准 |
| `wall_audio_fmod` | 142 | 未由扫描证明；以证据限定范围为准 |
| `building_audio` | 179 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_init_system` | 204 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_system` | 225 | 未由扫描证明；以证据限定范围为准 |
| `set_vca_volume` | 254 | 未由扫描证明；以证据限定范围为准 |
| `audio_core_update` | 255 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_loop` | 256 | 未由扫描证明；以证据限定范围为准 |
| `request_play` | 257 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_audio_w_position_and_volume` | 258 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_audio` | 262 | 未由扫描证明；以证据限定范围为准 |
| `set_position` | 265 | 未由扫描证明；以证据限定范围为准 |
| `set_parameter` | 266 | 未由扫描证明；以证据限定范围为准 |
| `start_if_not_playing` | 267 | 未由扫描证明；以证据限定范围为准 |
| `stop_loop` | 268 | 未由扫描证明；以证据限定范围为准 |
| `normalized_volume` | 272 | 未由扫描证明；以证据限定范围为准 |
| `compute_listener_position` | 275 | 未由扫描证明；以证据限定范围为准 |
| `loop_index_for_kind` | 278 | 未由扫描证明；以证据限定范围为准 |
| `wall_event_name` | 281 | 未由扫描证明；以证据限定范围为准 |
| `building_lookup` | 284 | 未由扫描证明；以证据限定范围为准 |
| `building_event_name` | 287 | 未由扫描证明；以证据限定范围为准 |
| `building_size` | 290 | 未由扫描证明；以证据限定范围为准 |
| `stair_event_name` | 293 | 未由扫描证明；以证据限定范围为准 |
| `stair_width_param` | 296 | 未由扫描证明；以证据限定范围为准 |
| `stair_size_param` | 299 | 未由扫描证明；以证据限定范围为准 |
| `volume_modulate` | 302 | 未由扫描证明；以证据限定范围为准 |
| `audio_core_from` | 305 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L46：/// 其余音频系统（wall/building/stairs/…）由其他模块注册（unknown，见 analysis §9）。
- L83：listener_transform: Res<[u8; 0]>, // unknown：+0xc0/+0xd0/+0xe0 变换矩阵
- L141：/// [0x1429d18cc]），AppMode==13 时音量=1.0 常量。全量参数清单待钉死（unknown）。
- L270：// ---- [evidence] 辅助（证据位，语义 unknown）----
### reconstruction/audio/src/wall_kinds.rs

[reconstruction/audio/src/wall_kinds.rs](../../reconstruction/audio/src/wall_kinds.rs)

SHA256：`686152fc8263a76f41dbc30595339c72c69f265ccc96620b8eca3050b0071bb1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_kind` | 85 | 未由扫描证明；以证据限定范围为准 |
| `play_wall_kind` | 92 | 未由扫描证明；以证据限定范围为准 |
| `wall_frame_tail` | 133 | 未由扫描证明；以证据限定范围为准 |
| `wall_event_position` | 139 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/events.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/events.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/events.rs)

SHA256：`a0b215c09dc458e75308768e51e3c2570bc5360e7bee79f0edb4b326b60d3c53`；缺口注释标记数：9。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `kind_is_loop` | 37 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：//! - 未确认字段保留 `unknown`/`_private`，不伪造。
- L16：/// - f32 @+0xc（部分瞬发 case 读取；unknown）
- L24：/// f32 @+0xc（unknown 语义）。
- L30：/// @+0x20 字节（unknown）。
- L76：/// f32 @+0xc（unknown，部分 case 读取）。
- L83：/// f32 @+0x28（unknown）。
- L85：/// f32 @+0x2c（unknown）。
- L144：/// 元素参数缓冲区（unknown 内部布局）。
- L151：/// 内部布局 largely unknown（事件系统按 id 查 0x2a8 条目，不常驻状态）。
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/mod.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/mod.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/mod.rs)

SHA256：`0e973c2c225050cfc9cf5211b63b8634e0c813978affb1c78e4f4ac98d7bbf94`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/music.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/music.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/music.rs)

SHA256：`41fd125fc3a7a7a6008287b2f3185a5e6e7796b97d81481ffe993cdc10208d1b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compute_music_richness` | 24 | 未由扫描证明；以证据限定范围为准 |
| `snap_off_boundaries` | 45 | 未由扫描证明；以证据限定范围为准 |
| `compute_new_glade_time` | 62 | 未由扫描证明；以证据限定范围为准 |
| `music_audio_system_step` | 84 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/stairs.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/stairs.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/stairs.rs)

SHA256：`55f2d0d676e718b4a4c9f3723b2a89054afcabcecfe0baffbb0ead17afd7ff57`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `stair_kind` | 47 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_system` | 67 | 未由扫描证明；以证据限定范围为准 |
| `terrain_stairs_audio_system` | 99 | 未由扫描证明；以证据限定范围为准 |
| `stair_position` | 115 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L104：_suppressed: bool,       // [evidence] SuppressPathSound 事件（unknown 精确字段）
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/update.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/update.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/update.rs)

SHA256：`23a9de4cdb117390648866937649f804b60df4a8d6f301798c479f0916487dcb`；缺口注释标记数：8。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `add_audio_events` | 48 | 未由扫描证明；以证据限定范围为准 |
| `add_audio_systems` | 72 | 未由扫描证明；以证据限定范围为准 |
| `music_debug_gui` | 82 | 未由扫描证明；以证据限定范围为准 |
| `water_audio_system` | 83 | 未由扫描证明；以证据限定范围为准 |
| `water_ambience_audio_system` | 84 | 未由扫描证明；以证据限定范围为准 |
| `water_audio_condition` | 85 | 未由扫描证明；以证据限定范围为准 |
| `water_ambience_condition` | 88 | 未由扫描证明；以证据限定范围为准 |
| `audio_update_system` | 107 | 未由扫描证明；以证据限定范围为准 |
| `boot_audio_update_system` | 126 | 未由扫描证明；以证据限定范围为准 |
| `wall_audio_init_system` | 141 | 未由扫描证明；以证据限定范围为准 |
| `wall_audio_fmod` | 168 | 未由扫描证明；以证据限定范围为准 |
| `building_audio` | 205 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_init_system` | 230 | 未由扫描证明；以证据限定范围为准 |
| `stairs_audio_system` | 251 | 未由扫描证明；以证据限定范围为准 |
| `set_vca_volume` | 282 | 未由扫描证明；以证据限定范围为准 |
| `audio_core_update` | 283 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_loop` | 284 | 未由扫描证明；以证据限定范围为准 |
| `request_play` | 285 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_audio_w_position_and_volume` | 286 | 未由扫描证明；以证据限定范围为准 |
| `instantiate_audio` | 290 | 未由扫描证明；以证据限定范围为准 |
| `set_position` | 293 | 未由扫描证明；以证据限定范围为准 |
| `set_parameter` | 294 | 未由扫描证明；以证据限定范围为准 |
| `start_if_not_playing` | 295 | 未由扫描证明；以证据限定范围为准 |
| `stop_loop` | 296 | 未由扫描证明；以证据限定范围为准 |
| `normalized_volume` | 300 | 未由扫描证明；以证据限定范围为准 |
| `compute_listener_position` | 303 | 未由扫描证明；以证据限定范围为准 |
| `loop_index_for_kind` | 306 | 未由扫描证明；以证据限定范围为准 |
| `wall_event_name` | 309 | 未由扫描证明；以证据限定范围为准 |
| `building_lookup` | 312 | 未由扫描证明；以证据限定范围为准 |
| `building_event_name` | 315 | 未由扫描证明；以证据限定范围为准 |
| `building_size` | 318 | 未由扫描证明；以证据限定范围为准 |
| `stair_event_name` | 321 | 未由扫描证明；以证据限定范围为准 |
| `stair_width_param` | 324 | 未由扫描证明；以证据限定范围为准 |
| `stair_size_param` | 327 | 未由扫描证明；以证据限定范围为准 |
| `volume_modulate` | 330 | 未由扫描证明；以证据限定范围为准 |
| `audio_core_from` | 333 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L7：//! 原草稿以 `Res<[u8;0]>` 标记听者/音量源/构建表/元素表的 unknown 资源身份；`[u8;0]` 不满足
- L9：//! 语义仍 unknown，不伪造。
- L20：// ===== glue：原草稿 `Res<[u8;0]>` 占位资源的具名化（语义 unknown）=====
- L26：/// audio_update_system arg2 听者变换（[evidence] +0xc0/+0xd0/+0xe0 3×3+偏移；布局 unknown）。
- L38：/// stairs_audio_system arg3 元素表（unknown 精确类型）。
- L71：/// 其余音频系统（wall/building/stairs/…）由其他模块注册（unknown，见 analysis §9）。
- L167：/// [0x1429d18cc]），AppMode==13 时音量=1.0 常量。全量参数清单待钉死（unknown）。
- L298：// ---- [evidence] 辅助（证据位，语义 unknown）----
### reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/wall_kinds.rs

[reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/wall_kinds.rs](../../reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/wall_kinds.rs)

SHA256：`09ac0d86bf0f86fdc30b114fe47b53210457124a693b6d399898d49d93d37def`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wall_kind` | 85 | 未由扫描证明；以证据限定范围为准 |
| `play_wall_kind` | 92 | 未由扫描证明；以证据限定范围为准 |
| `wall_frame_tail` | 133 | 未由扫描证明；以证据限定范围为准 |
| `wall_event_position` | 139 | 未由扫描证明；以证据限定范围为准 |
