# REBUILD — systems/camera_rig 真实逻辑重建（rebuild-cameraui）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
LOGIC.md: reports/interaction/camera_rig/LOGIC.md（4 个 `未实现-需重做` + eye×3 判等(限)）
证据: ghidra `--worker shared` fresh 反编译（0x1409149d0/0x140911520/0x140911dd0..e10、
`CameraMatrices` 0x140c96d40..e70、`from_euler_angles` 0x140d4b440）+ radare2 机器码
（0x140911520 全段、0x140911656..6df）。

## 分类结论（协议 §2026-10-17 用户补充）

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `current_dyn` | 0x1409149d0 | **真实业务** | 重建 |
| `current_dyn_mut` / `current_body_offset` | 行 155/167 内联 | 随依赖 | 随 current_dyn 重建 |
| `load_parameters` | 0x140911520 | **真实业务** | 重建 |
| `eye_position` / `eye_forward` / `xz_projective_rotation` | 0x140911dd0/df0/e10 | **真实业务（转发器）** | 重建为转发到维护 CameraMatrices |

无宏生成胶水。`CameraMode::previous`/`store_parameters`/`target_vertical_fov_degrees`/
`immediately_set_vertical_fov_degrees`/`reset` 判等函数按红线**不动**。

## 逐函数：分类 → 重建 → 逐语句判等自检

### 1. current_dyn（0x1409149d0）— 真实业务，重建
- fresh：switch(mode@+0x3d0) 六 case，每 case 写 4 元组
  `{dyn_body_ptr, vtable(anon_.._19/_17/_16/_18), fov_abs, body_abs}`。
- 分歧：Rust 旧 `BODY_OFFSETS=[0x100,0x1b8,0x270,0x2f0,0x390,0x448]` 与原 **body_abs**
  `[0x70,0x128,0x1a8,0x248,0x300,0x388]` 不符，且缺 vtable/dyn_body_ptr 返回值。
- 重建：`CurrentDyn { dyn_body_ptr, vtable: CameraBodyVtable, fov_abs, body_abs }`；
  表逐行对应 decompile case 0..5（含 mode 0 `dyn_body_ptr=+0x0` 矩阵块区）。
- 判等自检：六个 case 的四个输出逐项一致；`fov_abs` 与 `FOV_OFFSETS[0..6]` 一致；
  mode ≥ 6 无 case（输出未定义），crate 以 `FOV_OFFSETS` 兜底 fov、其余 0。

### 2. current_dyn_mut / current_body_offset（行 155/167 内联）— 随依赖，重建
- 改为返回/取 `CurrentDyn.body_abs`（原 Rust 取 `.0`=旧 body 偏移）。

### 3. load_parameters（0x140911520）— 真实业务，重建（重点）
- fresh + 机器码（radare2 0x140911520..6df）：
  1. `mode@+0x4d0 = 0`；
  2. `movsd xmm6,[rdx]`（yaw@pitch）、`mulss ×[0x142aab4b0]=0x3c8efa35`、
     `mov dl,2`（EulerRot::YXZ）→ `from_euler_angles`（0x140d4b440）；
     轨道四元数 `q = from_rotation_y(yaw·K)·from_rotation_x(pitch·K)`，旋转 `(0,0,arm)`，
     结果 `(params.position − camera[+0x138/0x13c/0x140]) + rotated` 写 `+0x144/+0x148/+0x14c`
     （**不直写 Normal body 位置/yaw/pitch**）；
  3. `+0x118 = 0`（`mov dword [rsi+0x118],0`）；
  4. push 28B ActiveRotation `{+0=0(u64), +8=pitch−cam.pitch, +0xc=yaw−cam.yaw,
     +0x10=0x80000000, +0x14=0x80000000, +0x18=−1.0}` 入 rotations Vec@+0x100
     （`[rsi+0x100]=len / [rsi+0x108]=ptr / [rsi+0x110]=cap`；**替换非 push**——
     `__rust_dealloc(旧ptr, 旧len·28, 4)` → 新 alloc len=1，len 恒 1）；
  5. `bl=ignore_snapshot` 非零跳；否则 `+0x194 = params[+0x18]`（pitch_offset）。
- 重建：`load_parameters(&mut self, params, ignore_snapshot: bool)`；MainCamera 增
  `rotations: Vec<ActiveRotation>` 字段（**替换为 len=1**，len 镜像写 raw +0x100）。
- 判等自检：写入偏移/常量位/条件写逐条对应机器码；Ghidra 反编译的 yaw/pitch Δ 序有误
  （未辨 `shufps 0xe1`），以机器码 `+8=pitch Δ、+0xc=yaw Δ` 为准。
- 证据更正：早前 RESTORE 记 "π/180" 错——实为 `0x3c8efa35`（非 π/180 字面量），按位保留
  `ORBIT_ANGLE_SCALE`；ActiveRotation 载荷为 pitch/yaw Δ（非 pos.x/y）。

### 4. eye_position / eye_forward / xz_projective_rotation（0x140911dd0/df0/e10）— 真实业务（转发器），重建
- fresh：12 B 转发器 `CameraMatrices::eye_position()/eye_direction()/xz_projective_rotation()`，
  以 `self+0` 转发到**维护的矩阵块@+0x0**（`inverse_view` 列 +0xc0，0x140c96d40 读
  `[+0xc0..0x100]`）。
- 分歧：Rust 旧 `eye_position` 直读 Normal body +0x138、`eye_forward/xz` 每次从 body
  重建矩阵——"每次重建"非转发。
- 重建：MainCamera 增 `set_camera_matrices(&mut self, inverse_view)`/`rebuild_camera_matrices_from_normal_body()`
  （写 raw +0xc0），三个转发器统一读 `self.camera_matrices()`（projection=IDENTITY）。
- 判等自检：转发 callee 一致；`eye_position = inverse_view` 平移列（+0xc0..0x100 的
  w_axis，与 0x140c96d40 读 `[+0xf0/+0xf4/+0xf8]` 一致）；`xz_projective_rotation` 复用
  `utils::camera` 既有 oracle（native_oracle 原 xz_rotation 逐位比对）。

## 编译与验证

- `cargo check -p country-core`：**通过**（lib 编译零错误；仓库另存他模块历史/并发测试错误不属本模块）。
- 纯算法独立验证（/tmp/rebuild-verify，glam 0.29.3）：load_parameters 轨道旋转
  （yaw/pitch=0 → 恒等；arm=4 → (0,0,4)；pitch 半周 → 绕 X 旋转）逐断言通过。
- 测试：main_camera.rs 测试更新为新语义（ActiveRotation 记录位、+0x144/+0x148/+0x14c 轨道
  位置、维护矩阵转发）。
