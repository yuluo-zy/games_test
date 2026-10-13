# WP20 Contract — `systems::camera_rig` / `main_camera` / `look_at_camera`

Source: `skeleton/B/crates/country-core/src/systems/{camera_rig/*,main_camera.rs,look_at_camera.rs}.rs`
Evidence: Ghidra B-patched-EXE decompiles in `analysis/b2_camera/`,`b3_camera/`; PDB bundles
`camera_rig/{butterfly,free,normal,walk,camera_body}.rs`, `main_camera.rs`, `look_at_camera.rs`.
Reconciled with prior verified camera-body recovery `reconstruction/camera/` (machine-code
adjudicated constants).

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## camera_rig/*（5 文件）

- **CameraBody trait**（`camera_body.rs`，O）：`rotate_yaw_pitch(yaw,pitch)` /
  `update(dt) -> CameraTransform`。原实现 = `reconstruction/camera::camera_body`（已机器码校验，
  含 `TerrainQueries`/`CollisionQueries` 边界与 `CameraTransform{position,rotation}`）。
  本包只恢复骨架对齐接口，不重复已校验的数学。
- **FreeCamera / NormalCamera / WalkCamera / ButterflyCamera**（O 名）：
  - `FreeCamera` = dolly `CameraRig`（`Position`+`YawPitch`+`Smooth`）：`get_position`
    @0x140992f10（驱动查找 typeid `0x6b8463577afc1a66`）、`get_yaw_pitch` @0x140993010、
    `set` @0x140993100（重建 rig）、`translate_screen_space` @0x1409938b0（地形采样 +0.75 高度）、
    `update` @0x140993d30。
  - `NormalCamera`：`ActiveRotation` 结构（O）；`rotate_yaw_pitch` @0x140949cc0 /
    `constrain_to_be_above_terrain` @0x140949dc0 / `translate_screen_space` @0x140949e70 /
    `update` @0x140949fe0（smooth + radial clamp + loose ground projection）。三种模式
    Normal/TiltShift/TopDown 数值见 `reconstruction/camera/normal.rs`（O 常量）。
  - `WalkCamera`：`new` @0x140993e20 / `teleport` @0x140994130 / `rotation` @0x140993e90 /
    `constrain_to_be_above_terrain` @0x140994320（含 spherecast 地面查询 filter `0x20`/`0x30cf`、
    blend 采样、平滑上升）/ `translate_screen_space` @0x140994700（8 步 spherecast 滑行）。
  - `ButterflyCamera`：`update` @0x140a0d090——`selected_index != applied_index` 时强制
    `dt = 1000.0`（瞬时吸附），遍历 dolly 驱动 vtable@+0x30，输出 32B Transform；`rotate_yaw_pitch`
    @0x14003bb80（折叠，1 字节）为空操作。
- **插件/资源接线**（`main_camera.rs`，O）：
  - `MainCamera` = **0x4e0 B** 资源（`reset_camera` @0x140912a40 `memcpy 0x4e0`）。布局（O）：
    +0x100 Normal body、+0x1b8 TiltShift、+0x270 Free、+0x2f0 Walk、+0x390 TopDown、
    +0x448 Butterfly（`paused_main_camera_update` @0x1409145a0 jumptable），模式字节 **@+0x4d0**，
    宽高比 **@+0x4d8**（= 窗口宽/高）。
  - `main_camera_update` @0x140912c20：读 ActionSetStates / GameTime / WindowSize / DebugGui，
    写 CameraMatrices（0x100 B）与鼠标深度字节 @+0x344；窗口宽高 < 3 时推 `WriteSettingsFileCmd`。
  - `paused_main_camera_update` @0x1409145a0：`mode@+0x4d0` 分发到各 body，
    `utils::camera::CameraProjectionParams::calc_matrices` + `into_camera_body_matrices`。
  - `camera_snapshot` @0x140911e40：按模式 jumptable 分发（`*+0x18` 模式 u32），fov × 插值系数
    `@+0xec`。
  - `reset_camera` @0x140912a40：`MainCamera::new` + 清 `CameraRigParametersCache` Vec + 推
    `WriteSettingsFileCmd`。
  - `MainCameraPlugin::build` @0x1409125e0：注册 `main_camera_update`（run_if
    `NewSessionStartedCmd`）、`paused_main_camera_update`（run_if not `GameTime+AppLoadingState`）、
    `reset_camera`（run_if not），insert `CameraSnapshot`，register `MainCameraMoved` event。
  - `CameraSnapshot` / `CameraSnapshotInterpolationState` / `CameraRotationPivot` /
    `CacheFreeCamRig` / `MainCameraMoved`（O 名；布局 U）。`CameraMode` 枚举 6 变体
    （Normal/TiltShift/Free/Walk/TopDown/Butterfly，I 顺序见 schema.rs）。
- **look_at_camera.rs**（O）：`look_at_camera` @0x140873de0——`LookAtCamera` 组件按目标
  每帧 look-at；`register_required_components` @0x1408756e0（插入 `LookAt`+`Transform` 组件）。

## Inferred (I)
- `CameraTransform{position: Vec3, rotation: Quat}`（O 输出 28B/32B 写模式）。
- `MainCamera` 内部各 body 偏移（O jumptable 区段，精确字段 U）。

## Unknown (U / typed gaps)
- `MainCamera` 0x4e0 内部字段语义、`CameraSnapshotInterpolationState`、`CameraRotationPivot`、
  `CacheFreeCamRig` 布局；`CameraBody` 各 body 的 `pending_translation`/`rotations` 精确结构
  （见 `reconstruction/camera` 已校验实现，本包不重推导）。
- `utils::camera::CameraProjectionParams` 字段（wp16 之外，另属 utils crate）。

Reused deps: `dolly` 0.6.0（rig/drivers）、glam、bevy_ecs。不重复实现 dolly。
