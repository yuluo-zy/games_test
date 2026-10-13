// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/lang-lib/src/language_library.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 28e2cf18c0f6ab5d12a6568e18162bd081d050d920b69ea40982aeed8d25cff8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 538, 1544, 4000
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use hashlink; // namespace candidate
//   package identities: [{"name":"hashlink","source":"PDB_registry_path_only","version":"0.8.4"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use yaml_rust2; // namespace candidate
//   package identities: [{"name":"yaml-rust2","source":"PDB_registry_path_only","version":"0.8.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::get
// 记录形式: compiled_procedure; instances=1
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1544,"record_offset":880,"rva":18269040,"va":5386978160}
// 编译实例 RVAs: 0x116c370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dcf90
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17364033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8bf0bf7fe","size":5,"soft_fail":false,"target_va":5369614224,"va":5386978252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17305790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e842eff7fe","size":5,"soft_fail":false,"target_va":5369673744,"va":5386979529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14730344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8983b1fff","size":5,"soft_fail":false,"target_va":5372249392,"va":5386979731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14353794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e87efa24ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5386978813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14229375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e881e026ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5386979066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14229927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e859de26ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386979634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16614901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8f585fd00","size":5,"soft_fail":false,"target_va":5403593344,"va":5386978438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16614186","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e82a83fd00","size":5,"soft_fail":false,"target_va":5403593360,"va":5386979169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15998701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8ed1ef400","size":5,"soft_fail":false,"target_va":5402978144,"va":5386979438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9449161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e837d16fff","size":5,"soft_fail":false,"target_va":5377529888,"va":5386979044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9452330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8d6c46fff","size":5,"soft_fail":false,"target_va":5377527120,"va":5386979445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16000744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8e826f400","size":5,"soft_fail":false,"target_va":5402980128,"va":5386979379,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] hashbrown::map::HashMap$LT$K$C$V$C$S$C$A$GT$::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8007272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8682e7a00","size":5,"soft_fail":false,"target_va":5394986480,"va":5386979203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-9625198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e892216dff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386978969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9612572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e8e4526dff","size":5,"soft_fail":false,"target_va":5377366224,"va":5386978791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t15996015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e86f14f400","size":5,"soft_fail":false,"target_va":5402975328,"va":5386979308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15996201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e82915f400","size":5,"soft_fail":false,"target_va":5402975504,"va":5386979298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15996298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386978160,"raw_hex":"e88a15f400","size":5,"soft_fail":false,"target_va":5402975696,"va":5386979393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1544,"record_offset":4576,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1544,"record_offset":4492,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":1544,"record_offset":4976,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1544,"record_offset":4408,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1544,"record_offset":4436,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1544,"record_offset":2696,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":5528,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1544,"record_offset":2652,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1544,"record_offset":4464,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1544,"record_offset":4324,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1544,"record_offset":1060,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1544,"record_offset":5500,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1544,"record_offset":2156,"parent_record_offset":880,"ancestor_record_offset":880,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":880,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":880,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":880,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4612,"parent_record_offset":4576,"ancestor_record_offset":880,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4644,"parent_record_offset":4612,"ancestor_record_offset":880,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":4900,"parent_record_offset":4844,"ancestor_record_offset":880,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1544,"record_offset":4540,"parent_record_offset":4516,"ancestor_record_offset":880,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":880,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":5224,"parent_record_offset":5196,"ancestor_record_offset":880,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":5196,"parent_record_offset":5168,"ancestor_record_offset":880,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":5380,"parent_record_offset":5196,"ancestor_record_offset":880,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":4844,"parent_record_offset":4816,"ancestor_record_offset":880,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1544,"record_offset":5132,"parent_record_offset":5104,"ancestor_record_offset":880,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1544,"record_offset":5104,"parent_record_offset":5076,"ancestor_record_offset":880,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1544,"record_offset":5048,"parent_record_offset":5004,"ancestor_record_offset":880,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1544,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":880,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":1544,"record_offset":5004,"parent_record_offset":4976,"ancestor_record_offset":880,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1544,"record_offset":5168,"parent_record_offset":5132,"ancestor_record_offset":880,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":880,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":880,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":3052,"parent_record_offset":2996,"ancestor_record_offset":880,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1544,"record_offset":1956,"parent_record_offset":1928,"ancestor_record_offset":880,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1544,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":880,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1544,"record_offset":1400,"parent_record_offset":1376,"ancestor_record_offset":880,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1544,"record_offset":1340,"parent_record_offset":1316,"ancestor_record_offset":880,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1544,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":880,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1544,"record_offset":1516,"parent_record_offset":1492,"ancestor_record_offset":880,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1544,"record_offset":2728,"parent_record_offset":2696,"ancestor_record_offset":880,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1544,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":880,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1544,"record_offset":4516,"parent_record_offset":4492,"ancestor_record_offset":880,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1544,"record_offset":5420,"parent_record_offset":5132,"ancestor_record_offset":880,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":4764,"parent_record_offset":4576,"ancestor_record_offset":880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1544,"record_offset":1860,"parent_record_offset":1836,"ancestor_record_offset":880,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1544,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":880,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":4720,"parent_record_offset":4692,"ancestor_record_offset":880,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1544,"record_offset":2288,"parent_record_offset":2228,"ancestor_record_offset":880,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1544,"record_offset":2040,"parent_record_offset":2012,"ancestor_record_offset":880,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1544,"record_offset":2068,"parent_record_offset":2040,"ancestor_record_offset":880,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1544,"record_offset":1984,"parent_record_offset":1956,"ancestor_record_offset":880,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":2572,"parent_record_offset":2544,"ancestor_record_offset":880,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":5780,"parent_record_offset":5592,"ancestor_record_offset":880,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":880,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":5872,"parent_record_offset":5840,"ancestor_record_offset":880,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":2476,"parent_record_offset":2444,"ancestor_record_offset":880,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":2260,"parent_record_offset":2228,"ancestor_record_offset":880,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":880,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":5804,"parent_record_offset":5780,"ancestor_record_offset":880,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1544,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":880,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1544,"record_offset":1928,"parent_record_offset":1900,"ancestor_record_offset":880,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1544,"record_offset":1084,"parent_record_offset":1060,"ancestor_record_offset":880,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1544,"record_offset":1108,"parent_record_offset":1084,"ancestor_record_offset":880,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1544,"record_offset":1900,"parent_record_offset":1680,"ancestor_record_offset":880,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1544,"record_offset":1140,"parent_record_offset":1108,"ancestor_record_offset":880,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":880,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1788,"parent_record_offset":1680,"ancestor_record_offset":880,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":880,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1648,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":880,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":880,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1168,"parent_record_offset":1108,"ancestor_record_offset":880,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":880,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1544,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":880,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1544,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":880,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1544,"record_offset":3692,"parent_record_offset":3568,"ancestor_record_offset":880,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1544,"record_offset":1432,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1264,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1544,"record_offset":1680,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1544,"record_offset":1292,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1544,"record_offset":1376,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1544,"record_offset":1556,"parent_record_offset":1216,"ancestor_record_offset":880,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1544,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":880,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":880,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1544,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":880,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":2996,"parent_record_offset":2848,"ancestor_record_offset":880,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":880,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1544,"record_offset":2444,"parent_record_offset":2416,"ancestor_record_offset":880,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1544,"record_offset":2416,"parent_record_offset":2388,"ancestor_record_offset":880,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1544,"record_offset":2324,"parent_record_offset":2156,"ancestor_record_offset":880,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1544,"record_offset":5556,"parent_record_offset":5528,"ancestor_record_offset":880,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1544,"record_offset":5592,"parent_record_offset":5556,"ancestor_record_offset":880,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1544,"record_offset":2544,"parent_record_offset":2352,"ancestor_record_offset":880,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1544,"record_offset":2352,"parent_record_offset":2324,"ancestor_record_offset":880,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1544,"record_offset":2196,"parent_record_offset":2156,"ancestor_record_offset":880,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1544,"record_offset":5840,"parent_record_offset":5556,"ancestor_record_offset":880,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1544,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":880,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1544,"record_offset":2928,"parent_record_offset":2900,"ancestor_record_offset":880,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1544,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":880,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1544,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":880,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1544,"record_offset":2388,"parent_record_offset":2352,"ancestor_record_offset":880,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x734A
// 首条位置: {"module":538,"inlinee":29514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":538,"record_offset":13336,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":538,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":12216,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":538,"record_offset":13136,"parent_record_offset":13108,"ancestor_record_offset":12216,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":538,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":12216,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":538,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":12216,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":538,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":12216,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":538,"record_offset":12696,"parent_record_offset":12672,"ancestor_record_offset":12216,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":12216,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":538,"record_offset":12932,"parent_record_offset":12908,"ancestor_record_offset":12216,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":13220,"parent_record_offset":13192,"ancestor_record_offset":12216,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":538,"record_offset":13248,"parent_record_offset":13220,"ancestor_record_offset":12216,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":538,"record_offset":13164,"parent_record_offset":13136,"ancestor_record_offset":12216,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":538,"record_offset":13108,"parent_record_offset":13080,"ancestor_record_offset":12216,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12264,"parent_record_offset":12240,"ancestor_record_offset":12216,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12288,"parent_record_offset":12264,"ancestor_record_offset":12216,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":13080,"parent_record_offset":12860,"ancestor_record_offset":12216,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":538,"record_offset":12320,"parent_record_offset":12288,"ancestor_record_offset":12216,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12216,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12968,"parent_record_offset":12860,"ancestor_record_offset":12216,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12908,"parent_record_offset":12884,"ancestor_record_offset":12216,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12828,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":12216,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12372,"parent_record_offset":12348,"ancestor_record_offset":12216,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12348,"parent_record_offset":12288,"ancestor_record_offset":12216,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12396,"parent_record_offset":12372,"ancestor_record_offset":12216,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12640,"parent_record_offset":12612,"ancestor_record_offset":12216,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12672,"parent_record_offset":12640,"ancestor_record_offset":12216,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":538,"record_offset":12612,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12444,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":538,"record_offset":12860,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12472,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12556,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":538,"record_offset":12736,"parent_record_offset":12396,"ancestor_record_offset":12216,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00538.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::LanguageLibrary::try_from_yml
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1544,"record_offset":5944,"rva":18270656,"va":5386979776}
// 编译实例 RVAs: 0x116c9c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..collections..hash..map..HashMap$LT$K$C$V$C$S$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$$LP$K$C$V$RP$$GT$$GT$::from_iter
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8020721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e8f1627a00","size":5,"soft_fail":false,"target_va":5395000752,"va":5386980026,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14355042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e89ef524ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5386980061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14230417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e86fdc26ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5386980124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..convert..From$LT$E$GT$$u20$for$u20$anyhow..Error$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t16545447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e8a776fc00","size":5,"soft_fail":false,"target_va":5403525424,"va":5386979972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15998704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e8f01ef400","size":5,"soft_fail":false,"target_va":5402978880,"va":5386980171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$yaml_rust2..yaml..Yaml$GT$$GT$
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e827f6ffff","size":5,"soft_fail":false,"target_va":5386977584,"va":5386980100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9574198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e8cae86dff","size":5,"soft_fail":false,"target_va":5377405616,"va":5386979809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9613771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e8354e6dff","size":5,"soft_fail":false,"target_va":5377366224,"va":5386979990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] yaml_rust2::yaml::YamlLoader::load_from_str
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8017994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386979776,"raw_hex":"e84a587a00","size":5,"soft_fail":false,"target_va":5394997856,"va":5386979857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1544,"record_offset":6220,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1544,"record_offset":6912,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1544,"record_offset":6856,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1544,"record_offset":6276,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":6940,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1544,"record_offset":6156,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1544,"record_offset":6344,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] hashlink::linked_hash_map::LinkedHashMap::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashlink","version":"0.8.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashlink-0.8.4/src/linked_hash_map.rs
//     evidence={"module":1544,"record_offset":6312,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":68332,"source_line":134,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1544,"record_offset":6372,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1544,"record_offset":6128,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":11086,"source_line":302,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_direct] yaml_rust2::yaml::Yaml::as_hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"yaml-rust2","version":"0.8.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/yaml-rust2-0.8.0/src/yaml.rs
//     evidence={"module":1544,"record_offset":6248,"parent_record_offset":5944,"ancestor_record_offset":5944,"inlinee":68336,"source_line":472,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":5944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1544,"record_offset":7068,"parent_record_offset":7012,"ancestor_record_offset":5944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":5944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1544,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":5944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":6692,"parent_record_offset":6664,"ancestor_record_offset":5944,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":5944,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1544,"record_offset":6720,"parent_record_offset":6664,"ancestor_record_offset":5944,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1544,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":5944,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1544,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":5944,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1544,"record_offset":6620,"parent_record_offset":6596,"ancestor_record_offset":5944,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":5944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1544,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":5944,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":6436,"parent_record_offset":6404,"ancestor_record_offset":5944,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1544,"record_offset":6536,"parent_record_offset":6500,"ancestor_record_offset":5944,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":6664,"parent_record_offset":6500,"ancestor_record_offset":5944,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1544,"record_offset":6404,"parent_record_offset":6372,"ancestor_record_offset":5944,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1544,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":5944,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1544,"record_offset":6596,"parent_record_offset":6564,"ancestor_record_offset":5944,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1544,"record_offset":6500,"parent_record_offset":6468,"ancestor_record_offset":5944,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1544,"record_offset":6468,"parent_record_offset":6436,"ancestor_record_offset":5944,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1BE9C
// 首条位置: {"module":4000,"inlinee":114332}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":4000,"record_offset":1736,"parent_record_offset":1092,"ancestor_record_offset":1092,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":4000,"record_offset":1152,"parent_record_offset":1092,"ancestor_record_offset":1092,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":4000,"record_offset":1904,"parent_record_offset":1092,"ancestor_record_offset":1092,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4000,"record_offset":2584,"parent_record_offset":1092,"ancestor_record_offset":1092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":1092,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":1092,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1092,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":1760,"parent_record_offset":1736,"ancestor_record_offset":1092,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":1552,"parent_record_offset":1460,"ancestor_record_offset":1092,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":4000,"record_offset":2712,"parent_record_offset":2656,"ancestor_record_offset":1092,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4000,"record_offset":2684,"parent_record_offset":2656,"ancestor_record_offset":1092,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4000,"record_offset":1460,"parent_record_offset":1432,"ancestor_record_offset":1092,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4000,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":1092,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4000,"record_offset":1672,"parent_record_offset":1432,"ancestor_record_offset":1092,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":4000,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":1092,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4000,"record_offset":1312,"parent_record_offset":1280,"ancestor_record_offset":1092,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4000,"record_offset":1280,"parent_record_offset":1248,"ancestor_record_offset":1092,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4000,"record_offset":1216,"parent_record_offset":1184,"ancestor_record_offset":1092,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4000,"record_offset":1248,"parent_record_offset":1216,"ancestor_record_offset":1092,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":4000,"record_offset":1184,"parent_record_offset":1152,"ancestor_record_offset":1092,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":4000,"record_offset":1404,"parent_record_offset":1312,"ancestor_record_offset":1092,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":4000,"record_offset":1496,"parent_record_offset":1460,"ancestor_record_offset":1092,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":4000,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":1092,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":4000,"record_offset":1372,"parent_record_offset":1348,"ancestor_record_offset":1092,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":4000,"record_offset":1348,"parent_record_offset":1312,"ancestor_record_offset":1092,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4000,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":1092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/04000.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":4000,"record_offset":1628,"parent_record_offset":1600,"ancestor_record_offset":1092,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/04000.jsonl.gz"}
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
// 完整结构及原始类型记录: language_library.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] lang_lib::language_library::LanguageLibrary
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >", "constructor": "core::option::Option", "arguments": ["alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::Guard", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::Guard<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::HybridStrategy<arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<lang_lib::language_library::LanguageLibrary,anyhow::Error,alloc::string::String,lang_lib::load_locale_impl::closure_env$0>", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["lang_lib::language_library::LanguageLibrary", "anyhow::Error", "alloc::string::String", "lang_lib::load_locale_impl::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::impl$2::pay_all::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<tuple$<>,arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> > >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["tuple$<>", "arc_swap::debt::impl$2::pay_all::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::strategy::hybrid::HybridProtection", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "constructor": "arc_swap::strategy::hybrid::impl$5::load::closure_env$0", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >", "arc_swap::strategy::hybrid::DefaultConfig"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::list::LocalNode::with<arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >,arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig> >", "constructor": "arc_swap::debt::list::LocalNode::with", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "arc_swap::strategy::hybrid::impl$5::load::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lang_lib::language_library::LanguageLibrary", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["arc_swap::strategy::hybrid::HybridProtection<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::strategy::hybrid::HybridProtection::fallback<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::strategy::hybrid::HybridProtection::fallback", "arguments": ["enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "arc_swap::debt::helping::Slots::help<arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>,enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > > >", "constructor": "arc_swap::debt::helping::Slots::help", "arguments": ["arc_swap::strategy::hybrid::impl$5::wait_for_readers::closure_env$0<enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >,arc_swap::strategy::hybrid::DefaultConfig>", "enum2$<core::option::Option<alloc::sync::Arc<lang_lib::language_library::LanguageLibrary,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: language_library.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: language_library.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::get
// 信息 ID: 54095e9b88cc6b42994bcb91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::lookup @ 0x719731 -> 0x116c370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_from_yml
// 信息 ID: 42595d4c012976e852078a4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] lang_lib::load_locale_impl @ 0x718933 -> 0x116c9c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0x718da3 -> 0x116c9c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] lang_lib::load_locale_impl @ 0x718ae4 -> 0x116c9c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::LanguageLibrary::try_get
// 信息 ID: 8ce7885c3ef8bdf43232c352
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] lang_lib::lookup; module=538, inline_record=12216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: lang_lib::language_library::impl$0::try_from_yml::closure$0
// 信息 ID: f80b20b9617cd09c26005499
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "lang_lib::language_library::impl$0::try_from_yml", "captures": "unknown"}
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- std::collections::hash::map::impl$79::from_iter<alloc::string::String,ref$<str$>,std::hash::random::RandomState,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] lang_lib::language_library::impl$0::try_from_yml::closure_env$0 <- core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tuple$<alloc::string::String,ref$<str$> >,hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<alloc::string::String,ref$<str$> >,hashbrown::map::impl$94::extend::closure_env$0<alloc::string::String,ref$<str$>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<hashlink::linked_hash_map::Iter<enum2$<yaml_rust2::yaml::Yaml>,enum2$<yaml_rust2::yaml::Yaml> >,lang_lib::language_library::impl$0::try_from_yml::closure_env$0> > > >; module=4000, inline_record=1092; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
