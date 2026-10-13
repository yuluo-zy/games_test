// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/temp_list.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 840257c08788607892f65689e4b13afbab07216dab4c049149474db7a6b1c8d6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: temp_list.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 504, 693, 703, 1657, 1965, 3312
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B5A
// 首条位置: {"module":703,"inlinee":39770}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":703,"record_offset":142800,"parent_record_offset":142772,"ancestor_record_offset":142772,"inlinee":28564,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142964,"parent_record_offset":142940,"ancestor_record_offset":142772,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142988,"parent_record_offset":142964,"ancestor_record_offset":142772,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142940,"parent_record_offset":142916,"ancestor_record_offset":142772,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142856,"parent_record_offset":142828,"ancestor_record_offset":142772,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142884,"parent_record_offset":142856,"ancestor_record_offset":142772,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142916,"parent_record_offset":142884,"ancestor_record_offset":142772,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":143064,"parent_record_offset":142828,"ancestor_record_offset":142772,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142828,"parent_record_offset":142800,"ancestor_record_offset":142772,"inlinee":21073,"source_line":1695,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":703,"record_offset":143088,"parent_record_offset":143064,"ancestor_record_offset":142772,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":143012,"parent_record_offset":142988,"ancestor_record_offset":142772,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::TempList::new
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F94
// 首条位置: {"module":693,"inlinee":28564}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4936,"parent_record_offset":4908,"ancestor_record_offset":4908,"inlinee":21073,"source_line":1695,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5072,"parent_record_offset":5048,"ancestor_record_offset":4908,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5096,"parent_record_offset":5072,"ancestor_record_offset":4908,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":693,"record_offset":5048,"parent_record_offset":5024,"ancestor_record_offset":4908,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4964,"parent_record_offset":4936,"ancestor_record_offset":4908,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":4992,"parent_record_offset":4964,"ancestor_record_offset":4908,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":5024,"parent_record_offset":4992,"ancestor_record_offset":4908,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":693,"record_offset":5172,"parent_record_offset":4936,"ancestor_record_offset":4908,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":693,"record_offset":5196,"parent_record_offset":5172,"ancestor_record_offset":4908,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":693,"record_offset":5120,"parent_record_offset":5096,"ancestor_record_offset":4908,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00693.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::temp_list::TempList::add
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F93
// 首条位置: {"module":1965,"inlinee":28563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1965,"record_offset":4532,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4224,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":28486,"source_line":204,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4372,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1965,"record_offset":4716,"parent_record_offset":4172,"ancestor_record_offset":4172,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4172,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4172,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":4560,"parent_record_offset":4532,"ancestor_record_offset":4172,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1965,"record_offset":4592,"parent_record_offset":4560,"ancestor_record_offset":4172,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4400,"parent_record_offset":4372,"ancestor_record_offset":4172,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4484,"parent_record_offset":4456,"ancestor_record_offset":4172,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4456,"parent_record_offset":4428,"ancestor_record_offset":4172,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1965,"record_offset":4332,"parent_record_offset":4276,"ancestor_record_offset":4172,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4428,"parent_record_offset":4400,"ancestor_record_offset":4172,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4276,"parent_record_offset":4248,"ancestor_record_offset":4172,"inlinee":28488,"source_line":54,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1965,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":4172,"inlinee":28487,"source_line":43,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":4668,"parent_record_offset":4640,"ancestor_record_offset":4172,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01965.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1965,"record_offset":4304,"parent_record_offset":4276,"ancestor_record_offset":4172,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01965.jsonl.gz"}
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
// 首条位置: {"module":504,"record_offset":2432,"rva":9493920,"va":5378203040}
// 编译实例 RVAs: 0x90dda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3121696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e8e05dd0ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5378203179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14087add0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-602266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e866cff6ff","size":5,"soft_fail":false,"target_va":5377600976,"va":5378203237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408985f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-481532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e804a7f8ff","size":5,"soft_fail":false,"target_va":5377721840,"va":5378203367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: temp_list.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33643958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e8b65d0102","size":5,"soft_fail":false,"target_va":5411847168,"va":5378203205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33337667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e843b1fc01","size":5,"soft_fail":false,"target_va":5411540992,"va":5378203320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33338986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378203040,"raw_hex":"e86ab6fc01","size":5,"soft_fail":false,"target_va":5411542296,"va":5378203305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":504,"record_offset":2884,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2600,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":28486,"source_line":204,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2752,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":26717,"source_line":732,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":504,"record_offset":3064,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":6271,"source_line":730,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":504,"record_offset":3092,"parent_record_offset":2432,"ancestor_record_offset":2432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":2432,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2432,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":2916,"parent_record_offset":2884,"ancestor_record_offset":2432,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":504,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":2432,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2432,"inlinee":26718,"source_line":689,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2844,"parent_record_offset":2824,"ancestor_record_offset":2432,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2432,"inlinee":26720,"source_line":713,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":504,"record_offset":2712,"parent_record_offset":2648,"ancestor_record_offset":2432,"inlinee":6210,"source_line":715,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":504,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2432,"inlinee":26719,"source_line":17,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":504,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2432,"inlinee":28488,"source_line":54,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":504,"record_offset":2624,"parent_record_offset":2600,"ancestor_record_offset":2432,"inlinee":28487,"source_line":43,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":504,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2432,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":504,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":2432,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00504.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":504,"record_offset":3016,"parent_record_offset":2992,"ancestor_record_offset":2432,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00504.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::add
// 信息 ID: 93da395015644bca9699a170
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >; module=1965, inline_record=4172; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >; module=1965, inline_record=9208; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::add<alloc::ffi::c_str::CString>
// 信息 ID: ee22b9fceb2d0a10efc58827
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::temp_list::TempList::add -> ["alloc::ffi::c_str::CString"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$20::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > > @ 0x147712a -> 0x90dda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::TempList::new
// 信息 ID: 0957ac23a62a2b9dfcf70c56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::shader::create_descriptor_set_layouts; module=693, inline_record=4908; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=703, inline_record=142800; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1657, inline_record=11084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::temp_list::impl$1::default
// 信息 ID: 08f3d388551acfa92e629334
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=703, inline_record=142772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
