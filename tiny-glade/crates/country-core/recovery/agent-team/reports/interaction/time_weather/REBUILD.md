# REBUILD — systems/time_of_day 真实逻辑重建（rebuild-cameraui）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
LOGIC.md: reports/interaction/time_weather/LOGIC.md（4 个 `未实现-需重做`）
证据: ghidra `--worker shared` fresh 反编译（0x140a73460 全 22KB）+ radare2 机器码
（0x140a74170..7425e 光照段）。

## 分类结论（协议 §2026-10-17 用户补充）

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `time_of_day`（主系统） | 0x140a73460 | **真实业务** | 重建（13 参签名） |
| `advance_playback` | 行 335..365 内联 | **真实业务** | 重建 |
| `advance_tod_hour` | 行 371..383 内联 | **真实业务** | 重建 |
| `suncalc_pos_to_vec` | 行 348..424 内联 | **真实业务** | 重建 |

无宏生成胶水。`remap_hour_to_fmod_hour`/`advance_angle`/`advance_hour` 判等函数按红线**不动**。

## 逐函数：分类 → 重建 → 逐语句判等自检

### 1. advance_playback（行 335..365 内联）— 真实业务，重建
- fresh：dev 关（`dev_speed_mult==0`）`iVar11=2`、dev 开 `iVar11=1`；**每帧强制 state 到
  dev 方向**——dev 关恒 `Rewinding(2)`、dev 开恒 `Playing(1)`；旧 state 只决定 progress：
  - Idle(0) → progress=0；
  - Playing(1) → dev 关 progress 不变 / dev 开 `+=dt/3` clamp≤1；
  - Rewinding(2) → dev 关 `+=dt/−5` clamp≥0 / dev 开 progress 不变。
- 分歧：Rust 旧实现只在 Idle 跳转，Playing/Rewinding 不随 dev 翻转。
- 重建：按上表 match 旧 state + 强制新 state。
- 判等自检：两分支五 case 逐条对应；clamp 方向/边界一致（`≤0.0→0.0`、`1.0≤→1.0`）。

### 2. advance_tod_hour（行 371..383 内联）— 真实业务，重建
- fresh：`fVar30 = dt·TIME_OF_DAY_SPEED`；`fVar29 = (float)exp2f(?)`；
  `target_hour = fmod(fVar29·progress·fVar30·dev_speed + target_hour + 1.0, 1.0)`；
  `hour = fmod(target_hour·24 + 8, 24)` → `remap_hour_to_fmod_hour`。
- 分歧：Rust 旧 `progress·24+8+dt·speed` 简化式、TOD_SPEED 恒 1.0。
- 重建：`advance_tod_hour(progress, dt, dev_speed, TIME_OF_DAY_SPEED, target_hour, winter)`；
  增量 `2.0f32.powf(progress)·progress·dt·TOD_SPEED·dev_speed`（exp2f 实参待取证，
  以 progress 承载），`(增量+target_hour+1.0).rem_euclid(1.0)`；`TIME_OF_DAY_SPEED` 全局常量。
- 判等自检：exp2f、+1.0、TOD_SPEED、dev_speed@+0x118 结构逐项对应；fmod 语义 `rem_euclid` 等价
  （含负值 wrap）。

### 3. suncalc_pos_to_vec（行 348..424 内联）— 真实业务，重建
- fresh：`colatitude=π/2−(azimuth+π)`；`f44=−sin(colat)·cos(elev)`、`f43=sin(elev)`、
  `f29=cos(colat)·cos(elev)`；正交基第一段用 **`basis_angle`**（`target_hour·2π+3.45`）的
  sin/cos + A..P 常量（@0x142ad590c..5984）；第二段 N/M；**末段半角四元数**用
  **`orbit_angle`（state.angle）** 的 sin/cos(angle/2) 做 Y 轴旋转，`out_y=sin(elev)+(−0.15)`，
  `1/sqrt` 归一化。
- 分歧：Rust 旧用 sin/cos(elev) 全角、无归一化、无 −0.15，且基角用错。
- 重建：`suncalc_pos_to_vec(azimuth, elevation, basis_angle, orbit_angle) -> Vec3`，逐行对应
  行 436..464（第一段矩阵、第二段、末段 `cos_a=ca²−sa²`/`sin_a=2·sa·ca` Y 旋转、
  −0.15、归一化）。
- 判等自检：A..P 常量、两级旋转次序、−0.15 落点、`1/SQRT` 因子逐项对应；次级向量
  （f45/f49/f41 族 → 光照快照 +0x30/+0x34/+0x38 槽）登记待接。

### 4. time_of_day（0x140a73460）— 真实业务，重建（主系统）
- fresh 控制流：音频使能门（`*in_RCX` 非 0）→ 角度插值（advance_angle）→ 小时插值
  （advance_hour）→ 双 beat/输入同步（行 273..327 u64 钳位减法）→
  `DEVELOPER_HOTKEYS_ENABLED==1` 且按键命中 → dev 热键 `byte^=1` → 播放状态机
  （advance_playback 强制方向）→ 小时推进（advance_tod_hour 含 exp2f）→
  `day_of_year` 越界 unwrap panic → `suncalc::get_position` → `snapshot_at_tod` →
  `DirectionalLight::set_direction_towards_light` → TodLightingSnapshot/SunSkyColors 写盘 →
  音频上报（"TOD-Enabled"/"Timetravel"/"DayNight"）。
- 重建：`time_of_day<S: Studio + Send + Sync + 'static>`（AudioCore<S> 音频上报、
  KeyboardBeatSeam 热键门、ResMut<TodLightingSnapshot> 写盘目标）；新增
  `sun_position_at`（suncalc get_position 源码级简式）、`TodLightingSnapshot`
  （sun_direction/day_night_factor/directional_light/sun_sky/updated）、`BeatConfig` +
  `sync_beat_counter`（双 beat 钳位）、`TimeOfDayState` 增 `dev_hotkey`/`beat`/`beat_config`。
- 判等自检：8 段控制流逐段落地；音频/热键/输入同步经可自持 seam 接线；`snapshot_at_tod`
  与 FMOD 实参登记宿主 seam。
- 副作用：run_game.rs 的 time_of_day 注册同步为 `::<S>`（签名从 3 参校正为 13 参，
  LOGIC 记录"系统签名缺 6 个参数"的落地）。

## 编译与验证

- `cargo check -p country-core`：**通过**（lib 编译零错误）。
- 纯算法独立验证（/tmp/rebuild-verify，glam 0.29.3）：advance_playback 强制方向、
  advance_tod_hour exp2f、suncalc_pos_to_vec 半角+归一化+−0.15 逐断言通过。
- 测试：systems/time_of_day.rs 测试更新（强制状态机、exp2f 结构、半角旋转、beat 钳位）。
