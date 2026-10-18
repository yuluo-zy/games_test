# LOGIC — systems/camera_rig（逐函数逻辑判等审计）

> 审计员: logic-verifier-session。日期: 2026-10-17。
> 依据: 2026-10-16-logic-verify-protocol.md；Ghidra shared 只读 fresh 反编译 + read_memory 机器码核对（PDB void() 占位处全部以机器码为准）。

## 函数判等表

| VA | 原函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x140911e30 | default_vertical_fov_degrees / serde_default_vertical_fov_degrees | `mod.rs::default_vertical_fov_degrees` | **判等** | 机器码 `movss xmm0,[0x142934620]`=0x42340000=45.0 |
| 0x140914ac0 | CameraMode::previous | `main_camera.rs::CameraMode::previous` | **判等** | 机器码 `movzx ecx,[rcx]; shl cl,3; movabs rax,0x40302010006; shr rax,cl`=(0x40302010006 >> ((mode&7)*8))&0xff。与 Rust TABLE/shift&63 一致（含 mode>=8 回绕） |
| 0x1409149d0 | CameraModeStates::current_dyn | `mod.rs::current_dyn` | **未实现-需重做** | 原 switch(mode@+0x3d0) 返回 4 元 `{dyn_body_ptr, vtable(anon_..llvm), fov_abs, body_abs}`；mode0..5 **body_abs=+0x70/0x128/0x1a8/0x248/0x300/0x388、fov_abs=+0x8c/0x144/0x1c4/0x264/0x31c/0x3a4**。Rust BODY_OFFSETS=[0x100,0x1b8,0x270,0x2f0,0x390,0x448] **与原 body 偏移不符**，且缺 vtable/dyn_body_ptr 返回值 |
| (inline) | current_dyn_mut / current_body_mut | `current_dyn_mut` / `current_body_offset` | **未实现** | 依赖 current_dyn，随其判非等 |
| 0x140911710 | MainCamera::store_parameters | `main_camera.rs::store_parameters` | **判等** | 位置(+0x138 三 f32)整体非有限→(0,5,0)(0x40a00000) 记日志；yaw(+0x154)/pitch(+0x150)/fov(+0x190) 各自非有限→0；输出序 yaw/pitch/pos/arm=0/fov/off=0。与 Rust finite_or 全符 |
| 0x140911520 | MainCamera::load_parameters | `main_camera.rs::load_parameters` | **未实现-需重做** | **重点**: 原函数 = 模式字节+0x4d0=0(Normal) + **from_euler_angles 算轨道四元数写 +0x144/+0x14c** + **+0x118=0** + **push 28B ActiveRotation 入 rotations Vec@+0x100**（{0, pos.x-ref, pos.y-ref, 0x80000000, 0x80000000, -1.0}）+ **ignore_snapshot==false 时写 +0x194=params[3]**；**不写 Normal body 位置(+0x138)/yaw(+0x154)/pitch(+0x150)**。Rust 字段直写位置/yaw/pitch+fov=45，无四元数/ActiveRotation/+0x118/+0x194——与原写入偏移完全不同，自造近似 |
| 0x1409114d0 | target_vertical_fov_degrees | `main_camera.rs::target_vertical_fov_degrees` | **判等** | 机器码 `movzx eax,[rcx+0x4d0]; lea rdx,[0x142aabb98]; mov rax,[rdx+rax*8]; movss xmm0,[rcx+rax+0x108]` = FOV_OFFSETS[mode]+0x108 |
| 0x1409114f0 | immediately_set_vertical_fov_degrees | `main_camera.rs::immediately_set_vertical_fov_degrees` | **判等** | 机器码 table[mode]+0x100 基址后写 +8/+4 = fov（0x140c974f0 为另一路径；本址直写两槽） |
| 0x140911dd0 | eye_position | `main_camera.rs::eye_position` | **判等(限)** | 12B 转发器→CameraMatrices::eye_position(0x140c96d40=inverse_view 平移列)。Rust 直读 Normal body +0x138（矩阵缓存由 main_camera_update 构建，未实现；Normal 模式下 body 位置==eye，近似成立） |
| 0x140911df0 | eye_forward | `main_camera.rs::eye_forward` | **判等(限)** | 转发器→CameraMatrices::eye_direction(0x140c96d80=normalize(-forward))。Rust 由 body 构 CameraMatrices 后调用同函数 |
| 0x140911e10 | xz_projective_rotation | `main_camera.rs::xz_projective_rotation` | **判等(限)** | 转发器→CameraMatrices::xz_projective_rotation(0x140c96e70)。Rust 同上 |
| (inline) | MainCamera::reset | `main_camera.rs::reset` | 内联 | 无独立 VA，未逐条反编译；RESTORE 已登记（清缓冲+Normal+fov45+pos(0,5,0)） |

域类型: CameraRigParametersCache/CameraRigParameters(6/7 字段 serde) 属卡0 域类型证据包（to_json_string 0x1416f8470），serde 为胶水。

## 汇总

- 判等: default_fov / CameraMode::previous / store_parameters / target_fov / immediate_set_fov / eye×3(限) = 8
- 未实现-需重做: **current_dyn（body 偏移与 vtable 返回值不符）+ load_parameters（重点：字段直写与原轨道四元数/ActiveRotation 流程不符）+ current_dyn_mut/current_body_offset（随依赖）** = 4
- RESTORE 所述 load_parameters "恢复 Normal body 位置/yaw/pitch 直写" 与原 0x140911520 fresh 反编译**不一致**——原不写这些偏移，需按轨道四元数+ActiveRotation 重做。
