# REVIEW2 — systems/camera_rig(卡D-slice2 终验 → 闭合·转发器+复用)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer RESTORE.md 二.卡D-slice2 节;`cargo check --lib` exit 0(独立复验)。

## 核对

### ① eye_position / eye_forward / xz_projective_rotation / reset — PASS(转发器 + 复用)
- `eye_position`(0x140911dd0,12B 转发器→FUN_140c96d40):读 Normal body 位置(0x138..144)✓。
- `eye_forward`(0x140911df0,eye_direction 同址→FUN_140c96d80):由 Normal body 的
  yaw/pitch 组 Quat → `utils::camera::CameraBodyMatrices::from_position_rotation`
  (inverse_view=Mat4::from_rotation_translation)→ `CameraMatrices::eye_direction()`
  (inverse_view·−Z 归一)✓,与 utils 共享实现对齐。
- `xz_projective_rotation`(0x140911e10→FUN_140c96e70):`CameraMatrices { projection:
  IDENTITY, inverse_view }` 上调用 `xz_projective_rotation()`(仅 inverse_view 参与,投影
  占位不参与该旋转——与 RESTORE 声明一致)✓。
- `reset`(行 428,内联,I):清缓冲 + 模式=Normal + fov=45 + 位置 (0,5,0) ✓(章节"重建默认
  装配"语义)。
- 非 Normal body 的位置读取/矩阵构造待取证(crate 内 MainCamera 不透明缓冲,normal_matrices
  仅按 Normal body 组,I)——诚实登记。

### ② 复用边界 — PASS
`utils::camera::{CameraMatrices, CameraBodyMatrices}`(共享相机数学,含 eye_direction/
xz_projective_rotation/from_position_rotation)、`store_parameters`/模式字节/FOV_OFFSETS
均复用既有,未重实现。编译通过证实方法存在。

### ③ 待取证登记诚实 — PASS
`immediately_finish_interpolating`(0x140911b50,106KB)/`init_walk_camera`(0x14090e510)/
`match_free_camera_to_normal`(0x14090e6f0)/`MainCamera::new`(六 body 装配)/`zoom`
(exp2(clamp(amount·−0.08,−1,1)))/模式转发(rotate_yaw_pitch/constrain/translate)/body 更新
算法——全列于 RESTORE 仍缺失/待取证;time/weather/daily 属下一子卡(TimeOfDay 衔接卡0)。
无冒充。

## 结论与 DISPATCH 建议

- 卡D-slice2 复验通过:**slice2(eye_* 转发器 + reset + utils 复用)闭合**。
- 建议:slice2 → 闭合;slice3(time/weather/daily)续派。camera_rig 维持"实现主体对得上、
  待取证诚实登记"限定结论,不宣布整模块完成。
