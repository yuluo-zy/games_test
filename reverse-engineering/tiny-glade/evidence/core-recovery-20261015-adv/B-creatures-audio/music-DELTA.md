# B 组 music DELTA（2026-10-16 第三轮，核对用户批改真实性）

> 核对对象：`D:/game/tiny-glade/crates/country-core/src/systems/audio/{music/*, mod.rs, sheep_audio.rs}` +
> `resources/audio_resources.rs` + `startup/load_glade_audio.rs`。
> SPEC：`music-SPEC.md`（2026-10-15 第二轮）。逐项对照当前源码与原始机码。
> 原侧抽查：`rne-read.py --hex` 复核常量/串池。
> 说明：music_player/native/audio_resources 在 f510c7e7 未动（git log 只到 fa337fd4/6845e43f）；该 commit 只改
> `systems/audio/mod.rs`（PlaySheepReactSound +pitch、PlayClutterSoundCmd +context）与 sheep_audio.rs（FirstPersonOff→Pitch）。

| SPEC 项 | 当前状态 | 证据（源码位置 / 原地址） | 备注 |
|---|---|---|---|
| music_player 五状态机（SwitchTrack/Playing/PlayingLoopedOnce/SwitchAtNextOpportunity/Switching） | **真实实现** | `systems/audio/music/music_player.rs` L87-265；测试 `native_transition_thresholds_preserve_command_order` L389-464 | 过渡阈值 1.5/18/18.5/19/0.2s 与概率 elapsed/120 有测试锁定；`get_timeline_beat_properties` L304 原子回调 |
| `load_tracks`（0x1408b6b10） | **真实实现** | `music_player.rs` L127-139 | 失败实例丢弃、只留 install_beat_callback 成功者；不 panic |
| `MusicBackend` / `MusicEvent` trait | **真实实现** | `music_player.rs` L12-34 | 泛型中间抽象在位；spec §2.4"须让 AudioCore 实现该接口"**未做**（见下） |
| native.rs libfmod 2.222.6 适配（非模拟） | **真实实现** | `music/native.rs` 整文件；`Cargo.toml` `libfmod = "=2.222.6"` optional + `native-audio` feature；Cargo.lock checksum `75defbe5991c…`（=SPEC 声明） | 真实 `get_event→create_instance`、`set_parameter_by_name`、`set_callback(TIMELINE_BEAT)`；`FmodMusicEvent::drop` release；非 mock 播放。仅 feature-gated，默认构建不含 |
| MusicAudioState / music_audio_system（0x140a71f20 / 0x140a72810） | **真实实现** | `music_system.rs` L16-179；`configure_music_consumers` L54-65 | 签名（AppMode/菜单/session/debug/GameTime+Time/EscMenu/Photomode）对齐 PDB；`session.meta.glade` 读 ActiveSession |
| 参数 `MusicRichness`/`NewGladeTime`/`PreviewGladeType` 策略 | **真实实现** | `music_system.rs` L119-178；`preview_glade_parameter` L68 | `GladeType::audio_id()` = [0,3,1,2,4]（`resources/session.rs` L78）与 spec 表 @0x142aa152c 一致 |
| §2.2 GladeAudio track producer（load_glade_audio @0x1408be360） | **仍缺** | 全 crate 无 `load_glade_audio` 系统、无 `GladeAudio` 结构（grep 无命中）；`startup/load_glade_audio.rs` 头注释明写"**未做：load_glade_audio（0x1408be360）本体**" | 只落地常量：4 个 Halloween 事件名 + 参数名 + `audio.json` 路径（串池已复核，见下）；pause→theme→拼事件→load_tracks→instantiate→set_parameter 的 producer 闭环未实现 |
| §2.3 AudioCore / Bank / AudioInstance 启动 | **真实实现** | `resources/audio_resources.rs`（690 行）；`AudioCore::new` L347、`Bank::load` L282、`AudioInstance`（判别 0/1/2）L60-170、`Studio` trait L300 | `AudioCoreInner::update/set_parameter/instantiate_audio` 语义在 `AudioCore::update/set_parameter/instantiate_audio`（L389/436/403）；Bank::load panic 文案、`loaded&1` 早退、`AudioSamplesLoadedPromise::recv` recv_timeout(1s) 均对齐 |
| §2.4 方案 A：`impl MusicBackend for AudioCore` | **待取证（未做）** | grep `MusicBackend for` 只有 `FmodMusicBackend`（native.rs）与测试 Backend | 后端未收敛到 `Res<AudioCore>`；现经泛型 `B: MusicBackend` + 宿主持有 FmodMusicBackend 注入。架构诚实但与原"Res\<AudioCore\> 后端"不同 |
| 常量/串池原侧复核 | **真实实现（已验证）** | rne-read `0x142aa0c33`="event:/Music/Halloween/Single Leaf…"、`0x142aa0ce8`="GladeTypeIsHalloween"、`0x142aa0d44`="audio.json" | 4 事件名与参数名逐字节对得上 |
| sheep_audio（FirstPersonOff→Pitch 纠偏，f510c7e7） | **真实实现** | `sheep_audio.rs`；7 one-shot 名 + Baah kind4 loop + `Pitch` 参数 | diff 确认把误读 `FirstPersonOff` 改为 `Pitch`（decomp 0x1409d6166）；`PlaySheepReactSound{pitch}` 在 mod.rs 定案 |
| `add_audio_events`（16 事件） / `add_audio_systems` | **真实实现（库侧入口）** | `audio/mod.rs` L185-221 | 16 事件顺序对齐 0x140941450；无 crate 内调用方（宿主接线），属库外接线登记 |

## 合成 agent 应关闭的真实剩余缺口（music）

1. **`load_glade_audio` producer 系统**（0x1408be360）：`(Res<AudioCore>|backend, Res<ActiveSession>, ResMut<MusicAudioState>)`——pause → 读 ActiveSession theme → 拼 `assets/glade/{theme}` → `load_tracks`（4 个 Halloween 事件名已就绪）→ 逐 track instantiate/注册 → `set_parameter("GladeTypeIsHalloween", table[theme])` / `set_parameter("IsHalloween", 1.0|0.0)` → theme 旗标 0.25。需接入调度（现无调用方）。
2. **`GladeAudio` 72B 结构 + Deserialize + default 回退**（0x1408bf460）：字段名依赖发行 `assets/glade/{theme}/audio.json` 或 serde visitor 串表（SPEC T2），落地前取证。
3. **后端收敛**：spec §2.4 方案 A `impl MusicBackend for AudioCore`（Inner.instantiate_audio / Inner.set_parameter 映射），或登记维持 FmodMusicBackend 泛型注入差异。
4. **T3 player 内 theme 旗标偏移**（puStack_58[0x70]/[0x71] = 0x1c0/0x1c4）与 **T5 DelayedBankLoader::load 构造**：现 `DelayedBankLoader::load` 的 `pending_bank` 返回 `Some(())` 是薄壳（有水分边缘，已标待取证）——合成 agent 需决定是否升级或明确登记。
5. 保持：FMOD 引擎实现复用依赖不重写；`visualize_sounds` 返回空列表（后置，诚实）。
