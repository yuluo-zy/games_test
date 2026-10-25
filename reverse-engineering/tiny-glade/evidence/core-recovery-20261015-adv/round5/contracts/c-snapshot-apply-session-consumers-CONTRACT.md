# Round5 C CONTRACT — 快照应用 + 会话消费端接线

> 逆向 agent ① 交付。证据：decomp-r5-0x1417a9c50（init_game_state_from_snapshot 26KB）、
> public-session-names（6 消费系统面）、procs.tsv into_configs（注册组）。
> 状态：**init_game_state_from_snapshot 快照结构面 O；逐域应用闭包细节需再取证（中优先）**。

---

## 1. init_game_state_from_snapshot（0x1417a9c50，O 结构面）

签名 `(param_1, param_2, param_3=snapshot)`。快照结构读面（capstone/反编译，`param_3+偏移`）：

| 偏移 | 宽 | 读面 |
|---|---|---|
| +0x00..0xbF | 0xc0 | **0xc0B 头部整体拷贝**（`func_0x000142923c00(&buf, snapshot, 0xc0)`） |
| +0xc0/+0xd0/+0xdc | 8/8/4 | 域 1 头（计数+指针+标量） |
| +0xe0/+0xf0/+0xfc | 8/8/4 | 域 2 头 |
| +0x100/+0x110/+0x11c | 8/8/4 | 域 3 头 |
| +0x120/+0x130/+0x13c | 8/8/4 | 域 4 头 |
| +0x140/+0x150/+0x15c | 8/8/4 | 域 5 头 |
| +0x160/+0x170/+0x17c | 8/8/4 | 域 6 头 |
| +0x180/+0x190/+0x19c | 8/8/4 | 域 7 头 |
| +0x1a0/+0x1b0/+0x1bc | 8/8/4 | 域 8 头 |
| +0x1c0/+0x1d0/+0x1dc | 8/8/4 | 域 9 头 |
| +0x1e0/+0x1f0/+0x1fc | 8/8/4 | 域 10 头 |
| +0x240/+0x250/+0x25c | 8/8/4 | 域 11 头 |

→ 快照 = 头部 0xc0B + 11 个 {ptr,len,count} 域描述符（每域 0x20B 对齐：8B 指针 + 8B + 4B）。
`func_0x00014067dd80(&UNK_142cdc898)` = AudioCore 客户端 getter（域应用入口候选）。

**逐域应用闭包**：26KB 内为各域的应用代码（颜色/时间/羊/地形），未逐指令拆——**需再取证**
（中优先）：域描述符 → 对应 `ResMut<T>` 写入的映射。② 现 `apply_loaded_snapshot` 空函数按
快照域描述符序实现，逐域写入目标资源即可（资源面=各域资源，与 save 采集器输出对偶）。

## 2. 会话切换消费者 6 系统（O，public-session-names 权威面，交叉引用 T2 契约）

| 系统 | SystemParam 面 | 归属 |
|---|---|---|
| `reset_input_state` | (ResMut<ActionSetValues>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, ResMut<ButtonInput<KeyCode>>) | country_core input |
| `time_of_day::time_of_day` | (Res<GameTime>, Res<AppMode>, Res<Toolbar>, Res<MainCamera>, ResMut<GameConfig>, ResMut<WindowSize>, ResMut<CursorPositionSS>, ResMut<ActionSetValues>, ResMut<InputRecordReplayState>, ER<EscapeMenuJustClosed>, Option<Res<NextSession>>) | country_core time_of_day |
| `input_record_replay` | 同 time_of_day（11 参） | country_core input |
| `add_preplaced_autoclutter` | (Res<UiSignifierStream>, Res<PreplacedAutoClutter>, ResMut<AutoClutterState>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Local<FramesSinceNewSession>) | system_clutter（库外） |
| `detect_overhang_support_segments` | (ER<OnWallChanged>, ER<RecalculateWallBricks>, ResMut<OverhangSupportDirtySubscriptions>, Res<AssetSsboLibrary>, Res<PublicWalls>, Query<&Roof>, Res<ShapeIntersections>, EW×3, ResMut<ElevationSupportStats>, ResMut<StoneColumnCollisionStorage>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Local<DelayOnLoad>, EW<RecalculateWoodenTurretCmd>) | system_wall_constructor（库外） |
| `sheep_unstuck` | (Query<(&Transform,&mut CritterLocomotion,&mut Sheep)>, Res<AssetSsboLibrary>, Res<WaterRaster>, Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<GameTime>, Res<RaycastWorld>, Local<FramesSinceNewSession>, ER<NewSessionStartedCmd>, ER<SessionLoadedCmd>, Res<UiSignifierStream>, Res<ActiveSession>) | system_critters（库外） |

**接线契约**：
- 事件链：`load_game_session` 发射 NewSessionStartedCmd + SessionLoadedCmd（Round4 T2 契约）→
  上述 6 系统各消费对应事件。
- 库内 3 系统（reset_input_state/time_of_day/input_record_replay）注册点 = input/time_of_day
  into_configs 组（procs.tsv 0x14168b370/0x14168ba60 等，A 契约 §5）。
- 库外 3 系统（add_preplaced_autoclutter/detect_overhang/sheep_unstuck）属 system_clutter/
  system_wall_constructor/system_critters crate——② 只保证事件注册 + 资源在场，不实现库外系统。
- ② 待办：`register_input_events` 已有（REVIEW F2 部分缓解）；补 time_of_day/input_record_replay
  注册 + `on_event<NewSessionStartedCmd>` run 条件（public-session-names entry6）。

## 3. 可实现性
- **可直接落地**：快照域描述符结构（0xc0B 头 + 11 域）、6 消费系统面与事件接线。
- **需再取证**：逐域应用闭包的目标资源映射（中优先）、库外系统注册点。

## 4. 证据索引
- round5/decomp-r5-0x1417a9c50.txt（init_game_state_from_snapshot 26KB）。
- public-session-names.txt（6 消费系统 SystemParam）。
- procs.tsv into_configs：0x14168b370/0x14168ba60（input 组）、0x141736760（time_of_day 组候选）。
- Round4 T2 契约：t2-load_game_session-session-switch（事件生产者 + 消费者链）。
