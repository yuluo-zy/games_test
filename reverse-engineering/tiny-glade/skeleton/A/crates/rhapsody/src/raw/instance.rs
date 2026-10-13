// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/instance.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 5d2f1b3f9e1b0ef8cab9f121bc9c04396bafa8612d86caee16ce6677c9f04242
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: instance.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 559, 1605
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::impl$2::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11219
// 首条位置: {"module":1605,"inlinee":70169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::InstanceBuilder::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1605,"record_offset":712,"rva":18359184,"va":5387068304}
// 编译实例 RVAs: 0x1182390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e16c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17436424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8f8f0f5fe","size":5,"soft_fail":false,"target_va":5369632448,"va":5387068867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402d0900
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15413990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e81acd14ff","size":5,"soft_fail":false,"target_va":5371660544,"va":5387074529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140382c20
//     owner=unresolved_namespace; count=1
//     target_aliases=37 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14686696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e818e61fff","size":5,"soft_fail":false,"target_va":5372390432,"va":5387077123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14323717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8fb6f25ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387073408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=22
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14321931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8f57625ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387071638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072cff0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10843096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8288c5aff","size":5,"soft_fail":false,"target_va":5376233456,"va":5387076547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9545755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8e5576eff","size":5,"soft_fail":false,"target_va":5377527952,"va":5387073702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e80885ffff","size":5,"soft_fail":false,"target_va":5387042160,"va":5387073635,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16518529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8810dfc00","size":5,"soft_fail":false,"target_va":5403593360,"va":5387074826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15902199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8f7a5f200","size":5,"soft_fail":false,"target_va":5402978144,"va":5387075940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9538919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e899726eff","size":5,"soft_fail":false,"target_va":5377529888,"va":5387068802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-15761927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8f97d0fff","size":5,"soft_fail":false,"target_va":5371306912,"va":5387068834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::join_generic_copy
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7932919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8f70b7900","size":5,"soft_fail":false,"target_va":5395004304,"va":5387071380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::extensions_generated::ext::debug_utils::Instance::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8687b0000","size":5,"soft_fail":false,"target_va":5387106832,"va":5387075235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ash::instance::Instance::load
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t46427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e85bb50000","size":5,"soft_fail":false,"target_va":5387121216,"va":5387074784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15904106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e86aadf200","size":5,"soft_fail":false,"target_va":5402980128,"va":5387076017,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=22
//     evidence={"assembly":"\tcallq\t-9714717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8e3c36bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387068488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::env::_set_var
//     owner=rust_standard_library; count=18
//     evidence={"assembly":"\tcallq\t-9667330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387068304,"raw_hex":"e8fe7c6cff","size":5,"soft_fail":false,"target_va":5377401200,"va":5387068525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*632(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387068304,"raw_hex":"ff9578020000","size":6,"soft_fail":false,"va":5387074487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":16932,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":17432,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":16904,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":856,"parent_record_offset":712,"ancestor_record_offset":712,"inlinee":70144,"source_line":81,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":712,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":712,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":712,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9824,"parent_record_offset":9764,"ancestor_record_offset":712,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":712,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1872,"parent_record_offset":1816,"ancestor_record_offset":712,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":16964,"parent_record_offset":16932,"ancestor_record_offset":712,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":712,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":10656,"parent_record_offset":10572,"ancestor_record_offset":712,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":11092,"parent_record_offset":11068,"ancestor_record_offset":712,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2132,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":712,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":712,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1680,"parent_record_offset":1648,"ancestor_record_offset":712,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11524,"parent_record_offset":11488,"ancestor_record_offset":712,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11488,"parent_record_offset":11452,"ancestor_record_offset":712,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11744,"parent_record_offset":11488,"ancestor_record_offset":712,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":712,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":712,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":9976,"parent_record_offset":9764,"ancestor_record_offset":712,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":2556,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":7946,"source_line":576,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1605,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":712,"inlinee":7902,"source_line":76,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":712,"inlinee":7911,"source_line":993,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":712,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":12736,"parent_record_offset":12708,"ancestor_record_offset":712,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1648,"parent_record_offset":1600,"ancestor_record_offset":712,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11816,"parent_record_offset":11784,"ancestor_record_offset":712,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":712,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1460,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1600,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11416,"parent_record_offset":11368,"ancestor_record_offset":712,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11452,"parent_record_offset":11416,"ancestor_record_offset":712,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":712,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":712,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1488,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":12060,"parent_record_offset":12016,"ancestor_record_offset":712,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":712,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1605,"record_offset":12172,"parent_record_offset":12060,"ancestor_record_offset":712,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1605,"record_offset":11784,"parent_record_offset":11368,"ancestor_record_offset":712,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1605,"record_offset":11328,"parent_record_offset":11288,"ancestor_record_offset":712,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1605,"record_offset":11368,"parent_record_offset":11328,"ancestor_record_offset":712,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::entry::Entry::create_instance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":1605,"record_offset":12532,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70161,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::extensions_generated::ext::debug_utils::Instance::create_debug_utils_messenger
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":1605,"record_offset":14408,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70167,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::ApplicationInfo::api_version
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12308,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70158,"source_line":1139,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::DebugUtilsMessengerCreateInfoEXT::pfn_user_callback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":14380,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70165,"source_line":19795,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::InstanceCreateInfo::enabled_extension_names
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12336,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70160,"source_line":1449,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$1658::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":14352,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70163,"source_line":19761,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::impl$25::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12280,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70156,"source_line":1085,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12568,"parent_record_offset":12532,"ancestor_record_offset":712,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":712,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":712,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1605,"record_offset":11568,"parent_record_offset":11524,"ancestor_record_offset":712,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1605,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":712,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":1348,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":6648,"parent_record_offset":6616,"ancestor_record_offset":712,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":1376,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":2780,"parent_record_offset":2640,"ancestor_record_offset":712,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1605,"record_offset":11880,"parent_record_offset":11848,"ancestor_record_offset":712,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1605,"record_offset":11972,"parent_record_offset":11912,"ancestor_record_offset":712,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1605,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":712,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":10404,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":10488,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":11848,"parent_record_offset":11816,"ancestor_record_offset":712,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":12016,"parent_record_offset":11972,"ancestor_record_offset":712,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1605,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":712,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":712,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=70
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1740,"parent_record_offset":1600,"ancestor_record_offset":712,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":712,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":712,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":712,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":712,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1712,"parent_record_offset":1600,"ancestor_record_offset":712,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":10684,"parent_record_offset":10572,"ancestor_record_offset":712,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":13984,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":712,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":712,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1605,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":712,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":712,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2072,"parent_record_offset":2008,"ancestor_record_offset":712,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":712,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":11912,"parent_record_offset":11880,"ancestor_record_offset":712,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2008,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1605,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":712,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12772,"parent_record_offset":12736,"ancestor_record_offset":712,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":712,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":712,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":712,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12828,"parent_record_offset":12736,"ancestor_record_offset":712,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=22
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":1264,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6428,"parent_record_offset":6348,"ancestor_record_offset":712,"inlinee":30681,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7440,"parent_record_offset":6348,"ancestor_record_offset":712,"inlinee":30682,"source_line":104,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLoader::supports_modern_layer_filtering
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":1132,"parent_record_offset":1068,"ancestor_record_offset":712,"inlinee":30685,"source_line":387,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6616,"parent_record_offset":6468,"ancestor_record_offset":712,"inlinee":30646,"source_line":95,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7556,"parent_record_offset":7464,"ancestor_record_offset":712,"inlinee":30648,"source_line":105,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::Instance::extension_names
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":9764,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70153,"source_line":58,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::Instance::layer_names
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":10572,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70154,"source_line":73,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":1068,"parent_record_offset":856,"ancestor_record_offset":712,"inlinee":70146,"source_line":152,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":6348,"parent_record_offset":1068,"ancestor_record_offset":712,"inlinee":70150,"source_line":217,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_legacy_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3760,"parent_record_offset":1068,"ancestor_record_offset":712,"inlinee":70148,"source_line":194,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":1160,"parent_record_offset":1068,"ancestor_record_offset":712,"inlinee":70147,"source_line":168,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":2640,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":70152,"source_line":174,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3836,"parent_record_offset":3760,"ancestor_record_offset":712,"inlinee":70149,"source_line":246,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":712,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::InstanceBuilder::debug_markers
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77C0
// 首条位置: {"module":559,"inlinee":30656}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::extension_names
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11209
// 首条位置: {"module":1605,"inlinee":70153}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9824,"parent_record_offset":9764,"ancestor_record_offset":9764,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":9976,"parent_record_offset":9764,"ancestor_record_offset":9764,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10060,"parent_record_offset":9764,"ancestor_record_offset":9764,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":10180,"parent_record_offset":9764,"ancestor_record_offset":9764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9764,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":9764,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":10356,"parent_record_offset":10332,"ancestor_record_offset":9764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":9764,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":10332,"parent_record_offset":10232,"ancestor_record_offset":9764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":10264,"parent_record_offset":10232,"ancestor_record_offset":9764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":9764,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":10232,"parent_record_offset":10204,"ancestor_record_offset":9764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":9764,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10092,"parent_record_offset":10060,"ancestor_record_offset":9764,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":9764,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1605,"record_offset":10296,"parent_record_offset":10264,"ancestor_record_offset":9764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":9764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":9764,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":10148,"parent_record_offset":10060,"ancestor_record_offset":9764,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::layer_names
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1120A
// 首条位置: {"module":1605,"inlinee":70154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":10656,"parent_record_offset":10572,"ancestor_record_offset":10572,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10628,"parent_record_offset":10572,"ancestor_record_offset":10572,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10932,"parent_record_offset":10572,"ancestor_record_offset":10572,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":10684,"parent_record_offset":10572,"ancestor_record_offset":10572,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":10572,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":10856,"parent_record_offset":10800,"ancestor_record_offset":10572,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":10572,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":11092,"parent_record_offset":11068,"ancestor_record_offset":10572,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":10828,"parent_record_offset":10800,"ancestor_record_offset":10572,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11008,"parent_record_offset":10972,"ancestor_record_offset":10572,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":10800,"parent_record_offset":10772,"ancestor_record_offset":10572,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10972,"parent_record_offset":10932,"ancestor_record_offset":10572,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":10724,"parent_record_offset":10684,"ancestor_record_offset":10572,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":11040,"parent_record_offset":10932,"ancestor_record_offset":10572,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::Instance::create
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11200
// 首条位置: {"module":1605,"inlinee":70144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::entry::Entry::create_instance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/entry.rs
//     evidence={"module":1605,"record_offset":12532,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70161,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Instance::create_debug_utils_messenger
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":1605,"record_offset":14408,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70167,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ApplicationInfo::api_version
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12308,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70158,"source_line":1139,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCreateInfoEXT::pfn_user_callback
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":14380,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70165,"source_line":19795,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::InstanceCreateInfo::enabled_extension_names
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12336,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70160,"source_line":1449,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$1658::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":14352,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70163,"source_line":19761,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$25::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":12280,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70156,"source_line":1085,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":12448,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":12476,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":14636,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::chain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":10404,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":7699,"source_line":480,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":10488,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":12656,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":13984,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":12504,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":12364,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::extension_names
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":9764,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70153,"source_line":58,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::Instance::layer_names
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":10572,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70154,"source_line":73,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":1068,"parent_record_offset":856,"ancestor_record_offset":856,"inlinee":70146,"source_line":152,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":856,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9904,"parent_record_offset":9880,"ancestor_record_offset":856,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":856,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9824,"parent_record_offset":9764,"ancestor_record_offset":856,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9856,"parent_record_offset":9824,"ancestor_record_offset":856,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1872,"parent_record_offset":1816,"ancestor_record_offset":856,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":11144,"parent_record_offset":11120,"ancestor_record_offset":856,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":10656,"parent_record_offset":10572,"ancestor_record_offset":856,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":11092,"parent_record_offset":11068,"ancestor_record_offset":856,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2132,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":856,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":856,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1680,"parent_record_offset":1648,"ancestor_record_offset":856,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11524,"parent_record_offset":11488,"ancestor_record_offset":856,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11488,"parent_record_offset":11452,"ancestor_record_offset":856,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":11744,"parent_record_offset":11488,"ancestor_record_offset":856,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":856,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":10000,"parent_record_offset":9976,"ancestor_record_offset":856,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":9976,"parent_record_offset":9764,"ancestor_record_offset":856,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":2556,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":7946,"source_line":576,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1605,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":856,"inlinee":7902,"source_line":76,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":856,"inlinee":7911,"source_line":993,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":856,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":12736,"parent_record_offset":12708,"ancestor_record_offset":856,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1648,"parent_record_offset":1600,"ancestor_record_offset":856,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11816,"parent_record_offset":11784,"ancestor_record_offset":856,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10024,"parent_record_offset":10000,"ancestor_record_offset":856,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1460,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1600,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11416,"parent_record_offset":11368,"ancestor_record_offset":856,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":11452,"parent_record_offset":11416,"ancestor_record_offset":856,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10512,"parent_record_offset":10488,"ancestor_record_offset":856,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":856,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1488,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":12060,"parent_record_offset":12016,"ancestor_record_offset":856,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":856,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1605,"record_offset":12172,"parent_record_offset":12060,"ancestor_record_offset":856,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1605,"record_offset":11784,"parent_record_offset":11368,"ancestor_record_offset":856,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1605,"record_offset":11328,"parent_record_offset":11288,"ancestor_record_offset":856,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1605,"record_offset":11368,"parent_record_offset":11328,"ancestor_record_offset":856,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12568,"parent_record_offset":12532,"ancestor_record_offset":856,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":856,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1605,"record_offset":12616,"parent_record_offset":12592,"ancestor_record_offset":856,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1605,"record_offset":11568,"parent_record_offset":11524,"ancestor_record_offset":856,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1605,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":856,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":1348,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":6648,"parent_record_offset":6616,"ancestor_record_offset":856,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":1376,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":2780,"parent_record_offset":2640,"ancestor_record_offset":856,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1605,"record_offset":11880,"parent_record_offset":11848,"ancestor_record_offset":856,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1605,"record_offset":11972,"parent_record_offset":11912,"ancestor_record_offset":856,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1605,"record_offset":10536,"parent_record_offset":10512,"ancestor_record_offset":856,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":11848,"parent_record_offset":11816,"ancestor_record_offset":856,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1605,"record_offset":12016,"parent_record_offset":11972,"ancestor_record_offset":856,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1605,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":856,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":856,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=51
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1740,"parent_record_offset":1600,"ancestor_record_offset":856,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":856,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":856,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":856,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9932,"parent_record_offset":9904,"ancestor_record_offset":856,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1712,"parent_record_offset":1600,"ancestor_record_offset":856,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":10684,"parent_record_offset":10572,"ancestor_record_offset":856,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":856,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":856,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1605,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":856,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":856,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2072,"parent_record_offset":2008,"ancestor_record_offset":856,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":856,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":11912,"parent_record_offset":11880,"ancestor_record_offset":856,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2008,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1605,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":856,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12772,"parent_record_offset":12736,"ancestor_record_offset":856,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":856,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":856,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":856,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":12828,"parent_record_offset":12736,"ancestor_record_offset":856,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":1264,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6428,"parent_record_offset":6348,"ancestor_record_offset":856,"inlinee":30681,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7440,"parent_record_offset":6348,"ancestor_record_offset":856,"inlinee":30682,"source_line":104,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLoader::supports_modern_layer_filtering
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":1132,"parent_record_offset":1068,"ancestor_record_offset":856,"inlinee":30685,"source_line":387,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6616,"parent_record_offset":6468,"ancestor_record_offset":856,"inlinee":30646,"source_line":95,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7556,"parent_record_offset":7464,"ancestor_record_offset":856,"inlinee":30648,"source_line":105,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":6348,"parent_record_offset":1068,"ancestor_record_offset":856,"inlinee":70150,"source_line":217,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_legacy_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3760,"parent_record_offset":1068,"ancestor_record_offset":856,"inlinee":70148,"source_line":194,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":1160,"parent_record_offset":1068,"ancestor_record_offset":856,"inlinee":70147,"source_line":168,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":2640,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":70152,"source_line":174,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3836,"parent_record_offset":3760,"ancestor_record_offset":856,"inlinee":70149,"source_line":246,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":856,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11202
// 首条位置: {"module":1605,"inlinee":70146}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLoader::supports_modern_layer_filtering
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":1132,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":30685,"source_line":387,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":6348,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":70150,"source_line":217,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_legacy_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3760,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":70148,"source_line":194,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_modern_blocklist
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":1160,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":70147,"source_line":168,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1872,"parent_record_offset":1816,"ancestor_record_offset":1068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2132,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1068,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":1068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1680,"parent_record_offset":1648,"ancestor_record_offset":1068,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":1068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":2556,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":7946,"source_line":576,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1605,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":1068,"inlinee":7902,"source_line":76,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":1068,"inlinee":7911,"source_line":993,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":1068,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1648,"parent_record_offset":1600,"ancestor_record_offset":1068,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1460,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1600,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1488,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":1068,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":1348,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":6648,"parent_record_offset":6616,"ancestor_record_offset":1068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":1376,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":2780,"parent_record_offset":2640,"ancestor_record_offset":1068,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1605,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":1068,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1068,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1740,"parent_record_offset":1600,"ancestor_record_offset":1068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":1068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":1068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1712,"parent_record_offset":1600,"ancestor_record_offset":1068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":1068,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":1068,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1605,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":1068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":1068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2072,"parent_record_offset":2008,"ancestor_record_offset":1068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":1068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2008,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1605,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":1068,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":1068,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":1068,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":1264,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6428,"parent_record_offset":6348,"ancestor_record_offset":1068,"inlinee":30681,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7440,"parent_record_offset":6348,"ancestor_record_offset":1068,"inlinee":30682,"source_line":104,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6616,"parent_record_offset":6468,"ancestor_record_offset":1068,"inlinee":30646,"source_line":95,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7556,"parent_record_offset":7464,"ancestor_record_offset":1068,"inlinee":30648,"source_line":105,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":2640,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":70152,"source_line":174,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=16
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3836,"parent_record_offset":3760,"ancestor_record_offset":1068,"inlinee":70149,"source_line":246,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":1068,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_modern_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11203
// 首条位置: {"module":1605,"inlinee":70147}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2132,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1605,"record_offset":2556,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":7946,"source_line":576,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1460,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1600,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1488,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":1348,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":1376,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":2868,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2008,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":1264,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":2640,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":70152,"source_line":174,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":1160,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1160,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":1872,"parent_record_offset":1816,"ancestor_record_offset":1160,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1605,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1160,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":1160,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1680,"parent_record_offset":1648,"ancestor_record_offset":1160,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":1816,"parent_record_offset":1788,"ancestor_record_offset":1160,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$1::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1605,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":1160,"inlinee":7902,"source_line":76,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":1160,"inlinee":7911,"source_line":993,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":1648,"parent_record_offset":1600,"ancestor_record_offset":1160,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":1160,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":2752,"parent_record_offset":2640,"ancestor_record_offset":1160,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":2780,"parent_record_offset":2640,"ancestor_record_offset":1160,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1605,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":1160,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1160,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1740,"parent_record_offset":1600,"ancestor_record_offset":1160,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1160,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":1160,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":2044,"parent_record_offset":2008,"ancestor_record_offset":1160,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":1712,"parent_record_offset":1600,"ancestor_record_offset":1160,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1605,"record_offset":1512,"parent_record_offset":1488,"ancestor_record_offset":1160,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":1536,"parent_record_offset":1512,"ancestor_record_offset":1160,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":2072,"parent_record_offset":2008,"ancestor_record_offset":1160,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":1160,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":1160,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":2808,"parent_record_offset":2640,"ancestor_record_offset":1160,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":2668,"parent_record_offset":2640,"ancestor_record_offset":1160,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":2836,"parent_record_offset":2640,"ancestor_record_offset":1160,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11208
// 首条位置: {"module":1605,"inlinee":70152}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":2752,"parent_record_offset":2640,"ancestor_record_offset":2640,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":2780,"parent_record_offset":2640,"ancestor_record_offset":2640,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":2808,"parent_record_offset":2640,"ancestor_record_offset":2640,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":2668,"parent_record_offset":2640,"ancestor_record_offset":2640,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":2836,"parent_record_offset":2640,"ancestor_record_offset":2640,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2640,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2640,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_legacy_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11204
// 首条位置: {"module":1605,"inlinee":70148}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":3836,"parent_record_offset":3760,"ancestor_record_offset":3760,"inlinee":70149,"source_line":246,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":3948,"parent_record_offset":3836,"ancestor_record_offset":3760,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":3976,"parent_record_offset":3836,"ancestor_record_offset":3760,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":3760,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3760,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":4004,"parent_record_offset":3836,"ancestor_record_offset":3760,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3760,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":4032,"parent_record_offset":3836,"ancestor_record_offset":3760,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11206
// 首条位置: {"module":1605,"inlinee":70150}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_layer
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6428,"parent_record_offset":6348,"ancestor_record_offset":6348,"inlinee":30681,"source_line":94,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::init::GraphicsInstanceLayerAllowlist::contains_vendor
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7440,"parent_record_offset":6348,"ancestor_record_offset":6348,"inlinee":30682,"source_line":104,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::instance::legacy_disable_layer
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":6852,"parent_record_offset":6348,"ancestor_record_offset":6348,"inlinee":70149,"source_line":246,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$80::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1605,"record_offset":6672,"parent_record_offset":6648,"ancestor_record_offset":6348,"inlinee":19418,"source_line":2321,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":6964,"parent_record_offset":6852,"ancestor_record_offset":6348,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":6648,"parent_record_offset":6616,"ancestor_record_offset":6348,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":6992,"parent_record_offset":6852,"ancestor_record_offset":6348,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":6568,"parent_record_offset":6512,"ancestor_record_offset":6348,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6348,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1605,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":6348,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":6348,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":6512,"parent_record_offset":6468,"ancestor_record_offset":6348,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1605,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":6348,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":6348,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":6348,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":7020,"parent_record_offset":6852,"ancestor_record_offset":6348,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":6880,"parent_record_offset":6852,"ancestor_record_offset":6348,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_layer::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":6616,"parent_record_offset":6468,"ancestor_record_offset":6348,"inlinee":30646,"source_line":95,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$1::contains_vendor::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":1605,"record_offset":7556,"parent_record_offset":7464,"ancestor_record_offset":6348,"inlinee":30648,"source_line":105,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::env::set_var
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":7048,"parent_record_offset":6852,"ancestor_record_offset":6348,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::legacy_disable_layer
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 246
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11205
// 首条位置: {"module":1605,"inlinee":70149}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":3948,"parent_record_offset":3836,"ancestor_record_offset":3836,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":3976,"parent_record_offset":3836,"ancestor_record_offset":3836,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":4004,"parent_record_offset":3836,"ancestor_record_offset":3836,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=16
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":3864,"parent_record_offset":3836,"ancestor_record_offset":3836,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] std::env::set_var
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\env.rs
//     evidence={"module":1605,"record_offset":4032,"parent_record_offset":3836,"ancestor_record_offset":3836,"inlinee":11089,"source_line":360,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":3888,"parent_record_offset":3864,"ancestor_record_offset":3836,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":3912,"parent_record_offset":3888,"ancestor_record_offset":3836,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger
// 记录形式: compiled_procedure; instances=1
// 源码行: 252
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1605,"record_offset":17464,"rva":18368176,"va":5387077296}
// 编译实例 RVAs: 0x11846b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ea0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: instance.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16517037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387077296,"raw_hex":"e8ad07fc00","size":5,"soft_fail":false,"target_va":5403594400,"va":5387077358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ffi::c_str::CStr::to_str
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-9517828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387077296,"raw_hex":"e8fcc46eff","size":5,"soft_fail":false,"target_va":5377559552,"va":5387077375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15901151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387077296,"raw_hex":"e8dfa1f200","size":5,"soft_fail":false,"target_va":5402978880,"va":5387077724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15902329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387077296,"raw_hex":"e879a6f200","size":5,"soft_fail":false,"target_va":5402980128,"va":5387077794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9723928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387077296,"raw_hex":"e8e89f6bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5387077699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCallbackDataEXT::message_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":17872,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":70174,"source_line":19884,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsMessengerCallbackDataEXT::message_id_name_as_c_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1605,"record_offset":17656,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":70171,"source_line":19866,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1605,"record_offset":18176,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1605,"record_offset":18204,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":17772,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":17624,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1605,"record_offset":18232,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1605,"record_offset":18092,"parent_record_offset":17464,"ancestor_record_offset":17464,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":17680,"parent_record_offset":17656,"ancestor_record_offset":17464,"inlinee":9936,"source_line":274,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":17708,"parent_record_offset":17680,"ancestor_record_offset":17464,"inlinee":9938,"source_line":739,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::const_strlen::strlen_rt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ffi\c_str.rs
//     evidence={"module":1605,"record_offset":17732,"parent_record_offset":17708,"ancestor_record_offset":17464,"inlinee":9940,"source_line":752,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":17828,"parent_record_offset":17800,"ancestor_record_offset":17464,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":18116,"parent_record_offset":18092,"ancestor_record_offset":17464,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":17464,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::vulkan_debug_messenger::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":17800,"parent_record_offset":17772,"ancestor_record_offset":17464,"inlinee":70173,"source_line":270,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::instance::vulkan_debug_messenger::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/instance.rs
//     evidence={"module":1605,"record_offset":18020,"parent_record_offset":17992,"ancestor_record_offset":17464,"inlinee":70175,"source_line":273,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1121D
// 首条位置: {"module":1605,"inlinee":70173}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":17828,"parent_record_offset":17800,"ancestor_record_offset":17800,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::instance::vulkan_debug_messenger::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 273
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1121F
// 首条位置: {"module":1605,"inlinee":70175}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1605,"record_offset":18048,"parent_record_offset":18020,"ancestor_record_offset":18020,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01605.jsonl.gz"}
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
// 完整结构及原始类型记录: instance.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::instance::Instance
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::instance::Instance,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::instance::Instance", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::instance::InstanceBuilder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: instance.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: instance.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::create
// 信息 ID: 7051c03405523a22dd2075e5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::extension_names
// 信息 ID: 3c744aa6648fdfcb3901e8b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=9764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::Instance::layer_names
// 信息 ID: 18282bf39e7705e93da86e37
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=10572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::InstanceBuilder::build
// 信息 ID: f65f2b5ca884b47420cec001
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsInstance::new @ 0x7308f0 -> 0x1182390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::InstanceBuilder::debug_markers
// 信息 ID: ae531dff8a9263f4155d7acc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::init::GraphicsInstance::new; module=559, inline_record=14256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_blocklist
// 信息 ID: 376613c8c48fb4c15be9a6d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=1068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_doubletap_blocklist
// 信息 ID: e3006f4cfc18d3e5f6ad2bad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=6348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_legacy_blocklist
// 信息 ID: ab41f48b77877f19acedf37b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=3760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_modern_blocklist
// 信息 ID: ddd847e2683f7d7b588739b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=1160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::apply_vulkan_modern_blocklist::enable_layer
// 信息 ID: ae6eaca9fbf1a7370c5b59b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=2640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::impl$2::default
// 信息 ID: 81a427204c62e32b448cade5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::legacy_disable_layer
// 信息 ID: 6a15992bd12f100d4add58cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=3836; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=4064; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=4292; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=4520; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=4748; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=4976; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=5204; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::instance::InstanceBuilder::build; module=1605, inline_record=5432; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger
// 信息 ID: 509960cdbe25e65e3d2e5728
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger::closure$0
// 信息 ID: 3569b93996d784d5c962c5a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::instance::vulkan_debug_messenger", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::vulkan_debug_messenger; module=1605, inline_record=17800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::instance::vulkan_debug_messenger::closure$1
// 信息 ID: 9d9a531f5f148bb40cf44b0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::instance::vulkan_debug_messenger", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::instance::vulkan_debug_messenger; module=1605, inline_record=18020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
