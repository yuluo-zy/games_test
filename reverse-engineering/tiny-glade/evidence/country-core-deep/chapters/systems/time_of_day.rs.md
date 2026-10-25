# systems/time_of_day.rs

> 文件作用：**昼夜系统**——13 参数系统：太阳角/小时双插值（exp(−3·dt) 平滑、wrap ±π/±0.5）、
> 播放状态机（dev 关 state2 倒带 dt/−5；dev 开 state1 前进 dt/3，hour=fmodf(progress·24+8,24)）、
> 音频上报（TOD-Enabled/Timetravel/DayNight）、suncalc 太阳位置（lat=状态@+0x10、lon=0、
> 日期=2023<<9|day_of_year）、TodSequence 光照快照插值、remap 小时→fmod 小时关键帧曲线。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `time_of_day` | 0x140a73460 | 0 | 见文件作用；资源面：当前态 {hour,angle,角度过渡@+3,小时过渡@+0xa,瞬时@+0xb}、播放态 {state,progress,target_hour,角度@+0xc,纬度@+0x10,日出@+0x14,日落@+0x18,太阳可见@+0x20/24,day_of_year@+0x28}、dev 态 {小时倍率@+0x23,热键@+0xe3}；越界 day_of_year unwrap panic | O：22KB 反编译全析（DIGEST §2.6） | camera-time\decomp\decomp-0x140a73460.txt |
| `remap_hour_to_fmod_hour` | 0x140a74790 | 287 | 小时→fmod 小时关键帧：x={0,3.5,9,17,20,24}、y={0,3,11,17,22,24}（冬季 x 插 4.52、y 插 5.0/6.2/9.0），线性插值查找 | O：两 Vec 构建+memmove 插入+lerp 全实证 | camera-time\decomp\decomp-0x140a74790.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `suncalc_pos_to_vec`（行 348）— suncalc 方位/高度 → 3D 方向向量（colatitude=π/2−(azimuth+π) + 正交基常量 @142ad5910..5984，并入 time_of_day 主体）

## 实现方式（算法级）

```
time_of_day（0x140a73460，13 参数系统）：
  angle 插值：向 2π−angle（6.28319@0x142934674），wrap ±π，1−exp(−3·dt)（−3@0x142ad58e8），收敛<0.01 清标志
  hour 插值：wrap ±0.5（0.5@0x142925870），exp(−3·dt·max(speed,1))，收敛<0.001 清标志；瞬时吸附直接对齐 fmod 后小时
  播放状态机：
    dev 关：0→progress=0→2；2→progress+=dt/−5（下限 0）倒带
    dev 开：0→progress=0→1；1→progress+=dt/3（上限 1）前进
  小时推进：target_hour=fmodf(progress·24+8+dt·TIME_OF_DAY_SPEED(全局 0x14362c0b8)·speed·trig, 24)
    hour=fmodf(target·24+8, 24)（24@0x142934600、8@0x142934604）→ remap_hour_to_fmod_hour(hour,winter)
  日期/太阳：day_of_year∈1..365（越界 unwrap panic）
    encoded=day_of_year|0xFCE00（2023<<9）；unix_ms=decode·1000+hour·3600000（3600000@0x142ad58f0）
    suncalc::get_position(ms, lat=state@+0x10, lon=0.0) → colatitude=π/2−(azimuth+π) + 正交基常量（@142ad5910..5984）
    → 3D 太阳方向 → 写光照资源（TodLightingSnapshot 插值、太阳可见两套 SunSkyColors @0x140b18830）
  音频上报：set_parameter "TOD-Enabled"(0xb)/"Timetravel"(10)/"DayNight"(8)
remap_hour_to_fmod_hour（0x140a74790）：
  x={0,3.5,9,17,20,24}；y={0,3,11,17,22,24}（合同写 {48,17,11,24,22,…} 是错的）
  冬季：x 插 4.52；y 插 5.0/6.2/9.0（memmove 右移 0x142923c10）→ 线性插值查找（func_0x140c918b0）
```
资源面：当前态 {hour,angle,角度过渡@+3,小时过渡@+0xa,瞬时@+0xb}；播放态 {state,progress,target_hour,角度@+0xc,纬度@+0x10,日出@+0x14,日落@+0x18,太阳可见@+0x20/24,day_of_year@+0x28}；dev 态 {**小时倍率@+0x23**（合同写 @+0x118 是错的）, 热键@+0xe3}。
