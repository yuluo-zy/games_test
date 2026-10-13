// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/render/load_texture.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: a31916cee3afc1ee8e8ba05deb577172cd1c12c56bab9561d0964d80b56600fc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 505
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use memmap2; // namespace candidate
//   package identities: [{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.8"},{"name":"memmap2","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.4"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3","version":"0.7.39"}]
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_compiled_texture_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":505,"record_offset":25892,"rva":9533536,"va":5378242656}
// 编译实例 RVAs: 0x917860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4400830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8be264300","size":5,"soft_fail":false,"target_va":5382644528,"va":5378243693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$memmap2..os..MmapInner$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t14109191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8074ad700","size":5,"soft_fail":false,"target_va":5392353392,"va":5378244196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3344885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e80bf6ccff","size":5,"soft_fail":false,"target_va":5374900736,"va":5378245616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3161920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8c0c0cfff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378243419,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2727488","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8c061d6ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378243515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1350216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8b865ebff","size":5,"soft_fail":false,"target_va":5376895600,"va":5378245811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d61b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4497422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e80ea04400","size":5,"soft_fail":false,"target_va":5382740736,"va":5378243309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4584402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8d2f34500","size":5,"soft_fail":false,"target_va":5382827728,"va":5378243321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a6700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14215375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8cfe8d800","size":5,"soft_fail":false,"target_va":5392459520,"va":5378244140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f87d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33425624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8d808fe01","size":5,"soft_fail":false,"target_va":5411669968,"va":5378244339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33297182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e81e13fc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5378244397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33297947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e81b16fc01","size":5,"soft_fail":false,"target_va":5411542422,"va":5378244470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33297258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e86a13fc01","size":5,"soft_fail":false,"target_va":5411541904,"va":5378244641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33298350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8ae17fc01","size":5,"soft_fail":false,"target_va":5411542880,"va":5378244525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33298682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8fa18fc01","size":5,"soft_fail":false,"target_va":5411543104,"va":5378244417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e87dd6ffff","size":5,"soft_fail":false,"target_va":5378232112,"va":5378242734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t33300544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e84020fc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5378242811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::MmapInner::map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t14110108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e89c4dd700","size":5,"soft_fail":false,"target_va":5392353952,"va":5378243839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] memmap2::os::file_len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"memmap2","version":"0.9.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2601520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e830b22700","size":5,"soft_fail":false,"target_va":5380845328,"va":5378243803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3142288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e890f22f00","size":5,"soft_fail":false,"target_va":5381385536,"va":5378243243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3147466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8ca063000","size":5,"soft_fail":false,"target_va":5381390640,"va":5378243169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3142181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e825f22f00","size":5,"soft_fail":false,"target_va":5381386480,"va":5378244294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3141985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e861f12f00","size":5,"soft_fail":false,"target_va":5381384704,"va":5378242714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::array_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2873168","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e850d72b00","size":5,"soft_fail":false,"target_va":5381117248,"va":5378244075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2872968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e888d62b00","size":5,"soft_fail":false,"target_va":5381117024,"va":5378244051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2872526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8ced42b00","size":5,"soft_fail":false,"target_va":5381116528,"va":5378243997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2872766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8bed52b00","size":5,"soft_fail":false,"target_va":5381116784,"va":5378244013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4464887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8f7204400","size":5,"soft_fail":false,"target_va":5382708672,"va":5378243780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4500726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8f6ac4400","size":5,"soft_fail":false,"target_va":5382744080,"va":5378243349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_with_extension
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4501192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8c8ae4400","size":5,"soft_fail":false,"target_va":5382744592,"va":5378243395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10580123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e89b70a100","size":5,"soft_fail":false,"target_va":5388825296,"va":5378245168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33292714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8aa01fc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5378244385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] ustr::Ustr::from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3110387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378242656,"raw_hex":"e8f3752f00","size":5,"soft_fail":false,"target_va":5381354480,"va":5378244088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*42444154(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5378242656,"raw_hex":"ff157aa58702","size":6,"soft_fail":false,"va":5378244208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":505,"record_offset":29644,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":30644,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":28936,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":30472,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":30700,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":30168,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":30512,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":29676,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":29560,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] memmap2::Mmap::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":505,"record_offset":30268,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":13581,"source_line":579,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":27556,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":26124,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":26580,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::bytes_as_slice
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":505,"record_offset":30612,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":28655,"source_line":22,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::ArchivedVec::as_slice
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":505,"record_offset":30820,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":28659,"source_line":50,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":505,"record_offset":29852,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":28736,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::with_extension
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":28904,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":21959,"source_line":2744,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":28540,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5169,"source_line":1825,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":28596,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":505,"record_offset":30764,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_direct] ustr::ustr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":505,"record_offset":30736,"parent_record_offset":25892,"ancestor_record_offset":25892,"inlinee":8806,"source_line":469,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":29184,"parent_record_offset":29160,"ancestor_record_offset":25892,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":505,"record_offset":29160,"parent_record_offset":29104,"ancestor_record_offset":25892,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":29132,"parent_record_offset":29104,"ancestor_record_offset":25892,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":29104,"parent_record_offset":29080,"ancestor_record_offset":25892,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":505,"record_offset":29080,"parent_record_offset":29056,"ancestor_record_offset":25892,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":27892,"parent_record_offset":27864,"ancestor_record_offset":25892,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":27944,"parent_record_offset":27920,"ancestor_record_offset":25892,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":27920,"parent_record_offset":27892,"ancestor_record_offset":25892,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":27840,"parent_record_offset":27812,"ancestor_record_offset":25892,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":27864,"parent_record_offset":27840,"ancestor_record_offset":25892,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":28036,"parent_record_offset":27864,"ancestor_record_offset":25892,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":505,"record_offset":28148,"parent_record_offset":28124,"ancestor_record_offset":25892,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":505,"record_offset":29936,"parent_record_offset":29852,"ancestor_record_offset":25892,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::impl$116::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":505,"record_offset":30668,"parent_record_offset":30644,"ancestor_record_offset":25892,"inlinee":28657,"source_line":255,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":30232,"parent_record_offset":30204,"ancestor_record_offset":25892,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":505,"record_offset":27968,"parent_record_offset":27944,"ancestor_record_offset":25892,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":505,"record_offset":26280,"parent_record_offset":26248,"ancestor_record_offset":25892,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":505,"record_offset":27708,"parent_record_offset":27664,"ancestor_record_offset":25892,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":505,"record_offset":30344,"parent_record_offset":30320,"ancestor_record_offset":25892,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":29732,"parent_record_offset":29700,"ancestor_record_offset":25892,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":505,"record_offset":29760,"parent_record_offset":29700,"ancestor_record_offset":25892,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":505,"record_offset":26528,"parent_record_offset":26500,"ancestor_record_offset":25892,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":505,"record_offset":28496,"parent_record_offset":27664,"ancestor_record_offset":25892,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":28100,"parent_record_offset":27812,"ancestor_record_offset":25892,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":505,"record_offset":26992,"parent_record_offset":26968,"ancestor_record_offset":25892,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":505,"record_offset":26968,"parent_record_offset":26944,"ancestor_record_offset":25892,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":505,"record_offset":27176,"parent_record_offset":27148,"ancestor_record_offset":25892,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":505,"record_offset":27992,"parent_record_offset":27968,"ancestor_record_offset":25892,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":30432,"parent_record_offset":30292,"ancestor_record_offset":25892,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":505,"record_offset":30392,"parent_record_offset":30368,"ancestor_record_offset":25892,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":30568,"parent_record_offset":30540,"ancestor_record_offset":25892,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":505,"record_offset":30540,"parent_record_offset":30512,"ancestor_record_offset":25892,"inlinee":9466,"source_line":521,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":505,"record_offset":26920,"parent_record_offset":26896,"ancestor_record_offset":25892,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":505,"record_offset":26944,"parent_record_offset":26920,"ancestor_record_offset":25892,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":505,"record_offset":27248,"parent_record_offset":27220,"ancestor_record_offset":25892,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":505,"record_offset":26896,"parent_record_offset":26780,"ancestor_record_offset":25892,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":505,"record_offset":26248,"parent_record_offset":26220,"ancestor_record_offset":25892,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":505,"record_offset":26608,"parent_record_offset":26580,"ancestor_record_offset":25892,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":505,"record_offset":26748,"parent_record_offset":26716,"ancestor_record_offset":25892,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":505,"record_offset":26716,"parent_record_offset":26680,"ancestor_record_offset":25892,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":505,"record_offset":26656,"parent_record_offset":26632,"ancestor_record_offset":25892,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":505,"record_offset":26680,"parent_record_offset":26656,"ancestor_record_offset":25892,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":505,"record_offset":26632,"parent_record_offset":26608,"ancestor_record_offset":25892,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":505,"record_offset":27220,"parent_record_offset":26680,"ancestor_record_offset":25892,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26220,"parent_record_offset":26188,"ancestor_record_offset":25892,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26188,"parent_record_offset":26160,"ancestor_record_offset":25892,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26380,"parent_record_offset":26348,"ancestor_record_offset":25892,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26476,"parent_record_offset":26380,"ancestor_record_offset":25892,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":25892,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26160,"parent_record_offset":26124,"ancestor_record_offset":25892,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":505,"record_offset":26500,"parent_record_offset":26476,"ancestor_record_offset":25892,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":505,"record_offset":26780,"parent_record_offset":26748,"ancestor_record_offset":25892,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":505,"record_offset":27064,"parent_record_offset":26780,"ancestor_record_offset":25892,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":505,"record_offset":27036,"parent_record_offset":26780,"ancestor_record_offset":25892,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":505,"record_offset":26840,"parent_record_offset":26780,"ancestor_record_offset":25892,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":29584,"parent_record_offset":29560,"ancestor_record_offset":25892,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":505,"record_offset":29608,"parent_record_offset":29584,"ancestor_record_offset":25892,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::load_texture::load_compiled_texture_image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/load_texture.rs
//     evidence={"module":505,"record_offset":30204,"parent_record_offset":30168,"ancestor_record_offset":25892,"inlinee":28669,"source_line":19,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":29788,"parent_record_offset":29700,"ancestor_record_offset":25892,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":505,"record_offset":29700,"parent_record_offset":29676,"ancestor_record_offset":25892,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":505,"record_offset":29812,"parent_record_offset":29788,"ancestor_record_offset":25892,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::get_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":505,"record_offset":30320,"parent_record_offset":30292,"ancestor_record_offset":25892,"inlinee":13584,"source_line":228,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::MmapOptions::map
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":505,"record_offset":30292,"parent_record_offset":30268,"ancestor_record_offset":25892,"inlinee":13583,"source_line":338,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] memmap2::impl$2::get_len::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"memmap2","version":"0.5.8","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memmap2-0.5.8/src/lib.rs
//     evidence={"module":505,"record_offset":30368,"parent_record_offset":30344,"ancestor_record_offset":25892,"inlinee":13586,"source_line":229,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":27592,"parent_record_offset":27556,"ancestor_record_offset":25892,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":31616,"parent_record_offset":31584,"ancestor_record_offset":25892,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":32116,"parent_record_offset":31856,"ancestor_record_offset":25892,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":505,"record_offset":27736,"parent_record_offset":27708,"ancestor_record_offset":25892,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":27812,"parent_record_offset":27664,"ancestor_record_offset":25892,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":505,"record_offset":28072,"parent_record_offset":27812,"ancestor_record_offset":25892,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":505,"record_offset":30896,"parent_record_offset":30872,"ancestor_record_offset":25892,"inlinee":28664,"source_line":212,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":505,"record_offset":30920,"parent_record_offset":30896,"ancestor_record_offset":25892,"inlinee":28665,"source_line":200,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":505,"record_offset":30872,"parent_record_offset":30844,"ancestor_record_offset":25892,"inlinee":28662,"source_line":418,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$18::to_isize
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/rel_ptr/mod.rs
//     evidence={"module":505,"record_offset":30952,"parent_record_offset":30920,"ancestor_record_offset":25892,"inlinee":28667,"source_line":110,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.7.39","source":"git+https://github.com/PounceLight/rkyv.git?rev=031d58108ccf8c0e125b06b017f72b4afd67e9a3#031d58108ccf8c0e125b06b017f72b4afd67e9a3"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rkyv-79c0f5fde0331f7b/031d581/rkyv/src/vec/mod.rs
//     evidence={"module":505,"record_offset":30844,"parent_record_offset":30820,"ancestor_record_offset":25892,"inlinee":28660,"source_line":32,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::OsStr::to_os_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":28564,"parent_record_offset":28540,"ancestor_record_offset":25892,"inlinee":5171,"source_line":926,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":28644,"parent_record_offset":28620,"ancestor_record_offset":25892,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":28692,"parent_record_offset":28668,"ancestor_record_offset":25892,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":505,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":25892,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":505,"record_offset":29876,"parent_record_offset":29852,"ancestor_record_offset":25892,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":505,"record_offset":30132,"parent_record_offset":29852,"ancestor_record_offset":25892,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":505,"record_offset":31196,"parent_record_offset":31172,"ancestor_record_offset":25892,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":28620,"parent_record_offset":28596,"ancestor_record_offset":25892,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":29984,"parent_record_offset":29960,"ancestor_record_offset":25892,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":505,"record_offset":29960,"parent_record_offset":29936,"ancestor_record_offset":25892,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":505,"record_offset":29904,"parent_record_offset":29876,"ancestor_record_offset":25892,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":505,"record_offset":27764,"parent_record_offset":27736,"ancestor_record_offset":25892,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":505,"record_offset":27664,"parent_record_offset":27628,"ancestor_record_offset":25892,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":505,"record_offset":27628,"parent_record_offset":27592,"ancestor_record_offset":25892,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":505,"record_offset":30788,"parent_record_offset":30764,"ancestor_record_offset":25892,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00505.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::load_texture::load_compiled_texture_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FFD
// 首条位置: {"module":505,"inlinee":28669}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":505,"record_offset":30232,"parent_record_offset":30204,"ancestor_record_offset":30204,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00505.jsonl.gz"}
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
// 完整结构及原始类型记录: load_texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_compiled_texture_image
// 信息 ID: e60ef55abd273539460ea3c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::texture_watch::texture_watch @ 0xaab5c7 -> 0x917860 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$2::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0x12fe541 -> 0x917860 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$2::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > @ 0x1307ced -> 0x917860 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<tuple$<relative_path::RelativePathBuf,rhapsody::raw::texture::Texture> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$1::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0x130aa38 -> 0x917860 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<enum2$<core::option::Option<tuple$<relative_path::RelativePathBuf,rhapsody::raw::texture::Texture> > >,core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$1::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<enum2$<core::option::Option<tuple$<relative_path::RelativePathBuf,rhapsody::raw::texture::Texture> > >,alloc::vec::impl$20::extend_trusted::closure_env$0<enum2$<core::option::Option<tuple$<relative_path::RelativePathBuf,rhapsody::raw::texture::Texture> > >,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<relative_path::RelativePathBuf>,country_core::startup::load_textures::load_textures::closure$1::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > > > @ 0x14782c3 -> 0x917860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::load_texture::load_compiled_texture_image::closure$0
// 信息 ID: e8abe42010459ebaf89cf86e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::load_texture::load_compiled_texture_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::load_texture::load_compiled_texture_image; module=505, inline_record=30204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
