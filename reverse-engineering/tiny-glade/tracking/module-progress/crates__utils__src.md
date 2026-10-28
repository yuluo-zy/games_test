# crates/utils/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：135/99/36。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

[业务功能对照](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)：{"implemented_scoped": 28, "partial_missing_logic": 20, "not_implemented": 9, "out_of_baseline": 1}。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。

定义归属：private_game_source。

22顶层文件候选：camera/RDP/Spline/Timer/区间等已对照并修复；原PDB内联接口仍有遗漏。

## 接线与验证

接线：候选模块可调用，完整原游戏消费者接线仍待完成。

全utils37测试、strict Clippy、12候选JSON场景通过；新增916原码记录、继承367记录；不等于全部原接口验收。

## 已恢复的函数和类型范围

- utils::angle_range::AngleRange::clamp::h1e38e505b21716f7
- utils::angle_range::AngleRange::from_min_and_len::h33d2357d24df0270
- utils::camera::CameraMatrices::eye_right::ha9cbece401b86c69
- utils::camera::CameraMatrices::eye_position::h197cc691c4961e2b
- utils::camera::CameraMatrices::xz_projective_rotation::h2a492c1381c13566
- utils::camera::CameraProjectionParams::calc_matrices::ha4168fdbac703f7b
- utils::camera::CameraLens::immediately_set_vertical_fov_degrees::hb8b585f91f9a3e21
- utils::camera::CameraMatrices::eye_direction::hdd8c9098532097c3
- utils::camera::CameraBodyMatrices::from_position_rotation::h15f3135b9d882184
- utils::camera::CameraMatrices::aspect_ratio::h2280663f762167ae
- utils::camera::CameraLens::new::hba96194044b772a6
- utils::circular_f32::CircularF32::new::h4a370bc0f4ecfd83
- utils::circular_f32::CircularF32::set_value::h2a27842468e6d8cd
- utils::circular_f32::CircularF32::distance::haeb25b59b8695df4
- utils::circular_f32::CircularF32::as_f32::h4a622c1cdce81d18
- utils::circular_f32::CircularF32::lerp::h3fb4800358aded15
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::distance::h7b2c52b959561051
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project::hb54ceb2dc35b6a0d
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project_unbounded::ha90815c3ad318565
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project_clamp::h403dbe9f0dcfece2
- _$LT$utils..comparative_space..CircularComparativeSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::distance::h63c7050070005b2d
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::subtract::hf7a5d0707cc7372d
- utils::easing_functions::EasingFunction::apply::hb692303304673b5b
- utils::easing_functions::cubic_in_out::h9f90081ff645ad35
- utils::file::generate_backup_file_path::hfb5bed1165e208fd
- utils::file::copy_file_to_backup::hcb9f224743c1f22d
- utils::file::date_time_string_for_filename::he181c464f5c8002c
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h21c466addbd36c5b (.llvm.10815386406131266915)
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h5f9a59d6ff3084ac
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h37f26bb52d35e9a5
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h6ec8cfbf8d357fe5
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h72e4e1f27b7e5695
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h054924d541f34fec
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h461c683309370f25 (.llvm.9386989208921278972)
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h7b46bd8afae5f9b9
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h40b3724ed41a6401
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::h9ee14f70b38e34f5
- _$LT$T$u20$as$u20$utils..float_grid..FloatGridSampler$GT$::bilinear_sample_at_world_xz::hdd12a535fe7377c0
- _$LT$glam..f32..sse2..vec4..Vec4$u20$as$u20$utils..halver..Halver$GT$::to_f16::hb66f3c2c26d2deba
- _$LT$f32$u20$as$u20$utils..halver..Halver$GT$::to_f16::h628d79fa31310e7a
- _$LT$$u5b$f32$u3b$$u20$4$u5d$$u20$as$u20$utils..halver..Halver$GT$::to_f16::h16803eaa118aabc6
- _$LT$$u5b$f32$u3b$$u20$3$u5d$$u20$as$u20$utils..halver..Halver$GT$::to_f16::ha8b503566bb8cdf3
- _$LT$glam..f32..vec3..Vec3$u20$as$u20$utils..halver..Halver$GT$::to_f16::h51c93e75a0ff6032
- _$LT$glam..f32..sse2..quat..Quat$u20$as$u20$utils..halver..Halver$GT$::to_f16::h7d373b4258af3426
- utils::index_condenser::IndexCondenser$LT$K$GT$::remove::h5522bb7ae0853256
- utils::local_time::init_local_time::h4b7eea84f2234896
- utils::local_time::system_time_to_local_time::h7f709757ba2dd5da
- utils::loop_range::LoopRange::start::he216921d941dd42d
- utils::loop_range::LoopIdx::raw_idx::h7d0ec4316db23cac
- utils::loop_range::LoopRange::len::h037da5194cbf249f
- utils::loop_range::LoopRange::is_empty::h5dcae5a5e640982a
- utils::loop_range::LoopRange::new_loop::hf6cc726d4bea6953
- utils::loop_range::LoopRange::end::hd947be2921f572df
- utils::loop_range::LoopIdx::index::h5da9681895b19d80
- utils::loop_range::LoopRange::subslice::h58705f0479c7b7d3
- utils::loop_range::LoopRange::new_line::h6253d498842a5e06
- utils::name_hash::NameHash::compute::h6604b19c12e29953
- utils::name_hash::NameHash::debug_name::h9048771f0a8c6cb2
- utils::natural_cubic::Spline2D::from_control_points_with_tangents::h8ff9c8fab7ec82bb
- utils::natural_cubic::Spline2D::from_periodic_loop::h2215646c48812760
- utils::natural_cubic::Spline3D::from_control_points_with_tangents::h4f96a7d574b7be56
- utils::natural_cubic::SplineSegment2D::position_at_local_t::he9e0d14242507778
- utils::natural_cubic::Spline3D::tangent_at_t::h5aeef5094617714b
- utils::natural_cubic::Spline3D::position_at_t::h7ec0ddb3ddad631c
- utils::natural_cubic::SplineSegment2D::tangent_at_local_t::h8fafdc5f1299a652
- utils::natural_cubic::Spline3D::to_owned::hef13392e53373cba
- utils::rdp::Rdp$LT$T$GT$::new_with_opts::hc10fe7c6319b6f02
- utils::rgb9e5::pack_rgb9e5::h1919f53c9d068e05
- utils::rng::init_rng_u64::haf5ce7081dff94f8
- utils::rng::init_rng::h2380124e07933713
- utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range::h38f45a92e1741cd8
- utils::subtractive_1d_boolean::SubtractiveBoolean1D::result::hd0197dcae2cf048c
- utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract::h9b7301b3325c1f55
- utils::threshold_range::threshold_range::hbf358fb23f7c7319
- utils::threshold_range::threshold_range::h4af01b25adab61ab
- utils::threshold_range::threshold_range::he03025b31b314fc7
- utils::timer::FilteredWallTime::delta_secs::h894dea9d709deba3
- utils::wall_space::WallSpaceCoord::get_pos_xz::h8fb9935763211cd2
- utils::wall_space::WallSpace::space_length::h8863a0d2fe8bbbf2
- utils::wall_space::WallSpaceCoord::to_world_space::h83d891612441f5e5
- utils::wall_space::WallSpaceCoord::with_y::h0af639b6442aa6e3
- utils::wall_space::WallSpaceCoord::from_world_space::h3dd7969c0bf40780
- utils::wall_space::WallSpaceCoord::with_y_offset::hb75ce7f8b3c3d42c
- utils::wall_space::WallSpaceCoord::from_curve_internal_coord::h268e5b71461535d7
- utils::wall_space::WallSpaceCoord::new_unchecked::h76dae8b6ebe6ff7f
- utils::wall_space::WallSpaceCoord::get_tangent::h651bf111d4d6fc73
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::distance::h7b2c52b959561051
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project::hb54ceb2dc35b6a0d
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project_unbounded::ha90815c3ad318565
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::project_clamp::h403dbe9f0dcfece2
- _$LT$utils..wall_space..WallSpace$u20$as$u20$utils..comparative_space..ComparativeSpace$GT$::subtract::hf7a5d0707cc7372d

## 明确缺口

- Swizzlers/IndexCondenser/WallSpace等已知原接口尚未恢复
- Spline退化/重复点边界未验证
- FloatGrid/WallSpace完整消费者及原存档visitor差分未完成

下一闭包：按原函数接口库存优先补未触及工作

## 证据

- [evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/delivery.json)
- [evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-tabloid/CONTRACT.md)
- [evidence/shared-foundation-2026-10-13/utils-tabloid/native-manifest.json](../../evidence/shared-foundation-2026-10-13/utils-tabloid/native-manifest.json)
- [evidence/shared-foundation-2026-10-13/integration/dependency-validation.json](../../evidence/shared-foundation-2026-10-13/integration/dependency-validation.json)
- [evidence/shared-foundation-2026-10-13/PARALLEL_PLAN.md](../../evidence/shared-foundation-2026-10-13/PARALLEL_PLAN.md)
- [evidence/shared-foundation-2026-10-13/utils-base/delivery.json](../../evidence/shared-foundation-2026-10-13/utils-base/delivery.json)
- [evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md](../../evidence/shared-foundation-2026-10-13/utils-base/CONTRACT.md)
- [evidence/shared-foundation-2026-10-13/utils-base/verification.json](../../evidence/shared-foundation-2026-10-13/utils-base/verification.json)
- [evidence/shared-foundation-review-2026-10-13/REVIEW.md](../../evidence/shared-foundation-review-2026-10-13/REVIEW.md)
- [tracking/shared-foundation-code-review.json](../shared-foundation-code-review.json)
- [evidence/shared-foundation-review-2026-10-13/integration-validation.json](../../evidence/shared-foundation-review-2026-10-13/integration-validation.json)
- [FOUNDATION_BUSINESS_COMPARISON.md](../../docs/reference/FOUNDATION_BUSINESS_COMPARISON.md)
- [tracking/shared-foundation-business-comparison.json](../shared-foundation-business-comparison.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/utils/src/angle_range.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/camera.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/circular_f32.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/comparative_space.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/easing_functions.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/file.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/float_grid.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/halver.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/index_condenser.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/lib.rs` | A,B,C | [reconstruction/roof/tiles.rs](../../reconstruction/roof/tiles.rs)：reviewed_partial_module_mapping<br>[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs)：explicit_candidate_delivery_mapping | utils::inverse_lerp（implemented_unverified；业务等价以限定证据为准）<br>utils::fit（implemented_unverified；业务等价以限定证据为准）<br>utils::impl$5::intersects（implemented_unverified；业务等价以限定证据为准）<br>utils::lerp（implemented_unverified；业务等价以限定证据为准）<br>utils::random_splits_into（original_located_source_candidate；业务等价以限定证据为准）<br>utils::random_splits（original_located_source_candidate；业务等价以限定证据为准） | candidate_present；私有random_splits调用策略，不包含RNG库实现：[evidence/roof-tiles/dependency-boundary.json](../../evidence/roof-tiles/dependency-boundary.json) |
| `crates/utils/src/local_time.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/loop_range.rs` | A,B,C | [skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs)：explicit_candidate_delivery_mapping | utils::loop_range::LoopRange::start（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::LoopRange::end（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::impl$1::sub（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::LoopRange::is_empty（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::LoopIdx::raw_idx（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::impl$3::next_back（implemented_unverified；业务等价以限定证据为准）<br>utils::loop_range::LoopRange::len（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/utils/src/name_hash.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/natural_cubic.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/rdp.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/rgb9e5.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/rng.rs` | A,B,C | [skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs](../../skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs](../../skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs)：explicit_candidate_delivery_mapping | utils::rng::init_rng_u64（implemented_unverified；业务等价以限定证据为准）<br>utils::rng::init_rng（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/utils/src/selective_deserialize.rs` | B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/subtractive_1d_boolean.rs` | A,B,C | [skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs](../../skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs)：explicit_candidate_delivery_mapping | utils::subtractive_1d_boolean::SubtractiveBoolean1D::result（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/utils/src/swizzlers.rs` | A,B,C | [skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs](../../skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs)：explicit_candidate_delivery_mapping | utils::swizzlers::impl$1::xvy（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/utils/src/threshold_range.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/utils/src/timer.rs` | A,B,C | [skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs](../../skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs](../../skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs)：explicit_candidate_delivery_mapping | utils::timer::_::impl$0::apply（implemented_unverified；业务等价以限定证据为准）<br>utils::timer::_::impl$0::queue（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |
| `crates/utils/src/wall_space.rs` | A,B,C | [skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs](../../skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs)：explicit_candidate_delivery_mapping<br>[skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs](../../skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs)：explicit_candidate_delivery_mapping | utils::wall_space::WallSpaceCoord::with_y_offset（implemented_unverified；业务等价以限定证据为准）<br>utils::wall_space::WallSpaceCoord::with_y（implemented_unverified；业务等价以限定证据为准） | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/roof/tiles.rs

[reconstruction/roof/tiles.rs](../../reconstruction/roof/tiles.rs)

SHA256：`0b643d6d41ffd1d4c0adf9476ba412c64e999dbe99ec6bc931e64543b249c68b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `next_f32` | 10 | 未由扫描证明；以证据限定范围为准 |
| `random_splits` | 22 | 未由扫描证明；以证据限定范围为准 |
| `circular_row_plan` | 47 | 未由扫描证明；以证据限定范围为准 |
| `circular_row_coordinates` | 61 | 未由扫描证明；以证据限定范围为准 |
| `circular_column_count` | 67 | 未由扫描证明；以证据限定范围为准 |
| `circular_boundaries` | 81 | 未由扫描证明；以证据限定范围为准 |
| `half` | 106 | 未由扫描证明；以证据限定范围为准 |
| `record` | 110 | 未由扫描证明；以证据限定范围为准 |
| `curve_coord` | 122 | 未由扫描证明；以证据限定范围为准 |
| `circle_unit` | 130 | 未由扫描证明；以证据限定范围为准 |
| `rotate` | 136 | 未由扫描证明；以证据限定范围为准 |
| `matrix_quat` | 138 | 未由扫描证明；以证据限定范围为准 |
| `assemble_circular_tiles` | 146 | 未由扫描证明；以证据限定范围为准 |
| `assemble_circular_from_observed` | 194 | 未由扫描证明；以证据限定范围为准 |
| `expanded_bounds_observed` | 220 | 未由扫描证明；以证据限定范围为准 |
| `bytes` | 236 | 未由扫描证明；以证据限定范围为准 |
| `from_bytes` | 237 | 未由扫描证明；以证据限定范围为准 |
| `axis_aligned` | 238 | 未由扫描证明；以证据限定范围为准 |
| `basis` | 239 | 未由扫描证明；以证据限定范围为准 |
| `corners` | 241 | 未由扫描证明；以证据限定范围为准 |
| `rectangular_context_from_observed` | 261 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/angle_range.rs

[reconstruction/shared-foundation/crates/utils/src/angle_range.rs](../../reconstruction/shared-foundation/crates/utils/src/angle_range.rs)

SHA256：`d24bbecf97ee0b890445dac75c985695ebae074bfc87288f9fee9ff6639cf392`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wrap` | 4 | 未由扫描证明；以证据限定范围为准 |
| `from_min_and_len` | 6 | 未由扫描证明；以证据限定范围为准 |
| `clamp` | 10 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/camera.rs

[reconstruction/shared-foundation/crates/utils/src/camera.rs](../../reconstruction/shared-foundation/crates/utils/src/camera.rs)

SHA256：`d0997f45be9bfa01027e9d2282ef825d708715a8e6c3261645c0fad91489fcb9`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `calc_matrices` | 16 | 未由扫描证明；以证据限定范围为准 |
| `from_position_rotation` | 44 | 未由扫描证明；以证据限定范围为准 |
| `eye_position` | 59 | 未由扫描证明；以证据限定范围为准 |
| `eye_direction` | 62 | 未由扫描证明；以证据限定范围为准 |
| `eye_right` | 65 | 未由扫描证明；以证据限定范围为准 |
| `aspect_ratio` | 68 | 未由扫描证明；以证据限定范围为准 |
| `xz_projective_rotation` | 71 | 未由扫描证明；以证据限定范围为准 |
| `new` | 104 | 未由扫描证明；以证据限定范围为准 |
| `immediately_set_vertical_fov_degrees` | 128 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/circular_f32.rs

[reconstruction/shared-foundation/crates/utils/src/circular_f32.rs](../../reconstruction/shared-foundation/crates/utils/src/circular_f32.rs)

SHA256：`88e5a2e7901c24c8c2e4364073319abf43e78ef3cb8d9aff891e48d902c04184`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `wrap` | 7 | 未由扫描证明；以证据限定范围为准 |
| `new` | 9 | 未由扫描证明；以证据限定范围为准 |
| `set_value` | 10 | 未由扫描证明；以证据限定范围为准 |
| `as_f32` | 11 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 12 | 未由扫描证明；以证据限定范围为准 |
| `lerp` | 13 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/comparative_space.rs

[reconstruction/shared-foundation/crates/utils/src/comparative_space.rs](../../reconstruction/shared-foundation/crates/utils/src/comparative_space.rs)

SHA256：`a287977adb195a0dcd7dd2c2889b6f10a405d7c936aad7752405a2e919fccd22`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `distance` | 6 | 未由扫描证明；以证据限定范围为准 |
| `project` | 7 | 未由扫描证明；以证据限定范围为准 |
| `project_clamp` | 8 | 未由扫描证明；以证据限定范围为准 |
| `project_unbounded` | 9 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 10 | 未由扫描证明；以证据限定范围为准 |
| `lerp_clamp` | 11 | 未由扫描证明；以证据限定范围为准 |
| `regions_intersect` | 16 | 未由扫描证明；以证据限定范围为准 |
| `classify_side` | 19 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 28 | 未由扫描证明；以证据限定范围为准 |
| `project` | 29 | 未由扫描证明；以证据限定范围为准 |
| `project_clamp` | 32 | 未由扫描证明；以证据限定范围为准 |
| `project_unbounded` | 33 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 34 | 未由扫描证明；以证据限定范围为准 |
| `distance` | 38 | 未由扫描证明；以证据限定范围为准 |
| `project` | 39 | 未由扫描证明；以证据限定范围为准 |
| `project_clamp` | 40 | 未由扫描证明；以证据限定范围为准 |
| `project_unbounded` | 41 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 42 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/easing_functions.rs

[reconstruction/shared-foundation/crates/utils/src/easing_functions.rs](../../reconstruction/shared-foundation/crates/utils/src/easing_functions.rs)

SHA256：`fb8011ab85c5e50ed82f2178d385cde3b042aac3550c173af77b96e72289298b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `cubic_in_out` | 7 | 未由扫描证明；以证据限定范围为准 |
| `apply` | 12 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/file.rs

[reconstruction/shared-foundation/crates/utils/src/file.rs](../../reconstruction/shared-foundation/crates/utils/src/file.rs)

SHA256：`1c53d1f379c7368dfbc50b89a9d9149c4d780a32c6fb163977dcfafa3fda44e5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `date_time_string_for_filename_at` | 3 | 未由扫描证明；以证据限定范围为准 |
| `date_time_string_for_filename` | 7 | 未由扫描证明；以证据限定范围为准 |
| `generate_backup_file_path_at` | 10 | 未由扫描证明；以证据限定范围为准 |
| `generate_backup_file_path` | 14 | 未由扫描证明；以证据限定范围为准 |
| `copy_file_to_backup` | 15 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/float_grid.rs

[reconstruction/shared-foundation/crates/utils/src/float_grid.rs](../../reconstruction/shared-foundation/crates/utils/src/float_grid.rs)

SHA256：`b975bbff901ab153bd20549fe37eed4c9d64b0793987fcd53293c2f05dfbd650`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `dimensions` | 4 | 未由扫描证明；以证据限定范围为准 |
| `load_index_unchecked` | 5 | 未由扫描证明；以证据限定范围为准 |
| `bilinear_sample_at_world_xz` | 10 | 未由扫描证明；以证据限定范围为准 |
| `dimensions` | 33 | 未由扫描证明；以证据限定范围为准 |
| `load_index_unchecked` | 34 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/halver.rs

[reconstruction/shared-foundation/crates/utils/src/halver.rs](../../reconstruction/shared-foundation/crates/utils/src/halver.rs)

SHA256：`10f1569eb954dfece1d7af10a53b01ee896e2e7b11f67072832edbc77ec19e33`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/utils/src/index_condenser.rs

[reconstruction/shared-foundation/crates/utils/src/index_condenser.rs](../../reconstruction/shared-foundation/crates/utils/src/index_condenser.rs)

SHA256：`6f9a86ee03f79b7448a2337542526e6199fa88f78a792b2e6a5994c73168f858`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `last_known_index` | 13 | 未由扫描证明；以证据限定范围为准 |
| `default` | 22 | 未由扫描证明；以证据限定范围为准 |
| `insert` | 32 | 未由扫描证明；以证据限定范围为准 |
| `keys` | 43 | 未由扫描证明；以证据限定范围为准 |
| `remove` | 46 | 未由扫描证明；以证据限定范围为准 |
| `resolve` | 59 | 未由扫描证明；以证据限定范围为准 |
| `get` | 62 | 未由扫描证明；以证据限定范围为准 |
| `clear` | 71 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/lib.rs

[reconstruction/shared-foundation/crates/utils/src/lib.rs](../../reconstruction/shared-foundation/crates/utils/src/lib.rs)

SHA256：`2fac701e3f0f91646f431e6cfd6eec0d4bb2234532bab284770cc10153fe2b50`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/shared-foundation/crates/utils/src/local_time.rs

[reconstruction/shared-foundation/crates/utils/src/local_time.rs](../../reconstruction/shared-foundation/crates/utils/src/local_time.rs)

SHA256：`e7987286ead98c76c8fbfad5f527f7c8d5e1ef6666a5e4cf0933c05541748f00`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `init_local_time` | 6 | 未由扫描证明；以证据限定范围为准 |
| `system_time_to_local_time` | 10 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/loop_range.rs

[reconstruction/shared-foundation/crates/utils/src/loop_range.rs](../../reconstruction/shared-foundation/crates/utils/src/loop_range.rs)

SHA256：`0df4ac3afa385b8397314408d997b79d33cb9bb34564cc6f3c1e4e4f6637c0de`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raw_idx` | 7 | 未由扫描证明；以证据限定范围为准 |
| `index` | 8 | 未由扫描证明；以证据限定范围为准 |
| `new_loop` | 14 | 未由扫描证明；以证据限定范围为准 |
| `new_line` | 20 | 未由扫描证明；以证据限定范围为准 |
| `start` | 26 | 未由扫描证明；以证据限定范围为准 |
| `end` | 27 | 未由扫描证明；以证据限定范围为准 |
| `len` | 28 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 29 | 未由扫描证明；以证据限定范围为准 |
| `subslice` | 32 | 未由扫描证明；以证据限定范围为准 |
| `map_slice` | 39 | 未由扫描证明；以证据限定范围为准 |
| `next` | 45 | 未由扫描证明；以证据限定范围为准 |
| `size_hint` | 49 | 未由扫描证明；以证据限定范围为准 |
| `next_back` | 52 | 未由扫描证明；以证据限定范围为准 |
| `clone` | 68 | 未由扫描证明；以证据限定范围为准 |
| `len` | 71 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 72 | 未由扫描证明；以证据限定范围为准 |
| `next` | 76 | 未由扫描证明；以证据限定范围为准 |
| `size_hint` | 79 | 未由扫描证明；以证据限定范围为准 |
| `next_back` | 82 | 未由扫描证明；以证据限定范围为准 |
| `index` | 89 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/name_hash.rs

[reconstruction/shared-foundation/crates/utils/src/name_hash.rs](../../reconstruction/shared-foundation/crates/utils/src/name_hash.rs)

SHA256：`4bcb4d8fc8c3a5ef179b669f57d58865caf57394eefd00d880df2a69dfff30c4`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `compute` | 5 | 未由扫描证明；以证据限定范围为准 |
| `debug_name` | 6 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/natural_cubic.rs

[reconstruction/shared-foundation/crates/utils/src/natural_cubic.rs](../../reconstruction/shared-foundation/crates/utils/src/natural_cubic.rs)

SHA256：`400febb381f2461a3d45ef6b72d0a246c566ce7bb08a5339226cbfea81bcf59e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `position_at_local_t` | 10 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_local_t` | 14 | 未由扫描证明；以证据限定范围为准 |
| `position_at_local_t` | 24 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_local_t` | 28 | 未由扫描证明；以证据限定范围为准 |
| `knots` | 46 | 未由扫描证明；以证据限定范围为准 |
| `second_derivatives` | 66 | 未由扫描证明；以证据限定范围为准 |
| `tridiagonal` | 98 | 未由扫描证明；以证据限定范围为准 |
| `periodic_second_derivatives` | 115 | 未由扫描证明；以证据限定范围为准 |
| `coefficients` | 145 | 未由扫描证明；以证据限定范围为准 |
| `from_control_points_with_tangents` | 188 | 未由扫描证明；以证据限定范围为准 |
| `from_periodic_loop` | 221 | 未由扫描证明；以证据限定范围为准 |
| `from_control_points_with_tangents` | 246 | 未由扫描证明；以证据限定范围为准 |
| `segment_at_t` | 279 | 未由扫描证明；以证据限定范围为准 |
| `position_at_t` | 288 | 未由扫描证明；以证据限定范围为准 |
| `tangent_at_t` | 292 | 未由扫描证明；以证据限定范围为准 |
| `to_owned` | 296 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/rdp.rs

[reconstruction/shared-foundation/crates/utils/src/rdp.rs](../../reconstruction/shared-foundation/crates/utils/src/rdp.rs)

SHA256：`98e7acef73ed3e64dfe29ef3c97ec5f43f6163289a446f9e423a8212bd8b1cc5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_vec3` | 5 | 未由扫描证明；以证据限定范围为准 |
| `as_vec3` | 8 | 未由扫描证明；以证据限定范围为准 |
| `as_vec3` | 13 | 未由扫描证明；以证据限定范围为准 |
| `new` | 24 | 未由扫描证明；以证据限定范围为准 |
| `stitch_with` | 30 | 未由扫描证明；以证据限定范围为准 |
| `new_with_opts` | 38 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/rgb9e5.rs

[reconstruction/shared-foundation/crates/utils/src/rgb9e5.rs](../../reconstruction/shared-foundation/crates/utils/src/rgb9e5.rs)

SHA256：`06063e7fbd43eaf38c5695ff85b1e9a4f87be273ebef060af3b7da8fcbc0b1af`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `sse_max` | 4 | 未由扫描证明；以证据限定范围为准 |
| `sse_min` | 5 | 未由扫描证明；以证据限定范围为准 |
| `pack_rgb9e5` | 6 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/rng.rs

[reconstruction/shared-foundation/crates/utils/src/rng.rs](../../reconstruction/shared-foundation/crates/utils/src/rng.rs)

SHA256：`07ac85d69572d5257812dabc93db19a90232c08df12d4c5b8b489ac3171ca08e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `init_rng_u64` | 4 | 未由扫描证明；以证据限定范围为准 |
| `init_rng` | 7 | 未由扫描证明；以证据限定范围为准 |
| `rng_from_seed_u64` | 10 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/selective_deserialize.rs

[reconstruction/shared-foundation/crates/utils/src/selective_deserialize.rs](../../reconstruction/shared-foundation/crates/utils/src/selective_deserialize.rs)

SHA256：`4da495878b1fb10e356a40565d956c6926b1fb6efeb917f3630a226e779beec4`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `deserialize` | 14 | 未由扫描证明；以证据限定范围为准 |
| `expecting` | 18 | 未由扫描证明；以证据限定范围为准 |
| `visit_str` | 21 | 未由扫描证明；以证据限定范围为准 |
| `visit_string` | 24 | 未由扫描证明；以证据限定范围为准 |
| `visit_bytes` | 27 | 未由扫描证明；以证据限定范围为准 |
| `visit_byte_buf` | 32 | 未由扫描证明；以证据限定范围为准 |
| `default` | 61 | 未由扫描证明；以证据限定范围为准 |
| `into_inner` | 69 | 未由扫描证明；以证据限定范围为准 |
| `deserialize` | 76 | 未由扫描证明；以证据限定范围为准 |
| `expecting` | 83 | 未由扫描证明；以证据限定范围为准 |
| `visit_map` | 86 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L118：let input=br#"{"unknown":{"deep":[1,2]},"History":{"snap\u0073hots":[7,9],"snapshots":"bad"},"broken": }"#;
### reconstruction/shared-foundation/crates/utils/src/subtractive_1d_boolean.rs

[reconstruction/shared-foundation/crates/utils/src/subtractive_1d_boolean.rs](../../reconstruction/shared-foundation/crates/utils/src/subtractive_1d_boolean.rs)

SHA256：`4565b4529dae9aadef75964b9d2b3fae5e18d7f62321362d757abd418664ac2e`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new_range` | 13 | 未由扫描证明；以证据限定范围为准 |
| `result` | 22 | 未由扫描证明；以证据限定范围为准 |
| `subtract` | 25 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/swizzlers.rs

[reconstruction/shared-foundation/crates/utils/src/swizzlers.rs](../../reconstruction/shared-foundation/crates/utils/src/swizzlers.rs)

SHA256：`04924394691bc65e08f23c77532176c99a2febec67a06121a4be2c9fef0f59cf`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `x0y` | 5 | 未由扫描证明；以证据限定范围为准 |
| `x0y` | 8 | 未由扫描证明；以证据限定范围为准 |
| `xvy` | 13 | 未由扫描证明；以证据限定范围为准 |
| `xvy` | 16 | 未由扫描证明；以证据限定范围为准 |
| `xvz` | 21 | 未由扫描证明；以证据限定范围为准 |
| `xvz` | 24 | 未由扫描证明；以证据限定范围为准 |
| `x0z` | 29 | 未由扫描证明；以证据限定范围为准 |
| `x0z` | 32 | 未由扫描证明；以证据限定范围为准 |
| `x0y` | 37 | 未由扫描证明；以证据限定范围为准 |
| `xvy` | 40 | 未由扫描证明；以证据限定范围为准 |
| `xvz` | 43 | 未由扫描证明；以证据限定范围为准 |
| `x0z` | 46 | 未由扫描证明；以证据限定范围为准 |
| `as_source` | 59 | 未由扫描证明；以证据限定范围为准 |
| `as_vec2` | 64 | 未由扫描证明；以证据限定范围为准 |
| `as_vec2` | 67 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/threshold_range.rs

[reconstruction/shared-foundation/crates/utils/src/threshold_range.rs](../../reconstruction/shared-foundation/crates/utils/src/threshold_range.rs)

SHA256：`5ca1547b7eaf9487552958e30bd58e23cad75c419aadb0df4445b732410d0579`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `threshold_range` | 5 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/timer.rs

[reconstruction/shared-foundation/crates/utils/src/timer.rs](../../reconstruction/shared-foundation/crates/utils/src/timer.rs)

SHA256：`6928585c8d83bb88fb5bae8f8b322d6a4cd4bc60a9fea34cc5f1ce54190d90cf`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `default` | 10 | 未由扫描证明；以证据限定范围为准 |
| `from_observed_state` | 22 | 未由扫描证明；以证据限定范围为准 |
| `samples` | 33 | 未由扫描证明；以证据限定范围为准 |
| `delta_secs` | 36 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/shared-foundation/crates/utils/src/wall_space.rs

[reconstruction/shared-foundation/crates/utils/src/wall_space.rs](../../reconstruction/shared-foundation/crates/utils/src/wall_space.rs)

SHA256：`f12760634a7163c022f5efbf36c90834d9447ab3f5b321308023a3dc675cc619`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs

[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs)

SHA256：`660112550fd4ba22a64a82ef070648c762893a88ee46f72d584f20267e17d346`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `lerp` | 10 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs

[skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs](../../skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs)

SHA256：`8d8b3774a46daf838d0bc3e35fa7cf2a525a44b9e3159a7c6ff46845c1e971a5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `inverse_lerp` | 10 | 未由扫描证明；以证据限定范围为准 |
| `fit` | 18 | 未由扫描证明；以证据限定范围为准 |
| `intersects` | 28 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs

[skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs](../../skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs)

SHA256：`baf4c73ddede219f78afa2bbdd706735c8ee822041c07f18e71e6582569a8ef9`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `apply` | 11 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs

[skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs](../../skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs)

SHA256：`81f6a5eb14ffab8979b65533de8563fd628a2b447c83a8c769d851532a4a08d5`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `xvy` | 10 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs

[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs)

SHA256：`3a085a8827a4e19fb1b94f9d0976ed2ca8dd00f6425a56a5c43920e60af93c79`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `len` | 10 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs

[skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs](../../skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs)

SHA256：`ca0601e143815df74d170669347d6a6cc17fc2d3ebd6a7885a56aa187929a858`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `start` | 24 | 未由扫描证明；以证据限定范围为准 |
| `end` | 34 | 未由扫描证明；以证据限定范围为准 |
| `is_empty` | 44 | 未由扫描证明；以证据限定范围为准 |
| `next_back` | 52 | 未由扫描证明；以证据限定范围为准 |
| `raw_idx` | 68 | 未由扫描证明；以证据限定范围为准 |
| `sub` | 75 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs

[skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs](../../skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs)

SHA256：`57fef91c880cf192645e1cd9ab4fc8c9916149c2bc5ab6503e5d5920c2454256`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `init_rng` | 10 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs

[skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs](../../skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs)

SHA256：`db2b50389d26ad1e93bd6cbd7d163c7a216e4d3f84f56f34d54228d4fc62f847`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `init_rng_u64` | 12 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs

[skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs](../../skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs)

SHA256：`e5e29424f43a23cba6e8defd86cc29a0b3d586fd5f96b9e3c2ae7e33707d1a07`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `with_y_offset` | 15 | 未由扫描证明；以证据限定范围为准 |
| `with_y` | 25 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs

[skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs](../../skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs)

SHA256：`fd71dae8935007599ded96930e9ce85d521a74cb88ad154b20918537a7e7f322`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `result` | 15 | 未由扫描证明；以证据限定范围为准 |
