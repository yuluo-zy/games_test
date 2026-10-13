# 共享基础：业务函数接口进度

基线：B。主表只统计已分类的业务接口；宏自动实现/库转发、类型/schema合同及待分类项另列。PDB不是作者API全集，不能从文件数、Rust声明数或测试数推导完成率。

| 范围 | 已观察 | 业务已分析（部分） | 未触及 | 候选实现 | 原入口对照（局部） | 待归属 |
|---|---:|---:|---:|---:|---:|---:|
| lang-lib | 11 | 11 | 0 | 11 | 0 | 0 |
| paths | 10 | 10 | 0 | 10 | 0 | 0 |
| tabloid | 3 | 3 | 0 | 3 | 0 | 0 |
| theme-defs | 0 | 0 | 0 | 0 | 0 | 0 |
| utils 顶层 | 135 | 99 | 36 | 100 | 20 | 0 |
| utils 几何/曲线/Transform | 277 | 128 | 149 | 121 | 39 | 0 |
| asset / asset-pipe | 32 | 29 | 3 | 29 | 2 | 0 |
| 合计 | 468 | 280 | 188 | 274 | 61 | 0 |

“未触及”指尚无接受的原接口业务分析；仅有PDB名字、伪代码或猜测适配器仍属未触及。候选实现与分析是独立维度。原入口对照不代表全部泛型/输入/调用链验收。

原始PDB库存仍保留609项。分类后业务接口468项；自动实现/库转发62项、类型/schema合同helper 46项、待分类33项均不计入未实现业务函数。数字变化是口径纠正，不是新增恢复。

[宏、trait与类型合同分类清单](INTERFACE_CLASSIFICATION.md)。

[完整机器台账](tracking/shared-foundation-interfaces.json)。原类型/参数未知时不使用void()占位推定ABI。

## 接口审计来源

- [small-interfaces](evidence/shared-foundation-2026-10-13/small-interfaces/interface-coverage.md)
- [utils-base](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.md)
- [geometry-curves](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.md)
- [assets-small-first](evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.md)

## 代码 review 与修复

[本轮独立 review、修复、性能实测与功能缺口](evidence/shared-foundation-review-2026-10-13/REVIEW.md)。
逐接口机器台账附有当前恢复文件的 review SHA 与同文件发现；这是代码检查记录，不将未触及接口提升为完成。

## 本批状态与后续

- utils顶层：按手写业务接口补齐；Default/fmt等先查宏/自定义归属，不以方法名创建逆向任务。
- curve：CurveUCircular九接口完成本批对照；真正CurveSlice状态对象正在进行原逻辑对照，未提前计入。
- asset：默认值/继承宏的字段策略另记类型合同，原scanner/loader/pipeline业务仍按接口推进。
- 小模块：theme-defs字段载荷和tabloid初始化/生命周期合同独立保留，派生或依赖转发不计业务缺口。

## 全部未触及业务接口

| 范围 | 原接口 | 依据与后续 |
|---|---|---|
| utils 顶层 | `utils::angle_range::wrap_angle` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::circular_f32::CircularF32::project_unbounded` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::comparative_space::ComparativeSpace::lerp_clamp` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::circular_in_out` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::cubic_out` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::ease_in_out_elastic` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::ease_out_back` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::shoulder` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::easing_functions::sine_in_out` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::index_condenser::IndexCondenser::insert` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::local_time::now_appox_local` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::loop_range::LoopRange::map_slice` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::loop_range::LoopSlice::len` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::natural_cubic::Spline2D::find_segment_and_dt` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::natural_cubic::solve_cyclic` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::natural_cubic::solve_tridiagonal` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::rdp::Rdp::into_indices` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::rdp::Rdp::new` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::rdp::Rdp::stitch_with` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::rgb9e5::clamp_range_for_rgb9e5` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::rgb9e5::floor_log2` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::selective_deserialize::impl$0::expecting` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::selective_deserialize::impl$4::visit_str` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::selective_deserialize::impl$5::deserialize` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::as_vec2` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::bound` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::is_bounded` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::new` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::offset_x_clamped` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::subtract` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::to_affine3` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::with_offset` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::with_offset_clamped` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::with_offset_unbounded` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceCoord::x` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 顶层 | `utils::wall_space::WallSpaceRegion::intersects` | [逐接口证据](evidence/shared-foundation-2026-10-13/utils-base/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::compute_affine` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::compute_matrix` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::forward` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::from_scale` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::from_translation` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::from_translation_scale` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::identity` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::Transform::with_translation` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::components::transform::impl$1::default` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve3XZ::into_curve3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::_deprecated_new_empty` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::bounding_box_2d` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::calculate_points_u_and_length` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_lerp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count_remap` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_per_segment` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_remap` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment_2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::crashy_to_curve2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::end_pos` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_coord_at_2d_ws_distance` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_coord_at_wall_space` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_curve_segment_from_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_pos_at_coord_unchecked` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_u_at_coord` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_unnormalized_central_tangent_at_coord` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::get_unnormalized_tangent_at_coord` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::is_empty` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::is_valid` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::length_ws` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::point_count` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::point_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::points_with_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::reinterpret_as_curve2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::segments` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::segments_with_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::smooth` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::smooth_in_place` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::smooth_weighted` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::start_pos` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::translate` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_new_from_points_remove_duplicates` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_resample` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count_remap` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_resample_remap` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_resample_remap_per_segment_2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::Curve<T,CSem>::try_to_curve2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::CurveInternalCoord<CSem>::i0` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::CurveInternalCoord<CSem>::i0_t` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::CurveInternalCoord<CSem>::i1` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::CurveInternalCoord<CSem>::new` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::CurveInternalCoord<CSem>::new_unchecked` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve2_serialized_as_curve3::deserialize` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve2_serialized_as_curve3::serialize` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurvePoints::is_empty` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::end` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left_until` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right_until` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_coord_range` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_curve_range` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_point_range` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::iter_coords` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::measure_length` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap_2d` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::start` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::impl$0::iter` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::impl$1::iter` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_slice::impl$1::len` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveU<CSem>::clamp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveU<CSem>::lerp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveU<CSem>::max` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveU<CSem>::min` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveU<CSem>::u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveUBoundless::clamp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveUBoundless::is_within_01_range` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveUBoundless::new` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::CurveUBoundless::u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::impl$2::visit_f64` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::curve_u::impl$3::deserialize` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$0::vector_norm` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$11::sub` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$12::sample_at_coord` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$13::smooth` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$14::smooth` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$1::vector_norm` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$2::vector_norm` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$5::hint_capacity` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::impl$5::receive_points` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::curve::smooth_in_place` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::Aabb2::from_points` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::Aabb2::overlaps_segment` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::Aabb3::from_center_extent` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::Aabb3::from_points` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::intersection_aabb_segment2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::aabb::intersection_aabb_segment3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::barycentric::barycentric2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::Circle2d::as_curve2_range` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::Circle2d::as_curve3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::Circle2d::lerp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::Circle2d::yaw_radians` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::Segment2d::new` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::circle::is_default` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::line_relation::LineInterval::cross` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::line_relation::LineInterval::div` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::line_relation::LineInterval::t_coord_to_point` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::ray::Ray::from_mouse_ss` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::_get_normal_at_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::as_curve3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::as_points3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::as_transformation_and_size` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::circular_slice` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::closest_edge_normal` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::expand_along_edge` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_closest_edge_id` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_corner_unnormal` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::get_normal_at_u` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::half_dimensions` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::intersects` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::intersects_circle` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::lerp` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::local_aabb_of` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::orientation_mat2` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle2d::orientation_mat3` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle3d::center` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle3d::height` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle3d::length` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle3d::rotate_projective_xz` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::Rectangle3d::width` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::rectangle::impl$2::from` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::triangle_triangle::cross_intersect::intersect_plane` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::ws_to_cs` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| utils 几何/曲线/Transform | `utils::geometry::ws_to_ss` | [逐接口证据](evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)；待原业务分析 |
| asset / asset-pipe | `asset::prefab::scan::PrefabScanResult::add_file_to_prefab_dependency` | [逐接口证据](evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json)；待原业务分析 |
| asset / asset-pipe | `asset::prefab::scan::load_prefab_config::do_load` | [逐接口证据](evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json)；待原业务分析 |
| asset / asset-pipe | `asset_pipe::texture::BcMode::block_bytes` | [逐接口证据](evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json)；待原业务分析 |
