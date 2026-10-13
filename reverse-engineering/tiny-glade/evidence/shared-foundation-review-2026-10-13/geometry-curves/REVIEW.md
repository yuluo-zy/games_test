# components / curve / geometry 独立代码 Review

发现并修复 1 项 P1 业务缺陷和 1 项 P2 重复实现。功能仍未完善，原游戏调用链尚未验收。

| 编号 | 严重度 / 类型 | 结论 |
|---|---|---|
| GEO-01 | P1：三角交叠漏判 | 已修复。3 组 0.001 尺度输入原作返回 Segment，候选返回 Disjoint。 |
| GEO-02 | P2：重复实现 std 搜索 | 已改为 slice::partition_point，保留右偏规则。 |
| GEO-03 | P1：功能缺口 | Thresholded3D / 泛型策略 / 原消费者链仍未闭合。 |
| GEO-04 | P2：性能风险 | reverse 全点 clone 和重新分配，未基准测量。原作入口也有 clone，未擅改。 |

## 业务缺陷与修复证据

原 B `0x141fdfd80 → 0x1424181a0` 先按 ±1e-10 的四点 determinant 选两条边，再计算边所在直线与平面的交点。容差用于选择边，不能把容差内顶点直接吸附到平面，也不能把求交参数夹到线段内。候选原来的 plane_cut + 两区间裁剪在小尺度下漏判。修复直接表达该业务策略，调用 glam 的 determinant / cross / normalize，保持原输出的端点顺序。

新增 **304 个真实原入口案例**：245 个 1e-4～1e8 尺度随机三角、32 个细三角/端点移动、27 个 NaN/±Inf；旧 257 案例继续通过。分类精确一致；新增有序端点残差界为 max(abs(inputs), abs(expected)) × 3e-5 + 1e-10，旧有序端点保持绝对 3e-5。没有声明端点逐位一致。新增案例 38 / 48 / 55 是实际失败复现。原指令在 Unicorn 执行，发行 RNE SHA 固定；未启动完整游戏。

## 性能与依赖

修正后的三角交叠热路径全部使用固定大小的栈上值，静态去掉每次调用两次 Vec 分配及两次排序。未执行延迟对比，不由此宣称整体提速。最近曲线查询为 O(log n)，近点搜索、裁切测长和延伸均为 O(n)；审查范围内没有发现新的 O(n²) 路径。reverse 的 O(n) clone/重建风险单列，原消费者调用频率未知。

glam / lerp / itertools / take-until / std 直接复用。唯一确认的通用轮子是 Curve::get_coord_at_u 中的手写二分，已删除；328 个原 Curve2 采样继续逐位通过。标量乘加顺序、投影选择、循环区间、碰撞边界是有原入口证据的私有规则，不能为消除表面算术代码而任意更改。CirclePoints2 是采样策略包装，不是重写迭代器库。

## CurveSlice 审查范围

现有状态对象、端点、坐标迭代、remap、延伸源码已经编译并通过现有 fixture；普通借用点集及逻辑 LoopCurvePoints 不复制底层点集。延伸以 tuple_windows 和原 take-until 依赖实现，停止顶点包含在输入中。零长度边导致 NaN、零跨度保留双端点等行为来自原指令，不能当成随意修复的缺陷。

但原 Thresholded3D 采样策略、所有 remap/stop 具体调用闭包、所有 RangeBounds 变体和原游戏消费者未验收。四组 fixture 不能代表 17 个原 API 全面完成。逐接口记录如下：

| 原接口 | 当前候选对应 | 仍缺 |
|---|---|---|
| `utils::curve::curve_slice::CurvePoints::is_empty` | `CurvePoints::is_empty` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::end` | `CurveSlice::end` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left` | `CurveSlice::extend_left` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left_until` | `CurveSlice::extend_left_until` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right` | `CurveSlice::extend_right` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right_until` | `CurveSlice::extend_right_until` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_coord_range` | `CurveSlice::from_coord_range` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_curve_range` | `CurveSlice::from_curve_range` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_point_range` | `CurveSlice::from_point_range` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::iter_coords` | `CurveSlice::iter_coords` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::measure_length` | `CurveSlice::measure_length` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap` | `CurveSlice::remap` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap_2d` | `CurveSlice::remap_2d` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::start` | `CurveSlice::start` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::impl$0::iter` | `CurvePoints::iter (slice/loop receiver ownership requires confirmation)` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::impl$1::iter` | `CurvePoints::iter (slice/loop receiver ownership requires confirmation)` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |
| `utils::curve::curve_slice::impl$1::len` | `CurvePoints::len (receiver ownership requires confirmation)` | 泛型/调用方策略及原消费者闭合；当前权威状态 `untouched` 保留 |

## 原接口未实现清单

以下来自当前权威接口审计。Review 不借源码存在或局部测试自动提升接口状态；CurveSlice 的新候选映射已在上表单列。源表的完整 inline 分母仍未知。

| 原文件 | 未实现原接口 | 原状态 / 归属 |
|---|---|---|
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::compute_affine` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::compute_matrix` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::forward` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::from_scale` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::from_translation` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::from_translation_scale` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::identity` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::Transform::with_translation` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::impl$1::default` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::impl$2::clone` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::impl$4::clone_behavior` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::impl$4::register_required_components` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/components/transform.rs` | `utils::components::transform::impl$7::eq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve3XZ::into_curve3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::_deprecated_new_empty` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::bounding_box_2d` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::calculate_points_u_and_length` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_lerp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count_remap` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_per_segment` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_remap` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment_2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::crashy_to_curve2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::end_pos` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_coord_at_2d_ws_distance` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_coord_at_wall_space` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_curve_segment_from_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_pos_at_coord_unchecked` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_u_at_coord` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_unnormalized_central_tangent_at_coord` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::get_unnormalized_tangent_at_coord` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::is_empty` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::is_on_right_side` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::is_valid` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::length_ws` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::point_count` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::point_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::points_with_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::reinterpret_as_curve2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::segments` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::segments_with_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::smooth` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::smooth_in_place` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::smooth_weighted` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::start_pos` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::translate` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_extend` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_new_from_points_remove_duplicates` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_resample` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count_remap` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_resample_remap` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_resample_remap_per_segment_2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::Curve<T,CSem>::try_to_curve2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::CurveInternalCoord<CSem>::i0` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::CurveInternalCoord<CSem>::i0_t` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::CurveInternalCoord<CSem>::i1` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::CurveInternalCoord<CSem>::new` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::CurveInternalCoord<CSem>::new_unchecked` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize::impl$0::visit_str` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize::impl$1::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize::impl$2::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize::impl$2::visit_map` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::_::impl$0::serialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::curve2_serialized_as_curve3::deserialize` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::curve2_serialized_as_curve3::serialize` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurvePoints::is_empty` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::end` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left_until` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right_until` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_coord_range` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_curve_range` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_point_range` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::iter_coords` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::measure_length` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap_2d` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::start` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::impl$0::iter` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::impl$1::iter` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_slice.rs` | `utils::curve::curve_slice::impl$1::len` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveU<CSem>::clamp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveU<CSem>::lerp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveU<CSem>::max` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveU<CSem>::min` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveU<CSem>::u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveUBoundless::clamp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveUBoundless::is_within_01_range` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveUBoundless::new` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::CurveUBoundless::u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$0::serialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$12::eq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$13::partial_cmp` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$2::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$2::visit_f64` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$3::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/curve_u.rs` | `utils::curve::curve_u::impl$8::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$0::vector_norm` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$11::sub` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$12::sample_at_coord` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$13::smooth` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$14::smooth` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$1::vector_norm` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$2::vector_norm` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$3::default` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$42::clone` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$46::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$48::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$5::hint_capacity` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$5::receive_points` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::impl$78::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/curve/mod.rs` | `utils::curve::smooth_in_place` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::Aabb2::from_points` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::Aabb2::overlaps_segment` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::Aabb3::from_center_extent` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::Aabb3::from_points` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::intersection_aabb_segment2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/aabb.rs` | `utils::geometry::aabb::intersection_aabb_segment3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/barycentric.rs` | `utils::geometry::barycentric::barycentric2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Circle2d::as_curve2` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Circle2d::as_curve2_range` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Circle2d::as_curve3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Circle2d::lerp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Circle2d::yaw_radians` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::Segment2d::new` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::_::impl$0::serialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::impl$3::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::impl$6::eq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/circle.rs` | `utils::geometry::circle::is_default` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/intersect.rs` | `utils::geometry::intersect::impl$1::hash` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/line_relation.rs` | `utils::geometry::line_relation::LineInterval::cross` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/line_relation.rs` | `utils::geometry::line_relation::LineInterval::div` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/line_relation.rs` | `utils::geometry::line_relation::LineInterval::t_coord_to_point` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/position_from_mesh_uvs.rs` | `utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/ray.rs` | `utils::geometry::ray::Ray::from_mouse_ss` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::_get_normal_at_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::as_curve3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::as_points3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::as_transformation_and_size` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::circular_slice` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::closest_edge_normal` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::expand_along_edge` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_closest_edge_id` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_corner_unnormal` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_normal_at_u` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::half_dimensions` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::intersects` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::intersects_circle` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::lerp` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::local_aabb_of` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::orientation_mat2` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::orientation_mat3` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge` | `analyzed_only` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle3d::center` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle3d::height` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle3d::length` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle3d::rotate_projective_xz` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::Rectangle3d::width` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::_::impl$0::serialize` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::impl$10::eq` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::impl$2::from` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::impl$3::clone` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::impl$6::clone` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/rectangle.rs` | `utils::geometry::rectangle::impl$7::fmt` | `untouched` / `private_or_generated_schema_trait_policy_requires_classification` |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | `utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | `utils::geometry::triangle_triangle::cross_intersect::intersect_plane` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/mod.rs` | `utils::geometry::ws_to_cs` | `untouched` / `private_game_business_interface` |
| `crates/utils/src/geometry/mod.rs` | `utils::geometry::ws_to_ss` | `untouched` / `private_game_business_interface` |

## 验证与输入快照

当前 `cargo test -p utils --locked --offline`：57/57；`cargo clippy -p utils --all-targets --locked --offline -- -D warnings` 通过。测试总数包含并行其他 owner 的新增测试，不全部记为本 Review 新增。精确源文件哈希、原入口类别、fixture 哈希与功能限制见 [validation.json](validation.json)，机器可读逐接口缺口见 [review.json](review.json)。本报告没有做原游戏 consumer/GPU/整帧/整游戏性能验收。
