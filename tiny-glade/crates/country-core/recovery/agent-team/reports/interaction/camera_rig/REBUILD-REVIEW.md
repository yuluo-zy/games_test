# REVIEW — camera_rig + time_weather + ui_mode 真实逻辑重建(rebuild-cameraui 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-cameraui REBUILD.md×3(三模块 20 需重做,fresh 反编译 + radare2 机器码);
`cargo check --lib` exit 0(独立复验)。

## 逐项复验

### ① 分类(20 全真业务)— PASS
camera_rig current_dyn/load_parameters/eye×3、time_weather 4、ui_mode 12——全部真实业务;
胶水 register_required_components(BrushPreview 注册存根)+ signifiers 注册/调度**不重建**(登记
接线缺口);UiFocus::is_none VA 0x140985f30 为 AllocationHandle::is_null 折叠(语义 ==0 平凡,
保持)✓。

### ② camera_rig — **FIX(rotations 替换)**
- `current_dyn` 4 元组 {dyn_body_ptr, vtable, fov_abs, body_abs}:六 case 表(0 Normal +0x0/
  anon_19/+0x8c/+0x70 … 5 Butterfly +0x348/anon_18/+0x3a4/+0x388);**BODY_OFFSETS 纠为
  +0x70/0x128/0x1a8/0x248/0x300/0x388**(原 +0x100 系错)✓;fov_abs 与 FOV_OFFSETS 前 6 项
  一致 ✓。
- `load_parameters`:ORBIT_ANGLE_SCALE=**0x3c8efa35**(非 π/180 字面量,按位保留)、
  from_euler(YXZ) 轨道四元数、写 +0x144/+0x148/+0x14c(不直写 Normal body)、+0x118=0、
  `!ignore_snapshot` 条件写 +0x194=pitch_offset ✓;ActiveRotation 载荷 = pitch/yaw Δ
  (机器码 `shufps 0xe1`,非 pos)✓。
- **⚠️ FIX(重建会话提示证实)**:机器码 0x140911520 的 rotations Vec@+0x100 处理为
  **替换非 push**:
  ```
  test rax(旧len); je 跳过; __rust_dealloc(旧ptr, 旧len*28, 4)
  len=1; ptr=r14(新alloc); cap=1
  ```
  当前 Rust `self.rotations.push(...)` **追加**(二次 load 后 len=2,原版恒 1)。**须改
  replace**(清空+push 单元素,或 `rotations = vec![new]`),并镜像 len/ptr/cap。这是本轮
  唯一确认 FIX。
- eye×3 转发:统一读维护的 `CameraMatrices`(inverse_view 平移列 +0xc0..0x100,非每次从
  body 重建)✓。

### ③ time_weather — PASS
- `advance_playback`:**每帧强制 state 到 dev 方向**(dev 关恒 Rewinding、dev 开恒 Playing),
  旧 state 只决定 progress(Idle→0、Playing→dev 关不变/dev 开 +=dt/3、Rewinding→dev 关
  +=dt/−5 clamp≥0/dev 开不变)✓。
- `advance_tod_hour`:`increment=2.0f32.powf(progress)·progress·dt·TOD_SPEED·dev_speed`,
  `(增量+target_hour+1.0).rem_euclid(1.0)`,hour=fmod(target·24+8,24)→remap ✓。
- `suncalc_pos_to_vec`(azimuth, elevation, basis_angle, orbit_angle):正交基矩阵 +
  **半角 Y 轴旋转**(sin/cos(angle/2))+ **−0.15** + **1/sqrt 归一化**;次级向量(光照快照
  +0x30/+0x34/+0x38)登记待接 ✓。
- `time_of_day` 主系统:音频使能门→角度/小时插值→双 beat/输入同步→dev 热键→播放机→
  小时推进→day_of_year panic→sun_position_at→DirectionalLight→TodLightingSnapshot/
  SunSkyColors 写盘→音频上报;13 参签名(音频上报/热键/beat/sun_position_at/snapshot/
  DirectionalLight/SunSkyColors)✓。

### ④ ui_mode — PASS
- WidgetParams::new:默认 u32 组 + **occlude=true(默认)+ flags=0x01(bit0=record_history_
  when_used)**;no_history_recording **清 bit0**(`&0xfe`,纠早期"置位"反);allow_negative_
  distance(bool)带参(`<<2 | &0xfb`);new_with_sounds_and_volume 带参面 ✓。
- as_mei 条件返回(self==Mei → Mei,否则 None,机器码 xor/cmp/sete)✓。
- slider3d:mouse_ss_to_cs(0x140c906d0)+ project_onto_ray(0x140ac84a0),Slider3dState
  {screen, first, second, range} ✓。
- UiSignifierStream:环形流 `[UiAppendSignifier; 8]` + write_counter + **满时自旋等 reader**
  (write_counter−read_cursor==槽数)✓。
- signifier_create 去重 + spawn;animation kind switch;brush_preview flag@+0xfa 门 →
  Query + Commands::remove<Transform> 清旧 → 直接挂 Transform(非事件化)✓。

## 结论与 DISPATCH 建议

- camera_rig/time_weather/ui_mode 重建复验:19/20 通过;**1 项 FIX = load_parameters
  rotations 替换非 push**(机器码 __rust_dealloc+len=1/cap=1,当前 push 追加)。
- 建议:除 rotations replace FIX 外需重做项 → 闭合;FIX 交 rebuild-cameraui 修后复验。
  ActiveRotation 载荷 pitch/yaw Δ、角度系数 0x3c8efa35、suncalc 半角/−0.15/归一化、
  advance_tod_hour exp2f、WidgetParams 默认位——证据更正全部落实 ✓。
