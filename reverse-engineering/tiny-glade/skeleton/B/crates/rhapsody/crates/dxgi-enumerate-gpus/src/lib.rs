// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 37409d5a7eec2db599e19c61724520810d98e8309977176ebb8b5ddb5412f5ff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1934
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use dxgi_enumerate_gpus; // namespace candidate
//   package identities: [{"name":"dxgi-enumerate-gpus","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use windows; // namespace candidate
//   package identities: [{"name":"windows","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.37.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::enumerate
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1934,"record_offset":268,"rva":25897408,"va":5394606528}
// 编译实例 RVAs: 0x18b29c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::fold
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8949218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8e28d8800","size":5,"soft_fail":false,"target_va":5403556304,"va":5394607081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$windows..core..error..Error$u20$as$u20$core..convert..From$LT$windows..core..hresult..HRESULT$GT$$GT$::from
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-816901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8fb88f3ff","size":5,"soft_fail":false,"target_va":5393790528,"va":5394607424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014a860
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24544920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8687989fe","size":5,"soft_fail":false,"target_va":5370062944,"va":5394607859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19526176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8e00dd6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5394607675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f7abf0
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9666265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e827816cff","size":5,"soft_fail":false,"target_va":5384940528,"va":5394606788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14213b8e0
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8949344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8608e8800","size":5,"soft_fail":false,"target_va":5403556064,"va":5394606715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14213b950
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8949414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e8a68e8800","size":5,"soft_fail":false,"target_va":5403556176,"va":5394606757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17240351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e81f110701","size":5,"soft_fail":false,"target_va":5411847168,"va":5394606812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-20251022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e872fecafe","size":5,"soft_fail":false,"target_va":5374356144,"va":5394607161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t17189534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e89e4a0601","size":5,"soft_fail":false,"target_va":5411796512,"va":5394606973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h4773efd2edfc9027 (.llvm.17888871689599594477)
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t17189166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e82e490601","size":5,"soft_fail":false,"target_va":5411796768,"va":5394607597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11912845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394606528,"raw_hex":"e873394aff","size":5,"soft_fail":false,"target_va":5382694656,"va":5394607496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*26084778(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5394606528,"raw_hex":"ff15aa058e01","size":6,"soft_fail":false,"va":5394606624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":408,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":1796,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1934,"record_offset":2784,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1934,"record_offset":688,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1934,"record_offset":3008,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] dxgi_enumerate_gpus::get_string_from_wide
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"dxgi-enumerate-gpus","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1934,"record_offset":828,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":86718,"source_line":36,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] windows::Windows::Win32::Graphics::Dxgi::CreateDXGIFactory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-0.37.0/src/Windows/Win32/Graphics/Dxgi/mod.rs
//     evidence={"module":1934,"record_offset":436,"parent_record_offset":268,"ancestor_record_offset":268,"inlinee":86716,"source_line":5,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1934,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":268,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1934,"record_offset":1620,"parent_record_offset":1564,"ancestor_record_offset":268,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1876,"parent_record_offset":1844,"ancestor_record_offset":268,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":268,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":268,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":268,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1392,"parent_record_offset":1360,"ancestor_record_offset":268,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1924,"parent_record_offset":1900,"ancestor_record_offset":268,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1900,"parent_record_offset":1876,"ancestor_record_offset":268,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":268,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":268,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf16_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1008,"parent_record_offset":828,"ancestor_record_offset":268,"inlinee":43695,"source_line":774,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1124,"parent_record_offset":1076,"ancestor_record_offset":268,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1276,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":19169,"source_line":1226,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$13::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1156,"parent_record_offset":1076,"ancestor_record_offset":268,"inlinee":19168,"source_line":2362,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":268,"inlinee":19183,"source_line":2283,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":1844,"parent_record_offset":1796,"ancestor_record_offset":268,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":268,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":268,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":1934,"record_offset":2856,"parent_record_offset":2816,"ancestor_record_offset":268,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":1934,"record_offset":2816,"parent_record_offset":2784,"ancestor_record_offset":268,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::char::decode::impl$0::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\decode.rs
//     evidence={"module":1934,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":268,"inlinee":43696,"source_line":76,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1934,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1934,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":268,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1934,"record_offset":1436,"parent_record_offset":1156,"ancestor_record_offset":268,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1934,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":268,"inlinee":6245,"source_line":3197,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1934,"record_offset":3504,"parent_record_offset":3476,"ancestor_record_offset":268,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1934,"record_offset":540,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1934,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":268,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1934,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":268,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1934,"record_offset":1964,"parent_record_offset":1796,"ancestor_record_offset":268,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1934,"record_offset":1704,"parent_record_offset":1008,"ancestor_record_offset":268,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1934,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":268,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1934,"record_offset":944,"parent_record_offset":868,"ancestor_record_offset":268,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1934,"record_offset":868,"parent_record_offset":828,"ancestor_record_offset":268,"inlinee":10616,"source_line":331,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] dxgi_enumerate_gpus::get_string_from_wide::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"dxgi-enumerate-gpus","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1934,"record_offset":904,"parent_record_offset":868,"ancestor_record_offset":268,"inlinee":86720,"source_line":37,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] windows::core::hresult::HRESULT::and_some
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-0.37.0/src/core/hresult.rs
//     evidence={"module":1934,"record_offset":472,"parent_record_offset":436,"ancestor_record_offset":268,"inlinee":33803,"source_line":46,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] windows::core::hresult::HRESULT::is_ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"windows","version":"0.37.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/windows-0.37.0/src/core/hresult.rs
//     evidence={"module":1934,"record_offset":512,"parent_record_offset":472,"ancestor_record_offset":268,"inlinee":33763,"source_line":14,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152BE
// 首条位置: {"module":1934,"inlinee":86718}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::String::from_utf16_lossy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1008,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":43695,"source_line":774,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::position
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1934,"record_offset":868,"parent_record_offset":828,"ancestor_record_offset":828,"inlinee":10616,"source_line":331,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1934,"record_offset":1644,"parent_record_offset":1620,"ancestor_record_offset":828,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1934,"record_offset":1620,"parent_record_offset":1564,"ancestor_record_offset":828,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1332,"parent_record_offset":1304,"ancestor_record_offset":828,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":828,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":828,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1392,"parent_record_offset":1360,"ancestor_record_offset":828,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":828,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1934,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":828,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1124,"parent_record_offset":1076,"ancestor_record_offset":828,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1276,"parent_record_offset":1156,"ancestor_record_offset":828,"inlinee":19169,"source_line":1226,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$13::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1156,"parent_record_offset":1076,"ancestor_record_offset":828,"inlinee":19168,"source_line":2362,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$7::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1934,"record_offset":1076,"parent_record_offset":1044,"ancestor_record_offset":828,"inlinee":19183,"source_line":2283,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1934,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":828,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::char::decode::impl$0::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\char\decode.rs
//     evidence={"module":1934,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":828,"inlinee":43696,"source_line":76,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1934,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":828,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1934,"record_offset":1044,"parent_record_offset":1008,"ancestor_record_offset":828,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1934,"record_offset":1436,"parent_record_offset":1156,"ancestor_record_offset":828,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1934,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":828,"inlinee":6245,"source_line":3197,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1934,"record_offset":1468,"parent_record_offset":1076,"ancestor_record_offset":828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1934,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":828,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1934,"record_offset":972,"parent_record_offset":944,"ancestor_record_offset":828,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1934,"record_offset":1704,"parent_record_offset":1008,"ancestor_record_offset":828,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1934,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":828,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1934,"record_offset":944,"parent_record_offset":868,"ancestor_record_offset":828,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   [pdb_inline_transitive] dxgi_enumerate_gpus::get_string_from_wide::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"dxgi-enumerate-gpus","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs
//     evidence={"module":1934,"record_offset":904,"parent_record_offset":868,"ancestor_record_offset":828,"inlinee":86720,"source_line":37,"archive":"_pdb/B/modules/01934.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: dxgi_enumerate_gpus::get_string_from_wide::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x152C0
// 首条位置: {"module":1934,"inlinee":86720}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::enumerate
// 信息 ID: e9af889c5e8bf24af7499b61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::init::GraphicsDevicesForSurface::missing_devices @ 0xbdb18c -> 0x18b29c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::get_string_from_wide
// 信息 ID: 9ae062e39546f78cf2f597dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] dxgi_enumerate_gpus::enumerate; module=1934, inline_record=828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: dxgi_enumerate_gpus::get_string_from_wide::closure$0
// 信息 ID: 51962a5a153be3b80375174b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "dxgi_enumerate_gpus::get_string_from_wide", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] dxgi_enumerate_gpus::enumerate; module=1934, inline_record=904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
