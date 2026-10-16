# Round5 A CONTRACT — ECS 调度器整合（GameStartup 系统序 + ivy 22 + 域注册点）

> 逆向 agent ① 交付。证据：procs.tsv `into_configs` 类型符号（**权威系统面**）+ capstone 注册结构
> （0x140941110/0x140944070/0x140b6d290/0x140a97110）。
> 状态：**系统名 + 注册序可直接落地；28-tuple 槽位精确映射需再取证（低优先）**。

---

## 1. GameStartup 28-系统 = into_configs 组（O，procs.tsv 权威）

`add_game_startup_systems`（0x140941110）注册序：FUN_140a97110 建 tuple → add_systems(Startup)
（0x140b67620）→ 子插件 build → add_systems(state=1)×2 → 音频组。系统分组（每组的 into_configs
符号 = 系统面权威）：

**G1：FUN_140a2bf40（10 系统，`impl$18::into_configs`，void(*)( 20 处）**——资产载入组：
1. `(R<GraphicsDeviceResource>, R<GameConfig>, RM<AssetTextureLibrary>, RM<TextureWatch>, Commands)` = **load_textures**
2. `(R<GraphicsDeviceResource>, R<AssetShaderLibrary>, R<ShaderWatch>)` = **load_shader 变体**
3. `(同上 + RM<FinalPostResources>)` = **load_shader + FinalPost 资源**
4. `(R<GraphicsDeviceResource>, R<AssetSsboLibrary>, EW<CustomCursorSsboCreatedCmd>)` = **SSBO/光标 SSBO 载入**
5. `(Option<RM<PrefabWatcher>>, RM<PrefabLibrary>)` = **load_prefabs**
6. `(R<GraphicsDeviceResource>, R<AssetSsboLibrary>, RM<ConfigWatch>, Commands)` = **config-watch 载入**
7. `(Commands)` = 纯 Commands 系统（身份待定）
8. `(RM<TerrainHeightsData<TerrainHeightSsbo>>, RM<PathRaster>, Commands)` = **startup terrain 高度/PathRaster**
9. `(RM<AudioSamplesLoadedPromise>, Commands)` = **audio 采样 promise**
10. （第 10 系统面被截断，同组）

**G2：0x140839b80（8 系统）**——启动/渲染：
1. `(RM<PhotomodeState>)` = **init_photomode_state**
2. `(RM<TodSequence>, RM<Weather>, R<GladeSettings>, R<ActiveSession>)` = **startup ToD/weather/glade**
3-5. `(R<GraphicsDeviceResource>, R<AssetMeshLibrary>, R<MeshWatch>, Commands, R<AssetShaderLibrary>, R<ShaderWatch>, R<ActiveSession>)` ×3 = **mesh 载入族**（load_mesh_atlases/load_prefabs/placement）
6. `(R<ActiveSession>, R<GraphicsDeviceResource>, RM<ColorLibrarySsbo>, R<AssetSsboLibrary>)` = **load 颜色**
7. `(R<ActiveSession>, RM<AssetTextureLibrary>)` = **load_textures 变体**
8. `(R<ScheduledTerrainReadback>)` = **ScheduledTerrainReadback**

**G3：0x1415f4750（1 系统）**：`(R<PrefabLibrary>, RM<MeshAtlasLibrary>, RM<AssetMeshLibrary>, …)` = **mesh-atlas 装配**
**G4：0x1409c8600（1 系统）**：`(R<GraphicsDeviceResource>, R<AssetShaderLibrary>, R<ShaderWatch>, …)` = shader 变体

**G5：0x141736760（6 系统）**：music_audio_init / image_formation / mesh·shader 载入族。
**G6：0x140b6bd50（6 系统，音频）**：timberframe / elevation_supports / clutter / forest / tree /
firstperson_steps（`PathRastersResources` SystemParam）——即 `add_audio_systems` 0x1409411a0 组。

→ **② 待办**：按 G1→G4 顺序把 asset 载入组并入 `add_game_startup_systems`；G6 并入
`add_audio_systems`。28-tuple 的槽位精确映射（FUN_140a97110 表 0x1c0B 到各组的分配）需再取证
（低优先，不影响系统名与顺序）。

## 2. ivy 22-系统（O，add_ivy_systems 0x140944070）

```
0x140944070: mov dl, 0x16(22) → call 0x141664d80（tuple 构建）→ call 0x141450fa0 → App::add_systems(0x140b67280)
```
已解系统面（into_configs 符号）：
- **ivy_grower**（0x1416643e0）：`(RM<IvyStorage>, RM<IvyLeafStorage>, R<PublicWalls>, R<WallSpatialHash>, R<PrevWallHoles>, ER<CmdCheckForSheepInsideBuilding>, ER<OnCircleRotated>)`
- **ivy_pruner**（0x1416645b0）：`(RM<IvyStorage>, RM<IvyLeafStorage>, ER<RemoveIvyCmd>, R<UiSignifierStream>)`
- **ivy_leaf_spawner**（0x141664930）：`(ER<IvySegmentDoneGrowingMsg>, RM<IvyStorage>, RM<IvyLeafStorage>, RM<IvyLeafRng>, R<IvySpawnNoise>, R<GameTime>, R<GladeSettings>)`
- **ivy_spawner**（0x141664d80 `after_ignore_deferred`，GameSystemSet）：`(RM<IvyStorage>, R<IvySpawnNoise>, R<PublicWalls>, R<CursorTerrainRaycast>, R<ActionSetStates>, R<WallSpatialHash>, R<GladeSettings>)`

→ ② 补 `register_ivy_storage` 时按上面对齐系统签名（现 crate 7 系统链 + send_to_gpu×2 + ivy_audio
缺）。其余 22 中未列系统（动画×2/send_to_gpu×2/ivy_audio/wall_coord/pruner_cmd/remove_ivy_at_location）
仍待逐个 into_configs 补（低优先）。

## 3. garden 插件 build（0x140b6d290，O）

```
0x140b6d2c0 call 0x14176a580(dl=0x10=16 系统组, 串 0x1f86b29)
0x140b6d2d5 call 0x1409a1ce0(dl=0x22=34 系统组, 串 0x1f86b2c)
→ App::add_systems(0x140b679c0)
→ 3× get_resource_id(hash) 条件注册：0x1dbf78e0d9cd349f→0x141517160、
  0x30caeddf11f1ff28→0x1415098d0、0xfbbe0392355af491→0x14150e940
```
（16+34 系统 = garden 全链；系统名需解 0x14176a580/0x1409a1ce0 的 into_configs，中优先待证。）

## 4. sheep 注册 = `system_critters::sheep::SheepPlugin`（O，procs.tsv）

Plugin::build 地址待从 SheepPlugin 的 build 符号定位（0x1404914d0 区为 Plugin trait 胶水）；
sheep 系统属 `system_critters` crate（库外），② 只接事件/资源。

## 5. input 注册（O，部分）

input 系统组经 into_configs（0x14168b370/0x14168b890/0x14168ba60 等）；注册点
`SubApp::add_systems`（0x140b67b50）。精确插件 build 地址待取证（中优先）。

## 6. Schedule 库内执行桥（O 定案：GameLoop）

- run_game_thread 装配两处 `call 0x1404d8100 new; call 0x1404d8250 run` = **`tiny_glade::game_loop::GameLoop::{new,run}`**
  （procs.tsv：0x1404d8100 = GameLoop::new 327B、0x1404d8250 = GameLoop::run 4899B，属 `tiny_glade` crate 模块 194）。
- run_game_thread 先 `add_stages`（0x140945ed0，bevy Schedule 分阶段）→ 装配资源/事件 → **spawn GameLoop
  线程（0x1404d8100+0x1404d8250 ×2）**。bevy Schedule 的逐帧执行 = `GameLoop::run` 内部
  （4899B，含 `Schedule::run` 执行桥调用点）。
- **② 宿主接线模型**：GameLoop = 宿主（tiny-glade crate）线程；country-core 只保证
  `add_game_startup_systems`/各域 add_*_systems 注册到 Schedule，宿主 GameLoop 驱动。
  Schedule::run 内部调用点（bevy_ecs 0.16 executor）由宿主态提供，不重建。

## 7. 可实现性
- **可直接落地**：§1 系统名与注册序（G1-G4 顺序）、§2 ivy 系统面、§6 GameLoop/Schedule 桥模型。
- **需再取证**：28-tuple 槽位映射、garden 16+34 系统名、input 插件地址、GameLoop::run 内
  Schedule::run 具体调用地址（低优先）。

## 8. 证据索引
- procs.tsv into_configs：0x140a2bf40/0x140839b80/0x14099b8b0/0x141736760/0x140b6bd50/
  0x1415f4750/0x1409c8600（系统面权威）；0x1416643e0/5b0/930/d80（ivy）；
  0x1404d8100/0x1404d8250（GameLoop::new/run）。
- capstone：0x140941110、0x140944070、0x140b6d290、0x140a97110（注册结构）。
- decomp：round4/decomp-plugin-0x140941110.txt、round4/decomp-builder-0x140a2bf40.txt。
