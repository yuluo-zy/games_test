# T2 调度器/startup 装配 + load_game_session 会话切换系统 DELIVERY（2026-10-16 第四轮 ②）

> 实现对象：① T2 两份契约：
> round4/contracts/t2-load_game_session-session-switch-CONTRACT.md（load_game_session
> 13-param SystemParam 面 + NewSessionStartedCmd/SessionLoadedCmd/DisplayCorruptedSaveFileWarning
> 生产者 + 会话切换消费者接线）与
> round4/contracts/t2-run_game_thread-register_core_startup-CONTRACT.md（run_game_thread
> 359 调用序装配对齐 + 事件注册集 + Schedule 桥）。
> 文件（全部在 ② 文件权内）：`startup/startup_session.rs`、`startup/run_game.rs`、
> `game_profile.rs`（若需）。**未修改** shared `resources/session.rs`/`systems/save/load_game.rs`
> （仅 import 类型）。

## 真实代码接线

1. **`load_game_session` 13-param ECS 系统**（`startup/startup_session.rs`，O：0x14166e7d0，
   PDB public-session-names 13-param 面 + decomp 13 参核对）：
   - 签名：`(Res<SavedSessionList>, Option<ResMut<ActiveSession>>, Res<GameProfile>,
     Res<AutoSaveState>, ResMut<NextSession>, ResMut<LoadGameManager>, ResMut<AppMode>,
     ResMut<GladeSettings>, ResMut<MetaCommander>, EventWriter<NewSessionStartedCmd>,
     EventWriter<SessionLoadedCmd>, EventWriter<DisplayCorruptedSaveFileWarning>, Commands)`
     + `Local<bool>` 门（每次游戏启动跑一次；后续会话切换经 NextSession → load_game_system）。
   - 行为（① 契约 §2，可直接落地部分）：
     a. attempt = `SessionLoadAttempt::continue_latest`（列表空 → 新会话，theme 默认 Autumn，O）。
     b. 新建（case2）：log "Starting a new game session" → `SessionId::new` → `create_dir_all`
        → `SessionMeta::write_to_disk`（theme = attempt.theme）→ `copy_dir_all`
        （STARTING_BUILD_FULL，demo 旗标 0x143634d90 宿主侧，best-effort）。
     c. 继续（case1）：log "Continuing game session {id}" → 就绪检查（`loading.tmp` 存在 /
        `history.json` 缺失 → `DisplayCorruptedSaveFileWarning` + 早退）。
     d. 载入 meta.json → `ActiveSession`（失败 log "Failed to load session meta. Using
        default. Error: " + `SessionMeta::default()` 回退，O）；载入 settings.json →
        `GladeSettings::load(&folder)`。
     e. 事件发射：成功 → `NewSessionStartedCmd` + `SessionLoadedCmd`（生产者定案，① §1）。
   - **`GameProfile` 补 `Resource` derive**（`game_profile.rs`，② 文件权）：native
     `Res<GameProfile>`（load_game 0x14146d030 第 3 参 / startup_session，O；文件头注释
     已有真实消费方登记）。Resource 为空标记 trait，无行为影响。
2. **`register_core_startup` 装配对齐**（`startup/run_game.rs`，① T2 run_game_thread §1）：
   - 顺序还原（359 调用序 O 的本地可落地部分）：**会话流 → MetaCommander → GameConfig →
     事件注册 → 域资源/系统 → 后台线程**。
   - 新增：`world.init_resource::<GameConfig>()`（native 0x140522ad0 载入；crate 语义结构
     Default）；`register_input_events(world)`（输入游标族事件/资源，run_game_thread §1 事件
     注册集）；`schedule.add_systems(startup_session::load_game_session)`。
   - 移除冗余的重复 `init_resource::<LoadGameManager>`（add_save_load_events_and_resources
     已 init）。
   - 保留：SessionFlowPlugin.build、MetaCommander + configure_session_switch_systems、
     add_save_load_events_and_resources、register_history_events、music_audio_init_system、
     add_history_cpu_systems、add_save_load_systems_impl（含 init_save_thread 后台线程）、
     terrain 事件/系统。

## candidate 链测试

- `load_game_session` 为 ECS 系统，整 ECS 运行测试需宿主注入 GameProfile/AutoSaveState/
  NextSession 等资源；startup_session.rs 既有 copy_dir_all/starting_build_dir 测试保留。
- 本文件在 lib 类型检查中零错误；整 crate `cargo check` 剩 1 错误在 `systems/prefab.rs`
  （`crate::resources::prefab` 未声明 pub mod——用户工作树未提交的新文件，非本组），
  本组文件零错误。

## 原侧验证

- decomp-0x14166e7d0：13 参（`func_0x000142923c00(&buf, param_5, 0xa0)` = NextSession 160B
  拷贝 → param_5 确为 ResMut<NextSession>）；procs.tsv public-session-names 13-param 面逐参对齐。
- ① 契约 §2 行为串：'Starting a new game session' / 'Continuing game session ' /
  'Failed to load session meta. Using default. Error: '（0x142c84940/78/18/730/698/b8、
  0x142c84c40）；theme 小写名（0x142c84cc0 区 = GladeType::assets_folder）。
- 就绪检查：`check_load_completion` 0x140a9f810（loading.tmp）/ `is_session_ok_to_load`
  0x140a9fa80 族；`SessionLoadAttempt`（startup_session 内联行 79）+0x10 空判 → theme 默认 3。
- run_game_thread 0x1405a3440 装配序（capstone 359 直接调用，① §1）：事件注册集
  （InputCursorDelta/InputCursorMoved 等在 crate 已注册）、GameConfig 载入 0x140522ad0、
  两后台线程（save_thread = init_save_thread 已在 add_save_load_systems_impl 接线）。

## 仍缺项（待取证，登记不补造）

1. `load_game_session` 面内消耗：`GameProfile`/`AutoSaveState`/`LoadGameManager`/`AppMode`/
   `MetaCommander` 在 78KB 内联中的精确消费待取证（现 underscore 登记，不伪造）；`Commands`
   同理。主题跳表 0x142c84d24 以 `SessionMetaSource` 二分表达；`SavedSessionList@+0x10`
   attempt 字段（theme byte@+6）待取证。
2. 会话切换消费者 6 系统（① 契约 §3）：`reset_input_state`（crate 内，F2 待接入）、
   `time_of_day`/`input_record_replay`（crate 内，宿主资源面待齐）、`add_preplaced_autoclutter`/
   `detect_overhang_support_segments`/`sheep_unstuck`（库外）——按契约"② 只提供事件 + 注册点，
   不实现外部 crate 系统"，事件已就绪，消费端待各域接入。
3. 28-系统逐名（叶子 builder 解出后）；`0x14093fa60 default×60` 身份；两后台线程精确角色。
4. Asset 库族（AssetShaderLibrary 等）/ VideoSettings / OS 窗口事件注册：宿主/D组 rhapsody
   装配面（GraphicsDevice 依赖），不在 register_core_startup 伪造。
5. copy_dir_all 起始 build 的 demo 旗标（0x143634d90）宿主侧待接。

## 组外申请记录

- 无：三文件（startup_session.rs、run_game.rs、game_profile.rs）均在 ② 文件权；shared
  `resources/session.rs` / `systems/save/load_game.rs` 仅 import 类型未修改。协调器提示的
  共享文件仲裁不触发。
