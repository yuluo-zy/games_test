# systems/camera_rig.rs

> 文件作用：**相机装配中心**——`MainCamera`（0x4e0B：@+0x0 矩阵块 0x100B、@+0x100/1b8/270/2f0/390/448
> 六个 body、@+0x4d0 模式字节 0..12 共 13 槽、@+0x4d8 aspect、@+0x344 鼠标深度）、`CameraModeStates`
> （模式→{body,fov对}动态分发）、模式间迁移（match_free_camera_to_normal/init_walk_camera/load_parameters/
> store_parameters/zoom/fov 访问器）与会话存档 CameraRigParametersCache 的 json 序列化。

函数总计 44：业务 21 / 内联无独立符号 13 / 派生胶水 10

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `CameraModeStates::current_dyn` | 0x1409149d0 | 149 | 模式@+0x3d0 jumptable → {body 指针, 表, (fov_current,fov_target) 对}：Normal/TiltShift/Free/Walk/TopDown/Butterfly 各一 case + 模式 6（body=+0x0 矩阵块区） | O：hex 手解（jumptable 分发 + 6 个 case 偏移对） | camera-time\decomp\decomp-0x1409149d0.txt（hex 手解） |
| `MainCamera::init_walk_camera` | 0x14090e510 | 196 | 地形采样+球投地面求 Walk 出生点，方向归一化后 `WalkCamera::teleport(body@+0x2f0, pos, dir)` | O：实证 | round2\decomp\r2-0x14090e510.txt |
| `MainCamera::match_free_camera_to_normal` | 0x14090e6f0 | 231 | 读 Normal position(+0x138..144)/yaw-pitch(+0x150) → 重建 Free rig@+0x270=Position+YawPitch+Smooth(1,1) → dt=0 跑一遍存 final_transform | O：实证（dolly typeid 查找全录） | camera-time\decomp\decomp-0x14090e6f0.txt |
| `MainCamera::new` | 0x14090ec70 | 246 | 构造：六 body 默认装配 + mode@+0x4d0=0 + aspect@+0x4d8=参数 | O：实证 | camera-time\decomp\decomp-0x14090ec70.txt |
| `MainCamera::mode` | 0x140911230 | 432 | 读模式字节 `[self+0x4d0]`（1 字节 getter） | O：hex 手解 `movzx eax,[rcx+0x4d0]; ret` | camera-time\decomp\decomp-0x140911230.txt（hex 手解） |
| `MainCamera::focus`（focus_mut 同址） | 0x140911240 | 436 | 模式→body 内 focus 字段指针：`self+0x100+表[0x142aabb68][mode]`（表 13 项） | O：hex 手解 | camera-time\decomp\decomp-0x140911240.txt（hex 手解） |
| `MainCamera::lens`（lens_mut 同址） | 0x140911260 | 444 | 模式→body 内 lens(fov) 字段指针：`self+0x108+表[0x142aabb98][mode]`（表 13 项，槽 6..9 落 Normal body） | O：hex 手解 | camera-time\decomp\decomp-0x140911260.txt（hex 手解） |
| `MainCamera::zoom` | 0x140911280 | 519 | 缩放：amount/anchor 有限性检查→模式 jumptable→Normal/TiltShift/TopDown：`factor=exp2(clamp(amount·−0.08,−1,1))`，active_rotation 判别(+0x18==1) 双分支（target_arm_length@+0x20 或 offset×factor 写 pending），上界 radial_max@+0x64 | O：582B 全函数手解（pdata 边界 0x911280–0x9114c6，见 round2 CLOSURE） | camera-time\decomp\decomp-0x140911280.txt（hex 手解） |
| `MainCamera::target_vertical_fov_degrees` | 0x1409114d0 | 574 | 读目标 fov：`[self+0x108+lens表[mode]]` | O：hex 手解 | camera-time\decomp\decomp-0x1409114d0.txt（hex 手解） |
| `MainCamera::immediately_set_vertical_fov_degrees` | 0x1409114f0 | 579 | 写 fov：`self+0x100+lens表[mode]` 尾调 body setter | O：hex 手解 | camera-time\decomp\decomp-0x1409114f0.txt（hex 手解） |
| `MainCamera::load_parameters` | 0x140911520 | 623 | 从会话 CameraRigParametersCache 记录恢复：mode=0（Normal）、轨道四元数（π/180）、+0x118=0、push 28B ActiveRotation 入 rotations Vec@+0x100、flag 时 +0x194=记录分量（v1 名 chase_butterfly 撤销） | O：实证 | camera-time\decomp\decomp-0x140911520.txt |
| `MainCamera::store_parameters` | 0x140911710 | 653 | 捕获 {yaw@+0x154,pitch@+0x150,position@+0x138,fov@+0x190} 成 0x24B 记录；NaN→默认并四条日志 "…invalid. Overriding param cache value." | O：实证 | camera-time\decomp\decomp-0x140911710.txt |
| `MainCamera::immediately_finish_interpolating` | 0x140911b50 | 700 | 巨函数（106KB，相邻函数粘连）：模式@+0x4d0+快照字节@+0x134 双 jumptable；调 store/load_parameters、MainCamera::new；fov 系数@+0xec×0.1；多 body 矩阵块 0x100 拷贝；winter==4 分支 | O（面）/U（逐分支） | camera-time\decomp\decomp-0x140911b50.txt |
| `MainCamera::eye_position` | 0x140911dd0 | 704 | 转发 utils::camera 工具（FUN_140c96d40 位置提取） | O：12B 转发器 | round2\decomp\r2-0x140911dd0.txt |
| `MainCamera::eye_forward`（eye_direction 同址） | 0x140911df0 | 712 | 转发 utils::camera 工具（FUN_140c96d80 前向提取） | O：12B 转发器 | camera-time\decomp\decomp-0x140911df0.txt |
| `MainCamera::xz_projective_rotation` | 0x140911e10 | 724 | 转发 utils::camera 工具（FUN_140c96e70 XZ 投影旋转） | O：12B 转发器 | round2\decomp\r2-0x140911e10.txt |
| `_::impl$0::deserialize::visit_str<serde_json>` | 0x1403b5ba0 | 730 | 会话相机记录 json 反序列化变体名匹配 | I（serde 胶水面） |  |
| `_::impl$0::serialize<serde_json Compact>` | 0x1403b4ca0 | 730 | 会话相机记录序列化（写 '{' + 字段 yaw/pitch/position/camera_rig_arm_length/pitch_offset/yaw_offset/position_cache） | O：实证（第一轮曾误记"反序列化"，已纠正） | camera-time\decomp\decomp-0x1403b4ca0.txt |
| `_::impl$0::serialize<serde_json Pretty>` | 0x1403b5000 | 730 | 同记录 Pretty 序列化 | I（同族） |  |
| `_::impl$0::serialize<serde_json::value::Serializer>` | 0x1403b4e10 | 730 | 同记录 Value 序列化 | I（同族） |  |
| `serde_default_vertical_fov_degrees` | 0x140911e30 | 754 | 返回默认 fov 常量（`movss xmm0,[const]; ret`） | O：hex 手解 | camera-time\decomp\decomp-0x140911e30.txt（hex 手解） |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `CameraState::new`（行 105）— 构造：相机运行时状态默认（并入 MainCamera::new）
- `CameraModeStates::current_dyn_mut`（行 155）— 查询/计算：current_dyn 可变版
- `CameraModeStates::current_body_mut`（行 167）— 查询/计算：按模式取 body 可变引用
- `MainCamera::reset`（行 428）— 重置到默认（v1 名曾误挂 0x140911280=zoom，真身内联；语义=重建默认装配，系统层 reset_camera 调用）
- `MainCamera::lock_rotation_pivot`（行 452）— 锁定旋转锚点（CameraRotationPivot 状态切换）
- `MainCamera::unlock_rotation_pivot`（行 471）— 解锁旋转锚点
- `MainCamera::rotate_yaw_pitch`（行 482）— 按模式转发到当前 body 的 rotate_yaw_pitch
- `MainCamera::constrain_to_be_above_terrain`（行 498）— 按模式转发 body 约束（地形/碰撞）
- `MainCamera::translate_screen_space`（行 510）— 按模式转发 body 平移（屏幕空间）
- `MainCamera::chase_butterfly`（行 589）— 追踪蝴蝶（内联于 immediately_finish_interpolating/main_camera_update；v1 名误挂 0x140911520=load_parameters）
- `MainCamera::be_normal`（行 598）— 切回 Normal 模式
- `MainCamera::update`（行 603）— 按模式调用 body update + 矩阵计算（并入 main_camera_update/paused 路径）
- `MainCamera::eye_direction`（行 708）— 与 eye_forward 同址转发

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_map`、`visit_seq` 等 10 项
