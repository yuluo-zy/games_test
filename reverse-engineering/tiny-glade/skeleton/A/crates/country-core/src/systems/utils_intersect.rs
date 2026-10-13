// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/utils_intersect.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 782dbb61c67192e34accb2349a0d3d6d8b813f6decdb2d255a048c2470060470
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: utils_intersect.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1403
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::utils_intersect::line_x_shape_overlap
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1403,"record_offset":9788,"rva":17058128,"va":5385767248}
// 编译实例 RVAs: 0x1044950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: utils_intersect.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17826311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e807021001","size":5,"soft_fail":false,"target_va":5403594144,"va":5385767828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17211001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e8799e0601","size":5,"soft_fail":false,"target_va":5402979152,"va":5385768146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1347569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e80f70ebff","size":5,"soft_fail":false,"target_va":5384420000,"va":5385767564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1347134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e8c271ebff","size":5,"soft_fail":false,"target_va":5384420448,"va":5385767577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::RayBoxIntersection::is_hit
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1345420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e87478ebff","size":5,"soft_fail":false,"target_va":5384422192,"va":5385767607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::intersect_ray
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7799268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e81cfe88ff","size":5,"soft_fail":false,"target_va":5377968480,"va":5385767743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7811937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e89fcc88ff","size":5,"soft_fail":false,"target_va":5377955440,"va":5385767372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-8828087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385767248,"raw_hex":"e8494b79ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5385768034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1403,"record_offset":10852,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::utils_intersect::line_x_shape_overlap::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/utils_intersect.rs
//     evidence={"module":1403,"record_offset":10588,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":65028,"source_line":19,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::inverse
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1403,"record_offset":9948,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":45796,"source_line":255,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1403,"record_offset":10212,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1403,"record_offset":10156,"parent_record_offset":9788,"ancestor_record_offset":9788,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1403,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":9788,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1403,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":9788,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":10068,"parent_record_offset":9948,"ancestor_record_offset":9788,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":10012,"parent_record_offset":9948,"ancestor_record_offset":9788,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1403,"record_offset":9984,"parent_record_offset":9948,"ancestor_record_offset":9788,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1403,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9788,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1403,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":9788,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1403,"record_offset":10644,"parent_record_offset":10616,"ancestor_record_offset":9788,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1403,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":9788,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1403,"record_offset":10616,"parent_record_offset":10588,"ancestor_record_offset":9788,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11116,"parent_record_offset":11064,"ancestor_record_offset":9788,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11208,"parent_record_offset":11064,"ancestor_record_offset":9788,"inlinee":24314,"source_line":48,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":9788,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::utils_intersect::line_x_shape_overlap::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFE04
// 首条位置: {"module":1403,"inlinee":65028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1403,"record_offset":10616,"parent_record_offset":10588,"ancestor_record_offset":10588,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11116,"parent_record_offset":11064,"ancestor_record_offset":11064,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::subtract
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11208,"parent_record_offset":11064,"ancestor_record_offset":11064,"inlinee":24314,"source_line":48,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1403,"record_offset":10692,"parent_record_offset":10668,"ancestor_record_offset":10588,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1403,"record_offset":10716,"parent_record_offset":10692,"ancestor_record_offset":10588,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1403,"record_offset":10668,"parent_record_offset":10644,"ancestor_record_offset":10588,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1403,"record_offset":10644,"parent_record_offset":10616,"ancestor_record_offset":10588,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1403,"record_offset":11168,"parent_record_offset":11144,"ancestor_record_offset":11064,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11232,"parent_record_offset":11208,"ancestor_record_offset":11064,"inlinee":24269,"source_line":41,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1403,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":11064,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01403.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: utils_intersect.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: utils_intersect.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: utils_intersect.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::systems::utils_intersect::line_x_shape_overlap
// 信息 ID: 58a9aeac86b5a72574738024
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::find_overlap_segments::find_overhang_support_segments @ 0xc93859 -> 0x1044950 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons @ 0x14e132c -> 0x1044950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::utils_intersect::line_x_shape_overlap::closure$0
// 信息 ID: d44f2e7cdbd52083ca61a8ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::utils_intersect::line_x_shape_overlap", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=1403, inline_record=10588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=1403, inline_record=10884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::utils_intersect::line_x_shape_overlap; module=1403, inline_record=11064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
