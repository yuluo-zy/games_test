# REVIEW — systems/time_of_day + weather/daily(卡D-slice3 终验 → 闭合·time 主闭包)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer RESTORE.md(reports/interaction/time_weather/);`cargo check --lib` exit 0
(独立复验,3 警告来自 save 组未用导入)。

## 核对

### ① TimeOfDayState 资源 — PASS
当前态 {hour, angle, angle_transitioning@+3, hour_transitioning@+0xa, instant@+0xb} + 播放态
{playback, progress, target_hour, latitude@+0x10, sunrise@+0x14, sunset@+0x18, day_of_year@
+0x28} + dev 小时倍率@+0x23;serde 导出衔接卡0 TimeOfDay 节/save 节。字段与 RESTORE/章节
布局一致 ✓。

### ② 纯算法 — PASS(与章节/常量一致)
- `advance_angle`:`(target−current).rem_euclid(2π)` 后 `>π → −2π` 归入 (−π,π];`1−exp(−3·dt)`
  (SMOOTH_K=−3 对应 0x142ad58e8);收敛 <0.01 返回 target+清过渡标志 ✓。
- `advance_hour`:`diff>12 → −24 / <−12 → +24` 24 小时圆最短路径(±0.5 天);因子
  `1−exp(−3·dt·max(speed,1))`;收敛 <0.001;`rem_euclid(24)` ✓。
- `advance_playback`:dev 开 Idle→Playing 前进 dt/3;dev 关 Idle→Rewinding 倒带 dt/5,
  clamp [0,1] ✓。
- `advance_tod_hour`:`target=(progress·24+8+dt·speed) mod 24`、`hour=(target·24+8) mod 24`
  → remap;与 RESTORE fmodf 链一致(rem_euclid 等价 fmodf 非负)✓。
- 常量:TWO_PI=6.2831855(0x142934674 实值 0x40C90FDA=6.2831855,章节"6.28319"为显示舍入)、
  HOUR_MOD/OFFSET、HOUR_TO_MS、YEAR_2023_BITS=0xFCE00 ✓。

### ③ suncalc_pos_to_vec — PASS(诚实标注 I)
colatitude = π/2 − (azimuth + π) 与章节一致;正交基常量 @0x142ad5910..5984 **未直读**,按
colatitude 语义登记,函数头明确标 I(不冒充 O)✓。

### ④ 登记诚实 — PASS
suncalc::get_position(suncalc crate 未入锁)、TodLightingSnapshot 插值/SunSkyColors 两套写盘、
FMOD "TOD-Enabled"/"Timetravel"/"DayNight" 上报、TIME_OF_DAY_SPEED 全局(0x14362c0b8,本实现
用 1.0 默认)——全部登记待接;weather/daily 库内资源 GAP(daily.rs/weather.rs no_formal_mapping)
如实登记,不冒充闭合。

## 结论与 DISPATCH 建议

- 卡D-slice3 复验通过:**time 主闭包(TimeOfDayState + 纯算法)闭合**;suncalc/光照/音频/全局
  速度为登记待接项。
- 建议:slice3 → 闭合;卡D 主体(相机+时间)闭合;camera_rig 剩余大件
  (immediately_finish_interpolating/init_walk/match_free/MainCamera::new 六 body 装配/zoom/
  body 更新算法)后置。weather/daily 归资源层 GAP。
