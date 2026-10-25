# components / curve / geometry：按原 B 接口核对

合并 244 条 public 和原 B 骨架 **407 条函数定义元数据**，目前已知 **326 个不同接口/待归属 trait-schema 策略项**。泛型 hash 实例合并；不同 owner 即使同地址仍分开。**inline 全集未知，不能作为完整分母。**

| 状态 | 已知接口数 | 定义 |
|---|---:|---|
| `untouched` | 206 | 完全未开始业务分析（只有名字/反编译也算这里） |
| `analyzed_only` | 7 | 已分析部分业务，尚无实现 |
| `implemented_static` | 82 | 实现并静态核对，尚无对应原入口对照 |
| `native_compared` | 31 | 已有原指令对照；变体、投影、容差仍分别限定 |
| `integrated` | 0 | 原游戏消费者闭包已验证 |

其中 44 个 untouched 接口虽有伪代码，但未形成业务分析，不能计已完成。

## 目录分布

| 原文件 | 已知接口 | 未触及 | 只分析 | 静态实现 | 原码对照 | 原消费者接线 |
|---|---:|---:|---:|---:|---:|---:|
| `crates/utils/src/components/transform.rs` | 15 | 13 | 0 | 0 | 2 | 0 |
| `crates/utils/src/curve/mod.rs` | 90 | 70 | 3 | 9 | 8 | 0 |
| `crates/utils/src/curve/curve_u.rs` | 17 | 16 | 0 | 1 | 0 | 0 |
| `crates/utils/src/curve/curve_u_circular.rs` | 9 | 9 | 0 | 0 | 0 | 0 |
| `crates/utils/src/curve/curve_slice.rs` | 17 | 17 | 0 | 0 | 0 | 0 |
| `crates/utils/src/geometry/aabb.rs` | 29 | 6 | 0 | 21 | 2 | 0 |
| `crates/utils/src/geometry/barycentric.rs` | 2 | 1 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/circle.rs` | 38 | 18 | 1 | 15 | 4 | 0 |
| `crates/utils/src/geometry/intersect.rs` | 10 | 1 | 0 | 7 | 2 | 0 |
| `crates/utils/src/geometry/line_relation.rs` | 5 | 3 | 0 | 1 | 1 | 0 |
| `crates/utils/src/geometry/mod.rs` | 2 | 2 | 0 | 0 | 0 | 0 |
| `crates/utils/src/geometry/plane.rs` | 1 | 0 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/position_from_mesh_uvs.rs` | 2 | 1 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/ray.rs` | 2 | 1 | 0 | 1 | 0 | 0 |
| `crates/utils/src/geometry/rectangle.rs` | 77 | 44 | 3 | 24 | 6 | 0 |
| `crates/utils/src/geometry/split_heightmap.rs` | 1 | 0 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | 5 | 4 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs` | 1 | 0 | 0 | 0 | 1 | 0 |
| `crates/utils/src/geometry/triangle_triangle/utils.rs` | 3 | 0 | 0 | 3 | 0 | 0 |

326项中，明确私有函数/方法 260 项；66 项 trait/schema 策略尚待确认归属或生成方式，保留在分母，不以 trait 名字排除。

`curve_u_circular.rs` 已纳入9条 inline元数据（含待归属trait）；不能因没有public地址忽略。私有closure单列，不算公共/方法接口分母。C 专有 brace 不计入 B。

原 `curve_slice.rs` 的17项 CurveSlice/CurvePoints API 仍完全未触及；候选文件里实现的4个裁切方法原本属于 `Curve`，故记在原 `curve/mod.rs`，不能从候选文件同名推定原CurveSlice类已恢复。

## 策略不能混算

- Curve2：XY 长度、XY 重复点和 XY 近点；构造、查询、裁切已有原入口对照。
- Curve3XYZ：XYZ 长度、全 XYZ 重复点；本次原入口对照仅构造及 points_u，3D 查询不能借 Curve2 测试算完成。
- Curve3XZ：XZ 长度仍检查全 XYZ 重复点；同样仅构造及 points_u 已原入口对照。
- **Curve3Thresholded**：原骨架明确存在 CurveCoordSemanticThresholded3D，阈值度量策略尚未恢复，不能当XYZ或XZ处理。
- 其他 generic hash 的 T/CSem 尚未全部解出，逐地址保留，不能把全部泛型实例自动视为已验收。
- JSON每个合并Curve接口有单独strategy_variants子表；以具体变体的实施/对照状态为准，不把一次Curve2测试复制给3D。

## 每个接口

| 原接口（合并 hash） | 状态 | 原地址 | 恢复实现 | 对照限定 / 缺口 |
|---|---|---|---|---|
| `utils::components::transform::Transform::abs_diff_eq` | `native_compared` | `0x140c9d0d0` | `reconstruction/shared-foundation/crates/utils/src/components/transform.rs` | component/tolerance/NaN cases / bool |
| `utils::components::transform::Transform::compute_affine` | `untouched` | `0x1407a45b0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::compute_matrix` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::forward` | `untouched` | `0x1407a4700` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::from_scale` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::from_translation` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::from_translation_scale` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::identity` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::Transform::look_at` | `native_compared` | `0x140c9ce00` | `reconstruction/shared-foundation/crates/utils/src/components/transform.rs` | complete_declared_business_input_case / bits |
| `utils::components::transform::Transform::with_translation` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::impl$1::default` | `untouched` | `0x140c9d180` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::impl$2::clone` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::impl$4::clone_behavior` | `untouched` | `0x140c9da40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::impl$4::register_required_components` | `untouched` | `0x140c9d1c0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::components::transform::impl$7::eq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve2<CSem>::project_point` | `implemented_static` | `0x14146c3c0` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve2<CSem>::to_curve3` | `implemented_static` | `0x140155120`, `0x14146c470` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve3XZ::into_curve3` | `untouched` | `0x140c9cc60` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::_deprecated_new_empty` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::bounding_box_2d` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::calculate_points_u_and_length` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::circular_slice` | `native_compared` | `0x1411f8750`, `0x141467300`, `0x1414677f0`, `0x141cd5a90` | `reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs` | Curve2 circular_slice / bits |
| `utils::curve::Curve<T,CSem>::crashy_lerp` | `untouched` | `0x141466a40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_new_from_points` | `implemented_static` | `0x1411fa190`, `0x1411fa3c0`, `0x14126a1e0`, `0x1421978d0` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::crashy_resample` | `untouched` | `0x1411f8ed0`, `0x1411f97b0`, `0x141467fa0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count` | `untouched` | `0x14146a830` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count_remap` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_per_segment` | `untouched` | `0x1411fa6b0`, `0x14146a2a0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_remap` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment` | `untouched` | `0x14146afe0`, `0x14146b5f0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment_2` | `untouched` | `0x14146bb70` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::crashy_slice` | `implemented_static` | `0x1411f8640`, `0x141466bb0` | `reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::crashy_to_curve2` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::end_pos` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_approx_coord_from_pos` | `native_compared` | `0x140153bc0`, `0x1411fa450`, `0x14126a450`, `0x14126a6b0`, `0x141469e00`, `0x14146a060`, `0x141cd64a0` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve2 nearest coord / bits |
| `utils::curve::Curve<T,CSem>::get_approx_u_from_pos` | `native_compared` | `0x1411f9ea0`, `0x14126a040`, `0x14126a110`, `0x141469950`, `0x141469a20`, `0x141469af0`, `0x141940a40` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve2 nearest u / bits |
| `utils::curve::Curve<T,CSem>::get_coord_at_2d_ws_distance` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_coord_at_u` | `native_compared` | `0x140153380`, `0x1411f8c10`, `0x141269710`, `0x141467ce0`, `0x141940780`, `0x141cd5f80`, `0x142197610` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve2 coord / bits |
| `utils::curve::Curve<T,CSem>::get_coord_at_wall_space` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_curve_segment_from_u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_pos_at_coord` | `implemented_static` |  | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::get_pos_at_coord_unchecked` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_pos_at_u` | `native_compared` | `0x1414671d0`, `0x141467270`, `0x142289c40` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve2 position / bits |
| `utils::curve::Curve<T,CSem>::get_tangent_at_coord` | `implemented_static` | `0x141269fa0` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::get_tangent_at_u` | `native_compared` | `0x141468ee0`, `0x141468f90` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve2 tangent / bits |
| `utils::curve::Curve<T,CSem>::get_u_at_coord` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_unnormalized_central_tangent_at_coord` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::get_unnormalized_tangent_at_coord` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::is_empty` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::is_on_right_side` | `analyzed_only` | `0x141469000` | — | Reviewed nearest-segment and normalized cross-side query; no implementation. |
| `utils::curve::Curve<T,CSem>::is_valid` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::length_ws` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::point_count` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::point_u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::points_with_u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::reinterpret_as_curve2` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::reverse` | `implemented_static` | `0x14146c190` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::segments` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::segments_with_u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::slice_into` | `implemented_static` | `0x1411f8130`, `0x141465da0`, `0x141466300`, `0x141cd5530` | `reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::Curve<T,CSem>::smooth` | `untouched` | `0x140154490` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::smooth_in_place` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::smooth_weighted` | `untouched` | `0x141468880` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::start_pos` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::translate` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_extend` | `analyzed_only` | `0x141466860` | — | Original mutates/appends the owned point vector then rebuilds with duplicate rejection; no recovered mutating API yet. |
| `utils::curve::Curve<T,CSem>::try_new_from_points` | `native_compared` | `0x140153640`, `0x140153900`, `0x1411f9920`, `0x1411f9be0`, `0x1412699d0`, `0x141269ce0`, `0x141469230`, `0x141469430`, `0x1414696f0`, `0x1419401b0`, `0x141940470`, `0x141cd6240` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Curve3XYZ constructor / bits; Curve3XZ constructor / bits; Curve2 constructor / bits |
| `utils::curve::Curve<T,CSem>::try_new_from_points_remove_duplicates` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_resample` | `untouched` | `0x140152b00`, `0x141268e90` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count` | `analyzed_only` | `0x140153de0` | — | Reviewed spacing/retained-feature and endpoint append paths; final algorithm not reconstructed. |
| `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count_remap` | `untouched` | `0x1411fac00` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_resample_remap` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_resample_remap_per_segment_2` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::Curve<T,CSem>::try_slice` | `native_compared` | `0x140154b60`, `0x14126a8f0` | `reconstruction/shared-foundation/crates/utils/src/curve/curve_slice.rs` | Curve2 try_slice / bits |
| `utils::curve::Curve<T,CSem>::try_to_curve2` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::CurveInternalCoord<CSem>::i0` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::CurveInternalCoord<CSem>::i0_t` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::CurveInternalCoord<CSem>::i1` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::CurveInternalCoord<CSem>::new` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::CurveInternalCoord<CSem>::new_for_point_count` | `implemented_static` | `0x1411fb2b0`, `0x14126af00`, `0x14261ef90` | `reconstruction/shared-foundation/crates/utils/src/curve/mod.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::CurveInternalCoord<CSem>::new_unchecked` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize::impl$0::visit_str` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize::impl$1::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize::impl$2::expecting` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize::impl$2::visit_map` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::_::impl$0::serialize` | `untouched` | `0x1401528c0`, `0x1401529e0`, `0x1414659e0`, `0x141465b60`, `0x141465c80` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve2_serialized_as_curve3::deserialize` | `untouched` | `0x141285770` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve2_serialized_as_curve3::serialize` | `untouched` | `0x141285a70` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurvePoints::is_empty` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::end` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left` | `untouched` | `0x141206c50`, `0x141206ed0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left_until` | `untouched` | `0x141207310` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right_until` | `untouched` | `0x1412075e0`, `0x141207a10`, `0x141207c10`, `0x1412898e0`, `0x1422e1e40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_coord_range` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_curve_range` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_point_range` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::iter_coords` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::measure_length` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap_2d` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::start` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::impl$0::iter` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::impl$1::iter` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_slice::impl$1::len` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveU<CSem>::clamp` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveU<CSem>::lerp` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveU<CSem>::max` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveU<CSem>::min` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveU<CSem>::new` | `implemented_static` | `0x1408e1630`, `0x141208a00` | `reconstruction/shared-foundation/crates/utils/src/curve/curve_u.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::curve::curve_u::CurveU<CSem>::u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveUBoundless::clamp` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveUBoundless::is_within_01_range` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveUBoundless::new` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::CurveUBoundless::u` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$0::serialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$12::eq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$13::partial_cmp` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$2::expecting` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$2::visit_f64` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$3::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u::impl$8::fmt` | `untouched` | `0x140155380`, `0x1411fb490`, `0x14126b0e0`, `0x14146c780`, `0x14146c800`, `0x141cd6700` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::add` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::distance` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::new` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::subtract` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::wrap_around` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::wrapping_contains` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::impl$3::from` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::curve_u_circular::impl$8::eq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$0::vector_norm` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$11::sub` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$12::sample_at_coord` | `untouched` | `0x141208d10` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$13::smooth` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$14::smooth` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$1::vector_norm` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$2::vector_norm` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$3::default` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$42::clone` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$46::fmt` | `untouched` | `0x1405b88c0`, `0x140a30120`, `0x140a52e60`, `0x140a787d0`, `0x1411fb3e0`, `0x14126b030`, `0x14146c6d0`, `0x141940d30`, `0x142197b40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$48::fmt` | `untouched` | `0x140c9cc90` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$5::hint_capacity` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$5::receive_points` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::impl$78::fmt` | `untouched` | `0x141207fb0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::curve::smooth_in_place` | `untouched` | `0x141465260` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::Aabb2::area` | `implemented_static` | `0x141240120` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::center` | `implemented_static` | `0x1412400c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::contains` | `implemented_static` | `0x141240140` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::expanded` | `implemented_static` | `0x141240300` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::from_center_dims` | `implemented_static` | `0x14123fff0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::from_center_radius` | `implemented_static` | `0x141240070` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::from_points` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::Aabb2::half_dims` | `implemented_static` | `0x1412400f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::height` | `implemented_static` | `0x1412402f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::intersect_ray` | `implemented_static` | `0x1412401c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::intersects` | `implemented_static` | `0x141240170` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::max_extent` | `implemented_static` | `0x1412402b0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb2::overlaps_segment` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::Aabb2::width` | `implemented_static` | `0x1412402e0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::center` | `implemented_static` | `0x141240420` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::classify_ray_intersection` | `native_compared` | `0x141240860` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | classify ray / bits |
| `utils::geometry::aabb::Aabb3::expand` | `implemented_static` | `0x1412403c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::expand_to_contain_point` | `implemented_static` | `0x141240330` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::extent` | `implemented_static` | `0x141240460` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::from_center_extent` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::Aabb3::from_points` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::Aabb3::inclusive_contains_point` | `implemented_static` | `0x1412404f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::intersect_segment` | `native_compared` | `0x141240570` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | intersect segment / bits |
| `utils::geometry::aabb::Aabb3::scaled` | `implemented_static` | `0x141240490` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::Aabb3::translated` | `implemented_static` | `0x1412404c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::RayBoxIntersection::is_hit` | `implemented_static` | `0x141240b10` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::aabb::intersection_aabb_segment2` | `untouched` | `0x141240b30` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::intersection_aabb_segment3` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::aabb::is_inside_bbx` | `implemented_static` | `0x14123ffc0` | `reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::barycentric::barycentric` | `native_compared` | `0x1416c2f20` | `reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs` | 3 weights / bits |
| `utils::geometry::barycentric::barycentric2` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::Circle2d::aabb` | `implemented_static` | `0x140c98140` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::area` | `implemented_static` | `0x140c97580` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::as_curve2` | `analyzed_only` | `0x140c97cf0` | — | Reviewed collect→curve build call; whether collection closes the final segment remains unresolved. |
| `utils::geometry::circle::Circle2d::as_curve2_range` | `untouched` | `0x140c97f50` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::Circle2d::as_curve3` | `untouched` | `0x140c97e10` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::Circle2d::as_points2` | `implemented_static` | `0x140c97bc0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::as_points2_ptcount` | `implemented_static` | `0x140c97ba0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::as_points3` | `implemented_static` | `0x140c97c50` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::closest_point_on_perimeter` | `implemented_static` | `0x140c97b40` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::contains_point` | `implemented_static` | `0x140c980f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::get_closest_u_from_pos` | `native_compared` | `0x140c976f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | closest u / unnormalizable 0 / bits |
| `utils::geometry::circle::Circle2d::get_normal_at_u` | `native_compared` | `0x140c978e0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | normal / bits |
| `utils::geometry::circle::Circle2d::get_pos_at_u` | `native_compared` | `0x140c975a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | position / bits |
| `utils::geometry::circle::Circle2d::get_tangent_at_u` | `native_compared` | `0x140c97660` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | tangent / bits |
| `utils::geometry::circle::Circle2d::intersect_circle` | `implemented_static` | `0x140c97a40` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::intersect_ray` | `implemented_static` | `0x140c97980` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::is_point_inside` | `implemented_static` | `0x140c97b10` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::lerp` | `untouched` | `0x140c98170` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::Circle2d::new` | `implemented_static` | `0x140c97540` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::new_rotated` | `implemented_static` | `0x140c97560` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::perimeter` | `implemented_static` | `0x140c97970` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::signed_distance` | `implemented_static` | `0x140c98120` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::with_expanded_diameter` | `implemented_static` | `0x140c980c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::circle::Circle2d::yaw_radians` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::Segment2d::new` | `untouched` | `0x140c97560` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting` | `untouched` | `0x140c98340` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes` | `untouched` | `0x1416e6e60` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting` | `untouched` | `0x140c98360` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::_::impl$0::serialize` | `untouched` | `0x1403d8a70`, `0x1416e7940` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::impl$3::fmt` | `untouched` | `0x1409a43e0`, `0x14165bd40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::impl$6::eq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::circle::is_default` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::intersect::Sphere::as_f32x4` | `implemented_static` | `0x140dc0a90` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::SphereIntersection::closest_intersection_distance` | `implemented_static` | `0x14123b200` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::Triangle::aabb` | `implemented_static` | `0x14123ae50` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::Triangle::intersect_triangle` | `implemented_static` | `0x14123af40` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::Triangle::new` | `implemented_static` | `0x14123ad70` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::Triangle::normal` | `implemented_static` | `0x14123ada0` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::Triangle::points_mut` | `implemented_static` | `0x14067dd80` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::intersect::impl$1::hash` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::intersect::ray_sphere_intersection` | `native_compared` | `0x14123b230` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | tangent/negative root/non-unit ray / bits |
| `utils::geometry::intersect::ray_triangle_intersection` | `native_compared` | `0x14123afa0` | `reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs` | 2-sided boundaries and epsilon / bits |
| `utils::geometry::line_relation::LineInterval::cross` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::line_relation::LineInterval::div` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::line_relation::LineInterval::line_segment` | `implemented_static` | `0x14261f450` | `reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::line_relation::LineInterval::relate` | `native_compared` | `0x14261f470` | `reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs` | closed line intervals / bits |
| `utils::geometry::line_relation::LineInterval::t_coord_to_point` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::plane::Plane::intersect_ray` | `native_compared` | `0x1412803f0` | `reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs` | plane min_t / bits |
| `utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv` | `native_compared` | `0x1416f34a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs` | first UV triangle and boundaries / bits |
| `utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::ray::Ray::from_mouse_ss` | `untouched` | `0x14127cab0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::ray::Ray::xz` | `implemented_static` | `0x1408d98a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::_get_normal_at_u` | `untouched` | `0x140c99e30` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::aabb` | `implemented_static` | `0x140c9b680` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::area` | `implemented_static` | `0x140c99520` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::as_curve2` | `implemented_static` | `0x140c98cf0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::as_curve3` | `untouched` | `0x140c991f0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::as_points2` | `implemented_static` | `0x140c98ab0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned` | `implemented_static` | `0x140c98a50` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::as_points3` | `untouched` | `0x140c98b90` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::as_transformation_and_size` | `untouched` | `0x140c995c0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::circular_slice` | `untouched` | `0x140c9bd90` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points` | `untouched` | `0x1412b1b60` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::closest_edge_normal` | `untouched` | `0x140c9a8f0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter` | `untouched` | `0x140c9ae20` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::contains_point` | `implemented_static` | `0x140c9a630` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::contains_point_padding` | `implemented_static` | `0x140c9a6a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge` | `analyzed_only` | `0x140c9a750` | — | Reviewed inverse footprint and hypot branch; not implemented. |
| `utils::geometry::rectangle::Rectangle2d::edge_length` | `implemented_static` | `0x140c9ca50` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::expand_along_edge` | `untouched` | `0x140c9c620` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_closest_edge_id` | `untouched` | `0x140c9acb0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices` | `untouched` | `0x140c9a9d0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos` | `untouched` | `0x140c9b000` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_corner_normal` | `implemented_static` | `0x140c9a140` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::get_corner_unnormal` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id` | `untouched` | `0x140c9a210` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u` | `untouched` | `0x140c9acc0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_edge_normal` | `implemented_static` | `0x140c9a5a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::get_edge_pts` | `implemented_static` | `0x140c9a470` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::get_normal_at_u` | `untouched` | `0x140c99e10` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_normal_at_u_non_normalized_corners` | `analyzed_only` | `0x140c99e20` | — | Reviewed exact-corner/edge dispatch and transformed corner normal sums; no implementation. |
| `utils::geometry::rectangle::Rectangle2d::get_point` | `implemented_static` | `0x140c99600` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::get_point_u` | `implemented_static` | `0x140c996d0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::get_pos_at_u` | `native_compared` | `0x140c99800` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | position / bits |
| `utils::geometry::rectangle::Rectangle2d::half_dimensions` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::intersects` | `untouched` | `0x140c9b990` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::intersects_circle` | `untouched` | `0x140c9b6e0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::lerp` | `untouched` | `0x140c9c730` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::local_aabb_of` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::new` | `implemented_static` | `0x140c98980` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::new_axis_aligned` | `implemented_static` | `0x140c98900` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned` | `implemented_static` | `0x140c98890` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::orientation_mat2` | `untouched` | `0x140c99540` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::orientation_mat3` | `untouched` | `0x140c99570` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d` | `native_compared` | `0x140c9b420` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | SAT overlaps / bool |
| `utils::geometry::rectangle::Rectangle2d::perimeter` | `implemented_static` | `0x140c99530` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::signed_distance_to_closest_edge` | `analyzed_only` | `0x140c9a820` | — | Reviewed inverse footprint/hypot/inside branch; not implemented. |
| `utils::geometry::rectangle::Rectangle2d::try_as_curve2` | `implemented_static` | `0x140c98fa0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle2d::with_expanded_size` | `implemented_static` | `0x140c9a730` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::center` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle3d::contains_point` | `native_compared` | `0x140c985a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Rectangle3d contains / bits |
| `utils::geometry::rectangle::Rectangle3d::expand` | `native_compared` | `0x140c98620` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Rectangle3d expand / bits |
| `utils::geometry::rectangle::Rectangle3d::extent` | `native_compared` | `0x140c98660` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Rectangle3d extent / bits |
| `utils::geometry::rectangle::Rectangle3d::height` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle3d::length` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle3d::new` | `implemented_static` | `0x140c98380` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d` | `implemented_static` | `0x140c98580` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::points` | `native_compared` | `0x140c98690` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Rectangle3d points / bits |
| `utils::geometry::rectangle::Rectangle3d::rotate_projective_xz` | `untouched` | `0x140c984f0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::Rectangle3d::set_orientation_xz` | `implemented_static` | `0x140c98520` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::set_size` | `implemented_static` | `0x140c98530` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::translate` | `implemented_static` | `0x140c984c0` | `reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::rectangle::Rectangle3d::width` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting` | `untouched` | `0x140c9cc20` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes` | `untouched` | `0x14157b930` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str` | `untouched` | `0x14157ba70` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting` | `untouched` | `0x140c9cc40` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::_::impl$0::serialize` | `untouched` | `0x1400efde0`, `0x14157c4c0`, `0x14157c5e0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::impl$10::eq` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::impl$2::from` | `untouched` | `0x140c9cb90` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::impl$3::clone` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::impl$6::clone` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::rectangle::impl$7::fmt` | `untouched` | `0x140b00840`, `0x141559ce0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::split_heightmap::split_heightmap_by_threshold` | `native_compared` | `0x141fe0750` | `reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs` | RowTrimmerSink callback bodies intercepted; full spans unvalidated / event_projection |
| `utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::triangle_triangle::cross_intersect::cross_intersect` | `native_compared` | `0x1424181a0` | `reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | indirect private callee of triangles_intersect / ordered_float_tolerance_3e-5 |
| `utils::geometry::triangle_triangle::cross_intersect::intersect_plane` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone` | `untouched` |  | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive` | `untouched` | `0x142418dc0` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect` | `native_compared` | `0x141fdfd80` | `reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs` | Point/Segment/Coplanar/Disjoint; ordered endpoints / ordered_float_tolerance_3e-5 |
| `utils::geometry::triangle_triangle::utils::is_tri_degenerated` | `implemented_static` |  | `reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::triangle_triangle::utils::orient3d` | `implemented_static` |  | `reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::triangle_triangle::utils::permute_tri_right` | `implemented_static` |  | `reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs` | Business candidate/static review exists; original entry differential not yet executed. |
| `utils::geometry::ws_to_cs` | `untouched` | `0x14127c970` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |
| `utils::geometry::ws_to_ss` | `untouched` | `0x14127c950` | — | Only located/PDB/decompiled evidence; no business rule analysis or implementation. |

## 真正尚未触及的接口

- `utils::components::transform::Transform::compute_affine`
- `utils::components::transform::Transform::compute_matrix`
- `utils::components::transform::Transform::forward`
- `utils::components::transform::Transform::from_scale`
- `utils::components::transform::Transform::from_translation`
- `utils::components::transform::Transform::from_translation_scale`
- `utils::components::transform::Transform::identity`
- `utils::components::transform::Transform::with_translation`
- `utils::components::transform::impl$1::default`
- `utils::components::transform::impl$2::clone`
- `utils::components::transform::impl$4::clone_behavior`
- `utils::components::transform::impl$4::register_required_components`
- `utils::components::transform::impl$7::eq`
- `utils::curve::Curve3XZ::into_curve3`
- `utils::curve::Curve<T,CSem>::_deprecated_new_empty`
- `utils::curve::Curve<T,CSem>::bounding_box_2d`
- `utils::curve::Curve<T,CSem>::calculate_points_u_and_length`
- `utils::curve::Curve<T,CSem>::crashy_lerp`
- `utils::curve::Curve<T,CSem>::crashy_resample`
- `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count`
- `utils::curve::Curve<T,CSem>::crashy_resample_approx_pt_count_remap`
- `utils::curve::Curve<T,CSem>::crashy_resample_per_segment`
- `utils::curve::Curve<T,CSem>::crashy_resample_remap`
- `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment`
- `utils::curve::Curve<T,CSem>::crashy_resample_remap_per_segment_2`
- `utils::curve::Curve<T,CSem>::crashy_to_curve2`
- `utils::curve::Curve<T,CSem>::end_pos`
- `utils::curve::Curve<T,CSem>::get_coord_at_2d_ws_distance`
- `utils::curve::Curve<T,CSem>::get_coord_at_wall_space`
- `utils::curve::Curve<T,CSem>::get_curve_segment_from_u`
- `utils::curve::Curve<T,CSem>::get_pos_at_coord_unchecked`
- `utils::curve::Curve<T,CSem>::get_u_at_coord`
- `utils::curve::Curve<T,CSem>::get_unnormalized_central_tangent_at_coord`
- `utils::curve::Curve<T,CSem>::get_unnormalized_tangent_at_coord`
- `utils::curve::Curve<T,CSem>::is_empty`
- `utils::curve::Curve<T,CSem>::is_valid`
- `utils::curve::Curve<T,CSem>::length_ws`
- `utils::curve::Curve<T,CSem>::point_count`
- `utils::curve::Curve<T,CSem>::point_u`
- `utils::curve::Curve<T,CSem>::points_with_u`
- `utils::curve::Curve<T,CSem>::reinterpret_as_curve2`
- `utils::curve::Curve<T,CSem>::segments`
- `utils::curve::Curve<T,CSem>::segments_with_u`
- `utils::curve::Curve<T,CSem>::smooth`
- `utils::curve::Curve<T,CSem>::smooth_in_place`
- `utils::curve::Curve<T,CSem>::smooth_weighted`
- `utils::curve::Curve<T,CSem>::start_pos`
- `utils::curve::Curve<T,CSem>::translate`
- `utils::curve::Curve<T,CSem>::try_new_from_points_remove_duplicates`
- `utils::curve::Curve<T,CSem>::try_resample`
- `utils::curve::Curve<T,CSem>::try_resample_approx_pt_count_remap`
- `utils::curve::Curve<T,CSem>::try_resample_remap`
- `utils::curve::Curve<T,CSem>::try_resample_remap_per_segment_2`
- `utils::curve::Curve<T,CSem>::try_to_curve2`
- `utils::curve::CurveInternalCoord<CSem>::i0`
- `utils::curve::CurveInternalCoord<CSem>::i0_t`
- `utils::curve::CurveInternalCoord<CSem>::i1`
- `utils::curve::CurveInternalCoord<CSem>::new`
- `utils::curve::CurveInternalCoord<CSem>::new_unchecked`
- `utils::curve::_::impl$0::deserialize`
- `utils::curve::_::impl$0::deserialize::impl$0::visit_str`
- `utils::curve::_::impl$0::deserialize::impl$1::deserialize`
- `utils::curve::_::impl$0::deserialize::impl$2::expecting`
- `utils::curve::_::impl$0::deserialize::impl$2::visit_map`
- `utils::curve::_::impl$0::deserialize::impl$2::visit_seq`
- `utils::curve::_::impl$0::serialize`
- `utils::curve::curve2_serialized_as_curve3::deserialize`
- `utils::curve::curve2_serialized_as_curve3::serialize`
- `utils::curve::curve_slice::CurvePoints::is_empty`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::end`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_left_until`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::extend_right_until`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_coord_range`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_curve_range`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::from_point_range`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::iter_coords`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::measure_length`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::remap_2d`
- `utils::curve::curve_slice::CurveSlice<T,Points,CSem,RemapT,RemapF>::start`
- `utils::curve::curve_slice::impl$0::iter`
- `utils::curve::curve_slice::impl$1::iter`
- `utils::curve::curve_slice::impl$1::len`
- `utils::curve::curve_u::CurveU<CSem>::clamp`
- `utils::curve::curve_u::CurveU<CSem>::lerp`
- `utils::curve::curve_u::CurveU<CSem>::max`
- `utils::curve::curve_u::CurveU<CSem>::min`
- `utils::curve::curve_u::CurveU<CSem>::u`
- `utils::curve::curve_u::CurveUBoundless::clamp`
- `utils::curve::curve_u::CurveUBoundless::is_within_01_range`
- `utils::curve::curve_u::CurveUBoundless::new`
- `utils::curve::curve_u::CurveUBoundless::u`
- `utils::curve::curve_u::impl$0::serialize`
- `utils::curve::curve_u::impl$12::eq`
- `utils::curve::curve_u::impl$13::partial_cmp`
- `utils::curve::curve_u::impl$2::expecting`
- `utils::curve::curve_u::impl$2::visit_f64`
- `utils::curve::curve_u::impl$3::deserialize`
- `utils::curve::curve_u::impl$8::fmt`
- `utils::curve::curve_u_circular::CurveUCircular::add`
- `utils::curve::curve_u_circular::CurveUCircular::distance`
- `utils::curve::curve_u_circular::CurveUCircular::new`
- `utils::curve::curve_u_circular::CurveUCircular::subtract`
- `utils::curve::curve_u_circular::CurveUCircular::wrap_around`
- `utils::curve::curve_u_circular::CurveUCircular::wrapping_contains`
- `utils::curve::curve_u_circular::CurveUCircular::wrapping_lerp`
- `utils::curve::curve_u_circular::impl$3::from`
- `utils::curve::curve_u_circular::impl$8::eq`
- `utils::curve::impl$0::vector_norm`
- `utils::curve::impl$11::sub`
- `utils::curve::impl$12::sample_at_coord`
- `utils::curve::impl$13::smooth`
- `utils::curve::impl$14::smooth`
- `utils::curve::impl$1::vector_norm`
- `utils::curve::impl$2::vector_norm`
- `utils::curve::impl$3::default`
- `utils::curve::impl$42::clone`
- `utils::curve::impl$46::fmt`
- `utils::curve::impl$48::fmt`
- `utils::curve::impl$5::hint_capacity`
- `utils::curve::impl$5::receive_points`
- `utils::curve::impl$78::fmt`
- `utils::curve::smooth_in_place`
- `utils::geometry::aabb::Aabb2::from_points`
- `utils::geometry::aabb::Aabb2::overlaps_segment`
- `utils::geometry::aabb::Aabb3::from_center_extent`
- `utils::geometry::aabb::Aabb3::from_points`
- `utils::geometry::aabb::intersection_aabb_segment2`
- `utils::geometry::aabb::intersection_aabb_segment3`
- `utils::geometry::barycentric::barycentric2`
- `utils::geometry::circle::Circle2d::as_curve2_range`
- `utils::geometry::circle::Circle2d::as_curve3`
- `utils::geometry::circle::Circle2d::lerp`
- `utils::geometry::circle::Circle2d::yaw_radians`
- `utils::geometry::circle::Segment2d::new`
- `utils::geometry::circle::_::impl$0::deserialize`
- `utils::geometry::circle::_::impl$0::deserialize::impl$0::expecting`
- `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_bytes`
- `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_str`
- `utils::geometry::circle::_::impl$0::deserialize::impl$0::visit_u64`
- `utils::geometry::circle::_::impl$0::deserialize::impl$1::deserialize`
- `utils::geometry::circle::_::impl$0::deserialize::impl$2::expecting`
- `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_map`
- `utils::geometry::circle::_::impl$0::deserialize::impl$2::visit_seq`
- `utils::geometry::circle::_::impl$0::serialize`
- `utils::geometry::circle::impl$3::fmt`
- `utils::geometry::circle::impl$6::eq`
- `utils::geometry::circle::is_default`
- `utils::geometry::intersect::impl$1::hash`
- `utils::geometry::line_relation::LineInterval::cross`
- `utils::geometry::line_relation::LineInterval::div`
- `utils::geometry::line_relation::LineInterval::t_coord_to_point`
- `utils::geometry::position_from_mesh_uvs::get_triangle_coords_from_mesh_uv`
- `utils::geometry::ray::Ray::from_mouse_ss`
- `utils::geometry::rectangle::Rectangle2d::_get_normal_at_u`
- `utils::geometry::rectangle::Rectangle2d::apply_orientation_to_normal`
- `utils::geometry::rectangle::Rectangle2d::as_curve3`
- `utils::geometry::rectangle::Rectangle2d::as_points3`
- `utils::geometry::rectangle::Rectangle2d::as_transformation_and_size`
- `utils::geometry::rectangle::Rectangle2d::circular_slice`
- `utils::geometry::rectangle::Rectangle2d::circular_slice_segment_points`
- `utils::geometry::rectangle::Rectangle2d::closest_edge_normal`
- `utils::geometry::rectangle::Rectangle2d::closest_point_on_perimeter`
- `utils::geometry::rectangle::Rectangle2d::expand_along_edge`
- `utils::geometry::rectangle::Rectangle2d::get_closest_edge_id`
- `utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices`
- `utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos`
- `utils::geometry::rectangle::Rectangle2d::get_corner_unnormal`
- `utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id`
- `utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u`
- `utils::geometry::rectangle::Rectangle2d::get_normal_at_u`
- `utils::geometry::rectangle::Rectangle2d::half_dimensions`
- `utils::geometry::rectangle::Rectangle2d::intersects`
- `utils::geometry::rectangle::Rectangle2d::intersects_circle`
- `utils::geometry::rectangle::Rectangle2d::lerp`
- `utils::geometry::rectangle::Rectangle2d::local_aabb_of`
- `utils::geometry::rectangle::Rectangle2d::orientation_mat2`
- `utils::geometry::rectangle::Rectangle2d::orientation_mat3`
- `utils::geometry::rectangle::Rectangle3d::center`
- `utils::geometry::rectangle::Rectangle3d::height`
- `utils::geometry::rectangle::Rectangle3d::length`
- `utils::geometry::rectangle::Rectangle3d::rotate_projective_xz`
- `utils::geometry::rectangle::Rectangle3d::width`
- `utils::geometry::rectangle::_::impl$0::deserialize`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::expecting`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_bytes`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_str`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$0::visit_u64`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$1::deserialize`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::expecting`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_map`
- `utils::geometry::rectangle::_::impl$0::deserialize::impl$2::visit_seq`
- `utils::geometry::rectangle::_::impl$0::serialize`
- `utils::geometry::rectangle::impl$10::eq`
- `utils::geometry::rectangle::impl$2::from`
- `utils::geometry::rectangle::impl$3::clone`
- `utils::geometry::rectangle::impl$6::clone`
- `utils::geometry::rectangle::impl$7::fmt`
- `utils::geometry::triangle_triangle::cross_intersect::compute_line_intersection`
- `utils::geometry::triangle_triangle::cross_intersect::intersect_plane`
- `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_alone`
- `utils::geometry::triangle_triangle::cross_intersect::make_tri_a_vertex_positive`
- `utils::geometry::ws_to_cs`
- `utils::geometry::ws_to_ss`

## 库存边界与补充

244 条原 public + 407 条骨架函数头中，仅确证panic/外部库胶水排除；私有closures 26 条单列。trait/default/drop/serde若缺乏自动生成或库归属证据，保留为待归属项，避免漏掉策略。`curve2_serialized_as_curve3::serialize/deserialize` 是真正的私有转换策略，记为 untouched。

详细机器表包含独立 analysis/implementation/comparison/integration 布尔值、原完整名称/地址/原PDB记录、候选声明与行号、伪代码证据、比较限定和逐接口缺口。`get_functions` 或符号定位不等于分析完成。

当前没有原游戏消费者闭包通过验收的接口；WallSpace 的候选 bridge、Triangle 的候选 wrapper 和 crate 出口分别记录，不能代替原消费链验证。
