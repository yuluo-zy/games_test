# camera_rig 核对注记（rebuild-session fresh 证据，2026-10-17）

> 依任务红线：rebuild-cameraui 并行重建 camera_rig；本 agent 以 fresh 证据核对吸收，不重复改码。

## current_dyn（0x1409149d0）— 与 cameraui 版本一致 ✓

fresh 反编译 4 元输出 `{dyn_body_ptr, vtable, fov_abs, body_abs}`：
mode0 (+0x0, A19, 0x8c, 0x70)、mode1 (+0xb8, A19, 0x144, 0x128)、
mode2 (+0x170, A17, 0x1c4, 0x1a8)、mode3 (+0x1f0, A16, 0x264, 0x248)、
mode4 (+0x290, A19, 0x31c, 0x300)、mode5 (+0x348, A18, 0x3a4, 0x388)。
cameraui 的 `CameraModeStates::current_dyn` + `CurrentDyn`/`CameraBodyVtable`/`BODY_OFFSETS` 与此逐项一致。✓

## load_parameters（0x140911520）— 与 cameraui 版本核对

fresh 反编译 + 反汇编逐条核对 cameraui 实现：
- mode=Normal ✓；`Quat::from_rotation_y(yaw·0x3c8efa35)*from_rotation_x(pitch·0x3c8efa35)`
  = `from_euler(YXZ, yaw·π/180, pitch·π/180, 0)`（glam 0.20.5 `rot_y(u)*rot_x(v)*rot_z(w)`，
  调用点 DL=2、arg1=XMM2=yaw'、arg2=XMM3=pitch'、arg3=stack 0）✓；
- 旋转 `(0,0,arm)` → `params.position - cam + rotated` 写 +0x144/+0x148/+0x14c，
  **不直写** Normal body 位置/yaw/pitch ✓（与 LOGIC 措辞一致）；
- +0x118=0 ✓；push 28B ActiveRotation `{0,0,pitch-Δ,yaw-Δ,0x80000000,0x80000000,-1.0}`
  （fresh 反汇编 `SHUFPS XMM6,XMM6,0xe1; SUBPS (params-camera+0x150/0x154)`；**LOGIC 的
  `{0,pos.x-ref,pos.y-ref}` 系误读，本 fresh 证据为 pitch/yaw Δ**）；
- `!ignore_snapshot` 时 +0x194 = params[+0x18]（CameraRigParametersCache.pitch_offset，
  6 字段存档布局）✓。

## 分歧标注（供 cameraui 修，本 agent 不并行改同一文件）

**rotations 是替换不是追加**：fresh 反汇编 `[rsi+0x100]=len` 读旧 len → 有旧 len 则
`dealloc(ptr,len*28,4)` → 置 `{len=1, ptr=new, cap=1}`。即每次 load_parameters **丢弃旧队列、
重置为单元素**。cameraui 用 `self.rotations.push(...)`（追加），且测试断言两次调用后
`len()==2`——与原相反。应改为 `self.rotations = vec![ActiveRotation::new(...)]`
（或 clear+push），测试断言改 `len()==1`。
