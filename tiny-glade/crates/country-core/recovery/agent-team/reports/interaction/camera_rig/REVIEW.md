# REVIEW — systems/camera_rig(卡D-slice1 终验 → 闭合·主闭包)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer-interaction RESTORE.md(reports/interaction/camera_rig/);`cargo check --lib` exit 0
(独立复验;复验期间曾见一次性瞬态编译错误——restorer 在途编辑,camera_rig 现状态通过)。

## 核对

### ① MainCamera::load_parameters @0x140911520 — PASS(结构 + 往返)
- Normal body 位置/yaw/pitch 直写(NORMAL_*_OFFSET = 0x138/0x150/0x154),与 store_parameters
  (读 yaw@+0x154/pitch@+0x150/pos@+0x138)同单位,`load_parameters_restores_normal_body_and_round_trips`
  测试验证 store→load 往返保持 ✓。
- 模式字节=Normal(0)✓;fov=默认 45(`serde_default_vertical_fov_degrees`@0x140911e30 返回 45,
  fov 不在 6 字段存档)✓。
- 章节明确原版另做:π/180 轨道四元数、push 28B ActiveRotation 入 rotations Vec@+0x100、
  +0x118=0、flag 时 +0x194=记录分量——依赖 body 子结构,**已诚实标待取证**(crate 内
  MainCamera 为不透明缓冲,未实现这些)。
- fov=45 直写为适配选择(I,章节未明确 load_parameters 写 fov;params 无 fov 字段,合理)。

### ② CameraModeStates::current_dyn @0x1409149d0 — PASS(结构)
- 模式@+0x3d0 → (body 区偏移, fov 槽偏移);六 body 区 @+0x100/0x1b8/0x270/0x2f0/0x390/0x448
  与章节一致 ✓;模式 6 = body@+0x0 矩阵块区 ✓。
- fov 槽经 FOV_OFFSETS(13 项)复用 main_camera ✓。
- 章节原返回 `{body 指针, 表, (fov_current, fov_target)对}` 三元组;当前返回
  `(body_offset, fov_offset)` 二元简化——crate 内 MainCamera 为不透明缓冲、调用方直接用偏移,
  简化可接受,标 O 结构。

### ③ 复用边界 — PASS
CameraRigParametersCache(6 字段存档 schema)/default_vertical_fov_degrees/FOV_OFFSETS/
CameraMode 复用卡0 既有;store_parameters 未动 ✓。

### ④ 待取证登记诚实 — PASS
immediately_finish_interpolating(0x140911b50, 106KB)/zoom(0x140911280)/init_walk_camera/
match_free_camera_to_normal/MainCamera::new/模式 body 更新算法——全部列于 RESTORE 仍缺失/待取证,
无冒充。eye_position/eye_forward/xz_projective_rotation(12B 转发器)本轮已按 Normal body 补
(extra,超出 slice-1 清单;非 Normal body 位置读取待取证)。

## 结论与 DISPATCH 建议

- 卡D-slice1 复验通过:**camera_rig 主闭包(load_parameters + CameraModeStates)闭合**。
- 建议:slice1 → 闭合;slice2(剩余 body 更新算法/时间天气)按既有待取证清单续派。
  camera_rig 维持实现层面"已实现主体对得上、待取证诚实登记"的限定结论,不宣布整模块完成。
