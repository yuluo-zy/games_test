# camera-time 域 DIGEST（tg_a6，2026-10-14）

反编译产物：`decomp/decomp-<va>.txt`（47 个批次 + 3 个补充：0x1409125e0 / 0x140912c20 /
0x14090ec70）+ `batch-report.json`、`summary.json`、`supp/supp-report.json`。
10 个 NotFound 已用 `rne-read --hex` 手解（见 §4）。数据库 `tg_a6`（批次）/ `tg_a6s`（补充）。

## 1. 模块清单（业务过程数，与 runner 汇总一致）

| 文件（PDB 编译单元） | 过程 | 备注 |
|---|---|---|
| systems/camera_rig.rs | 10 | MainCamera 方法族 + serde |
| systems/camera_rig/{free,normal,walk,butterfly}.rs | 18（含共享/折叠） | dolly 0.6.0 接线 |
| systems/main_camera.rs | 6（+补充 3） | build/main_camera_update/MainCamera::new |
| systems/look_at_camera.rs | 3 | look_at 系统 + derive 胶水 |
| systems/time_of_day.rs | 2 | time_of_day + remap_hour_to_fmod_hour |
| systems/time_update.rs | 1 | 游戏时钟 |
| systems/weather_system.rs | 1 | 冬季雪强振荡 |
| systems/timelapse_replay.rs | 4 | start/finish/cleanup/is_active |
| systems/ui_camera_mode_gizmos.rs | 2 | 系统 + 渲染闭包 |
| systems/arrow_gizmo_animation.rs | 1 | 箭头拉伸动画 |
| systems/clear_canvas.rs | 1 | 清空画布 |
| systems/despawn.rs | 1 | despawn_glade_entities |
| resources/time_of_day.rs | 1 | TodSequence::snapshot_at_tod |
| resources/daily.rs | 1 | get_name::closure$0 |

不在 sysres 索引内、但本域相关的：`resources/{time,weather,glade_settings,sparkle}.rs`（wp21
misc-resources.md 已覆盖，本域未重复反编译）；`resources/photo_mode.rs`（PhotoModeCameraMode，
属 ui-input 域，与相机模式枚举 6..12 变体强相关，跨域线索见 §6）。

## 2. 布局表（O=机器码直接支持；I=推断；U=未知）

### 2.1 MainCamera（0x4e0 B 资源，O 已证）

| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| +0x0 | 0x100 | CameraMatrices（view/proj 矩阵块） | `paused_main_camera_update` 0x1409145a0 末尾 `memcpy(self, stack, 0x100)`；`main_camera_update` 读 +0x40..+0x7c 矩阵元素 |
| +0x100 | 0xb8 | NormalCamera body（rotations Vec cap-first @+0x0/0x8/0x10，28B 元素；position {x@+0x38,y@+0x3c,z@+0x40}；pending_translation {x@+0x44,y@+0x48,z@+0x4c}；yaw@+0x50、pitch@+0x54、pitch_min@+0x58、pitch_max@+0x5c、radial_min@+0x60、radial_max@+0x64、terrain_clearance@+0x68、loose_ground_projection@+0x6c） | 0x140949dc0/0x140949e70/0x140949fe0；chase_butterfly 0x140911520 推 28B ActiveRotation 到 +0x100 Vec |
| +0x1b8 | 0xb8 | TiltShift body（同 NormalCamera 布局） | 0x1409145a0 case 1 |
| +0x270 | 0x38+ | Free body = dolly CameraRig {drivers Vec cap@0,ptr@8,len@0x10；final_transform position@+0x18、rotation quat@+0x24} | match_free_camera_to_normal 0x14090e6f0 重建 rig 写 +0x270；dolly rig.rs panic 串 |
| +0x2f0 | | Walk body（dolly rig + last_translation_dt@+0x50、allow_vertical@+0x54、ground_hit Option<CastHit>@+0x38..） | 0x140994320/0x140994700 |
| +0x390 | | TopDown body | 0x1409145a0 case 4 |
| +0x448 | | Butterfly body（rig 0x38 + selected_index@+0x38、applied_index@+0x3c） | 0x140a0d090 |
| +0xe0..+0xf4 | | 快照/插值区：fov 相关（+0xe8 被 gizmo 读作 fov、+0xec = fov 插值系数、+0x134 = 每模式快照状态字节） | 0x140911e40（`dt * camera+0xec`）、0x1409527a0（`*+0xe8`）、0x140911b50（+0x134 jumptable） |
| +0x344 | 1B | 鼠标深度字节 | main_camera_update 0x140912c20 |
| +0x4d0 | 1B | 模式字节（0..12，13 变体） | 各函数 `movzx [rcx+0x4d0]`；fov 偏移表 0x142aabb98 有 13 项 |
| +0x4d8 | f32 | 宽高比 = 窗口宽/高 | 0x1409145a0 / 0x140912c20 |

fov 偏移表 @0x142aabb98（相对 MainCamera+0x108，模式 0..12）：
`{0x8c, 0x144, 0x1c4, 0x264, 0x31c, 0x3a4, 0x08, 0x20, 0x14, 0x2c, 0x08, 0x14, 0x0c}` —
前 6 项对应 6 个 body 的目标 fov（如 Normal = +0x194 = body+0x94），第 7-13 项落在
Normal body 内（0x110/0x128/0x11c/0x134…）= 拍照/工具模式复用 Normal body 的 fov 槽。
`target_vertical_fov_degrees` 0x1409114d0 = `fov_table[mode]` 查表读 f32；
`immediately_set_vertical_fov_degrees` 0x1409114f0 = `self + table[mode] + 0x100` 后尾调 body setter。

### 2.2 CameraSnapshot / 插值状态（O 部分）

- `CameraSnapshot` 资源（Mut 包装）：mode u32 @+0x18、f32 @+0x1c（0x140911e40 读取）。
- `CameraSnapshotInterpolationState`：0x140911e40（PDB 名 is_interpolating）实为**按模式快照更新分发**
  （jumptable，调用 `body_fn(snapshot+0x1c, dt * camera+0xec, mode)`）——见 §7 错位。
- `CameraRotationPivot`：**枚举（7 变体）**，ron 序列化 0x14003ac90 变体名：
  `CameraRotationPivot / CursorDepthFiltered / CursorDepth / CenterDepthFiltered /
  CenterDepth / CursorTerrain / CenterTerrain`（I：旋转锚点 = 光标深度(滤波/不滤波) /
  中心深度(滤波/不滤波) / 光标地形 / 中心地形 / 显式 pivot）。
- 会话存档相机记录（0x1403b4ca0 反序列化器，串自 resources/session.rs）：
  `{ yaw: f32, pitch: f32, position: Vec3, camera_rig_arm_length: f32, pitch_offset: f32,
  yaw_offset: f32, position_cache: … }` = `CameraRigParametersCache` 元素（0x24B，
  reset_camera 0x140912a40 以 0x24 步长 free，元素大小一致）。
- `CacheFreeCamRig`：仅具名（U 布局）。

### 2.3 CameraMode 枚举（O 行为，变体名 I）

- 模式字节 0..12（**13 变体**，合同仅列 6）；0..5 = Normal/TiltShift/Free/Walk/TopDown/Butterfly
  （6 body 偏移实证），6..12 = 拍照/工具模式（复用 6 body + 各自 fov 槽；0x1409145a0 case
  7..10 的 (fov_target,fov_current) 对：7=(0x264,0x248)、8=(0x144,0x128) 或 (0x31c,0x300)、
  9=(0x1c4,0x1a8)、10=(0x8c,0x70)）。case 6/11 反编译损坏（bad instruction data）。
- `previous` 0x140914ac0：`table = [6,0,1,2,3,4,0,0]`，`previous(0)=6` → Normal 的前一个是模式 6
  （佐证第 7 变体存在）。`next` 未在索引中（合同给的 0x1407858016/0x1408230976 是截断垃圾地址，纠正）。
- DebugGui 滑块值 == 6 时映射为 0（main_camera_update / ui_camera_mode_gizmos 同款映射）。

### 2.4 dolly 0.6.0 接线（O）

- CameraRig 布局：`{drivers: Vec<Box<dyn RigDriver>> cap-first, final_transform @+0x18
  {position, rotation}}`（0x140993d30 update：逐 driver 调 vtable+0x30 写回 rig+0x18..0x34）。
- driver 查找 = vtable+0x38（type_id）比对 TypeId 常量：Position=0x6b8463577afc1a66、
  YawPitch=0x718e4fd258330fe5；`driver_mut` = vtable+0x40。panic 串
  `No <dolly::drivers::{position::Position|yaw_pitch::YawPitch|rotation::Rotation|look_at::LookAt}> driver found in the CameraRig`（dolly-0.6.0/src/rig.rs）。
- 使用面：Free = Position+YawPitch+Smooth(1,1)（0x14090e6f0 重建实证）；Walk = Position+YawPitch；
  Butterfly = Position+Rotation+Smooth(2,predictive)+Arm+Smooth(4)+LookAt(4)（I，rig 驱动 vtable
  遍历为 O）。**游戏自有逻辑 = 模式切换、地形/碰撞约束、快照插值；纯 dolly 数学不重实现。**

### 2.5 time_update 的 56B 游戏时钟（O，纠错一处）

`{ secs u64 @+0x0, nanos u32 @+0x4, flag u8 @+0x6, total **f64** @+0x10（合同写 f32，实为
(double) 存写）, total_60ths u64 @+0x18, frac_seconds f32 @+0x20, frame_count u32 @+0x2c,
flag @+0x30 }`。dt 来源：重放帧 @+0xc8（0xd0B 帧，`InputRecordReplayState+0x40 ==
i64::MIN+2` 判 Replaying）；否则 GameTime Duration（secs@+0x10、nanos@+0x18）。前两帧
clamp dt ≤ 1/60（0.016666668 @142aad4a8）。进位 panic `overflow when adding durations`、
f32→Duration 转换 panic（std 内联）。

### 2.6 time_of_day 系统（O，0x140a73460）

资源三件（I 命名，O 布局）：
- 当前状态（param_12）：`{ hour f32@+0x0, angle f32@+0x4, 角度过渡标志@+0x3,
  小时过渡标志@+0xa, 瞬时吸附标志@+0xb }`。
- 播放/目标状态（param_7）：`{ play_state i32@+0x0(0/1/2), progress f32@+0x4,
  target_hour f32@+0x8, 当前太阳角 f32@+0xc, 日出小时 f32@+0x14, 日落小时 f32@+0x18,
  @+0x1c, 太阳可见@+0x20/@+0x24（事件可覆盖）, day_of_year u32@+0x28（1..365，
  越界 unwrap panic）}`。
- 开发者/速率状态（param_9）：`小时倍率 f32@+0x23（min 1.0，exp(-3·dt·speed)）、
  开发者热键开关 byte@+0xe3`（**合同写 @+0x118=小时倍率，实为 +0x23，纠正**）。
  热键：FUN_14089aef0 轮询按键切换 +0xe3；音频上报 `set_parameter`：
  "TOD-Enabled"(0xb)、"Timetravel"(10)、"DayNight"(8)。
- 角度插值：向 2π−angle，wrap ±π，`(1−exp(−3·dt))` 平滑，收敛 <0.01 清标志（O，常量
  6.28319@142934674、−3.0@142ad58e8）。
- 小时插值：wrap ±0.5（0.5@142925870、−0.5@14295a0e8），`exp(−3·dt·max(speed,1))`，
  收敛 <0.001 清标志；瞬时吸附分支直接对齐 fmod 后的小时。
- 播放状态机（O）：dev 关：state0→progress=0→state2；state2→progress+=dt/−5（下限 0）
  倒带；dev 开：state0→progress=0→state1；state1→progress+=dt/3（上限 1）前进。
- 小时推进：`target_hour = fmodf(progress·24 + 8 + dt·TIME_OF_DAY_SPEED·speed·trig, …)`
  （24.0/8.0 @142934600/604；全局速率 fRam000000014362c0b8），`hour = fmodf(target·24+8, 24)`
  → `remap_hour_to_fmod_hour(hour, winter)`。
- 日期/太阳：day_of_year(1..365，`(year−1)&0xffff > 0x16c` 即越界) → 位打包 + unix_timestamp
  → `suncalc::get_position(ms, lat, lon)`（FUN_1417ab4b0）→ 方位/高度 →
  colatitude=π/2−(azimuth+π)（O 调用面；suncalc 输入 lat/long 常量 @142ad5910..5984
  一组 f32，U 语义）→ 3D 太阳方向归一化 → 写光照资源（param_8，~0x78B：方向、
  TodLightingSnapshot 插值结果、太阳可见分支两套 SunSkyColors set @0x140b18830）。

### 2.7 remap_hour_to_fmod_hour（O，0x140a74790）

动态建两条 keyframe Vec（f32，cap-first）后线性插值查找：
- x（小时）= **{0, 3.5, 9, 17, 20, 24}**（常量 @142ad5990/98/a0）；
- y（fmod 小时）= **{0, 3, 11, 17, 22, 24}**（@142ad59b0/b8/c0）——
  **合同写 {48,17,11,24,22,…} 是错的，纠正**；
- 冬季：x 插入 4.52；y 依次 memmove 插入 5.0、6.2、9.0（O：三次 0x142923c10 右移
  + 三处常量写）；插值 = `func_0x000140c918b0(hour,x0,x1,y0,y1)`（lerp）。

### 2.8 TodSequence / TodLightingSnapshot（O，0x140b067d0）

- 条目 0x50B：`{start_hour f32@+0x0, end_hour f32@+0x4, snapshot 0x44B@+0xc}`。
- 快照 0x44B：`{4×f32@+0x0, 4×f32@+0x10, 2×u64@+0x20（**直接拷贝不插值**——离散分量）,
  4×f32@+0x30, f32@+0x40}`。
- `snapshot_at_tod(tod)`：len==1 拷贝；len==0 panic；否则 `t = tod.rem_euclid(1)`
  定段（fmodf + 负值 wrap），`t = clamp01((t−a.start)/(b.start−a.start))`，
  `TodLightingSnapshot::lerp`（0x140a97650）。tod 值经 sret 缓冲首 4 字节传入
  （编译器把第一个参数放进输出槽）。骨架实现结构一致 ✓。

### 2.9 weather_system（O，0x1408e5b00）

`glade_type byte@+0x58 == 4`（冬）时：`game_time = GameTime+0x0(secs u64) + +0x8(nanos)/1e9`，
`sin(game_time/300·π)`（300@142aa5140、π@1429edba4）> 0 则 `intensity += 0.1·dt_f32`
否则 `−= 0.1·dt_f32`，clamp [0,1]。intensity f32 在资源 **+0xc**；dt_f32 = GameTime+0x28。
非冬季不写（连 tick 都不回写）。常量 0.1@142925984、1e9@1429a7640 实证。

### 2.10 timelapse_replay（O）

- `TimelapseState` 字节 0=idle/1=recording/2=finished。
- **0x14092e6b0（PDB is_active）机器码 = `state != 2`**（`cmp byte [rcx],2; setne al`）——
  骨架写 `== 1`，机器码为准纠正；相邻还有 `==3`、`<2` 两个谓词（0x14092e6b7/be，
  或属相邻枚举）。
- `start_timelapse` 0x140a8a310：消费 StartTimelapseReplayCmd 事件 → 修剪流 →
  有未消费帧 → `mode_manager::check_if_can_switch<closure>`（FUN_140af9e00）请求切模式。
- `finish_timelapse` 0x140a8a460：state != 2 时：未消费帧且 state 偶数 → 推 MetaCommand
  （payload 前 8B = i64::MIN 哨兵 + 16B 数据 @+0x60..0x6c 来源，FUN_140b59900 →
  VecDeque<MetaCommand>，meta_commander.rs）→ state=1；命令队列空（+0x18==0）→ 推
  `{tag: i64::MIN, byte: state}` 事件（stride 0x10，TimelapseReplayFinished/Interrupt 族）
  并清另一事件计数器。
- `cleanup_timelapse` 0x140a8a5d0：仍有未消费帧 → 日志（level 3）"The timelapse has
  finished"（模块串 `country_core::systems::timelapse_replay`）→ state=2。

### 2.11 arrow_gizmo_animation（O，0x140a92f80）

每实体（`EnableArrowGizmoStretch`）两个 f32 拉伸值（pfVar30[0]/[1]，U 语义，推测
双轴拉伸）。有输入事件（0x10B 步长，8B 向量 payload）：按**主轴**（|x|<|y| 取 y 轴）
`stretch = min(stretch + |delta|·0.02, 1.0)`；无事件：每帧衰减 `stretch = max(stretch − 0.1, 0)`。
常量 0.02@1429cd2d0、−0.1@1429eb6f8、1.0 钳位。Mut tick 回写 O。puffin scope
`country_core::systems::arrow_gizmo_animation::arrow_gizmo_animation`。

### 2.12 ui_camera_mode_gizmos（O，0x1409527a0 + closure 0x140952b60）

四重门槛：① AppMode ∈ {2, 10}（10 需 flag=0）；② 相机模式非 Free/Walk
（`(mode & 6) == 2` 返回）；③ 窗口 < 3px（宽或高 ≤ 2）；④ 签名符模式 ∈ {0,1,4}
（`0x13 >> mode & 1`，模式 6 先映射为 0）。通过后：`UiState::add("camera_mode_activerotation_")`
（状态 int != 20 时）→ 构造 0x20B UiAppendSignifier `{1,1,id_string,size=窗口字段,
fov=快照+0xe8}` → `AppendOnlyVec` push。闭包 = 渲染回调（"cursor_grab" 部件 +
vertex_color_shader，动画 `value/1000·20`，clamp 25.0@1429edb98）。

### 2.13 clear_canvas（O，0x140a7abd0）

事件驱动（EventReader 修剪流）。日志 "Clearing the canvas."（`country_core::systems::clear_canvas`）。
流程：`InnerWalls::remove_all`（FUN_14085a730，flag 时 +0xc0=0）→ 世界光栅
`clear_all<512>`（FUN_14093bee0）变体 0x13、0x14、6、5、7、1、2、3、4（+9、0xb、0xd、0xe）
→ 7× `clear_preset_texture`（FUN_140a15da0，3 个 color_id 0x5000=20KB 缓冲重置，
panic 串指向 color_id.rs）→ 上传记录/杂项清零（FUN_140aaae80×2、FUN_140aa5150、
FUN_1408950d0、FUN_14088acf0）。合同 ✓。

### 2.14 despawn_glade_entities（O，0x1408b3510）

Query 迭代，world 字节 @+0x108 二分：A 支（stride 0xa0，count@+0x58、实体数组@+0x50、
0x10B 元素）与 B 支（stride 0x48，count@+0x10、数组@+0x8、8B 元素）。逐槽逐实体
`Commands::despawn`（FUN_140d14770，file 串 despawn.rs）；u64 实体 id 高 32 位 == 0
即提前 return。合同 ✓。

### 2.15 look_at_camera（O，0x140873de0 / 0x1408756e0 / 0x140002370）

- `look_at_camera`：puffin scope；Query (LookAtCamera, Transform) 逐实体用共享工具
  FUN_140c96d40（transform 提取）计算朝向目标的方向/四元数，FUN_140c9ce00 写回实体
  Transform（位置+旋转），Mut tick 回写标记变更。
- `register_required_components` 0x1408756e0：**derive 生成的 Bevy 组件注册胶水**
  （28KB，递归调用自身 = required-components 递归；bevy_ecs-0.16.0/src/component.rs
  panic 串 + look_at_camera.rs）。骨架把它建模成"运行时插入 Transform 的系统"是错的，
  实为 LookAtCamera 的 `#[require(Transform, …)]` 注册（一次性 build 期调用）。
- `clone_behavior` 0x140002370：`xor eax,eax; ret`（返回 0 = 不克隆行为标志）。

### 2.16 daily.rs（O，closure 0x14083c440）

`get_name::closure$0`：`SystemTime::duration_since` 成功且 `elapsed_secs > 0x6977f`
（432511s ≈ **5.006 天**，非 7 天——合同/骨架 `7*86400` 与证据不符，纠正）才为真。
参数对象含时间戳 @+0xe0..+0xe8（u32 对 + u64）。

### 2.17 其余资源（引用 wp21 misc-resources.md，本域未重验）

- `GladeSettings`：load 0x8bbbc0 读 `glade_settings.json` serde_json Option 回退 Default；
  Default 0x8c2240 = `{+0x0 u32=0, +0x8 f32=0.01, +0xc f32=1.0, +0x10 u32=8,
  +0x18 f32=0.0, +0x1c f32=1.0, +0x20 u8=1}`。
- `Weather`（audio 消费）、`SparkleSsbo`（空 bundle）、`TimeOfDay(pub f32 hour)`、
  `TodEasing`（default fastrand 选段）、`TempUiId`。

## 3. 方法/系统行为（关键函数逐条）

| 函数 | 状态 | 要点 |
|---|---|---|
| `FreeCamera::get_position` 0x140992f10 | O | typeid 0x6b8463577afc1a66 查 Position driver，sret Vec3 |
| `FreeCamera::get_yaw_pitch` 0x140993010 | O | typeid 0x718e4fd258330fe5 查 YawPitch，返回 (yaw,pitch) 首 8B |
| `FreeCamera::translate_screen_space` 0x1409938b0 | O | 旋转移动→clamp xz ±91→terrain.max(0)+0.75→smooth(dt)=1−exp(−8dt)→delta×20/60→Position.translate；常量全实证（±91@142aba9b4/8、0.75、20/60） |
| `Free/Walk::update` 0x140993d30 | O | dolly rig 迭代：vtable+0x30(driver, parent_transform, dt)，写回 rig+0x18..0x34，sret 28B {pos,rot} |
| `WalkCamera::teleport` 0x140994130 | O | Position driver 写 (x,y,z)；方向向量→yaw/pitch 度（FUN_140334d80+FUN_141766a90 × rad2deg 常量 0x142aba9e0/9e4）。骨架用 quat 基构造是近似，实为欧拉转换 |
| `WalkCamera::rotation` 0x140994310 | O | 16B 拷贝 `[self+0x18]`→sret（rig.final_transform 首 16B 语义 U，可能含位置） |
| `WalkCamera::constrain_to_be_above_terrain` 0x140994320 | O | 两段 spherecast（filter 0x20 先、0x30cf/0 后，半径 0.45@142a7fca4，XZ clamp ±64.9）；地面下界 y−1.9−1.4；terrain blend（skip_blend 分支 +0x298 选择 vtable 对）；exp 平滑 `1−exp(−last_dt·20)`；ground_hit kind 0xb / 无命中 0x11 |
| `NormalCamera::constrain/translate` 0x140949dc0/9e70 | O | pending 公式与骨架一致（max(terrain+0.75, y+pending.y)；ratio=max(y/−fwd.y,1)/50 绕 yaw 旋转） |
| `NormalCamera::update` 0x140949fe0 | O | 结构同骨架（smooth、radial_max@+0x60 clamp、loose@+0x6c 投影、pitch/yaw 增量、retain 阈值）；每模式常量表在 reconstruction/camera 已机器码校验，本域不重推导 |
| `ButterflyCamera::update` 0x140a0d090 | O | `selected(+0x3c) != applied(+0x38)` → applied=selected、dt=1000.0（@14295b630），再跑 rig；三个空操作折叠到 0x14003bb80（`ret`） |
| `MainCamera::match_free_camera_to_normal` 0x14090e6f0 | O | 读 Normal position(+0x138..0x144)/yaw-pitch(+0x150) → 重建 Free rig@+0x270 = Position+YawPitch+Smooth(1,1) → dt=0 跑一遍 → 存 final_transform |
| `MainCamera::chase_butterfly` 0x140911520 | O | mode=0（Normal）；由蝴蝶 Transform 算轨道四元数；写 +0x144/+0x14c（相对旧位置 @+0x138 的 delta）；+0x118=0；push 28B ActiveRotation 到 +0x100 Vec（含 −1.0 常数）；flag 时 +0x194 = 蝴蝶旋转分量；π/180@142aab4b0 |
| `MainCamera::store_parameters` 0x140911710 | O | 读 yaw@+0x154/pitch@+0x150/position@+0x138/fov@+0x190，NaN→0（位置默认 (0,5,0)@142aab4c0）并分别日志 "Camera {position|yaw|pitch|vertical_fov_degrees} invalid. Overriding param cache value."（file camera_rig.rs）；输出 0x24B 记录 |
| `MainCamera::immediately_finish_interpolating` 0x140911b50 | O(面) | 巨函数（106KB，含相邻函数粘连）：模式@+0x4d0 + 快照字节@+0x134 双 jumptable；调 store_parameters/chase_butterfly/reset/MainCamera::new；fov 系数 +0xec×0.1；多 body memcpy 0x100 矩阵块；含 winter==4 判别（冬季分支传入相机相关调用） |
| `MainCamera::eye_position` 0x140911df0 | O | 直接尾调 FUN_140c96d80（utils::camera 共享工具） |
| `MainCamera::reset` 0x140911280 | O(部分) | f32 位检查前导 + 后续（NotFound，见 0x140912a40 的 reset_camera 是系统层） |
| `reset_camera` 0x140912a40 | O | DebugGui 层>4 日志 "Resetting the camera." → `MainCamera::new(旧 aspect@+0x4d8)`（0x14090ec70）→ memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24 元素 free，cap 留 4）→ 推 WriteSettingsFileCmd（8B 事件） |
| `main_camera_update` 0x140912c20 | O | aspect=w/h；NewSessionStarted 标志（+0x18）下窗口>2px 且光标(0,0)→跳过不消耗标志；窗口<3px→推 WriteSettingsFileCmd；鼠标深度字节→+0x344；DebugGui 模式滑块（+0x11c/0x11d，6→0）；DebugGui 自由相机开关(+0x168)→jumptable(0x142aabd30)；否则按 ActionSetStates(+0x10) jumptable(0x142aabd04) |
| `paused_main_camera_update` 0x1409145a0 | O | aspect=w/h（u32 对）；模式 0..5 → body 指针+偏移；调 body 更新 fn 指针；fov 相关退化读（+0x104/+0x108，反编译损坏一处）；`CameraProjectionParams::calc_matrices`（FUN_140c8ff70）+FUN_140c97390 → memcpy 0x100 → MainCamera@+0x0；case 6..11 部分损坏（bad instruction data），case 7..10 的 fov 对已录 |
| `MainCameraPlugin::build` 0x1409125e0 | O | 注册 2 系统（hash 0x142c96f70 / 0x142c5fd60）+ 2 资源（typeid 对 0x80c70da6101ec1a/0x668ae4faf3605349）+ 事件注册 + 调度（FUN_140a500f0/FUN_140672ed0，hash 0x142c6bd60）。run_if 条件为 hash 编码（合同的新会话/暂停条件 I 保留） |
| `time_update` 0x140922c30 | O | 见 §2.5；重放帧 delta @+0xc8(0xd0B 帧)；模式 10/7/2 分支 + byte flag |
| `time_of_day` 0x140a73460 | O | 见 §2.6；puffin `country_core::systems::time_of_day::time_of_day`；越界 unwrap panic（§4） |
| `snapshot_at_tod` 0x140b067d0 | O | 见 §2.8 |
| `weather_system` 0x1408e5b00 | O | 见 §2.9 |
| timelapse 三系统 | O | 见 §2.10 |
| `ui_camera_mode_gizmos` + closure | O | 见 §2.12 |
| `arrow_gizmo_animation` | O | 见 §2.11 |
| `clear_canvas` / `despawn_glade_entities` | O | 见 §2.13/2.14 |
| look_at 三函数 | O | 见 §2.15 |
| daily closure | O | 见 §2.16 |

ECS 参数面判别依据（PLAYBOOK §4）：`param[2] = param+0x1c` tick 回写 + 首字段指针 =
`&mut Mut<T>`（ResMut）；无回写 = Res；EventReader = 双缓冲 {counter 差 + 0x18/0x38/0x40}
修剪流模式；EventWriter = Vec grow（0x140562870/630 族）+ len 自增；Commands = world 指针
+ despawn 调用。

## 4. panic/日志串清单（原文 + 地址）

| 原文 | 地址（串） | 位置 |
|---|---|---|
| "Resetting the camera." | 0x142aabaf8 (fat) | reset_camera 0x140912a40（DebugGui 层>4） |
| "Camera position invalid. Overriding param cache value." | 0x142aab798 (fat) | store_parameters 0x140911710 |
| "Camera yaw invalid. Overriding param cache value." | 0x142aab818 (fat) | 同上 |
| "Camera pitch invalid. Overriding param cache value." | 0x142aab878 (fat) | 同上 |
| "Camera vertical_fov_degrees invalid. Overriding param cache value." | 0x142aab8e8 (fat) | 同上 |
| "The timelapse has finished" | 0x142ad90e8 (fat) | cleanup_timelapse 0x140a8a5d0（level 3） |
| "Clearing the canvas." | 0x142ad6f38 (fat) | clear_canvas 0x140a7abd0 |
| `called Result::unwrap() on an Err value` | 0x142ad5ee8 | time_of_day 0x140a73460（day_of_year ∉ 1..365） |
| `No <…> driver found in the CameraRig`（Position/YawPitch/Rotation/LookAt 四变体） | 0x142aecec8 / 0x142aecdc0 / 0x142aecde3 / 0x142aece05 | dolly 0.6.0 rig.rs（各 driver 查找失败） |
| `overflow when adding durations` | 0x142aad928 | time_update 0x140922c30（Duration 进位溢出） |
| `cannot convert float seconds to Duration: value is either too big or NaN` | 0x142aad61b | time_update（f32→Duration 内联 std） |
| `Out of bounds access` | 0x142ab3224 | time_update（str::pattern 内联） |
| `System built with a different world than the one it was added to.` | 0x1429cc860 | 0x1403b4ca0 邻接（bevy_ecs） |
| 音频参数名 "TOD-Enabled"/"Timetravel"/"DayNight" | 0x142ad63ef / 0x142ad63fa / 0x142ad6404 | time_of_day（AudioCoreInner::set_parameter） |
| 会话相机记录字段名 `yaw`/`pitch`/`position`/`camera_rig_arm_length`/`pitch_offset`/`yaw_offset`/`position_cache` | 0x1429cca00..cca3f | 0x1403b4ca0（ron 反序列化，resources/session.rs） |
| CameraRotationPivot 7 变体名（串见 §2.2） | 0x14292bb7c 起 | ron serialize 0x14003ac90 |
| 模块路径串 | 0x142aabb20（`country_core::systems::main_camera`）、0x142aabb98 表邻接、0x142ad9110（timelapse_replay）、0x142ad6f90（clear_canvas）、0x142ad9ffb（arrow_gizmo…）、0x142ab4418（`camera_mode_activerotation_`） | 各日志/UI 注册 |

## 5. 与现有合同/代码的差异（代码写 X，证据为 Y）

1. **合同 time-weather.md：remap_hour_to_fmod_hour y 基值 {48,17,11,24,22,…} → 实为 {0,3,11,17,22,24}**（常量 @142ad59b0/b8/c0）。x 基 {0,3.5,9,17,20,24} ✓，冬季 4.52/5.0/6.2/9.0 ✓。
2. **合同：`camera_snapshot @0x140911e40` → sysres/PDB 名 `CameraSnapshotInterpolationState::is_interpolating`**；机器码 = 按模式快照更新分发（mode@+0x18、dt×fov@+0xec）。地址同、名字与语义注释错位。
3. **合同：CameraMode 6 变体 → 模式字节有 13 个槽（0..12）**：fov 表 0x142aabb98 共 13 项；`previous` 表 [6,0,1,2,3,4,0,0]（Normal.previous=6）；paused update switch 有 case 6..11；gizmo/main_camera_update 都把模式 6 映射为 0。拍照/工具模式 = 复用 6 body + 各自 fov 槽。
4. **合同：`next @0x1407858016 / previous @0x1408230976` → 地址是截断垃圾**。真实 `previous` @0x140914ac0；`next` 未定位（本域未解）。
5. **合同：time_update 时钟 "total_f32@+0x10" → 实为 f64**（`(double)dt + (double)old` 存 u64）。
6. **合同：小时倍率 @+0x118 → 实读 +0x23**（f32，min 1.0，喂 `exp(−3·dt·speed)`）；@+0xe3 热键开关 ✓。
7. **合同：CameraRotationPivot 布局 U → 实为 7 变体枚举**（ron 变体名实证，见 §2.2），骨架把它写成空 struct。
8. **骨架 timelapse_replay.rs：`is_active = self.0 == 1` → 机器码 `state != 2`**（0x14092e6b0）。
9. **骨架 daily.rs：`is_weekly` 7×86400 → closure 阈值 0x6977f = 432511s（≈5.006 天）**（get_name 内闭包；is_weekly 本体未在本批）。
10. **骨架 look_at_camera.rs：register_required_components 建模为运行时系统 → 实为 derive 生成的一次性组件注册胶水**（bevy_ecs component.rs 递归）。
11. **骨架 walk.rs `teleport` 用 quat 基构造方向 → 实为方向向量→yaw/pitch 度欧拉转换**（FUN_140334d80/FUN_141766a90 × 度常量）。`WalkCamera::rotation` 0x140994310 是 16B 拷贝（[self+0x18]），非 rig.final_transform.rotation（骨架语义有偏差，见 §7 未决）。
12. **合同 time-weather.md weather：`intensity@+0xc += (sin>0 ? +0.1·frac : −0.1·frac)` ✓ 全证**（frac = GameTime+0x28 f32 = 帧 delta）；glade_type@+0x58==4 ✓。
13. **合同 camera.md 其余（body 偏移、0x4e0、+0x4d0、+0x4d8、dolly 接线、MainCameraPlugin 注册面）与证据一致** ✓；补充：MainCamera@+0x0..0x100 = CameraMatrices 块（paused update memcpy 0x100）；`MainCamera::new` @0x14090ec70（合同未给地址）；reset_camera 保留旧 aspect 传给 new。

## 6. 跨领域线索（地址 + 名称，供中央索引）

- `mode_manager::check_if_can_switch<closure>`（FUN_140af9e00）← start_timelapse（ui-input 域 mode_manager）。
- `AudioCoreInner::set_parameter`（FUN_140a53db0）← time_of_day 上报 TOD-Enabled/Timetravel/DayNight（audio-misc 域）。
- `suncalc::get_position`（FUN_1417ab4b0）← time_of_day（复用 suncalc crate，输入含 3600000@142ad58f0 小时→毫秒）。
- `resources::session` 相机记录反序列化（0x1403b4ca0）→ 0x24B CameraRigParametersCache（session-save 域）。
- `InnerWalls::remove_all`（FUN_14085a730）、`WorldRaster::clear_all<512>`（FUN_14093bee0，变体 1..0x14）、`AssetTextureLibrary::clear_preset_texture`（FUN_140a15da0）← clear_canvas（wall/terrain-water/audio-misc 域）。
- `InputRecordReplayState`（mode 字节 @+0x0；重放帧 0xd0B、game delta @+0xc8；i64::MIN+2 @+0x40 判 Replaying）← time_update（ui-input/input_stream 域，wp17 回放编排相关）。
- `TodSequence::snapshot_at_tod`（0x140b067d0）← time_of_day 每帧光照快照插值。
- utils::camera 共享工具 FUN_140c96d40 / FUN_140c96d80（transform 提取/eye）被 normal.rs / look_at_camera / eye_position 共用（utils crate，wp16 之外）。
- `PhotoModeCameraMode`（photo_mode.rs，next_demo 0x140b596b0）↔ 模式字节 6..12（ui-input 域，待联动核实变体名）。
- `TodEasing` 默认用 fastrand（FUN_14132ade0 域，PLAYBOOK 事实 5）。

## 7. 错位/纠错

- PDB `MainCamera::xz_projective_rotation` @0x1403b4ca0 = **ron 反序列化器**（写 '{' + 逐字段
  deserialize 会话相机记录）；真身或在邻接地址，未定位。
- PDB `CameraSnapshotInterpolationState::is_interpolating` @0x140911e40 = 快照按模式分发（见 §5.2）。
- PDB `TimelapseState::is_active` @0x14092e6b0 = `state != 2`（§5.8）；相邻 `==3`/`<2` 谓词归属未定。
- PDB `apply_camera_snapshot_op` @0x1408b99e0：该地址首函数 = `byte[rcx]+1`（`next` 风格的小步进），
  真函数从 +6（0x1408b99e6，`55 48 83 ec 50…`）开始——地址指向 impl 族首个小函数。
- PDB `clone_behavior` @0x140002370 = `return 0` 平凡函数。
- 合同 `next`/`previous` 地址 0x1407858016/0x1408230976 为截断垃圾（§5.4）。
- `WalkCamera::rotation` 0x140994310 复制 `[self+0x18]` 16B（= final_transform.position.xy +
  rotation.x？），与骨架 `final_transform.rotation` 语义有出入——**未决**：需对照 dolly
  CameraTransform 内存序（position 12B + rotation 首 4B）确认是否位置+旋转混合拷贝。

## 8. 未解决疑问（交 main）

1. CameraMode 变体 6..12 的确切名字/语义（拍照模式相机 7 种？）——需 ui-input 域
   photo_mode.rs（next_demo 0x140b596b0）联动；`previous` 表只覆盖 0..7。
2. `WalkCamera::rotation` 的 16B 拷贝语义（位置+旋转混合 or 布局不同）。
3. TodLightingSnapshot 两个 u64 字段（@+0x20/+0x28，拷贝不插值）的语义（颜色打包 id？）。
4. suncalc 的固定纬度/经度常量（@142ad5910..5984 一组 f32）与日期位打包
   （`uVar13 | 0xfce00`）的确切编码。
5. time_of_day 的 param_3（f32 资源，写入光照结构尾部）+ param_10/11（双 EventReader
   合并写 piVar29[8]）的类型名。
6. `MainCamera::reset` 0x140911280 全貌（NotFound，仅有前 64B 手解：f32 位检查）。
7. GameTime 头部布局（+0x0/+0x8 = game_time Duration；+0x10/+0x18 = delta Duration；
   +0x28 = delta f32）之外的字段。
