// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/render/nani/mesh_atlas.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0d75d120b08acbd59a2dd00aa11e7ade8fd57387f47b788e74f41d35a3f0eef1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh_atlas.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 398, 407, 410, 475, 1308, 1321
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$3::default
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5956
// 首条位置: {"module":398,"inlinee":22870}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":398,"record_offset":40568,"parent_record_offset":40540,"ancestor_record_offset":40540,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":40908,"parent_record_offset":40880,"ancestor_record_offset":40540,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":40960,"parent_record_offset":40936,"ancestor_record_offset":40540,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":398,"record_offset":40936,"parent_record_offset":40880,"ancestor_record_offset":40540,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":398,"record_offset":40984,"parent_record_offset":40960,"ancestor_record_offset":40540,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":398,"record_offset":40832,"parent_record_offset":40804,"ancestor_record_offset":40540,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":398,"record_offset":41008,"parent_record_offset":40984,"ancestor_record_offset":40540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":398,"record_offset":40600,"parent_record_offset":40568,"ancestor_record_offset":40540,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":398,"record_offset":40656,"parent_record_offset":40628,"ancestor_record_offset":40540,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":398,"record_offset":40744,"parent_record_offset":40712,"ancestor_record_offset":40540,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":398,"record_offset":40880,"parent_record_offset":40712,"ancestor_record_offset":40540,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":398,"record_offset":40628,"parent_record_offset":40600,"ancestor_record_offset":40540,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":398,"record_offset":40772,"parent_record_offset":40744,"ancestor_record_offset":40540,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":398,"record_offset":40804,"parent_record_offset":40772,"ancestor_record_offset":40540,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":398,"record_offset":40712,"parent_record_offset":40684,"ancestor_record_offset":40540,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":398,"record_offset":40684,"parent_record_offset":40656,"ancestor_record_offset":40540,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00398.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$8::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5BCD
// 首条位置: {"module":410,"inlinee":23501}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":32588,"rva":5845584,"va":5374554704}
// 编译实例 RVAs: 0x593250
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 记录形式: compiled_procedure; instances=1
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":32724,"rva":6721680,"va":5375430800}
// 编译实例 RVAs: 0x669090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5816733","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8633ea7ff","size":5,"soft_fail":false,"target_va":5369614224,"va":5375430952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5818627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8fd36a7ff","size":5,"soft_fail":false,"target_va":5369614592,"va":5375433214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2682182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8ba12d7ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375431873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2683273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8770ed7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375432980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t504019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8d3b00700","size":5,"soft_fail":false,"target_va":5375936112,"va":5375432088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d75820
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7389307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e87bc07000","size":5,"soft_fail":false,"target_va":5382821920,"va":5375432608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140dee140
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7883949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8ad4c7800","size":5,"soft_fail":false,"target_va":5383315776,"va":5375431822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df1610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7894898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e872777800","size":5,"soft_fail":false,"target_va":5383329296,"va":5375434393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea7610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8642030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8eedd8300","size":5,"soft_fail":false,"target_va":5384074768,"va":5375432733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f35470
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9223692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e80cbe8c00","size":5,"soft_fail":false,"target_va":5384655984,"va":5375432287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27658722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8e209a601","size":5,"soft_fail":false,"target_va":5403091664,"va":5375432937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28160516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e804b2ad01","size":5,"soft_fail":false,"target_va":5403593344,"va":5375432823,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh_atlas.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28161456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8b0b5ad01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375431899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27544256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8c04aa401","size":5,"soft_fail":false,"target_va":5402978144,"va":5375433883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2096955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e83bff1f00","size":5,"soft_fail":false,"target_va":5377529888,"va":5375432928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27484928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e80063a301","size":5,"soft_fail":false,"target_va":5402918528,"va":5375433595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4111989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e875be3e00","size":5,"soft_fail":false,"target_va":5379544224,"va":5375432230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2093247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8bff01f00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375433868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27545265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8b14ea401","size":5,"soft_fail":false,"target_va":5402978912,"va":5375433642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27546729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e86954a401","size":5,"soft_fail":false,"target_va":5402979152,"va":5375432418,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27545172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e8544ea401","size":5,"soft_fail":false,"target_va":5402978944,"va":5375433767,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::VertexAttributeValues::append
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9138176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375430800,"raw_hex":"e800708b00","size":5,"soft_fail":false,"target_va":5384571280,"va":5375433099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":475,"record_offset":40996,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":475,"record_offset":40408,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":8700,"source_line":207,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":39192,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":36684,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::partial_eq::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":475,"record_offset":40520,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":13790,"source_line":18,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":475,"record_offset":43952,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":475,"record_offset":35408,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":475,"record_offset":39852,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":39360,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":41080,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":475,"record_offset":33000,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":26960,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":475,"record_offset":32944,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":24591,"source_line":28,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::vertex_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":475,"record_offset":35724,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":23397,"source_line":205,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::VertexAttributeValues::as_f32x3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":475,"record_offset":35340,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":26961,"source_line":282,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":475,"record_offset":37656,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":23399,"source_line":11,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":39332,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::keys
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":39416,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":17284,"source_line":361,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$37::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":41520,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":17308,"source_line":2193,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$42::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":41880,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":8351,"source_line":2269,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":34200,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":17187,"source_line":1342,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":475,"record_offset":35696,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":38088,"parent_record_offset":38064,"ancestor_record_offset":32724,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":38112,"parent_record_offset":38088,"ancestor_record_offset":32724,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":39032,"parent_record_offset":39008,"ancestor_record_offset":32724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":38064,"parent_record_offset":37972,"ancestor_record_offset":32724,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":39008,"parent_record_offset":38984,"ancestor_record_offset":32724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":475,"record_offset":41044,"parent_record_offset":41020,"ancestor_record_offset":32724,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":36888,"parent_record_offset":36864,"ancestor_record_offset":32724,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":41164,"parent_record_offset":41132,"ancestor_record_offset":32724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":36864,"parent_record_offset":36832,"ancestor_record_offset":32724,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":36964,"parent_record_offset":36936,"ancestor_record_offset":32724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":36832,"parent_record_offset":36804,"ancestor_record_offset":32724,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":37972,"parent_record_offset":37940,"ancestor_record_offset":32724,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":37940,"parent_record_offset":37908,"ancestor_record_offset":32724,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":38184,"parent_record_offset":37940,"ancestor_record_offset":32724,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":475,"record_offset":38984,"parent_record_offset":38960,"ancestor_record_offset":32724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":475,"record_offset":37868,"parent_record_offset":37836,"ancestor_record_offset":32724,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":475,"record_offset":37836,"parent_record_offset":37804,"ancestor_record_offset":32724,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":475,"record_offset":37804,"parent_record_offset":37776,"ancestor_record_offset":32724,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":475,"record_offset":40432,"parent_record_offset":40408,"ancestor_record_offset":32724,"inlinee":6490,"source_line":842,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":475,"record_offset":40840,"parent_record_offset":40812,"ancestor_record_offset":32724,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":36936,"parent_record_offset":36756,"ancestor_record_offset":32724,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":36756,"parent_record_offset":36720,"ancestor_record_offset":32724,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":32968,"parent_record_offset":32944,"ancestor_record_offset":32724,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":36804,"parent_record_offset":36756,"ancestor_record_offset":32724,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37908,"parent_record_offset":37868,"ancestor_record_offset":32724,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37776,"parent_record_offset":37656,"ancestor_record_offset":32724,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":39876,"parent_record_offset":39852,"ancestor_record_offset":32724,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":475,"record_offset":37272,"parent_record_offset":37228,"ancestor_record_offset":32724,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":475,"record_offset":40868,"parent_record_offset":40840,"ancestor_record_offset":32724,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":475,"record_offset":37372,"parent_record_offset":37272,"ancestor_record_offset":32724,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":475,"record_offset":37616,"parent_record_offset":36756,"ancestor_record_offset":32724,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":475,"record_offset":37556,"parent_record_offset":37528,"ancestor_record_offset":32724,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":475,"record_offset":36720,"parent_record_offset":36684,"ancestor_record_offset":32724,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":475,"record_offset":38008,"parent_record_offset":37972,"ancestor_record_offset":32724,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":475,"record_offset":38032,"parent_record_offset":38008,"ancestor_record_offset":32724,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":475,"record_offset":38888,"parent_record_offset":38412,"ancestor_record_offset":32724,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":40812,"parent_record_offset":40784,"ancestor_record_offset":32724,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":40688,"parent_record_offset":40656,"ancestor_record_offset":32724,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":33996,"parent_record_offset":33968,"ancestor_record_offset":32724,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":40784,"parent_record_offset":40592,"ancestor_record_offset":32724,"inlinee":17672,"source_line":1658,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":36056,"parent_record_offset":36028,"ancestor_record_offset":32724,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":33356,"parent_record_offset":33332,"ancestor_record_offset":32724,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":33440,"parent_record_offset":33416,"ancestor_record_offset":32724,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":475,"record_offset":35556,"parent_record_offset":35528,"ancestor_record_offset":32724,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":475,"record_offset":35376,"parent_record_offset":35340,"ancestor_record_offset":32724,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":33380,"parent_record_offset":33356,"ancestor_record_offset":32724,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":475,"record_offset":37036,"parent_record_offset":37000,"ancestor_record_offset":32724,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":475,"record_offset":35504,"parent_record_offset":35432,"ancestor_record_offset":32724,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":475,"record_offset":40716,"parent_record_offset":40656,"ancestor_record_offset":32724,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":475,"record_offset":40656,"parent_record_offset":40628,"ancestor_record_offset":32724,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":475,"record_offset":40628,"parent_record_offset":40592,"ancestor_record_offset":32724,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":475,"record_offset":40372,"parent_record_offset":40348,"ancestor_record_offset":32724,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":475,"record_offset":37000,"parent_record_offset":36756,"ancestor_record_offset":32724,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":475,"record_offset":37228,"parent_record_offset":37148,"ancestor_record_offset":32724,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":37412,"parent_record_offset":37072,"ancestor_record_offset":32724,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":41196,"parent_record_offset":41164,"ancestor_record_offset":32724,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":33556,"parent_record_offset":33532,"ancestor_record_offset":32724,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":39384,"parent_record_offset":39360,"ancestor_record_offset":32724,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":36648,"parent_record_offset":35724,"ancestor_record_offset":32724,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":41020,"parent_record_offset":40996,"ancestor_record_offset":32724,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":38296,"parent_record_offset":37656,"ancestor_record_offset":32724,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":39648,"parent_record_offset":39616,"ancestor_record_offset":32724,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":475,"record_offset":38224,"parent_record_offset":37868,"ancestor_record_offset":32724,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":37444,"parent_record_offset":37072,"ancestor_record_offset":32724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":33900,"parent_record_offset":33876,"ancestor_record_offset":32724,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":33792,"parent_record_offset":33768,"ancestor_record_offset":32724,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":35476,"parent_record_offset":35432,"ancestor_record_offset":32724,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":38140,"parent_record_offset":38112,"ancestor_record_offset":32724,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":37324,"parent_record_offset":37272,"ancestor_record_offset":32724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::SlicePartialEq::not_equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":475,"record_offset":40568,"parent_record_offset":40544,"ancestor_record_offset":32724,"inlinee":13793,"source_line":48,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":475,"record_offset":34080,"parent_record_offset":34052,"ancestor_record_offset":32724,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":475,"record_offset":40544,"parent_record_offset":40520,"ancestor_record_offset":32724,"inlinee":13791,"source_line":19,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":475,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":32724,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":475,"record_offset":34108,"parent_record_offset":34080,"ancestor_record_offset":32724,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":475,"record_offset":35432,"parent_record_offset":35408,"ancestor_record_offset":32724,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":475,"record_offset":34024,"parent_record_offset":33996,"ancestor_record_offset":32724,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$2::add::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":475,"record_offset":35592,"parent_record_offset":35504,"ancestor_record_offset":32724,"inlinee":26964,"source_line":68,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$2::add::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":475,"record_offset":35528,"parent_record_offset":35504,"ancestor_record_offset":32724,"inlinee":26963,"source_line":69,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::nani::mesh_atlas::impl$2::add::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/nani/mesh_atlas.rs
//     evidence={"module":475,"record_offset":37180,"parent_record_offset":37148,"ancestor_record_offset":32724,"inlinee":26965,"source_line":88,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":475,"record_offset":35644,"parent_record_offset":35616,"ancestor_record_offset":32724,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":475,"record_offset":35616,"parent_record_offset":35592,"ancestor_record_offset":32724,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":475,"record_offset":33968,"parent_record_offset":33940,"ancestor_record_offset":32724,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":33056,"parent_record_offset":33028,"ancestor_record_offset":32724,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":34268,"parent_record_offset":34232,"ancestor_record_offset":32724,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":33084,"parent_record_offset":33056,"ancestor_record_offset":32724,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":36380,"parent_record_offset":36356,"ancestor_record_offset":32724,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":41568,"parent_record_offset":41544,"ancestor_record_offset":32724,"inlinee":6419,"source_line":801,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":33940,"parent_record_offset":33720,"ancestor_record_offset":32724,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":37740,"parent_record_offset":37716,"ancestor_record_offset":32724,"inlinee":10180,"source_line":196,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":35804,"parent_record_offset":35776,"ancestor_record_offset":32724,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$63::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":41908,"parent_record_offset":41880,"ancestor_record_offset":32724,"inlinee":8353,"source_line":4766,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":33180,"parent_record_offset":33084,"ancestor_record_offset":32724,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33852,"parent_record_offset":33828,"ancestor_record_offset":32724,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33828,"parent_record_offset":33720,"ancestor_record_offset":32724,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33768,"parent_record_offset":33744,"ancestor_record_offset":32724,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":36144,"parent_record_offset":35864,"ancestor_record_offset":32724,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33688,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":36484,"parent_record_offset":36428,"ancestor_record_offset":32724,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":35864,"parent_record_offset":35832,"ancestor_record_offset":32724,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33744,"parent_record_offset":33720,"ancestor_record_offset":32724,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33232,"parent_record_offset":33208,"ancestor_record_offset":32724,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33208,"parent_record_offset":33084,"ancestor_record_offset":32724,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33124,"parent_record_offset":33084,"ancestor_record_offset":32724,"inlinee":17280,"source_line":1571,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":36404,"parent_record_offset":36380,"ancestor_record_offset":32724,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33148,"parent_record_offset":33124,"ancestor_record_offset":32724,"inlinee":17281,"source_line":1565,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":39588,"parent_record_offset":39524,"ancestor_record_offset":32724,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":36456,"parent_record_offset":36428,"ancestor_record_offset":32724,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33256,"parent_record_offset":33232,"ancestor_record_offset":32724,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":36428,"parent_record_offset":36404,"ancestor_record_offset":32724,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":33500,"parent_record_offset":33472,"ancestor_record_offset":32724,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":33532,"parent_record_offset":33500,"ancestor_record_offset":32724,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":42876,"parent_record_offset":42752,"ancestor_record_offset":32724,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":33472,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33304,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":39148,"parent_record_offset":39124,"ancestor_record_offset":32724,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":35832,"parent_record_offset":35804,"ancestor_record_offset":32724,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":33720,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":33332,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":36028,"parent_record_offset":35864,"ancestor_record_offset":32724,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":33416,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":33596,"parent_record_offset":33256,"ancestor_record_offset":32724,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":36084,"parent_record_offset":36028,"ancestor_record_offset":32724,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":33028,"parent_record_offset":33000,"ancestor_record_offset":32724,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":34232,"parent_record_offset":34200,"ancestor_record_offset":32724,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":36356,"parent_record_offset":36332,"ancestor_record_offset":32724,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":41544,"parent_record_offset":41520,"ancestor_record_offset":32724,"inlinee":6418,"source_line":553,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":36332,"parent_record_offset":35724,"ancestor_record_offset":32724,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":35776,"parent_record_offset":35748,"ancestor_record_offset":32724,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$4::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":37716,"parent_record_offset":37656,"ancestor_record_offset":32724,"inlinee":10178,"source_line":1269,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":35748,"parent_record_offset":35724,"ancestor_record_offset":32724,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":475,"record_offset":38356,"parent_record_offset":38328,"ancestor_record_offset":32724,"inlinee":4734,"source_line":605,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":475,"record_offset":38328,"parent_record_offset":38296,"ancestor_record_offset":32724,"inlinee":4732,"source_line":1627,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$9::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":475,"record_offset":38384,"parent_record_offset":38356,"ancestor_record_offset":32724,"inlinee":4736,"source_line":12,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$34::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys_common\wtf8.rs
//     evidence={"module":475,"record_offset":38412,"parent_record_offset":38384,"ancestor_record_offset":32724,"inlinee":4738,"source_line":135,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$2::add::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6954
// 首条位置: {"module":475,"inlinee":26964}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":475,"record_offset":35616,"parent_record_offset":35592,"ancestor_record_offset":35592,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":475,"record_offset":35644,"parent_record_offset":35616,"ancestor_record_offset":35592,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$2::add::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6953
// 首条位置: {"module":475,"inlinee":26963}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":475,"record_offset":35556,"parent_record_offset":35528,"ancestor_record_offset":35528,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::impl$2::add::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6955
// 首条位置: {"module":475,"inlinee":26965}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 记录形式: inlinee_declaration; instances=1
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5B60
// 首条位置: {"module":407,"inlinee":23392}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: mesh_atlas.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlas
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::nani::mesh_atlas::MeshAtlas>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlas"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::MeshAtlasPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "constructor": "tuple$", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::render::mesh_atlas_library::AtlassedMeshName,country_core::render::nani::mesh_atlas::MeshAtlasPart,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::render::mesh_atlas_library::AtlassedMeshName", "country_core::render::nani::mesh_atlas::MeshAtlasPart", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::resources::render::mesh_atlas_library::AtlassedMeshName> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::ssbo::TypedStorageBuffer::new<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo>", "constructor": "country_core::render::ssbo::TypedStorageBuffer::new", "arguments": ["country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh_atlas.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh_atlas.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
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
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::add
// 信息 ID: b8c84e7599ce1a2a6040b534
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> > @ 0x541860 -> 0x669090 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName> @ 0x541f70 -> 0x669090 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlas::clear
// 信息 ID: 44f091f650d2f00813be78eb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::MeshAtlasPart::new
// 信息 ID: fa19107409fb99176a244234
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 94 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure$0::closure_env$1,tuple$<ref_mut$<egui::ui::Ui> > > @ 0x3e91e -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<time::format_description::well_known::rfc3339::Rfc3339> @ 0x397afb -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<time::format_description::well_known::rfc3339::Rfc3339> @ 0x397ea1 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<time::format_description::well_known::rfc3339::Rfc3339> @ 0x4149cb -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] time::formatting::formattable::sealed::Sealed::format<time::format_description::well_known::rfc3339::Rfc3339> @ 0x414d71 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_floors::construct_floors @ 0x4e5ab5 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500a2c -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x501115 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cull_oob_decorators::is_decorator_location_valid @ 0x500b3e -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x5ad298 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x6442d8 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0aa78 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a300 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0c921 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0ce51 -> 0x593250 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_dormer::generate_gothic_dormers @ 0xd0cfc5 -> 0x593250 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$2::add::closure$0
// 信息 ID: fd3cdc01337686a2bd502b39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$2::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=475, inline_record=35592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$2::add::closure$1
// 信息 ID: 1e1fb43df896213f434d3ec7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$2::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=475, inline_record=35528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$2::add::closure$2
// 信息 ID: 86b876948ecd9dab6b14b603
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::render::nani::mesh_atlas::impl$2::add", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=475, inline_record=37180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$3::default
// 信息 ID: 3857b20eabda50229e455b3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >; module=398, inline_record=40540; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>; module=398, inline_record=46132; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas>,hashbrown::map::impl$94::extend::closure_env$0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::resources::render::mesh_atlas_library::WorldSubsetMeshAtlas,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::resources::render::mesh_atlas_library::impl$6::default::closure_env$0> > > >; module=1321, inline_record=62748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::nani::mesh_atlas::impl$8::fmt
// 信息 ID: dd874601c43c469f06a99497
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<country_core::render::nani::mesh_atlas::MeshAtlasPart>; module=410, inline_record=416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
