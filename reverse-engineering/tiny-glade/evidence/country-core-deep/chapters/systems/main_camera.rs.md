# systems/main_camera.rs

> 文件作用：**主相机系统插件**——`MainCamera`（0x4e0B 资源）的每帧驱动：快照按模式分发
> （camera_snapshot）、会话开始/暂停两套更新（main_camera_update / paused_main_camera_update）、
> 相机重置（reset_camera）、`CameraRotationPivot` 7 变体枚举（旋转锚点：光标深度/中心深度/光标地形/
> 中心地形，各分滤波/不滤波）及其 ron 序列化与 enum-iterator 前驱循环。矩阵块 0x100B 写入 MainCamera@+0x0。

函数总计 22：业务 8 / 内联无独立符号 5 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `camera_snapshot` | 0x140911e40 | 113 | 按模式分发快照更新：读快照 mode u32@+0x18、f32@+0x1c，调 `body_fn(snapshot+0x1c, dt·fov系数@+0xec, mode)`（jumptable） | O：Mut 快照 tick 回写 + 逐模式 jumptable 实证 | camera-time\decomp\decomp-0x140911e40.txt |
| `_::impl$0::previous` | 0x140914ac0 | 217 | enum_iterator::Sequence::previous：表 [6,0,1,2,3,4,0,0]（前驱回绕，CameraRotationPivot 7 变体：previous(0)=6） | O：shr 表查找 8 字节函数 | camera-time\decomp\decomp-0x140914ac0.txt |
| `_::impl$0::serialize<ron::Serializer>` | 0x14003ac90 | 218 | CameraRotationPivot 枚举 ron 序列化（7 变体名实证：CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain） | O：变体名串 0x14292bb7c 族 | camera-time\decomp\decomp-0x14003ac90.txt |
| `_::impl$0::deserialize::visit_str` | 0x14003b6a0 | 219 | ron 反序列化变体名匹配 | I（derive 反序列化胶水面，未反编译） |  |
| `impl$3::build` | 0x1409125e0 | 266 | MainCameraPlugin::build：注册 2 系统（hash 0x142c96f70/0x142c5fd60）+ 2 资源（typeid 0x80c70da6101ec1a/0x668ae4faf3605349）+ 事件（0x142c6bd60） | O：注册调用序列实证；run_if 条件为 hash 编码（I） | camera-time\decomp\decomp-0x1409125e0.txt |
| `reset_camera` | 0x140912a40 | 291 | MainCamera::new(旧 aspect@+0x4d8) → memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24 元素）→ 推 WriteSettingsFileCmd；日志 "Resetting the camera." | O：全流程机器码实证 | camera-time\decomp\decomp-0x140912a40.txt |
| `main_camera_update` | 0x140912c20 | 310 | 每帧驱动：aspect=w/h@+0x4d8；新会话标志门（光标未动跳过）；窗口<3px 推 WriteSettingsFileCmd；鼠标深度字节→+0x344；DebugGui 模式滑块（6→0）/自由相机开关→双 jumptable 分发 | O：实证 | camera-time\decomp\decomp-0x140912c20.txt |
| `paused_main_camera_update` | 0x1409145a0 | 640 | 暂停时：aspect=w/h；mode@+0x4d0→6 body（+0x100/1b8/270/2f0/390/448）；body 更新 fn 指针；calc_matrices→memcpy 0x100→MainCamera@+0x0 | O：实证（case 6..11 反编译损坏，7..10 的 fov 对见 DIGEST） | camera-time\decomp\decomp-0x1409145a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `CameraSnapshotInterpolationState::is_interpolating`（行 48）— 谓词/校验：插值进行中判断（并入 camera_snapshot/apply_camera_snapshot_op）
- `CameraSnapshotInterpolationState::advance`（行 52）— 插值进度推进（dt·fov 系数，并入 camera_snapshot 的 jumptable 调用）
- `CameraSnapshotInterpolationState::interpolation_dt`（行 87）— 插值 dt 计算
- `apply_camera_snapshot_op`（行 157）— 应用快照操作（v1 名误挂 0x1408b99e0，实为 enum_iterator Sequence::next；真身内联于 camera_snapshot 路径）
- `CameraRotationPivot::use_camera_cursor_grab_icon`（行 234）— 谓词：该 pivot 变体是否显示光标抓取图标（ui_camera_mode_gizmos 的 cursor_icon 消费）

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`deserialize`、`eq`、`fmt`、`next`、`visit_enum` 等 9 项

## 实现方式（算法级）

```
camera_snapshot（0x140911e40）：Mut tick 回写；mode=snapshot.mode u32@+0x18；
  jumptable(0x142aabc88)[mode](snapshot.f32@+0x1c, dt·camera.fov_coeff@+0xec, mode)
  // 每模式 body 快照应用函数；dt×+0xec = 插值步长
main_camera_update（0x140912c20）：
  aspect=w/h → camera+0x4d8
  new_session.flag(+0x18)：窗口>2px 且 cursor==(0,0) → return（等鼠标首动）；否则清 flag
  窗口<3px → 推 WriteSettingsFileCmd（Vec grow + 8B 事件 + counter@+0x40）
  camera+0x344 = 鼠标深度字节
  debug_gui 模式滑块（6→0）；free_camera 开关(+0x168)≠0 → 自由相机 jumptable(0x142aabd30)
  否则 jumptable(0x142aabd04)[action_states.mode(+0x10)]
paused_main_camera_update（0x1409145a0）：
  aspect=w/h；mode→6 body（+0x100/1b8/270/2f0/390/448）→ body 更新 fn
  → CameraProjectionParams::calc_matrices（FUN_140c8ff70）+FUN_140c97390 → memcpy 0x100 → MainCamera@+0x0
reset_camera（0x140912a40）：DebugGui 层>4 日志 "Resetting the camera." → MainCamera::new(旧 aspect@+0x4d8)
  → memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24B 元素 free，cap 留 4）→ 推 WriteSettingsFileCmd
previous（0x140914ac0）：表 [6,0,1,2,3,4,0,0]（CameraRotationPivot 7 变体前驱循环，previous(0)=6）
CameraRotationPivot ron 变体名：CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain（0x14292bb7c）
```
