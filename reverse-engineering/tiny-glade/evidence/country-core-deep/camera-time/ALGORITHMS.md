# camera-time 域算法级文档（2026-10-15）

> 对象：country-core 相机/昼夜/天气/时间域。本文档把 DIGEST + round2 CLOSURE + 章节升级为**算法级**：
> 处理链、逐函数算法、数据结构、常量表、状态机。证据：`decomp/decomp-*.txt`（本域 55+3）、
> round2/camera-time-CLOSURE.md、rne-read 常量/串。标注 **O**=机器码直接支持；**I**=推断；**U**=未恢复。

---

## 1. MainCamera（0x4e0B 资源）总览

### 1.0 布局（O）

| 偏移 | 字段 |
|---|---|
| +0x00..+0x100 | CameraMatrices 矩阵块（paused update memcpy 0x100） |
| +0x100 | Normal body（0xb8B） |
| +0x1b8 | TiltShift body（同 Normal 布局） |
| +0x270 | Free body = dolly CameraRig（0x38B） |
| +0x2f0 | Walk body（rig + last_translation_dt@+0x50、allow_vertical@+0x54、ground_hit Option@+0x38..） |
| +0x390 | TopDown body |
| +0x448 | Butterfly body（rig 0x38 + selected@+0x38/applied@+0x3c） |
| +0xe0..+0xf4 | 快照/插值区（+0xec=fov 插值系数、+0xe8=gizmo 读作 fov、+0x134=每模式快照状态字节） |
| +0x344 | 鼠标深度字节（main_camera_update 写） |
| +0x4d0 | 模式字节 0..12（13 槽；0..5=六 body，6..12=拍照/工具槽复用 body） |
| +0x4d8 | aspect = 窗口宽/高 |

模式→字段查表：
- `focus 表 @0x142aabb68`（13 项，相对 +0x100）：{0x70,0x128,0x1a8,0x248,0x300,0x388,0x8c,0x8c,0x144,0x1c4,0x264,0x31c,0x3a4}
- `lens(fov) 表 @0x142aabb98`（13 项，相对 +0x108）：{0x8c,0x144,0x1c4,0x264,0x31c,0x3a4,0x08,0x20,0x14,0x2c,0x08,0x14,0x0c}

访问器（hex 手解）：`mode` 0x140911230 = `movzx eax,[rcx+0x4d0]`；`focus` 0x140911240 = `self+0x100+focus表[mode]`；`lens` 0x140911260 = `self+0x108+lens表[mode]`；`target_vertical_fov_degrees` 0x1409114d0 = `[self+0x108+lens表[mode]]`；`immediately_set_vertical_fov_degrees` 0x1409114f0 = `self+0x100+lens表[mode]` 尾调 body setter。

### 1.1 模式分发 / CameraRotationPivot

- CameraMode 核心 6 变体（0..5 = Normal/TiltShift/Free/Walk/TopDown/Butterfly）；模式字节另有 6..12 拍照/工具槽。
- `CameraRotationPivot` = **7 变体枚举**（ron 变体名 @0x14292bb7c：CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain）；`previous` 0x140914ac0 表 [6,0,1,2,3,4,0,0]（previous(0)=6 回绕）。

### 1.2 rotate_yaw_pitch（0x140993750，free/walk 共享）

```
fn rotate_yaw_pitch(&mut self, delta_yaw, delta_pitch):
  yawpitch = 遍历 drivers（stride 0x10）查 YawPitch driver（type_id 0x718e4fd258330fe5，vtable+0x40 driver_mut）
  if 未找到: panic "No <dolly::drivers::{yaw_pitch::YawPitch}> driver found in the CameraRig"（行 35）
  yaw   = fmodf(yaw   + delta_yaw   * 0.5, 720.0)     // 0.5@0x142925870、720@0x142aba9ac
  pitch = clamp(pitch + delta_pitch * 0.5, -90.0, 90.0) // ±90@0x142aba9b0 / 0x1429edb94
```

**增量折半（×0.5）；yaw 模 720（两整圈自由）；pitch 硬钳 ±90°。**

### 1.3 match_free_camera_to_normal（0x14090e6f0，rig 重建）

```
fn match_free_camera_to_normal(&mut self):
  pos = normal.position(+0x138..0x144)；yawpitch = normal.yaw_pitch(+0x150)
  建新 rig：Position(pos) + YawPitch(yawpitch) + Smooth(1,1)（各 alloc 12B/8B/0x40B，vtable 族）
  rig.update(dt=0)：逐 driver vtable+0x30(driver, parent_transform, 0) → 写 final_transform@+0x18..0x34
  drop 旧 rig@+0x270（FUN_1405cfe90）
  写新 rig@+0x270 = {drivers Vec cap-first, final_transform}   // 与 FreeCamera::set 同构建器（0x140993100）
```

### 1.4 zoom（0x140911280，582B 全函数手解）

```
fn zoom(&mut self, amount, anchor: Vec3):
  if !amount.is_finite() || !anchor.is_finite(): 错误路径（日志/panic 辅助）
  factor = exp2(clamp(amount * -0.08, -1.0, 1.0))
  match mode@+0x4d0:
      Normal → body=+0x100；TiltShift → +0x1b8（lower 查表 table2[mode==1]）；TopDown → +0x390
      其它 → 默认路径
  if body.active_rotation(+0x18) == 1:
      target_arm_length@+0x20 = clamp(factor * target_arm, lower, radial_max@+0x64)
  else:
      offset = (pos + pending) − anchor；pending *= factor（写回 pending）
```

### 1.5 camera_snapshot（0x140911e40，快照按模式分发）

```
fn camera_snapshot(mut snapshot: ResMut<CameraSnapshotInterpolationState>, time, camera):
  tick 回写（Mut +0x1c）
  mode = snapshot.mode u32@+0x18
  jumptable(0x142aabc88)[mode](snapshot.f32@+0x1c, dt * camera.fov_coeff(+0xec), mode, ...)
  // 每模式 body 快照应用函数；dt×+0xec = 插值步长（fov 系数）
```

### 1.6 main_camera_update（0x140912c20，会话/暂停驱动）

```
fn main_camera_update(window_size, mouse_depth, new_session_flag, debug_gui, action_states, camera):
  aspect = width/height；camera+0x4d8 = aspect
  if new_session.flag(+0x18) != 0:
      if width > 2 && cursor == (0,0)（cursor@+0x160）: return     // 等鼠标首次移动，不消耗标志
      new_session.flag = 0（消耗）
  if width < 3 || height < 3: push WriteSettingsFileCmd（Vec grow + 8B 事件 + counter+1@+0x40）
  camera+0x344 = 鼠标深度字节
  mode = debug_gui(+0x11d 区)；if mode == 6: mode = 0
  if debug_gui.free_camera_switch(+0x168) != 0.0:
      FUN_14127cab0（视口归一化 + 自由相机计算）→ jumptable(0x142aabd30)
  else:
      jumptable(0x142aabd04)[action_states.mode(+0x10)]           // 按 ActionSetStates 分发
```

`paused_main_camera_update`（0x1409145a0）：aspect=w/h；mode→6 body（+0x100/1b8/270/2f0/390/448）；调 body 更新 fn 指针；`CameraProjectionParams::calc_matrices`（FUN_140c8ff70）+FUN_140c97390 → memcpy 0x100 → MainCamera@+0x0。case 6..11 反编译损坏（fov 对 7..10 已录：7=(0x264,0x248)、8=(0x144,0x128)/(0x31c,0x300)、9=(0x1c4,0x1a8)、10=(0x8c,0x70)）。

### 1.7 参数捕获/恢复

- `store_parameters`（0x140911710）：读 yaw@+0x154/pitch@+0x150/position@+0x138/fov@+0x190 → 0x24B 记录；**NaN→0**（position 默认 (0,5,0)@0x142aab4c0）+ 四条日志 "Camera {position|yaw|pitch|vertical_fov_degrees} invalid. Overriding param cache value."（file camera_rig.rs）。
- `load_parameters`（0x140911520）：mode=0（Normal）；轨道四元数（π/180@0x142aab4b0）；+0x118=0；push 28B ActiveRotation 入 +0x100 Vec；flag 时 +0x194=记录分量。
- `reset_camera`（0x140912a40）：DebugGui 层>4 日志 "Resetting the camera." → `MainCamera::new(旧 aspect@+0x4d8)`（0x14090ec70）→ memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24B 元素 free，cap 留 4）→ 推 WriteSettingsFileCmd。
- `immediately_finish_interpolating`（0x140911b50）：巨函数（106KB 粘连）：模式@+0x4d0 + 快照字节@+0x134 双 jumptable；fov 系数 +0xec×0.1；多 body 矩阵块 0x100 拷贝；winter==4 判别分支。

---

## 2. dolly 0.6.0 接线（O）

### 2.0 分叉 CameraTransform（round2 CLOSED）

游戏 dolly 分叉：`CameraTransform = {rotation: Quat @+0x18 (16B), position: Vec3 @+0x28 (12B)}`——**rotation-first**（stock dolly 0.6.0 为 position-first，分叉改序）。`WalkCamera::rotation` 0x140994310 = 16B 拷贝 `[self+0x18]` = 完整四元数 ✓（骨架语义正确）。

### 2.1 rig 更新协议

- CameraRig：{drivers Vec cap-first{cap@0,ptr@8,len@0x10}, final_transform@+0x18}。
- driver 查找 = vtable+0x38（type_id）；driver_mut = vtable+0x40；update = vtable+0x30(driver, parent_transform, dt)。
- typeid：Position=0x6b8463577afc1a66、YawPitch=0x718e4fd258330fe5。
- panic 串族（dolly rig.rs）："No <…> driver found in the CameraRig"（Position/YawPitch/Rotation/LookAt 四变体 @0x142aecec8 等）。

### 2.2 body 装配

| body | 装配 | 地址 |
|---|---|---|
| Free | Position + YawPitch + Smooth(1,1) | 0x14090e6f0/0x140993100 |
| Walk | Position + YawPitch（无 Smooth） | 0x140993e20（Position(0,1,0)@0x1429345b0） |
| Butterfly | Position + Rotation + Smooth(2,predictive) + Arm + Smooth(4) + LookAt(4)（I） | 0x140a0d090 |

### 2.3 Free（0x1409938b0 translate_screen_space）

```
quat(driver) 旋转 movement → clamp xz ±91（0x142aba9b4/8）→ terrain.max(0)+0.75 贴地
smooth = 1 − exp(−8·dt) → delta = smooth·20/60 → Position.translate
```

### 2.4 Walk（0x140994700 translate / 0x140994320 constrain）

```
translate_screen_space:
  quat@+0x18..0x24 旋转 movement → flat（allow_vertical 时保留 y）→ step = 8/60 归一
  → 8 次 spherecast（半径 0.5、filter 0x30cf、法向滑移 t·0.9；1e-10 阈值）
  → clamp xz ±34 → smooth → Position.translate
constrain_to_be_above_terrain:
  两段 spherecast（filter 0x20 先、0x30cf/0 后，半径 0.45@0x142a7fca4，XZ clamp ±64.9）
  → 地面下界 y−1.9−1.4 → terrain walk_height_blend（skip_blend 分支 +0x298 选 vtable 对）
  → y = lerp(y, target, 1 − exp(−last_dt·20))；ground_hit kind 0xb / 无命中 0x11
teleport（0x140994130）:
  Position driver 写 (x,y,z)；方向向量 → yaw/pitch 度（FUN_140334d80 + FUN_141766a90 × rad2deg 常量 0x142aba9e0/9e4）
```

### 2.5 Normal（0x140949fe0 update / 0x140949cc0 rotate / 0x140949dc0 constrain / 0x140949e70 translate）

```
update：
  smooth 推进（1−exp(−8dt) 族）；radial_max@+0x60 clamp；loose@+0x6c 地面投影
    （lookahead = forward·18、anchor 边界 46.0、radial_min 投影 —— 或 TiltShift/TopDown 半正交：
      distance = ((pos.y−10)/max(−fwd.y, 0.15)).max(11.0125)、anchor 边界 33.45）
  ActiveRotation 弧长插值（28B 元素：pitch/yaw_delta 等）；pending 应用；pos.y ≥ 1.0
  retain：|pitch_delta| < 0.001 && |yaw_delta| < 0.001 即完成
rotate_yaw_pitch（active_rotation 存在时）：
  base = Σ rotations.pitch_delta + pitch@+0x50
  bounded = clamp(−(pitch + base), pitch_min@+0x58, pitch_max@+0x5c)
  active.pitch_delta += −bounded − base；yaw_delta += yaw（+0x24/+0x28 写回）
constrain：
  pending.y = max(pending.y, terrain(pos.xz + pending.xz) + clearance@+0x68 − pos.y)
translate_screen_space：
  ratio = max(pos.y / −fwd.y, 1) / 50（0x1429cd070）
  pending += 绕 Y(yaw) 旋转的 movement × ratio
```

### 2.6 Butterfly（0x140a0d090）

```
update：
  if selected(+0x3c) != applied(+0x38): applied = selected；dt = 1000.0（0x14295b630 瞬时吸附）
  rig 迭代 vtable+0x30 → sret 28B {pos,rot}
rotate/constrain/translate → 折叠 0x14003bb80（`ret` trampoline，蝴蝶不约束不旋转）
```

---

## 3. 时间 / 昼夜 / 天气

### 3.0 GameTime（resources/time.rs，O 部分）

`{game_time 累计 Duration（secs@+0x0、nanos@+0x8）、delta Duration（secs@+0x10、nanos@+0x18）、delta f32@+0x28；+0x20..0x27 与尾部 U}`。

### 3.1 time_update（0x140922c30，56B 游戏时钟）

```
fn time_update(game_time, replay_state, ...):
  dt = replaying（InputRecordReplayState+0x40 == i64::MIN+2）? replay_frame.delta@+0xc8（0xd0B 帧）
     : GameTime delta Duration（secs@+0x10、nanos@+0x18）
  前两帧 clamp dt ≤ 1/60（0.016666668@142aad4a8）
  total@+0x10 **f64** += dt（纠错：合同写 f32）
  60ths@+0x18 进位；frac@+0x20；frame_count@+0x2c++
  panic：Duration 加法溢出 "overflow when adding durations"（0x142aad928）；
        f32→Duration "cannot convert float seconds to Duration…"（std 内联）
```

### 3.2 time_of_day（0x140a73460，22KB）

资源面：
- 当前状态：{hour f32@+0x0, angle f32@+0x4, 角度过渡标志@+0x3, 小时过渡标志@+0xa, 瞬时吸附标志@+0xb}
- 播放/目标状态：{play_state i32@+0x0(0/1/2), progress f32@+0x4, target_hour f32@+0x8, 当前太阳角 f32@+0xc, **纬度 f32@+0x10**, 日出 f32@+0x14, 日落 f32@+0x18, @+0x1c, 太阳可见@+0x20/@+0x24（事件可覆盖）, day_of_year u32@+0x28（1..365，越界 unwrap panic）}
- 开发者/速率状态：{**小时倍率 f32@+0x23**（min 1.0，喂 exp(−3·dt·speed)），热键开关 byte@+0xe3}——合同写 @+0x118 是错的。

算法：
```
angle 插值：
  向 2π − angle 收敛，wrap ±π（0.5@0x142925870、−0.5@0x14295a0e8），(1 − exp(−3·dt))（−3@0x142ad58e8）
  收敛 < 0.01 清过渡标志
hour 插值：
  wrap ±0.5；exp(−3·dt·max(speed, 1))；收敛 < 0.001 清标志
  瞬时吸附分支：直接对齐 fmod 后的小时
播放状态机：
  dev 关：state0 → progress=0 → state2；state2 → progress += dt/−5（下限 0）倒带
  dev 开：state0 → progress=0 → state1；state1 → progress += dt/3（上限 1）前进
小时推进：
  target_hour = fmodf(progress·24 + 8 + dt·TIME_OF_DAY_SPEED(全局 f32@0x14362c0b8)·speed·trig, 24)
  hour = fmodf(target·24 + 8, 24)（24@0x142934600、8@0x142934604）→ remap_hour_to_fmod_hour(hour, winter)
日期/太阳：
  day_of_year ∈ 1..365（(year−1)&0xffff > 0x16c 越界 → Result::unwrap panic）
  encoded = day_of_year | 0xFCE00（= 2023<<9，round2 CLOSED）
  unix_ms = decode(encoded)·1000 + hour·3600000（3600000@0x142ad58f0）
  suncalc::get_position(ms, lat=state@+0x10, lon=0.0)（FUN_1417ab4b0）→ 方位/高度
  colatitude = π/2 − (azimuth + π)；正交基常量组 @0x142ad5910..5984（非经纬度，round2 CLOSED）
  → 3D 太阳方向归一化 → 写光照资源（param_8：TodLightingSnapshot 插值结果、太阳可见两套 SunSkyColors @0x140b18830）
音频上报：AudioCoreInner::set_parameter "TOD-Enabled"(0xb)/"Timetravel"(10)/"DayNight"(8)
尾：param_3（单 f32 Res）→ 光照结构[0x15](+0x54)；双 EventReader（0x10B 步长）判空 → FUN_140c90670 写太阳可见标志
```

### 3.3 remap_hour_to_fmod_hour（0x140a74790）

```
fn remap_hour_to_fmod_hour(hour, winter):
  x 关键帧 = {0, 3.5, 9, 17, 20, 24}（@0x142ad5990/98/a0）        len 6
  y 关键帧 = {0, 3, 11, 17, 22, 24}（@0x142ad59b0/b8/c0）        len 6
      // 合同写 {48,17,11,24,22,…} 是错的
  if winter:
      x 位置 3 插入 4.52（memmove 右移 @0x142923c10）→ len 7
      y 三次 memmove 插入 5.0、6.2、9.0 → len 9
  线性插值查找：func_0x140c918b0(hour, x0, x1, y0, y1) = lerp
  越界/扩容 panic 族（FUN_1428d9000/FUN_1428d9081）
```

### 3.4 weather_system（0x1408e5b00）

```
fn weather_system(mut snow: ResMut<SnowIntensity>, game_time, glade_type):
  if glade_type@+0x58 != 4: return                    // 非冬季不写（连 tick 都不回写）
  game_secs = GameTime+0x0(secs) + GameTime+0x8(nanos)/1e9（1e9@0x1429a7640）
  s = sin(game_secs / 300 · π)（300@0x142aa5140、π@0x1429edba4）   // 10 分钟正弦
  delta = GameTime+0x28(dt f32) × 0.1（0.1@0x142925984）
  tick 回写（Mut +0x1c）
  intensity@+0xc = clamp(intensity + (s > 0 ? +delta : −delta), 0, 1.0)（1.0@0x142925904）
```

### 3.5 TodSequence::snapshot_at_tod（0x140b067d0）

```
TodSequence entries（0x50B/条：{start_hour f32, end_hour f32, snapshot 0x44B@+0xc}）
TodLightingSnapshot 0x44B = {sun_sky@+0x0 8×f32, fog_tint@+0x20 2×u64（打包雾色，不插值）, base_fog_thickness@+0x30 4×f32, speed_bias@+0x40}

fn snapshot_at_tod(&self, tod) -> TodLightingSnapshot:
  len==1 → 拷贝；len==0 → panic
  t = tod.rem_euclid(1)（fmodf + 负值 wrap）
  定段：t = clamp01((t − a.start) / (b.start − a.start))
  TodLightingSnapshot::lerp(a.snapshot, b.snapshot, t)（0x140a97650；f32 组插值 + u64 对拷贝）
```

### 3.6 daily.rs（0x14083c440）

```
fn is_weekly(&self) -> bool:
  status, elapsed = SystemTime::duration_since(now − timestamp@+0xe0..0xe8)（FUN_140d63bd0）
  status==OK && elapsed > 0x6977f（= 432511s ≈ 5.006 天）
  // 非 7 天：合同/骨架 7*86400 与证据不符
get_name（0x14083c240）：kind==5 判别 + [rdx+4] 首字符匹配 locale 族（en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP）
```

---

## 4. 其它系统

### 4.1 timelapse_replay（O）

- `TimelapseState` 字节 0=idle/1=recording/2=finished；`is_active` 0x14092e6b0 = **`state != 2`**（骨架写 ==1 错；相邻 ==3/<2 谓词归属 U）。
- `start_timelapse`（0x140a8a310）：消费 StartTimelapseReplayCmd → 修剪输入流 → 有未消费帧 → `mode_manager::check_if_can_switch<closure>`（FUN_140af9e00）。
- `finish_timelapse`（0x140a8a460）：state != 2 时：未消费帧且 state 偶数 → 推 MetaCommand（payload 前 8B = i64::MIN 哨兵 + 16B 数据 @+0x60..0x6c，FUN_140b59900 → VecDeque<MetaCommand>）→ state=1；命令队列空（+0x18==0）→ 推 {tag: i64::MIN, byte: state} 事件（stride 0x10）并清另一计数器。
- `cleanup_timelapse`（0x140a8a5d0）：有未消费帧 → 日志（level 3）"The timelapse has finished"（module `country_core::systems::timelapse_replay`）→ state=2。

### 4.2 arrow_gizmo_animation（0x140a92f80）

```
每实体（EnableArrowGizmoStretch）双 f32 拉伸值：
  有输入事件（0x10B 步长，8B 向量 payload）：
      主轴 = |x| < |y| ? y : x
      stretch = min(stretch + |Δ|·0.02, 1.0)（0.02@0x1429cd2d0、1.0 钳）
  无事件：stretch = max(stretch − 0.1, 0)（−0.1@0x1429eb6f8）
  Mut tick 回写；puffin scope country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation
```

### 4.3 ui_camera_mode_gizmos（0x1409527a0 + 闭包 0x140952b60）

```
四重门槛：
  ① AppMode ∈ {2, 10}（10 需 flag=0）
  ② 相机模式非 Free/Walk：（mode & 6) == 2 → return
  ③ 窗口宽或高 ≤ 2 → return
  ④ 签名符模式 ∈ {0,1,4}（0x13 >> mode & 1；模式 6 先映射为 0）
通过后：
  UiState::add("camera_mode_activerotation_")（状态 int != 20 时）
  → 构造 0x20B UiAppendSignifier {1, 1, id_string, size=窗口字段, fov=snapshot+0xe8}
  → AppendOnlyVec push
闭包 = 渲染回调：cursor_grab 部件 + vertex_color_shader；动画 value/1000·20，clamp 25.0@0x1429edb98
```

### 4.4 clear_canvas（0x140a7abd0）

```
事件驱动（EventReader 修剪流）：日志 "Clearing the canvas."（module country_core::systems::clear_canvas）
→ InnerWalls::remove_all（0x14085a730，flag 时 +0xc0=0）
→ 世界光栅 clear_all<512>（0x14093bee0）变体 0x13/0x14/6/5/7/1/2/3/4（+9/0xb/0xd/0xe）
→ 7× clear_preset_texture（0x140a15da0，3 个 color_id 0x5000=20KB 缓冲重置，panic 串 color_id.rs）
→ 上传记录/杂项清零（0x140aaae80×2、0x140aa5150、0x1408950d0、0x14088acf0）
```

### 4.5 despawn_glade_entities（0x1408b3510）

```
Query 迭代，world 字节 @+0x108 二分：
  A 支：stride 0xa0、count@+0x58、实体数组@+0x50、0x10B 元素
  B 支：stride 0x48、count@+0x10、数组@+0x8、8B 元素
逐槽逐实体 Commands::despawn（0x140d14770，file despawn.rs）；u64 实体 id 高 32 位 == 0 提前 return
```

### 4.6 look_at_camera（0x140873de0）

```
puffin scope；Query (LookAtCamera, Transform)
逐实体：FUN_140c96d40 提取目标变换 → FUN_140c9ce00 写回实体 Transform（位置+旋转四元数），Mut tick
register_required_components（0x1408756e0）= **Bevy derive required-components 注册胶水**（非运行时系统）
clone_behavior（0x140002370）= `xor eax,eax; ret`（返回 0：不克隆）
```

### 4.7 photo_mode（ui-input 域共享，0x140b596b0）

`PhotoModeCameraMode` 4 变体：`next` 0x140b59720 表 {0→1,1→2,2→3,3→0}；`next_demo` 0x140b596b0 表 {0→1,1→2,2→0}，**变体 3 → panic "this camera mode is not part of the demo!"**（串 @0x142af1188）。与 CameraMode 槽 6..12 联动（round2 PARTIAL）。

### 4.8 MainCameraPlugin::build（0x1409125e0）/ reset_camera / 其它

- build：注册 2 系统（hash 0x142c96f70/0x142c5fd60）+ 2 资源（typeid 0x80c70da6101ec1a/0x668ae4faf3605349）+ 事件注册（0x142c6bd60）+ 调度（FUN_140a500f0/FUN_140672ed0）；run_if 为 hash 编码（I）。
- GladeSettings::load（0x1408bbbc0）：glade_settings.json serde_json Option → 回退 Default{+0 u32=0、+8 0.01、+0xc 1.0、+0x10 8、+0x18 0.0、+0x1c 1.0、+0x20 1}。
- GameTime::default（0x140b64d00）零化构造。

---

## 5. 常量表（camera-time，O）

| 常量 | 地址 | 用途 |
|---|---|---|
| 0.5 | 0x142925870 | rotate_yaw_pitch 增量折半 / time_of_day hour wrap |
| 720.0 | 0x142aba9ac | yaw fmod |
| 90.0 / −90.0 | 0x142aba9b0 / 0x1429edb94 | pitch clamp |
| 0.0174533（π/180） | 0x142aab4b0 | 度→弧度 |
| 1.0 | 0x14292e6b0 | Smooth 初始 / weather clamp 上界 |
| −3.0 | 0x142ad58e8 | exp(−3·dt) 平滑 |
| 24.0 / 8.0 | 0x142934600 / 0x142934604 | hour = fmodf(progress·24+8, 24) |
| 2π（6.28319） | 0x142934674 | 角度收敛目标 |
| 1e9 | 0x1429a7640 | nanos→秒 |
| 300 / π | 0x142aa5140 / 0x1429edba4 | 冬季正弦周期 |
| 0.1 | 0x142925984 | weather ±0.1·dt |
| 0x6977f = 432511 | — | daily is_weekly 阈值（≈5.006 天） |
| 3600000 | 0x142ad58f0 | 小时→毫秒 |
| 0xFCE00 = 2023<<9 | — | suncalc 日期位打包基准年 |
| 1000.0 | 0x14295b630 | Butterfly 瞬时吸附 dt |
| 0.02 / −0.1 | 0x1429cd2d0 / 0x1429eb6f8 | arrow gizmo 增量/衰减 |
| 25.0 | 0x1429edb98 | gizmo 动画 clamp |
| ±91 / 0.75 / 20/60 | 0x142aba9b4/8 / — / — | Free translate_screen_space |
| 0.45 / ±64.9 / 1.9/1.4 | 0x142a7fca4 / — / — | Walk constrain |
| 50.0 | 0x1429cd070 | Normal translate ratio |
| 1000.0（吸附） | 0x14295b630 | Butterfly |
| 0.016666668（1/60） | 0x142aad4a8 | time_update 前两帧 clamp |

---

## 6. 状态机汇总

| 状态机 | 转移 | 级别 |
|---|---|---|
| CameraMode | 模式字节 0..12；previous(0)=6 回绕；gizmo/update 中 6→0 映射 | O |
| time_of_day 播放态 | dev 关 0→(progress=0)→2→dt/−5 倒带；dev 开 0→1→dt/3 前进 | O |
| time_of_day 插值标志 | 角度过渡@+0x3 / 小时过渡@+0xa / 瞬时@+0xb；收敛清标志 | O |
| TimelapseState | 0 idle→1 recording→2 finished；is_active = state!=2 | O |
| draw_raster（见 terrain-water） | 0 start_new / 1 增量 / 其余 commit | O |

---

## 7. 剩余 U（不臆造）

- CameraMode 槽 6..12 确切名字/语义（拍照模式相机 7 种）——ui-input 域 photo_mode 联动。
- paused_main_camera_update case 6..11 反编译损坏（bad instruction data）逐槽 body 对应。
- suncalc 正交基常量组（@142ad5910..5984）与 `latitude_degrees` 字段单位/默认值。
- time_of_day 的 param_3（单 f32 资源）与双 EventReader（太阳可见事件族）类型名。
- GameTime +0x20..0x27 与尾部字段语义。
- TodLightingSnapshot fog_tint 打包格式（RGBA8×2/RGBA32F×4…）。
