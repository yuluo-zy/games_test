# G1 启动 → 装配 → 主循环契约（Round 10 ① 逆向取证）

> 取证日期：2026-10-19。只读原版机器码与当前工作树，未修改任何源码。
> 基线：B patched `workspace/input/tiny-glade.exe`（MCP 只读库 g1_r10_1..9，`run_auto_analysis=false`）。
> 本轮新反编译落盘：`evidence/round10/tools/dec-<VA>.c`（run_game_thread 0x14059d450、
> add_game_startup_systems 0x140941110、13 个 startup 函数、相机链、startup 批次 builders）。
> 直接调用扫描：`evidence/round10/tools/` 下脚本内联（capstone/裸 e8 解析），结果已写入本文。
> 置信度：**O** = 原版机器码/字符串/调用点直读；**I** = 由 O 证据推断；**U** = 证据不足，禁止编造。
>
> **并发说明**：取证期间 G1② 正在写 `crates/country-core/src/startup/*.rs`
> （13 个文件 mtime 12:50–12:57）。本文按“原版顺序 + 当前树事实”双基线写；
> ② 落盘代码以本契约的地址/常量/顺序为准核对，**不要**把本文当当前源码快照。

---

## 0. 结论先行：三处前提纠偏

1. **13 个 startup 模块文件已不再是“不存在”**。12:50–12:57 之间已出现
   `crates/country-core/src/startup/{clearing,load_audio,load_glade_grading,spawn_brush_previews,
   spawn_human_ref,spawn_preplaced_trees,startup_critters,startup_meadow,startup_roof,
   startup_sheep,startup_sky_and_sun,startup_time_of_day,startup_walls}.rs`，
   `startup/mod.rs:4-35` 已全部 `pub mod`。本文给的是**原版每个模块的地址/资源/常量/阶段**，
   ② 用它逐项核对已写内容，缺口只补“原版有、当前无”的部分。置信度 O（目录 listing）。
2. **主相机链不是“整链缺失”**。当前树已有：
   `systems/main_camera.rs:133 main_camera_update`、`:194 paused_main_camera_update`、
   `:218 camera_snapshot`、`:245 reset_camera`、`:268 init_walk_camera`、`:304 teleport`；
   `systems/camera_rig/walk.rs`（new/teleport/rotation/update/teleport_rig）、
   `butterfly.rs`（ButterflyCamera::update，dt=1000 已对上原版）、`free_camera.rs`；
   并且 `startup/run_game.rs:198-230` 已把 `main_camera_update`（clock_running）、
   `paused_main_camera_update`（clock_paused）、`camera_snapshot` 注册进 Update。
   **真实缺口**见 §3.6（不是“未进 Schedule”）。置信度 O（文件/行号直读）。
3. **装配面名字要按 PDB 真实符号**：
   `add_ui_systems` → `systems::ui::impl$0::add_ui` @ **0x140945c60**；
   `add_collision_systems` → `systems::collision::impl$0::build` @ **0x140b685e0**（Plugin::build）；
   `add_prefab_systems` → `systems::prefab::impl$0::build` @ **0x140840430**（Plugin::build）；
   `add_backend_systems` @ **0x140942c80** 确为直调函数。
   两个 `::build` 在原版**没有任何直接 call**（全 .text `e8` 扫描为 0），只能经
   `add_plugins` 的 vtable 间接进入 → 当前树若直接调用它们等价函数，顺序按 §5 表，但**不能声称**
   已复刻“插件注册”时序。置信度 O（符号 + 调用扫描）。

---

## 1. 启动顺序（原版，按调用先后）

### 1.1 宿主级装配（O）

包含 `add_stages` 与 `add_game_startup_systems` 的装配函数是 **0x140593b70–0x140599973**（24 KB）：

```
0x1405948be  call 0x140945ed0   systems::utils::impl$0::add_stages        ← 先建 Schedule 阶段
0x1405949c5  call 0x140941110   startup::impl$0::add_game_startup_systems  ← 启动系统注册
0x140594c00  call 0x140945320   systems::steam::impl$4::add_steam_integration
```

同一函数内此前已完成 `insert_terrain_resources`(0x1409457c0)、`insert_history_resources`(0x140943e10)、
`insert_rendering_resources`(0x140940190)、`insert_ivy_resources`(0x1409440c0)、
`insert_backend_resources`(0x140942f80)。即：**资源插入 → add_stages → 启动系统**。
置信度 O（直接 call 序列 + `sysres-procedures.json` 符号表）。

### 1.2 add_game_startup_systems 0x140941110：四次 `add_systems`（O）

反汇编（0x140941110–0x140941191）：

```
batch1: 0x140941125  call 0x140a97110(建 tuple1) ; 0x140941130 call 0x140b67620(App, tuple1)
batch2: 0x14094113d  FUN_141740a20(session tuple) → 0x140941150 FUN_141452870(wrap)
        0x14094115a  mov dl,1 ; 0x14094115d call 0x140b66830(App,1,rbx)
batch3: 0x14094116a  call 0x140b3f550(tuple3)
        0x140941172  mov dl,1 ; 0x140941177 call 0x140b66480(App,1,tuple3)
batch4: 0x14094117f  call 0x140b67b50(App)   （内部 0x140b67bda → 0x14099b8b0 建 tuple4）
尾部:   Schedules 查表(0x8e1bd251962ba784/0x6a782374c0e6529b)，写 schedule+0x5c = App+0x5b0
```

四个 helper（0x140b67620/66830/66480/67b50）都在做同一 `Schedules` 查表（同一对 type-id 常量），
差别是后三个多带 `dl=1` 的 label 判别值。**结论**：四批都是“一次性启动系统”，分四批只为绕开
Bevy tuple 长度限制。置信度 O（机器码）；四批精确 label 见 1.4。

### 1.3 四批的系统清单（所有系统名 = builder 内 `IntoSystem` 的 `type_name` 字符串，O）

**batch1（PreStartup）`FUN_140a97110` = 4 个 SystemConfigs：**

| # | 系统 | builder |
|---|---|---|
| 1 | `AssetTextureLibrary::create_preset_textures` | 0x140ce4b10 描述符 @0x142adaf38 |
| 2 | `startup::load_mesh_atlases::load_mesh_atlases` | `FUN_1415f4750` @0x1415f4750 |
| 3 | 聚合组 `FUN_140a2bf40`（9 个 leaf builder + 内联 load_textures/首个 ssbo 描述符，共 9–11 项），按 builder 调用序：`load_textures`(0x140a2c04d)、`init_populate_ssbo_library`(0x140a2bf40 首个描述符)、`load_ui_shaders`、`load_shaders`、`init_populate_ssbo_library(+minimal)`、`load_prefabs(+reload_prefabs)`、`init_color_library(+reload_color_library)`、`load_game_profile(+save_game_profile)`、`init_glade_border`、**`once_startup_walls`**、**`load_audio`** | 0x140a2bf40 |
| 4 | `startup::startup_nani::startup_nani(+reload_nani)` | `FUN_1409c8600` |

> 其中聚合器的调用地址顺序（O）：0x140a2bf40 内
> `0x140a2c163 → once_startup_walls`、`0x140a2c16f → load_audio`；leaf 0x140a2bf40 的
> 前 9 个 builder 依次为 0x14179f4e0(load_ui_shaders)、0x14179fa20(load_shaders)、
> 0x1408ae240(ssbo×2)、0x140a04b30(load_prefabs×2)、0x1413fb780(init_color_library×2)、
> 0x140a5f150(game_profile×2)、0x140af3ac0(init_glade_border)、0x1408c6e60(once_startup_walls)、
> 0x140a9f5c0(load_audio)。`load_textures`/首个 ssbo 描述符由聚合器自身内联发出，精确重数 = I。

**batch2（OnEnter(Game)）`FUN_141740a20` + `FUN_141452870`：**

```
FUN_141740a20 → load_game_session（描述符 @0x142ccfb99，0x39 字符）
FUN_141452870 → FUN_141736760 组  ++  FUN_140839b80 组
```

`FUN_141736760`（call 顺序，O）：
1. `startup::load_glade_audio::load_glade_audio`（0x141736878；组首描述符）
2. `load_glade_grading::load_glade_grading`（0x141736939 → builder 0x1417cce60）
3. `startup_walls::glade_startup_walls`（0x141736951 → 0x1408c6af0）
4. `startup_grass::startup_grass`（0x14173695d → 0x140a91810）
5. `startup_sky_and_sun::startup_sky_and_sun`（0x141736969 → 0x140935640）
6. `startup_critters::startup_critters`（0x141736975 → 0x140a11fe0）
7. `startup_roof::startup_roof`（0x141736981 → 0x140ac9150）
8. `startup_water::startup_water`（0x141736993 → 0x1409853c0）
9. `startup_garden_brush::startup_garden_brush`（0x14173699f → 0x14092af20）

`FUN_140839b80`（call 顺序，O）：
1. `init_photomode_state`（0x140839c98；组首描述符）
2. `load_glade_tod_and_weather::load_glade_tod_and_weather`（0x1413f7a10）
3. `load_glade_tod_and_weather::load_snow_particles`（0x1413f7bd0）
4. **`clearing::init_clearing`**（0x140a11c80）
5. **`startup_meadow::init_meadow`**（0x140a12190）
6. `init_color_library::reload_color_library`（0x1413fb5c0）
7. `load_textures::load_glade_texture_mappings`（0x140a2caf0）
8. `<App as StartupSystems>::add_game_startup_systems::{{closure}}`（0x140b3d920）

**batch3（同 label，dl=1）`FUN_140b3f550`：**
1. 闭包 `<App as StartupSystems>::add_game_startup_systems::{{closure}}`
   （skeleton `startup/mod.rs` closure$0 源行 164 = 地形 readback；
   closure$1 源行 177 = EventWriter::write；二者为本批闭包的内联体）
2. **`clearing::uninit_clearing`**（builder 0x140a11930，描述符 @0x142ac7bc0）
3. `systems::despawn::despawn_glade_entities`（builder `FUN_140a04980`）

**batch4（同 label，builder 内部查表）`FUN_140b67b50 → FUN_14099b8b0`：**
1. `panic_reporter_startup::panic_reporter_startup`（0x14099b9c8；组首）
2. **`spawn_brush_previews::spawn_brush_previews`**（0x140a71200）
3. **`spawn_human_ref::spawn_human_ref`**（0x140a11e30）
4. **`startup_time_of_day::startup_time_of_day`**（0x140a4c610）
5. `startup_deferred_raster::startup_deferred_raster`（0x1408c50f0）

**另有：`spawn_preplaced_trees` 不在 add_game_startup_systems**。
它由 `save::impl$1::add_save_load_systems`（**0x1409443b0**，源码行 94）注册：
builder 0x1409beac0 @0x140944507；该函数由 `run_game_thread` 在 **0x14059f8b6** 直调（O）。

### 1.4 阶段判定

- **batch1 = `PreStartup`（O）**。当前宿主 `crates/tiny-glade/src/run_game.rs:629-637` 注释与代码
  已按 `@140941125/1130` 把 `once_startup_walls` 放进 `bevy_app::PreStartup`
  （`.after(load_game_profile)`），与原版 batch1 内“game_profile 先、once_walls 后”一致。
- **batch2 = `OnEnter(AppLoadingState::Game)`（O）**。`run_game.rs:472-480` 注释引
  `@14094113d`（load_game_session 条件）与 `@140941158`（OnEnter state1=Game），当前宿主已照此注册。
- **batch3、batch4 = 与 batch2 同一 label（I）**：三点机器码都传 `dl=1`，且三者共用同一
  `Schedules` type-id 对；未从常量独立证明 label 值，标 I。
- **Startup vs PreStartup 的措辞**：项目内既有注释（`publish_public_wall_state.rs:105`）称
  “add_game_startup_systems installs it in the PreStartup chain”，与 batch1 一致；
  batch2-4 是进入 Game 状态时的一次性系统。② 不得把 13 个模块全部塞进 Startup 一张表。

### 1.5 ② 的装配顺序契约

```
initialize_core_resources(world)        // run_game.rs:95
  ↓
宿主/App 已建 Schedule（相当于 add_stages 0x140945ed0）
  ↓
PreStartup 批（batch1：资产/着色器/SSBO/prefab/profile/mesh atlas/nani/
              once_startup_walls/load_audio）
  ↓
OnEnter(Game) 批（batch2：load_game_session → 世界资产组；
                  batch3：closure+uninit_clearing+despawn；
                  batch4：panic_reporter/brush_previews/human_ref/tod/deferred_raster）
  ↓
run_game_thread 的 save 注册（0x1409443b0）加入 spawn_preplaced_trees
  ↓
Update：register_update_systems（含相机三系统、音频、地形、墙、历史、存档…）
```

`startup/mod.rs:121 add_game_startup_systems` 目前把 batch1 之前的三张表（startup/session/update）
先建好再加两组网格系统；② 应按上表改写为“PreStartup 批 + OnEnter 批”，并保持
`add_startup_mesh_systems`（`startup/mod.rs:99`）里的 11 个 mesh producer 与 batch1/batch2 的资产顺序一致。

---

## 2. 13 个 startup 模块逐项

公共取证：每个系统的 `FunctionSystem<fn(...)>` 类型名在 .rdata 中存在（O），
每个 builder 的描述符字符串即上表系统名（O）。下列“输入/输出”若无单独标注均为 I。

### 2.1 `clearing`（init_clearing / uninit_clearing）

- 符号/地址：`country_core::startup::clearing::init_clearing` **0x140a78aa0**（源码行 70，11 参数）；
  `uninit_clearing` **0x140a7a5b0**（源码行 265，8 参数）；`util_add_clearing_shader` 内联（源行 41）。
- 阶段：`init_clearing` = batch2/OnEnter(Game)（builder 0x140a11c80）；
  `uninit_clearing` = batch3（builder 0x140a11930，与 `despawn_glade_entities` 同组）。O。
- 输入（观测）：`&AssetShaderLibrary`（`*param_1 + 0x10`）、`&AssetMeshLibrary`、资源 watch、
  `Commands`、`&dyn ShaderBackend`、会话/格调资源（`*(char*)(*param_4 + 0x58) == 4` 才加冬季冰裂）、
  一个 `ResMut<usize/u64>`（`param_10`，写 cap）。
- 输出：`assets/meshes/ice_cracks.json`（name `ice_cracks`）+ `/water/ice_cracks.raster.hlsl`
  （name `ice_cracks`，仅格调判别 4 分支）；无条件 `assets/meshes/distant_billboard.json`
  （name `distant_billboard`）。两次 `Commands::spawn`，bundle 大小 **0x78**（drawable+pass 组）。
- 常量：cap = `min(x, 0x44c)` = **1100**（O: `0x44c` 立即数）；uninit 遍历 **27 项 × 0xc0** 花园槽并清空
  （O: `0x1440/0xc0`），置 `+0x38 = 1`。skeleton 依赖另列
  `AudioTriggerLibrary::clear`、`AssetPlacementDataLibrary::clear`（uninit，I）。
- 缺口：`param_10` 资源真实类型名（U）；格调判别 4 是否 Winter（I）；`util_add_clearing_shader`
  在 init 里如何被复用（U）。

### 2.2 `load_audio`

- 符号/地址：`startup::load_audio::load_audio` **0x140b0e0f0**（源码行 9，2 参数）。
- 阶段：batch1/PreStartup（builder 0x140a9f5c0，O）。
- 行为（O）：第 1 参数 `*param_1` 交给 `FUN_140a57100`（AudioTriggerLibrary 侧）；随后同步
  `Events` 状态（`param_1+0x1c → *param_1[2]`），向第 2 参数（`&mut Schedule`）追加 **2** 个
  system 条目：12 字节配置 `FUN_1412dccf0`、8 字节配置 `FUN_1412d91b0`；日志 target
  `country_core::startup::load_audio`（字符串 0x142ae7db0）。
- 输出：`Commands::insert_resource`/`remove_resource`（skeleton 注解）→ `game_loading_audio`
  的两个系统进入某 Schedule。
- 缺口：这两个系统的精确身份（U，地址不在有符号索引内）；追加到哪张表（U）。

### 2.3 `load_glade_grading`

- 符号/地址：`startup::load_glade_grading::load_glade_grading` **0x1408bf9c0**（源码行 10，2 参数）。
- 阶段：batch2/OnEnter(Game)（builder 0x1417cce60 ← 0x141736939，O）。
- 输入：`ResMut<GradingLut>`（`param_1`，写 `*param_1`）、`Res<ActiveSession>`（`param_2`：
  `+0x10` 路径长度、`+0x8` 指针、`+0x58` is_halloween 判别）。
- 输出（O 字符串）：`grading.json`（12 字符，相对会话 assets root）；
  LUT 名 **`RoastMeat_Cinema_709_1`**（0x142aa0e00，22 字符）与 **`default`**；
  路径模板 **`assets/luts/<name>.png`**（0x142ae6bef / `assets/luts/` 12 + `.png`）。
  读文件 `std::fs::read_to_string`，失败 `unwrap` panic（0x142aa03b8）。
- 缺口：halloween 分支选取哪张 LUT（I）；`GradingLut` 的字段（U）。

### 2.4 `spawn_brush_previews`

- 符号/地址：`spawn_brush_previews::spawn_brush_previews` **0x140924b40**（源码行 23，6 参数）。
- 阶段：batch4（builder 0x140a71200，O）。
- 输入：`&AssetShaderLibrary`（`*param_1+0x10`）、`&AssetMeshLibrary`、路径根、`Commands`、
  `&dyn ShaderBackend`、watch。
- 资源表（O，按出现序）：
  shader `/timed_terrain_gizmo.raster.hlsl`（name `timed_terrain_gizmo_shader`）；
  mesh/name：`brush_arrow`、`brush_circle`、`brush_circle_cross`、`brush_circle_scribble`、
  `brush_arrow_flat`、`brush_4_arrows`(name `brush_terrain_edit`)、`brush_2_arrows`(name `brush_flat_roof_edit`)、
  `brush_5_replay`(name `brush_replay`)、`hand`(name `temp_sphere`)、`dashed_circle`、`dashed_lines`(name `snapping_line`)。
- 输出：`Commands::spawn`，bundle 大小统计 **10×0x38、4×0x40、2×0x78、1×0x88**（O 立即数）；
  `Transform::from_translation_scale`（O 注解）。
- 缺口：每个 bundle 与 mesh 的逐项映射（U；需按 spawn 顺序与字符串交叉）。

### 2.5 `spawn_human_ref`

- 符号/地址：`spawn_human_ref::spawn_human_ref` **0x14089d5a0**（源码行 13，4 参数）。
- 阶段：batch4（builder 0x140a11e30，O）。
- 输入：`&AssetMeshLibrary`、`&AssetShaderLibrary`、?（`*param_3`）、`Commands`。
- 输出（O）：mesh `human_ref_cube`（14 字符，0x142a9947e）、shader 查表 `vertex_color_shader`
  （19 字符）；`geometry::cube::Box::new` 常量 `[-0.25, 0.25, -0.9, 0.9]`（0x142a98e30）与
  `[0.5, 1.8, 0.0, 0.0]`（0x142a98e50）；spawn bundle **0x40**，其中 `+0x30 = 0.5f`
  （`0x3f000000`，缩放/半格，I）。
- 缺口：常数语义（半宽/高/缩放）U。

### 2.6 `spawn_preplaced_trees`

- 符号/地址：`spawn_preplaced_trees::spawn_preplaced_trees` **0x140862270**（源码行 25，5 参数）。
- 阶段：**不在 add_game_startup_systems**；由 `save::impl$1::add_save_load_systems` 0x1409443b0
  注册（builder 0x1409beac0），run_game_thread 0x14059f8b6 直调（O）。
- 输入：`Res<ActiveSession>`（`param_1`：`*param_1+0x58` 格调判别、`assets_root`）、
  `EventReader`/`EventWriter`、`ResMut<TreeStorage>`、`Commands`。
- 输出：读 `<assets_root>/assets/glade.../trees.json`（`FUN_140d62810` + 后缀 `trees.json` 10 字符，
  0x142a96184）；反序列化失败 log `deserialize trees`；每棵树
  `TreeStorage::insert` + `Commands::spawn`/`EntityCommands::insert`，spawn 尺寸 **2×0x28、2×0x30**；
  `glam::vec3`（O 注解）。
- 缺口：trees.json 的字段 schema（U；属 G2/树工具域）；per-glade 路径表 0x142a96690/0x142a966b8 的条目（U）。

### 2.7 `startup_critters`

- 符号/地址：`startup_critters::startup_critters` **0x1409d49a0**（源码行 30，9 参数）。
- 阶段：batch2（builder 0x140a11fe0 ← 0x141736975，O）。
- 输入：shader/mesh 库、`Commands`、`Res<ActiveSession>`（`*param_8+0x1f` 冬季判别）、
  `Res<AudioCore>`（`instantiate_audio`），两处 `ResMut<Events>`（`param_8/param_9` 的 +0x1c→+0x10 模式）。
- 资源表（O）：mesh `butterfly` + `/critters/butterfly.raster.hlsl`；
  `firefly` + `/critters/firefly.raster.hlsl` + `/critters/firefly_glow.raster.hlsl`（name `firefly_glow_shader`）；
  `duck_flying` + `/critters/duck_animated.raster.hlsl`（name `duck_shader`）；
  `duck_heart`、`splash` + `/splash_particle.raster.hlsl`；
  音频事件 `event:/SFX/Butterfly/Shoosh`（`*param_8+0x1f == 1` 分支，0x142ac22e9）。
- 输出：`Commands::spawn`，bundle **2×0x78、1×0x88、1×0xa8**；`Transform::from_translation/from_scale`。
- 缺口：冬季分支语义（U）、butterfly 索引选择（U）。

### 2.8 `startup_meadow`（init_meadow）

- 符号/地址：`startup::startup_meadow::init_meadow` **0x1408639f0**（源码行 28，8 参数）。
- 阶段：batch2（builder 0x140a12190 ← 0x140839b80，O）。
- 输入：`Res<ActiveSession>`（assets root）、`Command`、shader/mesh 库、Commands。
- 输出（O）：`plants.json`（11 字符，0x142a962ad）；shader `/clearing/meadow_plant.raster.hlsl`
  （name `meadow_plant_shader`）；log `deserialize plants`；spawn **1×0x88**。
- 缺口：plants.json schema（U）。skeleton 另注 `Path::with_extension`、serde MapAccess（O 注解）。

### 2.9 `startup_roof`

- 符号/地址：`startup::startup_roof::startup_roof` **0x1414547d0**（源码行 14，6 参数）。
- 阶段：batch2（builder 0x140ac9150 ← 0x141736981，O）。
- 资源表（O）：`roof_spire` + `/instanced_roof_spire.raster.hlsl`（name `instanced_roof_spire_shader`）；
  `roof_support_beam` + `/instanced_roof_beams.raster.hlsl`（name `support_beam_roof_shader`）；
  `roof_button`；`roof_delete_scribble`。
- 输出：spawn **2×0x78**。
- 缺口：每个 spawn 的 drawable 组合与 pass（U）；属 roof 用户区，② 只接资源，不改 roof 语义。

### 2.10 `startup_sheep`

- 符号/地址：`startup::startup_sheep::startup_sheep` **0x140a093e0**（源码行 27，9 参数）。
- 阶段：batch2（builder 0x1417e2c20 ← 0x141736975 邻近组，O）。
- 资源表（O）：`sheep_animation/1.json`（36 字符）与 `sheep_animation/delighted.json`（44 字符）；
  mesh `sheep_mesh`、`sheep_heart`、`umbrella`（name `umbrella_mesh`）、`floatie`（name `floatie_mesh`）；
  shader `/sheep.raster.hlsl`（name `sheep_shader`）、查表 `solid_vertex_color`（18 字符）；
  音频 `event:/SFX/Sheep/Baah`；`BirdSpotManager::gen_id` + `HashMap::get`（skeleton O）。
- 输出：`Transform::from_translation_scale`、`AudioCore::instantiate_audio`、Vec push/collect；
  无 `Commands::spawn` 注解 → 实体经 render/prefab 通道生成（I）。
- 缺口：羊实体创建路径（U）、动画状态初始值（U）。

### 2.11 `startup_sky_and_sun`

- 符号/地址：`startup::startup_sky_and_sun::startup_sky_and_sun` **0x140864430**（源码行 14，6 参数）。
- 阶段：batch2（builder 0x140935640 ← 0x141736969，O）。
- 输出（O）：mesh `assets/meshes/skydome.json`（name `skydome`）、shader `/skydome.raster.hlsl`
  （name `skydome_shader`）；spawn **1×0x78**；`Transform::from_translation`；sun/天光实体常量未单独读取（U）。
- 缺口：太阳角度/灯光组件（U）。

### 2.12 `startup_time_of_day`

- 符号/地址：`startup::startup_time_of_day::startup_time_of_day` **0x14089d6f0**（源码行 6，1 参数）；
  `load_default_tod` 内联（源码行 10）。
- 阶段：batch4（builder 0x140a4c610 ← 0x14099b8b6，O）。
- 行为（O）：`FUN_140d59140(&out, "assets/tod/default.json", 0x17)`（0x142a99540）；
  失败 unwrap panic（`called Result::unwrap() on an Err value`）；成功把 0x10 字节序列写回 `*param_1`
  （`ResMut<TimeOfDay>` 语义，I）。
- 缺口：TimeOfDay 资源字段名（U）；加载器是否即 `resources/time_of_day`（I）。

### 2.13 `startup_walls`（once_startup_walls / glade_startup_walls）

- 符号/地址：`once_startup_walls` 源码行 19（inline，无独立 RVA）；
  `glade_startup_walls` **0x140a92670**（源码行 30，6 参数）；`create_half_cube_mesh` 源码行 120（inline）。
- 阶段：**两者不同批**：`once_startup_walls` = batch1/PreStartup（builder 0x1408c6e60 ← 0x140a2bf40）；
  `glade_startup_walls` = batch2/OnEnter(Game)（builder 0x1408c6af0 ← 0x141736951）。O。
- 资源表（O）：mesh `assets/meshes/brick.json`（name `brick`）；shader
  `/wall/wall_brick_lod0.raster.hlsl`（name `wall_brick_lod0`）、
  `/wall/wall_brick_lod1.raster.hlsl`（name `wall_brick_lod1`）；`create_half_cube_mesh` 产 `half_cube`；
  `Mesh::new`、`TerrainHeightsData::subscribe`（skeleton O）。
- 输出：spawn **7×0x88**（LOD0/LOD1 与 pass 组合，逐项映射 U）。
- 缺口：7 个 drawable 的 pass 编号（U）；`once` 与 `glade` 的职责切分（U，host 当前把 `once_startup_walls`
  放 PreStartup，O）。

---

## 3. 主相机更新链

### 3.1 `main_camera_update` 0x140912c20（9 参数）

控制流（O 机器码）：

```
aspect = Window.width / Window.height        (param_6+0xd8 的 u32 pair)
写入 MainCamera +0x4d8
if new_session_flag (+0x18) != 0:
    读 cursor delta (param_1+0x160, 两个 f32)；范数<=0 且窗口>2 → return
    清 flag
if Window.width<3 || height<3: 推 WriteSettingsFileCmd（param_9 的 Events 缓冲）
mouse_depth_byte = *param_2 → MainCamera +0x344
mode = table[0x11d - (window_small)] (param_6+0x118)；6 → 0
if *(f32)(param_1+0x168) != 0:            // free-camera / interpolation 使能
    按 mode 取 jump table(&UNK_142aabbf8 + mode*8) 的 body 参数
    以 dt 与 lerp 系数计算 16B 变换并 dispatch
else:
    按 *(u32)(param_1+0x10) 走 jump table 0x142aabd04
```

- 写回：`lVar10 = *param_5`（`ResMut<MainCamera>` 的值）→ `+0x4d8` aspect、`+0x344` 字节，
  以及 `+0x40..+0x7c` 处的 4×f32 矩阵行（原件把最终矩阵块写入 MainCamera）。
- 模式索引还有第二层 `*(u32)(lVar12+0x10)`（0..2）与 `+0x18`（0..2），对应
  动作模式与相机 pivot；跳表目标体仍有损坏（U）。
- 与 `render_world` 的关系：`MainCamera` 是 ECS 资源，渲染域读取其矩阵块（当前树
  `MainCamera::camera_matrices/set_camera_matrices` 已对齐，O 源码）。

### 3.2 `paused_main_camera_update` 0x1409145a0（3 参数）

- `aspect` 同 3.1；按 `MainCamera +0x4d0` 的 mode 字节 switch，六个 case 直接给出
  per-mode 状态基址：`(0x8c, +0x100)`、`(0x144,+0x1b8)`、`(0x1c4,+0x270)`、
  `(0x264,+0x2f0)`、`(0x31c,+0x390)`、`(0x3a4,+0x448)`（O 立即数）。
- 调 vtable 第 6 项（`puVar4[6]`）更新 mode body，然后拼 0x100 字节 frame 块写回 MainCamera。
- 缺口：每 case 的 body 身份（U，跳表损坏）。

### 3.3 `reset_camera` 0x140912a40（3 参数）

- log `Resetting the camera.`（字符串 0x142aabb20 附近）；`MainCamera::new(aspect)`（0x14090ec70）
  生成 0x4e0 字节临时值，`memcpy 0x4e0` 覆盖 `*param_1`（O）。
- 清 `param_2` 指向的 Vec（容量重置为 4）——参数缓存（**U**：cache 资源类型名）。
- 向 `param_3` 的 Events 缓冲 push 一项（事件值取 `+0x40`，`WriteSettingsFileCmd` 候选，I）。

### 3.4 `camera_snapshot` 0x140911e40（5 参数）

- 读 `snapshot.mode = *(u32)(*param_4 + 0x18)`；`dt = *(f32)(*param_2+0x28) * *(f32)(*param_5+0xec)`
  （GameTime.delta × DebugGui.camera_snapshot_interpolation_speed，O）。
- 按 mode 走 jump table 0x142aabc88（**27 项**，源码 `main_camera.rs:62` 已核）。
- 缺口：27 个 mode 的快照应用体（U）。

### 3.5 `MainCamera::new` 0x14090ec70（aspect → 0x4e0 字节）

`reset_camera` 用它；当前树 `camera_rig/main_camera.rs` 的 `MainCamera::new` 已按 0x4e0 实现。
模式/字段偏移已对齐：mode byte `+0x4d0`、aspect `+0x4d8`、mouse depth `+0x344`、
per-mode body 基址见 3.2。置信度 O（地址/大小）+ I（字段语义）。

### 3.6 当前树 vs 原版的真实剩余缺口（② 只补这些）

1. `init_walk_camera`（0x14090e510）无生产调用者：原版从 0x14234b644（非 country-core 段）
   的进入 walk 模式系统调用；当前只有测试。**缺口：walk 模式入口 + 地形采样/球投**。
2. `reset_camera` 未清参数缓存（原版清 Vec、capacity=4）。`systems/main_camera.rs:245` 注释自认 U。
3. `camera_snapshot` 只推进 `value`，未应用 27 个 mode 体（U 保留）。
4. free camera 输入（平移/旋转）未恢复：`main_camera_update` 中 free 分支只设 mode，
   `systems/main_camera.rs:186` 注释 `U: free camera 输入平移/旋转驱动未恢复`。
5. Normal body update（0x140949fe0 的 smooth/radial/loose 推进）未恢复，当前只重建矩阵
   （`main_camera.rs:116-127` 注释）。
6. 相机系统已在 Update，但**执行顺序只保证 after InputSystemSet::Derive**；原版在
   `add_backend_systems` 的 Update 集内（0x140942c80 → in_set @0x140aad7d0 → SubApp Update），
   ② 若接入 backend 集，应把相机放在同一 set 链、`before` 渲染消费点。置信度 I。

---

## 4. Walk / Butterfly

### 4.1 WalkCamera（`systems/camera_rig/walk.rs` 已存在）

| 项 | 原版证据 | 当前实现 |
|---|---|---|
| `WalkCamera::new` 0x993e20 | `Position(0,1,0) + YawPitch(0,0)` | `walk.rs:29` ✅ |
| `teleport` 0x994130 | 在 0x10 步长的 mode 列表里按类型 id 两次 downcast：写 position（u64+u32），再写 yaw/pitch，乘以 **57.29578**（180/π） | `walk.rs:76 teleport_rig`（用 from_rotation_arc + YXZ 分解，I 等价） |
| `rotation` 0x994310 | 拷贝 final_transform 的四元数 | `walk.rs:52` ✅ |
| `update` 0x993d30 | 遍历 mode 列表，逐项调 vtable+0x30，聚合 transform，写 +0x18..+0x34 | `walk.rs:58` ✅ |
| `init_walk_camera` 0x90e510 | 出生点常量 **(-30.192749, 15.641989)**（0x142aab390），向下 (0,-1,0) 从 y+500 投到地面，落点 y+**1.9**，下限 **0.4**；方向归一化；再 teleport | `main_camera.rs:268` 用调用方提供 spawn/direction，缺地形采样 |

### 4.2 ButterflyCamera（`butterfly.rs` 已存在）

- `update` 0xa0d090：`applied(+0x38) != selected(+0x3c)` 时 `applied = selected` 且
  `dt = 1000.0`（0x14295b630 = 1000.0，O），随后遍历 rig driver 聚合 transform。
- 当前 `butterfly.rs:44` 已完整复刻（含 1000.0），只有“selected 由谁写”没有消费者（I）。

### 4.3 Walk/Butterfly 的消费者契约

- `init_walk_camera`：应在“进入 Walk 模式”的一次性入口调用（原版 0x14234b644 所在函数），
  输入为地形采样结果；当前树可用 `TerrainHeightsData` + 竖直 raycast 实现（I），
  采样 API 属碰撞域（U）。
- `ButterflyCamera::selected_index`：应由蝴蝶 UI/游标选择系统写；原版写入点未定位（U）。

---

## 5. 装配注册契约（run_game_thread 0x14059d450 直调顺序，O）

从 0x14059d450 的 537 个调用中抽出 game 域装配段（索引 → 调用点 → 目标 → 符号）：

| # | 调用点 | 目标 | 符号 |
|---|---|---|---|
| 1 | — | 0x1409457c0 | `terrain::impl$0::insert_terrain_resources` |
| 2 | — | 0x140943e10 | `history::impl$0::insert_history_resources` |
| 3 | — | 0x140940190 | `resources::impl$0::insert_rendering_resources` |
| 4 | — | 0x1409440c0 | `ivy::impl$0::insert_ivy_resources` |
| 5 | — | 0x140942f80 | `backend::impl$0::insert_backend_resources` |
| 6 | — | 0x1404936b0 / 0x140493120 | **U**（无符号；同段墙/花园候选） |
| 7 | — | 0x140945f50 | `wall::shapes::impl$0::add_wall_shape_ux` |
| 8 | — | 0x140945a20 | `terrain::impl$0::add_terrain_events` |
| 9 | — | 0x1409455f0 | `terrain::impl$0::add_terrain_systems` |
| 10 | — | 0x140942c80 | `backend::impl$0::add_backend_systems` |
| 11 | — | 0x140943660 | `backend::impl$0::add_backend_events`（PDB 记作 insert_backend_resources::closure$0） |
| 12 | — | 0x1409438a0 | `history::impl$0::add_history_systems` |
| 13 | — | 0x140943f40 | `history::impl$0::add_history_events` |
| 14 | — | 0x140944070 | `ivy::impl$0::add_ivy_systems` |
| 15 | — | 0x140944280 | `ivy::impl$0::add_ivy_events` |
| 16 | — | 0x1409411a0 | `audio::impl$1::add_audio_systems` |
| 17 | — | 0x140941450 | `audio::impl$1::add_audio_events`（PDB 名 add_audio_startup_systems 错位；16 个事件，O） |
| 18 | — | 0x1409443b0 | `save::impl$1::add_save_load_systems`（内含 `spawn_preplaced_trees` builder） |
| 19 | — | 0x14062fcb0 | **U**（无符号；在 save 与 ui 之间） |
| 20 | 0x14059f98b | 0x140945c60 | `ui::impl$0::add_ui` |

`prefab::impl$0::build`(0x140840430) 与 `collision::impl$0::build`(0xb685e0) 不在此直调序列
（间接插件注册，O 扫描；具体 add_plugins 点 U）。

### 5.1 顺序契约（② 执行清单）

```
insert_* 资源（terrain→history→rendering→ivy→backend）
  → wall_shape_ux
  → terrain_events → terrain_systems
  → backend_systems → backend_events
  → history_systems → history_events
  → ivy_systems → ivy_events
  → audio_systems → audio_events
  → save_load_systems(+spawn_preplaced_trees)
  → ui::add_ui
```

- **先注册什么**：资源插入必须先于所有 `add_*`；`add_backend_systems` 必须早于相机/时间/UI 消费
  （原版其内部 `in_set @0x140aad7d0`，run_game.rs:458-460 已记录 Update）。
- **后注册什么**：`ui::add_ui` 最后；`spawn_preplaced_trees` 跟随 save（会话载入后）。
- 当前树缺口：`configure_glade_audio`（`load_glade_audio.rs:22`）、
  `configure_music_consumers`（`music_system.rs:54`）有实现无调用；`add_backend_systems`、
  `insert_backend_resources`、`add_backend_events`、`add_ui` 当前树**不存在对应入口**（O grep）。
  ② 应先在这四个域补等价注册入口，再按上序接入 `register_update_systems`/host。
- `configure_glade_audio` 必须在 `ActiveSession` 建立后（`load_glade_audio` 读 `Res<ActiveSession>`），
  即 **OnEnter(Game) 之后**，不能放 PreStartup。置信度 I（依赖注解 + 系统参数，O）。
- `configure_music_consumers` 依赖 `GameTime/AppMode/EscMenuState/ActiveSession`，放 Update，
  在 `music_audio_init_system` 之后（I）。

---

## 6. 验收门（每项最小判据）

| 模块/链路 | 可编译 | 真实写入 | 有消费者 |
|---|---|---|---|
| clearing | `cargo check -p country-core` | init 写 2 个 drawable + cap=1100；uninit 清 27×0xc0 | 渲染/despawn 读实体；`uninit` 被 batch3 调度 |
| load_audio | check | `AudioTriggerLibrary`/`game_loading_audio` 资源/系统入表 | 事件消费者存在（16 事件表） |
| load_glade_grading | check | `GradingLut` 两 LUT（RoastMeat_Cinema_709_1/default） | 渲染 grading pass 读 LUT |
| spawn_brush_previews | check | ≥10 个 drawable、tuple 尺寸对表 | brush_preview 系统按名字/实体更新 |
| spawn_human_ref | check | 1 个 human_ref_cube | 人体参照消费者 |
| spawn_preplaced_trees | check | `TreeStorage` 条目 + 实体 | 树系统/存档读 TreeStorage |
| startup_critters | check | butterfly/firefly/duck/splash 实体 | critters update |
| startup_meadow | check | plants.json → 1 drawable | 植被渲染 |
| startup_roof | check | 4 mesh/2 shader + 2 spawn | roof draw pass |
| startup_sheep | check | sheep 实体/音频 | sheep 系统 |
| startup_sky_and_sun | check | skydome + 1 spawn | sky/lighting pass |
| startup_time_of_day | check | `TimeOfDay` 默认序列 | time_of_day::time_of_day |
| startup_walls | check | brick/lod0/lod1 + 7 spawn + terrain subscribe | wall draw/publication |
| 相机链 | check | aspect/+0x344/矩阵块真实写 | render_world 读 MainCamera |
| 装配 | check -p tiny-glade | 每个 add_* 进入实际 Schedule | 事件 writer/reader 非零 |

回归基线：`cargo check -p country-core` / `-p tiny-glade` 通过、`cargo test -p country-core --lib`
0 失败（第 9 轮末 2467 passed）。

---

## 7. ② 实现清单（≤15 步，先 startup → 相机 → 装配）

1. 锁基线：先跑 `cargo check -p country-core -p tiny-glade` 与 `cargo test -p country-core --lib`，
   记录 ② 已落盘 13 文件是“可编译且有真实写入”还是占位；占位一律按本契约补，禁止 no-op。
2. `startup/mod.rs:121`：把 `add_game_startup_systems` 改成四批注册：
   PreStartup（batch1 清单）、OnEnter(Game)（batch2/3/4 清单，顺序见 §1.3）。
3. 保留 `once_startup_walls` 在 PreStartup 且 `.after(load_game_profile)`；`glade_startup_walls`
   放 OnEnter(Game) 组。
4. `spawn_preplaced_trees` 接入 `systems/save/mod.rs:75 add_save_load_systems_impl`（`load_game_system`
   之后），或 host 的 save 注册段，不得丢。
5. `clearing`：init 两 drawable（ice_cracks 仅格调 4、distant_billboard 恒有）+ cap `min(x,0x44c)`；
   uninit 清 27×0xc0 槽、置 `+0x38=1`、清 placement/audio-trigger 库。
6. `spawn_human_ref`：`human_ref_cube` + `vertex_color_shader`，Box 常量 [-0.25,0.25,-0.9,0.9]/[0.5,1.8]，
   bundle 0x40。
7. `spawn_brush_previews`：11 个 mesh 名 + `timed_terrain_gizmo_shader`；spawn 计数 10/4/2/1（0x38/0x40/0x78/0x88）。
8. `startup_critters`：butterfly/firefly(glow)/duck/splash 四组 mesh+shader，`Shoosh` 分支；
   bundle 2×0x78/1×0x88/1×0xa8。
9. `startup_meadow`（plants.json+meadow_plant_shader，1×0x88）、`startup_roof`（4 mesh/2 shader，2×0x78）、
   `startup_sky_and_sun`（skydome，1×0x78）。
10. `startup_sheep`：动画 JSON + 4 mesh + 2 shader + Baah 事件；`startup_time_of_day`：
    `assets/tod/default.json` 默认序列写入 TimeOfDay。
11. `startup_walls`：brick + lod0/lod1 + half_cube + `TerrainHeightsData::subscribe`；7×0x88。
12. `load_glade_grading`（grading.json + 两 LUT + `assets/luts/*.png`）；`load_audio`
    （插入资源 + 2 个 game-loading 系统入表）。
13. 相机：不重写已存在实现；补 (a) Walk 模式入口调用 `init_walk_camera`（地形采样/球投），
    (b) `reset_camera` 清参数缓存（capacity=4），(c) free camera 输入分支与 Normal body update
    继续显式 U，(d) 保持 `run_game.rs:198-230` 注册顺序。
14. 装配：新增/恢复 `backend::{insert_backend_resources,add_backend_systems,add_backend_events}`、
    `ui::add_ui`、collision/prefab 的注册入口，并按 §5.1 顺序接入；把
    `configure_glade_audio`（OnEnter 后）与 `configure_music_consumers`（Update）接上调用者。
15. 验收：跑 §6 表格 + 基线；逐项在 DELIVERY 里标 O/I/U 与真实写入/消费者，不用日志或测试 double 冒充。

---

## 8. 证据索引

- 反编译：`evidence/round10/tools/dec-140941110.c`（add_game_startup_systems）、
  `dec-14059d450.c`（run_game_thread）、`dec-140912c20.c`/`dec-1409145a0.c`/`dec-140912a40.c`/
  `dec-140911e40.c`（相机）、`dec-14090ec70.c`（MainCamera::new）、`dec-14090e510.c`（init_walk_camera）、
  `dec-140994130.c`（WalkCamera::teleport）、`dec-140a0d090.c`（butterfly update）、
  `dec-140a97110.c`/`dec-1415f4750.c`/`dec-140a2bf40.c`/`dec-1409c8600.c`/`dec-140b3f550.c`/
  `dec-140b67b50.c`/`dec-140a11930.c`（启动四批 builder）、13 个 startup 函数 `dec-<VA>.c`。
- 机器码直读：`.pdata` 函数边界、`.text` `e8` 直接调用扫描（本文件 §1/§5 的调用点）。
- 符号：`evidence/country-core-deep/sysres-procedures.json`（1098 条）与 `-v3.json`；
  `skeleton/B/crates/country-core/src/**` 的 PDB 注释（源行/RVA/依赖注解）。
- 当前树：`crates/country-core/src/startup/mod.rs`、`startup/run_game.rs`、
  `systems/main_camera.rs`、`systems/camera_rig/*.rs`、`systems/save/mod.rs`、
  `crates/tiny-glade/src/run_game.rs`（host 已注册段）。
