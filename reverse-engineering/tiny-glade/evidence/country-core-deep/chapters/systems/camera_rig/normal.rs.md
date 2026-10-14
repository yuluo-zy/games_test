# systems/camera_rig/normal.rs

> 文件作用：**NormalCamera（轨道相机）**——Normal/TiltShift/TopDown 三种模式共用的轨道 body：
> 布局 {rotations Vec@+0x0(28B 元素)、active_rotation@+0x18(判别 1=Some, 数据@+0x1c)、
> position@+0x38、pending_translation@+0x44、pitch@+0x50、yaw@+0x54、pitch_min@+0x58、
> pitch_max@+0x5c、radial_min@+0x60、radial_max@+0x64、terrain_clearance@+0x68、loose@+0x6c}；
> 平滑轨道 + 径向钳 + 松散地面投影（每模式常量在 reconstruction/camera 已机器码校验）。

函数总计 11：业务 4 / 内联无独立符号 5 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$1::update` | 0x140949fe0 | 0 | 每帧：smooth 推进、radial_max@+0x60 钳、loose@+0x6c 地面投影（lookahead 90·0.2 或 33.45/46.0 边界）、ActiveRotation 弧长插值、pending 应用、pos.y≥1.0、retain | O：结构与常量实证（每模式数值沿用 reconstruction/camera 已验证） | camera-time\decomp\decomp-0x140949fe0.txt |
| `impl$1::rotate_yaw_pitch` | 0x140949cc0 | 45 | active_rotation 存在时：base=sum(rotations.pitch_delta)+pitch@+0x50；bounded=clamp(−(pitch+base), pitch_min@+0x58, pitch_max@+0x5c)；active.pitch_delta += −bounded−base；yaw_delta += yaw | O：实证（+0x24/+0x28 写回） | camera-time\decomp\decomp-0x140949cc0.txt |
| `impl$1::constrain_to_be_above_terrain` | 0x140949dc0 | 74 | pending.y = max(pending.y, terrain_height(pos.xz+pending.xz)+clearance@+0x68 − pos.y)；x/z 恒等折叠 | O：实证 | camera-time\decomp\decomp-0x140949dc0.txt |
| `impl$1::translate_screen_space` | 0x140949e70 | 94 | ratio=max(pos.y/−fwd.y,1)/50；pending += 绕 Y(yaw) 旋转的 movement × ratio | O：实证（50@1429cd070） | camera-time\decomp\decomp-0x140949e70.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `NormalCamera::rotation`（行 39）— pitch/yaw（度）→ 四元数（YXZ × π/180，共用 pitch_yaw_rot）
- `ActiveRotation::is_finished`（行 202）— 谓词/校验：|pitch_delta|<0.001 && |yaw_delta|<0.001（retain 条件）
- `constrain_position`（行 211）— 位置约束分发（按 loose 选 normal/semi_ortho，并入 update）
- `constrain_position_normal`（行 224）— Normal 模式位置约束：lookahead=forward·18、anchor 边界 46.0、radial_min 投影（并入 update 的 else 分支）
- `constrain_position_semi_ortho`（行 243）— TiltShift/TopDown 半正交约束：distance=((pos.y−10)/max(−fwd.y,0.15)).max(11.0125)、anchor 边界 33.45（并入 update 的 loose 分支）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
