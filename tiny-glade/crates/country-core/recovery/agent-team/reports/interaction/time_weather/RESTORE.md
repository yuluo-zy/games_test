# RESTORE — systems/time_of_day + weather/daily（卡D-slice3）

实现者: restorer-interaction
日期: 2026-10-15
范围: time 主闭包（TimeOfDayState + time_of_day 纯算法 + suncalc_pos_to_vec）；weather/daily 评估登记。

## 原函数 → Rust 位置 → 证据

| 原函数 | VA | Rust 位置 | 证据 | 状态 |
|---|---|---|---|---|
| `time_of_day::time_of_day` | 0x140a73460 | `systems/time_of_day.rs::time_of_day` | chapter systems/time_of_day.rs.md（22KB 反编译全析） | O 结构（纯算法闭合；suncalc/光照/音频待接） |
| `time_of_day::remap_hour_to_fmod_hour` | 0x140a74790 | `resources/time_of_day.rs`（既有） | 既有（已核 OK） | 复用 |
| `time_of_day::suncalc_pos_to_vec`（内联 348） | — | `systems/time_of_day.rs::suncalc_pos_to_vec` | chapter（colatitude=π/2−(azimuth+π)） | O 结构（正交基常量未直读） |

## 新增实现与依赖复用边界

- `TimeOfDayState` 资源（衔接卡0 TimeOfDay 节载荷/save 节）：当前态 {hour, angle,
  angle_transitioning@+3, hour_transitioning@+0xa, instant@+0xb} + 播放态 {playback,
  progress, target_hour, latitude@+0x10, sunrise@+0x14, sunset@+0x18, day_of_year@+0x28}
  + dev 小时倍率@+0x23。serde 导出供存档。
- 纯算法（O）：`advance_angle`（wrap ±π、1−exp(−3·dt)、收敛<0.01）、`advance_hour`
  （24 小时圆 ±12 最短路径、exp(−3·dt·max(speed,1))、收敛<0.001）、`advance_playback`
  （dev 关 0→2 倒带 dt/−5、dev 开 0→1 前进 dt/3）、`advance_tod_hour`
  （target=fmodf(progress·24+8+dt·speed)、hour=fmodf(target·24+8,24)→remap）。
- `suncalc_pos_to_vec`（I：colatitude 语义；正交基常量 @0x142ad5910..5984 待 RNE 直读）。
- 复用：`remap_hour_to_fmod_hour`（resources/time_of_day.rs）、`SunSkyColors`（render/atmosphere）。

## 登记待接（阻塞项）

- `suncalc::get_position(ms, lat, lon)`（suncalc crate 未入锁依赖）。
- 光照写盘：`TodLightingSnapshot` 插值、`SunSkyColors` 两套（render/atmosphere 已有
  SunSkyColors，TodLightingSnapshot 待取证）。
- 音频上报：FMOD "TOD-Enabled"(0xb)/"Timetravel"(10)/"DayNight"(8)。
- TIME_OF_DAY_SPEED 全局（0x14362c0b8）。

## weather/daily 评估

- 库内无 `resources/daily.rs`/`resources/weather.rs` 实体（inventory 有
  daily.rs get_name/is_weekly、weather.rs default，均 no_formal_mapping GAP）。
- `systems/weather_system.rs`（weather_system）已存在库内。本卡登记：daily/weather 资源
  类型 GAP，待资源层闭合后补。

## 编译状态

`cargo check -p country-core --lib` 0 error；time_of_day + camera_rig 28 测试通过
（含 angle/hour/playback/tod_hour 纯算法 + day_of_year panic）。

## 留给 reviewer 的核对点

1. `advance_hour` 的 24 小时圆 ±12 最短路径 wrap（0.5 天）。
2. `advance_playback` dev 关/开状态机（dt/−5 倒带、dt/3 前进）。
3. `advance_tod_hour` fmodf 数学 + remap。
4. suncalc_pos_to_vec colatitude 语义。
