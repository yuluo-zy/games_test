# 手动反编译任务单（tree / water / critters / audio 补全）

2026-10-13。目的：补齐 `reconstruction/standalone/` 里还没有实现、或存在 UNKNOWN / 文档矛盾的函数。
只提取业务逻辑，不要求逐指令一致。调用到其他底层模块（地形、历史、射线、建筑查询等）的地方，
只记录"调用了谁、传了什么、用返回值做了什么"，不用深入被调用函数内部。

## 约定

- 基线：**B（patched EXE，cf048e27…）**。VA 取自 `*-inventory.json` 的 `va` 字段（已修正，= .text 偏移 + 0x140001000）。
  A/C 地址不通用，不要拿来对照。
- 机器码对照 oracle：`tiny-glade.rne`（原版 B）；PDB 行号表见 `skeleton/B/crates/**/<file>.rs.pdb.jsonl.gz`。
- 每个函数的交付物：在 `evidence/multi-baseline/` 下追加或新建 `<模块>-<主题>-analysis.md`，
  按下面"每个函数都要回答"的模板写一节。**不确定的写 unknown，不要猜**。
- 反编译结果原文放 `evidence/toolchain/<主题>/<函数名>.c`，分析文档里引用文件名即可。
- 优先级：**P0** 阻塞别的函数或有文档矛盾；**P1** 功能缺口；**P2** 可后置。

### 每个函数都要回答（模板）

1. **签名**：bevy 系统的参数列表（`Res`/`ResMut`/`Query`/`EventReader`/`EventWriter`/`Commands`/`Local`），
   可从调用者的泛型签名或 `system_type<FunctionSystem<...>>` / `validate_param` 包装器名字里读到。
2. **触发条件**：每帧都跑，还是有 `run_if`、事件驱动、状态门控（`AppMode` / `ActiveSession` 等）。
3. **主流程**：按分支写（条件 → 做什么），分支里读写了哪些组件/资源字段（给偏移 + 类型）。
4. **数值常量**：阈值、系数、范围，附 .rdata 地址和浮点/整数原值。
5. **输出**：发了哪些事件、写了哪些组件、调了哪些外部函数（给 VA 和函数名）。
6. **注册信息**：在哪个 plugin 的 `build` 里注册，属于哪个 SystemSet / 调度阶段、与谁有先后关系。
7. **unknown 清单**：没搞清楚的字段/分支，逐条列出。

---

## A. tree-tool（crate: `system-tree`，源文件 `crates/systems/tree-tool/src/`）

### A1 UI 入口：三个命令从哪来（P1）

现在 `MoveTreeCmd` / `InsertTreeCmd` / `DeleteTreeCmd` 只在回放和测试里发送，真实的来源（用户操作）没实现。

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `ui_place_tree` | 0x142269170 | 1441 | 什么输入触发放树（鼠标键/工具 id/拖拽状态）；在哪个条件下调用 `place_tree`；闭包 `closure$1`（0x142269720）捕获了什么；是否写历史（`TreeEdit`） |
| `place_tree` | 0x142269970 | 2038 | **最重要**：`InsertTreeCmd` 的完整字段怎么填（位置、高度、`+0x28` f32、flags `+0x2c`、`+0x30` 字节、`+0x38` index、`+0x40..0x48` 尾 8 字节 —— 这些现在是 unknown）；index 从哪分配（`next_index`？）；是否检查水/岩石/墙/重叠；放置失败的分支 |
| `random_tree_prefab` | 0x14226a170 | 225 | 候选树名列表从哪读（常量表还是资源）；随机数用 fastrand 还是 perchance；怎么选 |
| `ui_move_tree` | 0x14226b1a0 | 3268 | 拖拽移动：怎么选中一棵树（射线/空间查询）；`MoveTreeCmd` 的 index、position 怎么来；是否写历史（variant 1）；`closure$0`（0x141bd4120）干什么 |
| `ui_delete_tree` | 0x141bd2510 | 1871 | 删除：选中方式、是否写历史（variant 3 / 其余）、是否要求工具状态 |

### A2 树的生命周期系统（P1）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `hide_trees` | 0x14226a260 | 3644 | 隐藏条件（进入某模式？被墙/屋顶遮挡？拍照模式？）；隐藏是改组件还是发事件；与 `TreeComponent` 哪个字段有关 |
| `manage_tree_prefabs`（forest 模块） | 0x141bf16c0 | 5342 | 管理的是什么 prefab 池；谁消费 `SpawnPrefabCmd`/`DespawnPrefabCmd`；**`SpawnPrefabCmd` 的 72 字节载荷完整字段**（`+0x10` u64、`+0x30`=1.0、`+0x44` u32 现为 unknown）；怎么给树实体挂 `TreeComponent` 等 |
| `handplaced_tree_animation` | 0x142279500 | 3017 | `TreeAnimation` 的字段名与含义（尤其 `+4` 的 f32：生成时初值、每帧怎么衰减/增长、`<0.01` 触发销毁的语义）；手放树和森林树的差异 |
| `forest_tree_animation` | 0x14227a0d0 | 2228 | 同上，对照 forest 树的动画参数 |

### A3 装载与转换（P1）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `convert_preplaced_trees` | 0x14226be70 | 4827 | 6 个参数各是什么类型；消费 `SpawnPreplacedTrees` 事件的流程；调用 `WallSpatialHash::query_with_circle_callback`（0x1425defd0 / 0x1425df320 / 0x1422785d0）的目的与半径；把预置树转成运行时树的步骤（是否发 `InsertTreeCmd`？写 `History::insert_starting_edit`？）。**内联严重，反编译读不全时，请重点找 3 帧延迟与水/岩石判断之外的分支** |
| `startup_tree_tool` / `reset_tree_tool` | 无独立符号 | — | 内联在 `run_unsafe<FunctionSystem<void(*)(ResMut<TreeStorage>)>>` 包装器里；确认是否只清空 `TreeStorage`；确认注册在 `OnExit<AppLoadingState>` |
| `process_tree_cmds` 内的 Insert 分支 | 0x142267430 | 4758 | 现有分析已覆盖大部分，请只核对三件事：①`deny_load`/`is_corrupted`（`ActiveSession+0x58`）是否真的拦截生成；②`TreeStorage::insert` 返回值的高 32 位到底是不是 entity；③低 32 位 prefab 下标从哪来 |

---

## B. water（crate: `system-water`，部分在 country-core）

### B1 启动与历史（P0）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `startup_water` | 0x1408f1a20 | 3632 | 创建了哪些资源（`WorldRaster<512>` 双缓冲、staging、SSBO）；初始水位值；谁调用 |
| `generate_replay_cmds_from_edit<replay_water>` | 0x141444c70 | 10371 | 历史条目（`History` edit）→ 回放命令的转换规则；水的 edit 记录的字段布局（笔画点序列？半径？权重？）；与 `replay_raster` 的衔接 |
| `record_water_history`（system-water 源文件） | 在 `record_water_history.rs`，清单里无独立 VA | — | **请先在 PDB/Ghidra 里搜这个符号**：笔画何时开始/结束写历史；记录的字段；撤销/重做怎么还原栅格 |
| `BrushSizes::set_water` / `reset_water` | 0x1409f18a0 / 0x1409f1890 | 30 / 8 | 水笔刷尺寸的取值范围和默认值（影响笔画宽度） |

### B2 画水核心（P0，现有 UNKNOWN 集中处）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `draw_water` | 0x141ba9130 | 1487 | **谁置位 +0x29b 重上传标志**；笔画宽度如何取（对应 `BrushSizes`）；`zeno` 闭包里的覆盖系数/宽度公式（现在是假设 `255<<9` + 胶囊模型）；三种写模式（max/min/饱和加）的选择条件 |
| `rasterize_stroke<512>` / `draw_raster<WaterRasterStagingSsbo>` / `copy_to_staging` / `commit_scratch_to_composite` | 见 `system-water-inventory.json` | — | 脏块（tile）大小；半格偏移；双缓冲切换时机；`bilinear_sample_at_world_xz` 的半格偏移是否存在 |

### B3 水相关音频与植被（P1）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `update_water_audio_grid` | 0x141baefe0 | 2477 | 现有实现已按文档写，请核对帧位公式 `(n+(n+1)*10+1)&63`、u16 计数器、窗口大小 |
| `water_audio_system` | 0x140873630 | 1966 | 读 `WaterAudioGrid` 的哪些窗口；FMOD 事件名与参数；音量/距离衰减公式 |
| `water_audio_init_system` | 0x140873140 | 1253 | 创建了哪些 FMOD 循环音，事件名 |
| `water_ambience_audio_system` | 0x1409ecea0 | 2494 | 环境水声的触发条件（相机高度？水面面积？）；`load_index_unchecked`（0x1409ebe80）的索引含义 |
| `water_spawn_vegetation`（compute 阶段） | 0x140a13970 | 3246 | 睡莲/芦苇的生成规则（水深阈值、密度、随机种子）。GPU 计算，只提取 CPU 侧决策逻辑 |

---

## C. critters（crate: `system-critters`）

### C1 鸭子：先解决文档矛盾（P0）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `plan_duck_trajectories` | 0x14230ff00 | 8958 | **①** `TrajectoryPlan`（0x30 记录）每个字段，尤其有没有方向/类型字段区分"到达"和"离开"；**②** 是否被开发热键（`ButtonInput<KeyCode>`）门控，门控的键位；**③** 生成鸭子时的初始 `DuckState`/progress（文档说"状态 3、进度 1.0"，按字面会立刻反向走掉，请核实） |
| `spawn_ducks` | 0x140f8f210 | 1694 | 往计划列表写什么；触发条件；`DuckSpawnLocations`（0x140f8ee90 `new`，885）里 `+0x18` 到底是不是计时器、初值是 25.0 还是 0 |
| `despawn_ducks` | 0x140f8f8b0 | 4280 | 离开计划怎么写、与到达计划的区别；距离/数量阈值；`DespawnDuckCmd` 的消费者 |
| `ducks_react_to_being_inside_buildings` | 0x14231b140 | 1489 | 判定"在建筑内"的方法（射线/形状包含）；反应是什么（逃离？跳跃？发事件？） |
| `duck_splash_particle` | 0x142322650 | 1942 | 触发条件、生成的粒子参数（只记 CPU 决策） |
| `ui_pet_ducks` | 0x142322e30 | 5206 | 摸鸭子的选中方式、触发什么状态变化/事件；`closure$0`/`closure$1`（0x142324290 / 0x142324460） |

### C2 鸟：wren / 鸟位 / 鱼 / 蝴蝶（P1）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `update_wrens` | 0x14231e390 | 4736 | 状态机；与鸟位（BirdSpot）的关系；起飞/降落条件 |
| `update_decorator_bird_spots` | 0x14231c7d0 | 3821 | 装饰物上的鸟位怎么产生/回收（`BirdSpotManager::recycle` 0x1422f86e0） |
| `update_roof_bird_spots` | 0x14231d6c0 | 3268 | 屋顶鸟位的产生规则（位置、数量、与屋顶几何的关系） |
| `sim_wren_boids` | 0x142326a80 | 19140 | **wren 射线方向表 `RAY_DIRS`**（现缺失）；保持半径公式是 xz 平面还是 xyz（文档写 `sqrt(x²+y²)`，实现用 xz，请核实）；.rdata 常量表 |
| `despawn_fish` / `fish` | 0x140f90c10 / 0x140f90cd0 | 191 / 7533 | 鱼的墙面/光标逃逸逻辑（现缺失）；`FishSpawnLocations`（0x140f909f0）；删除条件 |
| `update_butterfly_states` / `update_butterfly_goals` | 0x141d84d40 / 0x141d86060 | 4894 / 1861 | **蝴蝶状态枚举 + 跳转表**（现缺失）；`multiplier += 1.0`（文档说未应用，请确认到底作用在哪）；与 firefly 的 `multiplier = 1.0` 的差别 |
| `update_butterfly_audio` | 0x141d86eb0 | 642 | FMOD 事件、位置更新（`ButterflyAudio::set_position` 0x140f85ba0）、音量条件 |

### C3 羊（P1）

| 函数 | VA | size | 重点调查 |
|---|---|---|---|
| `sheep_walk_animation` | 0x14231a630 | 1518 | 动画参数与速度的关系 |
| `sheep_pet_animation` | 0x14231ac20 | 1309 | 被摸时的动画状态 |
| `sheep_floaties` | 0x142320cf0 | 3202 | 游泳圈在什么条件下出现（水深？）；生成/移除 |
| `sheep_umbrellas` | 0x142321980 | 3266 | 下雨/天气/建筑内的条件；伞的生成与移除 |
| `react_to_being_inside_buildings` / `sheep_reaction_emoji` | 0x14231f610 / 0x14231ff70 | 2390 / 1284 | 在建筑内的判定；表情选择规则、持续时间；`register_required_components`（0x142320480）注册了哪些必需组件 |
| `ui_pet_sheep` | 0x142324f80 | 4582 | 选中与触发；闭包 0x142326170 / 0x142326340 |
| `update_sheep_audio` | 0x142316490 | 720 | 叫声触发条件与 FMOD 事件名（现有 kind 表里 kind 2 的名字是 unknown） |
| `sheep_input_record_replay` | 0x14231b720 | 1620 | 录制/回放的字段；与 `ReplayBuffer` 的关系 |
| `sheep_controls` / `apply_sheep_snapshot` | 0x1416f11a0 / 0x1416f24f0 | 4929 / 773 | 调试工具，P2；只记入口条件，判断是否是发布版也会执行 |
| `CritterLocomotion` 物种映射 | 在 `critters` 的 `register_required_components` 一族里 | — | 每个物种对应的 locomotion 参数（速度、加速度、转弯率、`retention_per_second`、乘数写入位置） |
| `startup_critters` | 无独立符号 | — | 启动时创建哪些资源（`DuckSpawnLocations`、`FishSpawnLocations`、`BirdSpotManager`） |

---

## D. audio（country-core `systems/audio/`，现在最缺的是"各玩法模块的音效系统"）

做法统一：每个系统都要抽出 **事件→FMOD 事件名→参数** 的映射表，格式参考 `audio-extra-analysis.md`
里已有的 wall/stairs kind 表（kind、事件名字符串、字符串长度、参数名、取值公式）。

### D1 没有任何实现的系统（P1）

| 系统 / init | VA（system / init） | size | 重点调查 |
|---|---|---|---|
| `tree_audio_system` / `tree_audio_init_system` | 0x1414638e0 / 0x1414633e0 | 4465 / 1274 | **消费 `PlayTreeSoundCmd`**：kind 0/1/2 对应的 FMOD 事件名；`in_water`、`flag`、`height` 如何影响参数；位置来源；同时播放上限 |
| `terrain_audio_system` / init | 0x1414618c0 / 0x1414613e0 | 5367 / 1237 | `PlayTerrainSound` 的 kind 表；挖地/抬升的循环音与速度参数 |
| `terrain_stairs_audio_system` | 0x141462dc0 | 1558 | `StairsCounter` / `SuppressPathSound` 的作用 |
| `forest_audio_system` / init | 0x140aa1f00 / 0x140aa18f0 | 4295 / 1543 | 森林环境音：按树数量/密度还是按相机位置；参数名 |
| `decorator_audio_system` / init | 0x14145dc20 / 0x14145c6b0 | 14269 / 5482 | **最大**：`PlayDecoratorSoundCmd` 的 kind 总表（装饰物各类型对应的事件名）；`DecoratorAudioState` 字段 |
| `roof_audio_system` / init | 0x140aa3860 / 0x140aa2fd0 | 4599 / 901 | 屋顶音效 kind；`instantiate_audio_w_position`（0x140aa3360）与 `..._and_volume`（0x140aa3590）的用法 |
| `path_audio_system` / init | 0x1409d0f80 / 0x1409d08d0 | 1928 / 1701 | 路径绘制循环音；`update_draw_audio<path>`（0x14166d890）的速度/音量公式 |
| `garden_audio_system` / init | 0x140a89480 / 0x140a88fa0 | 1838 / 1237 | 花园音效；`update_draw_audio<garden>`（0x14166d9f0） |
| `timberframe_audio` / init | 0x140a67b90 / 0x140a67810 | 2195 / 885 | `PlayTimberframeSound` 的 kind 表 |
| `trigger_audio_system` / init | 0x1409eafa0 / 0x1409ea8d0 | 3804 / 1736 | 触发器的种类与事件名；`get_triggers` / `boxes` 的含义 |
| `signifier_audio_system` / init | 0x1409d6b90 / 0x1409d6320 | 8893 / 2149 | 提示音种类；闭包 0x1409d8e50 |
| `replay_audio_system` / init | 0x140ad5800 / 0x140ad54d0 | 2640 / 805 | 回放时的音效如何重放（`PlayReplayAudio` 载荷） |
| `loading_audio` / `init_game_loading_audio` | 0x140b1b1b0 / 0x140b1ae70 | 1707 / 819 | 加载界面音效与状态机 |
| `firstperson_steps` | 0x140a889b0 | 1509 | 第一人称脚步：地面材质→事件名；步频公式 |
| `elevation_supports_audio` | 0x140ad51c0 | 389 | `PlayElevationSupportSoundCmd` 的映射 |
| `ivy_audio_init_system` | 0x140ad5350 | 371 | 常春藤音效的初始化内容 |
| `snapping_audio` | 0x140a6a480 | 223 | 吸附音触发条件 |
| `load_glade_audio` / `GladeAudio::load` | 0x1408be360 / 0x1408bf460 | 3927 / 1363 | **音频资源加载表**：bank 文件名、event 路径总表（这是核实"推断事件名"的最好来源） |
| `get_building_size_01` | 0x1417d3f10 | 82 | `inverse_lerp` 的范围（现为 unknown） |

### D2 已有实现但要核对 / 补 UNKNOWN（P0–P1）

| 对象 | VA | 重点调查 |
|---|---|---|
| `MusicPlayer::update` / `switch_track` | 0x1408b5500 / 0x1408b70d0 | 状态机（淡入淡出、停止、暂停）；**平滑函数**（现缺失）；`get_timeline_beat_properties`（0x1408b7620）；`music_audio_init_system`（0x140a71f20）里的**第三个音乐实例**是什么 |
| `audio_update_system` / `boot_audio_update_system` | 0x1409c9320（boot） | **相机→监听器的数学**；启动期默认监听器位置（现缺失） |
| `wall_audio_fmod` / `wall_audio_orchestrator` / `wall_audio_path` / `wall_audio_init_system` | 0x140950920 / 0x141234bf0 / 0x140a89bb0 / 0x14094fb80 | wall 已按文档做了；请核对：`+0x280` 处 `powf` 调制、wall 音量源类型、`AudioLoopPlayer::request_play`（0x1409285a0）的行为；kind 2/6/10 等菜单音名字 |
| `building_audio` | 在 `event_systems` 清单里 | 尾部 kind→loop 表的触发条件；building id 缺失时的行为 |
| `stairs_audio_system` | 在 `event_systems` 清单里 | 各 kind 的**参数值来源**（现在只记名字，不发参数）；`event:/SFX/Stairs/` 前缀 |
| `menu_audio` / `sheep_audio_system` / `clutter_audio_system` | 0x1408c02f0（clutter） | menu kind 3/5/6/10、sheep kind 2/4、clutter kind→名字映射（现缺失/惰性） |

---

## E. 跨模块共用、先记下来的"底层接口"（用于占位）

反编译上面函数时，凡是遇到下面这些调用，**只记录签名和用法，不用深入**，这些会放进占位层：

- 地形：`TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>`（0x141246fb0）
- 墙空间哈希：`WallSpatialHash::query_with_circle_callback`（0x1425defd0）
- 历史/撤销：`History::insert_starting_edit`、`ReplayBuffer::front_mut_bucket`（0x1409f1a80）
- 会话状态：`ActiveSession` 的 `is_dragging` / `deny_load` / `is_corrupted`（0x14092e6a0 / 0x140a9fa70 / 0x140ae8320）
- 工具：`BrushSizes`、`BirdSpotManager`、`TweakerView`
- 引擎：`GameTime`、`AppMode`（值 13 的含义）、`GameSystemSet`

## 建议顺序

1. **先做 C1（鸭子矛盾）、B2（`draw_water`）、A1（`place_tree`）**：这三处有文档矛盾或核心 UNKNOWN，改动会连带影响实现。
2. 再做 D1 里的 `load_glade_audio`（拿到全部事件名，后面的 audio 系统都能对照）。
3. 其余按模块并行。
4. 每完成一个主题，在 `PROJECT_PROGRESS.md` 记一行，我就可以开始写对应的 Rust。

## 不在本次范围

GPU 绘制与渲染（`draw_*_shader`、`*_send_to_gpu`、`water_planes_draw`、`lilypad_draw`、`reed_draw`）、
`music_debug_gui` / `wall_audio_ui`（调试 UI）、clutter 模块、A / C 版本。
