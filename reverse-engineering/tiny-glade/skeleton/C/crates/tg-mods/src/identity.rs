// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-mods/src/identity.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 105b66f97f5fe65374d69b35d3ee3e59c14bbc74c0cdf3c14fd6b9f85dd4451b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: identity.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 31, 81, 144, 166, 214, 352, 509, 529, 546, 559, 566, 612, 682, 683, 1237, 1267, 1636, 1643, 1665, 1694, 1770, 1793, 1802, 1803, 1821, 1822, 2029, 2030, 2031, 2045, 2873, 2896, 3558, 4413
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.6"}]
// use blake3; // namespace candidate
//   package identities: [{"name":"blake3","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.5.3"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::zip_mod_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":2352,"rva":28615120,"va":5397324240}
// 编译实例 RVAs: 0x1b4a1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::wtf8::Wtf8>::to_string_lossy
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12532192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e820c640ff","size":5,"soft_fail":false,"target_va":5384792224,"va":5397324411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12668085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e84bb33eff","size":5,"soft_fail":false,"target_va":5384656240,"va":5397324320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::_with_file_name
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12668586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e856b13eff","size":5,"soft_fail":false,"target_va":5384656064,"va":5397324645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12667720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8b8b43eff","size":5,"soft_fail":false,"target_va":5384657024,"va":5397324739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::path::Path>::file_stem
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12667084","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e834b73eff","size":5,"soft_fail":false,"target_va":5384657312,"va":5397324391,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-12716170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e876f73dff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397325573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=14
//     evidence={"assembly":"\tcallq\t-21645681","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e88fb6b5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397324380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27986204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8e4f654fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397325047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4ab10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8d3080000","size":5,"soft_fail":false,"target_va":5397326608,"va":5397324344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4bba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8c8150000","size":5,"soft_fail":false,"target_va":5397330848,"va":5397325267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12529739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8b5cf40ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397324598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::metadata
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12600035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e81dbd3fff","size":5,"soft_fail":false,"target_va":5384724912,"va":5397324942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12600897","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397324240,"raw_hex":"e8bfb93fff","size":5,"soft_fail":false,"target_va":5384724448,"va":5397325340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":8064,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":2512,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":2484,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::rename_legacy_sidecar
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":5068,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":87452,"source_line":255,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::zip_sidecar_path
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":2852,"parent_record_offset":2352,"ancestor_record_offset":2352,"inlinee":87449,"source_line":269,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":2732,"parent_record_offset":2708,"ancestor_record_offset":2352,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":2756,"parent_record_offset":2732,"ancestor_record_offset":2352,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2352,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":2708,"parent_record_offset":2656,"ancestor_record_offset":2352,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$14::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":1793,"record_offset":3056,"parent_record_offset":3024,"ancestor_record_offset":2352,"inlinee":69577,"source_line":439,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":3088,"parent_record_offset":2852,"ancestor_record_offset":2352,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2352,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":2352,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":2656,"parent_record_offset":2632,"ancestor_record_offset":2352,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":2632,"parent_record_offset":2612,"ancestor_record_offset":2352,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":6988,"parent_record_offset":5068,"ancestor_record_offset":2352,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1793,"record_offset":3172,"parent_record_offset":2852,"ancestor_record_offset":2352,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":2352,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":2908,"parent_record_offset":2852,"ancestor_record_offset":2352,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2352,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":3024,"parent_record_offset":2852,"ancestor_record_offset":2352,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=72
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":2532,"parent_record_offset":2512,"ancestor_record_offset":2352,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1793,"record_offset":6304,"parent_record_offset":6276,"ancestor_record_offset":2352,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":2352,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":2352,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":2352,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":7044,"parent_record_offset":5068,"ancestor_record_offset":2352,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":2352,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":2352,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":6904,"parent_record_offset":5068,"ancestor_record_offset":2352,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1793,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2352,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":2352,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":6876,"parent_record_offset":5068,"ancestor_record_offset":2352,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::exists
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":6252,"parent_record_offset":5068,"ancestor_record_offset":2352,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::with_file_name
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":3200,"parent_record_offset":2852,"ancestor_record_offset":2352,"inlinee":49065,"source_line":3038,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1793,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2352,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::zip_mod_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":8096,"parent_record_offset":8064,"ancestor_record_offset":2352,"inlinee":87454,"source_line":250,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::zip_sidecar_path::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":2936,"parent_record_offset":2908,"ancestor_record_offset":2352,"inlinee":87451,"source_line":272,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$11::eq
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x88B8
// 首条位置: {"module":559,"inlinee":35000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":559,"record_offset":17944,"parent_record_offset":17916,"ancestor_record_offset":17916,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":559,"record_offset":17972,"parent_record_offset":17944,"ancestor_record_offset":17916,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18000,"parent_record_offset":17972,"ancestor_record_offset":17916,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":559,"record_offset":18028,"parent_record_offset":18000,"ancestor_record_offset":17916,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00559.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$5::clone
// 记录形式: inline_instance, inlinee_declaration; instances=26
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x71E7
// 首条位置: {"module":509,"inlinee":29159}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$6::fmt
// 记录形式: compiled_procedure; instances=3
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":147752,"rva":11569600,"va":5380278720}
// 编译实例 RVAs: 0xb089c0, 0xd84a70, 0x1da5090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <core::fmt::Formatter>::debug_tuple_field1_finish
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t4538894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380278720,"raw_hex":"e80e424500","size":5,"soft_fail":false,"target_va":5384817664,"va":5380278765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$8::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1744E
// 首条位置: {"module":2031,"inlinee":95310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2031,"record_offset":608,"parent_record_offset":584,"ancestor_record_offset":584,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":2031,"record_offset":728,"parent_record_offset":704,"ancestor_record_offset":584,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":2031,"record_offset":632,"parent_record_offset":608,"ancestor_record_offset":584,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":2031,"record_offset":680,"parent_record_offset":656,"ancestor_record_offset":584,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":2031,"record_offset":704,"parent_record_offset":680,"ancestor_record_offset":584,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":2031,"record_offset":656,"parent_record_offset":632,"ancestor_record_offset":584,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/02031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$9::cmp
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C0D
// 首条位置: {"module":529,"inlinee":31757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":529,"record_offset":46204,"parent_record_offset":46180,"ancestor_record_offset":46180,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":529,"record_offset":46228,"parent_record_offset":46204,"ancestor_record_offset":46180,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":529,"record_offset":46308,"parent_record_offset":46276,"ancestor_record_offset":46180,"inlinee":5463,"source_line":2010,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":529,"record_offset":46276,"parent_record_offset":46252,"ancestor_record_offset":46180,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":529,"record_offset":46252,"parent_record_offset":46228,"ancestor_record_offset":46180,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00529.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$0::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":1972,"rva":20278880,"va":5388988000}
// 编译实例 RVAs: 0x1356e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] alias_set_at_0x140f5e510
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-4163939","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5388988000,"raw_hex":"e99d76c0ff","size":5,"soft_fail":false,"target_va":5384824080,"va":5388988014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::ModId::as_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155AF
// 首条位置: {"module":1793,"inlinee":87471}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::ModId::parse
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":19956,"rva":28621104,"va":5397330224}
// 编译实例 RVAs: 0x1b4b930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21652109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330224,"raw_hex":"e8739db5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397330792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28608586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330224,"raw_hex":"e8b6774bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397330773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19726237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330224,"raw_hex":"e89dff2c01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397330814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19290021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330224,"raw_hex":"e8a5572601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397330768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12499738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330224,"raw_hex":"e8e64441ff","size":5,"soft_fail":false,"target_va":5384830608,"va":5397330341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1793,"record_offset":21124,"parent_record_offset":19956,"ancestor_record_offset":19956,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::split_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":20080,"parent_record_offset":19956,"ancestor_record_offset":19956,"inlinee":31753,"source_line":1970,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::is_lowercase_hex
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":20424,"parent_record_offset":19956,"ancestor_record_offset":19956,"inlinee":87462,"source_line":163,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":19956,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21572,"parent_record_offset":21548,"ancestor_record_offset":19956,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21596,"parent_record_offset":21572,"ancestor_record_offset":19956,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21524,"parent_record_offset":21376,"ancestor_record_offset":19956,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21320,"parent_record_offset":21296,"ancestor_record_offset":19956,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21376,"parent_record_offset":21344,"ancestor_record_offset":19956,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":19956,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21412,"parent_record_offset":21376,"ancestor_record_offset":19956,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":19956,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":19956,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":19956,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":19956,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":21172,"parent_record_offset":21148,"ancestor_record_offset":19956,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":21296,"parent_record_offset":21268,"ancestor_record_offset":19956,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1793,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":19956,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":20332,"parent_record_offset":20104,"ancestor_record_offset":19956,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1793,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":19956,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1793,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":19956,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1793,"record_offset":20512,"parent_record_offset":20484,"ancestor_record_offset":19956,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20484,"parent_record_offset":20456,"ancestor_record_offset":19956,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20620,"parent_record_offset":20568,"ancestor_record_offset":19956,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":19956,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":19956,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":20192,"parent_record_offset":20164,"ancestor_record_offset":19956,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_digit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":20724,"parent_record_offset":20672,"ancestor_record_offset":19956,"inlinee":42082,"source_line":921,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":21652,"parent_record_offset":21268,"ancestor_record_offset":19956,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":19956,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1793,"record_offset":20960,"parent_record_offset":20936,"ancestor_record_offset":19956,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":20356,"parent_record_offset":20332,"ancestor_record_offset":19956,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":20380,"parent_record_offset":20356,"ancestor_record_offset":19956,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1793,"record_offset":20140,"parent_record_offset":20104,"ancestor_record_offset":19956,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1793,"record_offset":20224,"parent_record_offset":20164,"ancestor_record_offset":19956,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1793,"record_offset":20164,"parent_record_offset":20140,"ancestor_record_offset":19956,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1793,"record_offset":20936,"parent_record_offset":20908,"ancestor_record_offset":19956,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1793,"record_offset":20260,"parent_record_offset":20104,"ancestor_record_offset":19956,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1793,"record_offset":20288,"parent_record_offset":20260,"ancestor_record_offset":19956,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$9::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1793,"record_offset":20456,"parent_record_offset":20424,"ancestor_record_offset":19956,"inlinee":33965,"source_line":316,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1793,"record_offset":20104,"parent_record_offset":20080,"ancestor_record_offset":19956,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::impl$1::parse::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":21148,"parent_record_offset":21124,"ancestor_record_offset":19956,"inlinee":87466,"source_line":67,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::is_lowercase_hex::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":20672,"parent_record_offset":20620,"ancestor_record_offset":19956,"inlinee":87464,"source_line":167,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$1::parse::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155AA
// 首条位置: {"module":1793,"inlinee":87466}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":21172,"parent_record_offset":21148,"ancestor_record_offset":21148,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21548,"parent_record_offset":21524,"ancestor_record_offset":21148,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21572,"parent_record_offset":21548,"ancestor_record_offset":21148,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21596,"parent_record_offset":21572,"ancestor_record_offset":21148,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":21524,"parent_record_offset":21376,"ancestor_record_offset":21148,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21320,"parent_record_offset":21296,"ancestor_record_offset":21148,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21376,"parent_record_offset":21344,"ancestor_record_offset":21148,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":21148,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":21412,"parent_record_offset":21376,"ancestor_record_offset":21148,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":21148,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":21148,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21268,"parent_record_offset":21244,"ancestor_record_offset":21148,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":21148,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":21296,"parent_record_offset":21268,"ancestor_record_offset":21148,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1793,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":21148,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1793,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":21148,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":21148,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":21652,"parent_record_offset":21268,"ancestor_record_offset":21148,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":21676,"parent_record_offset":21652,"ancestor_record_offset":21148,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::ModId::recorded_entry
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 71
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155BE
// 首条位置: {"module":1793,"record_offset":19748,"rva":28621024,"va":5397330144}
// 编译实例 RVAs: 0x1b4b8e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12535346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330144,"raw_hex":"e8ceb940ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397330205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":19872,"parent_record_offset":19748,"ancestor_record_offset":19748,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":19920,"parent_record_offset":19896,"ancestor_record_offset":19748,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":19896,"parent_record_offset":19872,"ancestor_record_offset":19748,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::ModId::owner_hash8
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":17932,"rva":28620560,"va":5397329680}
// 编译实例 RVAs: 0x1b4b710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21651119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e851a1b5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397329802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28607600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e8907b4bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397329787,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19290927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e82f5b2601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397329862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hash::to_hex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12213158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e85aa445ff","size":5,"soft_fail":false,"target_va":5385116576,"va":5397329729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12212953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e827a545ff","size":5,"soft_fail":false,"target_va":5385116768,"va":5397329716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19291594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329680,"raw_hex":"e8ca5d2601","size":5,"soft_fail":false,"target_va":5416621376,"va":5397329777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":18272,"parent_record_offset":17932,"ancestor_record_offset":17932,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":18064,"parent_record_offset":17932,"ancestor_record_offset":17932,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":18116,"parent_record_offset":17932,"ancestor_record_offset":17932,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":17932,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":18588,"parent_record_offset":18564,"ancestor_record_offset":17932,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":18612,"parent_record_offset":18588,"ancestor_record_offset":17932,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":18540,"parent_record_offset":18512,"ancestor_record_offset":17932,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":18456,"parent_record_offset":18428,"ancestor_record_offset":17932,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":18512,"parent_record_offset":18484,"ancestor_record_offset":17932,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":17932,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":18340,"parent_record_offset":18300,"ancestor_record_offset":17932,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":18368,"parent_record_offset":18340,"ancestor_record_offset":17932,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":18396,"parent_record_offset":18368,"ancestor_record_offset":17932,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":18300,"parent_record_offset":18272,"ancestor_record_offset":17932,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":18428,"parent_record_offset":18396,"ancestor_record_offset":17932,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":18088,"parent_record_offset":18064,"ancestor_record_offset":17932,"inlinee":45537,"source_line":88,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":18228,"parent_record_offset":18196,"ancestor_record_offset":17932,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":18668,"parent_record_offset":18396,"ancestor_record_offset":17932,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":18692,"parent_record_offset":18668,"ancestor_record_offset":17932,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":18196,"parent_record_offset":18168,"ancestor_record_offset":17932,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":18168,"parent_record_offset":18140,"ancestor_record_offset":17932,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":17932,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::ModId::owner_matches
// 记录形式: compiled_procedure; instances=1
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":18748,"rva":28620752,"va":5397329872}
// 编译实例 RVAs: 0x1b4b7d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21651424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329872,"raw_hex":"e820a0b5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397330123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19726973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329872,"raw_hex":"e87d022d01","size":5,"soft_fail":false,"target_va":5417057040,"va":5397330062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12499386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329872,"raw_hex":"e8464641ff","size":5,"soft_fail":false,"target_va":5384830608,"va":5397329989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::ModId::owner_hash8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329872,"raw_hex":"e894feffff","size":5,"soft_fail":false,"target_va":5397329680,"va":5397330039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":19216,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::split_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":18876,"parent_record_offset":18748,"ancestor_record_offset":18748,"inlinee":31753,"source_line":1970,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":18748,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19652,"parent_record_offset":19628,"ancestor_record_offset":18748,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19676,"parent_record_offset":19652,"ancestor_record_offset":18748,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19604,"parent_record_offset":19540,"ancestor_record_offset":18748,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19572,"parent_record_offset":19540,"ancestor_record_offset":18748,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19540,"parent_record_offset":19512,"ancestor_record_offset":18748,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":18748,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$94::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1793,"record_offset":19272,"parent_record_offset":19244,"ancestor_record_offset":18748,"inlinee":87461,"source_line":2678,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":19124,"parent_record_offset":18900,"ancestor_record_offset":18748,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":18984,"parent_record_offset":18956,"ancestor_record_offset":18748,"inlinee":7937,"source_line":917,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":19428,"parent_record_offset":19244,"ancestor_record_offset":18748,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":18748,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":19172,"parent_record_offset":19148,"ancestor_record_offset":18748,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1793,"record_offset":18932,"parent_record_offset":18900,"ancestor_record_offset":18748,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1793,"record_offset":19016,"parent_record_offset":18956,"ancestor_record_offset":18748,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1793,"record_offset":18956,"parent_record_offset":18932,"ancestor_record_offset":18748,"inlinee":8594,"source_line":366,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1793,"record_offset":19052,"parent_record_offset":18900,"ancestor_record_offset":18748,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":1793,"record_offset":19080,"parent_record_offset":19052,"ancestor_record_offset":18748,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1793,"record_offset":18900,"parent_record_offset":18876,"ancestor_record_offset":18748,"inlinee":12692,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":19300,"parent_record_offset":19272,"ancestor_record_offset":18748,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::impl$1::owner_matches::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":19244,"parent_record_offset":19216,"ancestor_record_offset":18748,"inlinee":87460,"source_line":87,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$1::owner_matches::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A4
// 首条位置: {"module":1793,"inlinee":87460}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$94::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1793,"record_offset":19272,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":87461,"source_line":2678,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":19428,"parent_record_offset":19244,"ancestor_record_offset":19244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19628,"parent_record_offset":19604,"ancestor_record_offset":19244,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19652,"parent_record_offset":19628,"ancestor_record_offset":19244,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19676,"parent_record_offset":19652,"ancestor_record_offset":19244,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":19604,"parent_record_offset":19540,"ancestor_record_offset":19244,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19572,"parent_record_offset":19540,"ancestor_record_offset":19244,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19540,"parent_record_offset":19512,"ancestor_record_offset":19244,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":19512,"parent_record_offset":19484,"ancestor_record_offset":19244,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":19328,"parent_record_offset":19300,"ancestor_record_offset":19244,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":19456,"parent_record_offset":19428,"ancestor_record_offset":19244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":19356,"parent_record_offset":19328,"ancestor_record_offset":19244,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":19244,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":19300,"parent_record_offset":19272,"ancestor_record_offset":19244,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DFC
// 首条位置: {"module":31,"inlinee":7676}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":166,"record_offset":17968,"parent_record_offset":17916,"ancestor_record_offset":17916,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00166.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":166,"record_offset":17940,"parent_record_offset":17916,"ancestor_record_offset":17916,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/00166.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":96720,"parent_record_offset":96680,"ancestor_record_offset":96616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_string
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1665,"record_offset":96680,"parent_record_offset":96648,"ancestor_record_offset":96616,"inlinee":33611,"source_line":396,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x374A
// 首条位置: {"module":144,"inlinee":14154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_core::ser::impls::impl$1::serialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":144,"record_offset":218728,"parent_record_offset":218704,"ancestor_record_offset":218704,"inlinee":12739,"source_line":54,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":28216,"parent_record_offset":28192,"ancestor_record_offset":27724,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":28240,"parent_record_offset":28216,"ancestor_record_offset":27724,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":28264,"parent_record_offset":28240,"ancestor_record_offset":27724,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1802,"record_offset":28192,"parent_record_offset":28044,"ancestor_record_offset":27724,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":27984,"parent_record_offset":27956,"ancestor_record_offset":27724,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":28044,"parent_record_offset":28012,"ancestor_record_offset":27724,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":28012,"parent_record_offset":27984,"ancestor_record_offset":27724,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1802,"record_offset":28080,"parent_record_offset":28044,"ancestor_record_offset":27724,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":27864,"parent_record_offset":27836,"ancestor_record_offset":27724,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":27892,"parent_record_offset":27864,"ancestor_record_offset":27724,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":27920,"parent_record_offset":27892,"ancestor_record_offset":27724,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1802,"record_offset":27836,"parent_record_offset":27808,"ancestor_record_offset":27724,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1802,"record_offset":27808,"parent_record_offset":27780,"ancestor_record_offset":27724,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1802,"record_offset":27956,"parent_record_offset":27920,"ancestor_record_offset":27724,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1802,"record_offset":28104,"parent_record_offset":28080,"ancestor_record_offset":27724,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1802,"record_offset":28152,"parent_record_offset":28128,"ancestor_record_offset":27724,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1802,"record_offset":28128,"parent_record_offset":28104,"ancestor_record_offset":27724,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1802,"record_offset":28320,"parent_record_offset":27920,"ancestor_record_offset":27724,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1802,"record_offset":28344,"parent_record_offset":28320,"ancestor_record_offset":27724,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_str
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":144,"record_offset":218752,"parent_record_offset":218728,"ancestor_record_offset":218704,"inlinee":7400,"source_line":190,"archive":"_pdb/C/modules/00144.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":1802,"record_offset":27780,"parent_record_offset":27752,"ancestor_record_offset":27724,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/01802.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$13::clone
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2A50
// 首条位置: {"module":81,"inlinee":10832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$15::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111F2
// 首条位置: {"module":1237,"inlinee":70130}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$30::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1237,"record_offset":5668,"parent_record_offset":5644,"ancestor_record_offset":5644,"inlinee":14313,"source_line":2727,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5644,"inlinee":14304,"source_line":362,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1237,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":5644,"inlinee":12393,"source_line":864,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":5644,"inlinee":14302,"source_line":237,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\sip.rs
//     evidence={"module":1237,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":5644,"inlinee":14303,"source_line":303,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":1237,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5644,"inlinee":14301,"source_line":135,"archive":"_pdb/C/modules/01237.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5645
// 首条位置: {"module":352,"inlinee":22085}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":2873,"record_offset":17412,"parent_record_offset":17384,"ancestor_record_offset":17384,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":2873,"record_offset":17440,"parent_record_offset":17412,"ancestor_record_offset":17384,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2873,"record_offset":17468,"parent_record_offset":17440,"ancestor_record_offset":17384,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":2873,"record_offset":17496,"parent_record_offset":17468,"ancestor_record_offset":17384,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/02873.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::RecordedModEntry::as_str
// 记录形式: compiled_procedure; instances=1
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":16540,"rva":6916944,"va":5375626064}
// 编译实例 RVAs: 0x698b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::RecordedModEntry::mod_id
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB194
// 首条位置: {"module":682,"inlinee":45460}
// 编译实例 RVAs: 0x1b4b570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] tg_mods::identity::ModId::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329264,"raw_hex":"e86e030000","size":5,"soft_fail":false,"target_va":5397330224,"va":5397329341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::impl$48::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":682,"record_offset":94564,"parent_record_offset":94332,"ancestor_record_offset":94332,"inlinee":10158,"source_line":2779,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":682,"record_offset":94360,"parent_record_offset":94332,"ancestor_record_offset":94332,"inlinee":15542,"source_line":2443,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":682,"record_offset":94464,"parent_record_offset":94440,"ancestor_record_offset":94332,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":682,"record_offset":94488,"parent_record_offset":94464,"ancestor_record_offset":94332,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":682,"record_offset":94512,"parent_record_offset":94488,"ancestor_record_offset":94332,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":682,"record_offset":94440,"parent_record_offset":94416,"ancestor_record_offset":94332,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":682,"record_offset":94416,"parent_record_offset":94388,"ancestor_record_offset":94332,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":682,"record_offset":94388,"parent_record_offset":94360,"ancestor_record_offset":94332,"inlinee":15545,"source_line":1017,"archive":"_pdb/C/modules/00682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::impl$3::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":2208,"rva":1061104,"va":5369770224}
// 编译实例 RVAs: 0x1030f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] <str as core::fmt::Debug>::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15099917","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5369770224,"raw_hex":"e90d68e600","size":5,"soft_fail":false,"target_va":5384870160,"va":5369770238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$29::fmt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1793,"record_offset":2324,"parent_record_offset":2208,"ancestor_record_offset":2208,"inlinee":6305,"source_line":2719,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::mint
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155AB
// 首条位置: {"module":1793,"inlinee":87467}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":22180,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":22576,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22000,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1793,"record_offset":22264,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":22292,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1793,"record_offset":22092,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22636,"parent_record_offset":21900,"ancestor_record_offset":21900,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22468,"parent_record_offset":22444,"ancestor_record_offset":21900,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":21900,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":21900,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22444,"parent_record_offset":22388,"ancestor_record_offset":21900,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":22228,"parent_record_offset":22204,"ancestor_record_offset":21900,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22416,"parent_record_offset":22388,"ancestor_record_offset":21900,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":21900,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":21900,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":22600,"parent_record_offset":22576,"ancestor_record_offset":21900,"inlinee":33671,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23552,"parent_record_offset":23528,"ancestor_record_offset":21900,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":21900,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23628,"parent_record_offset":23600,"ancestor_record_offset":21900,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":1793,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":21900,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":1793,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":21900,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22052,"parent_record_offset":22024,"ancestor_record_offset":21900,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22024,"parent_record_offset":22000,"ancestor_record_offset":21900,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":22756,"parent_record_offset":22720,"ancestor_record_offset":21900,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21900,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":22316,"parent_record_offset":22292,"ancestor_record_offset":21900,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":22720,"parent_record_offset":22692,"ancestor_record_offset":21900,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22692,"parent_record_offset":22660,"ancestor_record_offset":21900,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22660,"parent_record_offset":22636,"ancestor_record_offset":21900,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1793,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":21900,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::mint::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":21900,"inlinee":87469,"source_line":150,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::mint::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155AD
// 首条位置: {"module":1793,"inlinee":87469}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1793,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":22120,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::is_lowercase_hex
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A6
// 首条位置: {"module":1793,"inlinee":87462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::iter::impl$9::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":1793,"record_offset":20456,"parent_record_offset":20424,"ancestor_record_offset":20424,"inlinee":33965,"source_line":316,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_try_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1793,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":20424,"inlinee":8671,"source_line":40,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1793,"record_offset":20512,"parent_record_offset":20484,"ancestor_record_offset":20424,"inlinee":8669,"source_line":68,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20484,"parent_record_offset":20456,"ancestor_record_offset":20424,"inlinee":8990,"source_line":2809,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::all::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20620,"parent_record_offset":20568,"ancestor_record_offset":20424,"inlinee":8997,"source_line":2816,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1793,"record_offset":20540,"parent_record_offset":20512,"ancestor_record_offset":20424,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_digit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":20724,"parent_record_offset":20672,"ancestor_record_offset":20424,"inlinee":42082,"source_line":921,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1793,"record_offset":20960,"parent_record_offset":20936,"ancestor_record_offset":20812,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1793,"record_offset":20936,"parent_record_offset":20908,"ancestor_record_offset":20812,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::is_lowercase_hex::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":20672,"parent_record_offset":20620,"ancestor_record_offset":20424,"inlinee":87464,"source_line":167,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::is_lowercase_hex::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 167
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A8
// 首条位置: {"module":1793,"inlinee":87464}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$6::is_ascii_digit
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":20724,"parent_record_offset":20672,"ancestor_record_offset":20672,"inlinee":42082,"source_line":921,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::read_id_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":9956,"rva":28617488,"va":5397326608}
// 编译实例 RVAs: 0x1b4ab10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12717558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e80af23dff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397326961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21649035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e875a9b5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397327718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-21648351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e821acb5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397327050,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28605512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8b8834bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397327699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27988507","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8e5ed54fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397327350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27985220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8bcfa54fe","size":5,"soft_fail":false,"target_va":5369343504,"va":5397328719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d81bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14454701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8537023ff","size":5,"soft_fail":false,"target_va":5382872048,"va":5397326744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84050
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14447573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e82b8c23ff","size":5,"soft_fail":false,"target_va":5382881360,"va":5397328928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ad9ef0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-462357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8ebf1f8ff","size":5,"soft_fail":false,"target_va":5396864752,"va":5397327104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3400
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2458530","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8a2832500","size":5,"soft_fail":false,"target_va":5399786496,"va":5397327961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19729307","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e89b0b2d01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397327744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::hab7539d1bf208f16 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2451812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e864692500","size":5,"soft_fail":false,"target_va":5399779808,"va":5397327991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19293637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8c5652601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397327152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$alloc..string..String$C$serde_json..value..Value$C$alloc..alloc..Global$GT$$GT$::h065e14f06e7b32d2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2446411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e84b542500","size":5,"soft_fail":false,"target_va":5399775408,"va":5397328992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h64cf2b524f0eadd2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2450428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8fc632500","size":5,"soft_fail":false,"target_va":5399777712,"va":5397327279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12574730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e8f61f40ff","size":5,"soft_fail":false,"target_va":5384751936,"va":5397326661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::ModId::parse
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326608,"raw_hex":"e881080000","size":5,"soft_fail":false,"target_va":5397330224,"va":5397328042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":10444,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":11268,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":11356,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":10612,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":10500,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10360,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::Value::get
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":11212,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":30733,"source_line":305,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":10108,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::Error::kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1793,"record_offset":10136,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":17357,"source_line":1005,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::io::error::impl$21::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error.rs
//     evidence={"module":1793,"record_offset":10324,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":8741,"source_line":230,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":9956,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":9956,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":9956,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":9956,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":9956,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10868,"parent_record_offset":10844,"ancestor_record_offset":9956,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":11812,"parent_record_offset":11660,"ancestor_record_offset":9956,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":10796,"parent_record_offset":10764,"ancestor_record_offset":9956,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1793,"record_offset":10764,"parent_record_offset":10736,"ancestor_record_offset":9956,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":13148,"parent_record_offset":13116,"ancestor_record_offset":9956,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":13272,"parent_record_offset":13244,"ancestor_record_offset":9956,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12152,"parent_record_offset":12128,"ancestor_record_offset":9956,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":9956,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":9956,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":9956,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":9956,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12176,"parent_record_offset":12152,"ancestor_record_offset":9956,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11660,"parent_record_offset":11624,"ancestor_record_offset":9956,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":9956,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":9956,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":11700,"parent_record_offset":11660,"ancestor_record_offset":9956,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11480,"parent_record_offset":11452,"ancestor_record_offset":9956,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11508,"parent_record_offset":11480,"ancestor_record_offset":9956,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11536,"parent_record_offset":11508,"ancestor_record_offset":9956,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":11452,"parent_record_offset":11424,"ancestor_record_offset":9956,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":11424,"parent_record_offset":11396,"ancestor_record_offset":9956,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":12128,"parent_record_offset":12100,"ancestor_record_offset":9956,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":11568,"parent_record_offset":11536,"ancestor_record_offset":9956,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":12100,"parent_record_offset":12068,"ancestor_record_offset":9956,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1793,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":9956,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1793,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":9956,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":9956,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":12380,"parent_record_offset":12352,"ancestor_record_offset":9956,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1793,"record_offset":11396,"parent_record_offset":11356,"ancestor_record_offset":9956,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":10468,"parent_record_offset":10444,"ancestor_record_offset":9956,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":11940,"parent_record_offset":11536,"ancestor_record_offset":9956,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":9956,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":10648,"parent_record_offset":10612,"ancestor_record_offset":9956,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":13180,"parent_record_offset":13148,"ancestor_record_offset":9956,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":10384,"parent_record_offset":10360,"ancestor_record_offset":9956,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":10408,"parent_record_offset":10384,"ancestor_record_offset":9956,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10548,"parent_record_offset":10524,"ancestor_record_offset":9956,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":10524,"parent_record_offset":10500,"ancestor_record_offset":9956,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":10572,"parent_record_offset":10548,"ancestor_record_offset":9956,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::Value::as_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":9956,"inlinee":31538,"source_line":492,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::index::impl$3::index_into
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/index.rs
//     evidence={"module":1793,"record_offset":11236,"parent_record_offset":11212,"ancestor_record_offset":9956,"inlinee":30728,"source_line":123,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::Repr::data
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10184,"parent_record_offset":10136,"ancestor_record_offset":9956,"inlinee":8739,"source_line":208,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10216,"parent_record_offset":10184,"ancestor_record_offset":9956,"inlinee":5225,"source_line":244,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::kind_from_prim
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1793,"record_offset":10256,"parent_record_offset":10216,"ancestor_record_offset":9956,"inlinee":8742,"source_line":290,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::io::error::windows::decode_error_kind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\io\error\windows.rs
//     evidence={"module":1793,"record_offset":10292,"parent_record_offset":10136,"ancestor_record_offset":9956,"inlinee":17334,"source_line":13,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::read_id_file::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":11296,"parent_record_offset":11268,"ancestor_record_offset":9956,"inlinee":87456,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::read_id_file::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A0
// 首条位置: {"module":1793,"inlinee":87456}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::Value::as_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":11320,"parent_record_offset":11296,"ancestor_record_offset":11296,"inlinee":31538,"source_line":492,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::mint_into
// 记录形式: compiled_procedure; instances=1
// 源码行: 205
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":21740,"rva":28621728,"va":5397330848}
// 编译实例 RVAs: 0x1b4bba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::duration_since
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12669994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8d6ab3eff","size":5,"soft_fail":false,"target_va":5384661120,"va":5397331109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::SystemTime>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12669870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e852ac3eff","size":5,"soft_fail":false,"target_va":5384661200,"va":5397331065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-12723990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8ead83dff","size":5,"soft_fail":false,"target_va":5384609408,"va":5397333393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21652986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8069ab5fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397331669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=15
//     evidence={"assembly":"\tcallq\t-21652664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8489bb5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397331363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28609467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e845744bfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397331654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27994941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8c3d454fe","size":5,"soft_fail":false,"target_va":5369338848,"va":5397333784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27991352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8c8e254fe","size":5,"soft_fail":false,"target_va":5369343504,"va":5397334851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d84050
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14454088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8b87223ff","size":5,"soft_fail":false,"target_va":5382881360,"va":5397335443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da31f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2454062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e82e722500","size":5,"soft_fail":false,"target_va":5399785968,"va":5397331901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141da3400
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2454299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e81b732500","size":5,"soft_fail":false,"target_va":5399786496,"va":5397332192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19018712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8d8332201","size":5,"soft_fail":false,"target_va":5416353040,"va":5397334323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19725217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8a1fb2c01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397331834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::hab7539d1bf208f16 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2447569","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8d1582500","size":5,"soft_fail":false,"target_va":5399779808,"va":5397332234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-12536428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e894b540ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397331287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t19289037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8cd532601","size":5,"soft_fail":false,"target_va":5416620794,"va":5397331752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hash::to_hex
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12214845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8c39d45ff","size":5,"soft_fail":false,"target_va":5385116576,"va":5397331416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::finalize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12209785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e887b145ff","size":5,"soft_fail":false,"target_va":5385121616,"va":5397331396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] blake3::Hasher::update
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-12211657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e837aa45ff","size":5,"soft_fail":false,"target_va":5385119680,"va":5397331332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19287166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e87e4c2601","size":5,"soft_fail":false,"target_va":5416621600,"va":5397334429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19287792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8f04e2601","size":5,"soft_fail":false,"target_va":5416622208,"va":5397334411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$alloc..string..String$C$serde_json..value..Value$C$alloc..alloc..Global$GT$$GT$::h065e14f06e7b32d2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2440392","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8c83c2500","size":5,"soft_fail":false,"target_va":5399775408,"va":5397335011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2443864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8584a2500","size":5,"soft_fail":false,"target_va":5399779584,"va":5397335715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::h64cf2b524f0eadd2 (.llvm.6356623600053828511)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2445661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e85d512500","size":5,"soft_fail":false,"target_va":5399777712,"va":5397332046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19289900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e82c572601","size":5,"soft_fail":false,"target_va":5416621376,"va":5397331471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] serde_core::ser::SerializeMap::serialize_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11543035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8fb21b000","size":5,"soft_fail":false,"target_va":5408876000,"va":5397332960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::write::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12581068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8340740ff","size":5,"soft_fail":false,"target_va":5384752464,"va":5397333527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::process::id
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12613399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e8e9883fff","size":5,"soft_fail":false,"target_va":5384717776,"va":5397331170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::identity::ModId::owner_hash8
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1268","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397330848,"raw_hex":"e80cfbffff","size":5,"soft_fail":false,"target_va":5397329680,"va":5397330943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":31192,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1793,"record_offset":23996,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":25328,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1793,"record_offset":25272,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":31248,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":31108,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::insert
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":1793,"record_offset":25300,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":11167,"source_line":127,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::map::Map::new
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":1793,"record_offset":23968,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":11166,"source_line":41,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":26660,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":8088,"source_line":2229,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":1793,"record_offset":24572,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::fs::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":31956,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":8109,"source_line":420,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::ModId::as_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":24544,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":87471,"source_line":57,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::identity::mint
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":21900,"parent_record_offset":21740,"ancestor_record_offset":21740,"inlinee":87467,"source_line":140,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":24356,"parent_record_offset":24332,"ancestor_record_offset":21740,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":24380,"parent_record_offset":24356,"ancestor_record_offset":21740,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22468,"parent_record_offset":22444,"ancestor_record_offset":21740,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22492,"parent_record_offset":22468,"ancestor_record_offset":21740,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":24404,"parent_record_offset":24380,"ancestor_record_offset":21740,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":21740,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":21740,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":22444,"parent_record_offset":22388,"ancestor_record_offset":21740,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1793,"record_offset":31508,"parent_record_offset":31480,"ancestor_record_offset":21740,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":27752,"parent_record_offset":27720,"ancestor_record_offset":21740,"inlinee":8809,"source_line":2698,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":21740,"inlinee":14166,"source_line":2821,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":26464,"parent_record_offset":26432,"ancestor_record_offset":21740,"inlinee":4643,"source_line":207,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$22::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":27836,"parent_record_offset":27792,"ancestor_record_offset":21740,"inlinee":8812,"source_line":1733,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":1793,"record_offset":26588,"parent_record_offset":26560,"ancestor_record_offset":21740,"inlinee":4868,"source_line":1904,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":1793,"record_offset":28096,"parent_record_offset":28036,"ancestor_record_offset":21740,"inlinee":4906,"source_line":18,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::mem::replace::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\mem.rs
//     evidence={"module":1793,"record_offset":28924,"parent_record_offset":28900,"ancestor_record_offset":21740,"inlinee":4926,"source_line":21,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::init_front
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":27928,"parent_record_offset":27884,"ancestor_record_offset":21740,"inlinee":4912,"source_line":221,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::LazyLeafRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":27884,"parent_record_offset":27836,"ancestor_record_offset":21740,"inlinee":6173,"source_line":160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::navigate::impl$22::next_unchecked::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":28144,"parent_record_offset":28096,"ancestor_record_offset":21740,"inlinee":8814,"source_line":533,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28512,"parent_record_offset":28416,"ancestor_record_offset":21740,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28456,"parent_record_offset":28416,"ancestor_record_offset":21740,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28724,"parent_record_offset":28144,"ancestor_record_offset":21740,"inlinee":7204,"source_line":1121,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":28188,"parent_record_offset":28144,"ancestor_record_offset":21740,"inlinee":6176,"source_line":380,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":28416,"parent_record_offset":28144,"ancestor_record_offset":21740,"inlinee":4910,"source_line":715,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":28036,"parent_record_offset":27884,"ancestor_record_offset":21740,"inlinee":7282,"source_line":532,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28692,"parent_record_offset":28416,"ancestor_record_offset":21740,"inlinee":4911,"source_line":811,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::right_kv
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28224,"parent_record_offset":28188,"ancestor_record_offset":21740,"inlinee":4913,"source_line":893,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::ascend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28292,"parent_record_offset":28188,"ancestor_record_offset":21740,"inlinee":4917,"source_line":328,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::first_leaf_edge
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\navigate.rs
//     evidence={"module":1793,"record_offset":27960,"parent_record_offset":27928,"ancestor_record_offset":21740,"inlinee":4915,"source_line":630,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":27992,"parent_record_offset":27960,"ancestor_record_offset":21740,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28256,"parent_record_offset":28224,"ancestor_record_offset":21740,"inlinee":4914,"source_line":287,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::impl$16::ascend::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":1793,"record_offset":28372,"parent_record_offset":28348,"ancestor_record_offset":21740,"inlinee":4919,"source_line":340,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":22180,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":22228,"parent_record_offset":22204,"ancestor_record_offset":21740,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":25752,"parent_record_offset":25728,"ancestor_record_offset":21740,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":29260,"parent_record_offset":29232,"ancestor_record_offset":21740,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":24244,"parent_record_offset":24216,"ancestor_record_offset":21740,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":29988,"parent_record_offset":29964,"ancestor_record_offset":21740,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22416,"parent_record_offset":22388,"ancestor_record_offset":21740,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22388,"parent_record_offset":22364,"ancestor_record_offset":21740,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":29320,"parent_record_offset":29288,"ancestor_record_offset":21740,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":25800,"parent_record_offset":25776,"ancestor_record_offset":21740,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":25776,"parent_record_offset":25752,"ancestor_record_offset":21740,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":29288,"parent_record_offset":29260,"ancestor_record_offset":21740,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":24304,"parent_record_offset":24272,"ancestor_record_offset":21740,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":24272,"parent_record_offset":24244,"ancestor_record_offset":21740,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":21740,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":24936,"parent_record_offset":24896,"ancestor_record_offset":21740,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":24124,"parent_record_offset":24080,"ancestor_record_offset":21740,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":24152,"parent_record_offset":24124,"ancestor_record_offset":21740,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":24180,"parent_record_offset":24152,"ancestor_record_offset":21740,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1793,"record_offset":24080,"parent_record_offset":24052,"ancestor_record_offset":21740,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1793,"record_offset":24052,"parent_record_offset":24024,"ancestor_record_offset":21740,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1793,"record_offset":24024,"parent_record_offset":23996,"ancestor_record_offset":21740,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":27404,"parent_record_offset":27380,"ancestor_record_offset":21740,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":25728,"parent_record_offset":25700,"ancestor_record_offset":21740,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":27356,"parent_record_offset":27332,"ancestor_record_offset":21740,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":29480,"parent_record_offset":29192,"ancestor_record_offset":21740,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":29232,"parent_record_offset":29192,"ancestor_record_offset":21740,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":26728,"parent_record_offset":26660,"ancestor_record_offset":21740,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":24216,"parent_record_offset":24180,"ancestor_record_offset":21740,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1793,"record_offset":25700,"parent_record_offset":25668,"ancestor_record_offset":21740,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1793,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":21740,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::ArrayString::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":22600,"parent_record_offset":22576,"ancestor_record_offset":21740,"inlinee":33671,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::array_string::impl$2::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/array_string.rs
//     evidence={"module":1793,"record_offset":22576,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":33669,"source_line":435,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23552,"parent_record_offset":23528,"ancestor_record_offset":21740,"inlinee":4748,"source_line":254,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$0::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":21740,"inlinee":4746,"source_line":50,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec.rs
//     evidence={"module":1793,"record_offset":23628,"parent_record_offset":23600,"ancestor_record_offset":21740,"inlinee":4753,"source_line":716,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::clear
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":1793,"record_offset":23576,"parent_record_offset":23552,"ancestor_record_offset":21740,"inlinee":4750,"source_line":72,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::truncate
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.6/src/arrayvec_impl.rs
//     evidence={"module":1793,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":21740,"inlinee":4751,"source_line":76,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] blake3::ChunkState::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22052,"parent_record_offset":22024,"ancestor_record_offset":21740,"inlinee":33665,"source_line":477,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22000,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":33662,"source_line":1010,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] blake3::Hasher::new_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"blake3","version":"1.5.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/blake3-1.5.3/src/lib.rs
//     evidence={"module":1793,"record_offset":22024,"parent_record_offset":22000,"ancestor_record_offset":21740,"inlinee":33663,"source_line":1001,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1793,"record_offset":24960,"parent_record_offset":24936,"ancestor_record_offset":21740,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":29624,"parent_record_offset":29592,"ancestor_record_offset":21740,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1793,"record_offset":22264,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1793,"record_offset":25008,"parent_record_offset":24984,"ancestor_record_offset":21740,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1793,"record_offset":29656,"parent_record_offset":29592,"ancestor_record_offset":21740,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1793,"record_offset":29592,"parent_record_offset":29564,"ancestor_record_offset":21740,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1793,"record_offset":29564,"parent_record_offset":29528,"ancestor_record_offset":21740,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":24984,"parent_record_offset":24960,"ancestor_record_offset":21740,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":29680,"parent_record_offset":29656,"ancestor_record_offset":21740,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1793,"record_offset":25980,"parent_record_offset":25952,"ancestor_record_offset":21740,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":22756,"parent_record_offset":22720,"ancestor_record_offset":21740,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":28320,"parent_record_offset":28292,"ancestor_record_offset":21740,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":31216,"parent_record_offset":31192,"ancestor_record_offset":21740,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":28348,"parent_record_offset":28292,"ancestor_record_offset":21740,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":21740,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":28868,"parent_record_offset":28144,"ancestor_record_offset":21740,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$17::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":27484,"parent_record_offset":27248,"ancestor_record_offset":21740,"inlinee":5272,"source_line":2424,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1793,"record_offset":24460,"parent_record_offset":24180,"ancestor_record_offset":21740,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":24484,"parent_record_offset":24460,"ancestor_record_offset":21740,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":22292,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1793,"record_offset":30068,"parent_record_offset":29832,"ancestor_record_offset":21740,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":26496,"parent_record_offset":26464,"ancestor_record_offset":21740,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1793,"record_offset":22092,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1793,"record_offset":28568,"parent_record_offset":28512,"ancestor_record_offset":21740,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1793,"record_offset":28592,"parent_record_offset":28568,"ancestor_record_offset":21740,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":22720,"parent_record_offset":22692,"ancestor_record_offset":21740,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22692,"parent_record_offset":22660,"ancestor_record_offset":21740,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22660,"parent_record_offset":22636,"ancestor_record_offset":21740,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1793,"record_offset":22636,"parent_record_offset":21900,"ancestor_record_offset":21740,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":31132,"parent_record_offset":31108,"ancestor_record_offset":21740,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":31156,"parent_record_offset":31132,"ancestor_record_offset":21740,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::as_nanos
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":1793,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":21740,"inlinee":52443,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":31296,"parent_record_offset":31272,"ancestor_record_offset":21740,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":31272,"parent_record_offset":31248,"ancestor_record_offset":21740,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":31320,"parent_record_offset":31296,"ancestor_record_offset":21740,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$0::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":1793,"record_offset":24644,"parent_record_offset":24620,"ancestor_record_offset":21740,"inlinee":8824,"source_line":42,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":1793,"record_offset":24596,"parent_record_offset":24572,"ancestor_record_offset":21740,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::Map::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":1793,"record_offset":27192,"parent_record_offset":27104,"ancestor_record_offset":21740,"inlinee":14165,"source_line":291,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$18::into_iter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":1793,"record_offset":27720,"parent_record_offset":27104,"ancestor_record_offset":21740,"inlinee":14168,"source_line":1051,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$23::next
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/map.rs
//     evidence={"module":1793,"record_offset":27792,"parent_record_offset":27104,"ancestor_record_offset":21740,"inlinee":14170,"source_line":574,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27048,"parent_record_offset":27004,"ancestor_record_offset":21740,"inlinee":8091,"source_line":39,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27072,"parent_record_offset":27048,"ancestor_record_offset":21740,"inlinee":8092,"source_line":52,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::begin_object
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27280,"parent_record_offset":27248,"ancestor_record_offset":21740,"inlinee":12492,"source_line":2018,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$16::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27512,"parent_record_offset":27248,"ancestor_record_offset":21740,"inlinee":12493,"source_line":2028,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27248,"parent_record_offset":27104,"ancestor_record_offset":21740,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":28976,"parent_record_offset":27104,"ancestor_record_offset":21740,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::indent
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":29528,"parent_record_offset":29024,"ancestor_record_offset":21740,"inlinee":8104,"source_line":2276,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer_pretty
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":1793,"record_offset":27004,"parent_record_offset":26660,"ancestor_record_offset":21740,"inlinee":8089,"source_line":2197,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$0::serialize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":1793,"record_offset":27104,"parent_record_offset":27004,"ancestor_record_offset":21740,"inlinee":14172,"source_line":13,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::ser::impl$1::serialize_str
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/ser.rs
//     evidence={"module":1793,"record_offset":24668,"parent_record_offset":24644,"ancestor_record_offset":21740,"inlinee":7423,"source_line":168,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1793,"record_offset":27332,"parent_record_offset":27308,"ancestor_record_offset":21740,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":1793,"record_offset":27308,"parent_record_offset":27280,"ancestor_record_offset":21740,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::mint::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":22120,"parent_record_offset":22092,"ancestor_record_offset":21740,"inlinee":87469,"source_line":150,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::read_mod_id
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xB1D1
// 首条位置: {"module":683,"inlinee":45521}
// 编译实例 RVAs: 0x1b4aa70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12670255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326448,"raw_hex":"e8d1aa3eff","size":5,"soft_fail":false,"target_va":5384656240,"va":5397326490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21647836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326448,"raw_hex":"e824aeb5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397326535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4ab10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t93","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397326448,"raw_hex":"e85d000000","size":5,"soft_fail":false,"target_va":5397326608,"va":5397326510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":5404,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":683,"record_offset":5376,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":5348,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5648,"parent_record_offset":5624,"ancestor_record_offset":5348,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5672,"parent_record_offset":5648,"ancestor_record_offset":5348,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":683,"record_offset":5600,"parent_record_offset":5548,"ancestor_record_offset":5348,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":5348,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5548,"parent_record_offset":5524,"ancestor_record_offset":5348,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":683,"record_offset":5524,"parent_record_offset":5504,"ancestor_record_offset":5348,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":683,"record_offset":5424,"parent_record_offset":5404,"ancestor_record_offset":5348,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00683.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::folder_mod_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":15576,"rva":28619904,"va":5397329024}
// 编译实例 RVAs: 0x1b4b480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12672836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329024,"raw_hex":"e8bca03eff","size":5,"soft_fail":false,"target_va":5384656240,"va":5397329071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21650457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329024,"raw_hex":"e8e7a3b5fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397329156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4ab10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329024,"raw_hex":"e844f6ffff","size":5,"soft_fail":false,"target_va":5397326608,"va":5397329095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b4bba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: identity.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397329024,"raw_hex":"e8b0060000","size":5,"soft_fail":false,"target_va":5397330848,"va":5397329131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":15728,"parent_record_offset":15576,"ancestor_record_offset":15576,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":15788,"parent_record_offset":15576,"ancestor_record_offset":15576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":15700,"parent_record_offset":15576,"ancestor_record_offset":15576,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":15576,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":15576,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":15576,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":16024,"parent_record_offset":15960,"ancestor_record_offset":15576,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":15992,"parent_record_offset":15960,"ancestor_record_offset":15576,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":15576,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":15932,"parent_record_offset":15908,"ancestor_record_offset":15576,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":15812,"parent_record_offset":15788,"ancestor_record_offset":15576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::folder_mod_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":15760,"parent_record_offset":15728,"ancestor_record_offset":15576,"inlinee":87458,"source_line":234,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::folder_mod_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 234
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x155A2
// 首条位置: {"module":1793,"inlinee":87458}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::zip_mod_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1559E
// 首条位置: {"module":1793,"inlinee":87454}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::rename_legacy_sidecar
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1559C
// 首条位置: {"module":1793,"inlinee":87452}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":5224,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":6988,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1793,"record_offset":5620,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":5140,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":5308,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":7044,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":6904,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1793,"record_offset":5168,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":6876,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":13844,"source_line":2818,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::exists
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":6252,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":14220,"source_line":3443,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_file_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":5648,"parent_record_offset":5068,"ancestor_record_offset":5068,"inlinee":49065,"source_line":3038,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":5068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":5532,"parent_record_offset":5508,"ancestor_record_offset":5068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":5556,"parent_record_offset":5532,"ancestor_record_offset":5068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":5484,"parent_record_offset":5428,"ancestor_record_offset":5068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":5272,"parent_record_offset":5248,"ancestor_record_offset":5068,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":5456,"parent_record_offset":5428,"ancestor_record_offset":5068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":5428,"parent_record_offset":5404,"ancestor_record_offset":5068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":5068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":7012,"parent_record_offset":6988,"ancestor_record_offset":5068,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":5248,"parent_record_offset":5224,"ancestor_record_offset":5068,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":5068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1793,"record_offset":6304,"parent_record_offset":6276,"ancestor_record_offset":5068,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":6928,"parent_record_offset":6904,"ancestor_record_offset":5068,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1793,"record_offset":6952,"parent_record_offset":6928,"ancestor_record_offset":5068,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":5068,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1793,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":5068,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1793,"record_offset":7116,"parent_record_offset":7092,"ancestor_record_offset":5068,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":1793,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5068,"inlinee":10150,"source_line":2737,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1793,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":5068,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::zip_sidecar_path
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 269
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15599
// 首条位置: {"module":1793,"inlinee":87449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":3088,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1793,"record_offset":3172,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":2908,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":3024,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":6035,"source_line":1093,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":3804,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_file_name
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1793,"record_offset":3200,"parent_record_offset":2852,"ancestor_record_offset":2852,"inlinee":49065,"source_line":3038,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":3408,"parent_record_offset":3384,"ancestor_record_offset":2852,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":3432,"parent_record_offset":3408,"ancestor_record_offset":2852,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":2852,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1793,"record_offset":3384,"parent_record_offset":3328,"ancestor_record_offset":2852,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::borrow::impl$14::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":1793,"record_offset":3056,"parent_record_offset":3024,"ancestor_record_offset":2852,"inlinee":69577,"source_line":439,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1793,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2852,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":3356,"parent_record_offset":3328,"ancestor_record_offset":2852,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":3328,"parent_record_offset":3304,"ancestor_record_offset":2852,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1793,"record_offset":3304,"parent_record_offset":3280,"ancestor_record_offset":2852,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1793,"record_offset":3112,"parent_record_offset":3088,"ancestor_record_offset":2852,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1793,"record_offset":3232,"parent_record_offset":3200,"ancestor_record_offset":2852,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1793,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2852,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1793,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2852,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::identity::zip_sidecar_path::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/identity.rs
//     evidence={"module":1793,"record_offset":2936,"parent_record_offset":2908,"ancestor_record_offset":2852,"inlinee":87451,"source_line":272,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::zip_sidecar_path::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 272
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1559B
// 首条位置: {"module":1793,"inlinee":87451}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1793,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":6789,"source_line":966,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::Slice::to_string_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1793,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2936,"inlinee":6790,"source_line":260,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::identity::sidecar_zip_stem
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1793,"record_offset":17032,"rva":28620240,"va":5397329360}
// 编译实例 RVAs: 0x1b4b5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":17696,"parent_record_offset":17032,"ancestor_record_offset":17032,"inlinee":29101,"source_line":2840,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":17900,"parent_record_offset":17032,"ancestor_record_offset":17032,"inlinee":5530,"source_line":177,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::split_at_checked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":17608,"parent_record_offset":17032,"ancestor_record_offset":17032,"inlinee":54262,"source_line":928,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":17164,"parent_record_offset":17032,"ancestor_record_offset":17032,"inlinee":7233,"source_line":2471,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1793,"record_offset":17276,"parent_record_offset":17248,"ancestor_record_offset":17032,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":17768,"parent_record_offset":17744,"ancestor_record_offset":17032,"inlinee":29090,"source_line":694,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":17840,"parent_record_offset":17804,"ancestor_record_offset":17032,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":17660,"parent_record_offset":17632,"ancestor_record_offset":17032,"inlinee":9300,"source_line":1181,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1793,"record_offset":17804,"parent_record_offset":17768,"ancestor_record_offset":17032,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":1793,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17032,"inlinee":29088,"source_line":60,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::eq_ignore_ascii_case_simple
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":1793,"record_offset":17744,"parent_record_offset":17720,"ancestor_record_offset":17032,"inlinee":29089,"source_line":83,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":17032,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1793,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":17032,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1793,"record_offset":17248,"parent_record_offset":17220,"ancestor_record_offset":17032,"inlinee":7237,"source_line":2650,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1793,"record_offset":17632,"parent_record_offset":17608,"ancestor_record_offset":17032,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1793,"record_offset":17220,"parent_record_offset":17192,"ancestor_record_offset":17032,"inlinee":7236,"source_line":1028,"archive":"_pdb/C/modules/01793.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":1793,"record_offset":17192,"parent_record_offset":17164,"ancestor_record_offset":17032,"inlinee":7235,"source_line":1037,"archive":"_pdb/C/modules/01793.jsonl.gz"}
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
// 完整结构及原始类型记录: identity.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_mods::identity::ModId
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >", "constructor": "tuple$", "arguments": ["tg_mods::identity::ModId", "enum2$<country_core::systems::steam::missing_mods::ModResolution>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::BTreeMap<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::BTreeMap", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::btree::map::BTreeMap<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::btree::map::BTreeMap<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::IntoIter", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::Entry::or_default<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::Entry::or_default", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<tuple$<tg_mods::identity::ModId,enum2$<country_core::systems::steam::missing_mods::ModResolution> >,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_mods::identity::ModId,tuple$<> >", "constructor": "tuple$", "arguments": ["tg_mods::identity::ModId", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tg_mods::identity::ModId", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tg_mods::identity::ModId,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tg_mods::identity::ModId,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$82::fmt<tg_mods::identity::ModId>", "constructor": "core::fmt::impl$82::fmt", "arguments": ["tg_mods::identity::ModId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::steam::missing_mods::MissingMod,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::systems::steam::missing_mods::MissingMod", "core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::systems::steam::missing_mods::MissingMod", "alloc::alloc::Global", "core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::systems::steam::missing_mods::MissingMod", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::systems::steam::missing_mods::MissingMod,alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::systems::steam::missing_mods::MissingMod", "alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::systems::steam::missing_mods::MissingMod,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::collections::btree::map::IntoIter<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>,country_core::systems::steam::missing_mods::collect_missing_mods::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::btree::node::Handle::insert_recursing<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global,alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "alloc::collections::btree::node::Handle::insert_recursing", "arguments": ["tg_mods::identity::ModId", "alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>", "alloc::alloc::Global", "alloc::collections::btree::map::entry::impl$8::insert_entry::closure_env$0<tg_mods::identity::ModId,alloc::vec::Vec<country_core::resources::clutter_registry::ClutterId,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::HashSet<tg_mods::identity::ModId,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::set::HashSet", "arguments": ["tg_mods::identity::ModId", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::set::HashSet<tg_mods::identity::ModId,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::set::HashSet<tg_mods::identity::ModId,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["tg_mods::identity::ModId", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tg_mods::identity::ModId>", "constructor": "ref$", "arguments": ["tg_mods::identity::ModId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_mods::identity::ModId> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<tg_mods::identity::ModId>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tg_mods::identity::RecordedModEntry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 57
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry>", "constructor": "serde_core::de::impls::impl$19::deserialize::VecVisitor", "arguments": ["tg_mods::identity::RecordedModEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::impl$7::deserialize_seq<serde_json::read::SliceRead,serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry> >", "constructor": "serde_json::de::impl$7::deserialize_seq", "arguments": ["serde_json::read::SliceRead", "serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$12::clone<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>", "constructor": "alloc::vec::impl$12::clone", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "serde_core::ser::SerializeMap::serialize_entry", "arguments": ["enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >", "str$", "alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tg_mods::identity::RecordedModEntry,serde_json::de::SeqAccess<serde_json::read::SliceRead> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tg_mods::identity::RecordedModEntry", "serde_json::de::SeqAccess<serde_json::read::SliceRead>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::calculate_hash<tg_mods::identity::RecordedModEntry>", "constructor": "utils::calculate_hash", "arguments": ["tg_mods::identity::RecordedModEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_platform::cell::sync_cell::SyncCell<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "bevy_platform::cell::sync_cell::SyncCell", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::mods::ModsTab>,bevy_platform::cell::sync_cell::SyncCell<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::mods::ModsTab>", "bevy_platform::cell::sync_cell::SyncCell<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::mods::ModsTab>,bevy_platform::cell::sync_cell::SyncCell<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<bevy_platform::cell::sync_cell::SyncCell<toolbar::menu::mods::ModsTab>,bevy_platform::cell::sync_cell::SyncCell<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "country_core::systems::steam::SteamOverlayOpenResources", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >", "tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >", "tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >", "tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >", "tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > > >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$1::dyn_hash<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> > >", "constructor": "bevy_ecs::label::impl$1::dyn_hash", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::initialize<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::get_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::set_last_run<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::apply_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::queue_deferred<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::check_change_tick<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::validate_param_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::name<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::flags<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::flags", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$4::deserialize_seq<serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry> >", "constructor": "serde_json::value::de::impl$4::deserialize_seq", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<tg_mods::identity::RecordedModEntry>", "constructor": "core::fmt::impl$80::fmt", "arguments": ["tg_mods::identity::RecordedModEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$28::fmt<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>", "constructor": "alloc::vec::impl$28::fmt", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tg_mods::identity::RecordedModEntry,ref_mut$<serde_json::value::de::SeqDeserializer> >", "constructor": "serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq", "arguments": ["tg_mods::identity::RecordedModEntry", "ref_mut$<serde_json::value::de::SeqDeserializer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "ref$", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >", "constructor": "serde_core::ser::Serializer::collect_seq", "arguments": ["serde_json::value::ser::Serializer", "ref$<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$8::serialize_field<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$8::serialize_field", "arguments": ["alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::visit_array<serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry> >", "constructor": "serde_json::value::de::visit_array", "arguments": ["serde_core::de::impls::impl$19::deserialize::VecVisitor<tg_mods::identity::RecordedModEntry>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>", "constructor": "core::slice::iter::Iter", "arguments": ["tg_mods::identity::RecordedModEntry"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>", "tg_mods::overlay::canonical_mod_entries::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tg_mods::identity::RecordedModEntry,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tg_mods::identity::RecordedModEntry", "core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>", "tg_mods::overlay::missing_entries::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0> >", "constructor": "core::iter::adapters::cloned::Cloned", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<tg_mods::identity::RecordedModEntry,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["tg_mods::identity::RecordedModEntry", "core::iter::adapters::cloned::Cloned<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tg_mods::identity::RecordedModEntry", "alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tg_mods::identity::RecordedModEntry,core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tg_mods::identity::RecordedModEntry", "core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>", "tg_mods::overlay::canonical_mod_entries::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$128::init_access<bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > > >", "constructor": "bevy_ecs::system::system_param::impl$128::init_access", "arguments": ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "country_core::systems::steam::SteamOverlayOpenResources", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >", "tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >", "tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >", "tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >", "tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >", "tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$12::default_system_sets<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>", "constructor": "bevy_ecs::system::function_system::impl$12::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)", "tuple$<>", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >),tuple$<>,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,country_core::systems::steam::SteamOverlayOpenResources,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_publish::WorkshopPublishState> > >,tuple$<enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_subscriptions::WorkshopSubscriptions> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::workshop_clutter::WorkshopClutterInstalls> > > >,tuple$<bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_subscriptions::UnsubscribeFromWorkshopItemCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd> >,tuple$<bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_publish::LocalMod,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<std::path::PathBuf,alloc::alloc::Global> >,bevy_ecs::system::system_param::Local<alloc::vec::Vec<country_core::systems::steam::workshop_subscriptions::WorkshopSubscription,alloc::alloc::Global> > >,bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,tuple$<bevy_ecs::system::system_param::Local<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::system::system_param::Local<bool> >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession> > >,tuple$<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound> >,tuple$<bevy_ecs::system::system_param::Local<toolbar::menu::mods::ModsTab>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: identity.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: identity.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 22; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::LastKnownGraphicsDevice (*)(ref$<rhapsody::raw::init::AvailableGraphicsDevice>)
// 候选参数类型: ["ref$<rhapsody::raw::init::AvailableGraphicsDevice>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::ModId::as_str
// 信息 ID: fe70b7b64a7c140f22dc5dd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::mint_into; module=1793, inline_record=24544; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::ModId::owner_hash8
// 信息 ID: 043edf49d1396afd714b120a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::identity::ModId::owner_matches @ 0x1b4b877 -> 0x1b4b710 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::mint_into @ 0x1b4bbff -> 0x1b4b710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::ModId::owner_matches
// 信息 ID: 5bac574551bf052af9f74a7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::iter::adapters::map::impl$2::next<country_core::systems::steam::missing_mods::ModClaimant,core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<core::ops::range::Range<u32>,steamworks::ugc::impl$22::iter::closure_env$0> >,country_core::systems::steam::workshop_subscriptions::lookup_mod_on_workshop::closure$0::closure_env$0> @ 0x15e9df4 -> 0x1b4b7d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::ModId::parse
// 信息 ID: 7e8327974cbdd2041e7e8139
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::missing_mods::collect_missing_mods @ 0xa0d8ac -> 0x1b4b930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::overlay::Mod::matches_recorded_entry @ 0xd8d575 -> 0x1b4b930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::read_id_file @ 0x1b4b0aa -> 0x1b4b930 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::RecordedModEntry::mod_id @ 0x1b4b5bd -> 0x1b4b930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::ModId::recorded_entry
// 信息 ID: dc7a35cb16fd8a7dbcc9b221
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::missing_mods::MissingModRef::display @ 0xa0d50a -> 0x1b4b8e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0> @ 0x231b0b2 -> 0x1b4b8e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tg_mods::identity::RecordedModEntry,core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> > > >; module=2045, inline_record=3388; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::RecordedModEntry::as_str
// 信息 ID: 2ac30ae7dc11b842a46d273b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 26 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672211 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x67227a -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672314 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::wall_selection_tags::wall_selection_tags @ 0x672d0b -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_textures::packaged_textures @ 0x9e32dd -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::missing_mods::MissingModRef::from_recorded_entry @ 0xa0d439 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::missing_mods::collect_missing_mods @ 0xa0dbb4 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93011 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::read_texture_list @ 0xa94371 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::reload_packaged_textures_by_name @ 0xa97101 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::start_upload @ 0xaff2bc -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_subscriptions::lookup_mod_on_workshop @ 0xb37e25 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb93812 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::grass2::read_grass2_runtime_config @ 0xc84941 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] clutter_icon_pipe::packaged_texture_blob_path @ 0xcef241 -> 0x698b50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::utils::wall_corners::add_wall_corners<system_wall_constructor::construct_walls::construct_walls::closure_env$3> @ 0x1436416 -> 0x698b50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::RecordedModEntry::mod_id
// 信息 ID: a2de88eb31b3ff46bcd9635c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::missing_mods::MissingModRef::from_recorded_entry @ 0xa0d428 -> 0x1b4b570 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::missing_mods::collect_missing_mods @ 0xa0db9f -> 0x1b4b570 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_mods::overlay::Mod::matches_recorded_entry; module=682, inline_record=94332; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::_::impl$0::deserialize
// 信息 ID: b6c67324a42b7788aa164826
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tg_mods::identity::RecordedModEntry,serde_json::de::SeqAccess<serde_json::read::SliceRead> >; module=166, inline_record=17916; 不是独立调用边。
// [PDB 内联宿主] serde_core::de::impls::impl$19::deserialize::impl$0::visit_seq<tg_mods::identity::RecordedModEntry,ref_mut$<serde_json::value::de::SeqDeserializer> >; module=1665, inline_record=96616; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::_::impl$0::serialize
// 信息 ID: b8d003953c504b191cbf29af
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >; module=144, inline_record=218704; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >; module=144, inline_record=218788; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >; module=1694, inline_record=198016; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::SerializeMap::serialize_entry<enum2$<serde_json::ser::Compound<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >,str$,alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> >; module=1694, inline_record=198100; 不是独立调用边。
// [PDB 内联宿主] serde_core::ser::Serializer::collect_seq<serde_json::value::ser::Serializer,ref$<alloc::vec::Vec<tg_mods::identity::RecordedModEntry,alloc::alloc::Global> > >; module=1802, inline_record=27724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::folder_mod_id
// 信息 ID: 0ce39caf2bce7935db844ee0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd91738 -> 0x1b4b480 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::folder_mod_id::closure$0
// 信息 ID: bd88c48cd92ccff4af42ad1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::folder_mod_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::folder_mod_id; module=1793, inline_record=15760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$0::fmt
// 信息 ID: 6d1440a0a8d7c61c62651ac2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::steam::missing_mods::MissingModRef::display @ 0xa0d549 -> 0x1356e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::fmt::impl$80::fmt<http::uri::authority::Authority> | core::fmt::impl$82::fmt<http::uri::authority::Authority> | core::fmt::impl$82::fmt<tg_mods::identity::ModId> @ 0x13157b3 -> 0x1356e60 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$11::eq
// 信息 ID: f209789348b84639c6af8411
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_workshop_mod; module=559, inline_record=17916; 不是独立调用边。
// [PDB 内联宿主] tg_mods::overlay::set_active_mods; module=682, inline_record=61632; 不是独立调用边。
// [PDB 内联宿主] tg_mods::overlay::set_active_mods; module=682, inline_record=62784; 不是独立调用边。
// [PDB 内联宿主] tg_mods::overlay::Mod::matches_recorded_entry; module=682, inline_record=95200; 不是独立调用边。
// [PDB 内联宿主] tg_mods::overlay::add_mods; module=682, inline_record=103248; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<tg_mods::identity::ModId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>; module=2030, inline_record=8688; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$13::clone
// 信息 ID: accbb238ab323e86ed7083c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::impl$12::clone<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>; module=81, inline_record=69448; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>; module=1267, inline_record=10676; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$12::clone<tg_mods::identity::RecordedModEntry,alloc::alloc::Global>; module=1643, inline_record=278448; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tg_mods::identity::RecordedModEntry,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0> > >; module=2029, inline_record=38040; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tg_mods::identity::RecordedModEntry,core::iter::adapters::cloned::Cloned<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::missing_entries::closure_env$0> > >; module=2029, inline_record=39104; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<tg_mods::identity::RecordedModEntry,core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::vec::impl$21::extend_trusted::closure_env$0<tg_mods::identity::RecordedModEntry,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<tg_mods::identity::RecordedModEntry>,tg_mods::overlay::canonical_mod_entries::closure_env$0> > > >; module=2045, inline_record=3064; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0>; module=2873, inline_record=17212; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::mods_glade_tab::apply_button; module=2896, inline_record=5708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$15::hash
// 信息 ID: 6794870e93bb2acfe81cde69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::calculate_hash<tg_mods::identity::RecordedModEntry>; module=1237, inline_record=5644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$17::eq
// 信息 ID: ce9542e52ed642da939b513d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::widgets::ScrollingListView::with_items<toolbar::menu::mod_checklist::replacement_mod_checklist::closure_env$0>; module=2873, inline_record=17384; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::mods_glade_tab::apply_button; module=2896, inline_record=1828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$1::owner_matches::closure$0
// 信息 ID: 4f62cc8fab411d41557ee4e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::impl$1::owner_matches", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::ModId::owner_matches; module=1793, inline_record=19244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$1::parse::closure$0
// 信息 ID: 88d9e7386f71d276660de7a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::impl$1::parse", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::ModId::parse; module=1793, inline_record=21148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$3::fmt
// 信息 ID: b7bf6208a327475b2da9bf39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$80::fmt<tg_mods::identity::RecordedModEntry> @ 0x1515f53 -> 0x1030f0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$5::clone
// 信息 ID: e21847402fd90317540d1ced
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::save::required_clutter::warn_missing_mods_on_load; module=509, inline_record=32792; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::missing_mods::MissingMods::drain_lookup_results; module=529, inline_record=38828; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::missing_mods::refresh_missing_mods_on_pack_change; module=529, inline_record=57980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::mod_editor::mod_editor_tick; module=546, inline_record=67292; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::mod_editor::mod_editor_tick; module=546, inline_record=86108; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_mods::register_workshop_mod; module=559, inline_record=15952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::steam::workshop_subscriptions::lookup_mod_on_workshop::closure$0; module=566, inline_record=54236; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::clutter_packs::discover_mod_clutter; module=612, inline_record=21320; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$6::fmt
// 信息 ID: 5217f0722372e4e8ea273188
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$8::hash
// 信息 ID: 1e20b06e0cf8240dabc52666
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<tg_mods::identity::ModId> >; module=2031, inline_record=584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::impl$9::cmp
// 信息 ID: dd6d07f4fc151a8b904363f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::steam::missing_mods::collect_missing_mods; module=529, inline_record=46180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::is_lowercase_hex
// 信息 ID: 5a283e7e76d934cd5c822554
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::ModId::parse; module=1793, inline_record=20424; 不是独立调用边。
// [PDB 内联宿主] tg_mods::identity::ModId::parse; module=1793, inline_record=20812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::is_lowercase_hex::closure$0
// 信息 ID: 248d65e57f42937182b01fd9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::is_lowercase_hex", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::ModId::parse; module=1793, inline_record=20672; 不是独立调用边。
// [PDB 内联宿主] tg_mods::identity::ModId::parse; module=1793, inline_record=21040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::mint
// 信息 ID: d403c88402891ba0bd6a45e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::mint_into; module=1793, inline_record=21900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::mint::closure$0
// 信息 ID: 4624874aab70e1179d248448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::mint", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::mint_into; module=1793, inline_record=22120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::mint_into
// 信息 ID: d7e0aee2111f5983d44c222f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::identity::zip_mod_id @ 0x1b4a5d3 -> 0x1b4bba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::folder_mod_id @ 0x1b4b4eb -> 0x1b4bba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::read_id_file
// 信息 ID: ab8bb49f87104ecee8a43966
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::mod_at_root @ 0xd8f4d4 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::zip_mod_id @ 0x1b4a238 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::zip_mod_id @ 0x1b4a5a5 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::read_mod_id @ 0x1b4aaae -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_mods::identity::folder_mod_id @ 0x1b4b4c7 -> 0x1b4ab10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::read_id_file::closure$0
// 信息 ID: 2a54ab8e0d0443a262f0f081
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::read_id_file", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::read_id_file; module=1793, inline_record=11296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::read_mod_id
// 信息 ID: 11c2484da1a6a05f0dddce7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::mod_editor::mod_editor_tick @ 0xaa1f29 -> 0x1b4aa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::steam::workshop_publish::validate_for_publish @ 0xb00b89 -> 0x1b4aa70 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] tg_mods::discovery::mod_at_root; module=683, inline_record=5348; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::rename_legacy_sidecar
// 信息 ID: 58d2968ff49eb9ef15b7605b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::zip_mod_id; module=1793, inline_record=5068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::sidecar_zip_stem
// 信息 ID: aab5bd0a3d208efdec87f7b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::discover_mods @ 0xd90726 -> 0x1b4b5d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::zip_mod_id
// 信息 ID: db989d35fa02155909f34cfa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tg_mods::discovery::scan_mods_root @ 0xd92335 -> 0x1b4a1d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::zip_mod_id::closure$0
// 信息 ID: 9f6e5bde56d9389b850acfd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::zip_mod_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::zip_mod_id; module=1793, inline_record=8096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::zip_sidecar_path
// 信息 ID: 62717acfd5a907805699c622
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::zip_mod_id; module=1793, inline_record=2852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::identity::zip_sidecar_path::closure$0
// 信息 ID: 22e09e4be9ed033ca1436263
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::identity::zip_sidecar_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::identity::zip_mod_id; module=1793, inline_record=2936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
