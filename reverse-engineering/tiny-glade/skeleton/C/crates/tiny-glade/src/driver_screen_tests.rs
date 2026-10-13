// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/driver_screen_tests.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: d9b07326078b3da57d5daa5932aed5168983414292091db09d67b183742d39cd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: driver_screen_tests.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 199
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use typed_index_collections; // namespace candidate
//   package identities: [{"name":"typed-index-collections","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.4.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_amd_stronk
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":11740,"rva":5650288,"va":5374359408}
// 编译实例 RVAs: 0x563770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1319237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359408,"raw_hex":"e845211400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374359446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1318860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359408,"raw_hex":"e8cc1f1400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374359839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5637244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359408,"raw_hex":"e884fba9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374359431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42261036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359408,"raw_hex":"e82cda8402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374359753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9699438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359408,"raw_hex":"e86e009400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374359485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":11876,"parent_record_offset":11740,"ancestor_record_offset":11740,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":13504,"parent_record_offset":11740,"ancestor_record_offset":11740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":11740,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":11740,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":11740,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":11740,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":11740,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":11740,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":12140,"parent_record_offset":12112,"ancestor_record_offset":11740,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":11740,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":12052,"parent_record_offset":12024,"ancestor_record_offset":11740,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":13624,"parent_record_offset":13600,"ancestor_record_offset":11740,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":12112,"parent_record_offset":12080,"ancestor_record_offset":11740,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":12080,"parent_record_offset":12052,"ancestor_record_offset":11740,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":11740,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":11932,"parent_record_offset":11904,"ancestor_record_offset":11740,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":11960,"parent_record_offset":11932,"ancestor_record_offset":11740,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":11988,"parent_record_offset":11960,"ancestor_record_offset":11740,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":11904,"parent_record_offset":11876,"ancestor_record_offset":11740,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":12024,"parent_record_offset":11988,"ancestor_record_offset":11740,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":12268,"parent_record_offset":11988,"ancestor_record_offset":11740,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":11740,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":13528,"parent_record_offset":13504,"ancestor_record_offset":11740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_intel_old
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":7140,"rva":5649280,"va":5374358400}
// 编译实例 RVAs: 0x563380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1320242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358400,"raw_hex":"e832251400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374358441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1319867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358400,"raw_hex":"e8bb231400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374358832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5636239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358400,"raw_hex":"e871ffa9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374358426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42262045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358400,"raw_hex":"e81dde8402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374358744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9700444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358400,"raw_hex":"e85c049400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374358479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":7276,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":8904,"parent_record_offset":7140,"ancestor_record_offset":7140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":7140,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7140,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9072,"parent_record_offset":9048,"ancestor_record_offset":7140,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":7140,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":7140,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9120,"parent_record_offset":9096,"ancestor_record_offset":7140,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":7140,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9048,"parent_record_offset":9024,"ancestor_record_offset":7140,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":7140,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":7140,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":7512,"parent_record_offset":7480,"ancestor_record_offset":7140,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":7480,"parent_record_offset":7452,"ancestor_record_offset":7140,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":9000,"parent_record_offset":8976,"ancestor_record_offset":7140,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":7332,"parent_record_offset":7304,"ancestor_record_offset":7140,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":7360,"parent_record_offset":7332,"ancestor_record_offset":7140,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":7388,"parent_record_offset":7360,"ancestor_record_offset":7140,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":7304,"parent_record_offset":7276,"ancestor_record_offset":7140,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":7424,"parent_record_offset":7388,"ancestor_record_offset":7140,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":7668,"parent_record_offset":7388,"ancestor_record_offset":7140,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":7140,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":7140,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_intel_weak
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":14040,"rva":5650784,"va":5374359904}
// 编译实例 RVAs: 0x563960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1318738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359904,"raw_hex":"e8521f1400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374359945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1318347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359904,"raw_hex":"e8cb1d1400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374360352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5637743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359904,"raw_hex":"e891f9a9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374359930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42260530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359904,"raw_hex":"e832d88402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374360259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9698929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374359904,"raw_hex":"e871fe9300","size":5,"soft_fail":false,"target_va":5384058928,"va":5374359994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":14176,"parent_record_offset":14040,"ancestor_record_offset":14040,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":15804,"parent_record_offset":14040,"ancestor_record_offset":14040,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":14464,"parent_record_offset":14440,"ancestor_record_offset":14040,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":14488,"parent_record_offset":14464,"ancestor_record_offset":14040,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":14040,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":15996,"parent_record_offset":15972,"ancestor_record_offset":14040,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":14512,"parent_record_offset":14488,"ancestor_record_offset":14040,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":16020,"parent_record_offset":15996,"ancestor_record_offset":14040,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":14040,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":15948,"parent_record_offset":15924,"ancestor_record_offset":14040,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":14352,"parent_record_offset":14324,"ancestor_record_offset":14040,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":15924,"parent_record_offset":15900,"ancestor_record_offset":14040,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":14412,"parent_record_offset":14380,"ancestor_record_offset":14040,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":14380,"parent_record_offset":14352,"ancestor_record_offset":14040,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":15900,"parent_record_offset":15876,"ancestor_record_offset":14040,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":14232,"parent_record_offset":14204,"ancestor_record_offset":14040,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":14260,"parent_record_offset":14232,"ancestor_record_offset":14040,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":14288,"parent_record_offset":14260,"ancestor_record_offset":14040,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":14204,"parent_record_offset":14176,"ancestor_record_offset":14040,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":14324,"parent_record_offset":14288,"ancestor_record_offset":14040,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":14568,"parent_record_offset":14288,"ancestor_record_offset":14040,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":14592,"parent_record_offset":14568,"ancestor_record_offset":14040,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":15828,"parent_record_offset":15804,"ancestor_record_offset":14040,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_ms_dozen
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":4840,"rva":5648752,"va":5374357872}
// 编译实例 RVAs: 0x563170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1320773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357872,"raw_hex":"e845271400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374357910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1320364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357872,"raw_hex":"e8ac251400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374358335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5635708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357872,"raw_hex":"e88401aaff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374357895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42262533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357872,"raw_hex":"e805e08402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374358256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9700948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357872,"raw_hex":"e854069400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374357975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":4976,"parent_record_offset":4840,"ancestor_record_offset":4840,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":6604,"parent_record_offset":4840,"ancestor_record_offset":4840,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":4840,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":4840,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6772,"parent_record_offset":6748,"ancestor_record_offset":4840,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":4840,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":4840,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":4840,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":5240,"parent_record_offset":5212,"ancestor_record_offset":4840,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":6748,"parent_record_offset":6724,"ancestor_record_offset":4840,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":4840,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":6724,"parent_record_offset":6700,"ancestor_record_offset":4840,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":5212,"parent_record_offset":5180,"ancestor_record_offset":4840,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":5180,"parent_record_offset":5152,"ancestor_record_offset":4840,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":6700,"parent_record_offset":6676,"ancestor_record_offset":4840,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":5032,"parent_record_offset":5004,"ancestor_record_offset":4840,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":5060,"parent_record_offset":5032,"ancestor_record_offset":4840,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":4840,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":5004,"parent_record_offset":4976,"ancestor_record_offset":4840,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":5124,"parent_record_offset":5088,"ancestor_record_offset":4840,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":5368,"parent_record_offset":5088,"ancestor_record_offset":4840,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":4840,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":6628,"parent_record_offset":6604,"ancestor_record_offset":4840,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_nv_oom
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":248,"rva":5647744,"va":5374356864}
// 编译实例 RVAs: 0x562d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1321778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374356864,"raw_hex":"e8322b1400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374356905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1321403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374356864,"raw_hex":"e8bb291400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374357296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5634703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374356864,"raw_hex":"e87105aaff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374356890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42263574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374356864,"raw_hex":"e816e48402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374357215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9701979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374356864,"raw_hex":"e85b0a9400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374356944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":380,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":2008,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":668,"parent_record_offset":644,"ancestor_record_offset":248,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":692,"parent_record_offset":668,"ancestor_record_offset":248,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":248,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2224,"parent_record_offset":2200,"ancestor_record_offset":248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":248,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2152,"parent_record_offset":2128,"ancestor_record_offset":248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":556,"parent_record_offset":528,"ancestor_record_offset":248,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":248,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":248,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":436,"parent_record_offset":408,"ancestor_record_offset":248,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":464,"parent_record_offset":436,"ancestor_record_offset":248,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":492,"parent_record_offset":464,"ancestor_record_offset":248,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":408,"parent_record_offset":380,"ancestor_record_offset":248,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":528,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":772,"parent_record_offset":492,"ancestor_record_offset":248,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":796,"parent_record_offset":772,"ancestor_record_offset":248,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":2032,"parent_record_offset":2008,"ancestor_record_offset":248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_nv_stronk
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":9440,"rva":5649776,"va":5374358896}
// 编译实例 RVAs: 0x563570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1319746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358896,"raw_hex":"e842231400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374358937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1319355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358896,"raw_hex":"e8bb211400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374359344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5636735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358896,"raw_hex":"e881fda9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374358922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42261529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358896,"raw_hex":"e819dc8402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374359260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9699940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374358896,"raw_hex":"e864029400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374358983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":9576,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":11204,"parent_record_offset":9440,"ancestor_record_offset":9440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":9440,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":9440,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":11372,"parent_record_offset":11348,"ancestor_record_offset":9440,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":11396,"parent_record_offset":11372,"ancestor_record_offset":9440,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9912,"parent_record_offset":9888,"ancestor_record_offset":9440,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":9440,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":9840,"parent_record_offset":9812,"ancestor_record_offset":9440,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":11348,"parent_record_offset":11324,"ancestor_record_offset":9440,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":9752,"parent_record_offset":9724,"ancestor_record_offset":9440,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":9440,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":9440,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":9780,"parent_record_offset":9752,"ancestor_record_offset":9440,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":9440,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":9632,"parent_record_offset":9604,"ancestor_record_offset":9440,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":9660,"parent_record_offset":9632,"ancestor_record_offset":9440,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":9688,"parent_record_offset":9660,"ancestor_record_offset":9440,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":9604,"parent_record_offset":9576,"ancestor_record_offset":9440,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":9724,"parent_record_offset":9688,"ancestor_record_offset":9440,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":9968,"parent_record_offset":9688,"ancestor_record_offset":9440,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":9992,"parent_record_offset":9968,"ancestor_record_offset":9440,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":9440,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_nv_weak
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":2544,"rva":5648240,"va":5374357360}
// 编译实例 RVAs: 0x562f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1321282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357360,"raw_hex":"e842291400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374357401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1320891","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357360,"raw_hex":"e8bb271400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374357808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5635199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357360,"raw_hex":"e88103aaff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374357386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42263070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357360,"raw_hex":"e81ee28402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374357719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9701476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374357360,"raw_hex":"e864089400","size":5,"soft_fail":false,"target_va":5384058928,"va":5374357447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":2676,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":4304,"parent_record_offset":2544,"ancestor_record_offset":2544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2964,"parent_record_offset":2940,"ancestor_record_offset":2544,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2988,"parent_record_offset":2964,"ancestor_record_offset":2544,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":2544,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":4496,"parent_record_offset":4472,"ancestor_record_offset":2544,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2544,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":2544,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2544,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":4448,"parent_record_offset":4424,"ancestor_record_offset":2544,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2544,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":4424,"parent_record_offset":4400,"ancestor_record_offset":2544,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":2912,"parent_record_offset":2880,"ancestor_record_offset":2544,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2544,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":2544,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":2732,"parent_record_offset":2704,"ancestor_record_offset":2544,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":2760,"parent_record_offset":2732,"ancestor_record_offset":2544,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":2788,"parent_record_offset":2760,"ancestor_record_offset":2544,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":2704,"parent_record_offset":2676,"ancestor_record_offset":2544,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":2824,"parent_record_offset":2788,"ancestor_record_offset":2544,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":3068,"parent_record_offset":2788,"ancestor_record_offset":2544,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":2544,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":4328,"parent_record_offset":4304,"ancestor_record_offset":2544,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::igpu_amd_ryzen
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":16340,"rva":5651296,"va":5374360416}
// 编译实例 RVAs: 0x563b60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1318229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360416,"raw_hex":"e8551d1400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374360454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1317852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360416,"raw_hex":"e8dc1b1400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374360847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5638252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360416,"raw_hex":"e894f7a9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374360439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t42260025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360416,"raw_hex":"e839d68402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374360764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9698427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360416,"raw_hex":"e87bfc9300","size":5,"soft_fail":false,"target_va":5384058928,"va":5374360496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":16476,"parent_record_offset":16340,"ancestor_record_offset":16340,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":18104,"parent_record_offset":16340,"ancestor_record_offset":16340,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":16764,"parent_record_offset":16740,"ancestor_record_offset":16340,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":16788,"parent_record_offset":16764,"ancestor_record_offset":16340,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":18272,"parent_record_offset":18248,"ancestor_record_offset":16340,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":18296,"parent_record_offset":18272,"ancestor_record_offset":16340,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":16812,"parent_record_offset":16788,"ancestor_record_offset":16340,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":18320,"parent_record_offset":18296,"ancestor_record_offset":16340,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":16740,"parent_record_offset":16712,"ancestor_record_offset":16340,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":16340,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":16652,"parent_record_offset":16624,"ancestor_record_offset":16340,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":18224,"parent_record_offset":18200,"ancestor_record_offset":16340,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":16712,"parent_record_offset":16680,"ancestor_record_offset":16340,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":16680,"parent_record_offset":16652,"ancestor_record_offset":16340,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":16340,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":16532,"parent_record_offset":16504,"ancestor_record_offset":16340,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":16560,"parent_record_offset":16532,"ancestor_record_offset":16340,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":16588,"parent_record_offset":16560,"ancestor_record_offset":16340,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":16504,"parent_record_offset":16476,"ancestor_record_offset":16340,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":16624,"parent_record_offset":16588,"ancestor_record_offset":16340,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":16868,"parent_record_offset":16588,"ancestor_record_offset":16340,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":16892,"parent_record_offset":16868,"ancestor_record_offset":16340,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":16340,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::get_driver_screen_test_case
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":199,"record_offset":18640,"rva":5651792,"va":5374360912}
// 编译实例 RVAs: 0x563d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=35
//     evidence={"assembly":"\tcallq\t1317624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8f81a1400","size":5,"soft_fail":false,"target_va":5375678688,"va":5374361059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=29
//     evidence={"assembly":"\tcallq\t1313917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e87d0c1400","size":5,"soft_fail":false,"target_va":5375678704,"va":5374364782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_realloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1313986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8c20c1400","size":5,"soft_fail":false,"target_va":5375678720,"va":5374364729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=35
//     target_aliases=1699 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5638857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e837f5a9ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5374361044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140092f20
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5057469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e843d4b2ff","size":5,"soft_fail":false,"target_va":5369311008,"va":5374368472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400aa020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4963238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e85a44b4ff","size":5,"soft_fail":false,"target_va":5369405472,"va":5374368705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400b1da0
//     owner=unresolved_namespace; count=12
//     target_aliases=2 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4928102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e89acdb4ff","size":5,"soft_fail":false,"target_va":5369437600,"va":5374365697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c0be0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4868306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e82eb7b5ff","size":5,"soft_fail":false,"target_va":5369498592,"va":5374366893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c1b10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4864406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e86ac6b5ff","size":5,"soft_fail":false,"target_va":5369502480,"va":5374366881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f8c60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2539400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e87840d9ff","size":5,"soft_fail":false,"target_va":5371825248,"va":5374364643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: driver_screen_tests.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42694452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e834778b02","size":5,"soft_fail":false,"target_va":5417057056,"va":5374362599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t42255736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e878c58402","size":5,"soft_fail":false,"target_va":5416620769,"va":5374365028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t42256039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8a7c68402","size":5,"soft_fail":false,"target_va":5416620794,"va":5374364750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<rhapsody::raw::init::GraphicsInitError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e863120000","size":5,"soft_fail":false,"target_va":5374373344,"va":5374368632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_amd_stronk
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e89ef3ffff","size":5,"soft_fail":false,"target_va":5374359408,"va":5374362573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_intel_old
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8c8f1ffff","size":5,"soft_fail":false,"target_va":5374358400,"va":5374362035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_intel_weak
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-2630","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8baf5ffff","size":5,"soft_fail":false,"target_va":5374359904,"va":5374362529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_ms_dozen
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8bdedffff","size":5,"soft_fail":false,"target_va":5374357872,"va":5374362542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_nv_oom
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8e6e9ffff","size":5,"soft_fail":false,"target_va":5374356864,"va":5374362517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_nv_stronk
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8abf1ffff","size":5,"soft_fail":false,"target_va":5374358896,"va":5374362560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::gpu_nv_weak
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8e2e7ffff","size":5,"soft_fail":false,"target_va":5374357360,"va":5374363529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::driver_screen_tests::igpu_amd_ryzen
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e888fbffff","size":5,"soft_fail":false,"target_va":5374360416,"va":5374361555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9697700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374360912,"raw_hex":"e8a4f99300","size":5,"soft_fail":false,"target_va":5384058928,"va":5374361223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":35840,"parent_record_offset":18640,"ancestor_record_offset":18640,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":199,"record_offset":36008,"parent_record_offset":18640,"ancestor_record_offset":18640,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":199,"record_offset":35896,"parent_record_offset":18640,"ancestor_record_offset":18640,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::driver_screen_tests::get_driver_screen_test_case_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":18796,"parent_record_offset":18640,"ancestor_record_offset":18640,"inlinee":15996,"source_line":31,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":18640,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19612,"parent_record_offset":19588,"ancestor_record_offset":18640,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":18640,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":18640,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":36492,"parent_record_offset":36468,"ancestor_record_offset":18640,"inlinee":6570,"source_line":353,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::shrink_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":36516,"parent_record_offset":36492,"ancestor_record_offset":18640,"inlinee":6571,"source_line":264,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":18640,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":18640,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19564,"parent_record_offset":19536,"ancestor_record_offset":18640,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":18640,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::shrink
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":36468,"parent_record_offset":36340,"ancestor_record_offset":18640,"inlinee":6569,"source_line":471,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::realloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":36540,"parent_record_offset":36516,"ancestor_record_offset":18640,"inlinee":6572,"source_line":134,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":29032,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":19792,"parent_record_offset":19768,"ancestor_record_offset":18640,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":32128,"parent_record_offset":32092,"ancestor_record_offset":18640,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":19768,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":32668,"parent_record_offset":32644,"ancestor_record_offset":18640,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36192,"parent_record_offset":36168,"ancestor_record_offset":18640,"inlinee":4787,"source_line":307,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::shrink_to_fit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36252,"parent_record_offset":36136,"ancestor_record_offset":18640,"inlinee":6565,"source_line":398,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19476,"parent_record_offset":19448,"ancestor_record_offset":18640,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36216,"parent_record_offset":36192,"ancestor_record_offset":18640,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36384,"parent_record_offset":36340,"ancestor_record_offset":18640,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":18640,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36312,"parent_record_offset":36280,"ancestor_record_offset":18640,"inlinee":6567,"source_line":811,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_to_fit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36280,"parent_record_offset":36252,"ancestor_record_offset":18640,"inlinee":6566,"source_line":743,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::shrink_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":36340,"parent_record_offset":36312,"ancestor_record_offset":18640,"inlinee":6568,"source_line":827,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19536,"parent_record_offset":19504,"ancestor_record_offset":18640,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19504,"parent_record_offset":19476,"ancestor_record_offset":18640,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21512,"parent_record_offset":21488,"ancestor_record_offset":18640,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":29056,"parent_record_offset":29032,"ancestor_record_offset":18640,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19356,"parent_record_offset":19328,"ancestor_record_offset":18640,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":18640,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":18640,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19328,"parent_record_offset":19300,"ancestor_record_offset":18640,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":19300,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":199,"record_offset":32092,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":36168,"parent_record_offset":36136,"ancestor_record_offset":18640,"inlinee":6564,"source_line":1421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":18640,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::into_boxed_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":36100,"parent_record_offset":36072,"ancestor_record_offset":18640,"inlinee":6562,"source_line":1708,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":19244,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::shrink_to_fit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":36136,"parent_record_offset":36100,"ancestor_record_offset":18640,"inlinee":6563,"source_line":1579,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":19448,"parent_record_offset":19412,"ancestor_record_offset":18640,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":35944,"parent_record_offset":35920,"ancestor_record_offset":18640,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":36784,"parent_record_offset":36756,"ancestor_record_offset":18640,"inlinee":6561,"source_line":4416,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":199,"record_offset":35968,"parent_record_offset":35944,"ancestor_record_offset":18640,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":199,"record_offset":36408,"parent_record_offset":36384,"ancestor_record_offset":18640,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":19692,"parent_record_offset":19412,"ancestor_record_offset":18640,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":18640,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":21416,"parent_record_offset":19940,"ancestor_record_offset":18640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":199,"record_offset":35864,"parent_record_offset":35840,"ancestor_record_offset":18640,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":199,"record_offset":36580,"parent_record_offset":36340,"ancestor_record_offset":18640,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::driver_screen_tests::gpu_amd_weak
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":27020,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":16000,"source_line":226,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::driver_screen_tests::gpu_nv_rtx50_bad_580
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":19940,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":15997,"source_line":186,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::driver_screen_tests::gpu_virtual
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":24652,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":15999,"source_line":349,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::driver_screen_tests::gpu_wut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":22296,"parent_record_offset":18796,"ancestor_record_offset":18640,"inlinee":15998,"source_line":326,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::slice::boxed::impl$7::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/slice/boxed.rs
//     evidence={"module":199,"record_offset":36040,"parent_record_offset":36008,"ancestor_record_offset":18640,"inlinee":12349,"source_line":71,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::TiVec::into_boxed_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":199,"record_offset":36072,"parent_record_offset":36040,"ancestor_record_offset":18640,"inlinee":12350,"source_line":310,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] typed_index_collections::vec::impl$42::from_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"typed-index-collections","version":"3.4.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/typed-index-collections-3.4.0/src/vec.rs
//     evidence={"module":199,"record_offset":35920,"parent_record_offset":35896,"ancestor_record_offset":18640,"inlinee":16002,"source_line":1219,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::get_driver_screen_test_case_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E7C
// 首条位置: {"module":199,"inlinee":15996}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":29032,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":19768,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":19300,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":199,"record_offset":32092,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":19244,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":32644,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::driver_screen_tests::gpu_amd_weak
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":27020,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":16000,"source_line":226,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::driver_screen_tests::gpu_nv_rtx50_bad_580
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":19940,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":15997,"source_line":186,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::driver_screen_tests::gpu_virtual
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":24652,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":15999,"source_line":349,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::driver_screen_tests::gpu_wut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/driver_screen_tests.rs
//     evidence={"module":199,"record_offset":22296,"parent_record_offset":18796,"ancestor_record_offset":18796,"inlinee":15998,"source_line":326,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":18796,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19612,"parent_record_offset":19588,"ancestor_record_offset":18796,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":18796,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":18796,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":18796,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":18796,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=35
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":19564,"parent_record_offset":19536,"ancestor_record_offset":18796,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":18796,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":19792,"parent_record_offset":19768,"ancestor_record_offset":18796,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":32128,"parent_record_offset":32092,"ancestor_record_offset":18796,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":199,"record_offset":32668,"parent_record_offset":32644,"ancestor_record_offset":18796,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19476,"parent_record_offset":19448,"ancestor_record_offset":18796,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":18796,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19536,"parent_record_offset":19504,"ancestor_record_offset":18796,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":19504,"parent_record_offset":19476,"ancestor_record_offset":18796,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21512,"parent_record_offset":21488,"ancestor_record_offset":18796,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":29056,"parent_record_offset":29032,"ancestor_record_offset":18796,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19356,"parent_record_offset":19328,"ancestor_record_offset":18796,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":18796,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":18796,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":19328,"parent_record_offset":19300,"ancestor_record_offset":18796,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":20012,"parent_record_offset":19940,"ancestor_record_offset":18796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":18796,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":19448,"parent_record_offset":19412,"ancestor_record_offset":18796,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":19692,"parent_record_offset":19412,"ancestor_record_offset":18796,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":18796,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":21416,"parent_record_offset":19940,"ancestor_record_offset":18796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_nv_rtx50_bad_580
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E7D
// 首条位置: {"module":199,"inlinee":15997}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":20012,"parent_record_offset":19940,"ancestor_record_offset":19940,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":21416,"parent_record_offset":19940,"ancestor_record_offset":19940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":19940,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":20324,"parent_record_offset":20300,"ancestor_record_offset":19940,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":19940,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21608,"parent_record_offset":21584,"ancestor_record_offset":19940,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19940,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21632,"parent_record_offset":21608,"ancestor_record_offset":19940,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":20276,"parent_record_offset":20248,"ancestor_record_offset":19940,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":19940,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":20188,"parent_record_offset":20160,"ancestor_record_offset":19940,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":19940,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":20248,"parent_record_offset":20216,"ancestor_record_offset":19940,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":20216,"parent_record_offset":20188,"ancestor_record_offset":19940,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":21512,"parent_record_offset":21488,"ancestor_record_offset":19940,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":20068,"parent_record_offset":20040,"ancestor_record_offset":19940,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":20096,"parent_record_offset":20068,"ancestor_record_offset":19940,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":20124,"parent_record_offset":20096,"ancestor_record_offset":19940,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":20040,"parent_record_offset":20012,"ancestor_record_offset":19940,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":20160,"parent_record_offset":20124,"ancestor_record_offset":19940,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":20404,"parent_record_offset":20124,"ancestor_record_offset":19940,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":20428,"parent_record_offset":20404,"ancestor_record_offset":19940,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":21440,"parent_record_offset":21416,"ancestor_record_offset":19940,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_amd_weak
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E80
// 首条位置: {"module":199,"inlinee":16000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":27092,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":28496,"parent_record_offset":27020,"ancestor_record_offset":27020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":27020,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":27404,"parent_record_offset":27380,"ancestor_record_offset":27020,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":27020,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":27020,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":27428,"parent_record_offset":27404,"ancestor_record_offset":27020,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":28712,"parent_record_offset":28688,"ancestor_record_offset":27020,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":27356,"parent_record_offset":27328,"ancestor_record_offset":27020,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":28640,"parent_record_offset":28616,"ancestor_record_offset":27020,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":27268,"parent_record_offset":27240,"ancestor_record_offset":27020,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":28616,"parent_record_offset":28592,"ancestor_record_offset":27020,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":27328,"parent_record_offset":27296,"ancestor_record_offset":27020,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":27296,"parent_record_offset":27268,"ancestor_record_offset":27020,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":27020,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":27148,"parent_record_offset":27120,"ancestor_record_offset":27020,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":27176,"parent_record_offset":27148,"ancestor_record_offset":27020,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":27204,"parent_record_offset":27176,"ancestor_record_offset":27020,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":27120,"parent_record_offset":27092,"ancestor_record_offset":27020,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":27240,"parent_record_offset":27204,"ancestor_record_offset":27020,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":27484,"parent_record_offset":27204,"ancestor_record_offset":27020,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":27508,"parent_record_offset":27484,"ancestor_record_offset":27020,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":28520,"parent_record_offset":28496,"ancestor_record_offset":27020,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_wut
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 326
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E7E
// 首条位置: {"module":199,"inlinee":15998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":22368,"parent_record_offset":22296,"ancestor_record_offset":22296,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":23772,"parent_record_offset":22296,"ancestor_record_offset":22296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":22296,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":22296,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":23940,"parent_record_offset":23916,"ancestor_record_offset":22296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":23964,"parent_record_offset":23940,"ancestor_record_offset":22296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":22704,"parent_record_offset":22680,"ancestor_record_offset":22296,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":23988,"parent_record_offset":23964,"ancestor_record_offset":22296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":22632,"parent_record_offset":22604,"ancestor_record_offset":22296,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":23916,"parent_record_offset":23892,"ancestor_record_offset":22296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":22544,"parent_record_offset":22516,"ancestor_record_offset":22296,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":23892,"parent_record_offset":23868,"ancestor_record_offset":22296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":22604,"parent_record_offset":22572,"ancestor_record_offset":22296,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":22572,"parent_record_offset":22544,"ancestor_record_offset":22296,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":22296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":22424,"parent_record_offset":22396,"ancestor_record_offset":22296,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":22452,"parent_record_offset":22424,"ancestor_record_offset":22296,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":22480,"parent_record_offset":22452,"ancestor_record_offset":22296,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":22396,"parent_record_offset":22368,"ancestor_record_offset":22296,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":22516,"parent_record_offset":22480,"ancestor_record_offset":22296,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":22760,"parent_record_offset":22480,"ancestor_record_offset":22296,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":22784,"parent_record_offset":22760,"ancestor_record_offset":22296,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":23796,"parent_record_offset":23772,"ancestor_record_offset":22296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::driver_screen_tests::gpu_virtual
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 349
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3E7F
// 首条位置: {"module":199,"inlinee":15999}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":199,"record_offset":24724,"parent_record_offset":24652,"ancestor_record_offset":24652,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":26128,"parent_record_offset":24652,"ancestor_record_offset":24652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":25012,"parent_record_offset":24988,"ancestor_record_offset":24652,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":25036,"parent_record_offset":25012,"ancestor_record_offset":24652,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":26296,"parent_record_offset":26272,"ancestor_record_offset":24652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":26320,"parent_record_offset":26296,"ancestor_record_offset":24652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":25060,"parent_record_offset":25036,"ancestor_record_offset":24652,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":26344,"parent_record_offset":26320,"ancestor_record_offset":24652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":24988,"parent_record_offset":24960,"ancestor_record_offset":24652,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":199,"record_offset":26272,"parent_record_offset":26248,"ancestor_record_offset":24652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":24900,"parent_record_offset":24872,"ancestor_record_offset":24652,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":24652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":24960,"parent_record_offset":24928,"ancestor_record_offset":24652,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":24928,"parent_record_offset":24900,"ancestor_record_offset":24652,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":199,"record_offset":26224,"parent_record_offset":26200,"ancestor_record_offset":24652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":24780,"parent_record_offset":24752,"ancestor_record_offset":24652,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":24808,"parent_record_offset":24780,"ancestor_record_offset":24652,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":24836,"parent_record_offset":24808,"ancestor_record_offset":24652,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":199,"record_offset":24752,"parent_record_offset":24724,"ancestor_record_offset":24652,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":199,"record_offset":24872,"parent_record_offset":24836,"ancestor_record_offset":24652,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":199,"record_offset":25116,"parent_record_offset":24836,"ancestor_record_offset":24652,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":25140,"parent_record_offset":25116,"ancestor_record_offset":24652,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00199.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":199,"record_offset":26152,"parent_record_offset":26128,"ancestor_record_offset":24652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00199.jsonl.gz"}
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
// 完整结构及原始类型记录: driver_screen_tests.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: driver_screen_tests.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: driver_screen_tests.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::get_driver_screen_test_case
// 信息 ID: caeb6154329deb365406a81b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42e30f -> 0x563d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::get_driver_screen_test_case_impl
// 信息 ID: 6088c620fc2fb445e4da8640
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::driver_screen_tests::get_driver_screen_test_case; module=199, inline_record=18796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_amd_stronk
// 信息 ID: 411fa5ed83117748a4d112b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::driver_screen_tests::get_driver_screen_test_case @ 0x5647bc -> 0x563770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_amd_weak
// 信息 ID: bc478990e808d965a9234b71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::driver_screen_tests::get_driver_screen_test_case; module=199, inline_record=27020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_intel_old
// 信息 ID: ca375c447d269367f0052bfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_intel_weak
// 信息 ID: ef6388f6cc02ef809b6cdb1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_ms_dozen
// 信息 ID: 8ba644751cc1936e642727f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_nv_oom
// 信息 ID: 24e0dfc09201c9132422aa90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_nv_rtx50_bad_580
// 信息 ID: 01dfd7a6bcf0c2afd749c1fd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::driver_screen_tests::get_driver_screen_test_case; module=199, inline_record=19940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_nv_stronk
// 信息 ID: 4c9be36180daae5a12537834
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_nv_weak
// 信息 ID: cb2e1f0cfa3a3b591a9e670e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::driver_screen_tests::get_driver_screen_test_case @ 0x5647cb -> 0x562f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_virtual
// 信息 ID: ef05b07d8c17e5c50ef98781
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::driver_screen_tests::get_driver_screen_test_case; module=199, inline_record=24652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::gpu_wut
// 信息 ID: a125bafa86ff8ff66557d560
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::driver_screen_tests::get_driver_screen_test_case; module=199, inline_record=22296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::driver_screen_tests::igpu_amd_ryzen
// 信息 ID: aba95c5ddd34a8dd14115f2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
