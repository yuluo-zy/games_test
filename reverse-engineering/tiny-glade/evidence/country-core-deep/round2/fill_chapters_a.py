# -*- coding: utf-8 -*-
"""Round-3 chapter filler for camera-time domain (writes evidence chapters only)."""
import os

CH = r'D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters'
D = 'camera-time\\decomp'
R2 = 'round2\\decomp'

def w(rel, content):
    p = os.path.join(CH, rel)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(content)

# ---------------- systems/main_camera.rs.md ----------------
w('systems/main_camera.rs.md', (
"# systems/main_camera.rs\n\n"
"> 文件作用：**主相机系统插件**——`MainCamera`（0x4e0B 资源）的每帧驱动：快照按模式分发\n"
"> （camera_snapshot）、会话开始/暂停两套更新（main_camera_update / paused_main_camera_update）、\n"
"> 相机重置（reset_camera）、`CameraRotationPivot` 7 变体枚举（旋转锚点：光标深度/中心深度/光标地形/\n"
"> 中心地形，各分滤波/不滤波）及其 ron 序列化与 enum-iterator 前驱循环。矩阵块 0x100B 写入 MainCamera@+0x0。\n\n"
"函数总计 22：业务 8 / 内联无独立符号 5 / 派生胶水 9\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `camera_snapshot` | 0x140911e40 | 113 | 按模式分发快照更新：读快照 mode u32@+0x18、f32@+0x1c，调 `body_fn(snapshot+0x1c, dt·fov系数@+0xec, mode)`（jumptable） | O：Mut 快照 tick 回写 + 逐模式 jumptable 实证 | %s\\decomp-0x140911e40.txt |\n"
"| `_::impl$0::previous` | 0x140914ac0 | 217 | enum_iterator::Sequence::previous：表 [6,0,1,2,3,4,0,0]（前驱回绕，CameraRotationPivot 7 变体：previous(0)=6） | O：shr 表查找 8 字节函数 | %s\\decomp-0x140914ac0.txt |\n"
"| `_::impl$0::serialize<ron::Serializer>` | 0x14003ac90 | 218 | CameraRotationPivot 枚举 ron 序列化（7 变体名实证：CameraRotationPivot/CursorDepthFiltered/CursorDepth/CenterDepthFiltered/CenterDepth/CursorTerrain/CenterTerrain） | O：变体名串 0x14292bb7c 族 | %s\\decomp-0x14003ac90.txt |\n"
"| `_::impl$0::deserialize::visit_str` | 0x14003b6a0 | 219 | ron 反序列化变体名匹配 | I（derive 反序列化胶水面，未反编译） |  |\n"
"| `impl$3::build` | 0x1409125e0 | 266 | MainCameraPlugin::build：注册 2 系统（hash 0x142c96f70/0x142c5fd60）+ 2 资源（typeid 0x80c70da6101ec1a/0x668ae4faf3605349）+ 事件（0x142c6bd60） | O：注册调用序列实证；run_if 条件为 hash 编码（I） | %s\\decomp-0x1409125e0.txt |\n"
"| `reset_camera` | 0x140912a40 | 291 | MainCamera::new(旧 aspect@+0x4d8) → memcpy 0x4e0 → 清 CameraRigParametersCache Vec（0x24 元素）→ 推 WriteSettingsFileCmd；日志 \"Resetting the camera.\" | O：全流程机器码实证 | %s\\decomp-0x140912a40.txt |\n"
"| `main_camera_update` | 0x140912c20 | 310 | 每帧驱动：aspect=w/h@+0x4d8；新会话标志门（光标未动跳过）；窗口<3px 推 WriteSettingsFileCmd；鼠标深度字节→+0x344；DebugGui 模式滑块（6→0）/自由相机开关→双 jumptable 分发 | O：实证 | %s\\decomp-0x140912c20.txt |\n"
"| `paused_main_camera_update` | 0x1409145a0 | 640 | 暂停时：aspect=w/h；mode@+0x4d0→6 body（+0x100/1b8/270/2f0/390/448）；body 更新 fn 指针；calc_matrices→memcpy 0x100→MainCamera@+0x0 | O：实证（case 6..11 反编译损坏，7..10 的 fov 对见 DIGEST） | %s\\decomp-0x1409145a0.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `CameraSnapshotInterpolationState::is_interpolating`（行 48）— 谓词/校验：插值进行中判断（并入 camera_snapshot/apply_camera_snapshot_op）\n"
"- `CameraSnapshotInterpolationState::advance`（行 52）— 插值进度推进（dt·fov 系数，并入 camera_snapshot 的 jumptable 调用）\n"
"- `CameraSnapshotInterpolationState::interpolation_dt`（行 87）— 插值 dt 计算\n"
"- `apply_camera_snapshot_op`（行 157）— 应用快照操作（v1 名误挂 0x1408b99e0，实为 enum_iterator Sequence::next；真身内联于 camera_snapshot 路径）\n"
"- `CameraRotationPivot::use_camera_cursor_grab_icon`（行 234）— 谓词：该 pivot 变体是否显示光标抓取图标（ui_camera_mode_gizmos 的 cursor_icon 消费）\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`default`、`deserialize`、`eq`、`fmt`、`next`、`visit_enum` 等 9 项\n"
) % ((D,)*7))

# ---------------- systems/camera_rig.rs.md ----------------
w('systems/camera_rig.rs.md', (
"# systems/camera_rig.rs\n\n"
"> 文件作用：**相机装配中心**——`MainCamera`（0x4e0B：@+0x0 矩阵块 0x100B、@+0x100/1b8/270/2f0/390/448\n"
"> 六个 body、@+0x4d0 模式字节 0..12 共 13 槽、@+0x4d8 aspect、@+0x344 鼠标深度）、`CameraModeStates`\n"
"> （模式→{body,fov对}动态分发）、模式间迁移（match_free_camera_to_normal/init_walk_camera/load_parameters/\n"
"> store_parameters/zoom/fov 访问器）与会话存档 CameraRigParametersCache 的 json 序列化。\n\n"
"函数总计 44：业务 21 / 内联无独立符号 13 / 派生胶水 10\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `CameraModeStates::current_dyn` | 0x1409149d0 | 149 | 模式@+0x3d0 jumptable → {body 指针, 表, (fov_current,fov_target) 对}：Normal/TiltShift/Free/Walk/TopDown/Butterfly 各一 case + 模式 6（body=+0x0 矩阵块区） | O：hex 手解（jumptable 分发 + 6 个 case 偏移对） | %s\\decomp-0x1409149d0.txt（hex 手解） |\n"
"| `MainCamera::init_walk_camera` | 0x14090e510 | 196 | 地形采样+球投地面求 Walk 出生点，方向归一化后 `WalkCamera::teleport(body@+0x2f0, pos, dir)` | O：实证 | %s\\r2-0x14090e510.txt |\n"
"| `MainCamera::match_free_camera_to_normal` | 0x14090e6f0 | 231 | 读 Normal position(+0x138..144)/yaw-pitch(+0x150) → 重建 Free rig@+0x270=Position+YawPitch+Smooth(1,1) → dt=0 跑一遍存 final_transform | O：实证（dolly typeid 查找全录） | %s\\decomp-0x14090e6f0.txt |\n"
"| `MainCamera::new` | 0x14090ec70 | 246 | 构造：六 body 默认装配 + mode@+0x4d0=0 + aspect@+0x4d8=参数 | O：实证 | %s\\decomp-0x14090ec70.txt |\n"
"| `MainCamera::mode` | 0x140911230 | 432 | 读模式字节 `[self+0x4d0]`（1 字节 getter） | O：hex 手解 `movzx eax,[rcx+0x4d0]; ret` | %s\\decomp-0x140911230.txt（hex 手解） |\n"
"| `MainCamera::focus`（focus_mut 同址） | 0x140911240 | 436 | 模式→body 内 focus 字段指针：`self+0x100+表[0x142aabb68][mode]`（表 13 项） | O：hex 手解 | %s\\decomp-0x140911240.txt（hex 手解） |\n"
"| `MainCamera::lens`（lens_mut 同址） | 0x140911260 | 444 | 模式→body 内 lens(fov) 字段指针：`self+0x108+表[0x142aabb98][mode]`（表 13 项，槽 6..9 落 Normal body） | O：hex 手解 | %s\\decomp-0x140911260.txt（hex 手解） |\n"
"| `MainCamera::zoom` | 0x140911280 | 519 | 缩放：amount/anchor 有限性检查→模式 jumptable→Normal/TiltShift/TopDown：`factor=exp2(clamp(amount·−0.08,−1,1))`，active_rotation 判别(+0x18==1) 双分支（target_arm_length@+0x20 或 offset×factor 写 pending），上界 radial_max@+0x64 | O：582B 全函数手解（pdata 边界 0x911280–0x9114c6，见 round2 CLOSURE） | %s\\decomp-0x140911280.txt（hex 手解） |\n"
"| `MainCamera::target_vertical_fov_degrees` | 0x1409114d0 | 574 | 读目标 fov：`[self+0x108+lens表[mode]]` | O：hex 手解 | %s\\decomp-0x1409114d0.txt（hex 手解） |\n"
"| `MainCamera::immediately_set_vertical_fov_degrees` | 0x1409114f0 | 579 | 写 fov：`self+0x100+lens表[mode]` 尾调 body setter | O：hex 手解 | %s\\decomp-0x1409114f0.txt（hex 手解） |\n"
"| `MainCamera::load_parameters` | 0x140911520 | 623 | 从会话 CameraRigParametersCache 记录恢复：mode=0（Normal）、轨道四元数（π/180）、+0x118=0、push 28B ActiveRotation 入 rotations Vec@+0x100、flag 时 +0x194=记录分量（v1 名 chase_butterfly 撤销） | O：实证 | %s\\decomp-0x140911520.txt |\n"
"| `MainCamera::store_parameters` | 0x140911710 | 653 | 捕获 {yaw@+0x154,pitch@+0x150,position@+0x138,fov@+0x190} 成 0x24B 记录；NaN→默认并四条日志 \"…invalid. Overriding param cache value.\" | O：实证 | %s\\decomp-0x140911710.txt |\n"
"| `MainCamera::immediately_finish_interpolating` | 0x140911b50 | 700 | 巨函数（106KB，相邻函数粘连）：模式@+0x4d0+快照字节@+0x134 双 jumptable；调 store/load_parameters、MainCamera::new；fov 系数@+0xec×0.1；多 body 矩阵块 0x100 拷贝；winter==4 分支 | O（面）/U（逐分支） | %s\\decomp-0x140911b50.txt |\n"
"| `MainCamera::eye_position` | 0x140911dd0 | 704 | 转发 utils::camera 工具（FUN_140c96d40 位置提取） | O：12B 转发器 | %s\\r2-0x140911dd0.txt |\n"
"| `MainCamera::eye_forward`（eye_direction 同址） | 0x140911df0 | 712 | 转发 utils::camera 工具（FUN_140c96d80 前向提取） | O：12B 转发器 | %s\\decomp-0x140911df0.txt |\n"
"| `MainCamera::xz_projective_rotation` | 0x140911e10 | 724 | 转发 utils::camera 工具（FUN_140c96e70 XZ 投影旋转） | O：12B 转发器 | %s\\r2-0x140911e10.txt |\n"
"| `_::impl$0::deserialize::visit_str<serde_json>` | 0x1403b5ba0 | 730 | 会话相机记录 json 反序列化变体名匹配 | I（serde 胶水面） |  |\n"
"| `_::impl$0::serialize<serde_json Compact>` | 0x1403b4ca0 | 730 | 会话相机记录序列化（写 '{' + 字段 yaw/pitch/position/camera_rig_arm_length/pitch_offset/yaw_offset/position_cache） | O：实证（第一轮曾误记\"反序列化\"，已纠正） | %s\\decomp-0x1403b4ca0.txt |\n"
"| `_::impl$0::serialize<serde_json Pretty>` | 0x1403b5000 | 730 | 同记录 Pretty 序列化 | I（同族） |  |\n"
"| `_::impl$0::serialize<serde_json::value::Serializer>` | 0x1403b4e10 | 730 | 同记录 Value 序列化 | I（同族） |  |\n"
"| `serde_default_vertical_fov_degrees` | 0x140911e30 | 754 | 返回默认 fov 常量（`movss xmm0,[const]; ret`） | O：hex 手解 | %s\\decomp-0x140911e30.txt（hex 手解） |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `CameraState::new`（行 105）— 构造：相机运行时状态默认（并入 MainCamera::new）\n"
"- `CameraModeStates::current_dyn_mut`（行 155）— 查询/计算：current_dyn 可变版\n"
"- `CameraModeStates::current_body_mut`（行 167）— 查询/计算：按模式取 body 可变引用\n"
"- `MainCamera::reset`（行 428）— 重置到默认（v1 名曾误挂 0x140911280=zoom，真身内联；语义=重建默认装配，系统层 reset_camera 调用）\n"
"- `MainCamera::lock_rotation_pivot`（行 452）— 锁定旋转锚点（CameraRotationPivot 状态切换）\n"
"- `MainCamera::unlock_rotation_pivot`（行 471）— 解锁旋转锚点\n"
"- `MainCamera::rotate_yaw_pitch`（行 482）— 按模式转发到当前 body 的 rotate_yaw_pitch\n"
"- `MainCamera::constrain_to_be_above_terrain`（行 498）— 按模式转发 body 约束（地形/碰撞）\n"
"- `MainCamera::translate_screen_space`（行 510）— 按模式转发 body 平移（屏幕空间）\n"
"- `MainCamera::chase_butterfly`（行 589）— 追踪蝴蝶（内联于 immediately_finish_interpolating/main_camera_update；v1 名误挂 0x140911520=load_parameters）\n"
"- `MainCamera::be_normal`（行 598）— 切回 Normal 模式\n"
"- `MainCamera::update`（行 603）— 按模式调用 body update + 矩阵计算（并入 main_camera_update/paused 路径）\n"
"- `MainCamera::eye_direction`（行 708）— 与 eye_forward 同址转发\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`clone`、`deserialize`、`eq`、`expecting`、`fmt`、`serialize`、`visit_map`、`visit_seq` 等 10 项\n"
) % ((D, R2, D, D, D, D, D, D, D, D, D, D, D, R2, D, R2, D, D)))

# ---------------- systems/camera_rig/camera_body.rs.md ----------------
w('systems/camera_rig/camera_body.rs.md', (
"# systems/camera_rig/camera_body.rs\n\n"
"> 文件作用：**相机 body 共享类型与 trait**——`CameraBody` 接口（rotate_yaw_pitch/update）、\n"
"> `CameraTransform`（游戏 dolly 分叉内为 rotation-first：rotation@+0x18 16B、position@+0x28，\n"
"> 0x140994700 实读四元数 + 0x140994310 拷贝 16B 坐实）、`CameraMode` 核心 6 变体（模式字节另有\n"
"> 6..12 拍照/工具槽）、lerp/smooth(1−exp(−8dt))/bounds_xz 共享数学（机器码实证）。\n\n"
"函数总计 1：业务 0 / 内联无独立符号 1 / 派生胶水 0\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `camera_body::pitch_yaw_rot`（行 24）— 由 pitch/yaw（度）构造旋转四元数（YXZ 欧拉 × π/180；被 NormalCamera::rotation 与 load_parameters 使用，常量 0.0174533@142aab4b0 实证）\n"
))

# ---------------- systems/camera_rig/free.rs.md ----------------
w('systems/camera_rig/free.rs.md', (
"# systems/camera_rig/free.rs\n\n"
"> 文件作用：**FreeCamera**——dolly rig（Position+YawPitch+Smooth(1,1)）封装：driver 按 typeid\n"
"> 查找（Position=0x6b8463577afc1a66/YawPitch=0x718e4fd258330fe5）、屏幕空间平移（±91 clamp、\n"
"> 地形+0.75 贴地、20/60 步长、smooth(dt)）、yaw 绕 720° fmod + pitch 钳 [-90,90]。\n\n"
"函数总计 9：业务 9 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `impl$1::translate_screen_space` | 0x1409938b0 | 0 | 屏幕空间平移：quat 旋转移动→clamp xz ±91→terrain.max(0)+0.75 贴地→smooth(dt)=1−exp(−8dt)→delta×20/60→Position.translate | O：全常量实证（±91@142aba9b4/8、0.75、20/60） | %s\\decomp-0x1409938b0.txt |\n"
"| `FreeCamera::get_position` | 0x140992f10 | 18 | typeid 查 Position driver → sret Vec3 | O：实证 | %s\\decomp-0x140992f10.txt |\n"
"| `FreeCamera::get_yaw_pitch` | 0x140993010 | 22 | typeid 查 YawPitch driver → (yaw,pitch) 首 8B | O：实证 | %s\\decomp-0x140993010.txt |\n"
"| `FreeCamera::set` | 0x140993100 | 27 | 重建 rig：alloc Position(12B)+YawPitch(8B)+Smooth(0x40B, {0,0,0,0,fnptr,1.0f}) 三 driver → dt=0 跑一遍存 final_transform | O：实证（与 match_free_camera_to_normal 同构建器） | %s\\decomp-0x140993100.txt |\n"
"| `impl$1::rotate_yaw_pitch`（free/walk 共享） | 0x140993750 | 44 | YawPitch driver_mut(vtable+0x40)：`yaw=fmodf(yaw·0.5+old, 720)`、`pitch=clamp(old+pitch·0.5, −90, 90)` | O：实证（720@142aba9ac、±90@142aba9b0/1429edb94） | %s\\decomp-0x140993750.txt |\n"
"| `impl$1::constrain_to_be_above_terrain` | 0x14003bb80 | 58 | 空操作（折叠 trampoline，仅 `ret`） | O：实证 | %s\\decomp-0x14003bb80.txt |\n"
"| `impl$1::update`（free/walk 共享） | 0x140993d30 | 103 | dolly rig 迭代：逐 driver vtable+0x30(driver, parent_transform, dt)，写回 rig+0x18..0x34，sret 28B {pos,rot} | O：实证 | %s\\decomp-0x140993d30.txt |\n"
"| `walk::impl$2::rotate_yaw_pitch` | 0x140993750 | 44 | 同 free（共享函数） | O：同上 | %s\\decomp-0x140993750.txt |\n"
"| `walk::impl$2::update` | 0x140993d30 | 103 | 同 free（共享函数） | O：同上 | %s\\decomp-0x140993d30.txt |\n"
) % ((D,)*9))

# ---------------- systems/camera_rig/normal.rs.md ----------------
w('systems/camera_rig/normal.rs.md', (
"# systems/camera_rig/normal.rs\n\n"
"> 文件作用：**NormalCamera（轨道相机）**——Normal/TiltShift/TopDown 三种模式共用的轨道 body：\n"
"> 布局 {rotations Vec@+0x0(28B 元素)、active_rotation@+0x18(判别 1=Some, 数据@+0x1c)、\n"
"> position@+0x38、pending_translation@+0x44、pitch@+0x50、yaw@+0x54、pitch_min@+0x58、\n"
"> pitch_max@+0x5c、radial_min@+0x60、radial_max@+0x64、terrain_clearance@+0x68、loose@+0x6c}；\n"
"> 平滑轨道 + 径向钳 + 松散地面投影（每模式常量在 reconstruction/camera 已机器码校验）。\n\n"
"函数总计 11：业务 4 / 内联无独立符号 5 / 派生胶水 2\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `impl$1::update` | 0x140949fe0 | 0 | 每帧：smooth 推进、radial_max@+0x60 钳、loose@+0x6c 地面投影（lookahead 90·0.2 或 33.45/46.0 边界）、ActiveRotation 弧长插值、pending 应用、pos.y≥1.0、retain | O：结构与常量实证（每模式数值沿用 reconstruction/camera 已验证） | %s\\decomp-0x140949fe0.txt |\n"
"| `impl$1::rotate_yaw_pitch` | 0x140949cc0 | 45 | active_rotation 存在时：base=sum(rotations.pitch_delta)+pitch@+0x50；bounded=clamp(−(pitch+base), pitch_min@+0x58, pitch_max@+0x5c)；active.pitch_delta += −bounded−base；yaw_delta += yaw | O：实证（+0x24/+0x28 写回） | %s\\decomp-0x140949cc0.txt |\n"
"| `impl$1::constrain_to_be_above_terrain` | 0x140949dc0 | 74 | pending.y = max(pending.y, terrain_height(pos.xz+pending.xz)+clearance@+0x68 − pos.y)；x/z 恒等折叠 | O：实证 | %s\\decomp-0x140949dc0.txt |\n"
"| `impl$1::translate_screen_space` | 0x140949e70 | 94 | ratio=max(pos.y/−fwd.y,1)/50；pending += 绕 Y(yaw) 旋转的 movement × ratio | O：实证（50@1429cd070） | %s\\decomp-0x140949e70.txt |\n\n"
"## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）\n\n"
"- `NormalCamera::rotation`（行 39）— pitch/yaw（度）→ 四元数（YXZ × π/180，共用 pitch_yaw_rot）\n"
"- `ActiveRotation::is_finished`（行 202）— 谓词/校验：|pitch_delta|<0.001 && |yaw_delta|<0.001（retain 条件）\n"
"- `constrain_position`（行 211）— 位置约束分发（按 loose 选 normal/semi_ortho，并入 update）\n"
"- `constrain_position_normal`（行 224）— Normal 模式位置约束：lookahead=forward·18、anchor 边界 46.0、radial_min 投影（并入 update 的 else 分支）\n"
"- `constrain_position_semi_ortho`（行 243）— TiltShift/TopDown 半正交约束：distance=((pos.y−10)/max(−fwd.y,0.15)).max(11.0125)、anchor 边界 33.45（并入 update 的 loose 分支）\n\n"
"## 派生胶水（derive/宏生成；不计独立业务）\n\n"
"`closure`、`default` 等 2 项\n"
) % ((D,)*4))

# ---------------- systems/camera_rig/walk.rs.md ----------------
w('systems/camera_rig/walk.rs.md', (
"# systems/camera_rig/walk.rs\n\n"
"> 文件作用：**WalkCamera（行走相机）**——dolly rig（Position+YawPitch，无 Smooth）+ 碰撞滑行：\n"
"> 布局 {rig@+0x0、ground_hit: Option<CastHit>@+0x38（判别 0xb 命中/0x11 空）、\n"
"> last_translation_dt@+0x50、allow_vertical@+0x54}；8 步 spherecast 滑行（filter 0x30cf、半径 0.5）、\n"
"> 地面球投（filter 0x20、半径 0.45）+ 地形 blend + 指数垂直平滑；XZ 钳 ±34。\n"
"> 分叉 CameraTransform rotation-first（rotation@+0x18 = 四元数，`rotation()` 拷贝 16B）。\n\n"
"函数总计 9：业务 9 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `impl$2::translate_screen_space` | 0x140994700 | 0 | 8 步 spherecast 滑行：`q@+0x18` 旋转移动→flat（allow_vertical 时保留 y）→step=8/60 归一→8 次 cast（0.5 半径/0x30cf/法向滑移 t·0.9）→±34 钳→smooth→Position.translate | O：实证（quat@+0x18..0x24 实读；1e-10 阈值） | %s\\decomp-0x140994700.txt |\n"
"| `WalkCamera::new` | 0x140993e20 | 35 | 构建 rig=Position(0,1,0)+YawPitch(0,0)（无 Smooth）+ 附加状态零化 | O：实证（(0,1,0)@1429345b0） | %s\\decomp-0x140993e20.txt |\n"
"| `WalkCamera::teleport` | 0x140994130 | 49 | Position driver 写 (x,y,z)；方向向量→yaw/pitch 度（FUN_140334d80+FUN_141766a90 × rad2deg 常量） | O：实证（骨架 quat 基构造为近似，实为欧拉转换） | %s\\decomp-0x140994130.txt |\n"
"| `WalkCamera::rotation` | 0x140994310 | 58 | 返回四元数：拷贝 `[self+0x18]` 16B（分叉 CameraTransform rotation-first） | O：CLOSED（round2；xrefs=0 纯 vtable 调用） | %s\\decomp-0x140994310.txt（hex 手解） |\n"
"| `impl$2::rotate_yaw_pitch` | 0x140993750 | 66 | 同 free（共享）：yaw fmod 720、pitch 钳 ±90 | O：共享函数 | %s\\decomp-0x140993750.txt |\n"
"| `impl$2::constrain_to_be_above_terrain` | 0x140994320 | 74 | 两段 spherecast（filter 0x20 先、0x30cf/0 后，半径 0.45，XZ 钳 ±64.9）→地面下界 y−1.9−1.4→terrain walk_height_blend→`y=lerp(y,target,1−exp(−last_dt·20))`；ground_hit kind 0xb | O：实证（常量全录 DIGEST） | %s\\decomp-0x140994320.txt |\n"
"| `impl$2::update` | 0x140993d30 | 288 | dolly rig 迭代（共享） | O | %s\\decomp-0x140993d30.txt |\n"
"| `free::impl$1::rotate_yaw_pitch` | 0x140993750 | 66 | 同共享函数 | O | %s\\decomp-0x140993750.txt |\n"
"| `free::impl$1::update` | 0x140993d30 | 288 | 同共享函数 | O | %s\\decomp-0x140993d30.txt |\n"
) % ((D,)*9))

# ---------------- systems/camera_rig/butterfly.rs.md ----------------
w('systems/camera_rig/butterfly.rs.md', (
"# systems/camera_rig/butterfly.rs\n\n"
"> 文件作用：**ButterflyCamera（蝴蝶追踪相机）**——dolly rig（Position+Rotation+Smooth(2,predictive)+\n"
"> Arm(0,1.5,3.5)+Smooth(4)+LookAt(tracking 4)，布局 {rig@+0x0、selected_index@+0x38、\n"
"> applied_index@+0x3c}；选中索引变化时 dt=1000 瞬时吸附；rotate/constrain/translate 三操作\n"
"> 折叠到空 trampoline（0x14003bb80 = 仅 `ret`）。相机书签缩放换挡（advance 按 zoom 符号）。\n\n"
"函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0\n\n"
"## 业务函数\n\n"
"| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |\n"
"|---|---|---|---|---|---|\n"
"| `impl$0::constrain_to_be_above_terrain` | 0x14003bb80 | 20 | 空操作（蝴蝶 body 不约束地形） | O：`ret` trampoline | %s\\decomp-0x14003bb80.txt |\n"
"| `impl$0::rotate_yaw_pitch` | 0x14003bb80 | 20 | 空操作（同上） | O：`ret` trampoline | %s\\decomp-0x14003bb80.txt |\n"
"| `impl$0::translate_screen_space` | 0x14003bb80 | 20 | 空操作（同上） | O：`ret` trampoline | %s\\decomp-0x14003bb80.txt |\n"
"| `impl$0::update` | 0x140a0d090 | 37 | `selected(+0x3c) != applied(+0x38)` → applied=selected、dt=1000（@14295b630）→ rig 迭代 vtable+0x30 → sret 28B | O：实证 | %s\\decomp-0x140a0d090.txt |\n"
) % ((D,)*4))

print('wrote systems part A')
