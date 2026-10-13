// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/in_progress_curve.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 98201f992e95d4c6bcc1073bdaa2b5c110425e1b3ce2f87dbf7bfb7e46e9198a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: in_progress_curve.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 389, 453, 1328, 1447
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5624
// 首条位置: {"module":389,"inlinee":22052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":38696,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":40612,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":389,"record_offset":37304,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] utils::rdp::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":389,"record_offset":39272,"parent_record_offset":37208,"ancestor_record_offset":37208,"inlinee":22058,"source_line":6,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":37732,"parent_record_offset":37708,"ancestor_record_offset":37208,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":37756,"parent_record_offset":37732,"ancestor_record_offset":37208,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":38648,"parent_record_offset":38624,"ancestor_record_offset":37208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":37708,"parent_record_offset":37608,"ancestor_record_offset":37208,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":38624,"parent_record_offset":38532,"ancestor_record_offset":37208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":38560,"parent_record_offset":38532,"ancestor_record_offset":37208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":37608,"parent_record_offset":37572,"ancestor_record_offset":37208,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":37572,"parent_record_offset":37536,"ancestor_record_offset":37208,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":37828,"parent_record_offset":37572,"ancestor_record_offset":37208,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":38532,"parent_record_offset":38508,"ancestor_record_offset":37208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":37492,"parent_record_offset":37452,"ancestor_record_offset":37208,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":37452,"parent_record_offset":37412,"ancestor_record_offset":37208,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":37412,"parent_record_offset":37372,"ancestor_record_offset":37208,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":37536,"parent_record_offset":37492,"ancestor_record_offset":37208,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":37372,"parent_record_offset":37304,"ancestor_record_offset":37208,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":37652,"parent_record_offset":37608,"ancestor_record_offset":37208,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":37676,"parent_record_offset":37652,"ancestor_record_offset":37208,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":389,"record_offset":37896,"parent_record_offset":37872,"ancestor_record_offset":37208,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":38588,"parent_record_offset":38560,"ancestor_record_offset":37208,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":389,"record_offset":37872,"parent_record_offset":37492,"ancestor_record_offset":37208,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":38484,"parent_record_offset":37304,"ancestor_record_offset":37208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":37784,"parent_record_offset":37756,"ancestor_record_offset":37208,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1328,"record_offset":3364,"rva":15265760,"va":5383974880}
// 编译实例 RVAs: 0xe8efe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11225524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e84cb654ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5383975215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11226345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e817b354ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383976052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068c2e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8401179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8e5ce7fff","size":5,"soft_fail":false,"target_va":5375574752,"va":5383975926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=3
//     target_aliases=14 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8096381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8837584ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5383977816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e891e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8329cffff","size":5,"soft_fail":false,"target_va":5383950816,"va":5383976361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8a230
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e83eb1ffff","size":5,"soft_fail":false,"target_va":5383954992,"va":5383975149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e85adfffff","size":5,"soft_fail":false,"target_va":5383966736,"va":5383975089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19617891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e863582b01","size":5,"soft_fail":false,"target_va":5403593360,"va":5383975464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t19002709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e855f52101","size":5,"soft_fail":false,"target_va":5402978144,"va":5383975430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18943636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8940e2101","size":5,"soft_fail":false,"target_va":5402919776,"va":5383976135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12672273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8efa23eff","size":5,"soft_fail":false,"target_va":5371305056,"va":5383977324,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6449960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8d8949dff","size":5,"soft_fail":false,"target_va":5377527120,"va":5383977075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19002037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8b5f22101","size":5,"soft_fail":false,"target_va":5402978880,"va":5383976838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19002176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e840f32101","size":5,"soft_fail":false,"target_va":5402979152,"va":5383976971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19002518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e896f42101","size":5,"soft_fail":false,"target_va":5402979344,"va":5383976821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19001997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e88df22101","size":5,"soft_fail":false,"target_va":5402978944,"va":5383976942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19003058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e8b2f62101","size":5,"soft_fail":false,"target_va":5402980128,"va":5383977065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19003281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383974880,"raw_hex":"e891f72101","size":5,"soft_fail":false,"target_va":5402980288,"va":5383977002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":10028,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":4164,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":3664,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7104,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7648,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1328,"record_offset":3924,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1328,"record_offset":8828,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1328,"record_offset":3628,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":6788,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1328,"record_offset":3600,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1328,"record_offset":3772,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":3572,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":8052,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":4788,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1328,"record_offset":3952,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":4816,"parent_record_offset":3364,"ancestor_record_offset":3364,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":4404,"parent_record_offset":4380,"ancestor_record_offset":3364,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":4428,"parent_record_offset":4404,"ancestor_record_offset":3364,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":3364,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":4380,"parent_record_offset":4288,"ancestor_record_offset":3364,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":6096,"parent_record_offset":6004,"ancestor_record_offset":3364,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":3364,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":3364,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":7236,"parent_record_offset":7208,"ancestor_record_offset":3364,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":3364,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":3364,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":4288,"parent_record_offset":4260,"ancestor_record_offset":3364,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":4260,"parent_record_offset":4232,"ancestor_record_offset":3364,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":4500,"parent_record_offset":4260,"ancestor_record_offset":3364,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":6004,"parent_record_offset":5980,"ancestor_record_offset":3364,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":4992,"parent_record_offset":4952,"ancestor_record_offset":3364,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":4952,"parent_record_offset":4912,"ancestor_record_offset":3364,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":4912,"parent_record_offset":4872,"ancestor_record_offset":3364,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7704,"parent_record_offset":7672,"ancestor_record_offset":3364,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7272,"parent_record_offset":7152,"ancestor_record_offset":3364,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7152,"parent_record_offset":7128,"ancestor_record_offset":3364,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":4540,"parent_record_offset":4196,"ancestor_record_offset":3364,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7864,"parent_record_offset":7704,"ancestor_record_offset":3364,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":3364,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":8984,"parent_record_offset":8936,"ancestor_record_offset":3364,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":4232,"parent_record_offset":4196,"ancestor_record_offset":3364,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":4872,"parent_record_offset":4816,"ancestor_record_offset":3364,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":8864,"parent_record_offset":8828,"ancestor_record_offset":3364,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":7464,"parent_record_offset":7432,"ancestor_record_offset":3364,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1328,"record_offset":7528,"parent_record_offset":7464,"ancestor_record_offset":3364,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1328,"record_offset":7328,"parent_record_offset":7152,"ancestor_record_offset":3364,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1328,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":3364,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1328,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":3364,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1328,"record_offset":4196,"parent_record_offset":4164,"ancestor_record_offset":3364,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1328,"record_offset":8900,"parent_record_offset":8864,"ancestor_record_offset":3364,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1328,"record_offset":8936,"parent_record_offset":8900,"ancestor_record_offset":3364,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":4324,"parent_record_offset":4288,"ancestor_record_offset":3364,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":4348,"parent_record_offset":4324,"ancestor_record_offset":3364,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1328,"record_offset":4040,"parent_record_offset":4016,"ancestor_record_offset":3364,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1328,"record_offset":4064,"parent_record_offset":4040,"ancestor_record_offset":3364,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1328,"record_offset":4648,"parent_record_offset":4616,"ancestor_record_offset":3364,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1328,"record_offset":4104,"parent_record_offset":3952,"ancestor_record_offset":3364,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1328,"record_offset":4132,"parent_record_offset":3952,"ancestor_record_offset":3364,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1328,"record_offset":5408,"parent_record_offset":5384,"ancestor_record_offset":3364,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1328,"record_offset":7356,"parent_record_offset":7152,"ancestor_record_offset":3364,"inlinee":8513,"source_line":114,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$9::spec_for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1328,"record_offset":7380,"parent_record_offset":7356,"ancestor_record_offset":3364,"inlinee":8515,"source_line":275,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$9::spec_for_each::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":1328,"record_offset":7432,"parent_record_offset":7404,"ancestor_record_offset":3364,"inlinee":8519,"source_line":282,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1328,"record_offset":9544,"parent_record_offset":9500,"ancestor_record_offset":3364,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1328,"record_offset":4616,"parent_record_offset":4584,"ancestor_record_offset":3364,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1328,"record_offset":4584,"parent_record_offset":4540,"ancestor_record_offset":3364,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1328,"record_offset":9420,"parent_record_offset":9388,"ancestor_record_offset":3364,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1328,"record_offset":9388,"parent_record_offset":9356,"ancestor_record_offset":3364,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1328,"record_offset":9620,"parent_record_offset":9420,"ancestor_record_offset":3364,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1328,"record_offset":7404,"parent_record_offset":7380,"ancestor_record_offset":3364,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":7560,"parent_record_offset":7432,"ancestor_record_offset":3364,"inlinee":4983,"source_line":604,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":3364,"inlinee":4984,"source_line":2074,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":9572,"parent_record_offset":9544,"ancestor_record_offset":3364,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":6060,"parent_record_offset":6032,"ancestor_record_offset":3364,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1328,"record_offset":3824,"parent_record_offset":3796,"ancestor_record_offset":3364,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1328,"record_offset":4016,"parent_record_offset":3992,"ancestor_record_offset":3364,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1328,"record_offset":3992,"parent_record_offset":3952,"ancestor_record_offset":3364,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1328,"record_offset":5384,"parent_record_offset":4992,"ancestor_record_offset":3364,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1328,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":3364,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":5956,"parent_record_offset":4816,"ancestor_record_offset":3364,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1328,"record_offset":4720,"parent_record_offset":4540,"ancestor_record_offset":3364,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1328,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":3364,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":3364,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":4688,"parent_record_offset":4540,"ancestor_record_offset":3364,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1328,"record_offset":8080,"parent_record_offset":8052,"ancestor_record_offset":3364,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":3696,"parent_record_offset":3664,"ancestor_record_offset":3364,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":3364,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":7060,"parent_record_offset":7032,"ancestor_record_offset":3364,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1328,"record_offset":7932,"parent_record_offset":7672,"ancestor_record_offset":3364,"inlinee":14888,"source_line":128,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1328,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":3364,"inlinee":14889,"source_line":90,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1328,"record_offset":3852,"parent_record_offset":3796,"ancestor_record_offset":3364,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1328,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3364,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1328,"record_offset":3884,"parent_record_offset":3852,"ancestor_record_offset":3364,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5627
// 首条位置: {"module":389,"inlinee":22055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":389,"record_offset":40988,"parent_record_offset":40936,"ancestor_record_offset":40936,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::end_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":389,"record_offset":40960,"parent_record_offset":40936,"ancestor_record_offset":40936,"inlinee":22056,"source_line":867,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6592
// 首条位置: {"module":453,"inlinee":26002}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":453,"record_offset":26824,"parent_record_offset":26740,"ancestor_record_offset":26740,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::start_pos
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":453,"record_offset":26764,"parent_record_offset":26740,"ancestor_record_offset":26740,"inlinee":26003,"source_line":863,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":453,"record_offset":26792,"parent_record_offset":26764,"ancestor_record_offset":26740,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00453.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::add_raw_input_points
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1328,"record_offset":11888,"rva":15269328,"va":5383978448}
// 编译实例 RVAs: 0xe8fdd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11228838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e85aa954ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5383978529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11229192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e8f8a754ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383978899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e87cd8ffff","size":5,"soft_fail":false,"target_va":5383968368,"va":5383978479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19614612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e8944b2b01","size":5,"soft_fail":false,"target_va":5403593360,"va":5383978743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18999291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e8fbe72101","size":5,"soft_fail":false,"target_va":5402978144,"va":5383978848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18941063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e887042101","size":5,"soft_fail":false,"target_va":5402919776,"va":5383978708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6451720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978448,"raw_hex":"e8f88d9dff","size":5,"soft_fail":false,"target_va":5377527120,"va":5383978835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11229109","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5383978448,"raw_hex":"e94ba854ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383978816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12060,"parent_record_offset":11888,"ancestor_record_offset":11888,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12672,"parent_record_offset":11888,"ancestor_record_offset":11888,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":11888,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":12320,"parent_record_offset":12296,"ancestor_record_offset":11888,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":13220,"parent_record_offset":13196,"ancestor_record_offset":11888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":12272,"parent_record_offset":12180,"ancestor_record_offset":11888,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":13196,"parent_record_offset":13140,"ancestor_record_offset":11888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12880,"parent_record_offset":12852,"ancestor_record_offset":11888,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":13168,"parent_record_offset":13140,"ancestor_record_offset":11888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12852,"parent_record_offset":12816,"ancestor_record_offset":11888,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12816,"parent_record_offset":12788,"ancestor_record_offset":11888,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12180,"parent_record_offset":12152,"ancestor_record_offset":11888,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12152,"parent_record_offset":12124,"ancestor_record_offset":11888,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":12388,"parent_record_offset":12152,"ancestor_record_offset":11888,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":13140,"parent_record_offset":13116,"ancestor_record_offset":11888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12744,"parent_record_offset":12704,"ancestor_record_offset":11888,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12428,"parent_record_offset":12088,"ancestor_record_offset":11888,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12932,"parent_record_offset":12744,"ancestor_record_offset":11888,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12788,"parent_record_offset":12744,"ancestor_record_offset":11888,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":12124,"parent_record_offset":12088,"ancestor_record_offset":11888,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1328,"record_offset":13044,"parent_record_offset":13020,"ancestor_record_offset":11888,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1328,"record_offset":13092,"parent_record_offset":13068,"ancestor_record_offset":11888,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1328,"record_offset":12704,"parent_record_offset":12672,"ancestor_record_offset":11888,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$1::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1328,"record_offset":12088,"parent_record_offset":12060,"ancestor_record_offset":11888,"inlinee":11922,"source_line":23,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":12216,"parent_record_offset":12180,"ancestor_record_offset":11888,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":11888,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1328,"record_offset":12532,"parent_record_offset":12500,"ancestor_record_offset":11888,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1328,"record_offset":12500,"parent_record_offset":12468,"ancestor_record_offset":11888,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1328,"record_offset":12468,"parent_record_offset":12428,"ancestor_record_offset":11888,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":13020,"parent_record_offset":12704,"ancestor_record_offset":11888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1328,"record_offset":12604,"parent_record_offset":12428,"ancestor_record_offset":11888,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":11888,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":12572,"parent_record_offset":12428,"ancestor_record_offset":11888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF22F
// 首条位置: {"module":1447,"inlinee":61999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1447,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":748,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01447.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5631
// 首条位置: {"module":389,"inlinee":22065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":42308,"parent_record_offset":42276,"ancestor_record_offset":42276,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":42336,"parent_record_offset":42276,"ancestor_record_offset":42276,"inlinee":5335,"source_line":896,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":389,"record_offset":42360,"parent_record_offset":42336,"ancestor_record_offset":42276,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::raw_input_pt_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 155
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5626
// 首条位置: {"module":389,"inlinee":22054}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::point_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":389,"record_offset":40848,"parent_record_offset":40824,"ancestor_record_offset":40824,"inlinee":21364,"source_line":871,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1328,"record_offset":13512,"rva":15269808,"va":5383978928}
// 编译实例 RVAs: 0xe8ffb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11229403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e825a754ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5383979094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11229563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e885a654ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5383979270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8406752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e820b97fff","size":5,"soft_fail":false,"target_va":5375573440,"va":5383980187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068c2e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8404497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8efc17fff","size":5,"soft_fail":false,"target_va":5375574752,"va":5383979244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d70e0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8099196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8846a84ff","size":5,"soft_fail":false,"target_va":5375881440,"va":5383980631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e88820
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8e185ffff","size":5,"soft_fail":false,"target_va":5383948320,"va":5383979578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e891e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8008dffff","size":5,"soft_fail":false,"target_va":5383950816,"va":5383980251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: in_progress_curve.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19613940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8f4482b01","size":5,"soft_fail":false,"target_va":5403593360,"va":5383979415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t18999023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8efe62101","size":5,"soft_fail":false,"target_va":5402978144,"va":5383979116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6453259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8f5879dff","size":5,"soft_fail":false,"target_va":5377527120,"va":5383980374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18999754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e8cae92101","size":5,"soft_fail":false,"target_va":5402980128,"va":5383980369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_start_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18999845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383978928,"raw_hex":"e825ea2101","size":5,"soft_fail":false,"target_va":5402980288,"va":5383980438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":16884,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":17464,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1328,"record_offset":13696,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":7276,"source_line":33,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":16432,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":19224,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$42::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1328,"record_offset":13812,"parent_record_offset":13512,"ancestor_record_offset":13512,"inlinee":22042,"source_line":46,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":14180,"parent_record_offset":14156,"ancestor_record_offset":13512,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13512,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":15048,"parent_record_offset":15024,"ancestor_record_offset":13512,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":14156,"parent_record_offset":14056,"ancestor_record_offset":13512,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1328,"record_offset":15024,"parent_record_offset":14932,"ancestor_record_offset":13512,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":14960,"parent_record_offset":14932,"ancestor_record_offset":13512,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":14056,"parent_record_offset":14028,"ancestor_record_offset":13512,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":14028,"parent_record_offset":14000,"ancestor_record_offset":13512,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":14276,"parent_record_offset":14028,"ancestor_record_offset":13512,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1328,"record_offset":14932,"parent_record_offset":14908,"ancestor_record_offset":13512,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":13964,"parent_record_offset":13932,"ancestor_record_offset":13512,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":13932,"parent_record_offset":13900,"ancestor_record_offset":13512,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1328,"record_offset":13900,"parent_record_offset":13868,"ancestor_record_offset":13512,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":14000,"parent_record_offset":13964,"ancestor_record_offset":13512,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1328,"record_offset":13868,"parent_record_offset":13812,"ancestor_record_offset":13512,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":14100,"parent_record_offset":14056,"ancestor_record_offset":13512,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1328,"record_offset":14124,"parent_record_offset":14100,"ancestor_record_offset":13512,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1328,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13512,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1328,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":13512,"inlinee":20600,"source_line":147,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1328,"record_offset":14988,"parent_record_offset":14960,"ancestor_record_offset":13512,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1328,"record_offset":17604,"parent_record_offset":17580,"ancestor_record_offset":13512,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1328,"record_offset":14312,"parent_record_offset":13964,"ancestor_record_offset":13512,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":14884,"parent_record_offset":13812,"ancestor_record_offset":13512,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1328,"record_offset":13772,"parent_record_offset":13744,"ancestor_record_offset":13512,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1328,"record_offset":14232,"parent_record_offset":14204,"ancestor_record_offset":13512,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1328,"record_offset":19252,"parent_record_offset":19224,"ancestor_record_offset":13512,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":13512,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":13512,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":17556,"parent_record_offset":17520,"ancestor_record_offset":13512,"inlinee":5473,"source_line":489,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1328,"record_offset":17520,"parent_record_offset":17492,"ancestor_record_offset":13512,"inlinee":5472,"source_line":501,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1328,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":13512,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01328.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5637
// 首条位置: {"module":389,"inlinee":22071}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::Curve::is_valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":389,"record_offset":47240,"parent_record_offset":47216,"ancestor_record_offset":47216,"inlinee":22072,"source_line":141,"archive":"_pdb/A/modules/00389.jsonl.gz"}
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
// 完整结构及原始类型记录: in_progress_curve.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::CurveSimplificationOptions
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::in_progress_curve::InProgressCurve
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 10; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::in_progress_curve::InProgressCurve>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::in_progress_curve::InProgressCurve"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: in_progress_curve.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: in_progress_curve.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::SavingGameComplete>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::save_game::DeleteSessionCmd>

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::add_raw_input_points
// 信息 ID: 4bc81983c8bfb17f11889534
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x510bcd -> 0xe8fdd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x510d48 -> 0xe8fdd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x510e9c -> 0xe8fdd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::add_weight_to_the_last_point
// 信息 ID: f27fc3fdb18f32e7d638f703
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=42276; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve
// 信息 ID: 804cb51a9b498a826a4fe451
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x51153f -> 0xe8ffb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x5115ae -> 0xe8ffb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::first_raw_input_point
// 信息 ID: 63c8683523b9d7b1851efca4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=453, inline_record=26740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::is_raw_input_curve_valid
// 信息 ID: 193f88968f3fdb0a9cbe7484
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=47216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::last_raw_input_point
// 信息 ID: 0df6f10b5370f2d4f332d336
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=40936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::new
// 信息 ID: fc825ef64e4e384228220ad7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x51074b -> 0xe8efe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::ui_continue_wall::ui_continue_wall @ 0x6b861c -> 0xe8efe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::raw_input_curve_length_ws
// 信息 ID: 63c35f43742f742a389bc7b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<ref$<country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment> > >,country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to::closure_env$0>; module=1447, inline_record=748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::InProgressCurve::raw_input_pt_count
// 信息 ID: 07e25cea0b86c74a5e0d82ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=40824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::in_progress_curve::impl$3::clone
// 信息 ID: 8408b3d1d80b003a2f483e60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=37208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
