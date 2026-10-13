# utils 顶层 22 文件：原 B 接口覆盖审计

已观察私有接口 **191** 项；有逐接口业务分析 **99** 项；严格未触及 **92** 项。候选实现存在 103 项，静态业务比较 96 项，直接原入口对照 20 项。这些状态交叉，不能相加，也不是完整游戏百分比。

独立记录原内联片段差分 **1** 项；它不混入直接原入口数。

已知但无对应候选实现 **88** 项；另有 **4** 项仅存在适配器、缺原接口业务分析，仍列未触及。

分母来自已有 B LLVM compiled/inline/declaration 元数据，不是 Rust 当前函数声明数。仅定位、metadata 或字节转储不算业务分析；有候选适配器但未验证原接口的仍列未触及。私有闭包另列，库/派生/Bevy forwarding 排除；业务 Local init_state 保留。Rust hash和泛型实例合并，但未解析 impl$N owner 保留区分；无证据不猜类型。

| 原 B 接口 | 业务分析 | 候选实现 | 静态比较 | 原码对照 | 接线 | 原入口/PDB | 缺口 |
|---|---|---|---|---|---|---|---|
| `utils::angle_range::AngleRange::clamp` | 部分已分析 | AngleRange::clamp | 是 | 直接原入口 | 否 | 0x1425f24d0 | Serde/wrap constructor edge policies outside fixture scope; A/C untested |
| `utils::angle_range::AngleRange::from_min_and_len` | 部分已分析 | AngleRange::from_min_and_len | 是 | 否 | 否 | 0x1425f2460 | Serde/wrap constructor edge policies outside fixture scope; A/C untested |
| `utils::angle_range::wrap_angle` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<$LP$glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat$RP$ as utils::camera::IntoCameraBodyMatrices>::into_camera_body_matrices` | **未触及** | 无 | 否 | 否 | 否 | 0x140c8ff70 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<utils::camera::TaaJitterSet as core::default::Default>::default` | **未触及** | 无 | 否 | 否 | 否 | 0x140c97500 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::camera::CameraBodyMatrices::from_position_rotation` | 部分已分析 | CameraBodyMatrices::from_position_rotation | 是 | 否 | 否 | 0x140c96fe0 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraLens::immediately_set_vertical_fov_degrees` | 部分已分析 | CameraLens::immediately_set_vertical_fov_degrees | 是 | 否 | 否 | 0x140c974f0 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraLens::new` | 部分已分析 | CameraLens::new | 是 | 否 | 否 | 0x140c97480 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraMatrices::aspect_ratio` | 部分已分析 | CameraMatrices::aspect_ratio | 是 | 否 | 否 | 0x140c96e60 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraMatrices::eye_direction` | 部分已分析 | CameraMatrices::eye_direction | 是 | 否 | 否 | 0x140c96d80 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraMatrices::eye_position` | 部分已分析 | CameraMatrices::eye_position | 是 | 否 | 否 | 0x140c96d40 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraMatrices::eye_right` | 部分已分析 | CameraMatrices::eye_right | 是 | 否 | 否 | 0x140c96df0 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraMatrices::xz_projective_rotation` | 部分已分析 | CameraMatrices::xz_projective_rotation | 是 | 直接原入口 | 否 | 0x140c96e70 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::CameraProjectionParams::calc_matrices` | 部分已分析 | CameraProjectionParams::calc_matrices | 是 | 直接原入口 | 否 | 0x140c97390 | Lens smoothing/update/input producer and camera resource assembly not recovered; body matrix constructor only static review |
| `utils::camera::impl$12::default` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::camera::impl$3::through` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<utils::circular_f32::_::::deserialize::__FieldVisitor as serde::de::Visitor>::expecting` | **未触及** | 无 | 否 | 否 | 否 | 0x1416f3460 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<utils::circular_f32::_::::deserialize::__Visitor as serde::de::Visitor>::expecting` | **未触及** | 无 | 否 | 否 | 否 | 0x1416f3480 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::CircularF32::as_f32` | 部分已分析 | CircularF32::as_f32 | 是 | 否 | 否 | 0x1416f3210 | Constructor serde normalization, NaN/tie endpoint policies not exhaustive |
| `utils::circular_f32::CircularF32::distance` | 部分已分析 | CircularF32::distance | 是 | 否 | 否 | 0x1416f3260 | Constructor serde normalization, NaN/tie endpoint policies not exhaustive |
| `utils::circular_f32::CircularF32::lerp` | 部分已分析 | CircularF32::lerp | 是 | 直接原入口 | 否 | 0x1416f3330 | Constructor serde normalization, NaN/tie endpoint policies not exhaustive |
| `utils::circular_f32::CircularF32::new` | 部分已分析 | CircularF32::new | 是 | 否 | 否 | 0x1416f31d0 | Constructor serde normalization, NaN/tie endpoint policies not exhaustive |
| `utils::circular_f32::CircularF32::project_unbounded` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::CircularF32::set_value` | 部分已分析 | CircularF32::set_value | 是 | 否 | 否 | 0x1416f3180 | Constructor serde normalization, NaN/tie endpoint policies not exhaustive |
| `utils::circular_f32::_::_::serialize` | **未触及** | 无 | 否 | 否 | 否 | 0x1400efd00, 0x140a44920, 0x140a44a00 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::impl$3::fmt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::circular_f32::impl$6::eq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<utils::comparative_space::CircularComparativeSpace as utils::comparative_space::ComparativeSpace>::distance` | 部分已分析 | distance | 是 | 否 | 否 | 0x142007170 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `utils::comparative_space::ComparativeSpace::lerp_clamp` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$0::classify_side` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$0::regions_intersect` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$14::fmt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$1::distance` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$2::distance` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$2::project` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$2::project_clamp` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$2::subtract` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$3::project` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$3::project_clamp` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$3::project_unbounded` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$3::subtract` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::comparative_space::impl$9::fmt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::EasingFunction::apply` | 部分已分析 | EasingFunction::apply | 是 | 直接原入口 | 否 | 0x140c95ef0 | Out-of-range/NaN input and Bias constructors not exhaustively compared |
| `utils::easing_functions::circular_in_out` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::cubic_in_out` | 部分已分析 | cubic_in_out | 是 | 否 | 否 | 0x140c961e0 | Out-of-range/NaN input and Bias constructors not exhaustively compared |
| `utils::easing_functions::cubic_out` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::ease_in_out_elastic` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::ease_out_back` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::shoulder` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::easing_functions::sine_in_out` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<std::path::Path as utils::file::PathTail>::tail` | **未触及** | 无 | 否 | 否 | 否 | 0x1416f6ac0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::file::copy_file_to_backup` | 部分已分析 | copy_file_to_backup | 是 | 否 | 否 | 0x140c96a80 | Filesystem failure/result/logging and non-UTF8 paths not native differential |
| `utils::file::date_time_string_for_filename` | 部分已分析 | date_time_string_for_filename | 是 | 否 | 否 | 0x140c96690 | Filesystem failure/result/logging and non-UTF8 paths not native differential |
| `utils::file::generate_backup_file_path` | 部分已分析 | generate_backup_file_path | 是 | 否 | 否 | 0x140c968b0 | Filesystem failure/result/logging and non-UTF8 paths not native differential |
| `<T as utils::float_grid::FloatGridSampler>::bilinear_sample_at_world_xz` | 部分已分析 | FloatGridSampler::bilinear_sample_at_world_xz | 是 | 否 | 否 | 0x140257ba0, 0x140a14df0, 0x14261f0c0 | Two policies reviewed, 11 public instances merge by interface; preserve dynamic world/512byte-grid and sampling_fix vs legacy strategies; input-format specialization and full differential unresolved. |
| `utils::float_grid::FloatGrid::load_unchecked` | 部分已分析 | FloatGrid::load_index_unchecked via explicit storage adapter | 否 | 否 | 否 | inline/IPI；无独立入口 | Trait delegates to real terrain/raster storage; candidate name/interface differs; original producer not wired. |
| `<[f32; 3] as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x141940140 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `<[f32; 4] as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x141940180 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `<f32 as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x1408b9f20 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `<glam::f32::sse2::quat::Quat as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x1420071b0 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `<glam::f32::sse2::vec4::Vec4 as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x1420071f0 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `<glam::f32::vec3::Vec3 as utils::halver::Halver>::to_f16` | 部分已分析 | Halver::to_f16 | 是 | 否 | 否 | 0x140c8ffb0 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `utils::halver::impl$1::to_f16` | 部分已分析 | Halver::to_f16 | 否 | 否 | 否 | inline/IPI；无独立入口 | Original half conversion reused; exact inline instance type and producer unresolved where no native/public binding. |
| `utils::index_condenser::Condex::last_known_index` | 部分已分析 | utils::index_condenser::Condex::last_known_index | 是 | 否 | 否 | inline/IPI；无独立入口 | Original IPI0x74F0 exposes last cached index in producer register; safe Cell getter restores value policy, no independent native method entry/ABI. |
| `utils::index_condenser::IndexCondenser::clear` | 部分已分析 | utils::index_condenser::IndexCondenser::clear | 是 | 否 | 否 | inline/IPI；无独立入口 | Original clear confirmed inside uninit_clearing, calling original Vec/HashMap clear; candidate capacity/reset tests only, full original startup consumer not executed. |
| `utils::index_condenser::IndexCondenser::get` | 部分已分析 | utils::index_condenser::IndexCondenser::get | 是 | 原内联片段 | 是 | inline/IPI；无独立入口 | No standalone original get entry: original inlined code compared at precise IPI0x74E3 boundaries. Recovered remove now calls get; full GPUWorld update consumer remains unwired. |
| `utils::index_condenser::IndexCondenser::insert` | **未触及** | candidate adapter; original method binding unverified | 否 | 否 | 否 | inline/IPI；无独立入口 | Candidate invariant producer exists; original inline insert policy not compared. |
| `utils::index_condenser::IndexCondenser::remove` | 部分已分析 | IndexCondenser::remove | 是 | 否 | 否 | 0x140a19f50 | Insert/default producer key policy and full GPUWorld/startup consumer integration not established; clear/getter no independent original entry differential |
| `utils::index_condenser::impl$3::default` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::index_condenser::impl$6::fmt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::local_time::init_local_time` | 部分已分析 | init_local_time | 是 | 否 | 否 | 0x140c91d60 | Startup call frequency and actual platform offset failure/refresh workflow not native runtime |
| `utils::local_time::now_appox_local` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::local_time::system_time_to_local_time` | 部分已分析 | system_time_to_local_time | 是 | 否 | 否 | 0x140c91d90 | Startup call frequency and actual platform offset failure/refresh workflow not native runtime |
| `<utils::loop_range::LoopIdx as core::ops::arith::Sub>::sub` | **未触及** | candidate adapter; original method binding unverified | 否 | 否 | 否 | 0x1412002e0 | Candidate LoopIdx::sub exists; trait receiver/type binding and independent method review unresolved. |
| `<utils::loop_range::LoopRange as core::iter::traits::double_ended::DoubleEndedIterator>::next_back` | **未触及** | candidate adapter; original method binding unverified | 否 | 否 | 否 | 0x141200320 | Candidate Iterator::next_back exists; exact iterator owner binding/edge behavior unresolved. |
| `<utils::loop_range::LoopRange as core::iter::traits::iterator::Iterator>::next` | **未触及** | candidate adapter; original method binding unverified | 否 | 否 | 否 | 0x1412002f0 | Candidate Iterator::next exists; original specialized iterator policy not individually verified. |
| `<utils::loop_range::LoopSlice as core::iter::traits::iterator::Iterator>::next` | **未触及** | 无 | 否 | 否 | 否 | 0x141208e10 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::LoopIdx::index` | 部分已分析 | LoopIdx::index | 是 | 直接原入口 | 否 | 0x141200280 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopIdx::raw_idx` | 部分已分析 | LoopIdx::raw_idx | 是 | 否 | 否 | 0x140bece40 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::end` | 部分已分析 | LoopRange::end | 是 | 否 | 否 | 0x141200380 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::is_empty` | 部分已分析 | LoopRange::is_empty | 是 | 否 | 否 | 0x141200360 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::len` | 部分已分析 | LoopRange::len | 是 | 否 | 否 | 0x141200350 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::map_slice` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::LoopRange::new_line` | 部分已分析 | LoopRange::new_line | 是 | 否 | 否 | 0x141200450 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::new_loop` | 部分已分析 | LoopRange::new_loop | 是 | 否 | 否 | 0x141200390 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::start` | 部分已分析 | LoopRange::start | 是 | 否 | 否 | 0x141200370 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopRange::subslice` | 部分已分析 | LoopRange::subslice | 是 | 否 | 否 | 0x141200510 | Full iterator ABI and exceptional/zero period paths incomplete |
| `utils::loop_range::LoopSlice::len` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::impl$13::clone` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::impl$16::fmt` | **未触及** | 无 | 否 | 否 | 否 | 0x14122fee0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::impl$7::next_back` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::loop_range::impl$8::index` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `<utils::name_hash::NameHash as core::cmp::PartialEq>::eq` | **未触及** | 无 | 否 | 否 | 否 | 0x140c9cd70 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::name_hash::NameHash::compute` | 部分已分析 | NameHash::compute | 是 | 否 | 否 | 0x140c9cd80 | Actual asset-name producer and hash consumer integration unverified |
| `utils::name_hash::NameHash::debug_name` | 部分已分析 | NameHash::debug_name | 是 | 否 | 否 | 0x140c9cdb0 | Actual asset-name producer and hash consumer integration unverified |
| `utils::name_hash::impl$0::hash` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::name_hash::impl$7::fmt` | **未触及** | 无 | 否 | 否 | 否 | 0x1409396e0, 0x14206f290 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::natural_cubic::Spline2D::find_segment_and_dt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::natural_cubic::Spline2D::from_control_points_with_tangents` | 部分已分析 | Spline2D::from_control_points_with_tangents | 是 | 直接原入口 | 否 | 0x140c926b0 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline2D::from_periodic_loop` | 部分已分析 | Spline2D::from_periodic_loop | 是 | 直接原入口 | 否 | 0x140c930f0 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline3D::find_segment_and_dt` | 部分已分析 | Spline3D::segment_at_t | 是 | 否 | 是 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline3D::from_control_points_with_tangents` | 部分已分析 | Spline3D::from_control_points_with_tangents | 是 | 直接原入口 | 否 | 0x140c93970 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline3D::position_at_t` | 部分已分析 | Spline3D::position_at_t | 是 | 直接原入口 | 否 | 0x140c946a0 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline3D::tangent_at_t` | 部分已分析 | Spline3D::tangent_at_t | 是 | 直接原入口 | 否 | 0x140c94800 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::Spline3D::to_owned` | 部分已分析 | Spline3D::to_owned | 是 | 否 | 否 | 0x140c94940 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::SplineSegment2D::position_at_local_t` | 部分已分析 | SplineSegment2D::position_at_local_t | 是 | 否 | 否 | 0x140c92610 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::SplineSegment2D::tangent_at_local_t` | 部分已分析 | SplineSegment2D::tangent_at_local_t | 是 | 否 | 否 | 0x140c92660 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::SplineSegment3D::position_at_local_t` | 部分已分析 | SplineSegment3D::position_at_local_t | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::SplineSegment3D::tangent_at_local_t` | 部分已分析 | SplineSegment3D::tangent_at_local_t | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::build_knots_closed` | 部分已分析 | knots(periodic=true) | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::build_knots_open` | 部分已分析 | knots(periodic=false) | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::build_knots_open3` | 部分已分析 | knots(periodic=false) | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::compute_periodic_spline` | 部分已分析 | periodic_second_derivatives + coefficients | 是 | 否 | 否 | 0x140c95200 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::eval_cubic` | 部分已分析 | SplineSegment2D/3D::position_at_local_t | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::eval_cubic_deriv` | 部分已分析 | SplineSegment2D/3D::tangent_at_local_t | 是 | 否 | 否 | inline/IPI；无独立入口 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::solve_clamped_spline` | 部分已分析 | second_derivatives + coefficients | 是 | 否 | 否 | 0x140c94a10 | Direct original entry comparison only where flagged; internal helper comparison through constructor/evaluator chain is indirect. Degenerate/control duplication/ABI remain unknown. |
| `utils::natural_cubic::solve_cyclic` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::natural_cubic::solve_tridiagonal` | **未触及** | 无 | 否 | 否 | 否 | 0x140c95c60 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rdp::Rdp::into_indices` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rdp::Rdp::new` | **未触及** | 无 | 否 | 否 | 否 | 0x140ad1fe0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rdp::Rdp::new_with_opts` | 部分已分析 | Rdp::new_with_opts | 是 | 直接原入口 | 否 | 0x141fe0b90 | Vec2 adapter not native differential; nonfinite/negative epsilon/extreme magnitudes/duplicate control inputs untested |
| `utils::rdp::Rdp::stitch_with` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rdp::impl$1::clone` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rgb9e5::clamp_range_for_rgb9e5` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rgb9e5::floor_log2` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::rgb9e5::pack_rgb9e5` | 部分已分析 | pack_rgb9e5 | 是 | 直接原入口 | 否 | 0x1417c66b0 | NaN/infinity/signed-zero and complete serialized field consumers not exhaustive |
| `utils::rng::init_rng` | 部分已分析 | init_rng | 是 | 否 | 否 | 0x140c96240 | Native typed ABI not claimed; downstream generation distributions/shuffle consumers remain external library |
| `utils::rng::init_rng_u64` | 部分已分析 | init_rng | 是 | 否 | 否 | 0x140c96270 | Native typed ABI not claimed; downstream generation distributions/shuffle consumers remain external library |
| `<utils::selective_deserialize::SelectiveDeserialize as serde::de::Deserialize>::deserialize` | 部分已分析 | SelectiveDeserialize::deserialize | 是 | 否 | 否 | 0x1413e4830, 0x1413e4990 | Outer wrapper original error/default policy compared; inner map policy inferred from exact History/snapshots constants and tested candidate only; full visitor/runtime and payload type unresolved. |
| `utils::selective_deserialize::impl$0::expecting` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::selective_deserialize::impl$0::visit_map` | 部分已分析 | nested SelectField::visit_map | 否 | 否 | 否 | inline/IPI；无独立入口 | Outer wrapper original error/default policy compared; inner map policy inferred from exact History/snapshots constants and tested candidate only; full visitor/runtime and payload type unresolved. |
| `utils::selective_deserialize::impl$4::visit_str` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::selective_deserialize::impl$5::deserialize` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range` | 部分已分析 | SubtractiveBoolean1D::new_range | 是 | 否 | 否 | 0x141fdc2d0 | Generated old upper_exclusive states, multiple sequential cuts and nonfinite/reversed ranges not exhaustive |
| `utils::subtractive_1d_boolean::SubtractiveBoolean1D::result` | 部分已分析 | SubtractiveBoolean1D::result | 是 | 否 | 否 | 0x140c37760 | Generated old upper_exclusive states, multiple sequential cuts and nonfinite/reversed ranges not exhaustive |
| `utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract` | 部分已分析 | SubtractiveBoolean1D::subtract | 是 | 直接原入口 | 否 | 0x141fdc350 | Generated old upper_exclusive states, multiple sequential cuts and nonfinite/reversed ranges not exhaustive |
| `<[glam::f32::vec3::Vec3] as utils::swizzlers::Vec3SliceAs2>::as_vec2` | 部分已分析 | utils::swizzlers::Vec3SliceAs2::as_vec2 | 是 | 直接原入口 | 否 | 0x14067dd80 | Original return wrapper type, projected axis/element stride/borrow aliases and actual consumer interpretation unresolved; safe borrowed transport does not cast &[Vec3] to packed &[Vec2]. |
| `<glam::f32::vec2::Vec2 as utils::swizzlers::SwizzlerX0Y>::x0y` | 部分已分析 | utils::swizzlers::SwizzlerX0Y::x0y | 是 | 直接原入口 | 否 | 0x140c9cdc0 | Original Rust receiver-by-value/reference ABI and full game consumer wiring unverified; coordinate values/lane rules verified. |
| `<glam::f32::vec2::Vec2 as utils::swizzlers::SwizzlerXVY>::xvy` | 部分已分析 | utils::swizzlers::SwizzlerXVY::xvy | 是 | 直接原入口 | 否 | 0x140c9cde0 | Original Rust receiver-by-value/reference ABI and full game consumer wiring unverified; coordinate values/lane rules verified. |
| `<glam::f32::vec3::Vec3 as utils::swizzlers::SwizzlerX0Z>::x0z` | 部分已分析 | utils::swizzlers::SwizzlerX0Z::x0z | 是 | 直接原入口 | 否 | 0x140c90030 | Original Rust receiver-by-value/reference ABI and full game consumer wiring unverified; coordinate values/lane rules verified. |
| `<glam::f32::vec3::Vec3 as utils::swizzlers::SwizzlerXVZ>::xvz` | 部分已分析 | utils::swizzlers::SwizzlerXVZ::xvz | 是 | 直接原入口 | 否 | 0x140c90010 | Original Rust receiver-by-value/reference ABI and full game consumer wiring unverified; coordinate values/lane rules verified. |
| `utils::threshold_range::threshold_range` | 部分已分析 | threshold_range | 是 | 否 | 否 | 0x1411ff790, 0x1411ffbd0, 0x1411fff30 | NaN singleton high policy and callback retention behavior lack complete original-entry differential |
| `utils::timer::FilteredWallTime::delta_secs` | 部分已分析 | FilteredWallTime::delta_secs | 是 | 直接原入口 | 否 | 0x141799e70 | Standalone state/Default expresses original Local values; Bevy SystemParam integration and Time resource production not yet wired |
| `utils::timer::TimeFilter::update` | 部分已分析 | FilteredWallTime::delta_secs private filter body | 是 | 否 | 否 | inline/IPI；无独立入口 | Inline TimeFilter update represented inside standalone filter; original 140-step caller oracle covers behavior indirectly; Bevy integration missing. |
| `utils::timer::_::_::init_state` | 部分已分析 | FilteredWallTime::default game Local values only | 是 | 否 | 否 | 0x14179a010 | Recover only warmup=1/dt1/60/empty deque-cap4; original SystemParam callable registration deliberately delegated to original Bevy and not wired. |
| `utils::timer::impl$2::default` | 部分已分析 | FilteredWallTime::default | 是 | 否 | 否 | inline/IPI；无独立入口 | Original Local values established by init_state; impl$2 exact Rust owner/ABI unresolved. Values not library Default reimplementation. |
| `<utils::wall_space::WallSpace as utils::comparative_space::ComparativeSpace>::distance` | 部分已分析 | distance | 是 | 否 | 否 | 0x140c91df0 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `<utils::wall_space::WallSpace as utils::comparative_space::ComparativeSpace>::project` | 部分已分析 | project | 是 | 否 | 否 | 0x140c91e40 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `<utils::wall_space::WallSpace as utils::comparative_space::ComparativeSpace>::project_clamp` | 部分已分析 | project_clamp | 是 | 否 | 否 | 0x140c91eb0 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `<utils::wall_space::WallSpace as utils::comparative_space::ComparativeSpace>::project_unbounded` | 部分已分析 | project_unbounded | 是 | 否 | 否 | 0x140c91f20 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `<utils::wall_space::WallSpace as utils::comparative_space::ComparativeSpace>::subtract` | 部分已分析 | subtract | 是 | 否 | 否 | 0x140c91f60 | Concrete trait owner bound by original compiled address; static game policy reviewed, full interface/consumer differential remains incomplete. |
| `<utils::wall_space::_::::deserialize::__Visitor as serde::de::Visitor>::expecting` | **未触及** | 无 | 否 | 否 | 否 | 0x140c925f0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpace::space_length` | 部分已分析 | WallSpace::space_length | 是 | 否 | 否 | 0x140c91de0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::as_vec2` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::bound` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::from_curve_internal_coord` | 部分已分析 | WallSpaceCoord::from_curve_internal_coord | 是 | 否 | 否 | 0x140c92000 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::from_world_space` | 部分已分析 | WallSpaceCoord::from_world_space | 是 | 否 | 否 | 0x140c91fb0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::get_pos_xz` | 部分已分析 | WallSpaceCoord::get_pos_xz | 是 | 否 | 否 | 0x140c923b0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::get_tangent` | 部分已分析 | WallSpaceCoord::get_tangent | 是 | 否 | 否 | 0x140c924f0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::is_bounded` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::new` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::offset_x_clamped` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::subtract` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::to_affine3` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::to_world_space` | 部分已分析 | WallSpaceCoord::to_world_space | 是 | 否 | 否 | 0x140c92090 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::with_offset` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::with_offset_clamped` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::with_offset_unbounded` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceCoord::with_y` | 部分已分析 | WallSpaceCoord::with_y | 是 | 否 | 否 | 0x140c925e0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::with_y_offset` | 部分已分析 | WallSpaceCoord::with_y | 是 | 否 | 否 | 0x140c925d0 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::WallSpaceCoord::x` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::WallSpaceRegion::intersects` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::_::serialize` | **未触及** | 无 | 否 | 否 | 否 | 0x14048a020, 0x1416e76e0, 0x1416e77e0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::impl$0::deserialize` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::impl$0::deserialize::impl$0::visit_str` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::impl$0::deserialize::impl$1::deserialize` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::impl$0::deserialize::impl$2::visit_map` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::_::impl$0::deserialize::impl$2::visit_seq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::impl$10::eq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::impl$13::fmt` | **未触及** | 无 | 否 | 否 | 否 | 0x140a44b30, 0x1412763f0 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::impl$15::eq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::impl$26::eq` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |
| `utils::wall_space::impl$4::from` | 部分已分析 | WallSpaceCoord::new_unchecked | 是 | 否 | 否 | 0x14003bb80 | Wall consumers, extrapolation/tangent/nearest-u coordinate chain needs original differential |
| `utils::wall_space::impl$8::fmt` | **未触及** | 无 | 否 | 否 | 否 | inline/IPI；无独立入口 | No per-interface business analysis recorded; PDB name/location is inventory only. |

## 严格未触及接口

- `utils::angle_range::wrap_angle`
- `<$LP$glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat$RP$ as utils::camera::IntoCameraBodyMatrices>::into_camera_body_matrices`
- `<utils::camera::TaaJitterSet as core::default::Default>::default`
- `utils::camera::impl$12::default`
- `utils::camera::impl$3::through`
- `<utils::circular_f32::_::::deserialize::__FieldVisitor as serde::de::Visitor>::expecting`
- `<utils::circular_f32::_::::deserialize::__Visitor as serde::de::Visitor>::expecting`
- `utils::circular_f32::CircularF32::project_unbounded`
- `utils::circular_f32::_::_::serialize`
- `utils::circular_f32::_::impl$0::deserialize`
- `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_bytes`
- `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_str`
- `utils::circular_f32::_::impl$0::deserialize::impl$0::visit_u64`
- `utils::circular_f32::_::impl$0::deserialize::impl$1::deserialize`
- `utils::circular_f32::_::impl$0::deserialize::impl$2::visit_map`
- `utils::circular_f32::_::impl$0::deserialize::impl$2::visit_seq`
- `utils::circular_f32::impl$3::fmt`
- `utils::circular_f32::impl$6::eq`
- `utils::comparative_space::ComparativeSpace::lerp_clamp`
- `utils::comparative_space::impl$0::classify_side`
- `utils::comparative_space::impl$0::regions_intersect`
- `utils::comparative_space::impl$14::fmt`
- `utils::comparative_space::impl$1::distance`
- `utils::comparative_space::impl$2::distance`
- `utils::comparative_space::impl$2::project`
- `utils::comparative_space::impl$2::project_clamp`
- `utils::comparative_space::impl$2::subtract`
- `utils::comparative_space::impl$3::project`
- `utils::comparative_space::impl$3::project_clamp`
- `utils::comparative_space::impl$3::project_unbounded`
- `utils::comparative_space::impl$3::subtract`
- `utils::comparative_space::impl$9::fmt`
- `utils::easing_functions::circular_in_out`
- `utils::easing_functions::cubic_out`
- `utils::easing_functions::ease_in_out_elastic`
- `utils::easing_functions::ease_out_back`
- `utils::easing_functions::shoulder`
- `utils::easing_functions::sine_in_out`
- `<std::path::Path as utils::file::PathTail>::tail`
- `utils::index_condenser::IndexCondenser::insert`
- `utils::index_condenser::impl$3::default`
- `utils::index_condenser::impl$6::fmt`
- `utils::local_time::now_appox_local`
- `<utils::loop_range::LoopIdx as core::ops::arith::Sub>::sub`
- `<utils::loop_range::LoopRange as core::iter::traits::double_ended::DoubleEndedIterator>::next_back`
- `<utils::loop_range::LoopRange as core::iter::traits::iterator::Iterator>::next`
- `<utils::loop_range::LoopSlice as core::iter::traits::iterator::Iterator>::next`
- `utils::loop_range::LoopRange::map_slice`
- `utils::loop_range::LoopSlice::len`
- `utils::loop_range::impl$13::clone`
- `utils::loop_range::impl$16::fmt`
- `utils::loop_range::impl$7::next_back`
- `utils::loop_range::impl$8::index`
- `<utils::name_hash::NameHash as core::cmp::PartialEq>::eq`
- `utils::name_hash::impl$0::hash`
- `utils::name_hash::impl$7::fmt`
- `utils::natural_cubic::Spline2D::find_segment_and_dt`
- `utils::natural_cubic::solve_cyclic`
- `utils::natural_cubic::solve_tridiagonal`
- `utils::rdp::Rdp::into_indices`
- `utils::rdp::Rdp::new`
- `utils::rdp::Rdp::stitch_with`
- `utils::rdp::impl$1::clone`
- `utils::rgb9e5::clamp_range_for_rgb9e5`
- `utils::rgb9e5::floor_log2`
- `utils::selective_deserialize::impl$0::expecting`
- `utils::selective_deserialize::impl$4::visit_str`
- `utils::selective_deserialize::impl$5::deserialize`
- `<utils::wall_space::_::::deserialize::__Visitor as serde::de::Visitor>::expecting`
- `utils::wall_space::WallSpaceCoord::as_vec2`
- `utils::wall_space::WallSpaceCoord::bound`
- `utils::wall_space::WallSpaceCoord::is_bounded`
- `utils::wall_space::WallSpaceCoord::new`
- `utils::wall_space::WallSpaceCoord::offset_x_clamped`
- `utils::wall_space::WallSpaceCoord::subtract`
- `utils::wall_space::WallSpaceCoord::to_affine3`
- `utils::wall_space::WallSpaceCoord::with_offset`
- `utils::wall_space::WallSpaceCoord::with_offset_clamped`
- `utils::wall_space::WallSpaceCoord::with_offset_unbounded`
- `utils::wall_space::WallSpaceCoord::x`
- `utils::wall_space::WallSpaceRegion::intersects`
- `utils::wall_space::_::_::serialize`
- `utils::wall_space::_::impl$0::deserialize`
- `utils::wall_space::_::impl$0::deserialize::impl$0::visit_str`
- `utils::wall_space::_::impl$0::deserialize::impl$1::deserialize`
- `utils::wall_space::_::impl$0::deserialize::impl$2::visit_map`
- `utils::wall_space::_::impl$0::deserialize::impl$2::visit_seq`
- `utils::wall_space::impl$10::eq`
- `utils::wall_space::impl$13::fmt`
- `utils::wall_space::impl$15::eq`
- `utils::wall_space::impl$26::eq`
- `utils::wall_space::impl$8::fmt`

## 分母和证据限制

- Existing per-file .pdb.jsonl.gz and full _pdb/B archives referenced by skeleton are missing locally; inventory comments retain record locations and IPI references but this audit does not invent missing parameter/inline-body data.
- PDB void() is not Rust ABI. Some impl$N owners cannot be resolved; they remain distinct rather than merged by method name.
- Untouched means no recorded interface-level business analysis, even if an unrelated/self-designed adapter shares a name. Located PDB/bytes alone is not analysis.
- No exhaustive API denominator: fully optimized-away/unobserved methods can be missing. Observed denominator is a lower bound, not completion percentage.
- Direct native comparison flags exclude indirect constructor-chain coverage and candidate-only tests. consumer_wired only true for actual recovered in-crate original call links, never full original game.
- Private-owned Clone/Hash/fmt/Serde schema methods whose generated-versus-authored role is not independently established are retained for triage, not claimed as algorithms requiring hand implementation. Do not exclude them merely by trait name, or reimplement Serde/std to close them.

完整来源行号/入口、排除项和闭包见 [interface-coverage.json](interface-coverage.json)。本轮只审计已有证据，未改源码、未新增反编译。
