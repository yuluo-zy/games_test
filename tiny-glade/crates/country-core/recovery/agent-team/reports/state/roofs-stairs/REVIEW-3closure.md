# REVIEW — roofs-stairs 三项闭合复验（组 state）

> 审查员：reviewer-state，2026-10-16。复核 restorer-roofs-stairs 闭合三项。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；
> roof_shape 7 + invalidate 1 测试通过；全 crate **1720/1**（唯一败 geometry::sphere，既知）。

## 逐项复核

| 项 | 修复 | verdict |
|---|---|---|
| roof_shape ~13 谓词 | GableRoofStyle is_stepped（tag==0，0x140833230）/ is_overhang（tag!=0，0x1408c5680）；FlatRoofStyle is_wooden（tag0/1/2 分支，0x1408e31a0 I）/ pattern_id（内联 I 字节+1）；profile_curve_normalized/ws 委托 system_roof（209/1,512 案例）；rotation_yaw_radians（rect atan2/圆分支）；roof_shape_at_intersection_y + RoofIntersectionSection（越界 tag2）；ridge_length_01_from_rectangle_ratio（常量 0.94736844/−1.5）；roof_height_ws_from_wall_shape（SineInOut tag2，circle 10/4·rect 6/3）；RoofShapeParams eave_length_ws/height_ws（system_roof）——**借 system_roof 不重复实现**。 | **OK** |
| stairs invalidate.rs | `invalidate_dirty_stair_artifacts`（0x140b37180）：脏边 drain → `contains_edge_or_stub` 判定 → `reset_edge`/`remove_edge`；碰撞体撤除×3 + Arc drop + RtMesh 删除面为胶水登记（collision/rt 域）。 | **OK** |
| RoofBoundsDims 消费端 | 全在 system_roof/decorator/wall_constructor 库内，country-core 无直接消费面；bottom/top 反编译定案，render 侧接线登记。 | OK（登记） |
| 待取证登记 | from_wall_shape（墙域）/ roof_uv_at_ws_pos（碰撞管线 R1 接线）/ wall_shape 精确类型 / y==floor 帧体近似——诚实标注。 | OK（登记） |

## 结论与 DISPATCH 建议
- **三项 → 闭合**（roof_shape 谓词族 + invalidate + RoofBoundsDims 消费端确认）。
- 待取证登记维持（from_wall_shape 墙域、roof_uv_at_ws_pos R1、wall_shape 类型、y==floor 帧体近似）。
- roofs/stairs → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（视觉 RtWorld/mesh 接线、
  collision/rt 胶水接线、render 消费端、全局调度 + 各域系统批量并入）。
