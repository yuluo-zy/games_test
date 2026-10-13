// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/tg-mods/src/legacy_texture.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 19d5d14c734bf91d27d42a61ec629d64023b0aaf05ebc334d8ca793070bec01f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: legacy_texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 1778, 3356
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tg_mods::legacy_texture::parse_legacy_texture
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1778,"record_offset":1204,"rva":28465488,"va":5397174608}
// 编译实例 RVAs: 0x1b25950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::backtrace::Backtrace>::capture
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12515655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8b90641ff","size":5,"soft_fail":false,"target_va":5384659264,"va":5397174914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21496122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8c6feb7fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5397174805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21496612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8dcfcb7fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397175311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: legacy_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28452605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e803d94dfe","size":5,"soft_fail":false,"target_va":5368722192,"va":5397174792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: legacy_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19882217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8e9602f01","size":5,"soft_fail":false,"target_va":5417057056,"va":5397174834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e891fdffff","size":5,"soft_fail":false,"target_va":5397174464,"va":5397175082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12380042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8761843ff","size":5,"soft_fail":false,"target_va":5384794864,"va":5397174901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19445551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e82fb72801","size":5,"soft_fail":false,"target_va":5416620794,"va":5397175238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19601959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8271a2b01","size":5,"soft_fail":false,"target_va":5416777152,"va":5397175188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h442f4b15ebd1d3f6 (.llvm.17100869928342201643)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19675103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8df372c01","size":5,"soft_fail":false,"target_va":5416850064,"va":5397174956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::msg
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t19674329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397174608,"raw_hex":"e8d9342c01","size":5,"soft_fail":false,"target_va":5416849424,"va":5397175090,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1864,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1778,"record_offset":2280,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":30730,"source_line":103,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":1812,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":8618,"source_line":1578,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":1784,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::legacy_texture::parse_legacy_texture::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":1352,"parent_record_offset":1204,"ancestor_record_offset":1204,"inlinee":86291,"source_line":72,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1204,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2132,"parent_record_offset":2108,"ancestor_record_offset":1204,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1204,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":1204,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1204,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":1204,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2084,"parent_record_offset":2052,"ancestor_record_offset":1204,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1778,"record_offset":2620,"parent_record_offset":2564,"ancestor_record_offset":1204,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":1204,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":1204,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1204,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":1204,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":1204,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2052,"parent_record_offset":2020,"ancestor_record_offset":1204,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2020,"parent_record_offset":1992,"ancestor_record_offset":1204,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1778,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":1204,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1896,"parent_record_offset":1864,"ancestor_record_offset":1204,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1778,"record_offset":1928,"parent_record_offset":1896,"ancestor_record_offset":1204,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1778,"record_offset":1964,"parent_record_offset":1928,"ancestor_record_offset":1204,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::construct_from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1778,"record_offset":2440,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":6351,"source_line":212,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1778,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1204,"inlinee":9708,"source_line":258,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1778,"record_offset":1372,"parent_record_offset":1352,"ancestor_record_offset":1204,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":1204,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1778,"record_offset":2212,"parent_record_offset":1928,"ancestor_record_offset":1204,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1778,"record_offset":2236,"parent_record_offset":2212,"ancestor_record_offset":1204,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1778,"record_offset":2468,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1778,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1204,"inlinee":9709,"source_line":1711,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::legacy_texture::parse_legacy_texture::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":1204,"inlinee":86292,"source_line":95,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::legacy_texture::parse_legacy_texture::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/legacy_texture.rs
//     evidence={"module":1778,"record_offset":2328,"parent_record_offset":2280,"ancestor_record_offset":1204,"inlinee":86293,"source_line":96,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::legacy_texture::parse_legacy_texture::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15113
// 首条位置: {"module":1778,"inlinee":86291}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":1778,"record_offset":1372,"parent_record_offset":1352,"ancestor_record_offset":1352,"inlinee":6246,"source_line":818,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$7::try_from
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\mod.rs
//     evidence={"module":1778,"record_offset":1396,"parent_record_offset":1372,"ancestor_record_offset":1352,"inlinee":9708,"source_line":258,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::copied
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1778,"record_offset":1420,"parent_record_offset":1396,"ancestor_record_offset":1352,"inlinee":9709,"source_line":1711,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::legacy_texture::parse_legacy_texture::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15114
// 首条位置: {"module":1778,"inlinee":86292}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tg_mods::legacy_texture::parse_legacy_texture::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15115
// 首条位置: {"module":1778,"inlinee":86293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2352,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1778,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":2328,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01778.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1778,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":2328,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01778.jsonl.gz"}
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
// 完整结构及原始类型记录: legacy_texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: legacy_texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: legacy_texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tg_mods::legacy_texture::parse_legacy_texture
// 信息 ID: 7cf88882d05cee217de9f388
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e42ec -> 0x1b25950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::legacy_texture::parse_legacy_texture::closure$0
// 信息 ID: 16ec14dc67cf9287df2d4e1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tg_mods::legacy_texture::parse_legacy_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=1352; 不是独立调用边。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=1460; 不是独立调用边。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=1568; 不是独立调用边。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=1676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::legacy_texture::parse_legacy_texture::closure$3
// 信息 ID: ca4f04d12b4d7d42260720ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tg_mods::legacy_texture::parse_legacy_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=1836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tg_mods::legacy_texture::parse_legacy_texture::closure$4
// 信息 ID: e06855eaaebdbde8935bb651
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tg_mods::legacy_texture::parse_legacy_texture", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tg_mods::legacy_texture::parse_legacy_texture; module=1778, inline_record=2328; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
