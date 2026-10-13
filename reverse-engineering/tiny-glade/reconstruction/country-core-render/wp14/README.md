# WP14 — country-core audio domain（资源 + 系统 + 启动）

版本 B（ljxsj_92385；PDB sha256 2d9686b8…）。恢复 country-core 音频域游戏自有逻辑，按
PDB 骨架实现 Rust。FMOD/libfmod 为 REUSED 依赖（S18）：只恢复游戏胶水，不重新实现 FMOD。

## 恢复范围

- **状态**：`resources/audio_resources.rs`（AudioCore/AudioCoreInner/AudioInstance/Bank/
  DelayedBankLoader/AudioSamplesLoaded(Promise)/ActiveAudioInstances）、
  `resources/audio_settings.rs`（AudioSettings，RON serde，音量字段偏移 O）。
- **启动**：`startup/load_audio.rs`（银行样例等 + AudioTriggerLibrary 装配）、
  `startup/load_glade_audio.rs`（GladeAudio 预览 glade 装载）。
- **资产库**：`asset_libraries/audio_trigger_library.rs`（触发盒库；wp4 deferred 项）。
- **系统**：`systems/audio/*` 31 模块（事件→FMOD 胶水）+ `music/`（player/system/debug_gui）
  + `systems/wall/snapping/snapping_audio.rs`。
- **工具**：`utils/audio_utils.rs`（AudioLoopDesc/AudioLoop 池化循环播放器）。

## 方法

1. 分析：PDB 骨架（enriched/structures）→ Ghidra 反编译（evidence/audio1-3-result.json，
   worker `wp14-audio`，只读）→ RNE .data 事件/参数串核查（211 个 event/snapshot/vca 串）。
2. 合同：`contracts/{audio-state,audio-systems,audio-trigger}.md`（O/I/U + 生产者/消费者）。
3. 实现：`src/`（51 个 .rs，骨架对齐；未知字段 → `typed_gap::Gap`）。

## 构建门

```sh
cargo check --offline
```
PASSES：0 errors、0 warnings。

## 与相邻 WP 调和

- wp7：`water_audio_system` 消费 `WaterRaster`/`WaterAudioGrid`（24B/格 契约一致）。
- wp9：`roof_audio.rs` 与 wp9 交付函数命名一致。
- wp10：critters FMOD（Duck::new 三事件、ButterflyAudio::set_position）在
  `AudioInstance::set_position` 侧一致；`PlayTreeSoundCmd` 由 tree-tool 写入、tree_audio 消费。

## 关键 FMOD 胶水面（游戏自有侧）

`AudioCore`（+0x10 `AudioCoreInner`）、`AudioCoreInner::instantiate_audio`（0x140a54440）、
`AudioCore::instantiate_loop`（0x140a59220）、`impl10::{set_position,set_parameter,
set_volume,has_parameter}`（0x140a57da0/0x140a57670/0x140a575a0）、`AudioCoreInner::set_parameter`
（0x140a53db0，全局 Studio 参数）、`set_vca_volume`（0x140a54810）、`AudioCoreInner::update`
（0x140a53a00，听者）、`AudioLoop::{start_if_not_playing,stop}`（0x140a596b0/0x140a59430）、
`wall_audio_utils::AudioLoopPlayer::request_play`（0x1409285a0）、`roof_audio::
instantiate_audio_w_position_and_volume`（0x140aa3590）。

## 剩余 gaps

见 `contracts/*.md` 与 `evidence.json` 的 `remaining_gaps`（类型内部布局、逐 kind 串映射、
weather 参数名、trigger 库盒语义等，一律 typed gap，不臆造）。
