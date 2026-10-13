# WP11 墙形状/轮廓几何契约 — S06 circle/rectangle/freehand

标注：**O**=反编译/PDB/机器码直接支持；**I**=推断；**U**=未恢复（typed gap）。B-only。

## 1. 形状记录 `WallShape`（PublicWallState +0x258 起，28B）

`get_wall_shape` 0x140b2a5d0：`*(u32*)(self+0x258) < 2` 成立则返回形状引用，否则 unwrap_failed。
`is_premade_shape`（行113）= 判别 <2。判别：**0=Circle、1=Rectangle、2=Freehand**（O）。

| 判别 | 载荷 | 证据 |
|---|---|---|
| 0 Circle | 16B（+0x25c..0x26c，4×f32） | O：is_circle 0x140b2a7d0 判别==0 复制 +0x25c/+0x264 |
| 1 Rectangle | 24B（+0x25c..0x274，6×f32） | O：is_rectangle 0x140b2a790 判别==1 复制 +0x25c/+0x264/+0x26c |
| 2 Freehand | 无固定载荷（徒手轮廓存于 Curve） | O：判别 2 跳过形状复制；get_wall_attachment_height Freehand 分支按 Curve 投影 |

- `is_rectangle` 返回 `Option<Rectangle>`（28B：u32 判别 + 24B 载荷，RCX 输出）；`is_circle` 返回 `Option<Circle>`（20B：u32 + 16B）。
- `corner_pos_and_normal_xz` 0x140b2a800 仅 Rectangle：经 `utils::geometry::rectangle::Rectangle2d::{get_point,get_corner_normal}` 输出 {present u32, corner Vec2, normal Vec2}（20B）。
- Roof 共享同一形状记录：Roof+8..+36 = 形状 28B（先验 `reconstruction/building/roof_spatial.rs`：tag0 比 4 浮点、tag1 比 6，ordered equality 正负零相等/NaN 不等；tag>1 跳过）。Roof 88B 组件见 wp9。
- 字段内名称 U（center/radius/dims/orientation 语义）；数值消费见 `wall_cache.rs`（regular_wall_height_cache 0x14078b865）、`wall_rectangle_edit.rs`、先验 `roof_shape.rs` RoofShapeParams。

## 2. `wall_path_segment.rs` — `WallPathSegment`

| 方法 | RVA | 业务 |
|---|---|---|
| `pt_count()` | 0x1401234970 | 预备路径点计数 |
| `length_ws()` | 0x1401234980 | 3D 世界空间长度 |
| `length_ws_2d()` | 0x1401234ad0 | 2D 长度 |
| `clone`（impl$2） | — | 深拷贝 |

语义 I：`WallPathSegment` 是墙段切分中间态（分段/轮廓预备产物），含预备点数组 + 长度。
消费方：`system_wall_constructor::segmentation`（calc_path_wall_segmentation）、`construct_platform_stairs`（process_loop_range_into_wallpath_segments）。布局 U（typed gap）。
先验对应：`reconstruction/building/wall_segmentation.rs`、`wall_platform_segmentation.rs`、`tg-building-runtime/src/wall_segments.rs`（WallLoopRange/掩码，未重推导）。

## 3. `in_progress_wall.rs` — `InProgressWall` / `InProgressWallResource`

| 类型 | 角色 | 证据 |
|---|---|---|
| `InProgressWall` | 编辑中墙（组件/瞬态） | O：`new`（行~30）；`impl$3::fmt` RVA 0x140abceb0。布局 U |
| `InProgressWallResource` | 编辑中墙资源 | O 命名实体；布局 U |

语义 I：徒手墙绘制输入→预备轮廓→提交创建的前置状态；与先验
`reconstruction/building/wall_freeform_cache.rs`（PreviewCircle/PreviewRectangle、circle_curve/rectangle_curve）、`wall_path_preparation.rs` 对应。
绘制/提交系统属 WP12；本包只列类型面。

## 4. 形状系统的 ECS 面（WP12 清单，本包不实现）
`systems/wall/shapes/*`：CreateCircleCmd/CreateRectangleCmd/EditRectangleCmd/PreviewCircle/PreviewRectangle/WipRectangle 等（骨架命名实体见 evidence.json），属 WP12 形状系统交付。
