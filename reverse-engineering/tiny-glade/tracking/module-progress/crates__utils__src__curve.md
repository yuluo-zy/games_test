# crates/utils/src/curve

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：115/29/86。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source_with_pending_trait_schema_classification。

B原接口台账：115项已观察，29项已有部分业务分析，86项未触及；具体状态见逐接口证据。

## 接线与验证

接线：candidate_crate_exported_original_game_consumer_not_accepted。

原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。

## 已恢复的函数和类型范围

- utils::curve::Curve2<CSem>::project_point
- utils::curve::Curve2<CSem>::to_curve3
- utils::curve::Curve<T,CSem>::circular_slice
- utils::curve::Curve<T,CSem>::crashy_new_from_points
- utils::curve::Curve<T,CSem>::crashy_slice
- utils::curve::Curve<T,CSem>::get_approx_coord_from_pos
- utils::curve::Curve<T,CSem>::get_approx_u_from_pos
- utils::curve::Curve<T,CSem>::get_coord_at_u
- utils::curve::Curve<T,CSem>::get_pos_at_coord
- utils::curve::Curve<T,CSem>::get_pos_at_u
- utils::curve::Curve<T,CSem>::get_tangent_at_coord
- utils::curve::Curve<T,CSem>::get_tangent_at_u
- utils::curve::Curve<T,CSem>::reverse
- utils::curve::Curve<T,CSem>::slice_into
- utils::curve::Curve<T,CSem>::try_new_from_points
- utils::curve::Curve<T,CSem>::try_slice
- utils::curve::CurveInternalCoord<CSem>::new_for_point_count
- utils::curve::curve_u::CurveU<CSem>::new
- utils::curve::curve_u_circular::CurveUCircular::add
- utils::curve::curve_u_circular::CurveUCircular::distance
- utils::curve::curve_u_circular::CurveUCircular::new
- utils::curve::curve_u_circular::CurveUCircular::subtract
- utils::curve::curve_u_circular::CurveUCircular::wrap_around
- utils::curve::curve_u_circular::CurveUCircular::wrapping_contains
- utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp
- utils::curve::curve_u_circular::impl$3::from

## 明确缺口

- 尚未分析的接口逐项列于 INTERFACE_PROGRESS.md
- 泛型策略及边界输入尚未全部对照
- 原游戏消费者链尚未验收

下一闭包：按原接口缺口补充业务分析、复用原依赖实现并对照修正。

## 证据

- [evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json)
- [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json)
- [evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/utils/src/curve/curve_slice.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/curve/curve_u.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs)：B_PDB_definition_source_and_reviewed_interface_contract | utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/curve/curve_u_circular.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/curve/mod.rs` | A,B,C | [reconstruction/roof/surface.rs](../../reconstruction/roof/surface.rs)：reviewed_partial_module_mapping<br>[reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs)：B_PDB_definition_source_and_reviewed_interface_contract<br>[reconstruction/shared-foundation/crates/utils/src/curve/mod.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/mod.rs)：B_PDB_definition_source_and_reviewed_interface_contract | utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；限定Curve2 CPU链：[evidence/roof-surface/verification.json](../../evidence/roof-surface/verification.json)<br>仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json)<br>仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/roof/surface.rs

[reconstruction/roof/surface.rs](../../reconstruction/roof/surface.rs)

SHA256：`132b17182082410908cae6a0d897638eeebb0e8095183d124767ec886428bbcd`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `f` | 7 | 未由扫描证明；以证据限定范围为准 |
| `put` | 8 | 未由扫描证明；以证据限定范围为准 |
| `write_observed` | 19 | 未由扫描证明；以证据限定范围为准 |
| `section` | 33 | 未由扫描证明；以证据限定范围为准 |
| `profile_exponent` | 40 | 未由扫描证明；以证据限定范围为准 |
| `get_roof_shape` | 46 | 未由扫描证明；以证据限定范围为准 |
| `eave_length_ws` | 76 | 未由扫描证明；以证据限定范围为准 |
| `sse_max` | 81 | 未由扫描证明；以证据限定范围为准 |
| `sse_min` | 82 | 未由扫描证明；以证据限定范围为准 |
| `height` | 86 | 未由扫描证明；以证据限定范围为准 |
| `profile_curve_normalized` | 105 | 未由扫描证明；以证据限定范围为准 |
| `profile_curve_ws_points` | 116 | 未由扫描证明；以证据限定范围为准 |
| `try_new` | 128 | 未由扫描证明；以证据限定范围为准 |
| `coord_at_u` | 148 | 未由扫描证明；以证据限定范围为准 |
| `pos_at_u` | 158 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_u` | 162 | 未由扫描证明；以证据限定范围为准 |
| `normalized_profile_curve` | 168 | 未由扫描证明；以证据限定范围为准 |
| `world_profile_curve` | 171 | 未由扫描证明；以证据限定范围为准 |
| `main` | 177 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L190：_=>panic!("unknown operation")
### reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs

[reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs)

SHA256：`e5ee88978bc70ab389af2425a453d85898300010266cf66f88c005197c76f24c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `len` | 10 | 未由扫描证明；以证据限定范围为准 |
| `point` | 11 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 12 | 未由扫描证明；以证据限定范围为准 |
| `iter` | 13 | 未由扫描证明；以证据限定范围为准 |
| `len` | 18 | 未由扫描证明；以证据限定范围为准 |
| `point` | 19 | 未由扫描证明；以证据限定范围为准 |
| `iter` | 20 | 未由扫描证明；以证据限定范围为准 |
| `len` | 28 | 未由扫描证明；以证据限定范围为准 |
| `point` | 29 | 未由扫描证明；以证据限定范围为准 |
| `iter` | 37 | 未由扫描证明；以证据限定范围为准 |
| `identity` | 41 | 未由扫描证明；以证据限定范围为准 |
| `from_point_range` | 51 | 未由扫描证明；以证据限定范围为准 |
| `from_coord_range` | 68 | 未由扫描证明；以证据限定范围为准 |
| `from_curve_range` | 76 | 未由扫描证明；以证据限定范围为准 |
| `start` | 81 | 未由扫描证明；以证据限定范围为准 |
| `end` | 82 | 未由扫描证明；以证据限定范围为准 |
| `remap` | 83 | 未由扫描证明；以证据限定范围为准 |
| `remap_2d` | 86 | 未由扫描证明；以证据限定范围为准 |
| `iter_coords` | 90 | 未由扫描证明；以证据限定范围为准 |
| `mapped_point` | 97 | 未由扫描证明；以证据限定范围为准 |
| `sample_at_coord` | 98 | 未由扫描证明；以证据限定范围为准 |
| `measure_length` | 101 | 未由扫描证明；以证据限定范围为准 |
| `extend_right` | 105 | 未由扫描证明；以证据限定范围为准 |
| `extend_left` | 106 | 未由扫描证明；以证据限定范围为准 |
| `extend_right_until` | 108 | 未由扫描证明；以证据限定范围为准 |
| `extend_left_until` | 127 | 未由扫描证明；以证据限定范围为准 |
| `slice_into` | 149 | 未由扫描证明；以证据限定范围为准 |
| `try_slice` | 174 | 未由扫描证明；以证据限定范围为准 |
| `crashy_slice` | 179 | 未由扫描证明；以证据限定范围为准 |
| `circular_slice` | 183 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs

[reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs)

SHA256：`c6494ac34a52b9b457b51ad2931e894f067f3cbe809a7fff31325dd8e90d3410`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 16 | 未由扫描证明；以证据限定范围为准 |
| `get` | 21 | 未由扫描证明；以证据限定范围为准 |
| `clamp` | 22 | 未由扫描证明；以证据限定范围为准 |
| `serialize` | 28 | 未由扫描证明；以证据限定范围为准 |
| `deserialize` | 31 | 未由扫描证明；以证据限定范围为准 |
| `expecting` | 35 | 未由扫描证明；以证据限定范围为准 |
| `visit_f64` | 36 | 未由扫描证明；以证据限定范围为准 |
| `new` | 46 | 未由扫描证明；以证据限定范围为准 |
| `get` | 53 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs

[reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/curve_u_circular.rs)

SHA256：`28d2222378fc125db45cf5537f23fe255bb398daf9c223ff15e5c11b3c9b38db`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `clone` | 15 | 未由扫描证明；以证据限定范围为准 |
| `eq` | 18 | 未由扫描证明；以证据限定范围为准 |
| `new` | 22 | 未由扫描证明；以证据限定范围为准 |
| `wrap_around` | 26 | 未由扫描证明；以证据限定范围为准 |
| `get` | 32 | 未由扫描证明；以证据限定范围为准 |
| `add` | 34 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 35 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 37 | 未由扫描证明；以证据限定范围为准 |
| `forward_end` | 40 | 未由扫描证明；以证据限定范围为准 |
| `wrapping_lerp` | 44 | 未由扫描证明；以证据限定范围为准 |
| `wrapping_contains` | 49 | 未由扫描证明；以证据限定范围为准 |
| `from` | 60 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/curve/mod.rs

[reconstruction/shared-foundation/crates/utils/src/curve/mod.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/mod.rs)

SHA256：`da3c76441384d4fd997ec725d36a0941fa4b2712a58519222417f8f448150d8f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `metric_squared` | 31 | 未由扫描证明；以证据限定范围为准 |
| `normalized` | 32 | 未由扫描证明；以证据限定范围为准 |
| `projected` | 33 | 未由扫描证明；以证据限定范围为准 |
| `metric_squared` | 36 | 未由扫描证明；以证据限定范围为准 |
| `normalized` | 39 | 未由扫描证明；以证据限定范围为准 |
| `projected` | 42 | 未由扫描证明；以证据限定范围为准 |
| `metric_squared` | 47 | 未由扫描证明；以证据限定范围为准 |
| `normalized` | 50 | 未由扫描证明；以证据限定范围为准 |
| `projected` | 53 | 未由扫描证明；以证据限定范围为准 |
| `segment_length` | 58 | 未由扫描证明；以证据限定范围为准 |
| `segment_length` | 61 | 未由扫描证明；以证据限定范围为准 |
| `segment_length` | 67 | 未由扫描证明；以证据限定范围为准 |
| `segment_length` | 72 | 未由扫描证明；以证据限定范围为准 |
| `new_for_point_count` | 93 | 未由扫描证明；以证据限定范围为准 |
| `clamp01` | 107 | 未由扫描证明；以证据限定范围为准 |
| `try_new_from_points` | 117 | 未由扫描证明；以证据限定范围为准 |
| `parameters_for_points` | 124 | 未由扫描证明；以证据限定范围为准 |
| `try_new` | 154 | 未由扫描证明；以证据限定范围为准 |
| `try_new_from_points_remove_duplicates` | 159 | 未由扫描证明；以证据限定范围为准 |
| `crashy_new_from_points` | 163 | 未由扫描证明；以证据限定范围为准 |
| `get_coord_at_u` | 167 | 未由扫描证明；以证据限定范围为准 |
| `coord_at_u` | 191 | 未由扫描证明；以证据限定范围为准 |
| `get_pos_at_coord` | 195 | 未由扫描证明；以证据限定范围为准 |
| `get_pos_at_coord_unchecked` | 198 | 未由扫描证明；以证据限定范围为准 |
| `get_unnormalized_tangent_at_coord` | 202 | 未由扫描证明；以证据限定范围为准 |
| `get_unnormalized_central_tangent_at_coord` | 206 | 未由扫描证明；以证据限定范围为准 |
| `get_pos_at_u` | 209 | 未由扫描证明；以证据限定范围为准 |
| `pos_at_u` | 212 | 未由扫描证明；以证据限定范围为准 |
| `get_tangent_at_coord` | 215 | 未由扫描证明；以证据限定范围为准 |
| `get_tangent_at_u` | 218 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_u` | 221 | 未由扫描证明；以证据限定范围为准 |
| `get_approx_coord_from_pos` | 226 | 未由扫描证明；以证据限定范围为准 |
| `get_approx_u_from_pos` | 246 | 未由扫描证明；以证据限定范围为准 |
| `reverse` | 254 | 未由扫描证明；以证据限定范围为准 |
| `try_extend` | 262 | 未由扫描证明；以证据限定范围为准 |
| `translate` | 269 | 未由扫描证明；以证据限定范围为准 |
| `smooth` | 272 | 未由扫描证明；以证据限定范围为准 |
| `smooth_weighted` | 278 | 未由扫描证明；以证据限定范围为准 |
| `smooth_in_place` | 293 | 未由扫描证明；以证据限定范围为准 |
| `get_coord_at_2d_ws_distance` | 301 | 未由扫描证明；以证据限定范围为准 |
| `get_coord_at_wall_space` | 306 | 未由扫描证明；以证据限定范围为准 |
| `is_on_right_side` | 311 | 未由扫描证明；以证据限定范围为准 |
| `smooth_in_place` | 322 | 未由扫描证明；以证据限定范围为准 |
| `reinterpret_as_curve2` | 339 | 未由扫描证明；以证据限定范围为准 |
| `try_to_curve2` | 343 | 未由扫描证明；以证据限定范围为准 |
| `crashy_to_curve2` | 346 | 未由扫描证明；以证据限定范围为准 |
| `into_curve3` | 350 | 未由扫描证明；以证据限定范围为准 |
| `to_curve3` | 355 | 未由扫描证明；以证据限定范围为准 |
| `project_point` | 367 | 未由扫描证明；以证据限定范围为准 |
| `length` | 372 | 未由扫描证明；以证据限定范围为准 |
| `point_u` | 375 | 未由扫描证明；以证据限定范围为准 |
| `position_at_u` | 378 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_u` | 381 | 未由扫描证明；以证据限定范围为准 |
| `approximate_u` | 384 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/curve/resample.rs

[reconstruction/shared-foundation/crates/utils/src/curve/resample.rs](../../reconstruction/shared-foundation/crates/utils/src/curve/resample.rs)

SHA256：`1186901e2a4b2b8cb069fb92139e2f29efdc8983b99f81311ce64c3f3846d46a`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `count_from_spacing` | 3 | 未由扫描证明；以证据限定范围为准 |
| `try_resample` | 11 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample` | 15 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_approx_pt_count` | 16 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_approx_pt_count` | 58 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_per_segment` | 63 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_per_segment` | 76 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_remap` | 79 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_approx_pt_count_remap` | 83 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_remap_per_segment` | 87 | 未由扫描证明；以证据限定范围为准 |
| `try_resample_remap_per_segment_2` | 92 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_remap` | 106 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_approx_pt_count_remap` | 109 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_remap_per_segment` | 112 | 未由扫描证明；以证据限定范围为准 |
| `crashy_resample_remap_per_segment_2` | 115 | 未由扫描证明；以证据限定范围为准 |
