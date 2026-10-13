# WP14 · country-core audio domain — contract 3: audio_trigger_library & trigger_audio

范围：`asset_libraries/audio_trigger_library.rs`、`systems/audio/trigger_audio.rs`、
`startup/load_audio.rs`（触发库的装配）。版本 B。

## 1. 类型（O 名，U 布局）

- `AudioTriggerLibrary`：触发器空间库。方法（O）：
  `add_trigger`（行 55，内联于 `systems::prefab::spawn_prefabs` 0x83…）、
  `update_trigger_shape`（行 63，`systems::prefab::update_prefabs` 0x1408430…）、
  `remove_trigger`（行 70，`systems::prefab::despawn_prefab_contents` 0x140843…）、
  `clear`（`uninit_clearing` 0x140a7abb0 行 76）、`get_triggers`（0x14157ce90，260B）。
  依赖 `slotmap`（O imports）。→ 语义：以实体/槽键索引的 `AudioTriggerCollection` 表。
- `AudioTriggerCollection`：单个实体的一组触发盒。方法（O）：`boxes`（0x14157ce40，532B）、
  `clear`。`boxes()` 返回 28 字节盒条目切片迭代器：
  `{ptr=base+0x1c, end=base+len*0x1c, is_some, stride=0x1c}`（O 反汇编）；
  盒条目 `@+0x18` 字节 bit0 = enabled（trigger_audio_system 判 `(*puVar2 & 1)`，O）。
- `AudioTriggerHandle` / `AudioTriggerCollectionHandle`：slotmap 键（O 名；Deref/From
  `impl$3::from` 0x140833250，3B）。
- `impl$18::eq` / `impl$22::default` / `impl$2::default`：派生实现（O 名）。

## 2. trigger_audio_init_system（O，0x1409ea8d0，行 321 区段）

`fn trigger_audio_init_system(core: Res<AudioCore>, commands: Commands)`：
`AudioCoreInner::instantiate_audio` ×4（`*core[0]` 非空分支），事件串（O .data/反汇编）：
1. `event:/SFX/Camera Tree Rustle`（行 1 串，movabs 分块 `…ameraT re…`，RNE .data 确认）
2. `event:/Amb/High Wind`
3. `event:/SFX/Camera Whoosh`
4. `snapshot:/InForest`
→ `insert_resource<TriggerAudioState>`。

## 3. trigger_audio_system（O，0x1409eafa0）

参数（I）：`ResMut<TriggerAudioState>`（含 4 实例 + 相机缓存）、`Res<CameraMatrices>`、
`Res<AudioTriggerLibrary>`、`Res<…位置/森林标记>`（`[arg+0x128+idx]` 数组、`[arg+0x4d0]`
索引字节）等。

业务（O 反汇编）：
1. `[state+0x58]` f32 阈值；相机 `eye_position`。
2. 相机段（本帧位移）对 `AudioTriggerLibrary` 全部 `boxes()` 盒做 `Aabb3::intersect_segment`
   （`utils::geometry::aabb::Aabb3`）；任一命中 → audibility +0.5，否则 -0.1，clamp [0,1]
   （状态 `+0x34` 平滑值）。
3. audibility<=0 → 停实例[1]（High Wind）；否则 `start_if_not_playing`。
4. 音量 = 相机本帧位移 / 阈值 × 0.0125（O），再乘位置衰减（`[arg+0x128+idx]` 数组非 0 → ×0.1），
   经 lerp（速率表 0x3dcccccd/0x3e4ccccd）平滑到 `+0x35`；`AudioCoreInner::set_parameter` ×2
   （Camera Whoosh 参数）。
5. 相机高度 `z>=11` 或 `dist<=70` → 停实例[3]（InForest 快照）并 set_parameter；
   否则 `start_if_not_playing` + set_parameter。
6. `libfmod::EventInstance::set_volume`（按 `[+0x18]` 状态）。

语义（I）：Camera Tree Rustle / Whoosh 响度 = 相机穿过树木触发器盒的强度；High Wind 常响；
InForest 快照在靠近原点（树木集中）时启用。参数名 U。

## 4. load_audio（O，0x140b0e0f0，startup 装配）

`fn load_audio(commands: Commands)`：
1. `Instant::now` → `AudioSamplesLoadedPromise::recv()`（阻塞等银行样例加载，O）。
2. 日志 `country_core::startup::load_audio` 耗时。
3. `Commands::insert_resource<AudioTriggerLibrary>`（行 15/42 内联，空库 + 配置）。
4. `insert_resource<StairsCounter>`（行 ~）。
5. `insert_resource<GameLoadingAudioState>`（行 11）。
6. `remove_resource<AudioSamplesLoadedPromise>`。

消费方：`trigger_audio_system`（本 WP）；`prefab::spawn_prefabs/update_prefabs/
despawn_prefab_contents/hot_reload_prefabs`（行 55/63/70 内联 add/update/remove trigger）。

## 5. 盒数据（I，U 语义）

28 字节盒条目 = `{aabb_min: Vec3, aabb_max: Vec3, flags: u32}`（I，24+4）；`@+0x18` bit0
= enabled。`Aabb3::intersect_segment` 为几何库函数（依赖复用，不还原）。盒集合来自
关卡/prefab 数据（`add_trigger` 在 spawn_prefabs 时按 prefab 配置建立，内容 U）。

## 6. Typed gaps（U）

- `AudioTriggerCollection`/`AudioTriggerLibrary` 内部容器布局（slotmap 键 → 集合映射细节）；
- 盒条目 28B 字段完整语义（`@+0x18` 之外字节）；`add/remove/update` 精确输入；
- `TriggerAudioState` 4 实例→ 4 事件串 的精确顺序（与 init 顺序一致，I）；
- `trigger_audio_system` 参数名与全参数清单。
