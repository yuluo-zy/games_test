// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/roofs/roof_ridge.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: bb54c965635762d1fc8bd4dfc52e579bf8fd061890ddb1074ffe4c64dbcc5ee0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: roof_ridge.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 391, 420, 492, 493
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use emath; // namespace candidate
//   package identities: [{"name":"emath","source":"PDB_registry_path_only","version":"0.27.2"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":420,"record_offset":10536,"rva":5879696,"va":5374588816}
// 编译实例 RVAs: 0x59b790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: roof_ridge.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2354808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374588816,"raw_hex":"e878ee2300","size":5,"soft_fail":false,"target_va":5376943968,"va":5374589155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28388811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374588816,"raw_hex":"e8cb2db101","size":5,"soft_fail":false,"target_va":5402978944,"va":5374590128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1176265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374588816,"raw_hex":"e8c9f21100","size":5,"soft_fail":false,"target_va":5375765488,"va":5374589218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::orientation_mat2
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t3366380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374588816,"raw_hex":"e8ec5d3300","size":5,"soft_fail":false,"target_va":5377955440,"va":5374589055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":420,"record_offset":13612,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":420,"record_offset":10720,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":24278,"source_line":27,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":420,"record_offset":10776,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":24280,"source_line":118,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":420,"record_offset":11188,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":24282,"source_line":72,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":11856,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":24283,"source_line":159,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":420,"record_offset":11036,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":420,"record_offset":10944,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":420,"record_offset":11156,"parent_record_offset":11036,"ancestor_record_offset":10536,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":420,"record_offset":11100,"parent_record_offset":11036,"ancestor_record_offset":10536,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":420,"record_offset":11068,"parent_record_offset":11036,"ancestor_record_offset":10536,"inlinee":21773,"source_line":943,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":420,"record_offset":11128,"parent_record_offset":11036,"ancestor_record_offset":10536,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":10536,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":420,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":10536,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":420,"record_offset":10972,"parent_record_offset":10944,"ancestor_record_offset":10536,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":11312,"parent_record_offset":11188,"ancestor_record_offset":10536,"inlinee":22264,"source_line":111,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":10536,"inlinee":22268,"source_line":169,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":11348,"parent_record_offset":11312,"ancestor_record_offset":10536,"inlinee":22265,"source_line":102,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat2::Mat2::mul_vec2
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat2.rs
//     evidence={"module":420,"record_offset":12036,"parent_record_offset":11856,"ancestor_record_offset":10536,"inlinee":21772,"source_line":273,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":420,"record_offset":10828,"parent_record_offset":10776,"ancestor_record_offset":10536,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":420,"record_offset":11980,"parent_record_offset":11856,"ancestor_record_offset":10536,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":420,"record_offset":11216,"parent_record_offset":11188,"ancestor_record_offset":10536,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":420,"record_offset":11384,"parent_record_offset":11188,"ancestor_record_offset":10536,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":420,"record_offset":9768,"rva":1104432,"va":5369813552}
// 编译实例 RVAs: 0x10da30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5ED6
// 首条位置: {"module":420,"inlinee":24278}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":10744,"parent_record_offset":10720,"ancestor_record_offset":10720,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56FB
// 首条位置: {"module":391,"inlinee":22267}
// 编译实例 RVAs: 0x59b6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_ridge.rs
//     evidence={"module":391,"record_offset":26508,"parent_record_offset":26444,"ancestor_record_offset":26444,"inlinee":22269,"source_line":42,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":391,"record_offset":26480,"parent_record_offset":26444,"ancestor_record_offset":26444,"inlinee":22268,"source_line":169,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26608,"parent_record_offset":26556,"ancestor_record_offset":26444,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26664,"parent_record_offset":26556,"ancestor_record_offset":26444,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26580,"parent_record_offset":26556,"ancestor_record_offset":26444,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":391,"record_offset":26556,"parent_record_offset":26508,"ancestor_record_offset":26444,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=10
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x56FD
// 首条位置: {"module":391,"inlinee":22269}
// 编译实例 RVAs: 0x59b740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":391,"record_offset":26556,"parent_record_offset":26508,"ancestor_record_offset":26508,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26608,"parent_record_offset":26556,"ancestor_record_offset":26508,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26664,"parent_record_offset":26556,"ancestor_record_offset":26508,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":26580,"parent_record_offset":26556,"ancestor_record_offset":26508,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EDA
// 首条位置: {"module":420,"inlinee":24282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::height
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":11312,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":22264,"source_line":111,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":420,"record_offset":11216,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":420,"record_offset":11384,"parent_record_offset":11188,"ancestor_record_offset":11188,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":420,"record_offset":11408,"parent_record_offset":11384,"ancestor_record_offset":11188,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":420,"record_offset":11248,"parent_record_offset":11216,"ancestor_record_offset":11188,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::roofs::roof_shape::Roof::ty
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":11348,"parent_record_offset":11312,"ancestor_record_offset":11188,"inlinee":22265,"source_line":102,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5ED8
// 首条位置: {"module":420,"inlinee":24280}
// 编译实例 RVAs: 0x59bcc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::roofs::roof_shape::Roof::shape_dims
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/roofs/roof_shape.rs
//     evidence={"module":420,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":22268,"source_line":169,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":420,"record_offset":10828,"parent_record_offset":10776,"ancestor_record_offset":10776,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":420,"record_offset":10852,"parent_record_offset":10828,"ancestor_record_offset":10776,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":420,"record_offset":13996,"rva":5881088,"va":5374590208}
// 编译实例 RVAs: 0x59bd00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28388566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374590208,"raw_hex":"e8d62cb101","size":5,"soft_fail":false,"target_va":5402978880,"va":5374590309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28388814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374590208,"raw_hex":"e8ce2db101","size":5,"soft_fail":false,"target_va":5402979152,"va":5374590333,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":14252,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":14280,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":14336,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":420,"record_offset":14220,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] emath::inverse_lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"emath","version":"0.27.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/emath-0.27.2/src/lib.rs
//     evidence={"module":420,"record_offset":14164,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":24284,"source_line":125,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$33::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":420,"record_offset":14192,"parent_record_offset":14164,"ancestor_record_offset":13996,"inlinee":4544,"source_line":1469,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":14304,"parent_record_offset":14280,"ancestor_record_offset":13996,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00420.jsonl.gz"}
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
// 完整结构及原始类型记录: roof_ridge.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::roofs::roof_ridge::RoofRidgeDims
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 4
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: roof_ridge.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: roof_ridge.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::calc
// 信息 ID: b11a13e2890ffe803fc8d1e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12eaa2e -> 0x59b740 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=26508; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::from; module=420, inline_record=10088; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=492, inline_record=15476; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=493, inline_record=16576; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=493, inline_record=21152; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::from
// 信息 ID: efe6426010e5c0b110c61da3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcc30 -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::roofs::roof_shape::Roof>,ref$<country_core::resources::roofs::roof_animation::RoofAnimation> > >,system_roof::visual::place_spires::place_roof_spires::closure_env$0> @ 0x12bf04c -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2c49 -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2fb9 -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x198c31c -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x198d15c -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3c922 -> 0x59b6d0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=26444; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom; module=492, inline_record=15440; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::roof_shape_at_intersection_y; module=493, inline_record=16540; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::Roof::get_roof_shape; module=493, inline_record=21116; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::get_inner
// 信息 ID: bf068d22ba56a3c1c922ebac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 213 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9f1 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfd8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5c8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec0f -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f19f -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f727 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbcb -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201a0 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<ref$<alloc::string::String> > > @ 0x20688 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableBool> @ 0x20c4d -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::RefreshRateMillihertz> > @ 0x210f8 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a22d -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a309 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a3ce -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::language_menu::do_gui_language_menu @ 0x6a499 -> 0x10da30 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game @ 0x6c120 -> 0x10da30 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::max_dim
// 信息 ID: 28f044d7d8931693a8fcd4e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=10720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space
// 信息 ID: 7dbb15caf42bad8792386eeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcc51 -> 0x59b790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2c66 -> 0x59b790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2fd0 -> 0x59b790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_length::create_ui_widget @ 0x198c339 -> 0x59b790 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_focused::ui_edit_roof_height_focused::ui_edit_roof_ridge_and_height_focused @ 0x198d179 -> 0x59b790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls
// 信息 ID: ea73df7186ee6a6887bf95fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dbec5 -> 0x59bcc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd017 -> 0x59bcc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::roof_trigger_volume::calculate_roof_trigger_volumes @ 0x1983910 -> 0x59bcc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3c950 -> 0x59bcc0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=10776; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::impl$0::to_world_space::closure$0
// 信息 ID: e39046492191c478d9a24e00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::roofs::roof_ridge::impl$0::to_world_space", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=11188; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=11704; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=12296; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=12796; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=13312; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space; module=420, inline_record=13464; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::roofs::roof_ridge::inverse_ridge_01_param
// 信息 ID: 9d78116c02d6cad43791c7dd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
