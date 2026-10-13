// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/utils/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b1d2d4755198c61ceeea7c036f4ec0c743430b7a40226c61b8d7b34fde26a28a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 72, 371, 373, 374, 379, 383, 391, 392, 401, 423, 434, 438, 452, 453, 454, 461, 463, 472, 476, 477, 484, 493, 495, 496, 497, 615, 734, 740, 741, 753, 754, 1012, 1016, 1206, 1216, 1217, 1219, 1220, 1223, 1226, 1229, 1230, 1231, 1241, 1243, 1246, 1248, 1249, 1250, 1251, 1252, 1257, 1261, 1264, 1272, 1274, 1283, 1284, 1286, 1302, 1308, 1321, 1327, 1390, 1395, 1447, 1461, 1501, 1707, 1893, 1903, 1904, 1924, 2272, 2281, 2414, 2503, 2504, 2708, 2718, 2722, 2733, 2748, 2756, 2761, 2767, 2792, 2802, 3086, 3187, 3192, 3387, 3425, 3456, 3480, 3508, 3515, 3526, 3529, 3537, 3608, 3620, 3631, 3821, 3832, 3842, 3860, 4216, 4241
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use half; // namespace candidate
//   package identities: [{"name":"half","source":"PDB_registry_path_only","version":"2.4.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use system_clutter; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// use toolbar; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::impl$4::assign_auto_resize<enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1308,"record_offset":95316,"rva":14106576,"va":5382815696}
// 编译实例 RVAs: 0xd73fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20237472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382815696,"raw_hex":"e8a0cc3401","size":5,"soft_fail":false,"target_va":5403053552,"va":5382816075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-11508691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382815696,"raw_hex":"e82d6450ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5382815790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20163089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382815696,"raw_hex":"e811aa3301","size":5,"soft_fail":false,"target_va":5402979344,"va":5382816250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95672,"parent_record_offset":95316,"ancestor_record_offset":95316,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95792,"parent_record_offset":95316,"ancestor_record_offset":95316,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95924,"parent_record_offset":95316,"ancestor_record_offset":95316,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95576,"parent_record_offset":95316,"ancestor_record_offset":95316,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":95548,"parent_record_offset":95316,"ancestor_record_offset":95316,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":95876,"parent_record_offset":95852,"ancestor_record_offset":95316,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":95852,"parent_record_offset":95820,"ancestor_record_offset":95316,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":95728,"parent_record_offset":95704,"ancestor_record_offset":95316,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1308,"record_offset":95820,"parent_record_offset":95792,"ancestor_record_offset":95316,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95704,"parent_record_offset":95672,"ancestor_record_offset":95316,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":95964,"parent_record_offset":95924,"ancestor_record_offset":95316,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":96444,"parent_record_offset":95924,"ancestor_record_offset":95316,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":96032,"parent_record_offset":95964,"ancestor_record_offset":95316,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1308,"record_offset":96412,"parent_record_offset":95964,"ancestor_record_offset":95316,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":96276,"parent_record_offset":96240,"ancestor_record_offset":95316,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1308,"record_offset":96240,"parent_record_offset":96204,"ancestor_record_offset":95316,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1308,"record_offset":96204,"parent_record_offset":95964,"ancestor_record_offset":95316,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1308,"record_offset":96116,"parent_record_offset":96092,"ancestor_record_offset":95316,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1308,"record_offset":96160,"parent_record_offset":95964,"ancestor_record_offset":95316,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1308,"record_offset":95760,"parent_record_offset":95672,"ancestor_record_offset":95316,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":95604,"parent_record_offset":95576,"ancestor_record_offset":95316,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":95632,"parent_record_offset":95604,"ancestor_record_offset":95316,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$4::assign_auto_resize<tg_text::AtlasAllocUsage>
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1501,"record_offset":628,"rva":18127984,"va":5386837104}
// 编译实例 RVAs: 0x1149c70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16216004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386837104,"raw_hex":"e8c46ff700","size":5,"soft_fail":false,"target_va":5403053552,"va":5386837543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-15530100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386837104,"raw_hex":"e88c0713ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5386837199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16141620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386837104,"raw_hex":"e8344df600","size":5,"soft_fail":false,"target_va":5402979344,"va":5386837719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":912,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":1032,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":1164,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":816,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$55::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1501,"record_offset":788,"parent_record_offset":628,"ancestor_record_offset":628,"inlinee":7631,"source_line":1565,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1501,"record_offset":1116,"parent_record_offset":1092,"ancestor_record_offset":628,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1501,"record_offset":1092,"parent_record_offset":1060,"ancestor_record_offset":628,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1501,"record_offset":968,"parent_record_offset":944,"ancestor_record_offset":628,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1501,"record_offset":1060,"parent_record_offset":1032,"ancestor_record_offset":628,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":628,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":1204,"parent_record_offset":1164,"ancestor_record_offset":628,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":1684,"parent_record_offset":1164,"ancestor_record_offset":628,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1501,"record_offset":1272,"parent_record_offset":1204,"ancestor_record_offset":628,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1501,"record_offset":1652,"parent_record_offset":1204,"ancestor_record_offset":628,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1501,"record_offset":1516,"parent_record_offset":1480,"ancestor_record_offset":628,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1501,"record_offset":1480,"parent_record_offset":1444,"ancestor_record_offset":628,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1501,"record_offset":1444,"parent_record_offset":1204,"ancestor_record_offset":628,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1501,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":628,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1501,"record_offset":1400,"parent_record_offset":1204,"ancestor_record_offset":628,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1501,"record_offset":1000,"parent_record_offset":912,"ancestor_record_offset":628,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1501,"record_offset":844,"parent_record_offset":816,"ancestor_record_offset":628,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1501,"record_offset":872,"parent_record_offset":844,"ancestor_record_offset":628,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01501.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length
// 记录形式: inline_instance, inlinee_declaration; instances=60
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A39
// 首条位置: {"module":401,"inlinee":23097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":17876,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17924,"parent_record_offset":17900,"ancestor_record_offset":17876,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":17956,"parent_record_offset":17924,"ancestor_record_offset":17876,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 60
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A14F
// 首条位置: {"module":3515,"inlinee":106831}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37336,"parent_record_offset":37312,"ancestor_record_offset":37312,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37360,"parent_record_offset":37336,"ancestor_record_offset":37312,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5777
// 首条位置: {"module":392,"inlinee":22391}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34708,"parent_record_offset":34684,"ancestor_record_offset":34684,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":20536,"parent_record_offset":20408,"ancestor_record_offset":20360,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34788,"parent_record_offset":34756,"ancestor_record_offset":34684,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34756,"parent_record_offset":34732,"ancestor_record_offset":34684,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":34732,"parent_record_offset":34708,"ancestor_record_offset":34684,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1261,"record_offset":3900,"parent_record_offset":3872,"ancestor_record_offset":3844,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/01261.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":392,"record_offset":34820,"parent_record_offset":34788,"ancestor_record_offset":34684,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=99
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x549A
// 首条位置: {"module":379,"inlinee":21658}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=54
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":379,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":2976,"parent_record_offset":2916,"ancestor_record_offset":2868,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":3004,"parent_record_offset":2916,"ancestor_record_offset":2868,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":379,"record_offset":2948,"parent_record_offset":2916,"ancestor_record_offset":2868,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=42
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":379,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2868,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00379.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29852,"parent_record_offset":29828,"ancestor_record_offset":29804,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$0::dot
// 记录形式: inline_instance, inlinee_declaration; instances=50
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5774
// 首条位置: {"module":392,"inlinee":22388}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":29612,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length
// 记录形式: inline_instance, inlinee_declaration; instances=49
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5701
// 首条位置: {"module":391,"inlinee":22273}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":28020,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=21
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":28068,"parent_record_offset":28044,"ancestor_record_offset":28020,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":391,"record_offset":28100,"parent_record_offset":28068,"ancestor_record_offset":28020,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::length_squared
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15E0E
// 首条位置: {"module":2708,"inlinee":89614}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":4488,"inlinee":22227,"source_line":446,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":4536,"parent_record_offset":4512,"ancestor_record_offset":4488,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::normalize
// 记录形式: inline_instance, inlinee_declaration; instances=28
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x544B
// 首条位置: {"module":373,"inlinee":21579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":9496,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":9676,"parent_record_offset":9544,"ancestor_record_offset":9496,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9608,"parent_record_offset":9576,"ancestor_record_offset":9496,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9576,"parent_record_offset":9544,"ancestor_record_offset":9496,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":9496,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":373,"record_offset":9640,"parent_record_offset":9608,"ancestor_record_offset":9496,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=62
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5449
// 首条位置: {"module":373,"inlinee":21577}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=34
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9196,"parent_record_offset":9172,"ancestor_record_offset":9172,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":9344,"parent_record_offset":9220,"ancestor_record_offset":9172,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=55
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":9400,"parent_record_offset":9220,"ancestor_record_offset":9172,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=50
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":373,"record_offset":9260,"parent_record_offset":9220,"ancestor_record_offset":9172,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=29
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":373,"record_offset":9220,"parent_record_offset":9196,"ancestor_record_offset":9172,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00373.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":20448,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$1::dot
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546E
// 首条位置: {"module":374,"inlinee":21614}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20280,"parent_record_offset":20248,"ancestor_record_offset":20248,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::length
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9C3
// 首条位置: {"module":1243,"inlinee":59843}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1249,"record_offset":15316,"parent_record_offset":15288,"ancestor_record_offset":15288,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01249.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$2::lerp
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x68C2
// 首条位置: {"module":472,"inlinee":26818}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::t_of_point_projection_onto_segment
// 记录形式: inline_instance, inlinee_declaration; instances=49
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546D
// 首条位置: {"module":374,"inlinee":21613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":20384,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29500,"parent_record_offset":29468,"ancestor_record_offset":29468,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20060,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=30
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29612,"parent_record_offset":29468,"ancestor_record_offset":29468,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20248,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=44
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":20008,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=30
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":29468,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20280,"parent_record_offset":20248,"ancestor_record_offset":20008,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_line_point
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A37
// 首条位置: {"module":401,"inlinee":23095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17820,"parent_record_offset":17472,"ancestor_record_offset":17472,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3488,"parent_record_offset":3008,"ancestor_record_offset":3008,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17876,"parent_record_offset":17472,"ancestor_record_offset":17472,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3580,"parent_record_offset":3008,"ancestor_record_offset":3008,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17500,"parent_record_offset":17472,"ancestor_record_offset":17472,"inlinee":23096,"source_line":176,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17748,"parent_record_offset":17720,"ancestor_record_offset":17472,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17776,"parent_record_offset":17720,"ancestor_record_offset":17472,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":17472,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":17472,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":17472,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17472,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17556,"parent_record_offset":17528,"ancestor_record_offset":17472,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17472,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":3008,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3608,"parent_record_offset":3580,"ancestor_record_offset":3008,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":3008,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":3008,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":3008,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":401,"record_offset":17956,"parent_record_offset":17924,"ancestor_record_offset":17472,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17612,"parent_record_offset":17528,"ancestor_record_offset":17472,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17672,"parent_record_offset":17500,"ancestor_record_offset":17472,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":3008,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3264,"parent_record_offset":3040,"ancestor_record_offset":3008,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17528,"parent_record_offset":17500,"ancestor_record_offset":17472,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5700
// 首条位置: {"module":391,"inlinee":22272}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23928,"parent_record_offset":23384,"ancestor_record_offset":23384,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":27932,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23984,"parent_record_offset":23384,"ancestor_record_offset":23384,"inlinee":23097,"source_line":55,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":391,"record_offset":28020,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":22273,"source_line":86,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":391,"record_offset":27284,"parent_record_offset":27256,"ancestor_record_offset":27256,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":391,"record_offset":27692,"parent_record_offset":27284,"ancestor_record_offset":27256,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":391,"record_offset":27660,"parent_record_offset":27312,"ancestor_record_offset":27256,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":27804,"parent_record_offset":27772,"ancestor_record_offset":27256,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":27860,"parent_record_offset":27772,"ancestor_record_offset":27256,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":391,"record_offset":27384,"parent_record_offset":27352,"ancestor_record_offset":27256,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23616,"parent_record_offset":23592,"ancestor_record_offset":23384,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":24008,"parent_record_offset":23984,"ancestor_record_offset":23384,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23796,"parent_record_offset":23768,"ancestor_record_offset":23384,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23480,"parent_record_offset":23448,"ancestor_record_offset":23384,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":740,"record_offset":23824,"parent_record_offset":23796,"ancestor_record_offset":23384,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":27560,"parent_record_offset":27532,"ancestor_record_offset":27256,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":28044,"parent_record_offset":28020,"ancestor_record_offset":27256,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":27748,"parent_record_offset":27724,"ancestor_record_offset":27256,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":27352,"parent_record_offset":27312,"ancestor_record_offset":27256,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":391,"record_offset":27772,"parent_record_offset":27748,"ancestor_record_offset":27256,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":391,"record_offset":28100,"parent_record_offset":28068,"ancestor_record_offset":27256,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23592,"parent_record_offset":23448,"ancestor_record_offset":23384,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":740,"record_offset":23768,"parent_record_offset":23416,"ancestor_record_offset":23384,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00740.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":391,"record_offset":27532,"parent_record_offset":27312,"ancestor_record_offset":27256,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":391,"record_offset":27724,"parent_record_offset":27284,"ancestor_record_offset":27256,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":391,"record_offset":27312,"parent_record_offset":27284,"ancestor_record_offset":27256,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00391.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::distance_squared_segment_point
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A14E
// 首条位置: {"module":3515,"inlinee":106830}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":36752,"parent_record_offset":36716,"ancestor_record_offset":36716,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length_squared
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3515,"record_offset":37312,"parent_record_offset":36716,"ancestor_record_offset":36716,"inlinee":106831,"source_line":60,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_direct] utils::project_point_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3515,"record_offset":36816,"parent_record_offset":36716,"ancestor_record_offset":36716,"inlinee":21612,"source_line":181,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3515,"record_offset":37132,"parent_record_offset":36816,"ancestor_record_offset":36716,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3515,"record_offset":37100,"parent_record_offset":36844,"ancestor_record_offset":36716,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3515,"record_offset":37240,"parent_record_offset":37212,"ancestor_record_offset":36716,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3515,"record_offset":37268,"parent_record_offset":37212,"ancestor_record_offset":36716,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3515,"record_offset":36780,"parent_record_offset":36752,"ancestor_record_offset":36716,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37012,"parent_record_offset":36988,"ancestor_record_offset":36716,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37336,"parent_record_offset":37312,"ancestor_record_offset":36716,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37188,"parent_record_offset":37164,"ancestor_record_offset":36716,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":36876,"parent_record_offset":36844,"ancestor_record_offset":36716,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3515,"record_offset":37212,"parent_record_offset":37188,"ancestor_record_offset":36716,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3515,"record_offset":36988,"parent_record_offset":36844,"ancestor_record_offset":36716,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3515,"record_offset":37164,"parent_record_offset":36816,"ancestor_record_offset":36716,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3515,"record_offset":36844,"parent_record_offset":36816,"ancestor_record_offset":36716,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/03515.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_line
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5A38
// 首条位置: {"module":401,"inlinee":23096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17672,"parent_record_offset":17500,"ancestor_record_offset":17500,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3264,"parent_record_offset":3040,"ancestor_record_offset":3040,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17528,"parent_record_offset":17500,"ancestor_record_offset":17500,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1461,"record_offset":72972,"parent_record_offset":72716,"ancestor_record_offset":72688,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01461.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17748,"parent_record_offset":17720,"ancestor_record_offset":17500,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17776,"parent_record_offset":17720,"ancestor_record_offset":17500,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":401,"record_offset":17580,"parent_record_offset":17556,"ancestor_record_offset":17500,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17636,"parent_record_offset":17612,"ancestor_record_offset":17500,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17500,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17556,"parent_record_offset":17528,"ancestor_record_offset":17500,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":401,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17500,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3224,"parent_record_offset":3196,"ancestor_record_offset":3040,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":3040,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":3040,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":2708,"record_offset":3320,"parent_record_offset":3292,"ancestor_record_offset":3040,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":401,"record_offset":17612,"parent_record_offset":17528,"ancestor_record_offset":17500,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00401.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":2708,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":3040,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/02708.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::project_point_onto_segment
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x546C
// 首条位置: {"module":374,"inlinee":21612}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":20416,"parent_record_offset":19980,"ancestor_record_offset":19980,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29804,"parent_record_offset":29440,"ancestor_record_offset":29440,"inlinee":21658,"source_line":75,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20448,"parent_record_offset":19980,"ancestor_record_offset":19980,"inlinee":21577,"source_line":106,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_direct] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":19980,"inlinee":21613,"source_line":149,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":374,"record_offset":20384,"parent_record_offset":20008,"ancestor_record_offset":19980,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20528,"parent_record_offset":20496,"ancestor_record_offset":19980,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20584,"parent_record_offset":20496,"ancestor_record_offset":19980,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":374,"record_offset":20096,"parent_record_offset":20060,"ancestor_record_offset":19980,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29644,"parent_record_offset":29612,"ancestor_record_offset":29440,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29828,"parent_record_offset":29804,"ancestor_record_offset":29440,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29500,"parent_record_offset":29468,"ancestor_record_offset":29440,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":392,"record_offset":29852,"parent_record_offset":29828,"ancestor_record_offset":29440,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20280,"parent_record_offset":20248,"ancestor_record_offset":19980,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":19980,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20060,"parent_record_offset":20008,"ancestor_record_offset":19980,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":374,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":19980,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=14
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":392,"record_offset":29612,"parent_record_offset":29468,"ancestor_record_offset":29440,"inlinee":22388,"source_line":80,"archive":"_pdb/A/modules/00392.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":374,"record_offset":20248,"parent_record_offset":20008,"ancestor_record_offset":19980,"inlinee":21614,"source_line":111,"archive":"_pdb/A/modules/00374.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits_into
// 记录形式: compiled_procedure; instances=1
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":464,"rva":8229792,"va":5376938912}
// 编译实例 RVAs: 0x7d93a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t25979060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938912,"raw_hex":"e8b4688c01","size":5,"soft_fail":false,"target_va":5402918528,"va":5376939463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5630689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938912,"raw_hex":"e81f15aaff","size":5,"soft_fail":false,"target_va":5371308432,"va":5376939116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] fastrand::Rng::f32
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8910687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376938912,"raw_hex":"e85ff78700","size":5,"soft_fail":false,"target_va":5385850032,"va":5376939340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":1320,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":584,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":1292,"parent_record_offset":464,"ancestor_record_offset":464,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":464,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":744,"parent_record_offset":708,"ancestor_record_offset":464,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":464,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":708,"parent_record_offset":680,"ancestor_record_offset":464,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":820,"parent_record_offset":640,"ancestor_record_offset":464,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":464,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":680,"parent_record_offset":640,"ancestor_record_offset":464,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":1040,"parent_record_offset":1020,"ancestor_record_offset":464,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":615,"record_offset":2248,"parent_record_offset":2192,"ancestor_record_offset":464,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":615,"record_offset":1252,"parent_record_offset":640,"ancestor_record_offset":464,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":615,"record_offset":1204,"parent_record_offset":1184,"ancestor_record_offset":464,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":615,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":464,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":615,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":464,"inlinee":6065,"source_line":262,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":615,"record_offset":996,"parent_record_offset":976,"ancestor_record_offset":464,"inlinee":6073,"source_line":267,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":615,"record_offset":2004,"parent_record_offset":1972,"ancestor_record_offset":464,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::by_ref_sized::impl$0::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\by_ref_sized.rs
//     evidence={"module":615,"record_offset":928,"parent_record_offset":900,"ancestor_record_offset":464,"inlinee":6067,"source_line":40,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":615,"record_offset":1868,"parent_record_offset":1836,"ancestor_record_offset":464,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":615,"record_offset":1724,"parent_record_offset":1520,"ancestor_record_offset":464,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":615,"record_offset":2104,"parent_record_offset":1900,"ancestor_record_offset":464,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":615,"record_offset":2036,"parent_record_offset":1972,"ancestor_record_offset":464,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":615,"record_offset":1972,"parent_record_offset":1944,"ancestor_record_offset":464,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":615,"record_offset":1944,"parent_record_offset":1900,"ancestor_record_offset":464,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":615,"record_offset":1748,"parent_record_offset":1724,"ancestor_record_offset":464,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":615,"record_offset":1900,"parent_record_offset":1868,"ancestor_record_offset":464,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":615,"record_offset":876,"parent_record_offset":640,"ancestor_record_offset":464,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":615,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":464,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":615,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":464,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":615,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":464,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":615,"record_offset":1108,"parent_record_offset":952,"ancestor_record_offset":464,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::try_trait::impl$0::wrap_mut_2::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\try_trait.rs
//     evidence={"module":615,"record_offset":976,"parent_record_offset":952,"ancestor_record_offset":464,"inlinee":6071,"source_line":390,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":615,"record_offset":1144,"parent_record_offset":900,"ancestor_record_offset":464,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":615,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":464,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] utils::random_splits_into::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":464,"inlinee":36055,"source_line":208,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits_into::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 208
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CD7
// 首条位置: {"module":615,"inlinee":36055}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::random_splits
// 记录形式: compiled_procedure; instances=1
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":2744,"rva":8230496,"va":5376939616}
// 编译实例 RVAs: 0x7d9660
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4190006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e8ca10c0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376939697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4190164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e82c10c0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376939871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26038331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e83b508d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5376939808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t587320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e838f60800","size":5,"soft_fail":false,"target_va":5377527120,"va":5376939795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26039357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e83d548d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5376939790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::random_splits_into
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939616,"raw_hex":"e8c3fcffff","size":5,"soft_fail":false,"target_va":5376938912,"va":5376939736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":2852,"parent_record_offset":2744,"ancestor_record_offset":2744,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":615,"record_offset":3188,"parent_record_offset":2744,"ancestor_record_offset":2744,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":615,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2744,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":615,"record_offset":3072,"parent_record_offset":3048,"ancestor_record_offset":2744,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":615,"record_offset":3364,"parent_record_offset":3340,"ancestor_record_offset":2744,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":615,"record_offset":3024,"parent_record_offset":2936,"ancestor_record_offset":2744,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":615,"record_offset":3340,"parent_record_offset":3240,"ancestor_record_offset":2744,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":3272,"parent_record_offset":3240,"ancestor_record_offset":2744,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":2936,"parent_record_offset":2908,"ancestor_record_offset":2744,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":2908,"parent_record_offset":2880,"ancestor_record_offset":2744,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":3144,"parent_record_offset":2908,"ancestor_record_offset":2744,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":615,"record_offset":3240,"parent_record_offset":3212,"ancestor_record_offset":2744,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":615,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2744,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":615,"record_offset":2968,"parent_record_offset":2936,"ancestor_record_offset":2744,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":615,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2744,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":615,"record_offset":3304,"parent_record_offset":3272,"ancestor_record_offset":2744,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":615,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2744,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":615,"record_offset":3100,"parent_record_offset":3072,"ancestor_record_offset":2744,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::smoothstep
// 记录形式: inline_instance, inlinee_declaration; instances=51
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5417
// 首条位置: {"module":371,"inlinee":21527}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":371,"record_offset":19592,"parent_record_offset":19564,"ancestor_record_offset":19564,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00371.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::inverse_lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=7
// 源码行: 254, 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CD8
// 首条位置: {"module":615,"inlinee":36056}
// 编译实例 RVAs: 0x7d9780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::linearstep
// 记录形式: compiled_procedure; instances=1
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":3508,"rva":8230800,"va":5376939920}
// 编译实例 RVAs: 0x7d9790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":3640,"parent_record_offset":3508,"ancestor_record_offset":3508,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] utils::inverse_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":3616,"parent_record_offset":3508,"ancestor_record_offset":3508,"inlinee":36056,"source_line":254,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=13
// 源码行: 263, 264
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CD9
// 首条位置: {"module":615,"inlinee":36057}
// 编译实例 RVAs: 0x7d97b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CDA
// 首条位置: {"module":615,"inlinee":36058}
// 编译实例 RVAs: 0x7d97c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26654057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376939968,"raw_hex":"e869b59601","size":5,"soft_fail":false,"target_va":5403594064,"va":5376940002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":3876,"parent_record_offset":3760,"ancestor_record_offset":3760,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":3904,"parent_record_offset":3760,"ancestor_record_offset":3760,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_vec2
// 记录形式: compiled_procedure; instances=1
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":3936,"rva":8230944,"va":5376940064}
// 编译实例 RVAs: 0x7d9820
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940064,"raw_hex":"e8eab49601","size":5,"soft_fail":false,"target_va":5403594064,"va":5376940129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":4056,"parent_record_offset":3936,"ancestor_record_offset":3936,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":4084,"parent_record_offset":3936,"ancestor_record_offset":3936,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_vec3
// 记录形式: compiled_procedure; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":4144,"rva":8231088,"va":5376940208}
// 编译实例 RVAs: 0x7d98b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940208,"raw_hex":"e87bb49601","size":5,"soft_fail":false,"target_va":5403594064,"va":5376940240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":4408,"parent_record_offset":4144,"ancestor_record_offset":4144,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":4264,"parent_record_offset":4144,"ancestor_record_offset":4144,"inlinee":36058,"source_line":267,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":4144,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":4320,"parent_record_offset":4264,"ancestor_record_offset":4144,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::time_continuous_lerp_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 285
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":4440,"rva":8231200,"va":5376940320}
// 编译实例 RVAs: 0x7d9920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d50
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940320,"raw_hex":"e804b49601","size":5,"soft_fail":false,"target_va":5403594064,"va":5376940359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::time_continuous_lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":4556,"parent_record_offset":4440,"ancestor_record_offset":4440,"inlinee":36058,"source_line":267,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::exp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":4584,"parent_record_offset":4556,"ancestor_record_offset":4440,"inlinee":16818,"source_line":467,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":4612,"parent_record_offset":4556,"ancestor_record_offset":4440,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::perspective_lerp
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 293
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x62E6
// 首条位置: {"module":438,"inlinee":25318}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::mouse_ss_to_cs
// 记录形式: compiled_procedure; instances=1
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":4648,"rva":8231296,"va":5376940416}
// 编译实例 RVAs: 0x7d9980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":4936,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":4764,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":4848,"parent_record_offset":4648,"ancestor_record_offset":4648,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":4648,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4648,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":4648,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_disk
// 记录形式: compiled_procedure; instances=1
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":5028,"rva":8231344,"va":5376940464}
// 编译实例 RVAs: 0x7d99b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940464,"raw_hex":"e829b39601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376940514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653800","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940464,"raw_hex":"e868b49601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376940531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5176,"parent_record_offset":5028,"ancestor_record_offset":5028,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5204,"parent_record_offset":5028,"ancestor_record_offset":5028,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5148,"parent_record_offset":5028,"ancestor_record_offset":5028,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_circle
// 记录形式: compiled_procedure; instances=1
// 源码行: 308
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":5236,"rva":8231456,"va":5376940576}
// 编译实例 RVAs: 0x7d9a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940576,"raw_hex":"e8c6b29601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376940613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940576,"raw_hex":"e805b49601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376940630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5352,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5380,"parent_record_offset":5236,"ancestor_record_offset":5236,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_sphere
// 记录形式: compiled_procedure; instances=1
// 源码行: 313
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":5412,"rva":8231552,"va":5376940672}
// 编译实例 RVAs: 0x7d9a80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940672,"raw_hex":"e819b29601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376940786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940672,"raw_hex":"e874b39601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376940775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":5540,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":5652,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5624,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5596,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5568,"parent_record_offset":5412,"ancestor_record_offset":5412,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::uniform_sample_cone
// 记录形式: compiled_procedure; instances=1
// 源码行: 321
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":5684,"rva":8231728,"va":5376940848}
// 编译实例 RVAs: 0x7d9b30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940848,"raw_hex":"e86fb19601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376940956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376940848,"raw_hex":"e8b3b29601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376940968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":5840,"parent_record_offset":5684,"ancestor_record_offset":5684,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":5924,"parent_record_offset":5684,"ancestor_record_offset":5684,"inlinee":14693,"source_line":76,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5868,"parent_record_offset":5684,"ancestor_record_offset":5684,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5896,"parent_record_offset":5684,"ancestor_record_offset":5684,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":5812,"parent_record_offset":5684,"ancestor_record_offset":5684,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::golden_spiral
// 记录形式: compiled_procedure; instances=1
// 源码行: 328
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":5956,"rva":8231920,"va":5376941040}
// 编译实例 RVAs: 0x7d9bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26652914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376941040,"raw_hex":"e8f2b09601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376941081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26653238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376941040,"raw_hex":"e836b29601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376941093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":6124,"parent_record_offset":5956,"ancestor_record_offset":5956,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":6068,"parent_record_offset":5956,"ancestor_record_offset":5956,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":6096,"parent_record_offset":5956,"ancestor_record_offset":5956,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":5956,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::radical_inverse
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 333
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CF7
// 首条位置: {"module":615,"record_offset":6212,"rva":8232016,"va":5376941136}
// 编译实例 RVAs: 0x7d9c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26037851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376941136,"raw_hex":"e85b4e8d01","size":5,"soft_fail":false,"target_va":5402979152,"va":5376941296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rotation_from_normal
// 记录形式: compiled_procedure; instances=1
// 源码行: 348
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":6312,"rva":8232192,"va":5376941312}
// 编译实例 RVAs: 0x7d9d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":7028,"parent_record_offset":6312,"ancestor_record_offset":6312,"inlinee":36059,"source_line":253,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":6432,"parent_record_offset":6312,"ancestor_record_offset":6312,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":6468,"parent_record_offset":6312,"ancestor_record_offset":6312,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":6632,"parent_record_offset":6496,"ancestor_record_offset":6312,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":615,"record_offset":7148,"parent_record_offset":7052,"ancestor_record_offset":6312,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":6312,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":7372,"parent_record_offset":7052,"ancestor_record_offset":6312,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":6564,"parent_record_offset":6532,"ancestor_record_offset":6312,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":6532,"parent_record_offset":6496,"ancestor_record_offset":6312,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":6496,"parent_record_offset":6468,"ancestor_record_offset":6312,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":6596,"parent_record_offset":6564,"ancestor_record_offset":6312,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::pack_2x16f_uint
// 记录形式: compiled_procedure; instances=1
// 源码行: 356
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":7412,"rva":8232832,"va":5376941952}
// 编译实例 RVAs: 0x7d9f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe3e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1948875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376941952,"raw_hex":"e83543e2ff","size":5,"soft_fail":false,"target_va":5374993376,"va":5376942246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std_detect::detect::cache::detect_and_initialize
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26253145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376941952,"raw_hex":"e859979001","size":5,"soft_fail":false,"target_va":5403195376,"va":5376942226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] half::binary16::f16::from_f32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16.rs
//     evidence={"module":615,"record_offset":7520,"parent_record_offset":7412,"ancestor_record_offset":7412,"inlinee":25677,"source_line":65,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":615,"record_offset":7772,"parent_record_offset":7656,"ancestor_record_offset":7412,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":615,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":7412,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":615,"record_offset":7736,"parent_record_offset":7712,"ancestor_record_offset":7412,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":615,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":7412,"inlinee":25679,"source_line":64,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] half::binary16::arch::f32_to_f16_fallback
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"half","version":"2.4.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/half-2.4.1/src/binary16/arch.rs
//     evidence={"module":615,"record_offset":7840,"parent_record_offset":7544,"ancestor_record_offset":7412,"inlinee":25688,"source_line":482,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::arch::x86::__is_feature_detected::f16c
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\macros.rs
//     evidence={"module":615,"record_offset":7600,"parent_record_offset":7544,"ancestor_record_offset":7412,"inlinee":25681,"source_line":170,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::Cache::test
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":615,"record_offset":7688,"parent_record_offset":7656,"ancestor_record_offset":7412,"inlinee":25687,"source_line":100,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":615,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":7412,"inlinee":25685,"source_line":180,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::cache::test::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\cache.rs
//     evidence={"module":615,"record_offset":7796,"parent_record_offset":7772,"ancestor_record_offset":7412,"inlinee":25690,"source_line":188,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std_detect::detect::check_for
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\stdarch\crates\std_detect\src\detect\mod.rs
//     evidence={"module":615,"record_offset":7628,"parent_record_offset":7600,"ancestor_record_offset":7412,"inlinee":25683,"source_line":81,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cartesian_to_polar
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 405
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CFD
// 首条位置: {"module":615,"record_offset":8340,"rva":8233456,"va":5376942576}
// 编译实例 RVAs: 0x7da1f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144b80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26650993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376942576,"raw_hex":"e871a99601","size":5,"soft_fail":false,"target_va":5403593600,"va":5376942602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cb0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26651282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376942576,"raw_hex":"e892aa9601","size":5,"soft_fail":false,"target_va":5403593904,"va":5376942617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::atan2
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":8484,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":13077,"source_line":886,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::hypot
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":8456,"parent_record_offset":8340,"ancestor_record_offset":8340,"inlinee":27085,"source_line":690,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::polar_to_cartesian
// 记录形式: compiled_procedure; instances=1
// 源码行: 412
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":8516,"rva":8233536,"va":5376942656}
// 编译实例 RVAs: 0x7da240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26651310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376942656,"raw_hex":"e8aeaa9601","size":5,"soft_fail":false,"target_va":5403594000,"va":5376942685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e60
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26651629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376942656,"raw_hex":"e8edab9601","size":5,"soft_fail":false,"target_va":5403594336,"va":5376942702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::f32::impl$0::cos
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":8632,"parent_record_offset":8516,"ancestor_record_offset":8516,"inlinee":13073,"source_line":736,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sin
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":8660,"parent_record_offset":8516,"ancestor_record_offset":8516,"inlinee":13074,"source_line":713,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3
// 记录形式: compiled_procedure; instances=1
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":8692,"rva":8233632,"va":5376942752}
// 编译实例 RVAs: 0x7da2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::mat3::Mat3::from_cols
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/mat3.rs
//     evidence={"module":615,"record_offset":9308,"parent_record_offset":8692,"ancestor_record_offset":8692,"inlinee":36060,"source_line":82,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":8812,"parent_record_offset":8692,"ancestor_record_offset":8692,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":9280,"parent_record_offset":8692,"ancestor_record_offset":8692,"inlinee":14686,"source_line":209,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":9040,"parent_record_offset":8692,"ancestor_record_offset":8692,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":8872,"parent_record_offset":8836,"ancestor_record_offset":8692,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":8936,"parent_record_offset":8904,"ancestor_record_offset":8692,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":8904,"parent_record_offset":8836,"ancestor_record_offset":8692,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":8692,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":9140,"parent_record_offset":9104,"ancestor_record_offset":8692,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":9104,"parent_record_offset":9068,"ancestor_record_offset":8692,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":615,"record_offset":9068,"parent_record_offset":9040,"ancestor_record_offset":8692,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":8968,"parent_record_offset":8936,"ancestor_record_offset":8692,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation_mat3a
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CDF
// 首条位置: {"module":615,"inlinee":36063}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":615,"record_offset":9696,"parent_record_offset":9448,"ancestor_record_offset":9448,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9780,"parent_record_offset":9448,"ancestor_record_offset":9448,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9928,"parent_record_offset":9448,"ancestor_record_offset":9448,"inlinee":36061,"source_line":541,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9476,"parent_record_offset":9448,"ancestor_record_offset":9448,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":10096,"parent_record_offset":9928,"ancestor_record_offset":9448,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":9448,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9896,"parent_record_offset":9780,"ancestor_record_offset":9448,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":10068,"parent_record_offset":9928,"ancestor_record_offset":9448,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9868,"parent_record_offset":9780,"ancestor_record_offset":9448,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":9632,"parent_record_offset":9500,"ancestor_record_offset":9448,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9448,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9448,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":9448,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":9448,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9500,"parent_record_offset":9476,"ancestor_record_offset":9448,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":9448,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":9448,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":9448,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_rotation
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 438
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE1
// 首条位置: {"module":615,"inlinee":36065}
// 编译实例 RVAs: 0x7da360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":10132,"parent_record_offset":9340,"ancestor_record_offset":9340,"inlinee":36064,"source_line":260,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_direct] utils::xz_rotation_mat3a
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":9448,"parent_record_offset":9340,"ancestor_record_offset":9340,"inlinee":36063,"source_line":428,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":615,"record_offset":9696,"parent_record_offset":9448,"ancestor_record_offset":9340,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":10096,"parent_record_offset":9928,"ancestor_record_offset":9340,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":9340,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9896,"parent_record_offset":9780,"ancestor_record_offset":9340,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":10068,"parent_record_offset":9928,"ancestor_record_offset":9340,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":9868,"parent_record_offset":9780,"ancestor_record_offset":9340,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":615,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":9340,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":9632,"parent_record_offset":9500,"ancestor_record_offset":9340,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":615,"record_offset":10336,"parent_record_offset":10240,"ancestor_record_offset":9340,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":10240,"parent_record_offset":10132,"ancestor_record_offset":9340,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":10560,"parent_record_offset":10240,"ancestor_record_offset":9340,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9780,"parent_record_offset":9448,"ancestor_record_offset":9340,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9744,"parent_record_offset":9720,"ancestor_record_offset":9340,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9928,"parent_record_offset":9448,"ancestor_record_offset":9340,"inlinee":36061,"source_line":541,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":9720,"parent_record_offset":9696,"ancestor_record_offset":9340,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$47::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9340,"inlinee":23171,"source_line":1375,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":9340,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":9340,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9500,"parent_record_offset":9476,"ancestor_record_offset":9340,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":9476,"parent_record_offset":9448,"ancestor_record_offset":9340,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":9340,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":9340,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":9596,"parent_record_offset":9564,"ancestor_record_offset":9340,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::xz_projective_rotation
// 记录形式: compiled_procedure; instances=1
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":10600,"rva":8234256,"va":5376943376}
// 编译实例 RVAs: 0x7da510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::xz_rotation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":10716,"parent_record_offset":10600,"ancestor_record_offset":10600,"inlinee":36065,"source_line":438,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":615,"record_offset":10988,"parent_record_offset":10740,"ancestor_record_offset":10600,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":11392,"parent_record_offset":11220,"ancestor_record_offset":10600,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":11104,"parent_record_offset":11072,"ancestor_record_offset":10600,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":11188,"parent_record_offset":11072,"ancestor_record_offset":10600,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":11364,"parent_record_offset":11220,"ancestor_record_offset":10600,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":615,"record_offset":11160,"parent_record_offset":11072,"ancestor_record_offset":10600,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":615,"record_offset":11500,"parent_record_offset":11476,"ancestor_record_offset":10600,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":615,"record_offset":10924,"parent_record_offset":10792,"ancestor_record_offset":10600,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::sqrt
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":615,"record_offset":11632,"parent_record_offset":11536,"ancestor_record_offset":10600,"inlinee":22228,"source_line":187,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_mat3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":11428,"parent_record_offset":10716,"ancestor_record_offset":10600,"inlinee":36064,"source_line":260,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_rotation_axes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":11536,"parent_record_offset":11428,"ancestor_record_offset":10600,"inlinee":26368,"source_line":192,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::from_xyzw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":615,"record_offset":11856,"parent_record_offset":11536,"ancestor_record_offset":10600,"inlinee":14673,"source_line":72,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":11072,"parent_record_offset":10740,"ancestor_record_offset":10600,"inlinee":14682,"source_line":220,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":10600,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":11220,"parent_record_offset":10740,"ancestor_record_offset":10600,"inlinee":36061,"source_line":541,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":10600,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$47::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":615,"record_offset":11476,"parent_record_offset":11452,"ancestor_record_offset":10600,"inlinee":23171,"source_line":1375,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":10856,"parent_record_offset":10824,"ancestor_record_offset":10600,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":10824,"parent_record_offset":10792,"ancestor_record_offset":10600,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":10792,"parent_record_offset":10768,"ancestor_record_offset":10600,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":615,"record_offset":10768,"parent_record_offset":10740,"ancestor_record_offset":10600,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":11276,"parent_record_offset":11248,"ancestor_record_offset":10600,"inlinee":21746,"source_line":42,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":615,"record_offset":11248,"parent_record_offset":11220,"ancestor_record_offset":10600,"inlinee":21745,"source_line":66,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":615,"record_offset":10888,"parent_record_offset":10856,"ancestor_record_offset":10600,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] utils::xz_rotation_mat3a
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":10740,"parent_record_offset":10716,"ancestor_record_offset":10600,"inlinee":36063,"source_line":428,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::cos_angle_between
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 447
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE9EA
// 首条位置: {"module":1248,"inlinee":59882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":1448,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1500,"parent_record_offset":1448,"ancestor_record_offset":1448,"inlinee":21909,"source_line":390,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1248,"record_offset":1556,"parent_record_offset":1448,"ancestor_record_offset":1448,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1248,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":1448,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01248.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$5::intersects
// 记录形式: compiled_procedure; instances=1
// 源码行: 473
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2722,"record_offset":244,"rva":24640544,"va":5393349664}
// 编译实例 RVAs: 0x177fc20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::impls::impl$10::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2722,"record_offset":360,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":43539,"source_line":1684,"archive":"_pdb/A/modules/02722.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2722,"record_offset":408,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/02722.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2722,"record_offset":380,"parent_record_offset":360,"ancestor_record_offset":244,"inlinee":17312,"source_line":1519,"archive":"_pdb/A/modules/02722.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2722,"record_offset":428,"parent_record_offset":408,"ancestor_record_offset":244,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/02722.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$6::intersects
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 479, 480
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB2D0
// 首条位置: {"module":740,"inlinee":45776}
// 编译实例 RVAs: 0x177fc40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<glam::i32::ivec3::IVec3>
// 记录形式: compiled_procedure; instances=2
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":10248,"rva":7083648,"va":5375792768}
// 编译实例 RVAs: 0x6c1680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=6
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6174857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375792768,"raw_hex":"e877c7a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375792884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] glam::i32::ivec3::impl$77::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/i32/ivec3.rs
//     evidence={"module":495,"record_offset":10416,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":27798,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":10388,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":10968,"parent_record_offset":10248,"ancestor_record_offset":10248,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":10248,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":10496,"parent_record_offset":10472,"ancestor_record_offset":10248,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":10448,"parent_record_offset":10416,"ancestor_record_offset":10248,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":10992,"parent_record_offset":10968,"ancestor_record_offset":10248,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":10248,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":10248,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":11064,"parent_record_offset":11016,"ancestor_record_offset":10248,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":11456,"parent_record_offset":11016,"ancestor_record_offset":10248,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":10520,"parent_record_offset":10496,"ancestor_record_offset":10248,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":11164,"parent_record_offset":11064,"ancestor_record_offset":10248,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":11132,"parent_record_offset":11064,"ancestor_record_offset":10248,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":10552,"parent_record_offset":10496,"ancestor_record_offset":10248,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<array$<u32,2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":252,"rva":28175472,"va":5396884592}
// 编译实例 RVAs: 0x1adec70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27266686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396884592,"raw_hex":"e882f15ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396884713,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141aa7950
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-226264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396884592,"raw_hex":"e8288cfcff","size":5,"soft_fail":false,"target_va":5396658512,"va":5396884771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":428,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":400,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":980,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":3529,"record_offset":792,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":624,"parent_record_offset":452,"ancestor_record_offset":252,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":252,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":252,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":932,"parent_record_offset":816,"ancestor_record_offset":252,"inlinee":23504,"source_line":811,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":252,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":1004,"parent_record_offset":980,"ancestor_record_offset":252,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":584,"parent_record_offset":560,"ancestor_record_offset":252,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":252,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":1076,"parent_record_offset":1028,"ancestor_record_offset":252,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":1468,"parent_record_offset":1028,"ancestor_record_offset":252,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":1176,"parent_record_offset":1076,"ancestor_record_offset":252,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":1144,"parent_record_offset":1076,"ancestor_record_offset":252,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3529,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3529,"record_offset":700,"parent_record_offset":656,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3529,"record_offset":732,"parent_record_offset":656,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3529,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":560,"parent_record_offset":504,"ancestor_record_offset":252,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<glam::u32::uvec3::UVec3> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":30552,"rva":7087472,"va":5375796592}
// 编译实例 RVAs: 0x6c2570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6178684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375796592,"raw_hex":"e884b8a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375796711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":30736,"parent_record_offset":30552,"ancestor_record_offset":30552,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":30708,"parent_record_offset":30552,"ancestor_record_offset":30552,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":31352,"parent_record_offset":30552,"ancestor_record_offset":30552,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":30932,"parent_record_offset":30760,"ancestor_record_offset":30552,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":30788,"parent_record_offset":30760,"ancestor_record_offset":30552,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":31096,"parent_record_offset":31072,"ancestor_record_offset":30552,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":30812,"parent_record_offset":30788,"ancestor_record_offset":30552,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":31072,"parent_record_offset":31040,"ancestor_record_offset":30552,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":30760,"parent_record_offset":30736,"ancestor_record_offset":30552,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":31376,"parent_record_offset":31352,"ancestor_record_offset":30552,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":30892,"parent_record_offset":30868,"ancestor_record_offset":30552,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":30552,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":31448,"parent_record_offset":31400,"ancestor_record_offset":30552,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":31840,"parent_record_offset":31400,"ancestor_record_offset":30552,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":30836,"parent_record_offset":30812,"ancestor_record_offset":30552,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":31120,"parent_record_offset":31096,"ancestor_record_offset":30552,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":31548,"parent_record_offset":31448,"ancestor_record_offset":30552,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":31516,"parent_record_offset":31448,"ancestor_record_offset":30552,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":31004,"parent_record_offset":30964,"ancestor_record_offset":30552,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":30964,"parent_record_offset":30932,"ancestor_record_offset":30552,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$76::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec3.rs
//     evidence={"module":495,"record_offset":31040,"parent_record_offset":30932,"ancestor_record_offset":30552,"inlinee":22469,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":30868,"parent_record_offset":30812,"ancestor_record_offset":30552,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":32412,"rva":7087968,"va":5375797088}
// 编译实例 RVAs: 0x6c2760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6179181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375797088,"raw_hex":"e893b6a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375797208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":32576,"parent_record_offset":32412,"ancestor_record_offset":32412,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":32548,"parent_record_offset":32412,"ancestor_record_offset":32412,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":32864,"parent_record_offset":32412,"ancestor_record_offset":32412,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":32628,"parent_record_offset":32600,"ancestor_record_offset":32412,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":32652,"parent_record_offset":32628,"ancestor_record_offset":32412,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":32600,"parent_record_offset":32576,"ancestor_record_offset":32412,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":32772,"parent_record_offset":32600,"ancestor_record_offset":32412,"inlinee":15045,"source_line":811,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":32888,"parent_record_offset":32864,"ancestor_record_offset":32412,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":32732,"parent_record_offset":32708,"ancestor_record_offset":32412,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":32912,"parent_record_offset":32888,"ancestor_record_offset":32412,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":32960,"parent_record_offset":32912,"ancestor_record_offset":32412,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":33352,"parent_record_offset":32912,"ancestor_record_offset":32412,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":32676,"parent_record_offset":32652,"ancestor_record_offset":32412,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":33060,"parent_record_offset":32960,"ancestor_record_offset":32412,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":33028,"parent_record_offset":32960,"ancestor_record_offset":32412,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":32708,"parent_record_offset":32652,"ancestor_record_offset":32412,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<ref$<str$> >
// 记录形式: compiled_procedure; instances=8
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":17016,"rva":908528,"va":5369617648}
// 编译实例 RVAs: 0xddcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=12
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617648,"raw_hex":"e815010000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369617750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":17168,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":17144,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":17336,"parent_record_offset":17016,"ancestor_record_offset":17016,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":17016,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":17192,"parent_record_offset":17168,"ancestor_record_offset":17016,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17360,"parent_record_offset":17336,"ancestor_record_offset":17016,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17240,"parent_record_offset":17216,"ancestor_record_offset":17016,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":17016,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":17016,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17432,"parent_record_offset":17384,"ancestor_record_offset":17016,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":17824,"parent_record_offset":17384,"ancestor_record_offset":17016,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=96
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":17532,"parent_record_offset":17432,"ancestor_record_offset":17016,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=64
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":17500,"parent_record_offset":17432,"ancestor_record_offset":17016,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":17216,"parent_record_offset":17192,"ancestor_record_offset":17016,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<rhapsody::raw::texture::TextureViewDesc>
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":20884,"rva":7085216,"va":5375794336}
// 编译实例 RVAs: 0x6c1ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=15
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6176429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375794336,"raw_hex":"e853c1a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375794456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$38::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":495,"record_offset":21068,"parent_record_offset":20884,"ancestor_record_offset":20884,"inlinee":27800,"source_line":251,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":21040,"parent_record_offset":20884,"ancestor_record_offset":20884,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":23804,"parent_record_offset":20884,"ancestor_record_offset":20884,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$350::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":495,"record_offset":21732,"parent_record_offset":21528,"ancestor_record_offset":20884,"inlinee":27804,"source_line":183,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$622::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":495,"record_offset":22520,"parent_record_offset":22316,"ancestor_record_offset":20884,"inlinee":27806,"source_line":389,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$306::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":495,"record_offset":21324,"parent_record_offset":21124,"ancestor_record_offset":20884,"inlinee":27802,"source_line":110,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":495,"record_offset":22112,"parent_record_offset":21908,"ancestor_record_offset":20884,"inlinee":22459,"source_line":649,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21372,"parent_record_offset":21348,"ancestor_record_offset":20884,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":20884,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21396,"parent_record_offset":21372,"ancestor_record_offset":20884,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":20884,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":20884,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$0::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":23688,"parent_record_offset":21068,"ancestor_record_offset":20884,"inlinee":27808,"source_line":842,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21756,"parent_record_offset":21732,"ancestor_record_offset":20884,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21348,"parent_record_offset":21324,"ancestor_record_offset":20884,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":20884,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":23828,"parent_record_offset":23804,"ancestor_record_offset":20884,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":21280,"parent_record_offset":21256,"ancestor_record_offset":20884,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":23852,"parent_record_offset":23828,"ancestor_record_offset":20884,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":23900,"parent_record_offset":23852,"ancestor_record_offset":20884,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":24292,"parent_record_offset":23852,"ancestor_record_offset":20884,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":20884,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":20884,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":24000,"parent_record_offset":23900,"ancestor_record_offset":20884,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":23968,"parent_record_offset":23900,"ancestor_record_offset":20884,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$50::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":21124,"parent_record_offset":21068,"ancestor_record_offset":20884,"inlinee":16342,"source_line":567,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":21256,"parent_record_offset":21200,"ancestor_record_offset":20884,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":17424,"rva":908160,"va":5369617280}
// 编译实例 RVAs: 0xddb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617280,"raw_hex":"e87d020000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369617390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":17600,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":17820,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$2::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":72,"record_offset":17624,"parent_record_offset":17424,"ancestor_record_offset":17424,"inlinee":8471,"source_line":93,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":17424,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":17424,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":17644,"parent_record_offset":17624,"ancestor_record_offset":17424,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":17844,"parent_record_offset":17820,"ancestor_record_offset":17424,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":17772,"parent_record_offset":17748,"ancestor_record_offset":17424,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":17868,"parent_record_offset":17844,"ancestor_record_offset":17424,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":17916,"parent_record_offset":17868,"ancestor_record_offset":17424,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":18308,"parent_record_offset":17868,"ancestor_record_offset":17424,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":17424,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":18016,"parent_record_offset":17916,"ancestor_record_offset":17424,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":17984,"parent_record_offset":17916,"ancestor_record_offset":17424,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":17748,"parent_record_offset":17692,"ancestor_record_offset":17424,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<country_core::resources::walls::WallId,2>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":10784,"rva":28177712,"va":5396886832}
// 编译实例 RVAs: 0x1adf530
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27268960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396886832,"raw_hex":"e8a0e85ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396886987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141991380
//     owner=unresolved_namespace; count=3
//     target_aliases=17 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1368605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396886832,"raw_hex":"e8e31debff","size":5,"soft_fail":false,"target_va":5395518336,"va":5396886936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":10992,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":10964,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":11340,"parent_record_offset":10784,"ancestor_record_offset":10784,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":3529,"record_offset":11016,"parent_record_offset":10992,"ancestor_record_offset":10784,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":11092,"parent_record_offset":11040,"ancestor_record_offset":10784,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":11064,"parent_record_offset":11040,"ancestor_record_offset":10784,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":10784,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":11144,"parent_record_offset":11116,"ancestor_record_offset":10784,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":11040,"parent_record_offset":11016,"ancestor_record_offset":10784,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":11364,"parent_record_offset":11340,"ancestor_record_offset":10784,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10784,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":11388,"parent_record_offset":11364,"ancestor_record_offset":10784,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":11436,"parent_record_offset":11388,"ancestor_record_offset":10784,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":11828,"parent_record_offset":11388,"ancestor_record_offset":10784,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10784,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":11536,"parent_record_offset":11436,"ancestor_record_offset":10784,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":11504,"parent_record_offset":11436,"ancestor_record_offset":10784,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3529,"record_offset":11116,"parent_record_offset":11092,"ancestor_record_offset":10784,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":11272,"parent_record_offset":11216,"ancestor_record_offset":10784,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":8376,"rva":7083168,"va":5375792288}
// 编译实例 RVAs: 0x6c14a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=5
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6174381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375792288,"raw_hex":"e853c9a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375792408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":8608,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":8580,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":9188,"parent_record_offset":8376,"ancestor_record_offset":8376,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":495,"record_offset":8632,"parent_record_offset":8608,"ancestor_record_offset":8376,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":8376,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":8684,"parent_record_offset":8656,"ancestor_record_offset":8376,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":8376,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":8772,"parent_record_offset":8656,"ancestor_record_offset":8376,"inlinee":23504,"source_line":811,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":8376,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":8656,"parent_record_offset":8632,"ancestor_record_offset":8376,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":9212,"parent_record_offset":9188,"ancestor_record_offset":8376,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":8376,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":8376,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":9284,"parent_record_offset":9236,"ancestor_record_offset":8376,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":9676,"parent_record_offset":9236,"ancestor_record_offset":8376,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":8732,"parent_record_offset":8708,"ancestor_record_offset":8376,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":9384,"parent_record_offset":9284,"ancestor_record_offset":8376,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":9352,"parent_record_offset":9284,"ancestor_record_offset":8376,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$31::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":495,"record_offset":8984,"parent_record_offset":8608,"ancestor_record_offset":8376,"inlinee":27796,"source_line":449,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":9112,"parent_record_offset":9056,"ancestor_record_offset":8376,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<array$<u32,3>,glam::u32::uvec3::UVec3> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":4808,"rva":7082288,"va":5375791408}
// 编译实例 RVAs: 0x6c1130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=5
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6173500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375791408,"raw_hex":"e8c4cca1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375791527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5000,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":4972,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":5672,"parent_record_offset":4808,"ancestor_record_offset":4808,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":495,"record_offset":5024,"parent_record_offset":5000,"ancestor_record_offset":4808,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4808,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4808,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":4808,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5232,"parent_record_offset":5200,"ancestor_record_offset":4808,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5164,"parent_record_offset":5048,"ancestor_record_offset":4808,"inlinee":23504,"source_line":811,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4808,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":5696,"parent_record_offset":5672,"ancestor_record_offset":4808,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":4808,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":4808,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":5768,"parent_record_offset":5720,"ancestor_record_offset":4808,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":6160,"parent_record_offset":5720,"ancestor_record_offset":4808,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":5124,"parent_record_offset":5100,"ancestor_record_offset":4808,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4808,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":5868,"parent_record_offset":5768,"ancestor_record_offset":4808,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":5836,"parent_record_offset":5768,"ancestor_record_offset":4808,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$76::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec3.rs
//     evidence={"module":495,"record_offset":5200,"parent_record_offset":5000,"ancestor_record_offset":4808,"inlinee":22469,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":5312,"parent_record_offset":5256,"ancestor_record_offset":4808,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::WallId,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":13984,"rva":907360,"va":5369616480}
// 编译实例 RVAs: 0xdd860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616480,"raw_hex":"e894050000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369616599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":14180,"parent_record_offset":13984,"ancestor_record_offset":13984,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":14152,"parent_record_offset":13984,"ancestor_record_offset":13984,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":14524,"parent_record_offset":13984,"ancestor_record_offset":13984,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":14252,"parent_record_offset":14228,"ancestor_record_offset":13984,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":14228,"parent_record_offset":14204,"ancestor_record_offset":13984,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":14548,"parent_record_offset":14524,"ancestor_record_offset":13984,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":14332,"parent_record_offset":14308,"ancestor_record_offset":13984,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":14572,"parent_record_offset":14548,"ancestor_record_offset":13984,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":14620,"parent_record_offset":14572,"ancestor_record_offset":13984,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":15012,"parent_record_offset":14572,"ancestor_record_offset":13984,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":14276,"parent_record_offset":14252,"ancestor_record_offset":13984,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":14720,"parent_record_offset":14620,"ancestor_record_offset":13984,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":14688,"parent_record_offset":14620,"ancestor_record_offset":13984,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":495,"record_offset":14204,"parent_record_offset":14180,"ancestor_record_offset":13984,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":14308,"parent_record_offset":14252,"ancestor_record_offset":13984,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2748,"record_offset":3560,"rva":24805568,"va":5393514688}
// 编译实例 RVAs: 0x17a80c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=3
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23896779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393514688,"raw_hex":"e8355d93fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5393514806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":3780,"parent_record_offset":3560,"ancestor_record_offset":3560,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":3752,"parent_record_offset":3560,"ancestor_record_offset":3560,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":4152,"parent_record_offset":3560,"ancestor_record_offset":3560,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":4100,"parent_record_offset":4072,"ancestor_record_offset":3560,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3560,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":3560,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":4000,"parent_record_offset":3976,"ancestor_record_offset":3560,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":3976,"parent_record_offset":3780,"ancestor_record_offset":3560,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3560,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":3560,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4048,"parent_record_offset":4024,"ancestor_record_offset":3560,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":3560,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":3560,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4248,"parent_record_offset":4200,"ancestor_record_offset":3560,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":4640,"parent_record_offset":4200,"ancestor_record_offset":3560,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":3560,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":4348,"parent_record_offset":4248,"ancestor_record_offset":3560,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":4316,"parent_record_offset":4248,"ancestor_record_offset":3560,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2748,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":3560,"inlinee":21684,"source_line":84,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":3908,"parent_record_offset":3852,"ancestor_record_offset":3560,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":4024,"parent_record_offset":4000,"ancestor_record_offset":3560,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2748,"record_offset":256,"rva":24804720,"va":5393513840}
// 编译实例 RVAs: 0x17a7d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23895932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393513840,"raw_hex":"e8846093fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5393513959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":480,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":452,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":996,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":256,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":256,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":528,"parent_record_offset":504,"ancestor_record_offset":256,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":256,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":676,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":256,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":256,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":256,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":256,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":772,"parent_record_offset":748,"ancestor_record_offset":256,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":1092,"parent_record_offset":1044,"ancestor_record_offset":256,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":1484,"parent_record_offset":1044,"ancestor_record_offset":256,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":576,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":1192,"parent_record_offset":1092,"ancestor_record_offset":256,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":1160,"parent_record_offset":1092,"ancestor_record_offset":256,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$22::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2748,"record_offset":504,"parent_record_offset":480,"ancestor_record_offset":256,"inlinee":21684,"source_line":84,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":608,"parent_record_offset":552,"ancestor_record_offset":256,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":724,"parent_record_offset":700,"ancestor_record_offset":256,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":26912,"rva":7086560,"va":5375795680}
// 编译实例 RVAs: 0x6c21e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6177772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375795680,"raw_hex":"e814bca1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375795799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27164,"parent_record_offset":26912,"ancestor_record_offset":26912,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":27136,"parent_record_offset":26912,"ancestor_record_offset":26912,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":27740,"parent_record_offset":26912,"ancestor_record_offset":26912,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27412,"parent_record_offset":27388,"ancestor_record_offset":26912,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27236,"parent_record_offset":27212,"ancestor_record_offset":26912,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27684,"parent_record_offset":27656,"ancestor_record_offset":26912,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27436,"parent_record_offset":27412,"ancestor_record_offset":26912,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27212,"parent_record_offset":27188,"ancestor_record_offset":26912,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27388,"parent_record_offset":27360,"ancestor_record_offset":26912,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27584,"parent_record_offset":27560,"ancestor_record_offset":26912,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":27560,"parent_record_offset":27360,"ancestor_record_offset":26912,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":26912,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27316,"parent_record_offset":27292,"ancestor_record_offset":26912,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27632,"parent_record_offset":27608,"ancestor_record_offset":26912,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27788,"parent_record_offset":27764,"ancestor_record_offset":26912,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27656,"parent_record_offset":27632,"ancestor_record_offset":26912,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":27836,"parent_record_offset":27788,"ancestor_record_offset":26912,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":28228,"parent_record_offset":27788,"ancestor_record_offset":26912,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":27460,"parent_record_offset":27436,"ancestor_record_offset":26912,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":27936,"parent_record_offset":27836,"ancestor_record_offset":26912,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":27260,"parent_record_offset":27236,"ancestor_record_offset":26912,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":27904,"parent_record_offset":27836,"ancestor_record_offset":26912,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$50::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":495,"record_offset":27360,"parent_record_offset":27164,"ancestor_record_offset":26912,"inlinee":16342,"source_line":567,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::resources::ui_state::impl$11::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/resources/ui_state.rs
//     evidence={"module":495,"record_offset":27188,"parent_record_offset":27164,"ancestor_record_offset":26912,"inlinee":27810,"source_line":10,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":27292,"parent_record_offset":27236,"ancestor_record_offset":26912,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":27608,"parent_record_offset":27584,"ancestor_record_offset":26912,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":15752,"rva":907760,"va":5369616880}
// 编译实例 RVAs: 0xdd9f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616880,"raw_hex":"e806040000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369616997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":15968,"parent_record_offset":15752,"ancestor_record_offset":15752,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":15940,"parent_record_offset":15752,"ancestor_record_offset":15752,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":16364,"parent_record_offset":15752,"ancestor_record_offset":15752,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":16016,"parent_record_offset":15992,"ancestor_record_offset":15752,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":16208,"parent_record_offset":16184,"ancestor_record_offset":15752,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":16040,"parent_record_offset":16016,"ancestor_record_offset":15752,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":16232,"parent_record_offset":16208,"ancestor_record_offset":15752,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":15992,"parent_record_offset":15968,"ancestor_record_offset":15752,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":16184,"parent_record_offset":16164,"ancestor_record_offset":15752,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":15752,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":15752,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":15752,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":16460,"parent_record_offset":16412,"ancestor_record_offset":15752,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":16852,"parent_record_offset":16412,"ancestor_record_offset":15752,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15752,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":15752,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":16560,"parent_record_offset":16460,"ancestor_record_offset":15752,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":16528,"parent_record_offset":16460,"ancestor_record_offset":15752,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":16096,"parent_record_offset":16040,"ancestor_record_offset":15752,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::gui::gui_tod::do_time_of_day_screen::impl$2::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/gui/gui_tod.rs
//     evidence={"module":72,"record_offset":16164,"parent_record_offset":15968,"ancestor_record_offset":15752,"inlinee":8471,"source_line":93,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":18396,"rva":7084592,"va":5375793712}
// 编译实例 RVAs: 0x6c1a30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=7
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6175811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375793712,"raw_hex":"e8bdc3a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375793838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18620,"parent_record_offset":18396,"ancestor_record_offset":18396,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":18592,"parent_record_offset":18396,"ancestor_record_offset":18396,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":19824,"parent_record_offset":18396,"ancestor_record_offset":18396,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":495,"record_offset":19012,"parent_record_offset":18840,"ancestor_record_offset":18396,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18840,"parent_record_offset":18668,"ancestor_record_offset":18396,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18696,"parent_record_offset":18668,"ancestor_record_offset":18396,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":19564,"parent_record_offset":19540,"ancestor_record_offset":18396,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18720,"parent_record_offset":18696,"ancestor_record_offset":18396,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":19540,"parent_record_offset":19508,"ancestor_record_offset":18396,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":19152,"parent_record_offset":19036,"ancestor_record_offset":18396,"inlinee":23504,"source_line":811,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":18396,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":18644,"parent_record_offset":18620,"ancestor_record_offset":18396,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":19848,"parent_record_offset":19824,"ancestor_record_offset":18396,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":18800,"parent_record_offset":18776,"ancestor_record_offset":18396,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":19872,"parent_record_offset":19848,"ancestor_record_offset":18396,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":19920,"parent_record_offset":19872,"ancestor_record_offset":18396,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":20312,"parent_record_offset":19872,"ancestor_record_offset":18396,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":18744,"parent_record_offset":18720,"ancestor_record_offset":18396,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":18396,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":20020,"parent_record_offset":19920,"ancestor_record_offset":18396,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":19988,"parent_record_offset":19920,"ancestor_record_offset":18396,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":18396,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":18920,"parent_record_offset":18872,"ancestor_record_offset":18396,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":18952,"parent_record_offset":18872,"ancestor_record_offset":18396,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":18872,"parent_record_offset":18840,"ancestor_record_offset":18396,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$76::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec3.rs
//     evidence={"module":495,"record_offset":19508,"parent_record_offset":19396,"ancestor_record_offset":18396,"inlinee":22469,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":18776,"parent_record_offset":18720,"ancestor_record_offset":18396,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":28800,"rva":7087024,"va":5375796144}
// 编译实例 RVAs: 0x6c23b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6178242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375796144,"raw_hex":"e83ebaa1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375796269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14051eae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1718670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375796144,"raw_hex":"e872c6e5ff","size":5,"soft_fail":false,"target_va":5374077664,"va":5375796329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f71040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9104384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375796144,"raw_hex":"e800ec8a00","size":5,"soft_fail":false,"target_va":5384900672,"va":5375796283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":29056,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":29028,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":29492,"parent_record_offset":28800,"ancestor_record_offset":28800,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":29132,"parent_record_offset":29104,"ancestor_record_offset":28800,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":28800,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":29104,"parent_record_offset":29080,"ancestor_record_offset":28800,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":28800,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":29516,"parent_record_offset":29492,"ancestor_record_offset":28800,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":29236,"parent_record_offset":29212,"ancestor_record_offset":28800,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":29540,"parent_record_offset":29516,"ancestor_record_offset":28800,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":29588,"parent_record_offset":29540,"ancestor_record_offset":28800,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":29980,"parent_record_offset":29540,"ancestor_record_offset":28800,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":29180,"parent_record_offset":29156,"ancestor_record_offset":28800,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":29688,"parent_record_offset":29588,"ancestor_record_offset":28800,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":29656,"parent_record_offset":29588,"ancestor_record_offset":28800,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":29212,"parent_record_offset":29156,"ancestor_record_offset":28800,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,i32,usize,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":3764,"rva":28176384,"va":5396885504}
// 编译实例 RVAs: 0x1adf000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=5
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27267583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396885504,"raw_hex":"e801ee5ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396885610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$25::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":3940,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":51807,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":3912,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":4596,"parent_record_offset":3764,"ancestor_record_offset":3764,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":3764,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3764,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3764,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":3764,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4448,"parent_record_offset":3940,"ancestor_record_offset":3764,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4304,"parent_record_offset":3940,"ancestor_record_offset":3764,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":4132,"parent_record_offset":3940,"ancestor_record_offset":3764,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3764,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3764,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4620,"parent_record_offset":4596,"ancestor_record_offset":3764,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3764,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4036,"parent_record_offset":4012,"ancestor_record_offset":3764,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":3764,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3764,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":4692,"parent_record_offset":4644,"ancestor_record_offset":3764,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":5084,"parent_record_offset":4644,"ancestor_record_offset":3764,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":4352,"parent_record_offset":4328,"ancestor_record_offset":3764,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":4204,"parent_record_offset":4180,"ancestor_record_offset":3764,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":4792,"parent_record_offset":4692,"ancestor_record_offset":3764,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":4760,"parent_record_offset":4692,"ancestor_record_offset":3764,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":4236,"parent_record_offset":4180,"ancestor_record_offset":3764,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3764,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":3056,"rva":7081872,"va":5375790992}
// 编译实例 RVAs: 0x6c0f90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=3
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6173071","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375790992,"raw_hex":"e871cea1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375791098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3324,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":3296,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":3748,"parent_record_offset":3056,"ancestor_record_offset":3056,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":3056,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3612,"parent_record_offset":3588,"ancestor_record_offset":3056,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3056,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3056,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":3056,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":3348,"parent_record_offset":3324,"ancestor_record_offset":3056,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3772,"parent_record_offset":3748,"ancestor_record_offset":3056,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3692,"parent_record_offset":3668,"ancestor_record_offset":3056,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3056,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":3056,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3056,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":3844,"parent_record_offset":3796,"ancestor_record_offset":3056,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":4236,"parent_record_offset":3796,"ancestor_record_offset":3056,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":3056,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":3944,"parent_record_offset":3844,"ancestor_record_offset":3056,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":3912,"parent_record_offset":3844,"ancestor_record_offset":3056,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$15::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/shader_uri.rs
//     evidence={"module":495,"record_offset":3540,"parent_record_offset":3516,"ancestor_record_offset":3056,"inlinee":27792,"source_line":100,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":3668,"parent_record_offset":3612,"ancestor_record_offset":3056,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":3056,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,ref$<tuple$<> > > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":18880,"rva":908528,"va":5369617648}
// 编译实例 RVAs: 0xddcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369617648,"raw_hex":"e815010000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369617750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":19056,"parent_record_offset":18880,"ancestor_record_offset":18880,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":19032,"parent_record_offset":18880,"ancestor_record_offset":18880,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":19252,"parent_record_offset":18880,"ancestor_record_offset":18880,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":18880,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":18880,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":18880,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":18880,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19152,"parent_record_offset":19128,"ancestor_record_offset":18880,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19300,"parent_record_offset":19276,"ancestor_record_offset":18880,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":18880,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19348,"parent_record_offset":19300,"ancestor_record_offset":18880,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":19740,"parent_record_offset":19300,"ancestor_record_offset":18880,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":19448,"parent_record_offset":19348,"ancestor_record_offset":18880,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":19416,"parent_record_offset":19348,"ancestor_record_offset":18880,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":19128,"parent_record_offset":19104,"ancestor_record_offset":18880,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,u32,country_core::resources::walls::WallId> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":9004,"rva":28177264,"va":5396886384}
// 编译实例 RVAs: 0x1adf370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27268463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396886384,"raw_hex":"e891ea5ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396886490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9212,"parent_record_offset":9004,"ancestor_record_offset":9004,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":9184,"parent_record_offset":9004,"ancestor_record_offset":9004,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":9724,"parent_record_offset":9004,"ancestor_record_offset":9004,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9428,"parent_record_offset":9404,"ancestor_record_offset":9004,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9356,"parent_record_offset":9332,"ancestor_record_offset":9004,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9596,"parent_record_offset":9572,"ancestor_record_offset":9004,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9404,"parent_record_offset":9212,"ancestor_record_offset":9004,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9572,"parent_record_offset":9548,"ancestor_record_offset":9004,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":9004,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":9004,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9748,"parent_record_offset":9724,"ancestor_record_offset":9004,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9508,"parent_record_offset":9484,"ancestor_record_offset":9004,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":9004,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9772,"parent_record_offset":9748,"ancestor_record_offset":9004,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":9004,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":9820,"parent_record_offset":9772,"ancestor_record_offset":9004,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":10212,"parent_record_offset":9772,"ancestor_record_offset":9004,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":9620,"parent_record_offset":9596,"ancestor_record_offset":9004,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":9452,"parent_record_offset":9428,"ancestor_record_offset":9004,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":9920,"parent_record_offset":9820,"ancestor_record_offset":9004,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":9888,"parent_record_offset":9820,"ancestor_record_offset":9004,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$6::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":3529,"record_offset":9548,"parent_record_offset":9212,"ancestor_record_offset":9004,"inlinee":25074,"source_line":65,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":9484,"parent_record_offset":9428,"ancestor_record_offset":9004,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":9004,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,usize,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":12400,"rva":28178128,"va":5396887248}
// 编译实例 RVAs: 0x1adf6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27269327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396887248,"raw_hex":"e831e75ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396887354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12572,"parent_record_offset":12400,"ancestor_record_offset":12400,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":12544,"parent_record_offset":12400,"ancestor_record_offset":12400,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":13056,"parent_record_offset":12400,"ancestor_record_offset":12400,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12400,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12716,"parent_record_offset":12692,"ancestor_record_offset":12400,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12788,"parent_record_offset":12764,"ancestor_record_offset":12400,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12908,"parent_record_offset":12572,"ancestor_record_offset":12400,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12764,"parent_record_offset":12572,"ancestor_record_offset":12400,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12620,"parent_record_offset":12596,"ancestor_record_offset":12400,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":12596,"parent_record_offset":12572,"ancestor_record_offset":12400,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":13080,"parent_record_offset":13056,"ancestor_record_offset":12400,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":12400,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":12668,"parent_record_offset":12644,"ancestor_record_offset":12400,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":13104,"parent_record_offset":13080,"ancestor_record_offset":12400,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":12692,"parent_record_offset":12668,"ancestor_record_offset":12400,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":13152,"parent_record_offset":13104,"ancestor_record_offset":12400,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":13544,"parent_record_offset":13104,"ancestor_record_offset":12400,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":12812,"parent_record_offset":12788,"ancestor_record_offset":12400,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":12400,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":13252,"parent_record_offset":13152,"ancestor_record_offset":12400,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":13220,"parent_record_offset":13152,"ancestor_record_offset":12400,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":12844,"parent_record_offset":12788,"ancestor_record_offset":12400,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":12644,"parent_record_offset":12620,"ancestor_record_offset":12400,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<ref$<str$>,usize> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":5656,"rva":28176848,"va":5396885968}
// 编译实例 RVAs: 0x1adf1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=3
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27268047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396885968,"raw_hex":"e831ec5ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396886074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":5824,"parent_record_offset":5656,"ancestor_record_offset":5656,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":5796,"parent_record_offset":5656,"ancestor_record_offset":5656,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":6164,"parent_record_offset":5656,"ancestor_record_offset":5656,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":5656,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5656,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":6016,"parent_record_offset":5824,"ancestor_record_offset":5656,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":5872,"parent_record_offset":5848,"ancestor_record_offset":5656,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":5656,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":6188,"parent_record_offset":6164,"ancestor_record_offset":5656,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":6120,"parent_record_offset":6096,"ancestor_record_offset":5656,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":5656,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":5656,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":5656,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":6260,"parent_record_offset":6212,"ancestor_record_offset":5656,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":6652,"parent_record_offset":6212,"ancestor_record_offset":5656,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":5656,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":6360,"parent_record_offset":6260,"ancestor_record_offset":5656,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":6328,"parent_record_offset":6260,"ancestor_record_offset":5656,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":6096,"parent_record_offset":6040,"ancestor_record_offset":5656,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":5656,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<system_clutter::clutter::ClutterRegionId,i32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":14116,"rva":10834704,"va":5379543824}
// 编译实例 RVAs: 0xa55310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9925915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379543824,"raw_hex":"e8e58a68ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5379543942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14312,"parent_record_offset":14116,"ancestor_record_offset":14116,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":14284,"parent_record_offset":14116,"ancestor_record_offset":14116,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":14684,"parent_record_offset":14116,"ancestor_record_offset":14116,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14532,"parent_record_offset":14508,"ancestor_record_offset":14116,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":14116,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14384,"parent_record_offset":14360,"ancestor_record_offset":14116,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14360,"parent_record_offset":14336,"ancestor_record_offset":14116,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":14508,"parent_record_offset":14312,"ancestor_record_offset":14116,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":14116,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":14116,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":14116,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":14780,"parent_record_offset":14732,"ancestor_record_offset":14116,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":15172,"parent_record_offset":14732,"ancestor_record_offset":14116,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":14116,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":14880,"parent_record_offset":14780,"ancestor_record_offset":14116,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":14408,"parent_record_offset":14384,"ancestor_record_offset":14116,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":14848,"parent_record_offset":14780,"ancestor_record_offset":14116,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":14440,"parent_record_offset":14384,"ancestor_record_offset":14116,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] system_clutter::clutter::impl$24::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/clutter/src/clutter/mod.rs
//     evidence={"module":3529,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":14116,"inlinee":106868,"source_line":331,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<toolbar::mei::core::GlobalId,u32> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1016,"record_offset":244,"rva":10834704,"va":5379543824}
// 编译实例 RVAs: 0xa55310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9925915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379543824,"raw_hex":"e8e58a68ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5379543942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1016,"record_offset":428,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1016,"record_offset":400,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1016,"record_offset":772,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1016,"record_offset":648,"parent_record_offset":624,"ancestor_record_offset":244,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1016,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":244,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1016,"record_offset":624,"parent_record_offset":428,"ancestor_record_offset":244,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1016,"record_offset":476,"parent_record_offset":452,"ancestor_record_offset":244,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1016,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":244,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1016,"record_offset":580,"parent_record_offset":556,"ancestor_record_offset":244,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1016,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":244,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1016,"record_offset":868,"parent_record_offset":820,"ancestor_record_offset":244,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1016,"record_offset":1260,"parent_record_offset":820,"ancestor_record_offset":244,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1016,"record_offset":672,"parent_record_offset":648,"ancestor_record_offset":244,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1016,"record_offset":968,"parent_record_offset":868,"ancestor_record_offset":244,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1016,"record_offset":524,"parent_record_offset":500,"ancestor_record_offset":244,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1016,"record_offset":936,"parent_record_offset":868,"ancestor_record_offset":244,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1016,"record_offset":556,"parent_record_offset":500,"ancestor_record_offset":244,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$45::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1016,"record_offset":452,"parent_record_offset":428,"ancestor_record_offset":244,"inlinee":8429,"source_line":58,"archive":"_pdb/A/modules/01016.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u32,usize,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":15744,"rva":28178576,"va":5396887696}
// 编译实例 RVAs: 0x1adf890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27269786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396887696,"raw_hex":"e866e55ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396887813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":15920,"parent_record_offset":15744,"ancestor_record_offset":15744,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":15892,"parent_record_offset":15744,"ancestor_record_offset":15744,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":16408,"parent_record_offset":15744,"ancestor_record_offset":15744,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":15968,"parent_record_offset":15944,"ancestor_record_offset":15744,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":15744,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":15744,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":15944,"parent_record_offset":15920,"ancestor_record_offset":15744,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":16088,"parent_record_offset":15920,"ancestor_record_offset":15744,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":16256,"parent_record_offset":16232,"ancestor_record_offset":15744,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":16232,"parent_record_offset":15920,"ancestor_record_offset":15744,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16432,"parent_record_offset":16408,"ancestor_record_offset":15744,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":15744,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16304,"parent_record_offset":16280,"ancestor_record_offset":15744,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16456,"parent_record_offset":16432,"ancestor_record_offset":15744,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":15744,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16504,"parent_record_offset":16456,"ancestor_record_offset":15744,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":16896,"parent_record_offset":16456,"ancestor_record_offset":15744,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":16136,"parent_record_offset":16112,"ancestor_record_offset":15744,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":15992,"parent_record_offset":15968,"ancestor_record_offset":15744,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":16604,"parent_record_offset":16504,"ancestor_record_offset":15744,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":16572,"parent_record_offset":16504,"ancestor_record_offset":15744,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":16024,"parent_record_offset":15968,"ancestor_record_offset":15744,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":16280,"parent_record_offset":16256,"ancestor_record_offset":15744,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,core::any::TypeId> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":12636,"rva":906960,"va":5369616080}
// 编译实例 RVAs: 0xdd6d0, 0x1dc7300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616080,"raw_hex":"e824070000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369616199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141991380
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4415468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399933696,"raw_hex":"e814a0bcff","size":5,"soft_fail":false,"target_va":5395518336,"va":5399933799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":12812,"parent_record_offset":12636,"ancestor_record_offset":12636,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":12784,"parent_record_offset":12636,"ancestor_record_offset":12636,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":13156,"parent_record_offset":12636,"ancestor_record_offset":12636,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$9::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":72,"record_offset":12980,"parent_record_offset":12812,"ancestor_record_offset":12636,"inlinee":6652,"source_line":652,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":12860,"parent_record_offset":12836,"ancestor_record_offset":12636,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":12636,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":13180,"parent_record_offset":13156,"ancestor_record_offset":12636,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":12636,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":13204,"parent_record_offset":13180,"ancestor_record_offset":12636,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":13252,"parent_record_offset":13204,"ancestor_record_offset":12636,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":13644,"parent_record_offset":13204,"ancestor_record_offset":12636,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":13352,"parent_record_offset":13252,"ancestor_record_offset":12636,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":12636,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":13320,"parent_record_offset":13252,"ancestor_record_offset":12636,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":12916,"parent_record_offset":12860,"ancestor_record_offset":12636,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":6732,"rva":7082752,"va":5375791872}
// 编译实例 RVAs: 0x6c1300
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6173964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375791872,"raw_hex":"e8f4caa1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375791991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f0e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-853290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375791872,"raw_hex":"e8d6faf2ff","size":5,"soft_fail":false,"target_va":5374938752,"va":5375792037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":6940,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":6912,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":7316,"parent_record_offset":6732,"ancestor_record_offset":6732,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":7160,"parent_record_offset":7132,"ancestor_record_offset":6732,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6732,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":6732,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6732,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":7132,"parent_record_offset":7108,"ancestor_record_offset":6732,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":7108,"parent_record_offset":6940,"ancestor_record_offset":6732,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":7340,"parent_record_offset":7316,"ancestor_record_offset":6732,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":6732,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":6732,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":7412,"parent_record_offset":7364,"ancestor_record_offset":6732,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":7804,"parent_record_offset":7364,"ancestor_record_offset":6732,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":6732,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":7512,"parent_record_offset":7412,"ancestor_record_offset":6732,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":6732,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":7480,"parent_record_offset":7412,"ancestor_record_offset":6732,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":7044,"parent_record_offset":6988,"ancestor_record_offset":6732,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":12028,"rva":7084080,"va":5375793200}
// 编译实例 RVAs: 0x6c1830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6175295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375793200,"raw_hex":"e8c1c5a1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375793322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12224,"parent_record_offset":12028,"ancestor_record_offset":12028,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":12196,"parent_record_offset":12028,"ancestor_record_offset":12028,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":12924,"parent_record_offset":12028,"ancestor_record_offset":12028,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12588,"parent_record_offset":12416,"ancestor_record_offset":12028,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12444,"parent_record_offset":12416,"ancestor_record_offset":12028,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":12028,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":12028,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12468,"parent_record_offset":12444,"ancestor_record_offset":12028,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12728,"parent_record_offset":12700,"ancestor_record_offset":12028,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":12028,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":12028,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":12392,"parent_record_offset":12224,"ancestor_record_offset":12028,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":12028,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":12028,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":12972,"parent_record_offset":12948,"ancestor_record_offset":12028,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":13020,"parent_record_offset":12972,"ancestor_record_offset":12028,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":13412,"parent_record_offset":12972,"ancestor_record_offset":12028,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":12492,"parent_record_offset":12468,"ancestor_record_offset":12028,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":12776,"parent_record_offset":12752,"ancestor_record_offset":12028,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":13120,"parent_record_offset":13020,"ancestor_record_offset":12028,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":12028,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":13088,"parent_record_offset":13020,"ancestor_record_offset":12028,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":12664,"parent_record_offset":12620,"ancestor_record_offset":12028,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":12620,"parent_record_offset":12588,"ancestor_record_offset":12028,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::impl$74::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":495,"record_offset":12700,"parent_record_offset":12588,"ancestor_record_offset":12028,"inlinee":25441,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":12328,"parent_record_offset":12272,"ancestor_record_offset":12028,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":495,"record_offset":24864,"rva":7086032,"va":5375795152}
// 编译实例 RVAs: 0x6c1fd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=5
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6177246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375795152,"raw_hex":"e822bea1ff","size":5,"soft_fail":false,"target_va":5369618032,"va":5375795273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25060,"parent_record_offset":24864,"ancestor_record_offset":24864,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":25032,"parent_record_offset":24864,"ancestor_record_offset":24864,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":25852,"parent_record_offset":24864,"ancestor_record_offset":24864,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hash::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25424,"parent_record_offset":25252,"ancestor_record_offset":24864,"inlinee":22467,"source_line":238,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25280,"parent_record_offset":25252,"ancestor_record_offset":24864,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25592,"parent_record_offset":25568,"ancestor_record_offset":24864,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25108,"parent_record_offset":25084,"ancestor_record_offset":24864,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25304,"parent_record_offset":25280,"ancestor_record_offset":24864,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25568,"parent_record_offset":25536,"ancestor_record_offset":24864,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25084,"parent_record_offset":25060,"ancestor_record_offset":24864,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25252,"parent_record_offset":25228,"ancestor_record_offset":24864,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":495,"record_offset":25228,"parent_record_offset":25060,"ancestor_record_offset":24864,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":24864,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":25188,"parent_record_offset":25164,"ancestor_record_offset":24864,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":24864,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":25948,"parent_record_offset":25900,"ancestor_record_offset":24864,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":495,"record_offset":26340,"parent_record_offset":25900,"ancestor_record_offset":24864,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":25328,"parent_record_offset":25304,"ancestor_record_offset":24864,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":25616,"parent_record_offset":25592,"ancestor_record_offset":24864,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":26048,"parent_record_offset":25948,"ancestor_record_offset":24864,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":25132,"parent_record_offset":25108,"ancestor_record_offset":24864,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":495,"record_offset":26016,"parent_record_offset":25948,"ancestor_record_offset":24864,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":25500,"parent_record_offset":25456,"ancestor_record_offset":24864,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":25456,"parent_record_offset":25424,"ancestor_record_offset":24864,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec3::impl$76::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec3.rs
//     evidence={"module":495,"record_offset":25536,"parent_record_offset":25424,"ancestor_record_offset":24864,"inlinee":22469,"source_line":18,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":495,"record_offset":25164,"parent_record_offset":25108,"ancestor_record_offset":24864,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<u64,u64> >
// 记录形式: compiled_procedure; instances=3
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":72,"record_offset":14216,"rva":907360,"va":5369616480}
// 编译实例 RVAs: 0xdd860, 0x1dc7190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369616480,"raw_hex":"e894050000","size":5,"soft_fail":false,"target_va":5369618032,"va":5369616599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141991380
//     owner=unresolved_namespace; count=2
//     target_aliases=17 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4415100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399933328,"raw_hex":"e884a1bcff","size":5,"soft_fail":false,"target_va":5395518336,"va":5399933431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":14376,"parent_record_offset":14216,"ancestor_record_offset":14216,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":14348,"parent_record_offset":14216,"ancestor_record_offset":14216,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":14692,"parent_record_offset":14216,"ancestor_record_offset":14216,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":14424,"parent_record_offset":14400,"ancestor_record_offset":14216,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":72,"record_offset":14400,"parent_record_offset":14376,"ancestor_record_offset":14216,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":14216,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":14504,"parent_record_offset":14480,"ancestor_record_offset":14216,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":14740,"parent_record_offset":14716,"ancestor_record_offset":14216,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":14788,"parent_record_offset":14740,"ancestor_record_offset":14216,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":72,"record_offset":15180,"parent_record_offset":14740,"ancestor_record_offset":14216,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":14888,"parent_record_offset":14788,"ancestor_record_offset":14216,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":14216,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":72,"record_offset":14856,"parent_record_offset":14788,"ancestor_record_offset":14216,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":72,"record_offset":14480,"parent_record_offset":14424,"ancestor_record_offset":14216,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/00072.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<usize,u8> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2748,"record_offset":2056,"rva":24805168,"va":5393514288}
// 编译实例 RVAs: 0x17a7f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=2
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23896380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393514288,"raw_hex":"e8c45e93fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5393514407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":2216,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":2188,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":2500,"parent_record_offset":2056,"ancestor_record_offset":2056,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":2408,"parent_record_offset":2384,"ancestor_record_offset":2056,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":2056,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":2056,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":2748,"record_offset":2384,"parent_record_offset":2216,"ancestor_record_offset":2056,"inlinee":17875,"source_line":806,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":2524,"parent_record_offset":2500,"ancestor_record_offset":2056,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":2056,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":2548,"parent_record_offset":2524,"ancestor_record_offset":2056,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":2596,"parent_record_offset":2548,"ancestor_record_offset":2056,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":2748,"record_offset":2988,"parent_record_offset":2548,"ancestor_record_offset":2056,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":2056,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":2696,"parent_record_offset":2596,"ancestor_record_offset":2056,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":2748,"record_offset":2664,"parent_record_offset":2596,"ancestor_record_offset":2056,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":2748,"record_offset":2320,"parent_record_offset":2264,"ancestor_record_offset":2056,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/02748.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::calculate_hash<tuple$<usize,usize,ref$<str$> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 504
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3529,"record_offset":2040,"rva":28175936,"va":5396885056}
// 编译实例 RVAs: 0x1adee40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27267147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396885056,"raw_hex":"e8b5ef5ffe","size":5,"soft_fail":false,"target_va":5369618032,"va":5396885174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$24::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2216,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":27794,"source_line":887,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":2188,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":2704,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2652,"parent_record_offset":2624,"ancestor_record_offset":2040,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":2040,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":2040,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":2040,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3529,"record_offset":2528,"parent_record_offset":2216,"ancestor_record_offset":2040,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2728,"parent_record_offset":2704,"ancestor_record_offset":2040,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2344,"parent_record_offset":2320,"ancestor_record_offset":2040,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2040,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":2040,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2040,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":2800,"parent_record_offset":2752,"ancestor_record_offset":2040,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3529,"record_offset":3192,"parent_record_offset":2752,"ancestor_record_offset":2040,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":2040,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":2900,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3529,"record_offset":2868,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":2320,"parent_record_offset":2264,"ancestor_record_offset":2040,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3529,"record_offset":2576,"parent_record_offset":2552,"ancestor_record_offset":2040,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::fit
// 记录形式: compiled_procedure; instances=1
// 源码行: 513
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":11900,"rva":8234720,"va":5376943840}
// 编译实例 RVAs: 0x7da6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::lerp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":11996,"parent_record_offset":11900,"ancestor_record_offset":11900,"inlinee":36057,"source_line":263,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_oetf
// 记录形式: inlinee_declaration; instances=1
// 源码行: 519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE8
// 首条位置: {"module":615,"inlinee":36072}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::srgb_eotf
// 记录形式: inlinee_declaration; instances=1
// 源码行: 527
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE7
// 首条位置: {"module":615,"inlinee":36071}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color
// 记录形式: compiled_procedure; instances=1
// 源码行: 538
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":615,"record_offset":12028,"rva":8234752,"va":5376943872}
// 编译实例 RVAs: 0x7da700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::scale_u32_packed_srgba_color::scale_linear
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":615,"record_offset":12148,"parent_record_offset":12028,"ancestor_record_offset":12028,"inlinee":36068,"source_line":540,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":12168,"parent_record_offset":12148,"ancestor_record_offset":12028,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color::scale_linear
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 540
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE4
// 首条位置: {"module":615,"inlinee":36068}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":615,"record_offset":12168,"parent_record_offset":12148,"ancestor_record_offset":12148,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00615.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 记录形式: inlinee_declaration; instances=1
// 源码行: 557
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE6
// 首条位置: {"module":615,"inlinee":36070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::scale_u32_packed_srgba_color_premultiplied::scale_linear
// 记录形式: inlinee_declaration; instances=1
// 源码行: 561
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8CE9
// 首条位置: {"module":615,"inlinee":36073}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::impl$9::iterator_slice_range
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 606
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE92D
// 首条位置: {"module":1230,"inlinee":59693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::slice_iter
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE92B
// 首条位置: {"module":1230,"inlinee":59691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::impl$9::iterator_slice_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1230,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":2200,"inlinee":59693,"source_line":606,"archive":"_pdb/A/modules/01230.jsonl.gz"}
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
// 完整结构及原始类型记录: lib.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::Side
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::firstperson_steps::FirstPersonViewAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>,bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>", "bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::decorator_audio::DecoratorAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::window::CursorMoved>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>,bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>)
// 候选参数类型: ["country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources", "bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>", "bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyRequests>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyRequestsPrevFrame>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::decorator_audio::PlayDecoratorSoundCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<glam::i32::ivec3::IVec3>
// 信息 ID: e39355740c61f3b52ecb7a40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["glam::i32::ivec3::IVec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0x53c3d3 -> 0x6c1680 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad59a1 -> 0x6c1680 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<array$<u32,2> > > >
// 信息 ID: c5e3cf2db5251c21caeb2c09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<u32,2>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<u32,2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<array$<u32,2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e48d0 -> 0x1adec70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e4973 -> 0x1adec70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<glam::u32::uvec3::UVec3> > >
// 信息 ID: a36e1e0b33ea97578f0e91fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_convex @ 0x589d41 -> 0x6c2570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<slice2$<u8> > >
// 信息 ID: ef855948af93850b29a51b44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::save::save_game::save_game @ 0x5dde51 -> 0x6c2760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<ref$<str$> >
// 信息 ID: 9de27ed9b3e01994287de702
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 158 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a6ed8 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a76f9 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$0::as_id @ 0x43e590 -> 0xddcf0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::label @ 0x44358f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::menu_button @ 0x44363f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::toggle_icon @ 0x443a37 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> > @ 0x51317d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mode_manager::mode_manager @ 0x5f21ba -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound @ 0x685b0d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > @ 0x685e9d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> @ 0x685fbd -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::MeiContext::safe_frame<ref$<str$> > @ 0xa5700c -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa573f5 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa57664 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa578d4 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::image<ref$<str$> > @ 0xa5b10f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<rhapsody::raw::texture::TextureViewDesc>
// 信息 ID: c2c0f41b2e0c39462c85fddb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["rhapsody::raw::texture::TextureViewDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset> @ 0x541209 -> 0x6c1ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > > @ 0x5ad2cf -> 0x6c1ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > > @ 0x64430f -> 0x6c1ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view @ 0x6606b0 -> 0x6c1ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>
// 信息 ID: f41b1a3951635970a9733829
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::calculate_hash -> ["tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f74c -> 0xddb80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<country_core::resources::walls::WallId,2>,usize> >
// 信息 ID: 8c38991d3e76c8af47493e0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["country_core::resources::walls::WallId", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<country_core::resources::walls::WallId,2>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<country_core::resources::walls::WallId,2>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitch_bricks @ 0x14e3c76 -> 0x1adf530 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType> >
// 信息 ID: 93e5a4eac8007d4bca8d0a81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] array$ -> ["u32", "2"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,2>", "array$<u32,2>", "country_core::resources::walls::wall_holes::HoleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,2>,array$<u32,2>,country_core::resources::walls::wall_holes::HoleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_holes::WallHoles::add @ 0x6287f8 -> 0x6c14a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_holes::InProgressHoleStorage::add @ 0x629a22 -> 0x6c14a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<array$<u32,3>,glam::u32::uvec3::UVec3> >
// 信息 ID: 7fbfe58e6c6fa743024a36f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["array$<u32,3>", "glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<array$<u32,3>,glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_cuboid @ 0x58afb5 -> 0x6c1130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::WallId,usize> >
// 信息 ID: 4d9cd32c2145742500c64966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["country_core::resources::walls::WallId", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::WallId,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 239 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d8fb -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da2b -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dee0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e012 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e4d0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e602 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb10 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec49 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f0a0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f1d9 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f62d -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f761 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fad3 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fc05 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x200ad -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201da -> 0xdd860 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
// 信息 ID: 67bf54c756104c180f10f52f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c62801 -> 0x17a80c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c62987 -> 0x17a80c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
// 信息 ID: 2c780866ca21900ac38dab6a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::resources::walls::decorator::DecoratorId", "ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf5c6d -> 0x17a7d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c5561a -> 0x17a7d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::generate_cottage_wall_windows @ 0x1c62681 -> 0x17a7d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > > >
// 信息 ID: 45a01db215a37d92cbdb012f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<str$>"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<str$> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["country_core::systems::ui::resources::ui_state::WidgetId", "enum2$<core::option::Option<ref$<str$> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<country_core::systems::ui::resources::ui_state::WidgetId,enum2$<core::option::Option<ref$<str$> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::signifier_create::signifier_create @ 0xcf002b -> 0x6c21e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete @ 0x1816c5b -> 0x6c21e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType> >
// 信息 ID: ce6da9a9a8eeaa071c76d717
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["i32", "tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<i32,tiny_glade::gui::gui_tod::do_time_of_day_screen::CircleType>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f95f -> 0xdd9f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x3402f1 -> 0xdd9f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: 993f1359e466371f7e6768f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["u32", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<u32,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<u32,3> >"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<slice2$<array$<u32,3> > >", "ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<slice2$<array$<u32,3> > >,ref$<slice2$<glam::u32::uvec3::UVec3> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x589e4d -> 0x6c1a30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > > >
// 信息 ID: 3fe31967fb57f5d1df9d7936
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["utils::geometry::intersect::Triangle"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<utils::geometry::intersect::Triangle>"]; 字段布局 = unknown。
// [类型关系] array$ -> ["glam::u32::uvec3::UVec3", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<glam::u32::uvec3::UVec3,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<glam::u32::uvec3::UVec3,3> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<slice2$<utils::geometry::intersect::Triangle> >", "ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<slice2$<utils::geometry::intersect::Triangle> >,ref$<slice2$<array$<glam::u32::uvec3::UVec3,3> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x58a7b5 -> 0x6c23b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,i32,usize,u32> >
// 信息 ID: f0d0c080895ce21d57afdef8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "i32", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,i32,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows @ 0x1acf322 -> 0x1adf000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> > >
// 信息 ID: 27ff07499be8e0d8e55af056
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 11 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$3::source_hash @ 0x54e4c3 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$7::source_hash @ 0x54e943 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::dof::impl$11::source_hash @ 0x54edc3 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketFlowers> @ 0xdcbcc2 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketWoodenPlank> @ 0xdcc5e2 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketBush> @ 0xdccf02 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0xdcea42 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0xdcf362 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketRoofTile> @ 0xdcfc82 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::NaniCullAndBucketMain> @ 0xdd05a2 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0xdd0ec2 -> 0x6c0f90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,ref$<tuple$<> > > >
// 信息 ID: 304ba311196538b29ceee10d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] ref$ -> ["tuple$<>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "ref$<tuple$<> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,ref$<tuple$<> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 158 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a6ed8 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a76f9 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::impl$0::as_id @ 0x43e590 -> 0xddcf0 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::label @ 0x44358f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::menu_button @ 0x44363f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$8::toggle_icon @ 0x443a37 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> > @ 0x51317d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mode_manager::mode_manager @ 0x5f21ba -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound @ 0x685b0d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > | country_core::systems::ui::resources::ui_state::UiState::add<ref$<str$> > @ 0x685e9d -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String> @ 0x685fbd -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::MeiContext::safe_frame<ref$<str$> > @ 0xa5700c -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa573f5 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa57664 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::splash::ui::closure_env$0> @ 0xa578d4 -> 0xddcf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$0::image<ref$<str$> > @ 0xa5b10f -> 0xddcf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,u32,country_core::resources::walls::WallId> >
// 信息 ID: 4d0c3867f9c73b3790184704
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "u32", "country_core::resources::walls::WallId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,u32,country_core::resources::walls::WallId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_clutter_around_gates::add_clutter_around_gates @ 0x1aca911 -> 0x1adf370 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_clutter_around_gates::add_clutter_around_gates @ 0x1acaaa1 -> 0x1adf370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize,u32> >
// 信息 ID: af10648f0152c4a3b61f8b31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows @ 0x1acf5eb -> 0x1adf6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::add_clutter_around_windows::add_clutter_around_windows @ 0x1acf7c0 -> 0x1adf6d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<ref$<str$>,usize> >
// 信息 ID: b49405f1fa311d6e6d9c545b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["ref$<str$>", "usize"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<ref$<str$>,usize>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_preplaced_clutter::add_preplaced_clutter @ 0x14e279a -> 0x1adf1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<system_clutter::clutter::ClutterRegionId,i32> >
// 信息 ID: 15d1ea97a90c49b449da0903
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["system_clutter::clutter::ClutterRegionId", "i32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<system_clutter::clutter::ClutterRegionId,i32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x43ae5d -> 0xa55310 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad7020 -> 0xa55310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<toolbar::mei::core::GlobalId,u32> >
// 信息 ID: 081d9393196eecddb3fe6140
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["toolbar::mei::core::GlobalId", "u32"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<toolbar::mei::core::GlobalId,u32>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::decorate::impl$0::decorate_hovered @ 0x43ae5d -> 0xa55310 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad7020 -> 0xa55310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u32,usize,ref$<str$> > >
// 信息 ID: 52e5355f0ddf9d9b7134628b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u32", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u32,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing @ 0x1ad3865 -> 0x1adf890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,core::any::TypeId> >
// 信息 ID: 11fb49236a47f731e62f43f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "core::any::TypeId"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,core::any::TypeId>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 253 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d8ea -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da1a -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1decf -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e001 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e4bf -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5f1 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eaff -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec38 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f08f -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f1c8 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f61c -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f750 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fac2 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbf4 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2009c -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201c9 -> 0xdd6d0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > > >
// 信息 ID: 582198ee65f44164237ec37e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["glam::u32::uvec2::UVec2", "3"]; 字段布局 = unknown。
// [类型关系] slice2$ -> ["array$<glam::u32::uvec2::UVec2,3>"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<array$<glam::u32::uvec2::UVec2,3> >"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<array$<glam::u32::uvec2::UVec2,3> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_unindexed @ 0x58a7fb -> 0x6c1300 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > > >
// 信息 ID: 5b61af942b4bf93f0a5829b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec2::UVec2"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec2::UVec2>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec2::UVec2> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec2::UVec2> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x589ede -> 0x6c1830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > > >
// 信息 ID: f6081332c9f6138aeb08c278
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["glam::u32::uvec3::UVec3"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<glam::u32::uvec3::UVec3>"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["u64", "ref$<slice2$<glam::u32::uvec3::UVec3> >"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,ref$<slice2$<glam::u32::uvec3::UVec3> > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastMeshData::new_indexed @ 0x589e97 -> 0x6c1fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<u64,u64> >
// 信息 ID: c112a5db4ed740fa35f924e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["u64", "u64"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<u64,u64>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 253 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d8fb -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1da2b -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dee0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e012 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e4d0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e602 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1eb10 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ec49 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f0a0 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f1d9 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f62d -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f761 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fad3 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fc05 -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x200ad -> 0xdd860 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x201da -> 0xdd860 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<usize,u8> >
// 信息 ID: 206e9e2e98bbf07b8a353220
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["usize", "u8"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<usize,u8>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::resources::ui_state::UiState::add<system_decorator::decorator_resources::DecoratorHandleName> @ 0x1c79e02 -> 0x17a7f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::calculate_hash<tuple$<usize,usize,ref$<str$> > >
// 信息 ID: 7190a6cab4d9a6e5d14f0cdd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["usize", "usize", "ref$<str$>"]; 字段布局 = unknown。
// [类型关系] utils::calculate_hash -> ["tuple$<usize,usize,ref$<str$> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::clutter::add_tree_vegetation_setdressing::add_tree_setdressing @ 0x1ad4283 -> 0x1adee40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cartesian_to_polar
// 信息 ID: 4d821c066f6d7df9f0b98a2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::color_menu::color_menu @ 0x1aa97d0 -> 0x7da1f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::circle::Circle2d::get_closest_u_from_pos; module=741, inline_record=1284; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::cos_angle_between
// 信息 ID: a0e59247ae15b99462f34c8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=1448; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_corners::add_wall_corners; module=1248, inline_record=2352; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_fences::process_segment::split_curve_based_on_angle; module=2718, inline_record=1636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_line_point
// 信息 ID: 155c271bc97bf5b49d47a5b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17472; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=8812; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3008; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_segment_point
// 信息 ID: 1a6f8e07803a7eaabac9d202
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27256; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=740, inline_record=23384; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=8352; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1327, inline_record=27708; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query_with_circle_callback<system_tree::hide_trees::hide_trees::closure_env$0>; module=4216, inline_record=3016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::distance_squared_segment_point
// 信息 ID: cc0fa613b7c796f0b35840bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions; module=3515, inline_record=36716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::fit
// 信息 ID: 82bb49aa549e4487b53928f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x34027b -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x34006f -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_tod::do_time_of_day_screen::closure$0 @ 0x33f8d1 -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0x558920 -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0x55895e -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0x55899c -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0x5589d5 -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::remap_hour_to_fmod_hour @ 0x558a13 -> 0x7da6e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::golden_spiral
// 信息 ID: ef5052ed5fd3b509c02820ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0xa4f307 -> 0x7d9bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x1b30af8 -> 0x7d9bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::dot
// 信息 ID: ee3ff6407c5037ae8ac1fa3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 30 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=29612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=29684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17612; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=434, inline_record=22296; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=434, inline_record=22352; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=438, inline_record=33104; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=438, inline_record=33160; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=454, inline_record=19212; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length
// 信息 ID: c95ed7a7946e51d11d4c4258
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5b7df84f2f6e6c08cc7d9fc1: glam::f32::vec2::Vec2 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 30 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17876; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=8076; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=9216; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=740, inline_record=23984; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=8932; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1229, inline_record=12100; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample_per_segment<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1229, inline_record=15972; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1229, inline_record=26564; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::length_squared
// 信息 ID: a557631dfaf436b848fca363
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions; module=3515, inline_record=37312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::lerp
// 信息 ID: 50c485efcb67f4f2c74987d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 54 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::decorator_on_freehand_wall_edited::decorator_on_freehand_wall_edited; module=379, inline_record=2868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=29804; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=32312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=35388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=36108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17672; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::eraser::wall_eraser::wall_eraser; module=423, inline_record=16588; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ivy::wall_coord::WallCoord::into_world_space; module=452, inline_record=6000; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$0::normalize
// 信息 ID: fff132ff9f1a0b80e9dc9396
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=34684; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::ivy::wall_coord::WallCoord::into_world_space; module=452, inline_record=6436; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::get_wall_double_sided_normal; module=454, inline_record=20360; 不是独立调用边。
// [PDB 内联宿主] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; module=753, inline_record=23844; 不是独立调用边。
// [PDB 内联宿主] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0>; module=754, inline_record=5844; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::to_world_space; module=1261, inline_record=3844; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::get_tangent; module=1261, inline_record=5152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement_signifiers::shape_placement_signifiers; module=1302, inline_record=54620; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::dot
// 信息 ID: 6ee0d86a10fc058137659446
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=20248; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=20320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27596; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3196; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3968; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions; module=3515, inline_record=37880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length
// 信息 ID: e24a9c3338bf8ae654fc77a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=28020; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>; module=1229, inline_record=9968; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1229, inline_record=14012; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1229, inline_record=22764; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1229, inline_record=23352; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1229, inline_record=23588; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>; module=1229, inline_record=24040; 不是独立调用边。
// [PDB 内联宿主] utils::curve::curve_slice::CurveSlice::extend_right_until<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3,glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>),system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate::closure_env$1>; module=1243, inline_record=13584; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::length_squared
// 信息 ID: 8d25cb8c01d27814888f5865
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=4488; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=4660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::lerp
// 信息 ID: 438f645ceafb76bb00d0097c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 08d3b0595202524f3cef82cc: glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 引用方: 原程序直接转移 0 条；内联宿主 34 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=9172; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=10896; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=20448; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=24392; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27724; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends; module=453, inline_record=28812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=62300; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=68640; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$1::normalize
// 信息 ID: ec4eafbda290c8b9fb3ac25c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=9496; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=11236; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_platform_stairs::construct_platform_stairs; module=373, inline_record=11736; 不是独立调用边。
// [PDB 内联宿主] system_decorator::cull_oob_decorators::is_decorator_location_valid; module=383, inline_record=24820; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_splitter::wall_splitter; module=461, inline_record=63232; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D> > >,ref$<f32>,ref$<f32> > >,country_core::resources::walls::wall_trigger_volume::impl$2::recompute_quads::closure_env$0>; module=476, inline_record=22180; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::construct_gates::construct_wooden_gate_doors; module=1241, inline_record=30036; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=12288; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::length
// 信息 ID: 946a7073084f35491d8b5f3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::utils::half_timber::half_timbering; module=1249, inline_record=15288; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=3164; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=4856; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::utils::wall_skirts::add_skirts; module=1250, inline_record=6176; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=6184; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=9516; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::wall_constructor::WallConstructor::from_curve; module=1252, inline_record=10620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$2::lerp
// 信息 ID: bfd5acfc6597db61f90ac6ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=472, inline_record=6212; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=472, inline_record=6240; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::time_of_day::TodSequence::snapshot_at_tod; module=472, inline_record=6436; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::brush_preview::brush_preview; module=477, inline_record=24976; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::lantern::mantain_lantern_lights; module=3860, inline_record=25408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> > >
// 信息 ID: 00ee72818a8f26db0fc9bcbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::option::Option -> ["country_core::asset_libraries::texture_library::DescriptorIndex"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex>"]; 字段布局 = unknown。
// [类型关系] utils::impl$4::assign_auto_resize -> ["enum2$<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index @ 0x562f94 -> 0xd73fd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$4::assign_auto_resize<tg_text::AtlasAllocUsage>
// 信息 ID: 27c7c8cd972729669851fe0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::impl$4::assign_auto_resize -> ["tg_text::AtlasAllocUsage"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_text::TextRenderer::get_glyph @ 0x6fe30d -> 0x1149c70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$5::intersects
// 信息 ID: 16d9f0bb70c05a0c96af2db4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd07cb -> 0x177fc20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$6::intersects
// 信息 ID: a1ab76b1945ba0e5ff2a9986
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners @ 0x14dc3b6 -> 0x177fc40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad97a4 -> 0x177fc40 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle3d::overlaps_rectangle3d; module=740, inline_record=1388; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::impl$9::iterator_slice_range
// 信息 ID: 42c267596b5080433b770d12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments; module=1230, inline_record=2224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::inverse_lerp
// 信息 ID: 4ab746e0ad5d780dac81fb0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 104 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4d8032 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eaef3 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_walls::construct_walls @ 0x4eaf63 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calc_path_wall_segmentation @ 0x4f463b -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height @ 0x4fb97e -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0x5581a5 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0x59dca7 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::stairs_audio::stairs_audio_system @ 0x59e38c -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0x5a7a62 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0x5a7d78 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish @ 0x5d753a -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish @ 0x5d7576 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x617123 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x61713c -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x61750f -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0x617547 -> 0x7d9780 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::linearstep; module=615, inline_record=3616; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::impl$1::get_pos_at_u::closure$0; module=740, inline_record=11808; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D> >; module=1903, inline_record=4952; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::lerp
// 信息 ID: a02e6d89ac7b7303fd5a01ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9e286f0216e1fbaff1175eba: array$<glam::f32::sse2::vec3a::Vec3A,32> (*)()
// 引用方: 原程序直接转移 299 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x28424f -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0c04 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0c36 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0c50 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0e5d -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0e90 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0ead -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e0fd6 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1009 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1026 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1f97 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1fd8 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e2001 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1aee -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1b2e -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::segmentation::sample_wall_path_intersections::sample_wall_path_intersections @ 0x4e1b56 -> 0x7d97b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp; module=615, inline_record=3904; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=615, inline_record=4084; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec2; module=615, inline_record=4112; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=615, inline_record=4320; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=615, inline_record=4376; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=615, inline_record=4612; 不是独立调用边。
// [PDB 内联宿主] utils::fit; module=615, inline_record=11996; 不是独立调用边。
// [PDB 内联宿主] utils::wall_space::WallSpaceCoord::from_curve_internal_coord; module=1261, inline_record=2152; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::linearstep
// 信息 ID: ad7e6050ec4035c03fa238b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::gui_escape_menu::wishlist_button @ 0x284237 -> 0x7d9790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::mouse_ss_to_cs
// 信息 ID: 5eed45d69500329d151aca88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::transform_screen_space @ 0x5e47ca -> 0x7d9980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode @ 0x6c671d -> 0x7d9980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::pack_2x16f_uint
// 信息 ID: 9c1d09d79f1153f5d1b9e503
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x625bd3 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0xd64603 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::resources::placement_data::_::impl$0::deserialize::__Visitor> @ 0xd64472 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::manage_tree_prefabs::manage_tree_prefabs @ 0x144067c -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > > >,system_critters::duck::update_duck_ripples::closure_env$0> @ 0x158f27e -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::DuckState> > > >,system_critters::duck::update_duck_ripples::closure_env$0> @ 0x158f28e -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_solid_vertex_color_bricks @ 0x181c553 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x181ce58 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x181ce6b -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x181ce7e -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::systems::nani_update_gothic_window_bricks @ 0x181ce99 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] forest::tree_animation::handplaced_tree_animation @ 0x1a9adf4 -> 0x7d9f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::perspective_lerp
// 信息 ID: 92a1a055cb1fc291fd73b30b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=438, inline_record=33248; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::get_closest_point_u_to_cursor; module=496, inline_record=25288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::polar_to_cartesian
// 信息 ID: 0c5900b181bf4a319f399f3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_color::color_menu::color_menu @ 0x1aa9804 -> 0x7da240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_line
// 信息 ID: f2a9078bcc7739c4209ba9fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17500; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=484, inline_record=8840; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims; module=1461, inline_record=72688; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims; module=1461, inline_record=73876; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3040; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=2708, inline_record=3812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::project_point_onto_segment
// 信息 ID: 605560af6e73cf66ab8e18bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=19980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=29440; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=454, inline_record=22924; 不是独立调用边。
// [PDB 内联宿主] utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices; module=740, inline_record=23416; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=8380; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=9056; 不是独立调用边。
// [PDB 内联宿主] utils::curve::Curve::is_on_right_side<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>; module=1327, inline_record=27804; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::radical_inverse
// 信息 ID: 6a40641c96499006fd56f854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x66e8ac -> 0x7d9c50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3086, inline_record=4752; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3086, inline_record=4800; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3086, inline_record=5076; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<glam::f32::vec2::Vec2,core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec2::Vec2,alloc::vec::impl$19::extend_trusted::closure_env$0<glam::f32::vec2::Vec2,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<u32>,utils::camera::impl$7::default::closure_env$0> > > >; module=3086, inline_record=5124; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits
// 信息 ID: 299470029139989b99407455
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 18 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4efe09 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dc2d7 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dcf92 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_rectangular_roof::assemble_rectangular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8dde64 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::utils::assemble_circular_roof::assemble_circular_roof<system_roof::visual::assemble_roof_tiles::assemble_roof_tiles::closure_env$0> @ 0x8e07bd -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::place_pillars::assemble_stone_pillar<system_wall_constructor::place_pillars::place_pillars::closure_env$4> @ 0xcc80ce -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccafb5 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd175f9 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd17963 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_shape_stitches::stitch_bricks::spawn_stitches @ 0x14e3289 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::split_into_random_rows::split_into_random_rows @ 0x1780c9f -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_assemble_bricks::construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_env$2> @ 0x17e3e68 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren::update_roof_bird_spots @ 0x1b3cac0 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_cross_brick_pillar::construct_crossbrick_pillar<ref_mut$<system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base::closure_env$1> > @ 0x1c31887 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c41244 -> 0x7d9660 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$6> @ 0x1c4166e -> 0x7d9660 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into
// 信息 ID: 12f2a2706eeb09b382293de6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] utils::random_splits @ 0x7d96d8 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer @ 0xc9e663 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret @ 0xca0564 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2aa5 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_skirts::add_skirts @ 0xcd786a -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new @ 0xcd9b7d -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::wall_constructor::WallConstructor::from_curve @ 0xcdc0c2 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::inter_roof_merlons::construct_inter_roof_merlons::construct_inter_roof_merlons @ 0x1acd520 -> 0x7d93a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::random_splits_into::closure$0
// 信息 ID: 7371e84c22a9673004850057
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::random_splits_into", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::random_splits_into; module=615, inline_record=2132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rotation_from_normal
// 信息 ID: 6222d4c1a7273dcaa7f29d23
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7f2d -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccd86c -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0be9 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::impl$1::signifier::closure$0 @ 0xce13ae -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget @ 0xce1bcb -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18a5a -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$7> | system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env$3> @ 0xd18cf8 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xf102a4 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow @ 0xf104c8 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x1021aab -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::UiRectangleOrientationEdit::signifier @ 0x1021d66 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12ea796 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12eab61 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] timberframe::timberframe_rectangle::construct_timberframe_rectangle @ 0x12eaecc -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_wooden_pillar_w_stone_base @ 0x17742d6 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::add_underwindow_pattern<system_wall_constructor::utils::half_timber::half_timbering::closure_env$0> @ 0x17758f3 -> 0x7d9d00 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color
// 信息 ID: 61620939feecd3de7ed9a7a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::impl$5::render @ 0x43b532 -> 0x7da700 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::impl$8::render @ 0x43b615 -> 0x7da700 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::rect @ 0x6fd2be -> 0x7da700 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::report_bug::impl$7::render @ 0x15eca0c -> 0x7da700 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color::scale_linear
// 信息 ID: 9740278d70b5c497c5301f09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::scale_u32_packed_srgba_color; module=615, inline_record=12148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_linear
// 信息 ID: 202f0f9246f7f7447525d2c9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::scale_u32_packed_srgba_color_premultiplied::scale_srgb
// 信息 ID: b8ab7597ccfa640df0774415
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::slice_iter
// 信息 ID: a66f450324b4646f8286bdde
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments; module=1230, inline_record=2200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::smoothstep
// 信息 ID: fc0659d5715b78b042c00df8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 32 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_walls::construct_walls; module=371, inline_record=19564; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_noise::apply_roof_noise; module=477, inline_record=3348; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio; module=493, inline_record=17256; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::ridge_length_01_from_rectangle_ratio; module=493, inline_record=17320; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws; module=493, inline_record=17584; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws; module=493, inline_record=17644; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::roofs::roof_shape::RoofShapeParams::height_ws; module=493, inline_record=17708; 不是独立调用边。
// [PDB 内联宿主] system_critters::fish::fish; module=1012, inline_record=14588; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_eotf
// 信息 ID: 81321e4ad649e5574462f1bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::srgb_oetf
// 信息 ID: 78518a46fdadd4a576b9af24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::t_of_point_projection_onto_segment
// 信息 ID: df1e36a3a54eef3adc5992cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 22 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::segmentation::calculate_wall_path_segmentation::score_split_candidate; module=374, inline_record=20008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge; module=391, inline_record=27312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_spawner::ivy_spawner; module=392, inline_record=29468; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::snapping::snapping_init::snapping_init; module=401, inline_record=17528; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::wall_spatial_hash::WallSegment::terrain_height_closest_to_ws_pos; module=434, inline_record=22148; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray; module=438, inline_record=32960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::closest_segment_index; module=454, inline_record=19068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ivy::ivy_grower::intersect_ivy_growth_w_wall_segment; module=454, inline_record=22952; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp
// 信息 ID: 5a414e934c89eb3b0ac554cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 11 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::camera_rig::walk::impl$1::constrain_to_be_above_terrain @ 0x619a9a -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::history::replay_mode::replay_mode @ 0x68f812 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::shapes::calc_shape_height::calc_shape_height @ 0x6bcfd0 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::music_audio::music_audio_system @ 0xe9161b -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_position_update @ 0x147c5a3 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_onboarding::hint_display_system::hint_position_update @ 0x147c5ca -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x15a919b -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x15a9ff0 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::tree_move_bend::tree_move_bend @ 0x1a8fb00 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_tree::tree_move_bend::tree_move_bend @ 0x1a8fb26 -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::window_lights::window_lights @ 0x1c5350f -> 0x7d97c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=615, inline_record=4264; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_vec3; module=615, inline_record=4352; 不是独立调用边。
// [PDB 内联宿主] utils::time_continuous_lerp_mut; module=615, inline_record=4556; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_mut
// 信息 ID: 5b0d061acef2e5c91072df77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::time_of_day::time_of_day @ 0x5581f9 -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::prepare @ 0x15eaa18 -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::prepare @ 0x15eaa3a -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::credits::impl$3::prepare @ 0x15eaa5b -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$4::render @ 0x15ed1cb -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$4::render @ 0x15ed384 -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::impl$4::render @ 0x15ed3c6 -> 0x7d9920 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b42b8d -> 0x7d9920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec2
// 信息 ID: f91c04b0361056d07c2e6c5c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::time_continuous_lerp_vec3
// 信息 ID: a0b7b122fb5a00851d3ee723
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 9 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::fish::fish @ 0xa519bc -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa51bf0 -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa51d7e -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa51eed -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::fish::fish @ 0xa5248d -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x15a8df9 -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3> @ 0x15a9751 -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0> @ 0x15aa58e -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::demo_stuff::demo_show_full_glade_size::demo_show_full_glade_size @ 0x165c7dd -> 0x7d98b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_circle
// 信息 ID: d3a400e29bf6f166489013dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::spawn_ducks::spawn_ducks @ 0xa4f3b0 -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0xa4ff8b -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::spawn_ducks::despawn_ducks @ 0xa501d1 -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x1b30baa -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x1b31a4a -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::plan_duck_trajectories @ 0x1b312b4 -> 0x7d9a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_cone
// 信息 ID: 9baa428bf97bdd0d35e73630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::boids::Boid::random_vel_change @ 0x15a8d85 -> 0x7d9b30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_disk
// 信息 ID: 0e6f4ef1c5766535e8232a33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0x66e7d7 -> 0x7d99b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x1b2f9fa -> 0x7d99b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::sheep::sheep_behavior::update_sheep_goals @ 0x1b2fc0f -> 0x7d99b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::update_duck_goals @ 0x1b33996 -> 0x7d99b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::uniform_sample_sphere
// 信息 ID: e126ecb4874c8173e9bfed7a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::butterfly::update_butterfly_goals @ 0x15733ef -> 0x7d9a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::firefly::update_firefly_goals @ 0x1b2eeac -> 0x7d9a80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::wren_boids::sim_wren_boids @ 0x1b40bbd -> 0x7d9a80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_projective_rotation
// 信息 ID: 39897039a541037de765be30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 21 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::wall_splitter::wall_splitter @ 0x6379ae -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4cff -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf4d95 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf52c9 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::generate_cottage_dormers @ 0xcf52df -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui_move_freehand_walls::ui_move_freehand_walls @ 0x112b900 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e2cf6 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir @ 0x12e308f -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::blueprint::validate_blueprints @ 0x17a2c13 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_wall_window::generate_gothic_wall_windows @ 0x1c3ea98 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::gothic_balcony_door::generate_gothic_balcony_doors @ 0x1c49c91 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_balcony_door::generate_cottage_balcony_doors @ 0x1c4f202 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::arrow_slit::generate_arrow_slits @ 0x1c511c6 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c543cd -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c54465 -> 0x7da510 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_corner_window::generate_cottage_corner_windows @ 0x1c54563 -> 0x7da510 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation
// 信息 ID: 5741231750c2232b9615ea44
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::systems::construct_gates @ 0x4e52b7 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd60e -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd441 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::cache_decorator_transforms::cache_decorator_transforms @ 0x4fd541 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::construct_rect_bottom_layer @ 0xca2612 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_small_brackets::construct_rectangle_small_brackets @ 0xca4e2e -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::rectangle_large_brackets::construct_rectangle_large_brackets @ 0xca7b70 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::flatten::impl$3::next<core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<country_core::resources::walls::wall_stone_columns::StoneColumn>,alloc::vec::Vec<utils::geometry::intersect::Triangle,alloc::alloc::Global>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$0>,array$<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders::closure_env$1> @ 0xca91f2 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc3ec5 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc1325 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::roof_outline @ 0xccdad5 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd0125 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2af0 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd2f36 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd3621 -> 0x7da360 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::half_timber::half_timbering @ 0xcd401a -> 0x7da360 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] utils::xz_projective_rotation; module=615, inline_record=10716; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3
// 信息 ID: 48d57a218779e2280877b45f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners @ 0xccbe4e -> 0x7da2a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::xz_rotation_mat3a
// 信息 ID: 6baeeedbe76663610ea8014f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::xz_rotation; module=615, inline_record=9448; 不是独立调用边。
// [PDB 内联宿主] utils::xz_projective_rotation; module=615, inline_record=10740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
