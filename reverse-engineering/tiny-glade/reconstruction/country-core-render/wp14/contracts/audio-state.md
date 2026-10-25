# WP14 · country-core audio domain — contract 1: audio state & resources

范围：`resources/audio_resources.rs`、`resources/audio_settings.rs`、`startup/load_audio.rs`、
`startup/load_glade_audio.rs`、`utils/audio_utils.rs`、`systems/audio/music/music_player.rs`
（音频状态类型侧）。版本 B（ljxsj_92385；PDB sha256 2d9686b8…）。

记法：**O**=observed（反汇编/PDB 骨架直接证据）；**U**=unrecovered（typed gap）；
**I**=inferred 但未逐字节证实。F（字段名）= 推断，带证据。

## 1. AudioCore / AudioCoreInner（O：结构，U：内部字段）

- `AudioCore`：Resource 包装。`AudioCore::update`（audio_update_system）经 `core[+0x10]`
  取 `AudioCoreInner`；`AudioCore::instantiate_audio`（0x140a55f40）判 `core[+0]` 为 0 时
  走无内核路径。→ 布局：`AudioCore { _tag: Gap @0, inner: Option<Arc<AudioCoreInner>> @0x10 }`（I）。
- `AudioCoreInner`：持有 FMOD Studio/Core System 与银行表。证据（`AudioCore::new` 0x140a54a40，
  内含 `AudioCoreInner::new_impl` 行 101）：
  - `Studio::create()` → `Studio::get_core_system()` → `System::set_software_format()`
    （记录 `Mixer buffer length/count/average latency` 日志）→ `Studio::initialize<u32,u32>()`。
  - **银行加载**：`ReadDir` 过滤 `impl$6::new_impl::closure$0`（过滤 `.bank`）→
    `HashMap<String, Bank>::extend` → 每银行 `Bank::load()` + `Bank::load_sample_data()`。
    银行目录串（RNE .data）：`assets/audio/Desktop`；银行名 `Master.strings.bank / Master.bank /
    Master.assets.bank / UI.bank / UI.assets.bank`。
  - `set_vca_volume(name, vol)`（0x140a54810）：`Studio::get_vca` → `Vca::set_volume`。
  - `set_parameter(name, value)`（0x140a53db0）：`Studio::set_parameter_by_name`（全局参数）；
    失败经 `anyhow::Context`（串 `Couldn't set global FMOD paramet…`）并写入全局 panic
    BTreeMap 报告（格式串 `"module_p_<name>: Level: …"`，O 0x140a53db0 尾部）。
  - `update(listener)`（0x140a53a00）：校验 `position`/`forward` 单位正交（否则 panic），
    `Studio::set_listener_attributes` → `Studio::update`；失败 log `FMOD update error`。
  - `instantiate_audio(&self, name:&str) -> AudioInstance`（0x140a54440）：
    `Studio::get_event(name)` → `EventDescription::create_instance()`；Ok 变体装填
    `{event: EventInstance, name: String, state: u8=2}`；Err 变体 = anyhow（串 `Failed to…`）。

## 2. AudioInstance（O 枚举判别）

- 判别式证据：`AudioCore::instantiate_audio` 无内核路径写 `*rcx=0`；`AudioCoreInner::
  instantiate_audio` 成功写 `1`、失败写 `2`。方法 `start/stop/start_if_not_playing/…
  `（0x140a57240/0x140a57470/0x140a57300/…）均以 `(*rcx & 1)!=0` 才执行 libfmod 调用 →
  判别：`0=Disabled{name:String,state:u8}`（O 布局，语义 I）、`1=Ok{event:EventInstance,
  name:String, state:u8}`、`2=Err(anyhow::Error)`。
- 方法（全部 libfmod 边界，只标注不复原）：
  `start`、`stop`、`pause`、`resume`、`is_paused`、`is_playing`（get_playback_state）、
  `get_timeline_position`、`stop_immediately`、`stop_on_drop`（Drop：`*rcx==1` 时 stop）、
  `start_if_not_playing`（get_playback_state → 未播则 start）。
- `impl$10`（`impl AudioInstance`）：`set_volume(f32)`（0x140a575a0）、
  `set_parameter(&str,f32)`（0x140a57670，`EventInstance::set_parameter_by_name`；失败串
  `Failed to set parameter named … on`）、`set_position(Vec3)`（0x140a57da0，`EventInstance::
  set_3d_attributes`，forward=(0,0,-1)、up=(0,1,0)、vel=0；失败串 `Failed to set 3d attributes for F…`）、
  `has_parameter(&str)->bool`（行 621，O 名）。
- `ActiveAudioInstances`（O 名）：`Vec<AudioInstance>` 活动实例集合（terrain_stairs 经
  `Vec::retain(closure: is_playing)` 清理，O 0x140a57550 调用点）。

## 3. Bank / DelayedBankLoader / AudioSamplesLoaded(Promise) / AsyncLoadedAudioResources

- `Bank`：`{loaded: bool @0, path: PathBuf @0x10, bank: FMOD Bank}`（I）；`load()`（0x140a53640）
  `Studio::load_bank_file(path)`、`load_sample_data()`（0x140a53870，`Bank::load_sample_data`；
  日志 `Loading sample data of FMOD bank`）。
- `DelayedBankLoader`：`load()`（行 64）——异步银行加载线程任务。
- `AudioSamplesLoaded` / `AudioSamplesLoadedPromise`：跨线程 mpmc 通道（array/list/zero
  Channel 分派，O 0x140a57100）携带银行样例加载完成信号；`recv()`（0x140a57100）。
- `AsyncLoadedAudioResources`（mod.rs，O 名）：`new()`（0x140a7abb0，24B）——bevy
  Commands 异步资源加载包（mpmc `Channel::recv/send` 泛型证据）。

## 4. AudioSettings（resources/audio_settings.rs，O serde 结构）

- `#[derive(Serialize, Deserialize, Clone, Default)]`（RON，impl$0 serialize/deserialize/
  clone/default；`game_config` 引用）。`impl$1::default`（O）。
- 字段（audio_update_system 反汇编，O 偏移）：`master_volume f32 @+0xf8`、
  `music_volume f32 @+0xfc`（Music 与 Music-DawnJingle 两个 VCA 共用）、`sfx_volume f32 @+0x100`、
  `ambience_volume f32 @+0x104`。`@0..0xf8` 其余字段 U（typed gap）。

## 5. AudioLoopDesc / AudioLoop（utils/audio_utils.rs）

- `AudioLoopDesc`（I 布局，O 反汇编 0x140a59220/0x140aa2fd0）：`{name: &'static str,
  count: u64 (=3), min_interval_ms: u32 (=0x32=50)}`。`new(name, param?, count, max)`（行 12）。
- `AudioLoop`（0x140a59220 返回，0x38–0x40 字节）：`{result: Result<Vec<AudioInstance>,
  anyhow::Error> @0/@8, count: usize @0x10, last_start: Instant @0x18/0x20 (初值 1000000000
  哨兵), current: usize @0x28, min_interval_ms: u32 @0x30, should_play: bool @0x34,
  has_parameter: bool @0x35, …}`（I）。
  - `current()`（0x140a593f0）：取 `instances[current]`，越界 panic。
  - `start_if_not_playing()`（0x140a596b0）：置 `last_start=now`；若 `!should_play`：
    若 `has_parameter` 先 `set_parameter`；取 `instances[current]`，若 `==Ok` 则
    `EventInstance::start`；置 `should_play=true`。
  - `stop()`（0x140a59430）：`stop` 有 50ms 冷却（`+0x30`）；冷却期后推进
    `current=(current+1)%count`、按 `has_parameter` 决定直接 `EventInstance::stop` 或先
    `set_parameter` 再停、清 `should_play`。
- `AudioCore::instantiate_loop(&self, desc) -> Result<AudioLoop,anyhow::Error>`（0x140a59220）：
  经 `from_iter(0..count, |_| AudioCore::instantiate_audio(desc.name))` 建实例池，对首个实例
  `EventInstance::get_parameter_by_name(desc.param)` 得 `has_parameter`。
- `AudioCore::instantiate_loop_snappy`（行 65；O 名，path_audio_init 用）。

## 6. MusicPlayer / MusicAudioState（systems/audio/music）

- `MusicPlayer`：`update(&mut self, …)`（0x1408b5500，行 544 状态机跳表 0x142a9fa34，字段
  `state @+0x118、last_position @+0x128、time_acc @+0x11c、event_state @+0x40、fmod_event @+0x48、
  track Vec @+0xf0/+0xf8`，I）+ `init`（0x1408b63c0，行 321 3×instantiate_audio）、
  `load_tracks`（0x1408b6b10）、`switch_track`（0x1408b70d0）、`stop_immediately`（0x1408b72e0）、
  `pause`（0x1408b7490）、`stop_photomode_snapshots`（0x1408b6250）、`get_timeline_beat_properties`
  （0x1408b7620）、`current_track`、`probabilistic_switch_track`、`generate_track_playlist`
  （fastrand，O 名）、`trigger_fpv_snapshot`、`trigger_photomode_minus_fpv_snapshot`。
  快照：`snapshot:/Track /FirstPerson /PhotoMode /PauseMenu /MusicFadeIn`（RNE .data）。
- `MusicAudioState`（music_system 插入，O 名）：richness 平滑字段 `@+0x1c4`（O 偏移）。
- 音乐事件：`event:/Music/mus_GladePreview`、`event:/Amb/GladePreview`；曲库
  `event:/Music/Halloween/{Tiny,Single,Forgotten,Enchanted}`（RNE .data，O）。

## 7. 每域状态资源（systems/audio/*，O 名 + 关键偏移）

| 状态 | 证据（O 偏移/反汇编） |
|---|---|
| `WallAudioState`（wall_audio_fmod） | 10×`AudioLoopPlayer`，stride 0x40；每 loop 应播标志 +0x38/+0x78/…；元素参数 Vec +0x88/+0xc8/… |
| `BuildingAudioState` | 7×AudioLoop（building_audio_init 7×instantiate_loop） |
| `StairsAudioState`（playermade_stairs） | 6×AudioLoop stride 0x38（+0/+0x38/+0x70/+0xa8/+0xe0/+0x118） |
| `TerrainAudioState` | 4×AudioLoop，stride 0x40，应播标志 +0x38/+0x78/+0xb8/+0xf8；`EditStateAudio::new/update`（行 18/25） |
| `TreeAudioState`/`TreeAmbience`（组件） | tree_audio_init 4×instantiate_loop；`TreeAmbience` 组件（O 名） |
| `ForestAudioState` | forest_audio_init 1×instantiate_loop |
| `GardenAudioState`/`PathAudioState` | 各 4×instantiate_loop（update_draw_audio 模板） |
| `DecoratorAudioState` | decorator_audio_init 16×instantiate_loop |
| `SignifierAudioState` | signifier_audio_init 1×instantiate_loop + 4×instantiate_audio |
| `MenuAudioState` | menu_audio_init 1×instantiate_audio（`snapshot:/PauseMenu`） |
| `ReplayAudioState` | 2×AudioInstance（`event:/UI/Undo`、`event:/UI/Redo`） |
| `GameLoadingAudioState` | `{loading_snapshot: AudioInstance @0, …: flag @+0x30, timer f32 @+0x34}`（O 偏移）；`impl$0::default`（行 11） |
| `TriggerAudioState` | 4×AudioInstance（Camera Tree Rustle/High Wind/Camera Whoosh/InForest 快照）+ 相机缓存 `{flag@+0x30, cached_pos@+0x31, audibility f32@+0x34, volume f32@+0x35, byte@+0x36, state@+0x18, playing@+0x24}`（O 偏移，语义 I） |
| `WaterAudioState` | 4×AudioLoop（water_audio_init 行 12 AudioLoopDesc::new ×4）；`mode u8 @+0x58` |
| `WaterAmbienceAudio` | `{loops: Vec<AudioLoop>, …}`（stride 0x38 证据）；`WaterAudioGrid`（wp7） |
| `FirstPersonViewAudioState` | 1×AudioLoop（`event:/SFX/Player/Walking`） |
| `SheepAudio` | `impl$0::new`（O 名） |
| `RoofAudioState` | 3×AudioLoop（roof_audio_init 3×instantiate_loop） |

## 8. 生产者/消费者总表（谁写入、谁消费）

- 生产者：`load_audio`（startup）→ `AudioSamplesLoadedPromise::recv` 后 `insert_resource<AudioTriggerLibrary>/<StairsCounter>/<GameLoadingAudioState>` + `remove_resource<AudioSamplesLoadedPromise>`。
- 消费者（audio 域自身）：`audio_update_system`（每帧）与 `boot_audio_update_system`（启动）读
  `AudioCore + AudioSettings + 相机`；每域 `*_system` 读各自状态 + 事件 → FMOD 胶水。
- 上游事件生产者（O，见 audio-systems.md §0）：wall/building/stairs 等游戏系统
  `EventWriter<Play*>` 写入，audio 系统 `EventReader` 消费。

## 9. Typed gaps（U，不伪造）

- AudioCoreInner 内部字段名/完整布局；Bank 完整字段；AudioInstance `0` 变体精确语义；
  AudioLoop 尾部字段（+0x36…）；每域状态内 AudioLoop 精确数量之外的附加字段；
  MusicPlayer 全量字段；AudioSettings @0..0xf8 字段；`StairsCounter`（load_audio 插入，O 名）。
