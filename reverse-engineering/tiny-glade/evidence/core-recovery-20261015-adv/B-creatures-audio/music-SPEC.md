# B 组 music SPEC（2026-10-15 第二轮）

> 组：B 音频。文件所有权：`crates/country-core/src/systems/audio/music/*` + `systems/audio/mod.rs`。
> 现状：`music_player`（五状态机）、`music_system`、`music_debug_gui`、`native.rs`（libfmod 2.222.6 适配）已实现；
> `music_player::load_tracks` 已实现但完整 GladeAudio 加载消费者未恢复。
> 本 SPEC 覆盖 contracts.json 剩余缺口：**完整 GladeAudio track producer、AudioCore/device/bank 启动、live FMOD 集成边界**。
> 证据：`evidence/core-recovery-20261015/music/*.asm`、`parameter-types.json`、`procedures.json`、
> `evidence/country-core-deep/chapters/{startup/load_glade_audio,startup/load_audio,resources/audio_resources}.rs.md`、
> `expansion-b2/decomp/decomp-0x1408be360.txt`、`decomp-0x1408bf460.txt`、`audio-misc/decomp/`。
> O=机器码证实；I=推断；U=未知。
> **文件权限提示**：`resources/audio_resources.rs`、`startup/load_glade_audio.rs`、`resources/audio*`/FMOD 为
> 共享待仲裁文件，落地前须发消息给中央 agent；本 SPEC 给出契约与接线点。

---

## 1. 剩余缺口清单

| # | 缺口 | 结论 | 见 § |
|---|---|---|---|
| 1 | GladeAudio track producer（track 从哪来/怎么加载/主题切换怎么触发） | **O（主流程）**：`load_glade_audio` @0x1408be360；`GladeAudio::load` @0x1408bf460。字段名待取证 | §2.2 |
| 2 | AudioCore / device / bank 启动 | **O（流程）**：`AudioCore::new` @0x140a54a40、`Bank::load` @0x140a53640、`AudioCoreInner::update` @0x140a53a00（PDB 名错位） | §2.3 |
| 3 | live FMOD 集成边界 | **O（签名/接线）**：原 `music_audio_system` 以 `Res<AudioCore>` 为后端；native.rs 已接 libfmod；定义真实 producer→music 接线与可测 CPU 边界 | §2.4, §3 |

---

## 2. 原行为契约

### 2.1 权威系统签名（O，`parameter-types.json`）

```
music_audio_system      @0x140a72810: (Res<AppMode>, Res<AudioCore>, ResMut<MusicAudioState>,
                                        Res<EscMenuState>, Res<ActiveSession>, Res<DebugGui>,
                                        Res<GameTime>, Res<Time<()>>, Res<PhotomodeScreen>,
                                        Res<PhotomodeState>)
music_audio_init_system @0x140a71f20: (…)
music_debug_gui         @0x14083c4a0: (Option<Res<EguiContextResource>>, ResMut<MusicAudioState>,
                                        ResMut<DebugGui>)
【theme producer】load_glade_audio @0x1408be360:
                                        (Res<AudioCore>, Res<ActiveSession>, ResMut<MusicAudioState>)
```
- **后端是 `Res<AudioCore>`，不是独立 backend 资源** —— 当前 `MusicBackend` trait 是中间抽象，落地时须让 AudioCore（或其 adapter）实现该接口。
- **theme 来源是 `Res<ActiveSession>`**（当前 `music_system` 已用 `session.meta.glade` 读，方向正确）。
- 另见 `toolbar/menu/new_session` 系统（`parameter-types.json` 第三组）也 `ResMut<MusicAudioState>`——主题切换还牵涉 new-session 流程（组外，登记）。

### 2.2 GladeAudio track producer（load_glade_audio @0x1408be360）

**主题切换触发（O）**：系统读 `ActiveSession` theme 字节 @+0x58 → ×8 索引 folder 表
（`@0x142aa1540` ptr / `@0x142aa1568` len；实测前 5 项有效：`summer/olden/flowery/autumn/winter`，
后 3 项是表外垃圾，游戏依赖 theme∈0..4）。

**主流程（O，decomp-0x1408be360）**：
1. `player.pause()`（FUN_1408b72e0）+ 清 current-track 字段（`puVar7[0x72]=0`，原生布局偏移实现时映射）。
2. 拼路径 `assets/glade`（前缀 @0x142aef51a，len 0xc）+ `/{theme}` → `GladeAudio::load`（FUN_1408bf460）
   读取 `{folder}/audio.json`（`"audio.json"` @0x142aa0d44 len10）。
3. 构造 **4 个 FMOD 事件全名**（串表 @0x142aa0c33 起，融合无 NUL）：
   - `event:/Music/Halloween/Single Leaf`（len 0x22=34，@0x142aa0c33）
   - `event:/Music/Halloween/Tiny Olden Glade`（len 0x27=39，@0x142aa0c55）
   - `event:/Music/Halloween/Forgotten Ruins`（len 0x26=38，@0x142aa0c7c）
   - `event:/Music/Halloween/Enchanted Forest`（len 0x27=39，@0x142aa0ca2）
4. `player.load_tracks`（FUN_1408b6b10）用这 4 个名字。
5. 每 track `AudioCoreInner::instantiate_audio`（FUN_140a54440）+ 容器注册（FUN_14089a280）。
6. `set_parameter`（FUN_140a53db0）：
   - `"GladeTypeIsHalloween"`（@0x142aa0ce8 len9）= **table @0x142aa152c + theme*4**（实测 f32
     [0,3,1,2,4] = 当前 `GladeType::audio_id()` 同表；theme 0..4 有效）
   - `"IsHalloween"`（@0x142aa0cf1 len0xb）= `1.0`（f32 @0x142925904）当 theme 文件夹串非空，否则 `0.0`
7. theme 旗标写回 player 域（`puStack_58[0x70]/[0x71]` = `0.25`（f32 @0x142925950）当 theme==4 否则 `0`；
   原生偏移实现时按布局映射）。

**`GladeAudio::load` @0x1408bf460（O 流程）**：
- 读文件（FUN_140d59140）→ `serde_json::from_str`（FUN_141406460，GladeAudio::Deserialize）
  → 成功写 **9 qword = 72 B** 结构（param_1[0..8]）到输出；
- 失败 → `GladeAudio::default`（FUN_1408bf2c0）回退（default 构造后写回）。
- **结构体字段名 U（待取证）**：serde visitor 的键名在串表；audio.json 实际内容/键名需对发行资产核对。
  当前重构 `GladeAudio` 无 4 主题音乐事件表（load_glade_audio.rs.md 契约缺口登记保持）。

### 2.3 AudioCore / device / bank 启动（resources/audio_resources.rs，组外仲裁）

**`AudioCore::new` @0x140a54a40（O）**：
`Studio::create` → `get_core_system` → `set_software_format`（Mixer 日志）→ `initialize`
→ 扫描 `assets/audio/Desktop/*.bank` → `HashMap<String,Bank>` extend → 每 bank `Bank::load` + `Bank::load_sample_data`。

**`Bank::load` @0x140a53640（O）**：`loaded&1` 早退；`Studio::load_bank_file(path@+0x10, flags=0)`；
**失败 = panic**（fmt `"Error loading FMOD bank file: {path} {err}"`）；成功 `loaded=1` + 句柄@+8。
`Bank::load_sample_data` @0x140a53870：log `"Loading sample data of FMOD bank "`。

**`AudioCoreInner::update` @0x140a53a00（PDB 名 DelayedBankLoader::load 错位）（O）**：
听者三断言（`|forward|-1`/`|up|-1`/`dot<0.001`）→ `set_listener_attributes(listener=0)` →
`Studio::update`；失败 log `"FMOD update error: "`。

**`AudioCoreInner::set_parameter` @0x140a53db0（O）**：`Studio::set_parameter_by_name`；失败 → 全局报告注册表
（槽 0x143634d60/d58，`module_p_<名>path!(…)::Warn:<err>`）；锁失败 panic `"Mutex was poisonned"`
（双 n，0x142ad2a48）；anyhow `"Couldn't set global FMOD parameter "`。

**`AudioCoreInner::instantiate_audio` @0x140a54440（O）**：`Studio::get_event` → `create_instance` →
`Active{disc@+0, *mut EventInstance@+8, name String@+0x10, state u8@+0x28 init 2}`；
失败 `Err(anyhow "FMOD error: "/"get_event(")`。
**`AudioCore::instantiate_audio` @0x140a55f40（O）**：core[+0] 非空→Inner 路径；空→Disabled。

**`AudioInstance` 判别 u64@+0（O）**：`0=Disabled / 1=Active / 2=Err(anyhow)`；
`start` @0x140a57240（disc==1→start）；`start_if_not_playing` @0x140a57300（disc&1→get_playback_state→掩码 0xb
={Playing,Sustaining,Starting} 跳过→start）；`stop` @0x140a57470（disc==1→stop(0)=STOP_IMMEDIATE）；
`stop_on_drop` @0x140a57540（Drop 时 disc==1→stop）；`set_position` @0x140a57da0（forward(0,0,-1)/up(0,1,0)/
velocity 0，常量 0x142ad0460）。
`AudioSamplesLoadedPromise::recv` @0x140a57100：crossbeam 三通道 `recv_timeout(1s)`；超时丢 payload 正常返回。

### 2.4 live FMOD 集成边界（O 签名 + 现有 native.rs 适配）

- 原系统签名要求 `Res<AudioCore>` 为 music 后端（§2.1）。当前 `native.rs::FmodMusicBackend`（libfmod 2.222.6
  官方 crate，Cargo.lock checksum `75defbe5…`）是纯适配层：`studio: Option<Arc<Studio>>`、真实
  `get_event→create_instance→release`、真实 `set_parameter_by_name`、真实 TIMELINE_BEAT 回调。
- **接线点**：AudioCore（持有 `Studio` + `HashMap<String,Bank>`）落地后：
  方案 A（最小）：`impl MusicBackend for AudioCore`（instantiate→Inner.instantiate_audio；set_global_parameter→Inner.set_parameter），
  music 域系统继续 `B: MusicBackend`；
  方案 B（原生对齐）：music_audio_system 直接 `Res<AudioCore>`。
  推荐 A 起步，把 AudioCore 的 FMOD 生命周期保留在其自身模块，B 组只暴露 trait。
- **可测 CPU 边界**：music_player/music_system 的 5 状态机与参数策略**不依赖 FMOD 即可测试**
  （当前 `native_transition_thresholds_preserve_command_order` 已用 mock backend 覆盖）；
  FMOD 真调用仅在 `Res<AudioCore>` 就绪 + bank 已加载后激活。禁止用模拟播放冒充真实后端。

---

## 3. 可直接落地项（实现 agent 优先；组外文件先仲裁）

1. **`GladeAudio`（startup/load_glade_audio.rs，组外仲裁）**：72 B 结构 + `Deserialize`（serde visitor）+ `default` 回退。
   字段名先取到再落地（见 §4 T2）。
2. **`load_glade_audio` producer 系统**：`(Res<AudioCore>|backend, Res<ActiveSession>, ResMut<MusicAudioState>)`——
   按 §2.2 步骤：pause → 读 theme → 拼 4 事件名 → `load_tracks` → 逐 track instantiate/注册 → 设
   `GladeTypeIsHalloween`/`IsHalloween` 参数 → theme 旗标。
3. **`AudioCore` / `AudioCoreInner` / `Bank` / `AudioInstance` / `AudioSamplesLoadedPromise`**（resources/audio_resources.rs，
   组外仲裁）：按 §2.3 契约。AudioCore 作为 music 后端接入（§2.4 方案 A）。
4. **music_system 收敛到 `Res<AudioCore>` 后端**：当前 `MusicBackend` 泛型已在位；把 backend 具体化为 AudioCore adapter。
5. 参数名常量表落地：`GladeTypeIsHalloween`@0x142aa0ce8、`IsHalloween`@0x142aa0cf1、`PreviewGladeType`、
   `MusicRichness`、`NewGladeTime`、`FirstPersonOff`。

---

## 4. 需先取证项（不落地、登记）

- **T1 主题切换触发条件**：load_glade_audio 的**调度阶段/触发时机**（何时运行：theme 变化检测在哪？new_session 也写
  MusicAudioState）。`toolbar/menu/new_session` 系统（parameter-types.json 第 3 组）持有 `ResMut<MusicAudioState>`——确认
  主题切换是否由 new-session 流驱动（组外，需与 C/session 组对齐）。
- **T2 GladeAudio 字段名与 audio.json 结构**：读发行 `assets/glade/{theme}/audio.json` 或 serde visitor 串表
  （`_::impl$0::deserialize` 族 @模块 1691）。字段名 U，不凭空建模。
- **T3 player 内 theme 旗标偏移**：`puStack_58[0x70]/[0x71]`（0x1c0/0x1c4）对应当前 Rust 布局哪个字段，按布局映射核对。
- **T4 常量复核**：`0.25`（@0x142925950）、`1.0`（@0x142925904）已实测；`assets/glade` 前缀串边界（0x142aef51a len12）
  与路径分隔符在实现时对真实目录核对。
- **T5 `AudioCoreInner::new_impl` / `DelayedBankLoader::load`（PDB 错位名）**：构造细节无独立 decomp 落盘，落地时补取证。

---

## 5. 排除项（明确不做/后置）

- FMOD Studio/EventInstance 的引擎级实现（复用 libfmod 2.222.6 依赖，不重实现）。
- GPU 音频可视化（`visualize_sounds` 调试渲染）后置。
- 其他音频子系统（sheep_audio、wall_audio、terrain_audio 等）不在本组。
