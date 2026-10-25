# camera-time 域第二轮审查（tg_a6，2026-10-14）

对照本域 DIGEST 的 O 条目逐文件审查 `reconstruction/country-core/src` 现有实现。
状态：ok=证据一致 / 需修=有具体错误或签名不符 / 未知=无法判定的占位。
**未改任何 reconstruction 源码**（本文件只登记差异）。

## 每文件审查表

| file | 状态 | 具体错误（代码写 X，证据为 Y） | 缺失项 |
|---|---|---|---|
| systems/camera_rig/mod.rs | 需修 | 文档声称"模式字节@+0x4d0 选 6 body"——证据：模式槽 0..12（13 槽，focus/lens 表各 13 项 @0x142aabb68/@0x142aabb98）；MainCamera@+0x0..0x100 = CameraMatrices 矩阵块未记；`MainCamera::new` @0x14090ec70、`mode/focus/focus_mut/lens/lens_mut` @0x140911230/240/260、`zoom` @0x140911280、`match_free_camera_to_normal` @0x14090e6f0、`init_walk_camera` @0x14090e510、`load_parameters` @0x140911520、`CameraModeStates::current_dyn` @0x1409149d0 全缺 | 13 槽模式字节模型；7 个 MainCamera 方法 stub（new/eye_position 等 return 零值） |
| systems/camera_rig/camera_body.rs | ok | 无（CameraMode 6 核心变体 ✓；lerp/smooth(1−exp(−8dt))/bounds_xz 机器码实证） | 模式 6..12 = 拍照/工具槽位的注释；CameraTransform 在 dolly 分叉内为 rotation-first（0x140994700 读 +0x18 作四元数、0x140994310 拷贝 +0x18 16B）——仅影响内部布局认知 |
| systems/camera_rig/free.rs | ok | 无（typeid 0x6b8463577afc1a66 Position / 0x718e4fd258330fe5 YawPitch、±91 clamp、terrain+0.75、20/60、smooth 全实证） | — |
| systems/camera_rig/normal.rs | ok | 无（position@+0x38/pending@+0x44/yaw@+0x50/pitch@+0x54/radial_max@+0x60/loose@+0x6c、max(terrain+0.75)、ratio=max(y/−fwd.y,1)/50 全实证） | `zoom` 的 lower 上界来自模式表（MainCamera::zoom 的 table2[(mode==1)?1:0]），skeleton 用参数传——语义等价 |
| systems/camera_rig/walk.rs | 需修 | `teleport` 用 quat 基构造方向——实为方向向量→yaw/pitch 度欧拉转换（FUN_140334d80+FUN_141766a90，rad2deg 常量 0x142aba9e0/9e4）；`rotation()` = 拷贝 [self+0x18] 16B（分叉 CameraTransform rotation-first）——skeleton 语义对但布局认知错 | ground_hit 判别 0xb（命中）/0x11（无命中）注释 |
| systems/camera_rig/butterfly.rs | ok | 无（selected/applied @+0x38/+0x3c、dt=1000 吸附、空 trampoline 0x14003bb80） | rig 各 Smooth 参数（2,predictive/4/4）为合同继承（I），未本域重验 |
| systems/main_camera.rs | 需修 | `camera_snapshot` 签名错：实参 =（Mut<快照：mode u32@+0x18、f32@+0x1c>、Res<时间：dt@+0x28>、Mut<…>、Res<MainCamera：fov 系数@+0x0ec>）逐模式 jumptable——skeleton 写成 (ResMut<MainCamera>, ResMut<CameraSnapshot>, EventWriter)；`paused_main_camera_update` 参数顺序错：实为 (&mut Mut<MainCamera>, Res<WindowSize{u32 w,h}>，…)，aspect=w/h 写 @+0x4d8、memcpy 0x100 矩阵块进 @+0x0；`CameraRotationPivot` 是 **7 变体枚举**（CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain，ron 0x14003ac90 + enum_iterator Sequence 0x1408b99e0/0x140914ac0），skeleton 写成空 struct | `next/previous` 实现（previous 表 [6,0,1,2,3,4] @0x140914ac0）；`apply_camera_snapshot_op`；mode 6→0 的 DebugGui/UI 映射 |
| systems/look_at_camera.rs | 需修 | `register_required_components` 建模为运行时插入 Transform 的系统——实为 Bevy derive 组件注册胶水（bevy_ecs-0.16.0 component.rs 递归，一次性）；`clone_behavior` = `return 0` 平凡函数 | look_at 本体的朝向数学在共享工具 FUN_140c96d40/FUN_140c9ce00（utils::camera），LookAtCamera{target,up} 字段布局 I |
| systems/time_of_day.rs | 需修 | 文件头 doc 的 remap 基值 {48,17,11,24,22,…} 错——实为 {0,3,11,17,22,24}（常量 @142ad59b0/b8/c0）；系统签名缺 6 个参数（实 13 参：AudioCoreInner@+0x10 上报、TodSequence、f32 资源、GameTime(dt@+0x28)、按键、sun-sky、播放态、光照、dev 态(小时倍率@+0x23/热键@+0xe3)、双 EventReader、GladeType、当前态） | remap 冬季插入 5.0/6.2/9.0 的注释；状态机/音频参数 "TOD-Enabled/Timetravel/DayNight" 说明 |
| systems/time_update.rs | 需修 | doc 写 "total_f32@+0x10"——实为 f64（`(double)dt + (double)old` 存 u64） | 重放帧 delta@+0xc8（0xd0B 帧）、i64::MIN+2 @+0x40 判 Replaying、前两帧 clamp 1/60 的注释 |
| systems/weather_system.rs | ok | 无（glade_type@+0x58==4、sin(game_time/300·π)、±0.1·dt@+0x28、intensity@+0xc、clamp[0,1] 全实证；1e9@1429a7640、300@142aa5140） | — |
| systems/timelapse_replay.rs | 需修 | `is_active = self.0 == 1` 错——机器码 0x14092e6b0 = `state != 2`；start/finish/cleanup 签名不全：实为 EventReader 修剪流 + check_if_can_switch（0x140af9e00）+ MetaCommand 推入（i64::MIN 哨兵，meta_commander.rs）+ 事件 {tag,byte}（stride 0x10） | 相邻谓词 `==3`/`<2`（0x14092e6b7/be）归属未定 |
| systems/arrow_gizmo_animation.rs | 需修 | doc 无行为描述；实为：有事件按主轴 `stretch=min(stretch+|δ|·0.02, 1.0)`（0.02@1429cd2d0），无事件每帧 `stretch=max(stretch−0.1, 0)`（−0.1@1429eb6f8），双 f32 拉伸值 + Mut tick 回写 | 事件 payload（8B 向量，0x10B 步长）的类型名 |
| systems/ui_camera_mode_gizmos.rs | ok | 无（四重门槛 AppMode∈{2,10}、非 Free/Walk、窗口<3px、模式∈{0,1,4}+6→0 映射、UiState::add("camera_mode_activerotation_")、0x20B 签名符 {1,1,id,size,fov@+0xe8} 全实证） | 闭包渲染细节（cursor_grab 部件、value/1000·20、clamp 25.0） |
| systems/clear_canvas.rs | ok | 无（InnerWalls::remove_all、光栅 clear_all<512> 变体 0x13/0x14/6/5/7/1/2/3/4(+9/0xb/0xd/0xe)、7×clear_preset_texture、日志 "Clearing the canvas." 全实证）；签名可补 EventReader | 上传记录重置的 3 个 color_id 语义 |
| systems/despawn.rs | ok | 无（@+0x108 二分、stride 0xa0/0x48、u64 高 32 位==0 提前返回、Commands despawn） | 两个实体 id 数组的资源名 |
| resources/time_of_day.rs | 需修 | TodLightingSnapshot 字段用 a/b/c/d/e/f 占位——现已实证：sun_sky(Vec4@+0x0)、fog_tint(2×u64@+0x20，**不插值**)、base_fog_thickness(4×f32@+0x30)、speed_bias(f32@+0x40)（ron 字段名 0x142930744/75d/74b/765）；`TimeOfDay(pub f32)` 不完整——会话序列化外层资源 = {time_of_day, snapshot, temp_ui_ids}（0x14007fd40） | 播放态资源 {state/progress/target_hour/angle/latitude@+0x10/sunrise@+0x14/sunset@+0x18/visible/day_of_year@+0x28} 的类型定义 |
| resources/time.rs | 需修 | 无错误，但 GameTime 布局已可补：{game_time Duration(secs@+0x0/nanos@+0x8), delta Duration(secs@+0x10/nanos@+0x18), delta f32@+0x28} | +0x20..0x27 与尾部字段（U） |
| resources/weather.rs | ok | 无（Weather 空资源 U 如实标注） | — |
| resources/daily.rs | 需修 | `is_weekly` 写 7*86400——实为 `elapsed_secs > 0x6977f`（432511s ≈ 5.006 天）；`get_name` 空实现——实为 locale 判别 jumptable（[rdx+4] 首字符匹配 S/E/F/R/T/J 族，en-US/de-DE/es-ES/fr-FR/it-IT/ja-JP 风格常量，r8==5 门） | 主题名的完整常量表 |
| resources/glade_settings.rs | ok | 无（Default {0,0.01,1.0,8,0.0,1.0,1} 与 load=serde_json Option 回退 均与 wp21 合同一致） | — |
| resources/sparkle.rs | ok | 无（SparkleSsbo 空 bundle，wp21 misc-resources 一致） | — |

## 统计

- ok：10 文件（camera_body、free、normal、butterfly、weather_system、ui_camera_mode_gizmos、clear_canvas、despawn、weather、glade_settings、sparkle = 11）
- 需修：11 文件（mod、walk、main_camera、look_at_camera、time_of_day、time_update、timelapse_replay、arrow_gizmo_animation、resources/time_of_day、resources/time、resources/daily）
- 未知：0（无 stub 文件；mod.rs 属需修而非 stub）

## 第二轮新纠错（相对第一轮 DIGEST）

1. **0x140911280 不是 reset 而是 `MainCamera::zoom`**（v2 名）；函数边界 0x911280–0x9114c6（pdata 表实证）。
2. **0x140911520 不是 chase_butterfly 而是 `MainCamera::load_parameters`**（v2 名）——从会话 CameraRigParametersCache 记录恢复 Normal 相机（模式=0、轨道四元数、28B ActiveRotation 入 rotations Vec、flag 时 +0x194）。chase_butterfly 地址未在 v2 出现（或内联/别处）。
3. **0x1403b4ca0 是 Serialize（非反序列化）**；deserialize 在 0x1403b4e10/0x1403b5000。
4. **0x140911e40 = camera_snapshot**（v2 名 h1fb3daeef0dd27ee）——第一轮据 v1 名 is_interpolating 记的"错位"撤销，wp20 合同原地址正确。
5. **0x14083c440 = is_weekly 本体**（v2 名）——第一轮记的 get_name::closure$0 即其内联闭包，语义一致。
6. **0x1408b99e0/0x140914ac0 = enum_iterator::Sequence for CameraRotationPivot**（7 变体枚举）——非 CameraMode。
7. **0x140911dd0/df0/e10 = eye_position/eye_forward/xz_projective_rotation**（三个 utils::camera 转发器，各 12B）。
8. **0x140911230/240/260 = mode/focus/lens**（getter，表 @0x142aabb68/98）。
9. **dolly 分叉 CameraTransform = rotation-first**（rotation@+0x18 16B、position@+0x28 12B）——stock dolly 0.6.0 为 position-first，游戏分叉改序。
