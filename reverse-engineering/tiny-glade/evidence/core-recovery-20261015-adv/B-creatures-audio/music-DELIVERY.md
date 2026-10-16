# B 组 music DELIVERY（2026-10-16 第三轮实现）

> 实现对象：music-DELTA.md 的真实剩余缺口。组：B。文件：`systems/audio/music/*`、
> `systems/audio/mod.rs`（未改，本组）、`resources/audio_resources.rs`（仅 `EventInstance`
> trait 加 `install_beat_callback`，中央 agent 批准）。

## 真实代码接线

1. **`load_glade_audio` producer**（`systems/audio/music/load_glade_audio.rs`，新建）：
   按 0x1408be360 主流程——`player.pause()` + 清 current_track → `GladeAudio::load`
   （`assets/glade/{theme}/audio.json`）→ 拼 4 个 `event:/Music/Halloween/{Single Leaf,
   Tiny Olden Glade, Forgotten Ruins, Enchanted Forest}` → `player.load_tracks` →
   `set_parameter("GladeTypeIsHalloween", audio_id[0,3,1,2,4])` +
   `set_parameter("IsHalloween", 1.0)` → theme 旗标（Winter→0.25 else 0）。
   签名 `(Res<B>, Res<ActiveSession>, ResMut<MusicAudioState<B::Event>>)`，`B: MusicBackend`
   泛型（native 后端是 `Res<AudioCore>`，经 [`audio_core_backend`] 适配实现后端）。
   注册点 `configure_glade_audio`（宿主在 GameStartup 装配；run_game.rs 库外需仲裁/接线）。
2. **`GladeAudio` 72B 结构 + serde + default 回退**（`systems/audio/music/glade_audio.rs`，
   新建）：字段名**由发行资产核实**（2026-10-16 读全部 5 主题
   `assets/glade/{summer,olden,flowery,autumn,winter}/audio.json`）：`ambience` String(24) +
   `music` Vec<String>(24) + `select_glade_stinger` String(24) = 72B = 9 qword
   （0x1408bf460 写 9 qword / default @0x1408bf2c0 回退）。`load` 读文件失败或反序列化
   失败 → default（不 panic，O）。
3. **`impl MusicBackend for AudioCore<S>`**（`systems/audio/music/audio_core_backend.rs`，
   新建）：`instantiate` → `AudioCoreInner::instantiate_audio`（0x140a54440），
   `set_global_parameter` → `AudioCoreInner::set_parameter`（0x140a53db0）。`AudioCoreEvent<E>`
   适配 `MusicEvent`；Disabled→None、Err→panic（`MusicPlayer::load_tracks` "creation errors
   panic" 0x1408b6b10 语义）。**中央 agent 批准**的 `EventInstance::install_beat_callback`
   已加到 `resources/audio_resources.rs`（native.rs `set_callback(TIMELINE_BEAT)` 已存在）。
4. **`MusicPlayer::glade_theme_flag: f32`**（`music_player.rs`）：映射 native `+0x1c0/+0x1c4`
   （load_glade_audio 写 0.25 for Winter），init=0。

## candidate 链测试

- `load_glade_audio.rs`：`load_tracks` 加载 4 个 Halloween 事件名 + 每个实例装 beat 回调
  （load_tracks 只留成功者语义）；audio_id 表 [0,3,1,2,4] 与 Winter 旗标常量。
- `glade_audio.rs`：`size_of::<GladeAudio>()==72`；wire 与发行 olden/audio.json 逐键一致；
  `load` 读临时 audio.json + 缺失目录 default 回退。
- 既有 `music_player` 五状态机 / `native_transition_thresholds_preserve_command_order`
  测试保持通过（本组改动不破坏）。

## 原侧验证

- GladeAudio 字段：发行 `assets/glade/{5 主题}/audio.json` 全量读取，键名/值形态逐一核实
  （此前字段名 U 已闭合）✓。
- 4 事件名 + `GladeTypeIsHalloween`/`IsHalloween` 参数名：rne-read 0x142aa0c33 起串池
  逐字节核对（DELTA 已复核）✓。
- `GladeAudio::load` 流程：decomp-0x1408bf460（读文件 → serde_json → 成功写 9 qword /
  失败 default）✓。
- load_glade_audio 主流程：decomp-0x1408be360（pause → 拼路径 → 4 事件 → load_tracks →
  参数 → 旗标）✓。

## 仍缺项（待取证 / 组外登记）

### 跨组缺口登记：garden_audio_system 距离门 + 音量调制（A 组 garden-DELTA P4.2，2026-10-16 裁定登记）

`systems/audio/garden_audio.rs:57-74` 现为"事件存在→播放"简化折叠（诚实标注）；native
0x140a89480 的完整契约已由本组解码，登记供正确 owner 关闭：

- **门函数** = `utils::float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>`
  （FUN_140a14df0，decomp-0x140a14df0 = 双线性光栅采样；参数含两个 130.0 =
  `0x43020000`，crate 有 `bilinear_sample_at_world_xz_130` 变体）。
- **gate**：`fVar30 >= 0.01`（`_UNK_142960674` = `0ad7233c`，f32≈0.01）→
  GardenDrawWater/EraseWater（native loop@+0x38/+0xa8 = index 1/3）应播；
  `fVar30 < 0.01` → GardenDraw/Erase（loop@+0/+0x70 = index 0/2）。事件计数非零且
  gate 通过才传 `count`，否则传 0。
- **音量**：`fVar34 = [vol_res+0x1c] / 60.0`（`_UNK_1429d18cc` = `00007042` = 60.0），
  当 `AppMode::native_tag() != 0xd`（0xd=Replay，mode_manager/types.rs L305）时；
  `== Replay` → `1.0`（`_UNK_142925904`）。逐循环 `FUN_14166d9f0(loop, should_play, fVar34)`。
- **签名**（native 6 参）：`(Res<AppMode>, ResMut<GardenAudioState>, draws, erases,
  音量资源, 光栅资源@+0x298 buffer 选择旗标)`。
- **阻塞**：音量资源（`*param_5 + 0x1c`）与光栅资源（param_6）身份未取证 → 接线需正确
  owner 确认，本组不猜资源。`garden_audio.rs` 不在 B 组严格所有权（`audio/music/*` +
  `mod.rs`）。

---

- **theme 旗标字段语义**（+0x1c0/+0x1c4 两个 f32 的精确意义）与 `0.25` 常量的业务含义
  （SPEC T3/T4）待取证；当前以单一 `glade_theme_flag` 表达写回行为。
- **逐 track 容器注册**（native 0x14089a280 每 track instantiate+注册细节）I 级，由
  `MusicPlayer::load_tracks` 实例化路径覆盖。
- **`configure_glade_audio` 实际进 GameStartup 调度**：run_game.rs（库外）未接，需宿主
  接线或仲裁。
- **T5 `DelayedBankLoader::load`**（audio_resources.rs）：`pending_bank` 返回 `Some(())`
  薄壳仍登记待取证（未升级，避免无证据改 audio_resources 其它部分）。
- **live FMOD 真机播放**：`native-audio` feature 门控，默认构建不含；真机验收后置。
- `visualize_sounds` 返回空列表（后置，诚实）。
