# systems/camera_rig/camera_body.rs

> 文件作用：**相机 body 共享类型与 trait**——`CameraBody` 接口（rotate_yaw_pitch/update）、
> `CameraTransform`（游戏 dolly 分叉内为 rotation-first：rotation@+0x18 16B、position@+0x28，
> 0x140994700 实读四元数 + 0x140994310 拷贝 16B 坐实）、`CameraMode` 核心 6 变体（模式字节另有
> 6..12 拍照/工具槽）、lerp/smooth(1−exp(−8dt))/bounds_xz 共享数学（机器码实证）。

函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `camera_body::pitch_yaw_rot`（行 24）— 由 pitch/yaw（度）构造旋转四元数（YXZ 欧拉 × π/180；被 NormalCamera::rotation 与 load_parameters 使用，常量 0.0174533@142aab4b0 实证）

## 实现方式（算法级）

- `CameraBody` trait：rotate_yaw_pitch / update（每 body 实现）。
- `CameraTransform`：**游戏 dolly 分叉 rotation-first**——rotation Quat@+0x18 (16B)、position Vec3@+0x28 (12B)（round2 CLOSED；stock dolly position-first）。
- `CameraMode` 核心 6 变体；模式字节另有 6..12 拍照/工具槽。
- `pitch_yaw_rot`：由 pitch/yaw（度）构造旋转四元数（YXZ 欧拉 × π/180，0.0174533@0x142aab4b0）。
- lerp / smooth(1−exp(−8dt)) / bounds_xz 共享数学（机器码实证）。
