// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/resource_pool.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 798b1e9c2d89eefaea00b20f58fdffaf035776634c0f9b3e851300a804e7b483
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_pool.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 256, 572, 575
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use gpu_allocator; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x159D
// 首条位置: {"module":14,"inlinee":5533}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":7120,"parent_record_offset":6192,"ancestor_record_offset":6192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":6228,"parent_record_offset":6192,"ancestor_record_offset":6192,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6492,"parent_record_offset":6464,"ancestor_record_offset":6192,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":6192,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":14,"record_offset":6520,"parent_record_offset":6464,"ancestor_record_offset":6192,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":14,"record_offset":6568,"parent_record_offset":6544,"ancestor_record_offset":6192,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":6424,"parent_record_offset":6400,"ancestor_record_offset":6192,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":6192,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":6192,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":14,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6192,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":14,"record_offset":6652,"parent_record_offset":6228,"ancestor_record_offset":6192,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":6192,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":6352,"parent_record_offset":6324,"ancestor_record_offset":6192,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6464,"parent_record_offset":6324,"ancestor_record_offset":6192,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":14,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":6192,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":14,"record_offset":6376,"parent_record_offset":6352,"ancestor_record_offset":6192,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":14,"record_offset":6400,"parent_record_offset":6376,"ancestor_record_offset":6192,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":6324,"parent_record_offset":6300,"ancestor_record_offset":6192,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":14,"record_offset":6300,"parent_record_offset":6276,"ancestor_record_offset":6192,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::get_texture
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79B2
// 首条位置: {"module":572,"inlinee":31154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5748,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":572,"record_offset":5720,"parent_record_offset":5688,"ancestor_record_offset":5688,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":572,"record_offset":5816,"parent_record_offset":5784,"ancestor_record_offset":5688,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":572,"record_offset":5784,"parent_record_offset":5748,"ancestor_record_offset":5688,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00572.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::insert_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":244,"rva":7636000,"va":5376345120}
// 编译实例 RVAs: 0x748420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3595630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e89222c9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376345321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3595819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e8d521c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376345526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-105578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e89663feff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376345493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141186930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10740715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e8ebe3a300","size":5,"soft_fail":false,"target_va":5387086128,"va":5376345408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141187030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10742750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e8deeba300","size":5,"soft_fail":false,"target_va":5387087920,"va":5376345165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26632581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e885619601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376345558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5036430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345120,"raw_hex":"e87226b3ff","size":5,"soft_fail":false,"target_va":5371308800,"va":5376345225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":656,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":808,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":436,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":920,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":408,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":892,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":244,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":736,"parent_record_offset":712,"ancestor_record_offset":244,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":1348,"parent_record_offset":1324,"ancestor_record_offset":244,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":244,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":1324,"parent_record_offset":1232,"ancestor_record_offset":244,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":244,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":508,"parent_record_offset":476,"ancestor_record_offset":244,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":244,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":244,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":476,"parent_record_offset":436,"ancestor_record_offset":244,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":856,"parent_record_offset":832,"ancestor_record_offset":244,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":976,"parent_record_offset":948,"ancestor_record_offset":244,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":1288,"parent_record_offset":1260,"ancestor_record_offset":244,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":568,"parent_record_offset":436,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":244,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":540,"parent_record_offset":436,"ancestor_record_offset":244,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::get_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":1664,"rva":7636688,"va":5376345808}
// 编译实例 RVAs: 0x7486d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141186020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10737954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376345808,"raw_hex":"e822d9a300","size":5,"soft_fail":false,"target_va":5387083808,"va":5376345849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":2864,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":1808,"parent_record_offset":1664,"ancestor_record_offset":1664,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":1664,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":2896,"parent_record_offset":2864,"ancestor_record_offset":1664,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":575,"record_offset":2764,"parent_record_offset":2732,"ancestor_record_offset":1664,"inlinee":31250,"source_line":114,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":1664,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":2064,"parent_record_offset":2040,"ancestor_record_offset":1664,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":2148,"parent_record_offset":2124,"ancestor_record_offset":1664,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":575,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":1664,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":575,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1664,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":1664,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":1664,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":575,"record_offset":2792,"parent_record_offset":2732,"ancestor_record_offset":1664,"inlinee":31252,"source_line":229,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":575,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":1664,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":1664,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1664,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":2648,"parent_record_offset":2428,"ancestor_record_offset":1664,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":1888,"parent_record_offset":1856,"ancestor_record_offset":1664,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2560,"parent_record_offset":2536,"ancestor_record_offset":1664,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2536,"parent_record_offset":2428,"ancestor_record_offset":1664,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1664,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2396,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1664,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":1940,"parent_record_offset":1916,"ancestor_record_offset":1664,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":1916,"parent_record_offset":1856,"ancestor_record_offset":1664,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":1664,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1664,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":2240,"parent_record_offset":2208,"ancestor_record_offset":1664,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":2180,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2012,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":2428,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":2040,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":2124,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":2304,"parent_record_offset":1964,"ancestor_record_offset":1664,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":575,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":1664,"inlinee":31248,"source_line":63,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::insert_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":2968,"rva":7637008,"va":5376346128}
// 编译实例 RVAs: 0x748810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3596838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8da1dc9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376346529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3597023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8211dc9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376346730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-106778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8e65efeff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376346693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141186020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10737607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8c7d7a300","size":5,"soft_fail":false,"target_va":5387083808,"va":5376346196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141186390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10738067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e893d9a300","size":5,"soft_fail":false,"target_va":5387084688,"va":5376346616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26631369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8c95c9601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376346770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5039176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376346128,"raw_hex":"e8b81bb3ff","size":5,"soft_fail":false,"target_va":5371307280,"va":5376346451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":4520,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":4672,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":4441,"source_line":461,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":4300,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":4784,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":3128,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":5613,"source_line":1070,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":4756,"parent_record_offset":2968,"ancestor_record_offset":2968,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":2968,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":4600,"parent_record_offset":4576,"ancestor_record_offset":2968,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":2968,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":4552,"parent_record_offset":4520,"ancestor_record_offset":2968,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":575,"record_offset":5188,"parent_record_offset":5096,"ancestor_record_offset":2968,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":2968,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":4372,"parent_record_offset":4340,"ancestor_record_offset":2968,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":575,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":2968,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":575,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":2968,"inlinee":4442,"source_line":100,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":4340,"parent_record_offset":4300,"ancestor_record_offset":2968,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":4720,"parent_record_offset":4696,"ancestor_record_offset":2968,"inlinee":4443,"source_line":819,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":575,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":2968,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":575,"record_offset":4088,"parent_record_offset":4056,"ancestor_record_offset":2968,"inlinee":31250,"source_line":114,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":575,"record_offset":4028,"parent_record_offset":4000,"ancestor_record_offset":2968,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":2968,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":575,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2968,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":575,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":2968,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":575,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":2968,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":575,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":2968,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":4432,"parent_record_offset":4300,"ancestor_record_offset":2968,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":3932,"parent_record_offset":3908,"ancestor_record_offset":2968,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":575,"record_offset":3824,"parent_record_offset":3800,"ancestor_record_offset":2968,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":4628,"parent_record_offset":4600,"ancestor_record_offset":2968,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":575,"record_offset":4404,"parent_record_offset":4300,"ancestor_record_offset":2968,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":575,"record_offset":4116,"parent_record_offset":4056,"ancestor_record_offset":2968,"inlinee":31252,"source_line":229,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":575,"record_offset":4000,"parent_record_offset":3972,"ancestor_record_offset":2968,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2968,"inlinee":5615,"source_line":1461,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":2968,"inlinee":5614,"source_line":1449,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":3972,"parent_record_offset":3752,"ancestor_record_offset":2968,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":3208,"parent_record_offset":3176,"ancestor_record_offset":2968,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":2968,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3860,"parent_record_offset":3752,"ancestor_record_offset":2968,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3800,"parent_record_offset":3776,"ancestor_record_offset":2968,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3720,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3776,"parent_record_offset":3752,"ancestor_record_offset":2968,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3260,"parent_record_offset":3236,"ancestor_record_offset":2968,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3236,"parent_record_offset":3176,"ancestor_record_offset":2968,"inlinee":5619,"source_line":1489,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3288,"parent_record_offset":3260,"ancestor_record_offset":2968,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":3532,"parent_record_offset":3504,"ancestor_record_offset":2968,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":3564,"parent_record_offset":3532,"ancestor_record_offset":2968,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":575,"record_offset":3504,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3336,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":575,"record_offset":3752,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":3364,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":3448,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":575,"record_offset":3628,"parent_record_offset":3288,"ancestor_record_offset":2968,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$10::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":575,"record_offset":4056,"parent_record_offset":4028,"ancestor_record_offset":2968,"inlinee":31248,"source_line":63,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_pool::ResourcePool::clear
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":575,"record_offset":5528,"rva":7637984,"va":5376347104}
// 编译实例 RVAs: 0x748be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141181090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10716323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376347104,"raw_hex":"e8a384a300","size":5,"soft_fail":false,"target_va":5387063440,"va":5376347112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x141180c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: resource_pool.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t10715234","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5376347104,"raw_hex":"e96280a300","size":5,"soft_fail":false,"target_va":5387062368,"va":5376347129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":575,"record_offset":5660,"parent_record_offset":5528,"ancestor_record_offset":5528,"inlinee":17320,"source_line":710,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":575,"record_offset":5680,"parent_record_offset":5660,"ancestor_record_offset":5528,"inlinee":14997,"source_line":1011,"archive":"_pdb/A/modules/00575.jsonl.gz"}
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
// 完整结构及原始类型记录: resource_pool.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_pool::ResourcePool
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_pool::ResourcePool>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_pool::ResourcePool"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_pool.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_pool.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::clear
// 信息 ID: 364859cf00b749416c8e9e84
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x24ac2c -> 0x748be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::get_buffer
// 信息 ID: b8b62700c075dbf26b8709e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::get_texture
// 信息 ID: d5ecf8c2978174054f98598a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources; module=572, inline_record=5688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::insert_buffer
// 信息 ID: c0136d3af81105fcf9023896
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::ResourcePool::insert_texture
// 信息 ID: dd6e627b272a155f8f815393
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_pool::impl$1::default
// 信息 ID: 00e750fd08193cf036c9008a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$7,tuple$<> >; module=14, inline_record=6192; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::run_game::run_game_thread; module=256, inline_record=55928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
