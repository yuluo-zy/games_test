# systems/camera_rig/walk.rs

> 文件作用：**WalkCamera（行走相机）**——dolly rig（Position+YawPitch，无 Smooth）+ 碰撞滑行：
> 布局 {rig@+0x0、ground_hit: Option<CastHit>@+0x38（判别 0xb 命中/0x11 空）、
> last_translation_dt@+0x50、allow_vertical@+0x54}；8 步 spherecast 滑行（filter 0x30cf、半径 0.5）、
> 地面球投（filter 0x20、半径 0.45）+ 地形 blend + 指数垂直平滑；XZ 钳 ±34。
> 分叉 CameraTransform rotation-first（rotation@+0x18 = 四元数，`rotation()` 拷贝 16B）。

函数总计 9：业务 9 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$2::translate_screen_space` | 0x140994700 | 0 | 8 步 spherecast 滑行：`q@+0x18` 旋转移动→flat（allow_vertical 时保留 y）→step=8/60 归一→8 次 cast（0.5 半径/0x30cf/法向滑移 t·0.9）→±34 钳→smooth→Position.translate | O：实证（quat@+0x18..0x24 实读；1e-10 阈值） | camera-time\decomp\decomp-0x140994700.txt |
| `WalkCamera::new` | 0x140993e20 | 35 | 构建 rig=Position(0,1,0)+YawPitch(0,0)（无 Smooth）+ 附加状态零化 | O：实证（(0,1,0)@1429345b0） | camera-time\decomp\decomp-0x140993e20.txt |
| `WalkCamera::teleport` | 0x140994130 | 49 | Position driver 写 (x,y,z)；方向向量→yaw/pitch 度（FUN_140334d80+FUN_141766a90 × rad2deg 常量） | O：实证（骨架 quat 基构造为近似，实为欧拉转换） | camera-time\decomp\decomp-0x140994130.txt |
| `WalkCamera::rotation` | 0x140994310 | 58 | 返回四元数：拷贝 `[self+0x18]` 16B（分叉 CameraTransform rotation-first） | O：CLOSED（round2；xrefs=0 纯 vtable 调用） | camera-time\decomp\decomp-0x140994310.txt（hex 手解） |
| `impl$2::rotate_yaw_pitch` | 0x140993750 | 66 | 同 free（共享）：yaw fmod 720、pitch 钳 ±90 | O：共享函数 | camera-time\decomp\decomp-0x140993750.txt |
| `impl$2::constrain_to_be_above_terrain` | 0x140994320 | 74 | 两段 spherecast（filter 0x20 先、0x30cf/0 后，半径 0.45，XZ 钳 ±64.9）→地面下界 y−1.9−1.4→terrain walk_height_blend→`y=lerp(y,target,1−exp(−last_dt·20))`；ground_hit kind 0xb | O：实证（常量全录 DIGEST） | camera-time\decomp\decomp-0x140994320.txt |
| `impl$2::update` | 0x140993d30 | 288 | dolly rig 迭代（共享） | O | camera-time\decomp\decomp-0x140993d30.txt |
| `free::impl$1::rotate_yaw_pitch` | 0x140993750 | 66 | 同共享函数 | O | camera-time\decomp\decomp-0x140993750.txt |
| `free::impl$1::update` | 0x140993d30 | 288 | 同共享函数 | O | camera-time\decomp\decomp-0x140993d30.txt |
