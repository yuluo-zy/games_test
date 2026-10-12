// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/temp_list.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 840257c08788607892f65689e4b13afbab07216dab4c049149474db7a6b1c8d6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: temp_list.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 416, 554, 570, 1332, 1594, 3009
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 9
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CC7
// 首条位置: {"module":416,"inlinee":23751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] arrayvec::arrayvec::impl$36::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":7800,"inlinee":23753,"source_line":1208,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7800,"inlinee":23754,"source_line":82,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x794D
// 首条位置: {"module":570,"inlinee":31053}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":570,"record_offset":83292,"parent_record_offset":83264,"ancestor_record_offset":83264,"inlinee":23788,"source_line":26,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83432,"parent_record_offset":83408,"ancestor_record_offset":83264,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83456,"parent_record_offset":83432,"ancestor_record_offset":83264,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83376,"parent_record_offset":83348,"ancestor_record_offset":83264,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83408,"parent_record_offset":83376,"ancestor_record_offset":83264,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":83348,"parent_record_offset":83320,"ancestor_record_offset":83264,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":83320,"parent_record_offset":83292,"ancestor_record_offset":83264,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":83480,"parent_record_offset":83456,"ancestor_record_offset":83264,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::TempList::new
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CEC
// 首条位置: {"module":554,"inlinee":23788}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":554,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":4076,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":4076,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":4076,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4160,"parent_record_offset":4132,"ancestor_record_offset":4076,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":4076,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":554,"record_offset":4132,"parent_record_offset":4104,"ancestor_record_offset":4076,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7728,"inlinee":23754,"source_line":82,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$36::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":7728,"inlinee":23753,"source_line":1208,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":4076,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::temp_list::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1332,"record_offset":7800,"parent_record_offset":7764,"ancestor_record_offset":7728,"inlinee":23751,"source_line":9,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::TempList::add
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5CEB
// 首条位置: {"module":1594,"inlinee":23787}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1594,"record_offset":3216,"parent_record_offset":2920,"ancestor_record_offset":2920,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":2972,"parent_record_offset":2920,"ancestor_record_offset":2920,"inlinee":23745,"source_line":204,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3120,"parent_record_offset":2920,"ancestor_record_offset":2920,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":3400,"parent_record_offset":2920,"ancestor_record_offset":2920,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2920,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":2920,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":2920,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":2920,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2920,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3080,"parent_record_offset":3024,"ancestor_record_offset":2920,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":2920,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":2920,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":2920,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":2920,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":2920,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3352,"parent_record_offset":3324,"ancestor_record_offset":2920,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2920,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::TempList::add<alloc::ffi::c_str::CString>
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":416,"record_offset":2324,"rva":5783024,"va":5374492144}
// 编译实例 RVAs: 0x583df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1742600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e8f868e5ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374492291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d9f30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1400938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e86a601500","size":5,"soft_fail":false,"target_va":5375893296,"va":5374492353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1493604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e864ca1600","size":5,"soft_fail":false,"target_va":5375986096,"va":5374492487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29101034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e8ea0bbc01","size":5,"soft_fail":false,"target_va":5403593360,"va":5374492321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28485694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e83ea8b201","size":5,"soft_fail":false,"target_va":5402978144,"va":5374492445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28486925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374492144,"raw_hex":"e80dadb201","size":5,"soft_fail":false,"target_va":5402979344,"va":5374492414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":416,"record_offset":2740,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":416,"record_offset":2492,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":23745,"source_line":204,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":416,"record_offset":2644,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":2928,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":3044,"parent_record_offset":2324,"ancestor_record_offset":2324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2324,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":2324,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":2772,"parent_record_offset":2740,"ancestor_record_offset":2324,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":416,"record_offset":2804,"parent_record_offset":2772,"ancestor_record_offset":2324,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":416,"record_offset":2672,"parent_record_offset":2644,"ancestor_record_offset":2324,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":416,"record_offset":2604,"parent_record_offset":2540,"ancestor_record_offset":2324,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":416,"record_offset":2700,"parent_record_offset":2672,"ancestor_record_offset":2324,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":416,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":2324,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":416,"record_offset":2516,"parent_record_offset":2492,"ancestor_record_offset":2324,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":416,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2324,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2324,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":416,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":2324,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":2324,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2324,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00416.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":416,"record_offset":3008,"parent_record_offset":2952,"ancestor_record_offset":2324,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00416.jsonl.gz"}
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
// 完整结构及原始类型记录: temp_list.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::temp_list::TempList
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::temp_list::TempList::add<alloc::ffi::c_str::CString>", "constructor": "rhapsody::raw::temp_list::TempList::add", "arguments": ["alloc::ffi::c_str::CString"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::temp_list::TempListInner
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>", "constructor": "rhapsody::raw::temp_list::TempListInner", "arguments": ["alloc::ffi::c_str::CString"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::temp_list::TempListInner<alloc::ffi::c_str::CString>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::temp_list::TempListInner<ash::vk::definitions::Sampler>", "constructor": "rhapsody::raw::temp_list::TempListInner", "arguments": ["ash::vk::definitions::Sampler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<ash::vk::definitions::Sampler>,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["rhapsody::raw::temp_list::TempListInner<ash::vk::definitions::Sampler>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<ash::vk::definitions::Sampler>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<rhapsody::raw::temp_list::TempListInner<ash::vk::definitions::Sampler>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: temp_list.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: temp_list.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::add
// 信息 ID: a7a654f684053fc83090847e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >; module=1594, inline_record=2920; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >; module=1594, inline_record=7448; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::add<alloc::ffi::c_str::CString>
// 信息 ID: 1450b7754d44832613fd4542
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::temp_list::TempList::add -> ["alloc::ffi::c_str::CString"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > > @ 0xde4a85 -> 0x583df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::new
// 信息 ID: 7a378c49e5242ac98427d39e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layouts; module=554, inline_record=4076; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=83292; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=7728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::impl$0::default
// 信息 ID: 8a230b423cd357b6210465ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=7800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::impl$1::default
// 信息 ID: d2ebf83727fe3f4c1f214a1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=83264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
