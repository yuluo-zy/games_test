# REBUILD — camera_rig 逐函数重建复验（restorer-camera-rig）

日期: 2026-10-17
角色: restorer（camera_rig 域 4 函数重建）
协议: planning/2026-10-16-logic-verify-protocol.md（fresh 反编译→重建控制流→Rust 逐语句判等 + 分歧模式检查表）
依据: 本会话 fresh ghidra `--worker shared` 反编译（0x140911520 / 0x1409149d0 / 0x140d4b440）、radare2 机器码
（0x140911520 全段 0x140911520..6fd、from_euler_angles YXZ case 0x140d4b4d2..6db）、read_memory
（0x142aab4b0=0x3c8efa35、0x1429cd330=0x80000000×2、0x142925904）、glam 0.29.3 独立数值验证
（/tmp/rebuild-verify：`rot_y(a)*rot_x(b)*(0,0,arm)` == (sin a·cos b·arm, −sin b·arm, cos a·cos b·arm)；
`from_euler(YXZ, a, b, 0)` == `rot_y(a)*rot_x(b)`，四组 yaw/pitch 全通过）。

## 结论表

| VA | 原函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x140911520 | MainCamera::load_parameters | main_camera.rs:324 | **判等** | 本轮补 ptr@+0x108/cap@+0x110 镜像（原仅 len）；mode≥? 无；详见下 |
| 0x1409149d0 | CameraModeStates::current_dyn | mod.rs:90 | **判等** | mode≥6 原输出未初始化(UB)，Rust 兜底（FOV_OFFSETS 取 fov，余 0）——安全默认，非逻辑分歧 |
| (inline 行155) | current_dyn_mut | mod.rs:114 | **判等** | 同 current_dyn（纯 mode 函数，可变版无差异） |
| (inline 行167) | current_body_offset / current_body_mut | mod.rs:109 | **判等** | 取 current_dyn().body_abs，与原 lVar3 表一致 |

## 逐函数判等细节

### 1. load_parameters 0x140911520 — 判等（重点）
fresh 反编译 + 机器码 0x140911520..0x1409116fd 逐条核对：
1. `MOV byte[+0x4d0],0` — 模式字节=Normal；**不写 Normal body 位置(+0x138)/yaw(+0x154)/pitch(+0x150)/fov(+0x190)**。Rust `set_mode_byte(NORMAL)` + 无 body 直写 ✓。
2. 轨道四元数：`XMM2=yaw*K`(MOVSD+shufps 0xe1+MULSS ×[0x142aab4b0]=0x3c8efa35)、`XMM3=pitch*K`、`[RSP+0x20]=0`、`DL=2`(YXZ) → `from_euler_angles`(0x140d4b440)。YXZ case 机器码 0x140d4b4d2..6db 解码：三轴角 XMM6=pitch/XMM7=yaw/XMM8=0，bVar1=true 分支产出
   `q=(C_y·S_p, S_y·C_p, −S_y·S_p, C_y·C_p)` = **glam `rot_y(yaw*K)·rot_x(pitch*K)`**（Hamilton 积 z=−S_y·S_p 由 −y1*x2 项，非零——早前近似错误，机器码为准）。Rust `Quat::from_rotation_y(yaw*K)*Quat::from_rotation_x(pitch*K)` ✓。
3. 旋转 `(0,0,arm)`（load_parameters 内联公式 = 标准 q*(0,0,arm)*q⁻¹，与 from_euler 后 XMM 数值一致）：结果 `(sin yaw·cos pitch·arm, −sin pitch·arm, cos yaw·cos pitch·arm)`。glam 独立数值验证通过 ✓。
4. 写 `+0x144/+0x148/+0x14c = (params.position − camera[+0x138/13c/140]) + rotated`。机器码 `MOVLPS[+0x144]`(x,y)+`MOVSS[+0x14c]`(z) ✓。Rust `new_pos = params.position − cam_pos + rotated` 写三槽 ✓。
5. `MOV dword[+0x118],0` ✓。
6. rotations Vec@+0x100 **替换非 push**：`TEST [RSI+0x100](旧len); JZ skip; __rust_dealloc(旧ptr, len*28, 4)` → `len=1 / ptr=r14(新alloc) / cap=1`。Rust `self.rotations = vec![...]`（drop 旧 Vec + 新 alloc，len 恒 1）✓。
7. ActiveRotation 28B：`[RAX]=0(u64)`、`MOVSD XMM0,[+0x150]; SUBPS XMM6,XMM0`（pitch Δ 进 +0x8、yaw Δ 进 +0xc）、`UNPCKLPD XMM6,[0x1429cd330]`（+0x10/+0x14=0x80000000）、`+0x18=0xbf800000`。即 `{0, pitch−cam.pitch@+0x150, yaw−cam.yaw@+0x154, 0x80000000, 0x80000000, −1.0}`。Rust `ActiveRotation::new(pitch_delta, yaw_delta)` ✓。
   - 分歧点（本轮修复）：机器码写 `[RSI+0x108]=新ptr` 与 `[RSI+0x110]=1`（cap）。原 Rust 只镜像 len@+0x100。**本轮补 ptr/cap 镜像**（`as_ptr()`/`capacity()`，新 Vec 容量恒 1 无再分配，镜像有效）。
8. `TEST BL,BL; JNZ skip; MOVSS [RSI+0x194],[RDI+0x18]` — `!ignore_snapshot` 时 `+0x194 = params.pitch_offset`。Rust `if !ignore_snapshot { write_f32(0x194, params.pitch_offset) }` ✓。

分类：真实业务（非宏胶水）。仅 camera_rig 域文件改动。

### 2. current_dyn 0x1409149d0 — 判等
fresh 反编译 `switch(byte[+0x3d0])` 六 case 写 4 元 `{dyn_body_ptr, vtable, fov_abs, body_abs}`：
- body_abs = 0x70/0x128/0x1a8/0x248/0x300/0x388 — 与 Rust `BODY_OFFSETS` **逐项一致**（旧错表 [0x100,0x1b8,0x270,0x2f0,0x390,0x448] 已弃）。
- fov_abs = 0x8c/0x144/0x1c4/0x264/0x31c/0x3a4 — 与 Rust `FOV_ABS` / `FOV_OFFSETS[0..6]` 一致。
- dyn_body_ptr = +0x0/+0xb8/+0x170/+0x1f0/+0x290/+0x348 — 与 Rust `DYN_BODY_PTRS` 一致。
- vtable = anon_19/anon_19/anon_17/anon_16/anon_19/anon_18 — 与 Rust `A19/A19/A17/A16/A19/A18` 一致。
- 输出 = `*out=lVar2; out[1]=vtable; out[2]=lVar4+base; out[3]=lVar3+base`（绝对地址）。Rust 存相对偏移（调用方加 base），crate 内表示选择，值与原表一致。
- 分歧点：mode≥6 反编译无 case（输出未初始化=UB）。Rust `_ =>` 返回定义值（fov 取 FOV_OFFSETS[mode]、ptr/body=0、vtable=A19）——安全默认，非逻辑分歧（原版该区间不调用/未定义）。

### 3. current_dyn_mut / current_body_offset — 判等（随依赖）
current_dyn 为纯 mode 函数，可变版输出相同。`current_body_mut`（行167内联）= 取 body_abs。Rust 两者均正确转出 `current_dyn().body_abs`。crate 内无调用方（仅定义，供未来 rollercoaster_update 等接线）。

## 调用方核对（load_parameters 签名未变，不破坏）
xrefs：record_and_replay_input_for_trailers(0x14083cc7d) / control_input_replay(0x140941b2e) / load_game(0x14146dcbb)。
- Rust 库内无 load_parameters 调用方：input_record_replay.rs:508-514、input_replay_for_trailers.rs:103-110 仅在注释描述"seek 末同步点→load_parameters+immediately_finish_interpolating"，实际接线为宿主（库外）。签名 (self, &CameraRigParametersCache, ignore_snapshot: bool) 与机器码 R8B 门一致，宿主接口不受影响。
- 单测：main_camera.rs 现有 5 个 load_parameters 相关测试（替换非 push、轨道常量、+0x118、ActiveRotation 载荷、ignore_snapshot 门）**已过**；本轮新增 ptr/cap 镜像断言。camera_rig 24 单测 + input 42 单测全过。
- 未改动判等函数（default_fov/previous/store_parameters/target_fov/immediate_set_fov/eye×3）。

## 编译与验证
- `cargo check -p country-core --lib`：exit 0（仅既有 warnings）。
- `cargo test -p country-core --lib camera_rig`：24 passed。
- `cargo test -p country-core --lib input::`：42 passed。
- 独立 glam 0.29.3 数值验证（/tmp/rebuild-verify，与 workspace glam 版本一致）：组合 yaw/pitch 轨道、from_euler(YXZ) 等价逐断言通过。
