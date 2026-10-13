// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/geometry/sphere.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0eee74e33c2dd307830a3af7771fbcaaab9bde840fa9246ae3828a0c6ccbcbcc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sphere.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 397, 1308, 1321, 1365, 1395
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
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.24.1"}]
// use hexasphere; // namespace candidate
//   package identities: [{"name":"hexasphere","source":"PDB_registry_path_only","version":"9.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::Sphere::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":397,"record_offset":3332,"rva":5482416,"va":5374191536}
// 编译实例 RVAs: 0x53a7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1365,"record_offset":13768,"rva":15856112,"va":5384565232}
// 编译实例 RVAs: 0xf1f1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11941052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e844cb49ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5384566071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11815809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e87fb44bff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384565500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11817084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e884af4bff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384566791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140543910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10336708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e83c4662ff","size":5,"soft_fail":false,"target_va":5374228752,"va":5384565455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9830129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e80f016aff","size":5,"soft_fail":false,"target_va":5374736096,"va":5384566220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bfc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9828832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e820066aff","size":5,"soft_fail":false,"target_va":5374737536,"va":5384566363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8540
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8679782","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e89a8e7bff","size":5,"soft_fail":false,"target_va":5375886656,"va":5384566433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8631219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e84d4c7cff","size":5,"soft_fail":false,"target_va":5375936112,"va":5384567326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d79250
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1728899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e87d9ee5ff","size":5,"soft_fail":false,"target_va":5382836816,"va":5384565710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sphere.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19027549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e85d562201","size":5,"soft_fail":false,"target_va":5403593360,"va":5384565806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18411622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e866f01801","size":5,"soft_fail":false,"target_va":5402978144,"va":5384566517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7035529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e877a594ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384565412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7039423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e8419694ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384566538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18412837","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e825f51801","size":5,"soft_fail":false,"target_va":5402979344,"va":5384566502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hexasphere::Triangle::add_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t447309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e84dd30600","size":5,"soft_fail":false,"target_va":5385013328,"va":5384566014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7199802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384565232,"raw_hex":"e8c62392ff","size":5,"soft_fail":false,"target_va":5377366224,"va":5384566021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1365,"record_offset":13940,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1365,"record_offset":15112,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":15732,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1365,"record_offset":16072,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1365,"record_offset":14052,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":16756,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1365,"record_offset":16260,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::get_indices
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"9.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hexasphere-9.1.0/src/lib.rs
//     evidence={"module":1365,"record_offset":16224,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":22978,"source_line":1138,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::indices_per_main_triangle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"9.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hexasphere-9.1.0/src/lib.rs
//     evidence={"module":1365,"record_offset":15704,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":63329,"source_line":1240,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::raw_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"9.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hexasphere-9.1.0/src/lib.rs
//     evidence={"module":1365,"record_offset":15084,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":22980,"source_line":1226,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_direct] hexasphere::Subdivided::raw_points
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hexasphere","version":"9.1.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hexasphere-9.1.0/src/lib.rs
//     evidence={"module":1365,"record_offset":14024,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":22977,"source_line":1122,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":14384,"parent_record_offset":14360,"ancestor_record_offset":13768,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":13768,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":16932,"parent_record_offset":16908,"ancestor_record_offset":13768,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":14360,"parent_record_offset":14268,"ancestor_record_offset":13768,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1365,"record_offset":16908,"parent_record_offset":16808,"ancestor_record_offset":13768,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1365,"record_offset":13988,"parent_record_offset":13964,"ancestor_record_offset":13768,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":16840,"parent_record_offset":16808,"ancestor_record_offset":13768,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":14268,"parent_record_offset":14240,"ancestor_record_offset":13768,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":14240,"parent_record_offset":14212,"ancestor_record_offset":13768,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":14480,"parent_record_offset":14240,"ancestor_record_offset":13768,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1365,"record_offset":16808,"parent_record_offset":16780,"ancestor_record_offset":13768,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1365,"record_offset":15256,"parent_record_offset":15220,"ancestor_record_offset":13768,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1365,"record_offset":15220,"parent_record_offset":15184,"ancestor_record_offset":13768,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1365,"record_offset":15148,"parent_record_offset":15112,"ancestor_record_offset":13768,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1365,"record_offset":15184,"parent_record_offset":15148,"ancestor_record_offset":13768,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13768,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":14184,"parent_record_offset":14148,"ancestor_record_offset":13768,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":14212,"parent_record_offset":14184,"ancestor_record_offset":13768,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":14084,"parent_record_offset":14052,"ancestor_record_offset":13768,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1365,"record_offset":14828,"parent_record_offset":14792,"ancestor_record_offset":13768,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1365,"record_offset":14924,"parent_record_offset":14828,"ancestor_record_offset":13768,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1365,"record_offset":14520,"parent_record_offset":14148,"ancestor_record_offset":13768,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1365,"record_offset":14116,"parent_record_offset":14084,"ancestor_record_offset":13768,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1365,"record_offset":14148,"parent_record_offset":14116,"ancestor_record_offset":13768,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1365,"record_offset":14304,"parent_record_offset":14268,"ancestor_record_offset":13768,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1365,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":13768,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":16584,"parent_record_offset":16556,"ancestor_record_offset":13768,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":16636,"parent_record_offset":16612,"ancestor_record_offset":13768,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1365,"record_offset":16612,"parent_record_offset":16556,"ancestor_record_offset":13768,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1365,"record_offset":16128,"parent_record_offset":16100,"ancestor_record_offset":13768,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1365,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":13768,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1365,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":13768,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1365,"record_offset":14592,"parent_record_offset":14568,"ancestor_record_offset":13768,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1365,"record_offset":14652,"parent_record_offset":14616,"ancestor_record_offset":13768,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1365,"record_offset":16156,"parent_record_offset":16100,"ancestor_record_offset":13768,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1365,"record_offset":16100,"parent_record_offset":16072,"ancestor_record_offset":13768,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1365,"record_offset":14568,"parent_record_offset":14544,"ancestor_record_offset":13768,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1365,"record_offset":14792,"parent_record_offset":14652,"ancestor_record_offset":13768,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1365,"record_offset":16660,"parent_record_offset":16636,"ancestor_record_offset":13768,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1365,"record_offset":16180,"parent_record_offset":16156,"ancestor_record_offset":13768,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1365,"record_offset":16872,"parent_record_offset":16840,"ancestor_record_offset":13768,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1365,"record_offset":16512,"parent_record_offset":16488,"ancestor_record_offset":13768,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1365,"record_offset":13964,"parent_record_offset":13940,"ancestor_record_offset":13768,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1365,"record_offset":15628,"parent_record_offset":15256,"ancestor_record_offset":13768,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":16780,"parent_record_offset":16756,"ancestor_record_offset":13768,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1365,"record_offset":14964,"parent_record_offset":14616,"ancestor_record_offset":13768,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":14436,"parent_record_offset":14408,"ancestor_record_offset":13768,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1365,"record_offset":14872,"parent_record_offset":14828,"ancestor_record_offset":13768,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1365,"record_offset":14616,"parent_record_offset":14592,"ancestor_record_offset":13768,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] country_core::geometry::sphere::impl$2::try_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/geometry/sphere.rs
//     evidence={"module":1365,"record_offset":14684,"parent_record_offset":14652,"ancestor_record_offset":13768,"inlinee":61556,"source_line":83,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1365,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":13768,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.24.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.24.1/src/f32/sse2/vec3a.rs
//     evidence={"module":1365,"record_offset":14716,"parent_record_offset":14684,"ancestor_record_offset":13768,"inlinee":23066,"source_line":851,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1365,"record_offset":16312,"parent_record_offset":16284,"ancestor_record_offset":13768,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":16360,"parent_record_offset":16336,"ancestor_record_offset":13768,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1365,"record_offset":16436,"parent_record_offset":16408,"ancestor_record_offset":13768,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":16556,"parent_record_offset":16408,"ancestor_record_offset":13768,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1365,"record_offset":16336,"parent_record_offset":16312,"ancestor_record_offset":13768,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1365,"record_offset":16460,"parent_record_offset":16436,"ancestor_record_offset":13768,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1365,"record_offset":16488,"parent_record_offset":16460,"ancestor_record_offset":13768,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1365,"record_offset":16408,"parent_record_offset":16384,"ancestor_record_offset":13768,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1365,"record_offset":16384,"parent_record_offset":16360,"ancestor_record_offset":13768,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF07F
// 首条位置: {"module":1308,"inlinee":61567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::acos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":332356,"parent_record_offset":332320,"ancestor_record_offset":332320,"inlinee":27691,"source_line":818,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1308,"record_offset":332384,"parent_record_offset":332320,"ancestor_record_offset":332320,"inlinee":13077,"source_line":886,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::geometry::sphere::impl$2::try_from::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF074
// 首条位置: {"module":1308,"inlinee":61556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.24.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.24.1/src/f32/sse2/vec3a.rs
//     evidence={"module":1365,"record_offset":14716,"parent_record_offset":14684,"ancestor_record_offset":14684,"inlinee":23066,"source_line":851,"archive":"_pdb/A/modules/01365.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1365,"record_offset":14748,"parent_record_offset":14716,"ancestor_record_offset":14684,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/01365.jsonl.gz"}
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
// 完整结构及原始类型记录: sphere.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::geometry::sphere::Sphere
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sphere.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sphere.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::Sphere::new
// 信息 ID: 3a71edbad67c4fec559dde1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43d60 -> 0x53a7b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from
// 信息 ID: 9cc7469e96d935440fd38672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x66e90b -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::spawn_brush_previews::spawn_brush_previews @ 0x6991a8 -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xed96cc -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xed98a0 -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xed9a6a -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xed9c3d -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::debug::debug @ 0xed9e12 -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43d6e -> 0xf1f1f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$0
// 信息 ID: 4b979c7ae90012abbc070946
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// [闭包环境类型引用] country_core::geometry::sphere::impl$2::try_from::closure_env$0 <- hexasphere::Subdivided::new<array$<f32,2>,hexasphere::shapes::IcoSphereBase,country_core::geometry::sphere::impl$2::try_from::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::geometry::sphere::impl$2::try_from::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec3a::Vec3A> >,country_core::geometry::sphere::impl$2::try_from::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,2>,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec3a::Vec3A> >,country_core::geometry::sphere::impl$2::try_from::closure_env$0> >; module=1308, inline_record=332320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::geometry::sphere::impl$2::try_from::closure$1
// 信息 ID: 3b61a6cf62293717fad07947
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::geometry::sphere::impl$2::try_from", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::geometry::sphere::impl$2::try_from; module=1365, inline_record=14684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
