# T2 CONTRACT — run_game_thread 装配顺序（register_core_startup 依据）

> 逆向 agent ① 交付。目标：从原机器码定出 run_game_thread 0x1405a3440（39KB）的**装配顺序**
> 与 **28-系统全局顺序**，供 ② 的 `register_core_startup` 对齐。
> 方法：RNE 二进制 + capstone 逐指令（脚本 `round4/dis_calls_named.py`）→ 直接调用序 +
> procs.tsv 符号映射。状态：**装配顺序（资源 init 先后）O 可落地；28-系统逐名身份待取证**
> （leaf builder 层次已定位，见 §4）。

---

## 1. 装配顺序（O，capstone 直接调用序，run_game_thread 全程 359 个直接调用）

按序分阶段（地址=调用点）：

```
A. 会话准备（0x1405a3440..）：
   0x1403b5150  NextSession::append_on_load（MetaCommand 三件套）
   0x140d76ed0/0x140d62810/0x140d62e10  文件打开/路径拼接/is_dir（会话目录 walk）
   0x140b46000  save_folder_path 族；0x1403107e0 PathBuf 族
   0x140b46920  NextSession::load；0x140b46a20 NextSession::ignore_snapshot
   0x140522ad0  GameConfig 载入；0x140d59140 ToD 载入；0x140d59430 snapshot 载入
B. 资源 init（0x1405a3ce0..；每个 `call <ctor>; call 0x1400dd6f0(get_resource_id)` = init_resource）：
   GameConfig > VideoSettings > PanicReproState > GameSystemSet > SystemSet 族
   （0x140119810/0x1405fdb80/0x1401209b0/0x1401525a0/0x140364d20/0x1400d67b0/0x1403276f0）
   → spawn_hang_watchdog_thread（0x140365be0）
   → add_stages（0x140945ed0，bevy Schedule 分阶段）
   → StatesPlugin（0x14032b3f0）/ SplashScreenPlugin（0x1400cd450）注册
   → 事件注册（Event::register，`call <Event>; call 0x1400dd6f0`）：
     OsCursorCaptureCmd、OsCursorVisibilityCmd、CustomCursorSsboCreatedCmd、OsWindowCmd、
     WriteSettingsFileCmd、WindowFocusEvent、InputCursorDelta、InputCursorMoved、
     ShutdownGameCmd、IccProfileChanged、PlayMenuUiSound
   → 资源 init 继续：FrameRenderConfig、AdaptiveRenderState、WindowSize、GameTime、GameConfig、
     AudioCore、RuntimePlatform、ConfigWatch、RenderTemporalState、
     AssetShaderLibrary(new 0x140936c60)、AssetTextureLibrary、AssetMeshLibrary、
     AssetSsboLibrary、ViewConstants、FrameConstants、MeshWatch、ShaderWatch、TextureWatch、
     AssetVAOLibrary、DynamicConstantsResource、GraphicsDeviceResource、
     AssetPlacementDataLibrary、SavedSessionList、ImageFormationConfigs、UiNavState
   → 0x14093fa60 `default` ×60（Vec/数组默认构造，身份待定）
   → ActionSetStates、ActiveInputModeTracker、AutoSaveState、GraphicsDeviceResource(2)、
     SubApp、GameEvent、CliArgs、EguiEventIo、VideoSettings、App、AsyncLoadedAudioResources、
     MetaCommander
C. 线程派发（0x1405a5f88/0x1405a614e）：`call 0x1404d8100 new; call 0x1404d8250 run` ×2
   （两个后台线程：save_thread/渲染线程候选，身份 I）
D. 收尾：SessionOnLoadCmd、NextSession、GameConfig 族 + panic 冷路径
```

**② 的 register_core_startup 应还原顺序**（本地可落地部分）：会话流资源（SavedSessionList/
ActiveSession/NextSession/AutoSaveState）→ MetaCommander → GameConfig/VideoSettings →
事件注册（窗口/光标/关闭/输入游标族）→ Asset 库族 → 域资源（terrain/save/audio/history）→
**两后台线程**（save_thread = init_save_thread，run_game_thread 里已有对应 spawn）。

## 2. 事件注册的"写回模式"（`*[T+0x10] = [T+0x1c]`）

run_game_thread 中多个 ResMut 资源（NextSession、EventWriter 状态、GardenAudioState 同构）
出现 `*(undefined4 *)(res[2]) = *(undefined4 *)((longlong)res + 0x1c)` 模式 = 把 +0x1c 的 u32
写回 +0x10 所指地址。此为 bevy Events/Resource 状态同步惯用（GardenAudioState 同款，
见 T1 契约 §4）。② 落地时按 bevy 0.16 原生 Events 双缓冲即可，无需手工复刻。

## 3. Schedule 库内执行桥（O）

- `add_game_startup_systems` 0x140941110（134B）注册序：FUN_140a97110 建 28-系统 tuple →
  App::add_systems(Startup)（0x140b67620）→ 子插件 build（0x141740a20）→ add_systems(state=1)
  ×2（0x140b66830/0x140b66480）→ 插件注册（0x140b67b50）→ hash 查表（0x1400dd6f0）panic 守卫。
- run_game_thread 的 `add_stages`（0x140945ed0）= bevy Schedule 分阶段装配；两后台线程经
  `0x1404d8100 new + 0x1404d8250 run`。
- 既有 C 组已按此接入 run_game.rs（register_core_startup）；本契约确认顺序与原生一致。

## 4. 28-系统全局顺序（部分 O，逐名待取证）

- 28-系统 tuple 构建器 FUN_140a97110（startup/decomp/decomp-0x140a97110.txt）调 4 个顶层 builder：
  FUN_140ce4b10 / FUN_140a2bf40 / FUN_1415f4750 / FUN_1409c8600。
- FUN_140a2bf40（0x460B 聚合器，decomp-builder-0x140a2bf40.txt）再聚合 10 个叶子 builder：
  FUN_140ce4b10 / FUN_14179f4e0 / FUN_14179fa20 / FUN_1408ae240 / FUN_140a04b30 /
  FUN_1413fb780 / FUN_140a5f150 / FUN_140af3ac0 / FUN_1408c6e60 / FUN_140a9f5c0。
- 已知系统（O）：`AssetTextureLibrary::create_preset_textures`（串 0x142adaf38,91 字符）；
  audio 组经 add_audio_startup_systems（PDB inline，audio/mod.rs:144）。
- **待取证**：28 个系统 fn 指针的逐名身份 = 需再解叶子 builder（每个写 16B {fn_ptr,cfg} 条目，
  fn_ptr 经 capstone 直读 + procs.tsv 映射即可定名；本轮未逐叶解）。

## 5. 可实现性

- **可直接落地**：§1 装配顺序（资源 init 先后 + 事件注册集 + 两后台线程 + add_stages），
  §3 Schedule 桥。② 按此整理 register_core_startup 的 init/insert 顺序。
- **待取证**：28-系统逐名（叶子 builder 解出后即可）；0x14093fa60 `default`×60 的身份；
  两后台线程（0x1404d8100/0x1404d8250）的精确角色。

## 6. 证据索引
- 脚本：round4/dis_calls_named.py（RNE+capstone+procs.tsv 映射，可直接复用解叶子 builder）。
- decomp：startup/decomp/decomp-0x140a97110.txt、round4/decomp-builder-0x140a2bf40.txt、
  round4/decomp-plugin-0x140941110.txt（add_game_startup_systems）、0x1409455f0（add_terrain_systems）。
- procs.tsv：0x140941110/0x1409455f0/0x1409411a0/0x140941450 全名确认。
