# REVIEW — 卡F audio 域第一批（reviewer-eco，2026-10-16）

复核对象：restorer-eco 交付（RESTORE.md 第一批：audio_resources.rs 新建）。证据：章节
audio_resources.rs.md + audio_instance.rs + 全库编译/测试实测。未改 src。

## 逐项核对（协调者 4 点）

### 1. AudioInstance 判别模型 + 方法 — ✅ 与章节逐条对

- 判别 u64@+0（0=Disabled/1=Active/2=Err）→ Rust enum Disabled/Active/Err ✓；Active 臂
  {event@+8, name@+0x10, state u8@+0x28 初值 2}（0x140a54440 写序）→ Active{event,name,state:2} ✓。
- `start` 0x140a57240（disc==1→start）✓；`start_if_not_playing` 0x140a57300：位掩码 **0xb**
  {Playing(0)/Sustaining(1)/Starting(3) 跳过}——`should_start_if_not_playing = (0xb >>
  (state&31))&1==0` 与原生 `bt 0xb, state`（32 位取模）一致 ✓（测试覆盖 0/1/2/3/4 与 mod 32）。
- `stop` 0x140a57470（stop(0)=STOP_IMMEDIATE）✓；`stop_on_drop` 0x140a57540（Drop→stop）✓；
  `is_playing` 0x140a57550（==Playing）✓；`set_volume`/`set_parameter`/`set_position` 0x140a575a0/
  57670/57da0（forward(0,0,-1)/up(0,1,0)/vel 0，常量 0x142ad0460）✓。
- **注（PARTIAL 低）**：原生 `set_parameter` 失败→**全局报告注册表**（槽 0x143634d60/d58）；
  Rust 侧返回 `Err`。报告机制属第二批（fmod_to_anyhow/报告注册），本批未复现——若消费方处理
  Err 则行为可接受，登记。

### 2. Bank::load — ✅ panic 文案逐字

- `loaded&1` 早退 ✓；`Studio::load_bank_file(path@+0x10)` 失败 → panic
  `"Error loading FMOD bank file: {path} {err}"`（fmt 0x142ad25b0）逐字 ✓；成功 loaded=true ✓。
- `load_sample_data` 0x140a53870：日志 `"Loading sample data of FMOD bank "` ✓。
- **注（Unresolved 诚实）**：Bank 缺原生句柄@+8（RESTORE §三.4 登记）。

### 3. AudioCore::new — ✅ 结构对，init 链注入边界

- 扫描 `assets/audio/Desktop` `.bank` → HashMap<String,Bank> → 每 bank load + load_sample_data
  ✓（0x140a54a40 结构）。
- 原生 Studio::create/get_core_system/set_software_format/initialize 委托给注入的 `Studio`
  （构造在库外）——与注入 trait 边界一致，可接受。

### 4. 注入 trait 边界 — ✅

`EventInstance`/`Studio` trait（同 `audio_instance::LoopInstance` 模式）；真实绑定 libfmod =
库外登记。✓

## 实测

- `cargo check -p country-core --offline --locked`：**通过**（4 警告；audio_resources.rs:183 的
  Studio::Error Display 缺界已由 restorer 补 `type Error: std::fmt::Display` 修复）。
- `cargo test -p country-core --lib`：**1629 过 / 1 败**（唯一败 geometry/sphere 他组）；
  audio_resources 3/3 过。

## 结论与 DISPATCH 建议

- **卡F 第一批 = review_passed（限定）**：AudioInstance 判别/方法、Bank::load/load_sample_data、
  AudioCore::new 结构、注入 trait 边界——全部与章节/机器码对齐；编译+测试通过。
- **登记项（不阻塞）**：set_parameter 全局报告路由（第二批）、Bank 句柄@+8 Unresolved、
  AudioCore 其余方法（update/set_*_volume/instantiate_audio/AudioSamplesLoadedPromise/DelayedBankLoader）
  与 34 个 audio 系统文件 = 续批派单（RESTORE §三 诚实）。
- 建议 DISPATCH：第一批 = review_passed；34 系统文件 + AudioCore 其余方法 = 第二批续派。

## 二批复核（2026-10-16，audio_update.rs + ivy_audio.rs）

### 1. audio_update_system 0x1409c8dd0 / boot 0x1409c9320 — ✅

- 5×set_vca_volume（vca:/Master /Music /Music-DawnJingle /SFX /Amb），音量取 AudioSettings
  （master/music/dawn_jingle/sfx/ambience 五字段）✓。
- 相机听者：旋转@+0xc0（Mat3）+ 平移@+0xf0 → forward(0,0,-1)（-z_axis）/up(0,1,0)/vel 0 ✓；
  boot 变体固定默认听者（pos0/forward(0,0,-1)/up(0,1,0)/vel0）✓；`AudioCore::update` ✓。
- **注（低）**：章节/协调者记 DawnJingle@+0xfc 与 Music@+0xfc 同偏移——两音量字段共享偏移
  可疑，需 fresh 对账（不影响 5 字段语义）。

### 2. AudioSettings 资源 — ✅

5 音量字段，偏移 +0xf8/+0xfc/+0x100/+0x104 已证；Rust Resource（Copy/Debug）。✓（同注 1
的 +0xfc 重叠）。

### 3. ivy_audio_init_system 0x140ad5350 — ⚠️ PARTIAL（init 为占位，instantiate 待 AudioCore）

- `IVY_GROW_EVENT="event:/SFX/IvyGrow"` ✓（单瞬发语义）。
- `ivy_audio_system` 消费 `IvySegmentDoneGrowingMsg` → 注入 `play()`（=start_if_not_playing
  语义）✓。
- **init_system 仅为 log 占位**——instantiate（create_instance→Active 布局 0x140a54440）委托
  render/audio host 经注入 EventInstance 接线；AudioCore::instantiate_audio 属第二批 remaining。
  登记 PARTIAL（诚实，未伪造瞬发）。

### 4. 注入 trait 边界 — ✅

`Studio` trait 补 `set_vca_volume`（0x140a54810）/`update`（0x140a53a00）/`ListenerAttributes`；
`AudioCore` 补 `set_vca_volume`/`update`/`is_connected` + `Resource` impl（S:Send+Sync）——
库外 FMOD 登记。✓

### 实测

audio 域 **39/39 测试过**；cargo check 过；`cargo test --lib` 1634/1（唯一败 geometry/sphere 他组）。

### 结论

**卡F 第二批 = review_passed（限定）**：audio_update（5 VCA + 听者 + update）、AudioSettings、
注入边界 ✅；ivy_audio init 的 instantiate 待 AudioCore::instantiate_audio（第三批登记）。
建议 DISPATCH：第二批 = review_passed；剩余 32 系统文件 = 第三批续派。

## 三批复核（2026-10-16，RESTORE-BATCH3，31 系统文件 + AudioCore 其余）

### 1. AudioLoopPlayer 签名改动 — ✅ 测试同步无回归

`LoopInstance` 增 `start`/`stop`/`set_position(position:[f32;3])`；`request_play` 增 position
参数（仍 set_position→set_parameter(editing_speed)→set_volume 序）；新增 `frame_end()`
（should_play→start/stop 清标志，0x140a596b0/0x140a59430 驱动侧）。测试同步更新（8 测试含
frame_end 用例），既有语义无回归 ✓。

### 2. kind 表逐条对账 — ✅ 抽查命中

- clutter 39-kind（0..0x26，kind 0x12 空跳）：事件名
  `event:/SFX/Cosmetics/*`（BirdsNest/Box/BoxSack/…）抽查前 25 项合理，stride 0x18 kind@+0x14 ✓。
- sheep kind 0-7：Aww/Think/**Sob**/Angry/Idea/Eyes/Confused——kind2 纠错 "So"→"Sob"
  （串表 `event:/SFX/Sheep/Sob` rne-read）✓。
- menu 13 线性表、wall 0-23 复用 `recovered_systems::audio::WALL_KIND_TABLE`（multi-baseline
  已验）——登记可信。

### 3. Studio trait &self — ✅

关联 `Event: EventInstance + Send+Sync+'static` + `create_event_instance(&self)`（0x140a54440
生产者）+ `set_parameter_by_name(&self)`（0x140a53db0）——均 `&self` 匹配原生
Res<AudioCore> 不可变签名；系统泛型 `S: Studio+Send+Sync+'static`（同 audio_update 模式）✓。

### 4. 事件字段布局 — ✅ 抽查

`PlayWallAudio`{kind u8@+8, value f32@+c, position Vec3@+0x10, volume f32@+0x1c, flag u8@+0x20}
对 multi-baseline §8 ✓；`PlayClutterSoundCmd`{kind@+0x14, position@+0/+8}（stride 0x18）✓。
instance-vs-global 归类按模式核查：实例面（set_3d_attributes/set_parameter_by_name 在
EventInstance，经 AudioInstance::set_position/set_parameter）+ 全局面（Studio::set_parameter_by_name
经 AudioCore::set_parameter）——归档正确。

### 5. 待取证登记诚实 — ✅

RESTORE-BATCH3 §三 逐条登记：sheep kind4 loop "Baah" 候选（未对跳表证实）、firstperson
WALK_LOOP_EVENT、weather 参数名、clutter set_parameter 参数名（"editing_speed" 占位）、replay
AppMode==0xd 全局参数名、wall_audio_ui 其余 kind、跨域待接线（TreeAmbience 最小本地类型/水地形
栅格采样/snapping_audio 墙域）——均未编造 ✓。

### 实测

cargo check 通过；`cargo test --lib` **1665/1**（唯一败 geometry/sphere 他组）；音频域 46 测试过
（audio 过滤 70 全过）。

### 结论

**卡F 第三批 = review_passed（限定）**：AudioLoopPlayer 扩展（测试同步）、kind 表（clutter/sheep/
menu/wall）、Studio trait &self、事件字段布局、待取证诚实——全部核过。**卡F 整体 = review_passed
（限定）**：audio_resources 主闭包 + audio_update/ivy_audio + 31 系统文件 + AudioCore 其余落地；
待取证项（sheep Baah/firstperson WALK/weather 参数/clutter 参数/replay 全局参数/跨域接线）按
RESTORE-BATCH3 §三登记，不宣布整模块 100%。

## E3 + E5 确认（2026-10-16，快速）

### E3（sheep kind4 loop）— ✅ 确认

`KIND4_LOOP_EVENT = "event:/SFX/Sheep/Baah"`（rne-read + startup_sheep Baah+Pitch@0x142ac2460）；
`SHEEP_PITCH_PARAM = "Pitch"`。kind4 → `should_play4=true` + 置 Pitch 参数（值 0.0 待取证），
**无 one-shot**；帧尾按标志 `loop4.start_if_not_playing()/stop(0)` 驱动连续循环后清标志——
与一次性 kind 0/1/2/3/5/6/7（instantiate + FirstPersonOff + start_if_not_playing）分界清晰 ✓。
注（低）：Pitch 值硬编码 0.0，原生值待取证。

### E5（ivy SsboId 复算）— ✅ 定案

用**默认 xxh3 secret**（=原 EXE 0x142b2ebb0，本机 rne-read 比对一致）复算（自实现 xxh3_17_128，
以 Cobblestone3RasterStagingSsbo→0x802ff23dee42ab47 / NaniNextVisibilityBitmaskSsbo→
0x69cdc275a605bad6 两例校准通过）：
- 全名：LeafFull(61B)→0x07dc3bf45d145dea、FlowerFull(63B)→0x38888307d1038103、
  SegFull(68B)→0x154e2b7a159fd798；
- 短名：IvyLeafSsbo→0xc5901ecda79f8076、IvyFlowerSsbo→0x64617578fd68dd43、
  IvySegmentSsbo→0xb7d6e5e1f0eaa3b9；
- **均 ≠ 二进制三键**（0x9e3cfa7f2993f406 / 0x1d6de5e69ac50d2a / 0x2002d2fbdff3d80f）。
结论：**ivy 键非 NameHash(xxh3_64(name)) 来源**（全名/短名皆非），ssbo_library"NAME 串待取证"
结论保持；ivy_send_to_gpu 现状（文档化 NAME 占位）**无需改**——建模缺口注释正确 ✓。
注：具体复算值以实际串为准（协调者引 0xa077c75cc59ea5e5 vs 本机 0x07dc3bf45d145dea 有差，
不影响"均不匹配"结论）。

### AudioLoopPlayer 签名改动 — ✅ 无回归

第三批已核：position 参数 + frame_end，测试同步 8 用例，无回归。

### ⚠️ 跨组编译阻断（转 wall 组）

实测当前树 `cargo test --lib` **测试目标编译失败**：`wall_spatial_hash.rs:134`
`seen.insert((endpoint, segment.wall))` 元组 (f32, WallId) 缺 Eq/Hash（wall 组在途，非 audio/eco）。
此错使全库测试不可运行（含 audio 过滤）；协调者"1671/1"基线已被打破，请转 restorer-wall 修复后复验。

## §六 audio 参数/事件名定案（2026-10-16）— ✅ 4 项全过

1. **sheep Pitch**：`SHEEP_PITCH_PARAM="Pitch"`（0x142ac2460 len 5）；一次性 + kind4 均
   `set_parameter("Pitch", event.pitch)`（decomp 0x1409d6158 movss [rbx+0x14]）——章节
   "FirstPersonOff" 误读已删 ✓。
2. **clutter "clutter-context"**：`CLUTTER_CONTEXT_PARAM="clutter-context"`（0x142aa12d0 len 0xf）；
   值查表 `{0.0,1.0,2.0}`@0x142aa1590 按事件 context 字节@+0x15 索引（disasm 0x1408c0aa7/aab）；
   `PlayClutterSoundCmd` 增 `context:u8` ✓。
3. **wall_audio_ui 全表 vs 跳表 0x142a9fa44**：kind 2 WallSnapToSegment/3 ShapeSnap/4 WallsMerged/
   5 WallsMergedEnclosed/6 ColorBricks/7 ColorGate/8 ColorFlag/9+0xf ToggleSmoke 选择器@+9/
   0xa ColorWindow/0xb ColorRoof/0xc ColorHalftimber/0xd ColorLantern/0xe LanternTurnOn/
   0x10 ChangeWallStyle/0x13·0x14 BuildingSupportsRemoveWooden；kind 0 设全局 wall_style；
   0x11·0x12·0x15·0x16 跳过——逐条对上 ✓。
4. **firstperson/weather/replay/roof**：WALK_LOOP_EVENT="event:/SFX/Player/Walking"（0x142ad8e28 len
   0x19）+ 全局 `"FirstPerson"`（0x142ad8eb0，非 FirstPersonOff）+ `"GroundType"`；
   weather `"RainStrength"`（0x142c63000）；replay AppMode==0xd 设 `"undo"/"replay"`
   （0x142ae1778/7c）值 1.0（0x142925904）；roof 事件/参数常量（RoofHeight/RoofProfile/
   editing_speed/is_gable + bound 0.2/0.05/5.0）登记 ✓。

实测：cargo test --lib **1724/1**（唯一败 geometry/sphere 他组，StairsState 在途已消）；音频 32 测试过。

**结论：audio 库内参数面闭合**。登记：kind1 预置事件两处待核、decorator kind→loop 近似、roof
ridge 几何跨域。
