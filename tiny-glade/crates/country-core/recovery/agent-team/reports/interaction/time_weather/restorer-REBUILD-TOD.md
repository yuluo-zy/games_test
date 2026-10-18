# REBUILD — time_of_day 域 4 函数（restorer）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（分类 + 逐语句判等）
SOP: planning/2026-10-17-recovery-sop.md
证据: **fresh** Ghidra `--worker shared` 反编译 0x140a73460 / 0x140b067d0 /
0x140b18830 / 0x1417ab4b0 / 0x140a53db0；radare2 机器码窗口 0xa736d0..0xa744xx；
常量池 0x142ad58e0..0x142ad5984、0x14362c0b8 直读；binary 字符串表 +
`assets/tod/default.json`（原始数据 schema）。
红线: `remap_hour_to_fmod_hour` / `advance_angle` / `advance_hour` 未改判等语义。

## 分类结论

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `time_of_day`（主系统） | 0x140a73460 | 真实业务 | 重建（13 参真实签名） |
| `advance_playback` | 内联 0xa73b9d..0xa73c39 | 真实业务 | 判等（本轮复核，dev 门来源修正） |
| `advance_tod_hour` | 内联 0xa73c53..0xa73cc3 | 真实业务 | **重建**（原公式错） |
| `suncalc_pos_to_vec` | 内联 0xa73dff..0xa741eb | 真实业务 | **重建**（basis 项错，补次级方向） |

无宏生成胶水。

## 逐函数判等

### 1. `time_of_day` 主系统（0x140a73460）
真实签名（PDB `FunctionSystem` 泛型实参；与反编译栈槽逐一对上）：
`Res<AudioCore>, Res<TodSequence>, Res<Weather>, Res<GameTime>,
Res<ButtonInput<KeyCode>>, ResMut<DirectionalLight>, ResMut<TimeOfDay>,
ResMut<DerivedAtmosphere>, ResMut<GameConfig>, EventReader<NewSessionStartedCmd>,
EventReader<LoadingGameComplete>, ResMut<TimeOfDayScreen>, Res<ActiveSession>`。

逐段判等（全部按 fresh 机器码/反编译重写）：
1. 音频：`AudioCoreInner` 非空 → `set_parameter("TOD-Enabled", dev_hotkey?1.0:0.0)`；
   再判非空 → `set_parameter("Timetravel", screen.instant?1.0:0.0)`（0xa7373c..0xa737ba；
   字符串 @0x142ad63ef/0x142ad63fa 直读）。
2. angle：`screen.angle_transitioning` 时 `tod.sun_xz_rotation` 向
   `2π − screen.sun_xz_rotation` 插值（`expf(-3·dt)`、wrap ±π、收敛 <0.01 清
   `screen` 标志）；未过渡且未小时过渡时 `screen.sun_xz_rotation = 2π − tod.angle`。
3. hour：三态 `screen.hour_transitioning` / `screen.instant`
   （平滑 `expf(max(dev,1)·dt·-3)` + wrap±0.5；instant 写回
   `tod.time_of_day = fmod(target+(1-exp(-dt))·wrap±0.5(screen-target),1)`；
   收敛比较对象是写入前的 screen 值——原 Rust 写法错，已修正）。
4. 事件：`EventReader<NewSessionStartedCmd>` / `EventReader<LoadingGameComplete>`
   各 `read().last().is_some()`；两者都空 → `time_continuous_lerp_mut`，否则吸附
   （0xa73aaa..0xa73b4f；两个集合查询的 `setbe` 标志语义已核为“迭代器为空”）。
5. 热键：`DEVELOPER_HOTKEYS_ENABLED == 1 && just_pressed(KeyCode 0xa9)`
   → `GameConfig+0xe3 ^= 1`（0xa73b52..0xa73b9d；0xa9 = KeyCode::F10，
   按 Bevy 0.16/0.19 变体顺序 160=F1…169=F10）。
6. `advance_playback`，dev 门 = `GameConfig+0xe3`（**原 Rust 用 dev_speed_mult!=0，
   门源错，已修正**）。
7. `advance_tod_hour`；门 `screen.hour_transitioning && screen.instant` 时跳过。
   `hour = fmod(t·24+8,24)`；`remap_hour_to_fmod_hour(hour, ActiveSession::is_winter)`
   → `set_parameter("DayNight", remapped)`（0xa73ce5..0xa73d12；**原 Rust 未读
   ActiveSession，且 DayNight 值错**）。
8. `day_of_year` 边界 `(day-1) as u16 < 365` panic；2023 序号日期（epoch 日 19358）
   + `hour·3600000` → `suncalc::get_position(ms, lat, 0.0)`；`colatitude`；
   `snapshot_at_tod(t)`；全局 `speed_bias` 写回；昼夜段
   `sunrise <= t < sunset` → `inverse_lerp`（否则 0）；昼夜连续值；
   `DirectionalLight::set_direction_towards_light`（day=主方向 / night=次级方向）；
   DerivedAtmosphere 写盘（见登记）。

### 2. `advance_playback`
判等：dev 关恒 `Rewinding(2)`、dev 开恒 `Playing(1)`；旧 state 决定 progress：
Idle→0；Playing dev 开 `+=dt/3` clamp≤1、dev 关不变；Rewinding dev 关 `+=dt/−5`
clamp≥0、dev 开不变。调用点 dev 门已改为 `TodDevConfig.dev_hotkey`（=GameConfig+0xe3）。

### 3. `advance_tod_hour`
判等重建（0xa73c53..0xa73cc3）：
```
dt_speed = dt · TIME_OF_DAY_SPEED        // 0x14362c0b8 = 0x3ada740d = 1/600
inc = exp2f(speed_bias_global) · (progress · dt_speed) · GameConfig+0x118
target' = fmodf(inc + target + 1.0, 1.0)  → rem_euclid(1.0)
```
`speed_bias_global` = 上一帧 `TodLightingSnapshot.speed_bias`（静态 0x143634d84），
**不是 progress**（原 Rust 用 `2^progress·progress…`，错）；TOD_SPEED 不再硬编码 1.0。

### 4. `suncalc_pos_to_vec`
判等重建（0xa73dff..0xa741eb）：`colatitude=π/2−(az+π)`；主方向
`f44=-sin(colat)cos(elev)`、`f43=sin(elev)`、`f29=cos(colat)cos(elev)`，
`orbit_angle` 半角 Y 旋转（`cos_a=c²−s²`、`sin_a=2cs`）、`y+=-0.15`、
`1/sqrt(z²+y²+x²)` 归一化。次级方向由 basis `f41/f42/f32s` → `N/M` 段 →
半角旋转，原样（不归一化）写光照。
**修正**：`f32s = s·0.647056·0.8340785 + s·0.7624425·0.5516458`（原 Rust 写成
`(1−c + s·(−0.7624425))·F`，与机器码不符）；常量改为常量池逐位值。

## 跨域/宿主登记（解锁条件）

| 原接口 | 当前 seam | 解锁条件 |
|---|---|---|
| `Res<Weather>`（+0x00 f32，加到 DerivedAtmosphere 一个标量槽） | `TodWeatherSeam` | `resources/weather.rs` 落地后换回 `Res<Weather>` |
| `ResMut<DerivedAtmosphere>`（0x1f×f32 写盘） | `TodLightingSeam` | `render::atmosphere::DerivedAtmosphere` 落地；按报告偏移表写入 |
| `ResMut<TimeOfDayScreen>`（+0x00 t，+0x04 angle，+0x0a/+0x0b/+0x0c 三个 u8） | `TodScreen` | `gui::TimeOfDayScreen` 补齐上述字段（现仅 `visible`；`+0xa` 注释与机器码冲突，需 gui 域裁决） |
| `ResMut<GameConfig>`（+0xe3 u8、+0x118 f32） | `TodDevConfig` | `GameConfig` 增两字段后换回 |
| `Res<ButtonInput<KeyCode>>::just_pressed(F10)` | `TodInputSeam` | 接 `bevy_input` 后换真实参数；判别值 0xa9 已核 |
| `suncalc::get_position`（crate 0.4.0） | `sun_position_at`（逐语句等价） | 入锁依赖后可直接替换；算法/常量已核 |
| `TodSequence` 数据（`assets/tod/default.json`） | `TodSequence` 空默认 | 启动模块 `startup_time_of_day::load_default_tod` 装载（库外）；空序列按原生 panic |
| `utils::inverse_lerp` / `time_continuous_lerp_mut` | 本域等价实现 | utils 域恢复后可换回 |
| `TimeOfDay` 默认值（sunrise/sunset 等） | schema 重建（0.25/0.75 等） | 原生 Default body 无独立符号；需运行/存档取证 |

## 落盘

- `crates/country-core/src/systems/time_of_day.rs`（主系统/4 函数重建）
- `crates/country-core/src/resources/time_of_day.rs`（TimeOfDay/TodSequence/
  snapshot/lerp/seams）
- `crates/country-core/src/render/directional_light.rs`（`#[derive(Resource)]`，
  Resource 胶水；原系统参数为 `ResMut<DirectionalLight>`）
- `crates/country-core/src/startup/run_game.rs`（13 参系统资源/事件注册同步）

## 验证

- `cargo check -p country-core --lib`：**通过**（0 error）。
- `cargo test -p country-core --lib time_of_day`：**16 passed / 0 failed**
  （本域 15 + mode_manager 同过滤词 1）。
