// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tabloid/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f92fc7a747914982d82110f661d1a88b9f949372ed7d4a454e0e8f54adfc822b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 360, 498, 500, 895, 1208, 1210, 1308, 1398, 2281, 3515, 3535, 4486, 4603
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use tabloid; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tabloid::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D25
// 首条位置: {"module":1398,"inlinee":27941}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":15880,"rva":7139344,"va":5375848464}
// 编译实例 RVAs: 0x6cf010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ce470
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4197570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848464,"raw_hex":"e83ef3bfff","size":5,"soft_fail":false,"target_va":5371651184,"va":5375848749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4191863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848464,"raw_hex":"e88909c0ff","size":5,"soft_fail":false,"target_va":5371656848,"va":5375848706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3098855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848464,"raw_hex":"e819b7d0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375848546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27129486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848464,"raw_hex":"e88ef69d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375848653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4543560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848464,"raw_hex":"e8b8abbaff","size":5,"soft_fail":false,"target_va":5371305056,"va":5375848611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":16084,"parent_record_offset":15880,"ancestor_record_offset":15880,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":16600,"parent_record_offset":15880,"ancestor_record_offset":15880,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":498,"record_offset":16688,"parent_record_offset":15880,"ancestor_record_offset":15880,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":16972,"parent_record_offset":15880,"ancestor_record_offset":15880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":498,"record_offset":16056,"parent_record_offset":15880,"ancestor_record_offset":15880,"inlinee":9703,"source_line":154,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":16200,"parent_record_offset":16176,"ancestor_record_offset":15880,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":16224,"parent_record_offset":16200,"ancestor_record_offset":15880,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":16148,"parent_record_offset":16112,"ancestor_record_offset":15880,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":16176,"parent_record_offset":16148,"ancestor_record_offset":15880,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":498,"record_offset":16112,"parent_record_offset":16084,"ancestor_record_offset":15880,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":498,"record_offset":16756,"parent_record_offset":16732,"ancestor_record_offset":15880,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":17032,"parent_record_offset":16996,"ancestor_record_offset":15880,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":16444,"parent_record_offset":16416,"ancestor_record_offset":15880,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":498,"record_offset":16732,"parent_record_offset":16688,"ancestor_record_offset":15880,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":16292,"parent_record_offset":16112,"ancestor_record_offset":15880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":498,"record_offset":17064,"parent_record_offset":17032,"ancestor_record_offset":15880,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":15880,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":16788,"parent_record_offset":16688,"ancestor_record_offset":15880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":16628,"parent_record_offset":16600,"ancestor_record_offset":15880,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":16480,"parent_record_offset":16444,"ancestor_record_offset":15880,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":15880,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":16504,"parent_record_offset":16480,"ancestor_record_offset":15880,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":16536,"parent_record_offset":16444,"ancestor_record_offset":15880,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":13904,"rva":7138480,"va":5375847600}
// 编译实例 RVAs: 0x6cecb0, 0x1ae0980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa90
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4191126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e86a0cc0ff","size":5,"soft_fail":false,"target_va":5371656848,"va":5375847969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3098152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e8d8b9d0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375847843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d704f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6952950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e8f6176a00","size":5,"soft_fail":false,"target_va":5382800624,"va":5375847669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1414e5130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6273178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396892032,"raw_hex":"e86647a0ff","size":5,"soft_fail":false,"target_va":5390618928,"va":5396892101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27745668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e8845da701","size":5,"soft_fail":false,"target_va":5403593360,"va":5375847687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27130003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e893f89d01","size":5,"soft_fail":false,"target_va":5402978144,"va":5375848136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4543027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e8cdadbaff","size":5,"soft_fail":false,"target_va":5371305056,"va":5375848078,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t27121213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e83dd69d01","size":5,"soft_fail":false,"target_va":5402968992,"va":5375847774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t27122765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375847600,"raw_hex":"e84ddc9d01","size":5,"soft_fail":false,"target_va":5402970800,"va":5375848030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":14824,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":14284,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":498,"record_offset":15268,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":498,"record_offset":14096,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":14540,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":14484,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":14912,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":498,"record_offset":14208,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":498,"record_offset":14312,"parent_record_offset":13904,"ancestor_record_offset":13904,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":13904,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":13904,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":13904,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14688,"parent_record_offset":14656,"ancestor_record_offset":13904,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":498,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":13904,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":498,"record_offset":15332,"parent_record_offset":15308,"ancestor_record_offset":13904,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":14600,"parent_record_offset":14572,"ancestor_record_offset":13904,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":13904,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":498,"record_offset":15308,"parent_record_offset":15268,"ancestor_record_offset":13904,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":14508,"parent_record_offset":14484,"ancestor_record_offset":13904,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":14936,"parent_record_offset":14912,"ancestor_record_offset":13904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":498,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":13904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":498,"record_offset":14248,"parent_record_offset":14208,"ancestor_record_offset":13904,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":14764,"parent_record_offset":14736,"ancestor_record_offset":13904,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":15364,"parent_record_offset":15268,"ancestor_record_offset":13904,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":498,"record_offset":14420,"parent_record_offset":14336,"ancestor_record_offset":13904,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":498,"record_offset":14444,"parent_record_offset":14420,"ancestor_record_offset":13904,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":498,"record_offset":14120,"parent_record_offset":14096,"ancestor_record_offset":13904,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":498,"record_offset":14144,"parent_record_offset":14120,"ancestor_record_offset":13904,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":15172,"parent_record_offset":15140,"ancestor_record_offset":13904,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":14364,"parent_record_offset":14336,"ancestor_record_offset":13904,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":14852,"parent_record_offset":14824,"ancestor_record_offset":13904,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":14996,"parent_record_offset":14960,"ancestor_record_offset":13904,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":13904,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":15196,"parent_record_offset":15172,"ancestor_record_offset":13904,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":14388,"parent_record_offset":14364,"ancestor_record_offset":13904,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":15020,"parent_record_offset":14996,"ancestor_record_offset":13904,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":15052,"parent_record_offset":14960,"ancestor_record_offset":13904,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":498,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":13904,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":498,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":13904,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":498,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":13904,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$1::publish::closure$1
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":498,"record_offset":14572,"parent_record_offset":14540,"ancestor_record_offset":13904,"inlinee":27937,"source_line":107,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 85
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF000
// 首条位置: {"module":1308,"inlinee":61440}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1308,"record_offset":57572,"parent_record_offset":57532,"ancestor_record_offset":57532,"inlinee":8523,"source_line":1703,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1308,"record_offset":57692,"parent_record_offset":57532,"ancestor_record_offset":57532,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1308,"record_offset":57948,"parent_record_offset":57532,"ancestor_record_offset":57532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$1::fold
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1308,"record_offset":57892,"parent_record_offset":57532,"ancestor_record_offset":57532,"inlinee":61442,"source_line":275,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1308,"record_offset":57720,"parent_record_offset":57532,"ancestor_record_offset":57532,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":2281,"record_offset":5972,"parent_record_offset":5608,"ancestor_record_offset":5608,"inlinee":9721,"source_line":1660,"archive":"_pdb/A/modules/02281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1308,"record_offset":57596,"parent_record_offset":57572,"ancestor_record_offset":57532,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":2281,"record_offset":5996,"parent_record_offset":5972,"ancestor_record_offset":5608,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/02281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1308,"record_offset":57628,"parent_record_offset":57596,"ancestor_record_offset":57532,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1308,"record_offset":57828,"parent_record_offset":57744,"ancestor_record_offset":57532,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1308,"record_offset":57852,"parent_record_offset":57828,"ancestor_record_offset":57532,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":58028,"parent_record_offset":57996,"ancestor_record_offset":57532,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":57772,"parent_record_offset":57744,"ancestor_record_offset":57532,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":58052,"parent_record_offset":58028,"ancestor_record_offset":57532,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":57796,"parent_record_offset":57772,"ancestor_record_offset":57532,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1308,"record_offset":57656,"parent_record_offset":57596,"ancestor_record_offset":57532,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1308,"record_offset":57916,"parent_record_offset":57892,"ancestor_record_offset":57532,"inlinee":59460,"source_line":165,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1308,"record_offset":57972,"parent_record_offset":57948,"ancestor_record_offset":57532,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1308,"record_offset":57744,"parent_record_offset":57720,"ancestor_record_offset":57532,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1308,"record_offset":57996,"parent_record_offset":57972,"ancestor_record_offset":57532,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6D21
// 首条位置: {"module":498,"inlinee":27937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":14600,"parent_record_offset":14572,"ancestor_record_offset":14572,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14712,"parent_record_offset":14688,"ancestor_record_offset":14572,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14736,"parent_record_offset":14712,"ancestor_record_offset":14572,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14656,"parent_record_offset":14628,"ancestor_record_offset":14572,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":498,"record_offset":14688,"parent_record_offset":14656,"ancestor_record_offset":14572,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":498,"record_offset":14628,"parent_record_offset":14600,"ancestor_record_offset":14572,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":14764,"parent_record_offset":14736,"ancestor_record_offset":14572,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x534D
// 首条位置: {"module":360,"inlinee":21325}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":360,"record_offset":10500,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":360,"record_offset":10240,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":360,"record_offset":10412,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":360,"record_offset":10640,"parent_record_offset":10208,"ancestor_record_offset":10208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":360,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":10208,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":360,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":10208,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":360,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":10208,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":360,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":10208,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":360,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":10208,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":360,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":10208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":360,"record_offset":10440,"parent_record_offset":10412,"ancestor_record_offset":10208,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":360,"record_offset":10468,"parent_record_offset":10412,"ancestor_record_offset":10208,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":360,"record_offset":10340,"parent_record_offset":10312,"ancestor_record_offset":10208,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":360,"record_offset":10364,"parent_record_offset":10340,"ancestor_record_offset":10208,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":360,"record_offset":10756,"parent_record_offset":10664,"ancestor_record_offset":10208,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":360,"record_offset":10288,"parent_record_offset":10264,"ancestor_record_offset":10208,"inlinee":5734,"source_line":2431,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":360,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":10208,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00360.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$3::deref
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x534B
// 首条位置: {"module":360,"inlinee":21323}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=3
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":498,"record_offset":17164,"rva":7139696,"va":5375848816}
// 编译实例 RVAs: 0x6cf170, 0xc96b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cfa90
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4192290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848816,"raw_hex":"e8de07c0ff","size":5,"soft_fail":false,"target_va":5371656848,"va":5375849133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t27119830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848816,"raw_hex":"e8d6d09d01","size":5,"soft_fail":false,"target_va":5402968992,"va":5375849157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t27121715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848816,"raw_hex":"e833d89d01","size":5,"soft_fail":false,"target_va":5402970800,"va":5375849080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t27117789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848816,"raw_hex":"e8ddc89d01","size":5,"soft_fail":false,"target_va":5402966736,"va":5375848942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t27119355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375848816,"raw_hex":"e8fbce9d01","size":5,"soft_fail":false,"target_va":5402968352,"va":5375848992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":18112,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":9128,"source_line":1485,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":17344,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":498,"record_offset":17552,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":17968,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":498,"record_offset":17372,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":498,"record_offset":17692,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":498,"record_offset":18448,"parent_record_offset":17164,"ancestor_record_offset":17164,"inlinee":5728,"source_line":1206,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":17164,"inlinee":9129,"source_line":1510,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":19068,"parent_record_offset":19032,"ancestor_record_offset":17164,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":498,"record_offset":18880,"parent_record_offset":18856,"ancestor_record_offset":17164,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":498,"record_offset":18472,"parent_record_offset":18448,"ancestor_record_offset":17164,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":498,"record_offset":17576,"parent_record_offset":17552,"ancestor_record_offset":17164,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":498,"record_offset":17832,"parent_record_offset":17744,"ancestor_record_offset":17164,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":498,"record_offset":17864,"parent_record_offset":17832,"ancestor_record_offset":17164,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":498,"record_offset":19100,"parent_record_offset":19068,"ancestor_record_offset":17164,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":17600,"parent_record_offset":17576,"ancestor_record_offset":17164,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":498,"record_offset":17628,"parent_record_offset":17576,"ancestor_record_offset":17164,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":498,"record_offset":17488,"parent_record_offset":17400,"ancestor_record_offset":17164,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":498,"record_offset":17512,"parent_record_offset":17488,"ancestor_record_offset":17164,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":18584,"parent_record_offset":18552,"ancestor_record_offset":17164,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":17432,"parent_record_offset":17400,"ancestor_record_offset":17164,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":17900,"parent_record_offset":17744,"ancestor_record_offset":17164,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":18044,"parent_record_offset":18016,"ancestor_record_offset":17164,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":17776,"parent_record_offset":17744,"ancestor_record_offset":17164,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":18608,"parent_record_offset":18584,"ancestor_record_offset":17164,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":17164,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":17800,"parent_record_offset":17776,"ancestor_record_offset":17164,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":18068,"parent_record_offset":18044,"ancestor_record_offset":17164,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":498,"record_offset":18972,"parent_record_offset":18880,"ancestor_record_offset":17164,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":498,"record_offset":18528,"parent_record_offset":18504,"ancestor_record_offset":17164,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":498,"record_offset":17992,"parent_record_offset":17968,"ancestor_record_offset":17164,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":498,"record_offset":17400,"parent_record_offset":17372,"ancestor_record_offset":17164,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":498,"record_offset":18552,"parent_record_offset":18528,"ancestor_record_offset":17164,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":498,"record_offset":17744,"parent_record_offset":17716,"ancestor_record_offset":17164,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":498,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":17164,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00498.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":498,"record_offset":18016,"parent_record_offset":17992,"ancestor_record_offset":17164,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00498.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publication
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 19
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publication", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::PublicationReader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publisher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscriber
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscription
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscription", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 文件级候选接口: 99; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 1f953b6b90db5e31af188d6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Publisher::publish -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::flush_colliders @ 0x588e4f -> 0x6cecb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeights::copy_from_gpu @ 0xc8db9f -> 0x6cecb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$1::copy_to_staging<512> @ 0x1032f23 -> 0x6cecb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0x14e49db -> 0x1ae0980 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$1::copy_to_staging<512> @ 0x1db8873 -> 0x1ae0980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 7dd6458b515159d319483252
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4cd2a4be13acb499e48dca4a: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] tabloid::Publisher::subscribe -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 10 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0x57b8ca -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> @ 0x57b8e3 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::subscribe_to_changes @ 0x589084 -> 0x6cf010 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeights::subscribe @ 0xc8d584 -> 0x6cf010 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0xe6f2d6 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0xe6f2fa -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids @ 0x1ad4d72 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids @ 0x1ad4dc1 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids @ 0x1ad4e11 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::clutter_subscribe_to_tabloids @ 0x1ad4e61 -> 0x6cf010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 16f323828bb2cb4b769b6c58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Subscriber::take -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1> @ 0x6d112d -> 0x6cf170 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0> @ 0x6d11ed -> 0x6cf170 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x4d4563 -> 0xc96b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions @ 0x1ad5457 -> 0xc96b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$0::default
// 信息 ID: feb5f7b8a0e4236049b2e413
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::world_raster::WorldRaster::new<512>; module=1398, inline_record=5240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$0
// 信息 ID: 34266af37ab613be4d6effa6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=57532; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=1308, inline_record=58520; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=2281, inline_record=5608; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=2281, inline_record=6784; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=4603, inline_record=1428; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles> >; module=4603, inline_record=2604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$1
// 信息 ID: a7fd02bc47f7d172ad790264
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>; module=498, inline_record=14572; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>; module=3535, inline_record=920; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles>; module=4486, inline_record=920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$2::drop
// 信息 ID: a5029e1033574cbf58ccf92b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=360, inline_record=10208; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>; module=498, inline_record=26096; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::dirty_tiles::impl$2::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>; module=498, inline_record=26972; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=500, inline_record=831628; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=895, inline_record=122828; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=1210, inline_record=43560; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions; module=3515, inline_record=11892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$3::deref
// 信息 ID: 80b00bb5a1f50e006d20686f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=360, inline_record=10156; 不是独立调用边。
// [PDB 内联宿主] system_clutter::clutter::populate_clutter_regions::populate_clutter_regions; module=3515, inline_record=11840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
