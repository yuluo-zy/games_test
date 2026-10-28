# crates/utils/src/geometry

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：151/97/54。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：private_game_source_with_pending_trait_schema_classification。

B原接口台账：151项已观察，97项已有部分业务分析，54项未触及；具体状态见逐接口证据。

## 接线与验证

接线：candidate_crate_exported_original_game_consumer_not_accepted。

原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。

## 已恢复的函数和类型范围

- utils::geometry::aabb::Aabb2::area
- utils::geometry::aabb::Aabb2::center
- utils::geometry::aabb::Aabb2::contains
- utils::geometry::aabb::Aabb2::expanded
- utils::geometry::aabb::Aabb2::from_center_dims
- utils::geometry::aabb::Aabb2::from_center_radius
- utils::geometry::aabb::Aabb2::half_dims
- utils::geometry::aabb::Aabb2::height
- utils::geometry::aabb::Aabb2::intersect_ray
- utils::geometry::aabb::Aabb2::intersects
- utils::geometry::aabb::Aabb2::max_extent
- utils::geometry::aabb::Aabb2::width
- utils::geometry::aabb::Aabb3::center
- utils::geometry::aabb::Aabb3::classify_ray_intersection
- utils::geometry::aabb::Aabb3::expand
- utils::geometry::aabb::Aabb3::expand_to_contain_point
- utils::geometry::aabb::Aabb3::extent
- utils::geometry::aabb::Aabb3::inclusive_contains_point
- utils::geometry::aabb::Aabb3::intersect_segment
- utils::geometry::aabb::Aabb3::scaled
- utils::geometry::aabb::Aabb3::translated
- utils::geometry::aabb::RayBoxIntersection::is_hit
- utils::geometry::aabb::is_inside_bbx
- utils::geometry::barycentric::barycentric
- utils::geometry::circle::Circle2d::aabb
- utils::geometry::circle::Circle2d::area
- utils::geometry::circle::Circle2d::as_points2
- utils::geometry::circle::Circle2d::as_points2_ptcount
- utils::geometry::circle::Circle2d::as_points3
- utils::geometry::circle::Circle2d::closest_point_on_perimeter
- utils::geometry::circle::Circle2d::contains_point
- utils::geometry::circle::Circle2d::get_closest_u_from_pos
- utils::geometry::circle::Circle2d::get_normal_at_u
- utils::geometry::circle::Circle2d::get_pos_at_u
- utils::geometry::circle::Circle2d::get_tangent_at_u
- utils::geometry::circle::Circle2d::intersect_circle
- utils::geometry::circle::Circle2d::intersect_ray
- utils::geometry::circle::Circle2d::is_point_inside
- utils::geometry::circle::Circle2d::new
- utils::geometry::circle::Circle2d::new_rotated
- utils::geometry::circle::Circle2d::perimeter
- utils::geometry::circle::Circle2d::signed_distance
- utils::geometry::circle::Circle2d::with_expanded_diameter
- utils::geometry::intersect::Sphere::as_f32x4
- utils::geometry::intersect::SphereIntersection::closest_intersection_distance
- utils::geometry::intersect::Triangle::aabb
- utils::geometry::intersect::Triangle::intersect_triangle
- utils::geometry::intersect::Triangle::new
- utils::geometry::intersect::Triangle::normal
- utils::geometry::intersect::Triangle::points_mut
- utils::geometry::intersect::ray_sphere_intersection
- utils::geometry::intersect::ray_triangle_intersection
- utils::geometry::line_relation::LineInterval::line_segment
- utils::geometry::line_relation::LineInterval::relate
- utils::geometry::plane::Plane::intersect_ray
- utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv
- utils::geometry::ray::Ray::xz
- utils::geometry::rectangle::Rectangle2d::aabb
- utils::geometry::rectangle::Rectangle2d::area
- utils::geometry::rectangle::Rectangle2d::as_curve2
- utils::geometry::rectangle::Rectangle2d::as_points2
- utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned
- utils::geometry::rectangle::Rectangle2d::contains_point
- utils::geometry::rectangle::Rectangle2d::contains_point_padding
- utils::geometry::rectangle::Rectangle2d::edge_length
- utils::geometry::rectangle::Rectangle2d::get_corner_normal
- utils::geometry::rectangle::Rectangle2d::get_edge_normal
- utils::geometry::rectangle::Rectangle2d::get_edge_pts
- utils::geometry::rectangle::Rectangle2d::get_point
- utils::geometry::rectangle::Rectangle2d::get_point_u
- utils::geometry::rectangle::Rectangle2d::get_pos_at_u
- utils::geometry::rectangle::Rectangle2d::new
- utils::geometry::rectangle::Rectangle2d::new_axis_aligned
- utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
- utils::geometry::rectangle::Rectangle2d::overlaps_rectangle2d
- utils::geometry::rectangle::Rectangle2d::perimeter
- utils::geometry::rectangle::Rectangle2d::try_as_curve2
- utils::geometry::rectangle::Rectangle2d::with_expanded_size
- utils::geometry::rectangle::Rectangle3d::contains_point
- utils::geometry::rectangle::Rectangle3d::expand
- utils::geometry::rectangle::Rectangle3d::extent
- utils::geometry::rectangle::Rectangle3d::new
- utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d
- utils::geometry::rectangle::Rectangle3d::points
- utils::geometry::rectangle::Rectangle3d::set_orientation_xz
- utils::geometry::rectangle::Rectangle3d::set_size
- utils::geometry::rectangle::Rectangle3d::translate
- utils::geometry::split_heightmap::split_heightmap_by_threshold
- utils::geometry::triangle_triangle::cross_intersect::cross_intersect
- utils::geometry::triangle_triangle::triangles_intersect::triangles_intersect
- utils::geometry::triangle_triangle::utils::is_tri_degenerated
- utils::geometry::triangle_triangle::utils::orient3d
- utils::geometry::triangle_triangle::utils::permute_tri_right

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
| `crates/utils/src/geometry/aabb.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs)：B_PDB_definition_source_and_reviewed_interface_contract<br>[skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs)：explicit_candidate_delivery_mapping | utils::geometry::aabb::Aabb3::extent（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::aabb::Aabb2::area（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::aabb::Aabb2::width（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::aabb::Aabb2::height（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::aabb::Aabb2::contains（implemented_unverified；业务等价以限定证据为准） | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/barycentric.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/brace.rs` | C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/geometry/circle.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs)：B_PDB_definition_source_and_reviewed_interface_contract<br>[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs)：explicit_candidate_delivery_mapping | utils::geometry::circle::Circle2d::signed_distance（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::as_points2_ptcount（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::is_point_inside（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::contains_point（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::new_rotated（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::get_pos_at_u（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::get_normal_at_u（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::circle::Circle2d::perimeter（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/intersect.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs)：B_PDB_definition_source_and_reviewed_interface_contract<br>[skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs](../../skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs](../../skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs](../../skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs)：explicit_candidate_delivery_mapping | utils::geometry::intersect::SphereIntersection::closest_intersection_distance（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::intersect::Triangle::points_mut（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::intersect::Triangle::new（implemented_unverified；业务等价以限定证据为准） | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/line_relation.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/mod.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/geometry/plane.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/position_from_mesh_uvs.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/ray.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/rectangle.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs)：B_PDB_definition_source_and_reviewed_interface_contract<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)：explicit_candidate_delivery_mapping | utils::geometry::rectangle::Rectangle2d::with_expanded_size（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::perimeter（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle3d::extent（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::area（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle3d::translate（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle3d::set_orientation_xz（implemented_unverified；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::new_axis_aligned（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::as_points2（original_located_source_candidate；业务等价以限定证据为准）<br>utils::geometry::rectangle::Rectangle2d::orientation_mat2（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/split_heightmap.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/triangle_triangle/cross_intersect.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |
| `crates/utils/src/geometry/triangle_triangle/utils.rs` | A,B,C | [reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs)：B_PDB_definition_source_and_reviewed_interface_contract | 全集/逐函数映射待核对 | candidate_present；仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。：[evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json](../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json), [tracking/shared-foundation-interfaces.json](../shared-foundation-interfaces.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/aabb.rs)

SHA256：`f7e18427827e01ca22ba9a0284ed8dd7534f03578b000a885463aa6352172786`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_points` | 12 | 未由扫描证明；以证据限定范围为准 |
| `intersect_segment` | 24 | 未由扫描证明；以证据限定范围为准 |
| `overlaps_segment` | 27 | 未由扫描证明；以证据限定范围为准 |
| `from_center_dims` | 30 | 未由扫描证明；以证据限定范围为准 |
| `from_center_radius` | 37 | 未由扫描证明；以证据限定范围为准 |
| `center` | 43 | 未由扫描证明；以证据限定范围为准 |
| `half_dims` | 46 | 未由扫描证明；以证据限定范围为准 |
| `area` | 49 | 未由扫描证明；以证据限定范围为准 |
| `contains` | 52 | 未由扫描证明；以证据限定范围为准 |
| `intersects` | 55 | 未由扫描证明；以证据限定范围为准 |
| `intersect_ray` | 62 | 未由扫描证明；以证据限定范围为准 |
| `max_extent` | 67 | 未由扫描证明；以证据限定范围为准 |
| `width` | 71 | 未由扫描证明；以证据限定范围为准 |
| `height` | 74 | 未由扫描证明；以证据限定范围为准 |
| `expanded` | 78 | 未由扫描证明；以证据限定范围为准 |
| `is_hit` | 99 | 未由扫描证明；以证据限定范围为准 |
| `from_points` | 106 | 未由扫描证明；以证据限定范围为准 |
| `expand_to_contain_point` | 120 | 未由扫描证明；以证据限定范围为准 |
| `expand` | 124 | 未由扫描证明；以证据限定范围为准 |
| `center` | 130 | 未由扫描证明；以证据限定范围为准 |
| `extent` | 133 | 未由扫描证明；以证据限定范围为准 |
| `scaled` | 137 | 未由扫描证明；以证据限定范围为准 |
| `translated` | 143 | 未由扫描证明；以证据限定范围为准 |
| `inclusive_contains_point` | 149 | 未由扫描证明；以证据限定范围为准 |
| `strict_contains` | 152 | 未由扫描证明；以证据限定范围为准 |
| `ray_interval` | 155 | 未由扫描证明；以证据限定范围为准 |
| `intersect_segment` | 162 | 未由扫描证明；以证据限定范围为准 |
| `classify_ray_intersection` | 178 | 未由扫描证明；以证据限定范围为准 |
| `is_inside_bbx` | 197 | 未由扫描证明；以证据限定范围为准 |
| `intersection_aabb_segment2` | 202 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/barycentric.rs)

SHA256：`d1b270395fd590228e3080cd09dd59b656731750988744d544307020f888a8b7`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `barycentric2` | 4 | 未由扫描证明；以证据限定范围为准 |
| `barycentric` | 13 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/circle.rs)

SHA256：`112f403d1fbe35385cb54e6072cd1af86115bf40d3849bd48ded7c3289e578b8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `lerp` | 15 | 未由扫描证明；以证据限定范围为准 |
| `new` | 23 | 未由扫描证明；以证据限定范围为准 |
| `new_rotated` | 30 | 未由扫描证明；以证据限定范围为准 |
| `area` | 37 | 未由扫描证明；以证据限定范围为准 |
| `perimeter` | 40 | 未由扫描证明；以证据限定范围为准 |
| `angle` | 43 | 未由扫描证明；以证据限定范围为准 |
| `get_pos_at_u` | 52 | 未由扫描证明；以证据限定范围为准 |
| `get_normal_at_u` | 59 | 未由扫描证明；以证据限定范围为准 |
| `get_tangent_at_u` | 64 | 未由扫描证明；以证据限定范围为准 |
| `get_closest_u_from_pos` | 68 | 未由扫描证明；以证据限定范围为准 |
| `signed_distance` | 78 | 未由扫描证明；以证据限定范围为准 |
| `is_point_inside` | 82 | 未由扫描证明；以证据限定范围为准 |
| `contains_point` | 86 | 未由扫描证明；以证据限定范围为准 |
| `closest_point_on_perimeter` | 90 | 未由扫描证明；以证据限定范围为准 |
| `with_expanded_diameter` | 94 | 未由扫描证明；以证据限定范围为准 |
| `aabb` | 100 | 未由扫描证明；以证据限定范围为准 |
| `intersect_ray` | 103 | 未由扫描证明；以证据限定范围为准 |
| `intersect_circle` | 121 | 未由扫描证明；以证据限定范围为准 |
| `as_points2_ptcount` | 138 | 未由扫描证明；以证据限定范围为准 |
| `as_points2` | 146 | 未由扫描证明；以证据限定范围为准 |
| `as_points3` | 149 | 未由扫描证明；以证据限定范围为准 |
| `try_as_curve2` | 157 | 未由扫描证明；以证据限定范围为准 |
| `as_curve2` | 162 | 未由扫描证明；以证据限定范围为准 |
| `as_curve3` | 165 | 未由扫描证明；以证据限定范围为准 |
| `as_curve2_range` | 169 | 未由扫描证明；以证据限定范围为准 |
| `next` | 189 | 未由扫描证明；以证据限定范围为准 |
| `size_hint` | 200 | 未由扫描证明；以证据限定范围为准 |
| `next_back` | 206 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/intersect.rs)

SHA256：`c4719ffbc6344d1362deef9a9ac6cd32b8fae3d8db15e9ef23c425f4f55dbd8e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 9 | 未由扫描证明；以证据限定范围为准 |
| `points_mut` | 12 | 未由扫描证明；以证据限定范围为准 |
| `normal` | 15 | 未由扫描证明；以证据限定范围为准 |
| `aabb` | 20 | 未由扫描证明；以证据限定范围为准 |
| `intersect_triangle` | 26 | 未由扫描证明；以证据限定范围为准 |
| `as_f32x4` | 39 | 未由扫描证明；以证据限定范围为准 |
| `closest_intersection_distance` | 50 | 未由扫描证明；以证据限定范围为准 |
| `ray_triangle_intersection` | 68 | 未由扫描证明；以证据限定范围为准 |
| `ray_sphere_intersection` | 97 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/line_relation.rs)

SHA256：`fe50d4b1a589e41095e0009e4698a5e3b98ef3be8b7a28be9487a30596938496`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `line_segment` | 18 | 未由扫描证明；以证据限定范围为准 |
| `relate` | 26 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/mod.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/mod.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/mod.rs)

SHA256：`ad521f76d803479b2e971f8b7f7c3dc941544c9430a481233a3c0f3dbb7ba1ce`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `ws_to_cs` | 22 | 未由扫描证明；以证据限定范围为准 |
| `ws_to_ss` | 26 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/plane.rs)

SHA256：`b4d0ccd0daabf6257939b422919c1528cac97e9ff2f8454048e5fc78157b0502`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `intersect_ray` | 10 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/position_from_mesh_uvs.rs)

SHA256：`49488a85b8bce34cd7602257f5a3434a87b8fe30d2bd003e1eb39598553ba9b1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `get_triangle_coords_from_mesh_uv` | 4 | 未由扫描证明；以证据限定范围为准 |
| `get_position_from_mesh_uv` | 19 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/ray.rs)

SHA256：`65953e42979151bccbc8a9ce7ed9dad00d3a36d8c455ee4153440818dbc6a149`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `from_mouse_ss` | 17 | 未由扫描证明；以证据限定范围为准 |
| `xz` | 33 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/rectangle.rs)

SHA256：`cc2bab0d31a2131f984a5f2fa1e2f1b7a582f3558ba0eb06f724d1cf25b03fae`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `emit` | 25 | 未由扫描证明；以证据限定范围为准 |
| `orientation_mat2` | 42 | 未由扫描证明；以证据限定范围为准 |
| `perimeter_local_point` | 47 | 未由扫描证明；以证据限定范围为准 |
| `as_transformation_and_size` | 51 | 未由扫描证明；以证据限定范围为准 |
| `new_centered_axis_aligned` | 54 | 未由扫描证明；以证据限定范围为准 |
| `new_axis_aligned` | 57 | 未由扫描证明；以证据限定范围为准 |
| `new` | 60 | 未由扫描证明；以证据限定范围为准 |
| `local_to_world` | 69 | 未由扫描证明；以证据限定范围为准 |
| `world_to_local` | 75 | 未由扫描证明；以证据限定范围为准 |
| `as_points2_centered_aligned` | 82 | 未由扫描证明；以证据限定范围为准 |
| `as_points2` | 92 | 未由扫描证明；以证据限定范围为准 |
| `area` | 96 | 未由扫描证明；以证据限定范围为准 |
| `perimeter` | 99 | 未由扫描证明；以证据限定范围为准 |
| `corner_us` | 103 | 未由扫描证明；以证据限定范围为准 |
| `get_pos_at_u` | 113 | 未由扫描证明；以证据限定范围为准 |
| `get_edge_normal` | 137 | 未由扫描证明；以证据限定范围为准 |
| `get_point` | 145 | 未由扫描证明；以证据限定范围为准 |
| `get_point_u` | 149 | 未由扫描证明；以证据限定范围为准 |
| `get_corner_normal` | 153 | 未由扫描证明；以证据限定范围为准 |
| `normal_at_u` | 167 | 未由扫描证明；以证据限定范围为准 |
| `get_normal_at_u` | 180 | 未由扫描证明；以证据限定范围为准 |
| `get_normal_at_u_non_normalized_corners` | 184 | 未由扫描证明；以证据限定范围为准 |
| `get_edge_id_at_u` | 187 | 未由扫描证明；以证据限定范围为准 |
| `closest_edge_normal` | 193 | 未由扫描证明；以证据限定范围为准 |
| `get_closest_edge_id` | 196 | 未由扫描证明；以证据限定范围为准 |
| `signed_distance_to_closest_edge` | 204 | 未由扫描证明；以证据限定范围为准 |
| `distance_to_closest_edge` | 208 | 未由扫描证明；以证据限定范围为准 |
| `get_edge_pts` | 211 | 未由扫描证明；以证据限定范围为准 |
| `edge_length` | 216 | 未由扫描证明；以证据限定范围为准 |
| `try_as_curve2` | 224 | 未由扫描证明；以证据限定范围为准 |
| `as_curve2` | 230 | 未由扫描证明；以证据限定范围为准 |
| `as_points3` | 233 | 未由扫描证明；以证据限定范围为准 |
| `as_curve3` | 236 | 未由扫描证明；以证据限定范围为准 |
| `circular_slice` | 244 | 未由扫描证明；以证据限定范围为准 |
| `circular_slice_segment_points` | 266 | 未由扫描证明；以证据限定范围为准 |
| `intersects_circle` | 307 | 未由扫描证明；以证据限定范围为准 |
| `intersects` | 317 | 未由扫描证明；以证据限定范围为准 |
| `expand_along_edge` | 337 | 未由扫描证明；以证据限定范围为准 |
| `overlaps_rectangle2d` | 346 | 未由扫描证明；以证据限定范围为准 |
| `within` | 347 | 未由扫描证明；以证据限定范围为准 |
| `contains_point` | 359 | 未由扫描证明；以证据限定范围为准 |
| `contains_point_padding` | 362 | 未由扫描证明；以证据限定范围为准 |
| `with_expanded_size` | 367 | 未由扫描证明；以证据限定范围为准 |
| `aabb` | 373 | 未由扫描证明；以证据限定范围为准 |
| `rotate_projective_xz` | 397 | 未由扫描证明；以证据限定范围为准 |
| `new` | 401 | 未由扫描证明；以证据限定范围为准 |
| `translate` | 413 | 未由扫描证明；以证据限定范围为准 |
| `set_orientation_xz` | 418 | 未由扫描证明；以证据限定范围为准 |
| `set_size` | 421 | 未由扫描证明；以证据限定范围为准 |
| `expand` | 428 | 未由扫描证明；以证据限定范围为准 |
| `extent` | 434 | 未由扫描证明；以证据限定范围为准 |
| `contains_point` | 441 | 未由扫描证明；以证据限定范围为准 |
| `overlaps_rectangle3d` | 446 | 未由扫描证明；以证据限定范围为准 |
| `points` | 450 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L16：/// Original callback discriminants 4 and 5; their nominal enum name is unknown.
### reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs)

SHA256：`4f6b6a57d00ac1b533e2ec573d4b9bbe123c36cdb64a77dce3ca7e7f0b798593`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `above_sample` | 7 | 未由扫描证明；以证据限定范围为准 |
| `below_to_above` | 8 | 未由扫描证明；以证据限定范围为准 |
| `above_to_below` | 9 | 未由扫描证明；以证据限定范围为准 |
| `split_heightmap_into` | 13 | 未由扫描证明；以证据限定范围为准 |
| `new` | 47 | 未由扫描证明；以证据限定范围为准 |
| `crossing_u` | 53 | 未由扫描证明；以证据限定范围为准 |
| `above_sample` | 64 | 未由扫描证明；以证据限定范围为准 |
| `below_to_above` | 75 | 未由扫描证明；以证据限定范围为准 |
| `above_to_below` | 82 | 未由扫描证明；以证据限定范围为准 |
| `split_heightmap_regions` | 91 | 未由扫描证明；以证据限定范围为准 |
| `split_heightmap_by_threshold` | 105 | 未由扫描证明；以证据限定范围为准 |
| `above_sample` | 108 | 未由扫描证明；以证据限定范围为准 |
| `below_to_above` | 109 | 未由扫描证明；以证据限定范围为准 |
| `above_to_below` | 112 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/cross_intersect.rs)

SHA256：`59ed8b31b4d0a705431329ea907c534e40fc2abeed878c46ae07b2e4cd06d991`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `cross_intersect` | 9 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/triangles_intersect.rs)

SHA256：`c8f18409adf8b1cf74f2d108f1efec4cb166a42ccffa4a4c9e7d4cab3f1292ce`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `triangles_intersect` | 9 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs

[reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs](../../reconstruction/shared-foundation/crates/utils/src/geometry/triangle_triangle/utils.rs)

SHA256：`b725f0bd3e30cdb73e99b2ac431344283a055acbe470c6b983bc8fe141ec2f15`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `degenerate` | 3 | 未由扫描证明；以证据限定范围为准 |
| `side` | 8 | 未由扫描证明；以证据限定范围为准 |
| `determinant` | 18 | 未由扫描证明；以证据限定范围为准 |
| `canonicalize` | 29 | 未由扫描证明；以证据限定范围为准 |
| `orient` | 45 | 未由扫描证明；以证据限定范围为准 |
| `edge_plane_intersection` | 50 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs

[skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs)

SHA256：`b0c677042a3cc0caa7cd861fd85354fd9c856ac3d15499771063488d94f8855f`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `width` | 11 | 未由扫描证明；以证据限定范围为准 |
| `height` | 18 | 未由扫描证明；以证据限定范围为准 |
| `contains` | 25 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs

[skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs](../../skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs)

SHA256：`de7791465afb039aa4a42de5d611c1d777cb66142d1d80d3a289457be74be947`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `area` | 21 | 未由扫描证明；以证据限定范围为准 |
| `extent` | 32 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs

[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs)

SHA256：`10190274c7e6773ac7939515194739cd935e4863aad50088e8208cc1e9761cae`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `area` | 11 | 未由扫描证明；以证据限定范围为准 |
| `perimeter` | 18 | 未由扫描证明；以证据限定范围为准 |
| `extent` | 37 | 未由扫描证明；以证据限定范围为准 |
| `translate` | 45 | 未由扫描证明；以证据限定范围为准 |
| `set_orientation_xz` | 54 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs

[skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs](../../skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs)

SHA256：`2b96f5fc78db8f8982a4d038b1c37c0adee8ee2fc97e1807a8af3af6497277c2`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `with_expanded_size` | 17 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs

[skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs](../../skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs)

SHA256：`5ca5c009e19ea3ed0866c25b2b87365839f794553fb5258cf42b741ea2d6efb8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 13 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs

[skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs](../../skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs)

SHA256：`887b174da80a803fb77d2eacd7f62cdd1575f93bd4158ed584b682f7dee366e5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `closest_intersection_distance` | 16 | 未由扫描证明；以证据限定范围为准 |
| `points_mut` | 36 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs

[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs)

SHA256：`c06f6c65b964f7a7ca210e32ddfb8a5abb7946ad1ea74ea0a7ddfd2673b220ad`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `contains_point` | 12 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs

[skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs](../../skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs)

SHA256：`6ff3abe960ebd49b68cef70bd843b4849e4db46e0e3065c13f47fbd35917c0d8`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `signed_distance` | 23 | 未由扫描证明；以证据限定范围为准 |
| `is_point_inside` | 32 | 未由扫描证明；以证据限定范围为准 |
| `as_points2_ptcount` | 41 | 未由扫描证明；以证据限定范围为准 |
