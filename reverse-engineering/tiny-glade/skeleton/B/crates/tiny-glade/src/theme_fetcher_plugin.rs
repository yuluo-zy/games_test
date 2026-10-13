// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/theme_fetcher_plugin.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: dfd0382834db89cd3b2d390622cf617dc150cd53db911be681a409176aa94f92
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: theme_fetcher_plugin.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 170
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_app; // namespace candidate
//   package identities: [{"name":"bevy_app","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use paths; // namespace candidate
//   package identities: [{"name":"paths","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":20152,"rva":4001648,"va":5372710768}
// 编译实例 RVAs: 0x3d0f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9933127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e847919700","size":5,"soft_fail":false,"target_va":5382644528,"va":5372711396,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140327210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-696705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e87f5ef5ff","size":5,"soft_fail":false,"target_va":5372015120,"va":5372711820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14054aa00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1546534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e826991700","size":5,"soft_fail":false,"target_va":5374257664,"va":5372711125,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d9ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2130916","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8e4832000","size":5,"soft_fail":false,"target_va":5374843616,"va":5372712695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405da700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2134038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e816902000","size":5,"soft_fail":false,"target_va":5374846720,"va":5372712677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e1260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2162135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8d7fd2000","size":5,"soft_fail":false,"target_va":5374874208,"va":5372712068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2187984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8d0622100","size":5,"soft_fail":false,"target_va":5374900736,"va":5372712747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405eabe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2201230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e88e962100","size":5,"soft_fail":false,"target_va":5374913504,"va":5372712269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2246150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e806462200","size":5,"soft_fail":false,"target_va":5374958624,"va":5372712469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2370536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8e82b2400","size":5,"soft_fail":false,"target_va":5375081488,"va":5372710947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2369331","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e833272400","size":5,"soft_fail":false,"target_va":5375081504,"va":5372712168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2804816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e850cc2a00","size":5,"soft_fail":false,"target_va":5375516032,"va":5372711211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14083c480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4632899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e843b14600","size":5,"soft_fail":false,"target_va":5377344640,"va":5372711736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t39136183","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8b72b5502","size":5,"soft_fail":false,"target_va":5411847168,"va":5372710980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38829055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8ff7b5002","size":5,"soft_fail":false,"target_va":5411540992,"va":5372711932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38829002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8ca7b5002","size":5,"soft_fail":false,"target_va":5411540963,"va":5372711956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9971620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8a4279800","size":5,"soft_fail":false,"target_va":5382683488,"va":5372711863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9943586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e822ba9700","size":5,"soft_fail":false,"target_va":5382655024,"va":5372711433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2732355","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8bd4ed6ff","size":5,"soft_fail":false,"target_va":5369979408,"va":5372711758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38830860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e80c835002","size":5,"soft_fail":false,"target_va":5411542880,"va":5372712015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] paths::standard_app_root_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"paths","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8178640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8d0cb7c00","size":5,"soft_fail":false,"target_va":5380889680,"va":5372711035,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10032992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e860179900","size":5,"soft_fail":false,"target_va":5382744080,"va":5372711083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::name
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9979433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e829469800","size":5,"soft_fail":false,"target_va":5382691056,"va":5372711618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::Builder::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1334624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372710768,"raw_hex":"e8a0a2ebff","size":5,"soft_fail":false,"target_va":5371377056,"va":5372711675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*47976518(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5372710768,"raw_hex":"ff154610dc02","size":6,"soft_fail":false,"va":5372711844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_app","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_app-0.16.0/src/app.rs
//     evidence={"module":170,"record_offset":21692,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":9225,"source_line":301,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":170,"record_offset":20748,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":170,"record_offset":21012,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":20956,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":21648,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":20780,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":20664,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":20636,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":170,"record_offset":20320,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":7841,"source_line":519,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::thread::Builder::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":170,"record_offset":20984,"parent_record_offset":20152,"ancestor_record_offset":20152,"inlinee":12559,"source_line":289,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":20524,"parent_record_offset":20500,"ancestor_record_offset":20152,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":20152,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":22716,"parent_record_offset":22692,"ancestor_record_offset":20152,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":20468,"parent_record_offset":20428,"ancestor_record_offset":20152,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":20500,"parent_record_offset":20468,"ancestor_record_offset":20152,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":22692,"parent_record_offset":22664,"ancestor_record_offset":20152,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":170,"record_offset":20428,"parent_record_offset":20392,"ancestor_record_offset":20152,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":21352,"parent_record_offset":21316,"ancestor_record_offset":20152,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":22664,"parent_record_offset":22640,"ancestor_record_offset":20152,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":21428,"parent_record_offset":21388,"ancestor_record_offset":20152,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":21388,"parent_record_offset":21352,"ancestor_record_offset":20152,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":22640,"parent_record_offset":22616,"ancestor_record_offset":20152,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":21276,"parent_record_offset":21240,"ancestor_record_offset":20152,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":21240,"parent_record_offset":21204,"ancestor_record_offset":20152,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":21168,"parent_record_offset":21120,"ancestor_record_offset":20152,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":21204,"parent_record_offset":21168,"ancestor_record_offset":20152,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":170,"record_offset":21120,"parent_record_offset":21084,"ancestor_record_offset":20152,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":170,"record_offset":21084,"parent_record_offset":21048,"ancestor_record_offset":20152,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":21048,"parent_record_offset":21012,"ancestor_record_offset":20152,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":170,"record_offset":22188,"parent_record_offset":22152,"ancestor_record_offset":20152,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":170,"record_offset":21988,"parent_record_offset":21964,"ancestor_record_offset":20152,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":21316,"parent_record_offset":21276,"ancestor_record_offset":20152,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":170,"record_offset":21584,"parent_record_offset":21560,"ancestor_record_offset":20152,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":170,"record_offset":20836,"parent_record_offset":20804,"ancestor_record_offset":20152,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":170,"record_offset":20864,"parent_record_offset":20804,"ancestor_record_offset":20152,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":170,"record_offset":21560,"parent_record_offset":21276,"ancestor_record_offset":20152,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":21748,"parent_record_offset":21720,"ancestor_record_offset":20152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":170,"record_offset":22220,"parent_record_offset":22188,"ancestor_record_offset":20152,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":20576,"parent_record_offset":20548,"ancestor_record_offset":20152,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":22024,"parent_record_offset":21988,"ancestor_record_offset":20152,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":20688,"parent_record_offset":20664,"ancestor_record_offset":20152,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":20712,"parent_record_offset":20688,"ancestor_record_offset":20152,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":22048,"parent_record_offset":22024,"ancestor_record_offset":20152,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":22080,"parent_record_offset":21988,"ancestor_record_offset":20152,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":20892,"parent_record_offset":20804,"ancestor_record_offset":20152,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":170,"record_offset":20804,"parent_record_offset":20780,"ancestor_record_offset":20152,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":170,"record_offset":20916,"parent_record_offset":20892,"ancestor_record_offset":20152,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":170,"record_offset":21876,"parent_record_offset":21852,"ancestor_record_offset":20152,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::channel
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":170,"record_offset":20356,"parent_record_offset":20320,"ancestor_record_offset":20152,"inlinee":7806,"source_line":197,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::counter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\counter.rs
//     evidence={"module":170,"record_offset":20392,"parent_record_offset":20356,"ancestor_record_offset":20152,"inlinee":7808,"source_line":20,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FB2
// 首条位置: {"module":31,"inlinee":8114}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":170,"record_offset":22836,"rva":4003664,"va":5372712784}
// 编译实例 RVAs: 0x3d1750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400570b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3647952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e83056c8ff","size":5,"soft_fail":false,"target_va":5369065648,"va":5372713595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140058e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3641984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8806dc8ff","size":5,"soft_fail":false,"target_va":5369073184,"va":5372715163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2648315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e80597d7ff","size":5,"soft_fail":false,"target_va":5370066128,"va":5372714438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2558376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e858f6d8ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5372716499,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140310740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-792064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e800eaf3ff","size":5,"soft_fail":false,"target_va":5371922240,"va":5372714299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403cf120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e87ecbffff","size":5,"soft_fail":false,"target_va":5372703008,"va":5372716445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2186401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8a15c2100","size":5,"soft_fail":false,"target_va":5374900736,"va":5372714330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e88a0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2189281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8e1672100","size":5,"soft_fail":false,"target_va":5374904480,"va":5372715194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f8f70
//     owner=unresolved_namespace; count=1
//     target_aliases=18 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2255115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e80b692200","size":5,"soft_fail":false,"target_va":5374971760,"va":5372716640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fa460
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2263971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8a38b2200","size":5,"soft_fail":false,"target_va":5374977120,"va":5372713144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=20
//     target_aliases=1 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2368362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e86a232400","size":5,"soft_fail":false,"target_va":5375081504,"va":5372713137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=7
//     target_aliases=5 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10114418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e872559a00","size":5,"soft_fail":false,"target_va":5382827728,"va":5372713305,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a2020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38601346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e882024d02","size":5,"soft_fail":false,"target_va":5411315744,"va":5372714393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38607160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e838194d02","size":5,"soft_fail":false,"target_va":5411320912,"va":5372713747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3530
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38607580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8dc1a4d02","size":5,"soft_fail":false,"target_va":5411321136,"va":5372713551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a3c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: theme_fetcher_plugin.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t38608899","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e803204d02","size":5,"soft_fail":false,"target_va":5411322912,"va":5372714008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..fmt..Display$u20$for$u20$anyhow..Error$GT$::fmt
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9940291","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e843ad9700","size":5,"soft_fail":false,"target_va":5382654992,"va":5372714696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9940275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e833ad9700","size":5,"soft_fail":false,"target_va":5382655024,"va":5372714744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t38827305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e829755002","size":5,"soft_fail":false,"target_va":5411542880,"va":5372715570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7817399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8b7487700","size":5,"soft_fail":false,"target_va":5380531328,"va":5372713924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t38776863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e81fb04f02","size":5,"soft_fail":false,"target_va":5411492368,"va":5372715500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t38781827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e883c34f02","size":5,"soft_fail":false,"target_va":5411494800,"va":5372712968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9110750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8de048b00","size":5,"soft_fail":false,"target_va":5381823600,"va":5372712845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t9111067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e81b068b00","size":5,"soft_fail":false,"target_va":5381824016,"va":5372712944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9981200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8104d9800","size":5,"soft_fail":false,"target_va":5382694656,"va":5372713451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9991442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e812759800","size":5,"soft_fail":false,"target_va":5382704816,"va":5372713369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t10030736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8900e9900","size":5,"soft_fail":false,"target_va":5382744080,"va":5372713339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::is_file
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t10031093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8f50f9900","size":5,"soft_fail":false,"target_va":5382745328,"va":5372714230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::array::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1182651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8bb0b1200","size":5,"soft_fail":false,"target_va":5373895552,"va":5372712896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::list::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1484215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e8495ae9ff","size":5,"soft_fail":false,"target_va":5371228672,"va":5372712882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sync::mpmc::zero::Channel$LT$T$GT$::try_recv
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1368653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372712784,"raw_hex":"e84de21400","size":5,"soft_fail":false,"target_va":5374081568,"va":5372712910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":170,"record_offset":23576,"parent_record_offset":22836,"ancestor_record_offset":22836,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":23208,"parent_record_offset":22836,"ancestor_record_offset":22836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":23920,"parent_record_offset":22836,"ancestor_record_offset":22836,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":170,"record_offset":22988,"parent_record_offset":22836,"ancestor_record_offset":22836,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mpsc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpsc.rs
//     evidence={"module":170,"record_offset":23144,"parent_record_offset":22836,"ancestor_record_offset":22836,"inlinee":7838,"source_line":785,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":23860,"parent_record_offset":23836,"ancestor_record_offset":22836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":23836,"parent_record_offset":23780,"ancestor_record_offset":22836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":23808,"parent_record_offset":23780,"ancestor_record_offset":22836,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":23780,"parent_record_offset":23756,"ancestor_record_offset":22836,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":22836,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30824,"parent_record_offset":30772,"ancestor_record_offset":22836,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30736,"parent_record_offset":30704,"ancestor_record_offset":22836,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30772,"parent_record_offset":30736,"ancestor_record_offset":22836,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":22836,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":22836,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":25248,"parent_record_offset":25208,"ancestor_record_offset":22836,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25208,"parent_record_offset":25168,"ancestor_record_offset":22836,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":26156,"parent_record_offset":26120,"ancestor_record_offset":22836,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25168,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":170,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":22836,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":27400,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":30852,"parent_record_offset":30772,"ancestor_record_offset":22836,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":28176,"parent_record_offset":28148,"ancestor_record_offset":22836,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":28592,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":29184,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=93
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":23660,"parent_record_offset":23636,"ancestor_record_offset":22836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":30880,"parent_record_offset":30772,"ancestor_record_offset":22836,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":30668,"parent_record_offset":23968,"ancestor_record_offset":22836,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":27332,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":23320,"parent_record_offset":23256,"ancestor_record_offset":22836,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":23052,"parent_record_offset":23016,"ancestor_record_offset":22836,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":23344,"parent_record_offset":23320,"ancestor_record_offset":22836,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":170,"record_offset":23076,"parent_record_offset":23052,"ancestor_record_offset":22836,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":170,"record_offset":23232,"parent_record_offset":23208,"ancestor_record_offset":22836,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":170,"record_offset":23016,"parent_record_offset":22988,"ancestor_record_offset":22836,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":170,"record_offset":23256,"parent_record_offset":23232,"ancestor_record_offset":22836,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":170,"record_offset":23108,"parent_record_offset":23016,"ancestor_record_offset":22836,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":170,"record_offset":23292,"parent_record_offset":23256,"ancestor_record_offset":22836,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":22836,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":22836,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":22836,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":170,"record_offset":24340,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24308,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":22836,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24168,"parent_record_offset":24012,"ancestor_record_offset":22836,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24372,"parent_record_offset":24340,"ancestor_record_offset":22836,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mpmc::Receiver::try_recv
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\mpmc\mod.rs
//     evidence={"module":170,"record_offset":23168,"parent_record_offset":23144,"ancestor_record_offset":22836,"inlinee":7804,"source_line":915,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":24012,"parent_record_offset":23968,"ancestor_record_offset":22836,"inlinee":13903,"source_line":61,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":28624,"parent_record_offset":28592,"ancestor_record_offset":22836,"inlinee":13907,"source_line":79,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":23968,"parent_record_offset":23920,"ancestor_record_offset":22836,"inlinee":13901,"source_line":56,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":30704,"parent_record_offset":30668,"ancestor_record_offset":22836,"inlinee":13905,"source_line":57,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x364D
// 首条位置: {"module":170,"inlinee":13901}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":30668,"parent_record_offset":23968,"ancestor_record_offset":23968,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::theme_fetcher_plugin::try_load_theme_info
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":24012,"parent_record_offset":23968,"ancestor_record_offset":23968,"inlinee":13903,"source_line":61,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":24788,"parent_record_offset":24764,"ancestor_record_offset":23968,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":24764,"parent_record_offset":24708,"ancestor_record_offset":23968,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24736,"parent_record_offset":24708,"ancestor_record_offset":23968,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":23968,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":23968,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30824,"parent_record_offset":30772,"ancestor_record_offset":23968,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30736,"parent_record_offset":30704,"ancestor_record_offset":23968,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30772,"parent_record_offset":30736,"ancestor_record_offset":23968,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":23968,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":23968,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":25248,"parent_record_offset":25208,"ancestor_record_offset":23968,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25208,"parent_record_offset":25168,"ancestor_record_offset":23968,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":26156,"parent_record_offset":26120,"ancestor_record_offset":23968,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25168,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":27400,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":30852,"parent_record_offset":30772,"ancestor_record_offset":23968,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":28176,"parent_record_offset":28148,"ancestor_record_offset":23968,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":28592,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":29184,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=85
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":24540,"parent_record_offset":24340,"ancestor_record_offset":23968,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":30880,"parent_record_offset":30772,"ancestor_record_offset":23968,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":27332,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":23968,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":23968,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":23968,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":170,"record_offset":24340,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24308,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":23968,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24168,"parent_record_offset":24012,"ancestor_record_offset":23968,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24372,"parent_record_offset":24340,"ancestor_record_offset":23968,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":28624,"parent_record_offset":28592,"ancestor_record_offset":23968,"inlinee":13907,"source_line":79,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":30704,"parent_record_offset":30668,"ancestor_record_offset":23968,"inlinee":13905,"source_line":57,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3651
// 首条位置: {"module":170,"inlinee":13905}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$34::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30736,"parent_record_offset":30704,"ancestor_record_offset":30704,"inlinee":6018,"source_line":2760,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":31152,"parent_record_offset":30704,"ancestor_record_offset":30704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":31092,"parent_record_offset":31068,"ancestor_record_offset":30704,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":31068,"parent_record_offset":31012,"ancestor_record_offset":30704,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":31040,"parent_record_offset":31012,"ancestor_record_offset":30704,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":31012,"parent_record_offset":30988,"ancestor_record_offset":30704,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":30988,"parent_record_offset":30964,"ancestor_record_offset":30704,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30824,"parent_record_offset":30772,"ancestor_record_offset":30704,"inlinee":6303,"source_line":446,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$35::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":170,"record_offset":30772,"parent_record_offset":30736,"ancestor_record_offset":30704,"inlinee":6302,"source_line":2777,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":30852,"parent_record_offset":30772,"ancestor_record_offset":30704,"inlinee":6304,"source_line":547,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":30916,"parent_record_offset":30772,"ancestor_record_offset":30704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":30880,"parent_record_offset":30772,"ancestor_record_offset":30704,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x364F
// 首条位置: {"module":170,"inlinee":13903}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25168,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":8212,"source_line":46,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":170,"record_offset":27400,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":28592,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":170,"record_offset":29184,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":25408,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":27332,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":170,"record_offset":24340,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24308,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24168,"parent_record_offset":24012,"ancestor_record_offset":24012,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":24788,"parent_record_offset":24764,"ancestor_record_offset":24012,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":24764,"parent_record_offset":24708,"ancestor_record_offset":24012,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24736,"parent_record_offset":24708,"ancestor_record_offset":24012,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24708,"parent_record_offset":24684,"ancestor_record_offset":24012,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":24684,"parent_record_offset":24660,"ancestor_record_offset":24012,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":170,"record_offset":27764,"parent_record_offset":27740,"ancestor_record_offset":24012,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":26180,"parent_record_offset":26156,"ancestor_record_offset":24012,"inlinee":12447,"source_line":316,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/error.rs
//     evidence={"module":170,"record_offset":25248,"parent_record_offset":25208,"ancestor_record_offset":24012,"inlinee":7901,"source_line":173,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":25208,"parent_record_offset":25168,"ancestor_record_offset":24012,"inlinee":7918,"source_line":23,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/context.rs
//     evidence={"module":170,"record_offset":26156,"parent_record_offset":26120,"ancestor_record_offset":24012,"inlinee":12446,"source_line":33,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":170,"record_offset":28176,"parent_record_offset":28148,"ancestor_record_offset":24012,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=68
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":24540,"parent_record_offset":24340,"ancestor_record_offset":24012,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":28652,"parent_record_offset":28624,"ancestor_record_offset":24012,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":24012,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24264,"parent_record_offset":24240,"ancestor_record_offset":24012,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":170,"record_offset":24240,"parent_record_offset":24216,"ancestor_record_offset":24012,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":24012,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":24012,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":170,"record_offset":24372,"parent_record_offset":24340,"ancestor_record_offset":24012,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/theme_fetcher_plugin.rs
//     evidence={"module":170,"record_offset":28624,"parent_record_offset":28592,"ancestor_record_offset":24012,"inlinee":13907,"source_line":79,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3653
// 首条位置: {"module":170,"inlinee":13907}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":28720,"parent_record_offset":28624,"ancestor_record_offset":28624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":170,"record_offset":28652,"parent_record_offset":28624,"ancestor_record_offset":28624,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":28896,"parent_record_offset":28872,"ancestor_record_offset":28624,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":170,"record_offset":28872,"parent_record_offset":28816,"ancestor_record_offset":28624,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":28844,"parent_record_offset":28816,"ancestor_record_offset":28624,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":28816,"parent_record_offset":28792,"ancestor_record_offset":28624,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":170,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":28624,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00170.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":170,"record_offset":28688,"parent_record_offset":28652,"ancestor_record_offset":28624,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00170.jsonl.gz"}
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
// 完整结构及原始类型记录: theme_fetcher_plugin.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ActiveEventReceiver
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 36
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$9::init_state<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::system::system_param::impl$9::init_state", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "bevy_ecs::change_detection::Res", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::impl$4::into_configs<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::schedule::config::impl$4::into_configs", "arguments": ["void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >,bevy_app::main_schedule::Update,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "bevy_app::main_schedule::Update", "void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::theme_fetcher_plugin::ActiveEventReceiver"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$12::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin>", "constructor": "bevy_app::plugin::sealed::impl$12::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: theme_fetcher_plugin.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: theme_fetcher_plugin.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>", "bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build
// 信息 ID: adc6fb03b3ad65de1bb77adc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::impl$0::build::closure$0
// 信息 ID: ff42ed049bae3f4cbabfbd3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- std::thread::Builder::spawn<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::theme_fetcher_plugin::impl$0::build::closure_env$0,tuple$<> >; module=31, inline_record=19180; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info
// 信息 ID: da550745495363d25557be7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=170, inline_record=24012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::try_load_theme_info::closure$0
// 信息 ID: a27c8fc67702682cd61637e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::try_load_theme_info", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=170, inline_record=28624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme
// 信息 ID: e850904bee3bd8e90c05c9a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c4ff6cd0298cc47ef1ee6925: void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > @ 0x25b87f -> 0x3d1750 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>),void (*)(bevy_ecs::change_detection::Res<tiny_glade::theme_fetcher_plugin::ActiveEventReceiver>,bevy_ecs::change_detection::ResMut<country_core::resources::daily::CurrentDaily>)> > > @ 0x47aec1 -> 0x3d1750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0
// 信息 ID: a3485e4e554ceb2b8e92d9d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=170, inline_record=23968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::theme_fetcher_plugin::update_daily_theme::closure$0::closure$0
// 信息 ID: d9e20f53083d554d297899f7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::theme_fetcher_plugin::update_daily_theme", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::theme_fetcher_plugin::update_daily_theme; module=170, inline_record=30704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
