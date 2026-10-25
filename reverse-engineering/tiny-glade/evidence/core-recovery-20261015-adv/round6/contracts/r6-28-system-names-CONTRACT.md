# Round6 CONTRACT — 收官批次：28-系统逐名 + on_event 应用对象

> 逆向 agent ① 交付。证据：procs.tsv into_configs（G1/G2 系统面）+ public-session-names（on_event）。
> 状态：**28-系统名定案可直接落地（供宿主）；on_event 应用对象定案**。

---

## 1. 28-系统逐名（O，into_configs 权威）

**G1 = FUN_140a2bf40（10 系统，资产载入组）**：
1. **load_textures**（AssetTextureLibrary+TextureWatch+GameConfig+GraphicsDeviceResource）
2. **load_shader 变体 1**（AssetShaderLibrary+ShaderWatch+GraphicsDeviceResource）
3. **load_shader + FinalPostResources**（+FinalPostResources）
4. **custom-cursor SSBO 载入**（AssetSsboLibrary+CustomCursorSsboCreatedCmd）
5. **load_prefabs**（PrefabWatcher+PrefabLibrary）
6. **config-watch 载入**（AssetSsboLibrary+ConfigWatch+Commands）
7. **纯 Commands 系统**（身份待定）
8. **terrain 高度 + PathRaster**（TerrainHeightsData+PathRaster+Commands）
9. **audio samples promise**（AudioSamplesLoadedPromise+Commands）

**G2 = 0x140839b80（8 系统，启动/渲染组）**：
1. **init_photomode_state**（PhotomodeState）
2. **startup ToD/weather/glade**（TodSequence+Weather+GladeSettings+ActiveSession）
3-5. **mesh 载入族 ×3**（AssetMeshLibrary+MeshWatch+ShaderWatch+…）
6. **color/SSBO**（AssetSsboLibrary+GraphicsDeviceResource+…）
7. **load_textures 变体**（AssetTextureLibrary+ActiveSession）
8. **ScheduledTerrainReadback**（terrain readback 装配）

**G3 = 0x1415f4750（1 系统）**：mesh-atlas 装配（PrefabLibrary+MeshAtlasLibrary+AssetMeshLibrary）。
**G4 = 0x1409c8600（1 系统）**：shader 变体（GraphicsDeviceResource+AssetShaderLibrary+ShaderWatch）。
**G5/G6 = 0x141736760 / 0x140b6bd50**（round5 a 契约 §1：6+6 音频/渲染）。

→ **宿主接线**：GameStartup 28-系统 = G1→G4 顺序 add_systems（Startup），宿主 GameLoop 驱动；
② 供宿主契约用此表（G1/G3/G4 asset 组实体由宿主/D 组提供，不重建）。

## 2. on_event<NewSessionStartedCmd> 应用对象（O，public-session-names entry6 + into_configs）

- `on_event<NewSessionStartedCmd>` run 条件 = bevy 通用 run_if（事件出现才跑）。
- 应用对象 = 消费 NewSessionStartedCmd 的系统（round5 c 契约 §2 表）：`add_preplaced_autoclutter`
  /`reset_input_state`/`detect_overhang_support_segments`/`sheep_unstuck`（各带
  ER<NewSessionStartedCmd>）；`time_of_day`/`input_record_replay` 用
  `resource_exists<NextSession>`（entry7）。
- **② 接线**：会话切换链 = load_game_session 发射 → on_event 门控的消费系统运行。

## 3. 可实现性
- **可直接落地**：§1 28-系统名（宿主契约）、§2 on_event 应用对象。
- **待取证**：纯 Commands 系统身份（G1 #7，低优先）。

## 4. 证据索引
- procs.tsv into_configs：0x140a2bf40/0x140839b80/0x1415f4750/0x1409c8600/0x141736760/0x140b6bd50。
- public-session-names entry6/7（on_event/resource_exists 条件）。
