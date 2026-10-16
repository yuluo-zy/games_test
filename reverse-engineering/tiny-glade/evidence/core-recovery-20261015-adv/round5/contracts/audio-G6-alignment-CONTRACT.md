# Round5 A CONTRACT — audio G6 对齐裁定（garden_audio 注册点 + G6 6 系统 ↔ crate）

> 逆向 agent ① 裁定（应③/中央请求）。证据：procs.tsv into_configs + add_audio_systems
> 0x1409411a0 反编译 + crate audio/mod.rs 现状。
> 状态：**裁定可直接落地**。

---

## 1. garden_audio_system 注册点 = **audio 组 add_audio_systems（0x1409411a0）**

- garden_audio.rs 属 `systems::audio` 模块（非 systems::garden）；其单独 into_configs
  @0x14166cd50（round4 T1 已证 FunctionSystem 6 参面）。
- native `add_audio_systems` = `systems::audio::impl$1`（procs），构建音频组配置
  （byte tag 0x26=38，`FUN_140b218a0`+`FUN_141453000` → `App::add_systems` 0x140b67280）。
- **garden build 0x140b6d290（16+34 系统）是 systems/garden 插件，不含音频**。
- → **② 待办**：把 `garden_audio_system` 加入 crate `add_audio_systems`（关 F3）。

## 2. G6（0x140b6bd50，6 系统）↔ crate 等价映射（O）

| G6 系统（native 系统面，into_configs 权威） | crate 文件 | 注册现状 |
|---|---|---|
| timberframe_audio `(R<AudioCore>, RM<TimberframeAudioState>, ER<PlayTimberframeSound>, ER<ModeJustChanged>, R<CursorTerrainRaycast>)` | systems/audio/timberframe_audio.rs | 存在未注册 |
| elevation_supports_audio `(ER<PlayElevationSupportSoundCmd>, R<AudioCore>)` | systems/audio/elevation_supports_audio.rs | 存在未注册 |
| clutter_audio `(ER<PlayClutterSoundCmd>, R<AudioCore>)` | systems/audio/clutter_audio.rs | 存在未注册 |
| forest_audio `(RM<ForestAudioState>, Query<(&InteractableForestTree,&PrefabInstanceData)>, R<EscMenuState>)` | systems/audio/forest_audio.rs | 存在未注册 |
| tree_audio `(R<AppMode>, R<AudioCore>, RM<TreeAudioState>, R<CursorTerrainRaycast>, Query<&TreeAmbience>, R<UiSignifierStream>, R<MainCamera>, ER<…>)` | systems/audio/tree_audio.rs | 存在未注册 |
| firstperson_steps `(R<AudioCore>, R<MainCamera>, R<AssetSsboLibrary>, PathRastersResources, RM<FirstPersonViewAudioState>)` | systems/audio/firstperson_steps.rs | 存在未注册 |

→ **G6 6 系统全部已存在于 crate**，均未进生产 Schedule。**② 待办**：按 native 系统面把 6 系统
注册进 `add_audio_systems`（G6 = 环境/即时音频子组）。

## 3. crate 现有 add_audio_systems（music_debug_gui + water×2）归属

- `water_audio_system`（0x140873630）/`water_ambience_audio_system`（0x1409ebe80）**不在 G6**
  （G6 无 water）——属其他音频子组（water 组，待取证精确分组）。
- `music_debug_gui` 属 music 组（0x141736760 候选）。
- crate 现 3 系统 = 局部子集；native add_audio_systems = **全部音频系统**（tag 0x26=38）。
- **② 待办**：add_audio_systems 补入 G6 6 系统 + garden_audio + 其余未注册音频系统
  （path/menu/terrain/terrain_stairs/replay/sheep/ivy/signifier/roof/trigger/decorator/
  playermade_stairs/wall_audio_*/game_loading/weather/building/tree——均已在 crate）。

## 4. 证据索引
- procs.tsv：0x140b6bd50（G6 into_configs 6 系统面）、0x1409411a0（add_audio_systems）、
  0x14166cd50（garden_audio into_configs）。
- round4/decomp-plugin-0x1409411a0.txt（add_audio_systems 配置构建 + 0x140b67280 add_systems）。
- crate：systems/audio/mod.rs L204-216（现 add_audio_systems 只挂 music_debug_gui+water×2）。
