# Round7 CONTRACT — 链3：session 切换消费端 6 系统完整接线

> 逆向 agent ① 交付。证据：public-session-names（6 系统面权威）+ round5 c 契约 §2 + procs into_configs。
> 状态：**6 系统面 + 接线可直接落地；库外注册点宿主面定案**。

---

## 1. 事件链（O）

`load_game_session`（13-param，round4 t2 契约）发射 **NewSessionStartedCmd + SessionLoadedCmd +
DisplayCorruptedSaveFileWarning** → 6 消费系统各读对应事件。

## 2. crate 内 3 系统（country_core，**可直接落地**）

| 系统 | SystemParam 面（public-session-names 权威） | 注册点 |
|---|---|---|
| `reset_input_state` | (ResMut<ActionSetValues>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, ResMut<ButtonInput<KeyCode>>) | input into_configs 组（0x14168b370/0x14168ba60） |
| `time_of_day::time_of_day` | (Res<GameTime>, Res<AppMode>, Res<Toolbar>, Res<MainCamera>, ResMut<GameConfig>, ResMut<WindowSize>, ResMut<CursorPositionSS>, ResMut<ActionSetValues>, ResMut<InputRecordReplayState>, ER<EscapeMenuJustClosed>, Option<Res<NextSession>>) | time_of_day 组（0x141736760 候选） |
| `input_record_replay` | 同 time_of_day（11 参） | input 组 |

- **② 接线**：按上表 SystemParam 实现/注册 3 系统；run 条件 = `on_event<NewSessionStartedCmd>`
  /`resource_exists<NextSession>`（public-session-names entry6/7）。

## 3. 库外 3 系统（system_clutter/wall_constructor/critters，宿主面定案）

| 系统 | SystemParam 面 | 归属 |
|---|---|---|
| `add_preplaced_autoclutter` | (Res<UiSignifierStream>, Res<PreplacedAutoClutter>, ResMut<AutoClutterState>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Local<FramesSinceNewSession>) | system_clutter |
| `detect_overhang_support_segments` | (ER<OnWallChanged>, ER<RecalculateWallBricks>, ResMut<OverhangSupportDirtySubscriptions>, Res<AssetSsboLibrary>, Res<PublicWalls>, Query<&Roof>, Res<ShapeIntersections>, EW×3, ResMut<ElevationSupportStats>, ResMut<StoneColumnCollisionStorage>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Local<DelayOnLoad>, EW<RecalculateWoodenTurretCmd>) | system_wall_constructor |
| `sheep_unstuck` | (Query<(&Transform,&mut CritterLocomotion,&mut Sheep)>, Res<AssetSsboLibrary>, Res<WaterRaster>, Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<GameTime>, Res<RaycastWorld>, Local<FramesSinceNewSession>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Res<UiSignifierStream>, Res<ActiveSession>) | system_critters |

- **②/宿主**：库外系统由对应 crate 实现；country-core 提供事件注册 + 资源在场
  （AutoClutterState/PreplacedAutoClutter/ElevationSupportStats 等 crate 资源），不实现库外系统。

## 4. 可实现性
- **可直接落地**：6 系统面 + 事件链 + on_event/resource_exists 条件 + 库外宿主面。
- **待取证**：库外系统注册点精确 into_configs（宿主侧，低优先）。

## 5. 证据索引
- public-session-names entries 6-15；round4 t2-load_game_session-session-switch（生产者+消费面）。
- procs.tsv into_configs：0x14168b370/0x14168ba60（input）、0x141736760（time_of_day 候选）。
