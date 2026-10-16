# T2 CONTRACT — load_game_session（会话切换系统）与 NewSessionStartedCmd/SessionLoadedCmd 生产者

> 逆向 agent ① 交付。目标：把 `load_game_session 0x14166e7d0`（78KB）的**完整系统契约**
> （SystemParam 面 + 行为 + 事件生产者）与"会话切换系统"定案，供 ② 落地为真实 ECS 系统。
> 状态：**SystemParam 面 + 事件生产/消费可直接落地**；精确 theme/attempt 字段语义局部待取证。

---

## 1. SystemParam 面（PDB 权威，`evidence/core-recovery-20261015/session/public-session-names.txt`）

`load_game_session` 是 bevy 0.16 系统，完整 13-param 元组（InfallibleSystemWrapper<FunctionSystem<...>>
drop_in_place 符号 + FunctionSystemState 符号双重确认）：

```
fn load_game_session(
    saved_list:      Res<SavedSessionList>,
    active_session:  Option<ResMut<ActiveSession>>,
    game_profile:    Res<GameProfile>,
    auto_save:       Res<AutoSaveState>,
    next_session:    ResMut<NextSession>,
    load_mgr:        ResMut<LoadGameManager>,
    app_mode:        ResMut<AppMode>,
    glade_settings:  ResMut<GladeSettings>,
    meta_cmdr:       ResMut<MetaCommander>,
    new_session_evt: EventWriter<NewSessionStartedCmd>,
    session_loaded:  EventWriter<SessionLoadedCmd>,
    corrupt_warn:    EventWriter<DisplayCorruptedSaveFileWarning>,
    commands:        Commands,
)
```
- decomp 0x14166e7d0 参数数 = 13，与 PDB 元组完全一致（`func_0x000142923c00(&buf, param_5, 0xa0)`
  = **NextSession 160B(0xa0) 拷贝** → param_5 确为 ResMut<NextSession>）。
- **生产者定案**：`load_game_session` 就是 NewSessionStartedCmd + SessionLoadedCmd +
  DisplayCorruptedSaveFileWarning 的生产者（PDB EventWriter 三件套）。

## 2. 行为契约（O：startup ALGORITHMS §1.2 + 章节 startup_session.rs.md + decomp）

```
1. 读会话尝试：SavedSessionList/attempt @+0x10 空 → theme 默认 3(Autumn)；否则 theme byte@+6
2. 主题 jump-table 0x142c84d24（niche ^ 0x8000…；case0 → 'internal error: entered unreachable code'
   panic；case 1/2 → 继续/新会话）
3. 继续会话（case1）：FUN_140a9fa80 判有效 → log 'Latest game session '/ 'Continuing game session '
   → 会话 Vec push（cap-first grow FUN_140562870，8B 元素，计数@+0x40）→ 拼 '<会话>/history.json'
   （'history.json' 0x142c845e7,12）
4. 新会话（case2）：log 'Starting a new game session' → FUN_140c102e0 生成会话 id →
   create_dir（panic 'Creating folder for game session'）→ 写 meta.json（panic 'Writing new session
   meta.json'；theme 小写 summer/olden/flowery/autumn/winter 串 0x142c84cc0 区）→
   起始 build：demo flag 0x143634d90 ? 'assets/starting-builds/demo' : 'assets/starting-builds/full'
   → 内联 copy_dir_all（"intro" 子目录跳过；排序后逐项复制）
5. 载入：meta.json（失败 log 'Failed to load session meta. Using default. Error: '）→ history.json
   → snapshots → screenshot.jpg；'Loading a pre-parsed game session' log
6. 事件发射：成功载入/新建 → EventWriter<NewSessionStartedCmd> + EventWriter<SessionLoadedCmd>；
   载入失败 → EventWriter<DisplayCorruptedSaveFileWarning>
```

**待取证**：`SavedSessionList@+0x10` 的 attempt 载荷精确字段（theme byte@+6 与 +0x10 空判）；
`NewSessionStartedCmd`/`SessionLoadedCmd` 事件的**载荷字段**（事件类型在 `resources/session.rs`
现有结构，载荷面 U，见 session-DELIVERY 仍缺项）。

## 3. 会话切换消费者（NewSessionStartedCmd/SessionLoadedCmd 的消费系统，public-session-names 全量）

| 系统 | SystemParam 面（O，PDB） | 运行条件 |
|---|---|---|
| `time_of_day::time_of_day` | (Res<GameTime>, Res<AppMode>, Res<Toolbar>, Res<MainCamera>, ResMut<GameConfig>, ResMut<WindowSize>, ResMut<CursorPositionSS>, ResMut<ActionSetValues>, ResMut<InputRecordReplayState>, EventReader<EscapeMenuJustClosed>, Option<Res<NextSession>>) | resource_exists<NextSession>（entry7） |
| `input_record_replay` | (Res<GameTime>, Res<AppMode>, Res<Toolbar>, Res<MainCamera>, ResMut<GameConfig>, ResMut<WindowSize>, ResMut<CursorPositionSS>, ResMut<ActionSetValues>, ResMut<InputRecordReplayState>, EventReader<EscapeMenuJustClosed>, Option<Res<NextSession>>) | 同（entry11） |
| `add_preplaced_autoclutter` | (Res<UiSignifierStream>, Res<PreplacedAutoClutter>, ResMut<AutoClutterState>, EventReader<NewSessionStartedCmd>, EventReader<SessionLoadedCmd>, Local<FramesSinceNewSession>) | — |
| `reset_input_state` | (ResMut<ActionSetValues>, EventReader<NewSessionStartedCmd>, EventReader<SessionLoadedCmd>, ResMut<ButtonInput<KeyCode>>) | — |
| `detect_overhang_support_segments` | (EventReader<OnWallChanged>, EventReader<RecalculateWallBricks>, ResMut<OverhangSupportDirtySubscriptions>, Res<AssetSsboLibrary>, Res<PublicWalls>, Query<&Roof>, Res<ShapeIntersections>, EventWriter<RecalculateRectangleSupports>, EventWriter<RecalculateCircleSupports>, ResMut<ElevationSupportStats>, ResMut<StoneColumnCollisionStorage>, EventReader<NewSessionStartedCmd>, EventReader<SessionLoadedCmd>, Local<DelayOnLoad>, EventWriter<RecalculateWoodenTurretCmd>) | — |
| `sheep_unstuck` | (Query<(&Transform,&mut CritterLocomotion,&mut Sheep)>, Res<AssetSsboLibrary>, Res<WaterRaster>, Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<GameTime>, Res<RaycastWorld>, Local<FramesSinceNewSession>, EventReader<NewSessionStartedCmd>, EventReader<SessionLoadedCmd>, Res<UiSignifierStream>, Res<ActiveSession>) | — |
| `on_event<NewSessionStartedCmd>` | run-condition `fn(EventReader<NewSessionStartedCmd>) -> bool` | entry6 |

→ 会话切换 = load_game_session 发射两个事件，上述系统各自反应（重置输入、autoclutter 预置、
屋顶支撑重算、sheep 脱困、ToD/回放恢复）。② 落地 register_core_startup 时，这组消费系统的
**事件接线**即"会话切换链"。

## 4. 当前源对照与②待办

- `resources/session.rs` 现有 `SessionFlowPlugin`、`NextSession`、事件类型；`load_game_system`
  （save/load_game.rs）已消费 NextSession。**缺**：load_game_session 未实现为系统（session-DELIVERY
  登记"当前 load_game_session 纯函数近似 + advance_load_game"）。
- ② 应把 load_game_session 落地为 13-param 系统（可先以 SessionFlowPlugin 已有资源承载），
  并按 §2 行为发射 NewSessionStartedCmd/SessionLoadedCmd。
- sheep_unstuck/add_preplaced_autoclutter 等消费系统多数属库外（system_critters/
  system_clutter）或本 crate 待恢复域——② 只需提供事件 + 注册点，不实现外部 crate 系统。

## 5. 证据索引
- PDB：public-session-names.txt（entries 4-15，含 SessionLoadAttempt VecDeque drop、
  on_event/resource_exists 条件、6 消费系统 SystemParam）。
- decomp：expansion-b2/decomp/decomp-0x14166e7d0.txt（13 参、NextSession 0xa0 拷贝、
  param_5[2] 写回模式）；startup/decomp + ALGORITHMS §1.2（行为）；章节 startup_session.rs.md。
- 串：0x142c84940/78/18/730/698/b8（Creating/Writing/Continuing/Latest/Loading a pre-parsed/
  snapshots）、0x142c84625（unreachable）、0x142c84c40（Failed to load session meta）、
  0x142c845e7（history.json）、0x142c849b0/cb（full/demo）、0x142c84cc0 区（theme 小写名）。
