# RESTORE — systems/camera_rig（卡D-slice1）

实现者: restorer-interaction
日期: 2026-10-15
范围: camera_rig 族主闭包 + CameraRigParametersCache 消费（衔接卡0 已建缓存）。time/weather 属下一子卡，先登记。

## 原函数 → Rust 位置 → 证据

| 原函数 | VA | Rust 位置 | 证据 | 状态 |
|---|---|---|---|---|
| `MainCamera::load_parameters` | 0x140911520 | `main_camera.rs::MainCamera::load_parameters` | chapter camera_rig.rs.md | O 结构（位置/yaw/pitch/模式/默认 fov）；ActiveRotation 待取证 |
| `CameraModeStates::current_dyn` | 0x1409149d0 | `mod.rs::CameraModeStates::current_dyn` | chapter（hex 手解 jumptable） | O 结构（body/fov 偏移分发） |
| `CameraModeStates::current_dyn_mut`（内联 155） | — | `mod.rs::CameraModeStates::current_dyn_mut` | 同上 | O 结构 |
| `CameraModeStates::current_body_mut`（内联 167） | — | `mod.rs::CameraModeStates::current_body_offset` | 同上 | O 结构 |
| `CameraState::new`（内联 105） | — | `mod.rs::CameraState` | chapter | I 结构 |
| `CameraRigParametersCache`（卡0 已建） | — | `mod.rs`（既有） | 卡0 域类型包 | 复用 |
| `default_vertical_fov_degrees` / `serde_default_vertical_fov_degrees` | 0x140911e30 | `mod.rs::default_vertical_fov_degrees`（既有） | 既有 | OK |

既有（未动）: `MainCamera::store_parameters` 0x140911710、`MainCamera::mode`/`focus`/`lens`/
`zoom`/`target_vertical_fov_degrees`/`immediately_set_vertical_fov_degrees`（main_camera.rs 已有）。

## 新增实现与依赖复用边界

- `MainCamera::load_parameters(&CameraRigParametersCache)`：恢复 Normal body
  （位置/yaw/pitch 直写，与 store_parameters 捕获同单位保证存档往返）+ 模式字节=Normal +
  fov=默认 45（fov 不在 6 字段存档）。π/180 轨道四元数、ActiveRotation 队列（rotations
  Vec@+0x100）、+0x118/+0x194 分量依赖 body 子结构，待取证。
- `CameraModeStates`：模式@+0x3d0 → `(body 区偏移, fov 槽偏移)`（六 body 区
  @+0x100/0x1b8/0x270/0x2f0/0x390/0x448；FOV_OFFSETS 13 槽复用 main_camera）。
- `CameraState`（I）：snapshot_byte@+0x134、fov_coefficient@+0xec（immediately_finish_interpolating）。

## 仍缺失/待取证

- `MainCamera::immediately_finish_interpolating`（0x140911b50，106KB，U 逐分支）。
- `MainCamera::init_walk_camera`（0x14090e510，地形采样+球投地面）、
  `match_free_camera_to_normal`（0x14090e6f0，dolly rig 重建）。
- `MainCamera::new`（0x14090ec70，六 body 默认装配）。
- `MainCamera::zoom`（0x140911280，exp2(clamp(amount·−0.08,−1,1))）、
  `MainCamera::reset`/`rotate_yaw_pitch`/`constrain_to_be_above_terrain`/
  `translate_screen_space`（内联转发）。
- `MainCamera::eye_position`/`eye_forward`/`xz_projective_rotation`（12B 转发器）。
- 模式 body 更新算法（butterfly/free/normal/walk/camera_body）。

## 编译状态

`cargo check -p country-core --lib` 0 error。测试二进制受其他 worker 在途错误阻塞
（meta_commander/save/session 组）。

## 二.卡D-slice2 追加（coordinator 续派，2026-10-15）

1. **`MainCamera::eye_position`**（O 0x140911dd0，12B 转发器）：读 Normal body 位置。
2. **`MainCamera::eye_forward`**（O 0x140911df0，`eye_direction` 同址）：Normal body
   yaw/pitch → `utils::camera` 前向（`CameraBodyMatrices` inverse_view）。
3. **`MainCamera::xz_projective_rotation`**（O 0x140911e10）：`utils::camera` XZ 投影旋转
   （projection 单位矩阵占位，仅用 inverse_view）。
4. **`MainCamera::reset`**（行 428，内联，I）：清缓冲 + 模式=Normal + fov=45 + 位置 (0,5,0)。
   新增 forwarder 测试。
5. **登记待取证**：`immediately_finish_interpolating`（0x140911b50，106KB）、
   `init_walk_camera`（0x14090e510）、`match_free_camera_to_normal`（0x14090e6f0）、
   `MainCamera::new`（六 body 装配）、`zoom`（exp2(clamp(amount·−0.08,−1,1))）、
   `rotate_yaw_pitch`/`constrain_to_be_above_terrain`/`translate_screen_space` 模式转发、
   模式 body 更新算法（butterfly/free/normal/walk/camera_body）。time/weather/daily 属
   下一子卡（TimeOfDay 节载荷衔接卡0）。

## 留给 reviewer 的核对点

1. `load_parameters` 直写单位与 `store_parameters` 往返一致。
2. `CameraModeStates::current_dyn` 的 body/fov 偏移分发（模式 0..5 + 模式 6）。
3. CameraRigParametersCache 6 字段消费（无 fov，fov 用默认 45）。
