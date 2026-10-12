// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tiny-glade/src/load_language_library.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 3cbf4d8ddbd489796147bb337ec85aa5c07d6805ef334adc17185f35fa02327e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_language_library.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 12
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use steamworks; // namespace candidate
//   package identities: [{"name":"steamworks","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466","version":"0.11.0"}]
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":12,"record_offset":536,"rva":992992,"va":5369702112}
// 编译实例 RVAs: 0xf26e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <str>::to_lowercase
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15084809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8092de600","size":5,"soft_fail":false,"target_va":5384787088,"va":5369702274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t14906407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e82774e300","size":5,"soft_fail":false,"target_va":5384609408,"va":5369702996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5976520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8c8315b00","size":5,"soft_fail":false,"target_va":5375678688,"va":5369702163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t5976400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e850315b00","size":5,"soft_fail":false,"target_va":5375678704,"va":5369702299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-979961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8070cf1ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5369702148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140196c30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t672959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8bf440a00","size":5,"soft_fail":false,"target_va":5370375216,"va":5369702252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6126842","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8fa7c5d00","size":5,"soft_fail":false,"target_va":5375829056,"va":5369702209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1b2a0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_language_library.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t47352657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8518bd202","size":5,"soft_fail":false,"target_va":5417054880,"va":5369702218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46917561","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8b9e7cb02","size":5,"soft_fail":false,"target_va":5416620794,"va":5369703228,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t12295085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8ad9bbb00","size":5,"soft_fail":false,"target_va":5381997408,"va":5369702318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14920547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e863abe300","size":5,"soft_fail":false,"target_va":5384624080,"va":5369703528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t46918417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e811ebcb02","size":5,"soft_fail":false,"target_va":5416621632,"va":5369703210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::load_locale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13427928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5369702112,"raw_hex":"e8d8e4cc00","size":5,"soft_fail":false,"target_va":5383131056,"va":5369703123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":12,"record_offset":708,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":4704,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":12,"record_offset":1176,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":12,"record_offset":4872,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5910,"source_line":1038,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":4968,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":12,"record_offset":4900,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":12,"record_offset":2924,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":4760,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":4620,"parent_record_offset":536,"ancestor_record_offset":536,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":536,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1020,"parent_record_offset":996,"ancestor_record_offset":536,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":536,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":536,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1044,"parent_record_offset":1020,"ancestor_record_offset":536,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":536,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":536,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1564,"parent_record_offset":1508,"ancestor_record_offset":536,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":884,"parent_record_offset":856,"ancestor_record_offset":536,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":536,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":536,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":944,"parent_record_offset":912,"ancestor_record_offset":536,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":912,"parent_record_offset":884,"ancestor_record_offset":536,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":536,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":764,"parent_record_offset":736,"ancestor_record_offset":536,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":792,"parent_record_offset":764,"ancestor_record_offset":536,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":536,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":12,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":536,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":536,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":536,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":536,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":856,"parent_record_offset":820,"ancestor_record_offset":536,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":2952,"parent_record_offset":2924,"ancestor_record_offset":536,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":12,"record_offset":4728,"parent_record_offset":4704,"ancestor_record_offset":536,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":12,"record_offset":1100,"parent_record_offset":820,"ancestor_record_offset":536,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":1124,"parent_record_offset":1100,"ancestor_record_offset":536,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":1412,"parent_record_offset":1228,"ancestor_record_offset":536,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":1344,"parent_record_offset":1312,"ancestor_record_offset":536,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":12,"record_offset":2980,"parent_record_offset":2952,"ancestor_record_offset":536,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":12,"record_offset":3012,"parent_record_offset":2980,"ancestor_record_offset":536,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":4644,"parent_record_offset":4620,"ancestor_record_offset":536,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1372,"parent_record_offset":1312,"ancestor_record_offset":536,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":4668,"parent_record_offset":4644,"ancestor_record_offset":536,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1840,"parent_record_offset":1780,"ancestor_record_offset":536,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1872,"parent_record_offset":1744,"ancestor_record_offset":536,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":536,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":12,"record_offset":4784,"parent_record_offset":4760,"ancestor_record_offset":536,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":12,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":536,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":12,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":536,"inlinee":5904,"source_line":355,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::load_language_library::load_language_library::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/load_language_library.rs
//     evidence={"module":12,"record_offset":1228,"parent_record_offset":1176,"ancestor_record_offset":536,"inlinee":5902,"source_line":8,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::load_language_library::load_language_library::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x170E
// 首条位置: {"module":12,"inlinee":5902}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":1412,"parent_record_offset":1228,"ancestor_record_offset":1228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] steamworks::Client::apps
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"steamworks","version":"0.11.0","source":"git+https://github.com/PounceLight/steamworks-rs.git?rev=f441c6128f04669ce471cf7bfd85e70755a9b466#f441c6128f04669ce471cf7bfd85e70755a9b466"}]
//     definition_source=/home/h3/.cargo/git/checkouts/steamworks-rs-f13bba748a20c02a/f441c61/src/lib.rs
//     evidence={"module":12,"record_offset":1256,"parent_record_offset":1228,"ancestor_record_offset":1228,"inlinee":5904,"source_line":355,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":1228,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1228,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1636,"parent_record_offset":1612,"ancestor_record_offset":1228,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":12,"record_offset":1564,"parent_record_offset":1508,"ancestor_record_offset":1228,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1536,"parent_record_offset":1508,"ancestor_record_offset":1228,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":1228,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":12,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1228,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":1228,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1284,"parent_record_offset":1256,"ancestor_record_offset":1228,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":12,"record_offset":1744,"parent_record_offset":1720,"ancestor_record_offset":1228,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":1228,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":1344,"parent_record_offset":1312,"ancestor_record_offset":1228,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1372,"parent_record_offset":1312,"ancestor_record_offset":1228,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1840,"parent_record_offset":1780,"ancestor_record_offset":1228,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":12,"record_offset":1872,"parent_record_offset":1744,"ancestor_record_offset":1228,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00012.jsonl.gz"}
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
// 完整结构及原始类型记录: load_language_library.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_language_library.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_language_library.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::panic_repro_state::PanicReproState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>

// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library
// 信息 ID: e70a3e4e81591647aaf9598f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x6436d2 -> 0xf26e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::load_language_library::load_language_library::closure$0
// 信息 ID: a6b7afdf2cb49e48e5e38d01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::load_language_library::load_language_library", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::load_language_library::load_language_library; module=12, inline_record=1228; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
