# 逐函数逻辑判等 — time_of_day（camera/time 组）

审计员: logic-verifier-camera-ui
日期: 2026-10-17
方法: 协议 2026-10-16-logic-verify-protocol.md，全部 fresh ghidra 反编译（`--worker shared`）。

## 判定统计

| verdict | 数量 |
|---|---|
| 判等 | 3 |
| 未实现-需重做 | 4 |

## 判等（3）

| VA | 函数 | Rust 位置 | 核对点 |
|---|---|---|---|
| 0x140a74790 | `remap_hour_to_fmod_hour` | resources/time_of_day.rs | x 数组[0,3.5,9,17,20,24]（6）、y 数组[0,3,11,17,22,24]；winter 两轮 grow+insert x{4.52,6.2}/y{5.0,9.0}→8 元素全命中；对相邻对调 `fit`(0x140c918b0) 插值，无命中原样返回 hour。判等 |
| — | `advance_angle`（纯算法） | systems/time_of_day.rs | 原生 angle 块：`fmod(angle,2π)`、wrap ±π、`expf(-3·dt)`、`1−exp` 系数、收敛<0.01 清标志。Rust 语义一致（rem_euclid+wrap）。判等 |
| — | `advance_hour`（纯算法） | systems/time_of_day.rs | 原生 hour 块：wrap ±0.5（0.5@0x142925870）、`expf(-3·dt·max(speed,1))`（speed 读 @+0x118 取 max 1）、收敛<0.001。Rust 12/24 小时单位换算一致。判等 |

## 未实现-需重做（4）

| VA | 函数 | Rust | 分歧点 |
|---|---|---|---|
| 0x140a73460 | `time_of_day`（主系统） | systems/time_of_day.rs | 原生 22KB 全链含：angle/hour/playback/tod_hour 内联推进 + **AudioCoreInner::set_parameter 音频上报×N** + **suncalc::get_position(0x1417ab4b0)** + **snapshot_at_tod(0x140b067d0)** + **DirectionalLight::set_direction_towards_light(0x140b18830)** + TodLightingSnapshot/SunSkyColors 写盘 + **DEVELOPER_HOTKEYS_ENABLED 门 toggle** + **双 beat/输入同步结构(u16-…/+0x40)计算** + `TIME_OF_DAY_SPEED` 全局。Rust 只做 angle/playback/tod_hour 推进 + day_of_year assert；光照/音频/suncalc/snapshot/热键/输入同步**全部登记待接未落地**；TOD_SPEED 硬编码 1.0。函数内部逻辑与原生不在同一完成度 |
| — | `advance_playback` | systems/time_of_day.rs | 原生 dev 关：Idle→progress=0 且 state=2；**Playing(1)→state=2 且 progress 不变**；Rewinding(2)→progress+=dt/−5 clamp≥0。dev 开：Idle→state=1 progress=0；Playing→+=dt/3 clamp≤1；**Rewinding(2)→state=1 且 progress 不变**。即**原生每帧把 state 强制到 dev 方向**（dev 关恒倒带、dev 开恒前进）。Rust 只在 Idle 时跳转，Playing/Rewinding 不随 dev 翻转——状态机行为分歧 |
| — | `advance_tod_hour` | systems/time_of_day.rs | 原生：`fmodf(exp2f(?)·progress·dt·TIME_OF_DAY_SPEED·dev_speed + target_hour + 1.0, 1.0)`（含 exp2f、+1.0、全局 TOD_SPEED、dev_speed_mult@+0x118）。Rust `progress·24+8+dt·speed` 简化式，TOD_SPEED 恒 1.0。公式结构分歧（restorer 自注 TOD_SPEED 待取证） |
| — | `suncalc_pos_to_vec`（行 348 内联） | systems/time_of_day.rs | colatitude=π/2−(azimuth+π) 与 A..P 正交基常量(−0.7624431/0.647056/0.5813199/0.418681/−0.4933432/0.551646/0.834078/0.806015/0.591895)均命中。**分歧**：原生末段用 **sin/cos(alt/2) 双角四元数 + 1/sqrt 归一化 + −0.15 偏移**，并级联进光照写盘；Rust 用 sin/cos(alt) 全角、无归一化、无 −0.15。final 旋转与归一化语义不符 |

## 附注

- `TimeOfDayState` 为重建资源；其默认值（hour=8/sunrise=6/sunset=20/day_of_year=1/dev_speed_mult=1.0）无原生默认函数判等依据，属 schema 重建。
- 主系统 `time_of_day` 判定依据：原生存在大量 Rust 未落地的 callee 链（audio/suncalc/snapshot/lighting/输入同步/热键），非"待接"可豁免。
- `suncalc_pos_to_vec` 的 basis 常量 `@0x142ad590c..5984` 与 fresh 反编译逐项一致（已核）。
