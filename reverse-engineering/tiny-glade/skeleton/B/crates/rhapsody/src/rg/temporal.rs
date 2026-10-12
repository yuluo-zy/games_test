// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/rg/temporal.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1311e93643100dbea4d96194c379e6507a5d121d50b983a237b94b62933d204e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: temporal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 102, 125, 194, 197, 200, 487, 518, 529, 536, 544, 692, 701, 1629
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use ustr; // namespace candidate
//   package identities: [{"name":"ustr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":8648,"rva":5204048,"va":5373913168}
// 编译实例 RVAs: 0x4f6850, 0x96ce90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4051381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e8b5d13d00","size":5,"soft_fail":false,"target_va":5382644528,"va":5378593142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$alloc..string..String$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2762637","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e88d272a00","size":5,"soft_fail":false,"target_va":5381355088,"va":5378592446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9639789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e893e86cff","size":5,"soft_fail":false,"target_va":5368953728,"va":5378593512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c8f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2284869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913168,"raw_hex":"e8bb22ddff","size":5,"soft_fail":false,"target_va":5371629392,"va":5373914256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140311cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1986267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913168,"raw_hex":"e825b1e1ff","size":5,"soft_fail":false,"target_va":5371927744,"va":5373914006,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t765330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913168,"raw_hex":"e892ad0b00","size":5,"soft_fail":false,"target_va":5374678592,"va":5373913257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ba5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t801380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913168,"raw_hex":"e8643a0c00","size":5,"soft_fail":false,"target_va":5374715376,"va":5373913991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3665578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e85611c8ff","size":5,"soft_fail":false,"target_va":5374928352,"va":5378593925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3076933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e8bb0cd1ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378592960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd60a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2527336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e868902600","size":5,"soft_fail":false,"target_va":5381120160,"va":5378592819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2576819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e8b3512700","size":5,"soft_fail":false,"target_va":5381170352,"va":5378593528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e8120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9941554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e832b29700","size":5,"soft_fail":false,"target_va":5388534048,"va":5378592489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12310875","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e85bd9bb00","size":5,"soft_fail":false,"target_va":5390904368,"va":5378593488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a53f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12812821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e81582c300","size":5,"soft_fail":false,"target_va":5391406064,"va":5378593238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a71f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13869092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e824a0d300","size":5,"soft_fail":false,"target_va":5392462320,"va":5378593223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37404057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373913168,"raw_hex":"e899bd3a02","size":5,"soft_fail":false,"target_va":5411318560,"va":5373914498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32747545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e819b0f301","size":5,"soft_fail":false,"target_va":5411341280,"va":5378593730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a998a
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32753032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e888c5f301","size":5,"soft_fail":false,"target_va":5411346826,"va":5378593789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4246481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378592400,"raw_hex":"e8d1cb4000","size":5,"soft_fail":false,"target_va":5382840208,"va":5378593722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":11012,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":200,"record_offset":10012,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":10388,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":200,"record_offset":8932,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":10828,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":10652,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":10684,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":200,"record_offset":10420,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":10304,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":10068,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":14686,"source_line":67,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":200,"record_offset":10248,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":200,"record_offset":10276,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":9698,"source_line":327,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":8960,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":10800,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":10596,"parent_record_offset":8648,"ancestor_record_offset":8648,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":8648,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":200,"record_offset":10132,"parent_record_offset":10068,"ancestor_record_offset":8648,"inlinee":14690,"source_line":183,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":200,"record_offset":10160,"parent_record_offset":10068,"ancestor_record_offset":8648,"inlinee":14692,"source_line":206,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":200,"record_offset":10188,"parent_record_offset":10068,"ancestor_record_offset":8648,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":200,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":8648,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":9740,"parent_record_offset":9716,"ancestor_record_offset":8648,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":8648,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":8648,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":9272,"parent_record_offset":9248,"ancestor_record_offset":8648,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":9836,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":9480,"parent_record_offset":9452,"ancestor_record_offset":8648,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":8648,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":200,"record_offset":11036,"parent_record_offset":11012,"ancestor_record_offset":8648,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":200,"record_offset":10476,"parent_record_offset":10444,"ancestor_record_offset":8648,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":200,"record_offset":10504,"parent_record_offset":10444,"ancestor_record_offset":8648,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":10856,"parent_record_offset":10828,"ancestor_record_offset":8648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":200,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":8648,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":200,"record_offset":9788,"parent_record_offset":9764,"ancestor_record_offset":8648,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":10328,"parent_record_offset":10304,"ancestor_record_offset":8648,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":10352,"parent_record_offset":10328,"ancestor_record_offset":8648,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":9392,"parent_record_offset":9364,"ancestor_record_offset":8648,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":9424,"parent_record_offset":9392,"ancestor_record_offset":8648,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":9364,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":9224,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":9864,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":9308,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":200,"record_offset":9196,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":200,"record_offset":9716,"parent_record_offset":9692,"ancestor_record_offset":8648,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":8648,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":8984,"parent_record_offset":8960,"ancestor_record_offset":8648,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":9692,"parent_record_offset":9524,"ancestor_record_offset":8648,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":9040,"parent_record_offset":9008,"ancestor_record_offset":8648,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9632,"parent_record_offset":9548,"ancestor_record_offset":8648,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9576,"parent_record_offset":9548,"ancestor_record_offset":8648,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9956,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9548,"parent_record_offset":9524,"ancestor_record_offset":8648,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8648,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9068,"parent_record_offset":9008,"ancestor_record_offset":8648,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9116,"parent_record_offset":9092,"ancestor_record_offset":8648,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9168,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":9524,"parent_record_offset":9116,"ancestor_record_offset":8648,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":10532,"parent_record_offset":10444,"ancestor_record_offset":8648,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":200,"record_offset":10444,"parent_record_offset":10420,"ancestor_record_offset":8648,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":10556,"parent_record_offset":10532,"ancestor_record_offset":8648,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":10104,"parent_record_offset":10068,"ancestor_record_offset":8648,"inlinee":14688,"source_line":38,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":10216,"parent_record_offset":10068,"ancestor_record_offset":8648,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":10768,"parent_record_offset":10740,"ancestor_record_offset":8648,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":8648,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":9764,"parent_record_offset":9740,"ancestor_record_offset":8648,"inlinee":11984,"source_line":166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":11268,"rva":5205584,"va":5373914704}
// 编译实例 RVAs: 0x4f6e50, 0x96c890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4052917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e8b5d73d00","size":5,"soft_fail":false,"target_va":5382644528,"va":5378591606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2764150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e8762d2a00","size":5,"soft_fail":false,"target_va":5381355072,"va":5378590917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9638253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e893ee6cff","size":5,"soft_fail":false,"target_va":5368953728,"va":5378591976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c8f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2286405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373914704,"raw_hex":"e8bb1cddff","size":5,"soft_fail":false,"target_va":5371629392,"va":5373915792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140311cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1987803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373914704,"raw_hex":"e825abe1ff","size":5,"soft_fail":false,"target_va":5371927744,"va":5373915542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t763787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373914704,"raw_hex":"e88ba70b00","size":5,"soft_fail":false,"target_va":5374678592,"va":5373914800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ba5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t799844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373914704,"raw_hex":"e864340c00","size":5,"soft_fail":false,"target_va":5374715376,"va":5373915527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3664042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e85617c8ff","size":5,"soft_fail":false,"target_va":5374928352,"va":5378592389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3075397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e8bb12d1ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5378591424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd60a0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2528872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e868962600","size":5,"soft_fail":false,"target_va":5381120160,"va":5378591283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2578355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e8b3572700","size":5,"soft_fail":false,"target_va":5381170352,"va":5378591992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e8120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9943083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e82bb89700","size":5,"soft_fail":false,"target_va":5388534048,"va":5378590960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12312411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e85bdfbb00","size":5,"soft_fail":false,"target_va":5390904368,"va":5378591952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a53f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12814357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e81588c300","size":5,"soft_fail":false,"target_va":5391406064,"va":5378591702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a71f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13870628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e824a6d300","size":5,"soft_fail":false,"target_va":5392462320,"va":5378591687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37402518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373914704,"raw_hex":"e896b73a02","size":5,"soft_fail":false,"target_va":5411318560,"va":5373916037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32749078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e816b6f301","size":5,"soft_fail":false,"target_va":5411341280,"va":5378592197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a998a
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32754565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e885cbf301","size":5,"soft_fail":false,"target_va":5411346826,"va":5378592256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4248014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378590864,"raw_hex":"e8ced14000","size":5,"soft_fail":false,"target_va":5382840208,"va":5378592189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":13624,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":200,"record_offset":12624,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":13000,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":200,"record_offset":11544,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":13440,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":13264,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":13296,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":200,"record_offset":13032,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":12916,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$31::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":12680,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":14686,"source_line":67,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":200,"record_offset":12860,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":200,"record_offset":12888,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":9698,"source_line":327,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":11572,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":13412,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] ustr::Ustr::as_str
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":13208,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":8808,"source_line":239,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":11268,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$346::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":200,"record_offset":12744,"parent_record_offset":12680,"ancestor_record_offset":11268,"inlinee":14690,"source_line":183,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$363::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":200,"record_offset":12772,"parent_record_offset":12680,"ancestor_record_offset":11268,"inlinee":14692,"source_line":206,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":200,"record_offset":12800,"parent_record_offset":12680,"ancestor_record_offset":11268,"inlinee":8804,"source_line":649,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":200,"record_offset":12648,"parent_record_offset":12624,"ancestor_record_offset":11268,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":12352,"parent_record_offset":12328,"ancestor_record_offset":11268,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":11268,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":11268,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":11268,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":12448,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":12092,"parent_record_offset":12064,"ancestor_record_offset":11268,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":12064,"parent_record_offset":12036,"ancestor_record_offset":11268,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":200,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":11268,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":200,"record_offset":13088,"parent_record_offset":13056,"ancestor_record_offset":11268,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":200,"record_offset":13116,"parent_record_offset":13056,"ancestor_record_offset":11268,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":13468,"parent_record_offset":13440,"ancestor_record_offset":11268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":200,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":11268,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":200,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":11268,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":11268,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":200,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":11268,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":12004,"parent_record_offset":11976,"ancestor_record_offset":11268,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":12036,"parent_record_offset":12004,"ancestor_record_offset":11268,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":11976,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":11836,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":12476,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":11920,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":200,"record_offset":11808,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":200,"record_offset":12328,"parent_record_offset":12304,"ancestor_record_offset":11268,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":11268,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":11596,"parent_record_offset":11572,"ancestor_record_offset":11268,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":12304,"parent_record_offset":12136,"ancestor_record_offset":11268,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":11652,"parent_record_offset":11620,"ancestor_record_offset":11268,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":12244,"parent_record_offset":12160,"ancestor_record_offset":11268,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":12188,"parent_record_offset":12160,"ancestor_record_offset":11268,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":12568,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":12160,"parent_record_offset":12136,"ancestor_record_offset":11268,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":11268,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":11680,"parent_record_offset":11620,"ancestor_record_offset":11268,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":11728,"parent_record_offset":11704,"ancestor_record_offset":11268,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":11780,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":12136,"parent_record_offset":11728,"ancestor_record_offset":11268,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":13144,"parent_record_offset":13056,"ancestor_record_offset":11268,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":200,"record_offset":13056,"parent_record_offset":13032,"ancestor_record_offset":11268,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":200,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":11268,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$17::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":12716,"parent_record_offset":12680,"ancestor_record_offset":11268,"inlinee":14688,"source_line":38,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":12828,"parent_record_offset":12680,"ancestor_record_offset":11268,"inlinee":12549,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":13380,"parent_record_offset":13352,"ancestor_record_offset":11268,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ustr::Ustr::len
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":11268,"inlinee":8809,"source_line":308,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":12376,"parent_record_offset":12352,"ancestor_record_offset":11268,"inlinee":11984,"source_line":166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":5620,"rva":5202640,"va":5373911760}
// 编译实例 RVAs: 0x4f62d0, 0x96c310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7443257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e839937100","size":5,"soft_fail":false,"target_va":5381355072,"va":5373911810,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=2
//     target_aliases=532 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4958905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e84755b4ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5373912628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c8f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2283220","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e82c29ddff","size":5,"soft_fail":false,"target_va":5371629392,"va":5373912607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140311960
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1985437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e863b4e1ff","size":5,"soft_fail":false,"target_va":5371926880,"va":5373912312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t766734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e80eb30b00","size":5,"soft_fail":false,"target_va":5374678592,"va":5373911853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b93b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t798406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e8c62e0c00","size":5,"soft_fail":false,"target_va":5374710704,"va":5373912293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1169134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e8eed61100","size":5,"soft_fail":false,"target_va":5375081504,"va":5373912365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c40
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1169328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e8b0d71100","size":5,"soft_fail":false,"target_va":5375081536,"va":5373912203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4fc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7203775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e8bfeb6d00","size":5,"soft_fail":false,"target_va":5381115840,"va":5373912060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7257703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e867be6e00","size":5,"soft_fail":false,"target_va":5381170352,"va":5373912644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412e8120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9944494","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378589456,"raw_hex":"e8aebd9700","size":5,"soft_fail":false,"target_va":5388534048,"va":5378589549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14152ac30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12314060","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378589456,"raw_hex":"e8cce5bb00","size":5,"soft_fail":false,"target_va":5390904368,"va":5378590303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415a5210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12815571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378589456,"raw_hex":"e8d38cc300","size":5,"soft_fail":false,"target_va":5391405584,"va":5378590008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a2dc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13854870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378589456,"raw_hex":"e89668d300","size":5,"soft_fail":false,"target_va":5392444864,"va":5378589989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2b20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37405710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e80ec43a02","size":5,"soft_fail":false,"target_va":5411318560,"va":5373912845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32750734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378589456,"raw_hex":"e88ebcf301","size":5,"soft_fail":false,"target_va":5411341280,"va":5378590541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8927366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e886388800","size":5,"soft_fail":false,"target_va":5382840208,"va":5373912837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37628065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373911760,"raw_hex":"e8a1283e02","size":5,"soft_fail":false,"target_va":5411540963,"va":5373912893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":8108,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":200,"record_offset":7012,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4759,"source_line":3186,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":200,"record_offset":5876,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":7440,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":7376,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":7412,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":200,"record_offset":6956,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":200,"record_offset":6984,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":9698,"source_line":327,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":5904,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":7896,"parent_record_offset":5620,"ancestor_record_offset":5620,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":5620,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":7320,"parent_record_offset":7296,"ancestor_record_offset":5620,"inlinee":4772,"source_line":173,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":5620,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate_zeroed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":7272,"parent_record_offset":7164,"ancestor_record_offset":5620,"inlinee":4771,"source_line":259,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":200,"record_offset":7568,"parent_record_offset":7512,"ancestor_record_offset":5620,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":8156,"parent_record_offset":8132,"ancestor_record_offset":5620,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7084,"parent_record_offset":7048,"ancestor_record_offset":5620,"inlinee":4769,"source_line":206,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7540,"parent_record_offset":7512,"ancestor_record_offset":5620,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7512,"parent_record_offset":7488,"ancestor_record_offset":5620,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7164,"parent_record_offset":7120,"ancestor_record_offset":5620,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_zeroed_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7120,"parent_record_offset":7084,"ancestor_record_offset":5620,"inlinee":4770,"source_line":442,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":5620,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":200,"record_offset":7216,"parent_record_offset":7164,"ancestor_record_offset":5620,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$3::from_elem
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":200,"record_offset":7048,"parent_record_offset":7012,"ancestor_record_offset":5620,"inlinee":5369,"source_line":53,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":200,"record_offset":7240,"parent_record_offset":7216,"ancestor_record_offset":5620,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":200,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5620,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":5620,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":6276,"parent_record_offset":6252,"ancestor_record_offset":5620,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":5620,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":200,"record_offset":6780,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":6424,"parent_record_offset":6396,"ancestor_record_offset":5620,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":6396,"parent_record_offset":6368,"ancestor_record_offset":5620,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":200,"record_offset":8132,"parent_record_offset":8108,"ancestor_record_offset":5620,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":200,"record_offset":7464,"parent_record_offset":7440,"ancestor_record_offset":5620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":200,"record_offset":6544,"parent_record_offset":6520,"ancestor_record_offset":5620,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":200,"record_offset":6732,"parent_record_offset":6708,"ancestor_record_offset":5620,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":6336,"parent_record_offset":6308,"ancestor_record_offset":5620,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":6368,"parent_record_offset":6336,"ancestor_record_offset":5620,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":6308,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":6168,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":6808,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":6252,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":200,"record_offset":6140,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":200,"record_offset":6660,"parent_record_offset":6636,"ancestor_record_offset":5620,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":5952,"parent_record_offset":5928,"ancestor_record_offset":5620,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":5620,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":6636,"parent_record_offset":6468,"ancestor_record_offset":5620,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":5984,"parent_record_offset":5952,"ancestor_record_offset":5620,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6576,"parent_record_offset":6492,"ancestor_record_offset":5620,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6520,"parent_record_offset":6492,"ancestor_record_offset":5620,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6900,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":5620,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6036,"parent_record_offset":6012,"ancestor_record_offset":5620,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6012,"parent_record_offset":5952,"ancestor_record_offset":5620,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":5620,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6112,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":6468,"parent_record_offset":6060,"ancestor_record_offset":5620,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":200,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":5620,"inlinee":11984,"source_line":166,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$14::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73C6
// 首条位置: {"module":518,"inlinee":29638}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$3::from
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 42
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2D6C
// 首条位置: {"module":102,"inlinee":11628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Parity::flip
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A7A
// 首条位置: {"module":701,"inlinee":39546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$18::default
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x38CC
// 首条位置: {"module":194,"inlinee":14540}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":89668,"parent_record_offset":88076,"ancestor_record_offset":88076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88116,"parent_record_offset":88076,"ancestor_record_offset":88076,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88684,"parent_record_offset":88652,"ancestor_record_offset":88076,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":194,"record_offset":88752,"parent_record_offset":88716,"ancestor_record_offset":88076,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":194,"record_offset":88532,"parent_record_offset":88504,"ancestor_record_offset":88076,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":194,"record_offset":88716,"parent_record_offset":88652,"ancestor_record_offset":88076,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":194,"record_offset":88264,"parent_record_offset":88228,"ancestor_record_offset":88076,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":194,"record_offset":88384,"parent_record_offset":88356,"ancestor_record_offset":88076,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":89692,"parent_record_offset":89668,"ancestor_record_offset":88076,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":194,"record_offset":88476,"parent_record_offset":88356,"ancestor_record_offset":88076,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88560,"parent_record_offset":88532,"ancestor_record_offset":88076,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88504,"parent_record_offset":88476,"ancestor_record_offset":88076,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":194,"record_offset":88592,"parent_record_offset":88532,"ancestor_record_offset":88076,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":194,"record_offset":89740,"parent_record_offset":89716,"ancestor_record_offset":88076,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":194,"record_offset":88872,"parent_record_offset":88116,"ancestor_record_offset":88076,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88172,"parent_record_offset":88144,"ancestor_record_offset":88076,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88436,"parent_record_offset":88412,"ancestor_record_offset":88076,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":194,"record_offset":88292,"parent_record_offset":88264,"ancestor_record_offset":88076,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88652,"parent_record_offset":88228,"ancestor_record_offset":88076,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":194,"record_offset":88144,"parent_record_offset":88116,"ancestor_record_offset":88076,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88324,"parent_record_offset":88292,"ancestor_record_offset":88076,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":194,"record_offset":88356,"parent_record_offset":88324,"ancestor_record_offset":88076,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88228,"parent_record_offset":88200,"ancestor_record_offset":88076,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":194,"record_offset":88200,"parent_record_offset":88172,"ancestor_record_offset":88076,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00194.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":701,"record_offset":256,"rva":12504784,"va":5381213904}
// 编译实例 RVAs: 0xbeced0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b1640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6535357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381213904,"raw_hex":"e843479cff","size":5,"soft_fail":false,"target_va":5374678592,"va":5381213944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418addf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13373006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381213904,"raw_hex":"e84e0ecc00","size":5,"soft_fail":false,"target_va":5394587120,"va":5381214109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Parity::flip
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":701,"record_offset":1460,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":39546,"source_line":73,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":701,"record_offset":420,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":701,"record_offset":1432,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":4947,"source_line":1189,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":701,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":256,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":701,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":256,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":701,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":256,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":701,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":256,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":701,"record_offset":1256,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":701,"record_offset":964,"parent_record_offset":936,"ancestor_record_offset":256,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":701,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":256,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":701,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":256,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":701,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":256,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":701,"record_offset":876,"parent_record_offset":848,"ancestor_record_offset":256,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":701,"record_offset":908,"parent_record_offset":876,"ancestor_record_offset":256,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":701,"record_offset":848,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":701,"record_offset":708,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":701,"record_offset":1284,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":701,"record_offset":792,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":701,"record_offset":680,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":701,"record_offset":1136,"parent_record_offset":1112,"ancestor_record_offset":256,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":701,"record_offset":476,"parent_record_offset":448,"ancestor_record_offset":256,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":701,"record_offset":448,"parent_record_offset":420,"ancestor_record_offset":256,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":701,"record_offset":1112,"parent_record_offset":1008,"ancestor_record_offset":256,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":701,"record_offset":512,"parent_record_offset":476,"ancestor_record_offset":256,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":1052,"parent_record_offset":1028,"ancestor_record_offset":256,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":1028,"parent_record_offset":1008,"ancestor_record_offset":256,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":1376,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":568,"parent_record_offset":540,"ancestor_record_offset":256,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":540,"parent_record_offset":476,"ancestor_record_offset":256,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":600,"parent_record_offset":568,"ancestor_record_offset":256,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":652,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":701,"record_offset":1008,"parent_record_offset":600,"ancestor_record_offset":256,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   [pdb_inline_transitive] ustr::impl$21::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ustr","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ustr-1.0.0/src/lib.rs
//     evidence={"module":701,"record_offset":1184,"parent_record_offset":1160,"ancestor_record_offset":256,"inlinee":11984,"source_line":166,"archive":"_pdb/B/modules/00701.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::into_inner
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x69B9
// 首条位置: {"module":487,"inlinee":27065}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::as_ref
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x391C
// 首条位置: {"module":197,"inlinee":14620}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::Temporal::split_pair
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 158
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x391B
// 首条位置: {"module":197,"inlinee":14619}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3096
// 首条位置: {"module":125,"inlinee":12438}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":20048,"parent_record_offset":20028,"ancestor_record_offset":20028,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":20028,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":125,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":20028,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 297
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3093
// 首条位置: {"module":125,"inlinee":12435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":18604,"parent_record_offset":18584,"ancestor_record_offset":18584,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":125,"record_offset":18652,"parent_record_offset":18628,"ancestor_record_offset":18584,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":125,"record_offset":18628,"parent_record_offset":18604,"ancestor_record_offset":18584,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00125.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 318
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":20212,"rva":5209248,"va":5373918368}
// 编译实例 RVAs: 0x4f7ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405daf00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t929778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373918368,"raw_hex":"e8f22f0e00","size":5,"soft_fail":false,"target_va":5374848768,"va":5373918985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc3d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1066187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373918368,"raw_hex":"e8cb441000","size":5,"soft_fail":false,"target_va":5374985168,"va":5373918976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd5120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7197389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373918368,"raw_hex":"e8cdd26d00","size":5,"soft_fail":false,"target_va":5381116192,"va":5373918798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6380
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7202288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373918368,"raw_hex":"e8f0e56d00","size":5,"soft_fail":false,"target_va":5381120896,"va":5373918603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: temporal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37928737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373918368,"raw_hex":"e821bf4202","size":5,"soft_fail":false,"target_va":5411847168,"va":5373918426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":200,"record_offset":21588,"parent_record_offset":20212,"ancestor_record_offset":20212,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::clone_unchecked
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":200,"record_offset":21524,"parent_record_offset":20212,"ancestor_record_offset":20212,"inlinee":9698,"source_line":327,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":20444,"parent_record_offset":20212,"ancestor_record_offset":20212,"inlinee":13598,"source_line":473,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$64::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":20820,"parent_record_offset":20212,"ancestor_record_offset":20212,"inlinee":14696,"source_line":2586,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":20700,"parent_record_offset":20672,"ancestor_record_offset":20212,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":200,"record_offset":20760,"parent_record_offset":20728,"ancestor_record_offset":20212,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":21128,"parent_record_offset":21096,"ancestor_record_offset":20212,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":200,"record_offset":21096,"parent_record_offset":21064,"ancestor_record_offset":20212,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":200,"record_offset":20644,"parent_record_offset":20616,"ancestor_record_offset":20212,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":200,"record_offset":21388,"parent_record_offset":21364,"ancestor_record_offset":20212,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":21024,"parent_record_offset":20984,"ancestor_record_offset":20212,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":21064,"parent_record_offset":21024,"ancestor_record_offset":20212,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":21172,"parent_record_offset":20984,"ancestor_record_offset":20212,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":200,"record_offset":20984,"parent_record_offset":20936,"ancestor_record_offset":20212,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":20672,"parent_record_offset":20616,"ancestor_record_offset":20212,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":20728,"parent_record_offset":20672,"ancestor_record_offset":20212,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":200,"record_offset":21264,"parent_record_offset":20936,"ancestor_record_offset":20212,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":20500,"parent_record_offset":20472,"ancestor_record_offset":20212,"inlinee":13574,"source_line":799,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":200,"record_offset":20876,"parent_record_offset":20848,"ancestor_record_offset":20212,"inlinee":11990,"source_line":3219,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":21364,"parent_record_offset":20936,"ancestor_record_offset":20212,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20616,"parent_record_offset":20556,"ancestor_record_offset":20212,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20936,"parent_record_offset":20904,"ancestor_record_offset":20212,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20528,"parent_record_offset":20500,"ancestor_record_offset":20212,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20588,"parent_record_offset":20556,"ancestor_record_offset":20212,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20556,"parent_record_offset":20528,"ancestor_record_offset":20212,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":200,"record_offset":20904,"parent_record_offset":20876,"ancestor_record_offset":20212,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":200,"record_offset":21556,"parent_record_offset":21524,"ancestor_record_offset":20212,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":20472,"parent_record_offset":20444,"ancestor_record_offset":20212,"inlinee":13573,"source_line":570,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$50::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":200,"record_offset":20848,"parent_record_offset":20820,"ancestor_record_offset":20212,"inlinee":14698,"source_line":2428,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 367
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":200,"record_offset":13880,"rva":5207120,"va":5373916240}
// 编译实例 RVAs: 0x4f7450, 0x96d490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$ustr..Ustr$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2761092","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378593936,"raw_hex":"e884212a00","size":5,"soft_fail":false,"target_va":5381355072,"va":5378593975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t4246131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378593936,"raw_hex":"e873ca4000","size":5,"soft_fail":false,"target_va":5382840208,"va":5378594072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2619539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378593936,"raw_hex":"e893f82700","size":5,"soft_fail":false,"target_va":5381213904,"va":5378594360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378593936,"raw_hex":"e84df9ffff","size":5,"soft_fail":false,"target_va":5378592400,"va":5378594110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":14144,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":200,"record_offset":14116,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":14224,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":200,"record_offset":14256,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":200,"record_offset":14192,"parent_record_offset":14168,"ancestor_record_offset":13880,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":200,"record_offset":14168,"parent_record_offset":14144,"ancestor_record_offset":13880,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00200.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::temporal::impl$3::from
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":200,"record_offset":14452,"parent_record_offset":14428,"ancestor_record_offset":13880,"inlinee":11628,"source_line":42,"archive":"_pdb/B/modules/00200.jsonl.gz"}
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
// 完整结构及原始类型记录: temporal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::Parity
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>", "constructor": "tuple$", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::Parity>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<ustr::Ustr,rhapsody::rg::temporal::Parity,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::Parity", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::ReadOnlyHandle
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture>", "constructor": "rhapsody::rg::temporal::ReadOnlyHandle", "arguments": ["rhapsody::raw::texture::Texture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >", "constructor": "ref$", "arguments": ["rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >", "constructor": "country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder", "arguments": ["ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::Temporal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalRenderGraphRecorder
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::temporal::TemporalRenderGraphRecorder", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphRecorder<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalRenderGraphState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalRenderGraphState>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState>", "constructor": "core::option::Option", "arguments": ["rhapsody::rg::temporal::TemporalRenderGraphState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::option::Option<rhapsody::rg::temporal::TemporalRenderGraphState> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::temporal::TemporalResourceValue
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "constructor": "tuple$", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::temporal::TemporalResourceValue>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::temporal::TemporalResourceValue"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ustr::Ustr", "rhapsody::rg::temporal::TemporalResourceValue", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ustr::Ustr,rhapsody::rg::temporal::TemporalResourceValue,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: temporal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "rhapsody::rg::temporal::TemporalResourceValue", "status": "evidence_association; original field ownership unknown"}, {"kind": "exact_PDB_named_entity_mention", "name": "rhapsody::rg::temporal::ReadOnlyHandle", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: temporal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Parity::flip
// 信息 ID: b3ab9da620f31c2175ebc901
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity; module=701, inline_record=1460; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::as_ref
// 信息 ID: fd0b728cb954bb35ee5f7902
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=197, inline_record=26056; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=197, inline_record=26240; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=197, inline_record=26268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::into_inner
// 信息 ID: 5e5d3077356f69a2ff7563f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::painterly::painterly_pass; module=487, inline_record=53320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=529, inline_record=43240; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=108112; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi; module=536, inline_record=108140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::calculate_tgsr; module=544, inline_record=83588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::Temporal::split_pair
// 信息 ID: e6d9482461a95ef4a29625ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render_world; module=197, inline_record=26024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::calculate_tgsr; module=544, inline_record=82088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 8472966f71cb76d6052fccae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::temporal::TemporalRenderGraphRecorder::export_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1637e7 -> 0x4f7ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x165516 -> 0x4f7ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166cb1 -> 0x4f7ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::TemporalRenderGraphState::get_and_flip_parity
// 信息 ID: e742f3b68a5f9a2587c44d7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f75f8 -> 0xbeced0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96d638 -> 0xbeced0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure$0
// 信息 ID: 46dcb810a37782f064d6405f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::temporal::impl$11::get_or_create_temporal", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=125, inline_record=20028; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::texture::Texture,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$11::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=1629, inline_record=110424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,alloc::string::String>
// 信息 ID: 9c0ec24a815755c058273aaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::temporal::impl$11::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "alloc::string::String"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f74fe -> 0x4f6850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f75a2 -> 0x4f6850 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96d53e -> 0x96ce90 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96d5e2 -> 0x96ce90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$11::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: befa0572ec0c9c1b807a3b1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$11::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec260 -> 0x4f6e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a674d -> 0x96c890 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a67e7 -> 0x96c890 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_pass @ 0x8a84b1 -> 0x96c890 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring @ 0x98fcf7 -> 0x96c890 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring @ 0x98fdca -> 0x96c890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure$0
// 信息 ID: 7a444b14609ccaf8b1301f14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::temporal::impl$12::get_or_create_temporal", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0 <- anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=125, inline_record=18584; 不是独立调用边。
// [PDB 内联宿主] anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >; module=1629, inline_record=109640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: a0729b8f6ebf965d65061cbe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$12::get_or_create_temporal -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0xfb99f -> 0x4f62d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a6f8c -> 0x96c310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a8f84 -> 0x96c310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9287da -> 0x96c310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset> @ 0xa3a9f7 -> 0x96c310 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0xa602f1 -> 0x96c310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$14::fmt
// 信息 ID: 989d83c166f072a13dfd66bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<ref$<rhapsody::rg::temporal::ReadOnlyHandle<rhapsody::raw::texture::Texture> > >; module=518, inline_record=11384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$18::default
// 信息 ID: 20f4d9e8a29a0cee1508cb75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=88076; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::game_loop::GameLoop::main_loop; module=194, inline_record=90512; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=11868; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=14140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::impl$0::default; module=692, inline_record=408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >
// 信息 ID: 8a3dc91d54041267dc207785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair -> ["country_core::render::pass_builder::pass_resources::PassResources", "ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 13 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render_world @ 0x4ec84d -> 0x4f7450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi @ 0x874c64 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0d80 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c1f46 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c2040 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0xa025a0 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0xa0267b -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi @ 0xa0275e -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows @ 0xa39efe -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::taa::calculate_tgsr @ 0xa3bb8a -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0xa452fe -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance @ 0xa453d9 -> 0x96d490 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights @ 0xa70038 -> 0x96d490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::temporal::impl$3::from
// 信息 ID: 22519b53e5a8b3dd074cf215
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>; module=102, inline_record=6068; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>; module=102, inline_record=6740; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>; module=102, inline_record=7412; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo>; module=102, inline_record=8076; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo>; module=102, inline_record=8744; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo>; module=102, inline_record=9416; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo>; module=102, inline_record=10080; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::temporal::impl$20::get_or_create_temporal_pair<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >; module=200, inline_record=14452; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
