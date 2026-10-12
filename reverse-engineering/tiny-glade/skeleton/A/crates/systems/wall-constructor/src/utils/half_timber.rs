// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/utils/half_timber.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 1ae82601733cd8f173a811a5d7a81f0fd4d027333af9cc40ac35578bd376098b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: half_timber.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1226, 1227, 1249, 1263, 1264, 2698, 2703
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"PDB_registry_path_only","version":"0.10.5"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use take_until; // namespace candidate
//   package identities: [{"name":"take-until","source":"PDB_registry_path_only","version":"0.2.0"}]
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2698,"record_offset":2080,"rva":24597408,"va":5393306528}
// 编译实例 RVAs: 0x17753a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=2
//     target_aliases=1889 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24586650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e866d688fe","size":5,"soft_fail":false,"target_va":5368720672,"va":5393307317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23494694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e8da7f99fe","size":5,"soft_fail":false,"target_va":5369813552,"va":5393308241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16363730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e82e4f06ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5393307693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11437998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e8527851ff","size":5,"soft_fail":false,"target_va":5381870240,"va":5393308233,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11388575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e8613952ff","size":5,"soft_fail":false,"target_va":5381919744,"va":5393308314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10286894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e82ef79c00","size":5,"soft_fail":false,"target_va":5403594144,"va":5393307245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9670785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e881909300","size":5,"soft_fail":false,"target_va":5402979152,"va":5393308362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9670461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e83d8f9300","size":5,"soft_fail":false,"target_va":5402978944,"va":5393308478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17582770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e84eb5f3fe","size":5,"soft_fail":false,"target_va":5375724704,"va":5393307469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15339649","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e87fef15ff","size":5,"soft_fail":false,"target_va":5377967920,"va":5393307564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16366584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e8084406ff","size":5,"soft_fail":false,"target_va":5376941312,"va":5393307891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-11050758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393306528,"raw_hex":"e8fa6057ff","size":5,"soft_fail":false,"target_va":5382256592,"va":5393307345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":2698,"record_offset":2524,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2698,"record_offset":5084,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2698,"record_offset":2368,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2698,"record_offset":3420,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":4288,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3272,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3572,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3824,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3152,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3916,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":4008,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":2698,"record_offset":2396,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":2698,"record_offset":5112,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":59900,"source_line":214,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":2698,"record_offset":4500,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":2698,"record_offset":2944,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":2698,"record_offset":2680,"parent_record_offset":2080,"ancestor_record_offset":2080,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2698,"record_offset":3028,"parent_record_offset":3004,"ancestor_record_offset":2080,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2698,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2080,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2698,"record_offset":4992,"parent_record_offset":4528,"ancestor_record_offset":2080,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2698,"record_offset":4792,"parent_record_offset":4740,"ancestor_record_offset":2080,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":2698,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":2080,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2698,"record_offset":2712,"parent_record_offset":2680,"ancestor_record_offset":2080,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2698,"record_offset":3120,"parent_record_offset":2944,"ancestor_record_offset":2080,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":2698,"record_offset":2588,"parent_record_offset":2552,"ancestor_record_offset":2080,"inlinee":6042,"source_line":155,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2698,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":2080,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2698,"record_offset":2488,"parent_record_offset":2396,"ancestor_record_offset":2080,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2698,"record_offset":3948,"parent_record_offset":3916,"ancestor_record_offset":2080,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2698,"record_offset":3184,"parent_record_offset":3152,"ancestor_record_offset":2080,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2698,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2080,"inlinee":6226,"source_line":46,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2698,"record_offset":2552,"parent_record_offset":2524,"ancestor_record_offset":2080,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":2698,"record_offset":2612,"parent_record_offset":2588,"ancestor_record_offset":2080,"inlinee":13144,"source_line":160,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":2698,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":2080,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":2698,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":2080,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":2698,"record_offset":4528,"parent_record_offset":4500,"ancestor_record_offset":2080,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":2698,"record_offset":4572,"parent_record_offset":4528,"ancestor_record_offset":2080,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2698,"record_offset":4644,"parent_record_offset":4572,"ancestor_record_offset":2080,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2698,"record_offset":4740,"parent_record_offset":4528,"ancestor_record_offset":2080,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":2698,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":2080,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2698,"record_offset":3500,"parent_record_offset":3444,"ancestor_record_offset":2080,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2698,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":2080,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3684,"parent_record_offset":3648,"ancestor_record_offset":2080,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3648,"parent_record_offset":3600,"ancestor_record_offset":2080,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":2080,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2698,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":2080,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2698,"record_offset":3532,"parent_record_offset":3500,"ancestor_record_offset":2080,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2698,"record_offset":2764,"parent_record_offset":2740,"ancestor_record_offset":2080,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":2698,"record_offset":2740,"parent_record_offset":2680,"ancestor_record_offset":2080,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1249,"record_offset":10472,"rva":13427392,"va":5382136512}
// 编译实例 RVAs: 0xcce2c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=6
//     target_aliases=1889 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13423402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8d62c33ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382144069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12536078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f2b640ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5382149593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=16
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12331149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e873d743ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382144696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9514611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e88dd16eff","size":5,"soft_fail":false,"target_va":5372625024,"va":5382139630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9391663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8d1b170ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382141354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=55
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9389711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e871b970ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382139418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da7b0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9388488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e838be70ff","size":5,"soft_fail":false,"target_va":5372749744,"va":5382138227,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7442503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b96f8eff","size":5,"soft_fail":false,"target_va":5374716832,"va":5382159330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140628e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6969021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e843a995ff","size":5,"soft_fail":false,"target_va":5375168144,"va":5382137160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=2
//     target_aliases=28 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6983278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8927195ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5382154329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406bb250
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6375584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e860b79eff","size":5,"soft_fail":false,"target_va":5375767120,"va":5382142699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5202737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8cf9cb0ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5382142636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5200269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e873a6b0ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382144232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da780
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5210969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8a77cb0ff","size":5,"soft_fail":false,"target_va":5376944000,"va":5382154964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-274442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f6cffbff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382144677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-258176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8800ffcff","size":5,"soft_fail":false,"target_va":5381889712,"va":5382147883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-253613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85321fcff","size":5,"soft_fail":false,"target_va":5381913664,"va":5382167272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-221647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8319efcff","size":5,"soft_fail":false,"target_va":5381915200,"va":5382136842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-225086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c290fcff","size":5,"soft_fail":false,"target_va":5381919744,"va":5382144825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c996d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-216639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c1b1fcff","size":5,"soft_fail":false,"target_va":5381920464,"va":5382137098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca1ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-206592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e800d9fcff","size":5,"soft_fail":false,"target_va":5381955280,"va":5382161867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca9730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85cadfdff","size":5,"soft_fail":false,"target_va":5381986096,"va":5382138319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caa800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-147790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b2befdff","size":5,"soft_fail":false,"target_va":5381990400,"va":5382138185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cab650
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-144036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85ccdfdff","size":5,"soft_fail":false,"target_va":5381994064,"va":5382138095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cac0c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-142872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8e8d1fdff","size":5,"soft_fail":false,"target_va":5381996736,"va":5382139603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cac3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-152311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e809adfdff","size":5,"soft_fail":false,"target_va":5381997520,"va":5382149826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cacf70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-137869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e873e5fdff","size":5,"soft_fail":false,"target_va":5382000496,"va":5382138360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cad680
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-134879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e821f1fdff","size":5,"soft_fail":false,"target_va":5382002304,"va":5382137178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cadae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-138593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e89fe2fdff","size":5,"soft_fail":false,"target_va":5382003424,"va":5382142012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cae4b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-151366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8bab0fdff","size":5,"soft_fail":false,"target_va":5382005936,"va":5382157297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb0200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-123965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c31bfeff","size":5,"soft_fail":false,"target_va":5382013440,"va":5382137400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb2290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-129581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8d305feff","size":5,"soft_fail":false,"target_va":5382021776,"va":5382151352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb2ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-125544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e89815feff","size":5,"soft_fail":false,"target_va":5382024928,"va":5382150467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cb4bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8e061feff","size":5,"soft_fail":false,"target_va":5382032368,"va":5382138379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbb890
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-90571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8359efeff","size":5,"soft_fail":false,"target_va":5382060176,"va":5382150742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b138ffff","size":5,"soft_fail":false,"target_va":5382107968,"va":5382158986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140efcda0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2271598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e86ea92200","size":5,"soft_fail":false,"target_va":5384424864,"va":5382153261,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417751d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11157429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b53faa00","size":5,"soft_fail":false,"target_va":5393306064,"va":5382148630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417753a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11161024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c04daa00","size":5,"soft_fail":false,"target_va":5393306528,"va":5382145499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177fc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11203664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e850f4aa00","size":5,"soft_fail":false,"target_va":5393349664,"va":5382145995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141780130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11191312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e810c4aa00","size":5,"soft_fail":false,"target_va":5393350960,"va":5382159643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21456433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e831664701","size":5,"soft_fail":false,"target_va":5403593344,"va":5382136906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21451253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f5514701","size":5,"soft_fail":false,"target_va":5403593360,"va":5382142102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21455427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e843624701","size":5,"soft_fail":false,"target_va":5403593376,"va":5382137944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21455485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e87d624701","size":5,"soft_fail":false,"target_va":5403593968,"va":5382138478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21443226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e89a324701","size":5,"soft_fail":false,"target_va":5403594096,"va":5382150865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21452635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85b574701","size":5,"soft_fail":false,"target_va":5403594144,"va":5382141504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t20815286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b69d3d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382162853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20780610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e842163d01","size":5,"soft_fail":false,"target_va":5402918528,"va":5382137913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t-10828904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e898c35aff","size":5,"soft_fail":false,"target_va":5371308432,"va":5382137331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4636080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85042b9ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382163195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t20816042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8aaa03d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382162833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20891201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e841c63e01","size":5,"soft_fail":false,"target_va":5403054624,"va":5382163418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=15
//     evidence={"assembly":"\tcallq\t20816626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f2a23d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382162521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t20831072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e860db3d01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382148267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t20816159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e81fa13d01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382162780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20816624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f0a23d01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382163499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7659276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f4208bff","size":5,"soft_fail":false,"target_va":5374493328,"va":5382152599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7659688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8581f8bff","size":5,"soft_fail":false,"target_va":5374492896,"va":5382152579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::gable_sides
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6373202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8aec09eff","size":5,"soft_fail":false,"target_va":5375763616,"va":5382136813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-6379084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8b4a99eff","size":5,"soft_fail":false,"target_va":5375763504,"va":5382142583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6373274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e866c09eff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382136821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::terrain_height::TerrainHeights::sample_at_world_xz
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-288413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e86399fbff","size":5,"soft_fail":false,"target_va":5381870304,"va":5382158712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7588676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8bc348cff","size":5,"soft_fail":false,"target_va":5374570992,"va":5382159663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6946918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e89aff95ff","size":5,"soft_fail":false,"target_va":5375212688,"va":5382159601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-204398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e892e1fcff","size":5,"soft_fail":false,"target_va":5381932864,"va":5382137257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-204245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e82be2fcff","size":5,"soft_fail":false,"target_va":5381932960,"va":5382137200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-203935","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e861e3fcff","size":5,"soft_fail":false,"target_va":5381932832,"va":5382136762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::split_height_into_skirts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7315875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85d5e90ff","size":5,"soft_fail":false,"target_va":5374821408,"va":5382137278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-6417745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8af129eff","size":5,"soft_fail":false,"target_va":5375724992,"va":5382142732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::get_normal_at_u
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6419495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8d90b9eff","size":5,"soft_fail":false,"target_va":5375724704,"va":5382144194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3704264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c8853800","size":5,"soft_fail":false,"target_va":5385850032,"va":5382145763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t20831972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8e4de3d01","size":5,"soft_fail":false,"target_va":5402968992,"va":5382137015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t20808546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e862833d01","size":5,"soft_fail":false,"target_va":5402970800,"va":5382162249,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4773389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f329b7ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5382139608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t31108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e884790000","size":5,"soft_fail":false,"target_va":5382172912,"va":5382141799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=21
//     evidence={"assembly":"\tcallq\t2279716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e824c92200","size":5,"soft_fail":false,"target_va":5384420192,"va":5382140471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2279822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e88ec92200","size":5,"soft_fail":false,"target_va":5384420320,"va":5382140493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2279239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e847c72200","size":5,"soft_fail":false,"target_va":5384420000,"va":5382140756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2280788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e854cd2200","size":5,"soft_fail":false,"target_va":5384420752,"va":5382139959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2274065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e811b32200","size":5,"soft_fail":false,"target_va":5384420368,"va":5382146298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t2280125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8bdca2200","size":5,"soft_fail":false,"target_va":5384420736,"va":5382140606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_pos_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4178776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8a83cc0ff","size":5,"soft_fail":false,"target_va":5377967920,"va":5382146691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::perimeter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4181842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8ae30c0ff","size":5,"soft_fail":false,"target_va":5377968464,"va":5382150301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_corner_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4200174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e812e9bfff","size":5,"soft_fail":false,"target_va":5377958496,"va":5382158665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4201392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e850e4bfff","size":5,"soft_fail":false,"target_va":5377958704,"va":5382160091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_edge_id_at_u
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4187926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8ea18c0ff","size":5,"soft_fail":false,"target_va":5377961152,"va":5382149073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4203015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8f9ddbfff","size":5,"soft_fail":false,"target_va":5377955632,"va":5382158642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-4193563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8e502c0ff","size":5,"soft_fail":false,"target_va":5377955840,"va":5382149398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::perimeter
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4200349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e863e8bfff","size":5,"soft_fail":false,"target_va":5377955424,"va":5382155768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5211188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8cc7bb0ff","size":5,"soft_fail":false,"target_va":5376939952,"va":5382151135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rng::init_rng
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t109922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e862ad0100","size":5,"soft_fail":false,"target_va":5382255664,"va":5382145737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5205742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e81291b0ff","size":5,"soft_fail":false,"target_va":5376941312,"va":5382147049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t11197530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85adcaa00","size":5,"soft_fail":false,"target_va":5393349744,"va":5382152209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t11196097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e8c1d6aa00","size":5,"soft_fail":false,"target_va":5393349872,"va":5382153770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t112476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e85cb70100","size":5,"soft_fail":false,"target_va":5382256592,"va":5382144111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t-5201354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382136512,"raw_hex":"e836a2b0ff","size":5,"soft_fail":false,"target_va":5376942944,"va":5382144293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382136512,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5382136926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":19248,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1249,"record_offset":12100,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6488,"source_line":263,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::dedup
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":39752,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":12918,"source_line":2673,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12840,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8652,"source_line":1535,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":11920,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":11976,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12156,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6475,"source_line":1653,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":13516,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":21028,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":16032,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":14288,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":19508,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":25068,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":38420,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":40988,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1249,"record_offset":18860,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1249,"record_offset":57440,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14752,"source_line":57,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1249,"record_offset":57208,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1249,"record_offset":26772,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":36020,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":18772,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":18744,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1249,"record_offset":14088,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::flatten::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1249,"record_offset":33592,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5070,"source_line":65,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1249,"record_offset":14384,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":11824,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":13376,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$108::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1249,"record_offset":46032,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":17484,"source_line":57,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::arith::impl$162::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":1249,"record_offset":57144,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":27674,"source_line":35,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":20332,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":25176,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":28120,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5483,"source_line":378,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::impl$55::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":45696,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":17482,"source_line":343,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":24464,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":11888,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":52228,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":11232,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":16664,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":33444,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":21740,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":13172,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":18596,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":35556,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":16248,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":7667,"source_line":325,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":42924,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":9336,"source_line":976,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::sort_unstable_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":39660,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":17357,"source_line":2967,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":18472,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":18680,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1249,"record_offset":10988,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":11268,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1249,"record_offset":11796,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59895,"source_line":433,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1249,"record_offset":29880,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59541,"source_line":846,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":29088,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":49256,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":46708,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":46836,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":46740,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":30564,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":28940,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":29644,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":25972,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":48712,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":29552,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$18::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":49160,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":25913,"source_line":1146,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":25876,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1249,"record_offset":13256,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":40444,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1249,"record_offset":18276,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":36136,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":31812,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":32264,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":17276,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":9059,"source_line":232,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":55600,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":56184,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":32848,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":17996,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":14060,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::floor
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":40596,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":15383,"source_line":51,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$0
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":26800,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59900,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::half_timbering::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":14468,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59896,"source_line":330,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::wrap_around
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":19692,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59899,"source_line":1704,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1249,"record_offset":26160,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":54340,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":59684,"source_line":954,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":11768,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::to_curve2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":13808,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21343,"source_line":938,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14180,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":40836,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21781,"source_line":134,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":25660,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":40540,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1249,"record_offset":25532,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":54664,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1249,"record_offset":56916,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1249,"record_offset":56884,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":27600,"parent_record_offset":10472,"ancestor_record_offset":10472,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":10472,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":19320,"parent_record_offset":19296,"ancestor_record_offset":10472,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":10472,"inlinee":6014,"source_line":171,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":16852,"parent_record_offset":16828,"ancestor_record_offset":10472,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":19272,"parent_record_offset":19248,"ancestor_record_offset":10472,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":13648,"parent_record_offset":13612,"ancestor_record_offset":10472,"inlinee":6013,"source_line":247,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":16828,"parent_record_offset":16736,"ancestor_record_offset":10472,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1249,"record_offset":63224,"parent_record_offset":63200,"ancestor_record_offset":10472,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":38632,"parent_record_offset":38604,"ancestor_record_offset":10472,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":16764,"parent_record_offset":16736,"ancestor_record_offset":10472,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":38604,"parent_record_offset":38572,"ancestor_record_offset":10472,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":12036,"parent_record_offset":12012,"ancestor_record_offset":10472,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":10472,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":13612,"parent_record_offset":13580,"ancestor_record_offset":10472,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":21232,"parent_record_offset":21192,"ancestor_record_offset":10472,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":13580,"parent_record_offset":13548,"ancestor_record_offset":10472,"inlinee":6012,"source_line":169,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":13736,"parent_record_offset":13580,"ancestor_record_offset":10472,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=53
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":16736,"parent_record_offset":16712,"ancestor_record_offset":10472,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1249,"record_offset":21144,"parent_record_offset":21104,"ancestor_record_offset":10472,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1249,"record_offset":21104,"parent_record_offset":21064,"ancestor_record_offset":10472,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1249,"record_offset":21064,"parent_record_offset":21028,"ancestor_record_offset":10472,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1249,"record_offset":12124,"parent_record_offset":12100,"ancestor_record_offset":10472,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":41072,"parent_record_offset":41040,"ancestor_record_offset":10472,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12012,"parent_record_offset":11976,"ancestor_record_offset":10472,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::dedup_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":39780,"parent_record_offset":39752,"ancestor_record_offset":10472,"inlinee":8654,"source_line":1815,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":38500,"parent_record_offset":38460,"ancestor_record_offset":10472,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12220,"parent_record_offset":12180,"ancestor_record_offset":10472,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12880,"parent_record_offset":12840,"ancestor_record_offset":10472,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":10472,"inlinee":6476,"source_line":1679,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":10472,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":21192,"parent_record_offset":21144,"ancestor_record_offset":10472,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":11852,"parent_record_offset":11824,"ancestor_record_offset":10472,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":35076,"parent_record_offset":33628,"ancestor_record_offset":10472,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":41012,"parent_record_offset":40988,"ancestor_record_offset":10472,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":39224,"parent_record_offset":39200,"ancestor_record_offset":10472,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12280,"parent_record_offset":12248,"ancestor_record_offset":10472,"inlinee":6480,"source_line":1657,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":10472,"inlinee":6487,"source_line":1707,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::process_loop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":12248,"parent_record_offset":12180,"ancestor_record_offset":10472,"inlinee":6478,"source_line":1727,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":61056,"parent_record_offset":61032,"ancestor_record_offset":10472,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$5::dedup::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":39836,"parent_record_offset":39780,"ancestor_record_offset":10472,"inlinee":8528,"source_line":2674,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1249,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":10472,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1249,"record_offset":19032,"parent_record_offset":19008,"ancestor_record_offset":10472,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1249,"record_offset":33776,"parent_record_offset":33748,"ancestor_record_offset":10472,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1249,"record_offset":39280,"parent_record_offset":39224,"ancestor_record_offset":10472,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1249,"record_offset":38740,"parent_record_offset":38500,"ancestor_record_offset":10472,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1249,"record_offset":39448,"parent_record_offset":39424,"ancestor_record_offset":10472,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1249,"record_offset":38460,"parent_record_offset":38420,"ancestor_record_offset":10472,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1249,"record_offset":41040,"parent_record_offset":40988,"ancestor_record_offset":10472,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1249,"record_offset":13548,"parent_record_offset":13516,"ancestor_record_offset":10472,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1249,"record_offset":21316,"parent_record_offset":21272,"ancestor_record_offset":10472,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1249,"record_offset":21340,"parent_record_offset":21316,"ancestor_record_offset":10472,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1249,"record_offset":57304,"parent_record_offset":57236,"ancestor_record_offset":10472,"inlinee":11996,"source_line":245,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1249,"record_offset":17600,"parent_record_offset":17572,"ancestor_record_offset":10472,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1249,"record_offset":17652,"parent_record_offset":17628,"ancestor_record_offset":10472,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1249,"record_offset":17628,"parent_record_offset":17572,"ancestor_record_offset":10472,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$17::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1249,"record_offset":45764,"parent_record_offset":45696,"ancestor_record_offset":10472,"inlinee":6622,"source_line":219,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":1249,"record_offset":45732,"parent_record_offset":45696,"ancestor_record_offset":10472,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":20380,"parent_record_offset":20356,"ancestor_record_offset":10472,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":25360,"parent_record_offset":25204,"ancestor_record_offset":10472,"inlinee":5479,"source_line":1672,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$13::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":39860,"parent_record_offset":39836,"ancestor_record_offset":10472,"inlinee":8530,"source_line":1709,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$21::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":36880,"parent_record_offset":36856,"ancestor_record_offset":10472,"inlinee":19598,"source_line":1469,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":20404,"parent_record_offset":20380,"ancestor_record_offset":10472,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":25384,"parent_record_offset":25360,"ancestor_record_offset":10472,"inlinee":5481,"source_line":1515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":14432,"parent_record_offset":14408,"ancestor_record_offset":10472,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":11060,"parent_record_offset":11020,"ancestor_record_offset":10472,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1249,"record_offset":26652,"parent_record_offset":26188,"ancestor_record_offset":10472,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":32452,"parent_record_offset":32424,"ancestor_record_offset":10472,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":36400,"parent_record_offset":36376,"ancestor_record_offset":10472,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":32512,"parent_record_offset":32480,"ancestor_record_offset":10472,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":30176,"parent_record_offset":29908,"ancestor_record_offset":10472,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":26452,"parent_record_offset":26400,"ancestor_record_offset":10472,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":26424,"parent_record_offset":26400,"ancestor_record_offset":10472,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":29972,"parent_record_offset":29908,"ancestor_record_offset":10472,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":22724,"parent_record_offset":22636,"ancestor_record_offset":10472,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":19768,"parent_record_offset":19728,"ancestor_record_offset":10472,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":22756,"parent_record_offset":22636,"ancestor_record_offset":10472,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1249,"record_offset":25840,"parent_record_offset":25660,"ancestor_record_offset":10472,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1249,"record_offset":23748,"parent_record_offset":23640,"ancestor_record_offset":10472,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1249,"record_offset":12556,"parent_record_offset":12348,"ancestor_record_offset":10472,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::Chain::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1249,"record_offset":40404,"parent_record_offset":40380,"ancestor_record_offset":10472,"inlinee":7701,"source_line":36,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1249,"record_offset":34272,"parent_record_offset":34248,"ancestor_record_offset":10472,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::and_then_or_clear
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1249,"record_offset":33684,"parent_record_offset":33628,"ancestor_record_offset":10472,"inlinee":5074,"source_line":839,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::flatten::impl$35::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\flatten.rs
//     evidence={"module":1249,"record_offset":33628,"parent_record_offset":33592,"ancestor_record_offset":10472,"inlinee":5072,"source_line":605,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$11::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1249,"record_offset":34196,"parent_record_offset":34172,"ancestor_record_offset":10472,"inlinee":5090,"source_line":358,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::fuse::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\fuse.rs
//     evidence={"module":1249,"record_offset":34172,"parent_record_offset":33628,"ancestor_record_offset":10472,"inlinee":5088,"source_line":49,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":10472,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":38804,"parent_record_offset":38768,"ancestor_record_offset":10472,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":10472,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":38928,"parent_record_offset":38840,"ancestor_record_offset":10472,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1249,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":10472,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":10472,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":10472,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1249,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":10472,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1249,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":10472,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":40380,"parent_record_offset":40220,"ancestor_record_offset":10472,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":13924,"parent_record_offset":13808,"ancestor_record_offset":10472,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":38768,"parent_record_offset":38500,"ancestor_record_offset":10472,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":39200,"parent_record_offset":38928,"ancestor_record_offset":10472,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1249,"record_offset":12596,"parent_record_offset":12180,"ancestor_record_offset":10472,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1249,"record_offset":57332,"parent_record_offset":57304,"ancestor_record_offset":10472,"inlinee":6097,"source_line":697,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1249,"record_offset":17676,"parent_record_offset":17652,"ancestor_record_offset":10472,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1249,"record_offset":43072,"parent_record_offset":43016,"ancestor_record_offset":10472,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1249,"record_offset":43104,"parent_record_offset":43072,"ancestor_record_offset":10472,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":11664,"parent_record_offset":11636,"ancestor_record_offset":10472,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::leading_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":39712,"parent_record_offset":39684,"ancestor_record_offset":10472,"inlinee":17359,"source_line":148,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":11688,"parent_record_offset":11664,"ancestor_record_offset":10472,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":39320,"parent_record_offset":38840,"ancestor_record_offset":10472,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=47
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":16792,"parent_record_offset":16764,"ancestor_record_offset":10472,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1249,"record_offset":33060,"parent_record_offset":33036,"ancestor_record_offset":10472,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15856,"parent_record_offset":15800,"ancestor_record_offset":10472,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15884,"parent_record_offset":15800,"ancestor_record_offset":10472,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":48748,"parent_record_offset":48712,"ancestor_record_offset":10472,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":49188,"parent_record_offset":49160,"ancestor_record_offset":10472,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15256,"parent_record_offset":14556,"ancestor_record_offset":10472,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":33748,"parent_record_offset":33684,"ancestor_record_offset":10472,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":10472,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1249,"record_offset":57272,"parent_record_offset":57236,"ancestor_record_offset":10472,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1249,"record_offset":57236,"parent_record_offset":57208,"ancestor_record_offset":10472,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":10472,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":22896,"parent_record_offset":22636,"ancestor_record_offset":10472,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":33720,"parent_record_offset":33684,"ancestor_record_offset":10472,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":17528,"parent_record_offset":17504,"ancestor_record_offset":10472,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":11564,"parent_record_offset":11268,"ancestor_record_offset":10472,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":22696,"parent_record_offset":22636,"ancestor_record_offset":10472,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":15108,"parent_record_offset":14908,"ancestor_record_offset":10472,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":11348,"parent_record_offset":11268,"ancestor_record_offset":10472,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":11604,"parent_record_offset":11268,"ancestor_record_offset":10472,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":23684,"parent_record_offset":23640,"ancestor_record_offset":10472,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":21536,"parent_record_offset":21144,"ancestor_record_offset":10472,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":57360,"parent_record_offset":57332,"ancestor_record_offset":10472,"inlinee":6098,"source_line":1250,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":41416,"parent_record_offset":41392,"ancestor_record_offset":10472,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=102
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":10472,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":12992,"parent_record_offset":12840,"ancestor_record_offset":10472,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":33280,"parent_record_offset":33256,"ancestor_record_offset":10472,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":33256,"parent_record_offset":33232,"ancestor_record_offset":10472,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":18560,"parent_record_offset":18536,"ancestor_record_offset":10472,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":20148,"parent_record_offset":20092,"ancestor_record_offset":10472,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":41392,"parent_record_offset":41368,"ancestor_record_offset":10472,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":13336,"parent_record_offset":13304,"ancestor_record_offset":10472,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":20172,"parent_record_offset":20148,"ancestor_record_offset":10472,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":19344,"parent_record_offset":19320,"ancestor_record_offset":10472,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":12068,"parent_record_offset":11976,"ancestor_record_offset":10472,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1249,"record_offset":11472,"parent_record_offset":11384,"ancestor_record_offset":10472,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1249,"record_offset":11496,"parent_record_offset":11472,"ancestor_record_offset":10472,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1249,"record_offset":14012,"parent_record_offset":13980,"ancestor_record_offset":10472,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1249,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":10472,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1249,"record_offset":11172,"parent_record_offset":11144,"ancestor_record_offset":10472,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::as_mut_ptr_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":42960,"parent_record_offset":42924,"ancestor_record_offset":10472,"inlinee":13092,"source_line":873,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":57596,"parent_record_offset":57568,"ancestor_record_offset":10472,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":10472,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":11532,"parent_record_offset":11268,"ancestor_record_offset":10472,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::reverse::revswap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":43016,"parent_record_offset":42924,"ancestor_record_offset":10472,"inlinee":9338,"source_line":999,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":16064,"parent_record_offset":16032,"ancestor_record_offset":10472,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":14316,"parent_record_offset":14288,"ancestor_record_offset":10472,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":21768,"parent_record_offset":21740,"ancestor_record_offset":10472,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":57624,"parent_record_offset":57596,"ancestor_record_offset":10472,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":16096,"parent_record_offset":16064,"ancestor_record_offset":10472,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":14344,"parent_record_offset":14316,"ancestor_record_offset":10472,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1249,"record_offset":41344,"parent_record_offset":41040,"ancestor_record_offset":10472,"inlinee":14888,"source_line":128,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":41368,"parent_record_offset":41344,"ancestor_record_offset":10472,"inlinee":14889,"source_line":90,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1249,"record_offset":13196,"parent_record_offset":13172,"ancestor_record_offset":10472,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":18536,"parent_record_offset":18472,"ancestor_record_offset":10472,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1249,"record_offset":18620,"parent_record_offset":18596,"ancestor_record_offset":10472,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":38840,"parent_record_offset":38804,"ancestor_record_offset":10472,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":10472,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":14828,"parent_record_offset":14800,"ancestor_record_offset":10472,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":21640,"parent_record_offset":21608,"ancestor_record_offset":10472,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::quicksort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\sort.rs
//     evidence={"module":1249,"record_offset":39684,"parent_record_offset":39660,"ancestor_record_offset":10472,"inlinee":17358,"source_line":892,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1249,"record_offset":11088,"parent_record_offset":11060,"ancestor_record_offset":10472,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":63060,"parent_record_offset":63028,"ancestor_record_offset":10472,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":11416,"parent_record_offset":11384,"ancestor_record_offset":10472,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":63088,"parent_record_offset":63060,"ancestor_record_offset":10472,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":11440,"parent_record_offset":11416,"ancestor_record_offset":10472,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1249,"record_offset":11020,"parent_record_offset":10988,"ancestor_record_offset":10472,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":11636,"parent_record_offset":11604,"ancestor_record_offset":10472,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":63000,"parent_record_offset":62976,"ancestor_record_offset":10472,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1249,"record_offset":26188,"parent_record_offset":26160,"ancestor_record_offset":10472,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1249,"record_offset":26232,"parent_record_offset":26188,"ancestor_record_offset":10472,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_quat
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":1249,"record_offset":29908,"parent_record_offset":29880,"ancestor_record_offset":10472,"inlinee":22239,"source_line":675,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":26304,"parent_record_offset":26232,"ancestor_record_offset":10472,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":26400,"parent_record_offset":26188,"ancestor_record_offset":10472,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":26676,"parent_record_offset":26652,"ancestor_record_offset":10472,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":29168,"parent_record_offset":29112,"ancestor_record_offset":10472,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":10472,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":52284,"parent_record_offset":52260,"ancestor_record_offset":10472,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":29744,"parent_record_offset":29708,"ancestor_record_offset":10472,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":29708,"parent_record_offset":29672,"ancestor_record_offset":10472,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":29672,"parent_record_offset":29644,"ancestor_record_offset":10472,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":55204,"parent_record_offset":54884,"ancestor_record_offset":10472,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":28964,"parent_record_offset":28940,"ancestor_record_offset":10472,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":48644,"parent_record_offset":48616,"ancestor_record_offset":10472,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1249,"record_offset":36832,"parent_record_offset":36808,"ancestor_record_offset":10472,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":36160,"parent_record_offset":36136,"ancestor_record_offset":10472,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":36184,"parent_record_offset":36160,"ancestor_record_offset":10472,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":55656,"parent_record_offset":55628,"ancestor_record_offset":10472,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":32288,"parent_record_offset":32264,"ancestor_record_offset":10472,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":36808,"parent_record_offset":36668,"ancestor_record_offset":10472,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":56212,"parent_record_offset":56184,"ancestor_record_offset":10472,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":32872,"parent_record_offset":32848,"ancestor_record_offset":10472,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":36216,"parent_record_offset":36184,"ancestor_record_offset":10472,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36720,"parent_record_offset":36696,"ancestor_record_offset":10472,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36696,"parent_record_offset":36668,"ancestor_record_offset":10472,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":37076,"parent_record_offset":37052,"ancestor_record_offset":10472,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":33232,"parent_record_offset":32924,"ancestor_record_offset":10472,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":37016,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32396,"parent_record_offset":32336,"ancestor_record_offset":10472,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32924,"parent_record_offset":32896,"ancestor_record_offset":10472,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":37052,"parent_record_offset":36268,"ancestor_record_offset":10472,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36268,"parent_record_offset":36244,"ancestor_record_offset":10472,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36244,"parent_record_offset":36184,"ancestor_record_offset":10472,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32312,"parent_record_offset":32288,"ancestor_record_offset":10472,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32368,"parent_record_offset":32336,"ancestor_record_offset":10472,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":10472,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32336,"parent_record_offset":32312,"ancestor_record_offset":10472,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":33004,"parent_record_offset":32972,"ancestor_record_offset":10472,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":33036,"parent_record_offset":33004,"ancestor_record_offset":10472,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":36636,"parent_record_offset":36516,"ancestor_record_offset":10472,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":32972,"parent_record_offset":32924,"ancestor_record_offset":10472,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36348,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":60988,"parent_record_offset":60964,"ancestor_record_offset":10472,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":32896,"parent_record_offset":32872,"ancestor_record_offset":10472,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":36668,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":36376,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":32424,"parent_record_offset":32396,"ancestor_record_offset":10472,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":36460,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":36924,"parent_record_offset":36296,"ancestor_record_offset":10472,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":32480,"parent_record_offset":32424,"ancestor_record_offset":10472,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1249,"record_offset":14656,"parent_record_offset":14556,"ancestor_record_offset":10472,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14884,"parent_record_offset":14556,"ancestor_record_offset":10472,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":10472,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1249,"record_offset":15800,"parent_record_offset":14468,"ancestor_record_offset":10472,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1249,"record_offset":11384,"parent_record_offset":11268,"ancestor_record_offset":10472,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1249,"record_offset":63028,"parent_record_offset":63000,"ancestor_record_offset":10472,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":55628,"parent_record_offset":55600,"ancestor_record_offset":10472,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":17304,"parent_record_offset":17276,"ancestor_record_offset":10472,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":10472,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":29200,"parent_record_offset":29168,"ancestor_record_offset":10472,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1249,"record_offset":17368,"parent_record_offset":17332,"ancestor_record_offset":10472,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1249,"record_offset":17452,"parent_record_offset":17424,"ancestor_record_offset":10472,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1249,"record_offset":17572,"parent_record_offset":17424,"ancestor_record_offset":10472,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1249,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":10472,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1249,"record_offset":17476,"parent_record_offset":17452,"ancestor_record_offset":10472,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1249,"record_offset":17504,"parent_record_offset":17476,"ancestor_record_offset":10472,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1249,"record_offset":17424,"parent_record_offset":17396,"ancestor_record_offset":10472,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1249,"record_offset":17396,"parent_record_offset":17368,"ancestor_record_offset":10472,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$10
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":22636,"parent_record_offset":22608,"ancestor_record_offset":10472,"inlinee":59582,"source_line":556,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$13
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":38956,"parent_record_offset":38928,"ancestor_record_offset":10472,"inlinee":59901,"source_line":955,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$17
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":52260,"parent_record_offset":52228,"ancestor_record_offset":10472,"inlinee":59902,"source_line":1287,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$3
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":12308,"parent_record_offset":12280,"ancestor_record_offset":10472,"inlinee":59577,"source_line":262,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":10472,"inlinee":59897,"source_line":326,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1249,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":10472,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1249,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":10472,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":13980,"parent_record_offset":13808,"ancestor_record_offset":10472,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":14628,"parent_record_offset":14556,"ancestor_record_offset":10472,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":15772,"parent_record_offset":14468,"ancestor_record_offset":10472,"inlinee":59887,"source_line":1123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":54776,"parent_record_offset":54664,"ancestor_record_offset":10472,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":54628,"parent_record_offset":54340,"ancestor_record_offset":10472,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14524,"parent_record_offset":14468,"ancestor_record_offset":10472,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14556,"parent_record_offset":14524,"ancestor_record_offset":10472,"inlinee":59845,"source_line":240,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":54384,"parent_record_offset":54340,"ancestor_record_offset":10472,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14212,"parent_record_offset":14180,"ancestor_record_offset":10472,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":54524,"parent_record_offset":54492,"ancestor_record_offset":10472,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":54492,"parent_record_offset":54340,"ancestor_record_offset":10472,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":10472,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":38984,"parent_record_offset":38956,"ancestor_record_offset":10472,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":10472,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::impl$12::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":39884,"parent_record_offset":39860,"ancestor_record_offset":10472,"inlinee":24539,"source_line":6,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1249,"record_offset":19728,"parent_record_offset":19692,"ancestor_record_offset":10472,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1249,"record_offset":19796,"parent_record_offset":19728,"ancestor_record_offset":10472,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":15288,"parent_record_offset":14556,"ancestor_record_offset":10472,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::overlapping_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1249,"record_offset":69640,"rva":13462096,"va":5382171216}
// 编译实例 RVAs: 0xcd6a50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9422677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8ab3870ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382172384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7454529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8bf408eff","size":5,"soft_fail":false,"target_va":5374716832,"va":5382171356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-10866345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e857315aff","size":5,"soft_fail":false,"target_va":5371305408,"va":5382171748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6958919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8b9d095ff","size":5,"soft_fail":false,"target_va":5375212688,"va":5382171602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t843","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e84b030000","size":5,"soft_fail":false,"target_va":5382172480,"va":5382171632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t2248391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8c74e2200","size":5,"soft_fail":false,"target_va":5384420192,"va":5382171796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2248092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e89c4d2200","size":5,"soft_fail":false,"target_va":5384420000,"va":5382171903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2248874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8aa502200","size":5,"soft_fail":false,"target_va":5384420752,"va":5382171873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::intersects
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2248653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8cd4f2200","size":5,"soft_fail":false,"target_va":5384420368,"va":5382171710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t2248923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382171216,"raw_hex":"e8db502200","size":5,"soft_fail":false,"target_va":5384420736,"va":5382171808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":69804,"parent_record_offset":69640,"ancestor_record_offset":69640,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":70192,"parent_record_offset":69640,"ancestor_record_offset":69640,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":71148,"parent_record_offset":69640,"ancestor_record_offset":69640,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":69832,"parent_record_offset":69640,"ancestor_record_offset":69640,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":70320,"parent_record_offset":69640,"ancestor_record_offset":69640,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":71324,"parent_record_offset":71300,"ancestor_record_offset":69640,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":71300,"parent_record_offset":71200,"ancestor_record_offset":69640,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":71232,"parent_record_offset":71200,"ancestor_record_offset":69640,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":70256,"parent_record_offset":70224,"ancestor_record_offset":69640,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":71200,"parent_record_offset":71172,"ancestor_record_offset":69640,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":70224,"parent_record_offset":70192,"ancestor_record_offset":69640,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":70044,"parent_record_offset":70016,"ancestor_record_offset":69640,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":70104,"parent_record_offset":70072,"ancestor_record_offset":69640,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":71264,"parent_record_offset":71232,"ancestor_record_offset":69640,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1249,"record_offset":70540,"parent_record_offset":70516,"ancestor_record_offset":69640,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":70140,"parent_record_offset":69988,"ancestor_record_offset":69640,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":71172,"parent_record_offset":71148,"ancestor_record_offset":69640,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":70788,"parent_record_offset":70764,"ancestor_record_offset":69640,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":70764,"parent_record_offset":70740,"ancestor_record_offset":69640,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":70288,"parent_record_offset":70192,"ancestor_record_offset":69640,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":69880,"parent_record_offset":69856,"ancestor_record_offset":69640,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":70344,"parent_record_offset":70320,"ancestor_record_offset":69640,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":70740,"parent_record_offset":70400,"ancestor_record_offset":69640,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":69988,"parent_record_offset":69928,"ancestor_record_offset":69640,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":70400,"parent_record_offset":70368,"ancestor_record_offset":69640,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":69904,"parent_record_offset":69880,"ancestor_record_offset":69640,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":69960,"parent_record_offset":69928,"ancestor_record_offset":69640,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":69928,"parent_record_offset":69904,"ancestor_record_offset":69640,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":70484,"parent_record_offset":70448,"ancestor_record_offset":69640,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":70516,"parent_record_offset":70484,"ancestor_record_offset":69640,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":70576,"parent_record_offset":70448,"ancestor_record_offset":69640,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":70448,"parent_record_offset":70400,"ancestor_record_offset":69640,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":70368,"parent_record_offset":70344,"ancestor_record_offset":69640,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":70016,"parent_record_offset":69988,"ancestor_record_offset":69640,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":70072,"parent_record_offset":70016,"ancestor_record_offset":69640,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":69856,"parent_record_offset":69832,"ancestor_record_offset":69640,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1249,"record_offset":260,"rva":13422080,"va":5382131200}
// 编译实例 RVAs: 0xccce00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14010da30
//     owner=unresolved_namespace; count=4
//     target_aliases=12 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12320652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8740044ff","size":5,"soft_fail":false,"target_va":5369813552,"va":5382134199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9382705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8cfd470ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382132412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6254738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e86e8fa0ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5382136173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5193239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8e9c1b0ff","size":5,"soft_fail":false,"target_va":5376939904,"va":5382133138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5188183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8a9d5b0ff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382132146,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8d2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-263956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8ecf8fbff","size":5,"soft_fail":false,"target_va":5381870240,"va":5382134191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c98240
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-216171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e895b3fcff","size":5,"soft_fail":false,"target_va":5381915200,"va":5382131366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c99400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-214557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8e3b9fcff","size":5,"soft_fail":false,"target_va":5381919744,"va":5382134296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140caaa10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-141669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e89bd6fdff","size":5,"soft_fail":false,"target_va":5381990928,"va":5382132592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21461903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e88f7b4701","size":5,"soft_fail":false,"target_va":5403593344,"va":5382131436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21461829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8457b4701","size":5,"soft_fail":false,"target_va":5403594288,"va":5382132454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t20843256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8f80a3e01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382135619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t20918869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e855323f01","size":5,"soft_fail":false,"target_va":5403054560,"va":5382135686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20843680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8a00c3e01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382135467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t20844154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e87a0e3e01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382135185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20843501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8ed0b3e01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382135438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7541596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8a4ec8cff","size":5,"soft_fail":false,"target_va":5374590144,"va":5382131735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6368517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8fbd29eff","size":5,"soft_fail":false,"target_va":5375763504,"va":5382132016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6368006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8fad49eff","size":5,"soft_fail":false,"target_va":5375763696,"va":5382131697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::profile_curve_ws
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6366215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8f9db9eff","size":5,"soft_fail":false,"target_va":5375766160,"va":5382132370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6401121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e89f539eff","size":5,"soft_fail":false,"target_va":5375730528,"va":5382131644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6401486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e832529eff","size":5,"soft_fail":false,"target_va":5375730144,"va":5382131625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-199032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e888f6fcff","size":5,"soft_fail":false,"target_va":5381932784,"va":5382131811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-198503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e899f8fcff","size":5,"soft_fail":false,"target_va":5381932832,"va":5382131330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6406832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8503d9eff","size":5,"soft_fail":false,"target_va":5375724992,"va":5382131819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t20837456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e850f43d01","size":5,"soft_fail":false,"target_va":5402968992,"va":5382131531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t20835463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e887ec3d01","size":5,"soft_fail":false,"target_va":5402970800,"va":5382135332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::as_points2
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4178548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e88c3dc0ff","size":5,"soft_fail":false,"target_va":5377953488,"va":5382132031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4178740","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8cc3cc0ff","size":5,"soft_fail":false,"target_va":5377953024,"va":5382131759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4178826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8763cc0ff","size":5,"soft_fail":false,"target_va":5377952896,"va":5382131717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4176396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e8f445c0ff","size":5,"soft_fail":false,"target_va":5377955440,"va":5382131831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::rotation_from_normal
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5192561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e88fc4b0ff","size":5,"soft_fail":false,"target_va":5376941312,"va":5382133868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::xz_rotation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5191546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382131200,"raw_hex":"e886c8b0ff","size":5,"soft_fail":false,"target_va":5376942944,"va":5382134485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5382131200,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5382131456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1249,"record_offset":7960,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":2880,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":2992,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1249,"record_offset":1284,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1249,"record_offset":2908,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":760,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":2384,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1249,"record_offset":520,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::sparse_instance_buffer::SparseInstanceBuffer::write_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":796,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14713,"source_line":158,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat4::Mat4::transform_point3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat4.rs
//     evidence={"module":1249,"record_offset":5908,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23594,"source_line":1098,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":2172,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":8100,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":8016,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":1772,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":8288,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":1828,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":6792,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::extend
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":1860,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14702,"source_line":154,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":2264,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":6952,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":6264,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":8044,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":7256,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":13205,"source_line":994,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":8376,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":2852,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":7988,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59894,"source_line":67,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::utils::half_timber::roof_outline::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":1368,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":59892,"source_line":91,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":1249,"record_offset":7348,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":3880,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":22382,"source_line":562,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":2356,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":2584,"parent_record_offset":2560,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":2560,"parent_record_offset":2460,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":2492,"parent_record_offset":2460,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":2460,"parent_record_offset":2432,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":3252,"parent_record_offset":3216,"ancestor_record_offset":260,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":260,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":260,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":1332,"parent_record_offset":1308,"ancestor_record_offset":260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":588,"parent_record_offset":548,"ancestor_record_offset":260,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1249,"record_offset":7868,"parent_record_offset":7376,"ancestor_record_offset":260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1480,"parent_record_offset":1396,"ancestor_record_offset":260,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":260,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":5972,"parent_record_offset":5944,"ancestor_record_offset":260,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1452,"parent_record_offset":1396,"ancestor_record_offset":260,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":4368,"parent_record_offset":3924,"ancestor_record_offset":260,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":3500,"parent_record_offset":3348,"ancestor_record_offset":260,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1249,"record_offset":3528,"parent_record_offset":3348,"ancestor_record_offset":260,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1249,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1249,"record_offset":3032,"parent_record_offset":2992,"ancestor_record_offset":260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1249,"record_offset":2956,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":260,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":1204,"parent_record_offset":1180,"ancestor_record_offset":260,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":2524,"parent_record_offset":2492,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":5720,"parent_record_offset":5624,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":7288,"parent_record_offset":7256,"ancestor_record_offset":260,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":3176,"parent_record_offset":3136,"ancestor_record_offset":260,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":260,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":3388,"parent_record_offset":3348,"ancestor_record_offset":260,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":1084,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":3136,"parent_record_offset":2992,"ancestor_record_offset":260,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":868,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":1124,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1249,"record_offset":992,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1249,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":260,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1249,"record_offset":672,"parent_record_offset":644,"ancestor_record_offset":260,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1249,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::binary_search_by
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":260,"inlinee":18231,"source_line":2788,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":1052,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":3996,"parent_record_offset":3924,"ancestor_record_offset":260,"inlinee":21349,"source_line":4188,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::partition_point::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":4060,"parent_record_offset":4020,"ancestor_record_offset":260,"inlinee":21351,"source_line":4192,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":260,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1249,"record_offset":616,"parent_record_offset":588,"ancestor_record_offset":260,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":9980,"parent_record_offset":9948,"ancestor_record_offset":260,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":936,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":260,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1249,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":260,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":1249,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":260,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$2::write_allocation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":260,"inlinee":14715,"source_line":175,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::sparse_instance_buffer::impl$5::drop
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/sparse_instance_buffer.rs
//     evidence={"module":1249,"record_offset":9920,"parent_record_offset":9892,"ancestor_record_offset":260,"inlinee":14717,"source_line":435,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":1249,"record_offset":7376,"parent_record_offset":7348,"ancestor_record_offset":260,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1249,"record_offset":7040,"parent_record_offset":7008,"ancestor_record_offset":260,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1249,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":260,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":1249,"record_offset":7432,"parent_record_offset":7376,"ancestor_record_offset":260,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":7520,"parent_record_offset":7432,"ancestor_record_offset":260,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":7616,"parent_record_offset":7376,"ancestor_record_offset":260,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":1249,"record_offset":7892,"parent_record_offset":7868,"ancestor_record_offset":260,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1249,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":260,"inlinee":14703,"source_line":92,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$10::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1249,"record_offset":5944,"parent_record_offset":5908,"ancestor_record_offset":260,"inlinee":21866,"source_line":975,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$13::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":1249,"record_offset":6176,"parent_record_offset":5908,"ancestor_record_offset":260,"inlinee":15701,"source_line":998,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":260,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":260,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":260,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":260,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":1512,"parent_record_offset":1368,"ancestor_record_offset":260,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":260,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":7008,"parent_record_offset":6976,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":6976,"parent_record_offset":6952,"ancestor_record_offset":260,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::swizzles::sse2::vec4_impl::impl$0::xyz
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/swizzles/sse2/vec4_impl.rs
//     evidence={"module":1249,"record_offset":6232,"parent_record_offset":5908,"ancestor_record_offset":260,"inlinee":21868,"source_line":223,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1249,"record_offset":904,"parent_record_offset":796,"ancestor_record_offset":260,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1249,"record_offset":9948,"parent_record_offset":9920,"ancestor_record_offset":260,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::roof_outline::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":3216,"parent_record_offset":3176,"ancestor_record_offset":260,"inlinee":59893,"source_line":119,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_coord_at_u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":3924,"parent_record_offset":3880,"ancestor_record_offset":260,"inlinee":21348,"source_line":525,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":4404,"parent_record_offset":3880,"ancestor_record_offset":260,"inlinee":21575,"source_line":622,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":4444,"parent_record_offset":4404,"ancestor_record_offset":260,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":4600,"parent_record_offset":4404,"ancestor_record_offset":260,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":3348,"parent_record_offset":3216,"ancestor_record_offset":260,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":4216,"parent_record_offset":3924,"ancestor_record_offset":260,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$4::get_coord_at_u::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":260,"inlinee":21353,"source_line":544,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":5576,"parent_record_offset":5260,"ancestor_record_offset":260,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9F6
// 首条位置: {"module":1249,"inlinee":59894}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9F4
// 首条位置: {"module":1249,"inlinee":59892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":1249,"record_offset":1396,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":1512,"parent_record_offset":1368,"ancestor_record_offset":1368,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1480,"parent_record_offset":1396,"ancestor_record_offset":1368,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":1368,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1249,"record_offset":1452,"parent_record_offset":1396,"ancestor_record_offset":1368,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":1368,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8D6
// 首条位置: {"module":1226,"inlinee":59606}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::roof_outline::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9F5
// 首条位置: {"module":1249,"inlinee":59893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1249,"record_offset":3252,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":3348,"parent_record_offset":3216,"ancestor_record_offset":3216,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":3216,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":3216,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":3500,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1249,"record_offset":3528,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":3216,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":3388,"parent_record_offset":3348,"ancestor_record_offset":3216,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":3280,"parent_record_offset":3252,"ancestor_record_offset":3216,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1249,"record_offset":3308,"parent_record_offset":3280,"ancestor_record_offset":3216,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9FC
// 首条位置: {"module":1249,"inlinee":59900}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8EB
// 首条位置: {"module":1226,"inlinee":59627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 259
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8F6
// 首条位置: {"module":1227,"inlinee":59638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":23296,"parent_record_offset":23268,"ancestor_record_offset":23268,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8B9
// 首条位置: {"module":1226,"inlinee":59577}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8D4
// 首条位置: {"module":1226,"inlinee":59604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 283
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15DF3
// 首条位置: {"module":2698,"inlinee":89587}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2698,"record_offset":548,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2698,"record_offset":608,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2698,"record_offset":744,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2698,"record_offset":804,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":2698,"record_offset":576,"parent_record_offset":480,"ancestor_record_offset":480,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":480,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":956,"parent_record_offset":856,"ancestor_record_offset":480,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2698,"record_offset":888,"parent_record_offset":856,"ancestor_record_offset":480,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2698,"record_offset":680,"parent_record_offset":648,"ancestor_record_offset":480,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2698,"record_offset":856,"parent_record_offset":828,"ancestor_record_offset":480,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2698,"record_offset":648,"parent_record_offset":608,"ancestor_record_offset":480,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2698,"record_offset":920,"parent_record_offset":888,"ancestor_record_offset":480,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2698,"record_offset":828,"parent_record_offset":804,"ancestor_record_offset":480,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2698,"record_offset":712,"parent_record_offset":608,"ancestor_record_offset":480,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":2698,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":480,"inlinee":89589,"source_line":290,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15DF5
// 首条位置: {"module":2698,"inlinee":89589}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9F9
// 首条位置: {"module":1249,"inlinee":59897}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":15180,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9F8
// 首条位置: {"module":1249,"inlinee":59896}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1249,"record_offset":15800,"parent_record_offset":14468,"ancestor_record_offset":14468,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::CurveInternalCoord::i0_t
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":15772,"parent_record_offset":14468,"ancestor_record_offset":14468,"inlinee":59887,"source_line":1123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14524,"parent_record_offset":14468,"ancestor_record_offset":14468,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1249,"record_offset":14704,"parent_record_offset":14680,"ancestor_record_offset":14468,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1249,"record_offset":14800,"parent_record_offset":14776,"ancestor_record_offset":14468,"inlinee":14694,"source_line":40,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14752,"parent_record_offset":14728,"ancestor_record_offset":14468,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1249,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":14468,"inlinee":8588,"source_line":165,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15856,"parent_record_offset":15800,"ancestor_record_offset":14468,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15884,"parent_record_offset":15800,"ancestor_record_offset":14468,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":15256,"parent_record_offset":14556,"ancestor_record_offset":14468,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1249,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14468,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":15108,"parent_record_offset":14908,"ancestor_record_offset":14468,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1249,"record_offset":15060,"parent_record_offset":15028,"ancestor_record_offset":14468,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":15028,"parent_record_offset":15004,"ancestor_record_offset":14468,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1249,"record_offset":14828,"parent_record_offset":14800,"ancestor_record_offset":14468,"inlinee":14695,"source_line":182,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::Itertools::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/lib.rs
//     evidence={"module":1249,"record_offset":14656,"parent_record_offset":14556,"ancestor_record_offset":14468,"inlinee":11990,"source_line":699,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$6::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14884,"parent_record_offset":14556,"ancestor_record_offset":14468,"inlinee":8087,"source_line":177,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::tuple_windows
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.10.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/itertools-0.10.5/src/tuple_impl.rs
//     evidence={"module":1249,"record_offset":14680,"parent_record_offset":14656,"ancestor_record_offset":14468,"inlinee":8089,"source_line":147,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":14468,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::utils::half_timber::half_timbering::closure$6
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/utils/half_timber.rs
//     evidence={"module":1249,"record_offset":15180,"parent_record_offset":15156,"ancestor_record_offset":14468,"inlinee":59897,"source_line":326,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] take_until::take_until::impl$2::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"take-until","version":"0.2.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/take-until-0.2.0/src/take_until.rs
//     evidence={"module":1249,"record_offset":14728,"parent_record_offset":14704,"ancestor_record_offset":14468,"inlinee":59839,"source_line":89,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1249,"record_offset":14628,"parent_record_offset":14556,"ancestor_record_offset":14468,"inlinee":21358,"source_line":1108,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right_until
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":14556,"parent_record_offset":14524,"ancestor_record_offset":14468,"inlinee":59845,"source_line":240,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::impl$6::extend_right_until::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1249,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":14468,"inlinee":59822,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::u
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":15204,"parent_record_offset":15180,"ancestor_record_offset":14468,"inlinee":21354,"source_line":123,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$2::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1249,"record_offset":15288,"parent_record_offset":14556,"ancestor_record_offset":14468,"inlinee":59843,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 378
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E6
// 首条位置: {"module":1226,"inlinee":59622}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 529
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8F1
// 首条位置: {"module":1227,"inlinee":59633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":1880,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 556
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8BE
// 首条位置: {"module":1226,"inlinee":59582}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":22724,"parent_record_offset":22636,"ancestor_record_offset":22636,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":22756,"parent_record_offset":22636,"ancestor_record_offset":22636,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":22896,"parent_record_offset":22636,"ancestor_record_offset":22636,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":22696,"parent_record_offset":22636,"ancestor_record_offset":22636,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1249,"record_offset":22784,"parent_record_offset":22636,"ancestor_record_offset":22636,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":22944,"parent_record_offset":22920,"ancestor_record_offset":22636,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":22968,"parent_record_offset":22944,"ancestor_record_offset":22636,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":22808,"parent_record_offset":22784,"ancestor_record_offset":22636,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":22864,"parent_record_offset":22784,"ancestor_record_offset":22636,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":22920,"parent_record_offset":22896,"ancestor_record_offset":22636,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 863
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15DF6
// 首条位置: {"module":2698,"inlinee":89590}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1384,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2698,"record_offset":1764,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2698,"record_offset":1848,"parent_record_offset":1300,"ancestor_record_offset":1300,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":1300,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1300,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1920,"parent_record_offset":1896,"ancestor_record_offset":1300,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1408,"parent_record_offset":1384,"ancestor_record_offset":1300,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2698,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1300,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":2698,"record_offset":1872,"parent_record_offset":1848,"ancestor_record_offset":1300,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2698,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":1300,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2698,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1300,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2698,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":1300,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02698.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 949
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E8
// 首条位置: {"module":1226,"inlinee":59624}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1226,"record_offset":47240,"parent_record_offset":47212,"ancestor_record_offset":47212,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1226,"record_offset":47324,"parent_record_offset":47300,"ancestor_record_offset":47212,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1226,"record_offset":47348,"parent_record_offset":47324,"ancestor_record_offset":47212,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1226,"record_offset":47388,"parent_record_offset":47240,"ancestor_record_offset":47212,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1226,"record_offset":47416,"parent_record_offset":47240,"ancestor_record_offset":47212,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1226,"record_offset":47300,"parent_record_offset":47276,"ancestor_record_offset":47212,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1226,"record_offset":47276,"parent_record_offset":47240,"ancestor_record_offset":47212,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01226.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 955
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9FD
// 首条位置: {"module":1249,"inlinee":59901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1249,"record_offset":38984,"parent_record_offset":38956,"ancestor_record_offset":38956,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":39068,"parent_record_offset":39044,"ancestor_record_offset":38956,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1249,"record_offset":39092,"parent_record_offset":39068,"ancestor_record_offset":38956,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":39132,"parent_record_offset":38984,"ancestor_record_offset":38956,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1249,"record_offset":39160,"parent_record_offset":38984,"ancestor_record_offset":38956,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":39044,"parent_record_offset":39020,"ancestor_record_offset":38956,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1249,"record_offset":39020,"parent_record_offset":38984,"ancestor_record_offset":38956,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 958
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8F9
// 首条位置: {"module":1227,"inlinee":59641}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":28968,"parent_record_offset":28920,"ancestor_record_offset":28920,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 记录形式: inline_instance, inlinee_declaration; instances=32
// 源码行: 976
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8FA
// 首条位置: {"module":1227,"inlinee":59642}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=29
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":29856,"parent_record_offset":29808,"ancestor_record_offset":29808,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 1277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8F2
// 首条位置: {"module":1227,"inlinee":59634}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1227,"record_offset":7244,"parent_record_offset":7216,"ancestor_record_offset":7216,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01227.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::half_timbering::closure$17
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9FE
// 首条位置: {"module":1249,"inlinee":59902}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":52284,"parent_record_offset":52260,"ancestor_record_offset":52260,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":52352,"parent_record_offset":52312,"ancestor_record_offset":52260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":52384,"parent_record_offset":52312,"ancestor_record_offset":52260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1249,"record_offset":52312,"parent_record_offset":52284,"ancestor_record_offset":52260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":52420,"parent_record_offset":52384,"ancestor_record_offset":52260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::get_window_aabb
// 记录形式: compiled_procedure; instances=1
// 源码行: 1654
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1249,"record_offset":71372,"rva":13463360,"va":5382172480}
// 编译实例 RVAs: 0xcd6f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9423045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e83b3770ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382172752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141780130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11178376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e88891aa00","size":5,"soft_fail":false,"target_va":5393350960,"va":5382172579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20806061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e8ad793d01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382172814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_door_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7679235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e8fdd28aff","size":5,"soft_fail":false,"target_va":5374493328,"va":5382172558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7679775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e8e1d08aff","size":5,"soft_fail":false,"target_va":5374492896,"va":5382172666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7601549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e873028cff","size":5,"soft_fail":false,"target_va":5374570992,"va":5382172536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2247218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172480,"raw_hex":"e8324a2200","size":5,"soft_fail":false,"target_va":5384420000,"va":5382172777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":71628,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1249,"record_offset":71684,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":71720,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1249,"record_offset":71656,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1249,"record_offset":71932,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1249,"record_offset":71600,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1249,"record_offset":71572,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":21735,"source_line":70,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1249,"record_offset":71544,"parent_record_offset":71372,"ancestor_record_offset":71372,"inlinee":26222,"source_line":146,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":71888,"parent_record_offset":71864,"ancestor_record_offset":71372,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1249,"record_offset":71864,"parent_record_offset":71772,"ancestor_record_offset":71372,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":71800,"parent_record_offset":71772,"ancestor_record_offset":71372,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1249,"record_offset":71772,"parent_record_offset":71748,"ancestor_record_offset":71372,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1249,"record_offset":71828,"parent_record_offset":71800,"ancestor_record_offset":71372,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1249,"record_offset":71956,"parent_record_offset":71932,"ancestor_record_offset":71372,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1249,"record_offset":71748,"parent_record_offset":71720,"ancestor_record_offset":71372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::wrap_around
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1704
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9FB
// 首条位置: {"module":1249,"inlinee":59899}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u_circular::CurveUCircular::new
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1249,"record_offset":19728,"parent_record_offset":19692,"ancestor_record_offset":19692,"inlinee":21570,"source_line":31,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1249,"record_offset":19768,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$3::project_unbounded
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":1249,"record_offset":19820,"parent_record_offset":19796,"ancestor_record_offset":19692,"inlinee":7458,"source_line":154,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u_circular::CurveUCircular::wrap_around
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u_circular.rs
//     evidence={"module":1249,"record_offset":19796,"parent_record_offset":19728,"ancestor_record_offset":19692,"inlinee":21571,"source_line":90,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 记录形式: compiled_procedure; instances=1
// 源码行: 1708
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1249,"record_offset":72212,"rva":13463792,"va":5382172912}
// 编译实例 RVAs: 0xcd70f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405baba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: half_timber.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7456196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e83c3a8eff","size":5,"soft_fail":false,"target_va":5374716832,"va":5382173023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6960642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8fec995ff","size":5,"soft_fail":false,"target_va":5375212688,"va":5382173325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::utils::half_timber::get_window_aabb
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e89dfcffff","size":5,"soft_fail":false,"target_va":5382172480,"va":5382173342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2247075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8a3492200","size":5,"soft_fail":false,"target_va":5384420192,"va":5382173112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::contains
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2247121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8d1492200","size":5,"soft_fail":false,"target_va":5384420320,"va":5382173194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::from_center_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2246816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8a0482200","size":5,"soft_fail":false,"target_va":5384420000,"va":5382173179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2247594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8aa4b2200","size":5,"soft_fail":false,"target_va":5384420752,"va":5382173153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::width
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t2247590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382172912,"raw_hex":"e8a64b2200","size":5,"soft_fail":false,"target_va":5384420736,"va":5382173141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::impl$36::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":72380,"parent_record_offset":72212,"ancestor_record_offset":72212,"inlinee":8927,"source_line":2181,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":72756,"parent_record_offset":72212,"ancestor_record_offset":72212,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":72636,"parent_record_offset":72608,"ancestor_record_offset":72212,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1249,"record_offset":72696,"parent_record_offset":72664,"ancestor_record_offset":72212,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1249,"record_offset":72980,"parent_record_offset":72956,"ancestor_record_offset":72212,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1249,"record_offset":72580,"parent_record_offset":72552,"ancestor_record_offset":72212,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":73200,"parent_record_offset":73176,"ancestor_record_offset":72212,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1249,"record_offset":73176,"parent_record_offset":73152,"ancestor_record_offset":72212,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":72436,"parent_record_offset":72408,"ancestor_record_offset":72212,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1249,"record_offset":72784,"parent_record_offset":72756,"ancestor_record_offset":72212,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":73152,"parent_record_offset":72844,"ancestor_record_offset":72212,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72552,"parent_record_offset":72492,"ancestor_record_offset":72212,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72844,"parent_record_offset":72812,"ancestor_record_offset":72212,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72464,"parent_record_offset":72436,"ancestor_record_offset":72212,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72524,"parent_record_offset":72492,"ancestor_record_offset":72212,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72492,"parent_record_offset":72464,"ancestor_record_offset":72212,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":72924,"parent_record_offset":72892,"ancestor_record_offset":72212,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":72956,"parent_record_offset":72924,"ancestor_record_offset":72212,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1249,"record_offset":72892,"parent_record_offset":72844,"ancestor_record_offset":72212,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1249,"record_offset":72812,"parent_record_offset":72784,"ancestor_record_offset":72212,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":72608,"parent_record_offset":72552,"ancestor_record_offset":72212,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1249,"record_offset":72664,"parent_record_offset":72608,"ancestor_record_offset":72212,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1249,"record_offset":72408,"parent_record_offset":72380,"ancestor_record_offset":72212,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01249.jsonl.gz"}
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
// 完整结构及原始类型记录: half_timber.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: half_timber.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: half_timber.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>
// 信息 ID: dc68fd028ace2204c106cadc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_wall_constructor::utils::half_timber::add_underwindow_pattern -> ["system_wall_constructor::utils::half_timber::half_timbering::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd05db -> 0x17753a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows
// 信息 ID: bd22f14dd143e5706512f719
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0541 -> 0xcd70f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xccf767 -> 0xcd70f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::get_window_aabb
// 信息 ID: 5ce6d1516debdbbb650ec6d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::overlapping_windows @ 0xcd6bf0 -> 0xcd6f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows @ 0xcd729e -> 0xcd6f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering
// 信息 ID: 177baa7d3a105a84e49f8e85
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eace8 -> 0xcce2c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$0
// 信息 ID: c2396fe844ac2927284006ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$0 <- system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=26800; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=31388; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=48584; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=50668; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=53656; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=59164; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=60392; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0>; module=2698, inline_record=5112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$1
// 信息 ID: d8aa243df142ec147fcbd567
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1226, inline_record=62736; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes::closure_env$0>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$1> >; module=1226, inline_record=63500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$10
// 信息 ID: f8b7acc2e29428a052edf405
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 10, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=22636; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=23120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$11
// 信息 ID: ed533d98db3e8d93c92d8349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 11, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<utils::geometry::aabb::Aabb2> >,alloc::vec::Vec<tuple$<usize,f32,f32>,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$11 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<usize,ref$<utils::geometry::aabb::Aabb2> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$11>; module=2698, inline_record=1300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$12
// 信息 ID: 7c1d77118dc671bdf1653b3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 12, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$12 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$12> >; module=1226, inline_record=47212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$13
// 信息 ID: dff2edcb0e33de9456b11ce1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 13, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=38956; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$14
// 信息 ID: d24ac0632d10e8c89b91122f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 14, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$14 <- core::slice::sort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=28920; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=32128; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=32484; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=40844; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=41084; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=58216; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=58364; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$14> >; module=1227, inline_record=58512; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$15
// 信息 ID: bb0e6e1deb7f0b4ce6a7696c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 15, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$15 <- core::slice::sort::heapsort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,ref_mut$<core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 29 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::slice::sort::partial_insertion_sort<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=29808; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=36528; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=36884; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=39852; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_right<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=40092; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=50756; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=50904; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::recurse<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,core::slice::impl$0::sort_unstable_by::closure_env$0<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$15> >; module=1227, inline_record=51052; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$16
// 信息 ID: 3e698a389a32a4f40f3b3ecc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 16, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$16 <- core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; module=1227, inline_record=7216; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; module=1227, inline_record=7348; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; module=1227, inline_record=7460; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; module=1227, inline_record=8500; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16>,alloc::slice::stable_sort::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$1<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$2<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >,alloc::slice::stable_sort::closure_env$3<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> > >; module=1227, inline_record=8836; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1227, inline_record=35320; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,alloc::slice::impl$0::sort_by::closure_env$0<tuple$<f32,glam::f32::vec3::Vec3,glam::f32::vec2::Vec2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$16> >; module=1227, inline_record=35720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$17
// 信息 ID: 6f6d86e504a26fabfaf6af0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 17, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=52260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$2
// 信息 ID: e21078487559cd9553c320e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$2 <- core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; module=1227, inline_record=23268; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; module=1227, inline_record=23396; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; module=1227, inline_record=23496; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; module=1227, inline_record=24892; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> > >; module=1227, inline_record=25228; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1227, inline_record=33160; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$2> >; module=1227, inline_record=33516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$3
// 信息 ID: 929606453c9a8e94864e1d09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=12308; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=12516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$4
// 信息 ID: 73d71c97dc3cae2654d79dd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$4 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >; module=1226, inline_record=25180; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<f32>,tuple$<ref$<f32>,ref$<f32> > >,system_wall_constructor::utils::half_timber::half_timbering::closure_env$4> >; module=1226, inline_record=25772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$5
// 信息 ID: ee07727c87314bd90f10382c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::ptr::drop_in_place<core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::iter::adapters::flatten::impl$3::next<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::aabb::Aabb2,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<usize>,alloc::vec::Vec<utils::geometry::aabb::Aabb2,alloc::alloc::Global>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$5 <- core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; module=2698, inline_record=480; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$5::closure$0
// 信息 ID: be9b1618e482ce1d6871a376
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<usize>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$5>; module=2698, inline_record=772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$6
// 信息 ID: ae0618a15a3ba2c6dde322b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=15180; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=15688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$7
// 信息 ID: 6a14beaf320637261b4afcfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=14468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$8
// 信息 ID: ac928fbfdcc620d6be6fc9a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$8 <- alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> >; module=1226, inline_record=44644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::half_timbering::closure$9
// 信息 ID: 7d5da10eb508c587d926f3c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "system_wall_constructor::utils::half_timber::half_timbering", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::half_timbering::closure_env$9 <- core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 6 条。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; module=1227, inline_record=1852; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; module=1227, inline_record=1980; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; module=1227, inline_record=2080; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; module=1227, inline_record=3476; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::merge_sort<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9>,alloc::slice::stable_sort::closure_env$0<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$1<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$2<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >,alloc::slice::stable_sort::closure_env$3<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> > >; module=1227, inline_record=3812; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1227, inline_record=37560; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::insertion_sort_shift_left<f32,alloc::slice::impl$0::sort_by::closure_env$0<f32,system_wall_constructor::utils::half_timber::half_timbering::closure_env$9> >; module=1227, inline_record=37916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::overlapping_windows
// 信息 ID: 4eb0809bb070b206da8b3791
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<alloc::vec::Vec<system_wall_constructor::utils::half_timber::WindowOverlappingWithThisRow,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::aabb::Aabb2>,system_wall_constructor::utils::half_timber::half_timbering::closure_env$8> > @ 0xcac1a8 -> 0xcd6a50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline
// 信息 ID: f39dec7cc2520c0552dfcaac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4ead28 -> 0xccce00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$0
// 信息 ID: 954e2eab63a65f1c53ab8e37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=7988; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=9176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$1
// 信息 ID: 603928423ba05b009e962bd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=1368; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=1572; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=1916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$2
// 信息 ID: ebd36de08bfe493205ac72f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// [闭包环境类型引用] system_wall_constructor::utils::half_timber::roof_outline::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1226, inline_record=27820; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env$2> >; module=1226, inline_record=28096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::roof_outline::closure$3
// 信息 ID: 6b2007a15b46e42c89632e76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::utils::half_timber::roof_outline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::roof_outline; module=1249, inline_record=3216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::utils::half_timber::wrap_around
// 信息 ID: 51eb9192405b6e6a331fef52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=19692; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=19864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
