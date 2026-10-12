// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/rdp.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ace6fcc97ebe2216502f566c83847989bc698b73df6c7596153cdea38606adcd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rdp.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 389, 484, 1226, 1235, 1264, 1308, 1321, 2708
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
// 函数符号: utils::rdp::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x562A
// 首条位置: {"module":389,"inlinee":22058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":39324,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":40400,"parent_record_offset":39272,"ancestor_record_offset":39272,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":39692,"parent_record_offset":39668,"ancestor_record_offset":39272,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":39716,"parent_record_offset":39692,"ancestor_record_offset":39272,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":40564,"parent_record_offset":40540,"ancestor_record_offset":39272,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":39668,"parent_record_offset":39568,"ancestor_record_offset":39272,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":389,"record_offset":40540,"parent_record_offset":40448,"ancestor_record_offset":39272,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":40476,"parent_record_offset":40448,"ancestor_record_offset":39272,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":39568,"parent_record_offset":39528,"ancestor_record_offset":39272,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":39528,"parent_record_offset":39488,"ancestor_record_offset":39272,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":39788,"parent_record_offset":39528,"ancestor_record_offset":39272,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":389,"record_offset":40448,"parent_record_offset":40424,"ancestor_record_offset":39272,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":39444,"parent_record_offset":39404,"ancestor_record_offset":39272,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":39404,"parent_record_offset":39364,"ancestor_record_offset":39272,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":389,"record_offset":39364,"parent_record_offset":39324,"ancestor_record_offset":39272,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":389,"record_offset":39488,"parent_record_offset":39444,"ancestor_record_offset":39272,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":39612,"parent_record_offset":39568,"ancestor_record_offset":39272,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":389,"record_offset":39636,"parent_record_offset":39612,"ancestor_record_offset":39272,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":389,"record_offset":39852,"parent_record_offset":39828,"ancestor_record_offset":39272,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":389,"record_offset":40504,"parent_record_offset":40476,"ancestor_record_offset":39272,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":389,"record_offset":39828,"parent_record_offset":39444,"ancestor_record_offset":39272,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":40424,"parent_record_offset":40400,"ancestor_record_offset":39272,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":389,"record_offset":39744,"parent_record_offset":39716,"ancestor_record_offset":39272,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00389.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":1760,"rva":6864320,"va":5375573440}
// 编译实例 RVAs: 0x68bdc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2823874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e83ee9d4ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375573565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2824524","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8b4e6d4ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375574231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t374219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8cbb50500","size":5,"soft_fail":false,"target_va":5375948944,"va":5375574720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28019760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8308cab01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375573595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28019711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8ff8bab01","size":5,"soft_fail":false,"target_va":5403593376,"va":5375573660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27403700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8b425a201","size":5,"soft_fail":false,"target_va":5402978144,"va":5375574439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27479808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8004fa301","size":5,"soft_fail":false,"target_va":5403053552,"va":5375573739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1952691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8b3cb1d00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375574424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27403970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8c226a201","size":5,"soft_fail":false,"target_va":5402978432,"va":5375574457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27405882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e83a2ea201","size":5,"soft_fail":false,"target_va":5402980304,"va":5375574417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_overflow_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27405995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375573440,"raw_hex":"e8ab2ea201","size":5,"soft_fail":false,"target_va":5402980400,"va":5375574400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-2824660","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375573440,"raw_hex":"e92ce6d4ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375574367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":2092,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":2612,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":1916,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":3868,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":3012,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":14886,"source_line":3157,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":4780,"parent_record_offset":1760,"ancestor_record_offset":1760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":1760,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1760,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":1760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":2384,"parent_record_offset":2296,"ancestor_record_offset":1760,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":4920,"parent_record_offset":4828,"ancestor_record_offset":1760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":4856,"parent_record_offset":4828,"ancestor_record_offset":1760,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":1760,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":2708,"parent_record_offset":2684,"ancestor_record_offset":1760,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":1760,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":2296,"parent_record_offset":2264,"ancestor_record_offset":1760,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":2264,"parent_record_offset":2232,"ancestor_record_offset":1760,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":2500,"parent_record_offset":2264,"ancestor_record_offset":1760,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":4828,"parent_record_offset":4804,"ancestor_record_offset":1760,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":2188,"parent_record_offset":2156,"ancestor_record_offset":1760,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":2156,"parent_record_offset":2124,"ancestor_record_offset":1760,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":2124,"parent_record_offset":2092,"ancestor_record_offset":1760,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1760,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":2684,"parent_record_offset":2612,"ancestor_record_offset":1760,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":1760,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":2656,"parent_record_offset":2612,"ancestor_record_offset":1760,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":3096,"parent_record_offset":3060,"ancestor_record_offset":1760,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":2780,"parent_record_offset":2612,"ancestor_record_offset":1760,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":2232,"parent_record_offset":2188,"ancestor_record_offset":1760,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":4444,"parent_record_offset":4400,"ancestor_record_offset":1760,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":484,"record_offset":4544,"parent_record_offset":4444,"ancestor_record_offset":1760,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":484,"record_offset":4092,"parent_record_offset":3916,"ancestor_record_offset":1760,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":484,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":1760,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":484,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":1760,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":484,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":1760,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":1760,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1760,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":484,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1760,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":4144,"parent_record_offset":4120,"ancestor_record_offset":1760,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":4224,"parent_record_offset":4168,"ancestor_record_offset":1760,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":4120,"parent_record_offset":3916,"ancestor_record_offset":1760,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":4400,"parent_record_offset":4224,"ancestor_record_offset":1760,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":4584,"parent_record_offset":4168,"ancestor_record_offset":1760,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":4884,"parent_record_offset":4856,"ancestor_record_offset":1760,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":4344,"parent_record_offset":4300,"ancestor_record_offset":1760,"inlinee":27294,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$14::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":484,"record_offset":4300,"parent_record_offset":4256,"ancestor_record_offset":1760,"inlinee":27292,"source_line":35,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":484,"record_offset":2016,"parent_record_offset":1980,"ancestor_record_offset":1760,"inlinee":27288,"source_line":454,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":2540,"parent_record_offset":2188,"ancestor_record_offset":1760,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":4616,"parent_record_offset":4168,"ancestor_record_offset":1760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":2740,"parent_record_offset":2612,"ancestor_record_offset":1760,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":1760,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":4496,"parent_record_offset":4444,"ancestor_record_offset":1760,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":1948,"parent_record_offset":1916,"ancestor_record_offset":1760,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":2044,"parent_record_offset":1980,"ancestor_record_offset":1760,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$8::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":1980,"parent_record_offset":1948,"ancestor_record_offset":1760,"inlinee":20602,"source_line":583,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":4168,"parent_record_offset":4144,"ancestor_record_offset":1760,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::stitch_with::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":484,"record_offset":4256,"parent_record_offset":4224,"ancestor_record_offset":1760,"inlinee":27290,"source_line":44,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::stitch_with::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A9A
// 首条位置: {"module":484,"inlinee":27290}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$14::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\internal_macros.rs
//     evidence={"module":484,"record_offset":4300,"parent_record_offset":4256,"ancestor_record_offset":4256,"inlinee":27292,"source_line":35,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":4344,"parent_record_offset":4300,"ancestor_record_offset":4256,"inlinee":27294,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::into_indices
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE965
// 首条位置: {"module":1235,"inlinee":59749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1235,"record_offset":7836,"parent_record_offset":7736,"ancestor_record_offset":7656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7768,"parent_record_offset":7736,"ancestor_record_offset":7656,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1235,"record_offset":7736,"parent_record_offset":7708,"ancestor_record_offset":7656,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1235,"record_offset":7800,"parent_record_offset":7768,"ancestor_record_offset":7656,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1235,"record_offset":7708,"parent_record_offset":7684,"ancestor_record_offset":7656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01235.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":484,"record_offset":6080,"rva":6865632,"va":5375574752}
// 编译实例 RVAs: 0x68c2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2825175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e829e4d4ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375574866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2826342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e89adfd4ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375576049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d79100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7260491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e84bc96e00","size":5,"soft_fail":false,"target_va":5382836480,"va":5375575984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28017466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e83a83ab01","size":5,"soft_fail":false,"target_va":5403593360,"va":5375575889,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27402284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e82c20a201","size":5,"soft_fail":false,"target_va":5402978144,"va":5375575855,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-4270018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e83ed8beff","size":5,"soft_fail":false,"target_va":5371305056,"va":5375575069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1950876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e89cc41d00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375576239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27402951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e8c722a201","size":5,"soft_fail":false,"target_va":5402979152,"va":5375576196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27403189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e8b523a201","size":5,"soft_fail":false,"target_va":5402979344,"va":5375576150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27404056","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375574752,"raw_hex":"e81827a201","size":5,"soft_fail":false,"target_va":5402980320,"va":5375576259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::rdp::Rdp::new_with_opts
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":484,"record_offset":6208,"parent_record_offset":6080,"ancestor_record_offset":6080,"inlinee":27296,"source_line":81,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":6080,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":6080,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":6080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":6080,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":10736,"parent_record_offset":10636,"ancestor_record_offset":6080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":10668,"parent_record_offset":10636,"ancestor_record_offset":6080,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":6080,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":6080,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":6080,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6880,"parent_record_offset":6704,"ancestor_record_offset":6080,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":10636,"parent_record_offset":10608,"ancestor_record_offset":6080,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7356,"parent_record_offset":7320,"ancestor_record_offset":6080,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7320,"parent_record_offset":7284,"ancestor_record_offset":6080,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7248,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7284,"parent_record_offset":7248,"ancestor_record_offset":6080,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7796,"parent_record_offset":7764,"ancestor_record_offset":6080,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":8360,"parent_record_offset":8324,"ancestor_record_offset":6080,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":6080,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7708,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":8324,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7764,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6648,"parent_record_offset":6472,"ancestor_record_offset":6080,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":6080,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":9844,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6416,"parent_record_offset":6388,"ancestor_record_offset":6080,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":6080,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":484,"record_offset":6920,"parent_record_offset":6472,"ancestor_record_offset":6080,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":484,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":6080,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":484,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":6080,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":10124,"parent_record_offset":10080,"ancestor_record_offset":6080,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":6080,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":6080,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9340,"parent_record_offset":8768,"ancestor_record_offset":6080,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":484,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":6080,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6080,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":7068,"parent_record_offset":7040,"ancestor_record_offset":6080,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":8532,"parent_record_offset":8496,"ancestor_record_offset":6080,"inlinee":27298,"source_line":60,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6080,"inlinee":27297,"source_line":54,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6080,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":6508,"parent_record_offset":6472,"ancestor_record_offset":6080,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":6080,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":484,"record_offset":8496,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":11439,"source_line":122,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":484,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":6080,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":6388,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6080,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":6080,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":10700,"parent_record_offset":10668,"ancestor_record_offset":6080,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9088,"parent_record_offset":9060,"ancestor_record_offset":6080,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9116,"parent_record_offset":9060,"ancestor_record_offset":6080,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":6080,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":484,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":6080,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":7636,"parent_record_offset":7356,"ancestor_record_offset":6080,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":6080,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":10584,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":6080,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":484,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6080,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":6080,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":6080,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":10416,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":8428,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":8456,"parent_record_offset":8428,"ancestor_record_offset":6080,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":6080,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6080,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6080,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":6080,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":6080,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":6080,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8020,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":6080,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":484,"record_offset":8156,"parent_record_offset":8124,"ancestor_record_offset":6080,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8812,"parent_record_offset":8768,"ancestor_record_offset":6080,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8952,"parent_record_offset":8868,"ancestor_record_offset":6080,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8076,"parent_record_offset":6208,"ancestor_record_offset":6080,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":9012,"parent_record_offset":8840,"ancestor_record_offset":6080,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":6080,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":484,"record_offset":8768,"parent_record_offset":8728,"ancestor_record_offset":6080,"inlinee":27300,"source_line":127,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":6080,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AA0
// 首条位置: {"module":484,"inlinee":27296}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7248,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7708,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":8324,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7764,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":9844,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":484,"record_offset":8496,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":11439,"source_line":122,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":6388,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":10584,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":484,"record_offset":10416,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":8428,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8020,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8076,"parent_record_offset":6208,"ancestor_record_offset":6208,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":6208,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":6208,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":10760,"parent_record_offset":10736,"ancestor_record_offset":6208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":6764,"parent_record_offset":6732,"ancestor_record_offset":6208,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":484,"record_offset":10736,"parent_record_offset":10636,"ancestor_record_offset":6208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":10668,"parent_record_offset":10636,"ancestor_record_offset":6208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":6208,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":6208,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":6208,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":6880,"parent_record_offset":6704,"ancestor_record_offset":6208,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":484,"record_offset":10636,"parent_record_offset":10608,"ancestor_record_offset":6208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7356,"parent_record_offset":7320,"ancestor_record_offset":6208,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7320,"parent_record_offset":7284,"ancestor_record_offset":6208,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":484,"record_offset":7284,"parent_record_offset":7248,"ancestor_record_offset":6208,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7796,"parent_record_offset":7764,"ancestor_record_offset":6208,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":8360,"parent_record_offset":8324,"ancestor_record_offset":6208,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":6208,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6648,"parent_record_offset":6472,"ancestor_record_offset":6208,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6676,"parent_record_offset":6648,"ancestor_record_offset":6208,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":6416,"parent_record_offset":6388,"ancestor_record_offset":6208,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":484,"record_offset":7140,"parent_record_offset":7116,"ancestor_record_offset":6208,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":484,"record_offset":6920,"parent_record_offset":6472,"ancestor_record_offset":6208,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":484,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":6208,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":484,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":6208,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":10124,"parent_record_offset":10080,"ancestor_record_offset":6208,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":484,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":6208,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":6208,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9340,"parent_record_offset":8768,"ancestor_record_offset":6208,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":484,"record_offset":7660,"parent_record_offset":7636,"ancestor_record_offset":6208,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":6208,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":7068,"parent_record_offset":7040,"ancestor_record_offset":6208,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":8532,"parent_record_offset":8496,"ancestor_record_offset":6208,"inlinee":27298,"source_line":60,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":484,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6208,"inlinee":27297,"source_line":54,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":6208,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":6508,"parent_record_offset":6472,"ancestor_record_offset":6208,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":484,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":6208,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":484,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":6208,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":6208,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":484,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":6208,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":484,"record_offset":10700,"parent_record_offset":10668,"ancestor_record_offset":6208,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9088,"parent_record_offset":9060,"ancestor_record_offset":6208,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9116,"parent_record_offset":9060,"ancestor_record_offset":6208,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":6208,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":484,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":6208,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":7636,"parent_record_offset":7356,"ancestor_record_offset":6208,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":484,"record_offset":6604,"parent_record_offset":6580,"ancestor_record_offset":6208,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":6208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":484,"record_offset":10464,"parent_record_offset":10440,"ancestor_record_offset":6208,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":484,"record_offset":6580,"parent_record_offset":6556,"ancestor_record_offset":6208,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":6208,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":484,"record_offset":7164,"parent_record_offset":7140,"ancestor_record_offset":6208,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":484,"record_offset":8456,"parent_record_offset":8428,"ancestor_record_offset":6208,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":484,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":6208,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":6208,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":484,"record_offset":6556,"parent_record_offset":6532,"ancestor_record_offset":6208,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8124,"parent_record_offset":8100,"ancestor_record_offset":6208,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":6208,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":6208,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8896,"parent_record_offset":8868,"ancestor_record_offset":6208,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":6208,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":484,"record_offset":8156,"parent_record_offset":8124,"ancestor_record_offset":6208,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8812,"parent_record_offset":8768,"ancestor_record_offset":6208,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8952,"parent_record_offset":8868,"ancestor_record_offset":6208,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":9216,"parent_record_offset":8812,"ancestor_record_offset":6208,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":9012,"parent_record_offset":8840,"ancestor_record_offset":6208,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":6208,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":484,"record_offset":8768,"parent_record_offset":8728,"ancestor_record_offset":6208,"inlinee":27300,"source_line":127,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":6208,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2708,"record_offset":260,"rva":24603776,"va":5393312896}
// 编译实例 RVAs: 0x1776c80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20563360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e8603ac6fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5393313051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20565129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e87733c6fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5393314836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cac270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11317608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e8984e53ff","size":5,"soft_fail":false,"target_va":5381997168,"va":5393314771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10278679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e817d79c00","size":5,"soft_fail":false,"target_va":5403593360,"va":5393314676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9663500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e80c749300","size":5,"soft_fail":false,"target_va":5402978144,"va":5393314639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-22008216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e8682eb0fe","size":5,"soft_fail":false,"target_va":5371305056,"va":5393313267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15787952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e850180fff","size":5,"soft_fail":false,"target_va":5377527120,"va":5393315067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9664123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e87b769300","size":5,"soft_fail":false,"target_va":5402979152,"va":5393315024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9664361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e869779300","size":5,"soft_fail":false,"target_va":5402979344,"va":5393314978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9665228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393312896,"raw_hex":"e8cc7a9300","size":5,"soft_fail":false,"target_va":5402980320,"va":5393315087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2708,"record_offset":1232,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":1752,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":2440,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":1808,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":5192,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\skip.rs
//     evidence={"module":2708,"record_offset":2616,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":11439,"source_line":122,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2708,"record_offset":448,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2708,"record_offset":5932,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2708,"record_offset":5764,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2708,"record_offset":2064,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2708,"record_offset":2544,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":2092,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":2180,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2708,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":260,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2708,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2708,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":260,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2708,"record_offset":744,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2708,"record_offset":6084,"parent_record_offset":5984,"ancestor_record_offset":260,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":6016,"parent_record_offset":5984,"ancestor_record_offset":260,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":260,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":652,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":260,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":864,"parent_record_offset":624,"ancestor_record_offset":260,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2708,"record_offset":5984,"parent_record_offset":5956,"ancestor_record_offset":260,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2708,"record_offset":1340,"parent_record_offset":1304,"ancestor_record_offset":260,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2708,"record_offset":1304,"parent_record_offset":1268,"ancestor_record_offset":260,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":2708,"record_offset":1268,"parent_record_offset":1232,"ancestor_record_offset":260,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":1840,"parent_record_offset":1808,"ancestor_record_offset":260,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":2476,"parent_record_offset":2440,"ancestor_record_offset":260,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":928,"parent_record_offset":904,"ancestor_record_offset":260,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":568,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":596,"parent_record_offset":568,"ancestor_record_offset":260,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":476,"parent_record_offset":448,"ancestor_record_offset":260,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2708,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":260,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2708,"record_offset":904,"parent_record_offset":532,"ancestor_record_offset":260,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":2708,"record_offset":504,"parent_record_offset":476,"ancestor_record_offset":260,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":2708,"record_offset":532,"parent_record_offset":504,"ancestor_record_offset":260,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2708,"record_offset":688,"parent_record_offset":652,"ancestor_record_offset":260,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":2708,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":260,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2708,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":260,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":2708,"record_offset":3716,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":2708,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":260,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2708,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":260,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2708,"record_offset":1052,"parent_record_offset":1024,"ancestor_record_offset":260,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2708,"record_offset":2656,"parent_record_offset":2616,"ancestor_record_offset":260,"inlinee":27298,"source_line":60,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2708,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":260,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":2708,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":260,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2708,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":260,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2708,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":260,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2708,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":260,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2708,"record_offset":6048,"parent_record_offset":6016,"ancestor_record_offset":260,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2708,"record_offset":3360,"parent_record_offset":3320,"ancestor_record_offset":260,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2708,"record_offset":3388,"parent_record_offset":3320,"ancestor_record_offset":260,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2708,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":260,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2708,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":260,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2708,"record_offset":1680,"parent_record_offset":1340,"ancestor_record_offset":260,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2708,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":260,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2708,"record_offset":5956,"parent_record_offset":5932,"ancestor_record_offset":260,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2708,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":260,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2708,"record_offset":2824,"parent_record_offset":2752,"ancestor_record_offset":260,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2708,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":260,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2708,"record_offset":1148,"parent_record_offset":1124,"ancestor_record_offset":260,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":2708,"record_offset":2576,"parent_record_offset":2544,"ancestor_record_offset":260,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":2708,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":260,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2708,"record_offset":1024,"parent_record_offset":1000,"ancestor_record_offset":260,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":2236,"parent_record_offset":2208,"ancestor_record_offset":260,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":260,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":260,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":260,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":260,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":260,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2708,"record_offset":2272,"parent_record_offset":2236,"ancestor_record_offset":260,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3008,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":260,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3580,"parent_record_offset":3008,"ancestor_record_offset":260,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":4488,"parent_record_offset":2932,"ancestor_record_offset":260,"inlinee":89614,"source_line":91,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3264,"parent_record_offset":3040,"ancestor_record_offset":260,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3040,"parent_record_offset":3008,"ancestor_record_offset":260,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":2708,"record_offset":2932,"parent_record_offset":2880,"ancestor_record_offset":260,"inlinee":27300,"source_line":127,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3072,"parent_record_offset":3040,"ancestor_record_offset":260,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6AA4
// 首条位置: {"module":484,"inlinee":27300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9340,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":13755,"source_line":560,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":4572,"parent_record_offset":2932,"ancestor_record_offset":2932,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8812,"parent_record_offset":8768,"ancestor_record_offset":8768,"inlinee":23095,"source_line":161,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":4488,"parent_record_offset":2932,"ancestor_record_offset":2932,"inlinee":89614,"source_line":91,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs_private
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":484,"record_offset":9364,"parent_record_offset":9340,"ancestor_record_offset":8768,"inlinee":13756,"source_line":511,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9088,"parent_record_offset":9060,"ancestor_record_offset":8768,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":9116,"parent_record_offset":9060,"ancestor_record_offset":8768,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":484,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":8768,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8976,"parent_record_offset":8952,"ancestor_record_offset":8768,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9240,"parent_record_offset":9216,"ancestor_record_offset":8768,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9036,"parent_record_offset":9012,"ancestor_record_offset":8768,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":8896,"parent_record_offset":8868,"ancestor_record_offset":8768,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":484,"record_offset":9060,"parent_record_offset":9036,"ancestor_record_offset":8768,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":2932,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":2932,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":2932,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":2932,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":2932,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":2932,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":484,"record_offset":9296,"parent_record_offset":9264,"ancestor_record_offset":8768,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8952,"parent_record_offset":8868,"ancestor_record_offset":8768,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":9216,"parent_record_offset":8812,"ancestor_record_offset":8768,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":9012,"parent_record_offset":8840,"ancestor_record_offset":8768,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":2932,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3580,"parent_record_offset":3008,"ancestor_record_offset":2932,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3264,"parent_record_offset":3040,"ancestor_record_offset":2932,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":8768,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":484,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":8768,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00484.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8E7
// 首条位置: {"module":1226,"inlinee":59623}
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
// 完整结构及原始类型记录: rdp.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::rdp::Rdp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp::stitch_with", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp::new", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::rdp::Rdp<glam::f32::vec2::Vec2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::rdp::Rdp<glam::f32::vec2::Vec2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>", "constructor": "utils::rdp::Rdp::new_with_opts", "arguments": ["glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rdp.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rdp.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 12; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::into_indices
// 信息 ID: 1abcddcaa8b5662d7bffdae5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1235, inline_record=7656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 信息 ID: 9afd2200cec8caf9e5053b90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::new @ 0xe8f3f6 -> 0x68c2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xe9027a -> 0x68c2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xe900ec -> 0x68c2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xe90436 -> 0x68c2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xe90178 -> 0x68c2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts
// 信息 ID: cbee9ee909c19cf0b11a9300
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=6208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 信息 ID: 58ad49febadcc5065975e7fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new_with_opts -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0xcbd7a3 -> 0x1776c80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>
// 信息 ID: 6c2ca14bb5dbe214ff45fa63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::stitch_with -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xe9049b -> 0x68bdc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$1
// 信息 ID: 5b5d9a876db91debc0d68499
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=8768; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=2932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$2
// 信息 ID: 038aee10531b98908c6243d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// [闭包环境类型引用] utils::rdp::impl$0::new_with_opts::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] utils::rdp::impl$0::new_with_opts::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec2::Vec2> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; module=1226, inline_record=45964; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec2::Vec2> > >; module=1308, inline_record=176768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::stitch_with::closure$0
// 信息 ID: ad47fb006d8f45cc80ba3544
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::rdp::impl$0::stitch_with", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::stitch_with<glam::f32::vec2::Vec2>; module=484, inline_record=4256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$1::clone
// 信息 ID: 90eda6927fbb8e9e9dbd99b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=389, inline_record=39272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
